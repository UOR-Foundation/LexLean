module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanRuntime.Models
public import Models.Ledger
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Models.Flows

public theorem flow_capped_sound (s : Nat) : ((Models.Ledger.cappedCheck (s) = true) -> Models.Ledger.Capped (s)) := by
  exact Models.Ledger.capped_check_sound (s)

public theorem FlowContract.invariant_sound : LexLeanModels.Sound1 ((Models.Ledger.cappedCheck)) ((Models.Ledger.Capped)) := Models.Flows.flow_capped_sound

@[expose] public def ClampGuess.initial : Nat := 0
@[expose] public def ClampGuess (s : Nat) (x : Nat) : (Prod (Nat) (Except ((Prod Bool Bool)) (Nat))) := (let __step1 : (Prod (Nat) (Nat)) := Models.Ledger.ClampModel (s) (x); (let __stage1 : Nat := (__step1).2; (let __state1 : Nat := (__step1).1; (if Models.Ledger.withinCheck (__stage1) then (let __stage2 : Nat := Models.Ledger.GuessModel (__stage1); (if Models.Ledger.belowCheck (__stage1) (__stage2) then (__state1, Except.ok (__stage2)) else (s, Except.error (((true, false) : Prod Bool Bool))))) else (s, Except.error (((false, false) : Prod Bool Bool)))))))

public theorem flow_entry (s : Nat) (_x : Nat) : (Models.Ledger.Capped (s) -> Models.Ledger.Capped (s)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Models.Ledger.Capped, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose] public def FlowModel.initial : Nat := Models.Flows.ClampGuess.initial
@[expose] public def FlowModel (__state : Nat) (__input : Nat) : (Prod (Nat) (Except ((Prod Bool Bool)) (Nat))) := Models.Flows.ClampGuess (__state) (__input)

public theorem pair_within_sound (x : Nat) : ((Models.Ledger.withinCheck (x) = true) -> Models.Ledger.Within (x)) := by
  exact Models.Ledger.within_check_sound (x)

public theorem PairGuessContract.precondition_sound : LexLeanModels.Sound1 ((Models.Ledger.withinCheck)) ((Models.Ledger.Within)) := Models.Flows.pair_within_sound

@[expose] public def GuessTwice (x : Nat) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (let __left : Nat := Models.Ledger.GuessModel (x); (if Models.Ledger.belowCheck (x) (__left) then (let __right : Nat := Models.Ledger.GuessExactModel (x); (if Models.Ledger.belowCheck (x) (__right) then Except.ok ((__left, __right)) else Except.error (((true, false) : Prod Bool Bool)))) else Except.error (((true, false) : Prod Bool Bool))))

public theorem pair_entry_left (x : Nat) : (Models.Ledger.Within (x) -> Models.Ledger.Within (x)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Models.Ledger.Within, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem pair_entry_right (x : Nat) : (Models.Ledger.Within (x) -> Models.Ledger.Within (x)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Models.Ledger.Within, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose] public def PairGuessModel (__input : Nat) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := Models.Flows.GuessTwice (__input)

@[expose] public def GuessOvershoot (x : Nat) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (let __left : Nat := Models.Ledger.GuessModel (x); (if Models.Ledger.belowCheck (x) (__left) then (let __right : Nat := Models.Ledger.OvershootModel (x); (if Models.Ledger.belowCheck (x) (__right) then Except.ok ((__left, __right)) else Except.error (((true, false) : Prod Bool Bool)))) else Except.error (((true, false) : Prod Bool Bool))))

@[expose] public def OvershootPairModel (__input : Nat) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := Models.Flows.GuessOvershoot (__input)

@[expose, reducible] public def Few (xs : List (Nat)) : Prop := ((LexLeanRuntime.length (xs) : Nat) <= 100)

@[expose] public def fewCheck (xs : List (Nat)) : Bool := (Nat.ble ((LexLeanRuntime.length (xs) : Nat)) (100))

public theorem few_check_sound (xs : List (Nat)) : ((fewCheck (xs) = true) -> Few (xs)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Few, fewCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem SpillStreamContract.precondition_sound : LexLeanModels.Sound1 ((Models.Flows.fewCheck)) ((Models.Flows.Few)) := Models.Flows.few_check_sound

@[expose] public def SpillStream (xs : List (Nat)) : Except ((Prod Bool Bool)) (List (Nat)) := (let __scanned : Except ((Prod Bool Bool)) ((Prod (Nat) (List (Nat)))) := (LexLeanCollections.listFold ((fun (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (List (Nat))))) (__item : Nat) => (match __acc with | Except.error _ => __acc | Except.ok __state => (if Models.Ledger.cappedCheck ((__state).1) then (if Models.Ledger.smallCheck ((__state).1) (__item) then (let __step : (Prod (Nat) (Nat)) := Models.Ledger.SpillModel ((__state).1) (__item); (if Models.Ledger.cappedCheck ((__step).1) then Except.ok (((__step).1, (LexLeanRuntime.append ((__state).2) (((__step).2 :: ([] : List (Nat)))) : List (Nat)))) else Except.error (((true, true) : Prod Bool Bool)))) else Except.error (((false, false) : Prod Bool Bool))) else Except.error (((false, true) : Prod Bool Bool)))))) (Except.ok ((Models.Ledger.SpillModel.initial, ([] : List (Nat))))) (xs) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (Nat))))); (match __scanned with | Except.error __violation => Except.error (__violation) | Except.ok __state => Except.ok ((__state).2)))

@[expose] public def SpillStreamModel (__input : List (Nat)) : Except ((Prod Bool Bool)) (List (Nat)) := Models.Flows.SpillStream (__input)

@[expose, reducible] public def TwinCapped (s : (Prod (Nat) (Nat))) : Prop := Models.Ledger.Capped ((s).1)

@[expose] public def twinCappedCheck (s : (Prod (Nat) (Nat))) : Bool := Models.Ledger.cappedCheck ((s).1)

public theorem twin_capped_check_sound (s : (Prod (Nat) (Nat))) : ((twinCappedCheck (s) = true) -> TwinCapped (s)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Models.Ledger.Capped, Models.Ledger.cappedCheck, TwinCapped, twinCappedCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose, reducible] public def TwinSmall (_s : (Prod (Nat) (Nat))) (x : Nat) : Prop := (x <= 10)

@[expose] public def twinSmallCheck (_s : (Prod (Nat) (Nat))) (x : Nat) : Bool := (Nat.ble (x) (10))

public theorem twin_small_check_sound (s : (Prod (Nat) (Nat))) (x : Nat) : ((twinSmallCheck (s) (x) = true) -> TwinSmall (s) (x)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [TwinSmall, twinSmallCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem TwinContract.invariant_sound : LexLeanModels.Sound1 ((Models.Flows.twinCappedCheck)) ((Models.Flows.TwinCapped)) := Models.Flows.twin_capped_check_sound
public theorem TwinContract.precondition_sound : LexLeanModels.Sound2 ((Models.Flows.twinSmallCheck)) ((Models.Flows.TwinSmall)) := Models.Flows.twin_small_check_sound

@[expose] public def ClampTally.initial : (Prod (Nat) (Nat)) := (0, 0)
@[expose] public def ClampTally (s : (Prod (Nat) (Nat))) (x : Nat) : (Prod ((Prod (Nat) (Nat))) ((Prod (Nat) (Nat)))) := (let __left_step : (Prod (Nat) (Nat)) := Models.Ledger.ClampModel ((s).1) (x); (let __left : Nat := (__left_step).2; (let __left_state : Nat := (__left_step).1; (let __right_step : (Prod (Nat) (Nat)) := Models.Ledger.TallyModel ((s).2) (x); (let __right : Nat := (__right_step).2; (let __right_state : Nat := (__right_step).1; ((__left_state, __right_state), (__left, __right))))))))

public theorem twin_entry_left (s : (Prod (Nat) (Nat))) (x : Nat) : (TwinCapped (s) -> (TwinSmall (s) (x) -> Models.Ledger.Capped ((s).1))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Models.Ledger.Capped, TwinCapped, TwinSmall, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem twin_entry_right (s : (Prod (Nat) (Nat))) (x : Nat) : (TwinCapped (s) -> (TwinSmall (s) (x) -> Models.Ledger.Small ((s).2) (x))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Models.Ledger.Small, TwinCapped, TwinSmall, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose] public def TwinModel.initial : (Prod (Nat) (Nat)) := Models.Flows.ClampTally.initial
@[expose] public def TwinModel (__state : (Prod (Nat) (Nat))) (__input : Nat) : (Prod ((Prod (Nat) (Nat))) ((Prod (Nat) (Nat)))) := Models.Flows.ClampTally (__state) (__input)

@[expose, reducible] public def BothCapped (s : (Prod (Nat) (Nat))) : Prop := (Models.Ledger.Capped ((s).1) /\ Models.Ledger.Capped ((s).2))

@[expose] public def bothCappedCheck (s : (Prod (Nat) (Nat))) : Bool := (Nat.ble (((LexLeanRuntime.subtract ((s).1) (100) : Nat) + (LexLeanRuntime.subtract ((s).2) (100) : Nat))) (0))

public theorem both_capped_check_sound (s : (Prod (Nat) (Nat))) : ((bothCappedCheck (s) = true) -> BothCapped (s)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [BothCapped, Models.Ledger.Capped, bothCappedCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem PairClampContract.invariant_sound : LexLeanModels.Sound1 ((Models.Flows.bothCappedCheck)) ((Models.Flows.BothCapped)) := Models.Flows.both_capped_check_sound

@[expose] public def ClampBoth.initial : (Prod (Nat) (Nat)) := (0, 0)
@[expose] public def ClampBoth (s : (Prod (Nat) (Nat))) (x : (Prod (Nat) (Nat))) : (Prod ((Prod (Nat) (Nat))) ((Prod (Nat) (Nat)))) := (let __left_step : (Prod (Nat) (Nat)) := Models.Ledger.ClampModel ((s).1) ((x).1); (let __left : Nat := (__left_step).2; (let __left_state : Nat := (__left_step).1; (let __right_step : (Prod (Nat) (Nat)) := Models.Ledger.ClampModel ((s).2) ((x).2); (let __right : Nat := (__right_step).2; (let __right_state : Nat := (__right_step).1; ((__left_state, __right_state), (__left, __right))))))))

public theorem both_entry_left (s : (Prod (Nat) (Nat))) (_x : (Prod (Nat) (Nat))) : (BothCapped (s) -> Models.Ledger.Capped ((s).1)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [BothCapped, Models.Ledger.Capped, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem both_entry_right (s : (Prod (Nat) (Nat))) (_x : (Prod (Nat) (Nat))) : (BothCapped (s) -> Models.Ledger.Capped ((s).2)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [BothCapped, Models.Ledger.Capped, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose] public def PairClampModel.initial : (Prod (Nat) (Nat)) := Models.Flows.ClampBoth.initial
@[expose] public def PairClampModel (__state : (Prod (Nat) (Nat))) (__input : (Prod (Nat) (Nat))) : (Prod ((Prod (Nat) (Nat))) ((Prod (Nat) (Nat)))) := Models.Flows.ClampBoth (__state) (__input)

public theorem BranchContract.invariant_sound : LexLeanModels.Sound1 ((Models.Flows.twinCappedCheck)) ((Models.Flows.TwinCapped)) := Models.Flows.twin_capped_check_sound
public theorem BranchContract.precondition_sound : LexLeanModels.Sound2 ((Models.Flows.twinSmallCheck)) ((Models.Flows.TwinSmall)) := Models.Flows.twin_small_check_sound

@[expose] public def ClampOrTally.initial : (Prod (Nat) (Nat)) := (0, 0)
@[expose] public def ClampOrTally (s : (Prod (Nat) (Nat))) (x : Nat) : (Prod ((Prod (Nat) (Nat))) (Nat)) := (if (Nat.ble (x) (5)) then (let __then_step : (Prod (Nat) (Nat)) := Models.Ledger.ClampModel ((s).1) (x); (let __then : Nat := (__then_step).2; (let __then_state : Nat := (__then_step).1; ((__then_state, (s).2), __then)))) else (let __else_step : (Prod (Nat) (Nat)) := Models.Ledger.TallyModel ((s).2) (x); (let __else : Nat := (__else_step).2; (let __else_state : Nat := (__else_step).1; (((s).1, __else_state), __else)))))

public theorem branch_entry_then (s : (Prod (Nat) (Nat))) (x : Nat) : (TwinCapped (s) -> (TwinSmall (s) (x) -> (((Nat.ble (x) (5)) = true) -> Models.Ledger.Capped ((s).1)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Models.Ledger.Capped, TwinCapped, TwinSmall, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem branch_entry_else (s : (Prod (Nat) (Nat))) (x : Nat) : (TwinCapped (s) -> (TwinSmall (s) (x) -> (((Nat.ble (x) (5)) = false) -> Models.Ledger.Small ((s).2) (x)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Models.Ledger.Small, TwinCapped, TwinSmall, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose] public def BranchModel.initial : (Prod (Nat) (Nat)) := Models.Flows.ClampOrTally.initial
@[expose] public def BranchModel (__state : (Prod (Nat) (Nat))) (__input : Nat) : (Prod ((Prod (Nat) (Nat))) (Nat)) := Models.Flows.ClampOrTally (__state) (__input)

public theorem ChainContract.invariant_sound : LexLeanModels.Sound1 ((Models.Flows.twinCappedCheck)) ((Models.Flows.TwinCapped)) := Models.Flows.twin_capped_check_sound
public theorem ChainContract.precondition_sound : LexLeanModels.Sound2 ((Models.Flows.twinSmallCheck)) ((Models.Flows.TwinSmall)) := Models.Flows.twin_small_check_sound

public theorem spill_tally (s : (Prod (Nat) (Nat))) (x : Nat) : (Models.Ledger.Capped ((s).1) -> (Models.Ledger.Small ((s).1) (x) -> Models.Ledger.Small ((s).2) ((Models.Ledger.SpillModel ((s).1) (x)).2))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Models.Ledger.Capped, Models.Ledger.Small, Models.Ledger.Spill, Models.Ledger.SpillModel, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose] public def SpillTally.initial : (Prod (Nat) (Nat)) := (0, 0)
@[expose] public def SpillTally (s : (Prod (Nat) (Nat))) (x : Nat) : (Prod ((Prod (Nat) (Nat))) (Except ((Prod Bool Bool)) (Nat))) := (let __step1 : (Prod (Nat) (Nat)) := Models.Ledger.SpillModel ((s).1) (x); (let __stage1 : Nat := (__step1).2; (let __state1 : Nat := (__step1).1; (if Models.Ledger.cappedCheck (__state1) then (let __step2 : (Prod (Nat) (Nat)) := Models.Ledger.TallyModel ((s).2) (__stage1); (let __stage2 : Nat := (__step2).2; (let __state2 : Nat := (__step2).1; ((__state1, __state2), Except.ok (__stage2))))) else (s, Except.error (((true, true) : Prod Bool Bool)))))))

public theorem chain_entry_invariant (s : (Prod (Nat) (Nat))) (x : Nat) : (TwinCapped (s) -> (TwinSmall (s) (x) -> Models.Ledger.Capped ((s).1))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Models.Ledger.Capped, TwinCapped, TwinSmall, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem chain_entry_precondition (s : (Prod (Nat) (Nat))) (x : Nat) : (TwinCapped (s) -> (TwinSmall (s) (x) -> Models.Ledger.Small ((s).1) (x))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Models.Ledger.Small, TwinCapped, TwinSmall, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose] public def ChainModel.initial : (Prod (Nat) (Nat)) := Models.Flows.SpillTally.initial
@[expose] public def ChainModel (__state : (Prod (Nat) (Nat))) (__input : Nat) : (Prod ((Prod (Nat) (Nat))) (Except ((Prod Bool Bool)) (Nat))) := Models.Flows.SpillTally (__state) (__input)

public theorem RawChainContract.invariant_sound : LexLeanModels.Sound1 ((Models.Flows.twinCappedCheck)) ((Models.Flows.TwinCapped)) := Models.Flows.twin_capped_check_sound

@[expose] public def TallySpillRaw.initial : (Prod (Nat) (Nat)) := (0, 0)
@[expose] public def TallySpillRaw (s : (Prod (Nat) (Nat))) (x : Nat) : (Prod ((Prod (Nat) (Nat))) (Except ((Prod Bool Bool)) (Nat))) := (let __step1 : (Prod (Nat) (Nat)) := Models.Ledger.FreeModel ((s).1) (x); (let __stage1 : Nat := (__step1).2; (let __state1 : Nat := (__step1).1; (if Models.Ledger.cappedCheck ((s).2) then (if Models.Ledger.smallCheck ((s).2) (__stage1) then (let __step2 : (Prod (Nat) (Nat)) := Models.Ledger.SpillRawModel ((s).2) (__stage1); (let __stage2 : Nat := (__step2).2; (let __state2 : Nat := (__step2).1; (if Models.Ledger.cappedCheck (__state2) then (if Models.Ledger.growsCheck ((s).2) (__stage1) (__state2) (__stage2) then ((__state1, __state2), Except.ok (__stage2)) else (s, Except.error (((true, false) : Prod Bool Bool)))) else (s, Except.error (((true, true) : Prod Bool Bool))))))) else (s, Except.error (((false, false) : Prod Bool Bool)))) else (s, Except.error (((false, true) : Prod Bool Bool)))))))

@[expose] public def RawChainModel.initial : (Prod (Nat) (Nat)) := Models.Flows.TallySpillRaw.initial
@[expose] public def RawChainModel (__state : (Prod (Nat) (Nat))) (__input : Nat) : (Prod ((Prod (Nat) (Nat))) (Except ((Prod Bool Bool)) (Nat))) := Models.Flows.TallySpillRaw (__state) (__input)

@[expose] public def Drain.initial : Nat := 0
@[expose] public def Drain (s : Nat) (x : Nat) : (Prod (Nat) (Nat)) := ((LexLeanRuntime.subtract (s) (x) : Nat), x)

@[expose] public def DrainModel.initial : Nat := Models.Flows.Drain.initial
@[expose] public def DrainModel (__state : Nat) (__input : Nat) : (Prod (Nat) (Nat)) := Models.Flows.Drain (__state) (__input)

@[expose] public def TallyDrain.initial : (Prod (Nat) (Nat)) := (0, 0)
@[expose] public def TallyDrain (s : (Prod (Nat) (Nat))) (x : Nat) : (Prod ((Prod (Nat) (Nat))) (Except ((Prod Bool Bool)) (Nat))) := (let __step1 : (Prod (Nat) (Nat)) := Models.Ledger.FreeModel ((s).1) (x); (let __stage1 : Nat := (__step1).2; (let __state1 : Nat := (__step1).1; (let __step2 : (Prod (Nat) (Nat)) := Models.Flows.DrainModel ((s).2) (__stage1); (let __stage2 : Nat := (__step2).2; (let __state2 : Nat := (__step2).1; (if Models.Ledger.growsCheck ((s).2) (__stage1) (__state2) (__stage2) then ((__state1, __state2), Except.ok (__stage2)) else (s, Except.error (((true, false) : Prod Bool Bool))))))))))

@[expose] public def DrainChainModel.initial : (Prod (Nat) (Nat)) := Models.Flows.TallyDrain.initial
@[expose] public def DrainChainModel (__state : (Prod (Nat) (Nat))) (__input : Nat) : (Prod ((Prod (Nat) (Nat))) (Except ((Prod Bool Bool)) (Nat))) := Models.Flows.TallyDrain (__state) (__input)

end Models.Flows
