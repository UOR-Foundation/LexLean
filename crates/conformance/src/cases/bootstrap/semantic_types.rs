//! `BS-68` (SPEC.md §17.23, §17.11, §17.12): the closed types of the semantic
//! module language, their primitives and integer families, as language-1.2
//! source, `LexLeanBoot.SemanticTypes`, held to the byte to the serde form of
//! `lexlean::ir::semantic::SemanticType` in canonical JSON, its `Display`, and
//! the facts of `SemanticPrimitive` and `SemanticInteger`.
//!
//! `typeText` writes a type as the semantic module's JSON carries it into the
//! linked IR; `typeSpelling` spells it as diagnostics do; `primitiveFacts`
//! gives a primitive's serialized name, whether it is language 1.2's, whether
//! linking widens it away, and the position of its function argument;
//! `integerName` gives an integer family's serialized name. Every type of every
//! committed semantic module is a run, and so are every primitive, every
//! integer family, and hand-built and seeded types.
//!
//! Each planted defect names the root and the one vector that catch it.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use lexlean::artifact::canonical_json::Json;
use lexlean::calculus::Value;
use lexlean::ir::semantic::{MemberRef, SemanticInteger, SemanticPrimitive, SemanticType};
use serde_json::Value as Json2;

use super::ir_document::linked_projects;
use super::rust_validate::{eligibility_of, rustc_differential};
use super::{option, text, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.SemanticTypes";
const SOURCE: &str = "bootstrap/src/SemanticTypes.lex.tex";
const ROOTS: [(&str, &[&str]); 4] = [
    ("primitiveFacts", &["allocation"]),
    ("integerName", &["allocation"]),
    ("typeText", &["allocation", "overflow", "recursion"]),
    ("typeSpelling", &["allocation", "recursion"]),
];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// Seeded types under the evaluator, and under rustc.
const EVALUATOR_SEEDED: usize = 150;
const RUSTC_SEEDED: usize = 800;

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
            out.insert(name, names);
        }
        out
    });
    tables
        .get(sort)
        .unwrap_or_else(|| panic!("the module has no inductive `{sort}`"))
}

fn adt(sort: &str, constructor: &str, fields: Vec<Value>) -> Value {
    let found = constructors(sort)
        .iter()
        .position(|name| name == constructor)
        .unwrap_or_else(|| panic!("`{sort}` has no constructor `{constructor}`"));
    Value::Adt {
        constructor: u64::try_from(found).expect("an index"),
        fields,
    }
}

// ------------------------------------------------------------- the encoders

/// A unit variant's constructor: `S` and the variant's name.
fn unit_name(ty: &SemanticType) -> Option<String> {
    let debug = format!("{ty:?}");
    debug
        .chars()
        .all(char::is_alphanumeric)
        .then(|| format!("S{debug}"))
}

pub(super) fn type_chain(items: &[SemanticType]) -> Value {
    items
        .iter()
        .rev()
        .fold(adt("SemanticType", "SNil", Vec::new()), |rest, item| {
            adt("SemanticType", "SItem", vec![enc_type(item), rest])
        })
}

pub(super) fn enc_type(ty: &SemanticType) -> Value {
    if let Some(name) = unit_name(ty) {
        return adt("SemanticType", &name, Vec::new());
    }
    match ty {
        SemanticType::Parameter { name } => adt("SemanticType", "SParameter", vec![text(name)]),
        SemanticType::Option { value } => adt("SemanticType", "SOption", vec![enc_type(value)]),
        SemanticType::Result { ok, error } => adt(
            "SemanticType",
            "SResult",
            vec![enc_type(ok), enc_type(error)],
        ),
        SemanticType::List { element } => adt("SemanticType", "SList", vec![enc_type(element)]),
        SemanticType::Named { member, arguments } => adt(
            "SemanticType",
            "SNamed",
            vec![
                option(member.module.as_deref().map(text)),
                text(&member.name),
                type_chain(arguments),
            ],
        ),
        SemanticType::Product { left, right } => adt(
            "SemanticType",
            "SProduct",
            vec![enc_type(left), enc_type(right)],
        ),
        SemanticType::Function { parameters, result } => adt(
            "SemanticType",
            "SFunction",
            vec![type_chain(parameters), enc_type(result)],
        ),
        SemanticType::Map { key, value } => {
            adt("SemanticType", "SMap", vec![enc_type(key), enc_type(value)])
        }
        SemanticType::Set { element } => adt("SemanticType", "SSet", vec![enc_type(element)]),
        other => panic!("no encoding of {other:?}"),
    }
}

pub(super) fn enc_primitive(primitive: SemanticPrimitive) -> Value {
    adt("SemanticPrimitive", &format!("P{primitive:?}"), Vec::new())
}

pub(super) fn enc_integer(integer: SemanticInteger) -> Value {
    adt("SemanticInteger", &format!("I{integer:?}"), Vec::new())
}

/// The serialized name a primitive or an integer family carries.
fn serde_name<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .expect("a name is JSON")
        .as_str()
        .expect("a unit variant serializes as its name")
        .to_owned()
}

/// The canonical JSON of a value's serde form, as the linked IR embeds it.
pub(super) fn canonical<T: serde::Serialize>(value: &T) -> String {
    Json::parse(&serde_json::to_vec(value).expect("a value is JSON"))
        .expect("canonical JSON values")
        .to_canonical_string()
}

#[derive(Clone)]
enum Input {
    Type(Box<SemanticType>),
    Primitive(SemanticPrimitive),
    Integer(SemanticInteger),
}

impl Input {
    /// The roots the input is a run of.
    const fn roots(&self) -> &'static [usize] {
        match self {
            Self::Primitive(_) => &[0],
            Self::Integer(_) => &[1],
            Self::Type(_) => &[2, 3],
        }
    }

    fn run(&self, root: usize) -> Run {
        let (argument, outcome) = match (self, root) {
            (Self::Primitive(primitive), 0) => (
                adt("SemanticPrimitive", &format!("P{primitive:?}"), Vec::new()),
                Value::Adt {
                    constructor: 0,
                    fields: vec![
                        text(&serde_name(primitive)),
                        Value::Bool {
                            value: primitive.language_1_2(),
                        },
                        Value::Bool {
                            value: primitive.widening(),
                        },
                        option(primitive.function_argument().map(|position| Value::Nat {
                            value: position.to_string(),
                        })),
                    ],
                },
            ),
            (Self::Integer(integer), 1) => (
                adt("SemanticInteger", &format!("I{integer:?}"), Vec::new()),
                text(&serde_name(integer)),
            ),
            (Self::Type(ty), 2) => (enc_type(ty), text(&canonical(&**ty))),
            (Self::Type(ty), 3) => (enc_type(ty), text(&ty.to_string())),
            _ => panic!("no run of root {root}"),
        };
        (
            vec![argument],
            serde_json::to_value(outcome).expect("a value is JSON"),
        )
    }
}

// ---------------------------------------------------------------- the corpus

/// Every primitive and integer family the module names, each read back
/// through its serialized name, so that a name the oracle does not have fails.
fn closed_lists() -> (Vec<SemanticPrimitive>, Vec<SemanticInteger>) {
    fn snake(name: &str) -> String {
        if let Some(rest) = name.strip_prefix("UInt") {
            return format!("uint{rest}");
        }
        let mut out = String::new();
        for (index, character) in name.chars().enumerate() {
            if character.is_ascii_uppercase() && index > 0 {
                out.push('_');
            }
            out.push(character.to_ascii_lowercase());
        }
        out
    }
    let primitives = constructors("SemanticPrimitive")
        .iter()
        .map(|name| {
            let serialized = snake(name.strip_prefix('P').expect("a P constructor"));
            serde_json::from_value(Json2::String(serialized.clone()))
                .unwrap_or_else(|_| panic!("the oracle has no primitive `{serialized}`"))
        })
        .collect();
    let integers = constructors("SemanticInteger")
        .iter()
        .map(|name| {
            let serialized = snake(name.strip_prefix('I').expect("an I constructor"));
            serde_json::from_value(Json2::String(serialized.clone()))
                .unwrap_or_else(|_| panic!("the oracle has no integer family `{serialized}`"))
        })
        .collect();
    (primitives, integers)
}

fn types_in(json: &Json2, out: &mut BTreeMap<String, SemanticType>) {
    match json {
        Json2::Object(members) => {
            if let Ok(ty) = serde_json::from_value::<SemanticType>(json.clone()) {
                out.insert(canonical(&ty), ty);
            }
            for value in members.values() {
                types_in(value, out);
            }
        }
        Json2::Array(items) => {
            for item in items {
                types_in(item, out);
            }
        }
        _ => {}
    }
}

/// Every distinct type of every committed semantic module that links.
fn committed() -> Vec<SemanticType> {
    let mut out = BTreeMap::new();
    for project in linked_projects() {
        for module in project.modules.values() {
            if let Some(semantic) = &module.semantic {
                types_in(
                    &serde_json::to_value(semantic).expect("a semantic module is JSON"),
                    &mut out,
                );
            }
        }
    }
    out.into_values().collect()
}

// ------------------------------------------------------------------ vectors

fn named_type(module: Option<&str>, name: &str, arguments: Vec<SemanticType>) -> SemanticType {
    SemanticType::Named {
        member: MemberRef {
            module: module.map(str::to_owned),
            name: name.to_owned(),
        },
        arguments,
    }
}

fn boxed(ty: SemanticType) -> Box<SemanticType> {
    Box::new(ty)
}

const UNIT_TYPES: &[SemanticType] = &[
    SemanticType::Type,
    SemanticType::Nat,
    SemanticType::Bool,
    SemanticType::Prop,
    SemanticType::Unit,
    SemanticType::Int,
    SemanticType::Int8,
    SemanticType::Int16,
    SemanticType::Int32,
    SemanticType::Int64,
    SemanticType::UInt8,
    SemanticType::UInt16,
    SemanticType::UInt32,
    SemanticType::UInt64,
    SemanticType::String,
    SemanticType::Bytes,
    SemanticType::Ordering,
    SemanticType::ContractViolation,
    SemanticType::ReasoningFailure,
];

struct Vector {
    label: &'static str,
    input: Input,
}

fn named() -> Vec<Vector> {
    let mut out = Vec::new();
    let mut add = |label: &'static str, ty: SemanticType| {
        out.push(Vector {
            label,
            input: Input::Type(Box::new(ty)),
        });
    };
    add(
        "every unit type",
        SemanticType::Function {
            parameters: UNIT_TYPES.to_vec(),
            result: boxed(SemanticType::Prop),
        },
    );
    add(
        "a parameter",
        SemanticType::Parameter {
            name: "alpha".to_owned(),
        },
    );
    add(
        "an option",
        SemanticType::Option {
            value: boxed(SemanticType::Nat),
        },
    );
    add(
        "a result",
        SemanticType::Result {
            ok: boxed(SemanticType::Nat),
            error: boxed(SemanticType::Bytes),
        },
    );
    add(
        "a list",
        SemanticType::List {
            element: boxed(SemanticType::Bool),
        },
    );
    add(
        "a named type of a module with arguments",
        named_type(
            Some("LexLeanStd.Json"),
            "Json",
            vec![SemanticType::Nat, SemanticType::Bytes],
        ),
    );
    add(
        "a named type of this module",
        named_type(None, "Tok", Vec::new()),
    );
    add(
        "a product",
        SemanticType::Product {
            left: boxed(SemanticType::Nat),
            right: boxed(SemanticType::Bytes),
        },
    );
    add(
        "a map",
        SemanticType::Map {
            key: boxed(SemanticType::Bytes),
            value: boxed(SemanticType::Nat),
        },
    );
    add(
        "a set",
        SemanticType::Set {
            element: boxed(SemanticType::Int),
        },
    );
    add(
        "a function of two",
        SemanticType::Function {
            parameters: vec![SemanticType::Nat, SemanticType::Bool],
            result: boxed(SemanticType::List {
                element: boxed(SemanticType::Nat),
            }),
        },
    );
    let primitive = |label: &'static str, value: &str| Vector {
        label,
        input: Input::Primitive(
            serde_json::from_value(Json2::String(value.to_owned())).expect("a primitive"),
        ),
    };
    out.push(primitive("utf8 encode", "utf8_encode"));
    out.push(primitive("less than", "less_than"));
    out.push(primitive("to nat", "to_nat"));
    out.push(primitive("iterate until", "iterate_until"));
    out.push(Vector {
        label: "uint64",
        input: Input::Integer(SemanticInteger::UInt64),
    });
    out
}

// ------------------------------------------------------------------- seeded

const NAMES: &[&str] = &["a", "Tok", "x.y", "\"q\"", "\u{e9}"];

fn drawn_type(seeded: &mut Seeded, depth: usize) -> SemanticType {
    let kind = if depth == 0 {
        seeded.below(2)
    } else {
        seeded.below(11)
    };
    let next = depth.saturating_sub(1);
    match kind {
        0 => seeded.pick(UNIT_TYPES).clone(),
        1 => SemanticType::Parameter {
            name: (*seeded.pick(NAMES)).to_owned(),
        },
        2 => SemanticType::Option {
            value: boxed(drawn_type(seeded, next)),
        },
        3 => SemanticType::Result {
            ok: boxed(drawn_type(seeded, next)),
            error: boxed(drawn_type(seeded, next)),
        },
        4 => SemanticType::List {
            element: boxed(drawn_type(seeded, next)),
        },
        5 => SemanticType::Named {
            member: MemberRef {
                module: seeded.chance(50).then(|| (*seeded.pick(NAMES)).to_owned()),
                name: (*seeded.pick(NAMES)).to_owned(),
            },
            arguments: (0..seeded.below(3))
                .map(|_| drawn_type(seeded, next))
                .collect(),
        },
        6 => SemanticType::Product {
            left: boxed(drawn_type(seeded, next)),
            right: boxed(drawn_type(seeded, next)),
        },
        7 => SemanticType::Function {
            parameters: (0..seeded.below(3))
                .map(|_| drawn_type(seeded, next))
                .collect(),
            result: boxed(drawn_type(seeded, next)),
        },
        8 => SemanticType::Map {
            key: boxed(drawn_type(seeded, next)),
            value: boxed(drawn_type(seeded, next)),
        },
        9 => SemanticType::Set {
            element: boxed(drawn_type(seeded, next)),
        },
        _ => seeded.pick(UNIT_TYPES).clone(),
    }
}

fn seeded_types(count: usize, seed: u64) -> Vec<Input> {
    let mut seeded = Seeded(seed);
    (0..count)
        .map(|_| {
            let depth = seeded.below(5);
            Input::Type(Box::new(drawn_type(&mut seeded, depth)))
        })
        .collect()
}

// ------------------------------------------------------------------- plants

struct Plant {
    label: &'static str,
    site: &'static str,
    replacement: &'static str,
    root: usize,
    vector: &'static str,
}

const PLANTS: &[Plant] = &[
    Plant {
        label: "uint8 is split",
        site: "| SemanticType.SUInt8 => (kinded \"uint8\"",
        replacement: "| SemanticType.SUInt8 => (kinded \"u_int8\"",
        root: 2,
        vector: "every unit type",
    },
    Plant {
        label: "a contract violation loses its underscore",
        site: "(kinded \"contract_violation\"",
        replacement: "(kinded \"contractviolation\"",
        root: 2,
        vector: "every unit type",
    },
    Plant {
        label: "a named type drops its module",
        site: "| Option.some source => (member \"module\" (ctor LexLeanStd.Json::Json.JStr source) (member \"name\"",
        replacement: "| Option.some source => (member \"name\" (ctor LexLeanStd.Json::Json.JStr source) (member \"name\"",
        root: 2,
        vector: "a named type of a module with arguments",
    },
    Plant {
        label: "a result swaps its sides",
        site: "(member \"ok\" (typeJson good) (member \"error\" (typeJson bad)",
        replacement: "(member \"ok\" (typeJson bad) (member \"error\" (typeJson good)",
        root: 2,
        vector: "a result",
    },
    Plant {
        label: "a function's result is its value",
        site: "(member \"result\" (typeJson outcome)",
        replacement: "(member \"value\" (typeJson outcome)",
        root: 2,
        vector: "a function of two",
    },
    Plant {
        label: "a map's value is its val",
        site: "(member \"value\" (typeJson entry)",
        replacement: "(member \"val\" (typeJson entry)",
        root: 2,
        vector: "a map",
    },
    Plant {
        label: "a chain is written last first",
        site: "| SemanticType.SItem head tail => (ctor LexLeanStd.Json::Json.JItem (typeJson head) (typeJson tail))",
        replacement: "| SemanticType.SItem head tail => (ctor LexLeanStd.Json::Json.JItem (typeJson tail) (typeJson head))",
        root: 2,
        vector: "a named type of a module with arguments",
    },
    Plant {
        label: "an option is spelled without parentheses",
        site: "(cat (lit \"Option (\") (cat (typeDisplay inner) (lit \")\")))",
        replacement: "(cat (lit \"Option \") (typeDisplay inner))",
        root: 3,
        vector: "an option",
    },
    Plant {
        label: "a result is spelled error first",
        site: "(cat (cat (lit \"Result (\") (typeDisplay good)) (cat (cat (lit \") (\") (typeDisplay bad)) (lit \")\")))",
        replacement: "(cat (cat (lit \"Result (\") (typeDisplay bad)) (cat (cat (lit \") (\") (typeDisplay good)) (lit \")\")))",
        root: 3,
        vector: "a result",
    },
    Plant {
        label: "a named type is spelled without its module",
        site: "| Option.some source => (cat (cat source (lit \".\")) named)",
        replacement: "| Option.some source => named",
        root: 3,
        vector: "a named type of a module with arguments",
    },
    Plant {
        label: "the arguments are spelled last first",
        site: "(displayItems arguments) : Bytes))",
        replacement: "(LexLeanStd.Order::reverseList [Bytes] (displayItems arguments)) : Bytes))",
        root: 3,
        vector: "a named type of a module with arguments",
    },
    Plant {
        label: "a function arrow is spelled with a symbol",
        site: "(lit \") -> \")",
        replacement: "(lit \") => \")",
        root: 3,
        vector: "a function of two",
    },
    Plant {
        label: "a product is spelled Product",
        site: "(lit \"Prod (\")",
        replacement: "(lit \"Product (\")",
        root: 3,
        vector: "a product",
    },
    Plant {
        label: "utf8 is split",
        site: "serdeName := (lit \"utf8_encode\")",
        replacement: "serdeName := (lit \"utf_8_encode\")",
        root: 0,
        vector: "utf8 encode",
    },
    Plant {
        label: "the strict order is language 1.1",
        site: "serdeName := (lit \"less_than\"), languageOneTwo := true",
        replacement: "serdeName := (lit \"less_than\"), languageOneTwo := false",
        root: 0,
        vector: "less than",
    },
    Plant {
        label: "the natural widening is not widened",
        site: "serdeName := (lit \"to_nat\"), languageOneTwo := true, widening := true",
        replacement: "serdeName := (lit \"to_nat\"), languageOneTwo := true, widening := false",
        root: 0,
        vector: "to nat",
    },
    Plant {
        label: "a bounded loop takes no function",
        site: "serdeName := (lit \"iterate_until\"), languageOneTwo := true, widening := false, functionArgument := (ctor Option.some [Nat] 0)",
        replacement: "serdeName := (lit \"iterate_until\"), languageOneTwo := true, widening := false, functionArgument := (ctor Option.none [Nat])",
        root: 0,
        vector: "iterate until",
    },
    Plant {
        label: "uint64 is split",
        site: "| SemanticInteger.IUInt64 => (lit \"uint64\")",
        replacement: "| SemanticInteger.IUInt64 => (lit \"u_int64\")",
        root: 1,
        vector: "uint64",
    },
];

fn plants(vectors: &[Vector]) {
    let mut missed = Vec::new();
    for plant in PLANTS {
        let vector = vectors
            .iter()
            .find(|vector| vector.label == plant.vector)
            .unwrap_or_else(|| panic!("{}: no vector `{}`", plant.label, plant.vector));
        assert!(
            vector.input.roots().contains(&plant.root),
            "{}: the vector is a run of the root planted",
            plant.label
        );
        let root = ROOTS[plant.root].0;
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/SemanticTypes.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/SemanticTypes.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, root);
        match super::evaluate(&low, &[vector.input.run(plant.root)], 1).first() {
            Some(first) => eprintln!(
                "BS-68: plant detected: {}: {}",
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
            "BS-68: {}: largest certificate part {largest} bytes",
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

fn split(inputs: &[Input]) -> [Vec<Run>; 4] {
    let mut out = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    for input in inputs {
        for root in input.roots() {
            out[*root].push(input.run(*root));
        }
    }
    out
}

pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("SemanticTypes"),
        "SemanticTypes is a linked module of the project"
    );
    eligibility_of(&project, "SemanticTypes", MODULE, &ROOTS);

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let (primitives, integers) = closed_lists();
    let types = committed();
    assert!(
        types.len() >= 100,
        "the committed semantic modules carry {} distinct types",
        types.len()
    );
    let mut corpus: Vec<Input> = vectors.iter().map(|vector| vector.input.clone()).collect();
    corpus.extend(primitives.iter().copied().map(Input::Primitive));
    corpus.extend(integers.iter().copied().map(Input::Integer));
    corpus.extend(types.iter().cloned().map(|ty| Input::Type(Box::new(ty))));
    let normative = support::normative_host();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (
            seeded_types(EVALUATOR_SEEDED, 0x0B55_0068),
            seeded_types(RUSTC_SEEDED, 0x0B55_1068),
        )
    } else {
        (Vec::new(), Vec::new())
    };
    eprintln!(
        "BS-68: {} vectors, {} primitives, {} integer families, {} committed types, {} and {} seeded types ({:?})",
        vectors.len(),
        primitives.len(),
        integers.len(),
        types.len(),
        EVALUATOR_SEEDED,
        RUSTC_SEEDED,
        started.elapsed()
    );

    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, super::Lowered> = ROOTS
        .iter()
        .map(|(root, _)| (*root, super::lower(&checked, &project, MODULE, root)))
        .collect();
    if normative {
        let mut inputs = corpus.clone();
        inputs.extend(evaluator_seeded);
        for (index, runs) in split(&inputs).iter().enumerate() {
            let root = ROOTS[index].0;
            let disagreements = super::evaluate(&lowered[root], runs, 5);
            assert!(
                disagreements.is_empty(),
                "{root} and the Rust oracle disagree: {disagreements:?}"
            );
            eprintln!(
                "BS-68: {root}: the evaluator agrees on {} runs ({:?})",
                runs.len(),
                started.elapsed()
            );
        }
        plants(&vectors);
    } else {
        eprintln!("BS-68: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled = corpus;
    compiled.extend(rustc_seeded);
    let runs = split(&compiled);
    let differential: Vec<(&str, &super::Lowered, &[Run])> = ROOTS
        .iter()
        .enumerate()
        .map(|(index, (root, _))| (*root, &lowered[root], runs[index].as_slice()))
        .collect();
    let total = rustc_differential("BS-68", &differential, "oracle");
    eprintln!("BS-68: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-68") {
        return;
    }
    export(&lowered);
    eprintln!("BS-68: passed in {:?}", started.elapsed());
}
