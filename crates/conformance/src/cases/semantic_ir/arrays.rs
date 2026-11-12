//! SM-39: the language-1.2 arrays of fixed-width integers (SPEC.md §17.12,
//! *Arrays*).

use lexlean::backend::runtime;

use crate::support::{self, P};

/// The committed example's source file.
const FILE: &str = "src/Main.lex.tex";

/// A language-1.2 module holding `declarations` and nothing else.
fn module(declarations: &str) -> String {
    format!(
        "\\begin{{lexlean}}{{Main}}\n\\useglossary{{lexlean.std.nat@1.2.0}}\n\\title{{Natural number addition}}\n\\begin{{semanticmodule}}\n{declarations}\\end{{semanticmodule}}\n\\end{{lexlean}}\n"
    )
}

/// Linking refuses a language-1.2 module holding `declarations` with a
/// message containing `message`.
fn refused(declarations: &str, message: &str) {
    let project = P::language_1_2_example();
    project.write(FILE, &module(declarations));
    let error = project.check_fails_with("LLT4001");
    assert!(
        error.to_string().contains(message),
        "expected {message:?}, got {error}"
    );
    project.assert_no_backend_output();
}

/// The arrays module of the shipped runtime.
fn arrays_module() -> &'static runtime::RuntimeModule {
    runtime::modules()
        .expect("the shipped runtime")
        .iter()
        .find(|module| module.name == runtime::ARRAYS)
        .expect("the runtime has the arrays module")
}

/// The module is LexLean source that imports the portable module alone, and
/// no row of it depends on more than `propext`.
fn runtime_module() {
    let arrays = arrays_module();
    assert_eq!(
        arrays.imports,
        [runtime::PORTABLE],
        "the arrays module imports the portable module alone"
    );
    for expected in [
        "LexLeanArrays.arrayIndexable",
        "LexLeanArrays.arrayLengthable",
        "LexLeanArrays.fromList",
        "LexLeanArrays.toList",
        "LexLeanArrays.set",
        "LexLeanArrays.to_list_from_list",
        "LexLeanArrays.from_list_to_list",
        "LexLeanArrays.length_to_list",
        "LexLeanArrays.index_to_list",
        "LexLeanArrays.index_beyond",
        "LexLeanArrays.length_set",
        "LexLeanArrays.index_set_same",
        "LexLeanArrays.index_set_other",
        "LexLeanArrays.set_beyond",
    ] {
        assert!(
            arrays.declarations.iter().any(|(name, _)| name == expected),
            "the arrays module declares {expected}"
        );
    }
    for (name, axioms) in &arrays.declarations {
        assert!(
            axioms.iter().all(|axiom| axiom == "propext"),
            "{name} depends on {axioms:?}, which is more than propext"
        );
    }
    // The definitions and the laws that hold by unfolding depend on nothing.
    for (name, axioms) in &arrays.declarations {
        if [
            "fromList",
            "toList",
            "set",
            "to_list_from_list",
            "from_list_to_list",
            "length_to_list",
        ]
        .iter()
        .any(|plain| name == &format!("LexLeanArrays.{plain}"))
        {
            assert!(axioms.is_empty(), "{name} depends on {axioms:?}");
        }
    }

    // Every registered array law is a declaration of the arrays module, and a
    // registry that names one the module does not have is refused.
    let registry =
        std::fs::read_to_string(support::repo_root().join("language/runtime-1.2/laws.toml"))
            .expect("the law registry");
    runtime::check_laws_registry(&registry).expect("the shipped registry is accepted");
    let planted = registry.replacen(
        "lean = \"LexLeanArrays.index_to_list\"",
        "lean = \"LexLeanArrays.index_to_lists\"",
        1,
    );
    assert_ne!(planted, registry, "the registry names the index law");
    let reason = runtime::check_laws_registry(&planted).expect_err("a missing law is refused");
    assert!(
        reason.contains(
            "`LexLeanArrays.index_to_lists` is not a declaration of `LexLeanRuntime.Arrays`"
        ),
        "{reason}"
    );
    // A law filed under the wrong module is refused too: the module a row
    // names is where its theorem is looked for.
    let misfiled = registry.replacen(
        "lean = \"LexLeanArrays.index_to_list\"\nmodule = \"LexLeanRuntime.Arrays\"",
        "lean = \"LexLeanArrays.index_to_list\"\nmodule = \"LexLeanRuntime.Portable\"",
        1,
    );
    assert_ne!(misfiled, registry, "the index law names its module");
    let reason = runtime::check_laws_registry(&misfiled).expect_err("a misfiled law is refused");
    assert!(
        reason.contains("is not a declaration of `LexLeanRuntime.Portable`"),
        "{reason}"
    );
}

/// What linking refuses, each before any backend runs.
fn refusals() {
    // An array holds one fixed-width integer type and nothing else.
    for (element, shown) in [
        ("Nat", "Nat"),
        ("Int", "Int"),
        ("Bool", "Bool"),
        ("Bytes", "Bytes"),
        ("(List Int8)", "List (Int8)"),
        ("array{element: Int8}", "Array (Int8)"),
    ] {
        refused(
            &format!(
                "definition{{\n  name: size,\n  parameters: [values : array{{element: {element}}}],\n  result: Nat,\n  body: 0\n}}\n"
            ),
            &format!("type {shown} cannot be the element of an array"),
        );
    }
    // Each primitive has one signature.
    refused(
        "definition{\n  name: put,\n  parameters: [values : array{element: Int8}, value : Int16],\n  result: array{element: Int8},\n  body: (prim array_set values 0 value : array{element: Int8})\n}\n",
        "primitive ArraySet argument 2 has type Int16, expected Int8",
    );
    refused(
        "definition{\n  name: put,\n  parameters: [values : (List Int8), value : Int8],\n  result: (List Int8),\n  body: (prim array_set values 0 value : (List Int8))\n}\n",
        "primitive ArraySet requires an array, not List (Int8)",
    );
    refused(
        "definition{\n  name: pack,\n  parameters: [values : (List Nat)],\n  result: array{element: Int8},\n  body: (prim array_from_list values : array{element: Int8})\n}\n",
        "type Nat cannot be the element of an array",
    );
    refused(
        "definition{\n  name: pack,\n  parameters: [values : (List Int8)],\n  result: array{element: Int16},\n  body: (prim array_from_list values : array{element: Int16})\n}\n",
        "primitive ArrayFromList result is Array (Int16), expected Array (Int8)",
    );
    refused(
        "definition{\n  name: unpack,\n  parameters: [values : (List Int8)],\n  result: (List Int8),\n  body: (prim array_to_list values : (List Int8))\n}\n",
        "primitive ArrayToList requires an array, not List (Int8)",
    );
    refused(
        "definition{\n  name: read,\n  parameters: [values : array{element: Int8}],\n  result: (Option Int16),\n  body: (prim index values 0 : (Option Int16))\n}\n",
        "primitive Index result is Option (Int16), expected Option (Int8)",
    );
    // An array is read by index, never by pattern.
    refused(
        "definition{\n  name: head,\n  parameters: [values : array{element: Int8}],\n  result: Nat,\n  body: (match values | List.nil => 0 | List.cons first rest => 1)\n}\n",
        "cannot pattern match value of type Array (Int8)",
    );
    // No array law is registered under a name the registry does not have.
    refused(
        "theorem{\n  name: bridge,\n  parameters: [values : array{element: Int8}],\n  statement: ((prim length values : Nat) = (prim length values : Nat)),\n  proof: law{name: array_size, type_arguments: [Int8], arguments: [values]}\n}\n",
        "`array_size` is not a registered law",
    );
    refused(
        "theorem{\n  name: bridge,\n  parameters: [values : (List Int8)],\n  statement: ((prim length values : Nat) = (prim length values : Nat)),\n  proof: law{name: array_length, type_arguments: [Int8], arguments: [values]}\n}\n",
        "law application argument has type List (Int8), expected Array (Int8)",
    );

    // Language 1.1 has neither the type nor the primitives.
    let eleven = P::semantic_example();
    let source = eleven.read(FILE);
    let anchor = r#""declarations":["#;
    assert!(
        source.contains(anchor),
        "a declaration list in the 1.1 fixture"
    );
    let declaration = r#"{"axioms":[],"body":{"kind":"nat","value":"0"},"kind":"definition","name":"size","parameters":[{"name":"values","type":{"element":{"kind":"int8"},"kind":"array"}}],"result":{"kind":"nat"}}"#;
    eleven.write(
        FILE,
        &source.replacen(anchor, &format!("{anchor}{declaration},"), 1),
    );
    let error = eleven.check_fails_with("LLT4001");
    assert!(
        error
            .to_string()
            .contains("`array type` is a language-1.2 construct"),
        "{error}"
    );

    // §17.13: an array is formal-only, so a production root over one fails
    // closed, naming the construct.
    let root = P::language_1_2_example();
    root.write(
        FILE,
        &module(
            "definition{\n  name: size,\n  parameters: [values : array{element: Int8}],\n  result: Nat,\n  body: (prim length values : Nat),\n  executable: true,\n  production: {effects: [], targets: [rust-std]}\n}\n",
        ),
    );
    let error = root.check_fails_with("LLT4005");
    assert!(
        error
            .to_string()
            .contains("holds an array, which has no production realization"),
        "{error}"
    );
}

pub(super) fn run() {
    runtime_module();

    let project = P::copy_example("arrays");
    project.fmt_check_ok();
    project.check_ok();
    let rendered = support::rendered(&project);
    let lean = support::lean_text(&rendered, "Main");
    for expected in [
        "public import LexLeanRuntime.Portable\n",
        "public import LexLeanRuntime.Arrays\n",
        "(left : _root_.Array (Int8))",
        "(LexLeanRuntime.index (left) (position) : Option (Int8))",
        "(LexLeanRuntime.length (left) : Nat)",
        "(_root_.LexLeanArrays.toList (value) : List (Int8))",
        "(_root_.LexLeanArrays.set (values) (position) (rectify («at» (values) (position))) : _root_.Array (Int8))",
        "_root_.LexLeanArrays.fromList",
        "exact @_root_.LexLeanArrays.index_to_list (Int8) (value) (offset)\n",
        "exact @_root_.LexLeanArrays.length_to_list (Int8) (value)\n",
        "exact @_root_.LexLeanArrays.length_set (Int8) (values) (position) (value)\n",
    ] {
        assert!(lean.contains(expected), "missing {expected:?} in:\n{lean}");
    }
    // The canonical document names the type and the primitives plainly.
    let tex = support::tex_text(&rendered, "Main");
    for expected in ["Array (Int8)", "LexLeanArrays.toList", "LexLeanArrays.set"] {
        assert!(tex.contains(expected), "the document states {expected:?}");
    }
    assert!(!tex.contains("_root_"), "the document names no Lean root");

    // A module that applies an array law and no fold law imports the arrays
    // module and not the laws module: a `law` proof imports the module that
    // states its law.
    let bridge = P::language_1_2_example();
    bridge.write(
        FILE,
        &module(
            "theorem{\n  name: bridge,\n  parameters: [values : array{element: UInt16}],\n  statement: ((prim length values : Nat) = (prim length (prim array_to_list values : (List UInt16)) : Nat)),\n  proof: law{name: array_length, type_arguments: [UInt16], arguments: [values]}\n}\n",
        ),
    );
    bridge.check_ok();
    let bridge_lean = support::lean_text(&support::rendered(&bridge), "Main");
    assert!(
        bridge_lean.contains("public import LexLeanRuntime.Arrays\n"),
        "{bridge_lean}"
    );
    assert!(
        !bridge_lean.contains("LexLeanRuntime.Laws") && !bridge_lean.contains("Collections"),
        "an array law imports the arrays module alone:\n{bridge_lean}"
    );
    // A module without an array construct imports no arrays module.
    let plain = P::language_1_2_example();
    plain.check_ok();
    assert!(
        !support::lean_text(&support::rendered(&plain), "Main").contains("LexLeanRuntime.Arrays"),
        "a module without arrays does not import the arrays module"
    );

    refusals();

    // The pinned Lean accepts the example with exactly its stated axioms, and
    // every element type, at both conversions.
    let _ = support::verify_ok_backed("SM-39", &project);
    if support::lean_backed("SM-39") {
        let widths = P::language_1_2_example();
        let mut declarations = String::new();
        for (index, ty) in [
            "Int8", "Int16", "Int32", "Int64", "UInt8", "UInt16", "UInt32", "UInt64",
        ]
        .into_iter()
        .enumerate()
        {
            if index > 0 {
                declarations.push('\n');
            }
            declarations.push_str(&format!(
                "theorem{{\n  name: round_trip_{},\n  parameters: [values : (List {ty})],\n  statement: ((prim array_to_list (prim array_from_list values : array{{element: {ty}}}) : (List {ty})) = values),\n  proof: law{{name: array_to_list_from_list, type_arguments: [{ty}], arguments: [values]}}\n}}\n",
                ty.to_lowercase()
            ));
        }
        widths.write(FILE, &module(&declarations));
        widths.check_ok();
        let _guard = support::env_shared();
        let _ = support::verify_ok(&widths);

        // A loop that skips every second element is not the list definition:
        // Lean refuses exactly the theorem that says it reads the list.
        let skipping = P::copy_example("arrays");
        let source = skipping.read(FILE);
        let from = "| Option.some y => (dotLoop left right (position + 1) rest";
        let to = "| Option.some y => (dotLoop left right (position + 2) rest";
        assert!(source.contains(from), "the array loop advances by one");
        skipping.write(FILE, &source.replacen(from, to, 1));
        skipping.check_ok();
        let error = skipping.verify_fails_with("LLV7002");
        assert!(
            error
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code.as_str() == "LLV7002"),
            "{error}"
        );

        // A law applied to a statement it does not prove is refused by Lean.
        let wrong = P::language_1_2_example();
        wrong.write(
            FILE,
            &module(
                "theorem{\n  name: bridge,\n  parameters: [values : array{element: Int8}, offset : Nat],\n  statement: ((prim index values (offset + 1) : (Option Int8)) = (prim index (prim array_to_list values : (List Int8)) offset : (Option Int8))),\n  proof: law{name: array_index, type_arguments: [Int8], arguments: [values, offset]},\n  axioms: [propext]\n}\n",
            ),
        );
        wrong.check_ok();
        wrong.verify_fails_with("LLV7002");
    }
}
