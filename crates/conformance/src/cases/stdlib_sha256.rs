//! SHA-256 in the standard library (SPEC.md §17.22, `SL-02`, #126 stage 1):
//! the `LexLeanStd.Sha256` module against three oracles that were written
//! independently of it, the FIPS 180-4 example digests, the NIST CAVP SHA-256
//! byte-oriented vectors (`tests/stdlib/sha256/`), and the `sha2` crate on a
//! seeded corpus, under the calculus evaluator and under the pinned rustc.
//!
//! An oracle is evidence, not authority: a disagreement fails the case and
//! is a defect of the module or of the pipeline, and nothing here adopts an
//! oracle's digest.

use std::collections::BTreeMap;

use lexlean::calculus::rust::package::Passing;
use lexlean::calculus::rust::{self, Profile};
use lexlean::calculus::{self as target, interp, Value};
use lexlean::production::lower::Lowered;
use serde_json::Value as Json;
use sha2::{Digest, Sha256};

use super::stdlib::{lower_in, Run};
use crate::production_export as harness;
use crate::rust_build::{cargo_in, workspace, Built};
use crate::rust_harness::{self, Calls};
use crate::rust_packages;
use crate::support::{self, P};

/// The module.
const MODULE: &str = "LexLeanStd.Sha256";

/// The members in the order the module declares them, each with the effects
/// its declaration admits, which are exactly the effects its closure
/// realizes: `Bytes` and the round-constant list allocate, the positions are
/// `Nat` arithmetic realized in 64 bits, and `iterate` is a library template
/// whose self call is a tail call, rendered as a loop (`RB-14`), so neither
/// closure recurses.
const MEMBERS: [(&str, &[&str]); 2] = [
    ("sha256", &["allocation", "overflow"]),
    ("sha256Hex", &["allocation", "overflow"]),
];

/// Seeded messages per member under the evaluator, beside every vector.
const EVALUATOR_RUNS: usize = 240;
/// Seeded messages per member under rustc, beside every vector.
const RUSTC_RUNS: usize = 600;
/// The evaluator's fuel: the longest CAVP vector (6400 bytes, 100 blocks)
/// finishes well within it, and a member that exhausts it fails the case.
const FUEL: u64 = 400_000_000;

/// The FIPS 180-4 examples (the one-block and two-block messages of its
/// appendix and the empty message), stated by the module as theorems too.
const FIPS: [&[u8]; 3] = [
    b"abc",
    b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
    b"",
];

/// The committed CAVP response files.
const CAVP: [&str; 2] = ["SHA256ShortMsg.rsp", "SHA256LongMsg.rsp"];

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
    fn message(&mut self, length: usize) -> Vec<u8> {
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

/// The `Digest` record of a 32-byte digest: the structure's one constructor
/// with its eight big-endian `UInt32` words.
fn digest(value: &[u8]) -> Value {
    Value::Adt {
        constructor: 0,
        fields: value
            .chunks(4)
            .map(|word| Value::U32 {
                value: u32::from_be_bytes(word.try_into().expect("four bytes")).to_string(),
            })
            .collect(),
    }
}

/// The lowercase hexadecimal of a digest, as the `Bytes` `sha256Hex` returns.
fn digest_hex(value: &[u8]) -> Value {
    bytes(hex(value).as_bytes())
}

/// A member's expected outcome on `message`, by the `sha2` crate.
fn oracle(member: &str, message: &[u8]) -> Json {
    let value = Sha256::digest(message);
    let value: &[u8] = value.as_slice();
    let expected = match member {
        "sha256" => digest(value),
        "sha256Hex" => digest_hex(value),
        other => panic!("`{other}` is not a member of {MODULE}"),
    };
    serde_json::to_value(expected).expect("a value is JSON")
}

/// A member's expected outcome on `message`, by a vector file's `MD`.
fn stated(member: &str, md: &[u8]) -> Json {
    let expected = match member {
        "sha256" => digest(md),
        "sha256Hex" => digest_hex(md),
        other => panic!("`{other}` is not a member of {MODULE}"),
    };
    serde_json::to_value(expected).expect("a value is JSON")
}

fn unhex(text: &str) -> Vec<u8> {
    assert!(text.len().is_multiple_of(2), "even hexadecimal: {text}");
    (0..text.len())
        .step_by(2)
        .map(|at| u8::from_str_radix(&text[at..at + 2], 16).expect("hexadecimal"))
        .collect()
}

/// The `(message, digest)` pairs of one CAVP response file: `Len` is the
/// message length in bits, `Msg` the message (`00` for the empty one), `MD`
/// the digest.
fn parse_rsp(text: &str) -> Vec<(Vec<u8>, Vec<u8>)> {
    let mut out = Vec::new();
    let mut length: Option<usize> = None;
    let mut message: Option<Vec<u8>> = None;
    for line in text.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("Len = ") {
            length = Some(rest.parse().expect("a bit length"));
        } else if let Some(rest) = line.strip_prefix("Msg = ") {
            let bits = length.expect("Len before Msg");
            assert_eq!(bits % 8, 0, "a byte-oriented vector");
            let mut parsed = unhex(rest);
            parsed.truncate(bits / 8);
            message = Some(parsed);
        } else if let Some(rest) = line.strip_prefix("MD = ") {
            let md = unhex(rest);
            assert_eq!(md.len(), 32, "a SHA-256 digest");
            out.push((message.take().expect("Msg before MD"), md));
            length = None;
        }
    }
    out
}

/// Every vector: the FIPS 180-4 examples, with their digests by `sha2`
/// (the module states the same digests as theorems), then the CAVP files
/// with the digests they state.
fn vectors() -> Vec<(Vec<u8>, Json, Json)> {
    let mut out: Vec<(Vec<u8>, Json, Json)> = FIPS
        .iter()
        .map(|message| {
            (
                message.to_vec(),
                oracle("sha256", message),
                oracle("sha256Hex", message),
            )
        })
        .collect();
    for file in CAVP {
        let path = support::repo_root().join("tests/stdlib/sha256").join(file);
        let text = std::fs::read_to_string(path.as_std_path())
            .unwrap_or_else(|error| panic!("{file}: {error}"));
        let parsed = parse_rsp(&text);
        assert!(parsed.len() >= 64, "{file}: {} vectors", parsed.len());
        for (message, md) in parsed {
            // The vector's digest is one oracle and `sha2` another; they
            // agree with each other before either judges the module.
            assert_eq!(
                Sha256::digest(&message).as_slice(),
                md.as_slice(),
                "{file}: the vector of {} bytes and `sha2` disagree",
                message.len()
            );
            out.push((message, stated("sha256", &md), stated("sha256Hex", &md)));
        }
    }
    out
}

/// The corpus of one member: every vector, then `runs` seeded messages whose
/// lengths cover every padding boundary (each length up to 200 bytes, so
/// 55, 56, 63, 64, 119, 120 and their neighbours, then random lengths up
/// to a few kilobytes), each with the digest `sha2` computes.
fn corpus(member: &str, runs: usize) -> Vec<Run> {
    let mut out: Vec<Run> = vectors()
        .into_iter()
        .map(|(message, words, text)| {
            (
                vec![bytes(&message)],
                if member == "sha256" { words } else { text },
            )
        })
        .collect();
    let mut seed = 0x9e37_79b9_7f4a_7c15_u64;
    for character in member.bytes() {
        seed = seed.rotate_left(8) ^ u64::from(character);
    }
    let mut seeded = Seeded(seed);
    let mut index = 0usize;
    while index < runs {
        let length = if index <= 200 {
            index
        } else {
            seeded.below(4_096)
        };
        let message = seeded.message(length);
        out.push((vec![bytes(&message)], oracle(member, &message)));
        index += 1;
    }
    out
}

/// The one-million `a` example of FIPS 180-4, run under rustc only.
fn million() -> Run {
    let message = vec![b'a'; 1_000_000];
    (vec![bytes(&message)], oracle("sha256Hex", &message))
}

/// Run the case for `SL-02`.
///
/// # Panics
///
/// Panics when the case's assertion fails.
#[allow(clippy::too_many_lines)]
pub(super) fn run(id: &str) {
    assert_eq!(id, "SL-02");
    let project = P::stdlib();
    project.check_ok();
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    let report: Json = serde_json::from_slice(
        &std::fs::read(
            project
                .build_dir(&build_id)
                .join("production/LexLeanStd/Sha256.eligibility.json")
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
        "the two members are the module's production roots, in order"
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
    // Every member lowers once; the evaluator runs the vectors and the
    // seeded corpus.
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
            "{member}: the evaluator and the oracles disagree on {} of {} runs: {:?}",
            disagreements.len(),
            runs.len(),
            &disagreements[..disagreements.len().min(3)]
        );
    }
    // The rendered rust-std package of every member, compiled by the pinned
    // rustc, runs the vectors, the larger corpus, and the million-`a`
    // example.
    let dir = tempfile::Builder::new()
        .prefix("lexlean-stdlib-sha256-")
        .tempdir()
        .expect("tempdir");
    let mut crates = Vec::new();
    let mut expected: BTreeMap<String, Vec<Json>> = BTreeMap::new();
    for (member, _) in MEMBERS {
        let low = &lowered[member];
        let manifest = rust_packages::manifest(
            &format!(
                "sha256_{}",
                if member == "sha256" { "words" } else { "hex" }
            ),
            &low.program,
            low.entry(),
            Profile::Std,
        );
        let package = target::package(&manifest.to_file_bytes().expect("canonical manifest"))
            .unwrap_or_else(|failure| panic!("{member}: {failure:?}"));
        let mut runs = corpus(member, RUSTC_RUNS);
        if member == "sha256Hex" {
            runs.push(million());
        }
        let arguments: Vec<Vec<Value>> = runs
            .iter()
            .map(|(arguments, _)| arguments.clone())
            .collect();
        let passing = vec![Passing::Own; 1];
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
        // PD-08: both roots recurse, so the harness that ran them records
        // (measuring) or demands (otherwise) their stack bound. The harness
        // crates are named for the member's output, the packages for the root.
        let package = if name.starts_with("sha256_words") {
            "stdlib/sha256"
        } else {
            "stdlib/sha256_hex"
        };
        crate::stack_bounds::observe(package, &binary);
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
            "{name}: the rendering and the oracles disagree on {} of {} runs: {:?}",
            disagreements.len(),
            outcomes.len(),
            &disagreements[..disagreements.len().min(3)]
        );
    }
    // The gate is armed: one bit of the round function flipped (the new
    // working variable `a` of every round, exclusive-or 1) and the first
    // FIPS vector disagrees.
    let planted = P::stdlib();
    let text = planted.read_raw("src/Sha256.lex.tex");
    let site = "a := (add t1 t2),";
    assert_eq!(text.matches(site).count(), 1, "the plant site exists once");
    planted.write(
        "src/Sha256.lex.tex",
        &text.replacen(
            site,
            "a := (prim bit_xor (add t1 t2) (1 : UInt32) : UInt32),",
            1,
        ),
    );
    let checked = support::checked_project(&planted);
    let hex_root = lower_in(&checked, &planted, MODULE, "sha256Hex");
    let first: Vec<Run> = corpus("sha256Hex", 0).into_iter().take(1).collect();
    let disagreements = evaluate(&hex_root, &first);
    assert_eq!(
        disagreements.len(),
        1,
        "a planted one-bit error in the round function is detected on `abc`"
    );
    // Under the pinned Lean, the production export certifies both members;
    // their packages are the committed ones and realize the programs the
    // differential ran.
    if !support::lean_backed("SL-02") {
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
    assert_eq!(realized, MEMBERS.len(), "one certified package per member");
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
