//! LexLean: a closed-lexicon LaTeX-to-Lean 4 compiler whose canonical
//! document and prose-free Lean program are generated from one semantic
//! representation.
//!
//! The complete implementation contract is `SPEC.md` at the repository root;
//! nothing here is normative beyond what it authorizes. The stable public
//! surface is [`Engine`] with its request and result types (SPEC.md §24);
//! the full mutable compiler IR stays internal.

#![forbid(unsafe_code)]

// The stable surface (§24): the engine, its requests and results, the
// error model, the diagnostic type, and the artifact schema types.
pub mod api;
pub mod diagnostic;
pub mod error;

// Unstable internals. They are `pub` because the repository's conformance
// runner and `xtask` drive the compiler stage by stage, but they are
// hidden from documentation and carry no stability promise: nothing
// outside this repository may rely on them (§24.4, "the full mutable
// compiler IR remains internal").
#[doc(hidden)]
pub mod artifact;
#[doc(hidden)]
pub mod backend;
#[doc(hidden)]
pub mod calculus;
#[doc(hidden)]
pub mod cli;
#[doc(hidden)]
pub mod config;
#[doc(hidden)]
pub mod elaborate;
#[doc(hidden)]
pub mod fmt;
#[doc(hidden)]
pub mod gnaf;
#[doc(hidden)]
pub mod grammar;
#[doc(hidden)]
pub mod ir;
#[doc(hidden)]
pub mod lexicon;
#[doc(hidden)]
pub mod link;
#[doc(hidden)]
pub mod lock;
#[doc(hidden)]
pub mod production;
#[doc(hidden)]
pub mod project;
#[doc(hidden)]
pub mod source;
#[doc(hidden)]
pub mod verify;

pub use api::{
    BuildRequest, BuiltUnit, CheckRequest, CheckedUnit, CheckedUnitSummary, Engine, FormatRequest,
    FormatResultSet, LockRequest, LockResult, ModuleArtifacts, ProjectResultSet, Selection,
    VerifiedProject, VerifiedUnit, VerifyRequest,
};
pub use artifact::content_id::Sha256Digest;
pub use artifact::snapshot::{
    SemanticSnapshot, SnapshotAxiomPolicy, SnapshotDeclaration, SnapshotElaboration,
    SnapshotModule, SnapshotOrigin, SnapshotRange, SnapshotSource,
};
// These owned values are the closed language-1.1 and language-1.2 portion of
// the stable snapshot contract. Re-exporting them under snapshot-specific names lets a
// downstream consumer exhaustively inspect every variant without depending
// on the hidden compiler-module path.
pub use diagnostic::{Diagnostic, DiagnosticCode, DiagnosticDetail, Label, Note, Span};
pub use error::{ErrorClass, LexLeanError};
pub use ir::semantic::{
    MemberRef as SnapshotMemberRef, ReasoningClaim as SnapshotReasoningClaim,
    ReasoningStrategy as SnapshotReasoningStrategy, SearchOrder as SnapshotSearchOrder,
    SemanticAssignment as SnapshotAssignment, SemanticBranch as SnapshotBranch,
    SemanticConstructor as SnapshotConstructor, SemanticDeclaration as SnapshotSemanticDeclaration,
    SemanticEdge as SnapshotEdge, SemanticField as SnapshotField,
    SemanticInteger as SnapshotInteger, SemanticMapEntry as SnapshotMapEntry,
    SemanticModule as SnapshotSemanticModule, SemanticParameter as SnapshotParameter,
    SemanticPrimitive as SnapshotPrimitive, SemanticProduction as SnapshotProduction,
    SemanticProof as SnapshotProof, SemanticProofBranch as SnapshotProofBranch,
    SemanticReflection as SnapshotReflection,
    SemanticReflectionComparison as SnapshotReflectionComparison,
    SemanticReflectionField as SnapshotReflectionField, SemanticTerm as SnapshotTerm,
    SemanticTermination as SnapshotTermination, SemanticType as SnapshotType,
};

use artifact::content_id::tree_digest;

/// The embedded normative data: `language/`, `schemas/`, and the committed
/// axiom-parser and canonical-JSON golden fixtures (SPEC.md §21.2).
pub mod embedded {
    include!(concat!(env!("OUT_DIR"), "/embedded.rs"));
}

/// The original language identifier. Kept as the compatibility default for
/// downstream code that compiled against LexLean 1.0.
pub const LANGUAGE_VERSION: &str = "1.0";

/// The language 1.1 identifier.
pub const LANGUAGE_1_1: &str = "1.1";

/// The language 1.2 identifier.
pub const LANGUAGE_1_2: &str = "1.2";

/// The newest supported language identifier, whose compiler-semantics ID
/// covers the whole embedded tree. `init` keeps `1.0` as its default so an
/// unqualified invocation never changes meaning.
pub const LATEST_LANGUAGE_VERSION: &str = "1.2";

/// The closed set of language versions accepted by this compiler.
pub const LANGUAGE_VERSIONS: &[&str] = &[LANGUAGE_VERSION, LANGUAGE_1_1, LANGUAGE_1_2];

/// Whether a project language is supported.
#[must_use]
pub fn supports_language(language: &str) -> bool {
    LANGUAGE_VERSIONS.contains(&language)
}

/// The exact pinned Lean toolchain string (SPEC.md §8.2).
pub const LEAN_TOOLCHAIN: &str = "leanprover/lean4:v4.32.1";

/// The Lean version a verification environment must report (SPEC.md §8.2).
pub const LEAN_VERSION: &str = "4.32.1";

/// The compiler crate version.
pub const COMPILER_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The compiler-semantics ID: the §11.5 tree digest of every embedded
/// normative file, computed once per process (SPEC.md §21.2). Repository
/// tests recompute the same digest from disk and compare (RP-10).
#[must_use]
pub fn compiler_semantics_id() -> Sha256Digest {
    compiler_semantics_id_for(LANGUAGE_VERSION)
}

fn is_v1_1_file(path: &str) -> bool {
    path.starts_with("language/core-1.1/")
        || path.starts_with("language/std/nat-1.1/")
        || path.starts_with("language/std/int-1.1/")
        || path.starts_with("language/std/bool-1.1/")
        || path == "language/bootstrap-1.1.toml"
        || path == "language/semantics-1.1.toml"
        || path == "schemas/semantic-module.schema.json"
        || path == "schemas/semantic-snapshot.schema.json"
}

fn is_v1_2_file(path: &str) -> bool {
    path.starts_with("language/core-1.2/")
        || path.starts_with("language/std/nat-1.2/")
        || path.starts_with("language/std/int-1.2/")
        || path.starts_with("language/std/bool-1.2/")
        || path == "language/bootstrap-1.2.toml"
        || path == "language/semantics-1.2.toml"
        || path == "schemas/attestation-v2.schema.json"
        || path == "schemas/build-manifest-v2.schema.json"
        || path == "schemas/lexicon-v2.schema.json"
        || path == "schemas/lock-1.1.schema.json"
        || path == "schemas/lock-v2.schema.json"
        || path == "schemas/project-v2.schema.json"
        || path == "schemas/semantic-module-v2.schema.json"
        || path == "schemas/semantic-snapshot-v2.schema.json"
        || path == "schemas/production-eligibility.schema.json"
        || path == crate::production::REGISTRY_PATH
        || path.starts_with("language/lcnf-1.2/")
        || path == "schemas/compiler-input.schema.json"
        || path == "schemas/gnaf-fixture.schema.json"
        || path == "schemas/gnaf-request.schema.json"
        || path == "schemas/target-fixture.schema.json"
        || path == "schemas/target-program.schema.json"
        || path == "schemas/rust-package.schema.json"
        || path == "schemas/rust-provenance.schema.json"
}

/// The compiler-semantics ID for one selected language. Language 1.0 excludes
/// all files introduced solely for 1.1 and 1.2, and Language 1.1 excludes
/// all files introduced solely for 1.2, preserving historical identities.
#[must_use]
pub fn compiler_semantics_id_for(language: &str) -> Sha256Digest {
    use std::sync::OnceLock;
    static V1_0: OnceLock<Sha256Digest> = OnceLock::new();
    static V1_1: OnceLock<Sha256Digest> = OnceLock::new();
    static V1_2: OnceLock<Sha256Digest> = OnceLock::new();
    match language {
        LANGUAGE_VERSION => *V1_0.get_or_init(|| {
            let files: Vec<(&str, &[u8])> = embedded::FILES
                .iter()
                .copied()
                .filter(|(path, _)| !is_v1_1_file(path) && !is_v1_2_file(path))
                .collect();
            tree_digest(&files)
        }),
        LANGUAGE_1_1 => *V1_1.get_or_init(|| {
            let files: Vec<(&str, &[u8])> = embedded::FILES
                .iter()
                .copied()
                .filter(|(path, _)| !is_v1_2_file(path))
                .collect();
            tree_digest(&files)
        }),
        LANGUAGE_1_2 => *V1_2.get_or_init(|| {
            let files: Vec<(&str, &[u8])> = embedded::FILES.to_vec();
            tree_digest(&files)
        }),
        // An unsupported language never shares an identity with a supported
        // one; configuration rejects it before any identity is published.
        other => Sha256Digest::of(format!("unsupported-language\0{other}").as_bytes()),
    }
}
