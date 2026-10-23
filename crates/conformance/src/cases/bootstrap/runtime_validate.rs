//! `BS-09` (SPEC.md §17.23, §17.16): the runtime syntax validator as
//! language-1.2 source, held to `lexlean::calculus::rust::rt_validate::validate`.
//!
//! The root takes the crate as module data, the constructors of the module's
//! inductives in declaration order. That order is read from the module's own
//! text (`sorts`), so the encoders below name a constructor and never number
//! it, and a constructor added to or moved in the module fails here instead
//! of being encoded as another.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use lexlean::calculus::rust::rt::{
    Arm, Block, Crate, Expr, Function, Generic, Item, Param, Pat, Profile, Sig, Type, Variant,
};
use lexlean::calculus::rust::{rt_decode, rt_validate};
use lexlean::calculus::Value;

use super::rust_validate::{eligibility_of, rustc_differential, value_nodes, verdict};
use super::{list, text, Run, Seeded};
use crate::cases::rust_heap_syntax::probe_plants;
use crate::cases::rust_support::{ast_plants, replace_nth, rules_named, text_plants};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.RuntimeValidate";
const SOURCE: &str = "bootstrap/src/RuntimeValidate.lex.tex";
const SUPPORT: &str = "compiler/src/RuntimeSupport.lex.tex";
const PROBE: &str = "compiler/src/RuntimeProbe.lex.tex";
const ROOTS: [(&str, &[&str]); 1] = [("validateRuntime", &["allocation", "overflow", "recursion"])];

// ---------------------------------------------------------------- the sorts

/// The constructors of each inductive of the module, in declaration order.
fn sorts() -> &'static BTreeMap<String, Vec<String>> {
    static SORTS: OnceLock<BTreeMap<String, Vec<String>>> = OnceLock::new();
    SORTS.get_or_init(|| {
        let source =
            std::fs::read_to_string(repo_root().join(SOURCE).as_std_path()).expect("the module");
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
            let constructors: Vec<String> = block
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
    })
}

fn index(sort: &str, constructor: &str) -> u64 {
    let found = sorts()
        .get(sort)
        .unwrap_or_else(|| panic!("the module has no sort `{sort}`"))
        .iter()
        .position(|name| name == constructor)
        .unwrap_or_else(|| panic!("`{sort}` has no constructor `{constructor}`"));
    u64::try_from(found).expect("an index")
}

fn adt(sort: &str, constructor: &str, fields: Vec<Value>) -> Value {
    Value::Adt {
        constructor: index(sort, constructor),
        fields,
    }
}

/// A record: a structure is the one constructor of its own sort.
fn record(fields: Vec<Value>) -> Value {
    Value::Adt {
        constructor: 0,
        fields,
    }
}

fn nat(number: u64) -> Value {
    Value::Nat {
        value: number.to_string(),
    }
}

fn boolean(value: bool) -> Value {
    Value::Bool { value }
}

fn texts(items: &[String]) -> Value {
    list(items.iter().map(|item| text(item)).collect())
}

/// A list of a recursive sort as a chain of its own `Nil` and `Cons`.
fn chain(sort: &str, nil: &str, cons: &str, items: Vec<Value>) -> Value {
    items
        .into_iter()
        .rev()
        .fold(adt(sort, nil, Vec::new()), |tail, head| {
            adt(sort, cons, vec![head, tail])
        })
}

// ----------------------------------------------------------------- encoders

fn enc_types(types: &[Type]) -> Value {
    chain(
        "Ty",
        "TyNil",
        "TyCons",
        types.iter().map(enc_type).collect(),
    )
}

fn enc_type(ty: &Type) -> Value {
    let plain = |name: &str| adt("Ty", name, Vec::new());
    match ty {
        Type::Unit => plain("TyUnit"),
        Type::Bool => plain("TyBool"),
        Type::Nat => plain("TyNat"),
        Type::Fixed(_) => plain("TyFixed"),
        Type::Ordering => plain("TyOrdering"),
        Type::SelfType => plain("TySelf"),
        Type::Param(at) => adt("Ty", "TyParam", vec![nat(*at)]),
        Type::Ref(inner) => adt("Ty", "TyRef", vec![enc_type(inner)]),
        Type::RefMut(inner) => adt("Ty", "TyRefMut", vec![enc_type(inner)]),
        Type::Pair(a, b) => adt("Ty", "TyPair", vec![enc_type(a), enc_type(b)]),
        Type::Named(name, arguments) => {
            adt("Ty", "TyNamed", vec![text(name), enc_types(arguments)])
        }
        Type::Wide(_) => plain("TyWide"),
        Type::Size => plain("TySize"),
        Type::Char => plain("TyChar"),
        Type::Str => plain("TyStr"),
        Type::Slice(inner) => adt("Ty", "TySlice", vec![enc_type(inner)]),
    }
}

fn enc_optional_type(ty: &Option<Type>) -> Value {
    match ty {
        Some(ty) => enc_type(ty),
        None => adt("Ty", "TyAbsent", Vec::new()),
    }
}

fn enc_pats(pats: &[Pat]) -> Value {
    chain("Pat", "PNil", "PCons", pats.iter().map(enc_pat).collect())
}

fn enc_pat(pat: &Pat) -> Value {
    match pat {
        Pat::Wild => adt("Pat", "PWild", Vec::new()),
        Pat::Bind(name) => adt("Pat", "PBind", vec![text(name)]),
        Pat::BindMut(name) => adt("Pat", "PBindMut", vec![text(name)]),
        Pat::Path(path) => adt("Pat", "PPath", vec![texts(path)]),
        Pat::Tuple(items) => adt("Pat", "PTuple", vec![enc_pats(items)]),
        Pat::TupleStruct(path, items) => {
            adt("Pat", "PTupleStruct", vec![texts(path), enc_pats(items)])
        }
        Pat::Struct(path, names, items) => adt(
            "Pat",
            "PStruct",
            vec![texts(path), texts(names), enc_pats(items)],
        ),
        Pat::Lit(_) => adt("Pat", "PLit", Vec::new()),
    }
}

fn exprs(items: &[Expr]) -> Value {
    chain(
        "Expr",
        "ENil",
        "ECons",
        items.iter().map(enc_expr).collect(),
    )
}

fn none_expr() -> Value {
    adt("Expr", "ENone", Vec::new())
}

fn optional_expr(expr: &Option<Box<Expr>>) -> Value {
    expr.as_ref()
        .map_or_else(none_expr, |inner| enc_expr(inner))
}

/// A block: its statements as a chain in which a `let` is a statement that
/// binds for the rest, and its tail.
fn enc_block(block: &Block) -> Value {
    let statements =
        block
            .statements
            .iter()
            .rev()
            .fold(
                adt("Expr", "ENil", Vec::new()),
                |rest, statement| match statement {
                    Expr::Let(pat, ty, value) => adt(
                        "Expr",
                        "ELetS",
                        vec![enc_pat(pat), enc_optional_type(ty), enc_expr(value), rest],
                    ),
                    other => adt("Expr", "ECons", vec![enc_expr(other), rest]),
                },
            );
    adt("Expr", "EBlock", vec![statements, enc_expr(&block.tail)])
}

fn enc_arms(arms: &[Arm]) -> Value {
    arms.iter()
        .rev()
        .fold(adt("Expr", "ENil", Vec::new()), |rest, arm| {
            adt(
                "Expr",
                "EArm",
                vec![
                    enc_pat(&arm.pattern),
                    arm.guard.as_ref().map_or_else(none_expr, enc_expr),
                    enc_expr(&arm.body),
                    rest,
                ],
            )
        })
}

fn enc_expr(expr: &Expr) -> Value {
    let e = |name: &str, fields: Vec<Value>| adt("Expr", name, fields);
    match expr {
        Expr::Var(name) => e("EVar", vec![text(name)]),
        Expr::Lit(_) => e("ELit", Vec::new()),
        Expr::Path(path) => e("EPath", vec![texts(path)]),
        Expr::Call(path, arguments) => e("ECall", vec![texts(path), exprs(arguments)]),
        Expr::Method(receiver, name, arguments) => e(
            "EMethod",
            vec![enc_expr(receiver), text(name), exprs(arguments)],
        ),
        Expr::Field(inner, at) => e("EField", vec![enc_expr(inner), nat(*at)]),
        Expr::FieldNamed(inner, name) => e("EFieldNamed", vec![enc_expr(inner), text(name)]),
        Expr::Borrow(inner) => e("EBorrow", vec![enc_expr(inner)]),
        Expr::Equals(a, b) => e("EEquals", vec![enc_expr(a), enc_expr(b)]),
        Expr::Match(scrutinee, arms) => e("EMatch", vec![enc_expr(scrutinee), enc_arms(arms)]),
        Expr::Let(pat, ty, value) => e(
            "ELet",
            vec![enc_pat(pat), enc_optional_type(ty), enc_expr(value)],
        ),
        Expr::Assign(place, value) => e("EAssign", vec![enc_expr(place), enc_expr(value)]),
        Expr::If(condition, then, otherwise) => e(
            "EIf",
            vec![enc_expr(condition), enc_block(then), enc_block(otherwise)],
        ),
        Expr::While(condition, body) => e("EWhile", vec![enc_expr(condition), enc_block(body)]),
        Expr::WhileLet(pat, value, body) => e(
            "EWhileLet",
            vec![enc_pat(pat), enc_expr(value), enc_block(body)],
        ),
        Expr::Loop(body) => e("ELoop", vec![enc_block(body)]),
        Expr::For(pat, value, body) => {
            e("EFor", vec![enc_pat(pat), enc_expr(value), enc_block(body)])
        }
        Expr::Break => e("EBreak", Vec::new()),
        Expr::Return(value) => e("EReturn", vec![optional_expr(value)]),
        Expr::Tuple(items) => e("ETuple", vec![exprs(items)]),
        Expr::StructLit(path, names, values) => {
            e("EStructLit", vec![texts(path), texts(names), exprs(values)])
        }
        Expr::Binary(_, a, b) => e("EBinary", vec![enc_expr(a), enc_expr(b)]),
        Expr::Not(inner) => e("ENot", vec![enc_expr(inner)]),
        Expr::Cast(inner, ty) => e("ECast", vec![enc_expr(inner), enc_type(ty)]),
        Expr::Index(a, b) => e("EIndex", vec![enc_expr(a), enc_expr(b)]),
        Expr::RangeSlice(inner, lower, upper) => e(
            "ERange",
            vec![enc_expr(inner), optional_expr(lower), optional_expr(upper)],
        ),
        Expr::Deref(inner) => e("EDeref", vec![enc_expr(inner)]),
        Expr::BorrowMut(inner) => e("EBorrowMut", vec![enc_expr(inner)]),
        Expr::MethodOf(receiver, name, types, arguments) => e(
            "EMethodOf",
            vec![
                enc_expr(receiver),
                text(name),
                enc_types(types),
                exprs(arguments),
            ],
        ),
        Expr::Block(block) => enc_block(block),
    }
}

fn enc_generics(generics: &[Generic]) -> Value {
    list(
        generics
            .iter()
            .map(|generic| record(vec![text(&generic.name), texts(&generic.bounds)]))
            .collect(),
    )
}

fn enc_param(param: &Param) -> Value {
    record(vec![
        text(&param.name),
        boolean(param.mutable),
        enc_type(&param.ty),
    ])
}

fn enc_sig(sig: &Sig) -> Value {
    record(vec![
        text(&sig.name),
        boolean(sig.receiver),
        boolean(sig.exclusive),
        list(sig.params.iter().map(enc_param).collect()),
        enc_type(&sig.result),
    ])
}

fn enc_function(function: &Function) -> Value {
    record(vec![
        enc_sig(&function.sig),
        enc_generics(&function.generics),
        boolean(function.public),
        enc_block(&function.body),
    ])
}

fn enc_variant(variant: &Variant) -> Value {
    match variant {
        Variant::Unit(name) => adt("Variant", "VUnit", vec![text(name)]),
        Variant::Tuple(name, types) => adt("Variant", "VTuple", vec![text(name), enc_types(types)]),
        Variant::Record(name, fields, types) => adt(
            "Variant",
            "VRecord",
            vec![text(name), texts(fields), enc_types(types)],
        ),
    }
}

fn enc_item(item: &Item) -> Value {
    let i = |name: &str, fields: Vec<Value>| adt("Item", name, fields);
    match item {
        Item::Use(path, alias) => i(
            "IUse",
            vec![
                texts(path),
                texts(&alias.iter().cloned().collect::<Vec<_>>()),
            ],
        ),
        Item::UnitStruct(name, _) => i("IUnit", vec![text(name)]),
        Item::Alias(name, parameters, ty) => {
            i("IAlias", vec![text(name), texts(parameters), enc_type(ty)])
        }
        Item::Static(name, ty, init) => {
            i("IStatic", vec![text(name), enc_type(ty), enc_expr(init)])
        }
        Item::Trait(name, sigs) => i(
            "ITrait",
            vec![text(name), list(sigs.iter().map(enc_sig).collect())],
        ),
        Item::Impl(generics, tr, ty, functions) => i(
            "IImpl",
            vec![
                enc_generics(generics),
                texts(&tr.iter().cloned().collect::<Vec<_>>()),
                enc_type(ty),
                list(functions.iter().map(enc_function).collect()),
            ],
        ),
        Item::Fn(function) => i("IFn", vec![enc_function(function)]),
        Item::TupleStruct(name, generics, _, types) => i(
            "ITuple",
            vec![text(name), texts(generics), enc_types(types)],
        ),
        Item::RecordStruct(name, generics, _, fields, types) => i(
            "IRecord",
            vec![text(name), texts(generics), texts(fields), enc_types(types)],
        ),
        Item::Enum(name, generics, _, variants) => i(
            "IEnum",
            vec![
                text(name),
                texts(generics),
                list(variants.iter().map(enc_variant).collect()),
            ],
        ),
    }
}

fn enc_crate(krate: &Crate) -> Value {
    record(vec![
        boolean(krate.profile == Profile::Core),
        list(krate.items.iter().map(enc_item).collect()),
    ])
}

/// A run of `validateRuntime`: the crate and what the Rust validator makes
/// of it.
fn run_of(krate: &Crate) -> Run {
    (
        vec![enc_crate(krate)],
        verdict(rt_validate::validate(krate)),
    )
}

// ------------------------------------------------------------------- corpus

fn read(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path).as_std_path())
        .unwrap_or_else(|error| panic!("{path}: {error}"))
}

/// A named crate for each refusal and acceptance the runtime's own cases
/// state: the two shipped crates, the profiles they are valid in, and every
/// plant of `RB-17` and `RB-19`.
fn named() -> Vec<(String, Crate)> {
    let support = read(SUPPORT);
    let probe = read(PROBE);
    let base = rt_decode::decode(&support).expect("the support crate decodes");
    let probed = rt_decode::decode_named(&probe, "probeCrate").expect("the probe decodes");
    let mut out: Vec<(String, Crate)> = vec![
        ("the support crate".to_owned(), base.clone()),
        ("the probe crate".to_owned(), probed.clone()),
    ];
    let mut std_support = base.clone();
    std_support.profile = Profile::Std;
    out.push(("the support crate as std".to_owned(), std_support));
    let mut core_probe = probed.clone();
    core_probe.profile = Profile::Core;
    out.push(("the probe crate as core".to_owned(), core_probe));
    for (what, from, to, nth, _) in text_plants() {
        let changed = replace_nth(&support, from, to, nth);
        out.push((
            format!("support: {what}"),
            rt_decode::decode(&changed).unwrap_or_else(|error| panic!("{what}: {error}")),
        ));
    }
    for (what, mutate, _) in ast_plants() {
        let mut planted = base.clone();
        mutate(&mut planted);
        out.push((format!("support: {what}"), planted));
    }
    for (what, planted, _) in probe_plants(&probed) {
        out.push((format!("probe: {what}"), planted));
    }
    out
}

/// The sources of the seeded mutants: a quoted string, a constructor of the
/// same enum, a mutable flag, a visibility, or a type parameter of a shipped
/// module, replaced by another of its kind. A mutant the decoder refuses is
/// drawn again.
fn mutants(count: usize, seeded: &mut Seeded) -> Vec<Crate> {
    let modules = [(read(SUPPORT), "supportCrate"), (read(PROBE), "probeCrate")];
    let strings: Vec<Vec<String>> = modules
        .iter()
        .map(|(source, _)| {
            let mut found: Vec<String> = source
                .split('"')
                .enumerate()
                .filter(|(at, piece)| {
                    at % 2 == 1
                        && !piece.is_empty()
                        && piece.chars().all(|c| c.is_alphanumeric() || c == '_')
                })
                .map(|(_, piece)| piece.to_owned())
                .collect();
            found.sort();
            found.dedup();
            found
        })
        .collect();
    let mut out = Vec::new();
    let mut tries = 0;
    while out.len() < count {
        tries += 1;
        assert!(tries < count * 400, "the mutants decode");
        let which = seeded.below(modules.len());
        let (source, name) = &modules[which];
        let edited = match seeded.below(4) {
            0 => {
                // another string where a string stood
                let pieces = &strings[which];
                let from = seeded.pick(pieces).clone();
                let to = seeded.pick(pieces).clone();
                if from == to {
                    continue;
                }
                let needle = format!("\"{from}\"");
                let hits = source.matches(&needle).count();
                if hits == 0 {
                    continue;
                }
                replace_nth(source, &needle, &format!("\"{to}\""), seeded.below(hits))
            }
            1 => {
                // another constructor of the same enum where one stood
                let sorts: Vec<&str> = vec![
                    "RtExpr",
                    "RtPat",
                    "RtType",
                    "RtOp",
                    "RtLit",
                    "RtItem",
                    "RtVariant",
                ];
                let sort = *seeded.pick(&sorts);
                let prefix = format!("{sort}.");
                let names: Vec<String> = {
                    let mut found: Vec<String> = source
                        .split(&prefix)
                        .skip(1)
                        .map(|rest| {
                            rest.chars()
                                .take_while(|c| c.is_alphanumeric())
                                .collect::<String>()
                        })
                        .collect();
                    found.sort();
                    found.dedup();
                    found
                };
                if names.len() < 2 {
                    continue;
                }
                let from = seeded.pick(&names).clone();
                let to = seeded.pick(&names).clone();
                if from == to {
                    continue;
                }
                let needle = format!("{prefix}{from} ");
                let hits = source.matches(&needle).count();
                if hits == 0 {
                    continue;
                }
                replace_nth(
                    source,
                    &needle,
                    &format!("{prefix}{to} "),
                    seeded.below(hits),
                )
            }
            2 => {
                let needle = if seeded.chance(50) {
                    "mutable := false"
                } else {
                    "public := true"
                };
                let hits = source.matches(needle).count();
                if hits == 0 {
                    continue;
                }
                let to = if needle == "mutable := false" {
                    "mutable := true"
                } else {
                    "public := false"
                };
                replace_nth(source, needle, to, seeded.below(hits))
            }
            _ => {
                let needle = if seeded.chance(50) {
                    "RtType.param 1"
                } else {
                    "RtType.param 0"
                };
                let hits = source.matches(needle).count();
                if hits == 0 {
                    continue;
                }
                replace_nth(
                    source,
                    needle,
                    &format!("RtType.param {}", 2 + seeded.below(5)),
                    seeded.below(hits),
                )
            }
        };
        if &edited == source {
            continue;
        }
        if let Ok(krate) = rt_decode::decode_named(&edited, name) {
            out.push(krate);
        }
    }
    out
}

// ------------------------------------------------------------------- plants

/// A defect planted in the source: the check it removes, the text it is
/// made at, and the corpus the differential must see it on.
const PLANTS: [super::Plant; 8] = [
    // A name declared twice is accepted.
    super::Plant {
        label: "duplicate names accepted",
        site: "(if (inList (project sc declared) name) then",
        replacement: "(if false then",
        root: "validateRuntime",
    },
    // A tuple of one is accepted.
    super::Plant {
        label: "one-element tuple accepted",
        site: "((exprLen items) <? 2)",
        replacement: "false",
        root: "validateRuntime",
    },
    // A `break` outside a loop is accepted.
    super::Plant {
        label: "break outside a loop accepted",
        site: "(if (project fr inLoop) then (ctor Option.none [Bytes]) else (ctor Option.some [Bytes] (rcat (rtext \"runtime support: break-position",
        replacement: "(if true then (ctor Option.none [Bytes]) else (ctor Option.some [Bytes] (rcat (rtext \"runtime support: break-position",
        root: "validateRuntime",
    },
    // A free function that is not public is accepted.
    super::Plant {
        label: "private free function accepted",
        site: "(if (not (project f public)) then",
        replacement: "(if false then",
        root: "validateRuntime",
    },
    // A `std` use in the core profile is accepted.
    super::Plant {
        label: "std use accepted in core",
        site: "(if (project sc core) then false else (bEq root (rtext \"std\")))",
        replacement: "true",
        root: "validateRuntime",
    },
    // An assignment to an immutable binding is accepted.
    super::Plant {
        label: "assignment to an immutable name accepted",
        site: "(if (project held mutable) then",
        replacement: "(if true then",
        root: "validateRuntime",
    },
    // A match with no arm is accepted.
    super::Plant {
        label: "match with no arm accepted",
        site: "(if (isEmptyChain arms) then",
        replacement: "(if false then",
        root: "validateRuntime",
    },
    // A heap type in the core profile is accepted.
    super::Plant {
        label: "heap type accepted in core",
        site: "(if (if (project sc core) then (isTableHeap sc name) else false) then",
        replacement: "(if false then",
        root: "validateRuntime",
    },
];

/// Crates the evaluator is given in full: those whose encoding has at most
/// this many nodes. The shipped crates are held to the oracle by the
/// compiled differential.
const EVALUATOR_NODES: usize = 4_000;
const MUTANTS: usize = 600;

pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("RuntimeValidate")
            && checked_units.units.contains_key("RustValidate"),
        "RuntimeValidate and RustValidate are linked modules of the project"
    );
    eligibility_of(&project, "RuntimeValidate", MODULE, &ROOTS);

    // The constructors the encoders name are the module's own, in its order.
    assert_eq!(
        sorts().keys().map(String::as_str).collect::<Vec<_>>(),
        ["Expr", "Item", "Pat", "Ty", "Variant"],
        "the sorts of the module"
    );

    let named = named();
    let mut seeded = Seeded(0x0B55_0009);
    let drawn = mutants(MUTANTS, &mut seeded);
    let vectors: Vec<Run> = named
        .iter()
        .map(|(_, krate)| run_of(krate))
        .chain(drawn.iter().map(run_of))
        .collect();
    let refused = vectors
        .iter()
        .filter(|run| run.1["kind"] == "error")
        .count();
    eprintln!(
        "BS-09: {} named crates, {} mutants, {} refused ({:?})",
        named.len(),
        drawn.len(),
        refused,
        started.elapsed()
    );
    assert_eq!(
        (named.len(), drawn.len(), vectors.len(), refused),
        (NAMED, MUTANTS, NAMED + MUTANTS, REFUSED),
        "the named crates, the seeded mutants, the vectors, and the refused among them"
    );
    let rules: std::collections::BTreeSet<String> = vectors
        .iter()
        .filter_map(|run| {
            let reason = run.1["value"]["hex"].as_str()?;
            let bytes: Vec<u8> = (0..reason.len())
                .step_by(2)
                .map(|at| u8::from_str_radix(&reason[at..at + 2], 16).expect("hex"))
                .collect();
            let message = String::from_utf8(bytes).ok()?;
            let rest = message.strip_prefix("runtime support: ")?;
            Some(rest.split(':').next()?.to_owned())
        })
        .collect();
    assert_eq!(
        rules,
        rules_named(&read("crates/lexlean/src/calculus/rust/rt_validate.rs")),
        "every rule of the validator is reached by a vector, and no other refusal occurs"
    );

    let checked = support::checked_project(&project);
    let root = super::lower(&checked, &project, MODULE, "validateRuntime");
    let normative = support::normative_host();
    if !normative {
        eprintln!(
            "BS-09: platform-bound host: the evaluator and the planted defects run on the normative host ({:?})",
            started.elapsed()
        );
    }
    if normative {
        let within: Vec<Run> = vectors
            .iter()
            .filter(|run| run.0.iter().map(value_nodes).sum::<usize>() <= EVALUATOR_NODES)
            .cloned()
            .collect();
        assert_eq!(within.len(), WITHIN, "the vectors the evaluator is given");
        let disagreements = super::evaluate(&root, &within, 5);
        assert!(
            disagreements.is_empty(),
            "the evaluator and the Rust validator disagree: {disagreements:?}"
        );
        eprintln!(
            "BS-09: the evaluator agrees on {} vectors ({:?})",
            within.len(),
            started.elapsed()
        );
        // The gate is armed: each defect planted in a copy of the source is a
        // disagreement with the oracle.
        for plant in &PLANTS {
            let planted = support::P::bootstrap();
            let source = planted.read_raw("src/RuntimeValidate.lex.tex");
            assert_eq!(
                source.matches(plant.site).count(),
                1,
                "{}: the plant site occurs once",
                plant.label
            );
            planted.write(
                "src/RuntimeValidate.lex.tex",
                &source.replacen(plant.site, plant.replacement, 1),
            );
            let checked = support::checked_project(&planted);
            let low = super::lower(&checked, &planted, MODULE, plant.root);
            let disagreements = super::evaluate(&low, &within, 1);
            assert!(
                !disagreements.is_empty(),
                "{}: the planted defect is detected by the differential",
                plant.label
            );
            eprintln!(
                "BS-09: plant detected: {}: {}",
                plant.label,
                &disagreements[0][..disagreements[0].len().min(120)]
            );
        }
    }
    // On every host: the rendered rust-std package of the root, compiled by
    // that host's pinned rustc, agrees with the oracle on the vectors.
    rustc_differential("BS-09", &[("validateRuntime", &root, &vectors)], "vectors");
    if !normative {
        return;
    }
    // Under the pinned Lean, the production export certifies the root; its
    // packages are the committed ones and realize the program the
    // differential ran.
    if !support::lean_backed("BS-09") {
        return;
    }
    let exported = harness::exported("bootstrap");
    let bundle = exported.bundle();
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
    let mut realized = 0usize;
    for exported_root in &exported.output.result.roots {
        let Some(member) = exported_root.root.strip_prefix(&format!("{MODULE}.")) else {
            continue;
        };
        assert_eq!(member, "validateRuntime");
        for output in &exported_root.outputs {
            let package: serde_json::Value = serde_json::from_slice(
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
                serde_json::to_value(&root.program).expect("a program is JSON"),
                "{member}: the exported package realizes the program the differential ran"
            );
            realized += 1;
        }
    }
    assert_eq!(realized, ROOTS.len(), "one certified package per root");
    eprintln!("BS-09: passed in {:?}", started.elapsed());
}

// The counts the specification states, over the committed tree: SPEC.md
// §17.23, the register statement of BS-09, its scenario, and VERIFICATION.md
// restate them, and the case fails when the tree no longer gives them.
const NAMED: usize = 48;
const REFUSED: usize = 495;
const WITHIN: usize = 648;
