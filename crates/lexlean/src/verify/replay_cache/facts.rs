//! Replay facts (SPEC.md §22.13): what the pinned Lean says about the
//! declarations of a compiled module, read as untrusted output.

use camino::Utf8Path;
use serde::Deserialize;

use crate::code;
use crate::config::Limits;
use crate::diagnostic::Diagnostic;
use crate::verify::child::{run, ChildHome, Normalizer};
use crate::verify::toolchain::Toolchain;

/// The program that exports the facts, run by the pinned `lean`.
const EXPORTER: &str = include_str!("../replay_facts.lean");

/// One declaration as the exporter states it.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeclarationFacts {
    /// The structured name, components joined by `.`.
    pub name: String,
    /// The module that declares it; no part of its canonical bytes.
    pub module: String,
    /// Universe parameter names, in order.
    pub level_params: Vec<String>,
    /// Kind and attributes, as the exporter wrote them.
    pub attrs: String,
    /// The node table of the type.
    #[serde(rename = "type")]
    pub type_text: String,
    /// The node table of the value or proof term, or of a recursor's rules.
    pub value: Option<String>,
    /// The declarations it names, sorted.
    pub deps: Vec<String>,
}

/// Read one JSON object per line. The exporter's output is untrusted: a line
/// that does not parse, or that has a member the exporter does not write, is
/// `LLV7020`.
pub fn parse_facts(text: &str) -> Result<Vec<DeclarationFacts>, Diagnostic> {
    let mut facts = Vec::new();
    for (number, line) in text.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        let declaration: DeclarationFacts = serde_json::from_str(line).map_err(|error| {
            Diagnostic::new(
                code!("LLV7020"),
                format!("replay facts line {} does not parse: {error}", number + 1),
            )
        })?;
        facts.push(declaration);
    }
    Ok(facts)
}

/// Export the facts of one compiled module by running the embedded exporter
/// under the pinned `lean`, through `lake env` like every Lean process
/// (§22.2), with `lean_path` as `LEAN_PATH`.
pub fn export_facts(
    toolchain: &Toolchain,
    module: &str,
    lean_path: &str,
    workspace: &Utf8Path,
    limits: &Limits,
    normalizer: &Normalizer,
) -> Result<Vec<DeclarationFacts>, Diagnostic> {
    let staging = tempfile::tempdir().map_err(|error| {
        Diagnostic::new(
            code!("LLV7010"),
            format!("cannot stage the replay facts exporter: {error}"),
        )
    })?;
    let script = staging.path().join("replay_facts.lean");
    std::fs::write(&script, EXPORTER).map_err(|error| {
        Diagnostic::new(
            code!("LLV7010"),
            format!("cannot stage the replay facts exporter: {error}"),
        )
    })?;
    let script = camino::Utf8PathBuf::from_path_buf(script)
        .map_err(|bad| crate::project::non_utf8_path(&bad))?;
    let bin_dir = toolchain.root.join("bin");
    let record = run(
        &crate::verify::child::ChildSpec {
            tool: "lean",
            module: Some(module.to_owned()),
            program: &toolchain.lake.path,
            executable_sha256: toolchain.lean.sha256,
            argv: vec![
                "env".to_owned(),
                toolchain.lean.path.to_string(),
                "--run".to_owned(),
                script.to_string(),
                module.to_owned(),
            ],
            cwd: workspace,
            extra_env: vec![("LEAN_PATH".to_owned(), lean_path.to_owned())],
            home: ChildHome::Toolchain {
                toolchain_bin: &bin_dir,
            },
        },
        limits,
        normalizer,
    )?;
    if record.exit_code != 0 {
        return Err(Diagnostic::new(
            code!("LLV7020"),
            format!(
                "the replay facts of `{module}` could not be exported (exit {}): {}",
                record.exit_code,
                record.stderr.trim_end()
            ),
        ));
    }
    parse_facts(&record.stdout)
}
