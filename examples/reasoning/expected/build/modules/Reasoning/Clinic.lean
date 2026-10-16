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
namespace Reasoning.Clinic

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

public theorem Findings.preserves : (LexLeanReasoning.Preserves ((Reasoning.Clinic.Justified)) ((Reasoning.Clinic.Consistent))) :=
  Reasoning.Clinic.consistent_preserved

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

@[expose] public def Fever.guard (s : Reasoning.Clinic.Chart) : Bool := ((Nat.beq ((s).fever) (0)) && Reasoning.Clinic.feverish ((s).vitals))
@[expose] public def Fever.conclusion (s : Reasoning.Clinic.Chart) : Reasoning.Clinic.Chart := ({ vitals := (s).vitals, fever := 1, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Reasoning.Clinic.Chart)
@[expose] public def Fever.apply (s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Chart) := (if Reasoning.Clinic.Fever.guard (s) then Option.some (Reasoning.Clinic.Fever.conclusion (s)) else Option.none)
public theorem Fever.apply_sound (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Fever.apply (s) = Option.some (__t)) -> Reasoning.Clinic.Justified (s) (__t)) :=
  (LexLeanReasoning.guarded ((Reasoning.Clinic.Justified)) (s) (Reasoning.Clinic.Fever.guard (s)) (Reasoning.Clinic.Fever.conclusion (s)) (Reasoning.Clinic.fever_sound (s)) (__t))
public theorem Fever.apply_progress (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : (Reasoning.Clinic.Consistent (s) -> ((Reasoning.Clinic.Fever.apply (s) = Option.some (__t)) -> (Reasoning.Clinic.Pending (__t) < Reasoning.Clinic.Pending (s)))) :=
  (fun llJ => (LexLeanReasoning.guardedRank ((Reasoning.Clinic.Pending)) (s) (Reasoning.Clinic.Fever.guard (s)) (Reasoning.Clinic.Fever.conclusion (s)) (Reasoning.Clinic.fever_progress (s) llJ) (__t)))

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

@[expose] public def Tachycardia.guard (s : Reasoning.Clinic.Chart) : Bool := ((Nat.beq ((s).tachycardia) (0)) && Reasoning.Clinic.tachycardic ((s).vitals))
@[expose] public def Tachycardia.conclusion (s : Reasoning.Clinic.Chart) : Reasoning.Clinic.Chart := ({ vitals := (s).vitals, fever := (s).fever, tachycardia := 1, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Reasoning.Clinic.Chart)
@[expose] public def Tachycardia.apply (s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Chart) := (if Reasoning.Clinic.Tachycardia.guard (s) then Option.some (Reasoning.Clinic.Tachycardia.conclusion (s)) else Option.none)
public theorem Tachycardia.apply_sound (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Tachycardia.apply (s) = Option.some (__t)) -> Reasoning.Clinic.Justified (s) (__t)) :=
  (LexLeanReasoning.guarded ((Reasoning.Clinic.Justified)) (s) (Reasoning.Clinic.Tachycardia.guard (s)) (Reasoning.Clinic.Tachycardia.conclusion (s)) (Reasoning.Clinic.tachycardia_sound (s)) (__t))
public theorem Tachycardia.apply_progress (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : (Reasoning.Clinic.Consistent (s) -> ((Reasoning.Clinic.Tachycardia.apply (s) = Option.some (__t)) -> (Reasoning.Clinic.Pending (__t) < Reasoning.Clinic.Pending (s)))) :=
  (fun llJ => (LexLeanReasoning.guardedRank ((Reasoning.Clinic.Pending)) (s) (Reasoning.Clinic.Tachycardia.guard (s)) (Reasoning.Clinic.Tachycardia.conclusion (s)) (Reasoning.Clinic.tachycardia_progress (s) llJ) (__t)))

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

@[expose] public def Tachypnea.guard (s : Reasoning.Clinic.Chart) : Bool := ((Nat.beq ((s).tachypnea) (0)) && Reasoning.Clinic.tachypneic ((s).vitals))
@[expose] public def Tachypnea.conclusion (s : Reasoning.Clinic.Chart) : Reasoning.Clinic.Chart := ({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := 1, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Reasoning.Clinic.Chart)
@[expose] public def Tachypnea.apply (s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Chart) := (if Reasoning.Clinic.Tachypnea.guard (s) then Option.some (Reasoning.Clinic.Tachypnea.conclusion (s)) else Option.none)
public theorem Tachypnea.apply_sound (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Tachypnea.apply (s) = Option.some (__t)) -> Reasoning.Clinic.Justified (s) (__t)) :=
  (LexLeanReasoning.guarded ((Reasoning.Clinic.Justified)) (s) (Reasoning.Clinic.Tachypnea.guard (s)) (Reasoning.Clinic.Tachypnea.conclusion (s)) (Reasoning.Clinic.tachypnea_sound (s)) (__t))
public theorem Tachypnea.apply_progress (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : (Reasoning.Clinic.Consistent (s) -> ((Reasoning.Clinic.Tachypnea.apply (s) = Option.some (__t)) -> (Reasoning.Clinic.Pending (__t) < Reasoning.Clinic.Pending (s)))) :=
  (fun llJ => (LexLeanReasoning.guardedRank ((Reasoning.Clinic.Pending)) (s) (Reasoning.Clinic.Tachypnea.guard (s)) (Reasoning.Clinic.Tachypnea.conclusion (s)) (Reasoning.Clinic.tachypnea_progress (s) llJ) (__t)))

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

@[expose] public def Leukocytosis.guard (s : Reasoning.Clinic.Chart) : Bool := ((Nat.beq ((s).leukocytosis) (0)) && Reasoning.Clinic.leukocytic ((s).vitals))
@[expose] public def Leukocytosis.conclusion (s : Reasoning.Clinic.Chart) : Reasoning.Clinic.Chart := ({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := 1, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Reasoning.Clinic.Chart)
@[expose] public def Leukocytosis.apply (s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Chart) := (if Reasoning.Clinic.Leukocytosis.guard (s) then Option.some (Reasoning.Clinic.Leukocytosis.conclusion (s)) else Option.none)
public theorem Leukocytosis.apply_sound (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Leukocytosis.apply (s) = Option.some (__t)) -> Reasoning.Clinic.Justified (s) (__t)) :=
  (LexLeanReasoning.guarded ((Reasoning.Clinic.Justified)) (s) (Reasoning.Clinic.Leukocytosis.guard (s)) (Reasoning.Clinic.Leukocytosis.conclusion (s)) (Reasoning.Clinic.leukocytosis_sound (s)) (__t))
public theorem Leukocytosis.apply_progress (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : (Reasoning.Clinic.Consistent (s) -> ((Reasoning.Clinic.Leukocytosis.apply (s) = Option.some (__t)) -> (Reasoning.Clinic.Pending (__t) < Reasoning.Clinic.Pending (s)))) :=
  (fun llJ => (LexLeanReasoning.guardedRank ((Reasoning.Clinic.Pending)) (s) (Reasoning.Clinic.Leukocytosis.guard (s)) (Reasoning.Clinic.Leukocytosis.conclusion (s)) (Reasoning.Clinic.leukocytosis_progress (s) llJ) (__t)))

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

@[expose] public def Sirs.guard (s : Reasoning.Clinic.Chart) : Bool := ((Nat.beq ((s).sirs) (0)) && (Nat.ble (2) (((((s).fever + (s).tachycardia) + (s).tachypnea) + (s).leukocytosis))))
@[expose] public def Sirs.conclusion (s : Reasoning.Clinic.Chart) : Reasoning.Clinic.Chart := ({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := 1, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Reasoning.Clinic.Chart)
@[expose] public def Sirs.apply (s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Chart) := (if Reasoning.Clinic.Sirs.guard (s) then Option.some (Reasoning.Clinic.Sirs.conclusion (s)) else Option.none)
public theorem Sirs.apply_sound (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Sirs.apply (s) = Option.some (__t)) -> Reasoning.Clinic.Justified (s) (__t)) :=
  (LexLeanReasoning.guarded ((Reasoning.Clinic.Justified)) (s) (Reasoning.Clinic.Sirs.guard (s)) (Reasoning.Clinic.Sirs.conclusion (s)) (Reasoning.Clinic.sirs_sound (s)) (__t))
public theorem Sirs.apply_progress (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : (Reasoning.Clinic.Consistent (s) -> ((Reasoning.Clinic.Sirs.apply (s) = Option.some (__t)) -> (Reasoning.Clinic.Pending (__t) < Reasoning.Clinic.Pending (s)))) :=
  (fun llJ => (LexLeanReasoning.guardedRank ((Reasoning.Clinic.Pending)) (s) (Reasoning.Clinic.Sirs.guard (s)) (Reasoning.Clinic.Sirs.conclusion (s)) (Reasoning.Clinic.sirs_progress (s) llJ) (__t)))

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

@[expose] public def Sepsis.guard (s : Reasoning.Clinic.Chart) : Bool := ((Nat.beq ((s).sepsis) (0)) && ((Nat.beq ((s).sirs) (1)) && (Nat.beq (((s).vitals).infection) (1))))
@[expose] public def Sepsis.conclusion (s : Reasoning.Clinic.Chart) : Reasoning.Clinic.Chart := ({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := 1, shock := (s).shock, level := (s).level } : Reasoning.Clinic.Chart)
@[expose] public def Sepsis.apply (s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Chart) := (if Reasoning.Clinic.Sepsis.guard (s) then Option.some (Reasoning.Clinic.Sepsis.conclusion (s)) else Option.none)
public theorem Sepsis.apply_sound (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Sepsis.apply (s) = Option.some (__t)) -> Reasoning.Clinic.Justified (s) (__t)) :=
  (LexLeanReasoning.guarded ((Reasoning.Clinic.Justified)) (s) (Reasoning.Clinic.Sepsis.guard (s)) (Reasoning.Clinic.Sepsis.conclusion (s)) (Reasoning.Clinic.sepsis_sound (s)) (__t))
public theorem Sepsis.apply_progress (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : (Reasoning.Clinic.Consistent (s) -> ((Reasoning.Clinic.Sepsis.apply (s) = Option.some (__t)) -> (Reasoning.Clinic.Pending (__t) < Reasoning.Clinic.Pending (s)))) :=
  (fun llJ => (LexLeanReasoning.guardedRank ((Reasoning.Clinic.Pending)) (s) (Reasoning.Clinic.Sepsis.guard (s)) (Reasoning.Clinic.Sepsis.conclusion (s)) (Reasoning.Clinic.sepsis_progress (s) llJ) (__t)))

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

@[expose] public def Shock.guard (s : Reasoning.Clinic.Chart) : Bool := ((Nat.beq ((s).shock) (0)) && ((Nat.beq ((s).sepsis) (1)) && Reasoning.Clinic.hypotensive ((s).vitals)))
@[expose] public def Shock.conclusion (s : Reasoning.Clinic.Chart) : Reasoning.Clinic.Chart := ({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := 1, level := (s).level } : Reasoning.Clinic.Chart)
@[expose] public def Shock.apply (s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Chart) := (if Reasoning.Clinic.Shock.guard (s) then Option.some (Reasoning.Clinic.Shock.conclusion (s)) else Option.none)
public theorem Shock.apply_sound (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Shock.apply (s) = Option.some (__t)) -> Reasoning.Clinic.Justified (s) (__t)) :=
  (LexLeanReasoning.guarded ((Reasoning.Clinic.Justified)) (s) (Reasoning.Clinic.Shock.guard (s)) (Reasoning.Clinic.Shock.conclusion (s)) (Reasoning.Clinic.shock_sound (s)) (__t))
public theorem Shock.apply_progress (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : (Reasoning.Clinic.Consistent (s) -> ((Reasoning.Clinic.Shock.apply (s) = Option.some (__t)) -> (Reasoning.Clinic.Pending (__t) < Reasoning.Clinic.Pending (s)))) :=
  (fun llJ => (LexLeanReasoning.guardedRank ((Reasoning.Clinic.Pending)) (s) (Reasoning.Clinic.Shock.guard (s)) (Reasoning.Clinic.Shock.conclusion (s)) (Reasoning.Clinic.shock_progress (s) llJ) (__t)))

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

@[expose] public def Escalate.guard (s : Reasoning.Clinic.Chart) : Bool := ((Nat.blt ((s).level) (3)) && (((Nat.beq ((s).level) (0)) && (Nat.beq ((s).sirs) (1))) || (((Nat.beq ((s).level) (1)) && (Nat.beq ((s).sepsis) (1))) || ((Nat.beq ((s).level) (2)) && (Nat.beq ((s).shock) (1))))))
@[expose] public def Escalate.conclusion (s : Reasoning.Clinic.Chart) : Reasoning.Clinic.Chart := ({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := ((s).level + 1) } : Reasoning.Clinic.Chart)
@[expose] public def Escalate.apply (s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Chart) := (if Reasoning.Clinic.Escalate.guard (s) then Option.some (Reasoning.Clinic.Escalate.conclusion (s)) else Option.none)
public theorem Escalate.apply_sound (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Escalate.apply (s) = Option.some (__t)) -> Reasoning.Clinic.Justified (s) (__t)) :=
  (LexLeanReasoning.guarded ((Reasoning.Clinic.Justified)) (s) (Reasoning.Clinic.Escalate.guard (s)) (Reasoning.Clinic.Escalate.conclusion (s)) (Reasoning.Clinic.escalate_sound (s)) (__t))
public theorem Escalate.apply_progress (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : (Reasoning.Clinic.Consistent (s) -> ((Reasoning.Clinic.Escalate.apply (s) = Option.some (__t)) -> (Reasoning.Clinic.Pending (__t) < Reasoning.Clinic.Pending (s)))) :=
  (fun llJ => (LexLeanReasoning.guardedRank ((Reasoning.Clinic.Pending)) (s) (Reasoning.Clinic.Escalate.guard (s)) (Reasoning.Clinic.Escalate.conclusion (s)) (Reasoning.Clinic.escalate_progress (s) llJ) (__t)))

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

public theorem Recommendation.sound : (LexLeanReasoning.Sound ((Reasoning.Clinic.recommendationCheck)) ((Reasoning.Clinic.Indicated))) :=
  Reasoning.Clinic.recommendation_sound
public theorem Recommendation.complete : (LexLeanReasoning.Complete ((Reasoning.Clinic.recommendationCheck)) ((Reasoning.Clinic.Indicated))) :=
  Reasoning.Clinic.recommendation_complete

@[expose, reducible] public def Discharged (_v : Vitals) (r : Nat) : Prop := (r = 0)

@[expose] public def dischargeCheck (_v : Vitals) (r : Nat) : Bool := (Nat.beq (r) (0))

public theorem discharge_sound (v : Vitals) (r : Nat) : ((dischargeCheck (v) (r) = true) -> Discharged (v) (r)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Discharged, dischargeCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem Outpatient.sound : (LexLeanReasoning.Sound ((Reasoning.Clinic.dischargeCheck)) ((Reasoning.Clinic.Discharged))) :=
  Reasoning.Clinic.discharge_sound

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
@[expose] public def Triage.observe (patient : Reasoning.Clinic.Vitals) : Reasoning.Clinic.Chart := ({ vitals := patient, fever := 0, tachycardia := 0, tachypnea := 0, leukocytosis := 0, sirs := 0, sepsis := 0, shock := 0, level := 0 } : Reasoning.Clinic.Chart)
@[expose] public def Triage.fire (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Clinic.Triage.Step) : Option (Reasoning.Clinic.Chart) := (match __step with | Reasoning.Clinic.Triage.Step.Fever => Reasoning.Clinic.Fever.apply (__s) | Reasoning.Clinic.Triage.Step.Tachycardia => Reasoning.Clinic.Tachycardia.apply (__s) | Reasoning.Clinic.Triage.Step.Tachypnea => Reasoning.Clinic.Tachypnea.apply (__s) | Reasoning.Clinic.Triage.Step.Leukocytosis => Reasoning.Clinic.Leukocytosis.apply (__s) | Reasoning.Clinic.Triage.Step.Sirs => Reasoning.Clinic.Sirs.apply (__s) | Reasoning.Clinic.Triage.Step.Sepsis => Reasoning.Clinic.Sepsis.apply (__s) | Reasoning.Clinic.Triage.Step.Shock => Reasoning.Clinic.Shock.apply (__s) | Reasoning.Clinic.Triage.Step.Escalate => Reasoning.Clinic.Escalate.apply (__s))
@[expose] public def Triage.replay (__acc : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart)) (__step : Reasoning.Clinic.Triage.Step) : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match Reasoning.Clinic.Triage.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def Triage.follow (patient : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Clinic.Triage.Step)) : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart) := (LexLeanCollections.listFold ((Reasoning.Clinic.Triage.replay)) (Except.ok (Reasoning.Clinic.Triage.observe (patient))) (__trace) : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart))
@[expose] public def Triage.extract (chart : Reasoning.Clinic.Chart) : Option (Nat) := Option.some ((chart).level)
@[expose] public def Triage.accept (patient : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) : Option (Nat) := (match Reasoning.Clinic.Triage.extract (__s) with | Option.none => Option.none | Option.some __v => (if Reasoning.Clinic.recommendationCheck (patient) (__v) then Option.some (__v) else Option.none))
@[expose] public def Triage.answer (patient : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Clinic.Triage.Step)) : Option (Nat) := (match Reasoning.Clinic.Triage.follow (patient) (__trace) with | Except.ok __s => Reasoning.Clinic.Triage.accept (patient) (__s) | Except.error _ => Option.none)
@[expose] public def Triage.select (__s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Triage.Step) := (match (match (match (if Reasoning.Clinic.Fever.guard (__s) then Option.some (Reasoning.Clinic.Triage.Step.Fever) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Tachycardia.guard (__s) then Option.some (Reasoning.Clinic.Triage.Step.Tachycardia) else Option.none)) with | Option.some __found => Option.some (__found) | Option.none => (match (if Reasoning.Clinic.Tachypnea.guard (__s) then Option.some (Reasoning.Clinic.Triage.Step.Tachypnea) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Leukocytosis.guard (__s) then Option.some (Reasoning.Clinic.Triage.Step.Leukocytosis) else Option.none))) with | Option.some __found => Option.some (__found) | Option.none => (match (match (if Reasoning.Clinic.Sirs.guard (__s) then Option.some (Reasoning.Clinic.Triage.Step.Sirs) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Sepsis.guard (__s) then Option.some (Reasoning.Clinic.Triage.Step.Sepsis) else Option.none)) with | Option.some __found => Option.some (__found) | Option.none => (match (if Reasoning.Clinic.Shock.guard (__s) then Option.some (Reasoning.Clinic.Triage.Step.Shock) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Escalate.guard (__s) then Option.some (Reasoning.Clinic.Triage.Step.Escalate) else Option.none))))
@[expose] public def Triage.attempts (__s : Reasoning.Clinic.Chart) : Nat := ((let __scan6l : (Prod (Bool) (Nat)) := (let __scan2l : (Prod (Bool) (Nat)) := (let __scan0l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Fever.guard (__s), 1); (if (__scan0l).1 then __scan0l else (let __scan0r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Tachycardia.guard (__s), 1); ((__scan0r).1, ((__scan0l).2 + (__scan0r).2))))); (if (__scan2l).1 then __scan2l else (let __scan2r : (Prod (Bool) (Nat)) := (let __scan1l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Tachypnea.guard (__s), 1); (if (__scan1l).1 then __scan1l else (let __scan1r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Leukocytosis.guard (__s), 1); ((__scan1r).1, ((__scan1l).2 + (__scan1r).2))))); ((__scan2r).1, ((__scan2l).2 + (__scan2r).2))))); (if (__scan6l).1 then __scan6l else (let __scan6r : (Prod (Bool) (Nat)) := (let __scan5l : (Prod (Bool) (Nat)) := (let __scan3l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Sirs.guard (__s), 1); (if (__scan3l).1 then __scan3l else (let __scan3r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Sepsis.guard (__s), 1); ((__scan3r).1, ((__scan3l).2 + (__scan3r).2))))); (if (__scan5l).1 then __scan5l else (let __scan5r : (Prod (Bool) (Nat)) := (let __scan4l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Shock.guard (__s), 1); (if (__scan4l).1 then __scan4l else (let __scan4r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Escalate.guard (__s), 1); ((__scan4r).1, ((__scan4l).2 + (__scan4r).2))))); ((__scan5r).1, ((__scan5l).2 + (__scan5r).2))))); ((__scan6r).1, ((__scan6l).2 + (__scan6r).2)))))).2
@[expose] public def Triage.next (__s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Chart) := (match Reasoning.Clinic.Triage.select (__s) with | Option.none => Option.none | Option.some __step => Reasoning.Clinic.Triage.fire (__s) (__step))
@[expose] public def Triage.saturate (patient : Reasoning.Clinic.Vitals) : (Prod (Reasoning.Clinic.Chart) (Bool)) := (LexLeanCollections.iterateUntil ((Reasoning.Clinic.Triage.next)) (11) (Reasoning.Clinic.Triage.observe (patient)) : (Prod (Reasoning.Clinic.Chart) (Bool)))
public structure Triage.Run where
  state : Reasoning.Clinic.Chart
  trace : List (Reasoning.Clinic.Triage.Step)
  ledger : Reasoning.Clinic.Triage.Ledger
@[expose] public def Triage.start (patient : Reasoning.Clinic.Vitals) : Reasoning.Clinic.Triage.Run := ({ state := Reasoning.Clinic.Triage.observe (patient), trace := ([] : List (Reasoning.Clinic.Triage.Step)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := 0 } : Reasoning.Clinic.Triage.Ledger) } : Reasoning.Clinic.Triage.Run)
@[expose] public def Triage.step (__r : Reasoning.Clinic.Triage.Run) : Option (Reasoning.Clinic.Triage.Run) := (match Reasoning.Clinic.Triage.select ((__r).state) with | Option.none => Option.none | Option.some __step => (match Reasoning.Clinic.Triage.fire ((__r).state) (__step) with | Option.none => Option.none | Option.some __t => Option.some (({ state := __t, trace := (LexLeanRuntime.append ((__r).trace) ((__step :: ([] : List (Reasoning.Clinic.Triage.Step)))) : List (Reasoning.Clinic.Triage.Step)), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + Reasoning.Clinic.Triage.attempts ((__r).state)), firings := (((__r).ledger).firings + 1), expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : Reasoning.Clinic.Triage.Ledger) } : Reasoning.Clinic.Triage.Run))))
@[expose] public def Triage.run (patient : Reasoning.Clinic.Vitals) : (Prod (Reasoning.Clinic.Triage.Run) (Bool)) := (LexLeanCollections.iterateUntil ((Reasoning.Clinic.Triage.step)) (11) (Reasoning.Clinic.Triage.start (patient)) : (Prod (Reasoning.Clinic.Triage.Run) (Bool)))
@[expose] public def Triage.account (patient : Reasoning.Clinic.Vitals) : Reasoning.Clinic.Triage.Ledger := (let __final : (Prod (Reasoning.Clinic.Triage.Run) (Bool)) := Reasoning.Clinic.Triage.run (patient); (match (__final).2 with | Bool.true => ({ iterations := (((__final).1).ledger).iterations, attempts := ((((__final).1).ledger).attempts + Reasoning.Clinic.Triage.attempts (((__final).1).state)), firings := (((__final).1).ledger).firings, expansions := (((__final).1).ledger).expansions, verifications := ((((__final).1).ledger).verifications + 1), frontier := (((__final).1).ledger).frontier } : Reasoning.Clinic.Triage.Ledger) | Bool.false => ((__final).1).ledger))
@[expose] public def Triage.conclude (patient : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) : Except ((Prod Bool Bool)) (Nat) := (match Reasoning.Clinic.Triage.accept (patient) (__s) with | Option.some __v => Except.ok (__v) | Option.none => (match Reasoning.Clinic.Triage.extract (__s) with | Option.some _ => Except.error (((true, false) : Prod Bool Bool)) | Option.none => Except.error (((false, true) : Prod Bool Bool))))
@[expose] public def Triage.verdict (patient : Reasoning.Clinic.Vitals) : Except ((Prod Bool Bool)) (Nat) := (let __final : (Prod (Reasoning.Clinic.Chart) (Bool)) := Reasoning.Clinic.Triage.saturate (patient); (match (__final).2 with | Bool.true => Reasoning.Clinic.Triage.conclude (patient) ((__final).1) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
@[expose] public def Triage (patient : Reasoning.Clinic.Vitals) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (Reasoning.Clinic.Triage.Step)))) := (let __final : (Prod (Reasoning.Clinic.Triage.Run) (Bool)) := Reasoning.Clinic.Triage.run (patient); (match (__final).2 with | Bool.true => (match Reasoning.Clinic.Triage.conclude (patient) (((__final).1).state) with | Except.ok __v => Except.ok ((__v, ((__final).1).trace)) | Except.error __e => Except.error (__e)) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
public theorem Triage.fire_sound (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Clinic.Triage.Step) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Triage.fire (__s) (__step) = Option.some (__t)) -> Reasoning.Clinic.Justified (__s) (__t)) :=
by
  cases __step with
  | Fever =>
    exact Reasoning.Clinic.Fever.apply_sound __s __t
  | Tachycardia =>
    exact Reasoning.Clinic.Tachycardia.apply_sound __s __t
  | Tachypnea =>
    exact Reasoning.Clinic.Tachypnea.apply_sound __s __t
  | Leukocytosis =>
    exact Reasoning.Clinic.Leukocytosis.apply_sound __s __t
  | Sirs =>
    exact Reasoning.Clinic.Sirs.apply_sound __s __t
  | Sepsis =>
    exact Reasoning.Clinic.Sepsis.apply_sound __s __t
  | Shock =>
    exact Reasoning.Clinic.Shock.apply_sound __s __t
  | Escalate =>
    exact Reasoning.Clinic.Escalate.apply_sound __s __t
public theorem Triage.fire_progress (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Clinic.Triage.Step) (__t : Reasoning.Clinic.Chart) : (Reasoning.Clinic.Consistent (__s) -> ((Reasoning.Clinic.Triage.fire (__s) (__step) = Option.some (__t)) -> (Reasoning.Clinic.Pending (__t) < Reasoning.Clinic.Pending (__s)))) :=
by
  cases __step with
  | Fever =>
    exact Reasoning.Clinic.Fever.apply_progress __s __t
  | Tachycardia =>
    exact Reasoning.Clinic.Tachycardia.apply_progress __s __t
  | Tachypnea =>
    exact Reasoning.Clinic.Tachypnea.apply_progress __s __t
  | Leukocytosis =>
    exact Reasoning.Clinic.Leukocytosis.apply_progress __s __t
  | Sirs =>
    exact Reasoning.Clinic.Sirs.apply_progress __s __t
  | Sepsis =>
    exact Reasoning.Clinic.Sepsis.apply_progress __s __t
  | Shock =>
    exact Reasoning.Clinic.Shock.apply_progress __s __t
  | Escalate =>
    exact Reasoning.Clinic.Escalate.apply_progress __s __t
public theorem Triage.replay_fire (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Clinic.Triage.Step) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Triage.fire (__s) (__step) = Option.some (__t)) -> (Reasoning.Clinic.Triage.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Triage.replay]
  rw [llE]
public theorem Triage.replay_sound (patient : Reasoning.Clinic.Vitals) (__acc : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart)) (__step : Reasoning.Clinic.Triage.Step) : ((LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Triage.observe (patient)) (__acc)) -> (LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Triage.observe (patient)) (Reasoning.Clinic.Triage.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [Reasoning.Clinic.Triage.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (Reasoning.Clinic.Triage.fire_sound llS __step _ ‹_›)
public theorem Triage.derivation (patient : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Clinic.Triage.Step)) : (LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Triage.observe (patient)) (Reasoning.Clinic.Triage.follow (patient) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((Reasoning.Clinic.Triage.replay)) (fun (__acc : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart)) => (LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Triage.observe (patient)) (__acc))) (Reasoning.Clinic.Triage.replay_sound (patient)) (__trace) (Except.ok (Reasoning.Clinic.Triage.observe (patient))) (LexLeanReasoning.reachesStart ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Triage.observe (patient))))
public theorem Triage.follow_invariant (patient : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Clinic.Triage.Step)) (__s : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Triage.follow (patient) (__trace) = Except.ok (__s)) -> Reasoning.Clinic.Consistent (__s)) :=
  (fun llE => (LexLeanReasoning.starPreserves ((Reasoning.Clinic.Justified)) ((Reasoning.Clinic.Consistent)) Reasoning.Clinic.Findings.preserves (Reasoning.Clinic.Triage.observe (patient)) (__s) (Reasoning.Clinic.Triage.derivation (patient) (__trace) (__s) llE) (Reasoning.Clinic.admitted_consistent (patient))))
public theorem Triage.accept_sound (patient : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) (__v : Nat) : ((Reasoning.Clinic.Triage.accept (patient) (__s) = Option.some (__v)) -> Reasoning.Clinic.Indicated (patient) (__v)) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Triage.accept] at llE
  split at llE
  · cases llE
  · have llC := LexLeanReasoning.checked _ _ _ llE
    exact llC.right ▸ Reasoning.Clinic.recommendation_sound _ _ llC.left
public theorem Triage.next_sound (__s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Triage.next (__s) = Option.some (__t)) -> Reasoning.Clinic.Justified (__s) (__t)) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Triage.next] at llE
  split at llE
  · cases llE
  · exact Reasoning.Clinic.Triage.fire_sound __s _ __t llE
public theorem Triage.next_preserves (__s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Triage.next (__s) = Option.some (__t)) -> (Reasoning.Clinic.Consistent (__s) -> Reasoning.Clinic.Consistent (__t))) :=
  (fun llE llH => (Reasoning.Clinic.consistent_preserved (__s) (__t) llH (Reasoning.Clinic.Triage.next_sound (__s) (__t) llE)))
public theorem Triage.next_progress (__s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Triage.next (__s) = Option.some (__t)) -> (Reasoning.Clinic.Consistent (__s) -> (Reasoning.Clinic.Pending (__t) < Reasoning.Clinic.Pending (__s)))) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Triage.next] at llE
  split at llE
  · cases llE
  · exact fun llJ => Reasoning.Clinic.Triage.fire_progress __s _ __t llJ llE
public theorem Triage.step_none (__r : Reasoning.Clinic.Triage.Run) : ((Reasoning.Clinic.Triage.step (__r) = Option.none) -> (Reasoning.Clinic.Triage.next ((__r).state) = Option.none)) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Triage.step] at llE
  split at llE
  · rename_i llH
    dsimp only [Reasoning.Clinic.Triage.next]
    rw [llH]
  · rename_i llH
    split at llE
    · dsimp only [Reasoning.Clinic.Triage.next]
      rw [llH]
      assumption
    · cases llE
public theorem Triage.step_some (__r : Reasoning.Clinic.Triage.Run) (__q : Reasoning.Clinic.Triage.Run) : ((Reasoning.Clinic.Triage.step (__r) = Option.some (__q)) -> (Reasoning.Clinic.Triage.next ((__r).state) = Option.some ((__q).state))) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Triage.step] at llE
  split at llE
  · cases llE
  · rename_i llH
    split at llE
    · cases llE
    · cases llE
      dsimp only [Reasoning.Clinic.Triage.next]
      rw [llH]
      assumption
public theorem Triage.step_trace (patient : Reasoning.Clinic.Vitals) (__r : Reasoning.Clinic.Triage.Run) (__q : Reasoning.Clinic.Triage.Run) : ((Reasoning.Clinic.Triage.step (__r) = Option.some (__q)) -> ((Reasoning.Clinic.Triage.follow (patient) ((__r).trace) = Except.ok ((__r).state)) -> (Reasoning.Clinic.Triage.follow (patient) ((__q).trace) = Except.ok ((__q).state)))) :=
by
  intro llE llH
  dsimp only [Reasoning.Clinic.Triage.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      dsimp only [Reasoning.Clinic.Triage.follow]
      rw [LexLeanReasoning.foldSnoc]
      dsimp only [Reasoning.Clinic.Triage.follow] at llH
      rw [llH]
      apply Reasoning.Clinic.Triage.replay_fire
      assumption
public theorem Triage.step_count (__r : Reasoning.Clinic.Triage.Run) (__q : Reasoning.Clinic.Triage.Run) : ((Reasoning.Clinic.Triage.step (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Triage.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Triage.step_fired (__r : Reasoning.Clinic.Triage.Run) (__q : Reasoning.Clinic.Triage.Run) : ((Reasoning.Clinic.Triage.step (__r) = Option.some (__q)) -> (((__q).ledger).firings = (((__r).ledger).firings + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Triage.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Triage.saturate_derivation (patient : Reasoning.Clinic.Vitals) : (LexLeanReasoning.Star ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Triage.observe (patient)) ((Reasoning.Clinic.Triage.saturate (patient)).1)) :=
by
  dsimp only [Reasoning.Clinic.Triage.saturate]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Clinic.Triage.next)) (fun (__s : Reasoning.Clinic.Chart) => (LexLeanReasoning.Star ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Triage.observe (patient)) (__s))) (fun llA llB llE llH => (LexLeanReasoning.Star.tail _ llA llB llH (Reasoning.Clinic.Triage.next_sound llA llB llE))) (11) (Reasoning.Clinic.Triage.observe (patient)) (LexLeanReasoning.Star.refl _))
public theorem Triage.run_state (patient : Reasoning.Clinic.Vitals) : ((((Reasoning.Clinic.Triage.run (patient)).1).state = (Reasoning.Clinic.Triage.saturate (patient)).1) /\ ((Reasoning.Clinic.Triage.run (patient)).2 = (Reasoning.Clinic.Triage.saturate (patient)).2)) :=
by
  dsimp only [Reasoning.Clinic.Triage.run, Reasoning.Clinic.Triage.saturate]
  exact (LexLeanReasoning.iterateUntilSimulate ((Reasoning.Clinic.Triage.step)) ((Reasoning.Clinic.Triage.next)) ((fun (__r : Reasoning.Clinic.Triage.Run) => (__r).state)) Reasoning.Clinic.Triage.step_none Reasoning.Clinic.Triage.step_some (11) (Reasoning.Clinic.Triage.start (patient)))
public theorem Triage.run_trace (patient : Reasoning.Clinic.Vitals) : (Reasoning.Clinic.Triage.follow (patient) (((Reasoning.Clinic.Triage.run (patient)).1).trace) = Except.ok (((Reasoning.Clinic.Triage.run (patient)).1).state)) :=
by
  dsimp only [Reasoning.Clinic.Triage.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Clinic.Triage.step)) (fun (__r : Reasoning.Clinic.Triage.Run) => (Reasoning.Clinic.Triage.follow (patient) ((__r).trace) = Except.ok ((__r).state))) (Reasoning.Clinic.Triage.step_trace (patient)) (11) (Reasoning.Clinic.Triage.start (patient)) rfl)
public theorem Triage.iterations_bounded (patient : Reasoning.Clinic.Vitals) : ((((Reasoning.Clinic.Triage.run (patient)).1).ledger).iterations <= 11) :=
by
  dsimp only [Reasoning.Clinic.Triage.run]
  exact (LexLeanReasoning.iterateUntilCount ((Reasoning.Clinic.Triage.step)) ((fun (__r : Reasoning.Clinic.Triage.Run) => ((__r).ledger).iterations)) Reasoning.Clinic.Triage.step_count (11) (Reasoning.Clinic.Triage.start (patient)) rfl)
public theorem Triage.firings_bounded (patient : Reasoning.Clinic.Vitals) : ((((Reasoning.Clinic.Triage.run (patient)).1).ledger).firings <= 11) :=
by
  dsimp only [Reasoning.Clinic.Triage.run]
  exact (LexLeanReasoning.iterateUntilCount ((Reasoning.Clinic.Triage.step)) ((fun (__r : Reasoning.Clinic.Triage.Run) => ((__r).ledger).firings)) Reasoning.Clinic.Triage.step_fired (11) (Reasoning.Clinic.Triage.start (patient)) rfl)
public theorem Triage.saturate_invariant (patient : Reasoning.Clinic.Vitals) : Reasoning.Clinic.Consistent ((Reasoning.Clinic.Triage.saturate (patient)).1) :=
by
  dsimp only [Reasoning.Clinic.Triage.saturate]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Clinic.Triage.next)) ((Reasoning.Clinic.Consistent)) Reasoning.Clinic.Triage.next_preserves (11) (Reasoning.Clinic.Triage.observe (patient)) (Reasoning.Clinic.admitted_consistent (patient)))
public theorem Triage.saturates (patient : Reasoning.Clinic.Vitals) : ((Reasoning.Clinic.Triage.saturate (patient)).2 = true) :=
by
  dsimp only [Reasoning.Clinic.Triage.saturate]
  exact (LexLeanReasoning.iterateUntilStops ((Reasoning.Clinic.Triage.next)) ((Reasoning.Clinic.Consistent)) ((Reasoning.Clinic.Pending)) (fun llA llB llE llH => (And.intro (Reasoning.Clinic.Triage.next_preserves llA llB llE llH) (Reasoning.Clinic.Triage.next_progress llA llB llE llH))) (11) (Reasoning.Clinic.Triage.observe (patient)) (Reasoning.Clinic.admitted_consistent (patient)) (Reasoning.Clinic.admitted_pending (patient)))
public theorem Triage.conclude_accept (patient : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) (__v : Nat) : ((Reasoning.Clinic.Triage.conclude (patient) (__s) = Except.ok (__v)) -> (Reasoning.Clinic.Triage.accept (patient) (__s) = Option.some (__v))) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Triage.conclude] at llE
  split at llE
  · cases llE
    assumption
  · split at llE
    · cases llE
    · cases llE
public theorem Triage.conclude_sound (patient : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) (__v : Nat) : ((Reasoning.Clinic.Triage.conclude (patient) (__s) = Except.ok (__v)) -> Reasoning.Clinic.Indicated (patient) (__v)) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Triage.conclude] at llE
  split at llE
  · cases llE
    exact Reasoning.Clinic.Triage.accept_sound _ _ _ ‹_›
  · split at llE
    · cases llE
    · cases llE
public theorem Triage.verdict_sound (patient : Reasoning.Clinic.Vitals) : (forall (__v : Nat), ((Reasoning.Clinic.Triage.verdict (patient) = Except.ok (__v)) -> Reasoning.Clinic.Indicated (patient) (__v))) :=
by
  intro llV llE
  dsimp only [Reasoning.Clinic.Triage.verdict] at llE
  generalize llRun : Reasoning.Clinic.Triage.saturate patient = llR at llE
  split at llE
  · exact Reasoning.Clinic.Triage.conclude_sound _ _ _ llE
  · cases llE
public theorem Triage.explained (patient : Reasoning.Clinic.Vitals) : (forall (__v : Nat), (forall (__trace : List (Reasoning.Clinic.Triage.Step)), ((Reasoning.Clinic.Triage (patient) = Except.ok ((__v, __trace))) -> ((Reasoning.Clinic.Triage.answer (patient) (__trace) = Option.some (__v)) /\ Reasoning.Clinic.Indicated (patient) (__v))))) :=
by
  intro llV llT llE
  have llTrace := Reasoning.Clinic.Triage.run_trace patient
  dsimp only [Reasoning.Clinic.Triage] at llE
  generalize llRun : Reasoning.Clinic.Triage.run patient = llR at llE llTrace
  split at llE
  · split at llE
    · rename_i llW llH
      cases llE
      exact And.intro (by dsimp only [Reasoning.Clinic.Triage.answer]; rw [llTrace]; exact Reasoning.Clinic.Triage.conclude_accept _ _ _ llH) (Reasoning.Clinic.Triage.conclude_sound _ _ _ llH)
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
@[expose] public def Review.observe (patient : Reasoning.Clinic.Vitals) : Reasoning.Clinic.Chart := ({ vitals := patient, fever := 0, tachycardia := 0, tachypnea := 0, leukocytosis := 0, sirs := 0, sepsis := 0, shock := 0, level := 0 } : Reasoning.Clinic.Chart)
@[expose] public def Review.fire (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Clinic.Review.Step) : Option (Reasoning.Clinic.Chart) := (match __step with | Reasoning.Clinic.Review.Step.Fever => Reasoning.Clinic.Fever.apply (__s) | Reasoning.Clinic.Review.Step.Tachycardia => Reasoning.Clinic.Tachycardia.apply (__s) | Reasoning.Clinic.Review.Step.Tachypnea => Reasoning.Clinic.Tachypnea.apply (__s) | Reasoning.Clinic.Review.Step.Leukocytosis => Reasoning.Clinic.Leukocytosis.apply (__s) | Reasoning.Clinic.Review.Step.Sirs => Reasoning.Clinic.Sirs.apply (__s) | Reasoning.Clinic.Review.Step.Sepsis => Reasoning.Clinic.Sepsis.apply (__s) | Reasoning.Clinic.Review.Step.Shock => Reasoning.Clinic.Shock.apply (__s) | Reasoning.Clinic.Review.Step.Escalate => Reasoning.Clinic.Escalate.apply (__s))
@[expose] public def Review.replay (__acc : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart)) (__step : Reasoning.Clinic.Review.Step) : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match Reasoning.Clinic.Review.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def Review.follow (patient : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Clinic.Review.Step)) : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart) := (LexLeanCollections.listFold ((Reasoning.Clinic.Review.replay)) (Except.ok (Reasoning.Clinic.Review.observe (patient))) (__trace) : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart))
@[expose] public def Review.extract (chart : Reasoning.Clinic.Chart) : Option (Nat) := Option.some ((chart).level)
@[expose] public def Review.accept (patient : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) : Option (Nat) := (match Reasoning.Clinic.Review.extract (__s) with | Option.none => Option.none | Option.some __v => (if Reasoning.Clinic.dischargeCheck (patient) (__v) then Option.some (__v) else Option.none))
@[expose] public def Review.answer (patient : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Clinic.Review.Step)) : Option (Nat) := (match Reasoning.Clinic.Review.follow (patient) (__trace) with | Except.ok __s => Reasoning.Clinic.Review.accept (patient) (__s) | Except.error _ => Option.none)
@[expose] public def Review.select (__s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Review.Step) := (match (match (match (if Reasoning.Clinic.Fever.guard (__s) then Option.some (Reasoning.Clinic.Review.Step.Fever) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Tachycardia.guard (__s) then Option.some (Reasoning.Clinic.Review.Step.Tachycardia) else Option.none)) with | Option.some __found => Option.some (__found) | Option.none => (match (if Reasoning.Clinic.Tachypnea.guard (__s) then Option.some (Reasoning.Clinic.Review.Step.Tachypnea) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Leukocytosis.guard (__s) then Option.some (Reasoning.Clinic.Review.Step.Leukocytosis) else Option.none))) with | Option.some __found => Option.some (__found) | Option.none => (match (match (if Reasoning.Clinic.Sirs.guard (__s) then Option.some (Reasoning.Clinic.Review.Step.Sirs) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Sepsis.guard (__s) then Option.some (Reasoning.Clinic.Review.Step.Sepsis) else Option.none)) with | Option.some __found => Option.some (__found) | Option.none => (match (if Reasoning.Clinic.Shock.guard (__s) then Option.some (Reasoning.Clinic.Review.Step.Shock) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Escalate.guard (__s) then Option.some (Reasoning.Clinic.Review.Step.Escalate) else Option.none))))
@[expose] public def Review.attempts (__s : Reasoning.Clinic.Chart) : Nat := ((let __scan6l : (Prod (Bool) (Nat)) := (let __scan2l : (Prod (Bool) (Nat)) := (let __scan0l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Fever.guard (__s), 1); (if (__scan0l).1 then __scan0l else (let __scan0r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Tachycardia.guard (__s), 1); ((__scan0r).1, ((__scan0l).2 + (__scan0r).2))))); (if (__scan2l).1 then __scan2l else (let __scan2r : (Prod (Bool) (Nat)) := (let __scan1l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Tachypnea.guard (__s), 1); (if (__scan1l).1 then __scan1l else (let __scan1r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Leukocytosis.guard (__s), 1); ((__scan1r).1, ((__scan1l).2 + (__scan1r).2))))); ((__scan2r).1, ((__scan2l).2 + (__scan2r).2))))); (if (__scan6l).1 then __scan6l else (let __scan6r : (Prod (Bool) (Nat)) := (let __scan5l : (Prod (Bool) (Nat)) := (let __scan3l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Sirs.guard (__s), 1); (if (__scan3l).1 then __scan3l else (let __scan3r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Sepsis.guard (__s), 1); ((__scan3r).1, ((__scan3l).2 + (__scan3r).2))))); (if (__scan5l).1 then __scan5l else (let __scan5r : (Prod (Bool) (Nat)) := (let __scan4l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Shock.guard (__s), 1); (if (__scan4l).1 then __scan4l else (let __scan4r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Escalate.guard (__s), 1); ((__scan4r).1, ((__scan4l).2 + (__scan4r).2))))); ((__scan5r).1, ((__scan5l).2 + (__scan5r).2))))); ((__scan6r).1, ((__scan6l).2 + (__scan6r).2)))))).2
@[expose] public def Review.next (__s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Chart) := (match Reasoning.Clinic.Review.select (__s) with | Option.none => Option.none | Option.some __step => Reasoning.Clinic.Review.fire (__s) (__step))
@[expose] public def Review.saturate (patient : Reasoning.Clinic.Vitals) : (Prod (Reasoning.Clinic.Chart) (Bool)) := (LexLeanCollections.iterateUntil ((Reasoning.Clinic.Review.next)) (6) (Reasoning.Clinic.Review.observe (patient)) : (Prod (Reasoning.Clinic.Chart) (Bool)))
public structure Review.Run where
  state : Reasoning.Clinic.Chart
  trace : List (Reasoning.Clinic.Review.Step)
  ledger : Reasoning.Clinic.Review.Ledger
@[expose] public def Review.start (patient : Reasoning.Clinic.Vitals) : Reasoning.Clinic.Review.Run := ({ state := Reasoning.Clinic.Review.observe (patient), trace := ([] : List (Reasoning.Clinic.Review.Step)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := 0 } : Reasoning.Clinic.Review.Ledger) } : Reasoning.Clinic.Review.Run)
@[expose] public def Review.step (__r : Reasoning.Clinic.Review.Run) : Option (Reasoning.Clinic.Review.Run) := (match Reasoning.Clinic.Review.select ((__r).state) with | Option.none => Option.none | Option.some __step => (match Reasoning.Clinic.Review.fire ((__r).state) (__step) with | Option.none => Option.none | Option.some __t => Option.some (({ state := __t, trace := (LexLeanRuntime.append ((__r).trace) ((__step :: ([] : List (Reasoning.Clinic.Review.Step)))) : List (Reasoning.Clinic.Review.Step)), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + Reasoning.Clinic.Review.attempts ((__r).state)), firings := (((__r).ledger).firings + 1), expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : Reasoning.Clinic.Review.Ledger) } : Reasoning.Clinic.Review.Run))))
@[expose] public def Review.run (patient : Reasoning.Clinic.Vitals) : (Prod (Reasoning.Clinic.Review.Run) (Bool)) := (LexLeanCollections.iterateUntil ((Reasoning.Clinic.Review.step)) (6) (Reasoning.Clinic.Review.start (patient)) : (Prod (Reasoning.Clinic.Review.Run) (Bool)))
@[expose] public def Review.account (patient : Reasoning.Clinic.Vitals) : Reasoning.Clinic.Review.Ledger := (let __final : (Prod (Reasoning.Clinic.Review.Run) (Bool)) := Reasoning.Clinic.Review.run (patient); (match (__final).2 with | Bool.true => ({ iterations := (((__final).1).ledger).iterations, attempts := ((((__final).1).ledger).attempts + Reasoning.Clinic.Review.attempts (((__final).1).state)), firings := (((__final).1).ledger).firings, expansions := (((__final).1).ledger).expansions, verifications := ((((__final).1).ledger).verifications + 1), frontier := (((__final).1).ledger).frontier } : Reasoning.Clinic.Review.Ledger) | Bool.false => ((__final).1).ledger))
@[expose] public def Review.conclude (patient : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) : Except ((Prod Bool Bool)) (Nat) := (match Reasoning.Clinic.Review.accept (patient) (__s) with | Option.some __v => Except.ok (__v) | Option.none => (match Reasoning.Clinic.Review.extract (__s) with | Option.some _ => Except.error (((true, false) : Prod Bool Bool)) | Option.none => Except.error (((false, true) : Prod Bool Bool))))
@[expose] public def Review.verdict (patient : Reasoning.Clinic.Vitals) : Except ((Prod Bool Bool)) (Nat) := (let __final : (Prod (Reasoning.Clinic.Chart) (Bool)) := Reasoning.Clinic.Review.saturate (patient); (match (__final).2 with | Bool.true => Reasoning.Clinic.Review.conclude (patient) ((__final).1) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
@[expose] public def Review (patient : Reasoning.Clinic.Vitals) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (Reasoning.Clinic.Review.Step)))) := (let __final : (Prod (Reasoning.Clinic.Review.Run) (Bool)) := Reasoning.Clinic.Review.run (patient); (match (__final).2 with | Bool.true => (match Reasoning.Clinic.Review.conclude (patient) (((__final).1).state) with | Except.ok __v => Except.ok ((__v, ((__final).1).trace)) | Except.error __e => Except.error (__e)) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
public theorem Review.fire_sound (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Clinic.Review.Step) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Review.fire (__s) (__step) = Option.some (__t)) -> Reasoning.Clinic.Justified (__s) (__t)) :=
by
  cases __step with
  | Fever =>
    exact Reasoning.Clinic.Fever.apply_sound __s __t
  | Tachycardia =>
    exact Reasoning.Clinic.Tachycardia.apply_sound __s __t
  | Tachypnea =>
    exact Reasoning.Clinic.Tachypnea.apply_sound __s __t
  | Leukocytosis =>
    exact Reasoning.Clinic.Leukocytosis.apply_sound __s __t
  | Sirs =>
    exact Reasoning.Clinic.Sirs.apply_sound __s __t
  | Sepsis =>
    exact Reasoning.Clinic.Sepsis.apply_sound __s __t
  | Shock =>
    exact Reasoning.Clinic.Shock.apply_sound __s __t
  | Escalate =>
    exact Reasoning.Clinic.Escalate.apply_sound __s __t
public theorem Review.replay_fire (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Clinic.Review.Step) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Review.fire (__s) (__step) = Option.some (__t)) -> (Reasoning.Clinic.Review.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Review.replay]
  rw [llE]
public theorem Review.replay_sound (patient : Reasoning.Clinic.Vitals) (__acc : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart)) (__step : Reasoning.Clinic.Review.Step) : ((LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Review.observe (patient)) (__acc)) -> (LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Review.observe (patient)) (Reasoning.Clinic.Review.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [Reasoning.Clinic.Review.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (Reasoning.Clinic.Review.fire_sound llS __step _ ‹_›)
public theorem Review.derivation (patient : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Clinic.Review.Step)) : (LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Review.observe (patient)) (Reasoning.Clinic.Review.follow (patient) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((Reasoning.Clinic.Review.replay)) (fun (__acc : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart)) => (LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Review.observe (patient)) (__acc))) (Reasoning.Clinic.Review.replay_sound (patient)) (__trace) (Except.ok (Reasoning.Clinic.Review.observe (patient))) (LexLeanReasoning.reachesStart ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Review.observe (patient))))
public theorem Review.follow_invariant (patient : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Clinic.Review.Step)) (__s : Reasoning.Clinic.Chart) : (Reasoning.Clinic.Consistent (Reasoning.Clinic.Review.observe (patient)) -> ((Reasoning.Clinic.Review.follow (patient) (__trace) = Except.ok (__s)) -> Reasoning.Clinic.Consistent (__s))) :=
  (fun llI llE => (LexLeanReasoning.starPreserves ((Reasoning.Clinic.Justified)) ((Reasoning.Clinic.Consistent)) Reasoning.Clinic.Findings.preserves (Reasoning.Clinic.Review.observe (patient)) (__s) (Reasoning.Clinic.Review.derivation (patient) (__trace) (__s) llE) llI))
public theorem Review.accept_sound (patient : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) (__v : Nat) : ((Reasoning.Clinic.Review.accept (patient) (__s) = Option.some (__v)) -> Reasoning.Clinic.Discharged (patient) (__v)) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Review.accept] at llE
  split at llE
  · cases llE
  · have llC := LexLeanReasoning.checked _ _ _ llE
    exact llC.right ▸ Reasoning.Clinic.discharge_sound _ _ llC.left
public theorem Review.next_sound (__s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Review.next (__s) = Option.some (__t)) -> Reasoning.Clinic.Justified (__s) (__t)) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Review.next] at llE
  split at llE
  · cases llE
  · exact Reasoning.Clinic.Review.fire_sound __s _ __t llE
public theorem Review.next_preserves (__s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Review.next (__s) = Option.some (__t)) -> (Reasoning.Clinic.Consistent (__s) -> Reasoning.Clinic.Consistent (__t))) :=
  (fun llE llH => (Reasoning.Clinic.consistent_preserved (__s) (__t) llH (Reasoning.Clinic.Review.next_sound (__s) (__t) llE)))
public theorem Review.step_none (__r : Reasoning.Clinic.Review.Run) : ((Reasoning.Clinic.Review.step (__r) = Option.none) -> (Reasoning.Clinic.Review.next ((__r).state) = Option.none)) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Review.step] at llE
  split at llE
  · rename_i llH
    dsimp only [Reasoning.Clinic.Review.next]
    rw [llH]
  · rename_i llH
    split at llE
    · dsimp only [Reasoning.Clinic.Review.next]
      rw [llH]
      assumption
    · cases llE
public theorem Review.step_some (__r : Reasoning.Clinic.Review.Run) (__q : Reasoning.Clinic.Review.Run) : ((Reasoning.Clinic.Review.step (__r) = Option.some (__q)) -> (Reasoning.Clinic.Review.next ((__r).state) = Option.some ((__q).state))) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Review.step] at llE
  split at llE
  · cases llE
  · rename_i llH
    split at llE
    · cases llE
    · cases llE
      dsimp only [Reasoning.Clinic.Review.next]
      rw [llH]
      assumption
public theorem Review.step_trace (patient : Reasoning.Clinic.Vitals) (__r : Reasoning.Clinic.Review.Run) (__q : Reasoning.Clinic.Review.Run) : ((Reasoning.Clinic.Review.step (__r) = Option.some (__q)) -> ((Reasoning.Clinic.Review.follow (patient) ((__r).trace) = Except.ok ((__r).state)) -> (Reasoning.Clinic.Review.follow (patient) ((__q).trace) = Except.ok ((__q).state)))) :=
by
  intro llE llH
  dsimp only [Reasoning.Clinic.Review.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      dsimp only [Reasoning.Clinic.Review.follow]
      rw [LexLeanReasoning.foldSnoc]
      dsimp only [Reasoning.Clinic.Review.follow] at llH
      rw [llH]
      apply Reasoning.Clinic.Review.replay_fire
      assumption
public theorem Review.step_count (__r : Reasoning.Clinic.Review.Run) (__q : Reasoning.Clinic.Review.Run) : ((Reasoning.Clinic.Review.step (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Review.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Review.step_fired (__r : Reasoning.Clinic.Review.Run) (__q : Reasoning.Clinic.Review.Run) : ((Reasoning.Clinic.Review.step (__r) = Option.some (__q)) -> (((__q).ledger).firings = (((__r).ledger).firings + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Review.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Review.saturate_derivation (patient : Reasoning.Clinic.Vitals) : (LexLeanReasoning.Star ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Review.observe (patient)) ((Reasoning.Clinic.Review.saturate (patient)).1)) :=
by
  dsimp only [Reasoning.Clinic.Review.saturate]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Clinic.Review.next)) (fun (__s : Reasoning.Clinic.Chart) => (LexLeanReasoning.Star ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Review.observe (patient)) (__s))) (fun llA llB llE llH => (LexLeanReasoning.Star.tail _ llA llB llH (Reasoning.Clinic.Review.next_sound llA llB llE))) (6) (Reasoning.Clinic.Review.observe (patient)) (LexLeanReasoning.Star.refl _))
public theorem Review.run_state (patient : Reasoning.Clinic.Vitals) : ((((Reasoning.Clinic.Review.run (patient)).1).state = (Reasoning.Clinic.Review.saturate (patient)).1) /\ ((Reasoning.Clinic.Review.run (patient)).2 = (Reasoning.Clinic.Review.saturate (patient)).2)) :=
by
  dsimp only [Reasoning.Clinic.Review.run, Reasoning.Clinic.Review.saturate]
  exact (LexLeanReasoning.iterateUntilSimulate ((Reasoning.Clinic.Review.step)) ((Reasoning.Clinic.Review.next)) ((fun (__r : Reasoning.Clinic.Review.Run) => (__r).state)) Reasoning.Clinic.Review.step_none Reasoning.Clinic.Review.step_some (6) (Reasoning.Clinic.Review.start (patient)))
public theorem Review.run_trace (patient : Reasoning.Clinic.Vitals) : (Reasoning.Clinic.Review.follow (patient) (((Reasoning.Clinic.Review.run (patient)).1).trace) = Except.ok (((Reasoning.Clinic.Review.run (patient)).1).state)) :=
by
  dsimp only [Reasoning.Clinic.Review.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Clinic.Review.step)) (fun (__r : Reasoning.Clinic.Review.Run) => (Reasoning.Clinic.Review.follow (patient) ((__r).trace) = Except.ok ((__r).state))) (Reasoning.Clinic.Review.step_trace (patient)) (6) (Reasoning.Clinic.Review.start (patient)) rfl)
public theorem Review.iterations_bounded (patient : Reasoning.Clinic.Vitals) : ((((Reasoning.Clinic.Review.run (patient)).1).ledger).iterations <= 6) :=
by
  dsimp only [Reasoning.Clinic.Review.run]
  exact (LexLeanReasoning.iterateUntilCount ((Reasoning.Clinic.Review.step)) ((fun (__r : Reasoning.Clinic.Review.Run) => ((__r).ledger).iterations)) Reasoning.Clinic.Review.step_count (6) (Reasoning.Clinic.Review.start (patient)) rfl)
public theorem Review.firings_bounded (patient : Reasoning.Clinic.Vitals) : ((((Reasoning.Clinic.Review.run (patient)).1).ledger).firings <= 6) :=
by
  dsimp only [Reasoning.Clinic.Review.run]
  exact (LexLeanReasoning.iterateUntilCount ((Reasoning.Clinic.Review.step)) ((fun (__r : Reasoning.Clinic.Review.Run) => ((__r).ledger).firings)) Reasoning.Clinic.Review.step_fired (6) (Reasoning.Clinic.Review.start (patient)) rfl)
public theorem Review.saturate_invariant (patient : Reasoning.Clinic.Vitals) : (Reasoning.Clinic.Consistent (Reasoning.Clinic.Review.observe (patient)) -> Reasoning.Clinic.Consistent ((Reasoning.Clinic.Review.saturate (patient)).1)) :=
by
  dsimp only [Reasoning.Clinic.Review.saturate]
  exact (fun llI => (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Clinic.Review.next)) ((Reasoning.Clinic.Consistent)) Reasoning.Clinic.Review.next_preserves (6) (Reasoning.Clinic.Review.observe (patient)) llI))
public theorem Review.conclude_accept (patient : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) (__v : Nat) : ((Reasoning.Clinic.Review.conclude (patient) (__s) = Except.ok (__v)) -> (Reasoning.Clinic.Review.accept (patient) (__s) = Option.some (__v))) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Review.conclude] at llE
  split at llE
  · cases llE
    assumption
  · split at llE
    · cases llE
    · cases llE
public theorem Review.conclude_sound (patient : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) (__v : Nat) : ((Reasoning.Clinic.Review.conclude (patient) (__s) = Except.ok (__v)) -> Reasoning.Clinic.Discharged (patient) (__v)) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Review.conclude] at llE
  split at llE
  · cases llE
    exact Reasoning.Clinic.Review.accept_sound _ _ _ ‹_›
  · split at llE
    · cases llE
    · cases llE
public theorem Review.verdict_sound (patient : Reasoning.Clinic.Vitals) : (forall (__v : Nat), ((Reasoning.Clinic.Review.verdict (patient) = Except.ok (__v)) -> Reasoning.Clinic.Discharged (patient) (__v))) :=
by
  intro llV llE
  dsimp only [Reasoning.Clinic.Review.verdict] at llE
  generalize llRun : Reasoning.Clinic.Review.saturate patient = llR at llE
  split at llE
  · exact Reasoning.Clinic.Review.conclude_sound _ _ _ llE
  · cases llE
public theorem Review.explained (patient : Reasoning.Clinic.Vitals) : (forall (__v : Nat), (forall (__trace : List (Reasoning.Clinic.Review.Step)), ((Reasoning.Clinic.Review (patient) = Except.ok ((__v, __trace))) -> ((Reasoning.Clinic.Review.answer (patient) (__trace) = Option.some (__v)) /\ Reasoning.Clinic.Discharged (patient) (__v))))) :=
by
  intro llV llT llE
  have llTrace := Reasoning.Clinic.Review.run_trace patient
  dsimp only [Reasoning.Clinic.Review] at llE
  generalize llRun : Reasoning.Clinic.Review.run patient = llR at llE llTrace
  split at llE
  · split at llE
    · rename_i llW llH
      cases llE
      exact And.intro (by dsimp only [Reasoning.Clinic.Review.answer]; rw [llTrace]; exact Reasoning.Clinic.Review.conclude_accept _ _ _ llH) (Reasoning.Clinic.Review.conclude_sound _ _ _ llH)
    · cases llE
  · cases llE

end Reasoning.Clinic
