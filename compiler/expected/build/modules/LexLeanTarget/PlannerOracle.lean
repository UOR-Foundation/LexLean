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
namespace LexLeanTarget.PlannerOracle

@[expose, reducible] public def Moves (_s : (Prod (Nat) (Nat))) (t : (Prod (Nat) (Nat))) : Prop := (((t).1 <= 4) /\ ((t).2 <= 3))

@[expose, reducible] public def Fits (s : (Prod (Nat) (Nat))) : Prop := (((s).1 <= 4) /\ ((s).2 <= 3))

public theorem fits_kept (s : (Prod (Nat) (Nat))) (t : (Prod (Nat) (Nat))) : (Fits (s) -> (Moves (s) (t) -> Fits (t))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Fits, Moves, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem Pouring.preserves : (LexLeanReasoning.Preserves ((LexLeanTarget.PlannerOracle.Moves)) ((LexLeanTarget.PlannerOracle.Fits))) :=
  LexLeanTarget.PlannerOracle.fits_kept

public theorem filla_sound (s : (Prod (Nat) (Nat))) : ((((Nat.blt ((s).1) (4)) && (Nat.ble ((s).2) (3))) = true) -> Moves (s) ((4, (s).2))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Moves, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

@[expose] public def FillA.guard (s : (Prod (Nat) (Nat))) : Bool := ((Nat.blt ((s).1) (4)) && (Nat.ble ((s).2) (3)))
@[expose] public def FillA.conclusion (s : (Prod (Nat) (Nat))) : (Prod (Nat) (Nat)) := (4, (s).2)
@[expose] public def FillA.apply (s : (Prod (Nat) (Nat))) : Option ((Prod (Nat) (Nat))) := (if LexLeanTarget.PlannerOracle.FillA.guard (s) then Option.some (LexLeanTarget.PlannerOracle.FillA.conclusion (s)) else Option.none)
public theorem FillA.apply_sound (s : (Prod (Nat) (Nat))) (__t : (Prod (Nat) (Nat))) : ((LexLeanTarget.PlannerOracle.FillA.apply (s) = Option.some (__t)) -> LexLeanTarget.PlannerOracle.Moves (s) (__t)) :=
  (LexLeanReasoning.guarded ((LexLeanTarget.PlannerOracle.Moves)) (s) (LexLeanTarget.PlannerOracle.FillA.guard (s)) (LexLeanTarget.PlannerOracle.FillA.conclusion (s)) (LexLeanTarget.PlannerOracle.filla_sound (s)) (__t))

public theorem emptyb_sound (s : (Prod (Nat) (Nat))) : ((((Nat.blt (0) ((s).2)) && (Nat.ble ((s).1) (4))) = true) -> Moves (s) (((s).1, 0))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Moves, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

@[expose] public def EmptyB.guard (s : (Prod (Nat) (Nat))) : Bool := ((Nat.blt (0) ((s).2)) && (Nat.ble ((s).1) (4)))
@[expose] public def EmptyB.conclusion (s : (Prod (Nat) (Nat))) : (Prod (Nat) (Nat)) := ((s).1, 0)
@[expose] public def EmptyB.apply (s : (Prod (Nat) (Nat))) : Option ((Prod (Nat) (Nat))) := (if LexLeanTarget.PlannerOracle.EmptyB.guard (s) then Option.some (LexLeanTarget.PlannerOracle.EmptyB.conclusion (s)) else Option.none)
public theorem EmptyB.apply_sound (s : (Prod (Nat) (Nat))) (__t : (Prod (Nat) (Nat))) : ((LexLeanTarget.PlannerOracle.EmptyB.apply (s) = Option.some (__t)) -> LexLeanTarget.PlannerOracle.Moves (s) (__t)) :=
  (LexLeanReasoning.guarded ((LexLeanTarget.PlannerOracle.Moves)) (s) (LexLeanTarget.PlannerOracle.EmptyB.guard (s)) (LexLeanTarget.PlannerOracle.EmptyB.conclusion (s)) (LexLeanTarget.PlannerOracle.emptyb_sound (s)) (__t))

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
@[expose] public def Pour.apply (s : (Prod (Nat) (Nat))) (k : Nat) : Option ((Prod (Nat) (Nat))) := (if LexLeanTarget.PlannerOracle.Pour.guard (s) (k) then Option.some (LexLeanTarget.PlannerOracle.Pour.conclusion (s) (k)) else Option.none)
public theorem Pour.apply_sound (s : (Prod (Nat) (Nat))) (k : Nat) (__t : (Prod (Nat) (Nat))) : ((LexLeanTarget.PlannerOracle.Pour.apply (s) (k) = Option.some (__t)) -> LexLeanTarget.PlannerOracle.Moves (s) (__t)) :=
  (LexLeanReasoning.guarded ((LexLeanTarget.PlannerOracle.Moves)) (s) (LexLeanTarget.PlannerOracle.Pour.guard (s) (k)) (LexLeanTarget.PlannerOracle.Pour.conclusion (s) (k)) (LexLeanTarget.PlannerOracle.pour_sound (s) (k)) (__t))

@[expose, reducible] public def Measures (target : Nat) (r : (Prod (Nat) (Nat))) : Prop := (((r).2 = target) /\ ((r).1 <= 4))

@[expose] public def measuresCheck (target : Nat) (r : (Prod (Nat) (Nat))) : Bool := ((Nat.beq ((r).2) (target)) && (Nat.ble ((r).1) (4)))

public theorem measures_sound (target : Nat) (r : (Prod (Nat) (Nat))) : ((measuresCheck (target) (r) = true) -> Measures (target) (r)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Measures, measuresCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem Measured.sound : (LexLeanReasoning.Sound ((LexLeanTarget.PlannerOracle.measuresCheck)) ((LexLeanTarget.PlannerOracle.Measures))) :=
  LexLeanTarget.PlannerOracle.measures_sound

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
@[expose] public def Plan.fire (__s : (Prod (Nat) (Nat))) (__step : LexLeanTarget.PlannerOracle.Plan.Step) : Option ((Prod (Nat) (Nat))) := (match __step with | LexLeanTarget.PlannerOracle.Plan.Step.FillA => LexLeanTarget.PlannerOracle.FillA.apply (__s) | LexLeanTarget.PlannerOracle.Plan.Step.EmptyB => LexLeanTarget.PlannerOracle.EmptyB.apply (__s) | LexLeanTarget.PlannerOracle.Plan.Step.Pour __b => LexLeanTarget.PlannerOracle.Pour.apply (__s) (__b))
@[expose] public def Plan.replay (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : LexLeanTarget.PlannerOracle.Plan.Step) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match LexLeanTarget.PlannerOracle.Plan.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def Plan.follow (target : Nat) (__trace : List (LexLeanTarget.PlannerOracle.Plan.Step)) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (LexLeanCollections.listFold ((LexLeanTarget.PlannerOracle.Plan.replay)) (Except.ok (LexLeanTarget.PlannerOracle.Plan.observe (target))) (__trace) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))))
@[expose] public def Plan.extract (jugs : (Prod (Nat) (Nat))) : Option ((Prod (Nat) (Nat))) := Option.some (jugs)
@[expose] public def Plan.accept (target : Nat) (__s : (Prod (Nat) (Nat))) : Option ((Prod (Nat) (Nat))) := (match LexLeanTarget.PlannerOracle.Plan.extract (__s) with | Option.none => Option.none | Option.some __v => (if LexLeanTarget.PlannerOracle.measuresCheck (target) (__v) then Option.some (__v) else Option.none))
@[expose] public def Plan.answer (target : Nat) (__trace : List (LexLeanTarget.PlannerOracle.Plan.Step)) : Option ((Prod (Nat) (Nat))) := (match LexLeanTarget.PlannerOracle.Plan.follow (target) (__trace) with | Except.ok __s => LexLeanTarget.PlannerOracle.Plan.accept (target) (__s) | Except.error _ => Option.none)
public structure Plan.Node where
  state : (Prod (Nat) (Nat))
  trace : List (LexLeanTarget.PlannerOracle.Plan.Step)
public structure Plan.Search where
  frontier : List (LexLeanTarget.PlannerOracle.Plan.Node)
  visited : List ((Prod (Nat) (Nat)))
  found : Option ((Prod ((Prod (Nat) (Nat))) (LexLeanTarget.PlannerOracle.Plan.Node)))
  truncated : Bool
  ledger : LexLeanTarget.PlannerOracle.Plan.Ledger
@[expose] public def Plan.FillA.successors (__node : LexLeanTarget.PlannerOracle.Plan.Node) : List (LexLeanTarget.PlannerOracle.Plan.Node) := (match LexLeanTarget.PlannerOracle.Plan.fire ((__node).state) (LexLeanTarget.PlannerOracle.Plan.Step.FillA) with | Option.none => ([] : List (LexLeanTarget.PlannerOracle.Plan.Node)) | Option.some __t => (({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((LexLeanTarget.PlannerOracle.Plan.Step.FillA :: ([] : List (LexLeanTarget.PlannerOracle.Plan.Step)))) : List (LexLeanTarget.PlannerOracle.Plan.Step)) } : LexLeanTarget.PlannerOracle.Plan.Node) :: ([] : List (LexLeanTarget.PlannerOracle.Plan.Node))))
@[expose] public def Plan.EmptyB.successors (__node : LexLeanTarget.PlannerOracle.Plan.Node) : List (LexLeanTarget.PlannerOracle.Plan.Node) := (match LexLeanTarget.PlannerOracle.Plan.fire ((__node).state) (LexLeanTarget.PlannerOracle.Plan.Step.EmptyB) with | Option.none => ([] : List (LexLeanTarget.PlannerOracle.Plan.Node)) | Option.some __t => (({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((LexLeanTarget.PlannerOracle.Plan.Step.EmptyB :: ([] : List (LexLeanTarget.PlannerOracle.Plan.Step)))) : List (LexLeanTarget.PlannerOracle.Plan.Step)) } : LexLeanTarget.PlannerOracle.Plan.Node) :: ([] : List (LexLeanTarget.PlannerOracle.Plan.Node))))
@[expose] public def Plan.Pour.collect (__node : LexLeanTarget.PlannerOracle.Plan.Node) (__acc : List (LexLeanTarget.PlannerOracle.Plan.Node)) (__b : Nat) : List (LexLeanTarget.PlannerOracle.Plan.Node) := (match LexLeanTarget.PlannerOracle.Plan.fire ((__node).state) (LexLeanTarget.PlannerOracle.Plan.Step.Pour (__b)) with | Option.none => __acc | Option.some __t => (LexLeanRuntime.append (__acc) ((({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((LexLeanTarget.PlannerOracle.Plan.Step.Pour (__b) :: ([] : List (LexLeanTarget.PlannerOracle.Plan.Step)))) : List (LexLeanTarget.PlannerOracle.Plan.Step)) } : LexLeanTarget.PlannerOracle.Plan.Node) :: ([] : List (LexLeanTarget.PlannerOracle.Plan.Node)))) : List (LexLeanTarget.PlannerOracle.Plan.Node)))
@[expose] public def Plan.Pour.successors (__node : LexLeanTarget.PlannerOracle.Plan.Node) : List (LexLeanTarget.PlannerOracle.Plan.Node) := (LexLeanCollections.listFold ((fun (__acc : List (LexLeanTarget.PlannerOracle.Plan.Node)) (__b : Nat) => LexLeanTarget.PlannerOracle.Plan.Pour.collect (__node) (__acc) (__b))) (([] : List (LexLeanTarget.PlannerOracle.Plan.Node))) (LexLeanTarget.PlannerOracle.Pour.candidates ((__node).state)) : List (LexLeanTarget.PlannerOracle.Plan.Node))
@[expose] public def Plan.successors (__node : LexLeanTarget.PlannerOracle.Plan.Node) : List (LexLeanTarget.PlannerOracle.Plan.Node) := (LexLeanRuntime.append (LexLeanTarget.PlannerOracle.Plan.FillA.successors (__node)) ((LexLeanRuntime.append (LexLeanTarget.PlannerOracle.Plan.EmptyB.successors (__node)) (LexLeanTarget.PlannerOracle.Plan.Pour.successors (__node)) : List (LexLeanTarget.PlannerOracle.Plan.Node))) : List (LexLeanTarget.PlannerOracle.Plan.Node))
@[expose] public def Plan.fresh (__visited : List ((Prod (Nat) (Nat)))) (__nodes : List (LexLeanTarget.PlannerOracle.Plan.Node)) : (Prod (List (LexLeanTarget.PlannerOracle.Plan.Node)) (List ((Prod (Nat) (Nat))))) := (LexLeanCollections.listFold ((fun (__acc : (Prod (List (LexLeanTarget.PlannerOracle.Plan.Node)) (List ((Prod (Nat) (Nat)))))) (__n : LexLeanTarget.PlannerOracle.Plan.Node) => (if (LexLeanCollections.setContains ((__acc).2) ((__n).state) : Bool) then __acc else ((LexLeanRuntime.append ((__acc).1) ((__n :: ([] : List (LexLeanTarget.PlannerOracle.Plan.Node)))) : List (LexLeanTarget.PlannerOracle.Plan.Node)), (LexLeanCollections.setInsert ((__acc).2) ((__n).state) : List ((Prod (Nat) (Nat)))))))) ((([] : List (LexLeanTarget.PlannerOracle.Plan.Node)), __visited)) (__nodes) : (Prod (List (LexLeanTarget.PlannerOracle.Plan.Node)) (List ((Prod (Nat) (Nat))))))
@[expose] public def Plan.start (target : Nat) : LexLeanTarget.PlannerOracle.Plan.Search := (let __first : List (LexLeanTarget.PlannerOracle.Plan.Node) := (LexLeanCollections.listFold ((fun (__acc : List (LexLeanTarget.PlannerOracle.Plan.Node)) (__n : LexLeanTarget.PlannerOracle.Plan.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) ((target + target))) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (LexLeanTarget.PlannerOracle.Plan.Node)))) : List (LexLeanTarget.PlannerOracle.Plan.Node)) else __acc))) (([] : List (LexLeanTarget.PlannerOracle.Plan.Node))) ((({ state := LexLeanTarget.PlannerOracle.Plan.observe (target), trace := ([] : List (LexLeanTarget.PlannerOracle.Plan.Step)) } : LexLeanTarget.PlannerOracle.Plan.Node) :: ([] : List (LexLeanTarget.PlannerOracle.Plan.Node)))) : List (LexLeanTarget.PlannerOracle.Plan.Node)); ({ frontier := __first, visited := (LexLeanCollections.setInsert (([] : List ((Prod (Nat) (Nat))))) (LexLeanTarget.PlannerOracle.Plan.observe (target)) : List ((Prod (Nat) (Nat)))), found := Option.none, truncated := (Nat.blt ((target + target)) (1)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := (LexLeanRuntime.length (__first) : Nat) } : LexLeanTarget.PlannerOracle.Plan.Ledger) } : LexLeanTarget.PlannerOracle.Plan.Search))
@[expose] public def Plan.attempts (__s : (Prod (Nat) (Nat))) : Nat := (1 + (1 + (LexLeanRuntime.length (LexLeanTarget.PlannerOracle.Pour.candidates (__s)) : Nat)))
@[expose] public def Plan.searchStep (target : Nat) (__r : LexLeanTarget.PlannerOracle.Plan.Search) : Option (LexLeanTarget.PlannerOracle.Plan.Search) := (match (__r).found with | Option.some _ => Option.none | Option.none => (match (__r).frontier with | List.nil => Option.none | List.cons __node __rest => (match LexLeanTarget.PlannerOracle.Plan.accept (target) ((__node).state) with | Option.some __v => Option.some (({ frontier := __rest, visited := (__r).visited, found := Option.some ((__v, __node)), truncated := (__r).truncated, ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := ((__r).ledger).attempts, firings := ((__r).ledger).firings, expansions := ((__r).ledger).expansions, verifications := (((__r).ledger).verifications + 1), frontier := ((__r).ledger).frontier } : LexLeanTarget.PlannerOracle.Plan.Ledger) } : LexLeanTarget.PlannerOracle.Plan.Search)) | Option.none => (let __successors : List (LexLeanTarget.PlannerOracle.Plan.Node) := LexLeanTarget.PlannerOracle.Plan.successors (__node); (let __fresh : (Prod (List (LexLeanTarget.PlannerOracle.Plan.Node)) (List ((Prod (Nat) (Nat))))) := LexLeanTarget.PlannerOracle.Plan.fresh ((__r).visited) (__successors); (let __ordered : List (LexLeanTarget.PlannerOracle.Plan.Node) := (LexLeanRuntime.append (__rest) ((__fresh).1) : List (LexLeanTarget.PlannerOracle.Plan.Node)); (let __next : List (LexLeanTarget.PlannerOracle.Plan.Node) := (LexLeanCollections.listFold ((fun (__acc : List (LexLeanTarget.PlannerOracle.Plan.Node)) (__n : LexLeanTarget.PlannerOracle.Plan.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) ((target + target))) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (LexLeanTarget.PlannerOracle.Plan.Node)))) : List (LexLeanTarget.PlannerOracle.Plan.Node)) else __acc))) (([] : List (LexLeanTarget.PlannerOracle.Plan.Node))) (__ordered) : List (LexLeanTarget.PlannerOracle.Plan.Node)); Option.some (({ frontier := __next, visited := (__fresh).2, found := Option.none, truncated := ((__r).truncated || (Nat.blt ((target + target)) ((LexLeanRuntime.length (__ordered) : Nat)))), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + LexLeanTarget.PlannerOracle.Plan.attempts ((__node).state)), firings := (((__r).ledger).firings + (LexLeanRuntime.length (__successors) : Nat)), expansions := (((__r).ledger).expansions + 1), verifications := (((__r).ledger).verifications + 1), frontier := (if (Nat.blt (((__r).ledger).frontier) ((LexLeanRuntime.length (__next) : Nat))) then (LexLeanRuntime.length (__next) : Nat) else ((__r).ledger).frontier) } : LexLeanTarget.PlannerOracle.Plan.Ledger) } : LexLeanTarget.PlannerOracle.Plan.Search)))))))))
@[expose] public def Plan.run (target : Nat) : (Prod (LexLeanTarget.PlannerOracle.Plan.Search) (Bool)) := (LexLeanCollections.iterateUntil ((fun (__r : LexLeanTarget.PlannerOracle.Plan.Search) => LexLeanTarget.PlannerOracle.Plan.searchStep (target) (__r))) ((LexLeanRuntime.multiply (target) (8) : Nat)) (LexLeanTarget.PlannerOracle.Plan.start (target)) : (Prod (LexLeanTarget.PlannerOracle.Plan.Search) (Bool)))
@[expose] public def Plan.failure (__saturated : Bool) (__truncated : Bool) : (Prod Bool Bool) := (if (__saturated && (!__truncated)) then ((false, true) : Prod Bool Bool) else ((false, false) : Prod Bool Bool))
@[expose] public def Plan (target : Nat) : Except ((Prod Bool Bool)) ((Prod ((Prod (Nat) (Nat))) (List (LexLeanTarget.PlannerOracle.Plan.Step)))) := (let __final : (Prod (LexLeanTarget.PlannerOracle.Plan.Search) (Bool)) := LexLeanTarget.PlannerOracle.Plan.run (target); (match ((__final).1).found with | Option.some __hit => Except.ok (((__hit).1, ((__hit).2).trace)) | Option.none => Except.error (LexLeanTarget.PlannerOracle.Plan.failure ((__final).2) (((__final).1).truncated))))
@[expose] public def Plan.verdict (target : Nat) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (match LexLeanTarget.PlannerOracle.Plan (target) with | Except.ok __p => Except.ok ((__p).1) | Except.error __e => Except.error (__e))
public theorem Plan.fire_sound (__s : (Prod (Nat) (Nat))) (__step : LexLeanTarget.PlannerOracle.Plan.Step) (__t : (Prod (Nat) (Nat))) : ((LexLeanTarget.PlannerOracle.Plan.fire (__s) (__step) = Option.some (__t)) -> LexLeanTarget.PlannerOracle.Moves (__s) (__t)) :=
by
  cases __step with
  | FillA =>
    exact LexLeanTarget.PlannerOracle.FillA.apply_sound __s __t
  | EmptyB =>
    exact LexLeanTarget.PlannerOracle.EmptyB.apply_sound __s __t
  | Pour __b =>
    exact LexLeanTarget.PlannerOracle.Pour.apply_sound __s __b __t
public theorem Plan.replay_fire (__s : (Prod (Nat) (Nat))) (__step : LexLeanTarget.PlannerOracle.Plan.Step) (__t : (Prod (Nat) (Nat))) : ((LexLeanTarget.PlannerOracle.Plan.fire (__s) (__step) = Option.some (__t)) -> (LexLeanTarget.PlannerOracle.Plan.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [LexLeanTarget.PlannerOracle.Plan.replay]
  rw [llE]
public theorem Plan.replay_sound (target : Nat) (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : LexLeanTarget.PlannerOracle.Plan.Step) : ((LexLeanReasoning.Reaches ((LexLeanTarget.PlannerOracle.Moves)) (LexLeanTarget.PlannerOracle.Plan.observe (target)) (__acc)) -> (LexLeanReasoning.Reaches ((LexLeanTarget.PlannerOracle.Moves)) (LexLeanTarget.PlannerOracle.Plan.observe (target)) (LexLeanTarget.PlannerOracle.Plan.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [LexLeanTarget.PlannerOracle.Plan.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (LexLeanTarget.PlannerOracle.Plan.fire_sound llS __step _ ‹_›)
public theorem Plan.derivation (target : Nat) (__trace : List (LexLeanTarget.PlannerOracle.Plan.Step)) : (LexLeanReasoning.Reaches ((LexLeanTarget.PlannerOracle.Moves)) (LexLeanTarget.PlannerOracle.Plan.observe (target)) (LexLeanTarget.PlannerOracle.Plan.follow (target) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((LexLeanTarget.PlannerOracle.Plan.replay)) (fun (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) => (LexLeanReasoning.Reaches ((LexLeanTarget.PlannerOracle.Moves)) (LexLeanTarget.PlannerOracle.Plan.observe (target)) (__acc))) (LexLeanTarget.PlannerOracle.Plan.replay_sound (target)) (__trace) (Except.ok (LexLeanTarget.PlannerOracle.Plan.observe (target))) (LexLeanReasoning.reachesStart ((LexLeanTarget.PlannerOracle.Moves)) (LexLeanTarget.PlannerOracle.Plan.observe (target))))
public theorem Plan.follow_invariant (target : Nat) (__trace : List (LexLeanTarget.PlannerOracle.Plan.Step)) (__s : (Prod (Nat) (Nat))) : ((LexLeanTarget.PlannerOracle.Plan.follow (target) (__trace) = Except.ok (__s)) -> LexLeanTarget.PlannerOracle.Fits (__s)) :=
  (fun llE => (LexLeanReasoning.starPreserves ((LexLeanTarget.PlannerOracle.Moves)) ((LexLeanTarget.PlannerOracle.Fits)) LexLeanTarget.PlannerOracle.Pouring.preserves (LexLeanTarget.PlannerOracle.Plan.observe (target)) (__s) (LexLeanTarget.PlannerOracle.Plan.derivation (target) (__trace) (__s) llE) (LexLeanTarget.PlannerOracle.empty_jugs_fit (target))))
public theorem Plan.accept_sound (target : Nat) (__s : (Prod (Nat) (Nat))) (__v : (Prod (Nat) (Nat))) : ((LexLeanTarget.PlannerOracle.Plan.accept (target) (__s) = Option.some (__v)) -> LexLeanTarget.PlannerOracle.Measures (target) (__v)) :=
by
  intro llE
  dsimp only [LexLeanTarget.PlannerOracle.Plan.accept] at llE
  split at llE
  · cases llE
  · have llC := LexLeanReasoning.checked _ _ _ llE
    exact llC.right ▸ LexLeanTarget.PlannerOracle.measures_sound _ _ llC.left
public theorem Plan.extend (target : Nat) (__node : LexLeanTarget.PlannerOracle.Plan.Node) (__step : LexLeanTarget.PlannerOracle.Plan.Step) (__t : (Prod (Nat) (Nat))) : ((LexLeanTarget.PlannerOracle.Plan.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> ((LexLeanTarget.PlannerOracle.Plan.fire ((__node).state) (__step) = Option.some (__t)) -> (LexLeanTarget.PlannerOracle.Plan.follow (target) ((LexLeanRuntime.append ((__node).trace) ((__step :: ([] : List (LexLeanTarget.PlannerOracle.Plan.Step)))) : List (LexLeanTarget.PlannerOracle.Plan.Step))) = Except.ok (__t)))) :=
by
  intro llH llE
  dsimp only [LexLeanTarget.PlannerOracle.Plan.follow]
  rw [LexLeanReasoning.foldSnoc]
  dsimp only [LexLeanTarget.PlannerOracle.Plan.follow] at llH
  rw [llH]
  exact LexLeanTarget.PlannerOracle.Plan.replay_fire _ __step __t llE
public theorem Plan.FillA.successors_ok (target : Nat) (__node : LexLeanTarget.PlannerOracle.Plan.Node) : ((LexLeanTarget.PlannerOracle.Plan.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : LexLeanTarget.PlannerOracle.Plan.Node) => (LexLeanTarget.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (LexLeanTarget.PlannerOracle.Plan.FillA.successors (__node)))) :=
by
  intro llH
  dsimp only [LexLeanTarget.PlannerOracle.Plan.FillA.successors]
  split
  · exact LexLeanReasoning.All.nil
  · exact LexLeanReasoning.allSingle _ _ (LexLeanTarget.PlannerOracle.Plan.extend _ __node _ _ llH ‹_›)
public theorem Plan.EmptyB.successors_ok (target : Nat) (__node : LexLeanTarget.PlannerOracle.Plan.Node) : ((LexLeanTarget.PlannerOracle.Plan.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : LexLeanTarget.PlannerOracle.Plan.Node) => (LexLeanTarget.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (LexLeanTarget.PlannerOracle.Plan.EmptyB.successors (__node)))) :=
by
  intro llH
  dsimp only [LexLeanTarget.PlannerOracle.Plan.EmptyB.successors]
  split
  · exact LexLeanReasoning.All.nil
  · exact LexLeanReasoning.allSingle _ _ (LexLeanTarget.PlannerOracle.Plan.extend _ __node _ _ llH ‹_›)
public theorem Plan.Pour.successors_ok (target : Nat) (__node : LexLeanTarget.PlannerOracle.Plan.Node) : ((LexLeanTarget.PlannerOracle.Plan.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : LexLeanTarget.PlannerOracle.Plan.Node) => (LexLeanTarget.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (LexLeanTarget.PlannerOracle.Plan.Pour.successors (__node)))) :=
by
  intro llH
  dsimp only [LexLeanTarget.PlannerOracle.Plan.Pour.successors]
  exact LexLeanReasoning.foldInvariant _ (LexLeanReasoning.All (fun (llN : LexLeanTarget.PlannerOracle.Plan.Node) => LexLeanTarget.PlannerOracle.Plan.follow _ llN.trace = Except.ok llN.state))
    (fun llA llB llP => by
      dsimp only [LexLeanTarget.PlannerOracle.Plan.Pour.collect]
      split
      · exact llP
      · exact LexLeanReasoning.allAppend _ _ _ llP (LexLeanReasoning.allSingle _ _ (LexLeanTarget.PlannerOracle.Plan.extend _ __node _ _ llH ‹_›)))
    _ _ LexLeanReasoning.All.nil
public theorem Plan.successors_ok (target : Nat) (__node : LexLeanTarget.PlannerOracle.Plan.Node) : ((LexLeanTarget.PlannerOracle.Plan.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : LexLeanTarget.PlannerOracle.Plan.Node) => (LexLeanTarget.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (LexLeanTarget.PlannerOracle.Plan.successors (__node)))) :=
  (fun llH => (LexLeanReasoning.allAppend _ _ _ (LexLeanTarget.PlannerOracle.Plan.FillA.successors_ok (target) (__node) llH) (LexLeanReasoning.allAppend _ _ _ (LexLeanTarget.PlannerOracle.Plan.EmptyB.successors_ok (target) (__node) llH) (LexLeanTarget.PlannerOracle.Plan.Pour.successors_ok (target) (__node) llH))))
public theorem Plan.fresh_ok (target : Nat) (__visited : List ((Prod (Nat) (Nat)))) (__nodes : List (LexLeanTarget.PlannerOracle.Plan.Node)) : ((LexLeanReasoning.All ((fun (__n : LexLeanTarget.PlannerOracle.Plan.Node) => (LexLeanTarget.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (__nodes)) -> (LexLeanReasoning.All ((fun (__n : LexLeanTarget.PlannerOracle.Plan.Node) => (LexLeanTarget.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((LexLeanTarget.PlannerOracle.Plan.fresh (__visited) (__nodes)).1))) :=
  (LexLeanReasoning.freshAll ((fun (__n : LexLeanTarget.PlannerOracle.Plan.Node) => (__n).state)) (fun (__n : LexLeanTarget.PlannerOracle.Plan.Node) => (LexLeanTarget.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state))) (__visited) (__nodes))
public theorem Plan.search_step (target : Nat) (__r : LexLeanTarget.PlannerOracle.Plan.Search) (__q : LexLeanTarget.PlannerOracle.Plan.Search) : ((LexLeanTarget.PlannerOracle.Plan.searchStep (target) (__r) = Option.some (__q)) -> ((LexLeanReasoning.SearchOk ((fun (__n : LexLeanTarget.PlannerOracle.Plan.Node) => (LexLeanTarget.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : LexLeanTarget.PlannerOracle.Plan.Node) => LexLeanTarget.PlannerOracle.Plan.accept (target) ((__n).state))) ((__r).frontier) ((__r).found)) -> (LexLeanReasoning.SearchOk ((fun (__n : LexLeanTarget.PlannerOracle.Plan.Node) => (LexLeanTarget.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : LexLeanTarget.PlannerOracle.Plan.Node) => LexLeanTarget.PlannerOracle.Plan.accept (target) ((__n).state))) ((__q).frontier) ((__q).found)))) :=
by
  intro llE llH
  dsimp only [LexLeanTarget.PlannerOracle.Plan.searchStep] at llE
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
        exact And.intro (LexLeanReasoning.capAll _ _ _ (LexLeanReasoning.allAppend _ _ _ llRestOk (LexLeanTarget.PlannerOracle.Plan.fresh_ok _ _ _ (LexLeanTarget.PlannerOracle.Plan.successors_ok _ llNode llNodeOk)))) True.intro
public theorem Plan.search_peak (target : Nat) (__r : LexLeanTarget.PlannerOracle.Plan.Search) (__q : LexLeanTarget.PlannerOracle.Plan.Search) : ((LexLeanTarget.PlannerOracle.Plan.searchStep (target) (__r) = Option.some (__q)) -> ((((__r).ledger).frontier <= (target + target)) -> (((__q).ledger).frontier <= (target + target)))) :=
by
  intro llE llH
  dsimp only [LexLeanTarget.PlannerOracle.Plan.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        exact llH
      · cases llE
        exact LexLeanReasoning.peakBound _ _ _ llH (LexLeanReasoning.capBound _ _)
public theorem Plan.search_count (target : Nat) (__r : LexLeanTarget.PlannerOracle.Plan.Search) (__q : LexLeanTarget.PlannerOracle.Plan.Search) : ((LexLeanTarget.PlannerOracle.Plan.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [LexLeanTarget.PlannerOracle.Plan.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        rfl
      · cases llE
        rfl
public theorem Plan.search_verify_growth (target : Nat) (__r : LexLeanTarget.PlannerOracle.Plan.Search) (__q : LexLeanTarget.PlannerOracle.Plan.Search) : ((LexLeanTarget.PlannerOracle.Plan.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).verifications <= (((__r).ledger).verifications + 1))) :=
by
  intro llE
  dsimp only [LexLeanTarget.PlannerOracle.Plan.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem Plan.search_expanded (target : Nat) (__r : LexLeanTarget.PlannerOracle.Plan.Search) (__q : LexLeanTarget.PlannerOracle.Plan.Search) : ((LexLeanTarget.PlannerOracle.Plan.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).expansions <= (((__r).ledger).expansions + 1))) :=
by
  intro llE
  dsimp only [LexLeanTarget.PlannerOracle.Plan.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem Plan.search_ok (target : Nat) : (LexLeanReasoning.SearchOk ((fun (__n : LexLeanTarget.PlannerOracle.Plan.Node) => (LexLeanTarget.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : LexLeanTarget.PlannerOracle.Plan.Node) => LexLeanTarget.PlannerOracle.Plan.accept (target) ((__n).state))) (((LexLeanTarget.PlannerOracle.Plan.run (target)).1).frontier) (((LexLeanTarget.PlannerOracle.Plan.run (target)).1).found)) :=
by
  dsimp only [LexLeanTarget.PlannerOracle.Plan.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((fun (__r : LexLeanTarget.PlannerOracle.Plan.Search) => LexLeanTarget.PlannerOracle.Plan.searchStep (target) (__r))) (fun (__r : LexLeanTarget.PlannerOracle.Plan.Search) => (LexLeanReasoning.SearchOk ((fun (__n : LexLeanTarget.PlannerOracle.Plan.Node) => (LexLeanTarget.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : LexLeanTarget.PlannerOracle.Plan.Node) => LexLeanTarget.PlannerOracle.Plan.accept (target) ((__n).state))) ((__r).frontier) ((__r).found))) (LexLeanTarget.PlannerOracle.Plan.search_step (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (LexLeanTarget.PlannerOracle.Plan.start (target)) (LexLeanReasoning.searchStart ((fun (__n : LexLeanTarget.PlannerOracle.Plan.Node) => (LexLeanTarget.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : LexLeanTarget.PlannerOracle.Plan.Node) => LexLeanTarget.PlannerOracle.Plan.accept (target) ((__n).state))) _ (LexLeanReasoning.capAll ((fun (__n : LexLeanTarget.PlannerOracle.Plan.Node) => (LexLeanTarget.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((target + target)) _ (LexLeanReasoning.allSingle ((fun (__n : LexLeanTarget.PlannerOracle.Plan.Node) => (LexLeanTarget.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (({ state := LexLeanTarget.PlannerOracle.Plan.observe (target), trace := ([] : List (LexLeanTarget.PlannerOracle.Plan.Step)) } : LexLeanTarget.PlannerOracle.Plan.Node)) rfl))))
public theorem Plan.frontier_bounded (target : Nat) : ((((LexLeanTarget.PlannerOracle.Plan.run (target)).1).ledger).frontier <= (target + target)) :=
by
  dsimp only [LexLeanTarget.PlannerOracle.Plan.run]
  exact (LexLeanReasoning.iterateUntilBound ((fun (__r : LexLeanTarget.PlannerOracle.Plan.Search) => LexLeanTarget.PlannerOracle.Plan.searchStep (target) (__r))) ((fun (__r : LexLeanTarget.PlannerOracle.Plan.Search) => ((__r).ledger).frontier)) ((target + target)) (LexLeanTarget.PlannerOracle.Plan.search_peak (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (LexLeanTarget.PlannerOracle.Plan.start (target)) (LexLeanReasoning.capBound ((target + target)) _))
public theorem Plan.verifications_bounded (target : Nat) : ((((LexLeanTarget.PlannerOracle.Plan.run (target)).1).ledger).verifications <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [LexLeanTarget.PlannerOracle.Plan.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : LexLeanTarget.PlannerOracle.Plan.Search) => LexLeanTarget.PlannerOracle.Plan.searchStep (target) (__r))) ((fun (__r : LexLeanTarget.PlannerOracle.Plan.Search) => ((__r).ledger).verifications)) (LexLeanTarget.PlannerOracle.Plan.search_verify_growth (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (LexLeanTarget.PlannerOracle.Plan.start (target)) rfl)
public theorem Plan.expansions_bounded (target : Nat) : ((((LexLeanTarget.PlannerOracle.Plan.run (target)).1).ledger).expansions <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [LexLeanTarget.PlannerOracle.Plan.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : LexLeanTarget.PlannerOracle.Plan.Search) => LexLeanTarget.PlannerOracle.Plan.searchStep (target) (__r))) ((fun (__r : LexLeanTarget.PlannerOracle.Plan.Search) => ((__r).ledger).expansions)) (LexLeanTarget.PlannerOracle.Plan.search_expanded (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (LexLeanTarget.PlannerOracle.Plan.start (target)) rfl)
public theorem Plan.iterations_bounded (target : Nat) : ((((LexLeanTarget.PlannerOracle.Plan.run (target)).1).ledger).iterations <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [LexLeanTarget.PlannerOracle.Plan.run]
  exact (LexLeanReasoning.iterateUntilCount ((fun (__r : LexLeanTarget.PlannerOracle.Plan.Search) => LexLeanTarget.PlannerOracle.Plan.searchStep (target) (__r))) ((fun (__r : LexLeanTarget.PlannerOracle.Plan.Search) => ((__r).ledger).iterations)) (LexLeanTarget.PlannerOracle.Plan.search_count (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (LexLeanTarget.PlannerOracle.Plan.start (target)) rfl)
public theorem Plan.explained (target : Nat) : (forall (__v : (Prod (Nat) (Nat))), (forall (__trace : List (LexLeanTarget.PlannerOracle.Plan.Step)), ((LexLeanTarget.PlannerOracle.Plan (target) = Except.ok ((__v, __trace))) -> ((LexLeanTarget.PlannerOracle.Plan.answer (target) (__trace) = Option.some (__v)) /\ LexLeanTarget.PlannerOracle.Measures (target) (__v))))) :=
by
  intro llV llT llE
  have llSearch := LexLeanTarget.PlannerOracle.Plan.search_ok target
  dsimp only [LexLeanTarget.PlannerOracle.Plan] at llE
  generalize llRun : LexLeanTarget.PlannerOracle.Plan.run target = llR at llE llSearch
  split at llE
  · rename_i llHit llF
    cases llE
    have llOk := And.right llSearch
    rw [llF] at llOk
    exact And.intro (by dsimp only [LexLeanTarget.PlannerOracle.Plan.answer]; rw [And.left llOk]; exact And.right llOk) (LexLeanTarget.PlannerOracle.Plan.accept_sound _ _ _ (And.right llOk))
  · cases llE
public theorem Plan.verdict_sound (target : Nat) : (forall (__v : (Prod (Nat) (Nat))), ((LexLeanTarget.PlannerOracle.Plan.verdict (target) = Except.ok (__v)) -> LexLeanTarget.PlannerOracle.Measures (target) (__v))) :=
by
  intro llV llE
  dsimp only [LexLeanTarget.PlannerOracle.Plan.verdict] at llE
  generalize llHP : LexLeanTarget.PlannerOracle.Plan target = llR at llE
  cases llR with
  | error _ => cases llE
  | ok llP =>
    cases llE
    exact And.right (LexLeanTarget.PlannerOracle.Plan.explained _ llP.1 llP.2 llHP)

@[expose, reducible] public def Within (_target : Nat) (r : Nat) : Prop := (r <= 4)

@[expose] public def withinCheck (_target : Nat) (r : Nat) : Bool := (Nat.ble (r) (4))

public theorem within_sound (target : Nat) (r : Nat) : ((withinCheck (target) (r) = true) -> Within (target) (r)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Within, withinCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem Inside.sound : (LexLeanReasoning.Sound ((LexLeanTarget.PlannerOracle.withinCheck)) ((LexLeanTarget.PlannerOracle.Within))) :=
  LexLeanTarget.PlannerOracle.within_sound

@[expose, reducible] public def Whole (_target : Nat) (r : Nat) : Prop := (0 <= r)

@[expose] public def wholeCheck (_target : Nat) (r : Nat) : Bool := (Nat.ble (0) (r))

public theorem whole_sound (target : Nat) (r : Nat) : ((wholeCheck (target) (r) = true) -> Whole (target) (r)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Whole, wholeCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem Anywhere.sound : (LexLeanReasoning.Sound ((LexLeanTarget.PlannerOracle.wholeCheck)) ((LexLeanTarget.PlannerOracle.Whole))) :=
  LexLeanTarget.PlannerOracle.whole_sound

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
@[expose] public def PlanFit.fire (__s : (Prod (Nat) (Nat))) (__step : LexLeanTarget.PlannerOracle.PlanFit.Step) : Option ((Prod (Nat) (Nat))) := (match __step with | LexLeanTarget.PlannerOracle.PlanFit.Step.FillA => LexLeanTarget.PlannerOracle.FillA.apply (__s) | LexLeanTarget.PlannerOracle.PlanFit.Step.EmptyB => LexLeanTarget.PlannerOracle.EmptyB.apply (__s) | LexLeanTarget.PlannerOracle.PlanFit.Step.Pour __b => LexLeanTarget.PlannerOracle.Pour.apply (__s) (__b))
@[expose] public def PlanFit.replay (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : LexLeanTarget.PlannerOracle.PlanFit.Step) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match LexLeanTarget.PlannerOracle.PlanFit.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def PlanFit.follow (target : Nat) (__trace : List (LexLeanTarget.PlannerOracle.PlanFit.Step)) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (LexLeanCollections.listFold ((LexLeanTarget.PlannerOracle.PlanFit.replay)) (Except.ok (LexLeanTarget.PlannerOracle.PlanFit.observe (target))) (__trace) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))))
@[expose] public def PlanFit.extract (jugs : (Prod (Nat) (Nat))) : Option (Nat) := Option.some ((jugs).1)
@[expose] public def PlanFit.accept (_target : Nat) (__s : (Prod (Nat) (Nat))) : Option (Nat) := LexLeanTarget.PlannerOracle.PlanFit.extract (__s)
@[expose] public def PlanFit.answer (target : Nat) (__trace : List (LexLeanTarget.PlannerOracle.PlanFit.Step)) : Option (Nat) := (match LexLeanTarget.PlannerOracle.PlanFit.follow (target) (__trace) with | Except.ok __s => LexLeanTarget.PlannerOracle.PlanFit.accept (target) (__s) | Except.error _ => Option.none)
public structure PlanFit.Node where
  state : (Prod (Nat) (Nat))
  trace : List (LexLeanTarget.PlannerOracle.PlanFit.Step)
public structure PlanFit.Search where
  frontier : List (LexLeanTarget.PlannerOracle.PlanFit.Node)
  visited : List ((Prod (Nat) (Nat)))
  found : Option ((Prod (Nat) (LexLeanTarget.PlannerOracle.PlanFit.Node)))
  truncated : Bool
  ledger : LexLeanTarget.PlannerOracle.PlanFit.Ledger
@[expose] public def PlanFit.FillA.successors (__node : LexLeanTarget.PlannerOracle.PlanFit.Node) : List (LexLeanTarget.PlannerOracle.PlanFit.Node) := (match LexLeanTarget.PlannerOracle.PlanFit.fire ((__node).state) (LexLeanTarget.PlannerOracle.PlanFit.Step.FillA) with | Option.none => ([] : List (LexLeanTarget.PlannerOracle.PlanFit.Node)) | Option.some __t => (({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((LexLeanTarget.PlannerOracle.PlanFit.Step.FillA :: ([] : List (LexLeanTarget.PlannerOracle.PlanFit.Step)))) : List (LexLeanTarget.PlannerOracle.PlanFit.Step)) } : LexLeanTarget.PlannerOracle.PlanFit.Node) :: ([] : List (LexLeanTarget.PlannerOracle.PlanFit.Node))))
@[expose] public def PlanFit.EmptyB.successors (__node : LexLeanTarget.PlannerOracle.PlanFit.Node) : List (LexLeanTarget.PlannerOracle.PlanFit.Node) := (match LexLeanTarget.PlannerOracle.PlanFit.fire ((__node).state) (LexLeanTarget.PlannerOracle.PlanFit.Step.EmptyB) with | Option.none => ([] : List (LexLeanTarget.PlannerOracle.PlanFit.Node)) | Option.some __t => (({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((LexLeanTarget.PlannerOracle.PlanFit.Step.EmptyB :: ([] : List (LexLeanTarget.PlannerOracle.PlanFit.Step)))) : List (LexLeanTarget.PlannerOracle.PlanFit.Step)) } : LexLeanTarget.PlannerOracle.PlanFit.Node) :: ([] : List (LexLeanTarget.PlannerOracle.PlanFit.Node))))
@[expose] public def PlanFit.Pour.collect (__node : LexLeanTarget.PlannerOracle.PlanFit.Node) (__acc : List (LexLeanTarget.PlannerOracle.PlanFit.Node)) (__b : Nat) : List (LexLeanTarget.PlannerOracle.PlanFit.Node) := (match LexLeanTarget.PlannerOracle.PlanFit.fire ((__node).state) (LexLeanTarget.PlannerOracle.PlanFit.Step.Pour (__b)) with | Option.none => __acc | Option.some __t => (LexLeanRuntime.append (__acc) ((({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((LexLeanTarget.PlannerOracle.PlanFit.Step.Pour (__b) :: ([] : List (LexLeanTarget.PlannerOracle.PlanFit.Step)))) : List (LexLeanTarget.PlannerOracle.PlanFit.Step)) } : LexLeanTarget.PlannerOracle.PlanFit.Node) :: ([] : List (LexLeanTarget.PlannerOracle.PlanFit.Node)))) : List (LexLeanTarget.PlannerOracle.PlanFit.Node)))
@[expose] public def PlanFit.Pour.successors (__node : LexLeanTarget.PlannerOracle.PlanFit.Node) : List (LexLeanTarget.PlannerOracle.PlanFit.Node) := (LexLeanCollections.listFold ((fun (__acc : List (LexLeanTarget.PlannerOracle.PlanFit.Node)) (__b : Nat) => LexLeanTarget.PlannerOracle.PlanFit.Pour.collect (__node) (__acc) (__b))) (([] : List (LexLeanTarget.PlannerOracle.PlanFit.Node))) (LexLeanTarget.PlannerOracle.Pour.candidates ((__node).state)) : List (LexLeanTarget.PlannerOracle.PlanFit.Node))
@[expose] public def PlanFit.successors (__node : LexLeanTarget.PlannerOracle.PlanFit.Node) : List (LexLeanTarget.PlannerOracle.PlanFit.Node) := (LexLeanRuntime.append (LexLeanTarget.PlannerOracle.PlanFit.FillA.successors (__node)) ((LexLeanRuntime.append (LexLeanTarget.PlannerOracle.PlanFit.EmptyB.successors (__node)) (LexLeanTarget.PlannerOracle.PlanFit.Pour.successors (__node)) : List (LexLeanTarget.PlannerOracle.PlanFit.Node))) : List (LexLeanTarget.PlannerOracle.PlanFit.Node))
@[expose] public def PlanFit.fresh (__visited : List ((Prod (Nat) (Nat)))) (__nodes : List (LexLeanTarget.PlannerOracle.PlanFit.Node)) : (Prod (List (LexLeanTarget.PlannerOracle.PlanFit.Node)) (List ((Prod (Nat) (Nat))))) := (LexLeanCollections.listFold ((fun (__acc : (Prod (List (LexLeanTarget.PlannerOracle.PlanFit.Node)) (List ((Prod (Nat) (Nat)))))) (__n : LexLeanTarget.PlannerOracle.PlanFit.Node) => (if (LexLeanCollections.setContains ((__acc).2) ((__n).state) : Bool) then __acc else ((LexLeanRuntime.append ((__acc).1) ((__n :: ([] : List (LexLeanTarget.PlannerOracle.PlanFit.Node)))) : List (LexLeanTarget.PlannerOracle.PlanFit.Node)), (LexLeanCollections.setInsert ((__acc).2) ((__n).state) : List ((Prod (Nat) (Nat)))))))) ((([] : List (LexLeanTarget.PlannerOracle.PlanFit.Node)), __visited)) (__nodes) : (Prod (List (LexLeanTarget.PlannerOracle.PlanFit.Node)) (List ((Prod (Nat) (Nat))))))
@[expose] public def PlanFit.start (target : Nat) : LexLeanTarget.PlannerOracle.PlanFit.Search := (let __first : List (LexLeanTarget.PlannerOracle.PlanFit.Node) := (LexLeanCollections.listFold ((fun (__acc : List (LexLeanTarget.PlannerOracle.PlanFit.Node)) (__n : LexLeanTarget.PlannerOracle.PlanFit.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) ((target + target))) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (LexLeanTarget.PlannerOracle.PlanFit.Node)))) : List (LexLeanTarget.PlannerOracle.PlanFit.Node)) else __acc))) (([] : List (LexLeanTarget.PlannerOracle.PlanFit.Node))) ((({ state := LexLeanTarget.PlannerOracle.PlanFit.observe (target), trace := ([] : List (LexLeanTarget.PlannerOracle.PlanFit.Step)) } : LexLeanTarget.PlannerOracle.PlanFit.Node) :: ([] : List (LexLeanTarget.PlannerOracle.PlanFit.Node)))) : List (LexLeanTarget.PlannerOracle.PlanFit.Node)); ({ frontier := __first, visited := (LexLeanCollections.setInsert (([] : List ((Prod (Nat) (Nat))))) (LexLeanTarget.PlannerOracle.PlanFit.observe (target)) : List ((Prod (Nat) (Nat)))), found := Option.none, truncated := (Nat.blt ((target + target)) (1)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := (LexLeanRuntime.length (__first) : Nat) } : LexLeanTarget.PlannerOracle.PlanFit.Ledger) } : LexLeanTarget.PlannerOracle.PlanFit.Search))
@[expose] public def PlanFit.attempts (__s : (Prod (Nat) (Nat))) : Nat := (1 + (1 + (LexLeanRuntime.length (LexLeanTarget.PlannerOracle.Pour.candidates (__s)) : Nat)))
@[expose] public def PlanFit.searchStep (target : Nat) (__r : LexLeanTarget.PlannerOracle.PlanFit.Search) : Option (LexLeanTarget.PlannerOracle.PlanFit.Search) := (match (__r).found with | Option.some _ => Option.none | Option.none => (match (__r).frontier with | List.nil => Option.none | List.cons __node __rest => (match LexLeanTarget.PlannerOracle.PlanFit.accept (target) ((__node).state) with | Option.some __v => Option.some (({ frontier := __rest, visited := (__r).visited, found := Option.some ((__v, __node)), truncated := (__r).truncated, ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := ((__r).ledger).attempts, firings := ((__r).ledger).firings, expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : LexLeanTarget.PlannerOracle.PlanFit.Ledger) } : LexLeanTarget.PlannerOracle.PlanFit.Search)) | Option.none => (let __successors : List (LexLeanTarget.PlannerOracle.PlanFit.Node) := LexLeanTarget.PlannerOracle.PlanFit.successors (__node); (let __fresh : (Prod (List (LexLeanTarget.PlannerOracle.PlanFit.Node)) (List ((Prod (Nat) (Nat))))) := LexLeanTarget.PlannerOracle.PlanFit.fresh ((__r).visited) (__successors); (let __ordered : List (LexLeanTarget.PlannerOracle.PlanFit.Node) := (LexLeanRuntime.append (__rest) ((__fresh).1) : List (LexLeanTarget.PlannerOracle.PlanFit.Node)); (let __next : List (LexLeanTarget.PlannerOracle.PlanFit.Node) := (LexLeanCollections.listFold ((fun (__acc : List (LexLeanTarget.PlannerOracle.PlanFit.Node)) (__n : LexLeanTarget.PlannerOracle.PlanFit.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) ((target + target))) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (LexLeanTarget.PlannerOracle.PlanFit.Node)))) : List (LexLeanTarget.PlannerOracle.PlanFit.Node)) else __acc))) (([] : List (LexLeanTarget.PlannerOracle.PlanFit.Node))) (__ordered) : List (LexLeanTarget.PlannerOracle.PlanFit.Node)); Option.some (({ frontier := __next, visited := (__fresh).2, found := Option.none, truncated := ((__r).truncated || (Nat.blt ((target + target)) ((LexLeanRuntime.length (__ordered) : Nat)))), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + LexLeanTarget.PlannerOracle.PlanFit.attempts ((__node).state)), firings := (((__r).ledger).firings + (LexLeanRuntime.length (__successors) : Nat)), expansions := (((__r).ledger).expansions + 1), verifications := ((__r).ledger).verifications, frontier := (if (Nat.blt (((__r).ledger).frontier) ((LexLeanRuntime.length (__next) : Nat))) then (LexLeanRuntime.length (__next) : Nat) else ((__r).ledger).frontier) } : LexLeanTarget.PlannerOracle.PlanFit.Ledger) } : LexLeanTarget.PlannerOracle.PlanFit.Search)))))))))
@[expose] public def PlanFit.run (target : Nat) : (Prod (LexLeanTarget.PlannerOracle.PlanFit.Search) (Bool)) := (LexLeanCollections.iterateUntil ((fun (__r : LexLeanTarget.PlannerOracle.PlanFit.Search) => LexLeanTarget.PlannerOracle.PlanFit.searchStep (target) (__r))) ((LexLeanRuntime.multiply (target) (8) : Nat)) (LexLeanTarget.PlannerOracle.PlanFit.start (target)) : (Prod (LexLeanTarget.PlannerOracle.PlanFit.Search) (Bool)))
@[expose] public def PlanFit.failure (__saturated : Bool) (__truncated : Bool) : (Prod Bool Bool) := (if (__saturated && (!__truncated)) then ((false, true) : Prod Bool Bool) else ((false, false) : Prod Bool Bool))
@[expose] public def PlanFit (target : Nat) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (LexLeanTarget.PlannerOracle.PlanFit.Step)))) := (let __final : (Prod (LexLeanTarget.PlannerOracle.PlanFit.Search) (Bool)) := LexLeanTarget.PlannerOracle.PlanFit.run (target); (match ((__final).1).found with | Option.some __hit => Except.ok (((__hit).1, ((__hit).2).trace)) | Option.none => Except.error (LexLeanTarget.PlannerOracle.PlanFit.failure ((__final).2) (((__final).1).truncated))))
@[expose] public def PlanFit.verdict (target : Nat) : Except ((Prod Bool Bool)) (Nat) := (match LexLeanTarget.PlannerOracle.PlanFit (target) with | Except.ok __p => Except.ok ((__p).1) | Except.error __e => Except.error (__e))
public theorem PlanFit.fire_sound (__s : (Prod (Nat) (Nat))) (__step : LexLeanTarget.PlannerOracle.PlanFit.Step) (__t : (Prod (Nat) (Nat))) : ((LexLeanTarget.PlannerOracle.PlanFit.fire (__s) (__step) = Option.some (__t)) -> LexLeanTarget.PlannerOracle.Moves (__s) (__t)) :=
by
  cases __step with
  | FillA =>
    exact LexLeanTarget.PlannerOracle.FillA.apply_sound __s __t
  | EmptyB =>
    exact LexLeanTarget.PlannerOracle.EmptyB.apply_sound __s __t
  | Pour __b =>
    exact LexLeanTarget.PlannerOracle.Pour.apply_sound __s __b __t
public theorem PlanFit.replay_fire (__s : (Prod (Nat) (Nat))) (__step : LexLeanTarget.PlannerOracle.PlanFit.Step) (__t : (Prod (Nat) (Nat))) : ((LexLeanTarget.PlannerOracle.PlanFit.fire (__s) (__step) = Option.some (__t)) -> (LexLeanTarget.PlannerOracle.PlanFit.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [LexLeanTarget.PlannerOracle.PlanFit.replay]
  rw [llE]
public theorem PlanFit.replay_sound (target : Nat) (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : LexLeanTarget.PlannerOracle.PlanFit.Step) : ((LexLeanReasoning.Reaches ((LexLeanTarget.PlannerOracle.Moves)) (LexLeanTarget.PlannerOracle.PlanFit.observe (target)) (__acc)) -> (LexLeanReasoning.Reaches ((LexLeanTarget.PlannerOracle.Moves)) (LexLeanTarget.PlannerOracle.PlanFit.observe (target)) (LexLeanTarget.PlannerOracle.PlanFit.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [LexLeanTarget.PlannerOracle.PlanFit.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (LexLeanTarget.PlannerOracle.PlanFit.fire_sound llS __step _ ‹_›)
public theorem PlanFit.derivation (target : Nat) (__trace : List (LexLeanTarget.PlannerOracle.PlanFit.Step)) : (LexLeanReasoning.Reaches ((LexLeanTarget.PlannerOracle.Moves)) (LexLeanTarget.PlannerOracle.PlanFit.observe (target)) (LexLeanTarget.PlannerOracle.PlanFit.follow (target) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((LexLeanTarget.PlannerOracle.PlanFit.replay)) (fun (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) => (LexLeanReasoning.Reaches ((LexLeanTarget.PlannerOracle.Moves)) (LexLeanTarget.PlannerOracle.PlanFit.observe (target)) (__acc))) (LexLeanTarget.PlannerOracle.PlanFit.replay_sound (target)) (__trace) (Except.ok (LexLeanTarget.PlannerOracle.PlanFit.observe (target))) (LexLeanReasoning.reachesStart ((LexLeanTarget.PlannerOracle.Moves)) (LexLeanTarget.PlannerOracle.PlanFit.observe (target))))
public theorem PlanFit.follow_invariant (target : Nat) (__trace : List (LexLeanTarget.PlannerOracle.PlanFit.Step)) (__s : (Prod (Nat) (Nat))) : ((LexLeanTarget.PlannerOracle.PlanFit.follow (target) (__trace) = Except.ok (__s)) -> LexLeanTarget.PlannerOracle.Fits (__s)) :=
  (fun llE => (LexLeanReasoning.starPreserves ((LexLeanTarget.PlannerOracle.Moves)) ((LexLeanTarget.PlannerOracle.Fits)) LexLeanTarget.PlannerOracle.Pouring.preserves (LexLeanTarget.PlannerOracle.PlanFit.observe (target)) (__s) (LexLeanTarget.PlannerOracle.PlanFit.derivation (target) (__trace) (__s) llE) (LexLeanTarget.PlannerOracle.empty_jugs_fit (target))))
public theorem PlanFit.accept_sound (target : Nat) (__s : (Prod (Nat) (Nat))) (__v : Nat) : (LexLeanTarget.PlannerOracle.Fits (__s) -> ((LexLeanTarget.PlannerOracle.PlanFit.accept (target) (__s) = Option.some (__v)) -> LexLeanTarget.PlannerOracle.Within (target) (__v))) :=
  (fun llJ llE => (LexLeanTarget.PlannerOracle.plan_fit_correct (target) (__s) (__v) llJ llE))
public theorem PlanFit.extend (target : Nat) (__node : LexLeanTarget.PlannerOracle.PlanFit.Node) (__step : LexLeanTarget.PlannerOracle.PlanFit.Step) (__t : (Prod (Nat) (Nat))) : ((LexLeanTarget.PlannerOracle.PlanFit.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> ((LexLeanTarget.PlannerOracle.PlanFit.fire ((__node).state) (__step) = Option.some (__t)) -> (LexLeanTarget.PlannerOracle.PlanFit.follow (target) ((LexLeanRuntime.append ((__node).trace) ((__step :: ([] : List (LexLeanTarget.PlannerOracle.PlanFit.Step)))) : List (LexLeanTarget.PlannerOracle.PlanFit.Step))) = Except.ok (__t)))) :=
by
  intro llH llE
  dsimp only [LexLeanTarget.PlannerOracle.PlanFit.follow]
  rw [LexLeanReasoning.foldSnoc]
  dsimp only [LexLeanTarget.PlannerOracle.PlanFit.follow] at llH
  rw [llH]
  exact LexLeanTarget.PlannerOracle.PlanFit.replay_fire _ __step __t llE
public theorem PlanFit.FillA.successors_ok (target : Nat) (__node : LexLeanTarget.PlannerOracle.PlanFit.Node) : ((LexLeanTarget.PlannerOracle.PlanFit.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : LexLeanTarget.PlannerOracle.PlanFit.Node) => (LexLeanTarget.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (LexLeanTarget.PlannerOracle.PlanFit.FillA.successors (__node)))) :=
by
  intro llH
  dsimp only [LexLeanTarget.PlannerOracle.PlanFit.FillA.successors]
  split
  · exact LexLeanReasoning.All.nil
  · exact LexLeanReasoning.allSingle _ _ (LexLeanTarget.PlannerOracle.PlanFit.extend _ __node _ _ llH ‹_›)
public theorem PlanFit.EmptyB.successors_ok (target : Nat) (__node : LexLeanTarget.PlannerOracle.PlanFit.Node) : ((LexLeanTarget.PlannerOracle.PlanFit.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : LexLeanTarget.PlannerOracle.PlanFit.Node) => (LexLeanTarget.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (LexLeanTarget.PlannerOracle.PlanFit.EmptyB.successors (__node)))) :=
by
  intro llH
  dsimp only [LexLeanTarget.PlannerOracle.PlanFit.EmptyB.successors]
  split
  · exact LexLeanReasoning.All.nil
  · exact LexLeanReasoning.allSingle _ _ (LexLeanTarget.PlannerOracle.PlanFit.extend _ __node _ _ llH ‹_›)
public theorem PlanFit.Pour.successors_ok (target : Nat) (__node : LexLeanTarget.PlannerOracle.PlanFit.Node) : ((LexLeanTarget.PlannerOracle.PlanFit.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : LexLeanTarget.PlannerOracle.PlanFit.Node) => (LexLeanTarget.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (LexLeanTarget.PlannerOracle.PlanFit.Pour.successors (__node)))) :=
by
  intro llH
  dsimp only [LexLeanTarget.PlannerOracle.PlanFit.Pour.successors]
  exact LexLeanReasoning.foldInvariant _ (LexLeanReasoning.All (fun (llN : LexLeanTarget.PlannerOracle.PlanFit.Node) => LexLeanTarget.PlannerOracle.PlanFit.follow _ llN.trace = Except.ok llN.state))
    (fun llA llB llP => by
      dsimp only [LexLeanTarget.PlannerOracle.PlanFit.Pour.collect]
      split
      · exact llP
      · exact LexLeanReasoning.allAppend _ _ _ llP (LexLeanReasoning.allSingle _ _ (LexLeanTarget.PlannerOracle.PlanFit.extend _ __node _ _ llH ‹_›)))
    _ _ LexLeanReasoning.All.nil
public theorem PlanFit.successors_ok (target : Nat) (__node : LexLeanTarget.PlannerOracle.PlanFit.Node) : ((LexLeanTarget.PlannerOracle.PlanFit.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : LexLeanTarget.PlannerOracle.PlanFit.Node) => (LexLeanTarget.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (LexLeanTarget.PlannerOracle.PlanFit.successors (__node)))) :=
  (fun llH => (LexLeanReasoning.allAppend _ _ _ (LexLeanTarget.PlannerOracle.PlanFit.FillA.successors_ok (target) (__node) llH) (LexLeanReasoning.allAppend _ _ _ (LexLeanTarget.PlannerOracle.PlanFit.EmptyB.successors_ok (target) (__node) llH) (LexLeanTarget.PlannerOracle.PlanFit.Pour.successors_ok (target) (__node) llH))))
public theorem PlanFit.fresh_ok (target : Nat) (__visited : List ((Prod (Nat) (Nat)))) (__nodes : List (LexLeanTarget.PlannerOracle.PlanFit.Node)) : ((LexLeanReasoning.All ((fun (__n : LexLeanTarget.PlannerOracle.PlanFit.Node) => (LexLeanTarget.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (__nodes)) -> (LexLeanReasoning.All ((fun (__n : LexLeanTarget.PlannerOracle.PlanFit.Node) => (LexLeanTarget.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((LexLeanTarget.PlannerOracle.PlanFit.fresh (__visited) (__nodes)).1))) :=
  (LexLeanReasoning.freshAll ((fun (__n : LexLeanTarget.PlannerOracle.PlanFit.Node) => (__n).state)) (fun (__n : LexLeanTarget.PlannerOracle.PlanFit.Node) => (LexLeanTarget.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state))) (__visited) (__nodes))
public theorem PlanFit.search_step (target : Nat) (__r : LexLeanTarget.PlannerOracle.PlanFit.Search) (__q : LexLeanTarget.PlannerOracle.PlanFit.Search) : ((LexLeanTarget.PlannerOracle.PlanFit.searchStep (target) (__r) = Option.some (__q)) -> ((LexLeanReasoning.SearchOk ((fun (__n : LexLeanTarget.PlannerOracle.PlanFit.Node) => (LexLeanTarget.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : LexLeanTarget.PlannerOracle.PlanFit.Node) => LexLeanTarget.PlannerOracle.PlanFit.accept (target) ((__n).state))) ((__r).frontier) ((__r).found)) -> (LexLeanReasoning.SearchOk ((fun (__n : LexLeanTarget.PlannerOracle.PlanFit.Node) => (LexLeanTarget.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : LexLeanTarget.PlannerOracle.PlanFit.Node) => LexLeanTarget.PlannerOracle.PlanFit.accept (target) ((__n).state))) ((__q).frontier) ((__q).found)))) :=
by
  intro llE llH
  dsimp only [LexLeanTarget.PlannerOracle.PlanFit.searchStep] at llE
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
        exact And.intro (LexLeanReasoning.capAll _ _ _ (LexLeanReasoning.allAppend _ _ _ llRestOk (LexLeanTarget.PlannerOracle.PlanFit.fresh_ok _ _ _ (LexLeanTarget.PlannerOracle.PlanFit.successors_ok _ llNode llNodeOk)))) True.intro
public theorem PlanFit.search_peak (target : Nat) (__r : LexLeanTarget.PlannerOracle.PlanFit.Search) (__q : LexLeanTarget.PlannerOracle.PlanFit.Search) : ((LexLeanTarget.PlannerOracle.PlanFit.searchStep (target) (__r) = Option.some (__q)) -> ((((__r).ledger).frontier <= (target + target)) -> (((__q).ledger).frontier <= (target + target)))) :=
by
  intro llE llH
  dsimp only [LexLeanTarget.PlannerOracle.PlanFit.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        exact llH
      · cases llE
        exact LexLeanReasoning.peakBound _ _ _ llH (LexLeanReasoning.capBound _ _)
public theorem PlanFit.search_count (target : Nat) (__r : LexLeanTarget.PlannerOracle.PlanFit.Search) (__q : LexLeanTarget.PlannerOracle.PlanFit.Search) : ((LexLeanTarget.PlannerOracle.PlanFit.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [LexLeanTarget.PlannerOracle.PlanFit.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        rfl
      · cases llE
        rfl
public theorem PlanFit.search_verify_growth (target : Nat) (__r : LexLeanTarget.PlannerOracle.PlanFit.Search) (__q : LexLeanTarget.PlannerOracle.PlanFit.Search) : ((LexLeanTarget.PlannerOracle.PlanFit.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).verifications <= (((__r).ledger).verifications + 1))) :=
by
  intro llE
  dsimp only [LexLeanTarget.PlannerOracle.PlanFit.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem PlanFit.search_expanded (target : Nat) (__r : LexLeanTarget.PlannerOracle.PlanFit.Search) (__q : LexLeanTarget.PlannerOracle.PlanFit.Search) : ((LexLeanTarget.PlannerOracle.PlanFit.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).expansions <= (((__r).ledger).expansions + 1))) :=
by
  intro llE
  dsimp only [LexLeanTarget.PlannerOracle.PlanFit.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem PlanFit.search_ok (target : Nat) : (LexLeanReasoning.SearchOk ((fun (__n : LexLeanTarget.PlannerOracle.PlanFit.Node) => (LexLeanTarget.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : LexLeanTarget.PlannerOracle.PlanFit.Node) => LexLeanTarget.PlannerOracle.PlanFit.accept (target) ((__n).state))) (((LexLeanTarget.PlannerOracle.PlanFit.run (target)).1).frontier) (((LexLeanTarget.PlannerOracle.PlanFit.run (target)).1).found)) :=
by
  dsimp only [LexLeanTarget.PlannerOracle.PlanFit.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((fun (__r : LexLeanTarget.PlannerOracle.PlanFit.Search) => LexLeanTarget.PlannerOracle.PlanFit.searchStep (target) (__r))) (fun (__r : LexLeanTarget.PlannerOracle.PlanFit.Search) => (LexLeanReasoning.SearchOk ((fun (__n : LexLeanTarget.PlannerOracle.PlanFit.Node) => (LexLeanTarget.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : LexLeanTarget.PlannerOracle.PlanFit.Node) => LexLeanTarget.PlannerOracle.PlanFit.accept (target) ((__n).state))) ((__r).frontier) ((__r).found))) (LexLeanTarget.PlannerOracle.PlanFit.search_step (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (LexLeanTarget.PlannerOracle.PlanFit.start (target)) (LexLeanReasoning.searchStart ((fun (__n : LexLeanTarget.PlannerOracle.PlanFit.Node) => (LexLeanTarget.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : LexLeanTarget.PlannerOracle.PlanFit.Node) => LexLeanTarget.PlannerOracle.PlanFit.accept (target) ((__n).state))) _ (LexLeanReasoning.capAll ((fun (__n : LexLeanTarget.PlannerOracle.PlanFit.Node) => (LexLeanTarget.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((target + target)) _ (LexLeanReasoning.allSingle ((fun (__n : LexLeanTarget.PlannerOracle.PlanFit.Node) => (LexLeanTarget.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (({ state := LexLeanTarget.PlannerOracle.PlanFit.observe (target), trace := ([] : List (LexLeanTarget.PlannerOracle.PlanFit.Step)) } : LexLeanTarget.PlannerOracle.PlanFit.Node)) rfl))))
public theorem PlanFit.frontier_bounded (target : Nat) : ((((LexLeanTarget.PlannerOracle.PlanFit.run (target)).1).ledger).frontier <= (target + target)) :=
by
  dsimp only [LexLeanTarget.PlannerOracle.PlanFit.run]
  exact (LexLeanReasoning.iterateUntilBound ((fun (__r : LexLeanTarget.PlannerOracle.PlanFit.Search) => LexLeanTarget.PlannerOracle.PlanFit.searchStep (target) (__r))) ((fun (__r : LexLeanTarget.PlannerOracle.PlanFit.Search) => ((__r).ledger).frontier)) ((target + target)) (LexLeanTarget.PlannerOracle.PlanFit.search_peak (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (LexLeanTarget.PlannerOracle.PlanFit.start (target)) (LexLeanReasoning.capBound ((target + target)) _))
public theorem PlanFit.verifications_bounded (target : Nat) : ((((LexLeanTarget.PlannerOracle.PlanFit.run (target)).1).ledger).verifications <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [LexLeanTarget.PlannerOracle.PlanFit.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : LexLeanTarget.PlannerOracle.PlanFit.Search) => LexLeanTarget.PlannerOracle.PlanFit.searchStep (target) (__r))) ((fun (__r : LexLeanTarget.PlannerOracle.PlanFit.Search) => ((__r).ledger).verifications)) (LexLeanTarget.PlannerOracle.PlanFit.search_verify_growth (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (LexLeanTarget.PlannerOracle.PlanFit.start (target)) rfl)
public theorem PlanFit.expansions_bounded (target : Nat) : ((((LexLeanTarget.PlannerOracle.PlanFit.run (target)).1).ledger).expansions <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [LexLeanTarget.PlannerOracle.PlanFit.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : LexLeanTarget.PlannerOracle.PlanFit.Search) => LexLeanTarget.PlannerOracle.PlanFit.searchStep (target) (__r))) ((fun (__r : LexLeanTarget.PlannerOracle.PlanFit.Search) => ((__r).ledger).expansions)) (LexLeanTarget.PlannerOracle.PlanFit.search_expanded (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (LexLeanTarget.PlannerOracle.PlanFit.start (target)) rfl)
public theorem PlanFit.iterations_bounded (target : Nat) : ((((LexLeanTarget.PlannerOracle.PlanFit.run (target)).1).ledger).iterations <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [LexLeanTarget.PlannerOracle.PlanFit.run]
  exact (LexLeanReasoning.iterateUntilCount ((fun (__r : LexLeanTarget.PlannerOracle.PlanFit.Search) => LexLeanTarget.PlannerOracle.PlanFit.searchStep (target) (__r))) ((fun (__r : LexLeanTarget.PlannerOracle.PlanFit.Search) => ((__r).ledger).iterations)) (LexLeanTarget.PlannerOracle.PlanFit.search_count (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (LexLeanTarget.PlannerOracle.PlanFit.start (target)) rfl)
public theorem PlanFit.explained (target : Nat) : (forall (__v : Nat), (forall (__trace : List (LexLeanTarget.PlannerOracle.PlanFit.Step)), ((LexLeanTarget.PlannerOracle.PlanFit (target) = Except.ok ((__v, __trace))) -> ((LexLeanTarget.PlannerOracle.PlanFit.answer (target) (__trace) = Option.some (__v)) /\ LexLeanTarget.PlannerOracle.Within (target) (__v))))) :=
by
  intro llV llT llE
  have llSearch := LexLeanTarget.PlannerOracle.PlanFit.search_ok target
  dsimp only [LexLeanTarget.PlannerOracle.PlanFit] at llE
  generalize llRun : LexLeanTarget.PlannerOracle.PlanFit.run target = llR at llE llSearch
  split at llE
  · rename_i llHit llF
    cases llE
    have llOk := And.right llSearch
    rw [llF] at llOk
    exact And.intro (by dsimp only [LexLeanTarget.PlannerOracle.PlanFit.answer]; rw [And.left llOk]; exact And.right llOk) (LexLeanTarget.PlannerOracle.PlanFit.accept_sound _ _ _ (LexLeanTarget.PlannerOracle.PlanFit.follow_invariant target _ _ (And.left llOk)) (And.right llOk))
  · cases llE
public theorem PlanFit.verdict_sound (target : Nat) : (forall (__v : Nat), ((LexLeanTarget.PlannerOracle.PlanFit.verdict (target) = Except.ok (__v)) -> LexLeanTarget.PlannerOracle.Within (target) (__v))) :=
by
  intro llV llE
  dsimp only [LexLeanTarget.PlannerOracle.PlanFit.verdict] at llE
  generalize llHP : LexLeanTarget.PlannerOracle.PlanFit target = llR at llE
  cases llR with
  | error _ => cases llE
  | ok llP =>
    cases llE
    exact And.right (LexLeanTarget.PlannerOracle.PlanFit.explained _ llP.1 llP.2 llHP)

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
@[expose] public def PlanFitR.fire (__s : (Prod (Nat) (Nat))) (__step : LexLeanTarget.PlannerOracle.PlanFitR.Step) : Option ((Prod (Nat) (Nat))) := (match __step with | LexLeanTarget.PlannerOracle.PlanFitR.Step.FillA => LexLeanTarget.PlannerOracle.FillA.apply (__s) | LexLeanTarget.PlannerOracle.PlanFitR.Step.EmptyB => LexLeanTarget.PlannerOracle.EmptyB.apply (__s) | LexLeanTarget.PlannerOracle.PlanFitR.Step.Pour __b => LexLeanTarget.PlannerOracle.Pour.apply (__s) (__b))
@[expose] public def PlanFitR.replay (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : LexLeanTarget.PlannerOracle.PlanFitR.Step) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match LexLeanTarget.PlannerOracle.PlanFitR.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def PlanFitR.follow (target : Nat) (__trace : List (LexLeanTarget.PlannerOracle.PlanFitR.Step)) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (LexLeanCollections.listFold ((LexLeanTarget.PlannerOracle.PlanFitR.replay)) (Except.ok (LexLeanTarget.PlannerOracle.PlanFitR.observe (target))) (__trace) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))))
@[expose] public def PlanFitR.extract (jugs : (Prod (Nat) (Nat))) : Option (Nat) := Option.some ((jugs).1)
@[expose] public def PlanFitR.accept (_target : Nat) (__s : (Prod (Nat) (Nat))) : Option (Nat) := LexLeanTarget.PlannerOracle.PlanFitR.extract (__s)
@[expose] public def PlanFitR.answer (target : Nat) (__trace : List (LexLeanTarget.PlannerOracle.PlanFitR.Step)) : Option (Nat) := (match LexLeanTarget.PlannerOracle.PlanFitR.follow (target) (__trace) with | Except.ok __s => LexLeanTarget.PlannerOracle.PlanFitR.accept (target) (__s) | Except.error _ => Option.none)
public structure PlanFitR.Node where
  state : (Prod (Nat) (Nat))
  trace : List (LexLeanTarget.PlannerOracle.PlanFitR.Step)
public structure PlanFitR.Search where
  frontier : List (LexLeanTarget.PlannerOracle.PlanFitR.Node)
  visited : List ((Prod (Nat) (Nat)))
  found : Option ((Prod (Nat) (LexLeanTarget.PlannerOracle.PlanFitR.Node)))
  truncated : Bool
  ledger : LexLeanTarget.PlannerOracle.PlanFitR.Ledger
@[expose] public def PlanFitR.FillA.successors (__node : LexLeanTarget.PlannerOracle.PlanFitR.Node) : List (LexLeanTarget.PlannerOracle.PlanFitR.Node) := (match LexLeanTarget.PlannerOracle.PlanFitR.fire ((__node).state) (LexLeanTarget.PlannerOracle.PlanFitR.Step.FillA) with | Option.none => ([] : List (LexLeanTarget.PlannerOracle.PlanFitR.Node)) | Option.some __t => (({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((LexLeanTarget.PlannerOracle.PlanFitR.Step.FillA :: ([] : List (LexLeanTarget.PlannerOracle.PlanFitR.Step)))) : List (LexLeanTarget.PlannerOracle.PlanFitR.Step)) } : LexLeanTarget.PlannerOracle.PlanFitR.Node) :: ([] : List (LexLeanTarget.PlannerOracle.PlanFitR.Node))))
@[expose] public def PlanFitR.EmptyB.successors (__node : LexLeanTarget.PlannerOracle.PlanFitR.Node) : List (LexLeanTarget.PlannerOracle.PlanFitR.Node) := (match LexLeanTarget.PlannerOracle.PlanFitR.fire ((__node).state) (LexLeanTarget.PlannerOracle.PlanFitR.Step.EmptyB) with | Option.none => ([] : List (LexLeanTarget.PlannerOracle.PlanFitR.Node)) | Option.some __t => (({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((LexLeanTarget.PlannerOracle.PlanFitR.Step.EmptyB :: ([] : List (LexLeanTarget.PlannerOracle.PlanFitR.Step)))) : List (LexLeanTarget.PlannerOracle.PlanFitR.Step)) } : LexLeanTarget.PlannerOracle.PlanFitR.Node) :: ([] : List (LexLeanTarget.PlannerOracle.PlanFitR.Node))))
@[expose] public def PlanFitR.Pour.collect (__node : LexLeanTarget.PlannerOracle.PlanFitR.Node) (__acc : List (LexLeanTarget.PlannerOracle.PlanFitR.Node)) (__b : Nat) : List (LexLeanTarget.PlannerOracle.PlanFitR.Node) := (match LexLeanTarget.PlannerOracle.PlanFitR.fire ((__node).state) (LexLeanTarget.PlannerOracle.PlanFitR.Step.Pour (__b)) with | Option.none => __acc | Option.some __t => (LexLeanRuntime.append (__acc) ((({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((LexLeanTarget.PlannerOracle.PlanFitR.Step.Pour (__b) :: ([] : List (LexLeanTarget.PlannerOracle.PlanFitR.Step)))) : List (LexLeanTarget.PlannerOracle.PlanFitR.Step)) } : LexLeanTarget.PlannerOracle.PlanFitR.Node) :: ([] : List (LexLeanTarget.PlannerOracle.PlanFitR.Node)))) : List (LexLeanTarget.PlannerOracle.PlanFitR.Node)))
@[expose] public def PlanFitR.Pour.successors (__node : LexLeanTarget.PlannerOracle.PlanFitR.Node) : List (LexLeanTarget.PlannerOracle.PlanFitR.Node) := (LexLeanCollections.listFold ((fun (__acc : List (LexLeanTarget.PlannerOracle.PlanFitR.Node)) (__b : Nat) => LexLeanTarget.PlannerOracle.PlanFitR.Pour.collect (__node) (__acc) (__b))) (([] : List (LexLeanTarget.PlannerOracle.PlanFitR.Node))) (LexLeanTarget.PlannerOracle.Pour.candidates ((__node).state)) : List (LexLeanTarget.PlannerOracle.PlanFitR.Node))
@[expose] public def PlanFitR.successors (__node : LexLeanTarget.PlannerOracle.PlanFitR.Node) : List (LexLeanTarget.PlannerOracle.PlanFitR.Node) := (LexLeanRuntime.append (LexLeanTarget.PlannerOracle.PlanFitR.FillA.successors (__node)) ((LexLeanRuntime.append (LexLeanTarget.PlannerOracle.PlanFitR.EmptyB.successors (__node)) (LexLeanTarget.PlannerOracle.PlanFitR.Pour.successors (__node)) : List (LexLeanTarget.PlannerOracle.PlanFitR.Node))) : List (LexLeanTarget.PlannerOracle.PlanFitR.Node))
@[expose] public def PlanFitR.fresh (__visited : List ((Prod (Nat) (Nat)))) (__nodes : List (LexLeanTarget.PlannerOracle.PlanFitR.Node)) : (Prod (List (LexLeanTarget.PlannerOracle.PlanFitR.Node)) (List ((Prod (Nat) (Nat))))) := (LexLeanCollections.listFold ((fun (__acc : (Prod (List (LexLeanTarget.PlannerOracle.PlanFitR.Node)) (List ((Prod (Nat) (Nat)))))) (__n : LexLeanTarget.PlannerOracle.PlanFitR.Node) => (if (LexLeanCollections.setContains ((__acc).2) ((__n).state) : Bool) then __acc else ((LexLeanRuntime.append ((__acc).1) ((__n :: ([] : List (LexLeanTarget.PlannerOracle.PlanFitR.Node)))) : List (LexLeanTarget.PlannerOracle.PlanFitR.Node)), (LexLeanCollections.setInsert ((__acc).2) ((__n).state) : List ((Prod (Nat) (Nat)))))))) ((([] : List (LexLeanTarget.PlannerOracle.PlanFitR.Node)), __visited)) (__nodes) : (Prod (List (LexLeanTarget.PlannerOracle.PlanFitR.Node)) (List ((Prod (Nat) (Nat))))))
@[expose] public def PlanFitR.start (target : Nat) : LexLeanTarget.PlannerOracle.PlanFitR.Search := (let __first : List (LexLeanTarget.PlannerOracle.PlanFitR.Node) := (LexLeanCollections.listFold ((fun (__acc : List (LexLeanTarget.PlannerOracle.PlanFitR.Node)) (__n : LexLeanTarget.PlannerOracle.PlanFitR.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) ((target + target))) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (LexLeanTarget.PlannerOracle.PlanFitR.Node)))) : List (LexLeanTarget.PlannerOracle.PlanFitR.Node)) else __acc))) (([] : List (LexLeanTarget.PlannerOracle.PlanFitR.Node))) ((({ state := LexLeanTarget.PlannerOracle.PlanFitR.observe (target), trace := ([] : List (LexLeanTarget.PlannerOracle.PlanFitR.Step)) } : LexLeanTarget.PlannerOracle.PlanFitR.Node) :: ([] : List (LexLeanTarget.PlannerOracle.PlanFitR.Node)))) : List (LexLeanTarget.PlannerOracle.PlanFitR.Node)); ({ frontier := __first, visited := (LexLeanCollections.setInsert (([] : List ((Prod (Nat) (Nat))))) (LexLeanTarget.PlannerOracle.PlanFitR.observe (target)) : List ((Prod (Nat) (Nat)))), found := Option.none, truncated := (Nat.blt ((target + target)) (1)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := (LexLeanRuntime.length (__first) : Nat) } : LexLeanTarget.PlannerOracle.PlanFitR.Ledger) } : LexLeanTarget.PlannerOracle.PlanFitR.Search))
@[expose] public def PlanFitR.attempts (__s : (Prod (Nat) (Nat))) : Nat := (1 + (1 + (LexLeanRuntime.length (LexLeanTarget.PlannerOracle.Pour.candidates (__s)) : Nat)))
@[expose] public def PlanFitR.searchStep (target : Nat) (__r : LexLeanTarget.PlannerOracle.PlanFitR.Search) : Option (LexLeanTarget.PlannerOracle.PlanFitR.Search) := (match (__r).found with | Option.some _ => Option.none | Option.none => (match (__r).frontier with | List.nil => Option.none | List.cons __node __rest => (match LexLeanTarget.PlannerOracle.PlanFitR.accept (target) ((__node).state) with | Option.some __v => Option.some (({ frontier := __rest, visited := (__r).visited, found := Option.some ((__v, __node)), truncated := (__r).truncated, ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := ((__r).ledger).attempts, firings := ((__r).ledger).firings, expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : LexLeanTarget.PlannerOracle.PlanFitR.Ledger) } : LexLeanTarget.PlannerOracle.PlanFitR.Search)) | Option.none => (let __successors : List (LexLeanTarget.PlannerOracle.PlanFitR.Node) := LexLeanTarget.PlannerOracle.PlanFitR.successors (__node); (let __fresh : (Prod (List (LexLeanTarget.PlannerOracle.PlanFitR.Node)) (List ((Prod (Nat) (Nat))))) := LexLeanTarget.PlannerOracle.PlanFitR.fresh ((__r).visited) (__successors); (let __ordered : List (LexLeanTarget.PlannerOracle.PlanFitR.Node) := (LexLeanRuntime.append (__rest) ((__fresh).1) : List (LexLeanTarget.PlannerOracle.PlanFitR.Node)); (let __next : List (LexLeanTarget.PlannerOracle.PlanFitR.Node) := (LexLeanCollections.listFold ((fun (__acc : List (LexLeanTarget.PlannerOracle.PlanFitR.Node)) (__n : LexLeanTarget.PlannerOracle.PlanFitR.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) ((target + target))) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (LexLeanTarget.PlannerOracle.PlanFitR.Node)))) : List (LexLeanTarget.PlannerOracle.PlanFitR.Node)) else __acc))) (([] : List (LexLeanTarget.PlannerOracle.PlanFitR.Node))) (__ordered) : List (LexLeanTarget.PlannerOracle.PlanFitR.Node)); Option.some (({ frontier := __next, visited := (__fresh).2, found := Option.none, truncated := ((__r).truncated || (Nat.blt ((target + target)) ((LexLeanRuntime.length (__ordered) : Nat)))), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + LexLeanTarget.PlannerOracle.PlanFitR.attempts ((__node).state)), firings := (((__r).ledger).firings + (LexLeanRuntime.length (__successors) : Nat)), expansions := (((__r).ledger).expansions + 1), verifications := ((__r).ledger).verifications, frontier := (if (Nat.blt (((__r).ledger).frontier) ((LexLeanRuntime.length (__next) : Nat))) then (LexLeanRuntime.length (__next) : Nat) else ((__r).ledger).frontier) } : LexLeanTarget.PlannerOracle.PlanFitR.Ledger) } : LexLeanTarget.PlannerOracle.PlanFitR.Search)))))))))
@[expose] public def PlanFitR.run (target : Nat) : (Prod (LexLeanTarget.PlannerOracle.PlanFitR.Search) (Bool)) := (LexLeanCollections.iterateUntil ((fun (__r : LexLeanTarget.PlannerOracle.PlanFitR.Search) => LexLeanTarget.PlannerOracle.PlanFitR.searchStep (target) (__r))) ((LexLeanRuntime.multiply (target) (8) : Nat)) (LexLeanTarget.PlannerOracle.PlanFitR.start (target)) : (Prod (LexLeanTarget.PlannerOracle.PlanFitR.Search) (Bool)))
@[expose] public def PlanFitR.failure (__saturated : Bool) (__truncated : Bool) : (Prod Bool Bool) := (if (__saturated && (!__truncated)) then ((false, true) : Prod Bool Bool) else ((false, false) : Prod Bool Bool))
@[expose] public def PlanFitR (target : Nat) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (LexLeanTarget.PlannerOracle.PlanFitR.Step)))) := (let __final : (Prod (LexLeanTarget.PlannerOracle.PlanFitR.Search) (Bool)) := LexLeanTarget.PlannerOracle.PlanFitR.run (target); (match ((__final).1).found with | Option.some __hit => Except.ok (((__hit).1, ((__hit).2).trace)) | Option.none => Except.error (LexLeanTarget.PlannerOracle.PlanFitR.failure ((__final).2) (((__final).1).truncated))))
@[expose] public def PlanFitR.verdict (target : Nat) : Except ((Prod Bool Bool)) (Nat) := (match LexLeanTarget.PlannerOracle.PlanFitR (target) with | Except.ok __p => Except.ok ((__p).1) | Except.error __e => Except.error (__e))
public theorem PlanFitR.fire_sound (__s : (Prod (Nat) (Nat))) (__step : LexLeanTarget.PlannerOracle.PlanFitR.Step) (__t : (Prod (Nat) (Nat))) : ((LexLeanTarget.PlannerOracle.PlanFitR.fire (__s) (__step) = Option.some (__t)) -> LexLeanTarget.PlannerOracle.Moves (__s) (__t)) :=
by
  cases __step with
  | FillA =>
    exact LexLeanTarget.PlannerOracle.FillA.apply_sound __s __t
  | EmptyB =>
    exact LexLeanTarget.PlannerOracle.EmptyB.apply_sound __s __t
  | Pour __b =>
    exact LexLeanTarget.PlannerOracle.Pour.apply_sound __s __b __t
public theorem PlanFitR.replay_fire (__s : (Prod (Nat) (Nat))) (__step : LexLeanTarget.PlannerOracle.PlanFitR.Step) (__t : (Prod (Nat) (Nat))) : ((LexLeanTarget.PlannerOracle.PlanFitR.fire (__s) (__step) = Option.some (__t)) -> (LexLeanTarget.PlannerOracle.PlanFitR.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitR.replay]
  rw [llE]
public theorem PlanFitR.replay_sound (target : Nat) (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : LexLeanTarget.PlannerOracle.PlanFitR.Step) : ((LexLeanReasoning.Reaches ((LexLeanTarget.PlannerOracle.Moves)) (LexLeanTarget.PlannerOracle.PlanFitR.observe (target)) (__acc)) -> (LexLeanReasoning.Reaches ((LexLeanTarget.PlannerOracle.Moves)) (LexLeanTarget.PlannerOracle.PlanFitR.observe (target)) (LexLeanTarget.PlannerOracle.PlanFitR.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [LexLeanTarget.PlannerOracle.PlanFitR.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (LexLeanTarget.PlannerOracle.PlanFitR.fire_sound llS __step _ ‹_›)
public theorem PlanFitR.derivation (target : Nat) (__trace : List (LexLeanTarget.PlannerOracle.PlanFitR.Step)) : (LexLeanReasoning.Reaches ((LexLeanTarget.PlannerOracle.Moves)) (LexLeanTarget.PlannerOracle.PlanFitR.observe (target)) (LexLeanTarget.PlannerOracle.PlanFitR.follow (target) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((LexLeanTarget.PlannerOracle.PlanFitR.replay)) (fun (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) => (LexLeanReasoning.Reaches ((LexLeanTarget.PlannerOracle.Moves)) (LexLeanTarget.PlannerOracle.PlanFitR.observe (target)) (__acc))) (LexLeanTarget.PlannerOracle.PlanFitR.replay_sound (target)) (__trace) (Except.ok (LexLeanTarget.PlannerOracle.PlanFitR.observe (target))) (LexLeanReasoning.reachesStart ((LexLeanTarget.PlannerOracle.Moves)) (LexLeanTarget.PlannerOracle.PlanFitR.observe (target))))
public theorem PlanFitR.follow_invariant (target : Nat) (__trace : List (LexLeanTarget.PlannerOracle.PlanFitR.Step)) (__s : (Prod (Nat) (Nat))) : (LexLeanTarget.PlannerOracle.Fits (LexLeanTarget.PlannerOracle.PlanFitR.observe (target)) -> ((LexLeanTarget.PlannerOracle.PlanFitR.follow (target) (__trace) = Except.ok (__s)) -> LexLeanTarget.PlannerOracle.Fits (__s))) :=
  (fun llI llE => (LexLeanReasoning.starPreserves ((LexLeanTarget.PlannerOracle.Moves)) ((LexLeanTarget.PlannerOracle.Fits)) LexLeanTarget.PlannerOracle.Pouring.preserves (LexLeanTarget.PlannerOracle.PlanFitR.observe (target)) (__s) (LexLeanTarget.PlannerOracle.PlanFitR.derivation (target) (__trace) (__s) llE) llI))
public theorem PlanFitR.follow_relation (target : Nat) (__trace : List (LexLeanTarget.PlannerOracle.PlanFitR.Step)) (__s : (Prod (Nat) (Nat))) : ((LexLeanTarget.PlannerOracle.PlanFitR.follow (target) (__trace) = Except.ok (__s)) -> LexLeanTarget.PlannerOracle.Held (target) (__s)) :=
  (fun llE => (LexLeanReasoning.starPreserves ((LexLeanTarget.PlannerOracle.Moves)) (fun (__s : (Prod (Nat) (Nat))) => LexLeanTarget.PlannerOracle.Held (target) (__s)) (LexLeanTarget.PlannerOracle.held_preserved (target)) (LexLeanTarget.PlannerOracle.PlanFitR.observe (target)) (__s) (LexLeanTarget.PlannerOracle.PlanFitR.derivation (target) (__trace) (__s) llE) (LexLeanTarget.PlannerOracle.held_initial (target))))
public theorem PlanFitR.accept_sound (target : Nat) (__s : (Prod (Nat) (Nat))) (__v : Nat) : (LexLeanTarget.PlannerOracle.Held (target) (__s) -> ((LexLeanTarget.PlannerOracle.PlanFitR.accept (target) (__s) = Option.some (__v)) -> LexLeanTarget.PlannerOracle.Within (target) (__v))) :=
  (fun llJ llE => (LexLeanTarget.PlannerOracle.plan_held_correct (target) (__s) (__v) llJ llE))
public theorem PlanFitR.extend (target : Nat) (__node : LexLeanTarget.PlannerOracle.PlanFitR.Node) (__step : LexLeanTarget.PlannerOracle.PlanFitR.Step) (__t : (Prod (Nat) (Nat))) : ((LexLeanTarget.PlannerOracle.PlanFitR.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> ((LexLeanTarget.PlannerOracle.PlanFitR.fire ((__node).state) (__step) = Option.some (__t)) -> (LexLeanTarget.PlannerOracle.PlanFitR.follow (target) ((LexLeanRuntime.append ((__node).trace) ((__step :: ([] : List (LexLeanTarget.PlannerOracle.PlanFitR.Step)))) : List (LexLeanTarget.PlannerOracle.PlanFitR.Step))) = Except.ok (__t)))) :=
by
  intro llH llE
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitR.follow]
  rw [LexLeanReasoning.foldSnoc]
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitR.follow] at llH
  rw [llH]
  exact LexLeanTarget.PlannerOracle.PlanFitR.replay_fire _ __step __t llE
public theorem PlanFitR.FillA.successors_ok (target : Nat) (__node : LexLeanTarget.PlannerOracle.PlanFitR.Node) : ((LexLeanTarget.PlannerOracle.PlanFitR.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitR.Node) => (LexLeanTarget.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (LexLeanTarget.PlannerOracle.PlanFitR.FillA.successors (__node)))) :=
by
  intro llH
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitR.FillA.successors]
  split
  · exact LexLeanReasoning.All.nil
  · exact LexLeanReasoning.allSingle _ _ (LexLeanTarget.PlannerOracle.PlanFitR.extend _ __node _ _ llH ‹_›)
public theorem PlanFitR.EmptyB.successors_ok (target : Nat) (__node : LexLeanTarget.PlannerOracle.PlanFitR.Node) : ((LexLeanTarget.PlannerOracle.PlanFitR.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitR.Node) => (LexLeanTarget.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (LexLeanTarget.PlannerOracle.PlanFitR.EmptyB.successors (__node)))) :=
by
  intro llH
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitR.EmptyB.successors]
  split
  · exact LexLeanReasoning.All.nil
  · exact LexLeanReasoning.allSingle _ _ (LexLeanTarget.PlannerOracle.PlanFitR.extend _ __node _ _ llH ‹_›)
public theorem PlanFitR.Pour.successors_ok (target : Nat) (__node : LexLeanTarget.PlannerOracle.PlanFitR.Node) : ((LexLeanTarget.PlannerOracle.PlanFitR.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitR.Node) => (LexLeanTarget.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (LexLeanTarget.PlannerOracle.PlanFitR.Pour.successors (__node)))) :=
by
  intro llH
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitR.Pour.successors]
  exact LexLeanReasoning.foldInvariant _ (LexLeanReasoning.All (fun (llN : LexLeanTarget.PlannerOracle.PlanFitR.Node) => LexLeanTarget.PlannerOracle.PlanFitR.follow _ llN.trace = Except.ok llN.state))
    (fun llA llB llP => by
      dsimp only [LexLeanTarget.PlannerOracle.PlanFitR.Pour.collect]
      split
      · exact llP
      · exact LexLeanReasoning.allAppend _ _ _ llP (LexLeanReasoning.allSingle _ _ (LexLeanTarget.PlannerOracle.PlanFitR.extend _ __node _ _ llH ‹_›)))
    _ _ LexLeanReasoning.All.nil
public theorem PlanFitR.successors_ok (target : Nat) (__node : LexLeanTarget.PlannerOracle.PlanFitR.Node) : ((LexLeanTarget.PlannerOracle.PlanFitR.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitR.Node) => (LexLeanTarget.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (LexLeanTarget.PlannerOracle.PlanFitR.successors (__node)))) :=
  (fun llH => (LexLeanReasoning.allAppend _ _ _ (LexLeanTarget.PlannerOracle.PlanFitR.FillA.successors_ok (target) (__node) llH) (LexLeanReasoning.allAppend _ _ _ (LexLeanTarget.PlannerOracle.PlanFitR.EmptyB.successors_ok (target) (__node) llH) (LexLeanTarget.PlannerOracle.PlanFitR.Pour.successors_ok (target) (__node) llH))))
public theorem PlanFitR.fresh_ok (target : Nat) (__visited : List ((Prod (Nat) (Nat)))) (__nodes : List (LexLeanTarget.PlannerOracle.PlanFitR.Node)) : ((LexLeanReasoning.All ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitR.Node) => (LexLeanTarget.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (__nodes)) -> (LexLeanReasoning.All ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitR.Node) => (LexLeanTarget.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((LexLeanTarget.PlannerOracle.PlanFitR.fresh (__visited) (__nodes)).1))) :=
  (LexLeanReasoning.freshAll ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitR.Node) => (__n).state)) (fun (__n : LexLeanTarget.PlannerOracle.PlanFitR.Node) => (LexLeanTarget.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state))) (__visited) (__nodes))
public theorem PlanFitR.search_step (target : Nat) (__r : LexLeanTarget.PlannerOracle.PlanFitR.Search) (__q : LexLeanTarget.PlannerOracle.PlanFitR.Search) : ((LexLeanTarget.PlannerOracle.PlanFitR.searchStep (target) (__r) = Option.some (__q)) -> ((LexLeanReasoning.SearchOk ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitR.Node) => (LexLeanTarget.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitR.Node) => LexLeanTarget.PlannerOracle.PlanFitR.accept (target) ((__n).state))) ((__r).frontier) ((__r).found)) -> (LexLeanReasoning.SearchOk ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitR.Node) => (LexLeanTarget.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitR.Node) => LexLeanTarget.PlannerOracle.PlanFitR.accept (target) ((__n).state))) ((__q).frontier) ((__q).found)))) :=
by
  intro llE llH
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitR.searchStep] at llE
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
        exact And.intro (LexLeanReasoning.capAll _ _ _ (LexLeanReasoning.allAppend _ _ _ llRestOk (LexLeanTarget.PlannerOracle.PlanFitR.fresh_ok _ _ _ (LexLeanTarget.PlannerOracle.PlanFitR.successors_ok _ llNode llNodeOk)))) True.intro
public theorem PlanFitR.search_peak (target : Nat) (__r : LexLeanTarget.PlannerOracle.PlanFitR.Search) (__q : LexLeanTarget.PlannerOracle.PlanFitR.Search) : ((LexLeanTarget.PlannerOracle.PlanFitR.searchStep (target) (__r) = Option.some (__q)) -> ((((__r).ledger).frontier <= (target + target)) -> (((__q).ledger).frontier <= (target + target)))) :=
by
  intro llE llH
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitR.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        exact llH
      · cases llE
        exact LexLeanReasoning.peakBound _ _ _ llH (LexLeanReasoning.capBound _ _)
public theorem PlanFitR.search_count (target : Nat) (__r : LexLeanTarget.PlannerOracle.PlanFitR.Search) (__q : LexLeanTarget.PlannerOracle.PlanFitR.Search) : ((LexLeanTarget.PlannerOracle.PlanFitR.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitR.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        rfl
      · cases llE
        rfl
public theorem PlanFitR.search_verify_growth (target : Nat) (__r : LexLeanTarget.PlannerOracle.PlanFitR.Search) (__q : LexLeanTarget.PlannerOracle.PlanFitR.Search) : ((LexLeanTarget.PlannerOracle.PlanFitR.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).verifications <= (((__r).ledger).verifications + 1))) :=
by
  intro llE
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitR.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem PlanFitR.search_expanded (target : Nat) (__r : LexLeanTarget.PlannerOracle.PlanFitR.Search) (__q : LexLeanTarget.PlannerOracle.PlanFitR.Search) : ((LexLeanTarget.PlannerOracle.PlanFitR.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).expansions <= (((__r).ledger).expansions + 1))) :=
by
  intro llE
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitR.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem PlanFitR.search_ok (target : Nat) : (LexLeanReasoning.SearchOk ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitR.Node) => (LexLeanTarget.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitR.Node) => LexLeanTarget.PlannerOracle.PlanFitR.accept (target) ((__n).state))) (((LexLeanTarget.PlannerOracle.PlanFitR.run (target)).1).frontier) (((LexLeanTarget.PlannerOracle.PlanFitR.run (target)).1).found)) :=
by
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitR.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((fun (__r : LexLeanTarget.PlannerOracle.PlanFitR.Search) => LexLeanTarget.PlannerOracle.PlanFitR.searchStep (target) (__r))) (fun (__r : LexLeanTarget.PlannerOracle.PlanFitR.Search) => (LexLeanReasoning.SearchOk ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitR.Node) => (LexLeanTarget.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitR.Node) => LexLeanTarget.PlannerOracle.PlanFitR.accept (target) ((__n).state))) ((__r).frontier) ((__r).found))) (LexLeanTarget.PlannerOracle.PlanFitR.search_step (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (LexLeanTarget.PlannerOracle.PlanFitR.start (target)) (LexLeanReasoning.searchStart ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitR.Node) => (LexLeanTarget.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitR.Node) => LexLeanTarget.PlannerOracle.PlanFitR.accept (target) ((__n).state))) _ (LexLeanReasoning.capAll ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitR.Node) => (LexLeanTarget.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((target + target)) _ (LexLeanReasoning.allSingle ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitR.Node) => (LexLeanTarget.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (({ state := LexLeanTarget.PlannerOracle.PlanFitR.observe (target), trace := ([] : List (LexLeanTarget.PlannerOracle.PlanFitR.Step)) } : LexLeanTarget.PlannerOracle.PlanFitR.Node)) rfl))))
public theorem PlanFitR.frontier_bounded (target : Nat) : ((((LexLeanTarget.PlannerOracle.PlanFitR.run (target)).1).ledger).frontier <= (target + target)) :=
by
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitR.run]
  exact (LexLeanReasoning.iterateUntilBound ((fun (__r : LexLeanTarget.PlannerOracle.PlanFitR.Search) => LexLeanTarget.PlannerOracle.PlanFitR.searchStep (target) (__r))) ((fun (__r : LexLeanTarget.PlannerOracle.PlanFitR.Search) => ((__r).ledger).frontier)) ((target + target)) (LexLeanTarget.PlannerOracle.PlanFitR.search_peak (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (LexLeanTarget.PlannerOracle.PlanFitR.start (target)) (LexLeanReasoning.capBound ((target + target)) _))
public theorem PlanFitR.verifications_bounded (target : Nat) : ((((LexLeanTarget.PlannerOracle.PlanFitR.run (target)).1).ledger).verifications <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitR.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : LexLeanTarget.PlannerOracle.PlanFitR.Search) => LexLeanTarget.PlannerOracle.PlanFitR.searchStep (target) (__r))) ((fun (__r : LexLeanTarget.PlannerOracle.PlanFitR.Search) => ((__r).ledger).verifications)) (LexLeanTarget.PlannerOracle.PlanFitR.search_verify_growth (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (LexLeanTarget.PlannerOracle.PlanFitR.start (target)) rfl)
public theorem PlanFitR.expansions_bounded (target : Nat) : ((((LexLeanTarget.PlannerOracle.PlanFitR.run (target)).1).ledger).expansions <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitR.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : LexLeanTarget.PlannerOracle.PlanFitR.Search) => LexLeanTarget.PlannerOracle.PlanFitR.searchStep (target) (__r))) ((fun (__r : LexLeanTarget.PlannerOracle.PlanFitR.Search) => ((__r).ledger).expansions)) (LexLeanTarget.PlannerOracle.PlanFitR.search_expanded (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (LexLeanTarget.PlannerOracle.PlanFitR.start (target)) rfl)
public theorem PlanFitR.iterations_bounded (target : Nat) : ((((LexLeanTarget.PlannerOracle.PlanFitR.run (target)).1).ledger).iterations <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitR.run]
  exact (LexLeanReasoning.iterateUntilCount ((fun (__r : LexLeanTarget.PlannerOracle.PlanFitR.Search) => LexLeanTarget.PlannerOracle.PlanFitR.searchStep (target) (__r))) ((fun (__r : LexLeanTarget.PlannerOracle.PlanFitR.Search) => ((__r).ledger).iterations)) (LexLeanTarget.PlannerOracle.PlanFitR.search_count (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (LexLeanTarget.PlannerOracle.PlanFitR.start (target)) rfl)
public theorem PlanFitR.explained (target : Nat) : (forall (__v : Nat), (forall (__trace : List (LexLeanTarget.PlannerOracle.PlanFitR.Step)), ((LexLeanTarget.PlannerOracle.PlanFitR (target) = Except.ok ((__v, __trace))) -> ((LexLeanTarget.PlannerOracle.PlanFitR.answer (target) (__trace) = Option.some (__v)) /\ LexLeanTarget.PlannerOracle.Within (target) (__v))))) :=
by
  intro llV llT llE
  have llSearch := LexLeanTarget.PlannerOracle.PlanFitR.search_ok target
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitR] at llE
  generalize llRun : LexLeanTarget.PlannerOracle.PlanFitR.run target = llR at llE llSearch
  split at llE
  · rename_i llHit llF
    cases llE
    have llOk := And.right llSearch
    rw [llF] at llOk
    exact And.intro (by dsimp only [LexLeanTarget.PlannerOracle.PlanFitR.answer]; rw [And.left llOk]; exact And.right llOk) (LexLeanTarget.PlannerOracle.PlanFitR.accept_sound _ _ _ (LexLeanTarget.PlannerOracle.PlanFitR.follow_relation target _ _ (And.left llOk)) (And.right llOk))
  · cases llE
public theorem PlanFitR.verdict_sound (target : Nat) : (forall (__v : Nat), ((LexLeanTarget.PlannerOracle.PlanFitR.verdict (target) = Except.ok (__v)) -> LexLeanTarget.PlannerOracle.Within (target) (__v))) :=
by
  intro llV llE
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitR.verdict] at llE
  generalize llHP : LexLeanTarget.PlannerOracle.PlanFitR target = llR at llE
  cases llR with
  | error _ => cases llE
  | ok llP =>
    cases llE
    exact And.right (LexLeanTarget.PlannerOracle.PlanFitR.explained _ llP.1 llP.2 llHP)

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
@[expose] public def PlanFitU.fire (__s : (Prod (Nat) (Nat))) (__step : LexLeanTarget.PlannerOracle.PlanFitU.Step) : Option ((Prod (Nat) (Nat))) := (match __step with | LexLeanTarget.PlannerOracle.PlanFitU.Step.FillA => LexLeanTarget.PlannerOracle.FillA.apply (__s) | LexLeanTarget.PlannerOracle.PlanFitU.Step.EmptyB => LexLeanTarget.PlannerOracle.EmptyB.apply (__s) | LexLeanTarget.PlannerOracle.PlanFitU.Step.Pour __b => LexLeanTarget.PlannerOracle.Pour.apply (__s) (__b))
@[expose] public def PlanFitU.replay (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : LexLeanTarget.PlannerOracle.PlanFitU.Step) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match LexLeanTarget.PlannerOracle.PlanFitU.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def PlanFitU.follow (target : Nat) (__trace : List (LexLeanTarget.PlannerOracle.PlanFitU.Step)) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (LexLeanCollections.listFold ((LexLeanTarget.PlannerOracle.PlanFitU.replay)) (Except.ok (LexLeanTarget.PlannerOracle.PlanFitU.observe (target))) (__trace) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))))
@[expose] public def PlanFitU.extract (jugs : (Prod (Nat) (Nat))) : Option (Nat) := Option.some ((jugs).1)
@[expose] public def PlanFitU.accept (_target : Nat) (__s : (Prod (Nat) (Nat))) : Option (Nat) := LexLeanTarget.PlannerOracle.PlanFitU.extract (__s)
@[expose] public def PlanFitU.answer (target : Nat) (__trace : List (LexLeanTarget.PlannerOracle.PlanFitU.Step)) : Option (Nat) := (match LexLeanTarget.PlannerOracle.PlanFitU.follow (target) (__trace) with | Except.ok __s => LexLeanTarget.PlannerOracle.PlanFitU.accept (target) (__s) | Except.error _ => Option.none)
public structure PlanFitU.Node where
  state : (Prod (Nat) (Nat))
  trace : List (LexLeanTarget.PlannerOracle.PlanFitU.Step)
public structure PlanFitU.Search where
  frontier : List (LexLeanTarget.PlannerOracle.PlanFitU.Node)
  visited : List ((Prod (Nat) (Nat)))
  found : Option ((Prod (Nat) (LexLeanTarget.PlannerOracle.PlanFitU.Node)))
  truncated : Bool
  ledger : LexLeanTarget.PlannerOracle.PlanFitU.Ledger
@[expose] public def PlanFitU.FillA.successors (__node : LexLeanTarget.PlannerOracle.PlanFitU.Node) : List (LexLeanTarget.PlannerOracle.PlanFitU.Node) := (match LexLeanTarget.PlannerOracle.PlanFitU.fire ((__node).state) (LexLeanTarget.PlannerOracle.PlanFitU.Step.FillA) with | Option.none => ([] : List (LexLeanTarget.PlannerOracle.PlanFitU.Node)) | Option.some __t => (({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((LexLeanTarget.PlannerOracle.PlanFitU.Step.FillA :: ([] : List (LexLeanTarget.PlannerOracle.PlanFitU.Step)))) : List (LexLeanTarget.PlannerOracle.PlanFitU.Step)) } : LexLeanTarget.PlannerOracle.PlanFitU.Node) :: ([] : List (LexLeanTarget.PlannerOracle.PlanFitU.Node))))
@[expose] public def PlanFitU.EmptyB.successors (__node : LexLeanTarget.PlannerOracle.PlanFitU.Node) : List (LexLeanTarget.PlannerOracle.PlanFitU.Node) := (match LexLeanTarget.PlannerOracle.PlanFitU.fire ((__node).state) (LexLeanTarget.PlannerOracle.PlanFitU.Step.EmptyB) with | Option.none => ([] : List (LexLeanTarget.PlannerOracle.PlanFitU.Node)) | Option.some __t => (({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((LexLeanTarget.PlannerOracle.PlanFitU.Step.EmptyB :: ([] : List (LexLeanTarget.PlannerOracle.PlanFitU.Step)))) : List (LexLeanTarget.PlannerOracle.PlanFitU.Step)) } : LexLeanTarget.PlannerOracle.PlanFitU.Node) :: ([] : List (LexLeanTarget.PlannerOracle.PlanFitU.Node))))
@[expose] public def PlanFitU.Pour.collect (__node : LexLeanTarget.PlannerOracle.PlanFitU.Node) (__acc : List (LexLeanTarget.PlannerOracle.PlanFitU.Node)) (__b : Nat) : List (LexLeanTarget.PlannerOracle.PlanFitU.Node) := (match LexLeanTarget.PlannerOracle.PlanFitU.fire ((__node).state) (LexLeanTarget.PlannerOracle.PlanFitU.Step.Pour (__b)) with | Option.none => __acc | Option.some __t => (LexLeanRuntime.append (__acc) ((({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((LexLeanTarget.PlannerOracle.PlanFitU.Step.Pour (__b) :: ([] : List (LexLeanTarget.PlannerOracle.PlanFitU.Step)))) : List (LexLeanTarget.PlannerOracle.PlanFitU.Step)) } : LexLeanTarget.PlannerOracle.PlanFitU.Node) :: ([] : List (LexLeanTarget.PlannerOracle.PlanFitU.Node)))) : List (LexLeanTarget.PlannerOracle.PlanFitU.Node)))
@[expose] public def PlanFitU.Pour.successors (__node : LexLeanTarget.PlannerOracle.PlanFitU.Node) : List (LexLeanTarget.PlannerOracle.PlanFitU.Node) := (LexLeanCollections.listFold ((fun (__acc : List (LexLeanTarget.PlannerOracle.PlanFitU.Node)) (__b : Nat) => LexLeanTarget.PlannerOracle.PlanFitU.Pour.collect (__node) (__acc) (__b))) (([] : List (LexLeanTarget.PlannerOracle.PlanFitU.Node))) (LexLeanTarget.PlannerOracle.Pour.candidates ((__node).state)) : List (LexLeanTarget.PlannerOracle.PlanFitU.Node))
@[expose] public def PlanFitU.successors (__node : LexLeanTarget.PlannerOracle.PlanFitU.Node) : List (LexLeanTarget.PlannerOracle.PlanFitU.Node) := (LexLeanRuntime.append (LexLeanTarget.PlannerOracle.PlanFitU.FillA.successors (__node)) ((LexLeanRuntime.append (LexLeanTarget.PlannerOracle.PlanFitU.EmptyB.successors (__node)) (LexLeanTarget.PlannerOracle.PlanFitU.Pour.successors (__node)) : List (LexLeanTarget.PlannerOracle.PlanFitU.Node))) : List (LexLeanTarget.PlannerOracle.PlanFitU.Node))
@[expose] public def PlanFitU.fresh (__visited : List ((Prod (Nat) (Nat)))) (__nodes : List (LexLeanTarget.PlannerOracle.PlanFitU.Node)) : (Prod (List (LexLeanTarget.PlannerOracle.PlanFitU.Node)) (List ((Prod (Nat) (Nat))))) := (LexLeanCollections.listFold ((fun (__acc : (Prod (List (LexLeanTarget.PlannerOracle.PlanFitU.Node)) (List ((Prod (Nat) (Nat)))))) (__n : LexLeanTarget.PlannerOracle.PlanFitU.Node) => (if (LexLeanCollections.setContains ((__acc).2) ((__n).state) : Bool) then __acc else ((LexLeanRuntime.append ((__acc).1) ((__n :: ([] : List (LexLeanTarget.PlannerOracle.PlanFitU.Node)))) : List (LexLeanTarget.PlannerOracle.PlanFitU.Node)), (LexLeanCollections.setInsert ((__acc).2) ((__n).state) : List ((Prod (Nat) (Nat)))))))) ((([] : List (LexLeanTarget.PlannerOracle.PlanFitU.Node)), __visited)) (__nodes) : (Prod (List (LexLeanTarget.PlannerOracle.PlanFitU.Node)) (List ((Prod (Nat) (Nat))))))
@[expose] public def PlanFitU.start (target : Nat) : LexLeanTarget.PlannerOracle.PlanFitU.Search := (let __first : List (LexLeanTarget.PlannerOracle.PlanFitU.Node) := (LexLeanCollections.listFold ((fun (__acc : List (LexLeanTarget.PlannerOracle.PlanFitU.Node)) (__n : LexLeanTarget.PlannerOracle.PlanFitU.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) ((target + target))) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (LexLeanTarget.PlannerOracle.PlanFitU.Node)))) : List (LexLeanTarget.PlannerOracle.PlanFitU.Node)) else __acc))) (([] : List (LexLeanTarget.PlannerOracle.PlanFitU.Node))) ((({ state := LexLeanTarget.PlannerOracle.PlanFitU.observe (target), trace := ([] : List (LexLeanTarget.PlannerOracle.PlanFitU.Step)) } : LexLeanTarget.PlannerOracle.PlanFitU.Node) :: ([] : List (LexLeanTarget.PlannerOracle.PlanFitU.Node)))) : List (LexLeanTarget.PlannerOracle.PlanFitU.Node)); ({ frontier := __first, visited := (LexLeanCollections.setInsert (([] : List ((Prod (Nat) (Nat))))) (LexLeanTarget.PlannerOracle.PlanFitU.observe (target)) : List ((Prod (Nat) (Nat)))), found := Option.none, truncated := (Nat.blt ((target + target)) (1)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := (LexLeanRuntime.length (__first) : Nat) } : LexLeanTarget.PlannerOracle.PlanFitU.Ledger) } : LexLeanTarget.PlannerOracle.PlanFitU.Search))
@[expose] public def PlanFitU.attempts (__s : (Prod (Nat) (Nat))) : Nat := (1 + (1 + (LexLeanRuntime.length (LexLeanTarget.PlannerOracle.Pour.candidates (__s)) : Nat)))
@[expose] public def PlanFitU.searchStep (target : Nat) (__r : LexLeanTarget.PlannerOracle.PlanFitU.Search) : Option (LexLeanTarget.PlannerOracle.PlanFitU.Search) := (match (__r).found with | Option.some _ => Option.none | Option.none => (match (__r).frontier with | List.nil => Option.none | List.cons __node __rest => (match LexLeanTarget.PlannerOracle.PlanFitU.accept (target) ((__node).state) with | Option.some __v => Option.some (({ frontier := __rest, visited := (__r).visited, found := Option.some ((__v, __node)), truncated := (__r).truncated, ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := ((__r).ledger).attempts, firings := ((__r).ledger).firings, expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : LexLeanTarget.PlannerOracle.PlanFitU.Ledger) } : LexLeanTarget.PlannerOracle.PlanFitU.Search)) | Option.none => (let __successors : List (LexLeanTarget.PlannerOracle.PlanFitU.Node) := LexLeanTarget.PlannerOracle.PlanFitU.successors (__node); (let __fresh : (Prod (List (LexLeanTarget.PlannerOracle.PlanFitU.Node)) (List ((Prod (Nat) (Nat))))) := LexLeanTarget.PlannerOracle.PlanFitU.fresh ((__r).visited) (__successors); (let __ordered : List (LexLeanTarget.PlannerOracle.PlanFitU.Node) := (LexLeanRuntime.append (__rest) ((__fresh).1) : List (LexLeanTarget.PlannerOracle.PlanFitU.Node)); (let __next : List (LexLeanTarget.PlannerOracle.PlanFitU.Node) := (LexLeanCollections.listFold ((fun (__acc : List (LexLeanTarget.PlannerOracle.PlanFitU.Node)) (__n : LexLeanTarget.PlannerOracle.PlanFitU.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) ((target + target))) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (LexLeanTarget.PlannerOracle.PlanFitU.Node)))) : List (LexLeanTarget.PlannerOracle.PlanFitU.Node)) else __acc))) (([] : List (LexLeanTarget.PlannerOracle.PlanFitU.Node))) (__ordered) : List (LexLeanTarget.PlannerOracle.PlanFitU.Node)); Option.some (({ frontier := __next, visited := (__fresh).2, found := Option.none, truncated := ((__r).truncated || (Nat.blt ((target + target)) ((LexLeanRuntime.length (__ordered) : Nat)))), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + LexLeanTarget.PlannerOracle.PlanFitU.attempts ((__node).state)), firings := (((__r).ledger).firings + (LexLeanRuntime.length (__successors) : Nat)), expansions := (((__r).ledger).expansions + 1), verifications := ((__r).ledger).verifications, frontier := (if (Nat.blt (((__r).ledger).frontier) ((LexLeanRuntime.length (__next) : Nat))) then (LexLeanRuntime.length (__next) : Nat) else ((__r).ledger).frontier) } : LexLeanTarget.PlannerOracle.PlanFitU.Ledger) } : LexLeanTarget.PlannerOracle.PlanFitU.Search)))))))))
@[expose] public def PlanFitU.run (target : Nat) : (Prod (LexLeanTarget.PlannerOracle.PlanFitU.Search) (Bool)) := (LexLeanCollections.iterateUntil ((fun (__r : LexLeanTarget.PlannerOracle.PlanFitU.Search) => LexLeanTarget.PlannerOracle.PlanFitU.searchStep (target) (__r))) ((LexLeanRuntime.multiply (target) (8) : Nat)) (LexLeanTarget.PlannerOracle.PlanFitU.start (target)) : (Prod (LexLeanTarget.PlannerOracle.PlanFitU.Search) (Bool)))
@[expose] public def PlanFitU.failure (__saturated : Bool) (__truncated : Bool) : (Prod Bool Bool) := (if (__saturated && (!__truncated)) then ((false, true) : Prod Bool Bool) else ((false, false) : Prod Bool Bool))
@[expose] public def PlanFitU (target : Nat) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (LexLeanTarget.PlannerOracle.PlanFitU.Step)))) := (let __final : (Prod (LexLeanTarget.PlannerOracle.PlanFitU.Search) (Bool)) := LexLeanTarget.PlannerOracle.PlanFitU.run (target); (match ((__final).1).found with | Option.some __hit => Except.ok (((__hit).1, ((__hit).2).trace)) | Option.none => Except.error (LexLeanTarget.PlannerOracle.PlanFitU.failure ((__final).2) (((__final).1).truncated))))
@[expose] public def PlanFitU.verdict (target : Nat) : Except ((Prod Bool Bool)) (Nat) := (match LexLeanTarget.PlannerOracle.PlanFitU (target) with | Except.ok __p => Except.ok ((__p).1) | Except.error __e => Except.error (__e))
public theorem PlanFitU.fire_sound (__s : (Prod (Nat) (Nat))) (__step : LexLeanTarget.PlannerOracle.PlanFitU.Step) (__t : (Prod (Nat) (Nat))) : ((LexLeanTarget.PlannerOracle.PlanFitU.fire (__s) (__step) = Option.some (__t)) -> LexLeanTarget.PlannerOracle.Moves (__s) (__t)) :=
by
  cases __step with
  | FillA =>
    exact LexLeanTarget.PlannerOracle.FillA.apply_sound __s __t
  | EmptyB =>
    exact LexLeanTarget.PlannerOracle.EmptyB.apply_sound __s __t
  | Pour __b =>
    exact LexLeanTarget.PlannerOracle.Pour.apply_sound __s __b __t
public theorem PlanFitU.replay_fire (__s : (Prod (Nat) (Nat))) (__step : LexLeanTarget.PlannerOracle.PlanFitU.Step) (__t : (Prod (Nat) (Nat))) : ((LexLeanTarget.PlannerOracle.PlanFitU.fire (__s) (__step) = Option.some (__t)) -> (LexLeanTarget.PlannerOracle.PlanFitU.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitU.replay]
  rw [llE]
public theorem PlanFitU.replay_sound (target : Nat) (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : LexLeanTarget.PlannerOracle.PlanFitU.Step) : ((LexLeanReasoning.Reaches ((LexLeanTarget.PlannerOracle.Moves)) (LexLeanTarget.PlannerOracle.PlanFitU.observe (target)) (__acc)) -> (LexLeanReasoning.Reaches ((LexLeanTarget.PlannerOracle.Moves)) (LexLeanTarget.PlannerOracle.PlanFitU.observe (target)) (LexLeanTarget.PlannerOracle.PlanFitU.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [LexLeanTarget.PlannerOracle.PlanFitU.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (LexLeanTarget.PlannerOracle.PlanFitU.fire_sound llS __step _ ‹_›)
public theorem PlanFitU.derivation (target : Nat) (__trace : List (LexLeanTarget.PlannerOracle.PlanFitU.Step)) : (LexLeanReasoning.Reaches ((LexLeanTarget.PlannerOracle.Moves)) (LexLeanTarget.PlannerOracle.PlanFitU.observe (target)) (LexLeanTarget.PlannerOracle.PlanFitU.follow (target) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((LexLeanTarget.PlannerOracle.PlanFitU.replay)) (fun (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) => (LexLeanReasoning.Reaches ((LexLeanTarget.PlannerOracle.Moves)) (LexLeanTarget.PlannerOracle.PlanFitU.observe (target)) (__acc))) (LexLeanTarget.PlannerOracle.PlanFitU.replay_sound (target)) (__trace) (Except.ok (LexLeanTarget.PlannerOracle.PlanFitU.observe (target))) (LexLeanReasoning.reachesStart ((LexLeanTarget.PlannerOracle.Moves)) (LexLeanTarget.PlannerOracle.PlanFitU.observe (target))))
public theorem PlanFitU.follow_invariant (target : Nat) (__trace : List (LexLeanTarget.PlannerOracle.PlanFitU.Step)) (__s : (Prod (Nat) (Nat))) : (LexLeanTarget.PlannerOracle.Fits (LexLeanTarget.PlannerOracle.PlanFitU.observe (target)) -> ((LexLeanTarget.PlannerOracle.PlanFitU.follow (target) (__trace) = Except.ok (__s)) -> LexLeanTarget.PlannerOracle.Fits (__s))) :=
  (fun llI llE => (LexLeanReasoning.starPreserves ((LexLeanTarget.PlannerOracle.Moves)) ((LexLeanTarget.PlannerOracle.Fits)) LexLeanTarget.PlannerOracle.Pouring.preserves (LexLeanTarget.PlannerOracle.PlanFitU.observe (target)) (__s) (LexLeanTarget.PlannerOracle.PlanFitU.derivation (target) (__trace) (__s) llE) llI))
public theorem PlanFitU.accept_sound (target : Nat) (__s : (Prod (Nat) (Nat))) (__v : Nat) : ((LexLeanTarget.PlannerOracle.PlanFitU.accept (target) (__s) = Option.some (__v)) -> LexLeanTarget.PlannerOracle.Whole (target) (__v)) :=
  (fun llE => (LexLeanTarget.PlannerOracle.plan_whole_correct (target) (__s) (__v) llE))
public theorem PlanFitU.extend (target : Nat) (__node : LexLeanTarget.PlannerOracle.PlanFitU.Node) (__step : LexLeanTarget.PlannerOracle.PlanFitU.Step) (__t : (Prod (Nat) (Nat))) : ((LexLeanTarget.PlannerOracle.PlanFitU.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> ((LexLeanTarget.PlannerOracle.PlanFitU.fire ((__node).state) (__step) = Option.some (__t)) -> (LexLeanTarget.PlannerOracle.PlanFitU.follow (target) ((LexLeanRuntime.append ((__node).trace) ((__step :: ([] : List (LexLeanTarget.PlannerOracle.PlanFitU.Step)))) : List (LexLeanTarget.PlannerOracle.PlanFitU.Step))) = Except.ok (__t)))) :=
by
  intro llH llE
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitU.follow]
  rw [LexLeanReasoning.foldSnoc]
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitU.follow] at llH
  rw [llH]
  exact LexLeanTarget.PlannerOracle.PlanFitU.replay_fire _ __step __t llE
public theorem PlanFitU.FillA.successors_ok (target : Nat) (__node : LexLeanTarget.PlannerOracle.PlanFitU.Node) : ((LexLeanTarget.PlannerOracle.PlanFitU.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitU.Node) => (LexLeanTarget.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (LexLeanTarget.PlannerOracle.PlanFitU.FillA.successors (__node)))) :=
by
  intro llH
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitU.FillA.successors]
  split
  · exact LexLeanReasoning.All.nil
  · exact LexLeanReasoning.allSingle _ _ (LexLeanTarget.PlannerOracle.PlanFitU.extend _ __node _ _ llH ‹_›)
public theorem PlanFitU.EmptyB.successors_ok (target : Nat) (__node : LexLeanTarget.PlannerOracle.PlanFitU.Node) : ((LexLeanTarget.PlannerOracle.PlanFitU.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitU.Node) => (LexLeanTarget.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (LexLeanTarget.PlannerOracle.PlanFitU.EmptyB.successors (__node)))) :=
by
  intro llH
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitU.EmptyB.successors]
  split
  · exact LexLeanReasoning.All.nil
  · exact LexLeanReasoning.allSingle _ _ (LexLeanTarget.PlannerOracle.PlanFitU.extend _ __node _ _ llH ‹_›)
public theorem PlanFitU.Pour.successors_ok (target : Nat) (__node : LexLeanTarget.PlannerOracle.PlanFitU.Node) : ((LexLeanTarget.PlannerOracle.PlanFitU.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitU.Node) => (LexLeanTarget.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (LexLeanTarget.PlannerOracle.PlanFitU.Pour.successors (__node)))) :=
by
  intro llH
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitU.Pour.successors]
  exact LexLeanReasoning.foldInvariant _ (LexLeanReasoning.All (fun (llN : LexLeanTarget.PlannerOracle.PlanFitU.Node) => LexLeanTarget.PlannerOracle.PlanFitU.follow _ llN.trace = Except.ok llN.state))
    (fun llA llB llP => by
      dsimp only [LexLeanTarget.PlannerOracle.PlanFitU.Pour.collect]
      split
      · exact llP
      · exact LexLeanReasoning.allAppend _ _ _ llP (LexLeanReasoning.allSingle _ _ (LexLeanTarget.PlannerOracle.PlanFitU.extend _ __node _ _ llH ‹_›)))
    _ _ LexLeanReasoning.All.nil
public theorem PlanFitU.successors_ok (target : Nat) (__node : LexLeanTarget.PlannerOracle.PlanFitU.Node) : ((LexLeanTarget.PlannerOracle.PlanFitU.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitU.Node) => (LexLeanTarget.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (LexLeanTarget.PlannerOracle.PlanFitU.successors (__node)))) :=
  (fun llH => (LexLeanReasoning.allAppend _ _ _ (LexLeanTarget.PlannerOracle.PlanFitU.FillA.successors_ok (target) (__node) llH) (LexLeanReasoning.allAppend _ _ _ (LexLeanTarget.PlannerOracle.PlanFitU.EmptyB.successors_ok (target) (__node) llH) (LexLeanTarget.PlannerOracle.PlanFitU.Pour.successors_ok (target) (__node) llH))))
public theorem PlanFitU.fresh_ok (target : Nat) (__visited : List ((Prod (Nat) (Nat)))) (__nodes : List (LexLeanTarget.PlannerOracle.PlanFitU.Node)) : ((LexLeanReasoning.All ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitU.Node) => (LexLeanTarget.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (__nodes)) -> (LexLeanReasoning.All ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitU.Node) => (LexLeanTarget.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((LexLeanTarget.PlannerOracle.PlanFitU.fresh (__visited) (__nodes)).1))) :=
  (LexLeanReasoning.freshAll ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitU.Node) => (__n).state)) (fun (__n : LexLeanTarget.PlannerOracle.PlanFitU.Node) => (LexLeanTarget.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state))) (__visited) (__nodes))
public theorem PlanFitU.search_step (target : Nat) (__r : LexLeanTarget.PlannerOracle.PlanFitU.Search) (__q : LexLeanTarget.PlannerOracle.PlanFitU.Search) : ((LexLeanTarget.PlannerOracle.PlanFitU.searchStep (target) (__r) = Option.some (__q)) -> ((LexLeanReasoning.SearchOk ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitU.Node) => (LexLeanTarget.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitU.Node) => LexLeanTarget.PlannerOracle.PlanFitU.accept (target) ((__n).state))) ((__r).frontier) ((__r).found)) -> (LexLeanReasoning.SearchOk ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitU.Node) => (LexLeanTarget.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitU.Node) => LexLeanTarget.PlannerOracle.PlanFitU.accept (target) ((__n).state))) ((__q).frontier) ((__q).found)))) :=
by
  intro llE llH
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitU.searchStep] at llE
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
        exact And.intro (LexLeanReasoning.capAll _ _ _ (LexLeanReasoning.allAppend _ _ _ llRestOk (LexLeanTarget.PlannerOracle.PlanFitU.fresh_ok _ _ _ (LexLeanTarget.PlannerOracle.PlanFitU.successors_ok _ llNode llNodeOk)))) True.intro
public theorem PlanFitU.search_peak (target : Nat) (__r : LexLeanTarget.PlannerOracle.PlanFitU.Search) (__q : LexLeanTarget.PlannerOracle.PlanFitU.Search) : ((LexLeanTarget.PlannerOracle.PlanFitU.searchStep (target) (__r) = Option.some (__q)) -> ((((__r).ledger).frontier <= (target + target)) -> (((__q).ledger).frontier <= (target + target)))) :=
by
  intro llE llH
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitU.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        exact llH
      · cases llE
        exact LexLeanReasoning.peakBound _ _ _ llH (LexLeanReasoning.capBound _ _)
public theorem PlanFitU.search_count (target : Nat) (__r : LexLeanTarget.PlannerOracle.PlanFitU.Search) (__q : LexLeanTarget.PlannerOracle.PlanFitU.Search) : ((LexLeanTarget.PlannerOracle.PlanFitU.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitU.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        rfl
      · cases llE
        rfl
public theorem PlanFitU.search_verify_growth (target : Nat) (__r : LexLeanTarget.PlannerOracle.PlanFitU.Search) (__q : LexLeanTarget.PlannerOracle.PlanFitU.Search) : ((LexLeanTarget.PlannerOracle.PlanFitU.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).verifications <= (((__r).ledger).verifications + 1))) :=
by
  intro llE
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitU.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem PlanFitU.search_expanded (target : Nat) (__r : LexLeanTarget.PlannerOracle.PlanFitU.Search) (__q : LexLeanTarget.PlannerOracle.PlanFitU.Search) : ((LexLeanTarget.PlannerOracle.PlanFitU.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).expansions <= (((__r).ledger).expansions + 1))) :=
by
  intro llE
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitU.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem PlanFitU.search_ok (target : Nat) : (LexLeanReasoning.SearchOk ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitU.Node) => (LexLeanTarget.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitU.Node) => LexLeanTarget.PlannerOracle.PlanFitU.accept (target) ((__n).state))) (((LexLeanTarget.PlannerOracle.PlanFitU.run (target)).1).frontier) (((LexLeanTarget.PlannerOracle.PlanFitU.run (target)).1).found)) :=
by
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitU.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((fun (__r : LexLeanTarget.PlannerOracle.PlanFitU.Search) => LexLeanTarget.PlannerOracle.PlanFitU.searchStep (target) (__r))) (fun (__r : LexLeanTarget.PlannerOracle.PlanFitU.Search) => (LexLeanReasoning.SearchOk ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitU.Node) => (LexLeanTarget.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitU.Node) => LexLeanTarget.PlannerOracle.PlanFitU.accept (target) ((__n).state))) ((__r).frontier) ((__r).found))) (LexLeanTarget.PlannerOracle.PlanFitU.search_step (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (LexLeanTarget.PlannerOracle.PlanFitU.start (target)) (LexLeanReasoning.searchStart ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitU.Node) => (LexLeanTarget.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitU.Node) => LexLeanTarget.PlannerOracle.PlanFitU.accept (target) ((__n).state))) _ (LexLeanReasoning.capAll ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitU.Node) => (LexLeanTarget.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((target + target)) _ (LexLeanReasoning.allSingle ((fun (__n : LexLeanTarget.PlannerOracle.PlanFitU.Node) => (LexLeanTarget.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (({ state := LexLeanTarget.PlannerOracle.PlanFitU.observe (target), trace := ([] : List (LexLeanTarget.PlannerOracle.PlanFitU.Step)) } : LexLeanTarget.PlannerOracle.PlanFitU.Node)) rfl))))
public theorem PlanFitU.frontier_bounded (target : Nat) : ((((LexLeanTarget.PlannerOracle.PlanFitU.run (target)).1).ledger).frontier <= (target + target)) :=
by
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitU.run]
  exact (LexLeanReasoning.iterateUntilBound ((fun (__r : LexLeanTarget.PlannerOracle.PlanFitU.Search) => LexLeanTarget.PlannerOracle.PlanFitU.searchStep (target) (__r))) ((fun (__r : LexLeanTarget.PlannerOracle.PlanFitU.Search) => ((__r).ledger).frontier)) ((target + target)) (LexLeanTarget.PlannerOracle.PlanFitU.search_peak (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (LexLeanTarget.PlannerOracle.PlanFitU.start (target)) (LexLeanReasoning.capBound ((target + target)) _))
public theorem PlanFitU.verifications_bounded (target : Nat) : ((((LexLeanTarget.PlannerOracle.PlanFitU.run (target)).1).ledger).verifications <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitU.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : LexLeanTarget.PlannerOracle.PlanFitU.Search) => LexLeanTarget.PlannerOracle.PlanFitU.searchStep (target) (__r))) ((fun (__r : LexLeanTarget.PlannerOracle.PlanFitU.Search) => ((__r).ledger).verifications)) (LexLeanTarget.PlannerOracle.PlanFitU.search_verify_growth (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (LexLeanTarget.PlannerOracle.PlanFitU.start (target)) rfl)
public theorem PlanFitU.expansions_bounded (target : Nat) : ((((LexLeanTarget.PlannerOracle.PlanFitU.run (target)).1).ledger).expansions <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitU.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : LexLeanTarget.PlannerOracle.PlanFitU.Search) => LexLeanTarget.PlannerOracle.PlanFitU.searchStep (target) (__r))) ((fun (__r : LexLeanTarget.PlannerOracle.PlanFitU.Search) => ((__r).ledger).expansions)) (LexLeanTarget.PlannerOracle.PlanFitU.search_expanded (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (LexLeanTarget.PlannerOracle.PlanFitU.start (target)) rfl)
public theorem PlanFitU.iterations_bounded (target : Nat) : ((((LexLeanTarget.PlannerOracle.PlanFitU.run (target)).1).ledger).iterations <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitU.run]
  exact (LexLeanReasoning.iterateUntilCount ((fun (__r : LexLeanTarget.PlannerOracle.PlanFitU.Search) => LexLeanTarget.PlannerOracle.PlanFitU.searchStep (target) (__r))) ((fun (__r : LexLeanTarget.PlannerOracle.PlanFitU.Search) => ((__r).ledger).iterations)) (LexLeanTarget.PlannerOracle.PlanFitU.search_count (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (LexLeanTarget.PlannerOracle.PlanFitU.start (target)) rfl)
public theorem PlanFitU.explained (target : Nat) : (forall (__v : Nat), (forall (__trace : List (LexLeanTarget.PlannerOracle.PlanFitU.Step)), ((LexLeanTarget.PlannerOracle.PlanFitU (target) = Except.ok ((__v, __trace))) -> ((LexLeanTarget.PlannerOracle.PlanFitU.answer (target) (__trace) = Option.some (__v)) /\ LexLeanTarget.PlannerOracle.Whole (target) (__v))))) :=
by
  intro llV llT llE
  have llSearch := LexLeanTarget.PlannerOracle.PlanFitU.search_ok target
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitU] at llE
  generalize llRun : LexLeanTarget.PlannerOracle.PlanFitU.run target = llR at llE llSearch
  split at llE
  · rename_i llHit llF
    cases llE
    have llOk := And.right llSearch
    rw [llF] at llOk
    exact And.intro (by dsimp only [LexLeanTarget.PlannerOracle.PlanFitU.answer]; rw [And.left llOk]; exact And.right llOk) (LexLeanTarget.PlannerOracle.PlanFitU.accept_sound _ _ _ (And.right llOk))
  · cases llE
public theorem PlanFitU.verdict_sound (target : Nat) : (forall (__v : Nat), ((LexLeanTarget.PlannerOracle.PlanFitU.verdict (target) = Except.ok (__v)) -> LexLeanTarget.PlannerOracle.Whole (target) (__v))) :=
by
  intro llV llE
  dsimp only [LexLeanTarget.PlannerOracle.PlanFitU.verdict] at llE
  generalize llHP : LexLeanTarget.PlannerOracle.PlanFitU target = llR at llE
  cases llR with
  | error _ => cases llE
  | ok llP =>
    cases llE
    exact And.right (LexLeanTarget.PlannerOracle.PlanFitU.explained _ llP.1 llP.2 llHP)

end LexLeanTarget.PlannerOracle
