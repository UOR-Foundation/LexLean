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
namespace LexLeanStdReasoning.Worklist

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
@[expose] public def Tick.apply (T : Type) (s : (Prod (T) (Nat))) : Option ((Prod (T) (Nat))) := (if LexLeanStdReasoning.Worklist.Tick.guard (T) (s) then Option.some (LexLeanStdReasoning.Worklist.Tick.conclusion (T) (s)) else Option.none)
public theorem Tick.apply_sound (T : Type) (s : (Prod (T) (Nat))) (__t : (Prod (T) (Nat))) : ((LexLeanStdReasoning.Worklist.Tick.apply (T) (s) = Option.some (__t)) -> LexLeanStdReasoning.Worklist.Spent (T) (s) (__t)) :=
  (LexLeanReasoning.guarded ((LexLeanStdReasoning.Worklist.Spent (T))) (s) (LexLeanStdReasoning.Worklist.Tick.guard (T) (s)) (LexLeanStdReasoning.Worklist.Tick.conclusion (T) (s)) ((LexLeanStdReasoning.Worklist.tick_sound (T)) (s)) (__t))
public theorem Tick.apply_progress (T : Type) (s : (Prod (T) (Nat))) (__t : (Prod (T) (Nat))) : ((LexLeanStdReasoning.Worklist.Tick.apply (T) (s) = Option.some (__t)) -> (LexLeanStdReasoning.Worklist.Remaining (T) (__t) < LexLeanStdReasoning.Worklist.Remaining (T) (s))) :=
  (LexLeanReasoning.guardedRank ((LexLeanStdReasoning.Worklist.Remaining (T))) (s) (LexLeanStdReasoning.Worklist.Tick.guard (T) (s)) (LexLeanStdReasoning.Worklist.Tick.conclusion (T) (s)) ((LexLeanStdReasoning.Worklist.tick_progress (T)) (s)) (__t))

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
@[expose] public def Burn.apply (T : Type) (s : (Prod (T) (Nat))) (k : Nat) : Option ((Prod (T) (Nat))) := (if LexLeanStdReasoning.Worklist.Burn.guard (T) (s) (k) then Option.some (LexLeanStdReasoning.Worklist.Burn.conclusion (T) (s) (k)) else Option.none)
public theorem Burn.apply_sound (T : Type) (s : (Prod (T) (Nat))) (k : Nat) (__t : (Prod (T) (Nat))) : ((LexLeanStdReasoning.Worklist.Burn.apply (T) (s) (k) = Option.some (__t)) -> LexLeanStdReasoning.Worklist.Spent (T) (s) (__t)) :=
  (LexLeanReasoning.guarded ((LexLeanStdReasoning.Worklist.Spent (T))) (s) (LexLeanStdReasoning.Worklist.Burn.guard (T) (s) (k)) (LexLeanStdReasoning.Worklist.Burn.conclusion (T) (s) (k)) ((LexLeanStdReasoning.Worklist.burn_sound (T)) (s) (k)) (__t))
public theorem Burn.apply_progress (T : Type) (s : (Prod (T) (Nat))) (k : Nat) (__t : (Prod (T) (Nat))) : ((LexLeanStdReasoning.Worklist.Burn.apply (T) (s) (k) = Option.some (__t)) -> (LexLeanStdReasoning.Worklist.Remaining (T) (__t) < LexLeanStdReasoning.Worklist.Remaining (T) (s))) :=
  (LexLeanReasoning.guardedRank ((LexLeanStdReasoning.Worklist.Remaining (T))) (s) (LexLeanStdReasoning.Worklist.Burn.guard (T) (s) (k)) (LexLeanStdReasoning.Worklist.Burn.conclusion (T) (s) (k)) ((LexLeanStdReasoning.Worklist.burn_progress (T)) (s) (k)) (__t))

@[expose, reducible] public def Empty (T : Type) (_x : T) (r : Nat) : Prop := (r = 0)

@[expose] public def emptyCheck (T : Type) (_x : T) (r : Nat) : Bool := (Nat.beq (r) (0))

public theorem empty_sound (T : Type) (x : T) (r : Nat) : ((emptyCheck (T) (x) (r) = true) -> Empty (T) (x) (r)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Empty, emptyCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem Drained.sound (T : Type) : (LexLeanReasoning.Sound ((LexLeanStdReasoning.Worklist.emptyCheck (T))) ((LexLeanStdReasoning.Worklist.Empty (T)))) :=
  (LexLeanStdReasoning.Worklist.empty_sound (T))

public theorem drain_bound (T : Type) (x : (Prod (T) (Nat))) : (Remaining (T) (x) < ((x).2 + 1)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Remaining, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public inductive Drain.Step (T : Type) where
  | Tick
  | Burn (_ : Nat)
public structure Drain.Ledger (T : Type) where
  iterations : Nat
  attempts : Nat
  firings : Nat
  expansions : Nat
  verifications : Nat
  frontier : Nat
@[expose] public def Drain.observe (T : Type) (x : (Prod (T) (Nat))) : (Prod (T) (Nat)) := x
@[expose] public def Drain.fire (T : Type) (__s : (Prod (T) (Nat))) (__step : LexLeanStdReasoning.Worklist.Drain.Step (T)) : Option ((Prod (T) (Nat))) := (match __step with | LexLeanStdReasoning.Worklist.Drain.Step.Tick => LexLeanStdReasoning.Worklist.Tick.apply (T) (__s) | LexLeanStdReasoning.Worklist.Drain.Step.Burn __b => LexLeanStdReasoning.Worklist.Burn.apply (T) (__s) (__b))
@[expose] public def Drain.replay (T : Type) (__acc : Except ((Prod Bool Bool)) ((Prod (T) (Nat)))) (__step : LexLeanStdReasoning.Worklist.Drain.Step (T)) : Except ((Prod Bool Bool)) ((Prod (T) (Nat))) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match LexLeanStdReasoning.Worklist.Drain.fire (T) (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def Drain.follow (T : Type) (x : (Prod (T) (Nat))) (__trace : List (LexLeanStdReasoning.Worklist.Drain.Step (T))) : Except ((Prod Bool Bool)) ((Prod (T) (Nat))) := (LexLeanCollections.listFold ((LexLeanStdReasoning.Worklist.Drain.replay (T))) (Except.ok (LexLeanStdReasoning.Worklist.Drain.observe (T) (x))) (__trace) : Except ((Prod Bool Bool)) ((Prod (T) (Nat))))
@[expose] public def Drain.extract (T : Type) (account : (Prod (T) (Nat))) : Option (Nat) := Option.some ((account).2)
@[expose] public def Drain.accept (T : Type) (x : (Prod (T) (Nat))) (__s : (Prod (T) (Nat))) : Option (Nat) := (match LexLeanStdReasoning.Worklist.Drain.extract (T) (__s) with | Option.none => Option.none | Option.some __v => (if LexLeanStdReasoning.Worklist.emptyCheck ((Prod (T) (Nat))) (x) (__v) then Option.some (__v) else Option.none))
@[expose] public def Drain.answer (T : Type) (x : (Prod (T) (Nat))) (__trace : List (LexLeanStdReasoning.Worklist.Drain.Step (T))) : Option (Nat) := (match LexLeanStdReasoning.Worklist.Drain.follow (T) (x) (__trace) with | Except.ok __s => LexLeanStdReasoning.Worklist.Drain.accept (T) (x) (__s) | Except.error _ => Option.none)
@[expose] public def Drain.select (T : Type) (__s : (Prod (T) (Nat))) : Option (LexLeanStdReasoning.Worklist.Drain.Step (T)) := (match (if LexLeanStdReasoning.Worklist.Tick.guard (T) (__s) then Option.some (LexLeanStdReasoning.Worklist.Drain.Step.Tick) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (LexLeanCollections.listFold ((fun (__acc : Option (LexLeanStdReasoning.Worklist.Drain.Step (T))) (__b : Nat) => (match __acc with | Option.some __found => Option.some (__found) | Option.none => (if LexLeanStdReasoning.Worklist.Burn.guard (T) (__s) (__b) then Option.some (LexLeanStdReasoning.Worklist.Drain.Step.Burn (__b)) else Option.none)))) (Option.none) (LexLeanStdReasoning.Worklist.Burn.candidates (T) (__s)) : Option (LexLeanStdReasoning.Worklist.Drain.Step (T))))
@[expose] public def Drain.attempts (T : Type) (__s : (Prod (T) (Nat))) : Nat := ((let __scan0l : (Prod (Bool) (Nat)) := (LexLeanStdReasoning.Worklist.Tick.guard (T) (__s), 1); (if (__scan0l).1 then __scan0l else (let __scan0r : (Prod (Bool) (Nat)) := (LexLeanCollections.listFold ((fun (__acc : (Prod (Bool) (Nat))) (__b : Nat) => (if (__acc).1 then __acc else (LexLeanStdReasoning.Worklist.Burn.guard (T) (__s) (__b), ((__acc).2 + 1))))) ((false, 0)) (LexLeanStdReasoning.Worklist.Burn.candidates (T) (__s)) : (Prod (Bool) (Nat))); ((__scan0r).1, ((__scan0l).2 + (__scan0r).2)))))).2
@[expose] public def Drain.next (T : Type) (__s : (Prod (T) (Nat))) : Option ((Prod (T) (Nat))) := (match LexLeanStdReasoning.Worklist.Drain.select (T) (__s) with | Option.none => Option.none | Option.some __step => LexLeanStdReasoning.Worklist.Drain.fire (T) (__s) (__step))
@[expose] public def Drain.saturate (T : Type) (x : (Prod (T) (Nat))) : (Prod ((Prod (T) (Nat))) (Bool)) := (LexLeanCollections.iterateUntil ((LexLeanStdReasoning.Worklist.Drain.next (T))) (((x).2 + 1)) (LexLeanStdReasoning.Worklist.Drain.observe (T) (x)) : (Prod ((Prod (T) (Nat))) (Bool)))
public structure Drain.Run (T : Type) where
  state : (Prod (T) (Nat))
  trace : List (LexLeanStdReasoning.Worklist.Drain.Step (T))
  ledger : LexLeanStdReasoning.Worklist.Drain.Ledger (T)
@[expose] public def Drain.start (T : Type) (x : (Prod (T) (Nat))) : LexLeanStdReasoning.Worklist.Drain.Run (T) := ({ state := LexLeanStdReasoning.Worklist.Drain.observe (T) (x), trace := ([] : List (LexLeanStdReasoning.Worklist.Drain.Step (T))), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := 0 } : LexLeanStdReasoning.Worklist.Drain.Ledger (T)) } : LexLeanStdReasoning.Worklist.Drain.Run (T))
@[expose] public def Drain.step (T : Type) (__r : LexLeanStdReasoning.Worklist.Drain.Run (T)) : Option (LexLeanStdReasoning.Worklist.Drain.Run (T)) := (match LexLeanStdReasoning.Worklist.Drain.select (T) ((__r).state) with | Option.none => Option.none | Option.some __step => (match LexLeanStdReasoning.Worklist.Drain.fire (T) ((__r).state) (__step) with | Option.none => Option.none | Option.some __t => Option.some (({ state := __t, trace := (LexLeanRuntime.append ((__r).trace) ((__step :: ([] : List (LexLeanStdReasoning.Worklist.Drain.Step (T))))) : List (LexLeanStdReasoning.Worklist.Drain.Step (T))), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + LexLeanStdReasoning.Worklist.Drain.attempts (T) ((__r).state)), firings := (((__r).ledger).firings + 1), expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : LexLeanStdReasoning.Worklist.Drain.Ledger (T)) } : LexLeanStdReasoning.Worklist.Drain.Run (T)))))
@[expose] public def Drain.run (T : Type) (x : (Prod (T) (Nat))) : (Prod (LexLeanStdReasoning.Worklist.Drain.Run (T)) (Bool)) := (LexLeanCollections.iterateUntil ((LexLeanStdReasoning.Worklist.Drain.step (T))) (((x).2 + 1)) (LexLeanStdReasoning.Worklist.Drain.start (T) (x)) : (Prod (LexLeanStdReasoning.Worklist.Drain.Run (T)) (Bool)))
@[expose] public def Drain.account (T : Type) (x : (Prod (T) (Nat))) : LexLeanStdReasoning.Worklist.Drain.Ledger (T) := (let __final : (Prod (LexLeanStdReasoning.Worklist.Drain.Run (T)) (Bool)) := LexLeanStdReasoning.Worklist.Drain.run (T) (x); (match (__final).2 with | Bool.true => ({ iterations := (((__final).1).ledger).iterations, attempts := ((((__final).1).ledger).attempts + LexLeanStdReasoning.Worklist.Drain.attempts (T) (((__final).1).state)), firings := (((__final).1).ledger).firings, expansions := (((__final).1).ledger).expansions, verifications := ((((__final).1).ledger).verifications + 1), frontier := (((__final).1).ledger).frontier } : LexLeanStdReasoning.Worklist.Drain.Ledger (T)) | Bool.false => ((__final).1).ledger))
@[expose] public def Drain.conclude (T : Type) (x : (Prod (T) (Nat))) (__s : (Prod (T) (Nat))) : Except ((Prod Bool Bool)) (Nat) := (match LexLeanStdReasoning.Worklist.Drain.accept (T) (x) (__s) with | Option.some __v => Except.ok (__v) | Option.none => (match LexLeanStdReasoning.Worklist.Drain.extract (T) (__s) with | Option.some _ => Except.error (((true, false) : Prod Bool Bool)) | Option.none => Except.error (((false, true) : Prod Bool Bool))))
@[expose] public def Drain.verdict (T : Type) (x : (Prod (T) (Nat))) : Except ((Prod Bool Bool)) (Nat) := (let __final : (Prod ((Prod (T) (Nat))) (Bool)) := LexLeanStdReasoning.Worklist.Drain.saturate (T) (x); (match (__final).2 with | Bool.true => LexLeanStdReasoning.Worklist.Drain.conclude (T) (x) ((__final).1) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
@[expose] public def Drain (T : Type) (x : (Prod (T) (Nat))) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (LexLeanStdReasoning.Worklist.Drain.Step (T))))) := (let __final : (Prod (LexLeanStdReasoning.Worklist.Drain.Run (T)) (Bool)) := LexLeanStdReasoning.Worklist.Drain.run (T) (x); (match (__final).2 with | Bool.true => (match LexLeanStdReasoning.Worklist.Drain.conclude (T) (x) (((__final).1).state) with | Except.ok __v => Except.ok ((__v, ((__final).1).trace)) | Except.error __e => Except.error (__e)) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
public theorem Drain.fire_sound (T : Type) (__s : (Prod (T) (Nat))) (__step : LexLeanStdReasoning.Worklist.Drain.Step (T)) (__t : (Prod (T) (Nat))) : ((LexLeanStdReasoning.Worklist.Drain.fire (T) (__s) (__step) = Option.some (__t)) -> LexLeanStdReasoning.Worklist.Spent (T) (__s) (__t)) :=
by
  cases __step with
  | Tick =>
    exact LexLeanStdReasoning.Worklist.Tick.apply_sound (T) __s __t
  | Burn __b =>
    exact LexLeanStdReasoning.Worklist.Burn.apply_sound (T) __s __b __t
public theorem Drain.fire_progress (T : Type) (__s : (Prod (T) (Nat))) (__step : LexLeanStdReasoning.Worklist.Drain.Step (T)) (__t : (Prod (T) (Nat))) : ((LexLeanStdReasoning.Worklist.Drain.fire (T) (__s) (__step) = Option.some (__t)) -> (LexLeanStdReasoning.Worklist.Remaining (T) (__t) < LexLeanStdReasoning.Worklist.Remaining (T) (__s))) :=
by
  cases __step with
  | Tick =>
    exact LexLeanStdReasoning.Worklist.Tick.apply_progress (T) __s __t
  | Burn __b =>
    exact LexLeanStdReasoning.Worklist.Burn.apply_progress (T) __s __b __t
public theorem Drain.replay_fire (T : Type) (__s : (Prod (T) (Nat))) (__step : LexLeanStdReasoning.Worklist.Drain.Step (T)) (__t : (Prod (T) (Nat))) : ((LexLeanStdReasoning.Worklist.Drain.fire (T) (__s) (__step) = Option.some (__t)) -> (LexLeanStdReasoning.Worklist.Drain.replay (T) (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [LexLeanStdReasoning.Worklist.Drain.replay]
  rw [llE]
public theorem Drain.replay_sound (T : Type) (x : (Prod (T) (Nat))) (__acc : Except ((Prod Bool Bool)) ((Prod (T) (Nat)))) (__step : LexLeanStdReasoning.Worklist.Drain.Step (T)) : ((LexLeanReasoning.Reaches ((LexLeanStdReasoning.Worklist.Spent (T))) (LexLeanStdReasoning.Worklist.Drain.observe (T) (x)) (__acc)) -> (LexLeanReasoning.Reaches ((LexLeanStdReasoning.Worklist.Spent (T))) (LexLeanStdReasoning.Worklist.Drain.observe (T) (x)) (LexLeanStdReasoning.Worklist.Drain.replay (T) (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [LexLeanStdReasoning.Worklist.Drain.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) ((LexLeanStdReasoning.Worklist.Drain.fire_sound (T)) llS __step _ ‹_›)
public theorem Drain.derivation (T : Type) (x : (Prod (T) (Nat))) (__trace : List (LexLeanStdReasoning.Worklist.Drain.Step (T))) : (LexLeanReasoning.Reaches ((LexLeanStdReasoning.Worklist.Spent (T))) (LexLeanStdReasoning.Worklist.Drain.observe (T) (x)) (LexLeanStdReasoning.Worklist.Drain.follow (T) (x) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((LexLeanStdReasoning.Worklist.Drain.replay (T))) (fun (__acc : Except ((Prod Bool Bool)) ((Prod (T) (Nat)))) => (LexLeanReasoning.Reaches ((LexLeanStdReasoning.Worklist.Spent (T))) (LexLeanStdReasoning.Worklist.Drain.observe (T) (x)) (__acc))) ((LexLeanStdReasoning.Worklist.Drain.replay_sound (T)) (x)) (__trace) (Except.ok (LexLeanStdReasoning.Worklist.Drain.observe (T) (x))) (LexLeanReasoning.reachesStart ((LexLeanStdReasoning.Worklist.Spent (T))) (LexLeanStdReasoning.Worklist.Drain.observe (T) (x))))
public theorem Drain.accept_sound (T : Type) (x : (Prod (T) (Nat))) (__s : (Prod (T) (Nat))) (__v : Nat) : ((LexLeanStdReasoning.Worklist.Drain.accept (T) (x) (__s) = Option.some (__v)) -> LexLeanStdReasoning.Worklist.Empty ((Prod (T) (Nat))) (x) (__v)) :=
by
  intro llE
  dsimp only [LexLeanStdReasoning.Worklist.Drain.accept] at llE
  split at llE
  · cases llE
  · have llC := LexLeanReasoning.checked _ _ _ llE
    exact llC.right ▸ LexLeanStdReasoning.Worklist.empty_sound ((Prod (T) (Nat))) _ _ llC.left
public theorem Drain.next_sound (T : Type) (__s : (Prod (T) (Nat))) (__t : (Prod (T) (Nat))) : ((LexLeanStdReasoning.Worklist.Drain.next (T) (__s) = Option.some (__t)) -> LexLeanStdReasoning.Worklist.Spent (T) (__s) (__t)) :=
by
  intro llE
  dsimp only [LexLeanStdReasoning.Worklist.Drain.next] at llE
  split at llE
  · cases llE
  · exact (LexLeanStdReasoning.Worklist.Drain.fire_sound (T)) __s _ __t llE
public theorem Drain.next_progress (T : Type) (__s : (Prod (T) (Nat))) (__t : (Prod (T) (Nat))) : ((LexLeanStdReasoning.Worklist.Drain.next (T) (__s) = Option.some (__t)) -> (LexLeanStdReasoning.Worklist.Remaining (T) (__t) < LexLeanStdReasoning.Worklist.Remaining (T) (__s))) :=
by
  intro llE
  dsimp only [LexLeanStdReasoning.Worklist.Drain.next] at llE
  split at llE
  · cases llE
  · exact (LexLeanStdReasoning.Worklist.Drain.fire_progress (T)) __s _ __t llE
public theorem Drain.step_none (T : Type) (__r : LexLeanStdReasoning.Worklist.Drain.Run (T)) : ((LexLeanStdReasoning.Worklist.Drain.step (T) (__r) = Option.none) -> (LexLeanStdReasoning.Worklist.Drain.next (T) ((__r).state) = Option.none)) :=
by
  intro llE
  dsimp only [LexLeanStdReasoning.Worklist.Drain.step] at llE
  split at llE
  · rename_i llH
    dsimp only [LexLeanStdReasoning.Worklist.Drain.next]
    rw [llH]
  · rename_i llH
    split at llE
    · dsimp only [LexLeanStdReasoning.Worklist.Drain.next]
      rw [llH]
      assumption
    · cases llE
public theorem Drain.step_some (T : Type) (__r : LexLeanStdReasoning.Worklist.Drain.Run (T)) (__q : LexLeanStdReasoning.Worklist.Drain.Run (T)) : ((LexLeanStdReasoning.Worklist.Drain.step (T) (__r) = Option.some (__q)) -> (LexLeanStdReasoning.Worklist.Drain.next (T) ((__r).state) = Option.some ((__q).state))) :=
by
  intro llE
  dsimp only [LexLeanStdReasoning.Worklist.Drain.step] at llE
  split at llE
  · cases llE
  · rename_i llH
    split at llE
    · cases llE
    · cases llE
      dsimp only [LexLeanStdReasoning.Worklist.Drain.next]
      rw [llH]
      assumption
public theorem Drain.step_trace (T : Type) (x : (Prod (T) (Nat))) (__r : LexLeanStdReasoning.Worklist.Drain.Run (T)) (__q : LexLeanStdReasoning.Worklist.Drain.Run (T)) : ((LexLeanStdReasoning.Worklist.Drain.step (T) (__r) = Option.some (__q)) -> ((LexLeanStdReasoning.Worklist.Drain.follow (T) (x) ((__r).trace) = Except.ok ((__r).state)) -> (LexLeanStdReasoning.Worklist.Drain.follow (T) (x) ((__q).trace) = Except.ok ((__q).state)))) :=
by
  intro llE llH
  dsimp only [LexLeanStdReasoning.Worklist.Drain.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      dsimp only [LexLeanStdReasoning.Worklist.Drain.follow]
      rw [LexLeanReasoning.foldSnoc]
      dsimp only [LexLeanStdReasoning.Worklist.Drain.follow] at llH
      rw [llH]
      apply (LexLeanStdReasoning.Worklist.Drain.replay_fire (T))
      assumption
public theorem Drain.step_count (T : Type) (__r : LexLeanStdReasoning.Worklist.Drain.Run (T)) (__q : LexLeanStdReasoning.Worklist.Drain.Run (T)) : ((LexLeanStdReasoning.Worklist.Drain.step (T) (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [LexLeanStdReasoning.Worklist.Drain.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Drain.step_fired (T : Type) (__r : LexLeanStdReasoning.Worklist.Drain.Run (T)) (__q : LexLeanStdReasoning.Worklist.Drain.Run (T)) : ((LexLeanStdReasoning.Worklist.Drain.step (T) (__r) = Option.some (__q)) -> (((__q).ledger).firings = (((__r).ledger).firings + 1))) :=
by
  intro llE
  dsimp only [LexLeanStdReasoning.Worklist.Drain.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Drain.saturate_derivation (T : Type) (x : (Prod (T) (Nat))) : (LexLeanReasoning.Star ((LexLeanStdReasoning.Worklist.Spent (T))) (LexLeanStdReasoning.Worklist.Drain.observe (T) (x)) ((LexLeanStdReasoning.Worklist.Drain.saturate (T) (x)).1)) :=
by
  dsimp only [LexLeanStdReasoning.Worklist.Drain.saturate]
  exact (LexLeanReasoning.iterateUntilInvariant ((LexLeanStdReasoning.Worklist.Drain.next (T))) (fun (__s : (Prod (T) (Nat))) => (LexLeanReasoning.Star ((LexLeanStdReasoning.Worklist.Spent (T))) (LexLeanStdReasoning.Worklist.Drain.observe (T) (x)) (__s))) (fun llA llB llE llH => (LexLeanReasoning.Star.tail _ llA llB llH ((LexLeanStdReasoning.Worklist.Drain.next_sound (T)) llA llB llE))) (((x).2 + 1)) (LexLeanStdReasoning.Worklist.Drain.observe (T) (x)) (LexLeanReasoning.Star.refl _))
public theorem Drain.run_state (T : Type) (x : (Prod (T) (Nat))) : ((((LexLeanStdReasoning.Worklist.Drain.run (T) (x)).1).state = (LexLeanStdReasoning.Worklist.Drain.saturate (T) (x)).1) /\ ((LexLeanStdReasoning.Worklist.Drain.run (T) (x)).2 = (LexLeanStdReasoning.Worklist.Drain.saturate (T) (x)).2)) :=
by
  dsimp only [LexLeanStdReasoning.Worklist.Drain.run, LexLeanStdReasoning.Worklist.Drain.saturate]
  exact (LexLeanReasoning.iterateUntilSimulate ((LexLeanStdReasoning.Worklist.Drain.step (T))) ((LexLeanStdReasoning.Worklist.Drain.next (T))) ((fun (__r : LexLeanStdReasoning.Worklist.Drain.Run (T)) => (__r).state)) (LexLeanStdReasoning.Worklist.Drain.step_none (T)) (LexLeanStdReasoning.Worklist.Drain.step_some (T)) (((x).2 + 1)) (LexLeanStdReasoning.Worklist.Drain.start (T) (x)))
public theorem Drain.run_trace (T : Type) (x : (Prod (T) (Nat))) : (LexLeanStdReasoning.Worklist.Drain.follow (T) (x) (((LexLeanStdReasoning.Worklist.Drain.run (T) (x)).1).trace) = Except.ok (((LexLeanStdReasoning.Worklist.Drain.run (T) (x)).1).state)) :=
by
  dsimp only [LexLeanStdReasoning.Worklist.Drain.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((LexLeanStdReasoning.Worklist.Drain.step (T))) (fun (__r : LexLeanStdReasoning.Worklist.Drain.Run (T)) => (LexLeanStdReasoning.Worklist.Drain.follow (T) (x) ((__r).trace) = Except.ok ((__r).state))) ((LexLeanStdReasoning.Worklist.Drain.step_trace (T)) (x)) (((x).2 + 1)) (LexLeanStdReasoning.Worklist.Drain.start (T) (x)) rfl)
public theorem Drain.iterations_bounded (T : Type) (x : (Prod (T) (Nat))) : ((((LexLeanStdReasoning.Worklist.Drain.run (T) (x)).1).ledger).iterations <= ((x).2 + 1)) :=
by
  dsimp only [LexLeanStdReasoning.Worklist.Drain.run]
  exact (LexLeanReasoning.iterateUntilCount ((LexLeanStdReasoning.Worklist.Drain.step (T))) ((fun (__r : LexLeanStdReasoning.Worklist.Drain.Run (T)) => ((__r).ledger).iterations)) (LexLeanStdReasoning.Worklist.Drain.step_count (T)) (((x).2 + 1)) (LexLeanStdReasoning.Worklist.Drain.start (T) (x)) rfl)
public theorem Drain.firings_bounded (T : Type) (x : (Prod (T) (Nat))) : ((((LexLeanStdReasoning.Worklist.Drain.run (T) (x)).1).ledger).firings <= ((x).2 + 1)) :=
by
  dsimp only [LexLeanStdReasoning.Worklist.Drain.run]
  exact (LexLeanReasoning.iterateUntilCount ((LexLeanStdReasoning.Worklist.Drain.step (T))) ((fun (__r : LexLeanStdReasoning.Worklist.Drain.Run (T)) => ((__r).ledger).firings)) (LexLeanStdReasoning.Worklist.Drain.step_fired (T)) (((x).2 + 1)) (LexLeanStdReasoning.Worklist.Drain.start (T) (x)) rfl)
public theorem Drain.saturates (T : Type) (x : (Prod (T) (Nat))) : ((LexLeanStdReasoning.Worklist.Drain.saturate (T) (x)).2 = true) :=
by
  dsimp only [LexLeanStdReasoning.Worklist.Drain.saturate]
  exact (LexLeanReasoning.iterateUntilStops ((LexLeanStdReasoning.Worklist.Drain.next (T))) (fun (_ : (Prod (T) (Nat))) => True) ((LexLeanStdReasoning.Worklist.Remaining (T))) (fun llA llB llE _llH => (And.intro True.intro ((LexLeanStdReasoning.Worklist.Drain.next_progress (T)) llA llB llE))) (((x).2 + 1)) (LexLeanStdReasoning.Worklist.Drain.observe (T) (x)) True.intro ((LexLeanStdReasoning.Worklist.drain_bound (T)) (x)))
public theorem Drain.conclude_accept (T : Type) (x : (Prod (T) (Nat))) (__s : (Prod (T) (Nat))) (__v : Nat) : ((LexLeanStdReasoning.Worklist.Drain.conclude (T) (x) (__s) = Except.ok (__v)) -> (LexLeanStdReasoning.Worklist.Drain.accept (T) (x) (__s) = Option.some (__v))) :=
by
  intro llE
  dsimp only [LexLeanStdReasoning.Worklist.Drain.conclude] at llE
  split at llE
  · cases llE
    assumption
  · split at llE
    · cases llE
    · cases llE
public theorem Drain.conclude_sound (T : Type) (x : (Prod (T) (Nat))) (__s : (Prod (T) (Nat))) (__v : Nat) : ((LexLeanStdReasoning.Worklist.Drain.conclude (T) (x) (__s) = Except.ok (__v)) -> LexLeanStdReasoning.Worklist.Empty ((Prod (T) (Nat))) (x) (__v)) :=
by
  intro llE
  dsimp only [LexLeanStdReasoning.Worklist.Drain.conclude] at llE
  split at llE
  · cases llE
    exact (LexLeanStdReasoning.Worklist.Drain.accept_sound (T)) _ _ _ ‹_›
  · split at llE
    · cases llE
    · cases llE
public theorem Drain.verdict_sound (T : Type) (x : (Prod (T) (Nat))) : (forall (__v : Nat), ((LexLeanStdReasoning.Worklist.Drain.verdict (T) (x) = Except.ok (__v)) -> LexLeanStdReasoning.Worklist.Empty ((Prod (T) (Nat))) (x) (__v))) :=
by
  intro llV llE
  dsimp only [LexLeanStdReasoning.Worklist.Drain.verdict] at llE
  generalize llRun : (LexLeanStdReasoning.Worklist.Drain.saturate (T)) x = llR at llE
  split at llE
  · exact (LexLeanStdReasoning.Worklist.Drain.conclude_sound (T)) _ _ _ llE
  · cases llE
public theorem Drain.explained (T : Type) (x : (Prod (T) (Nat))) : (forall (__v : Nat), (forall (__trace : List (LexLeanStdReasoning.Worklist.Drain.Step (T))), ((LexLeanStdReasoning.Worklist.Drain (T) (x) = Except.ok ((__v, __trace))) -> ((LexLeanStdReasoning.Worklist.Drain.answer (T) (x) (__trace) = Option.some (__v)) /\ LexLeanStdReasoning.Worklist.Empty ((Prod (T) (Nat))) (x) (__v))))) :=
by
  intro llV llT llE
  have llTrace := (LexLeanStdReasoning.Worklist.Drain.run_trace (T)) x
  dsimp only [LexLeanStdReasoning.Worklist.Drain] at llE
  generalize llRun : (LexLeanStdReasoning.Worklist.Drain.run (T)) x = llR at llE llTrace
  split at llE
  · split at llE
    · rename_i llW llH
      cases llE
      exact And.intro (by dsimp only [LexLeanStdReasoning.Worklist.Drain.answer]; rw [llTrace]; exact (LexLeanStdReasoning.Worklist.Drain.conclude_accept (T)) _ _ _ llH) ((LexLeanStdReasoning.Worklist.Drain.conclude_sound (T)) _ _ _ llH)
    · cases llE
  · cases llE

public theorem drain_clears_three : (Drain.verdict (Nat) ((7, 3)) = Except.ok (0)) := by
  rfl

public theorem drain_steps_three : ((match Drain (Nat) ((7, 3)) with | Except.ok explained => (LexLeanRuntime.length ((explained).2) : Nat) | Except.error _ => 99) = 2) := by
  rfl

end LexLeanStdReasoning.Worklist
