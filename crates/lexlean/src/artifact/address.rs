//! UOR addresses (SPEC.md §21.10): every content identity LexLean records is
//! the SHA-256 of one artifact's canonical bytes, and its UOR address is that
//! digest written `sha256:<64 lowercase hex digits>`, the form of the UOR
//! Foundation's `uor-addr` labels.
//!
//! Each artifact kind has one function here that returns its canonical
//! bytes, and [`Canonical::address`] is the only address function. The
//! framed kinds are written by the same recipe as the identity
//! (`content_id::*_frames`), into a buffer instead of a hash, so a recorded
//! identity and the address of the canonical bytes cannot differ. The kind
//! is separated inside the hashed bytes: every framed kind begins with its
//! own domain and a NUL, and the untagged kinds are file bytes, which no
//! domain-prefixed message equals by accident of construction.

use std::fmt;

use crate::artifact::canonical_json::Json;
use crate::artifact::content_id::{
    attestation_frames, build_frames, pdf_recipe_frames, semantic_frames, source_frames,
    tree_frames, Sha256Digest,
};
use crate::link::CheckedProject;

/// The hash function of every LexLean address, and the label's prefix.
pub const ALGORITHM: &str = "sha256";

/// A UOR address: the SHA-256 of canonical bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address(pub Sha256Digest);

impl Address {
    /// The address of `canonical` bytes.
    #[must_use]
    pub fn of(canonical: &[u8]) -> Self {
        Self(Sha256Digest::of(canonical))
    }

    /// The digest, which is the identity LexLean records (bare hex).
    #[must_use]
    pub const fn digest(self) -> Sha256Digest {
        self.0
    }

    /// Read the label form, refusing every other spelling: another algorithm,
    /// upper-case hex, a missing or extra digit.
    ///
    /// # Errors
    ///
    /// What is wrong with `text`.
    pub fn parse(text: &str) -> Result<Self, String> {
        let hex = text
            .strip_prefix(ALGORITHM)
            .and_then(|rest| rest.strip_prefix(':'))
            .ok_or_else(|| format!("an address begins `{ALGORITHM}:`: {text}"))?;
        Sha256Digest::from_hex(hex).map(Self)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{ALGORITHM}:{}", self.0.to_hex())
    }
}

/// Every kind of artifact LexLean identifies (§21.10).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    /// A file's bytes as stored: lock workspace files, package files,
    /// `.olean` files, executables, process streams, PDFs.
    File,
    /// The §11.5 tree of `(path, bytes)` rows.
    Tree,
    /// The compiler-semantics ID: the tree of a language's embedded files.
    CompilerSemantics,
    /// The canonical `lexlean.toml` (§10.1), `project_config_sha256`.
    ProjectConfig,
    /// The source ID (§21.3).
    Source,
    /// The semantic ID (§21.4).
    Semantic,
    /// The build ID (§21.5).
    Build,
    /// The attestation ID (§22.9).
    Attestation,
    /// The PDF recipe ID (§19.8).
    PdfRecipe,
    /// A Rust package's program (§17.23): its canonical JSON file bytes.
    RustProgram,
}

impl Kind {
    /// The domain that begins the kind's canonical bytes, if it has one.
    #[must_use]
    pub const fn domain(self) -> Option<&'static str> {
        match self {
            Self::File | Self::ProjectConfig | Self::RustProgram => None,
            Self::Tree | Self::CompilerSemantics => Some("lexlean-tree-v1"),
            Self::Source => Some("lexlean-source-v1"),
            Self::Semantic => Some("lexlean-semantic-v1"),
            Self::Build => Some("lexlean-build-v1"),
            Self::Attestation => Some("lexlean-attestation-v1"),
            Self::PdfRecipe => Some("lexlean-pdf-recipe-v1"),
        }
    }
}

/// One artifact's canonical bytes, with their kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Canonical {
    /// What the bytes are.
    pub kind: Kind,
    /// The canonical bytes.
    pub bytes: Vec<u8>,
}

impl Canonical {
    /// The UOR address of these bytes.
    #[must_use]
    pub fn address(&self) -> Address {
        Address::of(&self.bytes)
    }
}

/// A file: its bytes.
#[must_use]
pub fn file(bytes: &[u8]) -> Canonical {
    Canonical {
        kind: Kind::File,
        bytes: bytes.to_vec(),
    }
}

/// A §11.5 tree over `(path, bytes)` rows in strictly sorted path order.
#[must_use]
pub fn tree(files: &[(&str, &[u8])]) -> Canonical {
    Canonical {
        kind: Kind::Tree,
        bytes: tree_frames(Vec::new(), files),
    }
}

/// The compiler-semantics ID of `language`: the tree of the embedded files
/// that language selects (§21.2). `None` for an unsupported language.
#[must_use]
pub fn compiler_semantics(language: &str) -> Option<Canonical> {
    let rows = crate::compiler_semantics_files(language)?;
    let files: Vec<(&str, &[u8])> = rows
        .iter()
        .map(|(path, bytes)| (*path, bytes.as_ref()))
        .collect();
    Some(Canonical {
        kind: Kind::CompilerSemantics,
        bytes: tree_frames(Vec::new(), &files),
    })
}

/// A project's configuration: its canonical `lexlean.toml`.
#[must_use]
pub fn project_config(config: &crate::config::ProjectConfig) -> Canonical {
    Canonical {
        kind: Kind::ProjectConfig,
        bytes: config.canonical_toml().into_bytes(),
    }
}

/// The source ID's message (§21.3).
#[must_use]
pub fn source(
    canonical_project_toml: &[u8],
    canonical_lock: &[u8],
    sources: &[(String, Vec<u8>)],
) -> Canonical {
    Canonical {
        kind: Kind::Source,
        bytes: source_frames(Vec::new(), canonical_project_toml, canonical_lock, sources),
    }
}

/// The semantic ID's message (§21.4).
#[must_use]
pub fn semantic(
    compiler_semantics: Sha256Digest,
    language: &str,
    linked_ir_json: &str,
    lexicon_closure_json: &str,
) -> Canonical {
    Canonical {
        kind: Kind::Semantic,
        bytes: semantic_frames(
            Vec::new(),
            compiler_semantics,
            language,
            linked_ir_json,
            lexicon_closure_json,
        ),
    }
}

/// The build ID's message (§21.5).
#[must_use]
pub fn build(source: Sha256Digest, semantic: Sha256Digest) -> Canonical {
    Canonical {
        kind: Kind::Build,
        bytes: build_frames(Vec::new(), source, semantic),
    }
}

/// The attestation ID's message (§22.9).
#[must_use]
pub fn attestation(body_without_id_json: &str) -> Canonical {
    Canonical {
        kind: Kind::Attestation,
        bytes: attestation_frames(Vec::new(), body_without_id_json),
    }
}

/// The PDF recipe ID's message (§19.8).
#[must_use]
pub fn pdf_recipe(
    tex_sha256: Sha256Digest,
    program_sha256: Sha256Digest,
    version_stdout_sha256: Sha256Digest,
    compile_argv_json: &str,
    resources_json: &str,
) -> Canonical {
    Canonical {
        kind: Kind::PdfRecipe,
        bytes: pdf_recipe_frames(
            Vec::new(),
            tex_sha256,
            program_sha256,
            version_stdout_sha256,
            compile_argv_json,
            resources_json,
        ),
    }
}

/// A Rust package's program: its canonical JSON with one final LF.
#[must_use]
pub fn rust_program(program: &Json) -> Canonical {
    Canonical {
        kind: Kind::RustProgram,
        bytes: program.to_file_bytes(),
    }
}

/// The content identities of one checked project, as canonical bytes.
#[derive(Debug, Clone)]
pub struct Identities {
    /// The canonical `lexlean.toml`.
    pub project_config: Canonical,
    /// The compiler-semantics tree of the project's language.
    pub compiler_semantics: Canonical,
    /// The source ID's message.
    pub source: Canonical,
    /// The semantic ID's message.
    pub semantic: Canonical,
    /// The build ID's message.
    pub build: Canonical,
}

/// The canonical bytes of every content identity of `checked`, a check of
/// the project configured by `config`.
///
/// # Panics
///
/// When `config` names an unsupported language, which configuration refuses
/// before any project is checked.
#[must_use]
pub fn identities(config: &crate::config::ProjectConfig, checked: &CheckedProject) -> Identities {
    let project_config = project_config(config);
    let compiler_semantics =
        compiler_semantics(&config.language).expect("a checked project has a supported language");
    let mut sources: Vec<(String, Vec<u8>)> = checked
        .modules
        .values()
        .map(|module| {
            (
                module.document.source_path.clone(),
                module.normalized.clone().into_bytes(),
            )
        })
        .collect();
    sources.sort_by(|a, b| a.0.cmp(&b.0));
    let source = source(&project_config.bytes, &checked.canonical_lock, &sources);
    let semantic = semantic(
        compiler_semantics.address().digest(),
        &config.language,
        &checked.linked_json().to_canonical_string(),
        &checked
            .closure
            .closure_json("", &checked.visible_union)
            .to_canonical_string(),
    );
    let build = build(source.address().digest(), semantic.address().digest());
    Identities {
        project_config,
        compiler_semantics,
        source,
        semantic,
        build,
    }
}

#[cfg(test)]
mod tests {
    use super::{Address, Kind, ALGORITHM};
    use crate::artifact::content_id::{
        attestation_id, build_id, pdf_recipe_id, semantic_id_for, source_id, tree_digest,
        Sha256Digest,
    };

    #[test]
    fn every_recorded_identity_is_the_address_of_its_canonical_bytes() {
        let a = Sha256Digest::of(b"a");
        let b = Sha256Digest::of(b"b");
        let sources = vec![("src/Main.lex.tex".to_owned(), b"body".to_vec())];
        let files: [(&str, &[u8]); 2] = [("a", b"1"), ("b/c", b"2")];
        for (canonical, recorded) in [
            (super::tree(&files), tree_digest(&files)),
            (
                super::source(b"toml", b"lock", &sources),
                source_id(b"toml", b"lock", &sources),
            ),
            (
                super::semantic(a, "1.2", "{}", "[]"),
                semantic_id_for(a, "1.2", "{}", "[]"),
            ),
            (super::build(a, b), build_id(a, b)),
            (super::attestation("{}"), attestation_id("{}")),
            (
                super::pdf_recipe(a, b, a, "[]", "[]"),
                pdf_recipe_id(a, b, a, "[]", "[]"),
            ),
            (super::file(b"bytes"), Sha256Digest::of(b"bytes")),
        ] {
            assert_eq!(
                canonical.address().digest(),
                recorded,
                "{:?}",
                canonical.kind
            );
            if let Some(domain) = canonical.kind.domain() {
                assert!(
                    canonical
                        .bytes
                        .starts_with(format!("{domain}\0").as_bytes()),
                    "{:?} begins with its domain",
                    canonical.kind
                );
            }
        }
        assert_eq!(
            super::compiler_semantics(crate::LANGUAGE_1_2)
                .expect("a supported language")
                .address()
                .digest(),
            crate::compiler_semantics_id_for(crate::LANGUAGE_1_2)
        );
        assert!(super::compiler_semantics("0.9").is_none());
        // The canonical JSON of a small object is the canonical form the UOR
        // Foundation's `uor-addr` labels (`tests/byte_identity.rs`), and its
        // address is that crate's vector.
        let object = crate::artifact::canonical_json::Json::object(vec![(
            "foo",
            crate::artifact::canonical_json::Json::Str("bar".to_owned()),
        )]);
        assert_eq!(
            Address::of(object.to_canonical_string().as_bytes()).to_string(),
            "sha256:7a38bf81f383f69433ad6e900d35b3e2385593f76a7b7ab5d4355b8ba41ee24b"
        );
        assert_eq!(Kind::File.domain(), None);
    }

    #[test]
    fn the_label_is_sha256_and_lowercase_hex() {
        let address = Address::of(b"");
        let label = address.to_string();
        assert_eq!(
            label,
            "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(label.len(), 71);
        assert_eq!(Address::parse(&label), Ok(address));
        for refused in [
            label.replacen(ALGORITHM, "blake3", 1),
            label.to_uppercase(),
            label[..70].to_owned(),
            format!("{label}0"),
            label.replacen(':', "-", 1),
        ] {
            assert!(Address::parse(&refused).is_err(), "{refused}");
        }
    }
}
