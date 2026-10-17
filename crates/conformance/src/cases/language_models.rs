//! Conformance cases for the language-1.2 text components (SPEC.md §17.10,
//! §17.12): LM-01..LM-03, the tokenizer and its laws; LM-04..LM-06, context
//! windows and greedy and sampled generation; and LM-13, the laws as LexLean
//! source.
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
    write_module_data, EXAMPLE,
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
/// case is not yet wired — an unimplemented registered capability must not
/// pass silently.
pub fn run(id: &str) {
    match id {
        "LM-01" => lm_01(),
        "LM-02" => lm_02(),
        "LM-03" => lm_03(),
        "LM-04" => lm_04(),
        "LM-05" => lm_05(),
        "LM-06" => lm_06(),
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

/// A copy of the example with two tokenizers over the vocabulary: `Digits`
/// under `longest_match` and `Raw` under `byte_level`.
fn tokenized() -> P {
    let project = P::copy_example(EXAMPLE);
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
                format!("{name}.roundtrip"),
                format!("{name}.in_range"),
                format!("{name}.text_roundtrip")
            ],
            "{name} generates its three laws"
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
        for law in ["roundtrip", "in_range", "text_roundtrip"] {
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

/// A list literal of `int` scores.
fn scores(values: &[i64]) -> serde_json::Value {
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

/// A theorem `left = right` closed by reflexivity: Lean evaluates both sides.
fn evaluates(name: &str, left: serde_json::Value, right: serde_json::Value) -> serde_json::Value {
    json!({
        "kind": "theorem",
        "name": name,
        "parameters": [],
        "statement": {"kind": "eq", "left": left, "right": right},
        "proof": {"kind": "reflexivity"},
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
            ["bounded", "push_bounded", "exact", last].map(|law| format!("{name}.{law}")),
            "{name} generates its laws"
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
        for law in ["bounded", "push_bounded", "exact", last] {
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
            [format!("{name}.bounded"), format!("{name}.context_bounded")],
            "{name} generates its laws"
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
    if !support::lean_backed("LM-05") {
        return;
    }
    // The loop runs in the kernel: each generator reaches its own end.
    let prompt = || token_literal(&[7]);
    let ends = |generator: &str| product(token_list(), named_type(&format!("{generator}.End")));
    let project = generating_with(vec![
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
    ]);
    project.check_ok();
    let _guard = support::env_shared();
    let rows = attested(&support::verify_ok(&project));
    for name in ["Halt", "Run", "Overflow", "Mute"] {
        for law in ["bounded", "context_bounded"] {
            assert_ok(&rows, &format!("{LEAN_MODULE}.{name}.{law}"));
        }
    }
    for fact in [
        "haltStops",
        "runExhausts",
        "overflowRefuses",
        "muteHasNoToken",
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
        ["Draw.bounded", "Draw.context_bounded"],
        "a sampling generator has the same laws"
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
