import Lean
import LexLeanText













open Lean
def fail {α : Type} (msg : String) : IO α := throw (IO.userError msg)
/-- Names: dotted components; a numeric component is `#n`; in a string component `\\` is `\\\\`,
`.` is `\\d`, and a leading `#` is `\\h`. -/
def encSeg (s : String) : String :=
  let s := s.replace "\\" "\\\\" |>.replace "." "\\d"
  if s.startsWith "#" then "\\h" ++ (s.drop 1) else s

def encName (n : Name) : IO String := do
  let mut parts : Array String := #[]
  for c in n.components do
    match c with
    | .str _ s =>
      if s.isEmpty then fail s!"empty name segment in {n}"
      parts := parts.push (encSeg s)
    | .num _ k => parts := parts.push s!"#{k}"
    | .anonymous => pure ()
  return ".".intercalate parts.toList


open Lean
unsafe def main : IO Unit := do
  initSearchPath (← Lean.findSysroot)
  enableInitializersExecution
  let env ← importModules #[{module := `LexLeanText}] {} (loadExts := true)
  let names := env.header.moduleNames
  let mut compiled : Array String := #[]
  let mut total := 0
  let mut sparse : Array Json := #[]
  for (n, ci) in env.constants.map₁.toList do
    match env.getModuleIdxFor? n with
    | none => pure ()
    | some idx =>
      if (`LexLeanText).isPrefixOf names[idx.toNat]! then
        total := total + 1
        if (Lean.IR.findEnvDecl env n).isSome then compiled := compiled.push (← encName n)
        if isSparseCasesOn env n then
          match Lean.Meta.getSparseCasesOnInfoCore env n with
          | some info =>
            sparse := sparse.push (Json.mkObj [("name", Json.str (← encName n)), ("info", Json.mkObj [("ind_name", Json.str (← encName info.indName)),
              ("major_pos", info.majorPos), ("arity", info.arity),
              ("interesting_ctors", Json.arr (← info.insterestingCtors.mapM fun c => do pure (Json.str (← encName c))))])])
          | none => sparse := sparse.push (Json.mkObj [("name", Json.str (← encName n))])
  IO.println s!"{total} {compiled.size}"
  IO.FS.writeFile "sparse.json" (Json.arr sparse).compress
  IO.FS.writeFile "compiled.txt" ("\n".intercalate compiled.toList)
