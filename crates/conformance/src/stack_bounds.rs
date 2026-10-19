//! The stack a recursive root needs, measured and recorded (SPEC.md §17.13,
//! `PD-08`).
//!
//! A root that admits `recursion` is correct only under a stack its
//! input-dependent depth fits, and nothing in a package bounds that depth. A
//! conformance harness is the host of such a root, so the stack it gives the
//! call is a fact to record, not a size to pick: `tests/stack-bounds.json`
//! holds, for each recursive package of the `bootstrap` and `stdlib` exports,
//! the least stack on which the harness the cases run still finishes, found
//! by bisecting the stack of one built binary. The record is a measurement of
//! the cases' own inputs on the host that wrote it (level `build`: evidence,
//! never a guarantee for another host or input).

use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::rust_harness::{HARNESS_STACK, HARNESS_STACK_ENV};
use crate::support::repo_root;

/// The factor between a recorded bound and the stack a harness gives: the
/// recorded bound is the least stack that finished on one host, and an
/// unoptimized frame differs by a few tens of percent between hosts, so a
/// stack of this many times the bound is the one the cases are allowed to
/// rely on.
pub const HEADROOM: usize = 4;

/// The precision of a bound, in bytes: a stack is measured in multiples of it.
const GRANULE: usize = 64 << 10;

/// The environment variable that makes the cases measure and rewrite the
/// record instead of reading it.
const MEASURE_ENV: &str = "LEXLEAN_MEASURE_STACK";

/// The spec string of the record.
const SPEC: &str = "lexlean/stack-bounds/1";

/// What one subject needs, in bytes of thread stack.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Row {
    /// The least stack the debug build of the harness the cases run finished
    /// on, for the corpus the cases run. The cases build the debug profile,
    /// so this is the bound the check reads.
    pub debug: u64,
}

/// What a root needs for one input of a stated size.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Scale {
    /// The length of the one input, in bytes.
    pub input_bytes: u64,
    /// The least stack a debug build finished on.
    pub debug: u64,
    /// The least stack a release build finished on: the build and the method
    /// of the issue's own table.
    pub release: u64,
}

/// The record `tests/stack-bounds.json`.
#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    /// [`SPEC`].
    pub spec: String,
    /// [`HEADROOM`] when the record was written.
    pub headroom: u64,
    /// The precision of every bound, in bytes.
    pub granule: u64,
    /// How a root's need grows with one input, by `<corpus>/<package>`,
    /// ascending in `input_bytes`. The corpora of the cases are short inputs
    /// in great number, so they bound the need of those inputs and not the
    /// depth a long one reaches; this is the issue's own table.
    pub scaling: BTreeMap<String, Vec<Scale>>,
    /// The bounds, by `<export corpus>/<package name>`.
    pub rows: BTreeMap<String, Row>,
}

/// The record's path.
#[must_use]
pub fn path() -> std::path::PathBuf {
    repo_root()
        .join("tests/stack-bounds.json")
        .as_std_path()
        .to_path_buf()
}

/// The committed record.
///
/// # Panics
///
/// Panics when the record is missing or malformed.
#[must_use]
pub fn load() -> Record {
    let bytes = std::fs::read(path()).expect("tests/stack-bounds.json is committed");
    let record: Record = serde_json::from_slice(&bytes).expect("the record is JSON");
    assert_eq!(record.spec, SPEC, "the record's spec");
    record
}

/// The least stack a harness may be given for `subject`: its recorded bound
/// times [`HEADROOM`].
///
/// # Errors
///
/// Names the subject when it has no row.
pub fn required(record: &Record, subject: &str) -> Result<usize, String> {
    let row = record.rows.get(subject).ok_or_else(|| {
        format!("{subject} admits recursion and has no stack bound in tests/stack-bounds.json")
    })?;
    usize::try_from(row.debug)
        .map(|bound| bound.saturating_mul(HEADROOM))
        .map_err(|error| error.to_string())
}

/// Whether a harness for `subject` may run on `stack`.
///
/// # Errors
///
/// Names the subject, the stack, and the stack it needs.
pub fn check(record: &Record, subject: &str, stack: usize) -> Result<(), String> {
    let needed = required(record, subject)?;
    if stack < needed {
        return Err(format!(
            "{subject}: a stack of {stack} bytes is below its recorded bound with headroom ({needed} bytes)"
        ));
    }
    Ok(())
}

/// The corpora of production exports whose recursive packages carry a row.
const CORPORA: [&str; 2] = ["bootstrap", "stdlib"];

/// The key of the evaluator's row: the thread `bootstrap::run` gives the
/// calculus evaluator, which recurses with the depth of the program it runs.
pub const EVALUATOR: &str = "bootstrap/evaluator";

/// Every package of the committed `bootstrap` and `stdlib` exports, as
/// `<corpus>/<package directory>`, with whether its provenance states
/// `recursion`.
///
/// # Panics
///
/// Panics when a committed provenance cannot be read.
#[must_use]
pub fn packages() -> BTreeMap<String, bool> {
    let mut found = BTreeMap::new();
    for corpus in CORPORA {
        let base = repo_root().join("tests/production-export").join(corpus);
        for entry in walkdir::WalkDir::new(base.join("packages"))
            .into_iter()
            .map(|entry| entry.expect("the committed export is readable"))
            .filter(|entry| entry.file_name() == "provenance.json")
        {
            let provenance: serde_json::Value =
                serde_json::from_slice(&std::fs::read(entry.path()).expect("provenance.json"))
                    .expect("provenance.json is JSON");
            let recursive = provenance["effects"]
                .as_array()
                .expect("a package states its effects")
                .iter()
                .any(|effect| effect == "recursion");
            let package = entry
                .path()
                .parent()
                .and_then(Path::file_name)
                .expect("a package directory")
                .to_string_lossy();
            found.insert(format!("{corpus}/{package}"), recursive);
        }
    }
    found
}

/// The packages whose provenance states `recursion`.
#[must_use]
pub fn recursive_packages() -> std::collections::BTreeSet<String> {
    packages()
        .into_iter()
        .filter_map(|(package, recursive)| recursive.then_some(package))
        .collect()
}

/// The key of the export package a case built as crate `crate_name`
/// (`boot_<package>_std`, `text_<package>_std`) in `corpus`.
#[must_use]
pub fn subject(corpus: &str, crate_name: &str) -> String {
    let package = crate_name
        .split_once('_')
        .map_or(crate_name, |(_, rest)| rest);
    format!(
        "{corpus}/{}",
        package.strip_suffix("_std").unwrap_or(package)
    )
}

/// What the cases are asked to measure: `all`, or `scaling` for the growth
/// table alone (the evaluator's workload takes most of an hour); `None` when
/// they are not measuring.
#[must_use]
pub fn measuring() -> Option<String> {
    std::env::var(MEASURE_ENV).ok()
}

/// Whether `binary` runs to its end on a thread of `stack` bytes.
#[must_use]
pub fn finishes(binary: &Path, stack: usize) -> bool {
    std::process::Command::new(binary)
        .env(HARNESS_STACK_ENV, stack.to_string())
        .output()
        .is_ok_and(|ran| {
            ran.status.success() && String::from_utf8_lossy(&ran.stdout).contains("work ")
        })
}

/// The least stack, in multiples of the granule and at most `limit`, on
/// which `finishes` holds. It doubles a stack from one granule until `finishes`
/// holds and then bisects the last doubling, because a probe that finishes is
/// a whole run of the workload.
///
/// # Panics
///
/// Panics when it does not hold on `limit`.
pub fn least(limit: usize, finishes: impl Fn(usize) -> bool) -> usize {
    let mut high = GRANULE;
    while high < limit && !finishes(high) {
        high *= 2;
    }
    let high = high.min(limit);
    assert!(
        finishes(high),
        "the workload does not finish on {high} bytes"
    );
    let (mut low, mut high) = ((high / 2 / GRANULE).max(1), high / GRANULE);
    while low < high {
        let middle = low + (high - low) / 2;
        if finishes(middle * GRANULE) {
            high = middle;
        } else {
            low = middle + 1;
        }
    }
    low * GRANULE
}

/// The part of the current thread's stack the thread has touched, in bytes
/// rounded up to the granule: the resident size of the mapping `stack` bytes
/// long that holds it. A deep recursion touches every page down to its
/// deepest frame, so this is the peak depth of the work the thread has done,
/// and it needs one run where a bisection needs a dozen of a workload that
/// takes minutes. Linux only; the cases measure on the normative host.
///
/// # Panics
///
/// Panics when `/proc/self/smaps` does not hold exactly one such mapping.
#[must_use]
pub fn touched_stack(stack: usize) -> usize {
    let smaps = std::fs::read_to_string("/proc/self/smaps").expect("/proc/self/smaps");
    let mut found = Vec::new();
    let mut size = 0usize;
    for line in smaps.lines() {
        let mut words = line.split_whitespace();
        match (words.next(), words.next()) {
            (Some("Size:"), Some(kilobytes)) => size = kilobytes.parse().unwrap_or(0) * 1024,
            (Some("Rss:"), Some(kilobytes)) if size >= stack && size <= stack + (2 << 20) => {
                found.push(kilobytes.parse::<usize>().expect("kilobytes") * 1024);
            }
            _ => {}
        }
    }
    assert_eq!(found.len(), 1, "one mapping of {stack} bytes: {found:?}");
    found[0].div_ceil(GRANULE) * GRANULE
}

/// The least stack on which `binary` finishes, starting from
/// [`HARNESS_STACK`].
#[must_use]
pub fn bisect(binary: &Path) -> usize {
    least(HARNESS_STACK, |stack| finishes(binary, stack))
}

fn rewrite(change: impl FnOnce(&mut Record)) {
    let mut record = std::fs::read(path())
        .ok()
        .and_then(|bytes| serde_json::from_slice::<Record>(&bytes).ok())
        .unwrap_or_else(|| Record {
            spec: SPEC.to_owned(),
            headroom: 0,
            granule: 0,
            scaling: BTreeMap::new(),
            rows: BTreeMap::new(),
        });
    record.headroom = HEADROOM as u64;
    record.granule = GRANULE as u64;
    change(&mut record);
    let mut text = serde_json::to_string_pretty(&record).expect("the record serializes");
    text.push('\n');
    std::fs::write(path(), text).expect("the record is written");
}

/// Write the bound of `subject` into the record, creating the record and the
/// row when they do not exist.
///
/// # Panics
///
/// Panics when the record cannot be written.
pub fn record_row(subject: &str, debug: usize) {
    rewrite(|record| {
        record.rows.insert(
            subject.to_owned(),
            Row {
                debug: debug as u64,
            },
        );
    });
}

/// Write how the need of `subject` grows with its input into the record.
///
/// # Panics
///
/// Panics when the record cannot be written.
pub fn record_scaling(subject: &str, scales: Vec<Scale>) {
    rewrite(|record| {
        record.scaling.insert(subject.to_owned(), scales);
    });
}

/// The largest need each subject was measured to have in this process, so
/// that a case which runs one root's harness on several corpora records the
/// largest of their needs, not the last.
static OBSERVED: std::sync::Mutex<BTreeMap<String, usize>> = std::sync::Mutex::new(BTreeMap::new());

/// Take what a case has built, if `subject` is a recursive package: when
/// measuring, bisect the debug harness at `binary` and write the row (the
/// largest need this process measured for the subject); otherwise demand that
/// the record has the row, so a recursive root run by a harness without a
/// recorded bound fails the case that runs it.
///
/// # Panics
///
/// Panics when the row is missing, or when the release build fails.
pub fn observe(subject: &str, binary: &Path) {
    // A package that does not recurse needs no row, and a name that is no
    // package of the export is a mistake of the case, not a package that
    // does not recurse.
    let recursive = packages()
        .get(subject)
        .copied()
        .unwrap_or_else(|| panic!("{subject} is no package of the committed exports"));
    if !recursive {
        return;
    }
    if measuring().is_none() {
        required(&load(), subject).unwrap_or_else(|failure| panic!("{failure}"));
        return;
    }
    let measured = bisect(binary);
    let debug = {
        let mut observed = OBSERVED.lock().expect("the observed needs");
        let largest = observed.entry(subject.to_owned()).or_insert(0);
        *largest = (*largest).max(measured);
        *largest
    };
    eprintln!("{subject}: debug {measured} bytes (recorded {debug})");
    record_row(subject, debug);
}
