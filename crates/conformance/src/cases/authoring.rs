//! The `authoring` suite: AU-01..AU-12, the language-1.2 authoring notation
//! (SPEC.md §17.21).
//!
//! The notation is in bijection with the canonical JSON of the closed schema,
//! so the cases state that bijection (AU-01), what it buys (a module in the
//! notation is the module its data form is, AU-02), what it refuses (AU-03),
//! that it is the only way the product writes a module (AU-04), that the same
//! holds for tooling modules (AU-05), that no identity depends on which
//! form a module is written in (AU-06), that no writer of the repository owns
//! a product source (AU-07), and that the authoring API is how a program reads
//! and writes one (AU-08). Every committed module is a case's input, not a
//! sample of them.

use std::collections::BTreeSet;

use lexlean::artifact::canonical_json::Json;
use lexlean::surface::lex::lex;
use lexlean::surface::module::Kind;
use lexlean::surface::semantic::{
    parse_at, parse_module, print_at, print_module, schema_samples, Position,
};
use lexlean::{FormatRequest, Selection};

use crate::support::{self, repo_root, P};

/// Run the case for one authoring ID.
///
/// # Panics
///
/// When the case's assertion fails, or for an unwired ID.
pub fn run(id: &str) {
    match id {
        "AU-01" => round_trip(),
        "AU-02" => equivalence(),
        "AU-03" => fails_closed(),
        "AU-04" => one_surface(),
        "AU-05" => tooling_notation(),
        "AU-06" => identity_is_form_independent(),
        "AU-07" => no_other_authoring_path(),
        "AU-08" => authoring_api(),
        "AU-09" | "AU-10" | "AU-11" | "AU-12" => super::authoring_core::run(id),
        other => panic!("no authoring case is wired for {other}"),
    }
}

const SPEC: &str = "lexlean/semantic-module/2";

/// The schema tag of a tooling module.
const TOOLING_SPEC: &str = "lexlean/tooling-module/1";

/// The schema tag the notation of a core module names (§17.21.8).
const CORE_SPEC: &str = "lexlean/core-module/2";

// ---------------------------------------------------------------------------
// The committed corpus.
// ---------------------------------------------------------------------------

/// Every semantic module of a committed source, as canonical JSON, with the
/// file it is in. A module that is not well formed is a negative fixture's
/// business and is left out.
fn corpus() -> Vec<(String, Json)> {
    let root = repo_root();
    let mut modules = Vec::new();
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
        if let Ok(value) = support::try_module_json(&text) {
            if let Ok(json) = Json::parse(value.to_string().as_bytes()) {
                modules.push((path.display().to_string(), json));
            }
        }
    }
    modules
}

// ---------------------------------------------------------------------------
// AU-01.
// ---------------------------------------------------------------------------

/// A value written and read at a position is the value, and what is written
/// from what is read is what was written.
fn assert_round_trip(value: &Json, position: Position, what: &str) -> String {
    let text = print_at(value, position);
    let back = parse_at(&text, position).unwrap_or_else(|error| panic!("{what}: {error}\n{text}"));
    assert_eq!(&back, value, "{what}: read back differently\n{text}");
    assert_eq!(
        print_at(&back, position),
        text,
        "{what}: the notation is not its own canonical form"
    );
    text
}

fn round_trip() {
    // Every alternative of the four unions of the schema, with and without
    // its optional members. The samples are read from the schema, so a
    // variant added to it is sampled the day it is added.
    let samples = schema_samples();
    let mut generic = 0usize;
    for (position, label, value) in &samples {
        let text = assert_round_trip(value, *position, label);
        // A readable form is one that is not `kind{…}` spelled out.
        if text.starts_with(&format!(
            "{}{{",
            match value {
                Json::Obj(members) => match members.get("kind") {
                    Some(Json::Str(kind)) => kind.as_str(),
                    _ => "",
                },
                _ => "",
            }
        )) {
            generic += 1;
        }
    }
    assert!(
        samples.len() > 180 && generic > 30 && generic < samples.len(),
        "{} samples, {generic} written generically: the schema moved, or a readable form is lost",
        samples.len()
    );

    // A node that no readable form carries exactly is written generically.
    for (generic_text, position) in [
        (r#"var{name: let}"#, Position::Term),
        (r#"nat{value: "01"}"#, Position::Term),
        (
            r#"add{left: nat{value: "1"}, right: nat{value: "2"}, extra: true}"#,
            Position::Term,
        ),
        (
            r#"named{member: {name: Nat}, arguments: []}"#,
            Position::Type,
        ),
    ] {
        let value = lexlean::surface::generic::parse(generic_text).expect("generic notation");
        let text = assert_round_trip(&value, position, generic_text);
        assert!(text.contains('{'), "{text} is written generically");
    }

    // Every committed semantic module of language 1.2.
    let mut modules = 0usize;
    let mut declarations = 0usize;
    for (path, json) in corpus() {
        let Json::Obj(members) = &json else { continue };
        if members.get("spec") != Some(&Json::Str(SPEC.to_owned())) {
            continue;
        }
        let text = print_module(&json).unwrap_or_else(|error| panic!("{path}: {error}"));
        let back =
            parse_module(&text, SPEC).unwrap_or_else(|error| panic!("{path}: {error}\n{text}"));
        assert_eq!(back, json, "{path}: read back differently");
        assert_eq!(
            print_module(&back).expect("prints"),
            text,
            "{path}: the notation is not its own canonical form"
        );
        modules += 1;
        if let Some(Json::Arr(items)) = members.get("declarations") {
            declarations += items.len();
        }
    }
    // Anti-vacuity: the committed corpus is large, and says so.
    assert!(
        modules >= 250 && declarations >= 4_000,
        "{modules} modules, {declarations} declarations: the corpus the notation is checked on has shrunk"
    );
}

// ---------------------------------------------------------------------------
// AU-02.
// ---------------------------------------------------------------------------

/// Committed language-1.2 examples, which are written in the notation.
const EXAMPLES: [&str; 6] = [
    "language-1.2",
    "collections",
    "higher-order",
    "recursive-data",
    "models",
    "reasoning",
];

/// Every file under `directory`, by `/`-separated relative path.
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

/// Write every module of a project as `\semanticdata{…}`: the form a module
/// had before it was written in the notation.
fn convert_to_data(project: &P) {
    for entry in walkdir::WalkDir::new(project.root.join("src").as_std_path())
        .into_iter()
        .flatten()
    {
        let path = entry.path();
        if !path.to_string_lossy().ends_with(".lex.tex") {
            continue;
        }
        let text = std::fs::read_to_string(path).expect("read");
        if text.contains("\\begin{semanticmodule}") {
            std::fs::write(path, support::module_as_data(&text)).expect("write");
        }
    }
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

fn equivalence() {
    let declaration_words: BTreeSet<&str> = [
        "structure",
        "class",
        "instance",
        "inductive",
        "definition",
        "theorem",
        "artifact",
        "contract",
        "realization",
        "evidence",
        "model",
        "logic",
        "inference_rule",
        "verifier",
        "reasoner",
    ]
    .into_iter()
    .collect();
    for name in EXAMPLES {
        let data = P::copy_example(name);
        let notation = P::copy_example(name);
        convert_to_data(&data);
        // A committed module is written in the notation, and it is canonical
        // as it is: `fmt` leaves it unchanged, in the file and in the check.
        let sources: Vec<String> = walkdir::WalkDir::new(notation.root.join("src").as_std_path())
            .into_iter()
            .flatten()
            .filter(|entry| entry.path().to_string_lossy().ends_with(".lex.tex"))
            .map(|entry| entry.path().display().to_string())
            .collect();
        let before: Vec<String> = sources
            .iter()
            .map(|path| std::fs::read_to_string(path).expect("read"))
            .collect();
        for text in &before {
            assert!(
                !text.contains("\\semanticdata"),
                "{name}: a committed module is written as data"
            );
        }
        notation.fmt_check_ok();
        // The data form is the same module in text `fmt` does not write: it
        // is not canonical, and `fmt` migrates it to the notation, to exactly
        // the committed bytes.
        support::expect_code(
            &format_all(&data, true)
                .err()
                .expect("a module written as data is not canonical"),
            "LLL1003",
        );
        let migrated = P::copy_example(name);
        convert_to_data(&migrated);
        format_all(&migrated, false).expect("fmt migrates the data form");
        for (path, committed) in sources.iter().zip(&before) {
            let relative = std::path::Path::new(path)
                .strip_prefix(notation.root.as_std_path())
                .expect("under the project")
                .to_string_lossy()
                .replace('\\', "/");
            let relative = relative.as_str();
            assert_eq!(
                &migrated.read_raw(relative),
                committed,
                "{name}: fmt did not migrate {relative} to the committed notation"
            );
        }
        // The notation is canonical because `fmt` writes the same text from
        // any layout of it: collapse the layout of one module, and `fmt` is
        // what restores it.
        let (path, canonical) = sources
            .iter()
            .zip(&before)
            .find(|(_, text)| text.contains("\\begin{semanticmodule}"))
            .expect("a semantic module");
        let squeezed = support::squeezed_notation(canonical);
        assert_ne!(&squeezed, canonical, "{name}: the layout did not change");
        std::fs::write(path, &squeezed).expect("write");
        support::expect_code(
            &format_all(&notation, true)
                .err()
                .expect("another layout is not canonical"),
            "LLL1003",
        );
        format_all(&notation, false).expect("fmt writes the notation");
        assert_eq!(
            &std::fs::read_to_string(path).expect("read"),
            canonical,
            "{name}: fmt did not restore the canonical layout"
        );
        notation.fmt_check_ok();
        format_all(&notation, false).expect("fmt writes the notation");
        let after: Vec<String> = sources
            .iter()
            .map(|path| std::fs::read_to_string(path).expect("read"))
            .collect();
        assert_eq!(before, after, "{name}: fmt changed the notation");

        // The same module: the same semantic ID, and the same generated Lean
        // and LaTeX byte for byte.
        let data_checked = data.check_ok();
        let notation_checked = notation.check_ok();
        assert_eq!(
            data_checked.semantic_id, notation_checked.semantic_id,
            "{name}: the notation does not link to its data form's module"
        );
        assert_ne!(
            data_checked.source_id, notation_checked.source_id,
            "{name}: the source did not change"
        );
        let data_built = data.build_ok();
        let notation_built = notation.build_ok();
        let data_dir = data.build_dir(&data_built.build_id.expect("a build id"));
        let notation_dir = notation.build_dir(&notation_built.build_id.expect("a build id"));
        let data_files = files(&data_dir.join("modules"));
        let notation_files = files(&notation_dir.join("modules"));
        assert!(!data_files.is_empty(), "{name}: no generated files");
        assert_eq!(
            data_files, notation_files,
            "{name}: the generated Lean and LaTeX differ"
        );

        // The source maps relate each declaration to its own text in the
        // notation.
        let mut mapped = 0usize;
        for (relative, bytes) in files(&notation_dir.join("maps")) {
            let map: serde_json::Value = serde_json::from_slice(&bytes).expect("a source map");
            let Some(module) = relative
                .strip_suffix(".map.json")
                .map(|stem| format!("src/{}.lex.tex", stem.rsplit('/').next().unwrap_or(stem)))
            else {
                continue;
            };
            let Ok(source) = std::fs::read_to_string(notation.root.join(&module).as_std_path())
            else {
                continue;
            };
            for mapping in map["mappings"].as_array().expect("mappings") {
                let start =
                    usize::try_from(mapping["src_start"].as_u64().expect("start")).expect("fits");
                let end = usize::try_from(mapping["src_end"].as_u64().expect("end")).expect("fits");
                // The module's own pieces map the whole source.
                if mapping["role"] != "declaration" || (start == 0 && end == source.len()) {
                    continue;
                }
                let text = &source[start..end];
                let word = text.split(['{', ' ']).next().unwrap_or_default();
                assert!(
                    declaration_words.contains(word) && text.ends_with('}'),
                    "{name}: `{}` is not a declaration of the notation",
                    &text[..text.len().min(40)]
                );
                mapped += 1;
            }
        }
        assert!(
            mapped > 0,
            "{name}: no declaration is mapped to the notation"
        );
    }

    // Under language 1.1 the notation is refused.
    let project = P::copy_example("semantic-1.1");
    let main = project.read("src/Main.lex.tex");
    let begin = main
        .find("\\begin{semanticmodule}")
        .expect("a semantic module");
    let end = main.rfind("\\end{semanticmodule}").expect("its end");
    project.write(
        "src/Main.lex.tex",
        &format!(
            "{}\\begin{{semanticmodule}}\ntheorem{{name: t, parameters: [], statement: true, proof: decide{{}}}}\n{}",
            &main[..begin],
            &main[end..]
        ),
    );
    project.check_fails_with("LLP2003");
}

// ---------------------------------------------------------------------------
// AU-03.
// ---------------------------------------------------------------------------

const MAIN: &str = "src/Main.lex.tex";
const BEGIN: &str = "\\begin{semanticmodule}\n";
const END: &str = "\\end{semanticmodule}";

/// The body of the notation of a project's `Main`.
fn body(project: &P) -> String {
    let text = project.read_raw(MAIN);
    let start = text.find(BEGIN).expect("begin") + BEGIN.len();
    let end = text.rfind(END).expect("end");
    text[start..end].to_owned()
}

fn put_body(project: &P, body: &str) {
    let text = project.read_raw(MAIN);
    let start = text.find(BEGIN).expect("begin") + BEGIN.len();
    let end = text.rfind(END).expect("end");
    project.write(MAIN, &format!("{}{body}{}", &text[..start], &text[end..]));
}

fn fails_closed() {
    let project = P::copy_example("language-1.2");
    let original = body(&project);
    project.check_ok();

    // Malformed in each way the statement names.
    let named: [(&str, String); 5] = [
        (
            "an unclosed delimiter",
            original.replacen("definition{", "definition{ name: x, parameters: [", 1),
        ),
        (
            "a keyword used as a name",
            original.replacen("let doubled", "let let", 1),
        ),
        (
            "a literal that is not canonical",
            original.replacen("(doubledSuccessor 2)", "0x1", 1),
        ),
        (
            "a member written twice",
            original.replacen(
                "name: doubledSuccessor,",
                "name: doubledSuccessor, name: x,",
                1,
            ),
        ),
        (
            "an unknown kind",
            format!("{original}\nfrobnicate{{name: x}}\n"),
        ),
    ];
    for (what, text) in &named {
        assert_ne!(text, &original, "{what}: the plant changed nothing");
        put_body(&project, text);
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
    put_body(&project, &original);
    project.check_ok();

    // Every single-token deletion, duplication, and replacement of the
    // module is refused with a registered code or links to a module that
    // decodes; none panics.
    let tokens = lex(&original).expect("the module lexes");
    assert!(tokens.len() > 80, "{} tokens", tokens.len());
    let mut refused = 0usize;
    let mut linked = 0usize;
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
            put_body(&project, &mutant);
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
    put_body(&project, &original);

    // A numeral with a redundant leading zero is LLL1003 outside a string
    // and is string data inside one.
    put_body(
        &project,
        &original.replacen("(doubledSuccessor 2)", "(doubledSuccessor 02)", 1),
    );
    project.check_fails_with("LLL1003");
    put_body(
        &project,
        &format!(
            "{original}\ndefinition{{name: zeros, parameters: [], result: String, body: \"007\"}}\n"
        ),
    );
    project.check_ok();
}

// ---------------------------------------------------------------------------
// AU-04.
// ---------------------------------------------------------------------------

/// The `language` of the project a source belongs to: that of the nearest
/// ancestor directory with a `lexlean.toml`.
fn project_language(source: &std::path::Path) -> Option<String> {
    for directory in source.ancestors().skip(1) {
        let manifest = directory.join("lexlean.toml");
        if let Ok(text) = std::fs::read_to_string(&manifest) {
            return text
                .lines()
                .find_map(|line| line.strip_prefix("language = "))
                .map(|value| value.trim().trim_matches('"').to_owned());
        }
    }
    None
}

/// The payload of a module written as `\\semanticdata{…}` and the body of one
/// written in the notation, as the text between the module's markers.
enum Form<'a> {
    Data(&'a str),
    Notation(&'a str),
}

fn form(text: &str) -> Option<(&'static Kind, Form<'_>)> {
    let found = lexlean::surface::module::locate(text)?;
    Some(match found.data {
        Some((start, end)) => (found.kind, Form::Data(&text[start..end])),
        None => (
            found.kind,
            Form::Notation(&text[found.body.0..found.body.1]),
        ),
    })
}

/// Whether a payload written as data can be written in the notation: it is a
/// module of the schema the notation names, and reads back equal.
fn spellable(kind: &Kind, payload: &str) -> bool {
    let (Some(Ok(text)), Some(spec)) = (kind.notation_of_data(payload), (kind.spec)()) else {
        return false;
    };
    kind.data_text(&text, spec).is_ok_and(|back| {
        let (back, payload) = (
            Json::parse(back.as_bytes()),
            Json::parse(payload.as_bytes()),
        );
        back.is_ok() && back == payload
    })
}

fn one_surface() {
    let root = repo_root();
    let (mut notation, mut canonical, mut exempt_old, mut exempt_refused) = (0usize, 0usize, 0, 0);
    let mut defects = Vec::new();
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
        let Some((kind, form)) = form(&text) else {
            continue;
        };
        // `/`-separated on every host, as the repository states paths.
        let shown = path
            .strip_prefix(root.as_std_path())
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");
        // Only language 1.2 has the notation; 1.0 and 1.1 sources keep the
        // bytes they were released with.
        if project_language(path).as_deref() != Some("1.2") {
            if matches!(form, Form::Data(_)) {
                exempt_old += 1;
            }
            continue;
        }
        let refused = shown.starts_with("tests/negative/");
        match form {
            Form::Data(payload) if refused && !spellable(kind, payload) => exempt_refused += 1,
            Form::Data(_) => defects.push(format!(
                "{shown}: written as data, which the notation can say{}",
                if refused { " (a refused fixture)" } else { "" }
            )),
            Form::Notation(body) => {
                notation += 1;
                // A refused fixture may be refused for its text; a source of
                // the product is its own canonical source.
                let printed = (kind.spec)().and_then(|spec| kind.reprint(body, spec).ok());
                if body
                    .strip_prefix('\n')
                    .is_some_and(|rest| printed.as_deref() == Some(rest))
                {
                    canonical += 1;
                } else if !refused {
                    defects.push(format!("{shown}: not its own canonical source"));
                }
            }
        }
    }
    assert!(
        defects.is_empty(),
        "{} source(s) are not written in the authoring notation:\n{}",
        defects.len(),
        defects.join("\n")
    );
    // Anti-vacuity: the walk reads the product's sources, and the data form
    // it exempts is really there to be exempted.
    assert!(
        notation >= 250 && canonical >= 250,
        "{notation} sources in the notation, {canonical} canonical: the walk did not read the product"
    );
    assert!(
        exempt_old > 0 && exempt_refused > 0,
        "{exempt_old} sources under language 1.0 and 1.1, {exempt_refused} refused fixtures written as data: the exemptions are not exercised"
    );
}

// ---------------------------------------------------------------------------
// AU-05.
// ---------------------------------------------------------------------------

/// The committed tooling fixtures, and the embedded adapter project, whose
/// modules are written in the notation.
const TOOLING_FIXTURES: [&str; 3] = ["commands", "core", "traversal"];

/// The linked project of a tooling fixture copy, as the compiler embeds it.
fn tooling_checked(project: &P) -> lexlean::link::CheckedProject {
    let inner = lexlean::project::Project::load(&project.root.join("lexlean.toml"))
        .expect("the project loads")
        .with_tooling();
    let (lock, packages) = lexlean::lock::compute_lock(&inner, false).expect("the lock computes");
    lexlean::link::check_project(
        &inner,
        &lexlean::project::Selection::Entrypoints,
        &lock,
        packages,
    )
    .expect("the tooling project links")
}

/// Every tooling module of a committed source, as canonical JSON, with the
/// file it is in.
fn tooling_corpus() -> Vec<(String, Json)> {
    let root = repo_root();
    let mut modules = Vec::new();
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
        if !text.contains("\\begin{toolingmodule}") {
            continue;
        }
        let value = support::try_module_json(&text)
            .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        modules.push((
            path.display().to_string(),
            Json::parse(value.to_string().as_bytes()).expect("canonical JSON"),
        ));
    }
    modules
}

fn tooling_notation() {
    use lexlean::surface::tooling::{declaration_spans, parse_module, print_module};

    // The notation and the canonical JSON are in bijection for every
    // committed tooling module, and each declaration has the span of its own
    // text.
    let (mut modules, mut declarations) = (0usize, 0usize);
    for (path, json) in tooling_corpus() {
        let text = print_module(&json).unwrap_or_else(|error| panic!("{path}: {error}"));
        let back = parse_module(&text, TOOLING_SPEC)
            .unwrap_or_else(|error| panic!("{path}: {error}\n{}", &text[..text.len().min(400)]));
        assert_eq!(back, json, "{path}: read back differently");
        assert_eq!(
            print_module(&back).expect("prints"),
            text,
            "{path}: the notation is not its own canonical form"
        );
        let spans = declaration_spans(&text).unwrap_or_else(|error| panic!("{path}: {error}"));
        let Json::Obj(members) = &json else {
            panic!("{path}: a module is an object");
        };
        let Some(Json::Arr(items)) = members.get("declarations") else {
            panic!("{path}: a module has declarations");
        };
        assert_eq!(
            spans.len(),
            items.len(),
            "{path}: a span for each declaration"
        );
        for ((start, end), item) in spans.iter().zip(items) {
            let one = lexlean::surface::generic::parse(&text[*start..*end])
                .unwrap_or_else(|error| panic!("{path}: a span is not a declaration: {error}"));
            assert_eq!(&one, item, "{path}: a span is not its own declaration");
        }
        modules += 1;
        declarations += items.len();
    }
    assert!(
        modules >= 4 && declarations >= 380,
        "{modules} modules, {declarations} declarations: the tooling corpus has shrunk"
    );

    for name in TOOLING_FIXTURES {
        let notation = P::tooling(name);
        let data = P::tooling(name);
        let committed = notation.read_raw("src/Main.lex.tex");
        assert!(
            !committed.contains("\\toolingdata"),
            "{name}: a committed tooling module is written as data"
        );
        data.write("src/Main.lex.tex", &support::module_as_data(&committed));

        // `fmt` writes the data form as exactly the committed notation, and
        // the notation as itself.
        let from_data = tooling_checked(&data);
        let from_notation = tooling_checked(&notation);
        assert_eq!(
            lexlean::fmt::canonical_source(&from_data.modules["Main"], &from_data.closure)
                .expect("fmt writes the data form"),
            committed,
            "{name}: fmt did not write the data form as the committed notation"
        );
        assert_eq!(
            lexlean::fmt::canonical_source(&from_notation.modules["Main"], &from_notation.closure)
                .expect("fmt writes the notation"),
            committed,
            "{name}: fmt changed the notation"
        );

        // The same module: the same semantic ID, Lean, and LaTeX, and the
        // source map relates each declaration to its text in the notation.
        assert_eq!(
            from_data.semantic_id, from_notation.semantic_id,
            "{name}: the notation does not link to its data form's module"
        );
        assert_ne!(
            from_data.source_id, from_notation.source_id,
            "{name}: the source did not change"
        );
        let rendered_data = crate::tooling::render(&data.root.join("lexlean.toml"))
            .unwrap_or_else(|error| panic!("{name}: {error}"));
        let rendered_notation = crate::tooling::render(&notation.root.join("lexlean.toml"))
            .unwrap_or_else(|error| panic!("{name}: {error}"));
        assert_eq!(rendered_data.modules.len(), rendered_notation.modules.len());
        let mut mapped = 0usize;
        for (left, right) in rendered_data.modules.iter().zip(&rendered_notation.modules) {
            assert_eq!(
                left.lean_text, right.lean_text,
                "{name}: the generated Lean differs"
            );
            assert_eq!(
                left.tex_text, right.tex_text,
                "{name}: the generated LaTeX differs"
            );
            for mapping in &right.map.mappings {
                let is_declaration = right
                    .map
                    .nodes
                    .iter()
                    .any(|node| node.id == mapping.node && node.kind == "tooling-declaration");
                let Some((start, end)) = mapping.src_range else {
                    continue;
                };
                if !is_declaration || mapping.artifact != 0 {
                    continue;
                }
                let text = &committed[start..end];
                lexlean::surface::generic::parse(text).unwrap_or_else(|error| {
                    panic!("{name}: a mapped piece is not a declaration of the notation: {error}")
                });
                mapped += 1;
            }
        }
        assert!(
            mapped > 0,
            "{name}: no declaration is mapped to the notation"
        );
    }

    // Malformed notation, and every single-token deletion and duplication of
    // a committed module, fails with a position or decodes: nothing panics.
    let committed = P::tooling("core").read_raw("src/Main.lex.tex");
    let begin = committed.find("\\begin{toolingmodule}\n").expect("begin")
        + "\\begin{toolingmodule}\n".len();
    let end = committed.rfind("\\end{toolingmodule}").expect("end");
    let body = &committed[begin..end];
    for text in [
        "",
        "[]",
        "{",
        "{spec: x, declarations: []}",
        "{declarations: [type{]}",
    ] {
        let error = parse_module(text, TOOLING_SPEC).expect_err("malformed");
        assert!(
            error.offset <= text.len(),
            "{text:?}: the position is in the text"
        );
    }
    let pin = lexlean::ir::tooling::AuthorityPin {
        lean_version: lexlean::LEAN_VERSION,
        lean_githash: lexlean::LEAN_GITHASH,
    };
    let tokens = lex(body).expect("the committed notation lexes");
    let (mut refused, mut decoded) = (0usize, 0usize);
    for token in &tokens {
        let (start, stop) = (token.start, token.end);
        for mutated in [
            format!("{}{}", &body[..start], &body[stop..]),
            format!("{}{}", &body[..stop], &body[start..]),
        ] {
            match parse_module(&mutated, TOOLING_SPEC) {
                Err(error) => {
                    assert!(error.offset <= mutated.len(), "the position is in the text");
                    refused += 1;
                }
                Ok(json) => {
                    // What reads is decoded by the decoder the link stage
                    // uses: it refuses with a registered code or accepts.
                    match lexlean::ir::tooling::ToolingModule::parse(
                        &json.to_canonical_string(),
                        lexlean::LANGUAGE_1_2,
                        pin,
                    ) {
                        Ok(_) => decoded += 1,
                        Err(failure) => {
                            assert!(!failure.reason.is_empty());
                            refused += 1;
                        }
                    }
                }
            }
        }
    }
    assert!(
        refused > 1_000 && decoded > 0,
        "{refused} refused, {decoded} decoded: the sweep did not exercise both outcomes"
    );
}

// ---------------------------------------------------------------------------
// AU-06.
// ---------------------------------------------------------------------------

fn identity_is_form_independent() {
    // The embedded tree holds modules written in the notation (the adapter's
    // tooling module), and what each contributes to an identity is its data
    // form.
    let mut in_notation = 0usize;
    let mut rows: Vec<(String, Vec<u8>)> = Vec::new();
    let mut as_data: Vec<(String, Vec<u8>)> = Vec::new();
    for (path, bytes) in lexlean::embedded::FILES {
        rows.push(((*path).to_owned(), bytes.to_vec()));
        let contributed = lexlean::identity_bytes(path, bytes);
        let text = std::str::from_utf8(bytes)
            .ok()
            .filter(|_| path.ends_with(".lex.tex"));
        match text.filter(|text| support::is_notation_module(text)) {
            Some(text) => {
                assert_eq!(
                    contributed.as_ref(),
                    support::module_as_data(text).as_bytes(),
                    "{path}: a notation module contributes its data form"
                );
                in_notation += 1;
                as_data.push((
                    (*path).to_owned(),
                    support::module_as_data(text).into_bytes(),
                ));
            }
            None => {
                assert_eq!(
                    contributed.as_ref(),
                    *bytes,
                    "{path}: any other file, as it is"
                );
                as_data.push(((*path).to_owned(), bytes.to_vec()));
            }
        }
    }
    assert!(
        in_notation > 0,
        "no embedded module is written in the notation: the case would pass vacuously"
    );
    let digest = |rows: &[(String, Vec<u8>)]| {
        let borrowed: Vec<(&str, &[u8])> = rows
            .iter()
            .map(|(path, bytes)| (path.as_str(), bytes.as_slice()))
            .collect();
        support::identity_tree_digest(&borrowed)
    };
    let id = lexlean::compiler_semantics_id_for(lexlean::LATEST_LANGUAGE_VERSION);
    // The tree as committed, and the same tree with every such module
    // written as data, have the one identity.
    assert_eq!(digest(&rows), id, "the committed tree is the identity");
    assert_eq!(
        digest(&as_data),
        id,
        "the tree with every module written as data is the same identity"
    );
    // A change to a module's meaning is a change of identity, in either form.
    let adapter = "language/lcnf-1.2/tooling/src/Extract.lex.tex";
    let at = rows
        .iter()
        .position(|(path, _)| path == adapter)
        .expect("the embedded adapter source");
    let text = std::str::from_utf8(&rows[at].1).expect("UTF-8");
    let mut module = support::module_json(text);
    module["declarations"]
        .as_array_mut()
        .expect("declarations")
        .pop()
        .expect("a declaration");
    let mut planted = rows.clone();
    planted[at].1 = support::with_module_json(text, &module).into_bytes();
    assert_ne!(
        digest(&planted),
        id,
        "a removed declaration changes the identity"
    );
    let mut planted_data = as_data.clone();
    planted_data[at].1 =
        support::module_as_data(std::str::from_utf8(&planted[at].1).expect("UTF-8")).into_bytes();
    assert_eq!(
        digest(&planted),
        digest(&planted_data),
        "the changed module has one identity in either form"
    );

    // The identity a preservation record states for the native library is
    // over the same data forms: the library's modules are written in the
    // notation, and the record is the one committed before they were.
    let source_of = |rows: &[(String, Vec<u8>)], path: &str| -> String {
        let (_, bytes) = rows
            .iter()
            .find(|(candidate, _)| candidate == path)
            .unwrap_or_else(|| panic!("{path}: not in the embedded tree"));
        String::from_utf8(bytes.clone()).expect("UTF-8")
    };
    let as_committed =
        lexlean::production::preserve::library_identity_of(&|path| source_of(&rows, path));
    let as_written_as_data =
        lexlean::production::preserve::library_identity_of(&|path| source_of(&as_data, path));
    assert_eq!(
        as_committed, as_written_as_data,
        "the library identity of a record depends on the form its sources are written in"
    );
    let record: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(
            repo_root()
                .join("examples/models/expected/verify/preserve/preservation.json")
                .as_std_path(),
        )
        .expect("the committed preservation record of models"),
    )
    .expect("a JSON record");
    assert_eq!(
        record["library"]["registry_sha256"].as_str(),
        Some(as_committed.as_str()),
        "the committed record states the library identity of the committed library"
    );
}

// ---------------------------------------------------------------------------
// AU-07.
// ---------------------------------------------------------------------------

/// The sources a writer of the repository emits from an input that is not a
/// source: the fixture modules of the `compiler` project, from the committed
/// fixtures, requests, and certificates, and the extraction adapter's module,
/// from the registry the pinned Lean answers. No other source is emitted.
const GENERATED_RECORDS: [&str; 5] = [
    "compiler/src/TargetFixtures.lex.tex",
    "compiler/src/ReasoningFixtures.lex.tex",
    "compiler/src/GnafFixtures.lex.tex",
    "compiler/src/GnafCertificates.lex.tex",
    "language/lcnf-1.2/tooling/src/Extract.lex.tex",
];

/// The trees whose sources are the product's.
const PRODUCT_TREES: [&str; 3] = ["compiler/", "language/", "examples/"];

fn no_other_authoring_path() {
    let root = repo_root();
    // Every file any writer of the repository emits: the generators that
    // `cargo xtask check-calculus` compares and rewrites.
    let mut emitted: BTreeSet<String> = BTreeSet::new();
    emitted.extend(crate::calculus::files().into_keys());
    emitted.extend(crate::gnaf::files().into_keys());
    emitted.extend(crate::certification::files().into_keys());
    emitted.extend(
        crate::lcnf_registry::generated(root.as_std_path())
            .expect("the committed registry")
            .into_keys(),
    );
    let sources: BTreeSet<&str> = emitted
        .iter()
        .map(String::as_str)
        .filter(|path| {
            path.ends_with(".lex.tex") && PRODUCT_TREES.iter().any(|tree| path.starts_with(tree))
        })
        .collect();
    let records: BTreeSet<&str> = GENERATED_RECORDS.into_iter().collect();
    assert_eq!(
        sources, records,
        "the writers of the repository emit exactly the named records and no other product source"
    );
    // Anti-vacuity: the writers emit the files they are known to emit, and
    // the product trees hold sources that no writer emits.
    assert!(
        emitted.len() > 500
            && emitted.contains("compiler/fixtures/sum-to.json")
            && emitted
                .iter()
                .any(|path| path.starts_with("compiler/rust/")),
        "{} files: the writers were not enumerated",
        emitted.len()
    );
    let mut owned = 0usize;
    for entry in walkdir::WalkDir::new(root.as_std_path())
        .into_iter()
        .filter_entry(|entry| {
            let name = entry.file_name().to_string_lossy();
            name != "target" && name != ".git" && name != ".lexlean"
        })
        .flatten()
    {
        let relative = entry
            .path()
            .strip_prefix(root.as_std_path())
            .unwrap_or(entry.path())
            .to_string_lossy()
            .replace('\\', "/");
        if relative.ends_with(".lex.tex")
            && PRODUCT_TREES.iter().any(|tree| relative.starts_with(tree))
        {
            if records.contains(relative.as_str()) {
                continue;
            }
            owned += 1;
        }
    }
    assert!(
        owned >= 150,
        "{owned} product sources are owned by no writer: the walk did not read the product"
    );
    for record in GENERATED_RECORDS {
        assert!(
            root.join(record).as_std_path().is_file(),
            "{record} is a committed source"
        );
    }
}

// ---------------------------------------------------------------------------
// AU-08.
// ---------------------------------------------------------------------------

/// The size of a source above which AU-08 does not read the module as a value.
const LARGEST_READ_AS_VALUE: usize = 12 * 1024 * 1024;

fn authoring_api() {
    use lexlean::sdk::{
        assemble, declaration_spans, is_notation, module_kind, read_module, to_data_form,
        to_notation, write_module, ModuleKind, SourceHeader,
    };

    let root = repo_root();
    let (mut semantic, mut tooling, mut core) = (0usize, 0usize, 0usize);
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
        // Reading a module of tens of megabytes as a value takes minutes in a
        // debug build. The functions are the same code that runs on every
        // other module, and AU-09 reads these through the typed notation.
        if text.len() > LARGEST_READ_AS_VALUE {
            continue;
        }
        let Some(kind) = module_kind(&text) else {
            // Not a module of these kinds: the API says so, and reads nothing.
            assert!(read_module(&text).is_err(), "{}", path.display());
            continue;
        };
        // A refused fixture may not be a module; the rest are.
        let Ok(data) = read_module(&text) else {
            assert!(
                path.to_string_lossy()
                    .replace('\\', "/")
                    .contains("/negative/"),
                "{}: a committed module the API cannot read",
                path.display()
            );
            continue;
        };
        let shown = path.display();
        // A module under a tag the notation does not name (language 1.1) has
        // no notation: the API refuses to write one, and reads it as data.
        if data["spec"] != SPEC && data["spec"] != TOOLING_SPEC && data["spec"] != CORE_SPEC {
            assert!(
                to_notation(&text).is_err(),
                "{shown}: a notation was written"
            );
            assert_eq!(
                read_module(&to_data_form(&text).expect("the data form")).expect("reads"),
                data,
                "{shown}"
            );
            continue;
        }
        // The module is what the notation module reads to, in either form, and
        // the conversions are the identity on a canonical source.
        let notation = to_notation(&text).unwrap_or_else(|error| panic!("{shown}: {error}"));
        let as_data = to_data_form(&text).unwrap_or_else(|error| panic!("{shown}: {error}"));
        assert_eq!(read_module(&notation).expect("reads"), data, "{shown}");
        assert_eq!(read_module(&as_data).expect("reads"), data, "{shown}");
        assert!(is_notation(&notation), "{shown}");
        assert!(
            !is_notation(&as_data),
            "{shown}: the data form is not the notation"
        );
        assert_eq!(
            to_notation(&notation).expect("idempotent"),
            notation,
            "{shown}"
        );
        assert_eq!(
            to_notation(&as_data).expect("converts"),
            notation,
            "{shown}"
        );
        assert_eq!(
            to_data_form(&notation).expect("converts"),
            as_data,
            "{shown}"
        );
        // A rewrite with the same data is a canonical source in the form the
        // source has.
        assert_eq!(
            write_module(&notation, &data).expect("rewrites"),
            notation,
            "{shown}"
        );
        assert_eq!(
            write_module(&as_data, &data).expect("rewrites"),
            as_data,
            "{shown}"
        );
        // A source of the notation has a span for each declaration.
        let spans = declaration_spans(&notation)
            .unwrap_or_else(|error| panic!("{shown}: {error}"))
            .expect("a notation has spans");
        assert_eq!(
            spans.len(),
            data["declarations"].as_array().map_or(0, Vec::len),
            "{shown}"
        );
        assert_eq!(declaration_spans(&as_data).expect("spans"), None, "{shown}");
        match kind {
            ModuleKind::Semantic => semantic += 1,
            ModuleKind::Tooling => tooling += 1,
            ModuleKind::Core => core += 1,
        }
        // Assembled from its header and data, a source of the standard shape is
        // the canonical source: the header is what the source already says.
        let lines: Vec<&str> = notation.lines().collect();
        let name = lines
            .first()
            .and_then(|line| line.strip_prefix("\\begin{lexlean}{"))
            .and_then(|rest| rest.strip_suffix('}'));
        let glossaries: Vec<&str> = lines
            .iter()
            .filter_map(|line| line.strip_prefix("\\useglossary{")?.strip_suffix('}'))
            .collect();
        let imports: Vec<&str> = lines
            .iter()
            .filter_map(|line| line.strip_prefix("\\importmodule{")?.strip_suffix('}'))
            .collect();
        let title = lines
            .iter()
            .find_map(|line| line.strip_prefix("\\title{")?.strip_suffix('}'));
        if let (Some(name), Some(title)) = (name, title) {
            let header = SourceHeader {
                name,
                glossaries: &glossaries,
                imports: &imports,
                title,
            };
            let assembled =
                assemble(&header, kind, &data).unwrap_or_else(|error| panic!("{shown}: {error}"));
            // The header lines of the standard shape come in this order.
            let head_end = notation
                .find(&format!(
                    "\\begin{{{}}}",
                    match kind {
                        ModuleKind::Semantic => "semanticmodule",
                        ModuleKind::Tooling => "toolingmodule",
                        ModuleKind::Core => "coremodule",
                    }
                ))
                .expect("the environment");
            let standard = notation[..head_end].lines().all(|line| {
                line.starts_with("\\begin{lexlean}")
                    || line.starts_with("\\useglossary{")
                    || line.starts_with("\\importmodule{")
                    || line.starts_with("\\title{")
            }) && notation.ends_with("\\end{lexlean}\n");
            if standard && glossaries_precede_imports(&notation) {
                assert_eq!(
                    assembled, notation,
                    "{shown}: assembled from its header and data"
                );
            }
        }
    }
    assert!(
        semantic >= 250 && tooling >= 4 && core >= 20,
        "{semantic} semantic, {tooling} tooling and {core} core sources: the API was not exercised on the product"
    );

    // What is not a module, and what is malformed, is an error with a
    // position in the source, never a panic.
    for text in [
        "",
        "\\begin{lexlean}{M}\n\\end{lexlean}\n",
        "\\begin{semanticmodule}",
        "\\begin{semanticmodule}\n\\end{semanticmodule}\n",
        "\\begin{semanticmodule}\ntheorem{\n\\end{semanticmodule}\n",
        "\\begin{toolingmodule}\n{spec: x}\n\\end{toolingmodule}\n",
        "\\begin{semanticmodule}\n\\semanticdata{not json}\n\\end{semanticmodule}\n",
    ] {
        match read_module(text) {
            Err(error) => assert!(
                error.offset.is_none_or(|offset| offset <= text.len()),
                "{text:?}: {error}"
            ),
            Ok(value) => assert!(
                text.contains("\\begin{semanticmodule}\n\\end{semanticmodule}"),
                "{text:?} read as {value}"
            ),
        }
    }
    assert!(assemble(
        &SourceHeader {
            name: "M",
            glossaries: &[],
            imports: &[],
            title: "T"
        },
        ModuleKind::Semantic,
        &serde_json::json!(3)
    )
    .is_err());
}

/// Whether every `\useglossary` line of a source precedes its first
/// `\importmodule`, the order [`lexlean::sdk::assemble`] writes them in.
fn glossaries_precede_imports(text: &str) -> bool {
    let first_import = text.find("\\importmodule{");
    let last_glossary = text.rfind("\\useglossary{");
    match (first_import, last_glossary) {
        (Some(import), Some(glossary)) => glossary < import,
        _ => true,
    }
}
