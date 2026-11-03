//! Canonical JSON reading in the standard library (SPEC.md §17.22, `SL-13`,
//! #276): the `LexLeanStd.JsonRead` module against the compiler's own reader
//! (stage 0), `artifact::canonical_json::Json::parse`, under the calculus
//! evaluator and under the pinned rustc.
//!
//! The module's reader accepts a text exactly when it is canonical, which the
//! oracle states as: the compiler parses it, and writes what it parsed back
//! to the same bytes. A text with whitespace, a number that is not an
//! integer in its shortest form, an escape the writer would not emit, or keys
//! out of order or repeated is parsed by the oracle or refused by it, and
//! either way not written back as it was. The texts of the corpus are valid
//! UTF-8, which the module does not check, as the writer does not (`SL-11`);
//! nor does it count the nesting that `serde_json` limits to 128 levels.

use std::collections::BTreeMap;

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
const MODULE: &str = "LexLeanStd.JsonRead";

/// The members in the order the module declares them, each with the effects
/// its declaration admits, which are exactly the effects its closure
/// realizes.
const MEMBERS: [(&str, &[&str]); 2] = [
    ("readJson", &["allocation", "overflow"]),
    ("readJsonFile", &["allocation", "overflow"]),
];

/// Argument lists per member under the evaluator.
const EVALUATOR_RUNS: usize = 300;
/// Argument lists per member under rustc.
const RUSTC_RUNS: usize = 1_500;
/// The evaluator's fuel; a member that exhausts it fails the case.
const FUEL: u64 = 200_000_000;
/// The deepest tree a seeded run reads.
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
/// What a mutation inserts or puts in place of a character: the structure of
/// JSON, the digits and signs, the letters of its words and escapes, and
/// controls.
const MUTATIONS: &[char] = &[
    '"', '\\', ',', ':', '[', ']', '{', '}', '0', '1', '9', '-', '+', '.', 'e', 'E', 't', 'f', 'n',
    'u', '/', ' ', '\n', '\t', '\u{0}', 'a', 'b', 'x',
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

/// A seeded canonical tree: an object is a `BTreeMap`, so its keys ascend and
/// none repeats.
fn tree(seeded: &mut Seeded, depth: usize) -> Canonical {
    let kinds = if depth == 0 { 3 } else { 5 };
    match seeded.below(kinds) {
        0 => Canonical::Bool(seeded.below(2) == 0),
        1 => Canonical::Int(seeded.integer()),
        2 => Canonical::Str(seeded.text(STRING_ATOMS, 4)),
        3 => Canonical::Arr(
            (0..seeded.below(MAX_WIDTH + 1))
                .map(|_| tree(seeded, depth - 1))
                .collect(),
        ),
        _ => Canonical::Obj(
            (0..seeded.below(MAX_WIDTH + 1))
                .map(|_| {
                    let key = seeded.text(KEY_ATOMS, 3);
                    (key, tree(seeded, depth - 1))
                })
                .collect::<BTreeMap<_, _>>(),
        ),
    }
}

/// A text with one to three characters deleted, replaced, or inserted.
fn mutate(seeded: &mut Seeded, text: &str) -> String {
    let mut characters: Vec<char> = text.chars().collect();
    for _ in 0..=seeded.below(3) {
        let mutation = MUTATIONS[seeded.below(MUTATIONS.len())];
        match seeded.below(3) {
            0 if !characters.is_empty() => {
                let at = seeded.below(characters.len());
                characters.remove(at);
            }
            1 if !characters.is_empty() => {
                let at = seeded.below(characters.len());
                characters[at] = mutation;
            }
            _ => {
                let at = seeded.below(characters.len() + 1);
                characters.insert(at, mutation);
            }
        }
    }
    characters.into_iter().collect()
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

/// The module's value of a tree: the constructors in declaration order,
/// `JBool` first, with the elements of an array and the members of an object
/// as spines of `JItem` and `JMember` ended by `JNil`, the members in the
/// order a `BTreeMap` iterates them.
fn value(json: &Canonical) -> Value {
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
    match json {
        Canonical::Bool(flag) => adt(BOOL, vec![Value::Bool { value: *flag }]),
        Canonical::Int(number) => adt(
            INT,
            vec![
                Value::Bool { value: *number < 0 },
                Value::Nat {
                    value: number.unsigned_abs().to_string(),
                },
            ],
        ),
        Canonical::Str(text) => adt(STR, vec![bytes(text.as_bytes())]),
        Canonical::Arr(items) => adt(
            ARR,
            vec![items.iter().rev().fold(adt(NIL, vec![]), |rest, item| {
                adt(ITEM, vec![value(item), rest])
            })],
        ),
        Canonical::Obj(members) => adt(
            OBJ,
            vec![members
                .iter()
                .rev()
                .fold(adt(NIL, vec![]), |rest, (key, inner)| {
                    adt(MEMBER, vec![bytes(key.as_bytes()), value(inner), rest])
                })],
        ),
    }
}

/// The reader's oracle: the compiler parses the text, and writes what it
/// parsed back to the same bytes.
fn read(text: &[u8]) -> Option<Canonical> {
    let parsed = Canonical::parse(text).ok()?;
    (parsed.to_canonical_string().as_bytes() == text).then_some(parsed)
}
/// The file reader's oracle: the payload and exactly one final LF.
fn read_file(text: &[u8]) -> Option<Canonical> {
    read(text.strip_suffix(b"\n")?)
}
/// What a member returns on a text, as the evaluator observes it.
fn outcome(member: &str, text: &[u8]) -> Observed {
    let parsed = if member == "readJsonFile" {
        read_file(text)
    } else {
        read(text)
    };
    observed(&match parsed {
        Some(tree) => Value::Some {
            value: Box::new(value(&tree)),
        },
        None => Value::None,
    })
}

/// The texts of every corpus: canonical ones, then the ones that break one
/// rule of §21.7 each.
const FIXED: &[&str] = &[
    // Canonical.
    "true",
    "false",
    "0",
    "-1",
    "9223372036854775807",
    "-9223372036854775808",
    "\"\"",
    "[]",
    "{}",
    "[[],{},7]",
    "[[[[[[[[[[1]]]]]]]]]]",
    "{\"a\":1,\"ab\":[true,false],\"b\":{},\"é\":\"é\\u0001\"}",
    "\"\\\"\\\\\\b\\f\\n\\r\\t\\u0000\\u0007\\u000b\\u001f\u{7f}€😀\"",
    "{\"\":0,\"\\u0001\":1}",
    // Not canonical.
    "",
    " ",
    "null",
    "[null]",
    "1.0",
    "1e2",
    "1E2",
    "-0",
    "+1",
    "01",
    "-01",
    "-",
    "9223372036854775808",
    "-9223372036854775809",
    "18446744073709551616",
    "[1,]",
    "[,1]",
    "[1 ]",
    "[ 1]",
    " [1]",
    "[1]x",
    "[1]\n",
    "{\"a\":1,}",
    "{\"b\":1,\"a\":2}",
    "{\"a\":1,\"a\":2}",
    "{\"a\":1 }",
    "{\"a\"1}",
    "{1:2}",
    "{\"a\":}",
    "\"\\/\"",
    "\"\\u0041\"",
    "\"\\u000a\"",
    "\"\\u001F\"",
    "\"\\u007f\"",
    "\"\\u00\"",
    "\"\\ud83d\\ude00\"",
    "\"\\x\"",
    "\"\\",
    "\"a",
    "\"a\tb\"",
    "\"a\u{1}b\"",
    "tru",
    "truex",
    "fals",
    "[true,false",
    "[",
    "]",
    "{",
    "}",
];

/// The corpus of one member: the fixed texts, then seeded ones up to `runs`,
/// each with the outcome of the member's oracle.
fn corpus(member: &str, runs: usize) -> Vec<Run> {
    let mut seed = 0x9e37_79b9_7f4a_7c15_u64;
    for character in member.bytes() {
        seed = seed.rotate_left(8) ^ u64::from(character);
    }
    let mut seeded = Seeded(seed);
    let file = member == "readJsonFile";
    let case = |text: &[u8]| -> Run { (vec![bytes(text)], outcome(member, text)) };
    let mut out: Vec<Run> = Vec::new();
    for text in FIXED {
        out.push(case(text.as_bytes()));
        if file {
            out.push(case(format!("{text}\n").as_bytes()));
        }
    }
    while out.len() < runs {
        let written = tree(&mut seeded, MAX_DEPTH).to_canonical_string();
        let mut text = match seeded.below(4) {
            0 | 1 => written,
            _ => mutate(&mut seeded, &written),
        };
        if file && seeded.below(4) != 0 {
            text.push('\n');
        }
        out.push(case(text.as_bytes()));
    }
    out
}

/// A defect planted in the source of `JsonRead`, caught on a vector of its
/// own.
struct Plant {
    label: &'static str,
    /// The member the vector is run on.
    member: &'static str,
    /// The text replaced, once in the module.
    site: &'static str,
    replacement: &'static str,
    /// The text the vector gives the member.
    text: &'static str,
    /// Whether the oracle reads the text.
    accepted: bool,
}

/// One defect for each rule of the canonical form the module holds. Each
/// leaves every other rule intact, so that only the vector of its own catches
/// it.
const PLANTS: [Plant; 13] = [
    Plant {
        label: "keys out of order are read",
        member: "readJson",
        site: "(Order::lessBytes last key)",
        replacement: "true",
        text: "{\"b\":1,\"a\":2}",
        accepted: false,
    },
    Plant {
        label: "minus zero is read",
        member: "readJson",
        site: "((0 <? magnitude) && (magnitude <=? 9223372036854775808))",
        replacement: "(magnitude <=? 9223372036854775808)",
        text: "-0",
        accepted: false,
    },
    Plant {
        label: "a number of twenty digits reaches the decimal reader",
        member: "readJson",
        site: "(19 <? (prim subtract stop start : Nat))",
        replacement: "false",
        text: "18446744073709551616",
        accepted: false,
    },
    Plant {
        label: "a magnitude past i64 is read",
        member: "readJson",
        site: "else (magnitude <=? 9223372036854775807)",
        replacement: "else (magnitude <=? 9223372036854775808)",
        text: "9223372036854775808",
        accepted: false,
    },
    Plant {
        label: "the long form of a short escape is read",
        member: "readJson",
        site: "(if (shortForm value) then 0x else decoded)",
        replacement: "decoded",
        text: "\"\\u000a\"",
        accepted: false,
    },
    Plant {
        label: "an escape of a printable byte is read",
        member: "readJson",
        site: "(if (value <? 32) then (if (shortForm value) then 0x else decoded) else 0x)",
        replacement: "(if (shortForm value) then 0x else decoded)",
        text: "\"\\u0041\"",
        accepted: false,
    },
    Plant {
        label: "an escaped solidus is read",
        member: "readJson",
        site: "(if (letter =? 116) then 0x09 else 0x)",
        replacement: "(if (letter =? 116) then 0x09 else (if (letter =? 47) then 0x2f else 0x))",
        text: "\"\\/\"",
        accepted: false,
    },
    Plant {
        label: "a byte after the value is ignored",
        member: "readJson",
        site: "(if (byte =? 256) then 8 else 9)",
        replacement: "8",
        text: "[]x",
        accepted: false,
    },
    Plant {
        label: "a trailing comma is read",
        member: "readJson",
        site: "((byte =? 93) && (mode =? 1))",
        replacement: "(byte =? 93)",
        text: "[1,]",
        accepted: false,
    },
    Plant {
        label: "a raw control byte in a string is read",
        member: "readJson",
        site: "((byte <? 32) || (byte =? 256))",
        replacement: "(byte =? 256)",
        text: "\"a\tb\"",
        accepted: false,
    },
    Plant {
        label: "the elements of an array come out reversed",
        member: "readJson",
        site: "(ctor Json::Json.JArr (reverseSpine acc ((prim length text : Nat) + 1)))",
        replacement: "(ctor Json::Json.JArr acc)",
        text: "[1,2]",
        accepted: true,
    },
    Plant {
        label: "the final line feed stays in the payload",
        member: "readJsonFile",
        site: "(readValue (Text::sliceOr text 0 (prim subtract size 1 : Nat)))",
        replacement: "(readValue text)",
        text: "1\n",
        accepted: true,
    },
    Plant {
        label: "a newline escape reads as a carriage return",
        member: "readJson",
        site: "(if (letter =? 110) then 0x0a",
        replacement: "(if (letter =? 110) then 0x0d",
        text: "\"\\n\"",
        accepted: true,
    },
];

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

/// Run the case for `SL-13`.
///
/// # Panics
///
/// Panics when the case's assertion fails.
#[allow(clippy::too_many_lines)]
pub fn run(id: &str) {
    assert_eq!(id, "SL-13", "the case of {MODULE}");
    let project = P::stdlib();
    project.check_ok();
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    let report: Observed = serde_json::from_slice(
        &std::fs::read(
            project
                .build_dir(&build_id)
                .join("production/LexLeanStd/JsonRead.eligibility.json")
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
        .prefix("lexlean-json-read-")
        .tempdir()
        .expect("tempdir");
    let mut crates = Vec::new();
    let mut expected: BTreeMap<String, Vec<Observed>> = BTreeMap::new();
    for (member, _) in MEMBERS {
        let low = &lowered[member];
        let manifest = rust_packages::manifest(
            &format!("json_read_{}", snake(member)),
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
    // The gate is armed: each plant breaks one rule, and the vector of its
    // own, on which the oracle and the committed module agree, catches it.
    let mut missed = Vec::new();
    for plant in &PLANTS {
        let arguments = vec![bytes(plant.text.as_bytes())];
        let expected = outcome(plant.member, plant.text.as_bytes());
        let oracle_reads = if plant.member == "readJsonFile" {
            read_file(plant.text.as_bytes()).is_some()
        } else {
            read(plant.text.as_bytes()).is_some()
        };
        assert_eq!(
            oracle_reads, plant.accepted,
            "{}: the oracle reads the vector as the plant states",
            plant.label
        );
        assert_eq!(
            observe(&lowered[plant.member], &arguments),
            expected,
            "{}: the committed module agrees with the vector of {}",
            plant.label,
            plant.member
        );
        let planted = P::stdlib();
        let source = planted.read_raw("src/JsonRead.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/JsonRead.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let broken = lower_in(&checked, &planted, MODULE, plant.member);
        let seen = observe(&broken, &arguments);
        if seen == expected {
            missed.push(plant.label);
        } else {
            eprintln!(
                "SL-13: plant detected: {}: {seen} != {expected}",
                plant.label
            );
        }
    }
    assert!(
        missed.is_empty(),
        "planted defects the fixed vectors do not detect: {missed:?}"
    );
    // The measured stack bound of every package comes last, so that a bound not yet
    // committed hides none of the results above.
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
        // the `JsonRead` members.
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
