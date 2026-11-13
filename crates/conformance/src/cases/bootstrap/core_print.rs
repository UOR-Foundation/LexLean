//! `BS-39` (SPEC.md §17.23, §17.21.8): the printer of the core notation as
//! language-1.2 source, `LexLeanBoot.CorePrint`, held to the byte to
//! `lexlean::surface::core::print_module`.
//!
//! The root takes a core module as `BS-38`'s `CoreJson` takes it. It checks
//! that the notation can number the module, then decides in one pass over the
//! node table, in the order of the nodes, which nodes are written inside the
//! node that uses them: a stack holds the nodes still to be written inside another,
//! and a node takes a child from the top of the stack when the child is used
//! once, ends right before the nodes the node has taken, and is less than 48
//! deep. The same pass builds each node's term with the terms it takes; a term
//! names its bound variables and its constants by index and by name, which a
//! second walk resolves against the binders a statement writes, so the term of
//! a node does not depend on where it is written.
//!
//! Each planted defect names the one vector that catches it.

use std::collections::{BTreeMap, BTreeSet};

use lexlean::calculus::Value;
use lexlean::ir::core::{
    CoreBinderInfo, CoreEqnInfo, CoreInductive, CoreModule, CoreNode, CoreStructure,
    CoreStructureField,
};
use lexlean::surface::core::print_module;
use serde_json::Value as Json2;

use super::core_json::{committed, module_value};
use super::rust_validate::{eligibility_of, rustc_differential};
use super::{nat_val, text, Run, Seeded};
use crate::production_export as harness;
use crate::support;

const MODULE: &str = "LexLeanBoot.CorePrint";
const SOURCE: &str = "src/CorePrint.lex.tex";
const ROOTS: [(&str, &[&str]); 1] = [("printCoreModule", &["allocation", "overflow", "recursion"])];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// Committed core modules whose notation is at most this many bytes run under
/// the evaluator (one in `EVALUATOR_EVERY`); every committed module the reader
/// of `BS-38`'s case keeps runs under rustc.
const EVALUATOR_BYTES: usize = 9_000;
const EVALUATOR_EVERY: usize = 4;
/// Seeded edits of the committed modules under the evaluator and under rustc.
const EVALUATOR_SEEDED: usize = 25;
const RUSTC_SEEDED: usize = 500;

fn run(module: &CoreModule) -> (Run, Option<String>) {
    let (value, refusal) = match print_module(module) {
        Ok(printed) => (
            Value::Ok {
                value: Box::new(text(&printed)),
            },
            None,
        ),
        Err(failure) => (
            Value::Error {
                value: Box::new(Value::Adt {
                    constructor: 0,
                    fields: vec![nat_val(failure.offset), text(&failure.message)],
                }),
            },
            Some(failure.message),
        ),
    };
    (
        (
            vec![module_value(module)],
            serde_json::to_value(value).expect("a value is JSON"),
        ),
        refusal,
    )
}

// ------------------------------------------------------------------ vectors

fn nodes(text: &str) -> Vec<CoreNode> {
    serde_json::from_str(text).unwrap_or_else(|error| panic!("{text}: {error}"))
}

/// A chain of `count` annotations, each holding the one before it.
fn chain(count: usize) -> Vec<CoreNode> {
    let mut out = vec![CoreNode::Sort {
        l: lexlean::ir::core::CoreLevel::Zero,
    }];
    for index in 1..count {
        out.push(CoreNode::Mdata {
            m: Vec::new(),
            x: index - 1,
        });
    }
    out
}

/// An application of a long-named constant to `count` long-named arguments.
fn long_application(count: usize) -> Vec<CoreNode> {
    let mut out = vec![CoreNode::Const {
        n: "Some.Rather.Long.Function.Name".to_owned(),
        u: Vec::new(),
    }];
    for index in 0..count {
        let head = out.len() - 1;
        out.push(CoreNode::Const {
            n: format!("argument.with.a.long.name.number{index}"),
            u: Vec::new(),
        });
        out.push(CoreNode::App {
            f: head,
            x: head + 1,
        });
    }
    out
}

#[allow(clippy::too_many_lines)]
fn named(base: &CoreModule) -> Vec<(String, CoreModule)> {
    let mut out: Vec<(String, CoreModule)> = Vec::new();
    let mut add = |label: &str, change: &dyn Fn(&mut CoreModule)| {
        let mut module = base.clone();
        change(&mut module);
        out.push((label.to_owned(), module));
    };
    let table = |text: &'static str| {
        move |module: &mut CoreModule| {
            module.declarations.clear();
            module.proof_nodes.clear();
            module.nodes = nodes(text);
        }
    };
    add("the smallest committed module", &|_| {});
    add("another spec", &|module| {
        module.spec = "lexlean/core-module/1".to_owned()
    });
    add(
        "a node pointing to itself",
        &table(r#"[{"k":"a","f":0,"x":0}]"#),
    );
    add(
        "a node pointing forward",
        &table(r#"[{"k":"s","l":{"k":"z"}},{"k":"a","f":0,"x":5}]"#),
    );
    add("a repeated proof node", &|module| {
        module.proof_nodes = vec![0, 0];
    });
    add("a proof node outside", &|module| {
        module.proof_nodes = vec![module.nodes.len()];
    });
    add("a root outside", &|module| {
        let count = module.nodes.len();
        module.declarations[0].r#type = Some(count + 3);
    });
    add("an equation root outside", &|module| {
        let count = module.nodes.len();
        module.declarations[0].eqn_info = Some(CoreEqnInfo {
            rec_arg_pos: 0,
            declarations: vec!["f".to_owned()],
            num_fixed: 0,
            perms: Vec::new(),
            rev_deps: Vec::new(),
            r#type: 0,
            value: count,
            levels: Vec::new(),
        });
    });
    add("an auto parameter outside", &|module| {
        let count = module.nodes.len();
        module.declarations[0].inductive = Some(CoreInductive {
            num_params: 0,
            num_indices: 0,
            constructors: vec!["S.mk".to_owned()],
            structure: Some(CoreStructure {
                fields: vec![CoreStructureField {
                    name: "a".to_owned(),
                    projection: "S.a".to_owned(),
                    subobject: None,
                    binder: CoreBinderInfo::Explicit,
                    auto_param: Some(count + 1),
                }],
                parents: Vec::new(),
            }),
        });
    });
    add("an auto parameter inside", &|module| {
        module.declarations[0].inductive = Some(CoreInductive {
            num_params: 0,
            num_indices: 0,
            constructors: vec!["S.mk".to_owned()],
            structure: Some(CoreStructure {
                fields: vec![CoreStructureField {
                    name: "a".to_owned(),
                    projection: "S.a".to_owned(),
                    subobject: Some("P".to_owned()),
                    binder: CoreBinderInfo::Implicit,
                    auto_param: Some(0),
                }],
                parents: Vec::new(),
            }),
        });
    });
    add(
        "a surrogate scalar",
        &table(r#"[{"k":"t","v":[104,55296]},{"k":"t","v":[1114112]}]"#),
    );
    add(
        "a scalar beyond Unicode",
        &table(r#"[{"k":"t","v":[104]},{"k":"t","v":[1114112,55296]}]"#),
    );
    add(
        "a bound variable by name",
        &table(
            r#"[{"k":"s","l":{"k":"z"}},{"k":"b","i":0},{"k":"l","n":"x","b":"e","t":0,"v":1}]"#,
        ),
    );
    add(
        "a bound variable shadowed",
        &table(
            r#"[{"k":"s","l":{"k":"z"}},{"k":"b","i":1},{"k":"l","n":"x","b":"e","t":0,"v":1},{"k":"l","n":"x","b":"e","t":0,"v":2}]"#,
        ),
    );
    add(
        "a bound variable of no name",
        &table(
            r#"[{"k":"s","l":{"k":"z"}},{"k":"b","i":0},{"k":"l","n":"a b","b":"e","t":0,"v":1},{"k":"b","i":4}]"#,
        ),
    );
    add(
        "an empty binder name",
        &table(r#"[{"k":"s","l":{"k":"z"}},{"k":"b","i":0},{"k":"p","n":"","b":"i","t":0,"v":1}]"#),
    );
    add(
        "an arrow",
        &table(
            r#"[{"k":"s","l":{"k":"z"}},{"k":"s","l":{"k":"s","a":{"k":"z"}}},{"k":"p","n":"","b":"e","t":0,"v":1}]"#,
        ),
    );
    add(
        "an arrow whose domain is a binder form",
        &table(
            r#"[{"k":"s","l":{"k":"z"}},{"k":"b","i":0},{"k":"l","n":"y","b":"e","t":0,"v":1},{"k":"s","l":{"k":"z"}},{"k":"p","n":"","b":"e","t":2,"v":3}]"#,
        ),
    );
    add(
        "a constant named like a binder",
        &table(
            r#"[{"k":"s","l":{"k":"z"}},{"k":"c","n":"x","u":[]},{"k":"l","n":"x","b":"e","t":0,"v":1}]"#,
        ),
    );
    add(
        "constants with levels and quotes",
        &table(
            r#"[{"k":"c","n":"a b","u":[{"k":"s","a":{"k":"z"}}]},{"k":"c","n":"Nat.add","u":[{"k":"p","a":"u"},{"k":"m","a":[{"k":"z"},{"k":"p","a":"v"}]}]},{"k":"c","n":"fun","u":[]}]"#,
        ),
    );
    add(
        "an application argument",
        &table(
            r#"[{"k":"c","n":"f","u":[]},{"k":"c","n":"g","u":[]},{"k":"c","n":"a","u":[]},{"k":"a","f":1,"x":2},{"k":"a","f":0,"x":3},{"k":"c","n":"b","u":[]},{"k":"a","f":4,"x":5}]"#,
        ),
    );
    add(
        "a sort as an argument",
        &table(r#"[{"k":"c","n":"f","u":[]},{"k":"s","l":{"k":"z"}},{"k":"a","f":0,"x":1}]"#),
    );
    add(
        "a binder form as a head and an argument",
        &table(
            r#"[{"k":"s","l":{"k":"z"}},{"k":"b","i":0},{"k":"l","n":"x","b":"e","t":0,"v":1},{"k":"s","l":{"k":"z"}},{"k":"b","i":0},{"k":"l","n":"y","b":"e","t":3,"v":4},{"k":"a","f":2,"x":5}]"#,
        ),
    );
    add(
        "every binder visibility",
        &table(
            r#"[{"k":"s","l":{"k":"z"}},{"k":"b","i":0},{"k":"l","n":"a","b":"i","t":0,"v":1},{"k":"s","l":{"k":"z"}},{"k":"l","n":"b","b":"s","t":3,"v":2},{"k":"s","l":{"k":"z"}},{"k":"p","n":"c","b":"c","t":5,"v":4}]"#,
        ),
    );
    add(
        "let and have",
        &table(
            r#"[{"k":"s","l":{"k":"z"}},{"k":"c","n":"v","u":[]},{"k":"b","i":0},{"k":"e","n":"x","t":0,"v":1,"body":2,"d":false},{"k":"s","l":{"k":"z"}},{"k":"c","n":"w","u":[]},{"k":"e","n":"","t":4,"v":5,"body":3,"d":true}]"#,
        ),
    );
    add(
        "natural literals",
        &table(
            r#"[{"k":"n","v":"42"},{"k":"n","v":"0"},{"k":"n","v":"007"},{"k":"n","v":""},{"k":"n","v":"1x"}]"#,
        ),
    );
    add(
        "a string literal",
        &table(r#"[{"k":"t","v":[97,34,92,10,9,1,233,128512]},{"k":"t","v":[]}]"#),
    );
    add(
        "an annotation of every value",
        &table(
            r#"[{"k":"s","l":{"k":"z"}},{"k":"d","m":[{"n":"b","v":{"t":"b","v":true}},{"n":"c","v":{"t":"b","v":false}},{"n":"n","v":{"t":"n","v":"7"}},{"n":"n0","v":{"t":"n","v":"07"}},{"n":"i","v":{"t":"i","v":"-7"}},{"n":"i0","v":{"t":"i","v":"-0"}},{"n":"s","v":{"t":"s","v":"a\"b"}},{"n":"m","v":{"t":"m","v":"Nat.add"}},{"n":"a b","v":{"t":"m","v":"x y"}}],"x":0}]"#,
        ),
    );
    add(
        "a projection",
        &table(
            r#"[{"k":"c","n":"p","u":[]},{"k":"j","n":"Prod.fst","s":"Prod","i":0,"x":0},{"k":"c","n":"q","u":[]},{"k":"c","n":"r","u":[]},{"k":"a","f":2,"x":3},{"k":"j","n":"a b","s":"S","i":12,"x":4}]"#,
        ),
    );
    add(
        "sorts and levels",
        &table(
            r#"[{"k":"s","l":{"k":"s","a":{"k":"s","a":{"k":"s","a":{"k":"z"}}}}},{"k":"s","l":{"k":"s","a":{"k":"s","a":{"k":"p","a":"u"}}}},{"k":"s","l":{"k":"m","a":[{"k":"p","a":"u"},{"k":"s","a":{"k":"z"}}]}},{"k":"s","l":{"k":"i","a":[{"k":"s","a":{"k":"m","a":[{"k":"z"},{"k":"z"}]}},{"k":"p","a":"a b"}]}},{"k":"s","l":{"k":"z"}}]"#,
        ),
    );
    add("proof nodes", &|module| {
        module.declarations.clear();
        module.nodes = nodes(
            r#"[{"k":"c","n":"f","u":[]},{"k":"c","n":"a","u":[]},{"k":"a","f":0,"x":1},{"k":"c","n":"b","u":[]}]"#,
        );
        module.proof_nodes = vec![1, 2, 3];
    });
    add(
        "a node used twice",
        &table(r#"[{"k":"c","n":"f","u":[]},{"k":"a","f":0,"x":0}]"#),
    );
    add("a deep chain", &|module| {
        module.declarations.clear();
        module.proof_nodes.clear();
        module.nodes = chain(60);
    });
    add("a long statement", &|module| {
        module.declarations.clear();
        module.proof_nodes.clear();
        module.nodes = long_application(6);
    });
    add("a declaration with levels", &|module| {
        module.declarations[0].levels = Some(vec!["u".to_owned(), "a b".to_owned()]);
    });
    add("a derived declaration", &|module| {
        let declaration = &mut module.declarations[0];
        declaration.levels = None;
        declaration.r#type = None;
        declaration.value = None;
        declaration.derived = true;
    });
    add("a declaration whose name is quoted", &|module| {
        module.declarations[0].name = "a \"b\"".to_owned();
    });
    add("imports to quote", &|module| {
        module.imports = vec![
            "A.B".to_owned(),
            "a b".to_owned(),
            "fun".to_owned(),
            "x.".to_owned(),
            "_".to_owned(),
            "a01".to_owned(),
            "a.01".to_owned(),
            "é".to_owned(),
            "a\u{a0}b".to_owned(),
            "_x!?'@#".to_owned(),
            "1a".to_owned(),
        ];
    });
    add("no declarations and no imports", &|module| {
        module.declarations.clear();
        module.imports.clear();
        module.proof_nodes.clear();
        module.nodes.clear();
    });
    out
}

const REFUSALS: &[&str] = &[
    "the notation names the schema",
    "which is not earlier",
    "the proof nodes are not strictly increasing nodes of the module",
    "has a root that is not a node of the module",
    "is not a Unicode scalar value",
];

// ------------------------------------------------------------------- seeded

const PIECES: &[&str] = &["x", "", "a b", "é", "Nat", "fun", "x.", "0"];

fn edit(seeded: &mut Seeded, base: &CoreModule) -> CoreModule {
    let mut module = base.clone();
    for _ in 0..=seeded.below(3) {
        match seeded.below(6) {
            0 if !module.nodes.is_empty() => {
                let at = seeded.below(module.nodes.len());
                if let CoreNode::Const { n, .. }
                | CoreNode::Lambda { n, .. }
                | CoreNode::Forall { n, .. }
                | CoreNode::Let { n, .. } = &mut module.nodes[at]
                {
                    *n = (*seeded.pick(PIECES)).to_owned();
                }
            }
            1 if !module.nodes.is_empty() => {
                let at = seeded.below(module.nodes.len());
                if let CoreNode::App { f, .. } = &mut module.nodes[at] {
                    *f = seeded.below(at + 2);
                }
            }
            2 if !module.declarations.is_empty() => {
                let at = seeded.below(module.declarations.len());
                module.declarations[at].name = (*seeded.pick(PIECES)).to_owned();
            }
            3 => module.imports.push((*seeded.pick(PIECES)).to_owned()),
            4 if !module.nodes.is_empty() => {
                let at = seeded.below(module.nodes.len());
                if let CoreNode::BVar { i } = &mut module.nodes[at] {
                    *i = seeded.below(4);
                }
            }
            _ => module
                .proof_nodes
                .push(seeded.below(module.nodes.len() + 2)),
        }
    }
    module
}

fn seeded_runs(count: usize, bases: &[CoreModule], seed: u64) -> Vec<Run> {
    let mut seeded = Seeded(seed);
    (0..count)
        .map(|_| {
            let base = seeded.pick(bases).clone();
            run(&edit(&mut seeded, &base)).0
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
        label: "the inline height is 49",
        site: "((project top height) <? 48)",
        replacement: "((project top height) <? 49)",
        vector: "a deep chain",
    },
    Plant {
        label: "a node used twice is written inside",
        site: "((project top uses) =? 1)",
        replacement: "((project top uses) <=? 2)",
        vector: "a node used twice",
    },
    Plant {
        label: "a nearer binder does not hide a name",
        site: "(same nearer label)",
        replacement: "false",
        vector: "a bound variable shadowed",
    },
    Plant {
        label: "a constant named like a binder is bare",
        site: "(if (listed label scope) then false else true)",
        replacement: "true",
        vector: "a constant named like a binder",
    },
    Plant {
        label: "the empty binder name is quoted",
        site: "(if ((prim length text : Nat) =? 0) then (lit \"_\") else (spell text))",
        replacement: "(spell text)",
        vector: "an empty binder name",
    },
    Plant {
        label: "an explicit anonymous binder is no arrow",
        site: "(if (both ((prim length label : Nat) =? 0) (isExplicit info))",
        replacement: "(if false",
        vector: "an arrow",
    },
    Plant {
        label: "a binder form is a bare domain",
        site: "| Term.Low inner => (paren inner)\n    | Term.Atom inner => inner",
        replacement: "| Term.Low inner => inner\n    | Term.Atom inner => inner",
        vector: "an arrow whose domain is a binder form",
    },
    Plant {
        label: "an application is a bare argument",
        site: "| Term.App head argsRev => (paren (whole term))",
        replacement: "| Term.App head argsRev => (whole term)",
        vector: "an application argument",
    },
    Plant {
        label: "a sort is a bare argument",
        site: "| Term.Prefix inner => (paren inner)",
        replacement: "| Term.Prefix inner => inner",
        vector: "a sort as an argument",
    },
    Plant {
        label: "the instance binder is braced",
        site: "| CoreJson::Binder.BInstance => ((lit \"[\"), (lit \"]\")))",
        replacement: "| CoreJson::Binder.BInstance => ((lit \"{\"), (lit \"}\")))",
        vector: "every binder visibility",
    },
    Plant {
        label: "have is written let",
        site: "(if dependent then (lit \"have \") else (lit \"let \"))",
        replacement: "(lit \"let \")",
        vector: "let and have",
    },
    Plant {
        label: "a natural literal is never quoted",
        site: "(if (canonicalNatural text) then text else (cat (lit \"nat \") (SurfaceLex::quoteText text)))",
        replacement: "text",
        vector: "natural literals",
    },
    Plant {
        label: "a leading zero is canonical",
        site: "(if (LexLeanStd.Text::startsWith text (lit \"0\")) then false else true)",
        replacement: "true",
        vector: "natural literals",
    },
    Plant {
        label: "minus zero is a canonical integer",
        site: "(both (canonicalNatural digits) (if (same text (lit \"-0\")) then false else true))",
        replacement: "(canonicalNatural digits)",
        vector: "an annotation of every value",
    },
    Plant {
        label: "a successor of zero is a sum",
        site: "(if (project shown zero) then (num (project shown count))",
        replacement: "(if false then (num (project shown count))",
        vector: "sorts and levels",
    },
    Plant {
        label: "imax is written max",
        site: "(lit \"imax(\")",
        replacement: "(lit \"max(\")",
        vector: "sorts and levels",
    },
    Plant {
        label: "a redundant zero is a name",
        site: "(if (redundantZero text) then false else true)",
        replacement: "true",
        vector: "imports to quote",
    },
    Plant {
        label: "a name may end with a dot",
        site: "(if (LexLeanStd.Text::endsWith text (lit \".\")) then false else true)",
        replacement: "true",
        vector: "imports to quote",
    },
    Plant {
        label: "a keyword is a name",
        site: "(if (listed text (keywords)) then false else true)",
        replacement: "true",
        vector: "imports to quote",
    },
    Plant {
        label: "a no-break space continues a name",
        site: "(if (code <? 192) then true else ((AxiomOutput::whiteLength text position) =? 0))",
        replacement: "(if (code <? 192) then true else true)",
        vector: "imports to quote",
    },
    Plant {
        label: "a proof node is not marked",
        site: "(if proof then (ctor Term.Atom",
        replacement: "(if false then (ctor Term.Atom",
        vector: "proof nodes",
    },
    Plant {
        label: "a surrogate is a scalar",
        site: "(if (both (55296 <=? scalar) (scalar <=? 57343)) then false else true)",
        replacement: "true",
        vector: "a surrogate scalar",
    },
    Plant {
        label: "a node may point to itself",
        site: "(if (index <=? child) then",
        replacement: "(if (index <? child) then",
        vector: "a node pointing to itself",
    },
    Plant {
        label: "proof nodes may repeat",
        site: "| Option.some prior => (prior <? proof)",
        replacement: "| Option.some prior => (prior <=? proof)",
        vector: "a repeated proof node",
    },
    Plant {
        label: "an auto parameter is no root",
        site: "(pushOpt out (asNat (getIn field (lit \"auto_param\"))))",
        replacement: "out",
        vector: "an auto parameter outside",
    },
    Plant {
        label: "the type of a declaration is written with a colon only",
        site: "(cat (lit \" : @\") (num root))",
        replacement: "(cat (lit \": @\") (num root))",
        vector: "the smallest committed module",
    },
    Plant {
        label: "declaration levels are not braced",
        site: "(cat (cat (lit \".{\") (joined levels (lit \", \"))) (lit \"}\"))",
        replacement: "(joined levels (lit \", \"))",
        vector: "a declaration with levels",
    },
    Plant {
        label: "arguments are not indented",
        site: "(cons head (cons (ctor Tdoc.TNest 2 (tseq",
        replacement: "(cons head (cons (ctor Tdoc.TNest 0 (tseq",
        vector: "a long statement",
    },
    Plant {
        label: "a statement is not indented",
        site: "(cons (ctor Tdoc.TNest 2 (whole (project slot term)))",
        replacement: "(cons (whole (project slot term))",
        vector: "a long statement",
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
        match super::evaluate(&low, &[run(module).0], 1).first() {
            Some(first) => eprintln!(
                "BS-39: plant detected: {}: {}",
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
            "BS-39: {}: largest certificate part {largest} bytes",
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
        checked_units.units.contains_key("CorePrint")
            && checked_units.units.contains_key("CoreJson"),
        "CorePrint and CoreJson are linked modules of the project"
    );
    eligibility_of(&project, "CorePrint", MODULE, &ROOTS);

    let modules = committed();
    assert!(
        modules.len() >= 100,
        "the committed core modules in the notation are {}",
        modules.len()
    );
    let base = modules
        .iter()
        .min_by_key(|(_, _, size)| *size)
        .map(|(_, module, _)| module.clone())
        .expect("a committed core module");
    assert!(
        base.declarations.len() >= 2 && !base.nodes.is_empty(),
        "the smallest committed module has declarations and nodes"
    );
    let vectors = named(&base);
    let labels: BTreeSet<&str> = vectors.iter().map(|(label, _)| label.as_str()).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let mut messages = Vec::new();
    let mut named_runs = Vec::new();
    for (_, module) in &vectors {
        let (printed, refusal) = run(module);
        named_runs.push(printed);
        messages.extend(refusal);
    }
    for refusal in REFUSALS {
        assert!(
            messages.iter().any(|message| message.contains(refusal)),
            "no vector reaches the refusal `{refusal}`"
        );
    }

    let mut evaluator: Vec<Run> = Vec::new();
    let mut compiled: Vec<Run> = Vec::new();
    let mut bases: Vec<CoreModule> = Vec::new();
    for (index, (path, module, size)) in modules.iter().enumerate() {
        let (printed, refusal) = run(module);
        assert!(refusal.is_none(), "{path}: a committed module prints");
        if *size <= EVALUATOR_BYTES && index % EVALUATOR_EVERY == 0 {
            evaluator.push(printed.clone());
            bases.push(module.clone());
        }
        compiled.push(printed);
    }
    let normative = support::normative_host();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (
            seeded_runs(EVALUATOR_SEEDED, &bases, 0x0B55_0039),
            seeded_runs(RUSTC_SEEDED, &bases, 0x0B55_1039),
        )
    } else {
        (Vec::new(), Vec::new())
    };
    eprintln!(
        "BS-39: {} vectors, {} committed core modules ({} under the evaluator, {} under rustc), {} and {} seeded edits ({:?})",
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
            "printCoreModule and the Rust oracle disagree: {disagreements:?}"
        );
        eprintln!(
            "BS-39: printCoreModule: the evaluator agrees on {} runs ({:?})",
            corpus.len(),
            started.elapsed()
        );
        plants(&vectors);
    } else {
        eprintln!("BS-39: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut all = named_runs;
    all.extend(compiled);
    all.extend(rustc_seeded);
    let total = rustc_differential(
        "BS-39",
        &[(ROOTS[0].0, &lowered[ROOTS[0].0], all.as_slice())],
        "oracle",
    );
    eprintln!("BS-39: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-39") {
        return;
    }
    export(&lowered);
    eprintln!("BS-39: passed in {:?}", started.elapsed());
}
