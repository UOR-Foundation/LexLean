//! The test inventory behind RP-13 (SPEC.md §9.2): the ordinary `test` phase
//! and the `bdd` phase partition the tests of the workspace.
//!
//! `just vv` splits what `cargo test --workspace --all-features` used to run
//! in one piece: `test` runs every crate but `repo-conformance`, and `bdd` is
//! the sole owner of `repo-conformance`'s complete suite. A split is only
//! as good as the proof that it neither lost a test nor kept one twice, and
//! the proof has to be about the tests themselves, not about the flags: the
//! lists below come from `cargo test <args> -- --list` for the whole
//! workspace and for each phase, and [`compare`] requires the phases'
//! lists to be disjoint and their union to equal the whole.
//!
//! The `bdd` phase itself runs in CI as the parts of `.github/bdd-parts.json`
//! (§9.4), each selected by the harness's own filters; [`compare_parts`]
//! requires the parts' lists, made with those filters, to be disjoint and
//! their union to equal the `bdd` phase's list, so that every test of the
//! phase runs in exactly one part.

use std::collections::{BTreeMap, BTreeSet};

/// The arguments of the command whose coverage the two phases must equal:
/// the single `cargo test` that `vv` ran before the split.
pub const REFERENCE: [&str; 2] = ["--workspace", "--all-features"];

/// The package whose suite the `bdd` phase owns whole.
pub const OWNED_PACKAGE: &str = "repo-conformance";

/// One listed test, identified by the target that holds it and its name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Entry {
    /// The test target: its description (`unittests src/lib.rs`,
    /// `tests/bdd.rs`, `Doc-tests lexlean`) and its crate, the compiled
    /// binary's name without the hash `cargo` adds to it.
    pub target: String,
    /// The test's path as the harness lists it.
    pub name: String,
}

/// The crate name of a test binary, from its path: the file name without the
/// `-<hash>` that `cargo` appends.
fn crate_of(binary: &str) -> String {
    let file = binary.rsplit(['/', '\\']).next().unwrap_or(binary);
    let file = file.strip_suffix(".exe").unwrap_or(file);
    match file.rsplit_once('-') {
        Some((stem, hash))
            if hash.len() >= 8 && hash.bytes().all(|byte| byte.is_ascii_hexdigit()) =>
        {
            stem.to_owned()
        }
        _ => file.to_owned(),
    }
}

/// Parse the output of `cargo test … -- --list`.
///
/// Every target `cargo` runs is introduced by a `Running <description>
/// (<binary>)` or `Doc-tests <crate>` line, and the harness lists each test as
/// `<name>: test`. A target with no test is still recorded (with no entry), so
/// that a target that stops being run is a difference of the inventory and not
/// a silence.
///
/// # Errors
///
/// A listed test before any target header, which means the output is not what
/// this parser understands.
pub fn parse(output: &str) -> Result<Parsed, String> {
    let mut targets: BTreeSet<String> = BTreeSet::new();
    let mut entries: BTreeSet<Entry> = BTreeSet::new();
    let mut current: Option<String> = None;
    for line in output.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("Running ") {
            // `Running unittests src/lib.rs (target/debug/deps/lexlean-<hash>)`
            // — the parenthesis holds the binary, and a description can hold
            // spaces, so the split is at the last ` (`.
            let (description, binary) = rest
                .rsplit_once(" (")
                .map(|(description, binary)| (description, binary.trim_end_matches(')')))
                .ok_or_else(|| format!("unrecognized target line `{trimmed}`"))?;
            let target = format!("{description} [{}]", crate_of(binary));
            targets.insert(target.clone());
            current = Some(target);
        } else if let Some(rest) = trimmed.strip_prefix("Doc-tests ") {
            let target = format!("Doc-tests [{}]", rest.trim());
            targets.insert(target.clone());
            current = Some(target);
        } else if let Some(name) = trimmed
            .strip_suffix(": test")
            .or_else(|| trimmed.strip_suffix(": benchmark"))
        {
            let target = current
                .clone()
                .ok_or_else(|| format!("test `{name}` listed before any target"))?;
            entries.insert(Entry {
                target,
                name: name.to_owned(),
            });
        }
    }
    Ok(Parsed { targets, entries })
}

/// What a listing holds.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Parsed {
    /// Every target that was run.
    pub targets: BTreeSet<String>,
    /// Every test of every target.
    pub entries: BTreeSet<Entry>,
}

impl Parsed {
    /// How many tests each target lists.
    #[must_use]
    pub fn counts(&self) -> BTreeMap<&str, usize> {
        let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
        for target in &self.targets {
            counts.insert(target, 0);
        }
        for entry in &self.entries {
            *counts.entry(&entry.target).or_insert(0) += 1;
        }
        counts
    }
}

/// The result of a successful comparison.
#[derive(Debug, PartialEq, Eq)]
pub struct Summary {
    /// Tests in the whole workspace listing.
    pub workspace: usize,
    /// Tests in the ordinary phase.
    pub ordinary: usize,
    /// Tests in the `bdd` phase.
    pub owned: usize,
}

/// The crate of a target, as [`parse`] recorded it.
fn crate_of_target(target: &str) -> &str {
    target
        .rsplit_once('[')
        .map_or(target, |(_, rest)| rest.trim_end_matches(']'))
}

/// Require the ordinary phase and the `bdd` phase to be a partition of the
/// workspace listing, with the owned package's whole suite in `bdd`.
///
/// `owned_crates` names the binaries of the owned package's test targets
/// (its library, and each integration test target, which `cargo` names after
/// the target and not the package), as `cargo metadata` reports them.
///
/// # Errors
///
/// A test the phases lost, one they ran twice, one the workspace never had,
/// a target of the owned package in the ordinary phase, or a target of the
/// owned package missing from `bdd`.
pub fn compare(
    workspace: &Parsed,
    ordinary: &Parsed,
    owned: &Parsed,
    owned_crates: &BTreeSet<String>,
) -> Result<Summary, String> {
    if workspace.entries.is_empty() {
        return Err(
            "RP-13: the workspace lists no test at all; the inventory proves nothing".to_owned(),
        );
    }
    let twice: Vec<&Entry> = ordinary.entries.intersection(&owned.entries).collect();
    if let Some(first) = twice.first() {
        return Err(format!(
            "RP-13: {} test(s) run in both phases, e.g. `{}` of {}",
            twice.len(),
            first.name,
            first.target
        ));
    }
    let union: BTreeSet<&Entry> = ordinary.entries.iter().chain(&owned.entries).collect();
    let whole: BTreeSet<&Entry> = workspace.entries.iter().collect();
    if let Some(lost) = whole.difference(&union).next() {
        let count = whole.difference(&union).count();
        return Err(format!(
            "RP-13: {count} test(s) of the workspace run in neither phase, e.g. `{}` of {}",
            lost.name, lost.target
        ));
    }
    if let Some(extra) = union.difference(&whole).next() {
        return Err(format!(
            "RP-13: a phase runs `{}` of {}, which the workspace does not list",
            extra.name, extra.target
        ));
    }
    if owned_crates.is_empty() {
        return Err(format!(
            "RP-13: no test target of {OWNED_PACKAGE} is known; the inventory proves nothing about its ownership"
        ));
    }
    let in_owned_package = |target: &str| owned_crates.contains(crate_of_target(target));
    if let Some(target) = ordinary
        .targets
        .iter()
        .find(|target| in_owned_package(target))
    {
        return Err(format!(
            "RP-13: the ordinary phase runs {target}; `bdd` is the sole owner of the {OWNED_PACKAGE} suite"
        ));
    }
    let owned_targets: BTreeSet<&String> = owned.targets.iter().collect();
    for target in workspace
        .targets
        .iter()
        .filter(|target| in_owned_package(target))
    {
        if !owned_targets.contains(target) {
            return Err(format!(
                "RP-13: `bdd` does not run {target}, a target of the {OWNED_PACKAGE} suite"
            ));
        }
    }
    // A `conformance_<id>` test must run exactly once, which the disjointness
    // above already says; that there are some at all is what keeps the whole
    // comparison from being vacuous for the owned package.
    if !owned
        .entries
        .iter()
        .any(|entry| entry.name.starts_with("conformance_"))
    {
        return Err("RP-13: `bdd` lists no conformance_<id> test".to_owned());
    }
    Ok(Summary {
        workspace: whole.len(),
        ordinary: ordinary.entries.len(),
        owned: owned.entries.len(),
    })
}

/// Require the parts of the `bdd` plan to partition the `bdd` phase (§9.4):
/// each part's listing, made with that part's harness filters, disjoint from
/// every other part's, their union the whole phase's listing, and every test a
/// named part names among the tests that part lists.
///
/// `parts` holds each part's name and listing, in the plan's order.
///
/// # Errors
///
/// A test two parts run, a test of `bdd` no part runs, a test a part runs that
/// `bdd` does not list, or a named test its part does not list.
pub fn compare_parts(
    owned: &Parsed,
    parts: &[(String, Parsed)],
    plan: &crate::bdd_parts::Plan,
) -> Result<usize, String> {
    use crate::bdd_parts::PLAN_FILE;
    let mut runner: BTreeMap<&Entry, &str> = BTreeMap::new();
    for (name, listed) in parts {
        for entry in &listed.entries {
            if let Some(first) = runner.insert(entry, name) {
                return Err(format!(
                    "RP-13: `{}` of {} is run by the parts `{first}` and `{name}` of {PLAN_FILE}",
                    entry.name, entry.target
                ));
            }
        }
    }
    let whole: BTreeSet<&Entry> = owned.entries.iter().collect();
    let union: BTreeSet<&Entry> = runner.keys().copied().collect();
    if let Some(lost) = whole.difference(&union).next() {
        let count = whole.difference(&union).count();
        return Err(format!(
            "RP-13: {count} test(s) of `bdd` run in no part of {PLAN_FILE}, e.g. `{}` of {}",
            lost.name, lost.target
        ));
    }
    if let Some(extra) = union.difference(&whole).next() {
        return Err(format!(
            "RP-13: the part `{}` of {PLAN_FILE} runs `{}` of {}, which `bdd` does not list",
            runner[extra], extra.name, extra.target
        ));
    }
    for (name, ids) in &plan.parts {
        let listed: BTreeSet<&str> = parts
            .iter()
            .filter(|(part, _)| part == name)
            .flat_map(|(_, listed)| listed.entries.iter().map(|entry| entry.name.as_str()))
            .collect();
        for id in ids {
            let test = crate::bdd_parts::test_name(id);
            if !listed.contains(test.as_str()) {
                return Err(format!(
                    "RP-13: the part `{name}` of {PLAN_FILE} names `{id}`, and runs no test `{test}`"
                ));
            }
        }
    }
    Ok(parts.len())
}

/// The arguments of a recipe line `cargo test <args…>`.
///
/// # Errors
///
/// A line that is not a `cargo test` invocation, or one that already carries
/// `--` (the inventory appends its own harness arguments after it).
pub fn recipe_arguments(line: &str) -> Result<Vec<String>, String> {
    let rest = line
        .strip_prefix("cargo test ")
        .ok_or_else(|| format!("`{line}` is not a `cargo test` invocation"))?;
    let arguments: Vec<String> = rest.split_whitespace().map(str::to_owned).collect();
    if arguments.iter().any(|argument| argument == "--") {
        return Err(format!("`{line}` passes harness arguments of its own"));
    }
    Ok(arguments)
}

#[cfg(test)]
mod tests {
    use super::{compare, crate_of, parse, recipe_arguments, REFERENCE};

    const WORKSPACE: &str = "\
     Running unittests src/lib.rs (target/debug/deps/lexlean-5b460581093087bc)
a::one: test
a::two: test

2 tests, 0 benchmarks
     Running unittests src/lib.rs (target/debug/deps/repo_conformance-10b139a5c565674f)
c::lib: test

1 test, 0 benchmarks
     Running tests/bdd.rs (target/debug/deps/bdd-2770020b8191c4a2)
bdd_one: test

1 test, 0 benchmarks
     Running tests/conformance.rs (target/debug/deps/conformance-aec8e4616fa41234)
conformance_rp_01: test
conformance_rp_02: test

2 tests, 0 benchmarks
   Doc-tests lexlean
0 tests, 0 benchmarks
";

    fn ordinary() -> &'static str {
        "\
     Running unittests src/lib.rs (target/debug/deps/lexlean-5b460581093087bc)
a::one: test
a::two: test

2 tests, 0 benchmarks
   Doc-tests lexlean
0 tests, 0 benchmarks
"
    }

    fn owned() -> &'static str {
        "\
     Running unittests src/lib.rs (target/debug/deps/repo_conformance-10b139a5c565674f)
c::lib: test

1 test, 0 benchmarks
     Running tests/bdd.rs (target/debug/deps/bdd-2770020b8191c4a2)
bdd_one: test

1 test, 0 benchmarks
     Running tests/conformance.rs (target/debug/deps/conformance-aec8e4616fa41234)
conformance_rp_01: test
conformance_rp_02: test

2 tests, 0 benchmarks
"
    }

    fn crates() -> std::collections::BTreeSet<String> {
        ["repo_conformance", "bdd", "conformance"]
            .into_iter()
            .map(str::to_owned)
            .collect()
    }

    #[test]
    fn the_crate_of_a_binary_loses_only_its_hash() {
        assert_eq!(
            crate_of("target/debug/deps/repo_conformance-10b139a5c565674f"),
            "repo_conformance"
        );
        assert_eq!(crate_of("C:\\t\\deps\\xtask-0d71e9dc1bf12005.exe"), "xtask");
        assert_eq!(crate_of("deps/no-hash-here"), "no-hash-here");
    }

    #[test]
    fn a_partition_of_the_workspace_is_accepted() {
        let summary = compare(
            &parse(WORKSPACE).expect("parses"),
            &parse(ordinary()).expect("parses"),
            &parse(owned()).expect("parses"),
            &crates(),
        )
        .expect("the phases partition the workspace");
        assert_eq!(
            (summary.workspace, summary.ordinary, summary.owned),
            (6, 2, 4)
        );
    }

    #[test]
    fn a_test_run_twice_a_test_lost_and_a_target_misplaced_are_each_refused() {
        let workspace = parse(WORKSPACE).expect("parses");
        let ordinary_ok = parse(ordinary()).expect("parses");
        let owned_ok = parse(owned()).expect("parses");

        // Twice: the ordinary phase also runs the owned package (what the
        // single `cargo test --workspace` plus `bdd` did before the split).
        let both = parse(&format!("{}{}", ordinary(), owned())).expect("parses");
        let error = compare(&workspace, &both, &owned_ok, &crates()).expect_err("run twice");
        assert!(error.contains("both phases"), "{error}");

        // Misplaced: the ordinary phase runs an owned target that `bdd`
        // does not (the targets differ, so no single test is run twice).
        let misplaced = parse(&format!(
            "{}     Running tests/conformance.rs (target/debug/deps/conformance-aec8e4616fa41234)\nonly_here: test\n",
            ordinary()
        ))
        .expect("parses");
        let error = compare(&workspace, &misplaced, &owned_ok, &crates()).expect_err("misplaced");
        assert!(
            error.contains("sole owner") || error.contains("does not list"),
            "{error}"
        );

        // Lost: `bdd` runs the meta-gate target alone (what it used to be).
        let lost = parse(
            "     Running tests/bdd.rs (target/debug/deps/bdd-2770020b8191c4a2)\nbdd_one: test\n",
        )
        .expect("parses");
        let error = compare(&workspace, &ordinary_ok, &lost, &crates()).expect_err("tests lost");
        assert!(error.contains("neither phase"), "{error}");

        // Lost: one test of the ordinary phase disappears.
        let fewer = parse(&ordinary().replace("a::two: test\n", "")).expect("parses");
        let error = compare(&workspace, &fewer, &owned_ok, &crates()).expect_err("a test lost");
        assert!(error.contains("a::two"), "{error}");

        // Not in the workspace: a phase invents a test.
        let more = parse(&format!("{}extra: test\n", ordinary())).expect("parses");
        let error = compare(&workspace, &more, &owned_ok, &crates()).expect_err("an invented test");
        assert!(error.contains("does not list"), "{error}");

        // Nothing at all is not a proof.
        let empty = parse("").expect("parses");
        assert!(compare(&empty, &empty, &empty, &crates()).is_err());
    }

    #[test]
    fn the_parts_of_bdd_partition_it_and_every_defect_is_named() {
        use super::compare_parts;
        let plan = crate::bdd_parts::Plan::parse(
            r#"{ "remainder": "rest", "parts": { "first": ["RP-01"] } }"#,
        )
        .expect("a plan");
        let suite = parse(owned()).expect("parses");
        let first = parse(
            "     Running tests/conformance.rs (target/debug/deps/conformance-aec8e4616fa41234)\nconformance_rp_01: test\n",
        )
        .expect("parses");
        let rest = parse(&owned().replace("conformance_rp_01: test\n", "")).expect("parses");
        let parts = |first: &super::Parsed, rest: &super::Parsed| {
            vec![
                ("first".to_owned(), first.clone()),
                ("rest".to_owned(), rest.clone()),
            ]
        };
        assert_eq!(
            compare_parts(&suite, &parts(&first, &rest), &plan).expect("a partition"),
            2
        );
        // Twice: the remainder also runs the named test.
        let error = compare_parts(&suite, &parts(&first, &suite), &plan).expect_err("twice");
        assert!(
            error.contains("`conformance_rp_01`") && error.contains("parts `first` and `rest`"),
            "{error}"
        );
        // Lost: the remainder drops the meta-gate target's test.
        let fewer = parse(
            &owned()
                .replace("bdd_one: test\n", "")
                .replace("conformance_rp_01: test\n", ""),
        )
        .expect("parses");
        let error = compare_parts(&suite, &parts(&first, &fewer), &plan).expect_err("lost");
        assert!(
            error.contains("run in no part") && error.contains("bdd_one"),
            "{error}"
        );
        // Invented: a part runs a test `bdd` does not list.
        let invented = parse(&format!(
            "{}conformance_rp_99: test\n",
            owned().replace("conformance_rp_01: test\n", "")
        ))
        .expect("parses");
        let error = compare_parts(&suite, &parts(&first, &invented), &plan).expect_err("invented");
        assert!(error.contains("which `bdd` does not list"), "{error}");
        // A named ID whose test the part does not run.
        let missing = crate::bdd_parts::Plan::parse(
            r#"{ "remainder": "rest", "parts": { "first": ["RP-01", "RP-77"] } }"#,
        )
        .expect("a plan");
        let error = compare_parts(&suite, &parts(&first, &rest), &missing).expect_err("missing");
        assert!(error.contains("names `RP-77`"), "{error}");
    }

    #[test]
    fn a_test_before_any_target_is_not_understood() {
        assert!(parse("orphan: test\n").is_err());
    }

    #[test]
    fn recipe_arguments_are_read_from_the_cargo_line() {
        assert_eq!(
            recipe_arguments("cargo test --workspace --exclude repo-conformance --all-features")
                .expect("parses"),
            [
                "--workspace",
                "--exclude",
                "repo-conformance",
                "--all-features"
            ]
        );
        assert!(recipe_arguments("cargo build").is_err());
        assert!(recipe_arguments("cargo test -- --nocapture").is_err());
        assert_eq!(REFERENCE.len(), 2);
    }
}
