//! The `repository` suite: RP-01..RP-14.

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
                ("bdd", "cargo test -p repo-conformance --all-features"),
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
            assert_eq!(table.len(), 339, "§31 has 339 rows");
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
                    name != "target" && name != ".git" && name != ".lexlean" && name != "expected"
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
            let mut files: Vec<(String, Vec<u8>)> = Vec::new();
            for dir in [
                "language",
                "schemas",
                "tests/golden/axiom-parser",
                "tests/golden/canonical-json",
            ] {
                for entry in walkdir::WalkDir::new(root.join(dir).as_std_path())
                    .into_iter()
                    .flatten()
                {
                    if entry.file_type().is_file() {
                        let relative = format!(
                            "{dir}/{}",
                            entry
                                .path()
                                .strip_prefix(root.join(dir).as_std_path())
                                .expect("under dir")
                                .to_string_lossy()
                                .replace('\\', "/")
                        );
                        files.push((relative, std::fs::read(entry.path()).expect("read")));
                    }
                }
            }
            files.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
            let borrowed: Vec<(&str, &[u8])> = files
                .iter()
                .map(|(path, bytes)| (path.as_str(), bytes.as_slice()))
                .collect();
            let recomputed = lexlean::artifact::content_id::tree_digest(&borrowed);
            assert_eq!(
                recomputed,
                lexlean::compiler_semantics_id_for(lexlean::LATEST_LANGUAGE_VERSION),
                "RP-10: the embedded semantics ID differs from the disk recomputation"
            );

            // §21.2: the older languages' IDs are the same digest over the
            // tree minus the files SPEC.md lists as introduced later. The
            // lists are read from the specification, not from the compiler.
            let spec = support::spec_text();
            let section = spec
                .split("### 21.2 Compiler-semantics ID")
                .nth(1)
                .and_then(|rest| rest.split("### 21.3").next())
                .expect("§21.2");
            let listed = |start: &str, end: &str| -> Vec<String> {
                let text = section
                    .split(start)
                    .nth(1)
                    .and_then(|rest| rest.split(end).next())
                    .unwrap_or_else(|| panic!("§21.2 states `{start}`"));
                let mut out = Vec::new();
                for item in text.split('`').skip(1).step_by(2) {
                    match (item.find('{'), item.find('}')) {
                        (Some(open), Some(close)) => {
                            for choice in item[open + 1..close].split(',') {
                                out.push(format!(
                                    "{}{choice}{}",
                                    &item[..open],
                                    &item[close + 1..]
                                ));
                            }
                        }
                        _ => out.push(item.to_owned()),
                    }
                }
                assert!(!out.is_empty(), "§21.2 lists the files after `{start}`");
                out
            };
            let only_1_2 = listed(
                "The language-1.1 ID excludes the files introduced solely for 1.2:",
                "The language-1.0 ID",
            );
            let only_1_1 = listed(
                "excludes the files introduced solely for 1.1:",
                "Adding a file",
            );
            let excluded = |path: &str, lists: &[&Vec<String>]| {
                lists.iter().any(|list| {
                    list.iter().any(|item| {
                        if item.ends_with('/') {
                            path.starts_with(item.as_str())
                        } else {
                            path == item
                        }
                    })
                })
            };
            let filtered = |lists: &[&Vec<String>]| {
                let kept: Vec<(&str, &[u8])> = borrowed
                    .iter()
                    .copied()
                    .filter(|(path, _)| !excluded(path, lists))
                    .collect();
                lexlean::artifact::content_id::tree_digest(&kept)
            };
            let eleven = filtered(&[&only_1_2]);
            let ten = filtered(&[&only_1_2, &only_1_1]);
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
        // owns its complete suite, and the inventory proves the partition. The
        // inventory itself lists tests with `cargo test -- --list`; it is a
        // recipe of `vv`, one line of the log that runs once after both
        // phases, and this case holds the rest: the recipe lines, the
        // comparison it applies, and that the documents say what runs.
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
            assert_eq!(bdd, "cargo test -p repo-conformance --all-features");
            assert_eq!(inventory, "cargo xtask check-test-inventory");
            let test_arguments =
                crate::inventory::recipe_arguments(&test).expect("test is cargo test");
            let bdd_arguments =
                crate::inventory::recipe_arguments(&bdd).expect("bdd is cargo test");
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

            // The documents say what runs.
            let xtask = root_file("xtask/src/main.rs");
            assert!(xtask.contains("\"check-test-inventory\" =>"));
            let spec = root_file("SPEC.md");
            let verification = root_file("VERIFICATION.md");
            for recipe_command in [&test, &bdd, &inventory] {
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
            assert!(
                ci.contains("    runs-on: ubuntu-24.04\n    # Native Atlas"),
                "the normative job runs on the standard runner"
            );
            for variable in ["LEXLEAN_VERIFY_WIDTH", "vars.", "inputs."] {
                assert!(
                    !ci.contains(variable),
                    "the gate selects no runner and no width: `{variable}`"
                );
            }
            assert!(ci.contains("timeout-minutes: 360"));
            assert_eq!(
                ci.matches(" vv 2>&1 | tee ci-evidence/vv.txt").count(),
                1,
                "the gate runs once"
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
            // A condition appears only on the two evidence steps, in exactly
            // one form, as a step's own key: not on the job (a skipped job is a
            // skipped gate), not on the gate step, and never one that is
            // narrower than "unless the run was cancelled" (a gate that is
            // skipped on pull requests is the weaker PR-only gate).
            let mut conditions = 0;
            for (index, line) in ci.lines().enumerate() {
                if line.trim_start().starts_with("if:") {
                    conditions += 1;
                    assert_eq!(
                        line, "        if: ${{ !cancelled() }}",
                        "ci.yml:{}: the only condition allowed is a step's `if: ${{{{ !cancelled() }}}}`",
                        index + 1
                    );
                }
            }
            assert_eq!(
                conditions, 2,
                "the summary step and the evidence upload carry it"
            );
            let gate_step = &ci[ci
                .find("      - name: Run the normative acceptance gate")
                .expect("the gate step")..];
            let gate_step =
                &gate_step[..gate_step[1..].find("\n      - ").expect("a step follows") + 1];
            assert!(!gate_step.contains("if:") && gate_step.contains("just --timestamp"));
            assert!(
                ci.contains("name: acceptance-gate-evidence") && ci.contains("path: ci-evidence")
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
            let matrix = &ci[ci.find("\n  host-matrix:").expect("the host matrix")..];
            assert_eq!(
                matrix
                    .matches("cargo test --locked --workspace --all-features\n")
                    .count(),
                1,
                "the host matrix runs the workspace's tests once"
            );
            assert!(
                !ci.contains("cargo build"),
                "ci.yml has no `cargo build` step: the host matrix's `cargo test` builds the crate"
            );
            // The supported-host matrix keeps its four other legs.
            for host in [
                "ubuntu-24.04-arm",
                "macos-15-intel",
                "macos-15",
                "windows-2022",
            ] {
                assert!(ci.contains(host), "the host matrix keeps {host}");
            }
            // The honesty and reproducibility workflows stay separate.
            assert!(root_file(".github/workflows/honesty.yml").contains("just model spec-links"));
            assert!(root_file(".github/workflows/reproducibility.yml").contains("run: just repro"));
        }
        other => panic!("no repository case is wired for {other}"),
    }
}
