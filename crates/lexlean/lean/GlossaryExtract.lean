/-
  LexLean glossary extractor (SPEC.md §13.6-§13.8, issue #335).

  Reads declarations from an imported Lean environment and prints, as one
  canonical JSON document, a record per declaration: the LSE translation of
  its type, or the reason it cannot be translated. It decides nothing about
  names or files; `lexlean deps glossary` does that from this document.

  Invocation:  lean --run Extract.lean -- (--modules M ... | --names N ...)

  Determinism: records are sorted by Lean name; JSON objects print with
  sorted keys; no clock, path, or hash-map iteration order reaches the output.
-/
import Lean
open Lean Meta

namespace LexLeanGlossary

/-- A refusal: a stable reason code and a human detail. -/
structure Refusal where
  reason : String
  detail : String

structure TrState where
  refs : NameSet := {}
  erased : Nat := 0
  usedLocals : Std.HashSet String := {}
  fvarNames : Std.HashMap FVarId String := {}
  levelNames : Std.HashMap Name String := {}

abbrev TrM := StateRefT TrState (ExceptT Refusal MetaM)

def refuse {α} (reason detail : String) : TrM α := throwThe Refusal ⟨reason, detail⟩

/-- Lean constants that denote core lexicon entries, by qualified entry ID. -/
def coreEntries : List (Name × String) :=
  [(`Eq, "lexlean.core::eq"), (`And, "lexlean.core::land"),
   (`Or, "lexlean.core::lor"), (`Not, "lexlean.core::lnot"),
   (`Iff, "lexlean.core::iff"), (`Exists, "lexlean.core::exists-op")]

/-- Elaboration annotations that are identity on the type; erased. -/
def annotationNames : List Name := [`outParam, `semiOutParam, `optParam, `autoParam]

/-- Greek letters to the Latin letter they are conventionally read as, so that
`α β γ` stay distinct and readable in binder names; anything else outside
ASCII letters and digits is dropped. -/
def translit (c : Char) : String :=
  match c with
  | 'α' => "a" | 'β' => "b" | 'γ' => "g" | 'δ' => "d" | 'ε' => "e" | 'ζ' => "z"
  | 'η' => "h" | 'θ' => "t" | 'ι' => "i" | 'κ' => "k" | 'λ' => "l" | 'μ' => "m"
  | 'ν' => "n" | 'ξ' => "x" | 'ο' => "o" | 'π' => "p" | 'ρ' => "r" | 'σ' => "s"
  | 'τ' => "t" | 'υ' => "u" | 'φ' => "f" | 'χ' => "c" | 'ψ' => "y" | 'ω' => "w"
  | 'Γ' => "G" | 'Δ' => "D" | 'Θ' => "T" | 'Λ' => "L" | 'Ξ' => "X" | 'Π' => "P"
  | 'Σ' => "S" | 'Φ' => "F" | 'Ψ' => "Y" | 'Ω' => "W"
  | c => if c.isAlphanum && c.toNat < 128 then c.toString else ""

def asciiIdent (s : String) : String := String.join (s.toList.map translit)

/-- A fresh LSE identifier (`[A-Za-z][A-Za-z0-9]*`) from a Lean binder name. -/
def freshLocal (base : String) : TrM String := do
  let mut b := asciiIdent base
  if b.isEmpty || !b.front.isAlpha then b := "x" ++ b
  let st ← get
  let mut cand := b
  let mut i := 1
  while st.usedLocals.contains cand do
    cand := b ++ toString i
    i := i + 1
  modify fun s => { s with usedLocals := s.usedLocals.insert cand }
  return cand

def binderBase (n : Name) : String :=
  if n.isInternal || n.hasMacroScopes then "x" else n.eraseMacroScopes.toString

partial def trLevel (l : Level) : TrM String := do
  match l with
  | .zero => return "0"
  | .succ _ =>
    -- count a pure numeral
    let rec numeral : Level → Option Nat
      | .zero => some 0
      | .succ l => (· + 1) <$> numeral l
      | _ => none
    match numeral l with
    | some n => return toString n
    | none =>
      let .succ l' := l | unreachable!
      return s!"(succ {← trLevel l'})"
  | .max a b => return s!"(max {← trLevel a} {← trLevel b})"
  | .imax a b => return s!"(imax {← trLevel a} {← trLevel b})"
  | .param n =>
    match (← get).levelNames[n]? with
    | some s => return s
    | none => refuse "level-parameter" s!"unbound universe parameter {n}"
  | .mvar _ => refuse "level-metavariable" "universe metavariable"

/-- `Sort l` as an LSE sort: `Prop`, or `Type u` when `l` is a successor. -/
partial def predLevel? (l : Level) : Option Level :=
  match l.normalize with
  | .succ l' => some l'
  | .max a b => do return mkLevelMax' (← predLevel? a) (← predLevel? b)
  | _ => none

def trSort (l : Level) : TrM String := do
  let l := l.normalize
  if l.isZero then return "(sort prop)"
  match predLevel? l with
  | some p => return s!"(sort (type {← trLevel p}))"
  | none => refuse "sort-polymorphic" s!"Sort {l} is neither Prop nor Type u"

def modeOf (bi : BinderInfo) : TrM String :=
  match bi with
  | .default => return "explicit"
  | .implicit => return "implicit"
  | .instImplicit => return "instance"
  | .strictImplicit => refuse "strict-implicit-binder" "a ⦃strict implicit⦄ binder has no LSE binder mode"

def isExplicitBi : BinderInfo → Bool
  | .default => true
  | _ => false

def noteRef (n : Name) : TrM Unit := modify fun s => { s with refs := s.refs.insert n }

mutual
partial def tr (e : Expr) : TrM String := do
  match e with
  | .mdata _ b => tr b
  | .sort l => trSort l
  | .fvar id =>
    match (← get).fvarNames[id]? with
    | some s => return s!"(local {s})"
    | none => refuse "internal" "free variable outside binder scope"
  | .const .. | .app .. => trApp e
  | .lam .. => trBinders true e
  | .forallE .. => trBinders false e
  | .letE n t v b _ =>
    let ts ← tr t
    let vs ← tr v
    let nm ← freshLocal (binderBase n)
    withLetDecl n t v fun x => do
      modify fun s => { s with fvarNames := s.fvarNames.insert x.fvarId! nm }
      let bs ← tr (b.instantiate1 x)
      return s!"(let {nm} {ts} {vs} {bs})"
  | .lit (.natVal n) => return s!"(nat {n})"
  | .lit (.strVal _) => refuse "string-literal" "string literals are outside LSE"
  | .proj .. => refuse "primitive-projection" "Expr.proj is outside LSE"
  | .bvar _ => refuse "internal" "loose bound variable"
  | .mvar _ => refuse "internal" "metavariable"

partial def trConstHead (n : Name) (ls : List Level) : TrM String := do
  match coreEntries.lookup n with
  | some id =>
    -- The core entries are Type-level: `Eq`/`Exists` instantiated at `Sort 0`
    -- or at a non-successor level are different constants of the lexicon.
    if n == `Eq || n == `Exists then
      match ls with
      | [l] =>
        if (predLevel? l).isNone then
          refuse "core-level-mismatch" s!"{n} at level {l} is not the Type-level core entry"
      | _ => pure ()
    return s!"(const {id})"
  | none =>
    noteRef n
    let ci ← getConstInfo n
    if ci.levelParams.isEmpty then
      return s!"(const @{n})"
    else
      let args ← ls.mapM trLevel
      return s!"(const @{n} ({" ".intercalate args}))"

partial def trApp (e : Expr) : TrM String := do
  let f := e.getAppFn
  let args := e.getAppArgs
  -- numerals
  if let .const ``OfNat.ofNat _ := f then
    if args.size == 3 then
      if let .lit (.natVal n) := args[1]! then
        return s!"(nat {n})"
  -- identity annotations
  if let .const n _ := f then
    if annotationNames.contains n && args.size ≥ 1 && (n != `optParam && n != `autoParam || args.size == 2) then
      modify fun s => { s with erased := s.erased + 1 }
      return ← tr args[0]!
  let head ← match f with
    | .const n ls => trConstHead n ls
    | _ => tr f
  if args.isEmpty then return head
  let mut ty ← inferType f
  let mut explicitArgs : Array String := #[]
  for a in args do
    let ty' ← whnf ty
    let .forallE _ _ b bi := ty'
      | refuse "unsupported-expression" "application of a non-function"
    if isExplicitBi bi then
      explicitArgs := explicitArgs.push (← tr a)
    ty := b.instantiate1 a
  if explicitArgs.isEmpty then return head
  return s!"(app {head} {" ".intercalate explicitArgs.toList})"

partial def trBinders (isLam : Bool) (e : Expr) : TrM String := do
  let saved ← get
  let r ← go e #[]
  -- scope of local names ends with the binder group; keep `usedLocals`
  -- growing so names stay unique across the whole signature.
  modify fun s => { s with fvarNames := saved.fvarNames }
  return r
where
  go (e : Expr) (acc : Array String) : TrM String := do
    match e, isLam with
    | .lam n t b bi, true => step n t b bi acc
    | .forallE n t b bi, false => step n t b bi acc
    | _, _ =>
      let body ← tr e
      return s!"({if isLam then "lam" else "pi"} ({" ".intercalate acc.toList}) {body})"
  step (n : Name) (t b : Expr) (bi : BinderInfo) (acc : Array String) : TrM String := do
    let mode ← modeOf bi
    let ts ← tr t
    let nm ← freshLocal (binderBase n)
    withLocalDecl n bi t fun x => do
      modify fun s => { s with fvarNames := s.fvarNames.insert x.fvarId! nm }
      go (b.instantiate1 x) (acc.push s!"({mode} {nm} {ts})")
end

/-- What an entry denotes in the lexicon's terms. -/
inductive Kind | proof | type | predicate | term
  deriving Inhabited

def Kind.str : Kind → String
  | .proof => "proof" | .type => "type" | .predicate => "predicate" | .term => "term"

/-- Classify a declaration by the shape of its type. -/
def classify (ci : ConstantInfo) : MetaM Kind := do
  if (← isProp ci.type) then return .proof
  forallTelescope ci.type fun _ r => do
    let r ← whnf r
    match r with
    | .sort l => return if l.normalize.isZero then .predicate else .type
    | _ => return .term

structure Ctor where
  name : Name
  fields : Array String
  ihs : Array String

/-- Eliminator facts for an inductive type, or the reason there are none. -/
def eliminatorOf (ci : InductiveVal) : MetaM (Except String (String × String × Array Ctor)) := do
  if ci.all.length != 1 then return .error "mutual"
  if ci.isNested then return .error "nested"
  let env ← getEnv
  let mut ctors : Array Ctor := #[]
  for cn in ci.ctors do
    let some (.ctorInfo cv) := env.find? cn | return .error "missing-constructor"
    let r ← forallTelescope cv.type fun xs _ => do
      let mut fields := #[]
      let mut ihs := #[]
      let mut used : Std.HashSet String := {}
      for i in [cv.numParams:xs.size] do
        let d ← xs[i]!.fvarId!.getDecl
        let mut b := asciiIdent (binderBase d.userName)
        if b.isEmpty || !b.front.isAlpha then b := "x" ++ b
        let mut cand := b
        let mut k := 1
        while used.contains cand do
          cand := b ++ toString k
          k := k + 1
        used := used.insert cand
        fields := fields.push cand
        let fty ← forallTelescope d.type fun _ r => pure r
        if fty.getAppFn.isConstOf ci.name then ihs := ihs.push (cand ++ "ih")
      return (fields, ihs)
    ctors := ctors.push ⟨cn, r.1, r.2⟩
  let cases := `casesOn |> (ci.name ++ ·)
  let rec' := `rec |> (ci.name ++ ·)
  -- custom eliminators (`@[induction_eliminator]`, `@[cases_eliminator]`)
  let (casesN, indN) ← forallTelescope (← getConstInfo ci.name).type fun xs _ => do
    let ty := mkAppN (mkConst ci.name (ci.levelParams.map mkLevelParam)) xs
    withLocalDeclD `t ty fun t => do
      let c ← Meta.getCustomEliminator? #[t] (induction := false)
      let i ← Meta.getCustomEliminator? #[t] (induction := true)
      return (c.getD cases, i.getD rec')
  return .ok (casesN.toString, indN.toString, ctors)

def jstr (s : String) : Json := Json.str s

/-- Is `n` a name the glossary can carry (SPEC.md §13.6 conservative grammar)? -/
def conservativeName (n : Name) : Bool :=
  let s := n.toString (escape := false)
  !s.isEmpty && (s.splitOn ".").all fun seg =>
    match seg.toList with
    | [] => false
    | c :: cs => (c.isAlpha || c == '_') && c.toNat < 128 &&
        cs.all fun d => (d.isAlphanum || d == '_') && d.toNat < 128 &&
        (n.components.all fun c => c.isStr)

def kindOfInfo : ConstantInfo → String
  | .thmInfo _ => "theorem" | .defnInfo _ => "definition" | .axiomInfo _ => "axiom"
  | .opaqueInfo _ => "opaque" | .inductInfo _ => "inductive" | .ctorInfo _ => "constructor"
  | .recInfo _ => "recursor" | .quotInfo _ => "quotient"

/-- Should this declaration take part at all? Returns the exclusion reason. -/
def excluded? (env : Environment) (n : Name) (ci : ConstantInfo) : MetaM (Option String) := do
  -- `lexlean.core` already has an entry for these constants.
  if (coreEntries.lookup n).isSome then return some "core-entry"
  if n.isInternal || n.hasMacroScopes then return some "internal"
  if isPrivateName n then return some "private"
  if let .recInfo _ := ci then return some "recursor"
  if let .quotInfo _ := ci then return some "quotient"
  if isAuxRecursor env n || isNoConfusion env n then return some "auxiliary"
  if (← Meta.isMatcher n) then return some "auxiliary"
  if (← isInstance n) then return some "instance"
  if (Linter.deprecatedAttr.getParam? env n).isSome then return some "deprecated"
  return none

def levelNamesFor (ps : List Name) : Std.HashMap Name String × Array String := Id.run do
  let mut m : Std.HashMap Name String := {}
  let mut used : Std.HashSet String := {}
  let mut out := #[]
  let mut i := 0
  for p in ps do
    let mut b := asciiIdent p.toString
    if b.isEmpty || !b.front.isAlpha then b := "u" ++ b
    let mut cand := b
    let mut k := 1
    while used.contains cand do
      cand := b ++ toString k
      k := k + 1
    used := used.insert cand
    m := m.insert p cand
    out := out.push cand
    i := i + 1
  return (m, out)

/-- Translate one declaration to its JSON record. -/
def record (env : Environment) (n : Name) (ci : ConstantInfo) (requested : Bool) :
    MetaM (Json × NameSet) := do
  let modName := match env.getModuleIdxFor? n with
    | some i => env.header.moduleNames[i.toNat]!.toString
    | none => ""
  let base : List (String × Json) :=
    [("name", jstr n.toString), ("module", jstr modName),
     ("kind", jstr (kindOfInfo ci)), ("requested", Json.bool requested)]
  if !conservativeName n then
    return (Json.mkObj (base ++ [("refusal", Json.mkObj [("reason", jstr "name-not-conservative"),
      ("detail", jstr "the Lean name is outside the conservative ASCII grammar")])]), {})
  let (lm, lnames) := levelNamesFor ci.levelParams
  let kind ← classify ci
  let r ← (do
      let t ← tr ci.type
      let st ← get
      return (t, st)
    : TrM (String × TrState)).run { levelNames := lm } |>.run
  match r with
  | .error ⟨reason, detail⟩ =>
    return (Json.mkObj (base ++ [("refusal", Json.mkObj [("reason", jstr reason), ("detail", jstr detail)])]), {})
  | .ok ((sig, st), _) =>
    let sortedUniv := lnames.qsort (· < ·)
    let mut fields := base ++
      [("signature", jstr sig), ("class", jstr kind.str),
       ("universes", Json.arr (sortedUniv.map jstr)),
       ("erased", Json.num st.erased),
       ("refs", Json.arr (st.refs.toArray.qsort Name.lt |>.map (jstr ·.toString)))]
    if let .inductInfo iv := ci then
      match ← eliminatorOf iv with
      | .error why => fields := fields ++ [("eliminator_omitted", jstr why)]
      | .ok (c, i, ctors) =>
        fields := fields ++ [("eliminator", Json.mkObj
          [("cases", jstr c), ("induction", jstr i),
           ("constructors", Json.arr (ctors.map fun k => Json.mkObj
              [("name", jstr k.name.toString),
               ("fields", Json.arr (k.fields.map jstr)),
               ("induction_hypotheses", Json.arr (k.ihs.map jstr))]))])]
    let mut deps := st.refs
    if let .inductInfo iv := ci then
      for c in iv.ctors do deps := deps.insert c
    return (Json.mkObj fields, deps)

def run (env : Environment) (roots : Array Name) : MetaM Json := do
  let mut done : NameSet := {}
  let mut out : Array (Name × Json) := #[]
  let mut excludedCount : Std.TreeMap String Nat := {}
  let mut queue : Array (Name × Bool) := roots.map (·, true)
  let mut head := 0
  while head < queue.size do
    let (n, req) := queue[head]!
    head := head + 1
    if done.contains n then continue
    done := done.insert n
    let some ci := env.find? n
      | out := out.push (n, Json.mkObj [("name", jstr n.toString), ("requested", Json.bool req),
          ("refusal", Json.mkObj [("reason", jstr "unknown-declaration"),
            ("detail", jstr "not in the imported environment")])])
        continue
    if let some why ← excluded? env n ci then
      if req then
        excludedCount := excludedCount.insert why (excludedCount.getD why 0 + 1)
      -- The record is written either way, as not requested: a requested
      -- declaration that is set aside is counted once, under `excluded`, and
      -- a declaration that mentions it still needs the reason.
      out := out.push (n, Json.mkObj [("name", jstr n.toString), ("requested", Json.bool false),
        ("refusal", Json.mkObj [("reason", jstr s!"excluded-{why}"),
          ("detail", jstr s!"a {why} declaration is not exposed as an entry")])])
      continue
    let (j, deps) ← record env n ci req
    out := out.push (n, j)
    for d in deps.toArray.qsort Name.lt do
      queue := queue.push (d, false)
  let sorted := out.qsort (fun a b => Name.lt a.1 b.1)
  return Json.mkObj
    [("declarations", Json.arr (sorted.map (·.2))),
     ("excluded", Json.mkObj (excludedCount.toList.map fun (k, v) => (k, Json.num v)))]

end LexLeanGlossary

open LexLeanGlossary in
def parseArgs (args : List String) : Except String (List String × List String) := do
  let mut mods := []
  let mut names := []
  let mut mode := ""
  for a in args do
    if a == "--modules" then mode := "m"
    else if a == "--names" then mode := "n"
    else if mode == "m" then mods := mods ++ [a]
    else if mode == "n" then names := names ++ [a]
    else throw s!"unexpected argument {a}"
  return (mods, names)

unsafe def main (args : List String) : IO UInt32 := do
  let args := args.filter (· != "--")
  let (mods, names) ← match parseArgs args with
    | .ok r => pure r
    | .error m => do IO.eprintln m; return 2
  initSearchPath (← findSysroot)
  let imports : Array Import := (mods.map fun m => { module := m.toName : Import }).toArray
  let imports := if imports.isEmpty then #[{ module := `Init }] else imports
  -- `loadExts` runs the import hooks of environment extensions, which is
  -- what makes scoped attributes such as `@[induction_eliminator]` visible.
  enableInitializersExecution
  let env ← importModules imports {} (loadExts := true)
  do
    -- every declaration defined in a requested module, plus named ones
    let mut roots : Array Name := #[]
    for m in mods do
      match env.getModuleIdx? m.toName with
      | none => do IO.eprintln s!"module {m} is not in the environment"; return 3
      | some idx =>
        for n in env.header.moduleData[idx.toNat]!.constNames do roots := roots.push n
    for n in names do roots := roots.push n.toName
    roots := roots.qsort Name.lt
    let ctx : Core.Context := { fileName := "<glossary>", fileMap := default, maxHeartbeats := 0 }
    let act : CoreM Json := (LexLeanGlossary.run env roots).run'
    let (j, _) ← act.toIO ctx { env }
    IO.println j.compress
    return 0
