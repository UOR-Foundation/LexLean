module
public import Init
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

namespace LexLeanRuntime

public class ToMathInt (α : Type) where
  toInt : α -> Int

public class Fixed (α : Type) extends ToMathInt α where
  fromInt : Int -> α
  minimum : Int
  maximum : Int
  bitAnd : α -> α -> α
  bitOr : α -> α -> α
  bitXor : α -> α -> α
  bitNot : α -> α
  shiftLeft : α -> UInt32 -> Option α
  shiftRight : α -> UInt32 -> Option α

public instance : ToMathInt Int where toInt := fun value => value

public instance : Fixed Int8 where
  toInt := Int8.toInt
  fromInt := Int8.ofInt
  minimum := -128
  maximum := 127
  bitAnd := Int8.land
  bitOr := Int8.lor
  bitXor := Int8.xor
  bitNot := Int8.complement
  shiftLeft := fun value amount => if amount.toNat < 8 then some (Int8.shiftLeft value (Int8.ofNat amount.toNat)) else none
  shiftRight := fun value amount => if amount.toNat < 8 then some (Int8.shiftRight value (Int8.ofNat amount.toNat)) else none

public instance : Fixed Int16 where
  toInt := Int16.toInt
  fromInt := Int16.ofInt
  minimum := -32768
  maximum := 32767
  bitAnd := Int16.land
  bitOr := Int16.lor
  bitXor := Int16.xor
  bitNot := Int16.complement
  shiftLeft := fun value amount => if amount.toNat < 16 then some (Int16.shiftLeft value (Int16.ofNat amount.toNat)) else none
  shiftRight := fun value amount => if amount.toNat < 16 then some (Int16.shiftRight value (Int16.ofNat amount.toNat)) else none

public instance : Fixed Int32 where
  toInt := Int32.toInt
  fromInt := Int32.ofInt
  minimum := -2147483648
  maximum := 2147483647
  bitAnd := Int32.land
  bitOr := Int32.lor
  bitXor := Int32.xor
  bitNot := Int32.complement
  shiftLeft := fun value amount => if amount.toNat < 32 then some (Int32.shiftLeft value (Int32.ofNat amount.toNat)) else none
  shiftRight := fun value amount => if amount.toNat < 32 then some (Int32.shiftRight value (Int32.ofNat amount.toNat)) else none

public instance : Fixed Int64 where
  toInt := Int64.toInt
  fromInt := Int64.ofInt
  minimum := -9223372036854775808
  maximum := 9223372036854775807
  bitAnd := Int64.land
  bitOr := Int64.lor
  bitXor := Int64.xor
  bitNot := Int64.complement
  shiftLeft := fun value amount => if amount.toNat < 64 then some (Int64.shiftLeft value (Int64.ofNat amount.toNat)) else none
  shiftRight := fun value amount => if amount.toNat < 64 then some (Int64.shiftRight value (Int64.ofNat amount.toNat)) else none

public instance : Fixed UInt8 where
  toInt := fun value => Int.ofNat value.toNat
  fromInt := UInt8.ofInt
  minimum := 0
  maximum := 255
  bitAnd := UInt8.land
  bitOr := UInt8.lor
  bitXor := UInt8.xor
  bitNot := UInt8.complement
  shiftLeft := fun value amount => if amount.toNat < 8 then some (UInt8.shiftLeft value (UInt8.ofNat amount.toNat)) else none
  shiftRight := fun value amount => if amount.toNat < 8 then some (UInt8.shiftRight value (UInt8.ofNat amount.toNat)) else none

public instance : Fixed UInt16 where
  toInt := fun value => Int.ofNat value.toNat
  fromInt := UInt16.ofInt
  minimum := 0
  maximum := 65535
  bitAnd := UInt16.land
  bitOr := UInt16.lor
  bitXor := UInt16.xor
  bitNot := UInt16.complement
  shiftLeft := fun value amount => if amount.toNat < 16 then some (UInt16.shiftLeft value (UInt16.ofNat amount.toNat)) else none
  shiftRight := fun value amount => if amount.toNat < 16 then some (UInt16.shiftRight value (UInt16.ofNat amount.toNat)) else none

public instance : Fixed UInt32 where
  toInt := fun value => Int.ofNat value.toNat
  fromInt := UInt32.ofInt
  minimum := 0
  maximum := 4294967295
  bitAnd := UInt32.land
  bitOr := UInt32.lor
  bitXor := UInt32.xor
  bitNot := UInt32.complement
  shiftLeft := fun value amount => if amount.toNat < 32 then some (UInt32.shiftLeft value amount) else none
  shiftRight := fun value amount => if amount.toNat < 32 then some (UInt32.shiftRight value amount) else none

public instance : Fixed UInt64 where
  toInt := fun value => Int.ofNat value.toNat
  fromInt := UInt64.ofInt
  minimum := 0
  maximum := 18446744073709551615
  bitAnd := UInt64.land
  bitOr := UInt64.lor
  bitXor := UInt64.xor
  bitNot := UInt64.complement
  shiftLeft := fun value amount => if amount.toNat < 64 then some (UInt64.shiftLeft value (UInt64.ofNat amount.toNat)) else none
  shiftRight := fun value amount => if amount.toNat < 64 then some (UInt64.shiftRight value (UInt64.ofNat amount.toNat)) else none

@[expose] public def checkedFromInt {α : Type} [Fixed α] (value : Int) : Option α :=
  if value < Fixed.minimum (α := α) then none else if Fixed.maximum (α := α) < value then none else some (Fixed.fromInt value)

@[expose] public def checkedConvert {α β : Type} [ToMathInt α] [Fixed β] (value : α) : Option β :=
  checkedFromInt (ToMathInt.toInt value)

@[expose] public def checkedAdd {α : Type} [Fixed α] (left right : α) : Option α :=
  checkedFromInt (ToMathInt.toInt left + ToMathInt.toInt right)

@[expose] public def checkedSubtract {α : Type} [Fixed α] (left right : α) : Option α :=
  checkedFromInt (ToMathInt.toInt left - ToMathInt.toInt right)

@[expose] public def checkedMultiply {α : Type} [Fixed α] (left right : α) : Option α :=
  checkedFromInt (ToMathInt.toInt left * ToMathInt.toInt right)

@[expose] public def checkedNegate {α : Type} [Fixed α] (value : α) : Option α :=
  checkedFromInt (-ToMathInt.toInt value)

@[expose] public def checkedQuotient {α : Type} [Fixed α] (left right : α) : Option α :=
  if ToMathInt.toInt right = 0 then none else checkedFromInt (Int.tdiv (ToMathInt.toInt left) (ToMathInt.toInt right))

@[expose] public def checkedAddInt64 (left right : Int64) : Option Int64 :=
  let value := left + right
  if (0 < right && value < left) || (right < 0 && left < value) then none else some value

@[expose] public def checkedSubtractInt64 (left right : Int64) : Option Int64 :=
  let value := left - right
  if (0 < right && left < value) || (right < 0 && value < left) then none else some value

@[expose] public def checkedNegateInt64 (value : Int64) : Option Int64 :=
  if value == (-9223372036854775808 : Int64) then none else some (-value)

@[expose] public def magnitudeInt64 (value : Int64) : UInt64 :=
  let bits := value.toUInt64
  if value < 0 then 0 - bits else bits

@[expose] public def signedMagnitudeInt64 (negative : Bool) (value : UInt64) : Int64 :=
  (if negative then 0 - value else value).toInt64

@[expose] public def divideMagnitudeInt64 : Nat -> UInt64 -> UInt64 -> UInt64 -> UInt64 -> UInt64
  | 0, _, _, _, quotient => quotient
  | Nat.succ fuel, source, divisor, remainder, quotient =>
      let high := 9223372036854775808 <= source
      let source := source + source
      let remainder := remainder + remainder + if high then 1 else 0
      let quotient := quotient + quotient
      if divisor <= remainder then
        divideMagnitudeInt64 fuel source divisor (remainder - divisor) (quotient + 1)
      else
        divideMagnitudeInt64 fuel source divisor remainder quotient

@[expose] public def checkedQuotientInt64 (left right : Int64) : Option Int64 :=
  if right == 0 then none
  else if left == (-9223372036854775808 : Int64) && right == (-1 : Int64) then none
  else
    let negative := (left < 0) != (right < 0)
    some (signedMagnitudeInt64 negative
      (divideMagnitudeInt64 64 (magnitudeInt64 left) (magnitudeInt64 right) 0 0))

@[expose] public def multiplyMagnitudeInt64 : Nat -> UInt64 -> UInt64 -> UInt64 -> Bool -> Option UInt64
  | 0, _, _, accumulator, _ => some accumulator
  | Nat.succ fuel, source, multiplicand, accumulator, negative =>
      let high := 9223372036854775808 <= source
      let limit := if negative then 9223372036854775808 else 9223372036854775807
      let halfLimit := if negative then 4611686018427387904 else 4611686018427387903
      if halfLimit < accumulator then none
      else
        let doubled := accumulator + accumulator
        if high then
          if limit < multiplicand || limit - multiplicand < doubled then none
          else multiplyMagnitudeInt64 fuel (source + source) multiplicand
            (doubled + multiplicand) negative
        else
          multiplyMagnitudeInt64 fuel (source + source) multiplicand doubled negative

@[expose] public def checkedMultiplyInt64 (left right : Int64) : Option Int64 :=
  let negative := (left < 0) != (right < 0)
  match multiplyMagnitudeInt64 64 (magnitudeInt64 right) (magnitudeInt64 left) 0 negative with
  | none => none
  | some value => some (signedMagnitudeInt64 negative value)

@[expose, noinline] public def subtract {α : Type} [Sub α] (left right : α) : α := left - right
@[expose, noinline] public def multiply {α : Type} [Mul α] (left right : α) : α := left * right
@[expose, noinline] public def negate {α : Type} [Neg α] (value : α) : α := -value

public class Quotient (α : Type) where
  quotient : α -> α -> α
  remainder : α -> α -> α
  isZero : α -> Bool

public instance : Quotient Nat where
  quotient := Nat.div
  remainder := Nat.mod
  isZero := fun value => value == 0

public instance : Quotient Int where
  quotient := Int.tdiv
  remainder := Int.tmod
  isZero := fun value => value == 0

@[expose, noinline] public def quotient {α : Type} [Quotient α] (left right zeroCase : α) : α :=
  if Quotient.isZero right then zeroCase else Quotient.quotient left right

@[expose, noinline] public def remainder {α : Type} [Quotient α] (left right zeroCase : α) : α :=
  if Quotient.isZero right then zeroCase else Quotient.remainder left right

@[expose] public def bitAnd {α : Type} [Fixed α] (left right : α) : α := Fixed.bitAnd left right
@[expose] public def bitOr {α : Type} [Fixed α] (left right : α) : α := Fixed.bitOr left right
@[expose] public def bitXor {α : Type} [Fixed α] (left right : α) : α := Fixed.bitXor left right
@[expose] public def bitNot {α : Type} [Fixed α] (value : α) : α := Fixed.bitNot value
@[expose] public def shiftLeft {α : Type} [Fixed α] (value : α) (amount : UInt32) : Option α := Fixed.shiftLeft value amount
@[expose] public def shiftRight {α : Type} [Fixed α] (value : α) (amount : UInt32) : Option α := Fixed.shiftRight value amount

public class Appendable (α : Type) where append : α -> α -> α
public instance {α : Type} : Appendable (List α) where append := List.append
public instance : Appendable ByteArray where append := ByteArray.append
@[expose] public def append {α : Type} [Appendable α] (left right : α) : α := Appendable.append left right

public class Lengthable (α : Type) where length : α -> Nat
public instance {α : Type} : Lengthable (List α) where length := List.length
public instance : Lengthable ByteArray where length := ByteArray.size
public instance : Lengthable String where length := String.length
@[expose] public def length {α : Type} [Lengthable α] (value : α) : Nat := Lengthable.length value

@[expose] public def listIndex {α : Type} : List α -> Nat -> Option α
  | [], _ => none
  | head :: _, 0 => some head
  | _ :: tail, index + 1 => listIndex tail index

public class Indexable (α β : Type) where index : α -> Nat -> Option β
public instance {α : Type} : Indexable (List α) α where index := listIndex
public instance : Indexable ByteArray UInt8 where index := fun value offset => value.data[offset]?
@[expose, noinline] public def index {α β : Type} [Indexable α β] (value : α) (offset : Nat) : Option β := Indexable.index value offset

public class Sliceable (α : Type) where slice : α -> Nat -> Nat -> Option α
public instance {α : Type} : Sliceable (List α) where
  slice := fun value start count => if start + count <= value.length then some ((value.drop start).take count) else none
public instance : Sliceable ByteArray where
  slice := fun value start count => if start + count <= value.size then some (value.extract start (start + count)) else none
@[expose, noinline] public def slice {α : Type} [Sliceable α] (value : α) (start count : Nat) : Option α := Sliceable.slice value start count

@[expose, noinline] public def utf8Encode (value : String) : ByteArray := value.toUTF8
@[expose, noinline] public def utf8Decode (value : ByteArray) : Option String := String.fromUTF8? value
@[expose, noinline] public def compareBytes (left right : ByteArray) : Ordering := compare left.toList right.toList
@[expose] public def equal {α : Type} [BEq α] (left right : α) : Bool := left == right

@[expose, noinline] public def splitExact (value delimiter : String) (maximum : UInt32) : Option (List String) :=
  let fields := value.splitOn delimiter
  if delimiter.isEmpty || maximum.toNat < fields.length then none else some fields

@[expose, noinline] public def join (values : List String) (delimiter : String) : String := delimiter.intercalate values

public class Decimal (α : Type) where
  parse : String -> Option α
  format : α -> String

public instance : Decimal Int where
  parse := fun value => match value.toInt? with | some parsed => if toString parsed = value then some parsed else none | none => none
  format := toString

public instance {α : Type} [Fixed α] [ToString α] : Decimal α where
  parse := fun value => match value.toInt? with | some parsed => if toString parsed = value then checkedFromInt parsed else none | none => none
  format := toString

@[expose, noinline] public def parseDecimal {α : Type} [Decimal α] (value : String) : Option α := Decimal.parse value
@[expose, noinline] public def formatDecimal {α : Type} [Decimal α] (value : α) : String := Decimal.format value

end LexLeanRuntime

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

end Models.Main
