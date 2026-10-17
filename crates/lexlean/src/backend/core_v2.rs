//! The Lean runtime of core module 2.  It decodes a chunk of the module's
//! data into `Lean.Expr` and `Lean.Declaration` values, submits every row to
//! the kernel through `Lean.addDecl`, and then installs, row by row, the
//! environment state a downstream elaboration reads (SPEC.md §17.10): the
//! structure, class, instance, and reducibility registrations of core module
//! 1, plus tags, no-confusion shapes, matcher and projection registrations,
//! simplification-set membership, structural-recursion equation data, and the
//! code generator's pass over the rows the source environment compiled.  It
//! reads data and never produces a declaration the data does not name.
//!
//! The text is generated Lean and the audit of generated Lean forbids a
//! comment, a character literal, and the words of §18.2, so what its parts
//! do is recorded here instead. A partial definition, or one that is not safe, may refer to
//! itself and to the others of its group, so a group is one mutual
//! declaration, submitted with its first member. The code generator runs
//! over the rows the source environment compiled in this order: the inductive
//! types, then the compiler's partial implementation records, then the
//! remaining definitions by group. The extension state of a row is installed
//! after the whole module is in the environment, except that a sparse
//! `casesOn` is marked after its own code is generated, as the elaborator that
//! made it did (the mark makes the code generator treat the constant as a
//! primitive), and that a definition carrying a sparse record is not submitted
//! at all: the elaborator's own construction remakes it, and the row is
//! refused unless it is what the construction makes, since the record that
//! equation-lemma generation reads is private to that construction. The
//! elaborator numbers the constants of one owner in the order it makes them,
//! and the rows are in the order of their dependencies, so the construction is
//! told which number the row bears, and whether the name is a private one,
//! which the construction makes of a name made outside an exporting context
//! in a module.

/// The decoder module of core module 2, spliced after the generated module's
/// header.
pub const LEAN_CORE_DECODER_V2: &str = r##"
namespace LexLeanCore.Runtime

private def fail {α : Type} (message : String) : Except String α := .error message

private def field (json : Json) (name : String) : Except String Json :=
  json.getObjVal? name

private def stringField (json : Json) (name : String) : Except String String := do
  (← field json name).getStr?

private def natField (json : Json) (name : String) : Except String Nat := do
  (← field json name).getNat?

private def boolField (json : Json) (name : String) : Except String Bool := do
  (← field json name).getBool?

private def arrayField (json : Json) (name : String) : Except String (Array Json) := do
  (← field json name).getArr?

private def optionalField (json : Json) (name : String) : Option Json :=
  match json.getObjVal? name with
  | .ok value => some value
  | .error _ => none

private def unescapeSegment (segment : String) : String := Id.run do
  let mut out := ""
  let mut pending := false
  for c in segment.toList do
    if pending then
      out := out.push (if c.toNat == 100 then Char.ofNat 46 else if c.toNat == 104 then Char.ofNat 35 else c)
      pending := false
    else if c.toNat == 92 then pending := true
    else out := out.push c
  return out

private def segmentIsNumeric (segment : String) : Bool :=
  segment.startsWith "#" && segment.length > 1 && (segment.drop 1).all Char.isDigit

private def nameOf (value : String) : Name :=
  if value.isEmpty then .anonymous else
  value.splitOn "." |>.foldl (fun name part =>
    if segmentIsNumeric part then name.num (part.drop 1).toNat! else name.str (unescapeSegment part)) .anonymous

private partial def levelOf (json : Json) : Except String Level := do
  match ← stringField json "k" with
  | "z" => pure .zero
  | "s" => pure (.succ (← levelOf (← field json "a")))
  | "m" | "i" =>
      let values ← (← field json "a").getArr?
      let some left := values[0]? | fail "core level pair lacks its left value"
      let some right := values[1]? | fail "core level pair lacks its right value"
      if values.size != 2 then fail "core level pair has extra values"
      let left ← levelOf left
      let right ← levelOf right
      if (← stringField json "k") == "m" then pure (.max left right) else pure (.imax left right)
  | "p" => pure (.param (nameOf (← stringField json "a")))
  | kind => fail s!"unknown core level kind '{kind}'"

private def levelsOf (json : Json) : Except String (List Level) := do
  let values ← json.getArr?
  values.toList.mapM levelOf

private def namesOf (json : Json) : Except String (List Name) := do
  let values ← json.getArr?
  values.toList.mapM fun value => do pure (nameOf (← value.getStr?))

private def binderOf (value : String) : Except String BinderInfo :=
  match value with
  | "e" => pure .default
  | "i" => pure .implicit
  | "s" => pure .strictImplicit
  | "c" => pure .instImplicit
  | kind => fail s!"unknown core binder kind '{kind}'"

private def nodeAt (nodes : Array Expr) (index : Nat) : Except String Expr :=
  match nodes[index]? with
  | some node => .ok node
  | none => fail s!"core node reference {index} is out of range"

private def dataValueOf (json : Json) : Except String DataValue := do
  match ← stringField json "t" with
  | "b" => return .ofBool (← boolField json "v")
  | "n" =>
    let some n := (← stringField json "v").toNat? | fail "invalid mdata natural"
    return .ofNat n
  | "i" =>
    let some n := (← stringField json "v").toInt? | fail "invalid mdata integer"
    return .ofInt n
  | "s" => return .ofString (← stringField json "v")
  | "m" => return .ofName (nameOf (← stringField json "v"))
  | other => fail s!"unknown mdata value kind '{other}'"

private def nodeOf (nodes : Array Expr) (json : Json) : Except String Expr := do
  match ← stringField json "k" with
  | "b" => pure (.bvar (← natField json "i"))
  | "s" => pure (.sort (← levelOf (← field json "l")))
  | "c" => pure (.const (nameOf (← stringField json "n")) (← levelsOf (← field json "u")))
  | "a" => pure (.app (← nodeAt nodes (← natField json "f")) (← nodeAt nodes (← natField json "x")))
  | "l" => pure (.lam (nameOf (← stringField json "n")) (← nodeAt nodes (← natField json "t"))
      (← nodeAt nodes (← natField json "v")) (← binderOf (← stringField json "b")))
  | "p" => pure (.forallE (nameOf (← stringField json "n")) (← nodeAt nodes (← natField json "t"))
      (← nodeAt nodes (← natField json "v")) (← binderOf (← stringField json "b")))
  | "e" => pure (.letE (nameOf (← stringField json "n")) (← nodeAt nodes (← natField json "t"))
      (← nodeAt nodes (← natField json "v")) (← nodeAt nodes (← natField json "body"))
      (← boolField json "d"))
  | "n" =>
      let value ← stringField json "v"
      let some value := value.toNat? | fail s!"invalid core natural literal '{value}'"
      pure (.lit (.natVal value))
  | "t" =>
      let values ← (← field json "v").getArr?
      let chars ← values.toList.mapM fun value => do pure (Char.ofNat (← value.getNat?))
      pure (.lit (.strVal (String.ofList chars)))
  | "j" => pure (.proj (nameOf (← stringField json "s")) (← natField json "i")
      (← nodeAt nodes (← natField json "x")))
  | "d" =>
      let rows ← arrayField json "m"
      let mut data : KVMap := {}
      for row in rows do
        data := data.insert (nameOf (← stringField row "n")) (← dataValueOf (← field row "v"))
      pure (.mdata data (← nodeAt nodes (← natField json "x")))
  | kind => fail s!"unknown core node kind '{kind}'"

private def liftString {α : Type} : Except String α → CoreM α
  | .ok value => pure value
  | .error message => throwError message

private def findDeclaration (declarations : Array Json) (name : String) : CoreM Json := do
  for declaration in declarations do
    if (← liftString (stringField declaration "name")) == name then return declaration
  throwError s!"core declaration '{name}' is absent"

private def sparseIndices (last : String) : Option (List Nat) :=
  match last.dropPrefix? "_sparseCasesOn_" with
  | none => none
  | some rest =>
      let numbers := rest.toString.splitOn "_" |>.map String.toNat?
      if numbers.any Option.isNone then none else
      let numbers := numbers.filterMap id
      if numbers.isEmpty then none else some numbers.reverse

private def addOneUnchecked (nodes : Array Expr) (declarations : Array Json)
    (json : Json) : CoreM Unit := do
  if ← liftString (boolField json "generated") then return
  let nameString ← liftString (stringField json "name")
  let name := nameOf nameString
  let levelParams ← liftString (namesOf (← liftString (field json "levels")))
  let type ← liftString (nodeAt nodes (← liftString (natField json "type")))
  let kindString ← liftString (stringField json "kind")
  let all ← match optionalField json "all" with
    | some value => liftString (namesOf value)
    | none => pure [name]
  if let some sparse := optionalField json "sparse" then
    let indName := nameOf (← liftString (stringField sparse "ind_name"))
    let constructors ← liftString (namesOf (← liftString (field sparse "interesting_ctors")))
    let majorPos ← liftString (natField sparse "major_pos")
    let arity ← liftString (natField sparse "arity")
    let (owner, idx, parentIdxs) ← match name with
      | .str parent last =>
          match sparseIndices last with
          | some (idx :: parents) => pure (parent, idx, parents)
          | _ => throwError s!"core declaration '{name}' is not named like a sparse casesOn"
      | _ => throwError s!"core declaration '{name}' is not named like a sparse casesOn"
    Lean.enableRealizationsForConst indName
    Lean.Meta.MetaM.run' <| Lean.Meta.realizeConst indName name do
      setDeclNGen { namePrefix := owner, idx, parentIdxs }
      let made ← Lean.withExporting (isExporting := !Lean.isPrivateName name)
        (Lean.Meta.mkSparseCasesOn indName constructors.toArray)
      unless made == name do
        throwError s!"the sparse casesOn the elaborator makes is '{made}', not '{name}'"
    let some made := (← getEnv).find? name | throwError s!"core declaration '{name}' is absent"
    let value ← liftString (nodeAt nodes (← liftString (natField json "value")))
    unless made.type == type && made.value? == some value do
      throwError s!"core declaration '{name}' is not the sparse casesOn the elaborator makes"
    let some info := Lean.Meta.getSparseCasesOnInfoCore (← getEnv) name
      | throwError s!"core declaration '{name}' has no sparse casesOn record"
    unless info.indName == indName && info.majorPos == majorPos && info.arity == arity
        && info.insterestingCtors.toList == constructors do
      throwError s!"core declaration '{name}' states another sparse casesOn record than the elaborator keeps"
    return
  match kindString with
  | "inductive" =>
      let metadata ← liftString (field json "inductive")
      let numParams ← liftString (natField metadata "num_params")
      let constructorNames ← liftString (arrayField metadata "constructors")
      let mut constructors : List Constructor := []
      for constructorName in constructorNames do
        let constructorName ← liftString constructorName.getStr?
        let row ← findDeclaration declarations constructorName
        let constructorType ← liftString (nodeAt nodes (← liftString (natField row "type")))
        constructors := constructors.concat { name := nameOf constructorName, type := constructorType }
      Lean.addDecl (.inductDecl levelParams numParams [{ name, type, ctors := constructors }] false)
        (forceExpose := true)
  | "abbrev" | "definition" =>
      let valueIndex ← liftString (natField json "value")
      let value ← liftString (nodeAt nodes valueIndex)
      let metadata ← liftString (field json "reducibility")
      let reducibilityKind ← liftString (stringField metadata "kind")
      let hints ← match reducibilityKind with
        | "abbrev" => pure ReducibilityHints.abbrev
        | "opaque" => pure (default : ReducibilityHints)
        | "regular" => pure (ReducibilityHints.regular
            (UInt32.ofNat (← liftString (natField metadata "height"))))
        | other => throwError s!"unknown core reducibility kind '{other}'"
      let safety ← match optionalField json "safety" with
        | none => pure DefinitionSafety.safe
        | some v => match ← liftString v.getStr? with
          | "partial" => pure DefinitionSafety.partial
          | "unsafe" => pure DefinitionSafety.«unsafe»
          | other => throwError s!"unknown core definition safety '{other}'"
      let defn : DefinitionVal := { name, levelParams, type, value, hints, safety, all }
      if safety == .safe then Lean.addDecl (.defnDecl defn) (forceExpose := true)
      else
        if (← getEnv).contains name then return
        let mut group : List DefinitionVal := []
        for member in all do
          let row ← findDeclaration declarations member.toString
          let memberType ← liftString (nodeAt nodes (← liftString (natField row "type")))
          let memberValue ← liftString (nodeAt nodes (← liftString (natField row "value")))
          let memberLevels ← liftString (namesOf (← liftString (field row "levels")))
          let memberMetadata ← liftString (field row "reducibility")
          let memberHints ← match ← liftString (stringField memberMetadata "kind") with
            | "abbrev" => pure ReducibilityHints.abbrev
            | "opaque" => pure (default : ReducibilityHints)
            | "regular" => pure (ReducibilityHints.regular
                (UInt32.ofNat (← liftString (natField memberMetadata "height"))))
            | other => throwError s!"unknown core reducibility kind '{other}'"
          let memberDefn : DefinitionVal := {
            name := member
            levelParams := memberLevels
            type := memberType
            value := memberValue
            hints := memberHints
            safety
            all }
          group := group.concat memberDefn
        Lean.addDecl (.mutualDefnDecl group) (forceExpose := true)
  | "theorem" =>
      let value ← liftString (nodeAt nodes (← liftString (natField json "value")))
      Lean.addDecl (.thmDecl { name, levelParams, type, value, all }) (forceExpose := true)
  | "constructor" | "recursor" =>
      throwError s!"non-generated primitive core declaration '{nameString}'"
  | kind => throwError s!"unknown core declaration kind '{kind}'"

private def addOne (nodes : Array Expr) (declarations : Array Json) (json : Json) : CoreM Unit := do
  let name ← liftString (stringField json "name")
  try
    addOneUnchecked nodes declarations json
  catch error =>
    throwError m!"native core declaration '{name}':{indentD error.toMessageData}"

private def decodeNodeBatch (nodes : Array Expr) (data : String) : CoreM (Array Expr) := do
  let json ← liftString (Json.parse data)
  let rows ← liftString json.getArr?
  let mut nodes := nodes
  for row in rows do
    nodes := nodes.push (← liftString (nodeOf nodes row))
  pure nodes

private def natsOf (json : Json) : Except String (Array Nat) := do
  (← json.getArr?).mapM fun v => v.getNat?

private def optNatOf (json : Json) : Except String (Option Nat) := do
  let values ← json.getArr?
  match values.size with
  | 0 => pure none
  | 1 => pure (some (← values[0]!.getNat?))
  | _ => fail "an optional natural has more than one value"

private def installMetadata (nodes : Array Expr) (declaration : Json) : CoreM Unit := do
  let name := nameOf (← liftString (stringField declaration "name"))
  let status ← match ← liftString (stringField declaration "transparency") with
    | "reducible" => pure ReducibilityStatus.reducible
    | "semireducible" => pure ReducibilityStatus.semireducible
    | "irreducible" => pure ReducibilityStatus.irreducible
    | "implicit-reducible" => pure ReducibilityStatus.implicitReducible
    | other => throwError s!"unknown core transparency '{other}'"
  Lean.setReducibilityStatus name status
  if let some kind := optionalField declaration "inline" then
    let kind ← match ← liftString kind.getStr? with
      | "inline" => pure Lean.Compiler.InlineAttributeKind.inline
      | "noinline" => pure Lean.Compiler.InlineAttributeKind.noinline
      | "macro_inline" => pure Lean.Compiler.InlineAttributeKind.macroInline
      | "inline_if_reduce" => pure Lean.Compiler.InlineAttributeKind.inlineIfReduce
      | "always_inline" => pure Lean.Compiler.InlineAttributeKind.alwaysInline
      | other => throwError s!"unknown core inlining attribute '{other}'"
    match Lean.Compiler.setInlineAttribute (← getEnv) name kind with
    | .ok env => setEnv env
    | .error message => throwError message
  if let some tags := optionalField declaration "tags" then
    for tag in ← liftString tags.getArr? do
      match ← liftString tag.getStr? with
      | "protected" => modifyEnv (Lean.addProtected · name)
      | "aux_recursor" => modifyEnv (Lean.markAuxRecursor · name)
      | "elab_as_elim" => Lean.Elab.Term.elabAsElim.setTag name
      | "defeq" => Lean.defeqAttr.setTag name
      | "backward_defeq" => Lean.backwardDefeqAttr.setTag name
      | "recursive" => modifyEnv (Lean.Meta.recExt.tag · name)
      | "sparse_cases_on" => pure ()
      | other => throwError s!"unknown core declaration tag '{other}'"
  if let some info := optionalField declaration "no_confusion" then
    let arity ← liftString (natField info "arity")
    let value ← match ← liftString (stringField info "kind") with
      | "regular" => pure (Lean.NoConfusionInfo.regular arity (← liftString (natField info "lhs")) (← liftString (natField info "rhs")))
      | "per_ctor" => pure (Lean.NoConfusionInfo.perCtor arity (← liftString (natField info "fields")))
      | other => throwError s!"unknown no-confusion kind '{other}'"
    modifyEnv (Lean.markNoConfusion · name value)
  if let some info := optionalField declaration "matcher" then
    let altRows ← liftString (arrayField info "alts")
    let mut alts : Array Lean.Meta.Match.AltParamInfo := #[]
    for row in altRows do
      let numFields ← liftString (natField row "fields")
      let numOverlaps ← liftString (natField row "overlaps")
      let hasUnitThunk ← liftString (boolField row "unit_thunk")
      alts := alts.push { numFields, numOverlaps, hasUnitThunk }
    let discrRows ← liftString (arrayField info "discrs")
    let mut discrs : Array Lean.Meta.Match.DiscrInfo := #[]
    for row in discrRows do
      let cells ← liftString row.getArr?
      match cells.size with
      | 0 => discrs := discrs.push {}
      | 1 => discrs := discrs.push { hName? := some (nameOf (← liftString cells[0]!.getStr?)) }
      | _ => throwError "a matcher discriminant has more than one name"
    let uElimPos? ← match optionalField info "u_elim_pos" with
      | some v => pure (some (← liftString v.getNat?))
      | none => pure none
    let numParams ← liftString (natField info "num_params")
    let numDiscrs ← liftString (natField info "num_discrs")
    let mut overlaps : Lean.Meta.Match.Overlaps := {}
    for row in ← liftString (arrayField info "overlaps") do
      let pair ← liftString row.getArr?
      let some overlappedJson := pair[0]? | throwError "overlap entry lacks its key"
      let some overlappingJson := pair[1]? | throwError "overlap entry lacks its values"
      let overlapped ← liftString overlappedJson.getNat?
      for overlapping in ← liftString (natsOf overlappingJson) do
        overlaps := overlaps.insert overlapping overlapped
    modifyEnv (Lean.Meta.Match.Extension.addMatcherInfo · name
      { numParams, numDiscrs, altInfos := alts, uElimPos?, discrInfos := discrs, overlaps })
  if let some info := optionalField declaration "projection" then
    modifyEnv (Lean.addProjectionFnInfo · name (nameOf (← liftString (stringField info "constructor")))
      (← liftString (natField info "num_params")) (← liftString (natField info "index")) (← liftString (boolField info "from_class")))
  if let some info := optionalField declaration "simp" then
    let kind ← match ← liftString (stringField info "attribute") with
      | "global" => pure AttributeKind.global
      | "local" => pure AttributeKind.local
      | "scoped" => pure AttributeKind.scoped
      | other => throwError s!"unknown core simp attribute kind '{other}'"
    Lean.Meta.MetaM.run' <| Lean.Meta.addSimpTheorem Lean.Meta.simpExtension name
      (← liftString (boolField info "post")) false kind (← liftString (natField info "priority"))
  if let some info := optionalField declaration "eqn_info" then
    let permRows ← liftString (arrayField info "perms")
    let mut perms : Array (Array (Option Nat)) := #[]
    for row in permRows do
      let cells ← liftString row.getArr?
      perms := perms.push (← liftString (cells.mapM optNatOf))
    let revRows ← liftString (arrayField info "rev_deps")
    let mut rev : Array (Array (Array Nat)) := #[]
    for a in revRows do
      let mut inner : Array (Array Nat) := #[]
      for b in ← liftString a.getArr? do inner := inner.push (← liftString (natsOf b))
      rev := rev.push inner
    let levelParams ← liftString (namesOf (← liftString (field info "levels")))
    let type ← liftString (nodeAt nodes (← liftString (natField info "type")))
    let value ← liftString (nodeAt nodes (← liftString (natField info "value")))
    let recArgPos ← liftString (natField info "rec_arg_pos")
    let declNames := (← liftString (namesOf (← liftString (field info "declarations")))).toArray
    let numFixed ← liftString (natField info "num_fixed")
    let value : Lean.Elab.Structural.EqnInfo := {
      declName := name, levelParams, type, value, recArgPos, declNames,
      fixedParamPerms := { numFixed, perms, revDeps := rev } }
    modifyEnv (Lean.Elab.Structural.eqnInfoExt.insert · name value)

private def compileRows (declarations : Array Json) : CoreM Unit := do
  let mut compiledRows : Array (Json × Name) := #[]
  for declaration in declarations do
    if let some flag := optionalField declaration "compiled" then
      if ← liftString flag.getBool? then
        compiledRows := compiledRows.push (declaration, nameOf (← liftString (stringField declaration "name")))
  for (declaration, name) in compiledRows do
    if (← liftString (stringField declaration "kind")) == "inductive" then
      Lean.compileDecls #[name]
  for (declaration, name) in compiledRows do
    if (← liftString (stringField declaration "kind")) != "inductive" && (optionalField declaration "safety").isSome then
      Lean.compileDecls #[name]
  let mut done : NameSet := {}
  for declaration in declarations do
    let name := nameOf (← liftString (stringField declaration "name"))
    if (← liftString (stringField declaration "kind")) != "inductive" && (optionalField declaration "safety").isNone
        && compiledRows.any (·.2 == name) && !done.contains name then
      let all ← match optionalField declaration "all" with
        | some v => liftString (namesOf v)
        | none => pure [name]
      let group := all.filter fun n => compiledRows.any (·.2 == n)
      for n in group do done := done.insert n
      Lean.compileDecls group.toArray
    if let some tags := optionalField declaration "tags" then
      for tag in ← liftString tags.getArr? do
        if (← liftString tag.getStr?) == "sparse_cases_on" then
          modifyEnv (Lean.markSparseCasesOn · name)

def decodeAndAdd (nodeBatches : Array String) (data : String) : CoreM Unit := do
  let mut nodes : Array Expr := #[]
  for batch in nodeBatches do
    nodes ← decodeNodeBatch nodes batch
  let document ← liftString (Json.parse data)
  let declarations ← liftString (arrayField document "declarations")
  let order ← liftString (arrayField document "order")
  for index in order do
    let index ← liftString index.getNat?
    let some declaration := declarations[index]? |
      throwError s!"core declaration order index {index} is out of range"
    addOne nodes declarations declaration
  for declaration in declarations do
    if (← liftString (stringField declaration "kind")) == "inductive" then
      let metadata ← liftString (field declaration "inductive")
      if let some structureData := optionalField metadata "structure" then
        let name := nameOf (← liftString (stringField declaration "name"))
        let fieldRows ← liftString (arrayField structureData "fields")
        let mut fields : Array StructureFieldInfo := #[]
        for row in fieldRows do
          let subobject? : Option Name ← match optionalField row "subobject" with
            | some value => pure (some (nameOf (← liftString value.getStr?)))
            | none => pure none
          let autoParam? : Option Expr ← match optionalField row "auto_param" with
            | some value => pure (some (← liftString (nodeAt nodes (← liftString value.getNat?))))
            | none => pure none
          let fieldName := nameOf (← liftString (stringField row "name"))
          let projFn := nameOf (← liftString (stringField row "projection"))
          let binderInfo ← liftString (binderOf (← liftString (stringField row "binder")))
          fields := fields.push {
            fieldName
            projFn
            subobject?
            binderInfo
            autoParam? }
        setEnv <| Lean.registerStructure (← getEnv) { structName := name, fields }
  for declaration in declarations do
    if (← liftString (stringField declaration "kind")) == "inductive" then
      let metadata ← liftString (field declaration "inductive")
      if let some structureData := optionalField metadata "structure" then
        let name := nameOf (← liftString (stringField declaration "name"))
        let parentRows ← liftString (arrayField structureData "parents")
        let mut parents : Array StructureParentInfo := #[]
        for row in parentRows do
          let structName := nameOf (← liftString (stringField row "name"))
          let subobject ← liftString (boolField row "subobject")
          let projFn := nameOf (← liftString (stringField row "projection"))
          parents := parents.push {
            structName
            subobject
            projFn }
        Lean.setStructureParents name parents
  for declaration in declarations do
    if (← liftString (stringField declaration "kind")) == "inductive" then
      let metadata ← liftString (field declaration "inductive")
      if (optionalField metadata "structure").isSome then
        discard <| Lean.computeStructureResolutionOrder
          (nameOf (← liftString (stringField declaration "name"))) true
  for declaration in declarations do
    installMetadata nodes declaration
  compileRows declarations
  for declaration in declarations do
    if let some kind := optionalField declaration "grind" then
      let name := nameOf (← liftString (stringField declaration "name"))
      let kind ← match ← liftString kind.getStr? with
        | "eq_lhs" => pure (Lean.Meta.Grind.EMatchTheoremKind.eqLhs false)
        | "eq_rhs" => pure (Lean.Meta.Grind.EMatchTheoremKind.eqRhs false)
        | "eq_both" => pure (Lean.Meta.Grind.EMatchTheoremKind.eqBoth false)
        | "eq_bwd" => pure Lean.Meta.Grind.EMatchTheoremKind.eqBwd
        | "fwd" => pure Lean.Meta.Grind.EMatchTheoremKind.fwd
        | "bwd" => pure (Lean.Meta.Grind.EMatchTheoremKind.bwd false)
        | "left_right" => pure Lean.Meta.Grind.EMatchTheoremKind.leftRight
        | "right_left" => pure Lean.Meta.Grind.EMatchTheoremKind.rightLeft
        | "default" => pure (Lean.Meta.Grind.EMatchTheoremKind.default false)
        | other => throwError s!"unknown core grind attribute '{other}'"
      Lean.Meta.MetaM.run' (Lean.Meta.Grind.grindExt.addEMatchAttr name AttributeKind.global kind {})
  for declaration in declarations do
    let generated ← liftString (boolField declaration "generated")
    if !generated && (← liftString (boolField declaration "class")) then
      let name := nameOf (← liftString (stringField declaration "name"))
      match Lean.addClass (← getEnv) name with
      | .ok env => setEnv env
      | .error message => throwError message
  for declaration in declarations do
    if let some metadata := optionalField declaration "instance" then
      let name := nameOf (← liftString (stringField declaration "name"))
      let priority ← liftString (natField metadata "priority")
      let attributeKind ← match ← liftString (stringField metadata "attribute") with
        | "global" => pure AttributeKind.global
        | "local" => pure AttributeKind.local
        | "scoped" => pure AttributeKind.scoped
        | other => throwError s!"unknown core instance attribute kind '{other}'"
      Lean.Meta.MetaM.run' <| Lean.Meta.addInstance name attributeKind priority
      let expectedRows ← liftString (arrayField metadata "synthesis_order")
      let mut expected : Array Nat := #[]
      for row in expectedRows do expected := expected.push (← liftString row.getNat?)
      let some actual := (Lean.Meta.instanceExtension.getState (← getEnv)
          |>.instanceNames.find? name) |
        throwError s!"core instance '{name}' was not registered"
      unless actual.synthOrder == expected do
        throwError s!"core instance '{name}' reconstructed synthesis order {actual.synthOrder.toList} instead of {expected.toList}"

end LexLeanCore.Runtime
"##;
