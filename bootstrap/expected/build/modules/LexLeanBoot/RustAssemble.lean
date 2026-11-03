module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanBoot.CalculusCheck
public import LexLeanBoot.RustAst
public import LexLeanBoot.RustBodies
public import LexLeanBoot.RustLoops
public import LexLeanBoot.RustLower
public import LexLeanBoot.RustTerms
public import LexLeanBoot.RustValidate
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.RustAssemble

@[expose] public def kindText (ty : LexLeanBoot.CalculusCheck.Ty) : ByteArray := (match ty with | LexLeanBoot.CalculusCheck.Ty.TUnit => (LexLeanRuntime.utf8Encode ("type:unit") : ByteArray) | LexLeanBoot.CalculusCheck.Ty.TBool => (LexLeanRuntime.utf8Encode ("type:bool") : ByteArray) | LexLeanBoot.CalculusCheck.Ty.TNat => (LexLeanRuntime.utf8Encode ("type:nat") : ByteArray) | LexLeanBoot.CalculusCheck.Ty.TInt => (LexLeanRuntime.utf8Encode ("type:int") : ByteArray) | LexLeanBoot.CalculusCheck.Ty.TFixed _ => (LexLeanRuntime.utf8Encode ("type:fixed") : ByteArray) | LexLeanBoot.CalculusCheck.Ty.TString => (LexLeanRuntime.utf8Encode ("type:string") : ByteArray) | LexLeanBoot.CalculusCheck.Ty.TBytes => (LexLeanRuntime.utf8Encode ("type:bytes") : ByteArray) | LexLeanBoot.CalculusCheck.Ty.TOrdering => (LexLeanRuntime.utf8Encode ("type:ordering") : ByteArray) | LexLeanBoot.CalculusCheck.Ty.TOption _ => (LexLeanRuntime.utf8Encode ("type:option") : ByteArray) | LexLeanBoot.CalculusCheck.Ty.TResult _ _ => (LexLeanRuntime.utf8Encode ("type:result") : ByteArray) | LexLeanBoot.CalculusCheck.Ty.TList _ => (LexLeanRuntime.utf8Encode ("type:list") : ByteArray) | LexLeanBoot.CalculusCheck.Ty.TPair _ _ => (LexLeanRuntime.utf8Encode ("type:pair") : ByteArray) | LexLeanBoot.CalculusCheck.Ty.TAdt _ => (LexLeanRuntime.utf8Encode ("type:adt") : ByteArray) | LexLeanBoot.CalculusCheck.Ty.TFn _ _ => (LexLeanRuntime.utf8Encode ("type:fn") : ByteArray) | LexLeanBoot.CalculusCheck.Ty.TNil => (LexLeanRuntime.utf8Encode ("type:nil") : ByteArray) | LexLeanBoot.CalculusCheck.Ty.TCons _ _ => (LexLeanRuntime.utf8Encode ("type:nil") : ByteArray))

@[expose] public def typeOrigin (ty : LexLeanBoot.CalculusCheck.Ty) : LexLeanBoot.RustValidate.Origin := ({ element := kindText (ty), width := Option.none } : LexLeanBoot.RustValidate.Origin)

@[expose] public def functionAt (_seed : Nat) : LexLeanBoot.RustValidate.Origin := ({ element := (LexLeanRuntime.utf8Encode ("function") : ByteArray), width := Option.none } : LexLeanBoot.RustValidate.Origin)

@[expose] public def firstEmpty : (types : LexLeanBoot.CalculusCheck.Ty) -> (inhabited : LexLeanBoot.CalculusCheck.Ty) -> (index : Nat) -> Option (Nat)
  | LexLeanBoot.CalculusCheck.Ty.TUnit, _inhabited, _index => Option.none
  | LexLeanBoot.CalculusCheck.Ty.TBool, _inhabited, _index => Option.none
  | LexLeanBoot.CalculusCheck.Ty.TNat, _inhabited, _index => Option.none
  | LexLeanBoot.CalculusCheck.Ty.TInt, _inhabited, _index => Option.none
  | LexLeanBoot.CalculusCheck.Ty.TFixed _, _inhabited, _index => Option.none
  | LexLeanBoot.CalculusCheck.Ty.TString, _inhabited, _index => Option.none
  | LexLeanBoot.CalculusCheck.Ty.TBytes, _inhabited, _index => Option.none
  | LexLeanBoot.CalculusCheck.Ty.TOrdering, _inhabited, _index => Option.none
  | LexLeanBoot.CalculusCheck.Ty.TOption _, _inhabited, _index => Option.none
  | LexLeanBoot.CalculusCheck.Ty.TResult _ _, _inhabited, _index => Option.none
  | LexLeanBoot.CalculusCheck.Ty.TList _, _inhabited, _index => Option.none
  | LexLeanBoot.CalculusCheck.Ty.TPair _ _, _inhabited, _index => Option.none
  | LexLeanBoot.CalculusCheck.Ty.TAdt _, _inhabited, _index => Option.none
  | LexLeanBoot.CalculusCheck.Ty.TFn _ _, _inhabited, _index => Option.none
  | LexLeanBoot.CalculusCheck.Ty.TNil, _inhabited, _index => Option.none
  | LexLeanBoot.CalculusCheck.Ty.TCons head tail, inhabited, index => (if LexLeanBoot.RustLower.visible (head) (inhabited) then firstEmpty (tail) (inhabited) ((index + 1)) else Option.some (index))

@[expose] public def binderPat (type : LexLeanBoot.CalculusCheck.Ty) (body : LexLeanBoot.CalculusCheck.Expr) (name : Nat) : LexLeanBoot.RustAst.Pat := (if ((!LexLeanBoot.CalculusCheck.tyEq (type) (LexLeanBoot.CalculusCheck.Ty.TUnit)) && LexLeanBoot.RustBodies.mentions (body) (name)) then LexLeanBoot.RustAst.Pat.PBind (LexLeanBoot.RustValidate.Id.IdLocal (name)) else LexLeanBoot.RustBodies.wildPat (0))

@[expose] public def paramsOf : (types : LexLeanBoot.CalculusCheck.Ty) -> (names : List (Nat)) -> (body : LexLeanBoot.CalculusCheck.Expr) -> (known : LexLeanBoot.CalculusCheck.Ty) -> (position : Nat) -> (empty : Nat) -> LexLeanBoot.RustAst.Par
  | LexLeanBoot.CalculusCheck.Ty.TUnit, _names, _body, _known, _position, _empty => LexLeanBoot.RustAst.Par.ParNil
  | LexLeanBoot.CalculusCheck.Ty.TBool, _names, _body, _known, _position, _empty => LexLeanBoot.RustAst.Par.ParNil
  | LexLeanBoot.CalculusCheck.Ty.TNat, _names, _body, _known, _position, _empty => LexLeanBoot.RustAst.Par.ParNil
  | LexLeanBoot.CalculusCheck.Ty.TInt, _names, _body, _known, _position, _empty => LexLeanBoot.RustAst.Par.ParNil
  | LexLeanBoot.CalculusCheck.Ty.TFixed _, _names, _body, _known, _position, _empty => LexLeanBoot.RustAst.Par.ParNil
  | LexLeanBoot.CalculusCheck.Ty.TString, _names, _body, _known, _position, _empty => LexLeanBoot.RustAst.Par.ParNil
  | LexLeanBoot.CalculusCheck.Ty.TBytes, _names, _body, _known, _position, _empty => LexLeanBoot.RustAst.Par.ParNil
  | LexLeanBoot.CalculusCheck.Ty.TOrdering, _names, _body, _known, _position, _empty => LexLeanBoot.RustAst.Par.ParNil
  | LexLeanBoot.CalculusCheck.Ty.TOption _, _names, _body, _known, _position, _empty => LexLeanBoot.RustAst.Par.ParNil
  | LexLeanBoot.CalculusCheck.Ty.TResult _ _, _names, _body, _known, _position, _empty => LexLeanBoot.RustAst.Par.ParNil
  | LexLeanBoot.CalculusCheck.Ty.TList _, _names, _body, _known, _position, _empty => LexLeanBoot.RustAst.Par.ParNil
  | LexLeanBoot.CalculusCheck.Ty.TPair _ _, _names, _body, _known, _position, _empty => LexLeanBoot.RustAst.Par.ParNil
  | LexLeanBoot.CalculusCheck.Ty.TAdt _, _names, _body, _known, _position, _empty => LexLeanBoot.RustAst.Par.ParNil
  | LexLeanBoot.CalculusCheck.Ty.TFn _ _, _names, _body, _known, _position, _empty => LexLeanBoot.RustAst.Par.ParNil
  | LexLeanBoot.CalculusCheck.Ty.TNil, _names, _body, _known, _position, _empty => LexLeanBoot.RustAst.Par.ParNil
  | LexLeanBoot.CalculusCheck.Ty.TCons head tail, names, body, known, position, empty => (match names with | List.nil => LexLeanBoot.RustAst.Par.ParNil | List.cons name rest => LexLeanBoot.RustAst.Par.ParCons ((if (Nat.beq (empty) (0)) then binderPat (head) (body) (name) else (if (Nat.beq ((position + 1)) (empty)) then LexLeanBoot.RustAst.Pat.PBind (LexLeanBoot.RustValidate.Id.IdLocal (name)) else LexLeanBoot.RustBodies.wildPat (0)))) (LexLeanBoot.RustLower.vty (known) (head)) (paramsOf (tail) (rest) (body) (known) ((position + 1)) (empty)))

@[expose] public def emptyBody (name : Nat) (ty : LexLeanBoot.CalculusCheck.Ty) : LexLeanBoot.RustAst.Expr := LexLeanBoot.RustAst.Expr.EBlock (LexLeanBoot.RustAst.Expr.ENil) (LexLeanBoot.RustAst.Expr.EMatch (LexLeanBoot.RustAst.Expr.ERead (LexLeanBoot.RustValidate.ReadKind.RkMove) (LexLeanBoot.RustValidate.Id.IdLocal (name)) (typeOrigin (ty))) (LexLeanBoot.RustAst.Expr.ENil) (typeOrigin (ty)))

@[expose] public def nameAt (names : List (Nat)) (index : Nat) : Nat := (match LexLeanBoot.CalculusCheck.listAt (Nat) (names) (index) with | Option.some name => name | Option.none => 0)

@[expose] public def typeAt (types : LexLeanBoot.CalculusCheck.Ty) (index : Nat) : LexLeanBoot.CalculusCheck.Ty := (match LexLeanBoot.RustTerms.chainGet (types) (index) with | Option.some found => found | Option.none => LexLeanBoot.CalculusCheck.Ty.TNil)

@[expose] public def shell (names : List (Nat)) (types : LexLeanBoot.CalculusCheck.Ty) (result : LexLeanBoot.CalculusCheck.Ty) (failing : Bool) (body : LexLeanBoot.CalculusCheck.Expr) (known : LexLeanBoot.CalculusCheck.Ty) (inhabited : LexLeanBoot.CalculusCheck.Ty) (position : Nat) : LexLeanBoot.RustAst.Def := (let resultTy : LexLeanBoot.RustValidate.Ty := (if failing then LexLeanBoot.RustValidate.Ty.TyUnary (LexLeanBoot.RustValidate.UnaryKind.UkFallible) (LexLeanBoot.RustLower.vty (known) (result)) else LexLeanBoot.RustLower.vty (known) (result)); (match firstEmpty (types) (inhabited) (0) with | Option.some empty => LexLeanBoot.RustAst.Def.DFunction (LexLeanBoot.RustValidate.Id.IdFunction (position)) (paramsOf (types) (names) (body) (known) (0) ((empty + 1))) (resultTy) (emptyBody (nameAt (names) (empty)) (typeAt (types) (empty))) (functionAt (0)) | Option.none => LexLeanBoot.RustAst.Def.DFunction (LexLeanBoot.RustValidate.Id.IdFunction (position)) (paramsOf (types) (names) (body) (known) (0) (0)) (resultTy) (LexLeanBoot.RustAst.Expr.EBlock (LexLeanBoot.RustAst.Expr.ENil) (LexLeanBoot.RustAst.Expr.ENil)) (functionAt (0))))

@[expose] public def shellsFrom : (bodies : LexLeanBoot.CalculusCheck.Expr) -> (names : List (List (Nat))) -> (types : LexLeanBoot.CalculusCheck.Ty) -> (results : LexLeanBoot.CalculusCheck.Ty) -> (fallible : List (Bool)) -> (known : LexLeanBoot.CalculusCheck.Ty) -> (inhabited : LexLeanBoot.CalculusCheck.Ty) -> (position : Nat) -> LexLeanBoot.RustAst.Def
  | LexLeanBoot.CalculusCheck.Expr.EValue _ _, _names, _types, _results, _fallible, _known, _inhabited, _position => LexLeanBoot.RustAst.Def.DNil
  | LexLeanBoot.CalculusCheck.Expr.EVar _, _names, _types, _results, _fallible, _known, _inhabited, _position => LexLeanBoot.RustAst.Def.DNil
  | LexLeanBoot.CalculusCheck.Expr.ELet _ _ _ _, _names, _types, _results, _fallible, _known, _inhabited, _position => LexLeanBoot.RustAst.Def.DNil
  | LexLeanBoot.CalculusCheck.Expr.ECond _ _ _, _names, _types, _results, _fallible, _known, _inhabited, _position => LexLeanBoot.RustAst.Def.DNil
  | LexLeanBoot.CalculusCheck.Expr.EMatch _ _ _, _names, _types, _results, _fallible, _known, _inhabited, _position => LexLeanBoot.RustAst.Def.DNil
  | LexLeanBoot.CalculusCheck.Expr.EBuild _ _ _, _names, _types, _results, _fallible, _known, _inhabited, _position => LexLeanBoot.RustAst.Def.DNil
  | LexLeanBoot.CalculusCheck.Expr.ECall _ _, _names, _types, _results, _fallible, _known, _inhabited, _position => LexLeanBoot.RustAst.Def.DNil
  | LexLeanBoot.CalculusCheck.Expr.EClosure _ _, _names, _types, _results, _fallible, _known, _inhabited, _position => LexLeanBoot.RustAst.Def.DNil
  | LexLeanBoot.CalculusCheck.Expr.EApply _ _, _names, _types, _results, _fallible, _known, _inhabited, _position => LexLeanBoot.RustAst.Def.DNil
  | LexLeanBoot.CalculusCheck.Expr.EPrim _ _, _names, _types, _results, _fallible, _known, _inhabited, _position => LexLeanBoot.RustAst.Def.DNil
  | LexLeanBoot.CalculusCheck.Expr.EFirst _, _names, _types, _results, _fallible, _known, _inhabited, _position => LexLeanBoot.RustAst.Def.DNil
  | LexLeanBoot.CalculusCheck.Expr.ESecond _, _names, _types, _results, _fallible, _known, _inhabited, _position => LexLeanBoot.RustAst.Def.DNil
  | LexLeanBoot.CalculusCheck.Expr.EField _ _, _names, _types, _results, _fallible, _known, _inhabited, _position => LexLeanBoot.RustAst.Def.DNil
  | LexLeanBoot.CalculusCheck.Expr.ENil, _names, _types, _results, _fallible, _known, _inhabited, _position => LexLeanBoot.RustAst.Def.DNil
  | LexLeanBoot.CalculusCheck.Expr.ECons body rest, names, types, results, fallible, known, inhabited, position => (match names with | List.nil => LexLeanBoot.RustAst.Def.DNil | List.cons these restNames => (match types with | LexLeanBoot.CalculusCheck.Ty.TCons own restTypes => (match results with | LexLeanBoot.CalculusCheck.Ty.TCons result restResults => LexLeanBoot.RustAst.Def.DCons (shell (these) (own) (result) (LexLeanBoot.RustLower.flagAt (fallible) (position)) (body) (known) (inhabited) (position)) (shellsFrom (rest) (restNames) (restTypes) (restResults) (fallible) (known) (inhabited) ((position + 1))) | LexLeanBoot.CalculusCheck.Ty.TNil => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TUnit => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TBool => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TNat => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TInt => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TFixed _ => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TString => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TBytes => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TOrdering => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TOption _ => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TResult _ _ => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TList _ => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TPair _ _ => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TAdt _ => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TFn _ _ => LexLeanBoot.RustAst.Def.DNil) | LexLeanBoot.CalculusCheck.Ty.TNil => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TUnit => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TBool => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TNat => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TInt => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TFixed _ => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TString => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TBytes => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TOrdering => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TOption _ => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TResult _ _ => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TList _ => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TPair _ _ => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TAdt _ => LexLeanBoot.RustAst.Def.DNil | LexLeanBoot.CalculusCheck.Ty.TFn _ _ => LexLeanBoot.RustAst.Def.DNil))
  | LexLeanBoot.CalculusCheck.Expr.EArm _ _ _, _names, _types, _results, _fallible, _known, _inhabited, _position => LexLeanBoot.RustAst.Def.DNil

@[expose] public def functionShells (spec : ByteArray) (adts : LexLeanBoot.CalculusCheck.Ty) (names : List (List (Nat))) (types : LexLeanBoot.CalculusCheck.Ty) (results : LexLeanBoot.CalculusCheck.Ty) (bodies : LexLeanBoot.CalculusCheck.Expr) (fallible : List (Bool)) (frame : LexLeanBoot.RustBodies.Frame) : LexLeanBoot.RustAst.Def := (let program : LexLeanBoot.CalculusCheck.Program := ({ spec := spec, adts := adts, parameters := names, types := types, results := results, bodies := bodies } : LexLeanBoot.CalculusCheck.Program); (let known : LexLeanBoot.CalculusCheck.Ty := (frame).known; (let owners : LexLeanBoot.CalculusCheck.Ty := LexLeanBoot.RustLower.tyAppend (LexLeanBoot.RustLower.adtOwners (LexLeanBoot.CalculusCheck.tyChainLength (adts))) (known); (let inhabited : LexLeanBoot.CalculusCheck.Ty := LexLeanBoot.RustLower.inhabitation (program) (known) (LexLeanBoot.RustLower.revClos ((frame).ordered) (LexLeanBoot.RustLower.Clos.ClNil)) ((frame).edges) (owners); shellsFrom (bodies) (names) (types) (results) (fallible) (known) (inhabited) (0)))))

@[expose] public def appendDefs : (subject : LexLeanBoot.RustAst.Def) -> (tail : LexLeanBoot.RustAst.Def) -> LexLeanBoot.RustAst.Def
  | LexLeanBoot.RustAst.Def.DEnum _ _ _, tail => tail
  | LexLeanBoot.RustAst.Def.DApply _ _ _ _ _ _, tail => tail
  | LexLeanBoot.RustAst.Def.DFunction _ _ _ _ _, tail => tail
  | LexLeanBoot.RustAst.Def.DNil, tail => tail
  | LexLeanBoot.RustAst.Def.DCons head rest, tail => LexLeanBoot.RustAst.Def.DCons (head) (appendDefs (rest) (tail))

@[expose] public def placeLoop (item : LexLeanBoot.RustAst.Def) (out : LexLeanBoot.RustLoops.LoopOut) : LexLeanBoot.RustAst.Def := (match item with | LexLeanBoot.RustAst.Def.DEnum _ _ _ => item | LexLeanBoot.RustAst.Def.DApply _ _ _ _ _ _ => item | LexLeanBoot.RustAst.Def.DFunction name _ result _ «at» => LexLeanBoot.RustAst.Def.DFunction (name) ((out).params) (result) ((out).body) («at») | LexLeanBoot.RustAst.Def.DNil => item | LexLeanBoot.RustAst.Def.DCons _ _ => item)

@[expose] public def placeAll : (shells : LexLeanBoot.RustAst.Def) -> (outs : List (LexLeanBoot.RustLoops.LoopOut)) -> (dead : List (Bool)) -> (position : Nat) -> LexLeanBoot.RustAst.Def
  | LexLeanBoot.RustAst.Def.DEnum _ _ _, _outs, _dead, _position => LexLeanBoot.RustAst.Def.DNil
  | LexLeanBoot.RustAst.Def.DApply _ _ _ _ _ _, _outs, _dead, _position => LexLeanBoot.RustAst.Def.DNil
  | LexLeanBoot.RustAst.Def.DFunction _ _ _ _ _, _outs, _dead, _position => LexLeanBoot.RustAst.Def.DNil
  | LexLeanBoot.RustAst.Def.DNil, _outs, _dead, _position => LexLeanBoot.RustAst.Def.DNil
  | LexLeanBoot.RustAst.Def.DCons item rest, outs, dead, position => (if LexLeanBoot.RustLower.flagAt (dead) (position) then LexLeanBoot.RustAst.Def.DCons (item) (placeAll (rest) (outs) (dead) ((position + 1))) else (match outs with | List.nil => LexLeanBoot.RustAst.Def.DCons (item) (placeAll (rest) (outs) (dead) ((position + 1))) | List.cons out more => LexLeanBoot.RustAst.Def.DCons (placeLoop (item) (out)) (placeAll (rest) (more) (dead) ((position + 1)))))

@[expose] public def assembleItems (types : LexLeanBoot.RustAst.Def) (shells : LexLeanBoot.RustAst.Def) (outs : List (LexLeanBoot.RustLoops.LoopOut)) (dead : List (Bool)) : LexLeanBoot.RustAst.Def := appendDefs (types) (placeAll (shells) (outs) (dead) (0))

end LexLeanBoot.RustAssemble
