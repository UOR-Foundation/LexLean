module
public import Init
public import LexLeanRuntime.Models
public import Models.Triage
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Models.Policy

@[expose] public def isUrgent (level : Models.Triage.Level) : Bool := (match level with | Models.Triage.Level.routine => false | Models.Triage.Level.urgent => true | Models.Triage.Level.emergency => false)

@[expose, reducible] public def Actionable (level : Models.Triage.Level) : Prop := (Models.Triage.isRoutine (level) = false)

@[expose] public def actionableCheck (level : Models.Triage.Level) : Bool := (!Models.Triage.isRoutine (level))

public theorem actionable_check_sound (level : Models.Triage.Level) : ((actionableCheck (level) = true) -> Actionable (level)) := by
  cases level with
  | routine =>
    decide
  | urgent =>
    decide
  | emergency =>
    decide

public theorem actionable_check_complete (level : Models.Triage.Level) : (Actionable (level) -> (actionableCheck (level) = true)) := by
  cases level with
  | routine =>
    decide
  | urgent =>
    decide
  | emergency =>
    decide

@[expose, reducible] public def Escalates (_level : Models.Triage.Level) (action : Nat) : Prop := (1 <= action)

public theorem PolicyContract.precondition_sound : LexLeanModels.Sound1 ((Models.Policy.actionableCheck)) ((Models.Policy.Actionable)) := Models.Policy.actionable_check_sound
public theorem PolicyContract.precondition_complete : LexLeanModels.Complete1 ((Models.Policy.actionableCheck)) ((Models.Policy.Actionable)) := Models.Policy.actionable_check_complete

@[expose] public def PolicyRules.selected (level : Models.Triage.Level) : Nat := (if Models.Triage.isEmergency (level) then 0 else (if Models.Policy.isUrgent (level) then 1 else 2))
@[expose] public def PolicyRules (level : Models.Triage.Level) : Nat := (if Models.Triage.isEmergency (level) then 2 else (if Models.Policy.isUrgent (level) then 1 else 0))

public theorem policy_escalates (level : Models.Triage.Level) : (Actionable (level) -> Escalates (level) (PolicyRules (level))) := by
  cases level with
  | routine =>
    decide
  | urgent =>
    decide
  | emergency =>
    decide

public theorem emergency_dispatches : (PolicyRules.selected (Models.Triage.Level.emergency) = 0) := by
  decide

public theorem PolicyEvidence.policy_escalates : LexLeanModels.Satisfies ((Models.Policy.Actionable)) ((Models.Policy.Escalates)) ((Models.Policy.PolicyRules)) := Models.Policy.policy_escalates

@[expose] public def PolicyModel (__input : Models.Triage.Level) : Nat := Models.Policy.PolicyRules (__input)

end Models.Policy
