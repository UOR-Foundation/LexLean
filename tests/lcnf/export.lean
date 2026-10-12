import Lean
open Lean

abbrev E := Except String

def nm (n : Name) : String := n.toString (escape := false)

def binderInfoName : BinderInfo → String
  | .default => "explicit"
  | .implicit => "implicit"
  | .strictImplicit => "implicit"
  | .instImplicit => "instance"

def obj (fields : List (String × Json)) : Json := Json.mkObj fields

/-- A binder named by a macro scope gets a plain name, which nothing depends on. -/
def plain (n : Name) (fallback : String) : String := if n.hasMacroScopes then fallback else nm n

def typeSort (l : Level) : E Json :=
  match l with
  | .succ .zero => pure (obj [("kind", "type")])
  | .succ (.param u) => pure (obj [("kind", "type"), ("level", obj [("kind", "parameter"), ("name", nm u)])])
  | other => throw s!"unsupported sort {other}"

/-- A signature: constants, variables, applications, arrows, binders, sorts,
`optParam` defaults, and natural literals. -/
partial def sigOf (ctx : List String) (e : Expr) : E Json := do
  match e with
  | .bvar i =>
    match ctx[i]? with
    | some n => pure (obj [("kind", "var"), ("name", n)])
    | none => throw s!"loose bound variable {i}"
  | .const n _ => pure (obj [("kind", "const"), ("name", nm n)])
  | .mdata _ b => sigOf ctx b
  | .sort l => typeSort l
  | .lit (.natVal v) => pure (obj [("kind", "nat"), ("value", toString v)])
  | .app .. =>
    let f := e.getAppFn
    let args := e.getAppArgs
    if f.isConstOf ``optParam && args.size == 2 then
      pure (obj [("kind", "opt_param"), ("domain", ← sigOf ctx args[0]!), ("default", ← defaultOf ctx args[1]!)])
    else if f.isConstOf ``OfNat.ofNat && args.size == 3 then
      match args[1]! with
      | .lit (.natVal v) => pure (obj [("kind", "nat"), ("value", toString v)])
      | other => throw s!"unsupported numeral {other.ctorName}"
    else
      pure (obj [("kind", "app"), ("function", ← sigOf ctx f), ("arguments", Json.arr (← args.mapM (sigOf ctx)))])
  | .forallE n d b bi =>
    let dom ← domOf ctx d
    if bi == .default && !b.hasLooseBVar 0 then
      pure (obj [("kind", "arrow"), ("domain", dom), ("codomain", ← sigOf ("_" :: ctx) b)])
    else
      -- a binder named by a macro scope gets a plain name, which no signature depends on
      let bn := if n.hasMacroScopes then s!"h{ctx.length}" else nm n
      let fields := [("kind", Json.str "pi"), ("info", Json.str (binderInfoName bi)), ("domain", dom),
                     ("codomain", ← sigOf (bn :: ctx) b)]
      pure (obj (("name", Json.str bn) :: fields))
  | other => throw s!"unsupported expression {other.ctorName}"
where
  domOf (ctx : List String) (d : Expr) : E Json := sigOf ctx d
  /-- A default value: a structure instance whose fields all default, or an expression. -/
  defaultOf (ctx : List String) (d : Expr) : E Json := do
    match d.getAppFn with
    | .const n _ =>
      if d.isConst then sigOf ctx d
      else if n.isStr && n.getString! == "mk" then pure (obj [("kind", "empty_structure")])
      else sigOf ctx d
    | _ => sigOf ctx d

partial def binders (e : Expr) (count : Option Nat) : List (Name × Expr × BinderInfo) × Expr :=
  match e, count with
  | _, some 0 => ([], e)
  | .forallE n d b bi, c =>
    let (rest, r) := binders b (c.map (· - 1))
    ((n, d, bi) :: rest, r)
  | _, _ => ([], e)

partial def toolType (env : Environment) (ctx : List String) (e : Expr) : E Json := do
  match e with
  | .bvar i =>
    match ctx[i]? with
    | some n => pure (obj [("kind", "parameter"), ("name", n), ("arguments", Json.arr #[])])
    | none => throw s!"loose bound variable {i}"
  | .const n _ =>
    match env.find? n with
    | some (.ctorInfo _) => pure (obj [("kind", "index"), ("constructor", nm n)])
    | _ => pure (obj [("kind", "const"), ("name", nm n), ("arguments", Json.arr #[])])
  | .app .. =>
    let f := e.getAppFn
    -- an instance argument of a type is inferred by Lean and never stated
    let stated : Array Expr :=
      match f with
      | .const n _ =>
        match env.find? n with
        | some info =>
          let (bs, _) := binders info.type none
          (e.getAppArgs.zipIdx.filter fun (_, i) =>
            match bs[i]? with
            | some (_, _, .instImplicit) => false
            | _ => true).map (·.1)
        | none => e.getAppArgs
      | _ => e.getAppArgs
    let args ← stated.mapM (toolType env ctx)
    match f with
    | .const n _ => pure (obj [("kind", "const"), ("name", nm n), ("arguments", Json.arr args)])
    | .bvar i =>
      match ctx[i]? with
      | some n => pure (obj [("kind", "parameter"), ("name", n), ("arguments", Json.arr args)])
      | none => throw s!"loose bound variable {i}"
    | other => throw s!"unsupported application head {other.ctorName}"
  | .forallE _ d b .default =>
    if b.hasLooseBVar 0 then throw "dependent function type"
    else
      let dom ← toolType env ctx d
      let rest ← toolType env ("_" :: ctx) b
      match rest.getObjVal? "kind", rest.getObjVal? "parameters", rest.getObjVal? "result" with
      | .ok (.str "function"), .ok (.arr ps), .ok r =>
        pure (obj [("kind", "function"), ("parameters", Json.arr (#[dom] ++ ps)), ("result", r)])
      | _, _, _ => pure (obj [("kind", "function"), ("parameters", Json.arr #[dom]), ("result", rest)])
  | .mdata _ b => toolType env ctx b
  | other => throw s!"unsupported type expression {other.ctorName}"

def parameterKind (env : Environment) (d : Expr) : E Json :=
  match d with
  | .sort _ => pure (obj [("kind", "type")])
  | .forallE _ (.sort _) (.sort _) _ => pure (obj [("kind", "constructor")])
  | .const n _ =>
    match env.find? n with
    | some (.inductInfo _) => pure (obj [("kind", "index"), ("of", nm n)])
    | _ => throw s!"unsupported parameter kind {nm n}"
  | other => throw s!"unsupported parameter kind {other.ctorName}"

def moduleOf (env : Environment) (n : Name) : String :=
  match env.getModuleIdxFor? n with
  | some i => nm env.header.moduleNames[i.toNat]!
  | none => ""

def sourceJson (env : Environment) (n : Name) : Json :=
  obj [("module", moduleOf env n)]

def typeRow (env : Environment) (n : Name) (mode : String) : E Json := do
  let some info := env.find? n | throw s!"unknown constant {nm n}"
  match mode with
  | "alias" =>
    let .defnInfo v := info | throw s!"{nm n} is not a definition"
    let target ← toolType env [] v.value
    pure (obj [("kind", "type"), ("name", nm n), ("parameters", Json.arr #[]), ("constructors", Json.arr #[]),
               ("alias", target), ("source", sourceJson env n)])
  | "opaque" =>
    let (bs, _) := binders info.type none
    let mut ps : Array Json := #[]
    for (bn, d, bi) in bs do
      -- an instance parameter is inferred by Lean and never stated
      if bi == .instImplicit then continue
      ps := ps.push (obj [("name", plain bn s!"a{ps.size}"), ("kind", ← parameterKind env d)])
    pure (obj [("kind", "type"), ("name", nm n), ("parameters", Json.arr ps), ("constructors", Json.arr #[]),
               ("opaque", true), ("source", sourceJson env n)])
  | _ =>
    let .inductInfo v := info | throw s!"{nm n} is not an inductive type"
    let (pbs, _) := binders v.type (some v.numParams)
    let mut ps : Array Json := #[]
    let mut pnames : List String := []
    for (bn, d, bi) in pbs do
      if bi == .instImplicit then continue
      let pn := plain bn s!"a{ps.size}"
      ps := ps.push (obj [("name", pn), ("kind", ← parameterKind env d)])
      pnames := pn :: pnames
    let mut ctors : Array Json := #[]
    for c in v.ctors do
      let some (.ctorInfo cv) := env.find? c | throw s!"{nm c} is not a constructor"
      let (cbs, result) := binders cv.type none
      let params := cbs.take cv.numParams
      let fields := cbs.drop cv.numParams
      let mut ctx : List String := (params.zipIdx.map fun ((bn, _, _), i) => plain bn s!"a{i}").reverse
      let mut fjs : Array Json := #[]
      let mut eqRequires : Option Json := none
      for ((fname0, fd, _), fi) in fields.zipIdx do
        let fname : Name := (Name.mkSimple (plain fname0 s!"f{fi}"))
        let isIndexProof :=
          match fd.getAppFn, fd.getAppArgs with
          | .const ``autoParam _, args =>
            match args[0]!.getAppFn, args[0]!.getAppArgs with
            | .const ``Eq _, eqArgs =>
              match eqArgs[1]!, eqArgs[2]! with
              | .bvar i, .const k _ =>
                match ctx[i]? with
                | some pn => some (obj [("parameter", pn), ("constructor", nm k)])
                | none => none
              | _, _ => none
            | _, _ => none
          | _, _ => none
        match isIndexProof with
        | some r => eqRequires := some r
        | none => fjs := fjs.push (obj [("name", nm fname), ("type", ← toolType env ctx fd)])
        ctx := nm fname :: ctx
      let args := result.getAppArgs
      let mut requires : Option Json := eqRequires
      for h : i in [:args.size] do
        match args[i] with
        | .const k _ =>
          if (env.find? k).any (·.isCtor) then
            let pname := (pnames.reverse)[i]!
            requires := some (obj [("parameter", pname), ("constructor", nm k)])
        | _ => pure ()
      let short := ((nm c).drop (nm n).length |>.drop 1).toString
      let base := [("name", Json.str short), ("fields", Json.arr fjs)]
      ctors := ctors.push (obj (match requires with | some r => base ++ [("requires", r)] | none => base))
    pure (obj [("kind", "type"), ("name", nm n), ("parameters", Json.arr ps), ("constructors", Json.arr ctors),
               ("source", sourceJson env n)])

def callRow (env : Environment) (n : Name) : E Json := do
  let some info := env.find? n | throw s!"unknown constant {nm n}"
  pure (obj [("kind", "call"), ("name", nm n), ("signature", ← sigOf [] info.type), ("source", sourceJson env n)])

/-- The constants a signature or a type names, for the closure of the registry. -/
partial def constsOf (e : Expr) (out : NameSet) : NameSet :=
  e.getUsedConstants.foldl (fun acc c => acc.insert c) out

def isTypeFormer (info : ConstantInfo) : Bool := info.type.getForallBody.isSort

/-- The request a referenced constant needs, if it needs one: a constructor
needs its inductive type listed; a type former is an opaque type; any other
constant is a call. -/
def requestFor (env : Environment) (c : Name) : E (Option (String × Name)) := do
  match env.find? c with
  | none => throw s!"unknown constant {nm c}"
  | some (.ctorInfo cv) => pure (some ("inductive", cv.induct))
  | some info => if isTypeFormer info then pure (some ("opaque", c)) else pure (some ("call", c))

def mentionedBy (row : Json) : NameSet := Id.run do
  -- every constant a row names, from its signature, fields, alias, and index parameters
  let mut out : NameSet := {}
  let rec walk (j : Json) (acc : NameSet) (fuel : Nat) : NameSet :=
    match fuel with
    | 0 => acc
    | fuel + 1 =>
      match j with
      | .obj kvs =>
        let acc := match kvs.get? "kind", kvs.get? "name" with
          | some (.str "const"), some (.str n) => acc.insert n.toName
          | _, _ => acc
        let acc := match kvs.get? "kind", kvs.get? "of" with
          | some (.str "index"), some (.str n) => acc.insert n.toName
          | _, _ => acc
        let acc := match kvs.get? "kind", kvs.get? "constructor" with
          | some (.str "index"), some (.str n) => acc.insert n.toName.getPrefix
          | _, _ => acc
        kvs.toList.foldl (fun acc (_, v) => walk v acc fuel) acc
      | .arr items => items.foldl (fun acc v => walk v acc fuel) acc
      | _ => acc
  out := walk row out 64
  return out

def main (args : List String) : IO UInt32 := do
  initSearchPath (← findSysroot)
  let env ← importModules #[{ module := `Lean }] {} 0
  -- seeds: `call:Name`, `inductive:Name`, `opaque:Name`, `alias:Name`
  let mut queue : List (String × Name) := []
  for a in args do
    match a.splitOn ":" with
    | [kind, name] => queue := queue ++ [(kind, name.toName)]
    | _ => IO.eprintln s!"bad seed {a}"
  let mut rows : Std.HashMap String Json := {}
  let mut requested : Std.HashMap Name String := {}
  let mut errors : Array Json := #[]
  -- an explicit request for the listing of constructors wins over an opaque one
  for (kind, n) in queue do
    match requested.get? n with
    | some "inductive" => pure ()
    | _ => requested := requested.insert n kind
  let mut work := queue.map (·.2)
  let mut seen : NameSet := {}
  while !work.isEmpty do
    let n := work.head!
    work := work.tail!
    if seen.contains n then continue
    seen := seen.insert n
    let kind := (requested.get? n).getD "call"
    let result : E Json :=
      match kind with
      | "call" => callRow env n
      | "inductive" => typeRow env n "inductive"
      | "opaque" => typeRow env n "opaque"
      | "alias" => typeRow env n "alias"
      | _ => throw s!"bad kind {kind}"
    match result with
    | .error e => errors := errors.push (obj [("name", nm n), ("error", e)])
    | .ok row =>
      rows := rows.insert (nm n) row
      for c in (mentionedBy row).toList do
        if seen.contains c then continue
        match requestFor env c with
        | .error e => errors := errors.push (obj [("name", nm c), ("error", e)])
        | .ok none => pure ()
        | .ok (some (k, target)) =>
          if !seen.contains target then
            match requested.get? target with
            | some "inductive" => pure ()
            | some "alias" => pure ()
            | _ => requested := requested.insert target k
            work := work ++ [target]
  let sorted := rows.toList.toArray.qsort (fun a b => a.1 < b.1)
  IO.println (obj [("rows", Json.arr (sorted.map (·.2))), ("errors", Json.arr errors)]).compress
  return 0
