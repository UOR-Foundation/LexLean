//! `BS-33` (SPEC.md §10.1, §17.23): the canonical text of the project
//! manifest and its digest as language-1.2 source, `LexLeanBoot.ProjectText`,
//! held to the byte to `lexlean::config::ProjectConfig::{canonical_toml,
//! config_sha256}`.
//!
//! `canonicalProject` writes a configuration as `lexlean lock` canonicalizes
//! it: the ten header keys in their fixed order with the pinned Lean
//! toolchain, the lexicon sources sorted by package (or the empty array when
//! there is none), the artifact sources and the source packages in their
//! order, the eleven limits, and the PDF provider; `configSha256` is the
//! lower-case hex SHA-256 of that text. It writes through the TOML writers of
//! `BS-31`'s `LockText`. Every committed `lexlean.toml` that parses is a run.
//!
//! Each planted defect names the one vector that catches it.

use std::collections::{BTreeMap, BTreeSet};

use lexlean::calculus::Value;
use lexlean::config::{
    parse_project, ArtifactSource, LexiconSource, Limits, PdfProvider, ProjectConfig, SourcePackage,
};
use lexlean::Sha256Digest;
use serde_json::Value as Json2;

use super::rust_validate::{eligibility_of, rustc_differential};
use super::{bytes, list, option, text, texts, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.ProjectText";
const ROOTS: [(&str, &[&str]); 2] = [
    ("canonicalProject", &["allocation", "overflow"]),
    ("configSha256", &["allocation", "overflow"]),
];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// Seeded runs of each root under the evaluator, and under rustc.
const EVALUATOR_SEEDED: usize = 200;
const RUSTC_SEEDED: usize = 1_000;

// ------------------------------------------------------------- the encoders

fn nat(value: u64) -> Value {
    Value::Nat {
        value: value.to_string(),
    }
}

fn adt(constructor: u64, fields: Vec<Value>) -> Value {
    Value::Adt {
        constructor,
        fields,
    }
}

fn enc_source(source: &LexiconSource) -> Value {
    match source {
        LexiconSource::Builtin { package } => adt(0, vec![text(package)]),
        LexiconSource::Path { package, path } => adt(1, vec![text(package), text(path)]),
        LexiconSource::Git {
            package,
            url,
            revision,
            subdirectory,
        } => adt(
            2,
            vec![text(package), text(url), text(revision), text(subdirectory)],
        ),
    }
}

fn enc_limits(limits: &Limits) -> Value {
    adt(
        0,
        vec![
            nat(limits.max_file_bytes),
            nat(limits.max_total_source_bytes),
            nat(limits.max_primitive_atoms),
            nat(limits.max_token_lattice_edges),
            nat(limits.max_parse_states),
            nat(limits.max_ir_nodes),
            nat(limits.max_scope_depth),
            nat(limits.max_import_depth),
            nat(limits.max_diagnostics),
            nat(limits.max_child_output_bytes),
            nat(limits.child_timeout_ms),
        ],
    )
}

fn enc_pdf(pdf: &PdfProvider) -> Value {
    adt(
        0,
        vec![
            text(&pdf.program),
            bytes(&pdf.program_sha256.0),
            texts(&pdf.version_argv),
            bytes(&pdf.version_stdout_sha256.0),
            texts(&pdf.compile_argv),
            text(&pdf.output),
            texts(&pdf.resources),
        ],
    )
}

fn enc(config: &ProjectConfig) -> Value {
    adt(
        0,
        vec![
            text(&config.name),
            text(&config.language),
            text(&config.module_prefix),
            texts(&config.source_roots),
            texts(&config.entrypoints),
            text(&config.build_root),
            text(&config.lockfile),
            text(&config.lean_workspace),
            list(config.lexicon_sources.iter().map(enc_source).collect()),
            list(
                config
                    .artifact_sources
                    .iter()
                    .map(|artifact| adt(0, vec![bytes(&artifact.sha256.0), text(&artifact.path)]))
                    .collect(),
            ),
            list(
                config
                    .source_packages
                    .iter()
                    .map(|package| {
                        adt(
                            0,
                            vec![
                                text(&package.package),
                                text(&package.version),
                                text(&package.path),
                            ],
                        )
                    })
                    .collect(),
            ),
            enc_limits(&config.limits),
            option(config.pdf.as_ref().map(enc_pdf)),
        ],
    )
}

fn text_run(config: &ProjectConfig) -> Run {
    (
        vec![enc(config)],
        serde_json::to_value(text(&config.canonical_toml())).expect("a value is JSON"),
    )
}

fn digest_run(config: &ProjectConfig) -> Run {
    (
        vec![enc(config)],
        serde_json::to_value(text(&config.config_sha256().to_hex())).expect("a value is JSON"),
    )
}

// ------------------------------------------------------------------ corpora

fn manifests_under(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(std::fs::DirEntry::file_name);
    for entry in entries {
        let name = entry.file_name().to_string_lossy().into_owned();
        let path = entry.path();
        if path.is_dir() {
            if !matches!(
                name.as_str(),
                ".git" | "target" | ".lexlean" | "node_modules"
            ) {
                manifests_under(&path, out);
            }
        } else if name == "lexlean.toml" {
            out.push(path);
        }
    }
}

/// Every committed manifest that parses, how many committed manifests there
/// are, and how many of those that parse are their own canonical text.
fn committed() -> (Vec<ProjectConfig>, usize, usize) {
    let mut paths = Vec::new();
    manifests_under(repo_root().as_std_path(), &mut paths);
    let found = paths.len();
    let mut canonical = 0usize;
    let configs = paths
        .iter()
        .filter_map(|path| {
            let bytes = std::fs::read(path).expect("a committed manifest");
            let config = parse_project(&path.to_string_lossy(), &bytes).ok()?;
            if config.canonical_toml().as_bytes() == bytes.as_slice() {
                canonical += 1;
            }
            Some(config)
        })
        .collect();
    (configs, found, canonical)
}

// ------------------------------------------------------------------ vectors

fn sha(byte: u8) -> Sha256Digest {
    let mut out = [0u8; 32];
    for (index, slot) in out.iter_mut().enumerate() {
        *slot = byte.wrapping_add(u8::try_from(index).expect("a byte"));
    }
    Sha256Digest(out)
}

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| (*item).to_owned()).collect()
}

fn limits() -> Limits {
    Limits {
        max_file_bytes: 4_194_304,
        max_total_source_bytes: 67_108_864,
        max_primitive_atoms: 1_000_000,
        max_token_lattice_edges: 2_000_000,
        max_parse_states: 3_000_000,
        max_ir_nodes: 4_000_000,
        max_scope_depth: 256,
        max_import_depth: 64,
        max_diagnostics: 100,
        max_child_output_bytes: 16_777_216,
        child_timeout_ms: 300_000,
    }
}

fn minimal() -> ProjectConfig {
    ProjectConfig {
        name: "demo".to_owned(),
        language: "1.2".to_owned(),
        module_prefix: "Demo".to_owned(),
        source_roots: strings(&["src"]),
        entrypoints: strings(&["src/Main.lex.tex"]),
        build_root: ".lexlean/build".to_owned(),
        lockfile: "lexlean.lock".to_owned(),
        lean_workspace: ".lexlean/lean".to_owned(),
        lexicon_sources: Vec::new(),
        artifact_sources: Vec::new(),
        source_packages: Vec::new(),
        limits: limits(),
        pdf: None,
    }
}

fn builtin(package: &str) -> LexiconSource {
    LexiconSource::Builtin {
        package: package.to_owned(),
    }
}

fn pdf() -> PdfProvider {
    PdfProvider {
        program: "lualatex".to_owned(),
        program_sha256: sha(0x30),
        version_argv: strings(&["--version"]),
        version_stdout_sha256: sha(0x40),
        compile_argv: strings(&["-interaction=nonstopmode", "main.tex"]),
        output: "main.pdf".to_owned(),
        resources: strings(&["fonts/a.otf", "logo.png"]),
    }
}

struct Vector {
    label: &'static str,
    root: usize,
    run: Run,
}

#[allow(clippy::too_many_lines)]
fn named() -> Vec<Vector> {
    let mut out = Vec::new();
    let mut add = |label: &'static str, root: usize, run: Run| {
        out.push(Vector { label, root, run });
    };
    add("a minimal project", 0, text_run(&minimal()));
    add(
        "one builtin source",
        0,
        text_run(&ProjectConfig {
            lexicon_sources: vec![builtin("lexlean.std.nat")],
            ..minimal()
        }),
    );
    add(
        "sources out of order",
        0,
        text_run(&ProjectConfig {
            lexicon_sources: vec![builtin("z.pkg"), builtin("a.pkg"), builtin("m.pkg")],
            ..minimal()
        }),
    );
    add(
        "every kind of source",
        0,
        text_run(&ProjectConfig {
            lexicon_sources: vec![
                LexiconSource::Git {
                    package: "c.git".to_owned(),
                    url: "https://example.org/r.git".to_owned(),
                    revision: "0123456789abcdef0123456789abcdef01234567".to_owned(),
                    subdirectory: "lexicon".to_owned(),
                },
                LexiconSource::Path {
                    package: "b.path".to_owned(),
                    path: "../lexicon".to_owned(),
                },
                builtin("a.builtin"),
            ],
            ..minimal()
        }),
    );
    add(
        "equal packages keep their order",
        0,
        text_run(&ProjectConfig {
            lexicon_sources: vec![
                LexiconSource::Path {
                    package: "same".to_owned(),
                    path: "first".to_owned(),
                },
                LexiconSource::Path {
                    package: "same".to_owned(),
                    path: "second".to_owned(),
                },
            ],
            ..minimal()
        }),
    );
    add(
        "an artifact source",
        0,
        text_run(&ProjectConfig {
            artifact_sources: vec![ArtifactSource {
                sha256: sha(0xab),
                path: "models/m.bin".to_owned(),
            }],
            ..minimal()
        }),
    );
    add(
        "a source package",
        0,
        text_run(&ProjectConfig {
            source_packages: vec![SourcePackage {
                package: "lexlean.stdlib".to_owned(),
                version: "1.2.0".to_owned(),
                path: "../stdlib".to_owned(),
            }],
            ..minimal()
        }),
    );
    add(
        "a pdf provider",
        0,
        text_run(&ProjectConfig {
            pdf: Some(pdf()),
            ..minimal()
        }),
    );
    add(
        "escapes in the name",
        0,
        text_run(&ProjectConfig {
            name: "a \"quoted\"\\name\twith\u{7f}".to_owned(),
            ..minimal()
        }),
    );
    add(
        "limits at their largest",
        0,
        text_run(&ProjectConfig {
            limits: Limits {
                max_file_bytes: u64::MAX,
                child_timeout_ms: 0,
                ..limits()
            },
            ..minimal()
        }),
    );
    add("the digest of a minimal project", 1, digest_run(&minimal()));
    add(
        "the digest of a pdf project",
        1,
        digest_run(&ProjectConfig {
            pdf: Some(pdf()),
            ..minimal()
        }),
    );
    out
}

// ------------------------------------------------------------------- seeded

const PIECES: &[&str] = &[
    "a", "z", "0", ".", "/", "-", " ", "\"", "\\", "\n", "\u{1f}", "\u{7f}", "\u{e9}",
];
const PACKAGES: &[&str] = &["a", "a.b", "b", "lexlean.std.nat", ""];

fn drawn_text(seeded: &mut Seeded) -> String {
    let count = seeded.below(7);
    (0..count).map(|_| *seeded.pick(PIECES)).collect()
}

fn drawn_texts(seeded: &mut Seeded) -> Vec<String> {
    let count = seeded.below(4);
    (0..count).map(|_| drawn_text(seeded)).collect()
}

fn drawn_sha(seeded: &mut Seeded) -> Sha256Digest {
    let mut out = [0u8; 32];
    for slot in &mut out {
        *slot = u8::try_from(seeded.below(256)).expect("a byte");
    }
    Sha256Digest(out)
}

fn drawn_number(seeded: &mut Seeded) -> u64 {
    match seeded.below(4) {
        0 => 0,
        1 => u64::try_from(seeded.below(1_000)).expect("a number"),
        2 => seeded.next(),
        _ => u64::MAX,
    }
}

fn drawn_config(seeded: &mut Seeded) -> ProjectConfig {
    let lexicon_sources = (0..seeded.below(5))
        .map(|_| {
            let package = (*seeded.pick(PACKAGES)).to_owned();
            match seeded.below(3) {
                0 => LexiconSource::Builtin { package },
                1 => LexiconSource::Path {
                    package,
                    path: drawn_text(seeded),
                },
                _ => LexiconSource::Git {
                    package,
                    url: drawn_text(seeded),
                    revision: drawn_text(seeded),
                    subdirectory: drawn_text(seeded),
                },
            }
        })
        .collect();
    let artifact_sources = (0..seeded.below(3))
        .map(|_| ArtifactSource {
            sha256: drawn_sha(seeded),
            path: drawn_text(seeded),
        })
        .collect();
    let source_packages = (0..seeded.below(3))
        .map(|_| SourcePackage {
            package: drawn_text(seeded),
            version: drawn_text(seeded),
            path: drawn_text(seeded),
        })
        .collect();
    let limits = Limits {
        max_file_bytes: drawn_number(seeded),
        max_total_source_bytes: drawn_number(seeded),
        max_primitive_atoms: drawn_number(seeded),
        max_token_lattice_edges: drawn_number(seeded),
        max_parse_states: drawn_number(seeded),
        max_ir_nodes: drawn_number(seeded),
        max_scope_depth: drawn_number(seeded),
        max_import_depth: drawn_number(seeded),
        max_diagnostics: drawn_number(seeded),
        max_child_output_bytes: drawn_number(seeded),
        child_timeout_ms: drawn_number(seeded),
    };
    let pdf = if seeded.chance(30) {
        Some(PdfProvider {
            program: drawn_text(seeded),
            program_sha256: drawn_sha(seeded),
            version_argv: drawn_texts(seeded),
            version_stdout_sha256: drawn_sha(seeded),
            compile_argv: drawn_texts(seeded),
            output: drawn_text(seeded),
            resources: drawn_texts(seeded),
        })
    } else {
        None
    };
    ProjectConfig {
        name: drawn_text(seeded),
        language: (*seeded.pick(&["1.0", "1.1", "1.2"])).to_owned(),
        module_prefix: drawn_text(seeded),
        source_roots: drawn_texts(seeded),
        entrypoints: drawn_texts(seeded),
        build_root: drawn_text(seeded),
        lockfile: drawn_text(seeded),
        lean_workspace: drawn_text(seeded),
        lexicon_sources,
        artifact_sources,
        source_packages,
        limits,
        pdf,
    }
}

fn seeded_runs(count: usize, seed: u64) -> [Vec<Run>; 2] {
    let mut seeded = Seeded(seed);
    let mut out: [Vec<Run>; 2] = [Vec::new(), Vec::new()];
    for _ in 0..count {
        let config = drawn_config(&mut seeded);
        out[0].push(text_run(&config));
        out[1].push(digest_run(&config));
    }
    out
}

// ------------------------------------------------------------------- plants

struct Plant {
    label: &'static str,
    root: usize,
    site: &'static str,
    replacement: &'static str,
    vector: &'static str,
}

const PLANTS: &[Plant] = &[
    Plant {
        label: "the empty source list is omitted",
        root: 0,
        site: "(cat (lit \"lexicon_source = []\") 0x0a)",
        replacement: "0x",
        vector: "a minimal project",
    },
    Plant {
        label: "the toolchain line is missing",
        root: 0,
        site: "(stringLine \"lean_toolchain\" (LockText::leanToolchain))",
        replacement: "0x",
        vector: "one builtin source",
    },
    Plant {
        label: "the sources are not sorted",
        root: 0,
        site: "0x (sourcesSorted rows) : Bytes)",
        replacement: "0x rows : Bytes)",
        vector: "sources out of order",
    },
    Plant {
        label: "equal packages are reordered",
        root: 0,
        site: "(LexLeanStd.Order::sortByKey [LexiconSource] (LexLeanStd.Order::reverseList [(Bytes * LexiconSource)] (prim list_fold",
        replacement: "(LexLeanStd.Order::sortByKey [LexiconSource] (LexLeanStd.Order::sortByKey [LexiconSource] (prim list_fold",
        vector: "equal packages keep their order",
    },
    Plant {
        label: "a git source has no subdirectory",
        root: 0,
        site: "(stringLine \"subdirectory\" subdirectory)",
        replacement: "0x",
        vector: "every kind of source",
    },
    Plant {
        label: "the artifact path is its digest",
        root: 0,
        site: "(stringLine \"path\" (project artifact location))",
        replacement: "(stringLine \"path\" (project artifact digest))",
        vector: "an artifact source",
    },
    Plant {
        label: "a source package has no kind",
        root: 0,
        site: "(stringLine \"version\" (project member version))) (stringLine \"kind\" (lit \"path\")))",
        replacement: "(stringLine \"version\" (project member version))) 0x)",
        vector: "a source package",
    },
    Plant {
        label: "the scope depth is the import depth",
        root: 0,
        site: "(numberLine \"max_scope_depth\" (project given maxScopeDepth))",
        replacement: "(numberLine \"max_scope_depth\" (project given maxImportDepth))",
        vector: "a minimal project",
    },
    Plant {
        label: "the pdf mode is internal",
        root: 0,
        site: "(lit \"external\")",
        replacement: "(lit \"internal\")",
        vector: "a pdf provider",
    },
    Plant {
        label: "the resources are the compile argv",
        root: 0,
        site: "(arrayLine \"resources\" (project provider resources))",
        replacement: "(arrayLine \"resources\" (project provider compileArgv))",
        vector: "a pdf provider",
    },
    Plant {
        label: "the spec is another",
        root: 1,
        site: "(lit \"lexlean/project/1\")",
        replacement: "(lit \"lexlean/project/2\")",
        vector: "the digest of a minimal project",
    },
    Plant {
        label: "the digest is of the name",
        root: 1,
        site: "(LexLeanStd.Sha256::sha256Hex (canonicalProject config))",
        replacement: "(LexLeanStd.Sha256::sha256Hex (project config projectName))",
        vector: "the digest of a pdf project",
    },
];

fn plants(vectors: &[Vector]) {
    let mut missed = Vec::new();
    for plant in PLANTS {
        let vector = vectors
            .iter()
            .find(|vector| vector.label == plant.vector)
            .unwrap_or_else(|| panic!("{}: no vector `{}`", plant.label, plant.vector));
        assert_eq!(
            vector.root, plant.root,
            "{}: the vector is a run of the root planted",
            plant.label
        );
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/ProjectText.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/ProjectText.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[plant.root].0);
        match super::evaluate(&low, std::slice::from_ref(&vector.run), 1).first() {
            Some(first) => eprintln!(
                "BS-33: plant detected: {}: {}",
                plant.label,
                first.chars().take(160).collect::<String>()
            ),
            None => missed.push(plant.label),
        }
    }
    assert!(
        missed.is_empty(),
        "planted defects their own vector does not detect: {missed:?}"
    );
}

fn export(lowered: &BTreeMap<&str, super::Lowered>) {
    let exported = harness::exported("bootstrap");
    let bundle = exported.bundle();
    let mut realized = 0usize;
    for exported_root in &exported.output.result.roots {
        let Some((member, low)) = exported_root
            .root
            .strip_prefix(&format!("{MODULE}."))
            .and_then(|member| lowered.get(member).map(|low| (member, low)))
        else {
            continue;
        };
        let largest = exported_root
            .outputs
            .iter()
            .flat_map(|output| &output.certification.certificates)
            .map(|certificate| certificate.file.byte_length)
            .max()
            .expect("a certificate");
        eprintln!(
            "BS-33: {}: largest certificate part {largest} bytes",
            exported_root.root
        );
        assert!(
            largest <= CERTIFICATE_BUDGET,
            "{}: a certificate part of {largest} bytes is beyond {CERTIFICATE_BUDGET}",
            exported_root.root
        );
        for output in &exported_root.outputs {
            let package: Json2 = serde_json::from_slice(
                &std::fs::read(
                    bundle
                        .join(&output.package.directory)
                        .join("package.json")
                        .as_std_path(),
                )
                .expect("package.json"),
            )
            .expect("package JSON");
            assert_eq!(
                package["program"],
                serde_json::to_value(&low.program).expect("a program is JSON"),
                "{member}: the exported package realizes the program the differential ran"
            );
            realized += 1;
        }
    }
    assert_eq!(realized, ROOTS.len(), "one certified package per root");
    let packages = harness::tree(&bundle.join("packages"));
    let committed_packages = harness::tree(&harness::fixture_dir("bootstrap").join("packages"));
    assert_eq!(
        packages.keys().collect::<Vec<_>>(),
        committed_packages.keys().collect::<Vec<_>>(),
        "the exported package files are the committed ones"
    );
    for (relative, content) in &packages {
        assert!(
            committed_packages.get(relative) == Some(content),
            "packages/{relative} differs from the committed package"
        );
    }
}

#[allow(clippy::too_many_lines)]
pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("ProjectText")
            && checked_units.units.contains_key("LockText"),
        "ProjectText and LockText are linked modules of the project"
    );
    eligibility_of(&project, "ProjectText", MODULE, &ROOTS);

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let mut runs: [Vec<Run>; 2] = [Vec::new(), Vec::new()];
    for vector in &vectors {
        runs[vector.root].push(vector.run.clone());
    }

    // Every committed manifest that parses, both as text and as a digest.
    let (configs, found, canonical) = committed();
    assert!(
        configs.iter().any(|config| config.pdf.is_some())
            && configs
                .iter()
                .any(|config| !config.source_packages.is_empty())
            && configs
                .iter()
                .any(|config| config.lexicon_sources.is_empty()),
        "the committed manifests reach a pdf provider, a source package, and no lexicon source"
    );
    let corpus: [Vec<Run>; 2] = [
        configs.iter().map(text_run).collect(),
        configs.iter().map(digest_run).collect(),
    ];
    let normative = support::normative_host();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (
            seeded_runs(EVALUATOR_SEEDED, 0x0B55_0033),
            seeded_runs(RUSTC_SEEDED, 0x0B55_1033),
        )
    } else {
        ([Vec::new(), Vec::new()], [Vec::new(), Vec::new()])
    };
    eprintln!(
        "BS-33: {} vectors ({} texts, {} digests), {} of {} committed manifests parse ({} are their canonical text), {} and {} seeded runs of each root ({:?})",
        vectors.len(),
        runs[0].len(),
        runs[1].len(),
        configs.len(),
        found,
        canonical,
        EVALUATOR_SEEDED,
        RUSTC_SEEDED,
        started.elapsed()
    );

    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, super::Lowered> = ROOTS
        .iter()
        .map(|(root, _)| (*root, super::lower(&checked, &project, MODULE, root)))
        .collect();
    if normative {
        for (index, (root, _)) in ROOTS.iter().enumerate() {
            let mut all_runs: Vec<Run> = runs[index].clone();
            all_runs.extend(corpus[index].iter().cloned());
            all_runs.extend(evaluator_seeded[index].iter().cloned());
            let disagreements = super::evaluate(&lowered[root], &all_runs, 5);
            assert!(
                disagreements.is_empty(),
                "{root} and the Rust oracle disagree: {disagreements:?}"
            );
            eprintln!(
                "BS-33: {root}: the evaluator agrees on {} runs ({:?})",
                all_runs.len(),
                started.elapsed()
            );
        }
        plants(&vectors);
    } else {
        eprintln!("BS-33: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled: [Vec<Run>; 2] = [Vec::new(), Vec::new()];
    for index in 0..2 {
        compiled[index].extend(runs[index].iter().cloned());
        compiled[index].extend(corpus[index].iter().cloned());
        compiled[index].extend(rustc_seeded[index].iter().cloned());
    }
    let total = rustc_differential(
        "BS-33",
        &[
            (ROOTS[0].0, &lowered[ROOTS[0].0], compiled[0].as_slice()),
            (ROOTS[1].0, &lowered[ROOTS[1].0], compiled[1].as_slice()),
        ],
        "oracle",
    );
    eprintln!("BS-33: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-33") {
        return;
    }
    export(&lowered);
    eprintln!("BS-33: passed in {:?}", started.elapsed());
}
