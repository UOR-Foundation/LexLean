module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanBoot.CalculusCheck
public import LexLeanBoot.RustAst
public import LexLeanBoot.RustBodies
public import LexLeanBoot.RustLower
public import LexLeanBoot.RustTerms
public import LexLeanBoot.RustValidate
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.RustSucceed

@[expose] public def overflowAt (_seed : Nat) : LexLeanBoot.RustValidate.Origin := ({ element := (LexLeanRuntime.utf8Encode ("overflow") : ByteArray), width := Option.none } : LexLeanBoot.RustValidate.Origin)

@[expose] public def succeeded (value : LexLeanBoot.RustAst.Expr) : LexLeanBoot.RustAst.Expr := LexLeanBoot.RustAst.Expr.EWrap (LexLeanBoot.RustValidate.WrapKind.WkSucceed) (value) (overflowAt (0))

@[expose] public def unitBound (value : LexLeanBoot.RustAst.Expr) : LexLeanBoot.RustAst.Expr := LexLeanBoot.RustAst.Expr.EBlockE (LexLeanBoot.RustAst.Expr.EBlock (LexLeanBoot.RustAst.Expr.ELet (LexLeanBoot.RustAst.Pat.PLeaf (LexLeanBoot.RustAst.PatLeaf.PlUnit)) (false) (LexLeanBoot.RustBodies.unitTy (0)) (value) (overflowAt (0)) (LexLeanBoot.RustAst.Expr.ENil)) (succeeded (LexLeanBoot.RustAst.Expr.ELit (LexLeanBoot.RustAst.Lit.LitUnit) (overflowAt (0)))))

@[expose] public def otherwise (whole : LexLeanBoot.RustAst.Expr) (unit : Bool) (mode : Nat) : LexLeanBoot.RustAst.Expr := (if (Nat.beq (mode) (0)) then (if unit then unitBound (whole) else succeeded (whole)) else whole)

@[expose] public def succeedExpr : (subject : LexLeanBoot.RustAst.Expr) -> (whole : LexLeanBoot.RustAst.Expr) -> (unit : Bool) -> (mode : Nat) -> LexLeanBoot.RustAst.Expr
  | LexLeanBoot.RustAst.Expr.ELit _ _, whole, unit, mode => (if ((Nat.beq (mode) (0)) && LexLeanBoot.RustBodies.unitLit (whole)) then succeeded (whole) else otherwise (whole) (unit) (mode))
  | LexLeanBoot.RustAst.Expr.ERead _ _ _, whole, unit, mode => otherwise (whole) (unit) (mode)
  | LexLeanBoot.RustAst.Expr.EWrap _ _ _, whole, unit, mode => otherwise (whole) (unit) (mode)
  | LexLeanBoot.RustAst.Expr.ECall callee args propagate «at», whole, unit, mode => (if ((Nat.beq (mode) (0)) && propagate) then LexLeanBoot.RustAst.Expr.ECall (callee) (args) (false) («at») else otherwise (whole) (unit) (mode))
  | LexLeanBoot.RustAst.Expr.EApply holder args propagate «at», whole, unit, mode => (if ((Nat.beq (mode) (0)) && propagate) then LexLeanBoot.RustAst.Expr.EApply (holder) (args) (false) («at») else otherwise (whole) (unit) (mode))
  | LexLeanBoot.RustAst.Expr.EConstruct _ _ _, whole, unit, mode => otherwise (whole) (unit) (mode)
  | LexLeanBoot.RustAst.Expr.EPair _ _ _, whole, unit, mode => otherwise (whole) (unit) (mode)
  | LexLeanBoot.RustAst.Expr.EIf c t e «at», whole, unit, mode => (if (Nat.beq (mode) (0)) then LexLeanBoot.RustAst.Expr.EIf (c) (succeedExpr (t) (t) (unit) (1)) (succeedExpr (e) (e) (unit) (1)) («at») else whole)
  | LexLeanBoot.RustAst.Expr.EMatch s arms «at», whole, unit, mode => (if (Nat.beq (mode) (0)) then LexLeanBoot.RustAst.Expr.EMatch (s) (succeedExpr (arms) (arms) (unit) (2)) («at») else whole)
  | LexLeanBoot.RustAst.Expr.EBlockE _, whole, unit, mode => otherwise (whole) (unit) (mode)
  | LexLeanBoot.RustAst.Expr.EProbe _ _ _, whole, unit, mode => otherwise (whole) (unit) (mode)
  | LexLeanBoot.RustAst.Expr.EBlock lets tail, whole, unit, mode => (if (Nat.beq (mode) (1)) then LexLeanBoot.RustBodies.setTail (LexLeanBoot.RustAst.Expr.EBlock (lets) (tail)) (succeedExpr (tail) (tail) (unit) (0)) else otherwise (whole) (unit) (mode))
  | LexLeanBoot.RustAst.Expr.ENil, whole, unit, mode => otherwise (whole) (unit) (mode)
  | LexLeanBoot.RustAst.Expr.ECons _ _, whole, unit, mode => otherwise (whole) (unit) (mode)
  | LexLeanBoot.RustAst.Expr.ELet _ _ _ _ _ _, whole, unit, mode => otherwise (whole) (unit) (mode)
  | LexLeanBoot.RustAst.Expr.EArm pats block rest, whole, unit, mode => (if (Nat.beq (mode) (2)) then LexLeanBoot.RustAst.Expr.EArm (pats) (succeedExpr (block) (block) (unit) (1)) (succeedExpr (rest) (rest) (unit) (2)) else otherwise (whole) (unit) (mode))
  | LexLeanBoot.RustAst.Expr.ELoop _ _, whole, unit, mode => otherwise (whole) (unit) (mode)

@[expose] public def unitResult (results : LexLeanBoot.CalculusCheck.Ty) (index : Nat) : Bool := (match LexLeanBoot.RustTerms.chainGet (results) (index) with | Option.some result => LexLeanBoot.CalculusCheck.tyEq (result) (LexLeanBoot.CalculusCheck.Ty.TUnit) | Option.none => false)

@[expose] public def succeedChain : (results : LexLeanBoot.CalculusCheck.Ty) -> (fallible : List (Bool)) -> (subject : LexLeanBoot.RustAst.Expr) -> (index : Nat) -> LexLeanBoot.RustAst.Expr
  | _results, _fallible, LexLeanBoot.RustAst.Expr.ELit _ _, _index => LexLeanBoot.RustAst.Expr.ENil
  | _results, _fallible, LexLeanBoot.RustAst.Expr.ERead _ _ _, _index => LexLeanBoot.RustAst.Expr.ENil
  | _results, _fallible, LexLeanBoot.RustAst.Expr.EWrap _ _ _, _index => LexLeanBoot.RustAst.Expr.ENil
  | _results, _fallible, LexLeanBoot.RustAst.Expr.ECall _ _ _ _, _index => LexLeanBoot.RustAst.Expr.ENil
  | _results, _fallible, LexLeanBoot.RustAst.Expr.EApply _ _ _ _, _index => LexLeanBoot.RustAst.Expr.ENil
  | _results, _fallible, LexLeanBoot.RustAst.Expr.EConstruct _ _ _, _index => LexLeanBoot.RustAst.Expr.ENil
  | _results, _fallible, LexLeanBoot.RustAst.Expr.EPair _ _ _, _index => LexLeanBoot.RustAst.Expr.ENil
  | _results, _fallible, LexLeanBoot.RustAst.Expr.EIf _ _ _ _, _index => LexLeanBoot.RustAst.Expr.ENil
  | _results, _fallible, LexLeanBoot.RustAst.Expr.EMatch _ _ _, _index => LexLeanBoot.RustAst.Expr.ENil
  | _results, _fallible, LexLeanBoot.RustAst.Expr.EBlockE _, _index => LexLeanBoot.RustAst.Expr.ENil
  | _results, _fallible, LexLeanBoot.RustAst.Expr.EProbe _ _ _, _index => LexLeanBoot.RustAst.Expr.ENil
  | _results, _fallible, LexLeanBoot.RustAst.Expr.EBlock _ _, _index => LexLeanBoot.RustAst.Expr.ENil
  | _results, _fallible, LexLeanBoot.RustAst.Expr.ENil, _index => LexLeanBoot.RustAst.Expr.ENil
  | results, fallible, LexLeanBoot.RustAst.Expr.ECons head tail, index => LexLeanBoot.RustAst.Expr.ECons ((if (LexLeanBoot.RustLower.flagAt (fallible) (index) && (!LexLeanBoot.RustBodies.isNilE (head))) then succeedExpr (head) (head) (unitResult (results) (index)) (1) else head)) (succeedChain (results) (fallible) (tail) ((index + 1)))
  | _results, _fallible, LexLeanBoot.RustAst.Expr.ELet _ _ _ _ _ _, _index => LexLeanBoot.RustAst.Expr.ENil
  | _results, _fallible, LexLeanBoot.RustAst.Expr.EArm _ _ _, _index => LexLeanBoot.RustAst.Expr.ENil
  | _results, _fallible, LexLeanBoot.RustAst.Expr.ELoop _ _, _index => LexLeanBoot.RustAst.Expr.ENil

@[expose] public def succeedBodies (results : LexLeanBoot.CalculusCheck.Ty) (fallible : List (Bool)) (bodies : LexLeanBoot.RustAst.Expr) : LexLeanBoot.RustAst.Expr := succeedChain (results) (fallible) (bodies) (0)

end LexLeanBoot.RustSucceed
