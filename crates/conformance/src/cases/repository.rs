//! The `repository` suite: RP-01..RP-16 and RP-18.

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

/// The auxiliary §8.3 contexts are evidence identities, not a matrix size.
/// Linux x86-64 is covered by the normative gate rather than this list.
fn supported_host_contexts() -> Vec<String> {
    [
        "aarch64-unknown-linux-gnu",
        "x86_64-apple-darwin",
        "aarch64-apple-darwin",
        "x86_64-pc-windows-msvc",
    ]
    .into_iter()
    .map(|target| format!("build and non-Lean tests ({target})"))
    .collect()
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
    let rows: Vec<serde_json::Value> =
        serde_json::from_str(&root_file(".github/vv-rows.json")).expect("the legs");
    let parts: Vec<String> = rows
        .iter()
        .map(|row| row["part"].as_str().expect("a part").to_owned())
        .collect();
    let hosts = supported_host_contexts();
    // The jobs of a run of the whole gate that passed: the plan, every leg,
    // the Atlas join, and every host. The shards do not matter to the plan:
    // the join needs them all.
    let mut green: Vec<String> = vec!["plan\tsuccess".to_owned()];
    green.extend(parts.iter().map(|part| {
        if part == "honesty" {
            "honesty\tsuccess".to_owned()
        } else {
            format!("just vv ({part})\tsuccess")
        }
    }));
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
    // A merge whose second parent is the equal-tree merge above: the release
    // merge of a cut carried to `main`, whose tree is the regenerated head's.
    let regenerated = git(&["rev-parse", "head-contains-main"]);
    git(&["checkout", "-q", "-b", "carried", &base]);
    git(&["merge", "-q", "--no-ff", "-m", "carry", &merge_equal]);
    let carried = git(&["rev-parse", "HEAD"]);
    // Carried again, to four and five equal-tree merges above that head.
    let mut deeper = carried.clone();
    let mut chain = Vec::new();
    for hop in 3..=5 {
        git(&["checkout", "-q", "-b", &format!("carried-{hop}"), &base]);
        git(&["merge", "-q", "--no-ff", "-m", "carry", &deeper]);
        deeper = git(&["rev-parse", "HEAD"]);
        chain.push(deeper.clone());
    }
    let (four_down, five_down) = (chain[1].clone(), chain[2].clone());
    git(&["checkout", "-q", &carried]);
    assert_eq!(
        git(&["rev-parse", "HEAD^{tree}"]),
        git(&["rev-parse", &format!("{regenerated}^{{tree}}")]),
        "the planted carry lands the regenerated head's tree"
    );

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
           */workflows/ci.yml/runs\\?*)\n\
             case \"${STUB_RUNS:-}\" in\n\
               *@*) [[ \"$2\" != *\"head_sha=${STUB_RUNS#*@}&\"* ]] || printf '%s\\n' \"${STUB_RUNS%@*}\" ;;\n\
               *) printf '%s\\n' \"${STUB_RUNS:-}\" ;;\n\
             esac ;;\n\
           */runs/*/jobs\\?*) printf '%s\\n' \"${STUB_JOBS:-}\" ;;\n\
           */pulls\\?*) printf '%s\\n' \"${STUB_PULLS:-}\" ;;\n\
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
    // (full, rows' parts) for one planted event; a dispatch names its branch
    // and the open pull requests `gh` lists for it, as `draft<TAB>base` lines.
    let decide_with = |event: &str,
                       draft: &str,
                       base: &str,
                       commit: &str,
                       runs: &str,
                       jobs: &[String],
                       fail: bool,
                       branch: &str,
                       pulls: &str| {
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
            .env("BASE", base)
            .env("REPO", "owner/repo")
            .env("LEXLEAN_CI_ROWS", rows_file.as_std_path())
            .env("GITHUB_OUTPUT", &out)
            .env("GITHUB_STEP_SUMMARY", dir.join("summary"))
            .env("STUB_RUNS", runs)
            .env("STUB_JOBS", jobs.join("\n"))
            .env("REF", branch)
            .env("STUB_PULLS", pulls);
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
        let honesty: Vec<serde_json::Value> =
            serde_json::from_str(&value("honesty")).expect("honesty is JSON");
        let fanout: Vec<serde_json::Value> =
            serde_json::from_str(&value("fanout")).expect("fanout is JSON");
        assert!(honesty.iter().all(|row| row["part"] == "honesty"));
        assert!(fanout.iter().all(|row| row["part"] != "honesty"));
        let mut combined = honesty;
        combined.extend(fanout);
        assert_eq!(
            combined, planned,
            "honesty and fanout partition the planned rows"
        );

        (
            value("full"),
            planned
                .iter()
                .map(|row| row["part"].as_str().expect("a part").to_owned())
                .collect::<Vec<_>>(),
        )
    };
    let decide =
        |event: &str,
         draft: &str,
         base: &str,
         commit: &str,
         runs: &str,
         jobs: &[String],
         fail: bool| { decide_with(event, draft, base, commit, runs, jobs, fail, "", "") };
    let whole = ("true".to_owned(), parts.clone());
    let draft = (
        "false".to_owned(),
        vec!["honesty".to_owned(), "rest".to_owned()],
    );
    let nothing = ("false".to_owned(), Vec::<String>::new());
    // A draft pull request runs honesty then `rest`, as does a ready one stacked on
    // a feature branch, which cannot land on a line; a ready one into `main`
    // or a release line, a request, and a push are the whole gate when no run
    // is known. A base the event does not name is not known to be stacked.
    for (what, draft_flag, base, expected) in [
        (
            "a draft into a feature branch",
            "true",
            "claude/issue-91-h6",
            &draft,
        ),
        (
            "a ready pull request into a feature branch",
            "false",
            "claude/issue-91-h6",
            &draft,
        ),
        (
            "a ready pull request into a release line",
            "false",
            "release/v0.5",
            &whole,
        ),
        ("a ready pull request into main", "false", "main", &whole),
        (
            "a ready pull request with no base named",
            "false",
            "",
            &whole,
        ),
        (
            "a ready pull request into a branch that only starts like main",
            "false",
            "main-2",
            &draft,
        ),
    ] {
        assert_eq!(
            &decide("pull_request", draft_flag, base, &direct, "", &[], false),
            expected,
            "§9.4: {what}"
        );
    }
    assert_eq!(
        decide("pull_request", "true", "main", &direct, "", &[], false),
        draft
    );
    assert_eq!(
        decide("pull_request", "false", "main", &direct, "", &[], false),
        whole
    );
    assert_eq!(
        decide("workflow_dispatch", "", "main", &direct, "", &[], false),
        whole
    );
    // A dispatch on the head branch of one open pull request of this
    // repository is planned as that pull request (`regenerate.yml` commits
    // to the head with the workflow token, which starts no pull request run);
    // no pull request, several, or an unreadable list is the whole gate.
    for (what, pulls, fail, expected) in [
        ("a draft's head", "true\trelease/v0.4", false, &draft),
        ("a stacked head", "false\tclaude/issue-91-h6", false, &draft),
        (
            "a ready head into a line",
            "false\trelease/v0.4",
            false,
            &whole,
        ),
        ("a branch with no pull request", "", false, &whole),
        (
            "a branch that heads two pull requests",
            "true\trelease/v0.4\ntrue\tmain",
            false,
            &whole,
        ),
        ("an unreadable list", "true\trelease/v0.4", true, &whole),
    ] {
        assert_eq!(
            &decide_with(
                "workflow_dispatch",
                "",
                "",
                &direct,
                "",
                &[],
                fail,
                "claude/issue-91-h7",
                pulls
            ),
            expected,
            "§9.4: a dispatch on {what}"
        );
    }
    assert_eq!(
        decide_with(
            "workflow_dispatch",
            "",
            "",
            &direct,
            "",
            &[],
            false,
            "",
            "true\tmain"
        ),
        whole,
        "§9.4: a dispatch that names no branch is the whole gate"
    );
    // The tree a full run verified is not verified again when it is carried
    // down equal-tree merges: a push of the carry, and a dispatch on a ready
    // head that is the carry, find the run of the regenerated head two merges
    // down; a run of another commit, or a draft, does not count.
    let on_regenerated = format!("4242@{regenerated}");
    assert_eq!(
        decide("push", "", "main", &carried, &on_regenerated, &green, false),
        nothing,
        "§9.4: a push carrying a verified tree down two equal-tree merges"
    );
    assert_eq!(
        decide(
            "push",
            "",
            "main",
            &carried,
            &format!("4242@{direct}"),
            &green,
            false
        ),
        whole,
        "§9.4: a run of a commit the carry does not reach"
    );
    // The walk is bounded as `release-gate.sh` bounds it: four merges below
    // the pushed commit, and no fifth.
    assert_eq!(
        decide(
            "push",
            "",
            "main",
            &four_down,
            &on_regenerated,
            &green,
            false
        ),
        nothing,
        "§9.4: a verified tree four equal-tree merges below the push"
    );
    assert_eq!(
        decide(
            "push",
            "",
            "main",
            &five_down,
            &on_regenerated,
            &green,
            false
        ),
        whole,
        "§9.4: a verified tree five merges below the push is beyond the release evidence's walk"
    );
    assert_eq!(
        decide_with(
            "workflow_dispatch",
            "",
            "",
            &carried,
            &on_regenerated,
            &green,
            false,
            "release/v0.4",
            "false\tmain"
        ),
        nothing,
        "§9.4: a dispatch on a ready head whose tree a full run verified"
    );
    assert_eq!(
        decide_with(
            "workflow_dispatch",
            "",
            "",
            &carried,
            &format!("4242@{direct}"),
            &green,
            false,
            "release/v0.4",
            "false\tmain"
        ),
        whole,
        "§9.4: a dispatch on a ready head no full run verified"
    );
    assert_eq!(
        decide_with(
            "workflow_dispatch",
            "",
            "",
            &carried,
            &on_regenerated,
            &green,
            false,
            "release/v0.4",
            "true\tmain"
        ),
        draft,
        "§9.4: a draft head is planned as a draft whatever verified its tree"
    );
    assert_eq!(decide("push", "", "main", &direct, "", &[], false), whole);
    // A merge of its head's tree whose whole gate passed runs nothing.
    assert_eq!(
        decide("push", "", "main", &merge_equal, "4242", &green, false),
        nothing
    );
    // A complete jobs listing can exceed a pipe buffer. Matching the join
    // and required rows must consume it without a producer SIGPIPE.
    let mut large = green.clone();
    large.extend((0..2500).map(|index| format!("additional successful job {index}\tsuccess")));
    assert_eq!(
        decide("push", "", "main", &merge_equal, "4242", &large, false),
        nothing,
        "a large successful parent gate still avoids repeating the gate"
    );
    large.push("a late failed parent job\tfailure".to_owned());
    assert_eq!(
        decide("push", "", "main", &merge_equal, "4242", &large, false),
        whole,
        "a late failure never establishes successful parent evidence"
    );
    // Every way the evidence can fall short runs the whole gate: no run, a
    // run that skipped or failed a job, a run without the Atlas join, one
    // host or one leg missing, the gate unreadable, and a merge that changed
    // the tree, however good the run of its head.
    assert_eq!(
        decide("push", "", "main", &merge_equal, "", &green, false),
        whole
    );
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
        ("no honesty", without("honesty")),
        (
            "suffixed honesty",
            green
                .iter()
                .map(|job| {
                    job.replace(
                        "honesty\tsuccess",
                        "honesty (honesty, fmt-check model spec-links bdd, honesty)\tsuccess",
                    )
                })
                .collect(),
        ),
        (
            "failed honesty",
            green
                .iter()
                .map(|job| job.replace("honesty\tsuccess", "honesty\tfailure"))
                .collect(),
        ),
        (
            "skipped honesty",
            green
                .iter()
                .map(|job| job.replace("honesty\tsuccess", "honesty\tskipped"))
                .collect(),
        ),
        (
            "a draft run",
            vec![
                "plan\tsuccess".to_owned(),
                "just vv (rest)\tsuccess".to_owned(),
            ],
        ),
    ] {
        assert_eq!(
            decide("push", "", "main", &merge_equal, "4242", &jobs, false),
            whole,
            "§9.4: {what} is not the whole gate passing"
        );
    }
    for host in &hosts {
        let absent = without(host);
        let mut duplicate = green.clone();
        duplicate.push(format!("{host}\tsuccess"));
        let foreign: Vec<String> = green
            .iter()
            .map(|job| job.replace(host, "build and non-Lean tests (foreign-target)"))
            .collect();
        let suffixed: Vec<String> = green
            .iter()
            .map(|job| job.replace(host, &format!("{host} (partition)")))
            .collect();
        for (defect, jobs) in [
            ("absent", absent),
            ("duplicate", duplicate),
            ("foreign replacement", foreign),
            ("suffixed", suffixed),
        ] {
            assert_eq!(
                decide("push", "", "main", &merge_equal, "4242", &jobs, false),
                whole,
                "§9.4: {defect} host context `{host}` is not complete evidence"
            );
        }
    }
    assert_eq!(
        decide("push", "", "main", &merge_equal, "4242", &green, true),
        whole
    );
    assert_eq!(
        decide("push", "", "main", &merge_different, "4242", &green, false),
        whole
    );
    assert_eq!(
        decide("push", "", "main", &direct, "4242", &green, false),
        whole
    );
    // A run of another repository's pull request is not one of this one's.
    let script_text = root_file(".github/ci-plan.sh");
    assert!(
        script_text.contains(".head_repository.full_name == \\\"$repo\\\"")
            && script_text.contains("runs?head_sha=$commit&status=success")
            && script_text.contains(
                "select(.event == \\\"pull_request\\\" or .event == \\\"workflow_dispatch\\\")"
            )
            && script_text.contains("select(.head.repo.full_name == \\\"$repo\\\")"),
        "§9.4: only a passed pull request or dispatched run of this repository counts, and only this repository's pull request plans a dispatch"
    );
}

pub(crate) fn run(id: &str) {
    let root = support::repo_root();
    match id {
        // §2: exact identity — name, description, metadata, licenses.
        "RP-01" => {
            let workspace = root_file("Cargo.toml");
            for required in [
                "version = \"0.4.0\"",
                "edition = \"2021\"",
                "rust-version = \"1.97\"",
                "license = \"MIT OR Apache-2.0\"",
                "repository = \"https://github.com/UOR-Foundation/LexLean\"",
                "homepage = \"https://github.com/UOR-Foundation/LexLean\"",
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
                stdout.starts_with("lexlean 0.4.0\n"),
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
                (index == 1
                    && matches!(
                        parts[0],
                        "bootstrap" | "compiler" | "std-models" | "std-reasoning" | "stdlib"
                    ))
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
            assert_eq!(table.len(), 503, "§31 has 503 rows");
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
            // §8.4: a build without production-data reports this constant in
            // place of the ID it cannot compute. Nothing else would notice a
            // stale one: that build locks and checks with it consistently.
            assert_eq!(
                lexlean::LANGUAGE_1_2_SEMANTICS_ID,
                twelve.to_hex(),
                "RP-10: lexlean::LANGUAGE_1_2_SEMANTICS_ID is stale; the tree's language-1.2 ID is {}",
                twelve.to_hex()
            );
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
                .expect_err("a 0.4.0 tree must refuse release (§2.3, §30.4)");
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
            let target_plan = crate::bdd_parts::Plan::parse(
                r#"{"remainder":"rest","parts":{},"targets":{"honesty":["bdd"]}}"#,
            )
            .expect("complete-target ownership");
            let target_parts = |honesty: crate::inventory::Parsed,
                                rest: crate::inventory::Parsed| {
                vec![("honesty".to_owned(), honesty), ("rest".to_owned(), rest)]
            };
            crate::inventory::compare_parts(
                &owned,
                &target_parts(list(&[bdd_target]), list(&[conformance])),
                &target_plan,
            )
            .expect("the complete target moves to honesty");
            assert!(
                crate::inventory::compare_parts(
                    &owned,
                    &target_parts(list(&[bdd_target]), owned.clone()),
                    &target_plan
                )
                .is_err(),
                "the remainder cannot repeat honesty tests"
            );
            assert!(
                crate::inventory::compare_parts(
                    &owned,
                    &target_parts(Default::default(), owned.clone()),
                    &target_plan
                )
                .is_err(),
                "moving tests back to rest cannot silently empty honesty"
            );
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
                    && task.contains("inventory::compare_parts(")
                    && task.contains("plan.cargo_targets(part,")
                    && task.contains("plan.cargo_targets(&name,"),
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
            // The initial honesty job keeps its own signal. Ordinary hosts run
            // the whole workspace; Intel's construction and partitions own the
            // same complete inventory, including library/meta tests and doctests.
            assert!(root_file(".github/workflows/ci.yml").contains("  honesty:\n"));
            assert!(!support::repo_root()
                .join(".github/workflows/honesty.yml")
                .exists());
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
            // Normative gate jobs and the Intel receipt join run on the
            // standard runner. Only host execution runs elsewhere (§8.3).
            for (key, body) in &jobs {
                if matches!(
                    key.as_str(),
                    "host-matrix" | "intel-construction" | "intel-part" | "windows-part"
                ) {
                    continue;
                }
                assert!(
                    body.contains("\n    runs-on: ubuntu-24.04\n"),
                    "§9.4: the job `{key}` runs on the standard runner"
                );
                // The plan decides what the run is and runs no recipe: it has
                // ten minutes, not the six hours of the proof jobs.
                if key == "intel-join" || key == "windows-join" {
                    let limit = body
                        .lines()
                        .find_map(|line| line.trim().strip_prefix("timeout-minutes: "))
                        .and_then(|value| value.parse::<u32>().ok())
                        .expect("§8.3: the Intel aggregate has a bounded timeout");
                    assert!(
                        (1..=240).contains(&limit),
                        "the aggregate raises no host timeout"
                    );
                    continue;
                }
                let limit = match key.as_str() {
                    "plan" => 10,
                    "honesty" => 30,
                    _ => 360,
                };
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
            let initial: Vec<_> = rows
                .iter()
                .filter(|row| field(row, "part").as_deref() == Some("honesty"))
                .collect();
            assert_eq!(initial.len(), 1, "one initial honesty row");
            assert_eq!(
                field(initial[0], "recipes").as_deref(),
                Some("fmt-check model spec-links bdd")
            );
            assert_eq!(field(initial[0], "bdd_part").as_deref(), Some("honesty"));
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
                2,
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
            // §9.4: a release line is held to the same gate as `main`, so the
            // push trigger of every workflow names both; a workflow that named
            // `main` alone would leave a release line's pushes unverified.
            for workflow in ["ci.yml", "reproducibility.yml"] {
                let text = root_file(&format!(".github/workflows/{workflow}"));
                assert!(
                    text.contains("  push: { branches: [main, \"release/**\"] }\n"),
                    "§9.4: {workflow} runs on a push to `main` and to a release line"
                );
            }
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
            let mut honesty_conditions = 0;
            for (index, line) in jobs
                .iter()
                .filter(|(key, _)| !key.starts_with("intel-") && !key.starts_with("windows-"))
                .flat_map(|(_, body)| body.lines())
                .enumerate()
            {
                if line.trim_start().starts_with("if:") {
                    match line {
                        "        if: ${{ !cancelled() }}" => step_conditions += 1,
                        "    if: ${{ needs.plan.outputs.full == 'true' }}" => full_conditions += 1,
                        "    if: ${{ needs.plan.outputs.fanout != '[]' }}" => rows_conditions += 1,
                        "    if: ${{ needs.plan.outputs.honesty != '[]' }}" => honesty_conditions += 1,
                        other => panic!(
                            "ci.yml:{}: a condition other than a step's `!cancelled()` or a job's condition on the plan: {other}",
                            index + 1
                        ),
                    }
                }
            }
            assert_eq!(
                step_conditions,
                6 + 2 * atlas_jobs.len(),
                "the summary step and the evidence upload of every gate job carry it"
            );
            assert_eq!(
                full_conditions,
                atlas_jobs.len() + 2,
                "every Atlas job, the bootstrap export, and the host matrix, and no other job, wait for the plan to name the whole gate"
            );
            assert_eq!(rows_conditions, 1, "the legs run when the plan names a leg");
            assert_eq!(honesty_conditions, 1, "honesty runs when the plan names it");
            intel_host_workflow(&ci).expect("§8.3: the host inventory and Intel custody graph");
            windows_host_workflow(&ci).expect("§8.3: the Windows lanes and their join");
            for (from, to) in [
                ("lane: [producer, lower, print, validate, moves]", "lane: [producer, lower, print, validate]"),
                ("    needs: [plan, windows-part]", "    needs: [plan]"),
                (
                    "    if: ${{ always() && needs.plan.outputs.full == 'true' }}\n    runs-on: ubuntu-24.04\n    timeout-minutes: 15\n    steps:\n      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1\n      - uses: actions/download-artifact@9000827ccba6bdab643e8b6fd33ac0654aef8333 # v8.0.2\n        with:\n          pattern: host-x86_64",
                    "    if: ${{ needs.plan.outputs.full == 'true' }}\n    runs-on: ubuntu-24.04\n    timeout-minutes: 15\n    steps:\n      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1\n      - uses: actions/download-artifact@9000827ccba6bdab643e8b6fd33ac0654aef8333 # v8.0.2\n        with:\n          pattern: host-x86_64",
                ),
                ("host_lanes.py join --target", "host_lanes.py execute --target"),
                (
                    "windows-part.result }}\n        run: |\n          set -euo pipefail\n          test \"$PART_RESULT\" = success",
                    "windows-part.result }}\n        run: |\n          set -euo pipefail\n          test \"$PART_RESULT\" != cancelled",
                ),
                ("    runs-on: windows-2022\n", "    runs-on: windows-2025\n"),
                (
                    "python3 -m unittest discover -s .github/scripts -p 'test_host_lanes.py'",
                    "echo lane checks",
                ),
            ] {
                assert!(ci.contains(from), "the plant must change the workflow: {from}");
                assert!(
                    windows_host_workflow(&ci.replacen(from, to, 1)).is_err(),
                    "the Windows lanes must refuse `{from}` replaced by `{to}`"
                );
            }
            for (from, to) in [
                (
                    "name: build and non-Lean tests (x86_64-apple-darwin)",
                    "name: Intel aggregate",
                ),
                ("needs: [plan, intel-construction]", "needs: [plan]"),
                (
                    "needs: [plan, intel-construction, intel-part]",
                    "needs: [plan, intel-construction]",
                ),
                (
                    "if: ${{ always() && needs.plan.outputs.full == 'true' }}",
                    "if: ${{ needs.plan.outputs.full == 'true' }}",
                ),
                ("runs-on: macos-15-intel", "runs-on: macos-15"),
                (
                    "lane: ${{ fromJSON(needs.intel-construction.outputs.lanes) }}",
                    "lane: [lower]",
                ),
                (
                    "intel_host_tests.py execute --construction",
                    "intel_host_tests.py build --output",
                ),
                (
                    "test \"$PART_RESULT\" = success",
                    "test \"$PART_RESULT\" != cancelled",
                ),
                (
                    "python3 -m unittest discover -s .github/scripts -p 'test_intel_host_tests.py'",
                    "echo factory checks",
                ),
            ] {
                assert!(
                    ci.contains(from),
                    "the plant must change the workflow: {from}"
                );
                assert!(
                    intel_host_workflow(&ci.replacen(from, to, 1)).is_err(),
                    "the Intel graph must refuse `{from}` replaced by `{to}`"
                );
            }
            let take = root_file(TAKE_EXPORT);
            bootstrap_export_workflow(&ci, &take)
                .expect("§9.4: the run's one bootstrap export and its same-run custody");
            let export_job = workflow_job(&ci, "bootstrap-export").expect("the export job");
            for (from, to) in [
                ("    needs: [plan, honesty]", "    needs: [plan]"),
                (
                    "cargo xtask export-bootstrap --output \"$RUNNER_TEMP/bootstrap-export\"",
                    "echo exported",
                ),
                (
                    "      LEXLEAN_VERIFY_WIDTH: \"2\"",
                    "      LEXLEAN_VERIFY_WIDTH: \"4\"",
                ),
                (
                    "          name: bootstrap-export\n",
                    "          name: bootstrap-bundle\n",
                ),
                ("    name: bootstrap export\n", "    name: export\n"),
            ] {
                assert!(
                    export_job.contains(from),
                    "the plant must change the export job: {from}"
                );
                let planted = ci.replacen(&export_job, &export_job.replacen(from, to, 1), 1);
                assert!(
                    bootstrap_export_workflow(&planted, &take).is_err(),
                    "the bootstrap export's custody must refuse `{from}` replaced by `{to}`"
                );
            }
            for (from, to) in [
                (
                    "          FULL_GATE: ${{ needs.plan.outputs.full }}\n",
                    "          FULL_GATE: true\n",
                ),
                (
                    "    permissions:\n      contents: read\n      actions: read\n    # Complete proof",
                    "    permissions:\n      contents: read\n    # Complete proof",
                ),
                (
                    "            export LEXLEAN_BOOTSTRAP_EXPORT=\"$RUNNER_TEMP/bootstrap-export\"\n",
                    "            export LEXLEAN_BOOTSTRAP_EXPORT=\"$RUNNER_TEMP/elsewhere\"\n",
                ),
            ] {
                assert!(ci.contains(from), "the plant must change the workflow: {from}");
                assert!(
                    bootstrap_export_workflow(&ci.replacen(from, to, 1), &take).is_err(),
                    "the bootstrap export's custody must refuse `{from}` replaced by `{to}`"
                );
            }
            for (from, to) in [
                ("run=${GITHUB_RUN_ID:?}", "run=${OTHER_RUN_ID:?}"),
                (
                    "artifacts?name=bootstrap-export",
                    "artifacts?name=bootstrap-bundle",
                ),
                (
                    "select(.name == \"bootstrap export\")",
                    "select(.name == \"export\")",
                ),
                ("until [ -e \"$dir/wanted\" ]", "until true"),
            ] {
                assert!(
                    take.contains(from),
                    "the plant must change the fetcher: {from}"
                );
                assert!(
                    bootstrap_export_workflow(&ci, &take.replacen(from, to, 1)).is_err(),
                    "the fetcher's custody must refuse `{from}` replaced by `{to}`"
                );
            }
            bootstrap_custody_decisions();
            honesty_dependencies(&ci).expect("expensive roots require successful same-run honesty");
            assert!(
                honesty_dependencies(&ci.replacen("name: ${{ matrix.part }}", "name: honesty", 1))
                    .is_err(),
                "a static matrix name acquires a suffix instead of the exact honesty context"
            );
            for root in [
                "bootstrap-export",
                "vv",
                "atlas-prefix",
                "host-matrix",
                "intel-construction",
                "windows-part",
            ] {
                let original = format!("  {root}:\n");
                let pos = ci.find(&original).expect("the root");
                let end = ci[pos + original.len()..]
                    .find("\n  #")
                    .map_or(ci.len(), |next| pos + original.len() + next);
                let mut planted = ci.clone();
                let body = ci[pos..end].replace("needs: [plan, honesty]", "needs: [plan]");
                planted.replace_range(pos..end, &body);
                assert!(
                    honesty_dependencies(&planted).is_err(),
                    "{root} cannot bypass honesty"
                );
            }
            assert!(
                job("host-matrix").contains("    needs: [plan, honesty]\n")
                    && job("host-matrix")
                        .contains("    if: ${{ needs.plan.outputs.full == 'true' }}\n"),
                "§8.3: the hosts are checked when the plan says the run is the whole gate"
            );
            assert!(
                vv.contains("    needs: [plan, honesty]\n")
                    && vv.contains("        include: ${{ fromJSON(needs.plan.outputs.fanout) }}\n"),
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
                    && plan_job.contains("          BASE: ${{ github.event.pull_request.base.ref }}\n")
                    && plan_job.contains("          EVENT: ${{ github.event_name }}\n")
                    && plan_job.contains("          REF: ${{ github.ref_name }}\n"),
                "§9.4: the plan runs the script on the event and the base branch, and may read the runs of the repository"
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
            let honesty = job("honesty");
            // §9.4: stale generated files fail in honesty, before the gate
            // command and every expensive root.
            let fresh = honesty
                .find("        run: cargo xtask check-freshness\n")
                .expect("§9.4: honesty checks that the committed generated files are current");
            assert!(
                fresh < honesty.find("just --timestamp").expect("the gate command"),
                "§9.4: the freshness check runs before the honesty recipes"
            );
            assert!(
                honesty.contains("LEXLEAN_BDD_PART: ${{ matrix.bdd_part }}")
                    && honesty.contains("fromJSON(needs.plan.outputs.honesty)")
                    && honesty.contains("just --timestamp")
                    && !honesty.contains("elan"),
                "the initial part needs no Lean"
            );
            assert!(root_file(".github/workflows/reproducibility.yml").contains("run: just repro"));
        }
        // R8: a `match` that names several variants of a closed enum names
        // them all, in every consumer of the IR, the calculus, and its Rust AST.
        "RP-15" => rp_15(),
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
        // §30.3: the crate that goes to crates.io is the crate the release
        // attaches, proved on every run, and uploaded only with a credential.
        "RP-18" => rp_18(),
        // §8.4: a build without the production data checks and locks as the
        // default build does, and refuses the work that needs that data.
        "RP-19" => rp_19(),
        other => panic!("no repository case is wired for {other}"),
    }
}

/// RP-19: the binary built without `production-data`, held to the default
/// build this harness links. The language-1.2 ID it reports is a constant,
/// which RP-10 holds to the tree; here it must also land in the lock bytes,
/// which is what a wrong constant would change while every verdict agreed.
/// A file of the normative tree: its repository-relative path and its bytes,
/// as `support::normative_rows` reads them.
type NormativeRow = (String, Vec<u8>);

/// How many bytes of a file witness that an image holds it: the whole file
/// when it is short, else a window from its middle, past the header that
/// every file of a kind repeats (RP-19).
const WITNESS_BYTES: usize = 64;

/// The witness of one file: its own bytes, or a window of them.
fn witness_of(bytes: &[u8]) -> Vec<u8> {
    if bytes.len() <= WITNESS_BYTES {
        bytes.to_vec()
    } else {
        bytes[bytes.len() / 2 - WITNESS_BYTES / 2..][..WITNESS_BYTES].to_vec()
    }
}

/// Which of `witnesses`, each a path and the bytes that witness it, occur in
/// `image`, in one pass over it: a table of first bytes names the candidates
/// at each position and only a candidate is compared, so the cost is the
/// image's length and not its length times the inventory's size. RP-19 reads
/// a binary of twenty-three megabytes against an inventory of thirty-one.
fn witnessed(image: &[u8], witnesses: &[NormativeRow]) -> BTreeSet<String> {
    let mut by_first: Vec<Vec<usize>> = vec![Vec::new(); 256];
    for (index, (path, bytes)) in witnesses.iter().enumerate() {
        assert!(!bytes.is_empty(), "{path}: a witness is not empty");
        by_first[bytes[0] as usize].push(index);
    }
    let mut found = BTreeSet::new();
    for (at, byte) in image.iter().enumerate() {
        for index in &by_first[*byte as usize] {
            let (path, witness) = &witnesses[*index];
            if !found.contains(path) && image[at..].starts_with(witness) {
                found.insert(path.clone());
            }
        }
    }
    found
}

fn rp_19() {
    use lexlean::api::production::{
        self, Expectation, PackageName, ProductionRequest, RootRequest, TargetRef, REQUEST_SPEC,
    };
    let root = support::repo_root();
    let binary =
        support::check_only_binary(&root).expect("the crate builds without its default features");

    // The whole excluded inventory is bound to the built variant, not its
    // largest file alone: a build that left the largest out and embedded any
    // other would pass that and still break what RP-19 states. Every file of
    // the inventory is witnessed by bytes of its own, and the image is read
    // once for all of them.
    let image = std::fs::read(binary.as_std_path()).expect("the binary is readable");
    let rows = support::normative_rows();
    let (library, retained_rows): (Vec<&NormativeRow>, Vec<&NormativeRow>) = rows
        .iter()
        .partition(|(path, _)| path.starts_with("language/preservation-1.2/library/"));
    assert!(
        library.len() > 10,
        "RP-19: the excluded inventory is the library's files on disk"
    );
    let witnesses: Vec<NormativeRow> = library
        .iter()
        .map(|(path, bytes)| (path.clone(), witness_of(bytes)))
        .collect();
    // A file whose witness the retained data carries anyway cannot be told
    // from that data in an image. Such a file is named here with its reason
    // rather than passing in silence, and the reason is checked: its own
    // bytes are embedded outside the library, so an image holding them holds
    // no library source.
    let retained: Vec<u8> = retained_rows
        .iter()
        .flat_map(|(_, bytes)| bytes.iter().copied().chain(std::iter::once(0)))
        .collect();
    let indistinguishable = witnessed(&retained, &witnesses);
    for path in &indistinguishable {
        let (_, bytes) = library
            .iter()
            .find(|(candidate, _)| candidate == path)
            .expect("a file of the inventory");
        let whole = vec![(path.clone(), bytes.clone())];
        assert!(
            !witnessed(&retained, &whole).is_empty(),
            "RP-19: `{path}` cannot be told from the retained data in an image, and its own bytes are not part of that data; the inventory is not bound"
        );
    }
    let found = witnessed(&image, &witnesses);
    let embedded: Vec<&String> = found.difference(&indistinguishable).collect();
    assert!(
        embedded.is_empty(),
        "RP-19: the build without production-data embeds {embedded:?} of `language/preservation-1.2/library/`"
    );
    // The scan finds what is there: the preservation data outside `library/`,
    // which this build does embed, is found in the same image by the same
    // method, so an absence above is an absence and not a broken search.
    let siblings: Vec<NormativeRow> = retained_rows
        .iter()
        .filter(|(path, _)| path.starts_with("language/preservation-1.2/"))
        .map(|(path, bytes)| (path.clone(), witness_of(bytes)))
        .collect();
    assert!(
        siblings.len() > 1,
        "RP-19: the retained preservation data is more than one file"
    );
    assert_eq!(
        witnessed(&image, &siblings).len(),
        siblings.len(),
        "RP-19: the embedded preservation data outside `library/` is in the image"
    );

    let normalized =
        |text: &str, project: &camino::Utf8Path| text.replace(project.as_str(), "<project>");
    let same = |what: &str, linked: &support::P, built: &support::P, arguments: &[&str]| {
        let (exit_a, out_a, err_a) = linked.cli(arguments);
        let (exit_b, out_b, err_b) = support::cli_binary_in(&binary, &built.root, arguments);
        assert_eq!(
            exit_a,
            exit_b,
            "RP-19: {what}: `{}` exit code",
            arguments.join(" ")
        );
        assert_eq!(
            normalized(&out_a, &linked.root),
            normalized(&out_b, &built.root),
            "RP-19: {what}: `{}` stdout",
            arguments.join(" ")
        );
        assert_eq!(
            normalized(&err_a, &linked.root),
            normalized(&err_b, &built.root),
            "RP-19: {what}: `{}` stderr",
            arguments.join(" ")
        );
    };

    let (_, version, _) = support::cli_in(&root, &["--version"]);
    let (exit, built_version, _) = support::cli_binary_in(&binary, &root, &["--version"]);
    assert_eq!(exit, 0, "RP-19: --version");
    // The same four lines, and a fifth that keeps it from passing for a
    // release binary, which it cannot replace (§30.3).
    assert_eq!(
        format!(
            "{version}variant check-only
"
        ),
        built_version,
        "RP-19: --version"
    );

    // A lock records its language's compiler-semantics ID, so equal lock
    // bytes for a 1.0, 1.1, and 1.2 project are equal IDs.
    for example in ["nat-add-zero", "semantic-1.1", "language-1.2", "production"] {
        let linked = support::P::copy_example(example);
        let built = support::P::copy_example(example);
        for project in [&linked, &built] {
            std::fs::remove_file(project.root.join("lexlean.lock").as_std_path())
                .expect("the example has a lock");
        }
        same(example, &linked, &built, &["lock"]);
        let lock = |project: &support::P| {
            std::fs::read_to_string(project.root.join("lexlean.lock").as_std_path())
                .expect("a lock was written")
        };
        assert_eq!(
            lock(&linked),
            lock(&built),
            "RP-19: {example}: lexlean.lock bytes"
        );
        same(
            example,
            &linked,
            &built,
            &["--diagnostic-format", "json", "check"],
        );
    }
    for negative in [
        "type-parameter-capture",
        "term-cases-scrutinee-not-inductive",
        "induction-nested-type",
    ] {
        let linked = support::P::negative(negative);
        let built = support::P::negative(negative);
        same(
            negative,
            &linked,
            &built,
            &["--diagnostic-format", "json", "check"],
        );
    }

    // The two commands that generate certificates are refused with LLC0205
    // before any work is published: no build, verification, or export.
    let refused = |project: &support::P, arguments: &[&str]| {
        let (exit, out, err) = support::cli_binary_in(&binary, &project.root, arguments);
        assert_eq!(
            exit,
            2,
            "RP-19: `{}` exits 2: {out}{err}",
            arguments.join(" ")
        );
        assert!(
            out.contains("LLC0205") || err.contains("LLC0205"),
            "RP-19: `{}` reports LLC0205: {out}{err}",
            arguments.join(" ")
        );
        for area in ["build", "verified", "production"] {
            assert!(
                !project
                    .root
                    .join(".lexlean")
                    .join(area)
                    .as_std_path()
                    .exists(),
                "RP-19: `{}` published `.lexlean/{area}`",
                arguments.join(" ")
            );
        }
    };
    let project = support::P::copy_example("production");
    refused(&project, &["--diagnostic-format", "json", "verify"]);
    let digests: std::collections::BTreeMap<String, lexlean::Sha256Digest> = production::targets()
        .expect("targets")
        .into_iter()
        .map(|descriptor| (descriptor.id, descriptor.digest))
        .collect();
    let request = ProductionRequest {
        spec: REQUEST_SPEC.to_owned(),
        roots: vec![RootRequest {
            root: "Production.Main.checkedSum".to_owned(),
            package: PackageName {
                name: "pkg".to_owned(),
                version: "1.0.0".to_owned(),
            },
            targets: vec![TargetRef {
                id: "rust-std".to_owned(),
                digest: digests["rust-std"],
            }],
        }],
        expect: Expectation::default(),
        evidence: Vec::new(),
    };
    let project = support::P::copy_example("production");
    std::fs::write(
        project.root.join("request.json").as_std_path(),
        serde_json::to_vec(&request).expect("the request serializes"),
    )
    .expect("write the request");
    refused(
        &project,
        &[
            "--diagnostic-format",
            "json",
            "production",
            "build",
            "--request",
            "request.json",
        ],
    );
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

/// RP-15: the committed consumers are total, each of them is read, and a
/// catch-all planted in each is found.
fn rp_15() {
    use repo_model::classification as audit;
    let root = support::repo_root();
    let base = root.join(audit::SOURCE_ROOT);
    let mut sources: Vec<(String, String)> = walkdir::WalkDir::new(base.as_std_path())
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "rs"))
        .map(|entry| {
            let relative = entry
                .path()
                .strip_prefix(root.as_std_path())
                .expect("under the root")
                .to_string_lossy()
                .replace('\\', "/");
            (
                relative,
                std::fs::read_to_string(entry.path()).expect("a source"),
            )
        })
        .collect();
    sources.sort();
    let summary = audit::audit(&sources).expect("the committed sources are total");
    assert!(summary.files > 100, "the audit reads the whole crate");
    for (path, role) in audit::CONSUMERS {
        assert!(
            summary.per_file.get(path).is_some_and(|count| *count > 0),
            "{path} ({role}) holds a classification"
        );
    }
    // A catch-all planted in a classification of each consumer group.
    let group = |path: &str| {
        let mut enums: Vec<&str> = repo_model::exhaustive::AUDITED_ENUMS.to_vec();
        if path.starts_with(audit::CALCULUS_ROOT) {
            enums.extend(audit::CALCULUS_ENUMS);
        }
        enums
    };
    let plants = [
        ("crates/lexlean/src/backend/semantic.rs", "spelling"),
        ("crates/lexlean/src/backend/semantic.rs", "render_latex"),
        ("crates/lexlean/src/ir/semantic.rs", "infer_term"),
        ("crates/lexlean/src/ir/semantic.rs", "type_node_count"),
        ("crates/lexlean/src/production/eligibility.rs", "type_key"),
        ("crates/lexlean/src/calculus/check.rs", "prim_type"),
        ("crates/lexlean/src/calculus/realization.rs", "in_place"),
        ("crates/lexlean/src/calculus/rust/ast.rs", "expr"),
        ("crates/lexlean/src/calculus/rust/lower.rs", "item"),
        ("crates/lexlean/src/calculus/rust/validate.rs", "expr"),
    ];
    for (path, function) in plants {
        let at = sources
            .iter()
            .position(|(candidate, _)| candidate == path)
            .unwrap_or_else(|| panic!("{path} is read"));
        let planted = audit::plant_catch_all(&sources[at].1, &group(path), function)
            .unwrap_or_else(|| panic!("{path}: `{function}` holds a classification"));
        let mut copy = sources.clone();
        copy[at].1 = planted.clone();
        let error = audit::audit(&copy).expect_err("a planted wildcard is found");
        assert!(
            error.contains(path) && error.contains("a wildcard arm"),
            "{path}::{function}: {error}"
        );
        // The same arm bound to a name catches everything just the same.
        copy[at].1 = planted.replacen("_ => Default::default()", "other => other", 1);
        let error = audit::audit(&copy).expect_err("a planted binding is found");
        assert!(
            error.contains(path) && error.contains("a binding catch-all arm"),
            "{path}::{function}: {error}"
        );
    }
    // A consumer that no longer holds a classification, or is not read, would
    // be passed over without being checked.
    for (path, role) in audit::CONSUMERS {
        let at = sources
            .iter()
            .position(|(candidate, _)| candidate == path)
            .expect("read");
        let mut vacated = sources.clone();
        vacated[at].1 = "fn nothing() {}\n".to_owned();
        let error = audit::audit(&vacated).expect_err("a vacated consumer fails");
        assert!(
            error.contains(path) && error.contains(role) && error.contains("would pass over"),
            "{path}: {error}"
        );
        let mut absent = sources.clone();
        absent.remove(at);
        assert!(audit::audit(&absent)
            .expect_err("an unread consumer fails")
            .contains(path));
    }
}

/// The body of the job `name` of a workflow: the lines after its key at two
/// spaces of indentation, up to the next line at that indentation.
fn workflow_job(workflow: &str, name: &str) -> Result<String, String> {
    let header = format!("\n  {name}:\n");
    let start = workflow
        .find(&header)
        .ok_or_else(|| format!("no job `{name}`"))?
        + header.len();
    let body: Vec<&str> = workflow[start..]
        .lines()
        .take_while(|line| line.is_empty() || line.starts_with("   "))
        .collect();
    Ok(body.join("\n"))
}

/// Where `needle` first appears in `text`, or the refusal that names it.
fn position_of(text: &str, needle: &str, context: &str) -> Result<usize, String> {
    text.find(needle)
        .ok_or_else(|| format!("{context} has no `{needle}`"))
}

/// RP-18 (§30.3, #260): the workflows prove the crate on every run and gate its
/// upload. The crate job packages twice to the same bytes, dry-runs, checks the
/// archive against what crates.io needs, and compares the bytes with the crate
/// the release attaches and the checksum it lists; the tag's `publish` job
/// repeats the comparison against the dry-run package before it asks how the
/// upload is authorised, and the one step that runs `cargo publish` without
/// `--dry-run` there, and the one in the pre-release job, run only when it is.
fn crate_publication_workflows(release: &str, crate_workflow: &str) -> Result<(), String> {
    let in_order = |text: &str, context: &str, needles: &[&str]| -> Result<(), String> {
        let mut previous = 0;
        for needle in needles {
            let at = position_of(text, needle, context)?;
            if at < previous {
                return Err(format!(
                    "{context}: `{needle}` comes before the step it must follow"
                ));
            }
            previous = at;
        }
        Ok(())
    };
    let uploads = |text: &str| -> usize {
        text.lines()
            .filter(|line| line.contains("cargo publish ") && !line.contains("--dry-run"))
            .filter(|line| {
                !line.trim_start().starts_with('#') && !line.trim_start().starts_with("echo")
            })
            .count()
    };

    // release.yml: the gate job reads the gate run of the commit and runs no
    // recipe of the gate (§9.4, §30.3).
    let gate_job = workflow_job(release, "gate")?;
    for needle in [
        "needs: identity",
        "actions: read",
        ".github/release-gate.sh",
    ] {
        position_of(&gate_job, needle, "release.yml job `gate`")?;
    }
    if gate_job.lines().any(|line| {
        let line = line.trim_start();
        !line.starts_with('#') && (line.starts_with("just ") || line.contains(" just "))
    }) {
        return Err("release.yml job `gate` runs a recipe of the gate itself".to_owned());
    }

    // release.yml: every run calls the crate workflow, and the tag waits for it.
    let crate_job = workflow_job(release, "crate")?;
    for needle in [
        "needs: identity",
        "uses: ./.github/workflows/crate.yml",
        "prerelease: false",
    ] {
        position_of(&crate_job, needle, "release.yml job `crate`")?;
    }
    let publish = workflow_job(release, "publish")?;
    let needs = publish
        .lines()
        .find(|line| line.trim_start().starts_with("needs:"))
        .ok_or("release.yml job `publish` has no needs")?;
    if !needs.contains("crate,") {
        return Err("release.yml job `publish` does not wait for the `crate` job".to_owned());
    }
    in_order(
        &publish,
        "release.yml job `publish`",
        &[
            "cargo xtask release-artifacts",
            "cargo publish --dry-run --locked --package lexlean",
            ".github/crate-publication.sh identity release",
            ".github/crate-publication.sh mode",
            "if: steps.upload.outputs.mode == 'trusted'",
            "if: steps.upload.outputs.mode != 'none'",
            "cargo publish --locked --package lexlean",
        ],
    )?;
    position_of(
        &publish,
        "needs.crate.outputs.sha256",
        "release.yml job `publish`",
    )?;
    if uploads(&publish) != 1 {
        return Err(
            "release.yml job `publish` runs `cargo publish` without --dry-run other than once"
                .to_owned(),
        );
    }
    for guarded in [
        "- name: Publish the exact verified crate to crates.io\n        if: steps.upload.outputs.mode != 'none'\n",
        "- name: Acquire crates.io trusted-publisher token\n        if: steps.upload.outputs.mode == 'trusted'\n",
    ] {
        position_of(&publish, guarded, "release.yml job `publish`")?;
    }
    // The token reaches cargo in the one step that uploads.
    let token_uses = publish.matches("secrets.CARGO_REGISTRY_TOKEN").count();
    if token_uses != 2 {
        return Err(format!(
            "release.yml job `publish` reads the token {token_uses} times: once to learn it exists and once to upload"
        ));
    }

    // crate.yml: the proof, then the guarded upload of a pre-release.
    let readiness = workflow_job(crate_workflow, "readiness")?;
    in_order(
        &readiness,
        "crate.yml job `readiness`",
        &[
            ".github/crate-publication.sh prepare",
            "cargo package --locked \"${dirty[@]}\" --package lexlean --target-dir \"$a\"",
            "cargo package --locked --no-verify \"${dirty[@]}\" --package lexlean --target-dir \"$b\"",
            "cmp \"$a/package/$crate\" \"$b/package/$crate\"",
            "cargo publish --dry-run --locked \"${dirty[@]}\" --package lexlean --target-dir \"$dry\"",
            "cmp \"$a/package/$crate\" \"$dry_crate\"",
            ".github/crate-publication.sh metadata \"$dry_crate\"",
            "cargo xtask release-artifacts",
            ".github/crate-publication.sh identity release \"$dry_crate\"",
        ],
    )?;
    if uploads(&readiness) != 0 {
        return Err("crate.yml job `readiness` uploads".to_owned());
    }
    let prerelease = workflow_job(crate_workflow, "prerelease")?;
    position_of(
        &prerelease,
        "if: ${{ github.event_name == 'workflow_dispatch' && inputs.prerelease }}",
        "crate.yml job `prerelease`",
    )?;
    in_order(
        &prerelease,
        "crate.yml job `prerelease`",
        &[
            ".github/crate-publication.sh mode",
            "- name: Upload the exact checked crate\n        if: ${{ steps.upload.outputs.mode != 'none' }}\n",
            "test \"$(sha256sum \"${RUNNER_TEMP}/upload/package/lexlean-${VERSION}.crate\"",
            "cargo publish --locked --allow-dirty --package lexlean",
        ],
    )?;
    if uploads(&prerelease) != 1 {
        return Err(
            "crate.yml job `prerelease` runs `cargo publish` without --dry-run other than once"
                .to_owned(),
        );
    }
    position_of(
        &prerelease,
        "- name: Acquire crates.io trusted-publisher token\n        if: ${{ steps.upload.outputs.mode == 'trusted' }}\n",
        "crate.yml job `prerelease`",
    )?;
    Ok(())
}

/// What `.github/crate-publication.sh` decides for planted inputs (RP-18): the
/// script runs in scratch directories, so that each decision is shown and each
/// way of being wrong is refused. It is bash with `sha256sum`, `tar` and `sed`,
/// run where the gate runs (§8.3, §9.4: the normative host is Linux).
fn crate_publication_decisions() {
    use std::process::Command;
    if !cfg!(target_os = "linux") {
        return;
    }
    let root = support::repo_root();
    let script = root.join(".github/crate-publication.sh");
    let scratch = tempfile::tempdir().expect("tempdir");
    let dir = scratch.path();
    let run = |args: &[&str], environment: &[(&str, &str)]| -> (i32, String, String) {
        let mut command = Command::new("bash");
        command
            .arg(script.as_std_path())
            .args(args)
            .current_dir(dir)
            .env_remove("HAS_CARGO_REGISTRY_TOKEN")
            .env_remove("TRUSTED_PUBLISHING");
        for (key, value) in environment {
            command.env(key, value);
        }
        let output = command.output().expect("bash runs");
        (
            output.status.code().expect("an exit code"),
            String::from_utf8_lossy(&output.stdout).trim().to_owned(),
            String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        )
    };

    // How the upload is authorised: a token, trusted publishing, or neither.
    for (environment, expected) in [
        (vec![("HAS_CARGO_REGISTRY_TOKEN", "true")], "token"),
        (
            vec![
                ("HAS_CARGO_REGISTRY_TOKEN", "true"),
                ("TRUSTED_PUBLISHING", "true"),
            ],
            "token",
        ),
        (vec![("TRUSTED_PUBLISHING", "true")], "trusted"),
        (
            vec![
                ("HAS_CARGO_REGISTRY_TOKEN", "false"),
                ("TRUSTED_PUBLISHING", "false"),
            ],
            "none",
        ),
        (vec![("HAS_CARGO_REGISTRY_TOKEN", "")], "none"),
        (vec![], "none"),
    ] {
        let (status, out, _) = run(&["mode"], &environment);
        assert_eq!(
            (status, out.as_str()),
            (0, expected),
            "mode for {environment:?}"
        );
    }

    // A commit of a release line is a development pre-release of its release.
    assert_eq!(
        run(&["prerelease", "release/v0.4", "242"], &[]).1,
        "0.4.0-dev.242"
    );
    assert_eq!(
        run(&["prerelease", "release/v12.3", "7"], &[]).1,
        "12.3.0-dev.7"
    );
    for (reference, count) in [
        ("main", "242"),
        ("release/v0.4.1", "242"),
        ("refs/heads/release/v0.4", "242"),
        ("release/v0.4", "x"),
        ("release/v0.4", ""),
        ("release/v0.4", "-1"),
    ] {
        let (status, _, error) = run(&["prerelease", reference, count], &[]);
        assert_ne!(
            status, 0,
            "{reference} at {count:?} names no pre-release: {error}"
        );
    }

    // The workspace and its crates take the stamped version, and nothing else does.
    let manifest = root_file("Cargo.toml");
    let stamped = dir.join("stamped");
    std::fs::create_dir(&stamped).expect("mkdir");
    std::fs::write(stamped.join("Cargo.toml"), &manifest).expect("write");
    let (status, previous, error) = run(
        &["stamp", "0.4.0-dev.242", stamped.to_str().expect("utf8")],
        &[],
    );
    assert_eq!(status, 0, "{error}");
    let after = std::fs::read_to_string(stamped.join("Cargo.toml")).expect("read");
    assert_eq!(
        after.matches("0.4.0-dev.242").count(),
        4,
        "the workspace and its three crates"
    );
    assert!(
        !after.contains(&format!("version = \"{previous}\"")),
        "no version stays"
    );
    assert_eq!(
        after.replace("0.4.0-dev.242", &previous),
        manifest,
        "stamping changes the four versions and nothing else"
    );
    for version in ["0.4", "v0.4.0", "0.4.0-rc.1", "latest", ""] {
        let (status, _, _) = run(&["stamp", version, stamped.to_str().expect("utf8")], &[]);
        assert_ne!(
            status, 0,
            "{version:?} is no version a pre-release may take"
        );
    }
    let bare = dir.join("bare");
    std::fs::create_dir(&bare).expect("mkdir");
    std::fs::write(bare.join("Cargo.toml"), "[workspace]\nmembers = []\n").expect("write");
    assert_ne!(
        run(&["stamp", "0.4.0-dev.1", bare.to_str().expect("utf8")], &[]).0,
        0
    );

    // The crate cargo would upload is the crate the release holds and lists.
    let release = dir.join("release");
    std::fs::create_dir(&release).expect("mkdir");
    let crate_bytes: Vec<u8> = (0..4096u32).map(|n| (n * 31 % 251) as u8).collect();
    std::fs::write(release.join("lexlean.crate"), &crate_bytes).expect("write");
    let digest = |bytes: &[u8]| -> String {
        let output = Command::new("sha256sum")
            .current_dir(dir)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                use std::io::Write;
                child.stdin.take().expect("stdin").write_all(bytes)?;
                child.wait_with_output()
            })
            .expect("sha256sum runs");
        String::from_utf8_lossy(&output.stdout)
            .split_whitespace()
            .next()
            .expect("a digest")
            .to_owned()
    };
    let good = digest(&crate_bytes);
    let packaged = dir.join("packaged.crate");
    let release_arg = release.to_str().expect("utf8");
    let packaged_arg = packaged.to_str().expect("utf8");
    let other_hash = format!(
        "{}{}",
        if good.starts_with('0') { "1" } else { "0" },
        &good[1..]
    );
    let mut altered = crate_bytes.clone();
    altered[100] ^= 1;
    let cases: [(&str, Vec<u8>, String, bool); 6] = [
        (
            "the crate the release lists and holds",
            crate_bytes.clone(),
            format!("{good}  lexlean.crate\n"),
            true,
        ),
        (
            "a package one byte from the crate the release holds",
            altered.clone(),
            format!("{good}  lexlean.crate\n"),
            false,
        ),
        (
            "a checksum line of another digest",
            crate_bytes.clone(),
            format!("{other_hash}  lexlean.crate\n"),
            false,
        ),
        (
            "no line for the crate",
            crate_bytes.clone(),
            format!("{good}  SPEC.md\n"),
            false,
        ),
        (
            "two lines for the crate",
            crate_bytes.clone(),
            format!("{good}  lexlean.crate\n{good}  lexlean.crate\n"),
            false,
        ),
        (
            "an empty checksum list",
            crate_bytes.clone(),
            String::new(),
            false,
        ),
    ];
    for (label, package, checksums, accepted) in cases {
        std::fs::write(&packaged, &package).expect("write");
        std::fs::write(release.join("checksums.txt"), &checksums).expect("write");
        let (status, out, error) = run(&["identity", release_arg, packaged_arg], &[]);
        if accepted {
            assert_eq!(
                (status, out.as_str()),
                (0, good.as_str()),
                "{label}: {error}"
            );
        } else {
            assert_ne!(status, 0, "{label} is refused");
            assert!(error.starts_with("identity:"), "{label}: {error}");
        }
    }
    // The release without the crate, without its list, and without a package.
    std::fs::write(&packaged, &crate_bytes).expect("write");
    std::fs::write(
        release.join("checksums.txt"),
        format!("{good}  lexlean.crate\n"),
    )
    .expect("write");
    for missing in ["lexlean.crate", "checksums.txt"] {
        let kept = std::fs::read(release.join(missing)).expect("read");
        std::fs::remove_file(release.join(missing)).expect("remove");
        assert_ne!(
            run(&["identity", release_arg, packaged_arg], &[]).0,
            0,
            "{missing} is missing"
        );
        std::fs::write(release.join(missing), kept).expect("restore");
    }
    assert_ne!(
        run(
            &[
                "identity",
                release_arg,
                dir.join("absent.crate").to_str().expect("utf8")
            ],
            &[]
        )
        .0,
        0,
        "no package"
    );

    // What crates.io needs from an archive.
    let archive = |name: &str, manifest: &[&str], files: &[(&str, Vec<u8>)]| -> String {
        let folder = dir.join(format!("{name}-1.0.0"));
        std::fs::create_dir_all(&folder).expect("mkdir");
        let mut text = format!("[package]\nname = \"{name}\"\nversion = \"1.0.0\"\n");
        for line in manifest {
            text.push_str(line);
            text.push('\n');
        }
        std::fs::write(folder.join("Cargo.toml"), text).expect("write");
        for (file, bytes) in files {
            std::fs::write(folder.join(file), bytes).expect("write");
        }
        let path = dir.join(format!("{name}.crate"));
        let status = Command::new("tar")
            .current_dir(dir)
            .args([
                "-czf",
                path.to_str().expect("utf8"),
                &format!("{name}-1.0.0"),
            ])
            .status()
            .expect("tar runs");
        assert!(status.success());
        path.to_str().expect("utf8").to_owned()
    };
    let complete = [
        "description = \"A sample\"",
        "license = \"MIT\"",
        "repository = \"https://example.test/sample\"",
        "readme = \"README.md\"",
    ];
    let readme: Vec<(&str, Vec<u8>)> = vec![("README.md", b"# sample\n".to_vec())];
    let without = |field: &str| -> Vec<&str> {
        complete
            .iter()
            .copied()
            .filter(|line| !line.starts_with(field))
            .collect()
    };
    let (status, _, error) = run(&["metadata", &archive("complete", &complete, &readme)], &[]);
    assert_eq!(status, 0, "an archive with what crates.io needs: {error}");
    // Incompressible bytes, so that the archive is as large as its contents.
    let mut state = 0x2545_F491_4F6C_DD1Du64;
    let large: Vec<u8> = (0..11 * 1024 * 1024)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            (state >> 24) as u8
        })
        .collect();
    let mut oversize = readme.clone();
    oversize.push(("padding.bin", large));
    for (label, path, says) in [
        (
            "no description",
            archive("nodescription", &without("description"), &readme),
            "no `description`",
        ),
        (
            "no license",
            archive("nolicense", &without("license"), &readme),
            "neither `license` nor `license-file`",
        ),
        (
            "no repository",
            archive("norepository", &without("repository"), &readme),
            "no `repository`",
        ),
        (
            "a readme it lacks",
            archive("noreadme", &complete, &[]),
            "names a file the archive does not carry",
        ),
        (
            "more than 10 MiB",
            archive("oversize", &complete, &oversize),
            "exceeds the crates.io limit",
        ),
    ] {
        let (status, _, error) = run(&["metadata", &path], &[]);
        assert_ne!(status, 0, "{label} is refused");
        assert!(error.contains(says), "{label}: {error}");
    }
}

/// RP-18: see [`crate_publication_workflows`] and
/// [`crate_publication_decisions`]. The workflows are read as committed and
/// then with one check removed or reversed at a time; a removal that changes
/// nothing is itself a failure, because it would plant nothing.
fn rp_18() {
    let release = root_file(".github/workflows/release.yml");
    let crate_workflow = root_file(".github/workflows/crate.yml");
    crate_publication_workflows(&release, &crate_workflow)
        .expect("the committed workflows prove the crate and gate its upload");

    let replaced = |text: &str, from: &str, to: &str| -> String {
        assert!(text.contains(from), "the plant `{from}` is in the workflow");
        text.replacen(from, to, 1)
    };
    let publish_dry_run =
        "cargo publish --dry-run --locked --package lexlean --target-dir \"${RUNNER_TEMP}/dry\"";
    let release_plants: Vec<(&str, String)> = vec![
        (
            "a tag that does not wait for the crate job",
            replaced(&release, "needs: [identity, crate, gate, binaries, manifest]", "needs: [identity, gate, binaries, manifest]"),
        ),
        (
            "a release that does not call the crate workflow",
            replaced(&release, "uses: ./.github/workflows/crate.yml", "uses: ./.github/workflows/other.yml"),
        ),
        (
            "an upload without the dry run first",
            replaced(&release, publish_dry_run, "true"),
        ),
        (
            "an upload without the identity check",
            replaced(&release, ".github/crate-publication.sh identity release", "true"),
        ),
        (
            "an upload that ignores how it is authorised",
            replaced(
                &release,
                "- name: Publish the exact verified crate to crates.io\n        if: steps.upload.outputs.mode != 'none'\n",
                "- name: Publish the exact verified crate to crates.io\n",
            ),
        ),
        (
            "an upload whose guard is always true",
            replaced(
                &release,
                "if: steps.upload.outputs.mode != 'none'\n        env:\n          CARGO_REGISTRY_TOKEN",
                "if: always()\n        env:\n          CARGO_REGISTRY_TOKEN",
            ),
        ),
        (
            "trusted publishing acquired without being selected",
            replaced(
                &release,
                "- name: Acquire crates.io trusted-publisher token\n        if: steps.upload.outputs.mode == 'trusted'\n",
                "- name: Acquire crates.io trusted-publisher token\n",
            ),
        ),
        (
            "a second upload step",
            format!("{release}\n      - run: cargo publish --locked --package lexlean\n"),
        ),
        (
            "the token read where it is not needed",
            replaced(&release, "      TRUSTED_PUBLISHING:", "      EXTRA: ${{ secrets.CARGO_REGISTRY_TOKEN }}\n      TRUSTED_PUBLISHING:"),
        ),
        (
            "a gate job that runs the gate on one runner",
            replaced(&release, "          .github/release-gate.sh\n", "          just vv 2>&1 | tee -a release/vv-evidence.txt\n"),
        ),
        (
            "a gate job that cannot read the runs",
            replaced(&release, "      contents: read\n      actions: read\n    steps:\n      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1\n        with:\n          # The second parent", "      contents: read\n    steps:\n      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1\n        with:\n          # The second parent"),
        ),
    ];
    for (why, text) in &release_plants {
        assert!(
            crate_publication_workflows(text, &crate_workflow).is_err(),
            "release.yml with {why} is refused"
        );
    }
    let cmp_ab = "cmp \"$a/package/$crate\" \"$b/package/$crate\"";
    let crate_plants: Vec<(&str, String)> = vec![
        ("packages once, so never compared", replaced(&crate_workflow, cmp_ab, "true")),
        (
            "no dry run",
            replaced(
                &crate_workflow,
                "cargo publish --dry-run --locked \"${dirty[@]}\" --package lexlean --target-dir \"$dry\"",
                "true",
            ),
        ),
        (
            "no archive metadata check",
            replaced(&crate_workflow, ".github/crate-publication.sh metadata \"$dry_crate\"", "true"),
        ),
        (
            "no comparison with the checksum the release lists",
            replaced(&crate_workflow, ".github/crate-publication.sh identity release \"$dry_crate\"", "true"),
        ),
        (
            "a pre-release uploaded by a call from the release workflow",
            replaced(
                &crate_workflow,
                "if: ${{ github.event_name == 'workflow_dispatch' && inputs.prerelease }}",
                "if: ${{ inputs.prerelease }}",
            ),
        ),
        (
            "a pre-release uploaded without a credential",
            replaced(
                &crate_workflow,
                "- name: Upload the exact checked crate\n        if: ${{ steps.upload.outputs.mode != 'none' }}\n",
                "- name: Upload the exact checked crate\n",
            ),
        ),
        (
            "a pre-release uploaded without comparing its bytes",
            replaced(&crate_workflow, "test \"$(sha256sum \"${RUNNER_TEMP}/upload/package/lexlean-${VERSION}.crate\"", "true \"$(sha256sum \"${RUNNER_TEMP}/upload/package/lexlean-${VERSION}.crate\""),
        ),
        (
            "the readiness job uploading",
            replaced(&crate_workflow, "          cmp \"$a/package/$crate\" \"$dry_crate\"\n", "          cmp \"$a/package/$crate\" \"$dry_crate\"\n          cargo publish --locked --package lexlean\n"),
        ),
    ];
    for (why, text) in &crate_plants {
        assert!(
            crate_publication_workflows(&release, text).is_err(),
            "crate.yml with {why} is refused"
        );
    }

    crate_publication_decisions();
    release_gate_decisions();
}

/// §8.3 custody is a job graph: one construction, inventory-owned
/// executions, and the original host context only after the complete join.
/// Behavioral plants live in the Python suite; this checks that CI runs it
/// and connects the real build, execution and join operations to each other.
fn intel_host_workflow(ci: &str) -> Result<(), String> {
    let matrix = workflow_job(ci, "host-matrix")?;
    let construction = workflow_job(ci, "intel-construction")?;
    let part = workflow_job(ci, "intel-part")?;
    let join = workflow_job(ci, "intel-join")?;
    let honesty = workflow_job(ci, "honesty")?;
    let require = |body: &str, text: &str, rule: &str| -> Result<(), String> {
        if body.contains(text) {
            Ok(())
        } else {
            Err(format!("RP-14: {rule}: missing `{text}`"))
        }
    };
    let mut contexts: Vec<String> = matrix
        .lines()
        .filter_map(|line| line.trim().strip_prefix("- { os: "))
        .map(|row| {
            let target = row.split("target: ").nth(1).unwrap_or_default();
            format!(
                "build and non-Lean tests ({})",
                target.trim_end_matches(" }")
            )
        })
        .collect();
    let aggregate_name = join
        .lines()
        .find_map(|line| line.strip_prefix("    name: "))
        .ok_or("RP-14: the Intel aggregate has no context")?;
    contexts.push(aggregate_name.to_owned());
    let windows = workflow_job(ci, "windows-join")?;
    contexts.push(
        windows
            .lines()
            .find_map(|line| line.strip_prefix("    name: "))
            .ok_or("RP-14: the Windows aggregate has no context")?
            .to_owned(),
    );
    contexts.sort();
    let mut expected = supported_host_contexts();
    expected.sort();
    if contexts != expected {
        return Err(
            "RP-14: the matrix and Intel aggregate must expose each exact host context once"
                .to_owned(),
        );
    }
    require(
        &matrix,
        "name: build and non-Lean tests (${{ matrix.target }})",
        "the ordinary host contexts resolve their native target",
    )?;
    for (key, body, dependencies, condition) in [
        (
            "intel-construction",
            &construction,
            "needs: [plan, honesty]",
            "if: ${{ needs.plan.outputs.full == 'true' }}",
        ),
        (
            "intel-part",
            &part,
            "needs: [plan, intel-construction]",
            "if: ${{ needs.plan.outputs.full == 'true' }}",
        ),
        (
            "intel-join",
            &join,
            "needs: [plan, intel-construction, intel-part]",
            "if: ${{ always() && needs.plan.outputs.full == 'true' }}",
        ),
    ] {
        require(body, dependencies, key)?;
        let conditions: Vec<&str> = body
            .lines()
            .filter_map(|line| line.strip_prefix("    if: "))
            .collect();
        if conditions != vec![condition.trim_start_matches("if: ")] {
            return Err(format!(
                "RP-14: {key} does not follow its full-gate dependency policy"
            ));
        }
        for forbidden in [
            "run-id:",
            "github-token:",
            "continue-on-error",
            "actions/cache",
        ] {
            if body.contains(forbidden) {
                return Err(format!("RP-14: {key} admits `{forbidden}`"));
            }
        }
        // A job's evidence upload can run after failure, but its actual
        // operation must not be guarded by a success/skip shortcut.
        for step in body.split("\n      - ") {
            if step.contains(".github/scripts/intel_host_tests.py ")
                && step
                    .lines()
                    .any(|line| line.trim_start().starts_with("if:"))
            {
                return Err(format!(
                    "RP-14: {key} conditionally skips its factory operation"
                ));
            }
        }
    }
    for body in [&construction, &part] {
        require(
            body,
            "runs-on: macos-15-intel",
            "Intel execution uses its native host",
        )?;
        require(
            body,
            "run: rustup toolchain install",
            "Intel execution uses the Rust pin",
        )?;
        let timeout = body
            .lines()
            .find_map(|line| line.trim().strip_prefix("timeout-minutes: "))
            .and_then(|value| value.parse::<u32>().ok())
            .ok_or("RP-14: an Intel execution job has no timeout")?;
        if timeout == 0 || timeout > 240 {
            return Err(
                "RP-14: Intel execution cannot raise the prior 240-minute bound".to_owned(),
            );
        }
    }
    for (body, command) in [
        (&construction, "build --output"),
        (&part, "execute --construction"),
        (&join, "join --artifacts"),
    ] {
        require(
            body,
            &format!("python3 .github/scripts/intel_host_tests.py {command}"),
            "the job runs its factory operation",
        )?;
    }
    for (body, expected) in [
        (&construction, "lanes: ${{ steps.construct.outputs.lanes }}"),
        (&construction, "id: construct"),
        (&construction, "name: intel-construction"),
        (&part, "fail-fast: false"),
        (
            &part,
            "lane: ${{ fromJSON(needs.intel-construction.outputs.lanes) }}",
        ),
        (&part, "name: intel-construction"),
        (&part, "name: intel-${{ matrix.lane }}"),
        (&part, "--lane ${{ matrix.lane }}"),
        (&join, "pattern: intel-*"),
        (
            &join,
            "CONSTRUCTION_RESULT: ${{ needs.intel-construction.result }}",
        ),
        (&join, "PART_RESULT: ${{ needs.intel-part.result }}"),
        (&join, "test \"$CONSTRUCTION_RESULT\" = success"),
        (&join, "test \"$PART_RESULT\" = success"),
    ] {
        require(body, expected, "same-run construction and receipt custody")?;
    }
    let result_step = join
        .split("\n      - ")
        .find(|step| step.contains("test \"$PART_RESULT\" = success"))
        .ok_or("RP-14: the aggregate does not check its prerequisite outcomes")?;
    require(
        result_step,
        "if: ${{ always() }}",
        "the aggregate cannot hide a failed or cancelled prerequisite",
    )?;
    require(
        &honesty,
        "python3 -m unittest discover -s .github/scripts -p 'test_intel_host_tests.py'",
        "the factory's behavioral tests run before expensive roots",
    )?;
    Ok(())
}

/// The fetcher a leg runs beside its gate command to take the run's
/// bootstrap export (§9.4).
const TAKE_EXPORT: &str = ".github/scripts/take-bootstrap-export.sh";

/// §9.4: one bootstrap export per run, carried to the legs of the same run.
/// The export job follows honesty and the plan's whole-gate decision, runs
/// the export at the profile of the `examples` leg, and uploads it under the
/// one name the fetcher reads; a leg reads the artifacts of its own run only,
/// takes the export only in a run that is the whole gate, and only into the
/// directory its harness waits in; the fetcher names this run and the export
/// job, and asks nothing before a case wants the export.
fn bootstrap_export_workflow(ci: &str, take: &str) -> Result<(), String> {
    let job = workflow_job(ci, "bootstrap-export")?;
    let legs = workflow_job(ci, "vv")?;
    let require = |body: &str, text: &str, rule: &str| -> Result<(), String> {
        if body.contains(text) {
            Ok(())
        } else {
            Err(format!("RP-14: {rule}: missing `{text}`"))
        }
    };
    for (text, rule) in [
        ("    name: bootstrap export\n", "the job the fetcher watches"),
        ("    needs: [plan, honesty]\n", "the export follows honesty"),
        (
            "    if: ${{ needs.plan.outputs.full == 'true' }}\n",
            "the export runs in a run that is the whole gate",
        ),
        ("      LEXLEAN_VERIFY_WIDTH: \"2\"\n", "the examples leg's width"),
        (
            "      LEXLEAN_VERIFY_MEMORY_KIB: \"12582912\"\n",
            "the examples leg's budget",
        ),
        (
            "      LEXLEAN_VERIFY_HINTS: ${{ github.workspace }}/.github/verify-hints.json\n",
            "the committed hints",
        ),
        (
            "cargo xtask export-bootstrap --output \"$RUNNER_TEMP/bootstrap-export\" 2>&1 | tee ci-evidence/vv.txt\n",
            "the export, with the comparison and the custody record",
        ),
        ("          name: bootstrap-export\n", "the artifact the fetcher reads"),
        (
            "          path: ${{ runner.temp }}/bootstrap-export\n",
            "the directory the export wrote",
        ),
        (
            "name: acceptance-gate-evidence-bootstrap-export\n",
            "the job's evidence",
        ),
    ] {
        require(&job, text, rule)?;
    }
    for (text, rule) in [
        ("      actions: read\n", "a leg lists the run's artifacts"),
        (
            "          FULL_GATE: ${{ needs.plan.outputs.full }}\n",
            "a leg takes the export only in a run that is the whole gate",
        ),
        (
            "          if [ \"$FULL_GATE\" = true ]; then\n            export LEXLEAN_BOOTSTRAP_EXPORT=\"$RUNNER_TEMP/bootstrap-export\"\n            .github/scripts/take-bootstrap-export.sh \"$LEXLEAN_BOOTSTRAP_EXPORT\"",
            "the fetcher fills the directory the harness waits in",
        ),
    ] {
        require(&legs, text, rule)?;
    }
    for body in [&job, &legs] {
        for forbidden in [
            "run-id:",
            "github-token:",
            "actions/cache",
            "continue-on-error",
        ] {
            if body.contains(forbidden) {
                return Err(format!("RP-14: the export's custody admits `{forbidden}`"));
            }
        }
    }
    for (text, rule) in [
        ("run=${GITHUB_RUN_ID:?}", "the fetcher reads this run"),
        (
            "repos/$repo/actions/runs/$run/artifacts?name=bootstrap-export",
            "the fetcher reads the export job's artifact",
        ),
        (
            "select(.name == \"bootstrap export\")",
            "the fetcher watches the export job",
        ),
        (
            "until [ -e \"$dir/wanted\" ]",
            "the fetcher asks nothing before a case wants the export",
        ),
    ] {
        require(take, text, rule)?;
    }
    if take.matches("actions/runs/").count() != 2 || take.contains("GITHUB_RUN_ID=") {
        return Err("RP-14: the fetcher reads the run it is in and no other".to_owned());
    }
    Ok(())
}

/// §9.4: a custody record names the run and checkout that take the export,
/// and every way it can differ is refused.
fn bootstrap_custody_decisions() {
    use crate::export_custody::{check, Custody, Identity, SPEC};
    use std::collections::BTreeMap;
    let pins: BTreeMap<String, String> = [
        ("Cargo.lock", "a".repeat(64)),
        ("lean-toolchain", "b".repeat(64)),
        ("rust-toolchain.toml", "c".repeat(64)),
    ]
    .into_iter()
    .map(|(name, digest)| (name.to_owned(), digest))
    .collect();
    let toolchains: BTreeMap<String, String> =
        [("lean", "Lean (version 4.32.1)"), ("rustc", "rustc 1.97.1")]
            .into_iter()
            .map(|(name, text)| (name.to_owned(), text.to_owned()))
            .collect();
    let here = Identity {
        repository: "UOR-Foundation/LexLean".to_owned(),
        run_id: "100".to_owned(),
        run_attempt: "2".to_owned(),
        commit: "1".repeat(40),
        tree: "2".repeat(40),
        pins: pins.clone(),
        toolchains: toolchains.clone(),
    };
    let bundle = ("d".repeat(64), 162);
    let record = Custody {
        spec: SPEC.to_owned(),
        repository: here.repository.clone(),
        run_id: here.run_id.clone(),
        run_attempt: "1".to_owned(),
        commit: here.commit.clone(),
        tree: here.tree.clone(),
        pins,
        toolchains,
        bundle_path: ".lexlean/production/bundle".to_owned(),
        bundle_sha256: bundle.0.clone(),
        bundle_files: bundle.1,
    };
    check(&record, &here, &bundle).expect("§9.4: the run's own export, from an earlier attempt");
    type Plant = Box<dyn Fn(&mut Custody)>;
    let plants: Vec<(&str, Plant)> = vec![
        ("spec", Box::new(|record| record.spec.push('2'))),
        (
            "repository",
            Box::new(|record| record.repository = "fork/LexLean".to_owned()),
        ),
        ("run", Box::new(|record| record.run_id = "99".to_owned())),
        (
            "attempt",
            Box::new(|record| record.run_attempt = "3".to_owned()),
        ),
        ("commit", Box::new(|record| record.commit = "3".repeat(40))),
        ("tree", Box::new(|record| record.tree = "4".repeat(40))),
        (
            "pins",
            Box::new(|record| {
                record
                    .pins
                    .insert("lean-toolchain".to_owned(), "e".repeat(64));
            }),
        ),
        (
            "toolchains",
            Box::new(|record| {
                record
                    .toolchains
                    .insert("rustc".to_owned(), "rustc 1.97.0".to_owned());
            }),
        ),
        (
            "bundle",
            Box::new(|record| record.bundle_sha256 = "f".repeat(64)),
        ),
        ("bundle", Box::new(|record| record.bundle_files += 1)),
        (
            "bundle path",
            Box::new(|record| record.bundle_path = "../outside".to_owned()),
        ),
        (
            "bundle path",
            Box::new(|record| record.bundle_path = "/abs/bundle".to_owned()),
        ),
    ];
    for (field, plant) in plants {
        let mut planted = record.clone();
        plant(&mut planted);
        let refusal = check(&planted, &here, &bundle)
            .expect_err("§9.4: a custody record that names another run, tree, pin, or bundle");
        assert!(
            refusal.starts_with(field) && refusal.lines().count() == 1,
            "the refusal names the field `{field}` alone: {refusal}"
        );
    }
}

/// §8.3: Windows runs as the lanes of one run, the conformance lanes of the
/// Intel partition and the producer, after honesty and only in a run that is
/// the whole gate; its aggregate keeps the host's context, runs after any
/// lane's failure, joins every lane's receipt, and fails on a missing one.
fn windows_host_workflow(ci: &str) -> Result<(), String> {
    let part = workflow_job(ci, "windows-part")?;
    let join = workflow_job(ci, "windows-join")?;
    let honesty = workflow_job(ci, "honesty")?;
    let require = |body: &str, text: &str, rule: &str| -> Result<(), String> {
        if body.contains(text) {
            Ok(())
        } else {
            Err(format!("RP-14: {rule}: missing `{text}`"))
        }
    };
    let plan: serde_json::Value = serde_json::from_str(&root_file(".github/intel-host-lanes.json"))
        .map_err(|error| format!("RP-14: the lanes file: {error}"))?;
    let mut lanes = vec!["producer".to_owned()];
    lanes.extend(
        plan["lanes"]
            .as_object()
            .ok_or("RP-14: the lanes file names its lanes")?
            .keys()
            .cloned(),
    );
    lanes.sort();
    let mut matrix: Vec<String> = part
        .lines()
        .find_map(|line| line.strip_prefix("        lane: ["))
        .and_then(|rest| rest.strip_suffix(']'))
        .ok_or("RP-14: the Windows matrix names its lanes")?
        .split(',')
        .map(|lane| lane.trim().to_owned())
        .collect();
    matrix.sort();
    if matrix != lanes {
        return Err(format!(
            "RP-14: the Windows matrix runs {matrix:?}, the producer and the lanes of the partition are {lanes:?}"
        ));
    }
    for (body, text, rule) in [
        (&part, "    needs: [plan, honesty]\n", "the lanes follow honesty"),
        (
            &part,
            "    if: ${{ needs.plan.outputs.full == 'true' }}\n",
            "the lanes run in a run that is the whole gate",
        ),
        (&part, "      fail-fast: false\n", "a failing lane stops no other"),
        (&part, "    runs-on: windows-2022\n", "the lanes run on the host"),
        (&part, "        run: rustup toolchain install\n", "the Rust pin"),
        (
            &part,
            "if [ \"$host\" != x86_64-pc-windows-msvc ]; then",
            "a lane checks its host triple",
        ),
        (
            &part,
            "python .github/scripts/host_lanes.py execute --target x86_64-pc-windows-msvc --lane ${{ matrix.lane }} --output \"$RUNNER_TEMP/host-lane\"",
            "a lane executes and records its part",
        ),
        (
            &part,
            "          name: host-x86_64-pc-windows-msvc-${{ matrix.lane }}\n",
            "a lane uploads its receipt",
        ),
        (
            &join,
            "    name: build and non-Lean tests (x86_64-pc-windows-msvc)\n",
            "the aggregate keeps the host's context",
        ),
        (&join, "    needs: [plan, windows-part]\n", "the aggregate follows the lanes"),
        (
            &join,
            "    if: ${{ always() && needs.plan.outputs.full == 'true' }}\n",
            "the aggregate runs after a failed or cancelled lane",
        ),
        (
            &join,
            "          pattern: host-x86_64-pc-windows-msvc-*\n",
            "the aggregate reads every lane's receipt",
        ),
        (
            &join,
            "python3 .github/scripts/host_lanes.py join --target x86_64-pc-windows-msvc --artifacts \"$RUNNER_TEMP/host-lanes\"",
            "the aggregate joins the receipts",
        ),
        (
            &join,
            "PART_RESULT: ${{ needs.windows-part.result }}",
            "the aggregate reads the lanes' result",
        ),
        (
            &honesty,
            "python3 -m unittest discover -s .github/scripts -p 'test_host_lanes.py'",
            "the join's behavioral tests run before expensive roots",
        ),
    ] {
        require(body, text, rule)?;
    }
    let result_step = join
        .split("\n      - ")
        .find(|step| step.contains("test \"$PART_RESULT\" = success"))
        .ok_or("RP-14: the Windows aggregate does not check its lanes' result")?;
    require(
        result_step,
        "if: ${{ always() }}",
        "the aggregate cannot hide a failed or cancelled lane",
    )?;
    let timeout = part
        .lines()
        .find_map(|line| line.trim().strip_prefix("timeout-minutes: "))
        .and_then(|value| value.parse::<u32>().ok())
        .ok_or("RP-14: a Windows lane has no timeout")?;
    if timeout == 0 || timeout > 240 {
        return Err("RP-14: a Windows lane cannot raise the 240-minute bound".to_owned());
    }
    for body in [&part, &join] {
        for forbidden in [
            "run-id:",
            "github-token:",
            "continue-on-error",
            "actions/cache",
        ] {
            if body.contains(forbidden) {
                return Err(format!("RP-14: the Windows lanes admit `{forbidden}`"));
            }
        }
    }
    Ok(())
}

/// Claim drift must finish successfully before a costly root can start.
fn honesty_dependencies(ci: &str) -> Result<(), String> {
    let initial = workflow_job(ci, "honesty")?;
    if !initial.contains("name: ${{ matrix.part }}\n")
        || !initial.contains("needs: [plan]\n")
        || !initial.contains("if: ${{ needs.plan.outputs.honesty != '[]' }}")
    {
        return Err(
            "RP-14: honesty must expose its matrix-resolved context and follow the plan".to_owned(),
        );
    }
    for root in [
        "bootstrap-export",
        "vv",
        "atlas-prefix",
        "host-matrix",
        "intel-construction",
        "windows-part",
    ] {
        let body = workflow_job(ci, root)?;
        if !body.contains("needs: [plan, honesty]\n")
            || body
                .lines()
                .any(|line| line.starts_with("    if:") && line.contains("always()"))
        {
            return Err(format!("RP-14: {root} must wait for successful honesty"));
        }
    }
    Ok(())
}

/// What `.github/release-gate.sh` accepts as the release's evidence (§9.4,
/// §30.3): the script runs in a scratch repository with a stand-in for `gh`
/// that answers with planted runs and jobs, so that the run it names is shown
/// and each way of being wrong is refused.
fn release_gate_decisions() {
    use std::process::Command;
    // Bash, `git`, `jq`, and the stand-in on `PATH`, where the gate runs
    // (§8.3); the other hosts of the matrix skip this part as `plan_decisions`
    // does, and no decision depends on the host.
    if !cfg!(target_os = "linux") {
        return;
    }
    let root = support::repo_root();
    let script = root.join(".github/release-gate.sh");
    let rows_file = root.join(".github/vv-rows.json");
    let rows: Vec<serde_json::Value> =
        serde_json::from_str(&root_file(".github/vv-rows.json")).expect("the legs");
    let hosts = supported_host_contexts();
    let stamp = |name: &str, conclusion: &str| {
        format!("{name}\t{conclusion}\t2026-01-01T00:00:00Z\t2026-01-01T01:00:00Z")
    };
    let mut green: Vec<String> = vec![stamp("plan", "success")];
    green.extend(rows.iter().map(|row| {
        stamp(
            &if row["part"] == "honesty" {
                "honesty".to_owned()
            } else {
                format!("just vv ({})", row["part"].as_str().expect("a part"))
            },
            "success",
        )
    }));
    green.push(stamp("atlas (prefix)", "success"));
    green.push(stamp("atlas (join)", "success"));
    green.extend(hosts.iter().map(|host| stamp(host, "success")));
    // One host failed: the join, the hosts' count, and every leg are still
    // there, so only the reading of every conclusion refuses it.
    let one_failed: Vec<String> = green
        .iter()
        .map(|job| {
            job.replacen(
                &format!("{}\tsuccess", hosts[0]),
                &format!("{}\tfailure", hosts[0]),
                1,
            )
        })
        .collect();
    let without_honesty: Vec<String> = green
        .iter()
        .filter(|job| !job.starts_with("honesty"))
        .cloned()
        .collect();
    let failed_honesty: Vec<String> = green
        .iter()
        .map(|job| job.replace("honesty\tsuccess", "honesty\tfailure"))
        .collect();
    let skipped_honesty: Vec<String> = green
        .iter()
        .map(|job| job.replace("honesty\tsuccess", "honesty\tskipped"))
        .collect();
    let suffixed_honesty: Vec<String> = green
        .iter()
        .map(|job| {
            job.replace(
                "honesty\tsuccess",
                "honesty (honesty, fmt-check model spec-links bdd, honesty)\tsuccess",
            )
        })
        .collect();
    let no_join: Vec<String> = green
        .iter()
        .filter(|job| !job.starts_with("atlas (join)"))
        .cloned()
        .collect();
    let draft: Vec<String> = vec![stamp("plan", "success"), stamp("just vv (rest)", "success")];
    // Every job but one exact host context is incomplete evidence.
    let missing_host: Vec<String> = green
        .iter()
        .filter(|job| !job.starts_with(hosts[1].as_str()))
        .cloned()
        .collect();
    // Every job but one leg: only the reading of every leg refuses it.
    let missing_leg: Vec<String> = green
        .iter()
        .filter(|job| !job.starts_with("just vv (examples)"))
        .cloned()
        .collect();

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
    git(&["checkout", "-q", "-b", "head"]);
    write("b", "b\n");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "change"]);
    let head = git(&["rev-parse", "HEAD"]);
    git(&["checkout", "-q", "main"]);
    git(&["merge", "-q", "--no-ff", "-m", "merge", "head"]);
    let merge_equal = git(&["rev-parse", "HEAD"]);
    git(&["checkout", "-q", "-b", "behind", &base]);
    write("c", "c\n");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "change"]);
    let behind = git(&["rev-parse", "HEAD"]);
    git(&["checkout", "-q", "main"]);
    write("d", "d\n");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "main moves"]);
    git(&["merge", "-q", "--no-ff", "-m", "merge", "behind"]);
    let merge_different = git(&["rev-parse", "HEAD"]);
    // A merge of the equal-tree merge: a cut's release merge carried on, two
    // merges above the head whose run verified the tree.
    git(&["checkout", "-q", "-b", "carried", &base]);
    git(&["merge", "-q", "--no-ff", "-m", "carry", &merge_equal]);
    let carried = git(&["rev-parse", "HEAD"]);
    let mut deeper = carried.clone();
    let mut chain = Vec::new();
    for hop in 3..=5 {
        git(&["checkout", "-q", "-b", &format!("carried-{hop}"), &base]);
        git(&["merge", "-q", "--no-ff", "-m", "carry", &deeper]);
        deeper = git(&["rev-parse", "HEAD"]);
        chain.push(deeper.clone());
    }
    let (four_down, five_down) = (chain[1].clone(), chain[2].clone());

    // `gh`: the runs of a commit (`STUB_RUNS_<sha>`), the jobs of every run
    // (`STUB_JOBS`), and a download that writes one leg's transcript.
    let bin = dir.join("bin");
    std::fs::create_dir(&bin).expect("bin");
    let stub = bin.join("gh");
    std::fs::write(
        &stub,
        "#!/usr/bin/env bash\n\
         case \"$1 $2\" in\n\
           api\\ */workflows/ci.yml/runs\\?head_sha=*)\n\
             sha=${2#*head_sha=}; sha=${sha%%&*}\n\
             var=STUB_RUNS_$sha; printf '%s\\n' \"${!var:-}\" ;;\n\
           api\\ */runs/*/jobs\\?*) printf '%s\\n' \"${STUB_JOBS:-}\" ;;\n\
           run\\ download)\n\
             while [ $# -gt 0 ]; do [ \"$1\" = -D ] && dest=$2; shift; done\n\
             mkdir -p \"$dest/acceptance-gate-evidence-rest\"\n\
             printf '%s\\n' '[1.000] vv part: every recipe passed' > \"$dest/acceptance-gate-evidence-rest/vv.txt\" ;;\n\
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
    let run_line = |id: &str| {
        format!("{id}\tpush\t2026-01-01T00:00:00Z\t2026-01-01T02:00:00Z\thttps://example.test/runs/{id}")
    };
    // (exit code, the evidence, stderr) for a planted commit, the runs of each
    // commit, and the jobs of every run.
    let decide = |sha: &str, runs: &[(&str, String)], jobs: &[String]| {
        let out = dir.join("vv-evidence.txt");
        let _ = std::fs::remove_file(&out);
        let mut command = Command::new("bash");
        command
            .current_dir(dir)
            .arg(script.as_std_path())
            .env("PATH", &path)
            .env("REPO", "owner/repo")
            .env("SHA", sha)
            .env("OUT", &out)
            .env("REF", "refs/tags/v0.0.0")
            .env("RELEASE_RUN", "https://example.test/release")
            .env("LEXLEAN_CI_ROWS", rows_file.as_std_path())
            .env("STUB_JOBS", jobs.join("\n"));
        for (commit, lines) in runs {
            command.env(format!("STUB_RUNS_{commit}"), lines);
        }
        let output = command.output().expect("bash runs");
        (
            output.status.code().expect("an exit code"),
            std::fs::read_to_string(&out).unwrap_or_default(),
            String::from_utf8_lossy(&output.stderr).into_owned(),
        )
    };

    // A run of the whole gate on the commit itself.
    let (code, evidence, stderr) = decide(&head, &[(head.as_str(), run_line("11"))], &green);
    assert_eq!(
        code, 0,
        "a passed gate run on the commit is the evidence: {stderr}"
    );
    for needed in [
        &format!("commit {head}"),
        "gate run https://example.test/runs/11",
        &format!("gate run commit {head} (the tagged commit)"),
        "atlas (join)\tsuccess",
        "--- acceptance-gate-evidence-rest/vv.txt",
        "vv part: every recipe passed",
    ] {
        assert!(
            evidence.contains(needed),
            "the evidence carries `{needed}`:\n{evidence}"
        );
    }
    // The release job reads the file with the §30.3 criterion: what the
    // script writes is what `release-check` accepts, or the tag's run would
    // refuse evidence every rehearsal passed.
    assert_eq!(
        repo_model::release::ci_evidence(&evidence),
        Ok(()),
        "the evidence satisfies the release criterion"
    );
    // A complete listing can exceed a pipe buffer as the workflow grows.
    // Matching a required row must still consume the rest of the listing.
    let mut large = green.clone();
    large.extend(
        (0..1500).map(|index| stamp(&format!("additional successful job {index}"), "success")),
    );
    let (code, _, stderr) = decide(&head, &[(head.as_str(), run_line("19"))], &large);
    assert_eq!(code, 0, "a large complete gate is accepted: {stderr}");
    large.push(stamp("a failed job after every required row", "failure"));
    let (code, evidence, stderr) = decide(&head, &[(head.as_str(), run_line("20"))], &large);
    assert_eq!(code, 1, "a late failed job is refused: {stderr}");
    assert!(evidence.is_empty(), "a late failure writes no evidence");
    // The merge of a head whose tree it lands unchanged: the head's run.
    let (code, evidence, stderr) = decide(&merge_equal, &[(head.as_str(), run_line("12"))], &green);
    assert_eq!(
        code, 0,
        "the run on the tree-equal parent is the evidence: {stderr}"
    );
    assert!(
        evidence.contains(&format!("gate run commit {head} (the second parent")),
        "the evidence names the parent:\n{evidence}"
    );
    // Two equal-tree merges above the head: the head's run, as the plan finds
    // it for the push it ran nothing for.
    let (code, evidence, stderr) = decide(&carried, &[(head.as_str(), run_line("25"))], &green);
    assert_eq!(
        code, 0,
        "the run two equal-tree merges down is the evidence: {stderr}"
    );
    assert!(
        evidence.contains(&format!("gate run commit {head} (the second parent")),
        "the evidence names the head:\n{evidence}"
    );
    // Four merges is the bound the plan shares; a fifth is refused.
    let (code, _, stderr) = decide(&four_down, &[(head.as_str(), run_line("26"))], &green);
    assert_eq!(code, 0, "four equal-tree merges down: {stderr}");
    let (code, _, _) = decide(&five_down, &[(head.as_str(), run_line("27"))], &green);
    assert_eq!(code, 1, "five equal-tree merges down is beyond the walk");
    // A merge whose tree no run verified: the parent's run is not evidence.
    let (code, _, stderr) = decide(
        &merge_different,
        &[(behind.as_str(), run_line("13"))],
        &green,
    );
    assert_eq!(
        code, 1,
        "a run on a parent whose tree the merge changed is refused"
    );
    assert!(
        stderr.contains("no run of ci.yml was the whole gate"),
        "{stderr}"
    );
    // No run, a run with a failed job, a run without the join, a draft run.
    for (why, runs, jobs) in [
        ("no run", vec![], &green),
        (
            "suffixed honesty",
            vec![(head.as_str(), run_line("24"))],
            &suffixed_honesty,
        ),
        (
            "missing honesty",
            vec![(head.as_str(), run_line("21"))],
            &without_honesty,
        ),
        (
            "failed honesty",
            vec![(head.as_str(), run_line("22"))],
            &failed_honesty,
        ),
        (
            "skipped honesty",
            vec![(head.as_str(), run_line("23"))],
            &skipped_honesty,
        ),
        (
            "a failed job",
            vec![(head.as_str(), run_line("14"))],
            &one_failed,
        ),
        (
            "no Atlas join",
            vec![(head.as_str(), run_line("15"))],
            &no_join,
        ),
        ("a draft run", vec![(head.as_str(), run_line("16"))], &draft),
        (
            "a missing leg",
            vec![(head.as_str(), run_line("17"))],
            &missing_leg,
        ),
        (
            "a missing host",
            vec![(head.as_str(), run_line("18"))],
            &missing_host,
        ),
    ] {
        let (code, evidence, stderr) = decide(&head, &runs, jobs);
        assert_eq!(code, 1, "{why}: refused ({stderr})");
        assert!(evidence.is_empty(), "{why}: no evidence is written");
        assert!(
            stderr.contains("no run of ci.yml was the whole gate"),
            "{why}: {stderr}"
        );
    }
    for host in &hosts {
        let absent: Vec<String> = green
            .iter()
            .filter(|job| !job.starts_with(&format!("{host}\t")))
            .cloned()
            .collect();
        let mut duplicate = green.clone();
        duplicate.push(stamp(host, "success"));
        let foreign: Vec<String> = green
            .iter()
            .map(|job| job.replace(host, "build and non-Lean tests (foreign-target)"))
            .collect();
        let suffixed: Vec<String> = green
            .iter()
            .map(|job| job.replace(host, &format!("{host} (partition)")))
            .collect();
        for (defect, jobs) in [
            ("absent", absent),
            ("duplicate", duplicate),
            ("foreign replacement", foreign),
            ("suffixed", suffixed),
        ] {
            let (code, evidence, stderr) = decide(&head, &[(head.as_str(), run_line("25"))], &jobs);
            assert_eq!(code, 1, "{defect} host context `{host}`: {stderr}");
            assert!(evidence.is_empty(), "{defect} host writes no evidence");
        }
    }
}
