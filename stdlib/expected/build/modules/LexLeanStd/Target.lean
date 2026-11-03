module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanStd.Target

@[expose] public def plus (left : Int) (right : Int) : Int := (LexLeanRuntime.subtract (left) ((LexLeanRuntime.negate (right) : Int)) : Int)

@[expose] public def saturate16 (value : Int) : Int := (if (LexLeanCollections.lessThan (value) ((LexLeanRuntime.negate ((32768 : Int)) : Int)) : Bool) then (LexLeanRuntime.negate ((32768 : Int)) : Int) else (if (LexLeanCollections.lessThan ((32767 : Int)) (value) : Bool) then (32767 : Int) else value))

@[expose] public def wrap32 (value : Int) : Int := (if (LexLeanCollections.lessThan (value) ((LexLeanRuntime.negate ((2147483648 : Int)) : Int)) : Bool) then plus (value) ((4294967296 : Int)) else (if (LexLeanCollections.lessThan ((2147483647 : Int)) (value) : Bool) then (LexLeanRuntime.subtract (value) ((4294967296 : Int)) : Int) else value))

@[expose] public def finishPair (mode : Nat) (value : Int) : Int := (if (Nat.beq (mode) (0)) then saturate16 (value) else (if (Nat.beq (mode) (1)) then wrap32 (value) else value))

@[expose] public def products : (count : Nat) -> (left : List (Int)) -> (right : List (Int)) -> List (Int)
  | _count, List.nil, _right => ([] : List (Int))
  | count, List.cons l lrest, right => (match right with | List.nil => ([] : List (Int)) | List.cons r rrest => (if (Nat.beq (count) (0)) then ([] : List (Int)) else ((LexLeanRuntime.multiply (l) (r) : Int) :: products ((LexLeanRuntime.subtract (count) (1) : Nat)) (lrest) (rrest))))

@[expose] public def pairSums : (mode : Nat) -> (phase : Nat) -> (held : Int) -> (values : List (Int)) -> List (Int)
  | _mode, _phase, _held, List.nil => ([] : List (Int))
  | mode, phase, held, List.cons v rest => (if (Nat.beq (phase) (0)) then pairSums (mode) (1) (v) (rest) else (finishPair (mode) (plus (held) (v)) :: pairSums (mode) (0) ((0 : Int)) (rest)))

@[expose] public def saturatingAdd : (left : List (Int)) -> (right : List (Int)) -> List (Int)
  | List.nil, _right => ([] : List (Int))
  | List.cons l lrest, right => (match right with | List.nil => ([] : List (Int)) | List.cons r rrest => (saturate16 (plus (l) (r)) :: saturatingAdd (lrest) (rrest)))

@[expose] public def avx2MaddubsEpi16 (left : List (Int)) (right : List (Int)) : List (Int) := pairSums (0) (0) ((0 : Int)) (products (64) (left) (right))

@[expose] public def avx2MaddEpi16 (left : List (Int)) (right : List (Int)) : List (Int) := pairSums (1) (0) ((0 : Int)) (products (64) (left) (right))

@[expose] public def avx2AddsEpi16 (left : List (Int)) (right : List (Int)) : List (Int) := saturatingAdd (left) (right)

@[expose] public def neonMullS8 (left : List (Int)) (right : List (Int)) : List (Int) := products (8) (left) (right)

@[expose] public def neonPaddlqS16 (left : List (Int)) : List (Int) := pairSums (2) (0) ((0 : Int)) (left)

@[expose] public def neonQaddqS16 (left : List (Int)) (right : List (Int)) : List (Int) := saturatingAdd (left) (right)

@[expose] public def wasmExtmulLowI8x16 (left : List (Int)) (right : List (Int)) : List (Int) := products (8) (left) (right)

@[expose] public def wasmDotI16x8 (left : List (Int)) (right : List (Int)) : List (Int) := pairSums (1) (0) ((0 : Int)) (products (64) (left) (right))

@[expose] public def wasmAddSatI16x8 (left : List (Int)) (right : List (Int)) : List (Int) := saturatingAdd (left) (right)

public inductive Op where
  | avx2Maddubs
  | avx2Madd
  | avx2Adds
  | neonMull
  | neonPaddl
  | neonQadd
  | wasmExtmul
  | wasmDot
  | wasmAddSat

public structure Cost where
  ops : Nat
  bytes : Nat

@[expose] public def blockLanes (op : Op) : Nat := (match op with | Op.avx2Maddubs => 32 | Op.avx2Madd => 16 | Op.avx2Adds => 16 | Op.neonMull => 8 | Op.neonPaddl => 8 | Op.neonQadd => 8 | Op.wasmExtmul => 16 | Op.wasmDot => 8 | Op.wasmAddSat => 8)

@[expose] public def readBytes (op : Op) : Nat := (match op with | Op.avx2Maddubs => 64 | Op.avx2Madd => 64 | Op.avx2Adds => 64 | Op.neonMull => 16 | Op.neonPaddl => 16 | Op.neonQadd => 32 | Op.wasmExtmul => 32 | Op.wasmDot => 32 | Op.wasmAddSat => 32)

@[expose] public def price (op : Op) (lanes : Nat) : Cost := (let blocks : Nat := (LexLeanRuntime.quotient ((lanes + (LexLeanRuntime.subtract (blockLanes (op)) (1) : Nat))) (blockLanes (op)) (0) : Nat); ({ ops := blocks, bytes := (LexLeanRuntime.multiply (blocks) (readBytes (op)) : Nat) } : Cost))

@[expose] public def heavier (state : Cost) (candidate : Cost) : Cost := (if (Nat.blt ((state).bytes) ((candidate).bytes)) then candidate else state)

@[expose] public def worstCase (op : Op) (domain : List (Nat)) : Cost := (LexLeanCollections.listFold ((fun (state : Cost) (lanes : Nat) => heavier (state) (price (op) (lanes)))) (({ ops := 0, bytes := 0 } : Cost)) (domain) : Cost)

@[expose] public def bounded (op : Op) (domain : List (Nat)) : Bool := (Nat.ble ((worstCase (op) (domain)).bytes) ((LexLeanRuntime.multiply ((worstCase (op) (domain)).ops) (readBytes (op)) : Nat)))

@[expose] public def allBounded (domain : List (Nat)) : Bool := (if bounded (Op.avx2Maddubs) (domain) then (if bounded (Op.avx2Madd) (domain) then (if bounded (Op.avx2Adds) (domain) then (if bounded (Op.neonMull) (domain) then (if bounded (Op.neonPaddl) (domain) then (if bounded (Op.neonQadd) (domain) then (if bounded (Op.wasmExtmul) (domain) then (if bounded (Op.wasmDot) (domain) then bounded (Op.wasmAddSat) (domain) else false) else false) else false) else false) else false) else false) else false) else false)

public theorem maddubs_saturates_high : (avx2MaddubsEpi16 (((255 : Int) :: ((255 : Int) :: ([] : List (Int))))) (((127 : Int) :: ((127 : Int) :: ([] : List (Int))))) = ((32767 : Int) :: ([] : List (Int)))) := by
  decide

public theorem maddubs_saturates_low : (avx2MaddubsEpi16 (((255 : Int) :: ((255 : Int) :: ([] : List (Int))))) (((LexLeanRuntime.negate ((128 : Int)) : Int) :: ((LexLeanRuntime.negate ((128 : Int)) : Int) :: ([] : List (Int))))) = ((LexLeanRuntime.negate ((32768 : Int)) : Int) :: ([] : List (Int)))) := by
  decide

public theorem maddubs_exact : (avx2MaddubsEpi16 (((3 : Int) :: ((4 : Int) :: ([] : List (Int))))) (((5 : Int) :: ((LexLeanRuntime.negate ((6 : Int)) : Int) :: ([] : List (Int))))) = ((LexLeanRuntime.negate ((9 : Int)) : Int) :: ([] : List (Int)))) := by
  decide

public theorem madd_wraps : (avx2MaddEpi16 (((LexLeanRuntime.negate ((32768 : Int)) : Int) :: ((LexLeanRuntime.negate ((32768 : Int)) : Int) :: ([] : List (Int))))) (((LexLeanRuntime.negate ((32768 : Int)) : Int) :: ((LexLeanRuntime.negate ((32768 : Int)) : Int) :: ([] : List (Int))))) = ((LexLeanRuntime.negate ((2147483648 : Int)) : Int) :: ([] : List (Int)))) := by
  decide

public theorem madd_exact : (avx2MaddEpi16 (((1 : Int) :: ((2 : Int) :: ((3 : Int) :: ((4 : Int) :: ([] : List (Int))))))) (((5 : Int) :: ((6 : Int) :: ((7 : Int) :: ((8 : Int) :: ([] : List (Int))))))) = ((17 : Int) :: ((53 : Int) :: ([] : List (Int))))) := by
  decide

public theorem adds_saturates : (avx2AddsEpi16 (((32767 : Int) :: ((LexLeanRuntime.negate ((32768 : Int)) : Int) :: ([] : List (Int))))) (((1 : Int) :: ((LexLeanRuntime.negate ((1 : Int)) : Int) :: ([] : List (Int))))) = ((32767 : Int) :: ((LexLeanRuntime.negate ((32768 : Int)) : Int) :: ([] : List (Int))))) := by
  decide

public theorem mull_widens : (neonMullS8 (((LexLeanRuntime.negate ((128 : Int)) : Int) :: ((127 : Int) :: ([] : List (Int))))) (((LexLeanRuntime.negate ((128 : Int)) : Int) :: ((127 : Int) :: ([] : List (Int))))) = ((16384 : Int) :: ((16129 : Int) :: ([] : List (Int))))) := by
  decide

public theorem mull_takes_eight : (neonMullS8 (((3 : Int) :: ((3 : Int) :: ((3 : Int) :: ((3 : Int) :: ((3 : Int) :: ((3 : Int) :: ((3 : Int) :: ((3 : Int) :: ([] : List (Int))))))))))) (((5 : Int) :: ((5 : Int) :: ((5 : Int) :: ((5 : Int) :: ((5 : Int) :: ((5 : Int) :: ((5 : Int) :: ((5 : Int) :: ([] : List (Int))))))))))) = ((15 : Int) :: ((15 : Int) :: ((15 : Int) :: ((15 : Int) :: ((15 : Int) :: ((15 : Int) :: ((15 : Int) :: ((15 : Int) :: ([] : List (Int))))))))))) := by
  decide

public theorem paddl_widens : (neonPaddlqS16 (((32767 : Int) :: ((32767 : Int) :: ((LexLeanRuntime.negate ((32768 : Int)) : Int) :: ((LexLeanRuntime.negate ((32768 : Int)) : Int) :: ([] : List (Int))))))) = ((65534 : Int) :: ((LexLeanRuntime.negate ((65536 : Int)) : Int) :: ([] : List (Int))))) := by
  decide

public theorem qadd_saturates : (neonQaddqS16 (((32767 : Int) :: ((LexLeanRuntime.negate ((32768 : Int)) : Int) :: ([] : List (Int))))) (((1 : Int) :: ((LexLeanRuntime.negate ((1 : Int)) : Int) :: ([] : List (Int))))) = ((32767 : Int) :: ((LexLeanRuntime.negate ((32768 : Int)) : Int) :: ([] : List (Int))))) := by
  decide

public theorem extmul_takes_low_eight : (wasmExtmulLowI8x16 (((3 : Int) :: ((3 : Int) :: ((3 : Int) :: ((3 : Int) :: ((3 : Int) :: ((3 : Int) :: ((3 : Int) :: ((3 : Int) :: ((3 : Int) :: ((3 : Int) :: ((3 : Int) :: ((3 : Int) :: ((3 : Int) :: ((3 : Int) :: ((3 : Int) :: ((3 : Int) :: ([] : List (Int))))))))))))))))))) (((5 : Int) :: ((5 : Int) :: ((5 : Int) :: ((5 : Int) :: ((5 : Int) :: ((5 : Int) :: ((5 : Int) :: ((5 : Int) :: ((5 : Int) :: ((5 : Int) :: ((5 : Int) :: ((5 : Int) :: ((5 : Int) :: ((5 : Int) :: ((5 : Int) :: ((5 : Int) :: ([] : List (Int))))))))))))))))))) = ((15 : Int) :: ((15 : Int) :: ((15 : Int) :: ((15 : Int) :: ((15 : Int) :: ((15 : Int) :: ((15 : Int) :: ((15 : Int) :: ([] : List (Int))))))))))) := by
  decide

public theorem dot_wraps : (wasmDotI16x8 (((LexLeanRuntime.negate ((32768 : Int)) : Int) :: ((LexLeanRuntime.negate ((32768 : Int)) : Int) :: ([] : List (Int))))) (((LexLeanRuntime.negate ((32768 : Int)) : Int) :: ((LexLeanRuntime.negate ((32768 : Int)) : Int) :: ([] : List (Int))))) = ((LexLeanRuntime.negate ((2147483648 : Int)) : Int) :: ([] : List (Int)))) := by
  decide

public theorem add_sat_saturates : (wasmAddSatI16x8 (((32767 : Int) :: ((LexLeanRuntime.negate ((32768 : Int)) : Int) :: ([] : List (Int))))) (((1 : Int) :: ((LexLeanRuntime.negate ((1 : Int)) : Int) :: ([] : List (Int))))) = ((32767 : Int) :: ((LexLeanRuntime.negate ((32768 : Int)) : Int) :: ([] : List (Int))))) := by
  decide

public theorem price_reads_whole_instructions : ((price (Op.avx2Maddubs) (33)).ops = 2) := by
  decide

public theorem price_bytes : ((price (Op.avx2Maddubs) (33)).bytes = 128) := by
  decide

public theorem price_empty : ((price (Op.neonQadd) (0)).bytes = 0) := by
  decide

public theorem worst_case_is_largest : ((worstCase (Op.avx2Maddubs) ((0 :: (31 :: (1024 :: (33 :: ([] : List (Nat)))))))).bytes = 2048) := by
  decide

public theorem worst_case_bounded_by_operations : (allBounded ((0 :: (1 :: (7 :: (8 :: (9 :: (31 :: (32 :: (33 :: (1024 :: ([] : List (Nat)))))))))))) = true) := by
  decide

end LexLeanStd.Target
