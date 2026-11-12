//! Owned, bounded kernel-type inspection (§22.13). This produces input data,
//! never a verification attestation. The caller owns its library build.

use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};

use super::Engine;
use crate::artifact::content_id::Sha256Digest;
use crate::diagnostic::Diagnostic;
use crate::error::LexLeanError;
use crate::verify::child::{self, ChildHome, ChildSpec, Normalizer};

/// A bounded query against the project's pinned local Lean environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InspectionRequest {
    /// Fully qualified module names, imported in this order.
    pub imports: Vec<String>,
    /// Fully qualified declaration names, inspected in this order.
    pub declarations: Vec<String>,
    /// Shared traversal budget, including names and universe levels.
    pub max_nodes: u64,
    /// Maximum structural nesting depth.
    pub max_depth: u64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawReport {
    spec: String,
    lean: LeanIdentity,
    declarations: Vec<Declaration>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct LeanIdentity {
    version: String,
    githash: String,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Declaration {
    name: serde_json::Value,
    parameters: Vec<serde_json::Value>,
    #[serde(rename = "type")]
    ty: serde_json::Value,
}

fn refusal(reason: impl Into<String>) -> LexLeanError {
    LexLeanError::from_diagnostic(Diagnostic::new(crate::code!("LLV7002"), reason))
}

fn identifier(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 4096
        && name.split('.').all(|segment| {
            let mut bytes = segment.bytes();
            bytes
                .next()
                .is_some_and(|b| b.is_ascii_alphabetic() || b == b'_')
                && bytes.all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'\'')
        })
}

fn structural_name(name: &str) -> serde_json::Value {
    name.split('.').fold(
        serde_json::Value::Null,
        |parent, segment| serde_json::json!({"str": [parent, segment]}),
    )
}

/// Inspect exact kernel types through the embedded LexLean-authored adapter.
/// No caller-supplied type or receipt participates in the result.
///
/// # Errors
/// Rejects invalid bounds/names, toolchain drift, failed or noisy processes,
/// incomplete reports, unresolved terms, and exhausted traversal budgets.
pub fn inspect(engine: &Engine, request: &InspectionRequest) -> Result<Vec<u8>, LexLeanError> {
    if request.declarations.is_empty()
        || request.declarations.len() > 64
        || request.imports.is_empty()
        || request.imports.len() > 64
        || !(1..=65536).contains(&request.max_nodes)
        || !(1..=128).contains(&request.max_depth)
        || !request
            .imports
            .iter()
            .chain(&request.declarations)
            .all(|name| identifier(name))
        || request
            .declarations
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            != request.declarations.len()
    {
        return Err(LexLeanError::from_diagnostic(Diagnostic::new(
            crate::code!("LLC0002"), "inspection requires 1..64 distinct qualified declarations, 1..64 imports, 1..65536 nodes and 1..128 depth",
        )));
    }
    let project = engine.project();
    let mut limits = project.config.limits;
    limits.max_child_output_bytes = limits.max_child_output_bytes.min(8388608);
    let toolchain =
        crate::verify::toolchain::preflight(&limits).map_err(LexLeanError::from_diagnostic)?;
    let request_bytes = serde_json::to_vec(request).map_err(|error| refusal(error.to_string()))?;
    let request_id = Sha256Digest::of(&request_bytes);
    let driver = crate::production::lcnf::inspection_driver(
        &request_id.to_hex()[..32],
        &request.declarations,
        &request.imports,
        request.max_nodes,
        request.max_depth,
    )
    .map_err(refusal)?;
    let scratch = tempfile::Builder::new()
        .prefix("lexlean-inspection-")
        .tempdir()
        .map_err(|error| refusal(format!("inspection staging: {error}")))?;
    let directory = Utf8PathBuf::from_path_buf(scratch.path().to_owned())
        .map_err(|_| refusal("inspection staging path is not UTF-8"))?;
    let source = directory.join(format!("{}.lean", driver.name));
    std::fs::write(&source, driver.text.as_bytes())
        .map_err(|error| refusal(format!("inspection source: {error}")))?;
    let workspace = project.root.join(&project.config.lean_workspace);
    let bin = toolchain.root.join("bin");
    // Preserve literal kernel strings: path normalization would alter their meaning.
    let record = child::run(
        &ChildSpec {
            tool: "lean",
            module: Some(driver.name),
            program: &toolchain.lake.path,
            executable_sha256: toolchain.lake.sha256,
            argv: vec![
                "env".to_owned(),
                toolchain.lean.path.to_string(),
                source.to_string(),
            ],
            cwd: &workspace,
            extra_env: Vec::new(),
            home: ChildHome::Toolchain {
                toolchain_bin: &bin,
            },
        },
        &limits,
        &Normalizer::default(),
    )
    .map_err(LexLeanError::from_diagnostic)?;
    if record.exit_code != 0 || !record.stderr.trim().is_empty() {
        return Err(refusal(format!(
            "kernel inspection failed: {}{}",
            record.stdout, record.stderr
        )));
    }
    // The owned adapter prechecks structural depth and total expansion before
    // serialization; the process cap also bounds this parser's input bytes.
    let mut decoder = serde_json::Deserializer::from_str(&record.stdout);
    decoder.disable_recursion_limit();
    let raw = RawReport::deserialize(&mut decoder)
        .map_err(|error| refusal(format!("kernel inspection report: {error}")))?;
    decoder
        .end()
        .map_err(|error| refusal(format!("noisy kernel inspection: {error}")))?;
    if raw.spec != "lexlean/kernel-inspection/1"
        || raw.lean.version != "4.32.1"
        || raw.lean.githash != "f054605aea4b840552cca2e725580bffd1e1b704"
        || raw.declarations.len() != request.declarations.len()
        || raw
            .declarations
            .iter()
            .zip(&request.declarations)
            .any(|(row, name)| row.name != structural_name(name))
    {
        return Err(refusal(
            "kernel inspection identity or declaration coverage mismatch",
        ));
    }
    let executable = std::env::current_exe()
        .map_err(|error| refusal(format!("inspection compiler identity: {error}")))?;
    let executable_bytes = std::fs::read(executable)
        .map_err(|error| refusal(format!("inspection compiler identity: {error}")))?;
    let declarations = raw
        .declarations
        .iter()
        .map(|row| {
            serde_json::json!({
                "name": row.name, "parameters": row.parameters, "type": row.ty,
                "native": super::inspection_core::convert(&row.ty, &row.parameters),
            })
        })
        .collect::<Vec<_>>();
    let report = serde_json::json!({
        "spec": raw.spec, "lean": raw.lean, "declarations": declarations,
        "request": request, "request_id": request_id.to_hex(),
        "compiler_semantics_id": crate::compiler_semantics_id_for(&project.config.language).to_hex(),
        "adapter_sha256": Sha256Digest::of(driver.text.as_bytes()).to_hex(),
        "lexlean_sha256": Sha256Digest::of(&executable_bytes).to_hex(),
        "lean_sha256": toolchain.lean.sha256.to_hex(),
        "lake_sha256": toolchain.lake.sha256.to_hex(),
        "process": {"exit_code": record.exit_code, "stdout_sha256": Sha256Digest::of(record.stdout.as_bytes()).to_hex(), "stderr_sha256": Sha256Digest::of(record.stderr.as_bytes()).to_hex()}
    });
    let mut bytes = serde_json::to_vec(&report).map_err(|error| refusal(error.to_string()))?;
    bytes.push(b'\n');
    if bytes.len() > 8388608 {
        return Err(refusal("kernel inspection report exceeds 8388608 bytes"));
    }
    Ok(bytes)
}
