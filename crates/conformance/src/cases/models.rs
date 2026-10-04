//! The `models` suite: MD-01..MD-12, language-1.2 models, contracts,
//! realizations, and evidence (SPEC.md §17.12).
//!
//! Every mutation starts from the committed `models` example, so a refusal
//! is the mutation's own and not an unrelated defect's. The seeded
//! differential (MD-09) builds its own project, and its expectations come
//! from the integer model in this file, which shares no code with the
//! compiler's kernels.

use std::collections::BTreeSet;
use std::sync::OnceLock;

use lexlean::{CheckRequest, Selection, Sha256Digest, VerifyRequest};
use serde_json::{json, Value as Json};

use crate::support::{self, VerifiedFixture, P};

const EXAMPLE: &str = "models";

/// The four refusal constructors, as elaborated code spells them.
const VIOLATIONS: [&str; 4] = [
    "ContractViolation.precondition",
    "ContractViolation.input_invariant",
    "ContractViolation.postcondition",
    "ContractViolation.output_invariant",
];

/// The modules of the committed example.
const MODULES: [&str; 9] = [
    "Glyphs",
    "Recognizer",
    "Triage",
    "Policy",
    "Session",
    "Ledger",
    "Pipeline",
    "Flows",
    "Main",
];

/// An edit of one declaration's semantic JSON.
type Edit = fn(&mut Json);

/// A declaration (or a description of the edit), the edit, and the message
/// its refusal must name.
type NamedEdit = (&'static str, Edit, &'static str);

/// A module and declaration, the edit, and the message its refusal must name.
type TargetedEdit = (&'static str, &'static str, Edit, &'static str);

// ---------------------------------------------------------------------------
// Source access.

fn module_path(module: &str) -> String {
    format!("src/{module}.lex.tex")
}

fn data_bounds(text: &str) -> (usize, usize) {
    let start = text.find("\\semanticdata{").expect("semantic data") + "\\semanticdata{".len();
    let end = text
        .rfind("}\n\\end{semanticmodule}")
        .expect("end of semantic data");
    (start, end)
}

/// The semantic data of one module of a project copy.
fn module_data(project: &P, module: &str) -> Json {
    let text = project.read(&module_path(module));
    let (start, end) = data_bounds(&text);
    serde_json::from_str(&text[start..end]).expect("module data")
}

fn write_module_data(project: &P, module: &str, data: &Json) {
    let path = module_path(module);
    let text = project.read(&path);
    let (start, end) = data_bounds(&text);
    project.write(
        &path,
        &format!(
            "{}{}{}",
            &text[..start],
            serde_json::to_string(data).expect("serializes"),
            &text[end..]
        ),
    );
}

fn declaration_mut<'a>(data: &'a mut Json, name: &str) -> &'a mut Json {
    data["declarations"]
        .as_array_mut()
        .expect("declarations")
        .iter_mut()
        .find(|declaration| declaration["name"] == name)
        .unwrap_or_else(|| panic!("`{name}` is declared"))
}

/// Edit one declaration of one module in place.
fn edit_declaration(project: &P, module: &str, name: &str, edit: impl FnOnce(&mut Json)) {
    let mut data = module_data(project, module);
    edit(declaration_mut(&mut data, name));
    write_module_data(project, module, &data);
}

/// A copy of the committed example with one declaration edited.
fn mutated(module: &str, name: &str, edit: impl FnOnce(&mut Json)) -> P {
    let project = P::copy_example(EXAMPLE);
    edit_declaration(&project, module, name, edit);
    project
}

/// The mutation is refused with `code`, naming `message`, before either
/// backend runs.
fn refused(project: &P, code: &str, message: &str) {
    let error = project.check_fails_with(code);
    assert!(
        error.to_string().contains(message),
        "expected {message:?} under {code}, got {error}"
    );
    project.assert_no_backend_output_with(code);
}

/// A committed negative fixture fails with `code`, naming `message`.
fn negative(name: &str, code: &str, message: &str) {
    let project = P::negative(name);
    let error = project.check_fails_with(code);
    assert!(
        error.to_string().contains(message),
        "tests/negative/{name}: expected {message:?}, got {error}"
    );
}

// ---------------------------------------------------------------------------
// Artifacts in a project copy.

/// The configured artifact sources, as `(sha256, path)`.
fn artifact_sources(project: &P) -> Vec<(String, String)> {
    let config: toml::Value = project.read("lexlean.toml").parse().expect("config TOML");
    config
        .get("artifact_source")
        .and_then(toml::Value::as_array)
        .map(|rows| {
            rows.iter()
                .map(|row| {
                    (
                        row["sha256"].as_str().expect("sha256").to_owned(),
                        row["path"].as_str().expect("path").to_owned(),
                    )
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Restate the configured artifact sources, in the canonical order (by
/// digest), keeping every other configuration line.
fn set_artifact_sources(project: &P, mut sources: Vec<(String, String)>) {
    sources.sort();
    let text = project.read("lexlean.toml");
    let limits = text.find("\n[limits]").expect("limits table");
    let start = text.find("\n[[artifact_source]]").unwrap_or(limits);
    let mut out = text[..start].to_owned();
    for (sha256, path) in sources {
        out.push_str(&format!(
            "\n[[artifact_source]]\nsha256 = \"{sha256}\"\npath = \"{path}\"\n"
        ));
    }
    out.push_str(&text[limits..]);
    project.write("lexlean.toml", &out);
}

fn digest(bytes: &[u8]) -> String {
    Sha256Digest::of(bytes).to_hex()
}

/// Replace an artifact's bytes and restate its digest and length wherever
/// they are pinned (the configuration and the declaration), then relock, so
/// that only the bytes differ.
fn replace_artifact(project: &P, file: &str, module: &str, name: &str, bytes: &[u8]) {
    let path = project.root.join(file);
    let old = digest(&std::fs::read(path.as_std_path()).expect("artifact bytes"));
    let new = digest(bytes);
    std::fs::write(path.as_std_path(), bytes).expect("write artifact");
    let sources = artifact_sources(project)
        .into_iter()
        .map(|(sha256, path)| (if sha256 == old { new.clone() } else { sha256 }, path))
        .collect();
    set_artifact_sources(project, sources);
    edit_declaration(project, module, name, |declaration| {
        declaration["sha256"] = json!(new);
        declaration["length"] = json!(bytes.len());
    });
    project.relock();
}

// ---------------------------------------------------------------------------
// Snapshots and the shared verified run.

fn snapshot(project: &P) -> lexlean::SemanticSnapshot {
    project
        .engine()
        .snapshot(CheckRequest {
            selection: Selection::Entrypoints,
        })
        .expect("snapshot")
}

/// The elaboration of `module.name` in a snapshot.
fn elaboration<'a>(
    snapshot: &'a lexlean::SemanticSnapshot,
    module: &str,
    name: &str,
) -> &'a lexlean::SnapshotElaboration {
    snapshot
        .modules()
        .iter()
        .find(|candidate| candidate.name() == module)
        .unwrap_or_else(|| panic!("module {module}"))
        .declarations()
        .iter()
        .find(|declaration| declaration.logical_id() == name)
        .unwrap_or_else(|| panic!("`{module}.{name}` in the snapshot"))
        .elaboration()
        .unwrap_or_else(|| panic!("`{module}.{name}` carries its elaboration"))
}

fn names(values: &[Json]) -> Vec<String> {
    values
        .iter()
        .map(|value| value["name"].as_str().expect("name").to_owned())
        .collect()
}

/// The one verified run of the committed example, shared by the cases that
/// read its attestation.
fn verified_models() -> &'static VerifiedFixture {
    static FIXTURE: OnceLock<VerifiedFixture> = OnceLock::new();
    FIXTURE.get_or_init(|| {
        let _guard = support::env_lock();
        let project = P::copy_example(EXAMPLE);
        let outcome = project
            .engine()
            .verify(VerifyRequest {
                selection: Selection::Entrypoints,
            })
            .expect("the models example verifies under pinned Lean");
        let attestation: Json = serde_json::from_slice(
            &std::fs::read(outcome.root.join("attestation.json").as_std_path())
                .expect("attestation exists"),
        )
        .expect("attestation parses");
        VerifiedFixture {
            project,
            outcome,
            attestation,
        }
    })
}

/// [`verified_models`] behind the §8.3 host gate.
fn models_backed(id: &str) -> Option<&'static VerifiedFixture> {
    support::lean_backed(id).then(verified_models)
}

/// The attestation row of a generated or source Lean declaration.
fn attested<'a>(fixture: &'a VerifiedFixture, name: &str) -> &'a Json {
    fixture.attestation["declarations"]
        .as_array()
        .expect("declarations")
        .iter()
        .find(|row| row["name"] == name)
        .unwrap_or_else(|| panic!("the attestation records `{name}`"))
}

fn assert_attested(fixture: &VerifiedFixture, name: &str, policy: &str) {
    let row = attested(fixture, name);
    assert_eq!(row["result"], "ok", "{name}: {row}");
    assert_eq!(row["policy"]["kind"], policy, "{name}: {row}");
}

// ---------------------------------------------------------------------------
// The cases.

/// Run the case for one MD conformance ID.
///
/// # Panics
///
/// Panics when the case's assertion fails, and for an unwired ID.
pub fn run(id: &str) {
    match id {
        "MD-01" => md_01(),
        "MD-02" => md_02(),
        "MD-03" => md_03(),
        "MD-04" => md_04(),
        "MD-05" => md_05(),
        "MD-06" => md_06(),
        "MD-07" => md_07(),
        "MD-08" => md_08(),
        "MD-09" => md_09(),
        "MD-10" => md_10(),
        "MD-11" => md_11(),
        "MD-12" => md_12(),
        _ => panic!("no models case {id}"),
    }
}

/// The schema `$defs` entry `definition` admits exactly the `kind`s listed
/// under `property` anywhere beneath it.
fn schema_kinds(schema: &Json, definition: &str, property: &str) -> BTreeSet<String> {
    fn walk(value: &Json, property: &str, out: &mut BTreeSet<String>) {
        match value {
            Json::Object(map) => {
                if let Some(node) = map.get("properties").and_then(|p| p.get(property)) {
                    if let Some(constant) = node.get("const").and_then(Json::as_str) {
                        out.insert(constant.to_owned());
                    }
                    for item in node
                        .get("enum")
                        .and_then(Json::as_array)
                        .into_iter()
                        .flatten()
                    {
                        out.insert(item.as_str().expect("enum string").to_owned());
                    }
                }
                for child in map.values() {
                    walk(child, property, out);
                }
            }
            Json::Array(items) => {
                for item in items {
                    walk(item, property, out);
                }
            }
            _ => {}
        }
    }
    let mut kinds = BTreeSet::new();
    walk(&schema["$defs"][definition], property, &mut kinds);
    kinds
}

/// §17.12: the model constructs belong to the closed language-1.2 schemas,
/// are routed out of language 1.1, and admit no free-form member.
#[allow(clippy::too_many_lines)]
fn md_01() {
    let module = support::schema("semantic-module-v2");
    let declarations = schema_kinds(&module, "declaration", "kind");
    for kind in ["artifact", "contract", "realization", "evidence", "model"] {
        assert!(declarations.contains(kind), "module schema lacks `{kind}`");
    }
    assert!(schema_kinds(&module, "term", "kind").contains("checked_apply"));
    assert!(schema_kinds(&module, "type", "kind").contains("contract_violation"));
    assert!(schema_kinds(&module, "term", "operation").contains("less_than"));
    let snapshot_schema = support::schema("semantic-snapshot-v2");
    let snapshot_text = snapshot_schema.to_string();
    for needle in [
        "\"artifact\"",
        "\"contract\"",
        "\"realization\"",
        "\"evidence\"",
        "\"model\"",
        "\"checked_apply\"",
        "\"contract_violation\"",
        "\"less_than\"",
        "\"elaboration\"",
    ] {
        assert!(
            snapshot_text.contains(needle),
            "the snapshot schema closes over {needle}"
        );
    }
    // Every committed model module is an instance of the closed schema.
    let project = P::copy_example(EXAMPLE);
    for name in MODULES {
        support::assert_schema(
            "semantic-module-v2",
            &format!("examples/models/src/{name}.lex.tex"),
            &module_data(&project, name),
        );
    }
    // Language 1.1 routes every construct out before either backend runs.
    let nat = json!({"kind": "nat"});
    let under_1_1 = [
        (
            json!({"kind": "artifact", "name": "w", "role": "binary", "sha256": "0".repeat(64),
                   "length": 1, "schema": {"kind": "bytes"}, "type": {"kind": "bytes"}}),
            "`artifact declaration` is a language-1.2 construct",
        ),
        (
            json!({"kind": "realization", "name": "R", "input": {"name": "x", "type": nat},
                   "output": nat, "descriptor": {"kind": "deterministic",
                   "body": {"kind": "var", "name": "x"}}}),
            "`realization declaration` is a language-1.2 construct",
        ),
        (
            json!({"kind": "evidence", "name": "E", "contract": {"member": {"name": "C"}},
                   "realization": {"member": {"name": "R"}}, "claims": []}),
            "`evidence declaration` is a language-1.2 construct",
        ),
        (
            json!({"kind": "model", "name": "M", "contract": {"member": {"name": "C"}},
                   "realization": {"member": {"name": "R"}}}),
            "`model declaration` is a language-1.2 construct",
        ),
        (
            json!({"kind": "definition", "name": "f", "executable": true,
                   "parameters": [{"name": "x", "type": nat}], "result": nat,
                   "body": {"kind": "checked_apply", "model": {"name": "M"},
                            "arguments": [{"kind": "var", "name": "x"}], "checks": []}}),
            "`checked_apply` is a language-1.2 construct",
        ),
        (
            json!({"kind": "definition", "name": "f",
                   "parameters": [{"name": "x", "type": {"kind": "contract_violation"}}],
                   "result": nat, "body": {"kind": "nat", "value": "0"}}),
            "`contract_violation type` is a language-1.2 construct",
        ),
        (
            json!({"kind": "definition", "name": "f",
                   "parameters": [{"name": "x", "type": nat}], "result": {"kind": "bool"},
                   "body": {"kind": "primitive", "operation": "less_than",
                            "arguments": [{"kind": "var", "name": "x"}, {"kind": "var", "name": "x"}],
                            "result": {"kind": "bool"}}}),
            "is a language-1.2 construct",
        ),
    ];
    negative(
        "model-under-1.1",
        "LLT4001",
        "`contract declaration` is a language-1.2 construct",
    );
    for (declaration, message) in under_1_1 {
        let project = P::negative("model-under-1.1");
        let mut data = module_data(&project, "Main");
        data["declarations"] = json!([declaration]);
        write_module_data(&project, "Main", &data);
        refused(&project, "LLT4001", message);
    }
    let project = P::negative("model-artifact-source-under-1.1");
    let (exit, _, stderr) = project.cli(&["check"]);
    assert_eq!(exit, 2, "{stderr}");
    assert!(
        stderr.contains("LLC0101") && stderr.contains("artifact_source is a language-1.2 field"),
        "{stderr}"
    );
    // No member outside the closed schema: prompt text, a free-form
    // description, raw configuration, an open architecture or layer.
    for (module, name, member, message) in [
        ("Recognizer", "DigitNet", "prompt", "unknown field `prompt`"),
        (
            "Recognizer",
            "DigitContract",
            "description",
            "unknown field `description`",
        ),
        (
            "Recognizer",
            "DigitEvidence",
            "notes",
            "unknown field `notes`",
        ),
        (
            "Recognizer",
            "DigitModel",
            "prompt",
            "unknown field `prompt`",
        ),
        ("Recognizer", "banner", "format", "unknown field `format`"),
    ] {
        let project = mutated(module, name, |declaration| {
            declaration[member] = json!("classify the digit carefully");
        });
        refused(&project, "LLT4001", message);
    }
    let edits: [TargetedEdit; 6] = [
        (
            "DigitNet",
            "raw configuration",
            |declaration| declaration["descriptor"]["config"] = json!({"activation": "gelu"}),
            "unknown field `config`",
        ),
        (
            "DigitNet",
            "an open architecture",
            |declaration| declaration["descriptor"]["architecture"] = json!("transformer"),
            "unknown variant `transformer`",
        ),
        (
            "DigitNet",
            "an open layer",
            |declaration| {
                declaration["descriptor"]["layers"]
                    .as_array_mut()
                    .expect("layers")
                    .insert(1, json!({"kind": "softmax"}));
            },
            "unknown variant `softmax`",
        ),
        (
            "hiddenWeights",
            "a floating-point element",
            |declaration| declaration["schema"]["element"] = json!("float32"),
            "unknown variant `float32`",
        ),
        (
            "banner",
            "an open role",
            |declaration| declaration["role"] = json!("prompt"),
            "unknown variant `prompt`",
        ),
        (
            "banner",
            "an open schema",
            |declaration| declaration["schema"] = json!({"kind": "json"}),
            "unknown variant `json`",
        ),
    ];
    for (name, what, edit, message) in edits {
        let project = mutated("Recognizer", name, edit);
        let error = project.check_fails_with("LLT4001");
        assert!(
            error.to_string().contains(message),
            "{what}: expected {message:?}, got {error}"
        );
    }
    negative("model-opaque-member", "LLT4001", "unknown field `prompt`");
    negative(
        "model-float-descriptor",
        "LLT4001",
        "unknown variant `float32`",
    );
}

/// §10.1, §17.12, §21.6: artifacts are configured, confined, digested,
/// decoded under their closed schema, and embedded exactly.
#[allow(clippy::too_many_lines)]
/// Set in the address-space-limited process MD-02 starts: the project
/// whose wide artifact that process checks.
const WIDE_ARTIFACT_PROJECT: &str = "LEXLEAN_MD02_WIDE_ARTIFACT_PROJECT";

/// Check the wide-artifact project at `root` in this process: refused at
/// its first declaration, before a line is decoded.
fn check_wide_artifact(root: &camino::Utf8Path) {
    let (exit, _, stderr) = support::cli_in(root, &["check"]);
    assert_eq!(exit, 4, "{stderr}");
    assert!(
        stderr.contains("once artifact `vocab0` decodes to 9043965, before decoding"),
        "{stderr}"
    );
}

fn md_02() {
    if let Ok(root) = std::env::var(WIDE_ARTIFACT_PROJECT) {
        check_wide_artifact(camino::Utf8Path::new(&root));
        return;
    }
    let project = P::copy_example(EXAMPLE);
    let checked = support::checked_project(&project);
    let sources = artifact_sources(&project);
    assert_eq!(sources.len(), 8, "the example configures eight artifacts");
    let mut read: Vec<(String, usize, String)> = checked
        .artifacts
        .iter()
        .map(|(path, length, sha256)| (path.clone(), *length, sha256.to_hex()))
        .collect();
    read.sort();
    let mut expected: Vec<(String, usize, String)> = sources
        .iter()
        .map(|(sha256, path)| {
            let bytes =
                std::fs::read(project.root.join(path).as_std_path()).expect("artifact bytes");
            assert_eq!(&digest(&bytes), sha256, "{path} has its configured digest");
            (path.clone(), bytes.len(), sha256.clone())
        })
        .collect();
    expected.sort();
    assert_eq!(
        read, expected,
        "linking reads exactly the configured artifacts"
    );
    // The generated Lean embeds the exact bytes, and the typed value is
    // proved to be their decoding.
    let build = support::rendered(&project);
    let lean = support::lean_text(&build, "Recognizer");
    let banner = std::fs::read(project.root.join("artifacts/banner.bin").as_std_path())
        .expect("banner bytes");
    let embedded = banner
        .iter()
        .map(|byte| format!("UInt8.ofNat (nat_lit {byte})"))
        .collect::<Vec<_>>()
        .join(", ");
    assert!(lean.contains(&format!(
        "public def banner.bytes : ByteArray := ByteArray.mk #[{embedded}]"
    )));
    assert!(lean.contains("public theorem banner.decoded : banner = banner.bytes := rfl"));
    let weights = std::fs::read(
        project
            .root
            .join("artifacts/hidden-weights.bin")
            .as_std_path(),
    )
    .expect("weights bytes");
    let embedded = weights
        .iter()
        .map(|byte| format!("UInt8.ofNat (nat_lit {byte})"))
        .collect::<Vec<_>>()
        .join(", ");
    assert!(lean.contains(&format!(
        "public def hiddenWeights.bytes : ByteArray := ByteArray.mk #[{embedded}]"
    )));
    assert!(lean.contains(
        "public theorem hiddenWeights.decoded : (LexLeanModels.tensorMatches 1 true hiddenWeights.flatten hiddenWeights.bytes && (hiddenWeights.length == 12) && (hiddenWeights.map List.length == List.replicate 12 7)) = true := by decide"
    ));
    assert!(lean.contains(
        "public theorem digitNames.decoded : LexLeanModels.linesMatch digitNames digitNames.bytes = true := by decide"
    ));
    // The build manifest records every artifact as an input.
    let manifest: Json = serde_json::from_slice(
        &build
            .files
            .iter()
            .find(|(path, _)| path == "manifest.json")
            .expect("manifest")
            .1,
    )
    .expect("manifest JSON");
    let mut inputs: Vec<(String, usize, String)> = manifest["inputs"]
        .as_array()
        .expect("inputs")
        .iter()
        .filter(|row| row["kind"] == "model-artifact")
        .map(|row| {
            (
                row["path"].as_str().expect("path").to_owned(),
                usize::try_from(row["byte_length"].as_u64().expect("length")).expect("fits"),
                row["sha256"].as_str().expect("sha256").to_owned(),
            )
        })
        .collect();
    inputs.sort();
    assert_eq!(
        inputs, expected,
        "the manifest records every model artifact"
    );
    // The document states what an artifact is, never its bytes or its
    // decoded value (§17.12 rule 13).
    for module in MODULES {
        let tex = support::tex_text(&build, module);
        assert!(
            !tex.contains("ByteArray.mk"),
            "{module}.tex prints artifact bytes"
        );
        assert!(!tex.contains(&embedded), "{module}.tex prints the weights");
    }
    let tex = support::tex_text(&build, "Recognizer");
    assert!(tex.contains(
        "Elaborates to: \\texttt{hiddenWeights : List (List (Int)), their decoding under the schema}"
    ));
    assert!(tex.contains("Elaborates to: \\texttt{banner.bytes : ByteArray, the configured bytes}"));
    // A shape is bounded, and the declared type checked, before any byte is
    // decoded: a one-byte artifact with a 40000-deep shape fails at once.
    let started = std::time::Instant::now();
    negative(
        "model-artifact-rank-overflow",
        "LLR3008",
        "an integer tensor has at most 16 dimensions, not 40000",
    );
    assert!(
        started.elapsed() < std::time::Duration::from_secs(30),
        "the rank bound is checked before decoding"
    );
    // LLR3007: missing, digest, length, unconfigured.
    let missing = P::copy_example(EXAMPLE);
    std::fs::remove_file(missing.root.join("artifacts/banner.bin").as_std_path()).expect("remove");
    refused(
        &missing,
        "LLR3007",
        "model artifact `artifacts/banner.bin` with SHA-256",
    );
    let forged = P::copy_example(EXAMPLE);
    std::fs::write(
        forged.root.join("artifacts/banner.bin").as_std_path(),
        b"HELLO",
    )
    .expect("write");
    refused(&forged, "LLR3007", "not its configured");
    let length = mutated("Recognizer", "banner", |declaration| {
        declaration["length"] = json!(6);
    });
    refused(&length, "LLR3007", "has 5 bytes, not the declared length 6");
    let unconfigured = mutated("Recognizer", "banner", |declaration| {
        declaration["sha256"] = json!("0".repeat(64));
    });
    refused(&unconfigured, "LLR3007", "has no project artifact source");
    // LLR3008: shape, element, type, role, encoding.
    let edits: [NamedEdit; 5] = [
        (
            "hiddenWeights",
            |declaration| declaration["schema"]["shape"] = json!([12, 8]),
            "84 bytes cannot hold 96 elements",
        ),
        (
            "hiddenBias",
            |declaration| declaration["schema"]["element"] = json!("int16"),
            "48 bytes cannot hold 12 elements of 2 byte(s)",
        ),
        (
            "hiddenWeights",
            |declaration| declaration["type"] = json!({"kind": "list", "element": {"kind": "int"}}),
            "is not the schema's value type",
        ),
        (
            "banner",
            |declaration| declaration["role"] = json!("parameters"),
            "the role Parameters does not admit the bytes schema",
        ),
        (
            "hiddenBias",
            |declaration| declaration["role"] = json!("vocabulary"),
            "the role Vocabulary does not admit the int_tensor schema",
        ),
    ];
    for (name, edit, message) in edits {
        refused(&mutated("Recognizer", name, edit), "LLR3008", message);
    }
    let invalid = P::copy_example(EXAMPLE);
    let mut names =
        std::fs::read(invalid.root.join("artifacts/digit-names.txt").as_std_path()).expect("names");
    names[0] = 0xff;
    replace_artifact(
        &invalid,
        "artifacts/digit-names.txt",
        "Recognizer",
        "digitNames",
        &names,
    );
    refused(&invalid, "LLR3008", "the bytes are not UTF-8");
    negative(
        "model-artifact-schema-violation",
        "LLR3008",
        "cannot hold 6 elements",
    );
    negative("model-artifact-invalid-utf8", "LLR3008", "not UTF-8");
    negative("model-artifact-role-violation", "LLR3008", "does not admit");
    negative("model-artifact-missing", "LLR3007", "is missing");
    negative(
        "model-artifact-digest-mismatch",
        "LLR3007",
        "not its configured",
    );
    negative(
        "model-artifact-length-mismatch",
        "LLR3007",
        "not the declared length",
    );
    negative(
        "model-artifact-unconfigured",
        "LLR3007",
        "has no project artifact source",
    );
    // Confinement and the resource budget: artifacts are source bytes.
    if cfg!(unix) {
        let linked = P::copy_example(EXAMPLE);
        let banner = linked.root.join("artifacts/banner.bin");
        let real = linked.root.join("artifacts/banner-real.bin");
        std::fs::rename(banner.as_std_path(), real.as_std_path()).expect("rename");
        support::symlink_any("banner-real.bin", banner.as_std_path());
        refused(
            &linked,
            "LLS8001",
            "symlinks are rejected in confined paths",
        );
    }
    let project = P::negative("model-artifact-over-limit");
    let error = project.check_fails_with("LLS8002");
    assert!(error
        .to_string()
        .contains("max_file_bytes exceeded by model artifact"));
    // The budget counts artifact bytes: two projects that differ only in
    // the size of one artifact, under one budget, one fitting and one not.
    let limited = |project: &P| {
        project.edit(
            "lexlean.toml",
            "max_total_source_bytes = 67108864",
            "max_total_source_bytes = 2000000",
        );
        project.relock();
    };
    let small = P::copy_example(EXAMPLE);
    limited(&small);
    small.check_ok();
    let large = P::copy_example(EXAMPLE);
    limited(&large);
    replace_artifact(
        &large,
        "artifacts/banner.bin",
        "Recognizer",
        "banner",
        &vec![0; 3_000_000],
    );
    let error = large.check_fails_with("LLS8002");
    assert!(
        error
            .to_string()
            .contains("max_total_source_bytes exceeded"),
        "artifact bytes count toward the source budget: {error}"
    );
    // Each artifact declaration is charged before it decodes, and the charge
    // runs across declarations: one digest named three times, each decoding
    // to 1292 nodes under a budget of 3000, is refused at the third, before
    // it decodes, not after the module links.
    negative(
        "model-artifact-decode-budget",
        "LLS8002",
        "max_ir_nodes exceeded: configured 3000, observed 3876 IR nodes once artifact `vocab2` decodes to 1292, before decoding",
    );
    let names = ["vocab0", "vocab1", "vocab2"];
    // The bytes each declaration materializes again count toward
    // `max_total_source_bytes` the same way: a budget holding the loaded
    // sources and two more copies refuses the third declaration.
    let budgeted = |max_total_source_bytes: u64| {
        let project = P::negative("model-artifact-decode-budget");
        project.edit(
            "lexlean.toml",
            "max_ir_nodes = 3000",
            "max_ir_nodes = 2000000",
        );
        project.edit(
            "lexlean.toml",
            "max_total_source_bytes = 67108864",
            &format!("max_total_source_bytes = {max_total_source_bytes}"),
        );
        project.relock();
        project.check_fails_with("LLS8002").to_string()
    };
    // The probe has as many digits as the budget below, so the configuration
    // it counts has the same length.
    let loaded: u64 = budgeted(10_000)
        .split("observed ")
        .nth(1)
        .and_then(|tail| tail.split(|c: char| !c.is_ascii_digit()).next())
        .and_then(|count| count.parse().ok())
        .expect("the loaded source bytes");
    // Linking then counts the artifact file once and the module's source.
    let module = std::fs::read(
        support::repo_root()
            .join("tests/negative/model-artifact-decode-budget/project/src/Main.lex.tex")
            .as_std_path(),
    )
    .expect("module source")
    .len() as u64;
    let linked = loaded + 1200 + module;
    assert!(
        (10_000..97_600).contains(&linked),
        "{linked} linked source bytes"
    );
    let refused_bytes = budgeted(linked + 2 * 1200);
    assert!(
        refused_bytes.contains(&format!(
            "observed {} source bytes once artifact `vocab2` materializes its 1200 bytes",
            linked + 3 * 1200
        )),
        "{refused_bytes}"
    );
    // The amplification this closes: one 4 MiB artifact of empty lines
    // decodes to 9043965 nodes, so under the default limits its first
    // declaration is refused at once, before a line is allocated, however
    // many declarations name it (decoding them took about 1.5 GB each).
    let wide = P::negative("model-artifact-decode-budget");
    wide.edit(
        "lexlean.toml",
        "max_ir_nodes = 3000",
        "max_ir_nodes = 2000000",
    );
    let lines = vec![b'\n'; 4 * 1024 * 1024];
    std::fs::write(wide.root.join("artifacts/lines.txt").as_std_path(), &lines).expect("write");
    let sources = vec![(digest(&lines), "artifacts/lines.txt".to_owned())];
    set_artifact_sources(&wide, sources);
    for name in names {
        edit_declaration(&wide, "Main", name, |declaration| {
            declaration["sha256"] = json!(digest(&lines));
            declaration["length"] = json!(lines.len());
        });
    }
    wide.relock();
    // Decoding it takes about 1.5 GB. On Linux, the normative host, the check
    // runs again in a process whose address space `ulimit -v` bounds at
    // 1 GiB, so a charge made only after decoding aborts it rather than
    // passing; macOS's shell cannot set that limit, so elsewhere it runs here.
    if cfg!(target_os = "linux") {
        let ran = std::process::Command::new("sh")
            .args([
                "-c",
                "ulimit -v 1048576 && exec \"$0\" --exact conformance_md_02 --test-threads=1 --quiet",
            ])
            .arg(std::env::current_exe().expect("this test"))
            .env(WIDE_ARTIFACT_PROJECT, wide.root.as_str())
            .output()
            .expect("the limited check runs");
        assert!(
            ran.status.success(),
            "the wide artifact is refused within 1 GiB: {:?} {}{}",
            ran.status,
            String::from_utf8_lossy(&ran.stdout),
            String::from_utf8_lossy(&ran.stderr)
        );
    } else {
        check_wide_artifact(&wide.root);
    }
    // A path outside the project root is refused by the configuration.
    let escape = P::copy_example(EXAMPLE);
    let mut sources = artifact_sources(&escape);
    sources[0].1 = "../outside.bin".to_owned();
    set_artifact_sources(&escape, sources);
    let (exit, _, stderr) = escape.cli(&["check"]);
    assert_eq!(exit, 2, "{stderr}");
    assert!(
        stderr.contains("LLC0101") && stderr.contains("is not project-relative"),
        "{stderr}"
    );
}

/// §17.12: contracts, predicates, validators, and their link theorems.
#[allow(clippy::too_many_lines)]
fn md_03() {
    let edits: [NamedEdit; 7] = [
        (
            "recognizesCheck",
            |declaration| {
                declaration
                    .as_object_mut()
                    .expect("object")
                    .remove("executable");
            },
            "postcondition validator `recognizesCheck` is not executable",
        ),
        (
            "DigitContract",
            |declaration| declaration["postcondition"] = json!({"name": "recognizesCheck"}),
            "must take (Glyphs.Glyph, Nat) to Prop, but takes (Glyphs.Glyph, Nat) to Bool",
        ),
        (
            "DigitContract",
            |declaration| declaration["output"]["type"] = json!({"kind": "int"}),
            "must take (Glyphs.Glyph, Int) to Prop, but takes (Glyphs.Glyph, Nat) to Prop",
        ),
        (
            "DigitContract",
            |declaration| declaration["validators"][0]["predicate"] = json!("precondition"),
            "validates the absent precondition",
        ),
        (
            "DigitContract",
            |declaration| {
                declaration["validators"][0]["sound"] =
                    json!({"name": "recognizes_check_complete"});
            },
            "validator postcondition sound: `recognizes_check_complete` does not state exactly the generated obligation",
        ),
        (
            "DigitContract",
            |declaration| {
                declaration["validators"][0]["complete"] = json!({"name": "recognizes_check_sound"});
            },
            "validator postcondition complete: `recognizes_check_sound` does not state exactly the generated obligation",
        ),
        (
            "DigitContract",
            |declaration| {
                let validator = declaration["validators"][0].clone();
                declaration["validators"]
                    .as_array_mut()
                    .expect("validators")
                    .push(validator);
            },
            "validators are not strictly sorted by predicate",
        ),
    ];
    for (name, edit, message) in edits {
        refused(&mutated("Recognizer", name, edit), "LLT4006", message);
    }
    negative(
        "model-predicate-signature",
        "LLT4006",
        "must take (Nat, Nat) to Prop, but takes (Nat) to Prop",
    );
    negative(
        "model-validator-unsound-statement",
        "LLT4006",
        "does not state exactly the generated obligation",
    );
    // Lean restates each link against the fixed model semantics, so a
    // generator that stated anything else would be refused there.
    let project = P::copy_example(EXAMPLE);
    let build = support::rendered(&project);
    let lean = support::lean_text(&build, "Recognizer");
    assert!(lean.contains(
        "public theorem DigitContract.postcondition_sound : LexLeanModels.Sound2 ((Models.Recognizer.recognizesCheck)) ((Models.Recognizer.Recognizes)) := Models.Recognizer.recognizes_check_sound"
    ));
    assert!(lean.contains(
        "public theorem DigitContract.postcondition_complete : LexLeanModels.Complete2 ((Models.Recognizer.recognizesCheck)) ((Models.Recognizer.Recognizes)) := Models.Recognizer.recognizes_check_complete"
    ));
    for helper in ["def Sound1", "def Sound2", "def Complete1", "def Complete2"] {
        assert!(
            lean.contains(helper),
            "the fixed model runtime defines `{helper}`"
        );
    }
    let snapshot = snapshot(&project);
    let contract = elaboration(&snapshot, "Recognizer", "DigitContract");
    assert!(
        contract.declarations().is_empty(),
        "a contract is specification only"
    );
    assert_eq!(
        names(contract.cross_checks()),
        [
            "DigitContract.postcondition_sound",
            "DigitContract.postcondition_complete"
        ]
    );
    assert_eq!(contract.obligations().len(), 2);
    if let Some(fixture) = models_backed("MD-03") {
        for name in [
            "Models.Recognizer.DigitContract.postcondition_sound",
            "Models.Recognizer.DigitContract.postcondition_complete",
            "Models.Session.SessionContract.invariant_sound",
            "Models.Policy.PolicyContract.precondition_complete",
        ] {
            assert_attested(fixture, name, "allow");
        }
    }
}

/// §17.12: realizations elaborate to ordinary language-1.2 denotations
/// whose widths, shapes, roles, and labels are checked in linking.
#[allow(clippy::too_many_lines)]
fn md_04() {
    let project = P::copy_example(EXAMPLE);
    let snapshot = snapshot(&project);
    for (module, name, companions) in [
        (
            "Recognizer",
            "DigitNet",
            &[
                "DigitNet.encoded",
                "DigitNet.layer1",
                "DigitNet.layer2",
                "DigitNet.layer3",
                "DigitNet.layer4",
                "DigitNet.logits",
                "DigitNet",
            ][..],
        ),
        (
            "Triage",
            "TriageScore",
            &["TriageScore.features", "TriageScore.scores", "TriageScore"][..],
        ),
        (
            "Policy",
            "PolicyRules",
            &["PolicyRules.selected", "PolicyRules"][..],
        ),
        (
            "Session",
            "SessionStep",
            &["SessionStep.initial", "SessionStep"][..],
        ),
        ("Pipeline", "SegmentCounter", &["SegmentCounter"][..]),
    ] {
        let elaborated = elaboration(&snapshot, module, name);
        assert_eq!(names(elaborated.declarations()), companions, "{name}");
        for declaration in elaborated.declarations() {
            assert_eq!(declaration["kind"], "definition", "{name}: {declaration}");
            assert_eq!(declaration["executable"], true, "{name}: {declaration}");
            let text = declaration.to_string();
            for model_kind in [
                "\"artifact\"",
                "\"contract\"",
                "\"realization\"",
                "\"evidence\"",
                "\"checked_apply\"",
            ] {
                assert!(
                    !text.contains(&format!("\"kind\":{model_kind}")),
                    "{name} elaborates to ordinary terms only"
                );
            }
        }
    }
    // The first-maximum decoder compares with the less_than primitive.
    let net = elaboration(&snapshot, "Recognizer", "DigitNet");
    let decoder = net.declarations().last().expect("DigitNet").to_string();
    assert!(decoder.contains("\"operation\":\"less_than\""));
    let requantize = net.declarations()[3].to_string();
    assert!(requantize.contains("\"operation\":\"quotient\""));
    assert!(requantize.contains("\"operation\":\"less_than\""));
    // The width evidence is a statement-exact obligation.
    assert_eq!(net.obligations().len(), 1);
    assert_eq!(net.obligations()[0]["theorem"]["name"], "digit_net_width");
    // Width, shape, role, label, and slot mismatches fail with LLT4006.
    let edits: [TargetedEdit; 13] = [
        (
            "Recognizer",
            "DigitNet",
            |declaration| declaration["descriptor"]["layers"][0]["inputs"] = json!(8),
            "layer 1 takes 8 input(s) to 12 output(s), but receives 7",
        ),
        (
            "Recognizer",
            "DigitNet",
            |declaration| declaration["descriptor"]["layers"][3]["inputs"] = json!(11),
            "layer 4 takes 11 input(s) to 10 output(s), but receives 12",
        ),
        (
            "Recognizer",
            "DigitNet",
            |declaration| {
                declaration["descriptor"]["decoder"]["labels"]
                    .as_array_mut()
                    .expect("labels")
                    .pop();
            },
            "decodes 10 value(s) with 9 label(s)",
        ),
        (
            "Recognizer",
            "DigitNet",
            |declaration| declaration["descriptor"]["width"] = json!(8),
            "`digit_net_width` does not state exactly the generated obligation",
        ),
        (
            "Recognizer",
            "DigitNet",
            |declaration| {
                declaration["descriptor"]["layers"][0]["weights"] = json!({"name": "digitNames"});
            },
            "weights `digitNames` has role Vocabulary; a parameter slot needs role Parameters",
        ),
        (
            "Recognizer",
            "DigitNet",
            |declaration| {
                declaration["descriptor"]["layers"][0]["weights"] = json!({"name": "hiddenBias"});
                declaration["descriptor"]["layers"][0]["bias"] = json!({"name": "hiddenWeights"});
            },
            "the slot needs an integer tensor of shape [12, 7]",
        ),
        (
            "Recognizer",
            "DigitNet",
            |declaration| {
                declaration["descriptor"]["decoder"]["labels"][0] =
                    json!({"kind": "bool", "value": true});
            },
            "realization `DigitNet` label 1 has type Bool, expected Nat",
        ),
        (
            "Recognizer",
            "DigitNet",
            |declaration| declaration["descriptor"]["layers"][2]["minimum"] = json!("200"),
            "clamps to the empty range [200, 127]",
        ),
        (
            "Triage",
            "TriageScore",
            |declaration| {
                declaration["descriptor"]["labels"]
                    .as_array_mut()
                    .expect("labels")
                    .pop();
            },
            "the slot needs an integer tensor of shape [2, 3]",
        ),
        (
            "Triage",
            "TriageScore",
            |declaration| declaration["descriptor"]["width"] = json!(4),
            "the slot needs an integer tensor of shape [3, 4]",
        ),
        (
            "Policy",
            "PolicyRules",
            |declaration| {
                declaration["descriptor"]["rules"][0]["action"] =
                    json!({"kind": "bool", "value": true});
            },
            "rule `dispatch` action has type Bool, expected Nat",
        ),
        (
            "Policy",
            "PolicyRules",
            |declaration| {
                declaration["descriptor"]["rules"][0]["guard"] =
                    json!({"kind": "nat", "value": "1"});
            },
            "rule `dispatch` guard has type Nat, expected Bool",
        ),
        (
            "Pipeline",
            "SegmentCounter",
            |declaration| {
                declaration["descriptor"]["body"] = json!({"kind": "bool", "value": true});
            },
            "realization `SegmentCounter` denotation has type Bool, expected Nat",
        ),
    ];
    for (module, name, edit, message) in edits {
        refused(&mutated(module, name, edit), "LLT4006", message);
    }
    negative(
        "model-artifact-shape-mismatch",
        "LLT4006",
        "the slot needs an integer tensor",
    );
    negative(
        "model-encoder-width-forged",
        "LLT4006",
        "does not state exactly the generated obligation",
    );
}

/// §17.12: a model binds a contract to a realization of the identical
/// interface; Lean refuses behavior that violates a claim.
fn md_05() {
    let snapshot = snapshot(&P::copy_example(EXAMPLE));
    let model = elaboration(&snapshot, "Recognizer", "DigitModel");
    assert_eq!(names(model.declarations()), ["DigitModel"]);
    let body = model.declarations()[0]["body"].to_string();
    assert!(body.contains("\"name\":\"DigitNet\""), "{body}");
    let pipeline = elaboration(&snapshot, "Pipeline", "PipelineModel");
    assert_eq!(pipeline.obligations().len(), 1, "one entry obligation");
    assert_eq!(
        pipeline.obligations()[0]["theorem"]["name"],
        "pipeline_entry"
    );
    let mismatch = mutated("Pipeline", "ReportModel", |declaration| {
        declaration["realization"] = json!({"member": {"name": "SegmentCounter"}});
    });
    refused(
        &mismatch,
        "LLT4006",
        "realization `SegmentCounter` does not have the interface of contract `ReportContract`",
    );
    let not_realization = mutated("Recognizer", "DigitModel", |declaration| {
        declaration["realization"] = json!({"member": {"module": "Glyphs", "name": "digitOf"}});
    });
    refused(&not_realization, "LLT4006", "is not a prior realization");
    let no_entry = mutated("Pipeline", "PipelineModel", |declaration| {
        declaration.as_object_mut().expect("object").remove("entry");
    });
    refused(
        &no_entry,
        "LLT4007",
        "lists 0 entry theorem(s); its realization has 1 effective precondition(s)",
    );
    let wrong_entry = mutated("Pipeline", "PipelineModel", |declaration| {
        declaration["entry"] = json!([{"name": "triage_actionable"}]);
    });
    refused(
        &wrong_entry,
        "LLT4007",
        "entry 1: `triage_actionable` does not state exactly the generated obligation",
    );
    negative(
        "model-contract-realization-mismatch",
        "LLT4006",
        "does not have the interface of contract",
    );
    negative("model-missing-entry", "LLT4007", "entry theorem(s)");
    // The interface is linked, the behavior is Lean's: the fixture states a
    // contract claim its realization violates, and only the kernel can
    // refuse it.
    let project = P::negative("model-false-evidence");
    project.check_ok();
    if support::lean_backed("MD-05") {
        let _guard = support::env_lock();
        project.verify_fails_with("LLV7002");
    }
}

/// §17.12: claims are closed, generated, statement-exact, restated in Lean,
/// and only the attestation records verification.
#[allow(clippy::too_many_lines)]
fn md_06() {
    let project = P::copy_example(EXAMPLE);
    let snapshot = snapshot(&project);
    let evidence = elaboration(&snapshot, "Recognizer", "DigitEvidence");
    assert!(evidence.declarations().is_empty(), "evidence is proof only");
    let roles: Vec<&str> = evidence
        .obligations()
        .iter()
        .map(|obligation| obligation["role"].as_str().expect("role"))
        .collect();
    assert_eq!(
        roles,
        [
            "evidence `DigitEvidence` claim dataset_agreement",
            "evidence `DigitEvidence` claim dataset_agreement comparison",
            "evidence `DigitEvidence` claim equivalent_to",
            "evidence `DigitEvidence` claim satisfies_contract",
        ]
    );
    assert_eq!(
        names(evidence.cross_checks()),
        [
            "DigitEvidence.labeled_agreement",
            "DigitEvidence.digit_net_exact",
            "DigitEvidence.digit_net_correct",
        ]
    );
    let build = support::rendered(&project);
    let lean = support::lean_text(&build, "Recognizer");
    for line in [
        "public theorem DigitEvidence.labeled_agreement : LexLeanModels.Agreement ((Models.Recognizer.DigitNet)) ((Models.Recognizer.sameDigit)) (Models.Recognizer.labeledGlyphs) (40) (37) := by decide",
        "public theorem DigitEvidence.digit_net_exact : LexLeanModels.EquivalentTotal ((Models.Recognizer.DigitNet)) ((Models.Glyphs.digitOf)) := Models.Recognizer.digit_net_exact",
        "public theorem DigitEvidence.digit_net_correct : LexLeanModels.SatisfiesTotal ((Models.Recognizer.Recognizes)) ((Models.Recognizer.DigitNet)) := Models.Recognizer.digit_net_correct",
    ] {
        assert!(lean.contains(line), "missing cross-check {line}");
    }
    // The document says a claim is discharged, never that it is verified.
    let tex = support::tex_text(&build, "Recognizer");
    assert!(tex.contains(
        "Claim: \\texttt{satisfies the postcondition under the contract's premises, discharged by digit\\_net\\_correct}"
    ));
    for module in MODULES {
        let tex = support::tex_text(&build, module);
        assert!(
            !tex.to_lowercase().contains("verified"),
            "the canonical document of {module} claims no verification"
        );
    }
    let edits: [NamedEdit; 9] = [
        (
            "unsorted claims",
            |declaration| {
                declaration["claims"]
                    .as_array_mut()
                    .expect("claims")
                    .reverse();
            },
            "claims are not strictly sorted by kind and theorem",
        ),
        (
            "a duplicated claim",
            |declaration| {
                let claim = declaration["claims"][2].clone();
                declaration["claims"]
                    .as_array_mut()
                    .expect("claims")
                    .push(claim);
            },
            "claims are not strictly sorted by kind and theorem",
        ),
        (
            "an inexact theorem",
            |declaration| declaration["claims"][2]["theorem"] = json!({"name": "digit_net_exact"}),
            "claim satisfies_contract: `digit_net_exact` does not state exactly the generated obligation",
        ),
        (
            "an inflated agreement",
            |declaration| declaration["claims"][0]["agreements"] = json!(38),
            "claim dataset_agreement: `labeled_agreement` does not state exactly",
        ),
        (
            "an impossible agreement",
            |declaration| declaration["claims"][0]["agreements"] = json!(41),
            "41 agreements cannot be counted over 40 examples",
        ),
        (
            "a different reference",
            |declaration| {
                declaration["claims"][1]["reference"] =
                    json!({"member": {"module": "Glyphs", "name": "segmentCount"}});
            },
            "claim equivalent_to: `digit_net_exact` does not state exactly",
        ),
        (
            "a foreign theorem",
            |declaration| {
                declaration["claims"][1]["theorem"] =
                    json!({"module": "Glyphs", "name": "encode_width"});
            },
            "`Glyphs::encode_width` must be a prior theorem of this module",
        ),
        (
            "another contract",
            |declaration| {
                declaration["contract"] =
                    json!({"member": {"module": "Pipeline", "name": "SegmentContract"}});
            },
            "is not a prior contract",
        ),
        (
            "an invariant claim of a stateless contract",
            |declaration| {
                declaration["claims"]
                    .as_array_mut()
                    .expect("claims")
                    .insert(2, json!({"kind": "preserves_invariant", "theorem": {"name": "digit_net_correct"}}));
            },
            "a stateless contract has no invariant",
        ),
    ];
    for (what, edit, message) in edits {
        let project = mutated("Recognizer", "DigitEvidence", edit);
        let error = project.check_fails_with("LLT4009");
        assert!(
            error.to_string().contains(message),
            "{what}: expected {message:?}, got {error}"
        );
        project.assert_no_backend_output_with("LLT4009");
    }
    // A claim must be stated by a theorem that precedes the evidence.
    let later = P::copy_example(EXAMPLE);
    let mut data = module_data(&later, "Recognizer");
    let declarations = data["declarations"].as_array_mut().expect("declarations");
    let at = declarations
        .iter()
        .position(|declaration| declaration["name"] == "digit_net_correct")
        .expect("theorem");
    let theorem = declarations.remove(at);
    let evidence_at = declarations
        .iter()
        .position(|declaration| declaration["name"] == "DigitEvidence")
        .expect("evidence");
    declarations.insert(evidence_at + 1, theorem);
    write_module_data(&later, "Recognizer", &data);
    refused(
        &later,
        "LLT4009",
        "`digit_net_correct` is not a prior theorem of this module",
    );
    negative("model-forged-evidence", "LLT4009", "does not state exactly");
    negative(
        "model-vacuous-claim",
        "LLT4009",
        "a stateless contract has no invariant",
    );
    negative(
        "model-foreign-evidence",
        "LLT4009",
        "is about another contract or realization",
    );
    negative(
        "model-unregistered-claim",
        "LLT4001",
        "unknown variant `calibrated_confidence`",
    );
    if let Some(fixture) = models_backed("MD-06") {
        for name in [
            "Models.Recognizer.DigitEvidence.labeled_agreement",
            "Models.Recognizer.DigitEvidence.digit_net_exact",
            "Models.Recognizer.DigitEvidence.digit_net_correct",
            "Models.Session.SessionEvidence.session_initial",
            "Models.Session.SessionEvidence.session_preserves",
        ] {
            assert_attested(fixture, name, "allow");
        }
        assert_attested(fixture, "Models.Recognizer.digit_net_correct", "exact");
        assert!(
            fixture.attestation["declarations"]
                .as_array()
                .expect("declarations")
                .iter()
                .all(|row| row["name"] != "Models.Recognizer.DigitEvidence"),
            "evidence is attested through its generated declarations only"
        );
        assert_eq!(fixture.attestation["status"], "verified");
        // The owner's policy is exact over its elaboration as a whole: an
        // axiom no generated declaration observes is refused.
        let overstated = mutated("Recognizer", "DigitEvidence", |declaration| {
            declaration["axioms"] = json!(["Classical.choice", "propext"]);
        });
        overstated.check_ok();
        let _guard = support::env_lock();
        let error = overstated.verify_fails_with("LLV7005");
        assert!(
            error
                .to_string()
                .contains("its generated declarations observe exactly a different set"),
            "{error}"
        );
    }
}

/// §17.12: executable code validates every undischarged predicate in the
/// fixed order and returns the violation.
#[allow(clippy::too_many_lines)]
fn md_07() {
    let project = P::copy_example(EXAMPLE);
    let snapshot = snapshot(&project);
    for (module, name, required) in [
        ("Recognizer", "DigitModel", &[][..]),
        ("Recognizer", "RawDigitModel", &["postcondition"][..]),
        (
            "Session",
            "SessionModel",
            &["input_invariant", "precondition"][..],
        ),
        ("Pipeline", "PipelineModel", &["precondition"][..]),
        ("Pipeline", "StreamModel", &["precondition"][..]),
        // Evidence discharges a check only through the claim that states
        // it: a dataset agreement or an equivalence discharges nothing, and
        // satisfying the contract does not preserve the invariant.
        (
            "Ledger",
            "GuessModel",
            &["postcondition", "precondition"][..],
        ),
        (
            "Ledger",
            "GuessExactModel",
            &["postcondition", "precondition"][..],
        ),
        (
            "Ledger",
            "SpillModel",
            &["input_invariant", "output_invariant", "precondition"][..],
        ),
        (
            "Ledger",
            "SpillRawModel",
            &[
                "input_invariant",
                "output_invariant",
                "postcondition",
                "precondition",
            ][..],
        ),
        ("Ledger", "TallyModel", &["precondition"][..]),
        ("Ledger", "ClampModel", &["input_invariant"][..]),
        ("Ledger", "FreeModel", &[][..]),
        (
            "Ledger",
            "OvershootModel",
            &["postcondition", "precondition"][..],
        ),
        (
            "Flows",
            "FlowModel",
            &["input_invariant", "output_invariant"][..],
        ),
    ] {
        assert_eq!(
            elaboration(&snapshot, module, name).required_checks(),
            required,
            "{name}"
        );
    }
    // The stateful application with every check nests them in the fixed
    // order (canonical JSON writes a conditional's refusal before the rest),
    // the output invariant and the postcondition over the model's step.
    let full = elaboration(&snapshot, "Main", "ledgerFull");
    let body = full.declarations()[0]["body"].to_string();
    let order: Vec<usize> = [
        "ContractViolation.input_invariant",
        "ContractViolation.precondition",
        "ContractViolation.output_invariant",
        "ContractViolation.postcondition",
    ]
    .iter()
    .map(|needle| {
        body.find(needle)
            .unwrap_or_else(|| panic!("`ledgerFull` elaborates {needle}: {body}"))
    })
    .collect();
    let mut sorted = order.clone();
    sorted.sort_unstable();
    assert_eq!(order, sorted, "checks run in the fixed order: {body}");
    for (name, edit, message) in [
        (
            "guessChecked",
            (|declaration: &mut Json| declaration["body"]["checks"] = json!(["precondition"]))
                as Edit,
            "without the runtime checks [postcondition] its evidence does not discharge",
        ),
        (
            "guessChecked",
            |declaration: &mut Json| {
                declaration["body"] = json!({
                    "kind": "constructor", "constructor": {"name": "Result.ok"},
                    "type_arguments": [{"kind": "nat"}, {"kind": "contract_violation"}],
                    "arguments": [{"kind": "call", "function": {"module": "Ledger", "name": "GuessModel"},
                                   "arguments": [{"kind": "var", "name": "x"}]}]});
            },
            "applies model `Ledger::GuessModel` without its runtime checks [postcondition, precondition]",
        ),
        (
            "ledgerPost",
            |declaration: &mut Json| {
                declaration["body"]["checks"] = json!(["input_invariant", "precondition"]);
            },
            "without the runtime checks [output_invariant] its evidence does not discharge",
        ),
    ] {
        refused(&mutated("Main", name, edit), "LLT4008", message);
    }
    // The fixed order: input invariant, precondition, run, ... ; each
    // refusal is the violation of the predicate that failed.
    let step = elaboration(&snapshot, "Main", "step");
    let body = step.declarations()[0]["body"].to_string();
    let order: Vec<usize> = [
        "\"name\":\"Session::boundedCheck\"",
        "ContractViolation.input_invariant",
        "\"name\":\"Session::affordableCheck\"",
        "ContractViolation.precondition",
        "\"name\":\"Session::SessionModel\"",
    ]
    .iter()
    .map(|needle| {
        body.find(needle)
            .or_else(|| body.find(&needle.replace("Session::", "")))
            .unwrap_or_else(|| panic!("`step` elaborates {needle}: {body}"))
    })
    .collect();
    let mut sorted = order.clone();
    sorted.sort_unstable();
    assert_eq!(order, sorted, "checks run in the fixed order: {body}");
    let checked = elaboration(&snapshot, "Main", "classifyChecked");
    let body = checked.declarations()[0]["body"].to_string();
    assert!(body.contains("recognizesCheck") && body.contains("ContractViolation.postcondition"));
    let edits: [NamedEdit; 7] = [
        (
            "classifyChecked",
            |declaration| declaration["body"]["checks"] = json!([]),
            "applies model `Recognizer::RawDigitModel` without the runtime checks [postcondition] its evidence does not discharge",
        ),
        (
            "classify",
            |declaration| declaration["body"]["function"]["name"] = json!("RawDigitModel"),
            "applies model `Recognizer::RawDigitModel` without its runtime checks [postcondition]; apply it with checked_apply",
        ),
        (
            "classify",
            |declaration| {
                declaration["body"] = json!({
                    "kind": "apply",
                    "function": {"kind": "function_ref",
                                 "function": {"module": "Recognizer", "name": "RawDigitModel"}},
                    "arguments": [{"kind": "var", "name": "glyph"}],
                });
            },
            "applies model `Recognizer::RawDigitModel` without its runtime checks",
        ),
        (
            "classify",
            |declaration| declaration["body"]["function"]["name"] = json!("DigitNet"),
            "applies the realization function `Recognizer::DigitNet` directly",
        ),
        (
            "step",
            |declaration| declaration["body"]["checks"] = json!(["precondition", "input_invariant"]),
            "with checks that are not strictly sorted",
        ),
        (
            "step",
            |declaration| declaration["body"]["checks"] = json!(["precondition"]),
            "without the runtime checks [input_invariant] its evidence does not discharge",
        ),
        (
            "classifyChecked",
            |declaration| declaration["body"]["checks"] = json!(["postcondition", "precondition"]),
            "checks the precondition of model `Recognizer::RawDigitModel`, which its contract does not validate with a sound validator",
        ),
    ];
    for (name, edit, message) in edits {
        refused(&mutated("Main", name, edit), "LLT4008", message);
    }
    // Formal code may state anything about an unvalidated model.
    let formal = P::copy_example(EXAMPLE);
    let mut data = module_data(&formal, "Main");
    data["declarations"]
        .as_array_mut()
        .expect("declarations")
        .push(json!({
            "axioms": ["propext"],
            "kind": "theorem",
            "name": "raw_eight",
            "parameters": [],
            "proof": {"kind": "decide"},
            "statement": {"kind": "eq",
                "left": {"kind": "call",
                         "function": {"module": "Recognizer", "name": "RawDigitModel"},
                         "arguments": [{"kind": "constructor", "arguments": [],
                                        "constructor": {"module": "Glyphs", "name": "Glyph.g8"}}]},
                "right": {"kind": "nat", "value": "8"}},
        }));
    write_module_data(&formal, "Main", &data);
    formal.check_ok();
    negative(
        "model-unvalidated-boundary",
        "LLT4008",
        "apply it with checked_apply",
    );
    negative(
        "model-unvalidated-postcondition",
        "LLT4008",
        "its evidence does not discharge",
    );
    negative(
        "model-check-without-validator",
        "LLT4008",
        "does not validate with a sound validator",
    );
    negative(
        "model-realization-bypass",
        "LLT4008",
        "applies the realization function `Doubler` directly",
    );
    // The refusal is kernel-checked: admitting a stream with an
    // unaffordable cost returns the scan's violation.
    if let Some(fixture) = models_backed("MD-07") {
        assert_attested(fixture, "Models.Main.admit_rejects_costly", "none");
        assert_attested(fixture, "Models.Main.admit_accepts_cheap", "none");
        assert_attested(fixture, "Models.Main.classify_eight", "exact");
        assert_attested(fixture, "Models.Main.step", "none");
        // Every refusal is kernel-checked: input invariant, precondition,
        // output invariant (stateful), and postcondition.
        for name in [
            "Models.Main.ledger_accepts",
            "Models.Main.ledger_refuses_input",
            "Models.Main.ledger_refuses_precondition",
            "Models.Main.ledger_refuses_output",
            "Models.Main.ledger_full_accepts",
            "Models.Main.ledger_full_refuses_output",
            "Models.Main.guess_accepts",
            "Models.Main.guess_refuses_precondition",
            "Models.Main.overshoot_refuses_postcondition",
        ] {
            assert_eq!(attested(fixture, name)["result"], "ok", "{name}");
        }
    }
}

/// §17.12: the closed composition forms and their junctions.
#[allow(clippy::too_many_lines)]
fn md_08() {
    let project = P::copy_example(EXAMPLE);
    let snapshot = snapshot(&project);
    for (name, needle) in [
        ("TriagePipeline", "Policy::PolicyModel"),
        ("CheckedPipeline", "ContractViolation.precondition"),
        ("GlyphReport", "\"kind\":\"pair\""),
        ("GlyphPair", "\"kind\":\"first\""),
        ("GlyphRoute", "isEight"),
        ("SessionStream", "list_fold"),
    ] {
        let elaborated = elaboration(&snapshot, "Pipeline", name);
        let text = elaborated
            .declarations()
            .iter()
            .map(Json::to_string)
            .collect::<String>();
        assert!(
            text.contains(needle) || text.contains(&needle.replace("Policy::", "")),
            "{name} elaborates an ordinary composition mentioning {needle}: {text}"
        );
    }
    let sequence = elaboration(&snapshot, "Pipeline", "TriagePipeline");
    assert_eq!(sequence.obligations().len(), 1, "one proved junction");
    assert_eq!(
        sequence.obligations()[0]["theorem"]["name"],
        "triage_actionable"
    );
    let edits: [NamedEdit; 11] = [
        (
            "TriagePipeline",
            |declaration| {
                declaration["descriptor"]["form"]["junctions"] = json!([{"kind": "unconditional"}]);
            },
            "junction 1: stage `Policy::PolicyModel` has a precondition, so the junction must be proved or checked",
        ),
        (
            "TriagePipeline",
            |declaration| declaration["descriptor"]["form"]["junctions"] = json!([]),
            "found 2 stage(s) and 0 junction(s)",
        ),
        (
            "TriagePipeline",
            |declaration| {
                declaration["descriptor"]["form"]["junctions"] =
                    json!([{"kind": "proved", "evidence": {"name": "pipeline_entry"}}]);
            },
            "junction 1: `pipeline_entry` is not a prior theorem of this module",
        ),
        (
            "TriagePipeline",
            |declaration| {
                declaration["descriptor"]["form"]["stages"]
                    .as_array_mut()
                    .expect("stages")
                    .reverse();
            },
            "the first stage input is Triage.Level, expected Triage.Presentation",
        ),
        (
            "GlyphRoute",
            |declaration| {
                declaration["descriptor"]["form"]["then"] =
                    json!({"member": {"module": "Session", "name": "SessionModel"}});
            },
            "its stateful stages thread a state of type Session.Window, which it must declare",
        ),
        (
            "SessionStream",
            |declaration| {
                declaration["descriptor"]["form"]["stage"] = json!({"member": {"name": "SegmentModel"}});
            },
            "stage `SegmentModel` is stateless; a scan threads one stateful stage",
        ),
        (
            "SessionStream",
            |declaration| {
                declaration["descriptor"]["form"]["junction"] = json!({"kind": "unconditional"});
            },
            "the output is Result (List (Nat)) (ContractViolation), expected List (Nat)",
        ),
        (
            "GlyphRoute",
            |declaration| {
                declaration["descriptor"]["form"]["guard"] = json!({"kind": "nat", "value": "1"});
            },
            "branch guard has type Nat, expected Bool",
        ),
        (
            "GlyphRoute",
            |declaration| {
                declaration["descriptor"]["form"]["then"] = json!({"member": {"name": "ReportModel"}});
            },
            "the then stage output is Prod (Nat) (Nat), expected Nat",
        ),
        (
            "GlyphReport",
            |declaration| {
                declaration["descriptor"]["form"]["left"] =
                    json!({"member": {"module": "Recognizer", "name": "RawDigitModel"}});
            },
            "the output is Prod (Nat) (Nat), expected Result (Prod (Nat) (Nat)) (ContractViolation)",
        ),
        (
            "GlyphPair",
            |declaration| {
                declaration["descriptor"]["form"]["right"] = json!({"member": {"name": "SegmentCounter"}});
            },
            "`SegmentCounter` is not a prior model",
        ),
    ];
    for (name, edit, message) in edits {
        refused(&mutated("Pipeline", name, edit), "LLT4007", message);
    }
    // A stateful sequence threads the right-nested product of its stateful
    // stages' states; a stage whose evidence leaves its postcondition or
    // output invariant open is checked right after it runs, and its
    // precondition at a checked junction.
    let flow = elaboration(&snapshot, "Flows", "ClampGuess");
    let text = flow
        .declarations()
        .iter()
        .map(Json::to_string)
        .collect::<String>();
    for needle in [
        "\"name\":\"__state1\"",
        "Ledger::withinCheck",
        "Ledger::belowCheck",
        "ContractViolation.postcondition",
        "ContractViolation.precondition",
    ] {
        assert!(
            text.contains(needle) || text.contains(&needle.replace("Ledger::", "")),
            "ClampGuess elaborates {needle}: {text}"
        );
    }
    let stream = elaboration(&snapshot, "Flows", "SpillStream")
        .declarations()
        .iter()
        .map(Json::to_string)
        .collect::<String>();
    assert!(
        stream.contains("ContractViolation.output_invariant"),
        "{stream}"
    );
    let entry = elaboration(&snapshot, "Flows", "FlowModel");
    assert_eq!(
        entry.obligations()[0]["parameters"]
            .as_array()
            .expect("parameters")
            .len(),
        2,
        "a stateful model's entry obligation is over its state and input"
    );
    let flows: [NamedEdit; 7] = [
        (
            "ClampGuess",
            |declaration| {
                declaration.as_object_mut().expect("object").remove("state");
            },
            "its stateful stages thread a state of type Nat, which it must declare",
        ),
        (
            "ClampGuess",
            |declaration| declaration["state"]["type"] = json!({"kind": "int"}),
            "the state is Int, expected Nat",
        ),
        (
            "ClampGuess",
            |declaration| {
                declaration["descriptor"]["form"]["junctions"] = json!([{"kind": "unconditional"}]);
            },
            "has a precondition, so the junction must be proved or checked",
        ),
        (
            "ClampGuess",
            |declaration| {
                declaration["descriptor"]["form"]["stages"] = json!([
                    {"member": {"module": "Ledger", "name": "GuessModel"}},
                    {"member": {"module": "Ledger", "name": "ClampModel"}}]);
                declaration["descriptor"]["form"]["junctions"] = json!([{"kind": "unconditional"}]);
            },
            "its realization has 2 effective precondition(s)",
        ),
        (
            "GuessTwice",
            |declaration| {
                declaration["descriptor"]["form"]["left"] =
                    json!({"member": {"module": "Ledger", "name": "ClampModel"}});
            },
            "its stateful stages thread a state of type Nat, which it must declare",
        ),
        (
            "GuessTwice",
            |declaration| {
                declaration["output"] =
                    json!({"kind": "product", "left": {"kind": "nat"}, "right": {"kind": "nat"}})
            },
            "expected Result (Prod (Nat) (Nat)) (ContractViolation)",
        ),
        (
            "SpillStream",
            |declaration| {
                declaration["descriptor"]["form"]["junction"] = json!({"kind": "unconditional"})
            },
            "needs its output_invariant checked at run time, so the junction must be checked",
        ),
    ];
    for (name, edit, message) in flows {
        refused(&mutated("Flows", name, edit), "LLT4007", message);
    }
    // Stateful stages in parallel, in branches, and across a proved
    // junction: each composite threads the product of its stages' states.
    for (name, needles) in [
        ("ClampTally", &["__left_state", "__right_state"][..]),
        ("ClampBoth", &["__left_state", "__right_state"][..]),
        ("ClampOrTally", &["__then_state", "__else_state"][..]),
        ("SpillTally", &["__state1", "__state2"][..]),
    ] {
        let text = elaboration(&snapshot, "Flows", name)
            .declarations()
            .iter()
            .map(Json::to_string)
            .collect::<String>();
        for needle in needles {
            assert!(text.contains(needle), "{name} threads {needle}: {text}");
        }
    }
    let chain = elaboration(&snapshot, "Flows", "SpillTally");
    assert_eq!(chain.obligations()[0]["theorem"]["name"], "spill_tally");
    assert_eq!(
        chain.obligations()[0]["parameters"]
            .as_array()
            .expect("parameters")
            .len(),
        2,
        "a proved junction between stateful stages is over the state and the value"
    );
    // A checked junction before a stateful stage checks that stage's
    // invariant and precondition at run time, so neither is an entry
    // requirement: `RawChainModel` has none.
    let raw = elaboration(&snapshot, "Flows", "TallySpillRaw")
        .declarations()
        .iter()
        .map(Json::to_string)
        .collect::<String>();
    for needle in [
        "ContractViolation.input_invariant",
        "ContractViolation.precondition",
        "ContractViolation.output_invariant",
        "ContractViolation.postcondition",
    ] {
        assert!(
            raw.contains(needle),
            "TallySpillRaw elaborates {needle}: {raw}"
        );
    }
    assert!(
        elaboration(&snapshot, "Flows", "RawChainModel")
            .obligations()
            .is_empty(),
        "a checked junction's invariant and precondition are not entry requirements"
    );
    let branch = elaboration(&snapshot, "Flows", "BranchModel");
    assert_eq!(
        branch.obligations().len(),
        2,
        "one entry per guarded arm requirement"
    );
    let stateful: [NamedEdit; 5] = [
        (
            "ClampTally",
            |declaration| declaration["state"]["type"] = json!({"kind": "nat"}),
            "the state is Nat, expected Prod (Nat) (Nat)",
        ),
        (
            "TwinModel",
            |declaration| declaration["entry"] = json!([{"name": "twin_entry_left"}]),
            "lists 1 entry theorem(s); its realization has 2 effective precondition(s)",
        ),
        (
            "BranchModel",
            |declaration| {
                declaration["entry"] =
                    json!([{"name": "branch_entry_else"}, {"name": "branch_entry_then"}]);
            },
            "entry 1: `branch_entry_else` does not state exactly the generated obligation",
        ),
        (
            "SpillTally",
            |declaration| {
                declaration["descriptor"]["form"]["junctions"] =
                    json!([{"kind": "proved", "evidence": {"name": "twin_small_check_sound"}}]);
            },
            "junction 1: `twin_small_check_sound` does not state exactly the generated obligation",
        ),
        (
            "ClampOrTally",
            |declaration| {
                declaration["descriptor"]["form"]["else"] =
                    json!({"member": {"module": "Ledger", "name": "GuessModel"}});
            },
            "the state is Prod (Nat) (Nat), expected Nat",
        ),
    ];
    for (name, edit, message) in stateful {
        refused(&mutated("Flows", name, edit), "LLT4007", message);
    }
    // A stage whose open postcondition has no validator cannot compose.
    let unvalidated = mutated("Ledger", "GuessContract", |declaration| {
        declaration["validators"]
            .as_array_mut()
            .expect("validators")
            .remove(0);
    });
    refused(
        &unvalidated,
        "LLT4007",
        "has no evidence discharging its postcondition and no sound validator to check it at run time",
    );
    negative(
        "model-composition-missing-junction",
        "LLT4007",
        "so the junction must be proved or checked",
    );
    negative(
        "model-composition-forged-junction",
        "LLT4007",
        "does not state exactly the generated obligation",
    );
    negative(
        "model-composition-type-mismatch",
        "LLT4007",
        "the output is Nat, expected Prod (Nat) (Nat)",
    );
    if let Some(fixture) = models_backed("MD-08") {
        for name in [
            "Models.Pipeline.TriagePipeline",
            "Models.Pipeline.CheckedPipeline",
            "Models.Pipeline.GlyphReport",
            "Models.Pipeline.GlyphPair",
            "Models.Pipeline.GlyphRoute",
            "Models.Pipeline.SessionStream",
            "Models.Pipeline.PipelineEvidence.pipeline_responds",
        ] {
            assert_attested(fixture, name, "allow");
        }
        for name in [
            "Models.Main.flow_accepts",
            "Models.Main.flow_refuses_junction",
            "Models.Main.flow_refuses_input",
            "Models.Main.twice_accepts",
            "Models.Main.twice_refuses",
            "Models.Main.stream_accepts",
            "Models.Main.stream_refuses_output",
            "Models.Main.overshoot_pair_refuses_postcondition",
            "Models.Main.twin_accepts",
            "Models.Main.twin_refuses_precondition",
            "Models.Main.twin_refuses_input",
            "Models.Main.pair_clamp_accepts",
            "Models.Main.pair_clamp_refuses_input",
            "Models.Main.branch_takes_then",
            "Models.Main.branch_takes_else",
            "Models.Main.branch_refuses_precondition",
            "Models.Main.chain_accepts",
            "Models.Main.chain_refuses_stage_output",
            "Models.Main.chain_refuses_precondition",
            "Models.Main.raw_chain_accepts",
            "Models.Main.raw_chain_refuses_input",
            "Models.Main.raw_chain_refuses_precondition",
            "Models.Main.drain_chain_accepts",
            "Models.Main.drain_chain_refuses_postcondition",
            "Models.Flows.TallySpillRaw",
            "Models.Flows.TallyDrain",
            "Models.Flows.spill_tally",
            "Models.Flows.ClampTally",
            "Models.Flows.ClampBoth",
            "Models.Flows.ClampOrTally",
            "Models.Flows.SpillTally",
            "Models.Flows.ClampGuess",
            "Models.Flows.GuessOvershoot",
            "Models.Flows.GuessTwice",
            "Models.Flows.SpillStream",
        ] {
            assert_eq!(attested(fixture, name)["result"], "ok", "{name}");
        }
        assert_attested(fixture, "Models.Pipeline.triage_actionable", "exact");
        assert_attested(fixture, "Models.Pipeline.pipeline_entry", "none");
    }
}

// ---------------------------------------------------------------------------
// MD-09: the seeded differential.

/// A fixed-seed generator (`SplitMix64`), so every run poses the same
/// networks and a failure reproduces.
struct Seeded(u64);

impl Seeded {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut mixed = self.0;
        mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        mixed ^ (mixed >> 31)
    }

    /// A value in `low..=high`.
    fn within(&mut self, low: i64, high: i64) -> i64 {
        let span = u64::try_from(high - low + 1).expect("a nonempty range");
        low + i64::try_from(self.next() % span).expect("fits")
    }

    fn matrix(&mut self, rows: usize, columns: usize, low: i64, high: i64) -> Vec<Vec<i64>> {
        (0..rows)
            .map(|_| (0..columns).map(|_| self.within(low, high)).collect())
            .collect()
    }

    fn vector(&mut self, length: usize, low: i64, high: i64) -> Vec<i64> {
        (0..length).map(|_| self.within(low, high)).collect()
    }
}

// The independent integer model: plain `i64` arithmetic, written from the
// definitions in SPEC.md §17.12, not from the compiler's kernels.

fn model_dense(weights: &[Vec<i64>], bias: &[i64], values: &[i64]) -> Vec<i64> {
    weights
        .iter()
        .zip(bias)
        .map(|(row, b)| row.iter().zip(values).map(|(w, v)| w * v).sum::<i64>() + b)
        .collect()
}

fn model_relu(values: &[i64]) -> Vec<i64> {
    values.iter().map(|value| (*value).max(0)).collect()
}

/// Rust's `/` truncates toward zero, which is the declared semantics.
fn model_requantize(values: &[i64], shift: u32, minimum: i64, maximum: i64) -> Vec<i64> {
    values
        .iter()
        .map(|value| (value / (1_i64 << shift)).clamp(minimum, maximum))
        .collect()
}

fn model_first_max(values: &[i64]) -> usize {
    let mut best = 0;
    for (index, value) in values.iter().enumerate() {
        if *value > values[best] {
            best = index;
        }
    }
    best
}

/// The exact axioms pinned Lean's `decide` uses for a comparison: string
/// keys are compared through their character lists, whose decidable
/// equality reaches the classical axioms.
const DECIDED: &[&str] = &["propext"];
const STRING_DECIDED: &[&str] = &["Classical.choice", "Quot.sound", "propext"];

const WIDTH: usize = 6;
const HIDDEN: usize = 8;
const CLASSES: usize = 5;
const TIES: usize = 6;
const SAMPLES: usize = 8;

/// One seeded network, its inputs, and its tie-prone first-maximum layer.
struct Differential {
    inputs: Vec<Vec<i64>>,
    hidden_weights: Vec<Vec<i64>>,
    hidden_bias: Vec<i64>,
    shift: u32,
    minimum: i64,
    maximum: i64,
    output_weights: Vec<Vec<i64>>,
    output_bias: Vec<i64>,
    tie_weights: Vec<Vec<i64>>,
    tie_bias: Vec<i64>,
    comparisons: Vec<(Json, Json, bool, &'static [&'static str])>,
}

fn int_literal(value: i64) -> Json {
    json!({"kind": "integer", "representation": "int", "value": value.to_string()})
}

fn nat_literal(value: u64) -> Json {
    json!({"kind": "nat", "value": value.to_string()})
}

fn int_list(values: &[i64]) -> Json {
    values.iter().rev().fold(
        json!({"element": {"kind": "int"}, "kind": "nil"}),
        |tail, head| json!({"head": int_literal(*head), "kind": "cons", "tail": tail}),
    )
}

fn int8_bytes(matrix: &[Vec<i64>]) -> Vec<u8> {
    matrix
        .iter()
        .flatten()
        .map(|value| i8::try_from(*value).expect("an int8 weight").to_le_bytes()[0])
        .collect()
}

fn int32_bytes(vector: &[i64]) -> Vec<u8> {
    vector
        .iter()
        .flat_map(|value| i32::try_from(*value).expect("an int32 bias").to_le_bytes())
        .collect()
}

impl Differential {
    fn seeded(seed: u64) -> Self {
        let mut seeded = Seeded(seed);
        let inputs = (0..SAMPLES).map(|_| seeded.vector(WIDTH, -8, 8)).collect();
        let hidden_weights = seeded.matrix(HIDDEN, WIDTH, -128, 127);
        // Small enough that a requantized value is often inside its range,
        // so truncation, not only clamping, decides the outcome.
        let hidden_bias = seeded.vector(HIDDEN, -300, 300);
        let shift = u32::try_from(seeded.within(4, 7)).expect("small");
        let minimum = seeded.within(-60, -20);
        let maximum = seeded.within(20, 60);
        let output_weights = seeded.matrix(CLASSES, HIDDEN, -128, 127);
        let output_bias = seeded.vector(CLASSES, -100_000, 100_000);
        // The second half of the tie layer repeats the first, so every
        // maximum is reached twice and only the first may be chosen.
        let mut tie_weights = seeded.matrix(TIES / 2, WIDTH, -1, 1);
        tie_weights.extend_from_within(..);
        let mut tie_bias = seeded.vector(TIES / 2, -1, 1);
        tie_bias.extend_from_within(..);
        let mut comparisons = Vec::new();
        for _ in 0..6 {
            let (left, right) = (seeded.within(0, 12), seeded.within(0, 12));
            comparisons.push((
                nat_literal(u64::try_from(left).expect("nat")),
                nat_literal(u64::try_from(right).expect("nat")),
                left < right,
                DECIDED,
            ));
        }
        for _ in 0..6 {
            let (left, right) = (seeded.within(-9, 9), seeded.within(-9, 9));
            comparisons.push((int_literal(left), int_literal(right), left < right, DECIDED));
        }
        for _ in 0..4 {
            let (left, right) = (seeded.within(-128, 127), seeded.within(-128, 127));
            let literal = |value: i64| json!({"kind": "integer", "representation": "int8", "value": value.to_string()});
            comparisons.push((literal(left), literal(right), left < right, DECIDED));
        }
        let word = |seeded: &mut Seeded| -> String {
            let length = seeded.within(0, 3);
            (0..length)
                .map(|_| ['a', 'b', 'é'][usize::try_from(seeded.within(0, 2)).expect("index")])
                .collect()
        };
        for _ in 0..6 {
            let (left, right) = (word(&mut seeded), word(&mut seeded));
            let less = left < right;
            comparisons.push((
                json!({"kind": "string", "value": left}),
                json!({"kind": "string", "value": right}),
                less,
                STRING_DECIDED,
            ));
        }
        for _ in 0..4 {
            let (a, b) = (seeded.within(0, 2), seeded.within(-2, 2));
            let (c, d) = (seeded.within(0, 2), seeded.within(-2, 2));
            let pair = |n: i64, i: i64| {
                json!({"kind": "pair", "left": nat_literal(u64::try_from(n).expect("nat")),
                       "right": int_literal(i)})
            };
            comparisons.push((pair(a, b), pair(c, d), (a, b) < (c, d), DECIDED));
        }
        Self {
            inputs,
            hidden_weights,
            hidden_bias,
            shift,
            minimum,
            maximum,
            output_weights,
            output_bias,
            tie_weights,
            tie_bias,
            comparisons,
        }
    }

    fn logits(&self, input: &[i64]) -> Vec<i64> {
        let hidden = model_dense(&self.hidden_weights, &self.hidden_bias, input);
        let hidden = model_requantize(&model_relu(&hidden), self.shift, self.minimum, self.maximum);
        model_dense(&self.output_weights, &self.output_bias, &hidden)
    }

    /// The hidden layer requantized without the rectifier, so negative
    /// values meet the truncating division.
    fn quantized(&self, input: &[i64]) -> Vec<i64> {
        let hidden = model_dense(&self.hidden_weights, &self.hidden_bias, input);
        model_requantize(&hidden, self.shift, self.minimum, self.maximum)
    }

    fn tie(&self, input: &[i64]) -> usize {
        model_first_max(&model_dense(&self.tie_weights, &self.tie_bias, input))
    }

    fn artifacts(&self) -> [(&'static str, &'static str, Vec<u8>, Json); 6] {
        let tensor = |element: &str, shape: &[usize]| json!({"kind": "int_tensor", "element": element, "shape": shape});
        let matrix_type =
            json!({"kind": "list", "element": {"kind": "list", "element": {"kind": "int"}}});
        let vector_type = json!({"kind": "list", "element": {"kind": "int"}});
        [
            (
                "hiddenWeights",
                "artifacts/hidden-weights.bin",
                int8_bytes(&self.hidden_weights),
                json!({"schema": tensor("int8", &[HIDDEN, WIDTH]), "type": matrix_type}),
            ),
            (
                "hiddenBias",
                "artifacts/hidden-bias.bin",
                int32_bytes(&self.hidden_bias),
                json!({"schema": tensor("int32", &[HIDDEN]), "type": vector_type}),
            ),
            (
                "outputWeights",
                "artifacts/output-weights.bin",
                int8_bytes(&self.output_weights),
                json!({"schema": tensor("int8", &[CLASSES, HIDDEN]), "type": matrix_type}),
            ),
            (
                "outputBias",
                "artifacts/output-bias.bin",
                int32_bytes(&self.output_bias),
                json!({"schema": tensor("int32", &[CLASSES]), "type": vector_type}),
            ),
            (
                "tieWeights",
                "artifacts/tie-weights.bin",
                int8_bytes(&self.tie_weights),
                json!({"schema": tensor("int8", &[TIES, WIDTH]), "type": matrix_type}),
            ),
            (
                "tieBias",
                "artifacts/tie-bias.bin",
                int32_bytes(&self.tie_bias),
                json!({"schema": tensor("int32", &[TIES]), "type": vector_type}),
            ),
        ]
    }

    /// The module: the samples, the artifact-backed networks, and one
    /// theorem per model outcome, each decided by Lean. `skew` adds one to
    /// the first expected logit, which no correct kernel can then meet.
    #[allow(clippy::too_many_lines)]
    fn module(&self, skew: i64) -> String {
        let sample = json!({"arguments": [], "kind": "named", "member": {"name": "Sample"}});
        let constructor = |index: usize| {
            json!({"arguments": [], "constructor": {"name": format!("Sample.s{index}")},
                   "kind": "constructor"})
        };
        let mut declarations = vec![
            json!({
                "constructors": (0..SAMPLES)
                    .map(|index| json!({"fields": [], "name": format!("s{index}")}))
                    .collect::<Vec<_>>(),
                "kind": "inductive",
                "name": "Sample",
                "parameters": [],
                "type_parameters": [],
            }),
            json!({
                "body": {
                    "branches": self.inputs.iter().enumerate().map(|(index, input)| json!({
                        "binders": [],
                        "body": int_list(input),
                        "constructor": {"name": format!("Sample.s{index}")},
                    })).collect::<Vec<_>>(),
                    "kind": "match",
                    "scrutinee": {"kind": "var", "name": "sample"},
                },
                "executable": true,
                "kind": "definition",
                "name": "vectorOf",
                "parameters": [{"name": "sample", "type": sample}],
                "result": {"kind": "list", "element": {"kind": "int"}},
            }),
            json!({
                "kind": "theorem",
                "name": "vector_width",
                "parameters": [{"name": "sample", "type": sample}],
                "proof": {
                    "branches": (0..SAMPLES).map(|index| json!({
                        "binders": [],
                        "constructor": format!("s{index}"),
                        "proof": {"kind": "reflexivity"},
                    })).collect::<Vec<_>>(),
                    "kind": "cases",
                    "scrutinee": "sample",
                },
                "statement": {
                    "kind": "eq",
                    "left": {
                        "arguments": [{"arguments": [{"kind": "var", "name": "sample"}],
                                       "function": {"name": "vectorOf"}, "kind": "call"}],
                        "kind": "primitive", "operation": "length", "result": {"kind": "nat"},
                    },
                    "right": nat_literal(WIDTH as u64),
                },
            }),
        ];
        for (name, _, bytes, shape) in self.artifacts() {
            declarations.push(json!({
                "kind": "artifact",
                "length": bytes.len(),
                "name": name,
                "role": "parameters",
                "schema": shape["schema"],
                "sha256": digest(&bytes),
                "type": shape["type"],
            }));
        }
        let encoder = json!({"arguments": [{"kind": "var", "name": "sample"}],
                             "function": {"name": "vectorOf"}, "kind": "call"});
        let layers = json!([
            {"bias": {"name": "hiddenBias"}, "inputs": WIDTH, "kind": "dense",
             "outputs": HIDDEN, "weights": {"name": "hiddenWeights"}},
            {"kind": "relu"},
            {"kind": "requantize", "maximum": self.maximum.to_string(),
             "minimum": self.minimum.to_string(), "shift": self.shift},
            {"bias": {"name": "outputBias"}, "inputs": HIDDEN, "kind": "dense",
             "outputs": CLASSES, "weights": {"name": "outputWeights"}},
        ]);
        let realization = |name: &str, layers: &Json, decoder: Json, output: Json| {
            json!({
                "axioms": ["propext"],
                "descriptor": {
                    "architecture": "integer_feedforward",
                    "decoder": decoder,
                    "encoder": encoder,
                    "kind": "neural",
                    "layers": layers,
                    "width": WIDTH,
                    "width_evidence": {"name": "vector_width"},
                },
                "executable": true,
                "input": {"name": "sample", "type": sample},
                "kind": "realization",
                "name": name,
                "output": output,
            })
        };
        declarations.push(realization(
            "Logits",
            &layers,
            json!({"binder": "values", "body": {"kind": "var", "name": "values"},
                   "kind": "function"}),
            json!({"kind": "list", "element": {"kind": "int"}}),
        ));
        declarations.push(realization(
            "Quantized",
            &json!([
                {"bias": {"name": "hiddenBias"}, "inputs": WIDTH, "kind": "dense",
                 "outputs": HIDDEN, "weights": {"name": "hiddenWeights"}},
                {"kind": "requantize", "maximum": self.maximum.to_string(),
                 "minimum": self.minimum.to_string(), "shift": self.shift},
            ]),
            json!({"binder": "values", "body": {"kind": "var", "name": "values"},
                   "kind": "function"}),
            json!({"kind": "list", "element": {"kind": "int"}}),
        ));
        declarations.push(realization(
            "Label",
            &layers,
            json!({"kind": "argmax",
                   "labels": (0..CLASSES as u64).map(nat_literal).collect::<Vec<_>>()}),
            json!({"kind": "nat"}),
        ));
        declarations.push(realization(
            "Tie",
            &json!([{"bias": {"name": "tieBias"}, "inputs": WIDTH, "kind": "dense",
                     "outputs": TIES, "weights": {"name": "tieWeights"}}]),
            json!({"kind": "argmax",
                   "labels": (0..TIES as u64).map(nat_literal).collect::<Vec<_>>()}),
            json!({"kind": "nat"}),
        ));
        let theorem = |name: String, left: Json, right: Json| {
            json!({
                "axioms": ["propext"],
                "kind": "theorem",
                "name": name,
                "parameters": [],
                "proof": {"kind": "decide"},
                "statement": {"kind": "eq", "left": left, "right": right},
            })
        };
        let apply = |function: &str, index: usize| {
            json!({"arguments": [constructor(index)], "function": {"name": function},
                   "kind": "call"})
        };
        for (index, input) in self.inputs.iter().enumerate() {
            let mut logits = self.logits(input);
            if index == 0 {
                logits[0] += skew;
            }
            declarations.push(theorem(
                format!("logits_{index}"),
                apply("Logits", index),
                int_list(&logits),
            ));
            declarations.push(theorem(
                format!("quantized_{index}"),
                apply("Quantized", index),
                int_list(&self.quantized(input)),
            ));
            declarations.push(theorem(
                format!("label_{index}"),
                apply("Label", index),
                nat_literal(model_first_max(&self.logits(input)) as u64),
            ));
            declarations.push(theorem(
                format!("tie_{index}"),
                apply("Tie", index),
                nat_literal(self.tie(input) as u64),
            ));
        }
        for (index, (left, right, less, axioms)) in self.comparisons.iter().enumerate() {
            let mut declaration = theorem(
                format!("less_{index}"),
                json!({"arguments": [left, right], "kind": "primitive",
                       "operation": "less_than", "result": {"kind": "bool"}}),
                json!({"kind": "bool", "value": less}),
            );
            declaration["axioms"] = json!(axioms);
            declarations.push(declaration);
        }
        let data = json!({"declarations": declarations, "spec": "lexlean/semantic-module/2"});
        format!(
            "\\begin{{lexlean}}{{Main}}\n\\useglossary{{lexlean.std.nat@1.2.0}}\n\\title{{Natural number addition}}\n\\begin{{semanticmodule}}\n\\semanticdata{{{}}}\n\\end{{semanticmodule}}\n\\end{{lexlean}}\n",
            serde_json::to_string(&data).expect("serializes")
        )
    }

    /// A project holding exactly this network, its artifacts, and its
    /// expectations.
    fn project(&self, skew: i64) -> P {
        let project = P::copy_example(EXAMPLE);
        for entry in std::fs::read_dir(project.root.join("src").as_std_path()).expect("src") {
            std::fs::remove_file(entry.expect("entry").path()).expect("remove module");
        }
        for entry in
            std::fs::read_dir(project.root.join("artifacts").as_std_path()).expect("artifacts")
        {
            std::fs::remove_file(entry.expect("entry").path()).expect("remove artifact");
        }
        let mut sources = Vec::new();
        for (_, path, bytes, _) in self.artifacts() {
            std::fs::write(project.root.join(path).as_std_path(), &bytes).expect("artifact");
            sources.push((digest(&bytes), path.to_owned()));
        }
        set_artifact_sources(&project, sources);
        project.write("src/Main.lex.tex", &self.module(skew));
        project.relock();
        project
    }
}

/// §17.12: the exact integer kernels against an independent model under
/// Lean, on seeded weights and inputs.
fn md_09() {
    let differential = Differential::seeded(0x5eed_0029_0009);
    // The seed poses what it must: a first maximum that is not the last of
    // equal values, a requantization that both truncates and clamps, and
    // both answers of each comparison.
    assert!(
        differential.inputs.iter().any(|input| {
            let values = model_dense(&differential.tie_weights, &differential.tie_bias, input);
            let best = values[model_first_max(&values)];
            values.iter().filter(|value| **value == best).count() > 1
        }),
        "the seed poses a tie"
    );
    let divisor = 1_i64 << differential.shift;
    let pre: Vec<i64> = differential
        .inputs
        .iter()
        .flat_map(|input| {
            model_dense(
                &differential.hidden_weights,
                &differential.hidden_bias,
                input,
            )
        })
        .collect();
    let (low, high) = (differential.minimum, differential.maximum);
    assert!(
        pre.iter().any(|value| {
            let quotient = value / divisor;
            *value < 0 && value % divisor != 0 && low < quotient && quotient < high
        }),
        "the seed poses a negative value that truncation, not flooring, requantizes"
    );
    assert!(
        pre.iter().any(|value| value / divisor < low)
            && pre.iter().any(|value| value / divisor > high),
        "the seed poses clamping at both bounds"
    );
    assert!(
        pre.iter()
            .any(|value| *value > 0 && value / divisor < high && value % divisor != 0),
        "the seed poses a rectified value inside the range"
    );
    assert!(differential.comparisons.iter().any(|(_, _, less, _)| *less));
    assert!(differential
        .comparisons
        .iter()
        .any(|(_, _, less, _)| !*less));
    let project = differential.project(0);
    project.check_ok();
    let snapshot = snapshot(&project);
    let logits = elaboration(&snapshot, "Main", "Logits");
    assert!(logits.declarations().iter().any(|declaration| declaration
        .to_string()
        .contains("\"operation\":\"less_than\"")));
    if support::lean_backed("MD-09") {
        support::verify_ok(&project);
        // The theorems are not vacuous: one wrong expectation is refused.
        let skewed = differential.project(1);
        skewed.check_ok();
        let _guard = support::env_lock();
        skewed.verify_fails_with("LLV7002");
    }
}

/// §17.12, §21.4, §24.1: the semantic identity covers the model
/// declarations, and snapshots carry their elaboration.
#[allow(clippy::too_many_lines)]
fn md_10() {
    let base = P::copy_example(EXAMPLE);
    let id = |project: &P| support::checked_project(project).semantic_id;
    let original = id(&base);
    // A reformatted module is the same semantic module.
    let reformatted = P::copy_example(EXAMPLE);
    let data = module_data(&reformatted, "Recognizer");
    let path = module_path("Recognizer");
    let text = reformatted.read(&path);
    let (start, end) = data_bounds(&text);
    let pretty = serde_json::to_string_pretty(&data).expect("serializes");
    reformatted.write(
        &path,
        &format!("{}{}{}", &text[..start], pretty, &text[end..]),
    );
    if reformatted
        .engine()
        .check(CheckRequest {
            selection: Selection::Entrypoints,
        })
        .is_ok()
    {
        assert_eq!(id(&reformatted), original, "layout is not semantics");
    }
    let mut ids = vec![original];
    // An artifact byte, with its declared digest.
    let bytes = P::copy_example(EXAMPLE);
    replace_artifact(
        &bytes,
        "artifacts/banner.bin",
        "Recognizer",
        "banner",
        b"LLM\x00\x02",
    );
    bytes.check_ok();
    ids.push(id(&bytes));
    // A descriptor.
    let descriptor = mutated("Recognizer", "DigitNet", |declaration| {
        declaration["descriptor"]["layers"][2]["shift"] = json!(3);
    });
    descriptor.check_ok();
    ids.push(id(&descriptor));
    // A claim.
    let claim = mutated("Recognizer", "DigitEvidence", |declaration| {
        declaration["claims"]
            .as_array_mut()
            .expect("claims")
            .remove(1);
    });
    claim.check_ok();
    ids.push(id(&claim));
    // A check: the same model applied with and without a check its
    // evidence already discharges.
    let checked = mutated("Main", "classifyChecked", |declaration| {
        declaration["body"]["model"]["name"] = json!("DigitModel");
    });
    checked.check_ok();
    ids.push(id(&checked));
    let unchecked = mutated("Main", "classifyChecked", |declaration| {
        declaration["body"]["model"]["name"] = json!("DigitModel");
        declaration["body"]["checks"] = json!([]);
    });
    unchecked.check_ok();
    ids.push(id(&unchecked));
    let distinct: BTreeSet<String> = ids.iter().map(Sha256Digest::to_hex).collect();
    assert_eq!(
        distinct.len(),
        ids.len(),
        "every change changes the semantic ID"
    );
    // Snapshots carry, schema-valid, every elaboration.
    let first = snapshot(&base);
    let second = snapshot(&base);
    assert_eq!(first.canonical_bytes(), second.canonical_bytes());
    assert_eq!(first.language(), "1.2");
    let value: Json = serde_json::from_slice(&first.canonical_bytes()).expect("JSON");
    support::assert_schema("semantic-snapshot-v2", "the models snapshot", &value);
    let mut kinds = BTreeSet::new();
    let mut elaborated = 0;
    for module in first.modules() {
        for declaration in module.declarations() {
            if let Some(elaboration) = declaration.elaboration() {
                kinds.insert(declaration.kind().to_owned());
                elaborated += 1;
                assert!(
                    elaboration
                        .declarations()
                        .iter()
                        .all(|derived| derived["kind"] == "definition"
                            || derived["kind"] == "theorem"),
                    "{} elaborates to ordinary declarations",
                    declaration.logical_id()
                );
            }
        }
    }
    assert_eq!(
        kinds,
        BTreeSet::from([
            "artifact".to_owned(),
            "contract".to_owned(),
            "definition".to_owned(),
            "evidence".to_owned(),
            "model".to_owned(),
            "realization".to_owned(),
        ]),
        "every model kind, and a checked application, carries its elaboration"
    );
    assert!(elaborated > 30, "{elaborated} elaborations");
    let artifact = elaboration(&first, "Recognizer", "hiddenWeights");
    assert_eq!(
        names(artifact.declarations()),
        ["hiddenWeights.bytes", "hiddenWeights"],
        "an artifact is its exact bytes and a constant of their decoded value"
    );
    assert_eq!(artifact.cross_checks()[0]["statement"]["kind"], "tensor");
    // Every elaborated definition carries its alpha identity, as a source
    // definition does.
    let net = elaboration(&first, "Recognizer", "DigitNet");
    let defined: Vec<String> = names(net.declarations());
    assert_eq!(
        net.alpha_ids().keys().cloned().collect::<Vec<_>>(),
        {
            let mut sorted = defined.clone();
            sorted.sort();
            sorted
        },
        "one alpha identity per elaborated definition"
    );
    // A reserved generated name is admitted in an elaboration only.
    let mut forged = value.clone();
    let declaration = forged["modules"]
        .as_array_mut()
        .expect("modules")
        .iter_mut()
        .filter_map(|module| module["semantic"]["declarations"].as_array_mut())
        .flatten()
        .find(|declaration| declaration["kind"] == "definition")
        .expect("a source definition");
    declaration["name"] = json!("__forged");
    assert!(
        !crate::schema::validate(&support::schema("semantic-snapshot-v2"), &forged).is_empty(),
        "the snapshot schema rejects a reserved name in a source declaration"
    );
}

/// §17.13, §17.14: production dispositions, the realization table, and
/// eligible artifact-backed roots that Lean extracts with the same closure.
#[allow(clippy::too_many_lines)]
fn md_11() {
    let registry = lexlean::production::registry().expect("registry");
    for (key, disposition) in [
        ("declaration.artifact", "runtime"),
        ("declaration.contract", "erased"),
        ("declaration.realization", "runtime"),
        ("declaration.evidence", "erased"),
        ("declaration.model", "runtime"),
        ("term.checked_apply", "runtime"),
        ("type.contract_violation", "runtime"),
        ("primitive.less_than", "runtime"),
    ] {
        assert_eq!(
            registry.constructs[key].disposition.as_str(),
            disposition,
            "{key} has its reviewed disposition"
        );
    }
    let table: BTreeSet<&str> = lexlean::calculus::realization::TABLE
        .iter()
        .map(|(key, _)| *key)
        .collect();
    for key in [
        "declaration.artifact",
        "declaration.realization",
        "declaration.model",
        "term.checked_apply",
        "type.contract_violation",
        "primitive.less_than",
    ] {
        assert!(table.contains(key), "the realization table covers {key}");
    }
    let project = P::copy_example(EXAMPLE);
    let build = support::rendered(&project);
    let report: Json = serde_json::from_slice(
        &build
            .files
            .iter()
            .find(|(path, _)| path == "production/Models/Main.eligibility.json")
            .expect("Main publishes its eligibility report")
            .1,
    )
    .expect("report JSON");
    let closures: Vec<(String, BTreeSet<String>)> = report["roots"]
        .as_array()
        .expect("roots")
        .iter()
        .map(|root| {
            for target in root["targets"].as_array().expect("targets") {
                assert_eq!(target["status"], "eligible", "{}: {target}", root["root"]);
            }
            (
                root["root"].as_str().expect("root").to_owned(),
                root["runtime_closure"]
                    .as_array()
                    .expect("closure")
                    .iter()
                    .map(|member| {
                        member["declaration"]
                            .as_str()
                            .expect("declaration")
                            .to_owned()
                    })
                    .collect(),
            )
        })
        .collect();
    let roots: Vec<&str> = closures.iter().map(|(root, _)| root.as_str()).collect();
    assert_eq!(
        roots,
        [
            "Models.Main.classify",
            "Models.Main.classifyChecked",
            "Models.Main.respond",
            "Models.Main.admitCosts",
            "Models.Main.step",
            "Models.Main.ledgerPost",
            "Models.Main.ledgerFull",
            "Models.Main.guessChecked",
            "Models.Main.flowStep",
        ]
    );
    for (root, members) in [
        (
            "Models.Main.classify",
            &[
                "Models.Recognizer.DigitModel",
                "Models.Recognizer.DigitNet",
                "Models.Recognizer.hiddenWeights",
                "Models.Recognizer.outputBias",
            ][..],
        ),
        (
            "Models.Main.classifyChecked",
            &[
                "Models.Recognizer.RawDigitModel",
                "Models.Recognizer.recognizesCheck",
                "Models.Recognizer.hiddenWeights",
            ][..],
        ),
        (
            "Models.Main.respond",
            &[
                "Models.Pipeline.PipelineModel",
                "Models.Triage.symptomaticCheck",
                "Models.Triage.triageWeights",
            ][..],
        ),
    ] {
        let closure = &closures
            .iter()
            .find(|(candidate, _)| candidate == root)
            .expect("root")
            .1;
        for member in members {
            assert!(closure.contains(*member), "{root} reaches {member}");
        }
        for erased in [
            "Models.Recognizer.DigitContract",
            "Models.Recognizer.DigitEvidence",
        ] {
            assert!(!closure.contains(erased), "{root} never realizes {erased}");
        }
    }
    let constructs = |root: &str| -> BTreeSet<String> {
        report["roots"]
            .as_array()
            .expect("roots")
            .iter()
            .find(|candidate| candidate["root"] == root)
            .expect("root")["constructs"]
            .as_array()
            .expect("constructs")
            .iter()
            .map(|row| row["construct"].as_str().expect("construct").to_owned())
            .collect()
    };
    let checked_constructs = constructs("Models.Main.classifyChecked");
    for key in [
        "term.checked_apply",
        "type.contract_violation",
        "declaration.model",
        "declaration.realization",
        "declaration.artifact",
        "primitive.less_than",
    ] {
        assert!(
            checked_constructs.contains(key),
            "classifyChecked uses {key}"
        );
    }
    if let Some(fixture) = models_backed("MD-11") {
        let input: Json = serde_json::from_slice(
            &std::fs::read(
                fixture
                    .outcome
                    .root
                    .join("production/compiler-input.json")
                    .as_std_path(),
            )
            .expect("compiler input"),
        )
        .expect("compiler input JSON");
        for (root, members) in &closures {
            let extracted: BTreeSet<String> = input["closures"]
                .as_array()
                .expect("closures")
                .iter()
                .find(|closure| closure["root"] == root.as_str())
                .unwrap_or_else(|| panic!("Lean extracted {root}"))["declarations"]
                .as_array()
                .expect("declarations")
                .iter()
                .map(|name| name.as_str().expect("name").to_owned())
                .collect();
            assert_eq!(
                &extracted, members,
                "{root}: Lean extracts the eligible closure"
            );
        }
    }
}

/// §28: the committed example verifies, and a planted contract and
/// realization mismatch is refused by verification.
fn md_12() {
    let project = P::copy_example(EXAMPLE);
    project.check_ok();
    project.fmt_check_ok();
    // The inventory: every claim and link form of the fixed model semantics
    // is produced by a declaration of the committed example. The names are
    // read from the emitted runtime itself, so a new form is covered or this
    // case fails.
    let build = support::rendered(&project);
    let lean = support::lean_text(&build, "Ledger");
    let runtime = lean
        .split("namespace LexLeanModels\n")
        .nth(1)
        .and_then(|rest| rest.split("end LexLeanModels").next())
        .expect("the emitted model runtime");
    let decoders = BTreeSet::from([
        "encodeInt",
        "inRange",
        "linesMatch",
        "tensorMatches",
        "utf8Char",
    ]);
    let forms: BTreeSet<String> = runtime
        .lines()
        .filter_map(|line| line.split("public def ").nth(1))
        .filter_map(|rest| rest.split_whitespace().next())
        .filter(|name| !decoders.contains(name))
        .map(str::to_owned)
        .collect();
    assert!(forms.len() >= 20, "{forms:?}");
    let snapshot = snapshot(&project);
    let mut produced = BTreeSet::new();
    let mut statements = BTreeSet::new();
    let mut cross_checks = Vec::new();
    let mut constructors = BTreeSet::new();
    for module in snapshot.modules() {
        for declaration in module.declarations() {
            let Some(elaboration) = declaration.elaboration() else {
                continue;
            };
            for check in elaboration.cross_checks() {
                statements.insert(
                    check["statement"]["kind"]
                        .as_str()
                        .expect("kind")
                        .to_owned(),
                );
                if let Some(helper) = check["statement"]["helper"].as_str() {
                    produced.insert(helper.to_owned());
                }
                cross_checks.push(format!(
                    "Models.{}.{}",
                    module.name(),
                    check["name"].as_str().expect("name")
                ));
            }
            for derived in elaboration.declarations() {
                let text = derived.to_string();
                for constructor in VIOLATIONS {
                    if text.contains(constructor) {
                        constructors.insert(constructor);
                    }
                }
            }
        }
    }
    assert_eq!(
        produced, forms,
        "every form of the fixed semantics is produced"
    );
    assert_eq!(
        statements,
        BTreeSet::from([
            "bytes".to_owned(),
            "helper".to_owned(),
            "lines".to_owned(),
            "tensor".to_owned()
        ]),
        "every artifact decoding is restated"
    );
    assert_eq!(
        constructors.len(),
        4,
        "every refusal is generated in executable code: {constructors:?}"
    );
    let Some(fixture) = models_backed("MD-12") else {
        return;
    };
    // ... and every one of them is verified by pinned Lean.
    for name in &cross_checks {
        assert_eq!(attested(fixture, name)["result"], "ok", "{name}");
    }
    assert_eq!(fixture.attestation["status"], "verified");
    // Every declaration, generated or written, passed its axiom policy;
    // the generated ones are allowed what their source declaration states.
    let rows = fixture.attestation["declarations"]
        .as_array()
        .expect("declarations");
    assert!(rows.iter().all(|row| row["result"] == "ok"));
    for name in [
        "Models.Recognizer.hiddenWeights.decoded",
        "Models.Recognizer.DigitNet",
        "Models.Triage.TriageScore",
        "Models.Policy.PolicyRules",
        "Models.Session.SessionStep",
        "Models.Pipeline.SessionStream",
    ] {
        assert_attested(fixture, name, "allow");
    }
    for name in [
        "Models.Recognizer.digit_net_correct",
        "Models.Triage.triage_safe",
        "Models.Session.session_preserves",
        "Models.Pipeline.pipeline_responds",
    ] {
        assert_attested(fixture, name, "exact");
    }
    assert_attested(fixture, "Models.Policy.policy_escalates", "none");
    // Plant the mismatch: digit 8's output bias flips sign, so the network
    // no longer computes the contract's digit for every glyph, and only
    // Lean's kernel can see it.
    let planted = P::copy_example(EXAMPLE);
    let path = planted.root.join("artifacts/output-bias.bin");
    let mut bias = std::fs::read(path.as_std_path()).expect("output bias");
    let original = i32::from_le_bytes(bias[32..36].try_into().expect("four bytes"));
    let flipped = original
        .checked_neg()
        .expect("negatable")
        .saturating_add(if original == 0 { 1_000 } else { 0 });
    bias[32..36].copy_from_slice(&flipped.to_le_bytes());
    replace_artifact(
        &planted,
        "artifacts/output-bias.bin",
        "Recognizer",
        "outputBias",
        &bias,
    );
    planted.check_ok();
    let _guard = support::env_lock();
    planted.verify_fails_with("LLV7002");
}
