open Lean Compiler LCNF

namespace LexLeanExtract

meta def escape (text : String) : String :=
  text.foldl (fun out c =>
    if c == '"' then out ++ "\\\""
    else if c == '\\' then out ++ "\\\\"
    else if c.toNat < 0x20 then out ++ "\\u" ++ String.ofList ((Nat.toDigits 16 (c.toNat + 0x10000)).drop 1)
    else out.push c) ""

meta def str (text : String) : String := "\"" ++ escape text ++ "\""

meta def name (n : Name) : String := str n.toString

meta def arr (items : List String) : String := "[" ++ String.intercalate "," items ++ "]"

meta def obj (fields : List (String × String)) : String :=
  "{" ++ String.intercalate "," (fields.map fun (key, value) => str key ++ ":" ++ value) ++ "}"

meta def bool (value : Bool) : String := if value then "true" else "false"

meta def fvar (id : FVarId) : String := name id.name

meta def sorted (names : NameSet) : List Name :=
  (names.toList.toArray.qsort Name.lt).toList

meta partial def type (e : Expr) : String :=
  match e with
  | .const n us =>
    if n == ``lcErased then obj [("kind", str "erased")]
    else if n == ``lcAny then obj [("kind", str "any")]
    else obj [("kind", str "const"), ("name", name n), ("levels", arr (us.map (fun u => str (toString u))))]
  | .app .. =>
    obj [("kind", str "app"), ("head", type e.getAppFn), ("arguments", arr (e.getAppArgs.toList.map type))]
  | .forallE _ domain body _ =>
    obj [("kind", str "arrow"), ("domain", type domain), ("codomain", type body)]
  | .fvar id => obj [("kind", str "fvar"), ("id", fvar id)]
  | .bvar index => obj [("kind", str "bvar"), ("index", toString index)]
  | .sort u => obj [("kind", str "sort"), ("level", str (toString u))]
  | .mvar _ => obj [("kind", str "unsupported"), ("expression", str "metavariable")]
  | .lam .. => obj [("kind", str "unsupported"), ("expression", str "lambda")]
  | .letE .. => obj [("kind", str "unsupported"), ("expression", str "let")]
  | .lit _ => obj [("kind", str "unsupported"), ("expression", str "literal")]
  | .mdata _ body => type body
  | .proj .. => obj [("kind", str "unsupported"), ("expression", str "projection")]

meta def arg (a : Arg .pure) : String :=
  match a with
  | .erased => obj [("kind", str "erased")]
  | .fvar id => obj [("kind", str "fvar"), ("id", fvar id)]
  | .type e => obj [("kind", str "type"), ("type", type e)]

meta def literal (value : LitValue) : String :=
  match value with
  | .nat v => obj [("kind", str "nat"), ("value", str (toString v))]
  | .str v => obj [("kind", str "string"), ("value", str v)]
  | .uint8 v => obj [("kind", str "uint8"), ("value", str (toString v.toNat))]
  | .uint16 v => obj [("kind", str "uint16"), ("value", str (toString v.toNat))]
  | .uint32 v => obj [("kind", str "uint32"), ("value", str (toString v.toNat))]
  | .uint64 v => obj [("kind", str "uint64"), ("value", str (toString v.toNat))]
  | .usize v => obj [("kind", str "usize"), ("value", str (toString v.toNat))]

meta def letValue (v : LetValue .pure) : String :=
  match v with
  | .lit value => obj [("kind", str "literal"), ("literal", literal value)]
  | .erased => obj [("kind", str "erased")]
  | .proj typeName index struct => obj [("kind", str "projection"), ("type_name", name typeName), ("index", toString index), ("value", fvar struct)]
  | .const declName us args => obj [("kind", str "const"), ("name", name declName), ("levels", arr (us.map (fun u => str (toString u)))), ("arguments", arr (args.toList.map arg))]
  | .fvar id args => obj [("kind", str "apply"), ("function", fvar id), ("arguments", arr (args.toList.map arg))]

meta def param (p : Param .pure) : String :=
  obj [("id", fvar p.fvarId), ("type", type p.type), ("borrow", bool p.borrow)]

mutual
meta partial def code (c : Code .pure) : String :=
  match c with
  | .let decl k => obj [("kind", str "let"), ("id", fvar decl.fvarId), ("type", type decl.type), ("value", letValue decl.value), ("body", code k)]
  | .fun decl k => obj [("kind", str "fun"), ("declaration", funDecl decl), ("body", code k)]
  | .jp decl k => obj [("kind", str "join"), ("declaration", funDecl decl), ("body", code k)]
  | .jmp id args => obj [("kind", str "jump"), ("target", fvar id), ("arguments", arr (args.toList.map arg))]
  | .cases cs => obj [("kind", str "cases"), ("type_name", name cs.typeName), ("result_type", type cs.resultType), ("discriminant", fvar cs.discr), ("alternatives", arr (cs.alts.toList.map alt))]
  | .return id => obj [("kind", str "return"), ("id", fvar id)]
  | .unreach t => obj [("kind", str "unreachable"), ("type", type t)]

meta partial def funDecl (d : FunDecl .pure) : String :=
  obj [("id", fvar d.fvarId), ("parameters", arr (d.params.toList.map param)), ("type", type d.type), ("value", code d.value)]

meta partial def alt (a : Alt .pure) : String :=
  match a with
  | .alt ctorName params k => obj [("kind", str "constructor"), ("constructor", name ctorName), ("parameters", arr (params.toList.map param)), ("code", code k)]
  | .default k => obj [("kind", str "default"), ("code", code k)]
end

meta def kind (info : ConstantInfo) : String :=
  match info with
  | .defnInfo value =>
    match value.safety with
    | .safe => "definition"
    | .unsafe => "unsafe-definition"
    | .partial => "partial-definition"
  | .opaqueInfo _ => "opaque"
  | .thmInfo _ => "theorem"
  | .axiomInfo _ => "axiom"
  | .inductInfo _ => "inductive"
  | .ctorInfo _ => "constructor"
  | .recInfo _ => "recursor"
  | .quotInfo _ => "quotient"

meta partial def typeConstants (e : Expr) (out : NameSet) : NameSet :=
  match e with
  | .const n _ => if n == ``lcErased || n == ``lcAny then out else out.insert n
  | .app f a => typeConstants a (typeConstants f out)
  | .forallE _ d b _ => typeConstants b (typeConstants d out)
  | .bvar _ => out
  | .fvar _ => out
  | .mvar _ => out
  | .sort _ => out
  | .lit _ => out
  | .lam _ d b _ => typeConstants b (typeConstants d out)
  | .letE _ t v b _ => typeConstants b (typeConstants v (typeConstants t out))
  | .mdata _ b => typeConstants b out
  | .proj typeName _ b => typeConstants b (out.insert typeName)

meta def argConstants (a : Arg .pure) (out : NameSet) : NameSet :=
  match a with
  | .type e => typeConstants e out
  | .erased => out
  | .fvar _ => out

meta def paramConstants (ps : Array (Param .pure)) (out : NameSet) : NameSet :=
  ps.foldl (fun acc p => typeConstants p.type acc) out

meta def letValueConstants (v : LetValue .pure) (out : NameSet) : NameSet :=
  match v with
  | .const n _ args => args.foldl (fun acc a => argConstants a acc) (out.insert n)
  | .fvar _ args => args.foldl (fun acc a => argConstants a acc) out
  | .proj typeName _ _ => out.insert typeName
  | .lit _ => out
  | .erased => out

mutual
meta partial def codeConstants (c : Code .pure) (out : NameSet) : NameSet :=
  match c with
  | .let decl k => codeConstants k (letValueConstants decl.value (typeConstants decl.type out))
  | .fun decl k => codeConstants k (funConstants decl out)
  | .jp decl k => codeConstants k (funConstants decl out)
  | .jmp _ args => args.foldl (fun acc a => argConstants a acc) out
  | .cases cs => cs.alts.foldl (fun acc a => altConstants a acc) (typeConstants cs.resultType (out.insert cs.typeName))
  | .return _ => out
  | .unreach t => typeConstants t out

meta partial def funConstants (d : FunDecl .pure) (out : NameSet) : NameSet :=
  codeConstants d.value (typeConstants d.type (paramConstants d.params out))

meta partial def altConstants (a : Alt .pure) (out : NameSet) : NameSet :=
  match a with
  | .alt ctorName ps k => codeConstants k (paramConstants ps (out.insert ctorName))
  | .default k => codeConstants k out
end


meta partial def sameSignature : Expr → Expr → Bool
  | .forallE _ d b i, .forallE _ d' b' i' => i == i' && sameSignature d d' && sameSignature b b'
  | .lam _ d b i, .lam _ d' b' i' => i == i' && sameSignature d d' && sameSignature b b'
  | .app f a, .app f' a' => sameSignature f f' && sameSignature a a'
  | .mdata _ e, e' => sameSignature e e'
  | e, .mdata _ e' => sameSignature e e'
  | .const n ls, .const n' ls' => n == n' && ls == ls'
  | .sort u, .sort u' => u == u'
  | .bvar i, .bvar i' => i == i'
  | .lit l, .lit l' => l == l'
  | .fvar i, .fvar i' => i == i'
  | .mvar i, .mvar i' => i == i'
  | .letE _ t v b _, .letE _ t' v' b' _ => sameSignature t t' && sameSignature v v' && sameSignature b b'
  | .proj n i e, .proj n' i' e' => n == n' && i == i' && sameSignature e e'
  | _, _ => false

syntax (name := signatureCommand) "lexlean_signature " ident " : " term : command

@[command_elab signatureCommand]
public meta def elabSignature : Lean.Elab.Command.CommandElab := fun stx =>
  Lean.Elab.Command.liftTermElabM do
    let declName := stx[1].getId
    let info ← getConstInfo declName
    let registered ← instantiateMVars (← Lean.Elab.Term.elabType stx[3])
    unless sameSignature (← instantiateMVars info.type) registered do
      throwError "lexlean-extract-drift: the signature of `{declName}` is {info.type}"

meta def adapterConstants : Lean.Elab.Command.CommandElabM (List (Name × String)) := do
  let env ← getEnv
  let mut own : NameSet := {}
  for (n, _) in env.constants.map₂.toList do
    own := own.insert n
  let mut used : NameSet := {}
  for (n, info) in env.constants.map₂.toList do
    if (`LexLeanExtract).isPrefixOf n.eraseMacroScopes || n.toString.contains "LexLeanExtract" then
      for c in info.type.getUsedConstants do
        used := used.insert c
      for c in (info.value? (allowOpaque := true)).map Expr.getUsedConstants |>.getD #[] do
        used := used.insert c
  let mut out : List (Name × String) := []
  for c in sorted used do
    unless own.contains c do
      let some info := env.find? c
        | throwError "lexlean-extract-drift: the adapter uses the unknown constant `{c}`"
      let structural := (env.isProjectionFn c) || isCasesOnRecursor env c || isAuxRecursor env c
        || isNoConfusion env c || c.isInternal || (Lean.Meta.isInstanceCore env c)
        || (Lean.Meta.isMatcherCore env c) || c.getString! == "ctorIdx"
      let former := info.type.getForallBody.isSort
      let cls := match info with
        | .inductInfo _ => "type"
        | .defnInfo _ => if former then "type" else if (`Lean).isPrefixOf c && !structural then "call" else "plumbing"
        | .opaqueInfo _ => if (`Lean).isPrefixOf c && !structural then "call" else "plumbing"
        | .axiomInfo _ => if (`Lean).isPrefixOf c && !structural && !former then "call" else "plumbing"
        | .thmInfo _ => "plumbing"
        | .quotInfo _ => "plumbing"
        | .ctorInfo _ => "plumbing"
        | .recInfo _ => "plumbing"
      out := (c, cls) :: out
  return out.reverse

meta def moduleOf (env : Environment) (n : Name) : Name :=
  match env.getModuleIdxFor? n with
  | some index => env.header.moduleNames[index.toNat]!
  | none => Name.anonymous

meta def facts (env : Environment) (n : Name) (info : ConstantInfo) : CoreM (List (String × String)) := do
  let generates ← shouldGenerateCode n
  return [("name", name n), ("kind", str (kind info)), ("module", name (moduleOf env n)),
    ("computable", bool (!isNoncomputable env n)), ("generates_code", bool generates),
    ("internal", bool n.isInternal)]

meta def run (roots modules : Array Name) : CoreM String := do
  let env ← getEnv
  let inProject := fun (n : Name) => modules.contains (moduleOf env n)
  for root in roots do
    unless inProject root && (env.find? root).isSome do
      throwError "lexlean-extract: unknown root `{root}`"
  let mut translated : Array (Decl .pure) := #[]
  let mut rows : Std.HashMap Name (List (String × String)) := {}
  let mut referenced : NameSet := {}
  let mut codeQueue : Array Name := roots
  let mut kernelQueue : Array Name := #[]
  let mut seen : NameSet := {}
  let mut cursor := 0
  let mut kernelCursor := 0
  while cursor < codeQueue.size || kernelCursor < kernelQueue.size do
    let (n, fromCode) :=
      if cursor < codeQueue.size then (codeQueue[cursor]!, true) else (kernelQueue[kernelCursor]!, false)
    if fromCode then cursor := cursor + 1 else kernelCursor := kernelCursor + 1
    let some info := env.find? n
      | throwError "lexlean-extract: unresolved constant `{n}`"
    unless rows.contains n do
      let mut row ← facts env n info
      let kernel := sorted (((info.value? (allowOpaque := true)).map Expr.getUsedConstants |>.getD #[]).foldl NameSet.insert {})
      row := row ++ [("kernel_uses", arr (kernel.map name))]
      rows := rows.insert n row
      for used in kernel do
        if inProject used then
          kernelQueue := kernelQueue.push used
    if fromCode && !seen.contains n then
      seen := seen.insert n
      if kind info == "definition" && !isNoncomputable env n && (← shouldGenerateCode n) then
        let decl ← CompilerM.run (toDecl n)
        translated := translated.push decl
        let uses := match decl.value with
          | .code c => codeConstants c (typeConstants decl.type (paramConstants decl.params {}))
          | .extern _ => typeConstants decl.type (paramConstants decl.params {})
        for r in sorted uses do
          if inProject r then codeQueue := codeQueue.push r else referenced := referenced.insert r
  let mut declarations : Std.HashMap Name String := {}
  for decl in translated do
    let signature := typeConstants decl.type (paramConstants decl.params {})
    let (value, uses) := match decl.value with
      | .code c => (obj [("kind", str "code"), ("code", code c)], codeConstants c signature)
      | .extern _ => (obj [("kind", str "extern")], signature)
    declarations := declarations.insert decl.name (obj [
      ("safe", bool decl.safe),
      ("level_parameters", arr (decl.levelParams.map name)),
      ("type", type decl.type), ("parameters", arr (decl.params.toList.map param)),
      ("value", value), ("uses", arr ((sorted uses).map name))])
  let mut constants : Array String := #[]
  for n in sorted (rows.fold (fun acc key _ => acc.insert key) {}) do
    let row := (rows.get? n).getD []
    let declaration := (declarations.get? n).getD "null"
    let mut inductiveRow := "null"
    if let some (.inductInfo value) := env.find? n then
      let mut ctors : Array String := #[]
      for c in value.ctors do
        let some (.ctorInfo ctor) := env.find? c
          | throwError "lexlean-extract: `{c}` is not a constructor"
        let ctorType ← Meta.MetaM.run' (toLCNFType ctor.type)
        ctors := ctors.push (obj [("name", name c), ("parameters", toString ctor.numParams), ("fields", toString ctor.numFields), ("type", type ctorType)])
      inductiveRow := obj [("parameters", toString value.numParams), ("indices", toString value.numIndices), ("recursive", bool value.isRec), ("constructors", arr ctors.toList)]
    constants := constants.push (obj (row ++ [("declaration", declaration), ("inductive", inductiveRow)]))
  let mut externals : Array String := #[]
  for r in sorted referenced do
    let some info := env.find? r
      | throwError "lexlean-extract: unresolved constant `{r}`"
    let mut row ← facts env r info
    if let .ctorInfo ctor := info then
      row := row ++ [("inductive_type", name ctor.induct)]
    externals := externals.push (obj row)
  return obj [
    ("spec", str "lexlean/lcnf-extraction/2"),
    ("lean", obj [("version", str Lean.versionString), ("githash", str Lean.githash)]),
    ("roots", arr (roots.toList.map name)),
    ("constants", arr constants.toList),
    ("externals", arr externals.toList)]

meta def checkAuthority (calls : Array Name) (types : Array (Name × Array String))
    (plumbing : Array Name) : Lean.Elab.Command.CommandElabM Unit := do
  let env ← getEnv
  let mut problems : Array String := #[]
  let mut seen : NameSet := {}
  let typeNames : NameSet := types.foldl (fun acc (n, _) => acc.insert n) {}
  for (c, cls) in ← adapterConstants do
    seen := seen.insert c
    let registered :=
      if calls.contains c then "call"
      else if typeNames.contains c then "type"
      else if plumbing.contains c then "plumbing"
      else "unregistered"
    let expected := if cls == "type" && !(`Lean).isPrefixOf c then "plumbing" else cls
    unless registered == expected do
      problems := problems.push s!"`{c}` is used as {expected} but registered as {registered}"
  for c in calls ++ typeNames.toList.toArray ++ plumbing do
    unless seen.contains c do
      problems := problems.push s!"`{c}` is registered but unused"
  for (n, ctors) in types do
    match env.find? n with
    | some (.inductInfo value) =>
      let actual := value.ctors.toArray.map Name.getString!
      unless actual == ctors do
        problems := problems.push s!"`{n}` has constructors {actual}, registered {ctors}"
    | some _ =>
      unless ctors.isEmpty do
        problems := problems.push s!"`{n}` is not an inductive type, registered with constructors {ctors}"
    | none => problems := problems.push s!"`{n}` is not a constant"
  unless problems.isEmpty do
    throwError "lexlean-extract-drift: the adapter and its registry differ: {String.intercalate "; " problems.toList}"

meta def listAdapterConstants : Lean.Elab.Command.CommandElabM Unit := do
  for (c, cls) in ← adapterConstants do
    IO.println s!"{cls} {c}"

meta def main (roots modules : Array Name) : Lean.Elab.Command.CommandElabM Unit :=
  Lean.Elab.Command.liftCoreM do
    IO.println (← run roots modules)

end LexLeanExtract
