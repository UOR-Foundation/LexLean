module
public meta import Lean
import all Production.Kernel
import all Production.Main
set_option linter.unusedVariables false
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
  | .mdata _ b =>
    match annotation? `borrowed e with
    | some _ => obj [("kind", str "borrowed"), ("type", type b)]
    | none => obj [("kind", str "unsupported"), ("expression", str "metadata")]
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

meta def originalKind (env : Environment) (n : Name) : String :=
  match getOriginalConstKind? env n with
  | some .defn => "definition"
  | some .thm => "theorem"
  | some .axiom => "axiom"
  | some .opaque => "opaque"
  | some .quot => "quotient"
  | some .induct => "inductive"
  | some .ctor => "constructor"
  | some .recursor => "recursor"
  | none => "unknown"

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
    ("original_kind", str (originalKind env n)), ("internal", bool n.isInternal)]

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
    row := row ++ [("compiled", bool (IR.findEnvDecl env r).isSome)]
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
universe u_1 u_2
lexlean_signature Lean.annotation? : Lean.Name → Lean.Expr → Option Lean.Expr
lexlean_signature Lean.getConstInfo : {m : Type → Type} → [Monad m] → [Lean.MonadEnv m] → [Lean.MonadError m] → Lean.Name → m Lean.ConstantInfo
lexlean_signature Lean.getOriginalConstKind? : Lean.Environment → Lean.Name → Option Lean.ConstantKind
lexlean_signature Lean.githash : String
lexlean_signature Lean.instantiateMVars : {m : Type → Type} → [Monad m] → [Lean.MonadMCtx m] → Lean.Expr → m Lean.Expr
lexlean_signature Lean.isAuxRecursor : Lean.Environment → Lean.Name → Bool
lexlean_signature Lean.isCasesOnRecursor : Lean.Environment → Lean.Name → Bool
lexlean_signature Lean.isNoConfusion : Lean.Environment → Lean.Name → Bool
lexlean_signature Lean.isNoncomputable : Lean.Environment → Lean.Name → optParam Lean.EnvExtension.AsyncMode Lean.noncomputableExt.toEnvExtension.asyncMode → Bool
lexlean_signature Lean.noncomputableExt : Lean.TagDeclarationExtension
lexlean_signature Lean.throwError : {m : Type → Type} → {α : Type} → [Monad m] → [Lean.MonadError m] → Lean.MessageData → m α
lexlean_signature Lean.versionString : String
lexlean_signature Lean.ConstantInfo.type : Lean.ConstantInfo → Lean.Expr
lexlean_signature Lean.ConstantInfo.value? : Lean.ConstantInfo → optParam Bool Bool.false → Option Lean.Expr
lexlean_signature Lean.Environment.constants : Lean.Environment → Lean.ConstMap
lexlean_signature Lean.Environment.find? : Lean.Environment → Lean.Name → optParam Bool Bool.false → Option Lean.ConstantInfo
lexlean_signature Lean.Environment.getModuleIdxFor? : Lean.Environment → Lean.Name → Option Lean.ModuleIdx
lexlean_signature Lean.Environment.header : Lean.Environment → Lean.EnvironmentHeader
lexlean_signature Lean.Environment.isProjectionFn : Lean.Environment → Lean.Name → Bool
lexlean_signature Lean.EnvironmentHeader.moduleNames : Lean.EnvironmentHeader → Array Lean.Name
lexlean_signature Lean.Expr.getAppArgs : Lean.Expr → Array Lean.Expr
lexlean_signature Lean.Expr.getAppFn : Lean.Expr → Lean.Expr
lexlean_signature Lean.Expr.getForallBody : Lean.Expr → Lean.Expr
lexlean_signature Lean.Expr.getUsedConstants : Lean.Expr → Array Lean.Name
lexlean_signature Lean.Expr.isSort : Lean.Expr → Bool
lexlean_signature Lean.IR.findEnvDecl : Lean.Environment → Lean.Name → Option Lean.IR.Decl
lexlean_signature Lean.Meta.isInstanceCore : Lean.Environment → Lean.Name → Bool
lexlean_signature Lean.Meta.isMatcherCore : Lean.Environment → Lean.Name → Bool
lexlean_signature Lean.ModuleIdx.toNat : Lean.ModuleIdx → Nat
lexlean_signature Lean.Name.eraseMacroScopes : Lean.Name → Lean.Name
lexlean_signature Lean.Name.getString! : Lean.Name → String
lexlean_signature Lean.Name.isInternal : Lean.Name → Bool
lexlean_signature Lean.Name.isPrefixOf : Lean.Name → Lean.Name → Bool
lexlean_signature Lean.Name.lt : Lean.Name → Lean.Name → Bool
lexlean_signature Lean.Name.mkStr1 : String → Lean.Name
lexlean_signature Lean.Name.mkStr2 : String → String → Lean.Name
lexlean_signature Lean.Name.quickCmp : Lean.Name → Lean.Name → Ordering
lexlean_signature Lean.Name.toString : Lean.Name → optParam Bool Bool.true → String
lexlean_signature Lean.NameSet.contains : Lean.NameSet → Lean.Name → Bool
lexlean_signature Lean.NameSet.insert : Lean.NameSet → Lean.Name → Lean.NameSet
lexlean_signature Lean.PersistentArray.branching : USize
lexlean_signature Lean.PersistentArray.initShift : USize
lexlean_signature Lean.PersistentHashMap.mkEmptyEntriesArray : {α : Type u_1} → {β : Type u_2} → Array (Lean.PersistentHashMap.Entry α β (Lean.PersistentHashMap.Node α β))
lexlean_signature Lean.PersistentHashMap.toList : {α : Type u_1} → {β : Type u_2} → {x : BEq α} → {x_1 : Hashable α} → Lean.PersistentHashMap α β → List (α × β)
lexlean_signature Lean.Syntax.getId : Lean.Syntax → Lean.Name
lexlean_signature Lean.Compiler.LCNF.shouldGenerateCode : Lean.Name → Lean.Core.CoreM Bool
lexlean_signature Lean.Compiler.LCNF.toDecl : Lean.Name → Lean.Compiler.LCNF.CompilerM (Lean.Compiler.LCNF.Decl Lean.Compiler.LCNF.Purity.pure)
lexlean_signature Lean.Compiler.LCNF.toLCNFType : Lean.Expr → Lean.Meta.MetaM Lean.Expr
lexlean_signature Lean.Elab.Command.liftCoreM : {α : Type} → Lean.Core.CoreM α → Lean.Elab.Command.CommandElabM α
lexlean_signature Lean.Elab.Command.liftTermElabM : {α : Type} → Lean.Elab.Term.TermElabM α → Lean.Elab.Command.CommandElabM α
lexlean_signature Lean.Elab.Term.elabType : Lean.Syntax → Lean.Elab.Term.TermElabM Lean.Expr
lexlean_signature Lean.Meta.MetaM.run' : {α : Type} → Lean.Meta.MetaM α → optParam Lean.Meta.Context { } → optParam Lean.Meta.State { } → Lean.Core.CoreM α
lexlean_signature Lean.Compiler.LCNF.Cases.alts : {pu : Lean.Compiler.LCNF.Purity} → Lean.Compiler.LCNF.Cases pu → Array (Lean.Compiler.LCNF.Alt pu)
lexlean_signature Lean.Compiler.LCNF.Cases.discr : {pu : Lean.Compiler.LCNF.Purity} → Lean.Compiler.LCNF.Cases pu → Lean.FVarId
lexlean_signature Lean.Compiler.LCNF.Cases.resultType : {pu : Lean.Compiler.LCNF.Purity} → Lean.Compiler.LCNF.Cases pu → Lean.Expr
lexlean_signature Lean.Compiler.LCNF.Cases.typeName : {pu : Lean.Compiler.LCNF.Purity} → Lean.Compiler.LCNF.Cases pu → Lean.Name
lexlean_signature Lean.Compiler.LCNF.CompilerM.run : {α : Type} → Lean.Compiler.LCNF.CompilerM α → optParam Lean.Compiler.LCNF.CompilerM.State { } → optParam Lean.Compiler.LCNF.Phase Lean.Compiler.LCNF.Phase.base → Lean.Core.CoreM α
lexlean_signature Lean.Compiler.LCNF.FunDecl.fvarId : {pu : Lean.Compiler.LCNF.Purity} → Lean.Compiler.LCNF.FunDecl pu → Lean.FVarId
lexlean_signature Lean.Compiler.LCNF.FunDecl.params : {pu : Lean.Compiler.LCNF.Purity} → Lean.Compiler.LCNF.FunDecl pu → Array (Lean.Compiler.LCNF.Param pu)
lexlean_signature Lean.Compiler.LCNF.FunDecl.type : {pu : Lean.Compiler.LCNF.Purity} → Lean.Compiler.LCNF.FunDecl pu → Lean.Expr
lexlean_signature Lean.Compiler.LCNF.FunDecl.value : {pu : Lean.Compiler.LCNF.Purity} → Lean.Compiler.LCNF.FunDecl pu → Lean.Compiler.LCNF.Code pu
#eval LexLeanExtract.checkAuthority #["Lean.annotation?".toName, "Lean.getConstInfo".toName, "Lean.getOriginalConstKind?".toName, "Lean.githash".toName, "Lean.instantiateMVars".toName, "Lean.isAuxRecursor".toName, "Lean.isCasesOnRecursor".toName, "Lean.isNoConfusion".toName, "Lean.isNoncomputable".toName, "Lean.noncomputableExt".toName, "Lean.throwError".toName, "Lean.versionString".toName, "Lean.ConstantInfo.type".toName, "Lean.ConstantInfo.value?".toName, "Lean.Environment.constants".toName, "Lean.Environment.find?".toName, "Lean.Environment.getModuleIdxFor?".toName, "Lean.Environment.header".toName, "Lean.Environment.isProjectionFn".toName, "Lean.EnvironmentHeader.moduleNames".toName, "Lean.Expr.getAppArgs".toName, "Lean.Expr.getAppFn".toName, "Lean.Expr.getForallBody".toName, "Lean.Expr.getUsedConstants".toName, "Lean.Expr.isSort".toName, "Lean.IR.findEnvDecl".toName, "Lean.Meta.isInstanceCore".toName, "Lean.Meta.isMatcherCore".toName, "Lean.ModuleIdx.toNat".toName, "Lean.Name.eraseMacroScopes".toName, "Lean.Name.getString!".toName, "Lean.Name.isInternal".toName, "Lean.Name.isPrefixOf".toName, "Lean.Name.lt".toName, "Lean.Name.mkStr1".toName, "Lean.Name.mkStr2".toName, "Lean.Name.quickCmp".toName, "Lean.Name.toString".toName, "Lean.NameSet.contains".toName, "Lean.NameSet.insert".toName, "Lean.PersistentArray.branching".toName, "Lean.PersistentArray.initShift".toName, "Lean.PersistentHashMap.mkEmptyEntriesArray".toName, "Lean.PersistentHashMap.toList".toName, "Lean.Syntax.getId".toName, "Lean.Compiler.LCNF.shouldGenerateCode".toName, "Lean.Compiler.LCNF.toDecl".toName, "Lean.Compiler.LCNF.toLCNFType".toName, "Lean.Elab.Command.liftCoreM".toName, "Lean.Elab.Command.liftTermElabM".toName, "Lean.Elab.Term.elabType".toName, "Lean.Meta.MetaM.run'".toName, "Lean.Compiler.LCNF.Cases.alts".toName, "Lean.Compiler.LCNF.Cases.discr".toName, "Lean.Compiler.LCNF.Cases.resultType".toName, "Lean.Compiler.LCNF.Cases.typeName".toName, "Lean.Compiler.LCNF.CompilerM.run".toName, "Lean.Compiler.LCNF.FunDecl.fvarId".toName, "Lean.Compiler.LCNF.FunDecl.params".toName, "Lean.Compiler.LCNF.FunDecl.type".toName, "Lean.Compiler.LCNF.FunDecl.value".toName] #[("Lean.AxiomVal".toName, #["mk"]), ("Lean.BinderInfo".toName, #["default", "implicit", "strictImplicit", "instImplicit"]), ("Lean.ConstantInfo".toName, #["axiomInfo", "defnInfo", "thmInfo", "opaqueInfo", "quotInfo", "inductInfo", "ctorInfo", "recInfo"]), ("Lean.ConstantKind".toName, #["defn", "thm", "axiom", "opaque", "quot", "induct", "ctor", "recursor"]), ("Lean.ConstructorVal".toName, #["mk"]), ("Lean.DefinitionSafety".toName, #["unsafe", "safe", "partial"]), ("Lean.DefinitionVal".toName, #["mk"]), ("Lean.DelayedMetavarAssignment".toName, #["mk"]), ("Lean.Environment".toName, #["mk"]), ("Lean.Exception".toName, #["error", "internal"]), ("Lean.Expr".toName, #["bvar", "fvar", "mvar", "sort", "const", "app", "lam", "forallE", "letE", "lit", "mdata", "proj"]), ("Lean.ExternAttrData".toName, #["mk"]), ("Lean.FVarId".toName, #["mk"]), ("Lean.FVarIdMap".toName, #[]), ("Lean.FVarIdSet".toName, #[]), ("Lean.InductiveVal".toName, #["mk"]), ("Lean.LMVarId".toName, #[]), ("Lean.Level".toName, #["zero", "succ", "max", "imax", "param", "mvar"]), ("Lean.LevelMetavarDecl".toName, #["mk"]), ("Lean.Literal".toName, #["natVal", "strVal"]), ("Lean.LocalDecl".toName, #["cdecl", "ldecl"]), ("Lean.LocalInstance".toName, #["mk"]), ("Lean.Loop".toName, #["mk"]), ("Lean.MData".toName, #[]), ("Lean.MVarId".toName, #["mk"]), ("Lean.MessageData".toName, #["ofFormatWithInfos", "ofGoal", "ofWidget", "withContext", "withNamingContext", "nest", "group", "compose", "tagged", "trace", "ofLazy", "ofOriginatingSyntax"]), ("Lean.MetavarDecl".toName, #["mk"]), ("Lean.ModuleIdx".toName, #[]), ("Lean.Name".toName, #["anonymous", "str", "num"]), ("Lean.NameSet".toName, #[]), ("Lean.OpaqueVal".toName, #["mk"]), ("Lean.ParserDescr".toName, #["const", "unary", "binary", "node", "trailingNode", "symbol", "nonReservedSymbol", "cat", "parser", "nodeWithAntiquot", "sepBy", "sepBy1", "unicodeSymbol"]), ("Lean.PersistentArrayNode".toName, #["node", "leaf"]), ("Lean.PersistentEnvExtensionState".toName, #["mk"]), ("Lean.QuotVal".toName, #["mk"]), ("Lean.RecursorVal".toName, #["mk"]), ("Lean.Syntax".toName, #["missing", "node", "atom", "ident"]), ("Lean.TheoremVal".toName, #["mk"]), ("Lean.Core.Context".toName, #["mk"]), ("Lean.Core.CoreM".toName, #[]), ("Lean.Core.State".toName, #["mk"]), ("Lean.IR.Decl".toName, #["fdecl", "extern"]), ("Lean.Meta.AbstractMVarsResult".toName, #["mk"]), ("Lean.Meta.Config".toName, #["mk"]), ("Lean.Meta.ConfigWithKey".toName, #["mk"]), ("Lean.Meta.Context".toName, #["mk"]), ("Lean.Meta.DefEqCacheKey".toName, #["mk"]), ("Lean.Meta.DefEqContext".toName, #["mk"]), ("Lean.Meta.ExprConfigCacheKey".toName, #["mk"]), ("Lean.Meta.FunInfo".toName, #["mk"]), ("Lean.Meta.InfoCacheKey".toName, #["mk"]), ("Lean.Meta.MetaM".toName, #[]), ("Lean.Meta.PostponedEntry".toName, #["mk"]), ("Lean.Meta.State".toName, #["mk"]), ("Lean.Meta.SynthInstanceCacheKey".toName, #["mk"]), ("Lean.Compiler.LCNF.Alt".toName, #["alt", "ctorAlt", "default"]), ("Lean.Compiler.LCNF.Arg".toName, #["erased", "fvar", "type"]), ("Lean.Compiler.LCNF.Cases".toName, #["mk"]), ("Lean.Compiler.LCNF.Code".toName, #["let", "fun", "jp", "jmp", "cases", "return", "unreach", "oset", "uset", "sset", "setTag", "inc", "dec", "del"]), ("Lean.Compiler.LCNF.CtorInfo".toName, #["mk"]), ("Lean.Compiler.LCNF.Decl".toName, #["mk"]), ("Lean.Compiler.LCNF.DeclValue".toName, #["code", "extern"]), ("Lean.Compiler.LCNF.FunDecl".toName, #["mk"]), ("Lean.Compiler.LCNF.LetDecl".toName, #["mk"]), ("Lean.Compiler.LCNF.LetValue".toName, #["lit", "erased", "proj", "const", "fvar", "ctor", "oproj", "uproj", "sproj", "fap", "pap", "reset", "reuse", "box", "unbox", "isShared"]), ("Lean.Compiler.LCNF.LitValue".toName, #["nat", "str", "uint8", "uint16", "uint32", "uint64", "usize"]), ("Lean.Compiler.LCNF.Param".toName, #["mk"]), ("Lean.Compiler.LCNF.Purity".toName, #["pure", "impure"]), ("Lean.Elab.Command.CommandElab".toName, #[]), ("Lean.Elab.Command.CommandElabM".toName, #[]), ("Lean.Elab.Command.Context".toName, #["mk"]), ("Lean.Elab.Command.State".toName, #["mk"]), ("Lean.Elab.Term.Context".toName, #["mk"]), ("Lean.Elab.Term.State".toName, #["mk"]), ("Lean.Elab.Term.TermElabM".toName, #[])] #["Applicative.toPure".toName, "Array".toName, "Array.casesOn".toName, "Array.contains".toName, "Array.foldl".toName, "Array.instAppend".toName, "Array.instBEq".toName, "Array.instForIn'InferInstanceMembershipOfMonad".toName, "Array.instGetElem?NatLtSize".toName, "Array.instMembership".toName, "Array.isEmpty".toName, "Array.map".toName, "Array.mkEmpty".toName, "Array.push".toName, "Array.qsort".toName, "Array.size".toName, "Array.toList".toName, "BEq.beq".toName, "Bind.bind".toName, "Bool".toName, "Bool.and".toName, "Bool.casesOn".toName, "Bool.false".toName, "Bool.not".toName, "Bool.or".toName, "Bool.true".toName, "Char".toName, "Char.ofNat".toName, "Char.toNat".toName, "Decidable.decide".toName, "EIO".toName, "EmptyCollection.emptyCollection".toName, "Eq".toName, "Eq.casesOn".toName, "Eq.ndrec".toName, "Eq.refl".toName, "Eq.symm".toName, "False".toName, "False.elim".toName, "ForIn.forIn".toName, "ForInStep".toName, "ForInStep.done".toName, "ForInStep.yield".toName, "GetElem.getElem".toName, "GetElem?.getElem!".toName, "HAdd.hAdd".toName, "HAppend.hAppend".toName, "HEq".toName, "HEq.refl".toName, "HSub.hSub".toName, "IO".toName, "IO.Error".toName, "IO.RealWorld".toName, "IO.println".toName, "Inhabited".toName, "Inhabited.default".toName, "Inhabited.mk".toName, "LT.lt".toName, "Lean.Compiler.LCNF.Alt.alt".toName, "Lean.Compiler.LCNF.Alt.casesOn".toName, "Lean.Compiler.LCNF.Alt.ctorAlt".toName, "Lean.Compiler.LCNF.Alt.default".toName, "Lean.Compiler.LCNF.Arg.casesOn".toName, "Lean.Compiler.LCNF.Arg.erased".toName, "Lean.Compiler.LCNF.Arg.fvar".toName, "Lean.Compiler.LCNF.Arg.type".toName, "Lean.Compiler.LCNF.Code.cases".toName, "Lean.Compiler.LCNF.Code.casesOn".toName, "Lean.Compiler.LCNF.Code.dec".toName, "Lean.Compiler.LCNF.Code.del".toName, "Lean.Compiler.LCNF.Code.fun".toName, "Lean.Compiler.LCNF.Code.inc".toName, "Lean.Compiler.LCNF.Code.jmp".toName, "Lean.Compiler.LCNF.Code.jp".toName, "Lean.Compiler.LCNF.Code.let".toName, "Lean.Compiler.LCNF.Code.oset".toName, "Lean.Compiler.LCNF.Code.return".toName, "Lean.Compiler.LCNF.Code.setTag".toName, "Lean.Compiler.LCNF.Code.sset".toName, "Lean.Compiler.LCNF.Code.unreach".toName, "Lean.Compiler.LCNF.Code.uset".toName, "Lean.Compiler.LCNF.CompilerM.State.mk".toName, "Lean.Compiler.LCNF.CtorInfo.casesOn".toName, "Lean.Compiler.LCNF.Decl.toSignature".toName, "Lean.Compiler.LCNF.Decl.value".toName, "Lean.Compiler.LCNF.DeclValue.casesOn".toName, "Lean.Compiler.LCNF.DeclValue.code".toName, "Lean.Compiler.LCNF.DeclValue.extern".toName, "Lean.Compiler.LCNF.LCtx.mk".toName, "Lean.Compiler.LCNF.LetDecl.fvarId".toName, "Lean.Compiler.LCNF.LetDecl.type".toName, "Lean.Compiler.LCNF.LetDecl.value".toName, "Lean.Compiler.LCNF.LetValue.box".toName, "Lean.Compiler.LCNF.LetValue.casesOn".toName, "Lean.Compiler.LCNF.LetValue.const".toName, "Lean.Compiler.LCNF.LetValue.ctor".toName, "Lean.Compiler.LCNF.LetValue.erased".toName, "Lean.Compiler.LCNF.LetValue.fap".toName, "Lean.Compiler.LCNF.LetValue.fvar".toName, "Lean.Compiler.LCNF.LetValue.isShared".toName, "Lean.Compiler.LCNF.LetValue.lit".toName, "Lean.Compiler.LCNF.LetValue.oproj".toName, "Lean.Compiler.LCNF.LetValue.pap".toName, "Lean.Compiler.LCNF.LetValue.proj".toName, "Lean.Compiler.LCNF.LetValue.reset".toName, "Lean.Compiler.LCNF.LetValue.reuse".toName, "Lean.Compiler.LCNF.LetValue.sproj".toName, "Lean.Compiler.LCNF.LetValue.unbox".toName, "Lean.Compiler.LCNF.LetValue.uproj".toName, "Lean.Compiler.LCNF.LitValue.casesOn".toName, "Lean.Compiler.LCNF.LitValue.nat".toName, "Lean.Compiler.LCNF.LitValue.str".toName, "Lean.Compiler.LCNF.LitValue.uint16".toName, "Lean.Compiler.LCNF.LitValue.uint32".toName, "Lean.Compiler.LCNF.LitValue.uint64".toName, "Lean.Compiler.LCNF.LitValue.uint8".toName, "Lean.Compiler.LCNF.LitValue.usize".toName, "Lean.Compiler.LCNF.Param.borrow".toName, "Lean.Compiler.LCNF.Param.fvarId".toName, "Lean.Compiler.LCNF.Param.type".toName, "Lean.Compiler.LCNF.Phase.base".toName, "Lean.Compiler.LCNF.Purity.ctorIdx".toName, "Lean.Compiler.LCNF.Purity.impure".toName, "Lean.Compiler.LCNF.Purity.pure".toName, "Lean.Compiler.LCNF.Signature.levelParams".toName, "Lean.Compiler.LCNF.Signature.name".toName, "Lean.Compiler.LCNF.Signature.params".toName, "Lean.Compiler.LCNF.Signature.safe".toName, "Lean.Compiler.LCNF.Signature.type".toName, "Lean.ConstantInfo.axiomInfo".toName, "Lean.ConstantInfo.casesOn".toName, "Lean.ConstantInfo.ctorIdx".toName, "Lean.ConstantInfo.ctorInfo".toName, "Lean.ConstantInfo.defnInfo".toName, "Lean.ConstantInfo.inductInfo".toName, "Lean.ConstantInfo.opaqueInfo".toName, "Lean.ConstantInfo.quotInfo".toName, "Lean.ConstantInfo.rec".toName, "Lean.ConstantInfo.recInfo".toName, "Lean.ConstantInfo.thmInfo".toName, "Lean.ConstantKind.axiom".toName, "Lean.ConstantKind.casesOn".toName, "Lean.ConstantKind.ctor".toName, "Lean.ConstantKind.defn".toName, "Lean.ConstantKind.induct".toName, "Lean.ConstantKind.opaque".toName, "Lean.ConstantKind.quot".toName, "Lean.ConstantKind.recursor".toName, "Lean.ConstantKind.thm".toName, "Lean.ConstantVal.type".toName, "Lean.ConstructorVal.induct".toName, "Lean.ConstructorVal.numFields".toName, "Lean.ConstructorVal.numParams".toName, "Lean.ConstructorVal.toConstantVal".toName, "Lean.Core.instAddMessageContextCoreM".toName, "Lean.Core.instMonadCoreM".toName, "Lean.Core.instMonadEnvCoreM".toName, "Lean.Core.instMonadLiftIOCoreM".toName, "Lean.Core.instMonadRefCoreM".toName, "Lean.DefinitionSafety.casesOn".toName, "Lean.DefinitionSafety.partial".toName, "Lean.DefinitionSafety.safe".toName, "Lean.DefinitionSafety.unsafe".toName, "Lean.DefinitionVal.safety".toName, "Lean.Elab.Command.instAddErrorMessageContextCommandElabM".toName, "Lean.Elab.Command.instMonadCommandElabM".toName, "Lean.Elab.Command.instMonadEnvCommandElabM".toName, "Lean.Elab.Command.instMonadExceptOfExceptionCommandElabM".toName, "Lean.Elab.Command.instMonadLiftTIOCommandElabM".toName, "Lean.Elab.Command.instMonadRefCommandElabM".toName, "Lean.Elab.MonadMacroAdapter.toMonadQuotation".toName, "Lean.Elab.Term.instAddErrorMessageContextTermElabM".toName, "Lean.Elab.Term.instMonadMacroAdapterTermElabM".toName, "Lean.Elab.Term.instMonadTermElabM".toName, "Lean.EnvExtension.asyncMode".toName, "Lean.Expr.app".toName, "Lean.Expr.bvar".toName, "Lean.Expr.casesOn".toName, "Lean.Expr.const".toName, "Lean.Expr.ctorIdx".toName, "Lean.Expr.forallE".toName, "Lean.Expr.fvar".toName, "Lean.Expr.instBEq".toName, "Lean.Expr.instHashable".toName, "Lean.Expr.lam".toName, "Lean.Expr.letE".toName, "Lean.Expr.lit".toName, "Lean.Expr.mdata".toName, "Lean.Expr.mvar".toName, "Lean.Expr.proj".toName, "Lean.Expr.rec".toName, "Lean.Expr.sort".toName, "Lean.FVarId.casesOn".toName, "Lean.FVarId.name".toName, "Lean.InductiveVal.ctors".toName, "Lean.InductiveVal.isRec".toName, "Lean.InductiveVal.numIndices".toName, "Lean.InductiveVal.numParams".toName, "Lean.Level.instBEq".toName, "Lean.Level.instToString".toName, "Lean.LocalContext.mk".toName, "Lean.Loop.mk".toName, "Lean.MessageData.instAppend".toName, "Lean.Meta.Cache.mk".toName, "Lean.Meta.Context.mk".toName, "Lean.Meta.Diagnostics.mk".toName, "Lean.Meta.State.mk".toName, "Lean.Meta.instBEqDefEqCacheKey".toName, "Lean.Meta.instBEqExprConfigCacheKey".toName, "Lean.Meta.instBEqInfoCacheKey".toName, "Lean.Meta.instBEqSynthInstanceCacheKey".toName, "Lean.Meta.instHashableDefEqCacheKey".toName, "Lean.Meta.instHashableExprConfigCacheKey".toName, "Lean.Meta.instHashableInfoCacheKey".toName, "Lean.Meta.instHashableSynthInstanceCacheKey".toName, "Lean.Meta.instInhabitedConfigWithKey".toName, "Lean.Meta.instMonadEnvMetaM".toName, "Lean.Meta.instMonadMCtxMetaM".toName, "Lean.Meta.instMonadMetaM".toName, "Lean.MetavarContext.mk".toName, "Lean.MonadEnv.getEnv".toName, "Lean.MonadError.mk".toName, "Lean.MonadQuotation.toMonadRef".toName, "Lean.Name.anonymous".toName, "Lean.Name.instBEq".toName, "Lean.Name.instToString".toName, "Lean.NameSet.instEmptyCollection".toName, "Lean.ParserDescr.binary".toName, "Lean.ParserDescr.cat".toName, "Lean.ParserDescr.const".toName, "Lean.ParserDescr.node".toName, "Lean.ParserDescr.symbol".toName, "Lean.PersistentArray.mk".toName, "Lean.PersistentArrayNode.node".toName, "Lean.PersistentEnvExtension.toEnvExtension".toName, "Lean.PersistentHashMap.Node.entries".toName, "Lean.PersistentHashMap.mk".toName, "Lean.SMap.map₂".toName, "Lean.Syntax.instGetElemNatTrue".toName, "Lean.ToMessageData.toMessageData".toName, "Lean.instAddErrorMessageContextOfAddMessageContextOfMonad".toName, "Lean.instBEqBinderInfo".toName, "Lean.instBEqFVarId".toName, "Lean.instBEqLevelMVarId".toName, "Lean.instBEqLiteral".toName, "Lean.instBEqMVarId".toName, "Lean.instEmptyCollectionFVarIdMap".toName, "Lean.instEmptyCollectionFVarIdSet".toName, "Lean.instForInLoopUnitOfMonad".toName, "Lean.instHashableFVarId".toName, "Lean.instHashableLevelMVarId".toName, "Lean.instHashableMVarId".toName, "Lean.instHashableName".toName, "Lean.instInhabitedName".toName, "Lean.instMonadEnvOfMonadLift".toName, "Lean.instMonadExceptOfExceptionCoreM".toName, "Lean.instMonadMCtxOfMonadLift".toName, "Lean.instToMessageDataExpr".toName, "Lean.instToMessageDataName".toName, "Lean.instToMessageDataString".toName, "List".toName, "List.cons".toName, "List.drop".toName, "List.instAppend".toName, "List.instBEq".toName, "List.instForIn'InferInstanceMembershipOfMonad".toName, "List.instMembership".toName, "List.map".toName, "List.nil".toName, "List.reverse".toName, "List.toArray".toName, "Membership".toName, "Monad.toBind".toName, "Nat".toName, "Nat.decLt".toName, "Nat.hasNotBit".toName, "Nat.land".toName, "Nat.ne_of_beq_eq_false".toName, "Nat.shiftRight".toName, "Nat.toDigits".toName, "OfNat.ofNat".toName, "Option".toName, "Option.casesOn".toName, "Option.ctorIdx".toName, "Option.getD".toName, "Option.isSome".toName, "Option.map".toName, "Option.none".toName, "Option.rec".toName, "Option.some".toName, "PUnit".toName, "PUnit.unit".toName, "Prod".toName, "Prod.casesOn".toName, "Prod.fst".toName, "Prod.mk".toName, "Prod.snd".toName, "Pure.pure".toName, "ReaderT".toName, "ReaderT.instApplicativeOfMonad".toName, "ReaderT.instMonadExceptOf".toName, "ReaderT.instMonadLift".toName, "StateRefT'".toName, "StateRefT'.instMonad".toName, "StateRefT'.instMonadExceptOf".toName, "StateRefT'.instMonadLift".toName, "Std.HashMap".toName, "Std.HashMap.contains".toName, "Std.HashMap.fold".toName, "Std.HashMap.get?".toName, "Std.HashMap.insert".toName, "Std.HashMap.instEmptyCollection".toName, "Std.TreeSet.toList".toName, "String".toName, "String.Slice.Pattern.ForwardSliceSearcher".toName, "String.Slice.Pattern.ForwardSliceSearcher.instIteratorIdSearchStep".toName, "String.Slice.Pattern.ForwardSliceSearcher.instIteratorLoopIdSearchStep".toName, "String.Slice.Pattern.ForwardSliceSearcher.instToForwardSearcher_1".toName, "String.contains".toName, "String.foldl".toName, "String.instInhabited".toName, "String.intercalate".toName, "String.ofList".toName, "String.push".toName, "ToString.toString".toName, "True".toName, "True.intro".toName, "UInt16".toName, "UInt16.toNat".toName, "UInt32".toName, "UInt32.toNat".toName, "UInt64".toName, "UInt64.toNat".toName, "UInt8".toName, "UInt8.toNat".toName, "USize.toNat".toName, "Unit".toName, "Unit.unit".toName, "eq_of_heq".toName, "inferInstance".toName, "instAddNat".toName, "instAppendString".toName, "instBEqOfDecidableEq".toName, "instDecidableEqBool".toName, "instDecidableEqChar".toName, "instDecidableEqNat".toName, "instDecidableEqString".toName, "instForInOfForIn'".toName, "instHAdd".toName, "instHAppendOfAppend".toName, "instHSub".toName, "instInhabitedBool".toName, "instLTNat".toName, "instMonadEIO".toName, "instMonadLiftT".toName, "instMonadLiftTOfMonadLift".toName, "instOfNatNat".toName, "instSubNat".toName, "instToStringArray".toName, "instToStringNat".toName, "instToStringString".toName, "ite".toName, "liftM".toName, "noConfusion_of_Nat".toName]
#eval LexLeanExtract.main #["Production.Main.checkedSum".toName, "Production.Main.shapeArea".toName, "Production.Main.quadruple".toName, "Production.Main.halvings".toName, "Production.Main.sumAll".toName, "Production.Main.headOr".toName] #["Production.Kernel".toName, "Production.Main".toName]
