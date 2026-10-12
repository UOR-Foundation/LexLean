//! The conformance case registry (SPEC.md §27.8, §31).
//!
//! Every generated test `conformance_<id>` calls [`run`] with its ID; the
//! dispatch panics for an unwired ID, so a registered capability cannot
//! pass before its case exists.

mod artifacts;
mod calculus;
mod certification;
mod cli_api;
mod collections_model;
mod configuration_lock;
mod declarations;
mod examples;
mod extraction;
pub use extraction::{compare_with_oracle, counter_independence, ne_04_lean_half, oracle_records};
mod gnaf;
mod grammar;
mod latex_pdf;
mod lean_backend;
mod lexical_closure;
mod lexicon;
mod models;
mod operational;
mod oracle;
mod preservation;
mod production;
mod production_export;
mod proofs;
mod reasoning;
mod repository;
mod rust_backend;
mod security;
mod semantic_ir;
mod tooling;
mod typed_diagnostics;
mod verification;

/// The wall time of one case, written to the CI evidence directory when the
/// environment names one (SPEC.md §22.12) and nowhere else. It is written when
/// the case ends, by panic as well, and a failure to write it is dropped.
struct Timed {
    id: String,
    started: std::time::Instant,
}

impl Timed {
    fn start(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            started: std::time::Instant::now(),
        }
    }
}

impl Drop for Timed {
    fn drop(&mut self) {
        if let Some(directory) = std::env::var_os(lexlean::verify::operational::TELEMETRY_VARIABLE)
            .filter(|value| !value.is_empty())
        {
            lexlean::verify::telemetry::append(
                std::path::Path::new(&directory),
                "cases-telemetry.jsonl",
                &serde_json::json!({
                    "event": "case",
                    "case": self.id,
                    "wall_ms": u64::try_from(self.started.elapsed().as_millis()).unwrap_or(u64::MAX),
                    "ok": !std::thread::panicking(),
                }),
            );
        }
    }
}

/// Run the case for one conformance ID.
///
/// # Panics
///
/// Panics when the case's assertion fails, and for an ID with no wired
/// case — an unimplemented registered capability must not pass silently.
pub fn run(id: &str) {
    let _timed = Timed::start(id);
    let prefix = id.split('-').next().unwrap_or_default();
    match prefix {
        "RP" => repository::run(id),
        "CF" => configuration_lock::run(id),
        "LX" => lexical_closure::run(id),
        "GL" => lexicon::run(id),
        "GR" => grammar::run(id),
        "SM" => semantic_ir::run(id),
        "DF" => declarations::run(id),
        "PF" => proofs::run(id),
        "LN" => lean_backend::run(id),
        "TX" => latex_pdf::run(id),
        "AR" => artifacts::run(id),
        "VR" => verification::run(id),
        "CL" => cli_api::run(id),
        "SE" => security::run(id),
        "EX" => examples::run(id),
        "PD" => production::run(id),
        "PX" => production_export::run(id),
        "NE" => extraction::run(id),
        "TC" => calculus::run(id),
        "RB" => rust_backend::run(id),
        "GN" => gnaf::run(id),
        "GC" => certification::run(id),
        "SP" => preservation::run(id),
        "OR" => oracle::run(id),
        "MD" => models::run(id),
        "RS" => reasoning::run(id),
        "TL" => tooling::run(id),
        _ => panic!("no conformance case is wired for {id}"),
    }
}
