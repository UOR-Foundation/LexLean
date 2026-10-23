//! The core runtime as source (SPEC.md §17.16): `RB-16`.
//!
//! The functions of `RustRuntime` are decoded and printed where the
//! handwritten functions stood. A harness calls the generated runtime on the
//! boundary and seeded values of each defined function and records what each
//! call returned and the work it counted; the digests are the golden
//! transcript in `tests/rust-runtime-golden/transcript.txt`, which was recorded from
//! the handwritten runtime at the last pass of the differential (see
//! VERIFICATION.md).

use std::collections::{BTreeMap, BTreeSet};

use lexlean::calculus::rust::ast::{Ident, ItemDef};
use lexlean::calculus::rust::{runtime, source};
use lexlean::calculus::IntKind;

use crate::rust_build::{cargo_in, workspace, Built};
use crate::support::repo_root;

const MODULE: &str = "compiler/src/RustRuntime.lex.tex";
const SHIPPED: &str = "language/rust-runtime-1.2/RustRuntime.lex.tex";
/// The golden transcript of the harness on the core runtime.
const GOLDEN: &str = "core";

/// The calls the harness makes of each defined function: the `same!` of the
/// harness runs the function and records what it returns and the work it counts.
const SCALARS: [(&str, &str); 18] = [
    ("nat_add",
        "for &a in &nats { for &b in &nats { same!(nat_add(a, b)); } }\n    for _ in 0..1000 { let (a, b) = (next(), next()); same!(nat_add(a, b)); same!(nat_add(a >> 1, b >> 1)); }"),
    ("nat_sub",
        "for &a in &nats { for &b in &nats { same!(nat_sub(a, b)); } }\n    for _ in 0..1000 { let (a, b) = (next(), next()); same!(nat_sub(a, b)); same!(nat_sub(a >> 1, b >> 1)); }"),
    ("nat_mul",
        "for &a in &nats { for &b in &nats { same!(nat_mul(a, b)); } }\n    for _ in 0..1000 { let (a, b) = (next(), next()); same!(nat_mul(a, b)); same!(nat_mul(a >> 1, b >> 1)); }"),
    ("nat_quot",
        "for &a in &nats { for &b in &nats { for &z in &[0u64, 7, u64::MAX] { same!(nat_quot(a, b, z)); } } }\n    for _ in 0..1000 { let (a, b, z) = (next(), next(), next()); same!(nat_quot(a, b, z)); same!(nat_quot(a >> 40, b >> 50, z)); }"),
    ("nat_rem",
        "for &a in &nats { for &b in &nats { for &z in &[0u64, 7, u64::MAX] { same!(nat_rem(a, b, z)); } } }\n    for _ in 0..1000 { let (a, b, z) = (next(), next(), next()); same!(nat_rem(a, b, z)); same!(nat_rem(a >> 40, b >> 50, z)); }"),
    ("nat_eq",
        "for &a in &nats { for &b in &nats { same!(nat_eq(a, b)); } }\n    for _ in 0..1000 { let (a, b) = (next(), next()); same!(nat_eq(a, b)); same!(nat_eq(a >> 1, b >> 1)); }"),
    ("nat_le",
        "for &a in &nats { for &b in &nats { same!(nat_le(a, b)); } }\n    for _ in 0..1000 { let (a, b) = (next(), next()); same!(nat_le(a, b)); same!(nat_le(a >> 1, b >> 1)); }"),
    ("nat_lt",
        "for &a in &nats { for &b in &nats { same!(nat_lt(a, b)); } }\n    for _ in 0..1000 { let (a, b) = (next(), next()); same!(nat_lt(a, b)); same!(nat_lt(a >> 1, b >> 1)); }"),
    ("nat_succ",
        "for &a in &nats { same!(nat_succ(a)); }\n    for _ in 0..1000 { let a = next(); same!(nat_succ(a)); }"),
    ("int_add",
        "for &a in &ints { for &b in &ints { same!(int_add(a, b)); } }\n    for _ in 0..1000 { let (a, b) = (next() as i64, next() as i64); same!(int_add(a, b)); same!(int_add(a >> 1, b >> 1)); }"),
    ("int_sub",
        "for &a in &ints { for &b in &ints { same!(int_sub(a, b)); } }\n    for _ in 0..1000 { let (a, b) = (next() as i64, next() as i64); same!(int_sub(a, b)); same!(int_sub(a >> 1, b >> 1)); }"),
    ("int_mul",
        "for &a in &ints { for &b in &ints { same!(int_mul(a, b)); } }\n    for _ in 0..1000 { let (a, b) = (next() as i64, next() as i64); same!(int_mul(a, b)); same!(int_mul(a >> 1, b >> 1)); }"),
    ("int_neg",
        "for &a in &ints { same!(int_neg(a)); }\n    for _ in 0..1000 { let a = next() as i64; same!(int_neg(a)); }"),
    ("int_quot",
        "for &a in &ints { for &b in &ints { for &z in &[i64::MIN, -1, 0, 7, i64::MAX] { same!(int_quot(a, b, z)); } } }\n    for _ in 0..1000 { let (a, b, z) = (next() as i64, next() as i64, next() as i64); same!(int_quot(a, b, z)); same!(int_quot(a >> 40, b >> 50, z)); }"),
    ("int_rem",
        "for &a in &ints { for &b in &ints { for &z in &[i64::MIN, -1, 0, 7, i64::MAX] { same!(int_rem(a, b, z)); } } }\n    for _ in 0..1000 { let (a, b, z) = (next() as i64, next() as i64, next() as i64); same!(int_rem(a, b, z)); same!(int_rem(a >> 40, b >> 50, z)); }"),
    (
        "bool_not",
        "for &a in &[false, true] { same!(bool_not(a)); }",
    ),
    (
        "bool_and",
        "for &a in &[false, true] { for &b in &[false, true] { same!(bool_and(a, b)); } }",
    ),
    (
        "bool_or",
        "for &a in &[false, true] { for &b in &[false, true] { same!(bool_or(a, b)); } }",
    ),
];

/// The differential over every function: the scalar functions, and the
/// functions of each width on its boundary values and 200 seeded pairs.
fn differential() -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = SCALARS
        .iter()
        .map(|(name, calls)| ((*name).to_owned(), (*calls).to_owned()))
        .collect();
    for kind in IntKind::ALL {
        let k = kind.name();
        let values = format!(
            "let v = [{k}::MIN, {k}::MIN.wrapping_add(1), (0 as {k}).wrapping_sub(1), 0, 1, 2, {k}::MAX / 2, {k}::MAX - 1, {k}::MAX];"
        );
        for name in [
            "checked_add",
            "checked_sub",
            "checked_mul",
            "checked_quot",
            "bit_and",
            "bit_or",
            "bit_xor",
        ] {
            let path = format!("fixed_{k}::{name}");
            out.push((
                path.clone(),
                format!(
                    "{values}\n    for &a in &v {{ for &b in &v {{ same!({path}(a, b)); }} }}\n    for _ in 0..200 {{ let (a, b) = (next() as {k}, next() as {k}); same!({path}(a, b)); }}"
                ),
            ));
        }
        let path = format!("fixed_{k}::bit_not");
        out.push((
            path.clone(),
            format!("{values}\n    for &a in &v {{ same!({path}(a)); }}"),
        ));
        for name in ["shift_left", "shift_right"] {
            let path = format!("fixed_{k}::{name}");
            out.push((
                path.clone(),
                format!(
                    "{values}\n    for &a in &v {{ for &s in &[0u32, 1, {k}::BITS - 1, {k}::BITS, {k}::BITS + 1, 200] {{ same!({path}(a, s)); }} }}"
                ),
            ));
        }
        let path = format!("fixed_{k}::convert");
        out.push((
            path.clone(),
            format!(
                "for &w in &[i128::MIN, {k}::MIN as i128 - 1, {k}::MIN as i128, -1i128, 0, 1, {k}::MAX as i128, {k}::MAX as i128 + 1, i128::MAX] {{ same!({path}(w)); }}\n    for _ in 0..200 {{ let w = (next() as i128) << (next() % 64); same!({path}(w)); }}"
            ),
        ));
        if kind.signed() {
            let path = format!("checked_neg_{k}");
            out.push((
                path.clone(),
                format!("{values}\n    for &a in &v {{ same!({path}(a)); }}"),
            ));
        }
    }
    out
}

/// The names of the functions a decoded module defines.
fn defined(items: &[ItemDef]) -> Result<BTreeSet<String>, String> {
    items
        .iter()
        .map(|item| match item {
            ItemDef::Function {
                name: Ident::Export(name),
                ..
            } => Ok(name.clone()),
            _ => Err(
                "the core runtime module defines an item that is not a named function".to_owned(),
            ),
        })
        .collect()
}

/// Whether the differential covers exactly the functions the module defines.
fn covered(items: &[ItemDef]) -> Result<(), String> {
    let defined = defined(items)?;
    let differential: BTreeSet<String> = differential().into_iter().map(|(name, _)| name).collect();
    if defined != differential {
        return Err(format!(
            "the module defines {defined:?}, the differential covers {differential:?}"
        ));
    }
    Ok(())
}

/// The recorder every harness reports through: each observation (a label, the
/// debug form of what a call returned, and the work it counted) is folded, in
/// order, into a 64-bit FNV-1a digest kept per label, and the digests are
/// printed at the end. The digest of a label is what the golden transcripts of
/// `tests/rust-runtime-golden/` record.
pub(super) const RECORDER: &str = "use std::cell::RefCell;\nuse std::collections::BTreeMap;\nthread_local! { static SEEN: RefCell<BTreeMap<String, u64>> = const { RefCell::new(BTreeMap::new()) }; }\nfn seen(label: &str, value: &str, work: u64) {\n    SEEN.with(|seen| {\n        let mut seen = seen.borrow_mut();\n        let digest = seen.entry(label.to_owned()).or_insert(0xcbf2_9ce4_8422_2325);\n        for byte in value.bytes().chain(*b\"/\").chain(work.to_string().into_bytes()).chain(*b\";\") {\n            *digest ^= u64::from(byte);\n            *digest = digest.wrapping_mul(0x0100_0000_01b3);\n        }\n    });\n}\nfn report() {\n    SEEN.with(|seen| {\n        for (label, digest) in seen.borrow().iter() { println!(\"transcript {label} {digest:016x}\"); }\n    });\n}\n";

/// The prelude of every harness of the scalar runtime: the recorder and the
/// `same!` that runs a call of the runtime and records what it returns and
/// the work it counts.
pub(super) fn prelude() -> String {
    format!(
        "use runtime_diff::rt as m;\n{RECORDER}macro_rules! same {{\n    ($($f:ident)::+($($a:expr),*)) => {{{{\n        let w0 = m::work();\n        let x = m::$($f)::+($($a),*);\n        seen(&stringify!($($f)::+).replace(' ', \"\"), &format!(\"{{x:?}}\"), m::work() - w0);\n    }}}};\n}}\nfn main() {{\n    let nats = [0u64, 1, 2, 3, 7, u32::MAX as u64, u32::MAX as u64 + 1, u64::MAX / 2, u64::MAX - 2, u64::MAX - 1, u64::MAX];\n    let ints = [i64::MIN, i64::MIN + 1, -3037000500, -2, -1, 0, 1, 2, 3037000500, i64::MAX - 1, i64::MAX];\n    let mut seed = 0x9e37_79b9_7f4a_7c15_u64;\n    let mut next = move || {{ seed ^= seed << 13; seed ^= seed >> 7; seed ^= seed << 17; seed }};\n"
    )
}

/// The harness: both runtimes, every function of the differential.
pub(super) fn harness() -> String {
    let mut out = prelude();
    for (_, calls) in differential() {
        out.push_str("    {\n    ");
        out.push_str(&calls);
        out.push_str("\n    }\n");
    }
    out.push_str("    report();\n    println!(\"runtime\");\n}\n");
    out
}

/// Compile `runtime` as the module `rt` of a crate under the pinned rustc with
/// warnings denied and run `harness` on it: the standard output of the
/// harness, or the reason it failed.
pub(super) fn run_harness(runtime: &str, harness: String) -> Result<String, String> {
    let directory = tempfile::tempdir().expect("a directory");
    let lib = format!("pub mod rt {{\n{runtime}\n}}\n");
    let files: BTreeMap<String, Vec<u8>> = BTreeMap::from([
        (
            "Cargo.toml".to_owned(),
            b"[package]\nname = \"runtime_diff\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[lints.rust]\nwarnings = \"deny\"\n".to_vec(),
        ),
        ("src/lib.rs".to_owned(), lib.into_bytes()),
    ]);
    workspace(
        directory.path(),
        &[Built {
            name: "runtime_diff".to_owned(),
            files,
            harness,
        }],
    );
    let output = cargo_in(
        directory.path(),
        &["run", "--quiet", "--offline", "-p", "run_runtime_diff"],
    );
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    } else {
        Err(format!(
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

/// The transcript a harness printed: each label with the digest of what the
/// calls under it returned and counted.
pub(super) fn transcript_of(output: &str) -> Vec<(String, String)> {
    output
        .lines()
        .filter_map(|line| line.strip_prefix("transcript "))
        .filter_map(|line| line.rsplit_once(' '))
        .map(|(label, digest)| (label.to_owned(), digest.to_owned()))
        .collect()
}

/// The committed golden transcript: one file, whose first line names its
/// format and the SHA-256 of the lines after it, and whose other lines are a
/// section (`core`, `support`, or `std`), a label, and a 64-bit digest.
pub const GOLDEN_FILE: &str = "tests/rust-runtime-golden/transcript.txt";

const GOLDEN_FORMAT: &str = "lexlean/runtime-transcript/1";

/// The text of a golden file over `lines` (section, label, digest).
pub fn golden_text(lines: &[(String, String, String)]) -> String {
    use sha2::{Digest, Sha256};
    let body: String = lines
        .iter()
        .map(|(section, label, digest)| format!("{section} {label} {digest}\n"))
        .collect();
    format!(
        "{GOLDEN_FORMAT} sha256:{:x}\n{body}",
        Sha256::digest(body.as_bytes())
    )
}

/// The lines of a golden file, once its header names the SHA-256 of the rest.
///
/// # Errors
///
/// The header is not the format, its digest is not that of the lines, or a
/// line is not a section, a label, and a 16-digit hexadecimal digest.
pub fn parse_golden(text: &str) -> Result<Vec<(String, String, String)>, String> {
    let (header, body) = text.split_once('\n').ok_or("the golden file is empty")?;
    let lines = body
        .lines()
        .map(|line| {
            let (rest, digest) = line.rsplit_once(' ').ok_or("a line has no digest")?;
            let (section, label) = rest.split_once(' ').ok_or("a line has no label")?;
            if digest.len() != 16 || !digest.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err(format!("`{line}` does not end in a 64-bit digest"));
            }
            Ok((section.to_owned(), label.to_owned(), digest.to_owned()))
        })
        .collect::<Result<Vec<_>, String>>()?;
    if golden_text(&lines) != format!("{header}\n{body}") || !header.starts_with(GOLDEN_FORMAT) {
        return Err(
            "the golden file is not its header's digest of its lines; it is written by `cargo xtask check-golden --write` and never edited"
                .to_owned(),
        );
    }
    Ok(lines)
}

/// The golden transcript of `section`, one `label digest` per line.
pub(super) fn golden(section: &str) -> Vec<(String, String)> {
    let text = std::fs::read_to_string(repo_root().join(GOLDEN_FILE).as_std_path())
        .unwrap_or_else(|error| panic!("{GOLDEN_FILE}: {error}"));
    parse_golden(&text)
        .unwrap_or_else(|reason| panic!("{GOLDEN_FILE}: {reason}"))
        .into_iter()
        .filter(|(held, _, _)| held == section)
        .map(|(_, label, digest)| (label, digest))
        .collect()
}

/// The three transcripts of the printed runtime, as the lines of a golden file.
///
/// # Errors
///
/// A harness does not compile or run.
pub fn observed_lines() -> Result<Vec<(String, String, String)>, String> {
    let core = runtime::core()?;
    let std = runtime::std()?;
    let mut out = Vec::new();
    for (section, text, harness) in [
        ("core", core.to_owned(), harness()),
        ("support", core.to_owned(), super::rust_support::harness()),
        ("std", format!("{core}{std}"), super::rust_std::harness()),
    ] {
        let ran = run_harness(&text, harness)?;
        for (label, digest) in transcript_of(&ran) {
            out.push((section.to_owned(), label, digest));
        }
    }
    Ok(out)
}

/// `cargo xtask check-golden`: the committed runtime transcript is the one the
/// printed runtime gives, or (with `write`) is rewritten from it.
///
/// # Errors
///
/// A harness fails, or the file differs from the printed runtime's transcript.
pub fn check_golden(root: &std::path::Path, write: bool) -> Result<usize, String> {
    let lines = observed_lines()?;
    let text = golden_text(&lines);
    let path = root.join(GOLDEN_FILE);
    if write {
        std::fs::write(&path, &text).map_err(|error| error.to_string())?;
    } else if std::fs::read_to_string(&path).map_err(|error| error.to_string())? != text {
        return Err(format!(
            "R10: {GOLDEN_FILE} differs from the transcript of the printed runtime; run `cargo xtask check-golden --write` and review the diff"
        ));
    }
    Ok(lines.len())
}

/// The labels in which two transcripts differ, or `None` when they agree.
pub(super) fn disagreement(
    observed: &[(String, String)],
    expected: &[(String, String)],
) -> Option<String> {
    let mut labels: BTreeSet<&str> = BTreeSet::new();
    for (label, digest) in observed {
        if !expected.iter().any(|(l, d)| l == label && d == digest) {
            labels.insert(label);
        }
    }
    for (label, _) in expected {
        if !observed.iter().any(|(l, _)| l == label) {
            labels.insert(label);
        }
    }
    (!labels.is_empty()).then(|| {
        format!(
            "the transcript differs in: {}",
            labels.into_iter().collect::<Vec<_>>().join(", ")
        )
    })
}

/// Run `harness` on `runtime` and require its transcript to be the golden
/// one at `path`: the output of the run, or the reason it failed or differs.
pub(super) fn observe(runtime: &str, harness: String, path: &str) -> Result<String, String> {
    let ran = run_harness(runtime, harness)?;
    match disagreement(&transcript_of(&ran), &golden(path)) {
        Some(reason) => Err(reason),
        None => Ok(ran),
    }
}

/// Run the case of `RB-16`.
pub fn run(id: &str) {
    assert_eq!(id, "RB-16");
    let root = repo_root();
    let read = |path: &str| {
        std::fs::read_to_string(root.join(path).as_std_path())
            .unwrap_or_else(|error| panic!("{path}: {error}"))
    };
    let module = read(MODULE);
    let shipped = read(SHIPPED);
    // The shipped copy is the compiler project's module byte for byte: the
    // proofs are about one, the Rust crate reads the other.
    assert_eq!(module, shipped, "{SHIPPED} differs from {MODULE}");
    let support = runtime::support().expect("the embedded support");
    let decoded = source::decode(&shipped).expect("the module decodes");
    assert!(
        !decoded.core.items.is_empty(),
        "an empty runtime module defines nothing the differential could compare"
    );
    covered(&decoded.core.items).expect("the differential covers the module");
    assert_eq!(
        runtime::core().expect("the embedded module"),
        runtime::assemble(support, &decoded.core.items),
        "the embedded module is the committed one"
    );
    // The generated functions are the runtime's: the support holds none of
    // their names, so no function is declared twice, and the golden
    // transcript has a label for each.
    let recorded = golden(GOLDEN);
    for name in defined(&decoded.core.items).expect("named functions") {
        let (module, last) = name
            .rsplit_once("::")
            .map_or((None, name.as_str()), |(m, l)| (Some(m), l));
        if let Some(module) = module {
            assert!(
                !support.contains(&format!("pub mod {module} {{")),
                "the base still holds the module `{module}`"
            );
        } else {
            assert!(
                !support.contains(&format!("pub fn {last}(")),
                "the base still holds `{name}`"
            );
        }
        assert!(
            recorded.iter().any(|(label, _)| *label == name),
            "the golden transcript lacks `{name}`"
        );
    }
    let generated = runtime::assemble(support, &decoded.core.items);
    let ran = observe(&generated, harness(), GOLDEN)
        .expect("the generated runtime has the golden transcript");
    assert_eq!(ran.lines().last(), Some("runtime"));

    // A function whose body computes another result is caught by the
    // differential, not by the decoder: the checked product for the checked
    // sum, the checked difference for the checked sum of integers, the strict
    // order for the loose one, the remainder for the quotient.
    for (from, to, caught) in [
        ("Machine.checkedNatAdd", "Machine.checkedNatMul", "nat_add"),
        ("Machine.checkedIntAdd", "Machine.checkedIntSub", "int_add"),
        ("Machine.natLe", "Machine.natLt", "nat_le"),
        ("Machine.natQuot", "Machine.natRem", "nat_quot"),
        (
            "Machine.checkedAdd (ctor TargetSyntax::IntKind.u8)",
            "Machine.checkedSub (ctor TargetSyntax::IntKind.u8)",
            "fixed_u8::checked_add",
        ),
        (
            "Machine.checkedShl (ctor TargetSyntax::IntKind.i32)",
            "Machine.checkedShr (ctor TargetSyntax::IntKind.i32)",
            "fixed_i32::shift_left",
        ),
        (
            "Machine.bitXor (ctor TargetSyntax::IntKind.u16)",
            "Machine.bitOr (ctor TargetSyntax::IntKind.u16)",
            "fixed_u16::bit_xor",
        ),
    ] {
        let changed = shipped.replacen(from, to, 1);
        assert_ne!(changed, shipped, "the plant site exists");
        let items = source::decode(&changed)
            .expect("a changed body decodes")
            .core
            .items;
        let error = observe(&runtime::assemble(support, &items), harness(), GOLDEN)
            .expect_err("a changed body has another transcript");
        assert!(
            error
                .replace([' ', '\n'], "")
                .contains(&caught.replace(' ', "")),
            "{error}"
        );
    }

    // The runtime crate is checked by the validator that checks every
    // rendering, with the runtime's reading of its arithmetic rule: its
    // functions fail only with `Err(Overflow)`, and a core function allocates
    // nothing. The plants go through `core_from`, which is what `core()` runs,
    // so removing the call from it fails here.
    assert_eq!(
        runtime::core_from(&shipped).expect("the runtime is valid"),
        runtime::core().expect("the embedded module"),
        "core() is core_from of the embedded module"
    );
    let ok = "(ctor RustSyntax::Expr.succeed (ctor RustSyntax::Expr.copy (ctor RustSyntax::Ident.generated (ctor RustSyntax::IdentKind.binding) 0)))";
    let bare_ok = "(ctor RustSyntax::Expr.construct (ctor RustSyntax::Ctor.ok (ctor RustSyntax::RustType.nat) (ctor RustSyntax::RustType.overflow)) (cons (ctor RustSyntax::Expr.copy (ctor RustSyntax::Ident.generated (ctor RustSyntax::IdentKind.binding) 0)) (nil RustSyntax::Expr)))";
    for (what, from, to, message) in [
        (
            "an Err of a value that is not the overflow value",
            "(ctor RustSyntax::Expr.lit (ctor RustSyntax::Lit.overflow))",
            "(ctor RustSyntax::Expr.lit (ctor RustSyntax::Lit.nat 0))",
            "is not a fallible value",
        ),
        (
            "an Err whose error type is not the runtime's",
            "(ctor RustSyntax::RustType.nat) (ctor RustSyntax::RustType.overflow)",
            "(ctor RustSyntax::RustType.nat) (ctor RustSyntax::RustType.nat)",
            "is not a fallible value",
        ),
        (
            "a bare Ok of an arithmetic result in the Some arm every checked function shares",
            ok,
            bare_ok,
            "construct:ok, is not a fallible value",
        ),
        (
            "a heap type in the core runtime",
            "type := (ctor RustSyntax::RustType.nat)",
            "type := (ctor RustSyntax::RustType.str)",
            "hidden allocation",
        ),
    ] {
        let edited = shipped.replacen(from, to, 1);
        assert_ne!(edited, shipped, "{what}: the plant site exists");
        let error = runtime::core_from(&edited).expect_err(what);
        assert!(
            error.contains("the core runtime is not a valid crate") && error.contains(message),
            "{what}: {error}"
        );
    }

    // A module the decoder refuses, each for the rule it breaks.
    for (what, from, to, message) in [
        (
            "no core crate",
            "name: coreCrate",
            "name: otherCrate",
            "does not define `coreCrate`",
        ),
        (
            "a definition with a parameter",
            "name: natAddFunction,\n  parameters: []",
            "name: natAddFunction,\n  parameters: [extra : Nat]",
            "takes parameters",
        ),
        (
            "a reference to a definition the module lacks",
            "(cons (natAddFunction) ",
            "(cons (natAddFunctions) ",
            "which the module does not define",
        ),
        (
            "a machine primitive the machine lacks",
            "Machine.checkedNatAdd",
            "Machine.checkedNatAddd",
            "is not a machine primitive",
        ),
        (
            "a runtime function the runtime lacks",
            "ctor RustSyntax::Expr.lit (ctor RustSyntax::Lit.overflow)",
            "ctor RustSyntax::Expr.call (ctor RustSyntax::Callee.runtime (ctor RustSyntax::Item.natAddd)) (nil RustSyntax::Expr) false",
            "is not a runtime function",
        ),
        (
            "a constructor with another number of arguments",
            "(ctor RustSyntax::Expr.succeed (ctor RustSyntax::Expr.copy (ctor RustSyntax::Ident.generated (ctor RustSyntax::IdentKind.binding) 0)))",
            "(ctor RustSyntax::Expr.succeed (ctor RustSyntax::Expr.copy (ctor RustSyntax::Ident.generated (ctor RustSyntax::IdentKind.binding) 0)) (nil RustSyntax::Expr))",
            "takes 1 arguments",
        ),
    ] {
        let edited = shipped.replacen(from, to, 1);
        assert_ne!(edited, shipped, "{what}: the plant site exists");
        let error = source::decode(&edited).expect_err(what);
        assert!(error.contains(message), "{what}: {error}");
    }

    // A module that defines a function the differential does not cover is
    // refused by the case, whatever the function is.
    let start = shipped
        .find("definition{\n  name: natAddFunction,")
        .expect("the function's definition");
    let end = start + shipped[start..].find("\n}\n").expect("its end") + 3;
    let block = &shipped[start..end];
    let copy = block
        .replace("natAddFunction", "natOtherFunction")
        .replace("\"nat_add\"", "\"nat_other\"");
    let at = shipped.find("name: coreCrate").expect("the core crate");
    let (before, crate_text) = shipped.split_at(at);
    let crate_text = crate_text
        .replacen(
            "items := (cons (natAddFunction) ",
            "items := (cons (natOtherFunction) (cons (natAddFunction) ",
            1,
        )
        .replacen("})\n}\n", ")})\n}\n", 1);
    let extra = format!("{before}{crate_text}").replacen(block, &format!("{block}\n{copy}"), 1);
    let items = source::decode(&extra)
        .expect("a module with a second function decodes")
        .core
        .items;
    assert_eq!(
        items.len(),
        decoded.core.items.len() + 1,
        "the plant adds a function"
    );
    let error = covered(&items).expect_err("a function without a differential");
    assert!(error.contains("nat_other"), "{error}");
}
