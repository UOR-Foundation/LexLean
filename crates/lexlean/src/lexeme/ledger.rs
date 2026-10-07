//! Ledger storage (SPEC.md §33.7).
//!
//! A ledger is a directory of plain text: `log.json` holds one canonical entry
//! per line, `roots.txt` holds one published root per head in order, and
//! `heads.json` holds each head with the consistency proof that carries it
//! forward from its predecessor. Nothing here is a database, because a registry
//! that a court can read with `cat` is worth more than one that needs a driver.
//!
//! An append is atomic in the sense §33.7 requires: the leaf is appended, the
//! root is recorded, and the entry is rewritten with its inclusion proof through
//! a temporary file and a rename, so an interruption leaves the ledger at its
//! previous root rather than at a root that no entry reproduces.

use std::path::{Path, PathBuf};

use crate::artifact::canonical_json::Json;
use crate::artifact::content_id::Sha256Digest;
use crate::diagnostic::Diagnostic;
use crate::error::LexLeanError;

use super::entry::{Entry, Inclusion};
use super::signature::{sign_with_seed, Signature};

/// The file name of the appended entries.
pub const LOG_FILE: &str = "log.json";

/// The file name of the published roots, one per head, in order.
pub const ROOTS_FILE: &str = "roots.txt";

/// The file name of the published tree heads with their proofs.
pub const HEADS_FILE: &str = "heads.json";

/// A failure of the ledger's own storage, named for the code §26 registers.
fn storage_failure(reason: String) -> LexLeanError {
    LexLeanError::from_diagnostic(Diagnostic::new(crate::code!("LLG1009"), reason))
}

/// A ledger directory: the appended entries and the published heads.
#[derive(Debug, Clone)]
pub struct Ledger {
    /// The directory holding the three files of §33.7.
    directory: PathBuf,
    /// One canonical JSON line per appended entry, in index order.
    entries: Vec<String>,
    /// The leaf bytes of each entry, which are the lines with the inclusion
    /// proof omitted. Held beside the lines so that no recomputation has to
    /// re-read and re-parse them.
    leaves: Vec<Vec<u8>>,
    /// One published root per head, in publication order.
    roots: Vec<Sha256Digest>,
    /// One head per publication: its size, root, and the proof from its
    /// predecessor.
    heads: Vec<TreeHead>,
}

/// One published tree head.
///
/// §33.7 requires every published head to be signed by the log key, so a head
/// carries its own [`Signature`] alongside the fields a consistency proof
/// needs. The signature is over the head's own canonical form: the three fields
/// below, framed as §33.7's `heads.json` line. A head anyone may publish is not a
/// a head; the signature says who published it, which is what makes "the root
/// matches published log" a statement about a publisher rather than about a
/// file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeHead {
    /// The tree size this head commits to.
    pub tree_size: u64,
    /// The root of the tree of that size.
    pub root_hash: Sha256Digest,
    /// The consistency proof from the preceding head, empty for the first head
    /// and for a head of the same size.
    pub consistency_proof: Vec<Sha256Digest>,
    /// The detached Ed25519 signature over this head's own fields, by the log
    /// key.
    pub signature: Signature,
}

impl TreeHead {
    /// The canonical form this head's signature covers.
    ///
    /// The three fields that carry the tree, in the order §33.7's `heads.json`
    /// writes them, without the signature itself: a signature that covered its
    /// own value would be self-referential and unverifiable.
    #[must_use]
    pub fn signed_fields(&self) -> Json {
        Json::object(vec![
            (
                "tree_size",
                Json::Int(i64::try_from(self.tree_size).unwrap_or(i64::MAX)),
            ),
            ("root_hash", Json::Str(self.root_hash.to_hex())),
            (
                "consistency_proof",
                Json::Arr(
                    self.consistency_proof
                        .iter()
                        .map(|digest| Json::Str(digest.to_hex()))
                        .collect(),
                ),
            ),
        ])
    }

    /// The bytes this head's signature is computed over.
    #[must_use]
    pub fn signed_bytes(&self) -> Vec<u8> {
        self.signed_fields().to_canonical_string().into_bytes()
    }

    fn to_json(&self) -> Json {
        let object = vec![
            (
                "tree_size",
                Json::Int(i64::try_from(self.tree_size).unwrap_or(i64::MAX)),
            ),
            ("root_hash", Json::Str(self.root_hash.to_hex())),
            (
                "consistency_proof",
                Json::Arr(
                    self.consistency_proof
                        .iter()
                        .map(|digest| Json::Str(digest.to_hex()))
                        .collect(),
                ),
            ),
            ("signature", self.signature.to_json()),
        ];
        Json::object(object)
    }
}

impl Ledger {
    /// Create an empty ledger in `directory`, or open the one already there.
    ///
    /// # Errors
    /// Returns [`LLG1009`](crate::code) when the directory cannot be created or
    /// read, or when an existing ledger's stored root does not equal the root
    /// recomputed from its own entries: a ledger whose head disagrees with its
    /// contents is refused rather than loaded, because every later check would
    /// be measuring the wrong tree.
    pub fn open(directory: &Path) -> Result<Self, LexLeanError> {
        std::fs::create_dir_all(directory)
            .map_err(|error| storage_failure(format!("{}: {error}", directory.display())))?;
        let entries = read_lines(&directory.join(LOG_FILE))?;
        let roots = read_lines(&directory.join(ROOTS_FILE))?
            .into_iter()
            .map(|line| {
                Sha256Digest::from_hex(&line)
                    .map_err(|reason| storage_failure(format!("{ROOTS_FILE}: {reason}")))
            })
            .collect::<Result<Vec<Sha256Digest>, LexLeanError>>()?;
        let heads = read_heads(&directory.join(HEADS_FILE))?;
        let leaves = leaf_bytes_of(&entries)?;
        let ledger = Self {
            directory: directory.to_path_buf(),
            entries,
            leaves,
            roots,
            heads,
        };
        ledger.check_stored_head()?;
        Ok(ledger)
    }

    /// The recomputed root of the entries stored here.
    #[must_use]
    pub fn root(&self) -> Option<Sha256Digest> {
        self.leaf_bytes()
            .as_deref()
            .and_then(super::merkle::root_of)
    }

    /// The leaf bytes of every stored entry, in index order.
    #[must_use]
    pub fn leaf_bytes(&self) -> Option<Vec<Vec<u8>>> {
        if self.leaves.is_empty() {
            return None;
        }
        Some(self.leaves.clone())
    }

    /// The number of appended entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the ledger holds no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// The stored entry lines, in index order.
    #[must_use]
    pub fn entry_lines(&self) -> &[String] {
        &self.entries
    }

    /// The published heads, in publication order.
    #[must_use]
    pub fn heads(&self) -> &[TreeHead] {
        &self.heads
    }

    /// The leaf bytes at `index`.
    #[must_use]
    pub fn leaf_at(&self, index: usize) -> Option<&[u8]> {
        self.leaves.get(index).map(Vec::as_slice)
    }

    /// Append a signed entry, publish a head, and return the entry with its
    /// inclusion proof attached.
    ///
    /// The entry is written twice: once as the leaf, and once rewritten with the
    /// inclusion proof, which the leaf therefore does not contain. That is not a
    /// circularity but a consequence of the leaf being the *signed* record: the
    /// signature covers the content digest, which covers the sources, and the
    /// inclusion proof covers the leaf bytes. A verifier therefore checks the
    /// proof against the entry's fields with the inclusion omitted, which is
    /// what [`Entry::without_inclusion`] produces.
    ///
    /// The head this append publishes is signed by the log key over its own
    /// fields, so a reader can tell who published it (§33.7).
    ///
    /// # Errors
    /// Returns [`LLG1002`](crate::code) when the entry is unsigned or its digest
    /// does not follow from its fields, [`LLG1003`](crate::code) when the log
    /// key is not a 32-byte Ed25519 seed, and [`LLG1009`](crate::code) when the
    /// directory cannot be written.
    pub fn append(&mut self, mut entry: Entry, log_key: &[u8]) -> Result<Entry, LexLeanError> {
        entry.recompute_digest().map_err(|(reason, detail)| {
            crate::error::LexLeanError::from_diagnostic(Diagnostic::new(
                crate::code!("LLG1002"),
                format!("{reason}: {detail}"),
            ))
        })?;
        if !entry.signature.well_formed() {
            return Err(LexLeanError::from_diagnostic(Diagnostic::new(
                crate::code!("LLG1002"),
                "an unsigned entry cannot be appended",
            )));
        }
        let leaf = entry.leaf_bytes();
        let mut leaves = self.leaves.clone();
        let inclusion = super::merkle::append(&mut leaves, leaf);
        entry.with_inclusion(inclusion.clone());
        let line = entry.leaf_text();

        let old_size = leaves.len() as u64 - 1;
        let new_size = inclusion.tree_size;
        // The first append has no predecessor, so it publishes a head with an
        // empty proof: there is no earlier root for it to carry forward.
        let proof = if old_size == 0 {
            Vec::new()
        } else {
            super::merkle::consistency_path_from(&leaves, old_size)
        };
        let mut head = TreeHead {
            tree_size: new_size,
            root_hash: inclusion.root_hash,
            consistency_proof: proof,
            signature: Signature::unsigned(),
        };
        let digest = Sha256Digest::of(&head.signed_bytes());
        head.signature = sign_with_seed(&digest.0, log_key).map_err(|error| {
            LexLeanError::from_diagnostic(Diagnostic::new(
                crate::code!("LLG1009"),
                format!("the head at size {new_size} could not be signed: {error}"),
            ))
        })?;

        let mut entries = self.entries.clone();
        entries.push(line);
        let stored_leaves = leaves;
        let mut roots = self.roots.clone();
        roots.push(head.root_hash);
        let mut heads = self.heads.clone();
        heads.push(head);
        write_atomic(&self.directory.join(LOG_FILE), &joined(&entries))?;
        write_atomic(
            &self.directory.join(ROOTS_FILE),
            &joined(
                &roots
                    .iter()
                    .map(Sha256Digest::to_hex)
                    .collect::<Vec<String>>(),
            ),
        )?;
        write_atomic(
            &self.directory.join(HEADS_FILE),
            &joined(&heads_json(&heads)),
        )?;
        self.entries = entries;
        self.leaves = stored_leaves;
        self.roots = roots;
        self.heads = heads;
        Ok(entry)
    }

    /// Check that the stored head of every prefix equals the root recomputed
    /// from that prefix's own entries (§33.5), and that every head is signed by
    /// the log key (§33.7).
    ///
    /// A stored line is the whole entry, inclusion proof included; the leaf it
    /// commits to is the entry with the proof omitted, which is what
    /// [`Entry::leaf_bytes`] returns. Recomputing from the raw line instead
    /// would compare two different trees and refuse every healthy ledger.
    ///
    /// # Errors
    /// Returns [`LLG1009`](crate::code) naming the first head whose signature
    /// does not verify over its own fields, or the first head whose stored root
    /// disagrees with its prefix, or the first line that is not a readable
    /// entry.
    pub fn check_stored_head(&self) -> Result<(), LexLeanError> {
        for head in &self.heads {
            let digest = Sha256Digest::of(&head.signed_bytes());
            if !head.signature.verify(&digest.0) {
                return Err(storage_failure(format!(
                    "the head at size {} is not signed by the log key: its signature does not verify over its own fields",
                    head.tree_size
                )));
            }
            let width = usize::try_from(head.tree_size).unwrap_or(usize::MAX);
            if width > self.entries.len() {
                return Err(storage_failure(format!(
                    "the head claims {} leaves and the log holds {}",
                    head.tree_size,
                    self.entries.len()
                )));
            }
            let prefix: Vec<Vec<u8>> = self.leaves.iter().take(width).cloned().collect();
            match super::merkle::root_of(&prefix) {
                Some(root) if root == head.root_hash => {}
                Some(root) => {
                    return Err(storage_failure(format!(
                        "the head at size {} records {} and its entries recompute to {}",
                        head.tree_size, head.root_hash, root
                    )))
                }
                None => {
                    return Err(storage_failure(format!(
                        "the head claims {} leaves and has no root",
                        head.tree_size
                    )))
                }
            }
        }
        Ok(())
    }
}

/// The leaf bytes of a list of stored entry lines.
///
/// A stored line carries the entry whole, inclusion proof included; the leaf is
/// the entry with that proof omitted, which is the byte string the leaf hash of
/// §33.5 covers and the byte string a stored root must recompute from.
fn leaf_bytes_of(lines: &[String]) -> Result<Vec<Vec<u8>>, LexLeanError> {
    lines
        .iter()
        .map(|line| {
            let value = Json::parse(line.as_bytes())
                .map_err(|reason| storage_failure(format!("{LOG_FILE}: {reason}")))?;
            let entry = Entry::from_json(&value).map_err(|error| {
                storage_failure(format!(
                    "{LOG_FILE}: {}",
                    error.diagnostics[0].message.clone()
                ))
            })?;
            Ok(entry.leaf_bytes())
        })
        .collect()
}

/// The entries' lines as one file body.
fn joined(lines: &[String]) -> Vec<u8> {
    let mut out = Vec::new();
    for line in lines {
        out.extend_from_slice(line.as_bytes());
        out.push(b'\n');
    }
    out
}

/// The heads as JSON lines.
fn heads_json(heads: &[TreeHead]) -> Vec<String> {
    heads
        .iter()
        .map(|head| head.to_json().to_canonical_string())
        .collect()
}

/// Read a file as its lines, tolerating an absent file as an empty one.
fn read_lines(path: &Path) -> Result<Vec<String>, LexLeanError> {
    match std::fs::read_to_string(path) {
        Ok(text) => Ok(text
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(str::to_owned)
            .collect()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(error) => Err(storage_failure(format!("{}: {error}", path.display()))),
    }
}

/// Read a heads file.
fn read_heads(path: &Path) -> Result<Vec<TreeHead>, LexLeanError> {
    read_lines(path)?
        .into_iter()
        .map(|line| {
            let value = Json::parse(line.as_bytes())
                .map_err(|reason| storage_failure(format!("{}: {reason}", path.display())))?;
            let Json::Obj(object) = value else {
                return Err(storage_failure(format!(
                    "{}: a head is not an object",
                    path.display()
                )));
            };
            let size = object
                .get("tree_size")
                .and_then(|value| match value {
                    Json::Int(size) => u64::try_from(*size).ok(),
                    _ => None,
                })
                .ok_or_else(|| {
                    storage_failure(format!("{}: a head has no tree_size", path.display()))
                })?;
            let root = object
                .get("root_hash")
                .and_then(|value| match value {
                    Json::Str(text) => Sha256Digest::from_hex(text).ok(),
                    _ => None,
                })
                .ok_or_else(|| {
                    storage_failure(format!("{}: a head has no root_hash", path.display()))
                })?;
            let proof = match object.get("consistency_proof") {
                Some(Json::Arr(items)) => items
                    .iter()
                    .map(|item| match item {
                        Json::Str(text) => Sha256Digest::from_hex(text).map_err(|reason| {
                            storage_failure(format!("{}: {reason}", path.display()))
                        }),
                        _ => Err(storage_failure(format!(
                            "{}: a consistency proof is not hex",
                            path.display()
                        ))),
                    })
                    .collect::<Result<Vec<Sha256Digest>, LexLeanError>>()?,
                _ => Vec::new(),
            };
            let signature = match object.get("signature") {
                Some(value) => super::signature::Signature::from_json(value).map_err(|error| {
                    storage_failure(format!(
                        "{}: {}",
                        path.display(),
                        error.diagnostics[0].message.clone()
                    ))
                })?,
                None => {
                    return Err(storage_failure(format!(
                        "{}: a head has no signature",
                        path.display()
                    )))
                }
            };
            Ok(TreeHead {
                tree_size: size,
                root_hash: root,
                consistency_proof: proof,
                signature,
            })
        })
        .collect()
}

/// Write a file through a temporary sibling and a rename.
///
/// The rename is what makes the append atomic: a reader either sees the whole
/// previous file or the whole new one, never a partial line, which matters
/// because an entry's leaf bytes are the exact bytes of its line.
fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), LexLeanError> {
    let temporary = path.with_extension("tmp");
    std::fs::write(&temporary, bytes)
        .map_err(|error| storage_failure(format!("{}: {error}", temporary.display())))?;
    std::fs::rename(&temporary, path)
        .map_err(|error| storage_failure(format!("{}: {error}", path.display())))
}

/// An entry's fields without its inclusion proof, which is the byte string the
/// leaf hash covers.
///
/// # Errors
/// Returns [`LLG1002`](crate::code) when the entry's digest does not follow from
/// its own fields, because a proof over bytes that do not reconstruct the entry
/// would be checking nothing.
pub fn without_inclusion(entry: &Entry) -> Result<Vec<u8>, LexLeanError> {
    entry.recompute_digest().map_err(|(reason, detail)| {
        LexLeanError::from_diagnostic(Diagnostic::new(
            crate::code!("LLG1002"),
            format!("{reason}: {detail}"),
        ))
    })?;
    let mut stripped = entry.clone();
    stripped.inclusion = None;
    Ok(stripped.leaf_bytes())
}

/// The leaf bytes an inclusion proof is checked against.
///
/// # Errors
/// As [`without_inclusion`].
pub fn proof_subject(entry: &Entry) -> Result<Vec<u8>, LexLeanError> {
    without_inclusion(entry)
}

/// An inclusion proof read from an entry, or the reason it is absent.
#[must_use]
pub fn inclusion_of(entry: &Entry) -> Option<&Inclusion> {
    entry.inclusion.as_ref()
}
