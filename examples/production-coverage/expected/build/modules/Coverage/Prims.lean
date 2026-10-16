module
public import Init
public import LexLeanRuntime.Portable
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Coverage.Prims

@[expose] public def fixedSmall (a : Int8) (b : Int8) : (Prod (Option (Int8)) ((Prod (Option (Int8)) ((Prod (Option (Int8)) ((Prod (Option (Int8)) (Option (Int8))))))))) := ((LexLeanRuntime.checkedAdd (a) (b) : Option (Int8)), ((LexLeanRuntime.checkedSubtract (a) (b) : Option (Int8)), ((LexLeanRuntime.checkedMultiply (a) (b) : Option (Int8)), ((LexLeanRuntime.checkedQuotient (a) (b) : Option (Int8)), (LexLeanRuntime.checkedNegate (a) : Option (Int8))))))

@[expose] public def fixedMiddle (a : Int16) (b : Int32) : (Prod (Option (Int16)) ((Prod (Option (Int32)) ((Prod (Option (Int32)) (Bool)))))) := ((LexLeanRuntime.checkedMultiply (a) (a) : Option (Int16)), ((LexLeanRuntime.checkedAdd (b) ((-7 : Int32)) : Option (Int32)), ((LexLeanRuntime.checkedConvert (a) : Option (Int32)), (LexLeanRuntime.equal (b) ((3 : Int32)) : Bool))))

@[expose] public def fixedWide (a : Int64) (b : Int64) (c : UInt64) : (Prod (Option (Int64)) ((Prod (Option (Int64)) ((Prod (Option (Int64)) ((Prod (Option (Int64)) ((Prod (Option (Int64)) (Option (UInt64))))))))))) := ((LexLeanRuntime.checkedAddInt64 (a) (b) : Option (Int64)), ((LexLeanRuntime.checkedSubtractInt64 (a) (b) : Option (Int64)), ((LexLeanRuntime.checkedMultiplyInt64 (a) (b) : Option (Int64)), ((LexLeanRuntime.checkedQuotientInt64 (a) (b) : Option (Int64)), ((LexLeanRuntime.checkedNegateInt64 (a) : Option (Int64)), (LexLeanRuntime.checkedAdd (c) ((1 : UInt64)) : Option (UInt64)))))))

@[expose] public def bits (a : UInt16) (b : UInt16) (s : UInt32) : (Prod (UInt16) ((Prod (UInt16) ((Prod (UInt16) ((Prod (UInt16) ((Prod (Option (UInt16)) ((Prod (Option (UInt16)) (Bool)))))))))))) := ((LexLeanRuntime.bitAnd (a) (b) : UInt16), ((LexLeanRuntime.bitOr (a) (b) : UInt16), ((LexLeanRuntime.bitXor (a) (b) : UInt16), ((LexLeanRuntime.bitNot (a) : UInt16), ((LexLeanRuntime.shiftLeft (a) (s) : Option (UInt16)), ((LexLeanRuntime.shiftRight (a) ((3 : UInt32)) : Option (UInt16)), (LexLeanRuntime.equal (a) (b) : Bool)))))))

@[expose] public def conversions (a : Int) (b : Int8) : (Prod (Option (UInt16)) (Option (Int64))) := ((LexLeanRuntime.checkedConvert (a) : Option (UInt16)), (LexLeanRuntime.checkedConvert (b) : Option (Int64)))

@[expose] public def sequences (xs : List (Nat)) (b : ByteArray) (i : Nat) : (Prod (List (Nat)) ((Prod (ByteArray) ((Prod (Nat) ((Prod (Option (Nat)) ((Prod (Option (UInt8)) ((Prod (Option (List (Nat))) ((Prod (Option (ByteArray)) ((Prod (Bool) (Bool)))))))))))))))) := ((LexLeanRuntime.append (xs) ((1 :: ([] : List (Nat)))) : List (Nat)), ((LexLeanRuntime.append (b) (ByteArray.mk #[10]) : ByteArray), (((LexLeanRuntime.length (xs) : Nat) + (LexLeanRuntime.length (b) : Nat)), ((LexLeanRuntime.index (xs) (i) : Option (Nat)), ((LexLeanRuntime.index (b) (i) : Option (UInt8)), ((LexLeanRuntime.slice (xs) (i) (2) : Option (List (Nat))), ((LexLeanRuntime.slice (b) (0) (i) : Option (ByteArray)), ((LexLeanRuntime.equal (b) (ByteArray.mk #[]) : Bool), (LexLeanRuntime.equal ((LexLeanRuntime.equal (i) (3) : Bool)) (true) : Bool)))))))))

@[expose] public def decimals (s : String) : (Prod (Option (Int)) (Option (UInt32))) := ((LexLeanRuntime.parseDecimal (s) : Option (Int)), (LexLeanRuntime.parseDecimal (s) : Option (UInt32)))

@[expose] public def formats (a : Int) (b : Int8) : (Prod (String) (String)) := ((LexLeanRuntime.formatDecimal (a) : String), (LexLeanRuntime.formatDecimal (b) : String))

@[expose] public def text (s : String) (d : String) : (Prod (ByteArray) ((Prod (Option (String)) ((Prod (Ordering) ((Prod (Option (List (String))) ((Prod (String) ((Prod (Nat) (Bool)))))))))))) := (let raw : ByteArray := (LexLeanRuntime.utf8Encode (s) : ByteArray); (raw, ((LexLeanRuntime.utf8Decode (raw) : Option (String)), ((LexLeanRuntime.compareBytes (raw) (ByteArray.mk #[0, 255]) : Ordering), ((LexLeanRuntime.splitExact (s) (d) ((4 : UInt32)) : Option (List (String))), ((LexLeanRuntime.join ((s :: ("x\"y" :: ([] : List (String))))) (d) : String), ((LexLeanRuntime.length (s) : Nat), ((LexLeanRuntime.equal (s) (d) : Bool) && (LexLeanRuntime.equal ((LexLeanRuntime.compareBytes (raw) (raw) : Ordering)) ((LexLeanRuntime.compareBytes (ByteArray.mk #[]) (ByteArray.mk #[1]) : Ordering)) : Bool)))))))))

end Coverage.Prims
