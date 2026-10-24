//! Row 5a of issue 92 (placeholder ID `BS-90`, the maintainer assigns the
//! number at registration): the declaration check of the production registry
//! (SPEC.md §17.13, `production::check_declaration`) as language-1.2 source
//! (`LexLeanBoot.ProductionRegistry`), held to the oracle to the byte.
//!
//! The registry's rows are an argument, as decision 7 of the step 4 plan makes
//! host-read data (the host parses `language/production-1.2.toml`): the root
//! takes the declaration (a name, its targets, its effects) and the ids of the
//! registered targets and effects, and returns the oracle's refusal or unit.
//! The corpora are the declarations of every committed `.lex.tex` source
//! (positive and negative projects alike), the rules of the check, and a
//! seeded corpus over a pool of near-miss strings and random registries.

use lexlean::calculus::Value;
use lexlean::production::{check_declaration, registry};
use serde_json::Value as Json;

use super::rust_validate::rustc_differential;
use super::{lower, text, texts, Run, Seeded};
use crate::support;

const MODULE: &str = "LexLeanBoot.ProductionRegistry";

/// The row of a root in the eligibility report of its module: eligible for
/// `rust-std`, with its declared effects exactly the effects it realizes.
fn assert_eligible(row: &Json, root: &str, effects: &[&str]) {
    let declared: Vec<&str> = row["declared_effects"]
        .as_array()
        .expect("declared effects")
        .iter()
        .map(|effect| effect.as_str().expect("an effect"))
        .collect();
    assert_eq!(declared, effects, "{root}: the declared effects");
    let targets = row["targets"].as_array().expect("targets");
    assert_eq!(targets.len(), 1, "{root}: one target");
    let target = &targets[0];
    assert_eq!(target["target"], "rust-std", "{root}: the target");
    assert_eq!(target["status"], "eligible", "{root}: eligible");
    let realized: Vec<&str> = target["effects"]
        .as_array()
        .expect("effects")
        .iter()
        .map(|effect| effect["effect"].as_str().expect("an effect"))
        .collect();
    assert_eq!(
        realized, effects,
        "{root}: the declaration admits exactly the realized effects"
    );
}

fn eligibility_report(
    project: &support::P,
    build_id: &lexlean::artifact::content_id::Sha256Digest,
    module: &str,
) -> Json {
    let file = module.replace('.', "/");
    serde_json::from_slice(
        &std::fs::read(
            project
                .build_dir(build_id)
                .join(format!("production/{file}.eligibility.json"))
                .as_std_path(),
        )
        .expect("the eligibility report"),
    )
    .expect("the report is JSON")
}

/// A row registered after #259 is on the line takes 75% of the cap then in
/// force: 4,718,592 of 6,291,456.
const CERTIFICATE_BUDGET: u64 = 4_718_592;

// The counts SPEC.md §17.23, the register statement of the row, its scenario,
// and VERIFICATION.md state: the case fails when the tree no longer gives them.
const COMMITTED: usize = 297;
const RULES: usize = 170;
const SEEDED: usize = 2_000;
const RUNS: usize = 2_467;

fn outcome(name: &str, targets: &[String], effects: &[String]) -> Json {
    serde_json::to_value(match check_declaration(name, targets, effects) {
        Ok(()) => Value::Ok {
            value: Box::new(Value::Unit),
        },
        Err(reason) => Value::Error {
            value: Box::new(text(&reason)),
        },
    })
    .expect("a value is JSON")
}

/// One run against the real registry: the oracle reads the embedded registry,
/// the root is given its ids.
fn run(name: &str, targets: &[&str], effects: &[&str]) -> Run {
    let targets: Vec<String> = targets.iter().map(|item| (*item).to_owned()).collect();
    let effects: Vec<String> = effects.iter().map(|item| (*item).to_owned()).collect();
    let registry = registry().expect("the registry");
    let registered_targets: Vec<String> = registry.targets.keys().cloned().collect();
    let registered_effects: Vec<String> = registry.effects.keys().cloned().collect();
    (
        vec![
            text(name),
            texts(&targets),
            texts(&effects),
            texts(&registered_targets),
            texts(&registered_effects),
        ],
        outcome(name, &targets, &effects),
    )
}

/// The `production: {effects: [...], targets: [...]}` declarations of every
/// committed `.lex.tex` source, each with the name of the definition it ends.
fn committed() -> Vec<(String, Vec<String>, Vec<String>)> {
    let mut out: Vec<(String, Vec<String>, Vec<String>)> = Vec::new();
    for entry in walkdir::WalkDir::new(support::repo_root().as_std_path())
        .into_iter()
        .filter_entry(|entry| {
            let name = entry.file_name().to_string_lossy();
            name != "target" && name != ".git" && name != ".lexlean"
        })
        .flatten()
    {
        if !entry.file_name().to_string_lossy().ends_with(".lex.tex") {
            continue;
        }
        let Ok(source) = std::fs::read_to_string(entry.path()) else {
            continue;
        };
        let mut from = 0;
        while let Some(found) = source[from..].find("production: {") {
            let start = from + found;
            from = start + "production: {".len();
            let Some(end) = source[from..].find('}') else {
                break;
            };
            let body = &source[from..from + end];
            let list = |key: &str| -> Option<Vec<String>> {
                let at = body.find(&format!("{key}: ["))? + key.len() + 3;
                let close = body[at..].find(']')?;
                Some(
                    body[at..at + close]
                        .split(',')
                        .map(str::trim)
                        .filter(|item| !item.is_empty())
                        .map(str::to_owned)
                        .collect(),
                )
            };
            let name = source[..start]
                .rfind("name: ")
                .map(|at| {
                    source[at + 6..]
                        .split([',', '\n'])
                        .next()
                        .unwrap_or("")
                        .trim()
                        .to_owned()
                })
                .unwrap_or_default();
            if let (Some(effects), Some(targets)) = (list("effects"), list("targets")) {
                let row = (name, targets, effects);
                if !out.contains(&row) {
                    out.push(row);
                }
            }
        }
    }
    out
}

/// The rules of the check, in the order the oracle applies them.
fn rules() -> Vec<Run> {
    let mut out = Vec::new();
    let rule = |name: &str, targets: &[&str], effects: &[&str]| run(name, targets, effects);
    for (targets, effects) in [
        (&["rust-std"][..], &[][..]),
        (&["rust-core", "rust-std"], &["overflow"]),
        (
            &["rust-core", "rust-std"],
            &["allocation", "overflow", "recursion"],
        ),
        (&[], &[]),
        (&[], &["allocation"]),
        (&["rust-std", "rust-core"], &[]),
        (&["rust-std", "rust-std"], &[]),
        (&["rust-core", "rust-core", "rust-std"], &[]),
        (&["rust-core", "rust-std"], &["overflow", "allocation"]),
        (&["rust-core", "rust-std"], &["allocation", "allocation"]),
        (&["rust-std"], &["recursion", "overflow", "allocation"]),
        (&["rust-wasm"], &[]),
        (&["rust-core", "rust-wasm"], &[]),
        (&["rust-wasm", "rust-std"], &[]),
        (&["rust-std"], &["io"]),
        (&["rust-std"], &["allocation", "io"]),
        (&["rust-std"], &["io", "overflow"]),
        (&["rust-wasm"], &["io"]),
        (&["rust-std", "rust-core"], &["io"]),
        (&["rust-wasm"], &["overflow", "allocation"]),
        (&["rust-std"], &["Allocation"]),
        (&["Rust-std"], &[]),
        (&["rust-std "], &[]),
        (&[""], &[]),
        (&["rust-std"], &[""]),
        (&["rust-cor", "rust-std"], &[]),
        (&["rust-core", "rust-std", "rust-stdx"], &[]),
        // Bytewise order: shorter first is not the rule.
        (&["a", "aa", "b"], &[]),
        (&["aa", "b"], &[]),
        (&["b", "aa"], &[]),
        (&["Z", "a"], &[]),
        (&["a", "Z"], &[]),
        (&["a", "\u{e9}"], &[]),
        (&["\u{e9}", "z"], &[]),
    ] {
        for name in ["root", "Main.checkedSum", "a`b", "r\u{e9}sum\u{e9}", ""] {
            out.push(rule(name, targets, effects));
        }
    }
    out
}

/// Seeded runs: lists over a pool of registered ids and near misses, against
/// the real registry and against small random registries.
fn seeded(count: usize, seed: u64) -> Vec<Run> {
    let pool = [
        "rust-core",
        "rust-std",
        "allocation",
        "overflow",
        "recursion",
        "a",
        "aa",
        "b",
        "",
        "Z",
        "rust-cor",
        "rust-std ",
        "\u{e9}",
        "io",
    ];
    let mut random = Seeded(seed);
    let mut out = Vec::new();
    let draw = |random: &mut Seeded, sorted: bool| {
        let length = random.below(5);
        let mut items: Vec<String> = (0..length)
            .map(|_| (*random.pick(&pool)).to_owned())
            .collect();
        if sorted {
            items.sort();
            items.dedup();
        }
        items
    };
    while out.len() < count {
        let sorted = random.chance(70);
        let targets = draw(&mut random, sorted);
        let effects = draw(&mut random, sorted);
        let name = format!("root{}", random.below(1000));
        let oracle = outcome(&name, &targets, &effects);
        let registry = registry().expect("the registry");
        let registered_targets: Vec<String> = if random.chance(80) {
            registry.targets.keys().cloned().collect()
        } else {
            let mut subset = draw(&mut random, true);
            subset.sort();
            subset
        };
        let registered_effects: Vec<String> = if random.chance(80) {
            registry.effects.keys().cloned().collect()
        } else {
            let mut subset = draw(&mut random, true);
            subset.sort();
            subset
        };
        // Against a random registry the oracle's registry is not the one the
        // root is given, so the expectation is the rule applied to that one.
        let expected = if registered_targets == registry.targets.keys().cloned().collect::<Vec<_>>()
            && registered_effects == registry.effects.keys().cloned().collect::<Vec<_>>()
        {
            oracle
        } else {
            expected_with(
                &name,
                &targets,
                &effects,
                &registered_targets,
                &registered_effects,
            )
        };
        out.push((
            vec![
                text(&name),
                texts(&targets),
                texts(&effects),
                texts(&registered_targets),
                texts(&registered_effects),
            ],
            expected,
        ));
    }
    out
}

/// The oracle's rule over registries it was not built with: the same
/// function, with the registry's membership tests replaced. The oracle reads
/// the embedded registry, so a registry that differs is checked by
/// restating its rule here, and the run is kept only where the embedded
/// registry gives the same answer to the membership questions asked.
fn expected_with(
    name: &str,
    targets: &[String],
    effects: &[String],
    registered_targets: &[String],
    registered_effects: &[String],
) -> Json {
    let ascending = |items: &[String]| items.windows(2).all(|pair| pair[0] < pair[1]);
    let refuse = |rest: String| {
        serde_json::to_value(Value::Error {
            value: Box::new(text(&format!("production root `{name}` {rest}"))),
        })
        .expect("a value is JSON")
    };
    if targets.is_empty() {
        return refuse("names no target".to_owned());
    }
    if !ascending(targets) {
        return refuse("targets are not sorted and unique".to_owned());
    }
    if !ascending(effects) {
        return refuse("effects are not sorted and unique".to_owned());
    }
    if let Some(unknown) = targets
        .iter()
        .find(|item| !registered_targets.contains(item))
    {
        return refuse(format!("names unregistered target `{unknown}`"));
    }
    if let Some(unknown) = effects
        .iter()
        .find(|item| !registered_effects.contains(item))
    {
        return refuse(format!("names unregistered effect `{unknown}`"));
    }
    serde_json::to_value(Value::Ok {
        value: Box::new(Value::Unit),
    })
    .expect("a value is JSON")
}

/// A defect planted in the source of `ProductionRegistry`.
struct Plant {
    label: &'static str,
    site: &'static str,
    replacement: &'static str,
}

const PLANTS: [Plant; 8] = [
    Plant {
        label: "a declaration with no target is accepted",
        site: "((prim length targets : Nat) =? 0)",
        replacement: "false",
    },
    Plant {
        label: "a duplicate is accepted as sorted",
        site: "    | Option.some last => (less last item)",
        replacement: "    | Option.some last => (not (less item last))",
    },
    Plant {
        label: "the order of the targets is not checked",
        site: "(if (ascending targets) then",
        replacement: "(if true then",
    },
    Plant {
        label: "the order of the effects is not checked",
        site: "(if (ascending effects) then",
        replacement: "(if true then",
    },
    Plant {
        label: "an unregistered effect is accepted",
        site: "(match (unregistered effects registeredEffects)",
        replacement: "(match (ctor Option.none [Bytes])",
    },
    Plant {
        label: "the last unregistered name is reported, not the first",
        site: "    | Option.some earliest => (ctor Option.some [Bytes] earliest)",
        replacement: "    | Option.some earliest => (if (member item registered) then (ctor Option.some [Bytes] earliest) else (ctor Option.some [Bytes] item))",
    },
    Plant {
        label: "the order is by length, not bytewise",
        site: "body: (prim equal (prim compare_bytes left right : Ordering) (prim compare_bytes 0x 0x01 : Ordering) : Bool),",
        replacement: "body: ((prim length left : Nat) <? (prim length right : Nat)),",
    },
    Plant {
        label: "an unregistered target is called an effect",
        site: "(prim utf8_encode \"target\" : Bytes) unknown)",
        replacement: "(prim utf8_encode \"effect\" : Bytes) unknown)",
    },
];

fn plants(detection: &[Run]) {
    for plant in &PLANTS {
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/ProductionRegistry.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/ProductionRegistry.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = lower(&checked, &planted, MODULE, "checkDeclaration");
        let disagreements = super::evaluate(&low, detection, 1);
        assert!(
            !disagreements.is_empty(),
            "{}: the planted defect is detected by the differential",
            plant.label
        );
        eprintln!("BS-90: plant detected: {}", plant.label);
    }
}

pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("ProductionRegistry"),
        "ProductionRegistry is a linked module of the project"
    );
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    let report = eligibility_report(&project, &build_id, MODULE);
    let rows = report["roots"].as_array().expect("roots");
    assert_eq!(rows.len(), 1, "checkDeclaration is the one root");
    assert_eq!(rows[0]["root"], format!("{MODULE}.checkDeclaration"));
    assert_eligible(&rows[0], "checkDeclaration", &["allocation"]);
    let declarations = committed();
    let mut runs: Vec<Run> = declarations
        .iter()
        .map(|(name, targets, effects)| {
            let targets: Vec<&str> = targets.iter().map(String::as_str).collect();
            let effects: Vec<&str> = effects.iter().map(String::as_str).collect();
            run(name, &targets, &effects)
        })
        .collect();
    let rule_runs = rules();
    let seeded_runs = seeded(SEEDED, 0x0905_A001);
    let counts = (declarations.len(), rule_runs.len(), seeded_runs.len());
    runs.extend(rule_runs);
    eprintln!(
        "BS-90: {} committed declarations, {} rule runs, {} seeded runs, {} runs ({:?})",
        counts.0,
        counts.1,
        counts.2,
        runs.len() + counts.2,
        started.elapsed()
    );
    if std::env::var_os("BS90_RECOUNT").is_none() {
        assert_eq!(
            (counts.0, counts.1, counts.2, runs.len() + counts.2),
            (COMMITTED, RULES, SEEDED, RUNS),
            "the committed declarations, the rule runs, the seeded runs, and all runs"
        );
    }
    // The negative projects' declarations are refused by the oracle, so both
    // kinds of outcome are in the corpus.
    let refused = runs
        .iter()
        .filter(|(_, outcome)| outcome["kind"] == "error")
        .count();
    assert!(
        refused > 0 && refused < runs.len(),
        "the corpus has accepted and refused declarations"
    );
    let checked = support::checked_project(&project);
    let low = lower(&checked, &project, MODULE, "checkDeclaration");
    let all: Vec<Run> = runs.iter().chain(&seeded_runs).cloned().collect();
    // On every host: the rendered rust-std package, compiled by that host's
    // pinned rustc, agrees with the oracle on every run.
    rustc_differential("BS-90", &[("checkDeclaration", &low, &all)], "runs");
    if !support::normative_host() {
        eprintln!("BS-90: platform-bound host: the evaluator and the planted defects run on the normative host");
        return;
    }
    let disagreements = super::evaluate(&low, &all, 5);
    assert!(
        disagreements.is_empty(),
        "the evaluator and the oracle disagree: {disagreements:?}"
    );
    eprintln!(
        "BS-90: the evaluator agrees on {} runs ({:?})",
        all.len(),
        started.elapsed()
    );
    plants(&all);
    if !support::lean_backed("BS-90") {
        return;
    }
    let exported = crate::production_export::exported("bootstrap");
    let bundle = exported.bundle();
    let mut realized = 0usize;
    for exported_root in &exported.output.result.roots {
        if exported_root.root != format!("{MODULE}.checkDeclaration") {
            continue;
        }
        let largest = exported_root
            .outputs
            .iter()
            .flat_map(|output| &output.certification.certificates)
            .map(|certificate| certificate.file.byte_length)
            .max()
            .expect("a certificate");
        eprintln!(
            "BS-90: {}: largest certificate part {largest} bytes",
            exported_root.root
        );
        assert!(
            largest <= CERTIFICATE_BUDGET,
            "{}: a certificate part of {largest} bytes is beyond {CERTIFICATE_BUDGET}, three quarters of max_file_bytes",
            exported_root.root
        );
        for output in &exported_root.outputs {
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
                serde_json::to_value(&low.program).expect("a program is JSON"),
                "checkDeclaration: the exported package realizes the program the differential ran"
            );
            realized += 1;
        }
    }
    assert_eq!(realized, 1, "one certified package for the root");
    let packages = crate::production_export::tree(&bundle.join("packages"));
    let committed_packages = crate::production_export::tree(
        &crate::production_export::fixture_dir("bootstrap").join("packages"),
    );
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
    eprintln!("BS-90: passed in {:?}", started.elapsed());
}
