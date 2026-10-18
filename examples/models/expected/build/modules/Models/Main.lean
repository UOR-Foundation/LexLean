module
public import Init
public import LexLeanRuntime.Portable
public import Models.Flows
public import Models.Glyphs
public import Models.Ledger
public import Models.Pipeline
public import Models.Recognizer
public import Models.Session
public import Models.Triage
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Models.Main

@[expose] public def classify (glyph : Models.Glyphs.Glyph) : Nat := Models.Recognizer.DigitModel (glyph)

@[expose] public def classifyChecked (glyph : Models.Glyphs.Glyph) : Except ((Prod Bool Bool)) (Nat) := (let __checked1_input : Models.Glyphs.Glyph := glyph; (let __checked1_output : Nat := Models.Recognizer.RawDigitModel (__checked1_input); (if Models.Recognizer.recognizesCheck (__checked1_input) (__checked1_output) then Except.ok (__checked1_output) else Except.error (((true, false) : Prod Bool Bool)))))

@[expose] public def respond (presentation : Models.Triage.Presentation) : Except ((Prod Bool Bool)) (Nat) := (let __checked1_input : Models.Triage.Presentation := presentation; (if Models.Triage.symptomaticCheck (__checked1_input) then (let __checked1_output : Nat := Models.Pipeline.PipelineModel (__checked1_input); Except.ok (__checked1_output)) else Except.error (((false, false) : Prod Bool Bool))))

@[expose] public def admitCosts (costs : List (Nat)) : Except ((Prod Bool Bool)) (Except ((Prod Bool Bool)) (List (Nat))) := (let __checked1_input : List (Nat) := costs; (if Models.Pipeline.shortStreamCheck (__checked1_input) then (let __checked1_output : Except ((Prod Bool Bool)) (List (Nat)) := Models.Pipeline.StreamModel (__checked1_input); Except.ok (__checked1_output)) else Except.error (((false, false) : Prod Bool Bool))))

@[expose] public def step (window : Models.Session.Window) (cost : Nat) : Except ((Prod Bool Bool)) ((Prod (Models.Session.Window) (Nat))) := (let __checked1_state : Models.Session.Window := window; (let __checked1_input : Nat := cost; (if Models.Session.boundedCheck (__checked1_state) then (if Models.Session.affordableCheck (__checked1_state) (__checked1_input) then (let __checked1_step : (Prod (Models.Session.Window) (Nat)) := Models.Session.SessionModel (__checked1_state) (__checked1_input); Except.ok (__checked1_step)) else Except.error (((false, false) : Prod Bool Bool))) else Except.error (((false, true) : Prod Bool Bool)))))

public theorem classify_eight : (classify (Models.Glyphs.Glyph.g8) = 8) := by
  decide

@[expose] public def admission (outcome : Except ((Prod Bool Bool)) (Except ((Prod Bool Bool)) (List (Nat)))) : Nat := (match outcome with | Except.error _ => 0 | Except.ok scanned => (match scanned with | Except.error _ => 1 | Except.ok _ => 2))

public theorem admit_rejects_costly : (admission (admitCosts ((3 :: (40 :: ([] : List (Nat)))))) = 1) := by
  decide

public theorem admit_accepts_cheap : (admission (admitCosts ((3 :: (16 :: (9 :: ([] : List (Nat))))))) = 2) := by
  decide

@[expose] public def ledgerPost (s : Nat) (x : Nat) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (let __checked1_state : Nat := s; (let __checked1_input : Nat := x; (if Models.Ledger.cappedCheck (__checked1_state) then (if Models.Ledger.smallCheck (__checked1_state) (__checked1_input) then (let __checked1_step : (Prod (Nat) (Nat)) := Models.Ledger.SpillModel (__checked1_state) (__checked1_input); (if Models.Ledger.cappedCheck ((__checked1_step).1) then Except.ok (__checked1_step) else Except.error (((true, true) : Prod Bool Bool)))) else Except.error (((false, false) : Prod Bool Bool))) else Except.error (((false, true) : Prod Bool Bool)))))

@[expose] public def ledgerFull (s : Nat) (x : Nat) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (let __checked1_state : Nat := s; (let __checked1_input : Nat := x; (if Models.Ledger.cappedCheck (__checked1_state) then (if Models.Ledger.smallCheck (__checked1_state) (__checked1_input) then (let __checked1_step : (Prod (Nat) (Nat)) := Models.Ledger.SpillRawModel (__checked1_state) (__checked1_input); (if Models.Ledger.cappedCheck ((__checked1_step).1) then (if Models.Ledger.growsCheck (__checked1_state) (__checked1_input) ((__checked1_step).1) ((__checked1_step).2) then Except.ok (__checked1_step) else Except.error (((true, false) : Prod Bool Bool))) else Except.error (((true, true) : Prod Bool Bool)))) else Except.error (((false, false) : Prod Bool Bool))) else Except.error (((false, true) : Prod Bool Bool)))))

@[expose] public def guessChecked (x : Nat) : Except ((Prod Bool Bool)) (Nat) := (let __checked1_input : Nat := x; (if Models.Ledger.withinCheck (__checked1_input) then (let __checked1_output : Nat := Models.Ledger.GuessModel (__checked1_input); (if Models.Ledger.belowCheck (__checked1_input) (__checked1_output) then Except.ok (__checked1_output) else Except.error (((true, false) : Prod Bool Bool)))) else Except.error (((false, false) : Prod Bool Bool))))

@[expose] public def overshootChecked (x : Nat) : Except ((Prod Bool Bool)) (Nat) := (let __checked1_input : Nat := x; (if Models.Ledger.withinCheck (__checked1_input) then (let __checked1_output : Nat := Models.Ledger.OvershootModel (__checked1_input); (if Models.Ledger.belowCheck (__checked1_input) (__checked1_output) then Except.ok (__checked1_output) else Except.error (((true, false) : Prod Bool Bool)))) else Except.error (((false, false) : Prod Bool Bool))))

@[expose] public def flowStep (s : Nat) (x : Nat) : Except ((Prod Bool Bool)) ((Prod (Nat) (Except ((Prod Bool Bool)) (Nat)))) := (let __checked1_state : Nat := s; (let __checked1_input : Nat := x; (if Models.Ledger.cappedCheck (__checked1_state) then (let __checked1_step : (Prod (Nat) (Except ((Prod Bool Bool)) (Nat))) := Models.Flows.FlowModel (__checked1_state) (__checked1_input); (if Models.Ledger.cappedCheck ((__checked1_step).1) then Except.ok (__checked1_step) else Except.error (((true, true) : Prod Bool Bool)))) else Except.error (((false, true) : Prod Bool Bool)))))

@[expose] public def guessTwice (x : Nat) : Except ((Prod Bool Bool)) (Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) := (let __checked1_input : Nat := x; (if Models.Ledger.withinCheck (__checked1_input) then (let __checked1_output : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := Models.Flows.PairGuessModel (__checked1_input); Except.ok (__checked1_output)) else Except.error (((false, false) : Prod Bool Bool))))

@[expose] public def guessOvershoot (x : Nat) : Except ((Prod Bool Bool)) (Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) := (let __checked1_input : Nat := x; (if Models.Ledger.withinCheck (__checked1_input) then (let __checked1_output : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := Models.Flows.OvershootPairModel (__checked1_input); Except.ok (__checked1_output)) else Except.error (((false, false) : Prod Bool Bool))))

@[expose] public def twinStep (s : (Prod (Nat) (Nat))) (x : Nat) : Except ((Prod Bool Bool)) ((Prod ((Prod (Nat) (Nat))) ((Prod (Nat) (Nat))))) := (let __checked1_state : (Prod (Nat) (Nat)) := s; (let __checked1_input : Nat := x; (if Models.Flows.twinCappedCheck (__checked1_state) then (if Models.Flows.twinSmallCheck (__checked1_state) (__checked1_input) then (let __checked1_step : (Prod ((Prod (Nat) (Nat))) ((Prod (Nat) (Nat)))) := Models.Flows.TwinModel (__checked1_state) (__checked1_input); (if Models.Flows.twinCappedCheck ((__checked1_step).1) then Except.ok (__checked1_step) else Except.error (((true, true) : Prod Bool Bool)))) else Except.error (((false, false) : Prod Bool Bool))) else Except.error (((false, true) : Prod Bool Bool)))))

@[expose] public def pairClamp (s : (Prod (Nat) (Nat))) (x : (Prod (Nat) (Nat))) : Except ((Prod Bool Bool)) ((Prod ((Prod (Nat) (Nat))) ((Prod (Nat) (Nat))))) := (let __checked1_state : (Prod (Nat) (Nat)) := s; (let __checked1_input : (Prod (Nat) (Nat)) := x; (if Models.Flows.bothCappedCheck (__checked1_state) then (let __checked1_step : (Prod ((Prod (Nat) (Nat))) ((Prod (Nat) (Nat)))) := Models.Flows.PairClampModel (__checked1_state) (__checked1_input); (if Models.Flows.bothCappedCheck ((__checked1_step).1) then Except.ok (__checked1_step) else Except.error (((true, true) : Prod Bool Bool)))) else Except.error (((false, true) : Prod Bool Bool)))))

@[expose] public def branchStep (s : (Prod (Nat) (Nat))) (x : Nat) : Except ((Prod Bool Bool)) ((Prod ((Prod (Nat) (Nat))) (Nat))) := (let __checked1_state : (Prod (Nat) (Nat)) := s; (let __checked1_input : Nat := x; (if Models.Flows.twinCappedCheck (__checked1_state) then (if Models.Flows.twinSmallCheck (__checked1_state) (__checked1_input) then (let __checked1_step : (Prod ((Prod (Nat) (Nat))) (Nat)) := Models.Flows.BranchModel (__checked1_state) (__checked1_input); (if Models.Flows.twinCappedCheck ((__checked1_step).1) then Except.ok (__checked1_step) else Except.error (((true, true) : Prod Bool Bool)))) else Except.error (((false, false) : Prod Bool Bool))) else Except.error (((false, true) : Prod Bool Bool)))))

@[expose] public def chainStep (s : (Prod (Nat) (Nat))) (x : Nat) : Except ((Prod Bool Bool)) ((Prod ((Prod (Nat) (Nat))) (Except ((Prod Bool Bool)) (Nat)))) := (let __checked1_state : (Prod (Nat) (Nat)) := s; (let __checked1_input : Nat := x; (if Models.Flows.twinCappedCheck (__checked1_state) then (if Models.Flows.twinSmallCheck (__checked1_state) (__checked1_input) then (let __checked1_step : (Prod ((Prod (Nat) (Nat))) (Except ((Prod Bool Bool)) (Nat))) := Models.Flows.ChainModel (__checked1_state) (__checked1_input); (if Models.Flows.twinCappedCheck ((__checked1_step).1) then Except.ok (__checked1_step) else Except.error (((true, true) : Prod Bool Bool)))) else Except.error (((false, false) : Prod Bool Bool))) else Except.error (((false, true) : Prod Bool Bool)))))

@[expose] public def rawChainStep (s : (Prod (Nat) (Nat))) (x : Nat) : Except ((Prod Bool Bool)) ((Prod ((Prod (Nat) (Nat))) (Except ((Prod Bool Bool)) (Nat)))) := (let __checked1_state : (Prod (Nat) (Nat)) := s; (let __checked1_input : Nat := x; (if Models.Flows.twinCappedCheck (__checked1_state) then (let __checked1_step : (Prod ((Prod (Nat) (Nat))) (Except ((Prod Bool Bool)) (Nat))) := Models.Flows.RawChainModel (__checked1_state) (__checked1_input); (if Models.Flows.twinCappedCheck ((__checked1_step).1) then Except.ok (__checked1_step) else Except.error (((true, true) : Prod Bool Bool)))) else Except.error (((false, true) : Prod Bool Bool)))))

@[expose] public def drainChainStep (s : (Prod (Nat) (Nat))) (x : Nat) : Except ((Prod Bool Bool)) ((Prod ((Prod (Nat) (Nat))) (Except ((Prod Bool Bool)) (Nat)))) := (let __checked1_state : (Prod (Nat) (Nat)) := s; (let __checked1_input : Nat := x; (if Models.Flows.twinCappedCheck (__checked1_state) then (let __checked1_step : (Prod ((Prod (Nat) (Nat))) (Except ((Prod Bool Bool)) (Nat))) := Models.Flows.DrainChainModel (__checked1_state) (__checked1_input); (if Models.Flows.twinCappedCheck ((__checked1_step).1) then Except.ok (__checked1_step) else Except.error (((true, true) : Prod Bool Bool)))) else Except.error (((false, true) : Prod Bool Bool)))))

@[expose] public def spillStream (xs : List (Nat)) : Except ((Prod Bool Bool)) (Except ((Prod Bool Bool)) (List (Nat))) := (let __checked1_input : List (Nat) := xs; (if Models.Flows.fewCheck (__checked1_input) then (let __checked1_output : Except ((Prod Bool Bool)) (List (Nat)) := Models.Flows.SpillStreamModel (__checked1_input); Except.ok (__checked1_output)) else Except.error (((false, false) : Prod Bool Bool))))

@[expose] public def stepCode (outcome : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) : Nat := (match outcome with | Except.error violation => Models.Ledger.violationCode (violation) | Except.ok pair => (100 + (pair).1))

@[expose] public def valueCode (outcome : Except ((Prod Bool Bool)) (Nat)) : Nat := (match outcome with | Except.error violation => Models.Ledger.violationCode (violation) | Except.ok value => (100 + value))

@[expose] public def flowCode (outcome : Except ((Prod Bool Bool)) ((Prod (Nat) (Except ((Prod Bool Bool)) (Nat))))) : Nat := (match outcome with | Except.error violation => Models.Ledger.violationCode (violation) | Except.ok pair => (1000 + ((LexLeanRuntime.multiply ((pair).1) (1000) : Nat) + valueCode ((pair).2))))

@[expose] public def quadCode (outcome : Except ((Prod Bool Bool)) ((Prod ((Prod (Nat) (Nat))) ((Prod (Nat) (Nat)))))) : Nat := (match outcome with | Except.error violation => Models.Ledger.violationCode (violation) | Except.ok p => ((LexLeanRuntime.multiply (((p).1).1) (1000000000) : Nat) + ((LexLeanRuntime.multiply (((p).1).2) (1000000) : Nat) + ((LexLeanRuntime.multiply (((p).2).1) (1000) : Nat) + ((p).2).2))))

@[expose] public def branchCode (outcome : Except ((Prod Bool Bool)) ((Prod ((Prod (Nat) (Nat))) (Nat)))) : Nat := (match outcome with | Except.error violation => Models.Ledger.violationCode (violation) | Except.ok p => ((LexLeanRuntime.multiply (((p).1).1) (1000000) : Nat) + ((LexLeanRuntime.multiply (((p).1).2) (1000) : Nat) + (p).2)))

@[expose] public def chainCode (outcome : Except ((Prod Bool Bool)) ((Prod ((Prod (Nat) (Nat))) (Except ((Prod Bool Bool)) (Nat))))) : Nat := (match outcome with | Except.error violation => Models.Ledger.violationCode (violation) | Except.ok p => ((LexLeanRuntime.multiply (((p).1).1) (1000000) : Nat) + ((LexLeanRuntime.multiply (((p).1).2) (1000) : Nat) + valueCode ((p).2))))

@[expose] public def twiceCode (outcome : Except ((Prod Bool Bool)) (Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))))) : Nat := (match outcome with | Except.error violation => Models.Ledger.violationCode (violation) | Except.ok inner => (match inner with | Except.error violation => (10 + Models.Ledger.violationCode (violation)) | Except.ok values => (100 + ((LexLeanRuntime.multiply ((values).1) (10) : Nat) + (values).2))))

@[expose] public def streamCode (outcome : Except ((Prod Bool Bool)) (Except ((Prod Bool Bool)) (List (Nat)))) : Nat := (match outcome with | Except.error violation => Models.Ledger.violationCode (violation) | Except.ok inner => (match inner with | Except.error violation => (10 + Models.Ledger.violationCode (violation)) | Except.ok values => (100 + (LexLeanRuntime.length (values) : Nat))))

public theorem ledger_accepts : (stepCode (ledgerPost (5) (3)) = 108) := by
  decide

public theorem ledger_refuses_input : (stepCode (ledgerPost (200) (1)) = 2) := by
  decide

public theorem ledger_refuses_precondition : (stepCode (ledgerPost (5) (20)) = 1) := by
  decide

public theorem ledger_refuses_output : (stepCode (ledgerPost (95) (9)) = 4) := by
  decide

public theorem ledger_full_accepts : (stepCode (ledgerFull (5) (3)) = 108) := by
  decide

public theorem ledger_full_refuses_output : (stepCode (ledgerFull (95) (9)) = 4) := by
  decide

public theorem guess_accepts : (valueCode (guessChecked (5)) = 104) := by
  decide

public theorem guess_refuses_precondition : (valueCode (guessChecked (20)) = 1) := by
  decide

public theorem overshoot_refuses_postcondition : (valueCode (overshootChecked (3)) = 3) := by
  decide

public theorem flow_accepts : (flowCode (flowStep (5) (3)) = 9107) := by
  decide

public theorem flow_refuses_junction : (flowCode (flowStep (5) (30)) = 6001) := by
  decide

public theorem flow_refuses_input : (flowCode (flowStep (200) (1)) = 2) := by
  decide

public theorem twice_accepts : (twiceCode (guessTwice (4)) = 133) := by
  decide

public theorem twice_refuses : (twiceCode (guessTwice (40)) = 1) := by
  decide

public theorem overshoot_pair_refuses_postcondition : (twiceCode (guessOvershoot (4)) = 13) := by
  decide

public theorem twin_accepts : (quadCode (twinStep ((5, 7)) (3)) = 8010008007) := by
  decide

public theorem twin_refuses_precondition : (quadCode (twinStep ((5, 7)) (30)) = 1) := by
  decide

public theorem twin_refuses_input : (quadCode (twinStep ((200, 7)) (3)) = 2) := by
  decide

public theorem pair_clamp_accepts : (quadCode (pairClamp ((5, 90)) ((3, 20))) = 8100008100) := by
  decide

public theorem pair_clamp_refuses_input : (quadCode (pairClamp ((5, 200)) ((1, 1))) = 2) := by
  decide

public theorem branch_takes_then : (branchCode (branchStep ((5, 7)) (3)) = 8007008) := by
  decide

public theorem branch_takes_else : (branchCode (branchStep ((5, 7)) (8)) = 5015007) := by
  decide

public theorem branch_refuses_precondition : (branchCode (branchStep ((5, 7)) (20)) = 1) := by
  decide

public theorem chain_accepts : (chainCode (chainStep ((5, 7)) (3)) = 8010107) := by
  decide

public theorem chain_refuses_stage_output : (chainCode (chainStep ((95, 7)) (9)) = 95007004) := by
  decide

public theorem chain_refuses_precondition : (chainCode (chainStep ((5, 7)) (30)) = 1) := by
  decide

public theorem raw_chain_accepts : (chainCode (rawChainStep ((4, 20)) (3)) = 7024104) := by
  decide

public theorem raw_chain_refuses_input : (chainCode (rawChainStep ((4, 150)) (3)) = 4150002) := by
  decide

public theorem raw_chain_refuses_precondition : (chainCode (rawChainStep ((12, 20)) (3)) = 12020001) := by
  decide

public theorem drain_chain_accepts : (chainCode (drainChainStep ((0, 20)) (3)) = 3020100) := by
  decide

public theorem drain_chain_refuses_postcondition : (chainCode (drainChainStep ((4, 20)) (3)) = 4020003) := by
  decide

public theorem stream_accepts : (streamCode (spillStream ((10 :: (10 :: (10 :: (10 :: (10 :: (10 :: (10 :: (10 :: (10 :: (10 :: ([] : List (Nat)))))))))))))) = 110) := by
  decide

public theorem stream_refuses_output : (streamCode (spillStream ((10 :: (10 :: (10 :: (10 :: (10 :: (10 :: (10 :: (10 :: (10 :: (10 :: (10 :: ([] : List (Nat))))))))))))))) = 14) := by
  decide

@[expose] public def deriveScaleMultiplier (inputScaleBits : Int) (weightScaleBits : Int) (outputScaleBits : Int) : Option (Int) := Models.Recognizer.scaleRatioRound32 (inputScaleBits) (weightScaleBits) (outputScaleBits)

@[expose] public def deriveChannelMultipliers (inputScaleBits : Int) (weightScaleBits : List (Int)) (outputScaleBits : Int) : Option (List (Int)) := Models.Recognizer.scaleMultipliersRound32 (inputScaleBits) (weightScaleBits) (outputScaleBits)

public theorem corpus_scale_ratio_rounds_up : (deriveScaleMultiplier ((998277249 : Int)) ((1000693826 : Int)) ((1017370378 : Int)) = Option.some ((4250229 : Int))) := by
  decide

public theorem corpus_scale_ratio_rounds_down : (deriveScaleMultiplier ((1023410176 : Int)) ((1017370378 : Int)) ((1039516303 : Int)) = Option.some ((22369621 : Int))) := by
  decide

public theorem corpus_scale_ratio_tie_even : ((deriveScaleMultiplier ((1065353217 : Int)) ((1069547520 : Int)) ((1140850688 : Int)) = Option.some ((12582914 : Int))) /\ (deriveScaleMultiplier ((1065353219 : Int)) ((1069547520 : Int)) ((1140850688 : Int)) = Option.some ((12582916 : Int)))) := by
  decide

public theorem corpus_scale_multipliers : (deriveChannelMultipliers ((998277249 : Int)) (((1000593162 : Int) :: ((1000693826 : Int) :: ([] : List (Int))))) ((1017370378 : Int)) = Option.some (((4210752 : Int) :: ((4250229 : Int) :: ([] : List (Int)))))) := by
  decide

end Models.Main
