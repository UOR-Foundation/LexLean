//! Conformance cases for the Rust backend (SPEC.md §17.16).

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use lexlean::calculus::rust::ast::{
    Block, Callee, CaptureRead, Crate, Expr, ItemDef, Let, Lit, Origin, Pat, Type,
};
use lexlean::calculus::rust::package::{self, Manifest, Passing};
use lexlean::calculus::rust::runtime::Item;
use lexlean::calculus::rust::{self, validate, Profile};
use lexlean::calculus::{self as target, realization, Outcome};
use serde_json::Value as Json;

use crate::calculus::cases;
use crate::rust_differential;
use crate::rust_harness::{self, Caller, Calls};
use crate::rust_packages::{self, Committed};
use crate::support::{self, repo_root};

fn schema(name: &str) -> Json {
    serde_json::from_slice(
        &std::fs::read(repo_root().join("schemas").join(name).as_std_path()).expect("schema"),
    )
    .expect("schema JSON")
}

/// Apply `edit` to the first expression, depth first, it accepts.
fn edit_first(krate: &mut Crate, edit: &mut dyn FnMut(&mut Expr) -> bool) -> bool {
    fn block(block: &mut Block, edit: &mut dyn FnMut(&mut Expr) -> bool) -> bool {
        block
            .lets
            .iter_mut()
            .any(|binding| expr(&mut binding.value, edit))
            || expr(&mut block.tail, edit)
    }
    fn expr(expr_: &mut Expr, edit: &mut dyn FnMut(&mut Expr) -> bool) -> bool {
        if edit(expr_) {
            return true;
        }
        match expr_ {
            Expr::Box(inner, _)
            | Expr::Widen(inner, _)
            | Expr::Succeed(inner, _)
            | Expr::Not(inner, _) => expr(inner, edit),
            Expr::Call { args, .. } | Expr::Apply { args, .. } | Expr::Construct { args, .. } => {
                args.iter_mut().any(|arg| expr(arg, edit))
            }
            Expr::Pair(left, right, _) => expr(left, edit) || expr(right, edit),
            Expr::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => expr(condition, edit) || block(then_branch, edit) || block(else_branch, edit),
            Expr::Match {
                scrutinee, arms, ..
            } => expr(scrutinee, edit) || arms.iter_mut().any(|(_, body)| block(body, edit)),
            Expr::Block(inner) => block(inner, edit),
            _ => false,
        }
    }
    krate.items.iter_mut().any(|item| match item {
        ItemDef::Function { body, .. } => {
            let mut whole = Expr::Block(Box::new(body.clone()));
            let edited = expr(&mut whole, edit);
            if let Expr::Block(edited_body) = whole {
                *body = *edited_body;
            }
            edited
        }
        _ => false,
    })
}

fn lowered(name: &str, profile: Profile) -> Crate {
    let case = cases()
        .into_iter()
        .find(|case| case.fixture.name == name)
        .unwrap_or_else(|| panic!("fixture {name}"));
    rust::lower(&case.fixture.program, profile).expect("lowers")
}

fn refused(krate: &Crate, message: &str) {
    let error = validate::validate(krate).expect_err("the planted crate is refused");
    assert!(error.contains(message), "{error}");
}

/// Every committed negative manifest whose name starts with `prefix` fails
/// with `LLB6005` and exactly its stated error.
fn negatives(prefix: &str) -> usize {
    let dir = repo_root().join("compiler/rust/negative");
    let mut count = 0;
    for (name, (bytes, error)) in rust_packages::negatives() {
        if !name.starts_with(prefix) {
            continue;
        }
        let committed = std::fs::read(dir.join(format!("{name}.json")).as_std_path())
            .expect("negative manifest");
        assert_eq!(
            committed, bytes,
            "{name}: the committed manifest is generated"
        );
        let stated = std::fs::read_to_string(dir.join(format!("{name}.error")).as_std_path())
            .expect("stated error");
        assert_eq!(stated.trim_end(), error, "{name}");
        let Err(failure) = target::package(&committed) else {
            panic!("{name}: the negative manifest packages");
        };
        support::expect_code(&failure, "LLB6005");
        assert!(
            failure
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.message.contains(&error)),
            "{name}: {failure:?}"
        );
        count += 1;
    }
    assert!(count > 0, "a negative manifest exercises `{prefix}`");
    count
}

fn expected_outcome(fixture: &str) -> (Outcome, Vec<target::Value>, u64) {
    let case = cases()
        .into_iter()
        .find(|case| case.fixture.name == fixture)
        .unwrap_or_else(|| panic!("fixture {fixture}"));
    (
        case.fixture.expected,
        case.fixture.arguments,
        case.fixture.entry,
    )
}

fn cargo() -> String {
    std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned())
}

/// A package to build: its crate name, its files, and the harness that
/// calls its export.
struct Built {
    name: String,
    files: BTreeMap<String, Vec<u8>>,
    harness: String,
}

/// Write `packages` and one harness crate each into a workspace at `dir`.
fn workspace(dir: &Path, packages: &[Built]) {
    let mut members = Vec::new();
    for Built {
        name,
        files,
        harness,
    } in packages
    {
        let package = dir.join("p").join(name);
        for (path, bytes) in files {
            let file = package.join(path);
            std::fs::create_dir_all(file.parent().expect("parent")).expect("package directory");
            std::fs::write(file, bytes).expect("package file");
        }
        let runner = dir.join("h").join(format!("run_{name}"));
        std::fs::create_dir_all(runner.join("src")).expect("harness directory");
        std::fs::write(
            runner.join("Cargo.toml"),
            format!(
                "[package]\nname = \"run_{name}\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\n{name} = {{ path = \"../../p/{name}\" }}\n"
            ),
        )
        .expect("harness manifest");
        std::fs::write(runner.join("src/main.rs"), harness).expect("harness source");
        members.push(format!("\"p/{name}\""));
        members.push(format!("\"h/run_{name}\""));
    }
    std::fs::write(
        dir.join("Cargo.toml"),
        format!(
            "[workspace]\nresolver = \"2\"\nmembers = [{}]\n",
            members.join(", ")
        ),
    )
    .expect("workspace manifest");
}

fn cargo_in(dir: &Path, arguments: &[&str]) -> std::process::Output {
    std::process::Command::new(cargo())
        .args(arguments)
        .current_dir(dir)
        .env("CARGO_TARGET_DIR", dir.join("target"))
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .output()
        .expect("cargo runs")
}

fn harness_for(committed: &Committed) -> String {
    let (_, arguments, entry) = expected_outcome(&committed.fixture);
    let case = cases()
        .into_iter()
        .find(|case| case.fixture.name == committed.fixture)
        .expect("fixture");
    rust_harness::render_caller(
        &case.fixture.program,
        committed.profile,
        entry,
        &arguments,
        &Caller {
            library: &committed.manifest.name,
            function: "run",
            passing: &committed.manifest.exports[0].parameters,
        },
    )
    .expect("harness")
}

fn committed_files(committed: &Committed) -> BTreeMap<String, Vec<u8>> {
    let dir = repo_root().join(format!(
        "compiler/rust/{}/{}",
        committed.profile.target(),
        committed.directory
    ));
    ["Cargo.toml", "src/lib.rs"]
        .into_iter()
        .map(|path| {
            (
                path.to_owned(),
                std::fs::read(dir.join(path).as_std_path()).expect("committed package file"),
            )
        })
        .collect()
}

/// Whether RB-06 runs a package of a fixture with this outcome: its value
/// is observable, or it overflows.
fn runs(outcome: &Outcome) -> bool {
    match outcome {
        Outcome::Value { value, .. } => {
            !realization::value_elements(value).contains("value:closure")
        }
        Outcome::Overflow { .. } => true,
        Outcome::Stuck | Outcome::Exhausted => false,
    }
}

/// Every node kind of the closed AST, capture read, and dispatch shape a
/// package must emit for its run to cover the renderer.
const AST_SHAPES: &[&str] = &[
    "expr:lit",
    "expr:move",
    "expr:clone",
    "expr:copy",
    "expr:deref",
    "expr:not",
    "expr:unbox",
    "expr:box",
    "expr:call",
    "expr:apply",
    "expr:construct",
    "expr:pair",
    "expr:if",
    "expr:match",
    "expr:match-empty",
    "expr:block",
    "expr:uncons",
    "expr:is-zero",
    "expr:non-zero",
    "expr:predecessor",
    "expr:widen",
    "expr:succeed",
    "lit:string-escaped",
    "capture:copy",
    "capture:clone",
    "capture:unbox",
    "capture:unit",
    "dispatch:several",
    "dispatch:mixed-failure",
    "dispatch:empty",
];

fn ast_shapes(krate: &Crate, out: &mut BTreeSet<&'static str>) {
    fn block(body: &Block, out: &mut BTreeSet<&'static str>) {
        for binding in &body.lets {
            expr(&binding.value, out);
        }
        expr(&body.tail, out);
    }
    fn expr(node: &Expr, out: &mut BTreeSet<&'static str>) {
        if let Expr::Lit(Lit::Str(text), _) = node {
            if text
                .chars()
                .any(|character| !(' '..='~').contains(&character) || "\\\"".contains(character))
            {
                out.insert("lit:string-escaped");
            }
        }
        out.insert(match node {
            Expr::Lit(..) => "expr:lit",
            Expr::Move(..) => "expr:move",
            Expr::Clone(..) => "expr:clone",
            Expr::Copy(..) => "expr:copy",
            Expr::Deref(..) => "expr:deref",
            Expr::Not(..) => "expr:not",
            Expr::Unbox(..) => "expr:unbox",
            Expr::Box(..) => "expr:box",
            Expr::Call { .. } => "expr:call",
            Expr::Apply { .. } => "expr:apply",
            Expr::Construct { .. } => "expr:construct",
            Expr::Pair(..) => "expr:pair",
            Expr::If { .. } => "expr:if",
            Expr::Match { arms, .. } if arms.is_empty() => "expr:match-empty",
            Expr::Match { .. } => "expr:match",
            Expr::Block(_) => "expr:block",
            Expr::Uncons(_) => "expr:uncons",
            Expr::IsZero(..) => "expr:is-zero",
            Expr::NonZero(..) => "expr:non-zero",
            Expr::Predecessor(_) => "expr:predecessor",
            Expr::Widen(..) => "expr:widen",
            Expr::Succeed(..) => "expr:succeed",
        });
        match node {
            Expr::Not(inner, _)
            | Expr::Box(inner, _)
            | Expr::Widen(inner, _)
            | Expr::Succeed(inner, _) => expr(inner, out),
            Expr::Call { args, .. } | Expr::Apply { args, .. } | Expr::Construct { args, .. } => {
                args.iter().for_each(|arg| expr(arg, out));
            }
            Expr::Pair(left, right, _) => {
                expr(left, out);
                expr(right, out);
            }
            Expr::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                expr(condition, out);
                block(then_branch, out);
                block(else_branch, out);
            }
            Expr::Match {
                scrutinee, arms, ..
            } => {
                expr(scrutinee, out);
                arms.iter().for_each(|(_, body)| block(body, out));
            }
            Expr::Block(body) => block(body, out),
            _ => {}
        }
    }
    for item in &krate.items {
        match item {
            ItemDef::Apply { arms, fallible, .. } => {
                if arms.is_empty() {
                    out.insert("dispatch:empty");
                }
                if arms.len() > 1 {
                    out.insert("dispatch:several");
                }
                if *fallible && arms.iter().any(|arm| !arm.function_fallible) {
                    out.insert("dispatch:mixed-failure");
                }
                for read in arms.iter().flat_map(|arm| &arm.captures) {
                    out.insert(match read {
                        CaptureRead::Copy => "capture:copy",
                        CaptureRead::Clone => "capture:clone",
                        CaptureRead::Unbox => "capture:unbox",
                        CaptureRead::Unit => "capture:unit",
                    });
                }
            }
            ItemDef::Function { body, .. } => block(body, out),
            ItemDef::Enum { .. } => {}
        }
    }
}

/// The environment variable that makes RB-07 a renderer writing every
/// package under the root it names.
const RENDER_INTO: &str = "LEXLEAN_RB07_RENDER_INTO";

/// Render every committed package's manifest into `root`, by its committed
/// relative path.
fn render_into(root: &Path) {
    for committed in rust_packages::packages() {
        let relative = format!("{}/{}", committed.profile.target(), committed.directory);
        let manifest = std::fs::read(
            repo_root()
                .join(format!("compiler/rust/{relative}/package.json"))
                .as_std_path(),
        )
        .expect("manifest");
        let package = target::package(&manifest).expect("packages");
        for (path, bytes) in package.files {
            let file = root.join(&relative).join(path);
            std::fs::create_dir_all(file.parent().expect("parent")).expect("directory");
            std::fs::write(file, bytes).expect("file");
        }
    }
}

/// Run this test again, in a separate process with its own working
/// directory and environment, as a renderer writing into a fresh root.
fn cross_root(tag: &str) -> tempfile::TempDir {
    let root = tempfile::Builder::new()
        .prefix(&format!("lexlean-rust-root-{tag}-"))
        .tempdir()
        .expect("tempdir");
    let work = root.path().join("work");
    let out = root.path().join("out");
    std::fs::create_dir_all(&work).expect("work directory");
    let ran = std::process::Command::new(std::env::current_exe().expect("this test"))
        .args([
            "--exact",
            "conformance_rb_07",
            "--test-threads=1",
            "--quiet",
        ])
        .current_dir(&work)
        .env(RENDER_INTO, &out)
        .env("TMPDIR", &work)
        .env("HOME", &work)
        .env("LC_ALL", if tag == "a" { "C" } else { "C.UTF-8" })
        .env("TZ", if tag == "a" { "UTC" } else { "Asia/Kolkata" })
        .output()
        .expect("the renderer runs");
    assert!(
        ran.status.success(),
        "the renderer under root {tag} failed: {}{}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    );
    let moved = tempfile::Builder::new()
        .prefix(&format!("lexlean-rust-rendered-{tag}-"))
        .tempdir()
        .expect("tempdir");
    for (path, bytes) in tree(&out) {
        let file = moved.path().join(path);
        std::fs::create_dir_all(file.parent().expect("parent")).expect("directory");
        std::fs::write(file, bytes).expect("file");
    }
    moved
}

/// Every file under `root`, by its `/`-separated relative path.
fn tree(root: &Path) -> BTreeMap<String, Vec<u8>> {
    walkdir::WalkDir::new(root)
        .into_iter()
        .flatten()
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| {
            let relative = entry
                .path()
                .strip_prefix(root)
                .expect("under the root")
                .to_string_lossy()
                .replace('\\', "/");
            (relative, std::fs::read(entry.path()).expect("file"))
        })
        .collect()
}

/// LexLean's language-1.2 compiler semantics (`language/semantics-1.2.toml`).
fn semantics_table() -> toml::Value {
    std::fs::read_to_string(
        repo_root()
            .join("language/semantics-1.2.toml")
            .as_std_path(),
    )
    .expect("semantics")
    .parse()
    .expect("TOML")
}

/// The SHA-256 of each profile's runtime that LexLean's compiler semantics
/// records (`language/semantics-1.2.toml`).
fn runtime_digests() -> BTreeMap<Profile, String> {
    let table = semantics_table();
    Profile::ALL
        .into_iter()
        .map(|profile| {
            let key = format!(
                "rust_runtime_{}",
                profile.target().trim_start_matches("rust-")
            );
            (
                profile,
                table[key.as_str()]
                    .as_str()
                    .expect("a runtime digest")
                    .to_owned(),
            )
        })
        .collect()
}

#[allow(clippy::too_many_lines)]
pub fn run(id: &str) {
    match id {
        // §17.16: every emitted construct corresponds to an element of the
        // program it realizes.
        "RB-01" => {
            let mut emitted = BTreeSet::new();
            let mut lowered_count = 0;
            for case in cases() {
                for profile in Profile::ALL {
                    match rust::lower(&case.fixture.program, profile) {
                        // Every rendering is checked; coverage is counted
                        // below, from the packages RB-06 runs.
                        Ok(_) => lowered_count += 1,
                        // Only rust-core refuses, and only for the heap.
                        Err(reason) => assert!(
                            profile == Profile::Core && reason.contains("requires heap allocation"),
                            "{} ({}): {reason}",
                            case.fixture.name,
                            profile.target()
                        ),
                    }
                }
            }
            // Every correspondence row, every node of the closed AST, every
            // capture read, and a dispatch of several closures of mixed
            // failure are emitted by a package that RB-06 runs.
            let mut shapes = BTreeSet::new();
            for committed in rust_packages::packages() {
                let (krate, _, _) = package::lower(&committed.manifest).expect("package lowers");
                let (outcome, _, _) = expected_outcome(&committed.fixture);
                if runs(&outcome) {
                    emitted.extend(validate::constructs(&krate));
                    ast_shapes(&krate, &mut shapes);
                }
            }
            let missing: Vec<&str> = AST_SHAPES
                .iter()
                .copied()
                .filter(|shape| !shapes.contains(*shape))
                .collect();
            assert!(
                missing.is_empty(),
                "AST shapes no run package emits: {missing:?}"
            );
            assert!(lowered_count > 100, "{lowered_count} renderings");
            let table: BTreeSet<String> = validate::CORRESPONDENCE
                .iter()
                .map(|(name, _)| (*name).to_owned())
                .collect();
            let unexercised: Vec<&String> = table.difference(&emitted).collect();
            assert!(
                unexercised.is_empty(),
                "correspondence rows no run package exercises: {unexercised:?}"
            );
            // A construct whose element the program does not use is refused.
            let program = |name: &str| {
                cases()
                    .into_iter()
                    .find(|case| case.fixture.name == name)
                    .unwrap_or_else(|| panic!("fixture {name}"))
                    .fixture
                    .program
            };
            let sum = program("sum-to");
            let krate = rust::lower(&sum, Profile::Std).expect("lowers");
            let mut elements = rust::realized(&sum, Profile::Std).expect("valid");
            validate::correspond(&krate, &elements).expect("the full element set justifies it");
            elements.remove("prim:nat_add");
            let error = validate::correspond(&krate, &elements).expect_err("unjustified");
            assert!(
                error.contains(
                    "the construct `call:runtime:nat_add` realizes `prim:nat_add`, which the program does not use"
                ),
                "{error}"
            );
            // Correspondence is checked instance by instance: a call of
            // `nat_mul` lowered from a `nat_add` term is refused even though
            // the program uses both primitives.
            let arithmetic = program("nat-arithmetic");
            let elements = rust::realized(&arithmetic, Profile::Std).expect("valid");
            assert!(elements.contains("prim:nat_add") && elements.contains("prim:nat_mul"));
            let mut krate = rust::lower(&arithmetic, Profile::Std).expect("lowers");
            let planted = edit_first(&mut krate, &mut |expr| {
                if let Expr::Call {
                    callee: Callee::Runtime(item @ Item::NatAdd),
                    ..
                } = expr
                {
                    *item = Item::NatMul;
                    return true;
                }
                false
            });
            assert!(planted, "the plant site exists");
            let error = validate::correspond(&krate, &elements).expect_err("swapped");
            assert!(
                error.contains(
                    "the construct `call:runtime:nat_mul` does not realize `prim:nat_add`, the element it was lowered from"
                ),
                "{error}"
            );
            // And at its width: a `u16` addition lowered from a `u8` one is
            // refused.
            let fixed = program("fixed-width-u8");
            let elements = rust::realized(&fixed, Profile::Core).expect("valid");
            let mut krate = rust::lower(&fixed, Profile::Core).expect("lowers");
            let planted = edit_first(&mut krate, &mut |expr| {
                if let Expr::Call {
                    callee: Callee::Runtime(item @ Item::CheckedAdd(_)),
                    ..
                } = expr
                {
                    *item = Item::CheckedAdd(lexlean::calculus::IntKind::U16);
                    return true;
                }
                false
            });
            assert!(planted, "the plant site exists");
            let error = validate::correspond(&krate, &elements).expect_err("widened");
            assert!(
                error.contains("the construct `call:runtime:checked_add` works at width Some(U16), but `prim:checked_add` is at Some(U8)"),
                "{error}"
            );
            // A type whose element the program lacks is refused even when
            // every other construct is justified.
            let krate = rust::lower(&sum, Profile::Std).expect("lowers");
            let elements = rust::realized(&sum, Profile::Std).expect("valid");
            let mut extended = krate.clone();
            extended.items.push(ItemDef::Function {
                name: lexlean::calculus::rust::ast::Ident::Function(99),
                parameters: Vec::new(),
                result: Type::Ref(Box::new(Type::Nat)),
                body: Block::of(Expr::Lit(Lit::Nat(0), Origin::of("value:nat"))),
                at: Origin::of("function"),
            });
            let error = validate::correspond(&extended, &elements).expect_err("no export");
            assert!(error.contains("`type:ref`"), "{error}");
            // A negated zero test is refused: it is written as its
            // complement, and `!m == 0` would negate `m` alone.
            let mut krate = rust::lower(&program("boolean-shapes"), Profile::Core).expect("lowers");
            let planted = edit_first(&mut krate, &mut |expr| {
                if let Expr::NonZero(ident, origin) = expr {
                    *expr = Expr::Not(
                        Box::new(Expr::IsZero(ident.clone(), origin.clone())),
                        origin.clone(),
                    );
                    return true;
                }
                false
            });
            assert!(planted, "the plant site exists");
            refused(&krate, "a negated zero test or predecessor");
        }
        // §17.16: identifiers are hygienic and an exported name never
        // collides.
        "RB-02" => {
            assert!(negatives("identifier-") >= 7);
            let mut krate = lowered("sum-to", Profile::Std);
            let duplicated = edit_first(&mut krate, &mut |expr| {
                if let Expr::Block(block) = expr {
                    if let Some(binding) = block
                        .lets
                        .iter()
                        .find(|binding| matches!(binding.pat, Pat::Bind(_)))
                        .cloned()
                    {
                        block.lets.push(binding);
                        return true;
                    }
                }
                false
            });
            assert!(duplicated, "the plant site exists");
            refused(&krate, "hygiene: ");
        }
        // §17.16: every value is moved at most once and never read after.
        "RB-03" => {
            assert!(negatives("ownership-") >= 1);
            for (fixture, message) in [
                ("adt-evaluation", "is moved twice"),
                ("sum-to", "is read after it was moved"),
            ] {
                let mut krate = lowered(fixture, Profile::Std);
                let planted = edit_first(&mut krate, &mut |expr| {
                    if let Expr::Block(block) = expr {
                        if let Some(Pat::Bind(holder)) = block
                            .lets
                            .first()
                            .map(|binding| binding.pat.clone())
                            .filter(|pattern| {
                                matches!(
                                    pattern,
                                    Pat::Bind(lexlean::calculus::rust::ast::Ident::Holder(_))
                                )
                            })
                        {
                            block.lets.push(Let {
                                pat: Pat::Wild,
                                ty: None,
                                value: Expr::Move(holder, Origin::of("expr:match")),
                                at: Origin::of("expr:match"),
                            });
                            return true;
                        }
                    }
                    false
                });
                assert!(planted, "{fixture}: the plant site exists");
                refused(&krate, message);
            }
        }
        // §17.16: no unsupported type crosses a package boundary and
        // rust-core never allocates.
        "RB-04" => {
            assert!(negatives("unsupported-") >= 3);
            assert!(negatives("hidden-allocation-") >= 2);
            let mut krate = lowered("sum-to", Profile::Core);
            krate.items.push(ItemDef::Enum {
                name: Type::Adt(0),
                variants: vec![(0, vec![Type::Str])],
                at: Origin::of("type:adt"),
            });
            refused(&krate, "hidden allocation: rust-core renders the heap type");
            let mut krate = lowered("nat-overflow-add", Profile::Core);
            let planted = edit_first(&mut krate, &mut |expr| {
                if let Expr::Call {
                    callee: Callee::Runtime(item),
                    ..
                } = expr
                {
                    *item = Item::LengthBytes;
                    return true;
                }
                false
            });
            assert!(planted, "the plant site exists");
            refused(
                &krate,
                "hidden allocation: rust-core renders the runtime function `length_bytes`",
            );
        }
        // §17.16: failure is typed exactly: every fallible call propagates,
        // nothing else does, and an export states its function's failure.
        "RB-05" => {
            assert!(negatives("arithmetic-") >= 2);
            for case in cases() {
                let fallible = rust::fallible_functions(&case.fixture.program).expect("valid");
                if matches!(case.fixture.expected, Outcome::Overflow { .. }) {
                    assert!(
                        fallible[usize::try_from(case.fixture.entry).expect("entry")],
                        "{}: an entry that overflows is typed fallible",
                        case.fixture.name
                    );
                }
            }
            let mut krate = lowered("sum-to", Profile::Std);
            let planted = edit_first(&mut krate, &mut |expr| {
                if let Expr::Call {
                    propagate: propagate @ true,
                    ..
                } = expr
                {
                    *propagate = false;
                    return true;
                }
                false
            });
            assert!(planted, "the plant site exists");
            refused(&krate, "arithmetic: the fallible call of");
            // A fallible function's tail must be a fallible value.
            let mut krate = lowered("sum-to", Profile::Std);
            let planted = edit_first(&mut krate, &mut |expr| {
                if let Expr::Call {
                    callee: Callee::Runtime(Item::NatAdd),
                    propagate: propagate @ false,
                    ..
                } = expr
                {
                    *propagate = true;
                    return true;
                }
                false
            });
            assert!(planted, "the plant site exists");
            refused(&krate, "is not a fallible value");
            let mut krate = lowered("booleans", Profile::Std);
            let planted = edit_first(&mut krate, &mut |expr| {
                if let Expr::Call { propagate, .. } = expr {
                    *propagate = true;
                    return true;
                }
                false
            });
            assert!(planted, "the plant site exists");
            refused(&krate, "arithmetic: the infallible call of");
            let mut krate = lowered("sum-to", Profile::Std);
            for item in &mut krate.items {
                if let ItemDef::Function { result, .. } = item {
                    if let Type::Fallible(inner) = result.clone() {
                        *result = *inner;
                    }
                }
            }
            refused(&krate, "arithmetic: ");
        }
        // §17.16: every package builds under its declared gates and its
        // export prints the denotation's observable outcome.
        "RB-06" => {
            let dir = tempfile::Builder::new()
                .prefix("lexlean-rust-packages-")
                .tempdir()
                .expect("tempdir");
            let mut built = Vec::new();
            let mut expected = BTreeMap::new();
            // Every committed package is built and linted; each one whose
            // fixture has an observable outcome is also run.
            for committed in rust_packages::packages() {
                let (outcome, _, _) = expected_outcome(&committed.fixture);
                built.push(Built {
                    name: committed.manifest.name.clone(),
                    files: committed_files(&committed),
                    harness: harness_for(&committed),
                });
                if runs(&outcome) {
                    let observed = rust::observable(&outcome).expect("observable");
                    expected.insert(committed.manifest.name.clone(), observed);
                }
            }
            assert_eq!(
                built.len(),
                rust_packages::packages().len(),
                "every committed package is built"
            );
            assert!(expected.len() > 100, "{} packages run", expected.len());
            // The primitive differential: every primitive instance each
            // profile admits, on its boundary values and seeded inputs.
            let mut differentials = BTreeMap::new();
            for profile in Profile::ALL {
                let differential = rust_differential::differential(profile);
                let manifest = rust_differential::manifest(&differential, profile);
                let package =
                    target::package(&manifest.to_file_bytes().expect("canonical manifest"))
                        .unwrap_or_else(|failure| panic!("{}: {failure:?}", manifest.name));
                let owned: Vec<Vec<Passing>> = differential
                    .program
                    .functions
                    .iter()
                    .map(|function| vec![Passing::Own; function.types.len()])
                    .collect();
                let calls: Vec<Calls<'_>> = differential
                    .arguments
                    .iter()
                    .enumerate()
                    .map(|(function, arguments)| Calls {
                        entry: function as u64,
                        function: &differential.names[function],
                        passing: &owned[function],
                        arguments,
                    })
                    .collect();
                let harness = rust_harness::render_calls(
                    &differential.program,
                    profile,
                    &manifest.name,
                    &calls,
                )
                .expect("differential harness");
                let runs: usize = differential.arguments.iter().map(Vec::len).sum();
                assert!(
                    differential.names.len() > 60 && runs > 5_000,
                    "{}: {} functions, {runs} runs",
                    manifest.name,
                    differential.names.len()
                );
                differentials.insert(
                    manifest.name.clone(),
                    rust_differential::expected(&differential),
                );
                built.push(Built {
                    name: manifest.name.clone(),
                    files: package.files,
                    harness,
                });
            }
            workspace(dir.path(), &built);
            let build = cargo_in(
                dir.path(),
                &["build", "--offline", "--workspace", "--quiet"],
            );
            assert!(
                build.status.success(),
                "a package does not build:\n{}",
                String::from_utf8_lossy(&build.stderr)
            );
            let lint = cargo_in(
                dir.path(),
                &[
                    "clippy",
                    "--offline",
                    "--workspace",
                    "--quiet",
                    "--exclude",
                    "run_*",
                ],
            );
            assert!(
                lint.status.success(),
                "a package fails its lint gate:\n{}",
                String::from_utf8_lossy(&lint.stderr)
            );
            for (name, observed) in &expected {
                let binary = dir
                    .path()
                    .join("target/debug")
                    .join(format!("run_{name}{}", std::env::consts::EXE_SUFFIX));
                let ran = std::process::Command::new(&binary)
                    .output()
                    .expect("the harness runs");
                assert!(
                    ran.status.success(),
                    "{name}: {}",
                    String::from_utf8_lossy(&ran.stderr)
                );
                let stdout = String::from_utf8(ran.stdout).expect("utf8");
                let printed: Json =
                    serde_json::from_str(stdout.lines().next().expect("outcome line"))
                        .unwrap_or_else(|error| panic!("{name}: {error}: {stdout}"));
                assert_eq!(
                    &printed, observed,
                    "{name}: the export and the denotation disagree"
                );
            }
            for (name, outcomes) in &differentials {
                let binary = dir
                    .path()
                    .join("target/debug")
                    .join(format!("run_{name}{}", std::env::consts::EXE_SUFFIX));
                let ran = std::process::Command::new(&binary)
                    .output()
                    .expect("the differential runs");
                assert!(
                    ran.status.success(),
                    "{name}: {}",
                    String::from_utf8_lossy(&ran.stderr)
                );
                let stdout = String::from_utf8(ran.stdout).expect("utf8");
                let printed: Vec<Json> = stdout
                    .lines()
                    .filter(|line| !line.starts_with("work "))
                    .map(|line| {
                        serde_json::from_str(line)
                            .unwrap_or_else(|error| panic!("{name}: {error}: {line}"))
                    })
                    .collect();
                assert_eq!(printed.len(), outcomes.len(), "{name}: one outcome per run");
                let disagreements: Vec<String> = printed
                    .iter()
                    .zip(outcomes)
                    .enumerate()
                    .filter(|(_, (printed, expected))| printed != expected)
                    .map(|(run, (printed, expected))| format!("run {run}: {printed} != {expected}"))
                    .collect();
                assert!(
                    disagreements.is_empty(),
                    "{name}: the rendering and the denotation disagree on {} of {} runs: {:?}",
                    disagreements.len(),
                    outcomes.len(),
                    &disagreements[..disagreements.len().min(8)]
                );
            }
            // The lint gate and the differential are real: a clone of a
            // `Copy` value fails Clippy, and a wrapping subtraction changes
            // the printed value.
            let planted = tempfile::Builder::new()
                .prefix("lexlean-rust-planted-")
                .tempdir()
                .expect("tempdir");
            let committed = rust_packages::packages()
                .into_iter()
                .find(|committed| {
                    committed.fixture == "nat-arithmetic" && committed.profile == Profile::Std
                })
                .expect("nat-arithmetic package");
            let mut lint_files = committed_files(&committed);
            lint_files
                .get_mut("src/lib.rs")
                .expect("library")
                .extend_from_slice(b"\npub fn planted(a: u64) -> u64 {\n    a.clone()\n}\n");
            let mut wrong_files = committed_files(&committed);
            let library = String::from_utf8(wrong_files["src/lib.rs"].clone()).expect("utf8");
            let from = "pub fn nat_sub(a: u64, b: u64) -> u64 { a.saturating_sub(b) }";
            assert!(library.contains(from), "the plant site exists");
            wrong_files.insert(
                "src/lib.rs".to_owned(),
                library
                    .replacen(
                        from,
                        "pub fn nat_sub(a: u64, b: u64) -> u64 { a.wrapping_sub(b) }",
                        1,
                    )
                    .into_bytes(),
            );
            let harness = harness_for(&committed);
            let mut lint_package = committed.manifest.name.clone();
            lint_package.push_str("_lint");
            let lint_harness = harness.replace(
                &format!("use {}::*;", committed.manifest.name),
                &format!("use {lint_package}::*;"),
            );
            let lint_cargo = String::from_utf8(lint_files["Cargo.toml"].clone())
                .expect("utf8")
                .replace(
                    &format!("name = \"{}\"", committed.manifest.name),
                    &format!("name = \"{lint_package}\""),
                );
            lint_files.insert("Cargo.toml".to_owned(), lint_cargo.into_bytes());
            workspace(
                planted.path(),
                &[
                    Built {
                        name: committed.manifest.name.clone(),
                        files: wrong_files,
                        harness,
                    },
                    Built {
                        name: lint_package.clone(),
                        files: lint_files,
                        harness: lint_harness,
                    },
                ],
            );
            let lint = cargo_in(
                planted.path(),
                &["clippy", "--offline", "--quiet", "-p", &lint_package],
            );
            assert!(!lint.status.success(), "the planted lint is refused");
            assert!(
                String::from_utf8_lossy(&lint.stderr).contains("clippy::clone_on_copy"),
                "{}",
                String::from_utf8_lossy(&lint.stderr)
            );
            let build = cargo_in(
                planted.path(),
                &[
                    "build",
                    "--offline",
                    "--quiet",
                    "-p",
                    &format!("run_{}", committed.manifest.name),
                ],
            );
            assert!(
                build.status.success(),
                "{}",
                String::from_utf8_lossy(&build.stderr)
            );
            let ran =
                std::process::Command::new(planted.path().join("target/debug").join(format!(
                    "run_{}{}",
                    committed.manifest.name,
                    std::env::consts::EXE_SUFFIX
                )))
                .output()
                .expect("the planted harness runs");
            let printed: Json = serde_json::from_str(
                String::from_utf8(ran.stdout)
                    .expect("utf8")
                    .lines()
                    .next()
                    .expect("outcome line"),
            )
            .expect("JSON");
            assert_ne!(
                &printed, &expected[&committed.manifest.name],
                "the planted subtraction is detected"
            );
        }
        // §17.16: packages are deterministic, schema-valid, and bound to
        // their program, runtime, and LexLean's compiler semantics.
        "RB-07" => {
            if let Ok(root) = std::env::var(RENDER_INTO) {
                // A child of the cross-root check: render every package and
                // write it under `root`, then stop.
                render_into(Path::new(&root));
                return;
            }
            let manifest_schema = schema("rust-package.schema.json");
            let provenance_schema = schema("rust-provenance.schema.json");
            let program_schema = schema("target-program.schema.json");
            let semantics = lexlean::compiler_semantics_id_for(lexlean::LANGUAGE_1_2).to_hex();
            let runtimes = runtime_digests();
            // The renderer's sources are the ones LexLean's semantics records.
            let renderer: Vec<(&str, Vec<u8>)> = rust::RENDERER_FILES
                .into_iter()
                .map(|name| {
                    (
                        name,
                        std::fs::read(
                            repo_root()
                                .join("crates/lexlean/src/calculus/rust")
                                .join(name)
                                .as_std_path(),
                        )
                        .expect("renderer source"),
                    )
                })
                .collect();
            let framed: Vec<(&str, &[u8])> = renderer
                .iter()
                .map(|(name, bytes)| (*name, bytes.as_slice()))
                .collect();
            assert_eq!(
                Some(rust::renderer_digest(&framed).to_hex().as_str()),
                semantics_table()["rust_renderer"].as_str(),
                "the renderer changed without its record `rust_renderer` in language/semantics-1.2.toml"
            );
            // Two renderers, each a separate process with its own working
            // directory, temporary directory, home, locale, and time zone,
            // render every package; their bytes are compared with each other
            // and with the committed packages.
            let roots = [cross_root("a"), cross_root("b")];
            let rendered: Vec<BTreeMap<String, Vec<u8>>> =
                roots.iter().map(|root| tree(root.path())).collect();
            assert_eq!(
                rendered[0].len(),
                rendered[1].len(),
                "both renderers write every package"
            );
            for (path, bytes) in &rendered[0] {
                assert_eq!(
                    Some(bytes),
                    rendered[1].get(path),
                    "{path}: the renderers of two roots disagree"
                );
            }
            let mut count = 0;
            for committed in rust_packages::packages() {
                let relative = format!("{}/{}", committed.profile.target(), committed.directory);
                // Messages name the repository-relative path, never the host's.
                let shown = format!("compiler/rust/{relative}");
                let directory = repo_root().join(&shown);
                let manifest_bytes =
                    std::fs::read(directory.join("package.json").as_std_path()).expect("manifest");
                let manifest_json: Json = serde_json::from_slice(&manifest_bytes).expect("JSON");
                let violations = crate::schema::validate(&manifest_schema, &manifest_json);
                assert!(violations.is_empty(), "{shown}: {violations:?}");
                let violations =
                    crate::schema::validate(&program_schema, &manifest_json["program"]);
                assert!(violations.is_empty(), "{shown}: {violations:?}");
                let manifest = Manifest::parse(&manifest_bytes).expect("manifest parses");
                let first = target::package(&manifest_bytes).expect("packages");
                for (path, bytes) in &first.files {
                    let committed_bytes =
                        std::fs::read(directory.join(path).as_std_path()).expect("committed file");
                    assert_eq!(&committed_bytes, bytes, "{shown}/{path}");
                    assert_eq!(
                        rendered[0].get(&format!("{relative}/{path}")),
                        Some(bytes),
                        "{shown}/{path}: the separate renderer disagrees"
                    );
                }
                let provenance: Json =
                    serde_json::from_slice(&first.files["provenance.json"]).expect("provenance");
                let violations = crate::schema::validate(&provenance_schema, &provenance);
                assert!(violations.is_empty(), "{shown}: {violations:?}");
                for path in ["Cargo.toml", "src/lib.rs"] {
                    assert_eq!(
                        provenance["files"][path],
                        lexlean::artifact::content_id::Sha256Digest::of(&first.files[path])
                            .to_hex(),
                        "{shown}/{path}"
                    );
                }
                assert_eq!(
                    provenance["program"],
                    manifest.program.id().expect("valid").to_hex(),
                    "{shown}"
                );
                assert_eq!(provenance["compiler_semantics"], semantics.as_str());
                assert_eq!(
                    provenance["runtime"],
                    runtimes[&committed.profile].as_str(),
                    "{shown}: the runtime is the one LexLean's semantics records"
                );
                assert_eq!(provenance["sources"], manifest_json["sources"]);
                rust_packages::bound_to_source(&committed)
                    .unwrap_or_else(|reason| panic!("{shown}: {reason}"));
                count += 1;
            }
            assert!(count > 100, "{count} packages");
            assert_eq!(
                rendered[0].len(),
                3 * count,
                "the separate renderer writes exactly the committed packages"
            );
            // A forged or zero source is refused by the binding check.
            let mut forged = rust_packages::packages().remove(0);
            for source in ["0".repeat(64), "f".repeat(64)] {
                forged.manifest.sources = vec![source.clone()];
                let error = rust_packages::bound_to_source(&forged).expect_err("forged");
                assert!(
                    error.contains("is not") || error.contains("are not"),
                    "{error}"
                );
                assert!(error.contains(&source), "{error}");
            }
            // The malformed, misversioned, and misordered manifests fail,
            // and the schema refuses the versions the renderer refuses.
            for prefix in ["version-", "sources-", "export-", "malformed-"] {
                assert!(negatives(prefix) >= 1);
            }
            for (name, (bytes, _)) in rust_packages::negatives() {
                if name.starts_with("version-") {
                    let json: Json = serde_json::from_slice(&bytes).expect("JSON");
                    let violations = crate::schema::validate(&manifest_schema, &json);
                    assert!(
                        violations
                            .iter()
                            .any(|violation| violation.to_string().starts_with("/version")),
                        "{name}: the schema admits the version: {violations:?}"
                    );
                }
            }
            // A provenance or package file that drifts from its generator is
            // refused by the generated-file gate.
            crate::calculus::check(repo_root().as_std_path(), false)
                .expect("the committed packages equal their generator");
        }
        other => panic!("no Rust backend case is wired for {other}"),
    }
}
