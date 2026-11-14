module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanRuntime.Laws
public import LexLeanRuntime.Arrays
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Arrays.Main

@[expose] public def widen (value : Int8) : Int := (let __widen0 : Int8 := value; (LexLeanRuntime.subtract ((LexLeanRuntime.subtract ((LexLeanRuntime.subtract ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((-128 : Int8)) : Int8)) ((-128 : Int8)) : Bool) then (-128 : Int) else (0 : Int))) ((LexLeanRuntime.negate ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((64 : Int8)) : Int8)) ((64 : Int8)) : Bool) then (64 : Int) else (0 : Int))) : Int)) : Int)) ((LexLeanRuntime.negate ((LexLeanRuntime.subtract ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((32 : Int8)) : Int8)) ((32 : Int8)) : Bool) then (32 : Int) else (0 : Int))) ((LexLeanRuntime.negate ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((16 : Int8)) : Int8)) ((16 : Int8)) : Bool) then (16 : Int) else (0 : Int))) : Int)) : Int)) : Int)) : Int)) ((LexLeanRuntime.negate ((LexLeanRuntime.subtract ((LexLeanRuntime.subtract ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((8 : Int8)) : Int8)) ((8 : Int8)) : Bool) then (8 : Int) else (0 : Int))) ((LexLeanRuntime.negate ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((4 : Int8)) : Int8)) ((4 : Int8)) : Bool) then (4 : Int) else (0 : Int))) : Int)) : Int)) ((LexLeanRuntime.negate ((LexLeanRuntime.subtract ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((2 : Int8)) : Int8)) ((2 : Int8)) : Bool) then (2 : Int) else (0 : Int))) ((LexLeanRuntime.negate ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((1 : Int8)) : Int8)) ((1 : Int8)) : Bool) then (1 : Int) else (0 : Int))) : Int)) : Int)) : Int)) : Int)) : Int)) : Int))

@[expose] public def plus (left : Int) (right : Int) : Int := (LexLeanRuntime.subtract (left) ((LexLeanRuntime.negate (right) : Int)) : Int)

@[expose] public def dotList : (left : List (Int8)) -> (right : List (Int8)) -> Int
  | List.nil, _right => (0 : Int)
  | List.cons x xs, right => (match right with | List.nil => (0 : Int) | List.cons y ys => plus ((LexLeanRuntime.multiply (widen (x)) (widen (y)) : Int)) (dotList (xs) (ys)))

@[expose] public def dotIndex : (left : List (Int8)) -> (right : List (Int8)) -> (position : Nat) -> (fuel : Nat) -> (total : Int) -> Int
  | _left, _right, _position, Nat.zero, total => total
  | left, right, position, Nat.succ rest, total => (match (LexLeanRuntime.index (left) (position) : Option (Int8)) with | Option.none => total | Option.some x => (match (LexLeanRuntime.index (right) (position) : Option (Int8)) with | Option.none => total | Option.some y => dotIndex (left) (right) ((position + 1)) (rest) (plus (total) ((LexLeanRuntime.multiply (widen (x)) (widen (y)) : Int)))))

@[expose] public def dotLoop : (left : _root_.Array (Int8)) -> (right : _root_.Array (Int8)) -> (position : Nat) -> (fuel : Nat) -> (total : Int) -> Int
  | _left, _right, _position, Nat.zero, total => total
  | left, right, position, Nat.succ rest, total => (match (LexLeanRuntime.index (left) (position) : Option (Int8)) with | Option.none => total | Option.some x => (match (LexLeanRuntime.index (right) (position) : Option (Int8)) with | Option.none => total | Option.some y => dotLoop (left) (right) ((position + 1)) (rest) (plus (total) ((LexLeanRuntime.multiply (widen (x)) (widen (y)) : Int)))))

@[expose] public def dot (left : _root_.Array (Int8)) (right : _root_.Array (Int8)) : Int := dotLoop (left) (right) (0) ((LexLeanRuntime.length (left) : Nat)) ((0 : Int))

public theorem array_index (value : _root_.Array (Int8)) (offset : Nat) : ((LexLeanRuntime.index (value) (offset) : Option (Int8)) = (LexLeanRuntime.index ((_root_.LexLeanArrays.toList (value) : List (Int8))) (offset) : Option (Int8))) := by
  exact @_root_.LexLeanArrays.index_to_list (Int8) (value) (offset)

public theorem array_length (value : _root_.Array (Int8)) : ((LexLeanRuntime.length (value) : Nat) = (LexLeanRuntime.length ((_root_.LexLeanArrays.toList (value) : List (Int8))) : Nat)) := by
  exact @_root_.LexLeanArrays.length_to_list (Int8) (value)

public theorem loop_reads_list (left : _root_.Array (Int8)) (right : _root_.Array (Int8)) (position : Nat) (fuel : Nat) (total : Int) : (dotLoop (left) (right) (position) (fuel) (total) = dotIndex ((_root_.LexLeanArrays.toList (left) : List (Int8))) ((_root_.LexLeanArrays.toList (right) : List (Int8))) (position) (fuel) (total)) := by
  induction fuel generalizing position total with
  | zero =>
    simp only [dotIndex, dotLoop]
  | succ rest hypothesis =>
    simp only [array_index, dotIndex, dotLoop, hypothesis]

public theorem index_shift (x : Int8) (xs : List (Int8)) (position : Nat) : ((LexLeanRuntime.index ((x :: xs)) ((position + 1)) : Option (Int8)) = (LexLeanRuntime.index (xs) (position) : Option (Int8))) := by
  rfl

public theorem index_head (x : Int8) (xs : List (Int8)) : ((LexLeanRuntime.index ((x :: xs)) (0) : Option (Int8)) = Option.some (x)) := by
  rfl

public theorem index_empty (position : Nat) : ((LexLeanRuntime.index (([] : List (Int8))) (position) : Option (Int8)) = Option.none) := by
  rfl

public theorem length_empty : ((LexLeanRuntime.length (([] : List (Int8))) : Nat) = 0) := by
  rfl

public theorem length_step (x : Int8) (xs : List (Int8)) : ((LexLeanRuntime.length ((x :: xs)) : Nat) = ((LexLeanRuntime.length (xs) : Nat) + 1)) := by
  rfl

public theorem shift (x : Int8) (y : Int8) (xs : List (Int8)) (ys : List (Int8)) (position : Nat) (fuel : Nat) (total : Int) : (dotIndex ((x :: xs)) ((y :: ys)) ((position + 1)) (fuel) (total) = dotIndex (xs) (ys) (position) (fuel) (total)) := by
  induction fuel generalizing position total with
  | zero =>
    simp only [dotIndex]
  | succ rest hypothesis =>
    simp only [dotIndex, hypothesis, index_shift]

public theorem plus_assoc (a : Int) (b : Int) (c : Int) : (plus (plus (a) (b)) (c) = plus (a) (plus (b) (c))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [plus, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem plus_zero (a : Int) : (plus (a) ((0 : Int)) = a) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [plus, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem zero_plus (a : Int) : (plus ((0 : Int)) (a) = a) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [plus, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem index_sums (left : List (Int8)) (right : List (Int8)) (total : Int) : (dotIndex (left) (right) (0) ((LexLeanRuntime.length (left) : Nat)) (total) = plus (total) (dotList (left) (right))) := by
  induction left generalizing right total with
  | nil =>
    simp only [dotIndex, dotList, length_empty, plus_zero]
  | cons x xs hypothesis =>
    cases right with
    | nil =>
      simp only [dotIndex, dotList, index_empty, index_head, length_step, plus_zero]
    | cons y ys =>
      simp only [dotIndex, dotList, hypothesis, index_head, length_step, plus_assoc, shift]

public theorem dot_equals_list (left : _root_.Array (Int8)) (right : _root_.Array (Int8)) : (dot (left) (right) = dotList ((_root_.LexLeanArrays.toList (left) : List (Int8))) ((_root_.LexLeanArrays.toList (right) : List (Int8)))) := by
  simp only [array_length, dot, index_sums, loop_reads_list, zero_plus]

public theorem dot_sample : (dot ((_root_.LexLeanArrays.fromList (((2 : Int8) :: ((-3 : Int8) :: ([] : List (Int8))))) : _root_.Array (Int8))) ((_root_.LexLeanArrays.fromList (((4 : Int8) :: ((5 : Int8) :: ([] : List (Int8))))) : _root_.Array (Int8))) = (-7 : Int)) := by
  decide

@[expose] public def «at» (values : _root_.Array (Int8)) (position : Nat) : Int8 := (match (LexLeanRuntime.index (values) (position) : Option (Int8)) with | Option.none => (0 : Int8) | Option.some value => value)

@[expose] public def rectify (value : Int8) : Int8 := (if (LexLeanCollections.lessThan (value) ((0 : Int8)) : Bool) then (0 : Int8) else value)

@[expose] public def rectifyLoop : (values : _root_.Array (Int8)) -> (position : Nat) -> (fuel : Nat) -> _root_.Array (Int8)
  | values, _position, Nat.zero => values
  | values, position, Nat.succ rest => rectifyLoop ((_root_.LexLeanArrays.set (values) (position) (rectify («at» (values) (position))) : _root_.Array (Int8))) ((position + 1)) (rest)

@[expose] public def rectifyAll (values : _root_.Array (Int8)) : _root_.Array (Int8) := rectifyLoop (values) (0) ((LexLeanRuntime.length (values) : Nat))

public theorem set_keeps_length (values : _root_.Array (Int8)) (position : Nat) (value : Int8) : ((LexLeanRuntime.length ((_root_.LexLeanArrays.set (values) (position) (value) : _root_.Array (Int8))) : Nat) = (LexLeanRuntime.length (values) : Nat)) := by
  exact @_root_.LexLeanArrays.length_set (Int8) (values) (position) (value)

public theorem loop_keeps_length (values : _root_.Array (Int8)) (position : Nat) (fuel : Nat) : ((LexLeanRuntime.length (rectifyLoop (values) (position) (fuel)) : Nat) = (LexLeanRuntime.length (values) : Nat)) := by
  induction fuel generalizing position values with
  | zero =>
    simp only [rectifyLoop]
  | succ rest hypothesis =>
    simp only [hypothesis, rectifyLoop, set_keeps_length]

public theorem rectify_keeps_length (values : _root_.Array (Int8)) : ((LexLeanRuntime.length (rectifyAll (values)) : Nat) = (LexLeanRuntime.length (values) : Nat)) := by
  simp only [loop_keeps_length, rectifyAll]

public theorem rectify_sample : ((_root_.LexLeanArrays.toList (rectifyAll ((_root_.LexLeanArrays.fromList (((-3 : Int8) :: ((5 : Int8) :: ((-1 : Int8) :: ([] : List (Int8)))))) : _root_.Array (Int8)))) : List (Int8)) = ((0 : Int8) :: ((5 : Int8) :: ((0 : Int8) :: ([] : List (Int8)))))) := by
  decide

end Arrays.Main
