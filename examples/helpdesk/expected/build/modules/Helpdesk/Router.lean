module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanRuntime.Models
public import LexLeanRuntime.Laws
public import LexLeanStd.Text
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Helpdesk.Router

public inductive Topic where
  | outage
  | billing
  | access

@[expose] public def mentions (request : ByteArray) (needle : ByteArray) : Bool := (match LexLeanStd.Text.find (request) (needle) with | Option.some _ => true | Option.none => false)

@[expose] public def signal (seen : Bool) : Int := (if seen then (1 : Int) else (0 : Int))

@[expose] public def features (request : ByteArray) : List (Int) := (signal ((mentions (request) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 100)), (_root_.UInt8.ofNat (nat_lit 111)), (_root_.UInt8.ofNat (nat_lit 119)), (_root_.UInt8.ofNat (nat_lit 110))]) || mentions (request) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 111)), (_root_.UInt8.ofNat (nat_lit 117)), (_root_.UInt8.ofNat (nat_lit 116)), (_root_.UInt8.ofNat (nat_lit 97)), (_root_.UInt8.ofNat (nat_lit 103)), (_root_.UInt8.ofNat (nat_lit 101))]))) :: (signal ((mentions (request) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 105)), (_root_.UInt8.ofNat (nat_lit 110)), (_root_.UInt8.ofNat (nat_lit 118)), (_root_.UInt8.ofNat (nat_lit 111)), (_root_.UInt8.ofNat (nat_lit 105)), (_root_.UInt8.ofNat (nat_lit 99)), (_root_.UInt8.ofNat (nat_lit 101))]) || mentions (request) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 114)), (_root_.UInt8.ofNat (nat_lit 101)), (_root_.UInt8.ofNat (nat_lit 102)), (_root_.UInt8.ofNat (nat_lit 117)), (_root_.UInt8.ofNat (nat_lit 110)), (_root_.UInt8.ofNat (nat_lit 100))]))) :: (signal ((mentions (request) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 112)), (_root_.UInt8.ofNat (nat_lit 97)), (_root_.UInt8.ofNat (nat_lit 115)), (_root_.UInt8.ofNat (nat_lit 115)), (_root_.UInt8.ofNat (nat_lit 119)), (_root_.UInt8.ofNat (nat_lit 111)), (_root_.UInt8.ofNat (nat_lit 114)), (_root_.UInt8.ofNat (nat_lit 100))]) || mentions (request) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 108)), (_root_.UInt8.ofNat (nat_lit 111)), (_root_.UInt8.ofNat (nat_lit 103)), (_root_.UInt8.ofNat (nat_lit 105)), (_root_.UInt8.ofNat (nat_lit 110))]))) :: ([] : List (Int)))))

@[expose, reducible] public def Readable (request : ByteArray) : Prop := (0 < (LexLeanRuntime.length (request) : Nat))

@[expose] public def readableCheck (request : ByteArray) : Bool := (Nat.blt (0) ((LexLeanRuntime.length (request) : Nat)))

public theorem readable_check_sound (request : ByteArray) : ((readableCheck (request) = true) -> Readable (request)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Readable, readableCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem readable_check_complete (request : ByteArray) : (Readable (request) -> (readableCheck (request) = true)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Readable, readableCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

@[expose] public def topicWeights.bytes : ByteArray := (let __byte0 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 0); _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 3)), __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, (_root_.UInt8.ofNat (nat_lit 3)), __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, (_root_.UInt8.ofNat (nat_lit 3)), __byte0])
@[expose] public def topicWeights : List (List (Int)) := (((3 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int))))) :: (((0 : Int) :: ((3 : Int) :: ((0 : Int) :: ([] : List (Int))))) :: (((0 : Int) :: ((0 : Int) :: ((3 : Int) :: ([] : List (Int))))) :: ([] : List (List (Int))))))
public theorem topicWeights.decoded : (LexLeanModels.tensorMatches 2 true topicWeights.flatten topicWeights.bytes && (topicWeights.length == 3) && (topicWeights.map List.length == List.replicate 3 3)) = true := by decide

@[expose] public def topicBias.bytes : ByteArray := (let __byte0 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 0); _root_.ByteArray.mk #[__byte0, __byte0, __byte0, __byte0, __byte0, __byte0])
@[expose] public def topicBias : List (Int) := ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int)))))
public theorem topicBias.decoded : (LexLeanModels.tensorMatches 2 true topicBias topicBias.bytes && (topicBias.length == 3)) = true := by decide

public theorem ScoringContract.precondition_sound : LexLeanModels.Sound1 ((Helpdesk.Router.readableCheck)) ((Helpdesk.Router.Readable)) := Helpdesk.Router.readable_check_sound
public theorem ScoringContract.precondition_complete : LexLeanModels.Complete1 ((Helpdesk.Router.readableCheck)) ((Helpdesk.Router.Readable)) := Helpdesk.Router.readable_check_complete

public theorem routing_width (request : ByteArray) : ((LexLeanRuntime.length (features (request)) : Nat) = 3) := by
  rfl

@[expose] public def TopicScore.encoded (request : ByteArray) : List (Int) := Helpdesk.Router.features (request)
@[expose] public def TopicScore.layer1 (__values : List (Int)) : List (Int) := ((LexLeanCollections.listFold ((fun (__acc : (Prod (List (Int)) (List (Int)))) (__row : List (Int)) => (match (__acc).2 with | List.nil => __acc | List.cons __bias __biases => ((LexLeanRuntime.append ((__acc).1) (((LexLeanRuntime.subtract (__bias) ((LexLeanRuntime.negate (((LexLeanCollections.listFold ((fun (__sum : (Prod (Int) (List (Int)))) (__weight : Int) => (match (__sum).2 with | List.nil => __sum | List.cons __value __rest => ((LexLeanRuntime.subtract ((__sum).1) ((LexLeanRuntime.negate ((LexLeanRuntime.multiply (__weight) (__value) : Int)) : Int)) : Int), __rest)))) (((0 : Int), __values)) (__row) : (Prod (Int) (List (Int))))).1) : Int)) : Int) :: ([] : List (Int)))) : List (Int)), __biases)))) ((([] : List (Int)), Helpdesk.Router.topicBias)) (Helpdesk.Router.topicWeights) : (Prod (List (Int)) (List (Int))))).1
@[expose] public def TopicScore.logits (request : ByteArray) : List (Int) := Helpdesk.Router.TopicScore.layer1 (Helpdesk.Router.TopicScore.encoded (request))
@[expose] public def TopicScore (request : ByteArray) : List (Int) := (let values : List (Int) := Helpdesk.Router.TopicScore.logits (request); values)

@[expose] public def topicAt (index : Nat) : Topic := (if (Nat.beq (index) (0)) then Topic.outage else (if (Nat.beq (index) (1)) then Topic.billing else Topic.access))

@[expose] public def ScoringModel (__input : ByteArray) : List (Int) := Helpdesk.Router.TopicScore (__input)

end Helpdesk.Router
