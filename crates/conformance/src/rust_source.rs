//! The declared Rust machine (SPEC.md §17.17), as the `compiler` project
//! states it to Lean: the closed Rust AST of the renderer (`RustSyntax`) and
//! its evaluator (`RustSemantics`).
//!
//! `RustSyntax` mirrors `lexlean::calculus::rust::ast` construct for
//! construct, without origins, which carry no meaning. `RustSemantics`
//! evaluates a crate over the calculus's own values: a `u64` is a `nat`
//! below `2^64`, an `i64` an `int` in its range, a fixed width its scalar,
//! `Str`, `Bytes`, and `List` the calculus's string, bytes, and list, an
//! `Rc` or a box the value it holds, and a closure of the defunctionalized
//! enum the calculus closure of the same function and captures. A `?` on an
//! `Err` raises out of its function, which returns `Err(Overflow)` exactly
//! when its result type is fallible. The machine aborts where a runtime item
//! that cannot fail meets an overflow of its primitive: the length of a
//! sequence of `2^64` or more elements, which no machine holds. Every runtime item is declared by the
//! calculus primitive it realizes, read on the representation's values:
//! the item is no more than that primitive at its width, fallible exactly
//! when the primitive can overflow, and present in a profile exactly when
//! it needs no heap or the profile is `rust-std`. That rustc agrees with
//! this declaration is measured by the conformance differential, never
//! assumed by a proof.
//!
//! Like the calculus, the machine is read here, as the terms that build it;
//! the committed modules are exactly what [`files`] renders, which `cargo
//! xtask check-calculus` enforces.

use std::cell::Cell;
use std::collections::BTreeMap;

use lexlean::calculus::term::{self, SEMANTICS, SYNTAX};
use serde_json::Value as Json;

use crate::calculus_source::{value_constructors, Constructors, FIXED};
use crate::lx::{
    self, and, axioms, beq, bool_t, cons, definition, inductive, list_t, local_t, matching, member,
    module_tex, mutual, nat, nat_t, nil, none, not, option_t, parameter, prim, product_t, project,
    recursive, scalar_t, some, string_t, structure, value, var,
};

/// The module of the closed Rust AST.
pub const RUST_SYNTAX: &str = "RustSyntax";
/// The module of the declared Rust machine.
pub const RUST_SEMANTICS: &str = "RustSemantics";

/// All three of Lean's standard axioms.
const CLASSICAL: &[&str] = &["Classical.choice", "Quot.sound", "propext"];

/// The kinds of generated identifier, in `ast::Ident`'s order (`binding`
/// is `Local`, a name Lean reserves).
pub const IDENT_KINDS: [&str; 9] = [
    "binding", "holder", "operand", "callee", "boxed", "part", "capture", "param", "function",
];

/// The runtime items without a width, in `runtime::Item`'s order, each with
/// the calculus primitive it realizes (`natSucc` realizes `natAdd` with a
/// second operand of one), whether it can fail, and whether it needs the
/// heap.
pub const ITEMS: [(&str, &str, bool, bool); 42] = [
    ("natAdd", "natAdd", true, false),
    ("natSub", "natSub", false, false),
    ("natMul", "natMul", true, false),
    ("natQuot", "natQuot", false, false),
    ("natRem", "natRem", false, false),
    ("natEq", "natEq", false, false),
    ("natLe", "natLe", false, false),
    ("natLt", "natLt", false, false),
    ("natSucc", "natAdd", true, false),
    ("intAdd", "intAdd", true, false),
    ("intSub", "intSub", true, false),
    ("intMul", "intMul", true, false),
    ("intNeg", "intNeg", true, false),
    ("intQuot", "intQuot", true, false),
    ("intRem", "intRem", false, false),
    ("boolNot", "boolNot", false, false),
    ("boolAnd", "boolAnd", false, false),
    ("boolOr", "boolOr", false, false),
    ("equal", "equal", false, false),
    ("compare", "compare", false, false),
    ("checkedAdd", "checkedAdd", false, false),
    ("checkedSub", "checkedSub", false, false),
    ("checkedMul", "checkedMul", false, false),
    ("checkedQuot", "checkedQuot", false, false),
    ("checkedNeg", "checkedNeg", false, false),
    ("bitAnd", "bitAnd", false, false),
    ("bitOr", "bitOr", false, false),
    ("bitXor", "bitXor", false, false),
    ("bitNot", "bitNot", false, false),
    ("shiftLeft", "shiftLeft", false, false),
    ("shiftRight", "shiftRight", false, false),
    ("appendList", "append", false, true),
    ("appendBytes", "append", false, true),
    ("lengthList", "length", false, true),
    ("lengthBytes", "length", false, true),
    ("lengthString", "length", false, true),
    ("indexList", "index", false, true),
    ("indexBytes", "index", false, true),
    ("sliceList", "slice", false, true),
    ("sliceBytes", "slice", false, true),
    ("utf8Encode", "utf8Encode", false, true),
    ("utf8Decode", "utf8Decode", false, true),
];

/// The runtime items after [`ITEMS`] in `runtime::Item`'s order, each
/// with its primitive, whether it can fail, and whether it needs the heap.
pub const MORE_ITEMS: [(&str, &str, bool, bool); 4] = [
    ("compareBytes", "compareBytes", false, true),
    ("splitExact", "splitExact", false, true),
    ("join", "join", false, true),
    ("formatInt", "formatDecimal", false, true),
];

/// The items that carry a width, applied to operands of that width.
pub const WIDTH_ITEMS: [&str; 11] = [
    "checkedAdd",
    "checkedSub",
    "checkedMul",
    "checkedQuot",
    "checkedNeg",
    "bitAnd",
    "bitOr",
    "bitXor",
    "bitNot",
    "shiftLeft",
    "shiftRight",
];

// --- references -------------------------------------------------------------

fn syntax_t(name: &str) -> Json {
    term::named(RUST_SYNTAX, name)
}
fn target_t(name: &str) -> Json {
    term::named(SYNTAX, name)
}
fn value_t() -> Json {
    target_t("Value")
}
fn values_t() -> Json {
    list_t(value_t())
}
fn ident_t() -> Json {
    syntax_t("Ident")
}
/// An environment: each binding's identifier with its value, innermost
/// first.
fn environment_t() -> Json {
    list_t(product_t(ident_t(), value_t()))
}
fn outcome_t() -> Json {
    local_t("ROutcome")
}
fn crate_t() -> Json {
    syntax_t("Crate")
}

fn owned(binders: &[&str]) -> Vec<String> {
    binders.iter().map(|binder| (*binder).to_owned()).collect()
}

/// A call of a member of the module being generated.
fn call(function: &str, arguments: Vec<Json>) -> Json {
    lx::call(member(function), arguments)
}
/// A constructor of the module being generated, or a builtin.
fn ctor(constructor: &str, arguments: Vec<Json>) -> Json {
    lx::construct(member(constructor), arguments)
}
fn syntax_ctor(constructor: &str, arguments: Vec<Json>) -> Json {
    lx::construct(term::member(RUST_SYNTAX, constructor), arguments)
}
fn branch(constructor: &str, binders: &[&str], body: Json) -> Json {
    lx::branch(member(constructor), owned(binders), body)
}
fn syntax_branch(constructor: &str, binders: &[&str], body: Json) -> Json {
    lx::branch(term::member(RUST_SYNTAX, constructor), owned(binders), body)
}
fn target_branch(constructor: &str, binders: &[&str], body: Json) -> Json {
    lx::branch(term::member(SYNTAX, constructor), owned(binders), body)
}

fn outcome(constructor: &str, arguments: Vec<Json>) -> Json {
    ctor(&format!("ROutcome.{constructor}"), arguments)
}
fn outcomes(constructor: &str, arguments: Vec<Json>) -> Json {
    ctor(&format!("ROutcomes.{constructor}"), arguments)
}
fn stuck() -> Json {
    outcome("stuck", Vec::new())
}
fn produced(result: Json) -> Json {
    outcome("value", vec![result])
}

fn option_match(scrutinee: Json, absent: Json, binder: &str, present: Json) -> Json {
    matching(
        scrutinee,
        vec![
            branch("Option.none", &[], absent),
            branch("Option.some", &[binder], present),
        ],
    )
}

fn list_match(scrutinee: Json, empty: Json, head: &str, tail: &str, nonempty: Json) -> Json {
    matching(
        scrutinee,
        vec![
            branch("List.nil", &[], empty),
            branch("List.cons", &[head, tail], nonempty),
        ],
    )
}

fn bool_match(scrutinee: Json, yes: Json, no: Json) -> Json {
    lx::ite(scrutinee, yes, no)
}

// --- RustSyntax --------------------------------------------------------------

fn nullary(names: &[&'static str]) -> Constructors {
    names.iter().map(|name| (*name, Vec::new())).collect()
}

fn int_kind_t() -> Json {
    target_t("IntKind")
}

fn ident_constructors() -> Constructors {
    vec![
        ("generated", vec![local_t("IdentKind"), nat_t()]),
        ("exported", vec![string_t()]),
    ]
}

fn type_constructors() -> Constructors {
    let ty = || local_t("RustType");
    vec![
        ("unit", vec![]),
        ("bool", vec![]),
        ("nat", vec![]),
        ("int", vec![]),
        ("fixed", vec![int_kind_t()]),
        ("ordering", vec![]),
        ("str", vec![]),
        ("bytes", vec![]),
        ("option", vec![ty()]),
        ("result", vec![ty(), ty()]),
        ("list", vec![ty()]),
        ("pair", vec![ty(), ty()]),
        ("adt", vec![nat_t()]),
        ("fn", vec![nat_t()]),
        ("rc", vec![ty()]),
        ("fallible", vec![ty()]),
        ("ref", vec![ty()]),
    ]
}

fn lit_constructors() -> Constructors {
    vec![
        ("unit", vec![]),
        ("bool", vec![bool_t()]),
        ("nat", vec![nat_t()]),
        ("int", vec![lx::int_t()]),
        ("fixed", vec![int_kind_t(), lx::int_t()]),
        ("str", vec![string_t()]),
        ("bytes", vec![scalar_t("bytes")]),
        ("ordering", vec![target_t("Order")]),
    ]
}

fn ctor_constructors() -> Constructors {
    let ty = || local_t("RustType");
    vec![
        ("none", vec![ty()]),
        ("some", vec![]),
        ("ok", vec![ty(), ty()]),
        ("err", vec![ty(), ty()]),
        ("adt", vec![nat_t(), nat_t()]),
        ("closure", vec![nat_t(), nat_t()]),
        ("cons", vec![]),
        ("nil", vec![ty()]),
    ]
}

fn pat_constructors() -> Constructors {
    let pat = || local_t("Pat");
    vec![
        ("wild", vec![]),
        ("bind", vec![local_t("Ident")]),
        ("unit", vec![]),
        ("tuple", vec![list_t(pat())]),
        ("none", vec![]),
        ("some", vec![pat()]),
        ("ok", vec![pat()]),
        ("err", vec![pat()]),
        ("ordering", vec![target_t("Order")]),
        ("adt", vec![nat_t(), nat_t(), list_t(pat())]),
    ]
}

/// The runtime items, in `runtime::Item`'s order.
pub fn item_constructors() -> Constructors {
    let mut out: Constructors = Vec::new();
    for (name, _, _, _) in ITEMS.iter().take(20) {
        out.push((name, Vec::new()));
    }
    for name in WIDTH_ITEMS {
        out.push((name, vec![int_kind_t()]));
    }
    out.push(("convert", vec![int_kind_t()]));
    for (name, _, _, _) in ITEMS.iter().skip(31) {
        out.push((name, Vec::new()));
    }
    for (name, _, _, _) in MORE_ITEMS {
        out.push((name, Vec::new()));
    }
    out.push(("formatFixed", vec![int_kind_t()]));
    out.push(("parseInt", Vec::new()));
    out.push(("parseFixed", vec![int_kind_t()]));
    out
}

fn callee_constructors() -> Constructors {
    vec![
        ("function", vec![nat_t()]),
        ("runtime", vec![local_t("Item")]),
    ]
}

fn expr_constructors() -> Constructors {
    let expr = || local_t("Expr");
    let exprs = || list_t(local_t("Expr"));
    let ident = || local_t("Ident");
    vec![
        ("lit", vec![local_t("Lit")]),
        ("move", vec![ident()]),
        ("clone", vec![ident()]),
        ("copy", vec![ident()]),
        ("deref", vec![ident()]),
        ("not", vec![expr()]),
        ("unbox", vec![ident()]),
        ("box", vec![expr()]),
        ("call", vec![local_t("Callee"), exprs(), bool_t()]),
        ("apply", vec![ident(), exprs(), bool_t()]),
        ("construct", vec![local_t("Ctor"), exprs()]),
        ("pair", vec![expr(), expr()]),
        ("cond", vec![expr(), local_t("Block"), local_t("Block")]),
        ("matchOn", vec![expr(), list_t(local_t("Arm"))]),
        ("block", vec![local_t("Block")]),
        ("uncons", vec![ident()]),
        ("isZero", vec![ident()]),
        ("nonZero", vec![ident()]),
        ("predecessor", vec![ident()]),
        ("widen", vec![expr()]),
        ("succeed", vec![expr()]),
    ]
}

fn item_def_constructors() -> Constructors {
    let ty = || local_t("RustType");
    vec![
        ("enum", vec![ty(), list_t(local_t("Variant"))]),
        (
            "apply",
            vec![
                nat_t(),
                list_t(ty()),
                ty(),
                bool_t(),
                list_t(local_t("Dispatch")),
            ],
        ),
        (
            "function",
            vec![
                local_t("Ident"),
                list_t(local_t("Parameter")),
                ty(),
                local_t("Block"),
            ],
        ),
    ]
}

/// The `RustSyntax` module: the closed Rust AST of the renderer.
#[must_use]
pub fn syntax_module() -> String {
    let declarations = vec![
        inductive("IdentKind", nullary(&IDENT_KINDS)),
        inductive("Ident", ident_constructors()),
        inductive("RustType", type_constructors()),
        inductive("Lit", lit_constructors()),
        inductive("Ctor", ctor_constructors()),
        inductive("Pat", pat_constructors()),
        inductive("Item", item_constructors()),
        inductive("Callee", callee_constructors()),
        mutual("Code", inductive("Expr", expr_constructors())),
        mutual(
            "Code",
            inductive(
                "Let",
                vec![(
                    "mk",
                    vec![
                        local_t("Pat"),
                        option_t(local_t("RustType")),
                        local_t("Expr"),
                    ],
                )],
            ),
        ),
        mutual(
            "Code",
            inductive(
                "Block",
                vec![("mk", vec![list_t(local_t("Let")), local_t("Expr")])],
            ),
        ),
        mutual(
            "Code",
            inductive("Arm", vec![("mk", vec![local_t("Pat"), local_t("Block")])]),
        ),
        inductive("CaptureRead", nullary(&["copy", "clone", "unbox", "unit"])),
        structure(
            "Dispatch",
            vec![
                ("function", nat_t()),
                ("captures", list_t(local_t("CaptureRead"))),
                ("functionFallible", bool_t()),
            ],
        ),
        structure(
            "Variant",
            vec![("number", nat_t()), ("fields", list_t(local_t("RustType")))],
        ),
        structure(
            "Parameter",
            vec![("pattern", local_t("Pat")), ("type", local_t("RustType"))],
        ),
        inductive("ItemDef", item_def_constructors()),
        inductive("Profile", nullary(&["core", "std"])),
        structure(
            "Crate",
            vec![
                ("profile", local_t("Profile")),
                ("items", list_t(local_t("ItemDef"))),
            ],
        ),
    ];
    module_tex(RUST_SYNTAX, &[SYNTAX], declarations)
}

// --- RustSemantics: the builder ---------------------------------------------

/// Builds terms that need fresh binders, numbered in one sequence across
/// the module (so the order in which terms are built is load-bearing, as in
/// `calculus_source`).
struct Machine {
    drawn: Cell<u32>,
}

impl Machine {
    fn fresh(&self, base: &str) -> String {
        self.drawn.set(self.drawn.get() + 1);
        format!("{base}{}", self.drawn.get())
    }

    fn ignored(&self) -> String {
        self.fresh("ignored")
    }

    /// A match on `ty` of `module` over `constructors` where every
    /// constructor without a case takes `default`.
    fn match_or(
        &self,
        scrutinee: Json,
        module: &str,
        ty: &str,
        constructors: &Constructors,
        mut cases: Vec<(&str, Vec<String>, Json)>,
        default: Option<&Json>,
    ) -> Json {
        let branches = constructors
            .iter()
            .map(|(constructor, fields)| {
                let (binders, body) =
                    match cases.iter().position(|(name, _, _)| name == constructor) {
                        Some(position) => {
                            let (_, binders, body) = cases.remove(position);
                            (binders, body)
                        }
                        None => (
                            fields.iter().map(|_| self.ignored()).collect(),
                            default
                                .unwrap_or_else(|| panic!("{ty}.{constructor} has no case"))
                                .clone(),
                        ),
                    };
                lx::branch(
                    term::member(module, &format!("{ty}.{constructor}")),
                    binders,
                    body,
                )
            })
            .collect();
        assert!(cases.is_empty(), "a {ty} case names no constructor");
        matching(scrutinee, branches)
    }

    fn value_match(
        &self,
        scrutinee: Json,
        cases: Vec<(&str, &[&str], Json)>,
        default: Json,
    ) -> Json {
        let cases = cases
            .into_iter()
            .map(|(name, binders, body)| (name, owned(binders), body))
            .collect();
        self.match_or(
            scrutinee,
            SYNTAX,
            "Value",
            &value_constructors(),
            cases,
            Some(&default),
        )
    }

    fn syntax_match(
        &self,
        scrutinee: Json,
        ty: &str,
        constructors: &Constructors,
        cases: Vec<(&str, &[&str], Json)>,
        default: Option<Json>,
    ) -> Json {
        let cases = cases
            .into_iter()
            .map(|(name, binders, body)| (name, owned(binders), body))
            .collect();
        self.match_or(
            scrutinee,
            RUST_SYNTAX,
            ty,
            constructors,
            cases,
            default.as_ref(),
        )
    }

    /// A match on an `IntKind`, its body built for each fixed width.
    fn kind_match(&self, scrutinee: Json, mut body: impl FnMut(&str, &str) -> Json) -> Json {
        matching(
            scrutinee,
            FIXED
                .iter()
                .map(|(name, kind)| {
                    target_branch(&format!("IntKind.{name}"), &[], body(name, kind))
                })
                .collect(),
        )
    }
}

// --- RustSemantics: values and identifiers ---------------------------------

fn outcome_types() -> Vec<Json> {
    vec![
        inductive(
            "ROutcome",
            vec![
                ("value", vec![value_t()]),
                ("raise", vec![]),
                ("abort", vec![]),
                ("stuck", vec![]),
                ("exhausted", vec![]),
            ],
        ),
        inductive(
            "ROutcomes",
            vec![
                ("values", vec![values_t()]),
                ("raise", vec![]),
                ("abort", vec![]),
                ("stuck", vec![]),
                ("exhausted", vec![]),
            ],
        ),
    ]
}

/// Each identifier kind's number, which identifier equality compares.
fn kind_number() -> Json {
    definition(
        "kindNumber",
        vec![parameter("kind", syntax_t("IdentKind"))],
        nat_t(),
        matching(
            var("kind"),
            IDENT_KINDS
                .iter()
                .enumerate()
                .map(|(number, kind)| {
                    syntax_branch(&format!("IdentKind.{kind}"), &[], nat(number as u64))
                })
                .collect(),
        ),
    )
}

/// Whether two identifiers are the same name.
fn same_ident(m: &Machine) -> Json {
    let other = || var("other");
    let generated = m.match_or(
        other(),
        RUST_SYNTAX,
        "Ident",
        &ident_constructors(),
        vec![(
            "generated",
            owned(&["otherKind", "otherIndex"]),
            and(
                beq(
                    call("kindNumber", vec![var("kind")]),
                    call("kindNumber", vec![var("otherKind")]),
                ),
                beq(var("index"), var("otherIndex")),
            ),
        )],
        Some(&lx::boolean(false)),
    );
    let export = m.match_or(
        other(),
        RUST_SYNTAX,
        "Ident",
        &ident_constructors(),
        vec![(
            "exported",
            owned(&["otherName"]),
            prim("equal", vec![var("name"), var("otherName")], bool_t()),
        )],
        Some(&lx::boolean(false)),
    );
    definition(
        "sameIdent",
        vec![parameter("ident", ident_t()), parameter("other", ident_t())],
        bool_t(),
        matching(
            var("ident"),
            vec![
                syntax_branch("Ident.generated", &["kind", "index"], generated),
                syntax_branch("Ident.exported", &["name"], export),
            ],
        ),
    )
}

/// The value a name is bound to, innermost binding first.
fn lookup() -> Json {
    recursive(
        "environment",
        definition(
            "lookup",
            vec![
                parameter("environment", environment_t()),
                parameter("name", ident_t()),
            ],
            option_t(value_t()),
            list_match(
                var("environment"),
                none(value_t()),
                "entry",
                "rest",
                lx::ite(
                    call("sameIdent", vec![lx::first(var("entry")), var("name")]),
                    some(value_t(), lx::second(var("entry"))),
                    call("lookup", vec![var("rest"), var("name")]),
                ),
            ),
        ),
    )
}

/// `some` of a value built from the scalar `result` of width `name`, or
/// `none` when the scalar does not exist.
fn scalar_or_none(result: Json, constructor: &str) -> Json {
    option_match(
        result,
        none(value_t()),
        "scalar",
        some(value_t(), value(constructor, vec![var("scalar")])),
    )
}

/// A literal's value; a fixed-width literal outside its width has none.
fn literal(m: &Machine) -> Json {
    let fixed = m.kind_match(var("kind"), |name, kind| {
        scalar_or_none(
            prim(
                "checked_convert",
                vec![var("number")],
                option_t(scalar_t(kind)),
            ),
            name,
        )
    });
    let present = |built: Json| some(value_t(), built);
    axioms(
        &["Quot.sound", "propext"],
        definition(
            "literal",
            vec![parameter("written", syntax_t("Lit"))],
            option_t(value_t()),
            m.syntax_match(
                var("written"),
                "Lit",
                &lit_constructors(),
                vec![
                    ("unit", &[], present(value("unit", vec![]))),
                    ("bool", &["flag"], present(value("bool", vec![var("flag")]))),
                    (
                        "nat",
                        &["number"],
                        present(value("nat", vec![var("number")])),
                    ),
                    (
                        "int",
                        &["number"],
                        present(value("int", vec![var("number")])),
                    ),
                    ("fixed", &["kind", "number"], fixed),
                    (
                        "str",
                        &["text"],
                        present(value("string", vec![var("text")])),
                    ),
                    (
                        "bytes",
                        &["octets"],
                        present(value("bytes", vec![var("octets")])),
                    ),
                    (
                        "ordering",
                        &["order"],
                        present(value("ordering", vec![var("order")])),
                    ),
                ],
                None,
            ),
        ),
    )
}

/// An integer as Rust's `i128::from` widens it: an `int` or a fixed width,
/// whose value an `i128` holds exactly, so the machine keeps it in its own
/// representation; anything else has no widening.
fn widened(m: &Machine) -> Json {
    let mut cases: Vec<(&str, &[&str], Json)> = vec![(
        "int",
        &["number"],
        some(value_t(), value("int", vec![var("number")])),
    )];
    for (name, _) in FIXED {
        cases.push((
            name,
            &["number"],
            some(value_t(), value(name, vec![var("number")])),
        ));
    }
    definition(
        "widened",
        vec![parameter("value", value_t())],
        option_t(value_t()),
        m.value_match(var("value"), cases, none(value_t())),
    )
}

// --- RustSemantics: patterns and constructors -------------------------------

/// The bindings `pattern` makes when it matches `value`, before the
/// environment `environment`; `none` when it does not match.
fn bind_pattern(m: &Machine) -> Json {
    let extend = || var("environment");
    let unwrap = |constructor: &'static str, inner: &str, binder: &'static str| {
        m.value_match(
            var("value"),
            vec![(
                constructor,
                &[binder] as &[&str],
                call("bindPattern", vec![var(inner), var(binder), extend()]),
            )],
            none(environment_t()),
        )
    };
    let unit = m.value_match(
        var("value"),
        vec![("unit", &[], some(environment_t(), extend()))],
        none(environment_t()),
    );
    let absent = m.value_match(
        var("value"),
        vec![("none", &[], some(environment_t(), extend()))],
        none(environment_t()),
    );
    let tuple = m.value_match(
        var("value"),
        vec![(
            "pair",
            &["left", "right"],
            call(
                "bindPatterns",
                vec![
                    var("patterns"),
                    lx::list(value_t(), vec![var("left"), var("right")]),
                    extend(),
                ],
            ),
        )],
        none(environment_t()),
    );
    let ordering = m.value_match(
        var("value"),
        vec![(
            "ordering",
            &["found"],
            lx::ite(
                lx::call(
                    term::member(SEMANTICS, "sameOrder"),
                    vec![var("order"), var("found")],
                ),
                some(environment_t(), extend()),
                none(environment_t()),
            ),
        )],
        none(environment_t()),
    );
    let adt = m.value_match(
        var("value"),
        vec![(
            "adt",
            &["tag", "fields"],
            lx::ite(
                beq(var("tag"), var("constructor")),
                call(
                    "bindPatterns",
                    vec![var("patterns"), var("fields"), extend()],
                ),
                none(environment_t()),
            ),
        )],
        none(environment_t()),
    );
    let some_case = unwrap("some", "inner", "held");
    let ok_case = unwrap("ok", "inner", "held");
    let err_case = unwrap("error", "inner", "held");
    mutual(
        "Patterns",
        recursive(
            "pattern",
            definition(
                "bindPattern",
                vec![
                    parameter("pattern", syntax_t("Pat")),
                    parameter("value", value_t()),
                    parameter("environment", environment_t()),
                ],
                option_t(environment_t()),
                m.syntax_match(
                    var("pattern"),
                    "Pat",
                    &pat_constructors(),
                    vec![
                        ("wild", &[], some(environment_t(), extend())),
                        (
                            "bind",
                            &["name"],
                            some(
                                environment_t(),
                                cons(lx::pair(var("name"), var("value")), extend()),
                            ),
                        ),
                        ("unit", &[], unit),
                        ("tuple", &["patterns"], tuple),
                        ("none", &[], absent),
                        ("some", &["inner"], some_case),
                        ("ok", &["inner"], ok_case),
                        ("err", &["inner"], err_case),
                        ("ordering", &["order"], ordering),
                        ("adt", &["ignoredAdt", "constructor", "patterns"], adt),
                    ],
                    None,
                ),
            ),
        ),
    )
}

/// The bindings of patterns matched in order against values of the same
/// count.
fn bind_patterns() -> Json {
    mutual(
        "Patterns",
        recursive(
            "patterns",
            definition(
                "bindPatterns",
                vec![
                    parameter("patterns", list_t(syntax_t("Pat"))),
                    parameter("values", values_t()),
                    parameter("environment", environment_t()),
                ],
                option_t(environment_t()),
                list_match(
                    var("patterns"),
                    list_match(
                        var("values"),
                        some(environment_t(), var("environment")),
                        "extraValue",
                        "extraValues",
                        none(environment_t()),
                    ),
                    "pattern",
                    "rest",
                    list_match(
                        var("values"),
                        none(environment_t()),
                        "first",
                        "others",
                        option_match(
                            call(
                                "bindPattern",
                                vec![var("pattern"), var("first"), var("environment")],
                            ),
                            none(environment_t()),
                            "bound",
                            call(
                                "bindPatterns",
                                vec![var("rest"), var("others"), var("bound")],
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}

/// The value a constructor builds from its operands.
fn construct_value(m: &Machine) -> Json {
    let operands_none = |body: Json| {
        list_match(
            var("values"),
            some(value_t(), body),
            "extra",
            "extras",
            none(value_t()),
        )
    };
    let one = |m: &Machine, wrap: &dyn Fn(Json) -> Json| {
        let rest = m.fresh("rest");
        list_match(
            var("values"),
            none(value_t()),
            "operand",
            &rest,
            list_match(
                var(&rest),
                some(value_t(), wrap(var("operand"))),
                "extra",
                "extras",
                none(value_t()),
            ),
        )
    };
    let some_case = one(m, &|operand| value("some", vec![operand]));
    let ok_case = one(m, &|operand| value("ok", vec![operand]));
    let err_case = one(m, &|operand| value("error", vec![operand]));
    let cons_rest = m.fresh("rest");
    let cons_case = list_match(
        var("values"),
        none(value_t()),
        "head",
        &cons_rest,
        list_match(
            var(&cons_rest),
            none(value_t()),
            "tail",
            "extras",
            list_match(
                var("extras"),
                m.value_match(
                    var("tail"),
                    vec![(
                        "list",
                        &["items"],
                        some(
                            value_t(),
                            value("list", vec![cons(var("head"), var("items"))]),
                        ),
                    )],
                    none(value_t()),
                ),
                "extra",
                "more",
                none(value_t()),
            ),
        ),
    );
    definition(
        "constructValue",
        vec![
            parameter("building", syntax_t("Ctor")),
            parameter("values", values_t()),
        ],
        option_t(value_t()),
        m.syntax_match(
            var("building"),
            "Ctor",
            &ctor_constructors(),
            vec![
                (
                    "none",
                    &["ignoredNone"],
                    operands_none(value("none", vec![])),
                ),
                ("some", &[], some_case),
                ("ok", &["ignoredOk", "ignoredError"], ok_case),
                ("err", &["ignoredOkType", "ignoredErrorType"], err_case),
                (
                    "adt",
                    &["ignoredAdt", "tag"],
                    some(value_t(), value("adt", vec![var("tag"), var("values")])),
                ),
                (
                    "closure",
                    &["ignoredType", "function"],
                    some(
                        value_t(),
                        value("closure", vec![var("function"), var("values")]),
                    ),
                ),
                ("cons", &[], cons_case),
                (
                    "nil",
                    &["ignoredElement"],
                    operands_none(value("list", vec![nil(value_t())])),
                ),
            ],
            None,
        ),
    )
}

// --- RustSemantics: the runtime ---------------------------------------------

/// The calculus primitive each runtime item realizes.
fn item_primitive(m: &Machine) -> Json {
    let primitive =
        |name: &str| lx::construct(term::member(SYNTAX, &format!("Prim.{name}")), vec![]);
    let mut cases: Vec<(&str, &[&str], Json)> = Vec::new();
    for (name, realized, _, _) in ITEMS.iter().take(20) {
        cases.push((name, &[], primitive(realized)));
    }
    for name in WIDTH_ITEMS {
        cases.push((name, &["ignoredWidth"], primitive(name)));
    }
    cases.push((
        "convert",
        &["target"],
        lx::construct(term::member(SYNTAX, "Prim.convert"), vec![var("target")]),
    ));
    for (name, realized, _, _) in ITEMS.iter().skip(31) {
        cases.push((name, &[], primitive(realized)));
    }
    for (name, realized, _, _) in MORE_ITEMS {
        cases.push((name, &[], primitive(realized)));
    }
    cases.push((
        "formatFixed",
        &["ignoredFormat"],
        primitive("formatDecimal"),
    ));
    cases.push((
        "parseInt",
        &[],
        lx::construct(
            term::member(SYNTAX, "Prim.parseDecimal"),
            vec![lx::construct(term::member(SYNTAX, "Ty.int"), vec![])],
        ),
    ));
    cases.push((
        "parseFixed",
        &["kind"],
        lx::construct(
            term::member(SYNTAX, "Prim.parseDecimal"),
            vec![lx::construct(
                term::member(SYNTAX, "Ty.fixed"),
                vec![var("kind")],
            )],
        ),
    ));
    definition(
        "itemPrimitive",
        vec![parameter("item", syntax_t("Item"))],
        target_t("Prim"),
        m.syntax_match(var("item"), "Item", &item_constructors(), cases, None),
    )
}

/// A table over the items: `flag` for each item without a width, `width`
/// for each item that carries one.
fn item_table(
    m: &Machine,
    name: &str,
    result: Json,
    plain: impl Fn(&str) -> Json,
    width: impl Fn(&str) -> Json,
) -> Json {
    let mut cases: Vec<(&str, Vec<String>, Json)> = Vec::new();
    let mut plain_names: Vec<&str> = ITEMS.iter().map(|(item, _, _, _)| *item).collect();
    plain_names.extend(MORE_ITEMS.iter().map(|(item, _, _, _)| *item));
    plain_names.push("parseInt");
    for item in plain_names {
        if !WIDTH_ITEMS.contains(&item) {
            cases.push((item, Vec::new(), plain(item)));
        }
    }
    for item in WIDTH_ITEMS
        .iter()
        .copied()
        .chain(["convert", "formatFixed", "parseFixed"])
    {
        cases.push((item, owned(&["kind"]), width(item)));
    }
    definition(
        name,
        vec![parameter("item", syntax_t("Item"))],
        result,
        m.match_or(
            var("item"),
            RUST_SYNTAX,
            "Item",
            &item_constructors(),
            cases,
            None,
        ),
    )
}

fn table_flag(item: &str, column: fn(&(&str, &str, bool, bool)) -> bool) -> bool {
    ITEMS
        .iter()
        .chain(MORE_ITEMS.iter())
        .find(|row| row.0 == item)
        .is_some_and(column)
}

/// The failure and heap classes the machine declares for the item named
/// `name`: whether it returns `R<T>`, and whether it needs the heap.
#[must_use]
pub fn item_classes(name: &str) -> Option<(bool, bool)> {
    if !item_constructors().iter().any(|(item, _)| *item == name) {
        return None;
    }
    let widthed =
        WIDTH_ITEMS.contains(&name) || matches!(name, "convert" | "formatFixed" | "parseFixed");
    Some(if widthed {
        (false, matches!(name, "formatFixed" | "parseFixed"))
    } else {
        (
            name == "parseInt" || table_flag(name, |row| row.2),
            name == "parseInt" || table_flag(name, |row| row.3),
        )
    })
}

/// Whether an item returns `R<T>`: exactly the items whose primitive can
/// overflow.
fn item_fallible(m: &Machine) -> Json {
    item_table(
        m,
        "itemFallible",
        bool_t(),
        |item| lx::boolean(item == "parseInt" || table_flag(item, |row| row.2)),
        |_| lx::boolean(false),
    )
}

/// Whether an item takes or returns heap storage, and so exists only in
/// `rust-std`.
fn item_heap(m: &Machine) -> Json {
    item_table(
        m,
        "itemHeap",
        bool_t(),
        |item| lx::boolean(item == "parseInt" || table_flag(item, |row| row.3)),
        |item| lx::boolean(matches!(item, "formatFixed" | "parseFixed")),
    )
}

/// The operands a runtime item hands its primitive: `natSucc` adds one.
fn item_operands(m: &Machine) -> Json {
    let cases: Vec<(&str, Vec<String>, Json)> = vec![(
        "natSucc",
        Vec::new(),
        prim(
            "append",
            vec![
                var("values"),
                lx::list(value_t(), vec![value("nat", vec![nat(1)])]),
            ],
            values_t(),
        ),
    )];
    definition(
        "itemOperands",
        vec![
            parameter("item", syntax_t("Item")),
            parameter("values", values_t()),
        ],
        values_t(),
        m.match_or(
            var("item"),
            RUST_SYNTAX,
            "Item",
            &item_constructors(),
            cases,
            Some(&var("values")),
        ),
    )
}

/// A runtime item applied to `values` in `profile`: the primitive it
/// realizes, a fallible item's overflow as `Err(Overflow)` and its value
/// as `Ok`. An overflow of an item that cannot fail is the machine's abort:
/// the only such overflow is the length of a sequence of `2^64` or more
/// elements, which no machine holds. An item outside its profile and an
/// operation the primitive does not define are stuck. An operand of another width than the item's is a
/// Rust type error, which the renderer's checks refuse before any crate
/// exists, so the machine states nothing about it.
fn run_item(m: &Machine) -> Json {
    let fallible = || call("itemFallible", vec![var("item")]);
    let realized = lx::call(
        term::member(SEMANTICS, "primitive"),
        vec![
            call("itemPrimitive", vec![var("item")]),
            call("itemOperands", vec![var("item"), var("values")]),
        ],
    );
    let result = matching(
        realized,
        vec![
            lx::branch(
                term::member(SEMANTICS, "Outcome.value"),
                owned(&["computed", "ignoredSteps"]),
                bool_match(
                    fallible(),
                    produced(value("ok", vec![var("computed")])),
                    produced(var("computed")),
                ),
            ),
            lx::branch(
                term::member(SEMANTICS, "Outcome.overflow"),
                owned(&["ignoredOverflow"]),
                bool_match(
                    fallible(),
                    produced(value("error", vec![value("unit", vec![])])),
                    outcome("abort", vec![]),
                ),
            ),
            lx::branch(
                term::member(SEMANTICS, "Outcome.stuck"),
                Vec::new(),
                stuck(),
            ),
            lx::branch(
                term::member(SEMANTICS, "Outcome.exhausted"),
                Vec::new(),
                stuck(),
            ),
        ],
    );
    let available = m.syntax_match(
        var("profile"),
        "Profile",
        &nullary(&["core", "std"]),
        vec![
            (
                "core",
                &[],
                lx::ite(call("itemHeap", vec![var("item")]), stuck(), result.clone()),
            ),
            ("std", &[], result),
        ],
        None,
    );
    axioms(
        CLASSICAL,
        definition(
            "runItem",
            vec![
                parameter("profile", syntax_t("Profile")),
                parameter("item", syntax_t("Item")),
                parameter("values", values_t()),
            ],
            outcome_t(),
            available,
        ),
    )
}

/// The result of a call that propagates: `Ok(v)` is `v`, `Err` raises, and
/// any other value is stuck.
fn propagated(m: &Machine) -> Json {
    definition(
        "propagated",
        vec![parameter("result", outcome_t())],
        outcome_t(),
        matching(
            var("result"),
            vec![
                branch(
                    "ROutcome.value",
                    &["given"],
                    m.value_match(
                        var("given"),
                        vec![
                            ("ok", &["inner"], produced(var("inner"))),
                            ("error", &["ignoredError"], outcome("raise", vec![])),
                        ],
                        stuck(),
                    ),
                ),
                branch("ROutcome.raise", &[], outcome("raise", vec![])),
                branch("ROutcome.abort", &[], outcome("abort", vec![])),
                branch("ROutcome.stuck", &[], stuck()),
                branch("ROutcome.exhausted", &[], outcome("exhausted", vec![])),
            ],
        ),
    )
}

/// A call's result as the call site sees it: propagated through `?`, or
/// as returned.
fn returned() -> Json {
    definition(
        "returned",
        vec![
            parameter("result", outcome_t()),
            parameter("propagate", bool_t()),
        ],
        outcome_t(),
        lx::ite(
            var("propagate"),
            call("propagated", vec![var("result")]),
            var("result"),
        ),
    )
}

// --- RustSemantics: items ---------------------------------------------------

fn function_item_t() -> Json {
    product_t(
        list_t(syntax_t("Parameter")),
        product_t(syntax_t("RustType"), syntax_t("Block")),
    )
}

/// The function an identifier names among the items: its parameters,
/// result type, and body.
fn find_function(m: &Machine) -> Json {
    let found = m.syntax_match(
        var("item"),
        "ItemDef",
        &item_def_constructors(),
        vec![(
            "function",
            &["name", "parameters", "result", "body"],
            lx::ite(
                call("sameIdent", vec![var("name"), var("wanted")]),
                some(
                    function_item_t(),
                    lx::pair(var("parameters"), lx::pair(var("result"), var("body"))),
                ),
                call("findFunction", vec![var("rest"), var("wanted")]),
            ),
        )],
        Some(call("findFunction", vec![var("rest"), var("wanted")])),
    );
    recursive(
        "items",
        definition(
            "findFunction",
            vec![
                parameter("items", list_t(syntax_t("ItemDef"))),
                parameter("wanted", ident_t()),
            ],
            option_t(function_item_t()),
            list_match(var("items"), none(function_item_t()), "item", "rest", found),
        ),
    )
}

/// The dispatch arm of the closure of `function` holding `count` captures
/// among `arms`: whether that function can fail.
fn find_arm() -> Json {
    recursive(
        "arms",
        definition(
            "findArm",
            vec![
                parameter("arms", list_t(syntax_t("Dispatch"))),
                parameter("function", nat_t()),
                parameter("count", nat_t()),
            ],
            option_t(bool_t()),
            list_match(
                var("arms"),
                none(bool_t()),
                "arm",
                "rest",
                lx::ite(
                    and(
                        beq(project(var("arm"), "function"), var("function")),
                        beq(
                            prim("length", vec![project(var("arm"), "captures")], nat_t()),
                            var("count"),
                        ),
                    ),
                    some(bool_t(), project(var("arm"), "functionFallible")),
                    call("findArm", vec![var("rest"), var("function"), var("count")]),
                ),
            ),
        ),
    )
}

/// The dispatch of a closure of `function` with `count` captures: whether
/// the `apply` it goes through is fallible, and whether the function is.
/// Its function type is the one whose closures hold that many captures of
/// that function, so the first arm found is the one Rust's static dispatch
/// takes.
fn find_dispatch(m: &Machine) -> Json {
    let pair_t = || product_t(bool_t(), bool_t());
    let found = m.syntax_match(
        var("item"),
        "ItemDef",
        &item_def_constructors(),
        vec![(
            "apply",
            &[
                "ignoredFnType",
                "ignoredParameters",
                "ignoredResult",
                "fallible",
                "arms",
            ],
            option_match(
                call("findArm", vec![var("arms"), var("function"), var("count")]),
                call(
                    "findDispatch",
                    vec![var("rest"), var("function"), var("count")],
                ),
                "armFallible",
                some(pair_t(), lx::pair(var("fallible"), var("armFallible"))),
            ),
        )],
        Some(call(
            "findDispatch",
            vec![var("rest"), var("function"), var("count")],
        )),
    );
    recursive(
        "items",
        definition(
            "findDispatch",
            vec![
                parameter("items", list_t(syntax_t("ItemDef"))),
                parameter("function", nat_t()),
                parameter("count", nat_t()),
            ],
            option_t(pair_t()),
            list_match(var("items"), none(pair_t()), "item", "rest", found),
        ),
    )
}

/// Whether a function's result type is fallible, `R<T>`.
fn fallible_type(m: &Machine) -> Json {
    definition(
        "fallibleType",
        vec![parameter("type", syntax_t("RustType"))],
        bool_t(),
        m.syntax_match(
            var("type"),
            "RustType",
            &type_constructors(),
            vec![("fallible", &["ignoredInner"], lx::boolean(true))],
            Some(lx::boolean(false)),
        ),
    )
}

// --- RustSemantics: evaluation ----------------------------------------------

fn fuel_parameters(last: Vec<Json>) -> Vec<Json> {
    let mut out = vec![
        parameter("fuel", nat_t()),
        parameter("crate", crate_t()),
        parameter("environment", environment_t()),
    ];
    out.extend(last);
    out
}

fn on_fuel(exhausted: Json, remaining: Json) -> Json {
    matching(
        var("fuel"),
        vec![
            branch("Nat.zero", &[], exhausted),
            branch("Nat.succ", &["remaining"], remaining),
        ],
    )
}

fn eval(environment: Json, expression: Json) -> Json {
    call(
        "eval",
        vec![var("remaining"), var("crate"), environment, expression],
    )
}
fn eval_list(expressions: Json) -> Json {
    call(
        "evalList",
        vec![
            var("remaining"),
            var("crate"),
            var("environment"),
            expressions,
        ],
    )
}
fn eval_block(environment: Json, block: Json) -> Json {
    call(
        "evalBlock",
        vec![var("remaining"), var("crate"), environment, block],
    )
}

/// Continue with `scrutinee`'s value bound to `binder`; a raise, a stuck
/// evaluation, and exhaustion pass through.
fn then_value(scrutinee: Json, binder: &str, then: Json) -> Json {
    matching(
        scrutinee,
        vec![
            branch("ROutcome.value", &[binder], then),
            branch("ROutcome.raise", &[], outcome("raise", vec![])),
            branch("ROutcome.abort", &[], outcome("abort", vec![])),
            branch("ROutcome.stuck", &[], stuck()),
            branch("ROutcome.exhausted", &[], outcome("exhausted", vec![])),
        ],
    )
}

fn then_values(scrutinee: Json, binder: &str, then: Json) -> Json {
    matching(
        scrutinee,
        vec![
            branch("ROutcomes.values", &[binder], then),
            branch("ROutcomes.raise", &[], outcome("raise", vec![])),
            branch("ROutcomes.abort", &[], outcome("abort", vec![])),
            branch("ROutcomes.stuck", &[], stuck()),
            branch("ROutcomes.exhausted", &[], outcome("exhausted", vec![])),
        ],
    )
}

/// A read of a bound name: a move, a clone, a copy, a dereference, and an
/// unboxing all read the value the name is bound to.
fn read(constructor: &'static str) -> (&'static str, &'static [&'static str], Json) {
    (
        constructor,
        &["name"],
        option_match(
            call("lookup", vec![var("environment"), var("name")]),
            stuck(),
            "found",
            produced(var("found")),
        ),
    )
}

/// A function's result from its body's outcome: a raise out of a fallible
/// function is its `Err(Overflow)`, and out of any other is stuck.
fn finished() -> Json {
    definition(
        "finished",
        vec![
            parameter("result", outcome_t()),
            parameter("fallible", bool_t()),
        ],
        outcome_t(),
        matching(
            var("result"),
            vec![
                branch("ROutcome.value", &["computed"], produced(var("computed"))),
                branch(
                    "ROutcome.raise",
                    &[],
                    lx::ite(
                        var("fallible"),
                        produced(value("error", vec![value("unit", vec![])])),
                        stuck(),
                    ),
                ),
                branch("ROutcome.abort", &[], outcome("abort", vec![])),
                branch("ROutcome.stuck", &[], stuck()),
                branch("ROutcome.exhausted", &[], outcome("exhausted", vec![])),
            ],
        ),
    )
}

#[allow(clippy::too_many_lines)]
fn expression_cases(m: &Machine) -> Vec<(&'static str, &'static [&'static str], Json)> {
    let reading = read;
    let environment = || var("environment");
    let integer_case = |m: &Machine, test: fn(Json) -> Json| {
        option_match(
            call("lookup", vec![environment(), var("name")]),
            stuck(),
            "found",
            m.value_match(
                var("found"),
                vec![(
                    "nat",
                    &["number"],
                    produced(value("bool", vec![test(var("number"))])),
                )],
                stuck(),
            ),
        )
    };
    let call_case = {
        let function = m.syntax_match(
            var("callee"),
            "Callee",
            &callee_constructors(),
            vec![
                (
                    "function",
                    &["index"],
                    call(
                        "invoke",
                        vec![
                            var("remaining"),
                            var("crate"),
                            syntax_ctor(
                                "Ident.generated",
                                vec![syntax_ctor("IdentKind.function", vec![]), var("index")],
                            ),
                            var("arguments"),
                        ],
                    ),
                ),
                (
                    "runtime",
                    &["item"],
                    call(
                        "runItem",
                        vec![
                            project(var("crate"), "profile"),
                            var("item"),
                            var("arguments"),
                        ],
                    ),
                ),
            ],
            None,
        );
        then_values(
            eval_list(var("operands")),
            "arguments",
            call("returned", vec![function, var("propagate")]),
        )
    };
    let apply_case = option_match(
        call("lookup", vec![environment(), var("holder")]),
        stuck(),
        "target",
        m.value_match(
            var("target"),
            vec![(
                "closure",
                &["function", "captured"],
                then_values(
                    eval_list(var("operands")),
                    "arguments",
                    option_match(
                        call(
                            "findDispatch",
                            vec![
                                project(var("crate"), "items"),
                                var("function"),
                                prim("length", vec![var("captured")], nat_t()),
                            ],
                        ),
                        stuck(),
                        "dispatch",
                        call(
                            "returned",
                            vec![
                                call(
                                    "dispatched",
                                    vec![
                                        call(
                                            "invoke",
                                            vec![
                                                var("remaining"),
                                                var("crate"),
                                                syntax_ctor(
                                                    "Ident.generated",
                                                    vec![
                                                        syntax_ctor("IdentKind.function", vec![]),
                                                        var("function"),
                                                    ],
                                                ),
                                                prim(
                                                    "append",
                                                    vec![var("captured"), var("arguments")],
                                                    values_t(),
                                                ),
                                            ],
                                        ),
                                        lx::and(
                                            lx::first(var("dispatch")),
                                            not(lx::second(var("dispatch"))),
                                        ),
                                    ],
                                ),
                                var("propagate"),
                            ],
                        ),
                    ),
                ),
            )],
            stuck(),
        ),
    );
    let construct_case = then_values(
        eval_list(var("operands")),
        "arguments",
        option_match(
            call("constructValue", vec![var("constructor"), var("arguments")]),
            stuck(),
            "built",
            produced(var("built")),
        ),
    );
    let pair_case = then_value(
        eval(environment(), var("left")),
        "leftValue",
        then_value(
            eval(environment(), var("right")),
            "rightValue",
            produced(value("pair", vec![var("leftValue"), var("rightValue")])),
        ),
    );
    let not_case = then_value(
        eval(environment(), var("operand")),
        "negated",
        m.value_match(
            var("negated"),
            vec![(
                "bool",
                &["flag"],
                produced(value("bool", vec![not(var("flag"))])),
            )],
            stuck(),
        ),
    );
    let cond_case = then_value(
        eval(environment(), var("condition")),
        "tested",
        m.value_match(
            var("tested"),
            vec![(
                "bool",
                &["flag"],
                lx::ite(
                    var("flag"),
                    eval_block(environment(), var("thenBranch")),
                    eval_block(environment(), var("elseBranch")),
                ),
            )],
            stuck(),
        ),
    );
    let match_case = then_value(
        eval(environment(), var("scrutinee")),
        "matched",
        call(
            "evalArms",
            vec![
                var("remaining"),
                var("crate"),
                environment(),
                var("matched"),
                var("arms"),
            ],
        ),
    );
    let uncons_case = option_match(
        call("lookup", vec![environment(), var("name")]),
        stuck(),
        "found",
        m.value_match(
            var("found"),
            vec![(
                "list",
                &["items"],
                list_match(
                    var("items"),
                    produced(value("none", vec![])),
                    "head",
                    "tail",
                    produced(value(
                        "some",
                        vec![value(
                            "pair",
                            vec![var("head"), value("list", vec![var("tail")])],
                        )],
                    )),
                ),
            )],
            stuck(),
        ),
    );
    let predecessor_case = option_match(
        call("lookup", vec![environment(), var("name")]),
        stuck(),
        "found",
        m.value_match(
            var("found"),
            vec![(
                "nat",
                &["number"],
                matching(
                    var("number"),
                    vec![
                        branch("Nat.zero", &[], stuck()),
                        branch(
                            "Nat.succ",
                            &["below"],
                            produced(value("nat", vec![var("below")])),
                        ),
                    ],
                ),
            )],
            stuck(),
        ),
    );
    let widen_case = then_value(
        eval(environment(), var("operand")),
        "narrow",
        option_match(
            call("widened", vec![var("narrow")]),
            stuck(),
            "wide",
            produced(var("wide")),
        ),
    );
    let succeed_case = then_value(
        eval(environment(), var("operand")),
        "succeeded",
        produced(value("ok", vec![var("succeeded")])),
    );
    let zero_case = integer_case(m, |number| beq(number, nat(0)));
    let nonzero_case = integer_case(m, |number| not(beq(number, nat(0))));
    vec![
        (
            "lit",
            &["written"],
            option_match(
                call("literal", vec![var("written")]),
                stuck(),
                "found",
                produced(var("found")),
            ),
        ),
        reading("move"),
        reading("clone"),
        reading("copy"),
        reading("deref"),
        ("not", &["operand"], not_case),
        reading("unbox"),
        ("box", &["operand"], eval(environment(), var("operand"))),
        ("call", &["callee", "operands", "propagate"], call_case),
        ("apply", &["holder", "operands", "propagate"], apply_case),
        ("construct", &["constructor", "operands"], construct_case),
        ("pair", &["left", "right"], pair_case),
        (
            "cond",
            &["condition", "thenBranch", "elseBranch"],
            cond_case,
        ),
        ("matchOn", &["scrutinee", "arms"], match_case),
        ("block", &["inner"], eval_block(environment(), var("inner"))),
        ("uncons", &["name"], uncons_case),
        ("isZero", &["name"], zero_case),
        ("nonZero", &["name"], nonzero_case),
        ("predecessor", &["name"], predecessor_case),
        ("widen", &["operand"], widen_case),
        ("succeed", &["operand"], succeed_case),
    ]
}

/// A dispatch through a fallible `apply` of a function that cannot fail
/// wraps its value in `Ok`.
fn dispatched() -> Json {
    definition(
        "dispatched",
        vec![
            parameter("result", outcome_t()),
            parameter("wrap", bool_t()),
        ],
        outcome_t(),
        lx::ite(
            var("wrap"),
            then_value(
                var("result"),
                "inner",
                produced(value("ok", vec![var("inner")])),
            ),
            var("result"),
        ),
    )
}

#[allow(clippy::too_many_lines)]
fn evaluation(m: &Machine) -> Vec<Json> {
    let cases = expression_cases(m);
    let eval_definition = definition(
        "eval",
        fuel_parameters(vec![parameter("expression", syntax_t("Expr"))]),
        outcome_t(),
        on_fuel(
            outcome("exhausted", vec![]),
            m.syntax_match(var("expression"), "Expr", &expr_constructors(), cases, None),
        ),
    );
    let eval_list_definition = definition(
        "evalList",
        fuel_parameters(vec![parameter("expressions", list_t(syntax_t("Expr")))]),
        local_t("ROutcomes"),
        on_fuel(
            outcomes("exhausted", vec![]),
            list_match(
                var("expressions"),
                outcomes("values", vec![nil(value_t())]),
                "head",
                "rest",
                matching(
                    eval(var("environment"), var("head")),
                    vec![
                        branch(
                            "ROutcome.value",
                            &["headValue"],
                            matching(
                                eval_list(var("rest")),
                                vec![
                                    branch(
                                        "ROutcomes.values",
                                        &["restValues"],
                                        outcomes(
                                            "values",
                                            vec![cons(var("headValue"), var("restValues"))],
                                        ),
                                    ),
                                    branch("ROutcomes.raise", &[], outcomes("raise", vec![])),
                                    branch("ROutcomes.abort", &[], outcomes("abort", vec![])),
                                    branch("ROutcomes.stuck", &[], outcomes("stuck", vec![])),
                                    branch(
                                        "ROutcomes.exhausted",
                                        &[],
                                        outcomes("exhausted", vec![]),
                                    ),
                                ],
                            ),
                        ),
                        branch("ROutcome.raise", &[], outcomes("raise", vec![])),
                        branch("ROutcome.abort", &[], outcomes("abort", vec![])),
                        branch("ROutcome.stuck", &[], outcomes("stuck", vec![])),
                        branch("ROutcome.exhausted", &[], outcomes("exhausted", vec![])),
                    ],
                ),
            ),
        ),
    );
    // A block binds its lets in order, each in the scope of those before it,
    // then evaluates its tail; the bindings end with the block.
    let eval_block_definition = definition(
        "evalBlock",
        fuel_parameters(vec![parameter("block", syntax_t("Block"))]),
        outcome_t(),
        on_fuel(
            outcome("exhausted", vec![]),
            matching(
                var("block"),
                vec![syntax_branch(
                    "Block.mk",
                    &["lets", "tail"],
                    list_match(
                        var("lets"),
                        eval(var("environment"), var("tail")),
                        "binding",
                        "rest",
                        matching(
                            var("binding"),
                            vec![syntax_branch(
                                "Let.mk",
                                &["pattern", "ignoredType", "bound"],
                                then_value(
                                    eval(var("environment"), var("bound")),
                                    "boundValue",
                                    option_match(
                                        call(
                                            "bindPattern",
                                            vec![
                                                var("pattern"),
                                                var("boundValue"),
                                                var("environment"),
                                            ],
                                        ),
                                        stuck(),
                                        "extended",
                                        eval_block(
                                            var("extended"),
                                            syntax_ctor("Block.mk", vec![var("rest"), var("tail")]),
                                        ),
                                    ),
                                ),
                            )],
                        ),
                    ),
                )],
            ),
        ),
    );
    // The first arm whose pattern the value matches is taken; none is stuck,
    // which a match Rust's exhaustiveness admits never is.
    let eval_arms_definition = definition(
        "evalArms",
        fuel_parameters(vec![
            parameter("scrutinee", value_t()),
            parameter("arms", list_t(syntax_t("Arm"))),
        ]),
        outcome_t(),
        on_fuel(
            outcome("exhausted", vec![]),
            list_match(
                var("arms"),
                stuck(),
                "first",
                "rest",
                matching(
                    var("first"),
                    vec![syntax_branch(
                        "Arm.mk",
                        &["pattern", "body"],
                        option_match(
                            call(
                                "bindPattern",
                                vec![var("pattern"), var("scrutinee"), var("environment")],
                            ),
                            call(
                                "evalArms",
                                vec![
                                    var("remaining"),
                                    var("crate"),
                                    var("environment"),
                                    var("scrutinee"),
                                    var("rest"),
                                ],
                            ),
                            "extended",
                            eval_block(var("extended"), var("body")),
                        ),
                    )],
                ),
            ),
        ),
    );
    // A call binds the function's parameter patterns to the arguments in a
    // fresh environment, evaluates the body, and finishes it.
    let invoke_definition = definition(
        "invoke",
        vec![
            parameter("fuel", nat_t()),
            parameter("crate", crate_t()),
            parameter("callee", ident_t()),
            parameter("arguments", values_t()),
        ],
        outcome_t(),
        on_fuel(
            outcome("exhausted", vec![]),
            option_match(
                call(
                    "findFunction",
                    vec![project(var("crate"), "items"), var("callee")],
                ),
                stuck(),
                "function",
                option_match(
                    call(
                        "bindPatterns",
                        vec![
                            call("patternsOf", vec![lx::first(var("function"))]),
                            var("arguments"),
                            nil(product_t(ident_t(), value_t())),
                        ],
                    ),
                    stuck(),
                    "bound",
                    call(
                        "finished",
                        vec![
                            call(
                                "evalBlock",
                                vec![
                                    var("remaining"),
                                    var("crate"),
                                    var("bound"),
                                    lx::second(lx::second(var("function"))),
                                ],
                            ),
                            call("fallibleType", vec![lx::first(lx::second(var("function")))]),
                        ],
                    ),
                ),
            ),
        ),
    );
    vec![
        eval_definition,
        eval_list_definition,
        eval_block_definition,
        eval_arms_definition,
        invoke_definition,
    ]
    .into_iter()
    .map(|declaration| axioms(CLASSICAL, mutual("Machine", recursive("fuel", declaration))))
    .collect()
}

/// A function's parameter patterns.
fn patterns_of() -> Json {
    recursive(
        "parameters",
        definition(
            "patternsOf",
            vec![parameter("parameters", list_t(syntax_t("Parameter")))],
            list_t(syntax_t("Pat")),
            list_match(
                var("parameters"),
                nil(syntax_t("Pat")),
                "first",
                "rest",
                cons(
                    project(var("first"), "pattern"),
                    call("patternsOf", vec![var("rest")]),
                ),
            ),
        ),
    )
}

/// The outcome of an exported function of `crate` on `arguments`.
fn run_export() -> Json {
    axioms(
        CLASSICAL,
        definition(
            "runExport",
            vec![
                parameter("fuel", nat_t()),
                parameter("crate", crate_t()),
                parameter("name", string_t()),
                parameter("arguments", values_t()),
            ],
            outcome_t(),
            call(
                "invoke",
                vec![
                    var("fuel"),
                    var("crate"),
                    syntax_ctor("Ident.exported", vec![var("name")]),
                    var("arguments"),
                ],
            ),
        ),
    )
}

/// The `RustSemantics` module: the declared Rust machine.
#[must_use]
pub fn semantics_module() -> String {
    let m = Machine {
        drawn: Cell::new(0),
    };
    let mut declarations: Vec<Json> = Vec::new();
    declarations.extend(outcome_types());
    declarations.push(kind_number());
    declarations.push(same_ident(&m));
    declarations.push(lookup());
    declarations.push(literal(&m));
    declarations.push(widened(&m));
    declarations.push(bind_pattern(&m));
    declarations.push(bind_patterns());
    declarations.push(construct_value(&m));
    declarations.push(item_primitive(&m));
    declarations.push(item_fallible(&m));
    declarations.push(item_heap(&m));
    declarations.push(item_operands(&m));
    declarations.push(run_item(&m));
    declarations.push(propagated(&m));
    declarations.push(returned());
    declarations.push(dispatched());
    declarations.push(find_function(&m));
    declarations.push(find_arm());
    declarations.push(find_dispatch(&m));
    declarations.push(fallible_type(&m));
    declarations.push(finished());
    declarations.push(patterns_of());
    declarations.extend(evaluation(&m));
    declarations.push(run_export());
    module_tex(
        RUST_SEMANTICS,
        &[SYNTAX, SEMANTICS, RUST_SYNTAX],
        declarations,
    )
}

/// Every file this module generates, by path relative to the repository
/// root.
#[must_use]
pub fn files() -> BTreeMap<String, Vec<u8>> {
    let mut out = BTreeMap::new();
    for (module, text) in [
        (RUST_SYNTAX, syntax_module()),
        (RUST_SEMANTICS, semantics_module()),
    ] {
        out.insert(format!("compiler/src/{module}.lex.tex"), text.into_bytes());
    }
    out
}
