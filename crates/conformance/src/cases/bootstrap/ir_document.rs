//! `BS-67` (SPEC.md §17.23, §17.5, §17.9, §21.4): the document IR, the linked
//! project, and its semantic ID as language-1.2 source, `LexLeanBoot.IrDocument`,
//! held to the byte to `lexlean::ir::document::DocumentModule::to_json`,
//! `LinkedProject::to_json`, and the semantic ID the linker records.
//!
//! `moduleText` writes a linked module in the canonical JSON of the linked IR,
//! its title and blocks under one hashing renumbering of `BS-65`, its
//! declarations through `BS-66`; `linkedText` writes the linked project, its
//! modules in name order; `projectSemanticId` hashes that text with the
//! recipe of `BS-32`. A module's embedded closed-core, semantic, and tooling
//! modules are inputs in their serialized form. Every committed project that
//! links under its committed lock is a run, and its semantic ID is the one the
//! linker records for it.
//!
//! Each planted defect names the root and the one vector that catch it.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use lexlean::calculus::Value;
use lexlean::ir::document::{Block, DocumentModule, LinkedProject, Phrase, PhraseItem, Section};
use lexlean::ir::term::{Binder, BinderMode, LocalId, Term, Universe};
use lexlean::lexicon::lse::QualifiedId;
use lexlean::{Selection, Sha256Digest};
use serde_json::Value as Json2;

use super::ir_proof::enc_declaration;
use super::ir_term::{binder_chain, enc_term};
use super::rust_validate::{eligibility_of, rustc_differential};
use super::{bytes, list, option, pair, text, texts, Run};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.IrDocument";
const SOURCE: &str = "bootstrap/src/IrDocument.lex.tex";
const JSON_SOURCE: &str = "stdlib/src/Json.lex.tex";
const ROOTS: [(&str, &[&str]); 3] = [
    ("moduleText", &["allocation", "overflow", "recursion"]),
    ("linkedText", &["allocation", "overflow", "recursion"]),
    (
        "projectSemanticId",
        &["allocation", "overflow", "recursion"],
    ),
];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// A committed project whose linked IR is longer runs under rustc only.
const EVALUATOR_BYTES: usize = 262_144;
/// Directories never searched for committed projects: the negative fixtures
/// do not link, and UOR Atlas's closed-core modules are beyond a run's input.
const SKIPPED: &[&str] = &[
    ".git",
    "target",
    ".lexlean",
    "node_modules",
    "negative",
    "uor-atlas",
];

// ---------------------------------------------------------------- the sorts

fn constructors(sort: &str) -> &'static Vec<String> {
    static TABLES: OnceLock<BTreeMap<String, Vec<String>>> = OnceLock::new();
    let tables = TABLES.get_or_init(|| {
        let mut out = BTreeMap::new();
        for file in [SOURCE, JSON_SOURCE] {
            let source = std::fs::read_to_string(repo_root().join(file).as_std_path())
                .expect("the module's source");
            for block in source.split("inductive{").skip(1) {
                let block = block.split("\n}\n").next().expect("a block");
                let name = block
                    .split("name: ")
                    .nth(1)
                    .and_then(|rest| rest.split(',').next())
                    .expect("the inductive's name")
                    .trim()
                    .to_owned();
                let names: Vec<String> = block
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
                out.insert(name, names);
            }
        }
        out
    });
    tables
        .get(sort)
        .unwrap_or_else(|| panic!("no inductive `{sort}`"))
}

fn adt(sort: &str, constructor: &str, fields: Vec<Value>) -> Value {
    let found = constructors(sort)
        .iter()
        .position(|name| name == constructor)
        .unwrap_or_else(|| panic!("`{sort}` has no constructor `{constructor}`"));
    Value::Adt {
        constructor: u64::try_from(found).expect("an index"),
        fields,
    }
}

// ------------------------------------------------------------- the encoders

/// A JSON value as the standard library's `Json`: arrays and objects as
/// chains of items and members.
fn enc_json(json: &Json2) -> Value {
    match json {
        Json2::Bool(flag) => adt("Json", "JBool", vec![Value::Bool { value: *flag }]),
        Json2::Number(number) => {
            let (negative, magnitude) = if let Some(signed) = number.as_i64() {
                (signed < 0, signed.unsigned_abs())
            } else {
                (false, number.as_u64().expect("an integer"))
            };
            adt(
                "Json",
                "JInt",
                vec![
                    Value::Bool { value: negative },
                    Value::Nat {
                        value: magnitude.to_string(),
                    },
                ],
            )
        }
        Json2::String(content) => adt("Json", "JStr", vec![text(content)]),
        Json2::Array(items) => {
            let chain = items
                .iter()
                .rev()
                .fold(adt("Json", "JNil", Vec::new()), |rest, item| {
                    adt("Json", "JItem", vec![enc_json(item), rest])
                });
            adt("Json", "JArr", vec![chain])
        }
        Json2::Object(members) => {
            let chain = members
                .iter()
                .rev()
                .fold(adt("Json", "JNil", Vec::new()), |rest, (key, value)| {
                    adt("Json", "JMember", vec![text(key), enc_json(value), rest])
                });
            adt("Json", "JObj", vec![chain])
        }
        Json2::Null => panic!("the canonical JSON domain has no null"),
    }
}

fn enc_phrase(phrase: &Phrase) -> Value {
    list(
        phrase
            .items
            .iter()
            .map(|item| match item {
                PhraseItem::Word { entry, form } => adt(
                    "PhraseItem",
                    "WWord",
                    vec![text(&entry.to_string()), text(form)],
                ),
                PhraseItem::Math(term) => adt("PhraseItem", "WMath", vec![enc_term(term)]),
                PhraseItem::Punctuation(entry) => {
                    adt("PhraseItem", "WPunct", vec![text(&entry.to_string())])
                }
            })
            .collect(),
    )
}

fn block_chain(blocks: &[Block]) -> Value {
    blocks
        .iter()
        .rev()
        .fold(adt("Block", "BNil", Vec::new()), |rest, block| {
            let encoded = match block {
                Block::Section(section) => adt(
                    "Block",
                    "BSection",
                    vec![
                        text(&section.component),
                        enc_phrase(&section.heading),
                        binder_chain(&section.params),
                        block_chain(&section.blocks),
                    ],
                ),
                Block::Declaration(declaration) => {
                    adt("Block", "BDecl", vec![enc_declaration(declaration)])
                }
            };
            adt("Block", "BItem", vec![encoded, rest])
        })
}

fn enc_module(module: &DocumentModule) -> Value {
    Value::Adt {
        constructor: 0,
        fields: vec![
            text(&module.name),
            text(&module.lean_module),
            text(&module.source_path),
            bytes(&module.source_sha256.0),
            option(module.origin.as_deref().map(text)),
            texts(&module.glossary),
            texts(&module.imports),
            list(
                module
                    .import_lean
                    .iter()
                    .map(|(import, lean)| pair(text(import), text(lean)))
                    .collect(),
            ),
            enc_phrase(&module.title),
            block_chain(&module.blocks),
            option(
                module
                    .core
                    .as_ref()
                    .map(|core| text(&serde_json::to_string(core).unwrap_or_default())),
            ),
            option(module.semantic.as_ref().map(|semantic| {
                enc_json(&serde_json::to_value(semantic).expect("a semantic module is JSON"))
            })),
            option(module.tooling.as_ref().map(|tooling| {
                enc_json(&serde_json::to_value(tooling).expect("a tooling module is JSON"))
            })),
        ],
    }
}

/// The modules of a linked project as the root's list of named modules, in
/// the order given.
fn enc_modules(modules: &[(&String, &DocumentModule)]) -> Value {
    list(
        modules
            .iter()
            .map(|(name, module)| pair(text(name), enc_module(module)))
            .collect(),
    )
}

fn module_run(module: &DocumentModule) -> Run {
    (
        vec![enc_module(module)],
        serde_json::to_value(text(&module.to_json().to_canonical_string()))
            .expect("a value is JSON"),
    )
}

/// A linked project, its modules listed last first when `reversed`.
fn linked_run(project: &LinkedProject, reversed: bool) -> Run {
    let mut modules: Vec<(&String, &DocumentModule)> = project.modules.iter().collect();
    if reversed {
        modules.reverse();
    }
    (
        vec![enc_modules(&modules)],
        serde_json::to_value(text(&project.to_json().to_canonical_string()))
            .expect("a value is JSON"),
    )
}

struct Identity {
    project: LinkedProject,
    language: String,
    closure: String,
    semantic_id: Sha256Digest,
}

fn identity_run(identity: &Identity) -> Run {
    let modules: Vec<(&String, &DocumentModule)> = identity.project.modules.iter().collect();
    (
        vec![
            bytes(&lexlean::compiler_semantics_id_for(&identity.language).0),
            text(&identity.language),
            enc_modules(&modules),
            text(&identity.closure),
        ],
        serde_json::to_value(bytes(&identity.semantic_id.0)).expect("a value is JSON"),
    )
}

// ---------------------------------------------------------------- the corpus

fn projects_under(dir: &std::path::Path, out: &mut Vec<camino::Utf8PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(std::fs::DirEntry::file_name);
    for entry in entries {
        let name = entry.file_name().to_string_lossy().into_owned();
        let path = entry.path();
        if path.is_dir() {
            if !SKIPPED.contains(&name.as_str()) {
                projects_under(&path, out);
            }
        } else if name == "lexlean.toml" {
            out.push(camino::Utf8PathBuf::from_path_buf(dir.to_path_buf()).expect("UTF-8"));
        }
    }
}

/// Every committed project that links under its committed lock, or under one
/// computed from its manifest when it commits none, with the
/// semantic ID the linker records, each checked to be the recipe of `BS-32`
/// over the oracle's linked text.
fn committed() -> Vec<Identity> {
    let mut dirs = Vec::new();
    projects_under(repo_root().as_std_path(), &mut dirs);
    let mut out = Vec::new();
    for dir in &dirs {
        let project = match lexlean::project::Project::load(&dir.join("lexlean.toml")) {
            Ok(project) => project,
            Err(error) => {
                eprintln!("BS-67: {dir}: not loaded: {error:?}");
                continue;
            }
        };
        // A project that commits no lock links under one computed from its
        // manifest, as the tooling fixtures' own cases link them.
        let (lock, packages) = match lexlean::lock::read_current_lock(&project) {
            Ok(read) => read,
            Err(_) => match lexlean::lock::compute_lock(&project, false) {
                Ok(computed) => computed,
                Err(error) => {
                    eprintln!("BS-67: {dir}: no lock: {error:?}");
                    continue;
                }
            },
        };
        let checked = match lexlean::link::check_project(
            &project,
            &Selection::Entrypoints,
            &lock,
            packages,
        ) {
            Ok(checked) => checked,
            Err(error) => {
                eprintln!("BS-67: {dir}: does not link: {error:?}");
                continue;
            }
        };
        eprintln!("BS-67: {dir}: links, {} modules", checked.modules.len());
        let linked = LinkedProject {
            modules: checked
                .modules
                .iter()
                .map(|(name, module)| (name.clone(), module.document.clone()))
                .collect(),
        };
        let language = project.config.language.clone();
        let closure = checked
            .closure
            .closure_json("", &checked.visible_union)
            .to_canonical_string();
        assert_eq!(
            lexlean::artifact::content_id::semantic_id_for(
                lexlean::compiler_semantics_id_for(&language),
                &language,
                &linked.to_json().to_canonical_string(),
                &closure,
            ),
            checked.semantic_id,
            "{dir}: the recorded semantic ID is the recipe over the linked text"
        );
        out.push(Identity {
            project: linked,
            language,
            closure,
            semantic_id: checked.semantic_id,
        });
    }
    out
}

/// The linked projects of the committed projects that link.
pub(super) fn linked_projects() -> Vec<LinkedProject> {
    committed()
        .into_iter()
        .map(|identity| identity.project)
        .collect()
}

// ------------------------------------------------------------------ vectors

fn qualified(package: &str, entry: &str) -> QualifiedId {
    QualifiedId {
        package: package.to_owned(),
        entry: entry.to_owned(),
    }
}

fn local(id: u64) -> Term {
    Term::Local(LocalId(id))
}

fn module(name: &str, lean_module: &str) -> DocumentModule {
    DocumentModule {
        name: name.to_owned(),
        lean_module: lean_module.to_owned(),
        source_path: format!("src/{name}.lex.tex"),
        source_sha256: Sha256Digest([3; 32]),
        origin: None,
        glossary: vec!["lexlean.std.nat@1.2.0".to_owned()],
        imports: Vec::new(),
        import_lean: BTreeMap::new(),
        title: Phrase::default(),
        blocks: Vec::new(),
        core: None,
        semantic: None,
        tooling: None,
    }
}

fn declaration_block(component: &str, statement: Term) -> Block {
    use lexlean::ir::declaration::{AxiomPolicy, DeclBody, DeclKind, Declaration};
    use lexlean::ir::proof::Proof;
    Block::Declaration(Box::new(Declaration {
        component: component.to_owned(),
        lean_name: component.replace('-', "_"),
        kind: DeclKind::Theorem,
        params: Vec::new(),
        body: DeclBody::TheoremLike {
            statement: statement.clone(),
            proof: Proof::Exact(statement),
        },
        policy: AxiomPolicy::None,
    }))
}

fn math_title(id: u64) -> Phrase {
    Phrase {
        items: vec![PhraseItem::Math(local(id))],
    }
}

enum Input {
    Module(Box<DocumentModule>),
    Linked(LinkedProject, bool),
    Identity(Box<Identity>),
}

impl Input {
    const fn root(&self) -> usize {
        match self {
            Self::Module(_) => 0,
            Self::Linked(..) => 1,
            Self::Identity(_) => 2,
        }
    }

    fn run(&self) -> Run {
        match self {
            Self::Module(module) => module_run(module),
            Self::Linked(project, reversed) => linked_run(project, *reversed),
            Self::Identity(identity) => identity_run(identity),
        }
    }
}

struct Vector {
    label: &'static str,
    input: Input,
}

/// The smallest committed module with the given embedded module.
fn smallest(
    identities: &[Identity],
    kind: &str,
    pick: fn(&DocumentModule) -> bool,
) -> DocumentModule {
    identities
        .iter()
        .flat_map(|identity| identity.project.modules.values())
        .filter(|module| pick(module))
        .min_by_key(|module| module.to_json().to_canonical_string().len())
        .unwrap_or_else(|| panic!("no committed module embeds a {kind} module"))
        .clone()
}

/// The smallest closed-core module of UOR Atlas, read from its source alone:
/// no other committed project embeds one, and Atlas is beyond a linked run.
fn atlas_core() -> lexlean::ir::core::CoreModule {
    fn sources(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                sources(&path, out);
            } else if path.extension().is_some_and(|extension| extension == "tex") {
                out.push(path);
            }
        }
    }
    let atlas = repo_root().join("examples/uor-atlas");
    let language = lexlean::project::Project::load(&atlas.join("lexlean.toml"))
        .expect("UOR Atlas loads")
        .config
        .language;
    let mut paths = Vec::new();
    sources(atlas.as_std_path(), &mut paths);
    let mut texts: Vec<String> = paths
        .iter()
        .filter_map(|path| std::fs::read_to_string(path).ok())
        .filter(|source| source.contains("coremodule"))
        .collect();
    texts.sort_by_key(|source| (source.len(), source.clone()));
    texts
        .iter()
        .find_map(|source| lexlean::ir::core::CoreModule::from_source(source, &language).ok())
        .expect("a closed-core module of UOR Atlas")
}

#[allow(clippy::too_many_lines)]
fn named(identities: &[Identity]) -> Vec<Vector> {
    let mut out = Vec::new();
    let mut add = |label: &'static str, input: Input| out.push(Vector { label, input });
    add(
        "an empty module",
        Input::Module(Box::new(module("Main", "Demo.Main"))),
    );
    add(
        "the title renumbers first",
        Input::Module(Box::new(DocumentModule {
            title: math_title(5),
            blocks: vec![
                declaration_block("thm-a", local(7)),
                declaration_block("thm-b", local(5)),
            ],
            ..module("Main", "Demo.Main")
        })),
    );
    add(
        "a section with parameters",
        Input::Module(Box::new(DocumentModule {
            blocks: vec![Block::Section(Section {
                component: "sec-one".to_owned(),
                heading: math_title(9),
                params: vec![Binder {
                    id: LocalId(4),
                    mode: BinderMode::Implicit,
                    spelling: "x".to_owned(),
                    ty: Term::Sort(Universe::Num(1)),
                }],
                blocks: vec![declaration_block(
                    "thm-c",
                    Term::App {
                        function: Box::new(local(8)),
                        explicit_args: vec![local(4)],
                        omitted_implicit_binders: Vec::new(),
                    },
                )],
            })],
            ..module("Main", "Demo.Main")
        })),
    );
    add(
        "phrase items",
        Input::Module(Box::new(DocumentModule {
            title: Phrase {
                items: vec![
                    PhraseItem::Word {
                        entry: qualified("lexlean.std.nat", "nat"),
                        form: "plural".to_owned(),
                    },
                    PhraseItem::Punctuation(qualified("lexlean.core", "colon")),
                    PhraseItem::Math(local(1)),
                ],
            },
            ..module("Main", "Demo.Main")
        })),
    );
    add(
        "an origin",
        Input::Module(Box::new(DocumentModule {
            origin: Some("lexlean.stdlib".to_owned()),
            ..module("Json", "LexLeanStd.Json")
        })),
    );
    add(
        "foreign imports",
        Input::Module(Box::new(DocumentModule {
            imports: vec!["A".to_owned(), "B".to_owned()],
            import_lean: [
                ("A".to_owned(), "Demo.A".to_owned()),
                ("B".to_owned(), "Other.B".to_owned()),
            ]
            .into_iter()
            .collect(),
            ..module("Main", "Demo.Main")
        })),
    );
    add(
        "a module not under its prefix",
        Input::Module(Box::new(DocumentModule {
            imports: vec!["A".to_owned(), "B".to_owned()],
            import_lean: [
                ("A".to_owned(), "A".to_owned()),
                ("B".to_owned(), "Demo.B".to_owned()),
            ]
            .into_iter()
            .collect(),
            ..module("Main", "X.Y")
        })),
    );
    add(
        "a core module",
        Input::Module(Box::new(DocumentModule {
            core: Some(atlas_core()),
            ..module("Core", "UorAtlas.Core")
        })),
    );
    add(
        "a semantic module",
        Input::Module(Box::new(smallest(identities, "semantic", |module| {
            module.semantic.is_some()
        }))),
    );
    add(
        "a tooling module",
        Input::Module(Box::new(smallest(identities, "tooling", |module| {
            module.tooling.is_some()
        }))),
    );
    let two = LinkedProject {
        modules: [
            ("Zeta".to_owned(), module("Zeta", "Demo.Zeta")),
            ("Alpha".to_owned(), module("Alpha", "Demo.Alpha")),
        ]
        .into_iter()
        .collect(),
    };
    add("modules out of order", Input::Linked(two.clone(), true));
    add("modules in order", Input::Linked(two.clone(), false));
    add(
        "a semantic ID",
        Input::Identity(Box::new(Identity {
            semantic_id: lexlean::artifact::content_id::semantic_id_for(
                lexlean::compiler_semantics_id_for("1.2"),
                "1.2",
                &two.to_json().to_canonical_string(),
                "{}",
            ),
            project: two,
            language: "1.2".to_owned(),
            closure: "{}".to_owned(),
        })),
    );
    out
}

// ------------------------------------------------------------------- plants

struct Plant {
    label: &'static str,
    site: &'static str,
    replacement: &'static str,
    vector: &'static str,
}

const PLANTS: &[Plant] = &[
    Plant {
        label: "the blocks do not share the title's renumbering",
        site: "(blockJson (project linked blocks) (second titled))",
        replacement: "(blockJson (project linked blocks) (IrTerm::hashing))",
        vector: "the title renumbers first",
    },
    Plant {
        label: "a section's blocks do not see its parameters",
        site: "(blockJson nested (second bound))",
        replacement: "(blockJson nested (second headed))",
        vector: "a section with parameters",
    },
    Plant {
        label: "a punctuation is written as a word",
        site: "(IrTerm::jtext \"punct\")",
        replacement: "(IrTerm::jtext \"word\")",
        vector: "phrase items",
    },
    Plant {
        label: "an origin is never written",
        site: "| Option.some named => (IrTerm::member \"origin\" (jbytes named) withForeign)",
        replacement: "| Option.some named => withForeign",
        vector: "an origin",
    },
    Plant {
        label: "a default import is written",
        site: "then rest else (ctor LexLeanStd.Json::Json.JMember",
        replacement: "then (ctor LexLeanStd.Json::Json.JMember (first entry) (jbytes (second entry)) rest) else (ctor LexLeanStd.Json::Json.JMember",
        vector: "foreign imports",
    },
    Plant {
        label: "the stem keeps the module's own name",
        site: "(LexLeanStd.Text::sliceOr leanName 0 (prim subtract total short : Nat))",
        replacement: "leanName",
        vector: "foreign imports",
    },
    Plant {
        label: "a module not under its prefix keeps its whole name as the stem",
        site: "else 0x);",
        replacement: "else leanName);",
        vector: "a module not under its prefix",
    },
    Plant {
        label: "the glossary is written as the imports",
        site: "(IrTerm::member \"glossary\" (textArray (project linked glossary))",
        replacement: "(IrTerm::member \"glossary\" (textArray (project linked imports))",
        vector: "an empty module",
    },
    Plant {
        label: "a core module is never written",
        site: "| Option.some text => (IrTerm::member \"core\" (jbytes text) withSemantic)",
        replacement: "| Option.some text => withSemantic",
        vector: "a core module",
    },
    Plant {
        label: "a semantic module is never written",
        site: "| Option.some value => (IrTerm::member \"semantic\" value withTooling)",
        replacement: "| Option.some value => withTooling",
        vector: "a semantic module",
    },
    Plant {
        label: "a tooling module is written as a semantic one",
        site: "(IrTerm::member \"tooling\" value (jnil))",
        replacement: "(IrTerm::member \"semantic\" value (jnil))",
        vector: "a tooling module",
    },
    Plant {
        label: "the modules are not sorted by name",
        site: "(LexLeanStd.Order::sortByKey [DocumentModule] modules)",
        replacement: "modules",
        vector: "modules out of order",
    },
    Plant {
        label: "the linked IR names another specification",
        site: "(IrTerm::jtext \"lexlean/linked-ir/1\")",
        replacement: "(IrTerm::jtext \"lexlean/linked-ir/2\")",
        vector: "modules in order",
    },
    Plant {
        label: "the semantic ID hashes the linked text as a file",
        site: "(ContentId::semanticId compilerSemantics language (LexLeanStd.Json::writeJson (linkedJson modules)) lexiconClosure)",
        replacement: "(ContentId::semanticId compilerSemantics language (LexLeanStd.Json::writeJsonFile (linkedJson modules)) lexiconClosure)",
        vector: "a semantic ID",
    },
];

fn plants(vectors: &[Vector]) {
    let mut missed = Vec::new();
    for plant in PLANTS {
        let vector = vectors
            .iter()
            .find(|vector| vector.label == plant.vector)
            .unwrap_or_else(|| panic!("{}: no vector `{}`", plant.label, plant.vector));
        let root = ROOTS[vector.input.root()].0;
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/IrDocument.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/IrDocument.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, root);
        match super::evaluate(&low, &[vector.input.run()], 1).first() {
            Some(first) => eprintln!(
                "BS-67: plant detected: {}: {}",
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

fn export(lowered: &BTreeMap<&str, super::Lowered>) {
    let exported = harness::exported("bootstrap");
    let bundle = exported.bundle();
    let mut realized = 0usize;
    for exported_root in &exported.output.result.roots {
        let Some((member, low)) = exported_root
            .root
            .strip_prefix(&format!("{MODULE}."))
            .and_then(|member| lowered.get(member).map(|low| (member, low)))
        else {
            continue;
        };
        let largest = exported_root
            .outputs
            .iter()
            .flat_map(|output| &output.certification.certificates)
            .map(|certificate| certificate.file.byte_length)
            .max()
            .expect("a certificate");
        eprintln!(
            "BS-67: {}: largest certificate part {largest} bytes",
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
                serde_json::to_value(&low.program).expect("a program is JSON"),
                "{member}: the exported package realizes the program the differential ran"
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

/// The runs of each root: the vectors, every module and every project of the
/// corpus, and each project's identity; the evaluator takes those whose
/// linked text is short.
fn runs(vectors: &[Vector], identities: &[Identity], evaluator: bool) -> [Vec<Run>; 3] {
    let mut out = [Vec::new(), Vec::new(), Vec::new()];
    for vector in vectors {
        out[vector.input.root()].push(vector.input.run());
    }
    for identity in identities {
        let length = identity.project.to_json().to_canonical_string().len();
        if evaluator && length > EVALUATOR_BYTES {
            continue;
        }
        for module in identity.project.modules.values() {
            out[0].push(module_run(module));
        }
        out[1].push(linked_run(&identity.project, false));
        out[2].push(identity_run(identity));
    }
    out
}

pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("IrDocument"),
        "IrDocument is a linked module of the project"
    );
    eligibility_of(&project, "IrDocument", MODULE, &ROOTS);

    let identities = committed();
    assert!(
        identities.len() >= 20,
        "{} committed projects link under their committed locks",
        identities.len()
    );
    let vectors = named(&identities);
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    eprintln!(
        "BS-67: {} vectors, {} committed projects with {} modules, each project's semantic ID the recipe over its linked text ({:?})",
        vectors.len(),
        identities.len(),
        identities
            .iter()
            .map(|identity| identity.project.modules.len())
            .sum::<usize>(),
        started.elapsed()
    );

    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, super::Lowered> = ROOTS
        .iter()
        .map(|(root, _)| (*root, super::lower(&checked, &project, MODULE, root)))
        .collect();
    if support::normative_host() {
        for (index, corpus) in runs(&vectors, &identities, true).iter().enumerate() {
            let root = ROOTS[index].0;
            let disagreements = super::evaluate(&lowered[root], corpus, 5);
            assert!(
                disagreements.is_empty(),
                "{root} and the Rust oracle disagree: {disagreements:?}"
            );
            eprintln!(
                "BS-67: {root}: the evaluator agrees on {} runs ({:?})",
                corpus.len(),
                started.elapsed()
            );
        }
        plants(&vectors);
    } else {
        eprintln!("BS-67: platform-bound host: the evaluator and the planted defects run on the normative host");
    }

    let compiled = runs(&vectors, &identities, false);
    let differential: Vec<(&str, &super::Lowered, &[Run])> = ROOTS
        .iter()
        .enumerate()
        .map(|(index, (root, _))| (*root, &lowered[root], compiled[index].as_slice()))
        .collect();
    let total = rustc_differential("BS-67", &differential, "oracle");
    eprintln!("BS-67: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-67") {
        return;
    }
    export(&lowered);
    eprintln!("BS-67: passed in {:?}", started.elapsed());
}
