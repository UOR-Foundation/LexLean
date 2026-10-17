//! The standard library (SPEC.md §17.22): the `stdlib/` project, whose
//! members are certified production roots, and the differential of each
//! member against the function of Rust's standard library it corresponds
//! to, under the calculus evaluator and under the pinned rustc.

use std::collections::BTreeMap;

use lexlean::calculus::rust::package::Passing;
use lexlean::calculus::rust::{self, Profile};
use lexlean::calculus::{self as target, interp, Value};
use lexlean::production::lower::{linked_modules, lower_root, roots, Lowered};
use serde_json::Value as Json;

use crate::production_export as harness;
use crate::rust_build::{cargo_in, workspace, Built};
use crate::rust_harness::{self, Calls};
use crate::rust_packages;
use crate::support::{self, P};

/// The module the first member lives in.
const MODULE: &str = "LexLeanStd.Text";

/// The members in the order the module declares them, each with the effects
/// its declaration admits, which are exactly the effects its closure
/// realizes.
const MEMBERS: [(&str, &[&str]); 7] = [
    ("startsWith", &["allocation"]),
    ("endsWith", &["allocation"]),
    ("trim", &["allocation", "overflow", "recursion"]),
    ("splitOn", &["allocation", "overflow", "recursion"]),
    ("lines", &["allocation", "overflow", "recursion"]),
    ("joinWith", &["allocation", "recursion"]),
    ("find", &["allocation", "overflow", "recursion"]),
];

/// Argument lists per member under the evaluator.
const EVALUATOR_RUNS: usize = 600;
/// Argument lists per member under rustc.
const RUSTC_RUNS: usize = 3_000;
/// The evaluator's fuel; a member that exhausts it fails the case.
const FUEL: u64 = 50_000_000;

/// The bytes a byte-level corpus is drawn from: letters, every ASCII white
/// space byte with the vertical tab that is not one, punctuation, the
/// extremes, and the two bytes of `é`.
const BYTES: &[u8] = &[
    b'a', b'b', b'c', b' ', b'\t', b'\n', 0x0c, b'\r', 0x0b, b',', b'.', 0x00, 0xff, 0xc3, 0xa9,
];
/// The white space a trimmed corpus is padded with.
const PADDING: &[u8] = &[b' ', b'\t', b'\n', 0x0c, b'\r', 0x0b];
/// The pieces a UTF-8 corpus is drawn from, for the oracles over `str`.
const PIECES: &[&str] = &[
    "a", "b", "c", " ", "\n", "\r\n", "\r", "\u{e9}", "\u{65e5}", "\t", "ab", "",
];

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
    fn pick<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[self.below(items.len())]
    }
    fn chance(&mut self, percent: usize) -> bool {
        self.below(100) < percent
    }
    /// Up to `max` items, long on occasion.
    fn count(&mut self, max: usize) -> usize {
        if self.chance(5) {
            self.below(4 * max + 1)
        } else {
            self.below(max + 1)
        }
    }
    fn raw(&mut self, max: usize) -> Vec<u8> {
        (0..self.count(max)).map(|_| *self.pick(BYTES)).collect()
    }
    fn text(&mut self, max: usize) -> String {
        (0..self.count(max)).map(|_| *self.pick(PIECES)).collect()
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
fn bytes(value: &[u8]) -> Value {
    Value::Bytes { hex: hex(value) }
}
fn byte(value: u8) -> Value {
    Value::U8 {
        value: value.to_string(),
    }
}
fn boolean(value: bool) -> Value {
    Value::Bool { value }
}
fn nat(value: usize) -> Value {
    Value::Nat {
        value: value.to_string(),
    }
}
fn list(items: Vec<Value>) -> Value {
    Value::List { items }
}
fn byte_list<'a>(items: impl Iterator<Item = &'a [u8]>) -> Value {
    list(items.map(bytes).collect())
}
fn option(value: Option<Value>) -> Value {
    match value {
        Some(value) => Value::Some {
            value: Box::new(value),
        },
        None => Value::None,
    }
}

/// One run: the member's arguments and the oracle's observable outcome.
type Run = (Vec<Value>, Json);

fn record(out: &mut Vec<Run>, arguments: Vec<Value>, expected: &Value) {
    out.push((
        arguments,
        serde_json::to_value(expected).expect("a value is JSON"),
    ));
}

/// A prefix, suffix, or substring of `subject` at character boundaries.
fn part(seeded: &mut Seeded, subject: &str) -> String {
    let boundaries: Vec<usize> = subject
        .char_indices()
        .map(|(index, _)| index)
        .chain([subject.len()])
        .collect();
    let mut from = *seeded.pick(&boundaries);
    let mut to = *seeded.pick(&boundaries);
    if from > to {
        std::mem::swap(&mut from, &mut to);
    }
    subject[from..to].to_owned()
}

/// The corpus of one member: its edge cases, then seeded random inputs up
/// to `runs`, each with the outcome of the member's oracle in Rust's
/// standard library.
#[allow(clippy::too_many_lines)]
fn corpus(member: &str, runs: usize) -> Vec<Run> {
    let mut seed = 0x9e37_79b9_7f4a_7c15_u64;
    for character in member.bytes() {
        seed = seed.rotate_left(8) ^ u64::from(character);
    }
    let mut seeded = Seeded(seed);
    let mut out = Vec::new();
    match member {
        "startsWith" => {
            let fixed: [(&[u8], &[u8]); 7] = [
                (b"", b""),
                (b"", b"a"),
                (b"a", b""),
                (b"abc", b"ab"),
                (b"abc", b"abc"),
                (b"abc", b"abcd"),
                (b"abc", b"bc"),
            ];
            for (subject, prefix) in fixed {
                record(
                    &mut out,
                    vec![bytes(subject), bytes(prefix)],
                    &boolean(subject.starts_with(prefix)),
                );
            }
            while out.len() < runs {
                let subject = seeded.raw(16);
                let prefix = if seeded.chance(50) {
                    subject[..seeded.below(subject.len() + 1)].to_vec()
                } else {
                    seeded.raw(4)
                };
                record(
                    &mut out,
                    vec![bytes(&subject), bytes(&prefix)],
                    &boolean(subject.starts_with(&prefix)),
                );
            }
        }
        "endsWith" => {
            let fixed: [(&[u8], &[u8]); 7] = [
                (b"", b""),
                (b"", b"a"),
                (b"a", b""),
                (b"abc", b"bc"),
                (b"abc", b"abc"),
                (b"abc", b"zabc"),
                (b"abc", b"ab"),
            ];
            for (subject, suffix) in fixed {
                record(
                    &mut out,
                    vec![bytes(subject), bytes(suffix)],
                    &boolean(subject.ends_with(suffix)),
                );
            }
            while out.len() < runs {
                let subject = seeded.raw(16);
                let suffix = if seeded.chance(50) {
                    subject[seeded.below(subject.len() + 1)..].to_vec()
                } else {
                    seeded.raw(4)
                };
                record(
                    &mut out,
                    vec![bytes(&subject), bytes(&suffix)],
                    &boolean(subject.ends_with(&suffix)),
                );
            }
        }
        "trim" => {
            let fixed: [&[u8]; 8] = [
                b"",
                b"   ",
                b" a ",
                b"a",
                b"\x0b a \x0b",
                b"\t\n\x0c\r a\r\x0c\n\t",
                b" a b ",
                b"\x0b",
            ];
            for subject in fixed {
                record(&mut out, vec![bytes(subject)], &bytes(subject.trim_ascii()));
            }
            while out.len() < runs {
                let mut subject: Vec<u8> = (0..seeded.below(4))
                    .map(|_| *seeded.pick(PADDING))
                    .collect();
                subject.extend(seeded.raw(12));
                subject.extend((0..seeded.below(4)).map(|_| *seeded.pick(PADDING)));
                record(
                    &mut out,
                    vec![bytes(&subject)],
                    &bytes(subject.trim_ascii()),
                );
            }
        }
        "splitOn" => {
            let fixed: [(&[u8], u8); 7] = [
                (b"", b','),
                (b",", b','),
                (b"a,", b','),
                (b",a", b','),
                (b"a,b,,c", b','),
                (b"abc", b','),
                (b",,", b','),
            ];
            for (subject, delimiter) in fixed {
                record(
                    &mut out,
                    vec![bytes(subject), byte(delimiter)],
                    &byte_list(subject.split(|candidate| *candidate == delimiter)),
                );
            }
            while out.len() < runs {
                let subject = seeded.raw(16);
                let delimiter = if seeded.chance(80) {
                    *seeded.pick(BYTES)
                } else {
                    u8::try_from(seeded.below(256)).expect("a byte")
                };
                record(
                    &mut out,
                    vec![bytes(&subject), byte(delimiter)],
                    &byte_list(subject.split(|candidate| *candidate == delimiter)),
                );
            }
        }
        "lines" => {
            for subject in [
                "", "\n", "a", "a\n", "a\r\n", "a\r", "\r\n\r\n", "a\n\nb", "a\rb\n", "\r",
                "a\r\r\n",
            ] {
                record(
                    &mut out,
                    vec![bytes(subject.as_bytes())],
                    &byte_list(subject.lines().map(str::as_bytes)),
                );
            }
            while out.len() < runs {
                let subject = seeded.text(12);
                record(
                    &mut out,
                    vec![bytes(subject.as_bytes())],
                    &byte_list(subject.lines().map(str::as_bytes)),
                );
            }
        }
        "joinWith" => {
            let fixed: [(&[&[u8]], &[u8]); 6] = [
                (&[], b","),
                (&[b"a"], b","),
                (&[b"", b""], b","),
                (&[b"a", b"b"], b""),
                (&[b"a", b"b"], b", "),
                (&[b"", b"a", b""], b"\n"),
            ];
            for (pieces, separator) in fixed {
                let owned: Vec<Vec<u8>> = pieces.iter().map(|piece| piece.to_vec()).collect();
                record(
                    &mut out,
                    vec![byte_list(owned.iter().map(Vec::as_slice)), bytes(separator)],
                    &bytes(&owned.join(separator)),
                );
            }
            while out.len() < runs {
                let pieces: Vec<Vec<u8>> = (0..seeded.below(6)).map(|_| seeded.raw(5)).collect();
                let separator = seeded.raw(3);
                record(
                    &mut out,
                    vec![
                        byte_list(pieces.iter().map(Vec::as_slice)),
                        bytes(&separator),
                    ],
                    &bytes(&pieces.join(&separator[..])),
                );
            }
        }
        "find" => {
            for (haystack, needle) in [
                ("", ""),
                ("abc", ""),
                ("", "a"),
                ("abcabc", "ca"),
                ("ab", "abc"),
                ("aaa", "aa"),
                ("\u{e9}a", "a"),
                ("a\u{65e5}b", "\u{65e5}b"),
            ] {
                record(
                    &mut out,
                    vec![bytes(haystack.as_bytes()), bytes(needle.as_bytes())],
                    &option(haystack.find(needle).map(nat)),
                );
            }
            while out.len() < runs {
                let haystack = seeded.text(12);
                let needle = if seeded.chance(50) {
                    part(&mut seeded, &haystack)
                } else {
                    seeded.text(2)
                };
                record(
                    &mut out,
                    vec![bytes(haystack.as_bytes()), bytes(needle.as_bytes())],
                    &option(haystack.find(needle.as_str()).map(nat)),
                );
            }
        }
        other => panic!("`{other}` is not a member of {MODULE}"),
    }
    out
}

/// The lowered program of one member of the checked project.
fn lower(checked: &lexlean::link::CheckedProject, project: &P, member: &str) -> Lowered {
    let linked = linked_modules(checked);
    let roots = roots(checked).expect("the production roots");
    let root = format!("{MODULE}.{member}");
    let found = roots
        .iter()
        .find(|candidate| candidate.report.root == root)
        .unwrap_or_else(|| panic!("`{root}` is a production root"));
    lower_root(
        &linked,
        &found.module,
        &found.name,
        found.report,
        &support::limits(project),
    )
    .expect("the root lowers")
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

/// Run the case for one `SL` conformance ID.
///
/// # Panics
///
/// Panics when the case's assertion fails.
pub fn run(id: &str) {
    match id {
        // §17.22: the first member, bytes and text over `Bytes`.
        "SL-01" => {
            let project = P::stdlib();
            project.check_ok();
            let built = project.build_ok();
            let build_id = built.build_id.expect("built");
            let report: Json = serde_json::from_slice(
                &std::fs::read(
                    project
                        .build_dir(&build_id)
                        .join("production/LexLeanStd/Text.eligibility.json")
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
                "the seven members are the module's production roots, in order"
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
                .map(|(member, _)| (*member, lower(&checked, &project, member)))
                .collect();
            for (member, _) in MEMBERS {
                let disagreements = evaluate(&lowered[member], &corpus(member, EVALUATOR_RUNS));
                assert!(
                    disagreements.is_empty(),
                    "{member}: the evaluator and Rust's standard library disagree on {} of {EVALUATOR_RUNS} runs: {:?}",
                    disagreements.len(),
                    &disagreements[..disagreements.len().min(5)]
                );
            }
            // The rendered rust-std package of every member, compiled by the
            // pinned rustc, runs the larger corpus.
            let dir = tempfile::Builder::new()
                .prefix("lexlean-stdlib-")
                .tempdir()
                .expect("tempdir");
            let mut crates = Vec::new();
            let mut expected: BTreeMap<String, Vec<Json>> = BTreeMap::new();
            for (member, _) in MEMBERS {
                let low = &lowered[member];
                let manifest = rust_packages::manifest(
                    &format!("text_{}", snake(member)),
                    &low.program,
                    low.entry(),
                    Profile::Std,
                );
                let package =
                    target::package(&manifest.to_file_bytes().expect("canonical manifest"))
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
                let stdout = String::from_utf8(ran.stdout).expect("utf8");
                let printed: Vec<Json> = stdout
                    .lines()
                    .filter(|line| !line.starts_with("work "))
                    .map(|line| {
                        serde_json::from_str(line)
                            .unwrap_or_else(|error| panic!("{name}: {error}: {line}"))
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
                    "{name}: the rendering and Rust's standard library disagree on {} of {} runs: {:?}",
                    disagreements.len(),
                    outcomes.len(),
                    &disagreements[..disagreements.len().min(5)]
                );
            }
            // The gate is armed: a `trim` that takes the vertical tab for
            // white space and not the form feed disagrees with `trim_ascii`.
            let planted = P::stdlib();
            let text = planted.read_raw("src/Text.lex.tex");
            let site = "(prim equal byte (12 : UInt8) : Bool)";
            assert!(text.contains(site), "the plant site exists");
            planted.write(
                "src/Text.lex.tex",
                &text.replacen(site, "(prim equal byte (11 : UInt8) : Bool)", 1),
            );
            let checked = support::checked_project(&planted);
            let trim = lower(&checked, &planted, "trim");
            let disagreements = evaluate(&trim, &corpus("trim", EVALUATOR_RUNS));
            assert!(
                !disagreements.is_empty(),
                "a planted character class in `trim` is detected by the differential"
            );
            // Under the pinned Lean, the production export certifies every
            // member; its packages are the committed ones and realize the
            // programs the differential ran.
            if !support::lean_backed("SL-01") {
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
                let member = root
                    .root
                    .strip_prefix(&format!("{MODULE}."))
                    .unwrap_or_else(|| panic!("{} is a member", root.root));
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
        other => panic!("no stdlib case is wired for {other}"),
    }
}
