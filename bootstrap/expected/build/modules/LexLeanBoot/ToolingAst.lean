module
public import Init
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.ToolingAst

public inductive BinderKind where
  | BkExplicit
  | BkImplicit
  | BkInstance

public inductive Sig where
  | SgConst (_ : ByteArray)
  | SgVar (_ : ByteArray)
  | SgApp (_ : Sig) (_ : Sig)
  | SgArrow (_ : Sig) (_ : Sig)
  | SgPi (_ : Option (ByteArray)) (_ : BinderKind) (_ : Sig) (_ : Sig)
  | SgType (_ : Option (ByteArray))
  | SgOpt (_ : Sig) (_ : Sig)
  | SgEmpty
  | SgNat (_ : ByteArray)
  | SgNil
  | SgCons (_ : Sig) (_ : Sig)

public inductive Ty where
  | TyConst (_ : ByteArray) (_ : Ty)
  | TyParam (_ : ByteArray) (_ : Ty)
  | TyFn (_ : Ty) (_ : Ty)
  | TyIndex (_ : ByteArray)
  | TyNil
  | TyCons (_ : Ty) (_ : Ty)

public inductive Pars where
  | PrNil
  | PrCons (_ : ByteArray) (_ : Ty) (_ : Pars)

public inductive Term where
  | TmVar (_ : ByteArray)
  | TmNat (_ : ByteArray)
  | TmStr (_ : ByteArray)
  | TmChar (_ : Nat)
  | TmName (_ : ByteArray)
  | TmCall (_ : ByteArray) (_ : Ty) (_ : Term)
  | TmConstruct (_ : ByteArray) (_ : Ty) (_ : Term)
  | TmField (_ : Term) (_ : ByteArray)
  | TmList (_ : Ty) (_ : Term)
  | TmArray (_ : Ty) (_ : Term)
  | TmLet (_ : ByteArray) (_ : Ty) (_ : Term) (_ : Term)
  | TmLambda (_ : Pars) (_ : Term)
  | TmApply (_ : Term) (_ : Term)
  | TmIf (_ : Term) (_ : Term) (_ : Term)
  | TmMatch (_ : Term) (_ : Term)
  | TmNil
  | TmCons (_ : Term) (_ : Term)
  | TmBranchNil
  | TmBranch (_ : Option (ByteArray)) (_ : List (ByteArray)) (_ : Term) (_ : Term)

public inductive ParamKind where
  | KdType
  | KdConstructor
  | KdIndex (_ : ByteArray)

public inductive Atom where
  | AtToken (_ : ByteArray)
  | AtIdent
  | AtType

public inductive InvokeArg where
  | IaIdent (_ : ByteArray)
  | IaType (_ : Sig)

public structure TypeParam where
  name : ByteArray
  kind : ParamKind

public structure Requires where
  parameter : ByteArray
  target : ByteArray

public structure Constructor where
  name : ByteArray
  members : Pars
  requires : Option (Requires)

public structure Typing where
  callee : ByteArray
  params : List (ByteArray)

public structure Pin where
  file : ByteArray
  sha256 : ByteArray

public inductive Decl where
  | DType (_ : ByteArray) (_ : List (TypeParam)) (_ : List (Constructor)) (_ : Bool) (_ : Option (Ty)) (_ : Pin)
  | DCall (_ : ByteArray) (_ : Sig) (_ : ByteArray) (_ : Pin)
  | DPlumbing (_ : List (ByteArray))
  | DFunction (_ : ByteArray) (_ : Bool) (_ : Bool) (_ : Option (ByteArray)) (_ : List (ByteArray)) (_ : Pars) (_ : Ty) (_ : Term)
  | DCommand (_ : ByteArray) (_ : List (Atom)) (_ : ByteArray)
  | DInvoke (_ : ByteArray) (_ : List (InvokeArg))
  | DEvaluate (_ : ByteArray) (_ : Term)

end LexLeanBoot.ToolingAst
