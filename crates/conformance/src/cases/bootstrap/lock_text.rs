//! `BS-31` (SPEC.md §11.2, §17.23): the canonical text of the lock file as
//! language-1.2 source, `LexLeanBoot.LockText`, held to the byte to
//! `lexlean::lock::Lock::canonical_bytes` and `lexlean::config::toml_string`.
//!
//! `canonicalLock` writes a lock as `lexlean lock` writes it: the header keys
//! in their fixed order, the workspace pins sorted by path, the packages
//! sorted by identifier and version, the source packages sorted by
//! identifier, and the PDF record with its resources sorted by path;
//! `tomlString` is the TOML basic string of a text, with the quote, the
//! backslash, and the controls escaped. Every committed `lexlean.lock` that
//! parses is a run.
//!
//! Each planted defect names the one vector that catches it.

use std::collections::{BTreeMap, BTreeSet};

use lexlean::calculus::Value;
use lexlean::config::toml_string;
use lexlean::lock::{parse_lock, Lock, LockPackage, LockPdf, LockSourcePackage};
use lexlean::Sha256Digest;
use serde_json::Value as Json2;

use super::rust_validate::{eligibility_of, rustc_differential};
use super::{bytes, list, option, pair, text, texts, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.LockText";
const ROOTS: [(&str, &[&str]); 2] = [
    ("tomlString", &["allocation", "overflow"]),
    ("canonicalLock", &["allocation", "overflow"]),
];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// Seeded runs of each root under the evaluator, and under rustc.
const EVALUATOR_SEEDED: usize = 200;
const RUSTC_SEEDED: usize = 1_000;

// ------------------------------------------------------------- the encoders

fn digest(value: &Sha256Digest) -> Value {
    bytes(&value.0)
}

fn pins(rows: &[(String, Sha256Digest)]) -> Value {
    list(
        rows.iter()
            .map(|(path, sha256)| pair(text(path), digest(sha256)))
            .collect(),
    )
}

fn enc_package(package: &LockPackage) -> Value {
    Value::Adt {
        constructor: 0,
        fields: vec![
            text(&package.id),
            text(&package.version),
            text(&package.kind),
            text(&package.source),
            text(&package.revision),
            digest(&package.tree_sha256),
            digest(&package.manifest_sha256),
            texts(&package.imports),
        ],
    }
}

fn enc_source(package: &LockSourcePackage) -> Value {
    Value::Adt {
        constructor: 0,
        fields: vec![
            text(&package.id),
            text(&package.version),
            text(&package.kind),
            text(&package.source),
            text(&package.module_prefix),
            digest(&package.tree_sha256),
            digest(&package.manifest_sha256),
            texts(&package.imports),
        ],
    }
}

fn enc_pdf(pdf: &LockPdf) -> Value {
    Value::Adt {
        constructor: 0,
        fields: vec![
            text(&pdf.program),
            digest(&pdf.program_sha256),
            texts(&pdf.version_argv),
            digest(&pdf.version_stdout_sha256),
            texts(&pdf.compile_argv),
            text(&pdf.output),
            pins(&pdf.resources),
        ],
    }
}

fn enc(lock: &Lock) -> Value {
    Value::Adt {
        constructor: 0,
        fields: vec![
            text(&lock.language),
            digest(&lock.compiler_semantics),
            digest(&lock.project_config_sha256),
            pins(&lock.workspace_files),
            list(lock.packages.iter().map(enc_package).collect()),
            list(lock.source_packages.iter().map(enc_source).collect()),
            option(lock.pdf.as_ref().map(enc_pdf)),
        ],
    }
}

fn lock_run(lock: &Lock) -> Run {
    (
        vec![enc(lock)],
        serde_json::to_value(bytes(&lock.canonical_bytes())).expect("a value is JSON"),
    )
}

fn string_run(value: &str) -> Run {
    (
        vec![text(value)],
        serde_json::to_value(text(&toml_string(value))).expect("a value is JSON"),
    )
}

// ------------------------------------------------------------------ corpora

fn locks_under(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(std::fs::DirEntry::file_name);
    for entry in entries {
        let name = entry.file_name().to_string_lossy().into_owned();
        let path = entry.path();
        if path.is_dir() {
            if !matches!(name.as_str(), ".git" | "target" | ".lexlean") {
                locks_under(&path, out);
            }
        } else if name == "lexlean.lock" {
            out.push(path);
        }
    }
}

/// Every committed lock that parses, how many committed locks there are,
/// and how many of those that parse are their own canonical text.
fn committed() -> (Vec<Lock>, usize, usize) {
    let root = repo_root();
    let mut paths = Vec::new();
    locks_under(root.as_std_path(), &mut paths);
    let found = paths.len();
    let mut canonical = 0usize;
    let locks = paths
        .iter()
        .filter_map(|path| {
            let bytes = std::fs::read(path).expect("a committed lock");
            let lock = parse_lock(&path.to_string_lossy(), &bytes).ok()?;
            if lock.canonical_bytes() == bytes {
                canonical += 1;
            }
            Some(lock)
        })
        .collect();
    (locks, found, canonical)
}

/// Every text of the committed locks, once.
fn texts_of(locks: &[Lock]) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for lock in locks {
        out.insert(lock.language.clone());
        for (path, _) in &lock.workspace_files {
            out.insert(path.clone());
        }
        for package in &lock.packages {
            out.extend([
                package.id.clone(),
                package.version.clone(),
                package.kind.clone(),
                package.source.clone(),
                package.revision.clone(),
            ]);
            out.extend(package.imports.iter().cloned());
        }
        for package in &lock.source_packages {
            out.extend([
                package.id.clone(),
                package.version.clone(),
                package.kind.clone(),
                package.source.clone(),
                package.module_prefix.clone(),
            ]);
            out.extend(package.imports.iter().cloned());
        }
        if let Some(pdf) = &lock.pdf {
            out.insert(pdf.program.clone());
            out.insert(pdf.output.clone());
            out.extend(pdf.version_argv.iter().cloned());
            out.extend(pdf.compile_argv.iter().cloned());
            for (path, _) in &pdf.resources {
                out.insert(path.clone());
            }
        }
    }
    out
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

fn empty(language: &str) -> Lock {
    Lock {
        language: language.to_owned(),
        compiler_semantics: sha(0xa0),
        project_config_sha256: sha(0x0f),
        workspace_files: Vec::new(),
        packages: Vec::new(),
        source_packages: Vec::new(),
        pdf: None,
    }
}

fn package(id: &str, version: &str, imports: &[&str]) -> LockPackage {
    LockPackage {
        id: id.to_owned(),
        version: version.to_owned(),
        kind: "lexicon".to_owned(),
        source: format!("path:../{id}"),
        revision: format!("r-{id}-{version}"),
        tree_sha256: sha(0x10),
        manifest_sha256: sha(0xf0),
        imports: strings(imports),
    }
}

fn source(id: &str, prefix: &str) -> LockSourcePackage {
    LockSourcePackage {
        id: id.to_owned(),
        version: "1.2.0".to_owned(),
        kind: "source".to_owned(),
        source: format!("path:../{id}"),
        module_prefix: prefix.to_owned(),
        tree_sha256: sha(0x20),
        manifest_sha256: sha(0xe0),
        imports: strings(&["lexlean.std.nat"]),
    }
}

fn pdf(resources: &[&str]) -> LockPdf {
    LockPdf {
        program: "lualatex".to_owned(),
        program_sha256: sha(0x30),
        version_argv: strings(&["--version"]),
        version_stdout_sha256: sha(0x40),
        compile_argv: strings(&["-interaction=nonstopmode", "-halt-on-error", "main.tex"]),
        output: "main.pdf".to_owned(),
        resources: resources
            .iter()
            .enumerate()
            .map(|(index, path)| {
                (
                    (*path).to_owned(),
                    sha(u8::try_from(index).expect("a byte")),
                )
            })
            .collect(),
    }
}

fn files(paths: &[&str]) -> Vec<(String, Sha256Digest)> {
    paths
        .iter()
        .enumerate()
        .map(|(index, path)| {
            (
                (*path).to_owned(),
                sha(0x50 + u8::try_from(index).expect("a byte")),
            )
        })
        .collect()
}

enum Subject {
    Lock(Box<Lock>),
    Text(String),
}

struct Vector {
    label: String,
    subject: Subject,
}

impl Vector {
    fn root(&self) -> usize {
        match self.subject {
            Subject::Text(_) => 0,
            Subject::Lock(_) => 1,
        }
    }

    fn run(&self) -> Run {
        match &self.subject {
            Subject::Lock(lock) => lock_run(lock),
            Subject::Text(value) => string_run(value),
        }
    }
}

#[allow(clippy::too_many_lines)]
fn named() -> Vec<Vector> {
    let mut out: Vec<Vector> = Vec::new();
    let mut lock = |label: &str, value: Lock| {
        out.push(Vector {
            label: label.to_owned(),
            subject: Subject::Lock(Box::new(value)),
        });
    };
    lock("an empty language-1.1 lock", empty("1.1"));
    lock("an empty language-1.2 lock", empty("1.2"));
    lock(
        "one workspace file",
        Lock {
            workspace_files: files(&["src/Main.lex.tex"]),
            ..empty("1.2")
        },
    );
    lock(
        "workspace files out of order",
        Lock {
            workspace_files: files(&["src/b.lex.tex", "lexlean.toml", "src/a.lex.tex"]),
            ..empty("1.2")
        },
    );
    lock(
        "a path that is a prefix of another",
        Lock {
            workspace_files: files(&["a/b", "a.b", "a"]),
            ..empty("1.1")
        },
    );
    lock(
        "every field of a package",
        Lock {
            packages: vec![package("lexlean.std.nat", "1.2.0", &[])],
            ..empty("1.2")
        },
    );
    lock(
        "two versions of a package",
        Lock {
            packages: vec![package("p", "2.0.0", &[]), package("p", "1.0.0", &[])],
            ..empty("1.2")
        },
    );
    lock(
        "an identifier that is a prefix of another",
        Lock {
            packages: vec![package("a.b", "1", &[]), package("a", "2", &[])],
            ..empty("1.2")
        },
    );
    lock(
        "an identifier with a NUL",
        Lock {
            packages: vec![package("a\0", "1", &[]), package("a", "9", &[])],
            ..empty("1.2")
        },
    );
    lock(
        "imports in their order",
        Lock {
            packages: vec![package("q", "1.0.0", &["z", "a", "m"])],
            ..empty("1.2")
        },
    );
    lock(
        "source packages out of order",
        Lock {
            source_packages: vec![source("zeta", "Zeta"), source("alpha", "Alpha")],
            ..empty("1.2")
        },
    );
    lock(
        "a pdf record",
        Lock {
            workspace_files: files(&["main.tex"]),
            pdf: Some(pdf(&["fonts/b.otf", "fonts/a.otf"])),
            ..empty("1.2")
        },
    );
    lock(
        "a pdf with no resources",
        Lock {
            pdf: Some(pdf(&[])),
            ..empty("1.1")
        },
    );
    lock(
        "escapes in every table",
        Lock {
            workspace_files: files(&["dir with \"quote\"/a\\b"]),
            packages: vec![package("p", "1\t0", &["tab\there", "line\nnext"])],
            source_packages: vec![source("s", "Pre\u{7f}fix")],
            pdf: Some(pdf(&["caf\u{e9}.png"])),
            ..empty("1.2")
        },
    );
    let mut add = |label: &str, value: &str| {
        out.push(Vector {
            label: label.to_owned(),
            subject: Subject::Text(value.to_owned()),
        });
    };
    add("the empty text", "");
    add("plain text", "lexlean/lock/2");
    add("a quote", "say \"hi\"");
    add("a backslash", "a\\b");
    add("a newline", "one\ntwo");
    add("a tab", "a\tb");
    add("a NUL", "\0");
    add("a vertical tab", "\u{b}");
    add("the unit separator", "\u{1f}");
    add("a space", " ");
    add("delete", "\u{7f}");
    add("a tilde", "~");
    add("two-byte text", "caf\u{e9}");
    add("four-byte text", "\u{1f600}");
    out
}

// ------------------------------------------------------------------- seeded

const PIECES: &[&str] = &[
    "a",
    "b",
    "z",
    "A",
    "0",
    "9",
    ".",
    "/",
    "-",
    "_",
    " ",
    "\"",
    "\\",
    "\n",
    "\t",
    "\r",
    "\0",
    "\u{1}",
    "\u{1b}",
    "\u{1f}",
    "\u{7f}",
    "~",
    "\u{e9}",
    "\u{3b1}",
    "\u{1f600}",
];
const IDS: &[&str] = &["a", "a.b", "a-b", "a/b", "b", "lexlean.std.nat", "", "a\0"];
const VERSIONS: &[&str] = &["1.0.0", "1.0", "1.2.0", "2.0.0", "10.0.0", ""];

fn drawn_text(seeded: &mut Seeded) -> String {
    let count = seeded.below(9);
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

fn drawn_pins(seeded: &mut Seeded) -> Vec<(String, Sha256Digest)> {
    let count = seeded.below(5);
    (0..count)
        .map(|_| {
            let path = if seeded.chance(50) {
                (*seeded.pick(IDS)).to_owned()
            } else {
                drawn_text(seeded)
            };
            (path, drawn_sha(seeded))
        })
        .collect()
}

fn drawn_lock(seeded: &mut Seeded) -> Lock {
    let language = (*seeded.pick(&["1.0", "1.1", "1.2", "1.2", ""])).to_owned();
    let compiler_semantics = drawn_sha(seeded);
    let project_config_sha256 = drawn_sha(seeded);
    let workspace_files = drawn_pins(seeded);
    let packages = (0..seeded.below(5))
        .map(|_| LockPackage {
            id: (*seeded.pick(IDS)).to_owned(),
            version: (*seeded.pick(VERSIONS)).to_owned(),
            kind: drawn_text(seeded),
            source: drawn_text(seeded),
            revision: drawn_text(seeded),
            tree_sha256: drawn_sha(seeded),
            manifest_sha256: drawn_sha(seeded),
            imports: drawn_texts(seeded),
        })
        .collect();
    let source_packages = (0..seeded.below(4))
        .map(|_| LockSourcePackage {
            id: (*seeded.pick(IDS)).to_owned(),
            version: (*seeded.pick(VERSIONS)).to_owned(),
            kind: drawn_text(seeded),
            source: drawn_text(seeded),
            module_prefix: drawn_text(seeded),
            tree_sha256: drawn_sha(seeded),
            manifest_sha256: drawn_sha(seeded),
            imports: drawn_texts(seeded),
        })
        .collect();
    let pdf = if seeded.chance(30) {
        Some(LockPdf {
            program: drawn_text(seeded),
            program_sha256: drawn_sha(seeded),
            version_argv: drawn_texts(seeded),
            version_stdout_sha256: drawn_sha(seeded),
            compile_argv: drawn_texts(seeded),
            output: drawn_text(seeded),
            resources: drawn_pins(seeded),
        })
    } else {
        None
    };
    Lock {
        language,
        compiler_semantics,
        project_config_sha256,
        workspace_files,
        packages,
        source_packages,
        pdf,
    }
}

fn seeded_runs(count: usize, seed: u64) -> [Vec<Run>; 2] {
    let mut seeded = Seeded(seed);
    let mut out: [Vec<Run>; 2] = [Vec::new(), Vec::new()];
    for _ in 0..count {
        out[1].push(lock_run(&drawn_lock(&mut seeded)));
        out[0].push(string_run(&drawn_text(&mut seeded)));
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
        label: "the quote is not escaped",
        root: 0,
        site: "(if (value =? 34) then 0x5c22 else",
        replacement: "(if (value =? 34) then 0x22 else",
        vector: "a quote",
    },
    Plant {
        label: "the backslash is not escaped",
        root: 0,
        site: "(if (value =? 92) then 0x5c5c else",
        replacement: "(if (value =? 92) then 0x5c else",
        vector: "a backslash",
    },
    Plant {
        label: "the controls end before the newline",
        root: 0,
        site: "(if (value <? 32) then true else",
        replacement: "(if (value <? 10) then true else",
        vector: "a newline",
    },
    Plant {
        label: "delete is not escaped",
        root: 0,
        site: "then true else (value =? 127))",
        replacement: "then true else false)",
        vector: "delete",
    },
    Plant {
        label: "the escape digits are lower case",
        root: 0,
        site: "0x30313233343536373839414243444546",
        replacement: "0x30313233343536373839616263646566",
        vector: "a vertical tab",
    },
    Plant {
        label: "the high digit of an escape is dropped",
        root: 0,
        site: "(upperHex (prim quotient value 16 0 : Nat))",
        replacement: "(upperHex 0)",
        vector: "the unit separator",
    },
    Plant {
        label: "every lock is spec 2",
        root: 1,
        site: "(lit \"lexlean/lock/1\")",
        replacement: "(lit \"lexlean/lock/2\")",
        vector: "an empty language-1.1 lock",
    },
    Plant {
        label: "the toolchain is another",
        root: 1,
        site: "\"leanprover/lean4:v4.32.2\"",
        replacement: "\"leanprover/lean4:v4.32.0\"",
        vector: "an empty language-1.2 lock",
    },
    Plant {
        label: "a table has no blank line before it",
        root: 1,
        site: "(cat (cat 0x0a name) 0x0a)",
        replacement: "(cat name 0x0a)",
        vector: "one workspace file",
    },
    Plant {
        label: "the pins are not sorted",
        root: 1,
        site: "0x (pairsByPath rows) : Bytes)",
        replacement: "0x rows : Bytes)",
        vector: "workspace files out of order",
    },
    Plant {
        label: "the revision is the version",
        root: 1,
        site: "(stringLine (lit \"revision\") (project row revision))",
        replacement: "(stringLine (lit \"revision\") (project row version))",
        vector: "every field of a package",
    },
    Plant {
        label: "packages are sorted by identifier alone",
        root: 1,
        site: "(cat (escapedKey (project row ident)) (escapedKey (project row version)))",
        replacement: "(escapedKey (project row ident))",
        vector: "two versions of a package",
    },
    Plant {
        label: "the identifier key has no end",
        root: 1,
        site: "0x0001) 0x0000)",
        replacement: "0x0001) 0x)",
        vector: "an identifier that is a prefix of another",
    },
    Plant {
        label: "a NUL in a key is not escaped",
        root: 1,
        site: "(LexLeanStd.Text::joinWith (LexLeanStd.Text::splitOn text (0 : UInt8)) 0x0001)",
        replacement: "text",
        vector: "an identifier with a NUL",
    },
    Plant {
        label: "imports are written last first",
        root: 1,
        site: "(LexLeanStd.Order::reverseList [Bytes] (prim list_fold (fun (acc : (List Bytes)) (item : Bytes) => (cons (tomlString item) acc)) (nil Bytes) values : (List Bytes)))",
        replacement: "(prim list_fold (fun (acc : (List Bytes)) (item : Bytes) => (cons (tomlString item) acc)) (nil Bytes) values : (List Bytes))",
        vector: "imports in their order",
    },
    Plant {
        label: "source packages are not sorted",
        root: 1,
        site: "(sourcesSorted (project given sourcePackages))",
        replacement: "(project given sourcePackages)",
        vector: "source packages out of order",
    },
    Plant {
        label: "the array separator has no space",
        root: 1,
        site: "(lit \", \")",
        replacement: "(lit \",\")",
        vector: "a pdf record",
    },
    Plant {
        label: "the pdf record is dropped",
        root: 1,
        site: "| Option.some entry => (pdfText entry)",
        replacement: "| Option.some entry => 0x",
        vector: "a pdf with no resources",
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
            vector.root(),
            plant.root,
            "{}: the vector is a run of the root planted",
            plant.label
        );
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/LockText.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/LockText.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[plant.root].0);
        match super::evaluate(&low, &[vector.run()], 1).first() {
            Some(first) => eprintln!(
                "BS-31: plant detected: {}: {}",
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
            "BS-31: {}: largest certificate part {largest} bytes",
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
        checked_units.units.contains_key("LockText"),
        "LockText is a linked module of the project"
    );
    eligibility_of(&project, "LockText", MODULE, &ROOTS);

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label.as_str()).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let mut runs: [Vec<Run>; 2] = [Vec::new(), Vec::new()];
    for vector in &vectors {
        runs[vector.root()].push(vector.run());
    }

    // Every committed lock that parses, and every text in them.
    let (locks, found, canonical) = committed();
    assert!(
        locks.iter().any(|lock| lock.pdf.is_some())
            && locks.iter().any(|lock| !lock.source_packages.is_empty())
            && locks.iter().any(|lock| !lock.packages.is_empty()),
        "the committed locks reach every table"
    );
    let words = texts_of(&locks);
    let corpus: [Vec<Run>; 2] = [
        words.iter().map(|word| string_run(word)).collect(),
        locks.iter().map(lock_run).collect(),
    ];
    let normative = support::normative_host();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (
            seeded_runs(EVALUATOR_SEEDED, 0x0B55_0031),
            seeded_runs(RUSTC_SEEDED, 0x0B55_1031),
        )
    } else {
        ([Vec::new(), Vec::new()], [Vec::new(), Vec::new()])
    };
    eprintln!(
        "BS-31: {} vectors ({} texts, {} locks), {} of {} committed locks parse ({} are their canonical text), {} texts in them, {} and {} seeded runs of each root ({:?})",
        vectors.len(),
        runs[0].len(),
        runs[1].len(),
        locks.len(),
        found,
        canonical,
        words.len(),
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
                "BS-31: {root}: the evaluator agrees on {} runs ({:?})",
                all_runs.len(),
                started.elapsed()
            );
        }
        plants(&vectors);
    } else {
        eprintln!("BS-31: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled: [Vec<Run>; 2] = [Vec::new(), Vec::new()];
    for index in 0..2 {
        compiled[index].extend(runs[index].iter().cloned());
        compiled[index].extend(corpus[index].iter().cloned());
        compiled[index].extend(rustc_seeded[index].iter().cloned());
    }
    let total = rustc_differential(
        "BS-31",
        &[
            (ROOTS[0].0, &lowered[ROOTS[0].0], compiled[0].as_slice()),
            (ROOTS[1].0, &lowered[ROOTS[1].0], compiled[1].as_slice()),
        ],
        "oracle",
    );
    eprintln!("BS-31: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-31") {
        return;
    }
    export(&lowered);
    eprintln!("BS-31: passed in {:?}", started.elapsed());
}
