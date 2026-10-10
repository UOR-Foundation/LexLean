//! Named-root extraction through Lean's compiler front end (SPEC.md §22.10).
//!
//! Lean is the authority for how a verified generated declaration compiles.
//! The pinned adapter `language/lcnf-1.2/extract.lean` reports facts and
//! nothing else: for everything reachable from the roots, each constant's
//! kind, module, computability, and kernel-level uses, and, for every
//! code-generating definition of the project's own modules reached through
//! code, its base-phase LCNF. This module owns every decision about those
//! facts: the driver that pins the adapter, probes every authority signature
//! exactly, and checks the adapter's own constants against the closed
//! registry; the closed reading of the record; each root's computational
//! closure, its runtime members, and its erased proofs; the fail-closed
//! rejection classes; the comparison against the production-eligibility
//! closure; and the canonical compiler input with its content identity. No
//! optimization policy lives on either side of the boundary; the adapter runs
//! no LCNF pass after translation.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use super::ModuleReport;
use crate::artifact::content_id::Sha256Digest;

/// The embedded adapter, hashed into the language-1.2 compiler semantics ID.
pub const ADAPTER_PATH: &str = "language/lcnf-1.2/extract.lean";

/// The embedded authority registry, hashed into the same ID.
pub const AUTHORITY_PATH: &str = "language/lcnf-1.2/authority.toml";

/// The tag of the adapter's raw output.
pub const EXTRACTION_SPEC: &str = "lexlean/lcnf-extraction/2";

/// The tag of the canonical compiler input.
pub const INPUT_SPEC: &str = "lexlean/compiler-input/2";

/// The Lean namespaces of the fixed runtimes every generated module may
/// carry, below the module's own name.
pub const RUNTIME_NAMESPACES: [&str; 2] = ["LexLeanRuntime", "LexLeanCollections"];

/// One Lean operation the adapter calls.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityCall {
    pub name: String,
    pub signature: String,
    pub source: String,
    pub source_sha256: String,
    pub role: String,
}

/// One Lean data type the adapter matches exhaustively.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityType {
    pub name: String,
    pub source: String,
    pub source_sha256: String,
    pub constructors: Vec<String>,
}

/// The closed compiler-front-end interface: every constant the adapter's
/// own definitions use is exactly one call, one type, or one plumbing name.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Authority {
    pub spec: String,
    pub authority: String,
    pub lean_version: String,
    pub lean_githash: String,
    /// Structural plumbing the adapter elaborates to: constructors,
    /// projections, recursors, instances, and core-library data, whose
    /// meaning is fixed by the types and calls registered beside them.
    pub plumbing: Vec<String>,
    pub call: Vec<AuthorityCall>,
    #[serde(rename = "type")]
    pub types: Vec<AuthorityType>,
}

fn embedded_text(path: &str) -> Result<&'static str, String> {
    crate::embedded::FILES
        .iter()
        .find(|(candidate, _)| *candidate == path)
        .and_then(|(_, bytes)| std::str::from_utf8(bytes).ok())
        .ok_or_else(|| format!("embedded `{path}` is missing"))
}

/// The embedded adapter source.
///
/// # Errors
///
/// Returns the reason the embedded adapter is missing.
pub fn adapter() -> Result<&'static str, String> {
    embedded_text(ADAPTER_PATH)
}

/// The embedded authority registry.
///
/// # Errors
///
/// Returns the reason the embedded registry is missing or malformed; the
/// conformance suite parses it, so a malformed registry never ships.
pub fn authority() -> Result<&'static Authority, String> {
    static AUTHORITY: OnceLock<Result<Authority, String>> = OnceLock::new();
    AUTHORITY
        .get_or_init(|| {
            let text = embedded_text(AUTHORITY_PATH)?;
            let authority: Authority =
                toml::from_str(text).map_err(|error| format!("{AUTHORITY_PATH}: {error}"))?;
            if authority.spec != "lexlean/lcnf-authority/2" {
                return Err(format!(
                    "{AUTHORITY_PATH}: unsupported spec `{}`",
                    authority.spec
                ));
            }
            let mut seen = BTreeSet::new();
            for name in authority
                .call
                .iter()
                .map(|call| &call.name)
                .chain(authority.types.iter().map(|row| &row.name))
                .chain(&authority.plumbing)
            {
                if !seen.insert(name.clone()) {
                    return Err(format!("{AUTHORITY_PATH}: `{name}` is registered twice"));
                }
            }
            Ok(authority)
        })
        .as_ref()
        .map_err(Clone::clone)
}

/// A Lean name literal that survives any segment spelling: the name is
/// rebuilt from its string, so no source quoting rule is involved.
fn name_literal(name: &str) -> String {
    let escaped: String = name
        .chars()
        .flat_map(|character| match character {
            '"' => vec!['\\', '"'],
            '\\' => vec!['\\', '\\'],
            other => vec![other],
        })
        .collect();
    format!("\"{escaped}\".toName")
}

/// A Lean string literal.
fn string_literal(text: &str) -> String {
    let escaped: String = text
        .chars()
        .flat_map(|character| match character {
            '"' => vec!['\\', '"'],
            '\\' => vec!['\\', '\\'],
            other => vec![other],
        })
        .collect();
    format!("\"{escaped}\"")
}

/// The universe variables (`u_1`, `u_2`, …) a registered signature names,
/// which the driver declares so the signature elaborates as written.
fn signature_universes(signature: &str, out: &mut BTreeSet<String>) {
    let bytes = signature.as_bytes();
    let mut index = 0;
    while let Some(offset) = signature[index..].find("u_") {
        let start = index + offset;
        let mut end = start + 2;
        while end < bytes.len() && bytes[end].is_ascii_digit() {
            end += 1;
        }
        let bounded = start == 0
            || !(bytes[start - 1].is_ascii_alphanumeric()
                || bytes[start - 1] == b'_'
                || bytes[start - 1] == b'.');
        if bounded && end > start + 2 {
            out.insert(signature[start..end].to_owned());
        }
        index = end.max(start + 2);
    }
}

/// The generated extraction module: fixed header, the pinned adapter, one
/// exact signature probe per authority call, the check that the adapter's
/// constants are exactly the registry's, and one command naming the roots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Driver {
    /// The reserved module name (`LexLeanExtract.X<hex32>`).
    pub name: String,
    pub text: String,
    /// The 1-based line of each call probe, by call name.
    pub probe_lines: BTreeMap<usize, String>,
    /// The 1-based line of the registry closure check.
    pub authority_line: usize,
    /// The 1-based lines the pinned adapter occupies.
    pub adapter_lines: (usize, usize),
}

/// The reserved extraction module name for a semantic ID prefix.
#[must_use]
pub fn driver_name(semantic_hex32: &str) -> String {
    format!("LexLeanExtract.X{semantic_hex32}")
}

/// Whether a qualified name is a member of a generated module's runtime.
#[must_use]
pub fn is_runtime_member(name: &str, modules: &BTreeSet<String>) -> bool {
    modules.iter().any(|module| {
        RUNTIME_NAMESPACES
            .iter()
            .any(|namespace| name.starts_with(&format!("{module}.{namespace}.")))
    })
}

/// Generate the extraction module.
///
/// # Errors
///
/// Returns the reason the embedded adapter or registry is unavailable.
pub fn driver(
    semantic_hex32: &str,
    roots: &[String],
    modules: &[String],
) -> Result<Driver, String> {
    let authority = authority()?;
    let adapter = adapter()?;
    let mut text = String::from("module\npublic meta import Lean\n");
    // `import all` exposes the private kernel values, so a proof reached
    // only through a compiler-generated helper is still recorded.
    for module in modules {
        text.push_str(&format!("import all {module}\n"));
    }
    // Lean reports every message on standard output, where the record must
    // stand alone; a registered signature binds instance arguments by name.
    text.push_str("set_option linter.unusedVariables false\n");
    let adapter_start = text.lines().count() + 1;
    text.push_str(adapter);
    if !text.ends_with('\n') {
        text.push('\n');
    }
    let adapter_end = text.lines().count();
    let mut universes = BTreeSet::new();
    for call in &authority.call {
        signature_universes(&call.signature, &mut universes);
    }
    if !universes.is_empty() {
        text.push_str(&format!(
            "universe {}\n",
            universes.into_iter().collect::<Vec<_>>().join(" ")
        ));
    }
    let mut probe_lines = BTreeMap::new();
    for call in &authority.call {
        probe_lines.insert(text.lines().count() + 1, call.name.clone());
        text.push_str(&format!(
            "lexlean_signature {} : {}\n",
            call.name, call.signature
        ));
    }
    let list = |names: &mut dyn Iterator<Item = &String>| {
        format!(
            "#[{}]",
            names
                .map(|name| name_literal(name))
                .collect::<Vec<_>>()
                .join(", ")
        )
    };
    let types = authority
        .types
        .iter()
        .map(|row| {
            format!(
                "({}, #[{}])",
                name_literal(&row.name),
                row.constructors
                    .iter()
                    .map(|constructor| string_literal(constructor))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    let authority_line = text.lines().count() + 1;
    text.push_str(&format!(
        "#eval LexLeanExtract.checkAuthority {} #[{types}] {}\n",
        list(&mut authority.call.iter().map(|call| &call.name)),
        list(&mut authority.plumbing.iter())
    ));
    text.push_str(&format!(
        "#eval LexLeanExtract.main {} {}\n",
        list(&mut roots.iter()),
        list(&mut modules.iter())
    ));
    Ok(Driver {
        name: driver_name(semantic_hex32),
        text,
        probe_lines,
        authority_line,
        adapter_lines: (adapter_start, adapter_end),
    })
}

/// A rejected extraction: drift of the pinned authority, or a fail-closed
/// rejection of the extracted program.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rejection {
    /// `LLV7012`: the authority no longer matches its registry.
    Drift(String),
    /// `LLV7011`: the extraction is not an admissible compiler input.
    Rejected(String),
}

/// The drift a Lean message shows, if any: any message on a probe line, on
/// the registry check, or inside the pinned adapter, and any message the
/// adapter raises as drift.
fn drift_of(driver: &Driver, message: &crate::verify::LeanMessage) -> Option<String> {
    if let Some(call) = driver.probe_lines.get(&message.line) {
        return Some(format!(
            "the signature of `{call}` no longer matches the pinned registry: {}",
            message.message
        ));
    }
    if message.line == driver.authority_line || message.message.contains("lexlean-extract-drift") {
        return Some(format!(
            "the adapter's constants no longer match the pinned registry: {}",
            message.message
        ));
    }
    if (driver.adapter_lines.0..=driver.adapter_lines.1).contains(&message.line) {
        return Some(format!(
            "the pinned extraction adapter no longer elaborates cleanly: {}",
            message.message
        ));
    }
    None
}

/// Lean's messages in an extraction's output, without the record line a
/// later command may still have printed.
fn messages_of(output: &str) -> Vec<crate::verify::LeanMessage> {
    let messages: String = output
        .lines()
        .filter(|line| !line.starts_with("{\"spec\":"))
        .map(|line| format!("{line}\n"))
        .collect();
    crate::verify::parse_lean_messages(&messages)
}

/// Classify a failed extraction process from its Lean messages: drift of
/// the authority first, then a rejection the adapter raised by name.
#[must_use]
pub fn classify_failure(driver: &Driver, output: &str) -> Rejection {
    let messages = messages_of(output);
    if let Some(drift) = messages
        .iter()
        .find_map(|message| drift_of(driver, message))
    {
        return Rejection::Drift(drift);
    }
    if let Some(raised) = output
        .lines()
        .filter(|line| !line.starts_with("{\"spec\":"))
        .find_map(|line| line.split_once("lexlean-extract: "))
    {
        return Rejection::Rejected(raised.1.trim().to_owned());
    }
    Rejection::Rejected(format!(
        "the extraction process failed: {}",
        output.trim_end()
    ))
}

/// An LCNF type.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "snake_case",
    deny_unknown_fields,
    from = "WireType"
)]
pub enum LcnfType {
    Erased,
    Any,
    Const {
        name: String,
        /// The universe levels the constant is instantiated at.
        levels: Vec<String>,
    },
    App {
        head: Box<LcnfType>,
        arguments: Vec<LcnfType>,
    },
    Arrow {
        domain: Box<LcnfType>,
        codomain: Box<LcnfType>,
    },
    Fvar {
        id: String,
    },
    Bvar {
        index: u64,
    },
    Sort {
        level: String,
    },
    Unsupported {
        expression: String,
    },
}

/// An LCNF type as the adapter reports it. Lean's `toLCNFType` marks the
/// domain of an arrow whose parameter is borrowed with the `borrowed`
/// annotation (`Lean/Compiler/LCNF/Types.lean`), the only metadata an LCNF
/// type carries; LCNF type equivalence (`eqvTypes`,
/// `Lean/Compiler/LCNF/InferType.lean`) looks through it, and the ownership
/// hint it records is the parameter's own `borrow` fact. It is unwrapped on
/// reading, so no canonical type carries it.
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum WireType {
    Erased,
    Any,
    Const {
        name: String,
        levels: Vec<String>,
    },
    App {
        head: Box<LcnfType>,
        arguments: Vec<LcnfType>,
    },
    Arrow {
        domain: Box<LcnfType>,
        codomain: Box<LcnfType>,
    },
    Fvar {
        id: String,
    },
    Bvar {
        index: u64,
    },
    Sort {
        level: String,
    },
    Borrowed {
        #[serde(rename = "type")]
        ty: Box<LcnfType>,
    },
    Unsupported {
        expression: String,
    },
}

impl From<WireType> for LcnfType {
    fn from(wire: WireType) -> Self {
        match wire {
            WireType::Erased => Self::Erased,
            WireType::Any => Self::Any,
            WireType::Const { name, levels } => Self::Const { name, levels },
            WireType::App { head, arguments } => Self::App { head, arguments },
            WireType::Arrow { domain, codomain } => Self::Arrow { domain, codomain },
            WireType::Fvar { id } => Self::Fvar { id },
            WireType::Bvar { index } => Self::Bvar { index },
            WireType::Sort { level } => Self::Sort { level },
            WireType::Borrowed { ty } => *ty,
            WireType::Unsupported { expression } => Self::Unsupported { expression },
        }
    }
}

/// An LCNF argument.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum LcnfArg {
    Erased,
    Fvar {
        id: String,
    },
    Type {
        #[serde(rename = "type")]
        ty: LcnfType,
    },
}

/// An LCNF literal; values are decimal strings.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum LcnfLiteral {
    Nat { value: String },
    String { value: String },
    Uint8 { value: String },
    Uint16 { value: String },
    Uint32 { value: String },
    Uint64 { value: String },
    Usize { value: String },
}

/// An LCNF let value.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum LcnfValue {
    Literal {
        literal: LcnfLiteral,
    },
    Erased,
    Projection {
        type_name: String,
        index: u64,
        value: String,
    },
    Const {
        name: String,
        levels: Vec<String>,
        arguments: Vec<LcnfArg>,
    },
    Apply {
        function: String,
        arguments: Vec<LcnfArg>,
    },
}

/// An LCNF parameter.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LcnfParam {
    pub id: String,
    #[serde(rename = "type")]
    pub ty: LcnfType,
    pub borrow: bool,
}

/// An LCNF local function or join point.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LcnfFunDecl {
    pub id: String,
    pub parameters: Vec<LcnfParam>,
    #[serde(rename = "type")]
    pub ty: LcnfType,
    pub value: LcnfCode,
}

/// An LCNF case alternative.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum LcnfAlt {
    Constructor {
        constructor: String,
        parameters: Vec<LcnfParam>,
        code: LcnfCode,
    },
    Default {
        code: LcnfCode,
    },
}

/// LCNF code.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum LcnfCode {
    Let {
        id: String,
        #[serde(rename = "type")]
        ty: LcnfType,
        value: LcnfValue,
        body: Box<LcnfCode>,
    },
    Fun {
        declaration: Box<LcnfFunDecl>,
        body: Box<LcnfCode>,
    },
    Join {
        declaration: Box<LcnfFunDecl>,
        body: Box<LcnfCode>,
    },
    Jump {
        target: String,
        arguments: Vec<LcnfArg>,
    },
    Cases {
        type_name: String,
        result_type: LcnfType,
        discriminant: String,
        alternatives: Vec<LcnfAlt>,
    },
    Return {
        id: String,
    },
    Unreachable {
        #[serde(rename = "type")]
        ty: LcnfType,
    },
}

/// An LCNF declaration body.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum LcnfDeclValue {
    Code { code: LcnfCode },
    Extern,
}

/// The translation the adapter reports for a code-generating definition.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawTranslation {
    safe: bool,
    level_parameters: Vec<String>,
    #[serde(rename = "type")]
    ty: LcnfType,
    parameters: Vec<LcnfParam>,
    value: LcnfDeclValue,
    /// Every constant the translation names.
    uses: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawConstructor {
    name: String,
    parameters: u64,
    fields: u64,
    #[serde(rename = "type")]
    ty: LcnfType,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawInductive {
    parameters: u64,
    indices: u64,
    recursive: bool,
    constructors: Vec<RawConstructor>,
}

/// One constant of the project's modules, as Lean reports it.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawConstant {
    name: String,
    kind: String,
    module: String,
    computable: bool,
    generates_code: bool,
    /// The kind it was declared with, which a project module imported in
    /// full reports unchanged.
    original_kind: String,
    internal: bool,
    /// The constants its kernel value names.
    kernel_uses: Vec<String>,
    declaration: Option<RawTranslation>,
    inductive: Option<RawInductive>,
}

/// One constant outside the project's modules that translated code names.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawExternal {
    name: String,
    kind: String,
    module: String,
    computable: bool,
    generates_code: bool,
    /// The kind it was declared with. The module system exports a
    /// definition whose body its module does not expose as an axiom; this
    /// is what it was before that.
    original_kind: String,
    /// Whether Lean's compiler holds code for it (`Lean.IR.findEnvDecl`).
    compiled: bool,
    internal: bool,
    #[serde(default)]
    inductive_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LcnfConstructor {
    pub name: String,
    pub index: u64,
    pub parameters: u64,
    pub fields: u64,
    #[serde(rename = "type")]
    pub ty: LcnfType,
}

/// A project inductive the closure realizes.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LcnfInductive {
    pub name: String,
    pub parameters: u64,
    pub indices: u64,
    pub recursive: bool,
    pub constructors: Vec<LcnfConstructor>,
}

/// A constant the closure uses but does not define.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LcnfExternal {
    pub name: String,
    pub kind: String,
    pub module: String,
    pub computable: bool,
    pub generates_code: bool,
}

/// One instance of the eligibility analysis's monomorphization plan.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LcnfInstance {
    pub instance: String,
    pub declaration: String,
    pub type_arguments: Vec<String>,
}

/// One root's closure as Lean's compiler reaches it.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LcnfClosure {
    pub root: String,
    /// Source definitions, equal to the eligibility closure.
    pub declarations: Vec<String>,
    /// Members of the generated runtimes the source definitions reach.
    pub runtime: Vec<String>,
    /// The instances at which the source definitions are realized.
    pub instances: Vec<LcnfInstance>,
    pub erased: Vec<String>,
    pub internal_erased: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LeanIdentity {
    pub version: String,
    pub githash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawExtraction {
    spec: String,
    lean: LeanIdentity,
    roots: Vec<String>,
    constants: Vec<RawConstant>,
    externals: Vec<RawExternal>,
}

/// A verified code-generating definition in canonical form.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LcnfDeclaration {
    pub name: String,
    pub recursive: bool,
    pub level_parameters: Vec<String>,
    #[serde(rename = "type")]
    pub ty: LcnfType,
    pub parameters: Vec<LcnfParam>,
    pub code: LcnfCode,
}

/// The canonical compiler input (`lexlean/compiler-input/2`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompilerInput {
    pub spec: String,
    pub lean: LeanIdentity,
    pub roots: Vec<String>,
    pub closures: Vec<LcnfClosure>,
    pub declarations: Vec<LcnfDeclaration>,
    pub inductives: Vec<LcnfInductive>,
    pub externals: Vec<LcnfExternal>,
    pub erased: Vec<String>,
}

impl CompilerInput {
    /// The canonical file bytes.
    ///
    /// # Panics
    ///
    /// Panics only if `serde_json` produces text the canonical JSON parser
    /// rejects, which would be an internal invariant failure.
    #[must_use]
    pub fn to_file_bytes(&self) -> Vec<u8> {
        // The code of a match nests one level for each of its arms: the
        // input is serialized, parsed without a limit on nesting, written,
        // and dropped on a stack sized for it.
        let work = || {
            let text = serde_json::to_string(self).expect("compiler input serializes");
            crate::artifact::canonical_json::Json::parse_unbounded(text.as_bytes())
                .expect("compiler input is JSON")
                .to_file_bytes()
        };
        std::thread::scope(|scope| {
            match std::thread::Builder::new()
                .stack_size(1 << 30)
                .spawn_scoped(scope, work)
            {
                Ok(handle) => handle.join().expect("the compiler input serializes"),
                Err(_) => work(),
            }
        })
    }

    /// The content identity: the SHA-256 of the canonical bytes.
    #[must_use]
    pub fn id(&self) -> Sha256Digest {
        Sha256Digest::of(&self.to_file_bytes())
    }
}

/// Renames every LCNF free variable to its first-occurrence index, so the
/// canonical input does not depend on Lean's unique-name counter. Scope is
/// lexical: a variable bound in one alternative, local function, or join
/// point is not visible in a sibling.
#[derive(Default)]
struct Renamer {
    names: BTreeMap<String, String>,
    bound: BTreeSet<String>,
    counter: usize,
}

impl Renamer {
    fn bind(&mut self, id: &mut String) -> Result<(), String> {
        let fresh = format!("v{}", self.counter);
        self.counter += 1;
        if !self.bound.insert(id.clone()) {
            return Err(format!("LCNF variable `{id}` is bound twice"));
        }
        self.names.insert(id.clone(), fresh.clone());
        *id = fresh;
        Ok(())
    }

    /// Run `body` in a nested scope: its bindings end with it.
    fn scoped(&mut self, body: impl FnOnce(&mut Self) -> Result<(), String>) -> Result<(), String> {
        let saved = self.names.clone();
        let result = body(self);
        self.names = saved;
        result
    }

    fn reference(&self, id: &mut String) -> Result<(), String> {
        let renamed = self
            .names
            .get(id)
            .ok_or_else(|| format!("LCNF variable `{id}` is used before it is bound"))?;
        id.clone_from(renamed);
        Ok(())
    }

    fn ty(&self, ty: &mut LcnfType) -> Result<(), String> {
        match ty {
            LcnfType::Erased
            | LcnfType::Any
            | LcnfType::Const { .. }
            | LcnfType::Bvar { .. }
            | LcnfType::Sort { .. } => Ok(()),
            LcnfType::App { head, arguments } => {
                self.ty(head)?;
                arguments.iter_mut().try_for_each(|argument| self.ty(argument))
            }
            LcnfType::Arrow { domain, codomain } => {
                self.ty(domain)?;
                self.ty(codomain)
            }
            LcnfType::Fvar { id } => self.reference(id),
            LcnfType::Unsupported { expression } => Err(format!(
                "unsupported compiler form: the LCNF type `{expression}` has no closed representation"
            )),
        }
    }

    fn arg(&self, arg: &mut LcnfArg) -> Result<(), String> {
        match arg {
            LcnfArg::Erased => Ok(()),
            LcnfArg::Fvar { id } => self.reference(id),
            LcnfArg::Type { ty } => self.ty(ty),
        }
    }

    fn params(&mut self, params: &mut [LcnfParam]) -> Result<(), String> {
        for param in params {
            self.ty(&mut param.ty)?;
            self.bind(&mut param.id)?;
        }
        Ok(())
    }

    fn fun(&mut self, declaration: &mut LcnfFunDecl) -> Result<(), String> {
        self.bind(&mut declaration.id)?;
        let LcnfFunDecl {
            parameters,
            ty,
            value,
            ..
        } = declaration;
        self.scoped(|renamer| {
            renamer.params(parameters)?;
            renamer.ty(ty)?;
            renamer.code(value)
        })
    }

    fn code(&mut self, code: &mut LcnfCode) -> Result<(), String> {
        match code {
            LcnfCode::Let {
                id,
                ty,
                value,
                body,
            } => {
                self.ty(ty)?;
                match value {
                    LcnfValue::Literal { .. } | LcnfValue::Erased => {}
                    LcnfValue::Projection { value, .. } => self.reference(value)?,
                    LcnfValue::Const { arguments, .. } => {
                        arguments.iter_mut().try_for_each(|arg| self.arg(arg))?;
                    }
                    LcnfValue::Apply {
                        function,
                        arguments,
                    } => {
                        self.reference(function)?;
                        arguments.iter_mut().try_for_each(|arg| self.arg(arg))?;
                    }
                }
                self.bind(id)?;
                self.code(body)
            }
            LcnfCode::Fun { declaration, body } | LcnfCode::Join { declaration, body } => {
                self.fun(declaration)?;
                self.code(body)
            }
            LcnfCode::Jump { target, arguments } => {
                self.reference(target)?;
                arguments.iter_mut().try_for_each(|arg| self.arg(arg))
            }
            LcnfCode::Cases {
                result_type,
                discriminant,
                alternatives,
                ..
            } => {
                self.ty(result_type)?;
                self.reference(discriminant)?;
                for alternative in alternatives {
                    match alternative {
                        LcnfAlt::Constructor {
                            parameters, code, ..
                        } => self.scoped(|renamer| {
                            renamer.params(parameters)?;
                            renamer.code(code)
                        })?,
                        LcnfAlt::Default { code } => {
                            self.scoped(|renamer| renamer.code(code))?;
                        }
                    }
                }
                Ok(())
            }
            LcnfCode::Return { id } => self.reference(id),
            LcnfCode::Unreachable { ty } => self.ty(ty),
        }
    }
}

/// Every constant a type mentions.
fn type_constants(ty: &LcnfType, out: &mut BTreeSet<String>) {
    match ty {
        LcnfType::Const { name, .. } => {
            out.insert(name.clone());
        }
        LcnfType::App { head, arguments } => {
            type_constants(head, out);
            for argument in arguments {
                type_constants(argument, out);
            }
        }
        LcnfType::Arrow { domain, codomain } => {
            type_constants(domain, out);
            type_constants(codomain, out);
        }
        LcnfType::Erased
        | LcnfType::Any
        | LcnfType::Fvar { .. }
        | LcnfType::Bvar { .. }
        | LcnfType::Sort { .. }
        | LcnfType::Unsupported { .. } => {}
    }
}

fn arg_constants(arg: &LcnfArg, out: &mut BTreeSet<String>) {
    match arg {
        LcnfArg::Type { ty } => type_constants(ty, out),
        LcnfArg::Erased | LcnfArg::Fvar { .. } => {}
    }
}

fn param_constants(params: &[LcnfParam], out: &mut BTreeSet<String>) {
    for param in params {
        type_constants(&param.ty, out);
    }
}

/// Every constant code mentions: its callees, constructors, matched and
/// projected types, and the types it states.
fn code_constants(code: &LcnfCode, out: &mut BTreeSet<String>) {
    match code {
        LcnfCode::Let {
            ty, value, body, ..
        } => {
            type_constants(ty, out);
            match value {
                LcnfValue::Literal { .. } | LcnfValue::Erased => {}
                LcnfValue::Projection { type_name, .. } => {
                    out.insert(type_name.clone());
                }
                LcnfValue::Const {
                    name, arguments, ..
                } => {
                    out.insert(name.clone());
                    for argument in arguments {
                        arg_constants(argument, out);
                    }
                }
                LcnfValue::Apply { arguments, .. } => {
                    for argument in arguments {
                        arg_constants(argument, out);
                    }
                }
            }
            code_constants(body, out);
        }
        LcnfCode::Fun { declaration, body } | LcnfCode::Join { declaration, body } => {
            param_constants(&declaration.parameters, out);
            type_constants(&declaration.ty, out);
            code_constants(&declaration.value, out);
            code_constants(body, out);
        }
        LcnfCode::Jump { arguments, .. } => {
            for argument in arguments {
                arg_constants(argument, out);
            }
        }
        LcnfCode::Cases {
            type_name,
            result_type,
            alternatives,
            ..
        } => {
            out.insert(type_name.clone());
            type_constants(result_type, out);
            for alternative in alternatives {
                match alternative {
                    LcnfAlt::Constructor {
                        constructor,
                        parameters,
                        code,
                    } => {
                        out.insert(constructor.clone());
                        param_constants(parameters, out);
                        code_constants(code, out);
                    }
                    LcnfAlt::Default { code } => code_constants(code, out),
                }
            }
        }
        LcnfCode::Return { .. } => {}
        LcnfCode::Unreachable { ty } => type_constants(ty, out),
    }
}

/// One root's closure as the eligibility analysis computed it.
struct EligibilityClosure {
    declarations: BTreeSet<String>,
    erased: BTreeSet<String>,
    instances: Vec<LcnfInstance>,
}

/// The eligibility closures of a set of reports, per qualified root.
fn eligibility_closures(reports: &[&ModuleReport]) -> BTreeMap<String, EligibilityClosure> {
    let mut out = BTreeMap::new();
    for report in reports {
        for root in &report.roots {
            out.insert(
                root.root.clone(),
                EligibilityClosure {
                    declarations: root
                        .runtime
                        .iter()
                        .map(|member| member.declaration.clone())
                        .collect(),
                    erased: root.erased.clone(),
                    instances: root
                        .runtime
                        .iter()
                        .map(|member| LcnfInstance {
                            instance: member.instance.clone(),
                            declaration: member.declaration.clone(),
                            type_arguments: member.type_arguments.clone(),
                        })
                        .collect(),
                },
            );
        }
    }
    out
}

fn unique<'a>(values: impl Iterator<Item = &'a String>) -> bool {
    let mut seen = BTreeSet::new();
    values.into_iter().all(|value| seen.insert(value))
}

/// Whether a constant outside the project is one LexLean may hand to a
/// compiler: a computable constant of Lean's own `Init` library.
///
/// A definition of a `module` whose body that module does not expose is
/// exported as an axiom (`Lean.Environment.setExporting`); Lean's code
/// generator decides such a reference by the kind the constant was declared
/// with (`checkComputable`, `Lean/Compiler/LCNF/ToLCNF.lean`), and so does
/// this. It is admitted as the definition it was declared as only when it is
/// computable and Lean's compiler holds its code; a constant declared an
/// axiom, an opaque, or anything other than a definition is not.
fn admissible_external(external: &RawExternal) -> Result<(), String> {
    if external.module != "Init" && !external.module.starts_with("Init.") {
        return Err(format!(
            "unresolved dependency: `{}` ({} from `{}`) is neither a Lean core constant nor a member of the project",
            external.name,
            external.kind,
            external.module
        ));
    }
    let weakened = external.kind == "axiom" && external.original_kind == "definition";
    if declared_kind(&external.kind) != external.original_kind && !weakened {
        return Err(format!(
            "the extraction record reports `{}` as {} declared as {}, which no exported view produces",
            external.name,
            article(&external.kind),
            article(&external.original_kind)
        ));
    }
    let kind = if weakened {
        "definition"
    } else {
        external.kind.as_str()
    };
    match kind {
        "inductive" | "constructor" => Ok(()),
        "definition"
            if external.computable
                && external.compiled
                && (weakened || external.generates_code) =>
        {
            Ok(())
        }
        "definition" => Err(format!(
            "`{}` is noncomputable or has no compiled code, so no compiler can realize it",
            external.name
        )),
        other => Err(format!(
            "`{}` is {} and has no executable content",
            external.name,
            article(other)
        )),
    }
}

/// The kind `getOriginalConstKind?` reports for a constant of a reported
/// kind: Lean's `ConstantKind` records an `unsafe`-marked definition as a definition.
fn declared_kind(kind: &str) -> &str {
    match kind {
        "unsafe-definition" => "definition",
        other => other,
    }
}

fn article(kind: &str) -> String {
    match kind {
        "axiom" | "opaque" | "unsafe-definition" | "inductive" => format!("an {kind}"),
        other => format!("a {other}"),
    }
}

/// Why a project constant reached through code is not a definition the
/// compiler can realize, if it is not.
fn unrealizable(constant: &RawConstant) -> Option<String> {
    let name = &constant.name;
    match constant.kind.as_str() {
        "theorem" => Some(format!(
            "proof-as-runtime dependency: the theorem `{name}` is in the runtime closure"
        )),
        "definition" if !constant.computable => Some(format!("`{name}` is noncomputable")),
        "definition" if !constant.generates_code => Some(format!("`{name}` generates no code")),
        "definition" => None,
        "unsafe-definition" => Some(format!("`{name}` is unsafe")),
        other => Some(format!(
            "`{name}` is {}, not a definition the compiler can realize",
            article(other)
        )),
    }
}

/// The Lean messages a successful extraction printed beside its record.
fn stray_output(driver: &Driver, output: &str) -> Option<Rejection> {
    let lines: Vec<&str> = output.trim_end_matches('\n').lines().collect();
    if lines.len() <= 1 {
        return None;
    }
    let messages = messages_of(output);
    if let Some(drift) = messages
        .iter()
        .find_map(|message| drift_of(driver, message))
    {
        return Some(Rejection::Drift(drift));
    }
    Some(Rejection::Rejected(
        "the extraction printed more than its single JSON record".to_owned(),
    ))
}

/// Validate the adapter's facts and produce the canonical compiler input.
///
/// `roots` are the qualified production roots in request order, `modules`
/// the generated modules, and `reports` the eligibility reports whose
/// closures Lean's must equal.
///
/// # Errors
///
/// Returns the first drift or rejection found; nothing partial is produced.
pub fn compiler_input(
    driver: &Driver,
    output: &str,
    roots: &[String],
    modules: &[String],
    reports: &[&ModuleReport],
) -> Result<CompilerInput, Rejection> {
    // Lean's code for a match nests one level for each of its arms, and the
    // record, parsed without a depth limit, is walked recursively: it is read
    // on a stack of its own, sized by the record, so no source can overflow
    // the caller's.
    let stack = output.len().saturating_mul(512).clamp(16 << 20, 4 << 30);
    std::thread::scope(|scope| {
        std::thread::Builder::new()
            .stack_size(stack)
            .spawn_scoped(scope, || {
                compiler_input_nested(driver, output, roots, modules, reports)
            })
            .map_err(|error| {
                Rejection::Rejected(format!("the extraction record cannot be read: {error}"))
            })?
            .join()
            .map_err(|_| Rejection::Rejected("the extraction record cannot be read".to_owned()))?
    })
}

#[allow(clippy::too_many_lines)]
fn compiler_input_nested(
    driver: &Driver,
    output: &str,
    roots: &[String],
    modules: &[String],
    reports: &[&ModuleReport],
) -> Result<CompilerInput, Rejection> {
    let authority = authority().map_err(Rejection::Drift)?;
    if let Some(rejection) = stray_output(driver, output) {
        return Err(rejection);
    }
    let payload = output.trim_end_matches('\n');
    let raw: RawExtraction = {
        let mut reader = serde_json::Deserializer::from_str(payload);
        reader.disable_recursion_limit();
        let parsed = serde::Deserialize::deserialize(&mut reader)
            .and_then(|value| reader.end().map(|()| value));
        parsed.map_err(|error: serde_json::Error| {
            Rejection::Rejected(format!("the extraction record is malformed: {error}"))
        })?
    };
    if raw.spec != EXTRACTION_SPEC {
        return Err(Rejection::Rejected(format!(
            "the extraction record has spec `{}`",
            raw.spec
        )));
    }
    if raw.lean.version != authority.lean_version || raw.lean.githash != authority.lean_githash {
        return Err(Rejection::Drift(format!(
            "Lean reports version {} at {}, but the authority is pinned to {} at {}",
            raw.lean.version, raw.lean.githash, authority.lean_version, authority.lean_githash
        )));
    }
    if raw.roots != roots {
        return Err(Rejection::Rejected(format!(
            "the extraction answered roots {:?} for requested roots {roots:?}",
            raw.roots
        )));
    }
    let module_set: BTreeSet<String> = modules.iter().cloned().collect();
    if !unique(raw.constants.iter().map(|constant| &constant.name))
        || !unique(raw.externals.iter().map(|external| &external.name))
    {
        return Err(Rejection::Rejected(
            "the extraction record reports a constant twice".to_owned(),
        ));
    }
    let constants: BTreeMap<&str, &RawConstant> = raw
        .constants
        .iter()
        .map(|constant| (constant.name.as_str(), constant))
        .collect();
    let externals: BTreeMap<&str, &RawExternal> = raw
        .externals
        .iter()
        .map(|external| (external.name.as_str(), external))
        .collect();
    for constant in &raw.constants {
        if !module_set.contains(&constant.module) {
            return Err(Rejection::Rejected(format!(
                "`{}` is reported as a project constant of `{}`, which is not a generated module",
                constant.name, constant.module
            )));
        }
        if constant.declaration.is_some()
            && (constant.kind != "definition" || !constant.computable || !constant.generates_code)
        {
            return Err(Rejection::Rejected(format!(
                "`{}` carries a translation, but it is {} that Lean does not compile",
                constant.name,
                article(&constant.kind)
            )));
        }
        if constant.original_kind != declared_kind(&constant.kind) {
            return Err(Rejection::Rejected(format!(
                "the extraction record reports the project constant `{}` as {} declared as {}",
                constant.name,
                article(&constant.kind),
                article(&constant.original_kind)
            )));
        }
    }
    for external in &raw.externals {
        if module_set.contains(&external.module) {
            return Err(Rejection::Rejected(format!(
                "`{}` is reported as external but belongs to the project",
                external.name
            )));
        }
    }

    // Every translation, renamed into canonical form once; a translation
    // that is not admissible is refused only if a closure reaches it.
    let mut translations: BTreeMap<&str, LcnfDeclaration> = BTreeMap::new();
    let mut refused: BTreeMap<&str, String> = BTreeMap::new();
    for constant in &raw.constants {
        let Some(translation) = &constant.declaration else {
            continue;
        };
        let name = &constant.name;
        if !translation.safe {
            refused.insert(name.as_str(), format!("`{name}` is unsafe"));
            continue;
        }
        let LcnfDeclValue::Code { code } = &translation.value else {
            refused.insert(
                name.as_str(),
                format!("unsupported compiler form: `{name}` is implemented externally"),
            );
            continue;
        };
        let mut renamer = Renamer::default();
        let mut ty = translation.ty.clone();
        let mut parameters = translation.parameters.clone();
        let mut code = code.clone();
        if let Err(reason) = renamer
            .ty(&mut ty)
            .and_then(|()| renamer.params(&mut parameters))
            .and_then(|()| renamer.code(&mut code))
        {
            refused.insert(name.as_str(), format!("`{name}`: {reason}"));
            continue;
        }
        // The record's own list of what the code names must be exactly
        // what the code names.
        let mut used = BTreeSet::new();
        type_constants(&ty, &mut used);
        param_constants(&parameters, &mut used);
        code_constants(&code, &mut used);
        let listed: BTreeSet<String> = translation.uses.iter().cloned().collect();
        if used != listed {
            return Err(Rejection::Rejected(format!(
                "the extraction record of `{name}` lists uses that differ from its code"
            )));
        }
        translations.insert(
            name.as_str(),
            LcnfDeclaration {
                name: name.clone(),
                // Decided below from the graph of every translation's uses.
                recursive: false,
                level_parameters: translation.level_parameters.clone(),
                ty,
                parameters,
                code,
            },
        );
    }

    // A definition is recursive exactly when it lies on a cycle of the
    // translations' use graph: it reaches itself.
    let graph: BTreeMap<&str, Vec<&str>> = raw
        .constants
        .iter()
        .filter_map(|constant| {
            constant.declaration.as_ref().map(|translation| {
                (
                    constant.name.as_str(),
                    translation
                        .uses
                        .iter()
                        .map(String::as_str)
                        .filter(|used| translations.contains_key(used))
                        .collect(),
                )
            })
        })
        .collect();
    for (name, declaration) in &mut translations {
        let mut stack: Vec<&str> = graph.get(name).cloned().unwrap_or_default();
        let mut seen = BTreeSet::new();
        while let Some(next) = stack.pop() {
            if next == *name {
                declaration.recursive = true;
                break;
            }
            if seen.insert(next) {
                stack.extend(graph.get(next).into_iter().flatten().copied());
            }
        }
    }

    // Each root's computational closure, decided here from Lean's facts.
    let expected = eligibility_closures(reports);
    let mut closures = Vec::new();
    let mut realized: BTreeSet<String> = BTreeSet::new();
    let mut inductive_names: BTreeSet<String> = BTreeSet::new();
    let mut external_names: BTreeSet<String> = BTreeSet::new();
    let mut all_erased: BTreeSet<String> = BTreeSet::new();
    for root in roots {
        let Some(eligibility) = expected.get(root) else {
            return Err(Rejection::Rejected(format!(
                "unknown root: `{root}` is not a production root of this project"
            )));
        };
        let mut members: BTreeSet<String> = BTreeSet::new();
        let mut queue = vec![root.clone()];
        while let Some(name) = queue.pop() {
            if members.contains(&name) {
                continue;
            }
            let Some(constant) = constants.get(name.as_str()) else {
                return Err(Rejection::Rejected(format!(
                    "unresolved dependency: `{name}` is reached but not reported"
                )));
            };
            if let Some(reason) = unrealizable(constant) {
                return Err(Rejection::Rejected(reason));
            }
            if let Some(reason) = refused.get(name.as_str()) {
                return Err(Rejection::Rejected(reason.clone()));
            }
            let Some(translation) = &constant.declaration else {
                return Err(Rejection::Rejected(format!(
                    "dropped dependency: `{name}` is a definition Lean compiles, but the extraction reports no translation of it"
                )));
            };
            members.insert(name.clone());
            for used in &translation.uses {
                if let Some(project) = constants.get(used.as_str()) {
                    match project.kind.as_str() {
                        "inductive" => {
                            inductive_names.insert(used.clone());
                        }
                        "constructor" => {
                            let owner = raw.constants.iter().find(|candidate| {
                                candidate.inductive.as_ref().is_some_and(|inductive| {
                                    inductive
                                        .constructors
                                        .iter()
                                        .any(|constructor| &constructor.name == used)
                                })
                            });
                            let Some(owner) = owner else {
                                return Err(Rejection::Rejected(format!(
                                    "unresolved dependency: the constructor `{used}` has no reported inductive"
                                )));
                            };
                            inductive_names.insert(owner.name.clone());
                        }
                        _ => queue.push(used.clone()),
                    }
                } else if let Some(external) = externals.get(used.as_str()) {
                    admissible_external(external).map_err(Rejection::Rejected)?;
                    external_names.insert(used.clone());
                } else {
                    return Err(Rejection::Rejected(format!(
                        "dropped dependency: `{name}` uses `{used}`, which the extraction neither defines nor records"
                    )));
                }
            }
        }
        let (runtime, source): (BTreeSet<String>, BTreeSet<String>) = members
            .iter()
            .cloned()
            .partition(|member| is_runtime_member(member, &module_set));
        // Lean's closure of every root must be exactly the eligibility
        // analysis's: a member only Lean reaches is a dependency the
        // analysis dropped, and a member only the analysis reaches is one
        // Lean does not compile.
        if let Some(dropped) = source.difference(&eligibility.declarations).next() {
            return Err(Rejection::Rejected(format!(
                "dropped dependency: Lean's compiler reaches `{dropped}` from `{root}`, but the production-eligibility closure does not"
            )));
        }
        if let Some(phantom) = eligibility.declarations.difference(&source).next() {
            return Err(Rejection::Rejected(format!(
                "the production-eligibility closure of `{root}` realizes `{phantom}`, which Lean's compiler does not reach"
            )));
        }
        // The proofs the closure's kernel values reach, directly or through
        // compiler-generated helpers, are erased.
        let mut erased = BTreeSet::new();
        let mut internal_erased = BTreeSet::new();
        let mut visited = BTreeSet::new();
        let mut kernel: Vec<&str> = members.iter().map(String::as_str).collect();
        while let Some(name) = kernel.pop() {
            if !visited.insert(name) {
                continue;
            }
            let Some(constant) = constants.get(name) else {
                continue;
            };
            for used in &constant.kernel_uses {
                let Some(used_constant) = constants.get(used.as_str()) else {
                    // The adapter reports every project constant a kernel
                    // value names; one that is missing hides a dependency.
                    if is_project(used, &module_set, &externals) {
                        return Err(Rejection::Rejected(format!(
                            "unresolved dependency: the kernel value of `{name}` names `{used}`, which the extraction does not report"
                        )));
                    }
                    continue;
                };
                if used_constant.kind == "theorem" {
                    if used_constant.internal {
                        internal_erased.insert(used.clone());
                    } else {
                        erased.insert(used.clone());
                    }
                }
                if used_constant.internal {
                    kernel.push(used.as_str());
                }
            }
        }
        if erased != eligibility.erased {
            return Err(Rejection::Rejected(format!(
                "the proof-only dependencies of `{root}` differ: Lean erases {erased:?}, the eligibility analysis {:?}",
                eligibility.erased
            )));
        }
        all_erased.extend(erased.iter().cloned());
        realized.extend(members.iter().cloned());
        closures.push(LcnfClosure {
            root: root.clone(),
            declarations: source.into_iter().collect(),
            runtime: runtime.into_iter().collect(),
            instances: eligibility.instances.clone(),
            erased: erased.into_iter().collect(),
            internal_erased: internal_erased.into_iter().collect(),
        });
    }
    closures.sort_by(|left, right| left.root.cmp(&right.root));
    let declarations: Vec<LcnfDeclaration> = realized
        .iter()
        .filter_map(|name| translations.get(name.as_str()).cloned())
        .collect();
    let mut inductives = Vec::new();
    for name in &inductive_names {
        let Some(row) = constants
            .get(name.as_str())
            .and_then(|constant| constant.inductive.as_ref())
        else {
            return Err(Rejection::Rejected(format!(
                "unresolved dependency: the inductive `{name}` has no reported constructors"
            )));
        };
        let mut constructors = Vec::new();
        for (index, constructor) in row.constructors.iter().enumerate() {
            let mut ty = constructor.ty.clone();
            Renamer::default().ty(&mut ty).map_err(|reason| {
                Rejection::Rejected(format!("`{}`: {reason}", constructor.name))
            })?;
            constructors.push(LcnfConstructor {
                name: constructor.name.clone(),
                index: index as u64,
                parameters: constructor.parameters,
                fields: constructor.fields,
                ty,
            });
        }
        inductives.push(LcnfInductive {
            name: name.clone(),
            parameters: row.parameters,
            indices: row.indices,
            recursive: row.recursive,
            constructors,
        });
    }
    let externals: Vec<LcnfExternal> = external_names
        .iter()
        .filter_map(|name| externals.get(name.as_str()))
        .map(|external| LcnfExternal {
            name: external.name.clone(),
            kind: external.original_kind.clone(),
            module: external.module.clone(),
            computable: external.computable,
            generates_code: external.compiled,
        })
        .collect();
    Ok(CompilerInput {
        spec: INPUT_SPEC.to_owned(),
        lean: raw.lean,
        roots: roots.to_vec(),
        closures,
        declarations,
        inductives,
        externals,
        erased: all_erased.into_iter().collect(),
    })
}

/// Whether a name belongs to the project: reported as a project constant,
/// or not reported as external while lying under a generated module.
fn is_project(
    name: &str,
    modules: &BTreeSet<String>,
    externals: &BTreeMap<&str, &RawExternal>,
) -> bool {
    !externals.contains_key(name)
        && modules.iter().any(|module| {
            name.starts_with(&format!("{module}.")) || name.contains(&format!(".{module}."))
        })
}
