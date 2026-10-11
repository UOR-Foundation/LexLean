//! Noncanonical CI performance telemetry (SPEC.md §22.11, R2, VR-21).
//!
//! Telemetry is enabled only by the environment variable
//! `LEXLEAN_VERIFY_TELEMETRY` naming a host-chosen file, and only when the
//! operator asks for it. Each verification child process appends one JSON
//! line with its stage, module, and wall time; each measured stage appends
//! one boundary line. The rows are CI evidence, never a claim about meaning:
//! they are never read by the compiler, and no value here — a duration, a
//! host, a scheduling order — enters a semantic ID, a build ID, a verified
//! artifact, a normalized verification record, or an attestation identity.
//!
//! Writing is deliberate best-effort. Verification's acceptance obligations
//! are about proof, not about a measuring instrument, so an unwritable
//! telemetry path disables the row it would have written and leaves the
//! verification's result untouched. That is the opposite of the verifier's
//! other I/O, which fails closed; it is safe here only because nothing
//! downstream ever consumes this file.

use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use camino::Utf8PathBuf;

use crate::artifact::canonical_json::Json;

/// A telemetry sink for one verification run, or a disabled sink when the
/// environment names no file.
pub struct Telemetry {
    path: Option<Utf8PathBuf>,
}

impl Telemetry {
    /// Read the sink path from `LEXLEAN_VERIFY_TELEMETRY`. An unset or empty
    /// variable disables telemetry; a path that cannot be represented as
    /// UTF-8 is treated as disabled rather than as an error, because a
    /// telemetry failure must never change a verification's result.
    #[must_use]
    pub fn from_env() -> Self {
        let path = std::env::var_os("LEXLEAN_VERIFY_TELEMETRY")
            .filter(|value| !value.is_empty())
            .and_then(|value| Utf8PathBuf::from_path_buf(PathBuf::from(value)).ok());
        Self { path }
    }

    fn emit(&self, stage: &str, module: Option<&str>, kind: Option<&str>, wall: Duration) {
        let Some(path) = &self.path else {
            return;
        };
        let mut fields = vec![
            ("v", Json::from_usize(1)),
            ("stage", Json::Str(stage.to_owned())),
        ];
        if let Some(kind) = kind {
            fields.push(("kind", Json::Str(kind.to_owned())));
        }
        if let Some(module) = module {
            fields.push(("module", Json::Str(module.to_owned())));
        }
        let millis = usize::try_from(wall.as_millis()).unwrap_or(usize::MAX);
        fields.push(("wall_ms", Json::from_usize(millis)));
        let mut line = Json::object(fields).to_canonical_string();
        line.push('\n');
        // Append mode with one short write: a row from a concurrent
        // verification run in the same process cannot interleave with this
        // one, and a failed open or write drops the row without failing the
        // verification it measures.
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path.as_std_path())
        {
            let _ = file.write_all(line.as_bytes());
        }
    }

    /// Record one child process's wall time.
    pub fn child(&self, stage: &str, module: Option<&str>, wall: Duration) {
        self.emit(stage, module, None, wall);
    }

    /// Record one stage's total wall time.
    pub fn stage(&self, stage: &str, wall: Duration) {
        self.emit(stage, None, Some("stage"), wall);
    }

    /// Time `job` and record its wall time as a `stage` child under `module`.
    pub fn timed<T>(&self, stage: &str, module: Option<&str>, job: impl FnOnce() -> T) -> T {
        let started = Instant::now();
        let value = job();
        self.child(stage, module, started.elapsed());
        value
    }
}
