//! Sorting and order in the standard library (SPEC.md §17.22, `SL-10`,
//! #276): the `LexLeanStd.Order` module against the Rust the compiler
//! (stage 0) sorts with, under the calculus evaluator and under the pinned
//! rustc.
//!
//! The order is the byte order of `Ord for [u8]` (lexicographic, a proper
//! prefix first), which is the order `compare_bytes` states and the order of
//! every canonical identity. The oracles are `slice::sort_by` on the key
//! (stable: entries with equal keys keep their input order), `slice::sort`,
//! a `BTreeSet`, and a strict `windows(2)` comparison.

use std::collections::{BTreeMap, BTreeSet};

use lexlean::calculus::rust::package::Passing;
use lexlean::calculus::rust::{self, Profile};
use lexlean::calculus::{self as target, interp, Value};
use lexlean::production::lower::Lowered;
use serde_json::Value as Json;

use super::stdlib::{lower_in, Run};
use crate::production_export as harness;
use crate::rust_build::{cargo_in, workspace, Built};
use crate::rust_harness::{self, Calls};
use crate::rust_packages;
use crate::support::{self, P};

/// The module.
const MODULE: &str = "LexLeanStd.Order";

/// The members in the order the module declares them, each with the effects
/// its declaration admits, which are exactly the effects its closure
/// realizes.
const MEMBERS: [(&str, &[&str]); 5] = [
    ("sortBytes", &["allocation", "overflow"]),
    ("sortEntries", &["allocation", "overflow"]),
    ("sortedSet", &["allocation", "overflow"]),
    ("insertSorted", &["allocation"]),
    ("isAscending", &["allocation"]),
];

/// Argument lists per member under the evaluator.
const EVALUATOR_RUNS: usize = 400;
/// Argument lists per member under rustc.
const RUSTC_RUNS: usize = 2_000;
/// The evaluator's fuel; a member that exhausts it fails the case.
const FUEL: u64 = 50_000_000;
/// The longest list a seeded run sorts.
const MAX_ITEMS: usize = 24;

/// The bytes a key is drawn from: few enough that keys repeat and one is
/// often a prefix of another, with both ends of the byte range.
const KEY_BYTES: &[u8] = &[0x00, 0x01, 0x61, 0x62, 0x7f, 0x80, 0xff];

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
    /// A key of at most four bytes over `KEY_BYTES`.
    fn key(&mut self) -> Vec<u8> {
        (0..self.below(5))
            .map(|_| KEY_BYTES[self.below(KEY_BYTES.len())])
            .collect()
    }
    fn keys(&mut self) -> Vec<Vec<u8>> {
        (0..self.below(MAX_ITEMS + 1)).map(|_| self.key()).collect()
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
fn bytes(value: &[u8]) -> Value {
    Value::Bytes { hex: hex(value) }
}
fn list(items: impl IntoIterator<Item = Value>) -> Value {
    Value::List {
        items: items.into_iter().collect(),
    }
}
fn byte_list(items: &[Vec<u8>]) -> Value {
    list(items.iter().map(|item| bytes(item)))
}
fn entry_list(entries: &[(Vec<u8>, Vec<u8>)]) -> Value {
    list(entries.iter().map(|(key, value)| Value::Pair {
        left: Box::new(bytes(key)),
        right: Box::new(bytes(value)),
    }))
}
fn json(value: &Value) -> Json {
    serde_json::to_value(value).expect("a value is JSON")
}

/// `insertSorted`'s oracle: the strictly ascending list with the key in it
/// (a `BTreeSet` insertion). Its corpus is strictly ascending lists, the
/// member's precondition.
fn insert_sorted(items: &[Vec<u8>], key: &[u8]) -> Vec<Vec<u8>> {
    let mut set: BTreeSet<Vec<u8>> = items.iter().cloned().collect();
    set.insert(key.to_vec());
    set.into_iter().collect()
}

/// The corpus of one member: its edge cases, then seeded inputs up to
/// `runs`, each with the outcome of the member's oracle.
#[allow(clippy::too_many_lines)]
fn corpus(member: &str, runs: usize) -> Vec<Run> {
    let mut seed = 0x9e37_79b9_7f4a_7c15_u64;
    for character in member.bytes() {
        seed = seed.rotate_left(8) ^ u64::from(character);
    }
    let mut seeded = Seeded(seed);
    let mut out: Vec<Run> = Vec::new();
    let fixed: Vec<Vec<Vec<u8>>> = vec![
        vec![],
        vec![b"".to_vec()],
        vec![b"a".to_vec(), b"".to_vec()],
        vec![b"ab".to_vec(), b"a".to_vec(), b"b".to_vec()],
        vec![b"b".to_vec(), b"b".to_vec(), b"a".to_vec(), b"b".to_vec()],
        vec![vec![0xff], vec![0x00], vec![0x80], vec![0x7f]],
        vec![vec![0x00, 0x00], vec![0x00], vec![]],
        (0..=MAX_ITEMS)
            .rev()
            .map(|n| vec![u8::try_from(n).expect("a byte")])
            .collect(),
    ];
    match member {
        "sortBytes" | "sortedSet" | "isAscending" => {
            let oracle = |items: &[Vec<u8>]| -> Json {
                match member {
                    "sortBytes" => {
                        let mut sorted = items.to_vec();
                        sorted.sort();
                        json(&byte_list(&sorted))
                    }
                    "sortedSet" => {
                        let set: BTreeSet<Vec<u8>> = items.iter().cloned().collect();
                        json(&byte_list(&set.into_iter().collect::<Vec<_>>()))
                    }
                    _ => json(&Value::Bool {
                        value: items.windows(2).all(|pair| pair[0] < pair[1]),
                    }),
                }
            };
            for items in &fixed {
                out.push((vec![byte_list(items)], oracle(items)));
            }
            while out.len() < runs {
                let mut items = seeded.keys();
                // `isAscending` needs ascending inputs to be common.
                if member == "isAscending" && seeded.below(2) == 0 {
                    let set: BTreeSet<Vec<u8>> = items.into_iter().collect();
                    items = set.into_iter().collect();
                    if seeded.below(3) == 0 && !items.is_empty() {
                        let at = seeded.below(items.len());
                        let again = items[at].clone();
                        items.insert(at, again);
                    }
                }
                out.push((vec![byte_list(&items)], oracle(&items)));
            }
        }
        "sortEntries" => {
            // The value of an entry is its input position, so a sort that
            // moves two entries of one key past each other is visible.
            let tag = |items: &[Vec<u8>]| -> Vec<(Vec<u8>, Vec<u8>)> {
                items
                    .iter()
                    .enumerate()
                    .map(|(at, key)| (key.clone(), vec![u8::try_from(at).expect("a position")]))
                    .collect()
            };
            let oracle = |entries: &[(Vec<u8>, Vec<u8>)]| -> Json {
                let mut sorted = entries.to_vec();
                sorted.sort_by(|left, right| left.0.cmp(&right.0));
                json(&entry_list(&sorted))
            };
            for items in &fixed {
                let entries = tag(items);
                out.push((vec![entry_list(&entries)], oracle(&entries)));
            }
            while out.len() < runs {
                let entries = tag(&seeded.keys());
                out.push((vec![entry_list(&entries)], oracle(&entries)));
            }
        }
        "insertSorted" => {
            let ascending = |items: &[Vec<u8>]| -> Vec<Vec<u8>> {
                let set: BTreeSet<Vec<u8>> = items.iter().cloned().collect();
                set.into_iter().collect()
            };
            for items in &fixed {
                let items = ascending(items);
                for key in [b"".to_vec(), b"a".to_vec(), vec![0xff, 0xff]] {
                    let expected = json(&byte_list(&insert_sorted(&items, &key)));
                    out.push((vec![byte_list(&items), bytes(&key)], expected));
                }
            }
            while out.len() < runs {
                let items = ascending(&seeded.keys());
                // A key already present half the time when there is one.
                let key = if !items.is_empty() && seeded.below(2) == 0 {
                    items[seeded.below(items.len())].clone()
                } else {
                    seeded.key()
                };
                let expected = json(&byte_list(&insert_sorted(&items, &key)));
                out.push((vec![byte_list(&items), bytes(&key)], expected));
            }
        }
        other => panic!("`{other}` is not a member of {MODULE}"),
    }
    out
}

/// Run one lowered member on one argument list under the evaluator.
fn observe(lowered: &Lowered, arguments: &[Value]) -> Json {
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

/// Every run of `sortBytes`'s and `sortedSet`'s corpora whose result
/// `isAscending` does not judge as the result's own order: a sorted set is
/// strictly ascending, and a sorted list is strictly ascending exactly when
/// its input had no repeated key.
fn ascends(sort: &Lowered, set: &Lowered, check: &Lowered, corpus: &[Run]) -> Vec<String> {
    corpus
        .iter()
        .filter_map(|(arguments, _)| {
            let Value::List { items } = &arguments[0] else {
                panic!("a list argument");
            };
            let distinct = items.iter().collect::<BTreeSet<_>>().len() == items.len();
            let sorted: Value =
                serde_json::from_value(observe(sort, arguments)).expect("a sorted list");
            let deduped: Value =
                serde_json::from_value(observe(set, arguments)).expect("a sorted set");
            let sorted_ascends = observe(check, std::slice::from_ref(&sorted));
            let set_ascends = observe(check, std::slice::from_ref(&deduped));
            let want = |value: bool| json(&Value::Bool { value });
            (sorted_ascends != want(distinct) || set_ascends != want(true))
                .then(|| format!("{arguments:?}: {sorted_ascends} {set_ascends}"))
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

/// Run the case for `SL-10`.
///
/// # Panics
///
/// Panics when the case's assertion fails.
#[allow(clippy::too_many_lines)]
pub fn run(id: &str) {
    assert_eq!(id, "SL-10", "the case of {MODULE}");
    let project = P::stdlib();
    project.check_ok();
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    let report: Json = serde_json::from_slice(
        &std::fs::read(
            project
                .build_dir(&build_id)
                .join("production/LexLeanStd/Order.eligibility.json")
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
        "the five members are the module's production roots, in order"
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
        let disagreements = evaluate(&lowered[member], &corpus(member, EVALUATOR_RUNS));
        assert!(
            disagreements.is_empty(),
            "{member}: the evaluator and the oracle disagree on {} of {EVALUATOR_RUNS} runs: {:?}",
            disagreements.len(),
            &disagreements[..disagreements.len().min(5)]
        );
    }
    // `isAscending` judges the sorts' own results.
    let failures = ascends(
        &lowered["sortBytes"],
        &lowered["sortedSet"],
        &lowered["isAscending"],
        &corpus("sortBytes", EVALUATOR_RUNS),
    );
    assert!(
        failures.is_empty(),
        "isAscending misjudges a sorted result on {} of {EVALUATOR_RUNS} runs: {:?}",
        failures.len(),
        &failures[..failures.len().min(5)]
    );
    // The rendered rust-std package of every member, compiled by the pinned
    // rustc, runs the larger corpus.
    let dir = tempfile::Builder::new()
        .prefix("lexlean-order-")
        .tempdir()
        .expect("tempdir");
    let mut crates = Vec::new();
    let mut expected: BTreeMap<String, Vec<Json>> = BTreeMap::new();
    for (member, _) in MEMBERS {
        let low = &lowered[member];
        let manifest = rust_packages::manifest(
            &format!("order_{}", snake(member)),
            &low.program,
            low.entry(),
            Profile::Std,
        );
        let package = target::package(&manifest.to_file_bytes().expect("canonical manifest"))
            .unwrap_or_else(|failure| panic!("{member}: {failure:?}"));
        let runs = corpus(member, RUSTC_RUNS);
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
        let printed: Vec<Json> = stdout
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
    // The gate is armed: a merge that takes the right run's head on equal
    // keys still orders every key, and is caught only because the sort is
    // held to stability.
    let planted = P::stdlib();
    let text = planted.read_raw("src/Order.lex.tex");
    let site = "(lessBytes (first rightHead) (first leftHead))";
    assert_eq!(text.matches(site).count(), 1, "the plant site exists once");
    planted.write(
        "src/Order.lex.tex",
        &text.replacen(
            site,
            "(not (lessBytes (first leftHead) (first rightHead)))",
            1,
        ),
    );
    let checked = support::checked_project(&planted);
    let sort_entries = lower_in(&checked, &planted, MODULE, "sortEntries");
    let disagreements = evaluate(&sort_entries, &corpus("sortEntries", EVALUATOR_RUNS));
    assert!(
        !disagreements.is_empty(),
        "an unstable merge in `mergeStep` is detected by the differential"
    );
    let sort_bytes = lower_in(&checked, &planted, MODULE, "sortBytes");
    assert!(
        evaluate(&sort_bytes, &corpus("sortBytes", EVALUATOR_RUNS)).is_empty(),
        "the planted merge still orders every key, so only stability catches it"
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
        // the `Order` members.
        let Some(member) = root.root.strip_prefix(&format!("{MODULE}.")) else {
            continue;
        };
        for output in &root.outputs {
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
                serde_json::to_value(&lowered[member].program).expect("a program is JSON"),
                "{member}: the exported package realizes the program the differential ran"
            );
            realized += 1;
        }
    }
    assert_eq!(realized, MEMBERS.len(), "one certified package per member");
}
