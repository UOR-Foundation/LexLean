module
public import Init
public import LexLeanTarget.TargetSemantics
public import LexLeanTarget.TargetSyntax
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
namespace LexLeanTarget.Gnaf

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

public inductive ClaimClass where
  | exact
  | normalForm
  | canonical
  | representationMinimal
  | comparisonTheorem
  | profileDefinedComparison (_ : String) (_ : String) (_ : String)
  | inputTotal
  | globalOptimal
  | argminComplete
  | paretoOptimal
  | frontierComplete
  | pointwiseEnvelopeComplete
  | queryFamilyAnswerComplete
  | useCaseGlobalOptimal
  | workloadArgminComplete
  | workloadParetoOptimal
  | workloadFrontierComplete
  | familyOptimal
  | competitiveBound
  | competitiveOptimal
  | asymptoticBound
  | asymptoticOptimal
  | useCaseClassComplete
  | useCaseClassAnswerComplete
  | maintainedUseCaseClass
  | restrictedUniverseOptimal
  | revisionPreserved
  | bestKnown
  | measuredBestAmongTested
  | heuristicSelected
  | instanceOptimal (_ : Nat) (_ : Nat)

public inductive ActionKind where
  | observation
  | preprocessing
  | advice
  | retainedState
  | dispatch
  | fallback
  | communication
  | randomness
  | scheduling
  | execution

public inductive Charge where
  | steps
  | constant (_ : Nat)
  | free
  | undeclared

public structure Action where
  kind : ActionKind
  charge : Charge

public inductive Boundary where
  | complete
  | preparedState
  | preparedPlan

public inductive OperandSize where
  | weighted
  | unit

public structure Capacity where
  fuel : Nat
  domain : Nat
  systems : Nat
  charge : Nat

public structure Prepared where
  kind : ActionKind
  artifact : String

public structure Machine where
  fuel : Nat
  capacity : Capacity
  operandSize : OperandSize
  actions : List (Action)
  boundary : Boundary
  prepared : List (Prepared)

public inductive Selector where
  | fixed (_ : Nat)
  | dispatch (_ : Nat) (_ : Nat) (_ : Nat)

public structure Plan where
  function : LexLeanTarget.TargetSyntax.Function
  prepares : List (ActionKind)

public structure Grammar where
  argument : LexLeanTarget.TargetSyntax.Ty
  result : LexLeanTarget.TargetSyntax.Ty
  plans : List (Plan)
  thresholds : List (Nat)

public inductive Carrier where
  | grammar (_ : Grammar)
  | internalPlans
  | optimizerOutput
  | discovered (_ : List (Nat))
  | cached

public inductive Completeness where
  | grammarEquality
  | missing
  | citesUniverseId
  | citesOptimizer

public inductive Scope where
  | grammarUniverse
  | calculusPrograms
  | rustPrograms

public inductive Objective where
  | scalar
  | vector

public structure Request where
  reference : LexLeanTarget.TargetSyntax.Program
  domain : List (LexLeanTarget.TargetSyntax.Value)
  machine : Machine
  carrier : Carrier
  completeness : Completeness
  «universe» : String
  objective : Objective
  claim : ClaimClass
  scope : Scope

public inductive Rejection where
  | emptyDomain
  | internalPlanUniverse
  | optimizerDefinedUniverse
  | discoveredUniverse
  | cachedUniverse
  | missingCompleteness
  | selfReferentialCompleteness
  | optimizerCompleteness
  | beyondCapacity
  | unitCostOperands
  | duplicateAction (_ : ActionKind)
  | unaccountedAction (_ : ActionKind)
  | hiddenCost (_ : ActionKind)
  | unrealizableAction (_ : ActionKind)
  | unboundPreparation (_ : ActionKind)
  | strayPreparedArtifact (_ : ActionKind)
  | claimAlias
  | scalarClaimOverPartialOrder
  | vectorClaimOverTotalOrder
  | uncoveredScope
  | unsupportedClaim

public inductive Status where
  | admitted (_ : Nat) (_ : Nat)
  | inadmissible
  | unresolved

public inductive Answer where
  | rejected (_ : Rejection)
  | argmin (_ : List (Nat)) (_ : Nat)
  | frontier (_ : List (Nat))
  | infeasible
  | incomplete

@[expose] public def kindIndex (kind : ActionKind) : Nat := (match kind with | ActionKind.observation => 0 | ActionKind.preprocessing => 1 | ActionKind.advice => 2 | ActionKind.retainedState => 3 | ActionKind.dispatch => 4 | ActionKind.fallback => 5 | ActionKind.communication => 6 | ActionKind.randomness => 7 | ActionKind.scheduling => 8 | ActionKind.execution => 9)

@[expose] public def sameKind (left : ActionKind) (right : ActionKind) : Bool := (Nat.beq (kindIndex (left)) (kindIndex (right)))

@[expose] public def performed (kind : ActionKind) : Bool := (match kind with | ActionKind.observation => true | ActionKind.preprocessing => false | ActionKind.advice => false | ActionKind.retainedState => false | ActionKind.dispatch => true | ActionKind.fallback => true | ActionKind.communication => false | ActionKind.randomness => false | ActionKind.scheduling => false | ActionKind.execution => true)

@[expose] public def preparation (kind : ActionKind) : Bool := (match kind with | ActionKind.observation => false | ActionKind.preprocessing => true | ActionKind.advice => true | ActionKind.retainedState => true | ActionKind.dispatch => false | ActionKind.fallback => false | ActionKind.communication => false | ActionKind.randomness => false | ActionKind.scheduling => false | ActionKind.execution => false)

@[expose] public def findAction : (actions : List (Action)) -> (wanted : ActionKind) -> Option (Charge)
  | List.nil, _wanted => Option.none
  | List.cons action rest, wanted => (if sameKind ((action).kind) (wanted) then Option.some ((action).charge) else findAction (rest) (wanted))

@[expose] public def countKind : (actions : List (Action)) -> (wanted : ActionKind) -> Nat
  | List.nil, _wanted => 0
  | List.cons action rest, wanted => ((if sameKind ((action).kind) (wanted) then 1 else 0) + countKind (rest) (wanted))

@[expose] public def checkDistinct : (actions : List (Action)) -> (all : List (Action)) -> Option (Rejection)
  | List.nil, _all => Option.none
  | List.cons action rest, all => (if (Nat.blt (1) (countKind (all) ((action).kind))) then Option.some (Rejection.duplicateAction ((action).kind)) else checkDistinct (rest) (all))

@[expose] public def checkPerformed (machine : Machine) (wanted : ActionKind) : Option (Rejection) := (match findAction ((machine).actions) (wanted) with | Option.none => Option.some (Rejection.unaccountedAction (wanted)) | Option.some charge => (match charge with | Charge.steps => Option.none | Charge.constant _ => Option.some (Rejection.hiddenCost (wanted)) | Charge.free => Option.some (Rejection.hiddenCost (wanted)) | Charge.undeclared => Option.some (Rejection.hiddenCost (wanted))))

@[expose] public def preparedFor : (prepared : List (Prepared)) -> (kind : ActionKind) -> Bool
  | List.nil, _kind => false
  | List.cons artifact rest, kind => (sameKind ((artifact).kind) (kind) || preparedFor (rest) (kind))

@[expose] public def checkDeclared (machine : Machine) (action : Action) : Option (Rejection) := (let kind : ActionKind := (action).kind; (if performed (kind) then Option.none else (if preparation (kind) then (match (action).charge with | Charge.steps => Option.some (Rejection.hiddenCost (kind)) | Charge.constant amount => (if (Nat.beq (amount) (0)) then Option.some (Rejection.hiddenCost (kind)) else Option.none) | Charge.free => (match (machine).boundary with | Boundary.complete => Option.some (Rejection.unboundPreparation (kind)) | Boundary.preparedState => (if preparedFor ((machine).prepared) (kind) then Option.none else Option.some (Rejection.unboundPreparation (kind))) | Boundary.preparedPlan => (if preparedFor ((machine).prepared) (kind) then Option.none else Option.some (Rejection.unboundPreparation (kind)))) | Charge.undeclared => Option.some (Rejection.hiddenCost (kind))) else Option.some (Rejection.unrealizableAction (kind)))))

@[expose] public def checkAllDeclared : (machine : Machine) -> (actions : List (Action)) -> Option (Rejection)
  | _machine, List.nil => Option.none
  | machine, List.cons action rest => (match checkDeclared (machine) (action) with | Option.none => checkAllDeclared (machine) (rest) | Option.some rejection => Option.some (rejection))

@[expose] public def checkPrepared : (machine : Machine) -> (prepared : List (Prepared)) -> Option (Rejection)
  | _machine, List.nil => Option.none
  | machine, List.cons artifact rest => (match findAction ((machine).actions) ((artifact).kind) with | Option.none => Option.some (Rejection.strayPreparedArtifact ((artifact).kind)) | Option.some charge => (match charge with | Charge.steps => Option.some (Rejection.strayPreparedArtifact ((artifact).kind)) | Charge.constant _ => Option.some (Rejection.strayPreparedArtifact ((artifact).kind)) | Charge.free => checkPrepared (machine) (rest) | Charge.undeclared => Option.some (Rejection.strayPreparedArtifact ((artifact).kind))))

@[expose] public def unitCharge (actions : List (Action)) (kind : ActionKind) : Nat := (match findAction (actions) (kind) with | Option.none => 0 | Option.some charge => (match charge with | Charge.steps => 0 | Charge.constant amount => amount | Charge.free => 0 | Charge.undeclared => 0))

@[expose] public def planCharge : (actions : List (Action)) -> (kinds : List (ActionKind)) -> Nat
  | _actions, List.nil => 0
  | actions, List.cons kind rest => (unitCharge (actions) (kind) + planCharge (actions) (rest))

@[expose] public def checkKindsAccounted : (actions : List (Action)) -> (kinds : List (ActionKind)) -> Option (Rejection)
  | _actions, List.nil => Option.none
  | actions, List.cons kind rest => (match findAction (actions) (kind) with | Option.none => Option.some (Rejection.unaccountedAction (kind)) | Option.some _ => checkKindsAccounted (actions) (rest))

@[expose] public def checkPlanPreparation : (actions : List (Action)) -> (plans : List (Plan)) -> Option (Rejection)
  | _actions, List.nil => Option.none
  | actions, List.cons plan rest => (match checkKindsAccounted (actions) ((plan).prepares) with | Option.none => checkPlanPreparation (actions) (rest) | Option.some rejection => Option.some (rejection))

@[expose] public def chargeBeyond : (actions : List (Action)) -> (bound : Nat) -> Bool
  | List.nil, _bound => false
  | List.cons action rest, bound => ((match (action).charge with | Charge.steps => false | Charge.constant amount => (Nat.blt (bound) (amount)) | Charge.free => false | Charge.undeclared => false) || chargeBeyond (rest) (bound))

@[expose] public def appendAll (Item : Type) : (front : List (Item)) -> (back : List (Item)) -> List (Item)
  | List.nil, back => back
  | List.cons head rest, back => (head :: appendAll (Item) (rest) (back))

@[expose] public def range : (count : Nat) -> List (Nat)
  | Nat.zero => ([] : List (Nat))
  | Nat.succ remaining => appendAll (Nat) (range (remaining)) ((remaining :: ([] : List (Nat))))

@[expose] public def natIn : (item : Nat) -> (items : List (Nat)) -> Bool
  | _item, List.nil => false
  | item, List.cons head rest => ((Nat.beq (item) (head)) || natIn (item) (rest))

@[expose] public def selectorEq (left : Selector) (right : Selector) : Bool := (match left with | Selector.fixed plan => (match right with | Selector.fixed other => (Nat.beq (plan) (other)) | Selector.dispatch _ _ _ => false) | Selector.dispatch threshold small large => (match right with | Selector.fixed _ => false | Selector.dispatch otherThreshold otherSmall otherLarge => ((Nat.beq (threshold) (otherThreshold)) && ((Nat.beq (small) (otherSmall)) && (Nat.beq (large) (otherLarge))))))

@[expose] public def selectorIn : (selector : Selector) -> (selectors : List (Selector)) -> Bool
  | _selector, List.nil => false
  | selector, List.cons head rest => (selectorEq (selector) (head) || selectorIn (selector) (rest))

@[expose] public def fixedOver : (plans : List (Nat)) -> List (Selector)
  | List.nil => ([] : List (Selector))
  | List.cons plan rest => (Selector.fixed (plan) :: fixedOver (rest))

@[expose] public def dispatchOver : (threshold : Nat) -> (small : Nat) -> (larges : List (Nat)) -> List (Selector)
  | _threshold, _small, List.nil => ([] : List (Selector))
  | threshold, small, List.cons large rest => (Selector.dispatch (threshold) (small) (large) :: dispatchOver (threshold) (small) (rest))

@[expose] public def dispatchSmalls : (threshold : Nat) -> (smalls : List (Nat)) -> (larges : List (Nat)) -> List (Selector)
  | _threshold, List.nil, _larges => ([] : List (Selector))
  | threshold, List.cons small rest, larges => appendAll (Selector) (dispatchOver (threshold) (small) (larges)) (dispatchSmalls (threshold) (rest) (larges))

@[expose] public def dispatchThresholds : (thresholds : List (Nat)) -> (plans : List (Nat)) -> List (Selector)
  | List.nil, _plans => ([] : List (Selector))
  | List.cons threshold rest, plans => appendAll (Selector) (dispatchSmalls (threshold) (plans) (plans)) (dispatchThresholds (rest) (plans))

@[expose] public def expand (grammar : Grammar) : List (Selector) := (let all : List (Nat) := range ((LexLeanRuntime.length ((grammar).plans) : Nat)); appendAll (Selector) (fixedOver (all)) (dispatchThresholds ((grammar).thresholds) (all)))

@[expose] public def wellFormed (grammar : Grammar) (selector : Selector) : Bool := (let count : Nat := (LexLeanRuntime.length ((grammar).plans) : Nat); (match selector with | Selector.fixed plan => (Nat.blt (plan) (count)) | Selector.dispatch threshold small large => (natIn (threshold) ((grammar).thresholds) && ((Nat.blt (small) (count)) && (Nat.blt (large) (count))))))

public theorem orAssociative : (forall (a : Bool), (forall (b : Bool), (forall (c : Bool), (((a || b) || c) = (a || (b || c)))))) := by
  decide

public theorem orFalse : (forall (a : Bool), ((a || false) = a)) := by
  decide

public theorem falseOr : (forall (a : Bool), ((false || a) = a)) := by
  decide

public theorem andFalse : (forall (a : Bool), ((a && false) = false)) := by
  decide

public theorem falseAnd : (forall (a : Bool), ((false && a) = false)) := by
  decide

public theorem andOverOr : (forall (a : Bool), (forall (b : Bool), (forall (c : Bool), (((a && b) || (a && c)) = (a && (b || c)))))) := by
  decide

public theorem orUnderAnd : (forall (a : Bool), (forall (b : Bool), (forall (c : Bool), (((a && c) || (b && c)) = ((a || b) && c))))) := by
  decide

public theorem bltNext (left : Nat) (right : Nat) : ((Nat.blt ((left + 1)) ((right + 1))) = (Nat.blt (left) (right))) := by
  rfl

public theorem beqNext (left : Nat) (right : Nat) : ((Nat.beq ((left + 1)) ((right + 1))) = (Nat.beq (left) (right))) := by
  rfl

public theorem bltSucc (item : Nat) (bound : Nat) : ((Nat.blt (item) ((bound + 1))) = ((Nat.blt (item) (bound)) || (Nat.beq (item) (bound)))) := by
  induction item generalizing bound with
  | zero =>
    cases bound with
    | zero =>
      rfl
    | succ smaller =>
      rfl
  | succ previous hypothesis =>
    cases bound with
    | zero =>
      rfl
    | succ smaller =>
      simp only [beqNext, bltNext, hypothesis]

public theorem natInAppend (item : Nat) (front : List (Nat)) (back : List (Nat)) : (natIn (item) (appendAll (Nat) (front) (back)) = (natIn (item) (front) || natIn (item) (back))) := by
  induction front with
  | nil =>
    rfl
  | cons head rest hypothesis =>
    simp only [appendAll, hypothesis, natIn, orAssociative]

public theorem selectorInAppend (selector : Selector) (front : List (Selector)) (back : List (Selector)) : (selectorIn (selector) (appendAll (Selector) (front) (back)) = (selectorIn (selector) (front) || selectorIn (selector) (back))) := by
  induction front with
  | nil =>
    rfl
  | cons head rest hypothesis =>
    simp only [appendAll, hypothesis, orAssociative, selectorIn]

public theorem natInRange (item : Nat) (count : Nat) : (natIn (item) (range (count)) = (Nat.blt (item) (count))) := by
  induction count with
  | zero =>
    rfl
  | succ smaller hypothesis =>
    simp only [bltSucc, hypothesis, natIn, natInAppend, orFalse, range]

public theorem fixedInFixed (plan : Nat) (plans : List (Nat)) : (selectorIn (Selector.fixed (plan)) (fixedOver (plans)) = natIn (plan) (plans)) := by
  induction plans with
  | nil =>
    rfl
  | cons head rest hypothesis =>
    simp only [fixedOver, hypothesis, natIn, selectorEq, selectorIn]

public theorem dispatchInFixed (threshold : Nat) (small : Nat) (large : Nat) (plans : List (Nat)) : (selectorIn (Selector.dispatch (threshold) (small) (large)) (fixedOver (plans)) = false) := by
  induction plans with
  | nil =>
    rfl
  | cons head rest hypothesis =>
    simp only [falseOr, fixedOver, hypothesis, selectorEq, selectorIn]

public theorem fixedInOver (plan : Nat) (level : Nat) (origin : Nat) (larges : List (Nat)) : (selectorIn (Selector.fixed (plan)) (dispatchOver (level) (origin) (larges)) = false) := by
  induction larges with
  | nil =>
    rfl
  | cons head rest hypothesis =>
    simp only [dispatchOver, falseOr, hypothesis, selectorEq, selectorIn]

public theorem fixedInSmalls (plan : Nat) (level : Nat) (smalls : List (Nat)) (larges : List (Nat)) : (selectorIn (Selector.fixed (plan)) (dispatchSmalls (level) (smalls) (larges)) = false) := by
  induction smalls with
  | nil =>
    rfl
  | cons head rest hypothesis =>
    simp only [dispatchSmalls, falseOr, fixedInOver, hypothesis, selectorInAppend]

public theorem fixedInDispatch (plan : Nat) (thresholds : List (Nat)) (plans : List (Nat)) : (selectorIn (Selector.fixed (plan)) (dispatchThresholds (thresholds) (plans)) = false) := by
  induction thresholds with
  | nil =>
    rfl
  | cons head rest hypothesis =>
    simp only [dispatchThresholds, falseOr, fixedInSmalls, hypothesis, selectorInAppend]

public theorem dispatchInOver (threshold : Nat) (small : Nat) (large : Nat) (level : Nat) (origin : Nat) (larges : List (Nat)) : (selectorIn (Selector.dispatch (threshold) (small) (large)) (dispatchOver (level) (origin) (larges)) = ((Nat.beq (threshold) (level)) && ((Nat.beq (small) (origin)) && natIn (large) (larges)))) := by
  induction larges with
  | nil =>
    simp only [andFalse, dispatchOver, natIn, selectorIn]
  | cons head rest hypothesis =>
    simp only [andOverOr, dispatchOver, hypothesis, natIn, selectorEq, selectorIn]

public theorem dispatchInSmalls (threshold : Nat) (small : Nat) (large : Nat) (level : Nat) (smalls : List (Nat)) (larges : List (Nat)) : (selectorIn (Selector.dispatch (threshold) (small) (large)) (dispatchSmalls (level) (smalls) (larges)) = ((Nat.beq (threshold) (level)) && (natIn (small) (smalls) && natIn (large) (larges)))) := by
  induction smalls with
  | nil =>
    simp only [andFalse, dispatchSmalls, falseAnd, natIn, selectorIn]
  | cons head rest hypothesis =>
    simp only [andOverOr, dispatchInOver, dispatchSmalls, hypothesis, natIn, orUnderAnd, selectorInAppend]

public theorem dispatchInThresholds (threshold : Nat) (small : Nat) (large : Nat) (thresholds : List (Nat)) (plans : List (Nat)) : (selectorIn (Selector.dispatch (threshold) (small) (large)) (dispatchThresholds (thresholds) (plans)) = (natIn (threshold) (thresholds) && (natIn (small) (plans) && natIn (large) (plans)))) := by
  induction thresholds with
  | nil =>
    rfl
  | cons head rest hypothesis =>
    simp only [dispatchInSmalls, dispatchThresholds, hypothesis, natIn, orUnderAnd, selectorInAppend]

public theorem expandComplete (grammar : Grammar) (selector : Selector) : (selectorIn (selector) (expand (grammar)) = wellFormed (grammar) (selector)) := by
  cases selector with
  | fixed plan =>
    simp only [expand, fixedInDispatch, fixedInFixed, natInRange, orFalse, selectorInAppend, wellFormed]
  | dispatch threshold small large =>
    simp only [dispatchInFixed, dispatchInThresholds, expand, falseOr, natInRange, selectorInAppend, wellFormed]

@[expose] public def checkClaim (request : Request) : Option (Rejection) := (match (request).claim with | ClaimClass.exact => Option.some (Rejection.unsupportedClaim) | ClaimClass.normalForm => Option.some (Rejection.unsupportedClaim) | ClaimClass.canonical => Option.some (Rejection.unsupportedClaim) | ClaimClass.representationMinimal => Option.some (Rejection.unsupportedClaim) | ClaimClass.comparisonTheorem => Option.some (Rejection.unsupportedClaim) | ClaimClass.profileDefinedComparison _ _ _ => Option.some (Rejection.unsupportedClaim) | ClaimClass.inputTotal => Option.some (Rejection.unsupportedClaim) | ClaimClass.globalOptimal => (match (request).objective with | Objective.scalar => Option.none | Objective.vector => Option.some (Rejection.scalarClaimOverPartialOrder)) | ClaimClass.argminComplete => (match (request).objective with | Objective.scalar => Option.none | Objective.vector => Option.some (Rejection.scalarClaimOverPartialOrder)) | ClaimClass.paretoOptimal => (match (request).objective with | Objective.scalar => Option.some (Rejection.vectorClaimOverTotalOrder) | Objective.vector => Option.none) | ClaimClass.frontierComplete => (match (request).objective with | Objective.scalar => Option.some (Rejection.vectorClaimOverTotalOrder) | Objective.vector => Option.none) | ClaimClass.pointwiseEnvelopeComplete => Option.some (Rejection.unsupportedClaim) | ClaimClass.queryFamilyAnswerComplete => Option.some (Rejection.unsupportedClaim) | ClaimClass.useCaseGlobalOptimal => Option.some (Rejection.unsupportedClaim) | ClaimClass.workloadArgminComplete => Option.some (Rejection.unsupportedClaim) | ClaimClass.workloadParetoOptimal => Option.some (Rejection.unsupportedClaim) | ClaimClass.workloadFrontierComplete => Option.some (Rejection.unsupportedClaim) | ClaimClass.familyOptimal => Option.some (Rejection.unsupportedClaim) | ClaimClass.competitiveBound => Option.some (Rejection.unsupportedClaim) | ClaimClass.competitiveOptimal => Option.some (Rejection.unsupportedClaim) | ClaimClass.asymptoticBound => Option.some (Rejection.unsupportedClaim) | ClaimClass.asymptoticOptimal => Option.some (Rejection.unsupportedClaim) | ClaimClass.useCaseClassComplete => Option.some (Rejection.unsupportedClaim) | ClaimClass.useCaseClassAnswerComplete => Option.some (Rejection.unsupportedClaim) | ClaimClass.maintainedUseCaseClass => Option.some (Rejection.unsupportedClaim) | ClaimClass.restrictedUniverseOptimal => Option.some (Rejection.claimAlias) | ClaimClass.revisionPreserved => Option.some (Rejection.unsupportedClaim) | ClaimClass.bestKnown => Option.some (Rejection.unsupportedClaim) | ClaimClass.measuredBestAmongTested => Option.some (Rejection.unsupportedClaim) | ClaimClass.heuristicSelected => Option.some (Rejection.unsupportedClaim) | ClaimClass.instanceOptimal _ _ => Option.some (Rejection.unsupportedClaim))

@[expose] public def validate (request : Request) : Option (Rejection) := (match (match (request).domain with | List.nil => Option.some (Rejection.emptyDomain) | List.cons _ _ => Option.none) with | Option.none => (match (match (request).carrier with | Carrier.grammar _ => Option.none | Carrier.internalPlans => Option.some (Rejection.internalPlanUniverse) | Carrier.optimizerOutput => Option.some (Rejection.optimizerDefinedUniverse) | Carrier.discovered _ => Option.some (Rejection.discoveredUniverse) | Carrier.cached => Option.some (Rejection.cachedUniverse)) with | Option.none => (match (match (request).completeness with | Completeness.grammarEquality => Option.none | Completeness.missing => Option.some (Rejection.missingCompleteness) | Completeness.citesUniverseId => Option.some (Rejection.selfReferentialCompleteness) | Completeness.citesOptimizer => Option.some (Rejection.optimizerCompleteness)) with | Option.none => (match (match (request).carrier with | Carrier.grammar grammar => (if (((Nat.blt ((((request).machine).capacity).fuel) (((request).machine).fuel)) || (Nat.blt ((((request).machine).capacity).domain) ((LexLeanRuntime.length ((request).domain) : Nat)))) || ((Nat.blt ((((request).machine).capacity).systems) ((LexLeanRuntime.length (expand (grammar)) : Nat))) || chargeBeyond (((request).machine).actions) ((((request).machine).capacity).charge))) then Option.some (Rejection.beyondCapacity) else Option.none) | Carrier.internalPlans => Option.none | Carrier.optimizerOutput => Option.none | Carrier.discovered _ => Option.none | Carrier.cached => Option.none) with | Option.none => (match (match ((request).machine).operandSize with | OperandSize.weighted => Option.none | OperandSize.unit => Option.some (Rejection.unitCostOperands)) with | Option.none => (match checkDistinct (((request).machine).actions) (((request).machine).actions) with | Option.none => (match checkPerformed ((request).machine) (ActionKind.observation) with | Option.none => (match checkPerformed ((request).machine) (ActionKind.dispatch) with | Option.none => (match checkPerformed ((request).machine) (ActionKind.fallback) with | Option.none => (match checkPerformed ((request).machine) (ActionKind.execution) with | Option.none => (match checkAllDeclared ((request).machine) (((request).machine).actions) with | Option.none => (match checkPrepared ((request).machine) (((request).machine).prepared) with | Option.none => (match (match (request).carrier with | Carrier.grammar grammar => checkPlanPreparation (((request).machine).actions) ((grammar).plans) | Carrier.internalPlans => Option.none | Carrier.optimizerOutput => Option.none | Carrier.discovered _ => Option.none | Carrier.cached => Option.none) with | Option.none => (match checkClaim (request) with | Option.none => (match (match (request).scope with | Scope.grammarUniverse => Option.none | Scope.calculusPrograms => Option.some (Rejection.uncoveredScope) | Scope.rustPrograms => Option.some (Rejection.uncoveredScope)) with | Option.none => Option.none | Option.some rejection29 => Option.some (rejection29)) | Option.some rejection28 => Option.some (rejection28)) | Option.some rejection27 => Option.some (rejection27)) | Option.some rejection26 => Option.some (rejection26)) | Option.some rejection25 => Option.some (rejection25)) | Option.some rejection24 => Option.some (rejection24)) | Option.some rejection23 => Option.some (rejection23)) | Option.some rejection22 => Option.some (rejection22)) | Option.some rejection21 => Option.some (rejection21)) | Option.some rejection20 => Option.some (rejection20)) | Option.some rejection19 => Option.some (rejection19)) | Option.some rejection18 => Option.some (rejection18)) | Option.some rejection17 => Option.some (rejection17)) | Option.some rejection16 => Option.some (rejection16)) | Option.some rejection15 => Option.some (rejection15))

@[expose] public def entryBody (selector : Selector) : LexLeanTarget.TargetSyntax.Expr := (match selector with | Selector.fixed plan => LexLeanTarget.TargetSyntax.Expr.call ((plan + 1)) ((LexLeanTarget.TargetSyntax.Expr.var (0) :: ([] : List (LexLeanTarget.TargetSyntax.Expr)))) | Selector.dispatch threshold small large => LexLeanTarget.TargetSyntax.Expr.cond (LexLeanTarget.TargetSyntax.Expr.prim (LexLeanTarget.TargetSyntax.Prim.natLt) ((LexLeanTarget.TargetSyntax.Expr.prim (LexLeanTarget.TargetSyntax.Prim.length) ((LexLeanTarget.TargetSyntax.Expr.var (0) :: ([] : List (LexLeanTarget.TargetSyntax.Expr)))) :: (LexLeanTarget.TargetSyntax.Expr.value (LexLeanTarget.TargetSyntax.Ty.nat) (LexLeanTarget.TargetSyntax.Value.nat (threshold)) :: ([] : List (LexLeanTarget.TargetSyntax.Expr)))))) (LexLeanTarget.TargetSyntax.Expr.call ((small + 1)) ((LexLeanTarget.TargetSyntax.Expr.var (0) :: ([] : List (LexLeanTarget.TargetSyntax.Expr))))) (LexLeanTarget.TargetSyntax.Expr.call ((large + 1)) ((LexLeanTarget.TargetSyntax.Expr.var (0) :: ([] : List (LexLeanTarget.TargetSyntax.Expr))))))

@[expose] public def planFunctions : (plans : List (Plan)) -> List (LexLeanTarget.TargetSyntax.Function)
  | List.nil => ([] : List (LexLeanTarget.TargetSyntax.Function))
  | List.cons plan rest => ((plan).function :: planFunctions (rest))

@[expose] public def realize (grammar : Grammar) (selector : Selector) : LexLeanTarget.TargetSyntax.Program := ({ adts := ([] : List (LexLeanTarget.TargetSyntax.Adt)), functions := (({ parameters := (0 :: ([] : List (Nat))), types := ((grammar).argument :: ([] : List (LexLeanTarget.TargetSyntax.Ty))), result := (grammar).result, body := entryBody (selector) } : LexLeanTarget.TargetSyntax.Function) :: planFunctions ((grammar).plans)) } : LexLeanTarget.TargetSyntax.Program)

mutual
@[expose] public def valueEq : (left : LexLeanTarget.TargetSyntax.Value) -> (right : LexLeanTarget.TargetSyntax.Value) -> Bool
  | LexLeanTarget.TargetSyntax.Value.unit, right => (match right with | LexLeanTarget.TargetSyntax.Value.unit => true | LexLeanTarget.TargetSyntax.Value.bool _ => false | LexLeanTarget.TargetSyntax.Value.nat _ => false | LexLeanTarget.TargetSyntax.Value.int _ => false | LexLeanTarget.TargetSyntax.Value.u8 _ => false | LexLeanTarget.TargetSyntax.Value.u16 _ => false | LexLeanTarget.TargetSyntax.Value.u32 _ => false | LexLeanTarget.TargetSyntax.Value.u64 _ => false | LexLeanTarget.TargetSyntax.Value.i8 _ => false | LexLeanTarget.TargetSyntax.Value.i16 _ => false | LexLeanTarget.TargetSyntax.Value.i32 _ => false | LexLeanTarget.TargetSyntax.Value.i64 _ => false | LexLeanTarget.TargetSyntax.Value.string _ => false | LexLeanTarget.TargetSyntax.Value.bytes _ => false | LexLeanTarget.TargetSyntax.Value.ordering _ => false | LexLeanTarget.TargetSyntax.Value.none => false | LexLeanTarget.TargetSyntax.Value.some _ => false | LexLeanTarget.TargetSyntax.Value.ok _ => false | LexLeanTarget.TargetSyntax.Value.error _ => false | LexLeanTarget.TargetSyntax.Value.list _ => false | LexLeanTarget.TargetSyntax.Value.pair _ _ => false | LexLeanTarget.TargetSyntax.Value.adt _ _ => false | LexLeanTarget.TargetSyntax.Value.closure _ _ => false)
  | LexLeanTarget.TargetSyntax.Value.bool leftItem, right => (match right with | LexLeanTarget.TargetSyntax.Value.unit => false | LexLeanTarget.TargetSyntax.Value.bool rightItem => (LexLeanRuntime.equal (leftItem) (rightItem) : Bool) | LexLeanTarget.TargetSyntax.Value.nat _ => false | LexLeanTarget.TargetSyntax.Value.int _ => false | LexLeanTarget.TargetSyntax.Value.u8 _ => false | LexLeanTarget.TargetSyntax.Value.u16 _ => false | LexLeanTarget.TargetSyntax.Value.u32 _ => false | LexLeanTarget.TargetSyntax.Value.u64 _ => false | LexLeanTarget.TargetSyntax.Value.i8 _ => false | LexLeanTarget.TargetSyntax.Value.i16 _ => false | LexLeanTarget.TargetSyntax.Value.i32 _ => false | LexLeanTarget.TargetSyntax.Value.i64 _ => false | LexLeanTarget.TargetSyntax.Value.string _ => false | LexLeanTarget.TargetSyntax.Value.bytes _ => false | LexLeanTarget.TargetSyntax.Value.ordering _ => false | LexLeanTarget.TargetSyntax.Value.none => false | LexLeanTarget.TargetSyntax.Value.some _ => false | LexLeanTarget.TargetSyntax.Value.ok _ => false | LexLeanTarget.TargetSyntax.Value.error _ => false | LexLeanTarget.TargetSyntax.Value.list _ => false | LexLeanTarget.TargetSyntax.Value.pair _ _ => false | LexLeanTarget.TargetSyntax.Value.adt _ _ => false | LexLeanTarget.TargetSyntax.Value.closure _ _ => false)
  | LexLeanTarget.TargetSyntax.Value.nat leftItem, right => (match right with | LexLeanTarget.TargetSyntax.Value.unit => false | LexLeanTarget.TargetSyntax.Value.bool _ => false | LexLeanTarget.TargetSyntax.Value.nat rightItem => (Nat.beq (leftItem) (rightItem)) | LexLeanTarget.TargetSyntax.Value.int _ => false | LexLeanTarget.TargetSyntax.Value.u8 _ => false | LexLeanTarget.TargetSyntax.Value.u16 _ => false | LexLeanTarget.TargetSyntax.Value.u32 _ => false | LexLeanTarget.TargetSyntax.Value.u64 _ => false | LexLeanTarget.TargetSyntax.Value.i8 _ => false | LexLeanTarget.TargetSyntax.Value.i16 _ => false | LexLeanTarget.TargetSyntax.Value.i32 _ => false | LexLeanTarget.TargetSyntax.Value.i64 _ => false | LexLeanTarget.TargetSyntax.Value.string _ => false | LexLeanTarget.TargetSyntax.Value.bytes _ => false | LexLeanTarget.TargetSyntax.Value.ordering _ => false | LexLeanTarget.TargetSyntax.Value.none => false | LexLeanTarget.TargetSyntax.Value.some _ => false | LexLeanTarget.TargetSyntax.Value.ok _ => false | LexLeanTarget.TargetSyntax.Value.error _ => false | LexLeanTarget.TargetSyntax.Value.list _ => false | LexLeanTarget.TargetSyntax.Value.pair _ _ => false | LexLeanTarget.TargetSyntax.Value.adt _ _ => false | LexLeanTarget.TargetSyntax.Value.closure _ _ => false)
  | LexLeanTarget.TargetSyntax.Value.int leftItem, right => (match right with | LexLeanTarget.TargetSyntax.Value.unit => false | LexLeanTarget.TargetSyntax.Value.bool _ => false | LexLeanTarget.TargetSyntax.Value.nat _ => false | LexLeanTarget.TargetSyntax.Value.int rightItem => LexLeanTarget.TargetSemantics.sameOrder (LexLeanTarget.TargetSemantics.orderInt (leftItem) (rightItem)) (LexLeanTarget.TargetSyntax.Order.same) | LexLeanTarget.TargetSyntax.Value.u8 _ => false | LexLeanTarget.TargetSyntax.Value.u16 _ => false | LexLeanTarget.TargetSyntax.Value.u32 _ => false | LexLeanTarget.TargetSyntax.Value.u64 _ => false | LexLeanTarget.TargetSyntax.Value.i8 _ => false | LexLeanTarget.TargetSyntax.Value.i16 _ => false | LexLeanTarget.TargetSyntax.Value.i32 _ => false | LexLeanTarget.TargetSyntax.Value.i64 _ => false | LexLeanTarget.TargetSyntax.Value.string _ => false | LexLeanTarget.TargetSyntax.Value.bytes _ => false | LexLeanTarget.TargetSyntax.Value.ordering _ => false | LexLeanTarget.TargetSyntax.Value.none => false | LexLeanTarget.TargetSyntax.Value.some _ => false | LexLeanTarget.TargetSyntax.Value.ok _ => false | LexLeanTarget.TargetSyntax.Value.error _ => false | LexLeanTarget.TargetSyntax.Value.list _ => false | LexLeanTarget.TargetSyntax.Value.pair _ _ => false | LexLeanTarget.TargetSyntax.Value.adt _ _ => false | LexLeanTarget.TargetSyntax.Value.closure _ _ => false)
  | LexLeanTarget.TargetSyntax.Value.u8 leftItem, right => (match right with | LexLeanTarget.TargetSyntax.Value.unit => false | LexLeanTarget.TargetSyntax.Value.bool _ => false | LexLeanTarget.TargetSyntax.Value.nat _ => false | LexLeanTarget.TargetSyntax.Value.int _ => false | LexLeanTarget.TargetSyntax.Value.u8 rightItem => (LexLeanRuntime.equal (leftItem) (rightItem) : Bool) | LexLeanTarget.TargetSyntax.Value.u16 _ => false | LexLeanTarget.TargetSyntax.Value.u32 _ => false | LexLeanTarget.TargetSyntax.Value.u64 _ => false | LexLeanTarget.TargetSyntax.Value.i8 _ => false | LexLeanTarget.TargetSyntax.Value.i16 _ => false | LexLeanTarget.TargetSyntax.Value.i32 _ => false | LexLeanTarget.TargetSyntax.Value.i64 _ => false | LexLeanTarget.TargetSyntax.Value.string _ => false | LexLeanTarget.TargetSyntax.Value.bytes _ => false | LexLeanTarget.TargetSyntax.Value.ordering _ => false | LexLeanTarget.TargetSyntax.Value.none => false | LexLeanTarget.TargetSyntax.Value.some _ => false | LexLeanTarget.TargetSyntax.Value.ok _ => false | LexLeanTarget.TargetSyntax.Value.error _ => false | LexLeanTarget.TargetSyntax.Value.list _ => false | LexLeanTarget.TargetSyntax.Value.pair _ _ => false | LexLeanTarget.TargetSyntax.Value.adt _ _ => false | LexLeanTarget.TargetSyntax.Value.closure _ _ => false)
  | LexLeanTarget.TargetSyntax.Value.u16 leftItem, right => (match right with | LexLeanTarget.TargetSyntax.Value.unit => false | LexLeanTarget.TargetSyntax.Value.bool _ => false | LexLeanTarget.TargetSyntax.Value.nat _ => false | LexLeanTarget.TargetSyntax.Value.int _ => false | LexLeanTarget.TargetSyntax.Value.u8 _ => false | LexLeanTarget.TargetSyntax.Value.u16 rightItem => (LexLeanRuntime.equal (leftItem) (rightItem) : Bool) | LexLeanTarget.TargetSyntax.Value.u32 _ => false | LexLeanTarget.TargetSyntax.Value.u64 _ => false | LexLeanTarget.TargetSyntax.Value.i8 _ => false | LexLeanTarget.TargetSyntax.Value.i16 _ => false | LexLeanTarget.TargetSyntax.Value.i32 _ => false | LexLeanTarget.TargetSyntax.Value.i64 _ => false | LexLeanTarget.TargetSyntax.Value.string _ => false | LexLeanTarget.TargetSyntax.Value.bytes _ => false | LexLeanTarget.TargetSyntax.Value.ordering _ => false | LexLeanTarget.TargetSyntax.Value.none => false | LexLeanTarget.TargetSyntax.Value.some _ => false | LexLeanTarget.TargetSyntax.Value.ok _ => false | LexLeanTarget.TargetSyntax.Value.error _ => false | LexLeanTarget.TargetSyntax.Value.list _ => false | LexLeanTarget.TargetSyntax.Value.pair _ _ => false | LexLeanTarget.TargetSyntax.Value.adt _ _ => false | LexLeanTarget.TargetSyntax.Value.closure _ _ => false)
  | LexLeanTarget.TargetSyntax.Value.u32 leftItem, right => (match right with | LexLeanTarget.TargetSyntax.Value.unit => false | LexLeanTarget.TargetSyntax.Value.bool _ => false | LexLeanTarget.TargetSyntax.Value.nat _ => false | LexLeanTarget.TargetSyntax.Value.int _ => false | LexLeanTarget.TargetSyntax.Value.u8 _ => false | LexLeanTarget.TargetSyntax.Value.u16 _ => false | LexLeanTarget.TargetSyntax.Value.u32 rightItem => (LexLeanRuntime.equal (leftItem) (rightItem) : Bool) | LexLeanTarget.TargetSyntax.Value.u64 _ => false | LexLeanTarget.TargetSyntax.Value.i8 _ => false | LexLeanTarget.TargetSyntax.Value.i16 _ => false | LexLeanTarget.TargetSyntax.Value.i32 _ => false | LexLeanTarget.TargetSyntax.Value.i64 _ => false | LexLeanTarget.TargetSyntax.Value.string _ => false | LexLeanTarget.TargetSyntax.Value.bytes _ => false | LexLeanTarget.TargetSyntax.Value.ordering _ => false | LexLeanTarget.TargetSyntax.Value.none => false | LexLeanTarget.TargetSyntax.Value.some _ => false | LexLeanTarget.TargetSyntax.Value.ok _ => false | LexLeanTarget.TargetSyntax.Value.error _ => false | LexLeanTarget.TargetSyntax.Value.list _ => false | LexLeanTarget.TargetSyntax.Value.pair _ _ => false | LexLeanTarget.TargetSyntax.Value.adt _ _ => false | LexLeanTarget.TargetSyntax.Value.closure _ _ => false)
  | LexLeanTarget.TargetSyntax.Value.u64 leftItem, right => (match right with | LexLeanTarget.TargetSyntax.Value.unit => false | LexLeanTarget.TargetSyntax.Value.bool _ => false | LexLeanTarget.TargetSyntax.Value.nat _ => false | LexLeanTarget.TargetSyntax.Value.int _ => false | LexLeanTarget.TargetSyntax.Value.u8 _ => false | LexLeanTarget.TargetSyntax.Value.u16 _ => false | LexLeanTarget.TargetSyntax.Value.u32 _ => false | LexLeanTarget.TargetSyntax.Value.u64 rightItem => (LexLeanRuntime.equal (leftItem) (rightItem) : Bool) | LexLeanTarget.TargetSyntax.Value.i8 _ => false | LexLeanTarget.TargetSyntax.Value.i16 _ => false | LexLeanTarget.TargetSyntax.Value.i32 _ => false | LexLeanTarget.TargetSyntax.Value.i64 _ => false | LexLeanTarget.TargetSyntax.Value.string _ => false | LexLeanTarget.TargetSyntax.Value.bytes _ => false | LexLeanTarget.TargetSyntax.Value.ordering _ => false | LexLeanTarget.TargetSyntax.Value.none => false | LexLeanTarget.TargetSyntax.Value.some _ => false | LexLeanTarget.TargetSyntax.Value.ok _ => false | LexLeanTarget.TargetSyntax.Value.error _ => false | LexLeanTarget.TargetSyntax.Value.list _ => false | LexLeanTarget.TargetSyntax.Value.pair _ _ => false | LexLeanTarget.TargetSyntax.Value.adt _ _ => false | LexLeanTarget.TargetSyntax.Value.closure _ _ => false)
  | LexLeanTarget.TargetSyntax.Value.i8 leftItem, right => (match right with | LexLeanTarget.TargetSyntax.Value.unit => false | LexLeanTarget.TargetSyntax.Value.bool _ => false | LexLeanTarget.TargetSyntax.Value.nat _ => false | LexLeanTarget.TargetSyntax.Value.int _ => false | LexLeanTarget.TargetSyntax.Value.u8 _ => false | LexLeanTarget.TargetSyntax.Value.u16 _ => false | LexLeanTarget.TargetSyntax.Value.u32 _ => false | LexLeanTarget.TargetSyntax.Value.u64 _ => false | LexLeanTarget.TargetSyntax.Value.i8 rightItem => (LexLeanRuntime.equal (leftItem) (rightItem) : Bool) | LexLeanTarget.TargetSyntax.Value.i16 _ => false | LexLeanTarget.TargetSyntax.Value.i32 _ => false | LexLeanTarget.TargetSyntax.Value.i64 _ => false | LexLeanTarget.TargetSyntax.Value.string _ => false | LexLeanTarget.TargetSyntax.Value.bytes _ => false | LexLeanTarget.TargetSyntax.Value.ordering _ => false | LexLeanTarget.TargetSyntax.Value.none => false | LexLeanTarget.TargetSyntax.Value.some _ => false | LexLeanTarget.TargetSyntax.Value.ok _ => false | LexLeanTarget.TargetSyntax.Value.error _ => false | LexLeanTarget.TargetSyntax.Value.list _ => false | LexLeanTarget.TargetSyntax.Value.pair _ _ => false | LexLeanTarget.TargetSyntax.Value.adt _ _ => false | LexLeanTarget.TargetSyntax.Value.closure _ _ => false)
  | LexLeanTarget.TargetSyntax.Value.i16 leftItem, right => (match right with | LexLeanTarget.TargetSyntax.Value.unit => false | LexLeanTarget.TargetSyntax.Value.bool _ => false | LexLeanTarget.TargetSyntax.Value.nat _ => false | LexLeanTarget.TargetSyntax.Value.int _ => false | LexLeanTarget.TargetSyntax.Value.u8 _ => false | LexLeanTarget.TargetSyntax.Value.u16 _ => false | LexLeanTarget.TargetSyntax.Value.u32 _ => false | LexLeanTarget.TargetSyntax.Value.u64 _ => false | LexLeanTarget.TargetSyntax.Value.i8 _ => false | LexLeanTarget.TargetSyntax.Value.i16 rightItem => (LexLeanRuntime.equal (leftItem) (rightItem) : Bool) | LexLeanTarget.TargetSyntax.Value.i32 _ => false | LexLeanTarget.TargetSyntax.Value.i64 _ => false | LexLeanTarget.TargetSyntax.Value.string _ => false | LexLeanTarget.TargetSyntax.Value.bytes _ => false | LexLeanTarget.TargetSyntax.Value.ordering _ => false | LexLeanTarget.TargetSyntax.Value.none => false | LexLeanTarget.TargetSyntax.Value.some _ => false | LexLeanTarget.TargetSyntax.Value.ok _ => false | LexLeanTarget.TargetSyntax.Value.error _ => false | LexLeanTarget.TargetSyntax.Value.list _ => false | LexLeanTarget.TargetSyntax.Value.pair _ _ => false | LexLeanTarget.TargetSyntax.Value.adt _ _ => false | LexLeanTarget.TargetSyntax.Value.closure _ _ => false)
  | LexLeanTarget.TargetSyntax.Value.i32 leftItem, right => (match right with | LexLeanTarget.TargetSyntax.Value.unit => false | LexLeanTarget.TargetSyntax.Value.bool _ => false | LexLeanTarget.TargetSyntax.Value.nat _ => false | LexLeanTarget.TargetSyntax.Value.int _ => false | LexLeanTarget.TargetSyntax.Value.u8 _ => false | LexLeanTarget.TargetSyntax.Value.u16 _ => false | LexLeanTarget.TargetSyntax.Value.u32 _ => false | LexLeanTarget.TargetSyntax.Value.u64 _ => false | LexLeanTarget.TargetSyntax.Value.i8 _ => false | LexLeanTarget.TargetSyntax.Value.i16 _ => false | LexLeanTarget.TargetSyntax.Value.i32 rightItem => (LexLeanRuntime.equal (leftItem) (rightItem) : Bool) | LexLeanTarget.TargetSyntax.Value.i64 _ => false | LexLeanTarget.TargetSyntax.Value.string _ => false | LexLeanTarget.TargetSyntax.Value.bytes _ => false | LexLeanTarget.TargetSyntax.Value.ordering _ => false | LexLeanTarget.TargetSyntax.Value.none => false | LexLeanTarget.TargetSyntax.Value.some _ => false | LexLeanTarget.TargetSyntax.Value.ok _ => false | LexLeanTarget.TargetSyntax.Value.error _ => false | LexLeanTarget.TargetSyntax.Value.list _ => false | LexLeanTarget.TargetSyntax.Value.pair _ _ => false | LexLeanTarget.TargetSyntax.Value.adt _ _ => false | LexLeanTarget.TargetSyntax.Value.closure _ _ => false)
  | LexLeanTarget.TargetSyntax.Value.i64 leftItem, right => (match right with | LexLeanTarget.TargetSyntax.Value.unit => false | LexLeanTarget.TargetSyntax.Value.bool _ => false | LexLeanTarget.TargetSyntax.Value.nat _ => false | LexLeanTarget.TargetSyntax.Value.int _ => false | LexLeanTarget.TargetSyntax.Value.u8 _ => false | LexLeanTarget.TargetSyntax.Value.u16 _ => false | LexLeanTarget.TargetSyntax.Value.u32 _ => false | LexLeanTarget.TargetSyntax.Value.u64 _ => false | LexLeanTarget.TargetSyntax.Value.i8 _ => false | LexLeanTarget.TargetSyntax.Value.i16 _ => false | LexLeanTarget.TargetSyntax.Value.i32 _ => false | LexLeanTarget.TargetSyntax.Value.i64 rightItem => (LexLeanRuntime.equal (leftItem) (rightItem) : Bool) | LexLeanTarget.TargetSyntax.Value.string _ => false | LexLeanTarget.TargetSyntax.Value.bytes _ => false | LexLeanTarget.TargetSyntax.Value.ordering _ => false | LexLeanTarget.TargetSyntax.Value.none => false | LexLeanTarget.TargetSyntax.Value.some _ => false | LexLeanTarget.TargetSyntax.Value.ok _ => false | LexLeanTarget.TargetSyntax.Value.error _ => false | LexLeanTarget.TargetSyntax.Value.list _ => false | LexLeanTarget.TargetSyntax.Value.pair _ _ => false | LexLeanTarget.TargetSyntax.Value.adt _ _ => false | LexLeanTarget.TargetSyntax.Value.closure _ _ => false)
  | LexLeanTarget.TargetSyntax.Value.string leftItem, right => (match right with | LexLeanTarget.TargetSyntax.Value.unit => false | LexLeanTarget.TargetSyntax.Value.bool _ => false | LexLeanTarget.TargetSyntax.Value.nat _ => false | LexLeanTarget.TargetSyntax.Value.int _ => false | LexLeanTarget.TargetSyntax.Value.u8 _ => false | LexLeanTarget.TargetSyntax.Value.u16 _ => false | LexLeanTarget.TargetSyntax.Value.u32 _ => false | LexLeanTarget.TargetSyntax.Value.u64 _ => false | LexLeanTarget.TargetSyntax.Value.i8 _ => false | LexLeanTarget.TargetSyntax.Value.i16 _ => false | LexLeanTarget.TargetSyntax.Value.i32 _ => false | LexLeanTarget.TargetSyntax.Value.i64 _ => false | LexLeanTarget.TargetSyntax.Value.string rightItem => (LexLeanRuntime.equal (leftItem) (rightItem) : Bool) | LexLeanTarget.TargetSyntax.Value.bytes _ => false | LexLeanTarget.TargetSyntax.Value.ordering _ => false | LexLeanTarget.TargetSyntax.Value.none => false | LexLeanTarget.TargetSyntax.Value.some _ => false | LexLeanTarget.TargetSyntax.Value.ok _ => false | LexLeanTarget.TargetSyntax.Value.error _ => false | LexLeanTarget.TargetSyntax.Value.list _ => false | LexLeanTarget.TargetSyntax.Value.pair _ _ => false | LexLeanTarget.TargetSyntax.Value.adt _ _ => false | LexLeanTarget.TargetSyntax.Value.closure _ _ => false)
  | LexLeanTarget.TargetSyntax.Value.bytes leftItem, right => (match right with | LexLeanTarget.TargetSyntax.Value.unit => false | LexLeanTarget.TargetSyntax.Value.bool _ => false | LexLeanTarget.TargetSyntax.Value.nat _ => false | LexLeanTarget.TargetSyntax.Value.int _ => false | LexLeanTarget.TargetSyntax.Value.u8 _ => false | LexLeanTarget.TargetSyntax.Value.u16 _ => false | LexLeanTarget.TargetSyntax.Value.u32 _ => false | LexLeanTarget.TargetSyntax.Value.u64 _ => false | LexLeanTarget.TargetSyntax.Value.i8 _ => false | LexLeanTarget.TargetSyntax.Value.i16 _ => false | LexLeanTarget.TargetSyntax.Value.i32 _ => false | LexLeanTarget.TargetSyntax.Value.i64 _ => false | LexLeanTarget.TargetSyntax.Value.string _ => false | LexLeanTarget.TargetSyntax.Value.bytes rightItem => (LexLeanRuntime.equal (leftItem) (rightItem) : Bool) | LexLeanTarget.TargetSyntax.Value.ordering _ => false | LexLeanTarget.TargetSyntax.Value.none => false | LexLeanTarget.TargetSyntax.Value.some _ => false | LexLeanTarget.TargetSyntax.Value.ok _ => false | LexLeanTarget.TargetSyntax.Value.error _ => false | LexLeanTarget.TargetSyntax.Value.list _ => false | LexLeanTarget.TargetSyntax.Value.pair _ _ => false | LexLeanTarget.TargetSyntax.Value.adt _ _ => false | LexLeanTarget.TargetSyntax.Value.closure _ _ => false)
  | LexLeanTarget.TargetSyntax.Value.ordering leftItem, right => (match right with | LexLeanTarget.TargetSyntax.Value.unit => false | LexLeanTarget.TargetSyntax.Value.bool _ => false | LexLeanTarget.TargetSyntax.Value.nat _ => false | LexLeanTarget.TargetSyntax.Value.int _ => false | LexLeanTarget.TargetSyntax.Value.u8 _ => false | LexLeanTarget.TargetSyntax.Value.u16 _ => false | LexLeanTarget.TargetSyntax.Value.u32 _ => false | LexLeanTarget.TargetSyntax.Value.u64 _ => false | LexLeanTarget.TargetSyntax.Value.i8 _ => false | LexLeanTarget.TargetSyntax.Value.i16 _ => false | LexLeanTarget.TargetSyntax.Value.i32 _ => false | LexLeanTarget.TargetSyntax.Value.i64 _ => false | LexLeanTarget.TargetSyntax.Value.string _ => false | LexLeanTarget.TargetSyntax.Value.bytes _ => false | LexLeanTarget.TargetSyntax.Value.ordering rightItem => LexLeanTarget.TargetSemantics.sameOrder (leftItem) (rightItem) | LexLeanTarget.TargetSyntax.Value.none => false | LexLeanTarget.TargetSyntax.Value.some _ => false | LexLeanTarget.TargetSyntax.Value.ok _ => false | LexLeanTarget.TargetSyntax.Value.error _ => false | LexLeanTarget.TargetSyntax.Value.list _ => false | LexLeanTarget.TargetSyntax.Value.pair _ _ => false | LexLeanTarget.TargetSyntax.Value.adt _ _ => false | LexLeanTarget.TargetSyntax.Value.closure _ _ => false)
  | LexLeanTarget.TargetSyntax.Value.none, right => (match right with | LexLeanTarget.TargetSyntax.Value.unit => false | LexLeanTarget.TargetSyntax.Value.bool _ => false | LexLeanTarget.TargetSyntax.Value.nat _ => false | LexLeanTarget.TargetSyntax.Value.int _ => false | LexLeanTarget.TargetSyntax.Value.u8 _ => false | LexLeanTarget.TargetSyntax.Value.u16 _ => false | LexLeanTarget.TargetSyntax.Value.u32 _ => false | LexLeanTarget.TargetSyntax.Value.u64 _ => false | LexLeanTarget.TargetSyntax.Value.i8 _ => false | LexLeanTarget.TargetSyntax.Value.i16 _ => false | LexLeanTarget.TargetSyntax.Value.i32 _ => false | LexLeanTarget.TargetSyntax.Value.i64 _ => false | LexLeanTarget.TargetSyntax.Value.string _ => false | LexLeanTarget.TargetSyntax.Value.bytes _ => false | LexLeanTarget.TargetSyntax.Value.ordering _ => false | LexLeanTarget.TargetSyntax.Value.none => true | LexLeanTarget.TargetSyntax.Value.some _ => false | LexLeanTarget.TargetSyntax.Value.ok _ => false | LexLeanTarget.TargetSyntax.Value.error _ => false | LexLeanTarget.TargetSyntax.Value.list _ => false | LexLeanTarget.TargetSyntax.Value.pair _ _ => false | LexLeanTarget.TargetSyntax.Value.adt _ _ => false | LexLeanTarget.TargetSyntax.Value.closure _ _ => false)
  | LexLeanTarget.TargetSyntax.Value.some leftItem, right => (match right with | LexLeanTarget.TargetSyntax.Value.unit => false | LexLeanTarget.TargetSyntax.Value.bool _ => false | LexLeanTarget.TargetSyntax.Value.nat _ => false | LexLeanTarget.TargetSyntax.Value.int _ => false | LexLeanTarget.TargetSyntax.Value.u8 _ => false | LexLeanTarget.TargetSyntax.Value.u16 _ => false | LexLeanTarget.TargetSyntax.Value.u32 _ => false | LexLeanTarget.TargetSyntax.Value.u64 _ => false | LexLeanTarget.TargetSyntax.Value.i8 _ => false | LexLeanTarget.TargetSyntax.Value.i16 _ => false | LexLeanTarget.TargetSyntax.Value.i32 _ => false | LexLeanTarget.TargetSyntax.Value.i64 _ => false | LexLeanTarget.TargetSyntax.Value.string _ => false | LexLeanTarget.TargetSyntax.Value.bytes _ => false | LexLeanTarget.TargetSyntax.Value.ordering _ => false | LexLeanTarget.TargetSyntax.Value.none => false | LexLeanTarget.TargetSyntax.Value.some rightItem => valueEq (leftItem) (rightItem) | LexLeanTarget.TargetSyntax.Value.ok _ => false | LexLeanTarget.TargetSyntax.Value.error _ => false | LexLeanTarget.TargetSyntax.Value.list _ => false | LexLeanTarget.TargetSyntax.Value.pair _ _ => false | LexLeanTarget.TargetSyntax.Value.adt _ _ => false | LexLeanTarget.TargetSyntax.Value.closure _ _ => false)
  | LexLeanTarget.TargetSyntax.Value.ok leftItem, right => (match right with | LexLeanTarget.TargetSyntax.Value.unit => false | LexLeanTarget.TargetSyntax.Value.bool _ => false | LexLeanTarget.TargetSyntax.Value.nat _ => false | LexLeanTarget.TargetSyntax.Value.int _ => false | LexLeanTarget.TargetSyntax.Value.u8 _ => false | LexLeanTarget.TargetSyntax.Value.u16 _ => false | LexLeanTarget.TargetSyntax.Value.u32 _ => false | LexLeanTarget.TargetSyntax.Value.u64 _ => false | LexLeanTarget.TargetSyntax.Value.i8 _ => false | LexLeanTarget.TargetSyntax.Value.i16 _ => false | LexLeanTarget.TargetSyntax.Value.i32 _ => false | LexLeanTarget.TargetSyntax.Value.i64 _ => false | LexLeanTarget.TargetSyntax.Value.string _ => false | LexLeanTarget.TargetSyntax.Value.bytes _ => false | LexLeanTarget.TargetSyntax.Value.ordering _ => false | LexLeanTarget.TargetSyntax.Value.none => false | LexLeanTarget.TargetSyntax.Value.some _ => false | LexLeanTarget.TargetSyntax.Value.ok rightItem => valueEq (leftItem) (rightItem) | LexLeanTarget.TargetSyntax.Value.error _ => false | LexLeanTarget.TargetSyntax.Value.list _ => false | LexLeanTarget.TargetSyntax.Value.pair _ _ => false | LexLeanTarget.TargetSyntax.Value.adt _ _ => false | LexLeanTarget.TargetSyntax.Value.closure _ _ => false)
  | LexLeanTarget.TargetSyntax.Value.error leftItem, right => (match right with | LexLeanTarget.TargetSyntax.Value.unit => false | LexLeanTarget.TargetSyntax.Value.bool _ => false | LexLeanTarget.TargetSyntax.Value.nat _ => false | LexLeanTarget.TargetSyntax.Value.int _ => false | LexLeanTarget.TargetSyntax.Value.u8 _ => false | LexLeanTarget.TargetSyntax.Value.u16 _ => false | LexLeanTarget.TargetSyntax.Value.u32 _ => false | LexLeanTarget.TargetSyntax.Value.u64 _ => false | LexLeanTarget.TargetSyntax.Value.i8 _ => false | LexLeanTarget.TargetSyntax.Value.i16 _ => false | LexLeanTarget.TargetSyntax.Value.i32 _ => false | LexLeanTarget.TargetSyntax.Value.i64 _ => false | LexLeanTarget.TargetSyntax.Value.string _ => false | LexLeanTarget.TargetSyntax.Value.bytes _ => false | LexLeanTarget.TargetSyntax.Value.ordering _ => false | LexLeanTarget.TargetSyntax.Value.none => false | LexLeanTarget.TargetSyntax.Value.some _ => false | LexLeanTarget.TargetSyntax.Value.ok _ => false | LexLeanTarget.TargetSyntax.Value.error rightItem => valueEq (leftItem) (rightItem) | LexLeanTarget.TargetSyntax.Value.list _ => false | LexLeanTarget.TargetSyntax.Value.pair _ _ => false | LexLeanTarget.TargetSyntax.Value.adt _ _ => false | LexLeanTarget.TargetSyntax.Value.closure _ _ => false)
  | LexLeanTarget.TargetSyntax.Value.list leftItem, right => (match right with | LexLeanTarget.TargetSyntax.Value.unit => false | LexLeanTarget.TargetSyntax.Value.bool _ => false | LexLeanTarget.TargetSyntax.Value.nat _ => false | LexLeanTarget.TargetSyntax.Value.int _ => false | LexLeanTarget.TargetSyntax.Value.u8 _ => false | LexLeanTarget.TargetSyntax.Value.u16 _ => false | LexLeanTarget.TargetSyntax.Value.u32 _ => false | LexLeanTarget.TargetSyntax.Value.u64 _ => false | LexLeanTarget.TargetSyntax.Value.i8 _ => false | LexLeanTarget.TargetSyntax.Value.i16 _ => false | LexLeanTarget.TargetSyntax.Value.i32 _ => false | LexLeanTarget.TargetSyntax.Value.i64 _ => false | LexLeanTarget.TargetSyntax.Value.string _ => false | LexLeanTarget.TargetSyntax.Value.bytes _ => false | LexLeanTarget.TargetSyntax.Value.ordering _ => false | LexLeanTarget.TargetSyntax.Value.none => false | LexLeanTarget.TargetSyntax.Value.some _ => false | LexLeanTarget.TargetSyntax.Value.ok _ => false | LexLeanTarget.TargetSyntax.Value.error _ => false | LexLeanTarget.TargetSyntax.Value.list rightItem => valuesEq (leftItem) (rightItem) | LexLeanTarget.TargetSyntax.Value.pair _ _ => false | LexLeanTarget.TargetSyntax.Value.adt _ _ => false | LexLeanTarget.TargetSyntax.Value.closure _ _ => false)
  | LexLeanTarget.TargetSyntax.Value.pair leftFirst leftSecond, right => (match right with | LexLeanTarget.TargetSyntax.Value.unit => false | LexLeanTarget.TargetSyntax.Value.bool _ => false | LexLeanTarget.TargetSyntax.Value.nat _ => false | LexLeanTarget.TargetSyntax.Value.int _ => false | LexLeanTarget.TargetSyntax.Value.u8 _ => false | LexLeanTarget.TargetSyntax.Value.u16 _ => false | LexLeanTarget.TargetSyntax.Value.u32 _ => false | LexLeanTarget.TargetSyntax.Value.u64 _ => false | LexLeanTarget.TargetSyntax.Value.i8 _ => false | LexLeanTarget.TargetSyntax.Value.i16 _ => false | LexLeanTarget.TargetSyntax.Value.i32 _ => false | LexLeanTarget.TargetSyntax.Value.i64 _ => false | LexLeanTarget.TargetSyntax.Value.string _ => false | LexLeanTarget.TargetSyntax.Value.bytes _ => false | LexLeanTarget.TargetSyntax.Value.ordering _ => false | LexLeanTarget.TargetSyntax.Value.none => false | LexLeanTarget.TargetSyntax.Value.some _ => false | LexLeanTarget.TargetSyntax.Value.ok _ => false | LexLeanTarget.TargetSyntax.Value.error _ => false | LexLeanTarget.TargetSyntax.Value.list _ => false | LexLeanTarget.TargetSyntax.Value.pair rightFirst rightSecond => (valueEq (leftFirst) (rightFirst) && valueEq (leftSecond) (rightSecond)) | LexLeanTarget.TargetSyntax.Value.adt _ _ => false | LexLeanTarget.TargetSyntax.Value.closure _ _ => false)
  | LexLeanTarget.TargetSyntax.Value.adt leftTag leftFields, right => (match right with | LexLeanTarget.TargetSyntax.Value.unit => false | LexLeanTarget.TargetSyntax.Value.bool _ => false | LexLeanTarget.TargetSyntax.Value.nat _ => false | LexLeanTarget.TargetSyntax.Value.int _ => false | LexLeanTarget.TargetSyntax.Value.u8 _ => false | LexLeanTarget.TargetSyntax.Value.u16 _ => false | LexLeanTarget.TargetSyntax.Value.u32 _ => false | LexLeanTarget.TargetSyntax.Value.u64 _ => false | LexLeanTarget.TargetSyntax.Value.i8 _ => false | LexLeanTarget.TargetSyntax.Value.i16 _ => false | LexLeanTarget.TargetSyntax.Value.i32 _ => false | LexLeanTarget.TargetSyntax.Value.i64 _ => false | LexLeanTarget.TargetSyntax.Value.string _ => false | LexLeanTarget.TargetSyntax.Value.bytes _ => false | LexLeanTarget.TargetSyntax.Value.ordering _ => false | LexLeanTarget.TargetSyntax.Value.none => false | LexLeanTarget.TargetSyntax.Value.some _ => false | LexLeanTarget.TargetSyntax.Value.ok _ => false | LexLeanTarget.TargetSyntax.Value.error _ => false | LexLeanTarget.TargetSyntax.Value.list _ => false | LexLeanTarget.TargetSyntax.Value.pair _ _ => false | LexLeanTarget.TargetSyntax.Value.adt rightTag rightFields => ((Nat.beq (leftTag) (rightTag)) && valuesEq (leftFields) (rightFields)) | LexLeanTarget.TargetSyntax.Value.closure _ _ => false)
  | LexLeanTarget.TargetSyntax.Value.closure leftFunction leftCaptures, right => (match right with | LexLeanTarget.TargetSyntax.Value.unit => false | LexLeanTarget.TargetSyntax.Value.bool _ => false | LexLeanTarget.TargetSyntax.Value.nat _ => false | LexLeanTarget.TargetSyntax.Value.int _ => false | LexLeanTarget.TargetSyntax.Value.u8 _ => false | LexLeanTarget.TargetSyntax.Value.u16 _ => false | LexLeanTarget.TargetSyntax.Value.u32 _ => false | LexLeanTarget.TargetSyntax.Value.u64 _ => false | LexLeanTarget.TargetSyntax.Value.i8 _ => false | LexLeanTarget.TargetSyntax.Value.i16 _ => false | LexLeanTarget.TargetSyntax.Value.i32 _ => false | LexLeanTarget.TargetSyntax.Value.i64 _ => false | LexLeanTarget.TargetSyntax.Value.string _ => false | LexLeanTarget.TargetSyntax.Value.bytes _ => false | LexLeanTarget.TargetSyntax.Value.ordering _ => false | LexLeanTarget.TargetSyntax.Value.none => false | LexLeanTarget.TargetSyntax.Value.some _ => false | LexLeanTarget.TargetSyntax.Value.ok _ => false | LexLeanTarget.TargetSyntax.Value.error _ => false | LexLeanTarget.TargetSyntax.Value.list _ => false | LexLeanTarget.TargetSyntax.Value.pair _ _ => false | LexLeanTarget.TargetSyntax.Value.adt _ _ => false | LexLeanTarget.TargetSyntax.Value.closure rightFunction rightCaptures => ((Nat.beq (leftFunction) (rightFunction)) && valuesEq (leftCaptures) (rightCaptures)))
termination_by structural left _ => left

@[expose] public def valuesEq : (left : List (LexLeanTarget.TargetSyntax.Value)) -> (right : List (LexLeanTarget.TargetSyntax.Value)) -> Bool
  | List.nil, right => (match right with | List.nil => true | List.cons _ _ => false)
  | List.cons leftHead leftTail, right => (match right with | List.nil => false | List.cons rightHead rightTail => (valueEq (leftHead) (rightHead) && valuesEq (leftTail) (rightTail)))
termination_by structural left _ => left
end

mutual
@[expose] public def exprSize : (expression : LexLeanTarget.TargetSyntax.Expr) -> Nat
  | LexLeanTarget.TargetSyntax.Expr.value _ _ => 1
  | LexLeanTarget.TargetSyntax.Expr.var _ => 1
  | LexLeanTarget.TargetSyntax.Expr.«let» _ _ bound body => (1 + (exprSize (bound) + exprSize (body)))
  | LexLeanTarget.TargetSyntax.Expr.cond condition thenBranch elseBranch => (1 + (exprSize (condition) + (exprSize (thenBranch) + exprSize (elseBranch))))
  | LexLeanTarget.TargetSyntax.Expr.«match» _ scrutinee arms => (1 + (exprSize (scrutinee) + armsSize (arms)))
  | LexLeanTarget.TargetSyntax.Expr.build _ _ operands => (1 + exprsSize (operands))
  | LexLeanTarget.TargetSyntax.Expr.call _ operands => (1 + exprsSize (operands))
  | LexLeanTarget.TargetSyntax.Expr.closure _ operands => (1 + exprsSize (operands))
  | LexLeanTarget.TargetSyntax.Expr.apply target operands => (1 + (exprSize (target) + exprsSize (operands)))
  | LexLeanTarget.TargetSyntax.Expr.prim _ operands => (1 + exprsSize (operands))
  | LexLeanTarget.TargetSyntax.Expr.first inner => (1 + exprSize (inner))
  | LexLeanTarget.TargetSyntax.Expr.second inner => (1 + exprSize (inner))
  | LexLeanTarget.TargetSyntax.Expr.field inner _ => (1 + exprSize (inner))
termination_by structural expression => expression

@[expose] public def exprsSize : (expressions : List (LexLeanTarget.TargetSyntax.Expr)) -> Nat
  | List.nil => 0
  | List.cons head rest => (exprSize (head) + exprsSize (rest))
termination_by structural expressions => expressions

@[expose] public def armsSize : (arms : List (LexLeanTarget.TargetSyntax.Arm)) -> Nat
  | List.nil => 0
  | List.cons head rest => (armSize (head) + armsSize (rest))
termination_by structural arms => arms

@[expose] public def armSize : (arm : LexLeanTarget.TargetSyntax.Arm) -> Nat
  | LexLeanTarget.TargetSyntax.Arm.arm _ _ body => (1 + exprSize (body))
termination_by structural arm => arm
end

mutual
@[expose] public def exprCallees : (expression : LexLeanTarget.TargetSyntax.Expr) -> List (Nat)
  | LexLeanTarget.TargetSyntax.Expr.value _ _ => ([] : List (Nat))
  | LexLeanTarget.TargetSyntax.Expr.var _ => ([] : List (Nat))
  | LexLeanTarget.TargetSyntax.Expr.«let» _ _ bound body => appendAll (Nat) (([] : List (Nat))) (appendAll (Nat) (exprCallees (bound)) (exprCallees (body)))
  | LexLeanTarget.TargetSyntax.Expr.cond condition thenBranch elseBranch => appendAll (Nat) (([] : List (Nat))) (appendAll (Nat) (exprCallees (condition)) (appendAll (Nat) (exprCallees (thenBranch)) (exprCallees (elseBranch))))
  | LexLeanTarget.TargetSyntax.Expr.«match» _ scrutinee arms => appendAll (Nat) (([] : List (Nat))) (appendAll (Nat) (exprCallees (scrutinee)) (armsCallees (arms)))
  | LexLeanTarget.TargetSyntax.Expr.build _ _ operands => appendAll (Nat) (([] : List (Nat))) (exprsCallees (operands))
  | LexLeanTarget.TargetSyntax.Expr.call function operands => appendAll (Nat) ((function :: ([] : List (Nat)))) (exprsCallees (operands))
  | LexLeanTarget.TargetSyntax.Expr.closure function operands => appendAll (Nat) ((function :: ([] : List (Nat)))) (exprsCallees (operands))
  | LexLeanTarget.TargetSyntax.Expr.apply target operands => appendAll (Nat) (([] : List (Nat))) (appendAll (Nat) (exprCallees (target)) (exprsCallees (operands)))
  | LexLeanTarget.TargetSyntax.Expr.prim _ operands => appendAll (Nat) (([] : List (Nat))) (exprsCallees (operands))
  | LexLeanTarget.TargetSyntax.Expr.first inner => appendAll (Nat) (([] : List (Nat))) (exprCallees (inner))
  | LexLeanTarget.TargetSyntax.Expr.second inner => appendAll (Nat) (([] : List (Nat))) (exprCallees (inner))
  | LexLeanTarget.TargetSyntax.Expr.field inner _ => appendAll (Nat) (([] : List (Nat))) (exprCallees (inner))
termination_by structural expression => expression

@[expose] public def exprsCallees : (expressions : List (LexLeanTarget.TargetSyntax.Expr)) -> List (Nat)
  | List.nil => ([] : List (Nat))
  | List.cons head rest => appendAll (Nat) (exprCallees (head)) (exprsCallees (rest))
termination_by structural expressions => expressions

@[expose] public def armsCallees : (arms : List (LexLeanTarget.TargetSyntax.Arm)) -> List (Nat)
  | List.nil => ([] : List (Nat))
  | List.cons head rest => appendAll (Nat) (armCallees (head)) (armsCallees (rest))
termination_by structural arms => arms

@[expose] public def armCallees : (arm : LexLeanTarget.TargetSyntax.Arm) -> List (Nat)
  | LexLeanTarget.TargetSyntax.Arm.arm _ _ body => appendAll (Nat) (([] : List (Nat))) (exprCallees (body))
termination_by structural arm => arm
end

@[expose] public def functionAt : (items : List (LexLeanTarget.TargetSyntax.Function)) -> (index : Nat) -> Option (LexLeanTarget.TargetSyntax.Function)
  | List.nil, _index => Option.none
  | List.cons head rest, index => (match index with | Nat.zero => Option.some (head) | Nat.succ remaining => functionAt (rest) (remaining))

@[expose] public def planAt : (items : List (Plan)) -> (index : Nat) -> Option (Plan)
  | List.nil, _index => Option.none
  | List.cons head rest, index => (match index with | Nat.zero => Option.some (head) | Nat.succ remaining => planAt (rest) (remaining))

@[expose] public def calleesOf : (functions : List (LexLeanTarget.TargetSyntax.Function)) -> (known : List (Nat)) -> List (Nat)
  | _functions, List.nil => ([] : List (Nat))
  | functions, List.cons index rest => appendAll (Nat) ((match functionAt (functions) (index) with | Option.none => ([] : List (Nat)) | Option.some function => exprCallees ((function).body))) (calleesOf (functions) (rest))

@[expose] public def addNew : (known : List (Nat)) -> (found : List (Nat)) -> List (Nat)
  | known, List.nil => known
  | known, List.cons index rest => (if natIn (index) (known) then addNew (known) (rest) else addNew (appendAll (Nat) (known) ((index :: ([] : List (Nat))))) (rest))

@[expose] public def closeOver : (rounds : Nat) -> (functions : List (LexLeanTarget.TargetSyntax.Function)) -> (known : List (Nat)) -> List (Nat)
  | Nat.zero, _functions, known => known
  | Nat.succ remaining, functions, known => closeOver (remaining) (functions) (addNew (known) (calleesOf (functions) (known)))

@[expose] public def reachable (functions : List (LexLeanTarget.TargetSyntax.Function)) : List (Nat) := closeOver ((LexLeanRuntime.length (functions) : Nat)) (functions) ((0 :: ([] : List (Nat))))

@[expose] public def reachableSize : (functions : List (LexLeanTarget.TargetSyntax.Function)) -> (indices : List (Nat)) -> Nat
  | _functions, List.nil => 0
  | functions, List.cons index rest => ((match functionAt (functions) (index) with | Option.none => 0 | Option.some function => exprSize ((function).body)) + reachableSize (functions) (rest))

@[expose] public def reachableCharge : (actions : List (Action)) -> (plans : List (Plan)) -> (indices : List (Nat)) -> Nat
  | _actions, _plans, List.nil => 0
  | actions, plans, List.cons index rest => ((match index with | Nat.zero => 0 | Nat.succ remaining => (match planAt (plans) (remaining) with | Option.none => 0 | Option.some plan => planCharge (actions) ((plan).prepares))) + reachableCharge (actions) (plans) (rest))

@[expose] public def weaklyBelow : (left : List (Nat)) -> (right : List (Nat)) -> Bool
  | List.nil, right => (match right with | List.nil => true | List.cons _ _ => false)
  | List.cons leftHead leftRest, right => (match right with | List.nil => false | List.cons rightHead rightRest => ((Nat.ble (leftHead) (rightHead)) && weaklyBelow (leftRest) (rightRest)))

@[expose] public def dominates (left : List (Nat)) (right : List (Nat)) : Bool := (weaklyBelow (left) (right) && (!weaklyBelow (right) (left)))

@[expose] public def dominatedIn (Id : Type) : (cost : List (Nat)) -> (rows : List ((Prod (Id) (List (Nat))))) -> Bool
  | _cost, List.nil => false
  | cost, List.cons row rest => (dominates ((row).2) (cost) || dominatedIn (Id) (cost) (rest))

@[expose] public def frontierFrom (Id : Type) : (candidates : List ((Prod (Id) (List (Nat))))) -> (rows : List ((Prod (Id) (List (Nat))))) -> List (Id)
  | List.nil, _rows => ([] : List (Id))
  | List.cons row rest, rows => (if dominatedIn (Id) ((row).2) (rows) then frontierFrom (Id) (rest) (rows) else ((row).1 :: frontierFrom (Id) (rest) (rows)))

@[expose] public def frontier (Id : Type) (rows : List ((Prod (Id) (List (Nat))))) : List (Id) := frontierFrom (Id) (rows) (rows)

@[expose] public def scalarMinimum (Id : Type) : (rows : List ((Prod (Id) (Nat)))) -> (best : Nat) -> Nat
  | List.nil, best => best
  | List.cons row rest, best => scalarMinimum (Id) (rest) ((if (Nat.blt ((row).2) (best)) then (row).2 else best))

@[expose] public def attaining (Id : Type) : (rows : List ((Prod (Id) (Nat)))) -> (value : Nat) -> List (Id)
  | List.nil, _value => ([] : List (Id))
  | List.cons row rest, value => (if (Nat.beq ((row).2) (value)) then ((row).1 :: attaining (Id) (rest) (value)) else attaining (Id) (rest) (value))

@[expose] public def argmin (Id : Type) (rows : List ((Prod (Id) (Nat)))) : Option ((Prod (List (Id)) (Nat))) := (match rows with | List.nil => Option.none | List.cons head _ => (let best : Nat := scalarMinimum (Id) (rows) ((head).2); Option.some ((attaining (Id) (rows) (best), best))))

@[expose] public def anyBy (Id : Type) : (same : ((Id) -> (Id) -> (Bool))) -> (item : Id) -> (items : List (Id)) -> Bool
  | _same, _item, List.nil => false
  | same, item, List.cons head rest => ((same (item) (head)) || anyBy (Id) (same) (item) (rest))

@[expose] public def allIn (Id : Type) : (same : ((Id) -> (Id) -> (Bool))) -> (items : List (Id)) -> (others : List (Id)) -> Bool
  | _same, List.nil, _others => true
  | same, List.cons head rest, others => (anyBy (Id) (same) (head) (others) && allIn (Id) (same) (rest) (others))

@[expose] public def sameIdsBy (Id : Type) (same : ((Id) -> (Id) -> (Bool))) (left : List (Id)) (right : List (Id)) : Bool := (allIn (Id) (same) (left) (right) && allIn (Id) (same) (right) (left))

@[expose] public def certifiesArgmin (Id : Type) (same : ((Id) -> (Id) -> (Bool))) (rows : List ((Prod (Id) (Nat)))) (claimed : List (Id)) (cost : Nat) : Bool := (match argmin (Id) (rows) with | Option.none => false | Option.some found => ((Nat.beq ((found).2) (cost)) && sameIdsBy (Id) (same) ((found).1) (claimed)))

@[expose] public def certifiesFrontier (Id : Type) (same : ((Id) -> (Id) -> (Bool))) (rows : List ((Prod (Id) (List (Nat))))) (claimed : List (Id)) : Bool := sameIdsBy (Id) (same) (frontier (Id) (rows)) (claimed)

@[expose] public def vectorEq (left : List (Nat)) (right : List (Nat)) : Bool := (weaklyBelow (left) (right) && weaklyBelow (right) (left))

@[expose] public def attains (Id : Type) : (rows : List ((Prod (Id) (List (Nat))))) -> (cost : List (Nat)) -> Bool
  | List.nil, _cost => false
  | List.cons row rest, cost => (vectorEq ((row).2) (cost) || attains (Id) (rest) (cost))

@[expose] public def pointwiseMinimum : (left : List (Nat)) -> (right : List (Nat)) -> List (Nat)
  | List.nil, _right => ([] : List (Nat))
  | List.cons leftHead leftRest, right => (match right with | List.nil => ([] : List (Nat)) | List.cons rightHead rightRest => ((if (Nat.blt (rightHead) (leftHead)) then rightHead else leftHead) :: pointwiseMinimum (leftRest) (rightRest)))

@[expose] public def minimumFrom (Id : Type) : (rows : List ((Prod (Id) (List (Nat))))) -> (minimum : List (Nat)) -> List (Nat)
  | List.nil, minimum => minimum
  | List.cons row rest, minimum => minimumFrom (Id) (rest) (pointwiseMinimum (minimum) ((row).2))

@[expose] public def componentwiseMinimum (Id : Type) (rows : List ((Prod (Id) (List (Nat))))) : List (Nat) := (match rows with | List.nil => ([] : List (Nat)) | List.cons head _ => minimumFrom (Id) (rows) ((head).2))

@[expose] public def natSame (left : Nat) (right : Nat) : Bool := (Nat.beq (left) (right))

@[expose] public def statusOn : (system : LexLeanTarget.TargetSyntax.Program) -> (reference : LexLeanTarget.TargetSyntax.Program) -> (fuel : Nat) -> (domain : List (LexLeanTarget.TargetSyntax.Value)) -> Status
  | _system, _reference, _fuel, List.nil => Status.admitted (0) (0)
  | system, reference, fuel, List.cons argument rest => (match LexLeanTarget.TargetSemantics.run (fuel) (system) (0) ((argument :: ([] : List (LexLeanTarget.TargetSyntax.Value)))) with | LexLeanTarget.TargetSemantics.Outcome.value produced steps => (match LexLeanTarget.TargetSemantics.run (fuel) (reference) (0) ((argument :: ([] : List (LexLeanTarget.TargetSyntax.Value)))) with | LexLeanTarget.TargetSemantics.Outcome.value expected _ => (if valueEq (produced) (expected) then (match statusOn (system) (reference) (fuel) (rest) with | Status.admitted restSteps size => Status.admitted ((steps + restSteps)) (size) | Status.inadmissible => Status.inadmissible | Status.unresolved => Status.unresolved) else Status.inadmissible) | LexLeanTarget.TargetSemantics.Outcome.overflow _ => Status.unresolved | LexLeanTarget.TargetSemantics.Outcome.stuck => Status.unresolved | LexLeanTarget.TargetSemantics.Outcome.exhausted => Status.unresolved) | LexLeanTarget.TargetSemantics.Outcome.overflow _ => Status.inadmissible | LexLeanTarget.TargetSemantics.Outcome.stuck => Status.inadmissible | LexLeanTarget.TargetSemantics.Outcome.exhausted => Status.unresolved)

@[expose] public def status (request : Request) (grammar : Grammar) (selector : Selector) : Status := (let system : LexLeanTarget.TargetSyntax.Program := realize (grammar) (selector); (let indices : List (Nat) := reachable ((system).functions); (match statusOn (system) ((request).reference) (((request).machine).fuel) ((request).domain) with | Status.admitted steps _ => Status.admitted ((steps + (LexLeanRuntime.multiply ((LexLeanRuntime.length ((request).domain) : Nat)) (reachableCharge (((request).machine).actions) ((grammar).plans) (indices)) : Nat))) (reachableSize ((system).functions) (indices)) | Status.inadmissible => Status.inadmissible | Status.unresolved => Status.unresolved)))

@[expose] public def statuses : (request : Request) -> (grammar : Grammar) -> (selectors : List (Selector)) -> (next : Nat) -> List ((Prod (Nat) (Status)))
  | _request, _grammar, List.nil, _next => ([] : List ((Prod (Nat) (Status))))
  | request, grammar, List.cons selector rest, next => ((next, status (request) (grammar) (selector)) :: statuses (request) (grammar) (rest) ((next + 1)))

@[expose] public def anyUnresolved : (entries : List ((Prod (Nat) (Status)))) -> Bool
  | List.nil => false
  | List.cons entry rest => (match (entry).2 with | Status.unresolved => true | Status.admitted _ _ => anyUnresolved (rest) | Status.inadmissible => anyUnresolved (rest))

@[expose] public def admittedRows : (entries : List ((Prod (Nat) (Status)))) -> List ((Prod (Nat) (List (Nat))))
  | List.nil => ([] : List ((Prod (Nat) (List (Nat)))))
  | List.cons entry rest => (match (entry).2 with | Status.admitted steps size => (((entry).1, (steps :: (size :: ([] : List (Nat))))) :: admittedRows (rest)) | Status.inadmissible => admittedRows (rest) | Status.unresolved => admittedRows (rest))

@[expose] public def scalarRows : (rows : List ((Prod (Nat) (List (Nat))))) -> List ((Prod (Nat) (Nat)))
  | List.nil => ([] : List ((Prod (Nat) (Nat))))
  | List.cons row rest => (((row).1, (match (row).2 with | List.nil => 0 | List.cons steps _ => steps)) :: scalarRows (rest))

@[expose] public def evaluate (request : Request) (grammar : Grammar) : Answer := (let entries : List ((Prod (Nat) (Status))) := statuses (request) (grammar) (expand (grammar)) (0); (if anyUnresolved (entries) then Answer.incomplete else (let rows : List ((Prod (Nat) (List (Nat)))) := admittedRows (entries); (match (request).objective with | Objective.scalar => (match argmin (Nat) (scalarRows (rows)) with | Option.none => Answer.infeasible | Option.some found => Answer.argmin ((found).1) ((found).2)) | Objective.vector => (match rows with | List.nil => Answer.infeasible | List.cons _ _ => Answer.frontier (frontier (Nat) (rows)))))))

@[expose] public def answer (request : Request) : Answer := (match validate (request) with | Option.none => (match (request).carrier with | Carrier.grammar grammar => evaluate (request) (grammar) | Carrier.internalPlans => Answer.incomplete | Carrier.optimizerOutput => Answer.incomplete | Carrier.discovered _ => Answer.incomplete | Carrier.cached => Answer.incomplete) | Option.some rejection => Answer.rejected (rejection))

@[expose] public def certifies (request : Request) (claimed : Answer) : Bool := (match answer (request) with | Answer.rejected _ => false | Answer.argmin members steps => (match claimed with | Answer.rejected _ => false | Answer.argmin claimedMembers claimedSteps => ((Nat.beq (steps) (claimedSteps)) && sameIdsBy (Nat) ((natSame)) (members) (claimedMembers)) | Answer.frontier _ => false | Answer.infeasible => false | Answer.incomplete => false) | Answer.frontier members => (match claimed with | Answer.rejected _ => false | Answer.argmin _ _ => false | Answer.frontier claimedMembers => sameIdsBy (Nat) ((natSame)) (members) (claimedMembers) | Answer.infeasible => false | Answer.incomplete => false) | Answer.infeasible => false | Answer.incomplete => false)

@[expose] public def minimaAttained (request : Request) : Bool := (match (request).carrier with | Carrier.grammar grammar => (let rows : List ((Prod (Nat) (List (Nat)))) := admittedRows (statuses (request) (grammar) (expand (grammar)) (0)); attains (Nat) (rows) (componentwiseMinimum (Nat) (rows))) | Carrier.internalPlans => false | Carrier.optimizerOutput => false | Carrier.discovered _ => false | Carrier.cached => false)

@[expose] public def universeOf (request : Request) : List (Selector) := (match (request).carrier with | Carrier.grammar grammar => expand (grammar) | Carrier.internalPlans => ([] : List (Selector)) | Carrier.optimizerOutput => ([] : List (Selector)) | Carrier.discovered _ => ([] : List (Selector)) | Carrier.cached => ([] : List (Selector)))

@[expose] public def statusesOf (request : Request) : List ((Prod (Nat) (Status))) := (match (request).carrier with | Carrier.grammar grammar => statuses (request) (grammar) (expand (grammar)) (0) | Carrier.internalPlans => ([] : List ((Prod (Nat) (Status)))) | Carrier.optimizerOutput => ([] : List ((Prod (Nat) (Status)))) | Carrier.discovered _ => ([] : List ((Prod (Nat) (Status)))) | Carrier.cached => ([] : List ((Prod (Nat) (Status)))))

@[expose] public def extendWalk : (operations : List ((Prod (Nat) ((Prod (Nat) ((Prod (Nat) (Nat)))))))) -> (walk : (Prod (List (Nat)) ((Prod (Nat) (Nat))))) -> List ((Prod (List (Nat)) ((Prod (Nat) (Nat)))))
  | List.nil, _walk => ([] : List ((Prod (List (Nat)) ((Prod (Nat) (Nat))))))
  | List.cons operation rest, walk => (if (Nat.beq (((operation).2).1) (((walk).2).1)) then ((appendAll (Nat) ((walk).1) (((operation).1 :: ([] : List (Nat)))), ((((operation).2).2).1, (((walk).2).2 + (((operation).2).2).2))) :: extendWalk (rest) (walk)) else extendWalk (rest) (walk))

@[expose] public def extendAll : (operations : List ((Prod (Nat) ((Prod (Nat) ((Prod (Nat) (Nat)))))))) -> (walks : List ((Prod (List (Nat)) ((Prod (Nat) (Nat)))))) -> List ((Prod (List (Nat)) ((Prod (Nat) (Nat)))))
  | _operations, List.nil => ([] : List ((Prod (List (Nat)) ((Prod (Nat) (Nat))))))
  | operations, List.cons walk rest => appendAll ((Prod (List (Nat)) ((Prod (Nat) (Nat))))) (extendWalk (operations) (walk)) (extendAll (operations) (rest))

@[expose] public def arrivals : (target : Nat) -> (walks : List ((Prod (List (Nat)) ((Prod (Nat) (Nat)))))) -> List ((Prod (List (Nat)) (Nat)))
  | _target, List.nil => ([] : List ((Prod (List (Nat)) (Nat))))
  | target, List.cons walk rest => (if (Nat.beq (((walk).2).1) (target)) then (((walk).1, ((walk).2).2) :: arrivals (target) (rest)) else arrivals (target) (rest))

@[expose] public def compositions : (depth : Nat) -> (operations : List ((Prod (Nat) ((Prod (Nat) ((Prod (Nat) (Nat)))))))) -> (target : Nat) -> (layer : List ((Prod (List (Nat)) ((Prod (Nat) (Nat)))))) -> List ((Prod (List (Nat)) (Nat)))
  | Nat.zero, _operations, _target, _layer => ([] : List ((Prod (List (Nat)) (Nat))))
  | Nat.succ remaining, operations, target, layer => appendAll ((Prod (List (Nat)) (Nat))) (arrivals (target) (layer)) (compositions (remaining) (operations) (target) (extendAll (operations) (layer)))

@[expose] public def rewriteOnce : (rules : List ((Prod (Nat) (Nat)))) -> (item : Nat) -> Option (Nat)
  | List.nil, _item => Option.none
  | List.cons rule rest, item => (if (Nat.beq ((rule).1) (item)) then Option.some ((rule).2) else rewriteOnce (rest) (item))

@[expose] public def normalize : (fuel : Nat) -> (rules : List ((Prod (Nat) (Nat)))) -> (item : Nat) -> Nat
  | Nat.zero, _rules, item => item
  | Nat.succ remaining, rules, item => (match rewriteOnce (rules) (item) with | Option.none => item | Option.some next => normalize (remaining) (rules) (next))

@[expose] public def hiddenBitCost (system : Nat) (hidden : Nat) : Nat := (if (Nat.beq (system) (hidden)) then 0 else 10)

@[expose] public def environmentRows (hidden : Nat) : List ((Prod (Nat) (Nat))) := ((0, hiddenBitCost (0) (hidden)) :: ((1, hiddenBitCost (1) (hidden)) :: ([] : List ((Prod (Nat) (Nat))))))

@[expose] public def worstCase (system : Nat) : Nat := (let atZero : Nat := hiddenBitCost (system) (0); (let atOne : Nat := hiddenBitCost (system) (1); (if (Nat.blt (atZero) (atOne)) then atOne else atZero)))

@[expose] public def uniformRows : List ((Prod (Nat) (Nat))) := ((0, worstCase (0)) :: ((1, worstCase (1)) :: ([] : List ((Prod (Nat) (Nat))))))

@[expose] public def fairExpectation (hidden : Nat) : Nat := (LexLeanRuntime.quotient ((hiddenBitCost (0) (hidden) + hiddenBitCost (1) (hidden))) (2) (0) : Nat)

public theorem vec01Optimum : (argmin (List (Nat)) (compositions (4) (((0, (0, (1, 2))) :: ((1, (1, (2, 3))) :: ((2, (0, (2, 6))) :: ([] : List ((Prod (Nat) ((Prod (Nat) ((Prod (Nat) (Nat)))))))))))) (2) (((([] : List (Nat)), (0, 0)) :: ([] : List ((Prod (List (Nat)) ((Prod (Nat) (Nat))))))))) = Option.some ((((0 :: (1 :: ([] : List (Nat)))) :: ([] : List (List (Nat)))), 5))) := by
  rfl

public theorem vec01Extension : (argmin (List (Nat)) (compositions (5) (((0, (0, (1, 2))) :: ((1, (1, (2, 3))) :: ((2, (0, (2, 6))) :: ((3, (0, (2, 4))) :: ([] : List ((Prod (Nat) ((Prod (Nat) ((Prod (Nat) (Nat))))))))))))) (2) (((([] : List (Nat)), (0, 0)) :: ([] : List ((Prod (List (Nat)) ((Prod (Nat) (Nat))))))))) = Option.some ((((3 :: ([] : List (Nat))) :: ([] : List (List (Nat)))), 4))) := by
  rfl

public theorem vec01CertificateHolds : (certifiesArgmin (List (Nat)) ((vectorEq)) (compositions (4) (((0, (0, (1, 2))) :: ((1, (1, (2, 3))) :: ((2, (0, (2, 6))) :: ([] : List ((Prod (Nat) ((Prod (Nat) ((Prod (Nat) (Nat)))))))))))) (2) (((([] : List (Nat)), (0, 0)) :: ([] : List ((Prod (List (Nat)) ((Prod (Nat) (Nat))))))))) (((0 :: (1 :: ([] : List (Nat)))) :: ([] : List (List (Nat))))) (5) = true) := by
  rfl

public theorem vec01CertificateInvalidated : (certifiesArgmin (List (Nat)) ((vectorEq)) (compositions (5) (((0, (0, (1, 2))) :: ((1, (1, (2, 3))) :: ((2, (0, (2, 6))) :: ((3, (0, (2, 4))) :: ([] : List ((Prod (Nat) ((Prod (Nat) ((Prod (Nat) (Nat))))))))))))) (2) (((([] : List (Nat)), (0, 0)) :: ([] : List ((Prod (List (Nat)) ((Prod (Nat) (Nat))))))))) (((0 :: (1 :: ([] : List (Nat)))) :: ([] : List (List (Nat))))) (5) = false) := by
  rfl

public theorem vec02Frontier : (frontier (Nat) (((0, (1 :: (3 :: ([] : List (Nat))))) :: ((1, (2 :: (2 :: ([] : List (Nat))))) :: ((2, (3 :: (1 :: ([] : List (Nat))))) :: ((3, (3 :: (3 :: ([] : List (Nat))))) :: ([] : List ((Prod (Nat) (List (Nat)))))))))) = (0 :: (1 :: (2 :: ([] : List (Nat)))))) := by
  rfl

public theorem rej29FrontierOmission : (certifiesFrontier (Nat) ((natSame)) (((0, (1 :: (3 :: ([] : List (Nat))))) :: ((1, (2 :: (2 :: ([] : List (Nat))))) :: ((2, (3 :: (1 :: ([] : List (Nat))))) :: ((3, (3 :: (3 :: ([] : List (Nat))))) :: ([] : List ((Prod (Nat) (List (Nat)))))))))) ((0 :: (1 :: ([] : List (Nat))))) = false) := by
  rfl

public theorem rej14ComponentwiseMinima : (componentwiseMinimum (Nat) (((0, (1 :: (3 :: ([] : List (Nat))))) :: ((1, (2 :: (2 :: ([] : List (Nat))))) :: ((2, (3 :: (1 :: ([] : List (Nat))))) :: ((3, (3 :: (3 :: ([] : List (Nat))))) :: ([] : List ((Prod (Nat) (List (Nat)))))))))) = (1 :: (1 :: ([] : List (Nat))))) := by
  rfl

public theorem rej14MinimaUnattained : (attains (Nat) (((0, (1 :: (3 :: ([] : List (Nat))))) :: ((1, (2 :: (2 :: ([] : List (Nat))))) :: ((2, (3 :: (1 :: ([] : List (Nat))))) :: ((3, (3 :: (3 :: ([] : List (Nat))))) :: ([] : List ((Prod (Nat) (List (Nat)))))))))) (componentwiseMinimum (Nat) (((0, (1 :: (3 :: ([] : List (Nat))))) :: ((1, (2 :: (2 :: ([] : List (Nat))))) :: ((2, (3 :: (1 :: ([] : List (Nat))))) :: ((3, (3 :: (3 :: ([] : List (Nat))))) :: ([] : List ((Prod (Nat) (List (Nat))))))))))) = false) := by
  rfl

public theorem vec04NormalFormIsB : (normalize (10) (((0, 1) :: ([] : List ((Prod (Nat) (Nat)))))) (0) = 1) := by
  rfl

public theorem vec04GlobalMinimumIsC : (argmin (Nat) (((0, 2) :: ((1, 1) :: ((2, 0) :: ([] : List ((Prod (Nat) (Nat)))))))) = Option.some (((2 :: ([] : List (Nat))), 0))) := by
  rfl

public theorem vec04NormalFormRefused : (certifiesArgmin (Nat) ((natSame)) (((0, 2) :: ((1, 1) :: ((2, 0) :: ([] : List ((Prod (Nat) (Nat)))))))) ((normalize (10) (((0, 1) :: ([] : List ((Prod (Nat) (Nat)))))) (0) :: ([] : List (Nat)))) (1) = false) := by
  rfl

public theorem vec17EnvelopeAtZero : (argmin (Nat) (environmentRows (0)) = Option.some (((0 :: ([] : List (Nat))), 0))) := by
  rfl

public theorem vec17EnvelopeAtOne : (argmin (Nat) (environmentRows (1)) = Option.some (((1 :: ([] : List (Nat))), 0))) := by
  rfl

public theorem vec17UniformWorstCase : (argmin (Nat) (uniformRows) = Option.some (((0 :: (1 :: ([] : List (Nat)))), 10))) := by
  rfl

public theorem vec17FairExpectationAtZero : (fairExpectation (0) = 5) := by
  rfl

public theorem vec17FairExpectationAtOne : (fairExpectation (1) = 5) := by
  rfl

public theorem vec17PointwiseValueRefused : (certifiesArgmin (Nat) ((natSame)) (uniformRows) ((0 :: ([] : List (Nat)))) (0) = false) := by
  rfl

end LexLeanTarget.Gnaf
