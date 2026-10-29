module
import Blake3GHost
import Blake3GSpecs
import Init.Data.BitVec.Basic
import Init.Prelude
set_option autoImplicit false
universe p4u p4v p4w p5u p5v p5w p6u p6v p6w
example : (x0 : Nat) → Type := BitVec
example : {x0 : Nat} → (x1 : BitVec x0) → (x2 : Nat) → BitVec x0 := BitVec.rotateLeft
example : {x0 : Nat} → (x1 : BitVec x0) → (x2 : Nat) → BitVec x0 := BitVec.rotateRight
example : (x0 : BitVec (32 : Nat)) → (x1 : Nat) → Eq (BitVec.rotateLeft (BitVec.rotateRight x0 x1) x1) x0 := Blake3GHost.rotateLeft_rotateRight
example : {x0 : Type p4u} → {x1 : Type p4v} → {x2 : Type p4w} → [HAdd.{p4u, p4v, p4w} x0 x1 x2] → (x4 : x0) → (x5 : x1) → x2 := HAdd.hAdd
example : {x0 : Type p5u} → {x1 : Type p5v} → {x2 : Type p5w} → [HSub.{p5u, p5v, p5w} x0 x1 x2] → (x4 : x0) → (x5 : x1) → x2 := HSub.hSub
example : {x0 : Type p6u} → {x1 : Type p6v} → {x2 : Type p6w} → [HXor.{p6u, p6v, p6w} x0 x1 x2] → (x4 : x0) → (x5 : x1) → x2 := HXor.hXor
example : ((x0 : BitVec (32 : Nat)) → (x1 : BitVec (32 : Nat)) → (x2 : BitVec (32 : Nat)) → Eq (HAdd.hAdd.{0, 0, 0} (HAdd.hAdd.{0, 0, 0} x0 x1) x2) (HAdd.hAdd.{0, 0, 0} (HAdd.hAdd.{0, 0, 0} x0 x2) x1)) = (type_of% @LexLeanSpec.BitVec32.add_right_comm) := rfl
example : ((x0 : BitVec (32 : Nat)) → (x1 : BitVec (32 : Nat)) → Eq (HSub.hSub.{0, 0, 0} (HAdd.hAdd.{0, 0, 0} x0 x1) x1) x0) = (type_of% @LexLeanSpec.BitVec32.add_sub_cancel_right) := rfl
example : ((x0 : BitVec (32 : Nat)) → (x1 : BitVec (32 : Nat)) → Eq (HSub.hSub.{0, 0, 0} (HAdd.hAdd.{0, 0, 0} x0 x1) x1) x0) = (type_of% @LexLeanSpec.BitVec32.BitVec.add_sub_cancel) := rfl
example : ((x0 : BitVec (32 : Nat)) → (x1 : BitVec (32 : Nat)) → (x2 : BitVec (32 : Nat)) → Eq (HXor.hXor.{0, 0, 0} (HXor.hXor.{0, 0, 0} x0 x1) x2) (HXor.hXor.{0, 0, 0} x0 (HXor.hXor.{0, 0, 0} x1 x2))) = (type_of% @LexLeanSpec.BitVec32.BitVec.xor_assoc) := rfl
example : ({x0 : BitVec (32 : Nat)} → Eq (HXor.hXor.{0, 0, 0} x0 x0) (BitVec.ofNat (32 : Nat) (0 : Nat))) = (type_of% @LexLeanSpec.BitVec32.BitVec.xor_self) := rfl
example : ({x0 : BitVec (32 : Nat)} → Eq (HXor.hXor.{0, 0, 0} x0 (BitVec.ofNat (32 : Nat) (0 : Nat))) x0) = (type_of% @LexLeanSpec.BitVec32.BitVec.xor_zero) := rfl
example : ((x0 : UInt32) → (x1 : UInt32) → Eq (HSub.hSub.{0, 0, 0} (HAdd.hAdd.{0, 0, 0} x0 x1) x1) x0) = (type_of% @LexLeanSpec.U32.UInt32.add_sub_cancel) := rfl
example : ((x0 : UInt32) → (x1 : UInt32) → (x2 : UInt32) → Eq (HXor.hXor.{0, 0, 0} (HXor.hXor.{0, 0, 0} x0 x1) x2) (HXor.hXor.{0, 0, 0} x0 (HXor.hXor.{0, 0, 0} x1 x2))) = (type_of% @LexLeanSpec.U32.UInt32.xor_assoc) := rfl
example : ({x0 : UInt32} → Eq (HXor.hXor.{0, 0, 0} x0 x0) 0) = (type_of% @LexLeanSpec.U32.UInt32.xor_self) := rfl
example : ({x0 : UInt32} → Eq (HXor.hXor.{0, 0, 0} x0 0) x0) = (type_of% @LexLeanSpec.U32.UInt32.xor_zero) := rfl
example : Type := Nat
example : Type := UInt32
#print axioms Blake3GHost.rotateLeft_rotateRight
#print axioms LexLeanSpec.BitVec32.BitVec.add_sub_cancel
#print axioms LexLeanSpec.BitVec32.BitVec.xor_assoc
#print axioms LexLeanSpec.BitVec32.BitVec.xor_self
#print axioms LexLeanSpec.BitVec32.BitVec.xor_zero
#print axioms LexLeanSpec.BitVec32.add_right_comm
#print axioms LexLeanSpec.BitVec32.add_sub_cancel_right
#print axioms LexLeanSpec.U32.UInt32.add_sub_cancel
#print axioms LexLeanSpec.U32.UInt32.xor_assoc
#print axioms LexLeanSpec.U32.UInt32.xor_self
#print axioms LexLeanSpec.U32.UInt32.xor_zero
