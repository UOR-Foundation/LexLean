//! Maps and sets in the standard library (SPEC.md §17.22, `SL-14`, #276): the
//! `LexLeanStd.Map` module against the `BTreeMap` and `BTreeSet` the compiler
//! (stage 0) keeps its tables and name sets in, under the calculus evaluator
//! and under the pinned rustc.
//!
//! A map is a list of entries whose keys ascend strictly in the byte order of
//! `Order` (§17.22, `SL-10`), a set a list of keys that does; both are what a
//! `BTreeMap<String, _>` and a `BTreeSet<String>` iterate. Every operation
//! takes such a list and returns one, so the oracle of each is the
//! corresponding method on the tree. The inputs the corpus draws are
//! ascending, the members' precondition, except `mapFromEntries`, whose input
//! is any list of entries and whose oracle is collecting them into a
//! `BTreeMap`, where the last of a repeated key stays.

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
const MODULE: &str = "LexLeanStd.Map";

/// The members in the order the module declares them, each with the effects
/// its declaration admits, which are exactly the effects its closure
/// realizes.
const MEMBERS: [(&str, &[&str]); 10] = [
    ("mapFromEntries", &["allocation", "overflow"]),
    ("mapInsert", &["allocation"]),
    ("mapGet", &["allocation"]),
    ("mapRemove", &["allocation"]),
    ("mapExtend", &["allocation"]),
    ("setContains", &["allocation"]),
    ("setRemove", &["allocation"]),
    ("setUnion", &["allocation"]),
    ("setIntersection", &["allocation"]),
    ("setDifference", &["allocation"]),
];

/// Argument lists per member under the evaluator.
const EVALUATOR_RUNS: usize = 400;
/// Argument lists per member under rustc.
const RUSTC_RUNS: usize = 2_000;
/// The evaluator's fuel; a member that exhausts it fails the case.
const FUEL: u64 = 50_000_000;
/// The most entries or keys a seeded run holds.
const MAX_ITEMS: usize = 16;

/// The bytes a key or a value is drawn from: few enough that keys repeat and
/// one is often a prefix of another, with both ends of the byte range.
const KEY_BYTES: &[u8] = &[0x00, 0x01, 0x61, 0x62, 0x7f, 0x80, 0xff];

/// The entries of a map, in the order a `BTreeMap` iterates them.
type Entries = Vec<(Vec<u8>, Vec<u8>)>;

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
    /// A key or a value of at most four bytes over `KEY_BYTES`.
    fn key(&mut self) -> Vec<u8> {
        (0..self.below(5))
            .map(|_| KEY_BYTES[self.below(KEY_BYTES.len())])
            .collect()
    }
    /// Any list of entries, in no order, keys repeating.
    fn entries(&mut self) -> Entries {
        (0..self.below(MAX_ITEMS + 1))
            .map(|_| {
                let key = self.key();
                (key, self.key())
            })
            .collect()
    }
    /// A map: strictly ascending keys.
    fn map(&mut self) -> Entries {
        pairs(map_of(&self.entries()))
    }
    /// A set: strictly ascending keys.
    fn set(&mut self) -> Vec<Vec<u8>> {
        let items: Vec<Vec<u8>> = (0..self.below(MAX_ITEMS + 1)).map(|_| self.key()).collect();
        keys_of(&set_of(&items))
    }
    /// A key that is one of `present` half the time when there is one.
    fn key_in(&mut self, present: &[Vec<u8>]) -> Vec<u8> {
        if !present.is_empty() && self.below(2) == 0 {
            present[self.below(present.len())].clone()
        } else {
            self.key()
        }
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
fn flag(value: bool) -> Value {
    Value::Bool { value }
}
fn option(value: Option<&[u8]>) -> Value {
    match value {
        Some(value) => Value::Some {
            value: Box::new(bytes(value)),
        },
        None => Value::None,
    }
}
fn json(value: &Value) -> Json {
    serde_json::to_value(value).expect("a value is JSON")
}

/// Entries collected into a `BTreeMap`: of a repeated key the last stays.
fn map_of(entries: &[(Vec<u8>, Vec<u8>)]) -> BTreeMap<Vec<u8>, Vec<u8>> {
    entries.iter().cloned().collect()
}
/// A map as the entries a `BTreeMap` iterates.
fn pairs(map: BTreeMap<Vec<u8>, Vec<u8>>) -> Entries {
    map.into_iter().collect()
}
fn set_of(items: &[Vec<u8>]) -> BTreeSet<Vec<u8>> {
    items.iter().cloned().collect()
}
/// A set as the keys a `BTreeSet` iterates.
fn keys_of(set: &BTreeSet<Vec<u8>>) -> Vec<Vec<u8>> {
    set.iter().cloned().collect()
}
fn keys_of_map(entries: &[(Vec<u8>, Vec<u8>)]) -> Vec<Vec<u8>> {
    entries.iter().map(|(key, _)| key.clone()).collect()
}

/// An entry of text, for the fixed vectors.
fn entry(key: &str, value: &str) -> (Vec<u8>, Vec<u8>) {
    (key.as_bytes().to_vec(), value.as_bytes().to_vec())
}
/// A set of text, for the fixed vectors.
fn words(items: &[&str]) -> Vec<Vec<u8>> {
    items.iter().map(|item| item.as_bytes().to_vec()).collect()
}

/// Lists of entries in no order with repeated keys, the edge cases of
/// `mapFromEntries`; their maps are the edge cases of the rest.
fn fixed_entries() -> Vec<Entries> {
    vec![
        vec![],
        vec![entry("", "")],
        vec![entry("a", "1"), entry("", "2"), entry("a", "3")],
        vec![
            entry("ab", "1"),
            entry("a", "2"),
            entry("b", "3"),
            entry("a", "4"),
        ],
        vec![
            (vec![0xff], vec![0x00]),
            (vec![0x00], vec![0xff]),
            (vec![0x80], vec![]),
            (vec![0x7f], vec![0x7f]),
            (vec![0xff], vec![0x01]),
        ],
        (0..=MAX_ITEMS)
            .rev()
            .map(|n| {
                let at = u8::try_from(n).expect("a byte");
                (vec![at % 5], vec![at])
            })
            .collect(),
    ]
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
    let fixed = fixed_entries();
    let edge_keys = [b"".to_vec(), b"a".to_vec(), vec![0xff, 0xff]];
    match member {
        "mapFromEntries" => {
            let case = |entries: &Entries| -> Run {
                (
                    vec![entry_list(entries)],
                    json(&entry_list(&pairs(map_of(entries)))),
                )
            };
            for entries in &fixed {
                out.push(case(entries));
            }
            while out.len() < runs {
                out.push(case(&seeded.entries()));
            }
        }
        "mapInsert" => {
            let case = |map: &Entries, key: &[u8], value: &[u8]| -> Run {
                let mut oracle = map_of(map);
                oracle.insert(key.to_vec(), value.to_vec());
                (
                    vec![entry_list(map), bytes(key), bytes(value)],
                    json(&entry_list(&pairs(oracle))),
                )
            };
            for entries in &fixed {
                let map = pairs(map_of(entries));
                for key in &edge_keys {
                    out.push(case(&map, key, b"v"));
                }
                if let Some((key, _)) = map.first() {
                    out.push(case(&map, key, b"replaced"));
                }
            }
            while out.len() < runs {
                let map = seeded.map();
                let key = seeded.key_in(&keys_of_map(&map));
                let value = seeded.key();
                out.push(case(&map, &key, &value));
            }
        }
        "mapGet" => {
            let case = |map: &Entries, key: &[u8]| -> Run {
                let oracle = map_of(map);
                (
                    vec![entry_list(map), bytes(key)],
                    json(&option(oracle.get(key).map(Vec::as_slice))),
                )
            };
            for entries in &fixed {
                let map = pairs(map_of(entries));
                for key in &edge_keys {
                    out.push(case(&map, key));
                }
                for (key, _) in &map {
                    out.push(case(&map, key));
                }
            }
            while out.len() < runs {
                let map = seeded.map();
                let key = seeded.key_in(&keys_of_map(&map));
                out.push(case(&map, &key));
            }
        }
        "mapRemove" => {
            let case = |map: &Entries, key: &[u8]| -> Run {
                let mut oracle = map_of(map);
                oracle.remove(key);
                (
                    vec![entry_list(map), bytes(key)],
                    json(&entry_list(&pairs(oracle))),
                )
            };
            for entries in &fixed {
                let map = pairs(map_of(entries));
                for key in &edge_keys {
                    out.push(case(&map, key));
                }
                for (key, _) in &map {
                    out.push(case(&map, key));
                }
            }
            while out.len() < runs {
                let map = seeded.map();
                let key = seeded.key_in(&keys_of_map(&map));
                out.push(case(&map, &key));
            }
        }
        "mapExtend" => {
            let case = |map: &Entries, other: &Entries| -> Run {
                let mut oracle = map_of(map);
                oracle.extend(other.iter().cloned());
                (
                    vec![entry_list(map), entry_list(other)],
                    json(&entry_list(&pairs(oracle))),
                )
            };
            for entries in &fixed {
                let map = pairs(map_of(entries));
                out.push(case(&map, &Vec::new()));
                out.push(case(&Vec::new(), &map));
                out.push(case(&map, &map));
                let shifted: Entries = map
                    .iter()
                    .map(|(key, value)| (key.clone(), [value.as_slice(), b"+"].concat()))
                    .collect();
                out.push(case(&map, &shifted));
            }
            while out.len() < runs {
                let map = seeded.map();
                let other = seeded.map();
                out.push(case(&map, &other));
            }
        }
        "setContains" | "setRemove" => {
            let case = |items: &[Vec<u8>], key: &[u8]| -> Run {
                let mut oracle = set_of(items);
                let expected = if member == "setContains" {
                    flag(oracle.contains(key))
                } else {
                    oracle.remove(key);
                    byte_list(&keys_of(&oracle))
                };
                (vec![byte_list(items), bytes(key)], json(&expected))
            };
            for entries in &fixed {
                let items = keys_of_map(&pairs(map_of(entries)));
                for key in &edge_keys {
                    out.push(case(&items, key));
                }
                for key in &items {
                    out.push(case(&items, key));
                }
            }
            while out.len() < runs {
                let items = seeded.set();
                let key = seeded.key_in(&items);
                out.push(case(&items, &key));
            }
        }
        "setUnion" | "setIntersection" | "setDifference" => {
            let case = |left: &[Vec<u8>], right: &[Vec<u8>]| -> Run {
                let (left_set, right_set) = (set_of(left), set_of(right));
                let result: BTreeSet<Vec<u8>> = match member {
                    "setUnion" => left_set.union(&right_set).cloned().collect(),
                    "setIntersection" => left_set.intersection(&right_set).cloned().collect(),
                    _ => left_set.difference(&right_set).cloned().collect(),
                };
                (
                    vec![byte_list(left), byte_list(right)],
                    json(&byte_list(&keys_of(&result))),
                )
            };
            for entries in &fixed {
                let items = keys_of_map(&pairs(map_of(entries)));
                out.push(case(&items, &[]));
                out.push(case(&[], &items));
                out.push(case(&items, &items));
                let odd: Vec<Vec<u8>> = items.iter().step_by(2).cloned().collect();
                out.push(case(&items, &odd));
                out.push(case(&odd, &items));
            }
            while out.len() < runs {
                let left = seeded.set();
                let right = seeded.set();
                out.push(case(&left, &right));
            }
        }
        other => panic!("`{other}` is not a member of {MODULE}"),
    }
    out
}

/// A defect planted in the source of `Map`, caught on a vector of its own.
struct Plant {
    label: &'static str,
    /// The member the vector is run on.
    member: &'static str,
    /// The text replaced, once in the module.
    site: &'static str,
    replacement: &'static str,
}

/// One defect per member. Each leaves the order of the keys intact where the
/// member has one, so that only the vector of its own catches it.
const PLANTS: [Plant; 10] = [
    Plant {
        label: "collecting keeps the first of a repeated key",
        member: "mapFromEntries",
        site: "then (cons entry rest) else",
        replacement: "then kept else",
    },
    Plant {
        label: "insertion keeps the old value on an equal key",
        member: "mapInsert",
        site: "(true, (cons fresh (second state)))",
        replacement: "(true, (cons entry (second state)))",
    },
    Plant {
        label: "the lookup finds only the last entry",
        member: "mapGet",
        site: "(second entry)) else found)",
        replacement: "(second entry)) else (ctor Option.none [Bytes]))",
    },
    Plant {
        label: "removal keeps the entry",
        member: "mapRemove",
        site: "then kept else (cons entry kept)",
        replacement: "then (cons entry kept) else (cons entry kept)",
    },
    Plant {
        label: "the extension lets the map win over the other",
        member: "mapExtend",
        site: ") entries other : (List (Bytes * Bytes)))",
        replacement: ") other entries : (List (Bytes * Bytes)))",
    },
    Plant {
        label: "membership asks of the last key only",
        member: "setContains",
        site: "(if found then true else (prim equal key item : Bool))",
        replacement: "(prim equal key item : Bool)",
    },
    Plant {
        label: "removal keeps the key",
        member: "setRemove",
        site: "(if (prim equal key item : Bool) then kept else (cons item kept))",
        replacement:
            "(if (prim equal key item : Bool) then (cons item kept) else (cons item kept))",
    },
    Plant {
        label: "the union appends without ordering",
        member: "setUnion",
        site: "(Order::insertSorted merged item)",
        replacement: "(cons item merged)",
    },
    Plant {
        label: "the intersection keeps what the other lacks",
        member: "setIntersection",
        site: "(if (setContains other item) then (cons item kept) else kept)",
        replacement: "(if (setContains other item) then kept else (cons item kept))",
    },
    Plant {
        label: "the difference keeps what the other holds",
        member: "setDifference",
        site: "(if (setContains other item) then kept else (cons item kept))",
        replacement: "(if (setContains other item) then (cons item kept) else kept)",
    },
];

/// The fixed vector of a plant, with the outcome written out by hand: the
/// arguments, and what the oracle returns on them.
#[allow(clippy::too_many_lines)]
fn vector(member: &str) -> Run {
    match member {
        // The first of `k` is `1`, the last is `3`.
        "mapFromEntries" => (
            vec![entry_list(&[
                entry("k", "1"),
                entry("j", "2"),
                entry("k", "3"),
            ])],
            json(&entry_list(&[entry("j", "2"), entry("k", "3")])),
        ),
        "mapInsert" => (
            vec![
                entry_list(&[entry("a", "1"), entry("b", "2")]),
                bytes(b"a"),
                bytes(b"9"),
            ],
            json(&entry_list(&[entry("a", "9"), entry("b", "2")])),
        ),
        "mapGet" => (
            vec![entry_list(&[entry("a", "1"), entry("b", "2")]), bytes(b"a")],
            json(&option(Some(b"1"))),
        ),
        "mapRemove" => (
            vec![entry_list(&[entry("a", "1"), entry("b", "2")]), bytes(b"a")],
            json(&entry_list(&[entry("b", "2")])),
        ),
        "mapExtend" => (
            vec![
                entry_list(&[entry("a", "1"), entry("b", "2")]),
                entry_list(&[entry("a", "9")]),
            ],
            json(&entry_list(&[entry("a", "9"), entry("b", "2")])),
        ),
        "setContains" => (
            vec![byte_list(&words(&["a", "b"])), bytes(b"a")],
            json(&flag(true)),
        ),
        "setRemove" => (
            vec![byte_list(&words(&["a", "b"])), bytes(b"a")],
            json(&byte_list(&words(&["b"]))),
        ),
        "setUnion" => (
            vec![byte_list(&words(&["a"])), byte_list(&words(&["b"]))],
            json(&byte_list(&words(&["a", "b"]))),
        ),
        "setIntersection" => (
            vec![
                byte_list(&words(&["a", "b"])),
                byte_list(&words(&["b", "c"])),
            ],
            json(&byte_list(&words(&["b"]))),
        ),
        "setDifference" => (
            vec![
                byte_list(&words(&["a", "b"])),
                byte_list(&words(&["b", "c"])),
            ],
            json(&byte_list(&words(&["a"]))),
        ),
        other => panic!("`{other}` has no fixed vector"),
    }
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

/// Run the case for `SL-14`.
///
/// # Panics
///
/// Panics when the case's assertion fails.
#[allow(clippy::too_many_lines)]
pub fn run(id: &str) {
    assert_eq!(id, "SL-14", "the case of {MODULE}");
    let project = P::stdlib();
    project.check_ok();
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    let report: Json = serde_json::from_slice(
        &std::fs::read(
            project
                .build_dir(&build_id)
                .join("production/LexLeanStd/Map.eligibility.json")
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
        "the ten members are the module's production roots, in order"
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
    // The rendered rust-std package of every member, compiled by the pinned
    // rustc, runs the larger corpus.
    let dir = tempfile::Builder::new()
        .prefix("lexlean-map-")
        .tempdir()
        .expect("tempdir");
    let mut crates = Vec::new();
    let mut expected: BTreeMap<String, Vec<Json>> = BTreeMap::new();
    for (member, _) in MEMBERS {
        let low = &lowered[member];
        let manifest = rust_packages::manifest(
            &format!("maps_{}", snake(member)),
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
    let mut binaries = Vec::new();
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
        binaries.push((name.clone(), binary));
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
    // The gate is armed: each plant is a defect in one member that the
    // member's fixed vector catches, on a module the committed one agrees
    // with on that vector.
    let mut missed = Vec::new();
    for plant in &PLANTS {
        let (arguments, expected) = vector(plant.member);
        assert_eq!(
            observe(&lowered[plant.member], &arguments),
            expected,
            "{}: the committed module agrees with the vector of {}",
            plant.label,
            plant.member
        );
        let planted = P::stdlib();
        let text = planted.read_raw("src/Map.lex.tex");
        assert_eq!(
            text.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/Map.lex.tex",
            &text.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let broken = lower_in(&checked, &planted, MODULE, plant.member);
        let observed = observe(&broken, &arguments);
        if observed == expected {
            missed.push(plant.label);
        } else {
            eprintln!(
                "SL-14: plant detected: {}: {observed} != {expected}",
                plant.label
            );
        }
    }
    assert!(
        missed.is_empty(),
        "planted defects the fixed vectors do not detect: {missed:?}"
    );
    // The measured stack bound of every package comes last, so that a bound the
    // record lacks hides none of the results above.
    for (name, binary) in &binaries {
        crate::stack_bounds::observe(&crate::stack_bounds::subject("stdlib", name), binary);
    }
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
        // the `Map` members.
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
