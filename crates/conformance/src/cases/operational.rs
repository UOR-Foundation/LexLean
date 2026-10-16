//! VR-20..VR-22 (SPEC.md §22.11, §22.12): the verifier's operational width,
//! its invariance, and its telemetry.
//!
//! These cases are about how verification is run, so they check that running
//! it differently changes nothing that is kept: the same project at two widths
//! publishes the same bytes, a verification with telemetry on publishes what
//! one with it off does, and a result never depends on the order in which the
//! processes finish. Each half that needs the pinned Lean runs through the
//! host gate, and the rest runs everywhere.

use std::collections::BTreeSet;
use std::sync::mpsc;
use std::sync::Mutex;
use std::time::Duration;

use lexlean::verify::operational::{Operational, DEFAULT_WIDTH, MAX_WIDTH};
use lexlean::verify::telemetry::{LABELS, VOCABULARY};
use lexlean::{Selection, VerifyRequest};

use crate::support::{self, P};

pub(crate) fn run(id: &str) {
    match id {
        "VR-20" => width_is_operational(),
        "VR-21" => width_invariance(),
        "VR-22" => telemetry_is_noncanonical(),
        "VR-23" => parts_equal_the_whole(),
        other => panic!("no operational case for {other}"),
    }
}

fn verify_at(
    project: &P,
    operational: Operational,
) -> Result<lexlean::VerifiedProject, lexlean::LexLeanError> {
    // Shared with every other verification of the suite and exclusive of the
    // cases that replace `ELAN_HOME`: a verification that straddled one would
    // run its toolchain from two homes.
    let _environment = support::env_shared();
    operational.scoped(|| {
        project.engine().verify(VerifyRequest {
            selection: Selection::Entrypoints,
        })
    })
}

/// Every file under a verified directory, with its bytes.
fn tree(root: &camino::Utf8Path) -> std::collections::BTreeMap<String, Vec<u8>> {
    support::file_set(root)
        .into_iter()
        .map(|name| {
            let bytes =
                std::fs::read(root.join(&name).as_std_path()).expect("a verified file reads");
            (name, bytes)
        })
        .collect()
}

fn assert_trees_equal(
    left: &std::collections::BTreeMap<String, Vec<u8>>,
    right: &std::collections::BTreeMap<String, Vec<u8>>,
    what: &str,
) {
    assert_eq!(
        left.keys().collect::<Vec<_>>(),
        right.keys().collect::<Vec<_>>(),
        "{what}: the same files are published"
    );
    for (name, bytes) in left {
        assert!(
            bytes == &right[name],
            "{what}: `{name}` differs byte for byte between the two verifications"
        );
    }
}

/// VR-20: the width is an operational profile.
fn width_is_operational() {
    assert_eq!(DEFAULT_WIDTH, 1, "the conservative profile is the default");
    assert_eq!(MAX_WIDTH, 64);
    for (text, expected) in [
        (None, Some(1)),
        (Some(""), Some(1)),
        (Some("1"), Some(1)),
        (Some("2"), Some(2)),
        (Some("64"), Some(64)),
        (Some("0"), None),
        (Some("65"), None),
        (Some("+2"), None),
        (Some("2.0"), None),
        (Some("two"), None),
        (Some(" 2"), None),
    ] {
        match (Operational::parse_width(text), expected) {
            (Ok(width), Some(want)) => assert_eq!(width, want, "{text:?}"),
            (Err(error), None) => assert_eq!(error.code.as_str(), "LLV7017", "{text:?}"),
            (got, want) => panic!("{text:?}: expected {want:?}, got {got:?}"),
        }
    }

    // A value that is not Unicode is malformed, not absent: reading it as
    // unset would verify at the default width in a job that asked for another.
    #[cfg(unix)]
    let not_unicode = {
        use std::os::unix::ffi::OsStringExt;
        std::ffi::OsString::from_vec(vec![b'2', 0xff])
    };
    #[cfg(windows)]
    let not_unicode = {
        use std::os::windows::ffi::OsStringExt;
        std::ffi::OsString::from_wide(&[0x0032, 0xD800])
    };
    #[cfg(any(unix, windows))]
    assert_eq!(
        Operational::parse_width_os(Some(&not_unicode))
            .expect_err("a non-Unicode width")
            .code
            .as_str(),
        "LLV7017"
    );

    // A malformed width is refused by the command line before anything is
    // built or published, as a registered code, on every host.
    let root = support::repo_root();
    let fixture = root.join("tests/negative/invalid-verify-width");
    let observed = crate::fixtures::check_shared(&fixture).expect("the committed fixture holds");
    assert_eq!(observed.codes, ["LLV7017"]);
    assert_eq!(observed.exit, 3, "an environment-class failure");
    assert!(
        !observed.project.root.join(".lexlean").exists(),
        "a malformed width is refused before the build is published"
    );
    for bad in ["2.5", "65", "-1"] {
        let project = P::example();
        let (exit, stdout, _) = support::with_env(&[("LEXLEAN_VERIFY_WIDTH", Some(bad))], || {
            project.cli(&["--diagnostic-format", "json", "verify"])
        });
        assert_eq!(exit, 3, "{bad}");
        assert!(stdout.contains("LLV7017"), "{bad}: {stdout}");
    }

    // The width is nowhere in the project: a project that states one is
    // refused as an unknown field, so no file of a project can carry it.
    let project = P::example();
    project.edit(
        "lexlean.toml",
        "language = \"1.0\"\n",
        "language = \"1.0\"\nprocess_width = 2\n",
    );
    let error = lexlean::Engine::load(&project.root.join("lexlean.toml"))
        .err()
        .expect("a project that states a width does not load");
    assert!(
        error
            .diagnostics
            .iter()
            .any(|d| d.code.as_str().starts_with("LLC")),
        "a width in lexlean.toml is a configuration error: {error}"
    );

    if !support::lean_backed("VR-20") {
        return;
    }
    // And the width reaches no artifact: the project verified at width 2 has
    // the attestation the default run of the same project has, and neither
    // the attestation nor any other file says what width produced it.
    let baseline = support::verified();
    let project = P::example();
    let wide = verify_at(&project, Operational::new(2, None).expect("width 2")).expect("verifies");
    assert_eq!(wide.attestation_id, baseline.outcome.attestation_id);
    assert_trees_equal(&tree(&wide.root), &tree(&baseline.outcome.root), "width 2");
    for (name, bytes) in tree(&wide.root) {
        let text = String::from_utf8_lossy(&bytes);
        assert!(
            !text.contains("process_width") && !text.contains("LEXLEAN_VERIFY_WIDTH"),
            "{name} records the width"
        );
    }
    assert!(crate::evidence::telemetry_in(wide.root.as_std_path())
        .expect("scans")
        .is_empty());
}

/// Slack, in milliseconds, for comparing the start of one process with the end
/// of another: both are stamped to the millisecond from the clock after the
/// fact (the start is the end minus the wall time).
const SLACK_MS: u64 = 5;

/// How long two processes must overlap to count as running together.
const MIN_OVERLAP_MS: u64 = 50;

/// One child process of a verification, from its telemetry.
struct Span {
    stage: String,
    module: String,
    start: u64,
    end: u64,
}

impl Span {
    /// Do the two intervals share more than `by` milliseconds?
    fn overlaps(&self, other: &Self, by: u64) -> bool {
        self.start.max(other.start) + by < self.end.min(other.end)
    }
}

fn telemetry_events(directory: &std::path::Path) -> Vec<serde_json::Value> {
    std::fs::read_to_string(directory.join(lexlean::verify::telemetry::VERIFY_FILE))
        .expect("telemetry was written")
        .lines()
        .map(|line| serde_json::from_str(line).expect("a telemetry line is JSON"))
        .collect()
}

/// The child processes a verification reported, with their intervals.
fn process_spans(directory: &std::path::Path) -> Vec<Span> {
    telemetry_events(directory)
        .iter()
        .filter(|event| event["event"] == "process")
        .map(|event| {
            let start = event["started_unix_ms"].as_u64().expect("a start");
            Span {
                stage: event["stage"].as_str().unwrap_or_default().to_owned(),
                module: event["module"].as_str().unwrap_or_default().to_owned(),
                start,
                end: start + event["wall_ms"].as_u64().expect("a wall time"),
            }
        })
        .collect()
}

/// The imports of a project's modules, as `module -> imports`, from source.
fn imports_of(project: &P) -> std::collections::BTreeMap<String, Vec<String>> {
    let mut graph = std::collections::BTreeMap::new();
    for name in support::file_set(&project.root.join("src")) {
        let Some(module) = name.strip_suffix(".lex.tex") else {
            continue;
        };
        let text = project.read(&format!("src/{name}"));
        let imports = text
            .lines()
            .filter_map(|line| line.strip_prefix("\\importmodule{"))
            .filter_map(|rest| rest.split('}').next())
            .map(str::to_owned)
            .collect();
        graph.insert(module.to_owned(), imports);
    }
    graph
}

/// The sizes of the topological frontiers of an import graph.
fn frontiers(graph: &std::collections::BTreeMap<String, Vec<String>>) -> Vec<usize> {
    let mut placed: BTreeSet<&str> = BTreeSet::new();
    let mut sizes = Vec::new();
    while placed.len() < graph.len() {
        let ready: Vec<&str> = graph
            .iter()
            .filter(|(module, imports)| {
                !placed.contains(module.as_str())
                    && imports.iter().all(|i| placed.contains(i.as_str()))
            })
            .map(|(module, _)| module.as_str())
            .collect();
        assert!(!ready.is_empty(), "the import graph is acyclic");
        sizes.push(ready.len());
        placed.extend(ready);
    }
    sizes
}

/// VR-21: the same project at two widths, and a completion order that changes
/// nothing.
fn width_invariance() {
    // The scheduler, on every host: items that finish in reverse order are
    // consumed in item order, and of two failures the one first in item order
    // is the one reported, though the later one finished first.
    let items: Vec<usize> = (0..4).collect();
    let (senders, receivers): (Vec<_>, Vec<_>) =
        (0..items.len()).map(|_| mpsc::channel::<()>()).unzip();
    let receivers: Vec<Mutex<mpsc::Receiver<()>>> = receivers.into_iter().map(Mutex::new).collect();
    let finished = Mutex::new(Vec::new());
    let mut consumed = Vec::new();
    lexlean::verify::pool::run_ordered(
        4,
        &items,
        &|item: usize| {
            if item + 1 < items.len() {
                receivers[item]
                    .lock()
                    .expect("lock")
                    .recv_timeout(Duration::from_secs(10))
                    .expect("the next item finishes first");
            }
            finished.lock().expect("lock").push(item);
            if item > 0 {
                senders[item - 1].send(()).expect("wake the previous item");
            }
            item
        },
        &|_: &usize| false,
        |item, value| {
            assert_eq!(item, value);
            consumed.push(item);
            Ok(())
        },
    )
    .expect("four independent jobs");
    assert_eq!(
        *finished.lock().expect("lock"),
        [3, 2, 1, 0],
        "the jobs finish in reverse order"
    );
    assert_eq!(consumed, [0, 1, 2, 3], "and are consumed in item order");

    let (wake, asleep) = mpsc::channel::<()>();
    let asleep = Mutex::new(asleep);
    let error = lexlean::verify::pool::run_ordered(
        4,
        &items,
        &|item: usize| {
            match item {
                1 => asleep
                    .lock()
                    .expect("lock")
                    .recv_timeout(Duration::from_secs(10))
                    .expect("item 3 fails first"),
                3 => wake.send(()).expect("wake item 1"),
                _ => {}
            }
            item
        },
        &|item: &usize| *item == 1 || *item == 3,
        |item, value| {
            if value == 1 || value == 3 {
                Err(lexlean::LexLeanError::from_diagnostic(
                    lexlean::Diagnostic::new(
                        lexlean::code!("LLI9001"),
                        format!("item {item} failed"),
                    ),
                ))
            } else {
                Ok(())
            }
        },
    )
    .expect_err("two items fail");
    assert!(
        error.to_string().contains("item 1 failed"),
        "the failure first in item order is reported: {error}"
    );

    if !support::lean_backed("VR-21") {
        return;
    }
    // The committed peano-arithmetic project: `Addition` is imported by
    // `Divisibility`, `Doubling` and `Order`, which `Main` imports, so its
    // frontiers are one, three, and one modules wide.
    let narrow = P::copy_example("peano-arithmetic");
    assert_eq!(frontiers(&imports_of(&narrow)), [1, 3, 1]);

    let narrow_telemetry = tempfile::tempdir().expect("tempdir");
    let at_one = verify_at(
        &narrow,
        Operational::new(1, Some(narrow_telemetry.path().to_path_buf())).expect("width 1"),
    )
    .expect("width 1");
    let one = tree(&at_one.root);
    let attestation = |files: &std::collections::BTreeMap<String, Vec<u8>>| -> serde_json::Value {
        serde_json::from_slice(&files["attestation.json"]).expect("attestation parses")
    };
    let a1 = attestation(&one);

    // Width 1 is sequential: no two child processes of the verifier overlap.
    let sequential = process_spans(narrow_telemetry.path());
    assert!(
        sequential.len() >= 15,
        "the width-1 run reports its processes: {}",
        sequential.len()
    );
    for (index, left) in sequential.iter().enumerate() {
        for right in &sequential[index + 1..] {
            assert!(
                !left.overlaps(right, SLACK_MS),
                "width 1 ran {} and {} together",
                left.module,
                right.module
            );
        }
    }

    // The wide run. A host can stall for a moment, so a run in which no two
    // members of the three-wide frontier happened to overlap is tried again,
    // twice at most: a width that is applied overlaps three half-second
    // processes started within microseconds of each other, and one that is
    // not never does. Every attempt must be byte-identical to width 1.
    let mut overlapped = false;
    let mut wide = P::copy_example("peano-arithmetic");
    let mut at_three = None;
    for attempt in 0..3 {
        let telemetry = tempfile::tempdir().expect("tempdir");
        wide = P::copy_example("peano-arithmetic");
        let verified = verify_at(
            &wide,
            Operational::new(3, Some(telemetry.path().to_path_buf())).expect("width 3"),
        )
        .expect("width 3");
        assert_eq!(verified.attestation_id, at_one.attestation_id);
        let three = tree(&verified.root);
        assert_trees_equal(&one, &three, "peano-arithmetic at widths 1 and 3");
        let a3 = attestation(&three);
        assert_eq!(
            a1["declarations"], a3["declarations"],
            "observed axiom sets and policy results"
        );
        assert_eq!(
            a1["processes"], a3["processes"],
            "process records, in order"
        );
        assert_eq!(a1["attestation_id"], a3["attestation_id"]);

        let events = telemetry_events(telemetry.path());
        assert_eq!(events[0]["process_width"], 3);
        let members: Vec<u64> = events
            .iter()
            .filter(|event| event["event"] == "frontier")
            .filter_map(|event| event["members"].as_u64())
            .collect();
        assert_eq!(
            members,
            [1, 3, 1],
            "the frontiers of the project at width 3"
        );

        // The barriers hold at width 3 (SPEC.md §22.11): the frontier before
        // starts none of the next, and elaboration, replay and audit do not
        // overlap each other.
        let spans = process_spans(telemetry.path());
        let in_stage = |stage: &str| -> Vec<&Span> {
            spans.iter().filter(|span| span.stage == stage).collect()
        };
        let named = |names: &[&str]| -> Vec<&Span> {
            spans
                .iter()
                .filter(|span| {
                    span.stage == "elaboration"
                        && names
                            .iter()
                            .any(|name| span.module == format!("PeanoArithmetic.{name}"))
                })
                .collect()
        };
        let (first, middle, last) = (
            named(&["Addition"]),
            named(&["Divisibility", "Doubling", "Order"]),
            named(&["Main"]),
        );
        assert_eq!((first.len(), middle.len(), last.len()), (1, 3, 1));
        for (earlier, later) in [(&first, &middle), (&middle, &last)] {
            for a in earlier.iter() {
                for b in later.iter() {
                    assert!(
                        a.end <= b.start + SLACK_MS,
                        "{} was still running when {} started",
                        a.module,
                        b.module
                    );
                }
            }
        }
        let stages = ["elaboration", "replay", "audit"];
        for (index, earlier) in stages.iter().enumerate() {
            for later in &stages[index + 1..] {
                for a in in_stage(earlier) {
                    for b in in_stage(later) {
                        assert!(
                            a.end <= b.start + SLACK_MS,
                            "a {earlier} process ({}) overlaps a {later} process ({})",
                            a.module,
                            b.module
                        );
                    }
                }
            }
        }

        // The members of the three-wide frontier run side by side.
        overlapped = middle.iter().enumerate().any(|(index, left)| {
            middle[index + 1..]
                .iter()
                .any(|right| left.overlaps(right, MIN_OVERLAP_MS))
        });
        at_three = Some(verified);
        if overlapped {
            break;
        }
        eprintln!(
            "VR-21: attempt {} saw no overlap in the wide frontier",
            attempt + 1
        );
    }
    assert!(
        overlapped,
        "three attempts at width 3 never overlapped two members of a three-wide frontier: the width is not applied"
    );
    let at_three = at_three.expect("an attempt ran");

    // A prior verification is never taken in place of the work: a published
    // directory that no longer equals what this run staged is refused, at the
    // wide width as at the narrow one.
    let process_record = at_three
        .root
        .join("process/lean/PeanoArithmetic.Addition.json");
    let mut tampered = std::fs::read(process_record.as_std_path()).expect("reads");
    tampered.push(b' ');
    std::fs::write(process_record.as_std_path(), tampered).expect("tampers");
    let error = verify_at(&wide, Operational::new(3, None).expect("width 3"))
        .err()
        .expect("a tampered published directory is not reused");
    assert!(
        error.to_string().contains("LLB6003")
            && error.to_string().contains("refusing to overwrite"),
        "{error}"
    );

    compiler_input_is_width_invariant();
}

/// The committed `production` example (two modules, six production roots) at
/// width 1 and at width 2, side by side: the canonical compiler input, the
/// preservation record, every certificate, the whole published tree, and the
/// attestation are the same, and the compiler input is the committed one.
/// Extraction and the certificates run after the pooled stages, so this is
/// what shows that no pooled stage reaches them. It costs two production
/// verifications, run together, and is the only case that does.
fn compiler_input_is_width_invariant() {
    let verify = |width: usize| {
        let project = P::copy_example("production");
        let verified = verify_at(&project, Operational::new(width, None).expect("width"))
            .unwrap_or_else(|error| panic!("production at width {width}: {error}"));
        (project, verified)
    };
    let (narrow, wide) = std::thread::scope(|scope| {
        let narrow = scope.spawn(|| verify(1));
        let wide = scope.spawn(|| verify(2));
        (
            narrow.join().expect("width 1 finished"),
            wide.join().expect("width 2 finished"),
        )
    });
    let (_keep_narrow, at_one) = narrow;
    let (_keep_wide, at_two) = wide;
    assert_eq!(at_one.attestation_id, at_two.attestation_id);
    let (one, two) = (tree(&at_one.root), tree(&at_two.root));
    assert_trees_equal(&one, &two, "production at widths 1 and 2");
    let attestation = |files: &std::collections::BTreeMap<String, Vec<u8>>| -> serde_json::Value {
        serde_json::from_slice(&files["attestation.json"]).expect("attestation parses")
    };
    let (a1, a2) = (attestation(&one), attestation(&two));
    for key in [
        "compiler_input",
        "preservation",
        "declarations",
        "processes",
    ] {
        assert!(
            !a1[key].is_null(),
            "a production attestation records `{key}`"
        );
        assert_eq!(a1[key], a2[key], "`{key}` is the same at widths 1 and 2");
    }
    let committed = std::fs::read(
        support::repo_root()
            .join("examples/production/expected/verify/production/compiler-input.json")
            .as_std_path(),
    )
    .expect("the committed compiler input");
    assert!(
        two["production/compiler-input.json"] == committed,
        "the compiler input at width 2 is the committed one"
    );
}

/// The numbers of a spec sentence: the backticked words between the first `(`
/// after `marker` and its `)`.
fn quoted_words(text: &str, marker: &str) -> Vec<String> {
    let after = &text[text.find(marker).expect("the marker is in the text") + marker.len()..];
    let inside = &after[after.find('(').expect("an opening parenthesis") + 1..];
    let inside = &inside[..inside.find(')').expect("a closing parenthesis")];
    inside
        .split('`')
        .skip(1)
        .step_by(2)
        .map(str::to_owned)
        .collect()
}

/// VR-22: telemetry is evidence, never artifact.
fn telemetry_is_noncanonical() {
    // The vocabulary the gate refuses is the vocabulary §22.12 states.
    let stated = quoted_words(
        support::spec_text(),
        "Every key it uses is in a closed vocabulary",
    );
    assert_eq!(
        stated, VOCABULARY,
        "§22.12 and the verifier name the same keys"
    );
    assert!(
        LABELS.iter().all(|label| !VOCABULARY.contains(label)),
        "a label of an event is not a measurement"
    );

    // A scan finds a measurement planted anywhere in a verified directory, and
    // passes a directory that has none.
    let directory = tempfile::tempdir().expect("tempdir");
    std::fs::create_dir_all(directory.path().join("process/lean")).expect("mkdir");
    std::fs::write(
        directory.path().join("process/lean/M.json"),
        br#"{"tool":"lean","exit_code":0}"#,
    )
    .expect("write");
    assert!(crate::evidence::telemetry_in(directory.path())
        .expect("scans")
        .is_empty());
    for key in VOCABULARY {
        std::fs::write(
            directory.path().join("process/lean/M.json"),
            format!(r#"{{"tool":"lean","nested":[{{"{key}":1}}]}}"#),
        )
        .expect("plants");
        let found = crate::evidence::telemetry_in(directory.path()).expect("scans");
        assert_eq!(found.len(), 1, "{key} is found: {found:?}");
        assert!(found[0].contains(key));
    }

    if !support::lean_backed("VR-22") {
        return;
    }
    let project = P::example();
    let without =
        verify_at(&project, Operational::new(1, None).expect("width 1")).expect("verifies");
    let sink = tempfile::tempdir().expect("tempdir");
    assert!(
        !sink.path().starts_with(project.root.as_std_path()),
        "the telemetry directory is not inside the project"
    );
    let other = P::example();
    let with = verify_at(
        &other,
        Operational::new(1, Some(sink.path().to_path_buf())).expect("width 1"),
    )
    .expect("verifies with telemetry");
    assert_eq!(with.attestation_id, without.attestation_id);
    assert_trees_equal(
        &tree(&with.root),
        &tree(&without.root),
        "telemetry on and off",
    );
    assert!(crate::evidence::telemetry_in(with.root.as_std_path())
        .expect("scans")
        .is_empty());
    assert!(
        !other
            .root
            .join(".lexlean")
            .join("verified")
            .join("verify-telemetry.jsonl")
            .exists(),
        "no telemetry is written beside the artifacts"
    );

    let text = std::fs::read_to_string(sink.path().join(lexlean::verify::telemetry::VERIFY_FILE))
        .expect("telemetry was written");
    let events: Vec<serde_json::Value> = text
        .lines()
        .map(|line| serde_json::from_str(line).expect("a line is JSON"))
        .collect();
    let allowed: BTreeSet<&str> = VOCABULARY.iter().chain(LABELS.iter()).copied().collect();
    for event in &events {
        for key in event.as_object().expect("an object").keys() {
            assert!(
                allowed.contains(key.as_str()),
                "`{key}` is in no vocabulary: {event}"
            );
        }
    }
    let kinds: BTreeSet<&str> = events
        .iter()
        .filter_map(|event| event["event"].as_str())
        .collect();
    for kind in [
        "verify_begin",
        "stage",
        "frontier",
        "process",
        "completion",
        "verify_end",
    ] {
        assert!(kinds.contains(kind), "no `{kind}` event: {kinds:?}");
    }
    let stages: BTreeSet<&str> = events
        .iter()
        .filter(|event| event["event"] == "stage")
        .filter_map(|event| event["stage"].as_str())
        .collect();
    for stage in [
        "probe",
        "elaboration",
        "replay",
        "audit",
        "policy",
        "attestation",
    ] {
        assert!(stages.contains(stage), "no `{stage}` stage: {stages:?}");
    }
    assert_eq!(events.last().expect("events")["ok"], true);
    if cfg!(target_os = "linux") {
        assert!(
            events
                .iter()
                .filter(|event| event["event"] == "process" && event["tool"] == "lean")
                .all(|event| event["peak_rss_kib"].as_u64().is_some_and(|peak| peak > 0)),
            "Lean processes are measured where the host has /proc"
        );
    }

    // A planted measurement in a real verified directory is found: the
    // attestation of the run, with a wall time added to its first process.
    let planted = tempfile::tempdir().expect("tempdir");
    for (name, bytes) in tree(&with.root) {
        let destination = planted.path().join(&name);
        std::fs::create_dir_all(destination.parent().expect("parent")).expect("mkdir");
        std::fs::write(destination, bytes).expect("copy");
    }
    let path = planted.path().join("process/lean").join(
        support::file_set(&with.root.join("process/lean"))
            .into_iter()
            .next()
            .expect("a Lean process record"),
    );
    let mut record: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&path).expect("reads")).expect("parses");
    record["wall_ms"] = serde_json::json!(1234);
    std::fs::write(&path, serde_json::to_vec(&record).expect("serializes")).expect("plants");
    let found = crate::evidence::telemetry_in(planted.path()).expect("scans");
    assert_eq!(found.len(), 1, "{found:?}");

    // A telemetry directory that cannot be written changes no result: the
    // sink is a path under a regular file.
    let blocked = tempfile::tempdir().expect("tempdir");
    std::fs::write(blocked.path().join("file"), b"x").expect("write");
    let third = P::example();
    let unwritable = verify_at(
        &third,
        Operational::new(1, Some(blocked.path().join("file").join("sink"))).expect("width 1"),
    )
    .expect("an unwritable sink does not fail verification");
    assert_eq!(unwritable.attestation_id, without.attestation_id);
}

/// VR-23: a verification run as the parts of one run (SPEC.md §9.4) equals
/// the whole, and a plan that cannot run is refused.
fn parts_equal_the_whole() {
    use lexlean::{VerifyPart, VerifyPartOutcome, VerifyPlan};
    if !support::lean_backed("VR-23") {
        return;
    }
    let example = "peano-arithmetic";
    let whole_project = P::copy_example(example);
    let whole = verify_at(&whole_project, Operational::new(1, None).expect("width 1"))
        .expect("the whole verification");

    // The plan, from the example's committed build: its first topological
    // frontier is one shard, its second another that also replays the first,
    // and the rest is the join's.
    let graph = crate::shards::import_graph(
        support::repo_root()
            .join("examples")
            .join(example)
            .as_std_path(),
    )
    .expect("the import graph reads from the committed build");
    let mut placed: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let mut remaining: Vec<String> = graph.keys().cloned().collect();
    let mut frontiers: Vec<Vec<String>> = Vec::new();
    while !remaining.is_empty() {
        let ready: Vec<String> = remaining
            .iter()
            .filter(|module| graph[*module].iter().all(|import| placed.contains(import)))
            .cloned()
            .collect();
        assert!(!ready.is_empty(), "the import graph converges");
        placed.extend(ready.iter().cloned());
        remaining.retain(|module| !ready.contains(module));
        frontiers.push(ready);
    }
    assert!(
        frontiers.len() >= 3,
        "the fixture has at least three frontiers: {frontiers:?}"
    );
    let first = frontiers[0].clone();
    let second = frontiers[1].clone();
    let rest: Vec<String> = frontiers[2..].concat();
    let parts = tempfile::tempdir().expect("tempdir");
    let dir = |name: &str| {
        camino::Utf8PathBuf::from_path_buf(parts.path().join(name)).expect("utf8 tempdir")
    };
    // Each part verifies its own copy of the project; the copies live until
    // the end of the case, as the join's published directory is inside one.
    let copies: std::cell::RefCell<Vec<P>> = std::cell::RefCell::new(Vec::new());
    let run_part = |part: VerifyPart| -> Result<VerifyPartOutcome, lexlean::LexLeanError> {
        let _environment = support::env_shared();
        let project = P::copy_example(example);
        let outcome = Operational::new(1, None).expect("width 1").scoped(|| {
            lexlean::verify_part(
                &project.engine(),
                VerifyRequest {
                    selection: Selection::Entrypoints,
                },
                part,
            )
        });
        copies.borrow_mut().push(project);
        outcome
    };
    let plan = |elaborate: &[String], replay: &[String]| VerifyPlan {
        elaborate: elaborate.to_vec(),
        replay: replay.to_vec(),
    };

    let a = run_part(VerifyPart::Shard {
        plan: plan(&first, &[]),
        imports: vec![],
        output: dir("a"),
    })
    .expect("the first shard");
    assert!(
        matches!(a, VerifyPartOutcome::Shard { processes, .. } if processes == first.len()),
        "the first shard ran its elaborations and nothing else"
    );
    let mut first_and_second = first.clone();
    first_and_second.extend(second.iter().cloned());
    let b = run_part(VerifyPart::Shard {
        plan: plan(&second, &first_and_second),
        imports: vec![dir("a")],
        output: dir("b"),
    })
    .expect("the second shard, with the first's oleans");
    assert!(
        matches!(b, VerifyPartOutcome::Shard { processes, .. } if processes == second.len() + 2 * first_and_second.len()),
        "the second shard ran its elaborations and the replays and audits of both"
    );
    let joined = match run_part(VerifyPart::Join {
        plan: plan(&rest, &rest),
        parts: vec![dir("a"), dir("b")],
    })
    .expect("the join")
    {
        VerifyPartOutcome::Verified(verified) => verified,
        VerifyPartOutcome::Shard { .. } => panic!("the join publishes"),
    };
    assert_eq!(
        joined.attestation_id, whole.attestation_id,
        "the join and the whole verification have one attestation identity"
    );
    assert_trees_equal(&tree(&joined.root), &tree(&whole.root), "whole and joined");

    // A shard whose plan elaborates a module before its imports does not
    // converge, and names the import.
    let error = run_part(VerifyPart::Shard {
        plan: plan(&second, &[]),
        imports: vec![],
        output: dir("c"),
    })
    .map(|_| ())
    .expect_err("a module elaborated before its imports");
    let text = error.to_string();
    assert!(
        text.contains("module order did not converge") && text.contains(&first[0]),
        "{text}"
    );
    // A join that is handed one shard too few has no record of a process.
    let error = run_part(VerifyPart::Join {
        plan: plan(&rest, &rest),
        parts: vec![dir("b")],
    })
    .map(|_| ())
    .expect_err("a join missing a shard");
    let text = error.to_string();
    assert!(
        text.contains("from any part of the run") || text.contains("module order did not converge"),
        "{text}"
    );
    // A join whose plan leaves a replay to no part has no record of it.
    let error = run_part(VerifyPart::Join {
        plan: plan(&rest, &rest[1..]),
        parts: vec![dir("a"), dir("b")],
    })
    .map(|_| ())
    .expect_err("a replay no part ran");
    let text = error.to_string();
    // The audit member of the module is placed with its replay, so the hole
    // is seen at the audit stage, before the records are assembled.
    assert!(
        text.contains("from any part of the run") && text.contains(&rest[0]),
        "{text}"
    );
    // A join refuses a record that another `leanchecker` ran: the second
    // shard's replay record of a module names an executable that is not the
    // join's, as a part on a different toolchain would have written it.
    let replayed = &first[0];
    let record_path = dir("b")
        .join("process")
        .join("leanchecker")
        .join(format!("{replayed}.json"));
    let text = std::fs::read_to_string(record_path.as_std_path()).expect("the replay record");
    let recorded = serde_json::from_str::<serde_json::Value>(&text).expect("the record parses")
        ["executable_sha256"]
        .as_str()
        .expect("the record names its executable")
        .to_owned();
    let planted = "0".repeat(recorded.len());
    assert_ne!(recorded, planted);
    std::fs::write(
        record_path.as_std_path(),
        text.replacen(&recorded, &planted, 1),
    )
    .expect("plant another executable");
    let error = run_part(VerifyPart::Join {
        plan: plan(&rest, &rest),
        parts: vec![dir("a"), dir("b")],
    })
    .map(|_| ())
    .expect_err("a record of another leanchecker");
    support::expect_code(&error, "LLI9001");
    let text = error.to_string();
    assert!(
        text.contains(&format!("replay record of `{replayed}`"))
            && text.contains(&planted)
            && text.contains(&recorded),
        "{text}"
    );
}
