//! `BS-32` (SPEC.md §11.5, §17.23, §19.8, §21.1–§21.5, §22.9): the labeled
//! hash recipes of content identity as language-1.2 source,
//! `LexLeanBoot.ContentId`, held to the byte to
//! `lexlean::artifact::content_id::{tree_digest, source_id, semantic_id_for,
//! build_id, attestation_id, pdf_recipe_id}`.
//!
//! Each root writes its recipe's domain, a NUL, and its frames, each frame a
//! big-endian four-byte label length, the label, a big-endian eight-byte
//! payload length, and the payload, and hashes the message with the standard
//! library's SHA-256. The committed build manifests are runs of `buildId`,
//! and each manifest's build ID is the oracle's.
//!
//! Each planted defect names the one vector that catches it.

use std::collections::{BTreeMap, BTreeSet};

use lexlean::artifact::content_id::{
    attestation_id, build_id, pdf_recipe_id, semantic_id_for, source_id, tree_digest,
};
use lexlean::calculus::Value;
use lexlean::Sha256Digest;
use serde_json::Value as Json2;

use super::rust_validate::{eligibility_of, rustc_differential};
use super::{bytes, list, pair, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.ContentId";
const ROOTS: [(&str, &[&str]); 6] = [
    ("treeDigest", &["allocation", "overflow"]),
    ("sourceId", &["allocation", "overflow"]),
    ("semanticId", &["allocation", "overflow"]),
    ("buildId", &["allocation", "overflow"]),
    ("attestationId", &["allocation", "overflow"]),
    ("pdfRecipeId", &["allocation", "overflow"]),
];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// Seeded runs of each root under the evaluator, and under rustc.
const EVALUATOR_SEEDED: usize = 100;
const RUSTC_SEEDED: usize = 500;
/// The most bytes a committed tree may hold to be a run under the evaluator,
/// and under rustc.
const EVALUATOR_TREE_BYTES: usize = 4_096;
const RUSTC_TREE_BYTES: usize = 65_536;

// ---------------------------------------------------------------- the runs

type Rows = Vec<(Vec<u8>, Vec<u8>)>;

fn rows_value(rows: &Rows) -> Value {
    list(
        rows.iter()
            .map(|(path, content)| pair(bytes(path), bytes(content)))
            .collect(),
    )
}

fn digest_of(value: &[u8]) -> Sha256Digest {
    Sha256Digest(value.try_into().expect("a digest is 32 bytes"))
}

fn out(digest: Sha256Digest) -> Json2 {
    serde_json::to_value(bytes(&digest.0)).expect("a value is JSON")
}

fn utf8(value: &[u8]) -> &str {
    std::str::from_utf8(value).expect("the recipe takes text")
}

fn tree_run(rows: &Rows) -> Run {
    let borrowed: Vec<(&str, &[u8])> = rows
        .iter()
        .map(|(path, content)| (utf8(path), content.as_slice()))
        .collect();
    (vec![rows_value(rows)], out(tree_digest(&borrowed)))
}

fn source_run(project: &[u8], lock: &[u8], rows: &Rows) -> Run {
    let owned: Vec<(String, Vec<u8>)> = rows
        .iter()
        .map(|(path, content)| (utf8(path).to_owned(), content.clone()))
        .collect();
    (
        vec![bytes(project), bytes(lock), rows_value(rows)],
        out(source_id(project, lock, &owned)),
    )
}

fn semantic_run(semantics: &[u8], language: &str, ir: &str, closure: &str) -> Run {
    (
        vec![
            bytes(semantics),
            bytes(language.as_bytes()),
            bytes(ir.as_bytes()),
            bytes(closure.as_bytes()),
        ],
        out(semantic_id_for(digest_of(semantics), language, ir, closure)),
    )
}

fn build_run(source: &[u8], semantic: &[u8]) -> Run {
    (
        vec![bytes(source), bytes(semantic)],
        out(build_id(digest_of(source), digest_of(semantic))),
    )
}

fn attestation_run(body: &str) -> Run {
    (vec![bytes(body.as_bytes())], out(attestation_id(body)))
}

fn pdf_run(tex: &[u8], program: &[u8], version: &[u8], argv: &str, resources: &str) -> Run {
    (
        vec![
            bytes(tex),
            bytes(program),
            bytes(version),
            bytes(argv.as_bytes()),
            bytes(resources.as_bytes()),
        ],
        out(pdf_recipe_id(
            digest_of(tex),
            digest_of(program),
            digest_of(version),
            argv,
            resources,
        )),
    )
}

fn tree_bytes(rows: &Rows) -> usize {
    rows.iter()
        .map(|(path, content)| path.len() + content.len())
        .sum()
}

// ------------------------------------------------------------------ corpora

fn projects_under(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(std::fs::DirEntry::file_name);
    for entry in entries {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        if path.is_dir() {
            if !matches!(
                name.as_str(),
                ".git" | "target" | ".lexlean" | "node_modules"
            ) {
                projects_under(&path, out);
            }
        } else if name == "lexlean.toml" {
            out.push(dir.to_path_buf());
        }
    }
}

fn files_under(root: &std::path::Path, dir: &std::path::Path, rows: &mut Rows) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            files_under(root, &path, rows);
        } else if let Ok(relative) = path.strip_prefix(root) {
            let relative = relative.to_string_lossy().replace('\\', "/");
            rows.push((
                relative.into_bytes(),
                std::fs::read(&path).expect("a committed file"),
            ));
        }
    }
}

/// Each committed project: its manifest, its lock or nothing, and its
/// sources sorted by path bytes, as the tree digest takes them.
fn committed_projects() -> Vec<(Vec<u8>, Vec<u8>, Rows)> {
    let mut dirs = Vec::new();
    projects_under(repo_root().as_std_path(), &mut dirs);
    dirs.iter()
        .map(|dir| {
            let mut rows = Vec::new();
            files_under(dir, &dir.join("src"), &mut rows);
            for (path, _) in &mut rows {
                let mut prefixed = b"src/".to_vec();
                prefixed.extend_from_slice(path);
                *path = prefixed;
            }
            rows.retain(|(path, _)| std::str::from_utf8(path).is_ok());
            rows.sort_by(|a, b| a.0.cmp(&b.0));
            (
                std::fs::read(dir.join("lexlean.toml")).expect("a manifest"),
                std::fs::read(dir.join("lexlean.lock")).unwrap_or_default(),
                rows,
            )
        })
        .collect()
}

/// The source, semantic, and build IDs of each committed build manifest.
fn committed_builds() -> Vec<(Sha256Digest, Sha256Digest, Sha256Digest)> {
    let mut dirs = Vec::new();
    projects_under(repo_root().as_std_path(), &mut dirs);
    let mut out = Vec::new();
    for dir in dirs {
        let Ok(bytes) = std::fs::read(dir.join("expected/build/manifest.json")) else {
            continue;
        };
        let json: Json2 = serde_json::from_slice(&bytes).expect("a build manifest");
        let id = |key: &str| {
            Sha256Digest::from_hex(json[key].as_str().expect("an ID")).expect("a digest")
        };
        out.push((id("source_id"), id("semantic_id"), id("build_id")));
    }
    out
}

// ------------------------------------------------------------------ vectors

fn d(byte: u8) -> Vec<u8> {
    (0..32u8).map(|index| byte.wrapping_add(index)).collect()
}

fn row(path: &str, content: &[u8]) -> (Vec<u8>, Vec<u8>) {
    (path.as_bytes().to_vec(), content.to_vec())
}

struct Vector {
    label: &'static str,
    root: usize,
    run: Run,
}

#[allow(clippy::too_many_lines)]
fn named() -> Vec<Vector> {
    let long_path = "p".repeat(300);
    let mut out = Vec::new();
    let mut add = |label: &'static str, root: usize, run: Run| {
        out.push(Vector { label, root, run });
    };
    add("an empty tree", 0, tree_run(&Vec::new()));
    add("one empty file", 0, tree_run(&vec![row("a", b"")]));
    add(
        "one file",
        0,
        tree_run(&vec![row("src/Main.lex.tex", b"hello\n")]),
    );
    add(
        "two files",
        0,
        tree_run(&vec![row("a", b"first"), row("b/c", b"second")]),
    );
    add(
        "a file with an empty path",
        0,
        tree_run(&vec![row("", b"x")]),
    );
    add(
        "a file across blocks",
        0,
        tree_run(&vec![row("f", &[0xa5; 200])]),
    );
    add(
        "a path of 300 bytes",
        0,
        tree_run(&vec![row(&long_path, b"")]),
    );
    add(
        "no sources",
        1,
        source_run(b"[project]\n", b"spec = 1\n", &Vec::new()),
    );
    add(
        "two sources",
        1,
        source_run(
            b"[project]\nname = \"p\"\n",
            b"",
            &vec![row("src/A.lex.tex", b"a"), row("src/B.lex.tex", b"b")],
        ),
    );
    add(
        "an empty project and lock",
        1,
        source_run(b"", b"", &vec![row("x", b"")]),
    );
    add(
        "the language 1.2",
        2,
        semantic_run(&d(0x11), "1.2", "{\"modules\":[]}", "[]"),
    );
    add("an empty IR", 2, semantic_run(&d(0x22), "1.1", "", ""));
    add("a build of zero digests", 3, build_run(&[0; 32], &[0; 32]));
    add("two distinct digests", 3, build_run(&d(0x01), &d(0x80)));
    add("an empty body", 4, attestation_run(""));
    add("a body of 300 bytes", 4, attestation_run(&"b".repeat(300)));
    add(
        "a recipe",
        5,
        pdf_run(
            &d(0x30),
            &d(0x40),
            &d(0x50),
            "[\"lualatex\",\"main.tex\"]",
            "[]",
        ),
    );
    out
}

// ------------------------------------------------------------------- seeded

fn drawn_bytes(seeded: &mut Seeded, most: usize) -> Vec<u8> {
    let count = seeded.below(most + 1);
    (0..count)
        .map(|_| u8::try_from(seeded.below(256)).expect("a byte"))
        .collect()
}

fn drawn_text(seeded: &mut Seeded, most: usize) -> String {
    let count = seeded.below(most + 1);
    (0..count)
        .map(|_| *seeded.pick(&['a', 'z', '0', '"', '{', '}', '\n', '\u{e9}', '\u{1f600}']))
        .collect()
}

fn drawn_digest(seeded: &mut Seeded) -> Vec<u8> {
    (0..32)
        .map(|_| u8::try_from(seeded.below(256)).expect("a byte"))
        .collect()
}

fn drawn_rows(seeded: &mut Seeded) -> Rows {
    let mut rows: Rows = (0..seeded.below(5))
        .map(|_| {
            (
                drawn_text(seeded, 12).into_bytes(),
                drawn_bytes(seeded, 150),
            )
        })
        .collect();
    rows.sort_by(|a, b| a.0.cmp(&b.0));
    rows.dedup_by(|a, b| a.0 == b.0);
    rows
}

fn seeded_runs(count: usize, seed: u64) -> [Vec<Run>; 6] {
    let mut seeded = Seeded(seed);
    let mut out: [Vec<Run>; 6] = Default::default();
    for _ in 0..count {
        out[0].push(tree_run(&drawn_rows(&mut seeded)));
        let project = drawn_bytes(&mut seeded, 80);
        let lock = drawn_bytes(&mut seeded, 80);
        out[1].push(source_run(&project, &lock, &drawn_rows(&mut seeded)));
        let semantics = drawn_digest(&mut seeded);
        let language = drawn_text(&mut seeded, 4);
        let ir = drawn_text(&mut seeded, 120);
        let closure = drawn_text(&mut seeded, 40);
        out[2].push(semantic_run(&semantics, &language, &ir, &closure));
        let source = drawn_digest(&mut seeded);
        out[3].push(build_run(&source, &drawn_digest(&mut seeded)));
        out[4].push(attestation_run(&drawn_text(&mut seeded, 200)));
        let tex = drawn_digest(&mut seeded);
        let program = drawn_digest(&mut seeded);
        let version = drawn_digest(&mut seeded);
        let argv = drawn_text(&mut seeded, 40);
        out[5].push(pdf_run(
            &tex,
            &program,
            &version,
            &argv,
            &drawn_text(&mut seeded, 40),
        ));
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
        label: "the domain has no NUL",
        root: 0,
        site: "(cat (cat domain 0x00) frames)",
        replacement: "(cat domain frames)",
        vector: "an empty tree",
    },
    Plant {
        label: "the label length is two bytes",
        root: 0,
        site: "(bigEndian (prim length label : Nat) 4)",
        replacement: "(bigEndian (prim length label : Nat) 2)",
        vector: "one file",
    },
    Plant {
        label: "the payload length is four bytes",
        root: 4,
        site: "(bigEndian (prim length payload : Nat) 8)",
        replacement: "(bigEndian (prim length payload : Nat) 4)",
        vector: "an empty body",
    },
    Plant {
        label: "the lengths are little-endian",
        root: 0,
        site: "(cat (byteText (prim remainder (first state) 256 0 : Nat)) (second state))",
        replacement: "(cat (second state) (byteText (prim remainder (first state) 256 0 : Nat)))",
        vector: "a path of 300 bytes",
    },
    Plant {
        label: "the high byte of a word is dropped",
        root: 0,
        site: "(byteText (prim to_nat (LexLeanStd.Sha256::shiftRight word (24 : UInt32)) : Nat))",
        replacement: "(byteText 0)",
        vector: "one empty file",
    },
    Plant {
        label: "the low byte of a word is its third",
        root: 0,
        site: "(prim bit_and word (255 : UInt32) : UInt32)",
        replacement: "(prim bit_and (LexLeanStd.Sha256::shiftRight word (8 : UInt32)) (255 : UInt32) : UInt32)",
        vector: "two files",
    },
    Plant {
        label: "the digest ends a word early",
        root: 3,
        site: "(wordBytes (project digest h7))",
        replacement: "0x",
        vector: "a build of zero digests",
    },
    Plant {
        label: "the tree is hashed in the source domain",
        root: 0,
        site: "(lit \"lexlean-tree-v1\")",
        replacement: "(lit \"lexlean-source-v1\")",
        vector: "a file with an empty path",
    },
    Plant {
        label: "a source is not framed with its path",
        root: 1,
        site: "(cat (frame (lit \"path\") (first row)) (frame (lit \"source\") (second row)))",
        replacement: "(frame (lit \"source\") (second row))",
        vector: "two sources",
    },
    Plant {
        label: "the lock is framed as the project",
        root: 1,
        site: "(frame (lit \"lock\") lockText)",
        replacement: "(frame (lit \"project\") lockText)",
        vector: "no sources",
    },
    Plant {
        label: "the toolchain is another",
        root: 2,
        site: "(lit \"leanprover/lean4:v4.32.1\")",
        replacement: "(lit \"leanprover/lean4:v4.32.0\")",
        vector: "the language 1.2",
    },
    Plant {
        label: "the language is not framed",
        root: 2,
        site: "(frame (lit \"language\") language)",
        replacement: "0x",
        vector: "an empty IR",
    },
    Plant {
        label: "the build digests are swapped",
        root: 3,
        site: "(cat (frame (lit \"source-id\") sourceDigest) (frame (lit \"semantic-id\") semanticDigest))",
        replacement: "(cat (frame (lit \"source-id\") semanticDigest) (frame (lit \"semantic-id\") sourceDigest))",
        vector: "two distinct digests",
    },
    Plant {
        label: "the attestation label is another",
        root: 4,
        site: "(lit \"attestation-body\")",
        replacement: "(lit \"attestation\")",
        vector: "a body of 300 bytes",
    },
    Plant {
        label: "the argv is framed as the resources",
        root: 5,
        site: "(frame (lit \"argv\") compileArgv)",
        replacement: "(frame (lit \"argv\") resources)",
        vector: "a recipe",
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
        let source = planted.read_raw("src/ContentId.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/ContentId.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[plant.root].0);
        match super::evaluate(&low, std::slice::from_ref(&vector.run), 1).first() {
            Some(first) => eprintln!(
                "BS-32: plant detected: {}: {}",
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
            "BS-32: {}: largest certificate part {largest} bytes",
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
        checked_units.units.contains_key("ContentId"),
        "ContentId is a linked module of the project"
    );
    eligibility_of(&project, "ContentId", MODULE, &ROOTS);

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let mut runs: [Vec<Run>; 6] = Default::default();
    for vector in &vectors {
        runs[vector.root].push(vector.run.clone());
    }

    // The committed projects' trees and source inputs, and the committed
    // build manifests, each of whose build ID the oracle reproduces.
    let projects = committed_projects();
    let builds = committed_builds();
    for (source, semantic, recorded) in &builds {
        assert_eq!(
            build_id(*source, *semantic),
            *recorded,
            "a committed build ID is the build recipe of its source and semantic IDs"
        );
    }
    let mut evaluator_corpus: [Vec<Run>; 6] = Default::default();
    let mut rustc_corpus: [Vec<Run>; 6] = Default::default();
    for (manifest, lock, rows) in &projects {
        let size = tree_bytes(rows) + manifest.len() + lock.len();
        if size > RUSTC_TREE_BYTES {
            continue;
        }
        let tree = tree_run(rows);
        let source = source_run(manifest, lock, rows);
        if size <= EVALUATOR_TREE_BYTES {
            evaluator_corpus[0].push(tree.clone());
            evaluator_corpus[1].push(source.clone());
        }
        rustc_corpus[0].push(tree);
        rustc_corpus[1].push(source);
    }
    for (source, semantic, _) in &builds {
        let run = build_run(&source.0, &semantic.0);
        evaluator_corpus[3].push(run.clone());
        rustc_corpus[3].push(run);
    }
    let normative = support::normative_host();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (
            seeded_runs(EVALUATOR_SEEDED, 0x0B55_0032),
            seeded_runs(RUSTC_SEEDED, 0x0B55_1032),
        )
    } else {
        (Default::default(), Default::default())
    };
    eprintln!(
        "BS-32: {} vectors, {} committed projects ({} trees under the evaluator, {} under rustc), {} committed builds, {} and {} seeded runs of each root ({:?})",
        vectors.len(),
        projects.len(),
        evaluator_corpus[0].len(),
        rustc_corpus[0].len(),
        builds.len(),
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
            all_runs.extend(evaluator_corpus[index].iter().cloned());
            all_runs.extend(evaluator_seeded[index].iter().cloned());
            let disagreements = super::evaluate(&lowered[root], &all_runs, 5);
            assert!(
                disagreements.is_empty(),
                "{root} and the Rust oracle disagree: {disagreements:?}"
            );
            eprintln!(
                "BS-32: {root}: the evaluator agrees on {} runs ({:?})",
                all_runs.len(),
                started.elapsed()
            );
        }
        plants(&vectors);
    } else {
        eprintln!("BS-32: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled: [Vec<Run>; 6] = Default::default();
    for index in 0..6 {
        compiled[index].extend(runs[index].iter().cloned());
        compiled[index].extend(rustc_corpus[index].iter().cloned());
        compiled[index].extend(rustc_seeded[index].iter().cloned());
    }
    let roots: Vec<(&str, &super::Lowered, &[Run])> = ROOTS
        .iter()
        .enumerate()
        .map(|(index, (root, _))| (*root, &lowered[root], compiled[index].as_slice()))
        .collect();
    let total = rustc_differential("BS-32", &roots, "oracle");
    eprintln!("BS-32: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-32") {
        return;
    }
    export(&lowered);
    eprintln!("BS-32: passed in {:?}", started.elapsed());
}
