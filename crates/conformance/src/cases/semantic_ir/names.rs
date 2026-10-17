//! Accepted semantic identifiers must survive the pinned Lean parser.

use crate::support::{self, P};
use serde_json::json;

fn rename(name: &str) -> String {
    name.split('.')
        .map(|part| match part {
            "ComponentKind" => "inductive",
            "Component" => "structure",
            "Box" => "abbrev",
            "Validatable" => "class",
            "defaultValidatable" => "instance",
            "sampleComponent" => "def",
            "allConsecutive" => "while",
            "allConsecutive_sound_complete" => "theorem",
            "values" => "else",
            "expected" => "then",
            "value" => "prefix",
            "rest" => "in",
            "left" => "from",
            "right" => "at",
            "pair" => "match",
            "A" => "Type",
            "T" => "Sort",
            "service" => "where",
            "database" => "end",
            other => other,
        })
        .collect::<Vec<_>>()
        .join(".")
}

fn rename_identifiers(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(fields) => {
            for (key, value) in fields {
                if matches!(
                    key.as_str(),
                    "name"
                        | "field"
                        | "recursive_argument"
                        | "scrutinee"
                        | "parameter"
                        | "record"
                        | "values"
                        | "constructor"
                ) {
                    if let Some(name) = value.as_str() {
                        *value = json!(rename(name));
                        continue;
                    }
                }
                if matches!(key.as_str(), "type_parameters" | "binders" | "generalizing") {
                    for name in value.as_array_mut().expect("identifier array") {
                        *name = json!(rename(name.as_str().expect("identifier")));
                    }
                } else {
                    rename_identifiers(value);
                }
            }
        }
        serde_json::Value::Array(values) => values.iter_mut().for_each(rename_identifiers),
        _ => {}
    }
}

// Reuse the entire independently verified semantic example so names cross
// module, class, instance, recursion, pattern and proof/reflection boundaries.
// Only typed identifier positions change; literals and term kinds do not.
fn verify_all_variants() {
    let project = support::semantic_project();
    let mut changed = std::collections::BTreeSet::new();
    for entry in std::fs::read_dir(project.root.join("src")).expect("semantic sources") {
        let path = entry
            .expect("source entry")
            .file_name()
            .into_string()
            .expect("UTF-8 source");
        let relative = format!("src/{path}");
        let original = project.read(&relative);
        let mut value = crate::support::module_json(&original);
        let before = value.clone();
        rename_identifiers(&mut value);
        if value != before {
            changed.insert(path);
        }
        let rewritten = crate::support::with_module_json(&original, &value);
        project.write(&relative, &rewritten);
    }
    assert_eq!(
        changed,
        [
            "Main.lex.tex",
            "Portable.lex.tex",
            "PortableRecursion.lex.tex",
            "VariantInstances.lex.tex",
            "VariantLogic.lex.tex",
            "VariantProofs.lex.tex",
            "VariantTerms.lex.tex",
            "VariantTypes.lex.tex"
        ]
        .into_iter()
        .map(str::to_owned)
        .collect(),
        "every expected identifier-bearing module was exercised"
    );
    project.relock();
    let _ = support::verify_ok_backed("SM-19", &project);
}

pub(super) fn verify() {
    let project = P::example();
    project.edit("lexlean.toml", "language = \"1.0\"", "language = \"1.1\"");
    let member = |name: &str| json!({"name":name});
    let named = |name: &str| json!({"kind":"named","member":member(name),"arguments":[]});
    let value = json!({"kind":"nat","value":"7"});
    let constructed = json!({"kind":"record","type":member("Record"),"type_arguments":[],
        "fields":[{"field":"namespace","value":value}]});
    let module = json!({"spec":"lexlean/semantic-module/1","declarations":[
        {"kind":"structure","name":"Record","type_parameters":[],"parameters":[],
         "fields":[{"name":"namespace","type":{"kind":"nat"}}]},
        {"kind":"definition","name":"read","parameters":[{"name":"prefix","type":named("Record")}],
         "result":{"kind":"nat"},"body":{"kind":"project","value":{"kind":"var","name":"prefix"},"field":"namespace"}},
        {"kind":"theorem","name":"result","parameters":[],
         "statement":{"kind":"eq","left":{"kind":"call","function":member("read"),"arguments":[constructed]},"right":value},
         "proof":{"kind":"reflexivity"},"axioms":[]}
    ]});
    project.write("src/Main.lex.tex", &format!(
        "\\begin{{lexlean}}{{Main}}\n\\useglossary{{lexlean.std.nat@1.1.0}}\n\\title{{Natural number addition}}\n\n\\begin{{semanticmodule}}\n\\semanticdata{{{module}}}\n\\end{{semanticmodule}}\n\\end{{lexlean}}\n"
    ));
    project.relock();
    let _ = support::verify_ok_backed("SM-19", &project);
    verify_all_variants();
}
