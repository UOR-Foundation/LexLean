//! SM-39: fixed UTF-8 simplification bridges, without open proof syntax.

use crate::support::{self, P};

const BRIDGES: &str = "LexLeanRuntime.utf8Encode, String.toUTF8_eq_toByteArray";

fn source(bridges: &str, wrong: bool) -> String {
    let mut declarations = String::new();
    for (index, (text, bytes)) in [
        ("", ""),
        ("problem1", "70726f626c656d31"),
        ("é😀", "c3a9f09f9880"),
    ]
    .into_iter()
    .enumerate()
    {
        let literal = serde_json::to_string(text).expect("string literal");
        let bytes = if wrong && index == 1 { "00" } else { bytes };
        declarations.push_str(&format!(
            "theorem{{name: literal{index}, parameters: [], statement: ((prim utf8_encode {literal} : Bytes) = 0x{bytes}), proof: simplify{{definitions: [{bridges}]}}, axioms: []}}\n"
        ));
    }
    format!(
        "\\begin{{lexlean}}{{Main}}\n\\useglossary{{lexlean.std.nat@1.2.0}}\n\\title{{Natural number addition}}\n\\begin{{semanticmodule}}\n{declarations}definition{{name: natural, parameters: [], result: Nat, body: 0}}\ntheorem{{name: naturalSame, parameters: [], statement: ((natural) = 0), proof: simplify{{definitions: [natural]}}, axioms: [propext]}}\n\\end{{semanticmodule}}\n\\end{{lexlean}}\n"
    )
}

fn project(text: &str, language: &str) -> P {
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

pub fn run() {
    let accepted = project(&source(BRIDGES, false), "1.2");
    accepted.check_ok();
    let (exit, _, errors) = accepted.cli(&["fmt"]);
    assert_eq!(exit, 0, "{errors}");
    accepted.fmt_check_ok();
    let formatted = accepted.read("src/Main.lex.tex");
    let built = accepted.build_ok();
    let lean = std::fs::read_to_string(
        accepted
            .build_dir(&built.build_id.expect("build identity"))
            .join("modules/LexLeanExample/Main.lean"),
    )
    .expect("generated module");
    assert_eq!(lean.matches("simp only [_root_.LexLeanRuntime.utf8Encode, _root_.String.toUTF8_eq_toByteArray] <;> rfl").count(), 3);
    assert!(
        lean.contains("simp only [natural]\n"),
        "historical simplify changed: {lean}"
    );
    support::verify_ok_backed("SM-39", &accepted);

    for names in [
        "String.toUTF8",
        "LexLeanRuntime.utf8Encode, String.toUTF8_eq_toByteArray, String.toUTF8_eq_toByteArray",
        "String.toUTF8_eq_toByteArray, LexLeanRuntime.utf8Encode",
        "",
    ] {
        let refused = project(&source(names, false), "1.2");
        let error = refused.check_err();
        support::expect_code(&error, "LLT4001");
    }
    let old = project(
        &formatted
            .replace("@1.2.0", "@1.1.0")
            .replace("lexlean/semantic-module/2", "lexlean/semantic-module/1"),
        "1.1",
    );
    support::expect_code(&old.check_err(), "LLT4001");

    for name in ["LexLeanRuntime.utf8Encode", "String.toUTF8_eq_toByteArray"] {
        let shadow = source(BRIDGES, false).replace("name: natural,", &format!("name: {name},"));
        let error = project(&shadow, "1.2").check_err();
        assert!(format!("{error:?}").contains("reserved"), "{error:?}");
    }

    let false_equality = project(&source(BRIDGES, true), "1.2");
    false_equality.check_ok();
    if support::lean_backed("SM-39") {
        false_equality.verify_fails_with("LLV7002");
    }
}
