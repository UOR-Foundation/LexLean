module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Sha256
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanKappa.Blake3

public structure Cv where
  c0 : UInt32
  c1 : UInt32
  c2 : UInt32
  c3 : UInt32
  c4 : UInt32
  c5 : UInt32
  c6 : UInt32
  c7 : UInt32

public structure Block where
  m0 : UInt32
  m1 : UInt32
  m2 : UInt32
  m3 : UInt32
  m4 : UInt32
  m5 : UInt32
  m6 : UInt32
  m7 : UInt32
  m8 : UInt32
  m9 : UInt32
  m10 : UInt32
  m11 : UInt32
  m12 : UInt32
  m13 : UInt32
  m14 : UInt32
  m15 : UInt32

public structure State where
  v0 : UInt32
  v1 : UInt32
  v2 : UInt32
  v3 : UInt32
  v4 : UInt32
  v5 : UInt32
  v6 : UInt32
  v7 : UInt32
  v8 : UInt32
  v9 : UInt32
  v10 : UInt32
  v11 : UInt32
  v12 : UInt32
  v13 : UInt32
  v14 : UInt32
  v15 : UInt32

public structure Quad where
  a : UInt32
  b : UInt32
  c : UInt32
  d : UInt32

public structure Merge where
  cv : Cv
  total : Nat
  stack : List (Cv)

@[expose] public def g (a : UInt32) (b : UInt32) (c : UInt32) (d : UInt32) (x : UInt32) (y : UInt32) : Quad := (let a1 : UInt32 := LexLeanStd.Sha256.add (LexLeanStd.Sha256.add (a) (b)) (x); (let d1 : UInt32 := LexLeanStd.Sha256.rotateRight ((LexLeanRuntime.bitXor (d) (a1) : UInt32)) ((16 : UInt32)) ((16 : UInt32)); (let c1 : UInt32 := LexLeanStd.Sha256.add (c) (d1); (let b1 : UInt32 := LexLeanStd.Sha256.rotateRight ((LexLeanRuntime.bitXor (b) (c1) : UInt32)) ((12 : UInt32)) ((20 : UInt32)); (let a2 : UInt32 := LexLeanStd.Sha256.add (LexLeanStd.Sha256.add (a1) (b1)) (y); (let d2 : UInt32 := LexLeanStd.Sha256.rotateRight ((LexLeanRuntime.bitXor (d1) (a2) : UInt32)) ((8 : UInt32)) ((24 : UInt32)); (let c2 : UInt32 := LexLeanStd.Sha256.add (c1) (d2); (let b2 : UInt32 := LexLeanStd.Sha256.rotateRight ((LexLeanRuntime.bitXor (b1) (c2) : UInt32)) ((7 : UInt32)) ((25 : UInt32)); ({ a := a2, b := b2, c := c2, d := d2 } : Quad)))))))))

@[expose] public def columns (v : State) (m : Block) : State := (let q0 : Quad := g ((v).v0) ((v).v4) ((v).v8) ((v).v12) ((m).m0) ((m).m1); (let q1 : Quad := g ((v).v1) ((v).v5) ((v).v9) ((v).v13) ((m).m2) ((m).m3); (let q2 : Quad := g ((v).v2) ((v).v6) ((v).v10) ((v).v14) ((m).m4) ((m).m5); (let q3 : Quad := g ((v).v3) ((v).v7) ((v).v11) ((v).v15) ((m).m6) ((m).m7); ({ v0 := (q0).a, v1 := (q1).a, v2 := (q2).a, v3 := (q3).a, v4 := (q0).b, v5 := (q1).b, v6 := (q2).b, v7 := (q3).b, v8 := (q0).c, v9 := (q1).c, v10 := (q2).c, v11 := (q3).c, v12 := (q0).d, v13 := (q1).d, v14 := (q2).d, v15 := (q3).d } : State)))))

@[expose] public def diagonals (v : State) (m : Block) : State := (let q0 : Quad := g ((v).v0) ((v).v5) ((v).v10) ((v).v15) ((m).m8) ((m).m9); (let q1 : Quad := g ((v).v1) ((v).v6) ((v).v11) ((v).v12) ((m).m10) ((m).m11); (let q2 : Quad := g ((v).v2) ((v).v7) ((v).v8) ((v).v13) ((m).m12) ((m).m13); (let q3 : Quad := g ((v).v3) ((v).v4) ((v).v9) ((v).v14) ((m).m14) ((m).m15); ({ v0 := (q0).a, v1 := (q1).a, v2 := (q2).a, v3 := (q3).a, v4 := (q3).b, v5 := (q0).b, v6 := (q1).b, v7 := (q2).b, v8 := (q2).c, v9 := (q3).c, v10 := (q0).c, v11 := (q1).c, v12 := (q1).d, v13 := (q2).d, v14 := (q3).d, v15 := (q0).d } : State)))))

@[expose] public def permute (m : Block) : Block := ({ m0 := (m).m2, m1 := (m).m6, m2 := (m).m3, m3 := (m).m10, m4 := (m).m7, m5 := (m).m0, m6 := (m).m4, m7 := (m).m13, m8 := (m).m1, m9 := (m).m11, m10 := (m).m12, m11 := (m).m5, m12 := (m).m9, m13 := (m).m14, m14 := (m).m15, m15 := (m).m8 } : Block)

@[expose] public def roundStep (state : (Prod (State) (Block))) : (Prod (State) (Block)) := (diagonals (columns ((state).1) ((state).2)) ((state).2), permute ((state).2))

@[expose] public def compress (cv : Cv) (block : Block) (counter : Nat) (length : UInt32) (flags : UInt32) : Cv := (let final : State := ((LexLeanCollections.iterate ((fun (state : (Prod (State) (Block))) => roundStep (state))) (7) ((({ v0 := (cv).c0, v1 := (cv).c1, v2 := (cv).c2, v3 := (cv).c3, v4 := (cv).c4, v5 := (cv).c5, v6 := (cv).c6, v7 := (cv).c7, v8 := (1779033703 : UInt32), v9 := (3144134277 : UInt32), v10 := (1013904242 : UInt32), v11 := (2773480762 : UInt32), v12 := LexLeanStd.Sha256.wordOfNat ((LexLeanRuntime.remainder (counter) (4294967296) (0) : Nat)), v13 := LexLeanStd.Sha256.wordOfNat ((LexLeanRuntime.quotient (counter) (4294967296) (0) : Nat)), v14 := length, v15 := flags } : State), block)) : (Prod (State) (Block)))).1; ({ c0 := (LexLeanRuntime.bitXor ((final).v0) ((final).v8) : UInt32), c1 := (LexLeanRuntime.bitXor ((final).v1) ((final).v9) : UInt32), c2 := (LexLeanRuntime.bitXor ((final).v2) ((final).v10) : UInt32), c3 := (LexLeanRuntime.bitXor ((final).v3) ((final).v11) : UInt32), c4 := (LexLeanRuntime.bitXor ((final).v4) ((final).v12) : UInt32), c5 := (LexLeanRuntime.bitXor ((final).v5) ((final).v13) : UInt32), c6 := (LexLeanRuntime.bitXor ((final).v6) ((final).v14) : UInt32), c7 := (LexLeanRuntime.bitXor ((final).v7) ((final).v15) : UInt32) } : Cv))

@[expose] public def initial : Cv := ({ c0 := (1779033703 : UInt32), c1 := (3144134277 : UInt32), c2 := (1013904242 : UInt32), c3 := (2773480762 : UInt32), c4 := (1359893119 : UInt32), c5 := (2600822924 : UInt32), c6 := (528734635 : UInt32), c7 := (1541459225 : UInt32) } : Cv)

@[expose] public def byteAt (message : ByteArray) (position : Nat) : UInt32 := (match (LexLeanRuntime.index (message) (position) : Option (UInt8)) with | Option.some byte => (match (LexLeanRuntime.checkedConvert (byte) : Option (UInt32)) with | Option.some wide => wide | Option.none => (0 : UInt32)) | Option.none => (0 : UInt32))

@[expose] public def wordAt (message : ByteArray) (position : Nat) : UInt32 := (LexLeanRuntime.bitOr (byteAt (message) (position)) ((LexLeanRuntime.bitOr (LexLeanStd.Sha256.shiftLeft (byteAt (message) ((position + 1))) ((8 : UInt32))) ((LexLeanRuntime.bitOr (LexLeanStd.Sha256.shiftLeft (byteAt (message) ((position + 2))) ((16 : UInt32))) (LexLeanStd.Sha256.shiftLeft (byteAt (message) ((position + 3))) ((24 : UInt32))) : UInt32)) : UInt32)) : UInt32)

@[expose] public def load (message : ByteArray) (offset : Nat) : Block := ({ m0 := wordAt (message) ((offset + 0)), m1 := wordAt (message) ((offset + 4)), m2 := wordAt (message) ((offset + 8)), m3 := wordAt (message) ((offset + 12)), m4 := wordAt (message) ((offset + 16)), m5 := wordAt (message) ((offset + 20)), m6 := wordAt (message) ((offset + 24)), m7 := wordAt (message) ((offset + 28)), m8 := wordAt (message) ((offset + 32)), m9 := wordAt (message) ((offset + 36)), m10 := wordAt (message) ((offset + 40)), m11 := wordAt (message) ((offset + 44)), m12 := wordAt (message) ((offset + 48)), m13 := wordAt (message) ((offset + 52)), m14 := wordAt (message) ((offset + 56)), m15 := wordAt (message) ((offset + 60)) } : Block)

@[expose] public def «prefix» (message : ByteArray) (start : Nat) (index : Nat) (count : Nat) : Cv := ((LexLeanCollections.iterate ((fun (state : (Prod (Nat) (Cv))) => (((state).1 + 1), compress ((state).2) (load (message) ((start + (LexLeanRuntime.multiply (64) ((state).1) : Nat)))) (index) ((64 : UInt32)) ((if (Nat.beq ((state).1) (0)) then (1 : UInt32) else (0 : UInt32)))))) (count) ((0, initial)) : (Prod (Nat) (Cv)))).2

@[expose] public def available (size : Nat) (start : Nat) : Nat := (if (Nat.blt (1024) ((LexLeanRuntime.subtract (size) (start) : Nat))) then 1024 else (LexLeanRuntime.subtract (size) (start) : Nat))

@[expose] public def blocks (bytes : Nat) : Nat := (if (Nat.beq (bytes) (0)) then 1 else (LexLeanRuntime.quotient ((bytes + 63)) (64) (0) : Nat))

@[expose] public def chunk (message : ByteArray) (size : Nat) (index : Nat) (root : Bool) : Cv := (let start : Nat := (LexLeanRuntime.multiply (1024) (index) : Nat); (let rest : Nat := available (size) (start); (let count : Nat := blocks (rest); (let last : Nat := (LexLeanRuntime.subtract (count) (1) : Nat); compress («prefix» (message) (start) (index) (last)) (load (message) ((start + (LexLeanRuntime.multiply (64) (last) : Nat)))) (index) (LexLeanStd.Sha256.wordOfNat ((LexLeanRuntime.subtract (rest) ((LexLeanRuntime.multiply (64) (last) : Nat)) : Nat))) ((LexLeanRuntime.bitOr ((if (Nat.beq (last) (0)) then (3 : UInt32) else (2 : UInt32))) ((if root then (8 : UInt32) else (0 : UInt32))) : UInt32))))))

@[expose] public def parentBlock (left : Cv) (right : Cv) : Block := ({ m0 := (left).c0, m1 := (left).c1, m2 := (left).c2, m3 := (left).c3, m4 := (left).c4, m5 := (left).c5, m6 := (left).c6, m7 := (left).c7, m8 := (right).c0, m9 := (right).c1, m10 := (right).c2, m11 := (right).c3, m12 := (right).c4, m13 := (right).c5, m14 := (right).c6, m15 := (right).c7 } : Block)

@[expose] public def parent (left : Cv) (right : Cv) (root : Bool) : Cv := compress (initial) (parentBlock (left) (right)) (0) ((64 : UInt32)) ((LexLeanRuntime.bitOr ((4 : UInt32)) ((if root then (8 : UInt32) else (0 : UInt32))) : UInt32))

@[expose] public def mergeStep (state : Merge) : Merge := (if (Nat.beq ((LexLeanRuntime.remainder ((state).total) (2) (0) : Nat)) (0)) then (match (state).stack with | List.nil => state | List.cons top rest => ({ cv := parent (top) ((state).cv) (false), total := (LexLeanRuntime.quotient ((state).total) (2) (0) : Nat), stack := rest } : Merge)) else state)

@[expose] public def push (state : (Prod (Nat) (List (Cv)))) (chunkCv : Cv) : (Prod (Nat) (List (Cv))) := (let merged : Merge := (LexLeanCollections.iterate ((fun (pair : Merge) => mergeStep (pair))) (40) (({ cv := chunkCv, total := ((state).1 + 1), stack := (state).2 } : Merge)) : Merge); (((state).1 + 1), ((merged).cv :: (merged).stack)))

@[expose] public def fullChunks (message : ByteArray) (size : Nat) (count : Nat) : (Prod (Nat) (List (Cv))) := (LexLeanCollections.iterate ((fun (state : (Prod (Nat) (List (Cv)))) => push (state) (chunk (message) (size) ((state).1) (false)))) (count) ((0, ([] : List (Cv)))) : (Prod (Nat) (List (Cv))))

@[expose] public def foldStep (state : (Prod (List (Cv)) (Cv))) : (Prod (List (Cv)) (Cv)) := (match (state).1 with | List.nil => state | List.cons top rest => (rest, parent (top) ((state).2) ((match rest with | List.nil => true | List.cons _ _ => false))))

@[expose] public def nibble (value : UInt32) (shift : UInt32) : ByteArray := LexLeanStd.Sha256.hexDigit (LexLeanStd.Sha256.shiftRight (value) (shift))

@[expose] public def hexLittle (value : UInt32) : ByteArray := (LexLeanRuntime.append ((LexLeanRuntime.append (nibble (value) ((4 : UInt32))) (nibble (value) ((0 : UInt32))) : ByteArray)) ((LexLeanRuntime.append ((LexLeanRuntime.append (nibble (value) ((12 : UInt32))) (nibble (value) ((8 : UInt32))) : ByteArray)) ((LexLeanRuntime.append ((LexLeanRuntime.append (nibble (value) ((20 : UInt32))) (nibble (value) ((16 : UInt32))) : ByteArray)) ((LexLeanRuntime.append (nibble (value) ((28 : UInt32))) (nibble (value) ((24 : UInt32))) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)

@[expose] public def treeRoot (message : ByteArray) : Cv := (let size : Nat := (LexLeanRuntime.length (message) : Nat); (let count : Nat := (if (Nat.beq (size) (0)) then 1 else (LexLeanRuntime.quotient ((size + 1023)) (1024) (0) : Nat)); (let last : Nat := (LexLeanRuntime.subtract (count) (1) : Nat); (match (fullChunks (message) (size) (last)).2 with | List.nil => chunk (message) (size) (last) (true) | List.cons top rest => ((LexLeanCollections.iterate ((fun (state : (Prod (List (Cv)) (Cv))) => foldStep (state))) (64) (((top :: rest), chunk (message) (size) (last) (false))) : (Prod (List (Cv)) (Cv)))).2))))

@[expose] public def blake3Hex (message : ByteArray) : ByteArray := (let digest : Cv := treeRoot (message); (LexLeanRuntime.append (hexLittle ((digest).c0)) ((LexLeanRuntime.append (hexLittle ((digest).c1)) ((LexLeanRuntime.append (hexLittle ((digest).c2)) ((LexLeanRuntime.append (hexLittle ((digest).c3)) ((LexLeanRuntime.append (hexLittle ((digest).c4)) ((LexLeanRuntime.append (hexLittle ((digest).c5)) ((LexLeanRuntime.append (hexLittle ((digest).c6)) (hexLittle ((digest).c7)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray))

end LexLeanKappa.Blake3
