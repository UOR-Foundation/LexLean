//! The `extraction` suite: NE-01..NE-06, named-root extraction through
//! Lean's compiler front end (SPEC.md §22.10).
//!
//! Lean is the oracle twice over: it reports the facts every closure is
//! computed from, and the closure must equal the one the
//! production-eligibility analysis computed independently from the semantic
//! IR. The rejection classes are exercised against the committed extraction
//! record of the production example, so every planted record differs from a
//! real answer of pinned Lean in exactly the defect under test; the adapter's
//! own classification is exercised on real Lean over a hand-written module
//! that reaches every kind of constant it can refuse.

use std::collections::BTreeSet;

use crate::support::{self, P};
use lexlean::production::lcnf::{self, Rejection};
use lexlean::production::ModuleReport;

/// The committed raw extraction record of the production example: the
/// normalized stdout of the extraction process.
fn committed_record() -> String {
    let root =
        support::repo_root().join("examples/production/expected/verify/extract/process.json");
    let record: serde_json::Value = serde_json::from_slice(
        &std::fs::read(root.as_std_path()).expect("the committed extraction record"),
    )
    .expect("process record JSON");
    record["stdout"].as_str().expect("stdout").to_owned()
}

const MODULES: [&str; 2] = ["Production.Kernel", "Production.Main"];

/// The roots and eligibility reports of the production example.
fn production_inputs() -> (Vec<String>, Vec<ModuleReport>) {
    let project = P::copy_example("production");
    let checked = support::checked_project(&project);
    let reports: Vec<ModuleReport> = checked
        .modules
        .values()
        .filter_map(|module| module.production.clone())
        .filter(|report| !report.roots.is_empty())
        .collect();
    let roots = reports
        .iter()
        .flat_map(|report| report.roots.iter().map(|root| root.root.clone()))
        .collect();
    (roots, reports)
}

fn modules() -> Vec<String> {
    MODULES.map(str::to_owned).to_vec()
}

fn extract_with(record: &str, reports: &[ModuleReport]) -> Result<lcnf::CompilerInput, Rejection> {
    let (roots, _) = production_inputs();
    let driver = lcnf::driver(&"0".repeat(32), &roots, &modules()).expect("driver");
    let reports: Vec<&ModuleReport> = reports.iter().collect();
    lcnf::compiler_input(&driver, record, &roots, &modules(), &reports)
}

fn extract(record: &str) -> Result<lcnf::CompilerInput, Rejection> {
    extract_with(record, &production_inputs().1)
}

fn rejected_by(result: Result<lcnf::CompilerInput, Rejection>, fragment: &str) {
    match result {
        Err(Rejection::Rejected(reason)) => {
            assert!(
                reason.contains(fragment),
                "expected {fragment:?}, got {reason}"
            );
        }
        other => panic!("expected an LLV7011 rejection containing {fragment:?}, got {other:?}"),
    }
}

fn rejected(record: &str, fragment: &str) {
    rejected_by(extract(record), fragment);
}

fn mutate(record: &str, edit: impl FnOnce(&mut serde_json::Value)) -> String {
    let mut value: serde_json::Value =
        serde_json::from_str(record.trim_end()).expect("record JSON");
    edit(&mut value);
    serde_json::to_string(&value).expect("record serializes")
}

fn constant_mut<'a>(value: &'a mut serde_json::Value, name: &str) -> &'a mut serde_json::Value {
    value["constants"]
        .as_array_mut()
        .expect("constants")
        .iter_mut()
        .find(|constant| constant["name"] == name)
        .expect("the constant")
}

/// The pinned toolchain.
fn toolchain() -> lexlean::verify::toolchain::Toolchain {
    let project = P::copy_example("production");
    let loaded = lexlean::project::Project::load(&project.root.join("lexlean.toml")).expect("load");
    lexlean::verify::toolchain::preflight(&loaded.config.limits).expect("the pinned toolchain")
}

/// Run pinned `lean` on a standalone extraction module: no project module is
/// imported, so only the adapter, the probes, the registry check, and the
/// root lookup run.
fn run_lean(text: &str) -> (i32, String) {
    let _guard = support::env_lock();
    let toolchain = toolchain();
    let directory = tempfile::tempdir().expect("tempdir");
    let source = directory.path().join("Extract.lean");
    std::fs::write(&source, text).expect("write the extraction module");
    let output = std::process::Command::new(toolchain.lean.path.as_std_path())
        .arg(&source)
        .current_dir(directory.path())
        .output()
        .expect("run lean");
    (
        output.status.code().unwrap_or(-1),
        format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ),
    )
}

/// A hand-written module reaching every kind of constant the extraction
/// classifies, and every LCNF form base-phase translation produces.
const PROBE_MODULE: &str = "module
public section
namespace Probe.Main
opaque hidden : Nat
def viaOpaque : Nat := hidden + 1
axiom postulated : Nat
noncomputable def viaAxiom : Nat := postulated + 1
partial def spin (n : Nat) : Nat := spin (n + 1)
def viaPartial (n : Nat) : Nat := spin n
unsafe def risky : Nat := 1
@[extern \"lexlean_probe_external\"] def external (n : Nat) : Nat := n
def viaExternal (n : Nat) : Nat := external n
def greeting : String := \"hi\"
def joined (b : Bool) (n : Nat) : Nat :=
  let x := match b with
    | true => n + 1
    | false => n + 2
  x * x
def impossible (h : False) : Nat := nomatch h
def ordinal (o : Ordering) : Nat := match o with
  | .lt => 1
  | _ => 2
structure Box where
  value : Nat
  ok : value = value
def boxed (n : Nat) : Box := { value := n, ok := rfl }
def parsed (s : String) : Option Int := s.toInt?
structure Halver where
  run : Nat → Nat → Nat
def halver : Halver := ⟨Nat.div⟩
end Probe.Main
end
";

/// Build `PROBE_MODULE` in a Lake workspace with pinned Lean and run the
/// extraction driver for `roots` over it; the parsed raw record.
fn probe_extraction(roots: &[&str]) -> serde_json::Value {
    let _guard = support::env_lock();
    let toolchain = toolchain();
    let directory = tempfile::tempdir().expect("tempdir");
    let root = directory.path();
    std::fs::create_dir_all(root.join("Probe")).expect("module directory");
    std::fs::write(
        root.join("lakefile.toml"),
        "name = \"probe\"\nversion = \"0.1.0\"\ndefaultTargets = [\"Probe\"]\n\n[[lean_lib]]\nname = \"Probe\"\nglobs = [\"Probe.+\"]\n",
    )
    .expect("lakefile");
    std::fs::write(root.join("lean-toolchain"), "leanprover/lean4:v4.32.1\n").expect("toolchain");
    std::fs::write(root.join("Probe/Main.lean"), PROBE_MODULE).expect("module");
    let lake = |arguments: &[&str]| {
        std::process::Command::new(toolchain.lake.path.as_std_path())
            .args(arguments)
            .current_dir(root)
            .env(
                "PATH",
                format!(
                    "{}:{}",
                    toolchain.root.join("bin"),
                    std::env::var("PATH").unwrap_or_default()
                ),
            )
            .output()
            .expect("run lake")
    };
    let built = lake(&["build"]);
    assert!(
        built.status.success(),
        "{}",
        String::from_utf8_lossy(&built.stdout)
    );
    let roots: Vec<String> = roots.iter().map(|root| (*root).to_owned()).collect();
    let driver = lcnf::driver(&"0".repeat(32), &roots, &["Probe.Main".to_owned()]).expect("driver");
    std::fs::write(root.join("Extract.lean"), &driver.text).expect("driver");
    let output = lake(&["env", "lean", "Extract.lean"]);
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    assert!(output.status.success(), "{stdout}");
    assert_eq!(
        stdout.lines().count(),
        1,
        "one record and nothing else: {stdout}"
    );
    serde_json::from_str(&stdout).expect("record JSON")
}

/// Every `kind` tag the value carries, at any depth.
fn kinds(value: &serde_json::Value, out: &mut BTreeSet<String>) {
    match value {
        serde_json::Value::Object(object) => {
            if let Some(serde_json::Value::String(kind)) = object.get("kind") {
                out.insert(kind.clone());
            }
            object.values().for_each(|inner| kinds(inner, out));
        }
        serde_json::Value::Array(items) => items.iter().for_each(|inner| kinds(inner, out)),
        serde_json::Value::Null
        | serde_json::Value::Bool(_)
        | serde_json::Value::Number(_)
        | serde_json::Value::String(_) => {}
    }
}

/// Run the host over the probe record for one root whose eligibility plan
/// names exactly `members`.
fn probe_host(
    record: &serde_json::Value,
    root: &str,
    members: &[&str],
) -> Result<lcnf::CompilerInput, Rejection> {
    let roots = vec![format!("Probe.Main.{root}")];
    let report = ModuleReport {
        module: "Probe.Main".to_owned(),
        roots: vec![lexlean::production::RootReport {
            root: roots[0].clone(),
            declared_effects: Vec::new(),
            runtime: members
                .iter()
                .map(|member| lexlean::production::ClosureMember {
                    instance: format!("Probe.Main.{member}"),
                    declaration: format!("Probe.Main.{member}"),
                    construct: "declaration.definition".to_owned(),
                    type_arguments: Vec::new(),
                    path: vec![roots[0].clone()],
                })
                .collect(),
            types: std::collections::BTreeMap::new(),
            erased: BTreeSet::new(),
            constructs: std::collections::BTreeMap::new(),
            targets: Vec::new(),
        }],
    };
    let driver = lcnf::driver(&"0".repeat(32), &roots, &["Probe.Main".to_owned()]).expect("driver");
    let text = serde_json::to_string(record).expect("record");
    let restricted = mutate(&text, |value| value["roots"] = serde_json::json!(roots));
    lcnf::compiler_input(
        &driver,
        &restricted,
        &roots,
        &["Probe.Main".to_owned()],
        &[&report],
    )
}

#[allow(clippy::too_many_lines)]
pub fn run(id: &str) {
    match id {
        // §22.10: verification publishes one canonical compiler input whose
        // bytes do not depend on the project root.
        "NE-01" => {
            let record = committed_record();
            let input = extract(&record).expect("the committed record extracts");
            let bytes = input.to_file_bytes();
            let value: serde_json::Value = serde_json::from_slice(&bytes).expect("input JSON");
            support::assert_schema("compiler-input", "the production compiler input", &value);
            let committed = std::fs::read(
                support::repo_root()
                    .join("examples/production/expected/verify/production/compiler-input.json")
                    .as_std_path(),
            )
            .expect("the committed compiler input");
            assert_eq!(
                bytes, committed,
                "the committed input is the canonical form of the record"
            );
            assert_eq!(
                input.id(),
                lexlean::artifact::content_id::Sha256Digest::of(&committed)
            );
            // The input is a function of the program, not of Lean's unique
            // counter: renumbering every variable leaves it unchanged.
            let renumbered = record.replace("_uniq.", "_uniq.9");
            assert_ne!(renumbered, record);
            assert_eq!(
                extract(&renumbered).expect("renumbered").to_file_bytes(),
                bytes
            );
            if support::lean_backed("NE-01") {
                let first = P::copy_example("production");
                let second = P::copy_example("production");
                assert_ne!(first.root, second.root);
                let read = |project: &P| {
                    let verified = support::verify_ok(project);
                    let input = std::fs::read(
                        verified
                            .root
                            .join("production/compiler-input.json")
                            .as_std_path(),
                    )
                    .expect("published compiler input");
                    let path = verified.root.join("attestation.json");
                    support::assert_json_file_schema("attestation-v2", &path);
                    let attestation: serde_json::Value = serde_json::from_slice(
                        &std::fs::read(path.as_std_path()).expect("attestation"),
                    )
                    .expect("attestation JSON");
                    assert_eq!(attestation["spec"], "lexlean/attestation/2");
                    assert_eq!(
                        attestation["compiler_input"]["sha256"],
                        lexlean::artifact::content_id::Sha256Digest::of(&input).to_hex(),
                        "the attestation records the compiler input"
                    );
                    assert_eq!(
                        attestation["compiler_input"]["byte_length"],
                        input.len(),
                        "and its length"
                    );
                    input
                };
                let left = read(&first);
                assert_eq!(
                    left,
                    read(&second),
                    "the compiler input is root independent"
                );
                assert_eq!(
                    left, committed,
                    "pinned Lean reproduces the committed input"
                );
                // A project without a production root publishes no input,
                // and its language-1.2 attestation omits the field.
                let formal = P::copy_example("recursion");
                let verified = support::verify_ok(&formal);
                assert!(!verified.root.join("production").as_std_path().exists());
                assert!(!verified.root.join("extract").as_std_path().exists());
                let path = verified.root.join("attestation.json");
                support::assert_json_file_schema("attestation-v2", &path);
                let attestation: serde_json::Value =
                    serde_json::from_slice(&std::fs::read(path.as_std_path()).expect("read"))
                        .expect("attestation JSON");
                assert!(attestation.get("compiler_input").is_none());
            }
        }
        // §22.10: each root's closure is exactly its computational
        // dependencies, equal to the eligibility closure, complete, and
        // free of proofs, with its runtime members and instances.
        "NE-02" => {
            let input = extract(&committed_record()).expect("extracts");
            let closure = |root: &str| {
                input
                    .closures
                    .iter()
                    .find(|closure| closure.root == format!("Production.Main.{root}"))
                    .expect("the root's closure")
                    .clone()
            };
            let roots: Vec<&str> = input
                .closures
                .iter()
                .map(|closure| closure.root.as_str())
                .collect();
            let mut requested: Vec<&str> = input.roots.iter().map(String::as_str).collect();
            requested.sort_unstable();
            assert_eq!(roots, requested, "one closure per requested root");
            let halvings = closure("halvings");
            assert_eq!(
                halvings.declarations,
                ["Production.Kernel.countdown", "Production.Main.halvings"]
            );
            assert_eq!(
                halvings.runtime,
                ["Production.Kernel.LexLeanRuntime.subtract"]
            );
            assert_eq!(halvings.erased, ["Production.Kernel.countdown_decreases"]);
            assert_eq!(
                closure("shapeArea").declarations,
                ["Production.Kernel.area", "Production.Main.shapeArea"]
            );
            assert_eq!(
                closure("shapeArea").runtime,
                ["Production.Kernel.LexLeanRuntime.multiply"]
            );
            assert_eq!(
                closure("checkedSum").runtime,
                [
                    "Production.Main.LexLeanRuntime.checkedAdd",
                    "Production.Main.LexLeanRuntime.checkedFromInt",
                    "Production.Main.LexLeanRuntime.instFixedUInt32"
                ]
            );
            // The monomorphization plan travels with the closure.
            let head_or = closure("headOr");
            let first_or = head_or
                .instances
                .iter()
                .find(|instance| instance.declaration == "Production.Kernel.firstOr")
                .expect("the firstOr instance");
            assert_eq!(first_or.instance, "Production.Kernel.firstOr (Nat)");
            assert_eq!(first_or.type_arguments, ["Nat"]);
            let names: BTreeSet<&str> = input
                .declarations
                .iter()
                .map(|declaration| declaration.name.as_str())
                .collect();
            assert_eq!(names.len(), 16, "{names:?}");
            assert!(!names.contains("Production.Kernel.countdown_decreases"));
            assert_eq!(input.erased, ["Production.Kernel.countdown_decreases"]);
            // Recursion is a cycle of the use graph.
            let recursive: BTreeSet<&str> = input
                .declarations
                .iter()
                .filter(|declaration| declaration.recursive)
                .map(|declaration| declaration.name.as_str())
                .collect();
            assert_eq!(
                recursive,
                BTreeSet::from(["Production.Kernel.countdown", "Production.Kernel.total"])
            );
            // Every closure is the eligibility closure of its root, computed
            // independently from the semantic IR.
            let (_, reports) = production_inputs();
            for report in &reports {
                for root in &report.roots {
                    let lean = input
                        .closures
                        .iter()
                        .find(|closure| closure.root == root.root)
                        .expect("closure");
                    let analysis: BTreeSet<&str> = root
                        .runtime
                        .iter()
                        .map(|member| member.declaration.as_str())
                        .collect();
                    let extracted: BTreeSet<&str> =
                        lean.declarations.iter().map(String::as_str).collect();
                    assert_eq!(analysis, extracted, "{}", root.root);
                }
            }
            // Externals are Lean core constants only; runtime members are
            // translated and checked like every other member.
            for external in &input.externals {
                assert!(
                    external.module == "Init" || external.module.starts_with("Init."),
                    "{external:?}"
                );
            }
            // A proof-only dependency the eligibility analysis did not erase
            // is a disagreement, not a silent addition.
            rejected(
                &mutate(&committed_record(), |value| {
                    let proof = constant_mut(value, "Production.Kernel.countdown_decreases");
                    proof["kind"] = "definition".into();
                    proof["original_kind"] = "definition".into();
                }),
                "the proof-only dependencies of `Production.Main.halvings` differ",
            );
            let (_, mut reports) = production_inputs();
            for report in &mut reports {
                for root in &mut report.roots {
                    root.erased.clear();
                }
            }
            rejected_by(
                extract_with(&committed_record(), &reports),
                "the proof-only dependencies of `Production.Main.halvings` differ",
            );
            // A runtime member's dependencies are checked: a noncomputable
            // constant behind one is refused.
            rejected(
                &mutate(&committed_record(), |value| {
                    for external in value["externals"].as_array_mut().expect("externals") {
                        if external["name"] == "instSubNat" {
                            external["computable"] = false.into();
                        }
                    }
                }),
                "`instSubNat` is noncomputable or has no compiled code",
            );
        }
        // §22.10: every rejection class fails closed with LLV7011, and the
        // adapter classifies real Lean constants of every kind.
        "NE-03" => {
            let record = committed_record();
            // A project module is imported in full, so a constant of each
            // kind reports the kind it was declared with unchanged (Lean's
            // `ConstantKind` has no unsafe definition).
            let untranslated = |kind: &'static str| {
                mutate(&record, move |value| {
                    let area = constant_mut(value, "Production.Kernel.area");
                    area["kind"] = kind.into();
                    area["original_kind"] = match kind {
                        "unsafe-definition" => "definition",
                        other => other,
                    }
                    .into();
                    area["declaration"] = serde_json::Value::Null;
                })
            };
            rejected(
                &untranslated("opaque"),
                "`Production.Kernel.area` is an opaque",
            );
            rejected(
                &untranslated("axiom"),
                "`Production.Kernel.area` is an axiom",
            );
            rejected(
                &untranslated("partial-definition"),
                "`Production.Kernel.area` is a partial-definition",
            );
            rejected(
                &untranslated("unsafe-definition"),
                "`Production.Kernel.area` is unsafe",
            );
            rejected(
                &mutate(&record, |value| {
                    let area = constant_mut(value, "Production.Kernel.area");
                    area["computable"] = false.into();
                    area["declaration"] = serde_json::Value::Null;
                }),
                "`Production.Kernel.area` is noncomputable",
            );
            rejected(
                &mutate(&record, |value| {
                    constant_mut(value, "Production.Kernel.area")["kind"] = "opaque".into();
                }),
                "`Production.Kernel.area` carries a translation, but it is an opaque",
            );
            rejected(
                &mutate(&record, |value| {
                    constant_mut(value, "Production.Kernel.area")["declaration"]["safe"] =
                        false.into();
                }),
                "`Production.Kernel.area` is unsafe",
            );
            rejected(
                &mutate(&record, |value| {
                    constant_mut(value, "Production.Kernel.area")["declaration"]["value"] =
                        serde_json::json!({"kind": "extern"});
                }),
                "unsupported compiler form: `Production.Kernel.area` is implemented externally",
            );
            rejected(
                &mutate(&record, |value| {
                    constant_mut(value, "Production.Kernel.area")["declaration"]["type"] =
                        serde_json::json!({"kind": "unsupported", "expression": "lambda"});
                }),
                "unsupported compiler form: the LCNF type `lambda`",
            );
            rejected(
                &mutate(&record, |value| {
                    for external in value["externals"].as_array_mut().expect("externals") {
                        if external["name"] == "instMulNat" {
                            external["module"] = "Std.Data.HashMap.Basic".into();
                        }
                    }
                }),
                "unresolved dependency: `instMulNat`",
            );
            let external = |edit: fn(&mut serde_json::Value)| {
                mutate(&record, move |value| {
                    for external in value["externals"].as_array_mut().expect("externals") {
                        if external["name"] == "instMulNat" {
                            edit(external);
                        }
                    }
                })
            };
            // A constant of Lean's core declared an axiom or an opaque is
            // refused whatever view exports it.
            rejected(
                &external(|external| {
                    external["kind"] = "axiom".into();
                    external["original_kind"] = "axiom".into();
                }),
                "`instMulNat` is an axiom",
            );
            rejected(
                &external(|external| {
                    external["kind"] = "opaque".into();
                    external["original_kind"] = "opaque".into();
                }),
                "`instMulNat` is an opaque",
            );
            rejected(
                &external(|external| {
                    external["kind"] = "axiom".into();
                    external["original_kind"] = "opaque".into();
                }),
                "reports `instMulNat` as an axiom declared as an opaque",
            );
            rejected(
                &external(|external| {
                    external["kind"] = "unsafe-definition".into();
                }),
                "`instMulNat` is an unsafe-definition",
            );
            // A definition its module exports as an axiom is the definition
            // it was declared as only while it is computable and Lean's
            // compiler holds its code.
            let weakened = extract(&external(|external| {
                external["kind"] = "axiom".into();
                external["generates_code"] = false.into();
            }))
            .expect("a definition exported as an axiom is admitted");
            assert_eq!(
                weakened,
                extract(&record).expect("the committed record"),
                "the admitted definition canonicalizes as the definition it was declared as"
            );
            rejected(
                &external(|external| {
                    external["kind"] = "axiom".into();
                    external["generates_code"] = false.into();
                    external["compiled"] = false.into();
                }),
                "`instMulNat` is noncomputable or has no compiled code",
            );
            rejected(
                &external(|external| {
                    external["kind"] = "axiom".into();
                    external["generates_code"] = false.into();
                    external["computable"] = false.into();
                }),
                "`instMulNat` is noncomputable or has no compiled code",
            );
            rejected(
                &external(|external| external["compiled"] = false.into()),
                "`instMulNat` is noncomputable or has no compiled code",
            );
            // A project constant reports the kind it was declared with.
            rejected(
                &mutate(&record, |value| {
                    constant_mut(value, "Production.Kernel.area")["original_kind"] =
                        "axiom".into();
                }),
                "reports the project constant `Production.Kernel.area` as a definition declared as an axiom",
            );
            // The `borrowed` annotation is an ownership hint LCNF type
            // equivalence looks through; it canonicalizes away, and any
            // other metadata is an unsupported form.
            assert_eq!(
                extract(&mutate(&record, |value| {
                    let ty =
                        &mut constant_mut(value, "Production.Kernel.area")["declaration"]["type"];
                    let inner = ty["domain"].clone();
                    ty["domain"] = serde_json::json!({"kind": "borrowed", "type": inner});
                }))
                .expect("a borrowed domain"),
                extract(&record).expect("the committed record"),
            );
            rejected(
                &mutate(&record, |value| {
                    constant_mut(value, "Production.Kernel.area")["declaration"]["type"] =
                        serde_json::json!({"kind": "unsupported", "expression": "metadata"});
                }),
                "unsupported compiler form: the LCNF type `metadata`",
            );
            rejected(
                &mutate(&record, |value| {
                    let area = constant_mut(value, "Production.Kernel.area");
                    let uses = area["declaration"]["uses"].as_array_mut().expect("uses");
                    uses.pop();
                }),
                "the extraction record of `Production.Kernel.area` lists uses that differ from its code",
            );
            rejected(
                &mutate(&record, |value| {
                    let copy = constant_mut(value, "Production.Kernel.area").clone();
                    value["constants"]
                        .as_array_mut()
                        .expect("constants")
                        .push(copy);
                }),
                "the extraction record reports a constant twice",
            );
            rejected(
                &mutate(&record, |value| {
                    constant_mut(value, "Production.Kernel.area")["kernel_uses"]
                        .as_array_mut()
                        .expect("kernel uses")
                        .push("Production.Kernel.unreported".into());
                }),
                "unresolved dependency: the kernel value of `Production.Kernel.area` names `Production.Kernel.unreported`",
            );
            rejected(
                &format!("{record}\nnoise\n"),
                "more than its single JSON record",
            );
            rejected(
                &mutate(&record, |value| value["surplus"] = true.into()),
                "the extraction record is malformed",
            );
            rejected(
                &mutate(&record, |value| {
                    value["roots"] = serde_json::json!(["Production.Main.sumAll"]);
                }),
                "the extraction answered roots",
            );
            // A variable bound in one alternative is not in scope in its
            // sibling.
            rejected(
                &mutate(&record, |value| {
                    let total = constant_mut(value, "Production.Kernel.total");
                    let code = &mut total["declaration"]["value"]["code"];
                    let bound = find_alternative_parameter(code).expect("a bound parameter");
                    *code = replace_sibling_return(code.clone(), &bound);
                }),
                "is used before it is bound",
            );
            if support::lean_backed("NE-03") {
                // Pinned Lean itself refuses a root that does not exist.
                let driver = lcnf::driver(
                    "0".repeat(32).as_str(),
                    &["LexLeanNoSuch.root".to_owned()],
                    &[],
                )
                .expect("driver");
                let (exit, output) = run_lean(&driver.text);
                assert_ne!(exit, 0, "{output}");
                assert_eq!(
                    lcnf::classify_failure(&driver, &output),
                    Rejection::Rejected("unknown root `LexLeanNoSuch.root`".to_owned()),
                    "{output}"
                );
                // The adapter's facts on real Lean: every kind of constant a
                // root can reach, and every base-phase LCNF form.
                let record = probe_extraction(&[
                    "Probe.Main.viaOpaque",
                    "Probe.Main.viaAxiom",
                    "Probe.Main.viaPartial",
                    "Probe.Main.risky",
                    "Probe.Main.viaExternal",
                    "Probe.Main.greeting",
                    "Probe.Main.joined",
                    "Probe.Main.impossible",
                    "Probe.Main.ordinal",
                    "Probe.Main.boxed",
                    "Probe.Main.parsed",
                    "Probe.Main.halver",
                ]);
                let fact = |name: &str| {
                    record["constants"]
                        .as_array()
                        .expect("constants")
                        .iter()
                        .find(|constant| constant["name"] == format!("Probe.Main.{name}"))
                        .unwrap_or_else(|| panic!("{name} is reported"))
                        .clone()
                };
                assert_eq!(fact("hidden")["kind"], "opaque");
                assert_eq!(fact("postulated")["kind"], "axiom");
                assert_eq!(fact("viaAxiom")["computable"], false);
                assert!(fact("viaAxiom")["declaration"].is_null());
                // Lean implements a partial definition as an opaque constant.
                assert_eq!(fact("spin")["kind"], "opaque");
                assert_eq!(fact("risky")["kind"], "unsafe-definition");
                assert!(fact("risky")["declaration"].is_null());
                assert_eq!(fact("external")["declaration"]["value"]["kind"], "extern");
                // `String.toInt?`'s module does not expose its body, so the
                // module system exports it as an axiom; Lean records the
                // definition it was declared as and holds its code.
                let core = record["externals"]
                    .as_array()
                    .expect("externals")
                    .iter()
                    .find(|external| external["name"] == "String.toInt?")
                    .expect("String.toInt? is reported")
                    .clone();
                assert_eq!(core["kind"], "axiom", "{core}");
                assert_eq!(core["original_kind"], "definition", "{core}");
                assert_eq!(core["compiled"], true, "{core}");
                // `Nat.div` borrows its arguments, and its LCNF type says so.
                let mut halver = BTreeSet::new();
                kinds(&fact("halver")["declaration"], &mut halver);
                assert!(halver.contains("borrowed"), "{halver:?}");
                let mut forms = BTreeSet::new();
                for name in ["greeting", "joined", "impossible", "ordinal", "boxed"] {
                    kinds(&fact(name)["declaration"]["value"], &mut forms);
                }
                for form in [
                    "join",
                    "jump",
                    "default",
                    "unreachable",
                    "string",
                    "erased",
                    "cases",
                    "fun",
                ] {
                    assert!(
                        forms.contains(form),
                        "base-phase LCNF produced `{form}`: {forms:?}"
                    );
                }
                // The host refuses each refused kind and canonicalizes every
                // form.
                rejected_by(
                    probe_host(&record, "viaOpaque", &["viaOpaque"]),
                    "`Probe.Main.hidden` is an opaque",
                );
                rejected_by(
                    probe_host(&record, "viaAxiom", &["viaAxiom"]),
                    "`Probe.Main.viaAxiom` is noncomputable",
                );
                rejected_by(
                    probe_host(&record, "viaPartial", &["viaPartial"]),
                    "`Probe.Main.spin` is an opaque",
                );
                rejected_by(
                    probe_host(&record, "risky", &["risky"]),
                    "`Probe.Main.risky` is unsafe",
                );
                rejected_by(
                    probe_host(&record, "viaExternal", &["viaExternal"]),
                    "unsupported compiler form: `Probe.Main.external` is implemented externally",
                );
                for root in [
                    "greeting",
                    "joined",
                    "impossible",
                    "ordinal",
                    "boxed",
                    "parsed",
                    "halver",
                ] {
                    let input = probe_host(&record, root, &[root]).unwrap_or_else(|rejection| {
                        panic!("`{root}` canonicalizes: {rejection:?}")
                    });
                    support::assert_schema(
                        "compiler-input",
                        "a hand-written compiler input",
                        &serde_json::from_slice(&input.to_file_bytes()).expect("JSON"),
                    );
                }
                // The admitted core definition is handed on as the definition
                // it was declared as, with its compiled code.
                let parsed = probe_host(&record, "parsed", &["parsed"]).expect("parsed");
                let core = parsed
                    .externals
                    .iter()
                    .find(|external| external.name == "String.toInt?")
                    .expect("String.toInt? is an external");
                assert_eq!(
                    (core.kind.as_str(), core.computable, core.generates_code),
                    ("definition", true, true)
                );
            }
        }
        // §22.10: the authority interface is closed, pinned, and probed.
        "NE-04" => {
            let authority = lcnf::authority().expect("the authority registry");
            assert_eq!(authority.lean_version, "4.32.1");
            assert_eq!(
                authority.lean_githash,
                "f054605aea4b840552cca2e725580bffd1e1b704"
            );
            let model =
                repo_model::Model::load(&support::repo_root().join("model").into_std_path_buf())
                    .expect("model");
            assert!(
                model
                    .authorities
                    .authority
                    .iter()
                    .any(|row| row.id == authority.authority),
                "the registry cites a model authority row"
            );
            assert!(
                model
                    .ledger
                    .claim
                    .iter()
                    .any(
                        |claim| claim.authority.as_deref() == Some(authority.authority.as_str())
                            && claim.level == repo_model::registry::Level::SomeTrue
                    ),
                "the authority's statement is a some-true ledger claim"
            );
            let adapter = lcnf::adapter().expect("adapter");
            assert!(
                !adapter.contains("--") && !adapter.contains("/-"),
                "the adapter has no comment"
            );
            // The only compiler operations are the base-phase translation,
            // the constructor type query, the code-generation query, and the
            // monad runner; every other LCNF row is a structure field.
            for call in &authority.call {
                if call.name.starts_with("Lean.Compiler.LCNF.") {
                    let segment = call.name.rsplit('.').next().expect("segment");
                    assert!(
                        ["toDecl", "toLCNFType", "shouldGenerateCode", "run"].contains(&segment)
                            || call.role.starts_with("a field of the LCNF"),
                        "`{}` is not a registered kind of compiler operation",
                        call.name
                    );
                }
            }
            // A Lean identity other than the pin is drift.
            let foreign = mutate(&committed_record(), |value| {
                value["lean"]["githash"] = "0000000000000000000000000000000000000000".into();
            });
            assert!(
                matches!(extract(&foreign), Err(Rejection::Drift(reason)) if reason.contains("pinned to 4.32.1"))
            );
            if support::lean_backed("NE-04") {
                // Every source identity matches the pinned toolchain.
                let toolchain = toolchain();
                for (source, sha256) in authority
                    .call
                    .iter()
                    .map(|call| (&call.source, &call.source_sha256))
                    .chain(
                        authority
                            .types
                            .iter()
                            .map(|row| (&row.source, &row.source_sha256)),
                    )
                {
                    let bytes = std::fs::read(toolchain.root.join(source).as_std_path())
                        .expect("pinned source");
                    assert_eq!(
                        &lexlean::artifact::content_id::Sha256Digest::of(&bytes).to_hex(),
                        sha256,
                        "{source}"
                    );
                }
                // Every probe and the registry check pass under pinned Lean.
                let driver = lcnf::driver("0".repeat(32).as_str(), &[], &[]).expect("driver");
                let (exit, output) = run_lean(&driver.text);
                assert_eq!(exit, 0, "{output}");
                assert_eq!(
                    output.lines().count(),
                    1,
                    "nothing but the record: {output}"
                );
                assert!(
                    output.contains("\"spec\":\"lexlean/lcnf-extraction/2\""),
                    "{output}"
                );
                let drift = |text: String, fragment: &str| {
                    let drifted = lcnf::Driver {
                        text,
                        ..driver.clone()
                    };
                    assert_ne!(drifted.text, driver.text, "{fragment}");
                    let (exit, output) = run_lean(&drifted.text);
                    let classified = if exit == 0 {
                        lcnf::compiler_input(&drifted, &output, &[], &[], &[])
                            .expect_err("a drifted run is refused")
                    } else {
                        lcnf::classify_failure(&drifted, &output)
                    };
                    match classified {
                        Rejection::Drift(reason) => {
                            assert!(reason.contains(fragment), "{fragment:?} in {reason}");
                        }
                        other => panic!("expected drift naming {fragment:?}, got {other:?}"),
                    }
                };
                // A changed result, default value, or binder kind is drift,
                // though definitional equality would accept the last two.
                drift(
                    driver.text.replacen(
                        "lexlean_signature Lean.Compiler.LCNF.shouldGenerateCode : Lean.Name → Lean.Core.CoreM Bool",
                        "lexlean_signature Lean.Compiler.LCNF.shouldGenerateCode : Lean.Name → Lean.Core.CoreM Nat",
                        1,
                    ),
                    "`Lean.Compiler.LCNF.shouldGenerateCode`",
                );
                drift(
                    driver.text.replacen(
                        "Lean.Name → optParam Bool Bool.false → Option Lean.ConstantInfo",
                        "Lean.Name → optParam Bool Bool.true → Option Lean.ConstantInfo",
                        1,
                    ),
                    "`Lean.Environment.find?`",
                );
                drift(
                    driver.text.replacen(
                        "lexlean_signature Lean.Elab.Command.liftCoreM : {α : Type}",
                        "lexlean_signature Lean.Elab.Command.liftCoreM : (α : Type)",
                        1,
                    ),
                    "`Lean.Elab.Command.liftCoreM`",
                );
                // A constructor list that is not Lean's is drift.
                drift(
                    driver.text.replacen(
                        "#[\"alt\", \"ctorAlt\", \"default\"]",
                        "#[\"alt\", \"default\"]",
                        1,
                    ),
                    "`Lean.Compiler.LCNF.Alt` has constructors",
                );
                // A call the registry does not list is drift: an adapter
                // that ran an LCNF pass would use one.
                drift(
                    driver
                        .text
                        .replacen("\"Lean.Compiler.LCNF.toDecl\".toName, ", "", 1),
                    "`Lean.Compiler.LCNF.toDecl` is used as call but registered as unregistered",
                );
                // A warning inside the adapter is drift, though Lean exits
                // successfully: the record must stand alone.
                drift(
                    driver.text.replacen(
                        "meta def bool (value : Bool) : String",
                        "@[deprecated \"probe\" (since := \"2020-01-01\")]\nmeta def bool (value : Bool) : String",
                        1,
                    ),
                    "the pinned extraction adapter no longer elaborates cleanly",
                );
            }
        }
        // §22.10: a dependency dropped by either side is caught.
        "NE-05" => {
            let record = committed_record();
            // Lean's side no longer reports `Production.Kernel.area`:
            // `shapeArea` still calls it.
            let dropped = mutate(&record, |value| {
                value["constants"]
                    .as_array_mut()
                    .expect("constants")
                    .retain(|constant| constant["name"] != "Production.Kernel.area");
            });
            rejected(
                &dropped,
                "dropped dependency: `Production.Main.shapeArea` uses `Production.Kernel.area`, which the extraction neither defines nor records",
            );
            // The eligibility side drops it: Lean's compiler still reaches it.
            let (_, mut reports) = production_inputs();
            for report in &mut reports {
                for root in &mut report.roots {
                    root.runtime
                        .retain(|member| member.declaration != "Production.Kernel.area");
                }
            }
            rejected_by(
                extract_with(&record, &reports),
                "dropped dependency: Lean's compiler reaches `Production.Kernel.area` from `Production.Main.shapeArea`",
            );
            // The eligibility side adds a member Lean never reaches.
            let (_, mut reports) = production_inputs();
            for report in &mut reports {
                for root in &mut report.roots {
                    if root.root == "Production.Main.checkedSum" {
                        let mut phantom = root.runtime[0].clone();
                        phantom.declaration = "Production.Kernel.area".to_owned();
                        phantom.instance = "Production.Kernel.area".to_owned();
                        root.runtime.push(phantom);
                    }
                }
            }
            rejected_by(
                extract_with(&record, &reports),
                "the production-eligibility closure of `Production.Main.checkedSum` realizes `Production.Kernel.area`, which Lean's compiler does not reach",
            );
        }
        // §22.10: a proof presented as a runtime member is rejected.
        "NE-06" => {
            let record = committed_record();
            let proof_as_runtime = mutate(&record, |value| {
                let area = constant_mut(value, "Production.Kernel.area");
                area["kind"] = "theorem".into();
                area["original_kind"] = "theorem".into();
                area["declaration"] = serde_json::Value::Null;
            });
            rejected(
                &proof_as_runtime,
                "proof-as-runtime dependency: the theorem `Production.Kernel.area` is in the runtime closure",
            );
            // The eligibility analysis erasing a member Lean realizes is the
            // same disagreement from the other side.
            let (_, mut reports) = production_inputs();
            for report in &mut reports {
                for root in &mut report.roots {
                    if root.root == "Production.Main.shapeArea" {
                        root.erased.insert("Production.Kernel.area".to_owned());
                    }
                }
            }
            rejected_by(
                extract_with(&record, &reports),
                "the proof-only dependencies of `Production.Main.shapeArea` differ",
            );
        }
        other => panic!("no extraction case is wired for {other}"),
    }
}

/// The id of a parameter bound by a constructor alternative of a `cases`
/// with at least two alternatives, anywhere in `code`.
fn find_alternative_parameter(code: &serde_json::Value) -> Option<String> {
    match code {
        serde_json::Value::Object(object) => {
            if object.get("kind") == Some(&serde_json::Value::String("cases".to_owned())) {
                let alternatives = object.get("alternatives")?.as_array()?;
                if alternatives.len() >= 2 {
                    for alternative in alternatives {
                        if let Some(parameter) = alternative["parameters"]
                            .as_array()
                            .and_then(|parameters| parameters.first())
                        {
                            return parameter["id"].as_str().map(str::to_owned);
                        }
                    }
                }
            }
            object.values().find_map(find_alternative_parameter)
        }
        serde_json::Value::Array(items) => items.iter().find_map(find_alternative_parameter),
        serde_json::Value::Null
        | serde_json::Value::Bool(_)
        | serde_json::Value::Number(_)
        | serde_json::Value::String(_) => None,
    }
}

/// Make the alternative of the `cases` that does not bind `bound` return it.
fn replace_sibling_return(code: serde_json::Value, bound: &str) -> serde_json::Value {
    match code {
        serde_json::Value::Object(mut object) => {
            if object.get("kind") == Some(&serde_json::Value::String("cases".to_owned())) {
                if let Some(alternatives) = object
                    .get_mut("alternatives")
                    .and_then(serde_json::Value::as_array_mut)
                {
                    let binds = |alternative: &serde_json::Value| {
                        alternative["parameters"]
                            .as_array()
                            .is_some_and(|parameters| {
                                parameters.iter().any(|parameter| parameter["id"] == bound)
                            })
                    };
                    if let Some(binder) = alternatives.iter().position(binds) {
                        // The binding alternative comes first, so only scope,
                        // not order, can refuse the sibling's reference.
                        if let Some(sibling) = alternatives
                            .iter()
                            .position(|alternative| !binds(alternative))
                        {
                            alternatives.swap(0, binder);
                            let sibling = if sibling == 0 { binder } else { sibling };
                            alternatives[sibling]["code"] =
                                serde_json::json!({"kind": "return", "id": bound});
                        }
                        return serde_json::Value::Object(object);
                    }
                }
            }
            serde_json::Value::Object(
                object
                    .into_iter()
                    .map(|(key, value)| (key, replace_sibling_return(value, bound)))
                    .collect(),
            )
        }
        serde_json::Value::Array(items) => serde_json::Value::Array(
            items
                .into_iter()
                .map(|value| replace_sibling_return(value, bound))
                .collect(),
        ),
        other => other,
    }
}
