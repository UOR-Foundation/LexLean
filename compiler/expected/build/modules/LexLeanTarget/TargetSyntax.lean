module
public import Init
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanTarget.TargetSyntax

public inductive IntKind where
  | u8
  | u16
  | u32
  | u64
  | i8
  | i16
  | i32
  | i64

public inductive Order where
  | less
  | same
  | more

public inductive Ty where
  | unit
  | bool
  | nat
  | int
  | fixed (_ : IntKind)
  | string
  | bytes
  | ordering
  | option (_ : Ty)
  | result (_ : Ty) (_ : Ty)
  | list (_ : Ty)
  | pair (_ : Ty) (_ : Ty)
  | adt (_ : Nat)
  | fn (_ : List (Ty)) (_ : Ty)

public inductive Value where
  | unit
  | bool (_ : Bool)
  | nat (_ : Nat)
  | int (_ : Int)
  | u8 (_ : UInt8)
  | u16 (_ : UInt16)
  | u32 (_ : UInt32)
  | u64 (_ : UInt64)
  | i8 (_ : Int8)
  | i16 (_ : Int16)
  | i32 (_ : Int32)
  | i64 (_ : Int64)
  | string (_ : String)
  | bytes (_ : ByteArray)
  | ordering (_ : Order)
  | none
  | some (_ : Value)
  | ok (_ : Value)
  | error (_ : Value)
  | list (_ : List (Value))
  | pair (_ : Value) (_ : Value)
  | adt (_ : Nat) (_ : List (Value))
  | closure (_ : Nat) (_ : List (Value))

public inductive Shape where
  | none
  | some
  | ok
  | error
  | nil
  | cons
  | zero
  | succ
  | pair
  | true
  | false
  | unit
  | lt
  | eq
  | gt
  | adt (_ : Nat)

public inductive Prim where
  | natAdd
  | natSub
  | natMul
  | natQuot
  | natRem
  | natEq
  | natLe
  | natLt
  | intAdd
  | intSub
  | intMul
  | intNeg
  | intQuot
  | intRem
  | checkedAdd
  | checkedSub
  | checkedMul
  | checkedNeg
  | checkedQuot
  | bitAnd
  | bitOr
  | bitXor
  | bitNot
  | shiftLeft
  | shiftRight
  | equal
  | boolNot
  | boolAnd
  | boolOr
  | append
  | length
  | index
  | slice
  | utf8Encode
  | utf8Decode
  | compareBytes
  | splitExact
  | join
  | formatDecimal
  | compare
  | convert (_ : IntKind)
  | parseDecimal (_ : Ty)

mutual
public inductive Expr where
  | value (_ : Ty) (_ : Value)
  | var (_ : Nat)
  | «let» (_ : Nat) (_ : Ty) (_ : Expr) (_ : Expr)
  | cond (_ : Expr) (_ : Expr) (_ : Expr)
  | «match» (_ : Ty) (_ : Expr) (_ : List (Arm))
  | build (_ : Shape) (_ : Ty) (_ : List (Expr))
  | call (_ : Nat) (_ : List (Expr))
  | closure (_ : Nat) (_ : List (Expr))
  | apply (_ : Expr) (_ : List (Expr))
  | prim (_ : Prim) (_ : List (Expr))
  | first (_ : Expr)
  | second (_ : Expr)
  | field (_ : Expr) (_ : Nat)

public inductive Arm where
  | arm (_ : Shape) (_ : List (Nat)) (_ : Expr)
end

public structure Adt where
  constructors : List (List (Ty))

public structure Function where
  parameters : List (Nat)
  types : List (Ty)
  result : Ty
  body : Expr

public structure Program where
  adts : List (Adt)
  functions : List (Function)

end LexLeanTarget.TargetSyntax
