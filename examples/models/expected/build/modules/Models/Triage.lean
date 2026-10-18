module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanRuntime.Models
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Models.Triage

public inductive Presentation where
  | p0
  | p1
  | p2
  | p3
  | p4
  | p5
  | p6
  | p7

public inductive Level where
  | routine
  | urgent
  | emergency

@[expose] public def features (presentation : Presentation) : List (Int) := (match presentation with | Presentation.p0 => ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int))))) | Presentation.p1 => ((1 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int))))) | Presentation.p2 => ((0 : Int) :: ((1 : Int) :: ((0 : Int) :: ([] : List (Int))))) | Presentation.p3 => ((1 : Int) :: ((1 : Int) :: ((0 : Int) :: ([] : List (Int))))) | Presentation.p4 => ((0 : Int) :: ((0 : Int) :: ((1 : Int) :: ([] : List (Int))))) | Presentation.p5 => ((1 : Int) :: ((0 : Int) :: ((1 : Int) :: ([] : List (Int))))) | Presentation.p6 => ((0 : Int) :: ((1 : Int) :: ((1 : Int) :: ([] : List (Int))))) | Presentation.p7 => ((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ([] : List (Int))))))

@[expose] public def hasSymptom (presentation : Presentation) : Bool := (match presentation with | Presentation.p0 => false | Presentation.p1 => true | Presentation.p2 => true | Presentation.p3 => true | Presentation.p4 => true | Presentation.p5 => true | Presentation.p6 => true | Presentation.p7 => true)

@[expose] public def isBreathless (presentation : Presentation) : Bool := (match presentation with | Presentation.p0 => false | Presentation.p1 => false | Presentation.p2 => false | Presentation.p3 => false | Presentation.p4 => true | Presentation.p5 => true | Presentation.p6 => true | Presentation.p7 => true)

@[expose] public def isEmergency (level : Level) : Bool := (match level with | Level.routine => false | Level.urgent => false | Level.emergency => true)

@[expose] public def isRoutine (level : Level) : Bool := (match level with | Level.routine => true | Level.urgent => false | Level.emergency => false)

@[expose, reducible] public def Symptomatic (presentation : Presentation) : Prop := (hasSymptom (presentation) = true)

@[expose] public def symptomaticCheck (presentation : Presentation) : Bool := hasSymptom (presentation)

public theorem symptomatic_check_sound (presentation : Presentation) : ((symptomaticCheck (presentation) = true) -> Symptomatic (presentation)) := by
  cases presentation with
  | p0 =>
    decide
  | p1 =>
    decide
  | p2 =>
    decide
  | p3 =>
    decide
  | p4 =>
    decide
  | p5 =>
    decide
  | p6 =>
    decide
  | p7 =>
    decide

@[expose, reducible] public def Safe (presentation : Presentation) (level : Level) : Prop := ((isBreathless (presentation) = true) -> (isEmergency (level) = true))

@[expose] public def triageWeights.bytes : ByteArray := (let __byte0 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 0); _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 254)), (_root_.UInt8.ofNat (nat_lit 255)), (_root_.UInt8.ofNat (nat_lit 254)), (_root_.UInt8.ofNat (nat_lit 255)), (_root_.UInt8.ofNat (nat_lit 252)), (_root_.UInt8.ofNat (nat_lit 255)), (_root_.UInt8.ofNat (nat_lit 2)), __byte0, (_root_.UInt8.ofNat (nat_lit 2)), __byte0, __byte0, __byte0, __byte0, __byte0, (_root_.UInt8.ofNat (nat_lit 1)), __byte0, (_root_.UInt8.ofNat (nat_lit 5)), __byte0])
@[expose] public def triageWeights : List (List (Int)) := (((-2 : Int) :: ((-2 : Int) :: ((-4 : Int) :: ([] : List (Int))))) :: (((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ([] : List (Int))))) :: (((0 : Int) :: ((1 : Int) :: ((5 : Int) :: ([] : List (Int))))) :: ([] : List (List (Int))))))
public theorem triageWeights.decoded : (LexLeanModels.tensorMatches 2 true triageWeights.flatten triageWeights.bytes && (triageWeights.length == 3) && (triageWeights.map List.length == List.replicate 3 3)) = true := by decide

@[expose] public def triageBias.bytes : ByteArray := _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 1)), (_root_.UInt8.ofNat (nat_lit 0)), (_root_.UInt8.ofNat (nat_lit 0)), (_root_.UInt8.ofNat (nat_lit 0)), (_root_.UInt8.ofNat (nat_lit 255)), (_root_.UInt8.ofNat (nat_lit 255))]
@[expose] public def triageBias : List (Int) := ((1 : Int) :: ((0 : Int) :: ((-1 : Int) :: ([] : List (Int)))))
public theorem triageBias.decoded : (LexLeanModels.tensorMatches 2 true triageBias triageBias.bytes && (triageBias.length == 3)) = true := by decide

public theorem TriageContract.precondition_sound : LexLeanModels.Sound1 ((Models.Triage.symptomaticCheck)) ((Models.Triage.Symptomatic)) := Models.Triage.symptomatic_check_sound

public theorem triage_width (presentation : Presentation) : ((LexLeanRuntime.length (features (presentation)) : Nat) = 3) := by
  cases presentation with
  | p0 =>
    rfl
  | p1 =>
    rfl
  | p2 =>
    rfl
  | p3 =>
    rfl
  | p4 =>
    rfl
  | p5 =>
    rfl
  | p6 =>
    rfl
  | p7 =>
    rfl

@[expose] public def TriageScore.features (presentation : Models.Triage.Presentation) : List (Int) := Models.Triage.features (presentation)
@[expose] public def TriageScore.scores (presentation : Models.Triage.Presentation) : List (Int) := (let __values : List (Int) := Models.Triage.TriageScore.features (presentation); ((LexLeanCollections.listFold ((fun (__acc : (Prod (List (Int)) (List (Int)))) (__row : List (Int)) => (match (__acc).2 with | List.nil => __acc | List.cons __bias __biases => ((LexLeanRuntime.append ((__acc).1) (((LexLeanRuntime.subtract (__bias) ((LexLeanRuntime.negate (((LexLeanCollections.listFold ((fun (__sum : (Prod (Int) (List (Int)))) (__weight : Int) => (match (__sum).2 with | List.nil => __sum | List.cons __value __rest => ((LexLeanRuntime.subtract ((__sum).1) ((LexLeanRuntime.negate ((LexLeanRuntime.multiply (__weight) (__value) : Int)) : Int)) : Int), __rest)))) (((0 : Int), __values)) (__row) : (Prod (Int) (List (Int))))).1) : Int)) : Int) :: ([] : List (Int)))) : List (Int)), __biases)))) ((([] : List (Int)), Models.Triage.triageBias)) (Models.Triage.triageWeights) : (Prod (List (Int)) (List (Int))))).1)
@[expose] public def TriageScore (presentation : Models.Triage.Presentation) : Models.Triage.Level := (match ((LexLeanCollections.listFold ((fun (__best : (Prod (Option ((Prod (Models.Triage.Level) (Int)))) (List (Models.Triage.Level)))) (__score : Int) => (match (__best).2 with | List.nil => __best | List.cons __label __labels => ((match (__best).1 with | Option.none => Option.some ((__label, __score)) | Option.some __leader => (if (LexLeanCollections.lessThan ((__leader).2) (__score) : Bool) then Option.some ((__label, __score)) else Option.some (__leader))), __labels)))) ((Option.none, (Models.Triage.Level.routine :: (Models.Triage.Level.urgent :: (Models.Triage.Level.emergency :: ([] : List (Models.Triage.Level))))))) (Models.Triage.TriageScore.scores (presentation)) : (Prod (Option ((Prod (Models.Triage.Level) (Int)))) (List (Models.Triage.Level))))).1 with | Option.none => Models.Triage.Level.routine | Option.some __winner => (__winner).1)

public theorem triage_safe (presentation : Presentation) : (Symptomatic (presentation) -> Safe (presentation) (TriageScore (presentation))) := by
  cases presentation with
  | p0 =>
    decide
  | p1 =>
    decide
  | p2 =>
    decide
  | p3 =>
    decide
  | p4 =>
    decide
  | p5 =>
    decide
  | p6 =>
    decide
  | p7 =>
    decide

public theorem TriageEvidence.triage_safe : LexLeanModels.Satisfies ((Models.Triage.Symptomatic)) ((Models.Triage.Safe)) ((Models.Triage.TriageScore)) := Models.Triage.triage_safe

@[expose] public def TriageModel (__input : Models.Triage.Presentation) : Models.Triage.Level := Models.Triage.TriageScore (__input)

end Models.Triage
