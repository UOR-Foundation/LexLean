//! `BS-28` (SPEC.md §17.23, §17.21): where the module of a source is, as
//! language-1.2 source, `LexLeanBoot.SurfaceModule`, held to the byte to
//! `lexlean::surface::module::locate`.
//!
//! `locateModule` searches a text for the first environment of a kind with a
//! notation (semantic, tooling, core, in that order), takes the body to the last
//! closing marker of that kind, and, when the body is the control of the data
//! form followed by braces, the byte range of what the braces hold; it trims
//! white space as Rust's `char::is_whitespace` defines it, with the scanners of
//! `AxiomOutput`.
//!
//! Each planted defect names the one vector that catches it.

use std::collections::{BTreeMap, BTreeSet};

use lexlean::calculus::Value;
use lexlean::surface::module::locate;
use serde_json::Value as Json2;

use super::rust_validate::{eligibility_of, rustc_differential};
use super::{nat_val, option, pair, text, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.SurfaceModule";
const ROOTS: [(&str, &[&str]); 1] = [("locateModule", &["allocation", "overflow"])];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// Committed sources run under the evaluator: those of at most this many bytes.
const EVALUATOR_BYTES: usize = 4_000;
/// Committed sources run under rustc: those of at most this many bytes.
const RUSTC_BYTES: usize = 200_000;
/// Seeded texts under the evaluator, and under rustc.
const EVALUATOR_SEEDED: usize = 300;
const RUSTC_SEEDED: usize = 1_500;

const KINDS: [&str; 3] = ["semanticmodule", "toolingmodule", "coremodule"];

/// What `locate` says of a text, as the module's `Option Located`.
fn located(source: &str) -> Value {
    option(locate(source).map(|found| {
        let kind = KINDS
            .iter()
            .position(|environment| *environment == found.kind.environment)
            .expect("a kind of the table");
        Value::Adt {
            constructor: 0,
            fields: vec![
                nat_val(kind),
                nat_val(found.body.0),
                nat_val(found.body.1),
                option(
                    found
                        .data
                        .map(|(start, end)| pair(nat_val(start), nat_val(end))),
                ),
            ],
        }
    }))
}

fn run(source: &str) -> Run {
    (
        vec![text(source)],
        serde_json::to_value(located(source)).expect("a value is JSON"),
    )
}

// ------------------------------------------------------------------ vectors

const B: &str = "\\begin{semanticmodule}";
const E: &str = "\\end{semanticmodule}";

fn wrapped(inner: &str) -> String {
    format!("\\begin{{lexlean}}{{M}}\n{inner}\\end{{lexlean}}\n")
}

#[allow(clippy::too_many_lines)]
fn named() -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    let mut add = |label: &str, source: String| out.push((label.to_owned(), source));
    add(
        "the oracle's data form",
        wrapped("\\begin{semanticmodule}\n\\semanticdata{{\"a\":1}}\n\\end{semanticmodule}\n"),
    );
    add(
        "the oracle's notation form",
        wrapped("\\begin{toolingmodule}\n{declarations: []}\n\\end{toolingmodule}\n"),
    );
    add("no module", wrapped(""));
    add("a begin marker alone", B.to_owned());
    add("an end marker alone", E.to_owned());
    add("an empty text", String::new());
    add("an empty body", format!("{B}{E}"));
    add("the end before the begin", format!("{E}{B}"));
    add("the end before and after the begin", format!("{E}{B}x{E}"));
    add("the first end marker", format!("{B}a{E}b{E}"));
    add("the first begin marker", format!("x{B}a{B}b{E}"));
    add(
        "a core module",
        "\\begin{coremodule}\n\\coredata{}\n\\end{coremodule}".to_owned(),
    );
    add(
        "a core module in notation",
        "\\begin{coremodule}\nimports []\n\\end{coremodule}".to_owned(),
    );
    add(
        "a tooling module in data form",
        "\\begin{toolingmodule}\\toolingdata{x}\\end{toolingmodule}".to_owned(),
    );
    add(
        "the first environment is the module, tooling before semantic",
        "\\begin{toolingmodule}a\\end{toolingmodule}\\begin{semanticmodule}b\\end{semanticmodule}"
            .to_owned(),
    );
    add(
        "the first environment is the module, core before tooling",
        "\\begin{coremodule}a\\end{coremodule}\\begin{toolingmodule}b\\end{toolingmodule}"
            .to_owned(),
    );
    add(
        "an end of another kind after the opening is no module",
        "\\begin{semanticmodule}a\\begin{toolingmodule}b\\end{toolingmodule}".to_owned(),
    );
    add(
        "an end before the opening does not close it",
        "\\end{semanticmodule}\\begin{semanticmodule}\\begin{toolingmodule}b\\end{toolingmodule}"
            .to_owned(),
    );
    add(
        "the end of another kind is no end",
        "\\begin{semanticmodule}a\\end{toolingmodule}".to_owned(),
    );
    add(
        "an escaped marker is not a marker",
        format!("\\\\begin{{toolingmodule}}{B}a{E}"),
    );
    add(
        "an environment of no kind is no module",
        format!("\\begin{{document}}{B}a{E}"),
    );
    add(
        "a begin without its name is no module",
        format!("\\begin x{B}a{E}"),
    );
    add(
        "a begin with no open brace before its name",
        format!("\\begin?semanticmodule}}a{E}"),
    );
    add(
        "a control with more letters than begin",
        format!("\\beginx{{semanticmodule}}a{E}"),
    );
    add(
        "a backslash before a scalar beyond ASCII is no control",
        format!("\\\u{e9}{B}a{E}"),
    );
    add("an end inside the body", format!("{B}a\\end{{x}}b{E}"));
    add(
        "the data form without white space",
        format!("{B}\\semanticdata{{x}}{E}"),
    );
    add(
        "the data form with white space everywhere",
        format!("{B} \n\t\\semanticdata \n{{x}} \n\t{E}"),
    );
    add(
        "the data form with a no-break space",
        format!("{B}\u{a0}\\semanticdata\u{a0}{{x}}\u{a0}{E}"),
    );
    add(
        "the data form with a line separator",
        format!("{B}\u{2028}\\semanticdata{{x}}\u{3000}{E}"),
    );
    add("empty data", format!("{B}\\semanticdata{{}}{E}"));
    add(
        "data of only white space",
        format!("{B}\\semanticdata{{  }}{E}"),
    );
    add(
        "data with braces inside",
        format!("{B}\\semanticdata{{{{a}}{{b}}}}{E}"),
    );
    add("data without its close", format!("{B}\\semanticdata{{x{E}"));
    add("data without its open", format!("{B}\\semanticdata x}}{E}"));
    add(
        "data with text after the close",
        format!("{B}\\semanticdata{{x}} y{E}"),
    );
    add(
        "the control of another kind",
        format!("{B}\\toolingdata{{x}}{E}"),
    );
    add(
        "a control with more letters",
        format!("{B}\\semanticdatax{{x}}{E}"),
    );
    add(
        "text before the control",
        format!("{B}x \\semanticdata{{x}}{E}"),
    );
    add(
        "a brace that is only a close",
        format!("{B}\\semanticdata}}{E}"),
    );
    add("a lone open brace", format!("{B}\\semanticdata{{{E}"));
    add(
        "a control and nothing else",
        format!("{B}\\semanticdata{E}"),
    );
    add(
        "a scalar beyond ASCII in the body",
        format!("{B}\u{e9}\\semanticdata{{\u{e9}}}{E}"),
    );
    add(
        "a no-break space is white but a zero-width space is not",
        format!("{B}\u{200b}\\semanticdata{{x}}{E}"),
    );
    add(
        "white space of three bytes before the close",
        format!("{B}\\semanticdata{{x}}\u{3000}\u{2029}{E}"),
    );
    out
}

// ------------------------------------------------------------------- seeded

const PIECES: &[&str] = &[
    "\\begin{semanticmodule}",
    "\\end{semanticmodule}",
    "\\begin{toolingmodule}",
    "\\end{toolingmodule}",
    "\\begin{coremodule}",
    "\\end{coremodule}",
    "\\semanticdata",
    "\\toolingdata",
    "\\coredata",
    "{",
    "}",
    " ",
    "\n",
    "\u{a0}",
    "\u{3000}",
    "x",
];

fn edited(seeded: &mut Seeded, source: &str) -> String {
    let mut scalars: Vec<char> = source.chars().collect();
    for _ in 0..=seeded.below(3) {
        let at = seeded.below(scalars.len() + 1);
        let piece: Vec<char> = seeded.pick(PIECES).chars().collect();
        match seeded.below(3) {
            0 if at < scalars.len() => {
                scalars.remove(at);
            }
            1 => {
                drop(scalars.splice(at..at, piece));
            }
            _ if at < scalars.len() => {
                drop(scalars.splice(at..=at, piece));
            }
            _ => scalars.extend(piece),
        }
    }
    scalars.into_iter().collect()
}

fn drawn(seeded: &mut Seeded) -> String {
    let count = 1 + seeded.below(8);
    (0..count).map(|_| *seeded.pick(PIECES)).collect()
}

fn seeded_runs(count: usize, bases: &[String], seed: u64) -> Vec<Run> {
    let mut seeded = Seeded(seed);
    let mut out = Vec::with_capacity(count);
    while out.len() < count {
        let source = if seeded.chance(50) && !bases.is_empty() {
            let base = seeded.pick(bases).clone();
            edited(&mut seeded, &base)
        } else {
            drawn(&mut seeded)
        };
        out.push(run(&source));
    }
    out
}

/// Every committed `.lex.tex` source, with its path.
fn sources() -> Vec<(String, String)> {
    let root = repo_root();
    let walker = walkdir::WalkDir::new(root.as_std_path())
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|entry| {
            let name = entry.file_name().to_string_lossy();
            !(entry.file_type().is_dir()
                && (matches!(name.as_ref(), "target" | ".git" | ".lexlean")
                    || support::holds_cache_tag(entry)))
        });
    let mut out = Vec::new();
    for entry in walker.flatten() {
        let path = entry.path();
        if !entry.file_type().is_file() || !path.to_string_lossy().ends_with(".lex.tex") {
            continue;
        }
        let Ok(content) = std::fs::read_to_string(path) else {
            continue;
        };
        let relative = path
            .strip_prefix(root.as_std_path())
            .expect("under the root")
            .to_string_lossy()
            .replace('\\', "/");
        out.push((relative, content));
    }
    out
}

// ------------------------------------------------------------------- plants

struct Plant {
    label: &'static str,
    site: &'static str,
    replacement: &'static str,
    vector: &'static str,
}

const PLANTS: &[Plant] = &[
    Plant {
        label: "an end does not stop the search",
        site: "(if (isControl text atom (lit \"\\\\end\")) then (ctor Option.none [Nat]) else",
        replacement: "(if false then (ctor Option.none [Nat]) else",
        vector: "an empty body",
    },
    Plant {
        label: "the lexlean environment is not passed over",
        site: "(if (prim equal (nameBytes text name) (lit \"lexlean\") : Bool) then",
        replacement: "(if false then",
        vector: "the oracle's data form",
    },
    Plant {
        label: "an end of another kind closes the module",
        site: "(if (prim equal (nameBytes text name) (environment kind) : Bool) then",
        replacement: "(if true then",
        vector: "the end of another kind is no end",
    },
    Plant {
        label: "an escaped backslash is no atom of its own",
        site: "(ctor Option.some [(Nat * Nat)] (start, (start + 2)))",
        replacement: "(ctor Option.some [(Nat * Nat)] (start, (start + 1)))",
        vector: "an escaped marker is not a marker",
    },
    Plant {
        label: "the body ends after the end control",
        site: "(ctor Option.some [Nat] (first atom))",
        replacement: "(ctor Option.some [Nat] (second atom))",
        vector: "an empty body",
    },
    Plant {
        label: "the body begins at the close brace of the name",
        site: "(kind, (((second name) + 1), (second atom)))",
        replacement: "(kind, ((second name), (second atom)))",
        vector: "an empty body",
    },
    Plant {
        label: "a name needs no open brace",
        site: "(if (AxiomOutput::byteIs text at (123 : UInt8)) then",
        replacement: "(if true then",
        vector: "a begin with no open brace before its name",
    },
    Plant {
        label: "core is no kind",
        site: "(prim equal name (environment 2) : Bool)",
        replacement: "(prim equal name (environment 0) : Bool)",
        vector: "a core module",
    },
    Plant {
        label: "a control is read by its prefix",
        site: "(prim slice text (first atom) (prim subtract (second atom) (first atom) : Nat) : (Option Bytes))",
        replacement: "(prim slice text (first atom) (prim length word : Nat) : (Option Bytes))",
        vector: "a control with more letters than begin",
    },
    Plant {
        label: "the core control is the semantic one",
        site: "(lit \"\\\\coredata\")",
        replacement: "(lit \"\\\\semanticdata\")",
        vector: "a core module",
    },
    Plant {
        label: "no white space before the control",
        site: "(let lead : Nat := (AxiomOutput::skipWhite region);",
        replacement: "(let lead : Nat := 0;",
        vector: "the data form with white space everywhere",
    },
    Plant {
        label: "no white space before the open brace",
        site: "(let brace : Nat := (afterTag + (AxiomOutput::skipWhite (tail region afterTag)));",
        replacement: "(let brace : Nat := afterTag;",
        vector: "the data form with white space everywhere",
    },
    Plant {
        label: "white space before the close is kept",
        site: "(let trimmedStop : Nat := (AxiomOutput::dropWhiteEnd inside 0);",
        replacement: "(let trimmedStop : Nat := (prim length inside : Nat);",
        vector: "white space of three bytes before the close",
    },
    Plant {
        label: "the data range ends at the close brace",
        site: "(((start + brace) + 1) + (prim subtract trimmedStop 1 : Nat))",
        replacement: "(((start + brace) + 1) + trimmedStop)",
        vector: "the oracle's data form",
    },
    Plant {
        label: "the data range begins at the open brace",
        site: "(ctor Option.some [(Nat * Nat)] (((start + brace) + 1),",
        replacement: "(ctor Option.some [(Nat * Nat)] ((start + brace),",
        vector: "data with braces inside",
    },
    Plant {
        label: "data need not close",
        site: "(if (if (0 <? trimmedStop) then (LexLeanStd.Text::matchAt inside (lit \"}\") (prim subtract trimmedStop 1 : Nat)) else false) then",
        replacement: "(if (0 <? trimmedStop) then",
        vector: "data with text after the close",
    },
    Plant {
        label: "the open brace is not required",
        site: "(if (LexLeanStd.Text::startsWith (tail region brace) (lit \"{\")) then",
        replacement: "(if true then",
        vector: "data without its open",
    },
];

fn plants(vectors: &[(String, String)]) {
    let mut missed = Vec::new();
    for plant in PLANTS {
        let source = &vectors
            .iter()
            .find(|(label, _)| label == plant.vector)
            .unwrap_or_else(|| panic!("{}: no vector `{}`", plant.label, plant.vector))
            .1;
        let planted = support::P::bootstrap();
        let module = planted.read_raw("src/SurfaceModule.lex.tex");
        assert_eq!(
            module.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/SurfaceModule.lex.tex",
            &module.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[0].0);
        match super::evaluate(&low, &[run(source)], 1).first() {
            Some(first) => eprintln!(
                "BS-28: plant detected: {}: {}",
                plant.label,
                first.chars().take(160).collect::<String>()
            ),
            None => missed.push(plant.label),
        }
    }
    assert!(
        missed.is_empty(),
        "planted defects their own vector does not detect: {missed:?}"
    );
}

fn export(lowered: &BTreeMap<&str, super::Lowered>) {
    let exported = harness::exported("bootstrap");
    let bundle = exported.bundle();
    let mut realized = 0usize;
    for exported_root in &exported.output.result.roots {
        let Some((member, low)) = exported_root
            .root
            .strip_prefix(&format!("{MODULE}."))
            .and_then(|member| lowered.get(member).map(|low| (member, low)))
        else {
            continue;
        };
        let largest = exported_root
            .outputs
            .iter()
            .flat_map(|output| &output.certification.certificates)
            .map(|certificate| certificate.file.byte_length)
            .max()
            .expect("a certificate");
        eprintln!(
            "BS-28: {}: largest certificate part {largest} bytes",
            exported_root.root
        );
        assert!(
            largest <= CERTIFICATE_BUDGET,
            "{}: a certificate part of {largest} bytes is beyond {CERTIFICATE_BUDGET}",
            exported_root.root
        );
        for output in &exported_root.outputs {
            let package: Json2 = serde_json::from_slice(
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
                serde_json::to_value(&low.program).expect("a program is JSON"),
                "{member}: the exported package realizes the program the differential ran"
            );
            realized += 1;
        }
    }
    assert_eq!(realized, ROOTS.len(), "one certified package per root");
    let packages = harness::tree(&bundle.join("packages"));
    let committed_packages = harness::tree(&harness::fixture_dir("bootstrap").join("packages"));
    assert_eq!(
        packages.keys().collect::<Vec<_>>(),
        committed_packages.keys().collect::<Vec<_>>(),
        "the exported package files are the committed ones"
    );
    for (relative, content) in &packages {
        assert!(
            committed_packages.get(relative) == Some(content),
            "packages/{relative} differs from the committed package"
        );
    }
}

#[allow(clippy::too_many_lines)]
pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("SurfaceModule"),
        "SurfaceModule is a linked module of the project"
    );
    eligibility_of(&project, "SurfaceModule", MODULE, &ROOTS);

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|(label, _)| label.as_str()).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let outcomes: Vec<Option<bool>> = vectors
        .iter()
        .map(|(_, source)| locate(source).map(|found| found.data.is_some()))
        .collect();
    for (what, wanted) in [
        ("no module", None),
        ("a module in notation", Some(false)),
        ("a module in data form", Some(true)),
    ] {
        assert!(
            outcomes.contains(&wanted),
            "no vector reaches the outcome `{what}`"
        );
    }
    let mut named_runs: Vec<Run> = vectors.iter().map(|(_, source)| run(source)).collect();

    let texts = sources();
    assert!(
        texts.len() >= 100,
        "the committed sources are {}",
        texts.len()
    );
    let mut evaluator_runs: Vec<Run> = Vec::new();
    let mut rustc_runs: Vec<Run> = Vec::new();
    let mut bases: Vec<String> = Vec::new();
    for (_, content) in &texts {
        if content.len() <= EVALUATOR_BYTES {
            evaluator_runs.push(run(content));
            bases.push(content.clone());
        }
        if content.len() <= RUSTC_BYTES {
            rustc_runs.push(run(content));
        }
    }
    let normative = support::normative_host();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (
            seeded_runs(EVALUATOR_SEEDED, &bases, 0x0B55_0028),
            seeded_runs(RUSTC_SEEDED, &bases, 0x0B55_1028),
        )
    } else {
        (Vec::new(), Vec::new())
    };
    eprintln!(
        "BS-28: {} vectors, {} committed sources ({} under the evaluator, {} under rustc), {} and {} seeded texts ({:?})",
        vectors.len(),
        texts.len(),
        evaluator_runs.len(),
        rustc_runs.len(),
        evaluator_seeded.len(),
        rustc_seeded.len(),
        started.elapsed()
    );

    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, super::Lowered> = ROOTS
        .iter()
        .map(|(root, _)| (*root, super::lower(&checked, &project, MODULE, root)))
        .collect();
    if normative {
        let mut corpus: Vec<Run> = named_runs.clone();
        corpus.extend(evaluator_runs);
        corpus.extend(evaluator_seeded);
        let disagreements = super::evaluate(&lowered[ROOTS[0].0], &corpus, 5);
        assert!(
            disagreements.is_empty(),
            "locateModule and the Rust oracle disagree: {disagreements:?}"
        );
        eprintln!(
            "BS-28: locateModule: the evaluator agrees on {} runs ({:?})",
            corpus.len(),
            started.elapsed()
        );
        plants(&vectors);
    } else {
        eprintln!("BS-28: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    named_runs.extend(rustc_runs);
    named_runs.extend(rustc_seeded);
    let total = rustc_differential(
        "BS-28",
        &[(ROOTS[0].0, &lowered[ROOTS[0].0], named_runs.as_slice())],
        "oracle",
    );
    eprintln!("BS-28: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-28") {
        return;
    }
    export(&lowered);
    eprintln!("BS-28: passed in {:?}", started.elapsed());
}
