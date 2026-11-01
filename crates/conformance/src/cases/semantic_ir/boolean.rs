//! SM-36: a language-1.2 `cases` proof on a Boolean local (SPEC.md §17.10).

use crate::support::{self, P};

fn source(proof: &str) -> String {
    format!(
        "\\begin{{lexlean}}{{Main}}\n\\useglossary{{lexlean.std.nat@1.2.0}}\n\\title{{Natural number addition}}\n\\begin{{semanticmodule}}\ndefinition{{\n  name: pick,\n  parameters: [b : Bool],\n  result: Bool,\n  body: (if b then true else false)\n}}\n\ntheorem{{\n  name: pickSame,\n  parameters: [b : Bool],\n  statement: ((pick b) = b),\n  proof: {proof},\n  axioms: []\n}}\n\\end{{semanticmodule}}\n\\end{{lexlean}}\n"
    )
}

fn project_of(text: &str, language: &str) -> P {
    let project = P::example();
    project.edit(
        "lexlean.toml",
        "language = \"1.0\"",
        &format!("language = \"{language}\""),
    );
    project.write("src/Main.lex.tex", text);
    project.relock();
    project
}

/// Run the case of `SM-36`.
pub fn run() {
    let branch = |constructor: &str, binders: &str| {
        format!("{{proof: reflexivity{{}}, binders: [{binders}], constructor: \"{constructor}\"}}")
    };
    let good = format!(
        "cases{{scrutinee: b, branches: [{}, {}]}}",
        branch("false", ""),
        branch("true", "")
    );
    let accepted = project_of(&source(&good), "1.2");
    let checked = support::checked_project(&accepted);
    let core = serde_json::to_value(
        checked.modules["Main"]
            .document
            .semantic
            .as_ref()
            .expect("a semantic module"),
    )
    .expect("module JSON");
    support::assert_schema("semantic-module-v2", "a cases proof on a Boolean", &core);
    // The formatter keeps the proof, and the canonical source is a fixed
    // point that links to the same module.
    let (code, _, error) = accepted.cli(&["fmt"]);
    assert_eq!(code, 0, "{error}");
    let formatted = accepted.read("src/Main.lex.tex");
    assert!(
        formatted.contains("\"kind\":\"cases\"") && formatted.contains("\"constructor\":\"true\""),
        "the formatter keeps the proof: {formatted}"
    );
    let notation = lexlean::sdk::to_notation(&formatted).expect("the notation");
    assert_eq!(
        lexlean::sdk::read_module(&notation).expect("reads back"),
        lexlean::sdk::read_module(&formatted).expect("reads"),
        "the notation is the same module"
    );
    accepted.relock();
    accepted.fmt_check_ok();
    let built = accepted.build_ok();
    let build_dir = accepted.build_dir(&built.build_id.expect("a build id"));
    let lean = std::fs::read_dir(build_dir.join("modules").as_std_path())
        .expect("the modules")
        .flatten()
        .flat_map(|entry| {
            std::fs::read_dir(entry.path())
                .into_iter()
                .flatten()
                .flatten()
        })
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "lean"))
        .map(|entry| std::fs::read_to_string(entry.path()).expect("Lean text"))
        .collect::<String>();
    assert!(
        lean.contains("cases b with") && lean.contains("| false =>") && lean.contains("| true =>"),
        "{lean}"
    );
    support::verify_ok_backed("SM-36", &accepted);
    // The misuses are refused by the linker.
    let one = format!("cases{{scrutinee: b, branches: [{}]}}", branch("false", ""));
    let twice = format!(
        "cases{{scrutinee: b, branches: [{}, {}, {}]}}",
        branch("false", ""),
        branch("false", ""),
        branch("true", "")
    );
    let bound = format!(
        "cases{{scrutinee: b, branches: [{}, {}]}}",
        branch("false", "extra"),
        branch("true", "")
    );
    let induction = format!(
        "induction{{scrutinee: b, branches: [{}, {}]}}",
        branch("false", ""),
        branch("true", "")
    );
    for (what, proof, language, message) in [
        (
            "language 1.1",
            good.as_str(),
            "1.1",
            "cases on a Boolean local",
        ),
        ("a missing branch", one.as_str(), "1.2", "not exhaustive"),
        (
            "a repeated branch",
            twice.as_str(),
            "1.2",
            "duplicate proof branch",
        ),
        (
            "a branch with a binder",
            bound.as_str(),
            "1.2",
            "expects 0 binder",
        ),
        (
            "induction on a Boolean",
            induction.as_str(),
            "1.2",
            "induction on the Boolean",
        ),
    ] {
        let text = source(proof);
        let text = if language == "1.1" {
            // Language 1.1 reads the data form only.
            formatted
                .replace("@1.2.0", "@1.1.0")
                .replace("lexlean/semantic-module/2", "lexlean/semantic-module/1")
        } else {
            text
        };
        let planted = project_of(&text, language);
        let error = planted.check_err();
        let shown = format!("{error:?}");
        assert!(shown.contains(message), "{what}: {shown}");
    }
}
