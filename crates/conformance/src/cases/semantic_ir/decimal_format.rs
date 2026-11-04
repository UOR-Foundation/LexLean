//! SM-39: native policies, explicit boundary observations and preservation.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use lexlean::calculus::{self, Expr, Outcome, Ty, Value};
use lexlean::production::{certificate, lower, preserve};
use serde::Deserialize;

use crate::preservation;
use crate::support::{self, P};

const SOURCE: &str = include_str!("../../../../../tests/decimal-format/Main.lex.tex");
const CASES: &str = include_str!("../../../../../tests/decimal-format/cases.json");
const TYPES: [&str; 9] = [
    "UInt8", "UInt16", "UInt32", "UInt64", "Int8", "Int16", "Int32", "Int64", "Int",
];
const OWNER: &str = "LexLeanExample.Main";

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Case {
    id: String,
    kind: String,
    ty: String,
    input: String,
    expected: Option<String>,
}

impl Case {
    fn member(&self) -> String {
        format!("{OWNER}.{}{}", self.kind, self.ty)
    }

    fn argument(&self) -> String {
        if self.kind == "parse" {
            serde_json::to_string(&self.input).expect("ASCII string literal")
        } else {
            format!("({} : {})", self.input, self.ty)
        }
    }

    fn observation(&self) -> String {
        match (&*self.kind, &self.expected) {
            ("parse", None) => "none".to_owned(),
            ("parse", Some(value)) => format!("some {value}"),
            (_, Some(value)) => format!("value {value}"),
            (_, None) => panic!("{}: formatting has an expected string", self.id),
        }
    }

    fn fixed(&self, number: &str) -> Value {
        let kind = match self.ty.as_str() {
            "UInt8" => "u8",
            "UInt16" => "u16",
            "UInt32" => "u32",
            "UInt64" => "u64",
            "Int8" => "i8",
            "Int16" => "i16",
            "Int32" => "i32",
            "Int64" => "i64",
            other => panic!("{other}: unbounded Int is observed natively, never narrowed"),
        };
        serde_json::from_value(serde_json::json!({"kind": kind, "value": number}))
            .expect("a typed fixed-width boundary value")
    }

    fn input_value(&self) -> Value {
        if self.kind == "parse" {
            Value::String {
                value: self.input.clone(),
            }
        } else {
            self.fixed(&self.input)
        }
    }

    fn expected_value(&self) -> Value {
        match (&*self.kind, &self.expected) {
            ("parse", None) => Value::None,
            ("parse", Some(value)) => Value::Some {
                value: Box::new(self.fixed(value)),
            },
            (_, Some(value)) => Value::String {
                value: value.clone(),
            },
            (_, None) => panic!("{}: no expected output", self.id),
        }
    }
}

fn project(source: &str) -> P {
    let project = P::example();
    project.edit("lexlean.toml", "language = \"1.0\"", "language = \"1.2\"");
    project.write("src/Main.lex.tex", source);
    project.relock();
    project
}

/// Local strict decoding: the shared differential reader intentionally has a
/// broader contract; this case requires every indexed observation exactly once.
fn observations(output: &str, count: usize) -> Result<Vec<String>, String> {
    let mut rows = BTreeMap::new();
    for line in output.lines() {
        let (id, value) = line
            .strip_prefix("case ")
            .and_then(|rest| rest.split_once(": "))
            .ok_or_else(|| format!("unexpected observation line: {line:?}"))?;
        let id: usize = id.parse().map_err(|_| format!("invalid case ID: {id}"))?;
        if id >= count || rows.insert(id, value.to_owned()).is_some() {
            return Err(format!("duplicate or extra observation ID: {id}"));
        }
    }
    (0..count)
        .map(|id| {
            rows.remove(&id)
                .ok_or_else(|| format!("missing observation {id}"))
        })
        .collect()
}

fn matches_observations(actual: &[String], expected: &[String]) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!("actual {actual:?}; expected {expected:?}"))
    }
}

fn policies(verified: &lexlean::VerifiedProject) {
    let attestation: serde_json::Value = serde_json::from_slice(
        &std::fs::read(verified.root.join("attestation.json")).expect("attestation"),
    )
    .expect("attestation JSON");
    let rows = attestation["declarations"].as_array().expect("policy rows");
    let mut expected_names = BTreeSet::new();
    for ty in TYPES {
        for prefix in ["format", "call", "parse"] {
            let name = format!("{OWNER}.{prefix}{ty}");
            expected_names.insert(name.clone());
            let matching: Vec<_> = rows.iter().filter(|row| row["name"] == name).collect();
            assert_eq!(
                matching.len(),
                1,
                "one actual policy observation for {name}"
            );
            let expected: Vec<&str> = if prefix == "parse" {
                vec!["Classical.choice", "Quot.sound", "propext"]
            } else if ty == "Int" {
                vec!["propext"]
            } else {
                Vec::new()
            };
            let actual: Vec<_> = matching[0]["observed"]
                .as_array()
                .expect("observed axioms")
                .iter()
                .map(|axiom| axiom.as_str().expect("axiom name"))
                .collect();
            assert_eq!(actual, expected, "exact policy of {name}");
        }
    }
    let actual_names: BTreeSet<_> = rows
        .iter()
        .filter_map(|row| row["name"].as_str())
        .filter(|name| name.starts_with(&format!("{OWNER}.")))
        .map(str::to_owned)
        .collect();
    assert_eq!(
        actual_names, expected_names,
        "all 27 native declarations observed"
    );
}

/// Add only production metadata to the same definitions. The generated native
/// module must stay byte-identical to the one whose exact policies passed.
fn production_source() -> String {
    let mut source = SOURCE.to_owned();
    for ty in TYPES.into_iter().filter(|ty| *ty != "Int") {
        for prefix in ["format", "call", "parse"] {
            let name = format!("name: {prefix}{ty},");
            let start = source.find(&name).expect("fixture definition");
            let end = start + source[start..].find("\n}").expect("definition end");
            let block = &source[start..end];
            let effects = "allocation";
            let changed = block.replace(
                "executable: true",
                &format!("executable: true,\n  production: {{effects: [{effects}], targets: [rust-std]}}"),
            );
            assert_ne!(changed, block);
            source.replace_range(start..end, &changed);
        }
    }
    source
}

fn compile(root: &Path, module: &str, text: &str) -> String {
    let output = preservation::compile_text(root, module, text);
    assert!(output.status.success(), "{}", preservation::joined(&output));
    String::from_utf8(output.stdout).expect("UTF-8 observations")
}

fn native_observations(root: &Path, cases: &[Case]) {
    let mut text = format!("import {OWNER}\n");
    for (id, case) in cases.iter().enumerate() {
        let application = format!("{} {}", case.member(), case.argument());
        let value = if case.kind == "parse" {
            format!("(match {application} with | none => \"none\" | some value => \"some \" ++ toString value)")
        } else {
            format!("(\"value \" ++ ({application}))")
        };
        text.push_str(&format!("#eval IO.println (\"case {id}: \" ++ {value})\n"));
    }
    let output = compile(root, "SM39NativeObservations", &text);
    let actual = observations(&output, cases.len()).expect("exact native observation ID set");
    let expected: Vec<_> = cases.iter().map(Case::observation).collect();
    matches_observations(&actual, &expected).expect("all literal boundary and parse expectations");
    let plant = cases
        .iter()
        .position(|case| case.kind == "format" && case.ty == "UInt8" && case.input == "255")
        .expect("independent UInt8 maximum vector");
    assert_eq!(expected[plant], "value 255");
    let mut wrong = expected;
    wrong[plant] = "value 254".to_owned();
    assert!(
        matches_observations(&actual, &wrong).is_err(),
        "false output must be rejected"
    );
}

fn false_lowering(project: &P, root: &Path) {
    let checked = support::checked_project(project);
    let modules = lower::linked_modules(&checked);
    let roots = lower::roots(&checked).expect("production roots");
    let target = roots
        .iter()
        .find(|row| row.name == "formatUInt8")
        .expect("direct formatter root");
    let limits = support::limits(project);
    let original = lower::lower_root(
        &modules,
        &target.module,
        &target.name,
        target.report,
        &limits,
    )
    .expect("eligible formatter lowers");
    assert_eq!(original.program.functions[0].result, Ty::String);
    assert!(matches!(
        &original.layout.functions[0],
        lower::Origin::Definition { name, module, .. } if name == &target.name && module == &target.module
    ));
    assert_eq!(target.report.root, format!("{OWNER}.formatUInt8"));
    calculus::check::check(&original.program).expect("original valid program");
    let arguments = [Value::U8 {
        value: "255".to_owned(),
    }];
    let expect = |program: &calculus::Program, value: &str| {
        assert!(
            matches!(calculus::interp::run(program, 10_000_000, 0, &arguments),
            Outcome::Value { value: Value::String { value: actual }, .. } if actual == value)
        );
    };
    calculus::check::check_arguments(&original.program, 0, &arguments)
        .expect("original typed input");
    expect(&original.program, "255");
    let original_program = original.program.clone();
    let mut wrong = original;
    wrong.program.functions[0].body = Expr::Value {
        ty: Ty::String,
        value: Value::String {
            value: "254".to_owned(),
        },
    };
    let mut expected_program = original_program;
    expected_program.functions[0].body = wrong.program.functions[0].body.clone();
    assert_eq!(wrong.program, expected_program, "only body0 differs");
    calculus::check::check(&wrong.program).expect("mutant remains well-typed");
    calculus::check::check_arguments(&wrong.program, 0, &arguments).expect("mutant typed input");
    expect(&wrong.program, "254");
    match certificate::certificate(
        &modules,
        &target.module,
        &target.name,
        target.report,
        &wrong,
        "LexLeanPreserve.SM39FalseOutput",
        limits.max_file_bytes,
    ) {
        Err(error) => {
            assert_eq!(error.code.as_str(), "LLI9001");
            assert_eq!(
                error.message,
                "phase preservation: the lowering of a primitive is not a primitive"
            );
            eprintln!(
                "SM-39 false-output lowering: checked derivation refused before Lean: {}",
                error.message
            );
        }
        Ok(certificate) => {
            let output = preservation::compile_text(root, &certificate.module, &certificate.text);
            assert!(
                !output.status.success(),
                "a false-output preservation certificate was accepted"
            );
            let text = preservation::joined(&output);
            assert!(
                lexlean::verify::resource_death(
                    "SM39FalseOutput",
                    output.status.code().unwrap_or(-1),
                    &text,
                    certificate.text.len() as u64,
                    &limits
                )
                .is_none(),
                "resource failure is not rejection"
            );
            assert_eq!(
                preservation::failing_declaration(&certificate.text, &text).as_deref(),
                Some("__rel_0"),
                "targeted formatter relation: {text}"
            );
            eprintln!("SM-39 false-output lowering: Lean rejected formatter relation __rel_0");
        }
    }
}

fn certified_observations(
    project: &P,
    cases: &[Case],
    certified: &[preservation::Certified],
    root: &Path,
) {
    let mut denotes = Vec::new();
    for entry in certified {
        let mut vectors = Vec::new();
        for case in cases
            .iter()
            .filter(|case| case.ty != "Int" && case.member() == entry.root)
        {
            let argument = case.input_value();
            let outcome = calculus::interp::run(
                &entry.program,
                10_000_000,
                0,
                std::slice::from_ref(&argument),
            );
            assert!(
                matches!(&outcome, Outcome::Value { value, .. } if *value == case.expected_value()),
                "{}: interpreter disagrees with independent literal table: {outcome:?}",
                case.id
            );
            vectors.push(crate::differential::Case {
                root: entry.root.clone(),
                arguments: vec![case.argument()],
                values: vec![argument],
                outcome,
                function: 0,
                invalid: None,
            });
        }
        assert!(
            !vectors.is_empty(),
            "explicit boundary cases for {}",
            entry.root
        );
        denotes.push((
            entry.certificate.module.clone(),
            entry.certificate.denote.clone(),
            vectors,
        ));
    }
    let count: usize = denotes.iter().map(|(_, _, rows)| rows.len()).sum();
    assert_eq!(count, 136, "every fixed-width boundary case");
    assert_eq!(count, cases.iter().filter(|case| case.ty != "Int").count());
    let text = crate::differential::module(&denotes);
    let output = compile(root, "SM39CertifiedObservations", &text);
    let actual = observations(&output, count).expect("exact certified observation ID set");
    let expected: Vec<_> = denotes
        .iter()
        .flat_map(|(_, _, cases)| cases)
        .map(|case| crate::differential::observed(&case.outcome))
        .collect();
    matches_observations(&actual, &expected)
        .expect("certified denotations match every fixed vector");
    false_lowering(project, root);
}

pub(super) fn run() {
    let cases: Vec<Case> = serde_json::from_str(CASES).expect("literal boundary table");
    assert_eq!(cases.len(), 151, "all original 18 theorem conjuncts");
    assert_eq!(cases.iter().filter(|case| case.kind == "parse").count(), 97);
    assert_eq!(
        cases
            .iter()
            .map(|case| &case.id)
            .collect::<BTreeSet<_>>()
            .len(),
        cases.len()
    );
    let accepted = project(SOURCE);
    accepted.check_ok();
    // This intentionally fails with fixed-width LLV7005 before implementation.
    let Some(verified) = support::verify_ok_backed("SM-39", &accepted) else {
        return;
    };
    policies(&verified);
    let before: BTreeMap<_, _> = support::rendered(&accepted)
        .modules
        .into_iter()
        .map(|module| (module.lean_module, module.lean_text))
        .collect();
    accepted.write("src/Main.lex.tex", &production_source());
    accepted.relock();
    let after: BTreeMap<_, _> = support::rendered(&accepted)
        .modules
        .into_iter()
        .map(|module| (module.lean_module, module.lean_text))
        .collect();
    assert_eq!(
        after, before,
        "production metadata preserves the audited native function bytes"
    );
    let checked_roots = support::checked_project(&accepted);
    for row in lower::roots(&checked_roots).expect("fixed roots") {
        let expected = vec!["allocation".to_owned()];
        assert_eq!(
            row.report.declared_effects, expected,
            "exact effects for {}",
            row.report.root
        );
    }
    let certified = preservation::certificates(&accepted);
    assert_eq!(
        certified.len(),
        24,
        "all fixed formatter/caller/parser roots"
    );
    let expected_roots: BTreeSet<_> = TYPES
        .into_iter()
        .filter(|ty| *ty != "Int")
        .flat_map(|ty| ["format", "call", "parse"].map(|prefix| format!("{OWNER}.{prefix}{ty}")))
        .collect();
    assert_eq!(
        certified
            .iter()
            .map(|entry| entry.root.clone())
            .collect::<BTreeSet<_>>(),
        expected_roots
    );
    for entry in &certified {
        assert_eq!(entry.targets, ["rust-std"]);
        assert_eq!(entry.renderings.len(), 1, "certificate B");
        assert_eq!(entry.composed.len(), 1, "certificate E");
    }
    let workspace = preservation::staged(&accepted, &certified);
    let scratch = tempfile::tempdir().expect("bound native/certificate workspace");
    let checked = preservation::check(&workspace, scratch.path());
    assert!(checked.failure.is_none(), "{:?}", checked.failure);
    let certificates: Vec<_> = certified
        .iter()
        .flat_map(preservation::Certified::all)
        .collect();
    preserve::audit(&checked.audit_output, &certificates)
        .expect("exact A/B/E and library axiom policies");
    native_observations(scratch.path(), &cases);
    certified_observations(&accepted, &cases, &certified, scratch.path());

    let start = SOURCE.find("name: formatInt,").expect("Int formatter");
    let end = start + SOURCE[start..].find("\n}").expect("Int formatter end");
    let original = &SOURCE[start..end];
    let changed = original.replacen("axioms: [propext],", "axioms: [],", 1);
    assert_ne!(changed, original);
    let mut empty_int = SOURCE.to_owned();
    empty_int.replace_range(start..end, &changed);
    assert_eq!(
        empty_int.matches("axioms: [propext],").count(),
        1,
        "caller policy unchanged"
    );
    let wrong_policy = project(&empty_int);
    wrong_policy.check_ok();
    let error = wrong_policy.verify_fails_with("LLV7005");
    assert_eq!(
        error.diagnostics.len(),
        1,
        "only the Int formatter policy is changed"
    );
    let diagnostic = &error.diagnostics[0];
    assert_eq!(diagnostic.code.as_str(), "LLV7005");
    assert_eq!(
        diagnostic.message,
        "`LexLeanExample.Main.formatInt` violates its none axiom policy: observed [propext]"
    );
}
