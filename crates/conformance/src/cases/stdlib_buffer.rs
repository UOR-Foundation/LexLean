//! The output buffer of the standard library (SPEC.md §17.22, `SL-05`, #92):
//! the `LexLeanStd.Buffer` module against the concatenation Rust's standard
//! library gives, under the calculus evaluator and under the pinned rustc,
//! with the refusal at the capacity and the work the rendered package counts
//! for a run of one-byte writes.
//!
//! The module exists because a chain of `append` copies both operands at each
//! call, so n one-byte writes cost n squared units of work; the last leg of
//! this case measures that the buffer's cost grows in proportion to n.

use std::collections::BTreeMap;

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
const MODULE: &str = "LexLeanStd.Buffer";

/// The roots in the order the module declares them, each with the effects its
/// declaration admits, which are exactly the effects its closure realizes.
const MEMBERS: [(&str, &[&str]); 3] = [
    ("writeAll", &["allocation", "overflow"]),
    ("writeEach", &["allocation", "overflow"]),
    ("writeNumbers", &["allocation", "overflow"]),
];

/// The refusal a write past the capacity ends in, as the module spells it.
const REFUSAL: &str = "output exceeds the capacity of the buffer";

/// Seeded argument lists per root under the evaluator, beside the edges.
const EVALUATOR_RUNS: usize = 120;
/// Seeded argument lists per root under rustc, beside the edges.
const RUSTC_RUNS: usize = 1_500;
/// The evaluator's fuel.
const FUEL: u64 = 2_000_000_000;
/// The sizes of the runs of one-byte writes whose work is compared. The
/// ratio of the work at the last to the work at the first must stay below
/// twice the ratio of the sizes; a chain of `append` gives its square.
const LINEAR: [usize; 3] = [4_096, 16_384, 65_536];

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
    fn bytes(&mut self, length: usize) -> Vec<u8> {
        (0..length)
            .map(|_| u8::try_from(self.below(256)).expect("a byte"))
            .collect()
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
fn bytes(value: &[u8]) -> Value {
    Value::Bytes { hex: hex(value) }
}
fn nat(value: u64) -> Value {
    Value::Nat {
        value: value.to_string(),
    }
}

/// What the oracle makes of a write of `total` bytes into `capacity`: the
/// bytes, or the refusal.
fn outcome(joined: &[u8], capacity: u64) -> Run {
    let expected = if u64::try_from(joined.len()).expect("a length") > capacity {
        Value::Error {
            value: Box::new(bytes(REFUSAL.as_bytes())),
        }
    } else {
        Value::Ok {
            value: Box::new(bytes(joined)),
        }
    };
    (
        Vec::new(),
        serde_json::to_value(expected).expect("a value is JSON"),
    )
}

/// The capacities worth trying for `length` bytes: exactly enough, one short,
/// one to spare, none, and a seeded one.
fn capacities(seeded: &mut Seeded, length: usize) -> Vec<u64> {
    let length = u64::try_from(length).expect("a length");
    let mut out = vec![length, length + 1, 0, length / 2];
    if length > 0 {
        out.push(length - 1);
    }
    out.push(
        u64::try_from(seeded.below(2 * usize::try_from(length).expect("a length") + 2))
            .expect("a capacity"),
    );
    out
}

/// The size at which the module seals a chunk (chosen by measurement on
/// the largest library, VERIFICATION.md `SL-05`).
const CHUNK: usize = 256;

/// The size of the largest committed Rust library of a package, in bytes.
const LARGEST: usize = 256_713;

/// The sizes a piece takes: the chunk boundary at 256 on both sides, and the
/// small sizes in between.
const SIZES: [usize; 13] = [
    0, 1, 2, 7, 255, 256, 257, 1_023, 1_024, 1_025, 2_048, 3_000, 5_000,
];

fn write_all_runs(runs: usize) -> Vec<Run> {
    let mut seeded = Seeded(0x0B0F_0001);
    let mut out = Vec::new();
    let add = |capacity: u64, pieces: Vec<Vec<u8>>, out: &mut Vec<Run>| {
        let joined: Vec<u8> = pieces.concat();
        let (_, expected) = outcome(&joined, capacity);
        out.push((
            vec![
                nat(capacity),
                Value::List {
                    items: pieces.iter().map(|piece| bytes(piece)).collect(),
                },
            ],
            expected,
        ));
    };
    for pieces in [
        vec![],
        vec![vec![]],
        vec![b"a".to_vec()],
        vec![b"ab".to_vec(), b"c".to_vec()],
        vec![vec![b'x'; 255], b"y".to_vec()],
        vec![vec![b'x'; 256]],
        vec![vec![b'x'; 255], vec![b'y'; 2], vec![b'z'; 256]],
        vec![vec![b'q'; 4_000], vec![], vec![b'r'; 100]],
    ] {
        let length: usize = pieces.iter().map(Vec::len).sum();
        for capacity in capacities(&mut seeded, length) {
            add(capacity, pieces.clone(), &mut out);
        }
    }
    while out.len() < runs {
        let count = seeded.below(12);
        let pieces: Vec<Vec<u8>> = (0..count)
            .map(|_| {
                let size = SIZES[seeded.below(SIZES.len())];
                seeded.bytes(size)
            })
            .collect();
        let length: usize = pieces.iter().map(Vec::len).sum();
        let capacities = capacities(&mut seeded, length);
        let capacity = capacities[seeded.below(capacities.len())];
        add(capacity, pieces, &mut out);
    }
    out
}

fn write_each_runs(runs: usize) -> Vec<Run> {
    let mut seeded = Seeded(0x0B0F_0002);
    let mut out = Vec::new();
    let add = |capacity: u64, data: Vec<u8>, out: &mut Vec<Run>| {
        let (_, expected) = outcome(&data, capacity);
        out.push((vec![nat(capacity), bytes(&data)], expected));
    };
    for data in [
        vec![],
        vec![0],
        vec![255],
        vec![0, 1, 2, 255, 254],
        (0..=255u8).collect::<Vec<u8>>(),
        vec![b'x'; 255],
        vec![b'x'; 256],
        vec![b'x'; 257],
        vec![b'x'; 2_048],
    ] {
        for capacity in capacities(&mut seeded, data.len()) {
            add(capacity, data.clone(), &mut out);
        }
    }
    while out.len() < runs {
        let size = SIZES[seeded.below(SIZES.len())];
        let data = seeded.bytes(size);
        let capacities = capacities(&mut seeded, data.len());
        let capacity = capacities[seeded.below(capacities.len())];
        add(capacity, data, &mut out);
    }
    out
}

fn write_numbers_runs(runs: usize) -> Vec<Run> {
    let mut seeded = Seeded(0x0B0F_0003);
    let mut out = Vec::new();
    let add = |capacity: u64, numbers: Vec<u64>, out: &mut Vec<Run>| {
        let joined: Vec<u8> = numbers
            .iter()
            .flat_map(|number| format!("{number},").into_bytes())
            .collect();
        let (_, expected) = outcome(&joined, capacity);
        out.push((
            vec![
                nat(capacity),
                Value::List {
                    items: numbers.iter().map(|number| nat(*number)).collect(),
                },
            ],
            expected,
        ));
    };
    let mut edges: Vec<u64> = vec![0, 1, 9, 10, 99, 100, u64::MAX, u64::MAX - 1, 1 << 63];
    edges.extend((0..20).map(|power| 10u64.pow(power)));
    edges.extend((1..20).map(|power| 10u64.pow(power) - 1));
    for numbers in [
        vec![],
        vec![0],
        vec![u64::MAX],
        edges.clone(),
        edges.iter().rev().copied().collect(),
    ] {
        let length: usize = numbers
            .iter()
            .map(|number| number.to_string().len() + 1)
            .sum();
        for capacity in capacities(&mut seeded, length) {
            add(capacity, numbers.clone(), &mut out);
        }
    }
    while out.len() < runs {
        let count = seeded.below(40);
        let numbers: Vec<u64> = (0..count)
            .map(|_| {
                let digits = 1 + seeded.below(20);
                let mut value: u64 = 0;
                for _ in 0..digits {
                    value = value
                        .wrapping_mul(10)
                        .wrapping_add(u64::try_from(seeded.below(10)).expect("a digit"));
                }
                value
            })
            .collect();
        let length: usize = numbers
            .iter()
            .map(|number| number.to_string().len() + 1)
            .sum();
        let capacities = capacities(&mut seeded, length);
        let capacity = capacities[seeded.below(capacities.len())];
        add(capacity, numbers, &mut out);
    }
    out
}

fn corpus(member: &str, runs: usize) -> Vec<Run> {
    match member {
        "writeAll" => write_all_runs(runs),
        "writeEach" => write_each_runs(runs),
        "writeNumbers" => write_numbers_runs(runs),
        other => panic!("no corpus for {other}"),
    }
}

/// Every run of `corpus` on which the evaluator and the oracle disagree.
fn evaluate(lowered: &Lowered, corpus: &[Run]) -> Vec<String> {
    corpus
        .iter()
        .enumerate()
        .filter_map(|(run, (arguments, expected))| {
            let outcome = interp::run(&lowered.program, FUEL, lowered.entry(), arguments);
            let observed = rust::observable(&outcome)
                .unwrap_or_else(|| panic!("run {run} on {arguments:?}: {outcome:?}"));
            (&observed != expected).then(|| format!("run {run}: {observed} != {expected}"))
        })
        .collect()
}

/// The package of a root and a harness over `runs`.
fn built(name: &str, low: &Lowered, runs: &[Run]) -> Built {
    let manifest = rust_packages::manifest(name, &low.program, low.entry(), Profile::Std);
    let package = target::package(&manifest.to_file_bytes().expect("canonical manifest"))
        .unwrap_or_else(|failure| panic!("{name}: {failure:?}"));
    let arguments: Vec<Vec<Value>> = runs
        .iter()
        .map(|(arguments, _)| arguments.clone())
        .collect();
    let passing = vec![Passing::Own; low.program.functions[low.entry() as usize].types.len()];
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
    Built {
        name: manifest.name,
        files: package.files,
        harness: harness_source,
    }
}

/// The lines a harness printed that are outcomes, and the work it counted.
fn ran(dir: &std::path::Path, name: &str) -> (Vec<Json>, u64, std::time::Duration) {
    let started = std::time::Instant::now();
    let binary = dir
        .join("target/debug")
        .join(format!("run_{name}{}", std::env::consts::EXE_SUFFIX));
    let output = std::process::Command::new(&binary)
        .output()
        .unwrap_or_else(|error| {
            panic!(
                "{name}: the differential runs: {error} ({}): {:?}",
                binary.display(),
                std::fs::read_dir(binary.parent().expect("dir")).map(|entries| entries
                    .filter_map(Result::ok)
                    .map(|entry| entry.file_name())
                    .collect::<Vec<_>>())
            )
        });
    assert!(
        output.status.success(),
        "{name}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    let work = stdout
        .lines()
        .find_map(|line| line.strip_prefix("work "))
        .map(|count| count.parse::<u64>().expect("a work count"))
        .expect("the work line");
    let printed = stdout
        .lines()
        .filter(|line| !line.starts_with("work "))
        .map(|line| {
            serde_json::from_str(line).unwrap_or_else(|error| panic!("{name}: {error}: {line}"))
        })
        .collect();
    (printed, work, started.elapsed())
}

/// Whether the work grew in proportion to the size, within a factor of two:
/// the work at the largest size, over its size, against the work at the
/// smallest over its.
fn proportional(works: &BTreeMap<usize, u64>) -> bool {
    let (size_first, work_first) = works.iter().next().expect("a first run");
    let (size_last, work_last) = works.iter().next_back().expect("a last run");
    u128::from(*work_last) * u128::try_from(*size_first).expect("a size")
        <= 2 * u128::from(*work_first) * u128::try_from(*size_last).expect("a size")
}

/// Run the case for `SL-05`.
///
/// # Panics
///
/// Panics when the case's assertion fails.
#[allow(clippy::too_many_lines)]
pub(super) fn run(id: &str) {
    assert_eq!(id, "SL-05");
    let project = P::stdlib();
    project.check_ok();
    let built_project = project.build_ok();
    let build_id = built_project.build_id.expect("built");
    let report: Json = serde_json::from_slice(
        &std::fs::read(
            project
                .build_dir(&build_id)
                .join("production/LexLeanStd/Buffer.eligibility.json")
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
        "the three roots are the module's production roots, in order"
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
    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, Lowered> = MEMBERS
        .iter()
        .map(|(member, _)| (*member, lower_in(&checked, &project, MODULE, member)))
        .collect();
    for (member, _) in MEMBERS {
        let runs = corpus(member, EVALUATOR_RUNS);
        let disagreements = evaluate(&lowered[member], &runs);
        assert!(
            disagreements.is_empty(),
            "{member}: the evaluator and Rust's concatenation disagree on {} of {} runs: {:?}",
            disagreements.len(),
            runs.len(),
            &disagreements[..disagreements.len().min(3)]
        );
    }
    // The rendered packages under the pinned rustc: the corpus of every root,
    // and three runs of one-byte writes whose work is compared.
    let dir = tempfile::Builder::new()
        .prefix("lexlean-stdlib-buffer-")
        .tempdir()
        .expect("tempdir");
    let mut crates = Vec::new();
    let mut expected: BTreeMap<String, Vec<Json>> = BTreeMap::new();
    for (member, _) in MEMBERS {
        let runs = corpus(member, RUSTC_RUNS);
        let name = format!("buffer_{}", member.to_lowercase());
        let package = built(&name, &lowered[member], &runs);
        expected.insert(
            package.name.clone(),
            runs.into_iter().map(|(_, expected)| expected).collect(),
        );
        crates.push(package);
    }
    for size in LINEAR {
        let data = vec![b'x'; size];
        let runs = vec![(
            vec![nat(u64::try_from(size).expect("a size")), bytes(&data)],
            outcome(&data, u64::try_from(size).expect("a size")).1,
        )];
        let name = format!("buffer_linear_{size}");
        let package = built(&name, &lowered["writeEach"], &runs);
        expected.insert(
            package.name.clone(),
            runs.into_iter().map(|(_, expected)| expected).collect(),
        );
        crates.push(package);
    }
    // The largest committed library, in the writes of a printer (pieces of a
    // dozen bytes) and in one-byte writes.
    let data: Vec<u8> = (0..LARGEST)
        .map(|at| b'a' + u8::try_from(at % 26).expect("a letter"))
        .collect();
    let pieces: Vec<Vec<u8>> = data.chunks(12).map(<[u8]>::to_vec).collect();
    let capacity = u64::try_from(LARGEST).expect("a size");
    let big_all = vec![(
        vec![
            nat(capacity),
            Value::List {
                items: pieces.iter().map(|piece| bytes(piece)).collect(),
            },
        ],
        outcome(&data, capacity).1,
    )];
    let package = built("buffer_big_all", &lowered["writeAll"], &big_all);
    expected.insert(package.name.clone(), vec![big_all[0].1.clone()]);
    crates.push(package);
    let big_each = vec![(
        vec![nat(capacity), bytes(&data)],
        outcome(&data, capacity).1,
    )];
    let package = built("buffer_big_each", &lowered["writeEach"], &big_each);
    expected.insert(package.name.clone(), vec![big_each[0].1.clone()]);
    crates.push(package);
    workspace(dir.path(), &crates);
    let build = cargo_in(
        dir.path(),
        &["build", "--offline", "--workspace", "--quiet"],
    );
    assert!(
        build.status.success(),
        "a root's package does not build:\n{}",
        String::from_utf8_lossy(&build.stderr)
    );
    let mut works: BTreeMap<usize, u64> = BTreeMap::new();
    for (name, outcomes) in &expected {
        let (printed, work, elapsed) = ran(dir.path(), name);
        if name.starts_with("buffer_big_") {
            eprintln!("SL-05: {name}: {work} ticks, {elapsed:?} for {LARGEST} bytes");
        }
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
            "{name}: the rendering and Rust's concatenation disagree on {} of {} runs: {:?}",
            disagreements.len(),
            outcomes.len(),
            &disagreements[..disagreements.len().min(3)]
        );
        if let Some(size) = name
            .strip_prefix("buffer_linear_")
            .and_then(|size| size.strip_suffix("_std"))
            .and_then(|size| size.parse::<usize>().ok())
        {
            works.insert(size, work);
        }
    }
    let first = LINEAR[0];
    eprintln!("SL-05: the work of one-byte writes by number: {works:?}");
    assert!(
        proportional(&works),
        "the work of one-byte writes is not in proportion to their number: {works:?}"
    );
    // The gate is armed. A boundary that refuses at the capacity itself is a
    // disagreement with Rust's concatenation on the corpus; a buffer that
    // never seals is a chain of appends, which writes the same bytes and is
    // caught by the work of its one-byte writes under rustc.
    let capacity_site = "(if (grown <=? (project buffer capacity))";
    let planted = P::stdlib();
    let text = planted.read_raw("src/Buffer.lex.tex");
    assert_eq!(
        text.matches(capacity_site).count(),
        1,
        "the plant site exists once"
    );
    planted.write(
        "src/Buffer.lex.tex",
        &text.replacen(capacity_site, "(if (grown <? (project buffer capacity))", 1),
    );
    let checked = support::checked_project(&planted);
    let each = lower_in(&checked, &planted, MODULE, "writeEach");
    let disagreements = evaluate(&each, &corpus("writeEach", EVALUATOR_RUNS));
    assert!(
        !disagreements.is_empty(),
        "a boundary that refuses at the capacity is detected by the corpus"
    );
    eprintln!("SL-05: plant detected: {}", disagreements[0]);
    let never = P::stdlib();
    let text = never.read_raw("src/Buffer.lex.tex");
    let seal_site = format!("(if ({CHUNK} <=? (prim length joined : Nat))");
    let seal_site = seal_site.as_str();
    assert_eq!(
        text.matches(seal_site).count(),
        1,
        "the plant site exists once"
    );
    never.write(
        "src/Buffer.lex.tex",
        &text.replacen(seal_site, "(if (256 <=? 0)", 1),
    );
    let checked = support::checked_project(&never);
    let each = lower_in(&checked, &never, MODULE, "writeEach");
    let dir = tempfile::Builder::new()
        .prefix("lexlean-stdlib-buffer-planted-")
        .tempdir()
        .expect("tempdir");
    let mut crates = Vec::new();
    let mut names = BTreeMap::new();
    for size in [first, LINEAR[1]] {
        let data = vec![b'x'; size];
        let runs = vec![(
            vec![nat(u64::try_from(size).expect("a size")), bytes(&data)],
            outcome(&data, u64::try_from(size).expect("a size")).1,
        )];
        let package = built(&format!("buffer_never_{size}"), &each, &runs);
        names.insert(package.name.clone(), size);
        crates.push(package);
    }
    workspace(dir.path(), &crates);
    let build = cargo_in(
        dir.path(),
        &["build", "--offline", "--workspace", "--quiet"],
    );
    assert!(
        build.status.success(),
        "the planted package does not build:\n{}",
        String::from_utf8_lossy(&build.stderr)
    );
    let planted_works: BTreeMap<usize, u64> = names
        .iter()
        .map(|(name, size)| (*size, ran(dir.path(), name).1))
        .collect();
    eprintln!("SL-05: the work of a buffer that never seals: {planted_works:?}");
    assert!(
        !proportional(&planted_works),
        "a buffer that never seals is detected by the work of its one-byte writes"
    );
    // Under the pinned Lean, the production export certifies the roots; their
    // packages are the committed ones and realize the programs the
    // differential ran.
    if !support::lean_backed("SL-05") {
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
    assert_eq!(realized, MEMBERS.len(), "one certified package per root");
}
