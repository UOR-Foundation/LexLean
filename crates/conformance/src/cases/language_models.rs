//! Conformance cases for the language-1.2 text components (SPEC.md §17.10,
//! §17.12): LM-01..LM-03, the tokenizer and its laws, and LM-13, the laws as
//! LexLean source.
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
