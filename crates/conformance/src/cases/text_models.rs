//! The `text-models` example (SPEC.md §17.12, §17.13, `LM-10`..`LM-12`): a
//! tokenizer, windows, generators, and a classifier written in language-1.2
//! source over the source package `std`, whose production roots are text
//! components.

use lexlean::calculus::{interp, Value};
use lexlean::production::lower::roots;
use serde_json::{json, Value as Json};

use super::stdlib::lower_in;
use crate::support::{self, P};

/// The projects the example needs beside itself, by repository-relative path.
const MEMBERS: [&str; 2] = ["examples/text-models", "stdlib"];
/// The example's directory.
const EXAMPLE: &str = "examples/text-models";
/// The example's main module, by Lean name.
const MODULE: &str = "TextModels.Main";
/// The evaluator's fuel; a root that exhausts it fails the case.
const FUEL: u64 = 500_000_000;
/// The evaluator thread's stack, in bytes.
const EVALUATOR_STACK: usize = 1 << 29;

/// The roots of the example, in the order the report lists them.
const ROOTS: [&str; 9] = [
    "admitStrict",
    "continueDrawn",
    "continueGreedy",
    "detokenize",
    "keepRecent",
    "letters",
    "shapeOf",
    "tokenize",
    "topicOf",
];

/// The pieces of the `pieces` artifact: the vocabulary beyond the 256 bytes.
const PIECES: [&str; 6] = ["the", " cat", " sat", " on", " mat", "."];
/// The token that ends a generation, the last piece.
const STOP: u32 = 261;
/// The size of the vocabulary: 256 bytes and the pieces.
const SIZE: u32 = 262;
/// The window's capacity in tokens.
const CAPACITY: usize = 6;
/// The tokens a generator may emit.
const MAX_TOKENS: usize = 5;
/// What the scorer adds to each piece, in vocabulary order; a byte scores 0.
const WEIGHTS: [u64; 6] = [5, 4, 3, 2, 6, 1];

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
}

/// The independent oracle: what the example is specified to compute, written
/// without the example's source.
mod oracle {
    use super::{CAPACITY, MAX_TOKENS, PIECES, SIZE, STOP, WEIGHTS};

    /// The longest vocabulary piece at the front of `data`, if any.
    fn longest(data: &[u8]) -> Option<(u32, usize)> {
        let mut best: Option<(u32, usize)> = None;
        for (index, piece) in PIECES.iter().enumerate() {
            let piece = piece.as_bytes();
            if data.starts_with(piece) && best.is_none_or(|(_, length)| length < piece.len()) {
                best = Some((256 + u32::try_from(index).expect("an index"), piece.len()));
            }
        }
        best
    }

    /// Longest-match tokenization of text: a piece where one matches, the
    /// byte otherwise.
    pub fn words(text: &str) -> Vec<u32> {
        let mut data = text.as_bytes();
        let mut tokens = Vec::new();
        while let Some((&first, rest)) = data.split_first() {
            match longest(data) {
                Some((token, length)) => {
                    tokens.push(token);
                    data = &data[length..];
                }
                None => {
                    tokens.push(u32::from(first));
                    data = rest;
                }
            }
        }
        tokens
    }

    /// Byte-level tokenization.
    pub fn letters(text: &str) -> Vec<u32> {
        text.bytes().map(u32::from).collect()
    }

    /// The text of tokens, or nothing when a token is outside the vocabulary
    /// or the bytes are not UTF-8.
    pub fn text(tokens: &[u32]) -> Option<String> {
        let mut data = Vec::new();
        for token in tokens {
            match *token {
                byte if byte < 256 => data.push(u8::try_from(byte).expect("a byte")),
                piece if piece < SIZE => {
                    data.extend(PIECES[usize::try_from(piece - 256).expect("a piece")].bytes());
                }
                _ => return None,
            }
        }
        String::from_utf8(data).ok()
    }

    /// The window that keeps the last tokens.
    pub fn keep_recent(tokens: &[u32]) -> Vec<u32> {
        tokens[tokens.len().saturating_sub(CAPACITY)..].to_vec()
    }

    /// The window that refuses what does not fit.
    pub fn admit_strict(tokens: &[u32]) -> Option<Vec<u32>> {
        (tokens.len() <= CAPACITY).then(|| tokens.to_vec())
    }

    /// The label of text: how many distinct tokens it holds.
    pub fn topic(text: &str) -> usize {
        let mut seen = words(text);
        seen.sort_unstable();
        seen.dedup();
        seen.len()
    }

    /// The label of text under the byte-level tokenizer: how many distinct
    /// bytes it holds.
    pub fn shape(text: &str) -> usize {
        let mut seen = letters(text);
        seen.sort_unstable();
        seen.dedup();
        seen.len()
    }

    /// One step of the 32-bit xorshift generator.
    pub fn xorshift(mut state: u32) -> u32 {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        state
    }

    /// The weight the scorer gives a token.
    fn weight(token: usize) -> u64 {
        token.checked_sub(256).map_or(0, |piece| WEIGHTS[piece])
    }

    /// A generation from `prompt`: the tokens emitted, whether the stop token
    /// ended it, and the seed after the draws. `seed` is `None` for greedy
    /// decoding.
    pub fn generate(seed: Option<u32>) -> (Vec<u32>, bool, u32) {
        let total: u64 = WEIGHTS.iter().sum();
        let mut state = seed.unwrap_or(0);
        let mut out = Vec::new();
        for _ in 0..MAX_TOKENS {
            let token = if seed.is_some() {
                state = xorshift(state);
                let mut draw = u64::from(state) % total;
                (0..usize::try_from(SIZE).expect("a size"))
                    .find(|&token| {
                        let weight = weight(token);
                        if draw < weight {
                            true
                        } else {
                            draw -= weight;
                            false
                        }
                    })
                    .expect("a draw is below the total")
            } else {
                (0..usize::try_from(SIZE).expect("a size")).fold(0, |best, token| {
                    if weight(best) < weight(token) {
                        token
                    } else {
                        best
                    }
                })
            };
            if token == usize::try_from(STOP).expect("a stop") {
                return (out, true, state);
            }
            out.push(u32::try_from(token).expect("a token"));
        }
        (out, false, state)
    }
}

fn nat(value: usize) -> Value {
    Value::Nat {
        value: value.to_string(),
    }
}

fn string(value: &str) -> Value {
    Value::String {
        value: value.to_owned(),
    }
}

fn token(value: u32) -> Value {
    Value::U32 {
        value: value.to_string(),
    }
}

fn tokens(values: &[u32]) -> Value {
    Value::List {
        items: values.iter().copied().map(token).collect(),
    }
}

fn some(value: Value) -> Value {
    Value::Some {
        value: Box::new(value),
    }
}

fn option(value: Option<Value>) -> Value {
    value.map_or(Value::None, some)
}

/// The generator's end: the constructors are declared `stop`, `exhausted`.
fn ended(stopped: bool) -> Value {
    Value::Adt {
        constructor: u64::from(!stopped),
        fields: Vec::new(),
    }
}

fn pair(left: Value, right: Value) -> Value {
    Value::Pair {
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn ok(value: Value) -> Value {
    Value::Ok {
        value: Box::new(value),
    }
}

/// The oracle's answer for `root` on `arguments`.
fn expected(root: &str, arguments: &Arguments) -> Value {
    match (root, arguments) {
        ("tokenize", Arguments::Text(text)) => tokens(&oracle::words(text)),
        ("letters", Arguments::Text(text)) => tokens(&oracle::letters(text)),
        ("topicOf", Arguments::Text(text)) => nat(oracle::topic(text)),
        ("shapeOf", Arguments::Text(text)) => nat(oracle::shape(text)),
        ("detokenize", Arguments::Tokens(ids)) => option(oracle::text(ids).map(|t| string(&t))),
        ("keepRecent", Arguments::Tokens(ids)) => some(tokens(&oracle::keep_recent(ids))),
        ("admitStrict", Arguments::Tokens(ids)) => {
            option(oracle::admit_strict(ids).map(|kept| tokens(&kept)))
        }
        ("continueGreedy", Arguments::Text(_)) => {
            let (out, stopped, _) = oracle::generate(None);
            ok(pair(
                string(&oracle::text(&out).expect("generated text")),
                ended(stopped),
            ))
        }
        ("continueDrawn", Arguments::Drawn(_, seed)) => {
            let (out, stopped, next) = oracle::generate(Some(*seed));
            ok(pair(
                pair(
                    string(&oracle::text(&out).expect("generated text")),
                    ended(stopped),
                ),
                token(next),
            ))
        }
        other => panic!("no oracle for {other:?}"),
    }
}

/// What a root is applied to.
#[derive(Debug, Clone)]
enum Arguments {
    Text(String),
    Tokens(Vec<u32>),
    Drawn(String, u32),
}

impl Arguments {
    fn values(&self) -> Vec<Value> {
        match self {
            Self::Text(text) => vec![string(text)],
            Self::Tokens(ids) => vec![tokens(ids)],
            Self::Drawn(text, seed) => vec![string(text), token(*seed)],
        }
    }
}

/// Text drawn from the vocabulary's pieces, letters, white space, and
/// characters that take more than one byte.
const FRAGMENTS: &[&str] = &[
    "the", " cat", " sat", " on", " mat", ".", "th", "ca", " ", "  ", "a", "z", "\u{e9}",
    "\u{4e16}", "\n", "", "mat", "on",
];

fn text(seeded: &mut Seeded) -> String {
    let count = seeded.below(9);
    (0..count)
        .map(|_| FRAGMENTS[seeded.below(FRAGMENTS.len())])
        .collect()
}

/// Token lists of every length around the window, and ids across the
/// vocabulary's boundary.
fn token_list(seeded: &mut Seeded) -> Vec<u32> {
    let count = seeded.below(10);
    (0..count)
        .map(|_| match seeded.below(4) {
            0 => u32::try_from(seeded.below(256)).expect("a byte"),
            1 => 256 + u32::try_from(seeded.below(6)).expect("a piece"),
            2 => SIZE + u32::try_from(seeded.below(3)).expect("a token past the vocabulary"),
            _ => u32::try_from(seeded.below(128)).expect("an ASCII byte"),
        })
        .collect()
}

/// The runs of the differential: every root, on the edges and a seeded
/// corpus.
fn corpus() -> Vec<(&'static str, Arguments)> {
    let mut seeded = Seeded(0x7e47_5eed_0a11_c0de);
    let mut runs = Vec::new();
    let texts: Vec<String> = [
        "",
        "the cat",
        "the cat sat on the mat.",
        "th",
        " cat",
        "\u{e9}\u{4e16}",
        "mat mat mat",
    ]
    .iter()
    .map(|text| (*text).to_owned())
    .chain((0..40).map(|_| text(&mut seeded)))
    .collect();
    let lists: Vec<Vec<u32>> = [
        vec![],
        vec![256, 257],
        vec![1, 2, 3, 4, 5, 6],
        vec![1, 2, 3, 4, 5, 6, 7],
        vec![262],
        vec![0xc3],
        vec![0xc3, 0xa9],
    ]
    .into_iter()
    .chain((0..40).map(|_| token_list(&mut seeded)))
    .collect();
    for text in &texts {
        for root in [
            "tokenize",
            "letters",
            "topicOf",
            "shapeOf",
            "continueGreedy",
        ] {
            runs.push((root, Arguments::Text(text.clone())));
        }
        for seed in [0, 1, 5, 0xffff_ffff] {
            runs.push(("continueDrawn", Arguments::Drawn(text.clone(), seed)));
        }
        let seed = u32::try_from(seeded.next() & 0xffff_ffff).expect("a seed");
        runs.push(("continueDrawn", Arguments::Drawn(text.clone(), seed)));
    }
    for ids in &lists {
        for root in ["detokenize", "keepRecent", "admitStrict"] {
            runs.push((root, Arguments::Tokens(ids.clone())));
        }
    }
    runs
}

fn read(path: &str) -> String {
    std::fs::read_to_string(support::repo_root().join(path).as_std_path())
        .unwrap_or_else(|error| panic!("{path}: {error}"))
}

/// A copy of the example beside the standard library, locked.
fn project() -> P {
    let project = P::workspace_project(EXAMPLE, &MEMBERS);
    project.relock();
    project
}

/// A copy of the example edited by raw bytes: `relative` is
/// workspace-relative.
fn planted(relative: &str, from: &str, to: &str) -> P {
    let project = P::workspace_project(EXAMPLE, &MEMBERS);
    let path = project.workspace_dir().join(relative);
    let text = std::fs::read_to_string(path.as_std_path()).expect("read");
    assert!(text.contains(from), "{relative} does not contain {from:?}");
    std::fs::write(path.as_std_path(), text.replacen(from, to, 1)).expect("write");
    project.relock();
    project
}

/// Every run of the differential on which a lowered root and the oracle
/// disagree.
fn disagreements(
    checked: &lexlean::link::CheckedProject,
    project: &P,
    runs: &[(&'static str, Arguments)],
) -> Vec<String> {
    // The evaluator recurses as deeply as the lowered program does.
    std::thread::scope(|scope| {
        std::thread::Builder::new()
            .stack_size(EVALUATOR_STACK)
            .spawn_scoped(scope, || differ(checked, project, runs))
            .expect("the evaluator thread starts")
            .join()
            .expect("the evaluator finishes")
    })
}

fn differ(
    checked: &lexlean::link::CheckedProject,
    project: &P,
    runs: &[(&'static str, Arguments)],
) -> Vec<String> {
    let mut wrong = Vec::new();
    for root in ROOTS {
        let lowered = lower_in(checked, project, MODULE, root);
        for (name, arguments) in runs.iter().filter(|(name, _)| *name == root) {
            let outcome = interp::run(&lowered.program, FUEL, lowered.entry(), &arguments.values());
            let observed = lexlean::calculus::rust::observable(&outcome)
                .unwrap_or_else(|| panic!("{name} on {arguments:?}: {outcome:?}"));
            let wanted = serde_json::to_value(expected(name, arguments)).expect("a value is JSON");
            if observed != wanted {
                wrong.push(format!("{name} on {arguments:?}: {observed} != {wanted}"));
            }
        }
    }
    wrong
}

/// The eligibility rows of the example, by root.
fn text_rows(checked: &lexlean::link::CheckedProject) -> std::collections::BTreeMap<String, Json> {
    roots(checked)
        .expect("roots")
        .iter()
        .map(|root| {
            let value = root.report.to_value();
            (root.name.clone(), value)
        })
        .collect()
}

/// The row of `component` in a root's `text_rows`.
fn row<'a>(report: &'a Json, component: &str) -> &'a Json {
    report["text_rows"]
        .as_array()
        .unwrap_or_else(|| panic!("the report has text rows: {report}"))
        .iter()
        .find(|row| row["component"] == component)
        .unwrap_or_else(|| panic!("{component} has a row: {report}"))
}

/// Run the case for `LM-10`, `LM-11`, or `LM-12`.
///
/// # Panics
///
/// Panics when the case's assertion fails.
pub(super) fn run(id: &str) {
    match id {
        "LM-10" => lm_10(),
        "LM-11" => lm_11(),
        "LM-12" => lm_12(),
        other => panic!("no text-models case is wired for {other}"),
    }
}

/// LM-10 (§17.13): text roots are eligible for `rust-std`, refused for
/// `rust-core`, report the contract of each text component, and agree with
/// the oracle.
fn lm_10() {
    let project = project();
    let checked = support::checked_project(&project);
    let reports = text_rows(&checked);
    assert_eq!(
        reports.keys().map(String::as_str).collect::<Vec<_>>(),
        ROOTS,
        "the example's text roots"
    );
    for (name, report) in &reports {
        assert_eq!(
            report["targets"].as_array().map(|targets| targets
                .iter()
                .map(|target| target["target"].clone())
                .collect::<Vec<_>>()),
            Some(vec![json!("rust-std")]),
            "{name}: eligible for rust-std alone"
        );
        let effects: Vec<&str> = report["declared_effects"]
            .as_array()
            .expect("effects")
            .iter()
            .filter_map(Json::as_str)
            .collect();
        assert_eq!(
            effects,
            ["allocation", "overflow", "recursion"],
            "{name}: the effects a text root admits"
        );
    }

    // Each root reports the contract of the text components it runs, as
    // closed members and qualified names.
    let drawn = &reports["continueDrawn"];
    let sampler = row(drawn, "TextModels.Main.Drawn");
    assert_eq!(sampler["kind"], "text_generator");
    assert_eq!(sampler["policy"], "sample");
    assert_eq!(sampler["generator"], "xorshift32");
    assert_eq!(sampler["distribution"], "proportional");
    assert_eq!(sampler["max_tokens"], MAX_TOKENS);
    assert_eq!(
        (&sampler["seed_input"], &sampler["seed_output"]),
        (&json!("last"), &json!("second")),
        "the seed is the last input and the second result"
    );
    let greedy = row(&reports["continueGreedy"], "TextModels.Main.Greedy");
    assert_eq!(greedy["policy"], "greedy");
    for member in ["generator", "distribution", "seed_input", "seed_output"] {
        assert!(
            greedy.get(member).is_none(),
            "a greedy generator draws nothing, so it states no `{member}`: {greedy}"
        );
    }
    let window = row(drawn, "TextModels.Main.Recent");
    assert_eq!(
        (&window["kind"], &window["capacity"], &window["overflow"]),
        (
            &json!("context_window"),
            &json!(CAPACITY),
            &json!("keep_last")
        )
    );
    assert_eq!(
        row(&reports["admitStrict"], "TextModels.Main.Strict")["overflow"],
        "refuse"
    );
    assert_eq!(
        row(&reports["tokenize"], "TextModels.Main.Words")["scheme"],
        "longest_match"
    );
    assert_eq!(
        row(&reports["letters"], "TextModels.Main.Raw")["scheme"],
        "byte_level"
    );
    assert_eq!(
        row(&reports["topicOf"], "TextModels.Main.Topic")["kind"],
        "text_classifier"
    );
    // The bounds a row cites are generated theorems of the component.
    for (name, report) in &reports {
        for row in report["text_rows"].as_array().into_iter().flatten() {
            for bound in row["bounds"].as_array().expect("bounds") {
                let bound = bound.as_str().expect("a name");
                assert!(
                    bound.starts_with(MODULE) && bound.rsplit('.').next().is_some(),
                    "{name}: `{bound}` is a theorem of the module"
                );
            }
        }
    }
    // Nothing in a row is free text: a row has only closed members, numbers,
    // and qualified names.
    for report in reports.values() {
        for row in report["text_rows"].as_array().into_iter().flatten() {
            for (member, value) in row.as_object().expect("a row") {
                assert!(
                    value.as_str().is_none_or(|text| text
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '\''))),
                    "`{member}` of a row is not a closed member or a qualified name: {value}"
                );
            }
        }
    }

    // A target the closure cannot meet is refused with its code, before any
    // backend runs.
    let core = planted(
        "examples/text-models/src/Main.lex.tex",
        "name: tokenize,\n  parameters: [text : String],\n  result: (List UInt32),\n  body: (Words.encodeText text),\n  axioms: [Classical.choice, Quot.sound, propext],\n  executable: true,\n  production: {effects: [allocation, overflow, recursion], targets: [rust-std]}",
        "name: tokenize,\n  parameters: [text : String],\n  result: (List UInt32),\n  body: (Words.encodeText text),\n  axioms: [Classical.choice, Quot.sound, propext],\n  executable: true,\n  production: {effects: [allocation, overflow, recursion], targets: [rust-core]}",
    );
    core.check_fails_with("LLT4005");
    for output in [".lexlean/build", ".lexlean/verified"] {
        assert!(
            !core.root.join(output).as_std_path().exists(),
            "LLT4005: `{output}` was published"
        );
    }

    // The differential: every root agrees with the oracle on the edges and a
    // seeded corpus.
    let runs = corpus();
    let wrong = disagreements(&checked, &project, &runs);
    assert!(wrong.is_empty(), "disagreements with the oracle: {wrong:?}");
    for root in ROOTS {
        assert!(
            runs.iter().filter(|(name, _)| *name == root).count() >= 7,
            "{root}: the corpus runs it"
        );
    }
    // The gate is armed: an answer that keeps the first tokens of a window
    // disagrees with the lowered `keepRecent`.
    let observed = std::thread::scope(|scope| {
        std::thread::Builder::new()
            .stack_size(EVALUATOR_STACK)
            .spawn_scoped(scope, || {
                let lowered = lower_in(&checked, &project, MODULE, "keepRecent");
                let outcome = interp::run(
                    &lowered.program,
                    FUEL,
                    lowered.entry(),
                    &[tokens(&[1, 2, 3, 4, 5, 6, 7])],
                );
                lexlean::calculus::rust::observable(&outcome).expect("a value")
            })
            .expect("the evaluator thread starts")
            .join()
            .expect("the evaluator finishes")
    });
    assert_ne!(
        observed,
        serde_json::to_value(some(tokens(&[1, 2, 3, 4, 5, 6]))).expect("JSON"),
        "a window that keeps the first tokens is not the example's"
    );
    assert_eq!(
        observed,
        serde_json::to_value(some(tokens(&[2, 3, 4, 5, 6, 7]))).expect("JSON"),
        "the window keeps the last tokens"
    );

    if !support::lean_backed("LM-10") {
        return;
    }
    // Certificates A, B, and E of every text root are kernel-checked.
    let (_, verified) = verified_example();
    let preservation: Json = serde_json::from_slice(
        &std::fs::read(
            verified
                .root
                .join("preserve/preservation.json")
                .as_std_path(),
        )
        .expect("preservation.json"),
    )
    .expect("the certificates are JSON");
    let mut certified: Vec<String> = preservation["roots"]
        .as_array()
        .expect("roots")
        .iter()
        .map(|root| {
            assert_eq!(root["targets"], json!(["rust-std"]));
            assert!(
                root["theorem"]
                    .as_str()
                    .is_some_and(|theorem| theorem.ends_with(".root")),
                "{root}: a certificate states the root's theorem"
            );
            let renderings = root["renderings"].as_array().expect("renderings");
            assert_eq!(renderings.len(), 1, "one rendering for one target");
            assert!(
                renderings[0]["composed"]["theorem"].is_string(),
                "{root}: the composed certificate E is present"
            );
            root["root"].as_str().expect("a root").to_owned()
        })
        .collect();
    certified.sort();
    let mut wanted: Vec<String> = ROOTS
        .iter()
        .map(|root| format!("{MODULE}.{root}"))
        .collect();
    wanted.sort();
    assert_eq!(certified, wanted, "every text root is certified");
}

/// The example verified once for the cases that read its attestation.
fn verified_example() -> &'static (P, lexlean::VerifiedProject) {
    static VERIFIED: std::sync::OnceLock<(P, lexlean::VerifiedProject)> =
        std::sync::OnceLock::new();
    VERIFIED.get_or_init(|| {
        let project = project();
        let verified = support::verify_ok(&project);
        (project, verified)
    })
}

/// LM-11 (§17.22): the example is written from language-1.2 source alone,
/// verifies end to end under the pinned Lean, and carries its records; its
/// negatives and a planted mutation are refused.
fn lm_11() {
    // The manifest pins the one package and the one content-addressed
    // artifact.
    let config = lexlean::config::parse_project(
        "lexlean.toml",
        read(&format!("{EXAMPLE}/lexlean.toml")).as_bytes(),
    )
    .expect("the manifest parses");
    assert_eq!(
        config
            .source_packages
            .iter()
            .map(|package| (package.package.as_str(), package.path.as_str()))
            .collect::<Vec<_>>(),
        [("std", "stdlib")],
        "the example declares the standard library alone"
    );
    assert_eq!(config.artifact_sources.len(), 1, "one vocabulary artifact");
    // No handwritten Lean or Rust provides behavior.
    for file in walkdir::WalkDir::new(support::repo_root().join(EXAMPLE).as_std_path())
        .into_iter()
        .flatten()
        .filter(|entry| entry.file_type().is_file())
    {
        let relative = file.path().to_string_lossy().replace('\\', "/");
        if relative.contains("/expected/") || relative.contains("/.lexlean/") {
            continue;
        }
        assert!(
            file.path()
                .extension()
                .is_none_or(|extension| extension != "lean" && extension != "rs"),
            "{relative} is handwritten Lean or Rust"
        );
    }
    // The committed lock and the records of a build and a verification.
    let committed = P::workspace_project(EXAMPLE, &MEMBERS);
    let (exit, _, stderr) = committed.cli(&["lock", "--check"]);
    assert_eq!(exit, 0, "the committed lock is current: {stderr}");
    for records in [
        "expected/build/manifest.json",
        "expected/verify/audit/output.txt",
    ] {
        assert!(
            support::repo_root()
                .join(EXAMPLE)
                .join(records)
                .as_std_path()
                .is_file(),
            "the example carries `{records}`"
        );
    }
    committed.fmt_check_ok();

    // The negatives, on the lowered roots: a window that refuses what does not
    // fit, a token outside the vocabulary, and bytes that are not UTF-8.
    let checked = support::checked_project(&committed);
    let outcomes: Vec<(&str, Arguments, Value)> = vec![
        (
            "admitStrict",
            Arguments::Tokens(vec![1, 2, 3, 4, 5, 6, 7]),
            Value::None,
        ),
        (
            "admitStrict",
            Arguments::Tokens(vec![1, 2, 3]),
            some(tokens(&[1, 2, 3])),
        ),
        ("detokenize", Arguments::Tokens(vec![SIZE]), Value::None),
        ("detokenize", Arguments::Tokens(vec![0xc3]), Value::None),
        (
            "detokenize",
            Arguments::Tokens(vec![256, 257]),
            some(string("the cat")),
        ),
        (
            "tokenize",
            Arguments::Text("the cat".to_owned()),
            tokens(&[256, 257]),
        ),
        (
            "letters",
            Arguments::Text("ab".to_owned()),
            tokens(&[97, 98]),
        ),
    ];
    std::thread::scope(|scope| {
        std::thread::Builder::new()
            .stack_size(EVALUATOR_STACK)
            .spawn_scoped(scope, || {
                for (root, arguments, wanted) in &outcomes {
                    let lowered = lower_in(&checked, &committed, MODULE, root);
                    let outcome =
                        interp::run(&lowered.program, FUEL, lowered.entry(), &arguments.values());
                    assert_eq!(
                        lexlean::calculus::rust::observable(&outcome),
                        Some(serde_json::to_value(wanted).expect("JSON")),
                        "{root} on {arguments:?}"
                    );
                }
            })
            .expect("the evaluator thread starts")
            .join()
            .expect("the evaluator finishes");
    });

    // A vocabulary that is not UTF-8 is refused where the artifact is read.
    let artifact = planted(
        "examples/text-models/lexlean.toml",
        "path = \"artifacts/pieces.txt\"",
        "path = \"artifacts/pieces.txt\"",
    );
    let path = artifact.root.join("artifacts/pieces.txt");
    let bad = b"\xff\xfe\n".to_vec();
    std::fs::write(path.as_std_path(), &bad).expect("write");
    artifact.edit(
        "lexlean.toml",
        "c2a671b13cf191d6570a3a59b6b8b016ecf2ae5d5d80f247756f5887f67eb8bd",
        &lexlean::artifact::content_id::Sha256Digest::of(&bad).to_hex(),
    );
    artifact.edit("src/Main.lex.tex", "length: 25", "length: 3");
    artifact.relock();
    artifact.check_err();
    // A changed byte of the vocabulary is a digest mismatch.
    let tampered = P::workspace_project(EXAMPLE, &MEMBERS);
    let pieces = tampered.root.join("artifacts/pieces.txt");
    let mut content = std::fs::read(pieces.as_std_path()).expect("artifact");
    content[0] ^= 1;
    std::fs::write(pieces.as_std_path(), content).expect("write");
    tampered.check_fails_with("LLR3007");

    if !support::lean_backed("LM-11") {
        return;
    }
    // The example verifies under the pinned Lean offline; the production
    // forms it applies are audited as the package's.
    let (_, verified) = verified_example();
    let attestation: Json = serde_json::from_slice(
        &std::fs::read(verified.root.join("attestation.json").as_std_path()).expect("attestation"),
    )
    .expect("the attestation is JSON");
    assert_eq!(attestation["status"], "verified");
    assert_eq!(
        attestation["origins"]["LexLeanStd.TextModels"], "std",
        "the production forms carry the package as their origin"
    );
    let audited: Vec<&str> = attestation["declarations"]
        .as_array()
        .expect("declarations")
        .iter()
        .filter_map(|row| row["name"].as_str())
        .collect();
    for name in [
        "Words.encodeText_bridge",
        "Raw.encodeText_bridge",
        "Recent.push_bounded",
        "Strict.push_bounded",
        "Greedy.run_bridge",
        "Drawn.run_bridge",
        "Topic.classify_bridge",
        "window_keeps_last",
        "window_refuses",
        "greedy_continues",
        "drawn_seed_1",
        "drawn_seed_5",
        "tokenize",
        "continueDrawn",
    ] {
        let qualified = format!("{MODULE}.{name}");
        assert!(
            audited.contains(&qualified.as_str()),
            "`{qualified}` is audited: {audited:?}"
        );
    }

    // A planted mutation of the scorer changes what the generators emit, and
    // the kernel-evaluated facts the example states refuse it.
    let mutated = planted(
        "examples/text-models/src/Main.lex.tex",
        "(cons (6 : Int)",
        "(cons (0 : Int)",
    );
    let error = mutated.verify_fails_with("LLV7002");
    assert!(
        error
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("greedy_continues")
                || diagnostic.message.contains("Greedy.generateTokens")),
        "the refusal names the generator fact: {error:?}"
    );
}

/// How the text-laws library is used: the functions that generated
/// declarations apply and the lemmas that generated proofs apply, directly
/// or through the library declarations they apply.
#[derive(Debug)]
struct Coverage {
    functions: usize,
    lemmas: usize,
}

/// The audit of LM-12: every fixed name of the library is a declaration of
/// it, and is reached from the generated Lean, directly or through the
/// declarations a reached one applies. A function nothing realizes and a lemma
/// no generated proof applies is dead library, named.
fn audit_coverage(
    library: &lexlean::ir::core::CoreModule,
    fixed: &[&str],
    generated: &[String],
) -> Result<Coverage, String> {
    use lexlean::ir::core::{CoreDeclKind, CoreNode};
    let prefix = format!("{}.", lexlean::text_laws::PREFIX);
    let children = |node: &CoreNode| -> Vec<usize> {
        match node {
            CoreNode::App { f, x } => vec![*f, *x],
            CoreNode::Lambda { t, v, .. } | CoreNode::Forall { t, v, .. } => vec![*t, *v],
            CoreNode::Let { t, v, body, .. } => vec![*t, *v, *body],
            CoreNode::Mdata { x, .. } | CoreNode::Projection { x, .. } => vec![*x],
            CoreNode::BVar { .. }
            | CoreNode::Sort { .. }
            | CoreNode::Const { .. }
            | CoreNode::Nat { .. }
            | CoreNode::String { .. } => Vec::new(),
        }
    };
    // What each declaration of the library applies.
    let mut applies: std::collections::BTreeMap<&str, std::collections::BTreeSet<&str>> =
        std::collections::BTreeMap::new();
    for declaration in &library.declarations {
        let mut seen = std::collections::BTreeSet::new();
        let mut found = std::collections::BTreeSet::new();
        let mut pending: Vec<usize> = declaration
            .r#type
            .into_iter()
            .chain(declaration.value)
            .collect();
        while let Some(index) = pending.pop() {
            if !seen.insert(index) {
                continue;
            }
            if let CoreNode::Const { n, .. } = &library.nodes[index] {
                if n.starts_with(&prefix) {
                    found.insert(n.as_str());
                }
            }
            pending.extend(children(&library.nodes[index]));
        }
        applies.insert(declaration.name.as_str(), found);
    }
    for name in fixed {
        if !applies.contains_key(format!("{prefix}{name}").as_str()) {
            return Err(format!("the library does not declare `{prefix}{name}`"));
        }
    }
    // The names the generated Lean applies, whole-word.
    let mut reached: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    let mut pending: Vec<&str> = Vec::new();
    for declaration in applies.keys() {
        let short = &declaration[prefix.len()..];
        let word = |text: &String| {
            text.match_indices(*declaration).any(|(start, _)| {
                !text[start + declaration.len()..]
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_alphanumeric() || matches!(c, '_' | '\''))
            })
        };
        if !short.is_empty() && generated.iter().any(word) {
            pending.push(declaration);
        }
    }
    while let Some(name) = pending.pop() {
        if reached.insert(name) {
            pending.extend(applies[name].iter().copied());
        }
    }
    let mut coverage = Coverage {
        functions: 0,
        lemmas: 0,
    };
    for name in fixed {
        let qualified = format!("{prefix}{name}");
        let kind = &library
            .declarations
            .iter()
            .find(|declaration| declaration.name == qualified)
            .expect("declared")
            .kind;
        let lemma = matches!(kind, CoreDeclKind::Theorem);
        if !reached.contains(qualified.as_str()) {
            return Err(if lemma {
                format!("lemma `{qualified}` is applied by no generated proof and by no library declaration they apply: dead library")
            } else {
                format!("function `{qualified}` is realized by no generated declaration and applied by no library declaration they apply: dead library")
            });
        }
        if lemma {
            coverage.lemmas += 1;
        } else {
            coverage.functions += 1;
        }
    }
    Ok(coverage)
}

/// LM-12 (§17.12): every fixed helper of the text-laws library is covered.
fn lm_12() {
    let source = std::fs::read_to_string(
        support::repo_root()
            .join("language/text-1.2/library/src/LexLeanText.lex.tex")
            .as_std_path(),
    )
    .expect("the library source");
    let library =
        lexlean::ir::core::CoreModule::from_source(&source, "1.2").expect("the library parses");
    let project = project();
    let build = support::rendered(&project);
    let generated = vec![support::lean_text(&build, "Main")];
    let fixed = lexlean::text_laws::names::ALL;
    let coverage = audit_coverage(&library, &fixed, &generated)
        .unwrap_or_else(|reason| panic!("the coverage audit holds: {reason}"));
    assert!(
        coverage.functions > 0 && coverage.lemmas > 0,
        "the audit is armed: it reaches functions and lemmas ({} and {})",
        coverage.functions,
        coverage.lemmas
    );
    assert_eq!(
        coverage.functions + coverage.lemmas,
        fixed.len(),
        "every fixed name is classified"
    );

    // A name the library does not declare is caught by name.
    let mut ghost = fixed.to_vec();
    ghost.push("ghostHelper");
    let error = audit_coverage(&library, &ghost, &generated).expect_err("a ghost is caught");
    assert!(error.contains("ghostHelper"), "{error}");
    // Generated Lean that applies no library declaration leaves the first
    // fixed name dead, named.
    let error = audit_coverage(&library, &fixed, &[String::new()]).expect_err("dead is caught");
    assert!(
        error.contains("dead library") && error.contains("LexLeanText."),
        "{error}"
    );
    // A lemma whose every application is removed from the generated proofs is
    // named: the bytes law of the byte-level classifier is applied by one
    // proof only.
    let lemma = lexlean::text_laws::names::FEATURES_TOTAL_TEXT_BYTES_OF;
    let stripped: Vec<String> = generated
        .iter()
        .map(|text| text.replace(&format!("LexLeanText.{lemma}"), "LexLeanText.removed"))
        .collect();
    assert_ne!(stripped, generated, "the generated proofs apply `{lemma}`");
    let error = audit_coverage(&library, &fixed, &stripped).expect_err("an unused lemma is caught");
    assert!(
        error.contains(&format!("LexLeanText.{lemma}")) && error.contains("lemma"),
        "{error}"
    );
}
