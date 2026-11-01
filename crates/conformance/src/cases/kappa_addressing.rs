//! κ-addressing (SPEC.md §17.22, decision D7 of #110, #127): the `kappa`
//! project, whose κ-label and canonical JSON form are held to uor-addr, the
//! crate that defines them, as an oracle written independently of the
//! package. An oracle is evidence, not authority: a disagreement fails the
//! case and is a defect of the package, of the pipeline, or of the oracle, and
//! nothing here adopts the oracle's output.
//!
//! What is held, and what is not: the package writes the sha256 axis of the
//! κ-label, the JSON realization, and the five categorical compositions on
//! the sha256 axis. The GGUF and ONNX realizations of uor-addr and its four
//! other hash axes are not written; `KA-02`, `KA-03` and `KA-04` say so in
//! their statements by naming only what they hold.

use std::collections::BTreeMap;

use lexlean::calculus::rust::package::Passing;
use lexlean::calculus::rust::{self, Profile};
use lexlean::calculus::{self as target, interp, Value};
use lexlean::production::lower::Lowered;
use serde_json::Value as Json;

use super::stdlib::{lower_in, Run};
use crate::rust_build::{cargo_in, workspace, Built};
use crate::rust_harness::{self, Calls};
use crate::rust_packages;
use crate::support::{self, repo_root, P};

/// The members of the workspace the project needs.
const MEMBERS: [&str; 2] = ["kappa", "stdlib"];
/// The three modules of the package.
const LABEL: &str = "LexLeanKappa.Label";
const JSON: &str = "LexLeanKappa.Json";
const COMPOSE: &str = "LexLeanKappa.Compose";

/// The roots in the order their modules declare them, each with the effects
/// its declaration admits, which are exactly the effects its closure
/// realizes.
const ROOTS: [(&str, &str, &[&str]); 10] = [
    (LABEL, "kappaLabel", &["allocation", "overflow"]),
    (LABEL, "isLabel", &["allocation", "overflow"]),
    (LABEL, "handleWire", &["allocation", "overflow"]),
    (JSON, "canonicalJson", &["allocation", "overflow"]),
    (JSON, "jsonLabel", &["allocation", "overflow"]),
    (COMPOSE, "composeG2", &["allocation", "overflow"]),
    (COMPOSE, "composeF4", &["allocation", "overflow"]),
    (COMPOSE, "composeE6", &["allocation", "overflow"]),
    (COMPOSE, "composeE7", &["allocation", "overflow"]),
    (COMPOSE, "composeE8", &["allocation", "overflow"]),
];

/// Seeded documents under the evaluator.
const EVALUATOR_RUNS: usize = 120;
/// Seeded documents under rustc.
const RUSTC_RUNS: usize = 600;
/// Seeded operands of each composition, beside the fixed ones.
const COMPOSE_RUNS: usize = 40;
/// The evaluator's fuel; a root that exhausts it fails the case.
const FUEL: u64 = 400_000_000;

/// The twelve JSON fixtures of uor-addr's byte-identity test, harvested from
/// its reference endpoint: the raw input and the committed address.
const FIXTURES: [(&str, &str, &str); 12] = [
    (
        "simple_object",
        r#"{"foo": "bar"}"#,
        "sha256:7a38bf81f383f69433ad6e900d35b3e2385593f76a7b7ab5d4355b8ba41ee24b",
    ),
    (
        "empty_object",
        "{}",
        "sha256:44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a",
    ),
    (
        "empty_array",
        "[]",
        "sha256:4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945",
    ),
    (
        "key_sort_test",
        r#"{"b": 1, "a": 2}"#,
        "sha256:d3626ac30a87e6f7a6428233b3c68299976865fa5508e4267c5415c76af7a772",
    ),
    (
        "unicode_muller",
        "{\"name\": \"M\u{fc}ller\"}",
        "sha256:5e92260d138e28f7118a40c3c44be922d4569a39f9f1de676ca334cf19c3a37c",
    ),
    (
        "unicode_sao_paulo",
        "{\"city\": \"S\u{e3}o Paulo\"}",
        "sha256:0906524dcbec3bdb6aa4d7c22ed65d671ba32177b10823b6f256546280aa526b",
    ),
    (
        "unicode_cafe_composed",
        "{\"name\": \"caf\u{e9}\"}",
        "sha256:645fa443126a8954fc6d871912b8fc67bc2ee8feae417efe55546251962ca74d",
    ),
    (
        "unicode_cafe_decomposed",
        "{\"name\": \"cafe\u{301}\"}",
        "sha256:645fa443126a8954fc6d871912b8fc67bc2ee8feae417efe55546251962ca74d",
    ),
    (
        "mixed_types",
        r#"{"int": 42, "bool": true, "null_val": null}"#,
        "sha256:0966918f3851b97071b0e04d2576a2a86a197e71072b07061c8bfdaa6b6a5d2c",
    ),
    (
        "nested",
        r#"{"nested": {"deep": {"value": "found"}}}"#,
        "sha256:b18dce7d3cbf2ac3b908ad75c8420c4a42077192df69dffdbf786755724eff1d",
    ),
    (
        "string_array",
        r#"["a", "b", "c"]"#,
        "sha256:fa1844c2988ad15ab7b49e0ece09684500fad94df916859fb9a43ff85f5bb477",
    ),
    (
        "number_array",
        "[1, 2, 3]",
        "sha256:a615eeaee21de5179de080de8c3052c8da901138406ba71c38c032845f7d54f4",
    ),
];

/// The five committed `.kappa-label` files of uor-addr's GGUF and ONNX
/// fixtures (v0.2.0, `tests/fixtures/`), vendored under `tests/kappa/`. The
/// package writes neither realization, so these hold only the label form.
const COMMITTED_LABELS: [&str; 5] = [
    "gguf/aligned-256.kappa-label",
    "gguf/empty-metadata.kappa-label",
    "gguf/synthetic-f32.kappa-label",
    "onnx/synthetic-typed.kappa-label",
    "onnx/synthetic.kappa-label",
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
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
fn bytes(value: &[u8]) -> Value {
    Value::Bytes { hex: hex(value) }
}
fn option_bytes(value: Option<&[u8]>) -> Value {
    value.map_or(Value::None, |value| Value::Some {
        value: Box::new(bytes(value)),
    })
}

/// The expected observable of a run.
fn record(out: &mut Vec<Run>, arguments: Vec<Value>, expected: &Value) {
    out.push((
        arguments,
        serde_json::to_value(expected).expect("a value is JSON"),
    ));
}

/// What the oracle says of a document: its canonical form, or `None` for a
/// refusal.
fn oracle_canonical(raw: &[u8]) -> Option<Vec<u8>> {
    uor_addr::json::canonicalize(raw).ok()
}

/// What the oracle says of a document: its κ-label, or `None` for a refusal.
fn oracle_label(raw: &[u8]) -> Option<String> {
    uor_addr::json::address(raw)
        .ok()
        .map(|outcome| outcome.address.as_str().to_owned())
}

/// A generated document: its text, and whether it uses something the
/// package refuses by design (a number that is not an integer, a string
/// outside the subset whose NFC form it computes, or a repeated member name),
/// so that the oracle accepting it is not a disagreement.
struct Document {
    text: String,
    outside: bool,
}

/// Pieces of a string the package computes: ASCII, the escapes, the
/// characters that need escaping, Latin-1 in composed and decomposed form.
const INSIDE: &[&str] = &[
    "a",
    "Z",
    "0",
    " ",
    "-",
    "/",
    "\u{7f}",
    "\\\"",
    "\\\\",
    "\\/",
    "\\b",
    "\\f",
    "\\n",
    "\\r",
    "\\t",
    "\\u0001",
    "\\u001f",
    "\\u0041",
    "\\u00e9",
    "\\u00E9",
    "\\u0065\\u0301",
    "\u{e9}",
    "\u{c5}",
    "\u{ff}",
    "\u{a0}",
    "\u{df}",
    "e\u{301}",
    "A\u{30a}",
    "n\u{303}",
    "u\u{308}",
    "c\u{327}",
    "O\u{302}",
    "y\u{301}",
    "a\u{300}",
];
/// Pieces the package refuses: outside Latin-1, a combining mark with no
/// letter it composes with, and two marks.
const OUTSIDE: &[&str] = &[
    "\u{65e5}",
    "\u{1f600}",
    "\\u65e5",
    "\\ud83d\\ude00",
    "z\u{301}",
    "\u{e9}\u{301}",
    "e\u{304}",
    "\u{301}",
    "\u{153}",
];

impl Seeded {
    fn string(&mut self, outside: &mut bool) -> String {
        let mut text = String::new();
        for _ in 0..self.below(6) {
            if self.chance(4) {
                *outside = true;
                text.push_str(self.pick(OUTSIDE));
            } else {
                text.push_str(self.pick(INSIDE));
            }
        }
        text
    }

    fn gap(&mut self) -> &'static str {
        self.pick(&["", "", "", " ", "\n", "\t", "\r\n  "])
    }

    fn number(&mut self, outside: &mut bool) -> String {
        if self.chance(3) {
            *outside = true;
            return (*self.pick(&["1.5", "-0", "1e3", "2E-2", "0.0", "-0.5", "10.25e1"]))
                .to_owned();
        }
        let digits = [
            "0",
            "1",
            "7",
            "42",
            "100",
            "-1",
            "-17",
            "9007199254740993",
            "18446744073709551616",
            "-9223372036854775808",
        ];
        (*self.pick(&digits)).to_owned()
    }

    fn value(&mut self, depth: usize, outside: &mut bool) -> String {
        let kind = if depth == 0 {
            self.below(5)
        } else {
            self.below(7)
        };
        match kind {
            0 => "null".to_owned(),
            1 => (*self.pick(&["true", "false"])).to_owned(),
            2 | 3 => self.number(outside),
            4 => format!("\"{}\"", self.string(outside)),
            5 => {
                let count = self.below(4);
                let items: Vec<String> = (0..count)
                    .map(|_| {
                        format!(
                            "{}{}{}",
                            self.gap(),
                            self.value(depth - 1, outside),
                            self.gap()
                        )
                    })
                    .collect();
                format!("[{}{}]", self.gap(), items.join(","))
            }
            _ => {
                let count = self.below(5);
                let mut used: Vec<String> = Vec::new();
                let mut members = Vec::new();
                while members.len() < count {
                    let key = self.string(outside);
                    // Two raw keys that are one key once escapes and NFC are
                    // applied are a repeated name, which the package refuses.
                    let canon = oracle_canonical(format!("[\"{key}\"]").as_bytes());
                    let normal = canon.map_or_else(
                        || key.clone(),
                        |bytes| String::from_utf8_lossy(&bytes).into_owned(),
                    );
                    if used.contains(&normal) {
                        *outside = true;
                    }
                    used.push(normal);
                    members.push(format!(
                        "{}\"{key}\"{}:{}{}",
                        self.gap(),
                        self.gap(),
                        self.gap(),
                        self.value(depth - 1, outside)
                    ));
                }
                format!("{{{}{}}}", members.join(","), self.gap())
            }
        }
    }

    fn document(&mut self) -> Document {
        let mut outside = false;
        // The top level is an object or an array, so that no proper prefix
        // of a document is a document.
        let inner = self.value(3, &mut outside);
        let text = if inner.starts_with('{') || inner.starts_with('[') {
            inner
        } else {
            format!("[{inner}]")
        };
        Document {
            text: format!("{}{}{}", self.gap(), text, self.gap()),
            outside,
        }
    }
}

/// The corpus of canonical JSON runs: the fixtures, edge cases, malformed
/// documents, and seeded documents up to `runs`.
fn json_corpus(runs: usize, seed: u64, label: bool) -> Vec<Run> {
    let mut seeded = Seeded(seed);
    let mut out = Vec::new();
    let expect = |out: &mut Vec<Run>, raw: &[u8], outside: bool| {
        let canonical = oracle_canonical(raw).filter(|_| !outside);
        let expected = if label {
            canonical.map(|_| {
                let label = oracle_label(raw).expect("a label").into_bytes();
                assert_eq!(label.len(), 71, "a label has 71 bytes");
                label
            })
        } else {
            canonical
        };
        record(out, vec![bytes(raw)], &option_bytes(expected.as_deref()));
    };
    for (_, raw, _) in FIXTURES {
        expect(&mut out, raw.as_bytes(), false);
    }
    for raw in [
        "{\"a\":1,\"a\":2}",
        "[1.5]",
        "[-0]",
        "[1e3]",
        "[\"\\u65e5\"]",
        "",
        " ",
        "[",
        "{",
        "[1,]",
        "{\"a\":}",
        "{\"a\" 1}",
        "{,}",
        "[1] x",
        "[01]",
        "[\"\u{1}\"]",
        "[\"\\x\"]",
        "[\"\\u12\"]",
        "[tru]",
        "[nul]",
        "[-]",
        "{\"a\":1 \"b\":2}",
        "[1 2]",
        "\"x\"",
        "7",
        "null",
    ] {
        // The oracle and the package agree on a scalar at the top level and
        // on every malformed text; a repeated name and a number or string
        // outside the subset the oracle accepts are refused by design.
        let outside = matches!(
            raw,
            "{\"a\":1,\"a\":2}" | "[1.5]" | "[-0]" | "[1e3]" | "[\"\\u65e5\"]"
        );
        expect(&mut out, raw.as_bytes(), outside);
    }
    while out.len() < runs {
        let document = seeded.document();
        let raw = document.text.as_bytes();
        expect(&mut out, raw, document.outside);
        // Its proper prefixes and extensions are not documents.
        if !document.outside && out.len() < runs {
            let trimmed = document.text.trim_end();
            let cut = (1..trimmed.len())
                .filter(|at| trimmed.is_char_boundary(*at))
                .nth(seeded.below(trimmed.len().max(2) - 1))
                .unwrap_or(1);
            let prefix = &trimmed.as_bytes()[..cut];
            assert!(
                oracle_canonical(prefix).is_none(),
                "the oracle refuses a proper prefix of a document: {:?}",
                String::from_utf8_lossy(prefix)
            );
            expect(&mut out, prefix, false);
            let extended = format!("{trimmed},");
            expect(&mut out, extended.as_bytes(), false);
        }
    }
    out.truncate(runs);
    out
}

/// The runs of `kappaLabel`: the canonical bytes of every fixture and some
/// bytes of every length around the block size.
fn label_corpus() -> Vec<Run> {
    let mut out = Vec::new();
    let mut seeded = Seeded(0x6b61_7070_6100_0001);
    let mut subjects: Vec<Vec<u8>> = FIXTURES
        .iter()
        .map(|(_, raw, _)| oracle_canonical(raw.as_bytes()).expect("a fixture"))
        .collect();
    subjects.push(Vec::new());
    for length in [1, 55, 56, 63, 64, 65, 119, 120, 128, 1000] {
        subjects.push((0..length).map(|_| seeded.below(256) as u8).collect());
    }
    for subject in subjects {
        let label = format!("sha256:{}", {
            use sha2::{Digest, Sha256};
            hex(&Sha256::digest(&subject))
        });
        record(&mut out, vec![bytes(&subject)], &bytes(label.as_bytes()));
    }
    out
}

/// The runs of `isLabel`: the twelve fixture addresses, the six committed
/// label files, and strings that are one byte from a label.
fn is_label_corpus() -> Vec<Run> {
    let mut out = Vec::new();
    let boolean = |value: bool| Value::Bool { value };
    let mut accepted: Vec<String> = FIXTURES
        .iter()
        .map(|(_, _, label)| (*label).to_owned())
        .collect();
    let root = repo_root().join("tests/kappa");
    for file in COMMITTED_LABELS {
        let text = std::fs::read_to_string(root.join(file).as_std_path())
            .unwrap_or_else(|error| panic!("{file}: {error}"));
        accepted.push(text.trim_end().to_owned());
    }
    for label in &accepted {
        record(&mut out, vec![bytes(label.as_bytes())], &boolean(true));
        let mut upper = label.clone().into_bytes();
        upper[8] = upper[8].to_ascii_uppercase();
        let flipped = upper != label.as_bytes();
        let rejected = [
            label[..label.len() - 1].to_owned(),
            format!("{label}0"),
            label.replacen("sha256:", "sha512:", 1),
            label.replacen("sha256:", "sha256", 1),
            label.replacen("sha256:", "sha256;", 1),
            label.replacen(&label[7..8], "g", 1),
        ];
        for text in rejected {
            record(&mut out, vec![bytes(text.as_bytes())], &boolean(false));
        }
        if flipped {
            record(&mut out, vec![bytes(&upper)], &boolean(false));
        }
    }
    record(&mut out, vec![bytes(b"")], &boolean(false));
    out
}

/// The runs of `handleWire`: valid and invalid kinds against a canonical
/// form.
fn handle_corpus() -> Vec<Run> {
    let mut out = Vec::new();
    let canonical = oracle_canonical(br#"{"foo":"bar"}"#).expect("a fixture");
    let label = oracle_label(br#"{"foo":"bar"}"#).expect("a label");
    for kind in ["json", "onnx-model", "gguf", "int8-tensor", "a0", "0", "-"] {
        let wire = format!("{kind}@{label}");
        record(
            &mut out,
            vec![bytes(kind.as_bytes()), bytes(&canonical)],
            &option_bytes(Some(wire.as_bytes())),
        );
    }
    for kind in [
        "", "Json", "json ", "json@x", "on_nx", "\u{e9}", "json\n", "a:b",
    ] {
        record(
            &mut out,
            vec![bytes(kind.as_bytes()), bytes(&canonical)],
            &Value::None,
        );
    }
    out
}

/// The oracle's composition of the operands: the label it mints, or `None`
/// for any error (an operand of another axis, a malformed one, or one of the
/// wrong width).
fn oracle_compose(member: &str, operands: &[Vec<u8>]) -> Option<String> {
    use uor_addr::composition as c;
    use uor_addr::KappaLabel;
    let operand = |index: usize| KappaLabel::<71>::from_bytes(&operands[index]).ok();
    let outcome = match member {
        "composeG2" => c::compose_g2_product(&operand(0)?, &operand(1)?),
        "composeF4" => c::compose_f4_quotient(&operand(0)?),
        "composeE6" => c::compose_e6_filtration(&operand(0)?),
        "composeE7" => c::compose_e7_augmentation(&operand(0)?),
        "composeE8" => c::compose_e8_embedding(&operand(0)?),
        other => panic!("`{other}` is not a composition"),
    };
    outcome
        .ok()
        .map(|outcome| outcome.address.as_str().to_owned())
}

/// Operand labels of the sha256 axis: the extremes, digests whose first byte
/// modulo nine is eight (the degree-6 class of E6), a digest and its mirror,
/// digests whose quarters are in every order, and seeded digests.
fn valid_operands(seeded: &mut Seeded, random: usize) -> Vec<Vec<u8>> {
    let mut digests: Vec<Vec<u8>> = vec![vec![0; 32], vec![0xff; 32], (1..=32).collect()];
    for first in [0x08, 0x11, 0x1a, 0xf8, 0xfb, 0x00, 0x07, 0x09] {
        let mut digest: Vec<u8> = (0..32).map(|_| seeded.below(256) as u8).collect();
        digest[0] = first;
        digests.push(digest);
    }
    // Quarters in every one of the 24 orders of four distinct blocks.
    let quarters: Vec<Vec<u8>> = (0..4u8)
        .map(|quarter| vec![0x10 * (quarter + 1) + 3; 8])
        .collect();
    for a in 0..4usize {
        for b in (0..4usize).filter(|b| *b != a) {
            for c in (0..4usize).filter(|c| *c != a && *c != b) {
                let d = 6 - a - b - c;
                digests.push(
                    [a, b, c, d]
                        .iter()
                        .flat_map(|index| quarters[*index].clone())
                        .collect(),
                );
            }
        }
    }
    // Equal leading quarters, so that the order is decided late.
    digests.push([[7u8; 8].as_slice(), &[7; 8], &[3; 8], &[9; 8]].concat());
    for _ in 0..random {
        digests.push((0..32).map(|_| seeded.below(256) as u8).collect());
    }
    let mirrors: Vec<Vec<u8>> = digests
        .iter()
        .take(12)
        .map(|digest| digest.iter().map(|byte| !byte).collect())
        .collect();
    digests.extend(mirrors);
    digests
        .iter()
        .map(|digest| format!("sha256:{}", hex(digest)).into_bytes())
        .collect()
}

/// Operands the package refuses: another axis, the case of the prefix, a
/// digest of the wrong width, upper case, and characters that are not digits.
fn invalid_operands(valid: &[u8]) -> Vec<Vec<u8>> {
    let text = String::from_utf8(valid.to_vec()).expect("a label");
    let digits = &text[7..];
    let mut out: Vec<Vec<u8>> = vec![
        format!("blake3:{digits}").into_bytes(),
        format!("sha3-256:{digits}00").into_bytes(),
        format!("SHA256:{digits}").into_bytes(),
        format!("sha256:{}", &digits[1..]).into_bytes(),
        format!("sha256:{digits}0").into_bytes(),
        format!("sha256:{}", digits.to_uppercase()).into_bytes(),
        format!("sha256:g{}", &digits[1..]).into_bytes(),
        format!("sha256;{digits}").into_bytes(),
        format!("sha256:{digits}\n").into_bytes(),
        format!("sha512:{digits}{digits}").into_bytes(),
        Vec::new(),
        b"sha256:".to_vec(),
    ];
    // A label with a non-ASCII byte in a digit position.
    let mut high = valid.to_vec();
    high[10] = 0xe9;
    out.push(high);
    out
}

/// The runs of one composition: valid operands (the pairs in both orders for
/// the product) and the refused ones, with the oracle's label.
fn compose_corpus(member: &str, runs: usize) -> Vec<Run> {
    let mut seeded = Seeded(0x6b61_0004_0000_0001);
    let valid = valid_operands(&mut seeded, runs);
    let mut out = Vec::new();
    let mut run = |operands: Vec<Vec<u8>>| {
        let expected = option_bytes(
            oracle_compose(member, &operands)
                .as_deref()
                .map(str::as_bytes),
        );
        record(
            &mut out,
            operands.iter().map(|operand| bytes(operand)).collect(),
            &expected,
        );
    };
    if member == "composeG2" {
        for (index, left) in valid.iter().enumerate() {
            let right = &valid[(index * 7 + 3) % valid.len()];
            run(vec![left.clone(), right.clone()]);
            run(vec![right.clone(), left.clone()]);
            run(vec![left.clone(), left.clone()]);
        }
        for bad in invalid_operands(&valid[3]) {
            run(vec![bad.clone(), valid[3].clone()]);
            run(vec![valid[3].clone(), bad]);
        }
    } else {
        for operand in &valid {
            run(vec![operand.clone()]);
        }
        for bad in invalid_operands(&valid[3]) {
            run(vec![bad]);
        }
    }
    out
}

/// The corpus of one root, `runs` documents long where it is seeded.
fn corpus(member: &str, runs: usize) -> Vec<Run> {
    match member {
        "kappaLabel" => label_corpus(),
        "isLabel" => is_label_corpus(),
        "handleWire" => handle_corpus(),
        "canonicalJson" => json_corpus(runs, 0x4b41_0003, false),
        "jsonLabel" => json_corpus(runs, 0x4b41_0002, true),
        "composeG2" | "composeF4" | "composeE6" | "composeE7" | "composeE8" => {
            compose_corpus(member, runs)
        }
        other => panic!("`{other}` is not a root of the package"),
    }
}

/// Every run on which the evaluator and the oracle disagree.
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

fn lowered_roots(project: &P) -> BTreeMap<&'static str, Lowered> {
    let checked = support::checked_project(project);
    ROOTS
        .iter()
        .map(|(module, member, _)| (*member, lower_in(&checked, project, module, member)))
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

/// The rendered `rust-std` package of each root named, compiled by the
/// pinned rustc and run on the root's corpus; every run is compared with the
/// oracle's outcome.
fn rustc_differential(lowered: &BTreeMap<&'static str, Lowered>, members: &[&str]) {
    let dir = tempfile::Builder::new()
        .prefix("lexlean-kappa-")
        .tempdir()
        .expect("tempdir");
    let mut crates = Vec::new();
    let mut expected: BTreeMap<String, Vec<Json>> = BTreeMap::new();
    for member in members {
        let low = &lowered[member];
        let manifest = rust_packages::manifest(
            &format!("kappa_{}", snake(member)),
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
        "a root's package does not build:\n{}",
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
            "{name}: the rendering and uor-addr disagree on {} of {} runs: {:?}",
            disagreements.len(),
            outcomes.len(),
            &disagreements[..disagreements.len().min(5)]
        );
    }
}

/// A copy of the project with one edit to one module.
fn planted(module: &str, from: &str, to: &str) -> P {
    let project = P::workspace_project("kappa", &MEMBERS);
    let path = format!("src/{module}.lex.tex");
    let text = project.read_raw(&path);
    assert!(
        text.contains(from),
        "the plant site `{from}` exists in {module}"
    );
    project.write(&path, &text.replacen(from, to, 1));
    project
}

/// The pinned `compiler_semantics` of a committed lock.
fn pinned(lock: &str) -> String {
    let text = std::fs::read_to_string(repo_root().join(lock).as_std_path())
        .unwrap_or_else(|error| panic!("{lock}: {error}"));
    text.lines()
        .find_map(|line| {
            line.strip_prefix("compiler_semantics = \"")
                .and_then(|rest| rest.strip_suffix('"'))
        })
        .unwrap_or_else(|| panic!("{lock}: no compiler_semantics"))
        .to_owned()
}

/// Run the case for one `KA` conformance ID.
///
/// # Panics
///
/// Panics when the case's assertion fails.
pub fn run(id: &str) {
    match id {
        "KA-01" => package(id),
        "KA-02" => labels(id),
        "KA-03" => canonical_json(),
        "KA-04" => composition(),
        other => panic!("no kappa-addressing case is wired for {other}"),
    }
}

/// `KA-01`: the project, its roots, and the identities it does not move.
fn package(id: &str) {
    let project = P::workspace_project("kappa", &MEMBERS);
    let manifest = project.read_raw("lexlean.toml");
    assert!(
        manifest.contains("module_prefix = \"LexLeanKappa\""),
        "{id}: the module prefix"
    );
    assert_eq!(
        manifest.matches("[[source_package]]").count(),
        1,
        "{id}: one source package"
    );
    assert!(
        manifest.contains("package = \"std\"") && manifest.contains("path = \"stdlib\""),
        "{id}: the source package is the standard library"
    );
    for forbidden in ["[[artifact", "[[glossary"] {
        assert!(!manifest.contains(forbidden), "{id}: no {forbidden}");
    }
    let workspace = lexlean::source_package::Workspace::read(&repo_root())
        .expect("the workspace file reads")
        .expect("the repository has a workspace file");
    assert!(
        workspace.members.iter().any(|member| member == "kappa")
            && workspace.members.iter().any(|member| member == "stdlib"),
        "{id}: kappa is a member of the workspace beside stdlib"
    );
    project.check_ok();
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    for module in ["Label", "Json", "Compose"] {
        let report: Json = serde_json::from_slice(
            &std::fs::read(
                project
                    .build_dir(&build_id)
                    .join(format!("production/LexLeanKappa/{module}.eligibility.json"))
                    .as_std_path(),
            )
            .expect("the eligibility report"),
        )
        .expect("the report is JSON");
        let reported = report["roots"].as_array().expect("roots");
        let wanted: Vec<_> = ROOTS
            .iter()
            .filter(|(owner, _, _)| owner.ends_with(module))
            .collect();
        assert_eq!(
            reported
                .iter()
                .map(|row| row["root"].as_str().expect("root").to_owned())
                .collect::<Vec<_>>(),
            wanted
                .iter()
                .map(|(owner, member, _)| format!("{owner}.{member}"))
                .collect::<Vec<_>>(),
            "{id}: the production roots of {module}, in order"
        );
        for (row, (_, member, effects)) in reported.iter().zip(wanted) {
            let declared: Vec<&str> = row["declared_effects"]
                .as_array()
                .expect("declared effects")
                .iter()
                .map(|effect| effect.as_str().expect("an effect"))
                .collect();
            assert_eq!(declared, *effects, "{id}: {member}: the declared effects");
            let targets = row["targets"].as_array().expect("targets");
            assert_eq!(targets.len(), 1, "{id}: {member}: one target");
            assert_eq!(targets[0]["target"], "rust-std", "{id}: {member}: target");
            assert_eq!(targets[0]["status"], "eligible", "{id}: {member}: eligible");
            let realized: Vec<&str> = targets[0]["effects"]
                .as_array()
                .expect("effects")
                .iter()
                .map(|effect| effect["effect"].as_str().expect("an effect"))
                .collect();
            assert_eq!(
                realized, *effects,
                "{id}: {member}: the declaration admits exactly the realized effects"
            );
        }
    }
    // The package lives outside `language/` and `schemas/`: no file of either
    // names it, and the identities of the three languages are the ones the
    // committed locks pin.
    for tree in ["language", "schemas"] {
        for entry in walkdir::WalkDir::new(repo_root().join(tree).as_std_path())
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
        {
            if let Ok(text) = std::fs::read_to_string(entry.path()) {
                assert!(
                    !text.contains("LexLeanKappa"),
                    "{id}: {} names the package",
                    entry.path().display()
                );
            }
        }
    }
    for (language, lock) in [
        ("1.0", "examples/nat-add-zero/lexlean.lock"),
        ("1.1", "examples/semantic-1.1/lexlean.lock"),
        ("1.2", "examples/language-1.2/lexlean.lock"),
    ] {
        if repo_root().join(lock).as_std_path().exists() {
            assert_eq!(
                lexlean::compiler_semantics_id_for(language).to_hex(),
                pinned(lock),
                "{id}: the compiler-semantics ID of language {language} is the one {lock} pins"
            );
        }
    }
    assert_eq!(
        pinned("kappa/lexlean.lock"),
        pinned("stdlib/lexlean.lock"),
        "{id}: the package pins the identity of the standard library beside it"
    );
    support::verify_ok_backed(id, &project);
    // The gate is armed: a package that drops the declared source package
    // does not check, since its modules import the standard library.
    let broken = P::workspace_project("kappa", &MEMBERS);
    let dropped = manifest.replace(
        "[[source_package]]\npackage = \"std\"\nversion = \"1.0.0\"\nkind = \"path\"\npath = \"stdlib\"\n\n",
        "",
    );
    assert_ne!(dropped, manifest, "{id}: the plant site exists");
    broken.write_raw("lexlean.toml", &dropped);
    assert!(
        !broken.check_err_or_ok(),
        "{id}: a project without its source package does not check"
    );
    eprintln!("{id}: plant (source package dropped): the project does not check");
}

/// `KA-02`: the label, the handle, and the fixtures.
fn labels(id: &str) {
    let project = P::workspace_project("kappa", &MEMBERS);
    project.check_ok();
    let lowered = lowered_roots(&project);
    // The twelve fixtures: the committed address, the oracle's, and the
    // package's, as the evaluator computes it.
    let json = &lowered["jsonLabel"];
    for (name, raw, committed) in FIXTURES {
        assert_eq!(
            oracle_label(raw.as_bytes()).as_deref(),
            Some(committed),
            "{id}: {name}: the oracle gives the committed address"
        );
        let outcome = interp::run(&json.program, FUEL, json.entry(), &[bytes(raw.as_bytes())]);
        let observed = rust::observable(&outcome).expect("an observable");
        assert_eq!(
            observed,
            serde_json::to_value(option_bytes(Some(committed.as_bytes()))).expect("json"),
            "{id}: {name}: the package gives the committed address"
        );
    }
    for member in ["kappaLabel", "isLabel", "handleWire", "jsonLabel"] {
        let runs = corpus(member, EVALUATOR_RUNS);
        let disagreements = evaluate(&lowered[member], &runs);
        assert!(
            disagreements.is_empty(),
            "{id}: {member}: the evaluator and uor-addr disagree on {} of {} runs: {:?}",
            disagreements.len(),
            runs.len(),
            &disagreements[..disagreements.len().min(5)]
        );
    }
    rustc_differential(
        &lowered,
        &["kappaLabel", "isLabel", "handleWire", "jsonLabel"],
    );
    // The gate is armed: a label with the wrong prefix, a label whose
    // alphabet admits uppercase, and a handle that forgets its kind are each
    // detected.
    for (module, from, to, member) in [
        (
            "Label",
            "(text \"sha256:\") (LexLeanStd.Sha256::sha256Hex canonical)",
            "(text \"sha512:\") (LexLeanStd.Sha256::sha256Hex canonical)",
            "kappaLabel",
        ),
        (
            "Label",
            "(if (96 <? n) then (n <? 103) else false)",
            "(if (64 <? n) then (n <? 103) else false)",
            "isLabel",
        ),
        (
            "Label",
            "(prim append (project handle kind) (prim append (text \"@\")",
            "(prim append (project handle label) (prim append (text \"@\")",
            "handleWire",
        ),
    ] {
        let defective = planted(module, from, to);
        let checked = support::checked_project(&defective);
        let low = lower_in(&checked, &defective, LABEL, member);
        let runs = corpus(member, EVALUATOR_RUNS);
        let disagreements = evaluate(&low, &runs);
        assert!(
            !disagreements.is_empty(),
            "{id}: a planted defect in `{member}` is detected by the differential"
        );
        eprintln!(
            "{id}: plant in {member}: {} of {} runs disagree, the first {}",
            disagreements.len(),
            runs.len(),
            disagreements[0]
        );
    }
}

/// `KA-03`: the canonical JSON form.
fn canonical_json() {
    let project = P::workspace_project("kappa", &MEMBERS);
    project.check_ok();
    let lowered = lowered_roots(&project);
    let runs = corpus("canonicalJson", EVALUATOR_RUNS);
    let refusal = serde_json::to_value(Value::None).expect("a value is JSON");
    assert!(
        runs.iter().any(|(_, expected)| *expected == refusal)
            && runs.iter().any(|(_, expected)| *expected != refusal),
        "KA-03: the corpus holds both accepted and refused documents"
    );
    let refused = runs
        .iter()
        .filter(|(_, expected)| *expected == refusal)
        .count();
    eprintln!(
        "KA-03: {} evaluator runs, {refused} refused by uor-addr or by design, {} accepted",
        runs.len(),
        runs.len() - refused
    );
    let disagreements = evaluate(&lowered["canonicalJson"], &runs);
    assert!(
        disagreements.is_empty(),
        "KA-03: the evaluator and uor-addr disagree on {} of {} runs: {:?}",
        disagreements.len(),
        runs.len(),
        &disagreements[..disagreements.len().min(5)]
    );
    rustc_differential(&lowered, &["canonicalJson"]);
    // The gate is armed: members kept in input order, an escape that writes
    // a raw newline, and a composition that drops the mark are each detected.
    for (from, to) in [
        (
            "(if (lessKey key (first item))",
            "(if (lessKey (first item) key)",
        ),
        (
            "(if (b =? 10) then 0x5c6e",
            "(if (b =? 10) then 0x0a",
        ),
        (
            "(prim append (LexLeanStd.Text::sliceOr text 0 (prim subtract size 1 : Nat)) (encodeHigh replacement) : Bytes)",
            "(prim append (LexLeanStd.Text::sliceOr text 0 (prim subtract size 1 : Nat)) (byteOf base) : Bytes)",
        ),
    ] {
        let defective = planted("Json", from, to);
        let checked = support::checked_project(&defective);
        let low = lower_in(&checked, &defective, JSON, "canonicalJson");
        let runs = corpus("canonicalJson", EVALUATOR_RUNS);
        let disagreements = evaluate(&low, &runs);
        assert!(
            !disagreements.is_empty(),
            "KA-03: a planted defect `{to}` is detected by the differential"
        );
        eprintln!(
            "KA-03: plant `{to}`: {} of {} runs disagree, the first {}",
            disagreements.len(),
            runs.len(),
            disagreements[0]
        );
    }
}

/// The observable of one root on the given operands.
fn observe(low: &Lowered, operands: &[&[u8]]) -> Json {
    let arguments: Vec<Value> = operands.iter().map(|operand| bytes(operand)).collect();
    let outcome = interp::run(&low.program, FUEL, low.entry(), &arguments);
    rust::observable(&outcome).unwrap_or_else(|| panic!("{outcome:?}"))
}

/// `KA-04`: the five compositions of the sha256 axis.
fn composition() {
    let project = P::workspace_project("kappa", &MEMBERS);
    project.check_ok();
    let lowered = lowered_roots(&project);
    let members = [
        "composeG2",
        "composeF4",
        "composeE6",
        "composeE7",
        "composeE8",
    ];
    let refusal = serde_json::to_value(Value::None).expect("a value is JSON");
    for member in members {
        let runs = corpus(member, COMPOSE_RUNS);
        assert!(
            runs.iter().any(|(_, expected)| *expected == refusal)
                && runs.iter().any(|(_, expected)| *expected != refusal),
            "KA-04: {member}: the corpus holds both labels and refusals"
        );
        let disagreements = evaluate(&lowered[member], &runs);
        assert!(
            disagreements.is_empty(),
            "KA-04: {member}: the evaluator and uor-addr disagree on {} of {} runs: {:?}",
            disagreements.len(),
            runs.len(),
            &disagreements[..disagreements.len().min(5)]
        );
    }
    // The laws, read from the evaluator: the product is commutative and a
    // label and its mirror compose alike, as the oracle's own tests state.
    let operands = valid_operands(&mut Seeded(0x6b61_0004_0000_0001), COMPOSE_RUNS);
    for (index, left) in operands.iter().enumerate().take(24) {
        let right = &operands[(index * 5 + 1) % operands.len()];
        assert_eq!(
            observe(&lowered["composeG2"], &[left, right]),
            observe(&lowered["composeG2"], &[right, left]),
            "KA-04: the product is commutative"
        );
    }
    for operand in operands.iter().take(12) {
        let mirror: Vec<u8> = format!(
            "sha256:{}",
            String::from_utf8_lossy(&operand[7..])
                .chars()
                .map(|digit| {
                    let value = digit.to_digit(16).expect("a hexadecimal digit");
                    char::from_digit(15 - value, 16).expect("a digit")
                })
                .collect::<String>()
        )
        .into_bytes();
        assert_eq!(
            observe(&lowered["composeF4"], &[operand]),
            observe(&lowered["composeF4"], &[&mirror]),
            "KA-04: a label and its mirror compose alike"
        );
    }
    rustc_differential(&lowered, &members);
    // The gate is armed: a product that keeps the operands in the order
    // given, a mirror that is not the complement, a degree byte that is
    // never six, and quarters left in place are each detected.
    for (member, from, to) in [
        (
            "composeG2",
            "(prim append (lower left right) (upper left right) : Bytes)",
            "(prim append left right : Bytes)",
        ),
        (
            "composeF4",
            "(prim subtract 15 (Json::hexValue",
            "(prim subtract 14 (Json::hexValue",
        ),
        ("composeE6", "9 0 : Nat) =? 8)", "9 0 : Nat) =? 9)"),
        (
            "composeE7",
            "(prim append a2 (prim append b3 (prim append c3 d2 : Bytes) : Bytes) : Bytes)",
            "(prim append a0 (prim append b0 (prim append c0 d0 : Bytes) : Bytes) : Bytes)",
        ),
    ] {
        let defective = planted("Compose", from, to);
        let checked = support::checked_project(&defective);
        let low = lower_in(&checked, &defective, COMPOSE, member);
        let runs = corpus(member, COMPOSE_RUNS);
        let disagreements = evaluate(&low, &runs);
        assert!(
            !disagreements.is_empty(),
            "KA-04: a planted defect `{to}` in `{member}` is detected by the differential"
        );
        eprintln!(
            "KA-04: plant in {member}: {} of {} runs disagree, the first {}",
            disagreements.len(),
            runs.len(),
            disagreements[0]
        );
    }
}
