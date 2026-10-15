//! The `authoring` suite, core modules: AU-09..AU-12 (SPEC.md §17.21.8).
//!
//! A core module is a library of kernel terms, and the notation writes it as a
//! program over them. The cases state what that costs nothing of: every
//! committed core module is read back equal from its notation and from its data
//! (AU-09), a module in the notation is the module its data form is when it is
//! linked, built, and formatted (AU-10), and malformed text is refused at a
//! position and never panics (AU-11). The committed modules are the input, not
//! a sample of them.

use std::collections::BTreeSet;

use lexlean::ir::core::{CoreLevel, CoreMetaValue, CoreModule, CoreNode, CORE_SPEC_2};
use lexlean::sdk::{assemble, ModuleKind, SourceHeader};
use lexlean::surface::core as notation;
use lexlean::surface::lex::lex;
use lexlean::surface::module::{locate, CORE};
use lexlean::{FormatRequest, Selection};
use serde_json::{json, Value};

use crate::support::{self, repo_root, P};

/// Run the case for one core-module authoring ID.
///
/// # Panics
///
/// When the case's assertion fails, or for an unwired ID.
pub fn run(id: &str) {
    match id {
        "AU-09" => round_trip(),
        "AU-10" => equivalence(),
        "AU-11" => fails_closed(),
        "AU-12" => authored(),
        other => panic!("no core authoring case is wired for {other}"),
    }
}

// ---------------------------------------------------------------------------
// The committed corpus.
// ---------------------------------------------------------------------------

/// Every committed source with a core module: its `/`-separated path and its
/// text.
fn committed() -> Vec<(String, String)> {
    let root = repo_root();
    let mut found = Vec::new();
    for entry in walkdir::WalkDir::new(root.as_std_path())
        .into_iter()
        .filter_entry(|entry| {
            let name = entry.file_name().to_string_lossy();
            name != "target" && name != ".git" && name != ".lexlean"
        })
        .flatten()
    {
        let path = entry.path();
        if !path.to_string_lossy().ends_with(".lex.tex") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(path) else {
            continue;
        };
        if locate(&text).is_some_and(|located| located.kind.is(&CORE)) {
            let shown = path
                .strip_prefix(root.as_std_path())
                .unwrap_or(path)
                .to_string_lossy()
                .replace('\\', "/");
            found.push((shown, text));
        }
    }
    found.sort();
    found
}

// ---------------------------------------------------------------------------
// AU-09.
// ---------------------------------------------------------------------------

/// The number of nodes below which a node is written inside its user
/// (SPEC.md §17.21.8).
const INLINE_HEIGHT: usize = 48;

/// The nodes of `module` that have a statement, by the rule of §17.21.8, as
/// the case derives them from the module and not from the printer: a node is
/// written inside the node that uses it exactly when it is used once, by no
/// declaration, as a child of the node that comes right after it and its other
/// written-inside children, and fewer than 48 nodes deep.
fn statements_by_rule(module: &CoreModule) -> BTreeSet<usize> {
    let count = module.nodes.len();
    let children = |node: &CoreNode| -> Vec<usize> {
        match node {
            CoreNode::App { f, x } => vec![*f, *x],
            CoreNode::Lambda { t, v, .. } | CoreNode::Forall { t, v, .. } => vec![*t, *v],
            CoreNode::Let { t, v, body, .. } => vec![*t, *v, *body],
            CoreNode::Projection { x, .. } | CoreNode::Mdata { x, .. } => vec![*x],
            CoreNode::BVar { .. }
            | CoreNode::Sort { .. }
            | CoreNode::Const { .. }
            | CoreNode::Nat { .. }
            | CoreNode::String { .. } => Vec::new(),
        }
    };
    let mut uses = vec![0usize; count];
    for node in &module.nodes {
        for child in children(node) {
            uses[child] += 1;
        }
    }
    for declaration in &module.declarations {
        let mut roots = vec![declaration.r#type];
        roots.extend(declaration.value);
        if let Some(equations) = &declaration.eqn_info {
            roots.extend([equations.r#type, equations.value]);
        }
        if let Some(structure) = declaration
            .inductive
            .as_ref()
            .and_then(|inductive| inductive.structure.as_ref())
        {
            roots.extend(structure.fields.iter().filter_map(|field| field.auto_param));
        }
        for root in roots {
            uses[root] += 1;
        }
    }
    // For each node, where the nodes written inside it begin.
    let mut first = vec![0usize; count];
    let mut depth = vec![0usize; count];
    let mut inside = vec![false; count];
    for (index, node) in module.nodes.iter().enumerate() {
        let mut next = index; // the node right after what is written inside so far
        let mut deepest = 0;
        for child in children(node).into_iter().rev() {
            if uses[child] == 1 && child + 1 == next && depth[child] < INLINE_HEIGHT {
                inside[child] = true;
                next = first[child];
                deepest = deepest.max(depth[child]);
            }
        }
        first[index] = next;
        depth[index] = deepest + 1;
    }
    (0..count).filter(|index| !inside[*index]).collect()
}

/// The node numbers the statements of a notation carry, from its labels.
fn labels(text: &str) -> BTreeSet<usize> {
    text.lines()
        .filter_map(|line| line.strip_prefix('@'))
        .filter_map(|rest| rest.split(' ').next()?.parse().ok())
        .collect()
}

/// The name of a node's kind, in an exhaustive match: a kind added to the
/// schema fails to compile here until the case below carries it.
fn node_kind(node: &CoreNode) -> &'static str {
    match node {
        CoreNode::BVar { .. } => "b",
        CoreNode::Sort { .. } => "s",
        CoreNode::Const { .. } => "c",
        CoreNode::App { .. } => "a",
        CoreNode::Lambda { .. } => "l",
        CoreNode::Forall { .. } => "p",
        CoreNode::Let { .. } => "e",
        CoreNode::Nat { .. } => "n",
        CoreNode::String { .. } => "t",
        CoreNode::Mdata { .. } => "d",
        CoreNode::Projection { .. } => "j",
    }
}

/// The kinds of universe level a module's nodes and constants carry.
fn level_kinds(module: &CoreModule) -> BTreeSet<&'static str> {
    fn walk(level: &CoreLevel, out: &mut BTreeSet<&'static str>) {
        match level {
            CoreLevel::Zero => {
                out.insert("z");
            }
            CoreLevel::Succ(inner) => {
                out.insert("s");
                walk(inner, out);
            }
            CoreLevel::Max(pair) => {
                out.insert("m");
                walk(&pair.0, out);
                walk(&pair.1, out);
            }
            CoreLevel::IMax(pair) => {
                out.insert("i");
                walk(&pair.0, out);
                walk(&pair.1, out);
            }
            CoreLevel::Param(_) => {
                out.insert("p");
            }
        }
    }
    let mut out = BTreeSet::new();
    for node in &module.nodes {
        match node {
            CoreNode::Sort { l } => walk(l, &mut out),
            CoreNode::Const { u, .. } => u.iter().for_each(|level| walk(level, &mut out)),
            _ => {}
        }
    }
    out
}

/// A module that carries every kind of node, every kind of universe level,
/// every binder visibility, every kind of annotation value (and the spelling of
/// a number that is not canonical), names that need quotes, the empty binder
/// name, nodes that several nodes share, nodes a declaration states, nodes
/// the notation writes inside their user, nodes marked as proofs, and every
/// member of a declaration.
fn every_kind() -> CoreModule {
    let levels = json!({"a": [{"a": "u", "k": "p"}, {"a": {"a": {"k": "z"}, "k": "s"}, "k": "s"}], "k": "i"});
    let first = json!([
        {"k": "s", "l": {"k": "z"}},
        {"k": "s", "l": {"a": {"a": "u", "k": "p"}, "k": "s"}},
        {"k": "s", "l": {"a": [{"k": "z"}, {"a": "v", "k": "p"}], "k": "m"}},
        {"k": "s", "l": levels},
        {"k": "s", "l": {"a": {"a": [{"a": "u", "k": "p"}, {"a": "v", "k": "p"}], "k": "m"}, "k": "s"}},
        {"k": "c", "n": "Nat", "u": []},
        {"k": "c", "n": "List.map", "u": [{"a": "u", "k": "p"}, {"a": "v", "k": "p"}]},
        {"k": "c", "n": "fun", "u": []},
        {"k": "c", "n": "a b", "u": [{"k": "z"}]},
        {"i": 0, "k": "b"},
        {"i": 3, "k": "b"},
        {"f": 6, "k": "a", "x": 5},
        {"f": 11, "k": "a", "x": 9},
        {"b": "e", "k": "l", "n": "x", "t": 5, "v": 12},
        {"i": 0, "k": "b"},
        {"b": "i", "k": "l", "n": "", "t": 5, "v": 14},
        {"b": "s", "k": "p", "n": "y", "t": 5, "v": 5},
        {"b": "c", "k": "p", "n": "inst", "t": 5, "v": 5},
        {"b": "e", "k": "p", "n": "", "t": 5, "v": 5},
        {"b": "e", "k": "p", "n": "_", "t": 5, "v": 5},
        {"body": 9, "d": false, "k": "e", "n": "z", "t": 5, "v": 5},
        {"body": 9, "d": true, "k": "e", "n": "", "t": 5, "v": 5},
        {"k": "n", "v": "42"},
        {"k": "n", "v": "007"},
        {"k": "t", "v": [104, 105, 92, 34, 10, 955, 9, 1]}
    ]);
    let more = json!([
        {"k": "d", "m": [
            {"n": "k1", "v": {"t": "b", "v": true}},
            {"n": "k2", "v": {"t": "n", "v": "5"}},
            {"n": "k3", "v": {"t": "i", "v": "-5"}},
            {"n": "k4", "v": {"t": "s", "v": "text \\ \""}},
            {"n": "k5", "v": {"t": "m", "v": "Foo.bar"}},
            {"n": "k6", "v": {"t": "n", "v": "05"}},
            {"n": "k7", "v": {"t": "i", "v": "+5"}},
            {"n": "an entry", "v": {"t": "b", "v": false}}
        ], "x": 9},
        {"i": 1, "k": "j", "n": "Prod.fst", "s": "Prod", "x": 9},
        {"f": 13, "k": "a", "x": 24},
        {"b": "e", "k": "l", "n": "w", "t": 26, "v": 14},
        {"k": "c", "n": "A._@.Mod._hyg.#16", "u": []},
        {"k": "c", "n": "x1", "u": []},
        {"k": "c", "n": "Foo.007", "u": []},
        {"f": 29, "k": "a", "x": 30},
        {"f": 32, "k": "a", "x": 31},
        {"k": "c", "n": "α₁", "u": []},
        {"f": 33, "k": "a", "x": 22},
        {"i": 0, "k": "b"},
        {"b": "e", "k": "l", "n": "x", "t": 5, "v": 36},
        {"i": 1, "k": "b"},
        {"b": "e", "k": "l", "n": "x", "t": 5, "v": 38},
        {"b": "e", "k": "l", "n": "x", "t": 5, "v": 39},
        {"k": "c", "n": "y", "u": []},
        {"b": "e", "k": "l", "n": "y", "t": 5, "v": 41}
    ]);
    // `json!` expands its input by recursion, so the table is written in two.
    let nodes: Vec<Value> = first
        .as_array()
        .into_iter()
        .chain(more.as_array())
        .flatten()
        .cloned()
        .collect();
    let definition = json!({
        "all": ["Lib.f"], "class": false, "compiled": true, "generated": false, "inline": "always_inline",
        "kind": "definition", "levels": ["u", "v"], "name": "Lib.f",
        "policy": {"axioms": ["propext"], "kind": "exact"},
        "reducibility": {"height": 3, "kind": "regular"}, "safety": "partial",
        "tags": ["protected", "defeq"], "transparency": "semireducible", "type": 15, "value": 13,
        "eqn_info": {"declarations": ["Lib.f"], "levels": ["u"], "num_fixed": 1, "perms": [[[0]]],
                     "rec_arg_pos": 0, "rev_deps": [[[0]]], "type": 31, "value": 34},
        "matcher": {"alts": [{"fields": 1, "overlaps": 0, "unit_thunk": false}],
                    "discrs": [["x"]], "num_discrs": 1, "num_params": 0, "overlaps": [[0, [1]]],
                    "u_elim_pos": 0},
        "projection": {"constructor": "S.mk", "from_class": false, "index": 0, "num_params": 1},
        "simp": {"attribute": "global", "post": true, "priority": 1000},
        "no_confusion": {"arity": 2, "kind": "regular", "lhs": 3, "rhs": 4},
        "sparse": {"arity": 3, "ind_name": "I", "interesting_ctors": ["I.a"], "major_pos": 1},
        "instance": {"attribute": "global", "priority": 1000, "synthesis_order": [0]}
    });
    let inductive = json!({
        "class": true, "generated": false, "kind": "inductive", "levels": [], "name": "Lib.S",
        "policy": {"kind": "none"}, "transparency": "semireducible", "type": 17,
        "inductive": {"constructors": ["Lib.S.mk"], "num_indices": 0, "num_params": 1,
                      "structure": {"fields": [{"auto_param": 21, "binder": "e", "name": "f",
                                                "projection": "Lib.S.f", "subobject": "Lib.P"}],
                                    "parents": [{"name": "Lib.P", "projection": "Lib.S.toP", "subobject": true}]}}
    });
    let theorem = json!({
        "class": false, "generated": true, "grind": "default", "kind": "theorem", "levels": [],
        "name": "Lib.t", "policy": {"axioms": ["Classical.choice", "propext"], "kind": "allow"},
        "transparency": "irreducible", "type": 35, "value": 33
    });
    let named = json!({
        "class": false, "generated": false, "kind": "theorem", "levels": [],
        "name": "Lib.named", "policy": {"kind": "none"}, "transparency": "semireducible",
        "type": 37, "value": 40
    });
    let clash = json!({
        "class": false, "generated": false, "kind": "theorem", "levels": [],
        "name": "Lib.clash", "policy": {"kind": "none"}, "transparency": "semireducible",
        "type": 42, "value": 37
    });
    let module = json!({
        "declarations": [definition, inductive, theorem, named, clash],
        "imports": ["Init", "Lib.Base"],
        "nodes": nodes,
        "proof_nodes": [12, 22, 25],
        "spec": CORE_SPEC_2
    });
    serde_json::from_value(module).expect("a core module")
}

/// What the notation of one module satisfies: it reads back to the module, it
/// is its own canonical form, and its statements are the nodes the rule of
/// §17.21.8 gives one.
fn assert_notation(shown: &str, module: &CoreModule) -> String {
    let text = notation::print_module(module).unwrap_or_else(|error| panic!("{shown}: {error}"));
    let back = notation::parse_module(&text).unwrap_or_else(|error| {
        let (line, column) = error.line_column(&text);
        panic!("{shown}: the notation does not read: {error} ({line}:{column})")
    });
    assert!(
        back == *module,
        "{shown}: the notation reads to another module"
    );
    assert_eq!(
        notation::print_module(&back).expect("prints"),
        text,
        "{shown}: the notation is not its own canonical form"
    );
    assert_eq!(
        labels(&text),
        statements_by_rule(module),
        "{shown}: the statements are not the nodes the rule gives one"
    );
    text
}

fn round_trip() {
    // A module that carries every kind: each kind of node, level, annotation
    // value, binder, and member of a declaration is in it, and the case is
    // sure of that before it reads anything back.
    let sink = every_kind();
    assert_eq!(
        sink.nodes.iter().map(node_kind).collect::<BTreeSet<_>>(),
        BTreeSet::from(["a", "b", "c", "d", "e", "j", "l", "n", "p", "s", "t"]),
        "a kind of node is missing from the module that carries every kind"
    );
    assert_eq!(
        level_kinds(&sink),
        BTreeSet::from(["i", "m", "p", "s", "z"]),
        "a kind of level is missing"
    );
    let annotated: BTreeSet<&str> = sink
        .nodes
        .iter()
        .filter_map(|node| match node {
            CoreNode::Mdata { m, .. } => Some(m),
            _ => None,
        })
        .flatten()
        .map(|entry| match entry.v {
            CoreMetaValue::Bool { .. } => "b",
            CoreMetaValue::Nat { .. } => "n",
            CoreMetaValue::Int { .. } => "i",
            CoreMetaValue::Str { .. } => "s",
            CoreMetaValue::Name { .. } => "m",
        })
        .collect();
    assert_eq!(
        annotated,
        BTreeSet::from(["b", "i", "m", "n", "s"]),
        "a kind of annotation value is missing"
    );
    let text = assert_notation("the module that carries every kind", &sink);
    for (what, written) in [
        ("a quoted constant", "const \"a b\".{0}"),
        ("a keyword as a constant", "const \"fun\""),
        ("a non-canonical natural", "nat \"007\""),
        ("a strict-implicit binder", "{{y : "),
        ("an instance binder", "[inst : "),
        ("the empty binder name", "fun {_ : "),
        ("a binder named `_`", "(\"_\" : "),
        ("a proof marker", "proof("),
        ("an annotation", "annot ["),
        ("a projection", "proj Prod Prod.fst 1"),
        ("a have", "have _ : "),
        ("a level raised", "+1"),
        ("imax", "imax("),
        ("max", "max("),
        (
            "a variable written by its binder's name",
            "fun (x : @5) => x",
        ),
        (
            "a variable a nearer binder shadows",
            "fun (x : @5) => fun (x : @5) => #1",
        ),
        (
            "a constant a binder is named like",
            "fun (y : @5) => const \"y\"",
        ),
    ] {
        assert!(
            text.contains(written),
            "{what} is not written as `{written}`:\n{text}"
        );
    }
    // Its data form is the module, and the compiler's own decoder reads the
    // canonical JSON of what the notation reads to as that module.
    let data = notation::canonical_json(&sink).expect("the data form");
    let decoded: CoreModule = serde_json::from_str(&data).expect("decodes");
    assert!(decoded == sink, "the data form is not the module");
    assert_eq!(
        CORE.notation_of_data(&data)
            .expect("a module of the schema")
            .expect("prints"),
        text,
        "the notation of the data form is the notation of the module"
    );

    let (mut written, mut as_data, mut largest) = (0usize, 0usize, 0usize);
    for (shown, source) in committed() {
        let located = locate(&source).expect("a module");
        match located.data {
            // A module written in the notation: it is language 1.2's, it
            // decodes and validates as the compiler does, and its data form
            // is what the compiler accepts as canonical JSON.
            None => {
                let body = &source[located.body.0..located.body.1];
                let module =
                    notation::parse_module(body).unwrap_or_else(|error| panic!("{shown}: {error}"));
                let text = assert_notation(&shown, &module);
                assert_eq!(
                    body.strip_prefix('\n'),
                    Some(text.as_str()),
                    "{shown}: canonical"
                );
                let data = notation::data_text(body, CORE_SPEC_2).expect("the data form");
                let decoded = CoreModule::parse(&data, "1.2")
                    .unwrap_or_else(|reason| panic!("{shown}: the compiler refuses it: {reason}"));
                assert!(
                    decoded == module,
                    "{shown}: the data form is another module"
                );
                assert_eq!(
                    CORE.notation_of_data(&data)
                        .expect("a module of the schema")
                        .expect("prints"),
                    text,
                    "{shown}"
                );
                written += 1;
                largest = largest.max(module.nodes.len());
            }
            // A module written as data (language 1.0 and 1.1, or schema 1):
            // the data form is written back byte for byte, and read as a
            // module of the schema the notation names, its nodes and
            // declarations are written and read back as any other's.
            Some((from, to)) => {
                let data = &source[from..to];
                let Ok(module) = serde_json::from_str::<CoreModule>(data) else {
                    // A refused fixture whose payload is not a module.
                    assert!(
                        shown.starts_with("tests/negative/"),
                        "{shown}: not a module"
                    );
                    continue;
                };
                assert_eq!(
                    notation::canonical_json(&module).expect("writes"),
                    data,
                    "{shown}: the data form is not written back byte for byte"
                );
                let mut second = module.clone();
                second.spec = CORE_SPEC_2.to_owned();
                assert_notation(&shown, &second);
                as_data += 1;
            }
        }
    }
    // Anti-vacuity: the walk read the product, the data form it writes back
    // is really there, and a module is large enough for sharing to matter.
    assert!(
        written >= 26 && as_data >= 60 && largest >= 100_000,
        "{written} core modules in the notation, {as_data} as data, the largest of {largest} nodes: the walk did not read the product"
    );
}

// ---------------------------------------------------------------------------
// The module the linking cases check.
// ---------------------------------------------------------------------------

/// A module of a few declarations that links as any core module does: it
/// states shared nodes, nodes a declaration states, and nodes the notation
/// writes inside their user.
fn small() -> Value {
    let definition = |name: &str, kind: &str, ty: usize, value: usize| {
        json!({
            "class": false, "generated": false, "kind": kind, "levels": [], "name": name,
            "policy": {"kind": "none"}, "reducibility": {"height": 1, "kind": "regular"},
            "transparency": "semireducible", "type": ty, "value": value
        })
    };
    json!({
        "declarations": [
            definition("Lib.idNat", "definition", 4, 3),
            definition("Lib.seven", "definition", 1, 6),
            definition("Lib.apply", "definition", 4, 9),
            definition("Lib.letDemo", "definition", 1, 10)
        ],
        "imports": ["Init"],
        "nodes": [
            {"k": "s", "l": {"a": {"k": "z"}, "k": "s"}},
            {"k": "c", "n": "Nat", "u": []},
            {"i": 0, "k": "b"},
            {"b": "e", "k": "l", "n": "n", "t": 1, "v": 2},
            {"b": "e", "k": "p", "n": "n", "t": 1, "v": 1},
            {"k": "n", "v": "7"},
            {"f": 3, "k": "a", "x": 5},
            {"b": "e", "k": "p", "n": "", "t": 1, "v": 1},
            {"i": 0, "k": "b"},
            {"b": "e", "k": "l", "n": "f", "t": 7, "v": 8},
            {"body": 2, "d": false, "k": "e", "n": "x", "t": 1, "v": 5}
        ],
        "proof_nodes": [],
        "spec": CORE_SPEC_2
    })
}

const MAIN: &str = "src/Main.lex.tex";

/// A copy of the language-1.2 example whose one module is `data`, written in
/// the notation or as `\coredata{…}`.
fn project(data: &Value, in_notation: bool) -> P {
    let project = P::language_1_2_example();
    let header = SourceHeader {
        name: "Main",
        glossaries: &["lexlean.std.nat@1.2.0"],
        imports: &[],
        title: "Natural number addition",
    };
    let source = assemble(&header, ModuleKind::Core, data).expect("a core source");
    project.write(
        MAIN,
        &if in_notation {
            source
        } else {
            lexlean::sdk::to_data_form(&source).expect("the data form")
        },
    );
    project
}

fn files(directory: &camino::Utf8Path) -> std::collections::BTreeMap<String, Vec<u8>> {
    let mut out = std::collections::BTreeMap::new();
    for entry in walkdir::WalkDir::new(directory.as_std_path())
        .into_iter()
        .flatten()
    {
        if entry.file_type().is_file() {
            let relative = entry
                .path()
                .strip_prefix(directory.as_std_path())
                .expect("under the directory")
                .to_string_lossy()
                .replace('\\', "/");
            out.insert(relative, std::fs::read(entry.path()).expect("read"));
        }
    }
    out
}

fn format_all(
    project: &P,
    check_only: bool,
) -> Result<lexlean::FormatResultSet, lexlean::LexLeanError> {
    project.engine().format(FormatRequest {
        selection: Selection::All,
        check_only,
    })
}

// ---------------------------------------------------------------------------
// AU-10.
// ---------------------------------------------------------------------------

/// Link, build, and format a core module in both forms, and compare them.
fn same_module(what: &str, data: &P, notation: &P) {
    // The notation is canonical as it is; the data form is the same module in
    // text `fmt` does not write, and `fmt` writes it as exactly the notation.
    notation.fmt_check_ok();
    support::expect_code(
        &format_all(data, true)
            .err()
            .expect("a module written as data is not canonical"),
        "LLL1003",
    );
    let migrated = P::language_1_2_example();
    migrated.write(MAIN, &data.read_raw(MAIN));
    format_all(&migrated, false).expect("fmt migrates the data form");
    assert_eq!(
        migrated.read_raw(MAIN),
        notation.read_raw(MAIN),
        "{what}: fmt did not write the data form as the notation"
    );
    // Another layout of the notation is not canonical, and `fmt` restores
    // the layout.
    let canonical = notation.read_raw(MAIN);
    let loose = canonical
        .replace(" := ", "  :=  ")
        .replace("\n\n", "\n\n\n");
    assert_ne!(loose, canonical, "{what}: the layout did not change");
    notation.write(MAIN, &loose);
    support::expect_code(
        &format_all(notation, true)
            .err()
            .expect("another layout is not canonical"),
        "LLL1003",
    );
    format_all(notation, false).expect("fmt writes the notation");
    assert_eq!(
        notation.read_raw(MAIN),
        canonical,
        "{what}: fmt did not restore"
    );
    format_all(notation, false).expect("fmt writes the notation");
    assert_eq!(
        notation.read_raw(MAIN),
        canonical,
        "{what}: fmt changed the notation"
    );

    // The same module: the same semantic ID, and the same generated Lean and
    // LaTeX byte for byte.
    let data_checked = data.check_ok();
    let notation_checked = notation.check_ok();
    assert_eq!(
        data_checked.semantic_id, notation_checked.semantic_id,
        "{what}: the notation does not link to its data form's module"
    );
    assert_ne!(
        data_checked.source_id, notation_checked.source_id,
        "{what}: the source did not change"
    );
    let data_built = data.build_ok();
    let notation_built = notation.build_ok();
    let data_dir = data.build_dir(&data_built.build_id.expect("a build id"));
    let notation_dir = notation.build_dir(&notation_built.build_id.expect("a build id"));
    let data_files = files(&data_dir.join("modules"));
    assert!(!data_files.is_empty(), "{what}: no generated files");
    assert_eq!(
        data_files,
        files(&notation_dir.join("modules")),
        "{what}: the generated Lean and LaTeX differ"
    );

    // The source map of the notation names the notation: the pieces of the
    // module map the text between the module's markers, and the data form's
    // map names `coredata`.
    let owned = |directory: &camino::Utf8Path, owner: &str| -> usize {
        files(&directory.join("coverage"))
            .values()
            .map(|bytes| {
                String::from_utf8_lossy(bytes)
                    .matches(&format!("\"owner\":\"lexlean.core::{owner}\""))
                    .count()
            })
            .sum()
    };
    assert!(
        owned(&notation_dir, "coresource") > 0 && owned(&notation_dir, "coredata") == 0,
        "{what}: the coverage of the notation does not name the notation"
    );
    assert!(
        owned(&data_dir, "coredata") > 0 && owned(&data_dir, "coresource") == 0,
        "{what}: the coverage of the data form does not name the data"
    );
}

fn equivalence() {
    // A module of a few declarations, linked in both forms.
    let data = small();
    same_module(
        "the small module",
        &project(&data, false),
        &project(&data, true),
    );

    // A committed module of the preservation library, which is large, in both
    // forms: `Core` states the constructors of the observations of the machine.
    let library_with = |rewrite: &dyn Fn(&str) -> String| {
        let library = P::library_project();
        for entry in walkdir::WalkDir::new(library.root.join("src").as_std_path())
            .into_iter()
            .flatten()
        {
            let path = entry.path();
            if !path.to_string_lossy().ends_with(".lex.tex") {
                continue;
            }
            // By file name, not by a suffix with a separator: a path on
            // Windows is written with backslashes.
            if path.file_name().is_some_and(|name| name == "Core.lex.tex") {
                let text = std::fs::read_to_string(path).expect("read");
                std::fs::write(path, rewrite(&text)).expect("writes");
            } else {
                std::fs::remove_file(path).expect("removes");
            }
        }
        library.edit(
            "lexlean.toml",
            "entrypoints = [\"src/Compose.lex.tex\", \"src/Fixed.lex.tex\"]",
            "entrypoints = [\"src/Core.lex.tex\"]",
        );
        library.relock();
        library
    };
    let in_notation = library_with(&str::to_owned);
    assert!(
        in_notation
            .read_raw("src/Core.lex.tex")
            .contains("imports [Init, LexLeanTarget.TargetSemantics]"),
        "the committed module is written in the notation"
    );
    let as_data = library_with(&|text| lexlean::sdk::to_data_form(text).expect("the data form"));
    assert_eq!(
        in_notation.check_ok().semantic_id,
        as_data.check_ok().semantic_id,
        "Core: the notation does not link to its data form's module"
    );
    let (built_notation, built_data) = (in_notation.build_ok(), as_data.build_ok());
    assert_eq!(
        files(
            &in_notation
                .build_dir(&built_notation.build_id.expect("a build id"))
                .join("modules")
        ),
        files(
            &as_data
                .build_dir(&built_data.build_id.expect("a build id"))
                .join("modules")
        ),
        "Core: the generated Lean and LaTeX differ"
    );

    // Under language 1.0 and 1.1 the notation is refused.
    for (example, glossary) in [
        ("nat-add-zero", "lexlean.std.nat@1.0.0"),
        ("semantic-1.1", "lexlean.std.nat@1.1.0"),
    ] {
        let project = P::copy_example(example);
        let header = SourceHeader {
            name: "Main",
            glossaries: &[glossary],
            imports: &[],
            title: "Natural number addition",
        };
        project.write(
            MAIN,
            &assemble(&header, ModuleKind::Core, &small()).expect("a core source"),
        );
        project.check_fails_with("LLP2003");
    }
}

// ---------------------------------------------------------------------------
// AU-11.
// ---------------------------------------------------------------------------

fn fails_closed() {
    let project = project(&small(), true);
    let source = project.read_raw(MAIN);
    let located = locate(&source).expect("a module");
    let (from, to) = located.body;
    let original = source[from..to].to_owned();
    let put = |body: &str| {
        project.write(MAIN, &format!("{}{body}{}", &source[..from], &source[to..]));
    };
    project.check_ok();

    // Malformed in each way the statement names.
    let one = |text: &str, from: &str, to: &str| {
        assert!(text.contains(from), "{from:?} is not in the module");
        text.replacen(from, to, 1)
    };
    let nested = format!("@11 := {}Nat{}\n", "(".repeat(300), ")".repeat(300));
    let named: [(&str, String); 8] = [
        (
            "an unclosed delimiter",
            one(&original, "imports [Init]", "imports [Init"),
        ),
        (
            "a keyword used as a name",
            one(&original, "@1 := Nat", "@1 := fun"),
        ),
        (
            "a label that is not the number of its node",
            one(&original, "@1 := Nat", "@2 := Nat"),
        ),
        (
            "a reference to a node not yet written",
            one(&original, "@1 := Nat", "@1 := @7"),
        ),
        (
            "a statement that defines no node",
            one(&original, "@1 := Nat", "@1 := @0"),
        ),
        (
            "a statement that is cut",
            one(&original, "@1 := Nat", "@1 :="),
        ),
        ("a nesting beyond the bound", format!("{original}{nested}")),
        (
            "a declaration after the nodes",
            format!("{original}theorem Lib.t : @0\n"),
        ),
    ];
    for (what, text) in &named {
        assert_ne!(text, &original, "{what}: the plant changed nothing");
        put(text);
        let error = project.check_err();
        assert!(
            !error.diagnostics.is_empty(),
            "{what}: refused without a diagnostic"
        );
        for diagnostic in &error.diagnostics {
            let span = diagnostic
                .primary
                .as_ref()
                .unwrap_or_else(|| panic!("{what}: the diagnostic has no position"));
            assert!(span.line_start >= 1, "{what}: no line");
        }
    }
    put(&original);
    project.check_ok();

    // Every single-token deletion and duplication of the module, and its
    // replacement by a delimiter, is refused or links to a module that
    // decodes; none panics.
    let tokens = lex(&original).expect("the module lexes");
    assert!(tokens.len() > 100, "{} tokens", tokens.len());
    let (mut refused, mut linked) = (0usize, 0usize);
    for token in &tokens {
        let mutants = [
            format!("{}{}", &original[..token.start], &original[token.end..]),
            format!(
                "{}{} {}",
                &original[..token.end],
                &original[token.start..token.end],
                &original[token.end..]
            ),
            format!("{})", &original[..token.start]) + &original[token.end..],
            format!("{}{{{}", &original[..token.start], &original[token.end..]),
        ];
        for mutant in mutants {
            if mutant == original {
                continue;
            }
            put(&mutant);
            if project.check_err_or_ok() {
                linked += 1;
            } else {
                refused += 1;
            }
        }
    }
    assert!(
        refused > 100 && linked > 0,
        "{refused} refused, {linked} linked: the sweep did not exercise both outcomes"
    );
    put(&original);

    // A numeral with a redundant leading zero is `LLL1003` outside a string
    // and is string data inside one.
    put(&one(&original, "@2 := #0", "@2 := #00"));
    project.check_fails_with("LLL1003");
    put(&format!("{original}@11 := \"007\"\n"));
    project.check_ok();
}

// ---------------------------------------------------------------------------
// AU-12.
// ---------------------------------------------------------------------------

/// A module's notation with every number a statement is labelled by and every
/// reference to one replaced by a name, as a person who does not count nodes
/// writes it: the same module in text `fmt` does not write.
fn named_labels(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = String::with_capacity(text.len() + text.len() / 16);
    let mut copied = 0;
    for (at, byte) in bytes.iter().enumerate() {
        let after_margin_or_space = at == 0 || matches!(bytes[at - 1], b' ' | b'\n' | b'(' | b':');
        if *byte == b'@'
            && after_margin_or_space
            && bytes.get(at + 1).is_some_and(u8::is_ascii_digit)
        {
            out.push_str(&text[copied..=at]);
            out.push('n');
            copied = at + 1;
        }
    }
    out.push_str(&text[copied..]);
    out
}

/// The theorem the certification library gains, written as a person writes
/// it: no node counted, the variables named. `Eq.symm`, with every argument
/// and level explicit, which is all a kernel term has.
const AUTHORED_NAME: &str = "LexLeanCertification.authored_eq_symm";

const AUTHORED_DECLARATION: &str =
    "theorem LexLeanCertification.authored_eq_symm.{u} : @eq_symm_statement := @eq_symm_proof {
  all: [LexLeanCertification.authored_eq_symm],
  class: false,
  generated: false,
  policy: exact{axioms: []},
  transparency: semireducible
}
";

const AUTHORED_STATEMENTS: &str = "@eq_symm_statement := (α : Sort u) -> (a : α) -> (b : α) -> Eq.{u} α a b -> Eq.{u} α b a
@eq_symm_proof := fun (α : Sort u) => fun (a : α) => fun (b : α) => fun (h : Eq.{u} α a b) => Eq.rec.{0, u} α a (fun (b' : α) => fun (h' : Eq.{u} α a b') => Eq.{u} α b' a) (Eq.refl.{u} α a) b h
";

fn authored() {
    // A person who names the statements writes a module `fmt` writes with the
    // numbers: every committed module, with every label a name, reads to the
    // module it was and is written, by `fmt`, as the committed bytes.
    let mut relabelled = 0usize;
    for (shown, source) in committed() {
        let located = locate(&source).expect("a module");
        if located.data.is_some() {
            continue;
        }
        let body = &source[located.body.0..located.body.1];
        let module = notation::parse_module(body).expect("reads");
        let named = named_labels(body);
        assert_ne!(named, body, "{shown}: no label was replaced");
        let again = notation::parse_module(&named)
            .unwrap_or_else(|error| panic!("{shown}: the named labels do not read: {error}"));
        assert!(
            again == module,
            "{shown}: named labels read to another module"
        );
        assert_eq!(
            notation::print_module(&again).expect("prints"),
            body.strip_prefix('\n')
                .expect("a body begins with a line feed"),
            "{shown}: fmt does not write the numbers"
        );
        relabelled += 1;
    }
    assert!(
        relabelled >= 26,
        "{relabelled} modules: the walk did not read the product"
    );

    // A theorem a person adds to a library of kernel terms: the library
    // before it, the declaration and two statements written by name, and the
    // module the committed source is.
    let (shown, source) = committed()
        .into_iter()
        .find(|(shown, _)| shown == "compiler/proofs/src/Soundness.lex.tex")
        .expect("the last module of the certification library");
    let located = locate(&source).expect("a module");
    let body = &source[located.body.0..located.body.1];
    let committed_module = notation::parse_module(body).expect("reads");
    let theorem = committed_module
        .declarations
        .iter()
        .find(|row| row.name == AUTHORED_NAME)
        .unwrap_or_else(|| panic!("{shown}: the authored theorem is not declared"));
    let statement_at = body
        .find(&format!("\n@{} := ", theorem.r#type))
        .expect("the statement of the theorem")
        + 1;
    assert_eq!(
        body[statement_at..]
            .lines()
            .filter(|line| line.starts_with('@'))
            .count(),
        2,
        "the theorem is the last two statements of its module"
    );
    let from = body
        .find("theorem LexLeanCertification.authored_eq_symm")
        .expect("declared");
    let to = from + body[from..].find("\n}\n").expect("closed") + 3;
    let before = format!("{}{}", &body[..from], &body[to..statement_at]);
    let first_statement = before.find("\n\n@0 := ").expect("the nodes") + 1;
    let written = format!(
        "{}{AUTHORED_DECLARATION}{}{AUTHORED_STATEMENTS}",
        &before[..first_statement],
        &before[first_statement..]
    );
    let by_name = notation::parse_module(&written)
        .unwrap_or_else(|error| panic!("the authored theorem does not read: {error}"));
    assert!(
        by_name == committed_module,
        "the theorem written by name is not the committed module"
    );
    assert_eq!(
        notation::print_module(&by_name).expect("prints"),
        body.strip_prefix('\n')
            .expect("a body begins with a line feed"),
        "fmt does not write the committed source"
    );
    // The committed source keeps the names a person wrote.
    assert!(
        body.contains(" := fun (α : Sort u) =>"),
        "the committed theorem keeps the names of its binders"
    );

    // In a project: the authored text is not canonical, `fmt` writes the
    // numbers, and the module is the one the canonical notation is.
    let data = small();
    let canonical = project(&data, true);
    let text = canonical.read_raw(MAIN);
    let located = locate(&text).expect("a module");
    let body = text[located.body.0..located.body.1].to_owned();
    let authored_project = project(&data, true);
    authored_project.write(
        MAIN,
        &format!(
            "{}{}{}",
            &text[..located.body.0],
            named_labels(&body),
            &text[located.body.1..]
        ),
    );
    support::expect_code(
        &format_all(&authored_project, true)
            .err()
            .expect("a module with named statements is not canonical"),
        "LLL1003",
    );
    format_all(&authored_project, false).expect("fmt writes the numbers");
    assert_eq!(
        authored_project.read_raw(MAIN),
        text,
        "fmt did not write the canonical notation"
    );
    assert_eq!(
        authored_project.check_ok().semantic_id,
        canonical.check_ok().semantic_id
    );

    // A name that no statement has, a name two statements have, and a bound
    // variable given universe levels are refused with a position.
    for (what, mutant) in [
        (
            "an unknown label",
            body.replacen("@1 := Nat", "@1 := @nowhere", 1),
        ),
        (
            "a repeated label",
            format!("{body}@again := Nat\n@again := Nat\n"),
        ),
        (
            "a variable with levels",
            body.replacen("=> f\n", "=> f.{0}\n", 1),
        ),
    ] {
        assert_ne!(mutant, body, "{what}: the plant changed nothing");
        authored_project.write(
            MAIN,
            &format!(
                "{}{mutant}{}",
                &text[..located.body.0],
                &text[located.body.1..]
            ),
        );
        let error = authored_project.check_err();
        assert!(
            error
                .diagnostics
                .iter()
                .all(|diagnostic| diagnostic.primary.is_some()),
            "{what}: refused without a position"
        );
    }
}
