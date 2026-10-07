//! The ledger entry (SPEC.md §33.2).
//!
//! An entry is the only thing a third party needs. Everything a verifier
//! checks is recomputable from the entry's own fields: the content digest from
//! its per-source canonical digests, the signature from the digest, the
//! timestamp from its token, and the inclusion from its leaf hash and audit
//! path. Nothing here reads the inventor's filesystem, which is what makes the
//! entry checkable by a court rather than by its author.
//!
//! The digest is hierarchical for exactly that reason. A source's canonical
//! form frames its declaration *bodies*, so the source digest commits to the
//! Lean text. The entry's digest frames the *digests* of its sources and the
//! names of its declarations, so the entry commits to the sources without
//! carrying them. A verifier holding only the entry recomputes the entry
//! digest; a verifier who also downloaded the sources recomputes the source
//! digests and checks that they match the ones the entry commits to.

use crate::artifact::canonical_json::Json;
use crate::artifact::content_id::Sha256Digest;

use super::canonical::{self, CanonicalSource, Declaration};
use super::signature::Signature;
use super::timestamp::Timestamp;
use super::{CANONICALIZATION, CANONICAL_DOMAIN, ENTRY_SPEC};

/// One hashed source and the digest of its §33.1 canonical form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashedSource {
    /// The project-relative path, which is part of the entry's identity.
    pub path: String,
    /// SHA-256 over the source's canonical form.
    pub canonical_digest: Sha256Digest,
    /// The declaration names the source contributes, in canonical order.
    pub declarations: Vec<String>,
}

impl HashedSource {
    fn to_json(&self) -> Json {
        Json::object(vec![
            ("path", Json::Str(self.path.clone())),
            (
                "canonical_digest",
                Json::Str(self.canonical_digest.to_hex()),
            ),
            (
                "declarations",
                Json::Arr(
                    self.declarations
                        .iter()
                        .map(|name| Json::Str(name.clone()))
                        .collect(),
                ),
            ),
        ])
    }
}

/// An entry's place in the RFC 6962 log (SPEC.md §33.5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inclusion {
    /// The appended leaf's zero-based index.
    pub leaf_index: u64,
    /// The tree size after the append.
    pub tree_size: u64,
    /// The audit path from the leaf to the root, leaf-first.
    pub audit_path: Vec<Sha256Digest>,
    /// The root this proof reaches.
    pub root_hash: Sha256Digest,
}

impl Inclusion {
    fn to_json(&self) -> Json {
        Json::object(vec![
            (
                "leaf_index",
                Json::Int(i64::try_from(self.leaf_index).unwrap_or(i64::MAX)),
            ),
            (
                "tree_size",
                Json::Int(i64::try_from(self.tree_size).unwrap_or(i64::MAX)),
            ),
            (
                "audit_path",
                Json::Arr(
                    self.audit_path
                        .iter()
                        .map(|digest| Json::Str(digest.to_hex()))
                        .collect(),
                ),
            ),
            ("root_hash", Json::Str(self.root_hash.to_hex())),
        ])
    }
}

/// One appended record: a canonical digest, its authorship, its external time,
/// and its place in the log.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// Always [`ENTRY_SPEC`]; a document with any other value is not an entry.
    pub spec: String,
    /// The human-readable subject. Not a normative claim.
    pub title: String,
    /// The §33.1 canonicalization identifier.
    pub canonicalization: String,
    /// The hashed sources in ascending path order.
    pub sources: Vec<HashedSource>,
    /// The declaration names of every source, in the ascending order §33.1
    /// fixes.
    pub declarations: Vec<String>,
    /// SHA-256 over the §33.1 entry canonical form.
    pub content_digest: Sha256Digest,
    /// The pinned Lean toolchain the source elaborates under.
    pub toolchain: String,
    /// The detached Ed25519 signature over [`Self::content_digest`].
    pub signature: Signature,
    /// The external time anchor, when one was obtained.
    pub timestamp: Option<Timestamp>,
    /// The §33.5 inclusion proof, filled in when the entry is appended.
    pub inclusion: Option<Inclusion>,
    /// The optional §34 stratification layer.
    ///
    /// Every field of it is derived from the fields already above, so it is
    /// excluded from [`Self::leaf_bytes`] and cannot change a §33 result. See
    /// [`crate::lexeme::pirtm`] for why.
    pub pirtm: Option<crate::lexeme::pirtm::PirtmLayer>,
}

impl Entry {
    /// Build an entry over a project's sources, before it is signed.
    ///
    /// `sources` are `(project-relative path, source text)` pairs. Each is
    /// canonicalized independently, so a source's digest commits to its Lean
    /// text, and the entry's digest commits to those digests and to the
    /// declaration names.
    ///
    /// # Errors
    /// Returns the reason and the offending position when a source declares
    /// two declarations under one name.
    pub fn new(
        title: &str,
        toolchain: &str,
        sources: &[(&str, &str)],
    ) -> Result<Self, (String, String)> {
        let mut hashed: Vec<HashedSource> = Vec::with_capacity(sources.len());
        for (path, text) in sources {
            let canonical = canonical::canonicalize(text)
                .map_err(|(reason, line)| (reason, format!("{path}:{line}")))?;
            hashed.push(HashedSource {
                path: (*path).to_owned(),
                canonical_digest: canonical.content_digest(toolchain),
                declarations: canonical.names(),
            });
        }
        hashed.sort_by(|left, right| left.path.as_bytes().cmp(right.path.as_bytes()));
        let mut declarations: Vec<String> = hashed
            .iter()
            .flat_map(|source| source.declarations.iter().cloned())
            .collect();
        declarations.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
        declarations.dedup();
        for pair in declarations.windows(2) {
            if pair[0] == pair[1] {
                return Err((
                    format!("two sources declare `{}`", pair[0]),
                    "declaration names must be unique across the entry".to_owned(),
                ));
            }
        }
        let content_digest = Self::entry_digest(toolchain, &hashed, &declarations);
        Ok(Self {
            spec: ENTRY_SPEC.to_owned(),
            title: title.to_owned(),
            canonicalization: CANONICALIZATION.to_owned(),
            sources: hashed,
            declarations,
            content_digest,
            toolchain: toolchain.to_owned(),
            signature: Signature::unsigned(),
            timestamp: None,
            inclusion: None,
            pirtm: None,
        })
    }

    /// The §33.1 entry canonical form: the framed byte string, no trailing
    /// newline.
    ///
    /// This is the recipe a verifier applies to the entry's own fields, so it
    /// takes the same three inputs the entry carries and nothing else.
    #[must_use]
    pub fn entry_canonical_form(toolchain: &str, hashed: &[HashedSource]) -> Vec<u8> {
        let mut declarations: Vec<String> = hashed
            .iter()
            .flat_map(|source| source.declarations.iter().cloned())
            .collect();
        declarations.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
        declarations.dedup();
        Self::frames(toolchain, hashed, &declarations).finish()
    }

    /// The digest of the entry canonical form, which is `content_digest`.
    #[must_use]
    pub fn entry_digest(
        toolchain: &str,
        hashed: &[HashedSource],
        declarations: &[String],
    ) -> Sha256Digest {
        Sha256Digest::of(&Self::frames(toolchain, hashed, declarations).finish())
    }

    fn frames(
        toolchain: &str,
        hashed: &[HashedSource],
        declarations: &[String],
    ) -> canonical::FrameWriter {
        let mut frames = canonical::FrameWriter::new(CANONICAL_DOMAIN);
        frames.frame("canonicalization", CANONICALIZATION.as_bytes());
        frames.frame("toolchain", toolchain.as_bytes());
        for source in hashed {
            frames.frame("source-path", source.path.as_bytes());
            frames.frame("source-digest", source.canonical_digest.to_hex().as_bytes());
        }
        for name in declarations {
            frames.frame("declaration-name", name.as_bytes());
        }
        frames
    }

    /// Attach the signature that authenticates the content digest.
    pub fn with_signature(&mut self, signature: Signature) {
        self.signature = signature;
    }

    /// Attach the external time anchor.
    pub fn with_timestamp(&mut self, timestamp: Timestamp) {
        self.timestamp = Some(timestamp);
    }

    /// Attach the inclusion proof produced by appending the entry.
    pub fn with_inclusion(&mut self, inclusion: Inclusion) {
        self.inclusion = Some(inclusion);
    }

    /// Attach the §34 stratification layer.
    pub fn with_pirtm(&mut self, layer: crate::lexeme::pirtm::PirtmLayer) {
        self.pirtm = Some(layer);
    }

    /// The entry's canonical JSON, which is also the leaf bytes of §33.5.
    #[must_use]
    pub fn to_json(&self) -> Json {
        let mut object = vec![
            ("spec", Json::Str(self.spec.clone())),
            ("title", Json::Str(self.title.clone())),
            ("canonicalization", Json::Str(self.canonicalization.clone())),
            (
                "sources",
                Json::Arr(self.sources.iter().map(HashedSource::to_json).collect()),
            ),
            (
                "declarations",
                Json::Arr(
                    self.declarations
                        .iter()
                        .map(|name| Json::Str(name.clone()))
                        .collect(),
                ),
            ),
            ("content_digest", Json::Str(self.content_digest.to_hex())),
            ("toolchain", Json::Str(self.toolchain.clone())),
            ("signature", self.signature.to_json()),
        ];
        if let Some(timestamp) = &self.timestamp {
            object.push(("timestamp", timestamp.to_json()));
        }
        if let Some(inclusion) = &self.inclusion {
            object.push(("inclusion", inclusion.to_json()));
        }
        if let Some(layer) = &self.pirtm {
            object.push(("pirtm", layer.to_json()));
        }
        Json::object(object)
    }

    /// The leaf bytes of §33.5: the canonical JSON of the entry with its
    /// inclusion omitted.
    ///
    /// The inclusion is omitted because it is what the leaf hash is used to
    /// prove: an entry that carried its own proof inside the bytes the proof
    /// commits to would be self-referential.
    ///
    /// The §34 `pirtm` layer is omitted for the same reason and one more. It is
    /// what §34.4 requires: a leaf carrying the layer inside the bytes the §33
    /// checks hash would let attaching the layer change a §33 verdict, which is
    /// exactly the substitution stratification is defined to avoid. Every value
    /// in the layer is derived from the fields that remain, so dropping it loses
    /// no evidence --- a verifier recomputes it rather than reading it.
    ///
    /// Everything else stays, so the leaf is the whole signed record and a
    /// verifier needs nothing else to check steps 1 through 3 of §33.6.
    #[must_use]
    pub fn leaf_bytes(&self) -> Vec<u8> {
        let mut stripped = self.clone();
        stripped.inclusion = None;
        stripped.pirtm = None;
        stripped.leaf_text().into_bytes()
    }

    /// The entry as the canonical JSON text a ledger line stores.
    #[must_use]
    pub fn leaf_text(&self) -> String {
        self.to_json().to_canonical_string()
    }

    /// Decode an entry from canonical JSON, as a dropped file or a stored line.
    ///
    /// # Errors
    /// Returns [`LLG1002`](crate::code) naming the first field that is absent,
    /// of the wrong type, or malformed. A malformed entry is refused rather than
    /// partially read, because every field of the canonical JSON is part of the
    /// leaf hash and a field silently defaulted to something plausible would be
    /// indistinguishable from the value the author wrote.
    ///
    /// Recompute the content digest from the entry's own fields.
    ///
    /// This is §33.2's validity condition: the digest an entry declares must
    /// be the digest of the fields the entry carries. No source file and no
    /// `.olean` is read, which is what lets a browser recompute it.
    ///
    /// # Errors
    /// Returns the reason and the offending field when the entry's spec,
    /// canonicalization, or field ordering is wrong, or when the reconstructed
    /// digest differs from the recorded one.
    pub fn recompute_digest(&self) -> Result<Sha256Digest, (String, String)> {
        if self.spec != ENTRY_SPEC {
            return Err((format!("spec is `{}`", self.spec), ENTRY_SPEC.to_owned()));
        }
        if self.canonicalization != CANONICALIZATION {
            return Err((
                format!("canonicalization is `{}`", self.canonicalization),
                CANONICALIZATION.to_owned(),
            ));
        }
        for pair in self.sources.windows(2) {
            if pair[0].path.as_bytes() >= pair[1].path.as_bytes() {
                return Err((
                    format!(
                        "sources are not in ascending path order at `{}`",
                        pair[1].path
                    ),
                    pair[1].path.clone(),
                ));
            }
        }
        for pair in self.declarations.windows(2) {
            if pair[0].as_bytes() >= pair[1].as_bytes() {
                return Err((
                    format!("declarations are not in ascending order at `{}`", pair[1]),
                    self.declarations.join(", "),
                ));
            }
        }
        let declared: Vec<String> = self
            .sources
            .iter()
            .flat_map(|source| source.declarations.iter().cloned())
            .collect();
        let mut expected = declared.clone();
        expected.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
        expected.dedup();
        if expected != self.declarations {
            return Err((
                "the declaration list is not the deduplicated union of the sources'".to_owned(),
                format!("expected {expected:?}"),
            ));
        }
        let recomputed = Self::entry_digest(&self.toolchain, &self.sources, &self.declarations);
        if recomputed != self.content_digest {
            return Err((
                "the recorded content digest is not the digest of the entry's own fields"
                    .to_owned(),
                recomputed.to_hex(),
            ));
        }
        Ok(recomputed)
    }

    /// change the leaf without changing what a reader was told.
    pub fn from_json(value: &Json) -> Result<Self, crate::error::LexLeanError> {
        let Json::Obj(object) = value else {
            return Err(entry_error("a lexeme entry is a JSON object"));
        };
        let string = |key: &str| -> Result<String, crate::error::LexLeanError> {
            match object.get(key) {
                Some(Json::Str(text)) => Ok(text.clone()),
                Some(_) => Err(entry_error(format!("`{key}` is not a string"))),
                None => Err(entry_error(format!("`{key}` is absent"))),
            }
        };
        let strings = |key: &str| -> Result<Vec<String>, crate::error::LexLeanError> {
            match object.get(key) {
                Some(Json::Arr(items)) => items
                    .iter()
                    .map(|item| match item {
                        Json::Str(text) => Ok(text.clone()),
                        _ => Err(entry_error(format!("`{key}` holds a non-string"))),
                    })
                    .collect(),
                Some(_) => Err(entry_error(format!("`{key}` is not an array"))),
                None => Err(entry_error(format!("`{key}` is absent"))),
            }
        };
        let digest = |key: &str| -> Result<Sha256Digest, crate::error::LexLeanError> {
            Sha256Digest::from_hex(&string(key)?)
                .map_err(|reason| entry_error(format!("`{key}`: {reason}")))
        };
        let mut sources = Vec::new();
        match object.get("sources") {
            Some(Json::Arr(items)) => {
                for item in items {
                    let Json::Obj(source) = item else {
                        return Err(entry_error("a source is not a JSON object"));
                    };
                    let field = |key: &str| -> Result<String, crate::error::LexLeanError> {
                        match source.get(key) {
                            Some(Json::Str(text)) => Ok(text.clone()),
                            _ => Err(entry_error(format!(
                                "a source's `{key}` is absent or not a string"
                            ))),
                        }
                    };
                    let declarations = match source.get("declarations") {
                        Some(Json::Arr(names)) => names
                            .iter()
                            .map(|name| match name {
                                Json::Str(text) => Ok(text.clone()),
                                _ => Err(entry_error("a source declaration name is not a string")),
                            })
                            .collect::<Result<Vec<String>, crate::error::LexLeanError>>()?,
                        _ => return Err(entry_error("a source has no declaration list")),
                    };
                    let canonical_digest =
                        Sha256Digest::from_hex(field("canonical_digest")?.as_str())
                            .map_err(|reason| entry_error(format!("a source digest: {reason}")))?;
                    sources.push(HashedSource {
                        path: field("path")?,
                        canonical_digest,
                        declarations,
                    });
                }
            }
            Some(_) => return Err(entry_error("`sources` is not an array")),
            None => return Err(entry_error("`sources` is absent")),
        }

        let signature = super::signature::Signature::from_json(
            object
                .get("signature")
                .ok_or_else(|| entry_error("`signature` is absent"))?,
        )?;

        let timestamp = match object.get("timestamp") {
            Some(value) => Some(super::timestamp::Timestamp::from_json(value)?),
            None => None,
        };

        let inclusion = match object.get("inclusion") {
            Some(value) => {
                let Json::Obj(inclusion) = value else {
                    return Err(entry_error("an inclusion is not a JSON object"));
                };
                let hex_digest = |key: &str| -> Result<Sha256Digest, crate::error::LexLeanError> {
                    match inclusion.get(key) {
                        Some(Json::Str(text)) => Sha256Digest::from_hex(text)
                            .map_err(|reason| entry_error(format!("inclusion `{key}`: {reason}"))),
                        _ => Err(entry_error(format!("inclusion `{key}` is absent"))),
                    }
                };
                let audit_path = match inclusion.get("audit_path") {
                    Some(Json::Arr(items)) => items
                        .iter()
                        .map(|item| match item {
                            Json::Str(text) => Sha256Digest::from_hex(text).map_err(|reason| {
                                entry_error(format!("an audit path element: {reason}"))
                            }),
                            _ => Err(entry_error("an audit path element is not a string")),
                        })
                        .collect::<Result<Vec<Sha256Digest>, crate::error::LexLeanError>>()?,
                    _ => return Err(entry_error("an inclusion has no audit path")),
                };
                Some(Inclusion {
                    leaf_index: unsigned_at(inclusion, "leaf_index")?,
                    tree_size: unsigned_at(inclusion, "tree_size")?,
                    audit_path,
                    root_hash: hex_digest("root_hash")?,
                })
            }
            None => None,
        };

        Ok(Self {
            spec: string("spec")?,
            title: string("title")?,
            canonicalization: string("canonicalization")?,
            sources,
            declarations: strings("declarations")?,
            content_digest: digest("content_digest")?,
            toolchain: string("toolchain")?,
            signature,
            timestamp,
            inclusion,
            // Propagated rather than re-wrapped in LLG1002: the layer's own
            // refusals already carry the specific code the registry lists for
            // them, and wrapping would replace it with the generic entry code.
            pirtm: match object.get("pirtm") {
                None => None,
                Some(layer) => Some(crate::lexeme::pirtm::PirtmLayer::from_json(layer)?),
            },
        })
    }
}

/// Read a non-negative integer field of an inclusion.
fn unsigned_at(
    object: &std::collections::BTreeMap<String, Json>,
    key: &str,
) -> Result<u64, crate::error::LexLeanError> {
    match object.get(key) {
        Some(Json::Int(value)) => {
            u64::try_from(*value).map_err(|_| entry_error(format!("inclusion `{key}` is negative")))
        }
        _ => Err(entry_error(format!(
            "inclusion `{key}` is absent or not an integer"
        ))),
    }
}

/// A single-diagnostic entry failure.
pub(crate) fn entry_error(reason: impl Into<String>) -> crate::error::LexLeanError {
    use crate::diagnostic::Diagnostic;
    crate::error::LexLeanError::from_diagnostic(Diagnostic::new(
        crate::code!("LLG1002"),
        reason.into(),
    ))
}
/// The digest of one source's canonical form, exposed so the artifact store and
/// the conformance suite hash a source exactly as `Entry::new` did.
pub fn source_digest(toolchain: &str, source: &str) -> Result<Sha256Digest, (String, usize)> {
    let canonical: CanonicalSource = canonical::canonicalize(source)?;
    Ok(canonical.content_digest(toolchain))
}

/// The declaration names one source contributes.
pub fn source_declarations(source: &str) -> Result<Vec<Declaration>, (String, usize)> {
    canonical::canonicalize(source).map(|canonical| canonical.declarations)
}
