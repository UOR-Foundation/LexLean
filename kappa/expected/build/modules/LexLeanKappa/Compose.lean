module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanKappa.Axis
public import LexLeanKappa.Json
public import LexLeanKappa.Label
public import LexLeanStd.Text
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanKappa.Compose

@[expose] public def hexChar (n : Nat) : ByteArray := LexLeanKappa.Json.byteOf ((if (Nat.blt (n) (10)) then (48 + n) else (87 + n)))

@[expose] public def digestOn (operand : ByteArray) (start : Nat) (count : Nat) : ByteArray := LexLeanStd.Text.sliceOr (operand) (start) (count)

@[expose] public def mirrorStep (count : Nat) (digest : ByteArray) (state : (Prod (Nat) (ByteArray))) : Option ((Prod (Nat) (ByteArray))) := (if (Nat.blt ((state).1) (count)) then Option.some ((((state).1 + 1), (LexLeanRuntime.append ((state).2) (hexChar ((LexLeanRuntime.subtract (15) (LexLeanKappa.Json.hexValue (LexLeanKappa.Label.byteNat (digest) ((state).1))) : Nat))) : ByteArray))) else Option.none)

@[expose] public def mirror (operand : ByteArray) (start : Nat) (count : Nat) : ByteArray := (let digest : ByteArray := digestOn (operand) (start) (count); (((LexLeanCollections.iterateUntil ((fun (state : (Prod (Nat) (ByteArray))) => mirrorStep (count) (digest) (state))) ((count + 1)) ((0, _root_.ByteArray.mk #[])) : (Prod ((Prod (Nat) (ByteArray))) (Bool)))).1).2)

@[expose] public def lower (left : ByteArray) (right : ByteArray) : ByteArray := (if LexLeanKappa.Json.lessKey (right) (left) then right else left)

@[expose] public def upper (left : ByteArray) (right : ByteArray) : ByteArray := (if LexLeanKappa.Json.lessKey (right) (left) then left else right)

@[expose] public def sortedQuarters (operand : ByteArray) (start : Nat) (width : Nat) : ByteArray := (let a0 : ByteArray := LexLeanStd.Text.sliceOr (operand) (start) (width); (let b0 : ByteArray := LexLeanStd.Text.sliceOr (operand) ((start + width)) (width); (let c0 : ByteArray := LexLeanStd.Text.sliceOr (operand) ((start + (LexLeanRuntime.multiply (2) (width) : Nat))) (width); (let d0 : ByteArray := LexLeanStd.Text.sliceOr (operand) ((start + (LexLeanRuntime.multiply (3) (width) : Nat))) (width); (let a1 : ByteArray := lower (a0) (b0); (let b1 : ByteArray := upper (a0) (b0); (let c1 : ByteArray := lower (c0) (d0); (let d1 : ByteArray := upper (c0) (d0); (let a2 : ByteArray := lower (a1) (c1); (let c2 : ByteArray := upper (a1) (c1); (let b2 : ByteArray := lower (b1) (d1); (let d2 : ByteArray := upper (b1) (d1); (let b3 : ByteArray := lower (b2) (c2); (let c3 : ByteArray := upper (b2) (c2); (LexLeanRuntime.append (a2) ((LexLeanRuntime.append (b3) ((LexLeanRuntime.append (c3) (d2) : ByteArray)) : ByteArray)) : ByteArray)))))))))))))))

@[expose] public def degreeTag (operand : ByteArray) (start : Nat) : ByteArray := (let high : Nat := LexLeanKappa.Json.hexValue (LexLeanKappa.Label.byteNat (operand) (start)); (let low : Nat := LexLeanKappa.Json.hexValue (LexLeanKappa.Label.byteNat (operand) ((start + 1))); (if (Nat.beq ((LexLeanRuntime.remainder (((LexLeanRuntime.multiply (high) (16) : Nat) + low)) (9) (0) : Nat)) (8)) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 6))] else _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 5))])))

@[expose] public def composeG2On (axis : ByteArray) (left : ByteArray) (right : ByteArray) : Option (ByteArray) := (if LexLeanKappa.Axis.isLabelOn (axis) (left) then (if LexLeanKappa.Axis.isLabelOn (axis) (right) then LexLeanKappa.Axis.labelOn (axis) ((LexLeanRuntime.append (lower (left) (right)) (upper (left) (right)) : ByteArray)) else Option.none) else Option.none)

@[expose] public def composeF4On (axis : ByteArray) (operand : ByteArray) : Option (ByteArray) := (if LexLeanKappa.Axis.isLabelOn (axis) (operand) then (let start : Nat := (LexLeanRuntime.length (LexLeanKappa.Axis.prefixOf (axis)) : Nat); LexLeanKappa.Axis.labelOn (axis) ((LexLeanRuntime.append (LexLeanKappa.Axis.prefixOf (axis)) (lower (digestOn (operand) (start) (LexLeanKappa.Axis.width (axis))) (mirror (operand) (start) (LexLeanKappa.Axis.width (axis)))) : ByteArray))) else Option.none)

@[expose] public def composeE6On (axis : ByteArray) (operand : ByteArray) : Option (ByteArray) := (if LexLeanKappa.Axis.isLabelOn (axis) (operand) then LexLeanKappa.Axis.labelOn (axis) ((LexLeanRuntime.append (degreeTag (operand) ((LexLeanRuntime.length (LexLeanKappa.Axis.prefixOf (axis)) : Nat))) (operand) : ByteArray)) else Option.none)

@[expose] public def composeE7On (axis : ByteArray) (operand : ByteArray) : Option (ByteArray) := (if LexLeanKappa.Axis.isLabelOn (axis) (operand) then LexLeanKappa.Axis.labelOn (axis) ((LexLeanRuntime.append (LexLeanKappa.Axis.prefixOf (axis)) (sortedQuarters (operand) ((LexLeanRuntime.length (LexLeanKappa.Axis.prefixOf (axis)) : Nat)) ((LexLeanRuntime.quotient (LexLeanKappa.Axis.width (axis)) (4) (0) : Nat))) : ByteArray)) else Option.none)

@[expose] public def composeE8On (axis : ByteArray) (operand : ByteArray) : Option (ByteArray) := (if LexLeanKappa.Axis.isLabelOn (axis) (operand) then LexLeanKappa.Axis.labelOn (axis) (operand) else Option.none)

@[expose] public def composeG2 (left : ByteArray) (right : ByteArray) : Option (ByteArray) := composeG2On (LexLeanKappa.Label.text ("sha256")) (left) (right)

@[expose] public def composeF4 (operand : ByteArray) : Option (ByteArray) := composeF4On (LexLeanKappa.Label.text ("sha256")) (operand)

@[expose] public def composeE6 (operand : ByteArray) : Option (ByteArray) := composeE6On (LexLeanKappa.Label.text ("sha256")) (operand)

@[expose] public def composeE7 (operand : ByteArray) : Option (ByteArray) := composeE7On (LexLeanKappa.Label.text ("sha256")) (operand)

@[expose] public def composeE8 (operand : ByteArray) : Option (ByteArray) := composeE8On (LexLeanKappa.Label.text ("sha256")) (operand)

end LexLeanKappa.Compose
