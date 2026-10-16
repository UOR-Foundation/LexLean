module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Models
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Models.Session

public structure Window where
  used : Nat
  items : Nat

@[expose, reducible] public def Affordable (_window : Window) (cost : Nat) : Prop := (cost <= 16)

@[expose] public def affordableCheck (_window : Window) (cost : Nat) : Bool := (Nat.ble (cost) (16))

public theorem affordable_check_sound (window : Window) (cost : Nat) : ((affordableCheck (window) (cost) = true) -> Affordable (window) (cost)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Affordable, affordableCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose, reducible] public def Bounded (window : Window) : Prop := ((window).used <= 64)

@[expose] public def boundedCheck (window : Window) : Bool := (Nat.ble ((window).used) (64))

public theorem bounded_check_sound (window : Window) : ((boundedCheck (window) = true) -> Bounded (window)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Bounded, boundedCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose, reducible] public def Conserves (window : Window) (cost : Nat) (updated : Window) (overflow : Nat) : Prop := (((updated).used + overflow) = ((window).used + cost))

public theorem SessionContract.invariant_sound : LexLeanModels.Sound1 ((Models.Session.boundedCheck)) ((Models.Session.Bounded)) := Models.Session.bounded_check_sound
public theorem SessionContract.precondition_sound : LexLeanModels.Sound2 ((Models.Session.affordableCheck)) ((Models.Session.Affordable)) := Models.Session.affordable_check_sound

@[expose] public def SessionStep.initial : Models.Session.Window := ({ used := 0, items := 0 } : Models.Session.Window)
@[expose] public def SessionStep (window : Models.Session.Window) (cost : Nat) : (Prod (Models.Session.Window) (Nat)) := (let total : Nat := ((window).used + cost); (({ used := (LexLeanRuntime.subtract (total) ((LexLeanRuntime.subtract (total) (64) : Nat)) : Nat), items := ((window).items + 1) } : Models.Session.Window), (LexLeanRuntime.subtract (total) (64) : Nat)))

public theorem session_initial : Bounded (SessionStep.initial) := by
  decide

public theorem session_preserves (window : Window) (cost : Nat) : (Bounded (window) -> (Affordable (window) (cost) -> Bounded ((SessionStep (window) (cost)).1))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Affordable, Bounded, SessionStep, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem session_conserves (window : Window) (cost : Nat) : (Bounded (window) -> (Affordable (window) (cost) -> Conserves (window) (cost) ((SessionStep (window) (cost)).1) ((SessionStep (window) (cost)).2))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Affordable, Bounded, Conserves, SessionStep, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem SessionEvidence.session_initial : LexLeanModels.Initial ((Models.Session.Bounded)) (Models.Session.SessionStep.initial) := Models.Session.session_initial
public theorem SessionEvidence.session_preserves : LexLeanModels.Preserves ((Models.Session.Bounded)) ((Models.Session.Affordable)) ((Models.Session.SessionStep)) := Models.Session.session_preserves
public theorem SessionEvidence.session_conserves : LexLeanModels.SatisfiesStep ((Models.Session.Bounded)) ((Models.Session.Affordable)) ((Models.Session.Conserves)) ((Models.Session.SessionStep)) := Models.Session.session_conserves

@[expose] public def SessionModel.initial : Models.Session.Window := Models.Session.SessionStep.initial
@[expose] public def SessionModel (__state : Models.Session.Window) (__input : Nat) : (Prod (Models.Session.Window) (Nat)) := Models.Session.SessionStep (__state) (__input)

end Models.Session
