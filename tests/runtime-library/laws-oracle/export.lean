import Lean
import LexLeanRuntime.Portable
import LexLeanRuntime.Collections
import LexLeanRuntime.Laws
open Lean Meta

/-! One-time lossless exporter of an elaborated library into `lexlean/core-module/2` data
(scratch tool; never part of the release tree). -/

def libMods : List Name := [`LexLeanRuntime.Portable, `LexLeanRuntime.Collections, `LexLeanRuntime.Laws]

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

def encConstName (n : Name) : IO String := encName n

def encLevel : Level → IO Json
  | .zero => pure (Json.mkObj [("k", "z")])
  | .succ l => do pure (Json.mkObj [("k", "s"), ("a", ← encLevel l)])
  | .max a b => do pure (Json.mkObj [("k", "m"), ("a", Json.arr #[← encLevel a, ← encLevel b])])
  | .imax a b => do pure (Json.mkObj [("k", "i"), ("a", Json.arr #[← encLevel a, ← encLevel b])])
  | .param n => do pure (Json.mkObj [("k", "p"), ("a", Json.str (← encConstName n))])
  | .mvar _ => fail "universe metavariable"

def encBinder : BinderInfo → String
  | .default => "e" | .implicit => "i" | .strictImplicit => "s" | .instImplicit => "c"

structure St where
  nodes : Array Json := #[]
  memo : Std.HashMap ExprStructEq Nat := {}
  axioms : Std.HashMap Name (Array Name) := {}

abbrev M := StateRefT St IO

def push (j : Json) : M Nat := do
  let s ← get
  set { s with nodes := s.nodes.push j }
  return s.nodes.size

partial def node (env : Environment) (e : Expr) : M Nat := do
  let key : ExprStructEq := ⟨e⟩
  if let some i := (← get).memo[key]? then return i
  let idx ← match e with
    | .bvar i => push (Json.mkObj [("k", "b"), ("i", i)])
    | .sort l => do push (Json.mkObj [("k", "s"), ("l", ← encLevel l)])
    | .const n us => do
      let us ← us.mapM fun l => (encLevel l : M Json)
      push (Json.mkObj [("k", "c"), ("n", ← encConstName n), ("u", Json.arr us.toArray)])
    | .app f x => do
      let f ← node env f
      let x ← node env x
      push (Json.mkObj [("k", "a"), ("f", f), ("x", x)])
    | .lam n t b bi => do
      let t ← node env t
      let b ← node env b
      push (Json.mkObj [("k", "l"), ("n", ← encName n), ("b", encBinder bi), ("t", t), ("v", b)])
    | .forallE n t b bi => do
      let t ← node env t
      let b ← node env b
      push (Json.mkObj [("k", "p"), ("n", ← encName n), ("b", encBinder bi), ("t", t), ("v", b)])
    | .letE n t v b nd => do
      let t ← node env t
      let v ← node env v
      let b ← node env b
      push (Json.mkObj [("k", "e"), ("n", ← encName n), ("t", t), ("v", v), ("body", b), ("d", nd)])
    | .lit (.natVal v) => push (Json.mkObj [("k", "n"), ("v", toString v)])
    | .lit (.strVal s) => push (Json.mkObj [("k", "t"), ("v", Json.arr (s.toList.map fun c => toJson c.toNat).toArray)])
    | .mdata d b => do
      let b ← node env b
      let mut kvs : Array Json := #[]
      for (k, v) in d do
        let v ← match v with
          | .ofBool x => pure (Json.mkObj [("b", x)])
          | .ofNat x => pure (Json.mkObj [("n", toString x)])
          | .ofInt x => pure (Json.mkObj [("i", toString x)])
          | .ofString x => pure (Json.mkObj [("s", x)])
          | .ofName x => do pure (Json.mkObj [("m", ← encName x)])
          | .ofSyntax _ => fail "mdata syntax value"
        kvs := kvs.push (Json.arr #[Json.str (← encName k), v])
      push (Json.mkObj [("k", "d"), ("m", Json.arr kvs), ("x", b)])
    | .proj s i x => do
      let x ← node env x
      let field? := (getStructureInfo? env s).bind fun info => info.fieldNames[i]?
      let projFn := match field? with
        | some f => (getProjFnForField? env s f).getD s
        | none => s
      push (Json.mkObj [("k", "j"), ("n", ← encConstName projFn), ("s", ← encConstName s), ("i", i), ("x", x)])
    | .fvar _ => fail "free variable"
    | .mvar _ => fail "metavariable"
  modify fun s => { s with memo := s.memo.insert key idx }
  return idx

/-- Exactly the axioms `#print axioms` prints: Lean's own `collectAxioms`, which answers for an imported constant
from the axiom data stored in its module's `.olean`. That data was computed when the module was written, with a
cache shared across the module's constants, so for the constants of a reference cycle (a recursive unsafe
definition) it depends on the order of the module's constants: a row pins what the pinned Lean prints, not a set
derived from the term. -/
def axiomsOf (env : Environment) (n : Name) : M (Array Name) := do
  if let some r := (← get).axioms[n]? then return r
  let ctx : Core.Context := { fileName := "<export>", fileMap := default }
  let (found, _) ← (Lean.collectAxioms n : CoreM (Array Name)).toIO ctx { env }
  let out := found.qsort (fun a b => a.toString < b.toString)
  modify fun s => { s with axioms := s.axioms.insert n out }
  return out

def encAttrKind : AttributeKind → String
  | .global => "global" | .local => "local" | .scoped => "scoped"

def encStatus : ReducibilityStatus → String
  | .reducible => "reducible" | .semireducible => "semireducible" | .irreducible => "irreducible"
  | .implicitReducible => "implicit-reducible"

def namesJson (ns : List Name) : IO Json := do pure (Json.arr (← ns.mapM fun n => do pure (Json.str (← encConstName n))).toArray)

/-- The `grind` attribute of a theorem as the closed kinds the schema names; any other registration (a user
pattern, a generation counter, a cases, ext, or injectivity entry) is refused rather than dropped. -/
def grindKindOf (env : Environment) (n : Name) : IO (Option String) := do
  let ctx : Core.Context := { fileName := "<export>", fileMap := default }
  let (thms, _) ← (Lean.Meta.Grind.grindExt.getEMatchTheorems : CoreM _).toIO ctx { env }
  let mut kinds : Array String := #[]
  for t in thms.find (.decl n) do
    let k ← match t.kind with
      | .eqLhs false => pure "eq_lhs"
      | .eqRhs false => pure "eq_rhs"
      | .eqBoth false => pure "eq_both"
      | .eqBwd => pure "eq_bwd"
      | .fwd => pure "fwd"
      | .bwd false => pure "bwd"
      | .leftRight => pure "left_right"
      | .rightLeft => pure "right_left"
      | .default false => pure "default"
      | other => fail s!"grind attribute of {n} has the kind {repr other}, which the schema does not name"
    unless kinds.contains k do kinds := kinds.push k
  if kinds.size > 1 then fail s!"grind attribute of {n} has several kinds {kinds}"
  return kinds[0]?

def exportModule (env : Environment) (outDir : String) (m : Name) : M Unit := do
  let modNames := env.header.moduleNames
  modify fun s => { s with nodes := #[], memo := {} }
  do
    let some idx := modNames.findIdx? (· == m) | fail s!"module {m} is not imported"
    let data := env.header.moduleData[idx]!
    -- the decoder runtime a generated module carries is not library content
    let isRuntime (n : Name) : Bool := (`LexLeanCore).isPrefixOf ((privateToUserName? n).getD n)
    let own : Array Name := data.constNames.filter fun n => !isRuntime n
    let ownSet : Std.HashSet Name := own.foldl (·.insert ·) {}
    -- dependency order within the module, ties broken by name
    let deps (n : Name) : Array Name := Id.run do
      let some ci := env.find? n | return #[]
      let mut refs := ci.type.getUsedConstants
      match ci.value? (allowOpaque := true) with | some v => refs := refs ++ v.getUsedConstants | none => pure ()
      match ci with
      | .inductInfo v => refs := refs ++ v.ctors.toArray
      | .ctorInfo v => refs := refs.push v.induct
      | .recInfo v => refs := refs.push v.getMajorInduct
      | _ => pure ()
      return refs.filter fun r => r != n && ownSet.contains r
    -- constructors and recursors follow their inductive: treat them as satellites of the owner
    let mut order : Array Name := #[]
    let mut state : Std.HashMap Name Nat := {}  -- 1 = in progress, 2 = done
    let sorted := own.qsort (fun a b => a.toString < b.toString)
    for root in sorted do
      let mut stack : Array (Name × Bool) := #[(root, false)]
      while !stack.isEmpty do
        let (n, expanded) := stack.back!
        stack := stack.pop
        if expanded then
          if state[n]? != some 2 then
            state := state.insert n 2
            order := order.push n
        else
          match state[n]? with
          | some _ => pure ()
          | none =>
            state := state.insert n 1
            stack := stack.push (n, true)
            let ds := (deps n).qsort (fun a b => a.toString > b.toString)
            for d in ds do
              if state[d]? == none then stack := stack.push (d, false)
    let mut decls : Array Json := #[]
    for n in order do
      let some ci := env.find? n | fail s!"missing {n}"
      let type ← node env ci.type
      let value? ← match ci.value? (allowOpaque := true) with
        | some v => do pure (some (← node env v))
        | none => pure none
      let (kind, generated) ← match ci with
        | .defnInfo v => pure ((if v.hints.isAbbrev then "abbrev" else "definition"), v.safety != .safe)
        | .thmInfo _ => pure ("theorem", false)
        | .inductInfo _ => pure ("inductive", false)
        | .ctorInfo _ => pure ("constructor", true)
        | .recInfo _ => pure ("recursor", true)
        | .axiomInfo _ => fail s!"axiom {n}"
        | .opaqueInfo _ => fail s!"opaque {n}"
        | .quotInfo _ => fail s!"quot {n}"
      let axs ← axiomsOf env n
      let mut fields : Array (String × Json) := #[
        ("name", Json.str (← encConstName n)),
        ("levels", Json.arr (← ci.levelParams.mapM fun l => do pure (Json.str (← encConstName l))).toArray),
        ("kind", kind),
        ("transparency", encStatus (getReducibilityStatusCore env n)),
        ("policy", Json.mkObj [("kind", "exact"), ("axioms", Json.arr (← axs.mapM fun a => do pure (Json.str (← encConstName a)))) ]),
        ("type", type),
        ("class", isClass env n),
        ("generated", generated)]
      if let some v := value? then fields := fields.push ("value", v)
      match ci with
      | .defnInfo v =>
        let red := match v.hints with
          | .abbrev => Json.mkObj [("kind", "abbrev")]
          | .opaque => Json.mkObj [("kind", "opaque")]
          | .regular h => Json.mkObj [("kind", "regular"), ("height", h.toNat)]
        fields := fields.push ("reducibility", red)
        fields := fields.push ("all", ← namesJson v.all)
      | .thmInfo v => fields := fields.push ("all", ← namesJson v.all)
      | .inductInfo v =>
        let mut ind : Array (String × Json) := #[
          ("num_params", v.numParams), ("num_indices", v.numIndices),
          ("constructors", ← namesJson v.ctors)]
        if v.all.length != 1 || v.isNested || v.isUnsafe then fail s!"inductive {n} is mutual, nested, or unsafe"
        if let some info := getStructureInfo? env n then
          let mut flds : Array Json := #[]
          for f in info.fieldInfo do
            let mut o : Array (String × Json) := #[
              ("name", Json.str (← encName f.fieldName)), ("projection", Json.str (← encConstName f.projFn)),
              ("binder", encBinder f.binderInfo)]
            if let some s := f.subobject? then o := o.push ("subobject", Json.str (← encConstName s))
            if let some a := f.autoParam? then o := o.push ("auto_param", (← node env a))
            flds := flds.push (Json.mkObj o.toList)
          -- fields are stored in position order in the constructor; `fieldInfo` is sorted by name
          let mut ordered : Array Json := #[]
          for fname in info.fieldNames do
            let some f := info.fieldInfo.find? (·.fieldName == fname) | fail s!"field {fname} of {n}"
            let mut o : Array (String × Json) := #[
              ("name", Json.str (← encName f.fieldName)), ("projection", Json.str (← encConstName f.projFn)),
              ("binder", encBinder f.binderInfo)]
            if let some s := f.subobject? then o := o.push ("subobject", Json.str (← encConstName s))
            if let some a := f.autoParam? then o := o.push ("auto_param", (← node env a))
            ordered := ordered.push (Json.mkObj o.toList)
          let mut parents : Array Json := #[]
          for p in getStructureParentInfo env n do
            parents := parents.push (Json.mkObj [("name", Json.str (← encConstName p.structName)), ("subobject", p.subobject), ("projection", Json.str (← encConstName p.projFn))])
          ind := ind.push ("structure", Json.mkObj [("fields", Json.arr ordered), ("parents", Json.arr parents)])
        fields := fields.push ("inductive", Json.mkObj ind.toList)
        fields := fields.push ("all", ← namesJson v.all)
      | _ => pure ()
      -- instance registration
      if let some e := (Meta.instanceExtension.getState env).instanceNames.find? n then
        fields := fields.push ("instance", Json.mkObj [("priority", e.priority), ("attribute", encAttrKind e.attrKind),
          ("synthesis_order", Json.arr (e.synthOrder.map fun i => toJson i))])
      -- elaboration-visible tags
      let mut tags : Array String := #[]
      if isProtected env n then tags := tags.push "protected"
      if isAuxRecursor env n then tags := tags.push "aux_recursor"
      if Elab.Term.elabAsElim.hasTag env n then tags := tags.push "elab_as_elim"
      if defeqAttr.hasTag env n then tags := tags.push "defeq"
      if backwardDefeqAttr.hasTag env n then tags := tags.push "backward_defeq"
      if recExt.isTagged env n then tags := tags.push "recursive"
      if !tags.isEmpty then fields := fields.push ("tags", Json.arr (tags.map Json.str))
      if isNoConfusion env n then
        fields := fields.push ("no_confusion", match getNoConfusionInfo env n with
          | .regular a l r => Json.mkObj [("kind", "regular"), ("arity", a), ("lhs", l), ("rhs", r)]
          | .perCtor a f => Json.mkObj [("kind", "per_ctor"), ("arity", a), ("fields", f)])
      if let some mi := Match.Extension.getMatcherInfo? env n then
        fields := fields.push ("matcher", Json.mkObj [
          ("num_params", mi.numParams), ("num_discrs", mi.numDiscrs),
          ("alts", Json.arr (mi.altInfos.map fun a => Json.mkObj [("fields", a.numFields), ("overlaps", a.numOverlaps), ("unit_thunk", a.hasUnitThunk)])),
          ("u_elim_pos", match mi.uElimPos? with | some p => toJson p | none => Json.null),
          ("overlaps", Json.arr ((mi.overlaps.map.toList.map fun (k, s) => Json.arr #[toJson k, Json.arr (s.toArray.map fun i => toJson i)]).toArray.qsort (fun a b => a.compress < b.compress))),
          ("discrs", Json.arr (← mi.discrInfos.mapM fun d => (do
              match d.hName? with
              | some h => pure (Json.str (← encName h))
              | none => pure Json.null : M Json)))])
      if let some pi := env.getProjectionFnInfo? n then
        fields := fields.push ("projection", Json.mkObj [("constructor", Json.str (← encConstName pi.ctorName)),
          ("num_params", pi.numParams), ("index", pi.i), ("from_class", pi.fromClass)])
      if (Meta.simpExtension.getState env).lemmaNames.contains (.decl n) then
        fields := fields.push ("simp", Json.mkObj [("attribute", "global"), ("priority", 1000), ("post", true)])
      if let some ei := Elab.Structural.eqnInfoExt.find? env n then
        let perms := ei.fixedParamPerms.perms.map fun p => Json.arr (p.map fun o => match o with | some i => toJson i | none => Json.null)
        fields := fields.push ("eqn_info", Json.mkObj [
          ("rec_arg_pos", ei.recArgPos), ("declarations", ← namesJson ei.declNames.toList),
          ("num_fixed", ei.fixedParamPerms.numFixed), ("perms", Json.arr perms),
          ("rev_deps", Json.arr (ei.fixedParamPerms.revDeps.map fun a => Json.arr (a.map fun b => Json.arr (b.map fun i => toJson i)))),
          ("type", (← node env ei.type)), ("value", (← node env ei.value)),
          ("levels", Json.arr (← ei.levelParams.mapM fun l => do pure (Json.str (← encConstName l))).toArray)])
      if let some k := Lean.Compiler.getInlineAttribute? env n then
        fields := fields.push ("inline", Json.str (match k with
          | .inline => "inline" | .noinline => "noinline" | .macroInline => "macro_inline"
          | .inlineIfReduce => "inline_if_reduce" | .alwaysInline => "always_inline"))
      if let some si := Meta.getSparseCasesOnInfoCore env n then
        fields := fields.push ("sparse", Json.mkObj [("ind_name", Json.str (← encConstName si.indName)),
          ("major_pos", si.majorPos), ("arity", si.arity),
          ("interesting_ctors", ← namesJson si.insterestingCtors.toList)])
      if let some k ← grindKindOf env n then fields := fields.push ("grind", Json.str k)
      decls := decls.push (Json.mkObj fields.toList)
    let st ← get
    let imports := data.imports.map (·.module.toString)
    let doc := Json.mkObj [("spec", "lexlean/core-module/2"), ("imports", Json.arr (imports.map Json.str)),
      ("nodes", Json.arr st.nodes), ("proof_nodes", Json.arr #[]), ("declarations", Json.arr decls)]
    IO.FS.writeFile s!"{outDir}/{m}.json" doc.compress
    IO.println s!"{m}: decls={decls.size} nodes={st.nodes.size}"

unsafe def main : IO Unit := do
  initSearchPath (← Lean.findSysroot)
  enableInitializersExecution
  let outDir := (← IO.getEnv "EXPORT_DIR").getD "export-out"
  IO.FS.createDirAll outDir
  let env ← importModules (libMods.toArray.map fun m => {module := m}) {} (loadExts := true)
  let only := ((← IO.getEnv "EXPORT_MODS").getD "").splitOn ","
  let act : M Unit := do
    for m in libMods do
      if only == [""] || only.contains ((m.toString.drop 15).copy) then exportModule env outDir m
  discard <| act.run {}
