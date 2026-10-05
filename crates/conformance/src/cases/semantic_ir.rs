//! The `semantic-ir` suite: SM-01..SM-30.

use std::collections::BTreeSet;
use std::process::Command;

use sha2::Digest;

use crate::support::{self, P};

mod bytes;
mod names;
mod strings;

/// Every `"k"` and `"kind"` tag value in a canonical JSON document.
fn collect_tags(value: &serde_json::Value, key: &str, out: &mut BTreeSet<String>) {
    match value {
        serde_json::Value::Object(map) => {
            if let Some(serde_json::Value::String(tag)) = map.get(key) {
                out.insert(tag.clone());
            }
            for child in map.values() {
                collect_tags(child, key, out);
            }
        }
        serde_json::Value::Array(items) => {
            for child in items {
                collect_tags(child, key, out);
            }
        }
        _ => {}
    }
}

fn linked_json(project: &P) -> serde_json::Value {
    let checked = support::checked_project(project);
    serde_json::from_str(&checked.linked_json().to_canonical_string())
        .expect("linked IR is canonical JSON")
}

/// An application-shaped constant keeps all byte vectors inside one nested
/// declaration. Small scalar fixtures do not exercise the compiler work that
/// caused a real consumer's complete acceptance corpus to exhaust Lean's
/// default per-command heartbeat budget.
fn large_byte_declaration_project(varied: bool, language: &str) -> (P, Vec<String>) {
    use serde_json::json;

    let project = P::example();
    project.edit(
        "lexlean.toml",
        "language = \"1.0\"",
        &format!("language = {language:?}"),
    );
    let member = |name: &str| json!({"name": name});
    let named = |name: &str| json!({"kind":"named", "member":member(name), "arguments":[]});
    let bytes = |length: usize| {
        let hex = (0..length)
            .map(|index| format!("{:02x}", if varied { index % 256 } else { 120 }))
            .collect::<String>();
        json!({"kind":"bytes", "hex":hex})
    };
    let lengths = [
        (15, 67),
        (30, 82),
        (31, 83),
        (4096, 4148),
        (0, 46),
        (1, 46),
        (2, 46),
        (2, 46),
        (4097, 46),
        (8192, 46),
    ];
    let mut vectors = json!({"kind":"nil", "element":named("ByteVector")});
    for (request, response) in lengths.into_iter().rev() {
        let row = json!({
            "kind":"record", "type":member("ByteVector"), "type_arguments":[],
            "fields":[
                {"field":"request", "value":bytes(request)},
                {"field":"response", "value":bytes(response)},
            ],
        });
        vectors = json!({"kind":"cons", "head":row, "tail":vectors});
    }
    let module = json!({
        "spec":if language == "1.1" {"lexlean/semantic-module/1"} else {"lexlean/semantic-module/2"},
        "declarations":[
            {"kind":"structure", "name":"ByteVector", "type_parameters":[], "parameters":[],
             "fields":[{"name":"request", "type":{"kind":"bytes"}},
                       {"name":"response", "type":{"kind":"bytes"}}]},
            {"kind":"structure", "name":"ByteCorpus", "type_parameters":[], "parameters":[],
             "fields":[{"name":"before", "type":{"kind":"nat"}},
                       {"name":"vectors", "type":{"kind":"list", "element":named("ByteVector")}},
                       {"name":"after", "type":{"kind":"nat"}}]},
            {"kind":"definition", "name":"byteCorpus", "parameters":[
                {"name":"llb0", "type":{"kind":"nat"}},
                {"name":"llb255", "type":{"kind":"nat"}}
             ], "result":named("ByteCorpus"),
             "body":{"kind":"record", "type":member("ByteCorpus"), "type_arguments":[],
                     "fields":[{"field":"before", "value":{"kind":"var", "name":"llb0"}},
                               {"field":"vectors", "value":vectors},
                               {"field":"after", "value":{"kind":"var", "name":"llb255"}}]}},
        ],
    });
    project.write("src/Main.lex.tex", &format!(
        "\\begin{{lexlean}}{{Main}}\n\\useglossary{{lexlean.std.nat@{language}.0}}\n\\title{{Natural number addition}}\n\n\\begin{{semanticmodule}}\n\\semanticdata{{{module}}}\n\\end{{semanticmodule}}\n\\end{{lexlean}}\n"
    ));
    project.relock();
    let expected = lengths
        .into_iter()
        .flat_map(|(request, response)| {
            [request, response].map(|length| bytes(length)["hex"].as_str().expect("hex").to_owned())
        })
        .collect();
    (project, expected)
}

fn collect_byte_literals(value: &serde_json::Value, actual: &mut Vec<String>) {
    if let (Some("bytes"), Some(hex)) = (value["kind"].as_str(), value["hex"].as_str()) {
        actual.push(hex.to_owned());
    }
    match value {
        serde_json::Value::Array(items) => {
            for item in items {
                collect_byte_literals(item, actual);
            }
        }
        serde_json::Value::Object(fields) => {
            for value in fields.values() {
                collect_byte_literals(value, actual);
            }
        }
        _ => {}
    }
}

fn verify_large_byte_declaration(varied: bool, language: &str) {
    let (project, expected) = large_byte_declaration_project(varied, language);
    let first = project
        .engine()
        .snapshot(lexlean::CheckRequest {
            selection: lexlean::Selection::Entrypoints,
        })
        .expect("large byte corpus snapshot");
    let decoded: serde_json::Value =
        serde_json::from_slice(&first.canonical_bytes()).expect("snapshot JSON");
    let mut actual = Vec::new();
    assert_eq!(decoded["modules"].as_array().expect("modules").len(), 1);
    collect_byte_literals(&decoded["modules"][0]["semantic"], &mut actual);
    assert_eq!(
        actual, expected,
        "snapshot retains every literal byte, vector, and occurrence in order"
    );
    let rendered = support::rendered(&project);
    let generated = support::lean_text(&rendered, "Main");
    let octet = |value: &str| {
        value
            .strip_prefix("_root_.UInt8.ofNat (nat_lit ")
            .and_then(|value| value.strip_suffix(')'))
            .expect("explicit generated UInt8 construction")
            .parse::<u8>()
            .expect("exact generated UInt8 literal")
    };
    let mut remaining = generated.as_str();
    let mut emitted = Vec::new();
    while let Some((prefix, tail)) = remaining.split_once("_root_.ByteArray.mk #[") {
        let mut bindings = std::collections::BTreeMap::new();
        if let Some(start) = prefix.rfind("(let llb") {
            for binding in prefix[start + 1..]
                .split("; ")
                .filter(|part| !part.is_empty())
            {
                let (name, value) = binding
                    .strip_prefix("let ")
                    .and_then(|binding| binding.split_once(" : _root_.UInt8 := "))
                    .expect("closed typed local octet binding");
                assert!(bindings.insert(name, octet(value)).is_none());
            }
        }
        let (literal, rest) = tail.split_once(']').expect("closed generated array");
        emitted.push(if literal.is_empty() {
            String::new()
        } else {
            literal
                .split(", ")
                .map(|value| {
                    let value = bindings.get(value).copied().unwrap_or_else(|| octet(value));
                    format!("{value:02x}")
                })
                .collect::<String>()
        });
        remaining = rest;
    }
    assert_eq!(
        emitted, expected,
        "the actual generated declaration retains every byte in order"
    );
    let _ = support::verify_ok_backed("SM-19", &project);
    let second = project
        .engine()
        .snapshot(lexlean::CheckRequest {
            selection: lexlean::Selection::Entrypoints,
        })
        .expect("verified byte corpus snapshot");
    assert_eq!(
        first.canonical_bytes(),
        second.canonical_bytes(),
        "verification never substitutes the byte corpus"
    );
}

pub(crate) fn run(id: &str) {
    match id {
        // §17.1: phases run in order; nothing reaches a backend unlinked.
        "SM-01" => {
            // A normalization error and an unknown word in one file: the
            // earlier phase reports.
            let project = P::example();
            project.edit(
                "src/Main.lex.tex",
                "For every natural",
                "\tFor every banana natural",
            );
            let error = project.check_err();
            assert_eq!(
                error.diagnostics.first().map(|d| d.code.as_str()),
                Some("LLL1002"),
                "normalization precedes lexical resolution"
            );

            // A failed check emits no build artifacts.
            let failing = P::example();
            failing.edit(
                "src/Main.lex.tex",
                "For every natural",
                "For every banana natural",
            );
            let _ = failing
                .engine()
                .build(lexlean::BuildRequest {
                    selection: lexlean::Selection::Entrypoints,
                })
                .err()
                .expect("a build on a failing project fails");
            assert!(
                !failing.root.join(".lexlean/build").as_std_path().exists(),
                "no backend output exists for an unlinked program"
            );
        }
        // §17.2: closed reference kinds with stable identity.
        "SM-02" => {
            let project = support::defs_project();
            let mut kinds = BTreeSet::new();
            collect_tags(&linked_json(&project), "kind", &mut kinds);
            // The `kind` key also tags declaration kinds and policy kinds;
            // all three vocabularies are closed.
            let allowed: BTreeSet<String> = [
                "core",
                "external",
                "document",
                "defined",
                "typedefinition",
                "termdefinition",
                "predicatedefinition",
                "theorem",
                "lemma",
                "corollary",
                "none",
                "allow",
                "exact",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect();
            assert!(!kinds.is_empty(), "the fixture exercises reference kinds");
            assert!(
                kinds.is_subset(&allowed),
                "closed reference kinds only, found {kinds:?}"
            );
            // Stable identity: a second run reproduces the bytes and IDs.
            let first = support::checked_project(&project);
            let second = support::checked_project(&project);
            assert_eq!(first.semantic_id, second.semantic_id);
            assert_eq!(
                first.linked_json().to_canonical_string(),
                second.linked_json().to_canonical_string()
            );
        }
        // §17.3: term IR is exactly the closed variant set.
        "SM-03" | "SM-04" => {
            let mut tags = BTreeSet::new();
            for project in [P::example(), support::defs_project()] {
                collect_tags(&linked_json(&project), "k", &mut tags);
            }
            let allowed: BTreeSet<String> = [
                // §17.3 terms.
                "sort",
                "local",
                "global",
                "app",
                "pi",
                "lam",
                "let",
                "nat",
                // §17.4 proofs.
                "seq",
                "intro",
                "exact",
                "apply-one",
                "apply",
                "rfl",
                "witness",
                "left",
                "right",
                "have",
                "rw",
                "simp-only",
                "constructor",
                "cases",
                "induction",
                "calc",
                // §17.5 document phrase items and blocks.
                "word",
                "math",
                "punct",
                "declaration",
                "section",
                "definition",
                "theorem",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect();
            assert!(
                tags.is_subset(&allowed),
                "closed IR variant tags only, found extras: {:?}",
                tags.difference(&allowed).collect::<Vec<_>>()
            );
            for expected in ["app", "pi", "global", "rfl"] {
                assert!(tags.contains(expected), "the corpus exercises `{expected}`");
            }
        }
        // §17.6: conservative checks without kernel claims; an ill-typed
        // glossary application (too many explicit arguments) is rejected at
        // conversion with the entry named (S5).
        "SM-05" => {
            let overapplied = P::example();
            overapplied.add_package(
                "lexicons/test-arity",
                "test.arity",
                &["lexlean.core@1.0.0", "lexlean.std.nat@1.0.0"],
                &[
                    (
                        "nzz.toml",
                        &support::nzz_entry("Nat.le_refl").replace(
                            "(app (const lexlean.core::lnot) (app (const lexlean.std.nat::ne) (local n) (local n)))",
                            "(app (const lexlean.core::lnot) (app (const lexlean.std.nat::ne) (local n) (local n) (local n)))",
                        ),
                    ),
                    ("z.toml", Z_MATH),
                ],
            );
            overapplied.write(
                "src/Main.lex.tex",
                &support::nzz_module(&["test.arity@1.0.0"]),
            );
            // An over-applied signature is rejected when the lexicon closure
            // is built (§13.7: the signature is checked as an interface), so
            // it never reaches source elaboration.
            overapplied.relock();
            let error = overapplied.check_err();
            let diagnostic = error
                .diagnostics
                .iter()
                .find(|d| d.code.as_str() == "LLR3004")
                .unwrap_or_else(|| panic!("LLR3004 for an over-applied signature: {error}"));
            assert!(
                diagnostic.message.contains("lexlean.std.nat::ne")
                    && diagnostic.message.contains("3 explicit arguments"),
                "the diagnostic names the entry and the arity: {}",
                diagnostic.message
            );

            let ill_typed = P::example();
            ill_typed.edit(
                "src/Main.lex.tex",
                "\\(n + 0 = n\\)",
                "\\(n + 0 = n\\) and \\(n + 0\\)",
            );
            let error = ill_typed.check_err();
            assert!(
                error
                    .diagnostics
                    .iter()
                    .any(|d| matches!(d.code.as_str(), "LLT4001" | "LLP2001")),
                "a non-proposition conjunct fails conservative elaboration: {:?}",
                error
                    .diagnostics
                    .iter()
                    .map(|d| d.code.as_str())
                    .collect::<Vec<_>>()
            );

            // Expected types are read through definitions (§17.6, §17.7):
            // a binder typed by a document type definition satisfies `+`,
            // and a defined type noun (`natural number list`) meets a
            // polymorphic constant in either argument order — the solved
            // metavariable's solution unfolds too.
            support::f1_exact("SM-05", 
                "count_add_zero",
                "public theorem count_add_zero (llv0 : LexLeanExample.Main.count) : Eq (Nat.add llv0 0) llv0 := by\n  rfl",
            );
            support::f1_exact("SM-05", 
                "list_nil",
                "public theorem list_nil (llv0 : (List Nat)) : (Eq llv0 List.nil) → Eq List.nil llv0 := by\n  intro llh0\n  rw [llh0]",
            );
            // The core arrow elaborates in math to a non-dependent `Pi`
            // (§15.6), and the canonical formatter spells it back as `→`
            // inside an island where prose cannot hold it.
            support::f1_exact("SM-05", 
                "arrow_or",
                "public theorem arrow_or (llv0 : Prop) : Or (llv0 → llv0) llv0 := by\n  left\n  intro llh0\n  exact llh0",
            );
            let formatted = support::f1_project();
            formatted
                .engine()
                .format(lexlean::FormatRequest {
                    selection: lexlean::Selection::Entrypoints,
                    check_only: false,
                })
                .expect("the WS-F1 module formats");
            let source = formatted.read("src/Main.lex.tex");
            assert!(
                source.contains("\\(p → p\\) or \\(p\\)"),
                "the arrow inside an island keeps its math form: {source}"
            );
            formatted.fmt_check_ok();
            formatted.check_ok();
            // A rejection names why its candidates died (§20.1): an infix
            // entry whose document declaration is not yet available leaves
            // no candidate, and the `LLT4001` carries the reason as a note.
            let unavailable = support::f1_project();
            unavailable.edit(
                "src/Main.lex.tex",
                "For every count \\(c\\), \\(c + 0 = c\\).",
                "For every natural number \\(c\\), \\(c ⊕ 0 = c\\).",
            );
            let error = unavailable.check_err();
            let rejected = error
                .diagnostics
                .iter()
                .find(|d| d.code.as_str() == "LLT4001")
                .unwrap_or_else(|| panic!("an uninstantiable operator is LLT4001: {error}"));
            assert!(
                rejected.notes.iter().any(|note| {
                    note.message.contains("test.f1::oplus")
                        && note.message.contains("Main::oplus")
                        && note.message.contains("not available")
                }),
                "the note names the entry and why it cannot be instantiated: {rejected:#?}"
            );
        }
        // §17.3: omitted implicits recorded; user holes rejected.
        "SM-06" => {
            let project = P::example();
            let json = support::checked_project(&project)
                .linked_json()
                .to_canonical_string();
            assert!(
                json.contains("\"i\":["),
                "the Eq application records its omitted implicit binder: {json}"
            );
            let hole = P::example();
            hole.edit("src/Main.lex.tex", "\\(n + 0 = n\\)", "\\(n + _ = n\\)");
            hole.check_fails_with("LLL1004");
        }
        // §17.7: canonical signature comparison is alpha-safe.
        "SM-07" => {
            let project = support::defs_project();
            project.edit(
                "lexicons/test-defs/entries/double.toml",
                "(pi ((explicit n (const lexlean.std.nat::nat)))",
                "(pi ((explicit renamed (const lexlean.std.nat::nat)))",
            );
            project.relock();
            project.check_ok();
        }
        // §17.8: deterministic, collision-checked name generation.
        "SM-08" => {
            let project = P::example();
            let first = support::rendered(&project);
            let second = support::rendered(&project);
            assert_eq!(
                support::lean_text(&first, "Main"),
                support::lean_text(&second, "Main"),
                "generated names are deterministic"
            );

            let clash = P::example();
            let body = clash.read("src/Main.lex.tex");
            let second_theorem = "\n\\begin{theorem}{add-zero}\n\\noaxioms\nFor every natural number \\(m\\), \\(m + 0 = m\\).\n\\begin{proof}\nClose the goal by reflexivity.\n\\end{proof}\n\\end{theorem}\n\\end{lexlean}";
            clash.write(
                "src/Main.lex.tex",
                &body.replace("\n\\end{lexlean}", second_theorem),
            );
            let error = clash.check_err();
            assert!(
                error
                    .diagnostics
                    .iter()
                    .any(|d| matches!(d.code.as_str(), "LLP2003" | "LLR3002")),
                "a duplicate component ID is a collision: {:?}",
                error
                    .diagnostics
                    .iter()
                    .map(|d| d.code.as_str())
                    .collect::<Vec<_>>()
            );
            // A component converting to a pinned-Lean keyword is rejected
            // by name (C9); tactic names are not keywords.
            for keyword in [
                "def", "theorem", "at", "where", "instance", "with", "then", "let",
            ] {
                let project = P::example();
                project.edit(
                    "src/Main.lex.tex",
                    "\\begin{theorem}{add-zero}",
                    &format!("\\begin{{theorem}}{{{keyword}}}"),
                );
                let error = project.check_fails_with("LLP2003");
                let diagnostic = error
                    .diagnostics
                    .iter()
                    .find(|d| d.code.as_str() == "LLP2003")
                    .expect("matched");
                assert!(
                    diagnostic.message.contains("Lean keyword") && diagnostic.primary.is_some(),
                    "`{keyword}`: {}",
                    diagnostic.message
                );
            }
            for name in ["first", "left", "apply", "cases"] {
                let project = P::example();
                project.edit(
                    "src/Main.lex.tex",
                    "\\begin{theorem}{add-zero}",
                    &format!("\\begin{{theorem}}{{{name}}}"),
                );
                project.check_ok();
            }

            // A binder nothing references keeps its §17.8 index and carries
            // the `_` prefix that marks a deliberate binding: pinned Lean's
            // unused-variable linter warns otherwise, and a warning fails
            // verification (§20.2). The theorem still states, and proves,
            // the quantified proposition the source wrote.
            let unused = P::example();
            unused.edit(
                "src/Main.lex.tex",
                "For every natural number \\(n\\), \\(n + 0 = n\\).",
                "For every natural number \\(n\\) and natural number \\(m\\), \\(n + 0 = n\\).",
            );
            let lean = support::lean_text(&support::rendered(&unused), "Main");
            assert!(
                lean.contains(
                    "public theorem add_zero (llv0 : Nat) (_llv1 : Nat) : Eq (Nat.add llv0 0) llv0 := by"
                ),
                "an unreferenced lifted binder is `_llv1`: {lean}"
            );
            support::verify_ok_backed("SM-08", &unused);

            let unused_hypothesis = P::example();
            unused_hypothesis.edit(
                "src/Main.lex.tex",
                "For every natural number \\(n\\), \\(n + 0 = n\\).\n\\begin{proof}\nClose the goal by reflexivity.",
                "For every natural number \\(n\\), if \\(n = n\\), then \\(n + 0 = n\\).\n\\begin{proof}\nAssume \\(h\\).\nClose the goal by reflexivity.",
            );
            let lean = support::lean_text(&support::rendered(&unused_hypothesis), "Main");
            assert!(
                lean.contains("  intro _llh0\n"),
                "an unreferenced introduced hypothesis is `_llh0`: {lean}"
            );
            support::verify_ok_backed("SM-08", &unused_hypothesis);

            // Semantic-module binders follow the same rule: a definition
            // parameter, quantifier, `let`, or lambda parameter its scope
            // never mentions lowers as `_name`, and pinned Lean verifies the
            // module instead of failing it on a linter warning.
            let eleven = P::semantic_example();
            let support_source = eleven.read("src/Support.lex.tex");
            eleven.write(
                "src/Support.lex.tex",
                &support_source.replacen(
                    r#"{"declarations":["#,
                    r#"{"declarations":[{"body":{"kind":"bool","value":true},"kind":"definition","name":"constantTrue","parameters":[{"name":"ignored","type":{"kind":"nat"}}],"result":{"kind":"bool"}},{"body":{"binder":{"name":"ignored","type":{"kind":"nat"}},"body":{"kind":"eq","left":{"kind":"nat","value":"0"},"right":{"kind":"nat","value":"0"}},"kind":"forall"},"kind":"definition","name":"vacuous","parameters":[],"result":{"kind":"prop"}},"#,
                    1,
                ),
            );
            let lean = support::lean_text(&support::rendered(&eleven), "Support");
            for expected in [
                "public def constantTrue (_ignored : Nat) : Bool := true",
                "public def vacuous : Prop := (forall (_ignored : Nat), (0 = 0))",
            ] {
                assert!(lean.contains(expected), "missing {expected:?} in:\n{lean}");
            }
            support::verify_ok_backed("SM-08", &eleven);
            let twelve = P::copy_example("higher-order");
            let combinators = twelve.read("src/Combinators.lex.tex");
            twelve.write(
                "src/Combinators.lex.tex",
                &combinators.replacen(
                    r#"{"declarations":["#,
                    r#"{"declarations":[{"body":{"binder":{"name":"spare","type":{"kind":"nat"}},"body":{"arguments":[{"kind":"nat","value":"0"}],"function":{"body":{"kind":"nat","value":"1"},"captures":[],"kind":"lambda","parameters":[{"name":"value","type":{"kind":"nat"}}]},"kind":"apply"},"kind":"let","value":{"kind":"nat","value":"2"}},"kind":"definition","name":"constantOne","parameters":[],"result":{"kind":"nat"}},"#,
                    1,
                ),
            );
            let lean = support::lean_text(&support::rendered(&twelve), "Combinators");
            let expected = "public def constantOne : Nat := (let _spare : Nat := 2; ((fun (_value : Nat) => 1) (0)))";
            assert!(lean.contains(expected), "missing {expected:?} in:\n{lean}");
            support::verify_ok_backed("SM-08", &twelve);
        }
        // §17.9: alpha-safe serialization with dense binder indices.
        "SM-09" => {
            let with_n = P::example();
            let with_m = P::example();
            with_m.write(
                "src/Main.lex.tex",
                &with_n
                    .read("src/Main.lex.tex")
                    .replace("(n", "(m")
                    .replace("= n", "= m"),
            );
            let key = |project: &P| {
                let checked = support::checked_project(project);
                let module = &checked.modules["Main"];
                let declaration = module
                    .document
                    .declarations()
                    .into_iter()
                    .find(|d| d.component == "add-zero")
                    .expect("the theorem");
                match &declaration.body {
                    lexlean::ir::declaration::DeclBody::TheoremLike { statement, .. } => {
                        statement.canonical_key()
                    }
                    lexlean::ir::declaration::DeclBody::Definition { .. } => {
                        panic!("the fixture is a theorem")
                    }
                }
            };
            assert_eq!(
                key(&with_n),
                key(&with_m),
                "alpha-renamed statements share one canonical key"
            );
        }
        // §21.3: the source ID is exactly the specified framed hash.
        "SM-10" => {
            let project = P::example();
            let inner =
                lexlean::project::Project::load(&project.root.join("lexlean.toml")).expect("load");
            let checked = support::checked_project(&project);
            let mut hasher = sha2::Sha256::new();
            hasher.update(b"lexlean-source-v1\0");
            let frame = |hasher: &mut sha2::Sha256, label: &str, bytes: &[u8]| {
                hasher.update(u32::try_from(label.len()).expect("short").to_be_bytes());
                hasher.update(label.as_bytes());
                hasher.update((bytes.len() as u64).to_be_bytes());
                hasher.update(bytes);
            };
            frame(
                &mut hasher,
                "project",
                inner.config.canonical_toml().as_bytes(),
            );
            frame(&mut hasher, "lock", &checked.canonical_lock);
            frame(&mut hasher, "path", b"src/Main.lex.tex");
            frame(
                &mut hasher,
                "source",
                checked.modules["Main"].normalized.as_bytes(),
            );
            let manual: [u8; 32] = hasher.finalize().into();
            assert_eq!(
                lexlean::artifact::content_id::Sha256Digest(manual),
                checked.source_id,
                "§21.3: the source ID equals its manual recomputation"
            );

            // §21.4: the semantic ID is the specified function of the
            // linked IR and closure serializations.
            let module_closure = checked
                .closure
                .closure_json("", &checked.visible_union)
                .to_canonical_string();
            let recomputed = lexlean::artifact::content_id::semantic_id(
                lexlean::compiler_semantics_id(),
                &checked.linked_json().to_canonical_string(),
                &module_closure,
            );
            assert_eq!(
                recomputed, checked.semantic_id,
                "§21.4: the semantic ID equals the specified framed inputs"
            );
        }
        // §24.4: complete result sets in stable order.
        "SM-11" => {
            let project = P::example();
            project.write(
                "src/Helper.lex.tex",
                &project
                    .read("src/Main.lex.tex")
                    .replace("{Main}", "{Helper}"),
            );
            project.edit(
                "src/Main.lex.tex",
                "\\useglossary{lexlean.std.nat@1.0.0}",
                "\\useglossary{lexlean.std.nat@1.0.0}\n\\importmodule{Helper}",
            );
            let checked = project.check_ok();
            let names: Vec<&String> = checked.units.keys().collect();
            assert_eq!(
                names,
                ["Helper", "Main"],
                "sorted, each module exactly once"
            );
            let all = project
                .engine()
                .check(lexlean::CheckRequest {
                    selection: lexlean::Selection::All,
                })
                .expect("--all checks");
            assert_eq!(
                all.units.keys().collect::<Vec<_>>(),
                names,
                "every selection returns the same complete sorted set"
            );
        }
        // §17.5, I7: no opaque prose inside semantic IR.
        "SM-12" => {
            let project = P::example();
            let json = support::checked_project(&project)
                .linked_json()
                .to_canonical_string();
            for prose in [
                "natural number",
                "For every",
                "reflexivity",
                "Close the goal",
            ] {
                assert!(
                    !json.contains(prose),
                    "linked IR must not embed source prose ({prose:?})"
                );
            }
        }
        // §15.4: inherited parameters are explicit, and emitted only where
        // used.
        "SM-13" => {
            let project = P::example();
            project.write(
                "src/Main.lex.tex",
                "\\begin{lexlean}{Main}\n\\useglossary{lexlean.std.nat@1.0.0}\n\\title{Natural number addition}\n\n\\begin{section}{basics}\n\\heading{Natural number addition}\n\\parameters{natural number \\(p\\)}\n\\begin{theorem}{uses-param}\n\\noaxioms\n\\(p + 0 = p\\).\n\\begin{proof}\nClose the goal by reflexivity.\n\\end{proof}\n\\end{theorem}\n\\begin{theorem}{ignores-param}\n\\noaxioms\nFor every natural number \\(m\\), \\(m + 0 = m\\).\n\\begin{proof}\nClose the goal by reflexivity.\n\\end{proof}\n\\end{theorem}\n\\end{section}\n\\end{lexlean}\n",
            );
            let checked = support::checked_project(&project);
            let module = &checked.modules["Main"];
            let params_of = |component: &str| {
                module
                    .document
                    .declarations()
                    .into_iter()
                    .find(|d| d.component == component)
                    .unwrap_or_else(|| panic!("{component} exists"))
                    .params
                    .len()
            };
            assert_eq!(
                params_of("uses-param"),
                1,
                "the using theorem inherits the binder"
            );
            assert_eq!(params_of("ignores-param"), 0, "the non-user emits none");
            // A reference to a parameterized declaration applies the
            // parameters explicitly, inside and outside its section, and
            // Lean-verifies (C2, S4).
            if let Some(fixture) = support::corpus_backed("SM-13") {
                assert_eq!(fixture.attestation["status"], "verified");
            }
            assert_eq!(
                support::corpus_declaration_lean("use_inside"),
                "public theorem use_inside (llv0 : Nat) : Eq (Nat.succ (Nat.add llv0 0)) (Nat.succ llv0) := by\n  apply LexLeanExample.Main.succ_congr\n  exact LexLeanExample.Main.param_add_zero llv0"
            );
            assert_eq!(
                support::corpus_declaration_lean("use_outside"),
                "public theorem use_outside (llv0 : Nat) : Eq (Nat.add llv0 0) llv0 := by\n  exact LexLeanExample.Main.param_add_zero llv0"
            );
            // The bare reference has the parameter-abstracted type: it does
            // not close a goal that expects the instantiated statement.
            let bare = support::corpus_project();
            bare.edit(
                "src/Main.lex.tex",
                "Close the goal with \\(\\reference{Main::param-add-zero}(q)\\).",
                "Close the goal with \\(\\reference{Main::param-add-zero}\\).",
            );
            bare.check_fails_with("LLT4001");
        }
        // §15.5: numerals require a unique expected type.
        "SM-14" => {
            let project = P::example();
            project.edit("src/Main.lex.tex", "\\(n + 0 = n\\)", "\\(1 = 1\\)");
            let error = project.check_err();
            let diagnostic = error
                .diagnostics
                .iter()
                .find(|d| d.code.as_str() == "LLT4001")
                .unwrap_or_else(|| {
                    panic!("a numeral without a unique expected type is LLT4001: {error}")
                });
            assert!(
                diagnostic.message.contains("numeral `1`") && diagnostic.primary.is_some(),
                "the numeral is named: {}",
                diagnostic.message
            );
            // A numeral typed by a binder, an operator, or a witness slot is
            // accepted (corpus: `Use \\(0\\) as the witness`, `\\(0\\) is even`).
            if let Some(fixture) = support::corpus_backed("SM-14") {
                assert_eq!(fixture.attestation["status"], "verified");
            }
            assert_eq!(
                support::corpus_declaration_lean("zero_even"),
                "public theorem zero_even : LexLeanExample.Main.even 0 := by\n  refine ⟨(0 : Nat), ?_⟩\n  rfl"
            );
        }
        // §17.10: a native core module is semantic data, not backend text.
        "SM-15" => {
            const CORE_SOURCE: &str = "\\begin{lexlean}{Main}\n\\useglossary{lexlean.std.nat@1.0.0}\n\\title{Natural number addition}\n\\begin{coremodule}\n\\coredata{{\"declarations\":[{\"class\":false,\"generated\":false,\"kind\":\"theorem\",\"levels\":[],\"name\":\"CoreFixture.truth\",\"policy\":{\"kind\":\"none\"},\"transparency\":\"semireducible\",\"type\":0,\"value\":1}],\"imports\":[\"Init\"],\"nodes\":[{\"k\":\"c\",\"n\":\"True\",\"u\":[]},{\"k\":\"c\",\"n\":\"True.intro\",\"u\":[]}],\"proof_nodes\":[1],\"spec\":\"lexlean/core-module/1\"}}\n\\end{coremodule}\n\\end{lexlean}\n";
            let project = P::example();
            project.write("src/Main.lex.tex", CORE_SOURCE);
            let checked = support::checked_project(&project);
            let document = &checked.modules["Main"].document;
            let core = document.core.as_ref().expect("native core module");
            assert!(
                document.blocks.is_empty(),
                "native and prose forms are exclusive"
            );
            assert_eq!(core.nodes.len(), 2);
            assert_eq!(core.declarations.len(), 1);
            assert_eq!(core.declarations[0].policy.kind(), "none");
            support::assert_schema(
                "core-module",
                "native core fixture",
                &serde_json::to_value(core).expect("core JSON"),
            );

            let built = project.build_ok();
            let build_root = project.build_dir(&built.build_id.expect("build id"));
            let unit = &built.units["Main"];
            let mut lean = None;
            let mut tex = None;
            for relative in &unit.artifacts.paths {
                let text = std::fs::read_to_string(build_root.join(relative).as_std_path())
                    .expect("generated artifact");
                if relative.ends_with(".lean") {
                    lean = Some(text);
                } else if relative.ends_with(".tex") {
                    tex = Some(text);
                }
            }
            let lean = lean.expect("generated Lean");
            let tex = tex.expect("generated LaTeX");
            assert!(
                lean.contains("LexLeanCore.Runtime.decodeAndAdd")
                    && lean.contains("CoreFixture.truth"),
                "Lean reconstructs the same native declaration"
            );
            assert!(
                tex.contains("CoreFixture.truth") && tex.contains("True.intro"),
                "LaTeX traverses the same type and proof DAG"
            );
            let _ = support::verify_ok_backed("SM-15", &project);

            let invalid = P::example();
            invalid.write(
                "src/Main.lex.tex",
                &CORE_SOURCE.replace(
                    "\"imports\":[\"Init\"]",
                    "\"imports\":[\"Init\"],\"lean\":\"theorem truth := True.intro\"",
                ),
            );
            invalid.check_fails_with("LLI9001");
        }
        // §17.11, §24.1: the public snapshot carries every high-level
        // semantic variant as owned nested data and remains backend-free.
        "SM-16" => {
            let project = support::semantic_project();
            let engine = project.engine();
            let request = || lexlean::CheckRequest {
                selection: lexlean::Selection::Entrypoints,
            };
            let first = engine.snapshot(request()).expect("snapshot");
            let second = engine.snapshot(request()).expect("repeat snapshot");
            let _built = project.build_ok();
            let third = engine.snapshot(request()).expect("post-build snapshot");
            assert_eq!(first.canonical_bytes(), second.canonical_bytes());
            assert_eq!(first.canonical_bytes(), third.canonical_bytes());
            assert_eq!(first.snapshot_id(), second.snapshot_id());
            assert_eq!(first.snapshot_id(), third.snapshot_id());
            assert_eq!(first.language(), "1.1");
            let module = first
                .modules()
                .iter()
                .find(|module| module.name() == "Main")
                .expect("the all-declaration-variant Main module");
            assert!(module.core().is_none());
            let semantic = module.semantic().expect("typed semantic module");
            let typed_kinds = semantic
                .declarations
                .iter()
                .map(lexlean::SnapshotSemanticDeclaration::kind)
                .collect::<BTreeSet<_>>();
            assert_eq!(
                typed_kinds,
                BTreeSet::from([
                    "class",
                    "definition",
                    "inductive",
                    "instance",
                    "structure",
                    "theorem",
                ]),
                "every declaration variant is readable through stable public snapshot types"
            );
            support::check_downstream_snapshot_api();
            let value: serde_json::Value =
                serde_json::from_slice(&first.canonical_bytes()).expect("JSON");
            support::assert_schema("semantic-snapshot", "1.1 all-variant snapshot", &value);
            let mut kinds = BTreeSet::new();
            collect_tags(&value, "kind", &mut kinds);
            for expected in [
                "structure",
                "class",
                "instance",
                "inductive",
                "definition",
                "theorem",
                "var",
                "nat",
                "bool",
                "unit",
                "nil",
                "cons",
                "record",
                "constructor",
                "instance_value",
                "project",
                "call",
                "if",
                "match",
                "eq",
                "le",
                "lt",
                "add",
                "beq",
                "ble",
                "blt",
                "and",
                "prop_and",
                "or",
                "not",
                "implies",
                "iff",
                "forall",
                "cases",
                "induction",
                "simplify",
                "reflexivity",
                "decide",
                "congruence",
                "boolean_reflection",
                "apply",
            ] {
                assert!(
                    kinds.contains(expected),
                    "snapshot exercises `{expected}`: {kinds:?}"
                );
            }
            let bytes = String::from_utf8(first.canonical_bytes()).expect("utf8");
            assert!(
                bytes.contains("\"axiom_policy\":{\"axioms\":[\"propext\"],\"kind\":\"exact\"}"),
                "snapshot retains a nonempty exact semantic theorem policy"
            );
            if let Some(verified) = support::verify_ok_backed("SM-16", &project) {
                let attestation: serde_json::Value = serde_json::from_slice(
                    &std::fs::read(verified.root.join("attestation.json").as_std_path())
                        .expect("language-1.1 attestation"),
                )
                .expect("attestation JSON");
                let expected = lexlean::compiler_semantics_id_for(lexlean::LANGUAGE_1_1).to_hex();
                assert_eq!(
                    attestation["lexlean"]["compiler_semantics"].as_str(),
                    Some(expected.as_str()),
                    "language-1.1 attestation binds the selected compiler semantics"
                );
            }
            assert!(!bytes.contains(project.root.as_str()));
            assert!(!bytes.contains("public structure"));
            assert!(!bytes.contains("namespace SemanticFixture"));
        }
        "SM-17" => {
            let project = support::semantic_project();
            let snapshot = project
                .engine()
                .snapshot(lexlean::CheckRequest {
                    selection: lexlean::Selection::Entrypoints,
                })
                .expect("portable snapshot");
            let value: serde_json::Value =
                serde_json::from_slice(&snapshot.canonical_bytes()).expect("snapshot JSON");
            let mut kinds = BTreeSet::new();
            collect_tags(&value, "kind", &mut kinds);
            for expected in [
                "int", "int8", "int16", "int32", "int64", "uint8", "uint16", "uint32", "uint64",
                "string", "bytes", "option", "result",
            ] {
                assert!(kinds.contains(expected), "missing portable type {expected}");
            }
            let text = String::from_utf8(snapshot.canonical_bytes()).expect("utf8 snapshot");
            for representation in [
                "int", "int8", "int16", "int32", "int64", "uint8", "uint16", "uint32", "uint64",
            ] {
                assert!(
                    text.contains(&format!("\"representation\":\"{representation}\"")),
                    "missing literal representation {representation}"
                );
            }
            assert!(text.contains("\"hex\":\"aabb7fff\""));
        }
        "SM-18" => {
            let project = support::semantic_project();
            let snapshot = project
                .engine()
                .snapshot(lexlean::CheckRequest {
                    selection: lexlean::Selection::Entrypoints,
                })
                .expect("portable snapshot");
            let value: serde_json::Value =
                serde_json::from_slice(&snapshot.canonical_bytes()).expect("snapshot JSON");
            fn operations(value: &serde_json::Value, out: &mut BTreeSet<String>) {
                match value {
                    serde_json::Value::Object(map) => {
                        if let Some(serde_json::Value::String(operation)) = map.get("operation") {
                            out.insert(operation.clone());
                        }
                        for child in map.values() {
                            operations(child, out);
                        }
                    }
                    serde_json::Value::Array(items) => {
                        for child in items {
                            operations(child, out);
                        }
                    }
                    _ => {}
                }
            }
            let mut found = BTreeSet::new();
            operations(&value, &mut found);
            let expected = BTreeSet::from([
                "append",
                "bit_and",
                "bit_not",
                "bit_or",
                "bit_xor",
                "checked_add",
                "checked_convert",
                "checked_multiply",
                "checked_negate",
                "checked_quotient",
                "checked_subtract",
                "compare_bytes",
                "equal",
                "format_decimal",
                "index",
                "join",
                "length",
                "multiply",
                "negate",
                "parse_decimal",
                "quotient",
                "remainder",
                "shift_left",
                "shift_right",
                "slice",
                "split_exact",
                "subtract",
                "utf8_decode",
                "utf8_encode",
            ])
            .into_iter()
            .map(str::to_owned)
            .collect();
            assert_eq!(
                found, expected,
                "every closed portable primitive is exercised"
            );
            let lean = project.build_ok();
            let generated = std::fs::read_to_string(
                project
                    .build_dir(&lean.build_id.expect("build id"))
                    .join("modules/SemanticFixture/Portable.lean")
                    .as_std_path(),
            )
            .expect("portable generated Lean");
            assert!(generated.contains("public def isZeroInt64"));
            assert!(generated.contains("\"portable ✓\""));

            let mismatched = support::semantic_project();
            mismatched.edit(
                "src/Portable.lex.tex",
                "\"representation\":\"int64\",\"value\":\"0\"",
                "\"representation\":\"uint64\",\"value\":\"0\"",
            );
            let error = mismatched.check_err();
            assert!(
                error
                    .to_string()
                    .contains("has type UInt64, expected Int64"),
                "unexpected mismatch diagnostic: {error}"
            );
        }
        "SM-19" => {
            bytes::verify();
            strings::verify();
            names::verify();
            for language in ["1.1", "1.2"] {
                verify_large_byte_declaration(false, language);
                verify_large_byte_declaration(true, language);
            }
            let project = support::semantic_project();
            let built = project.build_ok();
            let root = project.build_dir(&built.build_id.expect("build id"));
            let lean = std::fs::read_to_string(
                root.join("modules/SemanticFixture/Portable.lean")
                    .as_std_path(),
            )
            .expect("portable generated Lean");
            for needle in [
                "public class Fixed",
                "public def checkedAdd",
                "public def utf8Decode",
                "public def parseDecimal",
                "public def checkedAddInt64",
            ] {
                assert!(lean.contains(needle), "generated Lean is missing {needle}");
            }
            let _ = support::verify_ok_backed("SM-19", &project);
            if support::lean_backed("SM-19") {
                let evaluations = r#"open SemanticFixture.Portable
#eval subtractInt 5 8 == -3
#eval multiplyInt (-7) 6 == -42
#eval quotientInt (-7) 2 99 == -3
#eval quotientInt 1 0 99 == 99
#eval remainderInt (-7) 2 99 == -1
#eval negateInt 42 == -42
#eval isZeroInt64 0
#eval !(isZeroInt64 1)
#eval convertInt64 42 == some 42
#eval checkedAddInt64 40 2 == some 42
#eval checkedSubtractInt64 40 2 == some 38
#eval checkedMultiplyInt64 6 7 == some 42
#eval checkedNegateInt64 42 == some (-42)
#eval checkedQuotientInt64 (-7) 2 == some (-3)
#eval andUInt64 12 10 == 8
#eval orUInt64 12 10 == 14
#eval xorUInt64 12 10 == 6
#eval notUInt64 0 == 18446744073709551615
#eval shiftUInt64 1 3 == some 8
#eval shiftRightUInt64 8 3 == some 1
#eval appendBytes (ByteArray.mk #[1]) (ByteArray.mk #[2]) == ByteArray.mk #[1, 2]
#eval byteLength (ByteArray.mk #[1, 2]) == 2
#eval byteAt (ByteArray.mk #[1, 2]) 1 == some 2
#eval sliceBytes (ByteArray.mk #[1, 2, 3]) 1 2 == some (ByteArray.mk #[2, 3])
#eval encodeUtf8 "A" == ByteArray.mk #[65]
#eval decodeUtf8 (ByteArray.mk #[65]) == some "A"
#eval compareByteStrings (ByteArray.mk #[1]) (ByteArray.mk #[2]) == Ordering.lt
#eval splitBounded "a::b" "::" 2 == some ["a", "b"]
#eval joinStrings ["a", "b"] "::" == "a::b"
#eval parseInt64 "-42" == some (-42)
#eval formatInt64 (-42) == "-42"
#eval checkedAddInt64 9223372036854775807 1 == none
#eval checkedQuotientInt64 (-9223372036854775808) (-1) == none
#eval shiftUInt64 1 64 == none
#eval byteAt (ByteArray.mk #[1, 2]) 2 == none
#eval sliceBytes (ByteArray.mk #[1, 2, 3]) 2 2 == none
#eval decodeUtf8 (ByteArray.mk #[255]) == none
#eval splitBounded "a::b" "::" 1 == none
#eval parseInt64 "01" == none
"#;
                let directory = tempfile::Builder::new()
                    .prefix("lexlean-portable-runtime-")
                    .tempdir()
                    .expect("runtime fixture tempdir");
                let path = directory.path().join("PortableRuntime.lean");
                std::fs::write(&path, format!("{lean}\n{evaluations}"))
                    .expect("write runtime evaluation module");
                let binary = support::real_elan_home()
                    .join("toolchains")
                    .join(support::mangled_toolchain_name())
                    .join("bin")
                    .join(if cfg!(windows) { "lean.exe" } else { "lean" });
                let output = Command::new(binary)
                    .arg(&path)
                    .current_dir(directory.path())
                    .env("LEAN_PATH", "")
                    .output()
                    .expect("pinned Lean evaluates portable runtime vectors");
                assert!(
                    output.status.success(),
                    "portable runtime evaluation failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                let stdout = String::from_utf8(output.stdout).expect("Lean output is UTF-8");
                let expected = evaluations
                    .lines()
                    .filter(|line| line.starts_with("#eval"))
                    .count();
                assert_eq!(stdout.lines().count(), expected);
                assert!(
                    stdout.lines().all(|line| line == "true"),
                    "portable runtime vector failed: {stdout}"
                );
            }
        }
        "SM-20" => {
            let range = support::semantic_project();
            range.edit(
                "src/Portable.lex.tex",
                "\"representation\":\"uint8\",\"value\":\"255\"",
                "\"representation\":\"uint8\",\"value\":\"256\"",
            );
            let error = range.check_err();
            assert!(error.to_string().contains("outside UInt8"));

            let bytes = support::semantic_project();
            bytes.edit(
                "src/Portable.lex.tex",
                "\"hex\":\"aabb7fff\"",
                "\"hex\":\"AAbb7fff\"",
            );
            let error = bytes.check_err();
            assert!(error.to_string().contains("lowercase hexadecimal"));

            let typing = support::semantic_project();
            typing.edit(
                "src/Portable.lex.tex",
                "\"operation\":\"checked_add\",\"result\":{\"kind\":\"option\",\"value\":{\"kind\":\"int64\"}}",
                "\"operation\":\"checked_add\",\"result\":{\"kind\":\"int64\"}",
            );
            let error = typing.check_err();
            assert!(error.to_string().contains("result is Int64"));
        }
        "SM-21" => {
            let project = support::semantic_project();
            project.check_ok();
            let bad = support::semantic_project();
            bad.edit(
                "src/PortableRecursion.lex.tex",
                "{\"kind\":\"var\",\"name\":\"tail\"}],\"function\":{\"name\":\"byteListLength\"}",
                "{\"kind\":\"var\",\"name\":\"bytes\"}],\"function\":{\"name\":\"byteListLength\"}",
            );
            let error = bad.check_err();
            assert!(error.to_string().contains("structurally smaller"));

            let missing_option_branch = support::semantic_project();
            missing_option_branch.edit(
                "src/PortableRecursion.lex.tex",
                ",{\"binders\":[\"payload\"],\"body\":{\"kind\":\"bool\",\"value\":true},\"constructor\":{\"name\":\"PortableOptionByte.some\"}}",
                "",
            );
            let error = missing_option_branch.check_err();
            assert!(error.to_string().contains("exhaustive"));

            let missing_result_branch = support::semantic_project();
            missing_result_branch.edit(
                "src/PortableRecursion.lex.tex",
                ",{\"binders\":[\"payload\"],\"body\":{\"kind\":\"bool\",\"value\":true},\"constructor\":{\"name\":\"PortableResultByte.ok\"}}",
                "",
            );
            let error = missing_result_branch.check_err();
            assert!(error.to_string().contains("exhaustive"));
        }
        "SM-22" => {
            use lexlean::{
                SnapshotInteger as I, SnapshotPrimitive as O, SnapshotTerm as Term,
                SnapshotType as T,
            };

            let integers = [
                I::Int,
                I::Int8,
                I::Int16,
                I::Int32,
                I::Int64,
                I::UInt8,
                I::UInt16,
                I::UInt32,
                I::UInt64,
            ];
            let primitives = [
                O::Subtract,
                O::Multiply,
                O::Quotient,
                O::Remainder,
                O::Negate,
                O::CheckedConvert,
                O::CheckedAdd,
                O::CheckedSubtract,
                O::CheckedMultiply,
                O::CheckedNegate,
                O::CheckedQuotient,
                O::BitAnd,
                O::BitOr,
                O::BitXor,
                O::BitNot,
                O::ShiftLeft,
                O::ShiftRight,
                O::Append,
                O::Length,
                O::Index,
                O::Slice,
                O::Utf8Encode,
                O::Utf8Decode,
                O::CompareBytes,
                O::Equal,
                O::SplitExact,
                O::Join,
                O::ParseDecimal,
                O::FormatDecimal,
            ];
            let types = vec![
                T::Int,
                T::Int8,
                T::Int16,
                T::Int32,
                T::Int64,
                T::UInt8,
                T::UInt16,
                T::UInt32,
                T::UInt64,
                T::String,
                T::Bytes,
                T::Ordering,
                T::Option {
                    value: Box::new(T::Int64),
                },
                T::Result {
                    ok: Box::new(T::Int64),
                    error: Box::new(T::String),
                },
            ];
            let terms = [
                Term::Integer {
                    representation: I::Int64,
                    value: "-1".to_owned(),
                },
                Term::String {
                    value: "portable".to_owned(),
                },
                Term::Bytes {
                    hex: "00ff".to_owned(),
                },
                Term::Primitive {
                    operation: O::CheckedAdd,
                    arguments: Vec::new(),
                    result: T::Option {
                        value: Box::new(T::Int64),
                    },
                },
            ];
            assert_eq!(integers.len(), 9, "downstream integer DTO variants");
            assert_eq!(primitives.len(), 29, "downstream primitive DTO variants");
            assert_eq!(types.len(), 14, "downstream portable type DTO variants");
            assert_eq!(terms.len(), 4, "downstream portable term DTO variants");

            let bounded = support::semantic_project();
            bounded.edit(
                "lexlean.toml",
                "max_ir_nodes = 2000000",
                "max_ir_nodes = 100",
            );
            bounded.relock();
            let error = bounded.check_fails_with("LLS8002");
            assert!(
                error
                    .diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic.message.contains("max_ir_nodes exceeded")),
                "portable semantic terms are recursively charged to max_ir_nodes"
            );

            let left = support::semantic_project();
            let right = support::semantic_project();
            let snapshot = |project: &P| {
                project
                    .engine()
                    .snapshot(lexlean::CheckRequest {
                        selection: lexlean::Selection::Entrypoints,
                    })
                    .expect("portable snapshot")
            };
            let first = snapshot(&left);
            let second = snapshot(&right);
            assert_eq!(first.canonical_bytes(), second.canonical_bytes());
            assert_eq!(first.snapshot_id(), second.snapshot_id());
            let value: serde_json::Value =
                serde_json::from_slice(&first.canonical_bytes()).expect("snapshot JSON");
            support::assert_schema("semantic-snapshot", "portable snapshot", &value);
            let bytes = String::from_utf8(first.canonical_bytes()).expect("utf8");
            assert!(!bytes.contains(left.root.as_str()));
            assert!(!bytes.contains("public def"));
            assert!(!bytes.contains("generated Rust"));
            assert!(bytes.contains("\"axioms\":[\"Quot.sound\",\"propext\"]"));
        }
        "SM-23" => {
            let snapshot_of = |project: &P| {
                let snapshot = project
                    .engine()
                    .snapshot(lexlean::CheckRequest {
                        selection: lexlean::Selection::Entrypoints,
                    })
                    .expect("snapshot");
                let value: serde_json::Value =
                    serde_json::from_slice(&snapshot.canonical_bytes()).expect("snapshot JSON");
                (snapshot.spec().to_owned(), value)
            };
            let no_backend = |project: &P| {
                // `.lexlean/.lock` is the §21.8 mutation lock, not output.
                for output in [".lexlean/build", ".lexlean/verified"] {
                    assert!(
                        !project.root.join(output).exists(),
                        "§17.12: a version failure occurs before any backend writes {output}"
                    );
                }
            };

            // Accepted under language 1.2: the let term is typed, snapshotted
            // under the versioned envelope, and lowered to a Lean let.
            let twelve = P::language_1_2_example();
            twelve.check_ok();
            let (spec, value) = snapshot_of(&twelve);
            assert_eq!(spec, "lexlean/semantic-snapshot/2");
            support::assert_schema("semantic-snapshot-v2", "the 1.2 snapshot", &value);
            support::assert_schema(
                "semantic-module-v2",
                "the 1.2 semantic module",
                &value["modules"][0]["semantic"],
            );
            let mut tags = BTreeSet::new();
            collect_tags(&value["modules"][0]["semantic"], "kind", &mut tags);
            assert!(tags.contains("let"), "the snapshot carries the let term");
            assert!(
                !crate::schema::validate(&support::schema("semantic-snapshot"), &value).is_empty(),
                "the 1.2 snapshot is not admitted by the 1.1 envelope schema"
            );
            let rendered = support::rendered(&twelve);
            let lean = support::lean_text(&rendered, "Main");
            assert!(
                lean.contains(
                    "public def doubledSuccessor (n : Nat) : Nat := (let doubled : Nat := (n + n); (doubled + 1))"
                ),
                "fixed Lean lowering of let:\n{lean}"
            );
            let _ = support::verify_ok_backed("SM-23", &twelve);

            // Rejected under language 1.1: the same construct, with the 1.1
            // discriminator and builtin versions, fails in linking.
            let eleven = P::language_1_2_example();
            eleven.edit("lexlean.toml", "language = \"1.2\"", "language = \"1.1\"");
            eleven.edit("src/Main.lex.tex", "@1.2.0", "@1.1.0");
            eleven.edit(
                "src/Main.lex.tex",
                "lexlean/semantic-module/2",
                "lexlean/semantic-module/1",
            );
            eleven.relock();
            let error = eleven.check_fails_with("LLT4001");
            assert!(
                error
                    .to_string()
                    .contains("`let` is a language-1.2 construct"),
                "the diagnostic names the construct: {error}"
            );
            no_backend(&eleven);
            let (exit, _, stderr) = eleven.cli(&["build"]);
            assert_eq!(exit, 1, "build refuses the 1.2 construct: {stderr}");
            no_backend(&eleven);

            // The discriminator is routed by the project language in both
            // directions: /1 under 1.2 and /2 under 1.1 are rejected.
            let v1_under_12 = P::language_1_2_example();
            v1_under_12.edit(
                "src/Main.lex.tex",
                "lexlean/semantic-module/2",
                "lexlean/semantic-module/1",
            );
            let error = v1_under_12.check_fails_with("LLT4001");
            assert!(error
                .to_string()
                .contains("requires `lexlean/semantic-module/2`"));
            let v2_under_11 = P::semantic_example();
            v2_under_11.edit(
                "src/Support.lex.tex",
                "lexlean/semantic-module/1",
                "lexlean/semantic-module/2",
            );
            let error = v2_under_11.check_fails_with("LLT4001");
            assert!(error
                .to_string()
                .contains("requires `lexlean/semantic-module/1`"));

            // The let binder is typed and lexically closed.
            let mistyped = P::language_1_2_example();
            mistyped.edit(
                "src/Main.lex.tex",
                r#""binder":{"name":"doubled","type":{"kind":"nat"}}"#,
                r#""binder":{"name":"doubled","type":{"kind":"bool"}}"#,
            );
            let error = mistyped.check_fails_with("LLT4001");
            assert!(
                error
                    .to_string()
                    .contains("let binder `doubled` has type Nat, expected Bool"),
                "{error}"
            );
            // A language-1.0 document has no semantic module at all.
            let ten = P::language_1_2_example();
            ten.edit("lexlean.toml", "language = \"1.2\"", "language = \"1.0\"");
            ten.edit("src/Main.lex.tex", "@1.2.0", "@1.0.0");
            ten.relock();
            let error = ten.check_fails_with("LLP2003");
            assert_eq!(
                error.to_string(),
                "LLP2003: semanticmodule requires language 1.1 or 1.2"
            );
            let shadowing = P::language_1_2_example();
            shadowing.edit(
                "src/Main.lex.tex",
                r#""binder":{"name":"doubled","#,
                r#""binder":{"name":"n","#,
            );
            let error = shadowing.check_fails_with("LLT4001");
            assert!(
                error.to_string().contains("shadowed let binder `n`"),
                "{error}"
            );
            let escaping = P::language_1_2_example();
            escaping.edit(
                "src/Main.lex.tex",
                r#""value":{"kind":"add","left":{"kind":"var","name":"n"}"#,
                r#""value":{"kind":"add","left":{"kind":"var","name":"doubled"}"#,
            );
            let error = escaping.check_fails_with("LLT4001");
            assert!(
                error.to_string().contains("unbound local `doubled`"),
                "{error}"
            );

            // Language 1.1 keeps its historical envelope byte for byte.
            let (spec, value) = snapshot_of(&P::semantic_example());
            assert_eq!(spec, "lexlean/semantic-snapshot/1");
            support::assert_schema("semantic-snapshot", "the 1.1 snapshot", &value);
        }
        "SM-24" => {
            let snapshot_of = |project: &P| {
                let snapshot = project
                    .engine()
                    .snapshot(lexlean::CheckRequest {
                        selection: lexlean::Selection::Entrypoints,
                    })
                    .expect("snapshot");
                snapshot.canonical_bytes()
            };
            // Two roots, one semantic identity and one snapshot.
            let first = P::copy_example("recursive-data");
            let second = P::copy_example("recursive-data");
            assert_ne!(first.root, second.root);
            let first_bytes = snapshot_of(&first);
            assert_eq!(
                first_bytes,
                snapshot_of(&second),
                "snapshots are root independent"
            );
            assert_eq!(
                support::checked_project(&first).semantic_id,
                support::checked_project(&second).semantic_id
            );
            let value: serde_json::Value =
                serde_json::from_slice(&first_bytes).expect("snapshot JSON");
            support::assert_schema(
                "semantic-snapshot-v2",
                "the recursive-data snapshot",
                &value,
            );
            let mut tags = BTreeSet::new();
            let mut mutual = 0;
            for module in value["modules"].as_array().expect("modules") {
                support::assert_schema(
                    "semantic-module-v2",
                    "a recursive-data module",
                    &module["semantic"],
                );
                collect_tags(&module["semantic"], "kind", &mut tags);
                mutual += module["semantic"]["declarations"]
                    .as_array()
                    .expect("declarations")
                    .iter()
                    .filter(|declaration| declaration["mutual"] == "Syntax")
                    .count();
            }
            assert_eq!(mutual, 2, "the snapshot records both mutual members");
            for tag in ["product", "pair", "first", "second", "inductive"] {
                assert!(tags.contains(tag), "snapshot carries `{tag}`");
            }
            let main = support::lean_text(&support::rendered(&first), "Main");
            for expected in [
                "public def swap (pair : (Prod (Nat) (Bool))) : (Prod (Bool) (Nat)) := ((pair).2, (pair).1)",
                "(match pair with | Prod.mk left right => (left + right))",
            ] {
                assert!(main.contains(expected), "missing {expected:?} in:\n{main}");
            }

            // Projections and product matches are typed.
            let source = first.read("src/Main.lex.tex");
            let mutate = |from: &str, to: &str, message: &str| {
                let copy = P::copy_example("recursive-data");
                assert!(source.contains(from), "fixture lacks {from:?}");
                copy.write("src/Main.lex.tex", &source.replacen(from, to, 1));
                let error = copy.check_fails_with("LLT4001");
                assert!(
                    error.to_string().contains(message),
                    "expected {message:?}, got {error}"
                );
                copy.assert_no_backend_output();
            };
            mutate(
                r#"{"kind":"second","value":{"kind":"var","name":"pair"}}"#,
                r#"{"kind":"second","value":{"kind":"nat","value":"1"}}"#,
                "pair projection of non-product type",
            );
            mutate(
                r#"{"binders":["left","right"],"body":{"kind":"add","left":{"kind":"var","name":"left"},"right":{"kind":"var","name":"right"}},"constructor":{"name":"Prod.mk"}}"#,
                r#"{"binders":["left"],"body":{"kind":"var","name":"left"},"constructor":{"name":"Prod.mk"}}"#,
                "match branch `Prod.mk` expects 2 binder(s), received 1",
            );

            // Language 1.1 rejects the product type and the pair term.
            let eleven = P::semantic_example();
            let support_source = eleven.read("src/Support.lex.tex");
            eleven.write(
                "src/Support.lex.tex",
                &support_source.replacen(
                    r#""body":{"kind":"bool","value":true},"kind":"definition","name":"remoteEnabled","parameters":[],"result":{"kind":"bool"}"#,
                    r#""body":{"kind":"pair","left":{"kind":"bool","value":true},"right":{"kind":"unit"}},"kind":"definition","name":"remoteEnabled","parameters":[],"result":{"kind":"product","left":{"kind":"bool"},"right":{"kind":"unit"}}"#,
                    1,
                ),
            );
            let error = eleven.check_fails_with("LLT4001");
            assert!(
                error
                    .to_string()
                    .contains("`product type` is a language-1.2 construct"),
                "{error}"
            );
            // Each 1.2 term is refused on its own, with no product type in
            // sight to be reported first.
            for (term, construct) in [
                (
                    r#"{"kind":"pair","left":{"kind":"bool","value":true},"right":{"kind":"unit"}}"#,
                    "pair",
                ),
                (
                    r#"{"kind":"first","value":{"kind":"bool","value":true}}"#,
                    "first",
                ),
                (
                    r#"{"kind":"second","value":{"kind":"bool","value":true}}"#,
                    "second",
                ),
            ] {
                let eleven = P::semantic_example();
                eleven.write(
                    "src/Support.lex.tex",
                    &support_source.replacen(
                        r#""body":{"kind":"bool","value":true},"kind":"definition","name":"remoteEnabled""#,
                        &format!(r#""body":{term},"kind":"definition","name":"remoteEnabled""#),
                        1,
                    ),
                );
                let error = eleven.check_fails_with("LLT4001");
                assert!(
                    error
                        .to_string()
                        .contains(&format!("`{construct}` is a language-1.2 construct")),
                    "{error}"
                );
            }
        }
        "SM-25" => {
            let project = P::copy_example("higher-order");
            project.check_ok();
            let rendered = support::rendered(&project);
            let main = support::lean_text(&rendered, "Main");
            for expected in [
                "HigherOrder.Combinators.mapList (Nat) (Nat) ((fun (value : Nat) => (value + offset))) (values)",
                "HigherOrder.Combinators.mapList (Nat) (Nat) ((HigherOrder.Combinators.increment)) (values)",
                "public def addThenDouble : ((Nat) -> (Nat)) :=",
                "(addThenDouble (4)) = 10",
            ] {
                assert!(main.contains(expected), "missing {expected:?} in:\n{main}");
            }
            let combinators = support::lean_text(&rendered, "Combinators");
            assert!(combinators.contains("(fun (value : Input) => (outer ((inner (value)))))"));
            let _ = support::verify_ok_backed("SM-25", &project);

            let mutate = |file: &str, from: &str, to: &str, message: &str| {
                let copy = P::copy_example("higher-order");
                let source = copy.read(file);
                assert!(source.contains(from), "fixture lacks {from:?}");
                copy.write(file, &source.replacen(from, to, 1));
                let error = copy.check_fails_with("LLT4001");
                assert!(
                    error.to_string().contains(message),
                    "expected {message:?}, got {error}"
                );
                copy.assert_no_backend_output();
            };
            // Capture is explicit and exact.
            mutate(
                "src/Main.lex.tex",
                r#""captures":["offset"]"#,
                r#""captures":[]"#,
                "declared (), used (offset)",
            );
            mutate(
                "src/Combinators.lex.tex",
                r#""captures":["inner","outer"]"#,
                r#""captures":["inner","outer","value"]"#,
                "lambda capture `value` is not an enclosing local",
            );
            mutate(
                "src/Combinators.lex.tex",
                r#""captures":["inner","outer"]"#,
                r#""captures":["outer","inner"]"#,
                "lambda captures are strictly sorted and unique",
            );
            // A lambda parameter never shadows a local.
            mutate(
                "src/Main.lex.tex",
                r#""captures":["offset"],"kind":"lambda","parameters":[{"name":"value""#,
                r#""captures":["offset"],"kind":"lambda","parameters":[{"name":"offset""#,
                "shadowed lambda parameter `offset`",
            );
            // Applications are full and typed.
            mutate(
                "src/Main.lex.tex",
                r#""arguments":[{"kind":"nat","value":"4"}]"#,
                r#""arguments":[{"kind":"nat","value":"4"},{"kind":"nat","value":"5"}]"#,
                "application expects 1 argument(s), received 2",
            );
            mutate(
                "src/Combinators.lex.tex",
                r#""function":{"kind":"var","name":"transform"}"#,
                r#""function":{"kind":"var","name":"head"}"#,
                "application of a non-function value",
            );
            // A reference names a function, not a constant.
            mutate(
                "src/Main.lex.tex",
                r#"{"arguments":[],"function":{"name":"addThenDouble"},"kind":"call"}"#,
                r#"{"function":{"name":"evaluator"},"kind":"function_ref"}"#,
                "`evaluator` has no parameters and is not a function value",
            );

            // Language 1.1 rejects a lambda.
            let eleven = P::semantic_example();
            let support_source = eleven.read("src/Support.lex.tex");
            eleven.write(
                "src/Support.lex.tex",
                &support_source.replacen(
                    r#""body":{"kind":"bool","value":true},"kind":"definition","name":"remoteEnabled""#,
                    r#""body":{"arguments":[{"kind":"bool","value":true}],"function":{"body":{"kind":"var","name":"flag"},"captures":[],"kind":"lambda","parameters":[{"name":"flag","type":{"kind":"bool"}}]},"kind":"apply"},"kind":"definition","name":"remoteEnabled""#,
                    1,
                ),
            );
            let error = eleven.check_fails_with("LLT4001");
            assert!(
                error.to_string().contains("is a language-1.2 construct"),
                "{error}"
            );
        }
        "SM-26" => {
            let snapshot_of = |project: &P| {
                project
                    .engine()
                    .snapshot(lexlean::CheckRequest {
                        selection: lexlean::Selection::Entrypoints,
                    })
                    .expect("snapshot")
            };
            let alpha = |project: &P, name: &str| {
                snapshot_of(project)
                    .alpha_ids()
                    .into_iter()
                    .find(|(_, declaration, _)| declaration == name)
                    .map(|(_, _, digest)| digest)
                    .unwrap_or_else(|| panic!("alpha identity of `{name}`"))
            };
            let original = P::copy_example("higher-order");
            let other_root = P::copy_example("higher-order");
            let first = snapshot_of(&original);
            assert_eq!(
                first.canonical_bytes(),
                snapshot_of(&other_root).canonical_bytes()
            );
            let value: serde_json::Value =
                serde_json::from_slice(&first.canonical_bytes()).expect("snapshot JSON");
            support::assert_schema("semantic-snapshot-v2", "the higher-order snapshot", &value);
            let definitions = first.alpha_ids().len();
            assert_eq!(
                definitions, 11,
                "every definition carries an alpha identity"
            );

            // Renaming the lambda binder and the parameter keeps the alpha
            // identity and changes the semantic identity.
            let renamed = P::copy_example("higher-order");
            let source = renamed.read("src/Main.lex.tex");
            let renamed_source = source
                .replacen(
                    r#"{"body":{"kind":"add","left":{"kind":"var","name":"value"},"right":{"kind":"var","name":"offset"}},"captures":["offset"],"kind":"lambda","parameters":[{"name":"value","type":{"kind":"nat"}}]}"#,
                    r#"{"body":{"kind":"add","left":{"kind":"var","name":"item"},"right":{"kind":"var","name":"shift"}},"captures":["shift"],"kind":"lambda","parameters":[{"name":"item","type":{"kind":"nat"}}]}"#,
                    1,
                )
                .replacen(
                    r#""name":"addAll","parameters":[{"name":"offset","type":{"kind":"nat"}}"#,
                    r#""name":"addAll","parameters":[{"name":"shift","type":{"kind":"nat"}}"#,
                    1,
                );
            assert_ne!(renamed_source, source, "the renaming applies");
            renamed.write("src/Main.lex.tex", &renamed_source);
            renamed.check_ok();
            assert_eq!(alpha(&renamed, "addAll"), alpha(&original, "addAll"));
            assert_ne!(
                support::checked_project(&renamed).semantic_id,
                support::checked_project(&original).semantic_id,
                "binder names remain part of the semantic identity"
            );
            // Swapping the operands is not an alpha renaming.
            let changed = P::copy_example("higher-order");
            changed.write(
                "src/Main.lex.tex",
                &source.replacen(
                    r#"{"kind":"add","left":{"kind":"var","name":"value"},"right":{"kind":"var","name":"offset"}}"#,
                    r#"{"kind":"add","left":{"kind":"var","name":"offset"},"right":{"kind":"var","name":"value"}}"#,
                    1,
                ),
            );
            changed.check_ok();
            assert_ne!(alpha(&changed, "addAll"), alpha(&original, "addAll"));
            // Every binder kind renames positionally: type parameters, value
            // parameters in a different order (so the sorted captures
            // renumber out of source order), `let`, and match binders.
            let identity = |json: &str| {
                serde_json::from_str::<lexlean::SnapshotSemanticDeclaration>(json)
                    .expect("declaration JSON")
                    .alpha_identity()
                    .expect("a definition has an alpha identity")
            };
            for (left, right) in [
                (
                    r#"{"kind":"definition","name":"pick","type_parameters":["Item"],"parameters":[{"name":"value","type":{"kind":"parameter","name":"Item"}}],"result":{"kind":"parameter","name":"Item"},"body":{"kind":"var","name":"value"}}"#,
                    r#"{"kind":"definition","name":"pick","type_parameters":["Source"],"parameters":[{"name":"value","type":{"kind":"parameter","name":"Source"}}],"result":{"kind":"parameter","name":"Source"},"body":{"kind":"var","name":"value"}}"#,
                ),
                (
                    r#"{"kind":"definition","name":"sum","parameters":[{"name":"a","type":{"kind":"nat"}},{"name":"b","type":{"kind":"nat"}}],"result":{"kind":"nat"},"body":{"kind":"apply","function":{"kind":"lambda","parameters":[{"name":"x","type":{"kind":"nat"}}],"captures":["a","b"],"body":{"kind":"add","left":{"kind":"var","name":"a"},"right":{"kind":"var","name":"b"}}},"arguments":[{"kind":"nat","value":"0"}]}}"#,
                    r#"{"kind":"definition","name":"sum","parameters":[{"name":"b","type":{"kind":"nat"}},{"name":"a","type":{"kind":"nat"}}],"result":{"kind":"nat"},"body":{"kind":"apply","function":{"kind":"lambda","parameters":[{"name":"x","type":{"kind":"nat"}}],"captures":["a","b"],"body":{"kind":"add","left":{"kind":"var","name":"b"},"right":{"kind":"var","name":"a"}}},"arguments":[{"kind":"nat","value":"0"}]}}"#,
                ),
                (
                    r#"{"kind":"definition","name":"twice","parameters":[{"name":"value","type":{"kind":"nat"}}],"result":{"kind":"nat"},"body":{"kind":"let","binder":{"name":"half","type":{"kind":"nat"}},"value":{"kind":"var","name":"value"},"body":{"kind":"add","left":{"kind":"var","name":"half"},"right":{"kind":"var","name":"half"}}}}"#,
                    r#"{"kind":"definition","name":"twice","parameters":[{"name":"value","type":{"kind":"nat"}}],"result":{"kind":"nat"},"body":{"kind":"let","binder":{"name":"part","type":{"kind":"nat"}},"value":{"kind":"var","name":"value"},"body":{"kind":"add","left":{"kind":"var","name":"part"},"right":{"kind":"var","name":"part"}}}}"#,
                ),
                (
                    r#"{"kind":"definition","name":"head","parameters":[{"name":"values","type":{"kind":"list","element":{"kind":"nat"}}}],"result":{"kind":"nat"},"body":{"kind":"match","scrutinee":{"kind":"var","name":"values"},"branches":[{"constructor":{"name":"List.nil"},"binders":[],"body":{"kind":"nat","value":"0"}},{"constructor":{"name":"List.cons"},"binders":["first","rest"],"body":{"kind":"var","name":"first"}}]}}"#,
                    r#"{"kind":"definition","name":"head","parameters":[{"name":"values","type":{"kind":"list","element":{"kind":"nat"}}}],"result":{"kind":"nat"},"body":{"kind":"match","scrutinee":{"kind":"var","name":"values"},"branches":[{"constructor":{"name":"List.nil"},"binders":[],"body":{"kind":"nat","value":"0"}},{"constructor":{"name":"List.cons"},"binders":["item","others"],"body":{"kind":"var","name":"item"}}]}}"#,
                ),
            ] {
                assert_eq!(identity(left), identity(right), "{left}\n{right}");
            }
            // A renaming that changes which binder is used is not one.
            assert_ne!(
                identity(
                    r#"{"kind":"definition","name":"head","parameters":[{"name":"values","type":{"kind":"list","element":{"kind":"nat"}}}],"result":{"kind":"nat"},"body":{"kind":"match","scrutinee":{"kind":"var","name":"values"},"branches":[{"constructor":{"name":"List.nil"},"binders":[],"body":{"kind":"nat","value":"0"}},{"constructor":{"name":"List.cons"},"binders":["first","rest"],"body":{"kind":"var","name":"first"}}]}}"#
                ),
                identity(
                    r#"{"kind":"definition","name":"head","parameters":[{"name":"values","type":{"kind":"list","element":{"kind":"nat"}}}],"result":{"kind":"nat"},"body":{"kind":"match","scrutinee":{"kind":"var","name":"values"},"branches":[{"constructor":{"name":"List.nil"},"binders":[],"body":{"kind":"nat","value":"0"}},{"constructor":{"name":"List.cons"},"binders":["first","rest"],"body":{"kind":"var","name":"values"}}]}}"#
                ),
            );
            assert!(snapshot_of(&P::semantic_example()).alpha_ids().is_empty());
        }
        "SM-27" => {
            let snapshot_of = |project: &P| {
                project
                    .engine()
                    .snapshot(lexlean::CheckRequest {
                        selection: lexlean::Selection::Entrypoints,
                    })
                    .expect("snapshot")
            };
            let original = P::copy_example("recursion");
            let first = snapshot_of(&original);
            assert_eq!(
                first.canonical_bytes(),
                snapshot_of(&P::copy_example("recursion")).canonical_bytes()
            );
            let value: serde_json::Value =
                serde_json::from_slice(&first.canonical_bytes()).expect("snapshot JSON");
            support::assert_schema("semantic-snapshot-v2", "the recursion snapshot", &value);
            let main = value["modules"]
                .as_array()
                .expect("modules")
                .iter()
                .find(|module| module["name"] == "Main")
                .expect("Main module");
            let definitions = main["semantic"]["declarations"]
                .as_array()
                .expect("declarations");
            let countdown = definitions
                .iter()
                .find(|declaration| declaration["name"] == "countdown")
                .expect("countdown");
            assert_eq!(countdown["termination"]["measure"]["name"], "number");
            assert_eq!(
                countdown["termination"]["evidence"][0]["name"],
                "countdown_decreases"
            );
            let mutual = definitions
                .iter()
                .filter(|declaration| declaration["mutual"] == "SyntaxSize")
                .count();
            assert_eq!(mutual, 3, "the snapshot records every group member");

            // Changing only the evidence binding changes both identities.
            let alpha = |snapshot: &lexlean::SemanticSnapshot, name: &str| {
                snapshot
                    .alpha_ids()
                    .into_iter()
                    .find(|(_, declaration, _)| declaration == name)
                    .map(|(_, _, digest)| digest)
                    .expect("alpha identity")
            };
            // Both copies declare the same extra theorem; only one binds it
            // as the evidence, so every difference is the binding's.
            let theorem_anchor = r#"{"axioms":["Quot.sound","propext"],"kind":"theorem","name":"countdown_decreases""#;
            let added_theorem = r#"{"axioms":["Quot.sound","propext"],"kind":"theorem","name":"countdown_shrinks","parameters":[{"name":"number","type":{"kind":"nat"}},{"name":"steps","type":{"kind":"nat"}}],"proof":{"kind":"linear_arithmetic"},"statement":{"conclusion":{"kind":"lt","left":{"arguments":[{"kind":"var","name":"number"},{"kind":"nat","value":"2"}],"kind":"primitive","operation":"subtract","result":{"kind":"nat"}},"right":{"kind":"var","name":"number"}},"kind":"implies","premise":{"kind":"eq","left":{"kind":"blt","left":{"kind":"var","name":"number"},"right":{"kind":"nat","value":"2"}},"right":{"kind":"bool","value":false}}}},"#;
            let source = original.read("src/Main.lex.tex");
            assert!(source.contains(theorem_anchor));
            let with_theorem = source.replacen(
                theorem_anchor,
                &format!("{added_theorem}{theorem_anchor}"),
                1,
            );
            let added = P::copy_example("recursion");
            added.write("src/Main.lex.tex", &with_theorem);
            let rebound = P::copy_example("recursion");
            let binding = r#""termination":{"evidence":[{"name":"countdown_decreases"}]"#;
            assert!(with_theorem.contains(binding));
            rebound.write(
                "src/Main.lex.tex",
                &with_theorem.replacen(
                    binding,
                    r#""termination":{"evidence":[{"name":"countdown_shrinks"}]"#,
                    1,
                ),
            );
            let unbound = snapshot_of(&added);
            let second = snapshot_of(&rebound);
            assert_eq!(
                alpha(&first, "countdown"),
                alpha(&unbound, "countdown"),
                "an unused theorem does not touch the definition's identity"
            );
            assert_ne!(alpha(&unbound, "countdown"), alpha(&second, "countdown"));
            assert_ne!(
                support::checked_project(&added).semantic_id,
                support::checked_project(&rebound).semantic_id
            );
            assert_eq!(alpha(&unbound, "reduce"), alpha(&second, "reduce"));

            // The measure and the mutual label are part of both identities:
            // `countdown` measured by `number + 0`, with its evidence restated
            // for that measure, and the `Bounce` group relabelled.
            let measured = P::copy_example("recursion");
            let measure_source = source
                .replacen(
                    r#""termination":{"evidence":[{"name":"countdown_decreases"}],"measure":{"kind":"var","name":"number"}}"#,
                    r#""termination":{"evidence":[{"name":"countdown_decreases"}],"measure":{"kind":"add","left":{"kind":"var","name":"number"},"right":{"kind":"nat","value":"0"}}}"#,
                    1,
                )
                .replacen(
                    r#""statement":{"conclusion":{"kind":"lt","left":{"arguments":[{"kind":"var","name":"number"},{"kind":"nat","value":"2"}],"kind":"primitive","operation":"subtract","result":{"kind":"nat"}},"right":{"kind":"var","name":"number"}}"#,
                    r#""statement":{"conclusion":{"kind":"lt","left":{"kind":"add","left":{"arguments":[{"kind":"var","name":"number"},{"kind":"nat","value":"2"}],"kind":"primitive","operation":"subtract","result":{"kind":"nat"}},"right":{"kind":"nat","value":"0"}},"right":{"kind":"add","left":{"kind":"var","name":"number"},"right":{"kind":"nat","value":"0"}}}"#,
                    1,
                );
            assert_ne!(measure_source, source, "the measure mutation applies");
            measured.write("src/Main.lex.tex", &measure_source);
            let remeasured = snapshot_of(&measured);
            assert_ne!(alpha(&first, "countdown"), alpha(&remeasured, "countdown"));
            assert_ne!(
                support::checked_project(&original).semantic_id,
                support::checked_project(&measured).semantic_id
            );
            let relabelled = P::copy_example("recursion");
            let label_source = source.replace(r#""mutual":"Bounce""#, r#""mutual":"Rebound""#);
            assert_eq!(label_source.matches(r#""mutual":"Rebound""#).count(), 2);
            relabelled.write("src/Main.lex.tex", &label_source);
            let renamed = snapshot_of(&relabelled);
            assert_ne!(alpha(&first, "ping"), alpha(&renamed, "ping"));
            assert_ne!(
                support::checked_project(&original).semantic_id,
                support::checked_project(&relabelled).semantic_id
            );
        }
        "SM-28" => {
            let project = P::copy_example("collections");
            project.check_ok();
            let rendered = support::rendered(&project);
            let tables = support::lean_text(&rendered, "Tables");
            for expected in [
                "public def declare (table : List (Prod (String) (Nat))) (name : String) (slot : Nat) : List (Prod (String) (Nat)) := (LexLeanCollections.mapInsert (table) (name) (slot) : List (Prod (String) (Nat)))",
                "namespace LexLeanCollections",
                "public class Key (α : Type) where",
            ] {
                assert!(tables.contains(expected), "missing {expected:?} in:\n{tables}");
            }
            // A module that only measures an imported table writes no
            // collection type or literal, yet calls the runtime: it emits its
            // own copy, and pinned Lean verifies it.
            let measure = support::lean_text(&rendered, "Measure");
            for expected in [
                "namespace LexLeanCollections",
                "(LexLeanCollections.mapSize (Collections.Tables.buildTable (",
            ] {
                assert!(
                    measure.contains(expected),
                    "missing {expected:?} in:\n{measure}"
                );
            }
            // Every operation agrees with an independent ordered-map model
            // on seeded sequences, under Lean (§17.12 item 2).
            let model = [
                super::collections_model::map_theorems(),
                super::collections_model::set_theorems(),
            ]
            .concat();
            assert_eq!(model.len(), 86, "every seeded case states its theorems");
            super::collections_model::with_theorems(&project, model);
            let main = support::lean_text(&support::rendered(&project), "Main");
            for expected in [
                "theorem model_map_15 ",
                "theorem model_set_difference_11 ",
                "theorem model_string_set_5 ",
            ] {
                assert!(main.contains(expected), "missing {expected:?}");
            }
            let _ = support::verify_ok_backed("SM-28", &project);
            let reject = |file: &str, from: &str, to: &str, message: &str| {
                P::assert_mutation_rejected("collections", file, from, to, message);
            };
            // Operations are typed against their collection.
            reject(
                "src/Tables.lex.tex",
                r#""operation":"map_insert","result":{"key":{"kind":"string"},"kind":"map","value":{"kind":"nat"}}"#,
                r#""operation":"map_insert","result":{"key":{"kind":"string"},"kind":"map","value":{"kind":"bool"}}"#,
                "primitive MapInsert result",
            );
            reject(
                "src/Tables.lex.tex",
                r#"{"arguments":[{"kind":"var","name":"table"}],"kind":"primitive","operation":"map_size""#,
                r#"{"arguments":[{"kind":"var","name":"name"}],"kind":"primitive","operation":"map_size""#,
                "primitive MapSize requires a map",
            );
            // Language 1.1 has no collections.
            let eleven = P::semantic_example();
            let support_source = eleven.read("src/Support.lex.tex");
            eleven.write(
                "src/Support.lex.tex",
                &support_source.replacen(
                    r#""body":{"kind":"bool","value":true},"kind":"definition","name":"remoteEnabled","parameters":[],"result":{"kind":"bool"}"#,
                    r#""body":{"element":{"kind":"nat"},"elements":[],"kind":"set_literal"},"kind":"definition","name":"remoteEnabled","parameters":[],"result":{"element":{"kind":"nat"},"kind":"set"}"#,
                    1,
                ),
            );
            let error = eleven.check_fails_with("LLT4001");
            assert!(
                error.to_string().contains("is a language-1.2 construct"),
                "{error}"
            );
        }
        "SM-29" => {
            // Reordered equivalent literals denote byte-identical linked data
            // and generated artifacts.
            let original = P::copy_example("collections");
            let reordered = P::copy_example("collections");
            let source = reordered.read("src/Tables.lex.tex");
            let shuffled = source
                .replacen(
                    r#"{"kind":"string","value":"main"},{"kind":"string","value":"parse"}"#,
                    r#"{"kind":"string","value":"parse"},{"kind":"string","value":"main"}"#,
                    1,
                )
                .replacen(
                    r#"{"source":{"kind":"string","value":"main"},"target":{"kind":"string","value":"parse"}},{"source":{"kind":"string","value":"main"},"target":{"kind":"string","value":"emit"}}"#,
                    r#"{"source":{"kind":"string","value":"main"},"target":{"kind":"string","value":"emit"}},{"source":{"kind":"string","value":"main"},"target":{"kind":"string","value":"parse"}}"#,
                    1,
                );
            assert_ne!(shuffled, source, "the reordering applies");
            reordered.write("src/Tables.lex.tex", &shuffled);
            // Map and set literals reorder the same way.
            let main_source = reordered.read("src/Main.lex.tex");
            let main_shuffled = main_source
                .replacen(
                    r#"{"key":{"kind":"string","value":"b"},"value":{"kind":"nat","value":"2"}},{"key":{"kind":"string","value":"a"},"value":{"kind":"nat","value":"1"}}"#,
                    r#"{"key":{"kind":"string","value":"a"},"value":{"kind":"nat","value":"1"}},{"key":{"kind":"string","value":"b"},"value":{"kind":"nat","value":"2"}}"#,
                    1,
                )
                .replacen(
                    r#"{"kind":"nat","value":"3"},{"kind":"nat","value":"1"},{"kind":"nat","value":"2"}"#,
                    r#"{"kind":"nat","value":"2"},{"kind":"nat","value":"3"},{"kind":"nat","value":"1"}"#,
                    1,
                );
            assert_ne!(
                main_shuffled, main_source,
                "the map and set reordering applies"
            );
            reordered.write("src/Main.lex.tex", &main_shuffled);
            let first = support::checked_project(&original);
            let second = support::checked_project(&reordered);
            assert_ne!(first.source_id, second.source_id, "the sources differ");
            assert_eq!(
                first.semantic_id, second.semantic_id,
                "the semantics do not"
            );
            // The snapshot differs only in the members that record the
            // source: its source identity and each module's source record.
            let without_source = |project: &P| {
                let snapshot = project
                    .engine()
                    .snapshot(lexlean::CheckRequest {
                        selection: lexlean::Selection::Entrypoints,
                    })
                    .expect("snapshot");
                let mut value: serde_json::Value =
                    serde_json::from_slice(&snapshot.canonical_bytes()).expect("snapshot JSON");
                let object = value.as_object_mut().expect("snapshot object");
                assert!(object.remove("source_id").is_some(), "source identity");
                for module in object
                    .get_mut("modules")
                    .and_then(serde_json::Value::as_array_mut)
                    .expect("snapshot modules")
                {
                    let module = module.as_object_mut().expect("snapshot module");
                    assert!(module.remove("source").is_some(), "module source");
                }
                value
            };
            assert_eq!(without_source(&original), without_source(&reordered));
            // Every published artifact except the manifest, source maps, and
            // coverage, which record source positions, is byte-identical.
            let artifacts = |project: &P| {
                let built = project.build_ok();
                let directory = project.build_dir(&built.build_id.expect("build id"));
                support::file_set(&directory)
                    .into_iter()
                    .map(|path| {
                        let bytes =
                            std::fs::read(directory.join(&path).as_std_path()).expect("artifact");
                        (path, bytes)
                    })
                    .collect::<std::collections::BTreeMap<_, _>>()
            };
            let (left, right) = (artifacts(&original), artifacts(&reordered));
            assert_eq!(
                left.keys().collect::<Vec<_>>(),
                right.keys().collect::<Vec<_>>(),
                "the same artifacts are published"
            );
            let mut compared = 0;
            for (path, bytes) in &left {
                if path == "manifest.json"
                    || path.starts_with("maps/")
                    || path.starts_with("coverage/")
                {
                    continue;
                }
                assert!(bytes == &right[path], "{path} differs");
                compared += 1;
            }
            assert_eq!(
                compared, 9,
                "three modules' Lean, LaTeX, and lexicon closures"
            );
            let reject = |from: &str, to: &str, message: &str| {
                P::assert_mutation_rejected("collections", "src/Main.lex.tex", from, to, message);
            };
            // A duplicate key, a non-literal key, and an unordered key type.
            reject(
                r#"{"key":{"kind":"nat","value":"2"},"value":{"kind":"bool","value":false}}"#,
                r#"{"key":{"kind":"nat","value":"1"},"value":{"kind":"bool","value":false}}"#,
                "duplicate map key in a map literal",
            );
            reject(
                r#"{"key":{"kind":"nat","value":"2"},"value":{"kind":"bool","value":false}}"#,
                r#"{"key":{"arguments":[],"function":{"module":"Tables","name":"callOrder"},"kind":"call"},"value":{"kind":"bool","value":false}}"#,
                "must be a literal value",
            );
            reject(
                r#""key":{"kind":"nat"},"kind":"map_literal","value":{"kind":"bool"}"#,
                r#""key":{"kind":"unit"},"kind":"map_literal","value":{"kind":"bool"}"#,
                "has no canonical order",
            );
            // A literal key is checked as a term before it is ordered: a
            // noncanonical spelling or an out-of-range fixed-width value has
            // no position in the canonical order.
            reject(
                r#"{"key":{"kind":"nat","value":"2"},"value":{"kind":"bool","value":false}}"#,
                r#"{"key":{"kind":"nat","value":"02"},"value":{"kind":"bool","value":false}}"#,
                "noncanonical natural literal `02`",
            );
            reject(
                r#"{"kind":"integer","representation":"uint8","value":"255"}"#,
                r#"{"kind":"integer","representation":"uint8","value":"256"}"#,
                "integer literal `256` is outside UInt8 [0, 255]",
            );
        }
        "SM-30" => {
            let project = P::copy_example("collections");
            project.check_ok();
            let tables = support::lean_text(&support::rendered(&project), "Tables");
            assert!(tables.contains(
                r#"public def callGraph : List (Prod (String) (List (String))) := ([("emit", ["report"]), ("lex", []), ("main", ["emit", "parse"]), ("parse", ["lex"]), ("report", ["lex"])] : List (Prod (String) (List (String))))"#
            ), "{tables}");
            // Seeded graphs, including successors without entries of their
            // own and a chain as deep as its node count allows, agree with a
            // direct breadth-first search and Kahn's algorithm under Lean.
            super::collections_model::with_theorems(
                &project,
                super::collections_model::graph_theorems(),
            );
            let main = support::lean_text(&support::rendered(&project), "Main");
            for expected in [
                "theorem model_graph_chain_reachable ",
                "theorem model_graph_chain_topological ",
            ] {
                assert!(main.contains(expected), "missing {expected:?}");
            }
            let _ = support::verify_ok_backed("SM-30", &project);
            let reject = |from: &str, to: &str, message: &str| {
                P::assert_mutation_rejected("collections", "src/Tables.lex.tex", from, to, message);
            };
            reject(
                r#""target":{"kind":"string","value":"report"}"#,
                r#""target":{"kind":"string","value":"render"}"#,
                "graph edge target is not a declared node",
            );
            reject(
                r#"{"source":{"kind":"string","value":"main"},"target":{"kind":"string","value":"emit"}}"#,
                r#"{"source":{"kind":"string","value":"main"},"target":{"kind":"string","value":"parse"}}"#,
                "duplicate edge in a graph literal",
            );
            reject(
                r#"{"kind":"string","value":"lex"},{"kind":"string","value":"emit"}"#,
                r#"{"kind":"string","value":"lex"},{"kind":"string","value":"lex"}"#,
                "duplicate node in a graph literal",
            );
            reject(
                r#""operation":"graph_reachable","result":{"element":{"kind":"string"},"kind":"set"}"#,
                r#""operation":"graph_reachable","result":{"element":{"kind":"nat"},"kind":"set"}"#,
                "primitive GraphReachable result",
            );
        }
        other => panic!("no semantic-ir case is wired for {other}"),
    }
}

/// The math-channel zero used by the arity fixture.
const Z_MATH: &str = r#"spec = "lexlean/entry/1"
id = "z"
category = "term-constant"
signature = "(const lexlean.std.nat::nat)"
surface_arity = 0
frame = "atom"

[denotation]
kind = "lean"
module = "Init"
name = "Nat.zero"

[[form]]
id = "z"
channel = "both"
surface = "z"
canonical_source = true
features = []

[render]
math = "(operator-name z)"
"#;
