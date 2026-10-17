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
namespace LexLeanTarget.BudgetOracle

@[expose, reducible] public def Spent (T : Type) (s : (Prod (T) (Nat))) (t : (Prod (T) (Nat))) : Prop := (((t).1 = (s).1) /\ ((t).2 < (s).2))

@[expose] public def Remaining (T : Type) (s : (Prod (T) (Nat))) : Nat := (s).2


public theorem tick_sound (T : Type) (s : (Prod (T) (Nat))) : (((Nat.blt (2) ((s).2)) = true) -> Spent (T) (s) (((s).1, (LexLeanRuntime.subtract ((s).2) (1) : Nat)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Spent, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem tick_progress (T : Type) (s : (Prod (T) (Nat))) : (((Nat.blt (2) ((s).2)) = true) -> (Remaining (T) (((s).1, (LexLeanRuntime.subtract ((s).2) (1) : Nat))) < Remaining (T) (s))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Remaining, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

@[expose] public def Tick.guard (T : Type) (s : (Prod (T) (Nat))) : Bool := (Nat.blt (2) ((s).2))
@[expose] public def Tick.conclusion (T : Type) (s : (Prod (T) (Nat))) : (Prod (T) (Nat)) := ((s).1, (LexLeanRuntime.subtract ((s).2) (1) : Nat))
@[expose] public def Tick.apply (T : Type) (s : (Prod (T) (Nat))) : Option ((Prod (T) (Nat))) := (if LexLeanTarget.BudgetOracle.Tick.guard (T) (s) then Option.some (LexLeanTarget.BudgetOracle.Tick.conclusion (T) (s)) else Option.none)
public theorem Tick.apply_sound (T : Type) (s : (Prod (T) (Nat))) (__t : (Prod (T) (Nat))) : ((LexLeanTarget.BudgetOracle.Tick.apply (T) (s) = Option.some (__t)) -> LexLeanTarget.BudgetOracle.Spent (T) (s) (__t)) :=
  (LexLeanReasoning.guarded ((LexLeanTarget.BudgetOracle.Spent (T))) (s) (LexLeanTarget.BudgetOracle.Tick.guard (T) (s)) (LexLeanTarget.BudgetOracle.Tick.conclusion (T) (s)) ((LexLeanTarget.BudgetOracle.tick_sound (T)) (s)) (__t))
public theorem Tick.apply_progress (T : Type) (s : (Prod (T) (Nat))) (__t : (Prod (T) (Nat))) : ((LexLeanTarget.BudgetOracle.Tick.apply (T) (s) = Option.some (__t)) -> (LexLeanTarget.BudgetOracle.Remaining (T) (__t) < LexLeanTarget.BudgetOracle.Remaining (T) (s))) :=
  (LexLeanReasoning.guardedRank ((LexLeanTarget.BudgetOracle.Remaining (T))) (s) (LexLeanTarget.BudgetOracle.Tick.guard (T) (s)) (LexLeanTarget.BudgetOracle.Tick.conclusion (T) (s)) ((LexLeanTarget.BudgetOracle.tick_progress (T)) (s)) (__t))

public theorem burn_sound (T : Type) (s : (Prod (T) (Nat))) (k : Nat) : ((((Nat.ble (k) ((s).2)) && (Nat.blt (0) (k))) = true) -> Spent (T) (s) (((s).1, (LexLeanRuntime.subtract ((s).2) (k) : Nat)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Spent, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem burn_progress (T : Type) (s : (Prod (T) (Nat))) (k : Nat) : ((((Nat.ble (k) ((s).2)) && (Nat.blt (0) (k))) = true) -> (Remaining (T) (((s).1, (LexLeanRuntime.subtract ((s).2) (k) : Nat))) < Remaining (T) (s))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Remaining, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

@[expose] public def Burn.guard (T : Type) (s : (Prod (T) (Nat))) (k : Nat) : Bool := ((Nat.ble (k) ((s).2)) && (Nat.blt (0) (k)))
@[expose] public def Burn.conclusion (T : Type) (s : (Prod (T) (Nat))) (k : Nat) : (Prod (T) (Nat)) := ((s).1, (LexLeanRuntime.subtract ((s).2) (k) : Nat))
@[expose] public def Burn.candidates (T : Type) (_s : (Prod (T) (Nat))) : List (Nat) := (2 :: (1 :: ([] : List (Nat))))
@[expose] public def Burn.apply (T : Type) (s : (Prod (T) (Nat))) (k : Nat) : Option ((Prod (T) (Nat))) := (if LexLeanTarget.BudgetOracle.Burn.guard (T) (s) (k) then Option.some (LexLeanTarget.BudgetOracle.Burn.conclusion (T) (s) (k)) else Option.none)
public theorem Burn.apply_sound (T : Type) (s : (Prod (T) (Nat))) (k : Nat) (__t : (Prod (T) (Nat))) : ((LexLeanTarget.BudgetOracle.Burn.apply (T) (s) (k) = Option.some (__t)) -> LexLeanTarget.BudgetOracle.Spent (T) (s) (__t)) :=
  (LexLeanReasoning.guarded ((LexLeanTarget.BudgetOracle.Spent (T))) (s) (LexLeanTarget.BudgetOracle.Burn.guard (T) (s) (k)) (LexLeanTarget.BudgetOracle.Burn.conclusion (T) (s) (k)) ((LexLeanTarget.BudgetOracle.burn_sound (T)) (s) (k)) (__t))
public theorem Burn.apply_progress (T : Type) (s : (Prod (T) (Nat))) (k : Nat) (__t : (Prod (T) (Nat))) : ((LexLeanTarget.BudgetOracle.Burn.apply (T) (s) (k) = Option.some (__t)) -> (LexLeanTarget.BudgetOracle.Remaining (T) (__t) < LexLeanTarget.BudgetOracle.Remaining (T) (s))) :=
  (LexLeanReasoning.guardedRank ((LexLeanTarget.BudgetOracle.Remaining (T))) (s) (LexLeanTarget.BudgetOracle.Burn.guard (T) (s) (k)) (LexLeanTarget.BudgetOracle.Burn.conclusion (T) (s) (k)) ((LexLeanTarget.BudgetOracle.burn_progress (T)) (s) (k)) (__t))

@[expose, reducible] public def Empty (T : Type) (_x : T) (r : Nat) : Prop := (r = 0)

@[expose] public def emptyCheck (T : Type) (_x : T) (r : Nat) : Bool := (Nat.beq (r) (0))

public theorem empty_sound (T : Type) (x : T) (r : Nat) : ((emptyCheck (T) (x) (r) = true) -> Empty (T) (x) (r)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Empty, emptyCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem Drained.sound (T : Type) : (LexLeanReasoning.Sound ((LexLeanTarget.BudgetOracle.emptyCheck (T))) ((LexLeanTarget.BudgetOracle.Empty (T)))) :=
  (LexLeanTarget.BudgetOracle.empty_sound (T))

public theorem allowance_bound (T : Type) (x : (Prod (T) (Nat))) : (Remaining (T) (((x).1, ((x).2 + 3))) < ((x).2 + 4)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Remaining, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public inductive Spend.Step (T : Type) where
  | Tick
  | Burn (_ : Nat)
public structure Spend.Ledger (T : Type) where
  iterations : Nat
  attempts : Nat
  firings : Nat
  expansions : Nat
  verifications : Nat
  frontier : Nat
@[expose] public def Spend.observe (T : Type) (x : (Prod (T) (Nat))) : (Prod (T) (Nat)) := ((x).1, ((x).2 + 3))
@[expose] public def Spend.fire (T : Type) (__s : (Prod (T) (Nat))) (__step : LexLeanTarget.BudgetOracle.Spend.Step (T)) : Option ((Prod (T) (Nat))) := (match __step with | LexLeanTarget.BudgetOracle.Spend.Step.Tick => LexLeanTarget.BudgetOracle.Tick.apply (T) (__s) | LexLeanTarget.BudgetOracle.Spend.Step.Burn __b => LexLeanTarget.BudgetOracle.Burn.apply (T) (__s) (__b))
@[expose] public def Spend.replay (T : Type) (__acc : Except ((Prod Bool Bool)) ((Prod (T) (Nat)))) (__step : LexLeanTarget.BudgetOracle.Spend.Step (T)) : Except ((Prod Bool Bool)) ((Prod (T) (Nat))) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match LexLeanTarget.BudgetOracle.Spend.fire (T) (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def Spend.follow (T : Type) (x : (Prod (T) (Nat))) (__trace : List (LexLeanTarget.BudgetOracle.Spend.Step (T))) : Except ((Prod Bool Bool)) ((Prod (T) (Nat))) := (LexLeanCollections.listFold ((LexLeanTarget.BudgetOracle.Spend.replay (T))) (Except.ok (LexLeanTarget.BudgetOracle.Spend.observe (T) (x))) (__trace) : Except ((Prod Bool Bool)) ((Prod (T) (Nat))))
@[expose] public def Spend.extract (T : Type) (account : (Prod (T) (Nat))) : Option (Nat) := Option.some ((account).2)
@[expose] public def Spend.accept (T : Type) (x : (Prod (T) (Nat))) (__s : (Prod (T) (Nat))) : Option (Nat) := (match LexLeanTarget.BudgetOracle.Spend.extract (T) (__s) with | Option.none => Option.none | Option.some __v => (if LexLeanTarget.BudgetOracle.emptyCheck ((Prod (T) (Nat))) (x) (__v) then Option.some (__v) else Option.none))
@[expose] public def Spend.answer (T : Type) (x : (Prod (T) (Nat))) (__trace : List (LexLeanTarget.BudgetOracle.Spend.Step (T))) : Option (Nat) := (match LexLeanTarget.BudgetOracle.Spend.follow (T) (x) (__trace) with | Except.ok __s => LexLeanTarget.BudgetOracle.Spend.accept (T) (x) (__s) | Except.error _ => Option.none)
@[expose] public def Spend.select (T : Type) (__s : (Prod (T) (Nat))) : Option (LexLeanTarget.BudgetOracle.Spend.Step (T)) := (match (if LexLeanTarget.BudgetOracle.Tick.guard (T) (__s) then Option.some (LexLeanTarget.BudgetOracle.Spend.Step.Tick) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (LexLeanCollections.listFold ((fun (__acc : Option (LexLeanTarget.BudgetOracle.Spend.Step (T))) (__b : Nat) => (match __acc with | Option.some __found => Option.some (__found) | Option.none => (if LexLeanTarget.BudgetOracle.Burn.guard (T) (__s) (__b) then Option.some (LexLeanTarget.BudgetOracle.Spend.Step.Burn (__b)) else Option.none)))) (Option.none) (LexLeanTarget.BudgetOracle.Burn.candidates (T) (__s)) : Option (LexLeanTarget.BudgetOracle.Spend.Step (T))))
@[expose] public def Spend.attempts (T : Type) (__s : (Prod (T) (Nat))) : Nat := ((let __scan0l : (Prod (Bool) (Nat)) := (LexLeanTarget.BudgetOracle.Tick.guard (T) (__s), 1); (if (__scan0l).1 then __scan0l else (let __scan0r : (Prod (Bool) (Nat)) := (LexLeanCollections.listFold ((fun (__acc : (Prod (Bool) (Nat))) (__b : Nat) => (if (__acc).1 then __acc else (LexLeanTarget.BudgetOracle.Burn.guard (T) (__s) (__b), ((__acc).2 + 1))))) ((false, 0)) (LexLeanTarget.BudgetOracle.Burn.candidates (T) (__s)) : (Prod (Bool) (Nat))); ((__scan0r).1, ((__scan0l).2 + (__scan0r).2)))))).2
@[expose] public def Spend.next (T : Type) (__s : (Prod (T) (Nat))) : Option ((Prod (T) (Nat))) := (match LexLeanTarget.BudgetOracle.Spend.select (T) (__s) with | Option.none => Option.none | Option.some __step => LexLeanTarget.BudgetOracle.Spend.fire (T) (__s) (__step))
@[expose] public def Spend.saturate (T : Type) (x : (Prod (T) (Nat))) : (Prod ((Prod (T) (Nat))) (Bool)) := (LexLeanCollections.iterateUntil ((LexLeanTarget.BudgetOracle.Spend.next (T))) (((x).2 + 4)) (LexLeanTarget.BudgetOracle.Spend.observe (T) (x)) : (Prod ((Prod (T) (Nat))) (Bool)))
public structure Spend.Run (T : Type) where
  state : (Prod (T) (Nat))
  trace : List (LexLeanTarget.BudgetOracle.Spend.Step (T))
  ledger : LexLeanTarget.BudgetOracle.Spend.Ledger (T)
@[expose] public def Spend.start (T : Type) (x : (Prod (T) (Nat))) : LexLeanTarget.BudgetOracle.Spend.Run (T) := ({ state := LexLeanTarget.BudgetOracle.Spend.observe (T) (x), trace := ([] : List (LexLeanTarget.BudgetOracle.Spend.Step (T))), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := 0 } : LexLeanTarget.BudgetOracle.Spend.Ledger (T)) } : LexLeanTarget.BudgetOracle.Spend.Run (T))
@[expose] public def Spend.step (T : Type) (__r : LexLeanTarget.BudgetOracle.Spend.Run (T)) : Option (LexLeanTarget.BudgetOracle.Spend.Run (T)) := (match LexLeanTarget.BudgetOracle.Spend.select (T) ((__r).state) with | Option.none => Option.none | Option.some __step => (match LexLeanTarget.BudgetOracle.Spend.fire (T) ((__r).state) (__step) with | Option.none => Option.none | Option.some __t => Option.some (({ state := __t, trace := (LexLeanRuntime.append ((__r).trace) ((__step :: ([] : List (LexLeanTarget.BudgetOracle.Spend.Step (T))))) : List (LexLeanTarget.BudgetOracle.Spend.Step (T))), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + LexLeanTarget.BudgetOracle.Spend.attempts (T) ((__r).state)), firings := (((__r).ledger).firings + 1), expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : LexLeanTarget.BudgetOracle.Spend.Ledger (T)) } : LexLeanTarget.BudgetOracle.Spend.Run (T)))))
@[expose] public def Spend.run (T : Type) (x : (Prod (T) (Nat))) : (Prod (LexLeanTarget.BudgetOracle.Spend.Run (T)) (Bool)) := (LexLeanCollections.iterateUntil ((LexLeanTarget.BudgetOracle.Spend.step (T))) (((x).2 + 4)) (LexLeanTarget.BudgetOracle.Spend.start (T) (x)) : (Prod (LexLeanTarget.BudgetOracle.Spend.Run (T)) (Bool)))
@[expose] public def Spend.account (T : Type) (x : (Prod (T) (Nat))) : LexLeanTarget.BudgetOracle.Spend.Ledger (T) := (let __final : (Prod (LexLeanTarget.BudgetOracle.Spend.Run (T)) (Bool)) := LexLeanTarget.BudgetOracle.Spend.run (T) (x); (match (__final).2 with | Bool.true => ({ iterations := (((__final).1).ledger).iterations, attempts := ((((__final).1).ledger).attempts + LexLeanTarget.BudgetOracle.Spend.attempts (T) (((__final).1).state)), firings := (((__final).1).ledger).firings, expansions := (((__final).1).ledger).expansions, verifications := ((((__final).1).ledger).verifications + 1), frontier := (((__final).1).ledger).frontier } : LexLeanTarget.BudgetOracle.Spend.Ledger (T)) | Bool.false => ((__final).1).ledger))
@[expose] public def Spend.conclude (T : Type) (x : (Prod (T) (Nat))) (__s : (Prod (T) (Nat))) : Except ((Prod Bool Bool)) (Nat) := (match LexLeanTarget.BudgetOracle.Spend.accept (T) (x) (__s) with | Option.some __v => Except.ok (__v) | Option.none => (match LexLeanTarget.BudgetOracle.Spend.extract (T) (__s) with | Option.some _ => Except.error (((true, false) : Prod Bool Bool)) | Option.none => Except.error (((false, true) : Prod Bool Bool))))
@[expose] public def Spend.verdict (T : Type) (x : (Prod (T) (Nat))) : Except ((Prod Bool Bool)) (Nat) := (let __final : (Prod ((Prod (T) (Nat))) (Bool)) := LexLeanTarget.BudgetOracle.Spend.saturate (T) (x); (match (__final).2 with | Bool.true => LexLeanTarget.BudgetOracle.Spend.conclude (T) (x) ((__final).1) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
@[expose] public def Spend (T : Type) (x : (Prod (T) (Nat))) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (LexLeanTarget.BudgetOracle.Spend.Step (T))))) := (let __final : (Prod (LexLeanTarget.BudgetOracle.Spend.Run (T)) (Bool)) := LexLeanTarget.BudgetOracle.Spend.run (T) (x); (match (__final).2 with | Bool.true => (match LexLeanTarget.BudgetOracle.Spend.conclude (T) (x) (((__final).1).state) with | Except.ok __v => Except.ok ((__v, ((__final).1).trace)) | Except.error __e => Except.error (__e)) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
public theorem Spend.fire_sound (T : Type) (__s : (Prod (T) (Nat))) (__step : LexLeanTarget.BudgetOracle.Spend.Step (T)) (__t : (Prod (T) (Nat))) : ((LexLeanTarget.BudgetOracle.Spend.fire (T) (__s) (__step) = Option.some (__t)) -> LexLeanTarget.BudgetOracle.Spent (T) (__s) (__t)) :=
by
  cases __step with
  | Tick =>
    exact LexLeanTarget.BudgetOracle.Tick.apply_sound (T) __s __t
  | Burn __b =>
    exact LexLeanTarget.BudgetOracle.Burn.apply_sound (T) __s __b __t
public theorem Spend.fire_progress (T : Type) (__s : (Prod (T) (Nat))) (__step : LexLeanTarget.BudgetOracle.Spend.Step (T)) (__t : (Prod (T) (Nat))) : ((LexLeanTarget.BudgetOracle.Spend.fire (T) (__s) (__step) = Option.some (__t)) -> (LexLeanTarget.BudgetOracle.Remaining (T) (__t) < LexLeanTarget.BudgetOracle.Remaining (T) (__s))) :=
by
  cases __step with
  | Tick =>
    exact LexLeanTarget.BudgetOracle.Tick.apply_progress (T) __s __t
  | Burn __b =>
    exact LexLeanTarget.BudgetOracle.Burn.apply_progress (T) __s __b __t
public theorem Spend.replay_fire (T : Type) (__s : (Prod (T) (Nat))) (__step : LexLeanTarget.BudgetOracle.Spend.Step (T)) (__t : (Prod (T) (Nat))) : ((LexLeanTarget.BudgetOracle.Spend.fire (T) (__s) (__step) = Option.some (__t)) -> (LexLeanTarget.BudgetOracle.Spend.replay (T) (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [LexLeanTarget.BudgetOracle.Spend.replay]
  rw [llE]
public theorem Spend.replay_sound (T : Type) (x : (Prod (T) (Nat))) (__acc : Except ((Prod Bool Bool)) ((Prod (T) (Nat)))) (__step : LexLeanTarget.BudgetOracle.Spend.Step (T)) : ((LexLeanReasoning.Reaches ((LexLeanTarget.BudgetOracle.Spent (T))) (LexLeanTarget.BudgetOracle.Spend.observe (T) (x)) (__acc)) -> (LexLeanReasoning.Reaches ((LexLeanTarget.BudgetOracle.Spent (T))) (LexLeanTarget.BudgetOracle.Spend.observe (T) (x)) (LexLeanTarget.BudgetOracle.Spend.replay (T) (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [LexLeanTarget.BudgetOracle.Spend.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) ((LexLeanTarget.BudgetOracle.Spend.fire_sound (T)) llS __step _ ‹_›)
public theorem Spend.derivation (T : Type) (x : (Prod (T) (Nat))) (__trace : List (LexLeanTarget.BudgetOracle.Spend.Step (T))) : (LexLeanReasoning.Reaches ((LexLeanTarget.BudgetOracle.Spent (T))) (LexLeanTarget.BudgetOracle.Spend.observe (T) (x)) (LexLeanTarget.BudgetOracle.Spend.follow (T) (x) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((LexLeanTarget.BudgetOracle.Spend.replay (T))) (fun (__acc : Except ((Prod Bool Bool)) ((Prod (T) (Nat)))) => (LexLeanReasoning.Reaches ((LexLeanTarget.BudgetOracle.Spent (T))) (LexLeanTarget.BudgetOracle.Spend.observe (T) (x)) (__acc))) ((LexLeanTarget.BudgetOracle.Spend.replay_sound (T)) (x)) (__trace) (Except.ok (LexLeanTarget.BudgetOracle.Spend.observe (T) (x))) (LexLeanReasoning.reachesStart ((LexLeanTarget.BudgetOracle.Spent (T))) (LexLeanTarget.BudgetOracle.Spend.observe (T) (x))))
public theorem Spend.accept_sound (T : Type) (x : (Prod (T) (Nat))) (__s : (Prod (T) (Nat))) (__v : Nat) : ((LexLeanTarget.BudgetOracle.Spend.accept (T) (x) (__s) = Option.some (__v)) -> LexLeanTarget.BudgetOracle.Empty ((Prod (T) (Nat))) (x) (__v)) :=
by
  intro llE
  dsimp only [LexLeanTarget.BudgetOracle.Spend.accept] at llE
  split at llE
  · cases llE
  · have llC := LexLeanReasoning.checked _ _ _ llE
    exact llC.right ▸ LexLeanTarget.BudgetOracle.empty_sound ((Prod (T) (Nat))) _ _ llC.left
public theorem Spend.next_sound (T : Type) (__s : (Prod (T) (Nat))) (__t : (Prod (T) (Nat))) : ((LexLeanTarget.BudgetOracle.Spend.next (T) (__s) = Option.some (__t)) -> LexLeanTarget.BudgetOracle.Spent (T) (__s) (__t)) :=
by
  intro llE
  dsimp only [LexLeanTarget.BudgetOracle.Spend.next] at llE
  split at llE
  · cases llE
  · exact (LexLeanTarget.BudgetOracle.Spend.fire_sound (T)) __s _ __t llE
public theorem Spend.next_progress (T : Type) (__s : (Prod (T) (Nat))) (__t : (Prod (T) (Nat))) : ((LexLeanTarget.BudgetOracle.Spend.next (T) (__s) = Option.some (__t)) -> (LexLeanTarget.BudgetOracle.Remaining (T) (__t) < LexLeanTarget.BudgetOracle.Remaining (T) (__s))) :=
by
  intro llE
  dsimp only [LexLeanTarget.BudgetOracle.Spend.next] at llE
  split at llE
  · cases llE
  · exact (LexLeanTarget.BudgetOracle.Spend.fire_progress (T)) __s _ __t llE
public theorem Spend.step_none (T : Type) (__r : LexLeanTarget.BudgetOracle.Spend.Run (T)) : ((LexLeanTarget.BudgetOracle.Spend.step (T) (__r) = Option.none) -> (LexLeanTarget.BudgetOracle.Spend.next (T) ((__r).state) = Option.none)) :=
by
  intro llE
  dsimp only [LexLeanTarget.BudgetOracle.Spend.step] at llE
  split at llE
  · rename_i llH
    dsimp only [LexLeanTarget.BudgetOracle.Spend.next]
    rw [llH]
  · rename_i llH
    split at llE
    · dsimp only [LexLeanTarget.BudgetOracle.Spend.next]
      rw [llH]
      assumption
    · cases llE
public theorem Spend.step_some (T : Type) (__r : LexLeanTarget.BudgetOracle.Spend.Run (T)) (__q : LexLeanTarget.BudgetOracle.Spend.Run (T)) : ((LexLeanTarget.BudgetOracle.Spend.step (T) (__r) = Option.some (__q)) -> (LexLeanTarget.BudgetOracle.Spend.next (T) ((__r).state) = Option.some ((__q).state))) :=
by
  intro llE
  dsimp only [LexLeanTarget.BudgetOracle.Spend.step] at llE
  split at llE
  · cases llE
  · rename_i llH
    split at llE
    · cases llE
    · cases llE
      dsimp only [LexLeanTarget.BudgetOracle.Spend.next]
      rw [llH]
      assumption
public theorem Spend.step_trace (T : Type) (x : (Prod (T) (Nat))) (__r : LexLeanTarget.BudgetOracle.Spend.Run (T)) (__q : LexLeanTarget.BudgetOracle.Spend.Run (T)) : ((LexLeanTarget.BudgetOracle.Spend.step (T) (__r) = Option.some (__q)) -> ((LexLeanTarget.BudgetOracle.Spend.follow (T) (x) ((__r).trace) = Except.ok ((__r).state)) -> (LexLeanTarget.BudgetOracle.Spend.follow (T) (x) ((__q).trace) = Except.ok ((__q).state)))) :=
by
  intro llE llH
  dsimp only [LexLeanTarget.BudgetOracle.Spend.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      dsimp only [LexLeanTarget.BudgetOracle.Spend.follow]
      rw [LexLeanReasoning.foldSnoc]
      dsimp only [LexLeanTarget.BudgetOracle.Spend.follow] at llH
      rw [llH]
      apply (LexLeanTarget.BudgetOracle.Spend.replay_fire (T))
      assumption
public theorem Spend.step_count (T : Type) (__r : LexLeanTarget.BudgetOracle.Spend.Run (T)) (__q : LexLeanTarget.BudgetOracle.Spend.Run (T)) : ((LexLeanTarget.BudgetOracle.Spend.step (T) (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [LexLeanTarget.BudgetOracle.Spend.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Spend.step_fired (T : Type) (__r : LexLeanTarget.BudgetOracle.Spend.Run (T)) (__q : LexLeanTarget.BudgetOracle.Spend.Run (T)) : ((LexLeanTarget.BudgetOracle.Spend.step (T) (__r) = Option.some (__q)) -> (((__q).ledger).firings = (((__r).ledger).firings + 1))) :=
by
  intro llE
  dsimp only [LexLeanTarget.BudgetOracle.Spend.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Spend.saturate_derivation (T : Type) (x : (Prod (T) (Nat))) : (LexLeanReasoning.Star ((LexLeanTarget.BudgetOracle.Spent (T))) (LexLeanTarget.BudgetOracle.Spend.observe (T) (x)) ((LexLeanTarget.BudgetOracle.Spend.saturate (T) (x)).1)) :=
by
  dsimp only [LexLeanTarget.BudgetOracle.Spend.saturate]
  exact (LexLeanReasoning.iterateUntilInvariant ((LexLeanTarget.BudgetOracle.Spend.next (T))) (fun (__s : (Prod (T) (Nat))) => (LexLeanReasoning.Star ((LexLeanTarget.BudgetOracle.Spent (T))) (LexLeanTarget.BudgetOracle.Spend.observe (T) (x)) (__s))) (fun llA llB llE llH => (LexLeanReasoning.Star.tail _ llA llB llH ((LexLeanTarget.BudgetOracle.Spend.next_sound (T)) llA llB llE))) (((x).2 + 4)) (LexLeanTarget.BudgetOracle.Spend.observe (T) (x)) (LexLeanReasoning.Star.refl _))
public theorem Spend.run_state (T : Type) (x : (Prod (T) (Nat))) : ((((LexLeanTarget.BudgetOracle.Spend.run (T) (x)).1).state = (LexLeanTarget.BudgetOracle.Spend.saturate (T) (x)).1) /\ ((LexLeanTarget.BudgetOracle.Spend.run (T) (x)).2 = (LexLeanTarget.BudgetOracle.Spend.saturate (T) (x)).2)) :=
by
  dsimp only [LexLeanTarget.BudgetOracle.Spend.run, LexLeanTarget.BudgetOracle.Spend.saturate]
  exact (LexLeanReasoning.iterateUntilSimulate ((LexLeanTarget.BudgetOracle.Spend.step (T))) ((LexLeanTarget.BudgetOracle.Spend.next (T))) ((fun (__r : LexLeanTarget.BudgetOracle.Spend.Run (T)) => (__r).state)) (LexLeanTarget.BudgetOracle.Spend.step_none (T)) (LexLeanTarget.BudgetOracle.Spend.step_some (T)) (((x).2 + 4)) (LexLeanTarget.BudgetOracle.Spend.start (T) (x)))
public theorem Spend.run_trace (T : Type) (x : (Prod (T) (Nat))) : (LexLeanTarget.BudgetOracle.Spend.follow (T) (x) (((LexLeanTarget.BudgetOracle.Spend.run (T) (x)).1).trace) = Except.ok (((LexLeanTarget.BudgetOracle.Spend.run (T) (x)).1).state)) :=
by
  dsimp only [LexLeanTarget.BudgetOracle.Spend.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((LexLeanTarget.BudgetOracle.Spend.step (T))) (fun (__r : LexLeanTarget.BudgetOracle.Spend.Run (T)) => (LexLeanTarget.BudgetOracle.Spend.follow (T) (x) ((__r).trace) = Except.ok ((__r).state))) ((LexLeanTarget.BudgetOracle.Spend.step_trace (T)) (x)) (((x).2 + 4)) (LexLeanTarget.BudgetOracle.Spend.start (T) (x)) rfl)
public theorem Spend.iterations_bounded (T : Type) (x : (Prod (T) (Nat))) : ((((LexLeanTarget.BudgetOracle.Spend.run (T) (x)).1).ledger).iterations <= ((x).2 + 4)) :=
by
  dsimp only [LexLeanTarget.BudgetOracle.Spend.run]
  exact (LexLeanReasoning.iterateUntilCount ((LexLeanTarget.BudgetOracle.Spend.step (T))) ((fun (__r : LexLeanTarget.BudgetOracle.Spend.Run (T)) => ((__r).ledger).iterations)) (LexLeanTarget.BudgetOracle.Spend.step_count (T)) (((x).2 + 4)) (LexLeanTarget.BudgetOracle.Spend.start (T) (x)) rfl)
public theorem Spend.firings_bounded (T : Type) (x : (Prod (T) (Nat))) : ((((LexLeanTarget.BudgetOracle.Spend.run (T) (x)).1).ledger).firings <= ((x).2 + 4)) :=
by
  dsimp only [LexLeanTarget.BudgetOracle.Spend.run]
  exact (LexLeanReasoning.iterateUntilCount ((LexLeanTarget.BudgetOracle.Spend.step (T))) ((fun (__r : LexLeanTarget.BudgetOracle.Spend.Run (T)) => ((__r).ledger).firings)) (LexLeanTarget.BudgetOracle.Spend.step_fired (T)) (((x).2 + 4)) (LexLeanTarget.BudgetOracle.Spend.start (T) (x)) rfl)
public theorem Spend.saturates (T : Type) (x : (Prod (T) (Nat))) : ((LexLeanTarget.BudgetOracle.Spend.saturate (T) (x)).2 = true) :=
by
  dsimp only [LexLeanTarget.BudgetOracle.Spend.saturate]
  exact (LexLeanReasoning.iterateUntilStops ((LexLeanTarget.BudgetOracle.Spend.next (T))) (fun (_ : (Prod (T) (Nat))) => True) ((LexLeanTarget.BudgetOracle.Remaining (T))) (fun llA llB llE _llH => (And.intro True.intro ((LexLeanTarget.BudgetOracle.Spend.next_progress (T)) llA llB llE))) (((x).2 + 4)) (LexLeanTarget.BudgetOracle.Spend.observe (T) (x)) True.intro ((LexLeanTarget.BudgetOracle.allowance_bound (T)) (x)))
public theorem Spend.conclude_accept (T : Type) (x : (Prod (T) (Nat))) (__s : (Prod (T) (Nat))) (__v : Nat) : ((LexLeanTarget.BudgetOracle.Spend.conclude (T) (x) (__s) = Except.ok (__v)) -> (LexLeanTarget.BudgetOracle.Spend.accept (T) (x) (__s) = Option.some (__v))) :=
by
  intro llE
  dsimp only [LexLeanTarget.BudgetOracle.Spend.conclude] at llE
  split at llE
  · cases llE
    assumption
  · split at llE
    · cases llE
    · cases llE
public theorem Spend.conclude_sound (T : Type) (x : (Prod (T) (Nat))) (__s : (Prod (T) (Nat))) (__v : Nat) : ((LexLeanTarget.BudgetOracle.Spend.conclude (T) (x) (__s) = Except.ok (__v)) -> LexLeanTarget.BudgetOracle.Empty ((Prod (T) (Nat))) (x) (__v)) :=
by
  intro llE
  dsimp only [LexLeanTarget.BudgetOracle.Spend.conclude] at llE
  split at llE
  · cases llE
    exact (LexLeanTarget.BudgetOracle.Spend.accept_sound (T)) _ _ _ ‹_›
  · split at llE
    · cases llE
    · cases llE
public theorem Spend.verdict_sound (T : Type) (x : (Prod (T) (Nat))) : (forall (__v : Nat), ((LexLeanTarget.BudgetOracle.Spend.verdict (T) (x) = Except.ok (__v)) -> LexLeanTarget.BudgetOracle.Empty ((Prod (T) (Nat))) (x) (__v))) :=
by
  intro llV llE
  dsimp only [LexLeanTarget.BudgetOracle.Spend.verdict] at llE
  generalize llRun : (LexLeanTarget.BudgetOracle.Spend.saturate (T)) x = llR at llE
  split at llE
  · exact (LexLeanTarget.BudgetOracle.Spend.conclude_sound (T)) _ _ _ llE
  · cases llE
public theorem Spend.explained (T : Type) (x : (Prod (T) (Nat))) : (forall (__v : Nat), (forall (__trace : List (LexLeanTarget.BudgetOracle.Spend.Step (T))), ((LexLeanTarget.BudgetOracle.Spend (T) (x) = Except.ok ((__v, __trace))) -> ((LexLeanTarget.BudgetOracle.Spend.answer (T) (x) (__trace) = Option.some (__v)) /\ LexLeanTarget.BudgetOracle.Empty ((Prod (T) (Nat))) (x) (__v))))) :=
by
  intro llV llT llE
  have llTrace := (LexLeanTarget.BudgetOracle.Spend.run_trace (T)) x
  dsimp only [LexLeanTarget.BudgetOracle.Spend] at llE
  generalize llRun : (LexLeanTarget.BudgetOracle.Spend.run (T)) x = llR at llE llTrace
  split at llE
  · split at llE
    · rename_i llW llH
      cases llE
      exact And.intro (by dsimp only [LexLeanTarget.BudgetOracle.Spend.answer]; rw [llTrace]; exact (LexLeanTarget.BudgetOracle.Spend.conclude_accept (T)) _ _ _ llH) ((LexLeanTarget.BudgetOracle.Spend.conclude_sound (T)) _ _ _ llH)
    · cases llE
  · cases llE

public inductive Refund.Step where
  | Tick
  | Burn (_ : Nat)
public structure Refund.Ledger where
  iterations : Nat
  attempts : Nat
  firings : Nat
  expansions : Nat
  verifications : Nat
  frontier : Nat
@[expose] public def Refund.observe (x : Nat) : (Prod (Nat) (Nat)) := (x, (x + 3))
@[expose] public def Refund.fire (__s : (Prod (Nat) (Nat))) (__step : LexLeanTarget.BudgetOracle.Refund.Step) : Option ((Prod (Nat) (Nat))) := (match __step with | LexLeanTarget.BudgetOracle.Refund.Step.Tick => LexLeanTarget.BudgetOracle.Tick.apply (Nat) (__s) | LexLeanTarget.BudgetOracle.Refund.Step.Burn __b => LexLeanTarget.BudgetOracle.Burn.apply (Nat) (__s) (__b))
@[expose] public def Refund.replay (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : LexLeanTarget.BudgetOracle.Refund.Step) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match LexLeanTarget.BudgetOracle.Refund.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def Refund.follow (x : Nat) (__trace : List (LexLeanTarget.BudgetOracle.Refund.Step)) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (LexLeanCollections.listFold ((LexLeanTarget.BudgetOracle.Refund.replay)) (Except.ok (LexLeanTarget.BudgetOracle.Refund.observe (x))) (__trace) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))))
@[expose] public def Refund.extract (account : (Prod (Nat) (Nat))) : Option (Nat) := (if (Nat.blt ((account).1) (5)) then Option.some ((account).2) else Option.none)
@[expose] public def Refund.accept (x : Nat) (__s : (Prod (Nat) (Nat))) : Option (Nat) := (match LexLeanTarget.BudgetOracle.Refund.extract (__s) with | Option.none => Option.none | Option.some __v => (if LexLeanTarget.BudgetOracle.emptyCheck (Nat) (x) (__v) then Option.some (__v) else Option.none))
@[expose] public def Refund.answer (x : Nat) (__trace : List (LexLeanTarget.BudgetOracle.Refund.Step)) : Option (Nat) := (match LexLeanTarget.BudgetOracle.Refund.follow (x) (__trace) with | Except.ok __s => LexLeanTarget.BudgetOracle.Refund.accept (x) (__s) | Except.error _ => Option.none)
@[expose] public def Refund.select (__s : (Prod (Nat) (Nat))) : Option (LexLeanTarget.BudgetOracle.Refund.Step) := (match (if LexLeanTarget.BudgetOracle.Tick.guard (Nat) (__s) then Option.some (LexLeanTarget.BudgetOracle.Refund.Step.Tick) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (LexLeanCollections.listFold ((fun (__acc : Option (LexLeanTarget.BudgetOracle.Refund.Step)) (__b : Nat) => (match __acc with | Option.some __found => Option.some (__found) | Option.none => (if LexLeanTarget.BudgetOracle.Burn.guard (Nat) (__s) (__b) then Option.some (LexLeanTarget.BudgetOracle.Refund.Step.Burn (__b)) else Option.none)))) (Option.none) (LexLeanTarget.BudgetOracle.Burn.candidates (Nat) (__s)) : Option (LexLeanTarget.BudgetOracle.Refund.Step)))
@[expose] public def Refund.attempts (__s : (Prod (Nat) (Nat))) : Nat := ((let __scan0l : (Prod (Bool) (Nat)) := (LexLeanTarget.BudgetOracle.Tick.guard (Nat) (__s), 1); (if (__scan0l).1 then __scan0l else (let __scan0r : (Prod (Bool) (Nat)) := (LexLeanCollections.listFold ((fun (__acc : (Prod (Bool) (Nat))) (__b : Nat) => (if (__acc).1 then __acc else (LexLeanTarget.BudgetOracle.Burn.guard (Nat) (__s) (__b), ((__acc).2 + 1))))) ((false, 0)) (LexLeanTarget.BudgetOracle.Burn.candidates (Nat) (__s)) : (Prod (Bool) (Nat))); ((__scan0r).1, ((__scan0l).2 + (__scan0r).2)))))).2
@[expose] public def Refund.next (__s : (Prod (Nat) (Nat))) : Option ((Prod (Nat) (Nat))) := (match LexLeanTarget.BudgetOracle.Refund.select (__s) with | Option.none => Option.none | Option.some __step => LexLeanTarget.BudgetOracle.Refund.fire (__s) (__step))
@[expose] public def Refund.saturate (x : Nat) : (Prod ((Prod (Nat) (Nat))) (Bool)) := (LexLeanCollections.iterateUntil ((LexLeanTarget.BudgetOracle.Refund.next)) ((x + 4)) (LexLeanTarget.BudgetOracle.Refund.observe (x)) : (Prod ((Prod (Nat) (Nat))) (Bool)))
public structure Refund.Run where
  state : (Prod (Nat) (Nat))
  trace : List (LexLeanTarget.BudgetOracle.Refund.Step)
  ledger : LexLeanTarget.BudgetOracle.Refund.Ledger
@[expose] public def Refund.start (x : Nat) : LexLeanTarget.BudgetOracle.Refund.Run := ({ state := LexLeanTarget.BudgetOracle.Refund.observe (x), trace := ([] : List (LexLeanTarget.BudgetOracle.Refund.Step)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := 0 } : LexLeanTarget.BudgetOracle.Refund.Ledger) } : LexLeanTarget.BudgetOracle.Refund.Run)
@[expose] public def Refund.step (__r : LexLeanTarget.BudgetOracle.Refund.Run) : Option (LexLeanTarget.BudgetOracle.Refund.Run) := (match LexLeanTarget.BudgetOracle.Refund.select ((__r).state) with | Option.none => Option.none | Option.some __step => (match LexLeanTarget.BudgetOracle.Refund.fire ((__r).state) (__step) with | Option.none => Option.none | Option.some __t => Option.some (({ state := __t, trace := (LexLeanRuntime.append ((__r).trace) ((__step :: ([] : List (LexLeanTarget.BudgetOracle.Refund.Step)))) : List (LexLeanTarget.BudgetOracle.Refund.Step)), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + LexLeanTarget.BudgetOracle.Refund.attempts ((__r).state)), firings := (((__r).ledger).firings + 1), expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : LexLeanTarget.BudgetOracle.Refund.Ledger) } : LexLeanTarget.BudgetOracle.Refund.Run))))
@[expose] public def Refund.run (x : Nat) : (Prod (LexLeanTarget.BudgetOracle.Refund.Run) (Bool)) := (LexLeanCollections.iterateUntil ((LexLeanTarget.BudgetOracle.Refund.step)) ((x + 4)) (LexLeanTarget.BudgetOracle.Refund.start (x)) : (Prod (LexLeanTarget.BudgetOracle.Refund.Run) (Bool)))
@[expose] public def Refund.account (x : Nat) : LexLeanTarget.BudgetOracle.Refund.Ledger := (let __final : (Prod (LexLeanTarget.BudgetOracle.Refund.Run) (Bool)) := LexLeanTarget.BudgetOracle.Refund.run (x); (match (__final).2 with | Bool.true => ({ iterations := (((__final).1).ledger).iterations, attempts := ((((__final).1).ledger).attempts + LexLeanTarget.BudgetOracle.Refund.attempts (((__final).1).state)), firings := (((__final).1).ledger).firings, expansions := (((__final).1).ledger).expansions, verifications := ((((__final).1).ledger).verifications + 1), frontier := (((__final).1).ledger).frontier } : LexLeanTarget.BudgetOracle.Refund.Ledger) | Bool.false => ((__final).1).ledger))
@[expose] public def Refund.conclude (x : Nat) (__s : (Prod (Nat) (Nat))) : Except ((Prod Bool Bool)) (Nat) := (match LexLeanTarget.BudgetOracle.Refund.accept (x) (__s) with | Option.some __v => Except.ok (__v) | Option.none => (match LexLeanTarget.BudgetOracle.Refund.extract (__s) with | Option.some _ => Except.error (((true, false) : Prod Bool Bool)) | Option.none => Except.error (((false, true) : Prod Bool Bool))))
@[expose] public def Refund.verdict (x : Nat) : Except ((Prod Bool Bool)) (Nat) := (let __final : (Prod ((Prod (Nat) (Nat))) (Bool)) := LexLeanTarget.BudgetOracle.Refund.saturate (x); (match (__final).2 with | Bool.true => LexLeanTarget.BudgetOracle.Refund.conclude (x) ((__final).1) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
@[expose] public def Refund (x : Nat) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (LexLeanTarget.BudgetOracle.Refund.Step)))) := (let __final : (Prod (LexLeanTarget.BudgetOracle.Refund.Run) (Bool)) := LexLeanTarget.BudgetOracle.Refund.run (x); (match (__final).2 with | Bool.true => (match LexLeanTarget.BudgetOracle.Refund.conclude (x) (((__final).1).state) with | Except.ok __v => Except.ok ((__v, ((__final).1).trace)) | Except.error __e => Except.error (__e)) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
public theorem Refund.fire_sound (__s : (Prod (Nat) (Nat))) (__step : LexLeanTarget.BudgetOracle.Refund.Step) (__t : (Prod (Nat) (Nat))) : ((LexLeanTarget.BudgetOracle.Refund.fire (__s) (__step) = Option.some (__t)) -> LexLeanTarget.BudgetOracle.Spent (Nat) (__s) (__t)) :=
by
  cases __step with
  | Tick =>
    exact LexLeanTarget.BudgetOracle.Tick.apply_sound (Nat) __s __t
  | Burn __b =>
    exact LexLeanTarget.BudgetOracle.Burn.apply_sound (Nat) __s __b __t
public theorem Refund.replay_fire (__s : (Prod (Nat) (Nat))) (__step : LexLeanTarget.BudgetOracle.Refund.Step) (__t : (Prod (Nat) (Nat))) : ((LexLeanTarget.BudgetOracle.Refund.fire (__s) (__step) = Option.some (__t)) -> (LexLeanTarget.BudgetOracle.Refund.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [LexLeanTarget.BudgetOracle.Refund.replay]
  rw [llE]
public theorem Refund.replay_sound (x : Nat) (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : LexLeanTarget.BudgetOracle.Refund.Step) : ((LexLeanReasoning.Reaches ((LexLeanTarget.BudgetOracle.Spent (Nat))) (LexLeanTarget.BudgetOracle.Refund.observe (x)) (__acc)) -> (LexLeanReasoning.Reaches ((LexLeanTarget.BudgetOracle.Spent (Nat))) (LexLeanTarget.BudgetOracle.Refund.observe (x)) (LexLeanTarget.BudgetOracle.Refund.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [LexLeanTarget.BudgetOracle.Refund.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (LexLeanTarget.BudgetOracle.Refund.fire_sound llS __step _ ‹_›)
public theorem Refund.derivation (x : Nat) (__trace : List (LexLeanTarget.BudgetOracle.Refund.Step)) : (LexLeanReasoning.Reaches ((LexLeanTarget.BudgetOracle.Spent (Nat))) (LexLeanTarget.BudgetOracle.Refund.observe (x)) (LexLeanTarget.BudgetOracle.Refund.follow (x) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((LexLeanTarget.BudgetOracle.Refund.replay)) (fun (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) => (LexLeanReasoning.Reaches ((LexLeanTarget.BudgetOracle.Spent (Nat))) (LexLeanTarget.BudgetOracle.Refund.observe (x)) (__acc))) (LexLeanTarget.BudgetOracle.Refund.replay_sound (x)) (__trace) (Except.ok (LexLeanTarget.BudgetOracle.Refund.observe (x))) (LexLeanReasoning.reachesStart ((LexLeanTarget.BudgetOracle.Spent (Nat))) (LexLeanTarget.BudgetOracle.Refund.observe (x))))
public theorem Refund.accept_sound (x : Nat) (__s : (Prod (Nat) (Nat))) (__v : Nat) : ((LexLeanTarget.BudgetOracle.Refund.accept (x) (__s) = Option.some (__v)) -> LexLeanTarget.BudgetOracle.Empty (Nat) (x) (__v)) :=
by
  intro llE
  dsimp only [LexLeanTarget.BudgetOracle.Refund.accept] at llE
  split at llE
  · cases llE
  · have llC := LexLeanReasoning.checked _ _ _ llE
    exact llC.right ▸ LexLeanTarget.BudgetOracle.empty_sound (Nat) _ _ llC.left
public theorem Refund.next_sound (__s : (Prod (Nat) (Nat))) (__t : (Prod (Nat) (Nat))) : ((LexLeanTarget.BudgetOracle.Refund.next (__s) = Option.some (__t)) -> LexLeanTarget.BudgetOracle.Spent (Nat) (__s) (__t)) :=
by
  intro llE
  dsimp only [LexLeanTarget.BudgetOracle.Refund.next] at llE
  split at llE
  · cases llE
  · exact LexLeanTarget.BudgetOracle.Refund.fire_sound __s _ __t llE
public theorem Refund.step_none (__r : LexLeanTarget.BudgetOracle.Refund.Run) : ((LexLeanTarget.BudgetOracle.Refund.step (__r) = Option.none) -> (LexLeanTarget.BudgetOracle.Refund.next ((__r).state) = Option.none)) :=
by
  intro llE
  dsimp only [LexLeanTarget.BudgetOracle.Refund.step] at llE
  split at llE
  · rename_i llH
    dsimp only [LexLeanTarget.BudgetOracle.Refund.next]
    rw [llH]
  · rename_i llH
    split at llE
    · dsimp only [LexLeanTarget.BudgetOracle.Refund.next]
      rw [llH]
      assumption
    · cases llE
public theorem Refund.step_some (__r : LexLeanTarget.BudgetOracle.Refund.Run) (__q : LexLeanTarget.BudgetOracle.Refund.Run) : ((LexLeanTarget.BudgetOracle.Refund.step (__r) = Option.some (__q)) -> (LexLeanTarget.BudgetOracle.Refund.next ((__r).state) = Option.some ((__q).state))) :=
by
  intro llE
  dsimp only [LexLeanTarget.BudgetOracle.Refund.step] at llE
  split at llE
  · cases llE
  · rename_i llH
    split at llE
    · cases llE
    · cases llE
      dsimp only [LexLeanTarget.BudgetOracle.Refund.next]
      rw [llH]
      assumption
public theorem Refund.step_trace (x : Nat) (__r : LexLeanTarget.BudgetOracle.Refund.Run) (__q : LexLeanTarget.BudgetOracle.Refund.Run) : ((LexLeanTarget.BudgetOracle.Refund.step (__r) = Option.some (__q)) -> ((LexLeanTarget.BudgetOracle.Refund.follow (x) ((__r).trace) = Except.ok ((__r).state)) -> (LexLeanTarget.BudgetOracle.Refund.follow (x) ((__q).trace) = Except.ok ((__q).state)))) :=
by
  intro llE llH
  dsimp only [LexLeanTarget.BudgetOracle.Refund.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      dsimp only [LexLeanTarget.BudgetOracle.Refund.follow]
      rw [LexLeanReasoning.foldSnoc]
      dsimp only [LexLeanTarget.BudgetOracle.Refund.follow] at llH
      rw [llH]
      apply LexLeanTarget.BudgetOracle.Refund.replay_fire
      assumption
public theorem Refund.step_count (__r : LexLeanTarget.BudgetOracle.Refund.Run) (__q : LexLeanTarget.BudgetOracle.Refund.Run) : ((LexLeanTarget.BudgetOracle.Refund.step (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [LexLeanTarget.BudgetOracle.Refund.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Refund.step_fired (__r : LexLeanTarget.BudgetOracle.Refund.Run) (__q : LexLeanTarget.BudgetOracle.Refund.Run) : ((LexLeanTarget.BudgetOracle.Refund.step (__r) = Option.some (__q)) -> (((__q).ledger).firings = (((__r).ledger).firings + 1))) :=
by
  intro llE
  dsimp only [LexLeanTarget.BudgetOracle.Refund.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Refund.saturate_derivation (x : Nat) : (LexLeanReasoning.Star ((LexLeanTarget.BudgetOracle.Spent (Nat))) (LexLeanTarget.BudgetOracle.Refund.observe (x)) ((LexLeanTarget.BudgetOracle.Refund.saturate (x)).1)) :=
by
  dsimp only [LexLeanTarget.BudgetOracle.Refund.saturate]
  exact (LexLeanReasoning.iterateUntilInvariant ((LexLeanTarget.BudgetOracle.Refund.next)) (fun (__s : (Prod (Nat) (Nat))) => (LexLeanReasoning.Star ((LexLeanTarget.BudgetOracle.Spent (Nat))) (LexLeanTarget.BudgetOracle.Refund.observe (x)) (__s))) (fun llA llB llE llH => (LexLeanReasoning.Star.tail _ llA llB llH (LexLeanTarget.BudgetOracle.Refund.next_sound llA llB llE))) ((x + 4)) (LexLeanTarget.BudgetOracle.Refund.observe (x)) (LexLeanReasoning.Star.refl _))
public theorem Refund.run_state (x : Nat) : ((((LexLeanTarget.BudgetOracle.Refund.run (x)).1).state = (LexLeanTarget.BudgetOracle.Refund.saturate (x)).1) /\ ((LexLeanTarget.BudgetOracle.Refund.run (x)).2 = (LexLeanTarget.BudgetOracle.Refund.saturate (x)).2)) :=
by
  dsimp only [LexLeanTarget.BudgetOracle.Refund.run, LexLeanTarget.BudgetOracle.Refund.saturate]
  exact (LexLeanReasoning.iterateUntilSimulate ((LexLeanTarget.BudgetOracle.Refund.step)) ((LexLeanTarget.BudgetOracle.Refund.next)) ((fun (__r : LexLeanTarget.BudgetOracle.Refund.Run) => (__r).state)) LexLeanTarget.BudgetOracle.Refund.step_none LexLeanTarget.BudgetOracle.Refund.step_some ((x + 4)) (LexLeanTarget.BudgetOracle.Refund.start (x)))
public theorem Refund.run_trace (x : Nat) : (LexLeanTarget.BudgetOracle.Refund.follow (x) (((LexLeanTarget.BudgetOracle.Refund.run (x)).1).trace) = Except.ok (((LexLeanTarget.BudgetOracle.Refund.run (x)).1).state)) :=
by
  dsimp only [LexLeanTarget.BudgetOracle.Refund.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((LexLeanTarget.BudgetOracle.Refund.step)) (fun (__r : LexLeanTarget.BudgetOracle.Refund.Run) => (LexLeanTarget.BudgetOracle.Refund.follow (x) ((__r).trace) = Except.ok ((__r).state))) (LexLeanTarget.BudgetOracle.Refund.step_trace (x)) ((x + 4)) (LexLeanTarget.BudgetOracle.Refund.start (x)) rfl)
public theorem Refund.iterations_bounded (x : Nat) : ((((LexLeanTarget.BudgetOracle.Refund.run (x)).1).ledger).iterations <= (x + 4)) :=
by
  dsimp only [LexLeanTarget.BudgetOracle.Refund.run]
  exact (LexLeanReasoning.iterateUntilCount ((LexLeanTarget.BudgetOracle.Refund.step)) ((fun (__r : LexLeanTarget.BudgetOracle.Refund.Run) => ((__r).ledger).iterations)) LexLeanTarget.BudgetOracle.Refund.step_count ((x + 4)) (LexLeanTarget.BudgetOracle.Refund.start (x)) rfl)
public theorem Refund.firings_bounded (x : Nat) : ((((LexLeanTarget.BudgetOracle.Refund.run (x)).1).ledger).firings <= (x + 4)) :=
by
  dsimp only [LexLeanTarget.BudgetOracle.Refund.run]
  exact (LexLeanReasoning.iterateUntilCount ((LexLeanTarget.BudgetOracle.Refund.step)) ((fun (__r : LexLeanTarget.BudgetOracle.Refund.Run) => ((__r).ledger).firings)) LexLeanTarget.BudgetOracle.Refund.step_fired ((x + 4)) (LexLeanTarget.BudgetOracle.Refund.start (x)) rfl)
public theorem Refund.conclude_accept (x : Nat) (__s : (Prod (Nat) (Nat))) (__v : Nat) : ((LexLeanTarget.BudgetOracle.Refund.conclude (x) (__s) = Except.ok (__v)) -> (LexLeanTarget.BudgetOracle.Refund.accept (x) (__s) = Option.some (__v))) :=
by
  intro llE
  dsimp only [LexLeanTarget.BudgetOracle.Refund.conclude] at llE
  split at llE
  · cases llE
    assumption
  · split at llE
    · cases llE
    · cases llE
public theorem Refund.conclude_sound (x : Nat) (__s : (Prod (Nat) (Nat))) (__v : Nat) : ((LexLeanTarget.BudgetOracle.Refund.conclude (x) (__s) = Except.ok (__v)) -> LexLeanTarget.BudgetOracle.Empty (Nat) (x) (__v)) :=
by
  intro llE
  dsimp only [LexLeanTarget.BudgetOracle.Refund.conclude] at llE
  split at llE
  · cases llE
    exact LexLeanTarget.BudgetOracle.Refund.accept_sound _ _ _ ‹_›
  · split at llE
    · cases llE
    · cases llE
public theorem Refund.verdict_sound (x : Nat) : (forall (__v : Nat), ((LexLeanTarget.BudgetOracle.Refund.verdict (x) = Except.ok (__v)) -> LexLeanTarget.BudgetOracle.Empty (Nat) (x) (__v))) :=
by
  intro llV llE
  dsimp only [LexLeanTarget.BudgetOracle.Refund.verdict] at llE
  generalize llRun : LexLeanTarget.BudgetOracle.Refund.saturate x = llR at llE
  split at llE
  · exact LexLeanTarget.BudgetOracle.Refund.conclude_sound _ _ _ llE
  · cases llE
public theorem Refund.explained (x : Nat) : (forall (__v : Nat), (forall (__trace : List (LexLeanTarget.BudgetOracle.Refund.Step)), ((LexLeanTarget.BudgetOracle.Refund (x) = Except.ok ((__v, __trace))) -> ((LexLeanTarget.BudgetOracle.Refund.answer (x) (__trace) = Option.some (__v)) /\ LexLeanTarget.BudgetOracle.Empty (Nat) (x) (__v))))) :=
by
  intro llV llT llE
  have llTrace := LexLeanTarget.BudgetOracle.Refund.run_trace x
  dsimp only [LexLeanTarget.BudgetOracle.Refund] at llE
  generalize llRun : LexLeanTarget.BudgetOracle.Refund.run x = llR at llE llTrace
  split at llE
  · split at llE
    · rename_i llW llH
      cases llE
      exact And.intro (by dsimp only [LexLeanTarget.BudgetOracle.Refund.answer]; rw [llTrace]; exact LexLeanTarget.BudgetOracle.Refund.conclude_accept _ _ _ llH) (LexLeanTarget.BudgetOracle.Refund.conclude_sound _ _ _ llH)
    · cases llE
  · cases llE

end LexLeanTarget.BudgetOracle
