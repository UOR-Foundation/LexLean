//! Read-only diagnostic using the owning public identity APIs.
//! Proposed location: crates/lexlean/examples/observe_forward_identity.rs.
//! Not an acceptance test and not part of the admitted model-only workflow.
use std::error::Error;
use std::path::PathBuf;

use lexlean::artifact::content_id::Sha256Digest;
use lexlean::calculus::rust::{renderer_digest, RENDERER_FILES};
use serde_json::json;

fn main() -> Result<(), Box<dyn Error>> {
    let root = PathBuf::from(std::env::args_os().nth(1).ok_or("checkout root required")?);
    let sources: Vec<Vec<u8>> = RENDERER_FILES
        .iter()
        .map(|name| std::fs::read(root.join("crates/lexlean/src/calculus/rust").join(name)))
        .collect::<Result<_, _>>()?;
    let files: Vec<(&str, &[u8])> = RENDERER_FILES
        .iter()
        .copied()
        .zip(sources.iter().map(Vec::as_slice))
        .collect();
    let renderer = renderer_digest(&files).to_hex();
    let renderer_inputs: Vec<_> = files
        .iter()
        .map(|(name, bytes)| json!({"path": name, "sha256": Sha256Digest::of(bytes).to_hex()}))
        .collect();
    let mut embedded_inputs = Vec::new();
    for (name, bytes) in lexlean::embedded::FILES {
        let disk = std::fs::read(root.join(name))?;
        if disk.as_slice() != *bytes {
            return Err(format!("binary embeds stale bytes for {name}; rebuild before observing").into());
        }
        embedded_inputs.push(json!({"path": name, "sha256": Sha256Digest::of(bytes).to_hex()}));
    }
    let semantics: toml::Value =
        std::fs::read_to_string(root.join("language/semantics-1.2.toml"))?.parse()?;
    let recorded = semantics.get("rust_renderer").and_then(toml::Value::as_str);
    let errors = std::fs::read(root.join("model/errors.toml"))?;
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "qualified": false,
            "renderer_observed": renderer,
            "renderer_recorded": recorded,
            "renderer_matches": recorded == Some(renderer.as_str()),
            "renderer_inputs": renderer_inputs,
            "compiler_semantics_1_2": lexlean::compiler_semantics_id_for(lexlean::LANGUAGE_1_2).to_hex(),
            "embedded_inputs": embedded_inputs,
            "errors_sha256": Sha256Digest::of(&errors).to_hex(),
        }))?
    );
    Ok(())
}
