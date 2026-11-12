//! Content identity (SPEC.md §21.1–§21.5, §22.9, §19.8): the frame function,
//! the tree digest, and every labeled hash recipe.

use sha2::{Digest, Sha256};

/// A SHA-256 digest with lowercase-hex display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sha256Digest(pub [u8; 32]);

impl Sha256Digest {
    /// Hash raw bytes.
    #[must_use]
    pub fn of(bytes: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        Self(hasher.finalize().into())
    }

    /// The 64-character lowercase hexadecimal form.
    #[must_use]
    pub fn to_hex(&self) -> String {
        let mut out = String::with_capacity(64);
        for byte in self.0 {
            out.push(char::from_digit(u32::from(byte >> 4), 16).expect("nibble"));
            out.push(char::from_digit(u32::from(byte & 0xf), 16).expect("nibble"));
        }
        out
    }

    /// Parse 64 lowercase hex digits.
    pub fn from_hex(text: &str) -> Result<Self, String> {
        let bytes = text.as_bytes();
        if bytes.len() != 64 {
            return Err(format!("expected 64 hex digits, found {}", bytes.len()));
        }
        let mut out = [0u8; 32];
        for (index, chunk) in bytes.chunks_exact(2).enumerate() {
            let value = |b: u8| -> Result<u8, String> {
                match b {
                    b'0'..=b'9' => Ok(b - b'0'),
                    b'a'..=b'f' => Ok(b - b'a' + 10),
                    _ => Err("lowercase hexadecimal is mandatory".to_owned()),
                }
            };
            out[index] = (value(chunk[0])? << 4) | value(chunk[1])?;
        }
        Ok(Self(out))
    }
}

impl std::fmt::Display for Sha256Digest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl serde::Serialize for Sha256Digest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_hex())
    }
}

impl<'de> serde::Deserialize<'de> for Sha256Digest {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::from_hex(&s).map_err(serde::de::Error::custom)
    }
}

/// Where a recipe's bytes go: a SHA-256 state, for the identity, or a
/// buffer, for the canonical bytes the identity is the SHA-256 of
/// (`artifact::address`). One recipe written once over a sink gives both,
/// so the two never differ.
pub trait FrameSink {
    /// Append bytes.
    fn put(&mut self, bytes: &[u8]);
}

impl FrameSink for Sha256 {
    fn put(&mut self, bytes: &[u8]) {
        self.update(bytes);
    }
}

impl FrameSink for Vec<u8> {
    fn put(&mut self, bytes: &[u8]) {
        self.extend_from_slice(bytes);
    }
}

/// One labeled recipe (§21.1): the domain, a NUL, then frames.
pub struct Framed<S: FrameSink> {
    sink: S,
}

impl<S: FrameSink> Framed<S> {
    /// Start a recipe in `sink` with its domain-separation prefix, e.g.
    /// `"lexlean-source-v1\0"`.
    #[must_use]
    pub fn with_sink(mut sink: S, domain: &str) -> Self {
        sink.put(domain.as_bytes());
        sink.put(&[0u8]);
        Self { sink }
    }

    /// Append one `frame(label, bytes)` (§21.1).
    pub fn frame(&mut self, label: &str, bytes: &[u8]) {
        let label_bytes = label.as_bytes();
        self.sink.put(
            &u32::try_from(label_bytes.len())
                .expect("label fits u32")
                .to_be_bytes(),
        );
        self.sink.put(label_bytes);
        self.sink.put(&(bytes.len() as u64).to_be_bytes());
        self.sink.put(bytes);
    }

    /// The sink, holding everything written.
    #[must_use]
    pub fn into_sink(self) -> S {
        self.sink
    }
}

/// An incremental framed hasher over one labeled recipe (§21.1).
pub type FramedHasher = Framed<Sha256>;

impl Framed<Sha256> {
    /// Start a hashed recipe with its domain-separation prefix.
    #[must_use]
    pub fn new(domain: &str) -> Self {
        Self::with_sink(Sha256::new(), domain)
    }

    /// Finish the recipe.
    #[must_use]
    pub fn finish(self) -> Sha256Digest {
        Sha256Digest(self.sink.finalize().into())
    }
}

impl Framed<Vec<u8>> {
    /// Start a recipe whose bytes are kept, with its domain prefix.
    #[must_use]
    pub fn bytes(domain: &str) -> Self {
        Self::with_sink(Vec::new(), domain)
    }
}

/// The recipe of the §11.5 tree digest: one frame per file, labeled by its
/// path, in the order given.
pub fn tree_frames<S: FrameSink>(sink: S, files: &[(&str, &[u8])]) -> S {
    debug_assert!(
        files
            .windows(2)
            .all(|pair| pair[0].0.as_bytes() < pair[1].0.as_bytes()),
        "tree digest input must be strictly sorted by path bytes"
    );
    let mut framed = Framed::with_sink(sink, "lexlean-tree-v1");
    for (path, bytes) in files {
        framed.frame(path, bytes);
    }
    framed.into_sink()
}

/// The §11.5 tree digest over `(relative path, bytes)` rows. The caller
/// supplies rows already restricted to participating files; this function
/// enforces bytewise-sorted path order and unique paths.
#[must_use]
pub fn tree_digest(files: &[(&str, &[u8])]) -> Sha256Digest {
    Sha256Digest(tree_frames(Sha256::new(), files).finalize().into())
}

/// The recipe of the source ID (§21.3).
pub fn source_frames<S: FrameSink>(
    sink: S,
    canonical_project_toml: &[u8],
    canonical_lock: &[u8],
    sources: &[(String, Vec<u8>)],
) -> S {
    let mut framed = Framed::with_sink(sink, "lexlean-source-v1");
    framed.frame("project", canonical_project_toml);
    framed.frame("lock", canonical_lock);
    for (path, bytes) in sources {
        framed.frame("path", path.as_bytes());
        framed.frame("source", bytes);
    }
    framed.into_sink()
}

/// The source ID (§21.3). `sources` are `(project-relative path, normalized
/// bytes)` in sorted path order.
#[must_use]
pub fn source_id(
    canonical_project_toml: &[u8],
    canonical_lock: &[u8],
    sources: &[(String, Vec<u8>)],
) -> Sha256Digest {
    Sha256Digest(
        source_frames(
            Sha256::new(),
            canonical_project_toml,
            canonical_lock,
            sources,
        )
        .finalize()
        .into(),
    )
}

/// The recipe of the semantic ID (§21.4).
pub fn semantic_frames<S: FrameSink>(
    sink: S,
    compiler_semantics: Sha256Digest,
    language: &str,
    linked_ir_json: &str,
    lexicon_closure_json: &str,
) -> S {
    let mut framed = Framed::with_sink(sink, "lexlean-semantic-v1");
    framed.frame("compiler-semantics", &compiler_semantics.0);
    framed.frame("language", language.as_bytes());
    framed.frame("toolchain", crate::LEAN_TOOLCHAIN.as_bytes());
    framed.frame("linked-ir", linked_ir_json.as_bytes());
    framed.frame("lexicon-closure", lexicon_closure_json.as_bytes());
    framed.into_sink()
}

/// The semantic ID (§21.4).
#[must_use]
pub fn semantic_id_for(
    compiler_semantics: Sha256Digest,
    language: &str,
    linked_ir_json: &str,
    lexicon_closure_json: &str,
) -> Sha256Digest {
    Sha256Digest(
        semantic_frames(
            Sha256::new(),
            compiler_semantics,
            language,
            linked_ir_json,
            lexicon_closure_json,
        )
        .finalize()
        .into(),
    )
}

/// The language-1.0 semantic ID recipe retained for API compatibility.
#[must_use]
pub fn semantic_id(
    compiler_semantics: Sha256Digest,
    linked_ir_json: &str,
    lexicon_closure_json: &str,
) -> Sha256Digest {
    semantic_id_for(
        compiler_semantics,
        crate::LANGUAGE_VERSION,
        linked_ir_json,
        lexicon_closure_json,
    )
}

/// The recipe of the build ID (§21.5).
pub fn build_frames<S: FrameSink>(sink: S, source: Sha256Digest, semantic: Sha256Digest) -> S {
    let mut framed = Framed::with_sink(sink, "lexlean-build-v1");
    framed.frame("source-id", &source.0);
    framed.frame("semantic-id", &semantic.0);
    framed.into_sink()
}

/// The build ID (§21.5).
#[must_use]
pub fn build_id(source: Sha256Digest, semantic: Sha256Digest) -> Sha256Digest {
    Sha256Digest(
        build_frames(Sha256::new(), source, semantic)
            .finalize()
            .into(),
    )
}

/// The recipe of the attestation ID (§22.9).
pub fn attestation_frames<S: FrameSink>(sink: S, body_without_id_json: &str) -> S {
    let mut framed = Framed::with_sink(sink, "lexlean-attestation-v1");
    framed.frame("attestation-body", body_without_id_json.as_bytes());
    framed.into_sink()
}

/// The attestation ID over the canonical body without its `attestation_id`
/// field (§22.9).
#[must_use]
pub fn attestation_id(body_without_id_json: &str) -> Sha256Digest {
    Sha256Digest(
        attestation_frames(Sha256::new(), body_without_id_json)
            .finalize()
            .into(),
    )
}

/// The recipe of the PDF recipe ID (§19.8).
pub fn pdf_recipe_frames<S: FrameSink>(
    sink: S,
    tex_sha256: Sha256Digest,
    program_sha256: Sha256Digest,
    version_stdout_sha256: Sha256Digest,
    compile_argv_json: &str,
    resources_json: &str,
) -> S {
    let mut framed = Framed::with_sink(sink, "lexlean-pdf-recipe-v1");
    framed.frame("tex", &tex_sha256.0);
    framed.frame("program", &program_sha256.0);
    framed.frame("version-output", &version_stdout_sha256.0);
    framed.frame("argv", compile_argv_json.as_bytes());
    framed.frame("resources", resources_json.as_bytes());
    framed.into_sink()
}

/// The PDF recipe ID (§19.8).
#[must_use]
pub fn pdf_recipe_id(
    tex_sha256: Sha256Digest,
    program_sha256: Sha256Digest,
    version_stdout_sha256: Sha256Digest,
    compile_argv_json: &str,
    resources_json: &str,
) -> Sha256Digest {
    Sha256Digest(
        pdf_recipe_frames(
            Sha256::new(),
            tex_sha256,
            program_sha256,
            version_stdout_sha256,
            compile_argv_json,
            resources_json,
        )
        .finalize()
        .into(),
    )
}
