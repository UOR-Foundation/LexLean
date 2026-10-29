//! GL-21 and VR-24: specialisation wrappers (SPEC.md §13.12, §18.8).
//!
//! The fixture is `tests/fixtures/specialisation`: a request for
//! `Std.Commutative.comm` and `Std.Associative.assoc` at `Nat`, `Int`,
//! `UInt32`, and `BitVec 32`, the host module the emitter writes for it
//! (committed, as the project's `SpecHost.lean`), and a project whose
//! entries denote two of the wrappers.

use std::process::Command;

use lexlean::wrappers::{emit, is_wrapper_name, parse_request};

use crate::support::{self, P};

const REQUEST: &str = "tests/fixtures/specialisation/request.toml";
const HOST: &str = "tests/fixtures/specialisation/project/SpecHost.lean";
const PROJECT: &str = "fixtures/specialisation/project";

fn read(relative: &str) -> String {
    std::fs::read_to_string(support::repo_root().join(relative).as_std_path())
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
}

fn refused(request: &str) -> String {
    let error = parse_request(request).expect_err(request);
    assert_eq!(error.code.as_str(), "LLT4018", "{request}");
    error.message
}

/// GL-21: the emitter writes terms, deterministically, and refuses the rest.
pub(super) fn emitter() {
    let request_text = read(REQUEST);
    let request = parse_request(&request_text).expect("the fixture request is valid");
    let first = emit(&request);
    let second = emit(&parse_request(&request_text).expect("valid"));
    assert_eq!(first, second, "two runs write the same bytes");
    assert_eq!(
        first.module_text,
        read(HOST),
        "the committed host module is the emitter's output"
    );

    // The request's order is not the module's: the wrappers are sorted by
    // Lean name.
    let mut blocks: Vec<&str> = request_text.split("\n[[wrapper]]").collect();
    let header = blocks.remove(0);
    blocks.reverse();
    let reversed = format!("{header}\n[[wrapper]]{}", blocks.join("\n[[wrapper]]"));
    assert_eq!(
        emit(&parse_request(&reversed).expect("valid")).module_text,
        first.module_text,
        "the order of the request does not reach the module"
    );

    assert_eq!(first.wrappers.len(), 8);
    let lines: Vec<&str> = first.module_text.lines().collect();
    assert_eq!(lines[0], "module");
    for wrapper in &first.wrappers {
        assert!(is_wrapper_name(&wrapper.lean_name));
        let line = lines[wrapper.line - 1];
        assert!(
            line.starts_with(&format!(
                "public theorem {} : type_of% (@_root_.",
                wrapper.lean_name
            )),
            "{line}"
        );
        // A term and nothing else: no tactic block, no `by`, no `sorry`, no
        // axiom, no binder, and every constant qualified from the root.
        for forbidden in [
            " by ", "by\n", "sorry", "axiom", "fun ", "λ", "calc", "decide",
        ] {
            assert!(!line.contains(forbidden), "`{forbidden}` in {line}");
        }
        assert!(!line.contains(" Nat ") && !line.contains("(Nat "), "{line}");
    }
    for tag in ["Nat", "Int", "UInt32", "BitVec32"] {
        assert!(
            first
                .wrappers
                .iter()
                .any(|wrapper| wrapper.tag == tag && wrapper.declaration == "Std.Commutative.comm"),
            "{tag} is specialised"
        );
    }
    let denotation = first.wrappers[0].denotation("SpecHost");
    assert!(denotation.contains("kind = \"lean\"") && denotation.contains("module = \"SpecHost\""));

    // The refusals.
    let wrapper = |body: &str| {
        format!("spec = \"lexlean/wrappers/1\"\nimports = [\"Init\"]\n[[wrapper]]\n{body}\n")
    };
    let row = |arguments: &str, tag: &str, declaration: &str| {
        wrapper(&format!(
            "declaration = \"{declaration}\"\nkind = \"theorem\"\ntag = \"{tag}\"\narguments = [{arguments}]"
        ))
    };
    let tactic = refused(&row("\"(by simp)\"", "T", "f"));
    assert!(tactic.contains("by"), "{tactic}");
    refused(&row("\"(fun x x)\"", "T", "f"));
    refused(&row("\"\\\"text\\\"\"", "T", "f"));
    refused(&row("\"_ -- comment\"", "T", "f"));
    refused(&row("\"(Nat\"", "T", "f"));
    refused(&row("", "T", "f g"));
    refused(&row("", "T.U", "f"));
    let duplicate = format!(
        "{}[[wrapper]]\ndeclaration = \"f\"\nkind = \"theorem\"\ntag = \"T\"\narguments = []\n",
        row("", "T", "f")
    );
    assert!(refused(&duplicate).contains("twice"));
}

/// Build the host library of a project copy: `lake build` against the
/// pinned toolchain, offline (the library has no dependency).
fn build_host(project: &P) {
    let output = Command::new("lake")
        .arg("build")
        .arg("SpecHost")
        .current_dir(project.root.as_std_path())
        .output()
        .expect("lake runs; is leanprover/lean4:v4.32.1 installed?");
    assert!(
        output.status.success(),
        "the host library builds: {}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn prepared() -> P {
    let project = P::copy_tests_project(PROJECT);
    build_host(&project);
    project
}

/// VR-24: the wrapper line of the probe, accepted for the exact signature,
/// refused for a wrong one, and `LLT4003` for a wrapper that is not there.
pub(super) fn probe() {
    if !support::lean_backed("VR-24") {
        return;
    }
    let exact = prepared();
    exact.relock();
    let verified = exact
        .engine()
        .verify(lexlean::VerifyRequest {
            selection: lexlean::Selection::Entrypoints,
        })
        .expect("the exact signatures verify");
    let probe_dir = verified.root.join("probe");
    let probe_file = support::file_set(&probe_dir)
        .into_iter()
        .find(|name| name.ends_with(".lean"))
        .expect("the probe module is published");
    let probe_text =
        std::fs::read_to_string(probe_dir.join(&probe_file).as_std_path()).expect("read");
    for wrapper in [
        "LexLeanSpec.Int.Std.Commutative.comm",
        "LexLeanSpec.Nat.Std.Commutative.comm",
    ] {
        assert!(
            probe_text.contains(&format!(") = (type_of% @{wrapper}) := rfl")),
            "the probe holds the equality of {wrapper}:\n{probe_text}"
        );
    }
    // An entry outside the reserved root is probed as it always was.
    assert!(probe_text.contains("example : (x0 : Nat) → (x1 : Nat) → Nat := Nat.add"));

    // A signature that is not the type of the wrapper: `a + a`'s twin.
    let planted = prepared();
    planted.edit(
        "lexicons/test-spec/entries/nat-comm.toml",
        "(app (const lexlean.std.nat::add) (local b) (local a))",
        "(app (const lexlean.std.nat::add) (local a) (local b))",
    );
    planted.relock();
    let error = planted.verify_fails_with("LLT4017");
    let text = error.to_string();
    assert!(
        text.contains("test.spec::nat-comm"),
        "names the entry: {text}"
    );
    assert!(
        !text.contains("int-comm"),
        "the exact entry is not blamed: {text}"
    );

    // A wrapper the host library does not have is the interface failure.
    let missing = prepared();
    missing.edit(
        "lexicons/test-spec/entries/nat-comm.toml",
        "LexLeanSpec.Nat.Std.Commutative.comm",
        "LexLeanSpec.Nat.Std.Commutative.nope",
    );
    missing.relock();
    missing.verify_fails_with("LLT4003");
}
