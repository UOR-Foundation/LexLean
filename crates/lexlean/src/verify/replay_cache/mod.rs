//! The content-addressed replay cache (SPEC.md §22.13).
//!
//! A declaration is keyed by a PrismPM ADR-015 DigestSet over its canonical
//! serialisation and the keys of the declarations it names, so the keys form a
//! Merkle DAG; the Lean toolchain and the Mathlib revision are inside every
//! key. A warm replay looks each unit up by key and replays only the misses.
//!
//! The cache does not decide what a unit is. A unit is the set of declaration
//! keys it covers, one declaration or one module, because which `leanchecker`
//! mode is cheapest is a measurement the caller makes, not a property of the
//! cache.

pub mod facts;
pub mod key;
pub mod store;

#[cfg(test)]
mod tests;

pub use facts::{export_facts, parse_facts, DeclarationFacts};
pub use key::{canonical_declaration, unit_key, Granularity, ReplayEnvironment, ReplayGraph};
pub use store::{
    executable_identity, CacheEntry, ExecutableIdentity, Lookup, Miss, ReplayCache, ReplayResult,
};

use crate::artifact::digest_set::DigestSet;
use crate::diagnostic::Diagnostic;
use crate::verify::child::ChildRecord;

/// A unit to replay.
#[derive(Debug, Clone)]
pub struct ReplayUnit {
    /// Its name: a declaration or a module.
    pub label: String,
    /// What it is.
    pub granularity: Granularity,
    /// Its key.
    pub key: DigestSet,
}

/// What a run did, by unit label, in order.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReplayReport {
    /// Units whose recorded result was reused.
    pub reused: Vec<String>,
    /// Units that were replayed.
    pub replayed: Vec<String>,
}

/// The result a [`ChildRecord`] of a replay stands for.
#[must_use]
pub fn result_of(record: &ChildRecord) -> ReplayResult {
    ReplayResult {
        exit_code: record.exit_code,
        stdout: record.stdout.clone(),
        stderr: record.stderr.clone(),
    }
}

/// Replay `units`, reusing the recorded result of every unit whose key the
/// cache holds for this executable and replaying the rest with `replay`. A
/// replay that fails, or whose result does not exit zero, ends the run and
/// records nothing, so a failure is never reused. A cache entry that is
/// rejected (`LLV7019`) ends the run too: a store that was tampered with is
/// not quietly repaired.
pub fn replay_units<F>(
    cache: &ReplayCache,
    environment: &ReplayEnvironment,
    executable: &ExecutableIdentity,
    units: &[ReplayUnit],
    mut replay: F,
) -> Result<ReplayReport, Diagnostic>
where
    F: FnMut(&ReplayUnit) -> Result<ReplayResult, Diagnostic>,
{
    let mut report = ReplayReport::default();
    for unit in units {
        if let Lookup::Hit(_) = cache.lookup(&unit.key, environment, executable)? {
            report.reused.push(unit.label.clone());
            continue;
        }
        let result = replay(unit)?;
        if result.exit_code != 0 {
            return Err(Diagnostic::new(
                crate::code!("LLV7003"),
                format!(
                    "replay of `{}` exited {}: {}{}",
                    unit.label,
                    result.exit_code,
                    result.stdout.trim_end(),
                    result.stderr.trim_end()
                ),
            ));
        }
        cache.record(&CacheEntry {
            key: unit.key,
            granularity: unit.granularity,
            label: unit.label.clone(),
            environment: environment.clone(),
            executable: executable.clone(),
            result,
        })?;
        report.replayed.push(unit.label.clone());
    }
    Ok(report)
}
