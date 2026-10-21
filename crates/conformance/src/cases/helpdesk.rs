//! The `helpdesk` example (SPEC.md §17.22, §10.5, `EX-09`): a text
//! application written entirely from language-1.2 source and the source
//! packages `std`, `std-models`, and `std-reasoning`, whose production roots
//! use the model alone, the reasoner alone, and both together.

use lexlean::calculus::{interp, Value};
use lexlean::production::lower::roots;
use serde_json::Value as Json;

use super::stdlib::lower_in;
use crate::production_export as harness;
use crate::support::{self, P};

/// The projects the example needs beside itself, by repository-relative path.
const MEMBERS: [&str; 4] = ["examples/helpdesk", "std-models", "std-reasoning", "stdlib"];
/// The example's directory.
const EXAMPLE: &str = "examples/helpdesk";
/// The evaluator's fuel; a root that exhausts it fails the case.
const FUEL: u64 = 50_000_000;
/// Requests per root under the evaluator.
const RUNS: usize = 150;

/// The abstention threshold of `Main.threshold`.
const THRESHOLD: i64 = 2;
/// The diagonal of the `topic-weights` artifact: what a mention adds to the
/// score of each topic.
const WEIGHTS: [i64; 3] = [4, 3, 2];
/// The keywords of each topic: the features of `Router.features`.
const KEYWORDS: [[&str; 2]; 3] = [
    ["down", "outage"],
    ["invoice", "refund"],
    ["password", "login"],
];
/// The topic names of `Reply.topicName`.
const NAMES: [&str; 3] = ["outage", "billing", "access"];
/// The steps `Drain` takes to clear the work of each topic (`Reply.workFor`
/// is 3, 2, 1): a tick while more than two remain, then one burn.
const STEPS: [usize; 3] = [2, 1, 1];

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

/// The pieces a request is drawn from: the keywords, fillers, and the white
/// space `trim` removes.
const PIECES: &[&str] = &[
    "down", "outage", "invoice", "refund", "password", "login", "hello", "please", "help", " ",
    "  ", "\t", "\n", "\u{e9}", "", "site", "is",
];

fn request(seeded: &mut Seeded) -> Vec<u8> {
    let count = seeded.below(7);
    (0..count)
        .map(|_| PIECES[seeded.below(PIECES.len())])
        .collect::<Vec<_>>()
        .join(" ")
        .into_bytes()
}

/// The independent oracle: the function the example is specified to compute,
/// written without the example's source. `None` is the refusal of a blank
/// request (the model's precondition); `Some(None)` is an abstention.
fn decide(request: &[u8]) -> Option<Option<usize>> {
    let text = request.trim_ascii();
    if text.is_empty() {
        return None;
    }
    let mentions = |needle: &str| {
        let needle = needle.as_bytes();
        text.windows(needle.len()).any(|window| window == needle)
    };
    let scores: Vec<i64> = KEYWORDS
        .iter()
        .zip(WEIGHTS)
        .map(|(words, weight)| {
            if words.iter().any(|word| mentions(word)) {
                weight
            } else {
                0
            }
        })
        .collect();
    let best = scores.iter().enumerate().fold(
        0,
        |best, (index, score)| if scores[best] < *score { index } else { best },
    );
    let mut ordered = scores.clone();
    ordered.sort_unstable_by(|left, right| right.cmp(left));
    Some((ordered[0] - ordered[1] >= THRESHOLD).then_some(best))
}

fn classify(request: &[u8]) -> usize {
    match decide(request) {
        None => 9,
        Some(None) => 8,
        Some(Some(index)) => index,
    }
}

fn dispatch(request: &[u8]) -> Vec<u8> {
    match decide(request) {
        None => b"refused".to_vec(),
        Some(None) => b"human".to_vec(),
        Some(Some(index)) => format!("{}:{}", NAMES[index], STEPS[index]).into_bytes(),
    }
}

fn plan(code: usize) -> usize {
    STEPS[code.min(2)]
}

fn bytes(value: &[u8]) -> Value {
    Value::Bytes {
        hex: value.iter().map(|byte| format!("{byte:02x}")).collect(),
    }
}

fn nat(value: usize) -> Value {
    Value::Nat {
        value: value.to_string(),
    }
}

/// Every run of `corpus` on which the lowered root and the oracle disagree.
fn disagreements(
    lowered: &lexlean::production::lower::Lowered,
    corpus: &[(Vec<Value>, Value)],
) -> Vec<String> {
    corpus
        .iter()
        .filter_map(|(arguments, expected)| {
            let outcome = interp::run(&lowered.program, FUEL, lowered.entry(), arguments);
            let observed = lexlean::calculus::rust::observable(&outcome)
                .unwrap_or_else(|| panic!("on {arguments:?}: {outcome:?}"));
            let expected = serde_json::to_value(expected).expect("a value is JSON");
            (observed != expected).then(|| format!("on {arguments:?}: {observed} != {expected}"))
        })
        .collect()
}

fn source_tree_files(directory: &str) -> Vec<std::path::PathBuf> {
    walkdir::WalkDir::new(support::repo_root().join(directory).as_std_path())
        .into_iter()
        .flatten()
        .filter(|entry| entry.file_type().is_file())
        .map(walkdir::DirEntry::into_path)
        .collect()
}

fn read(path: &str) -> String {
    std::fs::read_to_string(support::repo_root().join(path).as_std_path())
        .unwrap_or_else(|error| panic!("{path}: {error}"))
}

/// A copy of the example beside its packages, edited by raw bytes in the
/// workspace: `relative` is workspace-relative.
fn planted(relative: &str, from: &str, to: &str) -> P {
    let project = P::workspace_project(EXAMPLE, &MEMBERS);
    let path = project.workspace_dir().join(relative);
    let text = std::fs::read_to_string(path.as_std_path()).expect("read");
    assert!(text.contains(from), "{relative} does not contain {from:?}");
    std::fs::write(path.as_std_path(), text.replacen(from, to, 1)).expect("write");
    project
}

/// Refused with `code` before any backend runs.
fn refused(project: &P, code: &str) {
    project.check_fails_with(code);
    for output in [".lexlean/build", ".lexlean/verified"] {
        assert!(
            !project.root.join(output).as_std_path().exists(),
            "{code}: `{output}` was published"
        );
    }
}

/// Run the case for `EX-09`.
///
/// # Panics
///
/// Panics when the case's assertion fails.
pub(super) fn run(id: &str) {
    assert_eq!(id, "EX-09");

    // The manifest pins the three packages and two content-addressed
    // artifacts; the lock carries one row for each package.
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
        [
            ("std", "stdlib"),
            ("std-models", "std-models"),
            ("std-reasoning", "std-reasoning")
        ],
        "the example declares the three packages"
    );
    assert_eq!(config.artifact_sources.len(), 2, "two artifacts");
    let lock = read(&format!("{EXAMPLE}/lexlean.lock"));
    let rows: Vec<&str> = lock
        .split("[[source_package]]")
        .skip(1)
        .map(|row| {
            row.lines()
                .find_map(|line| line.strip_prefix("id = \""))
                .expect("a row has an id")
                .trim_end_matches('"')
        })
        .collect();
    assert_eq!(rows, ["std", "std-models", "std-reasoning"], "the pins");

    // No handwritten Lean or Rust provides behavior: the example's files are
    // LexLean source, the manifest and lock, the lake host, two artifacts, and
    // the glossary of titles.
    for file in source_tree_files(EXAMPLE) {
        let relative = file.to_string_lossy().replace('\\', "/");
        if relative.contains("/expected/") || relative.contains("/.lexlean/") {
            continue;
        }
        assert!(
            file.extension()
                .is_none_or(|extension| extension != "lean" && extension != "rs"),
            "{relative} is handwritten Lean or Rust"
        );
    }

    // The model and the reasoner are each used alone, and together: three
    // production roots, over one linked project of six modules.
    let project = P::workspace_project(EXAMPLE, &MEMBERS);
    project.fmt_check_ok();
    let (exit, _, stderr) = project.cli(&["lock", "--check"]);
    assert_eq!(exit, 0, "the committed lock is current: {stderr}");
    let checked = support::checked_project(&project);
    assert_eq!(
        checked.modules.keys().cloned().collect::<Vec<_>>(),
        [
            "LexLeanStd.Text",
            "LexLeanStdModels.Scores",
            "LexLeanStdReasoning.Worklist",
            "Main",
            "Reply",
            "Router"
        ],
        "the example links its three modules and one module of each package"
    );
    let mut found: Vec<String> = roots(&checked)
        .expect("roots")
        .iter()
        .map(|root| root.report.root.clone())
        .collect();
    found.sort();
    assert_eq!(
        found,
        [
            "Helpdesk.Main.classify",
            "Helpdesk.Main.dispatch",
            "Helpdesk.Main.plan"
        ],
        "the packages' own roots are not the example's"
    );
    let classify_program = lower_in(&checked, &project, "Helpdesk.Main", "classify");
    let plan_program = lower_in(&checked, &project, "Helpdesk.Main", "plan");
    let dispatch_program = lower_in(&checked, &project, "Helpdesk.Main", "dispatch");
    // What each root's closure reaches, by the declarations it realizes.
    let closure = |root: &str, prefix: &str| -> bool {
        roots(&checked)
            .expect("roots")
            .iter()
            .find(|found| found.report.root == root)
            .unwrap_or_else(|| panic!("`{root}` is a production root"))
            .report
            .runtime
            .iter()
            .any(|member| member.declaration.starts_with(prefix))
    };
    let models = "LexLeanStdModels.Scores.";
    let reasoning = "LexLeanStdReasoning.Worklist.";
    assert!(
        closure("Helpdesk.Main.classify", models) && !closure("Helpdesk.Main.classify", reasoning),
        "`classify` uses the model and not the reasoner"
    );
    assert!(
        closure("Helpdesk.Main.plan", reasoning) && !closure("Helpdesk.Main.plan", models),
        "`plan` uses the reasoner and not the model"
    );
    assert!(
        closure("Helpdesk.Main.dispatch", models) && closure("Helpdesk.Main.dispatch", reasoning),
        "`dispatch` uses both"
    );

    // The differential: each root agrees with the oracle on a seeded corpus
    // and on the edge requests.
    let mut seeded = Seeded(0x9e37_79b9_7f4a_7c15);
    let mut requests: Vec<Vec<u8>> = [
        "",
        "   ",
        "my site is down",
        "  refund my invoice  ",
        "forgot password",
        "invoice down",
        "hello",
        "login login",
        "outage\n",
    ]
    .iter()
    .map(|text| text.as_bytes().to_vec())
    .collect();
    requests.extend((0..RUNS).map(|_| request(&mut seeded)));
    let classify_corpus: Vec<_> = requests
        .iter()
        .map(|request| (vec![bytes(request)], nat(classify(request))))
        .collect();
    let dispatch_corpus: Vec<_> = requests
        .iter()
        .map(|request| (vec![bytes(request)], bytes(&dispatch(request))))
        .collect();
    let plan_corpus: Vec<_> = (0..6)
        .map(|code| (vec![nat(code)], nat(plan(code))))
        .collect();
    for (name, lowered, corpus) in [
        ("classify", &classify_program, &classify_corpus),
        ("dispatch", &dispatch_program, &dispatch_corpus),
        ("plan", &plan_program, &plan_corpus),
    ] {
        let wrong = disagreements(lowered, corpus);
        assert!(
            wrong.is_empty(),
            "{name} disagrees with the oracle: {wrong:?}"
        );
    }
    // The corpus reaches every outcome: a refusal, an abstention, and each
    // topic.
    let outcomes: std::collections::BTreeSet<usize> =
        requests.iter().map(|request| classify(request)).collect();
    assert_eq!(
        outcomes,
        [0, 1, 2, 8, 9].into_iter().collect(),
        "the corpus reaches every outcome"
    );

    // The gate is armed: an oracle that treats a tie as a decision disagrees
    // with the lowered `classify`.
    let tie = (vec![bytes(b"invoice down")], nat(0));
    assert!(
        !disagreements(&classify_program, &[tie]).is_empty(),
        "a tie is an abstention, not a decision"
    );

    // Each plant is refused with exactly its code, before any backend runs.
    // A changed byte of a package, and a changed version of one.
    let changed_byte = planted(
        "std-models/src/Scores.lex.tex",
        "name: margin,",
        "name: margin2,",
    );
    refused(&changed_byte, "LLC0105");
    let changed_version = planted(
        "examples/helpdesk/lexlean.toml",
        "package = \"std-models\"\nversion = \"1.0.0\"",
        "package = \"std-models\"\nversion = \"1.0.1\"",
    );
    refused(&changed_version, "LLC0105");
    // An import of a module of a package the importing scope does not
    // declare, though the closure holds it: `std-reasoning` declares
    // `std-models`, and the example does not.
    let undeclared = planted(
        "examples/helpdesk/lexlean.toml",
        "[[source_package]]\npackage = \"std-models\"\nversion = \"1.0.0\"\nkind = \"path\"\npath = \"std-models\"\n\n",
        "",
    );
    let reasoning = undeclared
        .workspace_dir()
        .join("std-reasoning/lexlean.toml");
    let manifest = std::fs::read_to_string(reasoning.as_std_path()).expect("read");
    std::fs::write(
        reasoning.as_std_path(),
        manifest.replacen(
            "\n[limits]",
            "\n[[source_package]]\npackage = \"std-models\"\nversion = \"1.0.0\"\nkind = \"path\"\npath = \"std-models\"\n\n[limits]",
            1,
        ),
    )
    .expect("write");
    undeclared.relock();
    refused(&undeclared, "LLR3009");
    // A tampered artifact.
    let tampered = P::workspace_project(EXAMPLE, &MEMBERS);
    let weights = tampered.root.join("artifacts/topic-weights.bin");
    let mut content = std::fs::read(weights.as_std_path()).expect("artifact");
    content[0] ^= 1;
    std::fs::write(weights.as_std_path(), content).expect("write");
    refused(&tampered, "LLR3007");
    // A model applied without its required check.
    let unchecked = planted(
        "examples/helpdesk/src/Main.lex.tex",
        "(checked Router::ScoringModel (LexLeanStd.Text::trim request) with [precondition])",
        "(checked Router::ScoringModel (LexLeanStd.Text::trim request) with [])",
    );
    refused(&unchecked, "LLT4008");

    if !support::lean_backed("EX-09") {
        return;
    }
    // The example verifies under the pinned Lean offline; the modules of the
    // packages are audited under their prefixes with the packages as their
    // origin.
    let verified = support::verify_ok(&project);
    let attestation: Json = serde_json::from_slice(
        &std::fs::read(
            project
                .root
                .join(".lexlean/verified")
                .join(verified.attestation_id.to_hex())
                .join("attestation.json")
                .as_std_path(),
        )
        .expect("attestation"),
    )
    .expect("the attestation is JSON");
    assert_eq!(
        attestation["origins"],
        serde_json::json!({
            "LexLeanStd.Text": "std",
            "LexLeanStdModels.Scores": "std-models",
            "LexLeanStdReasoning.Worklist": "std-reasoning"
        }),
        "the modules of a package carry the package as their origin"
    );
    let audited: Vec<&str> = attestation["declarations"]
        .as_array()
        .expect("declarations")
        .iter()
        .filter_map(|row| row["name"].as_str())
        .collect();
    for name in [
        "Helpdesk.Main.classify",
        "Helpdesk.Main.dispatch",
        "Helpdesk.Main.plan",
        "Helpdesk.Router.ScoringModel",
        "LexLeanStdModels.Scores.GateModel",
        "LexLeanStdReasoning.Worklist.Drain",
    ] {
        assert!(audited.contains(&name), "`{name}` is audited");
    }

    // The production export: every package is the committed one and realizes
    // the program the differential ran.
    let exported = harness::exported("helpdesk");
    let bundle = exported.bundle();
    let packages = harness::tree(&bundle.join("packages"));
    let committed = harness::tree(&harness::fixture_dir("helpdesk").join("packages"));
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
            .strip_prefix("Helpdesk.Main.")
            .unwrap_or_else(|| panic!("`{}` is a root of the example", root.root));
        let lowered = match member {
            "classify" => &classify_program,
            "plan" => &plan_program,
            "dispatch" => &dispatch_program,
            other => panic!("`{other}` is not a root of the example"),
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
                serde_json::to_value(&lowered.program).expect("a program is JSON"),
                "{member}: the exported package realizes the program the differential ran"
            );
            realized += 1;
        }
    }
    assert_eq!(realized, 3, "one certified package per root");
}
