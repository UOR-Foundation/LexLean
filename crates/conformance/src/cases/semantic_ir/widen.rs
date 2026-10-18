//! SM-32: the language-1.2 widening conversions `to_int` and `to_nat`
//! (SPEC.md §17.12, *Widening conversions*).

use lexlean::production::lower::{linked_modules, lower_root, roots};

use crate::support::{self, P};

/// Every fixed-width type: its notation, whether it is signed, and its width.
const WIDTHS: [(&str, bool, u32); 8] = [
    ("UInt8", false, 8),
    ("Int8", true, 8),
    ("UInt16", false, 16),
    ("Int16", true, 16),
    ("UInt32", false, 32),
    ("Int32", true, 32),
    ("UInt64", false, 64),
    ("Int64", true, 64),
];

/// The values at the bounds of a fixed-width type, as canonical decimals:
/// its least and greatest values, and the values that set exactly the top
/// bit, every bit but the top one, and the least bit.
fn bounds(signed: bool, width: u32) -> Vec<String> {
    if signed {
        let max = (1_i128 << (width - 1)) - 1;
        let min = -(1_i128 << (width - 1));
        vec![
            min.to_string(),
            (-1).to_string(),
            "0".to_owned(),
            "1".to_owned(),
            max.to_string(),
        ]
    } else {
        let max = (1_u128 << width) - 1;
        let top = 1_u128 << (width - 1);
        vec![
            "0".to_owned(),
            "1".to_owned(),
            (top - 1).to_string(),
            top.to_string(),
            max.to_string(),
        ]
    }
}

/// A language-1.2 module stating, for every fixed-width type and each of its
/// bounds, that `to_int` is the value itself, and for every unsigned type
/// that `to_nat` is too; each is decided by the kernel over the elaborated
/// terms, which is what the Lean module states.
fn bounds_module() -> String {
    let mut out = String::from(
        "\\begin{lexlean}{Main}\n\\useglossary{lexlean.std.nat@1.2.0}\n\\title{Natural number addition}\n\\begin{semanticmodule}\n",
    );
    let mut first = true;
    for (ty, signed, width) in WIDTHS {
        for (index, value) in bounds(signed, width).into_iter().enumerate() {
            let literal = format!("({value} : {ty})");
            let mut theorem = |name: String, statement: String| {
                if !first {
                    out.push('\n');
                }
                first = false;
                out.push_str(&format!(
                    "theorem{{\n  name: {name},\n  parameters: [],\n  statement: {statement},\n  proof: reflexivity{{}},\n  axioms: [Quot.sound, propext]\n}}\n"
                ));
            };
            theorem(
                format!("int_{}_{index}", ty.to_lowercase()),
                format!("((prim to_int {literal} : Int) = ({value} : Int))"),
            );
            if !signed {
                theorem(
                    format!("nat_{}_{index}", ty.to_lowercase()),
                    format!("((prim to_nat {literal} : Nat) = {value})"),
                );
            }
        }
    }
    out.push_str("\\end{semanticmodule}\n\\end{lexlean}\n");
    out
}

pub(super) fn run() {
    // The coverage example's roots widen every fixed-width type and decode
    // a byte string; nothing downstream of linking sees the conversions.
    let project = P::copy_example("production-coverage");
    project.check_ok();
    let rendered = support::rendered(&project);
    let lean = support::lean_text(&rendered, "Prims");
    let (_, tail) = lean
        .split_once("@[expose] public def widenings ")
        .expect("widenings definition is emitted");
    for absent in ["toInt", "toNat", "to_int", "to_nat"] {
        assert!(
            !tail.contains(absent),
            "the generated Lean for `widenings` and `pixels` names `{absent}`; it must state the elaboration:\n{tail}"
        );
    }
    for expected in [
        "@[expose] public def widenings ",
        "@[expose] public def pixels ",
        "(let __widen0 : UInt8 := u8;",
        "LexLeanRuntime.bitAnd",
        "LexLeanRuntime.equal",
    ] {
        assert!(lean.contains(expected), "missing {expected:?} in:\n{lean}");
    }
    // The canonical document states the source conversions.
    let tex = support::tex_text(&rendered, "Prims");
    assert!(
        tex.contains("toInt") && tex.contains("toNat"),
        "the document states the conversions"
    );

    // The snapshot records each widening definition's elaboration.
    let snapshot = project
        .engine()
        .snapshot(lexlean::CheckRequest {
            selection: lexlean::Selection::Entrypoints,
        })
        .expect("snapshot");
    let snapshot = String::from_utf8(snapshot.canonical_bytes()).expect("UTF-8 snapshot");
    assert!(
        snapshot.contains("\"name\":\"__widen0\""),
        "the snapshot carries the elaborated binder"
    );

    // The lowered program of each widening root realizes the conversions by
    // bit tests and exact sums, never through decimal text, and its report
    // states the conversions it uses.
    let checked = support::checked_project(&project);
    let modules = linked_modules(&checked);
    let limits = support::limits(&project);
    let all = roots(&checked).expect("the eligibility reports");
    for (root, constructs) in [
        (
            "Coverage.Prims.widenings",
            &["primitive.to_int", "primitive.to_nat"][..],
        ),
        ("Coverage.Prims.pixels", &["primitive.to_int"][..]),
    ] {
        let found = all
            .iter()
            .find(|candidate| candidate.report.root == root)
            .unwrap_or_else(|| panic!("{root} is a production root"));
        for construct in constructs {
            assert!(
                found.report.constructs.contains_key(*construct),
                "{root} reports {construct}: {:?}",
                found.report.constructs.keys().collect::<Vec<_>>()
            );
        }
        let lowered = lower_root(&modules, &found.module, &found.name, found.report, &limits)
            .unwrap_or_else(|diagnostic| panic!("{root}: lowering failed: {diagnostic:?}"));
        let program = serde_json::to_string(&lowered.program).expect("program JSON");
        for decimal in ["\"kind\":\"parse_decimal\"", "\"kind\":\"format_decimal\""] {
            assert!(
                !program.contains(decimal),
                "{root} lowers through decimal text ({decimal})"
            );
        }
        assert!(
            program.contains("\"kind\":\"bit_and\""),
            "{root} tests the operand's bits"
        );
    }

    // A production root over `UInt64` and `Int64` reaches `to_int`'s overflow
    // at or above 2^63 (§17.12 *Widening conversions* rule 2), while inputs
    // below 2^63 evaluate to their exact integer values.
    {
        use lexlean::calculus::{interp, Outcome, Value};
        let wide = P::copy_example("production");
        let wide_src = "\\begin{lexlean}{Main}\n\\useglossary{lexlean.std.nat@1.2.0}\n\\title{Natural number addition}\n\\begin{semanticmodule}\ndefinition{\n  name: widenWide,\n  parameters: [u64 : UInt64, i64 : Int64],\n  result: (Int * Nat),\n  body: ((prim subtract (prim to_int u64 : Int) (prim to_int i64 : Int) : Int), (prim to_nat u64 : Nat)),\n  axioms: [Quot.sound, propext],\n  executable: true,\n  production: {effects: [overflow], targets: [rust-core, rust-std]}\n}\n\\end{semanticmodule}\n\\end{lexlean}\n";
        wide.write("src/Main.lex.tex", wide_src);
        wide.check_ok();
        let certified = crate::preservation::certificates(&wide);
        assert_eq!(certified.len(), 1, "one production root");
        let program = &certified[0].program;
        let run = |u: u64, i: i64| -> Outcome {
            interp::run(
                program,
                100_000,
                0,
                &[
                    Value::U64 {
                        value: u.to_string(),
                    },
                    Value::I64 {
                        value: i.to_string(),
                    },
                ],
            )
        };
        match run((1_u64 << 63) - 1, 0) {
            Outcome::Value {
                value: Value::Pair { left, right },
                ..
            } => {
                assert_eq!(
                    *left,
                    Value::Int {
                        value: i64::MAX.to_string()
                    }
                );
                assert_eq!(
                    *right,
                    Value::Nat {
                        value: ((1_u64 << 63) - 1).to_string()
                    }
                );
            }
            other => panic!("expected Value at 2^63 - 1, got {other:?}"),
        }
        for u in [1_u64 << 63, u64::MAX] {
            let outcome = run(u, 0);
            assert!(
                matches!(outcome, Outcome::Overflow { .. }),
                "`to_int` on UInt64 {u} (>= 2^63) must overflow Int64 in the calculus: got {outcome:?}"
            );
        }
    }

    // The kernel decides every width at its bounds.
    let bounds = P::language_1_2_example();
    bounds.write("src/Main.lex.tex", &bounds_module());
    bounds.check_ok();
    let main = support::lean_text(&support::rendered(&bounds), "Main");
    let (_, theorems) = main
        .split_once("public theorem ")
        .expect("bounds theorems are emitted");
    assert!(
        !theorems.contains("toInt") && !theorems.contains("toNat"),
        "{theorems}"
    );
    let _ = support::verify_ok_backed("SM-32", &bounds);

    // Misuse fails in linking, before any build output.
    let reject = |from: &str, to: &str, message: &str| {
        P::assert_mutation_rejected(
            "production-coverage",
            "src/Prims.lex.tex",
            from,
            to,
            message,
        );
    };
    reject(
        r#"{"arguments":[{"kind":"var","name":"u8"}],"kind":"primitive","operation":"to_int","result":{"kind":"int"}}"#,
        r#"{"arguments":[{"kind":"string","value":"a"}],"kind":"primitive","operation":"to_int","result":{"kind":"int"}}"#,
        "primitive ToInt requires a fixed-width integer, not String",
    );
    reject(
        r#"{"arguments":[{"kind":"var","name":"u8"}],"kind":"primitive","operation":"to_nat","result":{"kind":"nat"}}"#,
        r#"{"arguments":[{"kind":"var","name":"i8"}],"kind":"primitive","operation":"to_nat","result":{"kind":"nat"}}"#,
        "primitive ToNat requires an unsigned fixed-width integer, not Int8",
    );
    reject(
        r#"{"arguments":[{"kind":"var","name":"u8"}],"kind":"primitive","operation":"to_int","result":{"kind":"int"}}"#,
        r#"{"arguments":[{"kind":"var","name":"u8"}],"kind":"primitive","operation":"to_int","result":{"kind":"nat"}}"#,
        "primitive ToInt result",
    );

    // Language 1.1 rejects the conversions before either backend runs.
    let earlier = P::copy_example("semantic-1.1");
    let source = earlier.read("src/Portable.lex.tex");
    let from = r#""operation":"format_decimal","result":{"kind":"string"}"#;
    assert!(source.contains(from), "the 1.1 example formats a decimal");
    earlier.write(
        "src/Portable.lex.tex",
        &source.replacen(from, r#""operation":"to_int","result":{"kind":"int"}"#, 1),
    );
    let error = earlier.check_fails_with("LLT4001");
    assert!(
        error
            .to_string()
            .contains("`widening primitive` is a language-1.2 construct"),
        "{error}"
    );
    earlier.assert_no_backend_output();
}
