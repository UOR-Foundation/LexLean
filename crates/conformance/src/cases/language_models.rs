//! Conformance cases for the language-1.2 text components (SPEC.md §17.10,
//! §17.12): LM-01..LM-03, the tokenizer and its laws; LM-04..LM-06, context
//! windows and greedy and sampled generation; LM-07, classifiers; LM-09, no prompt is code; and LM-13,
//! the laws as LexLean source.
//!
//! Every mutation starts from the committed `models` example, whose
//! `Recognizer` module declares the `utf8_lines` vocabulary artifact
//! `digitNames`; the cases add tokenizers over it and read what linking
//! elaborates them to.

use std::collections::BTreeSet;

use lexlean::text_laws;
use serde_json::json;

use super::models::{
    edit_declaration, elaboration, module_data, names, refused, replace_artifact, snapshot,
    write_module_data,
};
use crate::support::{self, repo_root, P};

/// The module of the example that declares the vocabulary artifact.
const MODULE: &str = "Recognizer";
/// The vocabulary artifact: role `vocabulary`, schema `utf8_lines`.
const VOCABULARY: &str = "digitNames";
/// The Lean module prefix of that module.
const LEAN_MODULE: &str = "Models.Recognizer";

/// Run the case for one language-models ID.
///
/// # Panics
///
/// Panics when the case's assertion fails, and for an ID of this suite whose
/// case is not wired — an unimplemented registered capability must not
/// pass silently.
pub fn run(id: &str) {
    match id {
        "LM-01" => lm_01(),
        "LM-02" => lm_02(),
        "LM-03" => lm_03(),
        "LM-04" => lm_04(),
        "LM-05" => lm_05(),
        "LM-06" => lm_06(),
        "LM-07" => lm_07(),
        "LM-08" => lm_08(),
        "LM-09" => lm_09(),
        "LM-13" => lm_13(),
        _ => panic!("no language-models conformance case is wired for {id}"),
    }
}

/// One tokenizer declaration as the closed schema writes it.
fn tokenizer(name: &str, scheme: &str, vocabulary: &str) -> serde_json::Value {
    json!({
        "kind": "tokenizer",
        "name": name,
        "scheme": scheme,
        "vocabulary": {"member": {"name": vocabulary}},
        "axioms": ["Classical.choice", "Quot.sound", "propext"],
    })
}

/// The directory of the example, below the workspace.
const EXAMPLE_DIR: &str = "examples/models";

/// A copy of the example beside the standard library, the workspace a text
/// declaration needs (§17.12): the project lists the `std` source package and
/// the module that declares the text components imports the module that
/// holds their production forms, as `\importmodule{LexLeanStd.TextModels}`.
/// Neither is in the committed example, which declares no text component.
fn text_workspace() -> P {
    let project = P::workspace_project(EXAMPLE_DIR, &[EXAMPLE_DIR, "stdlib"]);
    let workspace = project.workspace_dir();
    std::fs::write(
        workspace.join("lexlean-workspace.toml").as_std_path(),
        "spec = \"lexlean/workspace/1\"\nmembers = [\"examples/models\", \"stdlib\"]\n",
    )
    .expect("the workspace file");
    let manifest = project.root.join("lexlean.toml");
    let mut text = std::fs::read_to_string(manifest.as_std_path()).expect("manifest");
    text.push_str("\n[[source_package]]\npackage = \"std\"\nversion = \"1.0.0\"\nkind = \"path\"\npath = \"stdlib\"\n");
    std::fs::write(manifest.as_std_path(), text).expect("manifest");
    let module = project.root.join(format!("src/{MODULE}.lex.tex"));
    let source = std::fs::read_to_string(module.as_std_path()).expect("module");
    let imported = source.replacen(
        "\\importmodule{Glyphs}",
        &format!(
            "\\importmodule{{Glyphs}}\n\\importmodule{{{}}}",
            text_laws::PRODUCTION_MODULE
        ),
        1,
    );
    assert_ne!(source, imported, "the module imports Glyphs");
    std::fs::write(module.as_std_path(), imported).expect("module");
    project.relock();
    project
}

/// A copy of the example with two tokenizers over the vocabulary: `Digits`
/// under `longest_match` and `Raw` under `byte_level`.
fn tokenized() -> P {
    let project = text_workspace();
    let mut data = module_data(&project, MODULE);
    let declarations = data["declarations"].as_array_mut().expect("declarations");
    declarations.push(tokenizer("Digits", "longest_match", VOCABULARY));
    declarations.push(tokenizer("Raw", "byte_level", VOCABULARY));
    write_module_data(&project, MODULE, &data);
    project
}

/// The names of what one tokenizer elaborates to, in order.
fn elaborated_names(name: &str) -> Vec<String> {
    [
        "pieces",
        "size",
        "encode",
        "decode",
        "encodeText",
        "decodeText",
    ]
    .iter()
    .map(|member| format!("{name}.{member}"))
    .collect()
}

/// LM-01 (§17.12): a tokenizer is a closed declaration over a vocabulary
/// artifact, elaborating to `encode`/`decode` over `bytes` and
/// `encodeText`/`decodeText` over `string` through the laws library.
fn lm_01() {
    let project = tokenized();
    project.check_ok();
    let snapshot = snapshot(&project);
    for name in ["Digits", "Raw"] {
        let row = elaboration(&snapshot, MODULE, name);
        assert_eq!(
            names(row.declarations()),
            elaborated_names(name),
            "{name} elaborates to its six definitions, in order"
        );
        assert_eq!(
            names(row.theorems()),
            [
                format!("{name}.encode_bridge"),
                format!("{name}.decode_bridge"),
                format!("{name}.encodeText_bridge"),
                format!("{name}.decodeText_bridge"),
                format!("{name}.roundtrip"),
                format!("{name}.size_bridge"),
                format!("{name}.in_range"),
                format!("{name}.text_roundtrip")
            ],
            "{name} generates the bridges of its production forms and its three laws"
        );
        let typed: Vec<(String, String)> = row
            .declarations()
            .iter()
            .map(|declaration| {
                let parameters = declaration["parameters"].as_array().expect("parameters");
                let input = parameters
                    .first()
                    .map(|parameter| parameter["type"]["kind"].as_str().expect("kind").to_owned())
                    .unwrap_or_default();
                (
                    input,
                    declaration["result"]["kind"]
                        .as_str()
                        .expect("kind")
                        .to_owned(),
                )
            })
            .collect();
        assert_eq!(
            typed,
            [
                (String::new(), "list".to_owned()),
                (String::new(), "nat".to_owned()),
                ("bytes".to_owned(), "list".to_owned()),
                ("list".to_owned(), "option".to_owned()),
                ("string".to_owned(), "list".to_owned()),
                ("list".to_owned(), "option".to_owned()),
            ],
            "{name}: encode and decode are over bytes, encodeText and decodeText over string"
        );
    }
    // The generated Lean imports the laws library once and applies its
    // functions; the laws themselves are never spliced into the module.
    let build = support::rendered(&project);
    let lean = support::lean_text(&build, MODULE);
    assert_eq!(
        lean.matches(text_laws::IMPORT_LINE).count(),
        1,
        "the module imports the laws library once"
    );
    for applied in [
        "LexLeanText.vocabSize",
        "LexLeanText.encodeWith",
        "LexLeanText.decodeWith",
        "LexLeanText.encodeText",
        "LexLeanText.decodeText",
        "LexLeanText.encodeBytes",
        "LexLeanText.encodeTextBytes",
        "LexLeanText.allBelow",
    ] {
        assert!(
            lean.contains(applied),
            "the generated Lean applies {applied}"
        );
    }
    assert!(
        !lean.contains("namespace LexLeanText") && !lean.contains("def encodeWith"),
        "no law is spliced into the generated module"
    );
    for theorem in [
        "Digits.roundtrip",
        "Digits.in_range",
        "Digits.text_roundtrip",
    ] {
        assert!(
            lean.contains(&format!("public theorem {theorem}")),
            "the generated Lean states {theorem}"
        );
    }
    let tex = support::tex_text(&build, MODULE);
    assert!(
        tex.contains("Scheme") && tex.contains("LexLeanText.LexLeanText"),
        "the document states the scheme and the laws module"
    );
    // The closed members: a scheme outside the enumeration is a schema
    // violation, and a vocabulary of another role or schema, or a tokenizer
    // with type parameters, is refused before either backend runs.
    let project = tokenized();
    edit_declaration(&project, MODULE, "Digits", |declaration| {
        declaration["scheme"] = json!("bpe");
    });
    project.check_fails_with("LLT4001");
    let project = tokenized();
    edit_declaration(&project, MODULE, "Digits", |declaration| {
        declaration["vocabulary"] = json!({"member": {"name": "hiddenWeights"}});
    });
    refused(&project, "LLT4006", "a tokenizer needs role Vocabulary");
    let project = tokenized();
    edit_declaration(&project, MODULE, "Digits", |declaration| {
        declaration["vocabulary"] = json!({"member": {"name": "banner"}});
    });
    refused(&project, "LLT4006", "a tokenizer needs role Vocabulary");
    let project = tokenized();
    edit_declaration(&project, MODULE, "Digits", |declaration| {
        declaration["type_parameters"] = json!(["T"]);
    });
    refused(&project, "LLT4006", "a tokenizer is monomorphic");
    // The byte fallback and the ids from 256 are the library's, read from
    // its native data: the functions the elaboration applies are declared
    // there with exact axiom policies.
    let declared: BTreeSet<&str> = text_laws::declarations()
        .expect("the native library")
        .iter()
        .map(|declaration| declaration.name.as_str())
        .collect();
    for name in text_laws::names::ALL {
        assert!(
            declared.contains(format!("{}.{name}", text_laws::PREFIX).as_str()),
            "the native library declares {name}"
        );
    }
}

/// LM-02 (§17.12): the laws are generated and kernel-checked under the
/// pinned Lean, and a tokenizer whose elaboration does not close them within
/// its axiom policy fails verification.
fn lm_02() {
    if !support::lean_backed("LM-02") {
        return;
    }
    let project = tokenized();
    project.check_ok();
    let _guard = support::env_shared();
    let verified = support::verify_ok(&project);
    let attestation: serde_json::Value = serde_json::from_slice(
        &std::fs::read(verified.root.join("attestation.json").as_std_path())
            .expect("attestation exists"),
    )
    .expect("attestation parses");
    assert_eq!(attestation["status"], "verified");
    let rows = attestation["declarations"]
        .as_array()
        .expect("declarations");
    for tokenizer in ["Digits", "Raw"] {
        for law in [
            "encode_bridge",
            "decode_bridge",
            "encodeText_bridge",
            "decodeText_bridge",
            "roundtrip",
            "size_bridge",
            "in_range",
            "text_roundtrip",
        ] {
            let name = format!("{LEAN_MODULE}.{tokenizer}.{law}");
            let row = rows
                .iter()
                .find(|row| row["name"] == name)
                .unwrap_or_else(|| panic!("the attestation records `{name}`"));
            assert_eq!(row["result"], "ok", "{name}: {row}");
        }
    }
    // The policy is exact over the elaboration as a whole: a tokenizer whose
    // laws observe an axiom its policy omits fails the audit.
    let understated = tokenized();
    edit_declaration(&understated, MODULE, "Digits", |declaration| {
        declaration["axioms"] = json!(["Quot.sound", "propext"]);
    });
    understated.check_ok();
    let error = understated.verify_fails_with("LLV7005");
    assert!(
        error.to_string().contains("Digits"),
        "the audit names the tokenizer: {error}"
    );
}

/// LM-03 (§17.12): text that is not valid UTF-8 is refused and never
/// tokenized: a vocabulary that does not decode under `utf8_lines` fails by
/// the artifact rules, and `encodeText` takes a `string`, never bytes.
fn lm_03() {
    let file = "artifacts/digit-names.txt";
    for (label, bytes) in [
        ("invalid UTF-8", b"zero\n\xff\n".to_vec()),
        ("a CR", b"zero\r\n".to_vec()),
        ("a missing final LF", b"zero\none".to_vec()),
    ] {
        let project = tokenized();
        replace_artifact(&project, file, MODULE, VOCABULARY, &bytes);
        let error = project.check_fails_with("LLR3008");
        assert!(
            error.to_string().contains(VOCABULARY),
            "{label}: the artifact rules name the vocabulary: {error}"
        );
        project.assert_no_backend_output_with("LLR3008");
    }
    // `encodeText` is over `string`, which is valid UTF-8 by type, and
    // `decodeText` goes through the library's `utf8Decode`, which refuses
    // bytes that are not UTF-8 rather than transcoding them.
    let project = tokenized();
    let snapshot = snapshot(&project);
    let row = elaboration(&snapshot, MODULE, "Digits");
    let encode_text = row
        .declarations()
        .iter()
        .find(|declaration| declaration["name"] == "Digits.encodeText")
        .expect("encodeText");
    assert_eq!(encode_text["parameters"][0]["type"]["kind"], "string");
    let declared: BTreeSet<&str> = text_laws::declarations()
        .expect("the native library")
        .iter()
        .map(|declaration| declaration.name.as_str())
        .collect();
    assert!(declared.contains("LexLeanText.utf8Decode"));
    assert!(declared.contains("LexLeanText.decodeText"));
}

/// LM-13 (§17.10, §17.12): the text laws are LexLean source.
///
/// The module that states them is one native core module of the embedded
/// project, the compiler embeds it and generates its Lean, and no
/// independently authored Lean implementation of the laws exists in the
/// release tree: the project holds no `.lean` file, no `.lean` file anywhere
/// declares the library's namespace or lies under its module root, and the
/// one-time migration input is absent. The registry lists exactly the module
/// the elaboration imports, so the seam cannot drift from the data.
fn lm_13() {
    let library = text_laws::library().expect("the shipped registry");
    assert_eq!(
        library.modules,
        vec![text_laws::LAWS_MODULE.to_owned()],
        "the registry lists exactly the module the elaboration imports"
    );
    // The laws are LexLean source: no independently authored Lean
    // implementation of them exists in the release tree.
    support::assert_library_has_no_handwritten_lean(text_laws::PREFIX, "tests/text-laws/oracle");
    // The library project is one native module per listed module and
    // nothing else: no source without a module and no module without a
    // source.
    let project = repo_root().join(text_laws::LIBRARY_DIR);
    let mut sources = BTreeSet::new();
    let mut stack = vec![project.as_std_path().to_path_buf()];
    while let Some(directory) = stack.pop() {
        for entry in std::fs::read_dir(&directory).expect("the text library project") {
            let path = entry.expect("an entry").path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            let name = path.to_string_lossy().into_owned();
            assert!(
                !name.ends_with(".lean"),
                "{name}: an independently authored Lean file lies in the text library project"
            );
            // Compared as path components: a Windows checkout mixes the
            // separators of the repository root and of the library's
            // relative directory.
            if let Some(relative) = path
                .strip_prefix(project.as_std_path().join("src"))
                .ok()
                .map(|rest| rest.to_string_lossy().replace('\\', "/"))
                .and_then(|rest| rest.strip_suffix(".lex.tex").map(str::to_owned))
            {
                sources.insert(format!(
                    "{}.{}",
                    text_laws::PREFIX,
                    relative.replace('/', ".")
                ));
            }
        }
    }
    let listed: BTreeSet<String> = library.modules.iter().cloned().collect();
    assert_eq!(sources, listed, "the native sources are the listed modules");
    // The registry lists the module and the toolchain pin and nothing else:
    // every declaration and its exact axioms are the native module's own
    // data, read from it and never re-stated here.
    let registry: toml::Table =
        std::fs::read_to_string(repo_root().join(text_laws::LIBRARY_PATH).as_std_path())
            .expect("the registry")
            .parse()
            .expect("the registry is TOML");
    assert_eq!(
        registry.keys().map(String::as_str).collect::<Vec<_>>(),
        ["modules", "spec", "toolchain"],
        "the registry holds no declaration list"
    );
    // Every source is embedded, so it is hashed into the language-1.2
    // semantics and is what the compiler generates the Lean from, and the
    // native data declares every name the elaboration applies with an exact
    // axiom policy.
    for module in &library.modules {
        let path = text_laws::native_source_path(module).expect("a path");
        assert!(
            lexlean::embedded::FILES
                .iter()
                .any(|(candidate, _)| *candidate == path),
            "{path} is not embedded in the compiler"
        );
    }
    let declarations = text_laws::declarations().expect("the native library");
    for name in text_laws::names::ALL {
        let qualified = format!("{}.{name}", text_laws::PREFIX);
        let row = declarations
            .iter()
            .find(|declaration| declaration.name == qualified)
            .unwrap_or_else(|| panic!("the native data declares `{qualified}`"));
        assert!(
            row.axioms.iter().all(
                |axiom| ["Classical.choice", "Quot.sound", "propext"].contains(&axiom.as_str())
            ),
            "{qualified} pins its axioms within the standard three: {:?}",
            row.axioms
        );
    }
}

// ---------------------------------------------------------------------------
// Context windows and text generators (slice 2 of #30): LM-04..LM-06.

/// The standard axioms the window and generation laws observe.
const STANDARD: [&str; 2] = ["Quot.sound", "propext"];

fn list_type(element: serde_json::Value) -> serde_json::Value {
    json!({"kind": "list", "element": element})
}

fn token_list() -> serde_json::Value {
    list_type(json!({"kind": "uint32"}))
}

fn score_list() -> serde_json::Value {
    list_type(json!({"kind": "int"}))
}

fn integer(representation: &str, value: i64) -> serde_json::Value {
    json!({"kind": "integer", "representation": representation, "value": value.to_string()})
}

/// A list of `int` scores: a literal up to 32 scores, and above that the
/// `append` of its two halves, so that the canonical JSON of a long list
/// stays within the depth the module data allows.
fn scores(values: &[i64]) -> serde_json::Value {
    if values.len() > 32 {
        let (left, right) = values.split_at(values.len() / 2);
        return json!({
            "kind": "primitive",
            "operation": "append",
            "arguments": [scores(left), scores(right)],
            "result": score_list(),
        });
    }
    values.iter().rev().fold(
        json!({"kind": "nil", "element": {"kind": "int"}}),
        |tail, value| json!({"kind": "cons", "head": integer("int", *value), "tail": tail}),
    )
}

/// A list literal of `uint32` tokens.
fn token_literal(values: &[i64]) -> serde_json::Value {
    values.iter().rev().fold(
        json!({"kind": "nil", "element": {"kind": "uint32"}}),
        |tail, value| json!({"kind": "cons", "head": integer("uint32", *value), "tail": tail}),
    )
}

fn member(name: &str) -> serde_json::Value {
    json!({"name": name})
}

fn call(function: &str, arguments: Vec<serde_json::Value>) -> serde_json::Value {
    json!({"kind": "call", "function": member(function), "arguments": arguments})
}

fn constructor(
    name: &str,
    type_arguments: Vec<serde_json::Value>,
    arguments: Vec<serde_json::Value>,
) -> serde_json::Value {
    json!({
        "kind": "constructor",
        "constructor": member(name),
        "type_arguments": type_arguments,
        "arguments": arguments,
    })
}

fn pair(left: serde_json::Value, right: serde_json::Value) -> serde_json::Value {
    json!({"kind": "pair", "left": left, "right": right})
}

/// One context window declaration as the closed schema writes it.
fn window(name: &str, capacity: u64, overflow: &str, axioms: &[&str]) -> serde_json::Value {
    let mut value = json!({
        "kind": "context_window",
        "name": name,
        "capacity": capacity,
        "overflow": overflow,
    });
    if !axioms.is_empty() {
        value["axioms"] = json!(axioms);
    }
    value
}

/// One text generator declaration as the closed schema writes it.
fn generator(
    name: &str,
    window: &str,
    scorer: &str,
    stop: &str,
    max_tokens: u64,
    policy: serde_json::Value,
) -> serde_json::Value {
    json!({
        "kind": "text_generator",
        "name": name,
        "tokenizer": member("Digits"),
        "window": member(window),
        "scorer": {"member": member(scorer)},
        "max_tokens": max_tokens,
        "stop": member(stop),
        "policy": policy,
        "axioms": ["Classical.choice", "Quot.sound", "propext"],
    })
}

fn greedy() -> serde_json::Value {
    json!({"kind": "greedy"})
}

fn sample() -> serde_json::Value {
    json!({"kind": "sample", "generator": "xorshift32", "distribution": "proportional"})
}

/// A stateless scorer `name`: a contract from `list uint32` to `list int`
/// whose postcondition fixes the number of scores, a deterministic
/// realization whose scores ignore the context, the theorem that discharges
/// the postcondition, its evidence, and the model, which so needs no
/// runtime check.
fn scorer(name: &str, values: &[i64]) -> Vec<serde_json::Value> {
    let var = |name: &str| json!({"kind": "var", "name": name});
    let count = |list: serde_json::Value| {
        json!({
            "kind": "primitive",
            "operation": "length",
            "arguments": [list],
            "result": {"kind": "nat"},
        })
    };
    let width = json!({"kind": "nat", "value": values.len().to_string()});
    vec![
        json!({
            "kind": "definition",
            "name": format!("{name}Width"),
            "parameters": [
                {"name": "context", "type": token_list()},
                {"name": "scores", "type": score_list()},
            ],
            "result": {"kind": "prop"},
            "body": {"kind": "eq", "left": count(var("scores")), "right": width},
        }),
        json!({
            "kind": "contract",
            "name": format!("{name}Contract"),
            "input": {"name": "context", "type": token_list()},
            "output": {"name": "scores", "type": score_list()},
            "postcondition": member(&format!("{name}Width")),
        }),
        json!({
            "kind": "realization",
            "name": name,
            "input": {"name": "context", "type": token_list()},
            "output": score_list(),
            "descriptor": {"kind": "deterministic", "body": scores(values)},
        }),
        json!({
            "kind": "theorem",
            "name": format!("{name}_width"),
            "parameters": [{"name": "context", "type": token_list()}],
            "statement": call(&format!("{name}Width"), vec![var("context"), call(name, vec![var("context")])]),
            "proof": {"kind": "reflexivity"},
        }),
        json!({
            "kind": "evidence",
            "name": format!("{name}Evidence"),
            "claims": [{"kind": "satisfies_contract", "theorem": member(&format!("{name}_width"))}],
            "contract": {"member": member(&format!("{name}Contract"))},
            "realization": {"member": member(name)},
        }),
        json!({
            "kind": "model",
            "name": format!("{name}Model"),
            "contract": {"member": member(&format!("{name}Contract"))},
            "realization": {"member": member(name)},
            "evidence": [{"member": member(&format!("{name}Evidence"))}],
        }),
    ]
}

/// A nullary `uint32` definition: a stop token.
fn stop_token(name: &str, value: i64) -> serde_json::Value {
    json!({
        "kind": "definition",
        "name": name,
        "parameters": [],
        "result": {"kind": "uint32"},
        "body": integer("uint32", value),
    })
}

/// The tokenized example with two windows (`Last2` under `keep_last`,
/// `Strict2` under `refuse`, both of capacity 2), two scorers (`Prefer1`,
/// whose first maximum is token 1 and whose weights are 0, 5, 1; `Silent`,
/// which scores nothing), two stop tokens, and five generators:
///
/// - `Halt` stops at once: its stop token is the one `Prefer1` prefers;
/// - `Run` generates token 1 until its three tokens are spent;
/// - `Overflow` runs under `refuse`, so its third context does not fit;
/// - `Mute` has no score to choose from;
/// - `Draw` samples token 1 or 2 by weight, 2 being its stop token.
fn generating() -> P {
    let project = tokenized();
    let mut data = module_data(&project, MODULE);
    let declarations = data["declarations"].as_array_mut().expect("declarations");
    declarations.push(window("Last2", 2, "keep_last", &STANDARD));
    declarations.push(window("Strict2", 2, "refuse", &[]));
    declarations.extend(scorer("Prefer1", &[0, 5, 1]));
    declarations.extend(scorer("Silent", &[]));
    declarations.push(stop_token("stopOne", 1));
    declarations.push(stop_token("stopTwo", 2));
    declarations.push(generator(
        "Halt",
        "Last2",
        "Prefer1Model",
        "stopOne",
        4,
        greedy(),
    ));
    declarations.push(generator(
        "Run",
        "Last2",
        "Prefer1Model",
        "stopTwo",
        3,
        greedy(),
    ));
    declarations.push(generator(
        "Overflow",
        "Strict2",
        "Prefer1Model",
        "stopTwo",
        4,
        greedy(),
    ));
    declarations.push(generator(
        "Mute",
        "Last2",
        "SilentModel",
        "stopTwo",
        4,
        greedy(),
    ));
    declarations.push(generator(
        "Draw",
        "Last2",
        "Prefer1Model",
        "stopTwo",
        3,
        sample(),
    ));
    write_module_data(&project, MODULE, &data);
    project
}

/// A copy of [`generating`] with `declarations` appended to the module.
fn generating_with(extra: Vec<serde_json::Value>) -> P {
    let project = generating();
    let mut data = module_data(&project, MODULE);
    data["declarations"]
        .as_array_mut()
        .expect("declarations")
        .extend(extra);
    write_module_data(&project, MODULE, &data);
    project
}

/// Narrow the project's entrypoint from `Main`, which imports every module
/// of the example, to `Recognizer`, the module these cases extend, and lock
/// it again. The subject of a text case lives in `Recognizer` and its import
/// `Glyphs`; verifying `Main` would verify the rest of the example once more
/// for each case.
fn narrowed(project: &P) {
    let path = project.root.join("lexlean.toml");
    let config = std::fs::read_to_string(path.as_std_path()).expect("config");
    let narrowed = config.replace(
        "entrypoints = [\"src/Main.lex.tex\"]",
        "entrypoints = [\"src/Recognizer.lex.tex\"]",
    );
    assert_ne!(config, narrowed, "the example's entrypoint is `Main`");
    std::fs::write(path.as_std_path(), narrowed).expect("narrowed config");
    project.relock();
}

/// Verify a narrowed project with real Lean.
fn verified(project: &P) -> lexlean::VerifiedProject {
    narrowed(project);
    support::verify_ok(project)
}

/// Verify a narrowed project with real Lean, expecting the code.
fn verification_fails(project: &P, code: &str) -> lexlean::LexLeanError {
    narrowed(project);
    project.verify_fails_with(code)
}

/// A theorem `left = right` closed by reflexivity: Lean evaluates both sides.
fn evaluates(name: &str, left: serde_json::Value, right: serde_json::Value) -> serde_json::Value {
    json!({
        "kind": "theorem",
        "name": name,
        "parameters": [],
        "statement": {"kind": "eq", "left": left, "right": right},
        "proof": {"kind": "reflexivity"},
        // The kernel evaluates the production forms the generator applies,
        // whose definitions observe these axioms.
        "axioms": ["Quot.sound", "propext"],
    })
}

/// The attestation rows of a verified project, by Lean name.
fn attested(verified: &lexlean::VerifiedProject) -> Vec<serde_json::Value> {
    let attestation: serde_json::Value = serde_json::from_slice(
        &std::fs::read(verified.root.join("attestation.json").as_std_path())
            .expect("attestation exists"),
    )
    .expect("attestation parses");
    assert_eq!(attestation["status"], "verified");
    attestation["declarations"]
        .as_array()
        .expect("declarations")
        .clone()
}

fn assert_ok(rows: &[serde_json::Value], name: &str) {
    let row = rows
        .iter()
        .find(|row| row["name"] == name)
        .unwrap_or_else(|| panic!("the attestation records `{name}`"));
    assert_eq!(row["result"], "ok", "{name}: {row}");
}

/// LM-04 (§17.12): a context window is a closed declaration whose capacity
/// and overflow disposition elaborate to `capacity`, `fit`, and `push`
/// over the laws library, with its laws generated and kernel-checked.
fn lm_04() {
    let project = generating();
    project.check_ok();
    let snapshot = snapshot(&project);
    for (name, last) in [("Last2", "suffix"), ("Strict2", "refuses")] {
        let row = elaboration(&snapshot, MODULE, name);
        assert_eq!(
            names(row.declarations()),
            ["capacity", "fit", "push"].map(|member| format!("{name}.{member}")),
            "{name} elaborates to its capacity, admission, and push"
        );
        assert_eq!(
            names(row.theorems()),
            [
                "admit_bridge",
                "push_bridge",
                "bounded",
                "push_bounded",
                "exact",
                last
            ]
            .map(|law| format!("{name}.{law}")),
            "{name} generates the bridges of its production forms and its laws"
        );
    }
    let build = support::rendered(&project);
    let lean = support::lean_text(&build, MODULE);
    for applied in [
        "LexLeanText.admitKeepLast",
        "LexLeanText.pushKeepLast",
        "LexLeanText.admitRefuse",
        "LexLeanText.pushRefuse",
        "LexLeanText.fitsOption",
        "LexLeanText.suffixOption",
    ] {
        assert!(
            lean.contains(applied),
            "the generated Lean applies {applied}"
        );
    }
    assert_eq!(
        lean.matches(text_laws::IMPORT_LINE).count(),
        1,
        "the module imports the laws library once"
    );
    assert!(
        !lean.contains("def keepLast") && !lean.contains("namespace LexLeanText"),
        "no window law is spliced into the generated module"
    );
    let tex = support::tex_text(&build, MODULE);
    assert!(
        tex.contains("Capacity") && tex.contains("keep\\_last"),
        "the document states the capacity and the disposition"
    );
    // The bound is stated: a window of no tokens is refused, and a
    // disposition or capacity outside the closed schema is a schema
    // violation.
    let edited = |edit: fn(&mut serde_json::Value)| {
        let project = generating();
        edit_declaration(&project, MODULE, "Last2", edit);
        project
    };
    refused(
        &edited(|window| window["capacity"] = json!(0)),
        "LLT4016",
        "capacity 0",
    );
    edited(|window| window["overflow"] = json!("drop_first")).check_fails_with("LLT4001");
    edited(|window| window["capacity"] = json!(-1)).check_fails_with("LLT4001");
    refused(
        &edited(|window| window["axioms"] = json!(["propext", "Quot.sound"])),
        "LLT4006",
        "axiom policy is not sorted",
    );
    if !support::lean_backed("LM-04") {
        return;
    }
    let _guard = support::env_shared();
    let rows = attested(&support::verify_ok(&project));
    for (name, last) in [("Last2", "suffix"), ("Strict2", "refuses")] {
        for law in [
            "admit_bridge",
            "push_bridge",
            "bounded",
            "push_bounded",
            "exact",
            last,
        ] {
            assert_ok(&rows, &format!("{LEAN_MODULE}.{name}.{law}"));
        }
    }
    // The policy is exact over the elaboration: a window whose laws observe
    // an axiom its policy omits fails the audit.
    let understated = generating();
    edit_declaration(&understated, MODULE, "Last2", |window| {
        window["axioms"] = json!(["propext"]);
    });
    understated.check_ok();
    let error = understated.verify_fails_with("LLV7005");
    assert!(
        error.to_string().contains("Last2"),
        "the audit names the window: {error}"
    );
}

/// The type of a generated `End` or `Failure` value.
fn outcome(generator: &str, ty: &str, constructor_name: &str) -> serde_json::Value {
    constructor(
        &format!("{generator}.{ty}.{constructor_name}"),
        Vec::new(),
        Vec::new(),
    )
}

fn named_type(name: &str) -> serde_json::Value {
    json!({"kind": "named", "member": member(name), "arguments": []})
}

fn product(left: serde_json::Value, right: serde_json::Value) -> serde_json::Value {
    json!({"kind": "product", "left": left, "right": right})
}

/// `Result.ok value` of a generator's token result.
fn generated_ok(
    generator: &str,
    ok: serde_json::Value,
    value: serde_json::Value,
) -> serde_json::Value {
    constructor(
        "Result.ok",
        vec![ok, named_type(&format!("{generator}.Failure"))],
        vec![value],
    )
}

/// `Result.error failure` of a generator's token result.
fn generated_error(generator: &str, ok: serde_json::Value, failure: &str) -> serde_json::Value {
    constructor(
        "Result.error",
        vec![ok, named_type(&format!("{generator}.Failure"))],
        vec![outcome(generator, "Failure", failure)],
    )
}

/// LM-05 (§17.12): a greedy generator over a tokenizer, a window, and a
/// scorer elaborates to a bounded loop whose laws are kernel-checked, ends
/// at its stop token or its `max_tokens`, and reports the closed failures.
fn lm_05() {
    let project = generating();
    project.check_ok();
    let snapshot = snapshot(&project);
    for name in ["Halt", "Run", "Overflow", "Mute"] {
        let row = elaboration(&snapshot, MODULE, name);
        assert_eq!(
            names(row.declarations()),
            [
                "score",
                "End",
                "Failure",
                "run",
                "generateTokens",
                "generate"
            ]
            .map(|member| format!("{name}.{member}")),
            "{name} elaborates to its scorer, outcome types, and loop"
        );
        assert_eq!(
            names(row.theorems()),
            [
                format!("{name}.run_bridge"),
                format!("{name}.bounded"),
                format!("{name}.context_bounded")
            ],
            "{name} generates the bridge of its production loop and its laws"
        );
        // Greedy is deterministic by type: nothing it elaborates to takes a seed.
        for declaration in row.declarations() {
            let parameters = declaration["parameters"]
                .as_array()
                .cloned()
                .unwrap_or_default();
            assert!(
                parameters
                    .iter()
                    .all(|parameter| parameter["name"] != "__seed"),
                "{name}: `{}` takes no seed",
                declaration["name"]
            );
        }
    }
    let build = support::rendered(&project);
    let lean = support::lean_text(&build, MODULE);
    for applied in [
        "LexLeanText.generateGreedy",
        "LexLeanText.generateGreedyGuarded",
        "LexLeanText.atMost",
    ] {
        assert!(
            lean.contains(applied),
            "the generated Lean applies {applied}"
        );
    }
    assert!(!lean.contains("def genGo"), "the loop is not spliced");
    // Every bound is stated and every member is the kind it names.
    let edited = |name: &'static str, edit: fn(&mut serde_json::Value)| {
        let project = generating();
        edit_declaration(&project, MODULE, name, edit);
        project
    };
    refused(
        &edited("Run", |g| g["max_tokens"] = json!(0)),
        "LLT4016",
        "max_tokens 0",
    );
    edited("Run", |g| {
        g.as_object_mut().expect("an object").remove("max_tokens");
    })
    .check_fails_with("LLT4001");
    refused(
        &edited("Run", |g| g["tokenizer"] = json!({"name": "Last2"})),
        "LLT4006",
        "is not a prior tokenizer",
    );
    refused(
        &edited("Run", |g| g["window"] = json!({"name": "Digits"})),
        "LLT4006",
        "is not a prior context window",
    );
    refused(
        &edited("Run", |g| g["stop"] = json!({"name": "Digits.size"})),
        "LLT4006",
        "is not a prior definition of one `uint32` token",
    );
    refused(
        &edited("Run", |g| {
            g["scorer"] = json!({"member": {"name": "Prefer1"}})
        }),
        "LLT4006",
        "is not a prior model",
    );
    // The scorer is applied on every step, so its interface is closed: a
    // model that threads a state, a model whose evidence leaves a runtime
    // check, and a model of another shape are each refused.
    let scored_by = |model: &str, extra: Vec<serde_json::Value>| {
        let mut declarations = extra;
        declarations.push(generator("Refused", "Last2", model, "stopTwo", 3, greedy()));
        generating_with(declarations)
    };
    let shaped = |name: &str,
                  input: serde_json::Value,
                  output: serde_json::Value,
                  body: serde_json::Value| {
        vec![
            json!({
                "kind": "definition",
                "name": format!("{name}Holds"),
                "parameters": [
                    {"name": "context", "type": input},
                    {"name": "scores", "type": output},
                ],
                "result": {"kind": "prop"},
                "body": {
                    "kind": "eq",
                    "left": {"kind": "nat", "value": "1"},
                    "right": {"kind": "nat", "value": "1"},
                },
            }),
            json!({
                "kind": "contract",
                "name": format!("{name}Contract"),
                "input": {"name": "context", "type": input},
                "output": {"name": "scores", "type": output},
                "postcondition": member(&format!("{name}Holds")),
            }),
            json!({
                "kind": "realization",
                "name": name,
                "input": {"name": "context", "type": input},
                "output": output,
                "descriptor": {"kind": "deterministic", "body": body},
            }),
            json!({
                "kind": "model",
                "name": format!("{name}Model"),
                "contract": {"member": member(&format!("{name}Contract"))},
                "realization": {"member": member(name)},
            }),
        ]
    };
    let mut counting = shaped("Counting", token_list(), score_list(), scores(&[1]));
    counting[0]["parameters"] = json!([
        {"name": "count", "type": {"kind": "int"}},
        {"name": "context", "type": token_list()},
        {"name": "after", "type": {"kind": "int"}},
        {"name": "scores", "type": score_list()},
    ]);
    counting[1]["state"] = json!({"name": "count", "next": "after", "type": {"kind": "int"}});
    counting[2]["state"] =
        json!({"name": "count", "type": {"kind": "int"}, "initial": integer("int", 0)});
    counting[2]["descriptor"]["body"] = pair(json!({"kind": "var", "name": "count"}), scores(&[1]));
    refused(
        &scored_by("CountingModel", counting),
        "LLT4006",
        "is stateful",
    );
    let mut unproved = scorer("Unproved", &[0, 5, 1]);
    unproved.pop();
    unproved.pop();
    unproved.pop();
    unproved.push(json!({
        "kind": "model",
        "name": "UnprovedModel",
        "contract": {"member": member("UnprovedContract")},
        "realization": {"member": member("Unproved")},
    }));
    refused(
        &scored_by("UnprovedModel", unproved),
        "LLT4006",
        "needs runtime checks its evidence does not discharge",
    );
    refused(
        &scored_by(
            "CountModel",
            shaped(
                "Count",
                token_list(),
                json!({"kind": "nat"}),
                json!({"kind": "nat", "value": "1"}),
            ),
        ),
        "LLT4006",
        "does not map `list uint32` to `list int`",
    );
    refused(
        &scored_by(
            "ByteScoresModel",
            shaped("ByteScores", score_list(), score_list(), scores(&[1])),
        ),
        "LLT4006",
        "does not map `list uint32` to `list int`",
    );
    if !support::lean_backed("LM-05") {
        return;
    }
    // The loop runs in the kernel: each generator reaches its own end.
    let prompt = || token_literal(&[7]);
    let ends = |generator: &str| product(token_list(), named_type(&format!("{generator}.End")));
    // `generate` runs there too: the prompt is encoded, the tokens are
    // generated, and the output is decoded, which fails when the generated
    // bytes are not UTF-8. `Echo` scores token 98 (the byte `b`) highest and
    // `Garble` token 255 (a byte no UTF-8 text contains).
    let spiked = |size: usize, at: usize| {
        let mut values = vec![0; size];
        values[at] = 1;
        values
    };
    let mut speech = scorer("Letter", &spiked(99, 98));
    speech.extend(scorer("High", &spiked(256, 255)));
    speech.push(generator(
        "Echo",
        "Last2",
        "LetterModel",
        "stopTwo",
        2,
        greedy(),
    ));
    speech.push(generator(
        "Garble",
        "Last2",
        "HighModel",
        "stopTwo",
        1,
        greedy(),
    ));
    let text = |value: &str| json!({"kind": "string", "value": value});
    let string_type = json!({"kind": "string"});
    let texts =
        |generator: &str| product(string_type.clone(), named_type(&format!("{generator}.End")));
    let decoding = [
        evaluates(
            "haltSaysNothing",
            call("Halt.generate", vec![text("a")]),
            generated_ok(
                "Halt",
                texts("Halt"),
                pair(text(""), outcome("Halt", "End", "stop")),
            ),
        ),
        evaluates(
            "echoSpeaks",
            call("Echo.generate", vec![text("a")]),
            generated_ok(
                "Echo",
                texts("Echo"),
                pair(text("bb"), outcome("Echo", "End", "exhausted")),
            ),
        ),
        evaluates(
            "garbleIsNotText",
            call("Garble.generate", vec![text("a")]),
            generated_error("Garble", texts("Garble"), "invalid_utf8"),
        ),
    ];
    // Decoding bytes to a string goes through the classical axiom the
    // library's UTF-8 decoder uses, so the policy of these facts names it.
    speech.extend(decoding.into_iter().map(|mut fact| {
        fact["axioms"] = json!(["Classical.choice", "Quot.sound", "propext"]);
        fact
    }));
    let project = generating_with(
        speech
            .into_iter()
            .chain(vec![
                evaluates(
                    "haltStops",
                    call("Halt.generateTokens", vec![prompt()]),
                    generated_ok(
                        "Halt",
                        ends("Halt"),
                        pair(token_literal(&[]), outcome("Halt", "End", "stop")),
                    ),
                ),
                evaluates(
                    "runExhausts",
                    call("Run.generateTokens", vec![prompt()]),
                    generated_ok(
                        "Run",
                        ends("Run"),
                        pair(
                            token_literal(&[1, 1, 1]),
                            outcome("Run", "End", "exhausted"),
                        ),
                    ),
                ),
                evaluates(
                    "overflowRefuses",
                    call("Overflow.generateTokens", vec![prompt()]),
                    generated_error("Overflow", ends("Overflow"), "context_overflow"),
                ),
                evaluates(
                    "muteHasNoToken",
                    call("Mute.generateTokens", vec![prompt()]),
                    generated_error("Mute", ends("Mute"), "unknown_token"),
                ),
            ])
            .collect(),
    );
    project.check_ok();
    let _guard = support::env_shared();
    let rows = attested(&support::verify_ok(&project));
    for name in ["Halt", "Run", "Overflow", "Mute", "Echo", "Garble"] {
        for law in ["bounded", "context_bounded"] {
            assert_ok(&rows, &format!("{LEAN_MODULE}.{name}.{law}"));
        }
    }
    for fact in [
        "haltStops",
        "runExhausts",
        "overflowRefuses",
        "muteHasNoToken",
        "haltSaysNothing",
        "echoSpeaks",
        "garbleIsNotText",
    ] {
        assert_ok(&rows, &format!("{LEAN_MODULE}.{fact}"));
    }
    // A wrong claim about the loop is refused by the kernel.
    let planted = generating_with(vec![evaluates(
        "runStopsEarly",
        call("Run.generateTokens", vec![prompt()]),
        generated_ok(
            "Run",
            ends("Run"),
            pair(token_literal(&[1, 1]), outcome("Run", "End", "exhausted")),
        ),
    )]);
    planted.check_ok();
    planted.verify_fails_with("LLV7002");
}

/// LM-06 (§17.12): sampling is parameterized by an explicit seed through the
/// closed `xorshift32` and `proportional` members: the seed is an input and
/// the next seed an output, the same prompt and seed give the same output,
/// and nothing outside the closed members is admitted.
fn lm_06() {
    let project = generating();
    project.check_ok();
    let snapshot = snapshot(&project);
    let row = elaboration(&snapshot, MODULE, "Draw");
    for declaration in row.declarations() {
        let name = declaration["name"].as_str().expect("name");
        if ["Draw.run", "Draw.generateTokens", "Draw.generate"].contains(&name) {
            let parameters = declaration["parameters"].as_array().expect("parameters");
            assert_eq!(
                parameters
                    .last()
                    .map(|parameter| parameter["type"]["kind"].clone()),
                Some(json!("uint32")),
                "{name} takes the seed as its last input"
            );
        }
    }
    assert_eq!(
        names(row.theorems()),
        ["Draw.run_bridge", "Draw.bounded", "Draw.context_bounded"],
        "a sampling generator has the same laws and the bridge of its loop"
    );
    let build = support::rendered(&project);
    let lean = support::lean_text(&build, MODULE);
    assert!(
        lean.contains("LexLeanText.generateSample"),
        "the generated Lean applies the sampling loop"
    );
    // The closed members: another generator or distribution, or a seed
    // stated on the policy, is a schema violation.
    for policy in [
        json!({"kind": "sample", "generator": "mt19937", "distribution": "proportional"}),
        json!({"kind": "sample", "generator": "xorshift32", "distribution": "uniform"}),
        json!({"kind": "sample", "generator": "xorshift32"}),
        json!({"kind": "greedy", "seed": 7}),
        json!({"kind": "top_k"}),
    ] {
        let project = generating();
        edit_declaration(&project, MODULE, "Draw", |g| g["policy"] = policy.clone());
        project.check_fails_with("LLT4001");
    }
    if !support::lean_backed("LM-06") {
        return;
    }
    // The draw is a function of the prompt and the seed, through
    // `xorshift32` (Marsaglia's 13, 17, 5 triple): from seed 1 the seeds are
    // 270369, 67634689 and 2647435461, which modulo the total weight 6 are 3,
    // 1 and 3, so weights 0, 5, 1 choose token 1 three times and the last
    // seed is returned; from seed 48 the seed is 12977747, which is 5 modulo
    // 6, so the first draw is token 2, the stop token.
    let sampled_ok = product(
        product(token_list(), named_type("Draw.End")),
        json!({"kind": "uint32"}),
    );
    let draws = |seed: i64, tokens: &[i64], end: &str, next: i64| {
        let mut fact = evaluates(
            &format!("drawFrom{seed}"),
            call(
                "Draw.generateTokens",
                vec![token_literal(&[7]), integer("uint32", seed)],
            ),
            generated_ok(
                "Draw",
                sampled_ok.clone(),
                pair(
                    pair(token_literal(tokens), outcome("Draw", "End", end)),
                    integer("uint32", next),
                ),
            ),
        );
        // Lean's kernel evaluates the `uint32` arithmetic of the draw
        // through the standard axioms.
        fact["axioms"] = json!(STANDARD);
        fact
    };
    let project = generating_with(vec![
        draws(1, &[1, 1, 1], "exhausted", 2_647_435_461),
        draws(48, &[], "stop", 12_977_747),
    ]);
    project.check_ok();
    let _guard = support::env_shared();
    let rows = attested(&support::verify_ok(&project));
    for law in ["bounded", "context_bounded"] {
        assert_ok(&rows, &format!("{LEAN_MODULE}.Draw.{law}"));
    }
    for fact in ["drawFrom1", "drawFrom48"] {
        assert_ok(&rows, &format!("{LEAN_MODULE}.{fact}"));
    }
}

// ---------------------------------------------------------------------------
// LM-07: text classifiers.

/// The number of distinct token ids a feature vector counts, as a term: the
/// fold over the features that adds one for each id that occurs.
fn distinct_ids(features: serde_json::Value) -> serde_json::Value {
    let var = |name: &str| json!({"kind": "var", "name": name});
    json!({
        "kind": "primitive",
        "operation": "list_fold",
        "arguments": [
            {
                "kind": "lambda",
                "parameters": [
                    {"name": "count", "type": {"kind": "nat"}},
                    {"name": "seen", "type": {"kind": "int"}},
                ],
                "captures": [],
                "body": {
                    "kind": "if",
                    "condition": {
                        "kind": "primitive",
                        "operation": "less_than",
                        "arguments": [integer("int", 0), var("seen")],
                        "result": {"kind": "bool"},
                    },
                    "then_value": {
                        "kind": "add",
                        "left": var("count"),
                        "right": {"kind": "nat", "value": "1"},
                    },
                    "else_value": var("count"),
                },
            },
            {"kind": "nat", "value": "0"},
            features,
        ],
        "result": {"kind": "nat"},
    })
}

/// A stateless model `name` from `list int` to `nat`: the number of distinct
/// token ids a text has. Its contract's postcondition states that the label
/// is that count, the realization computes it, and the theorem that
/// discharges the postcondition is closed by reflexivity, so the model
/// needs no runtime check.
fn variety(name: &str) -> Vec<serde_json::Value> {
    let var = |name: &str| json!({"kind": "var", "name": name});
    let features = || list_type(json!({"kind": "int"}));
    let nat = || json!({"kind": "nat"});
    vec![
        json!({
            "kind": "definition",
            "name": format!("{name}Count"),
            "parameters": [
                {"name": "features", "type": features()},
                {"name": "label", "type": nat()},
            ],
            "result": {"kind": "prop"},
            "body": {"kind": "eq", "left": var("label"), "right": distinct_ids(var("features"))},
            "axioms": ["propext"],
        }),
        json!({
            "kind": "contract",
            "name": format!("{name}Contract"),
            "input": {"name": "features", "type": features()},
            "output": {"name": "label", "type": nat()},
            "postcondition": member(&format!("{name}Count")),
        }),
        json!({
            "kind": "realization",
            "name": name,
            "input": {"name": "features", "type": features()},
            "output": nat(),
            "descriptor": {"kind": "deterministic", "body": distinct_ids(var("features"))},
            "axioms": ["propext"],
        }),
        json!({
            "kind": "theorem",
            "name": format!("{name}_count"),
            "parameters": [{"name": "features", "type": features()}],
            "statement": call(&format!("{name}Count"), vec![var("features"), call(name, vec![var("features")])]),
            "proof": {"kind": "reflexivity"},
            "axioms": ["propext"],
        }),
        json!({
            "kind": "evidence",
            "name": format!("{name}Evidence"),
            "claims": [{"kind": "satisfies_contract", "theorem": member(&format!("{name}_count"))}],
            "contract": {"member": member(&format!("{name}Contract"))},
            "realization": {"member": member(name)},
            "axioms": ["propext"],
        }),
        json!({
            "kind": "model",
            "name": format!("{name}Model"),
            "contract": {"member": member(&format!("{name}Contract"))},
            "realization": {"member": member(name)},
            "evidence": [{"member": member(&format!("{name}Evidence"))}],
            "axioms": ["propext"],
        }),
    ]
}

/// One text classifier declaration as the closed schema writes it.
fn classifier(name: &str, tokenizer: &str, model: &str) -> serde_json::Value {
    json!({
        "kind": "text_classifier",
        "name": name,
        "tokenizer": member(tokenizer),
        "model": {"member": member(model)},
        "axioms": ["Classical.choice", "Quot.sound", "propext"],
    })
}

/// The tokenized example with the `Variety` model and two classifiers over
/// it: `Words` over the `longest_match` tokenizer `Digits`, whose digit
/// names are single tokens, and `Letters` over the `byte_level` tokenizer
/// `Raw`, which sees every byte.
fn classifying() -> P {
    let project = tokenized();
    let mut data = module_data(&project, MODULE);
    let declarations = data["declarations"].as_array_mut().expect("declarations");
    declarations.extend(variety("Variety"));
    declarations.push(classifier("Words", "Digits", "VarietyModel"));
    declarations.push(classifier("Letters", "Raw", "VarietyModel"));
    write_module_data(&project, MODULE, &data);
    project
}

/// A natural number literal term.
fn natural(value: u64) -> serde_json::Value {
    json!({"kind": "nat", "value": value.to_string()})
}

/// LM-07 (§17.12): a text classifier is a closed declaration over a prior
/// tokenizer and a prior stateless model from `list int`, elaborating to
/// `features`, `classifyTokens`, and `classify`, with its laws generated and
/// kernel-checked.
fn lm_07() {
    let project = classifying();
    project.check_ok();
    let snapshot = snapshot(&project);
    for name in ["Words", "Letters"] {
        let row = elaboration(&snapshot, MODULE, name);
        assert_eq!(
            names(row.declarations()),
            ["features", "classifyTokens", "classify"].map(|member| format!("{name}.{member}")),
            "{name} elaborates to its features, token classification, and text classification"
        );
        assert_eq!(
            names(row.theorems()),
            [
                "features_width",
                "features_total",
                "fitWidth_bridge",
                "features_fit"
            ]
            .map(|law| format!("{name}.{law}")),
            "{name} generates its three laws and the bridge of the production fitWidth"
        );
        let results: Vec<String> = row
            .declarations()
            .iter()
            .map(|declaration| {
                declaration["result"]["kind"]
                    .as_str()
                    .expect("kind")
                    .to_owned()
            })
            .collect();
        assert_eq!(
            results,
            ["list", "nat", "nat"],
            "{name}: the features are integers and the label is the model's output"
        );
        let inputs: Vec<String> = row
            .declarations()
            .iter()
            .map(|declaration| {
                declaration["parameters"][0]["type"]["kind"]
                    .as_str()
                    .expect("kind")
                    .to_owned()
            })
            .collect();
        assert_eq!(
            inputs,
            ["list", "list", "string"],
            "{name}: a text reaches the model only as tokens"
        );
    }
    let build = support::rendered(&project);
    let lean = support::lean_text(&build, MODULE);
    assert!(
        lean.contains("LexLeanText.features"),
        "the generated Lean states the library's features"
    );
    for lemma in [
        "LexLeanText.features_width_of",
        "LexLeanText.features_fit_bridge_of",
        "LexLeanText.features_total_text_of",
        "LexLeanText.features_total_text_bytes_of",
    ] {
        assert!(lean.contains(lemma), "the generated Lean proves by {lemma}");
    }
    assert_eq!(
        lean.matches(text_laws::IMPORT_LINE).count(),
        1,
        "the module imports the laws library once"
    );
    assert!(
        !lean.contains("def features") && !lean.contains("namespace LexLeanText"),
        "no feature law is spliced into the generated module"
    );
    let tex = support::tex_text(&build, MODULE);
    assert!(
        tex.contains("Tokenizer") && tex.contains("Model") && tex.contains("Words"),
        "the document states the tokenizer and the model"
    );
    // The closed interface: the tokenizer must be a prior tokenizer, the
    // model a stateless model of `list int` with every check discharged.
    let edited = |edit: fn(&mut serde_json::Value)| {
        let project = classifying();
        edit_declaration(&project, MODULE, "Words", edit);
        project
    };
    refused(
        &edited(|classifier| classifier["tokenizer"] = member(VOCABULARY)),
        "LLT4006",
        "is not a prior tokenizer",
    );
    refused(
        &edited(|classifier| classifier["model"] = json!({"member": member("Prefer1Model")})),
        "LLT4006",
        "is not a prior model",
    );
    refused(
        &edited(|classifier| classifier["axioms"] = json!(["propext", "Quot.sound"])),
        "LLT4006",
        "axiom policy is not sorted",
    );
    edited(|classifier| classifier["unit"] = json!(1)).check_fails_with("LLT4001");
    // A model from `list uint32` is not a model of features.
    let wrong_input = tokenized();
    let mut data = module_data(&wrong_input, MODULE);
    let declarations = data["declarations"].as_array_mut().expect("declarations");
    declarations.extend(scorer("Prefer1", &[0, 5, 1]));
    declarations.push(classifier("Words", "Digits", "Prefer1Model"));
    write_module_data(&wrong_input, MODULE, &data);
    refused(&wrong_input, "LLT4006", "does not take `list int`");
    // A model whose postcondition its evidence leaves open needs a runtime
    // check on every application, which a classifier does not make: the
    // example's `RawDigitModel` is checked at its call sites.
    let unproved = tokenized();
    let mut data = module_data(&unproved, MODULE);
    data["declarations"]
        .as_array_mut()
        .expect("declarations")
        .push(classifier("Words", "Digits", "RawDigitModel"));
    write_module_data(&unproved, MODULE, &data);
    refused(&unproved, "LLT4006", "needs runtime checks");
    // A classifier maps one feature vector to its label: a model with a state,
    // such as the window model that LM-08 composes, is refused.
    let stateful = composing();
    let mut data = module_data(&stateful, MODULE);
    data["declarations"]
        .as_array_mut()
        .expect("declarations")
        .push(classifier("Remembering", "Digits", "RecentModel"));
    write_module_data(&stateful, MODULE, &data);
    refused(&stateful, "LLT4006", "is stateful");
    if !support::lean_backed("LM-07") {
        return;
    }
    let _guard = support::env_shared();
    let rows = attested(&verified(&project));
    for name in ["Words", "Letters"] {
        for law in ["features_width", "features_total", "features_fit"] {
            assert_ok(&rows, &format!("{LEAN_MODULE}.{name}.{law}"));
        }
    }
    // The kernel evaluates the classification of a token list: the digit
    // name `one` is the single token 257 of the `longest_match` tokenizer
    // and the three bytes 111, 110, 101 of the `byte_level` one, so the two
    // classifiers count one and three distinct ids; 111 repeated is one id.
    let facts = vec![
        evaluates(
            "wordsEmpty",
            call("Words.classifyTokens", vec![token_literal(&[])]),
            natural(0),
        ),
        evaluates(
            "wordsOne",
            call("Words.classifyTokens", vec![token_literal(&[257])]),
            natural(1),
        ),
        evaluates(
            "lettersOne",
            call(
                "Letters.classifyTokens",
                vec![token_literal(&[111, 110, 101])],
            ),
            natural(3),
        ),
        evaluates(
            "lettersTwice",
            call("Letters.classifyTokens", vec![token_literal(&[111, 111])]),
            natural(1),
        ),
    ];
    let evaluated = classifying();
    let mut data = module_data(&evaluated, MODULE);
    data["declarations"]
        .as_array_mut()
        .expect("declarations")
        .extend(facts.into_iter().map(|mut fact| {
            fact["axioms"] = json!(["Quot.sound", "propext"]);
            fact
        }));
    write_module_data(&evaluated, MODULE, &data);
    evaluated.check_ok();
    let rows = attested(&verified(&evaluated));
    for fact in ["wordsEmpty", "wordsOne", "lettersOne", "lettersTwice"] {
        assert_ok(&rows, &format!("{LEAN_MODULE}.{fact}"));
    }
    // A wrong claim about a classification fails verification.
    let wrong = classifying();
    let mut data = module_data(&wrong, MODULE);
    data["declarations"]
        .as_array_mut()
        .expect("declarations")
        .push(evaluates(
            "lettersWrong",
            call(
                "Letters.classifyTokens",
                vec![token_literal(&[111, 110, 101])],
            ),
            natural(1),
        ));
    write_module_data(&wrong, MODULE, &data);
    wrong.check_ok();
    verification_fails(&wrong, "LLV7002");
    // The policy is exact over the elaboration: a classifier whose laws
    // observe an axiom its policy omits fails the audit.
    let understated = classifying();
    edit_declaration(&understated, MODULE, "Words", |classifier| {
        classifier["axioms"] = json!(["propext"]);
    });
    understated.check_ok();
    let error = verification_fails(&understated, "LLV7005");
    assert!(
        error.to_string().contains("Words"),
        "the audit names the classifier: {error}"
    );
}

// ---------------------------------------------------------------------------
// LM-09: no prompt is code.

/// What a primitive does with `string` or `bytes` operands, as SPEC.md §17.4
/// types it: `text` says whether any operand may be a `string` or `bytes`
/// value, and `result` classifies what it returns. The table is the
/// audit's reviewed classification: an operation absent from it is refused,
/// so a primitive added to the language must be classified before the
/// registries may name it.
const PRIMITIVES: [(&str, bool, &str); 56] = [
    ("append", true, "bytes_or_list"),
    ("bit_and", false, "fixed_width"),
    ("bit_not", false, "fixed_width"),
    ("bit_or", false, "fixed_width"),
    ("bit_xor", false, "fixed_width"),
    ("checked_add", false, "option_number"),
    ("checked_convert", false, "option_number"),
    ("checked_multiply", false, "option_number"),
    ("checked_negate", false, "option_number"),
    ("checked_quotient", false, "option_number"),
    ("checked_subtract", false, "option_number"),
    ("compare_bytes", true, "ordering"),
    ("equal", true, "bool"),
    ("format_decimal", false, "string"),
    ("graph_reachable", false, "container"),
    ("graph_successors", false, "container"),
    ("graph_topological", false, "container"),
    ("index", true, "option_element"),
    ("iterate", false, "value"),
    ("iterate_until", false, "value"),
    ("join", true, "string"),
    ("length", true, "nat"),
    ("less_than", false, "bool"),
    ("list_fold", false, "value"),
    ("map_contains", true, "bool"),
    ("map_entries", false, "container"),
    ("map_fold", false, "value"),
    ("map_insert", true, "container"),
    ("map_keys", false, "container"),
    ("map_lookup", true, "option_element"),
    ("map_remove", true, "container"),
    ("map_size", false, "nat"),
    ("map_values", false, "container"),
    ("multiply", false, "number"),
    ("negate", false, "number"),
    ("parse_decimal", true, "option_number"),
    ("quotient", false, "number"),
    ("remainder", false, "number"),
    ("set_contains", true, "bool"),
    ("set_difference", false, "container"),
    ("set_elements", false, "container"),
    ("set_fold", false, "value"),
    ("set_insert", true, "container"),
    ("set_intersection", false, "container"),
    ("set_remove", true, "container"),
    ("set_size", false, "nat"),
    ("set_union", false, "container"),
    ("shift_left", false, "option_number"),
    ("shift_right", false, "option_number"),
    ("slice", true, "option_bytes"),
    ("split_exact", true, "list_string"),
    ("subtract", false, "number"),
    ("to_int", false, "number"),
    ("to_nat", false, "number"),
    ("utf8_decode", true, "option_string"),
    ("utf8_encode", true, "bytes"),
];

/// Every result class that is data. A result that is code — a function, a
/// term, a declaration, or source — is in none of them.
const DATA_RESULTS: [&str; 15] = [
    "bool",
    "bytes",
    "bytes_or_list",
    "container",
    "fixed_width",
    "list_string",
    "nat",
    "number",
    "option_bytes",
    "option_element",
    "option_number",
    "option_string",
    "ordering",
    "string",
    "value",
];

/// The text declarations of the closed schema.
const TEXT_KINDS: [&str; 4] = [
    "tokenizer",
    "context_window",
    "text_generator",
    "text_classifier",
];

/// The operations of the primitive node of a schema.
fn schema_primitives(schema: &serde_json::Value) -> Result<Vec<String>, String> {
    let terms = schema["$defs"]["term"]["oneOf"]
        .as_array()
        .ok_or("the schema has no term variants")?;
    let primitive = terms
        .iter()
        .find(|term| term["properties"]["kind"]["const"] == "primitive")
        .ok_or("the schema has no primitive node")?;
    primitive["properties"]["operation"]["enum"]
        .as_array()
        .ok_or("the primitive node has no operation enumeration")?
        .iter()
        .map(|operation| {
            operation
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| "a primitive operation is not a name".to_owned())
        })
        .collect()
}

/// The operations the production registry lists as primitives.
fn production_primitives(registry: &toml::Value) -> Result<Vec<String>, String> {
    registry["construct"]
        .as_array()
        .ok_or("the production registry has no constructs")?
        .iter()
        .filter_map(|row| row["key"].as_str())
        .filter_map(|key| key.strip_prefix("primitive."))
        .map(|operation| Ok(operation.to_owned()))
        .collect()
}

/// Whether a schema node states a free `string`: a string type that no
/// pattern, enumeration, or constant closes, found through the node and
/// whatever it references. A free string is text a declaration could carry
/// as data of its own; an identifier (a pattern) is not.
fn free_string(
    schema: &serde_json::Value,
    node: &serde_json::Value,
    seen: &mut Vec<String>,
) -> bool {
    if let Some(reference) = node["$ref"].as_str() {
        if seen.iter().any(|visited| visited == reference) {
            return false;
        }
        seen.push(reference.to_owned());
        let Some(target) = reference
            .strip_prefix("#/$defs/")
            .map(|name| &schema["$defs"][name])
        else {
            return true;
        };
        return free_string(schema, target, seen);
    }
    if node["type"] == "string" {
        return node.get("pattern").is_none()
            && node.get("enum").is_none()
            && node.get("const").is_none();
    }
    let mut children: Vec<&serde_json::Value> = Vec::new();
    for key in ["oneOf", "anyOf", "allOf"] {
        if let Some(branches) = node[key].as_array() {
            children.extend(branches);
        }
    }
    if let Some(properties) = node["properties"].as_object() {
        children.extend(properties.values());
    }
    if node["items"].is_object() {
        children.push(&node["items"]);
    }
    children
        .into_iter()
        .any(|child| free_string(schema, child, &mut seen.clone()))
}

/// The audit: the registries name one set of operations, every operation is
/// classified, every operation that may take text returns data, and no text
/// declaration states a free string. Returns the number of operations that
/// take text, or the first violation with its name.
fn audit_no_prompt_as_code(
    schema: &serde_json::Value,
    production: &toml::Value,
    classified: &[(&str, bool, &str)],
) -> Result<usize, String> {
    let mut registered = schema_primitives(schema)?;
    registered.sort();
    let mut listed = production_primitives(production)?;
    listed.sort();
    if let Some(operation) = registered.iter().find(|name| !listed.contains(name)) {
        return Err(format!(
            "primitive `{operation}` is in the schema enumeration and not in the production registry"
        ));
    }
    if let Some(operation) = listed.iter().find(|name| !registered.contains(name)) {
        return Err(format!(
            "primitive `{operation}` is in the production registry and not in the schema enumeration"
        ));
    }
    for operation in &registered {
        if !classified.iter().any(|(name, _, _)| name == operation) {
            return Err(format!(
                "primitive `{operation}` is registered and not classified by the audit"
            ));
        }
    }
    let mut taking_text = 0;
    for (operation, text, result) in classified {
        if !registered.iter().any(|name| name == operation) {
            return Err(format!(
                "classified primitive `{operation}` is not registered"
            ));
        }
        if *text {
            taking_text += 1;
            if !DATA_RESULTS.contains(result) {
                return Err(format!(
                    "primitive `{operation}` takes a string or bytes operand and returns `{result}`, which is not data"
                ));
            }
        }
    }
    let declarations = schema["$defs"]["declaration"]["oneOf"]
        .as_array()
        .ok_or("the schema has no declarations")?;
    let mut text_declarations = 0;
    for declaration in declarations {
        let Some(kind) = declaration["properties"]["kind"]["const"].as_str() else {
            continue;
        };
        if !TEXT_KINDS.contains(&kind) {
            continue;
        }
        text_declarations += 1;
        let properties = declaration["properties"]
            .as_object()
            .ok_or_else(|| format!("text declaration `{kind}` has no members"))?;
        for (member, node) in properties {
            if free_string(schema, node, &mut Vec::new()) {
                return Err(format!(
                    "text declaration `{kind}` member `{member}` is a free string"
                ));
            }
        }
    }
    if text_declarations != TEXT_KINDS.len() {
        return Err(format!(
            "the schema states {text_declarations} of the {} text declarations",
            TEXT_KINDS.len()
        ));
    }
    Ok(taking_text)
}

/// The audit over the eligibility report: each member of a `text_rows` row is
/// a closed member, a number, or a qualified name, and none is a free string,
/// so a row records what bounds a component and never text a prompt could
/// reach. Returns the number of members audited, or the first violation.
fn audit_text_rows(report: &serde_json::Value) -> Result<usize, String> {
    let row = &report["$defs"]["text_row"];
    let properties = row["properties"]
        .as_object()
        .ok_or("the eligibility schema states no text row")?;
    if report["$defs"]["root"]["properties"]["text_rows"]["items"]["$ref"] != "#/$defs/text_row" {
        return Err("`text_rows` does not state its rows as `text_row`".to_owned());
    }
    for (member, node) in properties {
        if free_string(report, node, &mut Vec::new()) {
            return Err(format!("text row member `{member}` is a free string"));
        }
    }
    Ok(properties.len())
}

/// LM-09 (§17.12): no operation of the closed language maps a `string` or
/// `bytes` operand to a function, a term, or source, and no text declaration
/// states a string; a planted operation or member of that shape is caught
/// with its name.
fn lm_09() {
    let root = repo_root();
    let read = |path: &str| {
        std::fs::read_to_string(root.join(path).as_std_path())
            .unwrap_or_else(|error| panic!("{path} is readable: {error}"))
    };
    let schema: serde_json::Value =
        serde_json::from_str(&read("schemas/semantic-module-v2.schema.json")).expect("schema");
    let production: toml::Value = read("language/production-1.2.toml")
        .parse()
        .expect("registry");
    let taking_text = audit_no_prompt_as_code(&schema, &production, &PRIMITIVES)
        .unwrap_or_else(|reason| panic!("the audit holds: {reason}"));
    assert!(
        taking_text > 0,
        "the audit is armed: some operations take text"
    );
    // Every registered operation is a variant of the compiler's primitive
    // enumeration, so the schema names no operation the compiler lacks.
    for operation in schema_primitives(&schema).expect("operations") {
        serde_json::from_value::<lexlean::SnapshotPrimitive>(json!(operation))
            .unwrap_or_else(|error| panic!("`{operation}` is a compiler primitive: {error}"));
    }
    // The eligibility report's text rows are audited the same way: a root
    // reports the contract of each text component it runs and no free text.
    let report: serde_json::Value =
        serde_json::from_str(&read("schemas/production-eligibility.schema.json"))
            .expect("eligibility schema");
    let audited = audit_text_rows(&report)
        .unwrap_or_else(|reason| panic!("the text row audit holds: {reason}"));
    assert!(audited > 0, "the text row audit is armed: a row has members");
    let mut leaking = report.clone();
    leaking["$defs"]["text_row"]["properties"]["prompt"] = json!({"type": "string"});
    let error = audit_text_rows(&leaking).expect_err("a free string row member is caught");
    assert!(
        error.contains("prompt") && error.contains("free string"),
        "the audit names the member: {error}"
    );
    let mut unpatterned = report.clone();
    unpatterned["$defs"]["text_row"]["properties"]["component"] = json!({"type": "string"});
    let error = audit_text_rows(&unpatterned).expect_err("an open component name is caught");
    assert!(error.contains("component"), "{error}");
    let mut unreached = report.clone();
    unreached["$defs"]["root"]["properties"]["text_rows"]["items"] = json!({"type": "object"});
    assert!(
        audit_text_rows(&unreached).is_err(),
        "a report that does not state its rows as text rows is caught"
    );
    // The audit reads every text member of every text declaration: a
    // `prompt` member of type string on any of them is caught by name.
    let declarations = schema["$defs"]["declaration"]["oneOf"]
        .as_array()
        .expect("declarations");
    for kind in TEXT_KINDS {
        let index = declarations
            .iter()
            .position(|declaration| declaration["properties"]["kind"]["const"] == kind)
            .expect("a text declaration");
        let mut planted = schema.clone();
        planted["$defs"]["declaration"]["oneOf"][index]["properties"]["prompt"] =
            json!({"type": "string"});
        let error = audit_no_prompt_as_code(&planted, &production, &PRIMITIVES)
            .expect_err("a free string member is caught");
        assert!(
            error.contains(kind) && error.contains("prompt") && error.contains("free string"),
            "the audit names {kind} and its member: {error}"
        );
    }
    // A string reached through a reference is as free as one stated inline.
    let mut referenced = schema.clone();
    referenced["$defs"]["freeText"] = json!({"type": "string"});
    let index = declarations
        .iter()
        .position(|declaration| declaration["properties"]["kind"]["const"] == "text_classifier")
        .expect("the classifier");
    referenced["$defs"]["declaration"]["oneOf"][index]["properties"]["prompt"] =
        json!({"$ref": "#/$defs/freeText"});
    assert!(
        audit_no_prompt_as_code(&referenced, &production, &PRIMITIVES)
            .expect_err("a referenced free string is caught")
            .contains("prompt"),
        "a reference to a free string is a free string"
    );
    // A planted eval-shaped operation is caught in each registry and in the
    // classification, by name.
    let mut evaluating = schema.clone();
    let terms = evaluating["$defs"]["term"]["oneOf"]
        .as_array_mut()
        .expect("terms");
    let primitive = terms
        .iter_mut()
        .find(|term| term["properties"]["kind"]["const"] == "primitive")
        .expect("the primitive node");
    primitive["properties"]["operation"]["enum"]
        .as_array_mut()
        .expect("operations")
        .push(json!("eval_string"));
    let error = audit_no_prompt_as_code(&evaluating, &production, &PRIMITIVES)
        .expect_err("a schema operation unknown to the production registry is caught");
    assert!(error.contains("eval_string"), "{error}");
    let mut registry = production.clone();
    let mut row = toml::Table::new();
    row.insert("key".to_owned(), "primitive.eval_string".into());
    row.insert("disposition".to_owned(), "runtime".into());
    registry["construct"]
        .as_array_mut()
        .expect("constructs")
        .push(toml::Value::Table(row));
    let error = audit_no_prompt_as_code(&schema, &registry, &PRIMITIVES)
        .expect_err("a production row unknown to the schema is caught");
    assert!(error.contains("eval_string"), "{error}");
    let both = {
        let mut table = PRIMITIVES.to_vec();
        table.push(("eval_string", true, "function"));
        table
    };
    let error = audit_no_prompt_as_code(&evaluating, &registry, &both)
        .expect_err("a registered operation from a string to a function is caught");
    assert!(
        error.contains("eval_string") && error.contains("not data"),
        "the audit names the operation and its result: {error}"
    );
    // An operation the audit has not classified is refused until it is.
    let error = audit_no_prompt_as_code(&evaluating, &registry, &PRIMITIVES)
        .expect_err("an unclassified operation is caught");
    assert!(error.contains("eval_string"), "{error}");
    // Removing a classification is caught as well.
    let error = audit_no_prompt_as_code(&schema, &production, &PRIMITIVES[1..])
        .expect_err("a missing classification is caught");
    assert!(error.contains("append"), "{error}");
}

// ---------------------------------------------------------------------------
// LM-08: composition and adapters.

fn option_type(value: serde_json::Value) -> serde_json::Value {
    json!({"kind": "option", "value": value})
}

fn var(name: &str) -> serde_json::Value {
    json!({"kind": "var", "name": name})
}

fn length_of(list: serde_json::Value) -> serde_json::Value {
    json!({
        "kind": "primitive",
        "operation": "length",
        "arguments": [list],
        "result": {"kind": "nat"},
    })
}

fn library(function: &str) -> serde_json::Value {
    json!({"module": "LexLeanText", "name": function})
}

/// The tokenized, windowed, generating example with a stateful model
/// `RecentModel` over the window `Last2`, the classifiers `Words` and
/// `Letters`, and everything composed from them (§17.12, *Text components*
/// rule 9).
///
/// `RecentModel` pushes each input token through `Last2.push`, keeps the
/// context the push returns (or the old one when the push refuses), and
/// outputs the push itself. Its contract's postcondition states that the
/// pushed context fits the window, which is the statement of the window's
/// generated `push_bounded`, so `push_bounded` is the whole proof of its
/// `satisfies_contract` evidence. `RecentStream` scans it over a list of
/// tokens; `ClassifyText` and `GenerateText` are the adapters.
fn composing() -> P {
    let project = generating();
    let mut data = module_data(&project, MODULE);
    let declarations = data["declarations"].as_array_mut().expect("declarations");
    declarations.extend(variety("Variety"));
    declarations.push(classifier("Words", "Digits", "VarietyModel"));
    declarations.push(classifier("Letters", "Raw", "VarietyModel"));
    let pushed = option_type(token_list());
    let policy = ["Quot.sound", "propext"];
    declarations.push(json!({
        "kind": "definition",
        "name": "RecentPost",
        "parameters": [
            {"name": "context", "type": token_list()},
            {"name": "token", "type": {"kind": "uint32"}},
            {"name": "updated", "type": token_list()},
            {"name": "pushed", "type": pushed.clone()},
        ],
        "result": {"kind": "prop"},
        "body": {
            "kind": "eq",
            "left": {
                "kind": "call",
                "function": library("fitsOption"),
                "arguments": [call("Last2.capacity", vec![]), var("pushed")],
            },
            "right": {"kind": "bool", "value": true},
        },
    }));
    declarations.push(json!({
        "kind": "contract",
        "name": "RecentContract",
        "input": {"name": "token", "type": {"kind": "uint32"}},
        "output": {"name": "pushed", "type": pushed.clone()},
        "state": {"name": "context", "type": token_list(), "next": "updated"},
        "postcondition": member("RecentPost"),
    }));
    declarations.push(json!({
        "kind": "realization",
        "name": "RecentStep",
        "input": {"name": "token", "type": {"kind": "uint32"}},
        "output": pushed.clone(),
        "state": {
            "name": "context",
            "type": token_list(),
            "initial": {"kind": "nil", "element": {"kind": "uint32"}},
        },
        "descriptor": {
            "kind": "deterministic",
            "body": {
                "kind": "let",
                "binder": {"name": "next", "type": pushed.clone()},
                "value": call("Last2.push", vec![var("context"), var("token")]),
                "body": pair(
                    json!({
                        "kind": "match",
                        "scrutinee": var("next"),
                        "branches": [
                            {"constructor": member("Option.none"), "binders": [], "body": var("context")},
                            {"constructor": member("Option.some"), "binders": ["kept"], "body": var("kept")},
                        ],
                    }),
                    var("next"),
                ),
            },
        },
    }));
    declarations.push(json!({
        "kind": "theorem",
        "name": "recent_bounded",
        "parameters": [
            {"name": "context", "type": token_list()},
            {"name": "token", "type": {"kind": "uint32"}},
        ],
        "statement": call("RecentPost", vec![
            var("context"),
            var("token"),
            json!({"kind": "first", "value": call("RecentStep", vec![var("context"), var("token")])}),
            json!({"kind": "second", "value": call("RecentStep", vec![var("context"), var("token")])}),
        ]),
        "proof": {
            "kind": "apply",
            "theorem": member("Last2.push_bounded"),
            "arguments": [var("context"), var("token")],
        },
        "axioms": policy,
    }));
    declarations.push(json!({
        "kind": "evidence",
        "name": "RecentEvidence",
        "claims": [{"kind": "satisfies_contract", "theorem": member("recent_bounded")}],
        "contract": {"member": member("RecentContract")},
        "realization": {"member": member("RecentStep")},
        "axioms": policy,
    }));
    declarations.push(json!({
        "kind": "model",
        "name": "RecentModel",
        "contract": {"member": member("RecentContract")},
        "realization": {"member": member("RecentStep")},
        "evidence": [{"member": member("RecentEvidence")}],
    }));
    let sizes = list_type(pushed.clone());
    declarations.push(json!({
        "kind": "definition",
        "name": "RecentStreamPost",
        "parameters": [
            {"name": "tokens", "type": token_list()},
            {"name": "pushes", "type": sizes.clone()},
        ],
        "result": {"kind": "prop"},
        "body": {
            "kind": "eq",
            "left": length_of(var("pushes")),
            "right": length_of(var("tokens")),
        },
    }));
    declarations.push(json!({
        "kind": "contract",
        "name": "RecentStreamContract",
        "input": {"name": "tokens", "type": token_list()},
        "output": {"name": "pushes", "type": sizes.clone()},
        "postcondition": member("RecentStreamPost"),
    }));
    declarations.push(json!({
        "kind": "realization",
        "name": "RecentStream",
        "input": {"name": "tokens", "type": token_list()},
        "output": sizes,
        "descriptor": {
            "kind": "composite",
            "form": {
                "kind": "scan",
                "junction": {"kind": "unconditional"},
                "stage": {"member": member("RecentModel")},
            },
        },
    }));
    declarations.push(json!({
        "kind": "model",
        "name": "RecentStreamModel",
        "contract": {"member": member("RecentStreamContract")},
        "realization": {"member": member("RecentStream")},
    }));
    // The adapters: models over `string` whose realizations apply the
    // elaborated `classify` and `generate`, and whose postconditions state
    // laws the elaborations generate, so that the laws are their proofs.
    let text_type = || json!({"kind": "string"});
    let encoded = || call("Digits.encodeText", vec![var("text")]);
    let adapted = json!({
        "kind": "prop_and",
        "left": {
            "kind": "eq",
            "left": length_of(call("Words.features", vec![encoded()])),
            "right": call("Digits.size", vec![]),
        },
        "right": {
            "kind": "eq",
            "left": var("label"),
            "right": call("Words.classify", vec![var("text")]),
        },
    });
    declarations.push(json!({
        "kind": "definition",
        "name": "Adapted",
        "parameters": [
            {"name": "text", "type": text_type()},
            {"name": "label", "type": {"kind": "nat"}},
        ],
        "result": {"kind": "prop"},
        "body": adapted,
        "axioms": ["propext"],
    }));
    declarations.push(json!({
        "kind": "contract",
        "name": "ClassifyTextContract",
        "input": {"name": "text", "type": text_type()},
        "output": {"name": "label", "type": {"kind": "nat"}},
        "postcondition": member("Adapted"),
    }));
    declarations.push(json!({
        "kind": "realization",
        "name": "ClassifyText",
        "input": {"name": "text", "type": text_type()},
        "output": {"kind": "nat"},
        "descriptor": {
            "kind": "deterministic",
            "body": call("Words.classify", vec![var("text")]),
        },
        "axioms": ["propext"],
    }));
    declarations.push(json!({
        "kind": "theorem",
        "name": "classify_text_adapted",
        "parameters": [{"name": "text", "type": text_type()}],
        "statement": call("Adapted", vec![var("text"), call("ClassifyText", vec![var("text")])]),
        "proof": {
            "kind": "constructor",
            "branches": [
                {
                    "kind": "apply",
                    "theorem": member("Words.features_width"),
                    "arguments": [encoded()],
                },
                {"kind": "reflexivity"},
            ],
        },
        "axioms": ["propext"],
    }));
    declarations.push(json!({
        "kind": "evidence",
        "name": "ClassifyTextEvidence",
        "claims": [{"kind": "satisfies_contract", "theorem": member("classify_text_adapted")}],
        "contract": {"member": member("ClassifyTextContract")},
        "realization": {"member": member("ClassifyText")},
        "axioms": ["propext"],
    }));
    declarations.push(json!({
        "kind": "model",
        "name": "ClassifyTextModel",
        "contract": {"member": member("ClassifyTextContract")},
        "realization": {"member": member("ClassifyText")},
        "evidence": [{"member": member("ClassifyTextEvidence")}],
        "axioms": ["propext"],
    }));
    let generated = json!({
        "kind": "result",
        "ok": product(text_type(), named_type("Halt.End")),
        "error": named_type("Halt.Failure"),
    });
    declarations.push(json!({
        "kind": "definition",
        "name": "Prompted",
        "parameters": [
            {"name": "prompt", "type": text_type()},
            {"name": "outcome", "type": generated.clone()},
        ],
        "result": {"kind": "prop"},
        "body": {
            "kind": "prop_and",
            "left": {
                "kind": "eq",
                "left": call(
                    "Digits.decodeText",
                    vec![call("Digits.encodeText", vec![var("prompt")])],
                ),
                "right": constructor("Option.some", vec![text_type()], vec![var("prompt")]),
            },
            "right": {
                "kind": "eq",
                "left": var("outcome"),
                "right": call("Halt.generate", vec![var("prompt")]),
            },
        },
        "axioms": ["Classical.choice", "Quot.sound", "propext"],
    }));
    declarations.push(json!({
        "kind": "contract",
        "name": "GenerateTextContract",
        "input": {"name": "prompt", "type": text_type()},
        "output": {"name": "outcome", "type": generated.clone()},
        "postcondition": member("Prompted"),
    }));
    declarations.push(json!({
        "kind": "realization",
        "name": "GenerateText",
        "input": {"name": "prompt", "type": text_type()},
        "output": generated,
        "descriptor": {
            "kind": "deterministic",
            "body": call("Halt.generate", vec![var("prompt")]),
        },
        "axioms": ["Classical.choice", "Quot.sound", "propext"],
    }));
    declarations.push(json!({
        "kind": "theorem",
        "name": "generate_text_prompted",
        "parameters": [{"name": "prompt", "type": text_type()}],
        "statement": call("Prompted", vec![var("prompt"), call("GenerateText", vec![var("prompt")])]),
        "proof": {
            "kind": "constructor",
            "branches": [
                {
                    "kind": "apply",
                    "theorem": member("Digits.text_roundtrip"),
                    "arguments": [var("prompt")],
                },
                {"kind": "reflexivity"},
            ],
        },
        "axioms": ["Classical.choice", "Quot.sound", "propext"],
    }));
    declarations.push(json!({
        "kind": "evidence",
        "name": "GenerateTextEvidence",
        "claims": [{"kind": "satisfies_contract", "theorem": member("generate_text_prompted")}],
        "contract": {"member": member("GenerateTextContract")},
        "realization": {"member": member("GenerateText")},
        "axioms": ["Classical.choice", "Quot.sound", "propext"],
    }));
    declarations.push(json!({
        "kind": "model",
        "name": "GenerateTextModel",
        "contract": {"member": member("GenerateTextContract")},
        "realization": {"member": member("GenerateText")},
        "evidence": [{"member": member("GenerateTextEvidence")}],
        "axioms": ["Classical.choice", "Quot.sound", "propext"],
    }));
    // The stream the scan computes, which the kernel evaluates: the window of
    // two keeps the last two tokens, so the pushes are the prefixes' tails.
    let some_context = |tokens: &[i64]| {
        constructor(
            "Option.some",
            vec![token_list()],
            vec![token_literal(tokens)],
        )
    };
    let pushes = [vec![1], vec![1, 2], vec![2, 3]].iter().rev().fold(
        json!({"kind": "nil", "element": option_type(token_list())}),
        |tail, tokens| json!({"kind": "cons", "head": some_context(tokens), "tail": tail}),
    );
    declarations.push(evaluates(
        "recent_stream_run",
        call("RecentStream", vec![token_literal(&[1, 2, 3])]),
        pushes,
    ));
    write_module_data(&project, MODULE, &data);
    project
}

/// LM-08 (§17.12): text components compose and adapt through the model
/// constructs alone.
fn lm_08() {
    let project = composing();
    project.check_ok();
    // The stream is a scan of the stateful window model, and nothing but
    // model declarations built it: the closed list of declaration kinds is
    // the one the language had.
    let schema: serde_json::Value = serde_json::from_slice(
        &std::fs::read(
            repo_root()
                .join("schemas/semantic-module-v2.schema.json")
                .as_std_path(),
        )
        .expect("schema"),
    )
    .expect("schema parses");
    let kinds: Vec<&str> = schema["$defs"]["declaration"]["oneOf"]
        .as_array()
        .expect("declarations")
        .iter()
        .map(|declaration| {
            declaration["properties"]["kind"]["const"]
                .as_str()
                .expect("kind")
        })
        .collect();
    assert_eq!(
        kinds,
        [
            "structure",
            "class",
            "instance",
            "inductive",
            "definition",
            "theorem",
            "artifact",
            "contract",
            "realization",
            "evidence",
            "model",
            "logic",
            "inference_rule",
            "verifier",
            "reasoner",
            "tokenizer",
            "context_window",
            "text_generator",
            "text_classifier",
        ],
        "composition and adaptation add no declaration kind"
    );
    let build = support::rendered(&project);
    let lean = support::lean_text(&build, MODULE);
    for proved in [
        "Last2.push_bounded",
        "Words.features_width",
        "Digits.text_roundtrip",
    ] {
        assert!(
            lean.contains(proved),
            "the generated Lean proves an adapter's evidence by {proved}"
        );
    }
    // An adapter whose realization is not the one its contract names, or
    // whose model applies a declaration of another kind, is refused.
    let foreign = composing();
    edit_declaration(&foreign, MODULE, "ClassifyTextModel", |model| {
        model["realization"] = json!({"member": member("Words")});
    });
    refused(&foreign, "LLT4006", "is not a prior realization");
    if !support::lean_backed("LM-08") {
        return;
    }
    let _guard = support::env_shared();
    let rows = attested(&verified(&project));
    for proved in [
        "recent_bounded",
        "classify_text_adapted",
        "generate_text_prompted",
        "recent_stream_run",
    ] {
        assert_ok(&rows, &format!("{LEAN_MODULE}.{proved}"));
    }
    // An adapter whose realization does not meet its contract fails
    // verification: the classification is not the contract's label.
    let unfaithful = composing();
    edit_declaration(&unfaithful, MODULE, "ClassifyText", |realization| {
        realization["descriptor"]["body"] = json!({"kind": "nat", "value": "7"});
    });
    unfaithful.check_ok();
    verification_fails(&unfaithful, "LLV7002");
}
