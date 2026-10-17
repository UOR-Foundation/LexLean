//! The parts of one verification (SPEC.md §9.4, §22.11).
//!
//! A verification is one sequence of processes: the probe, the elaboration of
//! every module in topological frontiers, the replay of every module, and the
//! audit member of every module. The normative CI gate may run that sequence
//! as several parts on several hosts of one run: a *shard* elaborates,
//! replays, and audits the modules its plan names, reading the `.olean`
//! files of the modules it imports from the parts before it, and writes what
//! it ran to an output directory; the *join* runs its own plan, takes the
//! `.olean` files and the process records of every part, re-judges each
//! record as the stage that produced it did, and then continues through
//! policy, the attestation, and publication exactly as the whole
//! verification does. Every process of the verification runs exactly once
//! across the parts, and the attestation the join publishes is a function of
//! the same records in the same canonical order as a whole verification's.
//!
//! Nothing here is a cache: a part reads only what the parts of the same run
//! wrote, and a module without a record from any part fails the join.

use std::collections::{BTreeMap, BTreeSet};

use camino::{Utf8Path, Utf8PathBuf};

use crate::artifact::canonical_json::Json;
use crate::artifact::content_id::Sha256Digest;
use crate::diagnostic::Diagnostic;
use crate::verify::child::ChildRecord;

/// Which modules one part elaborates, and which it replays and audits.
///
/// Both sets hold full Lean module names (the labels of the verifier's
/// processes, §22.11). A module replayed by a part must have been elaborated
/// by that part or by a part whose output it imports.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Plan {
    /// The modules this part elaborates (§22.3).
    pub elaborate: BTreeSet<String>,
    /// The modules this part replays (§22.4) and audits (§22.5).
    pub replay: BTreeSet<String>,
}

/// What one run of the verifier is.
pub enum Part<'a> {
    /// The whole verification: every process, then the attestation.
    Whole,
    /// One shard: the plan's processes, written to `output`, nothing
    /// published.
    Shard {
        /// The processes of this shard.
        plan: &'a Plan,
        /// The output directories of the parts whose `.olean` files this
        /// shard imports.
        imports: &'a [Utf8PathBuf],
        /// Where this shard writes its `.olean` files and process records.
        output: &'a Utf8Path,
    },
    /// The join: the plan's processes, every other part's records, then the
    /// attestation and publication.
    Join {
        /// The processes of the join itself.
        plan: &'a Plan,
        /// The output directories of every other part.
        parts: &'a [Utf8PathBuf],
    },
}

impl Part<'_> {
    /// The plan, when this is a part and not the whole.
    #[must_use]
    pub fn plan(&self) -> Option<&Plan> {
        match self {
            Self::Whole => None,
            Self::Shard { plan, .. } | Self::Join { plan, .. } => Some(plan),
        }
    }

    /// The output directories this part reads.
    #[must_use]
    pub fn inputs(&self) -> &[Utf8PathBuf] {
        match self {
            Self::Whole => &[],
            Self::Shard { imports, .. } => imports,
            Self::Join { parts, .. } => parts,
        }
    }

    /// Does this part publish a verified directory?
    #[must_use]
    pub fn publishes(&self) -> bool {
        matches!(self, Self::Whole | Self::Join { .. })
    }

    /// Is a module one this part elaborates?
    #[must_use]
    pub fn elaborates(&self, lean_module: &str) -> bool {
        self.plan()
            .is_none_or(|plan| plan.elaborate.contains(lean_module))
    }

    /// Is a module one this part replays and audits?
    #[must_use]
    pub fn replays(&self, lean_module: &str) -> bool {
        self.plan()
            .is_none_or(|plan| plan.replay.contains(lean_module))
    }
}

/// What earlier parts of the run wrote, read back.
#[derive(Debug, Default)]
pub struct Imported {
    /// The modules whose `.olean` arrived, by full Lean module name.
    pub oleans: BTreeSet<String>,
    /// Elaboration records by module.
    pub lean: BTreeMap<String, ChildRecord>,
    /// Replay records by module.
    pub leanchecker: BTreeMap<String, ChildRecord>,
    /// Audit records by audit member name.
    pub audit: BTreeMap<String, ChildRecord>,
}

fn io(what: &str, error: &std::io::Error) -> Diagnostic {
    Diagnostic::new(
        crate::code!("LLB6003"),
        format!("verification part: {what}: {error}"),
    )
}

fn malformed(what: &str, reason: impl std::fmt::Display) -> Diagnostic {
    Diagnostic::new(
        crate::code!("LLI9001"),
        format!("phase verify: verification part: {what}: {reason}"),
    )
}

/// Copy the `.olean` tree and, when asked, read the process records of each
/// part in `parts` into the staging tree at `olean_root`.
///
/// A module's `.olean` may come from one part only, and a record of one
/// process from one part only: two parts that both wrote one is a plan that
/// ran a process twice, which is refused rather than resolved.
///
/// # Errors
///
/// A part that cannot be read, a duplicate `.olean` or record, or a record
/// that does not parse as the verifier wrote it.
pub fn import(
    parts: &[Utf8PathBuf],
    olean_root: &Utf8Path,
    with_records: bool,
) -> Result<Imported, Diagnostic> {
    let mut imported = Imported::default();
    for part in parts {
        if !part.as_std_path().is_dir() {
            return Err(malformed(
                part.as_str(),
                "the output directory of a part is required and is not a directory",
            ));
        }
        let oleans = part.join("oleans");
        if oleans.as_std_path().is_dir() {
            for entry in walkdir::WalkDir::new(oleans.as_std_path())
                .sort_by_file_name()
                .into_iter()
            {
                let entry = entry.map_err(|error| {
                    malformed(oleans.as_str(), format!("reading the olean tree: {error}"))
                })?;
                if !entry.file_type().is_file() {
                    continue;
                }
                let relative = entry
                    .path()
                    .strip_prefix(oleans.as_std_path())
                    .map_err(|error| malformed(oleans.as_str(), error))?;
                let relative = Utf8Path::from_path(relative).ok_or_else(|| {
                    malformed(oleans.as_str(), "a non-UTF-8 path in the olean tree")
                })?;
                let destination = olean_root.join(relative);
                if destination.as_std_path().exists() {
                    return Err(malformed(
                        part.as_str(),
                        format!("`oleans/{relative}` was written by two parts of the run"),
                    ));
                }
                if let Some(parent) = destination.parent() {
                    std::fs::create_dir_all(parent.as_std_path())
                        .map_err(|error| io("staging an imported olean", &error))?;
                }
                std::fs::copy(entry.path(), destination.as_std_path())
                    .map_err(|error| io("copying an imported olean", &error))?;
                if relative.extension() == Some("olean") {
                    let module = relative
                        .with_extension("")
                        .as_str()
                        .replace(['/', '\\'], ".");
                    imported.oleans.insert(module);
                }
            }
        }
        if !with_records {
            continue;
        }
        read_records(
            &part.join("process").join("lean"),
            ".json",
            &mut imported.lean,
            part,
        )?;
        read_records(
            &part.join("process").join("leanchecker"),
            ".json",
            &mut imported.leanchecker,
            part,
        )?;
        read_records(
            &part.join("audit"),
            ".process.json",
            &mut imported.audit,
            part,
        )?;
    }
    Ok(imported)
}

/// Require every imported record to name the executable the join itself
/// runs: `lean` for an elaboration or an audit member, `leanchecker` for a
/// replay (§9.4, §22.9).
///
/// In a whole verification every record carries the digest of the toolchain
/// the run's preflight hashed, by construction. Across the parts of a run it
/// holds only because every job installs the same pinned toolchain, which is
/// a practice and not a check: a part whose `lean` or `leanchecker` differed
/// from the join's would otherwise contribute records the attestation then
/// lists under the join's toolchain digests.
///
/// # Errors
///
/// The first record, in the order elaborations, replays, audit members, and
/// by name within each, whose `executable_sha256` is not the join's digest of
/// the tool that produced it, naming the record and both digests.
pub fn check_toolchain(
    imported: &Imported,
    lean: Sha256Digest,
    leanchecker: Sha256Digest,
) -> Result<(), Diagnostic> {
    for (what, tool, expected, records) in [
        ("elaboration", "lean", lean, &imported.lean),
        ("replay", "leanchecker", leanchecker, &imported.leanchecker),
        ("audit", "lean", lean, &imported.audit),
    ] {
        for (name, record) in records {
            if record.executable_sha256 != expected {
                return Err(malformed(
                    &format!("the imported {what} record of `{name}`"),
                    format!(
                        "its executable_sha256 is {}, but the `{tool}` of this run's toolchain is {}; every part of a run must run the join's pinned toolchain",
                        record.executable_sha256.to_hex(),
                        expected.to_hex()
                    ),
                ));
            }
        }
    }
    Ok(())
}

/// Read every `<name><suffix>` of `directory` as a process record keyed by
/// `<name>`.
fn read_records(
    directory: &Utf8Path,
    suffix: &str,
    into: &mut BTreeMap<String, ChildRecord>,
    part: &Utf8Path,
) -> Result<(), Diagnostic> {
    if !directory.as_std_path().is_dir() {
        return Ok(());
    }
    let mut names: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(directory.as_std_path())
        .map_err(|error| io("reading a part's records", &error))?
    {
        let entry = entry.map_err(|error| io("reading a part's records", &error))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if let Some(stem) = name.strip_suffix(suffix) {
            names.push(stem.to_owned());
        }
    }
    names.sort();
    for name in names {
        let path = directory.join(format!("{name}{suffix}"));
        let bytes = std::fs::read(path.as_std_path())
            .map_err(|error| io("reading a process record", &error))?;
        let json = Json::parse(&bytes).map_err(|reason| malformed(path.as_str(), reason))?;
        let record =
            ChildRecord::from_json(&json).map_err(|reason| malformed(path.as_str(), reason))?;
        if record.module.as_deref() != Some(name.as_str()) {
            return Err(malformed(
                path.as_str(),
                "the record's module is not the one its file names",
            ));
        }
        if into.insert(name.clone(), record).is_some() {
            return Err(malformed(
                part.as_str(),
                format!("the record of `{name}` was written by two parts of the run"),
            ));
        }
    }
    Ok(())
}

/// Move what a shard ran out of its staging tree into `output`: the
/// `.olean` files of the modules in `own`, with their module-system
/// intermediates (a later part's elaboration reads them), the elaboration
/// and replay records, and the audit records. The `.olean` files the shard
/// imported stay behind: they are the part that produced them's to export,
/// once. Nothing else leaves the staging tree, which is removed with it.
///
/// # Errors
///
/// An output directory that exists and is not empty, or a move that fails.
pub fn export(
    staging: &std::path::Path,
    output: &Utf8Path,
    own: &BTreeSet<String>,
) -> Result<(), Diagnostic> {
    if output.as_std_path().exists() {
        let mut entries = std::fs::read_dir(output.as_std_path())
            .map_err(|error| io("reading the part's output directory", &error))?;
        if entries.next().is_some() {
            return Err(malformed(
                output.as_str(),
                "the output directory of a part must be empty",
            ));
        }
    }
    std::fs::create_dir_all(output.as_std_path())
        .map_err(|error| io("creating the part's output directory", &error))?;
    let oleans = staging.join("oleans");
    if oleans.is_dir() {
        for entry in walkdir::WalkDir::new(&oleans).sort_by_file_name() {
            let entry = entry.map_err(|error| malformed("oleans", error))?;
            if !entry.file_type().is_file() {
                continue;
            }
            let relative = entry.path().strip_prefix(&oleans).unwrap_or(entry.path());
            // `X/M.olean`, `X/M.olean.private`, `X/M.ir`: the module is the
            // path up to the file name's first dot.
            let module = {
                let text = relative.to_string_lossy().replace('\\', "/");
                let stem = text.split_once('.').map_or(text.as_str(), |(stem, _)| stem);
                stem.replace('/', ".")
            };
            if !own.contains(&module) {
                continue;
            }
            let destination =
                output
                    .join("oleans")
                    .join(Utf8Path::from_path(relative).ok_or_else(|| {
                        malformed("oleans", "a non-UTF-8 path in the olean tree")
                    })?);
            if let Some(parent) = destination.parent() {
                std::fs::create_dir_all(parent.as_std_path())
                    .map_err(|error| io("creating the part's olean directory", &error))?;
            }
            move_file(entry.path(), destination.as_std_path())?;
        }
    }
    let process = staging.join("process");
    if process.is_dir() {
        move_tree(&process, output.join("process").as_std_path())?;
    }
    let audit = staging.join("audit");
    if audit.is_dir() {
        let to = output.join("audit");
        std::fs::create_dir_all(to.as_std_path())
            .map_err(|error| io("creating the part's audit directory", &error))?;
        for entry in
            std::fs::read_dir(&audit).map_err(|error| io("reading the audit records", &error))?
        {
            let entry = entry.map_err(|error| io("reading the audit records", &error))?;
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.ends_with(".process.json") {
                move_file(&entry.path(), to.join(&name).as_std_path())?;
            }
        }
    }
    Ok(())
}

fn move_tree(from: &std::path::Path, to: &std::path::Path) -> Result<(), Diagnostic> {
    if std::fs::rename(from, to).is_ok() {
        return Ok(());
    }
    // Across file systems a rename fails; copy file by file and remove.
    for entry in walkdir::WalkDir::new(from).into_iter().flatten() {
        let relative = entry.path().strip_prefix(from).unwrap_or(entry.path());
        let destination = to.join(relative);
        if entry.file_type().is_dir() {
            std::fs::create_dir_all(&destination)
                .map_err(|error| io("copying the part's output", &error))?;
        } else if entry.file_type().is_file() {
            if let Some(parent) = destination.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|error| io("copying the part's output", &error))?;
            }
            std::fs::copy(entry.path(), &destination)
                .map_err(|error| io("copying the part's output", &error))?;
        }
    }
    std::fs::remove_dir_all(from).map_err(|error| io("removing the staged copy", &error))
}

fn move_file(from: &std::path::Path, to: &std::path::Path) -> Result<(), Diagnostic> {
    if std::fs::rename(from, to).is_ok() {
        return Ok(());
    }
    std::fs::copy(from, to).map_err(|error| io("copying a part's record", &error))?;
    std::fs::remove_file(from).map_err(|error| io("removing the staged copy", &error))
}

/// The order in which a whole verification consumes its elaborations (§22.11):
/// topological frontiers of the import graph, the members of each in module
/// order. A part's attestation lists its elaboration records in this order
/// whichever part ran them.
///
/// `imports(index)` gives the project module names a module imports;
/// `name(index)` its project module name.
///
/// # Errors
///
/// A graph that does not converge (an import cycle, or an import of no
/// module), as the elaboration stage reports it.
pub fn elaboration_order(
    count: usize,
    name: &dyn Fn(usize) -> String,
    imports: &dyn Fn(usize) -> Vec<String>,
) -> Result<Vec<Vec<usize>>, Diagnostic> {
    let mut placed: BTreeSet<String> = BTreeSet::new();
    let mut remaining: Vec<usize> = (0..count).collect();
    let mut frontiers = Vec::new();
    while !remaining.is_empty() {
        let ready: Vec<usize> = remaining
            .iter()
            .copied()
            .filter(|index| imports(*index).iter().all(|import| placed.contains(import)))
            .collect();
        if ready.is_empty() {
            return Err(malformed("module order", "module order did not converge"));
        }
        for &index in &ready {
            placed.insert(name(index));
        }
        remaining.retain(|index| !ready.contains(index));
        frontiers.push(ready);
    }
    Ok(frontiers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_plan_names_what_a_part_runs_and_the_whole_runs_everything() {
        let plan = Plan {
            elaborate: ["A.M".to_owned()].into_iter().collect(),
            replay: ["A.N".to_owned()].into_iter().collect(),
        };
        let shard = Part::Shard {
            plan: &plan,
            imports: &[],
            output: Utf8Path::new("out"),
        };
        assert!(shard.elaborates("A.M") && !shard.elaborates("A.N"));
        assert!(shard.replays("A.N") && !shard.replays("A.M"));
        assert!(!shard.publishes());
        let join = Part::Join {
            plan: &plan,
            parts: &[],
        };
        assert!(join.publishes());
        assert!(Part::Whole.elaborates("anything") && Part::Whole.replays("anything"));
        assert!(Part::Whole.publishes() && Part::Whole.inputs().is_empty());
    }

    #[test]
    fn the_elaboration_order_is_the_frontiers_in_module_order() {
        // 0 imports nothing, 1 imports 0, 2 imports nothing, 3 imports 1 and 2.
        let names = ["A", "B", "C", "D"].map(str::to_owned);
        let imports = |index: usize| -> Vec<String> {
            match index {
                1 => vec!["A".to_owned()],
                3 => vec!["B".to_owned(), "C".to_owned()],
                _ => vec![],
            }
        };
        let order = elaboration_order(4, &|index| names[index].clone(), &imports).expect("order");
        assert_eq!(order, vec![vec![0, 2], vec![1], vec![3]]);
        let cyclic = |index: usize| -> Vec<String> {
            match index {
                0 => vec!["B".to_owned()],
                1 => vec!["A".to_owned()],
                _ => vec![],
            }
        };
        assert!(elaboration_order(2, &|index| names[index].clone(), &cyclic).is_err());
    }

    #[test]
    fn a_part_output_round_trips_its_oleans_and_records() {
        let staging = tempfile::tempdir().expect("tempdir");
        let olean = staging.path().join("oleans/X/M.olean");
        std::fs::create_dir_all(olean.parent().expect("parent")).expect("mkdir");
        std::fs::write(&olean, b"olean").expect("write");
        std::fs::write(staging.path().join("oleans/X/M.olean.private"), b"private").expect("write");
        std::fs::write(staging.path().join("oleans/X/N.olean"), b"imported").expect("write");
        let record = ChildRecord {
            tool: "lean".to_owned(),
            module: Some("X.M".to_owned()),
            argv: vec!["env".to_owned(), "lean".to_owned()],
            exit_code: 0,
            stdout: "\n".to_owned(),
            stderr: String::new(),
            executable_sha256: crate::artifact::content_id::Sha256Digest::of(b"lean"),
        };
        let audit_record = ChildRecord {
            module: Some("LexLeanAudit.X.M".to_owned()),
            ..record.clone()
        };
        for (relative, written) in [
            ("process/lean/X.M.json", &record),
            ("audit/LexLeanAudit.X.M.process.json", &audit_record),
        ] {
            let path = staging.path().join(relative);
            std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
            std::fs::write(&path, written.to_json().to_file_bytes()).expect("write");
        }
        std::fs::write(
            staging.path().join("audit/LexLeanAudit.X.M.lean"),
            b"source",
        )
        .expect("write");
        let out = tempfile::tempdir().expect("tempdir");
        let output = Utf8PathBuf::from_path_buf(out.path().join("part")).expect("utf8");
        let own: BTreeSet<String> = ["X.M".to_owned()].into_iter().collect();
        export(staging.path(), &output, &own).expect("exports");
        assert!(
            staging.path().join("oleans/X/N.olean").exists()
                && !staging.path().join("oleans/X/M.olean").exists(),
            "an imported olean stays behind and an own one leaves"
        );
        assert!(
            staging.path().join("audit/LexLeanAudit.X.M.lean").exists(),
            "the audit source stays with the staging tree"
        );
        let into = tempfile::tempdir().expect("tempdir");
        let olean_root = Utf8PathBuf::from_path_buf(into.path().join("oleans")).expect("utf8");
        let imported = import(std::slice::from_ref(&output), &olean_root, true).expect("imports");
        assert_eq!(imported.oleans, ["X.M".to_owned()].into_iter().collect());
        assert!(olean_root.join("X/M.olean.private").as_std_path().is_file());
        assert_eq!(imported.lean.len(), 1);
        assert!(imported.lean.contains_key("X.M"));
        assert!(imported.leanchecker.is_empty());
        assert!(imported.audit.contains_key("LexLeanAudit.X.M"));
        // The records name the join's `lean`; a join whose `lean` is another
        // executable refuses them, naming the record and both digests.
        let lean = crate::artifact::content_id::Sha256Digest::of(b"lean");
        let other = crate::artifact::content_id::Sha256Digest::of(b"another lean");
        check_toolchain(&imported, lean, other).expect("the records name the join's lean");
        let refused = check_toolchain(&imported, other, other)
            .expect_err("a record of another lean is refused");
        let message = format!("{refused:?}");
        assert!(
            message.contains("LLI9001")
                && message.contains("elaboration record of `X.M`")
                && message.contains(&lean.to_hex())
                && message.contains(&other.to_hex()),
            "{message}"
        );
        // The same part imported twice is a process that ran twice.
        let again = import(&[output.clone(), output.clone()], &olean_root, false);
        assert!(again.is_err(), "a duplicate olean is refused");
        // A record whose module is not the one its file names is refused.
        let wrong = out.path().join("wrong/process/lean/X.N.json");
        std::fs::create_dir_all(wrong.parent().expect("parent")).expect("mkdir");
        std::fs::write(&wrong, record.to_json().to_file_bytes()).expect("write");
        let wrong_part = Utf8PathBuf::from_path_buf(out.path().join("wrong")).expect("utf8");
        let fresh = tempfile::tempdir().expect("tempdir");
        let fresh_root = Utf8PathBuf::from_path_buf(fresh.path().join("oleans")).expect("utf8");
        assert!(import(std::slice::from_ref(&wrong_part), &fresh_root, true).is_err());
    }
}
