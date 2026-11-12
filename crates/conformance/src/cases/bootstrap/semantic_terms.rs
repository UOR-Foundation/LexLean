//! `BS-69` (SPEC.md §17.23, §17.11, §17.12): the closed terms and proofs of the
//! semantic module language as language-1.2 source, `LexLeanBoot.SemanticTerms`,
//! held to the byte to the serde form of `lexlean::ir::semantic::SemanticTerm`
//! and `SemanticProof` in canonical JSON, as the linked IR embeds them.
//!
//! `termText` writes a term and `proofText` a proof, each tagged by its kind,
//! a list of type arguments, definitions, generalized names, or premises only
//! when it is not empty where the oracle skips it, and its types through
//! `SemanticTypes` (`BS-68`). Every term and proof of every committed semantic
//! module that links is a run, and so are hand-built and seeded ones.
//!
//! Each planted defect names the root and the one vector that catch it.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use lexlean::calculus::Value;
use lexlean::ir::semantic::{
    MemberRef, ModelCheck, SemanticAssignment, SemanticBranch, SemanticEdge, SemanticInteger,
    SemanticMapEntry, SemanticNestedMember, SemanticParameter, SemanticPrimitive, SemanticProof,
    SemanticProofBranch, SemanticReflection, SemanticReflectionComparison, SemanticReflectionField,
    SemanticTerm, SemanticType,
};
use serde_json::Value as Json2;

use super::ir_document::linked_projects;
use super::rust_validate::{eligibility_of, rustc_differential};
use super::semantic_types::{canonical, enc_integer, enc_primitive, enc_type, type_chain};
use super::{list, option, pair, text, texts, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.SemanticTerms";
const SOURCE: &str = "bootstrap/src/SemanticTerms.lex.tex";
const ROOTS: [(&str, &[&str]); 2] = [
    ("termText", &["allocation", "overflow", "recursion"]),
    ("proofText", &["allocation", "overflow", "recursion"]),
];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// Seeded terms and proofs under the evaluator, and under rustc.
const EVALUATOR_SEEDED: usize = 120;
const RUSTC_SEEDED: usize = 600;

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

fn term(constructor: &str, fields: Vec<Value>) -> Value {
    adt("STerm", constructor, fields)
}

fn member_fields(member: &MemberRef) -> [Value; 2] {
    [
        option(member.module.as_deref().map(text)),
        text(&member.name),
    ]
}

fn members(refs: &[MemberRef]) -> Value {
    list(
        refs.iter()
            .map(|member| {
                let [module, name] = member_fields(member);
                pair(module, name)
            })
            .collect(),
    )
}

fn serde_text<T: serde::Serialize>(value: &T) -> Value {
    text(
        serde_json::to_value(value)
            .expect("a name is JSON")
            .as_str()
            .expect("a unit variant serializes as its name"),
    )
}

fn chain<T>(items: &[T], cell: impl Fn(&T, Value) -> Value) -> Value {
    items
        .iter()
        .rev()
        .fold(term("KEnd", Vec::new()), |rest, item| cell(item, rest))
}

fn term_chain(items: &[SemanticTerm]) -> Value {
    chain(items, |item, rest| {
        term("KItem", vec![enc_term(item), rest])
    })
}

fn assign_chain(items: &[SemanticAssignment]) -> Value {
    chain(items, |item, rest| {
        term(
            "KAssign",
            vec![text(&item.field), enc_term(&item.value), rest],
        )
    })
}

fn branch_chain(items: &[SemanticBranch]) -> Value {
    chain(items, |item, rest| {
        let [module, name] = member_fields(&item.constructor);
        term(
            "KBranch",
            vec![
                module,
                name,
                texts(&item.binders),
                enc_term(&item.body),
                rest,
            ],
        )
    })
}

fn entry_chain(items: &[SemanticMapEntry]) -> Value {
    chain(items, |item, rest| {
        term(
            "KEntry",
            vec![enc_term(&item.key), enc_term(&item.value), rest],
        )
    })
}

fn edge_chain(items: &[SemanticEdge]) -> Value {
    chain(items, |item, rest| {
        term(
            "KEdge",
            vec![enc_term(&item.source), enc_term(&item.target), rest],
        )
    })
}

fn param_fields(parameter: &SemanticParameter) -> [Value; 2] {
    [text(&parameter.name), enc_type(&parameter.r#type)]
}

#[allow(clippy::too_many_lines)]
fn enc_term(value: &SemanticTerm) -> Value {
    let binary = |name: &str, left: &SemanticTerm, right: &SemanticTerm| {
        term(name, vec![enc_term(left), enc_term(right)])
    };
    match value {
        SemanticTerm::Var { name } => term("TVar", vec![text(name)]),
        SemanticTerm::Nat { value } => term("TNat", vec![text(value)]),
        SemanticTerm::Integer {
            representation,
            value,
        } => term("TInteger", vec![enc_integer(*representation), text(value)]),
        SemanticTerm::String { value } => term("TString", vec![text(value)]),
        SemanticTerm::Bytes { hex } => term("TBytes", vec![text(hex)]),
        SemanticTerm::Primitive {
            operation,
            arguments,
            result,
        } => term(
            "TPrimitive",
            vec![
                enc_primitive(*operation),
                term_chain(arguments),
                enc_type(result),
            ],
        ),
        SemanticTerm::Bool { value } => term("TBool", vec![Value::Bool { value: *value }]),
        SemanticTerm::Unit => term("TUnit", Vec::new()),
        SemanticTerm::Nil { element } => term("TNil", vec![enc_type(element)]),
        SemanticTerm::Cons { head, tail } => binary("TCons", head, tail),
        SemanticTerm::Record {
            r#type,
            type_arguments,
            fields,
        } => {
            let [module, name] = member_fields(r#type);
            term(
                "TRecord",
                vec![
                    module,
                    name,
                    type_chain(type_arguments),
                    assign_chain(fields),
                ],
            )
        }
        SemanticTerm::Constructor {
            constructor,
            type_arguments,
            arguments,
        } => {
            let [module, name] = member_fields(constructor);
            term(
                "TConstructor",
                vec![
                    module,
                    name,
                    type_chain(type_arguments),
                    term_chain(arguments),
                ],
            )
        }
        SemanticTerm::InstanceValue {
            class,
            arguments,
            resolved,
        } => {
            let [module, name] = member_fields(class);
            let [resolved_module, resolved_name] = member_fields(resolved);
            term(
                "TInstanceValue",
                vec![
                    module,
                    name,
                    type_chain(arguments),
                    resolved_module,
                    resolved_name,
                ],
            )
        }
        SemanticTerm::Project { value, field } => {
            term("TProject", vec![enc_term(value), text(field)])
        }
        SemanticTerm::Call {
            function,
            type_arguments,
            arguments,
        } => {
            let [module, name] = member_fields(function);
            term(
                "TCall",
                vec![
                    module,
                    name,
                    type_chain(type_arguments),
                    term_chain(arguments),
                ],
            )
        }
        SemanticTerm::If {
            condition,
            then_value,
            else_value,
        } => term(
            "TIf",
            vec![
                enc_term(condition),
                enc_term(then_value),
                enc_term(else_value),
            ],
        ),
        SemanticTerm::Match {
            scrutinee,
            branches,
        } => term("TMatch", vec![enc_term(scrutinee), branch_chain(branches)]),
        SemanticTerm::Eq { left, right } => binary("TEq", left, right),
        SemanticTerm::Le { left, right } => binary("TLe", left, right),
        SemanticTerm::Lt { left, right } => binary("TLt", left, right),
        SemanticTerm::Add { left, right } => binary("TAdd", left, right),
        SemanticTerm::Beq { left, right } => binary("TBeq", left, right),
        SemanticTerm::Ble { left, right } => binary("TBle", left, right),
        SemanticTerm::Blt { left, right } => binary("TBlt", left, right),
        SemanticTerm::And { left, right } => binary("TAnd", left, right),
        SemanticTerm::PropAnd { left, right } => binary("TPropAnd", left, right),
        SemanticTerm::Or { left, right } => binary("TOr", left, right),
        SemanticTerm::Not { value } => term("TNot", vec![enc_term(value)]),
        SemanticTerm::Implies {
            premise,
            conclusion,
        } => binary("TImplies", premise, conclusion),
        SemanticTerm::Iff { left, right } => binary("TIff", left, right),
        SemanticTerm::Forall { binder, body } => {
            let [name, ty] = param_fields(binder);
            term("TForall", vec![name, ty, enc_term(body)])
        }
        SemanticTerm::Let {
            binder,
            value,
            body,
        } => {
            let [name, ty] = param_fields(binder);
            term("TLet", vec![name, ty, enc_term(value), enc_term(body)])
        }
        SemanticTerm::Pair { left, right } => binary("TPair", left, right),
        SemanticTerm::First { value } => term("TFirst", vec![enc_term(value)]),
        SemanticTerm::Second { value } => term("TSecond", vec![enc_term(value)]),
        SemanticTerm::Lambda {
            parameters,
            captures,
            body,
        } => term(
            "TLambda",
            vec![
                list(
                    parameters
                        .iter()
                        .map(|parameter| {
                            let [name, ty] = param_fields(parameter);
                            pair(name, ty)
                        })
                        .collect(),
                ),
                texts(captures),
                enc_term(body),
            ],
        ),
        SemanticTerm::Apply {
            function,
            arguments,
        } => term("TApply", vec![enc_term(function), term_chain(arguments)]),
        SemanticTerm::FunctionRef {
            function,
            type_arguments,
        } => {
            let [module, name] = member_fields(function);
            term(
                "TFunctionRef",
                vec![module, name, type_chain(type_arguments)],
            )
        }
        SemanticTerm::MapLiteral {
            key,
            value,
            entries,
        } => term(
            "TMapLiteral",
            vec![enc_type(key), enc_type(value), entry_chain(entries)],
        ),
        SemanticTerm::SetLiteral { element, elements } => {
            term("TSetLiteral", vec![enc_type(element), term_chain(elements)])
        }
        SemanticTerm::GraphLiteral { node, nodes, edges } => term(
            "TGraphLiteral",
            vec![enc_type(node), term_chain(nodes), edge_chain(edges)],
        ),
        SemanticTerm::CheckedApply {
            model,
            type_arguments,
            arguments,
            checks,
        } => {
            let [module, name] = member_fields(model);
            term(
                "TCheckedApply",
                vec![
                    module,
                    name,
                    type_chain(type_arguments),
                    term_chain(arguments),
                    list(checks.iter().map(serde_text).collect()),
                ],
            )
        }
    }
}

fn proof(constructor: &str, fields: Vec<Value>) -> Value {
    adt("SProof", constructor, fields)
}

fn proof_end() -> Value {
    proof("QEnd", Vec::new())
}

fn proof_chain(items: &[SemanticProof]) -> Value {
    items.iter().rev().fold(proof_end(), |rest, item| {
        proof("QItem", vec![enc_proof(item), rest])
    })
}

fn proof_branches(items: &[SemanticProofBranch]) -> Value {
    items.iter().rev().fold(proof_end(), |rest, item| {
        proof(
            "QBranch",
            vec![
                text(&item.constructor),
                texts(&item.binders),
                enc_proof(&item.proof),
                rest,
            ],
        )
    })
}

fn nested_members(items: &[SemanticNestedMember]) -> Value {
    items.iter().rev().fold(proof_end(), |rest, item| {
        proof(
            "QMember",
            vec![
                enc_type(&item.r#type),
                text(&item.binder),
                enc_term(&item.statement),
                proof_branches(&item.branches),
                rest,
            ],
        )
    })
}

fn enc_reflection(reflection: &SemanticReflection) -> Value {
    match reflection {
        SemanticReflection::List {
            parameter,
            values,
            boolean_definition,
            proposition_definition,
            comparison,
        } => {
            let [boolean_module, boolean_name] = member_fields(boolean_definition);
            let [proposition_module, proposition_name] = member_fields(proposition_definition);
            adt(
                "Reflection",
                "RList",
                vec![
                    text(parameter),
                    text(values),
                    boolean_module,
                    boolean_name,
                    proposition_module,
                    proposition_name,
                    serde_text(comparison),
                ],
            )
        }
        SemanticReflection::Record {
            record,
            boolean_definition,
            proposition_definition,
            fields,
        } => {
            let [boolean_module, boolean_name] = member_fields(boolean_definition);
            let [proposition_module, proposition_name] = member_fields(proposition_definition);
            adt(
                "Reflection",
                "RRecord",
                vec![
                    text(record),
                    boolean_module,
                    boolean_name,
                    proposition_module,
                    proposition_name,
                    list(
                        fields
                            .iter()
                            .map(|field| pair(text(&field.field), text(&field.expected)))
                            .collect(),
                    ),
                ],
            )
        }
    }
}

fn enc_proof(value: &SemanticProof) -> Value {
    match value {
        SemanticProof::Reflexivity => proof("PReflexivity", Vec::new()),
        SemanticProof::Decide => proof("PDecide", Vec::new()),
        SemanticProof::Congruence => proof("PCongruence", Vec::new()),
        SemanticProof::LinearArithmetic { definitions } => {
            proof("PLinearArithmetic", vec![members(definitions)])
        }
        SemanticProof::Simplify { definitions } => proof("PSimplify", vec![members(definitions)]),
        SemanticProof::Constructor { branches } => {
            proof("PConstructor", vec![proof_chain(branches)])
        }
        SemanticProof::Cases {
            scrutinee,
            branches,
        } => proof("PCases", vec![text(scrutinee), proof_branches(branches)]),
        SemanticProof::TermCases {
            scrutinee,
            definitions,
        } => proof(
            "PTermCases",
            vec![enc_term(scrutinee), members(definitions)],
        ),
        SemanticProof::Induction {
            scrutinee,
            generalizing,
            branches,
        } => proof(
            "PInduction",
            vec![
                text(scrutinee),
                texts(generalizing),
                proof_branches(branches),
            ],
        ),
        SemanticProof::NestedInduction {
            scrutinee,
            branches,
            members,
        } => proof(
            "PNestedInduction",
            vec![
                text(scrutinee),
                proof_branches(branches),
                nested_members(members),
            ],
        ),
        SemanticProof::BooleanReflection { reflection } => {
            proof("PBooleanReflection", vec![enc_reflection(reflection)])
        }
        SemanticProof::Law {
            name,
            type_arguments,
            arguments,
            premises,
        } => proof(
            "PLaw",
            vec![
                text(name),
                type_chain(type_arguments),
                term_chain(arguments),
                members(premises),
            ],
        ),
        SemanticProof::Apply {
            theorem,
            type_arguments,
            arguments,
        } => {
            let [module, name] = member_fields(theorem);
            proof(
                "PApply",
                vec![
                    module,
                    name,
                    type_chain(type_arguments),
                    term_chain(arguments),
                ],
            )
        }
    }
}

#[derive(Clone)]
enum Input {
    Term(Box<SemanticTerm>),
    Proof(Box<SemanticProof>),
}

impl Input {
    const fn root(&self) -> usize {
        match self {
            Self::Term(_) => 0,
            Self::Proof(_) => 1,
        }
    }

    fn run(&self) -> Run {
        let (argument, outcome) = match self {
            Self::Term(value) => (enc_term(value), text(&canonical(&**value))),
            Self::Proof(value) => (enc_proof(value), text(&canonical(&**value))),
        };
        (
            vec![argument],
            serde_json::to_value(outcome).expect("a value is JSON"),
        )
    }
}

// ---------------------------------------------------------------- the corpus

fn found_in(json: &Json2, terms: &mut BTreeMap<String, Input>) {
    match json {
        Json2::Object(fields) => {
            if let Ok(value) = serde_json::from_value::<SemanticTerm>(json.clone()) {
                terms.insert(canonical(&value), Input::Term(Box::new(value)));
            } else if let Ok(value) = serde_json::from_value::<SemanticProof>(json.clone()) {
                terms.insert(canonical(&value), Input::Proof(Box::new(value)));
            }
            for field in fields.values() {
                found_in(field, terms);
            }
        }
        Json2::Array(items) => {
            for item in items {
                found_in(item, terms);
            }
        }
        _ => {}
    }
}

/// Every distinct term and proof of every committed semantic module that
/// links.
fn committed() -> Vec<Input> {
    let mut out = BTreeMap::new();
    for project in linked_projects() {
        for module in project.modules.values() {
            if let Some(semantic) = &module.semantic {
                found_in(
                    &serde_json::to_value(semantic).expect("a semantic module is JSON"),
                    &mut out,
                );
            }
        }
    }
    out.into_values().collect()
}

// ------------------------------------------------------------------ vectors

fn var(name: &str) -> SemanticTerm {
    SemanticTerm::Var {
        name: name.to_owned(),
    }
}

fn nat(value: &str) -> SemanticTerm {
    SemanticTerm::Nat {
        value: value.to_owned(),
    }
}

fn boxed(value: SemanticTerm) -> Box<SemanticTerm> {
    Box::new(value)
}

fn reference(module: Option<&str>, name: &str) -> MemberRef {
    MemberRef {
        module: module.map(str::to_owned),
        name: name.to_owned(),
    }
}

fn parameter(name: &str, ty: SemanticType) -> SemanticParameter {
    SemanticParameter {
        name: name.to_owned(),
        r#type: ty,
    }
}

fn branch(constructor: &str, binders: &[&str], proof: SemanticProof) -> SemanticProofBranch {
    SemanticProofBranch {
        constructor: constructor.to_owned(),
        binders: binders.iter().map(|binder| (*binder).to_owned()).collect(),
        proof: Box::new(proof),
    }
}

fn names(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| (*item).to_owned()).collect()
}

struct Vector {
    label: &'static str,
    input: Input,
}

#[allow(clippy::too_many_lines)]
fn named() -> Vec<Vector> {
    let mut out = Vec::new();
    let mut add_term = |label: &'static str, value: SemanticTerm| {
        out.push(Vector {
            label,
            input: Input::Term(Box::new(value)),
        });
    };
    add_term(
        "every literal",
        SemanticTerm::Primitive {
            operation: SemanticPrimitive::Append,
            arguments: vec![
                var("x"),
                nat("7"),
                SemanticTerm::Integer {
                    representation: SemanticInteger::UInt8,
                    value: "255".to_owned(),
                },
                SemanticTerm::String {
                    value: "a\"b\n".to_owned(),
                },
                SemanticTerm::Bytes {
                    hex: "00ff".to_owned(),
                },
                SemanticTerm::Bool { value: true },
                SemanticTerm::Unit,
            ],
            result: SemanticType::Bytes,
        },
    );
    add_term(
        "a list",
        SemanticTerm::Cons {
            head: boxed(nat("1")),
            tail: boxed(SemanticTerm::Nil {
                element: SemanticType::Nat,
            }),
        },
    );
    add_term(
        "a record with type arguments",
        SemanticTerm::Record {
            r#type: reference(Some("M"), "Pair"),
            type_arguments: vec![SemanticType::Nat],
            fields: vec![
                SemanticAssignment {
                    field: "a".to_owned(),
                    value: nat("1"),
                },
                SemanticAssignment {
                    field: "b".to_owned(),
                    value: SemanticTerm::Bool { value: false },
                },
            ],
        },
    );
    add_term(
        "a record without type arguments",
        SemanticTerm::Record {
            r#type: reference(None, "Point"),
            type_arguments: Vec::new(),
            fields: vec![SemanticAssignment {
                field: "x".to_owned(),
                value: nat("0"),
            }],
        },
    );
    add_term(
        "a constructor and a call",
        SemanticTerm::Call {
            function: reference(Some("Lib"), "size"),
            type_arguments: vec![SemanticType::Bool],
            arguments: vec![SemanticTerm::Constructor {
                constructor: reference(None, "Leaf"),
                type_arguments: Vec::new(),
                arguments: vec![var("v")],
            }],
        },
    );
    add_term(
        "an instance value without arguments",
        SemanticTerm::InstanceValue {
            class: reference(None, "Show"),
            arguments: Vec::new(),
            resolved: reference(Some("M"), "showNat"),
        },
    );
    add_term(
        "an if",
        SemanticTerm::If {
            condition: boxed(var("c")),
            then_value: boxed(nat("1")),
            else_value: boxed(nat("2")),
        },
    );
    add_term(
        "a match",
        SemanticTerm::Match {
            scrutinee: boxed(SemanticTerm::Project {
                value: boxed(var("r")),
                field: "f".to_owned(),
            }),
            branches: vec![
                SemanticBranch {
                    constructor: reference(None, "none"),
                    binders: Vec::new(),
                    body: nat("0"),
                },
                SemanticBranch {
                    constructor: reference(Some("Option"), "some"),
                    binders: names(&["inner"]),
                    body: var("inner"),
                },
            ],
        },
    );
    let pair_of = |left: &str, right: &str| (boxed(var(left)), boxed(var(right)));
    let (a, b) = pair_of("a", "b");
    add_term(
        "every connective",
        SemanticTerm::Primitive {
            operation: SemanticPrimitive::Equal,
            arguments: vec![
                SemanticTerm::Eq {
                    left: a.clone(),
                    right: b.clone(),
                },
                SemanticTerm::Le {
                    left: a.clone(),
                    right: b.clone(),
                },
                SemanticTerm::Lt {
                    left: a.clone(),
                    right: b.clone(),
                },
                SemanticTerm::Add {
                    left: a.clone(),
                    right: b.clone(),
                },
                SemanticTerm::Beq {
                    left: a.clone(),
                    right: b.clone(),
                },
                SemanticTerm::Ble {
                    left: a.clone(),
                    right: b.clone(),
                },
                SemanticTerm::Blt {
                    left: a.clone(),
                    right: b.clone(),
                },
                SemanticTerm::And {
                    left: a.clone(),
                    right: b.clone(),
                },
                SemanticTerm::PropAnd {
                    left: a.clone(),
                    right: b.clone(),
                },
                SemanticTerm::Or {
                    left: a.clone(),
                    right: b.clone(),
                },
                SemanticTerm::Iff {
                    left: a.clone(),
                    right: b.clone(),
                },
                SemanticTerm::Implies {
                    premise: a.clone(),
                    conclusion: b,
                },
                SemanticTerm::Not { value: a },
            ],
            result: SemanticType::Bool,
        },
    );
    add_term(
        "binders",
        SemanticTerm::Forall {
            binder: parameter("x", SemanticType::Nat),
            body: boxed(SemanticTerm::Let {
                binder: parameter("y", SemanticType::Bool),
                value: boxed(var("x")),
                body: boxed(SemanticTerm::Lambda {
                    parameters: vec![parameter("z", SemanticType::Nat)],
                    captures: names(&["x", "y"]),
                    body: boxed(SemanticTerm::Apply {
                        function: boxed(var("f")),
                        arguments: vec![var("z")],
                    }),
                }),
            }),
        },
    );
    add_term(
        "pairs",
        SemanticTerm::Pair {
            left: boxed(SemanticTerm::First {
                value: boxed(var("p")),
            }),
            right: boxed(SemanticTerm::Second {
                value: boxed(var("p")),
            }),
        },
    );
    add_term(
        "a function reference",
        SemanticTerm::FunctionRef {
            function: reference(Some("Lib"), "map"),
            type_arguments: vec![SemanticType::Nat, SemanticType::Bool],
        },
    );
    add_term(
        "a map literal",
        SemanticTerm::MapLiteral {
            key: SemanticType::Bytes,
            value: SemanticType::Nat,
            entries: vec![SemanticMapEntry {
                key: SemanticTerm::Bytes {
                    hex: "61".to_owned(),
                },
                value: nat("1"),
            }],
        },
    );
    add_term(
        "a set literal",
        SemanticTerm::SetLiteral {
            element: SemanticType::Nat,
            elements: vec![nat("1"), nat("2")],
        },
    );
    add_term(
        "a graph literal",
        SemanticTerm::GraphLiteral {
            node: SemanticType::Nat,
            nodes: vec![nat("1"), nat("2")],
            edges: vec![SemanticEdge {
                source: nat("1"),
                target: nat("2"),
            }],
        },
    );
    add_term(
        "a checked application",
        SemanticTerm::CheckedApply {
            model: reference(None, "classify"),
            type_arguments: Vec::new(),
            arguments: vec![var("input")],
            checks: vec![ModelCheck::Postcondition, ModelCheck::Precondition],
        },
    );
    let mut add_proof = |label: &'static str, value: SemanticProof| {
        out.push(Vector {
            label,
            input: Input::Proof(Box::new(value)),
        });
    };
    add_proof(
        "unit proofs",
        SemanticProof::Constructor {
            branches: vec![
                SemanticProof::Reflexivity,
                SemanticProof::Decide,
                SemanticProof::Congruence,
            ],
        },
    );
    add_proof(
        "linear arithmetic with definitions",
        SemanticProof::LinearArithmetic {
            definitions: vec![reference(None, "size")],
        },
    );
    add_proof(
        "linear arithmetic without definitions",
        SemanticProof::LinearArithmetic {
            definitions: Vec::new(),
        },
    );
    add_proof(
        "simplify",
        SemanticProof::Simplify {
            definitions: vec![reference(Some("M"), "f"), reference(None, "g")],
        },
    );
    add_proof(
        "cases",
        SemanticProof::Cases {
            scrutinee: "n".to_owned(),
            branches: vec![
                branch("zero", &[], SemanticProof::Reflexivity),
                branch("succ", &["m"], SemanticProof::Decide),
            ],
        },
    );
    add_proof(
        "term cases",
        SemanticProof::TermCases {
            scrutinee: var("t"),
            definitions: vec![reference(None, "f")],
        },
    );
    add_proof(
        "induction generalizing",
        SemanticProof::Induction {
            scrutinee: "xs".to_owned(),
            generalizing: names(&["acc"]),
            branches: vec![branch("nil", &[], SemanticProof::Reflexivity)],
        },
    );
    add_proof(
        "induction without generalizing",
        SemanticProof::Induction {
            scrutinee: "xs".to_owned(),
            generalizing: Vec::new(),
            branches: vec![branch("cons", &["h", "t", "ih"], SemanticProof::Decide)],
        },
    );
    add_proof(
        "nested induction",
        SemanticProof::NestedInduction {
            scrutinee: "tree".to_owned(),
            branches: vec![branch(
                "node",
                &["children", "ih"],
                SemanticProof::Reflexivity,
            )],
            members: vec![SemanticNestedMember {
                r#type: SemanticType::List {
                    element: Box::new(SemanticType::Nat),
                },
                binder: "items".to_owned(),
                statement: SemanticTerm::Bool { value: true },
                branches: vec![branch("nil", &[], SemanticProof::Decide)],
            }],
        },
    );
    add_proof(
        "list reflection",
        SemanticProof::BooleanReflection {
            reflection: SemanticReflection::List {
                parameter: "bound".to_owned(),
                values: "xs".to_owned(),
                boolean_definition: reference(None, "allBelow"),
                proposition_definition: reference(Some("M"), "AllBelow"),
                comparison: SemanticReflectionComparison::NatBlt,
            },
        },
    );
    add_proof(
        "record reflection",
        SemanticProof::BooleanReflection {
            reflection: SemanticReflection::Record {
                record: "config".to_owned(),
                boolean_definition: reference(None, "valid"),
                proposition_definition: reference(None, "Valid"),
                fields: vec![SemanticReflectionField {
                    field: "size".to_owned(),
                    expected: "3".to_owned(),
                }],
            },
        },
    );
    add_proof(
        "a law with everything",
        SemanticProof::Law {
            name: "fold_append".to_owned(),
            type_arguments: vec![SemanticType::Nat],
            arguments: vec![var("xs"), var("ys")],
            premises: vec![reference(None, "lemma_one")],
        },
    );
    add_proof(
        "a law with nothing optional",
        SemanticProof::Law {
            name: "fold_nil".to_owned(),
            type_arguments: Vec::new(),
            arguments: Vec::new(),
            premises: Vec::new(),
        },
    );
    add_proof(
        "apply a theorem",
        SemanticProof::Apply {
            theorem: reference(Some("M"), "add_comm"),
            type_arguments: Vec::new(),
            arguments: vec![var("a"), var("b")],
        },
    );
    out
}

// ------------------------------------------------------------------- seeded

const NAMES: &[&str] = &["x", "acc", "\"q\"", "\u{e9}", ""];

fn drawn_name(seeded: &mut Seeded) -> String {
    (*seeded.pick(NAMES)).to_owned()
}

fn drawn_ref(seeded: &mut Seeded) -> MemberRef {
    MemberRef {
        module: seeded.chance(40).then(|| drawn_name(seeded)),
        name: drawn_name(seeded),
    }
}

fn drawn_types(seeded: &mut Seeded) -> Vec<SemanticType> {
    (0..seeded.below(3))
        .map(|_| {
            seeded
                .pick(&[SemanticType::Nat, SemanticType::Bool, SemanticType::Bytes])
                .clone()
        })
        .collect()
}

fn drawn_terms(seeded: &mut Seeded, depth: usize) -> Vec<SemanticTerm> {
    (0..seeded.below(3))
        .map(|_| drawn_term(seeded, depth))
        .collect()
}

fn drawn_term(seeded: &mut Seeded, depth: usize) -> SemanticTerm {
    let kind = if depth == 0 {
        seeded.below(3)
    } else {
        seeded.below(12)
    };
    let next = depth.saturating_sub(1);
    match kind {
        0 => var(&drawn_name(seeded)),
        1 => nat(seeded.pick(&["0", "7", "18446744073709551616"])),
        2 => SemanticTerm::Integer {
            representation: *seeded.pick(&[SemanticInteger::Int, SemanticInteger::Int64]),
            value: "-3".to_owned(),
        },
        3 => SemanticTerm::Call {
            function: drawn_ref(seeded),
            type_arguments: drawn_types(seeded),
            arguments: drawn_terms(seeded, next),
        },
        4 => SemanticTerm::Constructor {
            constructor: drawn_ref(seeded),
            type_arguments: drawn_types(seeded),
            arguments: drawn_terms(seeded, next),
        },
        5 => SemanticTerm::Record {
            r#type: drawn_ref(seeded),
            type_arguments: drawn_types(seeded),
            fields: (0..seeded.below(3))
                .map(|_| SemanticAssignment {
                    field: drawn_name(seeded),
                    value: drawn_term(seeded, next),
                })
                .collect(),
        },
        6 => SemanticTerm::Match {
            scrutinee: boxed(drawn_term(seeded, next)),
            branches: (0..seeded.below(3))
                .map(|_| SemanticBranch {
                    constructor: drawn_ref(seeded),
                    binders: (0..seeded.below(3)).map(|_| drawn_name(seeded)).collect(),
                    body: drawn_term(seeded, next),
                })
                .collect(),
        },
        7 => SemanticTerm::Lambda {
            parameters: (0..seeded.below(3))
                .map(|_| parameter(&drawn_name(seeded), SemanticType::Nat))
                .collect(),
            captures: (0..seeded.below(3)).map(|_| drawn_name(seeded)).collect(),
            body: boxed(drawn_term(seeded, next)),
        },
        8 => SemanticTerm::Let {
            binder: parameter(&drawn_name(seeded), SemanticType::Bool),
            value: boxed(drawn_term(seeded, next)),
            body: boxed(drawn_term(seeded, next)),
        },
        9 => SemanticTerm::FunctionRef {
            function: drawn_ref(seeded),
            type_arguments: drawn_types(seeded),
        },
        10 => SemanticTerm::CheckedApply {
            model: drawn_ref(seeded),
            type_arguments: drawn_types(seeded),
            arguments: drawn_terms(seeded, next),
            checks: (0..seeded.below(3))
                .map(|_| {
                    *seeded.pick(&[
                        ModelCheck::InputInvariant,
                        ModelCheck::OutputInvariant,
                        ModelCheck::Postcondition,
                        ModelCheck::Precondition,
                    ])
                })
                .collect(),
        },
        _ => SemanticTerm::If {
            condition: boxed(drawn_term(seeded, next)),
            then_value: boxed(drawn_term(seeded, next)),
            else_value: boxed(drawn_term(seeded, next)),
        },
    }
}

fn drawn_refs(seeded: &mut Seeded) -> Vec<MemberRef> {
    (0..seeded.below(3)).map(|_| drawn_ref(seeded)).collect()
}

fn drawn_branches(seeded: &mut Seeded, depth: usize) -> Vec<SemanticProofBranch> {
    (0..seeded.below(3))
        .map(|_| SemanticProofBranch {
            constructor: drawn_name(seeded),
            binders: (0..seeded.below(3)).map(|_| drawn_name(seeded)).collect(),
            proof: Box::new(drawn_proof(seeded, depth)),
        })
        .collect()
}

fn drawn_proof(seeded: &mut Seeded, depth: usize) -> SemanticProof {
    let kind = if depth == 0 {
        seeded.below(4)
    } else {
        seeded.below(10)
    };
    let next = depth.saturating_sub(1);
    match kind {
        0 => SemanticProof::Reflexivity,
        1 => SemanticProof::LinearArithmetic {
            definitions: drawn_refs(seeded),
        },
        2 => SemanticProof::Simplify {
            definitions: drawn_refs(seeded),
        },
        3 => SemanticProof::Law {
            name: drawn_name(seeded),
            type_arguments: drawn_types(seeded),
            arguments: drawn_terms(seeded, 1),
            premises: drawn_refs(seeded),
        },
        4 => SemanticProof::Constructor {
            branches: (0..seeded.below(3))
                .map(|_| drawn_proof(seeded, next))
                .collect(),
        },
        5 => SemanticProof::Cases {
            scrutinee: drawn_name(seeded),
            branches: drawn_branches(seeded, next),
        },
        6 => SemanticProof::Induction {
            scrutinee: drawn_name(seeded),
            generalizing: (0..seeded.below(3)).map(|_| drawn_name(seeded)).collect(),
            branches: drawn_branches(seeded, next),
        },
        7 => SemanticProof::TermCases {
            scrutinee: drawn_term(seeded, 1),
            definitions: drawn_refs(seeded),
        },
        8 => SemanticProof::Apply {
            theorem: drawn_ref(seeded),
            type_arguments: drawn_types(seeded),
            arguments: drawn_terms(seeded, 1),
        },
        _ => SemanticProof::NestedInduction {
            scrutinee: drawn_name(seeded),
            branches: drawn_branches(seeded, next),
            members: (0..seeded.below(2))
                .map(|_| SemanticNestedMember {
                    r#type: SemanticType::Nat,
                    binder: drawn_name(seeded),
                    statement: drawn_term(seeded, 1),
                    branches: drawn_branches(seeded, next),
                })
                .collect(),
        },
    }
}

fn seeded_inputs(count: usize, seed: u64) -> Vec<Input> {
    let mut seeded = Seeded(seed);
    (0..count)
        .flat_map(|_| {
            let depth = seeded.below(4);
            let drawn = drawn_term(&mut seeded, depth);
            let proved = drawn_proof(&mut seeded, depth);
            [Input::Term(Box::new(drawn)), Input::Proof(Box::new(proved))]
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
        label: "a propositional conjunction is a Boolean one",
        site: "(kinded \"prop_and\"",
        replacement: "(kinded \"and\"",
        vector: "every connective",
    },
    Plant {
        label: "empty type arguments are written",
        site: "| SemanticTypes::SemanticType.SNil => rest",
        replacement: "| SemanticTypes::SemanticType.SNil => (member key (ctor LexLeanStd.Json::Json.JArr (SemanticTypes::typeJson types)) rest)",
        vector: "a record without type arguments",
    },
    Plant {
        label: "an instance value skips empty arguments",
        site: "(member \"arguments\" (ctor LexLeanStd.Json::Json.JArr (SemanticTypes::typeJson typeArgs))",
        replacement: "(typesWhenAny \"arguments\" typeArgs",
        vector: "an instance value without arguments",
    },
    Plant {
        label: "an assignment names its field as a name",
        site: "(member \"field\" (ctor LexLeanStd.Json::Json.JStr field)",
        replacement: "(member \"name\" (ctor LexLeanStd.Json::Json.JStr field)",
        vector: "a record with type arguments",
    },
    Plant {
        label: "a match branch loses its binders",
        site: "(member \"binders\" (namesJson bound) (member \"body\"",
        replacement: "(member \"binders\" (namesJson (nil Bytes)) (member \"body\"",
        vector: "a match",
    },
    Plant {
        label: "a lambda loses its captures",
        site: "(member \"captures\" (namesJson captures)",
        replacement: "(member \"captures\" (namesJson (nil Bytes))",
        vector: "binders",
    },
    Plant {
        label: "every binder is a natural number",
        site: "(member \"name\" (ctor LexLeanStd.Json::Json.JStr named) (member \"type\" (SemanticTypes::typeJson typed)",
        replacement: "(member \"name\" (ctor LexLeanStd.Json::Json.JStr named) (member \"type\" (SemanticTypes::typeJson (ctor SemanticTypes::SemanticType.SNat))",
        vector: "binders",
    },
    Plant {
        label: "a checked application loses its checks",
        site: "(member \"checks\" (namesJson checks)",
        replacement: "(member \"checks\" (namesJson (nil Bytes))",
        vector: "a checked application",
    },
    Plant {
        label: "an edge points at its source",
        site: "(member \"target\" (termJson target)",
        replacement: "(member \"target\" (termJson source)",
        vector: "a graph literal",
    },
    Plant {
        label: "a map entry's key is its value",
        site: "(member \"key\" (termJson key)",
        replacement: "(member \"key\" (termJson entry)",
        vector: "a map literal",
    },
    Plant {
        label: "every integer is mathematical",
        site: "(member \"representation\" (ctor LexLeanStd.Json::Json.JStr (SemanticTypes::integerName family))",
        replacement: "(member \"representation\" (ctor LexLeanStd.Json::Json.JStr (lit \"int\"))",
        vector: "every literal",
    },
    Plant {
        label: "the branches of an if are swapped",
        site: "(member \"then_value\" (termJson whenTrue) (member \"else_value\" (termJson whenFalse)",
        replacement: "(member \"then_value\" (termJson whenFalse) (member \"else_value\" (termJson whenTrue)",
        vector: "an if",
    },
    Plant {
        label: "linear arithmetic writes no definitions as an empty list",
        site: "(kinded \"linear_arithmetic\" (match definitions\n      | List.nil => (ctor LexLeanStd.Json::Json.JNil)",
        replacement: "(kinded \"linear_arithmetic\" (match definitions\n      | List.nil => (member \"definitions\" (membersJson definitions) (ctor LexLeanStd.Json::Json.JNil))",
        vector: "linear arithmetic without definitions",
    },
    Plant {
        label: "an induction writes no generalized names as an empty list",
        site: "      | List.nil => (member \"branches\" (ctor LexLeanStd.Json::Json.JArr (proofJson branches)) (ctor LexLeanStd.Json::Json.JNil))",
        replacement: "      | List.nil => (member \"generalizing\" (namesJson generalized) (member \"branches\" (ctor LexLeanStd.Json::Json.JArr (proofJson branches)) (ctor LexLeanStd.Json::Json.JNil)))",
        vector: "induction without generalizing",
    },
    Plant {
        label: "a law loses its premises",
        site: "| List.cons head rest => (member \"premises\" (membersJson premises)",
        replacement: "| List.cons head rest => (member \"premises\" (membersJson (nil ((Option Bytes) * Bytes)))",
        vector: "a law with everything",
    },
    Plant {
        label: "a proof branch loses its binders",
        site: "(member \"binders\" (namesJson bound) (member \"proof\"",
        replacement: "(member \"binders\" (namesJson (nil Bytes)) (member \"proof\"",
        vector: "cases",
    },
    Plant {
        label: "a nested member states truth",
        site: "(member \"statement\" (termJson statement)",
        replacement: "(member \"statement\" (termJson (ctor STerm.TUnit))",
        vector: "nested induction",
    },
    Plant {
        label: "every reflection compares by equality",
        site: "(member \"comparison\" (ctor LexLeanStd.Json::Json.JStr comparison)",
        replacement: "(member \"comparison\" (ctor LexLeanStd.Json::Json.JStr (lit \"nat_beq\"))",
        vector: "list reflection",
    },
    Plant {
        label: "a reflected field expects its name",
        site: "(member \"expected\" (ctor LexLeanStd.Json::Json.JStr (second entry))",
        replacement: "(member \"expected\" (ctor LexLeanStd.Json::Json.JStr (first entry))",
        vector: "record reflection",
    },
    Plant {
        label: "an applied theorem is a lemma",
        site: "(kinded \"apply\" (member \"theorem\"",
        replacement: "(kinded \"apply\" (member \"lemma\"",
        vector: "apply a theorem",
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
        let source = planted.read_raw("src/SemanticTerms.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/SemanticTerms.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, root);
        match super::evaluate(&low, &[vector.input.run()], 1).first() {
            Some(first) => eprintln!(
                "BS-69: plant detected: {}: {}",
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
            "BS-69: {}: largest certificate part {largest} bytes",
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

fn split(inputs: &[Input]) -> [Vec<Run>; 2] {
    let mut out = [Vec::new(), Vec::new()];
    for input in inputs {
        out[input.root()].push(input.run());
    }
    out
}

pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("SemanticTerms"),
        "SemanticTerms is a linked module of the project"
    );
    eligibility_of(&project, "SemanticTerms", MODULE, &ROOTS);

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let corpus = committed();
    let (terms, proofs) = split(&corpus).map(|runs| runs.len()).into();
    assert!(
        terms >= 200 && proofs >= 20,
        "the committed semantic modules carry {terms} distinct terms and {proofs} distinct proofs"
    );
    let normative = support::normative_host();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (
            seeded_inputs(EVALUATOR_SEEDED, 0x0B55_0069),
            seeded_inputs(RUSTC_SEEDED, 0x0B55_1069),
        )
    } else {
        (Vec::new(), Vec::new())
    };
    eprintln!(
        "BS-69: {} vectors, {terms} committed terms and {proofs} proofs, {} and {} seeded terms and proofs ({:?})",
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
        for (index, runs) in split(&inputs).iter().enumerate() {
            let root = ROOTS[index].0;
            let disagreements = super::evaluate(&lowered[root], runs, 5);
            assert!(
                disagreements.is_empty(),
                "{root} and the Rust oracle disagree: {disagreements:?}"
            );
            eprintln!(
                "BS-69: {root}: the evaluator agrees on {} runs ({:?})",
                runs.len(),
                started.elapsed()
            );
        }
        plants(&vectors);
    } else {
        eprintln!("BS-69: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled = named_inputs;
    compiled.extend(corpus);
    compiled.extend(rustc_seeded);
    let runs = split(&compiled);
    let differential: Vec<(&str, &super::Lowered, &[Run])> = ROOTS
        .iter()
        .enumerate()
        .map(|(index, (root, _))| (*root, &lowered[root], runs[index].as_slice()))
        .collect();
    let total = rustc_differential("BS-69", &differential, "oracle");
    eprintln!("BS-69: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-69") {
        return;
    }
    export(&lowered);
    eprintln!("BS-69: passed in {:?}", started.elapsed());
}
