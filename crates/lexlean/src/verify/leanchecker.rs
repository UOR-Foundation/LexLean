//! Separate-process `leanchecker` replay (SPEC.md §22.4): every generated
//! module is replayed and every replay must exit zero. The replay shares
//! Lean's kernel; it is not an independent verifier. Like every Lean
//! process it runs through `lake env` from the pinned toolchain (§22.2)
//! with the generated `.olean` root prepended to `LEAN_PATH`.

use camino::Utf8Path;

use crate::code;
use crate::config::Limits;
use crate::diagnostic::Diagnostic;
use crate::verify::child::{run, ChildHome, ChildRecord, ChildSpec, Normalizer};
use crate::verify::toolchain::Toolchain;

/// Run `lake env <leanchecker> <module>` (the preflighted executable by
/// absolute path) and return its record, whatever the exit status.
pub fn run_leanchecker(
    toolchain: &Toolchain,
    module: &str,
    lean_path: &str,
    workspace: &Utf8Path,
    limits: &Limits,
    normalizer: &Normalizer,
) -> Result<ChildRecord, Diagnostic> {
    let bin_dir = toolchain.root.join("bin");
    run(
        &ChildSpec {
            tool: "leanchecker",
            module: Some(module.to_owned()),
            program: &toolchain.lake.path,
            executable_sha256: toolchain.leanchecker.sha256,
            // The preflighted, digest-checked executable by absolute path,
            // inside the Lake environment (§22.2): `lake env` supplies
            // `LEAN_PATH`, never the executable.
            argv: vec![
                "env".to_owned(),
                toolchain.leanchecker.path.to_string(),
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
    )
}

/// Replay one compiled module through `leanchecker`; a nonzero exit is a
/// replay failure (LLV7003).
pub fn replay_module(
    toolchain: &Toolchain,
    module: &str,
    lean_path: &str,
    workspace: &Utf8Path,
    limits: &Limits,
    normalizer: &Normalizer,
) -> Result<ChildRecord, Diagnostic> {
    let record = run_leanchecker(toolchain, module, lean_path, workspace, limits, normalizer)?;
    if record.exit_code != 0 {
        return Err(Diagnostic::new(
            code!("LLV7003"),
            format!(
                "leanchecker replay of `{module}` exited {}: {}{}",
                record.exit_code,
                record.stdout.trim_end(),
                record.stderr.trim_end()
            ),
        ));
    }
    Ok(record)
}

/// Require a byte-bound replay for every compilation without its kernel pass.
///
/// The existing toolchain and partition gates establish executable identities
/// and judge stage output. This guard checks the association of those records:
/// module identity, successful replay, complete olean parts, and the staged
/// project artifacts. Certificates retain their bindings after scratch cleanup.
///
/// # Errors
/// A missing, failed, duplicate, malformed, or substituted binding (LLI9001).
pub fn check_replay_bindings(
    records: &[ChildRecord],
    staged_projects: &std::collections::BTreeMap<String, crate::verify::child::OLeanParts>,
    certificates: &std::collections::BTreeSet<String>,
) -> Result<(), Diagnostic> {
    let hole = |message| {
        Diagnostic::new(
            code!("LLI9001"),
            format!("phase verify: kernel replay binding: {message}"),
        )
    };
    let skipped = |record: &ChildRecord| {
        record.argv.iter().any(|argument| {
            argument == "debug.skipKernelTC=true" || argument == "-Ddebug.skipKernelTC=true"
        })
    };
    let mut compiled = std::collections::BTreeSet::new();
    for compile in records.iter().filter(|record| skipped(record)) {
        let module = compile
            .module
            .as_deref()
            .ok_or_else(|| hole("a flagged compilation has no module".to_owned()))?;
        if compile.tool != "lean"
            || (!staged_projects.contains_key(module) && !certificates.contains(module))
        {
            return Err(hole(format!(
                "`{module}` is not an eligible project or certificate compilation"
            )));
        }
        if !compiled.insert(module) || compile.exit_code != 0 {
            return Err(hole(format!(
                "`{module}` has a duplicate or unsuccessful flagged compilation"
            )));
        }
        let produced = compile
            .olean
            .as_ref()
            .ok_or_else(|| hole(format!("`{module}` has no compile output binding")))?;
        produced.validate().map_err(hole)?;
        let mut matching = records.iter().filter(|record| {
            record.tool == "leanchecker" && record.module.as_deref() == Some(module)
        });
        let replay = matching
            .next()
            .ok_or_else(|| hole(format!("`{module}` has no replay record")))?;
        if matching.next().is_some() || replay.exit_code != 0 {
            return Err(hole(format!(
                "`{module}` has a duplicate or unsuccessful replay"
            )));
        }
        if replay.argv.len() != 3 || replay.argv[0] != "env" || replay.argv[2] != module {
            return Err(hole(format!(
                "`{module}` replay argv does not name that module"
            )));
        }
        let consumed = replay
            .olean
            .as_ref()
            .ok_or_else(|| hole(format!("`{module}` has no replay input binding")))?;
        consumed.validate().map_err(hole)?;
        if produced != consumed {
            return Err(hole(format!(
                "`{module}` replay input differs from its compiled olean parts"
            )));
        }
        if let Some(staged) = staged_projects.get(module) {
            staged.validate().map_err(hole)?;
            if produced != staged {
                return Err(hole(format!(
                    "`{module}` staged artifacts differ from its compiled and replayed olean parts"
                )));
            }
        }
    }
    Ok(())
}
