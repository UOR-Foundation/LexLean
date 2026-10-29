import Lean
open Lean

/-! Replay facts: for one compiled module, one JSON line per constant the
module declares and per constant those declarations reach, in the canonical
text the replay cache keys on. Terms are alpha-equivalence classes (binder
names, binder annotations and metadata are not written, as the kernel does
not read them) written as a node table in which a shared subterm is one
node, so a proof term with heavy sharing stays linear. -/

namespace ReplayFacts

/-- A name with its structure kept: components are joined by `.`, a numeric
component is written `#n`, and `\`, `.` and `#` inside a string component are
escaped with `\`. -/
def nameStr (n : Name) : String :=
  let rec go : Name → List String
    | .anonymous => []
    | .str p s => go p ++ [((s.replace "\\" "\\\\").replace "." "\\.").replace "#" "\\#"]
    | .num p k => go p ++ [s!"#{k}"]
  ".".intercalate (go n)

partial def levelStr : Level → String
  | .zero => "0"
  | .succ l => s!"(s {levelStr l})"
  | .max a b => s!"(max {levelStr a} {levelStr b})"
  | .imax a b => s!"(imax {levelStr a} {levelStr b})"
  | .param n => s!"(p {nameStr n})"
  | .mvar _ => "(mvar)"

structure Table where
  ids : Std.HashMap Expr Nat := {}
  lines : Array String := #[]

abbrev M := StateM Table

def emit (e : Expr) (line : String) : M Nat := do
  let s ← get
  let id := s.lines.size
  set { s with ids := s.ids.insert e id, lines := s.lines.push line }
  return id

partial def node (e : Expr) : M Nat := do
  if let some i := (← get).ids[e]? then
    return i
  match e with
  | .mdata _ b =>
    let i ← node b
    modify fun s => { s with ids := s.ids.insert e i }
    return i
  | .bvar i => emit e s!"v {i}"
  | .fvar _ => emit e "fvar"
  | .mvar _ => emit e "mvar"
  | .sort l => emit e s!"S {levelStr l}"
  | .const n ls => emit e s!"C {nameStr n} [{",".intercalate (ls.map levelStr)}]"
  | .app f a =>
    let fi ← node f
    let ai ← node a
    emit e s!"A {fi} {ai}"
  | .lam _ t b _ =>
    let ti ← node t
    let bi ← node b
    emit e s!"L {ti} {bi}"
  | .forallE _ t b _ =>
    let ti ← node t
    let bi ← node b
    emit e s!"P {ti} {bi}"
  | .letE _ t v b nd =>
    let ti ← node t
    let vi ← node v
    let bi ← node b
    emit e s!"Z {ti} {vi} {bi} {nd}"
  | .lit (.natVal n) => emit e s!"N {n}"
  | .lit (.strVal s) => emit e s!"T {s.quote}"
  | .proj s i b =>
    let bi ← node b
    emit e s!"J {nameStr s} {i} {bi}"

/-- The node table of a term and the index of its root, as text. -/
def termText (e : Expr) : String :=
  let (root, t) := (node e).run {}
  "\n".intercalate (t.lines.toList ++ [s!"root {root}"])

def hintsStr : ReducibilityHints → String
  | .«opaque» => "opaque"
  | .«abbrev» => "abbrev"
  | .regular h => s!"regular {h}"

def safetyStr : DefinitionSafety → String
  | .«unsafe» => "unsafe"
  | .safe => "safe"
  | .«partial» => "partial"

def names (ns : List Name) : String := ",".intercalate (ns.map nameStr)

/-- Facts of one constant: kind and attributes (small text), value text (a
term, or for a recursor its rules), the constants it names. -/
def facts (c : ConstantInfo) : String × Option String × Array Name :=
  let used := c.type.getUsedConstants ++ (match c.value? (allowOpaque := true) with
    | some v => v.getUsedConstants
    | none => #[])
  match c with
  | .axiomInfo v => (s!"axiom unsafe={v.isUnsafe}", none, used)
  | .defnInfo v =>
    (s!"def hints={hintsStr v.hints} safety={safetyStr v.safety} all={names v.all}",
      some (termText v.value), used)
  | .thmInfo v => (s!"theorem all={names v.all}", some (termText v.value), used)
  | .opaqueInfo v =>
    (s!"opaque unsafe={v.isUnsafe} all={names v.all}", some (termText v.value), used)
  | .quotInfo v =>
    let kind := match v.kind with
      | .type => "type"
      | .ctor => "ctor"
      | .lift => "lift"
      | .ind => "ind"
    (s!"quot {kind}", none, used)
  | .inductInfo v =>
    (s!"inductive params={v.numParams} indices={v.numIndices} all={names v.all} ctors={names v.ctors} rec={v.isRec} unsafe={v.isUnsafe} refl={v.isReflexive} nested={v.numNested}",
      none, used ++ v.ctors.toArray ++ v.all.toArray)
  | .ctorInfo v =>
    (s!"constructor induct={nameStr v.induct} idx={v.cidx} params={v.numParams} fields={v.numFields} unsafe={v.isUnsafe}",
      none, used.push v.induct)
  | .recInfo v =>
    let rules := v.rules.map fun r => s!"rule {nameStr r.ctor} {r.nfields}\n{termText r.rhs}"
    let ruleUsed := v.rules.foldl (fun acc r => acc ++ r.rhs.getUsedConstants ++ #[r.ctor]) #[]
    (s!"recursor all={names v.all} params={v.numParams} indices={v.numIndices} motives={v.numMotives} minors={v.numMinors} k={v.k} unsafe={v.isUnsafe}",
      some ("\n".intercalate rules), used ++ ruleUsed ++ v.all.toArray)

def moduleOf (env : Environment) (n : Name) : String :=
  match env.getModuleIdxFor? n with
  | some i => (env.header.moduleNames[i.toNat]?.getD .anonymous).toString
  | none => ""

def line (env : Environment) (c : ConstantInfo) : String × Array Name :=
  let (attrs, value, used) := facts c
  let deps := ((used.filter (· != c.name)).qsort Name.lt).toList.eraseDups
  let j := Json.mkObj [
    ("name", Json.str (nameStr c.name)), ("module", Json.str (moduleOf env c.name)),
    ("level_params", Json.arr (c.levelParams.toArray.map (Json.str ∘ nameStr))),
    ("attrs", Json.str attrs), ("type", Json.str (termText c.type)),
    ("value", match value with | some v => Json.str v | none => Json.null),
    ("deps", Json.arr (deps.toArray.map (Json.str ∘ nameStr)))]
  (j.compress, deps.toArray)

end ReplayFacts

open ReplayFacts in
def main (args : List String) : IO UInt32 := do
  let some target := args.head? | IO.eprintln "usage: replay_facts MODULE"; return 2
  initSearchPath (← findSysroot)
  let env ← importModules #[{ module := target.toName }] {} (loadExts := false)
  let some idx := env.getModuleIdx? target.toName | IO.eprintln s!"module {target} not found"; return 2
  let own := env.header.moduleData[idx.toNat]!.constNames
  let mut seen : Std.HashSet Name := {}
  let mut work : Array Name := own.reverse
  let stdout ← IO.getStdout
  while work.size > 0 do
    let n := work.back!
    work := work.pop
    if seen.contains n then continue
    seen := seen.insert n
    let some c := env.find? n | continue
    let (l, deps) := line env c
    stdout.putStrLn l
    for d in deps do
      if !seen.contains d then work := work.push d
  return 0
