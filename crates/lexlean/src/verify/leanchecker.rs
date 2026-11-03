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
/// project and certificate artifacts before scratch cleanup.
///
/// # Errors
/// A missing, failed, duplicate, malformed, or substituted binding (LLI9001).
pub fn check_replay_bindings(
    records: &[ChildRecord],
    staged_artifacts: &std::collections::BTreeMap<String, crate::verify::child::OLeanParts>,
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
            || (!staged_artifacts.contains_key(module) && !certificates.contains(module))
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
        if let Some(staged) = staged_artifacts.get(module) {
            staged.validate().map_err(hole)?;
            if produced != staged {
                return Err(hole(format!(
                    "`{module}` staged artifacts differ from its compiled and replayed olean parts"
                )));
            }
        }
    }
    for certificate in certificates {
        if !staged_artifacts.contains_key(certificate) {
            return Err(hole(format!(
                "`{certificate}` has no staged certificate artifacts"
            )));
        }
    }
    for expected in staged_artifacts.keys().chain(certificates) {
        if !compiled.contains(expected.as_str()) {
            return Err(hole(format!(
                "`{expected}` has no flagged compilation record"
            )));
        }
    }
    Ok(())
}

/// Decode the probe's full, closed packet before any textual normalization.
/// Only path strings are normalized; raw roots remain operational lookup data.
pub(crate) fn decode_search_path(
    bytes: &[u8],
    normalizer: &Normalizer,
) -> Result<(String, Vec<camino::Utf8PathBuf>), Diagnostic> {
    use crate::artifact::canonical_json::Json;
    use std::collections::{BTreeMap, BTreeSet};
    let paths = search_path_packet(bytes)?;
    let known: Vec<String> = paths
        .iter()
        .map(|path| {
            if absolute_root(path.as_str()) {
                normalizer.normalize_arg(path.as_str())
            } else {
                path.to_string()
            }
        })
        .collect();
    let mut occupied: BTreeSet<String> = known.iter().cloned().collect();
    let mut aliases: BTreeMap<String, String> = BTreeMap::new();
    let mut ordinal = 0usize;
    let normalized = paths
        .iter()
        .zip(&known)
        .map(|(raw, known)| {
            if !absolute_root(known) {
                return Json::Str(known.clone());
            }
            let alias = aliases.entry(raw.to_string()).or_insert_with(|| loop {
                let candidate = format!("$LEAN_SEARCH_ROOT_{ordinal}");
                ordinal += 1;
                if occupied.insert(candidate.clone()) {
                    break candidate;
                }
            });
            Json::Str(alias.clone())
        })
        .collect();
    let packet = Json::object(vec![("lexlean_search_path", Json::Arr(normalized))]);
    Ok((
        String::from_utf8(packet.to_file_bytes()).expect("canonical JSON is UTF-8"),
        paths,
    ))
}

fn absolute_root(path: &str) -> bool {
    camino::Utf8Path::new(path).has_root()
        || path.starts_with("\\\\")
        || matches!(path.as_bytes(), [drive, b':', b'/' | b'\\', ..] if drive.is_ascii_alphabetic())
}

fn search_path_packet(bytes: &[u8]) -> Result<Vec<camino::Utf8PathBuf>, Diagnostic> {
    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Packet {
        lexlean_search_path: Vec<String>,
    }
    let noise = || {
        Diagnostic::new(code!("LLV7006"), "unexpected output during the external-interface probe: expected one closed search-path packet")
    };
    let packet: Packet = serde_json::from_slice(bytes).map_err(|_| noise())?;
    packet
        .lexlean_search_path
        .into_iter()
        .map(|path| {
            if path.contains('\0') {
                return Err(noise());
            }
            Ok(camino::Utf8PathBuf::from(path))
        })
        .collect()
}

/// Validate the normalized fixed probe packet by whole decoded roots.
/// Relative roots may contain spaces and slash fragments without being absolute.
pub(crate) fn audit_search_path_packet(bytes: &[u8]) -> Result<(), Diagnostic> {
    if search_path_packet(bytes)?
        .iter()
        .any(|path| absolute_root(path.as_str()))
    {
        return Err(Diagnostic::new(
            code!("LLV7006"),
            "unexpected absolute root in the external-interface probe packet",
        ));
    }
    Ok(())
}

/// Read the actual exported/server/private set; an empty file is present.
///
/// # Errors
/// Missing main bytes, unreadable parts, or a private part without its server.
pub fn capture_olean_parts(
    main: &Utf8Path,
) -> Result<crate::verify::child::OLeanParts, Diagnostic> {
    use crate::artifact::content_id::Sha256Digest;
    let read = |path: &Utf8Path| {
        std::fs::read(path.as_std_path())
            .map(|bytes| Sha256Digest::of(&bytes))
            .map_err(|error| {
                Diagnostic::new(
                    code!("LLI9001"),
                    format!("phase verify: reading replay artifact `{path}`: {error}"),
                )
            })
    };
    let companion = |suffix: &str| {
        let path = camino::Utf8PathBuf::from(format!("{main}.{suffix}"));
        match std::fs::metadata(path.as_std_path()) {
            Ok(_) => read(&path).map(Some),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(Diagnostic::new(
                code!("LLI9001"),
                format!("phase verify: reading replay artifact `{path}`: {error}"),
            )),
        }
    };
    let parts = crate::verify::child::OLeanParts {
        main: read(main)?,
        server: companion("server")?,
        private: companion("private")?,
    };
    parts.validate().map_err(|reason| {
        Diagnostic::new(
            code!("LLI9001"),
            format!("phase verify: `{main}`: {reason}"),
        )
    })?;
    Ok(parts)
}

/// Resolve the requested module by the pinned Lean first-package-root rule.
/// A selected root with a missing full file is an error, never a fallback.
///
/// # Errors
/// A missing package root or missing selected module file.
pub fn resolve_olean(
    module: &str,
    search_path: &[camino::Utf8PathBuf],
    workspace: &Utf8Path,
) -> Result<camino::Utf8PathBuf, Diagnostic> {
    let package = module.split('.').next().unwrap_or(module);
    for path in search_path {
        let root = if path.is_absolute() {
            path.clone()
        } else {
            workspace.join(path)
        };
        if root.join(package).is_dir() || root.join(format!("{package}.olean")).exists() {
            let selected = root.join(format!("{}.olean", module.replace('.', "/")));
            if !selected.is_file() {
                return Err(Diagnostic::new(code!("LLI9001"), format!("phase verify: the first package root selects missing replay input `{selected}` for `{module}`")));
            }
            return Ok(selected);
        }
    }
    Err(Diagnostic::new(
        code!("LLI9001"),
        format!("phase verify: no replay package root for `{module}`"),
    ))
}

/// Operational lookup context observed by the existing interface probe.
pub struct ReplayInput<'a> {
    /// Raw effective Lean search roots, in their original order.
    pub search_path: &'a [camino::Utf8PathBuf],
    /// The compiled artifact the replay must consume.
    pub staged: &'a Utf8Path,
}

/// Run the existing pinned checker while binding its requested actual input.
/// The checker may also replay descendant modules; it is not replaced here.
///
/// # Errors
/// Ordinary child errors, shadowed input, or parts changed during success.
pub fn run_bound_leanchecker(
    toolchain: &Toolchain,
    module: &str,
    lean_path: &str,
    workspace: &Utf8Path,
    limits: &Limits,
    normalizer: &Normalizer,
    input: ReplayInput<'_>,
) -> Result<ChildRecord, Diagnostic> {
    let resolved = resolve_olean(module, input.search_path, workspace)?;
    let expected = capture_olean_parts(input.staged)?;
    let before = capture_olean_parts(&resolved)?;
    if expected != before {
        return Err(Diagnostic::new(code!("LLI9001"), format!("phase verify: actual replay input of `{module}` differs from its staged compiled artifacts")));
    }
    let mut record = run_leanchecker(toolchain, module, lean_path, workspace, limits, normalizer)?;
    if record.exit_code == 0 {
        let after_path = resolve_olean(module, input.search_path, workspace)?;
        if after_path != resolved || capture_olean_parts(&after_path)? != before {
            return Err(Diagnostic::new(
                code!("LLI9001"),
                format!(
                    "phase verify: actual replay input parts of `{module}` changed during replay"
                ),
            ));
        }
    }
    record.olean = Some(before);
    Ok(record)
}

/// Replay a generated module with the existing LLV7003 failure classification.
///
/// # Errors
/// A failed replay or an invalid input binding.
pub fn replay_bound_module(
    toolchain: &Toolchain,
    module: &str,
    lean_path: &str,
    workspace: &Utf8Path,
    limits: &Limits,
    normalizer: &Normalizer,
    input: ReplayInput<'_>,
) -> Result<ChildRecord, Diagnostic> {
    let record = run_bound_leanchecker(
        toolchain, module, lean_path, workspace, limits, normalizer, input,
    )?;
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

#[cfg(test)]
mod path_binding_tests {
    use super::*;
    use crate::artifact::canonical_json::Json;
    use camino::Utf8PathBuf;

    fn packet(paths: &[&str]) -> Vec<u8> {
        Json::object(vec![(
            "lexlean_search_path",
            Json::Arr(
                paths
                    .iter()
                    .map(|path| Json::Str((*path).to_owned()))
                    .collect(),
            ),
        )])
        .to_file_bytes()
    }

    #[test]
    fn probe_packet_preserves_escaped_relative_roots_and_aliases_unknown_roots() {
        let normalizer = Normalizer::new(
            Utf8Path::new("/stage"),
            Utf8Path::new("/project"),
            Utf8Path::new("/lake"),
            Utf8Path::new("/tool"),
        );
        let input = [
            "rel /segment",
            "line\nbreak\"\\root",
            "$LEAN_SEARCH_ROOT_0",
            "/unknown",
            "/unknown",
            "/other",
            "/stage/oleans",
        ];
        let (normalized, raw) = decode_search_path(&packet(&input), &normalizer).unwrap();
        assert_eq!(raw, input.map(Utf8PathBuf::from));
        let expected = packet(&[
            input[0],
            input[1],
            input[2],
            "$LEAN_SEARCH_ROOT_1",
            "$LEAN_SEARCH_ROOT_1",
            "$LEAN_SEARCH_ROOT_2",
            "$STAGING/oleans",
        ]);
        assert_eq!(normalized.as_bytes(), expected);
        audit_search_path_packet(normalized.as_bytes()).unwrap();
        assert!(normalizer.has_unexpected_absolute_path(&normalized));
        assert!(audit_search_path_packet(&packet(&["/unredacted"])).is_err());
    }

    #[test]
    fn probe_packet_rejects_noise_and_nonclosed_shapes() {
        assert!(audit_search_path_packet(
            br#"{"lexlean_search_path":[],"lexlean_search_\u0070ath":[]}"#
        )
        .is_err());
        for bytes in [
            b"{}".as_slice(),
            b"{\"lexlean_search_path\":[],\"extra\":0}",
            b"{\"lexlean_search_path\":[1]}",
            b"{\"lexlean_search_path\":[],\"lexlean_search_path\":[]}",
            b"{\"lexlean_search_path\":[]}\nwarning",
            b"warning\n{\"lexlean_search_path\":[]}",
        ] {
            assert!(audit_search_path_packet(bytes).is_err());
            assert!(decode_search_path(bytes, &Normalizer::default()).is_err());
        }
    }

    #[test]
    fn first_package_root_does_not_fall_through_missing_module() {
        let temp = tempfile::tempdir().unwrap();
        let root = Utf8PathBuf::from_path_buf(temp.path().to_owned()).unwrap();
        std::fs::create_dir_all(root.join("first/A")).unwrap();
        std::fs::create_dir_all(root.join("second/A")).unwrap();
        std::fs::write(root.join("second/A/B.olean"), b"later").unwrap();
        let roots = [Utf8PathBuf::from("first"), Utf8PathBuf::from("second")];
        assert!(resolve_olean("A.B", &roots, &root).is_err());
        std::fs::write(root.join("first/A/B.olean"), b"first").unwrap();
        assert_eq!(
            resolve_olean("A.B", &roots, &root).unwrap(),
            root.join("first/A/B.olean")
        );
    }

    #[test]
    fn capture_distinguishes_absent_empty_and_changed_companions() {
        let temp = tempfile::tempdir().unwrap();
        let main = Utf8PathBuf::from_path_buf(temp.path().join("A.olean")).unwrap();
        std::fs::write(&main, b"main").unwrap();
        let absent = capture_olean_parts(&main).unwrap();
        std::fs::write(format!("{main}.private"), b"").unwrap();
        assert!(capture_olean_parts(&main).is_err());
        std::fs::write(format!("{main}.server"), b"").unwrap();
        let empty = capture_olean_parts(&main).unwrap();
        assert_ne!(empty, absent);
        std::fs::write(format!("{main}.server"), b"changed").unwrap();
        assert_ne!(capture_olean_parts(&main).unwrap(), empty);
    }
}
