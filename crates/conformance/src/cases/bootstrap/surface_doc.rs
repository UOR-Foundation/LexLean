//! `BS-23` (SPEC.md §17.23, §17.21): the pretty-printing algebra of the
//! authoring notation as language-1.2 source, `LexLeanBoot.SurfaceDoc`, held
//! to the byte to `lexlean::surface::doc::render`.
//!
//! `renderDoc` takes a document, a width, and the number of nodes of the
//! document, and returns the layout. The oracle keeps a stack of
//! `(indent, mode, &Doc)` and the width test `fits` walks that stack and the
//! stack below it; neither recurses over the document here. A concatenation is
//! a chain of `DSeq` cells, so that putting its parts on the stack is putting
//! two items on it, and the number of nodes is the bound of the loops, which
//! pop a node at a time. The host counts the nodes as it builds the document.
//!
//! Each planted defect names the one vector that catches it. `BS-05` and
//! `BS-11` each lost a plant when their corpus grew, because a plant caught by
//! "some run of the corpus" is only as good as that corpus.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use lexlean::artifact::canonical_json::Json;
use lexlean::calculus::Value;
use lexlean::surface::doc::{render, Doc};
use lexlean::surface::generic;
use serde_json::Value as Json2;

use super::rust_validate::{eligibility_of, rustc_differential};
use super::{nat_val, text, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.SurfaceDoc";
const SOURCE: &str = "bootstrap/src/SurfaceDoc.lex.tex";
const ROOTS: [(&str, &[&str]); 1] = [("renderDoc", &["allocation", "overflow"])];

/// The most a part of a certificate of the root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// The documents of at most this many nodes are run under the evaluator; the
/// larger ones, up to the second bound, are run under rustc only.
const EVALUATOR_NODES: usize = 400;
const RUSTC_NODES: usize = 6_000;
/// The committed JSON files read, by size in bytes, and of those the ones the
/// evaluator and rustc are given: one in this many, by a hash of the path.
const JSON_BYTES: usize = 6_000;
const EVALUATOR_EVERY: u64 = 8;
const RUSTC_EVERY: u64 = 3;
/// The widths a document from a file is laid out at: the width the printer
/// uses, a narrow one, and one no line reaches.
const WIDTHS: [usize; 3] = [100, 40, 10_000];
/// Seeded documents under the evaluator, and under rustc.
const EVALUATOR_MUTANTS: usize = 500;
const RUSTC_MUTANTS: usize = 2_500;

// ---------------------------------------------------------------- the sorts

/// The constructors of each inductive of the module, in declaration order,
/// read from its text, so that the encoders name a constructor and never
/// number it, and one moved in the module fails here.
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

fn adt(constructor: &str, fields: Vec<Value>) -> Value {
    let found = sorts()
        .get("Doc")
        .expect("the module has the sort `Doc`")
        .iter()
        .position(|name| name == constructor)
        .unwrap_or_else(|| panic!("`Doc` has no constructor `{constructor}`"));
    Value::Adt {
        constructor: u64::try_from(found).expect("an index"),
        fields,
    }
}

// ------------------------------------------------------------- the encoders

/// A document as the module's `Doc`, and the number of nodes it has there: a
/// concatenation is a chain of `DSeq` cells that ends in a `DNil`.
fn encode(doc: &Doc, nodes: &mut usize) -> Value {
    *nodes += 1;
    match doc {
        Doc::Nil => adt("DNil", Vec::new()),
        Doc::Text(value) => adt("DText", vec![text(value)]),
        Doc::Concat(parts) => {
            let mut chain = adt("DNil", Vec::new());
            *nodes += parts.len();
            for part in parts.iter().rev() {
                chain = adt("DSeq", vec![encode(part, nodes), chain]);
            }
            // The `DNil` that ends the chain is the node counted above; an
            // empty concatenation is that node alone.
            chain
        }
        Doc::Nest(by, inner) => adt("DNest", vec![nat_val(*by), encode(inner, nodes)]),
        Doc::Line => adt("DLine", Vec::new()),
        Doc::Soft => adt("DSoft", Vec::new()),
        Doc::Group(inner) => adt("DGroup", vec![encode(inner, nodes)]),
    }
}

/// The run of `renderDoc` on a document at a width.
fn render_run(doc: &Doc, width: usize) -> Run {
    let mut nodes = 0;
    let encoded = encode(doc, &mut nodes);
    (
        vec![encoded, nat_val(width), nat_val(nodes)],
        serde_json::to_value(text(&render(doc, width))).expect("a value is JSON"),
    )
}

/// The nodes of a document in the module's encoding.
fn size(doc: &Doc) -> usize {
    let mut nodes = 0;
    let _ = encode(doc, &mut nodes);
    nodes
}

// ------------------------------------------------------------------ vectors

/// One named vector: a document laid out at a width.
struct Vector {
    label: String,
    doc: Doc,
    width: usize,
}

impl Vector {
    fn run(&self) -> Run {
        render_run(&self.doc, self.width)
    }
}

fn t(value: &str) -> Doc {
    Doc::text(value)
}

fn seq(parts: Vec<Doc>) -> Doc {
    Doc::concat(parts)
}

/// `a`, a line, and `b`, in a group.
fn pair(left: &str, right: &str) -> Doc {
    seq(vec![t(left), Doc::Line, t(right)]).group()
}

/// The list of the oracle's own tests: items between brackets, on one line
/// when they fit and one a line when they do not.
fn bracket(items: &[&str]) -> Doc {
    Doc::concat(vec![
        Doc::text("["),
        Doc::concat(vec![
            Doc::Soft,
            Doc::join(items.iter().map(|item| Doc::text(*item)).collect(), ","),
        ])
        .nest(2),
        Doc::Soft,
        Doc::text("]"),
    ])
    .group()
}

#[allow(clippy::too_many_lines)]
fn named() -> Vec<Vector> {
    let mut out: Vec<Vector> = Vec::new();
    let mut add = |label: &str, doc: Doc, width: usize| {
        out.push(Vector {
            label: label.to_owned(),
            doc,
            width,
        });
    };

    // The oracle's own documents.
    add("a list that fits", bracket(&["a", "bb", "ccc"]), 20);
    add("a list that does not fit", bracket(&["a", "bb", "ccc"]), 8);
    add(
        "nested lists",
        Doc::concat(vec![
            Doc::text("["),
            Doc::concat(vec![
                Doc::Soft,
                Doc::join(vec![bracket(&["x", "y"]), bracket(&["z"])], ","),
            ])
            .nest(2),
            Doc::Soft,
            Doc::text("]"),
        ])
        .group(),
        12,
    );
    add("a list of nothing", bracket(&[]), 0);

    // Nothing, text, and the lines outside any group.
    add("the empty document", Doc::Nil, 0);
    add("an empty text", t(""), 0);
    add("a text beyond the width", t("abcdef"), 3);
    add(
        "a concatenation",
        seq(vec![t("a"), t("b"), t("c"), t("d")]),
        80,
    );
    add("an empty concatenation", seq(Vec::new()), 80);
    add("a concatenation of one", seq(vec![t("a")]), 80);
    add(
        "a concatenation of concatenations",
        seq(vec![
            seq(vec![t("a"), t("b")]),
            seq(Vec::new()),
            seq(vec![t("c")]),
        ]),
        80,
    );
    add(
        "a line outside any group",
        seq(vec![t("a"), Doc::Line, t("b")]),
        80,
    );
    add(
        "a soft break outside any group",
        seq(vec![t("a"), Doc::Soft, t("b")]),
        80,
    );
    add(
        "lines and soft breaks outside any group",
        seq(vec![Doc::Line, Doc::Soft, Doc::Line]),
        80,
    );
    add(
        "joined items outside any group",
        Doc::join(vec![t("a"), t("b"), t("c")], ","),
        80,
    );
    add("joined items of one", Doc::join(vec![t("a")], ","), 80);
    add("joined items of none", Doc::join(Vec::new(), ","), 80);

    // The width test of a group.
    add("a group that fits", pair("a", "b"), 80);
    add("a group that fits exactly", pair("a", "b"), 3);
    add("a group one column short", pair("a", "b"), 2);
    add("a group at width zero", pair("a", "b"), 0);
    add(
        "a group of text that fits exactly",
        seq(vec![t("abc")]).group(),
        3,
    );
    add(
        "a group of text one column short",
        seq(vec![t("abc")]).group(),
        2,
    );
    add(
        "a group of a two-byte scalar at its exact width",
        pair("\u{e9}", "b"),
        3,
    );
    add(
        "a group of a three-byte scalar at its exact width",
        pair("\u{2115}", "b"),
        3,
    );
    add(
        "a group of a four-byte scalar at its exact width",
        pair("\u{1f600}", "b"),
        3,
    );
    add(
        "a group of the delete at its exact width",
        pair("\u{7f}", "b"),
        3,
    );
    // One column short: the delete is the column that decides it, so a
    // layout that counts it as no column lays the group out flat.
    add(
        "a group of the delete one column short",
        pair("\u{7f}", "b"),
        2,
    );
    add(
        "a group of a soft break at its exact width",
        seq(vec![t("["), Doc::Soft, t("]")]).group(),
        2,
    );
    add(
        "a group of a soft break one column short",
        seq(vec![t("["), Doc::Soft, t("]")]).group(),
        1,
    );
    add(
        "a group followed by text that fits",
        seq(vec![pair("a", "b"), t("cccc")]),
        7,
    );
    add(
        "a group followed by text that does not fit",
        seq(vec![pair("a", "b"), t("cccc")]),
        5,
    );
    add(
        "a group followed by a break line",
        seq(vec![pair("a", "b"), Doc::Line, t("ccccccccc")]),
        5,
    );
    add(
        "a group followed by a break soft",
        seq(vec![pair("a", "b"), Doc::Soft, t("ccccccccc")]),
        5,
    );
    add(
        "a group followed by a group",
        seq(vec![pair("a", "b"), pair("c", "d")]),
        6,
    );
    add(
        "a group followed by empty parts",
        seq(vec![pair("a", "b"), Doc::Nil, seq(Vec::new()), t("c")]),
        4,
    );
    add(
        "a group after text",
        seq(vec![t("xxxx"), pair("a", "b")]),
        6,
    );
    add(
        "a group after text of scalars beyond ASCII",
        seq(vec![t("\u{e9}\u{e9}"), pair("a", "b")]),
        5,
    );
    add(
        "a group after text past the width",
        seq(vec![t("abcdefgh"), pair("a", "b")]),
        4,
    );
    // A group of nothing but a soft break takes no column flat, so past the
    // width only the test of the column itself keeps it from fitting.
    add(
        "an empty group past the width",
        seq(vec![t("abcdefgh"), Doc::Soft.group()]),
        4,
    );
    add(
        "a group in a group that fits",
        seq(vec![t("("), pair("a", "b"), t(")")]).group(),
        80,
    );
    add(
        "a group in a group that breaks",
        seq(vec![t("("), Doc::Soft, pair("a", "b"), Doc::Soft, t(")")]).group(),
        4,
    );
    add(
        "a group in a group that breaks, the inner fits",
        seq(vec![t("("), Doc::Soft, pair("a", "b"), Doc::Soft, t(")")]).group(),
        5,
    );
    add(
        "a group of groups",
        seq(vec![
            pair("a", "b"),
            Doc::Line,
            pair("c", "d"),
            Doc::Line,
            pair("e", "f"),
        ])
        .group(),
        7,
    );
    add(
        "a group of a nest",
        seq(vec![
            t("["),
            seq(vec![Doc::Soft, t("x")]).nest(2),
            Doc::Soft,
            t("]"),
        ])
        .group(),
        3,
    );

    // Nests, and the indent of a break.
    add(
        "a nested break",
        Doc::Nest(
            2,
            Box::new(Doc::Nest(3, Box::new(seq(vec![t("a"), Doc::Line, t("b")])))),
        ),
        80,
    );
    add("a nest of nothing", Doc::Nest(4, Box::new(Doc::Nil)), 80);
    add("a nest without a break", Doc::Nest(4, Box::new(t("a"))), 80);
    add(
        "a break after a nest",
        seq(vec![Doc::Nest(4, Box::new(t("a"))), Doc::Line, t("b")]),
        80,
    );
    for indent in [1, 2, 8, 100, 255, 256, 257, 300, 511, 512, 513, 700] {
        add(
            &format!("a nest of {indent}"),
            Doc::Nest(
                indent,
                Box::new(seq(vec![t("a"), Doc::Line, t("b"), Doc::Line, t("c")])),
            ),
            80,
        );
    }
    add(
        "a group after a break in a nest at its width",
        Doc::Nest(2, Box::new(seq(vec![Doc::Line, pair("ab", "cd")]))),
        6,
    );
    add(
        "a group after a break in a nest that fits",
        Doc::Nest(2, Box::new(seq(vec![Doc::Line, pair("ab", "cd")]))),
        7,
    );
    add(
        "a group that breaks in a nest",
        Doc::Nest(
            3,
            Box::new(
                seq(vec![
                    t("["),
                    Doc::Soft,
                    t("abc"),
                    Doc::Line,
                    t("def"),
                    Doc::Soft,
                    t("]"),
                ])
                .group(),
            ),
        ),
        6,
    );
    add(
        "text with a break line in it",
        seq(vec![t("a\nb"), pair("c", "d")]),
        4,
    );
    out
}

// ------------------------------------------------------------------- seeded

/// Text of each width of scalar, and none.
const TEXTS: &[&str] = &[
    "",
    "a",
    "bb",
    "ccc",
    "abcdefgh",
    "\u{e9}",
    "\u{e9}\u{2115}",
    "\u{1f600}",
    "[",
    "]",
    ",",
    ":",
    "a longer piece of text",
    "\u{2115}\u{2115}\u{2115}\u{2115}",
];

/// A document of at most `depth` levels.
fn drawn(seeded: &mut Seeded, depth: usize) -> Doc {
    let kind = if depth == 0 {
        seeded.below(4)
    } else {
        seeded.below(9)
    };
    match kind {
        0 | 4 => Doc::text(*seeded.pick(TEXTS)),
        1 => Doc::Line,
        2 => Doc::Soft,
        3 => Doc::Nil,
        5 | 6 => {
            let parts = (0..seeded.below(5))
                .map(|_| drawn(seeded, depth - 1))
                .collect();
            Doc::concat(parts)
        }
        7 => {
            let by = if seeded.chance(3) {
                *seeded.pick(&[255, 256, 257, 300])
            } else {
                seeded.below(7)
            };
            drawn(seeded, depth - 1).nest(by)
        }
        _ => drawn(seeded, depth - 1).group(),
    }
}

/// `count` seeded documents, each at a width, with the nodes each has.
fn mutants(count: usize) -> Vec<(Doc, usize)> {
    let mut seeded = Seeded(0x0B55_0023);
    (0..count)
        .map(|_| {
            let depth = 1 + seeded.below(6);
            let doc = drawn(&mut seeded, depth);
            let width = *seeded.pick(&[0, 1, 2, 3, 5, 8, 12, 20, 40, 80, 100]);
            (doc, width)
        })
        .collect()
}

/// A hash of a path (FNV-1a).
fn path_hash(path: &str) -> u64 {
    path.bytes().fold(0xcbf2_9ce4_8422_2325_u64, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3)
    })
}

/// Every committed JSON file of at most `JSON_BYTES` bytes that parses, as the
/// document the notation's printer builds from it.
fn documents() -> Vec<(String, Doc)> {
    let root = repo_root();
    let walker = walkdir::WalkDir::new(root.as_std_path())
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|entry| {
            let name = entry.file_name().to_string_lossy();
            !(entry.file_type().is_dir()
                && (matches!(name.as_ref(), "target" | ".git" | ".lexlean")
                    || support::holds_cache_tag(entry)))
        });
    let mut out = Vec::new();
    for entry in walker.flatten() {
        let path = entry.path();
        if !entry.file_type().is_file() || !path.to_string_lossy().ends_with(".json") {
            continue;
        }
        let Ok(bytes) = std::fs::read(path) else {
            continue;
        };
        if bytes.len() > JSON_BYTES {
            continue;
        }
        let Ok(value) = Json::parse(&bytes) else {
            continue;
        };
        let relative = path
            .strip_prefix(root.as_std_path())
            .expect("under the root")
            .to_string_lossy()
            .replace('\\', "/");
        out.push((relative, generic::doc(&value)));
    }
    out
}

// ------------------------------------------------------------------- plants

/// A defect planted in the source, and the named vector that catches it.
struct Plant {
    label: &'static str,
    site: &'static str,
    replacement: &'static str,
    vector: &'static str,
}

const PLANTS: &[Plant] = &[
    Plant {
        label: "a scalar beyond ASCII counts every byte",
        site: "(if (value <? 192) then",
        replacement: "(if false then",
        vector: "a group of a two-byte scalar at its exact width",
    },
    Plant {
        label: "the delete is no scalar",
        site: "(if (value <? 128) then",
        replacement: "(if (value <? 127) then",
        vector: "a group of the delete one column short",
    },
    Plant {
        label: "a flat line is no column",
        site: "(fitWith f (spent + 1) work lower (ctor Option.none [Bool])) else (fitWith f spent work lower (ctor Option.some [Bool] true))",
        replacement: "(fitWith f spent work lower (ctor Option.none [Bool])) else (fitWith f spent work lower (ctor Option.some [Bool] true))",
        vector: "a group one column short",
    },
    Plant {
        label: "a break line does not end the scan",
        site: "(fitWith f (spent + 1) work lower (ctor Option.none [Bool])) else (fitWith f spent work lower (ctor Option.some [Bool] true))",
        replacement: "(fitWith f (spent + 1) work lower (ctor Option.none [Bool])) else (fitWith f (spent + 1) work lower (ctor Option.none [Bool]))",
        vector: "a group followed by a break line",
    },
    Plant {
        label: "a flat soft break is a column",
        site: "| Doc.DSoft => (if flat then (fitWith f spent work lower (ctor Option.none [Bool]))",
        replacement: "| Doc.DSoft => (if flat then (fitWith f (spent + 1) work lower (ctor Option.none [Bool]))",
        vector: "a group of a soft break at its exact width",
    },
    Plant {
        label: "a break soft break does not end the scan",
        site: "| Doc.DSoft => (if flat then (fitWith f spent work lower (ctor Option.none [Bool])) else (fitWith f spent work lower (ctor Option.some [Bool] true)))",
        replacement: "| Doc.DSoft => (if flat then (fitWith f spent work lower (ctor Option.none [Bool])) else (fitWith f spent work lower (ctor Option.none [Bool])))",
        vector: "a group followed by a break soft",
    },
    Plant {
        label: "a group fits only under its room",
        site: "((project f limit) <? (project f spent))",
        replacement: "((project f limit) <=? (project f spent))",
        vector: "a group that fits exactly",
    },
    Plant {
        label: "a group is tried at no column past the width",
        site: "(width <? column)",
        replacement: "false",
        vector: "an empty group past the width",
    },
    Plant {
        label: "the rest of the stack is not read",
        site: "| List.cons item rest => (ctor Option.some [Fit] (fitItem f item (nil Item) rest))",
        replacement: "| List.cons item rest => (ctor Option.some [Fit] (fitWith f (project f spent) (nil Item) (nil Item) (ctor Option.some [Bool] true)))",
        vector: "a group followed by text that does not fit",
    },
    Plant {
        label: "a group is tried at the start of the line",
        site: "(fits nodes width (project l column) rest",
        replacement: "(fits nodes width 0 rest",
        vector: "a group after text",
    },
    Plant {
        label: "text advances the column by its bytes",
        site: "column := ((project l column) + (scalars piece))",
        replacement: "column := ((project l column) + (prim length piece : Nat))",
        vector: "a group after text of scalars beyond ASCII",
    },
    Plant {
        label: "a group that fits is laid out broken",
        site: "(record Item {indent := indent, flat := one, node := inner})",
        replacement: "(record Item {indent := indent, flat := false, node := inner})",
        vector: "a group that fits",
    },
    Plant {
        label: "a document starts flat",
        site: "{indent := 0, flat := false, node := doc}",
        replacement: "{indent := 0, flat := true, node := doc}",
        vector: "a line outside any group",
    },
    Plant {
        label: "a flat line is two spaces",
        site: "(prim utf8_encode \" \" : Bytes)",
        replacement: "(prim utf8_encode \"  \" : Bytes)",
        vector: "a group that fits",
    },
    Plant {
        label: "a break leaves the column at zero",
        site: "column := indent, stack := rest",
        replacement: "column := 0, stack := rest",
        vector: "a group after a break in a nest at its width",
    },
    Plant {
        label: "a nest sets the indent and does not add to it",
        site: "stack := (cons (record Item {indent := (indent + amount), flat := flat, node := inner}) rest)})",
        replacement: "stack := (cons (record Item {indent := amount, flat := flat, node := inner}) rest)})",
        vector: "a nested break",
    },
    Plant {
        label: "an indent counts its spaces by 512",
        site: "(prim quotient indent 256 0 : Nat)",
        replacement: "(prim quotient indent 512 0 : Nat)",
        vector: "a nest of 300",
    },
    Plant {
        label: "an indent's remainder is taken by 255",
        site: "(prim remainder indent 256 0 : Nat)",
        replacement: "(prim remainder indent 255 0 : Nat)",
        vector: "a nest of 256",
    },
    Plant {
        label: "a concatenation's parts come last first",
        site: "stack := (cons (record Item {indent := indent, flat := flat, node := head}) (cons (record Item {indent := indent, flat := flat, node := tail}) rest))",
        replacement: "stack := (cons (record Item {indent := indent, flat := flat, node := tail}) (cons (record Item {indent := indent, flat := flat, node := head}) rest))",
        vector: "a concatenation",
    },
    Plant {
        label: "the layout stops one node early",
        // A concatenation counts its parts twice in the budget, so it has
        // steps to spare; one text is one node and one step.
        site: "(nodes + 1) (record Layout",
        replacement: "(prim subtract nodes 1 : Nat) (record Layout",
        vector: "a text beyond the width",
    },
    Plant {
        label: "the width test of a group is starved of nodes",
        site: "(let one : Bool := (fits nodes",
        replacement: "(let one : Bool := (fits (prim subtract nodes nodes : Nat)",
        vector: "a group that fits",
    },
];

fn plants(vectors: &[Vector]) {
    let mut missed = Vec::new();
    for plant in PLANTS {
        let vector = vectors
            .iter()
            .find(|vector| vector.label == plant.vector)
            .unwrap_or_else(|| panic!("{}: no vector `{}`", plant.label, plant.vector));
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/SurfaceDoc.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/SurfaceDoc.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[0].0);
        match super::evaluate(&low, &[vector.run()], 1).first() {
            Some(first) => eprintln!(
                "BS-23: plant detected: {}: {}",
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

fn export(lowered: &super::Lowered) {
    let exported = harness::exported("bootstrap");
    let bundle = exported.bundle();
    let mut realized = 0usize;
    for exported_root in &exported.output.result.roots {
        if exported_root.root != format!("{MODULE}.{}", ROOTS[0].0) {
            continue;
        }
        let largest = exported_root
            .outputs
            .iter()
            .flat_map(|output| &output.certification.certificates)
            .map(|certificate| certificate.file.byte_length)
            .max()
            .expect("a certificate");
        eprintln!(
            "BS-23: {}: largest certificate part {largest} bytes",
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
                serde_json::to_value(&lowered.program).expect("a program is JSON"),
                "{}: the exported package realizes the program the differential ran",
                exported_root.root
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
        checked_units.units.contains_key("SurfaceDoc"),
        "SurfaceDoc is a linked module of the project"
    );
    eligibility_of(&project, "SurfaceDoc", MODULE, &ROOTS);
    assert_eq!(
        sorts().keys().map(String::as_str).collect::<Vec<_>>(),
        ["Doc"],
        "the sorts of the module"
    );

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label.as_str()).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let vector_runs: Vec<Run> = vectors.iter().map(Vector::run).collect();
    let broken = vectors
        .iter()
        .filter(|vector| render(&vector.doc, vector.width).contains('\n'))
        .count();
    assert!(
        0 < broken && broken < vectors.len(),
        "the vectors give layouts of both kinds, with a break and without"
    );

    // The documents of the committed JSON files, at each width.
    let files = documents();
    let mut evaluator_files: Vec<Run> = Vec::new();
    let mut rustc_files: Vec<Run> = Vec::new();
    for (path, doc) in &files {
        let nodes = size(doc);
        let hash = path_hash(path);
        for width in WIDTHS {
            if nodes <= EVALUATOR_NODES && hash.is_multiple_of(EVALUATOR_EVERY) {
                evaluator_files.push(render_run(doc, width));
            }
            if nodes <= RUSTC_NODES && hash.is_multiple_of(RUSTC_EVERY) {
                rustc_files.push(render_run(doc, width));
            }
        }
    }
    let normative = support::normative_host();
    let drawn_docs = if normative {
        mutants(EVALUATOR_MUTANTS + RUSTC_MUTANTS)
    } else {
        Vec::new()
    };
    let (evaluator_drawn, rustc_drawn): (Vec<Run>, Vec<Run>) = {
        let split = drawn_docs.len().min(EVALUATOR_MUTANTS);
        (
            drawn_docs[..split]
                .iter()
                .map(|(doc, width)| render_run(doc, *width))
                .collect(),
            drawn_docs[split..]
                .iter()
                .map(|(doc, width)| render_run(doc, *width))
                .collect(),
        )
    };
    eprintln!(
        "BS-23: {} vectors, {} JSON documents ({} and {} runs), {} and {} seeded runs ({:?})",
        vectors.len(),
        files.len(),
        evaluator_files.len(),
        rustc_files.len(),
        evaluator_drawn.len(),
        rustc_drawn.len(),
        started.elapsed()
    );

    let checked = support::checked_project(&project);
    let lowered = super::lower(&checked, &project, MODULE, ROOTS[0].0);
    if normative {
        let mut corpus: Vec<Run> = vector_runs.clone();
        corpus.extend(evaluator_files);
        corpus.extend(evaluator_drawn);
        let disagreements = super::evaluate(&lowered, &corpus, 5);
        assert!(
            disagreements.is_empty(),
            "renderDoc and the Rust oracle disagree: {disagreements:?}"
        );
        eprintln!(
            "BS-23: the evaluator agrees on {} runs ({:?})",
            corpus.len(),
            started.elapsed()
        );
        plants(&vectors);
    } else {
        eprintln!("BS-23: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled = vector_runs;
    compiled.extend(rustc_files);
    compiled.extend(rustc_drawn);
    let total = rustc_differential(
        "BS-23",
        &[(ROOTS[0].0, &lowered, compiled.as_slice())],
        "oracle",
    );
    eprintln!("BS-23: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-23") {
        return;
    }
    export(&lowered);
    eprintln!("BS-23: passed in {:?}", started.elapsed());
}
