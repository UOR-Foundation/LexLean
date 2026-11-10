//! Native C emission and whole-group reconstruction regressions for RT-03.

use lexlean::backend::runtime;

/// Object-only verification does not exercise Lean's native declaration
/// ordering. Use a fresh native build, not the shared object-code cache.
pub(super) fn check() {
    groups();
    let root = tempfile::tempdir().expect("native runtime workspace");
    let modules = runtime::modules().expect("the shipped runtime");
    for module in modules {
        let path = root
            .path()
            .join(format!("{}.lean", module.name.replace('.', "/")));
        std::fs::create_dir_all(path.parent().expect("module parent")).expect("mkdir");
        // Fail before C emission on the original defect, rather than waiting
        // for Lean's repeated assertion failures to exhaust the time budget.
        let mut text = module.text.replacen(
            "import Lean\n",
            "import Lean\npublic meta import Lean.Compiler.LCNF.PhaseExt\n",
            1,
        );
        text.push_str(
            "\nrun_meta do\n  \
             let (names, seen) := Lean.Compiler.LCNF.declOrderExt.getState (← Lean.getEnv)\n  \
             unless names.length == seen.size do\n    \
             throwError \"duplicate native declaration order\"\n",
        );
        std::fs::write(path, text).expect("write runtime");
    }
    std::fs::write(
        root.path().join("lakefile.toml"),
        "name = \"native_runtime\"\nversion = \"0.1.0\"\n\n\
         [[lean_lib]]\nname = \"LexLeanRuntime\"\nmoreLeanArgs = [\"-j2\"]\n\n\
         [[lean_exe]]\nname = \"nativeRuntime\"\nroot = \"NativeRuntime\"\n",
    )
    .expect("write Lake project");
    std::fs::write(
        root.path().join("lean-toolchain"),
        format!("{}\n", lexlean::LEAN_TOOLCHAIN),
    )
    .expect("write toolchain");
    std::fs::write(
        root.path().join("NativeRuntime.lean"),
        "import LexLeanRuntime.Laws\n\n\
         def main : IO Unit := do\n  \
         unless LexLeanLaws.sumNat (fun n : Nat => n) [1, 2, 3, 5] == 11 do\n    \
         throw (IO.userError \"sumNat mismatch\")\n  \
         unless LexLeanCollections.lessThan (3 : UInt32) 5 do\n    \
         throw (IO.userError \"lessThan mismatch\")\n  \
         if LexLeanCollections.lessThan (5 : UInt32) 3 then\n    \
         throw (IO.userError \"lessThan reverse mismatch\")\n  \
         IO.println \"native runtime passed\"\n",
    )
    .expect("write native entry");
    let mut command = crate::preservation::pinned_with("lake", root.path(), &[]);
    command
        .args(["build", "nativeRuntime"])
        .env("LEAN_NUM_THREADS", "2");
    let result = crate::oracle::run(command, std::time::Duration::from_secs(180))
        .expect("bounded native runtime build");
    assert_eq!(result.status, Some(0), "{}\n{}", result.stdout, result.stderr);
    let executable = root
        .path()
        .join(".lake/build/bin")
        .join(format!("nativeRuntime{}", std::env::consts::EXE_SUFFIX));
    let result = crate::oracle::run(
        std::process::Command::new(executable),
        std::time::Duration::from_secs(30),
    )
    .expect("bounded native runtime execution");
    assert_eq!(result.status, Some(0), "{}\n{}", result.stdout, result.stderr);
    assert_eq!(result.stdout.trim(), "native runtime passed");
}

fn groups() {
    let root = tempfile::tempdir().expect("native groups workspace");
    let text = format!(
        "{}{}{}",
        GROUP_HEADER,
        lexlean::backend::core_v2::LEAN_CORE_DECODER_V2,
        GROUP_SUFFIX,
    );
    let source = root.path().join("NativeGroups.lean");
    std::fs::write(&source, text).expect("write group regression");
    let mut command = crate::preservation::pinned_with("lean", root.path(), &[]);
    command
        .arg("-j2")
        .arg("-c")
        .arg(root.path().join("NativeGroups.c"))
        .arg(source);
    let result = crate::oracle::run(command, std::time::Duration::from_secs(60))
        .expect("bounded native group regression");
    assert_eq!(result.status, Some(0), "{}\n{}", result.stdout, result.stderr);
    assert_eq!(result.stdout.matches("PASS ").count(), 7);
}

const GROUP_HEADER: &str = r#"module
public meta import Lean.Compiler.LCNF.PhaseExt
import Lean
import Lean.Compiler.LCNF.Main
import Lean.Compiler.LCNF.PhaseExt
import all Lean.Meta.Constructions.SparseCasesOn
set_option Elab.async false
set_option maxRecDepth 100000
open Lean

"#;

const GROUP_SUFFIX: &str = r#"public section
mutual
unsafe def sourceEven : Nat → Bool
  | 0 => true
  | n+1 => sourceOdd n
unsafe def sourceOdd : Nat → Bool
  | 0 => false
  | n+1 => sourceEven n
end
namespace LexLeanCore.Runtime
private def clonePair (base : Name) (safety : DefinitionSafety) (trivial := false) : CoreM (Array Name) := do
  let old := #[`sourceEven, `sourceOdd]
  let names := #[base ++ `even, base ++ `odd]
  let mut ds := []
  for i in [:2] do
    let .defnInfo info ← getConstInfo old[i]! | throwError "not defn"
    let value := info.value.replace fun e => match e with
      | .const n us => if n == old[0]! then some (.const names[0]! us)
          else if n == old[1]! then some (.const names[1]! us) else none
      | _ => none
    let value := if trivial then mkLambda `n .default (mkConst ``Nat) (mkConst ``Bool.true) else value
    ds := ds.concat { info with name := names[i]!, value, safety, all := names.toList }
  Lean.addDecl (.mutualDefnDecl ds)
  return names
private def rows (names : Array Name) (safety : String) : Array Json :=
  names.map fun n => Json.mkObj [("name", toJson n.toString), ("kind", toJson "definition"),
    ("safety", toJson safety), ("compiled", toJson true), ("all", toJson (names.map Name.toString))]
run_meta do
  for (base, safety, tag) in #[(`reviewPartial, DefinitionSafety.partial, "partial"),
      (`reviewUnsafe, DefinitionSafety.unsafe, "unsafe")] do
    let names ← clonePair base safety
    compileRows (rows names tag)
    let before := Lean.Compiler.LCNF.declOrderExt.getState (← getEnv)
    unless names.all before.2.contains do throwError "missing mutual member"
    compileRows (rows names tag)
    let after := Lean.Compiler.LCNF.declOrderExt.getState (← getEnv)
    unless before.1 == after.1 do throwError "repeat changed order"
    logInfo m!"PASS whole group and idempotence: {base}"
run_meta do
  let names ← clonePair `reviewMixed .unsafe true
  Lean.compileDecls #[names[0]!]
  let before := Lean.Compiler.LCNF.declOrderExt.getState (← getEnv)
  let mut refused := false
  try compileRows (rows names "unsafe")
  catch e =>
    if (← e.toMessageData.toString).contains "partially compiled" then refused := true
    else throw e
  unless refused do throwError "mixed group was not refused"
  let after := Lean.Compiler.LCNF.declOrderExt.getState (← getEnv)
  unless before.1 == after.1 do throwError "mixed refusal mutated order"
  logInfo "PASS mixed group refused without compilation"
end LexLeanCore.Runtime

public def proofOnly : True := True.intro
public def typeOnly : Type := Nat
public def regularValue : Nat := 7
namespace LexLeanCore.Runtime
run_meta do
  let before := Lean.Compiler.LCNF.declOrderExt.getState (← getEnv)
  compileGroup #[`regularValue, `proofOnly, `typeOnly]
  let after := Lean.Compiler.LCNF.declOrderExt.getState (← getEnv)
  unless before.1 == after.1 do throwError "no-code members induced recompilation"
  compileGroup #[`proofOnly, `typeOnly]
  let final := Lean.Compiler.LCNF.declOrderExt.getState (← getEnv)
  unless after.1 == final.1 do throwError "all-no-code group changed order"
  logInfo "PASS no-code members excluded, mixed and all-no-code groups"
end LexLeanCore.Runtime
namespace LexLeanCore.Runtime
run_meta do
  let names ← clonePair `reviewSignatureOnly .unsafe true
  let some sig ← Lean.Compiler.LCNF.getImpureSignature? `sourceEven | throwError "missing donor signature"
  modifyEnv fun env => Lean.Compiler.LCNF.impureSigExt.addEntry env { sig with name := names[0]! }
  let before := Lean.Compiler.LCNF.declOrderExt.getState (← getEnv)
  unless !before.2.contains names[0]! do throwError "unexpected local final entry"
  compileRows (rows names "unsafe")
  let after := Lean.Compiler.LCNF.declOrderExt.getState (← getEnv)
  unless names.all after.2.contains do throwError "signature suppressed required compilation"
  logInfo "PASS signature alone does not suppress compilation"
end LexLeanCore.Runtime
namespace LexLeanCore.Runtime
run_meta do
  let names ← clonePair `reviewMixedNoCode .unsafe true
  Lean.compileDecls #[names[0]!]
  let before := Lean.Compiler.LCNF.declOrderExt.getState (← getEnv)
  let mut refused := false
  try compileGroup (names ++ #[`proofOnly, `typeOnly])
  catch e =>
    if (← e.toMessageData.toString).contains "partially compiled" then refused := true
    else throw e
  unless refused do throwError "mixed group with no-code members was not refused"
  let after := Lean.Compiler.LCNF.declOrderExt.getState (← getEnv)
  unless before.1 == after.1 do throwError "mixed no-code refusal mutated order"
  logInfo "PASS mixed group plus no-code members refused without mutation"
end LexLeanCore.Runtime
"#;
