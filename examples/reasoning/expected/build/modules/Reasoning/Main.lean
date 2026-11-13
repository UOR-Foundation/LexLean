module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanRuntime.Models
public import LexLeanRuntime.Reasoning
public import LexLeanRuntime.Laws
public import Reasoning.Budget
public import Reasoning.Clinic
public import Reasoning.Planner
public import Reasoning.Screening
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Reasoning.Main

@[expose, reducible] public def Guaranteed (v : Reasoning.Clinic.Vitals) (o : Except ((Prod Bool Bool)) (Nat)) : Prop := (forall (r : Nat), ((o = Except.ok (r)) -> Reasoning.Clinic.Indicated (v) (r)))


@[expose] public def GatewayTriage (v : Reasoning.Clinic.Vitals) : Except ((Prod Bool Bool)) (Nat) := Reasoning.Clinic.Triage.verdict (v)

public theorem gateway_safe (v : Reasoning.Clinic.Vitals) : Guaranteed (v) (GatewayTriage (v)) := by
  exact Reasoning.Clinic.Triage.verdict_sound (v)

public theorem GatewayEvidence.gateway_safe : LexLeanModels.SatisfiesTotal ((Reasoning.Main.Guaranteed)) ((Reasoning.Main.GatewayTriage)) := Reasoning.Main.gateway_safe

@[expose] public def GatewayModel (__input : Reasoning.Clinic.Vitals) : Except ((Prod Bool Bool)) (Nat) := Reasoning.Main.GatewayTriage (__input)

@[expose, reducible] public def Graded (v : Reasoning.Clinic.Vitals) (r : Nat) : Prop := ((r <= 3) /\ ((r = 3) -> Reasoning.Clinic.Hypotensive (v)))

@[expose] public def gradedCheck (v : Reasoning.Clinic.Vitals) (r : Nat) : Bool := ((Nat.ble (r) (3)) && ((!(Nat.beq (r) (3))) || Reasoning.Clinic.hypotensive (v)))

public theorem graded_sound (v : Reasoning.Clinic.Vitals) (r : Nat) : ((gradedCheck (v) (r) = true) -> Graded (v) (r)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Reasoning.Clinic.Hypotensive, Reasoning.Clinic.hypotensive, Graded, gradedCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem Scale.sound : (LexLeanReasoning.Sound ((Reasoning.Main.gradedCheck)) ((Reasoning.Main.Graded))) :=
  Reasoning.Main.graded_sound

public theorem grade_admitted (v : Reasoning.Clinic.Vitals) : Reasoning.Clinic.Consistent (({ vitals := v, fever := 0, tachycardia := 0, tachypnea := 0, leukocytosis := 0, sirs := 0, sepsis := 0, shock := 0, level := 0 } : Reasoning.Clinic.Chart)) := by
  exact Reasoning.Clinic.admitted_consistent (v)

public theorem grade_pending (v : Reasoning.Clinic.Vitals) : (Reasoning.Clinic.Pending (({ vitals := v, fever := 0, tachycardia := 0, tachypnea := 0, leukocytosis := 0, sirs := 0, sepsis := 0, shock := 0, level := 0 } : Reasoning.Clinic.Chart)) < 11) := by
  exact Reasoning.Clinic.admitted_pending (v)

@[expose, reducible] public def Related (v : Reasoning.Clinic.Vitals) (c : Reasoning.Clinic.Chart) : Prop := (Reasoning.Clinic.Consistent (c) /\ (Reasoning.Clinic.Warranted (c) /\ ((((c).shock = 1) -> Reasoning.Clinic.Hypotensive ((c).vitals)) /\ (((c).vitals).systolic = (v).systolic))))

public theorem related_admitted (v : Reasoning.Clinic.Vitals) : Related (v) (({ vitals := v, fever := 0, tachycardia := 0, tachypnea := 0, leukocytosis := 0, sirs := 0, sepsis := 0, shock := 0, level := 0 } : Reasoning.Clinic.Chart)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Reasoning.Clinic.Consistent, Reasoning.Clinic.Hypotensive, Reasoning.Clinic.Warranted, Related, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem related_preserved (v : Reasoning.Clinic.Vitals) (s : Reasoning.Clinic.Chart) (t : Reasoning.Clinic.Chart) : (Related (v) (s) -> (Reasoning.Clinic.Justified (s) (t) -> Related (v) (t))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Reasoning.Clinic.Consistent, Reasoning.Clinic.Febrile, Reasoning.Clinic.Hypotensive, Reasoning.Clinic.Justified, Reasoning.Clinic.Leukocytic, Reasoning.Clinic.Tachycardic, Reasoning.Clinic.Tachypneic, Reasoning.Clinic.Warranted, Related, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem grade_correct (v : Reasoning.Clinic.Vitals) (chart : Reasoning.Clinic.Chart) (r : Nat) : (Related (v) (chart) -> ((Option.some ((chart).level) = Option.some (r)) -> Graded (v) (r))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Reasoning.Clinic.Consistent, Reasoning.Clinic.Hypotensive, Reasoning.Clinic.Warranted, Graded, Related, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public inductive Grade.Step where
  | Fever
  | Tachycardia
  | Tachypnea
  | Leukocytosis
  | Sirs
  | Sepsis
  | Shock
  | Escalate
public structure Grade.Ledger where
  iterations : Nat
  attempts : Nat
  firings : Nat
  expansions : Nat
  verifications : Nat
  frontier : Nat
@[expose] public def Grade.observe (v : Reasoning.Clinic.Vitals) : Reasoning.Clinic.Chart := ({ vitals := v, fever := 0, tachycardia := 0, tachypnea := 0, leukocytosis := 0, sirs := 0, sepsis := 0, shock := 0, level := 0 } : Reasoning.Clinic.Chart)
@[expose] public def Grade.fire (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Main.Grade.Step) : Option (Reasoning.Clinic.Chart) := (match __step with | Reasoning.Main.Grade.Step.Fever => Reasoning.Clinic.Fever.apply (__s) | Reasoning.Main.Grade.Step.Tachycardia => Reasoning.Clinic.Tachycardia.apply (__s) | Reasoning.Main.Grade.Step.Tachypnea => Reasoning.Clinic.Tachypnea.apply (__s) | Reasoning.Main.Grade.Step.Leukocytosis => Reasoning.Clinic.Leukocytosis.apply (__s) | Reasoning.Main.Grade.Step.Sirs => Reasoning.Clinic.Sirs.apply (__s) | Reasoning.Main.Grade.Step.Sepsis => Reasoning.Clinic.Sepsis.apply (__s) | Reasoning.Main.Grade.Step.Shock => Reasoning.Clinic.Shock.apply (__s) | Reasoning.Main.Grade.Step.Escalate => Reasoning.Clinic.Escalate.apply (__s))
@[expose] public def Grade.replay (__acc : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart)) (__step : Reasoning.Main.Grade.Step) : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match Reasoning.Main.Grade.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def Grade.follow (v : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Main.Grade.Step)) : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart) := (LexLeanCollections.listFold ((Reasoning.Main.Grade.replay)) (Except.ok (Reasoning.Main.Grade.observe (v))) (__trace) : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart))
@[expose] public def Grade.extract (chart : Reasoning.Clinic.Chart) : Option (Nat) := Option.some ((chart).level)
@[expose] public def Grade.accept (_v : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) : Option (Nat) := Reasoning.Main.Grade.extract (__s)
@[expose] public def Grade.answer (v : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Main.Grade.Step)) : Option (Nat) := (match Reasoning.Main.Grade.follow (v) (__trace) with | Except.ok __s => Reasoning.Main.Grade.accept (v) (__s) | Except.error _ => Option.none)
@[expose] public def Grade.select (__s : Reasoning.Clinic.Chart) : Option (Reasoning.Main.Grade.Step) := (match (match (match (if Reasoning.Clinic.Fever.guard (__s) then Option.some (Reasoning.Main.Grade.Step.Fever) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Tachycardia.guard (__s) then Option.some (Reasoning.Main.Grade.Step.Tachycardia) else Option.none)) with | Option.some __found => Option.some (__found) | Option.none => (match (if Reasoning.Clinic.Tachypnea.guard (__s) then Option.some (Reasoning.Main.Grade.Step.Tachypnea) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Leukocytosis.guard (__s) then Option.some (Reasoning.Main.Grade.Step.Leukocytosis) else Option.none))) with | Option.some __found => Option.some (__found) | Option.none => (match (match (if Reasoning.Clinic.Sirs.guard (__s) then Option.some (Reasoning.Main.Grade.Step.Sirs) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Sepsis.guard (__s) then Option.some (Reasoning.Main.Grade.Step.Sepsis) else Option.none)) with | Option.some __found => Option.some (__found) | Option.none => (match (if Reasoning.Clinic.Shock.guard (__s) then Option.some (Reasoning.Main.Grade.Step.Shock) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Escalate.guard (__s) then Option.some (Reasoning.Main.Grade.Step.Escalate) else Option.none))))
@[expose] public def Grade.attempts (__s : Reasoning.Clinic.Chart) : Nat := ((let __scan6l : (Prod (Bool) (Nat)) := (let __scan2l : (Prod (Bool) (Nat)) := (let __scan0l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Fever.guard (__s), 1); (if (__scan0l).1 then __scan0l else (let __scan0r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Tachycardia.guard (__s), 1); ((__scan0r).1, ((__scan0l).2 + (__scan0r).2))))); (if (__scan2l).1 then __scan2l else (let __scan2r : (Prod (Bool) (Nat)) := (let __scan1l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Tachypnea.guard (__s), 1); (if (__scan1l).1 then __scan1l else (let __scan1r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Leukocytosis.guard (__s), 1); ((__scan1r).1, ((__scan1l).2 + (__scan1r).2))))); ((__scan2r).1, ((__scan2l).2 + (__scan2r).2))))); (if (__scan6l).1 then __scan6l else (let __scan6r : (Prod (Bool) (Nat)) := (let __scan5l : (Prod (Bool) (Nat)) := (let __scan3l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Sirs.guard (__s), 1); (if (__scan3l).1 then __scan3l else (let __scan3r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Sepsis.guard (__s), 1); ((__scan3r).1, ((__scan3l).2 + (__scan3r).2))))); (if (__scan5l).1 then __scan5l else (let __scan5r : (Prod (Bool) (Nat)) := (let __scan4l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Shock.guard (__s), 1); (if (__scan4l).1 then __scan4l else (let __scan4r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Escalate.guard (__s), 1); ((__scan4r).1, ((__scan4l).2 + (__scan4r).2))))); ((__scan5r).1, ((__scan5l).2 + (__scan5r).2))))); ((__scan6r).1, ((__scan6l).2 + (__scan6r).2)))))).2
@[expose] public def Grade.next (__s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Chart) := (match Reasoning.Main.Grade.select (__s) with | Option.none => Option.none | Option.some __step => Reasoning.Main.Grade.fire (__s) (__step))
@[expose] public def Grade.saturate (v : Reasoning.Clinic.Vitals) : (Prod (Reasoning.Clinic.Chart) (Bool)) := (LexLeanCollections.iterateUntil ((Reasoning.Main.Grade.next)) (11) (Reasoning.Main.Grade.observe (v)) : (Prod (Reasoning.Clinic.Chart) (Bool)))
public structure Grade.Run where
  state : Reasoning.Clinic.Chart
  trace : List (Reasoning.Main.Grade.Step)
  ledger : Reasoning.Main.Grade.Ledger
@[expose] public def Grade.start (v : Reasoning.Clinic.Vitals) : Reasoning.Main.Grade.Run := ({ state := Reasoning.Main.Grade.observe (v), trace := ([] : List (Reasoning.Main.Grade.Step)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := 0 } : Reasoning.Main.Grade.Ledger) } : Reasoning.Main.Grade.Run)
@[expose] public def Grade.step (__r : Reasoning.Main.Grade.Run) : Option (Reasoning.Main.Grade.Run) := (match Reasoning.Main.Grade.select ((__r).state) with | Option.none => Option.none | Option.some __step => (match Reasoning.Main.Grade.fire ((__r).state) (__step) with | Option.none => Option.none | Option.some __t => Option.some (({ state := __t, trace := (LexLeanRuntime.append ((__r).trace) ((__step :: ([] : List (Reasoning.Main.Grade.Step)))) : List (Reasoning.Main.Grade.Step)), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + Reasoning.Main.Grade.attempts ((__r).state)), firings := (((__r).ledger).firings + 1), expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : Reasoning.Main.Grade.Ledger) } : Reasoning.Main.Grade.Run))))
@[expose] public def Grade.run (v : Reasoning.Clinic.Vitals) : (Prod (Reasoning.Main.Grade.Run) (Bool)) := (LexLeanCollections.iterateUntil ((Reasoning.Main.Grade.step)) (11) (Reasoning.Main.Grade.start (v)) : (Prod (Reasoning.Main.Grade.Run) (Bool)))
@[expose] public def Grade.account (v : Reasoning.Clinic.Vitals) : Reasoning.Main.Grade.Ledger := (let __final : (Prod (Reasoning.Main.Grade.Run) (Bool)) := Reasoning.Main.Grade.run (v); (match (__final).2 with | Bool.true => ({ iterations := (((__final).1).ledger).iterations, attempts := ((((__final).1).ledger).attempts + Reasoning.Main.Grade.attempts (((__final).1).state)), firings := (((__final).1).ledger).firings, expansions := (((__final).1).ledger).expansions, verifications := (((__final).1).ledger).verifications, frontier := (((__final).1).ledger).frontier } : Reasoning.Main.Grade.Ledger) | Bool.false => ((__final).1).ledger))
@[expose] public def Grade.conclude (v : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) : Except ((Prod Bool Bool)) (Nat) := (match Reasoning.Main.Grade.accept (v) (__s) with | Option.some __v => Except.ok (__v) | Option.none => (match Reasoning.Main.Grade.extract (__s) with | Option.some _ => Except.error (((true, false) : Prod Bool Bool)) | Option.none => Except.error (((false, true) : Prod Bool Bool))))
@[expose] public def Grade.verdict (v : Reasoning.Clinic.Vitals) : Except ((Prod Bool Bool)) (Nat) := (let __final : (Prod (Reasoning.Clinic.Chart) (Bool)) := Reasoning.Main.Grade.saturate (v); (match (__final).2 with | Bool.true => Reasoning.Main.Grade.conclude (v) ((__final).1) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
@[expose] public def Grade (v : Reasoning.Clinic.Vitals) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (Reasoning.Main.Grade.Step)))) := (let __final : (Prod (Reasoning.Main.Grade.Run) (Bool)) := Reasoning.Main.Grade.run (v); (match (__final).2 with | Bool.true => (match Reasoning.Main.Grade.conclude (v) (((__final).1).state) with | Except.ok __v => Except.ok ((__v, ((__final).1).trace)) | Except.error __e => Except.error (__e)) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
public theorem Grade.fire_sound (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Main.Grade.Step) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Main.Grade.fire (__s) (__step) = Option.some (__t)) -> Reasoning.Clinic.Justified (__s) (__t)) :=
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
public theorem Grade.fire_progress (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Main.Grade.Step) (__t : Reasoning.Clinic.Chart) : (Reasoning.Clinic.Consistent (__s) -> ((Reasoning.Main.Grade.fire (__s) (__step) = Option.some (__t)) -> (Reasoning.Clinic.Pending (__t) < Reasoning.Clinic.Pending (__s)))) :=
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
public theorem Grade.replay_fire (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Main.Grade.Step) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Main.Grade.fire (__s) (__step) = Option.some (__t)) -> (Reasoning.Main.Grade.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [Reasoning.Main.Grade.replay]
  rw [llE]
public theorem Grade.replay_sound (v : Reasoning.Clinic.Vitals) (__acc : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart)) (__step : Reasoning.Main.Grade.Step) : ((LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Main.Grade.observe (v)) (__acc)) -> (LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Main.Grade.observe (v)) (Reasoning.Main.Grade.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [Reasoning.Main.Grade.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (Reasoning.Main.Grade.fire_sound llS __step _ ‹_›)
public theorem Grade.derivation (v : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Main.Grade.Step)) : (LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Main.Grade.observe (v)) (Reasoning.Main.Grade.follow (v) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((Reasoning.Main.Grade.replay)) (fun (__acc : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart)) => (LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Main.Grade.observe (v)) (__acc))) (Reasoning.Main.Grade.replay_sound (v)) (__trace) (Except.ok (Reasoning.Main.Grade.observe (v))) (LexLeanReasoning.reachesStart ((Reasoning.Clinic.Justified)) (Reasoning.Main.Grade.observe (v))))
public theorem Grade.follow_invariant (v : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Main.Grade.Step)) (__s : Reasoning.Clinic.Chart) : ((Reasoning.Main.Grade.follow (v) (__trace) = Except.ok (__s)) -> Reasoning.Clinic.Consistent (__s)) :=
  (fun llE => (LexLeanReasoning.starPreserves ((Reasoning.Clinic.Justified)) ((Reasoning.Clinic.Consistent)) Reasoning.Clinic.Findings.preserves (Reasoning.Main.Grade.observe (v)) (__s) (Reasoning.Main.Grade.derivation (v) (__trace) (__s) llE) (Reasoning.Main.grade_admitted (v))))
public theorem Grade.follow_relation (v : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Main.Grade.Step)) (__s : Reasoning.Clinic.Chart) : ((Reasoning.Main.Grade.follow (v) (__trace) = Except.ok (__s)) -> Reasoning.Main.Related (v) (__s)) :=
  (fun llE => (LexLeanReasoning.starPreserves ((Reasoning.Clinic.Justified)) (fun (__s : Reasoning.Clinic.Chart) => Reasoning.Main.Related (v) (__s)) (Reasoning.Main.related_preserved (v)) (Reasoning.Main.Grade.observe (v)) (__s) (Reasoning.Main.Grade.derivation (v) (__trace) (__s) llE) (Reasoning.Main.related_admitted (v))))
public theorem Grade.accept_sound (v : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) (__v : Nat) : (Reasoning.Main.Related (v) (__s) -> ((Reasoning.Main.Grade.accept (v) (__s) = Option.some (__v)) -> Reasoning.Main.Graded (v) (__v))) :=
  (fun llJ llE => (Reasoning.Main.grade_correct (v) (__s) (__v) llJ llE))
public theorem Grade.next_sound (__s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Main.Grade.next (__s) = Option.some (__t)) -> Reasoning.Clinic.Justified (__s) (__t)) :=
by
  intro llE
  dsimp only [Reasoning.Main.Grade.next] at llE
  split at llE
  · cases llE
  · exact Reasoning.Main.Grade.fire_sound __s _ __t llE
public theorem Grade.next_preserves (__s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Main.Grade.next (__s) = Option.some (__t)) -> (Reasoning.Clinic.Consistent (__s) -> Reasoning.Clinic.Consistent (__t))) :=
  (fun llE llH => (Reasoning.Clinic.consistent_preserved (__s) (__t) llH (Reasoning.Main.Grade.next_sound (__s) (__t) llE)))
public theorem Grade.next_progress (__s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Main.Grade.next (__s) = Option.some (__t)) -> (Reasoning.Clinic.Consistent (__s) -> (Reasoning.Clinic.Pending (__t) < Reasoning.Clinic.Pending (__s)))) :=
by
  intro llE
  dsimp only [Reasoning.Main.Grade.next] at llE
  split at llE
  · cases llE
  · exact fun llJ => Reasoning.Main.Grade.fire_progress __s _ __t llJ llE
public theorem Grade.step_none (__r : Reasoning.Main.Grade.Run) : ((Reasoning.Main.Grade.step (__r) = Option.none) -> (Reasoning.Main.Grade.next ((__r).state) = Option.none)) :=
by
  intro llE
  dsimp only [Reasoning.Main.Grade.step] at llE
  split at llE
  · rename_i llH
    dsimp only [Reasoning.Main.Grade.next]
    rw [llH]
  · rename_i llH
    split at llE
    · dsimp only [Reasoning.Main.Grade.next]
      rw [llH]
      assumption
    · cases llE
public theorem Grade.step_some (__r : Reasoning.Main.Grade.Run) (__q : Reasoning.Main.Grade.Run) : ((Reasoning.Main.Grade.step (__r) = Option.some (__q)) -> (Reasoning.Main.Grade.next ((__r).state) = Option.some ((__q).state))) :=
by
  intro llE
  dsimp only [Reasoning.Main.Grade.step] at llE
  split at llE
  · cases llE
  · rename_i llH
    split at llE
    · cases llE
    · cases llE
      dsimp only [Reasoning.Main.Grade.next]
      rw [llH]
      assumption
public theorem Grade.step_trace (v : Reasoning.Clinic.Vitals) (__r : Reasoning.Main.Grade.Run) (__q : Reasoning.Main.Grade.Run) : ((Reasoning.Main.Grade.step (__r) = Option.some (__q)) -> ((Reasoning.Main.Grade.follow (v) ((__r).trace) = Except.ok ((__r).state)) -> (Reasoning.Main.Grade.follow (v) ((__q).trace) = Except.ok ((__q).state)))) :=
by
  intro llE llH
  dsimp only [Reasoning.Main.Grade.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      dsimp only [Reasoning.Main.Grade.follow]
      rw [LexLeanReasoning.foldSnoc]
      dsimp only [Reasoning.Main.Grade.follow] at llH
      rw [llH]
      apply Reasoning.Main.Grade.replay_fire
      assumption
public theorem Grade.step_count (__r : Reasoning.Main.Grade.Run) (__q : Reasoning.Main.Grade.Run) : ((Reasoning.Main.Grade.step (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Main.Grade.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Grade.step_fired (__r : Reasoning.Main.Grade.Run) (__q : Reasoning.Main.Grade.Run) : ((Reasoning.Main.Grade.step (__r) = Option.some (__q)) -> (((__q).ledger).firings = (((__r).ledger).firings + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Main.Grade.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Grade.saturate_derivation (v : Reasoning.Clinic.Vitals) : (LexLeanReasoning.Star ((Reasoning.Clinic.Justified)) (Reasoning.Main.Grade.observe (v)) ((Reasoning.Main.Grade.saturate (v)).1)) :=
by
  dsimp only [Reasoning.Main.Grade.saturate]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Main.Grade.next)) (fun (__s : Reasoning.Clinic.Chart) => (LexLeanReasoning.Star ((Reasoning.Clinic.Justified)) (Reasoning.Main.Grade.observe (v)) (__s))) (fun llA llB llE llH => (LexLeanReasoning.Star.tail _ llA llB llH (Reasoning.Main.Grade.next_sound llA llB llE))) (11) (Reasoning.Main.Grade.observe (v)) (LexLeanReasoning.Star.refl _))
public theorem Grade.run_state (v : Reasoning.Clinic.Vitals) : ((((Reasoning.Main.Grade.run (v)).1).state = (Reasoning.Main.Grade.saturate (v)).1) /\ ((Reasoning.Main.Grade.run (v)).2 = (Reasoning.Main.Grade.saturate (v)).2)) :=
by
  dsimp only [Reasoning.Main.Grade.run, Reasoning.Main.Grade.saturate]
  exact (LexLeanReasoning.iterateUntilSimulate ((Reasoning.Main.Grade.step)) ((Reasoning.Main.Grade.next)) ((fun (__r : Reasoning.Main.Grade.Run) => (__r).state)) Reasoning.Main.Grade.step_none Reasoning.Main.Grade.step_some (11) (Reasoning.Main.Grade.start (v)))
public theorem Grade.run_trace (v : Reasoning.Clinic.Vitals) : (Reasoning.Main.Grade.follow (v) (((Reasoning.Main.Grade.run (v)).1).trace) = Except.ok (((Reasoning.Main.Grade.run (v)).1).state)) :=
by
  dsimp only [Reasoning.Main.Grade.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Main.Grade.step)) (fun (__r : Reasoning.Main.Grade.Run) => (Reasoning.Main.Grade.follow (v) ((__r).trace) = Except.ok ((__r).state))) (Reasoning.Main.Grade.step_trace (v)) (11) (Reasoning.Main.Grade.start (v)) rfl)
public theorem Grade.iterations_bounded (v : Reasoning.Clinic.Vitals) : ((((Reasoning.Main.Grade.run (v)).1).ledger).iterations <= 11) :=
by
  dsimp only [Reasoning.Main.Grade.run]
  exact (LexLeanReasoning.iterateUntilCount ((Reasoning.Main.Grade.step)) ((fun (__r : Reasoning.Main.Grade.Run) => ((__r).ledger).iterations)) Reasoning.Main.Grade.step_count (11) (Reasoning.Main.Grade.start (v)) rfl)
public theorem Grade.firings_bounded (v : Reasoning.Clinic.Vitals) : ((((Reasoning.Main.Grade.run (v)).1).ledger).firings <= 11) :=
by
  dsimp only [Reasoning.Main.Grade.run]
  exact (LexLeanReasoning.iterateUntilCount ((Reasoning.Main.Grade.step)) ((fun (__r : Reasoning.Main.Grade.Run) => ((__r).ledger).firings)) Reasoning.Main.Grade.step_fired (11) (Reasoning.Main.Grade.start (v)) rfl)
public theorem Grade.saturate_invariant (v : Reasoning.Clinic.Vitals) : Reasoning.Clinic.Consistent ((Reasoning.Main.Grade.saturate (v)).1) :=
by
  dsimp only [Reasoning.Main.Grade.saturate]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Main.Grade.next)) ((Reasoning.Clinic.Consistent)) Reasoning.Main.Grade.next_preserves (11) (Reasoning.Main.Grade.observe (v)) (Reasoning.Main.grade_admitted (v)))
public theorem Grade.next_preserves_relation (v : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Main.Grade.next (__s) = Option.some (__t)) -> (Reasoning.Main.Related (v) (__s) -> Reasoning.Main.Related (v) (__t))) :=
  (fun llE llH => (Reasoning.Main.related_preserved (v) (__s) (__t) llH (Reasoning.Main.Grade.next_sound (__s) (__t) llE)))
public theorem Grade.saturate_relation (v : Reasoning.Clinic.Vitals) : Reasoning.Main.Related (v) ((Reasoning.Main.Grade.saturate (v)).1) :=
by
  dsimp only [Reasoning.Main.Grade.saturate]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Main.Grade.next)) (fun (__s : Reasoning.Clinic.Chart) => Reasoning.Main.Related (v) (__s)) (Reasoning.Main.Grade.next_preserves_relation (v)) (11) (Reasoning.Main.Grade.observe (v)) (Reasoning.Main.related_admitted (v)))
public theorem Grade.run_relation (v : Reasoning.Clinic.Vitals) : Reasoning.Main.Related (v) (((Reasoning.Main.Grade.run (v)).1).state) :=
by
  rw [And.left (Reasoning.Main.Grade.run_state v)]
  exact Reasoning.Main.Grade.saturate_relation v
public theorem Grade.saturates (v : Reasoning.Clinic.Vitals) : ((Reasoning.Main.Grade.saturate (v)).2 = true) :=
by
  dsimp only [Reasoning.Main.Grade.saturate]
  exact (LexLeanReasoning.iterateUntilStops ((Reasoning.Main.Grade.next)) ((Reasoning.Clinic.Consistent)) ((Reasoning.Clinic.Pending)) (fun llA llB llE llH => (And.intro (Reasoning.Main.Grade.next_preserves llA llB llE llH) (Reasoning.Main.Grade.next_progress llA llB llE llH))) (11) (Reasoning.Main.Grade.observe (v)) (Reasoning.Main.grade_admitted (v)) (Reasoning.Main.grade_pending (v)))
public theorem Grade.conclude_accept (v : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) (__v : Nat) : ((Reasoning.Main.Grade.conclude (v) (__s) = Except.ok (__v)) -> (Reasoning.Main.Grade.accept (v) (__s) = Option.some (__v))) :=
by
  intro llE
  dsimp only [Reasoning.Main.Grade.conclude] at llE
  split at llE
  · cases llE
    assumption
  · split at llE
    · cases llE
    · cases llE
public theorem Grade.conclude_sound (v : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) (__v : Nat) : (Reasoning.Main.Related (v) (__s) -> ((Reasoning.Main.Grade.conclude (v) (__s) = Except.ok (__v)) -> Reasoning.Main.Graded (v) (__v))) :=
by
  intro llJ llE
  dsimp only [Reasoning.Main.Grade.conclude] at llE
  split at llE
  · cases llE
    exact Reasoning.Main.Grade.accept_sound _ _ _ llJ ‹_›
  · split at llE
    · cases llE
    · cases llE
public theorem Grade.verdict_sound (v : Reasoning.Clinic.Vitals) : (forall (__v : Nat), ((Reasoning.Main.Grade.verdict (v) = Except.ok (__v)) -> Reasoning.Main.Graded (v) (__v))) :=
by
  intro llV llE
  have llJ := Reasoning.Main.Grade.saturate_relation v
  dsimp only [Reasoning.Main.Grade.verdict] at llE
  generalize llRun : Reasoning.Main.Grade.saturate v = llR at llE llJ
  split at llE
  · exact Reasoning.Main.Grade.conclude_sound _ _ _ llJ llE
  · cases llE
public theorem Grade.explained (v : Reasoning.Clinic.Vitals) : (forall (__v : Nat), (forall (__trace : List (Reasoning.Main.Grade.Step)), ((Reasoning.Main.Grade (v) = Except.ok ((__v, __trace))) -> ((Reasoning.Main.Grade.answer (v) (__trace) = Option.some (__v)) /\ Reasoning.Main.Graded (v) (__v))))) :=
by
  intro llV llT llE
  have llTrace := Reasoning.Main.Grade.run_trace v
  have llJ := Reasoning.Main.Grade.run_relation v
  dsimp only [Reasoning.Main.Grade] at llE
  generalize llRun : Reasoning.Main.Grade.run v = llR at llE llTrace llJ
  split at llE
  · split at llE
    · rename_i llW llH
      cases llE
      exact And.intro (by dsimp only [Reasoning.Main.Grade.answer]; rw [llTrace]; exact Reasoning.Main.Grade.conclude_accept _ _ _ llH) (Reasoning.Main.Grade.conclude_sound _ _ _ llJ llH)
    · cases llE
  · cases llE

@[expose, reducible] public def Capped (_v : Reasoning.Clinic.Vitals) (r : Nat) : Prop := (r <= 3)

@[expose] public def cappedCheck (_v : Reasoning.Clinic.Vitals) (r : Nat) : Bool := (Nat.ble (r) (3))

public theorem capped_sound (v : Reasoning.Clinic.Vitals) (r : Nat) : ((cappedCheck (v) (r) = true) -> Capped (v) (r)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Capped, cappedCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem Ceiling.sound : (LexLeanReasoning.Sound ((Reasoning.Main.cappedCheck)) ((Reasoning.Main.Capped))) :=
  Reasoning.Main.capped_sound

public theorem cap_correct (v : Reasoning.Clinic.Vitals) (chart : Reasoning.Clinic.Chart) (r : Nat) : ((Option.some ((LexLeanRuntime.subtract (3) ((LexLeanRuntime.subtract (3) ((chart).level) : Nat)) : Nat)) = Option.some (r)) -> Capped (v) (r)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Capped, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public inductive Cap.Step where
  | Fever
  | Tachycardia
  | Tachypnea
  | Leukocytosis
  | Sirs
  | Sepsis
  | Shock
  | Escalate
public structure Cap.Ledger where
  iterations : Nat
  attempts : Nat
  firings : Nat
  expansions : Nat
  verifications : Nat
  frontier : Nat
@[expose] public def Cap.observe (v : Reasoning.Clinic.Vitals) : Reasoning.Clinic.Chart := ({ vitals := v, fever := 0, tachycardia := 0, tachypnea := 0, leukocytosis := 0, sirs := 0, sepsis := 0, shock := 0, level := 0 } : Reasoning.Clinic.Chart)
@[expose] public def Cap.fire (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Main.Cap.Step) : Option (Reasoning.Clinic.Chart) := (match __step with | Reasoning.Main.Cap.Step.Fever => Reasoning.Clinic.Fever.apply (__s) | Reasoning.Main.Cap.Step.Tachycardia => Reasoning.Clinic.Tachycardia.apply (__s) | Reasoning.Main.Cap.Step.Tachypnea => Reasoning.Clinic.Tachypnea.apply (__s) | Reasoning.Main.Cap.Step.Leukocytosis => Reasoning.Clinic.Leukocytosis.apply (__s) | Reasoning.Main.Cap.Step.Sirs => Reasoning.Clinic.Sirs.apply (__s) | Reasoning.Main.Cap.Step.Sepsis => Reasoning.Clinic.Sepsis.apply (__s) | Reasoning.Main.Cap.Step.Shock => Reasoning.Clinic.Shock.apply (__s) | Reasoning.Main.Cap.Step.Escalate => Reasoning.Clinic.Escalate.apply (__s))
@[expose] public def Cap.replay (__acc : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart)) (__step : Reasoning.Main.Cap.Step) : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match Reasoning.Main.Cap.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def Cap.follow (v : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Main.Cap.Step)) : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart) := (LexLeanCollections.listFold ((Reasoning.Main.Cap.replay)) (Except.ok (Reasoning.Main.Cap.observe (v))) (__trace) : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart))
@[expose] public def Cap.extract (chart : Reasoning.Clinic.Chart) : Option (Nat) := Option.some ((LexLeanRuntime.subtract (3) ((LexLeanRuntime.subtract (3) ((chart).level) : Nat)) : Nat))
@[expose] public def Cap.accept (_v : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) : Option (Nat) := Reasoning.Main.Cap.extract (__s)
@[expose] public def Cap.answer (v : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Main.Cap.Step)) : Option (Nat) := (match Reasoning.Main.Cap.follow (v) (__trace) with | Except.ok __s => Reasoning.Main.Cap.accept (v) (__s) | Except.error _ => Option.none)
@[expose] public def Cap.select (__s : Reasoning.Clinic.Chart) : Option (Reasoning.Main.Cap.Step) := (match (match (match (if Reasoning.Clinic.Fever.guard (__s) then Option.some (Reasoning.Main.Cap.Step.Fever) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Tachycardia.guard (__s) then Option.some (Reasoning.Main.Cap.Step.Tachycardia) else Option.none)) with | Option.some __found => Option.some (__found) | Option.none => (match (if Reasoning.Clinic.Tachypnea.guard (__s) then Option.some (Reasoning.Main.Cap.Step.Tachypnea) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Leukocytosis.guard (__s) then Option.some (Reasoning.Main.Cap.Step.Leukocytosis) else Option.none))) with | Option.some __found => Option.some (__found) | Option.none => (match (match (if Reasoning.Clinic.Sirs.guard (__s) then Option.some (Reasoning.Main.Cap.Step.Sirs) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Sepsis.guard (__s) then Option.some (Reasoning.Main.Cap.Step.Sepsis) else Option.none)) with | Option.some __found => Option.some (__found) | Option.none => (match (if Reasoning.Clinic.Shock.guard (__s) then Option.some (Reasoning.Main.Cap.Step.Shock) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Escalate.guard (__s) then Option.some (Reasoning.Main.Cap.Step.Escalate) else Option.none))))
@[expose] public def Cap.attempts (__s : Reasoning.Clinic.Chart) : Nat := ((let __scan6l : (Prod (Bool) (Nat)) := (let __scan2l : (Prod (Bool) (Nat)) := (let __scan0l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Fever.guard (__s), 1); (if (__scan0l).1 then __scan0l else (let __scan0r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Tachycardia.guard (__s), 1); ((__scan0r).1, ((__scan0l).2 + (__scan0r).2))))); (if (__scan2l).1 then __scan2l else (let __scan2r : (Prod (Bool) (Nat)) := (let __scan1l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Tachypnea.guard (__s), 1); (if (__scan1l).1 then __scan1l else (let __scan1r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Leukocytosis.guard (__s), 1); ((__scan1r).1, ((__scan1l).2 + (__scan1r).2))))); ((__scan2r).1, ((__scan2l).2 + (__scan2r).2))))); (if (__scan6l).1 then __scan6l else (let __scan6r : (Prod (Bool) (Nat)) := (let __scan5l : (Prod (Bool) (Nat)) := (let __scan3l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Sirs.guard (__s), 1); (if (__scan3l).1 then __scan3l else (let __scan3r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Sepsis.guard (__s), 1); ((__scan3r).1, ((__scan3l).2 + (__scan3r).2))))); (if (__scan5l).1 then __scan5l else (let __scan5r : (Prod (Bool) (Nat)) := (let __scan4l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Shock.guard (__s), 1); (if (__scan4l).1 then __scan4l else (let __scan4r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Escalate.guard (__s), 1); ((__scan4r).1, ((__scan4l).2 + (__scan4r).2))))); ((__scan5r).1, ((__scan5l).2 + (__scan5r).2))))); ((__scan6r).1, ((__scan6l).2 + (__scan6r).2)))))).2
@[expose] public def Cap.next (__s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Chart) := (match Reasoning.Main.Cap.select (__s) with | Option.none => Option.none | Option.some __step => Reasoning.Main.Cap.fire (__s) (__step))
@[expose] public def Cap.saturate (v : Reasoning.Clinic.Vitals) : (Prod (Reasoning.Clinic.Chart) (Bool)) := (LexLeanCollections.iterateUntil ((Reasoning.Main.Cap.next)) (11) (Reasoning.Main.Cap.observe (v)) : (Prod (Reasoning.Clinic.Chart) (Bool)))
public structure Cap.Run where
  state : Reasoning.Clinic.Chart
  trace : List (Reasoning.Main.Cap.Step)
  ledger : Reasoning.Main.Cap.Ledger
@[expose] public def Cap.start (v : Reasoning.Clinic.Vitals) : Reasoning.Main.Cap.Run := ({ state := Reasoning.Main.Cap.observe (v), trace := ([] : List (Reasoning.Main.Cap.Step)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := 0 } : Reasoning.Main.Cap.Ledger) } : Reasoning.Main.Cap.Run)
@[expose] public def Cap.step (__r : Reasoning.Main.Cap.Run) : Option (Reasoning.Main.Cap.Run) := (match Reasoning.Main.Cap.select ((__r).state) with | Option.none => Option.none | Option.some __step => (match Reasoning.Main.Cap.fire ((__r).state) (__step) with | Option.none => Option.none | Option.some __t => Option.some (({ state := __t, trace := (LexLeanRuntime.append ((__r).trace) ((__step :: ([] : List (Reasoning.Main.Cap.Step)))) : List (Reasoning.Main.Cap.Step)), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + Reasoning.Main.Cap.attempts ((__r).state)), firings := (((__r).ledger).firings + 1), expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : Reasoning.Main.Cap.Ledger) } : Reasoning.Main.Cap.Run))))
@[expose] public def Cap.run (v : Reasoning.Clinic.Vitals) : (Prod (Reasoning.Main.Cap.Run) (Bool)) := (LexLeanCollections.iterateUntil ((Reasoning.Main.Cap.step)) (11) (Reasoning.Main.Cap.start (v)) : (Prod (Reasoning.Main.Cap.Run) (Bool)))
@[expose] public def Cap.account (v : Reasoning.Clinic.Vitals) : Reasoning.Main.Cap.Ledger := (let __final : (Prod (Reasoning.Main.Cap.Run) (Bool)) := Reasoning.Main.Cap.run (v); (match (__final).2 with | Bool.true => ({ iterations := (((__final).1).ledger).iterations, attempts := ((((__final).1).ledger).attempts + Reasoning.Main.Cap.attempts (((__final).1).state)), firings := (((__final).1).ledger).firings, expansions := (((__final).1).ledger).expansions, verifications := (((__final).1).ledger).verifications, frontier := (((__final).1).ledger).frontier } : Reasoning.Main.Cap.Ledger) | Bool.false => ((__final).1).ledger))
@[expose] public def Cap.conclude (v : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) : Except ((Prod Bool Bool)) (Nat) := (match Reasoning.Main.Cap.accept (v) (__s) with | Option.some __v => Except.ok (__v) | Option.none => (match Reasoning.Main.Cap.extract (__s) with | Option.some _ => Except.error (((true, false) : Prod Bool Bool)) | Option.none => Except.error (((false, true) : Prod Bool Bool))))
@[expose] public def Cap.verdict (v : Reasoning.Clinic.Vitals) : Except ((Prod Bool Bool)) (Nat) := (let __final : (Prod (Reasoning.Clinic.Chart) (Bool)) := Reasoning.Main.Cap.saturate (v); (match (__final).2 with | Bool.true => Reasoning.Main.Cap.conclude (v) ((__final).1) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
@[expose] public def Cap (v : Reasoning.Clinic.Vitals) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (Reasoning.Main.Cap.Step)))) := (let __final : (Prod (Reasoning.Main.Cap.Run) (Bool)) := Reasoning.Main.Cap.run (v); (match (__final).2 with | Bool.true => (match Reasoning.Main.Cap.conclude (v) (((__final).1).state) with | Except.ok __v => Except.ok ((__v, ((__final).1).trace)) | Except.error __e => Except.error (__e)) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
public theorem Cap.fire_sound (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Main.Cap.Step) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Main.Cap.fire (__s) (__step) = Option.some (__t)) -> Reasoning.Clinic.Justified (__s) (__t)) :=
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
public theorem Cap.replay_fire (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Main.Cap.Step) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Main.Cap.fire (__s) (__step) = Option.some (__t)) -> (Reasoning.Main.Cap.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [Reasoning.Main.Cap.replay]
  rw [llE]
public theorem Cap.replay_sound (v : Reasoning.Clinic.Vitals) (__acc : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart)) (__step : Reasoning.Main.Cap.Step) : ((LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Main.Cap.observe (v)) (__acc)) -> (LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Main.Cap.observe (v)) (Reasoning.Main.Cap.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [Reasoning.Main.Cap.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (Reasoning.Main.Cap.fire_sound llS __step _ ‹_›)
public theorem Cap.derivation (v : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Main.Cap.Step)) : (LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Main.Cap.observe (v)) (Reasoning.Main.Cap.follow (v) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((Reasoning.Main.Cap.replay)) (fun (__acc : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart)) => (LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Main.Cap.observe (v)) (__acc))) (Reasoning.Main.Cap.replay_sound (v)) (__trace) (Except.ok (Reasoning.Main.Cap.observe (v))) (LexLeanReasoning.reachesStart ((Reasoning.Clinic.Justified)) (Reasoning.Main.Cap.observe (v))))
public theorem Cap.follow_invariant (v : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Main.Cap.Step)) (__s : Reasoning.Clinic.Chart) : (Reasoning.Clinic.Consistent (Reasoning.Main.Cap.observe (v)) -> ((Reasoning.Main.Cap.follow (v) (__trace) = Except.ok (__s)) -> Reasoning.Clinic.Consistent (__s))) :=
  (fun llI llE => (LexLeanReasoning.starPreserves ((Reasoning.Clinic.Justified)) ((Reasoning.Clinic.Consistent)) Reasoning.Clinic.Findings.preserves (Reasoning.Main.Cap.observe (v)) (__s) (Reasoning.Main.Cap.derivation (v) (__trace) (__s) llE) llI))
public theorem Cap.accept_sound (v : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) (__v : Nat) : ((Reasoning.Main.Cap.accept (v) (__s) = Option.some (__v)) -> Reasoning.Main.Capped (v) (__v)) :=
  (fun llE => (Reasoning.Main.cap_correct (v) (__s) (__v) llE))
public theorem Cap.next_sound (__s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Main.Cap.next (__s) = Option.some (__t)) -> Reasoning.Clinic.Justified (__s) (__t)) :=
by
  intro llE
  dsimp only [Reasoning.Main.Cap.next] at llE
  split at llE
  · cases llE
  · exact Reasoning.Main.Cap.fire_sound __s _ __t llE
public theorem Cap.next_preserves (__s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Main.Cap.next (__s) = Option.some (__t)) -> (Reasoning.Clinic.Consistent (__s) -> Reasoning.Clinic.Consistent (__t))) :=
  (fun llE llH => (Reasoning.Clinic.consistent_preserved (__s) (__t) llH (Reasoning.Main.Cap.next_sound (__s) (__t) llE)))
public theorem Cap.step_none (__r : Reasoning.Main.Cap.Run) : ((Reasoning.Main.Cap.step (__r) = Option.none) -> (Reasoning.Main.Cap.next ((__r).state) = Option.none)) :=
by
  intro llE
  dsimp only [Reasoning.Main.Cap.step] at llE
  split at llE
  · rename_i llH
    dsimp only [Reasoning.Main.Cap.next]
    rw [llH]
  · rename_i llH
    split at llE
    · dsimp only [Reasoning.Main.Cap.next]
      rw [llH]
      assumption
    · cases llE
public theorem Cap.step_some (__r : Reasoning.Main.Cap.Run) (__q : Reasoning.Main.Cap.Run) : ((Reasoning.Main.Cap.step (__r) = Option.some (__q)) -> (Reasoning.Main.Cap.next ((__r).state) = Option.some ((__q).state))) :=
by
  intro llE
  dsimp only [Reasoning.Main.Cap.step] at llE
  split at llE
  · cases llE
  · rename_i llH
    split at llE
    · cases llE
    · cases llE
      dsimp only [Reasoning.Main.Cap.next]
      rw [llH]
      assumption
public theorem Cap.step_trace (v : Reasoning.Clinic.Vitals) (__r : Reasoning.Main.Cap.Run) (__q : Reasoning.Main.Cap.Run) : ((Reasoning.Main.Cap.step (__r) = Option.some (__q)) -> ((Reasoning.Main.Cap.follow (v) ((__r).trace) = Except.ok ((__r).state)) -> (Reasoning.Main.Cap.follow (v) ((__q).trace) = Except.ok ((__q).state)))) :=
by
  intro llE llH
  dsimp only [Reasoning.Main.Cap.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      dsimp only [Reasoning.Main.Cap.follow]
      rw [LexLeanReasoning.foldSnoc]
      dsimp only [Reasoning.Main.Cap.follow] at llH
      rw [llH]
      apply Reasoning.Main.Cap.replay_fire
      assumption
public theorem Cap.step_count (__r : Reasoning.Main.Cap.Run) (__q : Reasoning.Main.Cap.Run) : ((Reasoning.Main.Cap.step (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Main.Cap.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Cap.step_fired (__r : Reasoning.Main.Cap.Run) (__q : Reasoning.Main.Cap.Run) : ((Reasoning.Main.Cap.step (__r) = Option.some (__q)) -> (((__q).ledger).firings = (((__r).ledger).firings + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Main.Cap.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Cap.saturate_derivation (v : Reasoning.Clinic.Vitals) : (LexLeanReasoning.Star ((Reasoning.Clinic.Justified)) (Reasoning.Main.Cap.observe (v)) ((Reasoning.Main.Cap.saturate (v)).1)) :=
by
  dsimp only [Reasoning.Main.Cap.saturate]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Main.Cap.next)) (fun (__s : Reasoning.Clinic.Chart) => (LexLeanReasoning.Star ((Reasoning.Clinic.Justified)) (Reasoning.Main.Cap.observe (v)) (__s))) (fun llA llB llE llH => (LexLeanReasoning.Star.tail _ llA llB llH (Reasoning.Main.Cap.next_sound llA llB llE))) (11) (Reasoning.Main.Cap.observe (v)) (LexLeanReasoning.Star.refl _))
public theorem Cap.run_state (v : Reasoning.Clinic.Vitals) : ((((Reasoning.Main.Cap.run (v)).1).state = (Reasoning.Main.Cap.saturate (v)).1) /\ ((Reasoning.Main.Cap.run (v)).2 = (Reasoning.Main.Cap.saturate (v)).2)) :=
by
  dsimp only [Reasoning.Main.Cap.run, Reasoning.Main.Cap.saturate]
  exact (LexLeanReasoning.iterateUntilSimulate ((Reasoning.Main.Cap.step)) ((Reasoning.Main.Cap.next)) ((fun (__r : Reasoning.Main.Cap.Run) => (__r).state)) Reasoning.Main.Cap.step_none Reasoning.Main.Cap.step_some (11) (Reasoning.Main.Cap.start (v)))
public theorem Cap.run_trace (v : Reasoning.Clinic.Vitals) : (Reasoning.Main.Cap.follow (v) (((Reasoning.Main.Cap.run (v)).1).trace) = Except.ok (((Reasoning.Main.Cap.run (v)).1).state)) :=
by
  dsimp only [Reasoning.Main.Cap.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Main.Cap.step)) (fun (__r : Reasoning.Main.Cap.Run) => (Reasoning.Main.Cap.follow (v) ((__r).trace) = Except.ok ((__r).state))) (Reasoning.Main.Cap.step_trace (v)) (11) (Reasoning.Main.Cap.start (v)) rfl)
public theorem Cap.iterations_bounded (v : Reasoning.Clinic.Vitals) : ((((Reasoning.Main.Cap.run (v)).1).ledger).iterations <= 11) :=
by
  dsimp only [Reasoning.Main.Cap.run]
  exact (LexLeanReasoning.iterateUntilCount ((Reasoning.Main.Cap.step)) ((fun (__r : Reasoning.Main.Cap.Run) => ((__r).ledger).iterations)) Reasoning.Main.Cap.step_count (11) (Reasoning.Main.Cap.start (v)) rfl)
public theorem Cap.firings_bounded (v : Reasoning.Clinic.Vitals) : ((((Reasoning.Main.Cap.run (v)).1).ledger).firings <= 11) :=
by
  dsimp only [Reasoning.Main.Cap.run]
  exact (LexLeanReasoning.iterateUntilCount ((Reasoning.Main.Cap.step)) ((fun (__r : Reasoning.Main.Cap.Run) => ((__r).ledger).firings)) Reasoning.Main.Cap.step_fired (11) (Reasoning.Main.Cap.start (v)) rfl)
public theorem Cap.saturate_invariant (v : Reasoning.Clinic.Vitals) : (Reasoning.Clinic.Consistent (Reasoning.Main.Cap.observe (v)) -> Reasoning.Clinic.Consistent ((Reasoning.Main.Cap.saturate (v)).1)) :=
by
  dsimp only [Reasoning.Main.Cap.saturate]
  exact (fun llI => (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Main.Cap.next)) ((Reasoning.Clinic.Consistent)) Reasoning.Main.Cap.next_preserves (11) (Reasoning.Main.Cap.observe (v)) llI))
public theorem Cap.conclude_accept (v : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) (__v : Nat) : ((Reasoning.Main.Cap.conclude (v) (__s) = Except.ok (__v)) -> (Reasoning.Main.Cap.accept (v) (__s) = Option.some (__v))) :=
by
  intro llE
  dsimp only [Reasoning.Main.Cap.conclude] at llE
  split at llE
  · cases llE
    assumption
  · split at llE
    · cases llE
    · cases llE
public theorem Cap.conclude_sound (v : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) (__v : Nat) : ((Reasoning.Main.Cap.conclude (v) (__s) = Except.ok (__v)) -> Reasoning.Main.Capped (v) (__v)) :=
by
  intro llE
  dsimp only [Reasoning.Main.Cap.conclude] at llE
  split at llE
  · cases llE
    exact Reasoning.Main.Cap.accept_sound _ _ _ ‹_›
  · split at llE
    · cases llE
    · cases llE
public theorem Cap.verdict_sound (v : Reasoning.Clinic.Vitals) : (forall (__v : Nat), ((Reasoning.Main.Cap.verdict (v) = Except.ok (__v)) -> Reasoning.Main.Capped (v) (__v))) :=
by
  intro llV llE
  dsimp only [Reasoning.Main.Cap.verdict] at llE
  generalize llRun : Reasoning.Main.Cap.saturate v = llR at llE
  split at llE
  · exact Reasoning.Main.Cap.conclude_sound _ _ _ llE
  · cases llE
public theorem Cap.explained (v : Reasoning.Clinic.Vitals) : (forall (__v : Nat), (forall (__trace : List (Reasoning.Main.Cap.Step)), ((Reasoning.Main.Cap (v) = Except.ok ((__v, __trace))) -> ((Reasoning.Main.Cap.answer (v) (__trace) = Option.some (__v)) /\ Reasoning.Main.Capped (v) (__v))))) :=
by
  intro llV llT llE
  have llTrace := Reasoning.Main.Cap.run_trace v
  dsimp only [Reasoning.Main.Cap] at llE
  generalize llRun : Reasoning.Main.Cap.run v = llR at llE llTrace
  split at llE
  · split at llE
    · rename_i llW llH
      cases llE
      exact And.intro (by dsimp only [Reasoning.Main.Cap.answer]; rw [llTrace]; exact Reasoning.Main.Cap.conclude_accept _ _ _ llH) (Reasoning.Main.Cap.conclude_sound _ _ _ llH)
    · cases llE
  · cases llE

@[expose] public def triageLevel (temperature : Nat) (heartRate : Nat) (respiratoryRate : Nat) (whiteCells : Nat) (systolic : Nat) (infection : Nat) : Nat := (match Reasoning.Clinic.Triage.verdict (({ temperature := temperature, heartRate := heartRate, respiratoryRate := respiratoryRate, whiteCells := whiteCells, systolic := systolic, infection := infection } : Reasoning.Clinic.Vitals)) with | Except.ok level => level | Except.error _ => 9)

@[expose] public def gatewayLevel (temperature : Nat) (heartRate : Nat) (respiratoryRate : Nat) (whiteCells : Nat) (systolic : Nat) (infection : Nat) : Nat := (match GatewayModel (({ temperature := temperature, heartRate := heartRate, respiratoryRate := respiratoryRate, whiteCells := whiteCells, systolic := systolic, infection := infection } : Reasoning.Clinic.Vitals)) with | Except.ok level => level | Except.error _ => 9)

@[expose] public def triageSteps (temperature : Nat) (heartRate : Nat) (respiratoryRate : Nat) (whiteCells : Nat) (systolic : Nat) (infection : Nat) : Nat := (match Reasoning.Clinic.Triage (({ temperature := temperature, heartRate := heartRate, respiratoryRate := respiratoryRate, whiteCells := whiteCells, systolic := systolic, infection := infection } : Reasoning.Clinic.Vitals)) with | Except.ok explained => (LexLeanRuntime.length ((explained).2) : Nat) | Except.error _ => 0)

@[expose] public def planLeft (target : Nat) : Nat := (match Reasoning.Planner.Plan.verdict (target) with | Except.ok jugs => (jugs).1 | Except.error _ => 9)

@[expose] public def doseLevel (w : Nat) : Nat := (match Reasoning.Screening.Dose.verdict (w) with | Except.ok dose => dose | Except.error _ => 9)

@[expose] public def gradeLevel (temperature : Nat) (heartRate : Nat) (respiratoryRate : Nat) (whiteCells : Nat) (systolic : Nat) (infection : Nat) : Nat := (match Grade.verdict (({ temperature := temperature, heartRate := heartRate, respiratoryRate := respiratoryRate, whiteCells := whiteCells, systolic := systolic, infection := infection } : Reasoning.Clinic.Vitals)) with | Except.ok level => level | Except.error _ => 9)

@[expose] public def screenDose (w : Nat) : Nat := (match Reasoning.Screening.Screen.verdict (w) with | Except.ok dose => dose | Except.error _ => 9)

@[expose] public def spendLeft (x : Nat) : Nat := (match Reasoning.Budget.Spend.verdict (Nat) ((x, x)) with | Except.ok left => left | Except.error _ => 9)

public theorem triage_shock : (Reasoning.Clinic.Triage (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals)) = Except.ok ((3, (Reasoning.Clinic.Triage.Step.Fever :: (Reasoning.Clinic.Triage.Step.Tachycardia :: (Reasoning.Clinic.Triage.Step.Tachypnea :: (Reasoning.Clinic.Triage.Step.Leukocytosis :: (Reasoning.Clinic.Triage.Step.Sirs :: (Reasoning.Clinic.Triage.Step.Sepsis :: (Reasoning.Clinic.Triage.Step.Shock :: (Reasoning.Clinic.Triage.Step.Escalate :: (Reasoning.Clinic.Triage.Step.Escalate :: (Reasoning.Clinic.Triage.Step.Escalate :: ([] : List (Reasoning.Clinic.Triage.Step))))))))))))))) := by
  rfl

public theorem triage_level : (triageLevel (392) (118) (26) (15) (82) (1) = 3) := by
  rfl

public theorem triage_replays : (Reasoning.Clinic.Triage.answer (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals)) ((Reasoning.Clinic.Triage.Step.Fever :: (Reasoning.Clinic.Triage.Step.Tachycardia :: (Reasoning.Clinic.Triage.Step.Tachypnea :: (Reasoning.Clinic.Triage.Step.Leukocytosis :: (Reasoning.Clinic.Triage.Step.Sirs :: (Reasoning.Clinic.Triage.Step.Sepsis :: (Reasoning.Clinic.Triage.Step.Shock :: (Reasoning.Clinic.Triage.Step.Escalate :: (Reasoning.Clinic.Triage.Step.Escalate :: (Reasoning.Clinic.Triage.Step.Escalate :: ([] : List (Reasoning.Clinic.Triage.Step))))))))))))) = Option.some (3)) := by
  rfl

public theorem triage_iterations : ((((Reasoning.Clinic.Triage.run (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).1).ledger).iterations = 10) := by
  rfl

public theorem triage_well : (Reasoning.Clinic.Triage (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals)) = Except.ok ((0, ([] : List (Reasoning.Clinic.Triage.Step))))) := by
  rfl

public theorem triage_forged : (Reasoning.Clinic.Triage.follow (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals)) ((Reasoning.Clinic.Triage.Step.Sepsis :: ([] : List (Reasoning.Clinic.Triage.Step)))) = Except.error (((true, true) : Prod Bool Bool))) := by
  rfl

public theorem triage_inapplicable : (Reasoning.Clinic.Triage.fire (Reasoning.Clinic.Triage.observe (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))) (Reasoning.Clinic.Triage.Step.Shock) = Option.none) := by
  rfl

public theorem review_exhausted : (Reasoning.Clinic.Review.verdict (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals)) = Except.error (((false, false) : Prod Bool Bool))) := by
  rfl

public theorem review_rejected : (Reasoning.Clinic.Review.verdict (({ temperature := 390, heartRate := 100, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals)) = Except.error (((true, false) : Prod Bool Bool))) := by
  rfl

public theorem gateway_level : (gatewayLevel (392) (118) (26) (15) (82) (1) = 3) := by
  rfl

public theorem spend_drained : (Reasoning.Budget.Spend (Nat) ((2, 2)) = Except.ok ((0, (Reasoning.Budget.Spend.Step.Tick :: (Reasoning.Budget.Spend.Step.Tick :: (Reasoning.Budget.Spend.Step.Tick :: (Reasoning.Budget.Spend.Step.Burn (2) :: ([] : List (Reasoning.Budget.Spend.Step (Nat)))))))))) := by
  rfl

public theorem refund_drained : (Reasoning.Budget.Refund.verdict (2) = Except.ok (0)) := by
  rfl

public theorem refund_unsolved : (Reasoning.Budget.Refund.verdict (7) = Except.error (((false, true) : Prod Bool Bool))) := by
  rfl

public theorem plan_found : (Reasoning.Planner.Plan (2) = Except.ok (((2, 2), (Reasoning.Planner.Plan.Step.FillA :: (Reasoning.Planner.Plan.Step.Pour (2) :: ([] : List (Reasoning.Planner.Plan.Step))))))) := by
  rfl

public theorem plan_unsolved : (Reasoning.Planner.Plan.verdict (5) = Except.error (((false, true) : Prod Bool Bool))) := by
  rfl

public theorem plan_frontier : ((((Reasoning.Planner.Plan.run (2)).1).ledger).frontier = 4) := by
  rfl

public theorem screen_found : (Reasoning.Screening.Screen (60) = Except.ok ((30, (Reasoning.Screening.Screen.Step.Propose (30) :: ([] : List (Reasoning.Screening.Screen.Step)))))) := by
  rfl

public theorem grade_shock : (gradeLevel (392) (118) (26) (15) (82) (1) = 3) := by
  rfl

public theorem dose_found : (Reasoning.Screening.Dose (60) = Except.ok ((30, (Reasoning.Screening.Dose.Step.candidate (30) :: ([] : List (Reasoning.Screening.Dose.Step)))))) := by
  rfl

public theorem dose_exhausted : (Reasoning.Screening.Dose.verdict (120) = Except.error (((false, false) : Prod Bool Bool))) := by
  rfl

public theorem screen_truncated : (Reasoning.Screening.Screen.verdict (100) = Except.error (((false, false) : Prod Bool Bool))) := by
  rfl

public theorem triage_ledger : (((Reasoning.Clinic.Triage.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).iterations, ((Reasoning.Clinic.Triage.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).attempts, ((Reasoning.Clinic.Triage.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).firings, ((Reasoning.Clinic.Triage.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).expansions, ((Reasoning.Clinic.Triage.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).verifications, (Reasoning.Clinic.Triage.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).frontier))))) = (10, (60, (10, (0, (1, 0)))))) := by
  rfl

public theorem triage_well_ledger : (((Reasoning.Clinic.Triage.account (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).iterations, ((Reasoning.Clinic.Triage.account (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).attempts, ((Reasoning.Clinic.Triage.account (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).firings, ((Reasoning.Clinic.Triage.account (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).expansions, ((Reasoning.Clinic.Triage.account (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).verifications, (Reasoning.Clinic.Triage.account (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).frontier))))) = (0, (8, (0, (0, (1, 0)))))) := by
  rfl

public theorem review_ledger : (((Reasoning.Clinic.Review.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).iterations, ((Reasoning.Clinic.Review.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).attempts, ((Reasoning.Clinic.Review.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).firings, ((Reasoning.Clinic.Review.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).expansions, ((Reasoning.Clinic.Review.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).verifications, (Reasoning.Clinic.Review.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).frontier))))) = (6, (21, (6, (0, (0, 0)))))) := by
  rfl

public theorem grade_ledger : (((Grade.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).iterations, ((Grade.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).attempts, ((Grade.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).firings, ((Grade.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).expansions, ((Grade.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).verifications, (Grade.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).frontier))))) = (10, (60, (10, (0, (0, 0)))))) := by
  rfl

public theorem grade_well_ledger : (((Grade.account (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).iterations, ((Grade.account (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).attempts, ((Grade.account (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).firings, ((Grade.account (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).expansions, ((Grade.account (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).verifications, (Grade.account (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).frontier))))) = (0, (8, (0, (0, (0, 0)))))) := by
  rfl

public theorem review_rejected_ledger : (((Reasoning.Clinic.Review.account (({ temperature := 390, heartRate := 100, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).iterations, ((Reasoning.Clinic.Review.account (({ temperature := 390, heartRate := 100, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).attempts, ((Reasoning.Clinic.Review.account (({ temperature := 390, heartRate := 100, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).firings, ((Reasoning.Clinic.Review.account (({ temperature := 390, heartRate := 100, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).expansions, ((Reasoning.Clinic.Review.account (({ temperature := 390, heartRate := 100, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).verifications, (Reasoning.Clinic.Review.account (({ temperature := 390, heartRate := 100, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).frontier))))) = (4, (24, (4, (0, (1, 0)))))) := by
  rfl

public theorem spend_ledger : (((Reasoning.Budget.Spend.account (Nat) ((2, 2))).iterations, ((Reasoning.Budget.Spend.account (Nat) ((2, 2))).attempts, ((Reasoning.Budget.Spend.account (Nat) ((2, 2))).firings, ((Reasoning.Budget.Spend.account (Nat) ((2, 2))).expansions, ((Reasoning.Budget.Spend.account (Nat) ((2, 2))).verifications, (Reasoning.Budget.Spend.account (Nat) ((2, 2))).frontier))))) = (4, (8, (4, (0, (1, 0)))))) := by
  rfl

public theorem refund_ledger : (((Reasoning.Budget.Refund.account (2)).iterations, ((Reasoning.Budget.Refund.account (2)).attempts, ((Reasoning.Budget.Refund.account (2)).firings, ((Reasoning.Budget.Refund.account (2)).expansions, ((Reasoning.Budget.Refund.account (2)).verifications, (Reasoning.Budget.Refund.account (2)).frontier))))) = (4, (8, (4, (0, (1, 0)))))) := by
  rfl

public theorem refund_unsolved_ledger : (((Reasoning.Budget.Refund.account (7)).iterations, ((Reasoning.Budget.Refund.account (7)).attempts, ((Reasoning.Budget.Refund.account (7)).firings, ((Reasoning.Budget.Refund.account (7)).expansions, ((Reasoning.Budget.Refund.account (7)).verifications, (Reasoning.Budget.Refund.account (7)).frontier))))) = (9, (13, (9, (0, (1, 0)))))) := by
  rfl

public theorem plan_ledger : (((((Reasoning.Planner.Plan.run (2)).1).ledger).iterations, ((((Reasoning.Planner.Plan.run (2)).1).ledger).attempts, ((((Reasoning.Planner.Plan.run (2)).1).ledger).firings, ((((Reasoning.Planner.Plan.run (2)).1).ledger).expansions, ((((Reasoning.Planner.Plan.run (2)).1).ledger).verifications, (((Reasoning.Planner.Plan.run (2)).1).ledger).frontier))))) = (4, (15, (8, (3, (4, 4)))))) := by
  rfl

public theorem plan_unsolved_ledger : (((((Reasoning.Planner.Plan.run (5)).1).ledger).iterations, ((((Reasoning.Planner.Plan.run (5)).1).ledger).attempts, ((((Reasoning.Planner.Plan.run (5)).1).ledger).firings, ((((Reasoning.Planner.Plan.run (5)).1).ledger).expansions, ((((Reasoning.Planner.Plan.run (5)).1).ledger).verifications, (((Reasoning.Planner.Plan.run (5)).1).ledger).frontier))))) = (20, (100, (51, (20, (20, 10)))))) := by
  rfl

public theorem screen_ledger : (((((Reasoning.Screening.Screen.run (60)).1).ledger).iterations, ((((Reasoning.Screening.Screen.run (60)).1).ledger).attempts, ((((Reasoning.Screening.Screen.run (60)).1).ledger).firings, ((((Reasoning.Screening.Screen.run (60)).1).ledger).expansions, ((((Reasoning.Screening.Screen.run (60)).1).ledger).verifications, (((Reasoning.Screening.Screen.run (60)).1).ledger).frontier))))) = (4, (12, (4, (3, (4, 3)))))) := by
  rfl

public theorem screen_unsolved_ledger : (((((Reasoning.Screening.Screen.run (0)).1).ledger).iterations, ((((Reasoning.Screening.Screen.run (0)).1).ledger).attempts, ((((Reasoning.Screening.Screen.run (0)).1).ledger).firings, ((((Reasoning.Screening.Screen.run (0)).1).ledger).expansions, ((((Reasoning.Screening.Screen.run (0)).1).ledger).verifications, (((Reasoning.Screening.Screen.run (0)).1).ledger).frontier))))) = (1, (4, (0, (1, (1, 1)))))) := by
  rfl

public theorem screen_truncated_ledger : (((((Reasoning.Screening.Screen.run (100)).1).ledger).iterations, ((((Reasoning.Screening.Screen.run (100)).1).ledger).attempts, ((((Reasoning.Screening.Screen.run (100)).1).ledger).firings, ((((Reasoning.Screening.Screen.run (100)).1).ledger).expansions, ((((Reasoning.Screening.Screen.run (100)).1).ledger).verifications, (((Reasoning.Screening.Screen.run (100)).1).ledger).frontier))))) = (4, (16, (4, (4, (4, 3)))))) := by
  rfl

public theorem dose_ledger : ((((Reasoning.Screening.Dose.run (60)).ledger).iterations, (((Reasoning.Screening.Dose.run (60)).ledger).attempts, (((Reasoning.Screening.Dose.run (60)).ledger).firings, (((Reasoning.Screening.Dose.run (60)).ledger).expansions, (((Reasoning.Screening.Dose.run (60)).ledger).verifications, ((Reasoning.Screening.Dose.run (60)).ledger).frontier))))) = (3, (3, (0, (0, (3, 0)))))) := by
  rfl

public theorem dose_exhausted_ledger : ((((Reasoning.Screening.Dose.run (120)).ledger).iterations, (((Reasoning.Screening.Dose.run (120)).ledger).attempts, (((Reasoning.Screening.Dose.run (120)).ledger).firings, (((Reasoning.Screening.Dose.run (120)).ledger).expansions, (((Reasoning.Screening.Dose.run (120)).ledger).verifications, ((Reasoning.Screening.Dose.run (120)).ledger).frontier))))) = (3, (3, (0, (0, (3, 0)))))) := by
  rfl

end Reasoning.Main
