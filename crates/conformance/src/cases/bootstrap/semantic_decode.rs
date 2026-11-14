//! `BS-73` (SPEC.md §17.23, §17.12): decoding a semantic module from its
//! canonical JSON, as language-1.2 source, `LexLeanBoot.SemanticDecode`, held
//! to the decoding half of `lexlean::ir::semantic::SemanticModule::parse`.
//!
//! `decodeSemanticModule` reads a text and returns the module's discriminator
//! and declarations, or which of the parser's three decoding rejections the
//! text meets first: invalid semantic-module JSON (serde's reading of the
//! text, with whitespace, members in any order, and every escape, and its
//! typed decoding,
//! with its unknown-member, missing-field, duplicate-field and 128-level
//! nesting rules), JSON that is not canonical, or a member outside the closed
//! schema, with its path. The domain is the texts that the canonical JSON
//! parser accepts or that serde rejects first; a text the parser decodes and
//! then fails to validate decodes. The decoders are generated from the same
//! description of the oracle's types as the writers of `BS-68`..`BS-72`.
//!
//! Every semantic module of every committed project that links is a run, and
//! so are modules of examples reaching every model, reasoning and text
//! variant, a module nested to the 127 levels serde admits and one nested to
//! the 128 it rejects, and mutations of these modules: a member removed, an
//! unknown member added (with a value of `0`, `false`, or `[]`), an omitted
//! default written out, a kind renamed, a value of another type, a negative
//! count, and texts out of canonical form.
//!
//! Each planted defect weakens one decoding rule and is detected by those runs.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use lexlean::artifact::canonical_json::Json;
use lexlean::calculus::Value;
use lexlean::ir::semantic::model::ArtifactStore;
use lexlean::ir::semantic::SemanticModule;
use serde_json::Value as Json2;

use super::ir_document::linked_projects;
use super::rust_validate::{eligibility_of, rustc_differential};
use super::semantic_models::example_model_declaration;
use super::semantic_reasoning::{enc_any, example_reasoning_declaration};
use super::semantic_types::canonical;
use super::{list, text, Run};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.SemanticDecode";
const SOURCE: &str = "bootstrap/src/SemanticDecode.lex.tex";
const ROOTS: [(&str, &[&str]); 1] = [(
    "decodeSemanticModule",
    &["allocation", "overflow", "recursion"],
)];
/// A committed module whose JSON is longer decodes under rustc only.
const EVALUATOR_BYTES: usize = 16_384;
/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// The example modules: one per step, each holding a model and a reasoning
/// example declaration of that step.
const EXAMPLES: usize = 60;
/// The objects of a base module that are mutated, at most.
const MUTATED_OBJECTS: usize = 12;
/// The members of an object that are removed one at a time, at most.
const REMOVED_MEMBERS: usize = 4;
/// Defaults serialization omits, written out by a mutation where absent.
const DEFAULTS: &[(&str, bool)] = &[
    ("axioms", false),
    ("executable", true),
    ("type_parameters", false),
    ("type_arguments", false),
    ("definitions", false),
    ("generalizing", false),
    ("premises", false),
];

// ---------------------------------------------------------------- the sorts

fn constructors(sort: &str) -> &'static Vec<String> {
    static TABLES: OnceLock<BTreeMap<String, Vec<String>>> = OnceLock::new();
    let tables = TABLES.get_or_init(|| {
        let source = std::fs::read_to_string(repo_root().join(SOURCE).as_std_path())
            .expect("the module's source");
        let mut out = BTreeMap::new();
        for block in source.split("inductive{").skip(1) {
            let block = block.split("\n}\n").next().expect("a block");
            let name = block
                .split("name: ")
                .nth(1)
                .and_then(|rest| rest.split(',').next())
                .expect("the inductive's name")
                .trim()
                .to_owned();
            let names: Vec<String> = block
                .split("constructors:")
                .nth(1)
                .expect("the constructors")
                .split("name: ")
                .skip(1)
                .map(|rest| {
                    rest.split([',', '\n'])
                        .next()
                        .expect("a constructor")
                        .trim()
                        .to_owned()
                })
                .collect();
            out.insert(name, names);
        }
        out
    });
    tables
        .get(sort)
        .unwrap_or_else(|| panic!("the module has no inductive `{sort}`"))
}

fn decoded(constructor: &str, fields: Vec<Value>) -> Value {
    let found = constructors("Decoded")
        .iter()
        .position(|name| name == constructor)
        .unwrap_or_else(|| panic!("`Decoded` has no constructor `{constructor}`"));
    Value::Adt {
        constructor: u64::try_from(found).expect("an index"),
        fields,
    }
}

// --------------------------------------------------------------- the oracle

/// What the parser makes of a text, as the value the root returns, or `None`
/// for a text outside the domain: one serde decodes but the canonical JSON
/// parser rejects.
fn oracle(source: &str) -> Option<Value> {
    let store = ArtifactStore::new(u64::MAX, u64::MAX);
    let reason = match SemanticModule::parse(source, "1.2", "", &[], &BTreeMap::new(), &store) {
        Ok(_) => None,
        Err(failure) => Some(failure.reason),
    };
    if let Some(reason) = &reason {
        if reason.starts_with("invalid semantic-module JSON") {
            return Some(decoded("DInvalid", Vec::new()));
        }
        if reason == "semantic-module JSON is not canonical" {
            return Some(decoded("DNotCanonical", Vec::new()));
        }
        if let Some(rest) = reason.strip_prefix("semantic-module JSON member `") {
            let path = rest
                .strip_suffix("` is outside the closed schema")
                .expect("the closed-schema rejection names a path");
            return Some(decoded("DOutside", vec![text(path)]));
        }
        Json::parse(source.as_bytes()).ok()?;
    }
    let module: SemanticModule = serde_json::from_str(source).expect("the parser decoded it");
    Some(decoded(
        "DOk",
        vec![
            text(&module.spec),
            list(module.declarations.iter().map(enc_any).collect()),
        ],
    ))
}

/// A run of a text in the domain, with the constructor of its outcome.
fn run(source: &str) -> Option<(Run, u64)> {
    let outcome = oracle(source)?;
    let constructor = match &outcome {
        Value::Adt { constructor, .. } => *constructor,
        _ => u64::MAX,
    };
    Some((
        (
            vec![text(source)],
            serde_json::to_value(outcome).expect("a value is JSON"),
        ),
        constructor,
    ))
}

// ---------------------------------------------------------------- the texts

/// Every distinct semantic module of every committed project that links, as
/// its canonical JSON.
fn committed_modules() -> Vec<String> {
    let mut out = BTreeSet::new();
    for project in linked_projects() {
        for module in project.modules.values() {
            if let Some(semantic) = &module.semantic {
                out.insert(canonical(semantic));
            }
        }
    }
    out.into_iter().collect()
}

fn module_value(spec: &str, declarations: Vec<Json2>) -> Json2 {
    serde_json::json!({ "spec": spec, "declarations": declarations })
}

fn canonical_text(value: &Json2) -> String {
    canonical(value)
}

fn example_modules(spec: &str) -> Vec<Json2> {
    (0..EXAMPLES)
        .map(|step| {
            module_value(
                spec,
                vec![
                    serde_json::to_value(example_model_declaration(step)).expect("JSON"),
                    serde_json::to_value(example_reasoning_declaration(step)).expect("JSON"),
                ],
            )
        })
        .collect()
}

/// A module of one definition whose body nests `levels` objects deep, so the
/// text nests `levels + 3` deep.
fn nested_module(spec: &str, levels: usize) -> String {
    let mut body = serde_json::json!({ "kind": "var", "name": "x" });
    for _ in 1..levels {
        body = serde_json::json!({ "kind": "not", "value": body });
    }
    let declaration = serde_json::json!({
        "kind": "definition",
        "name": "deep",
        "parameters": [{ "name": "x", "type": { "kind": "bool" } }],
        "result": { "kind": "bool" },
        "body": body,
    });
    // `canonical` parses under serde's own nesting limit, so the deep text is
    // written by hand: the members of each object are already sorted.
    let mut text = String::from("{\"kind\":\"var\",\"name\":\"x\"}");
    for _ in 1..levels {
        text = format!("{{\"kind\":\"not\",\"value\":{text}}}");
    }
    let shell = canonical_text(&module_value(
        spec,
        vec![declaration_without_body(&declaration)],
    ));
    shell.replacen("\"body\":0", &format!("\"body\":{text}"), 1)
}

fn declaration_without_body(declaration: &Json2) -> Json2 {
    let mut out = declaration.clone();
    out["body"] = serde_json::json!(0);
    out
}

/// The JSON pointers of every object of a value, in document order.
fn objects(value: &Json2, pointer: &str, out: &mut Vec<String>) {
    match value {
        Json2::Object(members) => {
            out.push(pointer.to_owned());
            for (key, item) in members {
                objects(item, &format!("{pointer}/{key}"), out);
            }
        }
        Json2::Array(items) => {
            for (index, item) in items.iter().enumerate() {
                objects(item, &format!("{pointer}/{index}"), out);
            }
        }
        _ => {}
    }
}

/// The mutations of one module: each a canonical text of a changed value,
/// and two texts out of canonical form.
fn mutations(base: &Json2) -> Vec<String> {
    let mut pointers = Vec::new();
    objects(base, "", &mut pointers);
    let stride = pointers.len().div_ceil(MUTATED_OBJECTS).max(1);
    let mut out = Vec::new();
    let mut changed = |change: &dyn Fn(&mut serde_json::Map<String, Json2>), pointer: &str| {
        let mut value = base.clone();
        let object = value
            .pointer_mut(pointer)
            .and_then(Json2::as_object_mut)
            .expect("an object");
        change(object);
        out.push(canonical_text(&value));
    };
    for pointer in pointers.iter().step_by(stride) {
        let object = base
            .pointer(pointer)
            .and_then(Json2::as_object)
            .expect("an object");
        for key in object.keys().take(REMOVED_MEMBERS) {
            changed(
                &|members| {
                    members.remove(key);
                },
                pointer,
            );
        }
        for extra in [
            serde_json::json!(0),
            serde_json::json!(false),
            serde_json::json!([]),
        ] {
            changed(
                &|members| {
                    members.insert("zz".to_owned(), extra.clone());
                },
                pointer,
            );
        }
        for (key, flag) in DEFAULTS {
            if !object.contains_key(*key) {
                let written = if *flag {
                    serde_json::json!(false)
                } else {
                    serde_json::json!([])
                };
                changed(
                    &|members| {
                        members.insert((*key).to_owned(), written.clone());
                    },
                    pointer,
                );
            }
        }
        if object.contains_key("kind") {
            changed(
                &|members| {
                    members.insert("kind".to_owned(), serde_json::json!("zz"));
                },
                pointer,
            );
        }
        if let Some((key, _)) = object
            .iter()
            .find(|(key, item)| item.is_string() && key.as_str() != "kind")
        {
            changed(
                &|members| {
                    members.insert(key.clone(), serde_json::json!(7));
                },
                pointer,
            );
        }
        if let Some((key, _)) = object.iter().find(|(_, item)| item.is_u64()) {
            changed(
                &|members| {
                    members.insert(key.clone(), serde_json::json!(-1));
                },
                pointer,
            );
            changed(
                &|members| {
                    members.insert(key.clone(), serde_json::json!("1"));
                },
                pointer,
            );
        }
        if let Some((key, _)) = object.iter().find(|(_, item)| item.is_array()) {
            changed(
                &|members| {
                    members.insert(key.clone(), serde_json::json!({}));
                },
                pointer,
            );
        }
    }
    let text = canonical_text(base);
    out.push(text.replacen(':', ": ", 1));
    out.push(format!(" {text}\n"));
    out.push(text.replacen("\"spec\"", "\"\\u0073pec\"", 1));
    out.push(text.replacen("\"kind\":\"", "\"kind\":\"\\/", 1));
    // Repeated members: serde rejects a repeated field or tag.
    for key in ["spec", "name", "kind"] {
        let member = format!("\"{key}\":\"");
        out.push(text.replacen(&member, &format!("{member}zz\",{member}"), 1));
    }
    out.push(format!(
        "{{\"spec\":{},\"declarations\":{}}}",
        canonical_text(&base["spec"]),
        canonical_text(&base["declarations"])
    ));
    out
}

// ------------------------------------------------------------------- plants

/// Each plant weakens one decoding rule: its site, its replacement.
const PLANTS: &[(&str, &str, &str)] = &[
    (
        "serde's nesting limit is one level deeper",
        "(127 <? (depthOf json))",
        "(128 <? (depthOf json))",
    ),
    (
        "the canonical form is not compared",
        "(if (prim equal (LexLeanStd.Json::writeJson json) text : Bool) then",
        "(if true then",
    ),
    (
        "a unit variant admits other members",
        "{body: (prim equal head (lit \"kind\") : Bool), binders: [], constructor: List.nil}",
        "{body: false, binders: [], constructor: List.nil}",
    ),
    (
        "an empty list is no omitted default",
        "| LexLeanStd.Json::Json.JNil => true",
        "| LexLeanStd.Json::Json.JNil => false",
    ),
    (
        "`false` is no omitted default",
        "(if flag then false else true)",
        "(if flag then false else false)",
    ),
    (
        "an unknown member is admitted",
        "(if (listed allowed key) then",
        "(if true then",
    ),
    (
        "a duplicate member is admitted",
        "(if (listed (project state seen) key) then",
        "(if false then",
    ),
    (
        "a negative count decodes",
        "(if negative then (ctor Option.none [Nat])",
        "(if false then (ctor Option.none [Nat])",
    ),
    (
        "the kind `definition` is misspelled",
        "(prim equal tag (lit \"definition\") : Bool)",
        "(prim equal tag (lit \"definitionx\") : Bool)",
    ),
    (
        "an array index is written in parentheses",
        "(lit \"[\")",
        "(lit \"(\")",
    ),
    (
        "whitespace between tokens is refused",
        "(if (isSpace byte) then",
        "(if false then",
    ),
    (
        "a `\\u` escape is refused",
        "(if (letter =? 117) then (match (unicodeEscape text position)",
        "(if false then (match (unicodeEscape text position)",
    ),
];

fn plants(runs: &[Run]) -> usize {
    let mut missed = Vec::new();
    for (label, site, replacement) in PLANTS {
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/SemanticDecode.lex.tex");
        assert_eq!(
            source.matches(site).count(),
            1,
            "{label}: the plant site occurs once"
        );
        planted.write(
            "src/SemanticDecode.lex.tex",
            &source.replacen(site, replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[0].0);
        match super::evaluate(&low, runs, 1).first() {
            Some(first) => eprintln!(
                "BS-73: plant detected: {label}: {}",
                first.chars().take(120).collect::<String>()
            ),
            None => missed.push((*label).to_owned()),
        }
    }
    assert!(
        missed.is_empty(),
        "planted defects the runs do not detect: {missed:?}"
    );
    PLANTS.len()
}

fn export(lowered: &BTreeMap<&str, super::Lowered>) {
    let exported = harness::exported("bootstrap");
    let bundle = exported.bundle();
    let mut realized = 0usize;
    for exported_root in &exported.output.result.roots {
        let Some((member, low)) = exported_root
            .root
            .strip_prefix(&format!("{MODULE}."))
            .and_then(|member| lowered.get(member).map(|low| (member, low)))
        else {
            continue;
        };
        let largest = exported_root
            .outputs
            .iter()
            .flat_map(|output| &output.certification.certificates)
            .map(|certificate| certificate.file.byte_length)
            .max()
            .expect("a certificate");
        eprintln!(
            "BS-73: {}: largest certificate part {largest} bytes",
            exported_root.root
        );
        assert!(
            largest <= CERTIFICATE_BUDGET,
            "{}: a certificate part of {largest} bytes is beyond {CERTIFICATE_BUDGET}",
            exported_root.root
        );
        for output in &exported_root.outputs {
            let package: Json2 = serde_json::from_slice(
                &std::fs::read(
                    bundle
                        .join(&output.package.directory)
                        .join("package.json")
                        .as_std_path(),
                )
                .expect("package.json"),
            )
            .expect("package JSON");
            assert_eq!(
                package["program"],
                serde_json::to_value(&low.program).expect("a program is JSON"),
                "{member}: the exported package realizes the program the differential ran"
            );
            realized += 1;
        }
    }
    assert_eq!(realized, ROOTS.len(), "one certified package per root");
    let packages = harness::tree(&bundle.join("packages"));
    let committed_packages = harness::tree(&harness::fixture_dir("bootstrap").join("packages"));
    assert_eq!(
        packages.keys().collect::<Vec<_>>(),
        committed_packages.keys().collect::<Vec<_>>(),
        "the exported package files are the committed ones"
    );
    for (relative, content) in &packages {
        assert!(
            committed_packages.get(relative) == Some(content),
            "packages/{relative} differs from the committed package"
        );
    }
}

pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("SemanticDecode"),
        "SemanticDecode is a linked module of the project"
    );
    eligibility_of(&project, "SemanticDecode", MODULE, &ROOTS);

    let committed = committed_modules();
    assert!(
        committed.len() >= 100,
        "the committed projects carry {} distinct semantic modules",
        committed.len()
    );
    let spec: String = serde_json::from_str::<Json2>(&committed[0]).expect("JSON")["spec"]
        .as_str()
        .expect("a discriminator")
        .to_owned();
    let examples = example_modules(&spec);
    let small: Vec<Json2> = committed
        .iter()
        .filter(|module| module.len() <= 2_048)
        .take(20)
        .map(|module| serde_json::from_str(module).expect("JSON"))
        .collect();
    let mut texts: Vec<String> = examples.iter().map(canonical_text).collect();
    texts.push(nested_module(&spec, 124));
    texts.push(nested_module(&spec, 125));
    let mut mutated = Vec::new();
    for base in examples.iter().take(10).chain(&small) {
        mutated.extend(mutations(base));
    }
    let distinct: BTreeSet<String> = mutated.iter().cloned().collect();
    let classified: Vec<(Run, u64)> = texts
        .iter()
        .chain(&distinct)
        .filter_map(|source| run(source))
        .collect();
    let outside = texts.len() + distinct.len() - classified.len();
    let mut by_outcome: BTreeMap<u64, usize> = BTreeMap::new();
    for (_, constructor) in &classified {
        *by_outcome.entry(*constructor).or_default() += 1;
    }
    let runs: Vec<Run> = classified.into_iter().map(|(run, _)| run).collect();
    let corpus: Vec<Run> = committed
        .iter()
        .filter_map(|source| run(source).map(|(run, _)| run))
        .collect();
    assert_eq!(
        corpus.len(),
        committed.len(),
        "every committed module is in the domain"
    );
    eprintln!(
        "BS-73: {} committed modules, {} example and nested modules, {} distinct mutations, {} texts outside the domain, runs by outcome constructor {by_outcome:?} ({:?})",
        committed.len(),
        texts.len(),
        distinct.len(),
        outside,
        started.elapsed()
    );
    for constructor in ["DOk", "DInvalid", "DNotCanonical", "DOutside"] {
        let index = constructors("Decoded")
            .iter()
            .position(|name| name == constructor)
            .expect("a constructor");
        assert!(
            by_outcome
                .get(&u64::try_from(index).expect("an index"))
                .copied()
                .unwrap_or(0)
                >= 2,
            "the runs reach `{constructor}`"
        );
    }

    let checked = support::checked_project(&project);
    let root = ROOTS[0].0;
    let lowered: BTreeMap<&str, super::Lowered> =
        BTreeMap::from([(root, super::lower(&checked, &project, MODULE, root))]);
    let normative = support::normative_host();
    if normative {
        let mut evaluated = runs.clone();
        evaluated.extend(
            committed
                .iter()
                .filter(|source| source.len() <= EVALUATOR_BYTES)
                .filter_map(|source| run(source).map(|(run, _)| run)),
        );
        let disagreements = super::evaluate(&lowered[root], &evaluated, 5);
        assert!(
            disagreements.is_empty(),
            "{root} and the Rust oracle disagree: {disagreements:?}"
        );
        eprintln!(
            "BS-73: {root}: the evaluator agrees on {} runs ({:?})",
            evaluated.len(),
            started.elapsed()
        );
        let planted = plants(&runs);
        eprintln!(
            "BS-73: {planted} planted defects detected ({:?})",
            started.elapsed()
        );
    } else {
        eprintln!("BS-73: platform-bound host: the evaluator and the planted defects run on the normative host");
    }

    let mut compiled = runs;
    compiled.extend(corpus);
    let total = rustc_differential(
        "BS-73",
        &[(root, &lowered[root], compiled.as_slice())],
        "oracle",
    );
    eprintln!("BS-73: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-73") {
        return;
    }
    export(&lowered);
    eprintln!("BS-73: passed in {:?}", started.elapsed());
}
