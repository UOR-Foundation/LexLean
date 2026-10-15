//! The production export contract (SPEC.md §24.7): the stable, versioned
//! boundary through which a consumer compiles a project's production roots
//! to certified Rust packages, and replays the result without the source.
//!
//! A request names roots, targets by closed ID and descriptor digest, the
//! identities the caller pinned during setup, and certificate evidence to be
//! re-checked. It has no field through which a caller could state a
//! certification, an optimality claim, Lean, LCNF, a program, or Rust: every
//! such value in a result is computed here, and replay recomputes it.

use std::collections::{BTreeMap, BTreeSet};

use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};

use super::{acquire_lock, publish_tree, Engine, Selection, VerifyRequest};
use crate::artifact::canonical_json::Json;
use crate::artifact::content_id::{FramedHasher, Sha256Digest};
use crate::calculus::rust::package::{self, Errors, Export, Manifest, Passing};
use crate::calculus::rust::Profile;
use crate::calculus::Program;
use crate::code;
use crate::diagnostic::Diagnostic;
use crate::error::LexLeanError;

/// The schema tag of a production request.
pub const REQUEST_SPEC: &str = "lexlean/production-request/1";
/// The schema tag of a production result.
pub const RESULT_SPEC: &str = "lexlean/production-result/1";
/// The schema tag of one target descriptor.
pub const TARGET_SPEC: &str = "lexlean/target-descriptor/1";
/// The schema tag of the target-descriptor document.
pub const TARGETS_SPEC: &str = "lexlean/target-descriptors/1";
/// The schema tag of a replay report.
pub const REPLAY_SPEC: &str = "lexlean/production-replay/1";
/// The name every package exports its root's recorded entry under.
pub const EXPORT_NAME: &str = "run";
/// The build-root area that holds production bundles.
pub const BUNDLE_AREA: &str = "production";
/// The stages a replay does not re-run, because each needs the generated Lean
/// modules and the pinned toolchain; the attestation records that they ran.
pub const NOT_RECHECKED: [&str; 4] = [
    "axiom-audit",
    "lean-elaboration",
    "leanchecker-replay",
    "named-root-extraction",
];

const RESULT_FILE: &str = "result.json";
const SEMANTICS_PATH: &str = "language/semantics-1.2.toml";

// ---------------------------------------------------------------------------
// Request
// ---------------------------------------------------------------------------

/// A production request (`lexlean/production-request/1`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductionRequest {
    /// Exactly [`REQUEST_SPEC`].
    pub spec: String,
    /// The roots to export, strictly ascending by root name.
    pub roots: Vec<RootRequest>,
    /// The identities the caller pinned during setup.
    #[serde(default, skip_serializing_if = "Expectation::is_empty")]
    pub expect: Expectation,
    /// Certificate evidence to re-check, strictly ascending by root.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<Evidence>,
}

/// One requested root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RootRequest {
    /// The fully qualified Lean name of a production root.
    pub root: String,
    /// The crate name and version of its packages.
    pub package: PackageName,
    /// The targets to export, strictly ascending by ID.
    pub targets: Vec<TargetRef>,
}

/// A crate name and version.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageName {
    /// A lowercase snake-case crate name of at most 64 characters.
    pub name: String,
    /// Three canonical decimals, each at most `2^64 - 1`.
    pub version: String,
}

/// A target by its closed ID and the digest of its descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetRef {
    /// A registered target ID.
    pub id: String,
    /// The digest of that target's descriptor ([`targets`]).
    pub digest: Sha256Digest,
}

/// Identities pinned during setup; each present one must still hold.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Expectation {
    /// The project's source ID (§21.3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<Sha256Digest>,
    /// The project's semantic ID (§21.4).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantic_id: Option<Sha256Digest>,
    /// The SHA-256 of the lock file's bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock_sha256: Option<Sha256Digest>,
}

impl Expectation {
    fn is_empty(&self) -> bool {
        self.source_id.is_none() && self.semantic_id.is_none() && self.lock_sha256.is_none()
    }
}

/// The closed kinds of certificate evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    /// A GNAF request and certificate (§17.15, §17.18).
    Gnaf,
}

/// Certificate evidence for one root: inputs to re-check, never a claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    /// The requested root it is offered for.
    pub root: String,
    /// Its kind.
    pub kind: EvidenceKind,
    /// The project-relative path of the `lexlean/gnaf-request/1` document.
    pub request: String,
    /// The project-relative path of the `lexlean/gnaf-certificate/1` document.
    pub certificate: String,
    /// The identity of the certificate (§17.18) the caller means.
    pub certificate_id: Sha256Digest,
}

fn request_error(message: impl Into<String>) -> LexLeanError {
    LexLeanError::from_diagnostic(Diagnostic::new(code!("LLC0201"), message))
}

fn strictly_ascending<T, K: Ord>(items: &[T], key: impl Fn(&T) -> K) -> bool {
    items.windows(2).all(|pair| key(&pair[0]) < key(&pair[1]))
}

/// A crate name of the package form (§17.16); the renderer refuses the
/// keywords and reserved crate names among these.
fn is_crate_name(name: &str) -> bool {
    let mut characters = name.chars();
    name.len() <= 64
        && characters
            .next()
            .is_some_and(|first| first.is_ascii_lowercase())
        && characters.all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
        })
}

fn is_version(version: &str) -> bool {
    let parts: Vec<&str> = version.split('.').collect();
    parts.len() == 3
        && parts.iter().all(|part| {
            !part.is_empty()
                && part.chars().all(|character| character.is_ascii_digit())
                && part
                    .parse::<u64>()
                    .is_ok_and(|number| number.to_string() == *part)
        })
}

/// A root name is looked up, never interpreted: it is refused only when it
/// cannot be a Lean name at all, so that Lean text is not representable.
fn is_root_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 1024
        && name.split('.').count() >= 2
        && name.split('.').all(|segment| !segment.is_empty())
        && !name
            .chars()
            .any(|character| character.is_whitespace() || character.is_control())
        && !name.contains([
            ':', '(', ')', '{', '}', '[', ']', ',', ';', '"', '`', '\\', '/',
        ])
}

impl ProductionRequest {
    /// Read a request from its JSON bytes.
    ///
    /// # Errors
    ///
    /// `LLC0201` for bytes that are not a closed request of the right form.
    pub fn from_json(bytes: &[u8]) -> Result<Self, LexLeanError> {
        Json::parse(bytes)
            .map_err(|error| request_error(format!("production request: {error}")))?;
        let request: Self = serde_json::from_slice(bytes)
            .map_err(|error| request_error(format!("production request: {error}")))?;
        request.validate()?;
        Ok(request)
    }

    /// The canonical JSON bytes of the request.
    #[must_use]
    pub fn to_json(&self) -> Vec<u8> {
        canonical_bytes(self)
    }

    /// Check the request's form, independently of any project.
    ///
    /// # Errors
    ///
    /// `LLC0201` naming the first violated rule.
    pub fn validate(&self) -> Result<(), LexLeanError> {
        if self.spec != REQUEST_SPEC {
            return Err(request_error(format!(
                "production request has spec `{}`, expected `{REQUEST_SPEC}`",
                self.spec
            )));
        }
        if self.roots.is_empty() {
            return Err(request_error(
                "a production request names at least one root",
            ));
        }
        if !strictly_ascending(&self.roots, |root| root.root.clone()) {
            return Err(request_error(
                "production request roots are not strictly ascending by name",
            ));
        }
        let mut names: BTreeSet<(String, String)> = BTreeSet::new();
        for root in &self.roots {
            if !is_root_name(&root.root) {
                return Err(request_error(format!(
                    "`{}` is not the qualified name of a root",
                    root.root
                )));
            }
            if !is_crate_name(&root.package.name) {
                return Err(request_error(format!(
                    "root `{}`: `{}` is not a crate name `[a-z][a-z0-9_]{{0,63}}`",
                    root.root, root.package.name
                )));
            }
            if !is_version(&root.package.version) {
                return Err(request_error(format!(
                    "root `{}`: `{}` is not a version of three canonical decimals",
                    root.root, root.package.version
                )));
            }
            if root.targets.is_empty() {
                return Err(request_error(format!(
                    "root `{}` names no target",
                    root.root
                )));
            }
            if !strictly_ascending(&root.targets, |target| target.id.clone()) {
                return Err(request_error(format!(
                    "root `{}`: targets are not strictly ascending by ID",
                    root.root
                )));
            }
            for target in &root.targets {
                if !names.insert((target.id.clone(), root.package.name.clone())) {
                    return Err(request_error(format!(
                        "package `{}` is exported twice for target `{}`",
                        root.package.name, target.id
                    )));
                }
            }
        }
        if !strictly_ascending(&self.evidence, |evidence| evidence.root.clone()) {
            return Err(request_error(
                "production request evidence is not strictly ascending by root",
            ));
        }
        for evidence in &self.evidence {
            if !self.roots.iter().any(|root| root.root == evidence.root) {
                return Err(request_error(format!(
                    "evidence names `{}`, which the request does not export",
                    evidence.root
                )));
            }
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Target descriptors
// ---------------------------------------------------------------------------

/// What a registered target means (`lexlean/target-descriptor/1`): its
/// registry row, the runtime its profile carries, and the renderer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetDescriptor {
    /// Exactly [`TARGET_SPEC`].
    pub spec: String,
    /// The registered target ID.
    pub id: String,
    /// The registry's statement of the target.
    pub statement: String,
    /// Whether the target provides heap allocation.
    pub allocation: bool,
    /// The width of the natural-number representation.
    pub natural_bits: u32,
    /// The width of the integer representation.
    pub integer_bits: u32,
    /// The SHA-256 of the runtime the target's profile carries.
    pub runtime: Sha256Digest,
    /// The digest of the Rust renderer's sources.
    pub renderer: Sha256Digest,
    /// The framed SHA-256 of the canonical descriptor without this field.
    pub digest: Sha256Digest,
}

fn internal(message: impl Into<String>) -> LexLeanError {
    LexLeanError::from_diagnostic(Diagnostic::new(
        code!("LLI9001"),
        format!("phase production: {}", message.into()),
    ))
}

/// The framed digest of `value`'s canonical JSON with `field` removed.
fn digest_without<T: Serialize>(value: &T, field: &str, domain: &str, label: &str) -> Sha256Digest {
    let mut json = serde_json::to_value(value).unwrap_or(serde_json::Value::Null);
    if let serde_json::Value::Object(map) = &mut json {
        map.remove(field);
    }
    let canonical = Json::parse(json.to_string().as_bytes())
        .map(|parsed| parsed.to_canonical_string())
        .unwrap_or_default();
    let mut hasher = FramedHasher::new(domain);
    hasher.frame(label, canonical.as_bytes());
    hasher.finish()
}

fn canonical_bytes<T: Serialize>(value: &T) -> Vec<u8> {
    let text = serde_json::to_string(value).unwrap_or_default();
    Json::parse(text.as_bytes())
        .map(|parsed| parsed.to_file_bytes())
        .unwrap_or_default()
}

/// Every registered target's descriptor, in registry order.
///
/// # Errors
///
/// `LLI9001` when the embedded registry or semantics record is unreadable,
/// which the conformance suite rules out for a shipped compiler.
pub fn targets() -> Result<Vec<TargetDescriptor>, LexLeanError> {
    let registry = crate::production::registry().map_err(internal)?;
    let semantics = crate::embedded::FILES
        .iter()
        .find(|(path, _)| *path == SEMANTICS_PATH)
        .and_then(|(_, bytes)| std::str::from_utf8(bytes).ok())
        .and_then(|text| text.parse::<toml::Table>().ok())
        .ok_or_else(|| internal(format!("embedded `{SEMANTICS_PATH}` is unreadable")))?;
    let record = |key: &str| -> Result<Sha256Digest, LexLeanError> {
        semantics
            .get(key)
            .and_then(toml::Value::as_str)
            .ok_or_else(|| internal(format!("`{SEMANTICS_PATH}` records no `{key}`")))
            .and_then(|hex| Sha256Digest::from_hex(hex).map_err(internal))
    };
    let renderer = record("rust_renderer")?;
    let mut out = Vec::new();
    for target in registry.targets.values() {
        let profile = Profile::named(&target.id)
            .ok_or_else(|| internal(format!("target `{}` has no Rust profile", target.id)))?;
        let runtime = record(match profile {
            Profile::Core => "rust_runtime_core",
            Profile::Std => "rust_runtime_std",
        })?;
        let mut descriptor = TargetDescriptor {
            spec: TARGET_SPEC.to_owned(),
            id: target.id.clone(),
            statement: target.statement.clone(),
            allocation: target.allocation,
            natural_bits: target.natural_bits,
            integer_bits: target.integer_bits,
            runtime,
            renderer,
            digest: Sha256Digest([0; 32]),
        };
        descriptor.digest = digest_without(
            &descriptor,
            "digest",
            "lexlean-target-descriptor-v1",
            "descriptor",
        );
        out.push(descriptor);
    }
    Ok(out)
}

/// The canonical `lexlean/target-descriptors/1` document.
///
/// # Errors
///
/// As [`targets`].
pub fn targets_document() -> Result<Vec<u8>, LexLeanError> {
    #[derive(Serialize)]
    struct Document {
        spec: &'static str,
        targets: Vec<TargetDescriptor>,
    }
    Ok(canonical_bytes(&Document {
        spec: TARGETS_SPEC,
        targets: targets()?,
    }))
}

// ---------------------------------------------------------------------------
// Result
// ---------------------------------------------------------------------------

/// A production result (`lexlean/production-result/1`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductionResult {
    /// Exactly [`RESULT_SPEC`].
    pub spec: String,
    /// The framed SHA-256 of the canonical result without this field.
    pub result_id: Sha256Digest,
    /// The compiler that produced it.
    pub compiler: CompilerBinding,
    /// The project it was produced from.
    pub source: SourceBinding,
    /// The verification it rests on.
    pub verification: VerificationBinding,
    /// The exported roots, ascending by name.
    pub roots: Vec<RootOutput>,
}

/// The producing compiler.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompilerBinding {
    /// The crate version.
    pub version: String,
    /// The language-1.2 compiler-semantics ID (§21.2), an opaque digest.
    pub semantics: Sha256Digest,
    /// The pinned Lean toolchain.
    pub lean_toolchain: String,
}

/// The project's identities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceBinding {
    /// The source ID (§21.3).
    pub source_id: Sha256Digest,
    /// The semantic ID (§21.4).
    pub semantic_id: Sha256Digest,
    /// The SHA-256 of the lock file's bytes.
    pub lock_sha256: Sha256Digest,
}

/// One file of the bundle by its bundle-relative path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileDigest {
    /// The bundle-relative path.
    pub path: String,
    /// The byte length.
    pub byte_length: u64,
    /// The SHA-256 of the bytes.
    pub sha256: Sha256Digest,
}

/// The verification the outputs rest on (§22.9).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerificationBinding {
    /// The build ID.
    pub build_id: Sha256Digest,
    /// The attestation ID.
    pub attestation_id: Sha256Digest,
    /// The attestation.
    pub attestation: FileDigest,
    /// The canonical compiler input; its SHA-256 is the compiler-input ID.
    pub compiler_input: FileDigest,
    /// The preservation record.
    pub preservation: FileDigest,
}

/// One exported root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RootOutput {
    /// The fully qualified root name.
    pub root: String,
    /// Its position among the project's production roots (`R<index>`).
    pub index: u64,
    /// Its lowered program.
    pub program: FileDigest,
    /// What its certificate evidence establishes about optimality.
    pub optimality: Optimality,
    /// One output per requested target, ascending by target ID.
    pub outputs: Vec<TargetOutput>,
}

/// The output of one root in one target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetOutput {
    /// The target and its descriptor digest.
    pub target: TargetRef,
    /// The exporting package.
    pub package: PackageOutput,
    /// The crate certificates B and E are about (§17.17).
    #[serde(rename = "crate")]
    pub krate: FileDigest,
    /// The function a caller of the package invokes.
    pub entry: Entry,
    /// Its certification.
    pub certification: Certification,
}

/// An exporting package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageOutput {
    /// The crate name.
    pub name: String,
    /// The crate version.
    pub version: String,
    /// The bundle-relative package directory.
    pub directory: String,
    /// The name the entry is exported under: [`EXPORT_NAME`].
    pub export: String,
    /// Its files, ascending by path.
    pub files: Vec<FileDigest>,
}

/// The entry of a crate (`preservation.json`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    /// The program function.
    pub function: u64,
    /// Its symbol, `f<function>`.
    pub symbol: String,
}

/// The closed certification statuses of an output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CertificationStatus {
    /// Certificates A, B, and E compiled silently under the pinned Lean,
    /// replayed through `leanchecker`, and their root theorems depend on
    /// exactly the stated axioms (§17.17). Constructed here and checked by
    /// Lean's kernel: it is preservation, not optimality.
    KernelChecked,
}

/// The roles of a preservation certificate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificateRole {
    /// Certificate A: source to lowered program.
    A,
    /// Certificate B: lowered program to crate.
    B,
    /// Certificate E: A and B composed.
    E,
}

/// One preservation certificate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificateRef {
    /// Its role.
    pub role: CertificateRole,
    /// Its Lean module.
    pub module: String,
    /// Its theorem.
    pub theorem: String,
    /// Its file.
    pub file: FileDigest,
}

/// The certification of one output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Certification {
    /// The status.
    pub status: CertificationStatus,
    /// The exact axioms the root theorems depend on.
    pub axioms: Vec<String>,
    /// Certificates A, B, and E.
    pub certificates: Vec<CertificateRef>,
}

/// What certificate evidence establishes about a root's optimality.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Optimality {
    /// No evidence was offered; nothing is claimed.
    NotClaimed,
    /// A GNAF certificate re-checked by the host checker (§17.18).
    Gnaf(Box<GnafOptimality>),
}

/// A GNAF certificate's recorded re-check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GnafOptimality {
    /// The certificate's identity.
    pub certificate_id: Sha256Digest,
    /// The `SystemUniverseId` of its request.
    pub universe_id: Sha256Digest,
    /// The request.
    pub request: FileDigest,
    /// The certificate.
    pub certificate: FileDigest,
    /// The verdict the checker gives it: `certified` or `incomplete`.
    pub verdict: serde_json::Value,
}

/// The outcome of [`build`].
#[derive(Debug, Clone)]
pub struct ProductionOutput {
    /// The result.
    pub result: ProductionResult,
    /// The project-relative bundle directory.
    pub bundle: Utf8PathBuf,
}

impl ProductionResult {
    /// The identity of this result's content.
    #[must_use]
    pub fn recomputed_id(&self) -> Sha256Digest {
        digest_without(
            self,
            "result_id",
            "lexlean-production-result-v1",
            "result-body",
        )
    }

    /// The canonical file bytes.
    #[must_use]
    pub fn to_json(&self) -> Vec<u8> {
        canonical_bytes(self)
    }
}

// ---------------------------------------------------------------------------
// Records read from a verified directory
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Digested {
    byte_length: u64,
    sha256: Sha256Digest,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Theorem {
    byte_length: u64,
    module: String,
    sha256: Sha256Digest,
    theorem: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EntryRecord {
    function: u64,
    symbol: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Rendering {
    byte_length: u64,
    composed: Theorem,
    #[serde(rename = "crate")]
    krate: Digested,
    entry: EntryRecord,
    module: String,
    sha256: Sha256Digest,
    target: String,
    theorem: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PreservedRoot {
    byte_length: u64,
    module: String,
    program: Digested,
    renderings: Vec<Rendering>,
    root: String,
    sha256: Sha256Digest,
    #[allow(dead_code)]
    targets: Vec<String>,
    theorem: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Library {
    axioms: Vec<String>,
    #[allow(dead_code)]
    registry_sha256: Sha256Digest,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Preservation {
    library: Library,
    roots: Vec<PreservedRoot>,
    spec: String,
}

const PRESERVATION_SPEC: &str = "lexlean/preservation/1";

fn parse_preservation(bytes: &[u8]) -> Result<Preservation, String> {
    let record: Preservation =
        serde_json::from_slice(bytes).map_err(|error| format!("preservation.json: {error}"))?;
    if record.spec != PRESERVATION_SPEC {
        return Err(format!("preservation.json has spec `{}`", record.spec));
    }
    Ok(record)
}

/// The fields of an attestation the result binds, after its identity is
/// recomputed from its body (§22.9).
struct AttestationFacts {
    attestation_id: Sha256Digest,
    compiler_version: String,
    compiler_semantics: Sha256Digest,
    source_id: Sha256Digest,
    semantic_id: Sha256Digest,
    build_id: Sha256Digest,
    compiler_input: (u64, Sha256Digest),
    preservation: (u64, Sha256Digest),
}

fn reports_pinned_lean_version(tool: &str, banner: &str) -> bool {
    if banner.contains(['\n', '\r']) {
        return false;
    }
    // Lake's own version does not identify Lean; only the whole enclosed
    // Lean version field can establish the pin the result states.
    let version = match tool {
        "lean" => banner
            .strip_prefix("Lean (version ")
            .and_then(|rest| rest.strip_suffix(')'))
            .and_then(|rest| rest.split_once(", "))
            .filter(|(_, details)| !details.is_empty())
            .map(|(version, _)| version),
        "lake" => banner
            .strip_prefix("Lake version ")
            .and_then(|rest| rest.strip_suffix(')'))
            .and_then(|rest| rest.split_once(" (Lean version "))
            .filter(|(lake, _)| !lake.is_empty() && !lake.contains(char::is_whitespace))
            .map(|(_, version)| version),
        _ => None,
    };
    version == Some(crate::LEAN_VERSION)
}

fn require_attested_toolchain(body: &BTreeMap<String, Json>) -> Result<(), String> {
    let Some(Json::Obj(toolchain)) = body.get("toolchain") else {
        return Err("attestation.json states no `toolchain` object".to_owned());
    };
    for tool in ["lean", "lake"] {
        let Some(Json::Obj(record)) = toolchain.get(tool) else {
            return Err(format!(
                "attestation.json states no `toolchain.{tool}` object"
            ));
        };
        let Some(Json::Str(banner)) = record.get("version_output") else {
            return Err(format!(
                "attestation.json states no string `toolchain.{tool}.version_output`"
            ));
        };
        if !reports_pinned_lean_version(tool, banner) {
            return Err(format!(
                "attestation.json `toolchain.{tool}.version_output` does not report Lean {}",
                crate::LEAN_VERSION
            ));
        }
    }
    Ok(())
}

fn attestation_facts(bytes: &[u8]) -> Result<AttestationFacts, String> {
    let parsed = Json::parse(bytes).map_err(|error| format!("attestation.json: {error}"))?;
    let Json::Obj(mut body) = parsed else {
        return Err("attestation.json is not an object".to_owned());
    };
    let stated = match body.remove("attestation_id") {
        Some(Json::Str(hex)) => Sha256Digest::from_hex(&hex)?,
        _ => return Err("attestation.json states no attestation_id".to_owned()),
    };
    let recomputed =
        crate::artifact::content_id::attestation_id(&Json::Obj(body.clone()).to_canonical_string());
    if recomputed != stated {
        return Err(format!(
            "attestation.json states {stated}, its body hashes to {recomputed}"
        ));
    }
    for (field, expected) in [("spec", "lexlean/attestation/2"), ("status", "verified")] {
        if !matches!(body.get(field), Some(Json::Str(value)) if value == expected) {
            return Err(format!(
                "attestation.json requires `{field}` = `{expected}`"
            ));
        }
    }
    let Some(Json::Obj(compiler)) = body.get("lexlean") else {
        return Err("attestation.json states no `lexlean` compiler object".to_owned());
    };
    let Some(Json::Str(compiler_version)) = compiler.get("version") else {
        return Err("attestation.json states no string `lexlean.version`".to_owned());
    };
    if compiler_version.is_empty() {
        return Err("attestation.json states an empty `lexlean.version`".to_owned());
    }
    let Some(Json::Str(compiler_semantics)) = compiler.get("compiler_semantics") else {
        return Err("attestation.json states no string `lexlean.compiler_semantics`".to_owned());
    };
    let compiler_semantics = Sha256Digest::from_hex(compiler_semantics)
        .map_err(|reason| format!("attestation.json `lexlean.compiler_semantics`: {reason}"))?;
    require_attested_toolchain(&body)?;
    let digest = |name: &str| -> Result<Sha256Digest, String> {
        match body.get(name) {
            Some(Json::Str(hex)) => Sha256Digest::from_hex(hex),
            _ => Err(format!("attestation.json states no `{name}`")),
        }
    };
    let row = |name: &str| -> Result<(u64, Sha256Digest), String> {
        let Some(Json::Obj(row)) = body.get(name) else {
            return Err(format!(
                "attestation.json binds no `{name}`: the project has no production root"
            ));
        };
        let length = match row.get("byte_length") {
            Some(Json::Int(value)) => u64::try_from(*value).map_err(|error| error.to_string())?,
            _ => return Err(format!("attestation.json `{name}` has no byte_length")),
        };
        let sha = match row.get("sha256") {
            Some(Json::Str(hex)) => Sha256Digest::from_hex(hex)?,
            _ => return Err(format!("attestation.json `{name}` has no sha256")),
        };
        Ok((length, sha))
    };
    Ok(AttestationFacts {
        attestation_id: stated,
        compiler_version: compiler_version.clone(),
        compiler_semantics,
        source_id: digest("source_id")?,
        semantic_id: digest("semantic_id")?,
        build_id: digest("build_id")?,
        compiler_input: row("compiler_input")?,
        preservation: row("preservation")?,
    })
}

fn deserialize_object<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    struct ObjectVisitor<T>(std::marker::PhantomData<T>);

    impl<'de, T: Deserialize<'de>> serde::de::Visitor<'de> for ObjectVisitor<T> {
        type Value = T;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("a JSON object")
        }

        fn visit_map<A: serde::de::MapAccess<'de>>(self, map: A) -> Result<T, A::Error> {
            T::deserialize(serde::de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_map(ObjectVisitor(std::marker::PhantomData))
}

fn require_compiler_input_identity(bytes: &[u8]) -> Result<(), String> {
    #[derive(Deserialize)]
    struct Header {
        #[serde(deserialize_with = "deserialize_object")]
        lean: crate::production::lcnf::LeanIdentity,
    }
    // The LCNF payload can nest beyond the JSON parser's ordinary limit.
    // Deserialize only the identity: ignored fields are skipped iteratively.
    // Require objects while preserving the derived readers' duplicate checks.
    let mut reader = serde_json::Deserializer::from_slice(bytes);
    let header: Header = deserialize_object(&mut reader)
        .map_err(|error| format!("compiler-input.json Lean identity: {error}"))?;
    reader
        .end()
        .map_err(|error| format!("compiler-input.json Lean identity: {error}"))?;
    if header.lean.version != crate::LEAN_VERSION || header.lean.githash != crate::LEAN_GITHASH {
        return Err(format!(
            "compiler-input.json reports Lean {} at {}, expected {} at {}",
            header.lean.version,
            header.lean.githash,
            crate::LEAN_VERSION,
            crate::LEAN_GITHASH
        ));
    }
    Ok(())
}

/// The verified-directory path of a certificate module.
fn certificate_path(module: &str) -> String {
    format!("{}.lean", module.replace('.', "/"))
}

// ---------------------------------------------------------------------------
// Build
// ---------------------------------------------------------------------------

/// A requested root, resolved against the checked project before any Lean
/// runs.
struct Planned {
    request: RootRequest,
    program_bytes: Vec<u8>,
    entry: u64,
    packages: Vec<(TargetRef, Manifest, package::Package)>,
    evidence: Option<CheckedEvidence>,
}

struct CheckedEvidence {
    request_bytes: Vec<u8>,
    certificate_bytes: Vec<u8>,
    certificate_id: Sha256Digest,
    universe_id: Sha256Digest,
    verdict: serde_json::Value,
}

fn evidence_error(message: impl Into<String>) -> LexLeanError {
    LexLeanError::from_diagnostic(Diagnostic::new(code!("LLV7101"), message))
}

/// Re-check GNAF evidence against a root's lowered program (§17.18).
fn check_gnaf(
    root: &str,
    program: &Program,
    request_bytes: Vec<u8>,
    certificate_bytes: Vec<u8>,
    pinned: Sha256Digest,
) -> Result<CheckedEvidence, LexLeanError> {
    use crate::gnaf::certification::{self, Verdict};
    let request = crate::gnaf::load(&request_bytes)?;
    let certificate = certification::load(&certificate_bytes)?;
    let certificate_id = certification::certificate_id(&certificate)?;
    if certificate_id != pinned {
        return Err(evidence_error(format!(
            "root `{root}`: the certificate's identity is {certificate_id}, the request pins {pinned}"
        )));
    }
    let reference = request
        .reference
        .id()
        .map_err(|reason| evidence_error(format!("root `{root}`: {reason}")))?;
    let lowered = program
        .id()
        .map_err(|reason| internal(format!("root `{root}`: {reason}")))?;
    if reference != lowered {
        return Err(evidence_error(format!(
            "root `{root}`: the GNAF request's reference program {reference} is not the root's lowered program {lowered}"
        )));
    }
    let universe_id = crate::gnaf::universe_id(&request)?;
    let verdict = certification::check(&request, &certificate)?;
    if let Verdict::Refused { refusal } = &verdict {
        return Err(evidence_error(format!(
            "root `{root}`: the certificate does not bind its request: {refusal:?}"
        )));
    }
    let verdict = serde_json::to_value(&verdict).map_err(|error| internal(error.to_string()))?;
    Ok(CheckedEvidence {
        request_bytes,
        certificate_bytes,
        certificate_id,
        universe_id,
        verdict,
    })
}

/// The manifest of the package exporting `entry` of `program` as `run`.
fn manifest_for(
    package: &PackageName,
    target: &str,
    program: &Program,
    entry: u64,
    semantic_id: Sha256Digest,
) -> Result<Manifest, String> {
    let fallible = crate::calculus::rust::fallible_functions(program)?;
    let function = program
        .functions
        .get(usize::try_from(entry).map_err(|error| error.to_string())?)
        .ok_or_else(|| format!("the program has no function {entry}"))?;
    Ok(Manifest {
        spec: package::MANIFEST_SPEC.to_owned(),
        name: package.name.clone(),
        version: package.version.clone(),
        profile: target.to_owned(),
        program: program.clone(),
        exports: vec![Export {
            function: entry,
            name: EXPORT_NAME.to_owned(),
            parameters: vec![Passing::Own; function.types.len()],
            errors: if fallible.get(entry as usize).copied().unwrap_or(false) {
                Errors::Overflow
            } else {
                Errors::None
            },
        }],
        sources: vec![semantic_id.to_hex()],
    })
}

fn file_digest(path: String, bytes: &[u8]) -> FileDigest {
    FileDigest {
        path,
        byte_length: bytes.len() as u64,
        sha256: Sha256Digest::of(bytes),
    }
}

fn stale(message: impl Into<String>) -> LexLeanError {
    LexLeanError::from_diagnostic(Diagnostic::new(code!("LLC0204"), message))
}

fn target_error(message: impl Into<String>) -> LexLeanError {
    LexLeanError::from_diagnostic(Diagnostic::new(code!("LLC0203"), message))
}

/// Compile the requested production roots to certified Rust packages.
///
/// Every request, root, target, expectation, evidence, and package defect is
/// refused before any Lean runs or anything is published. The project is then
/// verified by the unchanged §22 pipeline, each package is rendered from the
/// verified lowered program exporting only the recorded entry, and the bundle
/// is published atomically at `<build_root>/production/<result-id>/`.
///
/// # Errors
///
/// The registered codes of §24.7, and every code of `verify`.
pub fn build(
    engine: &Engine,
    request: &ProductionRequest,
) -> Result<ProductionOutput, LexLeanError> {
    engine.bound("production", build_inner(engine, request))
}

#[allow(clippy::too_many_lines)]
fn build_inner(
    engine: &Engine,
    request: &ProductionRequest,
) -> Result<ProductionOutput, LexLeanError> {
    request.validate()?;
    let project = &engine.project;
    let (checked, _lock) = engine.checked(&Selection::Entrypoints)?;
    let lock_bytes =
        crate::lock::read_lock_bytes(project).map_err(LexLeanError::from_diagnostic)?;
    let lock_sha256 = Sha256Digest::of(&lock_bytes);
    for (what, pinned, actual) in [
        ("source ID", request.expect.source_id, checked.source_id),
        (
            "semantic ID",
            request.expect.semantic_id,
            checked.semantic_id,
        ),
        ("lock digest", request.expect.lock_sha256, lock_sha256),
    ] {
        if let Some(pinned) = pinned {
            if pinned != actual {
                return Err(stale(format!(
                    "the request pins the {what} {pinned}; the project's is {actual}"
                )));
            }
        }
    }
    let descriptors: BTreeMap<String, TargetDescriptor> = targets()?
        .into_iter()
        .map(|descriptor| (descriptor.id.clone(), descriptor))
        .collect();
    let linked = crate::production::lower::linked_modules(&checked);
    let all_roots =
        crate::production::lower::roots(&checked).map_err(LexLeanError::from_diagnostic)?;
    let limits = project.config.limits;
    let mut planned = Vec::new();
    for wanted in &request.roots {
        let Some(root) = all_roots
            .iter()
            .find(|root| root.report.root == wanted.root)
        else {
            return Err(LexLeanError::from_diagnostic(Diagnostic::new(
                code!("LLC0202"),
                format!(
                    "`{}` is not a production root of the project's entrypoint closure",
                    wanted.root
                ),
            )));
        };
        for target in &wanted.targets {
            let Some(descriptor) = descriptors.get(&target.id) else {
                return Err(target_error(format!(
                    "root `{}`: `{}` is not a registered target",
                    wanted.root, target.id
                )));
            };
            if descriptor.digest != target.digest {
                return Err(target_error(format!(
                    "root `{}`: target `{}` has descriptor digest {}, the request pins {}",
                    wanted.root, target.id, descriptor.digest, target.digest
                )));
            }
            if !root
                .report
                .targets
                .iter()
                .any(|row| row.target == target.id)
            {
                return Err(target_error(format!(
                    "root `{}` does not declare the target `{}`",
                    wanted.root, target.id
                )));
            }
        }
        let lowered = crate::production::lower::lower_root(
            &linked,
            &root.module,
            &root.name,
            root.report,
            &limits,
        )
        .map_err(LexLeanError::from_diagnostic)?;
        let entry = lowered.entry();
        let program = lowered.program;
        let mut packages = Vec::new();
        for target in &wanted.targets {
            let manifest = manifest_for(
                &wanted.package,
                &target.id,
                &program,
                entry,
                checked.semantic_id,
            )
            .map_err(|reason| internal(format!("root `{}`: {reason}", wanted.root)))?;
            let rendered = package::package(&manifest).map_err(|reason| {
                LexLeanError::from_diagnostic(Diagnostic::new(
                    code!("LLB6005"),
                    format!(
                        "root `{}`: package `{}` for {}: {reason}",
                        wanted.root, wanted.package.name, target.id
                    ),
                ))
            })?;
            packages.push((target.clone(), manifest, rendered));
        }
        let evidence = match request
            .evidence
            .iter()
            .find(|evidence| evidence.root == wanted.root)
        {
            Some(evidence) => {
                // Confinement refuses an absolute path, `..`, and any
                // symlinked component (§25.1), as `LLS8001`.
                let read = |relative: &str| -> Result<Vec<u8>, LexLeanError> {
                    let path = project
                        .confined_file(relative)
                        .map_err(LexLeanError::from_diagnostic)?;
                    std::fs::read(path.as_std_path()).map_err(|io_error| {
                        LexLeanError::from_diagnostic(Diagnostic::new(
                            code!("LLV7010"),
                            format!("evidence `{relative}`: {io_error}"),
                        ))
                    })
                };
                Some(check_gnaf(
                    &wanted.root,
                    &program,
                    read(&evidence.request)?,
                    read(&evidence.certificate)?,
                    evidence.certificate_id,
                )?)
            }
            None => None,
        };
        planned.push(Planned {
            request: wanted.clone(),
            program_bytes: program.to_file_bytes(),
            entry,
            packages,
            evidence,
        });
    }
    drop(checked);
    // The bundle's area is confined before verification spends its time: a
    // symlinked area is refused here, not after Lean has run (§25.1).
    project
        .confined_creatable(&format!("{}/{BUNDLE_AREA}", project.config.build_root))
        .map_err(LexLeanError::from_diagnostic)?;

    // The unchanged verification pipeline (§22), over the same selection.
    let verified = engine.verify(VerifyRequest {
        selection: Selection::Entrypoints,
    })?;
    // A project edited while it was being exported is stale, not exported.
    let lock_after =
        crate::lock::read_lock_bytes(project).map_err(LexLeanError::from_diagnostic)?;
    let (checked_source, checked_semantic) = {
        let (again, _) = engine.checked(&Selection::Entrypoints)?;
        (again.source_id, again.semantic_id)
    };
    if verified.source_id != checked_source
        || verified.semantic_id != checked_semantic
        || Sha256Digest::of(&lock_after) != lock_sha256
        || request
            .expect
            .source_id
            .is_some_and(|pinned| pinned != verified.source_id)
        || request
            .expect
            .semantic_id
            .is_some_and(|pinned| pinned != verified.semantic_id)
    {
        return Err(stale("the project changed while it was being exported"));
    }

    let read_verified = |relative: &str| -> Result<Vec<u8>, LexLeanError> {
        std::fs::read(verified.root.join(relative).as_std_path()).map_err(|io_error| {
            internal(format!(
                "the verified directory has no `{relative}`: {io_error}"
            ))
        })
    };
    let attestation_bytes = read_verified("attestation.json")?;
    let compiler_input_bytes = read_verified("production/compiler-input.json")?;
    let preservation_bytes = read_verified("preserve/preservation.json")?;
    let preservation = parse_preservation(&preservation_bytes).map_err(internal)?;

    let mut files: Vec<(String, Vec<u8>)> = Vec::new();
    let mut add = |path: String, bytes: Vec<u8>| -> FileDigest {
        let digest = file_digest(path.clone(), &bytes);
        files.push((path, bytes));
        digest
    };
    let attestation = add("evidence/attestation.json".to_owned(), attestation_bytes);
    let compiler_input = add(
        "evidence/compiler-input.json".to_owned(),
        compiler_input_bytes,
    );
    let preservation_file = add("evidence/preservation.json".to_owned(), preservation_bytes);

    let mut roots = Vec::new();
    for plan in planned {
        let (index, preserved) = preservation
            .roots
            .iter()
            .enumerate()
            .find(|(_, preserved)| preserved.root == plan.request.root)
            .ok_or_else(|| internal(format!("`{}` was not certified", plan.request.root)))?;
        let program_bytes = read_verified(&format!("preserve/program/R{index}.json"))?;
        if program_bytes != plan.program_bytes {
            return Err(internal(format!(
                "root `{}`: the verified program differs from the one the request was checked against",
                plan.request.root
            )));
        }
        let program = add(format!("evidence/program/R{index}.json"), program_bytes);
        let a_path = certificate_path(&preserved.module);
        let certificate_a = CertificateRef {
            role: CertificateRole::A,
            module: preserved.module.clone(),
            theorem: preserved.theorem.clone(),
            file: add(
                format!("evidence/certificates/{a_path}"),
                read_verified(&format!("preserve/{a_path}"))?,
            ),
        };
        let mut outputs = Vec::new();
        for (target, manifest, rendered) in plan.packages {
            let rendering = preserved
                .renderings
                .iter()
                .find(|rendering| rendering.target == target.id)
                .ok_or_else(|| {
                    internal(format!(
                        "root `{}` has no certified rendering in {}",
                        plan.request.root, target.id
                    ))
                })?;
            if rendering.entry.function != plan.entry {
                return Err(internal(format!(
                    "root `{}`: preservation.json records entry {}, the lowering {}",
                    plan.request.root, rendering.entry.function, plan.entry
                )));
            }
            let crate_bytes = read_verified(&format!("preserve/crate/R{index}.{}.rs", target.id))?;
            let library = rendered
                .files
                .get("src/lib.rs")
                .ok_or_else(|| internal("a package without src/lib.rs"))?;
            if !library.starts_with(&crate_bytes) {
                return Err(internal(format!(
                    "root `{}`: the package for {} does not begin with the certified crate",
                    plan.request.root, target.id
                )));
            }
            let krate = add(
                format!("evidence/crate/R{index}.{}.rs", target.id),
                crate_bytes,
            );
            let mut certificates = vec![certificate_a.clone()];
            for (role, module, theorem) in [
                (CertificateRole::B, &rendering.module, &rendering.theorem),
                (
                    CertificateRole::E,
                    &rendering.composed.module,
                    &rendering.composed.theorem,
                ),
            ] {
                let path = certificate_path(module);
                certificates.push(CertificateRef {
                    role,
                    module: module.clone(),
                    theorem: theorem.clone(),
                    file: add(
                        format!("evidence/certificates/{path}"),
                        read_verified(&format!("preserve/{path}"))?,
                    ),
                });
            }
            let directory = format!("packages/{}/{}", target.id, plan.request.package.name);
            let manifest_bytes = manifest
                .to_file_bytes()
                .map_err(|reason| internal(format!("package manifest: {reason}")))?;
            let mut package_files = vec![add(format!("{directory}/package.json"), manifest_bytes)];
            for (relative, bytes) in rendered.files {
                package_files.push(add(format!("{directory}/{relative}"), bytes));
            }
            package_files.sort_by(|left, right| left.path.cmp(&right.path));
            outputs.push(TargetOutput {
                target,
                package: PackageOutput {
                    name: plan.request.package.name.clone(),
                    version: plan.request.package.version.clone(),
                    directory,
                    export: EXPORT_NAME.to_owned(),
                    files: package_files,
                },
                krate,
                entry: Entry {
                    function: rendering.entry.function,
                    symbol: rendering.entry.symbol.clone(),
                },
                certification: Certification {
                    status: CertificationStatus::KernelChecked,
                    axioms: preservation.library.axioms.clone(),
                    certificates,
                },
            });
        }
        let optimality = match plan.evidence {
            None => Optimality::NotClaimed,
            Some(evidence) => Optimality::Gnaf(Box::new(GnafOptimality {
                certificate_id: evidence.certificate_id,
                universe_id: evidence.universe_id,
                request: add(
                    format!("evidence/gnaf/R{index}/request.json"),
                    evidence.request_bytes,
                ),
                certificate: add(
                    format!("evidence/gnaf/R{index}/certificate.json"),
                    evidence.certificate_bytes,
                ),
                verdict: evidence.verdict,
            })),
        };
        roots.push(RootOutput {
            root: plan.request.root.clone(),
            index: index as u64,
            program,
            optimality,
            outputs,
        });
    }
    let mut result = ProductionResult {
        spec: RESULT_SPEC.to_owned(),
        result_id: Sha256Digest([0; 32]),
        compiler: CompilerBinding {
            version: crate::COMPILER_VERSION.to_owned(),
            semantics: crate::compiler_semantics_id_for(crate::LANGUAGE_1_2),
            lean_toolchain: crate::LEAN_TOOLCHAIN.to_owned(),
        },
        source: SourceBinding {
            source_id: verified.source_id,
            semantic_id: verified.semantic_id,
            lock_sha256,
        },
        verification: VerificationBinding {
            build_id: verified.build_id,
            attestation_id: verified.attestation_id,
            attestation,
            compiler_input,
            preservation: preservation_file,
        },
        roots,
    };
    result.result_id = result.recomputed_id();
    files.push((RESULT_FILE.to_owned(), result.to_json()));
    files.sort_by(|left, right| left.0.cmp(&right.0));
    // The bundle states exactly what it holds; replay is the check.
    replay_files(&files)?;
    let guard = acquire_lock(project)?;
    publish_tree(
        project,
        BUNDLE_AREA,
        &result.result_id.to_hex(),
        &files,
        &guard,
    )?;
    drop(guard);
    Ok(ProductionOutput {
        bundle: Utf8PathBuf::from(format!(
            "{}/{BUNDLE_AREA}/{}",
            project.config.build_root,
            result.result_id.to_hex()
        )),
        result,
    })
}

// ---------------------------------------------------------------------------
// Replay
// ---------------------------------------------------------------------------

/// The closed replay statuses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReplayStatus {
    /// Every identity, digest, binding, re-rendered package, target
    /// descriptor, and certificate verdict agrees. Lean was not run: this is
    /// never `kernel-checked`.
    BindingsVerified,
}

/// What a replay established (`lexlean/production-replay/1`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayReport {
    /// Exactly [`REPLAY_SPEC`].
    pub spec: String,
    /// The replayed result.
    pub result_id: Sha256Digest,
    /// The status.
    pub status: ReplayStatus,
    /// The number of outputs replayed.
    pub outputs: u64,
    /// The stages not re-run: [`NOT_RECHECKED`].
    pub not_rechecked: Vec<String>,
}

fn replay_error(message: impl Into<String>) -> LexLeanError {
    LexLeanError::from_diagnostic(Diagnostic::new(code!("LLV7102"), message))
}

/// Replay a published bundle without the source, Lean, or the network.
///
/// # Errors
///
/// `LLS8001` for a symlink or special file in the bundle, `LLV7102` for any
/// disagreement, and `LLV7010` for an unreadable file.
pub fn verify_artifact(bundle: &Utf8Path) -> Result<ReplayReport, LexLeanError> {
    replay_files(&collect_bundle(bundle)?)
}

fn collect_bundle(bundle: &Utf8Path) -> Result<Vec<(String, Vec<u8>)>, LexLeanError> {
    let security =
        |message: String| LexLeanError::from_diagnostic(Diagnostic::new(code!("LLS8001"), message));
    let metadata = std::fs::symlink_metadata(bundle.as_std_path()).map_err(|io_error| {
        LexLeanError::from_diagnostic(Diagnostic::new(
            code!("LLV7010"),
            format!("{bundle}: {io_error}"),
        ))
    })?;
    if !metadata.is_dir() {
        return Err(security(format!("{bundle} is not a directory")));
    }
    let mut files = Vec::new();
    for entry in walkdir::WalkDir::new(bundle.as_std_path()).follow_links(false) {
        let entry = entry.map_err(|error| {
            LexLeanError::from_diagnostic(Diagnostic::new(code!("LLV7010"), error.to_string()))
        })?;
        let relative_path = entry
            .path()
            .strip_prefix(bundle.as_std_path())
            .map_err(|error| replay_error(format!("bundle path is outside its root: {error}")))?;
        let kind = entry.file_type();
        if kind.is_symlink() {
            return Err(security(format!(
                "`{}` in the bundle is a symlink",
                relative_path.display()
            )));
        }
        if !kind.is_dir() && !kind.is_file() {
            return Err(security(format!(
                "`{}` in the bundle is a special file",
                relative_path.display()
            )));
        }
        // Only actual path separators become `/`: on Unix a backslash
        // belongs to its filename and must not alias a different file.
        let relative = relative_path
            .components()
            .map(|component| {
                component
                    .as_os_str()
                    .to_str()
                    .ok_or_else(|| replay_error("a bundle filename is not valid UTF-8"))
            })
            .collect::<Result<Vec<_>, _>>()?
            .join("/");
        if kind.is_dir() {
            continue;
        }
        let bytes = std::fs::read(entry.path()).map_err(|io_error| {
            LexLeanError::from_diagnostic(Diagnostic::new(
                code!("LLV7010"),
                format!("`{relative}`: {io_error}"),
            ))
        })?;
        files.push((relative, bytes));
    }
    files.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(files)
}

#[allow(clippy::too_many_lines)]
fn replay_files(files: &[(String, Vec<u8>)]) -> Result<ReplayReport, LexLeanError> {
    let mut by_path: BTreeMap<&str, &[u8]> = BTreeMap::new();
    for (path, bytes) in files {
        if by_path.insert(path.as_str(), bytes.as_slice()).is_some() {
            return Err(replay_error(format!(
                "the bundle inventory contains duplicate path `{path}`"
            )));
        }
    }
    let result_bytes = by_path
        .get(RESULT_FILE)
        .ok_or_else(|| replay_error("the bundle has no result.json"))?;
    Json::parse(result_bytes).map_err(|error| replay_error(format!("result.json: {error}")))?;
    let result: ProductionResult = serde_json::from_slice(result_bytes)
        .map_err(|error| replay_error(format!("result.json: {error}")))?;
    if result.spec != RESULT_SPEC {
        return Err(replay_error(format!(
            "result.json has spec `{}`, expected `{RESULT_SPEC}`",
            result.spec
        )));
    }
    if result.to_json() != *result_bytes {
        return Err(replay_error(
            "result.json is not in canonical serialization",
        ));
    }
    let recomputed = result.recomputed_id();
    if recomputed != result.result_id {
        return Err(replay_error(format!(
            "result.json states result {}, its content hashes to {recomputed}",
            result.result_id
        )));
    }
    let this_compiler = crate::compiler_semantics_id_for(crate::LANGUAGE_1_2);
    if result.compiler.semantics != this_compiler
        || result.compiler.lean_toolchain != crate::LEAN_TOOLCHAIN
    {
        return Err(replay_error(format!(
            "the result was produced by compiler semantics {} under {}; this compiler is {this_compiler} under {}, and replay needs the producing compiler",
            result.compiler.semantics,
            result.compiler.lean_toolchain,
            crate::LEAN_TOOLCHAIN
        )));
    }

    // Every file the result states, and no other.
    let mut stated: BTreeSet<String> = BTreeSet::new();
    stated.insert(RESULT_FILE.to_owned());
    let mut check = |digest: &FileDigest| -> Result<&[u8], LexLeanError> {
        let bytes = by_path
            .get(digest.path.as_str())
            .copied()
            .ok_or_else(|| replay_error(format!("the bundle has no `{}`", digest.path)))?;
        if bytes.len() as u64 != digest.byte_length || Sha256Digest::of(bytes) != digest.sha256 {
            return Err(replay_error(format!(
                "`{}` does not have the length and SHA-256 the result states",
                digest.path
            )));
        }
        stated.insert(digest.path.clone());
        Ok(bytes)
    };

    let attestation =
        attestation_facts(check(&result.verification.attestation)?).map_err(replay_error)?;
    let compiler_input = check(&result.verification.compiler_input)?;
    let preservation_bytes = check(&result.verification.preservation)?;
    let bind = |what: &str, holds: bool| -> Result<(), LexLeanError> {
        if holds {
            Ok(())
        } else {
            Err(replay_error(format!("{what} disagrees with the bundle")))
        }
    };
    bind(
        "the attestation ID",
        attestation.attestation_id == result.verification.attestation_id,
    )?;
    bind(
        "the compiler version",
        attestation.compiler_version == result.compiler.version,
    )?;
    bind(
        "the compiler semantics",
        attestation.compiler_semantics == result.compiler.semantics,
    )?;
    bind(
        "the source ID",
        attestation.source_id == result.source.source_id,
    )?;
    bind(
        "the semantic ID",
        attestation.semantic_id == result.source.semantic_id,
    )?;
    bind(
        "the build ID",
        attestation.build_id == result.verification.build_id,
    )?;
    bind(
        "the compiler input the attestation binds",
        attestation.compiler_input
            == (
                compiler_input.len() as u64,
                Sha256Digest::of(compiler_input),
            ),
    )?;
    require_compiler_input_identity(compiler_input).map_err(replay_error)?;
    bind(
        "the preservation record the attestation binds",
        attestation.preservation
            == (
                preservation_bytes.len() as u64,
                Sha256Digest::of(preservation_bytes),
            ),
    )?;
    let preservation = parse_preservation(preservation_bytes).map_err(replay_error)?;
    let descriptors: BTreeMap<String, TargetDescriptor> = targets()?
        .into_iter()
        .map(|descriptor| (descriptor.id.clone(), descriptor))
        .collect();
    let semantic_hex = result.source.semantic_id.to_hex();

    if !strictly_ascending(&result.roots, |root| root.root.clone()) || result.roots.is_empty() {
        return Err(replay_error(
            "the result's roots are not strictly ascending",
        ));
    }
    let mut outputs = 0u64;
    for root in &result.roots {
        let preserved = usize::try_from(root.index)
            .ok()
            .and_then(|index| preservation.roots.get(index))
            .filter(|preserved| preserved.root == root.root)
            .ok_or_else(|| {
                replay_error(format!(
                    "the preservation record has no root `{}` at R{}",
                    root.root, root.index
                ))
            })?;
        bind(
            &format!("root `{}`'s program path", root.root),
            root.program.path == format!("evidence/program/R{}.json", root.index),
        )?;
        let program_bytes = check(&root.program)?;
        bind(
            &format!("root `{}`'s program digest", root.root),
            (preserved.program.byte_length, preserved.program.sha256)
                == (program_bytes.len() as u64, Sha256Digest::of(program_bytes)),
        )?;
        let program = Program::parse(program_bytes)
            .map_err(|reason| replay_error(format!("root `{}`: {reason}", root.root)))?;
        if program.to_file_bytes() != program_bytes {
            return Err(replay_error(format!(
                "root `{}`: the program is not canonical",
                root.root
            )));
        }
        match &root.optimality {
            Optimality::NotClaimed => {}
            Optimality::Gnaf(gnaf) => {
                let request_bytes = check(&gnaf.request)?.to_vec();
                let certificate_bytes = check(&gnaf.certificate)?.to_vec();
                let rechecked = check_gnaf(
                    &root.root,
                    &program,
                    request_bytes,
                    certificate_bytes,
                    gnaf.certificate_id,
                )
                .map_err(|error| replay_error(format!("root `{}`: {error}", root.root)))?;
                bind(
                    &format!("root `{}`'s GNAF universe", root.root),
                    rechecked.universe_id == gnaf.universe_id,
                )?;
                bind(
                    &format!("root `{}`'s GNAF verdict", root.root),
                    rechecked.verdict == gnaf.verdict,
                )?;
            }
        }
        if root.outputs.is_empty()
            || !strictly_ascending(&root.outputs, |output| output.target.id.clone())
        {
            return Err(replay_error(format!(
                "root `{}`: outputs are empty or not strictly ascending",
                root.root
            )));
        }
        for output in &root.outputs {
            outputs += 1;
            let target = &output.target.id;
            let descriptor = descriptors
                .get(target)
                .ok_or_else(|| replay_error(format!("`{target}` is not a registered target")))?;
            bind(
                &format!("root `{}`'s {target} descriptor digest", root.root),
                descriptor.digest == output.target.digest,
            )?;
            let rendering = preserved
                .renderings
                .iter()
                .find(|rendering| rendering.target == *target)
                .ok_or_else(|| {
                    replay_error(format!(
                        "root `{}` has no certified rendering in {target}",
                        root.root
                    ))
                })?;
            bind(
                &format!("root `{}`'s {target} entry", root.root),
                rendering.entry.function == output.entry.function
                    && rendering.entry.symbol == output.entry.symbol
                    && output.entry.symbol == format!("f{}", output.entry.function),
            )?;
            bind(
                &format!("root `{}`'s {target} crate path", root.root),
                output.krate.path == format!("evidence/crate/R{}.{target}.rs", root.index),
            )?;
            let crate_bytes = check(&output.krate)?;
            bind(
                &format!("root `{}`'s {target} crate digest", root.root),
                (rendering.krate.byte_length, rendering.krate.sha256)
                    == (crate_bytes.len() as u64, Sha256Digest::of(crate_bytes)),
            )?;
            // The certification is recomputed, never read: the record of
            // the certificates is what the attestation binds.
            bind(
                &format!("root `{}`'s {target} certification status", root.root),
                output.certification.status == CertificationStatus::KernelChecked
                    && output.certification.axioms == preservation.library.axioms,
            )?;
            let expected: [(CertificateRole, &str, &str, u64, Sha256Digest); 3] = [
                (
                    CertificateRole::A,
                    &preserved.module,
                    &preserved.theorem,
                    preserved.byte_length,
                    preserved.sha256,
                ),
                (
                    CertificateRole::B,
                    &rendering.module,
                    &rendering.theorem,
                    rendering.byte_length,
                    rendering.sha256,
                ),
                (
                    CertificateRole::E,
                    &rendering.composed.module,
                    &rendering.composed.theorem,
                    rendering.composed.byte_length,
                    rendering.composed.sha256,
                ),
            ];
            if output.certification.certificates.len() != expected.len() {
                return Err(replay_error(format!(
                    "root `{}`'s {target} certification does not state certificates A, B, and E",
                    root.root
                )));
            }
            for (stated_certificate, (role, module, theorem, length, sha)) in
                output.certification.certificates.iter().zip(expected)
            {
                bind(
                    &format!("root `{}`'s {target} certificate {role:?}", root.root),
                    stated_certificate.role == role
                        && stated_certificate.module == module
                        && stated_certificate.theorem == theorem
                        && stated_certificate.file.path
                            == format!("evidence/certificates/{}", certificate_path(module))
                        && (
                            stated_certificate.file.byte_length,
                            stated_certificate.file.sha256,
                        ) == (length, sha),
                )?;
                check(&stated_certificate.file)?;
            }
            // The package is re-rendered from its manifest and must be the
            // bundled package, exporting only the recorded entry.
            let package = &output.package;
            let directory = format!("packages/{target}/{}", package.name);
            bind(
                &format!("root `{}`'s {target} package directory", root.root),
                package.directory == directory && package.export == EXPORT_NAME,
            )?;
            let mut package_bytes: BTreeMap<String, &[u8]> = BTreeMap::new();
            for file in &package.files {
                let relative = file
                    .path
                    .strip_prefix(&format!("{directory}/"))
                    .ok_or_else(|| replay_error(format!("`{}` is outside its package", file.path)))?
                    .to_owned();
                package_bytes.insert(relative, check(file)?);
            }
            let manifest_bytes = package_bytes
                .get("package.json")
                .ok_or_else(|| replay_error(format!("{directory} has no package.json")))?;
            let manifest = Manifest::parse(manifest_bytes)
                .map_err(|reason| replay_error(format!("{directory}: {reason}")))?;
            let expected_manifest = manifest_for(
                &PackageName {
                    name: package.name.clone(),
                    version: package.version.clone(),
                },
                target,
                &program,
                output.entry.function,
                result.source.semantic_id,
            )
            .map_err(|reason| replay_error(format!("{directory}: {reason}")))?;
            bind(
                &format!("{directory}'s manifest"),
                manifest == expected_manifest
                    && manifest.sources == vec![semantic_hex.clone()]
                    && expected_manifest.to_file_bytes().ok().as_deref() == Some(*manifest_bytes),
            )?;
            let rendered = package::package(&manifest)
                .map_err(|reason| replay_error(format!("{directory}: {reason}")))?;
            let rendered_names: BTreeSet<&str> = rendered
                .files
                .keys()
                .map(String::as_str)
                .chain(["package.json"])
                .collect();
            let bundled_names: BTreeSet<&str> = package_bytes.keys().map(String::as_str).collect();
            bind(
                &format!("{directory}'s file set"),
                rendered_names == bundled_names,
            )?;
            for (relative, bytes) in &rendered.files {
                bind(
                    &format!("{directory}/{relative}"),
                    package_bytes.get(relative).copied() == Some(bytes.as_slice()),
                )?;
            }
            let library = rendered
                .files
                .get("src/lib.rs")
                .ok_or_else(|| replay_error(format!("{directory} renders no src/lib.rs")))?;
            bind(
                &format!("{directory}/src/lib.rs as the certified crate"),
                library.starts_with(crate_bytes),
            )?;
        }
    }
    let present: BTreeSet<String> = by_path.keys().map(|path| (*path).to_owned()).collect();
    if let Some(extra) = present.difference(&stated).next() {
        return Err(replay_error(format!(
            "the bundle holds `{extra}`, which the result does not state"
        )));
    }
    Ok(ReplayReport {
        spec: REPLAY_SPEC.to_owned(),
        result_id: result.result_id,
        status: ReplayStatus::BindingsVerified,
        outputs,
        not_rechecked: NOT_RECHECKED
            .iter()
            .map(|stage| (*stage).to_owned())
            .collect(),
    })
}

impl ReplayReport {
    /// The canonical file bytes.
    #[must_use]
    pub fn to_json(&self) -> Vec<u8> {
        canonical_bytes(self)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{attestation_facts, collect_bundle, replay_files, require_compiler_input_identity};
    use crate::artifact::canonical_json::Json;
    use crate::artifact::content_id::{attestation_id, Sha256Digest};
    use camino::Utf8Path;

    fn compiler_with_version(version: Json) -> Json {
        Json::object(vec![
            ("version", version),
            (
                "compiler_semantics",
                Json::Str(Sha256Digest::of(b"compiler semantics fixture").to_hex()),
            ),
        ])
    }

    fn hashed_attestation(mut body: BTreeMap<String, Json>) -> Vec<u8> {
        body.remove("attestation_id");
        let id = attestation_id(&Json::Obj(body.clone()).to_canonical_string());
        body.insert("attestation_id".to_owned(), Json::Str(id.to_hex()));
        Json::Obj(body).to_file_bytes()
    }

    fn attestation_with_compiler(compiler: Option<Json>) -> Vec<u8> {
        let digest = Sha256Digest::of(b"attestation fixture").to_hex();
        let row = || {
            Json::object(vec![
                ("byte_length", Json::Int(1)),
                ("sha256", Json::Str(digest.clone())),
            ])
        };
        let mut fields = vec![
            ("spec", Json::Str("lexlean/attestation/2".to_owned())),
            ("status", Json::Str("verified".to_owned())),
            ("source_id", Json::Str(digest.clone())),
            ("semantic_id", Json::Str(digest.clone())),
            ("build_id", Json::Str(digest.clone())),
            ("compiler_input", row()),
            ("preservation", row()),
            (
                "toolchain",
                Json::object(vec![
                    (
                        "lean",
                        Json::object(vec![(
                            "version_output",
                            Json::Str("Lean (version 4.32.1, release)".to_owned()),
                        )]),
                    ),
                    (
                        "lake",
                        Json::object(vec![(
                            "version_output",
                            Json::Str("Lake version 5.0.0 (Lean version 4.32.1)".to_owned()),
                        )]),
                    ),
                ]),
            ),
        ];
        if let Some(compiler) = compiler {
            fields.push(("lexlean", compiler));
        }
        hashed_attestation(
            fields
                .into_iter()
                .map(|(key, value)| (key.to_owned(), value))
                .collect(),
        )
    }

    fn edited_attestation(path: &[&str], value: Option<Json>) -> Vec<u8> {
        let bytes =
            attestation_with_compiler(Some(compiler_with_version(Json::Str("0.0.1".to_owned()))));
        let Json::Obj(mut body) = Json::parse(&bytes).expect("fixture JSON") else {
            panic!("fixture object");
        };
        let (field, parents) = path.split_last().expect("a field path");
        let mut object = &mut body;
        for parent in parents {
            let Some(Json::Obj(child)) = object.get_mut(*parent) else {
                panic!("fixture object {parent}");
            };
            object = child;
        }
        if let Some(value) = value {
            object.insert((*field).to_owned(), value);
        } else {
            object.remove(*field);
        }
        hashed_attestation(body)
    }

    #[test]
    fn attestation_facts_refuses_missing_compiler_version() {
        for compiler in [None, Some(Json::object(Vec::new()))] {
            assert!(
                attestation_facts(&attestation_with_compiler(compiler)).is_err(),
                "a hash-correct attestation must state its producing compiler version"
            );
        }
    }

    #[test]
    fn attestation_facts_refuses_non_string_compiler_version() {
        let compiler = compiler_with_version(Json::Int(1));
        assert!(
            attestation_facts(&attestation_with_compiler(Some(compiler))).is_err(),
            "a hash-correct attestation's producing compiler version must be a string"
        );
    }

    #[test]
    fn attestation_facts_refuses_empty_compiler_version() {
        let compiler = compiler_with_version(Json::Str(String::new()));
        assert!(
            attestation_facts(&attestation_with_compiler(Some(compiler))).is_err(),
            "a hash-correct attestation's producing compiler version must not be empty"
        );
    }

    #[test]
    fn attestation_facts_accepts_a_producing_compiler_version() {
        let compiler = compiler_with_version(Json::Str("0.0.1".to_owned()));
        let facts = attestation_facts(&attestation_with_compiler(Some(compiler)))
            .expect("the producing version may differ from this compiler's version");
        assert_eq!(facts.compiler_version, "0.0.1");
    }

    #[test]
    fn attestation_facts_refuses_missing_or_malformed_compiler_semantics() {
        for value in [
            None,
            Some(Json::Int(1)),
            Some(Json::Str(String::new())),
            Some(Json::Str("not a digest".to_owned())),
            Some(Json::Str("A".repeat(64))),
        ] {
            let bytes = edited_attestation(&["lexlean", "compiler_semantics"], value.clone());
            assert!(
                attestation_facts(&bytes).is_err(),
                "a hash-correct attestation needs a compiler-semantics digest: {value:?}"
            );
        }
    }

    #[test]
    fn attestation_facts_refuses_missing_or_malformed_verification_envelope() {
        for (field, wrong) in [("spec", "lexlean/attestation/1"), ("status", "unverified")] {
            for value in [None, Some(Json::Int(1)), Some(Json::Str(wrong.to_owned()))] {
                let bytes = edited_attestation(&[field], value.clone());
                assert!(
                    attestation_facts(&bytes).is_err(),
                    "a hash-correct attestation needs its verification envelope: {field} = {value:?}"
                );
            }
        }
    }

    #[test]
    fn attestation_facts_refuses_missing_or_malformed_toolchain_facts() {
        let paths: &[&[&str]] = &[
            &["toolchain"],
            &["toolchain", "lean"],
            &["toolchain", "lake"],
            &["toolchain", "lean", "version_output"],
            &["toolchain", "lake", "version_output"],
        ];
        for path in paths {
            for value in [None, Some(Json::Int(1)), Some(Json::object(Vec::new()))] {
                let bytes = edited_attestation(path, value.clone());
                assert!(
                    attestation_facts(&bytes).is_err(),
                    "a hash-correct attestation needs toolchain facts: {path:?} = {value:?}"
                );
            }
        }
    }

    #[test]
    fn attestation_facts_refuses_foreign_or_colliding_lean_versions() {
        for (tool, banner) in [
            ("lean", ""),
            ("lean", "unrecognized banner 4.32.1"),
            ("lean", "Lean (version 4.32.10, release)"),
            ("lean", "Lean (version 4.32.1-rc1, release)"),
            ("lean", "Lean (version 4.31.0, built with 4.32.1)"),
            ("lean", "Lean (version 4.32.1, release"),
            ("lean", "Lean (version 4.32.1, release)\nextra output"),
            ("lake", ""),
            ("lake", "unrecognized banner 4.32.1"),
            ("lake", "Lake version 5.0.0 (Lean version 4.32.10)"),
            ("lake", "Lake version 5.0.0 (Lean version 4.32.1-rc1)"),
            ("lake", "Lake version 4.32.1 (Lean version 4.31.0)"),
            ("lake", "Lake version 5.0.0 (Lean version 4.32.1"),
            (
                "lake",
                "Lake version 5.0.0 (Lean version 4.32.1)\nextra output",
            ),
        ] {
            let bytes = edited_attestation(
                &["toolchain", tool, "version_output"],
                Some(Json::Str(banner.to_owned())),
            );
            assert!(
                attestation_facts(&bytes).is_err(),
                "a hash-correct attestation needs the pinned Lean version: {tool}: {banner:?}"
            );
        }
    }

    #[test]
    fn compiler_input_identity_refuses_missing_malformed_or_foreign_facts() {
        for value in [
            serde_json::json!({}),
            serde_json::json!([{"version": crate::LEAN_VERSION, "githash": crate::LEAN_GITHASH}]),
            serde_json::json!({"lean": [crate::LEAN_VERSION, crate::LEAN_GITHASH]}),
            serde_json::json!({"lean": 1}),
            serde_json::json!({"lean": {"version": crate::LEAN_VERSION}}),
            serde_json::json!({"lean": {"githash": crate::LEAN_GITHASH}}),
            serde_json::json!({"lean": {"version": 1, "githash": crate::LEAN_GITHASH}}),
            serde_json::json!({"lean": {"version": crate::LEAN_VERSION, "githash": 1}}),
            serde_json::json!({"lean": {"version": "4.32.10", "githash": crate::LEAN_GITHASH}}),
            serde_json::json!({"lean": {"version": crate::LEAN_VERSION, "githash": "0".repeat(40)}}),
        ] {
            let bytes = serde_json::to_vec(&value).expect("JSON");
            assert!(
                require_compiler_input_identity(&bytes).is_err(),
                "a compiler input needs the pinned Lean identity: {value}"
            );
        }
        let version = serde_json::to_string(crate::LEAN_VERSION).expect("JSON string");
        let githash = serde_json::to_string(crate::LEAN_GITHASH).expect("JSON string");
        let lean = format!("{{\"version\":{version},\"githash\":{githash}}}");
        for bytes in [
            format!("{{\"lean\":{lean}}} null"),
            format!("{{\"lean\":{lean},\"lean\":{lean}}}"),
            format!("{{\"lean\":{{\"version\":{version},\"version\":{version},\"githash\":{githash}}}}}"),
            format!("{{\"lean\":{{\"version\":{version},\"githash\":{githash},\"githash\":{githash}}}}}"),
        ] {
            assert!(
                require_compiler_input_identity(bytes.as_bytes()).is_err(),
                "a compiler input needs one unambiguous Lean identity: {bytes}"
            );
        }
    }

    #[test]
    fn compiler_input_identity_skips_deep_payloads_before_and_after_lean() {
        let lean = serde_json::json!({
            "version": crate::LEAN_VERSION,
            "githash": crate::LEAN_GITHASH,
        })
        .to_string();
        for depth in [129, 257, 2048] {
            let nested = format!("{}0{}", "[".repeat(depth), "]".repeat(depth));
            for bytes in [
                format!("{{\"payload\":{nested},\"lean\":{lean}}}"),
                format!("{{\"lean\":{lean},\"payload\":{nested}}}"),
            ] {
                require_compiler_input_identity(bytes.as_bytes())
                    .expect("reading the identity does not impose a payload nesting limit");
            }
        }
    }

    fn literal_backslashes() -> bool {
        if std::path::Path::new("evidence\\attestation.json")
            .components()
            .count()
            == 1
        {
            true
        } else {
            eprintln!("bundle inventory: this host treats backslashes as path separators; literal-backslash assertions did not run (§8.3)");
            false
        }
    }

    #[test]
    fn bundle_collection_preserves_literal_backslash_filenames() {
        let directory = tempfile::tempdir().expect("tempdir");
        let bundle = Utf8Path::from_path(directory.path()).expect("UTF-8 tempdir");
        std::fs::create_dir(bundle.join("evidence")).expect("mkdir");
        std::fs::write(bundle.join("evidence/attestation.json"), b"canonical").expect("write");
        assert_eq!(
            collect_bundle(bundle).expect("collect"),
            [(
                "evidence/attestation.json".to_owned(),
                b"canonical".to_vec()
            )]
        );
        if literal_backslashes() {
            std::fs::write(bundle.join("evidence\\attestation.json"), b"alias").expect("write");
            assert_eq!(
                collect_bundle(bundle).expect("collect"),
                [
                    (
                        "evidence/attestation.json".to_owned(),
                        b"canonical".to_vec()
                    ),
                    ("evidence\\attestation.json".to_owned(), b"alias".to_vec()),
                ]
            );
        }
    }

    #[test]
    fn bundle_collection_does_not_restore_a_moved_canonical_filename() {
        let directory = tempfile::tempdir().expect("tempdir");
        let bundle = Utf8Path::from_path(directory.path()).expect("UTF-8 tempdir");
        std::fs::create_dir(bundle.join("evidence")).expect("mkdir");
        let canonical = bundle.join("evidence/attestation.json");
        std::fs::write(&canonical, b"canonical").expect("write");
        assert_eq!(collect_bundle(bundle).expect("collect").len(), 1);
        if literal_backslashes() {
            std::fs::rename(&canonical, bundle.join("evidence\\attestation.json")).expect("rename");
            assert_eq!(
                collect_bundle(bundle).expect("collect"),
                [(
                    "evidence\\attestation.json".to_owned(),
                    b"canonical".to_vec()
                )]
            );
        }
    }

    #[test]
    fn bundle_collection_refuses_non_utf8_filenames() {
        let directory = tempfile::tempdir().expect("tempdir");
        let bundle = Utf8Path::from_path(directory.path()).expect("UTF-8 tempdir");
        std::fs::write(bundle.join("canonical.json"), b"canonical").expect("write");
        assert_eq!(collect_bundle(bundle).expect("collect").len(), 1);
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStrExt;

            let invalid = std::ffi::OsStr::from_bytes(b"invalid\xff.json");
            if std::fs::write(directory.path().join(invalid), b"invalid").is_ok() {
                let error = collect_bundle(bundle).expect_err("non-UTF-8 filenames are refused");
                assert_eq!(error.diagnostics[0].code.as_str(), "LLV7102");
                assert!(error.diagnostics[0].message.contains("UTF-8"));
                return;
            }
        }
        eprintln!("bundle inventory: this host's filesystem refuses non-UTF-8 filenames; only the portable assertions ran (§8.3)");
    }

    #[test]
    fn replay_inventory_refuses_duplicate_paths_before_reading_the_result() {
        let files = vec![
            ("evidence/attestation.json".to_owned(), b"same".to_vec()),
            ("evidence/attestation.json".to_owned(), b"same".to_vec()),
        ];
        let error = replay_files(&files).expect_err("duplicate inventory paths are refused");
        assert_eq!(error.diagnostics[0].code.as_str(), "LLV7102");
        assert!(error.diagnostics[0].message.contains("duplicate"));
        assert!(error.diagnostics[0]
            .message
            .contains("evidence/attestation.json"));
    }
}
