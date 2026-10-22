//! Hexadecimal and decimal in the standard library (SPEC.md §17.22, `SL-08`,
//! #276): the `LexLeanStd.Numeral` module against the functions of the
//! compiler (stage 0) it replaces, under the calculus evaluator and under the
//! pinned rustc.
//!
//! The oracles are the compiler's own readings: `canonical_digits` and
//! `canonical_hex` of `surface::semantic`, a natural number as the `u64` of
//! the `rust-std` target, and the lowercase digits of `Sha256Digest::to_hex`.
//! An overflow is an outcome like any other: a canonical decimal past the
//! target width is `{"kind":"overflow"}` from the member and from `u64`
//! parsing alike.

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
const MODULE: &str = "LexLeanStd.Numeral";

/// The members in the order the module declares them, each with the effects
/// its declaration admits, which are exactly the effects its closure
/// realizes.
const MEMBERS: [(&str, &[&str]); 5] = [
    ("decimal", &["allocation"]),
    ("hexNat", &["allocation"]),
    ("parseDecimal", &["allocation", "overflow"]),
    ("hex", &["allocation", "overflow"]),
    ("parseHex", &["allocation", "overflow"]),
];

/// Argument lists per member under the evaluator.
const EVALUATOR_RUNS: usize = 600;
/// Argument lists per member under rustc.
const RUSTC_RUNS: usize = 3_000;
/// The evaluator's fuel; a member that exhausts it fails the case.
const FUEL: u64 = 50_000_000;

/// The bytes a decimal corpus is drawn from: the digits, the signs and the
/// white space `u64` parsing or a lenient reader might take, a letter, and
/// a byte past ASCII.
const DECIMAL_BYTES: &[u8] = b"0123456789+- x\xff";
/// The bytes a hexadecimal corpus is drawn from: the lowercase digits, their
/// uppercase forms, the letters just past them, and the bytes beside each
/// range.
const HEX_BYTES: &[u8] = b"0123456789abcdefABCDEFgG/:`@ ";

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
    /// A natural number of the target width: small, near a power of the
    /// radix, or anywhere below `u64::MAX`.
    fn number(&mut self) -> u64 {
        match self.below(4) {
            0 => self.next() % 1000,
            1 => {
                let power = 10u64.pow(u32::try_from(self.below(20)).expect("an exponent"));
                power.wrapping_add(self.next() % 3).wrapping_sub(1)
            }
            2 => (1u64 << self.below(64))
                .wrapping_add(self.next() % 3)
                .wrapping_sub(1),
            _ => self.next() >> self.below(64),
        }
    }
    fn raw(&mut self, alphabet: &[u8], max: usize) -> Vec<u8> {
        (0..self.below(max + 1))
            .map(|_| *self.pick(alphabet))
            .collect()
    }
    fn bytes(&mut self, max: usize) -> Vec<u8> {
        (0..self.below(max + 1))
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
fn option(value: Option<Value>) -> Value {
    match value {
        Some(value) => Value::Some {
            value: Box::new(value),
        },
        None => Value::None,
    }
}
fn json(value: &Value) -> Json {
    serde_json::to_value(value).expect("a value is JSON")
}

/// Decimal digits with no leading zero: `canonical_digits` of
/// `surface::semantic`, over bytes.
fn canonical_digits(text: &[u8]) -> bool {
    !text.is_empty() && text.iter().all(u8::is_ascii_digit) && (text.len() == 1 || text[0] != b'0')
}

/// An even number of lowercase hexadecimal digits: `canonical_hex` of
/// `surface::semantic`, over bytes.
fn canonical_hex(text: &[u8]) -> bool {
    text.len().is_multiple_of(2)
        && text
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
}

/// `parseDecimal`'s oracle: none unless canonical, then the `u64`, whose
/// only failure on canonical digits is an overflow.
fn read_decimal(text: &[u8]) -> Json {
    if !canonical_digits(text) {
        return json(&Value::None);
    }
    let digits = std::str::from_utf8(text).expect("ASCII digits");
    match digits.parse::<u64>() {
        Ok(value) => json(&option(Some(nat(value)))),
        Err(error) => {
            assert_eq!(
                *error.kind(),
                std::num::IntErrorKind::PosOverflow,
                "canonical digits fail to parse only by overflow"
            );
            serde_json::json!({"kind": "overflow"})
        }
    }
}

/// `parseHex`'s oracle: none unless canonical, then the bytes.
fn read_hex(text: &[u8]) -> Json {
    if !canonical_hex(text) {
        return json(&Value::None);
    }
    let decoded: Vec<u8> = text
        .chunks_exact(2)
        .map(|pair| {
            u8::from_str_radix(std::str::from_utf8(pair).expect("ASCII"), 16).expect("hex digits")
        })
        .collect();
    json(&option(Some(bytes(&decoded))))
}

/// A canonical decimal or hexadecimal text changed in one of the ways a
/// reader must refuse or must still take.
fn mutate(seeded: &mut Seeded, mut text: Vec<u8>, alphabet: &[u8]) -> Vec<u8> {
    match seeded.below(5) {
        0 => text.insert(0, b'0'),
        1 => {
            let at = seeded.below(text.len() + 1);
            text.insert(at, *seeded.pick(alphabet));
        }
        2 => {
            text.pop();
        }
        3 => {
            if let Some(at) = (!text.is_empty()).then(|| seeded.below(text.len())) {
                text[at] = text[at].to_ascii_uppercase();
            }
        }
        _ => text.extend(seeded.raw(b"0123456789", 6)),
    }
    text
}

/// The corpus of one member: its edge cases, then seeded inputs up to
/// `runs`, each with the outcome of the member's oracle.
fn corpus(member: &str, runs: usize) -> Vec<Run> {
    let mut seed = 0x9e37_79b9_7f4a_7c15_u64;
    for character in member.bytes() {
        seed = seed.rotate_left(8) ^ u64::from(character);
    }
    let mut seeded = Seeded(seed);
    let mut out: Vec<Run> = Vec::new();
    match member {
        "decimal" | "hexNat" => {
            let print = |value: u64| -> Json {
                let text = if member == "decimal" {
                    value.to_string()
                } else {
                    format!("{value:x}")
                };
                json(&bytes(text.as_bytes()))
            };
            let fixed = [
                0,
                1,
                9,
                10,
                15,
                16,
                99,
                100,
                255,
                256,
                12_345,
                9_999_999_999_999_999_999,
                10_000_000_000_000_000_000,
                u64::MAX - 1,
                u64::MAX,
            ];
            for value in fixed {
                out.push((vec![nat(value)], print(value)));
            }
            while out.len() < runs {
                let value = seeded.number();
                out.push((vec![nat(value)], print(value)));
            }
        }
        "parseDecimal" => {
            let fixed: [&[u8]; 18] = [
                b"",
                b"0",
                b"00",
                b"01",
                b"1",
                b"10",
                b"123",
                b"-1",
                b"+1",
                b" 1",
                b"1 ",
                b"1a",
                b"0x10",
                b"\xd9\xa3",
                b"18446744073709551615",
                b"18446744073709551616",
                b"99999999999999999999",
                b"123456789012345678901234567890",
            ];
            for text in fixed {
                out.push((vec![bytes(text)], read_decimal(text)));
            }
            while out.len() < runs {
                let text = match seeded.below(3) {
                    0 => seeded.number().to_string().into_bytes(),
                    1 => {
                        let canonical = seeded.number().to_string().into_bytes();
                        mutate(&mut seeded, canonical, DECIMAL_BYTES)
                    }
                    _ => seeded.raw(DECIMAL_BYTES, 8),
                };
                out.push((vec![bytes(&text)], read_decimal(&text)));
            }
        }
        "hex" => {
            let fixed: [&[u8]; 5] = [
                b"",
                b"\x00",
                b"\xff",
                b"\x0f\xf0",
                b"\x01\x23\x45\x67\x89\xab\xcd\xef",
            ];
            for value in fixed {
                out.push((vec![bytes(value)], json(&bytes(hex(value).as_bytes()))));
            }
            while out.len() < runs {
                let value = seeded.bytes(40);
                out.push((vec![bytes(&value)], json(&bytes(hex(&value).as_bytes()))));
            }
        }
        "parseHex" => {
            let fixed: [&[u8]; 12] = [
                b"",
                b"0",
                b"00",
                b"ff",
                b"FF",
                b"aB",
                b"0g",
                b"abc",
                b"  ",
                b"0123456789abcdef",
                b"/0:9`a@f",
                b"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            ];
            for text in fixed {
                out.push((vec![bytes(text)], read_hex(text)));
            }
            while out.len() < runs {
                let text = match seeded.below(3) {
                    0 => hex(&seeded.bytes(24)).into_bytes(),
                    1 => {
                        let canonical = hex(&seeded.bytes(24)).into_bytes();
                        mutate(&mut seeded, canonical, HEX_BYTES)
                    }
                    _ => seeded.raw(HEX_BYTES, 8),
                };
                out.push((vec![bytes(&text)], read_hex(&text)));
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

/// Every printed value of `printer`'s corpus that `reader` does not read
/// back to the printer's argument: the round trips `parseDecimal (decimal n)
/// = some n` and `parseHex (hex b) = some b`.
fn round_trips(printer: &Lowered, reader: &Lowered, corpus: &[Run]) -> Vec<String> {
    corpus
        .iter()
        .filter_map(|(arguments, _)| {
            let printed: Value =
                serde_json::from_value(observe(printer, arguments)).expect("a printed value");
            let read = observe(reader, std::slice::from_ref(&printed));
            let expected = json(&option(Some(arguments[0].clone())));
            (read != expected).then(|| format!("{arguments:?} -> {printed:?} -> {read}"))
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

/// Run the case for `SL-08`.
///
/// # Panics
///
/// Panics when the case's assertion fails.
#[allow(clippy::too_many_lines)]
pub fn run(id: &str) {
    assert_eq!(id, "SL-08", "the case of {MODULE}");
    let project = P::stdlib();
    project.check_ok();
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    let report: Json = serde_json::from_slice(
        &std::fs::read(
            project
                .build_dir(&build_id)
                .join("production/LexLeanStd/Numeral.eligibility.json")
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
            "{member}: the evaluator and the compiler disagree on {} of {EVALUATOR_RUNS} runs: {:?}",
            disagreements.len(),
            &disagreements[..disagreements.len().min(5)]
        );
    }
    // The readers read back what the printers print.
    for (printer, reader) in [("decimal", "parseDecimal"), ("hex", "parseHex")] {
        let failures = round_trips(
            &lowered[printer],
            &lowered[reader],
            &corpus(printer, EVALUATOR_RUNS),
        );
        assert!(
            failures.is_empty(),
            "{reader} does not read back {printer} on {} of {EVALUATOR_RUNS} runs: {:?}",
            failures.len(),
            &failures[..failures.len().min(5)]
        );
    }
    // The rendered rust-std package of every member, compiled by the pinned
    // rustc, runs the larger corpus.
    let dir = tempfile::Builder::new()
        .prefix("lexlean-numeral-")
        .tempdir()
        .expect("tempdir");
    let mut crates = Vec::new();
    let mut expected: BTreeMap<String, Vec<Json>> = BTreeMap::new();
    for (member, _) in MEMBERS {
        let low = &lowered[member];
        let manifest = rust_packages::manifest(
            &format!("numeral_{}", snake(member)),
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
            "{name}: the rendering and the compiler disagree on {} of {} runs: {:?}",
            disagreements.len(),
            outcomes.len(),
            &disagreements[..disagreements.len().min(5)]
        );
    }
    // The gate is armed: a `hexValue` that reads every letter one too high
    // disagrees with the compiler on any text with a letter digit.
    let planted = P::stdlib();
    let text = planted.read_raw("src/Numeral.lex.tex");
    let site = "(prim subtract value 87 : Nat)";
    assert!(text.contains(site), "the plant site exists");
    planted.write(
        "src/Numeral.lex.tex",
        &text.replacen(site, "(prim subtract value 86 : Nat)", 1),
    );
    let checked = support::checked_project(&planted);
    let parse_hex = lower_in(&checked, &planted, MODULE, "parseHex");
    let disagreements = evaluate(&parse_hex, &corpus("parseHex", EVALUATOR_RUNS));
    assert!(
        !disagreements.is_empty(),
        "a planted letter value in `hexValue` is detected by the differential"
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
        // the `Numeral` members.
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
