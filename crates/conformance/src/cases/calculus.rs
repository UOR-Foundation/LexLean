//! The `calculus` suite: TC-01..TC-07, the production realization calculus
//! (SPEC.md §17.14).

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use lexlean::calculus::library::Template;
use lexlean::calculus::{
    self as target, check, interp, realization, rust, Expr, Fixture, Outcome, Program, Value,
};
use serde_json::{json, Value as Json};

use crate::calculus::{self as fixtures, Case};
use crate::support::{self, repo_root, P};

/// Run every fixture program on seeded random well-typed arguments at
/// fuels up to the stated one, asserting no run is stuck; the number of
/// runs.
fn progress_runs(cases: &[Case]) -> usize {
    let mut seed = 0x9e37_79b9_7f4a_7c15_u64;
    let mut runs = 0;
    for case in cases {
        let fixture = &case.fixture;
        let entry = &fixture.program.functions[usize::try_from(fixture.entry).expect("entry")];
        for _ in 0..8 {
            let Some(arguments) = entry
                .types
                .iter()
                .map(|ty| random_value(&fixture.program, ty, &mut seed, 3))
                .collect::<Option<Vec<Value>>>()
            else {
                break;
            };
            check::check_arguments(&fixture.program, fixture.entry, &arguments)
                .expect("a generated argument is well typed");
            // Every seventh fuel up to the stated one, and no fewer than
            // six hundred of them for a fixture with a large fuel, so the
            // sample of a long search stays linear in its fuel.
            let stride = usize::try_from(fixture.fuel / 600).map_or(7, |each| each.max(7));
            for fuel in (0..=fixture.fuel).step_by(stride).chain([fixture.fuel]) {
                let outcome = interp::run(&fixture.program, fuel, fixture.entry, &arguments);
                assert_ne!(
                    outcome,
                    Outcome::Stuck,
                    "{}: stuck on {arguments:?} with fuel {fuel}",
                    fixture.name
                );
                runs += 1;
            }
        }
    }
    runs
}

/// The next value of a xorshift generator.
fn next(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}

/// A random value of `ty`, or `None` for a function type, which has no
/// literal. Numbers favour the edges of their range.
fn random_value(program: &Program, ty: &target::Ty, seed: &mut u64, depth: u32) -> Option<Value> {
    use target::Ty;
    let number = |seed: &mut u64, low: i128, high: i128| -> i128 {
        match next(seed) % 4 {
            0 => low,
            1 => high,
            _ => low + i128::from(next(seed) % 64).min(high - low),
        }
    };
    let pick = |seed: &mut u64, count: u64| next(seed) % count.max(1);
    Some(match ty {
        Ty::Unit => Value::Unit,
        Ty::Bool => Value::Bool {
            value: next(seed).is_multiple_of(2),
        },
        Ty::Nat => Value::Nat {
            value: number(seed, 0, i128::from(u64::MAX)).to_string(),
        },
        Ty::Int => Value::Int {
            value: number(seed, i128::from(i64::MIN), i128::from(i64::MAX)).to_string(),
        },
        Ty::Fixed { width } => {
            let (low, high) = width.range();
            let value = number(seed, low, high).to_string();
            serde_json::from_value(json!({"kind": width.name(), "value": value})).ok()?
        }
        Ty::String => Value::String {
            value: ["", "a", "a,b", "-12", "007", "é,", "9223372036854775808"]
                [usize::try_from(pick(seed, 7)).ok()?]
            .to_owned(),
        },
        Ty::Bytes => Value::Bytes {
            hex: ["", "00", "ff01", "c3a9", "c3"][usize::try_from(pick(seed, 5)).ok()?].to_owned(),
        },
        Ty::Ordering => {
            let order = ["lt", "eq", "gt"][usize::try_from(pick(seed, 3)).ok()?];
            serde_json::from_value(json!({"kind": "ordering", "value": order})).ok()?
        }
        Ty::Option { value } => {
            if depth == 0 || next(seed).is_multiple_of(3) {
                Value::None
            } else {
                Value::Some {
                    value: Box::new(random_value(program, value, seed, depth - 1)?),
                }
            }
        }
        Ty::Result { ok, error } => {
            if next(seed).is_multiple_of(2) {
                Value::Ok {
                    value: Box::new(random_value(program, ok, seed, depth.saturating_sub(1))?),
                }
            } else {
                Value::Error {
                    value: Box::new(random_value(program, error, seed, depth.saturating_sub(1))?),
                }
            }
        }
        Ty::List { element } => Value::List {
            items: (0..if depth == 0 { 0 } else { pick(seed, 5) })
                .map(|_| random_value(program, element, seed, depth - 1))
                .collect::<Option<_>>()?,
        },
        Ty::Pair { left, right } => Value::Pair {
            left: Box::new(random_value(program, left, seed, depth)?),
            right: Box::new(random_value(program, right, seed, depth)?),
        },
        Ty::Adt { index } => {
            let constructors = &program.adts[usize::try_from(*index).ok()?].constructors;
            // At depth zero, a constructor without fields if there is one.
            let choice = if depth == 0 {
                constructors.iter().position(Vec::is_empty).unwrap_or(0)
            } else {
                usize::try_from(pick(seed, constructors.len() as u64)).ok()?
            };
            Value::Adt {
                constructor: choice as u64,
                fields: constructors[choice]
                    .iter()
                    .map(|field| random_value(program, field, seed, depth.saturating_sub(1)))
                    .collect::<Option<_>>()?,
            }
        }
        Ty::Fn { .. } => return None,
    })
}

/// The committed fixtures, read from disk exactly as published.
fn committed() -> Vec<(PathBuf, Vec<u8>, Fixture)> {
    let dir = repo_root().join("compiler/fixtures");
    let mut out = Vec::new();
    for entry in std::fs::read_dir(dir.as_std_path()).expect("compiler/fixtures exists") {
        let path = entry.expect("entry").path();
        let bytes = std::fs::read(&path).expect("fixture bytes");
        let fixture: Fixture = serde_json::from_slice(&bytes)
            .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        out.push((path, bytes, fixture));
    }
    out.sort_by(|left, right| left.0.cmp(&right.0));
    assert!(
        !out.is_empty(),
        "the calculus has hand-constructed fixtures"
    );
    out
}

fn schema(name: &str) -> Json {
    serde_json::from_slice(
        &std::fs::read(repo_root().join("schemas").join(name).as_std_path()).expect("schema"),
    )
    .expect("schema parses")
}

/// Rename every local of every function by an injective map that differs
/// from first-binding order, keeping the program alpha-equivalent.
fn alpha_variant(program: &Program) -> Program {
    fn rename(expr: &mut Expr) {
        let fresh = |name: &mut u64| *name = *name * 7 + 1000;
        match expr {
            Expr::Value { .. } => {}
            Expr::Var { name } => fresh(name),
            Expr::Let {
                name, bound, body, ..
            } => {
                fresh(name);
                rename(bound);
                rename(body);
            }
            Expr::Cond {
                condition,
                then_branch,
                else_branch,
            } => {
                rename(condition);
                rename(then_branch);
                rename(else_branch);
            }
            Expr::Match {
                scrutinee, arms, ..
            } => {
                rename(scrutinee);
                for arm in arms {
                    arm.binders.iter_mut().for_each(fresh);
                    rename(&mut arm.body);
                }
            }
            Expr::Build { operands, .. }
            | Expr::Call { operands, .. }
            | Expr::Prim { operands, .. }
            | Expr::Closure {
                captures: operands, ..
            } => {
                operands.iter_mut().for_each(rename);
            }
            Expr::Apply { target, operands } => {
                rename(target);
                operands.iter_mut().for_each(rename);
            }
            Expr::First { value } | Expr::Second { value } | Expr::Field { value, .. } => {
                rename(value)
            }
        }
    }
    let mut out = program.clone();
    for function in &mut out.functions {
        function
            .parameters
            .iter_mut()
            .for_each(|name| *name = *name * 7 + 1000);
        rename(&mut function.body);
    }
    out
}

/// A load expected to fail closed with `LLB6005` naming `message`.
fn rejects(bytes: &[u8], message: &str) {
    let error = target::load(bytes)
        .err()
        .unwrap_or_else(|| panic!("an invalid program loads: expected `{message}`"));
    support::expect_code(&error, "LLB6005");
    assert_eq!(error.class.exit_code(), 1, "{error}");
    assert!(
        error.to_string().contains(message),
        "expected `{message}` in: {error}"
    );
}

fn program_json(name: &str) -> Json {
    let fixture = fixtures::cases()
        .into_iter()
        .find(|case| case.fixture.name == name)
        .expect("fixture")
        .fixture;
    serde_json::to_value(&fixture.program).expect("program serializes")
}

/// The Lean evaluator's outcome for every fixture: the generated sources
/// that verification published are compiled again by the same pinned Lean
/// (the published oleans carry no compiled code), and `#eval` runs them.
fn lean_evaluations(verified: &camino::Utf8Path, cases: &[Case]) -> BTreeMap<String, Json> {
    let toolchain = support::real_elan_home()
        .join("toolchains")
        .join(support::mangled_toolchain_name());
    let lean = toolchain.join("bin").join("lean");
    let scratch = tempfile::Builder::new()
        .prefix("lexlean-calculus-eval-")
        .tempdir()
        .expect("tempdir");
    let mut source = String::from(EVALUATOR);
    for case in cases {
        source.push_str(&format!(
            "#eval IO.println (\"FIXTURE {} \" ++ showOutcome Compiler.{}.{}Run)\n",
            case.fixture.name,
            fixtures::fixture_module(&case.fixture.name),
            fixtures::identifier(&case.fixture.name)
        ));
    }
    let file = scratch.path().join("Evaluate.lean");
    std::fs::write(&file, source).expect("evaluator source");
    let library = toolchain.join("lib").join("lean");
    // The verified tree publishes oleans without compiled IR, which `#eval`
    // needs, so the published generated sources are compiled again here by
    // the same pinned Lean, in import order.
    let compiled = scratch.path().join("out");
    std::fs::create_dir_all(compiled.join("Compiler")).expect("output directory");
    let search = std::env::join_paths([compiled.clone(), library]).expect("LEAN_PATH");
    for module in [
        "TargetSyntax",
        "TargetSemantics",
        "TargetOracle",
        "ReasoningOracle",
        "GradeOracle",
        "BudgetOracle",
        "PlannerOracle",
        "ScreeningOracle",
        "TargetFixtures",
        "ReasoningFixtures",
    ] {
        let built = std::process::Command::new(&lean)
            .arg("-o")
            .arg(compiled.join("Compiler").join(format!("{module}.olean")))
            .arg(format!("Compiler/{module}.lean"))
            // Lean names a module by its path relative to the working
            // directory.
            .current_dir(verified.join("modules").as_std_path())
            .env("LEAN_PATH", &search)
            .output()
            .expect("pinned lean runs");
        assert!(
            built.status.success(),
            "{module} does not compile:\n{}\n{}",
            String::from_utf8_lossy(&built.stdout),
            String::from_utf8_lossy(&built.stderr)
        );
    }
    let output = std::process::Command::new(&lean)
        .arg(&file)
        .env("LEAN_PATH", search)
        .output()
        .expect("pinned lean runs");
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(
        output.status.success(),
        "the evaluator fails:\n{stdout}\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut out = BTreeMap::new();
    for line in stdout.lines() {
        let rest = line
            .strip_prefix("FIXTURE ")
            .unwrap_or_else(|| panic!("unexpected evaluator output: {line}"));
        let (name, json) = rest.split_once(' ').expect("name and outcome");
        out.insert(
            name.to_owned(),
            serde_json::from_str(json).unwrap_or_else(|error| panic!("{name}: {error}: {json}")),
        );
    }
    out
}

/// Every fixture whose outcome under Lean's evaluator differs from the
/// outcome it states, with both.
fn evaluator_disagreements(evaluated: &BTreeMap<String, Json>, cases: &[Case]) -> Vec<String> {
    cases
        .iter()
        .filter_map(|case| {
            let stated = serde_json::to_value(&case.fixture.expected).expect("outcome");
            let observed = evaluated.get(&case.fixture.name);
            (observed != Some(&stated)).then(|| {
                format!(
                    "{}: Lean evaluates {observed:?}, the fixture states {stated}",
                    case.fixture.name
                )
            })
        })
        .collect()
}

/// Prints a denotation outcome in the fixtures' exact JSON form.
const EVALUATOR: &str = r#"import Compiler.TargetFixtures
import Compiler.ReasoningFixtures
open Compiler.TargetSyntax Compiler.TargetSemantics

def hexDigit (n : Nat) : Char := if n < 10 then Char.ofNat (48 + n) else Char.ofNat (87 + n)
def byteHex (b : UInt8) : String := String.ofList [hexDigit (b.toNat / 16), hexDigit (b.toNat % 16)]
def quote (s : String) : String :=
  "\"" ++ String.join (s.toList.map fun c =>
    if c = '"' then "\\\"" else if c = '\\' then "\\\\"
    else if c.toNat < 32 then "\\u00" ++ byteHex c.toNat.toUInt8 else c.toString) ++ "\""
def tagged (kind : String) (rest : String) : String := "{\"kind\":\"" ++ kind ++ "\"" ++ rest ++ "}"
def number (kind : String) (text : String) : String := tagged kind (",\"value\":\"" ++ text ++ "\"")
partial def showValue : Value → String
  | .unit => tagged "unit" ""
  | .bool b => tagged "bool" (",\"value\":" ++ toString b)
  | .nat n => number "nat" (toString n)
  | .int n => number "int" (toString n)
  | .u8 n => number "u8" (toString n)
  | .u16 n => number "u16" (toString n)
  | .u32 n => number "u32" (toString n)
  | .u64 n => number "u64" (toString n)
  | .i8 n => number "i8" (toString n)
  | .i16 n => number "i16" (toString n)
  | .i32 n => number "i32" (toString n)
  | .i64 n => number "i64" (toString n)
  | .string s => tagged "string" (",\"value\":" ++ quote s)
  | .bytes b => tagged "bytes" (",\"hex\":\"" ++ String.join (b.toList.map byteHex) ++ "\"")
  | .ordering o => tagged "ordering" (",\"value\":\"" ++ (match o with | .less => "lt" | .same => "eq" | .more => "gt") ++ "\"")
  | .none => tagged "none" ""
  | .some v => tagged "some" (",\"value\":" ++ showValue v)
  | .ok v => tagged "ok" (",\"value\":" ++ showValue v)
  | .error v => tagged "error" (",\"value\":" ++ showValue v)
  | .list items => tagged "list" (",\"items\":[" ++ ",".intercalate (items.map showValue) ++ "]")
  | .pair l r => tagged "pair" (",\"left\":" ++ showValue l ++ ",\"right\":" ++ showValue r)
  | .adt c fields => tagged "adt" (",\"constructor\":" ++ toString c ++ ",\"fields\":[" ++ ",".intercalate (fields.map showValue) ++ "]")
  | .closure f captures => tagged "closure" (",\"function\":" ++ toString f ++ ",\"captures\":[" ++ ",".intercalate (captures.map showValue) ++ "]")
def showOutcome : Outcome → String
  | .value v steps => tagged "value" (",\"value\":" ++ showValue v ++ ",\"steps\":" ++ toString steps)
  | .overflow steps => tagged "overflow" (",\"steps\":" ++ toString steps)
  | .stuck => tagged "stuck" ""
  | .exhausted => tagged "exhausted" ""
"#;

/// The planted verification: a copy of the compiler whose fixture module
/// states one wrong expected outcome and runs one mutated library instance
/// against LexLean's own primitive. Pinned Lean must reject both.
fn planted_verification() -> &'static Vec<String> {
    static ERROR: OnceLock<Vec<String>> = OnceLock::new();
    ERROR.get_or_init(|| {
        let named = |name: &str| {
            fixtures::cases()
                .into_iter()
                .find(|case| case.fixture.name == name)
                .expect("fixture")
        };
        // The interpreter and the denotation disagree: 56 is not the sum.
        let mut wrong_sum = named("sum-to");
        let Outcome::Value { steps, .. } = wrong_sum.fixture.expected.clone() else {
            panic!("sum-to is a value")
        };
        wrong_sum.fixture.expected = Outcome::Value {
            value: Value::Nat {
                value: "56".to_owned(),
            },
            steps,
        };
        // The realization inserts a smaller key after the element it
        // precedes; the interpreter follows the mutation, LexLean does not.
        let mut insert = named("set-insert-nat");
        let entry = usize::try_from(insert.libraries.first().expect("a library fixture").at)
            .expect("index");
        insert.fixture.program = mutate_insert(&insert.fixture.program, entry);
        insert.fixture.expected = interp::run(
            &insert.fixture.program,
            insert.fixture.fuel,
            insert.fixture.entry,
            &insert.fixture.arguments,
        );
        let project = P::compiler();
        project.write(
            "src/TargetFixtures.lex.tex",
            &fixtures::fixtures_module(&[wrong_sum, insert]),
        );
        let _guard = support::env_lock();
        let error = project.verify_fails_with("LLV7002");
        error
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.message.clone())
            .collect()
    })
}

/// Insert a smaller key one position too late: `lt => head :: key :: tail`
/// instead of `key :: head :: tail`.
fn mutate_insert(program: &Program, entry: usize) -> Program {
    let mut out = program.clone();
    let Expr::Match { arms, .. } = &mut out.functions[entry].body else {
        panic!("set_insert is a match")
    };
    let Expr::Match { arms: orders, .. } = &mut arms[1].body else {
        panic!("the cons arm compares")
    };
    let Expr::Build { operands, .. } = &mut orders[0].body else {
        panic!("the lt arm builds")
    };
    let ty = target::Ty::List {
        element: Box::new(target::Ty::Nat),
    };
    *operands = vec![
        Expr::Var { name: 2 },
        Expr::Build {
            shape: target::Shape::Cons,
            ty,
            operands: vec![Expr::Var { name: 1 }, Expr::Var { name: 3 }],
        },
    ];
    check::check(&out).expect("the mutated realization is still well typed");
    out
}

/// The rustc the toolchain file pins (`RUSTC-1-97-1`).
const RUSTC_VERSION: &str = "rustc 1.97.1 (8bab26f4f 2026-07-14)";

fn rustc() -> String {
    std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_owned())
}

fn rustc_version() -> String {
    let output = std::process::Command::new(rustc())
        .arg("--version")
        .output()
        .expect("rustc runs");
    String::from_utf8(output.stdout)
        .expect("utf8")
        .trim()
        .to_owned()
}

/// A rendered fixture: its library crate, its harness, and what the
/// denotation says it prints and how many steps it takes.
#[derive(Clone)]
struct Job {
    name: String,
    library: String,
    harness: String,
    observed: Json,
    steps: u64,
}

/// Compile a job's library crate and harness with warnings denied, run the
/// harness, and return the value it prints and the work it counted.
fn run_rust(job: &Job, dir: &Path) -> (Json, u64) {
    let library = dir.join(format!("{}_library.rs", job.name));
    let rlib = dir.join(format!("lib{}.rlib", job.name));
    let harness = dir.join(format!("{}_main.rs", job.name));
    let binary = dir.join(format!("{}{}", job.name, std::env::consts::EXE_SUFFIX));
    std::fs::write(&library, &job.library).expect("library source");
    std::fs::write(&harness, &job.harness).expect("harness source");
    let compile = |arguments: &[&std::ffi::OsStr], what: &str| {
        let compiled = std::process::Command::new(rustc())
            .args(["--edition", "2021", "-D", "warnings", "-C", "opt-level=0"])
            .args(arguments)
            .output()
            .expect("rustc runs");
        assert!(
            compiled.status.success(),
            "{}: rustc rejects the {what}:\n{}",
            job.name,
            String::from_utf8_lossy(&compiled.stderr)
        );
    };
    compile(
        &[
            "--crate-type".as_ref(),
            "rlib".as_ref(),
            "--crate-name".as_ref(),
            "program".as_ref(),
            "-o".as_ref(),
            rlib.as_os_str(),
            library.as_os_str(),
        ],
        "library",
    );
    let extern_program = format!("program={}", rlib.display());
    compile(
        &[
            "--crate-name".as_ref(),
            "harness".as_ref(),
            "--extern".as_ref(),
            extern_program.as_ref(),
            "-o".as_ref(),
            binary.as_os_str(),
            harness.as_os_str(),
        ],
        "harness",
    );
    let ran = std::process::Command::new(&binary)
        .output()
        .expect("the rendered program runs");
    assert!(
        ran.status.success(),
        "{}: the rendered program fails:\n{}",
        job.name,
        String::from_utf8_lossy(&ran.stderr)
    );
    let stdout = String::from_utf8(ran.stdout).expect("utf8");
    let mut lines = stdout.lines();
    let printed = lines.next().expect("the outcome line");
    let work = lines
        .next()
        .and_then(|line| line.strip_prefix("work "))
        .and_then(|count| count.parse().ok())
        .unwrap_or_else(|| panic!("{}: no work line in {stdout}", job.name));
    (
        serde_json::from_str(printed)
            .unwrap_or_else(|error| panic!("{}: {error}: {printed}", job.name)),
        work,
    )
}

/// Run every job, eight at a time.
fn run_jobs(jobs: &[Job], dir: &Path) -> BTreeMap<String, (Json, u64)> {
    std::thread::scope(|scope| {
        let handles: Vec<_> = jobs
            .chunks(jobs.len().div_ceil(8).max(1))
            .map(|chunk| {
                scope.spawn(move || {
                    chunk
                        .iter()
                        .map(|job| (job.name.clone(), run_rust(job, dir)))
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|handle| handle.join().expect("thread"))
            .collect()
    })
}

/// Whether a value holds a closure, which Rust does not observe.
fn observable(value: &Value) -> bool {
    !realization::value_elements(value).contains("value:closure")
}

pub fn run(id: &str) {
    match id {
        // §17.14: closed form, canonical bytes, content identity, alpha.
        "TC-01" => {
            let program_schema = schema("target-program.schema.json");
            let fixture_schema = schema("target-fixture.schema.json");
            let all = committed();
            let mut renamed = 0;
            for (path, bytes, fixture) in &all {
                let (bytes, fixture) = (bytes.clone(), fixture.clone());
                let name = path
                    .file_stem()
                    .expect("stem")
                    .to_string_lossy()
                    .into_owned();
                assert_eq!(fixture.name, name, "a fixture is named by its file");
                assert_eq!(fixture.spec, target::FIXTURE_SPEC);
                assert_eq!(
                    bytes,
                    fixture.to_file_bytes(),
                    "{name}: committed bytes are canonical"
                );
                let json: Json = serde_json::from_slice(&bytes).expect("json");
                let violations = crate::schema::validate(&fixture_schema, &json);
                assert!(violations.is_empty(), "{name}: {violations:?}");
                let violations = crate::schema::validate(&program_schema, &json["program"]);
                assert!(violations.is_empty(), "{name}: {violations:?}");
                let canonical = fixture.program.canonical().expect("bound");
                assert_eq!(
                    canonical, fixture.program,
                    "{name}: the committed program is in first-binding order"
                );
                let loaded = target::load(&fixture.program.to_file_bytes())
                    .expect("a fixture program loads");
                assert_eq!(loaded.to_file_bytes(), fixture.program.to_file_bytes());
                // Alpha-equivalent spellings canonicalize to the same bytes
                // and identity, and loading canonicalizes them.
                let variant = alpha_variant(&fixture.program);
                renamed += usize::from(variant != fixture.program);
                assert_eq!(
                    variant.canonical().expect("bound").to_file_bytes(),
                    fixture.program.to_file_bytes(),
                    "{name}"
                );
                assert_eq!(
                    variant.id().expect("bound"),
                    fixture.program.id().expect("bound"),
                    "{name}"
                );
                assert_eq!(
                    target::load(&variant.to_file_bytes())
                        .expect("loads")
                        .to_file_bytes(),
                    fixture.program.to_file_bytes()
                );
                // The denotation's reference transcription is deterministic.
                let first = interp::run(
                    &fixture.program,
                    fixture.fuel,
                    fixture.entry,
                    &fixture.arguments,
                );
                let second = interp::run(&variant, fixture.fuel, fixture.entry, &fixture.arguments);
                assert_eq!(
                    first, fixture.expected,
                    "{name}: the expected outcome is the interpreter's"
                );
                assert_eq!(
                    second, first,
                    "{name}: an alpha variant has the same outcome"
                );
            }
            assert!(
                renamed * 2 > all.len(),
                "most fixtures bind locals, so renaming is exercised ({renamed})"
            );
            // Schemas are self-contained, so the fixture schema carries the
            // program schema's definitions; they may not drift apart.
            assert_eq!(
                fixture_schema["$defs"], program_schema["$defs"],
                "the fixture and program schemas define the calculus identically"
            );
            // The schemas refuse what the closed form refuses.
            let (_, _, sample) = &all[0];
            let sample = serde_json::to_value(sample).expect("fixture");
            for (what, edit) in [
                (
                    "an unknown expression kind",
                    &(|json: &mut Json| {
                        json["program"]["functions"][0]["body"] = json!({"kind": "loop"})
                    }) as &dyn Fn(&mut Json),
                ),
                ("a missing step count", &|json: &mut Json| {
                    json["expected"] = json!({"kind": "value", "value": {"kind": "unit"}})
                }),
                ("an unknown width", &|json: &mut Json| {
                    json["program"]["functions"][0]["result"] =
                        json!({"kind": "fixed", "width": "u128"})
                }),
                ("an extra member", &|json: &mut Json| {
                    json["extra"] = json!(true)
                }),
                ("a negative fuel", &|json: &mut Json| {
                    json["fuel"] = json!(-1)
                }),
            ] {
                let mut json = sample.clone();
                edit(&mut json);
                assert!(
                    !crate::schema::validate(&fixture_schema, &json).is_empty(),
                    "the fixture schema accepts {what}"
                );
            }
            // A change that is not a renaming changes the identity.
            let program: Program =
                serde_json::from_value(program_json("closures")).expect("program");
            let mut swapped = program.clone();
            swapped.functions[2].body = Expr::Prim {
                operation: target::Prim::NatAdd,
                operands: vec![Expr::Var { name: 0 }, Expr::Var { name: 1 }],
            };
            assert_ne!(swapped.id().expect("bound"), program.id().expect("bound"));
            // An ill-typed program has neither a canonical form nor an
            // identity.
            let mut ill_typed = program.clone();
            ill_typed.functions[2].body = Expr::Prim {
                operation: target::Prim::IntNeg,
                operands: vec![Expr::Var { name: 0 }],
            };
            for error in [
                ill_typed
                    .canonical()
                    .map(|_| ())
                    .expect_err("no canonical form"),
                ill_typed.id().map(|_| ()).expect_err("no identity"),
            ] {
                assert!(error.contains("primitive IntNeg"), "{error}");
            }
            // Identity is the SHA-256 of the canonical bytes.
            assert_eq!(
                program.id().expect("bound"),
                lexlean::artifact::content_id::Sha256Digest::of(&program.to_file_bytes())
            );
        }
        // §17.14: every invalid target construct fails closed.
        "TC-02" => {
            let base = program_json("adt-evaluation");
            let bytes = |json: &Json| serde_json::to_vec(json).expect("json");
            let mutate = |edit: &dyn Fn(&mut Json)| {
                let mut json = base.clone();
                edit(&mut json);
                bytes(&json)
            };
            rejects(b"{", "target program is malformed");
            rejects(
                &mutate(&|json| json["extra"] = json!(1)),
                "unknown field `extra`",
            );
            rejects(
                &mutate(&|json| json["spec"] = json!("lexlean/target-program/9")),
                "has spec `lexlean/target-program/9`",
            );
            rejects(
                &mutate(&|json| json["functions"][0]["body"] = json!({"kind": "var", "name": 99})),
                "function 0: local 99 is unbound",
            );
            rejects(
                &mutate(
                    &|json| json["functions"][1]["body"] = json!({"kind": "value", "type": {"kind": "bool"}, "value": {"kind": "bool", "value": true}}),
                ),
                "function 1: the body has type Bool, expected Nat",
            );
            rejects(
                &mutate(&|json| {
                    json["functions"][0]["body"]["arms"]
                        .as_array_mut()
                        .expect("arms")
                        .pop();
                }),
                "the match is not exhaustive: Adt { constructor: 2 } is not covered",
            );
            rejects(
                &mutate(&|json| {
                    let arm = json["functions"][0]["body"]["arms"][0].clone();
                    json["functions"][0]["body"]["arms"]
                        .as_array_mut()
                        .expect("arms")
                        .push(arm);
                }),
                "the arm for Adt { constructor: 0 } is unreachable",
            );
            rejects(
                &mutate(&|json| {
                    json["functions"][1]["body"]["operands"]
                        .as_array_mut()
                        .expect("operands")
                        .push(json!({"kind": "var", "name": 0}));
                }),
                "function 0 takes 1 operand(s), received 2",
            );
            rejects(
                &mutate(&|json| json["functions"][1]["body"]["function"] = json!(9)),
                "function 9 is not declared",
            );
            rejects(
                &mutate(&|json| {
                    json["functions"][0]["types"][0] = json!({"kind": "adt", "index": 9})
                }),
                "ADT 9 is not declared",
            );
            rejects(
                &mutate(&|json| {
                    json["adts"]
                        .as_array_mut()
                        .expect("adts")
                        .push(json!({"constructors": []}))
                }),
                "ADT 2 has no constructor",
            );
            rejects(
                &mutate(&|json| {
                    json["functions"][0]["parameters"] = json!([0, 0]);
                    json["functions"][0]["types"]
                        .as_array_mut()
                        .expect("types")
                        .push(json!({"kind": "nat"}));
                }),
                "a parameter name repeats",
            );
            rejects(
                &mutate(&|json| {
                    json["functions"][0]["body"]["arms"][2]["body"]["operands"][0]["operands"][0] =
                        json!({"kind": "field", "value": {"kind": "var", "name": 5}, "index": 0})
                }),
                "`field` requires an ADT with exactly one constructor; ADT 0 has 3",
            );
            // Literals are in range and spelled canonically.
            let literal = |ty: Json, value: Json| {
                bytes(
                    &json!({"spec": target::PROGRAM_SPEC, "adts": [], "functions": [
                        {"parameters": [], "types": [], "result": ty, "body": {"kind": "value", "type": ty, "value": value}}
                    ]}),
                )
            };
            rejects(
                &literal(
                    json!({"kind": "fixed", "width": "u8"}),
                    json!({"kind": "u8", "value": "256"}),
                ),
                "256 is outside u8",
            );
            rejects(
                &literal(
                    json!({"kind": "nat"}),
                    json!({"kind": "nat", "value": "007"}),
                ),
                "`007` is not a canonical decimal",
            );
            rejects(
                &literal(
                    json!({"kind": "int"}),
                    json!({"kind": "int", "value": "-0"}),
                ),
                "`-0` is not a canonical decimal",
            );
            rejects(
                &literal(
                    json!({"kind": "nat"}),
                    json!({"kind": "nat", "value": "18446744073709551616"}),
                ),
                "outside the 64-bit realization",
            );
            rejects(
                &literal(
                    json!({"kind": "bytes"}),
                    json!({"kind": "bytes", "hex": "abc"}),
                ),
                "byte literal `abc` is not even-length lowercase hexadecimal",
            );
            rejects(
                &literal(json!({"kind": "nat"}), json!({"kind": "int", "value": "1"})),
                "does not have type Nat",
            );
            rejects(
                &literal(
                    json!({"kind": "fn", "parameters": [], "result": {"kind": "nat"}}),
                    json!({"kind": "unit"}),
                ),
                "a function type takes at least one parameter",
            );
            rejects(
                &literal(
                    json!({"kind": "nat"}),
                    json!({"kind": "closure", "function": 0, "captures": []}),
                ),
                "a closure has no literal form",
            );
            // Closures leave a parameter; key order exists only on keys;
            // primitives apply only at their closed signatures.
            let closures = program_json("closures");
            let mut capture_all = closures.clone();
            capture_all["functions"][0]["body"]["operands"][0]["captures"]
                .as_array_mut()
                .expect("captures")
                .push(json!({"kind": "var", "name": 0}));
            rejects(
                &bytes(&capture_all),
                "closure of function 2 captures 2 of its 2 parameters; at least one must remain",
            );
            let unary = |operation: Json, operand: Json, operand_ty: Json, result: Json| {
                bytes(
                    &json!({"spec": target::PROGRAM_SPEC, "adts": [], "functions": [
                        {"parameters": [0], "types": [operand_ty], "result": result,
                         "body": {"kind": "prim", "operation": operation, "operands": [operand, {"kind": "var", "name": 0}]}}
                    ]}),
                )
            };
            let nats = json!({"kind": "list", "element": {"kind": "nat"}});
            rejects(
                &unary(
                    json!({"kind": "compare"}),
                    json!({"kind": "var", "name": 0}),
                    nats,
                    json!({"kind": "ordering"}),
                ),
                "primitive Compare does not apply to",
            );
            rejects(
                &unary(
                    json!({"kind": "equal"}),
                    json!({"kind": "var", "name": 0}),
                    json!({"kind": "int"}),
                    json!({"kind": "bool"}),
                ),
                "primitive Equal does not apply to [Int, Int]",
            );
            rejects(
                &unary(
                    json!({"kind": "nat_add"}),
                    json!({"kind": "var", "name": 0}),
                    json!({"kind": "int"}),
                    json!({"kind": "int"}),
                ),
                "primitive NatAdd takes [Nat, Nat], received [Int, Int]",
            );
            // A valid program loads and renders; an invalid one does neither.
            let valid = target::load(&bytes(&base)).expect("loads");
            assert!(target::render(&valid, rust::Profile::Std)
                .expect("renders")
                .starts_with("#![forbid(unsafe_code)]"));
            let invalid: Program = serde_json::from_value({
                let mut json = base.clone();
                json["functions"][0]["body"] = json!({"kind": "var", "name": 99});
                json
            })
            .expect("well formed");
            let error = target::render(&invalid, rust::Profile::Std)
                .expect_err("an invalid program has no rendering");
            support::expect_code(&error, "LLB6005");
        }
        // §17.14: the denotation is a kernel-checked LexLean definition and
        // states every fixture's outcome.
        "TC-03" => {
            let cases = fixtures::cases();
            let module =
                fixtures::fixtures_module(&cases) + &fixtures::reasoning_fixtures_module(&cases);
            let mut opaque = 0;
            for case in &cases {
                let id = fixtures::identifier(&case.fixture.name);
                assert!(module.contains(&format!("\"name\":\"{id}Run\"")), "{id}Run");
                let theorem = module.contains(&format!("\"name\":\"{id}Outcome\""));
                assert_eq!(
                    theorem,
                    fixtures::kernel_reducible(&case.fixture),
                    "{id}: a kernel theorem exactly where the kernel reduces"
                );
                if !theorem {
                    opaque += 1;
                }
                // A valid program never gets stuck: overflow and exhaustion
                // are its only failures.
                assert_ne!(case.fixture.expected, Outcome::Stuck, "{id}");
            }
            // The same on seeded random well-typed arguments at every fuel
            // up to the stated one: build evidence for the progress
            // property, not a proof of it.
            // The interpreter recurses once per unit of fuel, so the sample
            // runs on a stack sized for the largest stated fuel.
            let sampled = cases.clone();
            let runs = std::thread::Builder::new()
                .stack_size(1 << 30)
                .spawn(move || progress_runs(&sampled))
                .expect("thread")
                .join()
                .expect("no fixture program is stuck");
            assert!(
                runs > 10_000,
                "the progress property is sampled ({runs} runs)"
            );
            assert!(
                opaque > 0 && opaque < cases.len() / 4,
                "kernel-opaque fixtures are the exception ({opaque})"
            );
            assert!(cases
                .iter()
                .any(|case| matches!(case.fixture.expected, Outcome::Overflow { .. })));
            assert!(cases
                .iter()
                .any(|case| case.fixture.expected == Outcome::Exhausted));
            fixtures::check(repo_root().as_std_path(), false)
                .expect("the committed fixtures equal their generator");
            if let Some(verified) = support::lean_backed("TC-03").then(support::verified_compiler) {
                assert!(
                    verified.outcome.units.contains_key("TargetFixtures")
                        && verified.outcome.units.contains_key("ReasoningFixtures"),
                    "the fixture modules are verified"
                );
                let planted = planted_verification();
                // Lean names the evaluation and the outcome it was told to
                // expect: the wrong sum is refused.
                assert!(
                    planted
                        .iter()
                        .any(|message| message.contains("sumToRun")
                            && message.contains("Value.nat 56")),
                    "a wrong expected outcome is rejected: {planted:#?}"
                );
            }
        }
        // §17.14: Lean's evaluator decides every fixture, the kernel-opaque
        // ones included.
        "TC-04" => {
            let cases = fixtures::cases();
            if !support::lean_backed("TC-04") {
                return;
            }
            let verified = support::verified_compiler();
            let evaluated = lean_evaluations(&verified.outcome.root, &cases);
            assert_eq!(evaluated.len(), cases.len());
            let disagreements = evaluator_disagreements(&evaluated, &cases);
            assert!(
                disagreements.is_empty(),
                "Lean's evaluator and the interpreter disagree: {disagreements:#?}"
            );
            // The same comparison refuses an evaluator-only fixture whose
            // stated outcome is one step off, as an interpreter that
            // mischarged one primitive would state it.
            let mut planted = cases.clone();
            let opaque = planted
                .iter_mut()
                .find(|case| !fixtures::kernel_reducible(&case.fixture))
                .expect("an evaluator-only fixture");
            let name = opaque.fixture.name.clone();
            opaque.fixture.expected = match &opaque.fixture.expected {
                Outcome::Value { value, steps } => Outcome::Value {
                    value: value.clone(),
                    steps: steps + 1,
                },
                Outcome::Overflow { steps } => Outcome::Overflow { steps: steps + 1 },
                other => panic!("{name} has no step count: {other:?}"),
            };
            let planted_disagreements = evaluator_disagreements(&evaluated, &planted);
            assert_eq!(planted_disagreements.len(), 1, "{planted_disagreements:#?}");
            assert!(
                planted_disagreements[0].starts_with(&format!("{name}: ")),
                "{planted_disagreements:#?}"
            );
        }
        // §17.14: every library template realizes LexLean's own primitive.
        "TC-05" => {
            let cases = fixtures::cases();
            let mut covered = BTreeSet::new();
            for case in cases.iter().filter(|case| !case.libraries.is_empty()) {
                // The instances follow the fixture's own functions, one
                // after another.
                let mut expected = case.fixture.program.clone();
                let first = usize::try_from(case.libraries[0].at).expect("index");
                expected.functions.truncate(first);
                for library in &case.libraries {
                    covered.insert(library.template);
                    assert_eq!(
                        expected.functions.len() as u64,
                        library.at,
                        "{}: an instance follows the previous one",
                        case.fixture.name
                    );
                    expected.functions.extend(
                        library
                            .template
                            .instantiate(&library.types, library.at)
                            .expect("instantiates"),
                    );
                }
                assert_eq!(
                    expected.canonical().expect("bound"),
                    case.fixture.program,
                    "{}: the committed instances are the templates'",
                    case.fixture.name
                );
                assert!(
                    case.oracle.is_some(),
                    "{}: a library fixture states LexLean's own result",
                    case.fixture.name
                );
                assert!(
                    fixtures::kernel_reducible(&case.fixture),
                    "{}: the kernel decides it",
                    case.fixture.name
                );
                let id = fixtures::identifier(&case.fixture.name);
                assert!((fixtures::fixtures_module(&cases)
                    + &fixtures::reasoning_fixtures_module(&cases))
                    .contains(&format!("\"name\":\"{id}Agrees\"")));
            }
            assert_eq!(
                covered.len(),
                Template::ALL.len(),
                "every template has a fixture"
            );
            if support::lean_backed("TC-05") {
                let _ = support::verified_compiler();
                let planted = planted_verification();
                // The mutated instance still matches its own interpreted
                // outcome, so only the statement against LexLean's
                // `set_insert` (through `TargetOracle`) fails.
                assert!(
                    planted
                        .iter()
                        .any(|message| message.contains("setInsertNatRun")
                            && message.contains("TargetOracle.encodeNats")),
                    "a mutated realization disagrees with LexLean: {planted:#?}"
                );
                assert!(
                    !planted
                        .iter()
                        .any(|message| message.contains("setInsertNatRun")
                            && !message.contains("TargetOracle")),
                    "the mutated instance agrees with its own interpreted outcome: {planted:#?}"
                );
            }
        }
        // §17.14, §17.13: the realization table covers the production
        // registry and the fixtures exercise every calculus element.
        "TC-06" => {
            let registry: toml::Value = toml::from_str(
                &std::fs::read_to_string(
                    repo_root()
                        .join("language/production-1.2.toml")
                        .as_std_path(),
                )
                .expect("registry"),
            )
            .expect("registry parses");
            let runtime: BTreeSet<String> = registry["construct"]
                .as_array()
                .expect("constructs")
                .iter()
                .filter(|row| row["disposition"].as_str() == Some("runtime"))
                .map(|row| row["key"].as_str().expect("key").to_owned())
                .collect();
            assert!(runtime.len() > 100, "the registry has runtime rows");
            realization::check_table(&runtime).expect("the realization table");
            // A construct requires heap allocation exactly when its
            // realization does (§17.13, §17.14).
            let disagreements: Vec<String> = registry["construct"]
                .as_array()
                .expect("constructs")
                .iter()
                .filter(|row| row["disposition"].as_str() == Some("runtime"))
                .filter_map(|row| {
                    let key = row["key"].as_str().expect("key");
                    let registered = row["allocation"].as_bool().expect("allocation");
                    let realized = realization::construct_allocates(key).expect("realization");
                    (registered != realized)
                        .then(|| format!("{key}: registry {registered}, realization {realized}"))
                })
                .collect();
            assert!(
                disagreements.is_empty(),
                "allocation disagreements: {disagreements:#?}"
            );
            // A dropped row and an unknown reference are both caught.
            let mut short = runtime.clone();
            short.insert("term.invented".to_owned());
            let error = realization::check_table(&short).expect_err("a missing row");
            assert!(
                error.contains("runtime construct `term.invented` has no realization row"),
                "{error}"
            );
            let mut fewer = runtime.clone();
            fewer.remove("primitive.map_insert");
            let error = realization::check_table(&fewer).expect_err("an extra row");
            assert!(
                error.contains("realization row `primitive.map_insert` is not a runtime construct"),
                "{error}"
            );
            let mut used = BTreeSet::new();
            for case in fixtures::cases() {
                used.extend(realization::program_elements(&case.fixture.program));
                for argument in &case.fixture.arguments {
                    used.extend(realization::value_elements(argument));
                }
                if let Outcome::Value { value, .. } = &case.fixture.expected {
                    used.extend(realization::value_elements(value));
                }
            }
            let templates: BTreeSet<Template> = fixtures::cases()
                .iter()
                .flat_map(|case| case.libraries.iter().map(|library| library.template))
                .collect();
            assert_eq!(
                templates.len(),
                Template::ALL.len(),
                "every library template is exercised"
            );
            let all = realization::all_elements().expect("complete samples");
            let missing: Vec<&String> = all.difference(&used).collect();
            assert!(
                missing.is_empty(),
                "calculus elements no fixture exercises: {missing:?}"
            );
            // Every fixed-width primitive at every width it admits, read from
            // the fixtures' typed programs.
            let mut pairs = BTreeSet::new();
            for case in fixtures::cases() {
                pairs.extend(realization::fixed_uses(&case.fixture.program).expect("well typed"));
            }
            let missing: Vec<_> = realization::fixed_pairs()
                .difference(&pairs)
                .cloned()
                .collect();
            assert!(
                missing.is_empty(),
                "(primitive, width) pairs no fixture exercises: {missing:?}"
            );
            assert!(
                pairs.is_subset(&realization::fixed_pairs()),
                "a fixture applies a primitive at a width the calculus does not admit"
            );
        }
        // §17.14: both Rust profiles reproduce every observable outcome
        // within the denotation's step count, `rust-core` exactly when the
        // program needs no heap, and planted renderer discrepancies are
        // detected.
        "TC-07" => {
            let version = rustc_version();
            assert!(
                version.starts_with(RUSTC_VERSION),
                "the pinned rustc is the oracle (RUSTC-1-97-1), found {version}"
            );
            let cases = fixtures::cases();
            let dir = tempfile::Builder::new()
                .prefix("lexlean-calculus-rust-")
                .tempdir()
                .expect("tempdir");
            let mut jobs: Vec<Job> = Vec::new();
            let mut unobservable = BTreeSet::new();
            let mut core = 0;
            for case in &cases {
                let fixture = &case.fixture;
                let allocates = realization::program_allocates(&fixture.program);
                for profile in rust::Profile::ALL {
                    let library = match rust::render(&fixture.program, profile) {
                        Ok(library) => library,
                        Err(reason) => {
                            assert_eq!(profile, rust::Profile::Core, "{}: {reason}", fixture.name);
                            assert!(
                                reason.contains(
                                    "requires heap allocation, which rust-core does not provide"
                                ),
                                "{}: {reason}",
                                fixture.name
                            );
                            assert!(
                                allocates,
                                "{}: rust-core refuses a program that needs no heap: {reason}",
                                fixture.name
                            );
                            continue;
                        }
                    };
                    let header = match profile {
                        rust::Profile::Core => {
                            assert!(
                                !allocates,
                                "{}: rust-core renders a program that needs the heap",
                                fixture.name
                            );
                            core += 1;
                            for heap in ["std::", "alloc", "extern crate", "Vec", "String", "Rc"] {
                                assert!(!library.contains(heap), "{}: {heap}", fixture.name);
                            }
                            "#![no_std]\n#![forbid(unsafe_code)]\n"
                        }
                        rust::Profile::Std => "#![forbid(unsafe_code)]\n",
                    };
                    assert!(library.starts_with(header), "{}", fixture.name);
                    for forbidden in [
                        "unsafe {",
                        "unreachable!",
                        "panic!",
                        "#![allow",
                        "#[allow",
                        "Box",
                        ".to_vec(",
                        ".insert(",
                    ] {
                        assert!(
                            !library.contains(forbidden),
                            "{}: the rendering contains `{forbidden}`",
                            fixture.name
                        );
                    }
                    let steps = match &fixture.expected {
                        Outcome::Value { value, steps } if observable(value) => *steps,
                        Outcome::Overflow { steps } => *steps,
                        _ => {
                            unobservable.insert(fixture.name.clone());
                            continue;
                        }
                    };
                    let harness = crate::rust_harness::render_harness(
                        &fixture.program,
                        profile,
                        fixture.entry,
                        &fixture.arguments,
                    )
                    .unwrap_or_else(|reason| panic!("{}: {reason}", fixture.name));
                    jobs.push(Job {
                        name: format!(
                            "{}_{}",
                            crate::calculus::identifier(&fixture.name),
                            profile.target().replace('-', "_")
                        ),
                        library,
                        harness,
                        observed: rust::observable(&fixture.expected).expect("observable"),
                        steps,
                    });
                }
            }
            // Only a closure-valued, exhausted, or stuck outcome escapes Rust.
            for name in &unobservable {
                let case = cases
                    .iter()
                    .find(|case| &case.fixture.name == name)
                    .expect("case");
                assert!(
                    match &case.fixture.expected {
                        Outcome::Value { value, .. } => !observable(value),
                        Outcome::Overflow { .. } => false,
                        Outcome::Stuck | Outcome::Exhausted => true,
                    },
                    "{name} is observable"
                );
            }
            assert!(
                core >= 10 && core < cases.len(),
                "rust-core renders the fixtures without heap values and only those ({core})"
            );
            let outputs = run_jobs(&jobs, dir.path());
            for job in &jobs {
                let (printed, work) = &outputs[&job.name];
                assert_eq!(
                    printed, &job.observed,
                    "{}: the Rust rendering and the denotation disagree",
                    job.name
                );
                assert!(
                    *work <= job.steps,
                    "{}: the rendering worked {work} units for {} steps",
                    job.name,
                    job.steps
                );
            }
            // Planted discrepancies: a wrapping addition and a Euclidean
            // quotient each change an observable outcome; an append that
            // copies its right operand per element keeps the value and
            // breaks the work bound.
            let plants = [
                (
                    "natOverflowAdd_rust_core",
                    "a.checked_add(b).ok_or(Overflow)",
                    "Ok(a.wrapping_add(b))",
                    false,
                ),
                (
                    "intArithmetic_rust_std",
                    "a.checked_div(b).ok_or(Overflow)",
                    "a.checked_div_euclid(b).ok_or(Overflow)",
                    false,
                ),
                (
                    "listAppendLong_rust_std",
                    "pub fn append_list<T: Clone>(a: List<T>, b: List<T>) -> List<T> { List::onto(a.items(), b) }",
                    "pub fn append_list<T: Clone>(a: List<T>, b: List<T>) -> List<T> { let mut out = b; for item in a.items().into_iter().rev() { tick(1); out = List::cons(item, List::onto(out.items(), List::nil())); } out }",
                    true,
                ),
            ];
            let planted: Vec<Job> = plants
                .iter()
                .map(|(name, from, to, _)| {
                    let job = jobs
                        .iter()
                        .find(|job| job.name == *name)
                        .unwrap_or_else(|| panic!("{name} is rendered"));
                    assert!(job.library.contains(from), "{name}: the plant site exists");
                    Job {
                        name: format!("planted_{name}"),
                        library: job.library.replacen(from, to, 1),
                        ..job.clone()
                    }
                })
                .collect();
            let planted_outputs = run_jobs(&planted, dir.path());
            for ((name, _, _, work_only), job) in plants.iter().zip(&planted) {
                let (printed, work) = &planted_outputs[&job.name];
                if *work_only {
                    assert_eq!(printed, &job.observed, "{name}: the plant keeps the value");
                    assert!(
                        *work > job.steps,
                        "{name}: the quadratic append is detected ({work} units, {} steps)",
                        job.steps
                    );
                } else {
                    assert_ne!(
                        printed, &job.observed,
                        "{name}: the planted discrepancy is detected"
                    );
                }
            }
        }
        other => panic!("no calculus case is wired for {other}"),
    }
}
