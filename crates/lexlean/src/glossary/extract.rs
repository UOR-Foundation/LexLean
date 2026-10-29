//! The Lean half of glossary generation: the metaprogram that reads
//! declarations from a Lean environment, its output document, and the child
//! process that runs it under the pinned toolchain (SPEC.md §13.12, §25.4).

use std::collections::BTreeMap;

use camino::{Utf8Path, Utf8PathBuf};
use serde::Deserialize;

use crate::code;
use crate::config::Limits;
use crate::diagnostic::Diagnostic;
use crate::verify::child::{run as run_child, ChildHome, ChildSpec, Normalizer};
use crate::verify::toolchain;

/// The extractor's source. It is compiled into the executable, so the Lean
/// that reads a library is the Lean this release shipped with, and is written
/// to a private temporary file for each run.
pub const EXTRACTOR: &str = include_str!("../../lean/GlossaryExtract.lean");

/// What a request names: every declaration of some modules, or declarations
/// by name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selection {
    /// Every declaration the named modules define.
    Modules(Vec<String>),
    /// The named declarations.
    Names(Vec<String>),
}

/// Why the extractor refused one declaration.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Refusal {
    /// The stable reason, which the planner maps to a registered code.
    pub reason: String,
    /// The human detail.
    pub detail: String,
}

/// One constructor row of an eliminator descriptor.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ElimConstructor {
    /// The constructor's Lean name.
    pub name: String,
    /// Field binder names, in order.
    pub fields: Vec<String>,
    /// Induction-hypothesis binder names, in order.
    pub induction_hypotheses: Vec<String>,
}

/// An eliminator as the extractor found it.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Elim {
    /// The `cases` eliminator's Lean name.
    pub cases: String,
    /// The `induction` eliminator's Lean name.
    pub induction: String,
    /// The constructors, in declaration order.
    pub constructors: Vec<ElimConstructor>,
}

/// One declaration of the extraction.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Decl {
    /// The Lean name.
    pub name: String,
    /// The module that defines it.
    #[serde(default)]
    pub module: String,
    /// `theorem`, `definition`, `axiom`, `opaque`, `inductive`, or
    /// `constructor`.
    #[serde(default)]
    pub kind: String,
    /// Did the request name it (as opposed to a dependency the closure added)?
    pub requested: bool,
    /// The LSE signature, with `(const @Lean.Name ...)` placeholders for every
    /// constant that is not a core entry.
    pub signature: Option<String>,
    /// `proof`, `type`, `predicate`, or `term`.
    pub class: Option<String>,
    /// The universe variables the signature uses, sorted.
    #[serde(default)]
    pub universes: Vec<String>,
    /// The number of elaboration annotations erased from the type.
    #[serde(default)]
    pub erased: u64,
    /// The Lean names the signature mentions, sorted.
    #[serde(default)]
    pub refs: Vec<String>,
    /// Why there is no signature.
    pub refusal: Option<Refusal>,
    /// The eliminator, for an inductive type that has one.
    pub eliminator: Option<Elim>,
    /// Why an inductive type has no eliminator.
    pub eliminator_omitted: Option<String>,
}

/// The extractor's whole document.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Extraction {
    /// Every declaration, requested or reached as a dependency, by name.
    pub declarations: Vec<Decl>,
    /// Requested declarations the extractor set aside, by kind.
    pub excluded: BTreeMap<String, u64>,
}

impl Extraction {
    /// Read the extractor's document.
    pub fn parse(bytes: &[u8]) -> Result<Self, Diagnostic> {
        serde_json::from_slice(bytes).map_err(|error| {
            Diagnostic::new(
                code!("LLV7002"),
                format!("the glossary extractor's output is not its document: {error}"),
            )
        })
    }
}

/// The resource policy of an extraction: a library module is millions of
/// bytes of JSON and minutes of work, so the child limits are wider than a
/// document's.
const LIMITS: Limits = Limits {
    max_file_bytes: 4_194_304,
    max_total_source_bytes: 67_108_864,
    max_primitive_atoms: 2_000_000,
    max_token_lattice_edges: 4_000_000,
    max_parse_states: 4_000_000,
    max_ir_nodes: 2_000_000,
    max_scope_depth: 1024,
    max_import_depth: 128,
    max_diagnostics: 256,
    max_child_output_bytes: 1_073_741_824,
    child_timeout_ms: 3_600_000,
};

/// Run the extractor under the pinned toolchain and return its document.
///
/// With a `workspace`, the extractor runs as `lake env lean --run` in that
/// Lake workspace, whose dependencies must already be present and built
/// (nothing is fetched or built here, I15); without one it runs as plain
/// `lean --run` and sees the toolchain's own libraries.
pub fn run(selection: &Selection, workspace: Option<&Utf8Path>) -> Result<Extraction, Diagnostic> {
    run_with(selection, workspace, None)
}

/// [`run`] with one more directory of compiled Lean modules on the search
/// path. The command line has no such option: it exists so the repository's
/// own cases can extract from a module they compile themselves.
pub fn run_with(
    selection: &Selection,
    workspace: Option<&Utf8Path>,
    lean_path: Option<&Utf8Path>,
) -> Result<Extraction, Diagnostic> {
    let toolchain = toolchain::preflight(&LIMITS)?;
    let scratch = tempfile::Builder::new()
        .prefix("lexlean-glossary-")
        .tempdir()
        .map_err(|error| {
            Diagnostic::new(
                code!("LLS8001"),
                format!("cannot create the extractor's scratch directory: {error}"),
            )
        })?;
    let scratch_path = Utf8PathBuf::from_path_buf(scratch.path().to_path_buf())
        .map_err(|bad| crate::project::non_utf8_path(&bad))?;
    let script = scratch_path.join("GlossaryExtract.lean");
    std::fs::write(script.as_std_path(), EXTRACTOR).map_err(|error| {
        Diagnostic::new(
            code!("LLS8001"),
            format!("cannot write the extractor: {error}"),
        )
    })?;
    let mut flags = Vec::new();
    match selection {
        Selection::Modules(modules) => {
            flags.push("--modules".to_owned());
            flags.extend(modules.iter().cloned());
        }
        Selection::Names(names) => {
            flags.push("--names".to_owned());
            flags.extend(names.iter().cloned());
        }
    }
    let (program, mut argv, cwd): (&Utf8Path, Vec<String>, &Utf8Path) = match workspace {
        Some(workspace) => (
            &toolchain.lake.path,
            vec!["env".to_owned(), "lean".to_owned()],
            workspace,
        ),
        None => (&toolchain.lean.path, Vec::new(), &scratch_path),
    };
    let executable_sha256 = if workspace.is_some() {
        toolchain.lake.sha256
    } else {
        toolchain.lean.sha256
    };
    argv.extend(["--run".to_owned(), script.to_string(), "--".to_owned()]);
    argv.extend(flags);
    let bin = toolchain.root.join("bin");
    let record = run_child(
        &ChildSpec {
            tool: "glossary-extract",
            module: None,
            program,
            executable_sha256,
            argv,
            cwd,
            extra_env: lean_path
                .map(|path| vec![("LEAN_PATH".to_owned(), path.to_string())])
                .unwrap_or_default(),
            home: ChildHome::Toolchain {
                toolchain_bin: &bin,
            },
        },
        &LIMITS,
        &Normalizer::default(),
    )?;
    if record.exit_code != 0 {
        return Err(Diagnostic::new(
            code!("LLV7002"),
            format!(
                "the glossary extractor failed with exit code {}",
                record.exit_code
            ),
        )
        .with_cause(record.stderr.trim_end().to_owned()));
    }
    Extraction::parse(record.stdout.as_bytes())
}
