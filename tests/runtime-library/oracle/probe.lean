import Lean
import LexLeanRuntime.Portable
import LexLeanRuntime.Collections
import LexLeanRuntime.Models
import LexLeanRuntime.Reasoning
open Lean Meta


def isGrind (env : Environment) (ctx : Core.Context) (n : Name) : IO Bool := do
  let (r, _) ← (Lean.Meta.Grind.grindExt.isEMatchTheorem n : CoreM Bool).toIO ctx { env }
  pure r

def tagsOf (env : Environment) (ctx : Core.Context) (n : Name) : IO (Array String) := do
  let mut tags : Array String := #[]
  match Lean.Compiler.getInlineAttribute? env n with
  | some k => tags := tags.push (match k with | .inline => "inline" | .noinline => "noinline" | .macroInline => "macro_inline" | .inlineIfReduce => "inline_if_reduce" | .alwaysInline => "always_inline")
  | none => pure ()
  if Lean.Compiler.hasSpecializeAttribute env n then tags := tags.push "specialize"
  if Lean.Compiler.hasNospecializeAttribute env n then tags := tags.push "nospecialize"
  if Lean.isNoncomputable env n then tags := tags.push "noncomputable"
  if (Lean.Meta.Match.Extension.getMatcherInfo? env n).isSome then tags := tags.push "matcher"
  match Lean.Meta.getSparseCasesOnInfoCore env n with
  | some info => tags := tags.push s!"sparse({info.indName},{info.majorPos},{info.arity},{info.insterestingCtors})"
  | none => pure ()
  if (← isGrind env ctx n) then tags := tags.push "grind-ematch"
  return tags

unsafe def main : IO Unit := do
  initSearchPath (← Lean.findSysroot)
  enableInitializersExecution
  let env ← importModules #[{module := `LexLeanRuntime.Models}, {module := `LexLeanRuntime.Reasoning}] {} (loadExts := true)
  let names := env.header.moduleNames
  let ctx : Core.Context := { fileName := "<probe>", fileMap := default }
  let mut out : Array String := #[]
  for (n, _) in env.constants.map₁.toList do
    match env.getModuleIdxFor? n with
    | none => pure ()
    | some idx =>
      if (`LexLeanRuntime).isPrefixOf names[idx.toNat]! then
        let tags ← tagsOf env ctx n
        if !tags.isEmpty then out := out.push s!"{n} {tags}"
  IO.println ("\n".intercalate (out.qsort (· < ·)).toList)
