//! Normative text as LexLean source (`NS-01`..`NS-04`): the `spec/` project
//! and the projection of its register modules to `model/*.toml`.

use std::ops::Range;
use std::path::Path;

use repo_model::project::{project, SOURCE_DIR};

use crate::support::{self, repo_root, P};

/// Run the case for one ID.
pub fn run(id: &str) {
    match id {
        "NS-01" => specification_project(),
        "NS-02" => projection_is_the_model(),
        "NS-03" => drift_is_refused(),
        "NS-04" => projector_fails_closed(),
        "NS-05" => specification_text(),
        "NS-06" => source_authority(),
        "NS-07" => chapter_editing(),
        other => panic!("no conformance case is wired for {other}"),
    }
}

fn root_file(relative: &str) -> String {
    std::fs::read_to_string(repo_root().join(relative).as_std_path())
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
}

/// The files of the source directory, sorted.
fn source_files(directory: &Path) -> Vec<String> {
    let mut files: Vec<String> = std::fs::read_dir(directory)
        .expect("source directory")
        .map(|entry| {
            entry
                .expect("entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    files.sort();
    files
}

// §17.21, §27.5: the project is a member of the workspace, every module is a
// semantic module in the notation, and it is canonical, locked, checked, built
// and, under the pinned toolchain, verified.
fn specification_project() {
    let workspace = root_file("lexlean-workspace.toml");
    let members: toml::Value = workspace.parse().expect("workspace file");
    let members: Vec<&str> = members["members"]
        .as_array()
        .expect("members")
        .iter()
        .map(|member| member.as_str().expect("member"))
        .collect();
    assert!(members.contains(&"spec"), "`spec` is a workspace member");

    let directory = repo_root().join("spec/src");
    let files = source_files(directory.as_std_path());
    assert!(
        files.len() > 30,
        "anti-vacuity: the project holds the register modules, found {}",
        files.len()
    );
    for file in &files {
        let text = root_file(&format!("spec/src/{file}"));
        assert!(
            lexlean::sdk::is_notation(&text)
                && lexlean::sdk::module_kind(&text) == Some(lexlean::sdk::ModuleKind::Semantic),
            "{file}: a module of the specification project is a semantic module in the authoring notation"
        );
    }
    // Each module is titled by its own glossary entry (`spec/glossary/spec.titles`),
    // never by the template title the SDK's writer puts on a module it assembles:
    // a title is part of the canonical document, and a reset passes every other
    // check of the project.
    for file in &files {
        let module = file.trim_end_matches(".lex.tex");
        let text = root_file(&format!("spec/src/{file}"));
        let title = text
            .lines()
            .find_map(|line| line.strip_prefix("\\title{")?.strip_suffix('}'))
            .unwrap_or_else(|| panic!("{file}: a module states its title"));
        let entry: toml::Value = root_file(&format!(
            "spec/glossary/spec.titles/entries/{}.toml",
            module.to_lowercase()
        ))
        .parse()
        .unwrap_or_else(|error| panic!("{file}: its glossary entry: {error}"));
        assert_eq!(
            entry["form"][0]["surface"].as_str(),
            Some(title),
            "{file}: the title is the surface of the module's own glossary entry"
        );
        assert!(
            text.contains("\\useglossary{spec.titles@1.0.0}"),
            "{file}: the module uses the glossary of its title"
        );
        assert_ne!(
            title, "Natural number addition",
            "{file}: the title is not the template's"
        );
    }
    let project_file: toml::Value = root_file("spec/lexlean.toml")
        .parse()
        .expect("project file");
    let entrypoints: Vec<String> = project_file["entrypoints"]
        .as_array()
        .expect("entrypoints")
        .iter()
        .map(|entry| entry.as_str().expect("entry").to_owned())
        .collect();
    let expected: Vec<String> = files.iter().map(|file| format!("src/{file}")).collect();
    assert_eq!(
        entrypoints, expected,
        "every module of the project is an entrypoint"
    );

    let project = P::specification();
    project.fmt_check_ok();
    project
        .engine()
        .lock(lexlean::LockRequest {
            check_only: true,
            allow_network: false,
        })
        .expect("the lock is current");
    let checked = project.check_ok();
    assert_eq!(checked.units.len(), files.len(), "every module is checked");
    let built = project.build_ok();
    assert_eq!(built.units.len(), files.len(), "every module is built");
    if support::lean_backed("NS-01") {
        let verified = project
            .engine()
            .verify(lexlean::VerifyRequest {
                selection: lexlean::Selection::Entrypoints,
            })
            .expect("the specification project verifies under the pinned toolchain");
        assert_eq!(
            verified.units.len(),
            files.len(),
            "every module is verified"
        );
    }
}

// §27.5: the model files and the two documents regenerate from the sources.
fn projection_is_the_model() {
    let root = repo_root();
    let projection = project(root.as_std_path()).expect("the sources project");
    let model = repo_model::Model::from_texts(root.as_std_path(), &projection)
        .expect("the projection parses as the model");
    assert!(
        model.ids.id.len() > 400 && !model.errors.error.is_empty(),
        "anti-vacuity: the projection holds the register and the registry"
    );
    for (path, projected) in projection.files() {
        assert_eq!(
            root_file(path),
            projected,
            "{path} is the projection of {SOURCE_DIR}"
        );
    }
    assert_eq!(
        root_file(repo_model::codegen::CONFORMANCE_PATH),
        repo_model::codegen::render_conformance(&model),
        "CONFORMANCE.md is the projection of the register modules"
    );
    assert_eq!(
        root_file(repo_model::codegen::ERRORS_PATH),
        repo_model::codegen::render_errors(&model),
        "ERRORS.md is the projection of the registry module"
    );
}

/// A scratch root holding the files a projection reads, to plant defects in.
struct Scratch {
    directory: tempfile::TempDir,
}

impl Scratch {
    fn new() -> Self {
        let directory = tempfile::Builder::new()
            .prefix("lexlean-ns-case-")
            .tempdir()
            .expect("tempdir");
        let root = directory.path();
        std::fs::create_dir_all(root.join(SOURCE_DIR)).expect("source directory");
        std::fs::create_dir_all(root.join("model")).expect("model directory");
        let real = repo_root();
        for file in source_files(real.join(SOURCE_DIR).as_std_path()) {
            std::fs::copy(
                real.join(SOURCE_DIR).join(&file).as_std_path(),
                root.join(SOURCE_DIR).join(&file),
            )
            .expect("copy source");
        }
        std::fs::copy(real.join("SPEC.md").as_std_path(), root.join("SPEC.md"))
            .expect("copy SPEC.md");
        for file in ["ids", "errors", "ledger", "authorities"] {
            std::fs::copy(
                real.join(format!("model/{file}.toml")).as_std_path(),
                root.join(format!("model/{file}.toml")),
            )
            .expect("copy model");
        }
        Self { directory }
    }

    fn root(&self) -> &Path {
        self.directory.path()
    }

    fn source(&self, file: &str) -> std::path::PathBuf {
        self.root().join(SOURCE_DIR).join(file)
    }

    /// Replace `from` with `to` once in a source or model file.
    fn plant(&self, relative: &str, from: &str, to: &str) {
        let path = self.root().join(relative);
        let text = std::fs::read_to_string(&path).expect("file");
        assert!(text.contains(from), "{relative}: the plant's anchor exists");
        std::fs::write(path, text.replacen(from, to, 1)).expect("write");
    }
}

fn expect_refusal(scratch: &Scratch, needles: &[&str]) {
    let error = project(scratch.root())
        .err()
        .map(|error| error.to_string())
        .expect("the planted source is refused");
    for needle in needles {
        assert!(error.contains(needle), "`{needle}` is not in: {error}");
    }
}

// R1: a model file that is not its projection, a source edit that is not
// projected, and a section reference that differs from the table each fail,
// and the committed tree passes.
fn drift_is_refused() {
    let real = repo_root();
    let committed = project(real.as_std_path()).expect("the sources project");
    committed
        .check_committed(real.as_std_path())
        .expect("the committed tree passes");

    // A hand edit of a model file.
    let scratch = Scratch::new();
    scratch.plant("model/errors.toml", "exit = 1", "exit = 2");
    let error = project(scratch.root())
        .expect("sources unchanged")
        .check_committed(scratch.root())
        .expect_err("a hand-edited model file is refused")
        .to_string();
    assert!(
        error.contains("R1") && error.contains("model/errors.toml") && error.contains("NS-03"),
        "{error}"
    );

    // A hand edit of SPEC.md.
    let scratch = Scratch::new();
    scratch.plant(
        "SPEC.md",
        "The repository, crate, executable, metadata, and licenses",
        "The repository, crate, executable, metadata, and some licenses",
    );
    let error = project(scratch.root())
        .expect("sources unchanged")
        .check_committed(scratch.root())
        .expect_err("a hand-edited SPEC.md is refused")
        .to_string();
    assert!(
        error.contains("R1") && error.contains("SPEC.md") && error.contains("NS-03"),
        "{error}"
    );

    // A source edit that is not projected.
    let scratch = Scratch::new();
    scratch.plant(
        "spec/src/IdsRP.lex.tex",
        "The repository, crate, executable, metadata, and licenses",
        "The repository, crate, executable, metadata, and some licenses",
    );
    let error = project(scratch.root())
        .expect("the edited source projects")
        .check_committed(scratch.root())
        .expect_err("an unprojected source edit is refused")
        .to_string();
    assert!(
        error.contains("R1") && error.contains("model/ids.toml") && error.contains("SPEC.md"),
        "{error}"
    );

    // A section reference of the table that differs from the source.
    let table: Vec<(String, String)> = support::spec_text()
        .split("## 31. Complete conformance-ID registry")
        .nth(1)
        .expect("§31")
        .lines()
        .filter_map(|line| {
            let cells: Vec<&str> = line.strip_prefix("| `")?.split(" | ").collect();
            (cells.len() == 4).then(|| {
                (
                    cells[0].trim_end_matches('`').to_owned(),
                    cells[3].trim_end_matches(" |").to_owned(),
                )
            })
        })
        .collect();
    assert!(table.len() > 400, "anti-vacuity: the table is read");
    assert_eq!(
        committed.first_section_difference(&table),
        None,
        "the committed table cites its sources' sections"
    );
    let mut skewed = table.clone();
    skewed[0].1.push_str(", §1");
    let difference = committed
        .first_section_difference(&skewed)
        .expect("a skewed cell is found");
    assert!(
        difference.contains("RP-01") && difference.contains("NS-03"),
        "{difference}"
    );
    let scratch = Scratch::new();
    scratch.plant(
        "spec/src/IdsRP.lex.tex",
        "sections := \"§2\"",
        "sections := \"§3\"",
    );
    let planted = project(scratch.root()).expect("the edited source projects");
    assert!(
        planted.first_section_difference(&table).is_some(),
        "a source whose section reference differs from the table is found"
    );
}

// R5, NS-04: the projector refuses what it does not read, naming the file
// and, for a declaration, its position.
fn projector_fails_closed() {
    project(repo_root().as_std_path()).expect("the committed modules project");
    let ids = "spec/src/IdsRP.lex.tex";
    let record_start = "definition{\n  name: rp01,";

    // A declaration kind the projector does not read.
    let scratch = Scratch::new();
    let extra = "structure{name: Extra, type_parameters: [], parameters: [], fields: []}\n\n";
    scratch.plant(ids, record_start, &format!("{extra}{record_start}"));
    expect_refusal(
        &scratch,
        &["IdsRP.lex.tex", "declaration 1 `Extra`", "NS-04"],
    );

    // A term kind where a literal is read.
    let scratch = Scratch::new();
    scratch.plant(
        ids,
        "level := \"build\", suite := \"repository\", statement := \"The repository, crate",
        "level := (if true then \"build\" else \"build\"), suite := \"repository\", statement := \"The repository, crate",
    );
    expect_refusal(
        &scratch,
        &["IdsRP.lex.tex", "declaration 1 `rp01`", "field `level`"],
    );

    // A field missing, and fields out of order.
    let scratch = Scratch::new();
    scratch.plant(ids, ", sections := \"§2\"}", "}");
    expect_refusal(&scratch, &["declaration 1 `rp01`", "fields"]);
    let scratch = Scratch::new();
    scratch.plant(
        ids,
        "level := \"build\", suite := \"repository\", statement := \"The repository, crate",
        "suite := \"repository\", level := \"build\", statement := \"The repository, crate",
    );
    expect_refusal(&scratch, &["declaration 1 `rp01`", "out of order"]);

    // A name that is not the row's.
    let scratch = Scratch::new();
    scratch.plant(ids, record_start, "definition{\n  name: rp99,");
    expect_refusal(
        &scratch,
        &["declaration 1 `rp99`", "not named for its row `RP-01`"],
    );

    // A file that belongs to no part of the projection.
    let scratch = Scratch::new();
    std::fs::copy(
        scratch.source("Order.lex.tex"),
        scratch.source("Extra.lex.tex"),
    )
    .expect("copy");
    expect_refusal(&scratch, &["Extra.lex.tex", "belongs to no part"]);

    // A family listed twice, and a family with no module.
    let scratch = Scratch::new();
    scratch.plant(
        "spec/src/Order.lex.tex",
        "(cons \"CF\"",
        "(cons \"RP\" (cons \"CF\"",
    );
    scratch.plant("spec/src/Order.lex.tex", "(nil String)", "(nil String))");
    expect_refusal(&scratch, &["Order.lex.tex", "listed once"]);

    // A duplicate identifier in a module, and a duplicate code.
    let scratch = Scratch::new();
    let text = std::fs::read_to_string(scratch.source("IdsRP.lex.tex")).expect("module");
    let start = text.find("definition{").expect("first record");
    let end = text[start..].find("\n}\n").expect("end") + start + 3;
    let record = &text[start..end];
    scratch.plant(ids, record, &format!("{record}\n{record}"));
    expect_refusal(
        &scratch,
        &["IdsRP.lex.tex", "declaration 2 `rp01`", "registered twice"],
    );
    let scratch = Scratch::new();
    let text = std::fs::read_to_string(scratch.source("Errors.lex.tex")).expect("module");
    let start = text.find("definition{").expect("first record");
    let end = text[start..].find("\n}\n").expect("end") + start + 3;
    let record = &text[start..end];
    scratch.plant(
        "spec/src/Errors.lex.tex",
        record,
        &format!("{record}\n{record}"),
    );
    expect_refusal(&scratch, &["Errors.lex.tex", "registered twice"]);

    // A module in the data form, and malformed notation.
    let scratch = Scratch::new();
    let text = std::fs::read_to_string(scratch.source("Order.lex.tex")).expect("module");
    std::fs::write(
        scratch.source("Order.lex.tex"),
        lexlean::sdk::to_data_form(&text).expect("data form"),
    )
    .expect("write");
    expect_refusal(&scratch, &["Order.lex.tex", "not written in the notation"]);
    let scratch = Scratch::new();
    scratch.plant("spec/src/Order.lex.tex", "(nil String)", "(nil String");
    expect_refusal(
        &scratch,
        &["Order.lex.tex", "not a module of the authoring notation"],
    );

    // A missing module.
    let scratch = Scratch::new();
    std::fs::remove_file(scratch.source("Ledger.lex.tex")).expect("remove");
    let error = project(scratch.root()).expect_err("refused").to_string();
    assert!(error.contains("Ledger.lex.tex"), "{error}");
}

// §27.5: SPEC.md is the projection of the chapter modules, its three derived
// regions are the register's and the registry's, and the projector refuses a
// chapter it does not read.
fn specification_text() {
    let real = repo_root();
    let committed = project(real.as_std_path()).expect("the sources project");
    assert_eq!(
        committed.spec,
        root_file("SPEC.md"),
        "SPEC.md is the projection of the chapter modules"
    );
    let model = repo_model::Model::from_texts(real.as_std_path(), &committed)
        .expect("the projection parses as the model");
    let spec = &committed.spec;
    assert!(
        spec.lines().count() > 10_000,
        "anti-vacuity: the specification text is projected"
    );

    // The derived regions are the register's and the registry's, not typed.
    let register_rows = spec.lines().filter(|line| is_register_row(line)).count();
    assert_eq!(register_rows, model.ids.id.len(), "§31 lists the register");
    for row in &model.ids.id {
        let line = spec
            .lines()
            .find(|line| line.starts_with(&format!("| `{}` | `{}` | ", row.id, row.suite)))
            .unwrap_or_else(|| panic!("{}: §31 has its row", row.id));
        assert!(
            line.contains(&format!(" | {} | ", row.statement)),
            "{}: §31 states the register's statement",
            row.id
        );
    }
    for row in &model.errors.error {
        assert!(
            spec.contains(&format!("| `{}` | {} |\n", row.code, row.statement)),
            "{}: §26.3 states the registry's meaning",
            row.code
        );
    }
    for row in &model.authorities.authority {
        assert!(
            spec.contains(&format!("| `{}` | ", row.id)),
            "{}: §27.4 lists every authority of the register",
            row.id
        );
    }
    assert!(
        spec.contains(&format!(
            "**Total required capability IDs:** {}.\n",
            model.ids.id.len()
        )),
        "the total is the register's"
    );

    // A register edit that is not projected is a difference in SPEC.md too.
    let scratch = Scratch::new();
    scratch.plant(
        "spec/src/IdsRP.lex.tex",
        "The repository, crate, executable, metadata, and licenses",
        "The repository, crate, executable, metadata, and some licenses",
    );
    let planted = project(scratch.root()).expect("the edited source projects");
    assert!(
        planted
            .spec
            .contains("The repository, crate, executable, metadata, and some licenses"),
        "the edited statement reaches the derived §31 table"
    );
    let error = planted
        .check_committed(scratch.root())
        .expect_err("an unprojected register edit is refused")
        .to_string();
    assert!(error.contains("R1") && error.contains("SPEC.md"), "{error}");

    // A hand edit of a derived table.
    let scratch = Scratch::new();
    scratch.plant(
        "SPEC.md",
        "| `LLC0001` | Invalid command-line usage. |",
        "| `LLC0001` | Invalid usage. |",
    );
    let error = project(scratch.root())
        .expect("sources unchanged")
        .check_committed(scratch.root())
        .expect_err("a hand-edited derived table is refused")
        .to_string();
    assert!(error.contains("R1") && error.contains("SPEC.md"), "{error}");

    // U+FF05 is reserved to spell a percent sign in the chapters: a register
    // statement that carries one would reach SPEC.md unmapped, so it is refused.
    let scratch = Scratch::new();
    scratch.plant(
        "spec/src/IdsRP.lex.tex",
        "The repository, crate, executable",
        "The repository\u{ff05} crate, executable",
    );
    expect_refusal(&scratch, &["U+FF05", "NS-05"]);

    // Chapter items the projector does not read.
    let chapter = "spec/src/Chapter00.lex.tex";
    let scratch = Scratch::new();
    scratch.plant(chapter, "DocRows::Item.para", "DocRows::Item.quote");
    expect_refusal(
        &scratch,
        &["Chapter00.lex.tex", "not one of `DocRows.Item`", "NS-04"],
    );
    let scratch = Scratch::new();
    scratch.plant(chapter, "name: i0002,", "name: i0009,");
    expect_refusal(
        &scratch,
        &["Chapter00.lex.tex", "declaration 2", "expected `i0002`"],
    );
    let scratch = Scratch::new();
    let long = format!(
        "{}(nil String){}",
        "(cons \"x\" ".repeat(61),
        ")".repeat(61)
    );
    scratch.plant(chapter, "(nil String)", &long);
    expect_refusal(&scratch, &["Chapter00.lex.tex", "at most 60 lines"]);
    let scratch = Scratch::new();
    scratch.plant(
        "spec/src/Chapter31.lex.tex",
        "\"register-table\"",
        "\"no-such-table\"",
    );
    expect_refusal(&scratch, &["Chapter31.lex.tex", "not a derived block"]);
    let scratch = Scratch::new();
    let order = std::fs::read_to_string(scratch.source("DocOrder.lex.tex")).expect("module");
    assert!(order.contains("(cons \"Chapter32\" (nil String))"));
    scratch.plant(
        "spec/src/DocOrder.lex.tex",
        "(cons \"Chapter32\" (nil String))",
        "(nil String)",
    );
    expect_refusal(&scratch, &["Chapter32.lex.tex", "belongs to no part"]);
}

/// Is this line a row of the §31 table: `| `XX-00` | `suite` | … | … |`?
fn is_register_row(line: &str) -> bool {
    let bytes = line.as_bytes();
    line.starts_with("| `")
        && bytes.len() > 10
        && bytes[3].is_ascii_uppercase()
        && bytes[4].is_ascii_uppercase()
        && bytes[5] == b'-'
        && bytes[6].is_ascii_digit()
        && bytes[7].is_ascii_digit()
        && bytes[8] == b'`'
        && line.split(" | ").count() == 4
}

// R1, `NS-06`: the audit passes on the committed repository and refuses a
// second source of each kind.
fn source_authority() {
    let real = repo_root();
    let projection = project(real.as_std_path()).expect("the sources project");
    let model = repo_model::Model::load_from_repo_root().expect("the model loads");
    let summary = repo_model::authority::audit(real.as_std_path(), &model, &projection)
        .expect("the committed repository has one source of its normative text");
    assert!(
        summary.contains("hand-written Markdown files restate no registered statement"),
        "{summary}"
    );

    let plant = |build: &dyn Fn(&Path), needles: &[&str]| {
        let directory = tempfile::Builder::new()
            .prefix("lexlean-ns06-case-")
            .tempdir()
            .expect("tempdir");
        let root = directory.path();
        std::fs::create_dir_all(root.join("model/authorities")).expect("model directory");
        for file in [
            "SPEC.md",
            "CONFORMANCE.md",
            "ERRORS.md",
            "AGENTS.md",
            "CHANGELOG.md",
            "README.md",
            "VERIFICATION.md",
            "model/ids.toml",
            "model/errors.toml",
            "model/ledger.toml",
            "model/authorities.toml",
        ] {
            std::fs::copy(real.join(file).as_std_path(), root.join(file)).expect("copy");
        }
        for entry in
            std::fs::read_dir(real.join("model/authorities").as_std_path()).expect("copies")
        {
            let entry = entry.expect("entry");
            std::fs::copy(
                entry.path(),
                root.join("model/authorities").join(entry.file_name()),
            )
            .expect("copy vendored");
        }
        repo_model::authority::audit(root, &model, &projection)
            .expect("the scratch copy of the committed repository passes");
        build(root);
        let error = repo_model::authority::audit(root, &model, &projection)
            .expect_err("the plant is refused")
            .to_string();
        assert!(error.contains("R1") && error.contains("NS-06"), "{error}");
        for needle in needles {
            assert!(error.contains(needle), "`{needle}` is not in: {error}");
        }
    };
    plant(
        &|root| std::fs::write(root.join("model/extra.toml"), "spec = \"x\"\n").expect("write"),
        &["model/extra.toml"],
    );
    plant(
        &|root| {
            std::fs::write(
                root.join("model/authorities/second.md"),
                "# A second authority\n",
            )
            .expect("write")
        },
        &["model/authorities/second.md"],
    );
    plant(
        &|root| std::fs::write(root.join("SPEC2.md"), "# A second specification\n").expect("write"),
        &["SPEC2.md"],
    );
    // Markdown inside the specification project is read like any other.
    plant(
        &|root| {
            std::fs::create_dir_all(root.join("spec")).expect("spec directory");
            std::fs::write(
                root.join("spec/README.md"),
                format!("# Notes\n\n{}\n", model.ids.id[0].statement),
            )
            .expect("write");
        },
        &["spec/README.md", "restates a registered statement"],
    );
    plant(
        &|root| {
            let text = std::fs::read_to_string(root.join("SPEC.md")).expect("SPEC.md");
            std::fs::write(
                root.join("SPEC.md"),
                text.replace(repo_model::authority::SPEC_NOTICE, "> **Hand-written.**"),
            )
            .expect("write");
        },
        &["SPEC.md", "notice"],
    );
    plant(
        &|root| {
            let text = std::fs::read_to_string(root.join("ERRORS.md")).expect("ERRORS.md");
            std::fs::write(
                root.join("ERRORS.md"),
                text.replacen(repo_model::authority::GENERATED_MARKER, "<!--", 1),
            )
            .expect("write");
        },
        &["ERRORS.md"],
    );
    plant(
        &|root| {
            let row = &model.ids.id[0];
            let mut text = std::fs::read_to_string(root.join("README.md")).expect("README.md");
            text.push_str(&format!("\n{}\n", row.statement));
            std::fs::write(root.join("README.md"), text).expect("write");
        },
        &["README.md", "restates a registered statement"],
    );
}

/// Every file of the scratch source directory, to prove that a refusal leaves
/// the tree as it was.
fn snapshot(root: &Path) -> std::collections::BTreeMap<String, Vec<u8>> {
    source_files(&root.join(SOURCE_DIR))
        .into_iter()
        .map(|file| {
            let bytes = std::fs::read(root.join(SOURCE_DIR).join(&file)).expect("read");
            (file, bytes)
        })
        .collect()
}

fn lines(text: &str) -> Vec<String> {
    text.strip_suffix('\n')
        .unwrap_or(text)
        .split('\n')
        .map(str::to_owned)
        .collect()
}

/// The names of a module's items, in order.
fn item_names(text: &str) -> Vec<String> {
    lexlean::sdk::read_module(text).expect("a module")["declarations"]
        .as_array()
        .expect("declarations")
        .iter()
        .map(|d| d["name"].as_str().expect("name").to_owned())
        .collect()
}

// §27.5, `NS-07`: the chapter edit tool, on a scratch copy of the sources.
fn chapter_editing() {
    use repo_model::project::BlockKind;
    use repo_model::spec_edit::{
        apply, insert_after, port, replace_block, verify, Change, InsertKind, Intended,
    };

    let scratch = Scratch::new();
    let root = scratch.root();
    let current = || project(root).expect("the scratch sources project");
    let expect_untouched = |attempt: &dyn Fn() -> Result<(), String>, needles: &[&str]| {
        let before = snapshot(root);
        let error = attempt().expect_err("the refusal plant is refused");
        for needle in needles {
            assert!(error.contains(needle), "`{needle}` is not in: {error}");
        }
        assert!(error.contains("NS-07"), "{error}");
        assert_eq!(
            before,
            snapshot(root),
            "a refusal leaves the tree unchanged: {error}"
        );
    };

    // A prose block replaced: backslash, quote and percent sign survive; the
    // percent sign is U+FF05 in the module and `%` in SPEC.md.
    let first = "This document is the complete implementation contract for `github.com/UOR-Foundation/LexLean`.";
    let replacement: Vec<String> = vec![
        "A rule of 50% with a \\ backslash and a \"quote\".".to_owned(),
        "A second line.".to_owned(),
    ];
    let before = current();
    let outcome = replace_block(root, "Chapter01", first, &replacement).expect("replace-block");
    apply(root, &outcome).expect("apply");
    let mut wanted = lines(&before.spec);
    let block = before
        .layout
        .blocks
        .iter()
        .find(|b| b.chapter == "Chapter01" && b.lines.first().map(String::as_str) == Some(first))
        .expect("the block");
    wanted.splice(block.body.clone(), replacement.iter().cloned());
    assert_eq!(
        lines(&current().spec),
        wanted,
        "the regenerated SPEC.md is the intended text"
    );
    let module =
        std::fs::read_to_string(root.join(SOURCE_DIR).join("Chapter01.lex.tex")).expect("module");
    assert!(
        !module.contains('%') && module.contains('\u{ff05}'),
        "a percent sign is spelled U+FF05 in the module"
    );
    assert_eq!(
        lexlean::sdk::to_notation(&module).expect("notation"),
        module,
        "the module is canonical as written"
    );
    assert!(
        module.lines().take(5).any(|l| l.starts_with("\\title{"))
            && module.contains("\\useglossary{spec.titles@1.0.0}"),
        "the module keeps its own header"
    );

    // A block inserted after it: the items are renumbered in order.
    let names_before = item_names(&module).len();
    let outcome = insert_after(
        root,
        "Chapter01",
        &replacement[0],
        InsertKind::Paragraph,
        &["An inserted line.".to_owned()],
    )
    .expect("insert-after");
    apply(root, &outcome).expect("apply");
    let module =
        std::fs::read_to_string(root.join(SOURCE_DIR).join("Chapter01.lex.tex")).expect("module");
    let names = item_names(&module);
    assert_eq!(names.len(), names_before + 1);
    assert!(
        names
            .iter()
            .enumerate()
            .all(|(index, name)| *name == format!("i{:04}", index + 1)),
        "the items are numbered in order"
    );
    assert!(
        current()
            .spec
            .contains("A second line.\n\nAn inserted line.\n"),
        "the block follows"
    );

    // A block of more than sixty lines continues in `cont` items.
    let long: Vec<String> = (0..130).map(|n| format!("long line {n}")).collect();
    let outcome =
        replace_block(root, "Chapter01", "An inserted line.", &long).expect("a long block");
    apply(root, &outcome).expect("apply");
    let module =
        std::fs::read_to_string(root.join(SOURCE_DIR).join("Chapter01.lex.tex")).expect("module");
    assert_eq!(
        item_names(&module).len(),
        names_before + 3,
        "130 lines are three items"
    );
    assert!(
        module.matches("DocRows::Item.cont").count() >= 2,
        "the block continues"
    );
    assert!(
        current().spec.contains("long line 129\n"),
        "the last line is in SPEC.md"
    );

    // The merged text of a line that still edits SPEC.md as text: a prose
    // hunk (ported) and a hunk inside a derived region (reported, not ported).
    let state = current();
    let mut merged = lines(&state.spec);
    let prose = state
        .layout
        .blocks
        .iter()
        .find(|b| b.chapter == "Chapter03" && b.kind == BlockKind::Paragraph && b.lines.len() >= 2)
        .expect("a paragraph of two lines");
    let last = prose.body.end - 1;
    merged[last].push_str(" (merged)");
    let row = merged
        .iter()
        .position(|l| l.starts_with("| `RP-01` | `repository` | "))
        .expect("the RP-01 row of the derived table");
    // A marker no sentence of the specification carries: the text has
    // "tampered" of its own (`LLR3007`, `EX-09`), so a plain word would be
    // found whether or not the hunk was ported.
    merged[row].push_str(" tampered-derived-row");
    let merged = merged.join("\n") + "\n";
    let outcome = port(root, &merged).expect("port");
    assert!(
        outcome.report.iter().any(|r| r.contains("derived region")),
        "the derived hunk is reported: {:?}",
        outcome.report
    );
    apply(root, &outcome).expect("apply");
    let after = current();
    assert!(
        lines(&after.spec)[last].ends_with(" (merged)"),
        "the prose hunk is ported"
    );
    assert!(
        !after.spec.contains(" tampered-derived-row"),
        "the derived hunk is regenerated, not ported"
    );

    // Refusals leave the tree as it was.
    expect_untouched(
        &|| {
            replace_block(root, "Chapter01", "no block begins with this", &replacement)
                .map(|_| ())
                .map_err(|e| e.to_string())
        },
        &["no block begins"],
    );
    let doubled = insert_after(
        root,
        "Chapter01",
        &replacement[0],
        InsertKind::Paragraph,
        &[replacement[0].clone()],
    )
    .expect("a block that repeats a first line");
    apply(root, &doubled).expect("apply");
    expect_untouched(
        &|| {
            replace_block(root, "Chapter01", &replacement[0], &replacement)
                .map(|_| ())
                .map_err(|e| e.to_string())
        },
        &["2 blocks begin", "name another first line"],
    );
    expect_untouched(
        &|| {
            replace_block(
                root,
                "Chapter01",
                "long line 0",
                &["has \u{ff05} in it".to_owned()],
            )
            .map(|_| ())
            .map_err(|e| e.to_string())
        },
        &["U+FF05"],
    );
    // A hunk that spans two blocks: two adjacent paragraphs merged into one.
    let state = current();
    let pair = state
        .layout
        .blocks
        .windows(2)
        .find(|w| {
            w[0].chapter == w[1].chapter
                && w[0].kind == BlockKind::Paragraph
                && w[1].kind == BlockKind::Paragraph
                && w[0].body.end + 1 == w[1].body.start
        })
        .expect("two adjacent paragraphs");
    let mut spanning = lines(&state.spec);
    let (tail, head) = (pair[0].body.end - 1, pair[1].body.start);
    spanning[tail].push_str(" (joined)");
    spanning[head].push_str(" (joined)");
    spanning.remove(pair[0].body.end);
    let spanning = spanning.join("\n") + "\n";
    expect_untouched(
        &|| port(root, &spanning).map(|_| ()).map_err(|e| e.to_string()),
        &["spans", pair[0].chapter.as_str(), " and "],
    );
    // A blank line deleted between two blocks touches no block's lines, and
    // is named by its neighbours.
    let mut blank = lines(&state.spec);
    blank.remove(pair[0].body.end);
    let blank = blank.join("\n") + "\n";
    expect_untouched(
        &|| port(root, &blank).map(|_| ()).map_err(|e| e.to_string()),
        &[
            "a line outside every paragraph, table and fence",
            pair[0].chapter.as_str(),
        ],
    );
    // Lines a block cannot hold are refused on every path, even though SPEC.md
    // would be the same text: the structure of the module is what differs.
    let refuse_lines = |lines: Vec<String>, needle: &str| {
        expect_untouched(
            &|| {
                replace_block(root, "Chapter01", "long line 0", &lines)
                    .map(|_| ())
                    .map_err(|e| e.to_string())
            },
            &[needle],
        );
    };
    refuse_lines(
        vec!["First.".to_owned(), String::new(), "Second.".to_owned()],
        "empty line",
    );
    refuse_lines(vec!["First.".to_owned(), String::new()], "empty line");
    refuse_lines(Vec::new(), "at least one line");
    refuse_lines(vec!["### 1.9 A planted heading".to_owned()], "heading");
    refuse_lines(vec!["```".to_owned()], "fence");
    expect_untouched(
        &|| {
            insert_after(
                root,
                "Chapter01",
                "long line 0",
                InsertKind::Table,
                &["not a row".to_owned()],
            )
            .map(|_| ())
            .map_err(|e| e.to_string())
        },
        &["each begin with `|`"],
    );
    // The same through `port`: an inserted heading and paragraph, a changed
    // line made a heading, and a changed line split in two.
    let state = current();
    let target = state
        .layout
        .blocks
        .iter()
        .find(|b| b.chapter == "Chapter03" && b.kind == BlockKind::Paragraph && b.lines.len() >= 2)
        .expect("a paragraph of two lines");
    let mut planted = lines(&state.spec);
    let after_block = target.body.end;
    planted.splice(
        after_block..after_block,
        [
            "".to_owned(),
            "### 1.9 A planted heading".to_owned(),
            "".to_owned(),
            "A paragraph under it.".to_owned(),
        ],
    );
    let planted = planted.join("\n") + "\n";
    expect_untouched(
        &|| port(root, &planted).map(|_| ()).map_err(|e| e.to_string()),
        &["heading"],
    );
    let mut as_heading = lines(&state.spec);
    as_heading[target.body.start] = "# turned into a heading".to_owned();
    let as_heading = as_heading.join("\n") + "\n";
    expect_untouched(
        &|| {
            port(root, &as_heading)
                .map(|_| ())
                .map_err(|e| e.to_string())
        },
        &["heading"],
    );
    let mut split = lines(&state.spec);
    split.splice(
        target.body.start..=target.body.start,
        ["one".to_owned(), String::new(), "two".to_owned()],
    );
    let split = split.join("\n") + "\n";
    expect_untouched(
        &|| port(root, &split).map(|_| ()).map_err(|e| e.to_string()),
        &["empty line"],
    );

    // A paragraph inserted at the edge of a derived table is ported, not
    // excused as regenerated; one strictly inside the table is regenerated.
    let region = state.layout.derived[0].clone();
    for blank_first in [false, true] {
        let mut edge = lines(&state.spec);
        let block: Vec<String> = if blank_first {
            vec![String::new(), "A paragraph before the table.".to_owned()]
        } else {
            vec!["A paragraph before the table.".to_owned(), String::new()]
        };
        let at = if blank_first {
            region.start - 1
        } else {
            region.start
        };
        edge.splice(at..at, block);
        let edge = edge.join("\n") + "\n";
        let outcome = port(root, &edge).expect("a block before a derived table is ported");
        assert!(
            outcome.spec.contains("A paragraph before the table.\n\n"),
            "the paragraph reaches SPEC.md (blank line first: {blank_first})"
        );
        assert!(
            !outcome.report.iter().any(|r| r.contains("derived region")),
            "it is not reported as regenerated: {:?}",
            outcome.report
        );
    }
    let mut inside = lines(&state.spec);
    inside.insert(
        region.start + 3,
        "| `ZZ-99` | `none` | a row that is not the register's | §1 |".to_owned(),
    );
    let inside = inside.join("\n") + "\n";
    let outcome = port(root, &inside).expect("a row inside a derived table is regenerated");
    assert!(
        outcome.changes.is_empty() && !outcome.spec.contains("ZZ-99"),
        "the row is not ported"
    );

    // A row appended at the end of a derived table is regenerated, as a
    // forward merge appends register rows; the same row in front of a block
    // inserted beside the table is not. Every derived table is tried.
    let tables: Vec<Range<usize>> = state
        .layout
        .derived
        .iter()
        .filter(|region| lines(&state.spec)[region.start].starts_with('|'))
        .cloned()
        .collect();
    assert!(!tables.is_empty(), "the projection has derived tables");
    for region in &tables {
        let mut appended = lines(&state.spec);
        appended.insert(
            region.end,
            "| `ZZ-99` | `none` | a row appended by a merge | §1 |".to_owned(),
        );
        let appended = appended.join("\n") + "\n";
        let outcome = port(root, &appended).expect("a row appended to a derived table");
        assert!(
            outcome.changes.is_empty() && !outcome.spec.contains("ZZ-99"),
            "the appended row is regenerated, not ported (table at line {})",
            region.start + 1
        );
        assert!(
            outcome.report.iter().any(|r| r.contains("derived region")),
            "it is reported as regenerated: {:?}",
            outcome.report
        );
    }

    // A paragraph inserted before a table whose first row changes in the same
    // merge is one hunk whose new lines hold a block boundary: it is judged on
    // those lines too, and refused rather than dropped.
    for region in &tables {
        let mut both = lines(&state.spec);
        let header = both[region.start].clone();
        both.splice(
            region.start..=region.start,
            [
                "A paragraph before a changed table.".to_owned(),
                String::new(),
                format!("{header} "),
            ],
        );
        let both = both.join("\n") + "\n";
        expect_untouched(
            &|| port(root, &both).map(|_| ()).map_err(|e| e.to_string()),
            &[],
        );
    }

    // A merged text that shares little with SPEC.md is refused before the
    // search for its difference, not by running out of memory.
    let same_count: String = (0..lines(&state.spec).len())
        .map(|n| format!("unrelated {n}\n"))
        .collect();
    for text in ["", "one line\n", &"unrelated\n".repeat(5000), &same_count] {
        expect_untouched(
            &|| port(root, text).map(|_| ()).map_err(|e| e.to_string()),
            &["differ by more than"],
        );
    }

    // An unknown chapter is named as such, and a name that is not a chapter
    // is refused before a path is made of it.
    expect_untouched(
        &|| {
            replace_block(root, "Chapter99", "x", &["y".to_owned()])
                .map(|_| ())
                .map_err(|e| e.to_string())
        },
        &["not a chapter module of the specification project"],
    );
    expect_untouched(
        &|| {
            replace_block(root, "../Rows", "x", &["y".to_owned()])
                .map(|_| ())
                .map_err(|e| e.to_string())
        },
        &["is not a chapter module (`Chapter` and two digits)"],
    );

    // The comparison covers the whole text: a difference at the last line, a
    // line more and a line less are each refused naming where.
    let change = Change {
        chapter: "Chapter01".to_owned(),
        text: std::fs::read_to_string(root.join(SOURCE_DIR).join("Chapter01.lex.tex"))
            .expect("module"),
    };
    let regenerated = lines(&current().spec);
    let last = regenerated.len() - 1;
    for (name, mutate, needle) in [
        (
            "last line differs",
            Box::new(|l: &mut Vec<String>| l[last].push_str(" wrong"))
                as Box<dyn Fn(&mut Vec<String>)>,
            format!("at line {}", last + 1),
        ),
        (
            "a line more",
            Box::new(|l: &mut Vec<String>| l.push("extra".to_owned())),
            format!("at line {}", last + 2),
        ),
        (
            "a line less",
            Box::new(|l: &mut Vec<String>| {
                l.pop();
            }),
            format!("at line {}", last + 1),
        ),
    ] {
        let mut wanted = regenerated.clone();
        mutate(&mut wanted);
        let wanted = wanted.join("\n") + "\n";
        expect_untouched(
            &|| {
                verify(
                    root,
                    std::slice::from_ref(&change),
                    &Intended::Exact(wanted.clone()),
                )
                .map(|_| ())
                .map_err(|e| e.to_string())
            },
            &[needle.as_str()],
        );
        let _ = name;
    }
    let mut merged_tail = regenerated.clone();
    merged_tail[last].push_str(" wrong");
    let merged_tail = merged_tail.join("\n") + "\n";
    expect_untouched(
        &|| {
            verify(
                root,
                std::slice::from_ref(&change),
                &Intended::Merged(merged_tail.clone()),
            )
            .map(|_| ())
            .map_err(|e| e.to_string())
        },
        &["outside the derived regions"],
    );
    // The check is not vacuous: edits whose regenerated text is not the
    // intended text are refused before anything is written.
    let change = Change {
        chapter: "Chapter01".to_owned(),
        text: std::fs::read_to_string(root.join(SOURCE_DIR).join("Chapter01.lex.tex"))
            .expect("module"),
    };
    let mut wrong = lines(&current().spec);
    wrong[0].push_str(" wrong");
    let wrong = wrong.join("\n") + "\n";
    expect_untouched(
        &|| {
            verify(
                root,
                std::slice::from_ref(&change),
                &Intended::Exact(wrong.clone()),
            )
            .map(|_| ())
            .map_err(|e| e.to_string())
        },
        &["differs from the intended text at line 1"],
    );
}
