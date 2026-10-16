module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanRuntime.Reasoning
public import LexLeanRuntime.Laws
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Reasoning.Planner

@[expose, reducible] public def Moves (_s : (Prod (Nat) (Nat))) (t : (Prod (Nat) (Nat))) : Prop := (((t).1 <= 4) /\ ((t).2 <= 3))

@[expose, reducible] public def Fits (s : (Prod (Nat) (Nat))) : Prop := (((s).1 <= 4) /\ ((s).2 <= 3))

public theorem fits_kept (s : (Prod (Nat) (Nat))) (t : (Prod (Nat) (Nat))) : (Fits (s) -> (Moves (s) (t) -> Fits (t))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Fits, Moves, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem Pouring.preserves : (LexLeanReasoning.Preserves ((Reasoning.Planner.Moves)) ((Reasoning.Planner.Fits))) :=
  Reasoning.Planner.fits_kept

public theorem filla_sound (s : (Prod (Nat) (Nat))) : ((((Nat.blt ((s).1) (4)) && (Nat.ble ((s).2) (3))) = true) -> Moves (s) ((4, (s).2))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Moves, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

@[expose] public def FillA.guard (s : (Prod (Nat) (Nat))) : Bool := ((Nat.blt ((s).1) (4)) && (Nat.ble ((s).2) (3)))
@[expose] public def FillA.conclusion (s : (Prod (Nat) (Nat))) : (Prod (Nat) (Nat)) := (4, (s).2)
@[expose] public def FillA.apply (s : (Prod (Nat) (Nat))) : Option ((Prod (Nat) (Nat))) := (if Reasoning.Planner.FillA.guard (s) then Option.some (Reasoning.Planner.FillA.conclusion (s)) else Option.none)
public theorem FillA.apply_sound (s : (Prod (Nat) (Nat))) (__t : (Prod (Nat) (Nat))) : ((Reasoning.Planner.FillA.apply (s) = Option.some (__t)) -> Reasoning.Planner.Moves (s) (__t)) :=
  (LexLeanReasoning.guarded ((Reasoning.Planner.Moves)) (s) (Reasoning.Planner.FillA.guard (s)) (Reasoning.Planner.FillA.conclusion (s)) (Reasoning.Planner.filla_sound (s)) (__t))

public theorem emptyb_sound (s : (Prod (Nat) (Nat))) : ((((Nat.blt (0) ((s).2)) && (Nat.ble ((s).1) (4))) = true) -> Moves (s) (((s).1, 0))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Moves, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

@[expose] public def EmptyB.guard (s : (Prod (Nat) (Nat))) : Bool := ((Nat.blt (0) ((s).2)) && (Nat.ble ((s).1) (4)))
@[expose] public def EmptyB.conclusion (s : (Prod (Nat) (Nat))) : (Prod (Nat) (Nat)) := ((s).1, 0)
@[expose] public def EmptyB.apply (s : (Prod (Nat) (Nat))) : Option ((Prod (Nat) (Nat))) := (if Reasoning.Planner.EmptyB.guard (s) then Option.some (Reasoning.Planner.EmptyB.conclusion (s)) else Option.none)
public theorem EmptyB.apply_sound (s : (Prod (Nat) (Nat))) (__t : (Prod (Nat) (Nat))) : ((Reasoning.Planner.EmptyB.apply (s) = Option.some (__t)) -> Reasoning.Planner.Moves (s) (__t)) :=
  (LexLeanReasoning.guarded ((Reasoning.Planner.Moves)) (s) (Reasoning.Planner.EmptyB.guard (s)) (Reasoning.Planner.EmptyB.conclusion (s)) (Reasoning.Planner.emptyb_sound (s)) (__t))

public theorem pour_sound (s : (Prod (Nat) (Nat))) (k : Nat) : ((((Nat.ble (k) ((s).1)) && ((Nat.ble (((s).2 + k)) (3)) && (Nat.ble ((s).1) (4)))) = true) -> Moves (s) (((LexLeanRuntime.subtract ((s).1) (k) : Nat), ((s).2 + k)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Moves, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

@[expose] public def Pour.guard (s : (Prod (Nat) (Nat))) (k : Nat) : Bool := ((Nat.ble (k) ((s).1)) && ((Nat.ble (((s).2 + k)) (3)) && (Nat.ble ((s).1) (4))))
@[expose] public def Pour.conclusion (s : (Prod (Nat) (Nat))) (k : Nat) : (Prod (Nat) (Nat)) := ((LexLeanRuntime.subtract ((s).1) (k) : Nat), ((s).2 + k))
@[expose] public def Pour.candidates (_s : (Prod (Nat) (Nat))) : List (Nat) := (1 :: (2 :: (3 :: ([] : List (Nat)))))
@[expose] public def Pour.apply (s : (Prod (Nat) (Nat))) (k : Nat) : Option ((Prod (Nat) (Nat))) := (if Reasoning.Planner.Pour.guard (s) (k) then Option.some (Reasoning.Planner.Pour.conclusion (s) (k)) else Option.none)
public theorem Pour.apply_sound (s : (Prod (Nat) (Nat))) (k : Nat) (__t : (Prod (Nat) (Nat))) : ((Reasoning.Planner.Pour.apply (s) (k) = Option.some (__t)) -> Reasoning.Planner.Moves (s) (__t)) :=
  (LexLeanReasoning.guarded ((Reasoning.Planner.Moves)) (s) (Reasoning.Planner.Pour.guard (s) (k)) (Reasoning.Planner.Pour.conclusion (s) (k)) (Reasoning.Planner.pour_sound (s) (k)) (__t))

@[expose, reducible] public def Measures (target : Nat) (r : (Prod (Nat) (Nat))) : Prop := (((r).2 = target) /\ ((r).1 <= 4))

@[expose] public def measuresCheck (target : Nat) (r : (Prod (Nat) (Nat))) : Bool := ((Nat.beq ((r).2) (target)) && (Nat.ble ((r).1) (4)))

public theorem measures_sound (target : Nat) (r : (Prod (Nat) (Nat))) : ((measuresCheck (target) (r) = true) -> Measures (target) (r)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Measures, measuresCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem Measured.sound : (LexLeanReasoning.Sound ((Reasoning.Planner.measuresCheck)) ((Reasoning.Planner.Measures))) :=
  Reasoning.Planner.measures_sound

public theorem empty_jugs_fit (_target : Nat) : Fits ((0, 0)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Fits, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public inductive Plan.Step where
  | FillA
  | EmptyB
  | Pour (_ : Nat)
public structure Plan.Ledger where
  iterations : Nat
  attempts : Nat
  firings : Nat
  expansions : Nat
  verifications : Nat
  frontier : Nat
@[expose] public def Plan.observe (_target : Nat) : (Prod (Nat) (Nat)) := (0, 0)
@[expose] public def Plan.fire (__s : (Prod (Nat) (Nat))) (__step : Reasoning.Planner.Plan.Step) : Option ((Prod (Nat) (Nat))) := (match __step with | Reasoning.Planner.Plan.Step.FillA => Reasoning.Planner.FillA.apply (__s) | Reasoning.Planner.Plan.Step.EmptyB => Reasoning.Planner.EmptyB.apply (__s) | Reasoning.Planner.Plan.Step.Pour __b => Reasoning.Planner.Pour.apply (__s) (__b))
@[expose] public def Plan.replay (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : Reasoning.Planner.Plan.Step) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match Reasoning.Planner.Plan.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def Plan.follow (target : Nat) (__trace : List (Reasoning.Planner.Plan.Step)) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (LexLeanCollections.listFold ((Reasoning.Planner.Plan.replay)) (Except.ok (Reasoning.Planner.Plan.observe (target))) (__trace) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))))
@[expose] public def Plan.extract (jugs : (Prod (Nat) (Nat))) : Option ((Prod (Nat) (Nat))) := Option.some (jugs)
@[expose] public def Plan.accept (target : Nat) (__s : (Prod (Nat) (Nat))) : Option ((Prod (Nat) (Nat))) := (match Reasoning.Planner.Plan.extract (__s) with | Option.none => Option.none | Option.some __v => (if Reasoning.Planner.measuresCheck (target) (__v) then Option.some (__v) else Option.none))
@[expose] public def Plan.answer (target : Nat) (__trace : List (Reasoning.Planner.Plan.Step)) : Option ((Prod (Nat) (Nat))) := (match Reasoning.Planner.Plan.follow (target) (__trace) with | Except.ok __s => Reasoning.Planner.Plan.accept (target) (__s) | Except.error _ => Option.none)
public structure Plan.Node where
  state : (Prod (Nat) (Nat))
  trace : List (Reasoning.Planner.Plan.Step)
public structure Plan.Search where
  frontier : List (Reasoning.Planner.Plan.Node)
  visited : List ((Prod (Nat) (Nat)))
  found : Option ((Prod ((Prod (Nat) (Nat))) (Reasoning.Planner.Plan.Node)))
  truncated : Bool
  ledger : Reasoning.Planner.Plan.Ledger
@[expose] public def Plan.FillA.successors (__node : Reasoning.Planner.Plan.Node) : List (Reasoning.Planner.Plan.Node) := (match Reasoning.Planner.Plan.fire ((__node).state) (Reasoning.Planner.Plan.Step.FillA) with | Option.none => ([] : List (Reasoning.Planner.Plan.Node)) | Option.some __t => (({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((Reasoning.Planner.Plan.Step.FillA :: ([] : List (Reasoning.Planner.Plan.Step)))) : List (Reasoning.Planner.Plan.Step)) } : Reasoning.Planner.Plan.Node) :: ([] : List (Reasoning.Planner.Plan.Node))))
@[expose] public def Plan.EmptyB.successors (__node : Reasoning.Planner.Plan.Node) : List (Reasoning.Planner.Plan.Node) := (match Reasoning.Planner.Plan.fire ((__node).state) (Reasoning.Planner.Plan.Step.EmptyB) with | Option.none => ([] : List (Reasoning.Planner.Plan.Node)) | Option.some __t => (({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((Reasoning.Planner.Plan.Step.EmptyB :: ([] : List (Reasoning.Planner.Plan.Step)))) : List (Reasoning.Planner.Plan.Step)) } : Reasoning.Planner.Plan.Node) :: ([] : List (Reasoning.Planner.Plan.Node))))
@[expose] public def Plan.Pour.collect (__node : Reasoning.Planner.Plan.Node) (__acc : List (Reasoning.Planner.Plan.Node)) (__b : Nat) : List (Reasoning.Planner.Plan.Node) := (match Reasoning.Planner.Plan.fire ((__node).state) (Reasoning.Planner.Plan.Step.Pour (__b)) with | Option.none => __acc | Option.some __t => (LexLeanRuntime.append (__acc) ((({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((Reasoning.Planner.Plan.Step.Pour (__b) :: ([] : List (Reasoning.Planner.Plan.Step)))) : List (Reasoning.Planner.Plan.Step)) } : Reasoning.Planner.Plan.Node) :: ([] : List (Reasoning.Planner.Plan.Node)))) : List (Reasoning.Planner.Plan.Node)))
@[expose] public def Plan.Pour.successors (__node : Reasoning.Planner.Plan.Node) : List (Reasoning.Planner.Plan.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Reasoning.Planner.Plan.Node)) (__b : Nat) => Reasoning.Planner.Plan.Pour.collect (__node) (__acc) (__b))) (([] : List (Reasoning.Planner.Plan.Node))) (Reasoning.Planner.Pour.candidates ((__node).state)) : List (Reasoning.Planner.Plan.Node))
@[expose] public def Plan.successors (__node : Reasoning.Planner.Plan.Node) : List (Reasoning.Planner.Plan.Node) := (LexLeanRuntime.append (Reasoning.Planner.Plan.FillA.successors (__node)) ((LexLeanRuntime.append (Reasoning.Planner.Plan.EmptyB.successors (__node)) (Reasoning.Planner.Plan.Pour.successors (__node)) : List (Reasoning.Planner.Plan.Node))) : List (Reasoning.Planner.Plan.Node))
@[expose] public def Plan.fresh (__visited : List ((Prod (Nat) (Nat)))) (__nodes : List (Reasoning.Planner.Plan.Node)) : (Prod (List (Reasoning.Planner.Plan.Node)) (List ((Prod (Nat) (Nat))))) := (LexLeanCollections.listFold ((fun (__acc : (Prod (List (Reasoning.Planner.Plan.Node)) (List ((Prod (Nat) (Nat)))))) (__n : Reasoning.Planner.Plan.Node) => (if (LexLeanCollections.setContains ((__acc).2) ((__n).state) : Bool) then __acc else ((LexLeanRuntime.append ((__acc).1) ((__n :: ([] : List (Reasoning.Planner.Plan.Node)))) : List (Reasoning.Planner.Plan.Node)), (LexLeanCollections.setInsert ((__acc).2) ((__n).state) : List ((Prod (Nat) (Nat)))))))) ((([] : List (Reasoning.Planner.Plan.Node)), __visited)) (__nodes) : (Prod (List (Reasoning.Planner.Plan.Node)) (List ((Prod (Nat) (Nat))))))
@[expose] public def Plan.start (target : Nat) : Reasoning.Planner.Plan.Search := (let __first : List (Reasoning.Planner.Plan.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Reasoning.Planner.Plan.Node)) (__n : Reasoning.Planner.Plan.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) ((target + target))) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (Reasoning.Planner.Plan.Node)))) : List (Reasoning.Planner.Plan.Node)) else __acc))) (([] : List (Reasoning.Planner.Plan.Node))) ((({ state := Reasoning.Planner.Plan.observe (target), trace := ([] : List (Reasoning.Planner.Plan.Step)) } : Reasoning.Planner.Plan.Node) :: ([] : List (Reasoning.Planner.Plan.Node)))) : List (Reasoning.Planner.Plan.Node)); ({ frontier := __first, visited := (LexLeanCollections.setInsert (([] : List ((Prod (Nat) (Nat))))) (Reasoning.Planner.Plan.observe (target)) : List ((Prod (Nat) (Nat)))), found := Option.none, truncated := (Nat.blt ((target + target)) (1)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := (LexLeanRuntime.length (__first) : Nat) } : Reasoning.Planner.Plan.Ledger) } : Reasoning.Planner.Plan.Search))
@[expose] public def Plan.attempts (__s : (Prod (Nat) (Nat))) : Nat := (1 + (1 + (LexLeanRuntime.length (Reasoning.Planner.Pour.candidates (__s)) : Nat)))
@[expose] public def Plan.searchStep (target : Nat) (__r : Reasoning.Planner.Plan.Search) : Option (Reasoning.Planner.Plan.Search) := (match (__r).found with | Option.some _ => Option.none | Option.none => (match (__r).frontier with | List.nil => Option.none | List.cons __node __rest => (match Reasoning.Planner.Plan.accept (target) ((__node).state) with | Option.some __v => Option.some (({ frontier := __rest, visited := (__r).visited, found := Option.some ((__v, __node)), truncated := (__r).truncated, ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := ((__r).ledger).attempts, firings := ((__r).ledger).firings, expansions := ((__r).ledger).expansions, verifications := (((__r).ledger).verifications + 1), frontier := ((__r).ledger).frontier } : Reasoning.Planner.Plan.Ledger) } : Reasoning.Planner.Plan.Search)) | Option.none => (let __successors : List (Reasoning.Planner.Plan.Node) := Reasoning.Planner.Plan.successors (__node); (let __fresh : (Prod (List (Reasoning.Planner.Plan.Node)) (List ((Prod (Nat) (Nat))))) := Reasoning.Planner.Plan.fresh ((__r).visited) (__successors); (let __ordered : List (Reasoning.Planner.Plan.Node) := (LexLeanRuntime.append (__rest) ((__fresh).1) : List (Reasoning.Planner.Plan.Node)); (let __next : List (Reasoning.Planner.Plan.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Reasoning.Planner.Plan.Node)) (__n : Reasoning.Planner.Plan.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) ((target + target))) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (Reasoning.Planner.Plan.Node)))) : List (Reasoning.Planner.Plan.Node)) else __acc))) (([] : List (Reasoning.Planner.Plan.Node))) (__ordered) : List (Reasoning.Planner.Plan.Node)); Option.some (({ frontier := __next, visited := (__fresh).2, found := Option.none, truncated := ((__r).truncated || (Nat.blt ((target + target)) ((LexLeanRuntime.length (__ordered) : Nat)))), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + Reasoning.Planner.Plan.attempts ((__node).state)), firings := (((__r).ledger).firings + (LexLeanRuntime.length (__successors) : Nat)), expansions := (((__r).ledger).expansions + 1), verifications := (((__r).ledger).verifications + 1), frontier := (if (Nat.blt (((__r).ledger).frontier) ((LexLeanRuntime.length (__next) : Nat))) then (LexLeanRuntime.length (__next) : Nat) else ((__r).ledger).frontier) } : Reasoning.Planner.Plan.Ledger) } : Reasoning.Planner.Plan.Search)))))))))
@[expose] public def Plan.run (target : Nat) : (Prod (Reasoning.Planner.Plan.Search) (Bool)) := (LexLeanCollections.iterateUntil ((fun (__r : Reasoning.Planner.Plan.Search) => Reasoning.Planner.Plan.searchStep (target) (__r))) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Reasoning.Planner.Plan.start (target)) : (Prod (Reasoning.Planner.Plan.Search) (Bool)))
@[expose] public def Plan.failure (__saturated : Bool) (__truncated : Bool) : (Prod Bool Bool) := (if (__saturated && (!__truncated)) then ((false, true) : Prod Bool Bool) else ((false, false) : Prod Bool Bool))
@[expose] public def Plan (target : Nat) : Except ((Prod Bool Bool)) ((Prod ((Prod (Nat) (Nat))) (List (Reasoning.Planner.Plan.Step)))) := (let __final : (Prod (Reasoning.Planner.Plan.Search) (Bool)) := Reasoning.Planner.Plan.run (target); (match ((__final).1).found with | Option.some __hit => Except.ok (((__hit).1, ((__hit).2).trace)) | Option.none => Except.error (Reasoning.Planner.Plan.failure ((__final).2) (((__final).1).truncated))))
@[expose] public def Plan.verdict (target : Nat) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (match Reasoning.Planner.Plan (target) with | Except.ok __p => Except.ok ((__p).1) | Except.error __e => Except.error (__e))
public theorem Plan.fire_sound (__s : (Prod (Nat) (Nat))) (__step : Reasoning.Planner.Plan.Step) (__t : (Prod (Nat) (Nat))) : ((Reasoning.Planner.Plan.fire (__s) (__step) = Option.some (__t)) -> Reasoning.Planner.Moves (__s) (__t)) :=
by
  cases __step with
  | FillA =>
    exact Reasoning.Planner.FillA.apply_sound __s __t
  | EmptyB =>
    exact Reasoning.Planner.EmptyB.apply_sound __s __t
  | Pour __b =>
    exact Reasoning.Planner.Pour.apply_sound __s __b __t
public theorem Plan.replay_fire (__s : (Prod (Nat) (Nat))) (__step : Reasoning.Planner.Plan.Step) (__t : (Prod (Nat) (Nat))) : ((Reasoning.Planner.Plan.fire (__s) (__step) = Option.some (__t)) -> (Reasoning.Planner.Plan.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [Reasoning.Planner.Plan.replay]
  rw [llE]
public theorem Plan.replay_sound (target : Nat) (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : Reasoning.Planner.Plan.Step) : ((LexLeanReasoning.Reaches ((Reasoning.Planner.Moves)) (Reasoning.Planner.Plan.observe (target)) (__acc)) -> (LexLeanReasoning.Reaches ((Reasoning.Planner.Moves)) (Reasoning.Planner.Plan.observe (target)) (Reasoning.Planner.Plan.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [Reasoning.Planner.Plan.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (Reasoning.Planner.Plan.fire_sound llS __step _ ‹_›)
public theorem Plan.derivation (target : Nat) (__trace : List (Reasoning.Planner.Plan.Step)) : (LexLeanReasoning.Reaches ((Reasoning.Planner.Moves)) (Reasoning.Planner.Plan.observe (target)) (Reasoning.Planner.Plan.follow (target) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((Reasoning.Planner.Plan.replay)) (fun (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) => (LexLeanReasoning.Reaches ((Reasoning.Planner.Moves)) (Reasoning.Planner.Plan.observe (target)) (__acc))) (Reasoning.Planner.Plan.replay_sound (target)) (__trace) (Except.ok (Reasoning.Planner.Plan.observe (target))) (LexLeanReasoning.reachesStart ((Reasoning.Planner.Moves)) (Reasoning.Planner.Plan.observe (target))))
public theorem Plan.follow_invariant (target : Nat) (__trace : List (Reasoning.Planner.Plan.Step)) (__s : (Prod (Nat) (Nat))) : ((Reasoning.Planner.Plan.follow (target) (__trace) = Except.ok (__s)) -> Reasoning.Planner.Fits (__s)) :=
  (fun llE => (LexLeanReasoning.starPreserves ((Reasoning.Planner.Moves)) ((Reasoning.Planner.Fits)) Reasoning.Planner.Pouring.preserves (Reasoning.Planner.Plan.observe (target)) (__s) (Reasoning.Planner.Plan.derivation (target) (__trace) (__s) llE) (Reasoning.Planner.empty_jugs_fit (target))))
public theorem Plan.accept_sound (target : Nat) (__s : (Prod (Nat) (Nat))) (__v : (Prod (Nat) (Nat))) : ((Reasoning.Planner.Plan.accept (target) (__s) = Option.some (__v)) -> Reasoning.Planner.Measures (target) (__v)) :=
by
  intro llE
  dsimp only [Reasoning.Planner.Plan.accept] at llE
  split at llE
  · cases llE
  · have llC := LexLeanReasoning.checked _ _ _ llE
    exact llC.right ▸ Reasoning.Planner.measures_sound _ _ llC.left
public theorem Plan.extend (target : Nat) (__node : Reasoning.Planner.Plan.Node) (__step : Reasoning.Planner.Plan.Step) (__t : (Prod (Nat) (Nat))) : ((Reasoning.Planner.Plan.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> ((Reasoning.Planner.Plan.fire ((__node).state) (__step) = Option.some (__t)) -> (Reasoning.Planner.Plan.follow (target) ((LexLeanRuntime.append ((__node).trace) ((__step :: ([] : List (Reasoning.Planner.Plan.Step)))) : List (Reasoning.Planner.Plan.Step))) = Except.ok (__t)))) :=
by
  intro llH llE
  dsimp only [Reasoning.Planner.Plan.follow]
  rw [LexLeanReasoning.foldSnoc]
  dsimp only [Reasoning.Planner.Plan.follow] at llH
  rw [llH]
  exact Reasoning.Planner.Plan.replay_fire _ __step __t llE
public theorem Plan.FillA.successors_ok (target : Nat) (__node : Reasoning.Planner.Plan.Node) : ((Reasoning.Planner.Plan.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Reasoning.Planner.Plan.Node) => (Reasoning.Planner.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Reasoning.Planner.Plan.FillA.successors (__node)))) :=
by
  intro llH
  dsimp only [Reasoning.Planner.Plan.FillA.successors]
  split
  · exact LexLeanReasoning.All.nil
  · exact LexLeanReasoning.allSingle _ _ (Reasoning.Planner.Plan.extend _ __node _ _ llH ‹_›)
public theorem Plan.EmptyB.successors_ok (target : Nat) (__node : Reasoning.Planner.Plan.Node) : ((Reasoning.Planner.Plan.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Reasoning.Planner.Plan.Node) => (Reasoning.Planner.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Reasoning.Planner.Plan.EmptyB.successors (__node)))) :=
by
  intro llH
  dsimp only [Reasoning.Planner.Plan.EmptyB.successors]
  split
  · exact LexLeanReasoning.All.nil
  · exact LexLeanReasoning.allSingle _ _ (Reasoning.Planner.Plan.extend _ __node _ _ llH ‹_›)
public theorem Plan.Pour.successors_ok (target : Nat) (__node : Reasoning.Planner.Plan.Node) : ((Reasoning.Planner.Plan.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Reasoning.Planner.Plan.Node) => (Reasoning.Planner.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Reasoning.Planner.Plan.Pour.successors (__node)))) :=
by
  intro llH
  dsimp only [Reasoning.Planner.Plan.Pour.successors]
  exact LexLeanReasoning.foldInvariant _ (LexLeanReasoning.All (fun (llN : Reasoning.Planner.Plan.Node) => Reasoning.Planner.Plan.follow _ llN.trace = Except.ok llN.state))
    (fun llA llB llP => by
      dsimp only [Reasoning.Planner.Plan.Pour.collect]
      split
      · exact llP
      · exact LexLeanReasoning.allAppend _ _ _ llP (LexLeanReasoning.allSingle _ _ (Reasoning.Planner.Plan.extend _ __node _ _ llH ‹_›)))
    _ _ LexLeanReasoning.All.nil
public theorem Plan.successors_ok (target : Nat) (__node : Reasoning.Planner.Plan.Node) : ((Reasoning.Planner.Plan.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Reasoning.Planner.Plan.Node) => (Reasoning.Planner.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Reasoning.Planner.Plan.successors (__node)))) :=
  (fun llH => (LexLeanReasoning.allAppend _ _ _ (Reasoning.Planner.Plan.FillA.successors_ok (target) (__node) llH) (LexLeanReasoning.allAppend _ _ _ (Reasoning.Planner.Plan.EmptyB.successors_ok (target) (__node) llH) (Reasoning.Planner.Plan.Pour.successors_ok (target) (__node) llH))))
public theorem Plan.fresh_ok (target : Nat) (__visited : List ((Prod (Nat) (Nat)))) (__nodes : List (Reasoning.Planner.Plan.Node)) : ((LexLeanReasoning.All ((fun (__n : Reasoning.Planner.Plan.Node) => (Reasoning.Planner.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (__nodes)) -> (LexLeanReasoning.All ((fun (__n : Reasoning.Planner.Plan.Node) => (Reasoning.Planner.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((Reasoning.Planner.Plan.fresh (__visited) (__nodes)).1))) :=
  (LexLeanReasoning.freshAll ((fun (__n : Reasoning.Planner.Plan.Node) => (__n).state)) (fun (__n : Reasoning.Planner.Plan.Node) => (Reasoning.Planner.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state))) (__visited) (__nodes))
public theorem Plan.search_step (target : Nat) (__r : Reasoning.Planner.Plan.Search) (__q : Reasoning.Planner.Plan.Search) : ((Reasoning.Planner.Plan.searchStep (target) (__r) = Option.some (__q)) -> ((LexLeanReasoning.SearchOk ((fun (__n : Reasoning.Planner.Plan.Node) => (Reasoning.Planner.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Reasoning.Planner.Plan.Node) => Reasoning.Planner.Plan.accept (target) ((__n).state))) ((__r).frontier) ((__r).found)) -> (LexLeanReasoning.SearchOk ((fun (__n : Reasoning.Planner.Plan.Node) => (Reasoning.Planner.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Reasoning.Planner.Plan.Node) => Reasoning.Planner.Plan.accept (target) ((__n).state))) ((__q).frontier) ((__q).found)))) :=
by
  intro llE llH
  dsimp only [Reasoning.Planner.Plan.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · rename_i llNode llRest llFr
      have llAll := And.left llH
      rw [llFr] at llAll
      have llNodeOk := LexLeanReasoning.allHead _ llNode llRest llAll
      have llRestOk := LexLeanReasoning.allTail _ llNode llRest llAll
      split at llE
      · rename_i llV llAcc
        cases llE
        exact And.intro llRestOk (And.intro llNodeOk llAcc)
      · cases llE
        exact And.intro (LexLeanReasoning.capAll _ _ _ (LexLeanReasoning.allAppend _ _ _ llRestOk (Reasoning.Planner.Plan.fresh_ok _ _ _ (Reasoning.Planner.Plan.successors_ok _ llNode llNodeOk)))) True.intro
public theorem Plan.search_peak (target : Nat) (__r : Reasoning.Planner.Plan.Search) (__q : Reasoning.Planner.Plan.Search) : ((Reasoning.Planner.Plan.searchStep (target) (__r) = Option.some (__q)) -> ((((__r).ledger).frontier <= (target + target)) -> (((__q).ledger).frontier <= (target + target)))) :=
by
  intro llE llH
  dsimp only [Reasoning.Planner.Plan.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        exact llH
      · cases llE
        exact LexLeanReasoning.peakBound _ _ _ llH (LexLeanReasoning.capBound _ _)
public theorem Plan.search_count (target : Nat) (__r : Reasoning.Planner.Plan.Search) (__q : Reasoning.Planner.Plan.Search) : ((Reasoning.Planner.Plan.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Planner.Plan.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        rfl
      · cases llE
        rfl
public theorem Plan.search_verify_growth (target : Nat) (__r : Reasoning.Planner.Plan.Search) (__q : Reasoning.Planner.Plan.Search) : ((Reasoning.Planner.Plan.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).verifications <= (((__r).ledger).verifications + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Planner.Plan.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem Plan.search_expanded (target : Nat) (__r : Reasoning.Planner.Plan.Search) (__q : Reasoning.Planner.Plan.Search) : ((Reasoning.Planner.Plan.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).expansions <= (((__r).ledger).expansions + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Planner.Plan.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem Plan.search_ok (target : Nat) : (LexLeanReasoning.SearchOk ((fun (__n : Reasoning.Planner.Plan.Node) => (Reasoning.Planner.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Reasoning.Planner.Plan.Node) => Reasoning.Planner.Plan.accept (target) ((__n).state))) (((Reasoning.Planner.Plan.run (target)).1).frontier) (((Reasoning.Planner.Plan.run (target)).1).found)) :=
by
  dsimp only [Reasoning.Planner.Plan.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((fun (__r : Reasoning.Planner.Plan.Search) => Reasoning.Planner.Plan.searchStep (target) (__r))) (fun (__r : Reasoning.Planner.Plan.Search) => (LexLeanReasoning.SearchOk ((fun (__n : Reasoning.Planner.Plan.Node) => (Reasoning.Planner.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Reasoning.Planner.Plan.Node) => Reasoning.Planner.Plan.accept (target) ((__n).state))) ((__r).frontier) ((__r).found))) (Reasoning.Planner.Plan.search_step (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Reasoning.Planner.Plan.start (target)) (LexLeanReasoning.searchStart ((fun (__n : Reasoning.Planner.Plan.Node) => (Reasoning.Planner.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Reasoning.Planner.Plan.Node) => Reasoning.Planner.Plan.accept (target) ((__n).state))) _ (LexLeanReasoning.capAll ((fun (__n : Reasoning.Planner.Plan.Node) => (Reasoning.Planner.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((target + target)) _ (LexLeanReasoning.allSingle ((fun (__n : Reasoning.Planner.Plan.Node) => (Reasoning.Planner.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (({ state := Reasoning.Planner.Plan.observe (target), trace := ([] : List (Reasoning.Planner.Plan.Step)) } : Reasoning.Planner.Plan.Node)) rfl))))
public theorem Plan.frontier_bounded (target : Nat) : ((((Reasoning.Planner.Plan.run (target)).1).ledger).frontier <= (target + target)) :=
by
  dsimp only [Reasoning.Planner.Plan.run]
  exact (LexLeanReasoning.iterateUntilBound ((fun (__r : Reasoning.Planner.Plan.Search) => Reasoning.Planner.Plan.searchStep (target) (__r))) ((fun (__r : Reasoning.Planner.Plan.Search) => ((__r).ledger).frontier)) ((target + target)) (Reasoning.Planner.Plan.search_peak (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Reasoning.Planner.Plan.start (target)) (LexLeanReasoning.capBound ((target + target)) _))
public theorem Plan.verifications_bounded (target : Nat) : ((((Reasoning.Planner.Plan.run (target)).1).ledger).verifications <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [Reasoning.Planner.Plan.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : Reasoning.Planner.Plan.Search) => Reasoning.Planner.Plan.searchStep (target) (__r))) ((fun (__r : Reasoning.Planner.Plan.Search) => ((__r).ledger).verifications)) (Reasoning.Planner.Plan.search_verify_growth (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Reasoning.Planner.Plan.start (target)) rfl)
public theorem Plan.expansions_bounded (target : Nat) : ((((Reasoning.Planner.Plan.run (target)).1).ledger).expansions <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [Reasoning.Planner.Plan.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : Reasoning.Planner.Plan.Search) => Reasoning.Planner.Plan.searchStep (target) (__r))) ((fun (__r : Reasoning.Planner.Plan.Search) => ((__r).ledger).expansions)) (Reasoning.Planner.Plan.search_expanded (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Reasoning.Planner.Plan.start (target)) rfl)
public theorem Plan.iterations_bounded (target : Nat) : ((((Reasoning.Planner.Plan.run (target)).1).ledger).iterations <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [Reasoning.Planner.Plan.run]
  exact (LexLeanReasoning.iterateUntilCount ((fun (__r : Reasoning.Planner.Plan.Search) => Reasoning.Planner.Plan.searchStep (target) (__r))) ((fun (__r : Reasoning.Planner.Plan.Search) => ((__r).ledger).iterations)) (Reasoning.Planner.Plan.search_count (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Reasoning.Planner.Plan.start (target)) rfl)
public theorem Plan.explained (target : Nat) : (forall (__v : (Prod (Nat) (Nat))), (forall (__trace : List (Reasoning.Planner.Plan.Step)), ((Reasoning.Planner.Plan (target) = Except.ok ((__v, __trace))) -> ((Reasoning.Planner.Plan.answer (target) (__trace) = Option.some (__v)) /\ Reasoning.Planner.Measures (target) (__v))))) :=
by
  intro llV llT llE
  have llSearch := Reasoning.Planner.Plan.search_ok target
  dsimp only [Reasoning.Planner.Plan] at llE
  generalize llRun : Reasoning.Planner.Plan.run target = llR at llE llSearch
  split at llE
  · rename_i llHit llF
    cases llE
    have llOk := And.right llSearch
    rw [llF] at llOk
    exact And.intro (by dsimp only [Reasoning.Planner.Plan.answer]; rw [And.left llOk]; exact And.right llOk) (Reasoning.Planner.Plan.accept_sound _ _ _ (And.right llOk))
  · cases llE
public theorem Plan.verdict_sound (target : Nat) : (forall (__v : (Prod (Nat) (Nat))), ((Reasoning.Planner.Plan.verdict (target) = Except.ok (__v)) -> Reasoning.Planner.Measures (target) (__v))) :=
by
  intro llV llE
  dsimp only [Reasoning.Planner.Plan.verdict] at llE
  generalize llHP : Reasoning.Planner.Plan target = llR at llE
  cases llR with
  | error _ => cases llE
  | ok llP =>
    cases llE
    exact And.right (Reasoning.Planner.Plan.explained _ llP.1 llP.2 llHP)

@[expose, reducible] public def Within (_target : Nat) (r : Nat) : Prop := (r <= 4)

@[expose] public def withinCheck (_target : Nat) (r : Nat) : Bool := (Nat.ble (r) (4))

public theorem within_sound (target : Nat) (r : Nat) : ((withinCheck (target) (r) = true) -> Within (target) (r)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Within, withinCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem Inside.sound : (LexLeanReasoning.Sound ((Reasoning.Planner.withinCheck)) ((Reasoning.Planner.Within))) :=
  Reasoning.Planner.within_sound

@[expose, reducible] public def Whole (_target : Nat) (r : Nat) : Prop := (0 <= r)

@[expose] public def wholeCheck (_target : Nat) (r : Nat) : Bool := (Nat.ble (0) (r))

public theorem whole_sound (target : Nat) (r : Nat) : ((wholeCheck (target) (r) = true) -> Whole (target) (r)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Whole, wholeCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem Anywhere.sound : (LexLeanReasoning.Sound ((Reasoning.Planner.wholeCheck)) ((Reasoning.Planner.Whole))) :=
  Reasoning.Planner.whole_sound

public theorem plan_fit_correct (target : Nat) (s : (Prod (Nat) (Nat))) (v : Nat) : (Fits (s) -> ((Option.some ((s).1) = Option.some (v)) -> Within (target) (v))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Fits, Within, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

@[expose, reducible] public def Held (target : Nat) (s : (Prod (Nat) (Nat))) : Prop := (Fits (s) /\ (((s).1 = target) -> (target <= 4)))

public theorem held_initial (target : Nat) : Held (target) ((0, 0)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Fits, Held, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem held_preserved (target : Nat) (s : (Prod (Nat) (Nat))) (t : (Prod (Nat) (Nat))) : (Held (target) (s) -> (Moves (s) (t) -> Held (target) (t))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Fits, Held, Moves, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem plan_held_correct (target : Nat) (s : (Prod (Nat) (Nat))) (v : Nat) : (Held (target) (s) -> ((Option.some ((s).1) = Option.some (v)) -> Within (target) (v))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Fits, Held, Within, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem plan_whole_correct (target : Nat) (s : (Prod (Nat) (Nat))) (v : Nat) : ((Option.some ((s).1) = Option.some (v)) -> Whole (target) (v)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Whole, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public inductive PlanFit.Step where
  | FillA
  | EmptyB
  | Pour (_ : Nat)
public structure PlanFit.Ledger where
  iterations : Nat
  attempts : Nat
  firings : Nat
  expansions : Nat
  verifications : Nat
  frontier : Nat
@[expose] public def PlanFit.observe (_target : Nat) : (Prod (Nat) (Nat)) := (0, 0)
@[expose] public def PlanFit.fire (__s : (Prod (Nat) (Nat))) (__step : Reasoning.Planner.PlanFit.Step) : Option ((Prod (Nat) (Nat))) := (match __step with | Reasoning.Planner.PlanFit.Step.FillA => Reasoning.Planner.FillA.apply (__s) | Reasoning.Planner.PlanFit.Step.EmptyB => Reasoning.Planner.EmptyB.apply (__s) | Reasoning.Planner.PlanFit.Step.Pour __b => Reasoning.Planner.Pour.apply (__s) (__b))
@[expose] public def PlanFit.replay (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : Reasoning.Planner.PlanFit.Step) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match Reasoning.Planner.PlanFit.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def PlanFit.follow (target : Nat) (__trace : List (Reasoning.Planner.PlanFit.Step)) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (LexLeanCollections.listFold ((Reasoning.Planner.PlanFit.replay)) (Except.ok (Reasoning.Planner.PlanFit.observe (target))) (__trace) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))))
@[expose] public def PlanFit.extract (jugs : (Prod (Nat) (Nat))) : Option (Nat) := Option.some ((jugs).1)
@[expose] public def PlanFit.accept (_target : Nat) (__s : (Prod (Nat) (Nat))) : Option (Nat) := Reasoning.Planner.PlanFit.extract (__s)
@[expose] public def PlanFit.answer (target : Nat) (__trace : List (Reasoning.Planner.PlanFit.Step)) : Option (Nat) := (match Reasoning.Planner.PlanFit.follow (target) (__trace) with | Except.ok __s => Reasoning.Planner.PlanFit.accept (target) (__s) | Except.error _ => Option.none)
public structure PlanFit.Node where
  state : (Prod (Nat) (Nat))
  trace : List (Reasoning.Planner.PlanFit.Step)
public structure PlanFit.Search where
  frontier : List (Reasoning.Planner.PlanFit.Node)
  visited : List ((Prod (Nat) (Nat)))
  found : Option ((Prod (Nat) (Reasoning.Planner.PlanFit.Node)))
  truncated : Bool
  ledger : Reasoning.Planner.PlanFit.Ledger
@[expose] public def PlanFit.FillA.successors (__node : Reasoning.Planner.PlanFit.Node) : List (Reasoning.Planner.PlanFit.Node) := (match Reasoning.Planner.PlanFit.fire ((__node).state) (Reasoning.Planner.PlanFit.Step.FillA) with | Option.none => ([] : List (Reasoning.Planner.PlanFit.Node)) | Option.some __t => (({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((Reasoning.Planner.PlanFit.Step.FillA :: ([] : List (Reasoning.Planner.PlanFit.Step)))) : List (Reasoning.Planner.PlanFit.Step)) } : Reasoning.Planner.PlanFit.Node) :: ([] : List (Reasoning.Planner.PlanFit.Node))))
@[expose] public def PlanFit.EmptyB.successors (__node : Reasoning.Planner.PlanFit.Node) : List (Reasoning.Planner.PlanFit.Node) := (match Reasoning.Planner.PlanFit.fire ((__node).state) (Reasoning.Planner.PlanFit.Step.EmptyB) with | Option.none => ([] : List (Reasoning.Planner.PlanFit.Node)) | Option.some __t => (({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((Reasoning.Planner.PlanFit.Step.EmptyB :: ([] : List (Reasoning.Planner.PlanFit.Step)))) : List (Reasoning.Planner.PlanFit.Step)) } : Reasoning.Planner.PlanFit.Node) :: ([] : List (Reasoning.Planner.PlanFit.Node))))
@[expose] public def PlanFit.Pour.collect (__node : Reasoning.Planner.PlanFit.Node) (__acc : List (Reasoning.Planner.PlanFit.Node)) (__b : Nat) : List (Reasoning.Planner.PlanFit.Node) := (match Reasoning.Planner.PlanFit.fire ((__node).state) (Reasoning.Planner.PlanFit.Step.Pour (__b)) with | Option.none => __acc | Option.some __t => (LexLeanRuntime.append (__acc) ((({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((Reasoning.Planner.PlanFit.Step.Pour (__b) :: ([] : List (Reasoning.Planner.PlanFit.Step)))) : List (Reasoning.Planner.PlanFit.Step)) } : Reasoning.Planner.PlanFit.Node) :: ([] : List (Reasoning.Planner.PlanFit.Node)))) : List (Reasoning.Planner.PlanFit.Node)))
@[expose] public def PlanFit.Pour.successors (__node : Reasoning.Planner.PlanFit.Node) : List (Reasoning.Planner.PlanFit.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Reasoning.Planner.PlanFit.Node)) (__b : Nat) => Reasoning.Planner.PlanFit.Pour.collect (__node) (__acc) (__b))) (([] : List (Reasoning.Planner.PlanFit.Node))) (Reasoning.Planner.Pour.candidates ((__node).state)) : List (Reasoning.Planner.PlanFit.Node))
@[expose] public def PlanFit.successors (__node : Reasoning.Planner.PlanFit.Node) : List (Reasoning.Planner.PlanFit.Node) := (LexLeanRuntime.append (Reasoning.Planner.PlanFit.FillA.successors (__node)) ((LexLeanRuntime.append (Reasoning.Planner.PlanFit.EmptyB.successors (__node)) (Reasoning.Planner.PlanFit.Pour.successors (__node)) : List (Reasoning.Planner.PlanFit.Node))) : List (Reasoning.Planner.PlanFit.Node))
@[expose] public def PlanFit.fresh (__visited : List ((Prod (Nat) (Nat)))) (__nodes : List (Reasoning.Planner.PlanFit.Node)) : (Prod (List (Reasoning.Planner.PlanFit.Node)) (List ((Prod (Nat) (Nat))))) := (LexLeanCollections.listFold ((fun (__acc : (Prod (List (Reasoning.Planner.PlanFit.Node)) (List ((Prod (Nat) (Nat)))))) (__n : Reasoning.Planner.PlanFit.Node) => (if (LexLeanCollections.setContains ((__acc).2) ((__n).state) : Bool) then __acc else ((LexLeanRuntime.append ((__acc).1) ((__n :: ([] : List (Reasoning.Planner.PlanFit.Node)))) : List (Reasoning.Planner.PlanFit.Node)), (LexLeanCollections.setInsert ((__acc).2) ((__n).state) : List ((Prod (Nat) (Nat)))))))) ((([] : List (Reasoning.Planner.PlanFit.Node)), __visited)) (__nodes) : (Prod (List (Reasoning.Planner.PlanFit.Node)) (List ((Prod (Nat) (Nat))))))
@[expose] public def PlanFit.start (target : Nat) : Reasoning.Planner.PlanFit.Search := (let __first : List (Reasoning.Planner.PlanFit.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Reasoning.Planner.PlanFit.Node)) (__n : Reasoning.Planner.PlanFit.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) ((target + target))) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (Reasoning.Planner.PlanFit.Node)))) : List (Reasoning.Planner.PlanFit.Node)) else __acc))) (([] : List (Reasoning.Planner.PlanFit.Node))) ((({ state := Reasoning.Planner.PlanFit.observe (target), trace := ([] : List (Reasoning.Planner.PlanFit.Step)) } : Reasoning.Planner.PlanFit.Node) :: ([] : List (Reasoning.Planner.PlanFit.Node)))) : List (Reasoning.Planner.PlanFit.Node)); ({ frontier := __first, visited := (LexLeanCollections.setInsert (([] : List ((Prod (Nat) (Nat))))) (Reasoning.Planner.PlanFit.observe (target)) : List ((Prod (Nat) (Nat)))), found := Option.none, truncated := (Nat.blt ((target + target)) (1)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := (LexLeanRuntime.length (__first) : Nat) } : Reasoning.Planner.PlanFit.Ledger) } : Reasoning.Planner.PlanFit.Search))
@[expose] public def PlanFit.attempts (__s : (Prod (Nat) (Nat))) : Nat := (1 + (1 + (LexLeanRuntime.length (Reasoning.Planner.Pour.candidates (__s)) : Nat)))
@[expose] public def PlanFit.searchStep (target : Nat) (__r : Reasoning.Planner.PlanFit.Search) : Option (Reasoning.Planner.PlanFit.Search) := (match (__r).found with | Option.some _ => Option.none | Option.none => (match (__r).frontier with | List.nil => Option.none | List.cons __node __rest => (match Reasoning.Planner.PlanFit.accept (target) ((__node).state) with | Option.some __v => Option.some (({ frontier := __rest, visited := (__r).visited, found := Option.some ((__v, __node)), truncated := (__r).truncated, ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := ((__r).ledger).attempts, firings := ((__r).ledger).firings, expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : Reasoning.Planner.PlanFit.Ledger) } : Reasoning.Planner.PlanFit.Search)) | Option.none => (let __successors : List (Reasoning.Planner.PlanFit.Node) := Reasoning.Planner.PlanFit.successors (__node); (let __fresh : (Prod (List (Reasoning.Planner.PlanFit.Node)) (List ((Prod (Nat) (Nat))))) := Reasoning.Planner.PlanFit.fresh ((__r).visited) (__successors); (let __ordered : List (Reasoning.Planner.PlanFit.Node) := (LexLeanRuntime.append (__rest) ((__fresh).1) : List (Reasoning.Planner.PlanFit.Node)); (let __next : List (Reasoning.Planner.PlanFit.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Reasoning.Planner.PlanFit.Node)) (__n : Reasoning.Planner.PlanFit.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) ((target + target))) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (Reasoning.Planner.PlanFit.Node)))) : List (Reasoning.Planner.PlanFit.Node)) else __acc))) (([] : List (Reasoning.Planner.PlanFit.Node))) (__ordered) : List (Reasoning.Planner.PlanFit.Node)); Option.some (({ frontier := __next, visited := (__fresh).2, found := Option.none, truncated := ((__r).truncated || (Nat.blt ((target + target)) ((LexLeanRuntime.length (__ordered) : Nat)))), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + Reasoning.Planner.PlanFit.attempts ((__node).state)), firings := (((__r).ledger).firings + (LexLeanRuntime.length (__successors) : Nat)), expansions := (((__r).ledger).expansions + 1), verifications := ((__r).ledger).verifications, frontier := (if (Nat.blt (((__r).ledger).frontier) ((LexLeanRuntime.length (__next) : Nat))) then (LexLeanRuntime.length (__next) : Nat) else ((__r).ledger).frontier) } : Reasoning.Planner.PlanFit.Ledger) } : Reasoning.Planner.PlanFit.Search)))))))))
@[expose] public def PlanFit.run (target : Nat) : (Prod (Reasoning.Planner.PlanFit.Search) (Bool)) := (LexLeanCollections.iterateUntil ((fun (__r : Reasoning.Planner.PlanFit.Search) => Reasoning.Planner.PlanFit.searchStep (target) (__r))) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Reasoning.Planner.PlanFit.start (target)) : (Prod (Reasoning.Planner.PlanFit.Search) (Bool)))
@[expose] public def PlanFit.failure (__saturated : Bool) (__truncated : Bool) : (Prod Bool Bool) := (if (__saturated && (!__truncated)) then ((false, true) : Prod Bool Bool) else ((false, false) : Prod Bool Bool))
@[expose] public def PlanFit (target : Nat) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (Reasoning.Planner.PlanFit.Step)))) := (let __final : (Prod (Reasoning.Planner.PlanFit.Search) (Bool)) := Reasoning.Planner.PlanFit.run (target); (match ((__final).1).found with | Option.some __hit => Except.ok (((__hit).1, ((__hit).2).trace)) | Option.none => Except.error (Reasoning.Planner.PlanFit.failure ((__final).2) (((__final).1).truncated))))
@[expose] public def PlanFit.verdict (target : Nat) : Except ((Prod Bool Bool)) (Nat) := (match Reasoning.Planner.PlanFit (target) with | Except.ok __p => Except.ok ((__p).1) | Except.error __e => Except.error (__e))
public theorem PlanFit.fire_sound (__s : (Prod (Nat) (Nat))) (__step : Reasoning.Planner.PlanFit.Step) (__t : (Prod (Nat) (Nat))) : ((Reasoning.Planner.PlanFit.fire (__s) (__step) = Option.some (__t)) -> Reasoning.Planner.Moves (__s) (__t)) :=
by
  cases __step with
  | FillA =>
    exact Reasoning.Planner.FillA.apply_sound __s __t
  | EmptyB =>
    exact Reasoning.Planner.EmptyB.apply_sound __s __t
  | Pour __b =>
    exact Reasoning.Planner.Pour.apply_sound __s __b __t
public theorem PlanFit.replay_fire (__s : (Prod (Nat) (Nat))) (__step : Reasoning.Planner.PlanFit.Step) (__t : (Prod (Nat) (Nat))) : ((Reasoning.Planner.PlanFit.fire (__s) (__step) = Option.some (__t)) -> (Reasoning.Planner.PlanFit.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [Reasoning.Planner.PlanFit.replay]
  rw [llE]
public theorem PlanFit.replay_sound (target : Nat) (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : Reasoning.Planner.PlanFit.Step) : ((LexLeanReasoning.Reaches ((Reasoning.Planner.Moves)) (Reasoning.Planner.PlanFit.observe (target)) (__acc)) -> (LexLeanReasoning.Reaches ((Reasoning.Planner.Moves)) (Reasoning.Planner.PlanFit.observe (target)) (Reasoning.Planner.PlanFit.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [Reasoning.Planner.PlanFit.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (Reasoning.Planner.PlanFit.fire_sound llS __step _ ‹_›)
public theorem PlanFit.derivation (target : Nat) (__trace : List (Reasoning.Planner.PlanFit.Step)) : (LexLeanReasoning.Reaches ((Reasoning.Planner.Moves)) (Reasoning.Planner.PlanFit.observe (target)) (Reasoning.Planner.PlanFit.follow (target) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((Reasoning.Planner.PlanFit.replay)) (fun (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) => (LexLeanReasoning.Reaches ((Reasoning.Planner.Moves)) (Reasoning.Planner.PlanFit.observe (target)) (__acc))) (Reasoning.Planner.PlanFit.replay_sound (target)) (__trace) (Except.ok (Reasoning.Planner.PlanFit.observe (target))) (LexLeanReasoning.reachesStart ((Reasoning.Planner.Moves)) (Reasoning.Planner.PlanFit.observe (target))))
public theorem PlanFit.follow_invariant (target : Nat) (__trace : List (Reasoning.Planner.PlanFit.Step)) (__s : (Prod (Nat) (Nat))) : ((Reasoning.Planner.PlanFit.follow (target) (__trace) = Except.ok (__s)) -> Reasoning.Planner.Fits (__s)) :=
  (fun llE => (LexLeanReasoning.starPreserves ((Reasoning.Planner.Moves)) ((Reasoning.Planner.Fits)) Reasoning.Planner.Pouring.preserves (Reasoning.Planner.PlanFit.observe (target)) (__s) (Reasoning.Planner.PlanFit.derivation (target) (__trace) (__s) llE) (Reasoning.Planner.empty_jugs_fit (target))))
public theorem PlanFit.accept_sound (target : Nat) (__s : (Prod (Nat) (Nat))) (__v : Nat) : (Reasoning.Planner.Fits (__s) -> ((Reasoning.Planner.PlanFit.accept (target) (__s) = Option.some (__v)) -> Reasoning.Planner.Within (target) (__v))) :=
  (fun llJ llE => (Reasoning.Planner.plan_fit_correct (target) (__s) (__v) llJ llE))
public theorem PlanFit.extend (target : Nat) (__node : Reasoning.Planner.PlanFit.Node) (__step : Reasoning.Planner.PlanFit.Step) (__t : (Prod (Nat) (Nat))) : ((Reasoning.Planner.PlanFit.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> ((Reasoning.Planner.PlanFit.fire ((__node).state) (__step) = Option.some (__t)) -> (Reasoning.Planner.PlanFit.follow (target) ((LexLeanRuntime.append ((__node).trace) ((__step :: ([] : List (Reasoning.Planner.PlanFit.Step)))) : List (Reasoning.Planner.PlanFit.Step))) = Except.ok (__t)))) :=
by
  intro llH llE
  dsimp only [Reasoning.Planner.PlanFit.follow]
  rw [LexLeanReasoning.foldSnoc]
  dsimp only [Reasoning.Planner.PlanFit.follow] at llH
  rw [llH]
  exact Reasoning.Planner.PlanFit.replay_fire _ __step __t llE
public theorem PlanFit.FillA.successors_ok (target : Nat) (__node : Reasoning.Planner.PlanFit.Node) : ((Reasoning.Planner.PlanFit.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Reasoning.Planner.PlanFit.Node) => (Reasoning.Planner.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Reasoning.Planner.PlanFit.FillA.successors (__node)))) :=
by
  intro llH
  dsimp only [Reasoning.Planner.PlanFit.FillA.successors]
  split
  · exact LexLeanReasoning.All.nil
  · exact LexLeanReasoning.allSingle _ _ (Reasoning.Planner.PlanFit.extend _ __node _ _ llH ‹_›)
public theorem PlanFit.EmptyB.successors_ok (target : Nat) (__node : Reasoning.Planner.PlanFit.Node) : ((Reasoning.Planner.PlanFit.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Reasoning.Planner.PlanFit.Node) => (Reasoning.Planner.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Reasoning.Planner.PlanFit.EmptyB.successors (__node)))) :=
by
  intro llH
  dsimp only [Reasoning.Planner.PlanFit.EmptyB.successors]
  split
  · exact LexLeanReasoning.All.nil
  · exact LexLeanReasoning.allSingle _ _ (Reasoning.Planner.PlanFit.extend _ __node _ _ llH ‹_›)
public theorem PlanFit.Pour.successors_ok (target : Nat) (__node : Reasoning.Planner.PlanFit.Node) : ((Reasoning.Planner.PlanFit.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Reasoning.Planner.PlanFit.Node) => (Reasoning.Planner.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Reasoning.Planner.PlanFit.Pour.successors (__node)))) :=
by
  intro llH
  dsimp only [Reasoning.Planner.PlanFit.Pour.successors]
  exact LexLeanReasoning.foldInvariant _ (LexLeanReasoning.All (fun (llN : Reasoning.Planner.PlanFit.Node) => Reasoning.Planner.PlanFit.follow _ llN.trace = Except.ok llN.state))
    (fun llA llB llP => by
      dsimp only [Reasoning.Planner.PlanFit.Pour.collect]
      split
      · exact llP
      · exact LexLeanReasoning.allAppend _ _ _ llP (LexLeanReasoning.allSingle _ _ (Reasoning.Planner.PlanFit.extend _ __node _ _ llH ‹_›)))
    _ _ LexLeanReasoning.All.nil
public theorem PlanFit.successors_ok (target : Nat) (__node : Reasoning.Planner.PlanFit.Node) : ((Reasoning.Planner.PlanFit.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Reasoning.Planner.PlanFit.Node) => (Reasoning.Planner.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Reasoning.Planner.PlanFit.successors (__node)))) :=
  (fun llH => (LexLeanReasoning.allAppend _ _ _ (Reasoning.Planner.PlanFit.FillA.successors_ok (target) (__node) llH) (LexLeanReasoning.allAppend _ _ _ (Reasoning.Planner.PlanFit.EmptyB.successors_ok (target) (__node) llH) (Reasoning.Planner.PlanFit.Pour.successors_ok (target) (__node) llH))))
public theorem PlanFit.fresh_ok (target : Nat) (__visited : List ((Prod (Nat) (Nat)))) (__nodes : List (Reasoning.Planner.PlanFit.Node)) : ((LexLeanReasoning.All ((fun (__n : Reasoning.Planner.PlanFit.Node) => (Reasoning.Planner.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (__nodes)) -> (LexLeanReasoning.All ((fun (__n : Reasoning.Planner.PlanFit.Node) => (Reasoning.Planner.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((Reasoning.Planner.PlanFit.fresh (__visited) (__nodes)).1))) :=
  (LexLeanReasoning.freshAll ((fun (__n : Reasoning.Planner.PlanFit.Node) => (__n).state)) (fun (__n : Reasoning.Planner.PlanFit.Node) => (Reasoning.Planner.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state))) (__visited) (__nodes))
public theorem PlanFit.search_step (target : Nat) (__r : Reasoning.Planner.PlanFit.Search) (__q : Reasoning.Planner.PlanFit.Search) : ((Reasoning.Planner.PlanFit.searchStep (target) (__r) = Option.some (__q)) -> ((LexLeanReasoning.SearchOk ((fun (__n : Reasoning.Planner.PlanFit.Node) => (Reasoning.Planner.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Reasoning.Planner.PlanFit.Node) => Reasoning.Planner.PlanFit.accept (target) ((__n).state))) ((__r).frontier) ((__r).found)) -> (LexLeanReasoning.SearchOk ((fun (__n : Reasoning.Planner.PlanFit.Node) => (Reasoning.Planner.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Reasoning.Planner.PlanFit.Node) => Reasoning.Planner.PlanFit.accept (target) ((__n).state))) ((__q).frontier) ((__q).found)))) :=
by
  intro llE llH
  dsimp only [Reasoning.Planner.PlanFit.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · rename_i llNode llRest llFr
      have llAll := And.left llH
      rw [llFr] at llAll
      have llNodeOk := LexLeanReasoning.allHead _ llNode llRest llAll
      have llRestOk := LexLeanReasoning.allTail _ llNode llRest llAll
      split at llE
      · rename_i llV llAcc
        cases llE
        exact And.intro llRestOk (And.intro llNodeOk llAcc)
      · cases llE
        exact And.intro (LexLeanReasoning.capAll _ _ _ (LexLeanReasoning.allAppend _ _ _ llRestOk (Reasoning.Planner.PlanFit.fresh_ok _ _ _ (Reasoning.Planner.PlanFit.successors_ok _ llNode llNodeOk)))) True.intro
public theorem PlanFit.search_peak (target : Nat) (__r : Reasoning.Planner.PlanFit.Search) (__q : Reasoning.Planner.PlanFit.Search) : ((Reasoning.Planner.PlanFit.searchStep (target) (__r) = Option.some (__q)) -> ((((__r).ledger).frontier <= (target + target)) -> (((__q).ledger).frontier <= (target + target)))) :=
by
  intro llE llH
  dsimp only [Reasoning.Planner.PlanFit.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        exact llH
      · cases llE
        exact LexLeanReasoning.peakBound _ _ _ llH (LexLeanReasoning.capBound _ _)
public theorem PlanFit.search_count (target : Nat) (__r : Reasoning.Planner.PlanFit.Search) (__q : Reasoning.Planner.PlanFit.Search) : ((Reasoning.Planner.PlanFit.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Planner.PlanFit.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        rfl
      · cases llE
        rfl
public theorem PlanFit.search_verify_growth (target : Nat) (__r : Reasoning.Planner.PlanFit.Search) (__q : Reasoning.Planner.PlanFit.Search) : ((Reasoning.Planner.PlanFit.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).verifications <= (((__r).ledger).verifications + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Planner.PlanFit.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem PlanFit.search_expanded (target : Nat) (__r : Reasoning.Planner.PlanFit.Search) (__q : Reasoning.Planner.PlanFit.Search) : ((Reasoning.Planner.PlanFit.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).expansions <= (((__r).ledger).expansions + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Planner.PlanFit.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem PlanFit.search_ok (target : Nat) : (LexLeanReasoning.SearchOk ((fun (__n : Reasoning.Planner.PlanFit.Node) => (Reasoning.Planner.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Reasoning.Planner.PlanFit.Node) => Reasoning.Planner.PlanFit.accept (target) ((__n).state))) (((Reasoning.Planner.PlanFit.run (target)).1).frontier) (((Reasoning.Planner.PlanFit.run (target)).1).found)) :=
by
  dsimp only [Reasoning.Planner.PlanFit.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((fun (__r : Reasoning.Planner.PlanFit.Search) => Reasoning.Planner.PlanFit.searchStep (target) (__r))) (fun (__r : Reasoning.Planner.PlanFit.Search) => (LexLeanReasoning.SearchOk ((fun (__n : Reasoning.Planner.PlanFit.Node) => (Reasoning.Planner.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Reasoning.Planner.PlanFit.Node) => Reasoning.Planner.PlanFit.accept (target) ((__n).state))) ((__r).frontier) ((__r).found))) (Reasoning.Planner.PlanFit.search_step (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Reasoning.Planner.PlanFit.start (target)) (LexLeanReasoning.searchStart ((fun (__n : Reasoning.Planner.PlanFit.Node) => (Reasoning.Planner.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Reasoning.Planner.PlanFit.Node) => Reasoning.Planner.PlanFit.accept (target) ((__n).state))) _ (LexLeanReasoning.capAll ((fun (__n : Reasoning.Planner.PlanFit.Node) => (Reasoning.Planner.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((target + target)) _ (LexLeanReasoning.allSingle ((fun (__n : Reasoning.Planner.PlanFit.Node) => (Reasoning.Planner.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (({ state := Reasoning.Planner.PlanFit.observe (target), trace := ([] : List (Reasoning.Planner.PlanFit.Step)) } : Reasoning.Planner.PlanFit.Node)) rfl))))
public theorem PlanFit.frontier_bounded (target : Nat) : ((((Reasoning.Planner.PlanFit.run (target)).1).ledger).frontier <= (target + target)) :=
by
  dsimp only [Reasoning.Planner.PlanFit.run]
  exact (LexLeanReasoning.iterateUntilBound ((fun (__r : Reasoning.Planner.PlanFit.Search) => Reasoning.Planner.PlanFit.searchStep (target) (__r))) ((fun (__r : Reasoning.Planner.PlanFit.Search) => ((__r).ledger).frontier)) ((target + target)) (Reasoning.Planner.PlanFit.search_peak (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Reasoning.Planner.PlanFit.start (target)) (LexLeanReasoning.capBound ((target + target)) _))
public theorem PlanFit.verifications_bounded (target : Nat) : ((((Reasoning.Planner.PlanFit.run (target)).1).ledger).verifications <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [Reasoning.Planner.PlanFit.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : Reasoning.Planner.PlanFit.Search) => Reasoning.Planner.PlanFit.searchStep (target) (__r))) ((fun (__r : Reasoning.Planner.PlanFit.Search) => ((__r).ledger).verifications)) (Reasoning.Planner.PlanFit.search_verify_growth (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Reasoning.Planner.PlanFit.start (target)) rfl)
public theorem PlanFit.expansions_bounded (target : Nat) : ((((Reasoning.Planner.PlanFit.run (target)).1).ledger).expansions <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [Reasoning.Planner.PlanFit.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : Reasoning.Planner.PlanFit.Search) => Reasoning.Planner.PlanFit.searchStep (target) (__r))) ((fun (__r : Reasoning.Planner.PlanFit.Search) => ((__r).ledger).expansions)) (Reasoning.Planner.PlanFit.search_expanded (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Reasoning.Planner.PlanFit.start (target)) rfl)
public theorem PlanFit.iterations_bounded (target : Nat) : ((((Reasoning.Planner.PlanFit.run (target)).1).ledger).iterations <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [Reasoning.Planner.PlanFit.run]
  exact (LexLeanReasoning.iterateUntilCount ((fun (__r : Reasoning.Planner.PlanFit.Search) => Reasoning.Planner.PlanFit.searchStep (target) (__r))) ((fun (__r : Reasoning.Planner.PlanFit.Search) => ((__r).ledger).iterations)) (Reasoning.Planner.PlanFit.search_count (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Reasoning.Planner.PlanFit.start (target)) rfl)
public theorem PlanFit.explained (target : Nat) : (forall (__v : Nat), (forall (__trace : List (Reasoning.Planner.PlanFit.Step)), ((Reasoning.Planner.PlanFit (target) = Except.ok ((__v, __trace))) -> ((Reasoning.Planner.PlanFit.answer (target) (__trace) = Option.some (__v)) /\ Reasoning.Planner.Within (target) (__v))))) :=
by
  intro llV llT llE
  have llSearch := Reasoning.Planner.PlanFit.search_ok target
  dsimp only [Reasoning.Planner.PlanFit] at llE
  generalize llRun : Reasoning.Planner.PlanFit.run target = llR at llE llSearch
  split at llE
  · rename_i llHit llF
    cases llE
    have llOk := And.right llSearch
    rw [llF] at llOk
    exact And.intro (by dsimp only [Reasoning.Planner.PlanFit.answer]; rw [And.left llOk]; exact And.right llOk) (Reasoning.Planner.PlanFit.accept_sound _ _ _ (Reasoning.Planner.PlanFit.follow_invariant target _ _ (And.left llOk)) (And.right llOk))
  · cases llE
public theorem PlanFit.verdict_sound (target : Nat) : (forall (__v : Nat), ((Reasoning.Planner.PlanFit.verdict (target) = Except.ok (__v)) -> Reasoning.Planner.Within (target) (__v))) :=
by
  intro llV llE
  dsimp only [Reasoning.Planner.PlanFit.verdict] at llE
  generalize llHP : Reasoning.Planner.PlanFit target = llR at llE
  cases llR with
  | error _ => cases llE
  | ok llP =>
    cases llE
    exact And.right (Reasoning.Planner.PlanFit.explained _ llP.1 llP.2 llHP)

public inductive PlanFitR.Step where
  | FillA
  | EmptyB
  | Pour (_ : Nat)
public structure PlanFitR.Ledger where
  iterations : Nat
  attempts : Nat
  firings : Nat
  expansions : Nat
  verifications : Nat
  frontier : Nat
@[expose] public def PlanFitR.observe (_target : Nat) : (Prod (Nat) (Nat)) := (0, 0)
@[expose] public def PlanFitR.fire (__s : (Prod (Nat) (Nat))) (__step : Reasoning.Planner.PlanFitR.Step) : Option ((Prod (Nat) (Nat))) := (match __step with | Reasoning.Planner.PlanFitR.Step.FillA => Reasoning.Planner.FillA.apply (__s) | Reasoning.Planner.PlanFitR.Step.EmptyB => Reasoning.Planner.EmptyB.apply (__s) | Reasoning.Planner.PlanFitR.Step.Pour __b => Reasoning.Planner.Pour.apply (__s) (__b))
@[expose] public def PlanFitR.replay (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : Reasoning.Planner.PlanFitR.Step) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match Reasoning.Planner.PlanFitR.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def PlanFitR.follow (target : Nat) (__trace : List (Reasoning.Planner.PlanFitR.Step)) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (LexLeanCollections.listFold ((Reasoning.Planner.PlanFitR.replay)) (Except.ok (Reasoning.Planner.PlanFitR.observe (target))) (__trace) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))))
@[expose] public def PlanFitR.extract (jugs : (Prod (Nat) (Nat))) : Option (Nat) := Option.some ((jugs).1)
@[expose] public def PlanFitR.accept (_target : Nat) (__s : (Prod (Nat) (Nat))) : Option (Nat) := Reasoning.Planner.PlanFitR.extract (__s)
@[expose] public def PlanFitR.answer (target : Nat) (__trace : List (Reasoning.Planner.PlanFitR.Step)) : Option (Nat) := (match Reasoning.Planner.PlanFitR.follow (target) (__trace) with | Except.ok __s => Reasoning.Planner.PlanFitR.accept (target) (__s) | Except.error _ => Option.none)
public structure PlanFitR.Node where
  state : (Prod (Nat) (Nat))
  trace : List (Reasoning.Planner.PlanFitR.Step)
public structure PlanFitR.Search where
  frontier : List (Reasoning.Planner.PlanFitR.Node)
  visited : List ((Prod (Nat) (Nat)))
  found : Option ((Prod (Nat) (Reasoning.Planner.PlanFitR.Node)))
  truncated : Bool
  ledger : Reasoning.Planner.PlanFitR.Ledger
@[expose] public def PlanFitR.FillA.successors (__node : Reasoning.Planner.PlanFitR.Node) : List (Reasoning.Planner.PlanFitR.Node) := (match Reasoning.Planner.PlanFitR.fire ((__node).state) (Reasoning.Planner.PlanFitR.Step.FillA) with | Option.none => ([] : List (Reasoning.Planner.PlanFitR.Node)) | Option.some __t => (({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((Reasoning.Planner.PlanFitR.Step.FillA :: ([] : List (Reasoning.Planner.PlanFitR.Step)))) : List (Reasoning.Planner.PlanFitR.Step)) } : Reasoning.Planner.PlanFitR.Node) :: ([] : List (Reasoning.Planner.PlanFitR.Node))))
@[expose] public def PlanFitR.EmptyB.successors (__node : Reasoning.Planner.PlanFitR.Node) : List (Reasoning.Planner.PlanFitR.Node) := (match Reasoning.Planner.PlanFitR.fire ((__node).state) (Reasoning.Planner.PlanFitR.Step.EmptyB) with | Option.none => ([] : List (Reasoning.Planner.PlanFitR.Node)) | Option.some __t => (({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((Reasoning.Planner.PlanFitR.Step.EmptyB :: ([] : List (Reasoning.Planner.PlanFitR.Step)))) : List (Reasoning.Planner.PlanFitR.Step)) } : Reasoning.Planner.PlanFitR.Node) :: ([] : List (Reasoning.Planner.PlanFitR.Node))))
@[expose] public def PlanFitR.Pour.collect (__node : Reasoning.Planner.PlanFitR.Node) (__acc : List (Reasoning.Planner.PlanFitR.Node)) (__b : Nat) : List (Reasoning.Planner.PlanFitR.Node) := (match Reasoning.Planner.PlanFitR.fire ((__node).state) (Reasoning.Planner.PlanFitR.Step.Pour (__b)) with | Option.none => __acc | Option.some __t => (LexLeanRuntime.append (__acc) ((({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((Reasoning.Planner.PlanFitR.Step.Pour (__b) :: ([] : List (Reasoning.Planner.PlanFitR.Step)))) : List (Reasoning.Planner.PlanFitR.Step)) } : Reasoning.Planner.PlanFitR.Node) :: ([] : List (Reasoning.Planner.PlanFitR.Node)))) : List (Reasoning.Planner.PlanFitR.Node)))
@[expose] public def PlanFitR.Pour.successors (__node : Reasoning.Planner.PlanFitR.Node) : List (Reasoning.Planner.PlanFitR.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Reasoning.Planner.PlanFitR.Node)) (__b : Nat) => Reasoning.Planner.PlanFitR.Pour.collect (__node) (__acc) (__b))) (([] : List (Reasoning.Planner.PlanFitR.Node))) (Reasoning.Planner.Pour.candidates ((__node).state)) : List (Reasoning.Planner.PlanFitR.Node))
@[expose] public def PlanFitR.successors (__node : Reasoning.Planner.PlanFitR.Node) : List (Reasoning.Planner.PlanFitR.Node) := (LexLeanRuntime.append (Reasoning.Planner.PlanFitR.FillA.successors (__node)) ((LexLeanRuntime.append (Reasoning.Planner.PlanFitR.EmptyB.successors (__node)) (Reasoning.Planner.PlanFitR.Pour.successors (__node)) : List (Reasoning.Planner.PlanFitR.Node))) : List (Reasoning.Planner.PlanFitR.Node))
@[expose] public def PlanFitR.fresh (__visited : List ((Prod (Nat) (Nat)))) (__nodes : List (Reasoning.Planner.PlanFitR.Node)) : (Prod (List (Reasoning.Planner.PlanFitR.Node)) (List ((Prod (Nat) (Nat))))) := (LexLeanCollections.listFold ((fun (__acc : (Prod (List (Reasoning.Planner.PlanFitR.Node)) (List ((Prod (Nat) (Nat)))))) (__n : Reasoning.Planner.PlanFitR.Node) => (if (LexLeanCollections.setContains ((__acc).2) ((__n).state) : Bool) then __acc else ((LexLeanRuntime.append ((__acc).1) ((__n :: ([] : List (Reasoning.Planner.PlanFitR.Node)))) : List (Reasoning.Planner.PlanFitR.Node)), (LexLeanCollections.setInsert ((__acc).2) ((__n).state) : List ((Prod (Nat) (Nat)))))))) ((([] : List (Reasoning.Planner.PlanFitR.Node)), __visited)) (__nodes) : (Prod (List (Reasoning.Planner.PlanFitR.Node)) (List ((Prod (Nat) (Nat))))))
@[expose] public def PlanFitR.start (target : Nat) : Reasoning.Planner.PlanFitR.Search := (let __first : List (Reasoning.Planner.PlanFitR.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Reasoning.Planner.PlanFitR.Node)) (__n : Reasoning.Planner.PlanFitR.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) ((target + target))) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (Reasoning.Planner.PlanFitR.Node)))) : List (Reasoning.Planner.PlanFitR.Node)) else __acc))) (([] : List (Reasoning.Planner.PlanFitR.Node))) ((({ state := Reasoning.Planner.PlanFitR.observe (target), trace := ([] : List (Reasoning.Planner.PlanFitR.Step)) } : Reasoning.Planner.PlanFitR.Node) :: ([] : List (Reasoning.Planner.PlanFitR.Node)))) : List (Reasoning.Planner.PlanFitR.Node)); ({ frontier := __first, visited := (LexLeanCollections.setInsert (([] : List ((Prod (Nat) (Nat))))) (Reasoning.Planner.PlanFitR.observe (target)) : List ((Prod (Nat) (Nat)))), found := Option.none, truncated := (Nat.blt ((target + target)) (1)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := (LexLeanRuntime.length (__first) : Nat) } : Reasoning.Planner.PlanFitR.Ledger) } : Reasoning.Planner.PlanFitR.Search))
@[expose] public def PlanFitR.attempts (__s : (Prod (Nat) (Nat))) : Nat := (1 + (1 + (LexLeanRuntime.length (Reasoning.Planner.Pour.candidates (__s)) : Nat)))
@[expose] public def PlanFitR.searchStep (target : Nat) (__r : Reasoning.Planner.PlanFitR.Search) : Option (Reasoning.Planner.PlanFitR.Search) := (match (__r).found with | Option.some _ => Option.none | Option.none => (match (__r).frontier with | List.nil => Option.none | List.cons __node __rest => (match Reasoning.Planner.PlanFitR.accept (target) ((__node).state) with | Option.some __v => Option.some (({ frontier := __rest, visited := (__r).visited, found := Option.some ((__v, __node)), truncated := (__r).truncated, ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := ((__r).ledger).attempts, firings := ((__r).ledger).firings, expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : Reasoning.Planner.PlanFitR.Ledger) } : Reasoning.Planner.PlanFitR.Search)) | Option.none => (let __successors : List (Reasoning.Planner.PlanFitR.Node) := Reasoning.Planner.PlanFitR.successors (__node); (let __fresh : (Prod (List (Reasoning.Planner.PlanFitR.Node)) (List ((Prod (Nat) (Nat))))) := Reasoning.Planner.PlanFitR.fresh ((__r).visited) (__successors); (let __ordered : List (Reasoning.Planner.PlanFitR.Node) := (LexLeanRuntime.append (__rest) ((__fresh).1) : List (Reasoning.Planner.PlanFitR.Node)); (let __next : List (Reasoning.Planner.PlanFitR.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Reasoning.Planner.PlanFitR.Node)) (__n : Reasoning.Planner.PlanFitR.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) ((target + target))) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (Reasoning.Planner.PlanFitR.Node)))) : List (Reasoning.Planner.PlanFitR.Node)) else __acc))) (([] : List (Reasoning.Planner.PlanFitR.Node))) (__ordered) : List (Reasoning.Planner.PlanFitR.Node)); Option.some (({ frontier := __next, visited := (__fresh).2, found := Option.none, truncated := ((__r).truncated || (Nat.blt ((target + target)) ((LexLeanRuntime.length (__ordered) : Nat)))), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + Reasoning.Planner.PlanFitR.attempts ((__node).state)), firings := (((__r).ledger).firings + (LexLeanRuntime.length (__successors) : Nat)), expansions := (((__r).ledger).expansions + 1), verifications := ((__r).ledger).verifications, frontier := (if (Nat.blt (((__r).ledger).frontier) ((LexLeanRuntime.length (__next) : Nat))) then (LexLeanRuntime.length (__next) : Nat) else ((__r).ledger).frontier) } : Reasoning.Planner.PlanFitR.Ledger) } : Reasoning.Planner.PlanFitR.Search)))))))))
@[expose] public def PlanFitR.run (target : Nat) : (Prod (Reasoning.Planner.PlanFitR.Search) (Bool)) := (LexLeanCollections.iterateUntil ((fun (__r : Reasoning.Planner.PlanFitR.Search) => Reasoning.Planner.PlanFitR.searchStep (target) (__r))) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Reasoning.Planner.PlanFitR.start (target)) : (Prod (Reasoning.Planner.PlanFitR.Search) (Bool)))
@[expose] public def PlanFitR.failure (__saturated : Bool) (__truncated : Bool) : (Prod Bool Bool) := (if (__saturated && (!__truncated)) then ((false, true) : Prod Bool Bool) else ((false, false) : Prod Bool Bool))
@[expose] public def PlanFitR (target : Nat) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (Reasoning.Planner.PlanFitR.Step)))) := (let __final : (Prod (Reasoning.Planner.PlanFitR.Search) (Bool)) := Reasoning.Planner.PlanFitR.run (target); (match ((__final).1).found with | Option.some __hit => Except.ok (((__hit).1, ((__hit).2).trace)) | Option.none => Except.error (Reasoning.Planner.PlanFitR.failure ((__final).2) (((__final).1).truncated))))
@[expose] public def PlanFitR.verdict (target : Nat) : Except ((Prod Bool Bool)) (Nat) := (match Reasoning.Planner.PlanFitR (target) with | Except.ok __p => Except.ok ((__p).1) | Except.error __e => Except.error (__e))
public theorem PlanFitR.fire_sound (__s : (Prod (Nat) (Nat))) (__step : Reasoning.Planner.PlanFitR.Step) (__t : (Prod (Nat) (Nat))) : ((Reasoning.Planner.PlanFitR.fire (__s) (__step) = Option.some (__t)) -> Reasoning.Planner.Moves (__s) (__t)) :=
by
  cases __step with
  | FillA =>
    exact Reasoning.Planner.FillA.apply_sound __s __t
  | EmptyB =>
    exact Reasoning.Planner.EmptyB.apply_sound __s __t
  | Pour __b =>
    exact Reasoning.Planner.Pour.apply_sound __s __b __t
public theorem PlanFitR.replay_fire (__s : (Prod (Nat) (Nat))) (__step : Reasoning.Planner.PlanFitR.Step) (__t : (Prod (Nat) (Nat))) : ((Reasoning.Planner.PlanFitR.fire (__s) (__step) = Option.some (__t)) -> (Reasoning.Planner.PlanFitR.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [Reasoning.Planner.PlanFitR.replay]
  rw [llE]
public theorem PlanFitR.replay_sound (target : Nat) (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : Reasoning.Planner.PlanFitR.Step) : ((LexLeanReasoning.Reaches ((Reasoning.Planner.Moves)) (Reasoning.Planner.PlanFitR.observe (target)) (__acc)) -> (LexLeanReasoning.Reaches ((Reasoning.Planner.Moves)) (Reasoning.Planner.PlanFitR.observe (target)) (Reasoning.Planner.PlanFitR.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [Reasoning.Planner.PlanFitR.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (Reasoning.Planner.PlanFitR.fire_sound llS __step _ ‹_›)
public theorem PlanFitR.derivation (target : Nat) (__trace : List (Reasoning.Planner.PlanFitR.Step)) : (LexLeanReasoning.Reaches ((Reasoning.Planner.Moves)) (Reasoning.Planner.PlanFitR.observe (target)) (Reasoning.Planner.PlanFitR.follow (target) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((Reasoning.Planner.PlanFitR.replay)) (fun (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) => (LexLeanReasoning.Reaches ((Reasoning.Planner.Moves)) (Reasoning.Planner.PlanFitR.observe (target)) (__acc))) (Reasoning.Planner.PlanFitR.replay_sound (target)) (__trace) (Except.ok (Reasoning.Planner.PlanFitR.observe (target))) (LexLeanReasoning.reachesStart ((Reasoning.Planner.Moves)) (Reasoning.Planner.PlanFitR.observe (target))))
public theorem PlanFitR.follow_invariant (target : Nat) (__trace : List (Reasoning.Planner.PlanFitR.Step)) (__s : (Prod (Nat) (Nat))) : (Reasoning.Planner.Fits (Reasoning.Planner.PlanFitR.observe (target)) -> ((Reasoning.Planner.PlanFitR.follow (target) (__trace) = Except.ok (__s)) -> Reasoning.Planner.Fits (__s))) :=
  (fun llI llE => (LexLeanReasoning.starPreserves ((Reasoning.Planner.Moves)) ((Reasoning.Planner.Fits)) Reasoning.Planner.Pouring.preserves (Reasoning.Planner.PlanFitR.observe (target)) (__s) (Reasoning.Planner.PlanFitR.derivation (target) (__trace) (__s) llE) llI))
public theorem PlanFitR.follow_relation (target : Nat) (__trace : List (Reasoning.Planner.PlanFitR.Step)) (__s : (Prod (Nat) (Nat))) : ((Reasoning.Planner.PlanFitR.follow (target) (__trace) = Except.ok (__s)) -> Reasoning.Planner.Held (target) (__s)) :=
  (fun llE => (LexLeanReasoning.starPreserves ((Reasoning.Planner.Moves)) (fun (__s : (Prod (Nat) (Nat))) => Reasoning.Planner.Held (target) (__s)) (Reasoning.Planner.held_preserved (target)) (Reasoning.Planner.PlanFitR.observe (target)) (__s) (Reasoning.Planner.PlanFitR.derivation (target) (__trace) (__s) llE) (Reasoning.Planner.held_initial (target))))
public theorem PlanFitR.accept_sound (target : Nat) (__s : (Prod (Nat) (Nat))) (__v : Nat) : (Reasoning.Planner.Held (target) (__s) -> ((Reasoning.Planner.PlanFitR.accept (target) (__s) = Option.some (__v)) -> Reasoning.Planner.Within (target) (__v))) :=
  (fun llJ llE => (Reasoning.Planner.plan_held_correct (target) (__s) (__v) llJ llE))
public theorem PlanFitR.extend (target : Nat) (__node : Reasoning.Planner.PlanFitR.Node) (__step : Reasoning.Planner.PlanFitR.Step) (__t : (Prod (Nat) (Nat))) : ((Reasoning.Planner.PlanFitR.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> ((Reasoning.Planner.PlanFitR.fire ((__node).state) (__step) = Option.some (__t)) -> (Reasoning.Planner.PlanFitR.follow (target) ((LexLeanRuntime.append ((__node).trace) ((__step :: ([] : List (Reasoning.Planner.PlanFitR.Step)))) : List (Reasoning.Planner.PlanFitR.Step))) = Except.ok (__t)))) :=
by
  intro llH llE
  dsimp only [Reasoning.Planner.PlanFitR.follow]
  rw [LexLeanReasoning.foldSnoc]
  dsimp only [Reasoning.Planner.PlanFitR.follow] at llH
  rw [llH]
  exact Reasoning.Planner.PlanFitR.replay_fire _ __step __t llE
public theorem PlanFitR.FillA.successors_ok (target : Nat) (__node : Reasoning.Planner.PlanFitR.Node) : ((Reasoning.Planner.PlanFitR.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Reasoning.Planner.PlanFitR.Node) => (Reasoning.Planner.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Reasoning.Planner.PlanFitR.FillA.successors (__node)))) :=
by
  intro llH
  dsimp only [Reasoning.Planner.PlanFitR.FillA.successors]
  split
  · exact LexLeanReasoning.All.nil
  · exact LexLeanReasoning.allSingle _ _ (Reasoning.Planner.PlanFitR.extend _ __node _ _ llH ‹_›)
public theorem PlanFitR.EmptyB.successors_ok (target : Nat) (__node : Reasoning.Planner.PlanFitR.Node) : ((Reasoning.Planner.PlanFitR.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Reasoning.Planner.PlanFitR.Node) => (Reasoning.Planner.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Reasoning.Planner.PlanFitR.EmptyB.successors (__node)))) :=
by
  intro llH
  dsimp only [Reasoning.Planner.PlanFitR.EmptyB.successors]
  split
  · exact LexLeanReasoning.All.nil
  · exact LexLeanReasoning.allSingle _ _ (Reasoning.Planner.PlanFitR.extend _ __node _ _ llH ‹_›)
public theorem PlanFitR.Pour.successors_ok (target : Nat) (__node : Reasoning.Planner.PlanFitR.Node) : ((Reasoning.Planner.PlanFitR.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Reasoning.Planner.PlanFitR.Node) => (Reasoning.Planner.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Reasoning.Planner.PlanFitR.Pour.successors (__node)))) :=
by
  intro llH
  dsimp only [Reasoning.Planner.PlanFitR.Pour.successors]
  exact LexLeanReasoning.foldInvariant _ (LexLeanReasoning.All (fun (llN : Reasoning.Planner.PlanFitR.Node) => Reasoning.Planner.PlanFitR.follow _ llN.trace = Except.ok llN.state))
    (fun llA llB llP => by
      dsimp only [Reasoning.Planner.PlanFitR.Pour.collect]
      split
      · exact llP
      · exact LexLeanReasoning.allAppend _ _ _ llP (LexLeanReasoning.allSingle _ _ (Reasoning.Planner.PlanFitR.extend _ __node _ _ llH ‹_›)))
    _ _ LexLeanReasoning.All.nil
public theorem PlanFitR.successors_ok (target : Nat) (__node : Reasoning.Planner.PlanFitR.Node) : ((Reasoning.Planner.PlanFitR.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Reasoning.Planner.PlanFitR.Node) => (Reasoning.Planner.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Reasoning.Planner.PlanFitR.successors (__node)))) :=
  (fun llH => (LexLeanReasoning.allAppend _ _ _ (Reasoning.Planner.PlanFitR.FillA.successors_ok (target) (__node) llH) (LexLeanReasoning.allAppend _ _ _ (Reasoning.Planner.PlanFitR.EmptyB.successors_ok (target) (__node) llH) (Reasoning.Planner.PlanFitR.Pour.successors_ok (target) (__node) llH))))
public theorem PlanFitR.fresh_ok (target : Nat) (__visited : List ((Prod (Nat) (Nat)))) (__nodes : List (Reasoning.Planner.PlanFitR.Node)) : ((LexLeanReasoning.All ((fun (__n : Reasoning.Planner.PlanFitR.Node) => (Reasoning.Planner.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (__nodes)) -> (LexLeanReasoning.All ((fun (__n : Reasoning.Planner.PlanFitR.Node) => (Reasoning.Planner.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((Reasoning.Planner.PlanFitR.fresh (__visited) (__nodes)).1))) :=
  (LexLeanReasoning.freshAll ((fun (__n : Reasoning.Planner.PlanFitR.Node) => (__n).state)) (fun (__n : Reasoning.Planner.PlanFitR.Node) => (Reasoning.Planner.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state))) (__visited) (__nodes))
public theorem PlanFitR.search_step (target : Nat) (__r : Reasoning.Planner.PlanFitR.Search) (__q : Reasoning.Planner.PlanFitR.Search) : ((Reasoning.Planner.PlanFitR.searchStep (target) (__r) = Option.some (__q)) -> ((LexLeanReasoning.SearchOk ((fun (__n : Reasoning.Planner.PlanFitR.Node) => (Reasoning.Planner.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Reasoning.Planner.PlanFitR.Node) => Reasoning.Planner.PlanFitR.accept (target) ((__n).state))) ((__r).frontier) ((__r).found)) -> (LexLeanReasoning.SearchOk ((fun (__n : Reasoning.Planner.PlanFitR.Node) => (Reasoning.Planner.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Reasoning.Planner.PlanFitR.Node) => Reasoning.Planner.PlanFitR.accept (target) ((__n).state))) ((__q).frontier) ((__q).found)))) :=
by
  intro llE llH
  dsimp only [Reasoning.Planner.PlanFitR.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · rename_i llNode llRest llFr
      have llAll := And.left llH
      rw [llFr] at llAll
      have llNodeOk := LexLeanReasoning.allHead _ llNode llRest llAll
      have llRestOk := LexLeanReasoning.allTail _ llNode llRest llAll
      split at llE
      · rename_i llV llAcc
        cases llE
        exact And.intro llRestOk (And.intro llNodeOk llAcc)
      · cases llE
        exact And.intro (LexLeanReasoning.capAll _ _ _ (LexLeanReasoning.allAppend _ _ _ llRestOk (Reasoning.Planner.PlanFitR.fresh_ok _ _ _ (Reasoning.Planner.PlanFitR.successors_ok _ llNode llNodeOk)))) True.intro
public theorem PlanFitR.search_peak (target : Nat) (__r : Reasoning.Planner.PlanFitR.Search) (__q : Reasoning.Planner.PlanFitR.Search) : ((Reasoning.Planner.PlanFitR.searchStep (target) (__r) = Option.some (__q)) -> ((((__r).ledger).frontier <= (target + target)) -> (((__q).ledger).frontier <= (target + target)))) :=
by
  intro llE llH
  dsimp only [Reasoning.Planner.PlanFitR.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        exact llH
      · cases llE
        exact LexLeanReasoning.peakBound _ _ _ llH (LexLeanReasoning.capBound _ _)
public theorem PlanFitR.search_count (target : Nat) (__r : Reasoning.Planner.PlanFitR.Search) (__q : Reasoning.Planner.PlanFitR.Search) : ((Reasoning.Planner.PlanFitR.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Planner.PlanFitR.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        rfl
      · cases llE
        rfl
public theorem PlanFitR.search_verify_growth (target : Nat) (__r : Reasoning.Planner.PlanFitR.Search) (__q : Reasoning.Planner.PlanFitR.Search) : ((Reasoning.Planner.PlanFitR.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).verifications <= (((__r).ledger).verifications + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Planner.PlanFitR.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem PlanFitR.search_expanded (target : Nat) (__r : Reasoning.Planner.PlanFitR.Search) (__q : Reasoning.Planner.PlanFitR.Search) : ((Reasoning.Planner.PlanFitR.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).expansions <= (((__r).ledger).expansions + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Planner.PlanFitR.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem PlanFitR.search_ok (target : Nat) : (LexLeanReasoning.SearchOk ((fun (__n : Reasoning.Planner.PlanFitR.Node) => (Reasoning.Planner.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Reasoning.Planner.PlanFitR.Node) => Reasoning.Planner.PlanFitR.accept (target) ((__n).state))) (((Reasoning.Planner.PlanFitR.run (target)).1).frontier) (((Reasoning.Planner.PlanFitR.run (target)).1).found)) :=
by
  dsimp only [Reasoning.Planner.PlanFitR.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((fun (__r : Reasoning.Planner.PlanFitR.Search) => Reasoning.Planner.PlanFitR.searchStep (target) (__r))) (fun (__r : Reasoning.Planner.PlanFitR.Search) => (LexLeanReasoning.SearchOk ((fun (__n : Reasoning.Planner.PlanFitR.Node) => (Reasoning.Planner.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Reasoning.Planner.PlanFitR.Node) => Reasoning.Planner.PlanFitR.accept (target) ((__n).state))) ((__r).frontier) ((__r).found))) (Reasoning.Planner.PlanFitR.search_step (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Reasoning.Planner.PlanFitR.start (target)) (LexLeanReasoning.searchStart ((fun (__n : Reasoning.Planner.PlanFitR.Node) => (Reasoning.Planner.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Reasoning.Planner.PlanFitR.Node) => Reasoning.Planner.PlanFitR.accept (target) ((__n).state))) _ (LexLeanReasoning.capAll ((fun (__n : Reasoning.Planner.PlanFitR.Node) => (Reasoning.Planner.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((target + target)) _ (LexLeanReasoning.allSingle ((fun (__n : Reasoning.Planner.PlanFitR.Node) => (Reasoning.Planner.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (({ state := Reasoning.Planner.PlanFitR.observe (target), trace := ([] : List (Reasoning.Planner.PlanFitR.Step)) } : Reasoning.Planner.PlanFitR.Node)) rfl))))
public theorem PlanFitR.frontier_bounded (target : Nat) : ((((Reasoning.Planner.PlanFitR.run (target)).1).ledger).frontier <= (target + target)) :=
by
  dsimp only [Reasoning.Planner.PlanFitR.run]
  exact (LexLeanReasoning.iterateUntilBound ((fun (__r : Reasoning.Planner.PlanFitR.Search) => Reasoning.Planner.PlanFitR.searchStep (target) (__r))) ((fun (__r : Reasoning.Planner.PlanFitR.Search) => ((__r).ledger).frontier)) ((target + target)) (Reasoning.Planner.PlanFitR.search_peak (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Reasoning.Planner.PlanFitR.start (target)) (LexLeanReasoning.capBound ((target + target)) _))
public theorem PlanFitR.verifications_bounded (target : Nat) : ((((Reasoning.Planner.PlanFitR.run (target)).1).ledger).verifications <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [Reasoning.Planner.PlanFitR.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : Reasoning.Planner.PlanFitR.Search) => Reasoning.Planner.PlanFitR.searchStep (target) (__r))) ((fun (__r : Reasoning.Planner.PlanFitR.Search) => ((__r).ledger).verifications)) (Reasoning.Planner.PlanFitR.search_verify_growth (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Reasoning.Planner.PlanFitR.start (target)) rfl)
public theorem PlanFitR.expansions_bounded (target : Nat) : ((((Reasoning.Planner.PlanFitR.run (target)).1).ledger).expansions <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [Reasoning.Planner.PlanFitR.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : Reasoning.Planner.PlanFitR.Search) => Reasoning.Planner.PlanFitR.searchStep (target) (__r))) ((fun (__r : Reasoning.Planner.PlanFitR.Search) => ((__r).ledger).expansions)) (Reasoning.Planner.PlanFitR.search_expanded (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Reasoning.Planner.PlanFitR.start (target)) rfl)
public theorem PlanFitR.iterations_bounded (target : Nat) : ((((Reasoning.Planner.PlanFitR.run (target)).1).ledger).iterations <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [Reasoning.Planner.PlanFitR.run]
  exact (LexLeanReasoning.iterateUntilCount ((fun (__r : Reasoning.Planner.PlanFitR.Search) => Reasoning.Planner.PlanFitR.searchStep (target) (__r))) ((fun (__r : Reasoning.Planner.PlanFitR.Search) => ((__r).ledger).iterations)) (Reasoning.Planner.PlanFitR.search_count (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Reasoning.Planner.PlanFitR.start (target)) rfl)
public theorem PlanFitR.explained (target : Nat) : (forall (__v : Nat), (forall (__trace : List (Reasoning.Planner.PlanFitR.Step)), ((Reasoning.Planner.PlanFitR (target) = Except.ok ((__v, __trace))) -> ((Reasoning.Planner.PlanFitR.answer (target) (__trace) = Option.some (__v)) /\ Reasoning.Planner.Within (target) (__v))))) :=
by
  intro llV llT llE
  have llSearch := Reasoning.Planner.PlanFitR.search_ok target
  dsimp only [Reasoning.Planner.PlanFitR] at llE
  generalize llRun : Reasoning.Planner.PlanFitR.run target = llR at llE llSearch
  split at llE
  · rename_i llHit llF
    cases llE
    have llOk := And.right llSearch
    rw [llF] at llOk
    exact And.intro (by dsimp only [Reasoning.Planner.PlanFitR.answer]; rw [And.left llOk]; exact And.right llOk) (Reasoning.Planner.PlanFitR.accept_sound _ _ _ (Reasoning.Planner.PlanFitR.follow_relation target _ _ (And.left llOk)) (And.right llOk))
  · cases llE
public theorem PlanFitR.verdict_sound (target : Nat) : (forall (__v : Nat), ((Reasoning.Planner.PlanFitR.verdict (target) = Except.ok (__v)) -> Reasoning.Planner.Within (target) (__v))) :=
by
  intro llV llE
  dsimp only [Reasoning.Planner.PlanFitR.verdict] at llE
  generalize llHP : Reasoning.Planner.PlanFitR target = llR at llE
  cases llR with
  | error _ => cases llE
  | ok llP =>
    cases llE
    exact And.right (Reasoning.Planner.PlanFitR.explained _ llP.1 llP.2 llHP)

public inductive PlanFitU.Step where
  | FillA
  | EmptyB
  | Pour (_ : Nat)
public structure PlanFitU.Ledger where
  iterations : Nat
  attempts : Nat
  firings : Nat
  expansions : Nat
  verifications : Nat
  frontier : Nat
@[expose] public def PlanFitU.observe (_target : Nat) : (Prod (Nat) (Nat)) := (0, 0)
@[expose] public def PlanFitU.fire (__s : (Prod (Nat) (Nat))) (__step : Reasoning.Planner.PlanFitU.Step) : Option ((Prod (Nat) (Nat))) := (match __step with | Reasoning.Planner.PlanFitU.Step.FillA => Reasoning.Planner.FillA.apply (__s) | Reasoning.Planner.PlanFitU.Step.EmptyB => Reasoning.Planner.EmptyB.apply (__s) | Reasoning.Planner.PlanFitU.Step.Pour __b => Reasoning.Planner.Pour.apply (__s) (__b))
@[expose] public def PlanFitU.replay (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : Reasoning.Planner.PlanFitU.Step) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match Reasoning.Planner.PlanFitU.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def PlanFitU.follow (target : Nat) (__trace : List (Reasoning.Planner.PlanFitU.Step)) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (LexLeanCollections.listFold ((Reasoning.Planner.PlanFitU.replay)) (Except.ok (Reasoning.Planner.PlanFitU.observe (target))) (__trace) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))))
@[expose] public def PlanFitU.extract (jugs : (Prod (Nat) (Nat))) : Option (Nat) := Option.some ((jugs).1)
@[expose] public def PlanFitU.accept (_target : Nat) (__s : (Prod (Nat) (Nat))) : Option (Nat) := Reasoning.Planner.PlanFitU.extract (__s)
@[expose] public def PlanFitU.answer (target : Nat) (__trace : List (Reasoning.Planner.PlanFitU.Step)) : Option (Nat) := (match Reasoning.Planner.PlanFitU.follow (target) (__trace) with | Except.ok __s => Reasoning.Planner.PlanFitU.accept (target) (__s) | Except.error _ => Option.none)
public structure PlanFitU.Node where
  state : (Prod (Nat) (Nat))
  trace : List (Reasoning.Planner.PlanFitU.Step)
public structure PlanFitU.Search where
  frontier : List (Reasoning.Planner.PlanFitU.Node)
  visited : List ((Prod (Nat) (Nat)))
  found : Option ((Prod (Nat) (Reasoning.Planner.PlanFitU.Node)))
  truncated : Bool
  ledger : Reasoning.Planner.PlanFitU.Ledger
@[expose] public def PlanFitU.FillA.successors (__node : Reasoning.Planner.PlanFitU.Node) : List (Reasoning.Planner.PlanFitU.Node) := (match Reasoning.Planner.PlanFitU.fire ((__node).state) (Reasoning.Planner.PlanFitU.Step.FillA) with | Option.none => ([] : List (Reasoning.Planner.PlanFitU.Node)) | Option.some __t => (({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((Reasoning.Planner.PlanFitU.Step.FillA :: ([] : List (Reasoning.Planner.PlanFitU.Step)))) : List (Reasoning.Planner.PlanFitU.Step)) } : Reasoning.Planner.PlanFitU.Node) :: ([] : List (Reasoning.Planner.PlanFitU.Node))))
@[expose] public def PlanFitU.EmptyB.successors (__node : Reasoning.Planner.PlanFitU.Node) : List (Reasoning.Planner.PlanFitU.Node) := (match Reasoning.Planner.PlanFitU.fire ((__node).state) (Reasoning.Planner.PlanFitU.Step.EmptyB) with | Option.none => ([] : List (Reasoning.Planner.PlanFitU.Node)) | Option.some __t => (({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((Reasoning.Planner.PlanFitU.Step.EmptyB :: ([] : List (Reasoning.Planner.PlanFitU.Step)))) : List (Reasoning.Planner.PlanFitU.Step)) } : Reasoning.Planner.PlanFitU.Node) :: ([] : List (Reasoning.Planner.PlanFitU.Node))))
@[expose] public def PlanFitU.Pour.collect (__node : Reasoning.Planner.PlanFitU.Node) (__acc : List (Reasoning.Planner.PlanFitU.Node)) (__b : Nat) : List (Reasoning.Planner.PlanFitU.Node) := (match Reasoning.Planner.PlanFitU.fire ((__node).state) (Reasoning.Planner.PlanFitU.Step.Pour (__b)) with | Option.none => __acc | Option.some __t => (LexLeanRuntime.append (__acc) ((({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((Reasoning.Planner.PlanFitU.Step.Pour (__b) :: ([] : List (Reasoning.Planner.PlanFitU.Step)))) : List (Reasoning.Planner.PlanFitU.Step)) } : Reasoning.Planner.PlanFitU.Node) :: ([] : List (Reasoning.Planner.PlanFitU.Node)))) : List (Reasoning.Planner.PlanFitU.Node)))
@[expose] public def PlanFitU.Pour.successors (__node : Reasoning.Planner.PlanFitU.Node) : List (Reasoning.Planner.PlanFitU.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Reasoning.Planner.PlanFitU.Node)) (__b : Nat) => Reasoning.Planner.PlanFitU.Pour.collect (__node) (__acc) (__b))) (([] : List (Reasoning.Planner.PlanFitU.Node))) (Reasoning.Planner.Pour.candidates ((__node).state)) : List (Reasoning.Planner.PlanFitU.Node))
@[expose] public def PlanFitU.successors (__node : Reasoning.Planner.PlanFitU.Node) : List (Reasoning.Planner.PlanFitU.Node) := (LexLeanRuntime.append (Reasoning.Planner.PlanFitU.FillA.successors (__node)) ((LexLeanRuntime.append (Reasoning.Planner.PlanFitU.EmptyB.successors (__node)) (Reasoning.Planner.PlanFitU.Pour.successors (__node)) : List (Reasoning.Planner.PlanFitU.Node))) : List (Reasoning.Planner.PlanFitU.Node))
@[expose] public def PlanFitU.fresh (__visited : List ((Prod (Nat) (Nat)))) (__nodes : List (Reasoning.Planner.PlanFitU.Node)) : (Prod (List (Reasoning.Planner.PlanFitU.Node)) (List ((Prod (Nat) (Nat))))) := (LexLeanCollections.listFold ((fun (__acc : (Prod (List (Reasoning.Planner.PlanFitU.Node)) (List ((Prod (Nat) (Nat)))))) (__n : Reasoning.Planner.PlanFitU.Node) => (if (LexLeanCollections.setContains ((__acc).2) ((__n).state) : Bool) then __acc else ((LexLeanRuntime.append ((__acc).1) ((__n :: ([] : List (Reasoning.Planner.PlanFitU.Node)))) : List (Reasoning.Planner.PlanFitU.Node)), (LexLeanCollections.setInsert ((__acc).2) ((__n).state) : List ((Prod (Nat) (Nat)))))))) ((([] : List (Reasoning.Planner.PlanFitU.Node)), __visited)) (__nodes) : (Prod (List (Reasoning.Planner.PlanFitU.Node)) (List ((Prod (Nat) (Nat))))))
@[expose] public def PlanFitU.start (target : Nat) : Reasoning.Planner.PlanFitU.Search := (let __first : List (Reasoning.Planner.PlanFitU.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Reasoning.Planner.PlanFitU.Node)) (__n : Reasoning.Planner.PlanFitU.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) ((target + target))) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (Reasoning.Planner.PlanFitU.Node)))) : List (Reasoning.Planner.PlanFitU.Node)) else __acc))) (([] : List (Reasoning.Planner.PlanFitU.Node))) ((({ state := Reasoning.Planner.PlanFitU.observe (target), trace := ([] : List (Reasoning.Planner.PlanFitU.Step)) } : Reasoning.Planner.PlanFitU.Node) :: ([] : List (Reasoning.Planner.PlanFitU.Node)))) : List (Reasoning.Planner.PlanFitU.Node)); ({ frontier := __first, visited := (LexLeanCollections.setInsert (([] : List ((Prod (Nat) (Nat))))) (Reasoning.Planner.PlanFitU.observe (target)) : List ((Prod (Nat) (Nat)))), found := Option.none, truncated := (Nat.blt ((target + target)) (1)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := (LexLeanRuntime.length (__first) : Nat) } : Reasoning.Planner.PlanFitU.Ledger) } : Reasoning.Planner.PlanFitU.Search))
@[expose] public def PlanFitU.attempts (__s : (Prod (Nat) (Nat))) : Nat := (1 + (1 + (LexLeanRuntime.length (Reasoning.Planner.Pour.candidates (__s)) : Nat)))
@[expose] public def PlanFitU.searchStep (target : Nat) (__r : Reasoning.Planner.PlanFitU.Search) : Option (Reasoning.Planner.PlanFitU.Search) := (match (__r).found with | Option.some _ => Option.none | Option.none => (match (__r).frontier with | List.nil => Option.none | List.cons __node __rest => (match Reasoning.Planner.PlanFitU.accept (target) ((__node).state) with | Option.some __v => Option.some (({ frontier := __rest, visited := (__r).visited, found := Option.some ((__v, __node)), truncated := (__r).truncated, ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := ((__r).ledger).attempts, firings := ((__r).ledger).firings, expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : Reasoning.Planner.PlanFitU.Ledger) } : Reasoning.Planner.PlanFitU.Search)) | Option.none => (let __successors : List (Reasoning.Planner.PlanFitU.Node) := Reasoning.Planner.PlanFitU.successors (__node); (let __fresh : (Prod (List (Reasoning.Planner.PlanFitU.Node)) (List ((Prod (Nat) (Nat))))) := Reasoning.Planner.PlanFitU.fresh ((__r).visited) (__successors); (let __ordered : List (Reasoning.Planner.PlanFitU.Node) := (LexLeanRuntime.append (__rest) ((__fresh).1) : List (Reasoning.Planner.PlanFitU.Node)); (let __next : List (Reasoning.Planner.PlanFitU.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Reasoning.Planner.PlanFitU.Node)) (__n : Reasoning.Planner.PlanFitU.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) ((target + target))) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (Reasoning.Planner.PlanFitU.Node)))) : List (Reasoning.Planner.PlanFitU.Node)) else __acc))) (([] : List (Reasoning.Planner.PlanFitU.Node))) (__ordered) : List (Reasoning.Planner.PlanFitU.Node)); Option.some (({ frontier := __next, visited := (__fresh).2, found := Option.none, truncated := ((__r).truncated || (Nat.blt ((target + target)) ((LexLeanRuntime.length (__ordered) : Nat)))), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + Reasoning.Planner.PlanFitU.attempts ((__node).state)), firings := (((__r).ledger).firings + (LexLeanRuntime.length (__successors) : Nat)), expansions := (((__r).ledger).expansions + 1), verifications := ((__r).ledger).verifications, frontier := (if (Nat.blt (((__r).ledger).frontier) ((LexLeanRuntime.length (__next) : Nat))) then (LexLeanRuntime.length (__next) : Nat) else ((__r).ledger).frontier) } : Reasoning.Planner.PlanFitU.Ledger) } : Reasoning.Planner.PlanFitU.Search)))))))))
@[expose] public def PlanFitU.run (target : Nat) : (Prod (Reasoning.Planner.PlanFitU.Search) (Bool)) := (LexLeanCollections.iterateUntil ((fun (__r : Reasoning.Planner.PlanFitU.Search) => Reasoning.Planner.PlanFitU.searchStep (target) (__r))) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Reasoning.Planner.PlanFitU.start (target)) : (Prod (Reasoning.Planner.PlanFitU.Search) (Bool)))
@[expose] public def PlanFitU.failure (__saturated : Bool) (__truncated : Bool) : (Prod Bool Bool) := (if (__saturated && (!__truncated)) then ((false, true) : Prod Bool Bool) else ((false, false) : Prod Bool Bool))
@[expose] public def PlanFitU (target : Nat) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (Reasoning.Planner.PlanFitU.Step)))) := (let __final : (Prod (Reasoning.Planner.PlanFitU.Search) (Bool)) := Reasoning.Planner.PlanFitU.run (target); (match ((__final).1).found with | Option.some __hit => Except.ok (((__hit).1, ((__hit).2).trace)) | Option.none => Except.error (Reasoning.Planner.PlanFitU.failure ((__final).2) (((__final).1).truncated))))
@[expose] public def PlanFitU.verdict (target : Nat) : Except ((Prod Bool Bool)) (Nat) := (match Reasoning.Planner.PlanFitU (target) with | Except.ok __p => Except.ok ((__p).1) | Except.error __e => Except.error (__e))
public theorem PlanFitU.fire_sound (__s : (Prod (Nat) (Nat))) (__step : Reasoning.Planner.PlanFitU.Step) (__t : (Prod (Nat) (Nat))) : ((Reasoning.Planner.PlanFitU.fire (__s) (__step) = Option.some (__t)) -> Reasoning.Planner.Moves (__s) (__t)) :=
by
  cases __step with
  | FillA =>
    exact Reasoning.Planner.FillA.apply_sound __s __t
  | EmptyB =>
    exact Reasoning.Planner.EmptyB.apply_sound __s __t
  | Pour __b =>
    exact Reasoning.Planner.Pour.apply_sound __s __b __t
public theorem PlanFitU.replay_fire (__s : (Prod (Nat) (Nat))) (__step : Reasoning.Planner.PlanFitU.Step) (__t : (Prod (Nat) (Nat))) : ((Reasoning.Planner.PlanFitU.fire (__s) (__step) = Option.some (__t)) -> (Reasoning.Planner.PlanFitU.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [Reasoning.Planner.PlanFitU.replay]
  rw [llE]
public theorem PlanFitU.replay_sound (target : Nat) (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : Reasoning.Planner.PlanFitU.Step) : ((LexLeanReasoning.Reaches ((Reasoning.Planner.Moves)) (Reasoning.Planner.PlanFitU.observe (target)) (__acc)) -> (LexLeanReasoning.Reaches ((Reasoning.Planner.Moves)) (Reasoning.Planner.PlanFitU.observe (target)) (Reasoning.Planner.PlanFitU.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [Reasoning.Planner.PlanFitU.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (Reasoning.Planner.PlanFitU.fire_sound llS __step _ ‹_›)
public theorem PlanFitU.derivation (target : Nat) (__trace : List (Reasoning.Planner.PlanFitU.Step)) : (LexLeanReasoning.Reaches ((Reasoning.Planner.Moves)) (Reasoning.Planner.PlanFitU.observe (target)) (Reasoning.Planner.PlanFitU.follow (target) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((Reasoning.Planner.PlanFitU.replay)) (fun (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) => (LexLeanReasoning.Reaches ((Reasoning.Planner.Moves)) (Reasoning.Planner.PlanFitU.observe (target)) (__acc))) (Reasoning.Planner.PlanFitU.replay_sound (target)) (__trace) (Except.ok (Reasoning.Planner.PlanFitU.observe (target))) (LexLeanReasoning.reachesStart ((Reasoning.Planner.Moves)) (Reasoning.Planner.PlanFitU.observe (target))))
public theorem PlanFitU.follow_invariant (target : Nat) (__trace : List (Reasoning.Planner.PlanFitU.Step)) (__s : (Prod (Nat) (Nat))) : (Reasoning.Planner.Fits (Reasoning.Planner.PlanFitU.observe (target)) -> ((Reasoning.Planner.PlanFitU.follow (target) (__trace) = Except.ok (__s)) -> Reasoning.Planner.Fits (__s))) :=
  (fun llI llE => (LexLeanReasoning.starPreserves ((Reasoning.Planner.Moves)) ((Reasoning.Planner.Fits)) Reasoning.Planner.Pouring.preserves (Reasoning.Planner.PlanFitU.observe (target)) (__s) (Reasoning.Planner.PlanFitU.derivation (target) (__trace) (__s) llE) llI))
public theorem PlanFitU.accept_sound (target : Nat) (__s : (Prod (Nat) (Nat))) (__v : Nat) : ((Reasoning.Planner.PlanFitU.accept (target) (__s) = Option.some (__v)) -> Reasoning.Planner.Whole (target) (__v)) :=
  (fun llE => (Reasoning.Planner.plan_whole_correct (target) (__s) (__v) llE))
public theorem PlanFitU.extend (target : Nat) (__node : Reasoning.Planner.PlanFitU.Node) (__step : Reasoning.Planner.PlanFitU.Step) (__t : (Prod (Nat) (Nat))) : ((Reasoning.Planner.PlanFitU.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> ((Reasoning.Planner.PlanFitU.fire ((__node).state) (__step) = Option.some (__t)) -> (Reasoning.Planner.PlanFitU.follow (target) ((LexLeanRuntime.append ((__node).trace) ((__step :: ([] : List (Reasoning.Planner.PlanFitU.Step)))) : List (Reasoning.Planner.PlanFitU.Step))) = Except.ok (__t)))) :=
by
  intro llH llE
  dsimp only [Reasoning.Planner.PlanFitU.follow]
  rw [LexLeanReasoning.foldSnoc]
  dsimp only [Reasoning.Planner.PlanFitU.follow] at llH
  rw [llH]
  exact Reasoning.Planner.PlanFitU.replay_fire _ __step __t llE
public theorem PlanFitU.FillA.successors_ok (target : Nat) (__node : Reasoning.Planner.PlanFitU.Node) : ((Reasoning.Planner.PlanFitU.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Reasoning.Planner.PlanFitU.Node) => (Reasoning.Planner.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Reasoning.Planner.PlanFitU.FillA.successors (__node)))) :=
by
  intro llH
  dsimp only [Reasoning.Planner.PlanFitU.FillA.successors]
  split
  · exact LexLeanReasoning.All.nil
  · exact LexLeanReasoning.allSingle _ _ (Reasoning.Planner.PlanFitU.extend _ __node _ _ llH ‹_›)
public theorem PlanFitU.EmptyB.successors_ok (target : Nat) (__node : Reasoning.Planner.PlanFitU.Node) : ((Reasoning.Planner.PlanFitU.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Reasoning.Planner.PlanFitU.Node) => (Reasoning.Planner.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Reasoning.Planner.PlanFitU.EmptyB.successors (__node)))) :=
by
  intro llH
  dsimp only [Reasoning.Planner.PlanFitU.EmptyB.successors]
  split
  · exact LexLeanReasoning.All.nil
  · exact LexLeanReasoning.allSingle _ _ (Reasoning.Planner.PlanFitU.extend _ __node _ _ llH ‹_›)
public theorem PlanFitU.Pour.successors_ok (target : Nat) (__node : Reasoning.Planner.PlanFitU.Node) : ((Reasoning.Planner.PlanFitU.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Reasoning.Planner.PlanFitU.Node) => (Reasoning.Planner.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Reasoning.Planner.PlanFitU.Pour.successors (__node)))) :=
by
  intro llH
  dsimp only [Reasoning.Planner.PlanFitU.Pour.successors]
  exact LexLeanReasoning.foldInvariant _ (LexLeanReasoning.All (fun (llN : Reasoning.Planner.PlanFitU.Node) => Reasoning.Planner.PlanFitU.follow _ llN.trace = Except.ok llN.state))
    (fun llA llB llP => by
      dsimp only [Reasoning.Planner.PlanFitU.Pour.collect]
      split
      · exact llP
      · exact LexLeanReasoning.allAppend _ _ _ llP (LexLeanReasoning.allSingle _ _ (Reasoning.Planner.PlanFitU.extend _ __node _ _ llH ‹_›)))
    _ _ LexLeanReasoning.All.nil
public theorem PlanFitU.successors_ok (target : Nat) (__node : Reasoning.Planner.PlanFitU.Node) : ((Reasoning.Planner.PlanFitU.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Reasoning.Planner.PlanFitU.Node) => (Reasoning.Planner.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Reasoning.Planner.PlanFitU.successors (__node)))) :=
  (fun llH => (LexLeanReasoning.allAppend _ _ _ (Reasoning.Planner.PlanFitU.FillA.successors_ok (target) (__node) llH) (LexLeanReasoning.allAppend _ _ _ (Reasoning.Planner.PlanFitU.EmptyB.successors_ok (target) (__node) llH) (Reasoning.Planner.PlanFitU.Pour.successors_ok (target) (__node) llH))))
public theorem PlanFitU.fresh_ok (target : Nat) (__visited : List ((Prod (Nat) (Nat)))) (__nodes : List (Reasoning.Planner.PlanFitU.Node)) : ((LexLeanReasoning.All ((fun (__n : Reasoning.Planner.PlanFitU.Node) => (Reasoning.Planner.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (__nodes)) -> (LexLeanReasoning.All ((fun (__n : Reasoning.Planner.PlanFitU.Node) => (Reasoning.Planner.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((Reasoning.Planner.PlanFitU.fresh (__visited) (__nodes)).1))) :=
  (LexLeanReasoning.freshAll ((fun (__n : Reasoning.Planner.PlanFitU.Node) => (__n).state)) (fun (__n : Reasoning.Planner.PlanFitU.Node) => (Reasoning.Planner.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state))) (__visited) (__nodes))
public theorem PlanFitU.search_step (target : Nat) (__r : Reasoning.Planner.PlanFitU.Search) (__q : Reasoning.Planner.PlanFitU.Search) : ((Reasoning.Planner.PlanFitU.searchStep (target) (__r) = Option.some (__q)) -> ((LexLeanReasoning.SearchOk ((fun (__n : Reasoning.Planner.PlanFitU.Node) => (Reasoning.Planner.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Reasoning.Planner.PlanFitU.Node) => Reasoning.Planner.PlanFitU.accept (target) ((__n).state))) ((__r).frontier) ((__r).found)) -> (LexLeanReasoning.SearchOk ((fun (__n : Reasoning.Planner.PlanFitU.Node) => (Reasoning.Planner.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Reasoning.Planner.PlanFitU.Node) => Reasoning.Planner.PlanFitU.accept (target) ((__n).state))) ((__q).frontier) ((__q).found)))) :=
by
  intro llE llH
  dsimp only [Reasoning.Planner.PlanFitU.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · rename_i llNode llRest llFr
      have llAll := And.left llH
      rw [llFr] at llAll
      have llNodeOk := LexLeanReasoning.allHead _ llNode llRest llAll
      have llRestOk := LexLeanReasoning.allTail _ llNode llRest llAll
      split at llE
      · rename_i llV llAcc
        cases llE
        exact And.intro llRestOk (And.intro llNodeOk llAcc)
      · cases llE
        exact And.intro (LexLeanReasoning.capAll _ _ _ (LexLeanReasoning.allAppend _ _ _ llRestOk (Reasoning.Planner.PlanFitU.fresh_ok _ _ _ (Reasoning.Planner.PlanFitU.successors_ok _ llNode llNodeOk)))) True.intro
public theorem PlanFitU.search_peak (target : Nat) (__r : Reasoning.Planner.PlanFitU.Search) (__q : Reasoning.Planner.PlanFitU.Search) : ((Reasoning.Planner.PlanFitU.searchStep (target) (__r) = Option.some (__q)) -> ((((__r).ledger).frontier <= (target + target)) -> (((__q).ledger).frontier <= (target + target)))) :=
by
  intro llE llH
  dsimp only [Reasoning.Planner.PlanFitU.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        exact llH
      · cases llE
        exact LexLeanReasoning.peakBound _ _ _ llH (LexLeanReasoning.capBound _ _)
public theorem PlanFitU.search_count (target : Nat) (__r : Reasoning.Planner.PlanFitU.Search) (__q : Reasoning.Planner.PlanFitU.Search) : ((Reasoning.Planner.PlanFitU.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Planner.PlanFitU.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        rfl
      · cases llE
        rfl
public theorem PlanFitU.search_verify_growth (target : Nat) (__r : Reasoning.Planner.PlanFitU.Search) (__q : Reasoning.Planner.PlanFitU.Search) : ((Reasoning.Planner.PlanFitU.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).verifications <= (((__r).ledger).verifications + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Planner.PlanFitU.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem PlanFitU.search_expanded (target : Nat) (__r : Reasoning.Planner.PlanFitU.Search) (__q : Reasoning.Planner.PlanFitU.Search) : ((Reasoning.Planner.PlanFitU.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).expansions <= (((__r).ledger).expansions + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Planner.PlanFitU.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem PlanFitU.search_ok (target : Nat) : (LexLeanReasoning.SearchOk ((fun (__n : Reasoning.Planner.PlanFitU.Node) => (Reasoning.Planner.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Reasoning.Planner.PlanFitU.Node) => Reasoning.Planner.PlanFitU.accept (target) ((__n).state))) (((Reasoning.Planner.PlanFitU.run (target)).1).frontier) (((Reasoning.Planner.PlanFitU.run (target)).1).found)) :=
by
  dsimp only [Reasoning.Planner.PlanFitU.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((fun (__r : Reasoning.Planner.PlanFitU.Search) => Reasoning.Planner.PlanFitU.searchStep (target) (__r))) (fun (__r : Reasoning.Planner.PlanFitU.Search) => (LexLeanReasoning.SearchOk ((fun (__n : Reasoning.Planner.PlanFitU.Node) => (Reasoning.Planner.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Reasoning.Planner.PlanFitU.Node) => Reasoning.Planner.PlanFitU.accept (target) ((__n).state))) ((__r).frontier) ((__r).found))) (Reasoning.Planner.PlanFitU.search_step (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Reasoning.Planner.PlanFitU.start (target)) (LexLeanReasoning.searchStart ((fun (__n : Reasoning.Planner.PlanFitU.Node) => (Reasoning.Planner.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Reasoning.Planner.PlanFitU.Node) => Reasoning.Planner.PlanFitU.accept (target) ((__n).state))) _ (LexLeanReasoning.capAll ((fun (__n : Reasoning.Planner.PlanFitU.Node) => (Reasoning.Planner.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((target + target)) _ (LexLeanReasoning.allSingle ((fun (__n : Reasoning.Planner.PlanFitU.Node) => (Reasoning.Planner.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (({ state := Reasoning.Planner.PlanFitU.observe (target), trace := ([] : List (Reasoning.Planner.PlanFitU.Step)) } : Reasoning.Planner.PlanFitU.Node)) rfl))))
public theorem PlanFitU.frontier_bounded (target : Nat) : ((((Reasoning.Planner.PlanFitU.run (target)).1).ledger).frontier <= (target + target)) :=
by
  dsimp only [Reasoning.Planner.PlanFitU.run]
  exact (LexLeanReasoning.iterateUntilBound ((fun (__r : Reasoning.Planner.PlanFitU.Search) => Reasoning.Planner.PlanFitU.searchStep (target) (__r))) ((fun (__r : Reasoning.Planner.PlanFitU.Search) => ((__r).ledger).frontier)) ((target + target)) (Reasoning.Planner.PlanFitU.search_peak (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Reasoning.Planner.PlanFitU.start (target)) (LexLeanReasoning.capBound ((target + target)) _))
public theorem PlanFitU.verifications_bounded (target : Nat) : ((((Reasoning.Planner.PlanFitU.run (target)).1).ledger).verifications <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [Reasoning.Planner.PlanFitU.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : Reasoning.Planner.PlanFitU.Search) => Reasoning.Planner.PlanFitU.searchStep (target) (__r))) ((fun (__r : Reasoning.Planner.PlanFitU.Search) => ((__r).ledger).verifications)) (Reasoning.Planner.PlanFitU.search_verify_growth (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Reasoning.Planner.PlanFitU.start (target)) rfl)
public theorem PlanFitU.expansions_bounded (target : Nat) : ((((Reasoning.Planner.PlanFitU.run (target)).1).ledger).expansions <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [Reasoning.Planner.PlanFitU.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : Reasoning.Planner.PlanFitU.Search) => Reasoning.Planner.PlanFitU.searchStep (target) (__r))) ((fun (__r : Reasoning.Planner.PlanFitU.Search) => ((__r).ledger).expansions)) (Reasoning.Planner.PlanFitU.search_expanded (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Reasoning.Planner.PlanFitU.start (target)) rfl)
public theorem PlanFitU.iterations_bounded (target : Nat) : ((((Reasoning.Planner.PlanFitU.run (target)).1).ledger).iterations <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [Reasoning.Planner.PlanFitU.run]
  exact (LexLeanReasoning.iterateUntilCount ((fun (__r : Reasoning.Planner.PlanFitU.Search) => Reasoning.Planner.PlanFitU.searchStep (target) (__r))) ((fun (__r : Reasoning.Planner.PlanFitU.Search) => ((__r).ledger).iterations)) (Reasoning.Planner.PlanFitU.search_count (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Reasoning.Planner.PlanFitU.start (target)) rfl)
public theorem PlanFitU.explained (target : Nat) : (forall (__v : Nat), (forall (__trace : List (Reasoning.Planner.PlanFitU.Step)), ((Reasoning.Planner.PlanFitU (target) = Except.ok ((__v, __trace))) -> ((Reasoning.Planner.PlanFitU.answer (target) (__trace) = Option.some (__v)) /\ Reasoning.Planner.Whole (target) (__v))))) :=
by
  intro llV llT llE
  have llSearch := Reasoning.Planner.PlanFitU.search_ok target
  dsimp only [Reasoning.Planner.PlanFitU] at llE
  generalize llRun : Reasoning.Planner.PlanFitU.run target = llR at llE llSearch
  split at llE
  · rename_i llHit llF
    cases llE
    have llOk := And.right llSearch
    rw [llF] at llOk
    exact And.intro (by dsimp only [Reasoning.Planner.PlanFitU.answer]; rw [And.left llOk]; exact And.right llOk) (Reasoning.Planner.PlanFitU.accept_sound _ _ _ (And.right llOk))
  · cases llE
public theorem PlanFitU.verdict_sound (target : Nat) : (forall (__v : Nat), ((Reasoning.Planner.PlanFitU.verdict (target) = Except.ok (__v)) -> Reasoning.Planner.Whole (target) (__v))) :=
by
  intro llV llE
  dsimp only [Reasoning.Planner.PlanFitU.verdict] at llE
  generalize llHP : Reasoning.Planner.PlanFitU target = llR at llE
  cases llR with
  | error _ => cases llE
  | ok llP =>
    cases llE
    exact And.right (Reasoning.Planner.PlanFitU.explained _ llP.1 llP.2 llHP)

end Reasoning.Planner
