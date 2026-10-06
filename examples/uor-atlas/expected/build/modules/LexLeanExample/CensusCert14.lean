module
public import Init
public import LexLeanExample.CensusLeaf14_0
public import LexLeanExample.CensusLeaf14_1
public import LexLeanExample.CensusLeaf14_2
public import LexLeanExample.CensusLeaf14_3
public import LexLeanExample.CensusLeaf14_4
public import LexLeanExample.CensusLeaf14_5
public import LexLeanExample.CensusLeaf14_6
public import LexLeanExample.CensusLeaf14_7
public import LexLeanExample.CensusLeaf14_8
public import LexLeanExample.CensusLeaf14_9
public import LexLeanExample.CensusClosure
import Lean
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option Elab.async false

open Lean

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

private def nameOf (value : String) : Name :=
  value.splitOn "." |>.foldl (fun name part => name.str part) .anonymous

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

private def nodeOf (nodes : Array Expr) (json : Json) : Except String Expr := do
  match ← stringField json "k" with
  | "b" => pure (.bvar (← natField json "i"))
  | "s" => pure (.sort (← levelOf (← field json "l")))
  | "c" => pure (.const (nameOf (← stringField json "n")) (← levelsOf (← field json "u")))
  | "a" => pure (.app (← nodeAt nodes (← natField json "f")) (← nodeAt nodes (← natField json "x")))
  | "l" => pure (.lam `_ (← nodeAt nodes (← natField json "t"))
      (← nodeAt nodes (← natField json "v")) (← binderOf (← stringField json "b")))
  | "p" => pure (.forallE `_ (← nodeAt nodes (← natField json "t"))
      (← nodeAt nodes (← natField json "v")) (← binderOf (← stringField json "b")))
  | "e" => pure (.letE `_ (← nodeAt nodes (← natField json "t"))
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
  | kind => fail s!"unknown core node kind '{kind}'"

private def liftString {α : Type} : Except String α → CoreM α
  | .ok value => pure value
  | .error message => throwError message

private def findDeclaration (declarations : Array Json) (name : String) : CoreM Json := do
  for declaration in declarations do
    if (← liftString (stringField declaration "name")) == name then return declaration
  throwError s!"core declaration '{name}' is absent"

private def addOneUnchecked (nodes : Array Expr) (declarations : Array Json)
    (json : Json) : CoreM Unit := do
  if ← liftString (boolField json "generated") then return
  let nameString ← liftString (stringField json "name")
  let name := nameOf nameString
  let levelParams ← liftString (namesOf (← liftString (field json "levels")))
  let type ← liftString (nodeAt nodes (← liftString (natField json "type")))
  let kindString ← liftString (stringField json "kind")
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
      Lean.addDecl (.defnDecl { name, levelParams, type, value, hints, safety := .safe })
        (forceExpose := true)
  | "theorem" =>
      let value ← liftString (nodeAt nodes (← liftString (natField json "value")))
      Lean.addDecl (.thmDecl { name, levelParams, type, value }) (forceExpose := true)
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
    let name := nameOf (← liftString (stringField declaration "name"))
    let status ← match ← liftString (stringField declaration "transparency") with
      | "reducible" => pure ReducibilityStatus.reducible
      | "semireducible" => pure ReducibilityStatus.semireducible
      | "irreducible" => pure ReducibilityStatus.irreducible
      | "implicit-reducible" => pure ReducibilityStatus.implicitReducible
      | other => throwError s!"unknown core transparency '{other}'"
    Lean.setReducibilityStatus name status
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

run_cmd Lean.Elab.Command.liftCoreM (LexLeanCore.Runtime.decodeAndAdd #[
  (
    "[{\"k\":\"c\",\"n\":\"Eq\",\"u\":[{\"k\":\"s\",\"a\":{\"k\":\"z\"}}]},{\"k\":\"c\",\"n\":\"Bool\",\"u\":[]},{\"k\":\"a\",\"f\":0,\"x\":1},{\"k\":\"c\",\"n\":\"UorAtlas.Census.gramRange\",\"u\":[]},{\"k\":\"c\",\"n\":\"OfNat.ofNat\",\"u\":[{\"k\":\"z\"}]},{\"k\":\"c\",\"n\":\"Nat\",\"u\":[]},{\"k\":\"a\",\"f\":4,\"x\":5},{\"k\":\"n\",\"v\":\"14000\"},{\"k\":\"a\",\"f\":6,\"x\":7},{\"k\":\"c\",\"n\":\"instOfNatNat\",\"u\":[]},{\"k\":\"a\",\"f\":9,\"x\":7},{\"k\":\"a\",\"f\":8,\"x\":10},{\"k\":\"a\",\"f\":3,\"x\":11},{\"k\":\"n\",\"v\":\"1000\"},{\"k\":\"a\",\"f\":6,\"x\":13},{\"k\":\"a\",\"f\":9,\"x\":13},{\"k\":\"a\",\"f\":14,\"x\":15},{\"k\":\"a\",\"f\":12,\"x\":16},{\"k\":\"a\",\"f\":2,\"x\":17},{\"k\":\"c\",\"n\":\"Bool.true\",\"u\":[]},{\"k\":\"a\",\"f\":18,\"x\":19},{\"k\":\"c\",\"n\":\"UorAtlas.Census.gramRange_append\",\"u\":[]},{\"k\":\"c\",\"n\":\"UorAtlas.Census.gramLeaf14_0\",\"u\":[]},{\"k\":\"n\",\"v\":\"14000\"},{\"k\":\"a\",\"f\":21,\"x\":23},{\"k\":\"n\",\"v\":\"100\"},{\"k\":\"a\",\"f\":24,\"x\":25},{\"k\":\"n\",\"v\":\"900\"},{\"k\":\"a\",\"f\":26,\"x\":27},{\"k\":\"a\",\"f\":28,\"x\":22},{\"k\":\"c\",\"n\":\"UorAtlas.Census.gramLeaf14_1\",\"u\":[]},{\"k\":\"n\",\"v\":\"14100\"},{\"k\":\"a\",\"f\":21,\"x\":31},{\"k\":\"n\",\"v\":\"100\"},{\"k\":\"a\",\"f\":32,\"x\":33},{\"k\":\"n\",\"v\":\"800\"},{\"k\":\"a\",\"f\":34,\"x\":35},{\"k\":\"a\",\"f\":36,\"x\":30},{\"k\":\"c\",\"n\":\"UorAtlas.Census.gramLeaf14_2\",\"u\":[]},{\"k\":\"n\",\"v\":\"14200\"},{\"k\":\"a\",\"f\":21,\"x\":39},{\"k\":\"n\",\"v\":\"100\"},{\"k\":\"a\",\"f\":40,\"x\":41},{\"k\":\"n\",\"v\":\"700\"},{\"k\":\"a\",\"f\":42,\"x\":43},{\"k\":\"a\",\"f\":44,\"x\":38},{\"k\":\"c\",\"n\":\"UorAtlas.Census.gramLeaf14_3\",\"u\":[]},{\"k\":\"n\",\"v\":\"14300\"},{\"k\":\"a\",\"f\":21,\"x\":47},{\"k\":\"n\",\"v\":\"100\"},{\"k\":\"a\",\"f\":48,\"x\":49},{\"k\":\"n\",\"v\":\"600\"},{\"k\":\"a\",\"f\":50,\"x\":51},{\"k\":\"a\",\"f\":52,\"x\":46},{\"k\":\"c\",\"n\":\"UorAtlas.Census.gramLeaf14_4\",\"u\":[]},{\"k\":\"n\",\"v\":\"14400\"},{\"k\":\"a\",\"f\":21,\"x\":55},{\"k\":\"n\",\"v\":\"100\"},{\"k\":\"a\",\"f\":56,\"x\":57},{\"k\":\"n\",\"v\":\"500\"},{\"k\":\"a\",\"f\":58,\"x\":59},{\"k\":\"a\",\"f\":60,\"x\":54},{\"k\":\"c\",\"n\":\"UorAtlas.Census.gramLeaf14_5\",\"u\":[]},{\"k\":\"n\",\"v\":\"14500\"},{\"k\":\"a\",\"f\":21,\"x\":63},{\"k\":\"n\",\"v\":\"100\"},{\"k\":\"a\",\"f\":64,\"x\":65},{\"k\":\"n\",\"v\":\"400\"},{\"k\":\"a\",\"f\":66,\"x\":67},{\"k\":\"a\",\"f\":68,\"x\":62},{\"k\":\"c\",\"n\":\"UorAtlas.Census.gramLeaf14_6\",\"u\":[]},{\"k\":\"n\",\"v\":\"14600\"},{\"k\":\"a\",\"f\":21,\"x\":71},{\"k\":\"n\",\"v\":\"100\"},{\"k\":\"a\",\"f\":72,\"x\":73},{\"k\":\"n\",\"v\":\"300\"},{\"k\":\"a\",\"f\":74,\"x\":75},{\"k\":\"a\",\"f\":76,\"x\":70},{\"k\":\"c\",\"n\":\"UorAtlas.Census.gramLeaf14_7\",\"u\":[]},{\"k\":\"n\",\"v\":\"14700\"},{\"k\":\"a\",\"f\":21,\"x\":79},{\"k\":\"n\",\"v\":\"100\"},{\"k\":\"a\",\"f\":80,\"x\":81},{\"k\":\"n\",\"v\":\"200\"},{\"k\":\"a\",\"f\":82,\"x\":83},{\"k\":\"a\",\"f\":84,\"x\":78},{\"k\":\"c\",\"n\":\"UorAtlas.Census.gramLeaf14_8\",\"u\":[]},{\"k\":\"n\",\"v\":\"14800\"},{\"k\":\"a\",\"f\":21,\"x\":87},{\"k\":\"n\",\"v\":\"100\"},{\"k\":\"a\",\"f\":88,\"x\":89},{\"k\":\"n\",\"v\":\"100\"},{\"k\":\"a\",\"f\":90,\"x\":91},{\"k\":\"a\",\"f\":92,\"x\":86},{\"k\":\"c\",\"n\":\"UorAtlas.Census.gramLeaf14_9\",\"u\":[]},{\"k\":\"a\",\"f\":93,\"x\":94},{\"k\":\"a\",\"f\":85,\"x\":95},{\"k\":\"a\",\"f\":77,\"x\":96},{\"k\":\"a\",\"f\":69,\"x\":97},{\"k\":\"a\",\"f\":61,\"x\":98},{\"k\":\"a\",\"f\":53,\"x\":99},{\"k\":\"a\",\"f\":45,\"x\":100},{\"k\":\"a\",\"f\":37,\"x\":101},{\"k\":\"a\",\"f\":29,\"x\":102},{\"k\":\"c\",\"n\":\"UorAtlas.LexLeanInternal.decl260\",\"u\":[]}]"
    )
] (
  "{\"declarations\":[{\"name\":\"UorAtlas.LexLeanInternal.decl260\",\"levels\":[],\"kind\":\"theorem\",\"transparency\":\"semireducible\",\"policy\":{\"kind\":\"allow\",\"axioms\":[\"Classical.choice\",\"Quot.sound\",\"propext\"]},\"type\":20,\"value\":103,\"class\":false,\"generated\":false},{\"name\":\"UorAtlas.Census.gramWin14\",\"levels\":[],\"kind\":\"theorem\",\"transparency\":\"semireducible\",\"policy\":{\"kind\":\"allow\",\"axioms\":[\"Classical.choice\",\"Quot.sound\",\"propext\"]},\"type\":20,\"value\":104,\"class\":false,\"generated\":false}],\"imports\":[\"Init\",\"LexLeanExample.CensusLeaf14_0\",\"LexLeanExample.CensusLeaf14_1\",\"LexLeanExample.CensusLeaf14_2\",\"LexLeanExample.CensusLeaf14_3\",\"LexLeanExample.CensusLeaf14_4\",\"LexLeanExample.CensusLeaf14_5\",\"LexLeanExample.CensusLeaf14_6\",\"LexLeanExample.CensusLeaf14_7\",\"LexLeanExample.CensusLeaf14_8\",\"LexLeanExample.CensusLeaf14_9\",\"LexLeanExample.CensusClosure\"],\"order\":[0,1],\"proof_nodes\":[21,22,24,26,28,29,30,32,34,36,37,38,40,42,44,45,46,48,50,52,53,54,56,58,60,61,62,64,66,68,69,70,72,74,76,77,78,80,82,84,85,86,88,90,92,93,94,95,96,97,98,99,100,101,102,103,104],\"spec\":\"lexlean/core-module/1\"}"
  ))
