//! GL-22: typed semantic exports are consumed by the ordinary prose pipeline.
use crate::support::{self, P};

const COLOR: &str = "(const repro.words::color)";
const DOMAIN: &str = r"\begin{lexlean}{Colors}
\useglossary{lexlean.std.nat@1.2.0}
\useglossary{languagetwelve.titles@1.0.0}
\title{Typed let terms}
\begin{semanticmodule}
inductive{name: Color, type_parameters: [], parameters: [], constructors: [{name: red, fields: []}, {name: green, fields: []}]}
structure{name: Point, type_parameters: [], parameters: [], fields: [{name: shade, type: Color}]}
definition{name: redValue, parameters: [], result: Color, body: (ctor Color.red)}
definition{name: natValue, parameters: [], result: Nat, body: 0}
definition{name: identityColor, parameters: [c : Color], result: Color, body: c}
definition{name: generic, type_parameters: [A], parameters: [x : 'A], result: 'A, body: x}
definition{name: isRed, parameters: [c : Color], result: Prop, body: (c = (redValue))}
definition{name: booleanRed, parameters: [c : Color], result: Bool, body: true}
definition{name: branching, parameters: [c : Color], result: Color, body: (match c | Color.red => (ctor Color.red) | Color.green => (ctor Color.green))}
class{name: ColorClass, type_parameters: [], parameters: [], fields: [{name: shade, type: Color}]}
theorem{name: nonLse, parameters: [], statement: ((match (ctor Color.red) | Color.red => 0 | Color.green => 0) = 0), proof: reflexivity{}, axioms: []}
theorem{name: letHeader, parameters: [], statement: (let number : Nat := 0; (number = number)), proof: reflexivity{}, axioms: []}
theorem{name: redRefl, parameters: [], statement: (isRed (redValue)), proof: reflexivity{}, axioms: []}
\end{semanticmodule}
\end{lexlean}
";
const MAIN: &str = r"\begin{lexlean}{Main}
\useglossary{lexlean.std.nat@1.2.0}
\useglossary{languagetwelve.titles@1.0.0}
\useglossary{repro.words@1.0.0}
\importmodule{Colors}
\title{Typed let terms}
\begin{theorem}{color-refl}
\noaxioms
For every color \(c\), \(c = c\).
\begin{proof}
Close the goal by reflexivity.
\end{proof}
\end{theorem}
\begin{theorem}{point-refl}
\noaxioms
For every point \(p\), \(p = p\).
\begin{proof}
Close the goal by reflexivity.
\end{proof}
\end{theorem}
\begin{theorem}{function-use}
\noaxioms
\(identitycolor(redvalue) = identitycolor(redvalue)\).
\begin{proof}
Close the goal by reflexivity.
\end{proof}
\end{theorem}
\begin{theorem}{generic-use}
\noaxioms
\(generic(ℕ, 0) = generic(ℕ, 0)\).
\begin{proof}
Close the goal by reflexivity.
\end{proof}
\end{theorem}
\begin{theorem}{predicate-and-proof-use}
\noaxioms
\(redvalue\) is red.
\begin{proof}
Close the goal with \(redrefl\).
\end{proof}
\end{theorem}
\end{lexlean}
";

fn entry(
    id: &str,
    category: &str,
    signature: &str,
    component: &str,
    surface: &str,
    arity: usize,
) -> String {
    let text = matches!(category, "type-noun" | "adjective-predicate");
    let channel = if text { "text" } else { "math" };
    let frame = if category == "adjective-predicate" {
        "adjective"
    } else if arity == 0 {
        "atom"
    } else {
        "call"
    };
    let features = if category == "type-noun" {
        r#"["article-a", "lower-case", "singular"]"#
    } else {
        "[]"
    };
    let render = if text {
        String::new()
    } else if arity == 0 {
        format!("\n[render]\nmath = \"(operator-name {surface})\"\n")
    } else {
        let arguments = (0..arity)
            .map(|index| format!("(slot {index})"))
            .collect::<Vec<_>>()
            .join(" (token comma) ");
        format!(
            "\n[render]\nmath = \"(seq (operator-name {surface}) (paren (seq {arguments})))\"\n"
        )
    };
    format!(
        r#"spec = "lexlean/entry/1"
id = "{id}"
category = "{category}"
signature = "{signature}"
surface_arity = {arity}
frame = "{frame}"
[denotation]
kind = "document"
module = "Colors"
component = "{component}"
[[form]]
id = "{id}"
channel = "{channel}"
surface = "{surface}"
canonical_source = true
features = {features}
{render}"#
    )
}

fn fixture() -> P {
    let project = P::language_1_2_example();
    let rows = [
        (
            "color",
            entry("color", "type-noun", "(sort (type 0))", "Color", "color", 0),
        ),
        (
            "point",
            entry("point", "type-noun", "(sort (type 0))", "Point", "point", 0),
        ),
        (
            "red",
            entry("red", "term-constant", COLOR, "redValue", "redvalue", 0),
        ),
        (
            "identity",
            entry(
                "identity",
                "function",
                &format!("(pi ((explicit c {COLOR})) {COLOR})"),
                "identityColor",
                "identitycolor",
                1,
            ),
        ),
        (
            "generic",
            entry(
                "generic",
                "function",
                "(pi ((explicit T (sort (type 0))) (explicit v (local T))) (local T))",
                "generic",
                "generic",
                2,
            ),
        ),
        (
            "isred",
            entry(
                "isred",
                "adjective-predicate",
                &format!("(pi ((explicit c {COLOR})) (sort prop))"),
                "isRed",
                "red",
                1,
            ),
        ),
        (
            "letproof",
            entry("letproof", "proof-constant", "(let renamed (const lexlean.std.nat::nat) (nat 0) (app (const lexlean.core::eq) (local renamed) (local renamed)))", "letHeader", "letproof", 0),
        ),
        (
            "redrefl",
            entry(
                "redrefl",
                "proof-constant",
                "(app (const repro.words::isred) (const repro.words::red))",
                "redRefl",
                "redrefl",
                0,
            ),
        ),
    ];
    let files = rows
        .iter()
        .map(|(name, data)| (format!("{name}.toml"), data))
        .collect::<Vec<_>>();
    let refs = files
        .iter()
        .map(|(name, data)| (name.as_str(), data.as_str()))
        .collect::<Vec<_>>();
    project.add_package(
        "glossary/repro.words",
        "repro.words",
        &["lexlean.core@1.2.0", "lexlean.std.nat@1.2.0"],
        &refs,
    );
    project.edit(
        "glossary/repro.words/lexicon.toml",
        "language = \"1.0\"",
        "language = \"1.2\"",
    );
    project.write_raw("src/Colors.lex.tex", DOMAIN);
    project.write_raw("src/Main.lex.tex", MAIN);
    project.relock();
    project
}

fn no_publication(project: &P) {
    for path in [".lexlean/build", ".lexlean/verified"] {
        assert!(
            !project.root.join(path).exists(),
            "a linking refusal published {path}"
        );
    }
}

fn rejected_check(project: &P, case: &str, code: &str) -> lexlean::LexLeanError {
    eprintln!("GL22 negative {case}: expect {code}");
    let error = project.check_fails_with(code);
    eprintln!("GL22 negative {case}: observed {code}");
    error
}

fn rejected_entry(file: &str, from: &str, to: &str, code: &str) {
    let project = fixture();
    project.edit(
        &format!("glossary/repro.words/entries/{file}.toml"),
        from,
        to,
    );
    project.relock();
    rejected_check(&project, &format!("entry/{file}: {from:?} -> {to:?}"), code);
    no_publication(&project);
}

pub(super) fn run() {
    // This first prose declaration is the original independently observed
    // Color failure; the remaining declarations extend that same real fixture.
    let project = fixture();
    project.check_ok();
    let checked = support::checked_project(&project);
    let_header_comparison(&checked);
    let linked = checked.linked_json().to_canonical_string();
    let lean = support::lean_text(&support::rendered(&project), "Main");
    for component in [
        "Color",
        "Point",
        "redValue",
        "identityColor",
        "generic",
        "isRed",
        "redRefl",
    ] {
        let actual = format!("LanguageTwelve.Colors.{component}");
        assert!(
            linked.contains(&actual),
            "checked DocumentDeclRef keeps {actual}"
        );
        assert!(
            lean.contains(&actual),
            "Lean uses the actual semantic declaration {actual}"
        );
    }
    assert!(!lean.contains("LanguageTwelve.Colors.color_refl"));

    // All positive declarations share one tiny real Lean/kernel/axiom run.
    if let Some(verified) = support::verify_ok_backed("GL-22", &project) {
        let directory = project
            .root
            .join(".lexlean/verified")
            .join(verified.attestation_id.to_hex());
        let attestation: serde_json::Value = serde_json::from_slice(
            &std::fs::read(directory.join("attestation.json")).expect("attestation bytes"),
        )
        .expect("attestation JSON");
        let declarations = attestation["declarations"]
            .as_array()
            .expect("audited declarations");
        for name in [
            "LanguageTwelve.Main.color_refl",
            "LanguageTwelve.Main.point_refl",
            "LanguageTwelve.Main.function_use",
            "LanguageTwelve.Main.generic_use",
            "LanguageTwelve.Main.predicate_and_proof_use",
            "LanguageTwelve.Colors.redRefl",
        ] {
            let row = declarations
                .iter()
                .find(|row| row["name"] == name)
                .unwrap_or_else(|| panic!("{name} is audited"));
            assert_eq!(
                row["observed"],
                serde_json::json!([]),
                "{name} has the exact empty observed axiom set"
            );
        }
        // False propositions remain rejected after the bridge exists.
        project.write_raw(
            "src/Colors.lex.tex",
            &DOMAIN.replace(
                "result: Prop, body: (c = (redValue))",
                "result: Prop, body: (c = (ctor Color.green))",
            ),
        );
        project.relock();
        eprintln!("GL22 negative false-predicate-body: expect LLV7002");
        project.verify_fails_with("LLV7002");
        eprintln!("GL22 negative false-predicate-body: observed LLV7002");
    }

    // Parameter spellings are alpha-equivalent, but mode, scope and sort
    // remain exact. Semantic type parameters are explicit and monomorphic.
    // Keep the altered entry locally well formed so the mode mismatch is
    // rejected by the semantic bridge, not package arity/render validation.
    let wrong_mode = fixture();
    wrong_mode.write_raw(
        "glossary/repro.words/entries/generic.toml",
        &entry(
            "generic",
            "function",
            "(pi ((implicit T (sort (type 0))) (explicit v (local T))) (local T))",
            "generic",
            "generic",
            1,
        ),
    );
    wrong_mode.write_raw(
        "src/Main.lex.tex",
        &MAIN.replace("generic(ℕ, 0)", "generic(redvalue)"),
    );
    wrong_mode.relock();
    let error = rejected_check(&wrong_mode, "implicit-mode", "LLT4001");
    assert!(error.diagnostics.iter().any(|diagnostic| diagnostic
        .message
        .contains("actual scoped semantic declaration")
        && diagnostic.message.contains("Colors::generic")));
    no_publication(&wrong_mode);
    rejected_entry("generic", "(sort (type 0))", "(sort (type 1))", "LLT4001");

    // Renaming a claimed universe cannot manufacture a semantic universe
    // parameter: these declarations quantify explicit monomorphic types.
    for universe in ["u", "renamed"] {
        let phantom = fixture();
        let path = "glossary/repro.words/entries/generic.toml";
        let entry = phantom
            .read_raw(path)
            .replace(
                "surface_arity = 2",
                &format!("universes = [\"{universe}\"]\nsurface_arity = 2"),
            )
            .replace("(sort (type 0))", &format!("(sort (type {universe}))"));
        phantom.write_raw(path, &entry);
        phantom.relock();
        rejected_check(&phantom, &format!("phantom-universe/{universe}"), "LLT4001");
        no_publication(&phantom);
    }

    let unsupported = fixture();
    unsupported.write_raw(
        "src/Main.lex.tex",
        &MAIN.replace(
            "Close the goal with \\(redrefl\\).",
            "Close the goal with \\(\\reference{Colors::nonLse}\\).",
        ),
    );
    unsupported.relock();
    let error = rejected_check(&unsupported, "unsupported-match-header", "LLT4001");
    assert!(error.diagnostics.iter().any(|diagnostic| diagnostic
        .message
        .contains("no exact prose LSE representation")
        && diagnostic.message.contains("match")));
    no_publication(&unsupported);
    let unsupported_kind = fixture();
    unsupported_kind.edit(
        "glossary/repro.words/entries/red.toml",
        "component = \"redValue\"",
        "component = \"ColorClass\"",
    );
    unsupported_kind.relock();
    let error = rejected_check(&unsupported_kind, "unsupported-class-kind", "LLT4001");
    assert!(error
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.message.contains("class")));
    no_publication(&unsupported_kind);

    rejected_entry("color", "(sort (type 0))", "(sort (type 1))", "LLT4001");
    // An independent entry avoids invalidating redRefl's own LSE
    // dependency on the correctly typed redValue glossary entry.
    let wrong_result = fixture();
    wrong_result.write_raw(
        "glossary/repro.words/entries/wrongresult.toml",
        &entry(
            "wrongresult",
            "term-constant",
            COLOR,
            "natValue",
            "wrongresult",
            0,
        ),
    );
    wrong_result.write_raw(
        "src/Main.lex.tex",
        &MAIN.replace("identitycolor(redvalue)", "wrongresult"),
    );
    wrong_result.relock();
    let error = rejected_check(&wrong_result, "wrong-result-natValue", "LLT4001");
    assert!(error.diagnostics.iter().any(|diagnostic| diagnostic
        .message
        .contains("actual scoped semantic declaration")
        && diagnostic.message.contains("Colors::natValue")));
    no_publication(&wrong_result);
    rejected_entry(
        "identity",
        &format!("(explicit c {COLOR})"),
        "(explicit c (const lexlean.std.nat::nat))",
        "LLT4001",
    );
    rejected_entry(
        "identity",
        &format!(")) {COLOR})"),
        ")) (const lexlean.std.nat::nat))",
        "LLT4001",
    );
    rejected_entry(
        "isred",
        "component = \"isRed\"",
        "component = \"booleanRed\"",
        "LLT4001",
    );
    rejected_entry("redrefl", "(app (const repro.words::isred) (const repro.words::red))", "(app (const lexlean.core::eq) (const lexlean.std.nat::zero) (const lexlean.std.nat::zero))", "LLT4001");
    rejected_entry(
        "red",
        "component = \"redValue\"",
        "component = \"redRefl\"",
        "LLT4001",
    );
    rejected_entry(
        "redrefl",
        "component = \"redRefl\"",
        "component = \"Color\"",
        "LLT4001",
    );

    // Direct references carry independently derived types too: no glossary
    // signature can turn a semantic type into a proof. Use a closed core
    // equality goal: an opaque document predicate intentionally defers
    // non-certain proof mismatches to native Lean. A certain mismatch keeps
    // the original expected-type diagnostic, LLT4001.
    let wrong_reference = fixture();
    wrong_reference.write_raw(
        "src/Main.lex.tex",
        &MAIN
            .replace("\\(redvalue\\) is red.", "\\(redvalue = redvalue\\).")
            .replace(
                "Close the goal with \\(redrefl\\).",
                "Close the goal with \\(\\reference{Colors::Color}\\).",
            ),
    );
    wrong_reference.relock();
    rejected_check(
        &wrong_reference,
        "type-reference-as-closed-equality-proof",
        "LLT4001",
    );
    no_publication(&wrong_reference);

    for (import, component) in [(false, "redValue"), (true, "missing")] {
        let unavailable = fixture();
        let source = MAIN.replace(
            "identitycolor(redvalue)",
            &format!("\\reference{{Colors::{component}}}"),
        );
        unavailable.write_raw(
            "src/Main.lex.tex",
            &if import {
                source
            } else {
                source.replace("\\importmodule{Colors}\n", "")
            },
        );
        unavailable.relock();
        // Type-noun checking comes first when the import is absent.
        rejected_check(
            &unavailable,
            &format!("unavailable/import={import}/{component}"),
            if import { "LLR3005" } else { "LLT4001" },
        );
        no_publication(&unavailable);
    }

    // Isolate constant availability from the first Color type-noun: this
    // consumer has no imported semantic types that could mask the refusal.
    let unimported_constant = fixture();
    unimported_constant.write_raw(
        "src/Main.lex.tex",
        r"\begin{lexlean}{Main}
\useglossary{lexlean.std.nat@1.2.0}
\useglossary{languagetwelve.titles@1.0.0}
\title{Typed let terms}
\begin{theorem}{unimported-value}
\noaxioms
\(\reference{Colors::natValue} = \reference{Colors::natValue}\).
\begin{proof}
Close the goal by reflexivity.
\end{proof}
\end{theorem}
\end{lexlean}
",
    );
    unimported_constant.relock();
    rejected_check(&unimported_constant, "unimported-constant", "LLR3005");
    no_publication(&unimported_constant);

    // An import's imports do not leak their semantic declarations.
    let transitive = fixture();
    transitive.write_raw(
        "src/Hidden.lex.tex",
        &DOMAIN.replace("{Colors}", "{Hidden}"),
    );
    transitive.write_raw(
        "src/Colors.lex.tex",
        &DOMAIN.replace("\\title", "\\importmodule{Hidden}\n\\title"),
    );
    transitive.write_raw(
        "src/Main.lex.tex",
        &MAIN.replace("identitycolor(redvalue)", "\\reference{Hidden::redValue}"),
    );
    transitive.relock();
    rejected_check(&transitive, "transitive-constant", "LLR3005");
    no_publication(&transitive);

    // Same-surface overloads stay separate until exact argument types select.
    let overloaded = fixture();
    let other = entry(
        "other",
        "term-constant",
        "(const lexlean.std.nat::nat)",
        "natValue",
        "redvalue",
        0,
    );
    overloaded.add_package(
        "glossary/repro.zother",
        "repro.zother",
        &["lexlean.core@1.2.0", "lexlean.std.nat@1.2.0"],
        &[("other.toml", &other)],
    );
    overloaded.edit(
        "glossary/repro.zother/lexicon.toml",
        "language = \"1.0\"",
        "language = \"1.2\"",
    );
    overloaded.write_raw(
        "src/Main.lex.tex",
        &MAIN.replace(
            "\\importmodule",
            "\\useglossary{repro.zother@1.0.0}\n\\importmodule",
        ),
    );
    overloaded.relock();
    let text = support::lean_text(&support::rendered(&overloaded), "Main");
    assert!(text.contains("LanguageTwelve.Colors.redValue"));
    assert!(!text.contains("LanguageTwelve.Colors.natValue"));

    prose_order_and_package_names();
}

fn prose_order_and_package_names() {
    // Keep existing prose source-position visibility exactly as before.
    let prose = support::defs_project();
    prose.check_ok();
    let source = prose.read_raw("src/Main.lex.tex");
    let anchor = source
        .find("\\begin{termdefinition}")
        .expect("a prose definition");
    let early = "\\begin{theorem}{too-early}\n\\noaxioms\n\\(\\reference{Main::double} = \\reference{Main::double}\\).\n\\begin{proof}\nClose the goal by reflexivity.\n\\end{proof}\n\\end{theorem}\n";
    prose.write_raw(
        "src/Main.lex.tex",
        &format!("{}{early}{}", &source[..anchor], &source[anchor..]),
    );
    prose.relock();
    rejected_check(&prose, "prose-before-definition", "LLR3005");
    no_publication(&prose);

    // Package sibling imports use the source-visible key `Shapes`, while
    // their reference and Lean constant retain `PkgLib.Shapes.quad`.
    let app = P::package_workspace("app");
    let path = app.workspace_dir().join("lib/src/Util.lex.tex");
    std::fs::write(
        path.as_std_path(),
        r"\begin{lexlean}{Util}
\useglossary{lexlean.std.nat@1.2.0}
\importmodule{Shapes}
\title{Natural number addition}
\begin{theorem}{alias-use}
\noaxioms
For every natural number \(n\), \(\reference{Shapes::quad}(n) = \reference{Shapes::quad}(n)\).
\begin{proof}
Close the goal by reflexivity.
\end{proof}
\end{theorem}
\end{lexlean}
",
    )
    .expect("write package consumer");
    app.write_raw(
        "src/Main.lex.tex",
        r"\begin{lexlean}{Main}
\useglossary{lexlean.std.nat@1.2.0}
\importmodule{PkgLib.Util}
\title{Natural number addition}
\begin{theorem}{import-use}
\noaxioms
For every natural number \(n\), \(n = n\).
\begin{proof}
Close the goal by reflexivity.
\end{proof}
\end{theorem}
\end{lexlean}
",
    );
    app.relock();
    let checked = support::checked_project(&app);
    let json = checked.linked_json().to_canonical_string();
    assert!(json.contains("PkgLib.Shapes.quad"));
    assert!(
        support::lean_text(&support::rendered(&app), "PkgLib.Util").contains("PkgLib.Shapes.quad")
    );
}

// This regression runs in the same GL22 harness: an actual semantic Let
// header must compare with an independently converted alpha-renamed LSE.
// Wrong binder types, out-of-scope values and unequal bodies never match.
fn let_header_comparison(checked: &lexlean::link::CheckedProject) {
    use lexlean::elaborate::expressions::{entry_signature_term, unify_closed};
    use lexlean::elaborate::resolve::LocalAlloc;
    use lexlean::elaborate::{DeclTable, Shared};
    use lexlean::ir::term::{LocalId, Term, Universe};
    use lexlean::lexicon::lse::QualifiedId;
    let module = &checked.modules["Colors"].document;
    let exports = lexlean::elaborate::semantic::exports(
        "Colors",
        &module.lean_module,
        module.semantic.as_ref().expect("semantic domain"),
        &module.import_lean,
    );
    let decls = DeclTable {
        rows: Vec::new(),
        semantic: exports,
    };
    let main = &checked.modules["Main"];
    let shared = Shared {
        path: &main.document.source_path,
        atoms: &main.atoms,
        closure: &checked.closure,
        visible: &main.visible,
        decls: &decls,
        module: "Main",
        module_prefix: "LanguageTwelve",
    };
    let mut alloc = LocalAlloc::default();
    let (_, claimed) = entry_signature_term(
        &shared,
        &mut alloc,
        &QualifiedId::parse("repro.words::letproof").expect("entry id"),
    )
    .expect("alpha-equivalent actual Let header accepted");
    let actual = decls
        .semantic_get("Colors", "letHeader")
        .expect("linked header")
        .ty(&checked.closure, &mut alloc)
        .expect("exact actual type");
    assert!(unify_closed(&actual, &claimed));
    let Term::Let {
        binder,
        value,
        body,
    } = &claimed
    else {
        panic!("actual header is a Let")
    };
    let mut wrong_type = (**binder).clone();
    wrong_type.ty = Term::Sort(Universe::Num(1));
    assert!(!unify_closed(
        &actual,
        &Term::Let {
            binder: Box::new(wrong_type),
            value: value.clone(),
            body: body.clone()
        }
    ));
    assert!(
        !unify_closed(
            &actual,
            &Term::Let {
                binder: binder.clone(),
                value: Box::new(Term::Local(binder.id)),
                body: body.clone()
            }
        ),
        "let binder is not in scope in its value"
    );
    let mut wrong_value = (**value).clone();
    let Term::NatLiteral { decimal, .. } = &mut wrong_value else {
        panic!("Nat literal value")
    };
    *decimal = "1".to_owned();
    assert!(!unify_closed(
        &actual,
        &Term::Let {
            binder: binder.clone(),
            value: Box::new(wrong_value),
            body: body.clone()
        }
    ));
    assert!(
        !unify_closed(
            &actual,
            &Term::Let {
                binder: binder.clone(),
                value: value.clone(),
                body: Box::new(Term::Local(LocalId(u64::MAX)))
            }
        ),
        "free body name cannot be captured"
    );
    // Scope-aware comparison also recurses through a containing lambda.
    let lambda = |body| Term::Lambda {
        binders: vec![lexlean::ir::term::Binder {
            id: LocalId(u64::MAX - 1),
            mode: lexlean::ir::term::BinderMode::Explicit,
            ty: Term::Sort(Universe::Num(1)),
            spelling: "outside".to_owned(),
        }],
        body: Box::new(body),
    };
    assert!(unify_closed(&lambda(actual), &lambda(claimed)));
}
