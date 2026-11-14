//! `BS-54` (SPEC.md §17.23, §13.1, §12.2): a `package@version` reference as
//! language-1.2 source, `LexLeanBoot.PackageRef`, held to the byte to
//! `lexlean::lexicon::package::PackageRef::parse`: the package ID (held to
//! `LexiconLse::isPackageId` of `BS-25`) and the exact version, a full
//! SemVer 2.0.0 version string (not only the plain `X.Y.Z` every committed
//! reference happens to write), since the oracle is `semver::Version::parse`
//! itself and a project's own `imports` may write a pre-release or build
//! metadata. Each planted defect is detected by the one vector it names.

use std::collections::BTreeMap;

use lexlean::calculus::Value;
use lexlean::lexicon::package::PackageRef as OraclePackageRef;
use serde_json::Value as Json2;

use super::rust_validate::{eligibility_of, rustc_differential};
use super::{option, text, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.PackageRef";
const ROOTS: [(&str, &[&str]); 1] = [("parsePackageRef", &["allocation", "overflow"])];

const CERTIFICATE_BUDGET: u64 = 3_145_728;
const EVALUATOR_SEEDED: usize = 300;
const RUSTC_SEEDED: usize = 2_000;

fn encode(reference: &OraclePackageRef) -> Value {
    Value::Adt {
        constructor: 0,
        fields: vec![text(&reference.package), text(&reference.version)],
    }
}

fn parse_run(input: &str) -> Run {
    let expected = OraclePackageRef::parse(input).ok();
    (
        vec![text(input)],
        serde_json::to_value(option(expected.as_ref().map(encode))).expect("a value is JSON"),
    )
}

// ------------------------------------------------------------------ vectors

struct Vector {
    label: &'static str,
    run: Run,
}

#[allow(clippy::too_many_lines)]
fn named() -> Vec<Vector> {
    [
        ("a plain version", "a@1.0.0"),
        ("a dotted package id", "lexlean.std.nat@1.2.0"),
        ("a pre-release", "a@1.0.0-alpha"),
        ("a dotted pre-release", "a@1.0.0-alpha.1"),
        ("a numeric pre-release identifier", "a@1.0.0-0"),
        (
            "a pre-release identifier with an internal hyphen",
            "a@1.0.0-alpha-beta",
        ),
        ("build metadata", "a@1.0.0+build"),
        ("dotted build metadata", "a@1.0.0+build.2"),
        ("pre-release and build metadata", "a@1.0.0-alpha.1+build.2"),
        ("build metadata with a leading zero is fine", "a@1.0.0+01"),
        ("the largest ordinary version", "a@999999.999999.999999"),
        ("a leading zero in major is refused", "a@01.0.0"),
        ("a leading zero in minor is refused", "a@1.00.0"),
        ("a leading zero in patch is refused", "a@1.0.00"),
        (
            "a leading zero in a numeric pre-release identifier is refused",
            "a@1.0.0-01",
        ),
        ("only two core parts is refused", "a@1.0"),
        ("four core parts is refused", "a@1.0.0.0"),
        ("an empty pre-release is refused", "a@1.0.0-"),
        ("an empty pre-release segment is refused", "a@1.0.0-alpha.."),
        ("an empty build is refused", "a@1.0.0+"),
        ("an empty build segment is refused", "a@1.0.0+build.."),
        (
            "a non-ascii-alnum pre-release byte is refused",
            "a@1.0.0-alpha_beta",
        ),
        ("a non-decimal core part is refused", "a@1.x.0"),
        ("no at-sign is refused", "a1.0.0"),
        ("an empty package id is refused", "@1.0.0"),
        ("an invalid package id is refused", "A@1.0.0"),
        (
            "a second at-sign is read as part of the version and refused",
            "a@b@1.0.0",
        ),
        ("an empty version is refused", "a@"),
    ]
    .into_iter()
    .map(|(label, input)| Vector {
        label,
        run: parse_run(input),
    })
    .collect()
}

// --------------------------------------------------------------- committed

/// Every `package@version` text the compiler's own files write: a
/// `\useglossary{...}` directive, or an `imports` array entry of a
/// `lexicon.toml` manifest.
fn committed() -> Vec<String> {
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
        if !entry.file_type().is_file() {
            continue;
        }
        if shown.ends_with(".lex.tex") {
            let Ok(content) = std::fs::read_to_string(path) else {
                continue;
            };
            let mut rest = content.as_str();
            while let Some(at) = rest.find("\\useglossary{") {
                rest = &rest[at + "\\useglossary{".len()..];
                if let Some(close) = rest.find('}') {
                    out.push(rest[..close].to_owned());
                    rest = &rest[close..];
                } else {
                    break;
                }
            }
        } else if shown.ends_with("/lexicon.toml") && !shown.contains("/tests/") {
            let Ok(content) = std::fs::read_to_string(path) else {
                continue;
            };
            let Ok(table) = content.parse::<toml::Table>() else {
                continue;
            };
            if let Some(imports) = table.get("imports").and_then(toml::Value::as_array) {
                for item in imports {
                    if let Some(text) = item.as_str() {
                        out.push(text.to_owned());
                    }
                }
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

// -------------------------------------------------------------------- seeds

const PACKAGE_WORDS: &[&str] = &["lexlean", "std", "nat", "core", "a", "a.b", "a.b.c"];
const IDENT_CHARS: &[u8] = b"abcdefghij0123456789-";

fn seeded_identifier(seeded: &mut Seeded) -> String {
    let len = 1 + seeded.below(5);
    (0..len)
        .map(|_| *seeded.pick(IDENT_CHARS) as char)
        .collect()
}

fn seeded_numeric_core_part(seeded: &mut Seeded) -> String {
    if seeded.chance(10) {
        // occasionally a leading zero, which the oracle refuses
        format!("0{}", seeded.below(10))
    } else if seeded.chance(20) {
        "0".to_owned()
    } else {
        (1 + seeded.below(1000)).to_string()
    }
}

/// A seeded `package@version` text, in and out of the subset in roughly
/// equal measure: a valid package id with a version built from random
/// core parts and an optional pre-release and build, each occasionally
/// malformed (an empty segment, a leading-zero numeric identifier, a
/// fourth core part, a missing at-sign).
fn seeded_text(seeded: &mut Seeded) -> String {
    if seeded.chance(5) {
        return seeded_identifier(seeded);
    }
    let package = *seeded.pick(PACKAGE_WORDS);
    let mut version = format!(
        "{}.{}.{}",
        seeded_numeric_core_part(seeded),
        seeded_numeric_core_part(seeded),
        seeded_numeric_core_part(seeded)
    );
    if seeded.chance(10) {
        version.push_str(&format!(".{}", seeded_numeric_core_part(seeded)));
    }
    if seeded.chance(40) {
        let count = 1 + seeded.below(3);
        let parts: Vec<String> = (0..count).map(|_| seeded_identifier(seeded)).collect();
        version.push('-');
        version.push_str(&parts.join("."));
    }
    if seeded.chance(40) {
        let count = 1 + seeded.below(3);
        let parts: Vec<String> = (0..count).map(|_| seeded_identifier(seeded)).collect();
        version.push('+');
        version.push_str(&parts.join("."));
    }
    if seeded.chance(90) {
        format!("{package}@{version}")
    } else {
        version
    }
}

fn seeded_runs(count: usize) -> Vec<Run> {
    let mut seeded = Seeded(0x0B55_0054);
    (0..count)
        .map(|_| parse_run(&seeded_text(&mut seeded)))
        .collect()
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
        label: "the package id is never checked",
        site: "(if (LexiconLse::isPackageId package) then",
        replacement: "(if true then",
        vector: "an invalid package id is refused",
    },
    Plant {
        label: "a fourth core part is accepted",
        site: "(if ((segCount parts) =? 3) then",
        replacement: "(if ((segCount parts) <=? 4) then",
        vector: "four core parts is refused",
    },
    Plant {
        label: "a leading zero in a core part is accepted",
        site: "(if ok then (LexLeanStd.Numeral::canonicalDecimal segment) else false)) true segments : Bool),\n  axioms: [Quot.sound, propext],\n  executable: true\n}\n\ndefinition{\n  name: allPrereleaseSegments,",
        replacement: "(if ok then (isAllDigits segment) else false)) true segments : Bool),\n  axioms: [Quot.sound, propext],\n  executable: true\n}\n\ndefinition{\n  name: allPrereleaseSegments,",
        vector: "a leading zero in major is refused",
    },
    Plant {
        label: "an empty pre-release or build segment is accepted",
        site: "(if (n =? 0) then false else ((alnumHyphenEnd segment) =? n))",
        replacement: "(if (n =? 0) then true else ((alnumHyphenEnd segment) =? n))",
        vector: "an empty build is refused",
    },
    Plant {
        label: "a leading zero in a numeric pre-release identifier is accepted",
        site: "(if (isAllDigits segment) then (LexLeanStd.Numeral::canonicalDecimal segment) else true)",
        replacement: "true",
        vector: "a leading zero in a numeric pre-release identifier is refused",
    },
];

fn plants(vectors: &[Vector]) {
    let mut missed = Vec::new();
    for plant in PLANTS {
        let vector = vectors
            .iter()
            .find(|vector| vector.label == plant.vector)
            .unwrap_or_else(|| panic!("{}: no vector `{}`", plant.label, plant.vector));
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/PackageRef.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/PackageRef.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[0].0);
        match super::evaluate(&low, std::slice::from_ref(&vector.run), 1).first() {
            Some(first) => eprintln!(
                "BS-54: plant detected: {}: {}",
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

// --------------------------------------------------------------------- case

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
            "BS-54: {}: largest certificate part {largest} bytes",
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
        checked_units.units.contains_key("PackageRef"),
        "PackageRef is a linked module of the project"
    );
    eligibility_of(&project, "PackageRef", MODULE, &ROOTS);

    let vectors = named();
    let labels: std::collections::BTreeSet<&str> =
        vectors.iter().map(|vector| vector.label).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let refused = vectors
        .iter()
        .filter(|vector| vector.run.1 == serde_json::to_value(Value::None).expect("a value"))
        .count();
    assert!(refused >= 10, "at least ten vectors are refused: {refused}");

    let corpus = committed();
    assert!(
        corpus.len() >= 10,
        "the committed files write {} package references",
        corpus.len()
    );
    for text in &corpus {
        assert!(
            OraclePackageRef::parse(text).is_ok(),
            "{text}: a committed package reference is valid"
        );
    }
    let committed_runs: Vec<Run> = corpus.iter().map(|text| parse_run(text)).collect();
    eprintln!(
        "BS-54: {} vectors, {} committed references ({:?})",
        vectors.len(),
        corpus.len(),
        started.elapsed()
    );

    let normative = support::normative_host();
    let evaluator_seeded = if normative {
        seeded_runs(EVALUATOR_SEEDED)
    } else {
        Vec::new()
    };
    let rustc_seeded = if normative {
        seeded_runs(RUSTC_SEEDED)
    } else {
        Vec::new()
    };

    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, super::Lowered> = ROOTS
        .iter()
        .map(|(root, _)| (*root, super::lower(&checked, &project, MODULE, root)))
        .collect();
    let low = &lowered["parsePackageRef"];

    if normative {
        let mut runs: Vec<Run> = vectors.iter().map(|vector| vector.run.clone()).collect();
        runs.extend(committed_runs.iter().cloned());
        runs.extend(evaluator_seeded.iter().cloned());
        let disagreements = super::evaluate(low, &runs, 5);
        assert!(
            disagreements.is_empty(),
            "parsePackageRef and the oracle disagree: {disagreements:?}"
        );
        eprintln!(
            "BS-54: parsePackageRef: the evaluator agrees on {} runs ({:?})",
            runs.len(),
            started.elapsed()
        );
        plants(&vectors);
    } else {
        eprintln!("BS-54: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled: Vec<Run> = vectors.into_iter().map(|vector| vector.run).collect();
    compiled.extend(committed_runs);
    compiled.extend(rustc_seeded);
    let suites: Vec<(&str, &super::Lowered, &[Run])> =
        vec![("parsePackageRef", low, compiled.as_slice())];
    let total = rustc_differential("BS-54", &suites, "PackageRef::parse");
    eprintln!("BS-54: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-54") {
        return;
    }
    export(&lowered);
    eprintln!("BS-54: passed in {:?}", started.elapsed());
}
