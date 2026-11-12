//! `BS-70` (SPEC.md §17.23, §17.11, §17.12): the ordinary declarations of the
//! semantic module language and the module that holds them as language-1.2
//! source, `LexLeanBoot.SemanticDecls`, held to the byte to the serde form of
//! `lexlean::ir::semantic::SemanticDeclaration` and `SemanticModule` in
//! canonical JSON, as the linked IR embeds them.
//!
//! `declarationText` writes a structure, class, instance, inductive,
//! definition, or theorem, each optional member left out exactly where the
//! oracle's serde attributes skip it; `moduleText` writes a module's
//! specification and declarations. Every ordinary declaration of every
//! committed semantic module that links is a run, every such module whose
//! declarations are all ordinary is a run, and so are hand-built and seeded
//! declarations.
//!
//! Each planted defect names the root and the one vector that catch it.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use lexlean::calculus::Value;
use lexlean::ir::semantic::{
    MemberRef, SemanticAssignment, SemanticConstructor, SemanticDeclaration, SemanticField,
    SemanticModule, SemanticParameter, SemanticProduction, SemanticProof, SemanticTerm,
    SemanticTermination, SemanticType,
};
use serde_json::Value as Json2;

use super::ir_document::linked_projects;
use super::rust_validate::{eligibility_of, rustc_differential};
use super::semantic_terms::{assign_chain, enc_proof, enc_term, members};
use super::semantic_types::{canonical, enc_type, type_chain};
use super::{list, option, pair, text, texts, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.SemanticDecls";
const SOURCE: &str = "bootstrap/src/SemanticDecls.lex.tex";
const ROOTS: [(&str, &[&str]); 2] = [
    (
        "semanticDeclarationText",
        &["allocation", "overflow", "recursion"],
    ),
    (
        "semanticModuleText",
        &["allocation", "overflow", "recursion"],
    ),
];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// Seeded declarations under the evaluator, and under rustc.
const EVALUATOR_SEEDED: usize = 100;
const RUSTC_SEEDED: usize = 500;
/// A committed module whose JSON is longer runs under rustc only.
const EVALUATOR_BYTES: usize = 262_144;

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
            // The constructors after `constructors:`, each named by its
            // `name:` whether the formatter keeps it on one line or wraps it.
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

fn adt(constructor: &str, fields: Vec<Value>) -> Value {
    let found = constructors("SDecl")
        .iter()
        .position(|name| name == constructor)
        .unwrap_or_else(|| panic!("`SDecl` has no constructor `{constructor}`"));
    Value::Adt {
        constructor: u64::try_from(found).expect("an index"),
        fields,
    }
}

// ------------------------------------------------------------- the encoders

fn nat(value: u64) -> Value {
    Value::Nat {
        value: value.to_string(),
    }
}

fn typed(name: &str, ty: &SemanticType) -> Value {
    pair(text(name), enc_type(ty))
}

fn params(parameters: &[SemanticParameter]) -> Value {
    list(
        parameters
            .iter()
            .map(|parameter| typed(&parameter.name, &parameter.r#type))
            .collect(),
    )
}

fn fields(fields: &[SemanticField]) -> Value {
    list(
        fields
            .iter()
            .map(|field| typed(&field.name, &field.r#type))
            .collect(),
    )
}

fn constructors_of(constructors: &[SemanticConstructor]) -> Value {
    list(
        constructors
            .iter()
            .map(|constructor| pair(text(&constructor.name), type_chain(&constructor.fields)))
            .collect(),
    )
}

fn text_option(value: Option<&String>) -> Value {
    option(value.map(|value| text(value)))
}

fn is_ordinary(declaration: &SemanticDeclaration) -> bool {
    matches!(
        declaration,
        SemanticDeclaration::Structure { .. }
            | SemanticDeclaration::Class { .. }
            | SemanticDeclaration::Instance { .. }
            | SemanticDeclaration::Inductive { .. }
            | SemanticDeclaration::Definition { .. }
            | SemanticDeclaration::Theorem { .. }
    )
}

fn enc_declaration(declaration: &SemanticDeclaration) -> Value {
    match declaration {
        SemanticDeclaration::Structure {
            name,
            type_parameters,
            parameters,
            fields: declared,
        } => adt(
            "DStructure",
            vec![
                text(name),
                texts(type_parameters),
                params(parameters),
                fields(declared),
            ],
        ),
        SemanticDeclaration::Class {
            name,
            type_parameters,
            parameters,
            fields: declared,
        } => adt(
            "DClass",
            vec![
                text(name),
                texts(type_parameters),
                params(parameters),
                fields(declared),
            ],
        ),
        SemanticDeclaration::Instance {
            name,
            class,
            arguments,
            priority,
            fields: assigned,
        } => adt(
            "DInstance",
            vec![
                text(name),
                option(class.module.as_deref().map(text)),
                text(&class.name),
                type_chain(arguments),
                nat(*priority),
                assign_chain(assigned),
            ],
        ),
        SemanticDeclaration::Inductive {
            name,
            type_parameters,
            parameters,
            constructors,
            mutual,
        } => adt(
            "DInductive",
            vec![
                text(name),
                texts(type_parameters),
                params(parameters),
                constructors_of(constructors),
                text_option(mutual.as_ref()),
            ],
        ),
        SemanticDeclaration::Definition {
            name,
            type_parameters,
            parameters,
            result,
            recursive_argument,
            body,
            axioms,
            executable,
            mutual,
            termination,
            production,
        } => adt(
            "DDefinition",
            vec![
                text(name),
                texts(type_parameters),
                params(parameters),
                enc_type(result),
                text_option(recursive_argument.as_ref()),
                enc_term(body),
                texts(axioms),
                Value::Bool { value: *executable },
                text_option(mutual.as_ref()),
                option(termination.as_ref().map(|evidence| {
                    pair(enc_term(&evidence.measure), members(&evidence.evidence))
                })),
                option(
                    production
                        .as_ref()
                        .map(|declared| pair(texts(&declared.targets), texts(&declared.effects))),
                ),
            ],
        ),
        SemanticDeclaration::Theorem {
            name,
            type_parameters,
            parameters,
            statement,
            proof,
            axioms,
        } => adt(
            "DTheorem",
            vec![
                text(name),
                texts(type_parameters),
                params(parameters),
                enc_term(statement),
                enc_proof(proof),
                texts(axioms),
            ],
        ),
        other => panic!("not an ordinary declaration: {other:?}"),
    }
}

#[derive(Clone)]
enum Input {
    Declaration(Box<SemanticDeclaration>),
    Module(Box<SemanticModule>),
}

impl Input {
    const fn root(&self) -> usize {
        match self {
            Self::Declaration(_) => 0,
            Self::Module(_) => 1,
        }
    }

    fn run(&self) -> Run {
        match self {
            Self::Declaration(declaration) => (
                vec![enc_declaration(declaration)],
                serde_json::to_value(text(&canonical(&**declaration))).expect("a value is JSON"),
            ),
            Self::Module(module) => (
                vec![
                    text(&module.spec),
                    list(module.declarations.iter().map(enc_declaration).collect()),
                ],
                serde_json::to_value(text(&canonical(&**module))).expect("a value is JSON"),
            ),
        }
    }
}

// ---------------------------------------------------------------- the corpus

/// Every distinct ordinary declaration of every committed semantic module
/// that links, and every such module whose declarations are all ordinary,
/// with how many modules carry another kind.
fn committed() -> (Vec<Input>, usize) {
    let mut declarations = BTreeMap::new();
    let mut modules = BTreeMap::new();
    let mut other = 0usize;
    for project in linked_projects() {
        for module in project.modules.values() {
            let Some(semantic) = &module.semantic else {
                continue;
            };
            for declaration in semantic
                .declarations
                .iter()
                .filter(|item| is_ordinary(item))
            {
                declarations.insert(
                    canonical(declaration),
                    Input::Declaration(Box::new(declaration.clone())),
                );
            }
            if semantic.declarations.iter().all(is_ordinary) {
                modules.insert(
                    canonical(semantic),
                    Input::Module(Box::new(semantic.clone())),
                );
            } else {
                other += 1;
            }
        }
    }
    let mut out: Vec<Input> = declarations.into_values().collect();
    out.extend(modules.into_values());
    (out, other)
}

// ------------------------------------------------------------------ vectors

fn parameter(name: &str, ty: SemanticType) -> SemanticParameter {
    SemanticParameter {
        name: name.to_owned(),
        r#type: ty,
    }
}

fn var(name: &str) -> SemanticTerm {
    SemanticTerm::Var {
        name: name.to_owned(),
    }
}

fn reference(module: Option<&str>, name: &str) -> MemberRef {
    MemberRef {
        module: module.map(str::to_owned),
        name: name.to_owned(),
    }
}

fn names(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| (*item).to_owned()).collect()
}

fn structure() -> SemanticDeclaration {
    SemanticDeclaration::Structure {
        name: "Point".to_owned(),
        type_parameters: Vec::new(),
        parameters: Vec::new(),
        fields: vec![
            SemanticField {
                name: "x".to_owned(),
                r#type: SemanticType::Nat,
            },
            SemanticField {
                name: "y".to_owned(),
                r#type: SemanticType::Nat,
            },
        ],
    }
}

fn definition_plain() -> SemanticDeclaration {
    SemanticDeclaration::Definition {
        name: "zero".to_owned(),
        type_parameters: Vec::new(),
        parameters: Vec::new(),
        result: SemanticType::Nat,
        recursive_argument: None,
        body: SemanticTerm::Nat {
            value: "0".to_owned(),
        },
        axioms: Vec::new(),
        executable: false,
        mutual: None,
        termination: None,
        production: None,
    }
}

struct Vector {
    label: &'static str,
    input: Input,
}

#[allow(clippy::too_many_lines)]
fn named() -> Vec<Vector> {
    let mut out = Vec::new();
    let mut add = |label: &'static str, declaration: SemanticDeclaration| {
        out.push(Vector {
            label,
            input: Input::Declaration(Box::new(declaration)),
        });
    };
    add("a structure", structure());
    add(
        "a class",
        SemanticDeclaration::Class {
            name: "Show".to_owned(),
            type_parameters: names(&["a"]),
            parameters: vec![parameter("bound", SemanticType::Nat)],
            fields: vec![SemanticField {
                name: "show".to_owned(),
                r#type: SemanticType::Function {
                    parameters: vec![SemanticType::Parameter {
                        name: "a".to_owned(),
                    }],
                    result: Box::new(SemanticType::String),
                },
            }],
        },
    );
    add(
        "an instance",
        SemanticDeclaration::Instance {
            name: "showNat".to_owned(),
            class: reference(Some("Lib"), "Show"),
            arguments: vec![SemanticType::Nat],
            priority: 100,
            fields: vec![SemanticAssignment {
                field: "show".to_owned(),
                value: var("format"),
            }],
        },
    );
    add(
        "an inductive in a mutual group",
        SemanticDeclaration::Inductive {
            name: "Tree".to_owned(),
            type_parameters: Vec::new(),
            parameters: Vec::new(),
            constructors: vec![
                SemanticConstructor {
                    name: "leaf".to_owned(),
                    fields: Vec::new(),
                },
                SemanticConstructor {
                    name: "node".to_owned(),
                    fields: vec![SemanticType::Nat, SemanticType::Bool],
                },
            ],
            mutual: Some("trees".to_owned()),
        },
    );
    add(
        "a definition with everything",
        SemanticDeclaration::Definition {
            name: "walk".to_owned(),
            type_parameters: names(&["a"]),
            parameters: vec![parameter("n", SemanticType::Nat)],
            result: SemanticType::Nat,
            recursive_argument: Some("n".to_owned()),
            body: var("n"),
            axioms: names(&["propext"]),
            executable: true,
            mutual: Some("walks".to_owned()),
            termination: Some(SemanticTermination {
                measure: var("n"),
                evidence: vec![reference(None, "walk_decreases")],
            }),
            production: Some(SemanticProduction {
                targets: names(&["rust-std"]),
                effects: names(&["allocation", "recursion"]),
            }),
        },
    );
    add("a definition with nothing optional", definition_plain());
    add(
        "a theorem with everything",
        SemanticDeclaration::Theorem {
            name: "walk_zero".to_owned(),
            type_parameters: names(&["a"]),
            parameters: vec![parameter("n", SemanticType::Nat)],
            statement: SemanticTerm::Bool { value: true },
            proof: SemanticProof::Decide,
            axioms: names(&["propext", "Quot.sound"]),
        },
    );
    add(
        "a theorem with nothing optional",
        SemanticDeclaration::Theorem {
            name: "trivial".to_owned(),
            type_parameters: Vec::new(),
            parameters: Vec::new(),
            statement: SemanticTerm::Bool { value: true },
            proof: SemanticProof::Reflexivity,
            axioms: Vec::new(),
        },
    );
    let module: SemanticModule = serde_json::from_value(serde_json::json!({
        "spec": "lexlean/semantic-module/2",
        "declarations": [
            serde_json::to_value(structure()).expect("JSON"),
            serde_json::to_value(definition_plain()).expect("JSON"),
        ],
    }))
    .expect("a module");
    out.push(Vector {
        label: "a module",
        input: Input::Module(Box::new(module)),
    });
    out
}

// ------------------------------------------------------------------- seeded

const NAMES: &[&str] = &["x", "acc", "\"q\"", "\u{e9}", ""];

fn drawn_name(seeded: &mut Seeded) -> String {
    (*seeded.pick(NAMES)).to_owned()
}

fn drawn_names(seeded: &mut Seeded) -> Vec<String> {
    (0..seeded.below(3)).map(|_| drawn_name(seeded)).collect()
}

fn drawn_type(seeded: &mut Seeded) -> SemanticType {
    seeded
        .pick(&[SemanticType::Nat, SemanticType::Bool, SemanticType::Bytes])
        .clone()
}

fn drawn_params(seeded: &mut Seeded) -> Vec<SemanticParameter> {
    (0..seeded.below(3))
        .map(|_| parameter(&drawn_name(seeded), drawn_type(seeded)))
        .collect()
}

fn drawn_option(seeded: &mut Seeded) -> Option<String> {
    seeded.chance(50).then(|| drawn_name(seeded))
}

fn drawn_declaration(seeded: &mut Seeded) -> SemanticDeclaration {
    match seeded.below(6) {
        0 => SemanticDeclaration::Structure {
            name: drawn_name(seeded),
            type_parameters: drawn_names(seeded),
            parameters: drawn_params(seeded),
            fields: (0..seeded.below(3))
                .map(|_| SemanticField {
                    name: drawn_name(seeded),
                    r#type: drawn_type(seeded),
                })
                .collect(),
        },
        1 => SemanticDeclaration::Class {
            name: drawn_name(seeded),
            type_parameters: drawn_names(seeded),
            parameters: drawn_params(seeded),
            fields: Vec::new(),
        },
        2 => SemanticDeclaration::Instance {
            name: drawn_name(seeded),
            class: reference(None, "C"),
            arguments: (0..seeded.below(3)).map(|_| drawn_type(seeded)).collect(),
            // The canonical JSON domain ends at `i64::MAX`.
            priority: *seeded.pick(&[0, 1, 100, 9_223_372_036_854_775_807]),
            fields: (0..seeded.below(3))
                .map(|_| SemanticAssignment {
                    field: drawn_name(seeded),
                    value: var(&drawn_name(seeded)),
                })
                .collect(),
        },
        3 => SemanticDeclaration::Inductive {
            name: drawn_name(seeded),
            type_parameters: drawn_names(seeded),
            parameters: drawn_params(seeded),
            constructors: (0..seeded.below(3))
                .map(|_| SemanticConstructor {
                    name: drawn_name(seeded),
                    fields: (0..seeded.below(3)).map(|_| drawn_type(seeded)).collect(),
                })
                .collect(),
            mutual: drawn_option(seeded),
        },
        4 => SemanticDeclaration::Definition {
            name: drawn_name(seeded),
            type_parameters: drawn_names(seeded),
            parameters: drawn_params(seeded),
            result: drawn_type(seeded),
            recursive_argument: drawn_option(seeded),
            body: var(&drawn_name(seeded)),
            axioms: drawn_names(seeded),
            executable: seeded.chance(50),
            mutual: drawn_option(seeded),
            termination: seeded.chance(40).then(|| SemanticTermination {
                measure: var("n"),
                evidence: (0..seeded.below(3))
                    .map(|_| reference(drawn_option(seeded).as_deref(), "e"))
                    .collect(),
            }),
            production: seeded.chance(40).then(|| SemanticProduction {
                targets: drawn_names(seeded),
                effects: drawn_names(seeded),
            }),
        },
        _ => SemanticDeclaration::Theorem {
            name: drawn_name(seeded),
            type_parameters: drawn_names(seeded),
            parameters: drawn_params(seeded),
            statement: var(&drawn_name(seeded)),
            proof: SemanticProof::Decide,
            axioms: drawn_names(seeded),
        },
    }
}

fn seeded_inputs(count: usize, seed: u64) -> Vec<Input> {
    let mut seeded = Seeded(seed);
    (0..count)
        .map(|_| Input::Declaration(Box::new(drawn_declaration(&mut seeded))))
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
        label: "a structure is a struct",
        site: "(kinded \"structure\"",
        replacement: "(kinded \"struct\"",
        vector: "a structure",
    },
    Plant {
        label: "a class is a structure",
        site: "(kinded \"class\"",
        replacement: "(kinded \"structure\"",
        vector: "a class",
    },
    Plant {
        label: "a priority is negative",
        site: "(ctor LexLeanStd.Json::Json.JInt false priority)",
        replacement: "(ctor LexLeanStd.Json::Json.JInt true priority)",
        vector: "an instance",
    },
    Plant {
        label: "an inductive loses its mutual group",
        site: "(member \"constructors\" (constructorsJson constructors) (textWhenSome \"mutual\" grouped (jnil)))",
        replacement: "(member \"constructors\" (constructorsJson constructors) (jnil))",
        vector: "an inductive in a mutual group",
    },
    Plant {
        label: "a constructor's fields are its arguments",
        site: "(member \"fields\" (ctor LexLeanStd.Json::Json.JArr (SemanticTypes::typeJson (second entry)))",
        replacement: "(member \"arguments\" (ctor LexLeanStd.Json::Json.JArr (SemanticTypes::typeJson (second entry)))",
        vector: "an inductive in a mutual group",
    },
    Plant {
        label: "empty type parameters of a definition are written",
        site: "    | List.nil => rest\n",
        replacement: "    | List.nil => (member key (SemanticTerms::namesJson names) rest)\n",
        vector: "a definition with nothing optional",
    },
    Plant {
        label: "an executable definition is written as not executable",
        site: "then (member \"executable\" (ctor LexLeanStd.Json::Json.JBool true)",
        replacement: "then (member \"executable\" (ctor LexLeanStd.Json::Json.JBool false)",
        vector: "a definition with everything",
    },
    Plant {
        label: "the recursive argument is the recursion",
        site: "(textWhenSome \"recursive_argument\" recursiveArg",
        replacement: "(textWhenSome \"recursion\" recursiveArg",
        vector: "a definition with everything",
    },
    Plant {
        label: "a termination loses its evidence",
        site: "(member \"evidence\" (SemanticTerms::membersJson (second evidenced))",
        replacement: "(member \"evidence\" (SemanticTerms::membersJson (nil ((Option Bytes) * Bytes)))",
        vector: "a definition with everything",
    },
    Plant {
        label: "a production's effects are its targets",
        site: "(member \"effects\" (SemanticTerms::namesJson (second declared))",
        replacement: "(member \"effects\" (SemanticTerms::namesJson (first declared))",
        vector: "a definition with everything",
    },
    Plant {
        label: "a theorem loses its axioms",
        site: "(namesWhenAny \"axioms\" axiomNames (jnil))",
        replacement: "(jnil)",
        vector: "a theorem with everything",
    },
    Plant {
        label: "a module names its specification otherwise",
        site: "(member \"spec\" (jtext spec)",
        replacement: "(member \"specification\" (jtext spec)",
        vector: "a module",
    },
    Plant {
        label: "a module's declarations are written last first",
        site: "(LexLeanStd.Order::reverseList [SDecl] declarations)",
        replacement: "declarations",
        vector: "a module",
    },
];

fn plants(vectors: &[Vector]) {
    let mut missed = Vec::new();
    for plant in PLANTS {
        let vector = vectors
            .iter()
            .find(|vector| vector.label == plant.vector)
            .unwrap_or_else(|| panic!("{}: no vector `{}`", plant.label, plant.vector));
        let root = ROOTS[vector.input.root()].0;
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/SemanticDecls.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/SemanticDecls.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, root);
        match super::evaluate(&low, &[vector.input.run()], 1).first() {
            Some(first) => eprintln!(
                "BS-70: plant detected: {}: {}",
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
            "BS-70: {}: largest certificate part {largest} bytes",
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

fn split(inputs: &[Input], evaluator: bool) -> [Vec<Run>; 2] {
    let mut out = [Vec::new(), Vec::new()];
    for input in inputs {
        if evaluator {
            if let Input::Module(module) = input {
                if canonical(&**module).len() > EVALUATOR_BYTES {
                    continue;
                }
            }
        }
        out[input.root()].push(input.run());
    }
    out
}

pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("SemanticDecls"),
        "SemanticDecls is a linked module of the project"
    );
    eligibility_of(&project, "SemanticDecls", MODULE, &ROOTS);

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let (corpus, other) = committed();
    let [declarations, modules] = split(&corpus, false).map(|runs| runs.len());
    assert!(
        declarations >= 200 && modules >= 30,
        "the committed semantic modules carry {declarations} distinct ordinary declarations and {modules} modules of only ordinary declarations"
    );
    let normative = support::normative_host();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (
            seeded_inputs(EVALUATOR_SEEDED, 0x0B55_0070),
            seeded_inputs(RUSTC_SEEDED, 0x0B55_1070),
        )
    } else {
        (Vec::new(), Vec::new())
    };
    eprintln!(
        "BS-70: {} vectors, {declarations} committed ordinary declarations, {modules} committed modules of only ordinary declarations ({other} others), {} and {} seeded declarations ({:?})",
        vectors.len(),
        EVALUATOR_SEEDED,
        RUSTC_SEEDED,
        started.elapsed()
    );

    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, super::Lowered> = ROOTS
        .iter()
        .map(|(root, _)| (*root, super::lower(&checked, &project, MODULE, root)))
        .collect();
    let named_inputs: Vec<Input> = vectors.iter().map(|vector| vector.input.clone()).collect();
    if normative {
        let mut inputs = named_inputs.clone();
        inputs.extend(corpus.iter().cloned());
        inputs.extend(evaluator_seeded);
        for (index, runs) in split(&inputs, true).iter().enumerate() {
            let root = ROOTS[index].0;
            let disagreements = super::evaluate(&lowered[root], runs, 5);
            assert!(
                disagreements.is_empty(),
                "{root} and the Rust oracle disagree: {disagreements:?}"
            );
            eprintln!(
                "BS-70: {root}: the evaluator agrees on {} runs ({:?})",
                runs.len(),
                started.elapsed()
            );
        }
        plants(&vectors);
    } else {
        eprintln!("BS-70: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled = named_inputs;
    compiled.extend(corpus);
    compiled.extend(rustc_seeded);
    let runs = split(&compiled, false);
    let differential: Vec<(&str, &super::Lowered, &[Run])> = ROOTS
        .iter()
        .enumerate()
        .map(|(index, (root, _))| (*root, &lowered[root], runs[index].as_slice()))
        .collect();
    let total = rustc_differential("BS-70", &differential, "oracle");
    eprintln!("BS-70: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-70") {
        return;
    }
    export(&lowered);
    eprintln!("BS-70: passed in {:?}", started.elapsed());
}
