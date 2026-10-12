//! The registry of the extraction adapter: the closed interface of Lean's
//! compiler front end that the adapter's tooling module (SPEC.md §17.20,
//! §22.10) is written against, derived from the pinned toolchain and not
//! written by hand.
//!
//! A row is exactly what Lean says: the exporter (`tests/lcnf/export.lean`)
//! reads the type of every constant the adapter's terms name and the
//! constructors of every type they match, construct, or project, and states
//! each as a registry row; the toolchain source file that defines it is pinned
//! by its SHA-256. The constants the elaborator inserts that no term names
//! are *plumbing*, found by elaborating the generated adapter and listing what
//! it uses. `tests/lcnf/registry.json` is that output; the committed tooling
//! project is the adapter's functions over it, and neither is edited by hand:
//! a registry that differs from a fresh export, or a project that differs from
//! its generator, fails the gate that names it.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{json, Value as Json};

use crate::tl;

/// The committed registry: rows, then plumbing.
pub const REGISTRY_PATH: &str = "tests/lcnf/registry.json";

/// The exporter, run by the pinned Lean.
pub const EXPORT_PATH: &str = "tests/lcnf/export.lean";

/// The committed tooling project the compiler embeds.
pub const PROJECT_DIR: &str = "language/lcnf-1.2/tooling";

/// The authority row of `model/authorities.toml` the module cites.
pub const AUTHORITY: &str = "LEAN-LCNF-4-32-1";

/// The module name of the committed tooling project.
pub const MODULE: &str = "Extract";

/// Types the adapter's terms project a field of, which no `match` or
/// constructor names: the exporter lists their constructors so the field is
/// typed.
const PROJECTED: [&str; 9] = [
    "Lean.ConstantVal",
    "Lean.FVarId",
    "Lean.Compiler.LCNF.Decl",
    "Lean.Compiler.LCNF.LetDecl",
    "Lean.Compiler.LCNF.Param",
    "Lean.Compiler.LCNF.Signature",
    "Lean.ConstructorVal",
    "Lean.DefinitionVal",
    "Lean.InductiveVal",
];

/// Abbreviations the adapter's types are stated through; each is the type it
/// abbreviates wherever types are compared.
const ABBREVIATIONS: [&str; 3] = [
    "Lean.ConstMap",
    "Lean.Elab.Command.CommandElab",
    "Lean.ModuleIdx",
];

fn owner(constructor: &str) -> &str {
    constructor
        .rsplit_once('.')
        .map_or(constructor, |(owner, _)| owner)
}

fn walk(
    value: &Json,
    defined: &BTreeSet<String>,
    calls: &mut BTreeSet<String>,
    inductive: &mut BTreeSet<String>,
    opaque: &mut BTreeSet<String>,
) {
    match value {
        Json::Array(items) => {
            for item in items {
                walk(item, defined, calls, inductive, opaque);
            }
        }
        Json::Object(members) => {
            match members.get("kind").and_then(Json::as_str) {
                Some("call") => {
                    if let Some(function) = members.get("function").and_then(Json::as_str) {
                        if !defined.contains(function) {
                            calls.insert(function.to_owned());
                        }
                    }
                }
                Some("construct" | "index") => {
                    if let Some(constructor) = members.get("constructor").and_then(Json::as_str) {
                        inductive.insert(owner(constructor).to_owned());
                    }
                }
                Some("const") => {
                    if let Some(name) = members.get("name").and_then(Json::as_str) {
                        if members.contains_key("arguments") {
                            opaque.insert(name.to_owned());
                        }
                    }
                }
                _ => {}
            }
            if let Some(branches) = members.get("branches").and_then(Json::as_array) {
                for branch in branches {
                    if let Some(constructor) = branch.get("constructor").and_then(Json::as_str) {
                        inductive.insert(owner(constructor).to_owned());
                    }
                }
            }
            for member in members.values() {
                walk(member, defined, calls, inductive, opaque);
            }
        }
        _ => {}
    }
}

/// The exporter's requests: a call for every constant a term calls, the
/// constructors of every type a term matches, constructs, or indexes by, an
/// opaque type for every other type named, and the abbreviations.
#[must_use]
pub fn seeds() -> Vec<String> {
    let functions = crate::lcnf_adapter::functions();
    let defined: BTreeSet<String> = functions
        .iter()
        .filter_map(|function| function["name"].as_str().map(str::to_owned))
        .collect();
    let (mut calls, mut inductive, mut opaque) =
        (BTreeSet::new(), BTreeSet::new(), BTreeSet::new());
    for function in &functions {
        walk(function, &defined, &mut calls, &mut inductive, &mut opaque);
    }
    inductive.extend(PROJECTED.iter().map(|name| (*name).to_owned()));
    let abbreviations: BTreeSet<String> = ABBREVIATIONS
        .iter()
        .map(|name| (*name).to_owned())
        .collect();
    opaque.retain(|name| !inductive.contains(name) && !abbreviations.contains(name));
    let mut seeds = Vec::new();
    seeds.extend(
        abbreviations
            .into_iter()
            .map(|name| format!("alias:{name}")),
    );
    seeds.extend(
        inductive
            .into_iter()
            .map(|name| format!("inductive:{name}")),
    );
    seeds.extend(opaque.into_iter().map(|name| format!("opaque:{name}")));
    seeds.extend(calls.into_iter().map(|name| format!("call:{name}")));
    seeds
}

/// The pinned toolchain's root.
fn toolchain_root() -> Result<(PathBuf, PathBuf), String> {
    let limits = lexlean::config::Limits {
        max_file_bytes: 16_777_216,
        max_total_source_bytes: 67_108_864,
        max_primitive_atoms: 2_000_000,
        max_token_lattice_edges: 4_000_000,
        max_parse_states: 4_000_000,
        max_ir_nodes: 2_000_000,
        max_scope_depth: 1024,
        max_import_depth: 128,
        max_diagnostics: 256,
        max_child_output_bytes: 16_777_216,
        child_timeout_ms: 300_000,
    };
    let toolchain = lexlean::verify::toolchain::preflight(&limits)
        .map_err(|error| format!("the pinned toolchain: {error:?}"))?;
    Ok((
        toolchain.root.as_std_path().to_path_buf(),
        toolchain.lean.path.as_std_path().to_path_buf(),
    ))
}

/// The source pin of a Lean module: its file in the toolchain and the SHA-256
/// of that file.
fn pin(toolchain: &Path, module: &str) -> Result<Json, String> {
    let relative = format!("src/lean/{}.lean", module.replace('.', "/"));
    let bytes = std::fs::read(toolchain.join(&relative))
        .map_err(|error| format!("the toolchain has no source for `{module}`: {error}"))?;
    Ok(json!({
        "file": relative,
        "sha256": lexlean::artifact::content_id::Sha256Digest::of(&bytes).to_hex(),
    }))
}

fn run_lean(root: &Path, arguments: &[String]) -> Result<(i32, String), String> {
    let (toolchain, lean) = toolchain_root()?;
    let output = Command::new(&lean)
        .args(arguments)
        .current_dir(root)
        .env("PATH", toolchain.join("bin"))
        .output()
        .map_err(|error| format!("{}: {error}", lean.display()))?;
    Ok((
        output.status.code().unwrap_or(-1),
        format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ),
    ))
}

/// The registry rows of the seeds, from the pinned Lean.
///
/// # Errors
///
/// The reason the exporter failed, or a constant it could not state.
pub fn export(root: &Path, requests: &BTreeSet<String>) -> Result<Vec<Json>, String> {
    let (toolchain, _) = toolchain_root()?;
    let mut arguments = vec!["--run".to_owned(), EXPORT_PATH.to_owned()];
    arguments.extend(requests.iter().cloned());
    let (code, output) = run_lean(root, &arguments)?;
    if code != 0 {
        return Err(format!("the exporter exited {code}: {output}"));
    }
    let parsed: Json = serde_json::from_str(&output)
        .map_err(|error| format!("the exporter's output: {error}: {output}"))?;
    if parsed["errors"]
        .as_array()
        .is_some_and(|errors| !errors.is_empty())
    {
        return Err(format!(
            "the exporter could not state: {}",
            serde_json::to_string_pretty(&parsed["errors"]).unwrap_or_default()
        ));
    }
    let mut rows = Vec::new();
    for row in parsed["rows"]
        .as_array()
        .ok_or("the exporter returned no rows")?
    {
        let mut row = row.clone();
        let module = row["source"]["module"]
            .as_str()
            .ok_or("a row without its module")?
            .to_owned();
        row["source"] = pin(&toolchain, &module)?;
        if row["kind"] == "call" {
            row["role"] = json!("a Lean operation the adapter calls");
        }
        rows.push(row);
    }
    rows.sort_by(|a, b| {
        let key = |row: &Json| {
            (
                u8::from(row["kind"] == "call"),
                row["name"].as_str().unwrap_or_default().to_owned(),
            )
        };
        key(a).cmp(&key(b))
    });
    Ok(rows)
}

/// The module's data over a registry's rows and plumbing.
#[must_use]
pub fn module_data(rows: &[Json], plumbing: &[String]) -> Json {
    let mut declarations: Vec<Json> = rows.to_vec();
    declarations.push(json!({"kind": "plumbing", "names": plumbing}));
    declarations.extend(crate::lcnf_adapter::functions());
    declarations.push(tl::command(
        "LexLeanExtract.signatureCommand",
        vec![
            tl::token("lexlean_signature "),
            tl::ident_atom(),
            tl::token(" : "),
            tl::type_atom(),
        ],
        "LexLeanExtract.elabSignature",
    ));
    tl::module(AUTHORITY, Vec::new(), declarations)
}

/// The committed registry's rows and plumbing.
///
/// # Errors
///
/// The reason the registry cannot be read.
pub fn committed(root: &Path) -> Result<(Vec<Json>, Vec<String>), String> {
    let text = std::fs::read_to_string(root.join(REGISTRY_PATH))
        .map_err(|error| format!("{REGISTRY_PATH}: {error}"))?;
    let value: Json =
        serde_json::from_str(&text).map_err(|error| format!("{REGISTRY_PATH}: {error}"))?;
    let rows = value["rows"].as_array().ok_or("rows")?.clone();
    let plumbing = value["plumbing"]
        .as_array()
        .ok_or("plumbing")?
        .iter()
        .map(|name| name.as_str().map(str::to_owned).ok_or("a plumbing name"))
        .collect::<Result<Vec<_>, _>>()?;
    Ok((rows, plumbing))
}

/// The committed registry as bytes.
#[must_use]
pub fn registry_bytes(rows: &[Json], plumbing: &[String]) -> Vec<u8> {
    let mut text = serde_json::to_string_pretty(&json!({"rows": rows, "plumbing": plumbing}))
        .expect("serializes");
    text.push('\n');
    text.into_bytes()
}

/// The configuration of the committed project.
#[must_use]
pub fn config() -> String {
    "spec = \"lexlean/project/1\"\nname = \"lcnf-tooling\"\nlanguage = \"1.2\"\nmodule_prefix = \"LcnfTooling\"\nsource_roots = [\"src\"]\nentrypoints = [\"src/Extract.lex.tex\"]\nbuild_root = \".lexlean\"\nlockfile = \"lexlean.lock\"\nlean_workspace = \".\"\nlean_toolchain = \"leanprover/lean4:v4.32.1\"\n\n[[lexicon_source]]\npackage = \"lexlean.std.nat\"\nkind = \"builtin\"\n\n[limits]\nmax_file_bytes = 16777216\nmax_total_source_bytes = 67108864\nmax_primitive_atoms = 2000000\nmax_token_lattice_edges = 4000000\nmax_parse_states = 4000000\nmax_ir_nodes = 2000000\nmax_scope_depth = 1024\nmax_import_depth = 128\nmax_diagnostics = 256\nmax_child_output_bytes = 16777216\nchild_timeout_ms = 300000\n".to_owned()
}

/// The Lake workspace a project names, which the compiler requires of every
/// project it loads and never runs for a tooling module.
const WORKSPACE: [(&str, &str); 3] = [
    ("lakefile.toml", "name = \"lcnf_tooling\"\nversion = \"0.1.0\"\n"),
    (
        "lake-manifest.json",
        "{\"version\": \"1.2.0\",\n \"packagesDir\": \".lake/packages\",\n \"packages\": [],\n \"name\": \"lcnf_tooling\",\n \"lakeDir\": \".lake\",\n \"fixedToolchain\": false}\n",
    ),
    ("lean-toolchain", "leanprover/lean4:v4.32.1\n"),
];

/// The generated files of the committed project, by repository path.
#[must_use]
pub fn files(rows: &[Json], plumbing: &[String]) -> BTreeMap<String, Vec<u8>> {
    let mut out = BTreeMap::new();
    out.insert(format!("{PROJECT_DIR}/lexlean.toml"), config().into_bytes());
    for (name, text) in WORKSPACE {
        out.insert(format!("{PROJECT_DIR}/{name}"), text.as_bytes().to_vec());
    }
    out.insert(
        format!("{PROJECT_DIR}/src/{MODULE}.lex.tex"),
        tl::module_tex(
            MODULE,
            "Natural number",
            "lexlean.std.nat@1.2.0",
            &module_data(rows, plumbing),
        )
        .into_bytes(),
    );
    out
}

/// What the pinned Lean says the generated adapter uses: one `class name` line
/// per constant that is not the adapter's own, from `listAdapterConstants`.
fn listing(rows: &[Json]) -> Result<Vec<(String, String)>, String> {
    let mut data = module_data(rows, &["LexLeanExtract.placeholder".to_owned()]);
    data["declarations"]
        .as_array_mut()
        .expect("declarations")
        .push(tl::evaluate(
            "LexLeanExtract.listAdapterConstants",
            Vec::new(),
        ));
    let temp = tempfile::Builder::new()
        .prefix("lexlean-lcnf-listing-")
        .tempdir()
        .map_err(|error| error.to_string())?;
    let source = temp.path().join("src");
    std::fs::create_dir_all(&source).map_err(|error| error.to_string())?;
    std::fs::write(temp.path().join("lexlean.toml"), config())
        .map_err(|error| error.to_string())?;
    for (name, text) in WORKSPACE {
        std::fs::write(temp.path().join(name), text).map_err(|error| error.to_string())?;
    }
    std::fs::write(
        source.join(format!("{MODULE}.lex.tex")),
        tl::module_tex(MODULE, "Natural number", "lexlean.std.nat@1.2.0", &data),
    )
    .map_err(|error| error.to_string())?;
    let config_path = camino::Utf8PathBuf::from_path_buf(temp.path().join("lexlean.toml"))
        .map_err(|bad| format!("non-UTF-8 path {}", bad.display()))?;
    let build = crate::tooling::render(&config_path)?;
    let text = &build
        .modules
        .first()
        .ok_or("the adapter project renders no module")?
        .lean_text;
    std::fs::write(temp.path().join("Tool.lean"), text).map_err(|error| error.to_string())?;
    let (code, output) = run_lean(temp.path(), &["Tool.lean".to_owned()])?;
    if code != 0 {
        return Err(format!(
            "Lean refused the generated adapter ({code}):\n{output}"
        ));
    }
    output
        .lines()
        .map(|line| {
            line.split_once(' ')
                .map(|(class, name)| (class.to_owned(), name.to_owned()))
                .ok_or_else(|| format!("an unexpected line of the listing: {line}"))
        })
        .collect()
}

/// What the elaborated adapter uses that no row declares: the plumbing, and
/// the requests that would declare the rest. A function of Lean's own
/// namespace the elaborator reached (the defaults of a structure instance
/// `{ }` run through them) is a call with its exact signature, like one a term
/// names; a type of Lean's namespace the elaborator reached is a registered
/// type; every other constant is plumbing.
fn classify(rows: &[Json], used: &[(String, String)]) -> (Vec<String>, BTreeSet<String>) {
    let declared: BTreeSet<&str> = rows.iter().filter_map(|row| row["name"].as_str()).collect();
    let constructors: BTreeSet<String> = rows
        .iter()
        .filter(|row| row["kind"] == "type")
        .flat_map(|row| {
            let owner = row["name"].as_str().unwrap_or_default().to_owned();
            row["constructors"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(move |constructor| {
                    constructor["name"]
                        .as_str()
                        .map(|short| format!("{owner}.{short}"))
                })
        })
        .collect();
    let mut plumbing = BTreeSet::new();
    let mut missing = BTreeSet::new();
    for (class, name) in used {
        if declared.contains(name.as_str()) || constructors.contains(name) {
            continue;
        }
        let lean = name == "Lean" || name.starts_with("Lean.");
        match (class.as_str(), lean) {
            ("call", _) => {
                missing.insert(format!("call:{name}"));
            }
            ("type", true) => {
                missing.insert(format!("opaque:{name}"));
            }
            _ => {
                plumbing.insert(name.clone());
            }
        }
    }
    (plumbing.into_iter().collect(), missing)
}

/// The registry the pinned Lean gives: the rows of the adapter's requests and
/// the plumbing of the adapter they generate.
///
/// # Errors
///
/// The reason the toolchain, the exporter, or the generated adapter failed.
pub fn derived(root: &Path) -> Result<(Vec<Json>, Vec<String>), String> {
    let mut requests: BTreeSet<String> = seeds().into_iter().collect();
    loop {
        let rows = export(root, &requests)?;
        let (plumbing, missing) = classify(&rows, &listing(&rows)?);
        if missing.is_empty() {
            return Ok((rows, plumbing));
        }
        let before = requests.len();
        requests.extend(missing);
        if requests.len() == before {
            return Err("the adapter's constants do not close over the requests".to_owned());
        }
    }
}

/// Regenerate `tests/lcnf/registry.json` from the pinned Lean.
///
/// # Errors
///
/// The reason the toolchain, the exporter, or the generated adapter failed.
pub fn regenerate(root: &Path) -> Result<(), String> {
    let (rows, plumbing) = derived(root)?;
    std::fs::write(root.join(REGISTRY_PATH), registry_bytes(&rows, &plumbing))
        .map_err(|error| format!("{REGISTRY_PATH}: {error}"))
}

/// The files of the committed tooling project, over the committed registry.
///
/// # Errors
///
/// The reason the committed registry cannot be read.
pub fn generated(root: &Path) -> Result<BTreeMap<String, Vec<u8>>, String> {
    let (rows, plumbing) = committed(root)?;
    Ok(files(&rows, &plumbing))
}
