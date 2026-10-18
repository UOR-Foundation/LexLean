//! The `repository` suite: RP-01..RP-16.

use std::collections::BTreeSet;

use crate::support;

fn root_file(relative: &str) -> String {
    std::fs::read_to_string(support::repo_root().join(relative).as_std_path())
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
}

/// The Justfile recipes as `name -> (dependency line, command lines)`.
fn just_recipes(justfile: &str) -> std::collections::BTreeMap<String, (String, Vec<String>)> {
    let mut recipes = std::collections::BTreeMap::new();
    let mut current: Option<String> = None;
    for line in justfile.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        if line.starts_with(' ') || line.starts_with('\t') {
            if let Some(name) = &current {
                if let Some((_, lines)) = recipes.get_mut(name) {
                    let lines: &mut Vec<String> = lines;
                    lines.push(line.trim().trim_start_matches('@').to_owned());
                }
            }
            continue;
        }
        if let Some((name, dependencies)) = line.split_once(':') {
            let name = name.trim().to_owned();
            recipes.insert(name.clone(), (dependencies.trim().to_owned(), Vec::new()));
            current = Some(name);
        }
    }
    recipes
}

/// The deferral markers §27.10 forbids in maintained source. Every
/// spelling is assembled from halves so this file passes its own scan.
fn deferral_markers() -> Vec<String> {
    vec![
        format!("TO{}", "DO"),
        format!("FIX{}", "ME"),
        format!("XX{}", "X"),
        format!("HA{}CK", ""),
        format!("unimpl{}!", "emented"),
        format!("to{}!(", "do"),
    ]
}

/// The disabled-test attributes one file carries: the ignore attribute in
/// every form (bare, or with a reason string) and a `cfg_attr` whose
/// applied attribute is ignore, across lines. An attribute stands at the
/// start of a line, after whitespace, or after a preceding attribute; an
/// occurrence inside a string literal or backticked prose is a scanner's
/// or a document's own mention, not an attribute. The spellings here are
/// assembled from halves for the same reason.
fn ignore_attributes(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    let is_attribute_position = |index: usize| {
        text[..index]
            .bytes()
            .next_back()
            .is_none_or(|b| b.is_ascii_whitespace() || b == b']')
    };
    let ignore_open = format!("#[ig{}", "nore");
    for (index, _) in text.match_indices(&ignore_open) {
        if !is_attribute_position(index) {
            continue;
        }
        let end = text[index..]
            .find(']')
            .map_or(text.len(), |offset| index + offset + 1);
        found.push(text[index..end].to_owned());
    }
    let ignore_word = format!("ig{}", "nore");
    let is_token_boundary = |byte: Option<&u8>| {
        byte.is_none_or(|b| !(b.is_ascii_alphanumeric() || *b == b'_' || *b == b'"'))
    };
    for (index, _) in text.match_indices("#[cfg_attr(") {
        if !is_attribute_position(index) {
            continue;
        }
        let tail = &text[index..];
        let end = tail.find(")]").map_or(tail.len(), |offset| offset + 2);
        let attribute = &tail[..end];
        let bytes = attribute.as_bytes();
        let names_ignore = attribute.match_indices(&ignore_word).any(|(at, _)| {
            at > 0
                && is_token_boundary(bytes.get(at - 1))
                && is_token_boundary(bytes.get(at + ignore_word.len()))
        });
        if names_ignore {
            found.push(attribute.split_whitespace().collect::<Vec<_>>().join(" "));
        }
    }
    found
}

/// Every §28.2 fixture directory of the repository.
fn repo_conformance_fixtures(root: &camino::Utf8Path) -> Vec<camino::Utf8PathBuf> {
    let fixtures = crate::fixtures::discover(root);
    assert!(
        fixtures.len() > 20,
        "the fixture suite is discovered: {fixtures:?}"
    );
    fixtures
}

/// The §31 table parsed as `(id, suite, statement)` rows.
fn spec_table() -> Vec<(String, String, String)> {
    let text = support::spec_text();
    let start = text
        .find("## 31. Complete conformance-ID registry")
        .expect("SPEC.md has §31");
    let mut rows = Vec::new();
    for line in text[start..].lines() {
        let Some(stripped) = line.strip_prefix("| `") else {
            continue;
        };
        let cells: Vec<&str> = stripped.split(" | ").collect();
        if cells.len() != 4 {
            continue;
        }
        let id = cells[0].trim_end_matches('`').to_owned();
        let suite = cells[1].trim_matches('`').to_owned();
        rows.push((id, suite, cells[2].to_owned()));
    }
    rows
}

/// What `.github/ci-plan.sh` decides for planted events (§9.4, RP-14): the
/// script runs in a scratch repository with a stand-in for the `gh` command,
/// so that each decision is shown, and each way of being wrong is refused.
fn plan_decisions() {
    use std::process::Command;
    // The script is bash with `git`, `jq`, and a stand-in for `gh` on `PATH`,
    // run where the gate runs (§8.3, §9.4: the normative host is Linux). The
    // other hosts of the matrix run the case's reading of the files and skip
    // this part: a Windows `PATH` is not a `:`-separated list, and no
    // decision of the plan depends on the host.
    if !cfg!(target_os = "linux") {
        return;
    }
    let root = support::repo_root();
    let script = root.join(".github/ci-plan.sh");
    let rows_file = root.join(".github/vv-rows.json");
    let workflow = root.join(".github/workflows/ci.yml");
    let rows: Vec<serde_json::Value> =
        serde_json::from_str(&root_file(".github/vv-rows.json")).expect("the legs");
    let parts: Vec<String> = rows
        .iter()
        .map(|row| row["part"].as_str().expect("a part").to_owned())
        .collect();
    let ci = root_file(".github/workflows/ci.yml");
    let hosts: Vec<String> = ci
        .lines()
        .filter(|line| line.starts_with("          - { os: "))
        .map(|line| {
            let target = line.split("target: ").nth(1).expect("a target");
            format!(
                "build and non-Lean tests ({})",
                target.trim_end_matches(" }")
            )
        })
        .collect();
    assert_eq!(hosts.len(), 4, "§8.3: the four hosts of the matrix");
    // The jobs of a run of the whole gate that passed: the plan, every leg,
    // the Atlas join, and every host. The shards do not matter to the plan:
    // the join needs them all.
    let mut green: Vec<String> = vec!["plan\tsuccess".to_owned()];
    green.extend(
        parts
            .iter()
            .map(|part| format!("just vv ({part})\tsuccess")),
    );
    green.push("atlas (prefix)\tsuccess".to_owned());
    green.push("atlas (join)\tsuccess".to_owned());
    green.extend(hosts.iter().map(|host| format!("{host}\tsuccess")));

    let scratch = tempfile::tempdir().expect("tempdir");
    let dir = scratch.path();
    let git = |args: &[&str]| -> String {
        let output = Command::new("git")
            .current_dir(dir)
            .args([
                "-c",
                "user.name=t",
                "-c",
                "user.email=t@t",
                "-c",
                "commit.gpgsign=false",
            ])
            .args(args)
            .output()
            .expect("git runs");
        assert!(
            output.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout)
            .expect("utf8")
            .trim()
            .to_owned()
    };
    let write = |name: &str, text: &str| std::fs::write(dir.join(name), text).expect("write");
    git(&["init", "-q", "-b", "main"]);
    write("a", "a\n");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "base"]);
    let base = git(&["rev-parse", "HEAD"]);
    // A pull request head that contains main's tip, merged: the merge has the
    // tree of the head.
    git(&["checkout", "-q", "-b", "head-contains-main"]);
    write("b", "b\n");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "change"]);
    git(&["checkout", "-q", "main"]);
    git(&[
        "merge",
        "-q",
        "--no-ff",
        "-m",
        "merge",
        "head-contains-main",
    ]);
    let merge_equal = git(&["rev-parse", "HEAD"]);
    assert_eq!(
        git(&["rev-parse", "HEAD^{tree}"]),
        git(&["rev-parse", "HEAD^2^{tree}"]),
        "the planted merge lands its head's tree"
    );
    // A pull request head that does not contain main's tip: main moved, so
    // the merge has a tree no pull request run verified.
    git(&["checkout", "-q", "-b", "head-behind-main", &base]);
    write("c", "c\n");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "change"]);
    git(&["checkout", "-q", "main"]);
    write("d", "d\n");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "main moves"]);
    git(&["merge", "-q", "--no-ff", "-m", "merge", "head-behind-main"]);
    let merge_different = git(&["rev-parse", "HEAD"]);
    assert_ne!(
        git(&["rev-parse", "HEAD^{tree}"]),
        git(&["rev-parse", "HEAD^2^{tree}"]),
        "the planted merge changes the tree of its head"
    );
    // A push that is no merge.
    write("e", "e\n");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "direct"]);
    let direct = git(&["rev-parse", "HEAD"]);

    // `gh`: the runs of the second parent, and the jobs of a run, as the
    // environment names them; `STUB_FAIL` makes the call fail.
    let bin = dir.join("bin");
    std::fs::create_dir(&bin).expect("bin");
    let stub = bin.join("gh");
    std::fs::write(
        &stub,
        "#!/usr/bin/env bash\n\
         [ -z \"${STUB_FAIL:-}\" ] || exit 1\n\
         case \"$2\" in\n\
           */workflows/ci.yml/runs\\?*) printf '%s\\n' \"${STUB_RUNS:-}\" ;;\n\
           */runs/*/jobs\\?*) printf '%s\\n' \"${STUB_JOBS:-}\" ;;\n\
           *) exit 2 ;;\n\
         esac\n",
    )
    .expect("stub");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&stub, std::fs::Permissions::from_mode(0o755)).expect("chmod");
    }
    let path = format!("{}:{}", bin.display(), std::env::var("PATH").expect("PATH"));
    // (full, rows' parts) for one planted event.
    let decide =
        |event: &str, draft: &str, commit: &str, runs: &str, jobs: &[String], fail: bool| {
            git(&["checkout", "-q", "--detach", commit]);
            let out = dir.join("out");
            let _ = std::fs::remove_file(&out);
            let mut command = Command::new("bash");
            command
                .current_dir(dir)
                .arg(script.as_std_path())
                .env("PATH", &path)
                .env("EVENT", event)
                .env("DRAFT", draft)
                .env("REPO", "owner/repo")
                .env("LEXLEAN_CI_ROWS", rows_file.as_std_path())
                .env("LEXLEAN_CI_WORKFLOW", workflow.as_std_path())
                .env("GITHUB_OUTPUT", &out)
                .env("GITHUB_STEP_SUMMARY", dir.join("summary"))
                .env("STUB_RUNS", runs)
                .env("STUB_JOBS", jobs.join("\n"));
            if fail {
                command.env("STUB_FAIL", "1");
            }
            let output = command.output().expect("the plan script runs");
            assert!(
                output.status.success(),
                "the plan script fails closed to the whole gate and never itself: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            let text = std::fs::read_to_string(&out).expect("outputs");
            let value = |name: &str| -> String {
                text.lines()
                    .find_map(|line| line.strip_prefix(&format!("{name}=")))
                    .unwrap_or_else(|| panic!("the plan writes `{name}`: {text}"))
                    .to_owned()
            };
            let planned: Vec<serde_json::Value> =
                serde_json::from_str(&value("rows")).expect("rows are JSON");
            (
                value("full"),
                planned
                    .iter()
                    .map(|row| row["part"].as_str().expect("a part").to_owned())
                    .collect::<Vec<_>>(),
            )
        };
    let whole = ("true".to_owned(), parts.clone());
    let draft = ("false".to_owned(), vec!["rest".to_owned()]);
    let nothing = ("false".to_owned(), Vec::<String>::new());
    // A draft pull request is the `rest` leg; a ready one, a request, and a
    // push are the whole gate when no run is known.
    assert_eq!(
        decide("pull_request", "true", &direct, "", &[], false),
        draft
    );
    assert_eq!(
        decide("pull_request", "false", &direct, "", &[], false),
        whole
    );
    assert_eq!(
        decide("workflow_dispatch", "", &direct, "", &[], false),
        whole
    );
    assert_eq!(decide("push", "", &direct, "", &[], false), whole);
    // A merge of its head's tree whose whole gate passed runs nothing.
    assert_eq!(
        decide("push", "", &merge_equal, "4242", &green, false),
        nothing
    );
    // Every way the evidence can fall short runs the whole gate: no run, a
    // run that skipped or failed a job, a run without the Atlas join, one
    // host or one leg missing, the gate unreadable, and a merge that changed
    // the tree, however good the run of its head.
    assert_eq!(decide("push", "", &merge_equal, "", &green, false), whole);
    let without = |name: &str| -> Vec<String> {
        green
            .iter()
            .filter(|job| !job.starts_with(name))
            .cloned()
            .collect()
    };
    let with = |job: &str| -> Vec<String> {
        let mut jobs = green.clone();
        jobs.push(job.to_owned());
        jobs
    };
    for (what, jobs) in [
        ("a skipped job", with("atlas (closure)\tskipped")),
        ("a failed job", with("atlas (closure)\tfailure")),
        ("no Atlas join", without("atlas (join)")),
        ("no host", without(&hosts[0])),
        ("no leg", without("just vv (rest)")),
        (
            "a draft run",
            vec![
                "plan\tsuccess".to_owned(),
                "just vv (rest)\tsuccess".to_owned(),
            ],
        ),
    ] {
        assert_eq!(
            decide("push", "", &merge_equal, "4242", &jobs, false),
            whole,
            "§9.4: {what} is not the whole gate passing"
        );
    }
    assert_eq!(
        decide("push", "", &merge_equal, "4242", &green, true),
        whole
    );
    assert_eq!(
        decide("push", "", &merge_different, "4242", &green, false),
        whole
    );
    assert_eq!(decide("push", "", &direct, "4242", &green, false), whole);
    // A run of another repository's pull request is not one of this one's.
    let script_text = root_file(".github/ci-plan.sh");
    assert!(
        script_text.contains(".head_repository.full_name == \\\"$repo\\\"")
            && script_text.contains("event=pull_request&status=success"),
        "§9.4: only a passed pull request run of this repository counts"
    );
}

pub(crate) fn run(id: &str) {
    let root = support::repo_root();
    match id {
        // §2: exact identity — name, description, metadata, licenses.
        "RP-01" => {
            let workspace = root_file("Cargo.toml");
            for required in [
                "version = \"0.3.0\"",
                "edition = \"2021\"",
                "rust-version = \"1.97\"",
                "license = \"MIT OR Apache-2.0\"",
                "repository = \"https://github.com/afflom/lexlean\"",
                "homepage = \"https://github.com/afflom/lexlean\"",
                "authors = [\"Alex Flom\"]",
                "keywords = [\"lean\", \"latex\", \"formal-methods\", \"compiler\", \"proof\"]",
                "categories = [\"compilers\", \"development-tools\"]",
            ] {
                assert!(
                    workspace.contains(required),
                    "Cargo.toml lacks §2.3 metadata row {required}"
                );
            }
            let crate_toml = root_file("crates/lexlean/Cargo.toml");
            assert!(
                crate_toml.contains("name = \"lexlean\""),
                "§2.1: the crate is lexlean"
            );
            let description = "A closed-lexicon LaTeX-to-Lean 4 compiler whose canonical document and prose-free Lean program are generated from one semantic representation.";
            assert!(
                crate_toml.contains(description),
                "§2.1: the crate carries the exact one-line description"
            );
            assert!(
                root.join("LICENSE-APACHE").is_file(),
                "§2.4: LICENSE-APACHE"
            );
            assert!(root.join("LICENSE-MIT").is_file(), "§2.4: LICENSE-MIT");
            let (exit, stdout, _) = support::cli_in(&root, &["--version"]);
            assert_eq!(exit, 0);
            assert!(
                stdout.starts_with("lexlean 0.3.0\n"),
                "§30.3: the binary reports the lexlean identity, got {stdout:?}"
            );
        }
        // §2.2: pinned template commit recorded; inherited domain logic gone.
        "RP-02" => {
            let commit = "0a1c799338d7db829aa23365e1acf4f9d01ff8b5";
            let agents = root_file("AGENTS.md");
            assert!(
                agents.contains(commit),
                "§2.2: AGENTS.md records the pinned template commit"
            );
            assert!(
                support::spec_text().contains(commit),
                "§2.2: SPEC.md pins the template commit"
            );
            // The inherited `audit-limits` allow-list must not survive.
            for source in [
                "xtask/src/audit.rs",
                "xtask/src/main.rs",
                "AGENTS.md",
                "Justfile",
            ] {
                let text = root_file(source);
                assert!(
                    !text.contains("audit-limits"),
                    "§2.2: {source} still carries the template's audit-limits remnant"
                );
            }
        }
        // §7: the committed layout tree exists on disk.
        "RP-03" => {
            let text = support::spec_text();
            let start = text.find("## 7. Repository layout").expect("§7");
            let end = text[start..].find("\n## 8.").expect("§8 follows") + start;
            let section = &text[start..end];
            let tree = section
                .split("```text\n")
                .nth(1)
                .and_then(|rest| rest.split("\n```").next())
                .expect("§7 has a layout tree");
            let mut stack: Vec<String> = Vec::new();
            let mut checked = 0usize;
            for line in tree.lines() {
                if line.trim() == "." {
                    continue;
                }
                let Some(marker) = line.find("── ") else {
                    continue;
                };
                // The prefix is chars, not bytes: each level is one 4-char
                // group ("│   " or "    ") before the "├──"/"└──" marker.
                let depth = (line[..marker].chars().count() - 1) / 4;
                let name = line[marker + "── ".len()..]
                    .split_whitespace()
                    .next()
                    .expect("a tree row names a path");
                stack.truncate(depth);
                let is_dir = name.ends_with('/');
                let clean = name.trim_end_matches('/');
                let relative = if stack.is_empty() {
                    clean.to_owned()
                } else {
                    format!("{}/{clean}", stack.join("/"))
                };
                let path = root.join(&relative);
                if is_dir {
                    assert!(path.is_dir(), "§7: {relative}/ is a required directory");
                    stack.push(clean.to_owned());
                } else {
                    assert!(path.is_file(), "§7: {relative} is a required file");
                }
                checked += 1;
            }
            assert!(checked > 80, "the §7 tree parse found only {checked} rows");
            // The committed tree holds inputs and oracles only: every
            // example and fixture project runs from a temporary copy, the
            // live build roots are ignored, and no fixture project carries
            // one (the runner refuses it; `expected/` is the oracle).
            let gitignore = root_file(".gitignore");
            for rule in ["examples/**/.lexlean/", "tests/**/.lexlean/"] {
                assert!(
                    gitignore.lines().any(|line| line.trim() == rule),
                    ".gitignore lacks the build-root rule `{rule}`"
                );
            }
            for fixture in repo_conformance_fixtures(&root) {
                assert!(
                    !fixture.join("project/.lexlean").as_std_path().exists(),
                    "{fixture}: the committed fixture project carries a build root; fixtures run from a temporary copy — remove it"
                );
            }
            // No tracked file is named `CACHEDIR.TAG` or sits under a path
            // component the repository walkers skip (`target`, `.git`,
            // `.lexlean`, `.lake`, `node_modules`, or `expected` outside the
            // sanctioned `expected/` oracle directories), so no tracked subtree
            // can hide from the walkers.
            let is_sanctioned_expected = |parts: &[&str], index: usize| -> bool {
                (index == 1 && matches!(parts[0], "bootstrap" | "compiler" | "stdlib"))
                    || (index == 2 && parts[0] == "examples")
                    || (index == 3
                        && parts[0] == "tests"
                        && matches!(parts[1], "negative" | "tooling"))
                    || (index == 4 && parts[0] == "tests" && parts[1] == "fixtures")
            };
            let walker_skip_defect = |tracked: &str| -> Option<String> {
                let parts: Vec<&str> = tracked.split('/').collect();
                for (index, component) in parts.iter().copied().enumerate() {
                    if component == "CACHEDIR.TAG" {
                        return Some(format!(
                            "{tracked}: tracked `CACHEDIR.TAG` would cause repository walkers to skip `{}`",
                            parts[..index].join("/")
                        ));
                    }
                    if matches!(
                        component,
                        "target" | ".git" | ".lexlean" | ".lake" | "node_modules"
                    ) {
                        return Some(format!(
                            "{tracked}: tracked path has walker-skipped component `{component}`"
                        ));
                    }
                    if component == "expected" && !is_sanctioned_expected(&parts, index) {
                        return Some(format!(
                            "{tracked}: tracked path has `expected` component outside sanctioned oracle directories"
                        ));
                    }
                }
                None
            };
            let ls_files = std::process::Command::new("git")
                .args(["ls-files", "-z"])
                .current_dir(root.as_std_path())
                .output()
                .expect("git ls-files runs in the repository");
            assert!(ls_files.status.success(), "git ls-files succeeded");
            let tracked: Vec<String> = ls_files
                .stdout
                .split(|byte| *byte == 0)
                .filter(|entry| !entry.is_empty())
                .map(|entry| String::from_utf8_lossy(entry).replace('\\', "/"))
                .collect();
            assert!(
                tracked.len() > 1000,
                "git ls-files returned {} tracked files",
                tracked.len()
            );
            let defects: Vec<String> = tracked
                .iter()
                .filter_map(|path| walker_skip_defect(path))
                .collect();
            assert_eq!(
                defects,
                Vec::<String>::new(),
                "§7 (RP-03): the tracked tree holds nothing the repository walkers skip"
            );
            for planted in [
                "examples/collections/CACHEDIR.TAG",
                "crates/lexlean/.lexlean/planted.lean",
                "crates/lexlean/src/expected/planted.rs",
            ] {
                assert!(
                    walker_skip_defect(planted).is_some(),
                    "planted tracked path `{planted}` is refused by RP-03"
                );
            }
        }
        // §7, §8.4: shipped crates derive from `publish = false`, and the
        // shipped crate depends on no repository-only tooling.
        "RP-04" => {
            for tooling in ["crates/model", "crates/conformance", "xtask"] {
                let text = root_file(&format!("{tooling}/Cargo.toml"));
                assert!(
                    text.contains("publish = false"),
                    "{tooling} must be repository-only (publish = false)"
                );
            }
            let shipped = root_file("crates/lexlean/Cargo.toml");
            assert!(
                !shipped.contains("publish = false"),
                "the lexlean crate is the shipped crate"
            );
            for forbidden in ["repo-model", "repo-conformance", "xtask"] {
                assert!(
                    !shipped.contains(forbidden),
                    "the shipped crate must not depend on {forbidden}"
                );
            }
        }
        // §9.2: the vv recipe runs every gate in the fixed order, and every
        // recipe is exactly the specified command line.
        "RP-05" => {
            let justfile = root_file("Justfile");
            let recipes = just_recipes(&justfile);
            let vv = recipes.get("vv").expect("§9.2: the Justfile defines vv");
            assert_eq!(
                vv.0,
                "fmt-check model spec-links lint test features bdd inventory examples golden repro deny",
                "§9.2: `just vv` runs the twelve gates in the normative order"
            );
            let specified: [(&str, &str); 12] = [
                ("fmt-check", "cargo fmt --all -- --check"),
                ("model", "cargo xtask validate-model"),
                ("spec-links", "cargo xtask validate-spec-links"),
                (
                    "lint",
                    "cargo clippy --workspace --all-targets --all-features -- -D warnings",
                ),
                (
                    "test",
                    "cargo test --workspace --exclude repo-conformance --all-features",
                ),
                (
                    "features",
                    "cargo check --workspace --all-features --all-targets",
                ),
                ("bdd", "cargo xtask bdd"),
                ("inventory", "cargo xtask check-test-inventory"),
                ("examples", "cargo xtask verify-examples"),
                ("golden", "cargo xtask check-golden"),
                ("repro", "cargo xtask check-reproducibility"),
                ("deny", "cargo deny --all-features check"),
            ];
            for (name, command) in specified {
                let (dependencies, lines) = recipes
                    .get(name)
                    .unwrap_or_else(|| panic!("§9.2: the Justfile defines `{name}`"));
                assert!(
                    dependencies.is_empty(),
                    "§9.2: `{name}` is a leaf recipe, found dependencies `{dependencies}`"
                );
                assert_eq!(
                    lines.as_slice(),
                    [command.to_owned()],
                    "§9.2: `{name}` runs exactly the specified command"
                );
            }
            assert_eq!(
                recipes
                    .get("model-write")
                    .map(|(_, lines)| lines.as_slice()),
                Some(&["cargo xtask validate-model --write".to_owned()][..]),
                "§9.2: model-write regenerates the generated documents"
            );
            assert_eq!(
                recipes
                    .get("golden-write")
                    .map(|(_, lines)| lines.as_slice()),
                Some(&["cargo xtask check-golden --write".to_owned()][..]),
                "§9.2: golden-write is the only golden rewrite path"
            );
            for dependency in vv.0.split_whitespace() {
                let (_, lines) = recipes
                    .get(dependency)
                    .unwrap_or_else(|| panic!("vv depends on undefined recipe {dependency}"));
                for line in lines {
                    assert!(
                        !line.contains("--write"),
                        "§9.2: no acceptance recipe rewrites source or expected output: {line}"
                    );
                }
            }
            for rewriting in ["model-write", "golden-write", "fixtures-write"] {
                assert!(
                    !vv.0.split_whitespace().any(|dep| dep == rewriting),
                    "§9.2: `{rewriting}` is never part of vv"
                );
            }
            // The gate runner's manifest names every task it dispatches, so
            // the crate description is an honest index of the gates.
            let xtask_main = root_file("xtask/src/main.rs");
            let dispatched: Vec<&str> = xtask_main
                .lines()
                .filter_map(|line| {
                    let trimmed = line.trim();
                    let name = trimmed.strip_prefix('"')?;
                    let (name, rest) = name.split_once('"')?;
                    rest.trim_start().starts_with("=>").then_some(name)
                })
                .collect();
            assert!(
                dispatched.len() >= 8,
                "xtask dispatches its documented tasks: {dispatched:?}"
            );
            let xtask_toml = root_file("xtask/Cargo.toml");
            let description = xtask_toml
                .lines()
                .find_map(|line| line.strip_prefix("description = "))
                .expect("xtask/Cargo.toml carries a description");
            for task in dispatched {
                assert!(
                    description.contains(task),
                    "xtask/Cargo.toml description omits the dispatched task `{task}`: {description}"
                );
            }
        }
        // §27.5: the committed documents equal regeneration from the model.
        "RP-06" => {
            let model = repo_model::Model::load(&root.join("model").into_std_path_buf())
                .expect("the model loads");
            assert_eq!(
                root_file(repo_model::codegen::CONFORMANCE_PATH),
                repo_model::codegen::render_conformance(&model),
                "CONFORMANCE.md is stale; run `just model-write`"
            );
            assert_eq!(
                root_file(repo_model::codegen::ERRORS_PATH),
                repo_model::codegen::render_errors(&model),
                "ERRORS.md is stale; run `just model-write`"
            );
        }
        // §27.6: the §31 table and the register are bijective and equal.
        "RP-07" => {
            let model = repo_model::Model::load(&root.join("model").into_std_path_buf())
                .expect("the model loads");
            let table = spec_table();
            assert_eq!(table.len(), 416, "§31 has 416 rows");
            assert_eq!(model.ids.id.len(), table.len(), "register row count");
            for ((spec_id, spec_suite, spec_statement), row) in
                table.iter().zip(model.ids.id.iter())
            {
                assert_eq!(&row.id, spec_id, "register order matches §31");
                assert_eq!(&row.suite, spec_suite, "{spec_id}: suite matches §31");
                assert_eq!(
                    &row.statement, spec_statement,
                    "{spec_id}: statement is byte-equal with §31"
                );
            }
        }
        // §27.10: no deferral markers anywhere in maintained source. The
        // marker spellings are assembled from halves so this file passes its
        // own scan.
        "RP-08" => {
            let markers = deferral_markers();
            // Anti-vacuity: the attribute scanner fires on every planted
            // form before it vouches for the tree.
            let ignore_word = format!("ig{}", "nore");
            let planted = format!(
                "#[test]\n#[{ignore_word} = \"later\"]\nfn a() {{}}\n#[cfg_attr(\n    windows,\n    {ignore_word}\n)]\nfn b() {{}}\n#[{ignore_word}]\nfn c() {{}}\n"
            );
            assert_eq!(
                ignore_attributes(&planted).len(),
                3,
                "the ignore scanner sees the `= reason`, cfg_attr, and bare forms"
            );
            assert!(
                ignore_attributes(&format!(
                    "starts_with(\"#[{ignore_word}\") #[cfg_attr(windows, {ignore_word}d_never)] #[cfg_attr(unix, allow(un{ignore_word}d))]"
                ))
                .is_empty(),
                "string mentions and other identifiers are not attributes"
            );
            let mut scanned = 0usize;
            for entry in walkdir::WalkDir::new(root.as_std_path())
                .into_iter()
                .filter_entry(|e| {
                    let name = e.file_name().to_string_lossy();
                    name != "target"
                        && name != ".git"
                        && name != ".lexlean"
                        && name != "expected"
                        && !support::holds_cache_tag(e)
                })
                .flatten()
            {
                if !entry.file_type().is_file() {
                    continue;
                }
                let path = entry.path();
                let Some(extension) = path.extension().and_then(|e| e.to_str()) else {
                    continue;
                };
                if !matches!(
                    extension,
                    "rs" | "toml" | "json" | "feature" | "tex" | "lean"
                ) {
                    continue;
                }
                let Ok(text) = std::fs::read_to_string(path) else {
                    continue;
                };
                scanned += 1;
                for marker in &markers {
                    assert!(
                        !text.contains(marker.as_str()),
                        "R4: {} contains the deferral marker `{marker}`",
                        path.display()
                    );
                }
                let ignored = ignore_attributes(&text);
                assert!(
                    ignored.is_empty(),
                    "R4: {} disables a test with {ignored:?}",
                    path.display()
                );
            }
            assert!(
                scanned > 100,
                "the deferral scan covered only {scanned} files"
            );
            // §27.8: no conformance test is ignored or hidden behind a cfg,
            // wherever the attribute sits in the block.
            let (_names, flagged) = crate::workspace_test_names_with_flags(root.as_std_path());
            let hidden: Vec<&String> = flagged
                .iter()
                .filter(|name| name.starts_with("conformance_"))
                .collect();
            assert!(
                hidden.is_empty(),
                "R4: hidden conformance tests: {hidden:?}"
            );
        }
        // §8.1, §27.10: the shipped crate forbids unsafe Rust, actively.
        "RP-09" => {
            let lib = root_file("crates/lexlean/src/lib.rs");
            let attribute = format!("#![forbid({}safe_code)]", "un");
            assert!(
                lib.contains(&attribute),
                "lib.rs carries the forbid attribute"
            );
            let token = format!("{}safe ", "un");
            for entry in walkdir::WalkDir::new(root.join("crates/lexlean/src").as_std_path())
                .into_iter()
                .flatten()
            {
                if entry.file_type().is_file() {
                    let text = std::fs::read_to_string(entry.path()).expect("source reads");
                    assert!(
                        !text.contains(&token),
                        "{}: an unsafe token in the shipped crate",
                        entry.path().display()
                    );
                }
            }
        }
        // §21.2: the embedded semantics ID equals a clean disk recomputation.
        "RP-10" => {
            let files = support::normative_rows();
            let borrowed: Vec<(&str, &[u8])> = files
                .iter()
                .map(|(path, bytes)| (path.as_str(), bytes.as_slice()))
                .collect();
            let recomputed = support::identity_tree_digest(&borrowed);
            assert_eq!(
                recomputed,
                lexlean::compiler_semantics_id_for(lexlean::LATEST_LANGUAGE_VERSION),
                "RP-10: the embedded semantics ID differs from the disk recomputation"
            );

            // §21.2: the older languages' IDs are the same digest over the
            // tree minus the files SPEC.md lists as introduced later. The
            // lists are read from the specification, not from the compiler.
            let (only_1_2, only_1_1) = support::introduced_later();
            let (ten, eleven, twelve) = support::semantics_ids(&borrowed);
            assert_eq!(twelve, recomputed, "RP-10: the 1.2 ID");
            assert_eq!(
                eleven,
                lexlean::compiler_semantics_id_for("1.1"),
                "RP-10: the 1.1 ID"
            );
            assert_eq!(
                ten,
                lexlean::compiler_semantics_id_for("1.0"),
                "RP-10: the 1.0 ID"
            );
            // Every listed path exists, so a renamed file cannot silently
            // drop out of a list.
            for item in only_1_2.iter().chain(&only_1_1) {
                assert!(
                    borrowed
                        .iter()
                        .any(|(path, _)| path.starts_with(item.as_str())),
                    "§21.2 lists `{item}`, which is not in the embedded tree"
                );
            }
            // The frozen identities are the ones the committed examples were
            // locked with.
            for (example, language, id) in [
                ("examples/nat-add-zero/lexlean.lock", "1.0", ten),
                ("examples/semantic-1.1/lexlean.lock", "1.1", eleven),
            ] {
                let bytes = std::fs::read(root.join(example).as_std_path()).expect("lock");
                let lock = lexlean::api::parse_lock_bytes(example, &bytes).expect("lock parses");
                assert_eq!(lock.language, language);
                assert_eq!(
                    lock.compiler_semantics, id,
                    "{example} pins the {language} ID"
                );
            }
        }
        // §30.4: every README capability claim ties to registered IDs at
        // their registered level, the claimed ranges are exactly the
        // register's, and every registered ID is claimed by one row.
        "RP-11" => {
            let readme = root_file("README.md");
            let model = repo_model::Model::load(&root.join("model").into_std_path_buf())
                .expect("the model loads");
            let mut in_table = false;
            let mut rows = 0usize;
            let mut claimed: BTreeSet<String> = BTreeSet::new();
            for line in readme.lines() {
                if line.starts_with("| Capability | IDs | Level |") {
                    in_table = true;
                    continue;
                }
                if !in_table {
                    continue;
                }
                if !line.starts_with('|') {
                    break;
                }
                if line.starts_with("| ---") {
                    continue;
                }
                let cells: Vec<&str> = line.trim_matches('|').split(" | ").map(str::trim).collect();
                assert_eq!(cells.len(), 3, "a capability row has three cells: {line}");
                rows += 1;
                assert_eq!(
                    cells[2], "`build`",
                    "RP-11: the level cell is `build`: {line}"
                );
                for claim in cells[1].split(',').map(str::trim) {
                    let ids: Vec<String> = match claim.split_once("..") {
                        Some((low, high)) => {
                            let low = low.trim_matches('`');
                            let high = high.trim_matches('`');
                            let prefix = &low[..2];
                            assert_eq!(&high[..2], prefix, "a range stays in one prefix: {claim}");
                            let registered: Vec<&str> = model
                                .ids
                                .id
                                .iter()
                                .map(|row| row.id.as_str())
                                .filter(|id| id.starts_with(prefix) && id.as_bytes()[2] == b'-')
                                .collect();
                            assert_eq!(
                                registered.first().copied(),
                                Some(low),
                                "RP-11: the range `{claim}` starts at the register's first {prefix} ID"
                            );
                            assert_eq!(
                                registered.last().copied(),
                                Some(high),
                                "RP-11: the range `{claim}` ends at the register's last {prefix} ID"
                            );
                            registered.iter().map(|id| (*id).to_owned()).collect()
                        }
                        None => vec![claim.trim_matches('`').to_owned()],
                    };
                    for id in ids {
                        assert!(
                            model.ids.get(&id).is_some(),
                            "README claims unregistered ID {id}"
                        );
                        assert!(claimed.insert(id.clone()), "README claims {id} in two rows");
                    }
                }
            }
            assert!(
                in_table && rows >= 1,
                "RP-11: the README has the capability table"
            );
            let registered: BTreeSet<String> =
                model.ids.id.iter().map(|row| row.id.clone()).collect();
            let unclaimed: Vec<&String> = registered.difference(&claimed).collect();
            assert!(
                unclaimed.is_empty(),
                "RP-11: registered IDs no README row claims: {unclaimed:?}"
            );
            let count_sentence = format!("All {} registered conformance IDs", registered.len());
            assert!(
                readme.contains(&count_sentence),
                "RP-11: the README states the exact register size: `{count_sentence}`"
            );
            assert!(
                readme.contains("honesty level `build`"),
                "RP-11: the README names the honesty level of its claims"
            );
        }
        // §30: the release gate refuses until the complete criterion holds,
        // checks every §30.3 artifact by content, and the crate package
        // builds standalone with the same semantics ID.
        "RP-12" => {
            let (_, hidden_tests) = crate::workspace_test_names_with_flags(root.as_std_path());
            assert!(
                hidden_tests.is_empty(),
                "no workspace test is ignored or cfg-hidden: {hidden_tests:?}"
            );
            let unmet = repo_model::release::check(root.as_std_path(), &hidden_tests)
                .expect_err("a 0.3.0 tree must refuse release (§2.3, §30.4)");
            // The criterion reads the attribute-block scan, so a hidden test
            // is unmet whatever attribute form hides it (§30.4).
            let planted: BTreeSet<String> = ["conformance_rp_12".to_owned()].into_iter().collect();
            let with_hidden = repo_model::release::check(root.as_std_path(), &planted)
                .expect_err("a hidden test is unmet");
            assert!(
                with_hidden
                    .iter()
                    .any(|reason| reason.contains("no-ignored-test")
                        && reason.contains("conformance_rp_12")),
                "the refusal names the hidden test: {with_hidden:?}"
            );
            assert!(
                unmet.iter().any(|reason| reason.contains("source-tag")),
                "the refusal names the version criterion"
            );
            let names: BTreeSet<&str> = repo_model::release::CRITERIA
                .iter()
                .map(|(name, _)| *name)
                .collect();
            for required in [
                "source-tag",
                "checksums",
                "host-binaries",
                "crate-package",
                "semantics-id",
                "conformance-doc",
                "errors-doc",
                "spec",
                "licenses",
                "sbom",
                "ci-evidence",
                "version-output",
                "no-ignored-test",
                "gate-evidence",
                "schemas-exercised",
            ] {
                assert!(
                    names.contains(required),
                    "§30.3/§30.4 criterion `{required}` is checked"
                );
            }
            let justfile = root_file("Justfile");
            assert!(
                justfile.contains("release: vv") && justfile.contains("cargo xtask release-check"),
                "the release recipe runs the full gate then the release check"
            );

            // A synthetic release tree satisfies the content checks except
            // the version, proving the shape checks discriminate.
            let staged = tempfile::tempdir().expect("tempdir");
            let stage =
                camino::Utf8PathBuf::from_path_buf(staged.path().to_path_buf()).expect("utf8");
            for relative in [
                "Cargo.toml",
                "SPEC.md",
                "LICENSE-APACHE",
                "LICENSE-MIT",
                "CONFORMANCE.md",
                "ERRORS.md",
                "VERIFICATION.md",
                "Justfile",
                "CHANGELOG.md",
            ] {
                let source = root.join(relative);
                if source.as_std_path().is_file() {
                    std::fs::copy(source.as_std_path(), stage.join(relative).as_std_path())
                        .expect("copy");
                }
            }
            // Recursive, because `model/` holds the vendored authorities the
            // model check rehashes as well as its registers.
            let model = root.join("model");
            for entry in walkdir::WalkDir::new(model.as_std_path())
                .into_iter()
                .flatten()
            {
                let target = stage.join("model").as_std_path().join(
                    entry
                        .path()
                        .strip_prefix(model.as_std_path())
                        .expect("under the model"),
                );
                if entry.file_type().is_dir() {
                    std::fs::create_dir_all(&target).expect("mkdir");
                } else {
                    std::fs::copy(entry.path(), &target).expect("copy");
                }
            }
            let refused = repo_model::release::check(stage.as_std_path(), &BTreeSet::new())
                .expect_err("no release/ directory");
            for name in ["checksums", "sbom", "crate-package", "version-output"] {
                assert!(
                    refused.iter().any(|reason| reason.starts_with(name)),
                    "criterion `{name}` is reported unmet on a tree without release artifacts: {refused:?}"
                );
            }

            // The packaged crate builds standalone and reports the same
            // four-line identity as the in-repository binary (§30.3).
            let (exit, in_repo, _) = support::cli_in(&root, &["--version"]);
            assert_eq!(exit, 0);
            let packaged = support::packaged_crate_version(&root)
                .unwrap_or_else(|error| panic!("the crate package must build standalone: {error}"));
            assert_eq!(
                packaged, in_repo,
                "the packaged crate reports the same version, language, semantics ID, and toolchain"
            );
        }
        // §9.2: the ordinary test phase excludes repo-conformance, `bdd` alone
        // owns its complete suite, and the inventory proves the partition, of
        // the workspace into the two phases and of `bdd` into the parts of its
        // committed plan (§9.4). The inventory itself lists tests with `cargo
        // test -- --list`; it is a recipe of `vv`, one line of the log that
        // runs once after both phases, and this case holds the rest: the
        // recipe lines, the comparisons it applies, and that the documents say
        // what runs.
        "RP-13" => {
            let justfile = root_file("Justfile");
            let recipes = just_recipes(&justfile);
            let command = |name: &str| -> String {
                let (_, lines) = recipes
                    .get(name)
                    .unwrap_or_else(|| panic!("the Justfile defines `{name}`"));
                assert_eq!(lines.len(), 1, "`{name}` is one command");
                lines[0].clone()
            };
            let (test, bdd, inventory) = (command("test"), command("bdd"), command("inventory"));
            assert_eq!(
                test,
                "cargo test --workspace --exclude repo-conformance --all-features"
            );
            assert_eq!(bdd, "cargo xtask bdd");
            assert_eq!(inventory, "cargo xtask check-test-inventory");
            // The recipe runs one `cargo test`, the one the phase always was,
            // whose arguments the task and the inventory read from one place.
            let bdd_command = format!("cargo test {}", crate::bdd_parts::CARGO_ARGUMENTS.join(" "));
            assert_eq!(bdd_command, "cargo test -p repo-conformance --all-features");
            let test_arguments =
                crate::inventory::recipe_arguments(&test).expect("test is cargo test");
            let bdd_arguments =
                crate::inventory::recipe_arguments(&bdd_command).expect("bdd is cargo test");
            assert!(
                test_arguments
                    .windows(2)
                    .any(|pair| pair == ["--exclude", "repo-conformance"]),
                "the ordinary phase excludes repo-conformance"
            );
            assert!(
                !bdd_arguments.iter().any(|argument| matches!(
                    argument.as_str(),
                    "--test" | "--lib" | "--doc" | "--bins" | "--bin" | "--exclude" | "--skip"
                )),
                "`bdd` selects no target of the crate it owns: {bdd_arguments:?}"
            );
            let vv = recipes
                .get("vv")
                .expect("vv")
                .0
                .split_whitespace()
                .collect::<Vec<_>>();
            let position = |name: &str| {
                vv.iter()
                    .position(|recipe| *recipe == name)
                    .unwrap_or_else(|| panic!("vv runs `{name}`"))
            };
            assert!(
                position("test") < position("bdd") && position("bdd") < position("inventory"),
                "the inventory follows both phases"
            );
            assert_eq!(
                vv.iter().filter(|recipe| **recipe == "inventory").count(),
                1
            );

            // The comparison refuses a test run twice, a test lost, and a
            // target of the owned crate in the ordinary phase.
            let list = |targets: &[(&str, &str, &[&str])]| -> crate::inventory::Parsed {
                let mut text = String::new();
                for (description, binary, tests) in targets {
                    text.push_str(&format!(
                        "     Running {description} (target/debug/deps/{binary})\n"
                    ));
                    for name in *tests {
                        text.push_str(&format!("{name}: test\n"));
                    }
                }
                crate::inventory::parse(&text).expect("parses")
            };
            let crates: BTreeSet<String> = ["repo_conformance", "bdd", "conformance"]
                .into_iter()
                .map(str::to_owned)
                .collect();
            let unit = (
                "unittests src/lib.rs",
                "lexlean-0123456789abcdef",
                &["a", "b"][..],
            );
            let conformance = (
                "tests/conformance.rs",
                "conformance-0123456789abcdef",
                &["conformance_rp_01", "conformance_rp_02"][..],
            );
            let bdd_target = ("tests/bdd.rs", "bdd-0123456789abcdef", &["meta"][..]);
            let whole = list(&[unit, conformance, bdd_target]);
            let ordinary = list(&[unit]);
            let owned = list(&[conformance, bdd_target]);
            crate::inventory::compare(&whole, &ordinary, &owned, &crates)
                .expect("a partition is accepted");
            let twice =
                crate::inventory::compare(&whole, &list(&[unit, conformance]), &owned, &crates)
                    .expect_err("a test run in both phases");
            assert!(twice.contains("both phases"), "{twice}");
            let lost = crate::inventory::compare(&whole, &ordinary, &list(&[bdd_target]), &crates)
                .expect_err("tests lost");
            assert!(lost.contains("neither phase"), "{lost}");

            // The parts of `bdd` (§9.4): the committed plan reads, every ID it
            // names has its test, and the comparison the inventory applies to
            // the parts' lists refuses a test two parts run and one no part
            // runs.
            let plan = crate::bdd_parts::Plan::read(support::repo_root().as_std_path())
                .expect("§9.4: the committed bdd plan reads");
            let tests = root_file("crates/conformance/tests/conformance.rs");
            for name in plan.named_tests() {
                assert!(
                    tests.contains(&format!("fn {name}()")),
                    "§9.4: the bdd plan names `{name}`, which is no conformance test"
                );
            }
            let plants = crate::bdd_parts::Plan::parse(
                r#"{ "remainder": "rest", "parts": { "first": ["RP-01"] } }"#,
            )
            .expect("a plan");
            let first = list(&[(
                "tests/conformance.rs",
                "conformance-0123456789abcdef",
                &["conformance_rp_01"][..],
            )]);
            let remainder = list(&[
                (
                    "tests/conformance.rs",
                    "conformance-0123456789abcdef",
                    &["conformance_rp_02"][..],
                ),
                bdd_target,
            ]);
            let parts = |first: &crate::inventory::Parsed, rest: &crate::inventory::Parsed| {
                vec![
                    ("first".to_owned(), first.clone()),
                    ("rest".to_owned(), rest.clone()),
                ]
            };
            crate::inventory::compare_parts(&owned, &parts(&first, &remainder), &plants)
                .expect("parts that partition bdd are accepted");
            let twice = crate::inventory::compare_parts(&owned, &parts(&first, &owned), &plants)
                .expect_err("a test two parts run");
            assert!(twice.contains("parts `first` and `rest`"), "{twice}");
            let lost = crate::inventory::compare_parts(
                &owned,
                &parts(&first, &list(&[bdd_target])),
                &plants,
            )
            .expect_err("a test no part runs");
            assert!(lost.contains("run in no part"), "{lost}");
            // The task runs the part the variable names through the plan's
            // filters, after the arguments of the whole phase, and the
            // inventory lists with the same filters.
            let task = root_file("xtask/src/ci.rs");
            assert!(
                task.contains(".args(CARGO_ARGUMENTS)")
                    && task.contains("std::env::var(PART_VARIABLE)")
                    && task.contains("plan.harness_filters(part)")
                    && task.contains("plan.harness_filters(&name)")
                    && task.contains("inventory::compare_parts("),
                "the bdd task and the inventory select a part through the plan's filters"
            );

            // The documents say what runs.
            let xtask = root_file("xtask/src/main.rs");
            assert!(xtask.contains("\"check-test-inventory\" =>"));
            assert!(xtask.contains("\"bdd\" => ci::bdd(&root)"));
            let spec = root_file("SPEC.md");
            let verification = root_file("VERIFICATION.md");
            for recipe_command in [&test, &bdd, &bdd_command, &inventory] {
                assert!(
                    verification.contains(&format!("`{recipe_command}`")),
                    "VERIFICATION.md names the command `{recipe_command}`"
                );
                assert!(
                    spec.contains(recipe_command.as_str()),
                    "SPEC.md names `{recipe_command}`"
                );
            }
            // The honesty workflow keeps its own signal, and the host matrix
            // (not part of vv) keeps the whole workspace on every host.
            assert!(root_file(".github/workflows/honesty.yml")
                .contains("cargo test --locked -p repo-conformance --test bdd"));
            assert!(root_file(".github/workflows/ci.yml")
                .contains("cargo test --locked --workspace --all-features"));
        }
        // §9.4: the normative job runs on the standard runner at the default
        // width and keeps no cache, filter, or skipped recipe.
        "RP-14" => {
            let ci = root_file(".github/workflows/ci.yml");
            for variable in ["vars.", "inputs."] {
                assert!(
                    !ci.contains(variable),
                    "the gate selects no runner and no profile from outside the file: `{variable}`"
                );
            }
            // The jobs of the file, each with its lines: a key at two spaces
            // of indentation under `jobs:` opens one.
            let jobs_text = &ci[ci.find("\njobs:\n").expect("a jobs section") + 7..];
            let mut jobs: Vec<(String, String)> = Vec::new();
            for line in jobs_text.lines() {
                if line.starts_with("  ")
                    && !line.starts_with("   ")
                    && line.trim_end().ends_with(':')
                {
                    jobs.push((line.trim().trim_end_matches(':').to_owned(), String::new()));
                } else if let Some((_, body)) = jobs.last_mut() {
                    body.push_str(line);
                    body.push('\n');
                }
            }
            let job = |name: &str| -> &str {
                &jobs
                    .iter()
                    .find(|(key, _)| key == name)
                    .unwrap_or_else(|| panic!("§9.4: the job `{name}`"))
                    .1
            };
            let atlas_jobs: Vec<&(String, String)> = jobs
                .iter()
                .filter(|(key, _)| key.starts_with("atlas-"))
                .collect();
            assert!(!atlas_jobs.is_empty(), "§9.4: the Atlas jobs");
            // Every gate job runs on the standard runner (§9.4); only the host
            // matrix runs elsewhere, on the hosts of §8.3.
            for (key, body) in &jobs {
                if key == "host-matrix" {
                    continue;
                }
                assert!(
                    body.contains("\n    runs-on: ubuntu-24.04\n"),
                    "§9.4: the job `{key}` runs on the standard runner"
                );
                // The plan decides what the run is and runs no recipe: it has
                // ten minutes, not the six hours of the proof jobs.
                let limit = if key == "plan" { 10 } else { 360 };
                assert!(
                    body.contains(&format!("\n    timeout-minutes: {limit}\n")),
                    "§9.4: the job `{key}` has its limit of {limit} minutes"
                );
            }
            // The resource profile (§22.11) is chosen by the leg's matrix row
            // and by nothing else: each variable is set once on the gate job,
            // from the row. An Atlas job names the examples profile itself,
            // once, the same on every shard.
            let vv = job("vv");
            for (variable, line) in [
                ("LEXLEAN_VERIFY_WIDTH", "      LEXLEAN_VERIFY_WIDTH: ${{ matrix.width }}\n"),
                (
                    "LEXLEAN_VERIFY_MEMORY_KIB",
                    "      LEXLEAN_VERIFY_MEMORY_KIB: ${{ matrix.memory_kib }}\n",
                ),
                (
                    "LEXLEAN_VERIFY_HINTS",
                    "      LEXLEAN_VERIFY_HINTS: ${{ matrix.hints && format('{0}/{1}', github.workspace, matrix.hints) || '' }}\n",
                ),
                (
                    "LEXLEAN_EXAMPLES_PART",
                    "      LEXLEAN_EXAMPLES_PART: ${{ matrix.examples_part }}\n",
                ),
                (
                    crate::bdd_parts::PART_VARIABLE,
                    "      LEXLEAN_BDD_PART: ${{ matrix.bdd_part }}\n",
                ),
            ] {
                assert_eq!(vv.matches(variable).count(), 1, "§9.4: `{variable}` is set once on the gate job");
                assert!(vv.contains(line), "§9.4: `{variable}` is read from the matrix row");
            }
            for (key, body) in &atlas_jobs {
                for line in [
                    "      LEXLEAN_VERIFY_WIDTH: \"2\"\n",
                    "      LEXLEAN_VERIFY_MEMORY_KIB: \"12582912\"\n",
                    "      LEXLEAN_VERIFY_HINTS: ${{ github.workspace }}/.github/verify-hints.json\n",
                    "      LEXLEAN_EXAMPLES_PARTS: ${{ github.workspace }}/ci-parts\n",
                    "      LEXLEAN_CI_TELEMETRY: ${{ github.workspace }}/ci-evidence\n",
                ] {
                    assert_eq!(body.matches(line).count(), 1, "§9.4: `{key}` sets {line:?} once");
                }
                assert_eq!(
                    body.matches("LEXLEAN_EXAMPLES_PART:").count(),
                    1,
                    "§9.4: `{key}` names its part once"
                );
            }
            // The gate is one matrix job whose legs run the recipes of `vv`
            // (§9.4): every recipe exactly once, but `bdd`, which runs once per
            // part of its committed plan, in the order of `vv` within a leg,
            // each leg through the one gate command.
            let recipes_of_vv: Vec<String> = just_recipes(&root_file("Justfile"))
                .get("vv")
                .expect("§9.2: the Justfile defines vv")
                .0
                .split_whitespace()
                .map(str::to_owned)
                .collect();
            // A leg, a row of `.github/vv-rows.json`, which the plan job turns
            // into the matrix: `{ part, recipes, examples_part, bdd_part,
            // width, memory_kib, hints }`, every value a string. A draft pull
            // request runs the row whose part is `rest` and no other.
            let field = |row: &serde_json::Value, name: &str| -> Option<String> {
                row.get(name)?.as_str().map(str::to_owned)
            };
            let rows: Vec<serde_json::Value> =
                serde_json::from_str(&root_file(".github/vv-rows.json"))
                    .expect("§9.4: the legs file is a JSON array");
            for row in &rows {
                assert_eq!(
                    row.as_object()
                        .map(|keys| keys.keys().cloned().collect::<Vec<_>>()),
                    Some(
                        [
                            "bdd_part",
                            "examples_part",
                            "hints",
                            "memory_kib",
                            "part",
                            "recipes",
                            "width"
                        ]
                        .map(str::to_owned)
                        .to_vec()
                    ),
                    "§9.4: a leg names exactly its part, recipes, and profile: {row}"
                );
            }
            assert_eq!(
                rows.iter()
                    .filter(|row| field(row, "part").as_deref() == Some("rest"))
                    .count(),
                1,
                "§9.4: the leg a draft pull request runs is the one named `rest`"
            );
            let legs: Vec<Vec<String>> = rows
                .iter()
                .filter_map(|row| {
                    let recipes = field(row, "recipes")?;
                    Some(recipes.split_whitespace().map(str::to_owned).collect())
                })
                .collect();
            assert_eq!(
                legs.len(),
                rows.len(),
                "every leg names its recipes: {rows:?}"
            );
            assert!(legs.len() >= 2, "the gate runs in legs: {legs:?}");
            // The leg that runs `examples` verifies at width 2 with the
            // committed hints and a 12 GiB budget, and every example but the
            // one the Atlas jobs verify as parts; every other leg names no
            // profile and no part (§9.4, §22.11). The values are the
            // verifier's to accept: a width it refuses, or hints it cannot
            // read, would fail the gate as `LLV7017` before any stage ran, so
            // they are parsed here with the verifier's own functions.
            for (row, leg) in rows.iter().zip(&legs) {
                let profile = ["width", "memory_kib", "hints", "examples_part"].map(|name| {
                    field(row, name).unwrap_or_else(|| panic!("§9.4: the row {row} names `{name}`"))
                });
                if leg.iter().any(|recipe| recipe == "examples") {
                    assert_eq!(
                        profile,
                        [
                            "2",
                            "12582912",
                            ".github/verify-hints.json",
                            "without:uor-atlas"
                        ]
                        .map(str::to_owned),
                        "§9.4: the examples leg's profile and part"
                    );
                    assert_eq!(
                        lexlean::verify::operational::Operational::parse_width(Some(&profile[0]))
                            .expect("the width is one the verifier accepts"),
                        2
                    );
                    let hints = lexlean::verify::operational::Operational::parse_hints(&root_file(
                        &profile[2],
                    ))
                    .expect("the committed hints are a hints file the verifier reads");
                    assert!(
                        hints.contains_key("LexLeanExample.Census"),
                        "the hints cover the Atlas module that dominates the verification"
                    );
                } else {
                    assert_eq!(
                        profile,
                        ["", "", "", ""].map(str::to_owned),
                        "§9.4: a leg that does not run examples names no profile and no part: {row}"
                    );
                }
            }
            // `bdd` runs as the parts of its committed plan (§9.4): a row runs
            // the recipe exactly when it names a part, every part of the plan,
            // the remainder included, is named by exactly one row, and a row
            // that runs `bdd` alone is named after its part.
            let bdd_plan = crate::bdd_parts::Plan::read(support::repo_root().as_std_path())
                .expect("§9.4: the committed bdd plan reads");
            let mut bdd_rows: std::collections::BTreeMap<String, String> =
                std::collections::BTreeMap::new();
            for (row, leg) in rows.iter().zip(&legs) {
                let part = field(row, "bdd_part")
                    .unwrap_or_else(|| panic!("§9.4: the row {row} names `bdd_part`"));
                let runs_bdd = leg.iter().any(|recipe| recipe == "bdd");
                assert_eq!(
                    runs_bdd,
                    !part.is_empty(),
                    "§9.4: a row names a part of bdd exactly when it runs bdd: {row}"
                );
                if !runs_bdd {
                    continue;
                }
                assert!(
                    bdd_plan.names().contains(&part),
                    "§9.4: the row {row} runs `{part}`, which is no part of {}",
                    crate::bdd_parts::PLAN_FILE
                );
                if leg.len() == 1 {
                    assert_eq!(
                        field(row, "part").as_deref(),
                        Some(format!("bdd-{part}").as_str()),
                        "§9.4: a row that runs bdd alone is named `bdd-{part}`: {row}"
                    );
                }
                assert!(
                    bdd_rows.insert(part.clone(), row.to_string()).is_none(),
                    "§9.4: the part `{part}` of bdd is run by two rows"
                );
            }
            for part in bdd_plan.names() {
                assert!(
                    bdd_rows.contains_key(&part),
                    "§9.4: the part `{part}` of bdd is run by no row"
                );
            }
            let mut ran: Vec<&String> = legs
                .iter()
                .flatten()
                .filter(|recipe| *recipe != "bdd")
                .collect();
            for leg in &legs {
                let positions: Vec<usize> = leg
                    .iter()
                    .map(|recipe| {
                        recipes_of_vv.iter().position(|name| name == recipe).unwrap_or_else(|| {
                            panic!("§9.4: the leg {leg:?} runs `{recipe}`, which is not a recipe of vv")
                        })
                    })
                    .collect();
                assert!(
                    positions.windows(2).all(|pair| pair[0] < pair[1]),
                    "§9.4: the leg {leg:?} runs its recipes in the order of vv"
                );
            }
            ran.sort();
            let mut all: Vec<&String> = recipes_of_vv
                .iter()
                .filter(|recipe| *recipe != "bdd")
                .collect();
            all.sort();
            assert_eq!(
                ran, all,
                "§9.4: the legs run every recipe of vv but bdd exactly once"
            );
            assert_eq!(
                legs.iter()
                    .flatten()
                    .filter(|recipe| *recipe == "bdd")
                    .count(),
                bdd_plan.names().len(),
                "§9.4: the legs run bdd once per part of its plan"
            );
            assert_eq!(
                ci.matches(
                    "just --timestamp --timestamp-format '%s%.3f' ${{ matrix.recipes }} 2>&1 | tee ci-evidence/vv.txt"
                )
                .count(),
                1,
                "every leg runs its recipes through the one gate command"
            );
            assert!(
                ci.contains(&format!(
                    "'{}' >> ci-evidence/vv.txt",
                    crate::evidence::PART_PASSED_LINE
                )),
                "a leg marks the end of its last recipe as the summary reads it"
            );
            // The leg's assertion is chosen by its recipes, never by what the
            // log happens to hold, and it refuses a leg that verified Atlas,
            // which the join verifies.
            assert!(
                vv.contains("case \" ${{ matrix.recipes }} \" in")
                    && vv.contains("*\" examples \"*)")
                    && vv.contains(
                        "if grep -F \"verify-examples: uor-atlas\" ci-evidence/vv.txt; then"
                    ),
                "the leg that runs examples asserts that it verified the examples and not Atlas"
            );
            // The Atlas jobs run every shard of the committed plan exactly
            // once, each after the jobs of the shards it needs, and the join
            // asserts that Atlas verified (§9.4). The plan itself is checked
            // as `cargo xtask ci-shards` checks it.
            let plan = crate::shards::Plan::read(support::repo_root().as_std_path())
                .expect("§9.4: the committed plan reads");
            assert_eq!(plan.project, "uor-atlas");
            let graph = crate::shards::import_graph(
                support::repo_root()
                    .join("examples")
                    .join(&plan.project)
                    .as_std_path(),
            )
            .expect("the Atlas import graph reads from its committed build");
            crate::shards::check(&plan, &graph)
                .expect("§9.4: the plan runs every process once in import order");
            // Which job runs which shard: a literal part, or the rows of a
            // `matrix: { shard: [...] }` with the part naming `matrix.shard`.
            let mut runs: std::collections::BTreeMap<String, String> =
                std::collections::BTreeMap::new();
            let mut needs: std::collections::BTreeMap<String, Vec<String>> =
                std::collections::BTreeMap::new();
            for (key, body) in &atlas_jobs {
                let listed: Vec<String> = body
                    .lines()
                    .find_map(|line| line.trim().strip_prefix("needs: ["))
                    .map(|rest| {
                        rest.trim_end_matches(']')
                            .split(',')
                            .map(|name| name.trim().to_owned())
                            .filter(|name| !name.is_empty())
                            .collect()
                    })
                    .unwrap_or_default();
                needs.insert((*key).clone(), listed);
                let part = body
                    .lines()
                    .find_map(|line| line.trim().strip_prefix("LEXLEAN_EXAMPLES_PART: "))
                    .unwrap_or_else(|| panic!("§9.4: `{key}` names its part"));
                let shards: Vec<String> = if part == "join:uor-atlas" {
                    vec![crate::shards::JOIN.to_owned()]
                } else if part == "shard:uor-atlas:${{ matrix.shard }}" {
                    let matrix = body
                        .lines()
                        .find_map(|line| {
                            line.trim()
                                .strip_prefix("strategy: { fail-fast: false, matrix: { shard: [")
                        })
                        .unwrap_or_else(|| panic!("§9.4: `{key}` runs the shards of its matrix"));
                    matrix
                        .split(']')
                        .next()
                        .unwrap_or_default()
                        .split(',')
                        .map(|name| name.trim().to_owned())
                        .collect()
                } else {
                    let name = part.strip_prefix("shard:uor-atlas:").unwrap_or_else(|| {
                        panic!("§9.4: `{key}` runs a shard of Atlas or the join: {part}")
                    });
                    assert!(
                        !body.contains("matrix:"),
                        "§9.4: `{key}` runs one shard and has no matrix"
                    );
                    vec![name.to_owned()]
                };
                for shard in shards {
                    assert!(
                        plan.shards.contains_key(&shard),
                        "§9.4: `{key}` runs `{shard}`, which is no shard of the plan"
                    );
                    assert!(
                        runs.insert(shard.clone(), (*key).clone()).is_none(),
                        "§9.4: the shard `{shard}` is run by two jobs"
                    );
                }
                assert!(
                    body.contains("    if: ${{ needs.plan.outputs.full == 'true' }}\n")
                        && needs
                            .get(key)
                            .is_some_and(|list| list.iter().any(|need| need == "plan")),
                    "§9.4: `{key}` runs only when the plan says the run is the whole gate"
                );
                assert!(
                    body.contains("name: acceptance-gate-evidence-atlas-")
                        && body.contains("path: ci-evidence\n"),
                    "§9.4: `{key}` uploads its evidence under a name of its own"
                );
                assert!(
                    body.contains(" ci-summary --evidence ci-evidence\n"),
                    "§9.4: `{key}` summarizes its run"
                );
            }
            for shard in plan.shards.keys() {
                assert!(
                    runs.contains_key(shard),
                    "§9.4: the shard `{shard}` is run by no job"
                );
            }
            // The job graph orders each shard after the jobs of the shards it
            // needs, directly or through another job; a shard never needs one
            // of its own job's rows, which run at once.
            let after = |job_key: &str| -> std::collections::BTreeSet<String> {
                let mut closure = std::collections::BTreeSet::new();
                let mut stack = vec![job_key.to_owned()];
                while let Some(key) = stack.pop() {
                    for need in needs.get(&key).map(Vec::as_slice).unwrap_or_default() {
                        if closure.insert(need.clone()) {
                            stack.push(need.clone());
                        }
                    }
                }
                closure
            };
            for (shard, plan_shard) in &plan.shards {
                let job_key = &runs[shard];
                let before = after(job_key);
                for need in &plan_shard.needs {
                    let need_job = &runs[need];
                    assert!(
                        need_job != job_key && before.contains(need_job),
                        "§9.4: `{shard}` runs in `{job_key}`, which does not wait for `{need_job}`, the job of `{need}` it needs"
                    );
                }
            }
            // The one build of the gate's executable is the prefix job's, from
            // its clean checkout; every other Atlas job takes it and the
            // shards' output from the run, and the shards upload their output
            // under their names. Nothing crosses runs: no cache action, no
            // artifact of another run.
            let prefix_job = &runs["prefix"];
            assert_eq!(
                ci.matches("cargo build").count(),
                1,
                "ci.yml builds the gate's executable once"
            );
            assert!(
                job(prefix_job).contains("        run: cargo build --locked --package xtask\n")
                    && job(prefix_job).contains("name: atlas-xtask\n")
                    && job(prefix_job).contains("path: target/debug/xtask\n"),
                "§9.4: the prefix job builds the executable from its clean checkout and uploads it"
            );
            for (key, body) in &atlas_jobs {
                if key == prefix_job {
                    assert!(
                        !body.contains("download-artifact"),
                        "the prefix job takes nothing from the run"
                    );
                    continue;
                }
                assert!(
                    body.contains("          pattern: atlas-*\n")
                        && body.contains("          path: ci-parts\n")
                        && body.contains("chmod +x ci-parts/atlas-xtask/xtask\n")
                        && body.contains("ci-parts/atlas-xtask/xtask verify-examples 2>&1 | tee ci-evidence/vv.txt\n"),
                    "§9.4: `{key}` takes the executable and the shards from the run and runs the gate's recipe through it"
                );
                assert!(
                    !body.contains("run-id") && !body.contains("github-token"),
                    "§9.4: `{key}` reads the artifacts of this run only"
                );
            }
            for (shard, job_key) in &runs {
                if shard == crate::shards::JOIN {
                    assert!(
                        !job(job_key).contains("name: atlas-part-"),
                        "the join uploads no part"
                    );
                    continue;
                }
                let body = job(job_key);
                let uploaded = if body.contains("matrix: { shard: [") {
                    "name: atlas-part-${{ matrix.shard }}\n".to_owned()
                } else {
                    format!("name: atlas-part-{shard}\n")
                };
                assert!(
                    body.contains(&uploaded),
                    "§9.4: `{job_key}` uploads the output of `{shard}`"
                );
            }
            let join_job = job(&runs[crate::shards::JOIN]);
            assert!(
                join_job.contains("grep -F \"verify-examples: uor-atlas verified (attestation \" ci-evidence/vv.txt")
                    && join_job.contains("grep -F \"verify-examples: uor-atlas: 347 normalized verification records equal expected/verify (§29.5)\" ci-evidence/vv.txt"),
                "the join asserts that Atlas verified"
            );
            assert!(
                ci.contains("  pull_request:\n    types: [opened, synchronize, reopened, ready_for_review]\n"),
                "§9.4: a pull request marked ready runs the jobs the draft did not"
            );
            for forbidden in [
                "actions/cache",
                "paths:",
                "paths-ignore:",
                "restore-keys",
                "continue-on-error",
                "if: github.event.pull_request",
            ] {
                assert!(!ci.contains(forbidden), "ci.yml has no `{forbidden}`");
            }
            // A condition appears in three forms and nowhere else: a step's
            // `if: ${{ !cancelled() }}` on the summary and evidence steps, so
            // that a failed run still reports; a job's `full` condition on the
            // Atlas jobs and the host matrix, which run only when the plan says
            // the run is the whole gate; and the legs' condition on `vv`, which
            // runs when the plan names a leg. Never on the gate step, and never
            // a condition on the event or the pull request itself: what a run
            // is, is decided once, by the plan (a gate skipped by a condition
            // written elsewhere is the weaker PR-only gate).
            let mut step_conditions = 0;
            let mut full_conditions = 0;
            let mut rows_conditions = 0;
            for (index, line) in ci.lines().enumerate() {
                if line.trim_start().starts_with("if:") {
                    match line {
                        "        if: ${{ !cancelled() }}" => step_conditions += 1,
                        "    if: ${{ needs.plan.outputs.full == 'true' }}" => full_conditions += 1,
                        "    if: ${{ needs.plan.outputs.rows != '[]' }}" => rows_conditions += 1,
                        other => panic!(
                            "ci.yml:{}: a condition other than a step's `!cancelled()` or a job's condition on the plan: {other}",
                            index + 1
                        ),
                    }
                }
            }
            assert_eq!(
                step_conditions,
                2 + 2 * atlas_jobs.len(),
                "the summary step and the evidence upload of every gate job carry it"
            );
            assert_eq!(
                full_conditions,
                atlas_jobs.len() + 1,
                "every Atlas job and the host matrix, and no other job, wait for the plan to name the whole gate"
            );
            assert_eq!(rows_conditions, 1, "the legs run when the plan names a leg");
            assert!(
                job("host-matrix").contains("    needs: [plan]\n")
                    && job("host-matrix")
                        .contains("    if: ${{ needs.plan.outputs.full == 'true' }}\n"),
                "§8.3: the hosts are checked when the plan says the run is the whole gate"
            );
            assert!(
                vv.contains("    needs: [plan]\n")
                    && vv.contains("        include: ${{ fromJSON(needs.plan.outputs.rows) }}\n"),
                "§9.4: the legs of vv are the rows the plan names"
            );
            // The plan: one job, no recipe, ten minutes, reading the event and
            // the repository and nothing else, through the one script whose
            // decisions are run here against planted events.
            let plan_job = job("plan");
            assert!(
                plan_job.contains("      actions: read\n")
                    && plan_job.contains("          fetch-depth: 2\n")
                    && plan_job.contains("        run: .github/ci-plan.sh\n")
                    && plan_job.contains("          DRAFT: ${{ github.event.pull_request.draft }}\n")
                    && plan_job.contains("          EVENT: ${{ github.event_name }}\n"),
                "§9.4: the plan runs the script on the event and may read the runs of the repository"
            );
            // The workflow runs the file as a command: a mode that is not
            // executable fails the plan, and with it every job that needs it.
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mode = std::fs::metadata(
                    support::repo_root()
                        .join(".github/ci-plan.sh")
                        .as_std_path(),
                )
                .expect("the plan script")
                .permissions()
                .mode();
                assert!(mode & 0o111 != 0, "§9.4: .github/ci-plan.sh is executable");
            }
            plan_decisions();
            let gate_step = &ci[ci
                .find("      - name: Run the normative acceptance gate")
                .expect("the gate step")..];
            let gate_step =
                &gate_step[..gate_step[1..].find("\n      - ").expect("a step follows") + 1];
            assert!(!gate_step.contains("if:") && gate_step.contains("just --timestamp"));
            assert!(
                ci.contains("name: acceptance-gate-evidence-${{ matrix.part }}")
                    && ci.contains("path: ci-evidence"),
                "each leg uploads its evidence under a name of its own"
            );
            assert!(ci.contains("LEXLEAN_CI_TELEMETRY: ${{ github.workspace }}/ci-evidence"));
            assert!(ci.contains("cargo xtask ci-summary --evidence ci-evidence"));
            // The summary reaches the job log and the step summary, and does
            // not depend on the gate having passed. `evidence::publish` has the
            // unit tests (a plant in it fails them); this pins that the task
            // calls it with the process environment and the process's stdout,
            // so neither can be dropped from the task itself.
            let task = root_file("xtask/src/ci.rs");
            assert!(
                task.contains("evidence::publish(")
                    && task.contains("&|name| std::env::var_os(name)")
                    && task.contains("&mut std::io::stdout().lock()"),
                "ci-summary prints to stdout and reads GITHUB_STEP_SUMMARY from the environment"
            );
            // The host matrix builds through its one `cargo test`: the same
            // profile, locking, and features build the crate's library and
            // binary, so a separate build step is time spent on nothing the
            // test does not already require (§8.3).
            let matrix = job("host-matrix");
            assert_eq!(
                matrix
                    .matches("cargo test --locked --workspace --all-features --no-fail-fast\n")
                    .count(),
                1,
                "the host matrix runs the workspace's tests once, and reports every failing test of the host, not the first binary's"
            );
            // The honesty and reproducibility workflows stay separate.
            assert!(root_file(".github/workflows/honesty.yml").contains("just model spec-links"));
            assert!(root_file(".github/workflows/reproducibility.yml").contains("run: just repro"));
        }
        // §9.3: the image installs the pins and nothing else. The checker is
        // run on planted Dockerfiles as well, because a check that only ever
        // sees the committed file cannot be told from one that accepts all.
        "RP-16" => {
            let rust = root_file("rust-toolchain.toml");
            let channel = rust
                .lines()
                .find_map(|line| line.trim().strip_prefix("channel = \""))
                .and_then(|rest| rest.strip_suffix('"'))
                .expect("rust-toolchain.toml names a channel");
            let lean = root_file("lean-toolchain");
            let lean = lean.trim();
            let dockerfile = root_file(".devcontainer/Dockerfile");
            devcontainer_pins(&dockerfile, channel, lean)
                .expect("the devcontainer installs exactly the pinned toolchains");
            for (planted, why) in [
                (dockerfile.replace(channel, "1.0.0"), "another Rust version"),
                (
                    dockerfile.replace(lean, "leanprover/lean4:v4.0.0"),
                    "another Lean version",
                ),
                (
                    format!("{dockerfile}RUN elan toolchain install leanprover/lean4:v4.0.0\n"),
                    "a second Lean toolchain",
                ),
                (
                    format!("{dockerfile}RUN rustup toolchain install nightly\n"),
                    "a second Rust toolchain",
                ),
                (
                    format!("{dockerfile}RUN elan default leanprover/lean4:stable\n"),
                    "an unpinned Lean default",
                ),
                (
                    dockerfile.replace(
                        &format!("--default-toolchain {channel}"),
                        "--default-toolchain stable",
                    ),
                    "an unpinned Rust default",
                ),
            ] {
                assert!(
                    devcontainer_pins(&planted, channel, lean).is_err(),
                    "a Dockerfile with {why} is refused"
                );
            }
        }
        other => panic!("no repository case is wired for {other}"),
    }
}

/// RP-16: every toolchain the Dockerfile installs is the pinned one. A Rust
/// toolchain is installed by `--default-toolchain <v>` or `rustup toolchain
/// install <v>`, a Lean toolchain by `elan toolchain install <v>`, and
/// `elan default` would select one; the pin must appear and nothing else may.
fn devcontainer_pins(dockerfile: &str, channel: &str, lean: &str) -> Result<(), String> {
    let words: Vec<&str> = dockerfile.split_whitespace().collect();
    let after = |lead: &[&str]| -> Vec<&str> {
        words
            .windows(lead.len() + 1)
            .filter(|window| window[..lead.len()] == *lead)
            .map(|window| window[lead.len()])
            .collect()
    };
    // `--default-toolchain none` is elan's own installer asking for no
    // toolchain, which the explicit pinned install after it then supplies.
    let rust: Vec<&str> = after(&["--default-toolchain"])
        .into_iter()
        .filter(|version| *version != "none")
        .chain(after(&["rustup", "toolchain", "install"]))
        .collect();
    let leans = after(&["elan", "toolchain", "install"]);
    if rust != [channel] {
        return Err(format!(
            "Rust toolchains installed {rust:?}, pinned {channel}"
        ));
    }
    if leans != [lean] {
        return Err(format!(
            "Lean toolchains installed {leans:?}, pinned {lean}"
        ));
    }
    if words.windows(2).any(|pair| pair == ["elan", "default"]) {
        return Err("`elan default` selects a toolchain".to_owned());
    }
    Ok(())
}
