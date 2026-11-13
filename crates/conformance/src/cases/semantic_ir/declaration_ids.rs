//! SM-38: declaration IDs and statement IDs (SPEC.md §21.9).

use std::collections::BTreeMap;

use lexlean::artifact::canonical_json::Json;
use lexlean::artifact::content_id::Sha256Digest;
use serde_json::Value;
use sha2::Digest;

use crate::support::P;

/// The module of the case: `twice` references `base`, `twice_one` states a
/// fact about `twice`, `spare` is referenced by nothing, and `Tree` and
/// `Forest` are one mutual group.
fn module(base_literal: &str, expected: &str, proof: &str, leaf: &str) -> String {
    module_with(
        base_literal,
        expected,
        proof,
        leaf,
        "{name: empty, fields: []}",
    )
}

/// The module, with the constructors of `Shape` given.
fn module_with(
    base_literal: &str,
    expected: &str,
    proof: &str,
    leaf: &str,
    shapes: &str,
) -> String {
    format!(
        "\\begin{{lexlean}}{{Main}}\n\\useglossary{{lexlean.std.nat@1.2.0}}\n\\title{{Natural number addition}}\n\\begin{{semanticmodule}}\n\
definition{{\n  name: base,\n  parameters: [n : Nat],\n  result: Nat,\n  body: (n + {base_literal})\n}}\n\n\
definition{{\n  name: twice,\n  parameters: [n : Nat],\n  result: Nat,\n  body: ((base n) + (base n))\n}}\n\n\
theorem{{\n  name: twice_one,\n  parameters: [],\n  statement: ((twice 1) = {expected}),\n  proof: {proof}\n}}\n\n\
definition{{\n  name: spare,\n  parameters: [n : Nat],\n  result: Nat,\n  body: (n + 1)\n}}\n\n\
inductive{{\n  name: Tree,\n  type_parameters: [],\n  parameters: [],\n  constructors: [{{name: leaf, fields: [{leaf}]}}, {{name: node, fields: [Forest]}}],\n  mutual: Woods\n}}\n\n\
inductive{{\n  name: Forest,\n  type_parameters: [],\n  parameters: [],\n  constructors: [{{name: nil, fields: []}}, {{name: cons, fields: [Tree, Forest]}}],\n  mutual: Woods\n}}\n\n\
inductive{{\n  name: Shape,\n  type_parameters: [],\n  parameters: [],\n  constructors: [{shapes}]\n}}\n\n\
theorem{{\n  name: shape_refl,\n  parameters: [],\n  statement: ((ctor Shape.empty) = (ctor Shape.empty)),\n  proof: reflexivity{{}}\n}}\n\
\\end{{semanticmodule}}\n\\end{{lexlean}}\n"
    )
}

fn base_module() -> String {
    module("10", "22", "reflexivity{}", "Nat")
}

fn project_of(text: &str) -> P {
    let project = P::example();
    project.edit("lexlean.toml", "language = \"1.0\"", "language = \"1.2\"");
    project.write("src/Main.lex.tex", text);
    project.relock();
    project
}

/// The build's manifest output rows of one kind, as `(path, value)`.
fn outputs_of_kind(project: &P, kind: &str) -> Vec<(String, Value)> {
    let built = project.build_ok();
    let dir = project.build_dir(&built.build_id.expect("a build id"));
    let manifest: Value = serde_json::from_slice(
        &std::fs::read(dir.join("manifest.json").as_std_path()).expect("manifest"),
    )
    .expect("manifest JSON");
    manifest["outputs"]
        .as_array()
        .expect("outputs")
        .iter()
        .filter(|row| row["kind"] == kind)
        .map(|row| {
            let path = row["path"].as_str().expect("path").to_owned();
            let bytes = std::fs::read(dir.join(&path).as_std_path()).expect("declarations file");
            assert_eq!(
                Sha256Digest::of(&bytes).to_hex(),
                row["sha256"].as_str().expect("sha256"),
                "§21.6: the manifest row hashes the declarations file"
            );
            (
                path,
                serde_json::from_slice(&bytes).expect("declarations JSON"),
            )
        })
        .collect()
}

/// The declarations file of the project's module `Main`.
fn declarations(project: &P) -> Value {
    let rows = outputs_of_kind(project, "declarations");
    assert_eq!(rows.len(), 1, "one semantic module, one declarations file");
    assert!(
        rows[0].0.starts_with("declarations/") && rows[0].0.ends_with(".json"),
        "§21.9: the file is published under declarations/: {}",
        rows[0].0
    );
    rows[0].1.clone()
}

/// `name -> (id, statement_id)` of a declarations file.
fn ids(file: &Value) -> BTreeMap<String, (String, Option<String>)> {
    file["declarations"]
        .as_array()
        .expect("declarations")
        .iter()
        .map(|row| {
            (
                row["name"].as_str().expect("name").to_owned(),
                (
                    row["id"].as_str().expect("id").to_owned(),
                    row["statement_id"].as_str().map(str::to_owned),
                ),
            )
        })
        .collect()
}

/// The names whose declaration ID differs between two files.
fn changed(
    before: &BTreeMap<String, (String, Option<String>)>,
    after: &BTreeMap<String, (String, Option<String>)>,
) -> Vec<String> {
    before
        .iter()
        .filter(|(name, (id, _))| after.get(*name).is_some_and(|(other, _)| other != id))
        .map(|(name, _)| name.clone())
        .collect()
}

fn frame(hasher: &mut sha2::Sha256, label: &str, bytes: &[u8]) {
    hasher.update(u32::try_from(label.len()).expect("short").to_be_bytes());
    hasher.update(label.as_bytes());
    hasher.update((bytes.len() as u64).to_be_bytes());
    hasher.update(bytes);
}

/// The canonical JSON of one declaration of the checked project's `Main`.
fn declaration_json(project: &P, name: &str) -> String {
    let checked = crate::support::checked_project(project);
    let semantic = checked.modules["Main"]
        .document
        .semantic
        .as_ref()
        .expect("a semantic module");
    let declaration = semantic
        .declarations
        .iter()
        .find(|declaration| declaration.name() == name)
        .expect("the declaration");
    Json::parse(&serde_json::to_vec(declaration).expect("serializes"))
        .expect("canonical")
        .to_canonical_string()
}

/// §21.9 recomputed by hand for a declaration in a group of its own.
fn recompute(json: &str, name: &str, references: &[(&str, &str)]) -> String {
    let mut group = sha2::Sha256::new();
    group.update(b"lexlean-declaration-group-v1\0");
    frame(
        &mut group,
        "compiler-semantics",
        &lexlean::compiler_semantics_id_for("1.2").0,
    );
    frame(&mut group, "language", b"1.2");
    frame(&mut group, "toolchain", b"leanprover/lean4:v4.32.2");
    frame(&mut group, "declaration", json.as_bytes());
    for (reference, id) in references {
        frame(&mut group, "reference", reference.as_bytes());
        frame(
            &mut group,
            "reference-id",
            &Sha256Digest::from_hex(id).expect("hex").0,
        );
    }
    let group: [u8; 32] = group.finalize().into();
    let mut member = sha2::Sha256::new();
    member.update(b"lexlean-declaration-v1\0");
    frame(&mut member, "group", &group);
    frame(&mut member, "name", name.as_bytes());
    let id: [u8; 32] = member.finalize().into();
    Sha256Digest(id).to_hex()
}

/// Run the case of `SM-38`.
pub fn run() {
    let base = project_of(&base_module());
    let file = declarations(&base);

    // The schema, outside `schemas/`, validates the published file.
    let schema: Value = serde_json::from_slice(
        &std::fs::read(
            crate::support::repo_root()
                .join("api/schemas/declarations.schema.json")
                .as_std_path(),
        )
        .expect("the declarations schema"),
    )
    .expect("schema JSON");
    let violations = crate::schema::validate(&schema, &file);
    assert!(
        violations.is_empty(),
        "the declarations file violates api/schemas/declarations.schema.json: {violations:?}"
    );
    assert_eq!(file["spec"], "lexlean/declarations/1");
    assert_eq!(file["module"], "Main");
    let names: Vec<&str> = file["declarations"]
        .as_array()
        .expect("declarations")
        .iter()
        .map(|row| row["name"].as_str().expect("name"))
        .collect();
    assert_eq!(
        names,
        [
            "base",
            "twice",
            "twice_one",
            "spare",
            "Tree",
            "Forest",
            "Shape",
            "shape_refl"
        ],
        "§21.9: rows are in source order"
    );
    let before = ids(&file);
    let row = |name: &str| {
        file["declarations"]
            .as_array()
            .expect("declarations")
            .iter()
            .find(|row| row["name"] == name)
            .expect("row")
            .clone()
    };
    assert_eq!(
        row("twice")["references"],
        serde_json::json!([{"name": "base"}]),
        "§21.9: a reference into the own module has no `module`"
    );
    assert!(
        before["twice_one"].1.is_some() && before["twice"].1.is_none(),
        "§21.9: exactly theorems carry a statement ID"
    );

    // The IDs are exactly the specified framed inputs: `spare` references
    // nothing, `twice` references `base`.
    assert_eq!(
        before["spare"].0,
        recompute(&declaration_json(&base, "spare"), "spare", &[]),
        "§21.9: a declaration without references"
    );
    assert_eq!(
        before["twice"].0,
        recompute(
            &declaration_json(&base, "twice"),
            "twice",
            &[("base", &before["base"].0)]
        ),
        "§21.9: a declaration's ID covers the IDs of its references"
    );
    assert_ne!(
        before["Tree"].0, before["Forest"].0,
        "§21.9: members of one group have distinct IDs"
    );

    // One literal: exactly the definition and what reaches it.
    let literal = project_of(&module("11", "24", "reflexivity{}", "Nat"));
    let after = ids(&declarations(&literal));
    assert_eq!(
        changed(&before, &after),
        ["base", "twice", "twice_one"],
        "§21.9: a literal changes the IDs of its definition and of the declarations that reach it"
    );

    // A proof alone: the theorem's ID changes, its statement ID does not.
    let proof = project_of(&module("10", "22", "decide{}", "Nat"));
    let after = ids(&declarations(&proof));
    assert_eq!(changed(&before, &after), ["twice_one"]);
    assert_eq!(
        before["twice_one"].1, after["twice_one"].1,
        "§21.9: a change of proof keeps the statement ID"
    );

    // A group is hashed as one: changing `Tree` changes `Forest`.
    let group = project_of(&module("10", "22", "reflexivity{}", "Nat, Nat"));
    let after = ids(&declarations(&group));
    assert_eq!(changed(&before, &after), ["Forest", "Tree"]);

    // A generated member depends on its owner: `shape_refl` names only the
    // constructor `Shape.empty`, and a new constructor of `Shape` changes it.
    assert_eq!(
        row("shape_refl")["references"],
        serde_json::json!([{"name": "Shape"}]),
        "§21.9: a constructor reference is a reference to its type"
    );
    let shapes = project_of(&module_with(
        "10",
        "22",
        "reflexivity{}",
        "Nat",
        "{name: empty, fields: []}, {name: full, fields: []}",
    ));
    let after = ids(&declarations(&shapes));
    assert_eq!(changed(&before, &after), ["Shape", "shape_refl"]);
    assert_ne!(
        before["shape_refl"].1, after["shape_refl"].1,
        "§21.9: the statement ID covers the owner of a constructor it names"
    );

    // An added declaration and a new order change no ID.
    let spare = "definition{\n  name: spare,\n  parameters: [n : Nat],\n  result: Nat,\n  body: (n + 1)\n}\n\n";
    let text = base_module();
    assert!(text.contains(spare), "the case's module holds `spare`");
    let reordered = text.replace(spare, "").replacen(
        "\\begin{semanticmodule}\n",
        &format!("\\begin{{semanticmodule}}\n{spare}"),
        1,
    );
    let reordered = reordered.replace(
        "\\end{semanticmodule}",
        "\ndefinition{\n  name: extra,\n  parameters: [n : Nat],\n  result: Nat,\n  body: (n + 2)\n}\n\\end{semanticmodule}",
    );
    let after = ids(&declarations(&project_of(&reordered)));
    assert!(
        after.contains_key("extra"),
        "the added declaration is listed"
    );
    assert!(
        changed(&before, &after).is_empty(),
        "§21.9: adding and reordering independent declarations changes no ID"
    );
    assert_eq!(
        before
            .iter()
            .filter(|(name, _)| after.contains_key(*name))
            .count(),
        before.len()
    );

    // Languages 1.0 and 1.1 publish none.
    assert!(
        outputs_of_kind(&P::example(), "declarations").is_empty(),
        "§21.9: a language-1.0 build publishes no declarations file"
    );
}
