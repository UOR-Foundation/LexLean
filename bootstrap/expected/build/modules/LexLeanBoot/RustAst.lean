module
public import Init
public import LexLeanBoot.RustValidate
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.RustAst

public inductive OrderingKind where
  | OkLess
  | OkEqual
  | OkGreater

public inductive Lit where
  | LitUnit
  | LitBool (_ : Bool)
  | LitNat (_ : Nat)
  | LitInt (_ : Bool) (_ : Nat)
  | LitFixed (_ : LexLeanBoot.RustValidate.IntKind) (_ : Bool) (_ : Nat)
  | LitStr (_ : ByteArray)
  | LitBytes (_ : ByteArray)
  | LitOrdering (_ : OrderingKind)
  | LitOverflow

public inductive PatLeaf where
  | PlWild
  | PlUnit
  | PlNone

public inductive Pat where
  | PLeaf (_ : PatLeaf)
  | PBind (_ : LexLeanBoot.RustValidate.Id)
  | PTuple (_ : Pat)
  | PWrap (_ : LexLeanBoot.RustValidate.PatWrap) (_ : Pat)
  | PAdt (_ : Nat) (_ : Nat) (_ : Pat)
  | PNil
  | PCons (_ : Pat) (_ : Pat)
  | POrdering (_ : OrderingKind)

public inductive Callee where
  | CalFn (_ : Nat)
  | CalRt (_ : LexLeanBoot.RustValidate.Rt)
  | CalMachine (_ : Nat) (_ : Option (LexLeanBoot.RustValidate.IntKind))

public inductive Expr where
  | ELit (_ : Lit) (_ : LexLeanBoot.RustValidate.Origin)
  | ERead (_ : LexLeanBoot.RustValidate.ReadKind) (_ : LexLeanBoot.RustValidate.Id) (_ : LexLeanBoot.RustValidate.Origin)
  | EWrap (_ : LexLeanBoot.RustValidate.WrapKind) (_ : Expr) (_ : LexLeanBoot.RustValidate.Origin)
  | ECall (_ : Callee) (_ : Expr) (_ : Bool) (_ : LexLeanBoot.RustValidate.Origin)
  | EApply (_ : LexLeanBoot.RustValidate.Id) (_ : Expr) (_ : Bool) (_ : LexLeanBoot.RustValidate.Origin)
  | EConstruct (_ : LexLeanBoot.RustValidate.Con) (_ : Expr) (_ : LexLeanBoot.RustValidate.Origin)
  | EPair (_ : Expr) (_ : Expr) (_ : LexLeanBoot.RustValidate.Origin)
  | EIf (_ : Expr) (_ : Expr) (_ : Expr) (_ : LexLeanBoot.RustValidate.Origin)
  | EMatch (_ : Expr) (_ : Expr) (_ : LexLeanBoot.RustValidate.Origin)
  | EBlockE (_ : Expr)
  | EProbe (_ : LexLeanBoot.RustValidate.ProbeKind) (_ : LexLeanBoot.RustValidate.Id) (_ : LexLeanBoot.RustValidate.Origin)
  | EBlock (_ : Expr) (_ : Expr)
  | ENil
  | ECons (_ : Expr) (_ : Expr)
  | ELet (_ : Pat) (_ : Bool) (_ : LexLeanBoot.RustValidate.Ty) (_ : Expr) (_ : LexLeanBoot.RustValidate.Origin) (_ : Expr)
  | EArm (_ : Pat) (_ : Expr) (_ : Expr)
  | ELoop (_ : LexLeanBoot.RustValidate.Id) (_ : Expr)

public inductive Par where
  | ParNil
  | ParCons (_ : Pat) (_ : LexLeanBoot.RustValidate.Ty) (_ : Par)

public inductive Def where
  | DEnum (_ : LexLeanBoot.RustValidate.Ty) (_ : LexLeanBoot.RustValidate.Row) (_ : LexLeanBoot.RustValidate.Origin)
  | DApply (_ : Nat) (_ : LexLeanBoot.RustValidate.Ty) (_ : LexLeanBoot.RustValidate.Ty) (_ : Bool) (_ : LexLeanBoot.RustValidate.Dis) (_ : LexLeanBoot.RustValidate.Origin)
  | DFunction (_ : LexLeanBoot.RustValidate.Id) (_ : Par) (_ : LexLeanBoot.RustValidate.Ty) (_ : Expr) (_ : LexLeanBoot.RustValidate.Origin)
  | DNil
  | DCons (_ : Def) (_ : Def)

public structure Crate where
  core : Bool
  items : Def

end LexLeanBoot.RustAst
