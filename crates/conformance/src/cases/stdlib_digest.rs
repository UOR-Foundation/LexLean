//! The compiler's digest is the standard library's `Sha256` package
//! (SPEC.md §17.22, `SL-05`, #126 stage 2), held through the compiler's own
//! `Sha256Digest::of` to the authorities that establish a SHA-256 digest (the
//! FIPS 180-4 examples and the NIST CAVP vectors) and to the `sha2` crate,
//! which may falsify and never establishes anything.

use lexlean::Sha256Digest;
use sha2::{Digest, Sha256};

use super::stdlib_sha256::parse_rsp;
use crate::support;

/// The FIPS 180-4 example messages: the one-block, the two-block, the empty.
const FIPS: [&[u8]; 3] = [
    b"abc",
    b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
    b"",
];

/// The longest message length every padding boundary is covered up to.
const LENGTHS: usize = 300;

/// A deterministic byte stream (xorshift64*), so the corpus is the same on
/// every host.
fn stream(seed: u64, length: usize) -> Vec<u8> {
    let mut state = seed | 1;
    (0..length)
        .map(|_| {
            state ^= state >> 12;
            state ^= state << 25;
            state ^= state >> 27;
            (state.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 56) as u8
        })
        .collect()
}

/// Run the case for `SL-05`.
///
/// # Panics
///
/// Panics when the case's assertion fails.
pub(super) fn run(id: &str) {
    assert_eq!(id, "SL-05");
    let root = support::repo_root();

    // R6: the shipped crate's boundary is explicit, and it has no second
    // implementation of the digest beside the package.
    let manifest = std::fs::read_to_string(root.join("crates/lexlean/Cargo.toml").as_std_path())
        .expect("the shipped crate's manifest");
    assert!(
        !manifest
            .lines()
            .any(|line| line.trim_start().starts_with("sha2")),
        "R6: the shipped `lexlean` crate still depends on `sha2`"
    );

    // The authorities first: a vector and `sha2` must agree before either
    // judges the compiler.
    let mut vectors: Vec<(Vec<u8>, Vec<u8>)> = FIPS
        .iter()
        .map(|message| (message.to_vec(), Sha256::digest(message).to_vec()))
        .collect();
    for file in ["SHA256ShortMsg.rsp", "SHA256LongMsg.rsp"] {
        let path = root.join("tests/stdlib/sha256").join(file);
        let text = std::fs::read_to_string(path.as_std_path())
            .unwrap_or_else(|error| panic!("{file}: {error}"));
        for (message, stated) in parse_rsp(&text) {
            assert_eq!(
                Sha256::digest(&message).as_slice(),
                stated.as_slice(),
                "{file}: the vector of {} bytes and `sha2` disagree",
                message.len()
            );
            vectors.push((message, stated));
        }
    }
    for (message, stated) in &vectors {
        assert_eq!(
            Sha256Digest::of(message).0.as_slice(),
            stated.as_slice(),
            "the compiler's digest of a vector of {} bytes disagrees with its authority",
            message.len()
        );
    }

    // Every padding boundary, then a seeded corpus, against the oracle.
    for length in 0..=LENGTHS {
        let message = stream(length as u64 + 1, length);
        assert_eq!(
            Sha256Digest::of(&message).0.as_slice(),
            Sha256::digest(&message).as_slice(),
            "the compiler's digest of {length} seeded bytes disagrees with the oracle"
        );
    }
}
