//! Canonical JSON writing in the standard library (SPEC.md §17.22, `SL-11`,
//! #276): the `LexLeanStd.Json` module against the compiler's own writer
//! (stage 0), `artifact::canonical_json::Json`, under the calculus evaluator
//! and under the pinned rustc.
//!
//! The oracle's object is a `BTreeMap`, so its keys are in UTF-8 byte order
//! and of a repeated key the last insertion stays. The module's value keeps
//! an object as the members it was given, in the order it was given them, so
//! a tree here is built as members and the oracle inserts them in order.

use std::collections::{BTreeMap, BTreeSet};

use lexlean::artifact::canonical_json::Json as Canonical;
use lexlean::calculus::rust::package::Passing;
use lexlean::calculus::rust::{self, Profile};
use lexlean::calculus::{self as target, interp, Value};
use lexlean::production::lower::Lowered;
use serde_json::Value as Observed;

use super::stdlib::{lower_in, Run};
use crate::production_export as harness;
use crate::rust_build::{cargo_in, workspace, Built};
use crate::rust_harness::{self, Calls};
use crate::rust_packages;
use crate::support::{self, P};

/// The module.
const MODULE: &str = "LexLeanStd.Json";

/// The members in the order the module declares them, each with the effects
/// its declaration admits, which are exactly the effects its closure
/// realizes.
const MEMBERS: [(&str, &[&str]); 3] = [
    ("escapeString", &["allocation", "overflow"]),
    ("writeJson", &["allocation", "overflow", "recursion"]),
    ("writeJsonFile", &["allocation", "overflow", "recursion"]),
];

/// Argument lists per member under the evaluator.
const EVALUATOR_RUNS: usize = 300;
/// Argument lists per member under rustc.
const RUSTC_RUNS: usize = 1_500;
/// The evaluator's fuel; a member that exhausts it fails the case.
const FUEL: u64 = 200_000_000;
/// The deepest tree a seeded run writes.
const MAX_DEPTH: usize = 4;
/// The most elements or members of an array or an object.
const MAX_WIDTH: usize = 4;

/// The pieces a string is made of: the escapes of §21.7, the controls that
/// take the `\u00xx` form and the one that does not (DEL), and scalars of two,
/// three and four bytes.
const STRING_ATOMS: &[&str] = &[
    "", "a", "b", "ab", "\"", "\\", "/", " ", "\n", "\r", "\t", "\u{8}", "\u{c}", "\u{0}", "\u{1}",
    "\u{1f}", "\u{7f}", "é", "€", "😀",
];
/// The pieces a key is made of: few enough that keys repeat and one is often
/// a prefix of another, with a byte below and a byte above ASCII.
const KEY_ATOMS: &[&str] = &["a", "b", "\u{1}", "é", "\"", "€"];

/// A deterministic generator (xorshift64*), so the corpus is the same on
/// every host.
struct Seeded(u64);

impl Seeded {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    fn below(&mut self, bound: usize) -> usize {
        usize::try_from(self.next() % u64::try_from(bound).expect("a bound")).expect("below")
    }
    fn text(&mut self, atoms: &[&str], longest: usize) -> String {
        (0..self.below(longest + 1))
            .map(|_| atoms[self.below(atoms.len())])
            .collect()
    }
    fn integer(&mut self) -> i64 {
        const EDGES: [i64; 9] = [0, 1, -1, 9, 10, 99, 100, i64::MAX, i64::MIN];
        match self.below(4) {
            0 => EDGES[self.below(EDGES.len())],
            1 => i64::try_from(self.below(2_000)).expect("small") - 1_000,
            _ => i64::from_ne_bytes(self.next().to_ne_bytes()),
        }
    }
}

/// A tree as it is given: an object is its members in input order, repeats
/// included.
#[derive(Clone, Debug)]
enum Tree {
    Bool(bool),
    Int(i64),
    Str(String),
    Arr(Vec<Tree>),
    Obj(Vec<(String, Tree)>),
}

impl Tree {
    /// The tree the oracle writes: an object inserts its members in order, so
    /// the last of a repeated key stays and the keys are in byte order.
    fn oracle(&self) -> Canonical {
        match self {
            Self::Bool(value) => Canonical::Bool(*value),
            Self::Int(value) => Canonical::Int(*value),
            Self::Str(value) => Canonical::Str(value.clone()),
            Self::Arr(items) => Canonical::Arr(items.iter().map(Self::oracle).collect()),
            Self::Obj(members) => {
                let mut object = BTreeMap::new();
                for (key, value) in members {
                    object.insert(key.clone(), value.oracle());
                }
                Canonical::Obj(object)
            }
        }
    }

    /// The module's value: the constructors in declaration order, `JBool`
    /// first, with the elements of an array and the members of an object as
    /// spines of `JItem` and `JMember` ended by `JNil`.
    fn value(&self) -> Value {
        const BOOL: u64 = 0;
        const INT: u64 = 1;
        const STR: u64 = 2;
        const ARR: u64 = 3;
        const OBJ: u64 = 4;
        const NIL: u64 = 5;
        const ITEM: u64 = 6;
        const MEMBER: u64 = 7;
        let adt = |constructor: u64, fields: Vec<Value>| Value::Adt {
            constructor,
            fields,
        };
        match self {
            Self::Bool(value) => adt(BOOL, vec![Value::Bool { value: *value }]),
            Self::Int(value) => adt(
                INT,
                vec![
                    Value::Bool { value: *value < 0 },
                    Value::Nat {
                        value: value.unsigned_abs().to_string(),
                    },
                ],
            ),
            Self::Str(value) => adt(STR, vec![bytes(value.as_bytes())]),
            Self::Arr(items) => adt(
                ARR,
                vec![items.iter().rev().fold(adt(NIL, vec![]), |rest, item| {
                    adt(ITEM, vec![item.value(), rest])
                })],
            ),
            Self::Obj(members) => adt(
                OBJ,
                vec![members
                    .iter()
                    .rev()
                    .fold(adt(NIL, vec![]), |rest, (key, value)| {
                        adt(MEMBER, vec![bytes(key.as_bytes()), value.value(), rest])
                    })],
            ),
        }
    }
}

/// A seeded tree. With `repeats` off, no object holds one key twice.
fn tree(seeded: &mut Seeded, depth: usize, repeats: bool) -> Tree {
    let kinds = if depth == 0 { 3 } else { 5 };
    match seeded.below(kinds) {
        0 => Tree::Bool(seeded.below(2) == 0),
        1 => Tree::Int(seeded.integer()),
        2 => Tree::Str(seeded.text(STRING_ATOMS, 4)),
        3 => Tree::Arr(
            (0..seeded.below(MAX_WIDTH + 1))
                .map(|_| tree(seeded, depth - 1, repeats))
                .collect(),
        ),
        _ => {
            let mut seen = BTreeSet::new();
            let mut members = Vec::new();
            for _ in 0..seeded.below(MAX_WIDTH + 2) {
                let key = seeded.text(KEY_ATOMS, 3);
                if repeats || seen.insert(key.clone()) {
                    members.push((key, tree(seeded, depth - 1, repeats)));
                }
            }
            Tree::Obj(members)
        }
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
fn bytes(value: &[u8]) -> Value {
    Value::Bytes { hex: hex(value) }
}
fn observed(value: &Value) -> Observed {
    serde_json::to_value(value).expect("a value is JSON")
}

/// The corpus of one member: its edge cases, then seeded inputs up to
/// `runs`, each with the outcome of the member's oracle. With `repeats` off
/// no object repeats a key.
#[allow(clippy::too_many_lines)]
fn corpus(member: &str, runs: usize, repeats: bool) -> Vec<Run> {
    let mut seed = 0x9e37_79b9_7f4a_7c15_u64;
    for character in member.bytes() {
        seed = seed.rotate_left(8) ^ u64::from(character);
    }
    let mut seeded = Seeded(seed);
    let mut out: Vec<Run> = Vec::new();
    if member == "escapeString" {
        let escaped = |text: &str| -> Observed {
            observed(&bytes(
                Canonical::Str(text.to_owned())
                    .to_canonical_string()
                    .as_bytes(),
            ))
        };
        let mut fixed: Vec<String> = STRING_ATOMS.iter().map(|atom| (*atom).to_owned()).collect();
        // Every byte below 0x20 alone, and the three bytes around the escapes.
        fixed.extend((0u8..0x20).map(|byte| char::from(byte).to_string()));
        fixed.push("\u{1f}\u{20}\u{21}".to_owned());
        for text in &fixed {
            out.push((vec![bytes(text.as_bytes())], escaped(text)));
        }
        while out.len() < runs {
            let text = seeded.text(STRING_ATOMS, 12);
            out.push((vec![bytes(text.as_bytes())], escaped(&text)));
        }
        return out;
    }
    let written = |tree: &Tree| -> Observed {
        let canonical = tree.oracle();
        if member == "writeJsonFile" {
            observed(&bytes(&canonical.to_file_bytes()))
        } else {
            observed(&bytes(canonical.to_canonical_string().as_bytes()))
        }
    };
    let mut fixed = vec![
        Tree::Bool(true),
        Tree::Bool(false),
        Tree::Int(0),
        Tree::Int(-1),
        Tree::Int(i64::MAX),
        Tree::Int(i64::MIN),
        Tree::Str(String::new()),
        Tree::Arr(vec![]),
        Tree::Obj(vec![]),
        Tree::Arr(vec![Tree::Arr(vec![]), Tree::Obj(vec![]), Tree::Int(7)]),
        Tree::Obj(vec![
            ("b".to_owned(), Tree::Int(2)),
            ("a".to_owned(), Tree::Int(1)),
            ("ab".to_owned(), Tree::Int(3)),
            ("".to_owned(), Tree::Bool(true)),
            ("é".to_owned(), Tree::Str("é\u{1}".to_owned())),
        ]),
    ];
    if repeats {
        fixed.push(Tree::Obj(vec![
            ("k".to_owned(), Tree::Int(1)),
            ("k".to_owned(), Tree::Int(2)),
            ("j".to_owned(), Tree::Int(3)),
            ("k".to_owned(), Tree::Int(4)),
        ]));
    }
    for tree in &fixed {
        out.push((vec![tree.value()], written(tree)));
    }
    // A negative zero is the integer zero.
    out.push((
        vec![Value::Adt {
            constructor: 1,
            fields: vec![
                Value::Bool { value: true },
                Value::Nat {
                    value: "0".to_owned(),
                },
            ],
        }],
        written(&Tree::Int(0)),
    ));
    while out.len() < runs {
        let tree = tree(&mut seeded, MAX_DEPTH, repeats);
        out.push((vec![tree.value()], written(&tree)));
    }
    out
}

/// Run one lowered member on one argument list under the evaluator.
fn observe(lowered: &Lowered, arguments: &[Value]) -> Observed {
    let outcome = interp::run(&lowered.program, FUEL, lowered.entry(), arguments);
    rust::observable(&outcome).unwrap_or_else(|| panic!("on {arguments:?}: {outcome:?}"))
}

/// Every run of `corpus` on which the evaluator and the oracle disagree.
fn evaluate(lowered: &Lowered, corpus: &[Run]) -> Vec<String> {
    corpus
        .iter()
        .enumerate()
        .filter_map(|(run, (arguments, expected))| {
            let observed = observe(lowered, arguments);
            (&observed != expected)
                .then(|| format!("run {run} on {arguments:?}: {observed} != {expected}"))
        })
        .collect()
}

fn snake(member: &str) -> String {
    let mut out = String::new();
    for character in member.chars() {
        if character.is_ascii_uppercase() {
            out.push('_');
            out.push(character.to_ascii_lowercase());
        } else {
            out.push(character);
        }
    }
    out
}

/// Run the case for `SL-11`.
///
/// # Panics
///
/// Panics when the case's assertion fails.
#[allow(clippy::too_many_lines)]
pub fn run(id: &str) {
    assert_eq!(id, "SL-11", "the case of {MODULE}");
    let project = P::stdlib();
    project.check_ok();
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    let report: Observed = serde_json::from_slice(
        &std::fs::read(
            project
                .build_dir(&build_id)
                .join("production/LexLeanStd/Json.eligibility.json")
                .as_std_path(),
        )
        .expect("the eligibility report"),
    )
    .expect("the report is JSON");
    let reported = report["roots"].as_array().expect("roots");
    assert_eq!(
        reported
            .iter()
            .map(|row| row["root"].as_str().expect("root").to_owned())
            .collect::<Vec<_>>(),
        MEMBERS
            .iter()
            .map(|(member, _)| format!("{MODULE}.{member}"))
            .collect::<Vec<_>>(),
        "the three members are the module's production roots, in order"
    );
    for (row, (member, effects)) in reported.iter().zip(MEMBERS) {
        let declared: Vec<&str> = row["declared_effects"]
            .as_array()
            .expect("declared effects")
            .iter()
            .map(|effect| effect.as_str().expect("an effect"))
            .collect();
        assert_eq!(declared, effects, "{member}: the declared effects");
        let targets = row["targets"].as_array().expect("targets");
        assert_eq!(targets.len(), 1, "{member}: one target");
        let target = &targets[0];
        assert_eq!(target["target"], "rust-std", "{member}: the target");
        assert_eq!(target["status"], "eligible", "{member}: eligible");
        let realized: Vec<&str> = target["effects"]
            .as_array()
            .expect("effects")
            .iter()
            .map(|effect| effect["effect"].as_str().expect("an effect"))
            .collect();
        assert_eq!(
            realized, effects,
            "{member}: the declaration admits exactly the realized effects"
        );
    }
    // Every member lowers once; the evaluator runs its corpus.
    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, Lowered> = MEMBERS
        .iter()
        .map(|(member, _)| (*member, lower_in(&checked, &project, MODULE, member)))
        .collect();
    for (member, _) in MEMBERS {
        let disagreements = evaluate(&lowered[member], &corpus(member, EVALUATOR_RUNS, true));
        assert!(
            disagreements.is_empty(),
            "{member}: the evaluator and the oracle disagree on {} of {EVALUATOR_RUNS} runs: {:?}",
            disagreements.len(),
            &disagreements[..disagreements.len().min(5)]
        );
    }
    // The rendered rust-std package of every member, compiled by the pinned
    // rustc, runs the larger corpus.
    let dir = tempfile::Builder::new()
        .prefix("lexlean-json-")
        .tempdir()
        .expect("tempdir");
    let mut crates = Vec::new();
    let mut expected: BTreeMap<String, Vec<Observed>> = BTreeMap::new();
    for (member, _) in MEMBERS {
        let low = &lowered[member];
        let manifest = rust_packages::manifest(
            &format!("json_{}", snake(member)),
            &low.program,
            low.entry(),
            Profile::Std,
        );
        let package = target::package(&manifest.to_file_bytes().expect("canonical manifest"))
            .unwrap_or_else(|failure| panic!("{member}: {failure:?}"));
        let runs = corpus(member, RUSTC_RUNS, true);
        let arguments: Vec<Vec<Value>> = runs
            .iter()
            .map(|(arguments, _)| arguments.clone())
            .collect();
        let arity = low.program.functions[usize::try_from(low.entry()).expect("entry")]
            .types
            .len();
        let passing = vec![Passing::Own; arity];
        let harness_source = rust_harness::render_calls(
            &low.program,
            Profile::Std,
            &manifest.name,
            &[Calls {
                entry: low.entry(),
                function: "run",
                passing: &passing,
                arguments: &arguments,
            }],
        )
        .expect("the differential harness");
        expected.insert(
            manifest.name.clone(),
            runs.into_iter().map(|(_, expected)| expected).collect(),
        );
        crates.push(Built {
            name: manifest.name,
            files: package.files,
            harness: harness_source,
        });
    }
    workspace(dir.path(), &crates);
    let build = cargo_in(
        dir.path(),
        &["build", "--offline", "--workspace", "--quiet"],
    );
    assert!(
        build.status.success(),
        "a member's package does not build:\n{}",
        String::from_utf8_lossy(&build.stderr)
    );
    for (name, outcomes) in &expected {
        let binary = dir
            .path()
            .join("target/debug")
            .join(format!("run_{name}{}", std::env::consts::EXE_SUFFIX));
        let ran = std::process::Command::new(&binary)
            .output()
            .expect("the differential runs");
        assert!(
            ran.status.success(),
            "{name}: {}",
            String::from_utf8_lossy(&ran.stderr)
        );
        crate::stack_bounds::observe(&crate::stack_bounds::subject("stdlib", name), &binary);
        let stdout = String::from_utf8(ran.stdout).expect("utf8");
        let printed: Vec<Observed> = stdout
            .lines()
            .filter(|line| !line.starts_with("work "))
            .map(|line| {
                serde_json::from_str(line).unwrap_or_else(|error| panic!("{name}: {error}: {line}"))
            })
            .collect();
        assert_eq!(printed.len(), outcomes.len(), "{name}: one outcome per run");
        let disagreements: Vec<String> = printed
            .iter()
            .zip(outcomes)
            .enumerate()
            .filter(|(_, (printed, expected))| printed != expected)
            .map(|(run, (printed, expected))| format!("run {run}: {printed} != {expected}"))
            .collect();
        assert!(
            disagreements.is_empty(),
            "{name}: the rendering and the oracle disagree on {} of {} runs: {:?}",
            disagreements.len(),
            outcomes.len(),
            &disagreements[..disagreements.len().min(5)]
        );
    }
    // The gate is armed: a writer that keeps the first of a repeated key
    // still orders every key and escapes every byte, and is caught only by
    // trees that repeat a key.
    let planted = P::stdlib();
    let text = planted.read_raw("src/Json.lex.tex");
    let site = "then (cons entry rest) else";
    assert_eq!(text.matches(site).count(), 1, "the plant site exists once");
    planted.write(
        "src/Json.lex.tex",
        &text.replacen(site, "then kept else", 1),
    );
    let checked = support::checked_project(&planted);
    let write_json = lower_in(&checked, &planted, MODULE, "writeJson");
    let disagreements = evaluate(&write_json, &corpus("writeJson", EVALUATOR_RUNS, true));
    assert!(
        !disagreements.is_empty(),
        "a writer that keeps the first of a repeated key is detected by the differential"
    );
    assert!(
        evaluate(&write_json, &corpus("writeJson", EVALUATOR_RUNS, false)).is_empty(),
        "the planted writer still orders every key, so only a repeated key catches it"
    );
    // Under the pinned Lean, the production export certifies every member;
    // its packages are the committed ones and realize the programs the
    // differential ran.
    if !support::lean_backed(id) {
        return;
    }
    let exported = harness::exported("stdlib");
    let bundle = exported.bundle();
    let packages = harness::tree(&bundle.join("packages"));
    let committed = harness::tree(&harness::fixture_dir("stdlib").join("packages"));
    assert_eq!(
        packages.keys().collect::<Vec<_>>(),
        committed.keys().collect::<Vec<_>>(),
        "the exported package files are the committed ones"
    );
    for (relative, content) in &packages {
        assert!(
            committed.get(relative) == Some(content),
            "packages/{relative} differs from the committed package"
        );
    }
    let mut realized = 0usize;
    for root in &exported.output.result.roots {
        // The export carries every module of the project; this case holds
        // the `Json` members.
        let Some(member) = root.root.strip_prefix(&format!("{MODULE}.")) else {
            continue;
        };
        for output in &root.outputs {
            let package: Observed = serde_json::from_slice(
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
                serde_json::to_value(&lowered[member].program).expect("a program is JSON"),
                "{member}: the exported package realizes the program the differential ran"
            );
            realized += 1;
        }
    }
    assert_eq!(realized, MEMBERS.len(), "one certified package per member");
}
