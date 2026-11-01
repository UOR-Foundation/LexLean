module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanKappa.Json
public import LexLeanKappa.Label
public import LexLeanStd.Text
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanKappa.Compose

@[expose] public def hexChar (n : Nat) : ByteArray := LexLeanKappa.Json.byteOf ((if (Nat.blt (n) (10)) then (48 + n) else (87 + n)))

@[expose] public def digestOf (operand : ByteArray) : ByteArray := LexLeanStd.Text.sliceOr (operand) (7) (64)

@[expose] public def mirrorStep (digest : ByteArray) (state : (Prod (Nat) (ByteArray))) : Option ((Prod (Nat) (ByteArray))) := (if (Nat.blt ((state).1) (64)) then Option.some ((((state).1 + 1), (LexLeanRuntime.append ((state).2) (hexChar ((LexLeanRuntime.subtract (15) (LexLeanKappa.Json.hexValue (LexLeanKappa.Label.byteNat (digest) ((state).1))) : Nat))) : ByteArray))) else Option.none)

@[expose] public def mirror (operand : ByteArray) : ByteArray := (let digest : ByteArray := digestOf (operand); (((LexLeanCollections.iterateUntil ((fun (state : (Prod (Nat) (ByteArray))) => mirrorStep (digest) (state))) (65) ((0, _root_.ByteArray.mk #[])) : (Prod ((Prod (Nat) (ByteArray))) (Bool)))).1).2)

@[expose] public def lower (left : ByteArray) (right : ByteArray) : ByteArray := (if LexLeanKappa.Json.lessKey (right) (left) then right else left)

@[expose] public def upper (left : ByteArray) (right : ByteArray) : ByteArray := (if LexLeanKappa.Json.lessKey (right) (left) then left else right)

@[expose] public def sortedQuarters (operand : ByteArray) : ByteArray := (let a0 : ByteArray := LexLeanStd.Text.sliceOr (operand) (7) (16); (let b0 : ByteArray := LexLeanStd.Text.sliceOr (operand) (23) (16); (let c0 : ByteArray := LexLeanStd.Text.sliceOr (operand) (39) (16); (let d0 : ByteArray := LexLeanStd.Text.sliceOr (operand) (55) (16); (let a1 : ByteArray := lower (a0) (b0); (let b1 : ByteArray := upper (a0) (b0); (let c1 : ByteArray := lower (c0) (d0); (let d1 : ByteArray := upper (c0) (d0); (let a2 : ByteArray := lower (a1) (c1); (let c2 : ByteArray := upper (a1) (c1); (let b2 : ByteArray := lower (b1) (d1); (let d2 : ByteArray := upper (b1) (d1); (let b3 : ByteArray := lower (b2) (c2); (let c3 : ByteArray := upper (b2) (c2); (LexLeanRuntime.append (a2) ((LexLeanRuntime.append (b3) ((LexLeanRuntime.append (c3) (d2) : ByteArray)) : ByteArray)) : ByteArray)))))))))))))))

@[expose] public def degreeTag (operand : ByteArray) : ByteArray := (let high : Nat := LexLeanKappa.Json.hexValue (LexLeanKappa.Label.byteNat (operand) (7)); (let low : Nat := LexLeanKappa.Json.hexValue (LexLeanKappa.Label.byteNat (operand) (8)); (if (Nat.beq ((LexLeanRuntime.remainder (((LexLeanRuntime.multiply (high) (16) : Nat) + low)) (9) (0) : Nat)) (8)) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 6))] else _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 5))])))

@[expose] public def composeG2 (left : ByteArray) (right : ByteArray) : Option (ByteArray) := (if LexLeanKappa.Label.isLabel (left) then (if LexLeanKappa.Label.isLabel (right) then Option.some (LexLeanKappa.Label.kappaLabel ((LexLeanRuntime.append (lower (left) (right)) (upper (left) (right)) : ByteArray))) else Option.none) else Option.none)

@[expose] public def composeF4 (operand : ByteArray) : Option (ByteArray) := (if LexLeanKappa.Label.isLabel (operand) then Option.some (LexLeanKappa.Label.kappaLabel ((LexLeanRuntime.append (LexLeanKappa.Label.text ("sha256:")) (lower (digestOf (operand)) (mirror (operand))) : ByteArray))) else Option.none)

@[expose] public def composeE6 (operand : ByteArray) : Option (ByteArray) := (if LexLeanKappa.Label.isLabel (operand) then Option.some (LexLeanKappa.Label.kappaLabel ((LexLeanRuntime.append (degreeTag (operand)) (operand) : ByteArray))) else Option.none)

@[expose] public def composeE7 (operand : ByteArray) : Option (ByteArray) := (if LexLeanKappa.Label.isLabel (operand) then Option.some (LexLeanKappa.Label.kappaLabel ((LexLeanRuntime.append (LexLeanKappa.Label.text ("sha256:")) (sortedQuarters (operand)) : ByteArray))) else Option.none)

@[expose] public def composeE8 (operand : ByteArray) : Option (ByteArray) := (if LexLeanKappa.Label.isLabel (operand) then Option.some (LexLeanKappa.Label.kappaLabel (operand)) else Option.none)

end LexLeanKappa.Compose
