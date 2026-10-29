import Lean.CoreM
import Lean.Replay
import Lean.Util.FoldConsts

/-!
# Declaration-closure replay (issue #337 prototype)

`leanchecker` replays whole modules: it takes module names and has no
declaration filter. `Lean.Environment.replay` is already dependency-driven,
though: it sends each constant of the map it is given to the kernel after the
constants that constant mentions. So the declaration closure is a map built
from the constants a project mentions, their mentions, and so on.

    closure-replay --module M            replay M's own constants and the closure of what they mention
    closure-replay --import A,B D1 D2    replay the closure of declarations D1, D2 of A, B
    ... --all                            replay every constant the imports load
                                         (what `leanchecker --fresh` does), same code path
    ... --list                           print the modules the imports load, one per line, and stop

Every constant is read from the most private part of its module (the part that
carries proof bodies), as `leanchecker` does. This is a same-kernel replay, not
an independent checker.
-/

open Lean

/-- The parts of a module's `.olean`, the most private last. -/
def readParts (module : Name) : IO (Array (ModuleData × CompactedRegion)) := do
  let mFile ← findOLean module
  unless (← mFile.pathExists) do
    throw <| IO.userError s!"object file '{mFile}' of module {module} does not exist"
  let mut fnames := #[mFile]
  let sFile := OLeanLevel.server.adjustFileName mFile
  if (← sFile.pathExists) then
    fnames := fnames.push sFile
    let pFile := OLeanLevel.private.adjustFileName mFile
    if (← pFile.pathExists) then
      fnames := fnames.push pFile
  readModuleDataParts fnames

structure Index where
  modules : NameSet := {}
  constants : Std.HashMap Name ConstantInfo := {}

/-- Index the constants of `module` and of everything it imports. -/
partial def indexModule (module : Name) : StateT Index IO Unit := do
  if (← get).modules.contains module then return
  modify fun s => { s with modules := s.modules.insert module }
  let parts ← readParts module
  if h : parts.size = 0 then throw <| IO.userError s!"failed to read module data of {module}" else
  for imp in parts[0].1.imports do
    indexModule imp.module
  let last := parts[parts.size - 1].1
  for n in last.constNames, ci in last.constants do
    modify fun s => { s with constants := s.constants.insert n ci }

/-- The constants reachable from `roots` through what they mention, with the
members of an inductive family and the type a constructor or recursor belongs
to, which `Environment.replay` reads from its map. -/
def closureOf (index : Std.HashMap Name ConstantInfo) (roots : Array Name) :
    Std.HashMap Name ConstantInfo × Array Name := Id.run do
  let mut out : Std.HashMap Name ConstantInfo := {}
  let mut missing : Array Name := #[]
  let mut pending := roots
  while true do
    let some n := pending.back? | break
    pending := pending.pop
    if out.contains n then continue
    let some ci := index[n]? | missing := missing.push n; continue
    out := out.insert n ci
    for u in ci.getUsedConstantsAsSet do
      unless out.contains u do pending := pending.push u
    match ci with
    | .inductInfo i => pending := (pending ++ i.all.toArray) ++ i.ctors.toArray
    | .ctorInfo c => pending := pending.push c.induct
    | .recInfo r => pending := pending ++ r.all.toArray
    | _ => pure ()
  return (out, missing)

def splitCommas (s : String) : Array Name :=
  (s.splitOn ",").toArray.filterMap fun p => if p.isEmpty then none else some p.toName

unsafe def main (args : List String) : IO UInt32 := do
  initSearchPath (← findSysroot)
  let t0 ← IO.monoMsNow
  let mut mode : Option (Name ⊕ Array Name) := none
  let mut all := false
  let mut list := false
  let mut decls : Array Name := #[]
  let mut rest := args
  while true do
    match rest with
    | "--module" :: m :: tl => mode := some (.inl m.toName); rest := tl
    | "--import" :: ms :: tl => mode := some (.inr (splitCommas ms)); rest := tl
    | "--all" :: tl => all := true; rest := tl
    | "--list" :: tl => list := true; rest := tl
    | d :: tl => decls := decls.push d.toName; rest := tl
    | [] => break
  let (index, roots) ← match mode with
    | none => throw <| IO.userError "usage: closure-replay (--module M | --import A,B DECL...) [--all]"
    | some (.inl m) =>
      let parts ← readParts m
      if h : parts.size = 0 then throw <| IO.userError s!"failed to read module data of {m}" else
      let idx ← (parts[0].1.imports.forM fun imp => indexModule imp.module).run {} |>.map (·.2)
      let last := parts[parts.size - 1].1
      -- The module's own constants join the map, so they are replayed with what they mention.
      let mut idx := idx
      for n in last.constNames, ci in last.constants do
        idx := { idx with constants := idx.constants.insert n ci }
      pure (idx, last.constNames)
    | some (.inr ms) =>
      let idx ← (ms.forM indexModule).run {} |>.map (·.2)
      pure (idx, decls)
  if list then
    for m in index.modules.toArray.qsort Name.lt do IO.println m
    return 0
  let t1 ← IO.monoMsNow
  let (closure, missing) :=
    if all then (index.constants, #[]) else closureOf index.constants roots
  let t2 ← IO.monoMsNow
  IO.println s!"modules={index.modules.size} indexed={index.constants.size} roots={roots.size} closure={closure.size} missing={missing.size}"
  unless missing.isEmpty do
    IO.eprintln s!"closure-replay: not in the loaded modules: {missing.toList.take 5}"
    return 1
  let env ← mkEmptyEnvironment
  try
    discard <| env.replay closure
  catch e =>
    IO.eprintln s!"closure-replay found a problem: {e}"
    return 1
  let t3 ← IO.monoMsNow
  IO.println s!"index_ms={t1 - t0} closure_ms={t2 - t1} replay_ms={t3 - t2}"
  return 0
