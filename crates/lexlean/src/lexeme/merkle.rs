//! The append-only transparency log (SPEC.md §33.5).
//!
//! This is RFC 6962's Merkle Tree Hash, unmodified: a leaf is
//! `SHA-256(0x00 || leaf_bytes)` and an internal node is
//! `SHA-256(0x01 || left || right)`. The two domain separators are the reason a
//! leaf digest can never be read as a node digest, which is what stops an
//! attacker from presenting an internal node as a leaf. The tree is the
//! unbalanced shape RFC 6962 §2.1 defines, so a proof for index `i` in a tree
//! of `n` leaves is the same proof whether or not `n` is a power of two.
//!
//! The construction is written here rather than taken from a crate because it
//! has to agree, byte for byte, with the Lean specification of §33.9 and with
//! the browser verifier of §33.8. Three implementations of one hash tree would
//! be three chances to disagree; one implementation, checked against the
//! published RFC 6962 reference vectors, is the alternative. Those vectors are
//! the `known_answer` tests at the bottom of this file.

use crate::artifact::content_id::Sha256Digest;

use super::entry::Inclusion;

/// The leaf domain separator of RFC 6962 §2.1.
pub const LEAF_PREFIX: u8 = 0x00;

/// The internal node domain separator of RFC 6962 §2.1.
pub const NODE_PREFIX: u8 = 0x01;

/// The leaf hash, `SHA-256(0x00 || leaf_bytes)`.
#[must_use]
pub fn leaf_hash(leaf_bytes: &[u8]) -> Sha256Digest {
    let mut framed = Vec::with_capacity(leaf_bytes.len() + 1);
    framed.push(LEAF_PREFIX);
    framed.extend_from_slice(leaf_bytes);
    Sha256Digest::of(&framed)
}

/// The internal node hash, `SHA-256(0x01 || left || right)`.
#[must_use]
pub fn node_hash(left: &Sha256Digest, right: &Sha256Digest) -> Sha256Digest {
    let mut framed = Vec::with_capacity(65);
    framed.push(NODE_PREFIX);
    framed.extend_from_slice(&left.0);
    framed.extend_from_slice(&right.0);
    Sha256Digest::of(&framed)
}

/// The largest power of two strictly less than `n`, for `n >= 2`.
///
/// This is the split point of RFC 6962 §2.1's unbalanced tree, and every
/// recursive step of both proofs is defined in terms of it.
#[must_use]
pub fn split_point(n: u64) -> u64 {
    debug_assert!(n >= 2, "the split point of a one-leaf tree is undefined");
    let mut k = 1u64;
    while k << 1 < n {
        k <<= 1;
    }
    k
}

/// The Merkle Tree Hash of a non-empty list of leaves (RFC 6962 §2.1).
#[must_use]
pub fn root_of_leaves(leaves: &[Sha256Digest]) -> Sha256Digest {
    assert!(!leaves.is_empty(), "the empty tree has no root (§33.5)");
    if leaves.len() == 1 {
        return leaves[0];
    }
    let count = u64::try_from(leaves.len()).expect("a leaf count fits u64");
    let split = usize::try_from(split_point(count)).expect("a split index fits usize");
    node_hash(
        &root_of_leaves(&leaves[..split]),
        &root_of_leaves(&leaves[split..]),
    )
}

/// The recursive form of RFC 6962 §2.1.1's `PATH`, reading the tree's own
/// roots, so that a proof is built from the same nodes a verifier recombines.
/// Append the sibling digests of `index` to `out`, leaf-first.
fn path_into(index: u64, tree_size: u64, leaves: &[Sha256Digest], out: &mut Vec<Sha256Digest>) {
    if tree_size <= 1 {
        return;
    }
    let split = split_point(tree_size);
    let width = usize::try_from(split).expect("a split index fits usize");
    if index < split {
        path_into(index, split, &leaves[..width], out);
        out.push(root_of_leaves(&leaves[width..]));
    } else {
        path_into(index - split, tree_size - split, &leaves[width..], out);
        out.push(root_of_leaves(&leaves[..width]));
    }
}

/// The consistency proof from `old_size` leaves to `tree_size` leaves
/// (RFC 6962 §2.1.2).
#[must_use]
pub fn consistency_path(old_size: u64, leaves: &[Sha256Digest]) -> Vec<Sha256Digest> {
    assert!(
        old_size > 0,
        "a consistency proof cannot start from an empty tree"
    );
    assert!(
        old_size <= u64::try_from(leaves.len()).expect("a leaf count fits u64"),
        "the older head cannot exceed the newer one"
    );
    let mut out = Vec::new();
    subproof_into(
        old_size,
        u64::try_from(leaves.len()).expect("a leaf count fits u64"),
        leaves,
        true,
        &mut out,
    );
    out
}

/// The recursive form of RFC 9162 §2.1.4.1's `SUBPROOF`.
///
/// `anchored` records whether the subtree being built is the older tree itself.
/// It is what makes the proof minimal in the one case that matters: when the
/// recursion has only ever taken the left branch, it ends on the older tree,
/// whose root the verifier already holds, so the base case contributes no
/// node. After a right branch the base subtree is an interior one, and the
/// verifier cannot know its root, so the base case contributes one.
fn subproof_into(
    m: u64,
    n: u64,
    leaves: &[Sha256Digest],
    anchored: bool,
    out: &mut Vec<Sha256Digest>,
) {
    if m == n {
        if !anchored {
            out.push(root_of_leaves(leaves));
        }
        return;
    }
    let split = split_point(n);
    let width = usize::try_from(split).expect("a split index fits usize");
    if m <= split {
        subproof_into(m, split, &leaves[..width], anchored, out);
        out.push(root_of_leaves(&leaves[width..]));
    } else {
        subproof_into(m - split, n - split, &leaves[width..], false, out);
        out.push(root_of_leaves(&leaves[..width]));
    }
}

/// Why an inclusion proof was refused; §33.5 names each case separately.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InclusionFailure {
    /// The audit path is not the length this tree size requires, which is what
    /// a truncated, extended, or reordered path looks like.
    MalformedPath,
    /// The leaf index is not below the tree size.
    IndexOutOfRange,
    /// The published root belongs to an empty tree, which has no leaves.
    EmptyTree,
    /// The path recombines to a different root.
    RootMismatch,
}

impl InclusionFailure {
    /// The verdict wording for this refusal.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MalformedPath => "the audit path is not the length this tree size requires",
            Self::IndexOutOfRange => "the leaf index is not below the tree size",
            Self::EmptyTree => "the published root belongs to an empty tree",
            Self::RootMismatch => "the audit path recombines to a different root",
        }
    }
}

/// Recompute the root from a leaf hash and its audit path
/// (RFC 6962 §2.1.1).
fn recompute(
    leaf: Sha256Digest,
    index: u64,
    tree_size: u64,
    path: &[Sha256Digest],
) -> Option<Sha256Digest> {
    if tree_size <= 1 {
        return if path.is_empty() { Some(leaf) } else { None };
    }
    let sibling = *path.last()?;
    let rest = &path[..path.len() - 1];
    let split = split_point(tree_size);
    if index < split {
        let left = recompute(leaf, index, split, rest)?;
        Some(node_hash(&left, &sibling))
    } else {
        let right = recompute(leaf, index - split, tree_size - split, rest)?;
        Some(node_hash(&sibling, &right))
    }
}

/// The audit path length a tree of `tree_size` leaves needs for `index`.
#[must_use]
pub fn audit_path_length(index: u64, tree_size: u64) -> usize {
    if tree_size <= 1 {
        return 0;
    }
    let split = split_point(tree_size);
    if index < split {
        1 + audit_path_length(index, split)
    } else {
        1 + audit_path_length(index - split, tree_size - split)
    }
}

/// Check that `leaf_bytes` sits at `inclusion.leaf_index` under the published
/// root (SPEC.md §33.5).
///
/// The proof comes from the entry's own audit path, so this reads nothing but
/// the entry: a verifier needs the log's shape, never its contents.
pub fn verify_inclusion(leaf_bytes: &[u8], inclusion: &Inclusion) -> Result<(), InclusionFailure> {
    if inclusion.tree_size == 0 {
        return Err(InclusionFailure::EmptyTree);
    }
    if inclusion.leaf_index >= inclusion.tree_size {
        return Err(InclusionFailure::IndexOutOfRange);
    }
    if inclusion.audit_path.len() != audit_path_length(inclusion.leaf_index, inclusion.tree_size) {
        return Err(InclusionFailure::MalformedPath);
    }
    match recompute(
        leaf_hash(leaf_bytes),
        inclusion.leaf_index,
        inclusion.tree_size,
        &inclusion.audit_path,
    ) {
        Some(root) if root == inclusion.root_hash => Ok(()),
        Some(_) => Err(InclusionFailure::RootMismatch),
        None => Err(InclusionFailure::MalformedPath),
    }
}

/// Why a consistency proof was refused (§33.5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsistencyFailure {
    /// The older head claims more leaves than the newer one.
    OldTreeLarger,
    /// The older head claims zero leaves, which admits no proof.
    OldTreeEmpty,
    /// The proof is not the length these two sizes require.
    MalformedPath,
    /// The proof does not carry the older root to the newer one.
    RootMismatch,
}

impl ConsistencyFailure {
    /// The verdict wording for this refusal.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OldTreeLarger => "the older head claims more leaves than the newer one",
            Self::OldTreeEmpty => "the older head claims zero leaves, which admits no proof",
            Self::MalformedPath => "the consistency proof is not the length these sizes require",
            Self::RootMismatch => "the consistency proof does not recompute the newer root",
        }
    }
}

/// The consistency proof length from `old_size` to `new_size` leaves.
#[must_use]
pub fn consistency_path_length(old_size: u64, new_size: u64) -> usize {
    anchored_path_length(old_size, new_size, true)
}

/// The proof length of one recursion level, mirroring [`subproof_into`].
fn anchored_path_length(m: u64, n: u64, anchored: bool) -> usize {
    if m == n {
        return if anchored { 0 } else { 1 };
    }
    let split = split_point(n);
    if m <= split {
        1 + anchored_path_length(m, split, anchored)
    } else {
        1 + anchored_path_length(m - split, n - split, false)
    }
}

/// The recursive check of RFC 9162 §2.1.4.2, returning the root the proof
/// carries.
///
/// `anchored` mirrors [`subproof_into`]: when it holds, the subtree being
/// reached is the older tree itself, whose root is `old_root`, and the proof
/// contributes nothing at that level. When it does not, the last path element
/// is that subtree's root, because the verifier has no other way to know it.
fn check_consistency(
    m: u64,
    n: u64,
    path: &[Sha256Digest],
    anchored: bool,
    old_root: Sha256Digest,
) -> Option<Sha256Digest> {
    if m == n {
        return if anchored {
            Some(old_root)
        } else {
            path.last().copied()
        };
    }
    let (sibling, rest) = path.split_last()?;
    let split = split_point(n);
    if m <= split {
        let left = check_consistency(m, split, rest, anchored, old_root)?;
        Some(node_hash(&left, sibling))
    } else {
        let right = check_consistency(m - split, n - split, rest, false, old_root)?;
        Some(node_hash(sibling, &right))
    }
}

/// Check that two published heads describe one append-only history
/// (SPEC.md §33.5).
///
/// Two heads of the same size are consistent exactly when their roots are
/// equal, which is the case a forked log presents: two different trees of the
/// same length both claiming to be the log.
pub fn verify_consistency(
    old_root: &Sha256Digest,
    old_size: u64,
    new_root: &Sha256Digest,
    new_size: u64,
    proof: &[Sha256Digest],
) -> Result<(), ConsistencyFailure> {
    if old_size > new_size {
        return Err(ConsistencyFailure::OldTreeLarger);
    }
    if old_size == 0 {
        return Err(ConsistencyFailure::OldTreeEmpty);
    }
    if old_size == new_size {
        return if old_root == new_root {
            Ok(())
        } else {
            Err(ConsistencyFailure::RootMismatch)
        };
    }
    if proof.len() != consistency_path_length(old_size, new_size) {
        return Err(ConsistencyFailure::MalformedPath);
    }
    match check_consistency(old_size, new_size, proof, true, *old_root) {
        Some(root) if root == *new_root => Ok(()),
        Some(_) => Err(ConsistencyFailure::RootMismatch),
        None => Err(ConsistencyFailure::MalformedPath),
    }
}

/// Append one entry's leaf bytes to a list of leaf byte strings and return the
/// resulting inclusion proof.
#[must_use]
pub fn append(leaves: &mut Vec<Vec<u8>>, leaf_bytes: Vec<u8>) -> Inclusion {
    leaves.push(leaf_bytes);
    inclusion_at(leaves, leaves.len() as u64 - 1)
}

/// The consistency proof from the first `old_size` leaves of `leaf_bytes` to
/// the whole list.
///
/// This is the byte-level entry point the ledger appender uses, so that the
/// proof a head publishes is built from exactly the leaves a verifier
/// re-reads.
///
/// # Panics
/// Panics when `old_size` is zero or exceeds the leaf count, both of which
/// describe no append-only history at all.
#[must_use]
pub fn consistency_path_from(leaf_bytes: &[Vec<u8>], old_size: u64) -> Vec<Sha256Digest> {
    let digests: Vec<Sha256Digest> = leaf_bytes.iter().map(|leaf| leaf_hash(leaf)).collect();
    consistency_path(old_size, &digests)
}

/// The inclusion proof of the leaf at `index` in `leaves`.
#[must_use]
pub fn inclusion_at(leaves: &[Vec<u8>], index: u64) -> Inclusion {
    let digests: Vec<Sha256Digest> = leaves.iter().map(|leaf| leaf_hash(leaf)).collect();
    let tree_size = u64::try_from(digests.len()).expect("a leaf count fits u64");
    let mut audit_path = Vec::new();
    path_into(index, tree_size, &digests, &mut audit_path);
    Inclusion {
        leaf_index: index,
        tree_size,
        audit_path,
        root_hash: root_of_leaves(&digests),
    }
}

/// The root of a non-empty list of leaf byte strings, or `None` for the empty
/// one: §33.5 gives an empty tree no root rather than a root of its own.
#[must_use]
pub fn root_of(leaves: &[Vec<u8>]) -> Option<Sha256Digest> {
    if leaves.is_empty() {
        return None;
    }
    let digests: Vec<Sha256Digest> = leaves.iter().map(|leaf| leaf_hash(leaf)).collect();
    Some(root_of_leaves(&digests))
}

#[cfg(test)]
mod tests {
    use super::{
        audit_path_length, consistency_path, leaf_hash, root_of_leaves, verify_consistency,
        verify_inclusion,
    };

    /// The reference leaves of the transparency-dev RFC 6962 vectors.
    const LEAVES: [&str; 8] = [
        "",
        "00",
        "10",
        "2021",
        "3031",
        "40414243",
        "5051525354555657",
        "606162636465666768696a6b6c6d6e6f",
    ];

    fn leaves(count: usize) -> Vec<Vec<u8>> {
        LEAVES[..count]
            .iter()
            .map(|leaf| crate::lexeme::signature::hex_decode(leaf).expect("a hex leaf"))
            .collect()
    }

    fn digests(count: usize) -> Vec<crate::artifact::content_id::Sha256Digest> {
        leaves(count).iter().map(|leaf| leaf_hash(leaf)).collect()
    }

    fn path_hex(path: &[crate::artifact::content_id::Sha256Digest]) -> String {
        path.iter()
            .map(crate::artifact::content_id::Sha256Digest::to_hex)
            .collect::<Vec<String>>()
            .join("")
    }

    /// RFC 6962 §2.1 reference audit paths, from the transparency-dev vectors.
    #[test]
    fn inclusion_known_answers() {
        let cases: [(usize, usize, &str); 7] = [
            (1, 0, ""),
            (
                2,
                0,
                "96a296d224f285c67bee93c30f8a309157f0daa35dc5b87e410b78630a09cfc7",
            ),
            (
                2,
                1,
                "6e340b9cffb37a989ca544e6bb780a2c78901d3fb33738768511a30617afa01d",
            ),
            (
                3,
                2,
                "fac54203e7cc696cf0dfcb42c92a1d9dbaf70ad9e621f4bd8d98662f00e3c125",
            ),
            (
                5,
                1,
                "6e340b9cffb37a989ca544e6bb780a2c78901d3fb33738768511a30617afa01d\
                 5f083f0a1a33ca076a95279832580db3e0ef4584bdff1f54c8a360f50de3031e\
                 bc1a0643b12e4d2d7c77918f44e0f4f79a838b6cf9ec5b5c283e1f4d88599e6b",
            ),
            (
                8,
                0,
                "96a296d224f285c67bee93c30f8a309157f0daa35dc5b87e410b78630a09cfc7\
                 5f083f0a1a33ca076a95279832580db3e0ef4584bdff1f54c8a360f50de3031e\
                 6b47aaf29ee3c2af9af889bc1fb9254dabd31177f16232dd6aab035ca39bf6e4",
            ),
            (
                8,
                5,
                "bc1a0643b12e4d2d7c77918f44e0f4f79a838b6cf9ec5b5c283e1f4d88599e6b\
                 ca854ea128ed050b41b35ffc1b87b8eb2bde461e9e3b5596ece6b9d5975a0ae0\
                 d37ee418976dd95753c1c73862b9398fa2a2cf9b4ff0fdfe8b30cd95209614b7",
            ),
        ];
        for (size, index, expected) in cases {
            let built = audit_path_at(size, index);
            assert_eq!(
                path_hex(&built),
                expected.replace(['\n', ' '], ""),
                "audit path for index {index} of {size}"
            );
            assert_eq!(built.len(), audit_path_length(index as u64, size as u64));
        }
    }

    /// The audit path of `index` in the first `size` reference leaves.
    fn audit_path_at(size: usize, index: usize) -> Vec<crate::artifact::content_id::Sha256Digest> {
        let all = digests(size);
        let mut out = Vec::new();
        super::path_into(index as u64, size as u64, &all, &mut out);
        out
    }

    /// RFC 9162 §2.1.4 reference consistency proofs.
    #[test]
    fn consistency_known_answers() {
        let cases: [(u64, u64, &str); 4] = [
            (1, 1, ""),
            (
                1,
                8,
                "96a296d224f285c67bee93c30f8a309157f0daa35dc5b87e410b78630a09cfc7\
                 5f083f0a1a33ca076a95279832580db3e0ef4584bdff1f54c8a360f50de3031e\
                 6b47aaf29ee3c2af9af889bc1fb9254dabd31177f16232dd6aab035ca39bf6e4",
            ),
            (
                2,
                5,
                "5f083f0a1a33ca076a95279832580db3e0ef4584bdff1f54c8a360f50de3031e\
                 bc1a0643b12e4d2d7c77918f44e0f4f79a838b6cf9ec5b5c283e1f4d88599e6b",
            ),
            (
                6,
                8,
                "0ebc5d3437fbe2db158b9f126a1d118e308181031d0a949f8dededebc558ef6a\
                 ca854ea128ed050b41b35ffc1b87b8eb2bde461e9e3b5596ece6b9d5975a0ae0\
                 d37ee418976dd95753c1c73862b9398fa2a2cf9b4ff0fdfe8b30cd95209614b7",
            ),
        ];
        for (old, new, expected) in cases {
            let built = consistency_path(old, &digests(new as usize));
            assert_eq!(
                path_hex(&built),
                expected.replace(['\n', ' '], ""),
                "consistency proof from {old} to {new}"
            );
        }
    }

    /// Every reference proof verifies against its own root.
    #[test]
    fn reference_proofs_verify() {
        for size in 1..=8usize {
            let all = leaves(size);
            let root = root_of_leaves(&digests(size));
            for index in 0..size {
                let inclusion = super::inclusion_at(&all, index as u64);
                assert!(verify_inclusion(&all[index], &inclusion).is_ok());
                assert_eq!(inclusion.root_hash, root);
            }
            for old in 1..=size as u64 {
                let proof = consistency_path(old, &digests(size));
                let old_root = root_of_leaves(&digests(old as usize));
                assert!(
                    verify_consistency(&old_root, old, &root, size as u64, &proof).is_ok(),
                    "heads {old} and {size} should be consistent"
                );
            }
        }
    }

    /// The domain separation of §33.5: a leaf digest is never a node digest.
    #[test]
    fn leaf_and_node_domains_are_disjoint() {
        let leaf = leaf_hash(b"payload");
        let node = super::node_hash(&leaf, &leaf);
        assert_ne!(leaf, node);
        assert_ne!(leaf, leaf_hash(b""));
    }

    /// A wrong-length, reordered, or out-of-range path is refused.
    #[test]
    fn tampered_inclusion_is_refused() {
        let all = leaves(8);
        let inclusion = super::inclusion_at(&all, 5);
        let mut short = inclusion.clone();
        short.audit_path.pop();
        assert_eq!(
            verify_inclusion(&all[5], &short),
            Err(super::InclusionFailure::MalformedPath)
        );
        let mut reordered = inclusion.clone();
        reordered.audit_path.swap(0, 1);
        assert_eq!(
            verify_inclusion(&all[5], &reordered),
            Err(super::InclusionFailure::RootMismatch)
        );
        let mut out_of_range = inclusion.clone();
        out_of_range.leaf_index = 8;
        assert_eq!(
            verify_inclusion(&all[5], &out_of_range),
            Err(super::InclusionFailure::IndexOutOfRange)
        );
        let mut wrong_root = inclusion.clone();
        wrong_root.root_hash = leaf_hash(b"not the root");
        assert_eq!(
            verify_inclusion(&all[5], &wrong_root),
            Err(super::InclusionFailure::RootMismatch)
        );
    }

    /// Two heads of equal size that disagree are not consistent: the fork case.
    #[test]
    fn equal_size_fork_is_refused() {
        let left = leaves(4);
        let mut right = leaves(4);
        right[3] = b"rewritten".to_vec();
        assert_eq!(
            verify_consistency(
                &root_of_leaves(&digests(4)),
                4,
                &root_of_leaves(&right.iter().map(|leaf| leaf_hash(leaf)).collect::<Vec<_>>()),
                4,
                &[]
            ),
            Err(super::ConsistencyFailure::RootMismatch)
        );
        let _ = left;
    }

    /// An empty tree has no root, and §33.5 says so rather than inventing one.
    #[test]
    fn empty_tree_has_no_root() {
        assert!(super::root_of(&[]).is_none());
    }

    /// An inclusion at index `n - 1` of `n` leaves verifies, which is what an
    /// appender produces.
    #[test]
    fn appended_leaf_verifies() {
        let mut leaves = Vec::new();
        for index in 0..5u8 {
            let inclusion = super::append(&mut leaves, vec![index]);
            assert_eq!(inclusion.leaf_index, index as u64);
            assert_eq!(inclusion.tree_size, index as u64 + 1);
            assert!(verify_inclusion(&leaves[index as usize], &inclusion).is_ok());
        }
    }
}
