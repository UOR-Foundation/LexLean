module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanRuntime.Models
public import Models.Glyphs
public import Models.Policy
public import Models.Recognizer
public import Models.Session
public import Models.Triage
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Models.Pipeline

@[expose, reducible] public def Responds (_presentation : Models.Triage.Presentation) (action : Nat) : Prop := (1 <= action)

public theorem symptomatic_sound (presentation : Models.Triage.Presentation) : ((Models.Triage.symptomaticCheck (presentation) = true) -> Models.Triage.Symptomatic (presentation)) := by
  exact Models.Triage.symptomatic_check_sound (presentation)

public theorem PipelineContract.precondition_sound : LexLeanModels.Sound1 ((Models.Triage.symptomaticCheck)) ((Models.Triage.Symptomatic)) := Models.Pipeline.symptomatic_sound

public theorem triage_actionable (presentation : Models.Triage.Presentation) : (Models.Triage.Symptomatic (presentation) -> Models.Policy.Actionable (Models.Triage.TriageModel (presentation))) := by
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

@[expose] public def TriagePipeline (presentation : Models.Triage.Presentation) : Nat := (let __stage1 : Models.Triage.Level := Models.Triage.TriageModel (presentation); (let __stage2 : Nat := Models.Policy.PolicyModel (__stage1); __stage2))

public theorem pipeline_entry (presentation : Models.Triage.Presentation) : (Models.Triage.Symptomatic (presentation) -> Models.Triage.Symptomatic (presentation)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Models.Triage.Symptomatic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem pipeline_responds (presentation : Models.Triage.Presentation) : (Models.Triage.Symptomatic (presentation) -> Responds (presentation) (TriagePipeline (presentation))) := by
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

public theorem PipelineEvidence.pipeline_responds : LexLeanModels.Satisfies ((Models.Triage.Symptomatic)) ((Models.Pipeline.Responds)) ((Models.Pipeline.TriagePipeline)) := Models.Pipeline.pipeline_responds

@[expose] public def PipelineModel (__input : Models.Triage.Presentation) : Nat := Models.Pipeline.TriagePipeline (__input)

@[expose] public def resultCode (outcome : Except ((Prod Bool Bool)) (Nat)) : Nat := (match outcome with | Except.error _ => 0 | Except.ok action => action)

@[expose, reducible] public def Answered (_presentation : Models.Triage.Presentation) (outcome : Except ((Prod Bool Bool)) (Nat)) : Prop := (resultCode (outcome) <= 2)


@[expose] public def CheckedPipeline (presentation : Models.Triage.Presentation) : Except ((Prod Bool Bool)) (Nat) := (let __stage1 : Models.Triage.Level := Models.Triage.TriageModel (presentation); (if Models.Policy.actionableCheck (__stage1) then (let __stage2 : Nat := Models.Policy.PolicyModel (__stage1); Except.ok (__stage2)) else Except.error (((false, false) : Prod Bool Bool))))

public theorem checked_answers (presentation : Models.Triage.Presentation) : (Models.Triage.Symptomatic (presentation) -> Answered (presentation) (CheckedPipeline (presentation))) := by
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

public theorem CheckedEvidence.checked_answers : LexLeanModels.Satisfies ((Models.Triage.Symptomatic)) ((Models.Pipeline.Answered)) ((Models.Pipeline.CheckedPipeline)) := Models.Pipeline.checked_answers

@[expose] public def CheckedModel (__input : Models.Triage.Presentation) : Except ((Prod Bool Bool)) (Nat) := Models.Pipeline.CheckedPipeline (__input)

@[expose, reducible] public def Lit (_glyph : Models.Glyphs.Glyph) (segments : Nat) : Prop := (segments <= 7)


@[expose] public def SegmentCounter (glyph : Models.Glyphs.Glyph) : Nat := Models.Glyphs.segmentCount (glyph)

public theorem segments_lit (glyph : Models.Glyphs.Glyph) : Lit (glyph) (SegmentCounter (glyph)) := by
  cases glyph with
  | g0 =>
    decide
  | g1 =>
    decide
  | g2 =>
    decide
  | g3 =>
    decide
  | g4 =>
    decide
  | g5 =>
    decide
  | g6 =>
    decide
  | g7 =>
    decide
  | g8 =>
    decide
  | g9 =>
    decide

public theorem SegmentEvidence.segments_lit : LexLeanModels.SatisfiesTotal ((Models.Pipeline.Lit)) ((Models.Pipeline.SegmentCounter)) := Models.Pipeline.segments_lit

@[expose] public def SegmentModel (__input : Models.Glyphs.Glyph) : Nat := Models.Pipeline.SegmentCounter (__input)

@[expose, reducible] public def Reported (glyph : Models.Glyphs.Glyph) (report : (Prod (Nat) (Nat))) : Prop := (Models.Recognizer.Recognizes (glyph) ((report).1) /\ ((report).2 <= 7))


@[expose] public def GlyphReport (glyph : Models.Glyphs.Glyph) : (Prod (Nat) (Nat)) := (Models.Recognizer.DigitModel (glyph), Models.Pipeline.SegmentModel (glyph))

public theorem report_correct (glyph : Models.Glyphs.Glyph) : Reported (glyph) (GlyphReport (glyph)) := by
  cases glyph with
  | g0 =>
    decide
  | g1 =>
    decide
  | g2 =>
    decide
  | g3 =>
    decide
  | g4 =>
    decide
  | g5 =>
    decide
  | g6 =>
    decide
  | g7 =>
    decide
  | g8 =>
    decide
  | g9 =>
    decide

public theorem ReportEvidence.report_correct : LexLeanModels.SatisfiesTotal ((Models.Pipeline.Reported)) ((Models.Pipeline.GlyphReport)) := Models.Pipeline.report_correct

@[expose] public def ReportModel (__input : Models.Glyphs.Glyph) : (Prod (Nat) (Nat)) := Models.Pipeline.GlyphReport (__input)

@[expose, reducible] public def PairLit (glyphs : (Prod (Models.Glyphs.Glyph) (Models.Glyphs.Glyph))) (counts : (Prod (Nat) (Nat))) : Prop := (Lit ((glyphs).1) ((counts).1) /\ Lit ((glyphs).2) ((counts).2))


@[expose] public def GlyphPair (glyphs : (Prod (Models.Glyphs.Glyph) (Models.Glyphs.Glyph))) : (Prod (Nat) (Nat)) := (Models.Pipeline.SegmentModel ((glyphs).1), Models.Pipeline.SegmentModel ((glyphs).2))

public theorem pair_correct (glyphs : (Prod (Models.Glyphs.Glyph) (Models.Glyphs.Glyph))) : PairLit (glyphs) (GlyphPair (glyphs)) := by
  constructor
  ·
    exact segments_lit ((glyphs).1)
  ·
    exact segments_lit ((glyphs).2)

public theorem PairEvidence.pair_correct : LexLeanModels.SatisfiesTotal ((Models.Pipeline.PairLit)) ((Models.Pipeline.GlyphPair)) := Models.Pipeline.pair_correct

@[expose] public def PairModel (__input : (Prod (Models.Glyphs.Glyph) (Models.Glyphs.Glyph))) : (Prod (Nat) (Nat)) := Models.Pipeline.GlyphPair (__input)

@[expose, reducible] public def Bounded9 (_glyph : Models.Glyphs.Glyph) (value : Nat) : Prop := (value <= 9)


@[expose] public def GlyphRoute (glyph : Models.Glyphs.Glyph) : Nat := (if Models.Glyphs.isEight (glyph) then Models.Pipeline.SegmentModel (glyph) else Models.Recognizer.DigitModel (glyph))

public theorem route_bounded (glyph : Models.Glyphs.Glyph) : Bounded9 (glyph) (GlyphRoute (glyph)) := by
  cases glyph with
  | g0 =>
    decide
  | g1 =>
    decide
  | g2 =>
    decide
  | g3 =>
    decide
  | g4 =>
    decide
  | g5 =>
    decide
  | g6 =>
    decide
  | g7 =>
    decide
  | g8 =>
    decide
  | g9 =>
    decide

public theorem RouteEvidence.route_bounded : LexLeanModels.SatisfiesTotal ((Models.Pipeline.Bounded9)) ((Models.Pipeline.GlyphRoute)) := Models.Pipeline.route_bounded

@[expose] public def RouteModel (__input : Models.Glyphs.Glyph) : Nat := Models.Pipeline.GlyphRoute (__input)

@[expose, reducible] public def ShortStream (costs : List (Nat)) : Prop := ((LexLeanRuntime.length (costs) : Nat) <= 1000)

@[expose] public def shortStreamCheck (costs : List (Nat)) : Bool := (Nat.blt ((LexLeanRuntime.length (costs) : Nat)) (1001))

public theorem short_stream_check_sound (costs : List (Nat)) : ((shortStreamCheck (costs) = true) -> ShortStream (costs)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [ShortStream, shortStreamCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem StreamContract.precondition_sound : LexLeanModels.Sound1 ((Models.Pipeline.shortStreamCheck)) ((Models.Pipeline.ShortStream)) := Models.Pipeline.short_stream_check_sound

@[expose] public def SessionStream (costs : List (Nat)) : Except ((Prod Bool Bool)) (List (Nat)) := (let __scanned : Except ((Prod Bool Bool)) ((Prod (Models.Session.Window) (List (Nat)))) := (LexLeanCollections.listFold ((fun (__acc : Except ((Prod Bool Bool)) ((Prod (Models.Session.Window) (List (Nat))))) (__item : Nat) => (match __acc with | Except.error _ => __acc | Except.ok __state => (if Models.Session.boundedCheck ((__state).1) then (if Models.Session.affordableCheck ((__state).1) (__item) then (let __step : (Prod (Models.Session.Window) (Nat)) := Models.Session.SessionModel ((__state).1) (__item); Except.ok (((__step).1, (LexLeanRuntime.append ((__state).2) (((__step).2 :: ([] : List (Nat)))) : List (Nat))))) else Except.error (((false, false) : Prod Bool Bool))) else Except.error (((false, true) : Prod Bool Bool)))))) (Except.ok ((Models.Session.SessionModel.initial, ([] : List (Nat))))) (costs) : Except ((Prod Bool Bool)) ((Prod (Models.Session.Window) (List (Nat))))); (match __scanned with | Except.error __violation => Except.error (__violation) | Except.ok __state => Except.ok ((__state).2)))

@[expose] public def StreamModel (__input : List (Nat)) : Except ((Prod Bool Bool)) (List (Nat)) := Models.Pipeline.SessionStream (__input)

end Models.Pipeline
