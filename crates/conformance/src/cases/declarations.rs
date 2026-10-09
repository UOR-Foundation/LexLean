//! The `declarations` suite: DF-01..DF-18.

use lexlean::ir::declaration::{DeclBody, DeclKind};

use crate::support::{self, P};

pub(crate) fn run(id: &str) {
    match id {
        // §15.7 rule 9: one nonrecursive sort-valued def, entry-linked; an
        // ambiguous type phrase is rejected, and formatting retains the
        // qualified selector that disambiguates (C7, D2).
        "DF-01" => {
            let project = support::defs_project();
            project.check_ok();
            let lean = support::lean_text(&support::rendered(&project), "Main");
            assert!(
                lean.contains("@[expose] public def count : Type :=\n  Nat\n"),
                "a type definition is a sort-valued def: {lean}"
            );
            // The right-hand side is a sort read through defined type nouns
            // (§13.6): a constant whose type is the defined noun `type`.
            support::f1_exact(
                "DF-01",
                "alias",
                "@[expose] public def alias : Type :=\n  Nat",
            );
            let ambiguous = support::defs_project();
            ambiguous.add_package(
                "lexicons/test-dupnat",
                "test.dupnat",
                &["lexlean.core@1.0.0"],
                &[("nat2.toml", DUP_NAT_ENTRY)],
            );
            ambiguous.edit(
                "src/Main.lex.tex",
                "\\useglossary{test.defs@1.0.0}",
                "\\useglossary{test.defs@1.0.0}\n\\useglossary{test.dupnat@1.0.0}",
            );
            ambiguous.edit(
                "src/Main.lex.tex",
                "A count is defined as \\(ℕ\\).",
                "A count is defined as natural number.",
            );
            ambiguous.relock();
            let error = ambiguous.check_fails_with("LLP2002");
            let diagnostic = error
                .diagnostics
                .iter()
                .find(|d| d.code.as_str() == "LLP2002")
                .expect("matched");
            assert!(
                diagnostic.message.contains("lexlean.std.nat::nat")
                    && diagnostic.message.contains("test.dupnat::nat2"),
                "the definition ambiguity names both candidates: {}",
                diagnostic.message
            );
            // With explicit selectors the module checks, and canonical
            // formatting keeps every selector the bare surface would not
            // resolve uniquely.
            ambiguous.write(
                "src/Main.lex.tex",
                &support::DEFS_MODULE
                    .replace(
                        "\\useglossary{test.defs@1.0.0}",
                        "\\useglossary{test.defs@1.0.0}\n\\useglossary{test.dupnat@1.0.0}",
                    )
                    .replace(
                        "A count is defined as \\(ℕ\\).",
                        "A count is defined as \\(\\lexeme{lexlean.std.nat::nat}\\).",
                    )
                    .replace(
                        "natural number \\(",
                        "\\(\\lexeme{lexlean.std.nat::nat}\\) \\(",
                    ),
            );
            let checked = support::checked_project(&ambiguous);
            let canonical =
                lexlean::fmt::canonical_source(&checked.modules["Main"], &checked.closure)
                    .expect("formats");
            assert!(
                canonical.contains("A count is defined as \\(\\lexeme{lexlean.std.nat::nat}\\).")
                    && canonical.contains("For every \\(\\lexeme{lexlean.std.nat::nat}\\) \\(n\\), \\(double(n)\\) is defined as \\(n + n\\)."),
                "formatting retains the disambiguating selectors: {canonical}"
            );
            ambiguous.write("src/Main.lex.tex", &canonical);
            ambiguous.check_ok();
            let checked = support::checked_project(&project);
            let declaration = checked.modules["Main"]
                .document
                .declarations()
                .into_iter()
                .find(|d| d.component == "count")
                .expect("count exists");
            match &declaration.body {
                DeclBody::Definition { entry, .. } => {
                    assert_eq!(entry.to_string(), "test.defs::count", "linked to its entry");
                }
                DeclBody::TheoremLike { .. } => panic!("count is a definition"),
            }
        }
        // §15.7: an explicitly typed nonrecursive term def, through a call
        // self head or a noun-of self head, with `;`-separated binders,
        // Lean-verified in the corpus and reproduced by the formatter.
        "DF-02" => {
            let project = support::defs_project();
            let lean = support::lean_text(&support::rendered(&project), "Main");
            assert!(
                lean.contains(
                    "@[expose] public def double (llv0 : Nat) : Nat :=\n  Nat.add llv0 llv0\n"
                ),
                "the term definition emits an explicitly typed def: {lean}"
            );
            if let Some(fixture) = support::corpus_backed("DF-02") {
                assert_eq!(fixture.attestation["status"], "verified");
            }
            assert_eq!(
                support::corpus_declaration_lean("double"),
                "@[expose] public def double (llv0 : Nat) : Nat :=\n  Nat.add llv0 llv0"
            );
            assert_eq!(
                support::corpus_declaration_lean("combine"),
                "@[expose] public def combine (llv0 : Nat) (llv1 : Nat) : Nat :=\n  Nat.add llv0 llv1"
            );
            let checked = support::checked_project(support::shared_corpus_project());
            let canonical =
                lexlean::fmt::canonical_source(&checked.modules["Main"], &checked.closure)
                    .expect("formats");
            assert!(
                canonical.contains("For every natural number \\(n\\), the double of \\(n\\) is defined as \\(n + n\\).")
                    && canonical.contains("For every natural number \\(a\\); natural number \\(b\\), \\(combine(a, b)\\) is defined as \\(a + b\\).")
                    && canonical.contains("For every natural number \\(n\\), the double of \\(n\\) is even."),
                "noun-of self heads, `;` binder lists, and noun-of arguments format canonically: {canonical}"
            );
            // The canonical source of the whole corpus is itself a valid,
            // canonical module (§23.5).
            let reformatted = support::corpus_project();
            reformatted.write("src/Main.lex.tex", &canonical);
            reformatted.check_ok();
            let again = support::checked_project(&reformatted);
            assert_eq!(
                lexlean::fmt::canonical_source(&again.modules["Main"], &again.closure)
                    .expect("formats"),
                canonical,
                "canonical formatting is idempotent over the corpus"
            );
            // `and` between definition binders is not the §15.4 BINDER-LIST
            // separator; a noun-of head with the wrong argument fails rule 4.
            let anded = support::corpus_project();
            anded.edit(
                "src/Main.lex.tex",
                "natural number \\(a\\); natural number \\(b\\), \\(combine(a, b)\\)",
                "natural number \\(a\\) and natural number \\(b\\), \\(combine(a, b)\\)",
            );
            anded.check_fails_with("LLF5001");
            let wrong = support::corpus_project();
            wrong.edit(
                "src/Main.lex.tex",
                "the double of \\(n\\) is defined as",
                "the double of \\(m\\) is defined as",
            );
            let error = wrong.check_fails_with("LLF5001");
            assert!(
                error.diagnostics.iter().all(|d| d.primary.is_some()),
                "self-head diagnostics carry spans: {error}"
            );
        }
        // §15.7 rule 10: a Prop-valued predicate def, through a constant
        // or a text predicate-frame self head (S10), Lean-verified.
        "DF-03" => {
            let project = support::defs_project();
            let lean = support::lean_text(&support::rendered(&project), "Main");
            assert!(
                lean.contains(
                    "@[expose] public def good : Prop :=\n  Exists (fun (llv0 : Nat) => Eq llv0 llv0)\n"
                ),
                "a predicate def returns Prop: {lean}"
            );
            if let Some(fixture) = support::corpus_backed("DF-03") {
                assert_eq!(fixture.attestation["status"], "verified");
            }
            assert_eq!(
                support::corpus_declaration_lean("even"),
                "@[expose] public def even (llv0 : Nat) : Prop :=\n  Exists (fun (llv1 : Nat) => Eq llv0 (Nat.add llv1 llv1))"
            );
            assert_eq!(
                support::corpus_declaration_lean("double_even"),
                "public theorem double_even (llv0 : Nat) : LexLeanExample.Main.even (LexLeanExample.Main.double llv0) := by\n  refine ⟨llv0, ?_⟩\n  rfl"
            );
            let checked = support::checked_project(support::shared_corpus_project());
            let canonical =
                lexlean::fmt::canonical_source(&checked.modules["Main"], &checked.closure)
                    .expect("formats");
            assert!(
                canonical.contains("For every natural number \\(n\\), \\(n\\) is even holds exactly when there exists a natural number \\(k\\) such that \\(n = k + k\\)."),
                "the predicate-frame self head formats canonically: {canonical}"
            );
            // The self head must be the frame over the declared binder.
            let wrong = support::corpus_project();
            wrong.edit(
                "src/Main.lex.tex",
                "\\(n\\) is even holds exactly when",
                "\\(k\\) is even holds exactly when",
            );
            let error = wrong.check_fails_with("LLF5001");
            assert!(
                error.diagnostics.iter().all(|d| d.primary.is_some()),
                "self-head diagnostics carry spans: {error}"
            );
        }
        // §15.7 rules 6-8: no self reference, mutual cycle, or forward use.
        "DF-04" => {
            let recursive = support::defs_project();
            recursive.edit(
                "src/Main.lex.tex",
                "\\(double(n)\\) is defined as \\(n + n\\)",
                "\\(double(n)\\) is defined as \\(double(n) + n\\)",
            );
            let error = recursive.check_err();
            assert!(
                error
                    .diagnostics
                    .iter()
                    .any(|d| matches!(d.code.as_str(), "LLF5001" | "LLR3003" | "LLR3005")),
                "self recursion is rejected: {:?}",
                error
                    .diagnostics
                    .iter()
                    .map(|d| d.code.as_str())
                    .collect::<Vec<_>>()
            );

            let forward = support::defs_project();
            // Move `good` (which references nothing) before `count`, and make
            // it reference the later `double`.
            forward.edit(
                "src/Main.lex.tex",
                "\\(good\\) holds exactly when there exists a natural number \\(k\\) such that \\(k = k\\)",
                "\\(good\\) holds exactly when there exists a natural number \\(k\\) such that \\(double(k) = k\\)",
            );
            let text = forward.read("src/Main.lex.tex");
            let good_block_start = text
                .find("\\begin{predicatedefinition}")
                .expect("good block");
            let good_block_end = text.find("\\end{predicatedefinition}").expect("good end")
                + "\\end{predicatedefinition}".len();
            let good_block = text[good_block_start..good_block_end].to_owned();
            let without = format!(
                "{}{}",
                &text[..good_block_start],
                text[good_block_end..].trim_start_matches('\n')
            );
            let reordered = without.replace(
                "\\begin{typedefinition}",
                &format!("{good_block}\n\n\\begin{{typedefinition}}"),
            );
            forward.write("src/Main.lex.tex", &reordered);
            let error = forward.check_err();
            assert!(
                error
                    .diagnostics
                    .iter()
                    .any(|d| matches!(d.code.as_str(), "LLR3005" | "LLF5001")),
                "a forward reference is rejected: {:?}",
                error
                    .diagnostics
                    .iter()
                    .map(|d| d.code.as_str())
                    .collect::<Vec<_>>()
            );
        }
        // §15.7 rule 4: the self application is exact and ordered.
        "DF-05" => {
            let doubled = support::defs_project();
            doubled.edit(
                "src/Main.lex.tex",
                "\\(double(n)\\) is defined as",
                "\\(double(n, n)\\) is defined as",
            );
            doubled.check_fails_with("LLF5001");

            let renamed = support::defs_project();
            renamed.edit(
                "src/Main.lex.tex",
                "For every natural number \\(n\\), \\(double(n)\\)",
                "For every natural number \\(n\\), \\(double(m)\\)",
            );
            renamed.check_fails_with("LLF5001");
            // Rule 4 holds without a `For every` prefix: a function entry
            // defined as a constant declares too few binders (C16).
            let constant = support::defs_project();
            constant.edit(
                "src/Main.lex.tex",
                "For every natural number \\(n\\), \\(double(n)\\) is defined as \\(n + n\\).",
                "\\(double\\) is defined as \\(0\\).",
            );
            constant.check_fails_with("LLT4004");
        }
        // §15.9: exactly one explicit axiom policy everywhere.
        "DF-06" => {
            let missing = support::defs_project();
            missing.edit(
                "src/Main.lex.tex",
                "\\begin{typedefinition}{count}{test.defs::count}\n\\noaxioms\n",
                "\\begin{typedefinition}{count}{test.defs::count}\n",
            );
            missing.check_fails_with("LLP2003");

            let project = support::defs_project();
            let json = support::checked_project(&project)
                .linked_json()
                .to_canonical_string();
            assert!(
                json.contains("\"policy\""),
                "every linked declaration records its policy: {json}"
            );
        }
        // §15.8: theorem, lemma, corollary all emit Lean theorems while the
        // document metadata stays distinct.
        "DF-07" => {
            let project = P::example();
            project.edit(
                "src/Main.lex.tex",
                "\\begin{theorem}{add-zero}",
                "\\begin{lemma}{add-zero}",
            );
            project.edit("src/Main.lex.tex", "\\end{theorem}", "\\end{lemma}");
            project.check_ok();
            let build = support::rendered(&project);
            let lean = support::lean_text(&build, "Main");
            assert!(
                lean.contains("theorem add_zero"),
                "a lemma emits Lean theorem: {lean}"
            );
            let tex = support::tex_text(&build, "Main");
            assert!(
                tex.contains("\\begin{lemma}"),
                "the document keeps the lemma kind: {tex}"
            );
            let checked = support::checked_project(&project);
            let kind = checked.modules["Main"]
                .document
                .declarations()
                .into_iter()
                .find(|d| d.component == "add-zero")
                .expect("declared")
                .kind;
            assert_eq!(kind, DeclKind::Lemma, "IR metadata keeps the kind");
        }
        // §15.8, §16.12: no author axioms, opaque forms, or proofless
        // theorem-likes.
        "DF-08" => {
            let axiom_env = P::example();
            axiom_env.edit(
                "src/Main.lex.tex",
                "\\begin{theorem}{add-zero}",
                "\\begin{axiom}{add-zero}",
            );
            axiom_env.edit("src/Main.lex.tex", "\\end{theorem}", "\\end{axiom}");
            axiom_env.check_fails_with("LLL1004");

            let proofless = P::example();
            proofless.edit(
                "src/Main.lex.tex",
                "\\begin{proof}\nClose the goal by reflexivity.\n\\end{proof}\n",
                "",
            );
            proofless.check_fails_with("LLF5005");
        }
        // §15.8: exactly one nonempty structured proof.
        "DF-09" => {
            let empty = P::example();
            empty.edit(
                "src/Main.lex.tex",
                "\\begin{proof}\nClose the goal by reflexivity.\n\\end{proof}",
                "\\begin{proof}\n\\end{proof}",
            );
            let error = empty.check_err();
            assert!(
                error
                    .diagnostics
                    .iter()
                    .any(|d| matches!(d.code.as_str(), "LLF5004" | "LLF5003" | "LLF5005")),
                "an empty proof is rejected: {:?}",
                error
                    .diagnostics
                    .iter()
                    .map(|d| d.code.as_str())
                    .collect::<Vec<_>>()
            );

            let doubled = P::example();
            doubled.edit(
                "src/Main.lex.tex",
                "\\end{proof}\n\\end{theorem}",
                "\\end{proof}\n\\begin{proof}\nClose the goal by reflexivity.\n\\end{proof}\n\\end{theorem}",
            );
            doubled.check_fails_with("LLP2003");
        }
        // §15.7 rule 7, §17.5: source order is preserved everywhere.
        "DF-10" => {
            let project = support::defs_project();
            let lean = support::lean_text(&support::rendered(&project), "Main");
            let positions: Vec<usize> = ["def count", "def double", "def good", "theorem add_zero"]
                .iter()
                .map(|needle| {
                    lean.find(needle)
                        .unwrap_or_else(|| panic!("{needle} in {lean}"))
                })
                .collect();
            let mut sorted = positions.clone();
            sorted.sort_unstable();
            assert_eq!(
                positions, sorted,
                "generated declarations preserve source order"
            );
        }
        // §17.11: generic semantic declarations are checked before either
        // fixed backend and generate source that the kernel verifies.
        "DF-11" => {
            let project = support::semantic_project();
            project.check_ok();
            let checked = support::checked_project(&project);
            let semantic = checked.modules["Main"]
                .document
                .semantic
                .as_ref()
                .expect("semantic module");
            support::assert_schema(
                "semantic-module",
                "language-1.1 semantic fixture",
                &serde_json::to_value(semantic).expect("semantic JSON"),
            );
            let rendered = support::rendered(&project);
            let lean = support::lean_text(&rendered, "Main");
            for expected in [
                "public inductive ComponentKind",
                "public structure Box (A : Type)",
                "public structure Component",
                "public class Validatable",
                "public instance (priority := 1000) defaultValidatable",
                "public def allConsecutive",
                "| expected, List.cons value rest =>",
                "public theorem allConsecutive_sound_complete",
                "cases value with",
                "induction values with",
                "simp only [allConsecutive]",
            ] {
                assert!(
                    lean.contains(expected),
                    "generated Lean contains `{expected}`:\n{lean}"
                );
            }
            assert!(!lean.contains("sorry") && !lean.contains("axiom"));
            let tex_path = support::tex_text(&rendered, "Main");
            for expected in ["ComponentKind", "Validatable", "allConsecutive"] {
                assert!(tex_path.contains(expected), "LaTeX contains {expected}");
            }
            let verified = support::verify_ok_backed("DF-11", &project);
            if let Some(attestation) = verified {
                assert!(attestation.root.as_std_path().is_dir());
                assert_ne!(attestation.attestation_id, lexlean::Sha256Digest([0; 32]));
            }

            let mutations = [
                ("\"priority\":1000", "\"priority\":999"),
                ("\"name\":\"sampleComponent\"", "\"name\":\"ComponentKind\""),
                (
                    "\"member\":{\"name\":\"Component\"}",
                    "\"member\":{\"name\":\"MissingType\"}",
                ),
                (
                    "{\"kind\":\"var\",\"name\":\"rest\"}]",
                    "{\"kind\":\"var\",\"name\":\"values\"}]",
                ),
                (
                    "\"spec\":\"lexlean/semantic-module/1\"",
                    "\"lean\":\"def escaped := true\",\"spec\":\"lexlean/semantic-module/1\"",
                ),
                (
                    "\"recursive_argument\":\"values\",\"result\":{\"kind\":\"bool\"}",
                    "\"recursive_argument\":\"values\",\"result\":{\"kind\":\"nat\"}",
                ),
                (
                    "\"field\":\"index\",\"value\":{\"kind\":\"nat\",\"value\":\"0\"}",
                    "\"field\":\"index\",\"value\":{\"kind\":\"bool\",\"value\":true}",
                ),
                (
                    "\"binders\":[],\"body\":{\"kind\":\"bool\",\"value\":true},\"constructor\":{\"name\":\"List.nil\"}",
                    "\"binders\":[],\"body\":{\"kind\":\"nat\",\"value\":\"0\"},\"constructor\":{\"name\":\"List.nil\"}",
                ),
            ];
            for (from, to) in mutations {
                let invalid = support::semantic_project();
                invalid.edit("src/Main.lex.tex", from, to);
                let error = invalid.check_err();
                support::expect_code(&error, "LLT4001");
                assert!(
                    !invalid.root.join(".lexlean/build").as_std_path().exists(),
                    "semantic rejection occurs before a backend"
                );
            }

            let nonexhaustive = support::semantic_project();
            let source = nonexhaustive.read("src/Main.lex.tex");
            let start = source.find("{\"binders\":[],\"body\":{\"kind\":\"bool\",\"value\":true},\"constructor\":{\"name\":\"List.nil\"}},")
                .expect("nil branch");
            let needle = "{\"binders\":[],\"body\":{\"kind\":\"bool\",\"value\":true},\"constructor\":{\"name\":\"List.nil\"}},";
            let mut changed = source;
            changed.replace_range(start..start + needle.len(), "");
            nonexhaustive.write("src/Main.lex.tex", &changed);
            nonexhaustive.check_fails_with("LLT4001");

            // Imported semantic declarations participate in the same typed
            // environment. A remote call cannot bypass arity or result-type
            // checking merely because its declaration is in another module.
            let remote_arity = support::P::semantic_example();
            remote_arity.edit(
                "src/Main.lex.tex",
                r#""arguments":[],"function":{"module":"Support","name":"remoteEnabled"}"#,
                r#""arguments":[{"kind":"nat","value":"0"}],"function":{"module":"Support","name":"remoteEnabled"}"#,
            );
            remote_arity.check_fails_with("LLT4001");

            let remote_result = support::P::semantic_example();
            remote_result.edit(
                "src/Support.lex.tex",
                r#""body":{"kind":"bool","value":true},"kind":"definition","name":"remoteEnabled","parameters":[],"result":{"kind":"bool"}"#,
                r#""body":{"kind":"nat","value":"0"},"kind":"definition","name":"remoteEnabled","parameters":[],"result":{"kind":"nat"}"#,
            );
            remote_result.check_fails_with("LLT4001");

            for (from, to) in [
                (
                    r#""value":{"arguments":[{"kind":"nat"}],"class":{"module":"VariantTypes","name":"DefaultValue"},"kind":"instance_value""#,
                    r#""value":{"arguments":[{"kind":"unit"}],"class":{"module":"VariantTypes","name":"DefaultValue"},"kind":"instance_value""#,
                ),
                (
                    r#""resolved":{"module":"VariantTypes","name":"natDefault"}"#,
                    r#""resolved":{"name":"natUsesDefault"}"#,
                ),
                (
                    r#""class":{"name":"UsesDefault"},"fields""#,
                    r#""class":{"module":"VariantTypes","name":"DefaultValue"},"fields""#,
                ),
            ] {
                let invalid = support::semantic_project();
                invalid.edit("src/VariantInstances.lex.tex", from, to);
                invalid.check_fails_with("LLT4001");
            }

            // Every conservative rejection promised by §17.11 occurs while
            // linking semantic data, before either fixed backend is entered.
            for (path, from, to) in [
                (
                    "src/VariantTypes.lex.tex",
                    r#""name":"MaybeNat","parameters":[]"#,
                    r#""name":"MaybeNat","parameters":[{"name":"index","type":{"kind":"nat"}}]"#,
                ),
                (
                    "src/VariantTypes.lex.tex",
                    r#"{"fields":[{"kind":"nat"}],"name":"some"}"#,
                    r#"{"fields":[{"arguments":[],"kind":"named","member":{"name":"MaybeNat"}}],"name":"some"}"#,
                ),
                (
                    "src/VariantTypes.lex.tex",
                    r#"{"fields":[],"name":"none"},{"fields""#,
                    r#"{"fields":[],"name":"some"},{"fields""#,
                ),
                (
                    "src/VariantTerms.lex.tex",
                    r#""kind":"definition","name":"chooseNat""#,
                    r#""kind":"definition","name":"chooseNat","recursive_argument":"flag""#,
                ),
                (
                    "src/VariantTerms.lex.tex",
                    r#""binders":["prior"],"body""#,
                    r#""binders":[],"body""#,
                ),
                (
                    "src/Main.lex.tex",
                    r#",{"binders":[],"constructor":"database","proof":{"kind":"reflexivity"}}"#,
                    "",
                ),
                (
                    "src/Main.lex.tex",
                    r#""binders":["head","tail","ih"],"constructor":"cons""#,
                    r#""binders":["head","tail"],"constructor":"cons""#,
                ),
                (
                    "src/Main.lex.tex",
                    r#""definitions":[{"name":"allConsecutive"}]"#,
                    r#""definitions":[{"name":"missingDefinition"}]"#,
                ),
                (
                    "src/Main.lex.tex",
                    r#""kind":"induction","scrutinee":"values""#,
                    r#""generalizing":["missing"],"kind":"induction","scrutinee":"values""#,
                ),
                (
                    "src/VariantProofs.lex.tex",
                    r#""kind":"apply","theorem":{"name":"conjunction_refl"}"#,
                    r#""kind":"apply","theorem":{"name":"missingTheorem"}"#,
                ),
                (
                    "src/VariantProofs.lex.tex",
                    r#"{"expected":"0","field":"left"},{"expected":"1","field":"right"}"#,
                    r#"{"expected":"0","field":"left"}"#,
                ),
            ] {
                let invalid = support::semantic_project();
                invalid.edit(path, from, to);
                invalid.check_fails_with("LLT4001");
                assert!(
                    !invalid.root.join(".lexlean/build").as_std_path().exists(),
                    "semantic rejection occurs before a backend"
                );
            }
        }
        // §17.12: recursive data is uniform, strictly positive, and
        // buildable; every rule holds before either backend runs.
        "DF-12" => {
            let project = P::copy_example("recursive-data");
            project.check_ok();
            let rendered = support::rendered(&project);
            let types = support::lean_text(&rendered, "Types");
            for expected in [
                "public inductive Tree (Item : Type) where\n  | leaf\n  | node (_ : Tree (Item)) (_ : Item) (_ : Tree (Item))\n",
                "public inductive Rose (Item : Type) where\n  | node (_ : Item) (_ : List (Rose (Item)))\n",
                "mutual\npublic inductive Expr where\n",
                "public inductive Stmt where\n  | assign (_ : String) (_ : Expr)\n  | sequence (_ : Stmt) (_ : Stmt)\nend\n",
                "mutual\npublic inductive Node (Item : Type) where\n  | node (_ : Item) (_ : Branches (Item))\n\npublic inductive Branches (Item : Type) where\n",
            ] {
                assert!(types.contains(expected), "missing {expected:?} in:\n{types}");
            }
            let tex = support::tex_text(&rendered, "Types");
            assert!(tex.contains("Mutual group: \\texttt{Syntax}"), "{tex}");
            for expected in [
                "\\subsection*{\\texttt{Tree}}\n\\noindent Kind: \\texttt{inductive}.\\par\n\\noindent Type parameters: \\texttt{(Item)}.\\par\n\\noindent Constructor \\texttt{leaf}: \\texttt{()}.\\par\n",
                "\\noindent Type parameters: \\texttt{(Problem, Item)}.\\par\n",
                "\\subsection*{\\texttt{Located}}\n\\noindent Kind: \\texttt{structure}.\\par\n\\noindent Type parameters: \\texttt{(Item)}.\\par\n\\noindent Field \\texttt{value}: \\texttt{Item}.\\par\n",
            ] {
                assert!(tex.contains(expected), "missing {expected:?} in:\n{tex}");
            }
            // Language 1.2 maps every declaration to its own source object,
            // in both artifacts.
            let (_, map_bytes) = rendered
                .files
                .iter()
                .find(|(path, _)| path == "maps/RecursiveData/Types.map.json")
                .expect("the Types source map");
            let map: serde_json::Value = serde_json::from_slice(map_bytes).expect("map JSON");
            let types_text = project.read("src/Types.lex.tex");
            let declaration_nodes: Vec<u64> = map["nodes"]
                .as_array()
                .expect("nodes")
                .iter()
                .filter(|node| node["kind"] == "semantic-declaration")
                .map(|node| node["id"].as_u64().expect("id"))
                .collect();
            assert_eq!(
                declaration_nodes.len(),
                16,
                "eight declarations in each artifact"
            );
            for mapping in map["mappings"].as_array().expect("mappings") {
                if !declaration_nodes.contains(&mapping["node"].as_u64().expect("node")) {
                    continue;
                }
                let range =
                    |key: &str| usize::try_from(mapping[key].as_u64().expect(key)).expect("fits");
                let object: serde_json::Value =
                    serde_json::from_str(&types_text[range("src_start")..range("src_end")])
                        .expect("a declaration maps to exactly its own source object");
                let name = object["name"].as_str().expect("a declaration name");
                let generated = if mapping["artifact"] == 0 {
                    &types
                } else {
                    &tex
                };
                assert!(
                    generated[range("gen_start")..range("gen_end")].contains(name),
                    "the generated range of `{name}` names it"
                );
            }
            let _ = support::verify_ok_backed("DF-12", &project);

            let types_source = project.read("src/Types.lex.tex");
            let mutate = |from: &str, to: &str, message: &str| {
                let copy = P::copy_example("recursive-data");
                assert!(types_source.contains(from), "fixture lacks {from:?}");
                copy.write("src/Types.lex.tex", &types_source.replacen(from, to, 1));
                let error = copy.check_fails_with("LLT4001");
                assert!(
                    error.to_string().contains(message),
                    "expected {message:?}, got {error}"
                );
                copy.assert_no_backend_output();
            };
            let tree = r#"{"arguments":[{"kind":"parameter","name":"Item"}],"kind":"named","member":{"name":"Tree"}}"#;
            // Non-uniform recursive occurrence.
            mutate(
                tree,
                r#"{"arguments":[{"kind":"nat"}],"kind":"named","member":{"name":"Tree"}}"#,
                "non-uniform recursive occurrence of `Tree`",
            );
            // Bad type arguments on a recursive occurrence.
            mutate(
                tree,
                r#"{"arguments":[],"kind":"named","member":{"name":"Tree"}}"#,
                "type `Tree` expects 1 argument(s), received 0",
            );
            // Positivity: a recursive occurrence inside another document
            // type. The added declaration is otherwise well formed and unused,
            // so only the positivity rule can refuse it.
            mutate(
                r#"{"fields":[{"name":"value""#,
                r#"{"constructors":[{"fields":[],"name":"empty"},{"fields":[{"arguments":[{"arguments":[],"kind":"named","member":{"name":"Wrap"}}],"kind":"named","member":{"name":"Tree"}}],"name":"wrap"}],"kind":"inductive","name":"Wrap","parameters":[],"type_parameters":[]},{"fields":[{"name":"value""#,
                "positivity violation in `Wrap.wrap`",
            );
            // An uninhabited cycle: removing the only base case of `Tree`.
            mutate(
                r#"{"fields":[],"name":"leaf"},"#,
                "",
                "uninhabited recursive cycle: no constructor of inductive `Tree`",
            );
            // A one-member group, and members with different type
            // parameters.
            mutate(
                r#""mutual":"Syntax","name":"Stmt""#,
                r#""mutual":"Other","name":"Stmt""#,
                "has one member; a standalone inductive omits `mutual`",
            );
            mutate(
                r#""mutual":"Syntax","name":"Stmt","parameters":[],"type_parameters":[]"#,
                r#""mutual":"Syntax","name":"Stmt","parameters":[],"type_parameters":["Item"]"#,
                "members must declare identical type parameters",
            );
            // A later group member is admitted by the group, and still meets
            // every naming rule: here it duplicates an earlier declaration.
            mutate(
                r#""mutual":"Syntax","name":"Stmt""#,
                r#""mutual":"Syntax","name":"Tree""#,
                "duplicate generated name `Tree`",
            );
            // A built-in constructor owner cannot be redeclared, or a local
            // `Option.some` would be ambiguous.
            mutate(
                r#""kind":"inductive","name":"Outcome""#,
                r#""kind":"inductive","name":"Option""#,
                "declaration name `Option` is reserved for the built-in type",
            );
            // Binder hygiene: a type parameter spelled like a declaration of
            // the module (the inductive itself or a mutual group member), a
            // built-in type the backend names, or the module prefix would
            // capture that name in generated Lean.
            let rename_problem = |to: &str, message: &str| {
                let copy = P::copy_example("recursive-data");
                copy.write(
                    "src/Types.lex.tex",
                    &types_source.replace(r#""Problem""#, &format!("\"{to}\"")),
                );
                let error = copy.check_fails_with("LLT4001");
                assert!(
                    error.to_string().contains(message),
                    "expected {message:?}, got {error}"
                );
                copy.assert_no_backend_output();
            };
            rename_problem(
                "Outcome",
                "binder `Outcome` in `Outcome` is spelled like the declaration `Outcome`",
            );
            rename_problem(
                "Expr",
                "binder `Expr` in `Outcome` is spelled like the declaration `Expr`",
            );
            rename_problem(
                "Prod",
                "binder `Prod` in `Outcome` is spelled like the built-in Lean name `Prod`",
            );
            rename_problem(
                "Nat",
                "binder `Nat` in `Outcome` is spelled like the built-in Lean name `Nat`",
            );
            rename_problem(
                "RecursiveData",
                "binder `RecursiveData` in `Outcome` is spelled like the module prefix `RecursiveData`",
            );
            // A declaration spelled like a name the backend writes without
            // qualification would be that name in every later signature of
            // its module (`error: type expected, got (Int : Nat -> Nat)`), and
            // quoting cannot help: linking refuses it, whatever it declares.
            // A name that the generated-Lean audit forbids but nothing
            // captures is not one of them.
            for name in lexlean::ir::semantic::BACKEND_BARE_NAMES {
                for kind in ["definition", "structure", "inductive"] {
                    let declaration = match kind {
                        "definition" => serde_json::json!({
                            "body": {"kind": "add", "left": {"kind": "var", "name": "n"}, "right": {"kind": "nat", "value": "1"}},
                            "executable": true, "kind": "definition", "name": name,
                            "parameters": [{"name": "n", "type": {"kind": "nat"}}],
                            "result": {"kind": "nat"},
                        }),
                        "structure" => serde_json::json!({
                            "fields": [{"name": "f", "type": {"kind": "nat"}}],
                            "kind": "structure", "name": name, "parameters": [], "type_parameters": [],
                        }),
                        _ => serde_json::json!({
                            "constructors": [{"fields": [], "name": "k"}],
                            "kind": "inductive", "name": name, "parameters": [], "type_parameters": [],
                        }),
                    };
                    let copy = P::negative("declaration-lean-name");
                    let data = serde_json::json!({"declarations": [declaration], "spec": "lexlean/semantic-module/2"});
                    copy.write(
                        "src/Main.lex.tex",
                        &format!(
                            "\\begin{{lexlean}}{{Main}}\n\\useglossary{{lexlean.std.nat@1.2.0}}\n\\title{{Natural number addition}}\n\\begin{{semanticmodule}}\n\\semanticdata{{{data}}}\n\\end{{semanticmodule}}\n\\end{{lexlean}}\n"
                        ),
                    );
                    let error = copy.check_fails_with("LLT4001");
                    assert!(
                        error
                            .to_string()
                            .contains(&format!("declaration name `{name}` is ")),
                        "a {kind} named `{name}`: {error}"
                    );
                    copy.assert_no_backend_output();
                }
            }
            // A self-referential structure.
            mutate(
                r#"{"name":"value","type":{"kind":"parameter","name":"Item"}}"#,
                r#"{"name":"value","type":{"arguments":[{"kind":"parameter","name":"Item"}],"kind":"named","member":{"name":"Located"}}}"#,
                "structure or class `Located` refers to itself",
            );

            // Language 1.1 keeps rejecting every recursive payload.
            let eleven = P::semantic_example();
            let support_source = eleven.read("src/Support.lex.tex");
            eleven.write(
                "src/Support.lex.tex",
                &support_source.replacen(
                    r#"{"fields":[],"name":"enabled"}"#,
                    r#"{"fields":[{"arguments":[],"kind":"named","member":{"name":"RemoteFlag"}}],"name":"enabled"}"#,
                    1,
                ),
            );
            let error = eleven.check_fails_with("LLT4001");
            assert_eq!(
                error.to_string(),
                "LLT4001: phase link: forward or missing type `RemoteFlag`"
            );

            // Resource accounting charges the data module exactly: one node
            // per declaration, per type node of every field, and, in
            // language 1.2, per type parameter, per constructor, and per
            // mutual label. The limit equal to the charge of the modules
            // through `Types` admits it, and one less overruns.
            let limited_project = |limit: u64| {
                let limited = P::copy_example("recursive-data");
                limited.edit(
                    "lexlean.toml",
                    "max_ir_nodes = 2000000",
                    &format!("max_ir_nodes = {limit}"),
                );
                limited.relock();
                limited
            };
            let limit_at = |limit: u64| {
                limited_project(limit)
                    .check_fails_with("LLS8002")
                    .to_string()
            };
            let message = limit_at(1);
            assert!(message.contains("through module `Types`"), "{message}");
            let observed: u64 = message
                .split("observed ")
                .nth(1)
                .and_then(|tail| tail.split(' ').next())
                .and_then(|count| count.parse().ok())
                .expect("observed node count");
            // Declarations 8; field type nodes 32; type parameters 7;
            // constructors 13; mutual labels 4.
            assert_eq!(observed, 64, "the exact charge of `Types`");
            assert!(limit_at(observed - 1).contains(&format!("observed {observed} ")));
            let at_limit = limited_project(observed)
                .check_fails_with("LLS8002")
                .to_string();
            assert!(
                !at_limit.contains("through module `Types`"),
                "the exact charge of `Types` is admitted: {at_limit}"
            );
            // A constructor or a field spelled like a member that Lean
            // declares for every type is declared twice (`constant has
            // already been declared 'T.rec'`): linking refuses it, for each
            // such name and the numbered ones of nested and mutual types.
            let member_project = |declaration: serde_json::Value, root: &str| {
                let copy = P::negative("declaration-lean-name");
                let data = serde_json::json!({"declarations": [declaration], "spec": "lexlean/semantic-module/2"});
                let _ = root;
                copy.write(
                    "src/Main.lex.tex",
                    &format!(
                        "\\begin{{lexlean}}{{Main}}\n\\useglossary{{lexlean.std.nat@1.2.0}}\n\\title{{Natural number addition}}\n\\begin{{semanticmodule}}\n\\semanticdata{{{data}}}\n\\end{{semanticmodule}}\n\\end{{lexlean}}\n"
                    ),
                );
                copy
            };
            let numbered = ["rec_1", "rec_2", "below_1", "brecOn_1"];
            for name in lexlean::ir::semantic::LEAN_GENERATED_MEMBERS
                .iter()
                .copied()
                .chain(numbered)
            {
                let constructor = member_project(
                    serde_json::json!({
                        "constructors": [{"fields": [], "name": name}, {"fields": [], "name": "other"}],
                        "kind": "inductive", "name": "T", "parameters": [], "type_parameters": [],
                    }),
                    "constructor",
                );
                // `mk` is the constructor of every structure and nothing
                // special for an inductive.
                if name == "mk" {
                    constructor.check_ok();
                } else {
                    let error = constructor.check_fails_with("LLT4001");
                    assert!(
                        error.to_string().contains(&format!(
                            "constructor name `{name}` is spelled like a member"
                        )),
                        "a constructor `{name}`: {error}"
                    );
                }
                let field = member_project(
                    serde_json::json!({
                        "fields": [{"name": name, "type": {"kind": "nat"}}],
                        "kind": "structure", "name": "T", "parameters": [], "type_parameters": [],
                    }),
                    "field",
                );
                let error = field.check_fails_with("LLT4001");
                assert!(
                    error
                        .to_string()
                        .contains(&format!("field name `{name}` is spelled like a member")),
                    "a field `{name}`: {error}"
                );
            }
            // The names that are not members are free: `rec_`, `recur`,
            // `mk` as a constructor, `toCtorIdx_`.
            for name in ["rec_", "recur", "rec1", "casesOn_", "ctorIdx0", "below1"] {
                member_project(
                    serde_json::json!({
                        "constructors": [{"fields": [], "name": name}],
                        "kind": "inductive", "name": "T", "parameters": [], "type_parameters": [],
                    }),
                    "constructor",
                )
                .check_ok();
            }
            if support::lean_backed("DF-12") {
                lean_generated_members_are_refused();
            }
        }
        // §17.12: structural recursion and induction over a recursive
        // inductive use exactly its direct recursive fields.
        "DF-13" => {
            let project = P::copy_example("recursive-data");
            project.check_ok();
            let main = support::lean_text(&support::rendered(&project), "Main");
            for expected in [
                "| RecursiveData.Types.Tree.node left _ right => ((treeSize (left) + 1) + treeSize (right))",
                "induction tree with\n  | leaf =>\n    simp only [mirror]\n  | node left value right leftHypothesis rightHypothesis =>\n",
            ] {
                assert!(main.contains(expected), "missing {expected:?} in:\n{main}");
            }
            let _ = support::verify_ok_backed("DF-13", &project);

            let source = project.read("src/Main.lex.tex");
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
            // A recursive call on the matched value itself.
            mutate(
                r#""arguments":[{"kind":"var","name":"left"}],"function":{"name":"treeSize"}"#,
                r#""arguments":[{"kind":"var","name":"tree"}],"function":{"name":"treeSize"}"#,
                "recursive call `treeSize` is not on a structurally smaller value",
            );
            // A value rebuilt from the fields is not smaller than the match
            // scrutinee: only a direct recursive field binder is. For a
            // self-recursive inductive these are exactly the binders of the
            // scrutinee's own type, so typing and this rule coincide.
            mutate(
                r#""arguments":[{"kind":"var","name":"right"}],"function":{"name":"mirror"}"#,
                r#""arguments":[{"arguments":[{"kind":"var","name":"left"},{"kind":"var","name":"value"},{"kind":"var","name":"right"}],"constructor":{"module":"Types","name":"Tree.node"},"kind":"constructor","type_arguments":[{"kind":"nat"}]}],"function":{"name":"mirror"}"#,
                "recursive call `mirror` is not on a structurally smaller value",
            );
            // A standalone definition cannot recurse over a nested type: it has
            // no single structural eliminator.
            mutate(
                r#""name":"roseLabel","parameters":[{"name":"rose","type":{"arguments":[{"kind":"nat"}],"kind":"named","member":{"module":"Types","name":"Rose"}}}],"result""#,
                r#""name":"roseLabel","parameters":[{"name":"rose","type":{"arguments":[{"kind":"nat"}],"kind":"named","member":{"module":"Types","name":"Rose"}}}],"recursive_argument":"rose","result""#,
                "standalone structural recursion requires a self-recursive inductive",
            );
            // One hypothesis per direct recursive field.
            mutate(
                r#""binders":["left","value","right","leftHypothesis","rightHypothesis"]"#,
                r#""binders":["left","value","right","leftHypothesis"]"#,
                "proof branch `node` expects 5 binder(s), received 4",
            );
            // Induction over a nested inductive is not a closed proof form.
            mutate(
                r#"{"kind":"theorem","name":"rose_label","parameters":[],"proof":{"kind":"reflexivity"}"#,
                r#"{"kind":"theorem","name":"rose_label","parameters":[{"name":"rose","type":{"arguments":[{"kind":"nat"}],"kind":"named","member":{"module":"Types","name":"Rose"}}}],"proof":{"branches":[{"binders":["label","children"],"constructor":"node","proof":{"kind":"reflexivity"}}],"kind":"induction","scrutinee":"rose"}"#,
                "requires a self-recursive inductive without nested or mutual occurrences",
            );
        }
        // §17.12: generic definitions and theorems are explicitly
        // instantiated, never polymorphically recursive, and closed over
        // their declared type parameters.
        "DF-14" => {
            let project = P::copy_example("higher-order");
            project.check_ok();
            let rendered = support::rendered(&project);
            let combinators = support::lean_text(&rendered, "Combinators");
            assert!(
                combinators.contains("public def mapList (Input : Type) (Output : Type) : (transform : ((Input) -> (Output))) -> (values : List (Input)) -> List (Output)\n"),
                "{combinators}"
            );
            assert!(combinators.contains("mapList (Input) (Output) (transform) (tail)"));
            let main = support::lean_text(&rendered, "Main");
            assert!(main.contains("exact map_identity (Nat) ("), "{main}");
            let _ = support::verify_ok_backed("DF-14", &project);

            // A type parameter its declaration never mentions lowers as
            // `_name`, so Lean's unused-variable linter stays silent and the
            // project still verifies with no unexpected output.
            let phantom = P::copy_example("higher-order");
            let source = phantom.read("src/Combinators.lex.tex");
            let end = r#"],"spec":"lexlean/semantic-module/2"}"#;
            phantom.write(
                "src/Combinators.lex.tex",
                &source.replacen(
                    end,
                    &format!(
                        r#",{{"body":{{"kind":"var","name":"value"}},"kind":"definition","name":"keepNat","parameters":[{{"name":"value","type":{{"kind":"nat"}}}}],"result":{{"kind":"nat"}},"type_parameters":["Phantom"]}},{{"kind":"theorem","name":"phantomTheorem","parameters":[],"proof":{{"kind":"reflexivity"}},"statement":{{"kind":"eq","left":{{"kind":"nat","value":"1"}},"right":{{"kind":"nat","value":"1"}}}},"type_parameters":["Ghost"]}}{end}"#
                    ),
                    1,
                ),
            );
            phantom.check_ok();
            let phantom_lean = support::lean_text(&support::rendered(&phantom), "Combinators");
            for expected in [
                "public def keepNat (_Phantom : Type) (value : Nat) : Nat := value\n",
                "public theorem phantomTheorem (_Ghost : Type) : (1 = 1) := by\n",
            ] {
                assert!(
                    phantom_lean.contains(expected),
                    "missing {expected:?} in:\n{phantom_lean}"
                );
            }
            let _ = support::verify_ok_backed("DF-14", &phantom);

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
            mutate(
                "src/Main.lex.tex",
                r#""function":{"module":"Combinators","name":"mapList"},"kind":"call","type_arguments":[{"kind":"nat"},{"kind":"nat"}]"#,
                r#""function":{"module":"Combinators","name":"mapList"},"kind":"call""#,
                "function `Combinators::mapList` expects 2 explicit type argument(s), received 0",
            );
            mutate(
                "src/Combinators.lex.tex",
                r#""function":{"name":"mapList"},"kind":"call","type_arguments":[{"kind":"parameter","name":"Input"},{"kind":"parameter","name":"Output"}]"#,
                r#""function":{"name":"mapList"},"kind":"call","type_arguments":[{"kind":"parameter","name":"Output"},{"kind":"parameter","name":"Input"}]"#,
                "polymorphic recursion is not permitted",
            );
            mutate(
                "src/Combinators.lex.tex",
                r#"{"element":{"kind":"parameter","name":"Output"},"kind":"nil"}"#,
                r#"{"element":{"kind":"parameter","name":"Other"},"kind":"nil"}"#,
                "unbound type parameter `Other`",
            );
            mutate(
                "src/Main.lex.tex",
                r#""theorem":{"name":"map_identity"},"type_arguments":[{"kind":"nat"}]"#,
                r#""theorem":{"name":"map_identity"}"#,
                "theorem `map_identity` expects 1 explicit type argument(s), received 0",
            );

            // Language 1.1 has no generic definitions, and a type it writes
            // inside a definition body is closed over the empty scope.
            let eleven = P::semantic_example();
            let support_source = eleven.read("src/Support.lex.tex");
            let generic = support_source.replacen(
                r#""name":"remoteEnabled","parameters":[],"result":{"kind":"bool"}"#,
                r#""name":"remoteEnabled","parameters":[],"result":{"kind":"bool"},"type_parameters":["Item"]"#,
                1,
            );
            assert_ne!(generic, support_source, "the 1.1 fixture is mutated");
            eleven.write("src/Support.lex.tex", &generic);
            let error = eleven.check_fails_with("LLT4001");
            assert!(
                error
                    .to_string()
                    .contains("`definition type parameters` is a language-1.2 construct"),
                "{error}"
            );
        }
        // §17.12: an executable definition forms only non-escaping closures
        // and calls only executable definitions.
        "DF-15" => {
            let project = P::copy_example("higher-order");
            project.check_ok();
            let rendered = support::rendered(&project);
            let tex = support::tex_text(&rendered, "Main");
            // The document states every parameter with its type and every
            // closure with exactly what it binds and captures.
            assert!(
                tex.contains("\\subsection*{\\texttt{addAll}}\n\\noindent Kind: \\texttt{definition}.\\par\n\\noindent Parameters: \\texttt{(offset : Nat) (values : List (Nat))}.\\par\n\\noindent Closure 1: binds \\texttt{(value)}, captures \\texttt{(offset)}.\\par\n\\noindent Execution: executable, non-escaping closures only.\\par\n"),
                "{tex}"
            );
            let combinators_tex = support::tex_text(&rendered, "Combinators");
            assert!(
                combinators_tex.contains("\\noindent Type parameters: \\texttt{(Input, Output)}.\\par\n\\noindent Parameters: \\texttt{(transform : ((Input) -> (Output))) (values : List (Input))}.\\par\n"),
                "{combinators_tex}"
            );
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
            // Returning a closure.
            mutate(
                "src/Combinators.lex.tex",
                r#""kind":"definition","name":"compose""#,
                r#""executable":true,"kind":"definition","name":"compose""#,
                "escaping closure: executable definition `compose` returns a function",
            );
            // Returning a record of closures: a document type whose fields
            // hold functions holds them too.
            mutate(
                "src/Main.lex.tex",
                r#""kind":"definition","name":"evaluator""#,
                r#""executable":true,"kind":"definition","name":"evaluator""#,
                "escaping closure: executable definition `evaluator` returns a value of type Combinators.Visitor, which holds a function",
            );
            // Receiving closures inside data.
            mutate(
                "src/Combinators.lex.tex",
                r#""kind":"definition","name":"visit""#,
                r#""executable":true,"kind":"definition","name":"visit""#,
                "escaping closure: executable definition `visit` parameter `visitor` stores a function in data",
            );
            // A closure stored in a pair escapes even when never returned.
            mutate(
                "src/Main.lex.tex",
                r#""body":{"arguments":[{"body":{"kind":"add","left":{"kind":"var","name":"sum"},"right":{"kind":"var","name":"value"}},"captures":[],"kind":"lambda","parameters":[{"name":"sum","type":{"kind":"nat"}},{"name":"value","type":{"kind":"nat"}}]},{"kind":"nat","value":"0"},{"kind":"var","name":"values"}]"#,
                r#""body":{"arguments":[{"body":{"kind":"add","left":{"kind":"var","name":"sum"},"right":{"kind":"var","name":"value"}},"captures":[],"kind":"lambda","parameters":[{"name":"sum","type":{"kind":"nat"}},{"name":"value","type":{"kind":"nat"}}]},{"kind":"first","value":{"kind":"pair","left":{"kind":"nat","value":"0"},"right":{"body":{"kind":"var","name":"probe"},"captures":[],"kind":"lambda","parameters":[{"name":"probe","type":{"kind":"nat"}}]}}},{"kind":"var","name":"values"}]"#,
                "escaping closure in executable definition `total`",
            );
            // A non-executable callee.
            mutate(
                "src/Combinators.lex.tex",
                r#""executable":true,"kind":"definition","name":"mapList""#,
                r#""kind":"definition","name":"mapList""#,
                "executable definition `addAll` calls non-executable `Combinators::mapList`",
            );
            // Every rejection rule of higher-order code, on a declaration
            // appended to an otherwise valid module.
            for (file, declarations, message) in [
                (
                    "src/Main.lex.tex",
                    r#"{"body":{"kind":"nat","value":"0"},"executable":true,"kind":"definition","name":"probeHigher","parameters":[{"name":"handler","type":{"kind":"function","parameters":[{"kind":"function","parameters":[{"kind":"nat"}],"result":{"kind":"nat"}}],"result":{"kind":"nat"}}}],"result":{"kind":"nat"}}"#,
                    "executable definition `probeHigher` parameter `handler` is a higher-order function of functions",
                ),
                (
                    "src/Main.lex.tex",
                    r#"{"body":{"binder":{"name":"step","type":{"kind":"function","parameters":[{"kind":"nat"}],"result":{"kind":"nat"}}},"body":{"arguments":[{"kind":"var","name":"number"}],"function":{"kind":"var","name":"step"},"kind":"apply"},"kind":"let","value":{"body":{"kind":"var","name":"value"},"captures":[],"kind":"lambda","parameters":[{"name":"value","type":{"kind":"nat"}}]}},"executable":true,"kind":"definition","name":"probeLet","parameters":[{"name":"number","type":{"kind":"nat"}}],"result":{"kind":"nat"}}"#,
                    "escaping closure in executable definition `probeLet`: a function is bound by let",
                ),
                (
                    "src/Main.lex.tex",
                    r#"{"body":{"kind":"first","value":{"kind":"pair","left":{"kind":"var","name":"number"},"right":{"kind":"var","name":"step"}}},"executable":true,"kind":"definition","name":"probeValue","parameters":[{"name":"step","type":{"kind":"function","parameters":[{"kind":"nat"}],"result":{"kind":"nat"}}},{"name":"number","type":{"kind":"nat"}}],"result":{"kind":"nat"}}"#,
                    "escaping closure in executable definition `probeValue`: the function `step` is used as a value",
                ),
                (
                    "src/Main.lex.tex",
                    r#"{"body":{"kind":"first","value":{"kind":"pair","left":{"kind":"var","name":"number"},"right":{"function":{"module":"Combinators","name":"increment"},"kind":"function_ref"}}},"executable":true,"kind":"definition","name":"probeReference","parameters":[{"name":"number","type":{"kind":"nat"}}],"result":{"kind":"nat"}}"#,
                    "escaping closure in executable definition `probeReference`: a function reference may only be passed directly to an executable function parameter or applied",
                ),
                (
                    "src/Main.lex.tex",
                    r#"{"body":{"arguments":[{"kind":"var","name":"number"}],"function":{"condition":{"kind":"bool","value":true},"else_value":{"kind":"var","name":"step"},"kind":"if","then_value":{"kind":"var","name":"step"}},"kind":"apply"},"executable":true,"kind":"definition","name":"probeApply","parameters":[{"name":"step","type":{"kind":"function","parameters":[{"kind":"nat"}],"result":{"kind":"nat"}}},{"name":"number","type":{"kind":"nat"}}],"result":{"kind":"nat"}}"#,
                    "executable definition `probeApply` applies a function value that is not a parameter, lambda, or function reference",
                ),
                (
                    "src/Main.lex.tex",
                    r#"{"body":{"kind":"add","left":{"kind":"var","name":"value"},"right":{"kind":"nat","value":"1"}},"kind":"definition","name":"plainIncrement","parameters":[{"name":"value","type":{"kind":"nat"}}],"result":{"kind":"nat"}},{"body":{"arguments":[{"function":{"name":"plainIncrement"},"kind":"function_ref"},{"kind":"var","name":"values"}],"function":{"module":"Combinators","name":"mapList"},"kind":"call","type_arguments":[{"kind":"nat"},{"kind":"nat"}]},"executable":true,"kind":"definition","name":"probeFormal","parameters":[{"name":"values","type":{"element":{"kind":"nat"},"kind":"list"}}],"result":{"element":{"kind":"nat"},"kind":"list"}}"#,
                    "executable definition `probeFormal` references non-executable `plainIncrement`",
                ),
                (
                    "src/Main.lex.tex",
                    r#"{"body":{"branches":[{"binders":[],"body":{"kind":"nat","value":"0"},"constructor":{"name":"Nat.zero"}},{"binders":["smaller"],"body":{"arguments":[{"kind":"var","name":"smaller"}],"function":{"function":{"name":"probeSelf"},"kind":"function_ref"},"kind":"apply"},"constructor":{"name":"Nat.succ"}}],"kind":"match","scrutinee":{"kind":"var","name":"number"}},"kind":"definition","name":"probeSelf","parameters":[{"name":"number","type":{"kind":"nat"}}],"recursive_argument":"number","result":{"kind":"nat"}}"#,
                    "recursive definition `probeSelf` cannot be referenced as a value of itself",
                ),
                (
                    "src/Combinators.lex.tex",
                    r#"{"body":{"kind":"var","name":"item"},"kind":"definition","name":"probeCapture","parameters":[{"name":"item","type":{"kind":"parameter","name":"Visitor"}}],"result":{"kind":"parameter","name":"Visitor"},"type_parameters":["Visitor"]}"#,
                    "binder `Visitor` in `probeCapture` is spelled like the declaration `Visitor`",
                ),
                (
                    "src/Main.lex.tex",
                    r#"{"body":{"kind":"var","name":"item"},"kind":"definition","name":"probeBound","parameters":[{"name":"item","type":{"kind":"parameter","name":"item"}}],"result":{"kind":"parameter","name":"item"},"type_parameters":["item"]}"#,
                    "type parameter `item` is also bound as a value in the same declaration",
                ),
                (
                    "src/Main.lex.tex",
                    r#"{"body":{"arguments":[{"kind":"var","name":"number"}],"function":{"body":{"kind":"nat","value":"1"},"captures":[],"kind":"lambda","parameters":[]},"kind":"apply"},"kind":"definition","name":"probeLambda","parameters":[{"name":"number","type":{"kind":"nat"}}],"result":{"kind":"nat"}}"#,
                    "a lambda binds at least one parameter",
                ),
                (
                    "src/Main.lex.tex",
                    r#"{"body":{"kind":"nat","value":"0"},"kind":"definition","name":"probeFunctionType","parameters":[{"name":"step","type":{"kind":"function","parameters":[],"result":{"kind":"nat"}}}],"result":{"kind":"nat"}}"#,
                    "a function type has at least one parameter",
                ),
                (
                    "src/Main.lex.tex",
                    r#"{"body":{"arguments":[],"function":{"kind":"var","name":"step"},"kind":"apply"},"kind":"definition","name":"probeNoArguments","parameters":[{"name":"step","type":{"kind":"function","parameters":[{"kind":"nat"}],"result":{"kind":"nat"}}}],"result":{"kind":"nat"}}"#,
                    "an application supplies at least one argument",
                ),
                (
                    "src/Main.lex.tex",
                    r#"{"body":{"kind":"var","name":"item"},"kind":"definition","name":"probeOwn","parameters":[{"name":"item","type":{"kind":"parameter","name":"probeOwn"}}],"result":{"kind":"parameter","name":"probeOwn"},"type_parameters":["probeOwn"]}"#,
                    "binder `probeOwn` in `probeOwn` is spelled like the declaration `probeOwn`",
                ),
                (
                    "src/Main.lex.tex",
                    r#"{"body":{"kind":"var","name":"item"},"kind":"definition","name":"probePrefix","parameters":[{"name":"item","type":{"kind":"parameter","name":"HigherOrder"}}],"result":{"kind":"parameter","name":"HigherOrder"},"type_parameters":["HigherOrder"]}"#,
                    "binder `HigherOrder` in `probePrefix` is spelled like the module prefix `HigherOrder`",
                ),
                (
                    "src/Main.lex.tex",
                    r#"{"body":{"arguments":[{"body":{"kind":"var","name":"addAll"},"captures":[],"kind":"lambda","parameters":[{"name":"addAll","type":{"kind":"nat"}}]},{"kind":"var","name":"values"}],"function":{"module":"Combinators","name":"mapList"},"kind":"call","type_arguments":[{"kind":"nat"},{"kind":"nat"}]},"kind":"definition","name":"probeShadow","parameters":[{"name":"values","type":{"element":{"kind":"nat"},"kind":"list"}}],"result":{"element":{"kind":"nat"},"kind":"list"}}"#,
                    "binder `addAll` in `probeShadow` is spelled like the declaration `addAll`",
                ),
            ] {
                let copy = P::copy_example("higher-order");
                let source = copy.read(file);
                let end = r#"],"spec":"lexlean/semantic-module/2"}"#;
                assert!(source.contains(end), "{file} ends its declarations");
                copy.write(file, &source.replacen(end, &format!(",{declarations}{end}"), 1));
                let error = copy.check_fails_with("LLT4001");
                assert!(
                    error.to_string().contains(message),
                    "expected {message:?}, got {error}"
                );
                copy.assert_no_backend_output();
            }
        }
        // §17.12: mutual structural recursion over one recursive family.
        "DF-16" => {
            let project = P::copy_example("recursion");
            project.check_ok();
            let rendered = support::rendered(&project);
            let main = support::lean_text(&rendered, "Main");
            for expected in [
                "mutual\n@[expose] public def roseSize (Item : Type) : (rose : Recursion.Syntax.Rose (Item)) -> Nat\n",
                "  | List.cons head tail => (roseSize (Item) (head) + forestSize (Item) (tail))\ntermination_by structural forest => forest\nend\n",
                "  | Recursion.Syntax.Stmt.assign _ value => (exprSize (value) + 1)\n",
                "termination_by structural number => number\nend\n",
            ] {
                assert!(main.contains(expected), "missing {expected:?} in:\n{main}");
            }
            let tex = support::tex_text(&rendered, "Main");
            assert!(tex.contains(
                "Mutual recursion group \\texttt{SyntaxSize}, decreasing on \\texttt{expression}."
            ));
            let _ = support::verify_ok_backed("DF-16", &project);
            // A member parameter that does not decrease is still bound by
            // `termination_by structural`; it lowers as `_` so Lean's
            // unused-variable linter, which fails verification, stays quiet.
            let extra = P::copy_example("recursion");
            let mut parity = extra.read("src/Main.lex.tex");
            for (from, to) in [
                (
                    r#"{"binders":[],"body":{"kind":"bool","value":true},"constructor":{"name":"Nat.zero"}},{"binders":["previous"],"body":{"arguments":[{"kind":"var","name":"previous"}],"function":{"name":"isOdd"},"kind":"call"}"#,
                    r#"{"binders":[],"body":{"kind":"var","name":"flip"},"constructor":{"name":"Nat.zero"}},{"binders":["previous"],"body":{"arguments":[{"kind":"var","name":"previous"},{"kind":"var","name":"flip"}],"function":{"name":"isOdd"},"kind":"call"}"#,
                ),
                (
                    r#"{"arguments":[{"kind":"var","name":"previous"}],"function":{"name":"isEven"},"kind":"call"}"#,
                    r#"{"arguments":[{"kind":"var","name":"previous"},{"kind":"var","name":"flip"}],"function":{"name":"isEven"},"kind":"call"}"#,
                ),
                (
                    r#""name":"isEven","parameters":[{"name":"number","type":{"kind":"nat"}}]"#,
                    r#""name":"isEven","parameters":[{"name":"number","type":{"kind":"nat"}},{"name":"flip","type":{"kind":"bool"}}]"#,
                ),
                (
                    r#""name":"isOdd","parameters":[{"name":"number","type":{"kind":"nat"}}]"#,
                    r#""name":"isOdd","parameters":[{"name":"number","type":{"kind":"nat"}},{"name":"flip","type":{"kind":"bool"}}]"#,
                ),
                (
                    r#""arguments":[{"kind":"nat","value":"10"}],"function":{"name":"isEven"}"#,
                    r#""arguments":[{"kind":"nat","value":"10"},{"kind":"bool","value":true}],"function":{"name":"isEven"}"#,
                ),
            ] {
                assert_eq!(parity.matches(from).count(), 1, "{from}");
                parity = parity.replacen(from, to, 1);
            }
            extra.write("src/Main.lex.tex", &parity);
            extra.check_ok();
            let extra_main = support::lean_text(&support::rendered(&extra), "Main");
            assert!(
                extra_main.contains("termination_by structural number _ => number\nend\n"),
                "{extra_main}"
            );
            let _ = support::verify_ok_backed("DF-16", &extra);
            let reject = |from: &str, to: &str, message: &str| {
                P::assert_mutation_rejected("recursion", "src/Main.lex.tex", from, to, message);
            };
            // A non-decreasing call between members.
            reject(
                r#""arguments":[{"kind":"var","name":"previous"}],"function":{"name":"isOdd"}"#,
                r#""arguments":[{"kind":"var","name":"number"}],"function":{"name":"isOdd"}"#,
                "recursive call `isOdd` is not on a structurally smaller value",
            );
            // A call on a value that is not a smaller family binder (a fresh
            // empty forest).
            reject(
                r#""arguments":[{"kind":"var","name":"children"}],"function":{"name":"forestSize"}"#,
                r#""arguments":[{"element":{"arguments":[{"kind":"parameter","name":"Item"}],"kind":"named","member":{"module":"Syntax","name":"Rose"}},"kind":"nil"}],"function":{"name":"forestSize"}"#,
                "recursive call `forestSize` is not on a structurally smaller value",
            );
            // A missing case, a member without a decreasing argument, a
            // one-member group, and different families.
            reject(
                r#"{"binders":[],"body":{"kind":"bool","value":false},"constructor":{"name":"Nat.zero"}},"#,
                "",
                "nonexhaustive or mixed match branches",
            );
            reject(
                r#""mutual":"Parity","name":"isOdd","parameters":[{"name":"number","type":{"kind":"nat"}}],"recursive_argument":"number""#,
                r#""mutual":"Parity","name":"isOdd","parameters":[{"name":"number","type":{"kind":"nat"}}]"#,
                "mutual group `Parity` member `isOdd` names no decreasing argument",
            );
            reject(
                r#""mutual":"Parity","name":"isOdd""#,
                r#""mutual":"Other","name":"isOdd""#,
                "has one member; a standalone definition omits `mutual`",
            );
            reject(
                r#""mutual":"Parity","name":"isEven""#,
                r#""mutual":"SyntaxSize","name":"isEven""#,
                "mutual group `SyntaxSize` members decrease on different recursive families",
            );
            // Production eligibility holds member by member: an executable
            // member may not call a formal one.
            reject(
                r#""kind":"definition","mutual":"Parity","name":"isEven""#,
                r#""executable":true,"kind":"definition","mutual":"Parity","name":"isEven""#,
                "executable definition `isEven` calls non-executable `isOdd`",
            );
        }
        // §17.12: well-founded recursion with statement-exact evidence.
        "DF-17" => {
            let project = P::copy_example("recursion");
            project.check_ok();
            let rendered = support::rendered(&project);
            let main = support::lean_text(&rendered, "Main");
            for expected in [
                "@[expose, semireducible] public def countdown (number : Nat) (steps : Nat) : Nat := (match (generalizing := false) __decrease0 : (Nat.blt (number) (2)) with | true => steps | false => countdown ((LexLeanRuntime.subtract (number) (2) : Nat)) ((steps + 1)))\ntermination_by number\ndecreasing_by all_goals first | (have __evidence := countdown_decreases (number) (steps) (__decrease0); subst_vars; exact __evidence)\n",
                "decreasing_by all_goals first | (have __evidence := search_decreases (target) (low) (high) (__decrease0) (__decrease1); subst_vars; exact __evidence)\n",
                "public theorem countdown_decreases (number : Nat) (_steps : Nat) :",
                "public def reassociate (term : Recursion.Syntax.Term) : Recursion.Syntax.Term := (match (generalizing := false) __decrease0 : term with ",
                "(match (generalizing := false) __decrease1 : left with ",
                // Enclosing match binders are solved from the hypotheses, so
                // a binder the branch ignores is `_` and never a lint.
                "decreasing_by all_goals first | (have __evidence := reassociate_literal (term) _ _ _ (__decrease0) (__decrease1); subst_vars; exact __evidence) | (have __evidence := reassociate_plus (term) _ _ _ _ (__decrease0) (__decrease1); subst_vars; exact __evidence)\n",
                "| Recursion.Syntax.Term.literal _ => prune (right) | Recursion.Syntax.Term.plus inner _ => prune (Recursion.Syntax.Term.plus (inner) (right))",
                        ] {
                assert!(main.contains(expected), "missing {expected:?} in:\n{main}");
            }
            // A well-founded mutual group is one `mutual` block; `ping`
            // calls `pong` on the same argument, which only the two measures
            // order, so the obligation compares the callee's measure.
            for expected in [
                "mutual\n@[expose, semireducible] public def ping (number : Nat) : Nat := ",
                "termination_by ((number + number) + 1)\ndecreasing_by all_goals first | (have __evidence := ping_to_pong (number) (__decrease0); subst_vars; exact __evidence)\n",
                "decreasing_by all_goals first | (have __evidence := pong_to_ping (number) (__decrease0); subst_vars; exact __evidence)\nend\n",
                "public theorem ping_to_pong (number : Nat) : (((Nat.beq (number) (0)) = false) -> ((number + number) < ((number + number) + 1)))",
            ] {
                assert!(main.contains(expected), "missing {expected:?} in:\n{main}");
            }
            let tex = support::tex_text(&rendered, "Main");
            assert!(tex.contains("Well-founded measure:"), "{tex}");
            // Lean must accept every well-founded definition with its bound
            // evidence; verify_ok_backed fails the case otherwise.
            let _ = support::verify_ok_backed("DF-17", &project);
            let reject = |from: &str, to: &str, message: &str| {
                P::assert_mutation_rejected("recursion", "src/Main.lex.tex", from, to, message);
            };
            // A cross-member obligation states the callee's measure: evidence
            // comparing the caller's own measure is not it.
            reject(
                r#""statement":{"conclusion":{"kind":"lt","left":{"kind":"add","left":{"kind":"var","name":"number"},"right":{"kind":"var","name":"number"}}"#,
                r#""statement":{"conclusion":{"kind":"lt","left":{"kind":"add","left":{"kind":"add","left":{"kind":"var","name":"number"},"right":{"kind":"var","name":"number"}},"right":{"kind":"nat","value":"1"}}"#,
                "evidence `ping_to_pong` for recursive call 0 of `ping` does not state its decrease obligation exactly",
            );
            // The group's call graph: `pong` stops calling `ping`.
            reject(
                r#""else_value":{"kind":"add","left":{"arguments":[{"arguments":[{"kind":"var","name":"number"},{"kind":"nat","value":"1"}],"kind":"primitive","operation":"subtract","result":{"kind":"nat"}}],"function":{"name":"ping"},"kind":"call"}"#,
                r#""else_value":{"kind":"add","left":{"arguments":[{"kind":"var","name":"number"},{"kind":"nat","value":"1"}],"kind":"primitive","operation":"subtract","result":{"kind":"nat"}}"#,
                "mutual group `Bounce` member `pong` calls no member of its group",
            );
            // A well-founded member cannot join a structural group.
            reject(
                r#""mutual":"Bounce","name":"pong""#,
                r#""mutual":"Folding","name":"pong""#,
                "mutual group `Folding`",
            );
            // Forged evidence: a theorem whose statement is not the call's
            // decrease obligation.
            reject(
                r#""statement":{"conclusion":{"kind":"lt","left":{"arguments":[{"kind":"var","name":"number"},{"kind":"nat","value":"2"}]"#,
                r#""statement":{"conclusion":{"kind":"lt","left":{"arguments":[{"kind":"var","name":"number"},{"kind":"nat","value":"1"}]"#,
                "does not state its decrease obligation exactly",
            );
            // A cyclic measure.
            reject(
                r#""termination":{"evidence":[{"name":"countdown_decreases"}],"measure":{"kind":"var","name":"number"}}"#,
                r#""termination":{"evidence":[{"name":"countdown_decreases"}],"measure":{"arguments":[{"kind":"var","name":"number"},{"kind":"var","name":"steps"}],"function":{"name":"countdown"},"kind":"call"}}"#,
                "refers to `countdown` itself or a member of its group (cyclic measure)",
            );
            // One evidence theorem per call site, no more.
            reject(
                r#""termination":{"evidence":[{"name":"countdown_decreases"}]"#,
                r#""termination":{"evidence":[{"name":"countdown_decreases"},{"name":"countdown_decreases"}]"#,
                "has 1 recursive call site(s) but 2 evidence theorem(s)",
            );

            // Both structural and well-founded recursion.
            reject(
                r#""name":"countdown","parameters":[{"name":"number","type":{"kind":"nat"}},{"name":"steps","type":{"kind":"nat"}}],"result":{"kind":"nat"}"#,
                r#""name":"countdown","parameters":[{"name":"number","type":{"kind":"nat"}},{"name":"steps","type":{"kind":"nat"}}],"recursive_argument":"number","result":{"kind":"nat"}"#,
                "declares both structural and well-founded recursion",
            );
            // Language 1.1 has no termination evidence.
            let eleven = P::semantic_example();
            let support_source = eleven.read("src/Support.lex.tex");
            eleven.write(
                "src/Support.lex.tex",
                &support_source.replacen(
                    r#""name":"remoteEnabled","parameters":[],"result":{"kind":"bool"}"#,
                    r#""name":"remoteEnabled","parameters":[],"result":{"kind":"bool"},"termination":{"evidence":[{"name":"missing"}],"measure":{"kind":"nat","value":"0"}}"#,
                    1,
                ),
            );
            let error = eleven.check_fails_with("LLT4001");
            assert!(
                error.to_string().contains("is a language-1.2 construct"),
                "{error}"
            );
        }
        // §17.12: explicit state threading through ordered folds and
        // bounded iteration is executable with direct closures.
        "DF-18" => {
            let project = P::copy_example("collections");
            project.check_ok();
            let main = support::lean_text(&support::rendered(&project), "Main");
            assert!(
                main.contains(
                    "(LexLeanCollections.iterateUntil ((fun (state : List ((Prod (Nat) (Nat)))) =>"
                ),
                "{main}"
            );
            let _ = support::verify_ok_backed("DF-18", &project);
            let reject = |file: &str, from: &str, to: &str, message: &str| {
                P::assert_mutation_rejected("collections", file, from, to, message);
            };
            // Iteration is always bounded: the fuel argument is required.
            reject(
                "src/Main.lex.tex",
                r#"{"kind":"nat","value":"8"},{"kind":"var","name":"parents"}"#,
                r#"{"kind":"var","name":"parents"}"#,
                "primitive IterateUntil expects 3 argument(s) (step, natural-number bound, initial state), received 2",
            );
            // A fold step must take the state first and return it: the
            // swapped step below is well typed on its own, so the only
            // error is that it does not thread the state.
            reject(
                "src/Main.lex.tex",
                r#""parameters":[{"name":"visited","type":{"element":{"kind":"nat"},"kind":"list"}},{"name":"element","type":{"kind":"nat"}}]},{"element":{"kind":"nat"},"kind":"nil"},{"head":{"kind":"nat","value":"5"}"#,
                r#""parameters":[{"name":"element","type":{"kind":"nat"}},{"name":"visited","type":{"element":{"kind":"nat"},"kind":"list"}}]},{"element":{"kind":"nat"},"kind":"nil"},{"head":{"kind":"nat","value":"5"}"#,
                "primitive ListFold argument 0 has type ((Nat) -> (List (Nat)) -> (List (Nat))), expected ((List (Nat)) -> (Nat) -> (List (Nat)))",
            );
        }
        other => panic!("no declarations case is wired for {other}"),
    }
}

/// A second type-noun with the text surface `natural number` and the math
/// surface `ℕ`, so bare surfaces resolve to two visible entries.
pub(super) const DUP_NAT_ENTRY: &str = r#"spec = "lexlean/entry/1"
id = "nat2"
category = "type-noun"
signature = "(sort (type 0))"
surface_arity = 0
frame = "atom"

[denotation]
kind = "lean"
module = "Init"
name = "Int"

[[form]]
id = "natural-number"
channel = "text"
surface = "natural number"
canonical_source = true
features = ["article-a", "lower-case", "singular"]

[[form]]
id = "blackboard"
channel = "math"
surface = "ℕ"
canonical_source = true
features = []

[render]
math = "(seq (token mathbb) (group (token blackboard-n)))"
"#;

/// The members Lean declares for a type besides the user's constructors and
/// fields are all among those linking refuses: sample inductives (recursive,
/// nested, mutual, higher-order, enumerations) and structures are declared
/// in the pinned Lean, and the first-level names it holds under each are
/// listed.
fn lean_generated_members_are_refused() {
    let project = P::copy_example("production");
    let loaded = lexlean::project::Project::load(&project.root.join("lexlean.toml")).expect("load");
    let toolchain =
        lexlean::verify::toolchain::preflight(&loaded.config.limits).expect("the pinned toolchain");
    let source = "import Lean
open Lean Elab Command Meta
inductive T1 | a | b (x : Nat)
inductive T2 | a | b (n : Nat) (t : T2)
inductive T3 (α : Type) | a | b (x : α) (t : T3 α) (l : List (T3 α))
structure S1 where
  x : Nat
structure S2 (α : Type) where
  x : α
  y : List α
mutual
inductive M1 | a | b (m : M2)
inductive M2 | c (m : M1) | d
end
inductive T4 | a (n : Nat) | b (f : Nat → T4)
structure S3 where
  x : Nat
  h : x = x
inductive E1 | a | b | c
def generated (ts : List Name) : CoreM Unit := do
  let env ← getEnv
  for t in ts do
    let mut names : Array String := #[]
    for (n, _) in env.constants.toList do
      if n.getPrefix == t then names := names.push n.getString!
    IO.println s!\"{t}: {names.qsort (· < ·)}\"
#eval generated [`T1, `T2, `T3, `S1, `S2, `M1, `M2, `T4, `S3, `E1]
";
    let _guard = support::env_lock();
    let directory = tempfile::tempdir().expect("tempdir");
    let path = directory.path().join("Generated.lean");
    std::fs::write(&path, source).expect("write");
    let output = std::process::Command::new(toolchain.lean.path.as_std_path())
        .arg(&path)
        .current_dir(directory.path())
        .output()
        .expect("lean runs");
    let text = String::from_utf8_lossy(&output.stdout).into_owned();
    // The user's own members of each sample.
    let own: std::collections::BTreeMap<&str, &[&str]> = [
        ("T1", &["a", "b"][..]),
        ("T2", &["a", "b"][..]),
        ("T3", &["a", "b"][..]),
        ("S1", &["x"][..]),
        ("S2", &["x", "y"][..]),
        ("M1", &["a", "b"][..]),
        ("M2", &["c", "d"][..]),
        ("T4", &["a", "b"][..]),
        ("S3", &["x", "h"][..]),
        ("E1", &["a", "b", "c"][..]),
    ]
    .into_iter()
    .collect();
    let mut seen = 0;
    for line in text.lines() {
        let Some((type_name, names)) = line.split_once(": #[") else {
            continue;
        };
        let names = names.trim_end_matches(']');
        let user = own[type_name];
        for name in names.split(", ") {
            if user.contains(&name) || name.starts_with('_') {
                continue;
            }
            let field = type_name.starts_with('S');
            assert!(
                lexlean::ir::semantic::is_lean_generated_member(name, true)
                    || lexlean::ir::semantic::is_lean_generated_member(name, field),
                "Lean declares `{type_name}.{name}`, which a constructor or field may still be named: {text}"
            );
            seen += 1;
        }
    }
    assert!(
        seen > 60,
        "the samples are read ({seen} generated names): {text}"
    );
}
