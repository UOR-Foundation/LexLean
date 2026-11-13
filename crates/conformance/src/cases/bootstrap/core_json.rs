//! `BS-38` (SPEC.md §17.23, §17.21.8, §21.7): the canonical JSON of a native
//! core module as language-1.2 source, `LexLeanBoot.CoreJson`, held to the byte
//! to `lexlean::surface::core::canonical_json`.
//!
//! The root takes the module as first-order data: its schema tag, its imports,
//! its node table (the kernel terms of §17.10 as an inductive of the module), its
//! proof nodes, and each declaration as the standard library's `Json` of the
//! value the oracle serializes it to. It writes the module's canonical JSON:
//! object keys in byte order, strings escaped as `quoteText` (`BS-22`) does, and
//! each declaration by the standard library's writer (`SL-11`).
//!
//! Each planted defect names the one vector that catches it.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use lexlean::artifact::canonical_json::Json;
use lexlean::calculus::Value;
use lexlean::ir::core::{
    CoreBinderInfo, CoreLevel, CoreMetaEntry, CoreMetaValue, CoreModule, CoreNode,
};
use lexlean::surface::core::{canonical_json, parse_module};
use lexlean::surface::module::{locate, CORE};
use serde_json::Value as Json2;

use super::rust_validate::{eligibility_of, rustc_differential};
use super::{bool_val, list, nat_val, text, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.CoreJson";
const SOURCE: &str = "src/CoreJson.lex.tex";
const MODULE_SOURCE: &str = "bootstrap/src/CoreJson.lex.tex";
const JSON_SOURCE: &str = "stdlib/src/Json.lex.tex";
const ROOTS: [(&str, &[&str]); 1] = [("coreJson", &["allocation", "overflow", "recursion"])];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// Committed core modules whose notation is at most this many bytes run under
/// the evaluator (one in `EVALUATOR_EVERY`), and under rustc those at most the
/// second bound.
const EVALUATOR_BYTES: usize = 16_000;
const EVALUATOR_EVERY: usize = 6;
const RUSTC_BYTES: usize = 300_000;
/// Seeded edits of the committed modules under the evaluator and under rustc.
const EVALUATOR_SEEDED: usize = 60;
const RUSTC_SEEDED: usize = 600;

// ---------------------------------------------------------------- the sorts

/// The constructors of each inductive of a module, in declaration order, read
/// from its text, so that an encoder names a constructor and never numbers it.
fn sorts_of(path: &str) -> BTreeMap<String, Vec<String>> {
    let source = std::fs::read_to_string(repo_root().join(path).as_std_path()).expect("a module");
    let mut out = BTreeMap::new();
    for block in source.split("inductive{").skip(1) {
        let block = block.split("\n}\n").next().expect("a block");
        let name = block
            .split("name: ")
            .nth(1)
            .and_then(|rest| rest.split(',').next())
            .expect("a sort name")
            .trim()
            .to_owned();
        let constructors = block
            .split("{name: ")
            .skip(1)
            .map(|rest| {
                rest.split(',')
                    .next()
                    .expect("a constructor")
                    .trim()
                    .to_owned()
            })
            .collect();
        out.insert(name, constructors);
    }
    out
}

fn sorts() -> &'static BTreeMap<String, Vec<String>> {
    static SORTS: OnceLock<BTreeMap<String, Vec<String>>> = OnceLock::new();
    SORTS.get_or_init(|| {
        let mut sorts = sorts_of(MODULE_SOURCE);
        let json = sorts_of(JSON_SOURCE);
        sorts.insert("Json".to_owned(), json["Json"].clone());
        sorts
    })
}

pub(super) fn adt(sort: &str, constructor: &str, fields: Vec<Value>) -> Value {
    let found = sorts()
        .get(sort)
        .unwrap_or_else(|| panic!("no sort `{sort}`"))
        .iter()
        .position(|name| name == constructor)
        .unwrap_or_else(|| panic!("`{sort}` has no constructor `{constructor}`"));
    Value::Adt {
        constructor: u64::try_from(found).expect("an index"),
        fields,
    }
}

fn record(fields: Vec<Value>) -> Value {
    Value::Adt {
        constructor: 0,
        fields,
    }
}

// ------------------------------------------------------------- the encoders

pub(super) fn enc_json(json: &Json) -> Value {
    match json {
        Json::Bool(flag) => adt("Json", "JBool", vec![bool_val(*flag)]),
        Json::Int(number) => adt(
            "Json",
            "JInt",
            vec![
                bool_val(*number < 0),
                Value::Nat {
                    value: number.unsigned_abs().to_string(),
                },
            ],
        ),
        Json::Str(value) => adt("Json", "JStr", vec![text(value)]),
        Json::Arr(items) => adt(
            "Json",
            "JArr",
            vec![items
                .iter()
                .rev()
                .fold(adt("Json", "JNil", Vec::new()), |rest, item| {
                    adt("Json", "JItem", vec![enc_json(item), rest])
                })],
        ),
        Json::Obj(members) => adt(
            "Json",
            "JObj",
            vec![members
                .iter()
                .rev()
                .fold(adt("Json", "JNil", Vec::new()), |rest, (key, value)| {
                    adt("Json", "JMember", vec![text(key), enc_json(value), rest])
                })],
        ),
    }
}

fn level_value(level: &CoreLevel) -> Value {
    match level {
        CoreLevel::Zero => adt("Level", "LZero", Vec::new()),
        CoreLevel::Succ(inner) => adt("Level", "LSucc", vec![level_value(inner)]),
        CoreLevel::Max(pair) => adt(
            "Level",
            "LMax",
            vec![level_value(&pair.0), level_value(&pair.1)],
        ),
        CoreLevel::IMax(pair) => adt(
            "Level",
            "LIMax",
            vec![level_value(&pair.0), level_value(&pair.1)],
        ),
        CoreLevel::Param(name) => adt("Level", "LParam", vec![text(name)]),
    }
}

fn binder_value(info: CoreBinderInfo) -> Value {
    let name = match info {
        CoreBinderInfo::Explicit => "BExplicit",
        CoreBinderInfo::Implicit => "BImplicit",
        CoreBinderInfo::StrictImplicit => "BStrict",
        CoreBinderInfo::Instance => "BInstance",
    };
    adt("Binder", name, Vec::new())
}

fn entry_value(entry: &CoreMetaEntry) -> Value {
    let value = match &entry.v {
        CoreMetaValue::Bool { v } => adt("Meta", "MBool", vec![bool_val(*v)]),
        CoreMetaValue::Nat { v } => adt("Meta", "MNat", vec![text(v)]),
        CoreMetaValue::Int { v } => adt("Meta", "MInt", vec![text(v)]),
        CoreMetaValue::Str { v } => adt("Meta", "MStr", vec![text(v)]),
        CoreMetaValue::Name { v } => adt("Meta", "MName", vec![text(v)]),
    };
    record(vec![text(&entry.n), value])
}

pub(super) fn node_value(node: &CoreNode) -> Value {
    match node {
        CoreNode::BVar { i } => adt("Node", "NBVar", vec![nat_val(*i)]),
        CoreNode::Sort { l } => adt("Node", "NSort", vec![level_value(l)]),
        CoreNode::Const { n, u } => adt(
            "Node",
            "NConst",
            vec![text(n), list(u.iter().map(level_value).collect())],
        ),
        CoreNode::App { f, x } => adt("Node", "NApp", vec![nat_val(*f), nat_val(*x)]),
        CoreNode::Lambda { n, b, t, v } => adt(
            "Node",
            "NLambda",
            vec![text(n), binder_value(*b), nat_val(*t), nat_val(*v)],
        ),
        CoreNode::Forall { n, b, t, v } => adt(
            "Node",
            "NForall",
            vec![text(n), binder_value(*b), nat_val(*t), nat_val(*v)],
        ),
        CoreNode::Let { n, t, v, body, d } => adt(
            "Node",
            "NLet",
            vec![
                text(n),
                nat_val(*t),
                nat_val(*v),
                nat_val(*body),
                bool_val(*d),
            ],
        ),
        CoreNode::Nat { v } => adt("Node", "NNat", vec![text(v)]),
        CoreNode::String { v } => adt(
            "Node",
            "NString",
            vec![list(
                v.iter()
                    .map(|scalar| Value::Nat {
                        value: scalar.to_string(),
                    })
                    .collect(),
            )],
        ),
        CoreNode::Mdata { m, x } => adt(
            "Node",
            "NMdata",
            vec![list(m.iter().map(entry_value).collect()), nat_val(*x)],
        ),
        CoreNode::Projection { n, s, i, x } => adt(
            "Node",
            "NProj",
            vec![text(n), text(s), nat_val(*i), nat_val(*x)],
        ),
    }
}

/// A module as the roots of `CoreJson` and `CorePrint` take it: each
/// declaration as the `Json` of the value the oracle serializes it to.
pub(super) fn module_value(module: &CoreModule) -> Value {
    record(vec![
        text(&module.spec),
        list(module.imports.iter().map(|name| text(name)).collect()),
        list(module.nodes.iter().map(node_value).collect()),
        list(
            module
                .proof_nodes
                .iter()
                .map(|node| nat_val(*node))
                .collect(),
        ),
        list(
            module
                .declarations
                .iter()
                .map(|declaration| {
                    let value = serde_json::to_value(declaration).expect("a declaration");
                    enc_json(&Json::parse(value.to_string().as_bytes()).expect("canonical JSON"))
                })
                .collect(),
        ),
    ])
}

fn run(module: &CoreModule) -> Run {
    (
        vec![module_value(module)],
        serde_json::to_value(text(&canonical_json(module).expect("the canonical JSON")))
            .expect("a value is JSON"),
    )
}

// ------------------------------------------------------------------ corpus

/// The committed core modules written in the notation, read, with the size of
/// their notation.
pub(super) fn committed() -> Vec<(String, CoreModule, usize)> {
    let mut out = Vec::new();
    for (path, content) in super::scan::sources() {
        if content.len() > RUSTC_BYTES {
            continue;
        }
        let Some(found) = locate(&content) else {
            continue;
        };
        if !found.kind.is(&CORE) || found.data.is_some() {
            continue;
        }
        let body = &content[found.body.0..found.body.1];
        if let Ok(module) = parse_module(body) {
            out.push((path, module, body.len()));
        }
    }
    out
}

/// The smallest committed module: the declarations the vectors reuse.
fn smallest(modules: &[(String, CoreModule, usize)]) -> CoreModule {
    modules
        .iter()
        .min_by_key(|(_, _, size)| *size)
        .map(|(_, module, _)| module.clone())
        .expect("a committed core module")
}

fn level(text: &str) -> CoreLevel {
    serde_json::from_str(text).unwrap_or_else(|error| panic!("{text}: {error}"))
}

fn node(text: &str) -> CoreNode {
    serde_json::from_str(text).unwrap_or_else(|error| panic!("{text}: {error}"))
}

/// Named vectors: the smallest committed module with its nodes, imports, proof
/// nodes, or spec replaced so that every form of the data is written.
pub(super) fn named(base: &CoreModule) -> Vec<(String, CoreModule)> {
    let mut out = Vec::new();
    let mut add = |label: &str, change: &dyn Fn(&mut CoreModule)| {
        let mut module = base.clone();
        change(&mut module);
        out.push((label.to_owned(), module));
    };
    add("the smallest committed module", &|_| {});
    add("an empty module", &|module| {
        module.imports.clear();
        module.nodes.clear();
        module.proof_nodes.clear();
        module.declarations.clear();
    });
    add("another spec", &|module| {
        module.spec = "lexlean/core-module/1".to_owned();
    });
    add("imports to escape", &|module| {
        module.imports = vec![
            "A".to_owned(),
            "a \"b\" \\ c".to_owned(),
            "line\nbreak\u{1}".to_owned(),
            "é".to_owned(),
        ];
    });
    add("every node kind", &|module| {
        module.nodes = vec![
            node(r#"{"k":"b","i":3}"#),
            node(r#"{"k":"s","l":{"k":"z"}}"#),
            node(r#"{"k":"c","n":"Nat.succ","u":[]}"#),
            node(r#"{"k":"a","f":2,"x":0}"#),
            node(r#"{"k":"l","n":"x","b":"e","t":1,"v":0}"#),
            node(r#"{"k":"p","n":"","b":"i","t":1,"v":1}"#),
            node(r#"{"k":"e","n":"y","t":1,"v":2,"body":3,"d":true}"#),
            node(r#"{"k":"e","n":"y","t":1,"v":2,"body":3,"d":false}"#),
            node(r#"{"k":"n","v":"12345678901234567890"}"#),
            node(r#"{"k":"t","v":[104,105,34,92,10,0,233,128512]}"#),
            node(
                r#"{"k":"d","m":[{"n":"b","v":{"t":"b","v":true}},{"n":"c","v":{"t":"b","v":false}},{"n":"n","v":{"t":"n","v":"7"}},{"n":"i","v":{"t":"i","v":"-7"}},{"n":"s","v":{"t":"s","v":"a\"b"}},{"n":"m","v":{"t":"m","v":"Nat.add"}}],"x":3}"#,
            ),
            node(r#"{"k":"j","n":"Prod.fst","s":"Prod","i":0,"x":3}"#),
        ];
    });
    add("every binder", &|module| {
        module.nodes = vec![
            node(r#"{"k":"s","l":{"k":"z"}}"#),
            node(r#"{"k":"l","n":"a","b":"e","t":0,"v":0}"#),
            node(r#"{"k":"l","n":"b","b":"i","t":0,"v":0}"#),
            node(r#"{"k":"p","n":"c","b":"s","t":0,"v":0}"#),
            node(r#"{"k":"p","n":"d","b":"c","t":0,"v":0}"#),
        ];
    });
    add("every level", &|module| {
        module.nodes = vec![
            CoreNode::Sort {
                l: level(r#"{"k":"s","a":{"k":"s","a":{"k":"p","a":"u"}}}"#),
            },
            CoreNode::Sort {
                l: level(
                    r#"{"k":"m","a":[{"k":"z"},{"k":"i","a":[{"k":"p","a":"v"},{"k":"s","a":{"k":"z"}}]}]}"#,
                ),
            },
            CoreNode::Const {
                n: "List.nil".to_owned(),
                u: vec![
                    level(r#"{"k":"p","a":"u"}"#),
                    level(r#"{"k":"z"}"#),
                    level(r#"{"k":"p","a":"a\"b"}"#),
                ],
            },
        ];
    });
    add("names to escape", &|module| {
        module.nodes = vec![
            node(r#"{"k":"c","n":"a \"quoted\" name\t","u":[]}"#),
            node(r#"{"k":"l","n":"é\\","b":"e","t":0,"v":0}"#),
            node(r#"{"k":"j","n":"x\ny","s":"S\"","i":12,"x":0}"#),
            node(r#"{"k":"n","v":"007"}"#),
        ];
    });
    add("proof nodes", &|module| {
        module.proof_nodes = vec![0, 2, 10, 1234];
    });
    add("one proof node", &|module| {
        module.proof_nodes = vec![5];
    });
    add("an empty string node", &|module| {
        module.nodes = vec![
            node(r#"{"k":"t","v":[]}"#),
            node(r#"{"k":"d","m":[],"x":0}"#),
        ];
    });
    add("one declaration", &|module| {
        module.declarations.truncate(1);
    });
    out
}

const PIECES: &[&str] = &["x", "", "a\"b", "é", "Nat", "\\", "\n", "0"];

fn edit(seeded: &mut Seeded, base: &CoreModule) -> CoreModule {
    let mut module = base.clone();
    for _ in 0..=seeded.below(3) {
        match seeded.below(5) {
            0 if !module.nodes.is_empty() => {
                let at = seeded.below(module.nodes.len());
                module.nodes.remove(at);
            }
            1 if !module.nodes.is_empty() => {
                let at = seeded.below(module.nodes.len());
                if let CoreNode::Const { n, .. }
                | CoreNode::Lambda { n, .. }
                | CoreNode::Forall { n, .. } = &mut module.nodes[at]
                {
                    *n = (*seeded.pick(PIECES)).to_owned();
                }
            }
            2 if !module.declarations.is_empty() => {
                let at = seeded.below(module.declarations.len());
                module.declarations[at].name = (*seeded.pick(PIECES)).to_owned();
            }
            3 => module.imports.push((*seeded.pick(PIECES)).to_owned()),
            _ => module.proof_nodes.push(seeded.below(100_000)),
        }
    }
    module
}

fn seeded_runs(count: usize, bases: &[CoreModule], seed: u64) -> Vec<Run> {
    let mut seeded = Seeded(seed);
    (0..count)
        .map(|_| {
            let base = seeded.pick(bases).clone();
            run(&edit(&mut seeded, &base))
        })
        .collect()
}

// ------------------------------------------------------------------- plants

struct Plant {
    label: &'static str,
    site: &'static str,
    replacement: &'static str,
    vector: &'static str,
}

const PLANTS: &[Plant] = &[
    Plant {
        label: "the instance binder is written implicit",
        site: "| Binder.BInstance => (lit \"c\")",
        replacement: "| Binder.BInstance => (lit \"i\")",
        vector: "every binder",
    },
    Plant {
        label: "the strict binder is written implicit",
        site: "| Binder.BStrict => (lit \"s\")",
        replacement: "| Binder.BStrict => (lit \"i\")",
        vector: "every binder",
    },
    Plant {
        label: "a lambda is written as a forall",
        site: "(binderJson (lit \"l\") label info domain body)",
        replacement: "(binderJson (lit \"p\") label info domain body)",
        vector: "every binder",
    },
    Plant {
        label: "a successor level is tagged zero",
        site: "(j \",'k':'s'}\")",
        replacement: "(j \",'k':'z'}\")",
        vector: "every level",
    },
    Plant {
        label: "an impredicative maximum is tagged maximum",
        site: "(j \"],'k':'i'}\")",
        replacement: "(j \"],'k':'m'}\")",
        vector: "every level",
    },
    Plant {
        label: "a level parameter is not escaped",
        site: "(j \"{'a':\") (quoted label)",
        replacement: "(j \"{'a':\") label",
        vector: "every level",
    },
    Plant {
        label: "levels are separated by a comma and a space",
        site: "(cons (levelJson level) out)) (nil Bytes) levels : (List Bytes))) (lit \",\"))",
        replacement:
            "(cons (levelJson level) out)) (nil Bytes) levels : (List Bytes))) (lit \", \"))",
        vector: "every level",
    },
    Plant {
        label: "a constant's levels are dropped",
        site: "(j \",'u':[\")) (levelsJson levels))",
        replacement: "(j \",'u':[\")) (lit \"\"))",
        vector: "every level",
    },
    Plant {
        label: "let and have are swapped",
        site: "(if dependent then (lit \"true\") else (lit \"false\"))",
        replacement: "(if dependent then (lit \"false\") else (lit \"true\"))",
        vector: "every node kind",
    },
    Plant {
        label: "a let's value is its body",
        site: "(cat (j \",'v':\") (num bound))",
        replacement: "(cat (j \",'v':\") (num rest))",
        vector: "every node kind",
    },
    Plant {
        label: "string scalars are separated by a semicolon",
        site: "(cons (num scalar) out)) (nil Bytes) scalars : (List Bytes))) (lit \",\"))",
        replacement: "(cons (num scalar) out)) (nil Bytes) scalars : (List Bytes))) (lit \";\"))",
        vector: "every node kind",
    },
    Plant {
        label: "an integer annotation is tagged natural",
        site: "(j \"{'t':'i','v':\")",
        replacement: "(j \"{'t':'n','v':\")",
        vector: "every node kind",
    },
    Plant {
        label: "a false annotation is true",
        site: "(if flag then (lit \"true\") else (lit \"false\"))",
        replacement: "(lit \"true\")",
        vector: "every node kind",
    },
    Plant {
        label: "a projection's index is its operand",
        site: "(j \"{'i':\") (num position)) (j \",'k':'j','n':\")",
        replacement: "(j \"{'i':\") (num inner)) (j \",'k':'j','n':\")",
        vector: "every node kind",
    },
    Plant {
        label: "a bound variable is tagged v",
        site: "(j \",'k':'b'}\")",
        replacement: "(j \",'k':'v'}\")",
        vector: "every node kind",
    },
    Plant {
        label: "a natural literal is not quoted",
        site: "(j \"{'k':'n','v':\") (quoted text)",
        replacement: "(j \"{'k':'n','v':\") text",
        vector: "names to escape",
    },
    Plant {
        label: "proof nodes are separated by a comma and a space",
        site: "(fun (index : Nat) => (num index))) (lit \",\"))",
        replacement: "(fun (index : Nat) => (num index))) (lit \", \"))",
        vector: "proof nodes",
    },
    Plant {
        label: "imports are not quoted",
        site: "(fun (name : Bytes) => (quoted name))",
        replacement: "(fun (name : Bytes) => name)",
        vector: "imports to escape",
    },
    Plant {
        label: "the spec is not quoted",
        site: "(cons (quoted (project module spec))",
        replacement: "(cons (project module spec)",
        vector: "the smallest committed module",
    },
    Plant {
        label: "a control character is not escaped in a name",
        site: "(SurfaceLex::quoteText text)",
        replacement: "(cat (cat 0x22 text) 0x22)",
        vector: "names to escape",
    },
];

fn plants(vectors: &[(String, CoreModule)]) {
    let mut missed = Vec::new();
    for plant in PLANTS {
        let module = &vectors
            .iter()
            .find(|(label, _)| label == plant.vector)
            .unwrap_or_else(|| panic!("{}: no vector `{}`", plant.label, plant.vector))
            .1;
        let planted = support::P::bootstrap();
        let source = planted.read_raw(SOURCE);
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(SOURCE, &source.replacen(plant.site, plant.replacement, 1));
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[0].0);
        match super::evaluate(&low, &[run(module)], 1).first() {
            Some(first) => eprintln!(
                "BS-38: plant detected: {}: {}",
                plant.label,
                first.chars().take(160).collect::<String>()
            ),
            None => missed.push(plant.label),
        }
    }
    assert!(
        missed.is_empty(),
        "planted defects their own vector does not detect: {missed:?}"
    );
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
            "BS-38: {}: largest certificate part {largest} bytes",
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

#[allow(clippy::too_many_lines)]
pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("CoreJson"),
        "CoreJson is a linked module of the project"
    );
    eligibility_of(&project, "CoreJson", MODULE, &ROOTS);

    let modules = committed();
    assert!(
        modules.len() >= 100,
        "the committed core modules in the notation are {}",
        modules.len()
    );
    let vectors = named(&smallest(&modules));
    let labels: BTreeSet<&str> = vectors.iter().map(|(label, _)| label.as_str()).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let named_runs: Vec<Run> = vectors.iter().map(|(_, module)| run(module)).collect();

    let mut evaluator: Vec<Run> = Vec::new();
    let mut compiled: Vec<Run> = Vec::new();
    let mut bases: Vec<CoreModule> = Vec::new();
    for (index, (_, module, size)) in modules.iter().enumerate() {
        if *size <= EVALUATOR_BYTES && index % EVALUATOR_EVERY == 0 {
            evaluator.push(run(module));
            bases.push(module.clone());
        }
        compiled.push(run(module));
    }
    let normative = support::normative_host();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (
            seeded_runs(EVALUATOR_SEEDED, &bases, 0x0B55_0038),
            seeded_runs(RUSTC_SEEDED, &bases, 0x0B55_1038),
        )
    } else {
        (Vec::new(), Vec::new())
    };
    eprintln!(
        "BS-38: {} vectors, {} committed core modules ({} under the evaluator, {} under rustc), {} and {} seeded edits ({:?})",
        vectors.len(),
        modules.len(),
        evaluator.len(),
        compiled.len(),
        evaluator_seeded.len(),
        rustc_seeded.len(),
        started.elapsed()
    );

    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, super::Lowered> = ROOTS
        .iter()
        .map(|(root, _)| (*root, super::lower(&checked, &project, MODULE, root)))
        .collect();
    if normative {
        let mut corpus = named_runs.clone();
        corpus.extend(evaluator);
        corpus.extend(evaluator_seeded);
        let disagreements = super::evaluate(&lowered[ROOTS[0].0], &corpus, 5);
        assert!(
            disagreements.is_empty(),
            "coreJson and the Rust oracle disagree: {disagreements:?}"
        );
        eprintln!(
            "BS-38: coreJson: the evaluator agrees on {} runs ({:?})",
            corpus.len(),
            started.elapsed()
        );
        plants(&vectors);
    } else {
        eprintln!("BS-38: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut all = named_runs;
    all.extend(compiled);
    all.extend(rustc_seeded);
    let total = rustc_differential(
        "BS-38",
        &[(ROOTS[0].0, &lowered[ROOTS[0].0], all.as_slice())],
        "oracle",
    );
    eprintln!("BS-38: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-38") {
        return;
    }
    export(&lowered);
    eprintln!("BS-38: passed in {:?}", started.elapsed());
}
