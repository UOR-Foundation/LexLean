//! The compiler running the `Sha256` package reproduces it (SPEC.md §17.22,
//! `SL-08`, #126 stage 2): the digests the committed exports record are the
//! oracle's, the package's `lib.rs` holds none of them, and the standard
//! library rebuilt by the compiler is the committed export, whose `sha256`
//! `lib.rs` is the file the compiler links. The oracle may falsify and never
//! establishes.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value as Json;
use sha2::{Digest, Sha256};

use crate::production_export as harness;
use crate::support;

/// The committed exports.
const EXPORTS: [&str; 4] = ["bootstrap", "production", "production-coverage", "stdlib"];
/// The two SHA-256 packages, by directory under the `rust-std` profile.
const SHA_PACKAGES: [&str; 2] = ["sha256", "sha256_hex"];
/// The link the compiler is built from.
const LINK: &str = "crates/lexlean/sha256_package.rs";

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// The committed packages of one export: package directory (`rust-std/name`)
/// to the package's files by relative path.
fn packages(files: BTreeMap<String, Vec<u8>>) -> BTreeMap<String, BTreeMap<String, Vec<u8>>> {
    let mut out: BTreeMap<String, BTreeMap<String, Vec<u8>>> = BTreeMap::new();
    for (path, bytes) in files {
        let mut parts = path.splitn(3, '/');
        let (Some(profile), Some(name), Some(rest)) = (parts.next(), parts.next(), parts.next())
        else {
            continue;
        };
        out.entry(format!("{profile}/{name}"))
            .or_default()
            .insert(rest.to_owned(), bytes);
    }
    out
}

/// The first file `provenance` names whose recorded digest is not the
/// oracle's digest of the package's file; the number of digests compared.
fn recorded_digests(provenance: &Json, files: &BTreeMap<String, Vec<u8>>) -> Result<usize, String> {
    let named = provenance["files"]
        .as_object()
        .ok_or("a provenance names its files")?;
    for (path, recorded) in named {
        let bytes = files
            .get(path)
            .ok_or_else(|| format!("{path} is named and absent"))?;
        let oracle = hex(Sha256::digest(bytes).as_slice());
        if recorded.as_str() != Some(oracle.as_str()) {
            return Err(format!(
                "{path}: recorded {recorded}, the oracle gives {oracle}"
            ));
        }
    }
    Ok(named.len())
}

/// Every 64-digit lowercase hexadecimal string in `text`.
fn digests_in(text: &str) -> BTreeSet<String> {
    let bytes = text.as_bytes();
    let mut out = BTreeSet::new();
    let mut start = 0;
    while start + 64 <= bytes.len() {
        let window = &bytes[start..start + 64];
        if window
            .iter()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(b))
        {
            out.insert(String::from_utf8_lossy(window).into_owned());
            start += 64;
        } else {
            start += 1;
        }
    }
    out
}

/// Whether `lib` holds a digest that `provenance` records.
fn lib_holds_a_digest(lib: &str, provenance: &str) -> Option<String> {
    digests_in(provenance)
        .into_iter()
        .find(|digest| lib.contains(digest.as_str()))
}

/// Run the case for `SL-08`.
///
/// # Panics
///
/// Panics when the case's assertion fails.
#[allow(clippy::too_many_lines)]
pub(super) fn run(id: &str) {
    assert_eq!(id, "SL-08");
    let root = support::repo_root();

    // The digests every committed export records are the oracle's.
    let mut compared = 0usize;
    let mut sha: Vec<(String, String, String)> = Vec::new();
    for example in EXPORTS {
        let tree = harness::tree(&harness::fixture_dir(example).join("packages"));
        for (directory, files) in packages(tree) {
            let provenance_bytes = files
                .get("provenance.json")
                .unwrap_or_else(|| panic!("{example}/{directory}: no provenance.json"));
            let provenance: Json =
                serde_json::from_slice(provenance_bytes).expect("a provenance is JSON");
            compared += recorded_digests(&provenance, &files)
                .unwrap_or_else(|reason| panic!("{example}/{directory}: {reason}"));
            if example == "stdlib" && SHA_PACKAGES.iter().any(|name| directory.ends_with(name)) {
                sha.push((
                    directory,
                    String::from_utf8(files["src/lib.rs"].clone()).expect("UTF-8"),
                    String::from_utf8(provenance_bytes.clone()).expect("UTF-8"),
                ));
            }
        }
    }
    assert!(compared > 100, "{compared} recorded digests compared");
    assert_eq!(sha.len(), SHA_PACKAGES.len(), "the two SHA-256 packages");

    // `lib.rs` holds none of the digests that describe the package.
    for (directory, lib, provenance) in &sha {
        assert_eq!(
            lib_holds_a_digest(lib, provenance),
            None,
            "{directory}: lib.rs holds a digest its provenance records"
        );
    }

    // Each planted defect is found by the check the tree just passed.
    let (directory, lib, provenance) = &sha[0];
    let mut altered: Json = serde_json::from_str(provenance).expect("JSON");
    let recorded = altered["files"]["src/lib.rs"]
        .as_str()
        .expect("lib.rs is named")
        .to_owned();
    let mut flipped = recorded.clone().into_bytes();
    flipped[0] = if flipped[0] == b'0' { b'1' } else { b'0' };
    altered["files"]["src/lib.rs"] = Json::String(String::from_utf8(flipped).expect("hexadecimal"));
    let files = packages(harness::tree(
        &harness::fixture_dir("stdlib").join("packages"),
    ));
    assert!(
        recorded_digests(&altered, &files[directory]).is_err(),
        "a bit changed in a recorded digest is found"
    );
    let planted = format!("{lib}\n// {recorded}\n");
    assert!(
        lib_holds_a_digest(&planted, provenance).is_some(),
        "a digest written into lib.rs is found"
    );

    // Under the pinned Lean, rebuilding the standard library with the
    // compiler running the package yields the committed packages, and the
    // rebuilt `lib.rs` of `sha256` is the file the compiler links.
    if !support::lean_backed("SL-08") {
        return;
    }
    let exported = harness::exported("stdlib");
    let bundle = exported.bundle();
    let rebuilt = harness::tree(&bundle.join("packages"));
    let committed = harness::tree(&harness::fixture_dir("stdlib").join("packages"));
    assert_eq!(
        rebuilt.keys().collect::<Vec<_>>(),
        committed.keys().collect::<Vec<_>>(),
        "the rebuilt package files are the committed ones"
    );
    for (relative, content) in &rebuilt {
        assert!(
            committed.get(relative) == Some(content),
            "packages/{relative} differs from the committed package"
        );
    }
    let linked = std::fs::read(root.join(LINK).as_std_path())
        .unwrap_or_else(|error| panic!("{LINK}: {error}"));
    let rebuilt_lib = rebuilt
        .get("rust-std/sha256/src/lib.rs")
        .expect("the rebuilt sha256 lib.rs");
    assert!(
        &linked == rebuilt_lib,
        "the file the compiler links is not the lib.rs it rebuilds"
    );
    let mut stale = linked.clone();
    let middle = stale.len() / 2;
    stale[middle] ^= 1;
    assert!(
        &stale != rebuilt_lib,
        "a link that is not the rebuilt lib.rs is found"
    );
}
