//! The lexeme ledger: canonical identity, authorship, external time, and an
//! append-only transparency log (SPEC.md §33).
//!
//! The claim this module makes is deliberately narrow. A verified entry
//! establishes three things and nothing else: that a specification existed at a
//! stated time, that it was authored by the holder of a stated key, and that it
//! has not been modified since. It never establishes novelty, validity, or
//! legal effect, and [`verify::Verdict`] is worded so that it cannot be read as
//! saying more than the cryptography carries.
//!
//! The pipeline is one direction with no shortcuts: a Lean source is
//! canonicalized ([`canonical`]), hashed into a content digest, signed
//! ([`signature`]), timestamped ([`timestamp`]), appended to an RFC 6962 log
//! ([`merkle`], [`ledger`]), and then re-checked from the entry alone
//! ([`verify`]). No verification step reads the inventor's filesystem, so a
//! third party needs only the entry.

pub mod canonical;
pub mod cli;
pub mod contractivity;
pub mod entry;
pub mod ledger;
pub mod merkle;
pub mod pirtm;
pub mod rational;
pub mod signature;
pub mod stratum;
pub mod timestamp;
pub mod tsa_crypto;
pub mod verify;
pub mod zeno;

/// The schema identifier every entry carries (SPEC.md §33.2).
pub const ENTRY_SPEC: &str = "lexlean/lexeme/1";

/// The canonicalization identifier (SPEC.md §33.1). Structural, not semantic:
/// two sources that elaborate alike but tokenize differently hash differently,
/// and no entry may claim otherwise.
pub const CANONICALIZATION: &str = "lean-token-stream-v1";

/// The hash domain of the canonical form (SPEC.md §33.1).
pub const CANONICAL_DOMAIN: &str = "lexlean-lexeme-v1";

/// The signature algorithm every entry uses (SPEC.md §33.3).
pub const SIGNATURE_ALGORITHM: &str = "ed25519";

/// The default RFC 3161 time stamp authority (SPEC.md §33.4).
pub const DEFAULT_TSA_URL: &str = "https://freetsa.org/tsr";

/// The timestamp artifact selector: an entry timestamps its detached
/// signature, not the bare digest, so that the token fixes the exact 64 bytes
/// a verifier checks (SPEC.md §33.6).
pub const TIMESTAMP_ARTIFACT: &str = "signature";
