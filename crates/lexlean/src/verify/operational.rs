//! The operational settings of one verification (SPEC.md §22.11, §22.12):
//! how many proof processes run at once, in which start order, within what
//! memory budget, and where performance telemetry goes.
//!
//! All of it is read from the environment of the process that calls `verify`
//! and from nowhere else. Nothing here is a field of the project
//! configuration, of the lock, or of a request: a setting that could be
//! written into a project would be an input of the verification it only
//! means to schedule, and the artifacts it produces would then have to say
//! which profile made them. They do not. Nothing here is read by a stage,
//! hashed, or recorded.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::code;
use crate::diagnostic::Diagnostic;

/// The environment variable that selects the verification process width.
pub const WIDTH_VARIABLE: &str = "LEXLEAN_VERIFY_WIDTH";

/// The environment variable that names the telemetry directory.
pub const TELEMETRY_VARIABLE: &str = "LEXLEAN_CI_TELEMETRY";

/// The environment variable that names the scheduling-hints file (§22.11).
pub const HINTS_VARIABLE: &str = "LEXLEAN_VERIFY_HINTS";

/// The environment variable that bounds the summed expected resident sets
/// of the proof processes running at once (§22.11).
pub const MEMORY_VARIABLE: &str = "LEXLEAN_VERIFY_MEMORY_KIB";

/// The largest width accepted. A bound exists so that a typo such as `800`
/// is refused instead of starting that many Lean processes; no host this
/// repository targets could hold more than a few at once, and 64 is already
/// far past the largest width anyone could validate on a hosted runner.
pub const MAX_WIDTH: usize = 64;

/// The conservative profile: one proof process at a time. A single Atlas
/// environment approaches the memory of the normative runner, and overlapping
/// two on it once made the hosted runner lose its control-plane heartbeat
/// while swapping, so any larger width is an opt-in for a host whose memory
/// has been measured.
pub const DEFAULT_WIDTH: usize = 1;

/// The largest scheduling-hints file accepted, in bytes. The hints of the
/// largest project this repository verifies are tens of KiB; a file above
/// 4 MiB is a wrong path noticed early, not a hint set.
pub const MAX_HINTS_BYTES: u64 = 4 * 1024 * 1024;

/// A scheduling hint for one item of a stage (§22.11): what an observed run
/// of its process took. A hint changes which ready item a worker starts
/// first and whether it fits the memory budget — never what the stage
/// computes — so it enters no source, identity, or verified artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hint {
    /// The peak resident set the process is expected to hold at its largest,
    /// in KiB, when observed; compared against the memory budget.
    pub peak_rss_kib: Option<u64>,
    /// The observed wall time in milliseconds; longer processes start
    /// earlier so one long item does not stretch the tail of a stage.
    pub wall_ms: u64,
}

/// Scheduling hints, keyed by the caller's labels of the items of a stage.
pub type Hints = BTreeMap<String, Hint>;

/// The operational settings of a verification.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Operational {
    width: usize,
    telemetry: Option<PathBuf>,
    hints: Option<Hints>,
    memory_kib: Option<u64>,
}

impl Operational {
    /// The settings with an explicit width and telemetry directory.
    ///
    /// # Errors
    ///
    /// `LLV7017` when `width` is outside `1..=64`.
    pub fn new(width: usize, telemetry: Option<PathBuf>) -> Result<Self, Diagnostic> {
        if !(1..=MAX_WIDTH).contains(&width) {
            return Err(invalid(&width.to_string()));
        }
        Ok(Self {
            width,
            telemetry,
            hints: None,
            memory_kib: None,
        })
    }

    /// These settings with scheduling hints attached.
    #[must_use]
    pub fn with_hints(mut self, hints: Hints) -> Self {
        self.hints = Some(hints);
        self
    }

    /// These settings with a memory budget, in KiB.
    #[must_use]
    pub fn with_memory_kib(mut self, memory_kib: u64) -> Self {
        self.memory_kib = Some(memory_kib);
        self
    }

    /// The number of proof processes that may run at once.
    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    /// The directory that receives telemetry, when there is one.
    #[must_use]
    pub fn telemetry(&self) -> Option<&std::path::Path> {
        self.telemetry.as_deref()
    }

    /// Scheduling hints for the processes of the run, when a file named
    /// them. An empty map (no label of the project is known) schedules in
    /// the caller's item order, as if there were no hints.
    #[must_use]
    pub fn hints(&self) -> Option<&Hints> {
        self.hints.as_ref()
    }

    /// The memory budget the scheduler of parallel processes respects, in
    /// KiB, when one was set. Unset, the width alone bounds concurrency.
    #[must_use]
    pub fn memory_kib(&self) -> Option<u64> {
        self.memory_kib
    }

    /// Parse the text of a decimal integer variable: unset or empty selects
    /// `None`, and anything but a plain decimal integer (a sign, a space, a
    /// fraction, or a word) is refused as `LLV7017`.
    ///
    /// # Errors
    ///
    /// `LLV7017` for a malformed value.
    fn parse_decimal_with(
        text: Option<&str>,
        invalid_error: &dyn Fn(&str) -> Diagnostic,
    ) -> Result<Option<u64>, Diagnostic> {
        let Some(text) = text.filter(|text| !text.is_empty()) else {
            return Ok(None);
        };
        if !text.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(invalid_error(text));
        }
        text.parse::<u64>()
            .map(Some)
            .map_err(|_| invalid_error(text))
    }

    /// [`Self::parse_decimal`] over what the environment holds. A value that
    /// is not Unicode is a malformed value, not an absent one.
    ///
    /// # Errors
    ///
    /// `LLV7017` for a malformed or non-Unicode value.
    fn parse_decimal_os_with(
        value: Option<&std::ffi::OsStr>,
        invalid_error: &dyn Fn(&str) -> Diagnostic,
    ) -> Result<Option<u64>, Diagnostic> {
        match value {
            None => Ok(None),
            Some(raw) => match raw.to_str() {
                Some(text) => Self::parse_decimal_with(Some(text), invalid_error),
                None => Err(invalid_error(&raw.to_string_lossy())),
            },
        }
    }

    /// Parse the width, which is always in force (unset selects the default).
    ///
    /// # Errors
    ///
    /// `LLV7017` for a malformed or out-of-range value.
    pub fn parse_width(text: Option<&str>) -> Result<usize, Diagnostic> {
        match Self::parse_decimal_with(text, &invalid)? {
            None => Ok(DEFAULT_WIDTH),
            Some(width) => {
                let width = usize::try_from(width).map_err(|_| invalid(&width.to_string()))?;
                if (1..=MAX_WIDTH).contains(&width) {
                    Ok(width)
                } else {
                    Err(invalid(&width.to_string()))
                }
            }
        }
    }

    /// [`Self::parse_width`] over what the environment holds. A value that is
    /// not Unicode is a malformed value, not an absent one: reading it as
    /// unset would silently verify at the default width in a job that asked
    /// for another.
    ///
    /// # Errors
    ///
    /// `LLV7017` for a malformed, out-of-range, or non-Unicode value.
    pub fn parse_width_os(value: Option<&std::ffi::OsStr>) -> Result<usize, Diagnostic> {
        match value {
            None => Ok(DEFAULT_WIDTH),
            Some(raw) => match raw.to_str() {
                Some(text) => Self::parse_width(Some(text)),
                None => Err(invalid(&raw.to_string_lossy())),
            },
        }
    }

    /// Parse the scheduling-hints file: a JSON object whose keys are the
    /// labels of the items of a stage (this verifier: full Lean module names,
    /// and audit-member names) and whose values carry `wall_ms` with an
    /// optional `peak_rss_kib`, like the `process` events the telemetry
    /// appends (§22.12). `cargo xtask ci-hints` derives such a file from a
    /// telemetry file (the heaviest observation of a repeated label wins).
    ///
    /// # Errors
    ///
    /// `LLV7017` when the file cannot be read, exceeds the size bound, or
    /// its content is malformed.
    pub fn parse_hints(text: &str) -> Result<Hints, Diagnostic> {
        let value: serde_json::Value =
            serde_json::from_str(text).map_err(|error| invalid_hints(&error.to_string()))?;
        if let Some(object) = value.as_object() {
            const MAX_HINTS_ENTRIES: usize = 200_000;
            if object.len() > MAX_HINTS_ENTRIES {
                return Err(invalid_hints("the hints object has too many entries"));
            }
            let mut hints = Hints::new();
            for (label, record) in object {
                let wall_ms = record["wall_ms"]
                    .as_u64()
                    .ok_or_else(|| invalid_hints("an entry has no `wall_ms`"))?;
                hints.insert(
                    label.clone(),
                    Hint {
                        peak_rss_kib: record["peak_rss_kib"].as_u64(),
                        wall_ms,
                    },
                );
            }
            return Ok(hints);
        }
        Err(invalid_hints(
            "the hints are not a JSON object keyed by label",
        ))
    }

    /// The hints the environment selects, when a file names them.
    ///
    /// # Errors
    ///
    /// `LLV7017` for an unreadable, oversized, or malformed hints file, and
    /// for a path that is not Unicode (malformed, not absent).
    pub fn hints_from_os(value: Option<&std::ffi::OsStr>) -> Result<Option<Hints>, Diagnostic> {
        let Some(raw) = value else {
            return Ok(None);
        };
        let name = raw
            .to_str()
            .ok_or_else(|| invalid_hints(&raw.to_string_lossy()))?;
        if name.is_empty() {
            return Ok(None);
        }
        let metadata = std::fs::metadata(name)
            .map_err(|io_error| invalid_hints(&format!("hints `{name}`: {io_error}")))?;
        if metadata.len() > MAX_HINTS_BYTES {
            return Err(invalid_hints(&format!(
                "hints `{name}` is {} bytes, past the {MAX_HINTS_BYTES}-byte bound",
                metadata.len()
            )));
        }
        let text = std::fs::read_to_string(name)
            .map_err(|io_error| invalid_hints(&format!("hints `{name}`: {io_error}")))?;
        Self::parse_hints(&text).map(Some)
    }

    /// The settings the environment selects, unless a scoped replacement is
    /// in force on this thread.
    ///
    /// # Errors
    ///
    /// `LLV7017` for a malformed width, hints, or memory variable.
    pub fn from_environment() -> Result<Self, Diagnostic> {
        if let Some(scoped) = SCOPED.with(|cell| cell.borrow().clone()) {
            return Ok(scoped);
        }
        let width = Self::parse_width_os(std::env::var_os(WIDTH_VARIABLE).as_deref())?;
        let telemetry = std::env::var_os(TELEMETRY_VARIABLE)
            .filter(|value| !value.is_empty())
            .map(PathBuf::from);
        let hints = Self::hints_from_os(std::env::var_os(HINTS_VARIABLE).as_deref())?;
        let memory_kib = Self::parse_decimal_os_with(
            std::env::var_os(MEMORY_VARIABLE).as_deref(),
            &invalid_memory,
        )?;
        Ok(Self {
            width,
            telemetry,
            hints,
            memory_kib,
        })
    }

    /// Run `body` with these settings in force on the current thread instead
    /// of what the environment says. For tests of another width or of
    /// telemetry, which would otherwise have to change the environment of a
    /// process that other tests are reading.
    pub fn scoped<T>(self, body: impl FnOnce() -> T) -> T {
        /// Restores the previous replacement even when `body` panics.
        struct Restore(Option<Operational>);
        impl Drop for Restore {
            fn drop(&mut self) {
                let previous = self.0.take();
                SCOPED.with(|cell| *cell.borrow_mut() = previous);
            }
        }
        let previous = SCOPED.with(|cell| cell.borrow_mut().replace(self));
        let _restore = Restore(previous);
        body()
    }
}

thread_local! {
    /// A scoped replacement for the environment, so that a test of another
    /// width does not have to change the environment of the whole process.
    static SCOPED: RefCell<Option<Operational>> = const { RefCell::new(None) };
}

fn invalid(text: &str) -> Diagnostic {
    Diagnostic::new(
        code!("LLV7017"),
        format!(
            "{WIDTH_VARIABLE} is `{text}`; the verification process width is a decimal integer from 1 to {MAX_WIDTH} (unset or empty selects {DEFAULT_WIDTH})"
        ),
    )
}

fn invalid_hints(message: &str) -> Diagnostic {
    Diagnostic::new(
        code!("LLV7017"),
        format!(
            "{HINTS_VARIABLE} is invalid; {message} (the scheduling hints are a JSON object keyed by label, with `wall_ms` and an optional `peak_rss_kib` per entry, §22.11)"
        ),
    )
}

fn invalid_memory(text: &str) -> Diagnostic {
    Diagnostic::new(
        code!("LLV7017"),
        format!(
            "{MEMORY_VARIABLE} is `{text}`; the verification process memory budget is a decimal integer of KiB (unset selects no budget, §22.11)"
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::{Hint, Operational, DEFAULT_WIDTH, HINTS_VARIABLE, MAX_WIDTH, MEMORY_VARIABLE};

    #[test]
    fn the_width_is_a_plain_decimal_integer_in_range() {
        assert_eq!(Operational::parse_width(None), Ok(DEFAULT_WIDTH));
        assert_eq!(Operational::parse_width(Some("")), Ok(DEFAULT_WIDTH));
        assert_eq!(Operational::parse_width(Some("1")), Ok(1));
        assert_eq!(Operational::parse_width(Some("07")), Ok(7));
        assert_eq!(
            Operational::parse_width(Some(&MAX_WIDTH.to_string())),
            Ok(MAX_WIDTH)
        );
        for bad in [
            "0",
            "00",
            "65",
            "+2",
            "-1",
            " 2",
            "2 ",
            "2.0",
            "two",
            "0x2",
            "99999999999999999999999999",
        ] {
            let error = Operational::parse_width(Some(bad)).expect_err(bad);
            assert_eq!(error.code.as_str(), "LLV7017", "{bad}");
        }
    }

    #[test]
    fn a_value_that_is_not_unicode_is_malformed_not_absent() {
        #[cfg(unix)]
        let raw = {
            use std::os::unix::ffi::OsStringExt;
            std::ffi::OsString::from_vec(vec![b'2', 0xff])
        };
        #[cfg(windows)]
        let raw = {
            use std::os::windows::ffi::OsStringExt;
            std::ffi::OsString::from_wide(&[b'2' as u16, 0xD800])
        };
        #[cfg(any(unix, windows))]
        {
            assert!(raw.to_str().is_none());
            let error = Operational::parse_width_os(Some(&raw)).expect_err("not Unicode");
            assert_eq!(error.code.as_str(), "LLV7017");
        }
        assert_eq!(Operational::parse_width_os(None), Ok(DEFAULT_WIDTH));
        assert_eq!(
            Operational::parse_width_os(Some(std::ffi::OsStr::new("3"))),
            Ok(3)
        );
    }

    fn not_unicode() -> std::ffi::OsString {
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStringExt;
            std::ffi::OsString::from_vec(vec![b'3', 0xfe])
        }
        #[cfg(windows)]
        {
            use std::os::windows::ffi::OsStringExt;
            std::ffi::OsString::from_wide(&[b'3' as u16, 0xD800])
        }
        #[cfg(not(any(unix, windows)))]
        {
            std::ffi::OsString::from("3")
        }
    }

    #[test]
    fn the_environment_is_read_as_bytes() {
        let raw = not_unicode();
        std::env::set_var(super::WIDTH_VARIABLE, &raw);
        let refused = Operational::from_environment();
        std::env::set_var(super::WIDTH_VARIABLE, "3");
        let accepted = Operational::from_environment();
        std::env::remove_var(super::WIDTH_VARIABLE);
        assert_eq!(accepted.expect("3").width(), 3);
        if raw.to_str().is_none() {
            assert_eq!(refused.expect_err("not Unicode").code.as_str(), "LLV7017");
        }
    }

    #[test]
    fn hints_select_a_start_order_and_nothing_else() {
        let hints = Operational::parse_hints(
            r#"{"Mod.A": {"wall_ms": 900, "peak_rss_kib": 100}, "Mod.B": {"wall_ms": 100}}"#,
        )
        .expect("valid hints");
        assert_eq!(
            hints.get("Mod.A"),
            Some(&Hint {
                peak_rss_kib: Some(100),
                wall_ms: 900
            })
        );
        for bad in [
            "[]",
            "42",
            "\"text\"",
            "{\"A\": {}}",
            "{\"A\": {\"peak_rss_kib\": 1}}",
            "{\"A\": {\"wall_ms\": -1}}",
        ] {
            let error = Operational::parse_hints(bad).expect_err(bad);
            assert_eq!(error.code.as_str(), "LLV7017", "{bad}");
        }
    }

    #[test]
    fn a_hints_file_from_the_environment() {
        let directory = tempfile::tempdir().expect("tempdir");
        let path = directory.path().join("hints.json");
        std::fs::write(&path, "{\"Mod.A\": {\"wall_ms\": 5}}").expect("write");
        std::env::set_var(HINTS_VARIABLE, &path);
        let read = Operational::from_environment().expect("hints parse");
        std::env::remove_var(HINTS_VARIABLE);
        assert_eq!(
            read.hints().and_then(|hints| hints.get("Mod.A").copied()),
            Some(Hint {
                peak_rss_kib: None,
                wall_ms: 5
            })
        );
        // A non-Unicode name is malformed, not absent.
        std::env::set_var(HINTS_VARIABLE, not_unicode());
        let refused = Operational::from_environment();
        std::env::remove_var(HINTS_VARIABLE);
        assert_eq!(refused.expect_err("not Unicode").code.as_str(), "LLV7017");
        // The memory budget parses like the width's decimal, unset by default.
        std::env::set_var(MEMORY_VARIABLE, "4194304");
        let budgeted = Operational::from_environment().expect("budget");
        std::env::remove_var(MEMORY_VARIABLE);
        assert_eq!(budgeted.memory_kib(), Some(4_194_304));
        std::env::set_var(MEMORY_VARIABLE, "4G");
        let refused = Operational::from_environment();
        std::env::remove_var(MEMORY_VARIABLE);
        assert_eq!(refused.expect_err("suffix").code.as_str(), "LLV7017");
    }

    #[test]
    fn a_scoped_replacement_wins_and_is_restored() {
        let inner = Operational::new(3, None).expect("width 3");
        let seen = inner.clone().scoped(|| {
            Operational::from_environment().expect("a scoped value never reads the environment")
        });
        assert_eq!(seen, inner);
        let after = std::panic::catch_unwind(|| {
            Operational::new(2, None)
                .expect("width 2")
                .scoped(|| panic!("a failing body"))
        });
        assert!(after.is_err());
        let restored = super::SCOPED.with(|cell| cell.borrow().clone());
        assert_eq!(restored, None, "a panicking body leaves nothing in force");
    }

    #[test]
    fn a_width_outside_the_range_is_refused() {
        assert!(Operational::new(0, None).is_err());
        assert!(Operational::new(MAX_WIDTH + 1, None).is_err());
        assert_eq!(Operational::new(2, None).map(|op| op.width()), Ok(2));
    }
}
