//! Every content identity is the one the `sha2` crate gave (SPEC.md §21,
//! `SL-07`, #126 stage 2): the compiler's digest is held, over the whole
//! repository tree, to the oracle that computed every committed identity
//! before the package replaced it, to the values those identities left in the
//! committed locks, and to the independent recomputations of the recipes. The
//! oracle may falsify and never establishes.

use lexlean::artifact::content_id::Sha256Digest;
use sha2::{Digest, Sha256};

use crate::support::{self, P};

/// The committed example whose lock pins each language's compiler-semantics
/// ID, written before the compiler's digest was the package.
const PINS: [(&str, &str); 3] = [
    ("1.0", "examples/nat-add-zero/lexlean.lock"),
    ("1.1", "examples/semantic-1.1/lexlean.lock"),
    ("1.2", "examples/language-1.2/lexlean.lock"),
];

/// The recipes whose independent recomputations are cases of their own.
const RECIPES: [&str; 6] = ["AR-05", "AR-06", "SM-10", "CF-07", "CF-09", "TX-10"];

/// The example whose lock and records the Atlas legs hold (it is checked by
/// `verify-examples` and the shards, not by this case).
const ATLAS: &str = "uor-atlas";

fn oracle(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}

/// The tracked files and their digests under `digest`, compared with the
/// oracle's: the first file that differs, and how many were compared.
fn compare_tree(
    files: &[(String, Vec<u8>)],
    digest: &dyn Fn(&[u8]) -> [u8; 32],
) -> (Option<String>, usize) {
    for (index, (path, bytes)) in files.iter().enumerate() {
        if digest(bytes) != oracle(bytes) {
            return (Some(path.clone()), index + 1);
        }
    }
    (None, files.len())
}

/// The pinned `compiler_semantics` of a committed lock.
fn pinned(root: &camino::Utf8Path, lock: &str) -> String {
    let text = std::fs::read_to_string(root.join(lock).as_std_path())
        .unwrap_or_else(|error| panic!("{lock}: {error}"));
    text.lines()
        .find_map(|line| {
            line.strip_prefix("compiler_semantics = \"")
                .and_then(|rest| rest.strip_suffix('"'))
        })
        .unwrap_or_else(|| panic!("{lock}: no compiler_semantics"))
        .to_owned()
}

/// Run the case for `SL-07`.
///
/// # Panics
///
/// Panics when the case's assertion fails.
pub(super) fn run(id: &str) {
    assert_eq!(id, "SL-07");
    let root = support::repo_root();

    // Armed: the two sides are two implementations, so agreement is evidence.
    super::stdlib_digest::has_no_sha2(&super::stdlib_digest::workspace_metadata()).unwrap_or_else(
        |reason| panic!("{reason}: the compiler's digest and the oracle are one implementation"),
    );

    // Every tracked file, by the compiler and by the oracle.
    let listing = std::process::Command::new("git")
        .args(["ls-files", "-z"])
        .current_dir(root.as_std_path())
        .output()
        .expect("git runs");
    assert!(listing.status.success(), "git ls-files");
    let mut files: Vec<(String, Vec<u8>)> = Vec::new();
    for path in listing
        .stdout
        .split(|byte| *byte == 0)
        .filter(|p| !p.is_empty())
    {
        let path = String::from_utf8(path.to_vec()).expect("tracked paths are UTF-8");
        // A tracked link to a directory has no bytes of its own; any other
        // path that cannot be read fails the case, so the tree cannot thin
        // silently.
        let full = root.join(&path);
        if full.as_std_path().is_dir() {
            continue;
        }
        let bytes = std::fs::read(full.as_std_path())
            .unwrap_or_else(|error| panic!("{path}: a tracked file cannot be read: {error}"));
        files.push((path, bytes));
    }
    assert!(
        files.len() > 5_000,
        "{} tracked files: the tree is not the repository",
        files.len()
    );
    let compiler = |bytes: &[u8]| Sha256Digest::of(bytes).0;
    let (differs, compared) = compare_tree(&files, &compiler);
    assert_eq!(
        differs, None,
        "the compiler's digest differs from the oracle's on a tracked file ({compared} compared)"
    );

    // The compiler-semantics IDs, against what the committed locks pinned.
    for (language, lock) in PINS {
        let committed = pinned(&root, lock);
        assert_eq!(
            lexlean::compiler_semantics_id_for(language).to_hex(),
            committed,
            "the compiler-semantics ID of language {language} is the one {lock} pins"
        );
    }

    // The recipes, by the cases that recompute them without the compiler.
    for recipe in RECIPES {
        super::run(recipe);
    }

    // Every locked project recomputes its pins and its lock.
    let mut names: Vec<String> = std::fs::read_dir(root.join("examples").as_std_path())
        .expect("the examples")
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| name != ATLAS)
        .collect();
    names.sort();
    assert!(names.len() >= 10, "{} examples", names.len());
    for name in &names {
        P::copy_example(name).check_ok();
    }
    for project in [P::stdlib(), P::compiler(), P::bootstrap()] {
        project.check_ok();
    }

    // Each planted defect is found by the check the tree just passed.
    let flipped = |bytes: &[u8]| {
        let mut out = Sha256Digest::of(bytes).0;
        out[0] ^= 1;
        out
    };
    assert!(
        compare_tree(&files, &flipped).0.is_some(),
        "a bit changed in a tracked file's digest is found"
    );
    let (_, lock) = PINS[2];
    let mut pin = pinned(&root, lock).into_bytes();
    pin[0] = if pin[0] == b'0' { b'1' } else { b'0' };
    assert_ne!(
        lexlean::compiler_semantics_id_for("1.2")
            .to_hex()
            .as_bytes(),
        pin.as_slice(),
        "a bit changed in a pinned compiler-semantics ID is found"
    );
    let mut right = lexlean::artifact::content_id::FramedHasher::new("lexlean-source-v1");
    right.frame("project", b"project");
    right.frame("lock", b"lock");
    let mut independent = Sha256::new();
    independent.update(b"lexlean-source-v1\0");
    // The frames in the wrong order: the lock before the project.
    for (label, bytes) in [("lock", b"lock".as_slice()), ("project", b"project")] {
        independent.update((label.len() as u32).to_be_bytes());
        independent.update(label.as_bytes());
        independent.update((bytes.len() as u64).to_be_bytes());
        independent.update(bytes);
    }
    assert_ne!(
        right.finish().0,
        <[u8; 32]>::from(independent.finalize()),
        "the order of a source-ID recipe's frames is found"
    );
}
