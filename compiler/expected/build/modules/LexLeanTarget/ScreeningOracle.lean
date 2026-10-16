module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanRuntime.Models
public import LexLeanRuntime.Reasoning
public import LexLeanRuntime.Laws
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanTarget.ScreeningOracle

@[expose, reducible] public def Short (_w : Nat) (doses : List (Nat)) : Prop := ((LexLeanRuntime.length (doses) : Nat) <= 4)

@[expose] public def shortCheck (_w : Nat) (doses : List (Nat)) : Bool := (Nat.ble ((LexLeanRuntime.length (doses) : Nat)) (4))

public theorem short_sound (w : Nat) (doses : List (Nat)) : ((shortCheck (w) (doses) = true) -> Short (w) (doses)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Short, shortCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem DoseContract.postcondition_sound : LexLeanModels.Sound2 ((LexLeanTarget.ScreeningOracle.shortCheck)) ((LexLeanTarget.ScreeningOracle.Short)) := LexLeanTarget.ScreeningOracle.short_sound

@[expose] public def DoseCandidates (w : Nat) : List (Nat) := ((LexLeanRuntime.subtract (w) (10) : Nat) :: ((LexLeanRuntime.subtract (w) (20) : Nat) :: ((LexLeanRuntime.subtract (w) (30) : Nat) :: ((LexLeanRuntime.subtract (w) (50) : Nat) :: ([] : List (Nat))))))

@[expose] public def DoseModel (__input : Nat) : List (Nat) := LexLeanTarget.ScreeningOracle.DoseCandidates (__input)

@[expose, reducible] public def Proposed (s : (Prod (Nat) (Nat))) (t : (Prod (Nat) (Nat))) : Prop := (((s).2 = 0) /\ (((t).1 = (s).1) /\ (0 < (t).2)))


public theorem propose_sound (s : (Prod (Nat) (Nat))) (d : Nat) : ((((Nat.beq ((s).2) (0)) && (Nat.blt (0) (d))) = true) -> Proposed (s) (((s).1, d))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Proposed, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

@[expose] public def Propose.guard (s : (Prod (Nat) (Nat))) (d : Nat) : Bool := ((Nat.beq ((s).2) (0)) && (Nat.blt (0) (d)))
@[expose] public def Propose.conclusion (s : (Prod (Nat) (Nat))) (d : Nat) : (Prod (Nat) (Nat)) := ((s).1, d)
@[expose] public def Propose.candidates (s : (Prod (Nat) (Nat))) : List (Nat) := (match (let __checked1_input : Nat := (s).1; (let __checked1_output : List (Nat) := LexLeanTarget.ScreeningOracle.DoseModel (__checked1_input); (if LexLeanTarget.ScreeningOracle.shortCheck (__checked1_input) (__checked1_output) then Except.ok (__checked1_output) else Except.error (((true, false) : Prod Bool Bool))))) with | Except.ok proposed => proposed | Except.error _ => ([] : List (Nat)))
@[expose] public def Propose.apply (s : (Prod (Nat) (Nat))) (d : Nat) : Option ((Prod (Nat) (Nat))) := (if LexLeanTarget.ScreeningOracle.Propose.guard (s) (d) then Option.some (LexLeanTarget.ScreeningOracle.Propose.conclusion (s) (d)) else Option.none)
public theorem Propose.apply_sound (s : (Prod (Nat) (Nat))) (d : Nat) (__t : (Prod (Nat) (Nat))) : ((LexLeanTarget.ScreeningOracle.Propose.apply (s) (d) = Option.some (__t)) -> LexLeanTarget.ScreeningOracle.Proposed (s) (__t)) :=
  (LexLeanReasoning.guarded ((LexLeanTarget.ScreeningOracle.Proposed)) (s) (LexLeanTarget.ScreeningOracle.Propose.guard (s) (d)) (LexLeanTarget.ScreeningOracle.Propose.conclusion (s) (d)) (LexLeanTarget.ScreeningOracle.propose_sound (s) (d)) (__t))

@[expose, reducible] public def Safe (w : Nat) (r : Nat) : Prop := ((r + r) <= w)

@[expose] public def safeCheck (w : Nat) (r : Nat) : Bool := (Nat.ble ((r + r)) (w))

public theorem safe_sound (w : Nat) (r : Nat) : ((safeCheck (w) (r) = true) -> Safe (w) (r)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Safe, safeCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem SafeDose.sound : (LexLeanReasoning.Sound ((LexLeanTarget.ScreeningOracle.safeCheck)) ((LexLeanTarget.ScreeningOracle.Safe))) :=
  LexLeanTarget.ScreeningOracle.safe_sound

public inductive Dose.Step where
  | candidate (_ : Nat)
public structure Dose.Ledger where
  iterations : Nat
  attempts : Nat
  firings : Nat
  expansions : Nat
  verifications : Nat
  frontier : Nat
@[expose] public def Dose.candidates (w : Nat) : List (Nat) := (match (let __checked1_input : Nat := w; (let __checked1_output : List (Nat) := LexLeanTarget.ScreeningOracle.DoseModel (__checked1_input); (if LexLeanTarget.ScreeningOracle.shortCheck (__checked1_input) (__checked1_output) then Except.ok (__checked1_output) else Except.error (((true, false) : Prod Bool Bool))))) with | Except.ok proposed => proposed | Except.error _ => ([] : List (Nat)))
@[expose] public def Dose.verify (w : Nat) (__c : Nat) : Option (Nat) := (if LexLeanTarget.ScreeningOracle.safeCheck (w) (__c) then Option.some (__c) else Option.none)
public structure Dose.Trial where
  found : Option (Nat)
  truncated : Bool
  ledger : LexLeanTarget.ScreeningOracle.Dose.Ledger
@[expose] public def Dose.«try» (w : Nat) (__t : LexLeanTarget.ScreeningOracle.Dose.Trial) (__c : Nat) : LexLeanTarget.ScreeningOracle.Dose.Trial := (match (__t).found with | Option.some _ => __t | Option.none => (match (Nat.blt (((__t).ledger).verifications) (3)) with | Bool.true => ({ found := LexLeanTarget.ScreeningOracle.Dose.verify (w) (__c), truncated := (__t).truncated, ledger := ({ iterations := (((__t).ledger).iterations + 1), attempts := (((__t).ledger).attempts + 1), firings := ((__t).ledger).firings, expansions := ((__t).ledger).expansions, verifications := (((__t).ledger).verifications + 1), frontier := ((__t).ledger).frontier } : LexLeanTarget.ScreeningOracle.Dose.Ledger) } : LexLeanTarget.ScreeningOracle.Dose.Trial) | Bool.false => ({ found := Option.none, truncated := true, ledger := (__t).ledger } : LexLeanTarget.ScreeningOracle.Dose.Trial)))
@[expose] public def Dose.run (w : Nat) : LexLeanTarget.ScreeningOracle.Dose.Trial := (LexLeanCollections.listFold ((fun (__t : LexLeanTarget.ScreeningOracle.Dose.Trial) (__c : Nat) => LexLeanTarget.ScreeningOracle.Dose.«try» (w) (__t) (__c))) (({ found := Option.none, truncated := false, ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := 0 } : LexLeanTarget.ScreeningOracle.Dose.Ledger) } : LexLeanTarget.ScreeningOracle.Dose.Trial)) (LexLeanTarget.ScreeningOracle.Dose.candidates (w)) : LexLeanTarget.ScreeningOracle.Dose.Trial)
@[expose] public def Dose.failure (__t : LexLeanTarget.ScreeningOracle.Dose.Trial) : (Prod Bool Bool) := (match (__t).truncated with | Bool.true => ((false, false) : Prod Bool Bool) | Bool.false => (match (Nat.beq (((__t).ledger).verifications) (0)) with | Bool.true => ((false, true) : Prod Bool Bool) | Bool.false => ((true, false) : Prod Bool Bool)))
@[expose] public def Dose.verdict (w : Nat) : Except ((Prod Bool Bool)) (Nat) := (let __final : LexLeanTarget.ScreeningOracle.Dose.Trial := LexLeanTarget.ScreeningOracle.Dose.run (w); (match (__final).found with | Option.some __v => Except.ok (__v) | Option.none => Except.error (LexLeanTarget.ScreeningOracle.Dose.failure (__final))))
@[expose] public def Dose (w : Nat) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (LexLeanTarget.ScreeningOracle.Dose.Step)))) := (let __final : LexLeanTarget.ScreeningOracle.Dose.Trial := LexLeanTarget.ScreeningOracle.Dose.run (w); (match (__final).found with | Option.some __v => Except.ok ((__v, (LexLeanTarget.ScreeningOracle.Dose.Step.candidate (__v) :: ([] : List (LexLeanTarget.ScreeningOracle.Dose.Step))))) | Option.none => Except.error (LexLeanTarget.ScreeningOracle.Dose.failure (__final))))
@[expose] public def Dose.answer (w : Nat) (__trace : List (LexLeanTarget.ScreeningOracle.Dose.Step)) : Option (Nat) := (match __trace with | List.nil => Option.none | List.cons __step __rest => (match __rest with | List.nil => (match __step with | LexLeanTarget.ScreeningOracle.Dose.Step.candidate __c => LexLeanTarget.ScreeningOracle.Dose.verify (w) (__c)) | List.cons _ _ => Option.none))
public theorem Dose.verify_sound (w : Nat) (__c : Nat) (__v : Nat) : ((LexLeanTarget.ScreeningOracle.Dose.verify (w) (__c) = Option.some (__v)) -> ((LexLeanTarget.ScreeningOracle.Dose.verify (w) (__v) = Option.some (__v)) /\ LexLeanTarget.ScreeningOracle.Safe (w) (__v))) :=
by
  intro llE
  dsimp only [LexLeanTarget.ScreeningOracle.Dose.verify] at llE
  have llC := LexLeanReasoning.checked _ _ _ llE
  cases llC.right
  exact And.intro (by dsimp only [LexLeanTarget.ScreeningOracle.Dose.verify]; rw [llC.left]; rfl) (LexLeanTarget.ScreeningOracle.safe_sound _ _ llC.left)
public theorem Dose.try_sound (w : Nat) (__t : LexLeanTarget.ScreeningOracle.Dose.Trial) (__c : Nat) : ((forall (__v : Nat), (((__t).found = Option.some (__v)) -> ((LexLeanTarget.ScreeningOracle.Dose.verify (w) (__v) = Option.some (__v)) /\ LexLeanTarget.ScreeningOracle.Safe (w) (__v)))) -> (forall (__v : Nat), (((LexLeanTarget.ScreeningOracle.Dose.«try» (w) (__t) (__c)).found = Option.some (__v)) -> ((LexLeanTarget.ScreeningOracle.Dose.verify (w) (__v) = Option.some (__v)) /\ LexLeanTarget.ScreeningOracle.Safe (w) (__v))))) :=
by
  intro llH llV llE
  dsimp only [LexLeanTarget.ScreeningOracle.Dose.«try»] at llE
  split at llE
  · exact llH llV llE
  · split at llE
    · exact LexLeanTarget.ScreeningOracle.Dose.verify_sound _ _ _ llE
    · cases llE
public theorem Dose.try_count (w : Nat) (__t : LexLeanTarget.ScreeningOracle.Dose.Trial) (__c : Nat) : ((((__t).ledger).verifications <= 3) -> (((LexLeanTarget.ScreeningOracle.Dose.«try» (w) (__t) (__c)).ledger).verifications <= 3)) :=
by
  intro llH
  dsimp only [LexLeanTarget.ScreeningOracle.Dose.«try»]
  split
  · exact llH
  · split
    · rename_i llB
      exact LexLeanReasoning.bltSucc _ _ llB
    · exact llH
public theorem Dose.run_sound (w : Nat) : (forall (__v : Nat), (((LexLeanTarget.ScreeningOracle.Dose.run (w)).found = Option.some (__v)) -> ((LexLeanTarget.ScreeningOracle.Dose.verify (w) (__v) = Option.some (__v)) /\ LexLeanTarget.ScreeningOracle.Safe (w) (__v)))) :=
by
  dsimp only [LexLeanTarget.ScreeningOracle.Dose.run]
  exact (LexLeanReasoning.foldInvariant ((fun (__t : LexLeanTarget.ScreeningOracle.Dose.Trial) (__c : Nat) => LexLeanTarget.ScreeningOracle.Dose.«try» (w) (__t) (__c))) (fun (__t : LexLeanTarget.ScreeningOracle.Dose.Trial) => (forall (__v : Nat), (((__t).found = Option.some (__v)) -> ((LexLeanTarget.ScreeningOracle.Dose.verify (w) (__v) = Option.some (__v)) /\ LexLeanTarget.ScreeningOracle.Safe (w) (__v))))) (LexLeanTarget.ScreeningOracle.Dose.try_sound (w)) (LexLeanTarget.ScreeningOracle.Dose.candidates (w)) (({ found := Option.none, truncated := false, ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := 0 } : LexLeanTarget.ScreeningOracle.Dose.Ledger) } : LexLeanTarget.ScreeningOracle.Dose.Trial)) (fun llV llE => (LexLeanReasoning.noneSome _ llE)))
public theorem Dose.verifications_bounded (w : Nat) : (((LexLeanTarget.ScreeningOracle.Dose.run (w)).ledger).verifications <= 3) :=
by
  dsimp only [LexLeanTarget.ScreeningOracle.Dose.run]
  exact (LexLeanReasoning.foldInvariant ((fun (__t : LexLeanTarget.ScreeningOracle.Dose.Trial) (__c : Nat) => LexLeanTarget.ScreeningOracle.Dose.«try» (w) (__t) (__c))) (fun (__t : LexLeanTarget.ScreeningOracle.Dose.Trial) => (((__t).ledger).verifications <= 3)) (LexLeanTarget.ScreeningOracle.Dose.try_count (w)) (LexLeanTarget.ScreeningOracle.Dose.candidates (w)) (({ found := Option.none, truncated := false, ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := 0 } : LexLeanTarget.ScreeningOracle.Dose.Ledger) } : LexLeanTarget.ScreeningOracle.Dose.Trial)) (LexLeanReasoning.zeroLe _))
public theorem Dose.verdict_sound (w : Nat) : (forall (__v : Nat), ((LexLeanTarget.ScreeningOracle.Dose.verdict (w) = Except.ok (__v)) -> LexLeanTarget.ScreeningOracle.Safe (w) (__v))) :=
by
  intro llV llE
  have llS := LexLeanTarget.ScreeningOracle.Dose.run_sound w
  dsimp only [LexLeanTarget.ScreeningOracle.Dose.verdict] at llE
  generalize llRun : LexLeanTarget.ScreeningOracle.Dose.run w = llR at llE llS
  split at llE
  · rename_i llHit llF
    cases llE
    exact And.right (llS _ llF)
  · cases llE
public theorem Dose.explained (w : Nat) : (forall (__v : Nat), (forall (__trace : List (LexLeanTarget.ScreeningOracle.Dose.Step)), ((LexLeanTarget.ScreeningOracle.Dose (w) = Except.ok ((__v, __trace))) -> ((LexLeanTarget.ScreeningOracle.Dose.answer (w) (__trace) = Option.some (__v)) /\ LexLeanTarget.ScreeningOracle.Safe (w) (__v))))) :=
by
  intro llV llT llE
  have llS := LexLeanTarget.ScreeningOracle.Dose.run_sound w
  dsimp only [LexLeanTarget.ScreeningOracle.Dose] at llE
  generalize llRun : LexLeanTarget.ScreeningOracle.Dose.run w = llR at llE llS
  split at llE
  · rename_i llHit llF
    cases llE
    have llOk := llS _ llF
    exact And.intro (by dsimp only [LexLeanTarget.ScreeningOracle.Dose.answer]; exact And.left llOk) (And.right llOk)
  · cases llE

public inductive Screen.Step where
  | Propose (_ : Nat)
public structure Screen.Ledger where
  iterations : Nat
  attempts : Nat
  firings : Nat
  expansions : Nat
  verifications : Nat
  frontier : Nat
@[expose] public def Screen.observe (w : Nat) : (Prod (Nat) (Nat)) := (w, 0)
@[expose] public def Screen.fire (__s : (Prod (Nat) (Nat))) (__step : LexLeanTarget.ScreeningOracle.Screen.Step) : Option ((Prod (Nat) (Nat))) := (match __step with | LexLeanTarget.ScreeningOracle.Screen.Step.Propose __b => LexLeanTarget.ScreeningOracle.Propose.apply (__s) (__b))
@[expose] public def Screen.replay (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : LexLeanTarget.ScreeningOracle.Screen.Step) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match LexLeanTarget.ScreeningOracle.Screen.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def Screen.follow (w : Nat) (__trace : List (LexLeanTarget.ScreeningOracle.Screen.Step)) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (LexLeanCollections.listFold ((LexLeanTarget.ScreeningOracle.Screen.replay)) (Except.ok (LexLeanTarget.ScreeningOracle.Screen.observe (w))) (__trace) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))))
@[expose] public def Screen.extract (pick : (Prod (Nat) (Nat))) : Option (Nat) := (if (Nat.blt (0) ((pick).2)) then Option.some ((pick).2) else Option.none)
@[expose] public def Screen.accept (w : Nat) (__s : (Prod (Nat) (Nat))) : Option (Nat) := (match LexLeanTarget.ScreeningOracle.Screen.extract (__s) with | Option.none => Option.none | Option.some __v => (if LexLeanTarget.ScreeningOracle.safeCheck (w) (__v) then Option.some (__v) else Option.none))
@[expose] public def Screen.answer (w : Nat) (__trace : List (LexLeanTarget.ScreeningOracle.Screen.Step)) : Option (Nat) := (match LexLeanTarget.ScreeningOracle.Screen.follow (w) (__trace) with | Except.ok __s => LexLeanTarget.ScreeningOracle.Screen.accept (w) (__s) | Except.error _ => Option.none)
public structure Screen.Node where
  state : (Prod (Nat) (Nat))
  trace : List (LexLeanTarget.ScreeningOracle.Screen.Step)
public structure Screen.Search where
  frontier : List (LexLeanTarget.ScreeningOracle.Screen.Node)
  found : Option ((Prod (Nat) (LexLeanTarget.ScreeningOracle.Screen.Node)))
  truncated : Bool
  ledger : LexLeanTarget.ScreeningOracle.Screen.Ledger
@[expose] public def Screen.Propose.collect (__node : LexLeanTarget.ScreeningOracle.Screen.Node) (__acc : List (LexLeanTarget.ScreeningOracle.Screen.Node)) (__b : Nat) : List (LexLeanTarget.ScreeningOracle.Screen.Node) := (match LexLeanTarget.ScreeningOracle.Screen.fire ((__node).state) (LexLeanTarget.ScreeningOracle.Screen.Step.Propose (__b)) with | Option.none => __acc | Option.some __t => (LexLeanRuntime.append (__acc) ((({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((LexLeanTarget.ScreeningOracle.Screen.Step.Propose (__b) :: ([] : List (LexLeanTarget.ScreeningOracle.Screen.Step)))) : List (LexLeanTarget.ScreeningOracle.Screen.Step)) } : LexLeanTarget.ScreeningOracle.Screen.Node) :: ([] : List (LexLeanTarget.ScreeningOracle.Screen.Node)))) : List (LexLeanTarget.ScreeningOracle.Screen.Node)))
@[expose] public def Screen.Propose.successors (__node : LexLeanTarget.ScreeningOracle.Screen.Node) : List (LexLeanTarget.ScreeningOracle.Screen.Node) := (LexLeanCollections.listFold ((fun (__acc : List (LexLeanTarget.ScreeningOracle.Screen.Node)) (__b : Nat) => LexLeanTarget.ScreeningOracle.Screen.Propose.collect (__node) (__acc) (__b))) (([] : List (LexLeanTarget.ScreeningOracle.Screen.Node))) (LexLeanTarget.ScreeningOracle.Propose.candidates ((__node).state)) : List (LexLeanTarget.ScreeningOracle.Screen.Node))
@[expose] public def Screen.successors (__node : LexLeanTarget.ScreeningOracle.Screen.Node) : List (LexLeanTarget.ScreeningOracle.Screen.Node) := LexLeanTarget.ScreeningOracle.Screen.Propose.successors (__node)
@[expose] public def Screen.start (w : Nat) : LexLeanTarget.ScreeningOracle.Screen.Search := (let __first : List (LexLeanTarget.ScreeningOracle.Screen.Node) := (LexLeanCollections.listFold ((fun (__acc : List (LexLeanTarget.ScreeningOracle.Screen.Node)) (__n : LexLeanTarget.ScreeningOracle.Screen.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) (3)) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (LexLeanTarget.ScreeningOracle.Screen.Node)))) : List (LexLeanTarget.ScreeningOracle.Screen.Node)) else __acc))) (([] : List (LexLeanTarget.ScreeningOracle.Screen.Node))) ((({ state := LexLeanTarget.ScreeningOracle.Screen.observe (w), trace := ([] : List (LexLeanTarget.ScreeningOracle.Screen.Step)) } : LexLeanTarget.ScreeningOracle.Screen.Node) :: ([] : List (LexLeanTarget.ScreeningOracle.Screen.Node)))) : List (LexLeanTarget.ScreeningOracle.Screen.Node)); ({ frontier := __first, found := Option.none, truncated := (Nat.blt (3) (1)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := (LexLeanRuntime.length (__first) : Nat) } : LexLeanTarget.ScreeningOracle.Screen.Ledger) } : LexLeanTarget.ScreeningOracle.Screen.Search))
@[expose] public def Screen.attempts (__s : (Prod (Nat) (Nat))) : Nat := (LexLeanRuntime.length (LexLeanTarget.ScreeningOracle.Propose.candidates (__s)) : Nat)
@[expose] public def Screen.searchStep (w : Nat) (__r : LexLeanTarget.ScreeningOracle.Screen.Search) : Option (LexLeanTarget.ScreeningOracle.Screen.Search) := (match (__r).found with | Option.some _ => Option.none | Option.none => (match (__r).frontier with | List.nil => Option.none | List.cons __node __rest => (match LexLeanTarget.ScreeningOracle.Screen.accept (w) ((__node).state) with | Option.some __v => Option.some (({ frontier := __rest, found := Option.some ((__v, __node)), truncated := (__r).truncated, ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := ((__r).ledger).attempts, firings := ((__r).ledger).firings, expansions := ((__r).ledger).expansions, verifications := (((__r).ledger).verifications + 1), frontier := ((__r).ledger).frontier } : LexLeanTarget.ScreeningOracle.Screen.Ledger) } : LexLeanTarget.ScreeningOracle.Screen.Search)) | Option.none => (let __successors : List (LexLeanTarget.ScreeningOracle.Screen.Node) := LexLeanTarget.ScreeningOracle.Screen.successors (__node); (let __fresh : List (LexLeanTarget.ScreeningOracle.Screen.Node) := __successors; (let __ordered : List (LexLeanTarget.ScreeningOracle.Screen.Node) := (LexLeanRuntime.append (__fresh) (__rest) : List (LexLeanTarget.ScreeningOracle.Screen.Node)); (let __next : List (LexLeanTarget.ScreeningOracle.Screen.Node) := (LexLeanCollections.listFold ((fun (__acc : List (LexLeanTarget.ScreeningOracle.Screen.Node)) (__n : LexLeanTarget.ScreeningOracle.Screen.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) (3)) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (LexLeanTarget.ScreeningOracle.Screen.Node)))) : List (LexLeanTarget.ScreeningOracle.Screen.Node)) else __acc))) (([] : List (LexLeanTarget.ScreeningOracle.Screen.Node))) (__ordered) : List (LexLeanTarget.ScreeningOracle.Screen.Node)); Option.some (({ frontier := __next, found := Option.none, truncated := ((__r).truncated || (Nat.blt (3) ((LexLeanRuntime.length (__ordered) : Nat)))), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + LexLeanTarget.ScreeningOracle.Screen.attempts ((__node).state)), firings := (((__r).ledger).firings + (LexLeanRuntime.length (__successors) : Nat)), expansions := (((__r).ledger).expansions + 1), verifications := (((__r).ledger).verifications + 1), frontier := (if (Nat.blt (((__r).ledger).frontier) ((LexLeanRuntime.length (__next) : Nat))) then (LexLeanRuntime.length (__next) : Nat) else ((__r).ledger).frontier) } : LexLeanTarget.ScreeningOracle.Screen.Ledger) } : LexLeanTarget.ScreeningOracle.Screen.Search)))))))))
@[expose] public def Screen.run (w : Nat) : (Prod (LexLeanTarget.ScreeningOracle.Screen.Search) (Bool)) := (LexLeanCollections.iterateUntil ((fun (__r : LexLeanTarget.ScreeningOracle.Screen.Search) => LexLeanTarget.ScreeningOracle.Screen.searchStep (w) (__r))) (8) (LexLeanTarget.ScreeningOracle.Screen.start (w)) : (Prod (LexLeanTarget.ScreeningOracle.Screen.Search) (Bool)))
@[expose] public def Screen.failure (__saturated : Bool) (__truncated : Bool) : (Prod Bool Bool) := (if (__saturated && (!__truncated)) then ((false, true) : Prod Bool Bool) else ((false, false) : Prod Bool Bool))
@[expose] public def Screen (w : Nat) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (LexLeanTarget.ScreeningOracle.Screen.Step)))) := (let __final : (Prod (LexLeanTarget.ScreeningOracle.Screen.Search) (Bool)) := LexLeanTarget.ScreeningOracle.Screen.run (w); (match ((__final).1).found with | Option.some __hit => Except.ok (((__hit).1, ((__hit).2).trace)) | Option.none => Except.error (LexLeanTarget.ScreeningOracle.Screen.failure ((__final).2) (((__final).1).truncated))))
@[expose] public def Screen.verdict (w : Nat) : Except ((Prod Bool Bool)) (Nat) := (match LexLeanTarget.ScreeningOracle.Screen (w) with | Except.ok __p => Except.ok ((__p).1) | Except.error __e => Except.error (__e))
public theorem Screen.fire_sound (__s : (Prod (Nat) (Nat))) (__step : LexLeanTarget.ScreeningOracle.Screen.Step) (__t : (Prod (Nat) (Nat))) : ((LexLeanTarget.ScreeningOracle.Screen.fire (__s) (__step) = Option.some (__t)) -> LexLeanTarget.ScreeningOracle.Proposed (__s) (__t)) :=
by
  cases __step with
  | Propose __b =>
    exact LexLeanTarget.ScreeningOracle.Propose.apply_sound __s __b __t
public theorem Screen.replay_fire (__s : (Prod (Nat) (Nat))) (__step : LexLeanTarget.ScreeningOracle.Screen.Step) (__t : (Prod (Nat) (Nat))) : ((LexLeanTarget.ScreeningOracle.Screen.fire (__s) (__step) = Option.some (__t)) -> (LexLeanTarget.ScreeningOracle.Screen.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [LexLeanTarget.ScreeningOracle.Screen.replay]
  rw [llE]
public theorem Screen.replay_sound (w : Nat) (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : LexLeanTarget.ScreeningOracle.Screen.Step) : ((LexLeanReasoning.Reaches ((LexLeanTarget.ScreeningOracle.Proposed)) (LexLeanTarget.ScreeningOracle.Screen.observe (w)) (__acc)) -> (LexLeanReasoning.Reaches ((LexLeanTarget.ScreeningOracle.Proposed)) (LexLeanTarget.ScreeningOracle.Screen.observe (w)) (LexLeanTarget.ScreeningOracle.Screen.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [LexLeanTarget.ScreeningOracle.Screen.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (LexLeanTarget.ScreeningOracle.Screen.fire_sound llS __step _ ‹_›)
public theorem Screen.derivation (w : Nat) (__trace : List (LexLeanTarget.ScreeningOracle.Screen.Step)) : (LexLeanReasoning.Reaches ((LexLeanTarget.ScreeningOracle.Proposed)) (LexLeanTarget.ScreeningOracle.Screen.observe (w)) (LexLeanTarget.ScreeningOracle.Screen.follow (w) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((LexLeanTarget.ScreeningOracle.Screen.replay)) (fun (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) => (LexLeanReasoning.Reaches ((LexLeanTarget.ScreeningOracle.Proposed)) (LexLeanTarget.ScreeningOracle.Screen.observe (w)) (__acc))) (LexLeanTarget.ScreeningOracle.Screen.replay_sound (w)) (__trace) (Except.ok (LexLeanTarget.ScreeningOracle.Screen.observe (w))) (LexLeanReasoning.reachesStart ((LexLeanTarget.ScreeningOracle.Proposed)) (LexLeanTarget.ScreeningOracle.Screen.observe (w))))
public theorem Screen.accept_sound (w : Nat) (__s : (Prod (Nat) (Nat))) (__v : Nat) : ((LexLeanTarget.ScreeningOracle.Screen.accept (w) (__s) = Option.some (__v)) -> LexLeanTarget.ScreeningOracle.Safe (w) (__v)) :=
by
  intro llE
  dsimp only [LexLeanTarget.ScreeningOracle.Screen.accept] at llE
  split at llE
  · cases llE
  · have llC := LexLeanReasoning.checked _ _ _ llE
    exact llC.right ▸ LexLeanTarget.ScreeningOracle.safe_sound _ _ llC.left
public theorem Screen.extend (w : Nat) (__node : LexLeanTarget.ScreeningOracle.Screen.Node) (__step : LexLeanTarget.ScreeningOracle.Screen.Step) (__t : (Prod (Nat) (Nat))) : ((LexLeanTarget.ScreeningOracle.Screen.follow (w) ((__node).trace) = Except.ok ((__node).state)) -> ((LexLeanTarget.ScreeningOracle.Screen.fire ((__node).state) (__step) = Option.some (__t)) -> (LexLeanTarget.ScreeningOracle.Screen.follow (w) ((LexLeanRuntime.append ((__node).trace) ((__step :: ([] : List (LexLeanTarget.ScreeningOracle.Screen.Step)))) : List (LexLeanTarget.ScreeningOracle.Screen.Step))) = Except.ok (__t)))) :=
by
  intro llH llE
  dsimp only [LexLeanTarget.ScreeningOracle.Screen.follow]
  rw [LexLeanReasoning.foldSnoc]
  dsimp only [LexLeanTarget.ScreeningOracle.Screen.follow] at llH
  rw [llH]
  exact LexLeanTarget.ScreeningOracle.Screen.replay_fire _ __step __t llE
public theorem Screen.Propose.successors_ok (w : Nat) (__node : LexLeanTarget.ScreeningOracle.Screen.Node) : ((LexLeanTarget.ScreeningOracle.Screen.follow (w) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : LexLeanTarget.ScreeningOracle.Screen.Node) => (LexLeanTarget.ScreeningOracle.Screen.follow (w) ((__n).trace) = Except.ok ((__n).state)))) (LexLeanTarget.ScreeningOracle.Screen.Propose.successors (__node)))) :=
by
  intro llH
  dsimp only [LexLeanTarget.ScreeningOracle.Screen.Propose.successors]
  exact LexLeanReasoning.foldInvariant _ (LexLeanReasoning.All (fun (llN : LexLeanTarget.ScreeningOracle.Screen.Node) => LexLeanTarget.ScreeningOracle.Screen.follow _ llN.trace = Except.ok llN.state))
    (fun llA llB llP => by
      dsimp only [LexLeanTarget.ScreeningOracle.Screen.Propose.collect]
      split
      · exact llP
      · exact LexLeanReasoning.allAppend _ _ _ llP (LexLeanReasoning.allSingle _ _ (LexLeanTarget.ScreeningOracle.Screen.extend _ __node _ _ llH ‹_›)))
    _ _ LexLeanReasoning.All.nil
public theorem Screen.successors_ok (w : Nat) (__node : LexLeanTarget.ScreeningOracle.Screen.Node) : ((LexLeanTarget.ScreeningOracle.Screen.follow (w) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : LexLeanTarget.ScreeningOracle.Screen.Node) => (LexLeanTarget.ScreeningOracle.Screen.follow (w) ((__n).trace) = Except.ok ((__n).state)))) (LexLeanTarget.ScreeningOracle.Screen.successors (__node)))) :=
  (fun llH => (LexLeanTarget.ScreeningOracle.Screen.Propose.successors_ok (w) (__node) llH))
public theorem Screen.search_step (w : Nat) (__r : LexLeanTarget.ScreeningOracle.Screen.Search) (__q : LexLeanTarget.ScreeningOracle.Screen.Search) : ((LexLeanTarget.ScreeningOracle.Screen.searchStep (w) (__r) = Option.some (__q)) -> ((LexLeanReasoning.SearchOk ((fun (__n : LexLeanTarget.ScreeningOracle.Screen.Node) => (LexLeanTarget.ScreeningOracle.Screen.follow (w) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : LexLeanTarget.ScreeningOracle.Screen.Node) => LexLeanTarget.ScreeningOracle.Screen.accept (w) ((__n).state))) ((__r).frontier) ((__r).found)) -> (LexLeanReasoning.SearchOk ((fun (__n : LexLeanTarget.ScreeningOracle.Screen.Node) => (LexLeanTarget.ScreeningOracle.Screen.follow (w) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : LexLeanTarget.ScreeningOracle.Screen.Node) => LexLeanTarget.ScreeningOracle.Screen.accept (w) ((__n).state))) ((__q).frontier) ((__q).found)))) :=
by
  intro llE llH
  dsimp only [LexLeanTarget.ScreeningOracle.Screen.searchStep] at llE
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
        exact And.intro (LexLeanReasoning.capAll _ _ _ (LexLeanReasoning.allAppend _ _ _ (LexLeanTarget.ScreeningOracle.Screen.successors_ok _ llNode llNodeOk) llRestOk)) True.intro
public theorem Screen.search_peak (w : Nat) (__r : LexLeanTarget.ScreeningOracle.Screen.Search) (__q : LexLeanTarget.ScreeningOracle.Screen.Search) : ((LexLeanTarget.ScreeningOracle.Screen.searchStep (w) (__r) = Option.some (__q)) -> ((((__r).ledger).frontier <= 3) -> (((__q).ledger).frontier <= 3))) :=
by
  intro llE llH
  dsimp only [LexLeanTarget.ScreeningOracle.Screen.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        exact llH
      · cases llE
        exact LexLeanReasoning.peakBound _ _ _ llH (LexLeanReasoning.capBound _ _)
public theorem Screen.search_count (w : Nat) (__r : LexLeanTarget.ScreeningOracle.Screen.Search) (__q : LexLeanTarget.ScreeningOracle.Screen.Search) : ((LexLeanTarget.ScreeningOracle.Screen.searchStep (w) (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [LexLeanTarget.ScreeningOracle.Screen.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        rfl
      · cases llE
        rfl
public theorem Screen.search_verify_growth (w : Nat) (__r : LexLeanTarget.ScreeningOracle.Screen.Search) (__q : LexLeanTarget.ScreeningOracle.Screen.Search) : ((LexLeanTarget.ScreeningOracle.Screen.searchStep (w) (__r) = Option.some (__q)) -> (((__q).ledger).verifications <= (((__r).ledger).verifications + 1))) :=
by
  intro llE
  dsimp only [LexLeanTarget.ScreeningOracle.Screen.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem Screen.search_expanded (w : Nat) (__r : LexLeanTarget.ScreeningOracle.Screen.Search) (__q : LexLeanTarget.ScreeningOracle.Screen.Search) : ((LexLeanTarget.ScreeningOracle.Screen.searchStep (w) (__r) = Option.some (__q)) -> (((__q).ledger).expansions <= (((__r).ledger).expansions + 1))) :=
by
  intro llE
  dsimp only [LexLeanTarget.ScreeningOracle.Screen.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem Screen.search_ok (w : Nat) : (LexLeanReasoning.SearchOk ((fun (__n : LexLeanTarget.ScreeningOracle.Screen.Node) => (LexLeanTarget.ScreeningOracle.Screen.follow (w) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : LexLeanTarget.ScreeningOracle.Screen.Node) => LexLeanTarget.ScreeningOracle.Screen.accept (w) ((__n).state))) (((LexLeanTarget.ScreeningOracle.Screen.run (w)).1).frontier) (((LexLeanTarget.ScreeningOracle.Screen.run (w)).1).found)) :=
by
  dsimp only [LexLeanTarget.ScreeningOracle.Screen.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((fun (__r : LexLeanTarget.ScreeningOracle.Screen.Search) => LexLeanTarget.ScreeningOracle.Screen.searchStep (w) (__r))) (fun (__r : LexLeanTarget.ScreeningOracle.Screen.Search) => (LexLeanReasoning.SearchOk ((fun (__n : LexLeanTarget.ScreeningOracle.Screen.Node) => (LexLeanTarget.ScreeningOracle.Screen.follow (w) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : LexLeanTarget.ScreeningOracle.Screen.Node) => LexLeanTarget.ScreeningOracle.Screen.accept (w) ((__n).state))) ((__r).frontier) ((__r).found))) (LexLeanTarget.ScreeningOracle.Screen.search_step (w)) (8) (LexLeanTarget.ScreeningOracle.Screen.start (w)) (LexLeanReasoning.searchStart ((fun (__n : LexLeanTarget.ScreeningOracle.Screen.Node) => (LexLeanTarget.ScreeningOracle.Screen.follow (w) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : LexLeanTarget.ScreeningOracle.Screen.Node) => LexLeanTarget.ScreeningOracle.Screen.accept (w) ((__n).state))) _ (LexLeanReasoning.capAll ((fun (__n : LexLeanTarget.ScreeningOracle.Screen.Node) => (LexLeanTarget.ScreeningOracle.Screen.follow (w) ((__n).trace) = Except.ok ((__n).state)))) (3) _ (LexLeanReasoning.allSingle ((fun (__n : LexLeanTarget.ScreeningOracle.Screen.Node) => (LexLeanTarget.ScreeningOracle.Screen.follow (w) ((__n).trace) = Except.ok ((__n).state)))) (({ state := LexLeanTarget.ScreeningOracle.Screen.observe (w), trace := ([] : List (LexLeanTarget.ScreeningOracle.Screen.Step)) } : LexLeanTarget.ScreeningOracle.Screen.Node)) rfl))))
public theorem Screen.frontier_bounded (w : Nat) : ((((LexLeanTarget.ScreeningOracle.Screen.run (w)).1).ledger).frontier <= 3) :=
by
  dsimp only [LexLeanTarget.ScreeningOracle.Screen.run]
  exact (LexLeanReasoning.iterateUntilBound ((fun (__r : LexLeanTarget.ScreeningOracle.Screen.Search) => LexLeanTarget.ScreeningOracle.Screen.searchStep (w) (__r))) ((fun (__r : LexLeanTarget.ScreeningOracle.Screen.Search) => ((__r).ledger).frontier)) (3) (LexLeanTarget.ScreeningOracle.Screen.search_peak (w)) (8) (LexLeanTarget.ScreeningOracle.Screen.start (w)) (LexLeanReasoning.capBound (3) _))
public theorem Screen.verifications_bounded (w : Nat) : ((((LexLeanTarget.ScreeningOracle.Screen.run (w)).1).ledger).verifications <= 8) :=
by
  dsimp only [LexLeanTarget.ScreeningOracle.Screen.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : LexLeanTarget.ScreeningOracle.Screen.Search) => LexLeanTarget.ScreeningOracle.Screen.searchStep (w) (__r))) ((fun (__r : LexLeanTarget.ScreeningOracle.Screen.Search) => ((__r).ledger).verifications)) (LexLeanTarget.ScreeningOracle.Screen.search_verify_growth (w)) (8) (LexLeanTarget.ScreeningOracle.Screen.start (w)) rfl)
public theorem Screen.expansions_bounded (w : Nat) : ((((LexLeanTarget.ScreeningOracle.Screen.run (w)).1).ledger).expansions <= 8) :=
by
  dsimp only [LexLeanTarget.ScreeningOracle.Screen.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : LexLeanTarget.ScreeningOracle.Screen.Search) => LexLeanTarget.ScreeningOracle.Screen.searchStep (w) (__r))) ((fun (__r : LexLeanTarget.ScreeningOracle.Screen.Search) => ((__r).ledger).expansions)) (LexLeanTarget.ScreeningOracle.Screen.search_expanded (w)) (8) (LexLeanTarget.ScreeningOracle.Screen.start (w)) rfl)
public theorem Screen.iterations_bounded (w : Nat) : ((((LexLeanTarget.ScreeningOracle.Screen.run (w)).1).ledger).iterations <= 8) :=
by
  dsimp only [LexLeanTarget.ScreeningOracle.Screen.run]
  exact (LexLeanReasoning.iterateUntilCount ((fun (__r : LexLeanTarget.ScreeningOracle.Screen.Search) => LexLeanTarget.ScreeningOracle.Screen.searchStep (w) (__r))) ((fun (__r : LexLeanTarget.ScreeningOracle.Screen.Search) => ((__r).ledger).iterations)) (LexLeanTarget.ScreeningOracle.Screen.search_count (w)) (8) (LexLeanTarget.ScreeningOracle.Screen.start (w)) rfl)
public theorem Screen.explained (w : Nat) : (forall (__v : Nat), (forall (__trace : List (LexLeanTarget.ScreeningOracle.Screen.Step)), ((LexLeanTarget.ScreeningOracle.Screen (w) = Except.ok ((__v, __trace))) -> ((LexLeanTarget.ScreeningOracle.Screen.answer (w) (__trace) = Option.some (__v)) /\ LexLeanTarget.ScreeningOracle.Safe (w) (__v))))) :=
by
  intro llV llT llE
  have llSearch := LexLeanTarget.ScreeningOracle.Screen.search_ok w
  dsimp only [LexLeanTarget.ScreeningOracle.Screen] at llE
  generalize llRun : LexLeanTarget.ScreeningOracle.Screen.run w = llR at llE llSearch
  split at llE
  · rename_i llHit llF
    cases llE
    have llOk := And.right llSearch
    rw [llF] at llOk
    exact And.intro (by dsimp only [LexLeanTarget.ScreeningOracle.Screen.answer]; rw [And.left llOk]; exact And.right llOk) (LexLeanTarget.ScreeningOracle.Screen.accept_sound _ _ _ (And.right llOk))
  · cases llE
public theorem Screen.verdict_sound (w : Nat) : (forall (__v : Nat), ((LexLeanTarget.ScreeningOracle.Screen.verdict (w) = Except.ok (__v)) -> LexLeanTarget.ScreeningOracle.Safe (w) (__v))) :=
by
  intro llV llE
  dsimp only [LexLeanTarget.ScreeningOracle.Screen.verdict] at llE
  generalize llHP : LexLeanTarget.ScreeningOracle.Screen w = llR at llE
  cases llR with
  | error _ => cases llE
  | ok llP =>
    cases llE
    exact And.right (LexLeanTarget.ScreeningOracle.Screen.explained _ llP.1 llP.2 llHP)

end LexLeanTarget.ScreeningOracle
