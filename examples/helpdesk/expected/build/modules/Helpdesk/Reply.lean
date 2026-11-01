module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanStdReasoning.Worklist
public import Helpdesk.Router
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Helpdesk.Reply

@[expose] public def workFor (topic : Helpdesk.Router.Topic) : Nat := (match topic with | Helpdesk.Router.Topic.outage => 3 | Helpdesk.Router.Topic.billing => 2 | Helpdesk.Router.Topic.access => 1)

@[expose] public def stepsFor (topic : Helpdesk.Router.Topic) : Nat := (match LexLeanStdReasoning.Worklist.Drain (Helpdesk.Router.Topic) ((topic, workFor (topic))) with | Except.ok explained => (LexLeanRuntime.length ((explained).2) : Nat) | Except.error _ => 0)

@[expose] public def topicName (topic : Helpdesk.Router.Topic) : ByteArray := (match topic with | Helpdesk.Router.Topic.outage => _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 111)), (_root_.UInt8.ofNat (nat_lit 117)), (_root_.UInt8.ofNat (nat_lit 116)), (_root_.UInt8.ofNat (nat_lit 97)), (_root_.UInt8.ofNat (nat_lit 103)), (_root_.UInt8.ofNat (nat_lit 101))] | Helpdesk.Router.Topic.billing => _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 98)), (_root_.UInt8.ofNat (nat_lit 105)), (_root_.UInt8.ofNat (nat_lit 108)), (_root_.UInt8.ofNat (nat_lit 108)), (_root_.UInt8.ofNat (nat_lit 105)), (_root_.UInt8.ofNat (nat_lit 110)), (_root_.UInt8.ofNat (nat_lit 103))] | Helpdesk.Router.Topic.access => _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 97)), (_root_.UInt8.ofNat (nat_lit 99)), (_root_.UInt8.ofNat (nat_lit 99)), (_root_.UInt8.ofNat (nat_lit 101)), (_root_.UInt8.ofNat (nat_lit 115)), (_root_.UInt8.ofNat (nat_lit 115))])

@[expose] public def digit (count : Nat) : ByteArray := (if (Nat.beq (count) (0)) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 48))] else (if (Nat.beq (count) (1)) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 49))] else (if (Nat.beq (count) (2)) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 50))] else (if (Nat.beq (count) (3)) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 51))] else _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 43))]))))

@[expose] public def message (topic : Helpdesk.Router.Topic) : ByteArray := (LexLeanRuntime.append (topicName (topic)) ((LexLeanRuntime.append (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 58))]) (digit (stepsFor (topic))) : ByteArray)) : ByteArray)

public theorem outage_takes_two_steps : (stepsFor (Helpdesk.Router.Topic.outage) = 2) := by
  rfl

end Helpdesk.Reply
