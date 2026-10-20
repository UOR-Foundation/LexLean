//! The compiler's digest is the standard library's `Sha256` package
//! (SPEC.md §17.22, `SL-05`, #126 stage 2), held through the compiler's own
//! `Sha256Digest::of`, `FramedHasher`, and `tree_digest` to the authorities
//! that establish a SHA-256 digest (the FIPS 180-4 examples and the NIST CAVP
//! vectors) and to the `sha2` crate, which may falsify and never establishes
//! anything. Every defect the statement lists is planted here, on a copy, and
//! must be found by the same check the real tree passes.

use lexlean::artifact::content_id::{tree_digest, FramedHasher, Sha256Digest};
use sha2::{Digest, Sha256};

use super::stdlib_sha256::parse_rsp;
use crate::support;

/// The committed export the compiler links (SPEC.md §17.22).
const EXPORT: &str = "tests/production-export/stdlib/packages/rust-std/sha256/src/lib.rs";
/// The in-crate link to it.
const LINK: &str = "crates/lexlean/sha256_package.rs";

/// The FIPS 180-4 example messages and their digests, as the standard states
/// them: the one-block message, the two-block message, and the empty one.
const FIPS: [(&[u8], &str); 3] = [
    (
        b"abc",
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
    ),
    (
        b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
        "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
    ),
    (
        b"",
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    ),
];
/// The FIPS 180-4 million-`a` example.
const MILLION_A: &str = "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0";

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

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn oracle(message: &[u8]) -> [u8; 32] {
    Sha256::digest(message).into()
}

/// The names `lexlean` depends on, of every kind, read from the output of
/// `cargo metadata --no-deps` for the workspace: the resolved package names,
/// so a `[dependencies.sha2]` table or a renamed `package = "sha2"` counts as
/// the dependency it is.
pub(super) fn shipped_dependencies(metadata: &serde_json::Value) -> Vec<String> {
    let package = metadata["packages"]
        .as_array()
        .and_then(|packages| packages.iter().find(|package| package["name"] == "lexlean"))
        .expect("cargo metadata lists the lexlean package");
    package["dependencies"]
        .as_array()
        .expect("a package lists its dependencies")
        .iter()
        .map(|dependency| dependency["name"].as_str().expect("a name").to_owned())
        .collect()
}

/// The dependency check of the statement: `lexlean` has no `sha2`.
pub(super) fn has_no_sha2(metadata: &serde_json::Value) -> Result<(), String> {
    let dependencies = shipped_dependencies(metadata);
    assert!(
        dependencies.len() > 5,
        "the shipped crate's dependency list is read ({dependencies:?})"
    );
    if dependencies.iter().any(|name| name == "sha2") {
        Err("R6: the shipped crate depends on `sha2`".to_owned())
    } else {
        Ok(())
    }
}

/// `cargo metadata` of this workspace, offline and against the lock.
pub(super) fn workspace_metadata() -> serde_json::Value {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let output = std::process::Command::new(cargo)
        .args([
            "metadata",
            "--no-deps",
            "--offline",
            "--locked",
            "--format-version",
            "1",
        ])
        .current_dir(support::repo_root().as_std_path())
        .output()
        .expect("cargo metadata runs");
    assert!(
        output.status.success(),
        "cargo metadata: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("cargo metadata is JSON")
}

/// The link check: the in-crate link resolves to the export, and the bytes it
/// reads are the export's.
fn link_is_the_export(link: &[u8], export: &[u8]) -> Result<(), String> {
    if link == export {
        Ok(())
    } else {
        Err(format!(
            "the linked package ({} bytes) is not the committed export ({} bytes)",
            link.len(),
            export.len()
        ))
    }
}

/// The first message on which `digest` differs from the authority or the
/// oracle, if any: the three FIPS examples by their stated digests, the
/// CAVP vectors by theirs, every length to `LENGTHS`, and the neighbours of
/// every power of two to 4 MiB by the oracle.
fn first_disagreement(
    digest: &dyn Fn(&[u8]) -> [u8; 32],
    vectors: &[(Vec<u8>, Vec<u8>)],
) -> Option<String> {
    for (message, stated) in FIPS.iter().map(|(m, d)| (m.to_vec(), *d)) {
        if hex(&digest(&message)) != stated {
            return Some(format!("FIPS 180-4 example of {} bytes", message.len()));
        }
    }
    for (message, stated) in vectors {
        if digest(message).as_slice() != stated.as_slice() {
            return Some(format!("CAVP vector of {} bytes", message.len()));
        }
    }
    for length in 0..=LENGTHS {
        let message = stream(length as u64 + 1, length);
        if digest(&message) != oracle(&message) {
            return Some(format!("seeded message of {length} bytes"));
        }
    }
    for power in 0..=22u32 {
        for length in [(1usize << power) - 1, 1 << power, (1 << power) + 1] {
            let message = stream(u64::from(power) + 1000, length);
            if digest(&message) != oracle(&message) {
                return Some(format!("seeded message of {length} bytes"));
            }
        }
    }
    None
}

/// The §21.1 frame encoding written independently of the compiler, with the
/// length prefixes' byte order chosen by `big_endian`; the oracle digests it.
fn recomputed_frames(domain: &str, frames: &[(&str, &[u8])], big_endian: bool) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(domain.as_bytes());
    hasher.update([0u8]);
    for (label, bytes) in frames {
        let (label_length, body_length) = (label.len() as u32, bytes.len() as u64);
        if big_endian {
            hasher.update(label_length.to_be_bytes());
            hasher.update(label.as_bytes());
            hasher.update(body_length.to_be_bytes());
        } else {
            hasher.update(label_length.to_le_bytes());
            hasher.update(label.as_bytes());
            hasher.update(body_length.to_le_bytes());
        }
        hasher.update(bytes);
    }
    hasher.finalize().into()
}

/// Whether the compiler's framing agrees with an encoding that writes the
/// prefixes in the stated order, on frames that make a wrong order visible.
fn frames_agree(big_endian: bool) -> bool {
    let long = stream(7, 70_000);
    let frames: [(&str, &[u8]); 4] = [
        ("a", b""),
        ("project", b"name = \"x\"\n"),
        ("source", &long),
        ("path", b"src/Main.lex.tex"),
    ];
    let mut hasher = FramedHasher::new("lexlean-source-v1");
    for (label, bytes) in frames {
        hasher.frame(label, bytes);
    }
    hasher.finish().0 == recomputed_frames("lexlean-source-v1", &frames, big_endian)
}

/// The §11.5 tree encoding, independently.
fn recomputed_tree(files: &[(&str, &[u8])]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"lexlean-tree-v1\0");
    for (path, bytes) in files {
        hasher.update((path.len() as u32).to_be_bytes());
        hasher.update(path.as_bytes());
        hasher.update((bytes.len() as u64).to_be_bytes());
        hasher.update(bytes);
    }
    hasher.finalize().into()
}

/// A package copy with one bit of its first round constant flipped, built
/// with `rustc` and run on the FIPS examples; its digests, one per example.
fn plant_round_constant() -> Vec<[u8; 32]> {
    let root = support::repo_root();
    let export = std::fs::read_to_string(root.join(EXPORT).as_std_path()).expect("the export");
    assert_eq!(export.matches("1116352408u32").count(), 1, "one K[0]");
    let planted = export.replace("1116352408u32", "1116352409u32");
    let directory = tempfile::tempdir().expect("a temporary directory");
    let package = directory.path().join("package.rs");
    std::fs::write(&package, planted).expect("the planted package");
    let harness = directory.path().join("main.rs");
    std::fs::write(
        &harness,
        r#"#[path = "package.rs"]
#[allow(warnings)]
mod package;
fn main() {
    for message in [&b"abc"[..], b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq", b""] {
        let package::Adt0::C0(a, b, c, d, e, f, g, h) =
            package::run(package::Bytes::lit(message)).expect("in domain");
        for word in [a, b, c, d, e, f, g, h] {
            print!("{word:08x}");
        }
        println!();
    }
}
"#,
    )
    .expect("the harness");
    let binary = directory.path().join("planted");
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_owned());
    let built = std::process::Command::new(rustc)
        .args(["--edition", "2021", "-C", "opt-level=1", "-o"])
        .arg(&binary)
        .arg(&harness)
        .output()
        .expect("rustc runs");
    assert!(
        built.status.success(),
        "the planted package builds: {}",
        String::from_utf8_lossy(&built.stderr)
    );
    let ran = std::process::Command::new(&binary)
        .output()
        .expect("the planted harness runs");
    String::from_utf8(ran.stdout)
        .expect("hexadecimal")
        .lines()
        .map(|line| {
            let mut out = [0u8; 32];
            for (index, byte) in out.iter_mut().enumerate() {
                *byte = u8::from_str_radix(&line[index * 2..index * 2 + 2], 16).expect("hex");
            }
            out
        })
        .collect()
}

/// Run the case for `SL-05`.
///
/// # Panics
///
/// Panics when the case's assertion fails.
#[allow(clippy::too_many_lines)]
pub(super) fn run(id: &str) {
    assert_eq!(id, "SL-05");
    let root = support::repo_root();

    // R6: the shipped crate's boundary is explicit, it has no second
    // implementation of the digest beside the package, and the package it
    // links is the committed export (a dangling or diverging link fails).
    let metadata = workspace_metadata();
    has_no_sha2(&metadata).unwrap_or_else(|reason| panic!("{reason}"));
    let export = std::fs::read(root.join(EXPORT).as_std_path()).expect("the committed export");
    let link = std::fs::read(root.join(LINK).as_std_path())
        .unwrap_or_else(|error| panic!("{LINK}: the link to the export is broken: {error}"));
    link_is_the_export(&link, &export).unwrap_or_else(|reason| panic!("{reason}"));

    // Each digest the case states is stated by a `some-true` claim of the
    // ledger, which cites the authority row that vendors the document or the
    // file (R2): a constant the register does not state is not an authority.
    let ledger =
        std::fs::read_to_string(root.join("model/ledger.toml").as_std_path()).expect("the ledger");
    for (_, stated) in FIPS.iter().chain([&(&b""[..], MILLION_A)]) {
        assert!(
            ledger.contains(stated),
            "no authority claim of model/ledger.toml states the digest {stated}"
        );
    }

    // The authorities first: a vector and `sha2` must agree before either
    // judges the compiler.
    for (message, stated) in FIPS {
        assert_eq!(hex(&oracle(message)), stated, "FIPS 180-4 and `sha2` agree");
    }
    assert_eq!(
        hex(&oracle(&vec![b'a'; 1_000_000])),
        MILLION_A,
        "FIPS 180-4's million-`a` example and `sha2` agree"
    );
    let mut vectors: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
    for file in ["SHA256ShortMsg.rsp", "SHA256LongMsg.rsp"] {
        let path = root.join("tests/stdlib/sha256").join(file);
        let text = std::fs::read_to_string(path.as_std_path())
            .unwrap_or_else(|error| panic!("{file}: {error}"));
        let parsed = parse_rsp(&text);
        assert!(parsed.len() >= 64, "{file}: {} vectors", parsed.len());
        for (message, stated) in parsed {
            assert_eq!(
                oracle(&message).as_slice(),
                stated.as_slice(),
                "{file}: the vector of {} bytes and `sha2` disagree",
                message.len()
            );
            vectors.push((message, stated));
        }
    }

    // The compiler's own digest against all of them.
    let compiler = |message: &[u8]| Sha256Digest::of(message).0;
    if let Some(found) = first_disagreement(&compiler, &vectors) {
        panic!("the compiler's digest disagrees with its authority or the oracle on a {found}");
    }
    assert_eq!(
        hex(&compiler(&vec![b'a'; 1_000_000])),
        MILLION_A,
        "the compiler's digest of FIPS 180-4's million-`a` example"
    );
    for round in 0..40u64 {
        let message = stream(round + 5000, (round as usize + 1) * 104_729);
        assert_eq!(
            compiler(&message),
            oracle(&message),
            "the compiler's digest of {} seeded bytes",
            message.len()
        );
    }

    // Frames and tree rows, against the encodings written out independently.
    assert!(frames_agree(true), "FramedHasher writes the §21.1 frames");
    let big = stream(11, 300_000);
    let rows: [(&str, &[u8]); 3] = [("a/b", b"one"), ("a/c", &big), ("z", b"")];
    assert_eq!(
        tree_digest(&rows).0,
        recomputed_tree(&rows),
        "tree_digest writes the §11.5 rows"
    );

    // Each planted defect is found by the check the tree just passed.
    for planted in [
        serde_json::json!({"name": "sha2", "kind": null}),
        serde_json::json!({"name": "sha2", "rename": "digest", "kind": "dev"}),
    ] {
        let mut altered = metadata.clone();
        let lexlean = altered["packages"]
            .as_array_mut()
            .and_then(|packages| {
                packages
                    .iter_mut()
                    .find(|package| package["name"] == "lexlean")
            })
            .expect("lexlean");
        lexlean["dependencies"]
            .as_array_mut()
            .expect("dependencies")
            .push(planted);
        assert!(
            has_no_sha2(&altered).is_err(),
            "a `sha2` dependency of any form or kind is found"
        );
    }
    let mut diverging = export.clone();
    let middle = diverging.len() / 2;
    diverging[middle] ^= 1;
    assert!(
        link_is_the_export(&diverging, &export).is_err(),
        "a link that differs from the export by one byte is found"
    );
    let truncated =
        |message: &[u8]| Sha256Digest::of(&message[..message.len().saturating_sub(1)]).0;
    assert!(
        first_disagreement(&truncated, &vectors).is_some(),
        "a message truncated by one byte is found"
    );
    assert!(
        !frames_agree(false),
        "a frame length written little-endian is found"
    );
    let planted = plant_round_constant();
    assert_eq!(planted.len(), FIPS.len(), "one digest per example");
    for ((_, stated), digest) in FIPS.iter().zip(&planted) {
        assert_ne!(
            hex(digest),
            *stated,
            "a bit of the first round constant is found on every FIPS 180-4 example"
        );
    }
}
