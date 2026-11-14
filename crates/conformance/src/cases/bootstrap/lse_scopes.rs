//! `BS-51` (SPEC.md §17.23, §13.8): the scopes, the references, and the
//! outer shape of a LexLean Semantic Expression as language-1.2 source,
//! `LexLeanBoot.LseScopes`, held to the byte to `Lse::check_scopes`,
//! `Lse::referenced_consts`, `Lse::outer_explicit_binders`, and `Lse::result`.
//!
//! The roots read the expressions of `BS-25` and walk them with an explicit
//! work stack, so that none admits `recursion`; the host passes the length of
//! the expression's text, which bounds the walk. Each planted defect names the
//! one vector that catches it.

use std::collections::{BTreeMap, BTreeSet};

use lexlean::calculus::Value;
use lexlean::lexicon::lse::{parse, Lse};
use serde_json::Value as Json2;

use super::lexicon_lse::{drawn_lse, enc_lse};
use super::rust_validate::{eligibility_of, rustc_differential};
use super::{list, option, pair, text, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.LseScopes";
const ROOTS: [(&str, &[&str]); 3] = [
    ("checkScopes", &["allocation", "overflow"]),
    ("referencedConsts", &["allocation", "overflow"]),
    ("outerShape", &["allocation", "overflow"]),
];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// The depth the committed expressions and the seeded ones are read at.
const DEPTH: u64 = 64;
/// Seeded expressions under the evaluator, and under rustc.
const EVALUATOR_SEEDED: usize = 300;
const RUSTC_SEEDED: usize = 2_000;
const UNIVERSE_NAMES: &[&str] = &["u", "v", "w1"];

fn nat(value: u64) -> Value {
    Value::Nat {
        value: value.to_string(),
    }
}

/// The bound the host passes: the length of the expression's text.
fn bound(expression: &Lse) -> Value {
    nat(u64::try_from(expression.print(false).len()).expect("a length"))
}

fn scopes_run(expression: &Lse, universes: &[&str]) -> Run {
    let declared: BTreeSet<String> = universes.iter().map(|name| (*name).to_owned()).collect();
    let outcome = expression.check_scopes(&declared).err();
    (
        vec![
            enc_lse(expression),
            list(universes.iter().map(|name| text(name)).collect()),
            bound(expression),
        ],
        serde_json::to_value(option(outcome.as_deref().map(text))).expect("a value is JSON"),
    )
}

fn consts_run(expression: &Lse) -> Run {
    let found = expression
        .referenced_consts()
        .iter()
        .map(|id| pair(text(&id.package), text(&id.entry)))
        .collect();
    (
        vec![enc_lse(expression), bound(expression)],
        serde_json::to_value(list(found)).expect("a value is JSON"),
    )
}

fn shape_run(expression: &Lse) -> Run {
    let explicit = u64::try_from(expression.outer_explicit_binders()).expect("a count");
    let sort = matches!(expression.result(), Lse::SortProp | Lse::SortType(_));
    (
        vec![enc_lse(expression), bound(expression)],
        serde_json::to_value(pair(nat(explicit), Value::Bool { value: sort }))
            .expect("a value is JSON"),
    )
}

fn parsed(source: &str) -> Lse {
    parse(source, DEPTH).unwrap_or_else(|error| panic!("`{source}` parses: {error}"))
}

// ------------------------------------------------------------------ vectors

struct Vector {
    label: &'static str,
    root: usize,
    run: Run,
}

const P: &str = "(sort prop)";

#[allow(clippy::too_many_lines)]
fn named() -> Vec<Vector> {
    let mut out = Vec::new();
    let scoped: &[(&'static str, &str, &[&str])] = &[
        (
            "a closed signature",
            "(pi ((explicit a (const p::nat))) (const p::nat))",
            &[],
        ),
        ("a free local", "(local x)", &[]),
        (
            "a local bound by a pi",
            "(pi ((explicit x (sort prop))) (local x))",
            &[],
        ),
        (
            "a local used after its pi",
            "(app (const f::g) (pi ((explicit x (sort prop))) (local x)) (local x))",
            &[],
        ),
        (
            "a local used after its lam",
            "(app (lam ((explicit y (sort prop))) (local y)) (local y))",
            &[],
        ),
        (
            "a binder sees the binders before it",
            "(pi ((explicit a (sort prop)) (implicit b (local a))) (local b))",
            &[],
        ),
        (
            "a binder's type does not see itself",
            "(pi ((explicit a (local a))) (sort prop))",
            &[],
        ),
        (
            "a binder's type does not see the binders after it",
            "(pi ((explicit a (local b)) (explicit b (sort prop))) (sort prop))",
            &[],
        ),
        (
            "a let value does not see its name",
            "(let x (sort prop) (local x) (local x))",
            &[],
        ),
        (
            "a let type does not see its name",
            "(let x (local x) (sort prop) (local x))",
            &[],
        ),
        (
            "a let body sees its name",
            "(let x (sort prop) (sort prop) (local x))",
            &[],
        ),
        (
            "a let's name ends with it",
            "(app (const f::g) (let x (sort prop) (sort prop) (local x)) (local x))",
            &[],
        ),
        (
            "an inner binder shadows an outer one",
            "(pi ((explicit x (sort prop))) (lam ((explicit x (local x))) (local x)))",
            &[],
        ),
        ("a declared universe variable", "(sort (type u))", &["u"]),
        ("an undeclared universe variable", "(sort (type u))", &[]),
        ("a universe of a numeral", "(sort (type 3))", &[]),
        (
            "an undeclared variable inside a max",
            "(sort (type (max u v w1)))",
            &["u"],
        ),
        (
            "two undeclared variables of an imax",
            "(sort (type (imax v w1)))",
            &[],
        ),
        (
            "an undeclared variable under a succ",
            "(sort (type (succ (succ w1))))",
            &["u", "v"],
        ),
        (
            "an undeclared universe of a constant",
            "(const a::b (u v))",
            &["u"],
        ),
        (
            "the declared universes of a constant",
            "(const a::b (u (max v 0)))",
            &["u", "v"],
        ),
        (
            "a universe error after a local error",
            "(app (local x) (sort (type u)))",
            &[],
        ),
        (
            "a local error after a universe error",
            "(app (const a::b (w1)) (local x))",
            &[],
        ),
        (
            "a universe error in a binder type",
            "(pi ((explicit a (sort (type v)))) (local z))",
            &["u"],
        ),
        ("a natural literal", "(nat 42)", &[]),
        ("a proposition", P, &[]),
    ];
    for &(label, source, universes) in scoped {
        out.push(Vector {
            label,
            root: 0,
            run: scopes_run(&parsed(source), universes),
        });
    }
    for (label, source) in [
        ("no constant", "(sort prop)"),
        ("one constant", "(const lexlean.std.nat::nat)"),
        (
            "the function's constant before its arguments'",
            "(app (const f::g) (const a::b) (const c::d))",
        ),
        (
            "a nested application",
            "(app (app (const f::g) (const a::b)) (const c::d))",
        ),
        (
            "a constant in a binder type",
            "(pi ((explicit x (const a::b)) (implicit y (const c::d))) (const e::f))",
        ),
        (
            "the constants of a lam",
            "(lam ((explicit x (const a::b))) (app (const f::g) (local x)))",
        ),
        (
            "the constants of a let",
            "(let x (const a::b) (const c::d) (const e::f))",
        ),
        ("a constant repeated", "(app (const a::b) (const a::b))"),
        ("a constant with universes", "(const a::b (u 1))"),
    ] {
        out.push(Vector {
            label,
            root: 1,
            run: consts_run(&parsed(source)),
        });
    }
    for (label, source) in [
        ("a pi of mixed binders", "(pi ((implicit a (sort prop)) (explicit b (local a)) (instance c (sort prop)) (explicit d (sort prop))) (local b))"),
        ("a pi into a sort", "(pi ((explicit a (sort prop))) (sort (type 0)))"),
        ("a pi into a proposition", "(pi ((explicit a (sort prop))) (sort prop))"),
        ("a pi of implicit binders only", "(pi ((implicit a (sort prop))) (local a))"),
        ("a pi into a pi", "(pi ((explicit a (sort prop))) (pi ((explicit b (sort prop))) (sort prop)))"),
        ("a lam is no pi", "(lam ((explicit a (sort prop))) (sort prop))"),
        ("a sort", "(sort (type 1))"),
        ("a constant", "(const a::b)"),
    ] {
        out.push(Vector {
            label,
            root: 2,
            run: shape_run(&parsed(source)),
        });
    }
    out
}

// --------------------------------------------------------------- committed

/// Every signature and defined value of the committed lexicon entries, with
/// the universes its entry declares.
fn committed() -> Vec<(String, Lse, Vec<String>)> {
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
        let shown = path.to_string_lossy().replace('\\', "/");
        if !entry.file_type().is_file() || !shown.ends_with(".toml") || !shown.contains("/entries/")
        {
            continue;
        }
        let Ok(content) = std::fs::read_to_string(path) else {
            continue;
        };
        let Ok(table) = content.parse::<toml::Table>() else {
            continue;
        };
        let universes: Vec<String> = table
            .get("universes")
            .and_then(toml::Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|name| name.as_str().map(str::to_owned))
            .collect();
        let mut texts: Vec<&str> = Vec::new();
        if let Some(signature) = table.get("signature").and_then(toml::Value::as_str) {
            texts.push(signature);
        }
        if let Some(value) = table
            .get("denotation")
            .and_then(|denotation| denotation.get("value"))
            .and_then(toml::Value::as_str)
        {
            texts.push(value);
        }
        for source in texts {
            if let Ok(expression) = parse(source, DEPTH) {
                out.push((shown.clone(), expression, universes.clone()));
            }
        }
    }
    out
}

/// `count` seeded expressions: each checked under a seeded set of declared
/// universes, its references, and its outer shape.
fn seeded_runs(count: usize) -> [Vec<Run>; 3] {
    let mut seeded = Seeded(0x0B55_0051);
    let mut runs: [Vec<Run>; 3] = Default::default();
    for _ in 0..count {
        let depth = 1 + seeded.below(4);
        let expression = drawn_lse(&mut seeded, depth);
        let universes: Vec<&str> = UNIVERSE_NAMES
            .iter()
            .copied()
            .filter(|_| seeded.chance(60))
            .collect();
        runs[0].push(scopes_run(&expression, &universes));
        runs[1].push(consts_run(&expression));
        runs[2].push(shape_run(&expression));
    }
    runs
}

// ------------------------------------------------------------------- plants

struct Plant {
    label: &'static str,
    root: usize,
    site: &'static str,
    replacement: &'static str,
    vector: &'static str,
}

const PLANTS: &[Plant] = &[
    Plant {
        label: "a local is in scope everywhere",
        root: 0,
        site: "| LexiconLse::Lse.LLocal name => (if (member (project st scope) name) then",
        replacement: "| LexiconLse::Lse.LLocal name => (if true then",
        vector: "a free local",
    },
    Plant {
        label: "a binder's type sees itself",
        root: 0,
        site: "(cons (ctor SWork.SExpr ty) (cons (ctor SWork.SBind name) (cons (ctor SWork.SExpr more) rest))",
        replacement: "(cons (ctor SWork.SBind name) (cons (ctor SWork.SExpr ty) (cons (ctor SWork.SExpr more) rest))",
        vector: "a binder's type does not see itself",
    },
    Plant {
        label: "a let value sees its name",
        root: 0,
        site: "(cons (ctor SWork.SExpr value) (cons (ctor SWork.SBind name)",
        replacement: "(cons (ctor SWork.SBind name) (cons (ctor SWork.SExpr value)",
        vector: "a let value does not see its name",
    },
    Plant {
        label: "a binder's scope outlives its pi",
        root: 0,
        site: "scope := (dropFront (project st scope) (prim subtract (project st depth) wanted : Nat)), depth := wanted",
        replacement: "scope := (project st scope), depth := (project st depth)",
        vector: "a local used after its pi",
    },
    Plant {
        label: "an undeclared universe variable passes",
        root: 0,
        site: "(if (member universes name) then (scGo st rest) else",
        replacement: "(if true then (scGo st rest) else",
        vector: "an undeclared universe variable",
    },
    Plant {
        label: "the arguments of a max are skipped",
        root: 0,
        site: "| LexiconLse::Universe.UMax items => (scGo st (cons (ctor SWork.SUni items) rest))",
        replacement: "| LexiconLse::Universe.UMax items => (scGo st rest)",
        vector: "an undeclared variable inside a max",
    },
    Plant {
        label: "an imax checks its right first",
        root: 0,
        site: "(scGo st (cons (ctor SWork.SUni a) (cons (ctor SWork.SUni b) rest)))",
        replacement: "(scGo st (cons (ctor SWork.SUni b) (cons (ctor SWork.SUni a) rest)))",
        vector: "two undeclared variables of an imax",
    },
    Plant {
        label: "an application's arguments are checked first",
        root: 0,
        site: "| LexiconLse::Lse.LApp f args => (scGo st (cons (ctor SWork.SExpr f) (cons (ctor SWork.SExpr args) rest)))",
        replacement: "| LexiconLse::Lse.LApp f args => (scGo st (cons (ctor SWork.SExpr args) (cons (ctor SWork.SExpr f) rest)))",
        vector: "a universe error after a local error",
    },
    Plant {
        label: "the universes of a constant are skipped",
        root: 0,
        site: "| LexiconLse::Lse.LConst p e us => (scGo st (cons (ctor SWork.SUni us) rest))",
        replacement: "| LexiconLse::Lse.LConst p e us => (scGo st rest)",
        vector: "an undeclared universe of a constant",
    },
    Plant {
        label: "an application's arguments are referenced first",
        root: 1,
        site: "| LexiconLse::Lse.LApp f args => (rcGo st (cons f (cons args rest)))",
        replacement: "| LexiconLse::Lse.LApp f args => (rcGo st (cons args (cons f rest)))",
        vector: "the function's constant before its arguments'",
    },
    Plant {
        label: "a let's value is skipped",
        root: 1,
        site: "(rcGo st (cons ty (cons value (cons body rest))))",
        replacement: "(rcGo st (cons ty (cons body rest)))",
        vector: "the constants of a let",
    },
    Plant {
        label: "a binder's type is skipped",
        root: 1,
        site: "| LexiconLse::Lse.LBinder mode name ty more => (rcGo st (cons ty (cons more rest)))",
        replacement: "| LexiconLse::Lse.LBinder mode name ty more => (rcGo st (cons more rest))",
        vector: "a constant in a binder type",
    },
    Plant {
        label: "every binder is explicit",
        root: 2,
        site: "(if (mode =? 0) then ((second state) + 1) else (second state))",
        replacement: "((second state) + 1)",
        vector: "a pi of mixed binders",
    },
    Plant {
        label: "a pi's result is never a sort",
        root: 2,
        site: "(isSort body))",
        replacement: "false)",
        vector: "a pi into a sort",
    },
];

fn plants(vectors: &[Vector]) {
    let mut missed = Vec::new();
    for plant in PLANTS {
        let vector = vectors
            .iter()
            .find(|vector| vector.label == plant.vector)
            .unwrap_or_else(|| panic!("{}: no vector `{}`", plant.label, plant.vector));
        assert_eq!(
            vector.root, plant.root,
            "{}: the vector is a run of the root planted",
            plant.label
        );
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/LseScopes.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/LseScopes.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[plant.root].0);
        match super::evaluate(&low, std::slice::from_ref(&vector.run), 1).first() {
            Some(first) => eprintln!(
                "BS-51: plant detected: {}: {}",
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
            "BS-51: {}: largest certificate part {largest} bytes",
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

/// Every error `check_scopes` can state, by a part of its message (R2).
const REFUSALS: &[&str] = &["is not in scope", "is not declared"];

#[allow(clippy::too_many_lines)]
pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("LseScopes")
            && checked_units.units.contains_key("LexiconLse"),
        "LseScopes and LexiconLse are linked modules of the project"
    );
    eligibility_of(&project, "LseScopes", MODULE, &ROOTS);

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let mut by_root: [Vec<Run>; 3] = Default::default();
    for vector in &vectors {
        by_root[vector.root].push(vector.run.clone());
    }
    let messages: Vec<String> = by_root[0].iter().map(|run| run.1.to_string()).collect();
    for refusal in REFUSALS {
        assert!(
            messages
                .iter()
                .any(|message| message.contains(&super::hex(refusal.as_bytes()))),
            "no vector reaches the error `{refusal}`"
        );
    }

    let corpus = committed();
    assert!(
        corpus.len() >= 200,
        "the committed entries carry {} signatures and values",
        corpus.len()
    );
    let mut committed_runs: [Vec<Run>; 3] = Default::default();
    for (path, expression, universes) in &corpus {
        let declared: Vec<&str> = universes.iter().map(String::as_str).collect();
        let run = scopes_run(expression, &declared);
        assert_eq!(
            run.1,
            serde_json::to_value(Value::None).expect("a value is JSON"),
            "{path}: a committed expression is in scope"
        );
        committed_runs[0].push(run);
        committed_runs[1].push(consts_run(expression));
        committed_runs[2].push(shape_run(expression));
    }
    let normative = support::normative_host();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (seeded_runs(EVALUATOR_SEEDED), seeded_runs(RUSTC_SEEDED))
    } else {
        (Default::default(), Default::default())
    };
    eprintln!(
        "BS-51: {} vectors, {} committed expressions, {} and {} seeded expressions under the evaluator and rustc ({:?})",
        vectors.len(),
        corpus.len(),
        evaluator_seeded[0].len(),
        rustc_seeded[0].len(),
        started.elapsed()
    );

    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, super::Lowered> = ROOTS
        .iter()
        .map(|(root, _)| (*root, super::lower(&checked, &project, MODULE, root)))
        .collect();
    if normative {
        for (index, (root, _)) in ROOTS.iter().enumerate() {
            let mut runs: Vec<Run> = by_root[index].clone();
            runs.extend(committed_runs[index].iter().cloned());
            runs.extend(evaluator_seeded[index].iter().cloned());
            let disagreements = super::evaluate(&lowered[root], &runs, 5);
            assert!(
                disagreements.is_empty(),
                "{root} and the Rust oracle disagree: {disagreements:?}"
            );
            eprintln!(
                "BS-51: {root}: the evaluator agrees on {} runs ({:?})",
                runs.len(),
                started.elapsed()
            );
        }
        plants(&vectors);
    } else {
        eprintln!("BS-51: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled: [Vec<Run>; 3] = Default::default();
    for (index, runs) in compiled.iter_mut().enumerate() {
        runs.append(&mut by_root[index]);
        runs.append(&mut committed_runs[index]);
        runs.extend(rustc_seeded[index].iter().cloned());
    }
    let suites: Vec<(&str, &super::Lowered, &[Run])> = ROOTS
        .iter()
        .enumerate()
        .map(|(index, (root, _))| (*root, &lowered[root], compiled[index].as_slice()))
        .collect();
    let total = rustc_differential("BS-51", &suites, "oracle");
    eprintln!("BS-51: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-51") {
        return;
    }
    export(&lowered);
    eprintln!("BS-51: passed in {:?}", started.elapsed());
}
