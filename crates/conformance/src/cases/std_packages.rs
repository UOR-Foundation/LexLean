//! The standard library's source packages (SPEC.md §17.22, §10.5): `SL-03`,
//! `std-models`, and `SL-04`, `std-reasoning`. Both are projects of the
//! workspace that an importer pins; their behavior is held by theorems the
//! kernel checks, not by the Rust-std differential of `SL-01`.

use lexlean::config::LexiconSource;

use crate::support::{self, P};

/// One package of the standard library and what its case holds it to.
struct Package {
    /// The register ID.
    id: &'static str,
    /// The repository-relative project directory.
    directory: &'static str,
    /// The Lean module prefix its manifest states.
    prefix: &'static str,
    /// The one module it ships.
    module: &'static str,
    /// Declarations its module states, by name.
    declarations: &'static [&'static str],
}

const MODELS: Package = Package {
    id: "SL-03",
    directory: "std-models",
    prefix: "LexLeanStdModels",
    module: "Scores",
    declarations: &[
        "leaderStep",
        "best",
        "runnerStep",
        "leaders",
        "margin",
        "decideOr",
        "best_first_maximal",
        "margin_example",
        "margin_tie",
        "decide_or_commits",
        "decide_or_tie_abstains",
        "decide_or_single_abstains",
        "decide_or_empty_abstains",
        "Candidate",
        "isConfident",
        "verdictOf",
        "isCommitted",
        "Faithful",
        "Gate",
        "gate_faithful",
        "Gating",
        "GateEvidence",
        "GateModel",
    ],
};

const REASONING: Package = Package {
    id: "SL-04",
    directory: "std-reasoning",
    prefix: "LexLeanStdReasoning",
    module: "Worklist",
    declarations: &[
        "Spent",
        "Remaining",
        "Countdown",
        "Tick",
        "Burn",
        "Drained",
        "drain_bound",
        "Drain",
        "drain_clears_three",
        "drain_steps_three",
    ],
};

/// Run the case for `SL-03` or `SL-04`.
///
/// # Panics
///
/// Panics when the case's assertion fails.
pub(crate) fn run(id: &str) {
    match id {
        "SL-03" => package(&MODELS),
        "SL-04" => package(&REASONING),
        other => panic!("no standard-library package case is wired for {other}"),
    }
}

fn read(path: &str) -> String {
    std::fs::read_to_string(support::repo_root().join(path).as_std_path())
        .unwrap_or_else(|error| panic!("{path}: {error}"))
}

fn package(spec: &Package) {
    let root = support::repo_root();
    let directory = spec.directory;

    // The manifest of a package (SPEC.md §10.5): language 1.2, builtin
    // lexicons only, and no source package, artifact, or glossary of its own.
    let config = lexlean::config::parse_project(
        "lexlean.toml",
        read(&format!("{directory}/lexlean.toml")).as_bytes(),
    )
    .expect("the manifest parses");
    assert_eq!(config.language, "1.2", "{}", spec.id);
    assert_eq!(config.module_prefix, spec.prefix, "{}", spec.id);
    assert_eq!(
        config.entrypoints,
        [format!("src/{}.lex.tex", spec.module)],
        "{}: the package ships one module",
        spec.id
    );
    assert!(
        config.source_packages.is_empty() && config.artifact_sources.is_empty(),
        "{}: the package declares no source package and no artifact",
        spec.id
    );
    assert!(
        config
            .lexicon_sources
            .iter()
            .all(|source| matches!(source, LexiconSource::Builtin { .. })),
        "{}: the package declares builtin lexicons only",
        spec.id
    );
    assert!(config.pdf.is_none(), "{}: no [pdf]", spec.id);

    // It is a member of the workspace an importer reads, and the prefixes of
    // the workspace's projects are pairwise unrelated.
    let workspace = lexlean::source_package::Workspace::read(&root)
        .expect("the workspace file reads")
        .expect("the repository has a workspace file");
    assert!(
        workspace.members.iter().any(|member| member == directory),
        "{}: `{directory}` is a member of the workspace",
        spec.id
    );

    // No handwritten Lean: the package is source of the product and nothing
    // else, so no file of Lean source accompanies it.
    let mut files = 0usize;
    for entry in walkdir::WalkDir::new(root.join(directory).as_std_path())
        .into_iter()
        .flatten()
        .filter(|entry| entry.file_type().is_file())
    {
        files += 1;
        assert!(
            entry.path().extension().is_none_or(|extension| extension != "lean"),
            "{}: {} is Lean source",
            spec.id,
            entry.path().display()
        );
    }
    assert!(files > 0, "{}: the package has files", spec.id);

    // The declarations the statement names, and no composite: language 1.2
    // is generic over types and not over predicates or stages.
    let source = read(&format!("{directory}/src/{}.lex.tex", spec.module));
    for name in spec.declarations {
        assert!(
            source.contains(&format!("\n  name: {name},\n")),
            "{}: the module states `{name}`",
            spec.id
        );
    }
    assert!(
        !source.contains("composite{"),
        "{}: no composite takes an importer's models",
        spec.id
    );

    // The package alone checks, is canonical, and its committed lock is
    // current.
    let project = P::workspace_project(directory, &[directory]);
    project.fmt_check_ok();
    let (exit, _, stderr) = project.cli(&["lock", "--check"]);
    assert_eq!(exit, 0, "{}: the committed lock is current: {stderr}", spec.id);
    project.check_ok();

    // The falsifiable plant of each package, recorded in VERIFICATION.md.
    let planted = P::workspace_project(directory, &[directory]);
    let path = format!("src/{}.lex.tex", spec.module);
    let text = planted.read_raw(&path);
    let (site, replacement) = if spec.id == "SL-03" {
        // The leader replaced on a tie: the first maximal score is lost.
        (
            "(if (prim less_than (second (second state)) score : Bool) then",
            "(if (not (prim less_than score (second (second state)) : Bool)) then",
        )
    } else {
        // A bound the ranking does not meet.
        (
            "statement: ((Remaining ['T] x) < ((second x) + 1))",
            "statement: ((Remaining ['T] x) < (second x))",
        )
    };
    assert!(text.contains(site), "{}: the plant site exists", spec.id);
    planted.write_raw(&path, &text.replacen(site, replacement, 1));
    if spec.id == "SL-04" {
        planted.check_fails_with("LLT4010");
        planted.assert_no_backend_output_with("LLT4010");
    } else {
        planted.check_ok();
    }

    if !support::lean_backed(spec.id) {
        return;
    }
    if spec.id == "SL-03" {
        let _environment = support::env_shared();
        planted.verify_fails_with("LLV7002");
    }
    let verified = support::verify_ok(&project);
    let attestation: serde_json::Value = serde_json::from_slice(
        &std::fs::read(
            project
                .root
                .join(".lexlean/verified")
                .join(verified.attestation_id.to_hex())
                .join("attestation.json")
                .as_std_path(),
        )
        .expect("attestation"),
    )
    .expect("the attestation is JSON");
    let rows = attestation["declarations"].as_array().expect("declarations");
    for name in spec.declarations {
        let full = format!("{}.{}.{name}", spec.prefix, spec.module);
        let row = rows
            .iter()
            .find(|row| row["name"].as_str() == Some(full.as_str()))
            .unwrap_or_else(|| panic!("{}: `{full}` is audited", spec.id));
        assert!(
            row["observed"].as_array().is_some_and(|observed| observed
                .iter()
                .all(|axiom| matches!(axiom.as_str(), Some("propext" | "Quot.sound")))),
            "{}: `{full}` observes no axiom but propext and Quot.sound: {row}",
            spec.id
        );
    }

    if spec.id == "SL-04" {
        // An importing project instantiates the package's reasoner at its
        // own type.
        assert!(
            read("examples/helpdesk/src/Reply.lex.tex")
                .contains("LexLeanStdReasoning.Worklist::Drain [Router::Topic]"),
            "SL-04: the helpdesk instantiates `Drain` at its own `Topic`"
        );
    }
}
