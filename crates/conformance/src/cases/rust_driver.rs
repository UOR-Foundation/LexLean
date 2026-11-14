//! The driver of a command root (SPEC.md §17.13, §17.16): `RB-24`.
//!
//! The driver is the text of the language-1.2 module `RuntimeHost`. The case
//! holds the module and its shipped copy, the seven crates of it under the
//! validator, the shape a command's host types must have, the effects a
//! driver may name, and, on a Lean host, a built and run package of the
//! committed fixture against a fixed vector, and the same run with each
//! planted defect.

use std::collections::BTreeMap;
use std::path::Path;
use std::time::{Duration, Instant};

use lexlean::calculus::rust::package::{Errors, Export, Passing};
use lexlean::calculus::rust::rt::{Crate, Item, Profile};
use lexlean::calculus::rust::rt_validate::Host;
use lexlean::calculus::rust::{host, rt_decode, rt_validate};
use lexlean::calculus::{Program, Ty};

use crate::rust_build::cargo_in;
use crate::support::{self, repo_root, P};

/// The rules of the validator that `RB-24` plants: the host table and the
/// `cfg` items of the driver.
pub(super) const RULES: [&str; 4] = ["cfg-condition", "cfg-item", "cfg-pair", "hidden-effect"];

const MODULE: &str = "compiler/src/RuntimeHost.lex.tex";
const SHIPPED: &str = "language/rust-runtime-1.2/RuntimeHost.lex.tex";
const FIXTURE: &str = "tests/fixtures/command-driver";
const ROOT: &str = "CommandDriver.Main.step";

/// The crates of the module, by the effect a group of them is for.
const CRATES: [&str; 7] = [
    "hostBase",
    "stepFallible",
    "stepPlain",
    "filesOn",
    "filesOff",
    "processOn",
    "processOff",
];

/// The names that realize `files` or `process`, which a driver whose root
/// does not admit the effect may not print.
const HOST_NAMES: [&str; 6] = ["Command", "Stdio", "File", "fs::", "Instant", "Duration"];

/// The fixed result of the fixture command: the child's status, and the
/// output and errors it wrote, which the parent writes as its own.
const STATUS: i32 = 3;
const OUTPUT: &[u8] = b"out\n";
const ERRORS: &[u8] = b"err\n";

/// A defect planted in the printed `src/main.rs`: its name, the text it
/// replaces, and the text it puts there. Each changes the result of the run
/// above.
const PLANTS: [(&str, &str, &str); 4] = [
    (
        "output and errors swapped",
        "leave(status, &output, &errors)",
        "leave(status, &errors, &output)",
    ),
    ("a child's status dropped", "status.code()", "None"),
    (
        "the state of the previous step passed to the next",
        "state = Some(kept);",
        "state = None;",
    ),
    (
        "a listing in the reverse of byte order",
        "out.sort();",
        "out.sort(); out.reverse();",
    ),
];

fn read(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path).as_std_path())
        .unwrap_or_else(|error| panic!("{path}: {error}"))
}

/// The crates of the module composed as the renderer composes a driver: the
/// base, a step, and a group of each effect, each `use` kept once.
fn compose(text: &str, parts: &[&str]) -> Crate {
    let mut items: Vec<Item> = Vec::new();
    for part in parts {
        let krate = rt_decode::decode_host(text, part)
            .unwrap_or_else(|error| panic!("{part} decodes: {error}"));
        for item in krate.items {
            if matches!(item, Item::Use(..)) && items.contains(&item) {
                continue;
            }
            items.push(item);
        }
    }
    Crate {
        profile: Profile::Std,
        items,
    }
}

/// The module and its shipped copy, and every driver the renderer can
/// compose of its crates under the validator, with the effects that make its
/// names admitted. A crate is not checked alone: the base names the type of
/// each effect's group (`Files`, `Procs`), which the group declares, so only
/// a composition is a closed crate.
fn module() {
    let text = read(MODULE);
    assert_eq!(
        text,
        read(SHIPPED),
        "the shipped copy is the module byte for byte"
    );
    let embedded = rt_decode::embedded_host().expect("the embedded module");
    assert_eq!(embedded, text, "the embedded copy is the shipped one");
    for name in CRATES {
        rt_decode::decode_host(&text, name)
            .unwrap_or_else(|error| panic!("{name} decodes: {error}"));
    }
    // Each composition is valid with exactly the effects its groups realize,
    // and refused as `hidden-effect` by a root that lacks one of them.
    for step in ["stepFallible", "stepPlain"] {
        for files in [true, false] {
            for process in [true, false] {
                let parts = [
                    "hostBase",
                    step,
                    if files { "filesOn" } else { "filesOff" },
                    if process { "processOn" } else { "processOff" },
                ];
                let krate = compose(&text, &parts);
                let mut effects: Vec<&str> = Vec::new();
                if files {
                    effects.push("files");
                }
                if process {
                    effects.push("process");
                }
                rt_validate::validate_in(
                    &krate,
                    Host {
                        roots: &["host"],
                        effects: &effects,
                    },
                )
                .unwrap_or_else(|error| panic!("{parts:?} is valid: {error}"));
                for effect in effects.clone() {
                    let fewer: Vec<&str> = effects
                        .iter()
                        .copied()
                        .filter(|held| *held != effect)
                        .collect();
                    let error = rt_validate::validate_in(
                        &krate,
                        Host {
                            roots: &["host"],
                            effects: &fewer,
                        },
                    )
                    .expect_err("a group whose effect the root lacks");
                    assert!(
                        error.contains("hidden-effect"),
                        "{parts:?} without {effect}: {error}"
                    );
                }
            }
        }
    }
    // The validator refuses a `cfg` function without its twin.
    let full = compose(&text, &["hostBase", "stepFallible", "filesOn", "processOn"]);
    let both = Host {
        roots: &["host"],
        effects: &["files", "process"],
    };
    let mut lone = full.clone();
    lone.items.retain(|item| match item {
        Item::Cfg(condition, inner) => {
            !(condition == "not_unix"
                && matches!(inner.as_ref(), Item::Fn(function)
                    if function.sig.name == "end_group"))
        }
        _ => true,
    });
    assert!(lone.items.len() < full.items.len(), "the twin is removed");
    assert!(rt_validate::validate_in(&lone, both)
        .expect_err("a cfg function without its twin")
        .contains("cfg-pair"));
    // A condition outside the two, and an item that is neither a `use` nor a
    // function under one.
    for (what, condition, inner, rule) in [
        (
            "a condition outside the closed set",
            "maybe",
            Item::Use(vec!["std".to_owned(), "mem".to_owned()], None),
            "cfg-condition",
        ),
        (
            "a cfg item that is not a use or a function",
            "unix",
            Item::UnitStruct("Marker".to_owned(), Vec::new()),
            "cfg-item",
        ),
    ] {
        let mut planted = full.clone();
        planted
            .items
            .push(Item::Cfg(condition.to_owned(), Box::new(inner)));
        let error = rt_validate::validate_in(&planted, both).expect_err(what);
        assert!(error.contains(rule), "{what}: {error}");
    }
}

/// The export of function `function` as `run`, by value, as production
/// exports a root.
fn export(function: u64, errors: Errors, passing: Passing) -> Export {
    Export {
        function,
        name: "run".to_owned(),
        parameters: vec![passing, Passing::Own],
        errors,
    }
}

/// The index of the function of `program` that is a command's step.
fn step_of(program: &Program) -> (u64, Errors) {
    for at in 0..program.functions.len() {
        for errors in [Errors::Overflow, Errors::None] {
            let function = u64::try_from(at).expect("an index");
            let exports = [export(function, errors, Passing::Own)];
            if host::recognize(program, &exports, "run").is_ok() {
                return (function, errors);
            }
        }
    }
    panic!("the fixture's step is a command");
}

/// The shape a command's types must have: the fixture's step is recognized,
/// and each of these is refused with the reason it names.
fn shape(program: &Program) {
    let (function, errors) = step_of(program);
    let good = [export(function, errors, Passing::Own)];
    let command = host::recognize(program, &good, "run").expect("the step is a command");
    assert_eq!(command.entry, "run");
    assert_eq!(command.fallible, errors == Errors::Overflow);

    let refused = |program: &Program, exports: &[Export], entry: &str, needle: &str| {
        let error = host::recognize(program, exports, entry).expect_err(needle);
        assert!(error.contains(needle), "{needle}: {error}");
    };
    refused(program, &good, "other", "is not an export");
    refused(
        program,
        &[export(function, errors, Passing::Borrow)],
        "run",
        "by value",
    );
    let function_ty = &program.functions[usize::try_from(function).expect("an index")];
    let Ty::Pair { right, .. } = &function_ty.result else {
        panic!("the step returns a pair")
    };
    let Ty::Adt { index: request } = right.as_ref() else {
        panic!("the step returns a request")
    };
    let request = usize::try_from(*request).expect("an index");

    // A request type with a constructor fewer, and one with a field of
    // another type.
    let mut fewer = program.clone();
    fewer.adts[request].constructors.pop();
    refused(&fewer, &good, "run", "constructors");
    let mut other = program.clone();
    other.adts[request].constructors[0][0] = Ty::Nat;
    refused(&other, &good, "run", "another type");

    // A step whose result is not a pair of the session and a request.
    let mut plain = program.clone();
    plain.functions[usize::try_from(function).expect("an index")].result = Ty::Nat;
    refused(&plain, &good, "run", "is not of the type");
}

/// The text of `src/main.rs` for the command, by the effects the root admits.
fn rendered(program: &Program, effects: &[&str]) -> String {
    let (function, errors) = step_of(program);
    let exports = [export(function, errors, Passing::Own)];
    let command = host::recognize(program, &exports, "run").expect("a command");
    let effects: Vec<String> = effects.iter().map(|effect| (*effect).to_owned()).collect();
    host::main_rs(&command, "command_driver", &effects).expect("the driver prints")
}

/// Whether `text` names `name`: as a whole name, so the driver's own type
/// `Files`, which every driver declares, is not the standard library's `File`.
fn mentions(text: &str, name: &str) -> bool {
    let word = |c: char| c.is_alphanumeric() || c == '_';
    text.match_indices(name).any(|(at, _)| {
        let before = text[..at].chars().next_back();
        let after = text[at + name.len()..].chars().next();
        !before.is_some_and(word) && (name.ends_with(':') || !after.is_some_and(word))
    })
}

/// A root that admits an effect has the names of its group, and one that does
/// not has none of them, only refusals.
fn effects(program: &Program) {
    let full = rendered(program, &["allocation", "files", "overflow", "process"]);
    for name in HOST_NAMES {
        assert!(
            mentions(&full, name),
            "a root with both effects names `{name}`"
        );
    }
    let bare = rendered(program, &["allocation", "overflow"]);
    for name in HOST_NAMES {
        assert!(
            !mentions(&bare, name),
            "a root with neither effect names `{name}`"
        );
    }
    assert!(bare.contains("refuse"), "the groups it lacks refuse");
    let files_only = rendered(program, &["allocation", "files", "overflow"]);
    assert!(mentions(&files_only, "fs::") && !mentions(&files_only, "Command"));
}

/// Run `binary` in `directory` and wait at most `limit`: its status, standard
/// output, and standard error, or `None` for a run that did not end.
fn execute(
    binary: &Path,
    directory: &Path,
    limit: Duration,
) -> Option<(Option<i32>, Vec<u8>, Vec<u8>)> {
    let out = directory.join("stdout.bin");
    let err = directory.join("stderr.bin");
    let work = directory.join("run");
    std::fs::create_dir_all(&work).expect("a directory");
    let mut child = std::process::Command::new(binary)
        .current_dir(&work)
        .stdin(std::process::Stdio::null())
        .stdout(std::fs::File::create(&out).expect("a file"))
        .stderr(std::fs::File::create(&err).expect("a file"))
        .spawn()
        .expect("the driver starts");
    let started = Instant::now();
    loop {
        if let Some(status) = child.try_wait().expect("a status") {
            let read = |path: &Path| std::fs::read(path).expect("output");
            return Some((status.code(), read(&out), read(&err)));
        }
        if started.elapsed() > limit {
            let _ = child.kill();
            let _ = child.wait();
            return None;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

/// What a run of the driver gives: its exit status, standard output, and standard error.
type Ran = (Option<i32>, Vec<u8>, Vec<u8>);

/// Build the package `files` under cargo and run it, as the `run` function does.
fn build_and_run(files: &BTreeMap<String, Vec<u8>>) -> Result<Option<Ran>, String> {
    let directory = tempfile::tempdir().expect("a directory");
    let package = directory.path().join("package");
    // The driver is the binary of the package itself, so the files are
    // written as they are and built with cargo directly.
    for (path, bytes) in files {
        let target = package.join(path);
        std::fs::create_dir_all(target.parent().expect("a parent")).expect("a directory");
        std::fs::write(&target, bytes).expect("a file");
    }
    let output = cargo_in(&package, &["build", "--quiet"]);
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }
    let binary = package
        .join("target")
        .join("debug")
        .join(format!("command_driver{}", std::env::consts::EXE_SUFFIX));
    Ok(execute(&binary, directory.path(), Duration::from_secs(120)))
}

/// The package of the fixture, built and run on this host.
fn fixture(project: &P) {
    let application = crate::production_consumer::Application {
        roots: vec![crate::production_consumer::AppRoot {
            root: ROOT.to_owned(),
            name: "command_driver".to_owned(),
            version: "0.1.0".to_owned(),
            targets: vec!["rust-std".to_owned()],
        }],
    };
    let (output, _) =
        crate::production_consumer::export(&project.root.join("lexlean.toml"), &application, &[])
            .expect("the command root certifies and exports");
    let tree = crate::production_export::tree(&project.root.join(&output.bundle));
    let main = tree
        .keys()
        .find(|path| path.ends_with("/src/main.rs"))
        .expect("the package of a command root carries src/main.rs")
        .clone();
    let prefix = main.trim_end_matches("src/main.rs").to_owned();
    let files: BTreeMap<String, Vec<u8>> = ["Cargo.toml", "src/lib.rs", "src/main.rs"]
        .iter()
        .map(|name| {
            let bytes = tree
                .get(&format!("{prefix}{name}"))
                .unwrap_or_else(|| panic!("the package has {name}"))
                .clone();
            ((*name).to_owned(), bytes)
        })
        .collect();

    // The provenance binds the driver and names the entry.
    let provenance: serde_json::Value =
        serde_json::from_slice(&tree[&format!("{prefix}provenance.json")]).expect("provenance");
    assert_eq!(provenance["entry"], "run");
    assert_eq!(
        provenance["files"]["src/main.rs"],
        lexlean::Sha256Digest::of(&files["src/main.rs"]).to_hex(),
        "the provenance holds the digest of the driver"
    );

    // The fixed result.
    let vector = Some((Some(STATUS), OUTPUT.to_vec(), ERRORS.to_vec()));
    let ran = build_and_run(&files).unwrap_or_else(|error| panic!("the driver builds: {error}"));
    assert_eq!(ran, vector, "the fixture command runs to its fixed result");

    // Each plant changes it, or stops the driver from building.
    let text = String::from_utf8(files["src/main.rs"].clone()).expect("utf-8");
    for (name, from, to) in PLANTS {
        assert!(
            text.contains(from),
            "{name}: the printed driver has {from:?}"
        );
        let mut planted = files.clone();
        planted.insert(
            "src/main.rs".to_owned(),
            text.replacen(from, to, 1).into_bytes(),
        );
        let result = build_and_run(&planted);
        assert!(
            result.as_ref().map_or(true, |ran| *ran != vector),
            "{name}: the planted driver gives the fixed result"
        );
    }
}

/// Run the case of `RB-24`.
pub fn run(id: &str) {
    assert_eq!(id, "RB-24");
    module();
    let project = {
        let project = P::workspace_project(FIXTURE, &[FIXTURE, "stdlib"]);
        std::fs::write(
            project
                .workspace_dir()
                .join("lexlean-workspace.toml")
                .as_std_path(),
            format!("spec = \"lexlean/workspace/1\"\nmembers = [\"stdlib\", \"{FIXTURE}\"]\n"),
        )
        .expect("the workspace file");
        project.relock();
        project
    };
    let program = crate::production_export::lowered_program(&project, ROOT);
    shape(&program);
    effects(&program);
    if !support::lean_backed("RB-24") {
        return;
    }
    let _environment = support::env_shared();
    fixture(&project);
}
