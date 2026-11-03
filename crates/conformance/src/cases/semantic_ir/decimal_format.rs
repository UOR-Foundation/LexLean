//! SM-39: formatting must not carry the parser's instance evidence.
//! Closed boundary proofs require #396's kernel-decision source integration.

use crate::support::{self, P};

const SOURCE: &str = include_str!("../../../../../tests/decimal-format/Main.lex.tex");
const TYPES: [&str; 9] = [
    "UInt8", "UInt16", "UInt32", "UInt64", "Int8", "Int16", "Int32", "Int64", "Int",
];

fn project(source: &str) -> P {
    let project = P::example();
    project.edit("lexlean.toml", "language = \"1.0\"", "language = \"1.2\"");
    project.write("src/Main.lex.tex", source);
    project.relock();
    project
}

pub(super) fn run() {
    let accepted = project(SOURCE);
    accepted.check_ok();
    // After #396 integration, this must first fail with LLV7005 for a
    // fixed-width formatter on the unchanged Decimal-coupled runtime.
    // An unsupported decision proof is not that R3 witness.
    let Some(verified) = support::verify_ok_backed("SM-39", &accepted) else {
        return;
    };
    // The attestation records the verification's actual per-declaration
    // observations; aggregated runtime audit text is not a Main-only audit.
    let attestation: serde_json::Value = serde_json::from_slice(
        &std::fs::read(verified.root.join("attestation.json")).expect("attestation"),
    )
    .expect("attestation JSON");
    let declarations = attestation["declarations"].as_array().expect("policy rows");
    for ty in TYPES {
        for prefix in ["format", "call", "parse", "formatBounds", "parseControls"] {
            let name = format!("LexLeanExample.Main.{prefix}{ty}");
            let rows = declarations
                .iter()
                .filter(|row| row["name"].as_str() == Some(name.as_str()))
                .collect::<Vec<_>>();
            assert_eq!(rows.len(), 1, "exactly one observation for {name}");
            let observed = rows[0]["observed"]
                .as_array()
                .expect("observed axioms")
                .iter()
                .map(|value| value.as_str().expect("axiom name"))
                .collect::<Vec<_>>();
            let expected = if prefix.starts_with("parse") {
                vec!["Classical.choice", "Quot.sound", "propext"]
            } else if ty == "Int" {
                vec!["propext"]
            } else {
                Vec::new()
            };
            assert_eq!(observed, expected, "{name}");
        }
    }
    let empty_int = SOURCE.replace("axioms: [propext],", "axioms: [],");
    assert_ne!(empty_int, SOURCE);
    let wrong_policy = project(&empty_int);
    wrong_policy.check_ok();
    wrong_policy.verify_fails_with("LLV7005");

    let boundary = "((formatUInt8 (255 : UInt8)) = \"255\")";
    assert_eq!(SOURCE.matches(boundary).count(), 1);
    let wrong_source = SOURCE.replace(boundary, "((formatUInt8 (255 : UInt8)) = \"254\")");
    let wrong_boundary = project(&wrong_source);
    wrong_boundary.check_ok();
    wrong_boundary.verify_fails_with("LLV7002");
}
