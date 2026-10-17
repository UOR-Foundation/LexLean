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
namespace LexLeanTarget.ReasoningOracle

public structure Vitals where
  temperature : Nat
  heartRate : Nat
  respiratoryRate : Nat
  whiteCells : Nat
  systolic : Nat
  infection : Nat

public structure Chart where
  vitals : Vitals
  fever : Nat
  tachycardia : Nat
  tachypnea : Nat
  leukocytosis : Nat
  sirs : Nat
  sepsis : Nat
  shock : Nat
  level : Nat

@[expose] public def feverish (v : Vitals) : Bool := (Nat.blt (380) ((v).temperature))

@[expose] public def tachycardic (v : Vitals) : Bool := (Nat.blt (90) ((v).heartRate))

@[expose] public def tachypneic (v : Vitals) : Bool := (Nat.blt (20) ((v).respiratoryRate))

@[expose] public def leukocytic (v : Vitals) : Bool := (Nat.blt (12) ((v).whiteCells))

@[expose] public def hypotensive (v : Vitals) : Bool := (Nat.blt ((v).systolic) (90))

@[expose, reducible] public def Febrile (v : Vitals) : Prop := (380 < (v).temperature)

@[expose, reducible] public def Tachycardic (v : Vitals) : Prop := (90 < (v).heartRate)

@[expose, reducible] public def Tachypneic (v : Vitals) : Prop := (20 < (v).respiratoryRate)

@[expose, reducible] public def Leukocytic (v : Vitals) : Prop := (12 < (v).whiteCells)

@[expose, reducible] public def Hypotensive (v : Vitals) : Prop := ((v).systolic < 90)

@[expose, reducible] public def Warranted (c : Chart) : Prop := (((c).level <= 3) /\ ((((c).level = 1) -> ((c).sirs = 1)) /\ ((((c).level = 2) -> ((c).sepsis = 1)) /\ (((c).level = 3) -> ((c).shock = 1)))))

@[expose, reducible] public def Justified (s : Chart) (t : Chart) : Prop := ((((t).vitals).temperature = ((s).vitals).temperature) /\ ((((t).vitals).heartRate = ((s).vitals).heartRate) /\ ((((t).vitals).respiratoryRate = ((s).vitals).respiratoryRate) /\ ((((t).vitals).whiteCells = ((s).vitals).whiteCells) /\ ((((t).vitals).systolic = ((s).vitals).systolic) /\ ((((t).vitals).infection = ((s).vitals).infection) /\ (((s).fever <= (t).fever) /\ ((((s).fever < (t).fever) -> (((t).fever = 1) /\ Febrile ((t).vitals))) /\ (((s).tachycardia <= (t).tachycardia) /\ ((((s).tachycardia < (t).tachycardia) -> (((t).tachycardia = 1) /\ Tachycardic ((t).vitals))) /\ (((s).tachypnea <= (t).tachypnea) /\ ((((s).tachypnea < (t).tachypnea) -> (((t).tachypnea = 1) /\ Tachypneic ((t).vitals))) /\ (((s).leukocytosis <= (t).leukocytosis) /\ ((((s).leukocytosis < (t).leukocytosis) -> (((t).leukocytosis = 1) /\ Leukocytic ((t).vitals))) /\ (((s).sirs <= (t).sirs) /\ ((((s).sirs < (t).sirs) -> (((t).sirs = 1) /\ (2 <= ((((t).fever + (t).tachycardia) + (t).tachypnea) + (t).leukocytosis)))) /\ (((s).sepsis <= (t).sepsis) /\ ((((s).sepsis < (t).sepsis) -> (((t).sepsis = 1) /\ (((t).sirs = 1) /\ (((t).vitals).infection = 1)))) /\ (((s).shock <= (t).shock) /\ ((((s).shock < (t).shock) -> (((t).shock = 1) /\ (((t).sepsis = 1) /\ Hypotensive ((t).vitals)))) /\ (((s).level <= (t).level) /\ (((s).level < (t).level) -> (((t).level = ((s).level + 1)) /\ Warranted (t))))))))))))))))))))))))

@[expose, reducible] public def Consistent (c : Chart) : Prop := (((c).fever <= 1) /\ (((c).tachycardia <= 1) /\ (((c).tachypnea <= 1) /\ (((c).leukocytosis <= 1) /\ (((c).sirs <= 1) /\ (((c).sepsis <= 1) /\ (((c).shock <= 1) /\ ((c).level <= 3))))))))

@[expose] public def Pending (c : Chart) : Nat := ((LexLeanRuntime.subtract (1) ((c).fever) : Nat) + ((LexLeanRuntime.subtract (1) ((c).tachycardia) : Nat) + ((LexLeanRuntime.subtract (1) ((c).tachypnea) : Nat) + ((LexLeanRuntime.subtract (1) ((c).leukocytosis) : Nat) + ((LexLeanRuntime.subtract (1) ((c).sirs) : Nat) + ((LexLeanRuntime.subtract (1) ((c).sepsis) : Nat) + ((LexLeanRuntime.subtract (1) ((c).shock) : Nat) + (LexLeanRuntime.subtract (3) ((c).level) : Nat))))))))

public theorem consistent_preserved (s : Chart) (t : Chart) : (Consistent (s) -> (Justified (s) (t) -> Consistent (t))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Consistent, Febrile, Hypotensive, Justified, Leukocytic, Tachycardic, Tachypneic, Warranted, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem Findings.preserves : (LexLeanReasoning.Preserves ((LexLeanTarget.ReasoningOracle.Justified)) ((LexLeanTarget.ReasoningOracle.Consistent))) :=
  LexLeanTarget.ReasoningOracle.consistent_preserved

public theorem fever_sound (s : Chart) : ((((Nat.beq ((s).fever) (0)) && feverish ((s).vitals)) = true) -> Justified (s) (({ vitals := (s).vitals, fever := 1, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Chart))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Febrile, Hypotensive, Justified, Leukocytic, Tachycardic, Tachypneic, Warranted, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem fever_progress (s : Chart) : (Consistent (s) -> ((((Nat.beq ((s).fever) (0)) && feverish ((s).vitals)) = true) -> (Pending (({ vitals := (s).vitals, fever := 1, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Chart)) < Pending (s)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Consistent, Pending, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

@[expose] public def Fever.guard (s : LexLeanTarget.ReasoningOracle.Chart) : Bool := ((Nat.beq ((s).fever) (0)) && LexLeanTarget.ReasoningOracle.feverish ((s).vitals))
@[expose] public def Fever.conclusion (s : LexLeanTarget.ReasoningOracle.Chart) : LexLeanTarget.ReasoningOracle.Chart := ({ vitals := (s).vitals, fever := 1, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : LexLeanTarget.ReasoningOracle.Chart)
@[expose] public def Fever.apply (s : LexLeanTarget.ReasoningOracle.Chart) : Option (LexLeanTarget.ReasoningOracle.Chart) := (if LexLeanTarget.ReasoningOracle.Fever.guard (s) then Option.some (LexLeanTarget.ReasoningOracle.Fever.conclusion (s)) else Option.none)
public theorem Fever.apply_sound (s : LexLeanTarget.ReasoningOracle.Chart) (__t : LexLeanTarget.ReasoningOracle.Chart) : ((LexLeanTarget.ReasoningOracle.Fever.apply (s) = Option.some (__t)) -> LexLeanTarget.ReasoningOracle.Justified (s) (__t)) :=
  (LexLeanReasoning.guarded ((LexLeanTarget.ReasoningOracle.Justified)) (s) (LexLeanTarget.ReasoningOracle.Fever.guard (s)) (LexLeanTarget.ReasoningOracle.Fever.conclusion (s)) (LexLeanTarget.ReasoningOracle.fever_sound (s)) (__t))
public theorem Fever.apply_progress (s : LexLeanTarget.ReasoningOracle.Chart) (__t : LexLeanTarget.ReasoningOracle.Chart) : (LexLeanTarget.ReasoningOracle.Consistent (s) -> ((LexLeanTarget.ReasoningOracle.Fever.apply (s) = Option.some (__t)) -> (LexLeanTarget.ReasoningOracle.Pending (__t) < LexLeanTarget.ReasoningOracle.Pending (s)))) :=
  (fun llJ => (LexLeanReasoning.guardedRank ((LexLeanTarget.ReasoningOracle.Pending)) (s) (LexLeanTarget.ReasoningOracle.Fever.guard (s)) (LexLeanTarget.ReasoningOracle.Fever.conclusion (s)) (LexLeanTarget.ReasoningOracle.fever_progress (s) llJ) (__t)))

public theorem tachycardia_sound (s : Chart) : ((((Nat.beq ((s).tachycardia) (0)) && tachycardic ((s).vitals)) = true) -> Justified (s) (({ vitals := (s).vitals, fever := (s).fever, tachycardia := 1, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Chart))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Febrile, Hypotensive, Justified, Leukocytic, Tachycardic, Tachypneic, Warranted, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem tachycardia_progress (s : Chart) : (Consistent (s) -> ((((Nat.beq ((s).tachycardia) (0)) && tachycardic ((s).vitals)) = true) -> (Pending (({ vitals := (s).vitals, fever := (s).fever, tachycardia := 1, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Chart)) < Pending (s)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Consistent, Pending, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

@[expose] public def Tachycardia.guard (s : LexLeanTarget.ReasoningOracle.Chart) : Bool := ((Nat.beq ((s).tachycardia) (0)) && LexLeanTarget.ReasoningOracle.tachycardic ((s).vitals))
@[expose] public def Tachycardia.conclusion (s : LexLeanTarget.ReasoningOracle.Chart) : LexLeanTarget.ReasoningOracle.Chart := ({ vitals := (s).vitals, fever := (s).fever, tachycardia := 1, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : LexLeanTarget.ReasoningOracle.Chart)
@[expose] public def Tachycardia.apply (s : LexLeanTarget.ReasoningOracle.Chart) : Option (LexLeanTarget.ReasoningOracle.Chart) := (if LexLeanTarget.ReasoningOracle.Tachycardia.guard (s) then Option.some (LexLeanTarget.ReasoningOracle.Tachycardia.conclusion (s)) else Option.none)
public theorem Tachycardia.apply_sound (s : LexLeanTarget.ReasoningOracle.Chart) (__t : LexLeanTarget.ReasoningOracle.Chart) : ((LexLeanTarget.ReasoningOracle.Tachycardia.apply (s) = Option.some (__t)) -> LexLeanTarget.ReasoningOracle.Justified (s) (__t)) :=
  (LexLeanReasoning.guarded ((LexLeanTarget.ReasoningOracle.Justified)) (s) (LexLeanTarget.ReasoningOracle.Tachycardia.guard (s)) (LexLeanTarget.ReasoningOracle.Tachycardia.conclusion (s)) (LexLeanTarget.ReasoningOracle.tachycardia_sound (s)) (__t))
public theorem Tachycardia.apply_progress (s : LexLeanTarget.ReasoningOracle.Chart) (__t : LexLeanTarget.ReasoningOracle.Chart) : (LexLeanTarget.ReasoningOracle.Consistent (s) -> ((LexLeanTarget.ReasoningOracle.Tachycardia.apply (s) = Option.some (__t)) -> (LexLeanTarget.ReasoningOracle.Pending (__t) < LexLeanTarget.ReasoningOracle.Pending (s)))) :=
  (fun llJ => (LexLeanReasoning.guardedRank ((LexLeanTarget.ReasoningOracle.Pending)) (s) (LexLeanTarget.ReasoningOracle.Tachycardia.guard (s)) (LexLeanTarget.ReasoningOracle.Tachycardia.conclusion (s)) (LexLeanTarget.ReasoningOracle.tachycardia_progress (s) llJ) (__t)))

public theorem tachypnea_sound (s : Chart) : ((((Nat.beq ((s).tachypnea) (0)) && tachypneic ((s).vitals)) = true) -> Justified (s) (({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := 1, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Chart))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Febrile, Hypotensive, Justified, Leukocytic, Tachycardic, Tachypneic, Warranted, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem tachypnea_progress (s : Chart) : (Consistent (s) -> ((((Nat.beq ((s).tachypnea) (0)) && tachypneic ((s).vitals)) = true) -> (Pending (({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := 1, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Chart)) < Pending (s)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Consistent, Pending, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

@[expose] public def Tachypnea.guard (s : LexLeanTarget.ReasoningOracle.Chart) : Bool := ((Nat.beq ((s).tachypnea) (0)) && LexLeanTarget.ReasoningOracle.tachypneic ((s).vitals))
@[expose] public def Tachypnea.conclusion (s : LexLeanTarget.ReasoningOracle.Chart) : LexLeanTarget.ReasoningOracle.Chart := ({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := 1, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : LexLeanTarget.ReasoningOracle.Chart)
@[expose] public def Tachypnea.apply (s : LexLeanTarget.ReasoningOracle.Chart) : Option (LexLeanTarget.ReasoningOracle.Chart) := (if LexLeanTarget.ReasoningOracle.Tachypnea.guard (s) then Option.some (LexLeanTarget.ReasoningOracle.Tachypnea.conclusion (s)) else Option.none)
public theorem Tachypnea.apply_sound (s : LexLeanTarget.ReasoningOracle.Chart) (__t : LexLeanTarget.ReasoningOracle.Chart) : ((LexLeanTarget.ReasoningOracle.Tachypnea.apply (s) = Option.some (__t)) -> LexLeanTarget.ReasoningOracle.Justified (s) (__t)) :=
  (LexLeanReasoning.guarded ((LexLeanTarget.ReasoningOracle.Justified)) (s) (LexLeanTarget.ReasoningOracle.Tachypnea.guard (s)) (LexLeanTarget.ReasoningOracle.Tachypnea.conclusion (s)) (LexLeanTarget.ReasoningOracle.tachypnea_sound (s)) (__t))
public theorem Tachypnea.apply_progress (s : LexLeanTarget.ReasoningOracle.Chart) (__t : LexLeanTarget.ReasoningOracle.Chart) : (LexLeanTarget.ReasoningOracle.Consistent (s) -> ((LexLeanTarget.ReasoningOracle.Tachypnea.apply (s) = Option.some (__t)) -> (LexLeanTarget.ReasoningOracle.Pending (__t) < LexLeanTarget.ReasoningOracle.Pending (s)))) :=
  (fun llJ => (LexLeanReasoning.guardedRank ((LexLeanTarget.ReasoningOracle.Pending)) (s) (LexLeanTarget.ReasoningOracle.Tachypnea.guard (s)) (LexLeanTarget.ReasoningOracle.Tachypnea.conclusion (s)) (LexLeanTarget.ReasoningOracle.tachypnea_progress (s) llJ) (__t)))

public theorem leukocytosis_sound (s : Chart) : ((((Nat.beq ((s).leukocytosis) (0)) && leukocytic ((s).vitals)) = true) -> Justified (s) (({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := 1, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Chart))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Febrile, Hypotensive, Justified, Leukocytic, Tachycardic, Tachypneic, Warranted, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem leukocytosis_progress (s : Chart) : (Consistent (s) -> ((((Nat.beq ((s).leukocytosis) (0)) && leukocytic ((s).vitals)) = true) -> (Pending (({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := 1, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Chart)) < Pending (s)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Consistent, Pending, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

@[expose] public def Leukocytosis.guard (s : LexLeanTarget.ReasoningOracle.Chart) : Bool := ((Nat.beq ((s).leukocytosis) (0)) && LexLeanTarget.ReasoningOracle.leukocytic ((s).vitals))
@[expose] public def Leukocytosis.conclusion (s : LexLeanTarget.ReasoningOracle.Chart) : LexLeanTarget.ReasoningOracle.Chart := ({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := 1, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : LexLeanTarget.ReasoningOracle.Chart)
@[expose] public def Leukocytosis.apply (s : LexLeanTarget.ReasoningOracle.Chart) : Option (LexLeanTarget.ReasoningOracle.Chart) := (if LexLeanTarget.ReasoningOracle.Leukocytosis.guard (s) then Option.some (LexLeanTarget.ReasoningOracle.Leukocytosis.conclusion (s)) else Option.none)
public theorem Leukocytosis.apply_sound (s : LexLeanTarget.ReasoningOracle.Chart) (__t : LexLeanTarget.ReasoningOracle.Chart) : ((LexLeanTarget.ReasoningOracle.Leukocytosis.apply (s) = Option.some (__t)) -> LexLeanTarget.ReasoningOracle.Justified (s) (__t)) :=
  (LexLeanReasoning.guarded ((LexLeanTarget.ReasoningOracle.Justified)) (s) (LexLeanTarget.ReasoningOracle.Leukocytosis.guard (s)) (LexLeanTarget.ReasoningOracle.Leukocytosis.conclusion (s)) (LexLeanTarget.ReasoningOracle.leukocytosis_sound (s)) (__t))
public theorem Leukocytosis.apply_progress (s : LexLeanTarget.ReasoningOracle.Chart) (__t : LexLeanTarget.ReasoningOracle.Chart) : (LexLeanTarget.ReasoningOracle.Consistent (s) -> ((LexLeanTarget.ReasoningOracle.Leukocytosis.apply (s) = Option.some (__t)) -> (LexLeanTarget.ReasoningOracle.Pending (__t) < LexLeanTarget.ReasoningOracle.Pending (s)))) :=
  (fun llJ => (LexLeanReasoning.guardedRank ((LexLeanTarget.ReasoningOracle.Pending)) (s) (LexLeanTarget.ReasoningOracle.Leukocytosis.guard (s)) (LexLeanTarget.ReasoningOracle.Leukocytosis.conclusion (s)) (LexLeanTarget.ReasoningOracle.leukocytosis_progress (s) llJ) (__t)))

public theorem sirs_sound (s : Chart) : ((((Nat.beq ((s).sirs) (0)) && (Nat.ble (2) (((((s).fever + (s).tachycardia) + (s).tachypnea) + (s).leukocytosis)))) = true) -> Justified (s) (({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := 1, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Chart))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Febrile, Hypotensive, Justified, Leukocytic, Tachycardic, Tachypneic, Warranted, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem sirs_progress (s : Chart) : (Consistent (s) -> ((((Nat.beq ((s).sirs) (0)) && (Nat.ble (2) (((((s).fever + (s).tachycardia) + (s).tachypnea) + (s).leukocytosis)))) = true) -> (Pending (({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := 1, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Chart)) < Pending (s)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Consistent, Pending, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

@[expose] public def Sirs.guard (s : LexLeanTarget.ReasoningOracle.Chart) : Bool := ((Nat.beq ((s).sirs) (0)) && (Nat.ble (2) (((((s).fever + (s).tachycardia) + (s).tachypnea) + (s).leukocytosis))))
@[expose] public def Sirs.conclusion (s : LexLeanTarget.ReasoningOracle.Chart) : LexLeanTarget.ReasoningOracle.Chart := ({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := 1, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : LexLeanTarget.ReasoningOracle.Chart)
@[expose] public def Sirs.apply (s : LexLeanTarget.ReasoningOracle.Chart) : Option (LexLeanTarget.ReasoningOracle.Chart) := (if LexLeanTarget.ReasoningOracle.Sirs.guard (s) then Option.some (LexLeanTarget.ReasoningOracle.Sirs.conclusion (s)) else Option.none)
public theorem Sirs.apply_sound (s : LexLeanTarget.ReasoningOracle.Chart) (__t : LexLeanTarget.ReasoningOracle.Chart) : ((LexLeanTarget.ReasoningOracle.Sirs.apply (s) = Option.some (__t)) -> LexLeanTarget.ReasoningOracle.Justified (s) (__t)) :=
  (LexLeanReasoning.guarded ((LexLeanTarget.ReasoningOracle.Justified)) (s) (LexLeanTarget.ReasoningOracle.Sirs.guard (s)) (LexLeanTarget.ReasoningOracle.Sirs.conclusion (s)) (LexLeanTarget.ReasoningOracle.sirs_sound (s)) (__t))
public theorem Sirs.apply_progress (s : LexLeanTarget.ReasoningOracle.Chart) (__t : LexLeanTarget.ReasoningOracle.Chart) : (LexLeanTarget.ReasoningOracle.Consistent (s) -> ((LexLeanTarget.ReasoningOracle.Sirs.apply (s) = Option.some (__t)) -> (LexLeanTarget.ReasoningOracle.Pending (__t) < LexLeanTarget.ReasoningOracle.Pending (s)))) :=
  (fun llJ => (LexLeanReasoning.guardedRank ((LexLeanTarget.ReasoningOracle.Pending)) (s) (LexLeanTarget.ReasoningOracle.Sirs.guard (s)) (LexLeanTarget.ReasoningOracle.Sirs.conclusion (s)) (LexLeanTarget.ReasoningOracle.sirs_progress (s) llJ) (__t)))

public theorem sepsis_sound (s : Chart) : ((((Nat.beq ((s).sepsis) (0)) && ((Nat.beq ((s).sirs) (1)) && (Nat.beq (((s).vitals).infection) (1)))) = true) -> Justified (s) (({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := 1, shock := (s).shock, level := (s).level } : Chart))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Febrile, Hypotensive, Justified, Leukocytic, Tachycardic, Tachypneic, Warranted, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem sepsis_progress (s : Chart) : (Consistent (s) -> ((((Nat.beq ((s).sepsis) (0)) && ((Nat.beq ((s).sirs) (1)) && (Nat.beq (((s).vitals).infection) (1)))) = true) -> (Pending (({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := 1, shock := (s).shock, level := (s).level } : Chart)) < Pending (s)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Consistent, Pending, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

@[expose] public def Sepsis.guard (s : LexLeanTarget.ReasoningOracle.Chart) : Bool := ((Nat.beq ((s).sepsis) (0)) && ((Nat.beq ((s).sirs) (1)) && (Nat.beq (((s).vitals).infection) (1))))
@[expose] public def Sepsis.conclusion (s : LexLeanTarget.ReasoningOracle.Chart) : LexLeanTarget.ReasoningOracle.Chart := ({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := 1, shock := (s).shock, level := (s).level } : LexLeanTarget.ReasoningOracle.Chart)
@[expose] public def Sepsis.apply (s : LexLeanTarget.ReasoningOracle.Chart) : Option (LexLeanTarget.ReasoningOracle.Chart) := (if LexLeanTarget.ReasoningOracle.Sepsis.guard (s) then Option.some (LexLeanTarget.ReasoningOracle.Sepsis.conclusion (s)) else Option.none)
public theorem Sepsis.apply_sound (s : LexLeanTarget.ReasoningOracle.Chart) (__t : LexLeanTarget.ReasoningOracle.Chart) : ((LexLeanTarget.ReasoningOracle.Sepsis.apply (s) = Option.some (__t)) -> LexLeanTarget.ReasoningOracle.Justified (s) (__t)) :=
  (LexLeanReasoning.guarded ((LexLeanTarget.ReasoningOracle.Justified)) (s) (LexLeanTarget.ReasoningOracle.Sepsis.guard (s)) (LexLeanTarget.ReasoningOracle.Sepsis.conclusion (s)) (LexLeanTarget.ReasoningOracle.sepsis_sound (s)) (__t))
public theorem Sepsis.apply_progress (s : LexLeanTarget.ReasoningOracle.Chart) (__t : LexLeanTarget.ReasoningOracle.Chart) : (LexLeanTarget.ReasoningOracle.Consistent (s) -> ((LexLeanTarget.ReasoningOracle.Sepsis.apply (s) = Option.some (__t)) -> (LexLeanTarget.ReasoningOracle.Pending (__t) < LexLeanTarget.ReasoningOracle.Pending (s)))) :=
  (fun llJ => (LexLeanReasoning.guardedRank ((LexLeanTarget.ReasoningOracle.Pending)) (s) (LexLeanTarget.ReasoningOracle.Sepsis.guard (s)) (LexLeanTarget.ReasoningOracle.Sepsis.conclusion (s)) (LexLeanTarget.ReasoningOracle.sepsis_progress (s) llJ) (__t)))

public theorem shock_sound (s : Chart) : ((((Nat.beq ((s).shock) (0)) && ((Nat.beq ((s).sepsis) (1)) && hypotensive ((s).vitals))) = true) -> Justified (s) (({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := 1, level := (s).level } : Chart))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Febrile, Hypotensive, Justified, Leukocytic, Tachycardic, Tachypneic, Warranted, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem shock_progress (s : Chart) : (Consistent (s) -> ((((Nat.beq ((s).shock) (0)) && ((Nat.beq ((s).sepsis) (1)) && hypotensive ((s).vitals))) = true) -> (Pending (({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := 1, level := (s).level } : Chart)) < Pending (s)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Consistent, Pending, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

@[expose] public def Shock.guard (s : LexLeanTarget.ReasoningOracle.Chart) : Bool := ((Nat.beq ((s).shock) (0)) && ((Nat.beq ((s).sepsis) (1)) && LexLeanTarget.ReasoningOracle.hypotensive ((s).vitals)))
@[expose] public def Shock.conclusion (s : LexLeanTarget.ReasoningOracle.Chart) : LexLeanTarget.ReasoningOracle.Chart := ({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := 1, level := (s).level } : LexLeanTarget.ReasoningOracle.Chart)
@[expose] public def Shock.apply (s : LexLeanTarget.ReasoningOracle.Chart) : Option (LexLeanTarget.ReasoningOracle.Chart) := (if LexLeanTarget.ReasoningOracle.Shock.guard (s) then Option.some (LexLeanTarget.ReasoningOracle.Shock.conclusion (s)) else Option.none)
public theorem Shock.apply_sound (s : LexLeanTarget.ReasoningOracle.Chart) (__t : LexLeanTarget.ReasoningOracle.Chart) : ((LexLeanTarget.ReasoningOracle.Shock.apply (s) = Option.some (__t)) -> LexLeanTarget.ReasoningOracle.Justified (s) (__t)) :=
  (LexLeanReasoning.guarded ((LexLeanTarget.ReasoningOracle.Justified)) (s) (LexLeanTarget.ReasoningOracle.Shock.guard (s)) (LexLeanTarget.ReasoningOracle.Shock.conclusion (s)) (LexLeanTarget.ReasoningOracle.shock_sound (s)) (__t))
public theorem Shock.apply_progress (s : LexLeanTarget.ReasoningOracle.Chart) (__t : LexLeanTarget.ReasoningOracle.Chart) : (LexLeanTarget.ReasoningOracle.Consistent (s) -> ((LexLeanTarget.ReasoningOracle.Shock.apply (s) = Option.some (__t)) -> (LexLeanTarget.ReasoningOracle.Pending (__t) < LexLeanTarget.ReasoningOracle.Pending (s)))) :=
  (fun llJ => (LexLeanReasoning.guardedRank ((LexLeanTarget.ReasoningOracle.Pending)) (s) (LexLeanTarget.ReasoningOracle.Shock.guard (s)) (LexLeanTarget.ReasoningOracle.Shock.conclusion (s)) (LexLeanTarget.ReasoningOracle.shock_progress (s) llJ) (__t)))

public theorem escalate_sound (s : Chart) : ((((Nat.blt ((s).level) (3)) && (((Nat.beq ((s).level) (0)) && (Nat.beq ((s).sirs) (1))) || (((Nat.beq ((s).level) (1)) && (Nat.beq ((s).sepsis) (1))) || ((Nat.beq ((s).level) (2)) && (Nat.beq ((s).shock) (1)))))) = true) -> Justified (s) (({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := ((s).level + 1) } : Chart))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Febrile, Hypotensive, Justified, Leukocytic, Tachycardic, Tachypneic, Warranted, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem escalate_progress (s : Chart) : (Consistent (s) -> ((((Nat.blt ((s).level) (3)) && (((Nat.beq ((s).level) (0)) && (Nat.beq ((s).sirs) (1))) || (((Nat.beq ((s).level) (1)) && (Nat.beq ((s).sepsis) (1))) || ((Nat.beq ((s).level) (2)) && (Nat.beq ((s).shock) (1)))))) = true) -> (Pending (({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := ((s).level + 1) } : Chart)) < Pending (s)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Consistent, Pending, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

@[expose] public def Escalate.guard (s : LexLeanTarget.ReasoningOracle.Chart) : Bool := ((Nat.blt ((s).level) (3)) && (((Nat.beq ((s).level) (0)) && (Nat.beq ((s).sirs) (1))) || (((Nat.beq ((s).level) (1)) && (Nat.beq ((s).sepsis) (1))) || ((Nat.beq ((s).level) (2)) && (Nat.beq ((s).shock) (1))))))
@[expose] public def Escalate.conclusion (s : LexLeanTarget.ReasoningOracle.Chart) : LexLeanTarget.ReasoningOracle.Chart := ({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := ((s).level + 1) } : LexLeanTarget.ReasoningOracle.Chart)
@[expose] public def Escalate.apply (s : LexLeanTarget.ReasoningOracle.Chart) : Option (LexLeanTarget.ReasoningOracle.Chart) := (if LexLeanTarget.ReasoningOracle.Escalate.guard (s) then Option.some (LexLeanTarget.ReasoningOracle.Escalate.conclusion (s)) else Option.none)
public theorem Escalate.apply_sound (s : LexLeanTarget.ReasoningOracle.Chart) (__t : LexLeanTarget.ReasoningOracle.Chart) : ((LexLeanTarget.ReasoningOracle.Escalate.apply (s) = Option.some (__t)) -> LexLeanTarget.ReasoningOracle.Justified (s) (__t)) :=
  (LexLeanReasoning.guarded ((LexLeanTarget.ReasoningOracle.Justified)) (s) (LexLeanTarget.ReasoningOracle.Escalate.guard (s)) (LexLeanTarget.ReasoningOracle.Escalate.conclusion (s)) (LexLeanTarget.ReasoningOracle.escalate_sound (s)) (__t))
public theorem Escalate.apply_progress (s : LexLeanTarget.ReasoningOracle.Chart) (__t : LexLeanTarget.ReasoningOracle.Chart) : (LexLeanTarget.ReasoningOracle.Consistent (s) -> ((LexLeanTarget.ReasoningOracle.Escalate.apply (s) = Option.some (__t)) -> (LexLeanTarget.ReasoningOracle.Pending (__t) < LexLeanTarget.ReasoningOracle.Pending (s)))) :=
  (fun llJ => (LexLeanReasoning.guardedRank ((LexLeanTarget.ReasoningOracle.Pending)) (s) (LexLeanTarget.ReasoningOracle.Escalate.guard (s)) (LexLeanTarget.ReasoningOracle.Escalate.conclusion (s)) (LexLeanTarget.ReasoningOracle.escalate_progress (s) llJ) (__t)))

@[expose, reducible] public def Indicated (v : Vitals) (r : Nat) : Prop := ((r <= 3) /\ ((Hypotensive (v) /\ (((v).infection = 1) /\ (Febrile (v) /\ Tachycardic (v)))) -> (r = 3)))

@[expose] public def recommendationCheck (v : Vitals) (r : Nat) : Bool := ((Nat.ble (r) (3)) && ((Nat.beq (r) (3)) || (!(hypotensive (v) && ((Nat.beq ((v).infection) (1)) && (feverish (v) && tachycardic (v)))))))

public theorem recommendation_sound (v : Vitals) (r : Nat) : ((recommendationCheck (v) (r) = true) -> Indicated (v) (r)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Febrile, Hypotensive, Indicated, Leukocytic, Tachycardic, Tachypneic, feverish, hypotensive, leukocytic, recommendationCheck, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem recommendation_complete (v : Vitals) (r : Nat) : (Indicated (v) (r) -> (recommendationCheck (v) (r) = true)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Febrile, Hypotensive, Indicated, Leukocytic, Tachycardic, Tachypneic, feverish, hypotensive, leukocytic, recommendationCheck, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem Recommendation.sound : (LexLeanReasoning.Sound ((LexLeanTarget.ReasoningOracle.recommendationCheck)) ((LexLeanTarget.ReasoningOracle.Indicated))) :=
  LexLeanTarget.ReasoningOracle.recommendation_sound
public theorem Recommendation.complete : (LexLeanReasoning.Complete ((LexLeanTarget.ReasoningOracle.recommendationCheck)) ((LexLeanTarget.ReasoningOracle.Indicated))) :=
  LexLeanTarget.ReasoningOracle.recommendation_complete

@[expose, reducible] public def Discharged (_v : Vitals) (r : Nat) : Prop := (r = 0)

@[expose] public def dischargeCheck (_v : Vitals) (r : Nat) : Bool := (Nat.beq (r) (0))

public theorem discharge_sound (v : Vitals) (r : Nat) : ((dischargeCheck (v) (r) = true) -> Discharged (v) (r)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Discharged, dischargeCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem Outpatient.sound : (LexLeanReasoning.Sound ((LexLeanTarget.ReasoningOracle.dischargeCheck)) ((LexLeanTarget.ReasoningOracle.Discharged))) :=
  LexLeanTarget.ReasoningOracle.discharge_sound

public theorem admitted_consistent (patient : Vitals) : Consistent (({ vitals := patient, fever := 0, tachycardia := 0, tachypnea := 0, leukocytosis := 0, sirs := 0, sepsis := 0, shock := 0, level := 0 } : Chart)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Consistent, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem admitted_pending (patient : Vitals) : (Pending (({ vitals := patient, fever := 0, tachycardia := 0, tachypnea := 0, leukocytosis := 0, sirs := 0, sepsis := 0, shock := 0, level := 0 } : Chart)) < 11) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Pending, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public inductive Triage.Step where
  | Fever
  | Tachycardia
  | Tachypnea
  | Leukocytosis
  | Sirs
  | Sepsis
  | Shock
  | Escalate
public structure Triage.Ledger where
  iterations : Nat
  attempts : Nat
  firings : Nat
  expansions : Nat
  verifications : Nat
  frontier : Nat
@[expose] public def Triage.observe (patient : LexLeanTarget.ReasoningOracle.Vitals) : LexLeanTarget.ReasoningOracle.Chart := ({ vitals := patient, fever := 0, tachycardia := 0, tachypnea := 0, leukocytosis := 0, sirs := 0, sepsis := 0, shock := 0, level := 0 } : LexLeanTarget.ReasoningOracle.Chart)
@[expose] public def Triage.fire (__s : LexLeanTarget.ReasoningOracle.Chart) (__step : LexLeanTarget.ReasoningOracle.Triage.Step) : Option (LexLeanTarget.ReasoningOracle.Chart) := (match __step with | LexLeanTarget.ReasoningOracle.Triage.Step.Fever => LexLeanTarget.ReasoningOracle.Fever.apply (__s) | LexLeanTarget.ReasoningOracle.Triage.Step.Tachycardia => LexLeanTarget.ReasoningOracle.Tachycardia.apply (__s) | LexLeanTarget.ReasoningOracle.Triage.Step.Tachypnea => LexLeanTarget.ReasoningOracle.Tachypnea.apply (__s) | LexLeanTarget.ReasoningOracle.Triage.Step.Leukocytosis => LexLeanTarget.ReasoningOracle.Leukocytosis.apply (__s) | LexLeanTarget.ReasoningOracle.Triage.Step.Sirs => LexLeanTarget.ReasoningOracle.Sirs.apply (__s) | LexLeanTarget.ReasoningOracle.Triage.Step.Sepsis => LexLeanTarget.ReasoningOracle.Sepsis.apply (__s) | LexLeanTarget.ReasoningOracle.Triage.Step.Shock => LexLeanTarget.ReasoningOracle.Shock.apply (__s) | LexLeanTarget.ReasoningOracle.Triage.Step.Escalate => LexLeanTarget.ReasoningOracle.Escalate.apply (__s))
@[expose] public def Triage.replay (__acc : Except ((Prod Bool Bool)) (LexLeanTarget.ReasoningOracle.Chart)) (__step : LexLeanTarget.ReasoningOracle.Triage.Step) : Except ((Prod Bool Bool)) (LexLeanTarget.ReasoningOracle.Chart) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match LexLeanTarget.ReasoningOracle.Triage.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def Triage.follow (patient : LexLeanTarget.ReasoningOracle.Vitals) (__trace : List (LexLeanTarget.ReasoningOracle.Triage.Step)) : Except ((Prod Bool Bool)) (LexLeanTarget.ReasoningOracle.Chart) := (LexLeanCollections.listFold ((LexLeanTarget.ReasoningOracle.Triage.replay)) (Except.ok (LexLeanTarget.ReasoningOracle.Triage.observe (patient))) (__trace) : Except ((Prod Bool Bool)) (LexLeanTarget.ReasoningOracle.Chart))
@[expose] public def Triage.extract (chart : LexLeanTarget.ReasoningOracle.Chart) : Option (Nat) := Option.some ((chart).level)
@[expose] public def Triage.accept (patient : LexLeanTarget.ReasoningOracle.Vitals) (__s : LexLeanTarget.ReasoningOracle.Chart) : Option (Nat) := (match LexLeanTarget.ReasoningOracle.Triage.extract (__s) with | Option.none => Option.none | Option.some __v => (if LexLeanTarget.ReasoningOracle.recommendationCheck (patient) (__v) then Option.some (__v) else Option.none))
@[expose] public def Triage.answer (patient : LexLeanTarget.ReasoningOracle.Vitals) (__trace : List (LexLeanTarget.ReasoningOracle.Triage.Step)) : Option (Nat) := (match LexLeanTarget.ReasoningOracle.Triage.follow (patient) (__trace) with | Except.ok __s => LexLeanTarget.ReasoningOracle.Triage.accept (patient) (__s) | Except.error _ => Option.none)
@[expose] public def Triage.select (__s : LexLeanTarget.ReasoningOracle.Chart) : Option (LexLeanTarget.ReasoningOracle.Triage.Step) := (match (match (match (if LexLeanTarget.ReasoningOracle.Fever.guard (__s) then Option.some (LexLeanTarget.ReasoningOracle.Triage.Step.Fever) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if LexLeanTarget.ReasoningOracle.Tachycardia.guard (__s) then Option.some (LexLeanTarget.ReasoningOracle.Triage.Step.Tachycardia) else Option.none)) with | Option.some __found => Option.some (__found) | Option.none => (match (if LexLeanTarget.ReasoningOracle.Tachypnea.guard (__s) then Option.some (LexLeanTarget.ReasoningOracle.Triage.Step.Tachypnea) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if LexLeanTarget.ReasoningOracle.Leukocytosis.guard (__s) then Option.some (LexLeanTarget.ReasoningOracle.Triage.Step.Leukocytosis) else Option.none))) with | Option.some __found => Option.some (__found) | Option.none => (match (match (if LexLeanTarget.ReasoningOracle.Sirs.guard (__s) then Option.some (LexLeanTarget.ReasoningOracle.Triage.Step.Sirs) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if LexLeanTarget.ReasoningOracle.Sepsis.guard (__s) then Option.some (LexLeanTarget.ReasoningOracle.Triage.Step.Sepsis) else Option.none)) with | Option.some __found => Option.some (__found) | Option.none => (match (if LexLeanTarget.ReasoningOracle.Shock.guard (__s) then Option.some (LexLeanTarget.ReasoningOracle.Triage.Step.Shock) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if LexLeanTarget.ReasoningOracle.Escalate.guard (__s) then Option.some (LexLeanTarget.ReasoningOracle.Triage.Step.Escalate) else Option.none))))
@[expose] public def Triage.attempts (__s : LexLeanTarget.ReasoningOracle.Chart) : Nat := ((let __scan6l : (Prod (Bool) (Nat)) := (let __scan2l : (Prod (Bool) (Nat)) := (let __scan0l : (Prod (Bool) (Nat)) := (LexLeanTarget.ReasoningOracle.Fever.guard (__s), 1); (if (__scan0l).1 then __scan0l else (let __scan0r : (Prod (Bool) (Nat)) := (LexLeanTarget.ReasoningOracle.Tachycardia.guard (__s), 1); ((__scan0r).1, ((__scan0l).2 + (__scan0r).2))))); (if (__scan2l).1 then __scan2l else (let __scan2r : (Prod (Bool) (Nat)) := (let __scan1l : (Prod (Bool) (Nat)) := (LexLeanTarget.ReasoningOracle.Tachypnea.guard (__s), 1); (if (__scan1l).1 then __scan1l else (let __scan1r : (Prod (Bool) (Nat)) := (LexLeanTarget.ReasoningOracle.Leukocytosis.guard (__s), 1); ((__scan1r).1, ((__scan1l).2 + (__scan1r).2))))); ((__scan2r).1, ((__scan2l).2 + (__scan2r).2))))); (if (__scan6l).1 then __scan6l else (let __scan6r : (Prod (Bool) (Nat)) := (let __scan5l : (Prod (Bool) (Nat)) := (let __scan3l : (Prod (Bool) (Nat)) := (LexLeanTarget.ReasoningOracle.Sirs.guard (__s), 1); (if (__scan3l).1 then __scan3l else (let __scan3r : (Prod (Bool) (Nat)) := (LexLeanTarget.ReasoningOracle.Sepsis.guard (__s), 1); ((__scan3r).1, ((__scan3l).2 + (__scan3r).2))))); (if (__scan5l).1 then __scan5l else (let __scan5r : (Prod (Bool) (Nat)) := (let __scan4l : (Prod (Bool) (Nat)) := (LexLeanTarget.ReasoningOracle.Shock.guard (__s), 1); (if (__scan4l).1 then __scan4l else (let __scan4r : (Prod (Bool) (Nat)) := (LexLeanTarget.ReasoningOracle.Escalate.guard (__s), 1); ((__scan4r).1, ((__scan4l).2 + (__scan4r).2))))); ((__scan5r).1, ((__scan5l).2 + (__scan5r).2))))); ((__scan6r).1, ((__scan6l).2 + (__scan6r).2)))))).2
@[expose] public def Triage.next (__s : LexLeanTarget.ReasoningOracle.Chart) : Option (LexLeanTarget.ReasoningOracle.Chart) := (match LexLeanTarget.ReasoningOracle.Triage.select (__s) with | Option.none => Option.none | Option.some __step => LexLeanTarget.ReasoningOracle.Triage.fire (__s) (__step))
@[expose] public def Triage.saturate (patient : LexLeanTarget.ReasoningOracle.Vitals) : (Prod (LexLeanTarget.ReasoningOracle.Chart) (Bool)) := (LexLeanCollections.iterateUntil ((LexLeanTarget.ReasoningOracle.Triage.next)) (11) (LexLeanTarget.ReasoningOracle.Triage.observe (patient)) : (Prod (LexLeanTarget.ReasoningOracle.Chart) (Bool)))
public structure Triage.Run where
  state : LexLeanTarget.ReasoningOracle.Chart
  trace : List (LexLeanTarget.ReasoningOracle.Triage.Step)
  ledger : LexLeanTarget.ReasoningOracle.Triage.Ledger
@[expose] public def Triage.start (patient : LexLeanTarget.ReasoningOracle.Vitals) : LexLeanTarget.ReasoningOracle.Triage.Run := ({ state := LexLeanTarget.ReasoningOracle.Triage.observe (patient), trace := ([] : List (LexLeanTarget.ReasoningOracle.Triage.Step)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := 0 } : LexLeanTarget.ReasoningOracle.Triage.Ledger) } : LexLeanTarget.ReasoningOracle.Triage.Run)
@[expose] public def Triage.step (__r : LexLeanTarget.ReasoningOracle.Triage.Run) : Option (LexLeanTarget.ReasoningOracle.Triage.Run) := (match LexLeanTarget.ReasoningOracle.Triage.select ((__r).state) with | Option.none => Option.none | Option.some __step => (match LexLeanTarget.ReasoningOracle.Triage.fire ((__r).state) (__step) with | Option.none => Option.none | Option.some __t => Option.some (({ state := __t, trace := (LexLeanRuntime.append ((__r).trace) ((__step :: ([] : List (LexLeanTarget.ReasoningOracle.Triage.Step)))) : List (LexLeanTarget.ReasoningOracle.Triage.Step)), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + LexLeanTarget.ReasoningOracle.Triage.attempts ((__r).state)), firings := (((__r).ledger).firings + 1), expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : LexLeanTarget.ReasoningOracle.Triage.Ledger) } : LexLeanTarget.ReasoningOracle.Triage.Run))))
@[expose] public def Triage.run (patient : LexLeanTarget.ReasoningOracle.Vitals) : (Prod (LexLeanTarget.ReasoningOracle.Triage.Run) (Bool)) := (LexLeanCollections.iterateUntil ((LexLeanTarget.ReasoningOracle.Triage.step)) (11) (LexLeanTarget.ReasoningOracle.Triage.start (patient)) : (Prod (LexLeanTarget.ReasoningOracle.Triage.Run) (Bool)))
@[expose] public def Triage.account (patient : LexLeanTarget.ReasoningOracle.Vitals) : LexLeanTarget.ReasoningOracle.Triage.Ledger := (let __final : (Prod (LexLeanTarget.ReasoningOracle.Triage.Run) (Bool)) := LexLeanTarget.ReasoningOracle.Triage.run (patient); (match (__final).2 with | Bool.true => ({ iterations := (((__final).1).ledger).iterations, attempts := ((((__final).1).ledger).attempts + LexLeanTarget.ReasoningOracle.Triage.attempts (((__final).1).state)), firings := (((__final).1).ledger).firings, expansions := (((__final).1).ledger).expansions, verifications := ((((__final).1).ledger).verifications + 1), frontier := (((__final).1).ledger).frontier } : LexLeanTarget.ReasoningOracle.Triage.Ledger) | Bool.false => ((__final).1).ledger))
@[expose] public def Triage.conclude (patient : LexLeanTarget.ReasoningOracle.Vitals) (__s : LexLeanTarget.ReasoningOracle.Chart) : Except ((Prod Bool Bool)) (Nat) := (match LexLeanTarget.ReasoningOracle.Triage.accept (patient) (__s) with | Option.some __v => Except.ok (__v) | Option.none => (match LexLeanTarget.ReasoningOracle.Triage.extract (__s) with | Option.some _ => Except.error (((true, false) : Prod Bool Bool)) | Option.none => Except.error (((false, true) : Prod Bool Bool))))
@[expose] public def Triage.verdict (patient : LexLeanTarget.ReasoningOracle.Vitals) : Except ((Prod Bool Bool)) (Nat) := (let __final : (Prod (LexLeanTarget.ReasoningOracle.Chart) (Bool)) := LexLeanTarget.ReasoningOracle.Triage.saturate (patient); (match (__final).2 with | Bool.true => LexLeanTarget.ReasoningOracle.Triage.conclude (patient) ((__final).1) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
@[expose] public def Triage (patient : LexLeanTarget.ReasoningOracle.Vitals) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (LexLeanTarget.ReasoningOracle.Triage.Step)))) := (let __final : (Prod (LexLeanTarget.ReasoningOracle.Triage.Run) (Bool)) := LexLeanTarget.ReasoningOracle.Triage.run (patient); (match (__final).2 with | Bool.true => (match LexLeanTarget.ReasoningOracle.Triage.conclude (patient) (((__final).1).state) with | Except.ok __v => Except.ok ((__v, ((__final).1).trace)) | Except.error __e => Except.error (__e)) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
public theorem Triage.fire_sound (__s : LexLeanTarget.ReasoningOracle.Chart) (__step : LexLeanTarget.ReasoningOracle.Triage.Step) (__t : LexLeanTarget.ReasoningOracle.Chart) : ((LexLeanTarget.ReasoningOracle.Triage.fire (__s) (__step) = Option.some (__t)) -> LexLeanTarget.ReasoningOracle.Justified (__s) (__t)) :=
by
  cases __step with
  | Fever =>
    exact LexLeanTarget.ReasoningOracle.Fever.apply_sound __s __t
  | Tachycardia =>
    exact LexLeanTarget.ReasoningOracle.Tachycardia.apply_sound __s __t
  | Tachypnea =>
    exact LexLeanTarget.ReasoningOracle.Tachypnea.apply_sound __s __t
  | Leukocytosis =>
    exact LexLeanTarget.ReasoningOracle.Leukocytosis.apply_sound __s __t
  | Sirs =>
    exact LexLeanTarget.ReasoningOracle.Sirs.apply_sound __s __t
  | Sepsis =>
    exact LexLeanTarget.ReasoningOracle.Sepsis.apply_sound __s __t
  | Shock =>
    exact LexLeanTarget.ReasoningOracle.Shock.apply_sound __s __t
  | Escalate =>
    exact LexLeanTarget.ReasoningOracle.Escalate.apply_sound __s __t
public theorem Triage.fire_progress (__s : LexLeanTarget.ReasoningOracle.Chart) (__step : LexLeanTarget.ReasoningOracle.Triage.Step) (__t : LexLeanTarget.ReasoningOracle.Chart) : (LexLeanTarget.ReasoningOracle.Consistent (__s) -> ((LexLeanTarget.ReasoningOracle.Triage.fire (__s) (__step) = Option.some (__t)) -> (LexLeanTarget.ReasoningOracle.Pending (__t) < LexLeanTarget.ReasoningOracle.Pending (__s)))) :=
by
  cases __step with
  | Fever =>
    exact LexLeanTarget.ReasoningOracle.Fever.apply_progress __s __t
  | Tachycardia =>
    exact LexLeanTarget.ReasoningOracle.Tachycardia.apply_progress __s __t
  | Tachypnea =>
    exact LexLeanTarget.ReasoningOracle.Tachypnea.apply_progress __s __t
  | Leukocytosis =>
    exact LexLeanTarget.ReasoningOracle.Leukocytosis.apply_progress __s __t
  | Sirs =>
    exact LexLeanTarget.ReasoningOracle.Sirs.apply_progress __s __t
  | Sepsis =>
    exact LexLeanTarget.ReasoningOracle.Sepsis.apply_progress __s __t
  | Shock =>
    exact LexLeanTarget.ReasoningOracle.Shock.apply_progress __s __t
  | Escalate =>
    exact LexLeanTarget.ReasoningOracle.Escalate.apply_progress __s __t
public theorem Triage.replay_fire (__s : LexLeanTarget.ReasoningOracle.Chart) (__step : LexLeanTarget.ReasoningOracle.Triage.Step) (__t : LexLeanTarget.ReasoningOracle.Chart) : ((LexLeanTarget.ReasoningOracle.Triage.fire (__s) (__step) = Option.some (__t)) -> (LexLeanTarget.ReasoningOracle.Triage.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [LexLeanTarget.ReasoningOracle.Triage.replay]
  rw [llE]
public theorem Triage.replay_sound (patient : LexLeanTarget.ReasoningOracle.Vitals) (__acc : Except ((Prod Bool Bool)) (LexLeanTarget.ReasoningOracle.Chart)) (__step : LexLeanTarget.ReasoningOracle.Triage.Step) : ((LexLeanReasoning.Reaches ((LexLeanTarget.ReasoningOracle.Justified)) (LexLeanTarget.ReasoningOracle.Triage.observe (patient)) (__acc)) -> (LexLeanReasoning.Reaches ((LexLeanTarget.ReasoningOracle.Justified)) (LexLeanTarget.ReasoningOracle.Triage.observe (patient)) (LexLeanTarget.ReasoningOracle.Triage.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [LexLeanTarget.ReasoningOracle.Triage.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (LexLeanTarget.ReasoningOracle.Triage.fire_sound llS __step _ ‹_›)
public theorem Triage.derivation (patient : LexLeanTarget.ReasoningOracle.Vitals) (__trace : List (LexLeanTarget.ReasoningOracle.Triage.Step)) : (LexLeanReasoning.Reaches ((LexLeanTarget.ReasoningOracle.Justified)) (LexLeanTarget.ReasoningOracle.Triage.observe (patient)) (LexLeanTarget.ReasoningOracle.Triage.follow (patient) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((LexLeanTarget.ReasoningOracle.Triage.replay)) (fun (__acc : Except ((Prod Bool Bool)) (LexLeanTarget.ReasoningOracle.Chart)) => (LexLeanReasoning.Reaches ((LexLeanTarget.ReasoningOracle.Justified)) (LexLeanTarget.ReasoningOracle.Triage.observe (patient)) (__acc))) (LexLeanTarget.ReasoningOracle.Triage.replay_sound (patient)) (__trace) (Except.ok (LexLeanTarget.ReasoningOracle.Triage.observe (patient))) (LexLeanReasoning.reachesStart ((LexLeanTarget.ReasoningOracle.Justified)) (LexLeanTarget.ReasoningOracle.Triage.observe (patient))))
public theorem Triage.follow_invariant (patient : LexLeanTarget.ReasoningOracle.Vitals) (__trace : List (LexLeanTarget.ReasoningOracle.Triage.Step)) (__s : LexLeanTarget.ReasoningOracle.Chart) : ((LexLeanTarget.ReasoningOracle.Triage.follow (patient) (__trace) = Except.ok (__s)) -> LexLeanTarget.ReasoningOracle.Consistent (__s)) :=
  (fun llE => (LexLeanReasoning.starPreserves ((LexLeanTarget.ReasoningOracle.Justified)) ((LexLeanTarget.ReasoningOracle.Consistent)) LexLeanTarget.ReasoningOracle.Findings.preserves (LexLeanTarget.ReasoningOracle.Triage.observe (patient)) (__s) (LexLeanTarget.ReasoningOracle.Triage.derivation (patient) (__trace) (__s) llE) (LexLeanTarget.ReasoningOracle.admitted_consistent (patient))))
public theorem Triage.accept_sound (patient : LexLeanTarget.ReasoningOracle.Vitals) (__s : LexLeanTarget.ReasoningOracle.Chart) (__v : Nat) : ((LexLeanTarget.ReasoningOracle.Triage.accept (patient) (__s) = Option.some (__v)) -> LexLeanTarget.ReasoningOracle.Indicated (patient) (__v)) :=
by
  intro llE
  dsimp only [LexLeanTarget.ReasoningOracle.Triage.accept] at llE
  split at llE
  · cases llE
  · have llC := LexLeanReasoning.checked _ _ _ llE
    exact llC.right ▸ LexLeanTarget.ReasoningOracle.recommendation_sound _ _ llC.left
public theorem Triage.next_sound (__s : LexLeanTarget.ReasoningOracle.Chart) (__t : LexLeanTarget.ReasoningOracle.Chart) : ((LexLeanTarget.ReasoningOracle.Triage.next (__s) = Option.some (__t)) -> LexLeanTarget.ReasoningOracle.Justified (__s) (__t)) :=
by
  intro llE
  dsimp only [LexLeanTarget.ReasoningOracle.Triage.next] at llE
  split at llE
  · cases llE
  · exact LexLeanTarget.ReasoningOracle.Triage.fire_sound __s _ __t llE
public theorem Triage.next_preserves (__s : LexLeanTarget.ReasoningOracle.Chart) (__t : LexLeanTarget.ReasoningOracle.Chart) : ((LexLeanTarget.ReasoningOracle.Triage.next (__s) = Option.some (__t)) -> (LexLeanTarget.ReasoningOracle.Consistent (__s) -> LexLeanTarget.ReasoningOracle.Consistent (__t))) :=
  (fun llE llH => (LexLeanTarget.ReasoningOracle.consistent_preserved (__s) (__t) llH (LexLeanTarget.ReasoningOracle.Triage.next_sound (__s) (__t) llE)))
public theorem Triage.next_progress (__s : LexLeanTarget.ReasoningOracle.Chart) (__t : LexLeanTarget.ReasoningOracle.Chart) : ((LexLeanTarget.ReasoningOracle.Triage.next (__s) = Option.some (__t)) -> (LexLeanTarget.ReasoningOracle.Consistent (__s) -> (LexLeanTarget.ReasoningOracle.Pending (__t) < LexLeanTarget.ReasoningOracle.Pending (__s)))) :=
by
  intro llE
  dsimp only [LexLeanTarget.ReasoningOracle.Triage.next] at llE
  split at llE
  · cases llE
  · exact fun llJ => LexLeanTarget.ReasoningOracle.Triage.fire_progress __s _ __t llJ llE
public theorem Triage.step_none (__r : LexLeanTarget.ReasoningOracle.Triage.Run) : ((LexLeanTarget.ReasoningOracle.Triage.step (__r) = Option.none) -> (LexLeanTarget.ReasoningOracle.Triage.next ((__r).state) = Option.none)) :=
by
  intro llE
  dsimp only [LexLeanTarget.ReasoningOracle.Triage.step] at llE
  split at llE
  · rename_i llH
    dsimp only [LexLeanTarget.ReasoningOracle.Triage.next]
    rw [llH]
  · rename_i llH
    split at llE
    · dsimp only [LexLeanTarget.ReasoningOracle.Triage.next]
      rw [llH]
      assumption
    · cases llE
public theorem Triage.step_some (__r : LexLeanTarget.ReasoningOracle.Triage.Run) (__q : LexLeanTarget.ReasoningOracle.Triage.Run) : ((LexLeanTarget.ReasoningOracle.Triage.step (__r) = Option.some (__q)) -> (LexLeanTarget.ReasoningOracle.Triage.next ((__r).state) = Option.some ((__q).state))) :=
by
  intro llE
  dsimp only [LexLeanTarget.ReasoningOracle.Triage.step] at llE
  split at llE
  · cases llE
  · rename_i llH
    split at llE
    · cases llE
    · cases llE
      dsimp only [LexLeanTarget.ReasoningOracle.Triage.next]
      rw [llH]
      assumption
public theorem Triage.step_trace (patient : LexLeanTarget.ReasoningOracle.Vitals) (__r : LexLeanTarget.ReasoningOracle.Triage.Run) (__q : LexLeanTarget.ReasoningOracle.Triage.Run) : ((LexLeanTarget.ReasoningOracle.Triage.step (__r) = Option.some (__q)) -> ((LexLeanTarget.ReasoningOracle.Triage.follow (patient) ((__r).trace) = Except.ok ((__r).state)) -> (LexLeanTarget.ReasoningOracle.Triage.follow (patient) ((__q).trace) = Except.ok ((__q).state)))) :=
by
  intro llE llH
  dsimp only [LexLeanTarget.ReasoningOracle.Triage.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      dsimp only [LexLeanTarget.ReasoningOracle.Triage.follow]
      rw [LexLeanReasoning.foldSnoc]
      dsimp only [LexLeanTarget.ReasoningOracle.Triage.follow] at llH
      rw [llH]
      apply LexLeanTarget.ReasoningOracle.Triage.replay_fire
      assumption
public theorem Triage.step_count (__r : LexLeanTarget.ReasoningOracle.Triage.Run) (__q : LexLeanTarget.ReasoningOracle.Triage.Run) : ((LexLeanTarget.ReasoningOracle.Triage.step (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [LexLeanTarget.ReasoningOracle.Triage.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Triage.step_fired (__r : LexLeanTarget.ReasoningOracle.Triage.Run) (__q : LexLeanTarget.ReasoningOracle.Triage.Run) : ((LexLeanTarget.ReasoningOracle.Triage.step (__r) = Option.some (__q)) -> (((__q).ledger).firings = (((__r).ledger).firings + 1))) :=
by
  intro llE
  dsimp only [LexLeanTarget.ReasoningOracle.Triage.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Triage.saturate_derivation (patient : LexLeanTarget.ReasoningOracle.Vitals) : (LexLeanReasoning.Star ((LexLeanTarget.ReasoningOracle.Justified)) (LexLeanTarget.ReasoningOracle.Triage.observe (patient)) ((LexLeanTarget.ReasoningOracle.Triage.saturate (patient)).1)) :=
by
  dsimp only [LexLeanTarget.ReasoningOracle.Triage.saturate]
  exact (LexLeanReasoning.iterateUntilInvariant ((LexLeanTarget.ReasoningOracle.Triage.next)) (fun (__s : LexLeanTarget.ReasoningOracle.Chart) => (LexLeanReasoning.Star ((LexLeanTarget.ReasoningOracle.Justified)) (LexLeanTarget.ReasoningOracle.Triage.observe (patient)) (__s))) (fun llA llB llE llH => (LexLeanReasoning.Star.tail _ llA llB llH (LexLeanTarget.ReasoningOracle.Triage.next_sound llA llB llE))) (11) (LexLeanTarget.ReasoningOracle.Triage.observe (patient)) (LexLeanReasoning.Star.refl _))
public theorem Triage.run_state (patient : LexLeanTarget.ReasoningOracle.Vitals) : ((((LexLeanTarget.ReasoningOracle.Triage.run (patient)).1).state = (LexLeanTarget.ReasoningOracle.Triage.saturate (patient)).1) /\ ((LexLeanTarget.ReasoningOracle.Triage.run (patient)).2 = (LexLeanTarget.ReasoningOracle.Triage.saturate (patient)).2)) :=
by
  dsimp only [LexLeanTarget.ReasoningOracle.Triage.run, LexLeanTarget.ReasoningOracle.Triage.saturate]
  exact (LexLeanReasoning.iterateUntilSimulate ((LexLeanTarget.ReasoningOracle.Triage.step)) ((LexLeanTarget.ReasoningOracle.Triage.next)) ((fun (__r : LexLeanTarget.ReasoningOracle.Triage.Run) => (__r).state)) LexLeanTarget.ReasoningOracle.Triage.step_none LexLeanTarget.ReasoningOracle.Triage.step_some (11) (LexLeanTarget.ReasoningOracle.Triage.start (patient)))
public theorem Triage.run_trace (patient : LexLeanTarget.ReasoningOracle.Vitals) : (LexLeanTarget.ReasoningOracle.Triage.follow (patient) (((LexLeanTarget.ReasoningOracle.Triage.run (patient)).1).trace) = Except.ok (((LexLeanTarget.ReasoningOracle.Triage.run (patient)).1).state)) :=
by
  dsimp only [LexLeanTarget.ReasoningOracle.Triage.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((LexLeanTarget.ReasoningOracle.Triage.step)) (fun (__r : LexLeanTarget.ReasoningOracle.Triage.Run) => (LexLeanTarget.ReasoningOracle.Triage.follow (patient) ((__r).trace) = Except.ok ((__r).state))) (LexLeanTarget.ReasoningOracle.Triage.step_trace (patient)) (11) (LexLeanTarget.ReasoningOracle.Triage.start (patient)) rfl)
public theorem Triage.iterations_bounded (patient : LexLeanTarget.ReasoningOracle.Vitals) : ((((LexLeanTarget.ReasoningOracle.Triage.run (patient)).1).ledger).iterations <= 11) :=
by
  dsimp only [LexLeanTarget.ReasoningOracle.Triage.run]
  exact (LexLeanReasoning.iterateUntilCount ((LexLeanTarget.ReasoningOracle.Triage.step)) ((fun (__r : LexLeanTarget.ReasoningOracle.Triage.Run) => ((__r).ledger).iterations)) LexLeanTarget.ReasoningOracle.Triage.step_count (11) (LexLeanTarget.ReasoningOracle.Triage.start (patient)) rfl)
public theorem Triage.firings_bounded (patient : LexLeanTarget.ReasoningOracle.Vitals) : ((((LexLeanTarget.ReasoningOracle.Triage.run (patient)).1).ledger).firings <= 11) :=
by
  dsimp only [LexLeanTarget.ReasoningOracle.Triage.run]
  exact (LexLeanReasoning.iterateUntilCount ((LexLeanTarget.ReasoningOracle.Triage.step)) ((fun (__r : LexLeanTarget.ReasoningOracle.Triage.Run) => ((__r).ledger).firings)) LexLeanTarget.ReasoningOracle.Triage.step_fired (11) (LexLeanTarget.ReasoningOracle.Triage.start (patient)) rfl)
public theorem Triage.saturate_invariant (patient : LexLeanTarget.ReasoningOracle.Vitals) : LexLeanTarget.ReasoningOracle.Consistent ((LexLeanTarget.ReasoningOracle.Triage.saturate (patient)).1) :=
by
  dsimp only [LexLeanTarget.ReasoningOracle.Triage.saturate]
  exact (LexLeanReasoning.iterateUntilInvariant ((LexLeanTarget.ReasoningOracle.Triage.next)) ((LexLeanTarget.ReasoningOracle.Consistent)) LexLeanTarget.ReasoningOracle.Triage.next_preserves (11) (LexLeanTarget.ReasoningOracle.Triage.observe (patient)) (LexLeanTarget.ReasoningOracle.admitted_consistent (patient)))
public theorem Triage.saturates (patient : LexLeanTarget.ReasoningOracle.Vitals) : ((LexLeanTarget.ReasoningOracle.Triage.saturate (patient)).2 = true) :=
by
  dsimp only [LexLeanTarget.ReasoningOracle.Triage.saturate]
  exact (LexLeanReasoning.iterateUntilStops ((LexLeanTarget.ReasoningOracle.Triage.next)) ((LexLeanTarget.ReasoningOracle.Consistent)) ((LexLeanTarget.ReasoningOracle.Pending)) (fun llA llB llE llH => (And.intro (LexLeanTarget.ReasoningOracle.Triage.next_preserves llA llB llE llH) (LexLeanTarget.ReasoningOracle.Triage.next_progress llA llB llE llH))) (11) (LexLeanTarget.ReasoningOracle.Triage.observe (patient)) (LexLeanTarget.ReasoningOracle.admitted_consistent (patient)) (LexLeanTarget.ReasoningOracle.admitted_pending (patient)))
public theorem Triage.conclude_accept (patient : LexLeanTarget.ReasoningOracle.Vitals) (__s : LexLeanTarget.ReasoningOracle.Chart) (__v : Nat) : ((LexLeanTarget.ReasoningOracle.Triage.conclude (patient) (__s) = Except.ok (__v)) -> (LexLeanTarget.ReasoningOracle.Triage.accept (patient) (__s) = Option.some (__v))) :=
by
  intro llE
  dsimp only [LexLeanTarget.ReasoningOracle.Triage.conclude] at llE
  split at llE
  · cases llE
    assumption
  · split at llE
    · cases llE
    · cases llE
public theorem Triage.conclude_sound (patient : LexLeanTarget.ReasoningOracle.Vitals) (__s : LexLeanTarget.ReasoningOracle.Chart) (__v : Nat) : ((LexLeanTarget.ReasoningOracle.Triage.conclude (patient) (__s) = Except.ok (__v)) -> LexLeanTarget.ReasoningOracle.Indicated (patient) (__v)) :=
by
  intro llE
  dsimp only [LexLeanTarget.ReasoningOracle.Triage.conclude] at llE
  split at llE
  · cases llE
    exact LexLeanTarget.ReasoningOracle.Triage.accept_sound _ _ _ ‹_›
  · split at llE
    · cases llE
    · cases llE
public theorem Triage.verdict_sound (patient : LexLeanTarget.ReasoningOracle.Vitals) : (forall (__v : Nat), ((LexLeanTarget.ReasoningOracle.Triage.verdict (patient) = Except.ok (__v)) -> LexLeanTarget.ReasoningOracle.Indicated (patient) (__v))) :=
by
  intro llV llE
  dsimp only [LexLeanTarget.ReasoningOracle.Triage.verdict] at llE
  generalize llRun : LexLeanTarget.ReasoningOracle.Triage.saturate patient = llR at llE
  split at llE
  · exact LexLeanTarget.ReasoningOracle.Triage.conclude_sound _ _ _ llE
  · cases llE
public theorem Triage.explained (patient : LexLeanTarget.ReasoningOracle.Vitals) : (forall (__v : Nat), (forall (__trace : List (LexLeanTarget.ReasoningOracle.Triage.Step)), ((LexLeanTarget.ReasoningOracle.Triage (patient) = Except.ok ((__v, __trace))) -> ((LexLeanTarget.ReasoningOracle.Triage.answer (patient) (__trace) = Option.some (__v)) /\ LexLeanTarget.ReasoningOracle.Indicated (patient) (__v))))) :=
by
  intro llV llT llE
  have llTrace := LexLeanTarget.ReasoningOracle.Triage.run_trace patient
  dsimp only [LexLeanTarget.ReasoningOracle.Triage] at llE
  generalize llRun : LexLeanTarget.ReasoningOracle.Triage.run patient = llR at llE llTrace
  split at llE
  · split at llE
    · rename_i llW llH
      cases llE
      exact And.intro (by dsimp only [LexLeanTarget.ReasoningOracle.Triage.answer]; rw [llTrace]; exact LexLeanTarget.ReasoningOracle.Triage.conclude_accept _ _ _ llH) (LexLeanTarget.ReasoningOracle.Triage.conclude_sound _ _ _ llH)
    · cases llE
  · cases llE

public inductive Review.Step where
  | Fever
  | Tachycardia
  | Tachypnea
  | Leukocytosis
  | Sirs
  | Sepsis
  | Shock
  | Escalate
public structure Review.Ledger where
  iterations : Nat
  attempts : Nat
  firings : Nat
  expansions : Nat
  verifications : Nat
  frontier : Nat
@[expose] public def Review.observe (patient : LexLeanTarget.ReasoningOracle.Vitals) : LexLeanTarget.ReasoningOracle.Chart := ({ vitals := patient, fever := 0, tachycardia := 0, tachypnea := 0, leukocytosis := 0, sirs := 0, sepsis := 0, shock := 0, level := 0 } : LexLeanTarget.ReasoningOracle.Chart)
@[expose] public def Review.fire (__s : LexLeanTarget.ReasoningOracle.Chart) (__step : LexLeanTarget.ReasoningOracle.Review.Step) : Option (LexLeanTarget.ReasoningOracle.Chart) := (match __step with | LexLeanTarget.ReasoningOracle.Review.Step.Fever => LexLeanTarget.ReasoningOracle.Fever.apply (__s) | LexLeanTarget.ReasoningOracle.Review.Step.Tachycardia => LexLeanTarget.ReasoningOracle.Tachycardia.apply (__s) | LexLeanTarget.ReasoningOracle.Review.Step.Tachypnea => LexLeanTarget.ReasoningOracle.Tachypnea.apply (__s) | LexLeanTarget.ReasoningOracle.Review.Step.Leukocytosis => LexLeanTarget.ReasoningOracle.Leukocytosis.apply (__s) | LexLeanTarget.ReasoningOracle.Review.Step.Sirs => LexLeanTarget.ReasoningOracle.Sirs.apply (__s) | LexLeanTarget.ReasoningOracle.Review.Step.Sepsis => LexLeanTarget.ReasoningOracle.Sepsis.apply (__s) | LexLeanTarget.ReasoningOracle.Review.Step.Shock => LexLeanTarget.ReasoningOracle.Shock.apply (__s) | LexLeanTarget.ReasoningOracle.Review.Step.Escalate => LexLeanTarget.ReasoningOracle.Escalate.apply (__s))
@[expose] public def Review.replay (__acc : Except ((Prod Bool Bool)) (LexLeanTarget.ReasoningOracle.Chart)) (__step : LexLeanTarget.ReasoningOracle.Review.Step) : Except ((Prod Bool Bool)) (LexLeanTarget.ReasoningOracle.Chart) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match LexLeanTarget.ReasoningOracle.Review.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def Review.follow (patient : LexLeanTarget.ReasoningOracle.Vitals) (__trace : List (LexLeanTarget.ReasoningOracle.Review.Step)) : Except ((Prod Bool Bool)) (LexLeanTarget.ReasoningOracle.Chart) := (LexLeanCollections.listFold ((LexLeanTarget.ReasoningOracle.Review.replay)) (Except.ok (LexLeanTarget.ReasoningOracle.Review.observe (patient))) (__trace) : Except ((Prod Bool Bool)) (LexLeanTarget.ReasoningOracle.Chart))
@[expose] public def Review.extract (chart : LexLeanTarget.ReasoningOracle.Chart) : Option (Nat) := Option.some ((chart).level)
@[expose] public def Review.accept (patient : LexLeanTarget.ReasoningOracle.Vitals) (__s : LexLeanTarget.ReasoningOracle.Chart) : Option (Nat) := (match LexLeanTarget.ReasoningOracle.Review.extract (__s) with | Option.none => Option.none | Option.some __v => (if LexLeanTarget.ReasoningOracle.dischargeCheck (patient) (__v) then Option.some (__v) else Option.none))
@[expose] public def Review.answer (patient : LexLeanTarget.ReasoningOracle.Vitals) (__trace : List (LexLeanTarget.ReasoningOracle.Review.Step)) : Option (Nat) := (match LexLeanTarget.ReasoningOracle.Review.follow (patient) (__trace) with | Except.ok __s => LexLeanTarget.ReasoningOracle.Review.accept (patient) (__s) | Except.error _ => Option.none)
@[expose] public def Review.select (__s : LexLeanTarget.ReasoningOracle.Chart) : Option (LexLeanTarget.ReasoningOracle.Review.Step) := (match (match (match (if LexLeanTarget.ReasoningOracle.Fever.guard (__s) then Option.some (LexLeanTarget.ReasoningOracle.Review.Step.Fever) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if LexLeanTarget.ReasoningOracle.Tachycardia.guard (__s) then Option.some (LexLeanTarget.ReasoningOracle.Review.Step.Tachycardia) else Option.none)) with | Option.some __found => Option.some (__found) | Option.none => (match (if LexLeanTarget.ReasoningOracle.Tachypnea.guard (__s) then Option.some (LexLeanTarget.ReasoningOracle.Review.Step.Tachypnea) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if LexLeanTarget.ReasoningOracle.Leukocytosis.guard (__s) then Option.some (LexLeanTarget.ReasoningOracle.Review.Step.Leukocytosis) else Option.none))) with | Option.some __found => Option.some (__found) | Option.none => (match (match (if LexLeanTarget.ReasoningOracle.Sirs.guard (__s) then Option.some (LexLeanTarget.ReasoningOracle.Review.Step.Sirs) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if LexLeanTarget.ReasoningOracle.Sepsis.guard (__s) then Option.some (LexLeanTarget.ReasoningOracle.Review.Step.Sepsis) else Option.none)) with | Option.some __found => Option.some (__found) | Option.none => (match (if LexLeanTarget.ReasoningOracle.Shock.guard (__s) then Option.some (LexLeanTarget.ReasoningOracle.Review.Step.Shock) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if LexLeanTarget.ReasoningOracle.Escalate.guard (__s) then Option.some (LexLeanTarget.ReasoningOracle.Review.Step.Escalate) else Option.none))))
@[expose] public def Review.attempts (__s : LexLeanTarget.ReasoningOracle.Chart) : Nat := ((let __scan6l : (Prod (Bool) (Nat)) := (let __scan2l : (Prod (Bool) (Nat)) := (let __scan0l : (Prod (Bool) (Nat)) := (LexLeanTarget.ReasoningOracle.Fever.guard (__s), 1); (if (__scan0l).1 then __scan0l else (let __scan0r : (Prod (Bool) (Nat)) := (LexLeanTarget.ReasoningOracle.Tachycardia.guard (__s), 1); ((__scan0r).1, ((__scan0l).2 + (__scan0r).2))))); (if (__scan2l).1 then __scan2l else (let __scan2r : (Prod (Bool) (Nat)) := (let __scan1l : (Prod (Bool) (Nat)) := (LexLeanTarget.ReasoningOracle.Tachypnea.guard (__s), 1); (if (__scan1l).1 then __scan1l else (let __scan1r : (Prod (Bool) (Nat)) := (LexLeanTarget.ReasoningOracle.Leukocytosis.guard (__s), 1); ((__scan1r).1, ((__scan1l).2 + (__scan1r).2))))); ((__scan2r).1, ((__scan2l).2 + (__scan2r).2))))); (if (__scan6l).1 then __scan6l else (let __scan6r : (Prod (Bool) (Nat)) := (let __scan5l : (Prod (Bool) (Nat)) := (let __scan3l : (Prod (Bool) (Nat)) := (LexLeanTarget.ReasoningOracle.Sirs.guard (__s), 1); (if (__scan3l).1 then __scan3l else (let __scan3r : (Prod (Bool) (Nat)) := (LexLeanTarget.ReasoningOracle.Sepsis.guard (__s), 1); ((__scan3r).1, ((__scan3l).2 + (__scan3r).2))))); (if (__scan5l).1 then __scan5l else (let __scan5r : (Prod (Bool) (Nat)) := (let __scan4l : (Prod (Bool) (Nat)) := (LexLeanTarget.ReasoningOracle.Shock.guard (__s), 1); (if (__scan4l).1 then __scan4l else (let __scan4r : (Prod (Bool) (Nat)) := (LexLeanTarget.ReasoningOracle.Escalate.guard (__s), 1); ((__scan4r).1, ((__scan4l).2 + (__scan4r).2))))); ((__scan5r).1, ((__scan5l).2 + (__scan5r).2))))); ((__scan6r).1, ((__scan6l).2 + (__scan6r).2)))))).2
@[expose] public def Review.next (__s : LexLeanTarget.ReasoningOracle.Chart) : Option (LexLeanTarget.ReasoningOracle.Chart) := (match LexLeanTarget.ReasoningOracle.Review.select (__s) with | Option.none => Option.none | Option.some __step => LexLeanTarget.ReasoningOracle.Review.fire (__s) (__step))
@[expose] public def Review.saturate (patient : LexLeanTarget.ReasoningOracle.Vitals) : (Prod (LexLeanTarget.ReasoningOracle.Chart) (Bool)) := (LexLeanCollections.iterateUntil ((LexLeanTarget.ReasoningOracle.Review.next)) (6) (LexLeanTarget.ReasoningOracle.Review.observe (patient)) : (Prod (LexLeanTarget.ReasoningOracle.Chart) (Bool)))
public structure Review.Run where
  state : LexLeanTarget.ReasoningOracle.Chart
  trace : List (LexLeanTarget.ReasoningOracle.Review.Step)
  ledger : LexLeanTarget.ReasoningOracle.Review.Ledger
@[expose] public def Review.start (patient : LexLeanTarget.ReasoningOracle.Vitals) : LexLeanTarget.ReasoningOracle.Review.Run := ({ state := LexLeanTarget.ReasoningOracle.Review.observe (patient), trace := ([] : List (LexLeanTarget.ReasoningOracle.Review.Step)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := 0 } : LexLeanTarget.ReasoningOracle.Review.Ledger) } : LexLeanTarget.ReasoningOracle.Review.Run)
@[expose] public def Review.step (__r : LexLeanTarget.ReasoningOracle.Review.Run) : Option (LexLeanTarget.ReasoningOracle.Review.Run) := (match LexLeanTarget.ReasoningOracle.Review.select ((__r).state) with | Option.none => Option.none | Option.some __step => (match LexLeanTarget.ReasoningOracle.Review.fire ((__r).state) (__step) with | Option.none => Option.none | Option.some __t => Option.some (({ state := __t, trace := (LexLeanRuntime.append ((__r).trace) ((__step :: ([] : List (LexLeanTarget.ReasoningOracle.Review.Step)))) : List (LexLeanTarget.ReasoningOracle.Review.Step)), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + LexLeanTarget.ReasoningOracle.Review.attempts ((__r).state)), firings := (((__r).ledger).firings + 1), expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : LexLeanTarget.ReasoningOracle.Review.Ledger) } : LexLeanTarget.ReasoningOracle.Review.Run))))
@[expose] public def Review.run (patient : LexLeanTarget.ReasoningOracle.Vitals) : (Prod (LexLeanTarget.ReasoningOracle.Review.Run) (Bool)) := (LexLeanCollections.iterateUntil ((LexLeanTarget.ReasoningOracle.Review.step)) (6) (LexLeanTarget.ReasoningOracle.Review.start (patient)) : (Prod (LexLeanTarget.ReasoningOracle.Review.Run) (Bool)))
@[expose] public def Review.account (patient : LexLeanTarget.ReasoningOracle.Vitals) : LexLeanTarget.ReasoningOracle.Review.Ledger := (let __final : (Prod (LexLeanTarget.ReasoningOracle.Review.Run) (Bool)) := LexLeanTarget.ReasoningOracle.Review.run (patient); (match (__final).2 with | Bool.true => ({ iterations := (((__final).1).ledger).iterations, attempts := ((((__final).1).ledger).attempts + LexLeanTarget.ReasoningOracle.Review.attempts (((__final).1).state)), firings := (((__final).1).ledger).firings, expansions := (((__final).1).ledger).expansions, verifications := ((((__final).1).ledger).verifications + 1), frontier := (((__final).1).ledger).frontier } : LexLeanTarget.ReasoningOracle.Review.Ledger) | Bool.false => ((__final).1).ledger))
@[expose] public def Review.conclude (patient : LexLeanTarget.ReasoningOracle.Vitals) (__s : LexLeanTarget.ReasoningOracle.Chart) : Except ((Prod Bool Bool)) (Nat) := (match LexLeanTarget.ReasoningOracle.Review.accept (patient) (__s) with | Option.some __v => Except.ok (__v) | Option.none => (match LexLeanTarget.ReasoningOracle.Review.extract (__s) with | Option.some _ => Except.error (((true, false) : Prod Bool Bool)) | Option.none => Except.error (((false, true) : Prod Bool Bool))))
@[expose] public def Review.verdict (patient : LexLeanTarget.ReasoningOracle.Vitals) : Except ((Prod Bool Bool)) (Nat) := (let __final : (Prod (LexLeanTarget.ReasoningOracle.Chart) (Bool)) := LexLeanTarget.ReasoningOracle.Review.saturate (patient); (match (__final).2 with | Bool.true => LexLeanTarget.ReasoningOracle.Review.conclude (patient) ((__final).1) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
@[expose] public def Review (patient : LexLeanTarget.ReasoningOracle.Vitals) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (LexLeanTarget.ReasoningOracle.Review.Step)))) := (let __final : (Prod (LexLeanTarget.ReasoningOracle.Review.Run) (Bool)) := LexLeanTarget.ReasoningOracle.Review.run (patient); (match (__final).2 with | Bool.true => (match LexLeanTarget.ReasoningOracle.Review.conclude (patient) (((__final).1).state) with | Except.ok __v => Except.ok ((__v, ((__final).1).trace)) | Except.error __e => Except.error (__e)) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
public theorem Review.fire_sound (__s : LexLeanTarget.ReasoningOracle.Chart) (__step : LexLeanTarget.ReasoningOracle.Review.Step) (__t : LexLeanTarget.ReasoningOracle.Chart) : ((LexLeanTarget.ReasoningOracle.Review.fire (__s) (__step) = Option.some (__t)) -> LexLeanTarget.ReasoningOracle.Justified (__s) (__t)) :=
by
  cases __step with
  | Fever =>
    exact LexLeanTarget.ReasoningOracle.Fever.apply_sound __s __t
  | Tachycardia =>
    exact LexLeanTarget.ReasoningOracle.Tachycardia.apply_sound __s __t
  | Tachypnea =>
    exact LexLeanTarget.ReasoningOracle.Tachypnea.apply_sound __s __t
  | Leukocytosis =>
    exact LexLeanTarget.ReasoningOracle.Leukocytosis.apply_sound __s __t
  | Sirs =>
    exact LexLeanTarget.ReasoningOracle.Sirs.apply_sound __s __t
  | Sepsis =>
    exact LexLeanTarget.ReasoningOracle.Sepsis.apply_sound __s __t
  | Shock =>
    exact LexLeanTarget.ReasoningOracle.Shock.apply_sound __s __t
  | Escalate =>
    exact LexLeanTarget.ReasoningOracle.Escalate.apply_sound __s __t
public theorem Review.replay_fire (__s : LexLeanTarget.ReasoningOracle.Chart) (__step : LexLeanTarget.ReasoningOracle.Review.Step) (__t : LexLeanTarget.ReasoningOracle.Chart) : ((LexLeanTarget.ReasoningOracle.Review.fire (__s) (__step) = Option.some (__t)) -> (LexLeanTarget.ReasoningOracle.Review.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [LexLeanTarget.ReasoningOracle.Review.replay]
  rw [llE]
public theorem Review.replay_sound (patient : LexLeanTarget.ReasoningOracle.Vitals) (__acc : Except ((Prod Bool Bool)) (LexLeanTarget.ReasoningOracle.Chart)) (__step : LexLeanTarget.ReasoningOracle.Review.Step) : ((LexLeanReasoning.Reaches ((LexLeanTarget.ReasoningOracle.Justified)) (LexLeanTarget.ReasoningOracle.Review.observe (patient)) (__acc)) -> (LexLeanReasoning.Reaches ((LexLeanTarget.ReasoningOracle.Justified)) (LexLeanTarget.ReasoningOracle.Review.observe (patient)) (LexLeanTarget.ReasoningOracle.Review.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [LexLeanTarget.ReasoningOracle.Review.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (LexLeanTarget.ReasoningOracle.Review.fire_sound llS __step _ ‹_›)
public theorem Review.derivation (patient : LexLeanTarget.ReasoningOracle.Vitals) (__trace : List (LexLeanTarget.ReasoningOracle.Review.Step)) : (LexLeanReasoning.Reaches ((LexLeanTarget.ReasoningOracle.Justified)) (LexLeanTarget.ReasoningOracle.Review.observe (patient)) (LexLeanTarget.ReasoningOracle.Review.follow (patient) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((LexLeanTarget.ReasoningOracle.Review.replay)) (fun (__acc : Except ((Prod Bool Bool)) (LexLeanTarget.ReasoningOracle.Chart)) => (LexLeanReasoning.Reaches ((LexLeanTarget.ReasoningOracle.Justified)) (LexLeanTarget.ReasoningOracle.Review.observe (patient)) (__acc))) (LexLeanTarget.ReasoningOracle.Review.replay_sound (patient)) (__trace) (Except.ok (LexLeanTarget.ReasoningOracle.Review.observe (patient))) (LexLeanReasoning.reachesStart ((LexLeanTarget.ReasoningOracle.Justified)) (LexLeanTarget.ReasoningOracle.Review.observe (patient))))
public theorem Review.follow_invariant (patient : LexLeanTarget.ReasoningOracle.Vitals) (__trace : List (LexLeanTarget.ReasoningOracle.Review.Step)) (__s : LexLeanTarget.ReasoningOracle.Chart) : (LexLeanTarget.ReasoningOracle.Consistent (LexLeanTarget.ReasoningOracle.Review.observe (patient)) -> ((LexLeanTarget.ReasoningOracle.Review.follow (patient) (__trace) = Except.ok (__s)) -> LexLeanTarget.ReasoningOracle.Consistent (__s))) :=
  (fun llI llE => (LexLeanReasoning.starPreserves ((LexLeanTarget.ReasoningOracle.Justified)) ((LexLeanTarget.ReasoningOracle.Consistent)) LexLeanTarget.ReasoningOracle.Findings.preserves (LexLeanTarget.ReasoningOracle.Review.observe (patient)) (__s) (LexLeanTarget.ReasoningOracle.Review.derivation (patient) (__trace) (__s) llE) llI))
public theorem Review.accept_sound (patient : LexLeanTarget.ReasoningOracle.Vitals) (__s : LexLeanTarget.ReasoningOracle.Chart) (__v : Nat) : ((LexLeanTarget.ReasoningOracle.Review.accept (patient) (__s) = Option.some (__v)) -> LexLeanTarget.ReasoningOracle.Discharged (patient) (__v)) :=
by
  intro llE
  dsimp only [LexLeanTarget.ReasoningOracle.Review.accept] at llE
  split at llE
  · cases llE
  · have llC := LexLeanReasoning.checked _ _ _ llE
    exact llC.right ▸ LexLeanTarget.ReasoningOracle.discharge_sound _ _ llC.left
public theorem Review.next_sound (__s : LexLeanTarget.ReasoningOracle.Chart) (__t : LexLeanTarget.ReasoningOracle.Chart) : ((LexLeanTarget.ReasoningOracle.Review.next (__s) = Option.some (__t)) -> LexLeanTarget.ReasoningOracle.Justified (__s) (__t)) :=
by
  intro llE
  dsimp only [LexLeanTarget.ReasoningOracle.Review.next] at llE
  split at llE
  · cases llE
  · exact LexLeanTarget.ReasoningOracle.Review.fire_sound __s _ __t llE
public theorem Review.next_preserves (__s : LexLeanTarget.ReasoningOracle.Chart) (__t : LexLeanTarget.ReasoningOracle.Chart) : ((LexLeanTarget.ReasoningOracle.Review.next (__s) = Option.some (__t)) -> (LexLeanTarget.ReasoningOracle.Consistent (__s) -> LexLeanTarget.ReasoningOracle.Consistent (__t))) :=
  (fun llE llH => (LexLeanTarget.ReasoningOracle.consistent_preserved (__s) (__t) llH (LexLeanTarget.ReasoningOracle.Review.next_sound (__s) (__t) llE)))
public theorem Review.step_none (__r : LexLeanTarget.ReasoningOracle.Review.Run) : ((LexLeanTarget.ReasoningOracle.Review.step (__r) = Option.none) -> (LexLeanTarget.ReasoningOracle.Review.next ((__r).state) = Option.none)) :=
by
  intro llE
  dsimp only [LexLeanTarget.ReasoningOracle.Review.step] at llE
  split at llE
  · rename_i llH
    dsimp only [LexLeanTarget.ReasoningOracle.Review.next]
    rw [llH]
  · rename_i llH
    split at llE
    · dsimp only [LexLeanTarget.ReasoningOracle.Review.next]
      rw [llH]
      assumption
    · cases llE
public theorem Review.step_some (__r : LexLeanTarget.ReasoningOracle.Review.Run) (__q : LexLeanTarget.ReasoningOracle.Review.Run) : ((LexLeanTarget.ReasoningOracle.Review.step (__r) = Option.some (__q)) -> (LexLeanTarget.ReasoningOracle.Review.next ((__r).state) = Option.some ((__q).state))) :=
by
  intro llE
  dsimp only [LexLeanTarget.ReasoningOracle.Review.step] at llE
  split at llE
  · cases llE
  · rename_i llH
    split at llE
    · cases llE
    · cases llE
      dsimp only [LexLeanTarget.ReasoningOracle.Review.next]
      rw [llH]
      assumption
public theorem Review.step_trace (patient : LexLeanTarget.ReasoningOracle.Vitals) (__r : LexLeanTarget.ReasoningOracle.Review.Run) (__q : LexLeanTarget.ReasoningOracle.Review.Run) : ((LexLeanTarget.ReasoningOracle.Review.step (__r) = Option.some (__q)) -> ((LexLeanTarget.ReasoningOracle.Review.follow (patient) ((__r).trace) = Except.ok ((__r).state)) -> (LexLeanTarget.ReasoningOracle.Review.follow (patient) ((__q).trace) = Except.ok ((__q).state)))) :=
by
  intro llE llH
  dsimp only [LexLeanTarget.ReasoningOracle.Review.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      dsimp only [LexLeanTarget.ReasoningOracle.Review.follow]
      rw [LexLeanReasoning.foldSnoc]
      dsimp only [LexLeanTarget.ReasoningOracle.Review.follow] at llH
      rw [llH]
      apply LexLeanTarget.ReasoningOracle.Review.replay_fire
      assumption
public theorem Review.step_count (__r : LexLeanTarget.ReasoningOracle.Review.Run) (__q : LexLeanTarget.ReasoningOracle.Review.Run) : ((LexLeanTarget.ReasoningOracle.Review.step (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [LexLeanTarget.ReasoningOracle.Review.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Review.step_fired (__r : LexLeanTarget.ReasoningOracle.Review.Run) (__q : LexLeanTarget.ReasoningOracle.Review.Run) : ((LexLeanTarget.ReasoningOracle.Review.step (__r) = Option.some (__q)) -> (((__q).ledger).firings = (((__r).ledger).firings + 1))) :=
by
  intro llE
  dsimp only [LexLeanTarget.ReasoningOracle.Review.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Review.saturate_derivation (patient : LexLeanTarget.ReasoningOracle.Vitals) : (LexLeanReasoning.Star ((LexLeanTarget.ReasoningOracle.Justified)) (LexLeanTarget.ReasoningOracle.Review.observe (patient)) ((LexLeanTarget.ReasoningOracle.Review.saturate (patient)).1)) :=
by
  dsimp only [LexLeanTarget.ReasoningOracle.Review.saturate]
  exact (LexLeanReasoning.iterateUntilInvariant ((LexLeanTarget.ReasoningOracle.Review.next)) (fun (__s : LexLeanTarget.ReasoningOracle.Chart) => (LexLeanReasoning.Star ((LexLeanTarget.ReasoningOracle.Justified)) (LexLeanTarget.ReasoningOracle.Review.observe (patient)) (__s))) (fun llA llB llE llH => (LexLeanReasoning.Star.tail _ llA llB llH (LexLeanTarget.ReasoningOracle.Review.next_sound llA llB llE))) (6) (LexLeanTarget.ReasoningOracle.Review.observe (patient)) (LexLeanReasoning.Star.refl _))
public theorem Review.run_state (patient : LexLeanTarget.ReasoningOracle.Vitals) : ((((LexLeanTarget.ReasoningOracle.Review.run (patient)).1).state = (LexLeanTarget.ReasoningOracle.Review.saturate (patient)).1) /\ ((LexLeanTarget.ReasoningOracle.Review.run (patient)).2 = (LexLeanTarget.ReasoningOracle.Review.saturate (patient)).2)) :=
by
  dsimp only [LexLeanTarget.ReasoningOracle.Review.run, LexLeanTarget.ReasoningOracle.Review.saturate]
  exact (LexLeanReasoning.iterateUntilSimulate ((LexLeanTarget.ReasoningOracle.Review.step)) ((LexLeanTarget.ReasoningOracle.Review.next)) ((fun (__r : LexLeanTarget.ReasoningOracle.Review.Run) => (__r).state)) LexLeanTarget.ReasoningOracle.Review.step_none LexLeanTarget.ReasoningOracle.Review.step_some (6) (LexLeanTarget.ReasoningOracle.Review.start (patient)))
public theorem Review.run_trace (patient : LexLeanTarget.ReasoningOracle.Vitals) : (LexLeanTarget.ReasoningOracle.Review.follow (patient) (((LexLeanTarget.ReasoningOracle.Review.run (patient)).1).trace) = Except.ok (((LexLeanTarget.ReasoningOracle.Review.run (patient)).1).state)) :=
by
  dsimp only [LexLeanTarget.ReasoningOracle.Review.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((LexLeanTarget.ReasoningOracle.Review.step)) (fun (__r : LexLeanTarget.ReasoningOracle.Review.Run) => (LexLeanTarget.ReasoningOracle.Review.follow (patient) ((__r).trace) = Except.ok ((__r).state))) (LexLeanTarget.ReasoningOracle.Review.step_trace (patient)) (6) (LexLeanTarget.ReasoningOracle.Review.start (patient)) rfl)
public theorem Review.iterations_bounded (patient : LexLeanTarget.ReasoningOracle.Vitals) : ((((LexLeanTarget.ReasoningOracle.Review.run (patient)).1).ledger).iterations <= 6) :=
by
  dsimp only [LexLeanTarget.ReasoningOracle.Review.run]
  exact (LexLeanReasoning.iterateUntilCount ((LexLeanTarget.ReasoningOracle.Review.step)) ((fun (__r : LexLeanTarget.ReasoningOracle.Review.Run) => ((__r).ledger).iterations)) LexLeanTarget.ReasoningOracle.Review.step_count (6) (LexLeanTarget.ReasoningOracle.Review.start (patient)) rfl)
public theorem Review.firings_bounded (patient : LexLeanTarget.ReasoningOracle.Vitals) : ((((LexLeanTarget.ReasoningOracle.Review.run (patient)).1).ledger).firings <= 6) :=
by
  dsimp only [LexLeanTarget.ReasoningOracle.Review.run]
  exact (LexLeanReasoning.iterateUntilCount ((LexLeanTarget.ReasoningOracle.Review.step)) ((fun (__r : LexLeanTarget.ReasoningOracle.Review.Run) => ((__r).ledger).firings)) LexLeanTarget.ReasoningOracle.Review.step_fired (6) (LexLeanTarget.ReasoningOracle.Review.start (patient)) rfl)
public theorem Review.saturate_invariant (patient : LexLeanTarget.ReasoningOracle.Vitals) : (LexLeanTarget.ReasoningOracle.Consistent (LexLeanTarget.ReasoningOracle.Review.observe (patient)) -> LexLeanTarget.ReasoningOracle.Consistent ((LexLeanTarget.ReasoningOracle.Review.saturate (patient)).1)) :=
by
  dsimp only [LexLeanTarget.ReasoningOracle.Review.saturate]
  exact (fun llI => (LexLeanReasoning.iterateUntilInvariant ((LexLeanTarget.ReasoningOracle.Review.next)) ((LexLeanTarget.ReasoningOracle.Consistent)) LexLeanTarget.ReasoningOracle.Review.next_preserves (6) (LexLeanTarget.ReasoningOracle.Review.observe (patient)) llI))
public theorem Review.conclude_accept (patient : LexLeanTarget.ReasoningOracle.Vitals) (__s : LexLeanTarget.ReasoningOracle.Chart) (__v : Nat) : ((LexLeanTarget.ReasoningOracle.Review.conclude (patient) (__s) = Except.ok (__v)) -> (LexLeanTarget.ReasoningOracle.Review.accept (patient) (__s) = Option.some (__v))) :=
by
  intro llE
  dsimp only [LexLeanTarget.ReasoningOracle.Review.conclude] at llE
  split at llE
  · cases llE
    assumption
  · split at llE
    · cases llE
    · cases llE
public theorem Review.conclude_sound (patient : LexLeanTarget.ReasoningOracle.Vitals) (__s : LexLeanTarget.ReasoningOracle.Chart) (__v : Nat) : ((LexLeanTarget.ReasoningOracle.Review.conclude (patient) (__s) = Except.ok (__v)) -> LexLeanTarget.ReasoningOracle.Discharged (patient) (__v)) :=
by
  intro llE
  dsimp only [LexLeanTarget.ReasoningOracle.Review.conclude] at llE
  split at llE
  · cases llE
    exact LexLeanTarget.ReasoningOracle.Review.accept_sound _ _ _ ‹_›
  · split at llE
    · cases llE
    · cases llE
public theorem Review.verdict_sound (patient : LexLeanTarget.ReasoningOracle.Vitals) : (forall (__v : Nat), ((LexLeanTarget.ReasoningOracle.Review.verdict (patient) = Except.ok (__v)) -> LexLeanTarget.ReasoningOracle.Discharged (patient) (__v))) :=
by
  intro llV llE
  dsimp only [LexLeanTarget.ReasoningOracle.Review.verdict] at llE
  generalize llRun : LexLeanTarget.ReasoningOracle.Review.saturate patient = llR at llE
  split at llE
  · exact LexLeanTarget.ReasoningOracle.Review.conclude_sound _ _ _ llE
  · cases llE
public theorem Review.explained (patient : LexLeanTarget.ReasoningOracle.Vitals) : (forall (__v : Nat), (forall (__trace : List (LexLeanTarget.ReasoningOracle.Review.Step)), ((LexLeanTarget.ReasoningOracle.Review (patient) = Except.ok ((__v, __trace))) -> ((LexLeanTarget.ReasoningOracle.Review.answer (patient) (__trace) = Option.some (__v)) /\ LexLeanTarget.ReasoningOracle.Discharged (patient) (__v))))) :=
by
  intro llV llT llE
  have llTrace := LexLeanTarget.ReasoningOracle.Review.run_trace patient
  dsimp only [LexLeanTarget.ReasoningOracle.Review] at llE
  generalize llRun : LexLeanTarget.ReasoningOracle.Review.run patient = llR at llE llTrace
  split at llE
  · split at llE
    · rename_i llW llH
      cases llE
      exact And.intro (by dsimp only [LexLeanTarget.ReasoningOracle.Review.answer]; rw [llTrace]; exact LexLeanTarget.ReasoningOracle.Review.conclude_accept _ _ _ llH) (LexLeanTarget.ReasoningOracle.Review.conclude_sound _ _ _ llH)
    · cases llE
  · cases llE

end LexLeanTarget.ReasoningOracle
