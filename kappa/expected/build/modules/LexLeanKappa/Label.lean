module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Sha256
public import LexLeanStd.Text
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanKappa.Label

public structure Handle where
  kind : ByteArray
  label : ByteArray

@[expose] public def text (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def byteNat (subject : ByteArray) (position : Nat) : Nat := (match (LexLeanRuntime.index (subject) (position) : Option (UInt8)) with | Option.some byte => (let __widen0 : UInt8 := byte; ((((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((128 : UInt8)) : UInt8)) ((128 : UInt8)) : Bool) then 128 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((64 : UInt8)) : UInt8)) ((64 : UInt8)) : Bool) then 64 else 0)) + ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((32 : UInt8)) : UInt8)) ((32 : UInt8)) : Bool) then 32 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((16 : UInt8)) : UInt8)) ((16 : UInt8)) : Bool) then 16 else 0))) + (((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((8 : UInt8)) : UInt8)) ((8 : UInt8)) : Bool) then 8 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((4 : UInt8)) : UInt8)) ((4 : UInt8)) : Bool) then 4 else 0)) + ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((2 : UInt8)) : UInt8)) ((2 : UInt8)) : Bool) then 2 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((1 : UInt8)) : UInt8)) ((1 : UInt8)) : Bool) then 1 else 0))))) | Option.none => 256)

@[expose] public def kappaLabel (canonical : ByteArray) : ByteArray := (LexLeanRuntime.append (text ("sha256:")) (LexLeanStd.Sha256.sha256Hex (canonical)) : ByteArray)

@[expose] public def lowerHexAt (subject : ByteArray) (position : Nat) : Bool := (let n : Nat := byteNat (subject) (position); (if (Nat.blt (47) (n)) then (if (Nat.blt (n) (58)) then true else (if (Nat.blt (96) (n)) then (Nat.blt (n) (103)) else false)) else false))

@[expose] public def lowerHexRun (subject : ByteArray) («from» : Nat) : Nat := ((LexLeanCollections.iterateUntil ((fun (position : Nat) => (if lowerHexAt (subject) (position) then Option.some ((position + 1)) else Option.none))) ((LexLeanRuntime.length (subject) : Nat)) («from») : (Prod (Nat) (Bool)))).1

@[expose] public def isLabel (candidate : ByteArray) : Bool := (if (Nat.beq ((LexLeanRuntime.length (candidate) : Nat)) (71)) then (if LexLeanStd.Text.startsWith (candidate) (text ("sha256:")) then (Nat.beq (lowerHexRun (candidate) (7)) (71)) else false) else false)

@[expose] public def kindByteAt (subject : ByteArray) (position : Nat) : Bool := (let n : Nat := byteNat (subject) (position); (if (Nat.blt (96) (n)) then (Nat.blt (n) (123)) else (if (Nat.blt (47) (n)) then (Nat.blt (n) (58)) else (Nat.beq (n) (45)))))

@[expose] public def kindRun (subject : ByteArray) : Nat := ((LexLeanCollections.iterateUntil ((fun (position : Nat) => (if kindByteAt (subject) (position) then Option.some ((position + 1)) else Option.none))) ((LexLeanRuntime.length (subject) : Nat)) (0) : (Prod (Nat) (Bool)))).1

@[expose] public def isKind (kind : ByteArray) : Bool := (if (Nat.blt (0) ((LexLeanRuntime.length (kind) : Nat))) then (Nat.beq (kindRun (kind)) ((LexLeanRuntime.length (kind) : Nat))) else false)

@[expose] public def handleOf (kind : ByteArray) (canonical : ByteArray) : Option (Handle) := (if isKind (kind) then Option.some (({ kind := kind, label := kappaLabel (canonical) } : Handle)) else Option.none)

@[expose] public def handleWire (kind : ByteArray) (canonical : ByteArray) : Option (ByteArray) := (match handleOf (kind) (canonical) with | Option.some handle => Option.some ((LexLeanRuntime.append ((handle).kind) ((LexLeanRuntime.append (text ("@")) ((handle).label) : ByteArray)) : ByteArray)) | Option.none => Option.none)

end LexLeanKappa.Label
