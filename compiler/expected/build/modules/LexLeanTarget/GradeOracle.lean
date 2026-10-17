module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanRuntime.Reasoning
public import LexLeanRuntime.Laws
public import LexLeanTarget.ReasoningOracle
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanTarget.GradeOracle

@[expose, reducible] public def Graded (v : LexLeanTarget.ReasoningOracle.Vitals) (r : Nat) : Prop := ((r <= 3) /\ ((r = 3) -> LexLeanTarget.ReasoningOracle.Hypotensive (v)))

@[expose] public def gradedCheck (v : LexLeanTarget.ReasoningOracle.Vitals) (r : Nat) : Bool := ((Nat.ble (r) (3)) && ((!(Nat.beq (r) (3))) || LexLeanTarget.ReasoningOracle.hypotensive (v)))

public theorem graded_sound (v : LexLeanTarget.ReasoningOracle.Vitals) (r : Nat) : ((gradedCheck (v) (r) = true) -> Graded (v) (r)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Graded, LexLeanTarget.ReasoningOracle.Hypotensive, LexLeanTarget.ReasoningOracle.hypotensive, gradedCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem Scale.sound : (LexLeanReasoning.Sound ((LexLeanTarget.GradeOracle.gradedCheck)) ((LexLeanTarget.GradeOracle.Graded))) :=
  LexLeanTarget.GradeOracle.graded_sound

public theorem grade_admitted (v : LexLeanTarget.ReasoningOracle.Vitals) : LexLeanTarget.ReasoningOracle.Consistent (({ vitals := v, fever := 0, tachycardia := 0, tachypnea := 0, leukocytosis := 0, sirs := 0, sepsis := 0, shock := 0, level := 0 } : LexLeanTarget.ReasoningOracle.Chart)) := by
  exact LexLeanTarget.ReasoningOracle.admitted_consistent (v)

public theorem grade_pending (v : LexLeanTarget.ReasoningOracle.Vitals) : (LexLeanTarget.ReasoningOracle.Pending (({ vitals := v, fever := 0, tachycardia := 0, tachypnea := 0, leukocytosis := 0, sirs := 0, sepsis := 0, shock := 0, level := 0 } : LexLeanTarget.ReasoningOracle.Chart)) < 11) := by
  exact LexLeanTarget.ReasoningOracle.admitted_pending (v)

@[expose, reducible] public def Related (v : LexLeanTarget.ReasoningOracle.Vitals) (c : LexLeanTarget.ReasoningOracle.Chart) : Prop := (LexLeanTarget.ReasoningOracle.Consistent (c) /\ (LexLeanTarget.ReasoningOracle.Warranted (c) /\ ((((c).shock = 1) -> LexLeanTarget.ReasoningOracle.Hypotensive ((c).vitals)) /\ (((c).vitals).systolic = (v).systolic))))

public theorem related_admitted (v : LexLeanTarget.ReasoningOracle.Vitals) : Related (v) (({ vitals := v, fever := 0, tachycardia := 0, tachypnea := 0, leukocytosis := 0, sirs := 0, sepsis := 0, shock := 0, level := 0 } : LexLeanTarget.ReasoningOracle.Chart)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [LexLeanTarget.ReasoningOracle.Consistent, LexLeanTarget.ReasoningOracle.Hypotensive, LexLeanTarget.ReasoningOracle.Warranted, Related, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem related_preserved (v : LexLeanTarget.ReasoningOracle.Vitals) (s : LexLeanTarget.ReasoningOracle.Chart) (t : LexLeanTarget.ReasoningOracle.Chart) : (Related (v) (s) -> (LexLeanTarget.ReasoningOracle.Justified (s) (t) -> Related (v) (t))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [LexLeanTarget.ReasoningOracle.Consistent, LexLeanTarget.ReasoningOracle.Febrile, LexLeanTarget.ReasoningOracle.Hypotensive, LexLeanTarget.ReasoningOracle.Justified, LexLeanTarget.ReasoningOracle.Leukocytic, LexLeanTarget.ReasoningOracle.Tachycardic, LexLeanTarget.ReasoningOracle.Tachypneic, LexLeanTarget.ReasoningOracle.Warranted, Related, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem grade_correct (v : LexLeanTarget.ReasoningOracle.Vitals) (chart : LexLeanTarget.ReasoningOracle.Chart) (r : Nat) : (Related (v) (chart) -> ((Option.some ((chart).level) = Option.some (r)) -> Graded (v) (r))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Graded, LexLeanTarget.ReasoningOracle.Consistent, LexLeanTarget.ReasoningOracle.Hypotensive, LexLeanTarget.ReasoningOracle.Warranted, Related, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
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
@[expose] public def Grade.observe (v : LexLeanTarget.ReasoningOracle.Vitals) : LexLeanTarget.ReasoningOracle.Chart := ({ vitals := v, fever := 0, tachycardia := 0, tachypnea := 0, leukocytosis := 0, sirs := 0, sepsis := 0, shock := 0, level := 0 } : LexLeanTarget.ReasoningOracle.Chart)
@[expose] public def Grade.fire (__s : LexLeanTarget.ReasoningOracle.Chart) (__step : LexLeanTarget.GradeOracle.Grade.Step) : Option (LexLeanTarget.ReasoningOracle.Chart) := (match __step with | LexLeanTarget.GradeOracle.Grade.Step.Fever => LexLeanTarget.ReasoningOracle.Fever.apply (__s) | LexLeanTarget.GradeOracle.Grade.Step.Tachycardia => LexLeanTarget.ReasoningOracle.Tachycardia.apply (__s) | LexLeanTarget.GradeOracle.Grade.Step.Tachypnea => LexLeanTarget.ReasoningOracle.Tachypnea.apply (__s) | LexLeanTarget.GradeOracle.Grade.Step.Leukocytosis => LexLeanTarget.ReasoningOracle.Leukocytosis.apply (__s) | LexLeanTarget.GradeOracle.Grade.Step.Sirs => LexLeanTarget.ReasoningOracle.Sirs.apply (__s) | LexLeanTarget.GradeOracle.Grade.Step.Sepsis => LexLeanTarget.ReasoningOracle.Sepsis.apply (__s) | LexLeanTarget.GradeOracle.Grade.Step.Shock => LexLeanTarget.ReasoningOracle.Shock.apply (__s) | LexLeanTarget.GradeOracle.Grade.Step.Escalate => LexLeanTarget.ReasoningOracle.Escalate.apply (__s))
@[expose] public def Grade.replay (__acc : Except ((Prod Bool Bool)) (LexLeanTarget.ReasoningOracle.Chart)) (__step : LexLeanTarget.GradeOracle.Grade.Step) : Except ((Prod Bool Bool)) (LexLeanTarget.ReasoningOracle.Chart) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match LexLeanTarget.GradeOracle.Grade.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def Grade.follow (v : LexLeanTarget.ReasoningOracle.Vitals) (__trace : List (LexLeanTarget.GradeOracle.Grade.Step)) : Except ((Prod Bool Bool)) (LexLeanTarget.ReasoningOracle.Chart) := (LexLeanCollections.listFold ((LexLeanTarget.GradeOracle.Grade.replay)) (Except.ok (LexLeanTarget.GradeOracle.Grade.observe (v))) (__trace) : Except ((Prod Bool Bool)) (LexLeanTarget.ReasoningOracle.Chart))
@[expose] public def Grade.extract (chart : LexLeanTarget.ReasoningOracle.Chart) : Option (Nat) := Option.some ((chart).level)
@[expose] public def Grade.accept (_v : LexLeanTarget.ReasoningOracle.Vitals) (__s : LexLeanTarget.ReasoningOracle.Chart) : Option (Nat) := LexLeanTarget.GradeOracle.Grade.extract (__s)
@[expose] public def Grade.answer (v : LexLeanTarget.ReasoningOracle.Vitals) (__trace : List (LexLeanTarget.GradeOracle.Grade.Step)) : Option (Nat) := (match LexLeanTarget.GradeOracle.Grade.follow (v) (__trace) with | Except.ok __s => LexLeanTarget.GradeOracle.Grade.accept (v) (__s) | Except.error _ => Option.none)
@[expose] public def Grade.select (__s : LexLeanTarget.ReasoningOracle.Chart) : Option (LexLeanTarget.GradeOracle.Grade.Step) := (match (match (match (if LexLeanTarget.ReasoningOracle.Fever.guard (__s) then Option.some (LexLeanTarget.GradeOracle.Grade.Step.Fever) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if LexLeanTarget.ReasoningOracle.Tachycardia.guard (__s) then Option.some (LexLeanTarget.GradeOracle.Grade.Step.Tachycardia) else Option.none)) with | Option.some __found => Option.some (__found) | Option.none => (match (if LexLeanTarget.ReasoningOracle.Tachypnea.guard (__s) then Option.some (LexLeanTarget.GradeOracle.Grade.Step.Tachypnea) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if LexLeanTarget.ReasoningOracle.Leukocytosis.guard (__s) then Option.some (LexLeanTarget.GradeOracle.Grade.Step.Leukocytosis) else Option.none))) with | Option.some __found => Option.some (__found) | Option.none => (match (match (if LexLeanTarget.ReasoningOracle.Sirs.guard (__s) then Option.some (LexLeanTarget.GradeOracle.Grade.Step.Sirs) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if LexLeanTarget.ReasoningOracle.Sepsis.guard (__s) then Option.some (LexLeanTarget.GradeOracle.Grade.Step.Sepsis) else Option.none)) with | Option.some __found => Option.some (__found) | Option.none => (match (if LexLeanTarget.ReasoningOracle.Shock.guard (__s) then Option.some (LexLeanTarget.GradeOracle.Grade.Step.Shock) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if LexLeanTarget.ReasoningOracle.Escalate.guard (__s) then Option.some (LexLeanTarget.GradeOracle.Grade.Step.Escalate) else Option.none))))
@[expose] public def Grade.attempts (__s : LexLeanTarget.ReasoningOracle.Chart) : Nat := ((let __scan6l : (Prod (Bool) (Nat)) := (let __scan2l : (Prod (Bool) (Nat)) := (let __scan0l : (Prod (Bool) (Nat)) := (LexLeanTarget.ReasoningOracle.Fever.guard (__s), 1); (if (__scan0l).1 then __scan0l else (let __scan0r : (Prod (Bool) (Nat)) := (LexLeanTarget.ReasoningOracle.Tachycardia.guard (__s), 1); ((__scan0r).1, ((__scan0l).2 + (__scan0r).2))))); (if (__scan2l).1 then __scan2l else (let __scan2r : (Prod (Bool) (Nat)) := (let __scan1l : (Prod (Bool) (Nat)) := (LexLeanTarget.ReasoningOracle.Tachypnea.guard (__s), 1); (if (__scan1l).1 then __scan1l else (let __scan1r : (Prod (Bool) (Nat)) := (LexLeanTarget.ReasoningOracle.Leukocytosis.guard (__s), 1); ((__scan1r).1, ((__scan1l).2 + (__scan1r).2))))); ((__scan2r).1, ((__scan2l).2 + (__scan2r).2))))); (if (__scan6l).1 then __scan6l else (let __scan6r : (Prod (Bool) (Nat)) := (let __scan5l : (Prod (Bool) (Nat)) := (let __scan3l : (Prod (Bool) (Nat)) := (LexLeanTarget.ReasoningOracle.Sirs.guard (__s), 1); (if (__scan3l).1 then __scan3l else (let __scan3r : (Prod (Bool) (Nat)) := (LexLeanTarget.ReasoningOracle.Sepsis.guard (__s), 1); ((__scan3r).1, ((__scan3l).2 + (__scan3r).2))))); (if (__scan5l).1 then __scan5l else (let __scan5r : (Prod (Bool) (Nat)) := (let __scan4l : (Prod (Bool) (Nat)) := (LexLeanTarget.ReasoningOracle.Shock.guard (__s), 1); (if (__scan4l).1 then __scan4l else (let __scan4r : (Prod (Bool) (Nat)) := (LexLeanTarget.ReasoningOracle.Escalate.guard (__s), 1); ((__scan4r).1, ((__scan4l).2 + (__scan4r).2))))); ((__scan5r).1, ((__scan5l).2 + (__scan5r).2))))); ((__scan6r).1, ((__scan6l).2 + (__scan6r).2)))))).2
@[expose] public def Grade.next (__s : LexLeanTarget.ReasoningOracle.Chart) : Option (LexLeanTarget.ReasoningOracle.Chart) := (match LexLeanTarget.GradeOracle.Grade.select (__s) with | Option.none => Option.none | Option.some __step => LexLeanTarget.GradeOracle.Grade.fire (__s) (__step))
@[expose] public def Grade.saturate (v : LexLeanTarget.ReasoningOracle.Vitals) : (Prod (LexLeanTarget.ReasoningOracle.Chart) (Bool)) := (LexLeanCollections.iterateUntil ((LexLeanTarget.GradeOracle.Grade.next)) (11) (LexLeanTarget.GradeOracle.Grade.observe (v)) : (Prod (LexLeanTarget.ReasoningOracle.Chart) (Bool)))
public structure Grade.Run where
  state : LexLeanTarget.ReasoningOracle.Chart
  trace : List (LexLeanTarget.GradeOracle.Grade.Step)
  ledger : LexLeanTarget.GradeOracle.Grade.Ledger
@[expose] public def Grade.start (v : LexLeanTarget.ReasoningOracle.Vitals) : LexLeanTarget.GradeOracle.Grade.Run := ({ state := LexLeanTarget.GradeOracle.Grade.observe (v), trace := ([] : List (LexLeanTarget.GradeOracle.Grade.Step)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := 0 } : LexLeanTarget.GradeOracle.Grade.Ledger) } : LexLeanTarget.GradeOracle.Grade.Run)
@[expose] public def Grade.step (__r : LexLeanTarget.GradeOracle.Grade.Run) : Option (LexLeanTarget.GradeOracle.Grade.Run) := (match LexLeanTarget.GradeOracle.Grade.select ((__r).state) with | Option.none => Option.none | Option.some __step => (match LexLeanTarget.GradeOracle.Grade.fire ((__r).state) (__step) with | Option.none => Option.none | Option.some __t => Option.some (({ state := __t, trace := (LexLeanRuntime.append ((__r).trace) ((__step :: ([] : List (LexLeanTarget.GradeOracle.Grade.Step)))) : List (LexLeanTarget.GradeOracle.Grade.Step)), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + LexLeanTarget.GradeOracle.Grade.attempts ((__r).state)), firings := (((__r).ledger).firings + 1), expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : LexLeanTarget.GradeOracle.Grade.Ledger) } : LexLeanTarget.GradeOracle.Grade.Run))))
@[expose] public def Grade.run (v : LexLeanTarget.ReasoningOracle.Vitals) : (Prod (LexLeanTarget.GradeOracle.Grade.Run) (Bool)) := (LexLeanCollections.iterateUntil ((LexLeanTarget.GradeOracle.Grade.step)) (11) (LexLeanTarget.GradeOracle.Grade.start (v)) : (Prod (LexLeanTarget.GradeOracle.Grade.Run) (Bool)))
@[expose] public def Grade.account (v : LexLeanTarget.ReasoningOracle.Vitals) : LexLeanTarget.GradeOracle.Grade.Ledger := (let __final : (Prod (LexLeanTarget.GradeOracle.Grade.Run) (Bool)) := LexLeanTarget.GradeOracle.Grade.run (v); (match (__final).2 with | Bool.true => ({ iterations := (((__final).1).ledger).iterations, attempts := ((((__final).1).ledger).attempts + LexLeanTarget.GradeOracle.Grade.attempts (((__final).1).state)), firings := (((__final).1).ledger).firings, expansions := (((__final).1).ledger).expansions, verifications := (((__final).1).ledger).verifications, frontier := (((__final).1).ledger).frontier } : LexLeanTarget.GradeOracle.Grade.Ledger) | Bool.false => ((__final).1).ledger))
@[expose] public def Grade.conclude (v : LexLeanTarget.ReasoningOracle.Vitals) (__s : LexLeanTarget.ReasoningOracle.Chart) : Except ((Prod Bool Bool)) (Nat) := (match LexLeanTarget.GradeOracle.Grade.accept (v) (__s) with | Option.some __v => Except.ok (__v) | Option.none => (match LexLeanTarget.GradeOracle.Grade.extract (__s) with | Option.some _ => Except.error (((true, false) : Prod Bool Bool)) | Option.none => Except.error (((false, true) : Prod Bool Bool))))
@[expose] public def Grade.verdict (v : LexLeanTarget.ReasoningOracle.Vitals) : Except ((Prod Bool Bool)) (Nat) := (let __final : (Prod (LexLeanTarget.ReasoningOracle.Chart) (Bool)) := LexLeanTarget.GradeOracle.Grade.saturate (v); (match (__final).2 with | Bool.true => LexLeanTarget.GradeOracle.Grade.conclude (v) ((__final).1) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
@[expose] public def Grade (v : LexLeanTarget.ReasoningOracle.Vitals) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (LexLeanTarget.GradeOracle.Grade.Step)))) := (let __final : (Prod (LexLeanTarget.GradeOracle.Grade.Run) (Bool)) := LexLeanTarget.GradeOracle.Grade.run (v); (match (__final).2 with | Bool.true => (match LexLeanTarget.GradeOracle.Grade.conclude (v) (((__final).1).state) with | Except.ok __v => Except.ok ((__v, ((__final).1).trace)) | Except.error __e => Except.error (__e)) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
public theorem Grade.fire_sound (__s : LexLeanTarget.ReasoningOracle.Chart) (__step : LexLeanTarget.GradeOracle.Grade.Step) (__t : LexLeanTarget.ReasoningOracle.Chart) : ((LexLeanTarget.GradeOracle.Grade.fire (__s) (__step) = Option.some (__t)) -> LexLeanTarget.ReasoningOracle.Justified (__s) (__t)) :=
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
public theorem Grade.fire_progress (__s : LexLeanTarget.ReasoningOracle.Chart) (__step : LexLeanTarget.GradeOracle.Grade.Step) (__t : LexLeanTarget.ReasoningOracle.Chart) : (LexLeanTarget.ReasoningOracle.Consistent (__s) -> ((LexLeanTarget.GradeOracle.Grade.fire (__s) (__step) = Option.some (__t)) -> (LexLeanTarget.ReasoningOracle.Pending (__t) < LexLeanTarget.ReasoningOracle.Pending (__s)))) :=
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
public theorem Grade.replay_fire (__s : LexLeanTarget.ReasoningOracle.Chart) (__step : LexLeanTarget.GradeOracle.Grade.Step) (__t : LexLeanTarget.ReasoningOracle.Chart) : ((LexLeanTarget.GradeOracle.Grade.fire (__s) (__step) = Option.some (__t)) -> (LexLeanTarget.GradeOracle.Grade.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [LexLeanTarget.GradeOracle.Grade.replay]
  rw [llE]
public theorem Grade.replay_sound (v : LexLeanTarget.ReasoningOracle.Vitals) (__acc : Except ((Prod Bool Bool)) (LexLeanTarget.ReasoningOracle.Chart)) (__step : LexLeanTarget.GradeOracle.Grade.Step) : ((LexLeanReasoning.Reaches ((LexLeanTarget.ReasoningOracle.Justified)) (LexLeanTarget.GradeOracle.Grade.observe (v)) (__acc)) -> (LexLeanReasoning.Reaches ((LexLeanTarget.ReasoningOracle.Justified)) (LexLeanTarget.GradeOracle.Grade.observe (v)) (LexLeanTarget.GradeOracle.Grade.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [LexLeanTarget.GradeOracle.Grade.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (LexLeanTarget.GradeOracle.Grade.fire_sound llS __step _ ‹_›)
public theorem Grade.derivation (v : LexLeanTarget.ReasoningOracle.Vitals) (__trace : List (LexLeanTarget.GradeOracle.Grade.Step)) : (LexLeanReasoning.Reaches ((LexLeanTarget.ReasoningOracle.Justified)) (LexLeanTarget.GradeOracle.Grade.observe (v)) (LexLeanTarget.GradeOracle.Grade.follow (v) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((LexLeanTarget.GradeOracle.Grade.replay)) (fun (__acc : Except ((Prod Bool Bool)) (LexLeanTarget.ReasoningOracle.Chart)) => (LexLeanReasoning.Reaches ((LexLeanTarget.ReasoningOracle.Justified)) (LexLeanTarget.GradeOracle.Grade.observe (v)) (__acc))) (LexLeanTarget.GradeOracle.Grade.replay_sound (v)) (__trace) (Except.ok (LexLeanTarget.GradeOracle.Grade.observe (v))) (LexLeanReasoning.reachesStart ((LexLeanTarget.ReasoningOracle.Justified)) (LexLeanTarget.GradeOracle.Grade.observe (v))))
public theorem Grade.follow_invariant (v : LexLeanTarget.ReasoningOracle.Vitals) (__trace : List (LexLeanTarget.GradeOracle.Grade.Step)) (__s : LexLeanTarget.ReasoningOracle.Chart) : ((LexLeanTarget.GradeOracle.Grade.follow (v) (__trace) = Except.ok (__s)) -> LexLeanTarget.ReasoningOracle.Consistent (__s)) :=
  (fun llE => (LexLeanReasoning.starPreserves ((LexLeanTarget.ReasoningOracle.Justified)) ((LexLeanTarget.ReasoningOracle.Consistent)) LexLeanTarget.ReasoningOracle.Findings.preserves (LexLeanTarget.GradeOracle.Grade.observe (v)) (__s) (LexLeanTarget.GradeOracle.Grade.derivation (v) (__trace) (__s) llE) (LexLeanTarget.GradeOracle.grade_admitted (v))))
public theorem Grade.follow_relation (v : LexLeanTarget.ReasoningOracle.Vitals) (__trace : List (LexLeanTarget.GradeOracle.Grade.Step)) (__s : LexLeanTarget.ReasoningOracle.Chart) : ((LexLeanTarget.GradeOracle.Grade.follow (v) (__trace) = Except.ok (__s)) -> LexLeanTarget.GradeOracle.Related (v) (__s)) :=
  (fun llE => (LexLeanReasoning.starPreserves ((LexLeanTarget.ReasoningOracle.Justified)) (fun (__s : LexLeanTarget.ReasoningOracle.Chart) => LexLeanTarget.GradeOracle.Related (v) (__s)) (LexLeanTarget.GradeOracle.related_preserved (v)) (LexLeanTarget.GradeOracle.Grade.observe (v)) (__s) (LexLeanTarget.GradeOracle.Grade.derivation (v) (__trace) (__s) llE) (LexLeanTarget.GradeOracle.related_admitted (v))))
public theorem Grade.accept_sound (v : LexLeanTarget.ReasoningOracle.Vitals) (__s : LexLeanTarget.ReasoningOracle.Chart) (__v : Nat) : (LexLeanTarget.GradeOracle.Related (v) (__s) -> ((LexLeanTarget.GradeOracle.Grade.accept (v) (__s) = Option.some (__v)) -> LexLeanTarget.GradeOracle.Graded (v) (__v))) :=
  (fun llJ llE => (LexLeanTarget.GradeOracle.grade_correct (v) (__s) (__v) llJ llE))
public theorem Grade.next_sound (__s : LexLeanTarget.ReasoningOracle.Chart) (__t : LexLeanTarget.ReasoningOracle.Chart) : ((LexLeanTarget.GradeOracle.Grade.next (__s) = Option.some (__t)) -> LexLeanTarget.ReasoningOracle.Justified (__s) (__t)) :=
by
  intro llE
  dsimp only [LexLeanTarget.GradeOracle.Grade.next] at llE
  split at llE
  · cases llE
  · exact LexLeanTarget.GradeOracle.Grade.fire_sound __s _ __t llE
public theorem Grade.next_preserves (__s : LexLeanTarget.ReasoningOracle.Chart) (__t : LexLeanTarget.ReasoningOracle.Chart) : ((LexLeanTarget.GradeOracle.Grade.next (__s) = Option.some (__t)) -> (LexLeanTarget.ReasoningOracle.Consistent (__s) -> LexLeanTarget.ReasoningOracle.Consistent (__t))) :=
  (fun llE llH => (LexLeanTarget.ReasoningOracle.consistent_preserved (__s) (__t) llH (LexLeanTarget.GradeOracle.Grade.next_sound (__s) (__t) llE)))
public theorem Grade.next_progress (__s : LexLeanTarget.ReasoningOracle.Chart) (__t : LexLeanTarget.ReasoningOracle.Chart) : ((LexLeanTarget.GradeOracle.Grade.next (__s) = Option.some (__t)) -> (LexLeanTarget.ReasoningOracle.Consistent (__s) -> (LexLeanTarget.ReasoningOracle.Pending (__t) < LexLeanTarget.ReasoningOracle.Pending (__s)))) :=
by
  intro llE
  dsimp only [LexLeanTarget.GradeOracle.Grade.next] at llE
  split at llE
  · cases llE
  · exact fun llJ => LexLeanTarget.GradeOracle.Grade.fire_progress __s _ __t llJ llE
public theorem Grade.step_none (__r : LexLeanTarget.GradeOracle.Grade.Run) : ((LexLeanTarget.GradeOracle.Grade.step (__r) = Option.none) -> (LexLeanTarget.GradeOracle.Grade.next ((__r).state) = Option.none)) :=
by
  intro llE
  dsimp only [LexLeanTarget.GradeOracle.Grade.step] at llE
  split at llE
  · rename_i llH
    dsimp only [LexLeanTarget.GradeOracle.Grade.next]
    rw [llH]
  · rename_i llH
    split at llE
    · dsimp only [LexLeanTarget.GradeOracle.Grade.next]
      rw [llH]
      assumption
    · cases llE
public theorem Grade.step_some (__r : LexLeanTarget.GradeOracle.Grade.Run) (__q : LexLeanTarget.GradeOracle.Grade.Run) : ((LexLeanTarget.GradeOracle.Grade.step (__r) = Option.some (__q)) -> (LexLeanTarget.GradeOracle.Grade.next ((__r).state) = Option.some ((__q).state))) :=
by
  intro llE
  dsimp only [LexLeanTarget.GradeOracle.Grade.step] at llE
  split at llE
  · cases llE
  · rename_i llH
    split at llE
    · cases llE
    · cases llE
      dsimp only [LexLeanTarget.GradeOracle.Grade.next]
      rw [llH]
      assumption
public theorem Grade.step_trace (v : LexLeanTarget.ReasoningOracle.Vitals) (__r : LexLeanTarget.GradeOracle.Grade.Run) (__q : LexLeanTarget.GradeOracle.Grade.Run) : ((LexLeanTarget.GradeOracle.Grade.step (__r) = Option.some (__q)) -> ((LexLeanTarget.GradeOracle.Grade.follow (v) ((__r).trace) = Except.ok ((__r).state)) -> (LexLeanTarget.GradeOracle.Grade.follow (v) ((__q).trace) = Except.ok ((__q).state)))) :=
by
  intro llE llH
  dsimp only [LexLeanTarget.GradeOracle.Grade.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      dsimp only [LexLeanTarget.GradeOracle.Grade.follow]
      rw [LexLeanReasoning.foldSnoc]
      dsimp only [LexLeanTarget.GradeOracle.Grade.follow] at llH
      rw [llH]
      apply LexLeanTarget.GradeOracle.Grade.replay_fire
      assumption
public theorem Grade.step_count (__r : LexLeanTarget.GradeOracle.Grade.Run) (__q : LexLeanTarget.GradeOracle.Grade.Run) : ((LexLeanTarget.GradeOracle.Grade.step (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [LexLeanTarget.GradeOracle.Grade.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Grade.step_fired (__r : LexLeanTarget.GradeOracle.Grade.Run) (__q : LexLeanTarget.GradeOracle.Grade.Run) : ((LexLeanTarget.GradeOracle.Grade.step (__r) = Option.some (__q)) -> (((__q).ledger).firings = (((__r).ledger).firings + 1))) :=
by
  intro llE
  dsimp only [LexLeanTarget.GradeOracle.Grade.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Grade.saturate_derivation (v : LexLeanTarget.ReasoningOracle.Vitals) : (LexLeanReasoning.Star ((LexLeanTarget.ReasoningOracle.Justified)) (LexLeanTarget.GradeOracle.Grade.observe (v)) ((LexLeanTarget.GradeOracle.Grade.saturate (v)).1)) :=
by
  dsimp only [LexLeanTarget.GradeOracle.Grade.saturate]
  exact (LexLeanReasoning.iterateUntilInvariant ((LexLeanTarget.GradeOracle.Grade.next)) (fun (__s : LexLeanTarget.ReasoningOracle.Chart) => (LexLeanReasoning.Star ((LexLeanTarget.ReasoningOracle.Justified)) (LexLeanTarget.GradeOracle.Grade.observe (v)) (__s))) (fun llA llB llE llH => (LexLeanReasoning.Star.tail _ llA llB llH (LexLeanTarget.GradeOracle.Grade.next_sound llA llB llE))) (11) (LexLeanTarget.GradeOracle.Grade.observe (v)) (LexLeanReasoning.Star.refl _))
public theorem Grade.run_state (v : LexLeanTarget.ReasoningOracle.Vitals) : ((((LexLeanTarget.GradeOracle.Grade.run (v)).1).state = (LexLeanTarget.GradeOracle.Grade.saturate (v)).1) /\ ((LexLeanTarget.GradeOracle.Grade.run (v)).2 = (LexLeanTarget.GradeOracle.Grade.saturate (v)).2)) :=
by
  dsimp only [LexLeanTarget.GradeOracle.Grade.run, LexLeanTarget.GradeOracle.Grade.saturate]
  exact (LexLeanReasoning.iterateUntilSimulate ((LexLeanTarget.GradeOracle.Grade.step)) ((LexLeanTarget.GradeOracle.Grade.next)) ((fun (__r : LexLeanTarget.GradeOracle.Grade.Run) => (__r).state)) LexLeanTarget.GradeOracle.Grade.step_none LexLeanTarget.GradeOracle.Grade.step_some (11) (LexLeanTarget.GradeOracle.Grade.start (v)))
public theorem Grade.run_trace (v : LexLeanTarget.ReasoningOracle.Vitals) : (LexLeanTarget.GradeOracle.Grade.follow (v) (((LexLeanTarget.GradeOracle.Grade.run (v)).1).trace) = Except.ok (((LexLeanTarget.GradeOracle.Grade.run (v)).1).state)) :=
by
  dsimp only [LexLeanTarget.GradeOracle.Grade.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((LexLeanTarget.GradeOracle.Grade.step)) (fun (__r : LexLeanTarget.GradeOracle.Grade.Run) => (LexLeanTarget.GradeOracle.Grade.follow (v) ((__r).trace) = Except.ok ((__r).state))) (LexLeanTarget.GradeOracle.Grade.step_trace (v)) (11) (LexLeanTarget.GradeOracle.Grade.start (v)) rfl)
public theorem Grade.iterations_bounded (v : LexLeanTarget.ReasoningOracle.Vitals) : ((((LexLeanTarget.GradeOracle.Grade.run (v)).1).ledger).iterations <= 11) :=
by
  dsimp only [LexLeanTarget.GradeOracle.Grade.run]
  exact (LexLeanReasoning.iterateUntilCount ((LexLeanTarget.GradeOracle.Grade.step)) ((fun (__r : LexLeanTarget.GradeOracle.Grade.Run) => ((__r).ledger).iterations)) LexLeanTarget.GradeOracle.Grade.step_count (11) (LexLeanTarget.GradeOracle.Grade.start (v)) rfl)
public theorem Grade.firings_bounded (v : LexLeanTarget.ReasoningOracle.Vitals) : ((((LexLeanTarget.GradeOracle.Grade.run (v)).1).ledger).firings <= 11) :=
by
  dsimp only [LexLeanTarget.GradeOracle.Grade.run]
  exact (LexLeanReasoning.iterateUntilCount ((LexLeanTarget.GradeOracle.Grade.step)) ((fun (__r : LexLeanTarget.GradeOracle.Grade.Run) => ((__r).ledger).firings)) LexLeanTarget.GradeOracle.Grade.step_fired (11) (LexLeanTarget.GradeOracle.Grade.start (v)) rfl)
public theorem Grade.saturate_invariant (v : LexLeanTarget.ReasoningOracle.Vitals) : LexLeanTarget.ReasoningOracle.Consistent ((LexLeanTarget.GradeOracle.Grade.saturate (v)).1) :=
by
  dsimp only [LexLeanTarget.GradeOracle.Grade.saturate]
  exact (LexLeanReasoning.iterateUntilInvariant ((LexLeanTarget.GradeOracle.Grade.next)) ((LexLeanTarget.ReasoningOracle.Consistent)) LexLeanTarget.GradeOracle.Grade.next_preserves (11) (LexLeanTarget.GradeOracle.Grade.observe (v)) (LexLeanTarget.GradeOracle.grade_admitted (v)))
public theorem Grade.next_preserves_relation (v : LexLeanTarget.ReasoningOracle.Vitals) (__s : LexLeanTarget.ReasoningOracle.Chart) (__t : LexLeanTarget.ReasoningOracle.Chart) : ((LexLeanTarget.GradeOracle.Grade.next (__s) = Option.some (__t)) -> (LexLeanTarget.GradeOracle.Related (v) (__s) -> LexLeanTarget.GradeOracle.Related (v) (__t))) :=
  (fun llE llH => (LexLeanTarget.GradeOracle.related_preserved (v) (__s) (__t) llH (LexLeanTarget.GradeOracle.Grade.next_sound (__s) (__t) llE)))
public theorem Grade.saturate_relation (v : LexLeanTarget.ReasoningOracle.Vitals) : LexLeanTarget.GradeOracle.Related (v) ((LexLeanTarget.GradeOracle.Grade.saturate (v)).1) :=
by
  dsimp only [LexLeanTarget.GradeOracle.Grade.saturate]
  exact (LexLeanReasoning.iterateUntilInvariant ((LexLeanTarget.GradeOracle.Grade.next)) (fun (__s : LexLeanTarget.ReasoningOracle.Chart) => LexLeanTarget.GradeOracle.Related (v) (__s)) (LexLeanTarget.GradeOracle.Grade.next_preserves_relation (v)) (11) (LexLeanTarget.GradeOracle.Grade.observe (v)) (LexLeanTarget.GradeOracle.related_admitted (v)))
public theorem Grade.run_relation (v : LexLeanTarget.ReasoningOracle.Vitals) : LexLeanTarget.GradeOracle.Related (v) (((LexLeanTarget.GradeOracle.Grade.run (v)).1).state) :=
by
  rw [And.left (LexLeanTarget.GradeOracle.Grade.run_state v)]
  exact LexLeanTarget.GradeOracle.Grade.saturate_relation v
public theorem Grade.saturates (v : LexLeanTarget.ReasoningOracle.Vitals) : ((LexLeanTarget.GradeOracle.Grade.saturate (v)).2 = true) :=
by
  dsimp only [LexLeanTarget.GradeOracle.Grade.saturate]
  exact (LexLeanReasoning.iterateUntilStops ((LexLeanTarget.GradeOracle.Grade.next)) ((LexLeanTarget.ReasoningOracle.Consistent)) ((LexLeanTarget.ReasoningOracle.Pending)) (fun llA llB llE llH => (And.intro (LexLeanTarget.GradeOracle.Grade.next_preserves llA llB llE llH) (LexLeanTarget.GradeOracle.Grade.next_progress llA llB llE llH))) (11) (LexLeanTarget.GradeOracle.Grade.observe (v)) (LexLeanTarget.GradeOracle.grade_admitted (v)) (LexLeanTarget.GradeOracle.grade_pending (v)))
public theorem Grade.conclude_accept (v : LexLeanTarget.ReasoningOracle.Vitals) (__s : LexLeanTarget.ReasoningOracle.Chart) (__v : Nat) : ((LexLeanTarget.GradeOracle.Grade.conclude (v) (__s) = Except.ok (__v)) -> (LexLeanTarget.GradeOracle.Grade.accept (v) (__s) = Option.some (__v))) :=
by
  intro llE
  dsimp only [LexLeanTarget.GradeOracle.Grade.conclude] at llE
  split at llE
  · cases llE
    assumption
  · split at llE
    · cases llE
    · cases llE
public theorem Grade.conclude_sound (v : LexLeanTarget.ReasoningOracle.Vitals) (__s : LexLeanTarget.ReasoningOracle.Chart) (__v : Nat) : (LexLeanTarget.GradeOracle.Related (v) (__s) -> ((LexLeanTarget.GradeOracle.Grade.conclude (v) (__s) = Except.ok (__v)) -> LexLeanTarget.GradeOracle.Graded (v) (__v))) :=
by
  intro llJ llE
  dsimp only [LexLeanTarget.GradeOracle.Grade.conclude] at llE
  split at llE
  · cases llE
    exact LexLeanTarget.GradeOracle.Grade.accept_sound _ _ _ llJ ‹_›
  · split at llE
    · cases llE
    · cases llE
public theorem Grade.verdict_sound (v : LexLeanTarget.ReasoningOracle.Vitals) : (forall (__v : Nat), ((LexLeanTarget.GradeOracle.Grade.verdict (v) = Except.ok (__v)) -> LexLeanTarget.GradeOracle.Graded (v) (__v))) :=
by
  intro llV llE
  have llJ := LexLeanTarget.GradeOracle.Grade.saturate_relation v
  dsimp only [LexLeanTarget.GradeOracle.Grade.verdict] at llE
  generalize llRun : LexLeanTarget.GradeOracle.Grade.saturate v = llR at llE llJ
  split at llE
  · exact LexLeanTarget.GradeOracle.Grade.conclude_sound _ _ _ llJ llE
  · cases llE
public theorem Grade.explained (v : LexLeanTarget.ReasoningOracle.Vitals) : (forall (__v : Nat), (forall (__trace : List (LexLeanTarget.GradeOracle.Grade.Step)), ((LexLeanTarget.GradeOracle.Grade (v) = Except.ok ((__v, __trace))) -> ((LexLeanTarget.GradeOracle.Grade.answer (v) (__trace) = Option.some (__v)) /\ LexLeanTarget.GradeOracle.Graded (v) (__v))))) :=
by
  intro llV llT llE
  have llTrace := LexLeanTarget.GradeOracle.Grade.run_trace v
  have llJ := LexLeanTarget.GradeOracle.Grade.run_relation v
  dsimp only [LexLeanTarget.GradeOracle.Grade] at llE
  generalize llRun : LexLeanTarget.GradeOracle.Grade.run v = llR at llE llTrace llJ
  split at llE
  · split at llE
    · rename_i llW llH
      cases llE
      exact And.intro (by dsimp only [LexLeanTarget.GradeOracle.Grade.answer]; rw [llTrace]; exact LexLeanTarget.GradeOracle.Grade.conclude_accept _ _ _ llH) (LexLeanTarget.GradeOracle.Grade.conclude_sound _ _ _ llJ llH)
    · cases llE
  · cases llE

end LexLeanTarget.GradeOracle
