//! The `performance-corpus` suite: PC-01..PC-02, the general performance
//! corpus (SPEC.md §28.7).
//!
//! The corpus is the one object over which every acceptance criterion about
//! the performance of compiled programs quantifies, so what it is cannot rest
//! on a reading of its sources: a manifest row is a production root of the
//! project declared as the row states, every kind has a member or states the
//! precondition under which it gets one, and nothing in the corpus reaches
//! outside LexLean. Each rule here is armed by a planted defect.

use std::collections::{BTreeMap, BTreeSet};

use lexlean::api::production::{CertificationStatus, ReplayStatus};
use serde::Deserialize;

use crate::production_consumer::{self, AppRoot, Application};
use crate::support::{self, P};

/// The example directory that is the corpus.
pub const EXAMPLE: &str = "performance-corpus";
/// The manifest beside the project configuration.
pub const MANIFEST: &str = "corpus.toml";
/// The manifest's `spec`.
pub const SPEC: &str = "lexlean/performance-corpus/1";
/// The identifier by which a criterion names the whole corpus.
pub const CORPUS_ID: &str = "performance-corpus";
/// The closed kinds, in manifest order.
pub const KINDS: [&str; 6] = [
    "integer-kernels",
    "byte-parsing",
    "long-folds",
    "maps-and-sets",
    "recursion",
    "compiler",
];
/// The kinds that state a precondition and hold no member while it does not
/// hold: the compiler itself is a member once it is a production root
/// (§17.23).
pub const CONDITIONAL: [&str; 1] = ["compiler"];
/// Text no file of the corpus may hold: a network address or a repository.
/// Compared on the lower-cased file, so a capitalized host is found too.
const OUTSIDE: [&str; 4] = ["://", "www.", "github", "uor-foundation/"];

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    spec: String,
    id: String,
    kind: Vec<Kind>,
    member: Vec<Member>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Kind {
    id: String,
    #[serde(default)]
    precondition: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Member {
    id: String,
    kind: String,
    root: String,
    targets: Vec<String>,
    effects: Vec<String>,
    contract: String,
}

/// What a production root of the project declares (§17.13): its targets and
/// its effects, each as the declaration orders them.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Declared {
    targets: Vec<String>,
    effects: Vec<String>,
}

/// Every production root of the corpus project with its declaration, read
/// from the eligibility analysis of the checked project, never from the
/// manifest.
fn declared_roots(project: &P) -> BTreeMap<String, Declared> {
    let checked = support::checked_project(project);
    lexlean::production::lower::roots(&checked)
        .expect("the corpus project's roots")
        .into_iter()
        .map(|root| {
            (
                root.report.root.clone(),
                Declared {
                    targets: root
                        .report
                        .targets
                        .iter()
                        .map(|row| row.target.clone())
                        .collect(),
                    effects: root.report.declared_effects.clone(),
                },
            )
        })
        .collect()
}

/// PC-01: read a manifest against the project's production roots, refusing
/// every defect the register names.
fn read(text: &str, roots: &BTreeMap<String, Declared>) -> Result<Manifest, String> {
    let manifest: Manifest =
        toml::from_str(text).map_err(|error| format!("the manifest does not parse: {error}"))?;
    if manifest.spec != SPEC {
        return Err(format!(
            "the manifest's spec is `{}`, not `{SPEC}`",
            manifest.spec
        ));
    }
    if manifest.id != CORPUS_ID {
        return Err(format!(
            "the corpus is `{}`, not `{CORPUS_ID}`",
            manifest.id
        ));
    }
    let kinds: Vec<&str> = manifest.kind.iter().map(|kind| kind.id.as_str()).collect();
    if kinds != KINDS {
        return Err(format!(
            "the kinds are {kinds:?}, not the registered {KINDS:?}"
        ));
    }
    let mut ids = BTreeSet::new();
    for member in &manifest.member {
        let Some((kind, name)) = member.id.split_once('/') else {
            return Err(format!("member `{}` is not `<kind>/<name>`", member.id));
        };
        if kind != member.kind {
            return Err(format!(
                "member `{}` is of kind `{}`, not of its identifier's kind",
                member.id, member.kind
            ));
        }
        if !KINDS.contains(&kind) {
            return Err(format!(
                "member `{}` has the unregistered kind `{kind}`",
                member.id
            ));
        }
        if name.is_empty()
            || !name
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        {
            return Err(format!(
                "member `{}`: the name is not lower-case ASCII words joined by `_`",
                member.id
            ));
        }
        if !ids.insert(member.id.clone()) {
            return Err(format!("member `{}` is declared twice", member.id));
        }
        if member.contract.trim().is_empty() {
            return Err(format!("member `{}` states no contract", member.id));
        }
        let Some(declared) = roots.get(&member.root) else {
            return Err(format!(
                "member `{}`: `{}` is not a production root of the project",
                member.id, member.root
            ));
        };
        if declared.targets != member.targets {
            return Err(format!(
                "member `{}`: the root declares targets {:?}, the row states {:?}",
                member.id, declared.targets, member.targets
            ));
        }
        if declared.effects != member.effects {
            return Err(format!(
                "member `{}`: the root declares effects {:?}, the row states {:?}",
                member.id, declared.effects, member.effects
            ));
        }
    }
    for kind in &manifest.kind {
        let members = manifest
            .member
            .iter()
            .filter(|member| member.kind == kind.id)
            .count();
        match (&kind.precondition, members) {
            (Some(precondition), 0) if !precondition.trim().is_empty() => {}
            (Some(_), 0) => {
                return Err(format!("kind `{}` states an empty precondition", kind.id));
            }
            (Some(_), _) => {
                return Err(format!(
                    "kind `{}` states a precondition and has a member",
                    kind.id
                ));
            }
            (None, 0) => {
                return Err(format!(
                    "kind `{}` has no member and states no precondition",
                    kind.id
                ));
            }
            (None, _) => {}
        }
    }
    let conditional: Vec<&str> = manifest
        .kind
        .iter()
        .filter(|kind| kind.precondition.is_some())
        .map(|kind| kind.id.as_str())
        .collect();
    if conditional != CONDITIONAL {
        return Err(format!(
            "the kinds with a precondition are {conditional:?}, not the registered {CONDITIONAL:?}"
        ));
    }
    let mut seen_roots = BTreeSet::new();
    for member in &manifest.member {
        if !seen_roots.insert(member.root.as_str()) {
            return Err(format!("root `{}` is the root of two members", member.root));
        }
    }
    Ok(manifest)
}

/// Every file of the corpus by project-relative path, as text: the sources,
/// the configuration, the manifest, the glossary, the Lake files, and the
/// lock. Build output and oracles are derived from these and are not read.
fn corpus_files(project: &P) -> BTreeMap<String, String> {
    let mut files = BTreeMap::new();
    for entry in walkdir::WalkDir::new(project.root.as_std_path())
        .into_iter()
        .flatten()
    {
        if !entry.file_type().is_file() {
            continue;
        }
        let relative = entry
            .path()
            .strip_prefix(project.root.as_std_path())
            .expect("under the project")
            .to_string_lossy()
            .replace('\\', "/");
        if relative.starts_with(".lexlean/")
            || relative.starts_with(".lake/")
            || relative.starts_with("expected/")
        {
            continue;
        }
        let bytes = std::fs::read(entry.path()).expect("a corpus file reads");
        files.insert(relative, String::from_utf8_lossy(&bytes).into_owned());
    }
    files
}

/// PC-02, C1: the corpus reaches nothing outside LexLean. A member is
/// specific to a consumer when its project depends on another project or
/// when the corpus names one; both are refused structurally, so that the
/// check itself names no consumer.
fn consumer_free(files: &BTreeMap<String, String>) -> Result<(), String> {
    let config = files
        .get("lexlean.toml")
        .ok_or_else(|| "the corpus has no lexlean.toml".to_owned())?;
    let config: toml::Value =
        toml::from_str(config).map_err(|error| format!("lexlean.toml does not parse: {error}"))?;
    if config.get("source_package").is_some() {
        return Err("lexlean.toml declares a source package".to_owned());
    }
    for source in config
        .get("lexicon_source")
        .and_then(toml::Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or_default()
    {
        let kind = source
            .get("kind")
            .and_then(toml::Value::as_str)
            .unwrap_or_default();
        let path = source
            .get("path")
            .and_then(toml::Value::as_str)
            .unwrap_or_default();
        let inside = kind == "builtin"
            || (kind == "path"
                && !path.is_empty()
                && !path.starts_with('/')
                && !path.contains("..")
                && !path.contains(':'));
        if !inside {
            return Err(format!(
                "lexlean.toml declares the lexicon source `{}` of kind `{kind}`, which is outside the project",
                source.get("package").and_then(toml::Value::as_str).unwrap_or_default()
            ));
        }
    }
    for (path, text) in files {
        let lowered = text.to_ascii_lowercase();
        for needle in OUTSIDE {
            if lowered.contains(needle) {
                return Err(format!(
                    "{path} holds `{needle}`: a network address or a repository name"
                ));
            }
        }
    }
    Ok(())
}

/// The crate name a member's package is exported under.
fn crate_name(member: &str) -> String {
    format!("corpus_{}", member.replace(['/', '-'], "_"))
}

pub(crate) fn run(id: &str) {
    match id {
        "PC-01" => {
            let project = P::copy_example(EXAMPLE);
            let text = std::fs::read_to_string(project.root.join(MANIFEST).as_std_path())
                .expect("the corpus manifest");
            let roots = declared_roots(&project);
            let manifest = read(&text, &roots).unwrap_or_else(|why| panic!("PC-01: {why}"));
            for kind in KINDS {
                let members = manifest.member.iter().filter(|m| m.kind == kind).count();
                assert_eq!(
                    members > 0,
                    !CONDITIONAL.contains(&kind),
                    "PC-01: kind `{kind}` has a member exactly when it states no precondition"
                );
            }
            // The specification names the corpus and every kind where a
            // criterion is told how to quantify over it (§28.7).
            let spec = std::fs::read_to_string(support::repo_root().join("SPEC.md").as_std_path())
                .expect("SPEC.md");
            let section = spec
                .split("### 28.7 ")
                .nth(1)
                .and_then(|rest| rest.split("\n## ").next())
                .expect("§28.7 exists");
            assert!(
                section.contains(&format!("`{CORPUS_ID}`")),
                "§28.7 names the corpus `{CORPUS_ID}`"
            );
            for kind in KINDS {
                assert!(
                    section.contains(&format!("`{kind}`")),
                    "§28.7 names the kind `{kind}`"
                );
            }
            // Armed: each registered defect is refused, naming itself.
            let first_member = text.find("[[member]]").expect("a member row");
            let long_folds_row = text
                .find("id = \"long-folds/moments\"")
                .expect("the long-folds member");
            let long_folds_start = text[..long_folds_row]
                .rfind("[[member]]")
                .expect("the row's header");
            let long_folds_end = text[long_folds_row..]
                .find("\n[[member]]")
                .map_or(text.len(), |offset| long_folds_row + offset);
            let plants: [(&str, String, &str); 6] = [
                (
                    "an unknown field",
                    format!("{text}\n[[member]]\nid = \"recursion/extra\"\nkind = \"recursion\"\nroot = \"Corpus.Recursion.bisect\"\ntargets = []\neffects = []\ncontract = \"x\"\nowner = \"x\"\n"),
                    "owner",
                ),
                (
                    "an unregistered kind",
                    text.replacen("id = \"long-folds\"", "id = \"long-fold\"", 1),
                    "not the registered",
                ),
                (
                    "a member that is not a production root",
                    text.replacen(
                        "root = \"Corpus.IntKernels.requantize\"",
                        "root = \"Corpus.IntKernels.clamp\"",
                        1,
                    ),
                    "is not a production root",
                ),
                (
                    "a member declared with other targets",
                    text.replacen(
                        "root = \"Corpus.IntKernels.requantize\"\ntargets = [\"rust-core\", \"rust-std\"]",
                        "root = \"Corpus.IntKernels.requantize\"\ntargets = [\"rust-std\"]",
                        1,
                    ),
                    "declares targets",
                ),
                (
                    "a kind without a member",
                    format!("{}{}", &text[..long_folds_start], &text[long_folds_end..]),
                    "has no member and states no precondition",
                ),
                (
                    "a member of the compiler kind",
                    format!(
                        "{}[[member]]\nid = \"compiler/self\"\nkind = \"compiler\"\nroot = \"Corpus.Recursion.bisect\"\ntargets = [\"rust-core\", \"rust-std\"]\neffects = [\"overflow\"]\ncontract = \"x\"\n\n{}",
                        &text[..first_member],
                        &text[first_member..]
                    ),
                    "states a precondition and has a member",
                ),
            ];
            for (defect, planted, names) in &plants {
                assert_ne!(planted, &text, "the plant of {defect} changes the manifest");
                let why = read(planted, &roots)
                    .err()
                    .unwrap_or_else(|| panic!("PC-01: a manifest with {defect} is refused"));
                assert!(
                    why.contains(names),
                    "PC-01: {defect} is refused naming it (`{names}`): {why}"
                );
            }
        }
        "PC-02" => {
            let project = P::copy_example(EXAMPLE);
            let files = corpus_files(&project);
            assert!(files.contains_key(MANIFEST) && files.keys().any(|k| k.starts_with("src/")));
            consumer_free(&files).unwrap_or_else(|why| panic!("PC-02: {why}"));
            // Armed: each way of reaching outside LexLean is refused.
            let config = files["lexlean.toml"].clone();
            let source = files
                .keys()
                .find(|key| key.starts_with("src/"))
                .expect("a source")
                .clone();
            let plants: [(&str, &str, String, &str); 4] = [
                (
                    "a source package",
                    "lexlean.toml",
                    format!(
                        "{config}\n[[source_package]]\nname = \"other\"\nversion = \"1.0.0\"\n"
                    ),
                    "source package",
                ),
                (
                    "a lexicon source outside the project",
                    "lexlean.toml",
                    config.replacen("kind = \"builtin\"", "kind = \"git\"", 1),
                    "outside the project",
                ),
                (
                    "a network address",
                    source.as_str(),
                    format!("{}\n% https://example.test/weights\n", files[&source]),
                    "network address",
                ),
                (
                    "a repository name",
                    MANIFEST,
                    format!("{}\n# UOR-Foundation/other\n", files[MANIFEST]),
                    "repository",
                ),
            ];
            for (defect, path, planted, names) in &plants {
                let mut altered = files.clone();
                assert_ne!(
                    &altered[*path], planted,
                    "the plant of {defect} changes {path}"
                );
                altered.insert((*path).to_owned(), planted.clone());
                let why = consumer_free(&altered)
                    .err()
                    .unwrap_or_else(|| panic!("PC-02: a corpus with {defect} is refused"));
                assert!(
                    why.contains(names),
                    "PC-02: {defect} is refused naming it: {why}"
                );
            }
            // The export needs the pinned toolchain (§8.3).
            if !support::lean_backed("PC-02") {
                return;
            }
            let roots = declared_roots(&project);
            let manifest =
                read(&files[MANIFEST], &roots).unwrap_or_else(|why| panic!("PC-02: {why}"));
            let application = Application {
                roots: manifest
                    .member
                    .iter()
                    .map(|member| AppRoot {
                        root: member.root.clone(),
                        name: crate_name(&member.id),
                        version: "1.0.0".to_owned(),
                        targets: member.targets.clone(),
                    })
                    .collect(),
            };
            let (output, report) =
                production_consumer::export(&project.root.join("lexlean.toml"), &application, &[])
                    .unwrap_or_else(|error| panic!("PC-02: the corpus exports: {error}"));
            let shipped: usize = manifest.member.iter().map(|m| m.targets.len()).sum();
            let outputs: usize = output.result.roots.iter().map(|r| r.outputs.len()).sum();
            assert_eq!(outputs, shipped, "PC-02: one output per member and target");
            let exported: BTreeSet<&str> = output
                .result
                .roots
                .iter()
                .map(|root| root.root.as_str())
                .collect();
            let members: BTreeSet<&str> = manifest.member.iter().map(|m| m.root.as_str()).collect();
            assert_eq!(
                exported, members,
                "PC-02: every member and nothing else is exported"
            );
            for root in &output.result.roots {
                let member = manifest
                    .member
                    .iter()
                    .find(|m| m.root == root.root)
                    .expect("a member");
                let targets: Vec<&str> =
                    root.outputs.iter().map(|o| o.target.id.as_str()).collect();
                assert_eq!(
                    targets,
                    member
                        .targets
                        .iter()
                        .map(String::as_str)
                        .collect::<Vec<_>>(),
                    "PC-02: `{}` is exported in every target it declares",
                    member.id
                );
                for output in &root.outputs {
                    assert_eq!(
                        output.certification.status,
                        CertificationStatus::KernelChecked,
                        "PC-02: `{}` on `{}` is kernel-checked",
                        member.id,
                        output.target.id
                    );
                }
            }
            assert_eq!(report.status, ReplayStatus::BindingsVerified);
            assert_eq!(report.outputs as usize, shipped);
        }
        _ => panic!("no performance-corpus case is wired for {id}"),
    }
}
