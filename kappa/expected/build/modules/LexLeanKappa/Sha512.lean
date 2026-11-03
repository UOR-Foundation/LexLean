module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanKappa.Word
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanKappa.Sha512

public structure Digest where
  h0 : UInt64
  h1 : UInt64
  h2 : UInt64
  h3 : UInt64
  h4 : UInt64
  h5 : UInt64
  h6 : UInt64
  h7 : UInt64

public structure Schedule where
  w0 : UInt64
  w1 : UInt64
  w2 : UInt64
  w3 : UInt64
  w4 : UInt64
  w5 : UInt64
  w6 : UInt64
  w7 : UInt64
  w8 : UInt64
  w9 : UInt64
  w10 : UInt64
  w11 : UInt64
  w12 : UInt64
  w13 : UInt64
  w14 : UInt64
  w15 : UInt64

public structure Working where
  a : UInt64
  b : UInt64
  c : UInt64
  d : UInt64
  e : UInt64
  f : UInt64
  g : UInt64
  h : UInt64
  window : Schedule
  index : Nat

@[expose] public def choose (e : UInt64) (f : UInt64) (g : UInt64) : UInt64 := (LexLeanRuntime.bitXor ((LexLeanRuntime.bitAnd (e) (f) : UInt64)) ((LexLeanRuntime.bitAnd ((LexLeanRuntime.bitNot (e) : UInt64)) (g) : UInt64)) : UInt64)

@[expose] public def majority (a : UInt64) (b : UInt64) (c : UInt64) : UInt64 := (LexLeanRuntime.bitXor ((LexLeanRuntime.bitXor ((LexLeanRuntime.bitAnd (a) (b) : UInt64)) ((LexLeanRuntime.bitAnd (a) (c) : UInt64)) : UInt64)) ((LexLeanRuntime.bitAnd (b) (c) : UInt64)) : UInt64)

@[expose] public def bigSigma0 (value : UInt64) : UInt64 := (LexLeanRuntime.bitXor ((LexLeanRuntime.bitXor (LexLeanKappa.Word.rotateRight (value) ((28 : UInt32)) ((36 : UInt32))) (LexLeanKappa.Word.rotateRight (value) ((34 : UInt32)) ((30 : UInt32))) : UInt64)) (LexLeanKappa.Word.rotateRight (value) ((39 : UInt32)) ((25 : UInt32))) : UInt64)

@[expose] public def bigSigma1 (value : UInt64) : UInt64 := (LexLeanRuntime.bitXor ((LexLeanRuntime.bitXor (LexLeanKappa.Word.rotateRight (value) ((14 : UInt32)) ((50 : UInt32))) (LexLeanKappa.Word.rotateRight (value) ((18 : UInt32)) ((46 : UInt32))) : UInt64)) (LexLeanKappa.Word.rotateRight (value) ((41 : UInt32)) ((23 : UInt32))) : UInt64)

@[expose] public def smallSigma0 (value : UInt64) : UInt64 := (LexLeanRuntime.bitXor ((LexLeanRuntime.bitXor (LexLeanKappa.Word.rotateRight (value) ((1 : UInt32)) ((63 : UInt32))) (LexLeanKappa.Word.rotateRight (value) ((8 : UInt32)) ((56 : UInt32))) : UInt64)) (LexLeanKappa.Word.shiftRight (value) ((7 : UInt32))) : UInt64)

@[expose] public def smallSigma1 (value : UInt64) : UInt64 := (LexLeanRuntime.bitXor ((LexLeanRuntime.bitXor (LexLeanKappa.Word.rotateRight (value) ((19 : UInt32)) ((45 : UInt32))) (LexLeanKappa.Word.rotateRight (value) ((61 : UInt32)) ((3 : UInt32))) : UInt64)) (LexLeanKappa.Word.shiftRight (value) ((6 : UInt32))) : UInt64)

@[expose] public def advance (window : Schedule) : Schedule := ({ w0 := (window).w1, w1 := (window).w2, w2 := (window).w3, w3 := (window).w4, w4 := (window).w5, w5 := (window).w6, w6 := (window).w7, w7 := (window).w8, w8 := (window).w9, w9 := (window).w10, w10 := (window).w11, w11 := (window).w12, w12 := (window).w13, w13 := (window).w14, w14 := (window).w15, w15 := LexLeanKappa.Word.add (smallSigma1 ((window).w14)) (LexLeanKappa.Word.add ((window).w9) (LexLeanKappa.Word.add (smallSigma0 ((window).w1)) ((window).w0))) } : Schedule)

@[expose] public def roundConstant (position : Nat) : UInt64 := (if (Nat.blt (position) (40)) then (if (Nat.blt (position) (20)) then (if (Nat.blt (position) (10)) then (if (Nat.blt (position) (5)) then (if (Nat.blt (position) (2)) then (if (Nat.blt (position) (1)) then (4794697086780616226 : UInt64) else (8158064640168781261 : UInt64)) else (if (Nat.blt (position) (3)) then (13096744586834688815 : UInt64) else (if (Nat.blt (position) (4)) then (16840607885511220156 : UInt64) else (4131703408338449720 : UInt64)))) else (if (Nat.blt (position) (7)) then (if (Nat.blt (position) (6)) then (6480981068601479193 : UInt64) else (10538285296894168987 : UInt64)) else (if (Nat.blt (position) (8)) then (12329834152419229976 : UInt64) else (if (Nat.blt (position) (9)) then (15566598209576043074 : UInt64) else (1334009975649890238 : UInt64))))) else (if (Nat.blt (position) (15)) then (if (Nat.blt (position) (12)) then (if (Nat.blt (position) (11)) then (2608012711638119052 : UInt64) else (6128411473006802146 : UInt64)) else (if (Nat.blt (position) (13)) then (8268148722764581231 : UInt64) else (if (Nat.blt (position) (14)) then (9286055187155687089 : UInt64) else (11230858885718282805 : UInt64)))) else (if (Nat.blt (position) (17)) then (if (Nat.blt (position) (16)) then (13951009754708518548 : UInt64) else (16472876342353939154 : UInt64)) else (if (Nat.blt (position) (18)) then (17275323862435702243 : UInt64) else (if (Nat.blt (position) (19)) then (1135362057144423861 : UInt64) else (2597628984639134821 : UInt64)))))) else (if (Nat.blt (position) (30)) then (if (Nat.blt (position) (25)) then (if (Nat.blt (position) (22)) then (if (Nat.blt (position) (21)) then (3308224258029322869 : UInt64) else (5365058923640841347 : UInt64)) else (if (Nat.blt (position) (23)) then (6679025012923562964 : UInt64) else (if (Nat.blt (position) (24)) then (8573033837759648693 : UInt64) else (10970295158949994411 : UInt64)))) else (if (Nat.blt (position) (27)) then (if (Nat.blt (position) (26)) then (12119686244451234320 : UInt64) else (12683024718118986047 : UInt64)) else (if (Nat.blt (position) (28)) then (13788192230050041572 : UInt64) else (if (Nat.blt (position) (29)) then (14330467153632333762 : UInt64) else (15395433587784984357 : UInt64))))) else (if (Nat.blt (position) (35)) then (if (Nat.blt (position) (32)) then (if (Nat.blt (position) (31)) then (489312712824947311 : UInt64) else (1452737877330783856 : UInt64)) else (if (Nat.blt (position) (33)) then (2861767655752347644 : UInt64) else (if (Nat.blt (position) (34)) then (3322285676063803686 : UInt64) else (5560940570517711597 : UInt64)))) else (if (Nat.blt (position) (37)) then (if (Nat.blt (position) (36)) then (5996557281743188959 : UInt64) else (7280758554555802590 : UInt64)) else (if (Nat.blt (position) (38)) then (8532644243296465576 : UInt64) else (if (Nat.blt (position) (39)) then (9350256976987008742 : UInt64) else (10552545826968843579 : UInt64))))))) else (if (Nat.blt (position) (60)) then (if (Nat.blt (position) (50)) then (if (Nat.blt (position) (45)) then (if (Nat.blt (position) (42)) then (if (Nat.blt (position) (41)) then (11727347734174303076 : UInt64) else (12113106623233404929 : UInt64)) else (if (Nat.blt (position) (43)) then (14000437183269869457 : UInt64) else (if (Nat.blt (position) (44)) then (14369950271660146224 : UInt64) else (15101387698204529176 : UInt64)))) else (if (Nat.blt (position) (47)) then (if (Nat.blt (position) (46)) then (15463397548674623760 : UInt64) else (17586052441742319658 : UInt64)) else (if (Nat.blt (position) (48)) then (1182934255886127544 : UInt64) else (if (Nat.blt (position) (49)) then (1847814050463011016 : UInt64) else (2177327727835720531 : UInt64))))) else (if (Nat.blt (position) (55)) then (if (Nat.blt (position) (52)) then (if (Nat.blt (position) (51)) then (2830643537854262169 : UInt64) else (3796741975233480872 : UInt64)) else (if (Nat.blt (position) (53)) then (4115178125766777443 : UInt64) else (if (Nat.blt (position) (54)) then (5681478168544905931 : UInt64) else (6601373596472566643 : UInt64)))) else (if (Nat.blt (position) (57)) then (if (Nat.blt (position) (56)) then (7507060721942968483 : UInt64) else (8399075790359081724 : UInt64)) else (if (Nat.blt (position) (58)) then (8693463985226723168 : UInt64) else (if (Nat.blt (position) (59)) then (9568029438360202098 : UInt64) else (10144078919501101548 : UInt64)))))) else (if (Nat.blt (position) (70)) then (if (Nat.blt (position) (65)) then (if (Nat.blt (position) (62)) then (if (Nat.blt (position) (61)) then (10430055236837252648 : UInt64) else (11840083180663258601 : UInt64)) else (if (Nat.blt (position) (63)) then (13761210420658862357 : UInt64) else (if (Nat.blt (position) (64)) then (14299343276471374635 : UInt64) else (14566680578165727644 : UInt64)))) else (if (Nat.blt (position) (67)) then (if (Nat.blt (position) (66)) then (15097957966210449927 : UInt64) else (16922976911328602910 : UInt64)) else (if (Nat.blt (position) (68)) then (17689382322260857208 : UInt64) else (if (Nat.blt (position) (69)) then (500013540394364858 : UInt64) else (748580250866718886 : UInt64))))) else (if (Nat.blt (position) (75)) then (if (Nat.blt (position) (72)) then (if (Nat.blt (position) (71)) then (1242879168328830382 : UInt64) else (1977374033974150939 : UInt64)) else (if (Nat.blt (position) (73)) then (2944078676154940804 : UInt64) else (if (Nat.blt (position) (74)) then (3659926193048069267 : UInt64) else (4368137639120453308 : UInt64)))) else (if (Nat.blt (position) (77)) then (if (Nat.blt (position) (76)) then (4836135668995329356 : UInt64) else (5532061633213252278 : UInt64)) else (if (Nat.blt (position) (78)) then (6448918945643986474 : UInt64) else (if (Nat.blt (position) (79)) then (6902733635092675308 : UInt64) else (7801388544844847127 : UInt64))))))))

@[expose] public def round (state : Working) : Working := (let constant : UInt64 := roundConstant ((state).index); (let t1 : UInt64 := LexLeanKappa.Word.add ((state).h) (LexLeanKappa.Word.add (bigSigma1 ((state).e)) (LexLeanKappa.Word.add (choose ((state).e) ((state).f) ((state).g)) (LexLeanKappa.Word.add (constant) (((state).window).w0)))); (let t2 : UInt64 := LexLeanKappa.Word.add (bigSigma0 ((state).a)) (majority ((state).a) ((state).b) ((state).c)); ({ a := LexLeanKappa.Word.add (t1) (t2), b := (state).a, c := (state).b, d := (state).c, e := LexLeanKappa.Word.add ((state).d) (t1), f := (state).e, g := (state).f, h := (state).g, window := advance ((state).window), index := ((state).index + 1) } : Working))))

@[expose] public def initial : Digest := ({ h0 := (7640891576956012808 : UInt64), h1 := (13503953896175478587 : UInt64), h2 := (4354685564936845355 : UInt64), h3 := (11912009170470909681 : UInt64), h4 := (5840696475078001361 : UInt64), h5 := (11170449401992604703 : UInt64), h6 := (2270897969802886507 : UInt64), h7 := (6620516959819538809 : UInt64) } : Digest)

@[expose] public def compress (hash : Digest) (block : Schedule) : Digest := (let final : Working := (LexLeanCollections.iterate ((fun (state : Working) => round (state))) (80) (({ a := (hash).h0, b := (hash).h1, c := (hash).h2, d := (hash).h3, e := (hash).h4, f := (hash).h5, g := (hash).h6, h := (hash).h7, window := block, index := 0 } : Working)) : Working); ({ h0 := LexLeanKappa.Word.add ((hash).h0) ((final).a), h1 := LexLeanKappa.Word.add ((hash).h1) ((final).b), h2 := LexLeanKappa.Word.add ((hash).h2) ((final).c), h3 := LexLeanKappa.Word.add ((hash).h3) ((final).d), h4 := LexLeanKappa.Word.add ((hash).h4) ((final).e), h5 := LexLeanKappa.Word.add ((hash).h5) ((final).f), h6 := LexLeanKappa.Word.add ((hash).h6) ((final).g), h7 := LexLeanKappa.Word.add ((hash).h7) ((final).h) } : Digest))

@[expose] public def byteAt (message : ByteArray) (size : Nat) (position : Nat) : UInt64 := (if (Nat.blt (position) (size)) then LexLeanKappa.Word.byteWord (message) (position) else (if (Nat.beq (position) (size)) then (128 : UInt64) else (0 : UInt64)))

@[expose] public def scheduleWord (message : ByteArray) (size : Nat) (total : Nat) (position : Nat) : UInt64 := (if (Nat.beq ((position + 16)) (total)) then (0 : UInt64) else (if (Nat.beq ((position + 8)) (total)) then LexLeanKappa.Word.ofNat ((LexLeanRuntime.multiply (size) (8) : Nat)) else LexLeanKappa.Word.fromBytes (byteAt (message) (size) ((position + 0))) (byteAt (message) (size) ((position + 1))) (byteAt (message) (size) ((position + 2))) (byteAt (message) (size) ((position + 3))) (byteAt (message) (size) ((position + 4))) (byteAt (message) (size) ((position + 5))) (byteAt (message) (size) ((position + 6))) (byteAt (message) (size) ((position + 7)))))

@[expose] public def load (message : ByteArray) (size : Nat) (total : Nat) (offset : Nat) : Schedule := ({ w0 := scheduleWord (message) (size) (total) ((offset + 0)), w1 := scheduleWord (message) (size) (total) ((offset + 8)), w2 := scheduleWord (message) (size) (total) ((offset + 16)), w3 := scheduleWord (message) (size) (total) ((offset + 24)), w4 := scheduleWord (message) (size) (total) ((offset + 32)), w5 := scheduleWord (message) (size) (total) ((offset + 40)), w6 := scheduleWord (message) (size) (total) ((offset + 48)), w7 := scheduleWord (message) (size) (total) ((offset + 56)), w8 := scheduleWord (message) (size) (total) ((offset + 64)), w9 := scheduleWord (message) (size) (total) ((offset + 72)), w10 := scheduleWord (message) (size) (total) ((offset + 80)), w11 := scheduleWord (message) (size) (total) ((offset + 88)), w12 := scheduleWord (message) (size) (total) ((offset + 96)), w13 := scheduleWord (message) (size) (total) ((offset + 104)), w14 := scheduleWord (message) (size) (total) ((offset + 112)), w15 := scheduleWord (message) (size) (total) ((offset + 120)) } : Schedule)

@[expose] public def sha512 (message : ByteArray) : Digest := (let size : Nat := (LexLeanRuntime.length (message) : Nat); (let total : Nat := (LexLeanRuntime.multiply ((LexLeanRuntime.quotient ((size + 144)) (128) (0) : Nat)) (128) : Nat); ((LexLeanCollections.iterate ((fun (state : (Prod (Nat) (Digest))) => (((state).1 + 128), compress ((state).2) (load (message) (size) (total) ((state).1))))) ((LexLeanRuntime.quotient (total) (128) (0) : Nat)) ((0, initial)) : (Prod (Nat) (Digest)))).2))

@[expose] public def sha512Hex (message : ByteArray) : ByteArray := (let digest : Digest := sha512 (message); (LexLeanRuntime.append (LexLeanKappa.Word.hexBig ((digest).h0)) ((LexLeanRuntime.append (LexLeanKappa.Word.hexBig ((digest).h1)) ((LexLeanRuntime.append (LexLeanKappa.Word.hexBig ((digest).h2)) ((LexLeanRuntime.append (LexLeanKappa.Word.hexBig ((digest).h3)) ((LexLeanRuntime.append (LexLeanKappa.Word.hexBig ((digest).h4)) ((LexLeanRuntime.append (LexLeanKappa.Word.hexBig ((digest).h5)) ((LexLeanRuntime.append (LexLeanKappa.Word.hexBig ((digest).h6)) (LexLeanKappa.Word.hexBig ((digest).h7)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray))

end LexLeanKappa.Sha512
