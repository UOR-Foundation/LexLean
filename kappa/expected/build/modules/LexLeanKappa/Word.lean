module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Sha256
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanKappa.Word

@[expose] public def shiftRight (value : UInt64) (amount : UInt32) : UInt64 := (match (LexLeanRuntime.shiftRight (value) (amount) : Option (UInt64)) with | Option.some shifted => shifted | Option.none => (0 : UInt64))

@[expose] public def shiftLeft (value : UInt64) (amount : UInt32) : UInt64 := (match (LexLeanRuntime.shiftLeft (value) (amount) : Option (UInt64)) with | Option.some shifted => shifted | Option.none => (0 : UInt64))

@[expose] public def rotateRight (value : UInt64) (amount : UInt32) (complement : UInt32) : UInt64 := (LexLeanRuntime.bitOr (shiftRight (value) (amount)) (shiftLeft (value) (complement)) : UInt64)

@[expose] public def rotateLeft (value : UInt64) (amount : UInt32) (complement : UInt32) : UInt64 := (LexLeanRuntime.bitOr (shiftLeft (value) (amount)) (shiftRight (value) (complement)) : UInt64)

@[expose] public def sum (left : UInt64) (right : UInt64) : UInt64 := (match (LexLeanRuntime.checkedAdd (left) (right) : Option (UInt64)) with | Option.some total => total | Option.none => (0 : UInt64))

@[expose] public def add (left : UInt64) (right : UInt64) : UInt64 := (let mask : UInt64 := (4294967295 : UInt64); (let low : UInt64 := sum ((LexLeanRuntime.bitAnd (left) (mask) : UInt64)) ((LexLeanRuntime.bitAnd (right) (mask) : UInt64)); (let high : UInt64 := sum (sum (shiftRight (left) ((32 : UInt32))) (shiftRight (right) ((32 : UInt32)))) (shiftRight (low) ((32 : UInt32))); (LexLeanRuntime.bitOr (shiftLeft ((LexLeanRuntime.bitAnd (high) (mask) : UInt64)) ((32 : UInt32))) ((LexLeanRuntime.bitAnd (low) (mask) : UInt64)) : UInt64))))

@[expose] public def nibble (value : UInt64) (shift : UInt32) : ByteArray := LexLeanStd.Sha256.hexDigit (LexLeanStd.Sha256.narrow (shiftRight (value) (shift)))

@[expose] public def hexBig (value : UInt64) : ByteArray := (LexLeanRuntime.append (nibble (value) ((60 : UInt32))) ((LexLeanRuntime.append (nibble (value) ((56 : UInt32))) ((LexLeanRuntime.append (nibble (value) ((52 : UInt32))) ((LexLeanRuntime.append (nibble (value) ((48 : UInt32))) ((LexLeanRuntime.append (nibble (value) ((44 : UInt32))) ((LexLeanRuntime.append (nibble (value) ((40 : UInt32))) ((LexLeanRuntime.append (nibble (value) ((36 : UInt32))) ((LexLeanRuntime.append (nibble (value) ((32 : UInt32))) ((LexLeanRuntime.append (nibble (value) ((28 : UInt32))) ((LexLeanRuntime.append (nibble (value) ((24 : UInt32))) ((LexLeanRuntime.append (nibble (value) ((20 : UInt32))) ((LexLeanRuntime.append (nibble (value) ((16 : UInt32))) ((LexLeanRuntime.append (nibble (value) ((12 : UInt32))) ((LexLeanRuntime.append (nibble (value) ((8 : UInt32))) ((LexLeanRuntime.append (nibble (value) ((4 : UInt32))) (nibble (value) ((0 : UInt32))) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)

@[expose] public def hexLittle (value : UInt64) : ByteArray := (LexLeanRuntime.append ((LexLeanRuntime.append (nibble (value) ((4 : UInt32))) (nibble (value) ((0 : UInt32))) : ByteArray)) ((LexLeanRuntime.append ((LexLeanRuntime.append (nibble (value) ((12 : UInt32))) (nibble (value) ((8 : UInt32))) : ByteArray)) ((LexLeanRuntime.append ((LexLeanRuntime.append (nibble (value) ((20 : UInt32))) (nibble (value) ((16 : UInt32))) : ByteArray)) ((LexLeanRuntime.append ((LexLeanRuntime.append (nibble (value) ((28 : UInt32))) (nibble (value) ((24 : UInt32))) : ByteArray)) ((LexLeanRuntime.append ((LexLeanRuntime.append (nibble (value) ((36 : UInt32))) (nibble (value) ((32 : UInt32))) : ByteArray)) ((LexLeanRuntime.append ((LexLeanRuntime.append (nibble (value) ((44 : UInt32))) (nibble (value) ((40 : UInt32))) : ByteArray)) ((LexLeanRuntime.append ((LexLeanRuntime.append (nibble (value) ((52 : UInt32))) (nibble (value) ((48 : UInt32))) : ByteArray)) ((LexLeanRuntime.append (nibble (value) ((60 : UInt32))) (nibble (value) ((56 : UInt32))) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)

@[expose] public def fromBytes (b0 : UInt64) (b1 : UInt64) (b2 : UInt64) (b3 : UInt64) (b4 : UInt64) (b5 : UInt64) (b6 : UInt64) (b7 : UInt64) : UInt64 := (LexLeanRuntime.bitOr (shiftLeft (b0) ((56 : UInt32))) ((LexLeanRuntime.bitOr (shiftLeft (b1) ((48 : UInt32))) ((LexLeanRuntime.bitOr (shiftLeft (b2) ((40 : UInt32))) ((LexLeanRuntime.bitOr (shiftLeft (b3) ((32 : UInt32))) ((LexLeanRuntime.bitOr (shiftLeft (b4) ((24 : UInt32))) ((LexLeanRuntime.bitOr (shiftLeft (b5) ((16 : UInt32))) ((LexLeanRuntime.bitOr (shiftLeft (b6) ((8 : UInt32))) (b7) : UInt64)) : UInt64)) : UInt64)) : UInt64)) : UInt64)) : UInt64)) : UInt64)

@[expose] public def fromBytesLittle (b0 : UInt64) (b1 : UInt64) (b2 : UInt64) (b3 : UInt64) (b4 : UInt64) (b5 : UInt64) (b6 : UInt64) (b7 : UInt64) : UInt64 := (LexLeanRuntime.bitOr (shiftLeft (b7) ((56 : UInt32))) ((LexLeanRuntime.bitOr (shiftLeft (b6) ((48 : UInt32))) ((LexLeanRuntime.bitOr (shiftLeft (b5) ((40 : UInt32))) ((LexLeanRuntime.bitOr (shiftLeft (b4) ((32 : UInt32))) ((LexLeanRuntime.bitOr (shiftLeft (b3) ((24 : UInt32))) ((LexLeanRuntime.bitOr (shiftLeft (b2) ((16 : UInt32))) ((LexLeanRuntime.bitOr (shiftLeft (b1) ((8 : UInt32))) (b0) : UInt64)) : UInt64)) : UInt64)) : UInt64)) : UInt64)) : UInt64)) : UInt64)

@[expose] public def byteWord (message : ByteArray) (position : Nat) : UInt64 := (match (LexLeanRuntime.index (message) (position) : Option (UInt8)) with | Option.some byte => (match (LexLeanRuntime.checkedConvert (byte) : Option (UInt64)) with | Option.some wide => wide | Option.none => (0 : UInt64)) | Option.none => (0 : UInt64))

@[expose] public def ofNat (value : Nat) : UInt64 := (((LexLeanCollections.iterate ((fun (state : (Prod (Nat) ((Prod (UInt64) (UInt64))))) => (let remaining : Nat := (state).1; (let mask : UInt64 := ((state).2).1; (let bits : UInt64 := ((state).2).2; ((LexLeanRuntime.quotient (remaining) (2) (0) : Nat), (shiftLeft (mask) ((1 : UInt32)), (if (Nat.beq ((LexLeanRuntime.remainder (remaining) (2) (0) : Nat)) (1)) then (LexLeanRuntime.bitOr (bits) (mask) : UInt64) else bits)))))))) (64) ((value, ((1 : UInt64), (0 : UInt64)))) : (Prod (Nat) ((Prod (UInt64) (UInt64)))))).2).2

end LexLeanKappa.Word
