//! The `production-export` suite: PX-01..PX-12, and the `production` CLI of
//! CL-23 (SPEC.md §24.7).

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use camino::{Utf8Path, Utf8PathBuf};
use lexlean::api::production::{
    self, CertificateRole, CertificationStatus, Evidence, EvidenceKind, Expectation, Optimality,
    PackageName, ProductionRequest, ProductionResult, RootRequest, TargetRef, REQUEST_SPEC,
};
use lexlean::{Engine, LexLeanError, Sha256Digest};

use crate::production_consumer;
use crate::production_export::{self as harness, Exported, GNAF_ROOT};
use crate::support::{self, P};

/// The modules `lib.rs` declares `#[doc(hidden)]`: the unstable internals.
fn hidden_modules() -> BTreeSet<String> {
    let lib = std::fs::read_to_string(
        support::repo_root()
            .join("crates/lexlean/src/lib.rs")
            .as_std_path(),
    )
    .expect("lib.rs");
    let mut hidden = BTreeSet::new();
    let mut previous_hidden = false;
    for line in lib.lines() {
        let trimmed = line.trim();
        if previous_hidden {
            if let Some(module) = trimmed.strip_prefix("pub mod ") {
                hidden.insert(module.trim_end_matches(';').to_owned());
            }
        }
        previous_hidden = trimmed == "#[doc(hidden)]";
    }
    hidden
}

/// Every path the source names into `lexlean` that is not the stable
/// surface: `lexlean::api::production`, or a crate-root type.
fn unstable_paths(source: &str, hidden: &BTreeSet<String>) -> Vec<String> {
    let mut found = Vec::new();
    for (offset, _) in source.match_indices("lexlean::") {
        let rest = &source[offset + "lexlean::".len()..];
        if let Some(group) = rest.strip_prefix('{') {
            let names = group.split('}').next().unwrap_or_default();
            for name in names
                .split(',')
                .map(str::trim)
                .filter(|name| !name.is_empty())
            {
                let head = name.split("::").next().unwrap_or_default();
                if hidden.contains(head) || (head == "api" && !name.starts_with("api::production"))
                {
                    found.push(format!("lexlean::{{{name}}}"));
                }
            }
            continue;
        }
        let segment: String = rest
            .chars()
            .take_while(|character| character.is_alphanumeric() || *character == '_')
            .collect();
        let stable = if segment == "api" {
            rest.starts_with("api::production")
        } else {
            !hidden.contains(&segment) && segment.chars().next().is_some_and(char::is_uppercase)
        };
        if !stable {
            found.push(format!("lexlean::{segment}"));
        }
    }
    found
}

fn digests() -> BTreeMap<String, Sha256Digest> {
    production::targets()
        .expect("targets")
        .into_iter()
        .map(|descriptor| (descriptor.id, descriptor.digest))
        .collect()
}

/// A request for `roots` of the production example with the registered
/// digests.
fn request_for(roots: &[(&str, &str, &[&str])]) -> ProductionRequest {
    let digests = digests();
    ProductionRequest {
        spec: REQUEST_SPEC.to_owned(),
        roots: roots
            .iter()
            .map(|(root, name, targets)| RootRequest {
                root: (*root).to_owned(),
                package: PackageName {
                    name: (*name).to_owned(),
                    version: "1.0.0".to_owned(),
                },
                targets: targets
                    .iter()
                    .map(|id| TargetRef {
                        id: (*id).to_owned(),
                        digest: digests.get(*id).copied().unwrap_or(Sha256Digest([0; 32])),
                    })
                    .collect(),
            })
            .collect(),
        expect: Expectation::default(),
        evidence: Vec::new(),
    }
}

fn engine(project: &P) -> Engine {
    Engine::load(&project.root.join("lexlean.toml")).expect("the project loads")
}

/// Export and require a failure with `code` that published nothing: no
/// build, verification, or production directory exists afterwards, so no
/// Lean process ran.
fn refused(project: &P, request: &ProductionRequest, code: &str) -> LexLeanError {
    let error = production::build(&engine(project), request)
        .err()
        .unwrap_or_else(|| panic!("the request is refused with {code}"));
    support::expect_code(&error, code);
    for area in ["build", "verified", "production"] {
        assert!(
            !project
                .root
                .join(".lexlean")
                .join(area)
                .as_std_path()
                .exists(),
            "{code}: `.lexlean/{area}` was published"
        );
    }
    error
}

fn sha(bytes: &[u8]) -> Sha256Digest {
    Sha256Digest::of(bytes)
}

/// The production example exported a second time, from another absolute
/// directory and another working directory, through the CLI, with a git
/// sentinel first on PATH and an unreachable proxy in the environment.
pub struct Offline {
    /// The project copy.
    pub project: P,
    /// The CLI's exit code, stdout, and stderr.
    pub run: (i32, String, String),
    /// Whether the sentinel ran.
    pub sentinel_ran: bool,
    /// The sentinel's directory, kept alive.
    #[allow(dead_code)]
    sentinel: tempfile::TempDir,
}

/// The offline second export, shared by PX-08, PX-10, and CL-23.
///
/// # Panics
///
/// On a host without a POSIX shell for the sentinel.
pub fn offline() -> &'static Offline {
    static OFFLINE: OnceLock<Offline> = OnceLock::new();
    OFFLINE.get_or_init(|| {
        let project = P::copy_example("production");
        let first = harness::exported("production");
        let gnaf = first
            .gnaf
            .as_ref()
            .expect("the production export offers evidence");
        project.write(
            harness::GNAF_REQUEST,
            std::str::from_utf8(&gnaf.request).expect("utf8"),
        );
        project.write(
            harness::GNAF_CERTIFICATE,
            std::str::from_utf8(&gnaf.certificate).expect("utf8"),
        );
        let (_, pins) =
            production_consumer::setup(&project.root.join("lexlean.toml")).expect("setup");
        let request = production_consumer::request(
            &harness::application("production"),
            &pins,
            &[production_consumer::AppEvidence {
                root: GNAF_ROOT.to_owned(),
                request: harness::GNAF_REQUEST.to_owned(),
                certificate: harness::GNAF_CERTIFICATE.to_owned(),
                certificate_id: gnaf.certificate_id,
            }],
        );
        let request_file = project.root.join("request.json");
        std::fs::write(request_file.as_std_path(), request.to_json()).expect("request file");
        let sentinel = tempfile::Builder::new()
            .prefix("lexlean-git-sentinel-")
            .tempdir()
            .expect("tempdir");
        let marker = sentinel.path().join("ran");
        let script = sentinel.path().join("git");
        std::fs::write(
            &script,
            format!("#!/bin/sh\necho \"$@\" >> '{}'\nexit 1\n", marker.display()),
        )
        .expect("sentinel");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755))
                .expect("executable");
        }
        let elsewhere = tempfile::Builder::new()
            .prefix("lexlean-cwd-")
            .tempdir()
            .expect("tempdir");
        let cwd = Utf8Path::from_path(elsewhere.path()).expect("utf8");
        let path = format!(
            "{}:{}",
            sentinel.path().display(),
            std::env::var("PATH").unwrap_or_default()
        );
        let unreachable = "http://127.0.0.1:9";
        let config = project.root.join("lexlean.toml");
        let run = support::with_env(
            &[
                ("PATH", Some(path.as_str())),
                ("HTTP_PROXY", Some(unreachable)),
                ("HTTPS_PROXY", Some(unreachable)),
                ("ALL_PROXY", Some(unreachable)),
                ("http_proxy", Some(unreachable)),
                ("https_proxy", Some(unreachable)),
            ],
            || {
                support::cli_in(
                    cwd,
                    &[
                        "--project",
                        config.as_str(),
                        "--diagnostic-format",
                        "json",
                        "production",
                        "build",
                        "--request",
                        request_file.as_str(),
                    ],
                )
            },
        );
        Offline {
            sentinel_ran: marker.exists(),
            project,
            run,
            sentinel,
        }
    })
}

fn result_of(bundle: &Utf8Path) -> ProductionResult {
    serde_json::from_slice(
        &std::fs::read(bundle.join("result.json").as_std_path()).expect("result.json"),
    )
    .expect("a result")
}

/// A fresh copy of a bundle to tamper with.
fn bundle_copy(exported: &Exported) -> (tempfile::TempDir, Utf8PathBuf) {
    let dir = tempfile::Builder::new()
        .prefix("lexlean-bundle-")
        .tempdir()
        .expect("tempdir");
    let root = Utf8PathBuf::from_path_buf(dir.path().join("bundle")).expect("utf8");
    for (relative, bytes) in harness::tree(&exported.bundle()) {
        let path = root.join(&relative);
        std::fs::create_dir_all(path.parent().expect("parent").as_std_path()).expect("mkdir");
        std::fs::write(path.as_std_path(), bytes).expect("write");
    }
    (dir, root)
}

fn flip_byte(path: &Utf8Path, needle: u8, replacement: u8) {
    let mut bytes = std::fs::read(path.as_std_path()).expect("read");
    let at = bytes
        .iter()
        .rposition(|byte| *byte == needle)
        .unwrap_or_else(|| panic!("{path} holds {}", needle as char));
    bytes[at] = replacement;
    std::fs::write(path.as_std_path(), bytes).expect("write");
}

/// Rewrite result.json through `edit`, recomputing the result identity, so
/// that only the cross-checks can catch the change.
fn rewrite_result(bundle: &Utf8Path, edit: impl FnOnce(&mut ProductionResult)) {
    let mut result = result_of(bundle);
    edit(&mut result);
    result.result_id = result.recomputed_id();
    std::fs::write(bundle.join("result.json").as_std_path(), result.to_json()).expect("write");
}

/// Change an attestation fact with both content identities and every
/// enclosing file binding refreshed, so only consistency checks reject it.
fn rewrite_attestation(bundle: &Utf8Path, path: &[&str], value: Option<serde_json::Value>) {
    let result = result_of(bundle);
    let file = bundle.join(&result.verification.attestation.path);
    let mut body: serde_json::Value =
        serde_json::from_slice(&std::fs::read(file.as_std_path()).expect("attestation"))
            .expect("attestation JSON");
    body.as_object_mut()
        .expect("object")
        .remove("attestation_id");
    let (field, parents) = path.split_last().expect("a field path");
    let mut object = body.as_object_mut().expect("object");
    for parent in parents {
        object = object
            .get_mut(*parent)
            .and_then(serde_json::Value::as_object_mut)
            .expect("attestation parent object");
    }
    if let Some(value) = value {
        object.insert((*field).to_owned(), value);
    } else {
        object.remove(*field);
    }
    let canonical_body = lexlean::artifact::canonical_json::Json::parse(&harness::canonical(&body))
        .expect("canonical body");
    let id = lexlean::artifact::content_id::attestation_id(&canonical_body.to_canonical_string());
    body["attestation_id"] = serde_json::json!(id.to_hex());
    let bytes = harness::canonical(&body);
    std::fs::write(file.as_std_path(), &bytes).expect("write attestation");
    rewrite_result(bundle, |result| {
        result.verification.attestation_id = id;
        result.verification.attestation.byte_length = u64::try_from(bytes.len()).expect("length");
        result.verification.attestation.sha256 = sha(&bytes);
    });
}

/// Refresh the input's result and attestation bindings after changing one
/// scalar pin, so its disagreement with the producing toolchain is exposed.
fn rewrite_compiler_input_identity(bundle: &Utf8Path, field: &str, value: &str) {
    let result = result_of(bundle);
    let file = bundle.join(&result.verification.compiler_input.path);
    let mut input: serde_json::Value =
        serde_json::from_slice(&std::fs::read(file.as_std_path()).expect("compiler input"))
            .expect("compiler input JSON");
    input["lean"][field] = serde_json::json!(value);
    let bytes = harness::canonical(&input);
    std::fs::write(file.as_std_path(), &bytes).expect("write compiler input");
    rewrite_attestation(
        bundle,
        &["compiler_input"],
        Some(serde_json::json!({"byte_length": bytes.len(), "sha256": sha(&bytes).to_hex()})),
    );
    rewrite_result(bundle, |result| {
        result.verification.compiler_input.byte_length =
            u64::try_from(bytes.len()).expect("length");
        result.verification.compiler_input.sha256 = sha(&bytes);
    });
}

fn replay_refused(bundle: &Utf8Path, code: &str, what: &str) {
    match production::verify_artifact(bundle) {
        Ok(report) => panic!("{what}: the tampered bundle replayed: {report:?}"),
        Err(error) => {
            assert!(
                error
                    .diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic.code.as_str() == code),
                "{what}: expected {code}, got {:?}",
                error.diagnostics
            );
        }
    }
}

/// The schemas of §24.7 and the values each validates.
pub const SCHEMAS: [(&str, &str); 5] = [
    ("production-request", production::REQUEST_SPEC),
    ("production-result", production::RESULT_SPEC),
    ("target-descriptor", production::TARGET_SPEC),
    ("target-descriptors", production::TARGETS_SPEC),
    ("production-replay", production::REPLAY_SPEC),
];

fn api_schema(name: &str) -> (Vec<u8>, serde_json::Value) {
    let bytes = std::fs::read(
        support::repo_root()
            .join("api/schemas")
            .join(format!("{name}.schema.json"))
            .as_std_path(),
    )
    .unwrap_or_else(|error| panic!("api/schemas/{name}.schema.json: {error}"));
    let value = serde_json::from_slice(&bytes).expect("schema JSON");
    (bytes, value)
}

fn assert_valid(name: &str, value: &serde_json::Value, what: &str) {
    let violations = crate::schema::validate(&api_schema(name).1, value);
    assert!(
        violations.is_empty(),
        "{what} does not validate against api/schemas/{name}.schema.json: {violations:?}"
    );
}

/// The recorded schema versions: `(file, spec, sha256)` rows.
fn recorded_versions() -> Vec<(String, String, String)> {
    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Row {
        schema: String,
        spec: String,
        sha256: String,
    }
    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Versions {
        version: Vec<Row>,
    }
    let text = std::fs::read_to_string(
        support::repo_root()
            .join("api/schemas/versions.toml")
            .as_std_path(),
    )
    .expect("api/schemas/versions.toml");
    let versions: Versions = toml::from_str(&text).expect("versions.toml");
    versions
        .version
        .into_iter()
        .map(|row| (row.schema, row.spec, row.sha256))
        .collect()
}

/// The schema-version gate: every schema's bytes are recorded under its
/// current spec, no spec is recorded twice, and the spec is the façade's.
///
/// # Errors
///
/// The first unmet rule.
pub fn schema_versions_hold(
    schemas: &BTreeMap<String, Vec<u8>>,
    versions: &[(String, String, String)],
) -> Result<(), String> {
    let mut specs = BTreeSet::new();
    for (_, spec, _) in versions {
        if !specs.insert(spec.clone()) {
            return Err(format!("`{spec}` is recorded twice"));
        }
    }
    for (name, current) in SCHEMAS {
        let bytes = schemas
            .get(name)
            .ok_or_else(|| format!("no schema `{name}`"))?;
        let value: serde_json::Value =
            serde_json::from_slice(bytes).map_err(|error| error.to_string())?;
        let stated = value["properties"]["spec"]["const"]
            .as_str()
            .unwrap_or_default();
        if stated != current {
            return Err(format!(
                "{name}: the schema states `{stated}`, the façade `{current}`"
            ));
        }
        let digest = sha(bytes).to_hex();
        let row = versions
            .iter()
            .find(|(schema, spec, _)| schema == name && spec == current)
            .ok_or_else(|| format!("{name}: `{current}` has no recorded version"))?;
        if row.2 != digest {
            return Err(format!(
                "{name}: the schema changed under `{current}` (recorded {}, now {digest}); a changed schema needs a new spec version",
                row.2
            ));
        }
    }
    Ok(())
}

/// Run the case for one PX ID.
#[allow(clippy::too_many_lines)]
pub fn run(id: &str) {
    match id {
        // §24.7: a public-surface consumer receives canonical Rust bound to
        // every identity of the export.
        "PX-01" => {
            let hidden = hidden_modules();
            assert!(hidden.contains("production") && hidden.len() >= 10);
            let consumer = std::fs::read_to_string(
                support::repo_root()
                    .join("crates/conformance/src/production_consumer.rs")
                    .as_std_path(),
            )
            .expect("the consumer");
            assert!(
                consumer.contains("lexlean::api::production"),
                "the consumer uses the façade"
            );
            assert_eq!(
                unstable_paths(&consumer, &hidden),
                Vec::<String>::new(),
                "the consumer names only the stable surface"
            );
            // The gate is armed: a planted reach into an internal is found.
            for planted in [
                "use lexlean::production::lower;",
                "lexlean::verify::run(",
                "use lexlean::{Engine, calculus};",
                "lexlean::api::render_build(",
            ] {
                assert!(
                    !unstable_paths(&format!("{consumer}\n{planted}\n"), &hidden).is_empty(),
                    "the stable-surface gate misses `{planted}`"
                );
            }
            // The exports need the pinned toolchain (§8.3).
            if !support::lean_backed("PX-01") {
                return;
            }
            let this_compiler = lexlean::compiler_semantics_id_for(lexlean::LANGUAGE_1_2);
            let digests = digests();
            for example in harness::EXAMPLES {
                let exported = harness::exported(example);
                let result = &exported.output.result;
                let application = harness::application(example);
                // Every package is the committed one.
                let packages = harness::tree(&exported.bundle().join("packages"));
                let expected = harness::tree(&harness::fixture_dir(example).join("packages"));
                assert_eq!(
                    packages.keys().collect::<Vec<_>>(),
                    expected.keys().collect::<Vec<_>>(),
                    "{example}: the exported package files are the committed ones"
                );
                for (relative, bytes) in &packages {
                    assert!(
                        expected.get(relative) == Some(bytes),
                        "{example}: packages/{relative} differs from the committed package"
                    );
                }
                // The bindings.
                let project = engine(&exported.project);
                let checked = project
                    .check(lexlean::CheckRequest {
                        selection: lexlean::Selection::Entrypoints,
                    })
                    .expect("check");
                assert_eq!(result.result_id, result.recomputed_id());
                assert_eq!(result.compiler.semantics, this_compiler);
                assert_eq!(result.compiler.lean_toolchain, lexlean::LEAN_TOOLCHAIN);
                assert_eq!(result.source.source_id, checked.source_id);
                assert_eq!(result.source.semantic_id, checked.semantic_id);
                assert_eq!(
                    result.source.lock_sha256,
                    sha(
                        &std::fs::read(exported.project.root.join("lexlean.lock").as_std_path())
                            .expect("lock")
                    )
                );
                let bundle = exported.bundle();
                let attestation: serde_json::Value = serde_json::from_slice(
                    &std::fs::read(bundle.join("evidence/attestation.json").as_std_path())
                        .expect("attestation"),
                )
                .expect("attestation JSON");
                let compiler_input =
                    std::fs::read(bundle.join("evidence/compiler-input.json").as_std_path())
                        .expect("compiler input");
                let preservation =
                    std::fs::read(bundle.join("evidence/preservation.json").as_std_path())
                        .expect("preservation");
                assert_eq!(
                    attestation["attestation_id"],
                    result.verification.attestation_id.to_hex()
                );
                assert_eq!(
                    attestation["compiler_input"]["sha256"],
                    sha(&compiler_input).to_hex()
                );
                assert_eq!(
                    attestation["preservation"]["sha256"],
                    sha(&preservation).to_hex()
                );
                assert_eq!(
                    result.verification.compiler_input.sha256,
                    sha(&compiler_input)
                );
                assert_eq!(result.verification.preservation.sha256, sha(&preservation));
                let shipped: usize = application
                    .roots
                    .iter()
                    .map(|root| root.targets.len())
                    .sum();
                let outputs: usize = result.roots.iter().map(|root| root.outputs.len()).sum();
                assert_eq!(
                    outputs, shipped,
                    "{example}: one output per root and target"
                );
                assert_eq!(exported.report.outputs as usize, shipped);
                for root in &result.roots {
                    let offered = example == "production" && root.root == GNAF_ROOT;
                    assert_eq!(
                        matches!(root.optimality, Optimality::NotClaimed),
                        !offered,
                        "{}: optimality is claimed exactly where evidence was offered",
                        root.root
                    );
                    for output in &root.outputs {
                        assert_eq!(Some(&output.target.digest), digests.get(&output.target.id));
                        assert_eq!(output.package.export, production::EXPORT_NAME);
                        assert_eq!(
                            output.certification.status,
                            CertificationStatus::KernelChecked
                        );
                        assert_eq!(
                            output.certification.axioms,
                            ["Classical.choice", "Quot.sound", "propext"]
                        );
                        assert_eq!(
                            output
                                .certification
                                .certificates
                                .iter()
                                .map(|certificate| certificate.role)
                                .collect::<Vec<_>>(),
                            [CertificateRole::A, CertificateRole::B, CertificateRole::E]
                        );
                        let library = std::fs::read_to_string(
                            bundle
                                .join(&output.package.directory)
                                .join("src/lib.rs")
                                .as_std_path(),
                        )
                        .expect("lib.rs");
                        let krate =
                            std::fs::read_to_string(bundle.join(&output.krate.path).as_std_path())
                                .expect("crate");
                        let wrapper = library
                            .strip_prefix(&krate)
                            .expect("the package begins with the certified crate");
                        assert_eq!(
                            wrapper.matches("pub fn ").count(),
                            1,
                            "{}: one export",
                            root.root
                        );
                        assert!(
                            wrapper.contains("pub fn run("),
                            "{}: the export is `run`: {wrapper}",
                            root.root
                        );
                        assert!(
                            wrapper.contains(&format!("{}(", output.entry.symbol)),
                            "{}: `run` calls the recorded entry {}",
                            root.root,
                            output.entry.symbol
                        );
                        let provenance: serde_json::Value = serde_json::from_slice(
                            &std::fs::read(
                                bundle
                                    .join(&output.package.directory)
                                    .join("provenance.json")
                                    .as_std_path(),
                            )
                            .expect("provenance"),
                        )
                        .expect("provenance JSON");
                        assert_eq!(provenance["compiler_semantics"], this_compiler.to_hex());
                        assert_eq!(
                            provenance["sources"],
                            serde_json::json!([result.source.semantic_id.to_hex()])
                        );
                        assert_eq!(provenance["profile"], output.target.id);
                    }
                }
            }
            // `cargo xtask check-production-export` compares the same packages.
        }
        // §24.7: a root that is not a production root is refused before Lean.
        "PX-02" => {
            let project = P::copy_example("production");
            for root in [
                "Production.Main.absent",
                "Production.Kernel.area",
                "Production.Main.quadruple_three",
                "Other.Main.checkedSum",
            ] {
                let error = refused(
                    &project,
                    &request_for(&[(root, "pkg", &["rust-std"])]),
                    "LLC0202",
                );
                assert!(
                    error.diagnostics[0].message.contains(root),
                    "the refusal names `{root}`"
                );
            }
        }
        // §24.7: targets are closed IDs with descriptor digests.
        "PX-03" => {
            let project = P::copy_example("production");
            refused(
                &project,
                &request_for(&[("Production.Main.checkedSum", "pkg", &["rust-wasm"])]),
                "LLC0203",
            );
            refused(
                &project,
                &request_for(&[("Production.Main.sumAll", "pkg", &["rust-core"])]),
                "LLC0203",
            );
            let mut stale = request_for(&[("Production.Main.checkedSum", "pkg", &["rust-std"])]);
            stale.roots[0].targets[0].digest = Sha256Digest::of(b"another descriptor");
            refused(&project, &stale, "LLC0203");
            // The digest is the framed SHA-256 of the whole canonical
            // descriptor without it, recomputed independently, so every
            // field is inside it.
            let descriptors = production::targets().expect("targets");
            assert_eq!(
                descriptors
                    .iter()
                    .map(|d| d.id.as_str())
                    .collect::<Vec<_>>(),
                ["rust-core", "rust-std"]
            );
            let recompute = |value: &serde_json::Value| {
                let mut body = value.clone();
                body.as_object_mut().expect("object").remove("digest");
                let canonical =
                    lexlean::artifact::canonical_json::Json::parse(body.to_string().as_bytes())
                        .expect("canonical")
                        .to_canonical_string();
                let mut hasher = lexlean::artifact::content_id::FramedHasher::new(
                    "lexlean-target-descriptor-v1",
                );
                hasher.frame("descriptor", canonical.as_bytes());
                hasher.finish()
            };
            for descriptor in &descriptors {
                let value = serde_json::to_value(descriptor).expect("JSON");
                assert_eq!(recompute(&value), descriptor.digest, "{}", descriptor.id);
                for field in value.as_object().expect("object").keys() {
                    if field == "digest" {
                        continue;
                    }
                    let mut changed = value.clone();
                    changed[field] = match &value[field] {
                        serde_json::Value::Bool(flag) => serde_json::Value::Bool(!flag),
                        serde_json::Value::Number(number) => {
                            serde_json::json!(number.as_u64().expect("u64") + 1)
                        }
                        serde_json::Value::String(text) => {
                            serde_json::Value::String(format!("{text}0"))
                        }
                        other => panic!("{field}: {other}"),
                    };
                    assert_ne!(
                        recompute(&changed),
                        descriptor.digest,
                        "{}: `{field}` is inside the digest",
                        descriptor.id
                    );
                }
            }
        }
        // §24.7: pins taken at setup must still hold.
        "PX-04" => {
            let project = P::copy_example("production");
            let (_, pins) =
                production_consumer::setup(&project.root.join("lexlean.toml")).expect("setup");
            let pinned = |source: bool, semantic: bool, lock: bool| {
                let mut request =
                    request_for(&[("Production.Main.checkedSum", "checked_sum", &["rust-core"])]);
                request.expect = Expectation {
                    source_id: pins.expect.source_id.filter(|_| source),
                    semantic_id: pins.expect.semantic_id.filter(|_| semantic),
                    lock_sha256: pins.expect.lock_sha256.filter(|_| lock),
                };
                request
            };
            // A source module edited after pinning.
            project.edit(
                "src/Main.lex.tex",
                r#"{"kind":"nat","value":"0"}],"function":{"module":"Kernel","name":"firstOr"}"#,
                r#"{"kind":"nat","value":"1"}],"function":{"module":"Kernel","name":"firstOr"}"#,
            );
            refused(&project, &pinned(true, false, false), "LLC0204");
            refused(&project, &pinned(false, true, false), "LLC0204");
            // The configuration edited: the lock is stale.
            project.edit(
                "lexlean.toml",
                "max_diagnostics = 256",
                "max_diagnostics = 255",
            );
            refused(&project, &pinned(false, false, true), "LLC0102");
            // Relocked, the lock is current but not the pinned one.
            project.relock();
            refused(&project, &pinned(false, false, true), "LLC0204");
        }
        // §24.7: every production refusal fixture fails through the façade.
        "PX-05" => {
            let negative = support::repo_root().join("tests/negative");
            let mut names: Vec<String> = std::fs::read_dir(negative.as_std_path())
                .expect("tests/negative")
                .flatten()
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
                .filter(|name| name.starts_with("production-"))
                .collect();
            names.sort();
            assert!(
                names.len() >= 13,
                "the production refusal fixtures: {names:?}"
            );
            for name in &names {
                let diagnostics: serde_json::Value = serde_json::from_slice(
                    &std::fs::read(
                        negative
                            .join(name)
                            .join("expected/diagnostics.json")
                            .as_std_path(),
                    )
                    .expect("diagnostics"),
                )
                .expect("JSON");
                let code = diagnostics[0]["code"].as_str().expect("a committed code");
                let project = P::negative(name);
                refused(
                    &project,
                    &request_for(&[("Fixture.Main.root", "fixture", &["rust-std"])]),
                    code,
                );
            }
        }
        // §24.7, D3: certificate evidence is re-checked, and a request cannot
        // state a claim.
        "PX-06" => {
            let project = P::copy_example("production");
            let gnaf = harness::gnaf_evidence(&project);
            project.write(
                harness::GNAF_REQUEST,
                std::str::from_utf8(&gnaf.request).expect("utf8"),
            );
            project.write(
                harness::GNAF_CERTIFICATE,
                std::str::from_utf8(&gnaf.certificate).expect("utf8"),
            );
            let offer = |root: &str, request: &str, certificate: &str, id: Sha256Digest| {
                let mut asked = request_for(&[(root, "pkg", &["rust-std"])]);
                asked.evidence = vec![Evidence {
                    root: root.to_owned(),
                    kind: EvidenceKind::Gnaf,
                    request: request.to_owned(),
                    certificate: certificate.to_owned(),
                    certificate_id: id,
                }];
                asked
            };
            // Another identity than the request pins.
            refused(
                &project,
                &offer(
                    GNAF_ROOT,
                    harness::GNAF_REQUEST,
                    harness::GNAF_CERTIFICATE,
                    sha(b"forged"),
                ),
                "LLV7101",
            );
            // The certificate altered after its identity was pinned.
            let mut altered: serde_json::Value =
                serde_json::from_slice(&gnaf.certificate).expect("certificate");
            altered["premises"][0] = serde_json::json!({"kind": "excluded", "position": 0});
            project.write("evidence/altered.json", &altered.to_string());
            refused(
                &project,
                &offer(
                    GNAF_ROOT,
                    harness::GNAF_REQUEST,
                    "evidence/altered.json",
                    gnaf.certificate_id,
                ),
                "LLV7101",
            );
            // A certificate for another universe earns a refused verdict.
            let mut foreign: lexlean::gnaf::certification::Certificate =
                serde_json::from_slice(&gnaf.certificate).expect("certificate");
            foreign.universe = Sha256Digest::of(b"another universe");
            let foreign_id =
                lexlean::gnaf::certification::certificate_id(&foreign).expect("identity");
            project.write(
                "evidence/foreign.json",
                std::str::from_utf8(&harness::canonical(&foreign)).expect("utf8"),
            );
            refused(
                &project,
                &offer(
                    GNAF_ROOT,
                    harness::GNAF_REQUEST,
                    "evidence/foreign.json",
                    foreign_id,
                ),
                "LLV7101",
            );
            // Evidence for one root offered for another.
            refused(
                &project,
                &offer(
                    "Production.Main.headOr",
                    harness::GNAF_REQUEST,
                    harness::GNAF_CERTIFICATE,
                    gnaf.certificate_id,
                ),
                "LLV7101",
            );
            // Malformed documents.
            project.write("evidence/broken.json", "{\"spec\":");
            refused(
                &project,
                &offer(
                    GNAF_ROOT,
                    "evidence/broken.json",
                    harness::GNAF_CERTIFICATE,
                    gnaf.certificate_id,
                ),
                "LLB6006",
            );
            refused(
                &project,
                &offer(
                    GNAF_ROOT,
                    harness::GNAF_REQUEST,
                    "evidence/broken.json",
                    gnaf.certificate_id,
                ),
                "LLB6007",
            );
            // No claim is representable in a request.
            let valid = request_for(&[(GNAF_ROOT, "pkg", &["rust-std"])]).to_json();
            let valid: serde_json::Value = serde_json::from_slice(&valid).expect("JSON");
            for field in [
                "certification",
                "optimality",
                "preservation",
                "certificates",
                "lean",
                "program",
                "crate",
            ] {
                for nested in [false, true] {
                    let mut forged = valid.clone();
                    let target = if nested {
                        &mut forged["roots"][0]
                    } else {
                        &mut forged
                    };
                    target[field] = serde_json::json!("kernel-checked");
                    let error = ProductionRequest::from_json(forged.to_string().as_bytes())
                        .err()
                        .unwrap_or_else(|| panic!("a request with `{field}` deserialized"));
                    support::expect_code(&error, "LLC0201");
                }
            }
            if !support::lean_backed("PX-06") {
                return;
            }
            // Evidence that binds the root is recorded with the recomputed
            // verdict.
            let exported = harness::exported("production");
            let root = exported
                .output
                .result
                .roots
                .iter()
                .find(|root| root.root == GNAF_ROOT)
                .expect("the root with evidence");
            let recorded = exported.gnaf.as_ref().expect("evidence");
            match &root.optimality {
                Optimality::Gnaf(gnaf) => {
                    assert_eq!(gnaf.certificate_id, recorded.certificate_id);
                    assert_eq!(gnaf.verdict, recorded.verdict);
                    assert_eq!(gnaf.verdict["kind"], "certified");
                }
                Optimality::NotClaimed => panic!("the evidence was not recorded"),
            }
        }
        // §24.7, W5: replay without the source.
        "PX-07" => {
            if !support::lean_backed("PX-07") {
                return;
            }
            let exported = harness::exported("production");
            let report = production::verify_artifact(&exported.bundle()).expect("replays");
            assert_eq!(report, exported.report);
            assert_eq!(report.status, production::ReplayStatus::BindingsVerified);
            assert_eq!(report.not_rechecked, production::NOT_RECHECKED);
            assert_eq!(report.result_id, exported.output.result.result_id);
            let result = &exported.output.result;
            let first = &result.roots[0];
            let output = &first.outputs[0];
            let byte_tampers: Vec<(String, u8, u8)> = vec![
                (
                    format!("{}/src/lib.rs", output.package.directory),
                    b'}',
                    b')',
                ),
                (
                    format!("{}/Cargo.toml", output.package.directory),
                    b'"',
                    b'\'',
                ),
                (output.krate.path.clone(), b'}', b')'),
                (first.program.path.clone(), b'}', b']'),
                (
                    output.certification.certificates[0].file.path.clone(),
                    b'a',
                    b'b',
                ),
                (
                    output.certification.certificates[1].file.path.clone(),
                    b'a',
                    b'b',
                ),
                (
                    output.certification.certificates[2].file.path.clone(),
                    b'a',
                    b'b',
                ),
                ("evidence/attestation.json".to_owned(), b'a', b'b'),
                ("evidence/compiler-input.json".to_owned(), b'a', b'b'),
                ("evidence/preservation.json".to_owned(), b'a', b'b'),
                ("result.json".to_owned(), b'a', b'b'),
            ];
            for (relative, needle, replacement) in byte_tampers {
                let (_dir, bundle) = bundle_copy(exported);
                flip_byte(&bundle.join(&relative), needle, replacement);
                replay_refused(&bundle, "LLV7102", &relative);
            }
            type Edit = Box<dyn Fn(&mut ProductionResult)>;
            let gnaf_value = result
                .roots
                .iter()
                .find(|root| root.root == GNAF_ROOT)
                .expect("evidence root")
                .optimality
                .clone();
            let field_tampers: Vec<(&str, Edit)> = vec![
                (
                    "semantic ID",
                    Box::new(|r| r.source.semantic_id = sha(b"x")),
                ),
                ("source ID", Box::new(|r| r.source.source_id = sha(b"x"))),
                (
                    "attestation ID",
                    Box::new(|r| r.verification.attestation_id = sha(b"x")),
                ),
                (
                    "build ID",
                    Box::new(|r| r.verification.build_id = sha(b"x")),
                ),
                (
                    "compiler semantics",
                    Box::new(|r| r.compiler.semantics = sha(b"x")),
                ),
                (
                    "compiler version",
                    Box::new(|r| r.compiler.version.push('x')),
                ),
                (
                    "target digest",
                    Box::new(|r| r.roots[0].outputs[0].target.digest = sha(b"x")),
                ),
                (
                    "entry",
                    Box::new(|r| r.roots[0].outputs[0].entry.function += 1),
                ),
                ("root index", Box::new(|r| r.roots[0].index += 1)),
                (
                    "package name",
                    Box::new(|r| r.roots[0].outputs[0].package.name.push('x')),
                ),
                (
                    "export name",
                    Box::new(|r| r.roots[0].outputs[0].package.export = "root".to_owned()),
                ),
                (
                    "certificate theorem",
                    Box::new(|r| {
                        r.roots[0].outputs[0].certification.certificates[1]
                            .theorem
                            .push('x')
                    }),
                ),
                (
                    "certificate order",
                    Box::new(|r| r.roots[0].outputs[0].certification.certificates.swap(1, 2)),
                ),
                (
                    "axioms",
                    Box::new(|r| {
                        r.roots[0].outputs[0]
                            .certification
                            .axioms
                            .pop()
                            .map(drop)
                            .unwrap_or(())
                    }),
                ),
                (
                    "optimality claimed without evidence",
                    Box::new(move |r| {
                        let target = r
                            .roots
                            .iter_mut()
                            .find(|root| root.root != GNAF_ROOT)
                            .expect("a root without evidence");
                        target.optimality = gnaf_value.clone();
                    }),
                ),
                (
                    "verdict",
                    Box::new(|r| {
                        for root in &mut r.roots {
                            if let Optimality::Gnaf(gnaf) = &mut root.optimality {
                                gnaf.verdict["result"]["members"] = serde_json::json!([0]);
                            }
                        }
                    }),
                ),
            ];
            for (what, edit) in field_tampers {
                let (_dir, bundle) = bundle_copy(exported);
                rewrite_result(&bundle, edit);
                replay_refused(&bundle, "LLV7102", what);
            }
            let attestation_tampers: Vec<(&[&str], Option<serde_json::Value>)> = vec![
                (&["lexlean", "compiler_semantics"], None),
                (
                    &["lexlean", "compiler_semantics"],
                    Some(serde_json::json!("not a digest")),
                ),
                (
                    &["lexlean", "compiler_semantics"],
                    Some(serde_json::json!(
                        sha(b"foreign compiler semantics").to_hex()
                    )),
                ),
                (&["lexlean", "version"], Some(serde_json::json!("0.0.1"))),
                (&["toolchain"], None),
                (&["toolchain"], Some(serde_json::json!(7))),
                (&["toolchain", "lean"], None),
                (&["toolchain", "lean"], Some(serde_json::json!(7))),
                (&["toolchain", "lean", "version_output"], None),
                (
                    &["toolchain", "lean", "version_output"],
                    Some(serde_json::json!("unrecognized banner 4.32.1")),
                ),
                (
                    &["toolchain", "lean", "version_output"],
                    Some(serde_json::json!("Lean (version 4.32.10, release)")),
                ),
                (
                    &["toolchain", "lean", "version_output"],
                    Some(serde_json::json!(
                        "Lean (version 4.31.0, built with 4.32.1)"
                    )),
                ),
                (&["toolchain", "lake"], None),
                (&["toolchain", "lake"], Some(serde_json::json!(7))),
                (&["toolchain", "lake", "version_output"], None),
                (
                    &["toolchain", "lake", "version_output"],
                    Some(serde_json::json!("unrecognized banner 4.32.1")),
                ),
                (
                    &["toolchain", "lake", "version_output"],
                    Some(serde_json::json!(
                        "Lake version 5.0.0 (Lean version 4.32.10)"
                    )),
                ),
                (
                    &["toolchain", "lake", "version_output"],
                    Some(serde_json::json!(
                        "Lake version 4.32.1 (Lean version 4.31.0)"
                    )),
                ),
                (&["spec"], None),
                (&["spec"], Some(serde_json::json!("lexlean/attestation/1"))),
                (&["status"], None),
                (&["status"], Some(serde_json::json!("unverified"))),
            ];
            for (path, value) in attestation_tampers {
                let what = format!("hash-correct attestation {} = {value:?}", path.join("."));
                let (_dir, bundle) = bundle_copy(exported);
                rewrite_attestation(&bundle, path, value);
                replay_refused(&bundle, "LLV7102", &what);
            }
            for (field, value) in [
                ("version", "4.31.0"),
                ("githash", "0000000000000000000000000000000000000000"),
            ] {
                let (_dir, bundle) = bundle_copy(exported);
                rewrite_compiler_input_identity(&bundle, field, value);
                replay_refused(
                    &bundle,
                    "LLV7102",
                    &format!("hash-correct compiler input Lean {field}"),
                );
            }
            // Historical crate versions remain replayable when the result
            // and its producing attestation state the same version.
            let (_dir, bundle) = bundle_copy(exported);
            rewrite_attestation(
                &bundle,
                &["lexlean", "version"],
                Some(serde_json::json!("0.0.1")),
            );
            rewrite_result(&bundle, |result| {
                result.compiler.version = "0.0.1".to_owned()
            });
            production::verify_artifact(&bundle).expect("a self-consistent historical version");
            // A file added, a file removed.
            let (_dir, bundle) = bundle_copy(exported);
            std::fs::write(bundle.join("evidence/extra.txt").as_std_path(), b"x").expect("write");
            replay_refused(&bundle, "LLV7102", "an added file");
            let (_dir, bundle) = bundle_copy(exported);
            std::fs::remove_file(bundle.join(&output.krate.path).as_std_path()).expect("remove");
            replay_refused(&bundle, "LLV7102", "a removed file");
            if std::path::Path::new("evidence\\attestation.json")
                .components()
                .count()
                == 1
            {
                let (_dir, bundle) = bundle_copy(exported);
                std::fs::copy(
                    bundle.join("evidence/attestation.json"),
                    bundle.join("evidence\\attestation.json"),
                )
                .expect("copy alias");
                replay_refused(&bundle, "LLV7102", "an added literal-backslash alias");
                let (_dir, bundle) = bundle_copy(exported);
                std::fs::rename(
                    bundle.join("evidence/attestation.json"),
                    bundle.join("evidence\\attestation.json"),
                )
                .expect("move to alias");
                replay_refused(
                    &bundle,
                    "LLV7102",
                    "a file moved to a literal-backslash alias",
                );
            } else {
                support::unix_only("PX-07", "literal-backslash filenames");
            }
            if support::non_utf8_names_backed("PX-07") {
                #[cfg(unix)]
                {
                    use std::os::unix::ffi::OsStrExt;

                    let (_dir, bundle) = bundle_copy(exported);
                    let invalid = std::ffi::OsStr::from_bytes(b"invalid\xff.json");
                    std::fs::write(bundle.as_std_path().join(invalid), b"invalid").expect("write");
                    replay_refused(&bundle, "LLV7102", "a non-UTF-8 filename");
                }
            }
            // A symlink.
            let (_dir, bundle) = bundle_copy(exported);
            let link = bundle.join("evidence/linked.json");
            support::symlink_any(bundle.join("result.json"), &link);
            replay_refused(&bundle, "LLS8001", "a symlink");
        }
        // §24.7, D2: determinism across directories.
        "PX-08" => {
            if !support::lean_backed("PX-08") {
                return;
            }
            let first = harness::exported("production");
            let second = offline();
            assert_eq!(second.run.0, 0, "the second export: {}", second.run.1);
            assert_ne!(first.project.root, second.project.root);
            let second_result: serde_json::Value =
                serde_json::from_str(&second.run.1).expect("a command result");
            let bundle = second.project.root.join(
                second_result["artifacts"][0]
                    .as_str()
                    .expect("the bundle artifact"),
            );
            let digests = |root: &Utf8Path| -> BTreeMap<String, Sha256Digest> {
                harness::tree(root)
                    .into_iter()
                    .map(|(path, bytes)| (path, sha(&bytes)))
                    .collect()
            };
            let (left, right) = (digests(&first.bundle()), digests(&bundle));
            let differing: BTreeSet<&String> = left
                .keys()
                .chain(right.keys())
                .filter(|path| left.get(*path) != right.get(*path))
                .collect();
            assert!(
                differing.is_empty(),
                "the two bundles are byte-identical; these differ: {differing:?}"
            );
            assert_eq!(result_of(&bundle).result_id, first.output.result.result_id);
            let text = String::from_utf8(
                std::fs::read(first.bundle().join("result.json").as_std_path()).expect("result"),
            )
            .expect("utf8");
            for absolute in [
                first.project.root.as_str(),
                second.project.root.as_str(),
                std::env::temp_dir().to_str().expect("utf8"),
            ] {
                assert!(!text.contains(absolute), "the result names `{absolute}`");
            }
            if let Ok(home) = std::env::var("HOME") {
                assert!(
                    !text.contains(&format!("\"{home}")),
                    "the result names HOME"
                );
            }
            // No key records a time or a host, and no value is an absolute
            // path.
            fn walk(value: &serde_json::Value, out: &mut Vec<String>) {
                match value {
                    serde_json::Value::Object(map) => {
                        for (key, child) in map {
                            if ["time", "timestamp", "date", "host", "user", "cwd"]
                                .contains(&key.as_str())
                            {
                                out.push(format!("key `{key}`"));
                            }
                            walk(child, out);
                        }
                    }
                    serde_json::Value::Array(items) => {
                        items.iter().for_each(|item| walk(item, out));
                    }
                    serde_json::Value::String(text)
                        if text.starts_with('/') || text.contains(":\\") =>
                    {
                        out.push(format!("value `{text}`"));
                    }
                    _ => {}
                }
            }
            let mut found = Vec::new();
            walk(&serde_json::from_str(&text).expect("JSON"), &mut found);
            assert!(found.is_empty(), "the result holds {found:?}");
        }
        // §24.7, §25.1: evidence paths and the bundle area are confined.
        "PX-09" => {
            let project = P::copy_example("production");
            let gnaf = harness::gnaf_evidence(&project);
            let outside = tempfile::Builder::new()
                .prefix("lexlean-outside-")
                .tempdir()
                .expect("tempdir");
            let outside_root = Utf8Path::from_path(outside.path()).expect("utf8");
            std::fs::write(
                outside_root.join("request.json").as_std_path(),
                &gnaf.request,
            )
            .expect("write");
            std::fs::write(
                outside_root.join("certificate.json").as_std_path(),
                &gnaf.certificate,
            )
            .expect("write");
            let absolute = outside_root.join("request.json");
            support::symlink_any(outside_root, project.root.join("linked"));
            for path in [
                absolute.as_str(),
                "../request.json",
                "evidence/../../request.json",
                "linked/request.json",
            ] {
                let mut request = request_for(&[(GNAF_ROOT, "pkg", &["rust-std"])]);
                request.evidence = vec![Evidence {
                    root: GNAF_ROOT.to_owned(),
                    kind: EvidenceKind::Gnaf,
                    request: path.to_owned(),
                    certificate: harness::GNAF_CERTIFICATE.to_owned(),
                    certificate_id: gnaf.certificate_id,
                }];
                refused(&project, &request, "LLS8001");
            }
            // A symlinked production area is refused, not followed, before
            // verification runs.
            std::fs::create_dir_all(project.root.join(".lexlean").as_std_path()).expect("mkdir");
            support::symlink_any(outside_root, project.root.join(".lexlean/production"));
            let error = production::build(
                &engine(&project),
                &request_for(&[("Production.Main.checkedSum", "pkg", &["rust-std"])]),
            )
            .expect_err("a symlinked area is refused");
            support::expect_code(&error, "LLS8001");
            assert!(!project
                .root
                .join(".lexlean/verified")
                .as_std_path()
                .exists());
            let written: Vec<_> = std::fs::read_dir(outside.path())
                .expect("outside")
                .flatten()
                .map(|entry| entry.file_name())
                .collect();
            assert_eq!(written.len(), 2, "nothing was written outside the project");
        }
        // §24.7: offline after setup.
        "PX-10" => {
            // An unacquired package is refused, never fetched.
            let project = P::copy_example("production");
            project.edit(
                "lexlean.toml",
                "\n[limits]",
                "\n[[lexicon_source]]\npackage = \"test.remote\"\nkind = \"git\"\nurl = \"https://example.invalid/repo.git\"\nrevision = \"0123456789abcdef0123456789abcdef01234567\"\nsubdirectory = \"pkg\"\n\n[limits]",
            );
            let error = production::build(
                &engine(&project),
                &request_for(&[("Production.Main.checkedSum", "pkg", &["rust-std"])]),
            )
            .expect_err("an unacquired package is refused");
            let codes: Vec<&str> = error.diagnostics.iter().map(|d| d.code.as_str()).collect();
            assert!(
                codes
                    .iter()
                    .any(|code| *code == "LLC0102" || *code == "LLS8003"),
                "refused without acquisition: {codes:?}"
            );
            assert!(
                !project.root.join(".lexlean/cache").as_std_path().exists(),
                "nothing was fetched"
            );
            // A set-up project exports with a git sentinel first on PATH,
            // which never runs (§8.3: a POSIX shell and the toolchain).
            if support::posix_shell_backed("PX-10") && support::lean_backed("PX-10") {
                let second = offline();
                assert_eq!(
                    second.run.0, 0,
                    "exported offline: {} {}",
                    second.run.1, second.run.2
                );
                assert!(!second.sentinel_ran, "no git operation ran");
            }
        }
        // §24.7, D1: no source text is representable.
        "PX-11" => {
            // Exhaustive destructuring: a new field fails to compile here, and
            // each field's type is one of the closed forms.
            fn name(_: &String) {}
            fn digest(_: &Sha256Digest) {}
            fn pinned(_: &Option<Sha256Digest>) {}
            let request = request_for(&[("Production.Main.checkedSum", "pkg", &["rust-std"])]);
            let ProductionRequest {
                spec,
                roots,
                expect,
                evidence,
            } = &request;
            name(spec);
            for RootRequest {
                root,
                package,
                targets,
            } in roots
            {
                name(root);
                let PackageName {
                    name: crate_name,
                    version,
                } = package;
                name(crate_name);
                name(version);
                for TargetRef { id, digest: d } in targets {
                    name(id);
                    digest(d);
                }
            }
            let Expectation {
                source_id,
                semantic_id,
                lock_sha256,
            } = expect;
            pinned(source_id);
            pinned(semantic_id);
            pinned(lock_sha256);
            for Evidence {
                root,
                kind,
                request: path,
                certificate,
                certificate_id,
            } in evidence
            {
                name(root);
                let EvidenceKind::Gnaf = kind;
                name(path);
                name(certificate);
                digest(certificate_id);
            }
            // Injection is refused by form.
            let project = P::copy_example("production");
            let lean = "Production.Main.checkedSum := by sorry";
            let mut injected = request_for(&[(lean, "pkg", &["rust-std"])]);
            refused(&project, &injected, "LLC0201");
            injected = request_for(&[("Production.Main.f(x)", "pkg", &["rust-std"])]);
            refused(&project, &injected, "LLC0201");
            injected = request_for(&[(
                "Production.Main.checkedSum",
                "pkg; pub fn evil() {}",
                &["rust-std"],
            )]);
            refused(&project, &injected, "LLC0201");
            injected = request_for(&[("Production.Main.checkedSum", "pkg", &["rust-std"])]);
            injected.roots[0].package.version = "1.0.0\" }\nfn evil() {".to_owned();
            refused(&project, &injected, "LLC0201");
            // A Lean file offered as evidence is a malformed GNAF document.
            project.write("evidence/Evil.lean", "theorem evil : False := sorry\n");
            injected = request_for(&[("Production.Main.checkedSum", "pkg", &["rust-std"])]);
            injected.evidence = vec![Evidence {
                root: "Production.Main.checkedSum".to_owned(),
                kind: EvidenceKind::Gnaf,
                request: "evidence/Evil.lean".to_owned(),
                certificate: "evidence/Evil.lean".to_owned(),
                certificate_id: sha(b"x"),
            }];
            refused(&project, &injected, "LLB6006");
        }
        // §24.7, W2: schema stability.
        "PX-12" => {
            let mut schemas = BTreeMap::new();
            for (name, _) in SCHEMAS {
                schemas.insert(name.to_owned(), api_schema(name).0);
            }
            let versions = recorded_versions();
            schema_versions_hold(&schemas, &versions).unwrap_or_else(|reason| panic!("{reason}"));
            // Armed: a changed schema under its recorded version fails, and
            // so does a version recorded twice.
            let mut drifted = schemas.clone();
            drifted
                .get_mut("production-result")
                .expect("result schema")
                .extend_from_slice(b" ");
            assert!(schema_versions_hold(&drifted, &versions).is_err());
            let mut twice = versions.clone();
            twice.push(twice[0].clone());
            assert!(schema_versions_hold(&schemas, &twice).is_err());
            // The produced values validate, and an unknown field does not.
            let project = P::copy_example("production");
            let request: serde_json::Value = serde_json::from_slice(
                &production_consumer::request(
                    &harness::application("production"),
                    &production_consumer::setup(&project.root.join("lexlean.toml"))
                        .expect("setup")
                        .1,
                    &[],
                )
                .to_json(),
            )
            .expect("JSON");
            let document: serde_json::Value =
                serde_json::from_slice(&production::targets_document().expect("document"))
                    .expect("JSON");
            let mut values = vec![
                ("production-request", request),
                ("target-descriptor", document["targets"][0].clone()),
                ("target-descriptors", document),
            ];
            // The result and the replay report need an export (§8.3).
            if support::lean_backed("PX-12") {
                let exported = harness::exported("production");
                values.push((
                    "production-result",
                    serde_json::from_slice(&exported.output.result.to_json()).expect("JSON"),
                ));
                values.push((
                    "production-replay",
                    serde_json::from_slice(&exported.report.to_json()).expect("JSON"),
                ));
            }
            for (name, value) in values {
                assert_valid(name, &value, name);
                let mut unknown = value.clone();
                unknown["unknown"] = serde_json::json!(true);
                assert!(
                    !crate::schema::validate(&api_schema(name).1, &unknown).is_empty(),
                    "{name} refuses an unknown field"
                );
            }
        }
        _ => panic!("no production-export case is wired for {id}"),
    }
}

/// CL-23: the `production` CLI.
pub fn cli() {
    let project = P::copy_example("production");
    // Targets: one document in either mode.
    let document = production::targets_document().expect("document");
    for arguments in [
        &["production", "targets"][..],
        &["--diagnostic-format", "json", "production", "targets"][..],
    ] {
        let (exit, stdout, stderr) = project.cli(arguments);
        assert_eq!(exit, 0, "{arguments:?}: {stderr}");
        assert_eq!(stdout.as_bytes(), document.as_slice(), "{arguments:?}");
        assert!(stderr.is_empty());
    }
    // Refusals map to their class's exit code, in JSON mode one result.
    let write = |name: &str, request: &ProductionRequest| {
        project.write(name, std::str::from_utf8(&request.to_json()).expect("utf8"));
    };
    project.write(
        "malformed.json",
        "{\"spec\":\"lexlean/production-request/1\",\"roots\":[],\"lean\":1}",
    );
    write(
        "unknown-root.json",
        &request_for(&[("Production.Main.absent", "pkg", &["rust-std"])]),
    );
    let mut escaping = request_for(&[(GNAF_ROOT, "pkg", &["rust-std"])]);
    escaping.evidence = vec![Evidence {
        root: GNAF_ROOT.to_owned(),
        kind: EvidenceKind::Gnaf,
        request: "../outside.json".to_owned(),
        certificate: "../outside.json".to_owned(),
        certificate_id: sha(b"x"),
    }];
    write("escaping.json", &escaping);
    let mut forged = request_for(&[(GNAF_ROOT, "pkg", &["rust-std"])]);
    let gnaf = harness::gnaf_evidence(&project);
    project.write(
        "evidence/r.json",
        std::str::from_utf8(&gnaf.request).expect("utf8"),
    );
    project.write(
        "evidence/c.json",
        std::str::from_utf8(&gnaf.certificate).expect("utf8"),
    );
    forged.evidence = vec![Evidence {
        root: GNAF_ROOT.to_owned(),
        kind: EvidenceKind::Gnaf,
        request: "evidence/r.json".to_owned(),
        certificate: "evidence/c.json".to_owned(),
        certificate_id: sha(b"forged"),
    }];
    write("forged.json", &forged);
    for (file, code, exit) in [
        ("malformed.json", "LLC0201", 2),
        ("unknown-root.json", "LLC0202", 2),
        ("forged.json", "LLV7101", 1),
        ("escaping.json", "LLS8001", 4),
        ("absent.json", "LLC0001", 2),
    ] {
        let (got, stdout, stderr) = project.cli(&[
            "--diagnostic-format",
            "json",
            "production",
            "build",
            "--request",
            file,
        ]);
        assert_eq!(got, exit, "{file}: {stdout}");
        assert!(stderr.is_empty(), "{file}: json mode keeps stderr empty");
        let value: serde_json::Value = serde_json::from_str(&stdout).expect("one result");
        assert_eq!(value["command"], "production");
        assert_eq!(value["success"], false);
        assert_eq!(value["exit_code"], exit);
        assert_eq!(value["diagnostics"][0]["code"], code, "{file}");
        let (human, _, stderr) =
            project.cli(&["--color", "never", "production", "build", "--request", file]);
        assert_eq!(human, exit);
        assert!(
            stderr.starts_with(&format!("error[{code}]")),
            "{file}: {stderr}"
        );
    }
    // A published export reports its bundle and identities.
    if !support::lean_backed("CL-23") {
        return;
    }
    let offline = offline();
    let (exit, stdout, _) = &offline.run;
    assert_eq!(*exit, 0, "{stdout}");
    let value: serde_json::Value = serde_json::from_str(stdout).expect("one result");
    let result = &harness::exported("production").output.result;
    assert_eq!(value["command"], "production");
    assert_eq!(value["success"], true);
    assert_eq!(
        value["artifacts"],
        serde_json::json!([format!(".lexlean/production/{}", result.result_id.to_hex())])
    );
    assert_eq!(value["source_id"], result.source.source_id.to_hex());
    assert_eq!(value["semantic_id"], result.source.semantic_id.to_hex());
    assert_eq!(value["build_id"], result.verification.build_id.to_hex());
    assert_eq!(
        value["attestation_id"],
        result.verification.attestation_id.to_hex()
    );
    // Replay needs no project, and a rejected replay exits 1.
    let bundle = harness::exported("production").bundle();
    let nowhere = tempfile::tempdir().expect("tempdir");
    let nowhere = Utf8Path::from_path(nowhere.path()).expect("utf8");
    let (exit, stdout, stderr) =
        support::cli_in(nowhere, &["production", "verify-artifact", bundle.as_str()]);
    assert_eq!(exit, 0, "{stderr}");
    assert_eq!(
        stdout,
        format!(
            "replayed result {}: {} outputs, bindings verified; not re-run: {}\n",
            result.result_id.to_hex(),
            harness::exported("production").report.outputs,
            production::NOT_RECHECKED.join(", ")
        )
    );
    let (_dir, tampered) = bundle_copy(harness::exported("production"));
    flip_byte(&tampered.join("evidence/preservation.json"), b'a', b'b');
    let (exit, stdout, _) = support::cli_in(
        nowhere,
        &[
            "--diagnostic-format",
            "json",
            "production",
            "verify-artifact",
            tampered.as_str(),
        ],
    );
    assert_eq!(exit, 1);
    let value: serde_json::Value = serde_json::from_str(&stdout).expect("one result");
    assert_eq!(value["diagnostics"][0]["code"], "LLV7102");
}
