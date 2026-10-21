module
public import Init
public import LexLeanStd.Text
public import LexLeanStdModels.Scores
public import Helpdesk.Reply
public import Helpdesk.Router
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Helpdesk.Main

@[expose] public def threshold : Int := (2 : Int)

@[expose] public def committed (seen : Option (Nat)) : Bool := (match seen with | Option.some _ => true | Option.none => false)

@[expose] public def classify (request : ByteArray) : Nat := (match (let __checked1_input : ByteArray := LexLeanStd.Text.trim (request); (if Helpdesk.Router.readableCheck (__checked1_input) then (let __checked1_output : List (Int) := Helpdesk.Router.ScoringModel (__checked1_input); Except.ok (__checked1_output)) else Except.error (((false, false) : Prod Bool Bool)))) with | Except.ok scores => (match LexLeanStdModels.Scores.decideOr (threshold) (scores) with | Option.some index => index | Option.none => 8) | Except.error _ => 9)

@[expose] public def plan (code : Nat) : Nat := Helpdesk.Reply.stepsFor (Helpdesk.Router.topicAt (code))

@[expose] public def dispatch (request : ByteArray) : ByteArray := (match (let __checked1_input : ByteArray := LexLeanStd.Text.trim (request); (if Helpdesk.Router.readableCheck (__checked1_input) then (let __checked1_output : List (Int) := Helpdesk.Router.ScoringModel (__checked1_input); Except.ok (__checked1_output)) else Except.error (((false, false) : Prod Bool Bool)))) with | Except.ok scores => (let topic : Helpdesk.Router.Topic := Helpdesk.Router.topicAt (LexLeanStdModels.Scores.best (scores)); (match LexLeanStdModels.Scores.GateModel (Helpdesk.Router.Topic) ((if committed (LexLeanStdModels.Scores.decideOr (threshold) (scores)) then LexLeanStdModels.Scores.Candidate.confident (topic) else LexLeanStdModels.Scores.Candidate.unsure (topic))) with | Option.some answered => Helpdesk.Reply.message (answered) | Option.none => _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 104)), (_root_.UInt8.ofNat (nat_lit 117)), (_root_.UInt8.ofNat (nat_lit 109)), (_root_.UInt8.ofNat (nat_lit 97)), (_root_.UInt8.ofNat (nat_lit 110))])) | Except.error _ => _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 114)), (_root_.UInt8.ofNat (nat_lit 101)), (_root_.UInt8.ofNat (nat_lit 102)), (_root_.UInt8.ofNat (nat_lit 117)), (_root_.UInt8.ofNat (nat_lit 115)), (_root_.UInt8.ofNat (nat_lit 101)), (_root_.UInt8.ofNat (nat_lit 100))])

public theorem plan_outage : (plan (0) = 2) := by
  rfl

public theorem plan_access : (plan (2) = 1) := by
  rfl

end Helpdesk.Main
