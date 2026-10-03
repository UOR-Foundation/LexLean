module
public import Init
public import LexLeanTarget.TargetSyntax
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
namespace LexLeanTarget.RustSyntax

public inductive IdentKind where
  | binding
  | holder
  | operand
  | callee
  | boxed
  | part
  | capture
  | param
  | function

public inductive Ident where
  | generated (_ : IdentKind) (_ : Nat)
  | exported (_ : String)

public inductive RustType where
  | unit
  | bool
  | nat
  | int
  | fixed (_ : LexLeanTarget.TargetSyntax.IntKind)
  | ordering
  | str
  | bytes
  | option (_ : RustType)
  | result (_ : RustType) (_ : RustType)
  | list (_ : RustType)
  | pair (_ : RustType) (_ : RustType)
  | adt (_ : Nat)
  | fn (_ : Nat)
  | rc (_ : RustType)
  | fallible (_ : RustType)
  | ref (_ : RustType)

public inductive Lit where
  | unit
  | bool (_ : Bool)
  | nat (_ : Nat)
  | int (_ : Int)
  | fixed (_ : LexLeanTarget.TargetSyntax.IntKind) (_ : Int)
  | str (_ : String)
  | bytes (_ : ByteArray)
  | ordering (_ : LexLeanTarget.TargetSyntax.Order)

public inductive Ctor where
  | none (_ : RustType)
  | some
  | ok (_ : RustType) (_ : RustType)
  | err (_ : RustType) (_ : RustType)
  | adt (_ : Nat) (_ : Nat)
  | closure (_ : Nat) (_ : Nat)
  | cons
  | nil (_ : RustType)

public inductive Pat where
  | wild
  | bind (_ : Ident)
  | unit
  | tuple (_ : List (Pat))
  | none
  | some (_ : Pat)
  | ok (_ : Pat)
  | err (_ : Pat)
  | ordering (_ : LexLeanTarget.TargetSyntax.Order)
  | adt (_ : Nat) (_ : Nat) (_ : List (Pat))

public inductive Item where
  | natAdd
  | natSub
  | natMul
  | natQuot
  | natRem
  | natEq
  | natLe
  | natLt
  | natSucc
  | intAdd
  | intSub
  | intMul
  | intNeg
  | intQuot
  | intRem
  | boolNot
  | boolAnd
  | boolOr
  | equal
  | compare
  | checkedAdd (_ : LexLeanTarget.TargetSyntax.IntKind)
  | checkedSub (_ : LexLeanTarget.TargetSyntax.IntKind)
  | checkedMul (_ : LexLeanTarget.TargetSyntax.IntKind)
  | checkedQuot (_ : LexLeanTarget.TargetSyntax.IntKind)
  | checkedNeg (_ : LexLeanTarget.TargetSyntax.IntKind)
  | bitAnd (_ : LexLeanTarget.TargetSyntax.IntKind)
  | bitOr (_ : LexLeanTarget.TargetSyntax.IntKind)
  | bitXor (_ : LexLeanTarget.TargetSyntax.IntKind)
  | bitNot (_ : LexLeanTarget.TargetSyntax.IntKind)
  | shiftLeft (_ : LexLeanTarget.TargetSyntax.IntKind)
  | shiftRight (_ : LexLeanTarget.TargetSyntax.IntKind)
  | convert (_ : LexLeanTarget.TargetSyntax.IntKind)
  | appendList
  | appendBytes
  | lengthList
  | lengthBytes
  | lengthString
  | indexList
  | indexBytes
  | sliceList
  | sliceBytes
  | utf8Encode
  | utf8Decode
  | compareBytes
  | splitExact
  | join
  | formatInt
  | formatFixed (_ : LexLeanTarget.TargetSyntax.IntKind)
  | parseInt
  | parseFixed (_ : LexLeanTarget.TargetSyntax.IntKind)

public inductive Callee where
  | function (_ : Nat)
  | runtime (_ : Item)

mutual
public inductive Expr where
  | lit (_ : Lit)
  | move (_ : Ident)
  | clone (_ : Ident)
  | copy (_ : Ident)
  | deref (_ : Ident)
  | not (_ : Expr)
  | unbox (_ : Ident)
  | box (_ : Expr)
  | call (_ : Callee) (_ : List (Expr)) (_ : Bool)
  | apply (_ : Ident) (_ : List (Expr)) (_ : Bool)
  | construct (_ : Ctor) (_ : List (Expr))
  | pair (_ : Expr) (_ : Expr)
  | cond (_ : Expr) (_ : Block) (_ : Block)
  | matchOn (_ : Expr) (_ : List (Arm))
  | block (_ : Block)
  | uncons (_ : Ident)
  | isZero (_ : Ident)
  | nonZero (_ : Ident)
  | predecessor (_ : Ident)
  | widen (_ : Expr)
  | succeed (_ : Expr)

public inductive Let where
  | mk (_ : Pat) (_ : Option (RustType)) (_ : Expr)

public inductive Block where
  | mk (_ : List (Let)) (_ : Expr)

public inductive Arm where
  | mk (_ : Pat) (_ : Block)
end

public inductive CaptureRead where
  | copy
  | clone
  | unbox
  | unit

public structure Dispatch where
  function : Nat
  captures : List (CaptureRead)
  functionFallible : Bool

public structure Variant where
  number : Nat
  fields : List (RustType)

public structure Parameter where
  pattern : Pat
  type : RustType

public inductive ItemDef where
  | enum (_ : RustType) (_ : List (Variant))
  | apply (_ : Nat) (_ : List (RustType)) (_ : RustType) (_ : Bool) (_ : List (Dispatch))
  | function (_ : Ident) (_ : List (Parameter)) (_ : RustType) (_ : Block)

public inductive Profile where
  | core
  | std

public structure Crate where
  profile : Profile
  items : List (ItemDef)

end LexLeanTarget.RustSyntax
