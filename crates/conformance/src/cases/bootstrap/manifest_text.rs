//! `BS-34` (SPEC.md §17.23, §21.6, §21.7): the build manifest as
//! language-1.2 source, `LexLeanBoot.ManifestText`, held to the byte to
//! `lexlean::artifact::manifest::BuildManifest::to_json` written as a file.
//!
//! `buildManifest` writes a manifest through the standard library's canonical
//! JSON writer: the specification, the compiler, the language, the project,
//! the three IDs, the pinned Lean toolchain, the selection and the modules in
//! their order, and the input and output rows sorted by kind, path, length,
//! and digest. Every committed `expected/build/manifest.json` is a run, and
//! each is the oracle's text of its own fields.
//!
//! Each planted defect names the one vector that catches it.

use std::collections::{BTreeMap, BTreeSet};

use lexlean::artifact::manifest::{BuildManifest, FileRow, ModuleRow};
use lexlean::calculus::Value;
use lexlean::Sha256Digest;
use serde_json::Value as Json2;

use super::rust_validate::{eligibility_of, rustc_differential};
use super::{bytes, list, option, text, texts, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.ManifestText";
const ROOTS: [(&str, &[&str]); 1] = [("buildManifest", &["allocation", "overflow", "recursion"])];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// Seeded runs of the root under the evaluator, and under rustc.
const EVALUATOR_SEEDED: usize = 200;
const RUSTC_SEEDED: usize = 1_000;

// ------------------------------------------------------------- the encoders

fn adt(fields: Vec<Value>) -> Value {
    Value::Adt {
        constructor: 0,
        fields,
    }
}

fn nat(value: usize) -> Value {
    Value::Nat {
        value: value.to_string(),
    }
}

fn enc_row(row: &FileRow) -> Value {
    adt(vec![
        text(&row.kind),
        text(&row.path),
        nat(row.byte_length),
        bytes(&row.sha256.0),
    ])
}

fn enc_module(row: &ModuleRow) -> Value {
    adt(vec![
        text(&row.module),
        text(&row.lean_module),
        text(&row.source_path),
        option(row.origin.as_deref().map(text)),
    ])
}

fn enc(manifest: &BuildManifest) -> Value {
    adt(vec![
        text(&manifest.language),
        text(&manifest.compiler_version),
        bytes(&manifest.semantics_id.0),
        text(&manifest.project),
        bytes(&manifest.source_id.0),
        bytes(&manifest.semantic_id.0),
        bytes(&manifest.build_id.0),
        texts(&manifest.selection),
        list(manifest.modules.iter().map(enc_module).collect()),
        list(manifest.inputs.iter().map(enc_row).collect()),
        list(manifest.outputs.iter().map(enc_row).collect()),
    ])
}

fn manifest_run(manifest: &BuildManifest) -> Run {
    (
        vec![enc(manifest)],
        serde_json::to_value(bytes(&manifest.to_json().to_file_bytes())).expect("a value is JSON"),
    )
}

// ------------------------------------------------------------------ corpora

fn digest(json: &Json2) -> Sha256Digest {
    Sha256Digest::from_hex(json.as_str().expect("a digest")).expect("a digest")
}

fn string(json: &Json2) -> String {
    json.as_str().expect("a string").to_owned()
}

fn rows_of(json: &Json2) -> Vec<FileRow> {
    json.as_array()
        .expect("rows")
        .iter()
        .map(|row| FileRow {
            kind: string(&row["kind"]),
            path: string(&row["path"]),
            byte_length: usize::try_from(row["byte_length"].as_u64().expect("a length"))
                .expect("usize"),
            sha256: digest(&row["sha256"]),
        })
        .collect()
}

fn manifest_of(json: &Json2) -> BuildManifest {
    BuildManifest {
        language: string(&json["language"]),
        compiler_version: string(&json["compiler"]["version"]),
        semantics_id: digest(&json["compiler"]["semantics_id"]),
        project: string(&json["project"]),
        source_id: digest(&json["source_id"]),
        semantic_id: digest(&json["semantic_id"]),
        build_id: digest(&json["build_id"]),
        selection: json["selection"]
            .as_array()
            .expect("a selection")
            .iter()
            .map(string)
            .collect(),
        modules: json["modules"]
            .as_array()
            .expect("modules")
            .iter()
            .map(|row| ModuleRow {
                module: string(&row["module"]),
                lean_module: string(&row["lean_module"]),
                source_path: string(&row["source_path"]),
                origin: row.get("origin").map(string),
            })
            .collect(),
        inputs: rows_of(&json["inputs"]),
        outputs: rows_of(&json["outputs"]),
    }
}

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
        } else if name == "manifest.json" && path.ends_with("expected/build/manifest.json") {
            out.push(path);
        }
    }
}

/// Every committed build manifest, each checked to be the oracle's text of
/// its own fields.
fn committed() -> Vec<BuildManifest> {
    let mut paths = Vec::new();
    manifests_under(repo_root().as_std_path(), &mut paths);
    paths
        .iter()
        .map(|path| {
            let committed_bytes = std::fs::read(path).expect("a committed manifest");
            let json: Json2 = serde_json::from_slice(&committed_bytes).expect("manifest JSON");
            let manifest = manifest_of(&json);
            assert_eq!(
                manifest.to_json().to_file_bytes(),
                committed_bytes,
                "{}: the committed manifest is the oracle's text of its own fields",
                path.display()
            );
            manifest
        })
        .collect()
}

// ------------------------------------------------------------------ vectors

fn sha(byte: u8) -> Sha256Digest {
    let mut out = [0u8; 32];
    for (index, slot) in out.iter_mut().enumerate() {
        *slot = byte.wrapping_add(u8::try_from(index).expect("a byte"));
    }
    Sha256Digest(out)
}

fn row(kind: &str, path: &str, byte_length: usize, digest_byte: u8) -> FileRow {
    FileRow {
        kind: kind.to_owned(),
        path: path.to_owned(),
        byte_length,
        sha256: sha(digest_byte),
    }
}

fn module(name: &str, origin: Option<&str>) -> ModuleRow {
    ModuleRow {
        module: name.to_owned(),
        lean_module: format!("Demo.{name}"),
        source_path: format!("src/{name}.lex.tex"),
        origin: origin.map(str::to_owned),
    }
}

fn empty() -> BuildManifest {
    BuildManifest {
        language: "1.2".to_owned(),
        compiler_version: "0.4.0".to_owned(),
        semantics_id: sha(0x10),
        project: "demo".to_owned(),
        source_id: sha(0x20),
        semantic_id: sha(0x30),
        build_id: sha(0x40),
        selection: Vec::new(),
        modules: Vec::new(),
        inputs: Vec::new(),
        outputs: Vec::new(),
    }
}

struct Vector {
    label: &'static str,
    run: Run,
}

fn named() -> Vec<Vector> {
    let mut out = Vec::new();
    let mut add = |label: &'static str, manifest: BuildManifest| {
        out.push(Vector {
            label,
            run: manifest_run(&manifest),
        });
    };
    add("an empty manifest", empty());
    add(
        "inputs out of order",
        BuildManifest {
            inputs: vec![
                row("source", "src/B.lex.tex", 10, 1),
                row("lock", "lexlean.lock", 20, 2),
                row("source", "src/A.lex.tex", 30, 3),
            ],
            ..empty()
        },
    );
    add(
        "outputs out of order",
        BuildManifest {
            outputs: vec![row("tex", "b.tex", 1, 1), row("map", "a.map.json", 2, 2)],
            ..empty()
        },
    );
    add(
        "rows of one kind and path",
        BuildManifest {
            inputs: vec![row("source", "f", 10, 1), row("source", "f", 9, 2)],
            ..empty()
        },
    );
    add(
        "lengths across a byte",
        BuildManifest {
            inputs: vec![row("source", "f", 256, 1), row("source", "f", 255, 2)],
            ..empty()
        },
    );
    add(
        "rows equal but the digest",
        BuildManifest {
            inputs: vec![row("source", "f", 1, 0x90), row("source", "f", 1, 0x10)],
            ..empty()
        },
    );
    add(
        "a kind that is a prefix",
        BuildManifest {
            inputs: vec![row("a-b", "a", 1, 1), row("a", "z", 1, 2)],
            ..empty()
        },
    );
    add(
        "a module from a source package",
        BuildManifest {
            modules: vec![module("Std", Some("lexlean.stdlib"))],
            ..empty()
        },
    );
    add(
        "modules in their order",
        BuildManifest {
            modules: vec![module("Zeta", None), module("Alpha", None)],
            selection: vec!["Zeta".to_owned(), "Alpha".to_owned()],
            ..empty()
        },
    );
    add(
        "escapes in a path",
        BuildManifest {
            outputs: vec![row("tex", "a \"b\"\\c\n\u{1}", 3, 3)],
            ..empty()
        },
    );
    out
}

// ------------------------------------------------------------------- seeded

const KINDS: &[&str] = &["source", "lock", "lean", "tex", "map", "a", "a-b", ""];
const PIECES: &[&str] = &["a", "z", "/", ".", "-", "\"", "\\", "\n", "\u{e9}"];

fn drawn_text(seeded: &mut Seeded) -> String {
    let count = seeded.below(6);
    (0..count).map(|_| *seeded.pick(PIECES)).collect()
}

fn drawn_sha(seeded: &mut Seeded) -> Sha256Digest {
    let mut out = [0u8; 32];
    for slot in &mut out {
        *slot = u8::try_from(seeded.below(256)).expect("a byte");
    }
    Sha256Digest(out)
}

fn drawn_rows(seeded: &mut Seeded) -> Vec<FileRow> {
    (0..seeded.below(5))
        .map(|_| FileRow {
            kind: (*seeded.pick(KINDS)).to_owned(),
            path: drawn_text(seeded),
            byte_length: *seeded.pick(&[0, 1, 255, 256, 65_536, 4_194_304]),
            sha256: drawn_sha(seeded),
        })
        .collect()
}

fn drawn_manifest(seeded: &mut Seeded) -> BuildManifest {
    BuildManifest {
        language: (*seeded.pick(&["1.1", "1.2"])).to_owned(),
        compiler_version: drawn_text(seeded),
        semantics_id: drawn_sha(seeded),
        project: drawn_text(seeded),
        source_id: drawn_sha(seeded),
        semantic_id: drawn_sha(seeded),
        build_id: drawn_sha(seeded),
        selection: (0..seeded.below(3)).map(|_| drawn_text(seeded)).collect(),
        modules: (0..seeded.below(3))
            .map(|_| ModuleRow {
                module: drawn_text(seeded),
                lean_module: drawn_text(seeded),
                source_path: drawn_text(seeded),
                origin: if seeded.chance(30) {
                    Some(drawn_text(seeded))
                } else {
                    None
                },
            })
            .collect(),
        inputs: drawn_rows(seeded),
        outputs: drawn_rows(seeded),
    }
}

fn seeded_runs(count: usize, seed: u64) -> Vec<Run> {
    let mut seeded = Seeded(seed);
    (0..count)
        .map(|_| manifest_run(&drawn_manifest(&mut seeded)))
        .collect()
}

// ------------------------------------------------------------------- plants

struct Plant {
    label: &'static str,
    site: &'static str,
    replacement: &'static str,
    vector: &'static str,
}

const PLANTS: &[Plant] = &[
    Plant {
        label: "the rows are not sorted",
        site: "(LexLeanStd.Order::reverseList [FileRow] (rowsSorted rows))",
        replacement: "(LexLeanStd.Order::reverseList [FileRow] rows)",
        vector: "inputs out of order",
    },
    Plant {
        label: "the length is not in the key",
        site: "(bigEndian (project row byteLength) 8)",
        replacement: "0x",
        vector: "rows of one kind and path",
    },
    Plant {
        label: "the length key is little-endian",
        site: "(cat (LexLeanStd.Numeral::pick (LexLeanStd.Numeral::byteValues) (prim remainder (first state) 256 0 : Nat)) (second state))",
        replacement: "(cat (second state) (LexLeanStd.Numeral::pick (LexLeanStd.Numeral::byteValues) (prim remainder (first state) 256 0 : Nat)))",
        vector: "lengths across a byte",
    },
    Plant {
        label: "the digest is not in the key",
        site: "(bigEndian (project row byteLength) 8)) (project row digest))",
        replacement: "(bigEndian (project row byteLength) 8)) 0x)",
        vector: "rows equal but the digest",
    },
    Plant {
        label: "the kind key has no end",
        site: "0x0001) 0x0000)",
        replacement: "0x0001) 0x)",
        vector: "a kind that is a prefix",
    },
    Plant {
        label: "the origin is never written",
        site: "| Option.some named => (member \"origin\" (jsonText named) (ctor LexLeanStd.Json::Json.JNil))",
        replacement: "| Option.some named => (ctor LexLeanStd.Json::Json.JNil)",
        vector: "a module from a source package",
    },
    Plant {
        label: "the modules are written last first",
        site: "(LexLeanStd.Order::reverseList [ModuleRow] rows)",
        replacement: "rows",
        vector: "modules in their order",
    },
    Plant {
        label: "the toolchain is another",
        site: "(lit \"leanprover/lean4:v4.32.1\")",
        replacement: "(lit \"leanprover/lean4:v4.32.0\")",
        vector: "an empty manifest",
    },
    Plant {
        label: "the semantics ID is the source ID",
        site: "(jsonDigest (project manifest semanticsId))",
        replacement: "(jsonDigest (project manifest sourceId))",
        vector: "an empty manifest",
    },
    Plant {
        label: "a byte length is negative",
        site: "(ctor LexLeanStd.Json::Json.JInt false (project row byteLength))",
        replacement: "(ctor LexLeanStd.Json::Json.JInt true (project row byteLength))",
        vector: "outputs out of order",
    },
];

fn plants(vectors: &[Vector]) {
    let mut missed = Vec::new();
    for plant in PLANTS {
        let vector = vectors
            .iter()
            .find(|vector| vector.label == plant.vector)
            .unwrap_or_else(|| panic!("{}: no vector `{}`", plant.label, plant.vector));
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/ManifestText.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/ManifestText.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[0].0);
        match super::evaluate(&low, std::slice::from_ref(&vector.run), 1).first() {
            Some(first) => eprintln!(
                "BS-34: plant detected: {}: {}",
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
            "BS-34: {}: largest certificate part {largest} bytes",
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

pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("ManifestText"),
        "ManifestText is a linked module of the project"
    );
    eligibility_of(&project, "ManifestText", MODULE, &ROOTS);

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let runs: Vec<Run> = vectors.iter().map(|vector| vector.run.clone()).collect();

    // Every committed build manifest, each the oracle's text of its fields.
    let manifests = committed();
    assert!(
        manifests
            .iter()
            .any(|manifest| manifest.modules.iter().any(|row| row.origin.is_some())),
        "a committed manifest has a module from a source package"
    );
    let corpus: Vec<Run> = manifests.iter().map(manifest_run).collect();
    let normative = support::normative_host();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (
            seeded_runs(EVALUATOR_SEEDED, 0x0B55_0034),
            seeded_runs(RUSTC_SEEDED, 0x0B55_1034),
        )
    } else {
        (Vec::new(), Vec::new())
    };
    eprintln!(
        "BS-34: {} vectors, {} committed manifests, {} and {} seeded runs ({:?})",
        vectors.len(),
        manifests.len(),
        EVALUATOR_SEEDED,
        RUSTC_SEEDED,
        started.elapsed()
    );

    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, super::Lowered> = ROOTS
        .iter()
        .map(|(root, _)| (*root, super::lower(&checked, &project, MODULE, root)))
        .collect();
    let root = ROOTS[0].0;
    if normative {
        let mut all_runs = runs.clone();
        all_runs.extend(corpus.iter().cloned());
        all_runs.extend(evaluator_seeded.iter().cloned());
        let disagreements = super::evaluate(&lowered[root], &all_runs, 5);
        assert!(
            disagreements.is_empty(),
            "{root} and the Rust oracle disagree: {disagreements:?}"
        );
        eprintln!(
            "BS-34: {root}: the evaluator agrees on {} runs ({:?})",
            all_runs.len(),
            started.elapsed()
        );
        plants(&vectors);
    } else {
        eprintln!("BS-34: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled = runs;
    compiled.extend(corpus);
    compiled.extend(rustc_seeded);
    let total = rustc_differential(
        "BS-34",
        &[(root, &lowered[root], compiled.as_slice())],
        "oracle",
    );
    eprintln!("BS-34: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-34") {
        return;
    }
    export(&lowered);
    eprintln!("BS-34: passed in {:?}", started.elapsed());
}
