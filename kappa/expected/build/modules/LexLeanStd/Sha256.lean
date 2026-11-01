module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanStd.Sha256

public structure Digest where
  h0 : UInt32
  h1 : UInt32
  h2 : UInt32
  h3 : UInt32
  h4 : UInt32
  h5 : UInt32
  h6 : UInt32
  h7 : UInt32

public structure Schedule where
  w0 : UInt32
  w1 : UInt32
  w2 : UInt32
  w3 : UInt32
  w4 : UInt32
  w5 : UInt32
  w6 : UInt32
  w7 : UInt32
  w8 : UInt32
  w9 : UInt32
  w10 : UInt32
  w11 : UInt32
  w12 : UInt32
  w13 : UInt32
  w14 : UInt32
  w15 : UInt32

public structure Working where
  a : UInt32
  b : UInt32
  c : UInt32
  d : UInt32
  e : UInt32
  f : UInt32
  g : UInt32
  h : UInt32
  window : Schedule
  index : Nat

@[expose] public def widen (value : UInt32) : UInt64 := (match (LexLeanRuntime.checkedConvert (value) : Option (UInt64)) with | Option.some wide => wide | Option.none => (0 : UInt64))

@[expose] public def narrow (value : UInt64) : UInt32 := (match (LexLeanRuntime.checkedConvert ((LexLeanRuntime.bitAnd (value) ((4294967295 : UInt64)) : UInt64)) : Option (UInt32)) with | Option.some low => low | Option.none => (0 : UInt32))

@[expose] public def add (left : UInt32) (right : UInt32) : UInt32 := (match (LexLeanRuntime.checkedAdd (widen (left)) (widen (right)) : Option (UInt64)) with | Option.some sum => narrow (sum) | Option.none => (0 : UInt32))

@[expose] public def shiftRight (value : UInt32) (amount : UInt32) : UInt32 := (match (LexLeanRuntime.shiftRight (value) (amount) : Option (UInt32)) with | Option.some shifted => shifted | Option.none => (0 : UInt32))

@[expose] public def shiftLeft (value : UInt32) (amount : UInt32) : UInt32 := (match (LexLeanRuntime.shiftLeft (value) (amount) : Option (UInt32)) with | Option.some shifted => shifted | Option.none => (0 : UInt32))

@[expose] public def rotateRight (value : UInt32) (amount : UInt32) (complement : UInt32) : UInt32 := (LexLeanRuntime.bitOr (shiftRight (value) (amount)) (shiftLeft (value) (complement)) : UInt32)

@[expose] public def choose (e : UInt32) (f : UInt32) (g : UInt32) : UInt32 := (LexLeanRuntime.bitXor ((LexLeanRuntime.bitAnd (e) (f) : UInt32)) ((LexLeanRuntime.bitAnd ((LexLeanRuntime.bitNot (e) : UInt32)) (g) : UInt32)) : UInt32)

@[expose] public def majority (a : UInt32) (b : UInt32) (c : UInt32) : UInt32 := (LexLeanRuntime.bitXor ((LexLeanRuntime.bitXor ((LexLeanRuntime.bitAnd (a) (b) : UInt32)) ((LexLeanRuntime.bitAnd (a) (c) : UInt32)) : UInt32)) ((LexLeanRuntime.bitAnd (b) (c) : UInt32)) : UInt32)

@[expose] public def bigSigma0 (value : UInt32) : UInt32 := (LexLeanRuntime.bitXor ((LexLeanRuntime.bitXor (rotateRight (value) ((2 : UInt32)) ((30 : UInt32))) (rotateRight (value) ((13 : UInt32)) ((19 : UInt32))) : UInt32)) (rotateRight (value) ((22 : UInt32)) ((10 : UInt32))) : UInt32)

@[expose] public def bigSigma1 (value : UInt32) : UInt32 := (LexLeanRuntime.bitXor ((LexLeanRuntime.bitXor (rotateRight (value) ((6 : UInt32)) ((26 : UInt32))) (rotateRight (value) ((11 : UInt32)) ((21 : UInt32))) : UInt32)) (rotateRight (value) ((25 : UInt32)) ((7 : UInt32))) : UInt32)

@[expose] public def smallSigma0 (value : UInt32) : UInt32 := (LexLeanRuntime.bitXor ((LexLeanRuntime.bitXor (rotateRight (value) ((7 : UInt32)) ((25 : UInt32))) (rotateRight (value) ((18 : UInt32)) ((14 : UInt32))) : UInt32)) (shiftRight (value) ((3 : UInt32))) : UInt32)

@[expose] public def smallSigma1 (value : UInt32) : UInt32 := (LexLeanRuntime.bitXor ((LexLeanRuntime.bitXor (rotateRight (value) ((17 : UInt32)) ((15 : UInt32))) (rotateRight (value) ((19 : UInt32)) ((13 : UInt32))) : UInt32)) (shiftRight (value) ((10 : UInt32))) : UInt32)

@[expose] public def advance (window : Schedule) : Schedule := ({ w0 := (window).w1, w1 := (window).w2, w2 := (window).w3, w3 := (window).w4, w4 := (window).w5, w5 := (window).w6, w6 := (window).w7, w7 := (window).w8, w8 := (window).w9, w9 := (window).w10, w10 := (window).w11, w11 := (window).w12, w12 := (window).w13, w13 := (window).w14, w14 := (window).w15, w15 := add (smallSigma1 ((window).w14)) (add ((window).w9) (add (smallSigma0 ((window).w1)) ((window).w0))) } : Schedule)

@[expose] public def roundConstant (position : Nat) : UInt32 := (if (Nat.blt (position) (32)) then (if (Nat.blt (position) (16)) then (if (Nat.blt (position) (8)) then (if (Nat.blt (position) (4)) then (if (Nat.blt (position) (2)) then (if (Nat.blt (position) (1)) then (1116352408 : UInt32) else (1899447441 : UInt32)) else (if (Nat.blt (position) (3)) then (3049323471 : UInt32) else (3921009573 : UInt32))) else (if (Nat.blt (position) (6)) then (if (Nat.blt (position) (5)) then (961987163 : UInt32) else (1508970993 : UInt32)) else (if (Nat.blt (position) (7)) then (2453635748 : UInt32) else (2870763221 : UInt32)))) else (if (Nat.blt (position) (12)) then (if (Nat.blt (position) (10)) then (if (Nat.blt (position) (9)) then (3624381080 : UInt32) else (310598401 : UInt32)) else (if (Nat.blt (position) (11)) then (607225278 : UInt32) else (1426881987 : UInt32))) else (if (Nat.blt (position) (14)) then (if (Nat.blt (position) (13)) then (1925078388 : UInt32) else (2162078206 : UInt32)) else (if (Nat.blt (position) (15)) then (2614888103 : UInt32) else (3248222580 : UInt32))))) else (if (Nat.blt (position) (24)) then (if (Nat.blt (position) (20)) then (if (Nat.blt (position) (18)) then (if (Nat.blt (position) (17)) then (3835390401 : UInt32) else (4022224774 : UInt32)) else (if (Nat.blt (position) (19)) then (264347078 : UInt32) else (604807628 : UInt32))) else (if (Nat.blt (position) (22)) then (if (Nat.blt (position) (21)) then (770255983 : UInt32) else (1249150122 : UInt32)) else (if (Nat.blt (position) (23)) then (1555081692 : UInt32) else (1996064986 : UInt32)))) else (if (Nat.blt (position) (28)) then (if (Nat.blt (position) (26)) then (if (Nat.blt (position) (25)) then (2554220882 : UInt32) else (2821834349 : UInt32)) else (if (Nat.blt (position) (27)) then (2952996808 : UInt32) else (3210313671 : UInt32))) else (if (Nat.blt (position) (30)) then (if (Nat.blt (position) (29)) then (3336571891 : UInt32) else (3584528711 : UInt32)) else (if (Nat.blt (position) (31)) then (113926993 : UInt32) else (338241895 : UInt32)))))) else (if (Nat.blt (position) (48)) then (if (Nat.blt (position) (40)) then (if (Nat.blt (position) (36)) then (if (Nat.blt (position) (34)) then (if (Nat.blt (position) (33)) then (666307205 : UInt32) else (773529912 : UInt32)) else (if (Nat.blt (position) (35)) then (1294757372 : UInt32) else (1396182291 : UInt32))) else (if (Nat.blt (position) (38)) then (if (Nat.blt (position) (37)) then (1695183700 : UInt32) else (1986661051 : UInt32)) else (if (Nat.blt (position) (39)) then (2177026350 : UInt32) else (2456956037 : UInt32)))) else (if (Nat.blt (position) (44)) then (if (Nat.blt (position) (42)) then (if (Nat.blt (position) (41)) then (2730485921 : UInt32) else (2820302411 : UInt32)) else (if (Nat.blt (position) (43)) then (3259730800 : UInt32) else (3345764771 : UInt32))) else (if (Nat.blt (position) (46)) then (if (Nat.blt (position) (45)) then (3516065817 : UInt32) else (3600352804 : UInt32)) else (if (Nat.blt (position) (47)) then (4094571909 : UInt32) else (275423344 : UInt32))))) else (if (Nat.blt (position) (56)) then (if (Nat.blt (position) (52)) then (if (Nat.blt (position) (50)) then (if (Nat.blt (position) (49)) then (430227734 : UInt32) else (506948616 : UInt32)) else (if (Nat.blt (position) (51)) then (659060556 : UInt32) else (883997877 : UInt32))) else (if (Nat.blt (position) (54)) then (if (Nat.blt (position) (53)) then (958139571 : UInt32) else (1322822218 : UInt32)) else (if (Nat.blt (position) (55)) then (1537002063 : UInt32) else (1747873779 : UInt32)))) else (if (Nat.blt (position) (60)) then (if (Nat.blt (position) (58)) then (if (Nat.blt (position) (57)) then (1955562222 : UInt32) else (2024104815 : UInt32)) else (if (Nat.blt (position) (59)) then (2227730452 : UInt32) else (2361852424 : UInt32))) else (if (Nat.blt (position) (62)) then (if (Nat.blt (position) (61)) then (2428436474 : UInt32) else (2756734187 : UInt32)) else (if (Nat.blt (position) (63)) then (3204031479 : UInt32) else (3329325298 : UInt32)))))))

@[expose] public def round (state : Working) : Working := (let constant : UInt32 := roundConstant ((state).index); (let t1 : UInt32 := add ((state).h) (add (bigSigma1 ((state).e)) (add (choose ((state).e) ((state).f) ((state).g)) (add (constant) (((state).window).w0)))); (let t2 : UInt32 := add (bigSigma0 ((state).a)) (majority ((state).a) ((state).b) ((state).c)); ({ a := add (t1) (t2), b := (state).a, c := (state).b, d := (state).c, e := add ((state).d) (t1), f := (state).e, g := (state).f, h := (state).g, window := advance ((state).window), index := ((state).index + 1) } : Working))))

@[expose] public def initial : Digest := ({ h0 := (1779033703 : UInt32), h1 := (3144134277 : UInt32), h2 := (1013904242 : UInt32), h3 := (2773480762 : UInt32), h4 := (1359893119 : UInt32), h5 := (2600822924 : UInt32), h6 := (528734635 : UInt32), h7 := (1541459225 : UInt32) } : Digest)

@[expose] public def compress (hash : Digest) (block : Schedule) : Digest := (let final : Working := (LexLeanCollections.iterate ((fun (state : Working) => round (state))) (64) (({ a := (hash).h0, b := (hash).h1, c := (hash).h2, d := (hash).h3, e := (hash).h4, f := (hash).h5, g := (hash).h6, h := (hash).h7, window := block, index := 0 } : Working)) : Working); ({ h0 := add ((hash).h0) ((final).a), h1 := add ((hash).h1) ((final).b), h2 := add ((hash).h2) ((final).c), h3 := add ((hash).h3) ((final).d), h4 := add ((hash).h4) ((final).e), h5 := add ((hash).h5) ((final).f), h6 := add ((hash).h6) ((final).g), h7 := add ((hash).h7) ((final).h) } : Digest))

@[expose] public def byteAt (message : ByteArray) (size : Nat) (position : Nat) : UInt32 := (if (Nat.blt (position) (size)) then (match (LexLeanRuntime.index (message) (position) : Option (UInt8)) with | Option.some byte => (match (LexLeanRuntime.checkedConvert (byte) : Option (UInt32)) with | Option.some wide => wide | Option.none => (0 : UInt32)) | Option.none => (0 : UInt32)) else (if (Nat.beq (position) (size)) then (128 : UInt32) else (0 : UInt32)))

@[expose] public def wordAt (b0 : UInt32) (b1 : UInt32) (b2 : UInt32) (b3 : UInt32) : UInt32 := (LexLeanRuntime.bitOr (shiftLeft (b0) ((24 : UInt32))) ((LexLeanRuntime.bitOr (shiftLeft (b1) ((16 : UInt32))) ((LexLeanRuntime.bitOr (shiftLeft (b2) ((8 : UInt32))) (b3) : UInt32)) : UInt32)) : UInt32)

@[expose] public def wordOfNat (value : Nat) : UInt32 := (((LexLeanCollections.iterate ((fun (state : (Prod (Nat) ((Prod (UInt32) (UInt32))))) => (let remaining : Nat := (state).1; (let mask : UInt32 := ((state).2).1; (let bits : UInt32 := ((state).2).2; ((LexLeanRuntime.quotient (remaining) (2) (0) : Nat), (shiftLeft (mask) ((1 : UInt32)), (if (Nat.beq ((LexLeanRuntime.remainder (remaining) (2) (0) : Nat)) (1)) then (LexLeanRuntime.bitOr (bits) (mask) : UInt32) else bits)))))))) (32) ((value, ((1 : UInt32), (0 : UInt32)))) : (Prod (Nat) ((Prod (UInt32) (UInt32)))))).2).2

@[expose] public def scheduleWord (message : ByteArray) (size : Nat) (total : Nat) (position : Nat) : UInt32 := (if (Nat.beq ((position + 8)) (total)) then wordOfNat ((LexLeanRuntime.quotient ((LexLeanRuntime.multiply (size) (8) : Nat)) (4294967296) (0) : Nat)) else (if (Nat.beq ((position + 4)) (total)) then wordOfNat ((LexLeanRuntime.remainder ((LexLeanRuntime.multiply (size) (8) : Nat)) (4294967296) (0) : Nat)) else wordAt (byteAt (message) (size) (position)) (byteAt (message) (size) ((position + 1))) (byteAt (message) (size) ((position + 2))) (byteAt (message) (size) ((position + 3)))))

@[expose] public def load (message : ByteArray) (size : Nat) (total : Nat) (offset : Nat) : Schedule := ({ w0 := scheduleWord (message) (size) (total) (offset), w1 := scheduleWord (message) (size) (total) ((offset + 4)), w2 := scheduleWord (message) (size) (total) ((offset + 8)), w3 := scheduleWord (message) (size) (total) ((offset + 12)), w4 := scheduleWord (message) (size) (total) ((offset + 16)), w5 := scheduleWord (message) (size) (total) ((offset + 20)), w6 := scheduleWord (message) (size) (total) ((offset + 24)), w7 := scheduleWord (message) (size) (total) ((offset + 28)), w8 := scheduleWord (message) (size) (total) ((offset + 32)), w9 := scheduleWord (message) (size) (total) ((offset + 36)), w10 := scheduleWord (message) (size) (total) ((offset + 40)), w11 := scheduleWord (message) (size) (total) ((offset + 44)), w12 := scheduleWord (message) (size) (total) ((offset + 48)), w13 := scheduleWord (message) (size) (total) ((offset + 52)), w14 := scheduleWord (message) (size) (total) ((offset + 56)), w15 := scheduleWord (message) (size) (total) ((offset + 60)) } : Schedule)

@[expose] public def sha256 (message : ByteArray) : Digest := (let size : Nat := (LexLeanRuntime.length (message) : Nat); (let total : Nat := (LexLeanRuntime.multiply ((LexLeanRuntime.quotient ((size + 72)) (64) (0) : Nat)) (64) : Nat); ((LexLeanCollections.iterate ((fun (state : (Prod (Nat) (Digest))) => (((state).1 + 64), compress ((state).2) (load (message) (size) (total) ((state).1))))) ((LexLeanRuntime.quotient (total) (64) (0) : Nat)) ((0, initial)) : (Prod (Nat) (Digest)))).2))

@[expose] public def hexDigit (nibble : UInt32) : ByteArray := (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((8 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((4 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((2 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((1 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 48))] else _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 49))]) else (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((1 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 50))] else _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 51))])) else (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((2 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((1 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 52))] else _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 53))]) else (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((1 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 54))] else _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 55))]))) else (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((4 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((2 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((1 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 56))] else _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 57))]) else (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((1 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 97))] else _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 98))])) else (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((2 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((1 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 99))] else _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 100))]) else (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((1 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 101))] else _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 102))]))))

@[expose] public def hexWord (value : UInt32) : ByteArray := (LexLeanRuntime.append (hexDigit ((LexLeanRuntime.bitAnd (shiftRight (value) ((28 : UInt32))) ((15 : UInt32)) : UInt32))) ((LexLeanRuntime.append (hexDigit ((LexLeanRuntime.bitAnd (shiftRight (value) ((24 : UInt32))) ((15 : UInt32)) : UInt32))) ((LexLeanRuntime.append (hexDigit ((LexLeanRuntime.bitAnd (shiftRight (value) ((20 : UInt32))) ((15 : UInt32)) : UInt32))) ((LexLeanRuntime.append (hexDigit ((LexLeanRuntime.bitAnd (shiftRight (value) ((16 : UInt32))) ((15 : UInt32)) : UInt32))) ((LexLeanRuntime.append (hexDigit ((LexLeanRuntime.bitAnd (shiftRight (value) ((12 : UInt32))) ((15 : UInt32)) : UInt32))) ((LexLeanRuntime.append (hexDigit ((LexLeanRuntime.bitAnd (shiftRight (value) ((8 : UInt32))) ((15 : UInt32)) : UInt32))) ((LexLeanRuntime.append (hexDigit ((LexLeanRuntime.bitAnd (shiftRight (value) ((4 : UInt32))) ((15 : UInt32)) : UInt32))) (hexDigit ((LexLeanRuntime.bitAnd (value) ((15 : UInt32)) : UInt32))) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)

@[expose] public def hexDigest (digest : Digest) : ByteArray := (LexLeanRuntime.append (hexWord ((digest).h0)) ((LexLeanRuntime.append (hexWord ((digest).h1)) ((LexLeanRuntime.append (hexWord ((digest).h2)) ((LexLeanRuntime.append (hexWord ((digest).h3)) ((LexLeanRuntime.append (hexWord ((digest).h4)) ((LexLeanRuntime.append (hexWord ((digest).h5)) ((LexLeanRuntime.append (hexWord ((digest).h6)) (hexWord ((digest).h7)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)

@[expose] public def sha256Hex (message : ByteArray) : ByteArray := hexDigest (sha256 (message))

public theorem fips_one_block : (sha256 (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 97)), (_root_.UInt8.ofNat (nat_lit 98)), (_root_.UInt8.ofNat (nat_lit 99))]) = ({ h0 := (3128432319 : UInt32), h1 := (2399260650 : UInt32), h2 := (1094795486 : UInt32), h3 := (1571693091 : UInt32), h4 := (2953011619 : UInt32), h5 := (2518121116 : UInt32), h6 := (3021012833 : UInt32), h7 := (4060091821 : UInt32) } : Digest)) := by
  rfl

public theorem fips_two_blocks : (sha256 ((let __byte100 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 100); let __byte101 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 101); let __byte102 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 102); let __byte103 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 103); let __byte104 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 104); let __byte105 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 105); let __byte106 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 106); let __byte107 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 107); let __byte108 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 108); let __byte109 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 109); let __byte110 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 110); _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 97)), (_root_.UInt8.ofNat (nat_lit 98)), (_root_.UInt8.ofNat (nat_lit 99)), __byte100, (_root_.UInt8.ofNat (nat_lit 98)), (_root_.UInt8.ofNat (nat_lit 99)), __byte100, __byte101, (_root_.UInt8.ofNat (nat_lit 99)), __byte100, __byte101, __byte102, __byte100, __byte101, __byte102, __byte103, __byte101, __byte102, __byte103, __byte104, __byte102, __byte103, __byte104, __byte105, __byte103, __byte104, __byte105, __byte106, __byte104, __byte105, __byte106, __byte107, __byte105, __byte106, __byte107, __byte108, __byte106, __byte107, __byte108, __byte109, __byte107, __byte108, __byte109, __byte110, __byte108, __byte109, __byte110, (_root_.UInt8.ofNat (nat_lit 111)), __byte109, __byte110, (_root_.UInt8.ofNat (nat_lit 111)), (_root_.UInt8.ofNat (nat_lit 112)), __byte110, (_root_.UInt8.ofNat (nat_lit 111)), (_root_.UInt8.ofNat (nat_lit 112)), (_root_.UInt8.ofNat (nat_lit 113))])) = ({ h0 := (613247585 : UInt32), h1 := (3523623096 : UInt32), h2 := (3854575251 : UInt32), h3 := (205414457 : UInt32), h4 := (2738676825 : UInt32), h5 := (1694441831 : UInt32), h6 := (4142722516 : UInt32), h7 := (433784513 : UInt32) } : Digest)) := by
  rfl

public theorem fips_empty : (sha256 (_root_.ByteArray.mk #[]) = ({ h0 := (3820012610 : UInt32), h1 := (2566659092 : UInt32), h2 := (2600203464 : UInt32), h3 := (2574235940 : UInt32), h4 := (665731556 : UInt32), h5 := (1687917388 : UInt32), h6 := (2761267483 : UInt32), h7 := (2018687061 : UInt32) } : Digest)) := by
  rfl

end LexLeanStd.Sha256
