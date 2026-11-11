//! `BS-15`: the package renderer of SPEC.md §17.16 as language-1.2 source
//! (`LexLeanBoot.RustPackage`), held to `calculus::rust::package`.
//!
//! `package::lower` checks a manifest's own fields before it reads the
//! program, and `package::package` writes the Cargo manifest and the
//! provenance around the printed library. `checkManifest` is the first and
//! `renderPackage` the second; the program's checks, the lowering, the
//! exports, and the library's bytes are the earlier rows, composed by the
//! host. The oracle of `checkManifest` is `package::lower` itself on the
//! manifest with its program and exports replaced by a pair every later check
//! accepts, so a refusal it gives is a refusal of the fields alone.

use std::collections::BTreeSet;

use serde_json::Value as Json;

use super::{bytes, list, text, Run, Seeded};
use crate::production_export as harness;
use crate::support;
use lexlean::artifact::canonical_json::Json as Canonical;
use lexlean::calculus::rust::package::{self as oracle, Errors, Export, Manifest, Passing};
use lexlean::calculus::rust::scaffold::{self, Gate, Level, Scaffold};
use lexlean::calculus::rust::{runtime, Profile};
use lexlean::calculus::{Expr, Function, Program, Ty, Value, PROGRAM_SPEC};

const MODULE: &str = "LexLeanBoot.RustPackage";
const ROOTS: [&str; 2] = ["checkManifest", "renderPackage"];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// The capacity of each output buffer; no package file comes near it.
const CAPACITY: usize = 1 << 24;
/// The libraries the evaluator renders around: those of at most this many
/// bytes. Under rustc every library is rendered.
const EVALUATOR_LIBRARY: usize = 16_384;

const MANIFESTS: usize = 327;
const ACCEPTED: usize = 308;
const HEADER_RULES: usize = 182;
const RENDER_RULES: usize = 137;
const HEADER_MUTANTS: usize = 2_000;
const RENDER_MUTANTS: usize = 308;
const EVALUATOR_RENDERS: usize = 241;

fn nat(value: usize) -> Value {
    Value::Nat {
        value: value.to_string(),
    }
}
fn boolean(value: bool) -> Value {
    Value::Bool { value }
}
fn record(fields: Vec<Value>) -> Value {
    Value::Adt {
        constructor: 0,
        fields,
    }
}
fn texts<'a>(items: impl IntoIterator<Item = &'a String>) -> Value {
    list(items.into_iter().map(|item| text(item)).collect())
}
fn ok(value: Value) -> Value {
    Value::Ok {
        value: Box::new(value),
    }
}
fn refused(message: &str) -> Value {
    Value::Error {
        value: Box::new(text(message)),
    }
}
fn json(value: &Value) -> Json {
    serde_json::to_value(value).expect("a value is JSON")
}

/// Every committed package manifest the oracle can read, once each: the
/// committed packages, the exported ones, and the negative manifests.
fn committed() -> Vec<Manifest> {
    let root = support::repo_root();
    let mut files = Vec::new();
    for top in ["compiler/rust", "tests/production-export"] {
        walk(&root.join(top).into_std_path_buf(), &mut files);
    }
    files.sort();
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for path in files {
        let name = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        let negative = path
            .parent()
            .and_then(|parent| parent.file_name())
            .is_some_and(|parent| parent == "negative");
        if name != "package.json" && !(negative && name.ends_with(".json")) {
            continue;
        }
        let raw = std::fs::read(&path).expect("a manifest");
        if !seen.insert(raw.clone()) {
            continue;
        }
        if let Ok(manifest) = Manifest::parse(&raw) {
            out.push(manifest);
        }
    }
    out
}

fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries {
        let path = entry.expect("an entry").path();
        if path.is_dir() {
            walk(&path, out);
        } else {
            out.push(path);
        }
    }
}

/// The manifest with a program and exports every check after the fields
/// accepts on either profile: one function from `bool` to `bool`.
fn probe(manifest: &Manifest) -> Manifest {
    let mut probe = manifest.clone();
    probe.program = Program {
        spec: PROGRAM_SPEC.to_owned(),
        adts: Vec::new(),
        functions: vec![Function {
            parameters: vec![0],
            types: vec![Ty::Bool],
            result: Ty::Bool,
            body: Expr::Var { name: 0 },
        }],
    };
    probe.exports = vec![Export {
        function: 0,
        name: "run".to_owned(),
        parameters: vec![Passing::Own],
        errors: Errors::None,
    }];
    probe
}

/// What `package::lower` makes of the manifest's own fields.
fn header_outcome(manifest: &Manifest) -> Value {
    match oracle::lower(&probe(manifest)) {
        Ok((_, _, profile)) => ok(boolean(profile == Profile::Std)),
        Err(message) => refused(&message),
    }
}

fn registered() -> Vec<String> {
    lexlean::production::registry()
        .expect("the production registry")
        .effects
        .keys()
        .cloned()
        .collect()
}

fn header_run(manifest: &Manifest, registered: &[String]) -> Run {
    (
        vec![
            text(&manifest.spec),
            text(&manifest.name),
            text(&manifest.version),
            text(&manifest.profile),
            texts(&manifest.sources),
            texts(&manifest.effects),
            texts(registered),
        ],
        json(&header_outcome(manifest)),
    )
}

fn level(level: Level) -> Value {
    Value::Adt {
        constructor: match level {
            Level::Allow => 0,
            Level::Warn => 1,
            Level::Deny => 2,
            Level::Forbid => 3,
        },
        fields: Vec::new(),
    }
}

fn gate(gate: &Gate) -> Value {
    record(vec![
        text(&gate.lint),
        level(gate.level),
        boolean(gate.priority < 0),
        nat(usize::try_from(gate.priority.unsigned_abs()).expect("a priority")),
    ])
}

fn scaffold_value(scaffold: &Scaffold) -> Value {
    record(vec![
        text(&scaffold.edition),
        boolean(scaffold.publish),
        text(&scaffold.library),
        list(scaffold.rustc.iter().map(gate).collect()),
        list(scaffold.clippy.iter().map(gate).collect()),
        list(scaffold.allowed.iter().map(|row| text(&row.lint)).collect()),
    ])
}

/// What `renderPackage` is given besides the scaffold and the library.
#[derive(Clone)]
struct Render {
    scaffold: Scaffold,
    name: String,
    version: String,
    std: bool,
    program: String,
    sources: Vec<String>,
    effects: Vec<String>,
    exports: Vec<(String, u64)>,
    library: Vec<u8>,
}

/// The two files `renderPackage` writes: the Cargo manifest and the provenance.
type Files = (Vec<u8>, Vec<u8>);

/// One change to the scaffold, for a render rule.
type Reshape = Box<dyn Fn(&mut Scaffold)>;

fn sha256(data: &[u8]) -> String {
    lexlean::Sha256Digest::of(data).to_hex()
}

fn semantics() -> String {
    lexlean::compiler_semantics_id_for(lexlean::LANGUAGE_1_2).to_hex()
}

fn runtime_text(std: bool) -> String {
    let core = runtime::core().expect("the core runtime");
    if std {
        format!("{core}{}", runtime::std().expect("the std runtime"))
    } else {
        core.to_owned()
    }
}

/// The two files as `package::package` writes them: the Cargo manifest over
/// the scaffold, and the provenance by the recipe of `package::package`,
/// read back as canonical JSON. The case holds this to `package::package` on
/// every manifest the oracle accepts, and uses it for the inputs it refuses.
fn files_of(render: &Render) -> Files {
    let cargo = oracle::cargo_toml_of(&render.scaffold, &render.name, &render.version).into_bytes();
    let provenance = serde_json::json!({
        "spec": oracle::PROVENANCE_SPEC,
        "name": render.name,
        "version": render.version,
        "profile": if render.std { "rust-std" } else { "rust-core" },
        "program": render.program,
        "compiler_semantics": semantics(),
        "runtime": sha256(runtime_text(render.std).as_bytes()),
        "sources": render.sources,
        "effects": render.effects,
        "exports": render.exports.iter().map(|(name, function)| serde_json::json!({
            "name": name,
            "function": function,
        })).collect::<Vec<_>>(),
        "files": {
            "Cargo.toml": sha256(&cargo),
            (render.scaffold.library.as_str()): sha256(&render.library),
        },
    });
    let provenance = Canonical::parse(provenance.to_string().as_bytes())
        .expect("canonical JSON")
        .to_file_bytes();
    (cargo, provenance)
}

fn render_run(render: &Render, expected: Files) -> Run {
    let exports = render
        .exports
        .iter()
        .map(|(name, function)| {
            record(vec![
                text(name),
                nat(usize::try_from(*function).expect("a function")),
            ])
        })
        .collect();
    let value = ok(Value::Pair {
        left: Box::new(bytes(&expected.0)),
        right: Box::new(bytes(&expected.1)),
    });
    (
        vec![
            nat(CAPACITY),
            scaffold_value(&render.scaffold),
            text(&render.name),
            text(&render.version),
            boolean(render.std),
            text(&render.program),
            text(&semantics()),
            text(runtime::core().expect("the core runtime")),
            text(runtime::std().expect("the std runtime")),
            texts(&render.sources),
            texts(&render.effects),
            list(exports),
            bytes(&render.library),
        ],
        json(&value),
    )
}

/// A manifest the oracle accepts, what `renderPackage` is given for it, and
/// the two files `package::package` wrote.
fn accepted(manifest: &Manifest) -> Option<(Render, Files)> {
    let package = oracle::package(manifest).ok()?;
    let render = Render {
        scaffold: scaffold::get().expect("the scaffold").clone(),
        name: manifest.name.clone(),
        version: manifest.version.clone(),
        std: manifest.profile == "rust-std",
        program: manifest.program.id().expect("an accepted program").to_hex(),
        sources: manifest.sources.clone(),
        effects: manifest.effects.clone(),
        exports: manifest
            .exports
            .iter()
            .map(|export| (export.name.clone(), export.function))
            .collect(),
        library: package.files[&scaffold::get().expect("the scaffold").library].clone(),
    };
    let files = (
        package.files["Cargo.toml"].clone(),
        package.files["provenance.json"].clone(),
    );
    assert_eq!(
        files_of(&render),
        files,
        "{}: the recipe the case uses is the one package::package writes",
        manifest.name
    );
    Some((render, files))
}

/// The fields of a manifest a mutation changes.
const NAMES: &[&str] = &[
    "",
    "a",
    "run",
    "gen",
    "raw",
    "safe",
    "Self",
    "self",
    "std",
    "core",
    "alloc",
    "proc_macro",
    "test",
    "harness",
    "harness2",
    "Ab",
    "aB",
    "1a",
    "_a",
    "a_",
    "a__b",
    "a_b",
    "a-b",
    "a.b",
    "é",
    "a\u{e9}",
    "a b",
    "sum_to_std",
];
const VERSIONS: &[&str] = &[
    "",
    "1",
    "1.0",
    "1.0.0",
    "0.0.0",
    "1.0.0.0",
    "01.0.0",
    "1.00.0",
    "1.0.01",
    "1..0",
    "1.0.",
    ".1.0",
    "+1.0.0",
    " 1.0.0",
    "1.0.0 ",
    "1.0.a",
    "18446744073709551615.0.0",
    "18446744073709551616.0.0",
    "99999999999999999999.0.0",
    "0.18446744073709551615.18446744073709551615",
    "0.0.184467440737095516150",
    "1.0.\u{661}",
];
const PROFILES: &[&str] = &["rust-std", "rust-core", "rust-wasm", "", "rust-Std", "std"];
const SPECS: &[&str] = &[
    "lexlean/rust-package/1",
    "lexlean/rust-package/2",
    "",
    "lexlean/rust-provenance/1",
];
const SOURCES: &[&str] = &[
    "09d5f3630d4f28017017da9df6a1032805f8848c3bb64aa228c953fd1534d028",
    "1111111111111111111111111111111111111111111111111111111111111111",
    "0000000000000000000000000000000000000000000000000000000000000000",
    "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
    "09D5F3630D4F28017017DA9DF6A1032805F8848C3BB64AA228C953FD1534D028",
    "09d5f3630d4f28017017da9df6a1032805f8848c3bb64aa228c953fd1534d02",
    "09d5f3630d4f28017017da9df6a1032805f8848c3bb64aa228c953fd1534d0280",
    "g9d5f3630d4f28017017da9df6a1032805f8848c3bb64aa228c953fd1534d028",
    "",
];
const EFFECTS: &[&str] = &[
    "allocation",
    "overflow",
    "recursion",
    "io",
    "Overflow",
    "",
    "allocation ",
];

fn mutate(seeded: &mut Seeded, manifest: &mut Manifest) {
    match seeded.below(6) {
        0 => manifest.name = (*seeded.pick(NAMES)).to_owned(),
        1 => manifest.version = (*seeded.pick(VERSIONS)).to_owned(),
        2 => manifest.profile = (*seeded.pick(PROFILES)).to_owned(),
        3 => manifest.spec = (*seeded.pick(SPECS)).to_owned(),
        4 => {
            let count = seeded.below(4);
            manifest.sources = (0..count)
                .map(|_| (*seeded.pick(SOURCES)).to_owned())
                .collect();
            if seeded.chance(50) {
                manifest.sources.sort();
            }
        }
        _ => {
            let count = seeded.below(4);
            manifest.effects = (0..count)
                .map(|_| (*seeded.pick(EFFECTS)).to_owned())
                .collect();
            if seeded.chance(50) {
                manifest.effects.sort();
            }
        }
    }
}

/// One manifest per refusal of the fields and per path through the checks,
/// each from both bases.
fn header_rules(bases: &[Manifest]) -> Vec<Manifest> {
    let mut out = Vec::new();
    for base in bases {
        let mut each = |edit: &dyn Fn(&mut Manifest)| {
            let mut manifest = base.clone();
            edit(&mut manifest);
            out.push(manifest);
        };
        for spec in SPECS {
            each(&|m| m.spec = (*spec).to_owned());
        }
        for name in NAMES {
            each(&|m| m.name = (*name).to_owned());
        }
        each(&|m| m.name = "a".repeat(64));
        each(&|m| m.name = "a".repeat(65));
        for version in VERSIONS {
            each(&|m| m.version = (*version).to_owned());
        }
        for profile in PROFILES {
            each(&|m| m.profile = (*profile).to_owned());
        }
        for source in SOURCES {
            each(&|m| m.sources = vec![(*source).to_owned()]);
        }
        each(&|m| m.sources = vec![SOURCES[1].to_owned(), SOURCES[0].to_owned()]);
        each(&|m| m.sources = vec![SOURCES[0].to_owned(), SOURCES[0].to_owned()]);
        each(&|m| m.sources = vec![SOURCES[0].to_owned(), SOURCES[1].to_owned()]);
        for effect in EFFECTS {
            each(&|m| m.effects = vec![(*effect).to_owned()]);
        }
        each(&|m| m.effects = vec!["recursion".to_owned(), "overflow".to_owned()]);
        each(&|m| m.effects = vec!["overflow".to_owned(), "overflow".to_owned()]);
        each(&|m| m.effects = vec!["io".to_owned(), "allocation".to_owned()]);
        each(&|m| {
            m.effects = vec![
                "allocation".to_owned(),
                "overflow".to_owned(),
                "recursion".to_owned(),
            ]
        });
        // Two failures at once: the first in the oracle's order is the one.
        each(&|m| {
            m.spec = String::new();
            m.name = "gen".to_owned();
        });
        each(&|m| {
            m.name = "gen".to_owned();
            m.version = "01.0.0".to_owned();
        });
        each(&|m| {
            m.version = "1.0".to_owned();
            m.profile = "rust-wasm".to_owned();
        });
        each(&|m| {
            m.profile = "rust-wasm".to_owned();
            m.sources = vec![String::new()];
        });
        each(&|m| {
            m.sources = vec![SOURCES[1].to_owned(), SOURCES[0].to_owned()];
            m.effects = vec!["io".to_owned()];
        });
        each(&|m| {
            m.effects = vec!["io".to_owned(), "allocation".to_owned()];
            m.profile = "rust-core".to_owned();
        });
    }
    out
}

/// Inputs of `renderPackage` that reach the escaper and the scaffold's
/// shapes: names, versions and exports `package::lower` would refuse, and
/// scaffold records that differ from the committed one.
fn render_rules(base: &Render) -> Vec<Render> {
    let awkward: Vec<String> = (0u8..0x20)
        .map(|byte| format!("a{}b", char::from(byte)))
        .chain(
            [
                "\"",
                "\\",
                "\u{7f}",
                "\u{e9}",
                "\u{65e5}",
                "\u{1f600}",
                "/",
                "'",
                "a\"b\\c",
            ]
            .iter()
            .map(|piece| format!("x{piece}y")),
        )
        .collect();
    let mut out = Vec::new();
    for name in &awkward {
        let mut render = base.clone();
        render.name = name.clone();
        out.push(render);
        let mut render = base.clone();
        render.exports = vec![(name.clone(), 0), ("run".to_owned(), 18_446_744)];
        out.push(render);
        let mut render = base.clone();
        render.effects = vec![name.clone()];
        render.sources = vec![name.clone(), name.clone()];
        out.push(render);
    }
    let mut render = base.clone();
    render.exports = Vec::new();
    render.sources = Vec::new();
    render.effects = Vec::new();
    out.push(render);
    let mut render = base.clone();
    render.std = !render.std;
    out.push(render);
    let mut render = base.clone();
    render.library = Vec::new();
    out.push(render);
    let mut render = base.clone();
    render.library = (0u8..=255).collect();
    out.push(render);
    let reshaped: Vec<Reshape> = vec![
        Box::new(|s| s.publish = true),
        Box::new(|s| s.edition = "2024".to_owned()),
        // The path is a key of `files`, whose order is the bytes of its keys.
        Box::new(|s| s.library = "lib/main.rs".to_owned()),
        Box::new(|s| s.library = "A.rs".to_owned()),
        Box::new(|s| s.library = "Cargo.toml".to_owned()),
        Box::new(|s| s.library = "a\"b.rs".to_owned()),
        Box::new(|s| s.rustc.clear()),
        Box::new(|s| s.clippy.clear()),
        Box::new(|s| s.allowed.clear()),
        Box::new(|s| {
            for (at, gate) in s.rustc.iter_mut().chain(s.clippy.iter_mut()).enumerate() {
                gate.priority = [0, 1, -1, 7, -20, i64::MAX, i64::MIN + 1][at % 7];
                gate.level = [Level::Allow, Level::Warn, Level::Deny, Level::Forbid][at % 4];
            }
        }),
    ];
    for reshape in &reshaped {
        let mut render = base.clone();
        reshape(&mut render.scaffold);
        out.push(render);
    }
    out
}

/// A defect planted in the source of `RustPackage`, and the root whose
/// differential must catch it.
struct Plant {
    label: &'static str,
    root: usize,
    site: &'static str,
    replacement: &'static str,
}

const PLANTS: [Plant; 21] = [
    Plant {
        label: "`gen` is not a keyword",
        root: 0,
        site: "macro_rules gen raw",
        replacement: "macro_rules raw",
    },
    Plant {
        label: "`harness` is an available crate name",
        root: 0,
        site: "proc_macro test harness\"",
        replacement: "proc_macro test\"",
    },
    Plant {
        label: "a part with a leading zero is canonical",
        root: 0,
        site: "(1 <? (prim length part : Nat))",
        replacement: "false",
    },
    Plant {
        label: "a part past u64::MAX fits",
        root: 0,
        site: "(not (before (text \"18446744073709551615\") digits))",
        replacement: "true",
    },
    Plant {
        label: "equal neighbours are ascending",
        root: 0,
        site: "(before previous item)",
        replacement: "(not (before item previous))",
    },
    Plant {
        label: "the order of the effects is not checked",
        root: 0,
        site: "(if (ascending effects) then",
        replacement: "(if true then",
    },
    Plant {
        label: "a rust-core package may state allocation",
        root: 0,
        site: "(core && (member effects (text \"allocation\")))",
        replacement: "false",
    },
    Plant {
        label: "a doubled underscore is snake case",
        root: 0,
        site: "(LexLeanStd.Text::find name (text \"__\"))",
        replacement: "(LexLeanStd.Text::find name (text \"___\"))",
    },
    Plant {
        label: "a crate name may have 65 bytes",
        root: 0,
        site: "<=? 64)",
        replacement: "<=? 65)",
    },
    Plant {
        label: "upper-case hex is an identity",
        root: 0,
        site: "(within byte 97 102)",
        replacement: "(within byte 65 102)",
    },
    Plant {
        label: "a profile that is neither is accepted",
        root: 0,
        site: "(if (std || core) then",
        replacement: "(if true then",
    },
    Plant {
        label: "the spec is not checked",
        root: 0,
        site: "(if (same spec (text \"lexlean/rust-package/1\")) then",
        replacement: "(if true then",
    },
    Plant {
        label: "a published crate is printed unpublished",
        root: 1,
        site: "\"\\\"\\npublish = true\")",
        replacement: "\"\\\"\\npublish = false\")",
    },
    Plant {
        label: "a negative priority loses its sign",
        root: 1,
        site: "\"\\\", priority = -\")",
        replacement: "\"\\\", priority = \")",
    },
    Plant {
        label: "the digests of Cargo.toml and lib.rs are swapped",
        root: 1,
        site: "(putEntry (put (putEntry opened (text \"Cargo.toml\") (LexLeanStd.Sha256::sha256Hex cargo)) \",\") path (LexLeanStd.Sha256::sha256Hex library))",
        replacement: "(putEntry (put (putEntry opened (text \"Cargo.toml\") (LexLeanStd.Sha256::sha256Hex library)) \",\") path (LexLeanStd.Sha256::sha256Hex cargo))",
    },
    Plant {
        label: "the library path is always src/lib.rs in Cargo.toml",
        root: 1,
        site: "(project scaffold library)) \"\\\"\\n\\n[lints.rust]",
        replacement: "(text \"src/lib.rs\")) \"\\\"\\n\\n[lints.rust]",
    },
    Plant {
        label: "a path that sorts before Cargo.toml is written after it",
        root: 1,
        site: "(before path (text \"Cargo.toml\"))",
        replacement: "false",
    },
    Plant {
        label: "a library named Cargo.toml is written as two keys",
        root: 1,
        site: "(same path (text \"Cargo.toml\"))",
        replacement: "false",
    },
    Plant {
        label: "the std runtime's digest leaves out the std part",
        root: 1,
        site: "(if std then (cat core stdRuntime) else core)",
        replacement: "core",
    },
    Plant {
        label: "U+001F is written raw",
        root: 1,
        site: "(if (n <? 32) then",
        replacement: "(if (n <? 31) then",
    },
    Plant {
        label: "a quotation mark is not escaped",
        root: 1,
        site: "(if (n =? 34) then",
        replacement: "(if (n =? 35) then",
    },
];

fn plants(header: &[Run], render: &[Run]) {
    let mut missed = Vec::new();
    for plant in &PLANTS {
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/RustPackage.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/RustPackage.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[plant.root]);
        let corpus = if plant.root == 0 { header } else { render };
        match super::evaluate(&low, corpus, 1).first() {
            Some(first) => eprintln!(
                "BS-15: plant detected: {}: {}",
                plant.label,
                &first[..first.len().min(160)]
            ),
            None => missed.push(plant.label),
        }
    }
    assert!(
        missed.is_empty(),
        "planted defects the differential does not detect: {missed:?}"
    );
}

pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("RustPackage"),
        "RustPackage is a linked module of the project"
    );
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    let report = super::rust_lower::eligibility_report(&project, &build_id, MODULE);
    let rows = report["roots"].as_array().expect("roots");
    assert_eq!(rows.len(), ROOTS.len(), "the roots of RustPackage");
    for (name, row) in ROOTS.iter().zip(rows) {
        assert_eq!(row["root"], format!("{MODULE}.{name}"));
        super::rust_lower::assert_eligible(row, name, &["allocation", "overflow"]);
    }

    let registered = registered();
    let manifests = committed();
    // The committed refusals are a second oracle of the fields' checks.
    let negative = support::repo_root().join("compiler/rust/negative");
    for entry in std::fs::read_dir(negative.as_std_path()).expect("the negative manifests") {
        let path = entry.expect("an entry").path();
        if path
            .extension()
            .is_some_and(|extension| extension == "json")
        {
            let Ok(manifest) = Manifest::parse(&std::fs::read(&path).expect("a manifest")) else {
                continue;
            };
            let refusal =
                std::fs::read_to_string(path.with_extension("error")).expect("its refusal");
            // A committed refusal is the part of the message the fixture
            // pins, so the fields' refusal contains it.
            if let Err(message) = oracle::lower(&probe(&manifest)) {
                assert!(
                    message.contains(refusal.trim_end_matches('\n')),
                    "{}: the fields' refusal `{message}` contains the committed one",
                    path.display()
                );
            }
        }
    }
    let renders: Vec<(Render, Files)> = manifests.iter().filter_map(accepted).collect();
    let std_base = manifests
        .iter()
        .find(|manifest| manifest.profile == "rust-std" && oracle::package(manifest).is_ok())
        .expect("an accepted rust-std manifest");
    let core_base = manifests
        .iter()
        .find(|manifest| manifest.profile == "rust-core" && oracle::package(manifest).is_ok())
        .expect("an accepted rust-core manifest");
    let rules = header_rules(&[std_base.clone(), core_base.clone()]);
    let mut seeded = Seeded(0x00B5_0115);
    let mutants: Vec<Manifest> = (0..2_000)
        .map(|_| {
            let mut manifest = seeded.pick(&manifests).clone();
            for _ in 0..=seeded.below(3) {
                mutate(&mut seeded, &mut manifest);
            }
            manifest
        })
        .collect();
    let header: Vec<Run> = manifests
        .iter()
        .chain(&rules)
        .chain(&mutants)
        .map(|manifest| header_run(manifest, &registered))
        .collect();

    // The shaped inputs are held to the recipe, which reads the library only
    // as bytes to digest, so a prefix of it keeps them within the
    // evaluator's bound.
    let mut base = renders
        .iter()
        .find(|(render, _)| render.std)
        .expect("an accepted rust-std render")
        .0
        .clone();
    base.library.truncate(4_096);
    let shaped = render_rules(&base);
    let mut render_mutants = Vec::new();
    for (at, (render, _)) in renders.iter().enumerate() {
        let mut render = render.clone();
        render.name = (*seeded.pick(&["a", "b_c", "sum_to_std", "x9"])).to_owned();
        render.version = (*seeded.pick(&["0.0.0", "1.2.3", "18446744073709551615.0.1"])).to_owned();
        if at % 2 == 0 {
            render.sources.push(SOURCES[3].to_owned());
        }
        if at % 3 == 0 {
            render.exports.reverse();
        }
        render_mutants.push(render);
    }
    let render_runs: Vec<Run> = renders
        .iter()
        .map(|(render, files)| render_run(render, files.clone()))
        .chain(
            shaped
                .iter()
                .chain(&render_mutants)
                .map(|render| render_run(render, files_of(render))),
        )
        .collect();
    let within: Vec<Run> = renders
        .iter()
        .map(|(render, _)| render)
        .chain(&shaped)
        .chain(&render_mutants)
        .zip(&render_runs)
        .filter(|(render, _)| render.library.len() <= EVALUATOR_LIBRARY)
        .map(|(_, run)| run.clone())
        .collect();
    // rustc compiles each byte of an argument as one array element, and the
    // libraries of the committed packages are megabytes together, so a render
    // whose library is beyond the evaluator's bound runs under rustc on the
    // first 4,096 bytes of it, held to the recipe, which reads it only as
    // bytes to digest.
    let compiled: Vec<Run> = renders
        .iter()
        .map(|(render, _)| render)
        .chain(&shaped)
        .chain(&render_mutants)
        .zip(&render_runs)
        .map(|(render, run)| {
            if render.library.len() <= EVALUATOR_LIBRARY {
                return run.clone();
            }
            let mut render = render.clone();
            render.library.truncate(4_096);
            render_run(&render, files_of(&render))
        })
        .collect();
    eprintln!(
        "BS-15: {} manifests ({} accepted), {} and {} rule inputs, {} and {} mutants, {} renders under the evaluator ({:?})",
        manifests.len(),
        renders.len(),
        rules.len(),
        shaped.len(),
        mutants.len(),
        render_mutants.len(),
        within.len(),
        started.elapsed()
    );
    assert_eq!(
        (
            manifests.len(),
            renders.len(),
            rules.len(),
            shaped.len(),
            mutants.len(),
            render_mutants.len(),
            within.len()
        ),
        (
            MANIFESTS,
            ACCEPTED,
            HEADER_RULES,
            RENDER_RULES,
            HEADER_MUTANTS,
            RENDER_MUTANTS,
            EVALUATOR_RENDERS
        ),
        "the manifests, the accepted ones, the rule inputs, the mutants, and the renders the evaluator is given"
    );

    let checked = support::checked_project(&project);
    let check_root = super::lower(&checked, &project, MODULE, "checkManifest");
    let render_root = super::lower(&checked, &project, MODULE, "renderPackage");
    if support::normative_host() {
        let disagreements = super::evaluate(&check_root, &header, 5);
        assert!(
            disagreements.is_empty(),
            "checkManifest and package::lower disagree: {disagreements:?}"
        );
        let disagreements = super::evaluate(&render_root, &within, 5);
        assert!(
            disagreements.is_empty(),
            "renderPackage and package::package disagree: {disagreements:?}"
        );
        eprintln!(
            "BS-15: the evaluator agrees on {} manifests and {} renders ({:?})",
            header.len(),
            within.len(),
            started.elapsed()
        );
        plants(&header, &within);
    } else {
        eprintln!("BS-15: platform-bound host: the evaluator and the planted defects run on the normative host");
    }
    super::rust_validate::rustc_differential(
        "BS-15",
        &[
            ("checkManifest", &check_root, &header),
            ("renderPackage", &render_root, &compiled),
        ],
        "oracle",
    );
    if !support::lean_backed("BS-15") {
        return;
    }
    let exported = harness::exported("bootstrap");
    let bundle = exported.bundle();
    let mut realized = 0usize;
    for exported_root in &exported.output.result.roots {
        let low = match exported_root.root.strip_prefix(&format!("{MODULE}.")) {
            Some("checkManifest") => &check_root,
            Some("renderPackage") => &render_root,
            _ => continue,
        };
        let largest = exported_root
            .outputs
            .iter()
            .flat_map(|output| &output.certification.certificates)
            .map(|certificate| certificate.file.byte_length)
            .max()
            .expect("a certificate");
        eprintln!(
            "BS-15: {}: largest certificate part {largest} bytes",
            exported_root.root
        );
        assert!(
            largest <= CERTIFICATE_BUDGET,
            "{}: a certificate part of {largest} bytes is beyond {CERTIFICATE_BUDGET}",
            exported_root.root
        );
        for output in &exported_root.outputs {
            let package: Json = serde_json::from_slice(
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
                "{}: the exported package realizes the program the differential ran",
                exported_root.root
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
    eprintln!("BS-15: passed in {:?}", started.elapsed());
}
