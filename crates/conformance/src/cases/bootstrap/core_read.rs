//! `BS-40` (SPEC.md §17.23, §17.21.8): the reader of the core notation as
//! language-1.2 source, `LexLeanBoot.CoreRead`, held to the byte to
//! `lexlean::surface::core::read_module`, the reader of `parse_module` up to
//! the decoding of each declaration's members, which stays in the host.
//!
//! The root reads with an explicit stack of jobs (a term, a binder, the parts
//! of a `fun`, a `let`, an arrow, an application, a `proof`, a projection, an
//! annotation) and a second machine for universe levels, so it admits no
//! `recursion`; it scans a token where the oracle peeks it, so that the first
//! error is the oracle's. A declaration is its members, read by the generic
//! notation of `BS-24`, with its kind, name, levels, and roots added.
//!
//! Each planted defect names the one vector that catches it.

use std::collections::{BTreeMap, BTreeSet};

use lexlean::artifact::canonical_json::Json;
use lexlean::calculus::Value;
use lexlean::surface::core::{parse_module, read_module};
use lexlean::surface::module::{locate, CORE};
use serde_json::Value as Json2;

use super::core_json::{enc_json, node_value};
use super::rust_validate::{eligibility_of, rustc_differential};
use super::{list, nat_val, text, Run, Seeded};
use crate::production_export as harness;
use crate::support;

const MODULE: &str = "LexLeanBoot.CoreRead";
const SOURCE: &str = "src/CoreRead.lex.tex";
const ROOTS: [(&str, &[&str]); 1] = [("readCoreModule", &["allocation", "overflow"])];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// Committed core modules of at most this many bytes of notation are read under
/// the evaluator (one in `EVALUATOR_EVERY`), and under rustc those at most the
/// second bound.
const EVALUATOR_BYTES: usize = 9_000;
const EVALUATOR_EVERY: usize = 10;
const RUSTC_BYTES: usize = 200_000;
/// Seeded edits of committed modules under the evaluator and under rustc.
const EVALUATOR_SEEDED: usize = 80;
const RUSTC_SEEDED: usize = 800;

fn failure_value(offset: usize, message: &str) -> Value {
    Value::Adt {
        constructor: 0,
        fields: vec![nat_val(offset), text(message)],
    }
}

/// What the oracle's reader says of a text, as the root's `Result Read Failure`.
fn run(source: &str) -> (Run, Option<String>) {
    let (value, refusal) = match read_module(source) {
        Err(failure) => (
            Value::Error {
                value: Box::new(failure_value(failure.offset, &failure.message)),
            },
            Some(failure.message),
        ),
        Ok(read) => {
            let first_error = read
                .declarations
                .iter()
                .find_map(|declaration| declaration.as_ref().err());
            match first_error {
                Some(failure) => (
                    Value::Error {
                        value: Box::new(failure_value(failure.offset, &failure.message)),
                    },
                    Some(failure.message.clone()),
                ),
                None => (
                    Value::Ok {
                        value: Box::new(Value::Adt {
                            constructor: 0,
                            fields: vec![
                                list(read.imports.iter().map(|name| text(name)).collect()),
                                list(read.nodes.iter().map(node_value).collect()),
                                list(read.proof_nodes.iter().map(|node| nat_val(*node)).collect()),
                                list(
                                    read.declarations
                                        .iter()
                                        .map(|declaration| {
                                            let (object, _) =
                                                declaration.as_ref().expect("no error");
                                            enc_json(&Json::Obj(object.clone()))
                                        })
                                        .collect(),
                                ),
                                list(
                                    read.spans
                                        .iter()
                                        .map(|(start, stop)| Value::Adt {
                                            constructor: 0,
                                            fields: vec![nat_val(*start), nat_val(*stop)],
                                        })
                                        .collect(),
                                ),
                            ],
                        }),
                    },
                    None,
                ),
            }
        }
    };
    (
        (
            vec![text(source)],
            serde_json::to_value(value).expect("a value is JSON"),
        ),
        refusal,
    )
}

// ------------------------------------------------------------------ vectors

const VALID: &str = "imports [A.B, \"a b\"]

definition f.{u, v} : @s := @l {
  class: false,
  generated: false,
  policy: exact{axioms: []},
  transparency: semireducible
}
theorem \"g h\" : @t {class: false, generated: false, policy: exact{axioms: []}, transparency: semireducible}
inductive d {derived: true}

@s := Sort (max(u, imax(v, 0)) + 2)
@f := fun (x : @s) => fun {y : Sort u+1} => fun {{z : Sort 0}} => fun [w : Sort 1] => x y z w #7 const \"x\" Nat.succ.{u, 0}
@l := let a : @s := @f; have _ : Sort 0 := 0; proof(a) \"s\\n\\u00e9\" nat \"007\" (Sort 0)
@p := (b : Sort 0) -> @s -> b
@t := proj Prod \"fst\" 0 (annot [b := bool true, c := bool false, n := nat 7, m := nat \"07\", i := int -7, j := int \"-0\", s := str \"q\", k := name \"x y\"] @l)
";

#[allow(clippy::too_many_lines)]
fn named() -> Vec<(&'static str, String)> {
    let mut out: Vec<(&'static str, String)> = Vec::new();
    let mut add = |label: &'static str, source: &str| out.push((label, source.to_owned()));
    add("every form", VALID);
    add("an empty module", "imports []\n");
    add("an empty text", "");
    add("no imports", "definition f : @0 := @0\n");
    add("imports without a bracket", "imports A\n");
    add("imports without a comma", "imports [A B]\n");
    add("a keyword import", "imports [fun]\n");
    add("a numeral import", "imports [1]\n");
    add("a name beginning with a digit", "imports [1a]\n");
    add("a trailing comma in imports", "imports [A,]\n");
    add("an unterminated string", "imports [\"a]");
    add("a bad escape", "imports [\"\\q\"]\n");
    add("a tab", "imports []\n\t@0 := Sort 0\n");
    add("a carriage return", "imports []\r\n");
    add("an unexpected scalar", "imports [=]\n");
    add("a lone dot", "imports [.]\n");
    add(
        "a declaration not at the margin",
        "imports []\n definition f : @0\n",
    );
    add("an unknown declaration kind", "imports []\nlemma f : @0\n");
    add("a declaration of no name", "imports []\ndefinition : @0\n");
    add(
        "universe parameters",
        "imports []\ndefinition f.{u, \"v w\"} : @0\n@0 := Sort u\n",
    );
    add(
        "universe parameters unclosed",
        "imports []\ndefinition f.{u, v : @0\n",
    );
    add(
        "a root that is no reference",
        "imports []\ndefinition f : x\n",
    );
    add(
        "a value root that is no reference",
        "imports []\ndefinition f : @0 := x\n@0 := Sort 0\n",
    );
    add(
        "a root too large",
        "imports []\ndefinition f : @99999999999999999999\n",
    );
    add(
        "a derived row with parameters",
        "imports []\ninductive f.{u}\n",
    );
    add(
        "members that are no object",
        "imports []\ndefinition f : @0 [1]\n@0 := Sort 0\n",
    );
    add("members unclosed", "imports []\ndefinition f : @0 {a: 1\n");
    add(
        "members with a brace in a string",
        "imports []\ndefinition f : @0 {a: \"}\", b: \"{\"}\n@0 := Sort 0\n",
    );
    add(
        "members malformed",
        "imports []\ndefinition f : @0 {a 1}\n@0 := Sort 0\n",
    );
    add(
        "members that write the kind",
        "imports []\ndefinition f : @0 {kind: x, name: y, levels: [z], type: 9}\n@0 := Sort 0\n",
    );
    add(
        "members on the next line",
        "imports []\ndefinition f : @0\n{a: 1}\n",
    );
    add(
        "a declaration after a node",
        "imports []\n@0 := Sort 0\ndefinition f : @0\n",
    );
    add(
        "a statement not at the margin",
        "imports []\n@0 := Sort 0 @1 := Sort 0\n",
    );
    add("a label of the wrong number", "imports []\n@1 := Sort 0\n");
    add(
        "a statement that names an earlier node",
        "imports []\n@0 := Sort 0\n@1 := @0\n",
    );
    add(
        "a statement that ends with an earlier node",
        "imports []\n@0 := Sort 0\n@2 := (Sort 1) @0\n",
    );
    add("a reference to a later node", "imports []\n@0 := @1\n");
    add(
        "a reference to the node being written",
        "imports []\n@0 := @0\n",
    );
    add(
        "a reference too large",
        "imports []\n@0 := @99999999999999999999\n",
    );
    add(
        "a reference to an unknown name",
        "imports []\n@0 := @nobody\n",
    );
    add(
        "named statements",
        "imports []\ndefinition f : @ty := @val\n@ty := Sort 1\n@val := fun (x : @ty) => @ty\n",
    );
    add(
        "a statement named twice",
        "imports []\n@a := Sort 0\n@a := Sort 1\n",
    );
    add(
        "a root naming no statement",
        "imports []\ndefinition f : @missing\n@0 := Sort 0\n",
    );
    add(
        "a second root naming no statement",
        "imports []\ndefinition f : @0\ndefinition g : @missing\n@0 := Sort 0\n",
    );
    add(
        "a statement without its assignment",
        "imports []\n@0 Sort 0\n",
    );
    add("a label of no number", "imports []\n@ := Sort 0\n");
    add(
        "a number followed by letters",
        "imports []\n@0x := Sort 0\n",
    );
    add(
        "a bound variable followed by letters",
        "imports []\n@0 := #0x\n",
    );
    add(
        "a bound variable too large",
        "imports []\n@0 := #99999999999999999999\n",
    );
    add("a numeral with a leading zero", "imports []\n@0 := 007\n");
    add(
        "a term that begins with a keyword",
        "imports []\n@0 := max\n",
    );
    add("the underscore as a term", "imports []\n@0 := _\n");
    add(
        "a bound variable given levels",
        "imports []\n@0 := Sort 0\n@1 := fun (x : @0) => x.{u}\n",
    );
    add(
        "a binder without its colon",
        "imports []\n@0 := fun (x @0) => x\n",
    );
    add(
        "a binder unclosed",
        "imports []\n@0 := Sort 0\n@1 := fun (x : @0 => x\n",
    );
    add(
        "a strict binder half closed",
        "imports []\n@0 := Sort 0\n@1 := fun {{x : @0} => x\n",
    );
    add("no binder after fun", "imports []\n@0 := fun x => x\n");
    add(
        "fun without its arrow",
        "imports []\n@0 := Sort 0\n@1 := fun (x : @0) x\n",
    );
    add(
        "a binder arrow missing",
        "imports []\n@0 := Sort 0\n@1 := (x : @0) x\n",
    );
    add("let without its colon", "imports []\n@0 := let x := 0; x\n");
    add(
        "let without its assignment",
        "imports []\n@0 := Sort 0\n@1 := let x : @0 0; x\n",
    );
    add(
        "let without its semicolon",
        "imports []\n@0 := Sort 0\n@1 := let x : @0 := 0 x\n",
    );
    add("an unclosed parenthesis", "imports []\n@0 := (Sort 0\n");
    add(
        "a proof of a reference",
        "imports []\n@0 := Sort 0\n@1 := proof(@0)\n",
    );
    add("a proof unclosed", "imports []\n@0 := proof(Sort 0\n");
    add("nat without a string", "imports []\n@0 := nat 7\n");
    add("const without a string", "imports []\n@0 := const x\n");
    add("a level of no form", "imports []\n@0 := Sort [\n");
    add("a level too large", "imports []\n@0 := Sort 1025\n");
    add("a level raised too far", "imports []\n@0 := Sort u+1025\n");
    add("a level at the bound", "imports []\n@0 := Sort u+1024\n");
    add(
        "max without a parenthesis",
        "imports []\n@0 := Sort max u\n",
    );
    add("max without a comma", "imports []\n@0 := Sort max(u v)\n");
    add("max unclosed", "imports []\n@0 := Sort imax(u, v\n");
    add(
        "a level parenthesis unclosed",
        "imports []\n@0 := Sort (u\n",
    );
    add("levels unclosed", "imports []\n@0 := Nat.succ.{u, v\n");
    add("a keyword level", "imports []\n@0 := Sort true\n");
    add(
        "an annotation without a bracket",
        "imports []\n@0 := annot x @0\n",
    );
    add(
        "an annotation key without its assignment",
        "imports []\n@0 := Sort 0\n@1 := annot [a bool true] @0\n",
    );
    add(
        "an annotation value of no type",
        "imports []\n@0 := Sort 0\n@1 := annot [a := 7] @0\n",
    );
    add(
        "an annotation of an unknown type",
        "imports []\n@0 := Sort 0\n@1 := annot [a := real 7] @0\n",
    );
    add(
        "an annotation Boolean of no value",
        "imports []\n@0 := Sort 0\n@1 := annot [a := bool maybe] @0\n",
    );
    add(
        "an annotation number of no digits",
        "imports []\n@0 := Sort 0\n@1 := annot [a := nat x] @0\n",
    );
    add(
        "an annotation natural with a minus",
        "imports []\n@0 := Sort 0\n@1 := annot [a := nat -1] @0\n",
    );
    add(
        "an annotation integer of minus zero",
        "imports []\n@0 := Sort 0\n@1 := annot [a := int -0] @0\n",
    );
    add(
        "an annotation integer with a leading zero",
        "imports []\n@0 := Sort 0\n@1 := annot [a := int -07] @0\n",
    );
    add(
        "an annotation string of no string",
        "imports []\n@0 := Sort 0\n@1 := annot [a := str x] @0\n",
    );
    add(
        "an annotation unclosed",
        "imports []\n@0 := Sort 0\n@1 := annot [a := bool true @0\n",
    );
    add(
        "a projection of no index",
        "imports []\n@0 := Sort 0\n@1 := proj S n x @0\n",
    );
    add(
        "a projection index with a leading zero",
        "imports []\n@0 := Sort 0\n@1 := proj S n 01 @0\n",
    );
    add(
        "an application across lines",
        "imports []\n@0 := Sort 0\n@1 := f @0\n@2 := g\n",
    );
    add(
        "an arrow binder with no name",
        "imports []\n@0 := Sort 0\n@1 := (_ : @0) -> @0\n",
    );
    add(
        "a name shadowed by an inner binder",
        "imports []\n@0 := Sort 0\n@1 := fun (x : @0) => fun (x : @0) => x\n",
    );
    add(
        "a name of an outer binder past an arrow",
        "imports []\n@0 := Sort 0\n@f := fun (x : @0) => @0 -> x\n",
    );
    add("a statement that is no term", "imports []\n@0 := )\n");
    let deep = format!(
        "imports []\n@0 := {}Sort 0{}\n",
        "(".repeat(260),
        ")".repeat(260)
    );
    out.push(("nested beyond the bound", deep));
    let at_bound = format!(
        "imports []\n@0 := {}Sort 0{}\n",
        "(".repeat(120),
        ")".repeat(120)
    );
    out.push(("nested within the bound", at_bound));
    out
}

const REFUSALS: &[&str] = &[
    "a tab or a carriage return is not part of the notation",
    "a core module begins with its `imports`",
    "`@` and `#` are followed by a number or, after `@`, a name",
    "a number ends where its text does",
    "a name does not begin with a digit; quote it",
    "unexpected `",
    "unterminated string",
    "is a keyword; quote it to write a name",
    "the number is too large",
    "a numeral has no redundant leading zero",
    "nested more deeply than the notation reads",
    "a level is raised by at most 1024 at once",
    "a level is at most 1024",
    "expected a universe level",
    "expected a binder",
    "is a bound variable and has no universe levels",
    "is not a node that has been written",
    "no statement written before this is named",
    "is a keyword; a term that begins with it is written in parentheses",
    "`_` is the name of a binder, not a constant",
    "`proof` marks a node written inside it, not a reference",
    "expected a term",
    "expected the type of the value",
    "expected `true` or `false`",
    "is not the type of an annotation value",
    "expected a number",
    "the braces of the declaration are not closed",
    "does not begin a declaration or a node",
    // `the members of a declaration are an object` is kept in the module as the
    // oracle writes it but no text reaches it: the members are the text from a
    // `{` to its matching `}`, which the generic reader reads as an object or
    // refuses.
    "a row without a type states no universe parameters",
    "expected `@N`, the type",
    "a statement defines a node, not another name for one",
    // `a statement defines the node its term ends with` is kept in the module
    // as the oracle writes it but no text reaches it: a term's own node is the
    // last one reading it numbers, and a term that is an earlier node is the
    // refusal above.
    "the label is the number of the node",
    "names two statements",
    "a statement ends at the end of its line",
    "a declaration or a node begins its line",
    "the declarations come before the nodes",
    "no statement is named",
];

// ------------------------------------------------------------------- seeded

const PIECES: &[&str] = &[
    "(", ")", "{", "}", "[", "]", ",", ":", ":=", ";", "->", "=>", "+", "-", ".{", "@0", "@1",
    "@a", "#0", "0", "07", "\"s\"", "x", "fun", "let", "have", "Sort", "proj", "annot", "const",
    "max", "proof", "_", "\n", " ", "\t", "Nat.succ",
];

fn edited(seeded: &mut Seeded, source: &str) -> String {
    let mut scalars: Vec<char> = source.chars().collect();
    for _ in 0..=seeded.below(2) {
        let at = seeded.below(scalars.len() + 1);
        let piece: Vec<char> = seeded.pick(PIECES).chars().collect();
        match seeded.below(3) {
            0 if at < scalars.len() => {
                scalars.remove(at);
            }
            1 => {
                drop(scalars.splice(at..at, piece));
            }
            _ if at < scalars.len() => {
                drop(scalars.splice(at..=at, piece));
            }
            _ => scalars.extend(piece),
        }
    }
    scalars.into_iter().collect()
}

fn seeded_runs(count: usize, bases: &[String], seed: u64) -> Vec<Run> {
    let mut seeded = Seeded(seed);
    (0..count)
        .map(|_| {
            let base = seeded.pick(bases).clone();
            run(&edited(&mut seeded, &base)).0
        })
        .collect()
}

/// The committed core modules written in the notation: the text of each body.
fn committed() -> Vec<(String, String)> {
    let mut out = Vec::new();
    for (path, content) in super::scan::sources() {
        if content.len() > RUSTC_BYTES {
            continue;
        }
        let Some(found) = locate(&content) else {
            continue;
        };
        if found.kind.is(&CORE) && found.data.is_none() {
            out.push((path, content[found.body.0..found.body.1].to_owned()));
        }
    }
    out
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
        label: "a tab is white space",
        site: "(if (if (code =? 9) then true else (code =? 13)) then",
        replacement: "(if (code =? 13) then",
        vector: "a tab",
    },
    Plant {
        label: "a carriage return is white space",
        site: "(if (if (code =? 9) then true else (code =? 13)) then",
        replacement: "(if (code =? 9) then",
        vector: "a carriage return",
    },
    Plant {
        label: "a level may be raised by 1025",
        site: "(if (1024 <? (project got value)) then (lfail machine (failure (project next start)",
        replacement: "(if (1025 <? (project got value)) then (lfail machine (failure (project next start)",
        vector: "a level raised too far",
    },
    Plant {
        label: "a level numeral may be 1025",
        site: "(if (1024 <? (project got value)) then (lfail machine (failure (project tok start) (lit \"a level is at most 1024\")))",
        replacement: "(if (1025 <? (project got value)) then (lfail machine (failure (project tok start) (lit \"a level is at most 1024\")))",
        vector: "a level too large",
    },
    Plant {
        label: "the nesting bound is 300",
        site: "(if (256 <? depth) then",
        replacement: "(if (300 <? depth) then",
        vector: "nested beyond the bound",
    },
    Plant {
        label: "a strict binder is implicit",
        site: "(if ((project following kind) =? 2) then (ctor Result.ok [(Nat * Cur), SurfaceLex::Failure] (2, (moved following)))",
        replacement: "(if false then (ctor Result.ok [(Nat * Cur), SurfaceLex::Failure] (2, (moved following)))",
        vector: "every form",
    },
    Plant {
        label: "an instance binder is explicit",
        site: "(ctor Result.ok [(Nat * Cur), SurfaceLex::Failure] (3, after))",
        replacement: "(ctor Result.ok [(Nat * Cur), SurfaceLex::Failure] (0, after))",
        vector: "every form",
    },
    Plant {
        label: "the strict close is one brace",
        site: "(if (info =? 2) then (match (expectTok text (project step cur) 3 (lit \"`}}`\"))",
        replacement: "(if false then (match (expectTok text (project step cur) 3 (lit \"`}}`\"))",
        vector: "a strict binder half closed",
    },
    Plant {
        label: "have is let",
        site: "(cons (ctor Held.HLet (project named text) (same word (lit \"have\")))",
        replacement: "(cons (ctor Held.HLet (project named text) false)",
        vector: "every form",
    },
    Plant {
        label: "a bound name is a constant",
        site: "(match (resolveName (project machine scope) word)",
        replacement: "(match (ctor Option.none [Nat])",
        vector: "every form",
    },
    Plant {
        label: "an arrow binds no scope",
        site: "(cons (ctor Job.JTerm (depth + 1)) (cons (ctor Job.JArrowDone) rest)) (project machine vals) (project machine held) (cons (lit \"\") (project machine scope)))",
        replacement: "(cons (ctor Job.JTerm (depth + 1)) (cons (ctor Job.JArrowDone) rest)) (project machine vals) (project machine held) (project machine scope))",
        vector: "a name of an outer binder past an arrow",
    },
    Plant {
        label: "an application continues on the next line",
        site: "(ctor Result.ok [Bool, SurfaceLex::Failure] (if (project tok col0) then false else",
        replacement: "(ctor Result.ok [Bool, SurfaceLex::Failure] (if false then false else",
        vector: "an application across lines",
    },
    Plant {
        label: "a reference may name the node being written",
        site: "(if ((project machine count) <=? target) then",
        replacement: "(if ((project machine count) <? target) then",
        vector: "a reference to the node being written",
    },
    Plant {
        label: "a proof may mark a reference",
        site: "(if (if (both ((inner + 1) =? (project machine count)) (before <=? inner)) then false else true) then",
        replacement: "(if false then",
        vector: "a proof of a reference",
    },
    Plant {
        label: "the label need not be the number",
        site: "(if (both isRef (if (target =? defined) then false else true)) then",
        replacement: "(if false then",
        vector: "a label of the wrong number",
    },
    Plant {
        label: "a statement may name an earlier node",
        site: "(if (defined <? before) then",
        replacement: "(if false then",
        vector: "a statement that names an earlier node",
    },
    Plant {
        label: "a name may name two statements",
        site: "(labelOf (project ran labels) named)",
        replacement: "(labelOf (nil Lbl) named)",
        vector: "a statement named twice",
    },
    Plant {
        label: "a statement may continue its line",
        site: "(if (both (if ((project after kind) =? 21) then false else true) (if (project after col0) then false else true)) then",
        replacement: "(if false then",
        vector: "a statement not at the margin",
    },
    Plant {
        label: "members on the next line belong to the row",
        site: "(if (both ((project brace kind) =? 2) (if (project brace col0) then false else true)) then",
        replacement: "(if ((project brace kind) =? 2) then",
        vector: "members on the next line",
    },
    Plant {
        label: "a brace in a string counts",
        site: "(if (code =? 34) then (match (stringEnd text position)",
        replacement: "(if false then (match (stringEnd text position)",
        vector: "members with a brace in a string",
    },
    Plant {
        label: "the members keep the written kind",
        site: "(withMember (withMember (first members) (lit \"kind\") (ctor LexLeanStd.Json::Json.JStr word))",
        replacement: "(withMember (withMember (first members) (lit \"kinds\") (ctor LexLeanStd.Json::Json.JStr word))",
        vector: "members that write the kind",
    },
    Plant {
        label: "a derived row may state parameters",
        site: "(if (0 <? (prim length levels : Nat)) then (ctor Result.error",
        replacement: "(if false then (ctor Result.error",
        vector: "a derived row with parameters",
    },
    Plant {
        label: "a stated row has no levels member",
        site: "members := (withMember named (lit \"levels\") (ctor LexLeanStd.Json::Json.JArr (chainOf levels)))",
        replacement: "members := named",
        vector: "universe parameters",
    },
    Plant {
        label: "a named root is never found",
        site: "(match (labelOf labels label)",
        replacement: "(match (labelOf (nil Lbl) label)",
        vector: "named statements",
    },
    Plant {
        label: "a declaration after a node is read",
        site: "(if (project top statements) then (topFail top",
        replacement: "(if false then (topFail top",
        vector: "a declaration after a node",
    },
    Plant {
        label: "a number may run into a name",
        site: "(if (nameCharAt text stop) then (refuse stop",
        replacement: "(if false then (refuse stop",
        vector: "a number followed by letters",
    },
    Plant {
        label: "a name may begin with a digit",
        site: "(if (nameCharAt text stop) then (refuse at (lit \"a name does not begin with a digit; quote it\"))",
        replacement: "(if false then (refuse at (lit \"a name does not begin with a digit; quote it\"))",
        vector: "a name beginning with a digit",
    },
    Plant {
        label: "the dot of a name before its levels stays",
        site: "(if (both ((codeAt text (prim subtract stop 1 : Nat)) =? 46) ((codeAt text stop) =? 123)) then (prim subtract stop 1 : Nat) else stop)",
        replacement: "stop",
        vector: "every form",
    },
    Plant {
        label: "an integer annotation may be minus zero",
        site: "(if (if (both (1 <? (prim length digits : Nat)) (LexLeanStd.Text::startsWith digits (lit \"0\"))) then true else (same digits (lit \"0\"))) then",
        replacement: "(if (both (1 <? (prim length digits : Nat)) (LexLeanStd.Text::startsWith digits (lit \"0\"))) then",
        vector: "an annotation integer of minus zero",
    },
];

fn plants(vectors: &[(&'static str, String)]) {
    let mut missed = Vec::new();
    for plant in PLANTS {
        let source = &vectors
            .iter()
            .find(|(label, _)| *label == plant.vector)
            .unwrap_or_else(|| panic!("{}: no vector `{}`", plant.label, plant.vector))
            .1;
        let planted = support::P::bootstrap();
        let module = planted.read_raw(SOURCE);
        assert_eq!(
            module.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(SOURCE, &module.replacen(plant.site, plant.replacement, 1));
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[0].0);
        match super::evaluate(&low, &[run(source).0], 1).first() {
            Some(first) => eprintln!(
                "BS-40: plant detected: {}: {}",
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
            "BS-40: {}: largest certificate part {largest} bytes",
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
        checked_units.units.contains_key("CoreRead"),
        "CoreRead is a linked module of the project"
    );
    eligibility_of(&project, "CoreRead", MODULE, &ROOTS);

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|(label, _)| *label).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let mut messages = Vec::new();
    let mut named_runs = Vec::new();
    for (_, source) in &vectors {
        let (outcome, refusal) = run(source);
        named_runs.push(outcome);
        messages.extend(refusal);
    }
    let unreached: Vec<&str> = REFUSALS
        .iter()
        .filter(|refusal| !messages.iter().any(|message| message.contains(*refusal)))
        .copied()
        .collect();
    assert!(
        unreached.is_empty(),
        "no vector reaches the refusals {unreached:?}"
    );

    let modules = committed();
    assert!(
        modules.len() >= 100,
        "the committed core modules in the notation are {}",
        modules.len()
    );
    let mut evaluator: Vec<Run> = Vec::new();
    let mut compiled: Vec<Run> = Vec::new();
    let mut bases: Vec<String> = Vec::new();
    for (index, (path, body)) in modules.iter().enumerate() {
        // The reader the root is held to is the oracle's: what it reads, the
        // decoding makes the module `parse_module` returns.
        assert!(
            parse_module(body).is_ok(),
            "{path}: a committed module reads"
        );
        let (outcome, refusal) = run(body);
        assert!(refusal.is_none(), "{path}: a committed module reads");
        if body.len() <= EVALUATOR_BYTES && index % EVALUATOR_EVERY == 0 {
            evaluator.push(outcome.clone());
            bases.push(body.clone());
        }
        compiled.push(outcome);
    }
    bases.push(VALID.to_owned());
    let normative = support::normative_host();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (
            seeded_runs(EVALUATOR_SEEDED, &bases, 0x0B55_0040),
            seeded_runs(RUSTC_SEEDED, &bases, 0x0B55_1040),
        )
    } else {
        (Vec::new(), Vec::new())
    };
    eprintln!(
        "BS-40: {} vectors, {} committed core modules ({} under the evaluator, {} under rustc), {} and {} seeded edits ({:?})",
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
            "readCoreModule and the Rust oracle disagree: {disagreements:?}"
        );
        eprintln!(
            "BS-40: readCoreModule: the evaluator agrees on {} runs ({:?})",
            corpus.len(),
            started.elapsed()
        );
        plants(&vectors);
    } else {
        eprintln!("BS-40: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut all = named_runs;
    all.extend(compiled);
    all.extend(rustc_seeded);
    let total = rustc_differential(
        "BS-40",
        &[(ROOTS[0].0, &lowered[ROOTS[0].0], all.as_slice())],
        "oracle",
    );
    eprintln!("BS-40: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-40") {
        return;
    }
    export(&lowered);
    eprintln!("BS-40: passed in {:?}", started.elapsed());
}
