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
namespace Reasoning.Budget

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
@[expose] public def Tick.apply (T : Type) (s : (Prod (T) (Nat))) : Option ((Prod (T) (Nat))) := (if Reasoning.Budget.Tick.guard (T) (s) then Option.some (Reasoning.Budget.Tick.conclusion (T) (s)) else Option.none)
public theorem Tick.apply_sound (T : Type) (s : (Prod (T) (Nat))) (__t : (Prod (T) (Nat))) : ((Reasoning.Budget.Tick.apply (T) (s) = Option.some (__t)) -> Reasoning.Budget.Spent (T) (s) (__t)) :=
  (LexLeanReasoning.guarded ((Reasoning.Budget.Spent (T))) (s) (Reasoning.Budget.Tick.guard (T) (s)) (Reasoning.Budget.Tick.conclusion (T) (s)) ((Reasoning.Budget.tick_sound (T)) (s)) (__t))
public theorem Tick.apply_progress (T : Type) (s : (Prod (T) (Nat))) (__t : (Prod (T) (Nat))) : ((Reasoning.Budget.Tick.apply (T) (s) = Option.some (__t)) -> (Reasoning.Budget.Remaining (T) (__t) < Reasoning.Budget.Remaining (T) (s))) :=
  (LexLeanReasoning.guardedRank ((Reasoning.Budget.Remaining (T))) (s) (Reasoning.Budget.Tick.guard (T) (s)) (Reasoning.Budget.Tick.conclusion (T) (s)) ((Reasoning.Budget.tick_progress (T)) (s)) (__t))

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
@[expose] public def Burn.apply (T : Type) (s : (Prod (T) (Nat))) (k : Nat) : Option ((Prod (T) (Nat))) := (if Reasoning.Budget.Burn.guard (T) (s) (k) then Option.some (Reasoning.Budget.Burn.conclusion (T) (s) (k)) else Option.none)
public theorem Burn.apply_sound (T : Type) (s : (Prod (T) (Nat))) (k : Nat) (__t : (Prod (T) (Nat))) : ((Reasoning.Budget.Burn.apply (T) (s) (k) = Option.some (__t)) -> Reasoning.Budget.Spent (T) (s) (__t)) :=
  (LexLeanReasoning.guarded ((Reasoning.Budget.Spent (T))) (s) (Reasoning.Budget.Burn.guard (T) (s) (k)) (Reasoning.Budget.Burn.conclusion (T) (s) (k)) ((Reasoning.Budget.burn_sound (T)) (s) (k)) (__t))
public theorem Burn.apply_progress (T : Type) (s : (Prod (T) (Nat))) (k : Nat) (__t : (Prod (T) (Nat))) : ((Reasoning.Budget.Burn.apply (T) (s) (k) = Option.some (__t)) -> (Reasoning.Budget.Remaining (T) (__t) < Reasoning.Budget.Remaining (T) (s))) :=
  (LexLeanReasoning.guardedRank ((Reasoning.Budget.Remaining (T))) (s) (Reasoning.Budget.Burn.guard (T) (s) (k)) (Reasoning.Budget.Burn.conclusion (T) (s) (k)) ((Reasoning.Budget.burn_progress (T)) (s) (k)) (__t))

@[expose, reducible] public def Empty (T : Type) (_x : T) (r : Nat) : Prop := (r = 0)

@[expose] public def emptyCheck (T : Type) (_x : T) (r : Nat) : Bool := (Nat.beq (r) (0))

public theorem empty_sound (T : Type) (x : T) (r : Nat) : ((emptyCheck (T) (x) (r) = true) -> Empty (T) (x) (r)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Empty, emptyCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem Drained.sound (T : Type) : (LexLeanReasoning.Sound ((Reasoning.Budget.emptyCheck (T))) ((Reasoning.Budget.Empty (T)))) :=
  (Reasoning.Budget.empty_sound (T))

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
@[expose] public def Spend.fire (T : Type) (__s : (Prod (T) (Nat))) (__step : Reasoning.Budget.Spend.Step (T)) : Option ((Prod (T) (Nat))) := (match __step with | Reasoning.Budget.Spend.Step.Tick => Reasoning.Budget.Tick.apply (T) (__s) | Reasoning.Budget.Spend.Step.Burn __b => Reasoning.Budget.Burn.apply (T) (__s) (__b))
@[expose] public def Spend.replay (T : Type) (__acc : Except ((Prod Bool Bool)) ((Prod (T) (Nat)))) (__step : Reasoning.Budget.Spend.Step (T)) : Except ((Prod Bool Bool)) ((Prod (T) (Nat))) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match Reasoning.Budget.Spend.fire (T) (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def Spend.follow (T : Type) (x : (Prod (T) (Nat))) (__trace : List (Reasoning.Budget.Spend.Step (T))) : Except ((Prod Bool Bool)) ((Prod (T) (Nat))) := (LexLeanCollections.listFold ((Reasoning.Budget.Spend.replay (T))) (Except.ok (Reasoning.Budget.Spend.observe (T) (x))) (__trace) : Except ((Prod Bool Bool)) ((Prod (T) (Nat))))
@[expose] public def Spend.extract (T : Type) (account : (Prod (T) (Nat))) : Option (Nat) := Option.some ((account).2)
@[expose] public def Spend.accept (T : Type) (x : (Prod (T) (Nat))) (__s : (Prod (T) (Nat))) : Option (Nat) := (match Reasoning.Budget.Spend.extract (T) (__s) with | Option.none => Option.none | Option.some __v => (if Reasoning.Budget.emptyCheck ((Prod (T) (Nat))) (x) (__v) then Option.some (__v) else Option.none))
@[expose] public def Spend.answer (T : Type) (x : (Prod (T) (Nat))) (__trace : List (Reasoning.Budget.Spend.Step (T))) : Option (Nat) := (match Reasoning.Budget.Spend.follow (T) (x) (__trace) with | Except.ok __s => Reasoning.Budget.Spend.accept (T) (x) (__s) | Except.error _ => Option.none)
@[expose] public def Spend.select (T : Type) (__s : (Prod (T) (Nat))) : Option (Reasoning.Budget.Spend.Step (T)) := (match (if Reasoning.Budget.Tick.guard (T) (__s) then Option.some (Reasoning.Budget.Spend.Step.Tick) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (LexLeanCollections.listFold ((fun (__acc : Option (Reasoning.Budget.Spend.Step (T))) (__b : Nat) => (match __acc with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Budget.Burn.guard (T) (__s) (__b) then Option.some (Reasoning.Budget.Spend.Step.Burn (__b)) else Option.none)))) (Option.none) (Reasoning.Budget.Burn.candidates (T) (__s)) : Option (Reasoning.Budget.Spend.Step (T))))
@[expose] public def Spend.attempts (T : Type) (__s : (Prod (T) (Nat))) : Nat := ((let __scan0l : (Prod (Bool) (Nat)) := (Reasoning.Budget.Tick.guard (T) (__s), 1); (if (__scan0l).1 then __scan0l else (let __scan0r : (Prod (Bool) (Nat)) := (LexLeanCollections.listFold ((fun (__acc : (Prod (Bool) (Nat))) (__b : Nat) => (if (__acc).1 then __acc else (Reasoning.Budget.Burn.guard (T) (__s) (__b), ((__acc).2 + 1))))) ((false, 0)) (Reasoning.Budget.Burn.candidates (T) (__s)) : (Prod (Bool) (Nat))); ((__scan0r).1, ((__scan0l).2 + (__scan0r).2)))))).2
@[expose] public def Spend.next (T : Type) (__s : (Prod (T) (Nat))) : Option ((Prod (T) (Nat))) := (match Reasoning.Budget.Spend.select (T) (__s) with | Option.none => Option.none | Option.some __step => Reasoning.Budget.Spend.fire (T) (__s) (__step))
@[expose] public def Spend.saturate (T : Type) (x : (Prod (T) (Nat))) : (Prod ((Prod (T) (Nat))) (Bool)) := (LexLeanCollections.iterateUntil ((Reasoning.Budget.Spend.next (T))) (((x).2 + 4)) (Reasoning.Budget.Spend.observe (T) (x)) : (Prod ((Prod (T) (Nat))) (Bool)))
public structure Spend.Run (T : Type) where
  state : (Prod (T) (Nat))
  trace : List (Reasoning.Budget.Spend.Step (T))
  ledger : Reasoning.Budget.Spend.Ledger (T)
@[expose] public def Spend.start (T : Type) (x : (Prod (T) (Nat))) : Reasoning.Budget.Spend.Run (T) := ({ state := Reasoning.Budget.Spend.observe (T) (x), trace := ([] : List (Reasoning.Budget.Spend.Step (T))), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := 0 } : Reasoning.Budget.Spend.Ledger (T)) } : Reasoning.Budget.Spend.Run (T))
@[expose] public def Spend.step (T : Type) (__r : Reasoning.Budget.Spend.Run (T)) : Option (Reasoning.Budget.Spend.Run (T)) := (match Reasoning.Budget.Spend.select (T) ((__r).state) with | Option.none => Option.none | Option.some __step => (match Reasoning.Budget.Spend.fire (T) ((__r).state) (__step) with | Option.none => Option.none | Option.some __t => Option.some (({ state := __t, trace := (LexLeanRuntime.append ((__r).trace) ((__step :: ([] : List (Reasoning.Budget.Spend.Step (T))))) : List (Reasoning.Budget.Spend.Step (T))), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + Reasoning.Budget.Spend.attempts (T) ((__r).state)), firings := (((__r).ledger).firings + 1), expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : Reasoning.Budget.Spend.Ledger (T)) } : Reasoning.Budget.Spend.Run (T)))))
@[expose] public def Spend.run (T : Type) (x : (Prod (T) (Nat))) : (Prod (Reasoning.Budget.Spend.Run (T)) (Bool)) := (LexLeanCollections.iterateUntil ((Reasoning.Budget.Spend.step (T))) (((x).2 + 4)) (Reasoning.Budget.Spend.start (T) (x)) : (Prod (Reasoning.Budget.Spend.Run (T)) (Bool)))
@[expose] public def Spend.account (T : Type) (x : (Prod (T) (Nat))) : Reasoning.Budget.Spend.Ledger (T) := (let __final : (Prod (Reasoning.Budget.Spend.Run (T)) (Bool)) := Reasoning.Budget.Spend.run (T) (x); (match (__final).2 with | Bool.true => ({ iterations := (((__final).1).ledger).iterations, attempts := ((((__final).1).ledger).attempts + Reasoning.Budget.Spend.attempts (T) (((__final).1).state)), firings := (((__final).1).ledger).firings, expansions := (((__final).1).ledger).expansions, verifications := ((((__final).1).ledger).verifications + 1), frontier := (((__final).1).ledger).frontier } : Reasoning.Budget.Spend.Ledger (T)) | Bool.false => ((__final).1).ledger))
@[expose] public def Spend.conclude (T : Type) (x : (Prod (T) (Nat))) (__s : (Prod (T) (Nat))) : Except ((Prod Bool Bool)) (Nat) := (match Reasoning.Budget.Spend.accept (T) (x) (__s) with | Option.some __v => Except.ok (__v) | Option.none => (match Reasoning.Budget.Spend.extract (T) (__s) with | Option.some _ => Except.error (((true, false) : Prod Bool Bool)) | Option.none => Except.error (((false, true) : Prod Bool Bool))))
@[expose] public def Spend.verdict (T : Type) (x : (Prod (T) (Nat))) : Except ((Prod Bool Bool)) (Nat) := (let __final : (Prod ((Prod (T) (Nat))) (Bool)) := Reasoning.Budget.Spend.saturate (T) (x); (match (__final).2 with | Bool.true => Reasoning.Budget.Spend.conclude (T) (x) ((__final).1) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
@[expose] public def Spend (T : Type) (x : (Prod (T) (Nat))) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (Reasoning.Budget.Spend.Step (T))))) := (let __final : (Prod (Reasoning.Budget.Spend.Run (T)) (Bool)) := Reasoning.Budget.Spend.run (T) (x); (match (__final).2 with | Bool.true => (match Reasoning.Budget.Spend.conclude (T) (x) (((__final).1).state) with | Except.ok __v => Except.ok ((__v, ((__final).1).trace)) | Except.error __e => Except.error (__e)) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
public theorem Spend.fire_sound (T : Type) (__s : (Prod (T) (Nat))) (__step : Reasoning.Budget.Spend.Step (T)) (__t : (Prod (T) (Nat))) : ((Reasoning.Budget.Spend.fire (T) (__s) (__step) = Option.some (__t)) -> Reasoning.Budget.Spent (T) (__s) (__t)) :=
by
  cases __step with
  | Tick =>
    exact Reasoning.Budget.Tick.apply_sound (T) __s __t
  | Burn __b =>
    exact Reasoning.Budget.Burn.apply_sound (T) __s __b __t
public theorem Spend.fire_progress (T : Type) (__s : (Prod (T) (Nat))) (__step : Reasoning.Budget.Spend.Step (T)) (__t : (Prod (T) (Nat))) : ((Reasoning.Budget.Spend.fire (T) (__s) (__step) = Option.some (__t)) -> (Reasoning.Budget.Remaining (T) (__t) < Reasoning.Budget.Remaining (T) (__s))) :=
by
  cases __step with
  | Tick =>
    exact Reasoning.Budget.Tick.apply_progress (T) __s __t
  | Burn __b =>
    exact Reasoning.Budget.Burn.apply_progress (T) __s __b __t
public theorem Spend.replay_fire (T : Type) (__s : (Prod (T) (Nat))) (__step : Reasoning.Budget.Spend.Step (T)) (__t : (Prod (T) (Nat))) : ((Reasoning.Budget.Spend.fire (T) (__s) (__step) = Option.some (__t)) -> (Reasoning.Budget.Spend.replay (T) (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Spend.replay]
  rw [llE]
public theorem Spend.replay_sound (T : Type) (x : (Prod (T) (Nat))) (__acc : Except ((Prod Bool Bool)) ((Prod (T) (Nat)))) (__step : Reasoning.Budget.Spend.Step (T)) : ((LexLeanReasoning.Reaches ((Reasoning.Budget.Spent (T))) (Reasoning.Budget.Spend.observe (T) (x)) (__acc)) -> (LexLeanReasoning.Reaches ((Reasoning.Budget.Spent (T))) (Reasoning.Budget.Spend.observe (T) (x)) (Reasoning.Budget.Spend.replay (T) (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [Reasoning.Budget.Spend.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) ((Reasoning.Budget.Spend.fire_sound (T)) llS __step _ ‹_›)
public theorem Spend.derivation (T : Type) (x : (Prod (T) (Nat))) (__trace : List (Reasoning.Budget.Spend.Step (T))) : (LexLeanReasoning.Reaches ((Reasoning.Budget.Spent (T))) (Reasoning.Budget.Spend.observe (T) (x)) (Reasoning.Budget.Spend.follow (T) (x) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((Reasoning.Budget.Spend.replay (T))) (fun (__acc : Except ((Prod Bool Bool)) ((Prod (T) (Nat)))) => (LexLeanReasoning.Reaches ((Reasoning.Budget.Spent (T))) (Reasoning.Budget.Spend.observe (T) (x)) (__acc))) ((Reasoning.Budget.Spend.replay_sound (T)) (x)) (__trace) (Except.ok (Reasoning.Budget.Spend.observe (T) (x))) (LexLeanReasoning.reachesStart ((Reasoning.Budget.Spent (T))) (Reasoning.Budget.Spend.observe (T) (x))))
public theorem Spend.accept_sound (T : Type) (x : (Prod (T) (Nat))) (__s : (Prod (T) (Nat))) (__v : Nat) : ((Reasoning.Budget.Spend.accept (T) (x) (__s) = Option.some (__v)) -> Reasoning.Budget.Empty ((Prod (T) (Nat))) (x) (__v)) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Spend.accept] at llE
  split at llE
  · cases llE
  · have llC := LexLeanReasoning.checked _ _ _ llE
    exact llC.right ▸ Reasoning.Budget.empty_sound ((Prod (T) (Nat))) _ _ llC.left
public theorem Spend.next_sound (T : Type) (__s : (Prod (T) (Nat))) (__t : (Prod (T) (Nat))) : ((Reasoning.Budget.Spend.next (T) (__s) = Option.some (__t)) -> Reasoning.Budget.Spent (T) (__s) (__t)) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Spend.next] at llE
  split at llE
  · cases llE
  · exact (Reasoning.Budget.Spend.fire_sound (T)) __s _ __t llE
public theorem Spend.next_progress (T : Type) (__s : (Prod (T) (Nat))) (__t : (Prod (T) (Nat))) : ((Reasoning.Budget.Spend.next (T) (__s) = Option.some (__t)) -> (Reasoning.Budget.Remaining (T) (__t) < Reasoning.Budget.Remaining (T) (__s))) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Spend.next] at llE
  split at llE
  · cases llE
  · exact (Reasoning.Budget.Spend.fire_progress (T)) __s _ __t llE
public theorem Spend.step_none (T : Type) (__r : Reasoning.Budget.Spend.Run (T)) : ((Reasoning.Budget.Spend.step (T) (__r) = Option.none) -> (Reasoning.Budget.Spend.next (T) ((__r).state) = Option.none)) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Spend.step] at llE
  split at llE
  · rename_i llH
    dsimp only [Reasoning.Budget.Spend.next]
    rw [llH]
  · rename_i llH
    split at llE
    · dsimp only [Reasoning.Budget.Spend.next]
      rw [llH]
      assumption
    · cases llE
public theorem Spend.step_some (T : Type) (__r : Reasoning.Budget.Spend.Run (T)) (__q : Reasoning.Budget.Spend.Run (T)) : ((Reasoning.Budget.Spend.step (T) (__r) = Option.some (__q)) -> (Reasoning.Budget.Spend.next (T) ((__r).state) = Option.some ((__q).state))) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Spend.step] at llE
  split at llE
  · cases llE
  · rename_i llH
    split at llE
    · cases llE
    · cases llE
      dsimp only [Reasoning.Budget.Spend.next]
      rw [llH]
      assumption
public theorem Spend.step_trace (T : Type) (x : (Prod (T) (Nat))) (__r : Reasoning.Budget.Spend.Run (T)) (__q : Reasoning.Budget.Spend.Run (T)) : ((Reasoning.Budget.Spend.step (T) (__r) = Option.some (__q)) -> ((Reasoning.Budget.Spend.follow (T) (x) ((__r).trace) = Except.ok ((__r).state)) -> (Reasoning.Budget.Spend.follow (T) (x) ((__q).trace) = Except.ok ((__q).state)))) :=
by
  intro llE llH
  dsimp only [Reasoning.Budget.Spend.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      dsimp only [Reasoning.Budget.Spend.follow]
      rw [LexLeanReasoning.foldSnoc]
      dsimp only [Reasoning.Budget.Spend.follow] at llH
      rw [llH]
      apply (Reasoning.Budget.Spend.replay_fire (T))
      assumption
public theorem Spend.step_count (T : Type) (__r : Reasoning.Budget.Spend.Run (T)) (__q : Reasoning.Budget.Spend.Run (T)) : ((Reasoning.Budget.Spend.step (T) (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Spend.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Spend.step_fired (T : Type) (__r : Reasoning.Budget.Spend.Run (T)) (__q : Reasoning.Budget.Spend.Run (T)) : ((Reasoning.Budget.Spend.step (T) (__r) = Option.some (__q)) -> (((__q).ledger).firings = (((__r).ledger).firings + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Spend.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Spend.saturate_derivation (T : Type) (x : (Prod (T) (Nat))) : (LexLeanReasoning.Star ((Reasoning.Budget.Spent (T))) (Reasoning.Budget.Spend.observe (T) (x)) ((Reasoning.Budget.Spend.saturate (T) (x)).1)) :=
by
  dsimp only [Reasoning.Budget.Spend.saturate]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Budget.Spend.next (T))) (fun (__s : (Prod (T) (Nat))) => (LexLeanReasoning.Star ((Reasoning.Budget.Spent (T))) (Reasoning.Budget.Spend.observe (T) (x)) (__s))) (fun llA llB llE llH => (LexLeanReasoning.Star.tail _ llA llB llH ((Reasoning.Budget.Spend.next_sound (T)) llA llB llE))) (((x).2 + 4)) (Reasoning.Budget.Spend.observe (T) (x)) (LexLeanReasoning.Star.refl _))
public theorem Spend.run_state (T : Type) (x : (Prod (T) (Nat))) : ((((Reasoning.Budget.Spend.run (T) (x)).1).state = (Reasoning.Budget.Spend.saturate (T) (x)).1) /\ ((Reasoning.Budget.Spend.run (T) (x)).2 = (Reasoning.Budget.Spend.saturate (T) (x)).2)) :=
by
  dsimp only [Reasoning.Budget.Spend.run, Reasoning.Budget.Spend.saturate]
  exact (LexLeanReasoning.iterateUntilSimulate ((Reasoning.Budget.Spend.step (T))) ((Reasoning.Budget.Spend.next (T))) ((fun (__r : Reasoning.Budget.Spend.Run (T)) => (__r).state)) (Reasoning.Budget.Spend.step_none (T)) (Reasoning.Budget.Spend.step_some (T)) (((x).2 + 4)) (Reasoning.Budget.Spend.start (T) (x)))
public theorem Spend.run_trace (T : Type) (x : (Prod (T) (Nat))) : (Reasoning.Budget.Spend.follow (T) (x) (((Reasoning.Budget.Spend.run (T) (x)).1).trace) = Except.ok (((Reasoning.Budget.Spend.run (T) (x)).1).state)) :=
by
  dsimp only [Reasoning.Budget.Spend.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Budget.Spend.step (T))) (fun (__r : Reasoning.Budget.Spend.Run (T)) => (Reasoning.Budget.Spend.follow (T) (x) ((__r).trace) = Except.ok ((__r).state))) ((Reasoning.Budget.Spend.step_trace (T)) (x)) (((x).2 + 4)) (Reasoning.Budget.Spend.start (T) (x)) rfl)
public theorem Spend.iterations_bounded (T : Type) (x : (Prod (T) (Nat))) : ((((Reasoning.Budget.Spend.run (T) (x)).1).ledger).iterations <= ((x).2 + 4)) :=
by
  dsimp only [Reasoning.Budget.Spend.run]
  exact (LexLeanReasoning.iterateUntilCount ((Reasoning.Budget.Spend.step (T))) ((fun (__r : Reasoning.Budget.Spend.Run (T)) => ((__r).ledger).iterations)) (Reasoning.Budget.Spend.step_count (T)) (((x).2 + 4)) (Reasoning.Budget.Spend.start (T) (x)) rfl)
public theorem Spend.firings_bounded (T : Type) (x : (Prod (T) (Nat))) : ((((Reasoning.Budget.Spend.run (T) (x)).1).ledger).firings <= ((x).2 + 4)) :=
by
  dsimp only [Reasoning.Budget.Spend.run]
  exact (LexLeanReasoning.iterateUntilCount ((Reasoning.Budget.Spend.step (T))) ((fun (__r : Reasoning.Budget.Spend.Run (T)) => ((__r).ledger).firings)) (Reasoning.Budget.Spend.step_fired (T)) (((x).2 + 4)) (Reasoning.Budget.Spend.start (T) (x)) rfl)
public theorem Spend.saturates (T : Type) (x : (Prod (T) (Nat))) : ((Reasoning.Budget.Spend.saturate (T) (x)).2 = true) :=
by
  dsimp only [Reasoning.Budget.Spend.saturate]
  exact (LexLeanReasoning.iterateUntilStops ((Reasoning.Budget.Spend.next (T))) (fun (_ : (Prod (T) (Nat))) => True) ((Reasoning.Budget.Remaining (T))) (fun llA llB llE _llH => (And.intro True.intro ((Reasoning.Budget.Spend.next_progress (T)) llA llB llE))) (((x).2 + 4)) (Reasoning.Budget.Spend.observe (T) (x)) True.intro ((Reasoning.Budget.allowance_bound (T)) (x)))
public theorem Spend.conclude_accept (T : Type) (x : (Prod (T) (Nat))) (__s : (Prod (T) (Nat))) (__v : Nat) : ((Reasoning.Budget.Spend.conclude (T) (x) (__s) = Except.ok (__v)) -> (Reasoning.Budget.Spend.accept (T) (x) (__s) = Option.some (__v))) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Spend.conclude] at llE
  split at llE
  · cases llE
    assumption
  · split at llE
    · cases llE
    · cases llE
public theorem Spend.conclude_sound (T : Type) (x : (Prod (T) (Nat))) (__s : (Prod (T) (Nat))) (__v : Nat) : ((Reasoning.Budget.Spend.conclude (T) (x) (__s) = Except.ok (__v)) -> Reasoning.Budget.Empty ((Prod (T) (Nat))) (x) (__v)) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Spend.conclude] at llE
  split at llE
  · cases llE
    exact (Reasoning.Budget.Spend.accept_sound (T)) _ _ _ ‹_›
  · split at llE
    · cases llE
    · cases llE
public theorem Spend.verdict_sound (T : Type) (x : (Prod (T) (Nat))) : (forall (__v : Nat), ((Reasoning.Budget.Spend.verdict (T) (x) = Except.ok (__v)) -> Reasoning.Budget.Empty ((Prod (T) (Nat))) (x) (__v))) :=
by
  intro llV llE
  dsimp only [Reasoning.Budget.Spend.verdict] at llE
  generalize llRun : (Reasoning.Budget.Spend.saturate (T)) x = llR at llE
  split at llE
  · exact (Reasoning.Budget.Spend.conclude_sound (T)) _ _ _ llE
  · cases llE
public theorem Spend.explained (T : Type) (x : (Prod (T) (Nat))) : (forall (__v : Nat), (forall (__trace : List (Reasoning.Budget.Spend.Step (T))), ((Reasoning.Budget.Spend (T) (x) = Except.ok ((__v, __trace))) -> ((Reasoning.Budget.Spend.answer (T) (x) (__trace) = Option.some (__v)) /\ Reasoning.Budget.Empty ((Prod (T) (Nat))) (x) (__v))))) :=
by
  intro llV llT llE
  have llTrace := (Reasoning.Budget.Spend.run_trace (T)) x
  dsimp only [Reasoning.Budget.Spend] at llE
  generalize llRun : (Reasoning.Budget.Spend.run (T)) x = llR at llE llTrace
  split at llE
  · split at llE
    · rename_i llW llH
      cases llE
      exact And.intro (by dsimp only [Reasoning.Budget.Spend.answer]; rw [llTrace]; exact (Reasoning.Budget.Spend.conclude_accept (T)) _ _ _ llH) ((Reasoning.Budget.Spend.conclude_sound (T)) _ _ _ llH)
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
@[expose] public def Refund.fire (__s : (Prod (Nat) (Nat))) (__step : Reasoning.Budget.Refund.Step) : Option ((Prod (Nat) (Nat))) := (match __step with | Reasoning.Budget.Refund.Step.Tick => Reasoning.Budget.Tick.apply (Nat) (__s) | Reasoning.Budget.Refund.Step.Burn __b => Reasoning.Budget.Burn.apply (Nat) (__s) (__b))
@[expose] public def Refund.replay (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : Reasoning.Budget.Refund.Step) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match Reasoning.Budget.Refund.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def Refund.follow (x : Nat) (__trace : List (Reasoning.Budget.Refund.Step)) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (LexLeanCollections.listFold ((Reasoning.Budget.Refund.replay)) (Except.ok (Reasoning.Budget.Refund.observe (x))) (__trace) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))))
@[expose] public def Refund.extract (account : (Prod (Nat) (Nat))) : Option (Nat) := (if (Nat.blt ((account).1) (5)) then Option.some ((account).2) else Option.none)
@[expose] public def Refund.accept (x : Nat) (__s : (Prod (Nat) (Nat))) : Option (Nat) := (match Reasoning.Budget.Refund.extract (__s) with | Option.none => Option.none | Option.some __v => (if Reasoning.Budget.emptyCheck (Nat) (x) (__v) then Option.some (__v) else Option.none))
@[expose] public def Refund.answer (x : Nat) (__trace : List (Reasoning.Budget.Refund.Step)) : Option (Nat) := (match Reasoning.Budget.Refund.follow (x) (__trace) with | Except.ok __s => Reasoning.Budget.Refund.accept (x) (__s) | Except.error _ => Option.none)
@[expose] public def Refund.select (__s : (Prod (Nat) (Nat))) : Option (Reasoning.Budget.Refund.Step) := (match (if Reasoning.Budget.Tick.guard (Nat) (__s) then Option.some (Reasoning.Budget.Refund.Step.Tick) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (LexLeanCollections.listFold ((fun (__acc : Option (Reasoning.Budget.Refund.Step)) (__b : Nat) => (match __acc with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Budget.Burn.guard (Nat) (__s) (__b) then Option.some (Reasoning.Budget.Refund.Step.Burn (__b)) else Option.none)))) (Option.none) (Reasoning.Budget.Burn.candidates (Nat) (__s)) : Option (Reasoning.Budget.Refund.Step)))
@[expose] public def Refund.attempts (__s : (Prod (Nat) (Nat))) : Nat := ((let __scan0l : (Prod (Bool) (Nat)) := (Reasoning.Budget.Tick.guard (Nat) (__s), 1); (if (__scan0l).1 then __scan0l else (let __scan0r : (Prod (Bool) (Nat)) := (LexLeanCollections.listFold ((fun (__acc : (Prod (Bool) (Nat))) (__b : Nat) => (if (__acc).1 then __acc else (Reasoning.Budget.Burn.guard (Nat) (__s) (__b), ((__acc).2 + 1))))) ((false, 0)) (Reasoning.Budget.Burn.candidates (Nat) (__s)) : (Prod (Bool) (Nat))); ((__scan0r).1, ((__scan0l).2 + (__scan0r).2)))))).2
@[expose] public def Refund.next (__s : (Prod (Nat) (Nat))) : Option ((Prod (Nat) (Nat))) := (match Reasoning.Budget.Refund.select (__s) with | Option.none => Option.none | Option.some __step => Reasoning.Budget.Refund.fire (__s) (__step))
@[expose] public def Refund.saturate (x : Nat) : (Prod ((Prod (Nat) (Nat))) (Bool)) := (LexLeanCollections.iterateUntil ((Reasoning.Budget.Refund.next)) ((x + 4)) (Reasoning.Budget.Refund.observe (x)) : (Prod ((Prod (Nat) (Nat))) (Bool)))
public structure Refund.Run where
  state : (Prod (Nat) (Nat))
  trace : List (Reasoning.Budget.Refund.Step)
  ledger : Reasoning.Budget.Refund.Ledger
@[expose] public def Refund.start (x : Nat) : Reasoning.Budget.Refund.Run := ({ state := Reasoning.Budget.Refund.observe (x), trace := ([] : List (Reasoning.Budget.Refund.Step)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := 0 } : Reasoning.Budget.Refund.Ledger) } : Reasoning.Budget.Refund.Run)
@[expose] public def Refund.step (__r : Reasoning.Budget.Refund.Run) : Option (Reasoning.Budget.Refund.Run) := (match Reasoning.Budget.Refund.select ((__r).state) with | Option.none => Option.none | Option.some __step => (match Reasoning.Budget.Refund.fire ((__r).state) (__step) with | Option.none => Option.none | Option.some __t => Option.some (({ state := __t, trace := (LexLeanRuntime.append ((__r).trace) ((__step :: ([] : List (Reasoning.Budget.Refund.Step)))) : List (Reasoning.Budget.Refund.Step)), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + Reasoning.Budget.Refund.attempts ((__r).state)), firings := (((__r).ledger).firings + 1), expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : Reasoning.Budget.Refund.Ledger) } : Reasoning.Budget.Refund.Run))))
@[expose] public def Refund.run (x : Nat) : (Prod (Reasoning.Budget.Refund.Run) (Bool)) := (LexLeanCollections.iterateUntil ((Reasoning.Budget.Refund.step)) ((x + 4)) (Reasoning.Budget.Refund.start (x)) : (Prod (Reasoning.Budget.Refund.Run) (Bool)))
@[expose] public def Refund.account (x : Nat) : Reasoning.Budget.Refund.Ledger := (let __final : (Prod (Reasoning.Budget.Refund.Run) (Bool)) := Reasoning.Budget.Refund.run (x); (match (__final).2 with | Bool.true => ({ iterations := (((__final).1).ledger).iterations, attempts := ((((__final).1).ledger).attempts + Reasoning.Budget.Refund.attempts (((__final).1).state)), firings := (((__final).1).ledger).firings, expansions := (((__final).1).ledger).expansions, verifications := ((((__final).1).ledger).verifications + 1), frontier := (((__final).1).ledger).frontier } : Reasoning.Budget.Refund.Ledger) | Bool.false => ((__final).1).ledger))
@[expose] public def Refund.conclude (x : Nat) (__s : (Prod (Nat) (Nat))) : Except ((Prod Bool Bool)) (Nat) := (match Reasoning.Budget.Refund.accept (x) (__s) with | Option.some __v => Except.ok (__v) | Option.none => (match Reasoning.Budget.Refund.extract (__s) with | Option.some _ => Except.error (((true, false) : Prod Bool Bool)) | Option.none => Except.error (((false, true) : Prod Bool Bool))))
@[expose] public def Refund.verdict (x : Nat) : Except ((Prod Bool Bool)) (Nat) := (let __final : (Prod ((Prod (Nat) (Nat))) (Bool)) := Reasoning.Budget.Refund.saturate (x); (match (__final).2 with | Bool.true => Reasoning.Budget.Refund.conclude (x) ((__final).1) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
@[expose] public def Refund (x : Nat) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (Reasoning.Budget.Refund.Step)))) := (let __final : (Prod (Reasoning.Budget.Refund.Run) (Bool)) := Reasoning.Budget.Refund.run (x); (match (__final).2 with | Bool.true => (match Reasoning.Budget.Refund.conclude (x) (((__final).1).state) with | Except.ok __v => Except.ok ((__v, ((__final).1).trace)) | Except.error __e => Except.error (__e)) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
public theorem Refund.fire_sound (__s : (Prod (Nat) (Nat))) (__step : Reasoning.Budget.Refund.Step) (__t : (Prod (Nat) (Nat))) : ((Reasoning.Budget.Refund.fire (__s) (__step) = Option.some (__t)) -> Reasoning.Budget.Spent (Nat) (__s) (__t)) :=
by
  cases __step with
  | Tick =>
    exact Reasoning.Budget.Tick.apply_sound (Nat) __s __t
  | Burn __b =>
    exact Reasoning.Budget.Burn.apply_sound (Nat) __s __b __t
public theorem Refund.replay_fire (__s : (Prod (Nat) (Nat))) (__step : Reasoning.Budget.Refund.Step) (__t : (Prod (Nat) (Nat))) : ((Reasoning.Budget.Refund.fire (__s) (__step) = Option.some (__t)) -> (Reasoning.Budget.Refund.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Refund.replay]
  rw [llE]
public theorem Refund.replay_sound (x : Nat) (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : Reasoning.Budget.Refund.Step) : ((LexLeanReasoning.Reaches ((Reasoning.Budget.Spent (Nat))) (Reasoning.Budget.Refund.observe (x)) (__acc)) -> (LexLeanReasoning.Reaches ((Reasoning.Budget.Spent (Nat))) (Reasoning.Budget.Refund.observe (x)) (Reasoning.Budget.Refund.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [Reasoning.Budget.Refund.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (Reasoning.Budget.Refund.fire_sound llS __step _ ‹_›)
public theorem Refund.derivation (x : Nat) (__trace : List (Reasoning.Budget.Refund.Step)) : (LexLeanReasoning.Reaches ((Reasoning.Budget.Spent (Nat))) (Reasoning.Budget.Refund.observe (x)) (Reasoning.Budget.Refund.follow (x) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((Reasoning.Budget.Refund.replay)) (fun (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) => (LexLeanReasoning.Reaches ((Reasoning.Budget.Spent (Nat))) (Reasoning.Budget.Refund.observe (x)) (__acc))) (Reasoning.Budget.Refund.replay_sound (x)) (__trace) (Except.ok (Reasoning.Budget.Refund.observe (x))) (LexLeanReasoning.reachesStart ((Reasoning.Budget.Spent (Nat))) (Reasoning.Budget.Refund.observe (x))))
public theorem Refund.accept_sound (x : Nat) (__s : (Prod (Nat) (Nat))) (__v : Nat) : ((Reasoning.Budget.Refund.accept (x) (__s) = Option.some (__v)) -> Reasoning.Budget.Empty (Nat) (x) (__v)) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Refund.accept] at llE
  split at llE
  · cases llE
  · have llC := LexLeanReasoning.checked _ _ _ llE
    exact llC.right ▸ Reasoning.Budget.empty_sound (Nat) _ _ llC.left
public theorem Refund.next_sound (__s : (Prod (Nat) (Nat))) (__t : (Prod (Nat) (Nat))) : ((Reasoning.Budget.Refund.next (__s) = Option.some (__t)) -> Reasoning.Budget.Spent (Nat) (__s) (__t)) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Refund.next] at llE
  split at llE
  · cases llE
  · exact Reasoning.Budget.Refund.fire_sound __s _ __t llE
public theorem Refund.step_none (__r : Reasoning.Budget.Refund.Run) : ((Reasoning.Budget.Refund.step (__r) = Option.none) -> (Reasoning.Budget.Refund.next ((__r).state) = Option.none)) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Refund.step] at llE
  split at llE
  · rename_i llH
    dsimp only [Reasoning.Budget.Refund.next]
    rw [llH]
  · rename_i llH
    split at llE
    · dsimp only [Reasoning.Budget.Refund.next]
      rw [llH]
      assumption
    · cases llE
public theorem Refund.step_some (__r : Reasoning.Budget.Refund.Run) (__q : Reasoning.Budget.Refund.Run) : ((Reasoning.Budget.Refund.step (__r) = Option.some (__q)) -> (Reasoning.Budget.Refund.next ((__r).state) = Option.some ((__q).state))) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Refund.step] at llE
  split at llE
  · cases llE
  · rename_i llH
    split at llE
    · cases llE
    · cases llE
      dsimp only [Reasoning.Budget.Refund.next]
      rw [llH]
      assumption
public theorem Refund.step_trace (x : Nat) (__r : Reasoning.Budget.Refund.Run) (__q : Reasoning.Budget.Refund.Run) : ((Reasoning.Budget.Refund.step (__r) = Option.some (__q)) -> ((Reasoning.Budget.Refund.follow (x) ((__r).trace) = Except.ok ((__r).state)) -> (Reasoning.Budget.Refund.follow (x) ((__q).trace) = Except.ok ((__q).state)))) :=
by
  intro llE llH
  dsimp only [Reasoning.Budget.Refund.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      dsimp only [Reasoning.Budget.Refund.follow]
      rw [LexLeanReasoning.foldSnoc]
      dsimp only [Reasoning.Budget.Refund.follow] at llH
      rw [llH]
      apply Reasoning.Budget.Refund.replay_fire
      assumption
public theorem Refund.step_count (__r : Reasoning.Budget.Refund.Run) (__q : Reasoning.Budget.Refund.Run) : ((Reasoning.Budget.Refund.step (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Refund.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Refund.step_fired (__r : Reasoning.Budget.Refund.Run) (__q : Reasoning.Budget.Refund.Run) : ((Reasoning.Budget.Refund.step (__r) = Option.some (__q)) -> (((__q).ledger).firings = (((__r).ledger).firings + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Refund.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Refund.saturate_derivation (x : Nat) : (LexLeanReasoning.Star ((Reasoning.Budget.Spent (Nat))) (Reasoning.Budget.Refund.observe (x)) ((Reasoning.Budget.Refund.saturate (x)).1)) :=
by
  dsimp only [Reasoning.Budget.Refund.saturate]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Budget.Refund.next)) (fun (__s : (Prod (Nat) (Nat))) => (LexLeanReasoning.Star ((Reasoning.Budget.Spent (Nat))) (Reasoning.Budget.Refund.observe (x)) (__s))) (fun llA llB llE llH => (LexLeanReasoning.Star.tail _ llA llB llH (Reasoning.Budget.Refund.next_sound llA llB llE))) ((x + 4)) (Reasoning.Budget.Refund.observe (x)) (LexLeanReasoning.Star.refl _))
public theorem Refund.run_state (x : Nat) : ((((Reasoning.Budget.Refund.run (x)).1).state = (Reasoning.Budget.Refund.saturate (x)).1) /\ ((Reasoning.Budget.Refund.run (x)).2 = (Reasoning.Budget.Refund.saturate (x)).2)) :=
by
  dsimp only [Reasoning.Budget.Refund.run, Reasoning.Budget.Refund.saturate]
  exact (LexLeanReasoning.iterateUntilSimulate ((Reasoning.Budget.Refund.step)) ((Reasoning.Budget.Refund.next)) ((fun (__r : Reasoning.Budget.Refund.Run) => (__r).state)) Reasoning.Budget.Refund.step_none Reasoning.Budget.Refund.step_some ((x + 4)) (Reasoning.Budget.Refund.start (x)))
public theorem Refund.run_trace (x : Nat) : (Reasoning.Budget.Refund.follow (x) (((Reasoning.Budget.Refund.run (x)).1).trace) = Except.ok (((Reasoning.Budget.Refund.run (x)).1).state)) :=
by
  dsimp only [Reasoning.Budget.Refund.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Budget.Refund.step)) (fun (__r : Reasoning.Budget.Refund.Run) => (Reasoning.Budget.Refund.follow (x) ((__r).trace) = Except.ok ((__r).state))) (Reasoning.Budget.Refund.step_trace (x)) ((x + 4)) (Reasoning.Budget.Refund.start (x)) rfl)
public theorem Refund.iterations_bounded (x : Nat) : ((((Reasoning.Budget.Refund.run (x)).1).ledger).iterations <= (x + 4)) :=
by
  dsimp only [Reasoning.Budget.Refund.run]
  exact (LexLeanReasoning.iterateUntilCount ((Reasoning.Budget.Refund.step)) ((fun (__r : Reasoning.Budget.Refund.Run) => ((__r).ledger).iterations)) Reasoning.Budget.Refund.step_count ((x + 4)) (Reasoning.Budget.Refund.start (x)) rfl)
public theorem Refund.firings_bounded (x : Nat) : ((((Reasoning.Budget.Refund.run (x)).1).ledger).firings <= (x + 4)) :=
by
  dsimp only [Reasoning.Budget.Refund.run]
  exact (LexLeanReasoning.iterateUntilCount ((Reasoning.Budget.Refund.step)) ((fun (__r : Reasoning.Budget.Refund.Run) => ((__r).ledger).firings)) Reasoning.Budget.Refund.step_fired ((x + 4)) (Reasoning.Budget.Refund.start (x)) rfl)
public theorem Refund.conclude_accept (x : Nat) (__s : (Prod (Nat) (Nat))) (__v : Nat) : ((Reasoning.Budget.Refund.conclude (x) (__s) = Except.ok (__v)) -> (Reasoning.Budget.Refund.accept (x) (__s) = Option.some (__v))) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Refund.conclude] at llE
  split at llE
  · cases llE
    assumption
  · split at llE
    · cases llE
    · cases llE
public theorem Refund.conclude_sound (x : Nat) (__s : (Prod (Nat) (Nat))) (__v : Nat) : ((Reasoning.Budget.Refund.conclude (x) (__s) = Except.ok (__v)) -> Reasoning.Budget.Empty (Nat) (x) (__v)) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Refund.conclude] at llE
  split at llE
  · cases llE
    exact Reasoning.Budget.Refund.accept_sound _ _ _ ‹_›
  · split at llE
    · cases llE
    · cases llE
public theorem Refund.verdict_sound (x : Nat) : (forall (__v : Nat), ((Reasoning.Budget.Refund.verdict (x) = Except.ok (__v)) -> Reasoning.Budget.Empty (Nat) (x) (__v))) :=
by
  intro llV llE
  dsimp only [Reasoning.Budget.Refund.verdict] at llE
  generalize llRun : Reasoning.Budget.Refund.saturate x = llR at llE
  split at llE
  · exact Reasoning.Budget.Refund.conclude_sound _ _ _ llE
  · cases llE
public theorem Refund.explained (x : Nat) : (forall (__v : Nat), (forall (__trace : List (Reasoning.Budget.Refund.Step)), ((Reasoning.Budget.Refund (x) = Except.ok ((__v, __trace))) -> ((Reasoning.Budget.Refund.answer (x) (__trace) = Option.some (__v)) /\ Reasoning.Budget.Empty (Nat) (x) (__v))))) :=
by
  intro llV llT llE
  have llTrace := Reasoning.Budget.Refund.run_trace x
  dsimp only [Reasoning.Budget.Refund] at llE
  generalize llRun : Reasoning.Budget.Refund.run x = llR at llE llTrace
  split at llE
  · split at llE
    · rename_i llW llH
      cases llE
      exact And.intro (by dsimp only [Reasoning.Budget.Refund.answer]; rw [llTrace]; exact Reasoning.Budget.Refund.conclude_accept _ _ _ llH) (Reasoning.Budget.Refund.conclude_sound _ _ _ llH)
    · cases llE
  · cases llE

end Reasoning.Budget
