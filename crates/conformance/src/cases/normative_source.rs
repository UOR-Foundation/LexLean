//! Normative text as LexLean source (`NS-01`..`NS-04`): the `spec/` project
//! and the projection of its register modules to `model/*.toml`.

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
