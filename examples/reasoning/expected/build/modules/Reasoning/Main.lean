module
public import Init
public import Reasoning.Budget
public import Reasoning.Clinic
public import Reasoning.Planner
public import Reasoning.Screening
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
namespace Reasoning.Main

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

namespace LexLeanCollections

public class Key (α : Type) where
  compare : α -> α -> Ordering

@[expose] public def compareCodes : List Char -> List Char -> Ordering
  | [], [] => .eq
  | [], _ :: _ => .lt
  | _ :: _, [] => .gt
  | left :: lefts, right :: rights =>
    match Ord.compare left.val.toNat right.val.toNat with
    | .eq => compareCodes lefts rights
    | other => other

public instance : Key Nat where compare := Ord.compare
public instance : Key Int where compare := Ord.compare
public instance : Key Bool where compare := Ord.compare
public instance : Key Int8 where compare left right := Ord.compare left.toInt right.toInt
public instance : Key Int16 where compare left right := Ord.compare left.toInt right.toInt
public instance : Key Int32 where compare left right := Ord.compare left.toInt right.toInt
public instance : Key Int64 where compare left right := Ord.compare left.toInt right.toInt
public instance : Key UInt8 where compare left right := Ord.compare left.toNat right.toNat
public instance : Key UInt16 where compare left right := Ord.compare left.toNat right.toNat
public instance : Key UInt32 where compare left right := Ord.compare left.toNat right.toNat
public instance : Key UInt64 where compare left right := Ord.compare left.toNat right.toNat
public instance : Key String where compare left right := compareCodes left.toList right.toList
public instance {α β : Type} [Key α] [Key β] : Key (Prod α β) where
  compare left right := match Key.compare left.1 right.1 with
    | .eq => Key.compare left.2 right.2
    | other => other

@[expose] public def insertEntry {κ ν : Type} [Key κ] (key : κ) (value : ν) : List (Prod κ ν) -> List (Prod κ ν)
  | [] => [(key, value)]
  | (other, stored) :: rest => match Key.compare key other with
    | .lt => (key, value) :: (other, stored) :: rest
    | .eq => (key, value) :: rest
    | .gt => (other, stored) :: insertEntry key value rest

@[expose] public def removeEntry {κ ν : Type} [Key κ] (key : κ) : List (Prod κ ν) -> List (Prod κ ν)
  | [] => []
  | (other, stored) :: rest => match Key.compare key other with
    | .lt => (other, stored) :: rest
    | .eq => rest
    | .gt => (other, stored) :: removeEntry key rest

@[expose] public def lookupEntry {κ ν : Type} [Key κ] (key : κ) : List (Prod κ ν) -> Option ν
  | [] => none
  | (other, stored) :: rest => match Key.compare key other with
    | .lt => none
    | .eq => some stored
    | .gt => lookupEntry key rest

@[expose] public def insertElement {κ : Type} [Key κ] (key : κ) : List κ -> List κ
  | [] => [key]
  | other :: rest => match Key.compare key other with
    | .lt => key :: other :: rest
    | .eq => other :: rest
    | .gt => other :: insertElement key rest

@[expose] public def removeElement {κ : Type} [Key κ] (key : κ) : List κ -> List κ
  | [] => []
  | other :: rest => match Key.compare key other with
    | .lt => other :: rest
    | .eq => rest
    | .gt => other :: removeElement key rest

@[expose] public def containsElement {κ : Type} [Key κ] (key : κ) : List κ -> Bool
  | [] => false
  | other :: rest => match Key.compare key other with
    | .lt => false
    | .eq => true
    | .gt => containsElement key rest

@[expose] public def mapInsert {κ ν : Type} [Key κ] (map : List (Prod κ ν)) (key : κ) (value : ν) : List (Prod κ ν) := insertEntry key value map
@[expose] public def mapRemove {κ ν : Type} [Key κ] (map : List (Prod κ ν)) (key : κ) : List (Prod κ ν) := removeEntry key map
@[expose] public def mapLookup {κ ν : Type} [Key κ] (map : List (Prod κ ν)) (key : κ) : Option ν := lookupEntry key map
@[expose] public def mapContains {κ ν : Type} [Key κ] (map : List (Prod κ ν)) (key : κ) : Bool := (lookupEntry key map).isSome
@[expose] public def mapSize {κ ν : Type} (map : List (Prod κ ν)) : Nat := map.length
@[expose] public def mapKeys {κ ν : Type} (map : List (Prod κ ν)) : List κ := map.map Prod.fst
@[expose] public def mapValues {κ ν : Type} (map : List (Prod κ ν)) : List ν := map.map Prod.snd
@[expose] public def mapEntries {κ ν : Type} (map : List (Prod κ ν)) : List (Prod κ ν) := map
@[expose] public def mapFold {κ ν β : Type} (step : β -> κ -> ν -> β) (initial : β) (map : List (Prod κ ν)) : β :=
  map.foldl (fun state entry => step state entry.1 entry.2) initial

@[expose] public def setInsert {κ : Type} [Key κ] (set : List κ) (key : κ) : List κ := insertElement key set
@[expose] public def setRemove {κ : Type} [Key κ] (set : List κ) (key : κ) : List κ := removeElement key set
@[expose] public def setContains {κ : Type} [Key κ] (set : List κ) (key : κ) : Bool := containsElement key set
@[expose] public def setSize {κ : Type} (set : List κ) : Nat := set.length
@[expose] public def setElements {κ : Type} (set : List κ) : List κ := set
@[expose] public def setUnion {κ : Type} [Key κ] (left right : List κ) : List κ := right.foldl (fun acc key => insertElement key acc) left
@[expose] public def setIntersection {κ : Type} [Key κ] (left right : List κ) : List κ := left.filter (fun key => containsElement key right)
@[expose] public def setDifference {κ : Type} [Key κ] (left right : List κ) : List κ := left.filter (fun key => !containsElement key right)

@[expose] public def graphSuccessors {κ : Type} [Key κ] (graph : List (Prod κ (List κ))) (node : κ) : List κ :=
  (lookupEntry node graph).getD []

@[expose] public def graphNodes {κ : Type} [Key κ] (graph : List (Prod κ (List κ))) : List κ :=
  graph.foldl (fun acc entry => entry.2.foldl (fun inner node => insertElement node inner) (insertElement entry.1 acc)) []

@[expose] public def reachableFrom {κ : Type} [Key κ] (graph : List (Prod κ (List κ))) : Nat -> List κ -> List κ -> List κ
  | 0, _, seen => seen
  | Nat.succ fuel, frontier, seen =>
    let next := frontier.foldl (fun acc node =>
      (graphSuccessors graph node).foldl (fun acc2 succ =>
        if containsElement succ seen || containsElement succ acc2 then acc2 else insertElement succ acc2) acc) []
    match next with
    | [] => seen
    | _ => reachableFrom graph fuel next (next.foldl (fun acc key => insertElement key acc) seen)

@[expose] public def graphReachable {κ : Type} [Key κ] (graph : List (Prod κ (List κ))) (start : κ) : List κ :=
  reachableFrom graph ((graphNodes graph).length + 1) [start] [start]

@[expose] public def topological {κ : Type} [Key κ] (graph : List (Prod κ (List κ))) : Nat -> List κ -> List κ -> Option (List κ)
  | 0, remaining, order => if remaining.isEmpty then some order.reverse else none
  | Nat.succ fuel, remaining, order =>
    match remaining.filter (fun node => remaining.all (fun other => !containsElement node (graphSuccessors graph other))) with
    | [] => if remaining.isEmpty then some order.reverse else none
    | ready :: _ => topological graph fuel (removeElement ready remaining) (ready :: order)

@[expose] public def graphTopological {κ : Type} [Key κ] (graph : List (Prod κ (List κ))) : Option (List κ) :=
  let nodes := graphNodes graph
  topological graph (nodes.length + 1) nodes []
@[expose] public def listFold {α σ : Type} (step : σ -> α -> σ) (initial : σ) (values : List α) : σ :=
  values.foldl step initial

@[expose] public def setFold {κ σ : Type} (step : σ -> κ -> σ) (initial : σ) (set : List κ) : σ :=
  set.foldl step initial

@[expose] public def iterate {σ : Type} (step : σ -> σ) : Nat -> σ -> σ
  | 0, state => state
  | Nat.succ count, state => iterate step count (step state)

@[expose] public def iterateUntil {σ : Type} (step : σ -> Option σ) : Nat -> σ -> Prod σ Bool
  | 0, state => (state, false)
  | Nat.succ fuel, state => match step state with
    | none => (state, true)
    | some next => iterateUntil step fuel next

@[expose] public def lessThan {κ : Type} [Key κ] (left right : κ) : Bool :=
  match Key.compare left right with
  | .lt => true
  | _ => false
end LexLeanCollections

namespace LexLeanModels

@[expose] public def Sound1 {α : Type} (v : α -> Bool) (p : α -> Prop) : Prop :=
  forall (a : α), v a = true -> p a
@[expose] public def Sound2 {α β : Type} (v : α -> β -> Bool) (p : α -> β -> Prop) : Prop :=
  forall (a : α) (b : β), v a b = true -> p a b
@[expose] public def Sound4 {α β γ δ : Type} (v : α -> β -> γ -> δ -> Bool) (p : α -> β -> γ -> δ -> Prop) : Prop :=
  forall (a : α) (b : β) (c : γ) (d : δ), v a b c d = true -> p a b c d
@[expose] public def Complete1 {α : Type} (v : α -> Bool) (p : α -> Prop) : Prop :=
  forall (a : α), p a -> v a = true
@[expose] public def Complete2 {α β : Type} (v : α -> β -> Bool) (p : α -> β -> Prop) : Prop :=
  forall (a : α) (b : β), p a b -> v a b = true
@[expose] public def Complete4 {α β γ δ : Type} (v : α -> β -> γ -> δ -> Bool) (p : α -> β -> γ -> δ -> Prop) : Prop :=
  forall (a : α) (b : β) (c : γ) (d : δ), p a b c d -> v a b c d = true

@[expose] public def Satisfies {α β : Type} (p : α -> Prop) (q : α -> β -> Prop) (r : α -> β) : Prop :=
  forall (a : α), p a -> q a (r a)
@[expose] public def SatisfiesTotal {α β : Type} (q : α -> β -> Prop) (r : α -> β) : Prop :=
  forall (a : α), q a (r a)
@[expose] public def SatisfiesStep {σ α β : Type} (j : σ -> Prop) (p : σ -> α -> Prop) (q : σ -> α -> σ -> β -> Prop) (r : σ -> α -> Prod σ β) : Prop :=
  forall (s : σ) (a : α), j s -> p s a -> q s a (r s a).1 (r s a).2
@[expose] public def SatisfiesStepNoInvariant {σ α β : Type} (p : σ -> α -> Prop) (q : σ -> α -> σ -> β -> Prop) (r : σ -> α -> Prod σ β) : Prop :=
  forall (s : σ) (a : α), p s a -> q s a (r s a).1 (r s a).2
@[expose] public def SatisfiesStepNoPrecondition {σ α β : Type} (j : σ -> Prop) (q : σ -> α -> σ -> β -> Prop) (r : σ -> α -> Prod σ β) : Prop :=
  forall (s : σ) (a : α), j s -> q s a (r s a).1 (r s a).2
@[expose] public def SatisfiesStepTotal {σ α β : Type} (q : σ -> α -> σ -> β -> Prop) (r : σ -> α -> Prod σ β) : Prop :=
  forall (s : σ) (a : α), q s a (r s a).1 (r s a).2
@[expose] public def Preserves {σ α β : Type} (j : σ -> Prop) (p : σ -> α -> Prop) (r : σ -> α -> Prod σ β) : Prop :=
  forall (s : σ) (a : α), j s -> p s a -> j (r s a).1
@[expose] public def PreservesTotal {σ α β : Type} (j : σ -> Prop) (r : σ -> α -> Prod σ β) : Prop :=
  forall (s : σ) (a : α), j s -> j (r s a).1
@[expose] public def Initial {σ : Type} (j : σ -> Prop) (i : σ) : Prop :=
  j i
@[expose] public def Equivalent {α β : Type} (p : α -> Prop) (r f : α -> β) : Prop :=
  forall (a : α), p a -> r a = f a
@[expose] public def EquivalentTotal {α β : Type} (r f : α -> β) : Prop :=
  forall (a : α), r a = f a
@[expose] public def EquivalentStep {σ α β : Type} (j : σ -> Prop) (p : σ -> α -> Prop) (r f : σ -> α -> Prod σ β) : Prop :=
  forall (s : σ) (a : α), j s -> p s a -> r s a = f s a
@[expose] public def EquivalentStepNoInvariant {σ α β : Type} (p : σ -> α -> Prop) (r f : σ -> α -> Prod σ β) : Prop :=
  forall (s : σ) (a : α), p s a -> r s a = f s a
@[expose] public def EquivalentStepNoPrecondition {σ α β : Type} (j : σ -> Prop) (r f : σ -> α -> Prod σ β) : Prop :=
  forall (s : σ) (a : α), j s -> r s a = f s a
@[expose] public def EquivalentStepTotal {σ α β : Type} (r f : σ -> α -> Prod σ β) : Prop :=
  forall (s : σ) (a : α), r s a = f s a
@[expose, reducible] public def Agreement {α β : Type} (r : α -> β) (cmp : β -> β -> Bool) (d : List (Prod α β)) (examples agreements : Nat) : Prop :=
  d.length = examples /\ d.foldl (fun c e => if cmp (r e.1) e.2 then c + 1 else c) 0 = agreements

@[expose] public def inRange (width : Nat) (signed : Bool) (value : Int) : Bool :=
  if signed then
    decide (-((2 : Int) ^ (8 * width - 1)) <= value) && decide (value < (2 : Int) ^ (8 * width - 1))
  else
    decide (0 <= value) && decide (value < (2 : Int) ^ (8 * width))
@[expose] public def encodeInt (width : Nat) (value : Int) : List Nat :=
  let unsigned := if value < 0 then value + (2 : Int) ^ (8 * width) else value
  (List.range width).map (fun i => (unsigned.toNat / 256 ^ i) % 256)
@[expose] public def tensorMatches (width : Nat) (signed : Bool) (values : List Int) (bytes : ByteArray) : Bool :=
  values.all (inRange width signed) && values.flatMap (encodeInt width) == bytes.data.toList.map UInt8.toNat
@[expose] public def utf8Char (c : Char) : List Nat :=
  let n := c.toNat
  if n < 128 then [n]
  else if n < 2048 then [192 + n / 64, 128 + n % 64]
  else if n < 65536 then [224 + n / 4096, 128 + (n / 64) % 64, 128 + n % 64]
  else [240 + n / 262144, 128 + (n / 4096) % 64, 128 + (n / 64) % 64, 128 + n % 64]
@[expose] public def linesMatch (lines : List String) (bytes : ByteArray) : Bool :=
  lines.all (fun line => line.toList.all (fun c => c.toNat != 10 && c.toNat != 13)) &&
    lines.foldr (fun line rest => line.toList.foldr (fun c tail => utf8Char c ++ tail) [10] ++ rest) [] ==
      bytes.data.toList.map UInt8.toNat
end LexLeanModels

namespace LexLeanReasoning

public inductive Star {σ : Type} (r : σ -> σ -> Prop) : σ -> σ -> Prop where
  | refl (a : σ) : Star r a a
  | tail (a b c : σ) : Star r a b -> r b c -> Star r a c

public inductive All {α : Type} (p : α -> Prop) : List α -> Prop where
  | nil : All p []
  | cons (a : α) (rest : List α) : p a -> All p rest -> All p (a :: rest)

@[expose] public def Preserves {σ : Type} (r : σ -> σ -> Prop) (j : σ -> Prop) : Prop :=
  forall (s t : σ), j s -> r s t -> j t
@[expose] public def Sound {α β : Type} (v : α -> β -> Bool) (p : α -> β -> Prop) : Prop :=
  forall (a : α) (b : β), v a b = true -> p a b
@[expose] public def Complete {α β : Type} (v : α -> β -> Bool) (p : α -> β -> Prop) : Prop :=
  forall (a : α) (b : β), p a b -> v a b = true
@[expose] public def Reaches {σ ε : Type} (r : σ -> σ -> Prop) (a : σ) (acc : Except ε σ) : Prop :=
  forall (s : σ), acc = Except.ok s -> Star r a s
@[expose] public def Found {ν ρ : Type} (ok : ν -> Prop) (accept : ν -> Option ρ) : Option (Prod ρ ν) -> Prop
  | none => True
  | some hit => ok hit.2 /\ accept hit.2 = some hit.1
@[expose] public def SearchOk {ν ρ : Type} (ok : ν -> Prop) (accept : ν -> Option ρ) (frontier : List ν) (found : Option (Prod ρ ν)) : Prop :=
  All ok frontier /\ Found ok accept found

public theorem starPreserves {σ : Type} (r : σ -> σ -> Prop) (j : σ -> Prop) (h : Preserves r j) :
    forall (a b : σ), Star r a b -> j a -> j b := by
  intro a b hs
  induction hs with
  | refl => exact fun ha => ha
  | tail y z _ hyz ih => exact fun ha => h y z (ih ha) hyz

public theorem reachesStart {σ ε : Type} (r : σ -> σ -> Prop) (a : σ) : Reaches (ε := ε) r a (Except.ok a) := by
  intro s e
  cases e
  exact Star.refl a

public theorem guarded {σ : Type} (r : σ -> σ -> Prop) (s : σ) (g : Bool) (c : σ) (h : g = true -> r s c) (t : σ) :
    (if g then some c else none) = some t -> r s t := by
  cases g with
  | false => intro e; cases e
  | true => intro e; cases e; exact h rfl

public theorem guardedRank {σ : Type} (μ : σ -> Nat) (s : σ) (g : Bool) (c : σ) (h : g = true -> μ c < μ s) (t : σ) :
    (if g then some c else none) = some t -> μ t < μ s := by
  cases g with
  | false => intro e; cases e
  | true => intro e; cases e; exact h rfl

public theorem checked {ρ : Type} (g : Bool) (a b : ρ) : (if g then some a else none) = some b -> g = true /\ a = b := by
  cases g with
  | false => intro e; cases e
  | true => intro e; cases e; exact And.intro rfl rfl

public theorem foldInvariant {σ α : Type} (f : σ -> α -> σ) (p : σ -> Prop) (h : forall (a : σ) (x : α), p a -> p (f a x)) :
    forall (xs : List α) (a : σ), p a -> p (LexLeanCollections.listFold f a xs) := by
  intro xs
  induction xs with
  | nil => exact fun _ ha => ha
  | cons x rest ih => exact fun a ha => ih (f a x) (h a x ha)

public theorem foldSnoc {σ α : Type} (f : σ -> α -> σ) (a : σ) (xs : List α) (y : α) :
    LexLeanCollections.listFold f a (LexLeanRuntime.append xs (y :: [])) = f (LexLeanCollections.listFold f a xs) y := by
  induction xs generalizing a with
  | nil => rfl
  | cons x rest ih => exact ih (f a x)

public theorem allAppend {α : Type} (p : α -> Prop) : forall (xs ys : List α), All p xs -> All p ys -> All p (LexLeanRuntime.append xs ys) := by
  intro xs
  induction xs with
  | nil => exact fun _ _ h => h
  | cons x rest ih =>
    intro ys hx hy
    cases hx with
    | cons _ _ hp hr => exact All.cons x (LexLeanRuntime.append rest ys) hp (ih ys hr hy)

public theorem allSingle {α : Type} (p : α -> Prop) (a : α) (h : p a) : All p (a :: []) :=
  All.cons a [] h All.nil

public theorem allHead {α : Type} (p : α -> Prop) (a : α) (rest : List α) (h : All p (a :: rest)) : p a := by
  cases h with
  | cons _ _ hp _ => exact hp

public theorem allTail {α : Type} (p : α -> Prop) (a : α) (rest : List α) (h : All p (a :: rest)) : All p rest := by
  cases h with
  | cons _ _ _ hr => exact hr

public theorem allFold {σ α : Type} (f : σ -> α -> σ) (p : σ -> Prop) (q : α -> Prop) (h : forall (a : σ) (x : α), q x -> p a -> p (f a x)) :
    forall (xs : List α) (a : σ), All q xs -> p a -> p (LexLeanCollections.listFold f a xs) := by
  intro xs
  induction xs with
  | nil => exact fun _ _ ha => ha
  | cons x rest ih =>
    intro a hx ha
    cases hx with
    | cons _ _ hq hr => exact ih (f a x) hr (h a x hq ha)

public theorem capAll {α : Type} (p : α -> Prop) (k : Nat) (xs : List α) (h : All p xs) :
    All p (LexLeanCollections.listFold (fun (acc : List α) (n : α) => if Nat.blt (LexLeanRuntime.length acc) k then LexLeanRuntime.append acc (n :: []) else acc) [] xs) :=
  allFold (fun (acc : List α) (n : α) => if Nat.blt (LexLeanRuntime.length acc) k then LexLeanRuntime.append acc (n :: []) else acc) (All p) p
    (fun a x hq ha => by
      show All p (if Nat.blt (LexLeanRuntime.length a) k then LexLeanRuntime.append a (x :: []) else a)
      cases Nat.blt (LexLeanRuntime.length a) k with
      | false => exact ha
      | true => exact allAppend p a (x :: []) ha (allSingle p x hq)) xs [] h All.nil

public theorem lengthSnoc {α : Type} (xs : List α) (y : α) : LexLeanRuntime.length (LexLeanRuntime.append xs (y :: [])) = LexLeanRuntime.length xs + 1 := by
  induction xs with
  | nil => rfl
  | cons x rest ih => exact congrArg Nat.succ ih

public theorem capBound {α : Type} (k : Nat) (xs : List α) :
    LexLeanRuntime.length (LexLeanCollections.listFold (fun (acc : List α) (n : α) => if Nat.blt (LexLeanRuntime.length acc) k then LexLeanRuntime.append acc (n :: []) else acc) [] xs) <= k :=
  foldInvariant (fun (acc : List α) (n : α) => if Nat.blt (LexLeanRuntime.length acc) k then LexLeanRuntime.append acc (n :: []) else acc) (fun (acc : List α) => LexLeanRuntime.length acc <= k)
    (fun a x ha => by
      show LexLeanRuntime.length (if Nat.blt (LexLeanRuntime.length a) k then LexLeanRuntime.append a (x :: []) else a) <= k
      cases e : Nat.blt (LexLeanRuntime.length a) k with
      | false => exact ha
      | true =>
        show LexLeanRuntime.length (LexLeanRuntime.append a (x :: [])) <= k
        rw [lengthSnoc]
        exact Nat.le_of_ble_eq_true e) xs [] (Nat.zero_le k)

public theorem freshAll {ν κ : Type} [LexLeanCollections.Key κ] (key : ν -> κ) (p : ν -> Prop) (v : List κ) (ns : List ν) (h : All p ns) :
    All p (LexLeanCollections.listFold (fun (acc : Prod (List ν) (List κ)) (n : ν) => if LexLeanCollections.setContains acc.2 (key n) then acc else (LexLeanRuntime.append acc.1 (n :: []), LexLeanCollections.setInsert acc.2 (key n))) (([] : List ν), v) ns).1 :=
  allFold (fun (acc : Prod (List ν) (List κ)) (n : ν) => if LexLeanCollections.setContains acc.2 (key n) then acc else (LexLeanRuntime.append acc.1 (n :: []), LexLeanCollections.setInsert acc.2 (key n))) (fun (acc : Prod (List ν) (List κ)) => All p acc.1) p
    (fun a x hq ha => by
      show All p (if LexLeanCollections.setContains a.2 (key x) then a else (LexLeanRuntime.append a.1 (x :: []), LexLeanCollections.setInsert a.2 (key x))).1
      cases LexLeanCollections.setContains a.2 (key x) with
      | true => exact ha
      | false => exact allAppend p a.1 (x :: []) ha (allSingle p x hq)) ns (([] : List ν), v) h All.nil

public theorem searchStart {ν ρ : Type} (ok : ν -> Prop) (accept : ν -> Option ρ) (frontier : List ν) (h : All ok frontier) :
    SearchOk ok accept frontier none :=
  And.intro h True.intro

public theorem peakBound (p len k : Nat) (hp : p <= k) (hl : len <= k) : (if Nat.blt p len then len else p) <= k := by
  cases Nat.blt p len with
  | false => exact hp
  | true => exact hl

public theorem iterateUntilInvariant {σ : Type} (step : σ -> Option σ) (p : σ -> Prop) (h : forall (a b : σ), step a = some b -> p a -> p b) :
    forall (n : Nat) (a : σ), p a -> p (LexLeanCollections.iterateUntil step n a).1 := by
  intro n
  induction n with
  | zero => exact fun _ ha => ha
  | succ n ih =>
    intro a ha
    show p (match step a with | none => (a, true) | some next => LexLeanCollections.iterateUntil step n next).1
    cases e : step a with
    | none => exact ha
    | some b => exact ih b (h a b e ha)

public theorem iterateUntilSimulate {σ τ : Type} (f : σ -> Option σ) (g : τ -> Option τ) (π : σ -> τ)
    (hnone : forall (a : σ), f a = none -> g (π a) = none)
    (hsome : forall (a b : σ), f a = some b -> g (π a) = some (π b)) :
    forall (n : Nat) (a : σ), π (LexLeanCollections.iterateUntil f n a).1 = (LexLeanCollections.iterateUntil g n (π a)).1 /\ (LexLeanCollections.iterateUntil f n a).2 = (LexLeanCollections.iterateUntil g n (π a)).2 := by
  intro n
  induction n with
  | zero => exact fun _ => And.intro rfl rfl
  | succ n ih =>
    intro a
    show π (match f a with | none => (a, true) | some next => LexLeanCollections.iterateUntil f n next).1 = (match g (π a) with | none => (π a, true) | some next => LexLeanCollections.iterateUntil g n next).1 /\ (match f a with | none => (a, true) | some next => LexLeanCollections.iterateUntil f n next).2 = (match g (π a) with | none => (π a, true) | some next => LexLeanCollections.iterateUntil g n next).2
    cases e : f a with
    | none => rw [hnone a e]; exact And.intro rfl rfl
    | some b => rw [hsome a b e]; exact ih b

public theorem iterateUntilCount {σ : Type} (step : σ -> Option σ) (c : σ -> Nat) (h : forall (a b : σ), step a = some b -> c b = c a + 1) :
    forall (n : Nat) (a : σ), c a = 0 -> c (LexLeanCollections.iterateUntil step n a).1 <= n := by
  have general : forall (n : Nat) (a : σ), c (LexLeanCollections.iterateUntil step n a).1 <= c a + n := by
    intro n
    induction n with
    | zero => exact fun a => Nat.le_refl (c a)
    | succ n ih =>
      intro a
      show c (match step a with | none => (a, true) | some next => LexLeanCollections.iterateUntil step n next).1 <= c a + (n + 1)
      cases e : step a with
      | none => exact Nat.le_add_right (c a) (n + 1)
      | some b =>
        have hb := ih b
        rw [h a b e] at hb
        rw [Nat.add_right_comm] at hb
        exact (Nat.add_assoc (c a) n 1) ▸ (Nat.add_right_comm (c a) 1 n) ▸ hb
  intro n a h0
  have hg := general n a
  rw [h0, Nat.zero_add] at hg
  exact hg

public theorem iterateUntilGrowth {σ : Type} (step : σ -> Option σ) (c : σ -> Nat) (h : forall (a b : σ), step a = some b -> c b <= c a + 1) :
    forall (n : Nat) (a : σ), c a = 0 -> c (LexLeanCollections.iterateUntil step n a).1 <= n := by
  have general : forall (n : Nat) (a : σ), c (LexLeanCollections.iterateUntil step n a).1 <= c a + n := by
    intro n
    induction n with
    | zero => exact fun a => Nat.le_refl (c a)
    | succ n ih =>
      intro a
      show c (match step a with | none => (a, true) | some next => LexLeanCollections.iterateUntil step n next).1 <= c a + (n + 1)
      cases e : step a with
      | none => exact Nat.le_add_right (c a) (n + 1)
      | some b =>
        have hb := Nat.le_trans (ih b) (Nat.add_le_add_right (h a b e) n)
        rw [Nat.add_right_comm] at hb
        exact hb
  intro n a h0
  have hg := general n a
  rw [h0, Nat.zero_add] at hg
  exact hg

public theorem iterateUntilStops {σ : Type} (step : σ -> Option σ) (p : σ -> Prop) (μ : σ -> Nat)
    (h : forall (a b : σ), step a = some b -> p a -> p b /\ μ b < μ a) :
    forall (n : Nat) (a : σ), p a -> μ a < n -> (LexLeanCollections.iterateUntil step n a).2 = true := by
  intro n
  induction n with
  | zero => intro a _ hm; exact absurd hm (Nat.not_lt_zero (μ a))
  | succ n ih =>
    intro a ha hm
    show (match step a with | none => (a, true) | some next => LexLeanCollections.iterateUntil step n next).2 = true
    cases e : step a with
    | none => rfl
    | some b =>
      have hb := h a b e ha
      exact ih b hb.left (Nat.lt_of_lt_of_le hb.right (Nat.le_of_lt_succ hm))

public theorem iterateUntilBound {σ : Type} (step : σ -> Option σ) (c : σ -> Nat) (k : Nat) (h : forall (a b : σ), step a = some b -> c a <= k -> c b <= k) :
    forall (n : Nat) (a : σ), c a <= k -> c (LexLeanCollections.iterateUntil step n a).1 <= k :=
  iterateUntilInvariant step (fun (a : σ) => c a <= k) h
public theorem noneSome {ρ : Type} {q : Prop} (v : ρ) (h : (none : Option ρ) = some v) : q := by
  cases h

public theorem zeroLe (n : Nat) : 0 <= n :=
  Nat.zero_le n

public theorem bltSucc (a b : Nat) (h : Nat.blt a b = true) : a + 1 <= b :=
  Nat.le_of_ble_eq_true h

end LexLeanReasoning

@[expose, reducible] public def Guaranteed (v : Reasoning.Clinic.Vitals) (o : Except ((Prod Bool Bool)) (Nat)) : Prop := (forall (r : Nat), ((o = Except.ok (r)) -> Reasoning.Clinic.Indicated (v) (r)))


@[expose] public def GatewayTriage (v : Reasoning.Clinic.Vitals) : Except ((Prod Bool Bool)) (Nat) := Reasoning.Clinic.Triage.verdict (v)

public theorem gateway_safe (v : Reasoning.Clinic.Vitals) : Guaranteed (v) (GatewayTriage (v)) := by
  exact Reasoning.Clinic.Triage.verdict_sound (v)

public theorem GatewayEvidence.gateway_safe : LexLeanModels.SatisfiesTotal ((Reasoning.Main.Guaranteed)) ((Reasoning.Main.GatewayTriage)) := Reasoning.Main.gateway_safe

@[expose] public def GatewayModel (__input : Reasoning.Clinic.Vitals) : Except ((Prod Bool Bool)) (Nat) := Reasoning.Main.GatewayTriage (__input)

@[expose, reducible] public def Graded (_v : Reasoning.Clinic.Vitals) (r : Nat) : Prop := (r <= 3)

@[expose] public def gradedCheck (_v : Reasoning.Clinic.Vitals) (r : Nat) : Bool := (Nat.ble (r) (3))

public theorem graded_sound (v : Reasoning.Clinic.Vitals) (r : Nat) : ((gradedCheck (v) (r) = true) -> Graded (v) (r)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Graded, gradedCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem Scale.sound : (LexLeanReasoning.Sound ((Reasoning.Main.gradedCheck)) ((Reasoning.Main.Graded))) :=
  Reasoning.Main.graded_sound

public theorem grade_admitted (v : Reasoning.Clinic.Vitals) : Reasoning.Clinic.Consistent (({ vitals := v, fever := 0, tachycardia := 0, tachypnea := 0, leukocytosis := 0, sirs := 0, sepsis := 0, shock := 0, level := 0 } : Reasoning.Clinic.Chart)) := by
  exact Reasoning.Clinic.admitted_consistent (v)

public theorem grade_pending (v : Reasoning.Clinic.Vitals) : (Reasoning.Clinic.Pending (({ vitals := v, fever := 0, tachycardia := 0, tachypnea := 0, leukocytosis := 0, sirs := 0, sepsis := 0, shock := 0, level := 0 } : Reasoning.Clinic.Chart)) < 11) := by
  exact Reasoning.Clinic.admitted_pending (v)

public theorem grade_correct (v : Reasoning.Clinic.Vitals) (chart : Reasoning.Clinic.Chart) (r : Nat) : (Reasoning.Clinic.Consistent (chart) -> ((Option.some ((chart).level) = Option.some (r)) -> Graded (v) (r))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Reasoning.Clinic.Consistent, Graded, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public inductive Grade.Step where
  | Fever
  | Tachycardia
  | Tachypnea
  | Leukocytosis
  | Sirs
  | Sepsis
  | Shock
  | Escalate
public structure Grade.Ledger where
  iterations : Nat
  attempts : Nat
  firings : Nat
  expansions : Nat
  verifications : Nat
  frontier : Nat
@[expose] public def Grade.observe (v : Reasoning.Clinic.Vitals) : Reasoning.Clinic.Chart := ({ vitals := v, fever := 0, tachycardia := 0, tachypnea := 0, leukocytosis := 0, sirs := 0, sepsis := 0, shock := 0, level := 0 } : Reasoning.Clinic.Chart)
@[expose] public def Grade.fire (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Main.Grade.Step) : Option (Reasoning.Clinic.Chart) := (match __step with | Reasoning.Main.Grade.Step.Fever => Reasoning.Clinic.Fever.apply (__s) | Reasoning.Main.Grade.Step.Tachycardia => Reasoning.Clinic.Tachycardia.apply (__s) | Reasoning.Main.Grade.Step.Tachypnea => Reasoning.Clinic.Tachypnea.apply (__s) | Reasoning.Main.Grade.Step.Leukocytosis => Reasoning.Clinic.Leukocytosis.apply (__s) | Reasoning.Main.Grade.Step.Sirs => Reasoning.Clinic.Sirs.apply (__s) | Reasoning.Main.Grade.Step.Sepsis => Reasoning.Clinic.Sepsis.apply (__s) | Reasoning.Main.Grade.Step.Shock => Reasoning.Clinic.Shock.apply (__s) | Reasoning.Main.Grade.Step.Escalate => Reasoning.Clinic.Escalate.apply (__s))
@[expose] public def Grade.replay (__acc : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart)) (__step : Reasoning.Main.Grade.Step) : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match Reasoning.Main.Grade.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def Grade.follow (v : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Main.Grade.Step)) : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart) := (LexLeanCollections.listFold ((Reasoning.Main.Grade.replay)) (Except.ok (Reasoning.Main.Grade.observe (v))) (__trace) : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart))
@[expose] public def Grade.extract (chart : Reasoning.Clinic.Chart) : Option (Nat) := Option.some ((chart).level)
@[expose] public def Grade.accept (_v : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) : Option (Nat) := Reasoning.Main.Grade.extract (__s)
@[expose] public def Grade.answer (v : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Main.Grade.Step)) : Option (Nat) := (match Reasoning.Main.Grade.follow (v) (__trace) with | Except.ok __s => Reasoning.Main.Grade.accept (v) (__s) | Except.error _ => Option.none)
@[expose] public def Grade.select (__s : Reasoning.Clinic.Chart) : Option (Reasoning.Main.Grade.Step) := (match (match (match (if Reasoning.Clinic.Fever.guard (__s) then Option.some (Reasoning.Main.Grade.Step.Fever) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Tachycardia.guard (__s) then Option.some (Reasoning.Main.Grade.Step.Tachycardia) else Option.none)) with | Option.some __found => Option.some (__found) | Option.none => (match (if Reasoning.Clinic.Tachypnea.guard (__s) then Option.some (Reasoning.Main.Grade.Step.Tachypnea) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Leukocytosis.guard (__s) then Option.some (Reasoning.Main.Grade.Step.Leukocytosis) else Option.none))) with | Option.some __found => Option.some (__found) | Option.none => (match (match (if Reasoning.Clinic.Sirs.guard (__s) then Option.some (Reasoning.Main.Grade.Step.Sirs) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Sepsis.guard (__s) then Option.some (Reasoning.Main.Grade.Step.Sepsis) else Option.none)) with | Option.some __found => Option.some (__found) | Option.none => (match (if Reasoning.Clinic.Shock.guard (__s) then Option.some (Reasoning.Main.Grade.Step.Shock) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Escalate.guard (__s) then Option.some (Reasoning.Main.Grade.Step.Escalate) else Option.none))))
@[expose] public def Grade.attempts (__s : Reasoning.Clinic.Chart) : Nat := ((let __scan6l : (Prod (Bool) (Nat)) := (let __scan2l : (Prod (Bool) (Nat)) := (let __scan0l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Fever.guard (__s), 1); (if (__scan0l).1 then __scan0l else (let __scan0r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Tachycardia.guard (__s), 1); ((__scan0r).1, ((__scan0l).2 + (__scan0r).2))))); (if (__scan2l).1 then __scan2l else (let __scan2r : (Prod (Bool) (Nat)) := (let __scan1l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Tachypnea.guard (__s), 1); (if (__scan1l).1 then __scan1l else (let __scan1r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Leukocytosis.guard (__s), 1); ((__scan1r).1, ((__scan1l).2 + (__scan1r).2))))); ((__scan2r).1, ((__scan2l).2 + (__scan2r).2))))); (if (__scan6l).1 then __scan6l else (let __scan6r : (Prod (Bool) (Nat)) := (let __scan5l : (Prod (Bool) (Nat)) := (let __scan3l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Sirs.guard (__s), 1); (if (__scan3l).1 then __scan3l else (let __scan3r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Sepsis.guard (__s), 1); ((__scan3r).1, ((__scan3l).2 + (__scan3r).2))))); (if (__scan5l).1 then __scan5l else (let __scan5r : (Prod (Bool) (Nat)) := (let __scan4l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Shock.guard (__s), 1); (if (__scan4l).1 then __scan4l else (let __scan4r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Escalate.guard (__s), 1); ((__scan4r).1, ((__scan4l).2 + (__scan4r).2))))); ((__scan5r).1, ((__scan5l).2 + (__scan5r).2))))); ((__scan6r).1, ((__scan6l).2 + (__scan6r).2)))))).2
@[expose] public def Grade.next (__s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Chart) := (match Reasoning.Main.Grade.select (__s) with | Option.none => Option.none | Option.some __step => Reasoning.Main.Grade.fire (__s) (__step))
@[expose] public def Grade.saturate (v : Reasoning.Clinic.Vitals) : (Prod (Reasoning.Clinic.Chart) (Bool)) := (LexLeanCollections.iterateUntil ((Reasoning.Main.Grade.next)) (11) (Reasoning.Main.Grade.observe (v)) : (Prod (Reasoning.Clinic.Chart) (Bool)))
public structure Grade.Run where
  state : Reasoning.Clinic.Chart
  trace : List (Reasoning.Main.Grade.Step)
  ledger : Reasoning.Main.Grade.Ledger
@[expose] public def Grade.start (v : Reasoning.Clinic.Vitals) : Reasoning.Main.Grade.Run := ({ state := Reasoning.Main.Grade.observe (v), trace := ([] : List (Reasoning.Main.Grade.Step)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := 0 } : Reasoning.Main.Grade.Ledger) } : Reasoning.Main.Grade.Run)
@[expose] public def Grade.step (__r : Reasoning.Main.Grade.Run) : Option (Reasoning.Main.Grade.Run) := (match Reasoning.Main.Grade.select ((__r).state) with | Option.none => Option.none | Option.some __step => (match Reasoning.Main.Grade.fire ((__r).state) (__step) with | Option.none => Option.none | Option.some __t => Option.some (({ state := __t, trace := (LexLeanRuntime.append ((__r).trace) ((__step :: ([] : List (Reasoning.Main.Grade.Step)))) : List (Reasoning.Main.Grade.Step)), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + Reasoning.Main.Grade.attempts ((__r).state)), firings := (((__r).ledger).firings + 1), expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : Reasoning.Main.Grade.Ledger) } : Reasoning.Main.Grade.Run))))
@[expose] public def Grade.run (v : Reasoning.Clinic.Vitals) : (Prod (Reasoning.Main.Grade.Run) (Bool)) := (LexLeanCollections.iterateUntil ((Reasoning.Main.Grade.step)) (11) (Reasoning.Main.Grade.start (v)) : (Prod (Reasoning.Main.Grade.Run) (Bool)))
@[expose] public def Grade.account (v : Reasoning.Clinic.Vitals) : Reasoning.Main.Grade.Ledger := (let __final : (Prod (Reasoning.Main.Grade.Run) (Bool)) := Reasoning.Main.Grade.run (v); (match (__final).2 with | Bool.true => ({ iterations := (((__final).1).ledger).iterations, attempts := ((((__final).1).ledger).attempts + Reasoning.Main.Grade.attempts (((__final).1).state)), firings := (((__final).1).ledger).firings, expansions := (((__final).1).ledger).expansions, verifications := (((__final).1).ledger).verifications, frontier := (((__final).1).ledger).frontier } : Reasoning.Main.Grade.Ledger) | Bool.false => ((__final).1).ledger))
@[expose] public def Grade.conclude (v : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) : Except ((Prod Bool Bool)) (Nat) := (match Reasoning.Main.Grade.accept (v) (__s) with | Option.some __v => Except.ok (__v) | Option.none => (match Reasoning.Main.Grade.extract (__s) with | Option.some _ => Except.error (((true, false) : Prod Bool Bool)) | Option.none => Except.error (((false, true) : Prod Bool Bool))))
@[expose] public def Grade.verdict (v : Reasoning.Clinic.Vitals) : Except ((Prod Bool Bool)) (Nat) := (let __final : (Prod (Reasoning.Clinic.Chart) (Bool)) := Reasoning.Main.Grade.saturate (v); (match (__final).2 with | Bool.true => Reasoning.Main.Grade.conclude (v) ((__final).1) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
@[expose] public def Grade (v : Reasoning.Clinic.Vitals) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (Reasoning.Main.Grade.Step)))) := (let __final : (Prod (Reasoning.Main.Grade.Run) (Bool)) := Reasoning.Main.Grade.run (v); (match (__final).2 with | Bool.true => (match Reasoning.Main.Grade.conclude (v) (((__final).1).state) with | Except.ok __v => Except.ok ((__v, ((__final).1).trace)) | Except.error __e => Except.error (__e)) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
public theorem Grade.fire_sound (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Main.Grade.Step) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Main.Grade.fire (__s) (__step) = Option.some (__t)) -> Reasoning.Clinic.Justified (__s) (__t)) :=
by
  cases __step with
  | Fever =>
    exact Reasoning.Clinic.Fever.apply_sound __s __t
  | Tachycardia =>
    exact Reasoning.Clinic.Tachycardia.apply_sound __s __t
  | Tachypnea =>
    exact Reasoning.Clinic.Tachypnea.apply_sound __s __t
  | Leukocytosis =>
    exact Reasoning.Clinic.Leukocytosis.apply_sound __s __t
  | Sirs =>
    exact Reasoning.Clinic.Sirs.apply_sound __s __t
  | Sepsis =>
    exact Reasoning.Clinic.Sepsis.apply_sound __s __t
  | Shock =>
    exact Reasoning.Clinic.Shock.apply_sound __s __t
  | Escalate =>
    exact Reasoning.Clinic.Escalate.apply_sound __s __t
public theorem Grade.fire_progress (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Main.Grade.Step) (__t : Reasoning.Clinic.Chart) : (Reasoning.Clinic.Consistent (__s) -> ((Reasoning.Main.Grade.fire (__s) (__step) = Option.some (__t)) -> (Reasoning.Clinic.Pending (__t) < Reasoning.Clinic.Pending (__s)))) :=
by
  cases __step with
  | Fever =>
    exact Reasoning.Clinic.Fever.apply_progress __s __t
  | Tachycardia =>
    exact Reasoning.Clinic.Tachycardia.apply_progress __s __t
  | Tachypnea =>
    exact Reasoning.Clinic.Tachypnea.apply_progress __s __t
  | Leukocytosis =>
    exact Reasoning.Clinic.Leukocytosis.apply_progress __s __t
  | Sirs =>
    exact Reasoning.Clinic.Sirs.apply_progress __s __t
  | Sepsis =>
    exact Reasoning.Clinic.Sepsis.apply_progress __s __t
  | Shock =>
    exact Reasoning.Clinic.Shock.apply_progress __s __t
  | Escalate =>
    exact Reasoning.Clinic.Escalate.apply_progress __s __t
public theorem Grade.replay_fire (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Main.Grade.Step) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Main.Grade.fire (__s) (__step) = Option.some (__t)) -> (Reasoning.Main.Grade.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [Reasoning.Main.Grade.replay]
  rw [llE]
public theorem Grade.replay_sound (v : Reasoning.Clinic.Vitals) (__acc : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart)) (__step : Reasoning.Main.Grade.Step) : ((LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Main.Grade.observe (v)) (__acc)) -> (LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Main.Grade.observe (v)) (Reasoning.Main.Grade.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [Reasoning.Main.Grade.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (Reasoning.Main.Grade.fire_sound llS __step _ ‹_›)
public theorem Grade.derivation (v : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Main.Grade.Step)) : (LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Main.Grade.observe (v)) (Reasoning.Main.Grade.follow (v) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((Reasoning.Main.Grade.replay)) (fun (__acc : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart)) => (LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Main.Grade.observe (v)) (__acc))) (Reasoning.Main.Grade.replay_sound (v)) (__trace) (Except.ok (Reasoning.Main.Grade.observe (v))) (LexLeanReasoning.reachesStart ((Reasoning.Clinic.Justified)) (Reasoning.Main.Grade.observe (v))))
public theorem Grade.follow_invariant (v : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Main.Grade.Step)) (__s : Reasoning.Clinic.Chart) : ((Reasoning.Main.Grade.follow (v) (__trace) = Except.ok (__s)) -> Reasoning.Clinic.Consistent (__s)) :=
  (fun llE => (LexLeanReasoning.starPreserves ((Reasoning.Clinic.Justified)) ((Reasoning.Clinic.Consistent)) Reasoning.Clinic.Findings.preserves (Reasoning.Main.Grade.observe (v)) (__s) (Reasoning.Main.Grade.derivation (v) (__trace) (__s) llE) (Reasoning.Main.grade_admitted (v))))
public theorem Grade.accept_sound (v : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) (__v : Nat) : (Reasoning.Clinic.Consistent (__s) -> ((Reasoning.Main.Grade.accept (v) (__s) = Option.some (__v)) -> Reasoning.Main.Graded (v) (__v))) :=
  (fun llJ llE => (Reasoning.Main.grade_correct (v) (__s) (__v) llJ llE))
public theorem Grade.next_sound (__s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Main.Grade.next (__s) = Option.some (__t)) -> Reasoning.Clinic.Justified (__s) (__t)) :=
by
  intro llE
  dsimp only [Reasoning.Main.Grade.next] at llE
  split at llE
  · cases llE
  · exact Reasoning.Main.Grade.fire_sound __s _ __t llE
public theorem Grade.next_preserves (__s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Main.Grade.next (__s) = Option.some (__t)) -> (Reasoning.Clinic.Consistent (__s) -> Reasoning.Clinic.Consistent (__t))) :=
  (fun llE llH => (Reasoning.Clinic.consistent_preserved (__s) (__t) llH (Reasoning.Main.Grade.next_sound (__s) (__t) llE)))
public theorem Grade.next_progress (__s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Main.Grade.next (__s) = Option.some (__t)) -> (Reasoning.Clinic.Consistent (__s) -> (Reasoning.Clinic.Pending (__t) < Reasoning.Clinic.Pending (__s)))) :=
by
  intro llE
  dsimp only [Reasoning.Main.Grade.next] at llE
  split at llE
  · cases llE
  · exact fun llJ => Reasoning.Main.Grade.fire_progress __s _ __t llJ llE
public theorem Grade.step_none (__r : Reasoning.Main.Grade.Run) : ((Reasoning.Main.Grade.step (__r) = Option.none) -> (Reasoning.Main.Grade.next ((__r).state) = Option.none)) :=
by
  intro llE
  dsimp only [Reasoning.Main.Grade.step] at llE
  split at llE
  · rename_i llH
    dsimp only [Reasoning.Main.Grade.next]
    rw [llH]
  · rename_i llH
    split at llE
    · dsimp only [Reasoning.Main.Grade.next]
      rw [llH]
      assumption
    · cases llE
public theorem Grade.step_some (__r : Reasoning.Main.Grade.Run) (__q : Reasoning.Main.Grade.Run) : ((Reasoning.Main.Grade.step (__r) = Option.some (__q)) -> (Reasoning.Main.Grade.next ((__r).state) = Option.some ((__q).state))) :=
by
  intro llE
  dsimp only [Reasoning.Main.Grade.step] at llE
  split at llE
  · cases llE
  · rename_i llH
    split at llE
    · cases llE
    · cases llE
      dsimp only [Reasoning.Main.Grade.next]
      rw [llH]
      assumption
public theorem Grade.step_trace (v : Reasoning.Clinic.Vitals) (__r : Reasoning.Main.Grade.Run) (__q : Reasoning.Main.Grade.Run) : ((Reasoning.Main.Grade.step (__r) = Option.some (__q)) -> ((Reasoning.Main.Grade.follow (v) ((__r).trace) = Except.ok ((__r).state)) -> (Reasoning.Main.Grade.follow (v) ((__q).trace) = Except.ok ((__q).state)))) :=
by
  intro llE llH
  dsimp only [Reasoning.Main.Grade.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      dsimp only [Reasoning.Main.Grade.follow]
      rw [LexLeanReasoning.foldSnoc]
      dsimp only [Reasoning.Main.Grade.follow] at llH
      rw [llH]
      apply Reasoning.Main.Grade.replay_fire
      assumption
public theorem Grade.step_count (__r : Reasoning.Main.Grade.Run) (__q : Reasoning.Main.Grade.Run) : ((Reasoning.Main.Grade.step (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Main.Grade.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Grade.step_fired (__r : Reasoning.Main.Grade.Run) (__q : Reasoning.Main.Grade.Run) : ((Reasoning.Main.Grade.step (__r) = Option.some (__q)) -> (((__q).ledger).firings = (((__r).ledger).firings + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Main.Grade.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Grade.saturate_derivation (v : Reasoning.Clinic.Vitals) : (LexLeanReasoning.Star ((Reasoning.Clinic.Justified)) (Reasoning.Main.Grade.observe (v)) ((Reasoning.Main.Grade.saturate (v)).1)) :=
by
  dsimp only [Reasoning.Main.Grade.saturate]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Main.Grade.next)) (fun (__s : Reasoning.Clinic.Chart) => (LexLeanReasoning.Star ((Reasoning.Clinic.Justified)) (Reasoning.Main.Grade.observe (v)) (__s))) (fun llA llB llE llH => (LexLeanReasoning.Star.tail _ llA llB llH (Reasoning.Main.Grade.next_sound llA llB llE))) (11) (Reasoning.Main.Grade.observe (v)) (LexLeanReasoning.Star.refl _))
public theorem Grade.run_state (v : Reasoning.Clinic.Vitals) : ((((Reasoning.Main.Grade.run (v)).1).state = (Reasoning.Main.Grade.saturate (v)).1) /\ ((Reasoning.Main.Grade.run (v)).2 = (Reasoning.Main.Grade.saturate (v)).2)) :=
by
  dsimp only [Reasoning.Main.Grade.run, Reasoning.Main.Grade.saturate]
  exact (LexLeanReasoning.iterateUntilSimulate ((Reasoning.Main.Grade.step)) ((Reasoning.Main.Grade.next)) ((fun (__r : Reasoning.Main.Grade.Run) => (__r).state)) Reasoning.Main.Grade.step_none Reasoning.Main.Grade.step_some (11) (Reasoning.Main.Grade.start (v)))
public theorem Grade.run_trace (v : Reasoning.Clinic.Vitals) : (Reasoning.Main.Grade.follow (v) (((Reasoning.Main.Grade.run (v)).1).trace) = Except.ok (((Reasoning.Main.Grade.run (v)).1).state)) :=
by
  dsimp only [Reasoning.Main.Grade.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Main.Grade.step)) (fun (__r : Reasoning.Main.Grade.Run) => (Reasoning.Main.Grade.follow (v) ((__r).trace) = Except.ok ((__r).state))) (Reasoning.Main.Grade.step_trace (v)) (11) (Reasoning.Main.Grade.start (v)) rfl)
public theorem Grade.iterations_bounded (v : Reasoning.Clinic.Vitals) : ((((Reasoning.Main.Grade.run (v)).1).ledger).iterations <= 11) :=
by
  dsimp only [Reasoning.Main.Grade.run]
  exact (LexLeanReasoning.iterateUntilCount ((Reasoning.Main.Grade.step)) ((fun (__r : Reasoning.Main.Grade.Run) => ((__r).ledger).iterations)) Reasoning.Main.Grade.step_count (11) (Reasoning.Main.Grade.start (v)) rfl)
public theorem Grade.firings_bounded (v : Reasoning.Clinic.Vitals) : ((((Reasoning.Main.Grade.run (v)).1).ledger).firings <= 11) :=
by
  dsimp only [Reasoning.Main.Grade.run]
  exact (LexLeanReasoning.iterateUntilCount ((Reasoning.Main.Grade.step)) ((fun (__r : Reasoning.Main.Grade.Run) => ((__r).ledger).firings)) Reasoning.Main.Grade.step_fired (11) (Reasoning.Main.Grade.start (v)) rfl)
public theorem Grade.saturate_invariant (v : Reasoning.Clinic.Vitals) : Reasoning.Clinic.Consistent ((Reasoning.Main.Grade.saturate (v)).1) :=
by
  dsimp only [Reasoning.Main.Grade.saturate]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Main.Grade.next)) ((Reasoning.Clinic.Consistent)) Reasoning.Main.Grade.next_preserves (11) (Reasoning.Main.Grade.observe (v)) (Reasoning.Main.grade_admitted (v)))
public theorem Grade.run_invariant (v : Reasoning.Clinic.Vitals) : Reasoning.Clinic.Consistent (((Reasoning.Main.Grade.run (v)).1).state) :=
by
  rw [And.left (Reasoning.Main.Grade.run_state v)]
  exact Reasoning.Main.Grade.saturate_invariant v
public theorem Grade.saturates (v : Reasoning.Clinic.Vitals) : ((Reasoning.Main.Grade.saturate (v)).2 = true) :=
by
  dsimp only [Reasoning.Main.Grade.saturate]
  exact (LexLeanReasoning.iterateUntilStops ((Reasoning.Main.Grade.next)) ((Reasoning.Clinic.Consistent)) ((Reasoning.Clinic.Pending)) (fun llA llB llE llH => (And.intro (Reasoning.Main.Grade.next_preserves llA llB llE llH) (Reasoning.Main.Grade.next_progress llA llB llE llH))) (11) (Reasoning.Main.Grade.observe (v)) (Reasoning.Main.grade_admitted (v)) (Reasoning.Main.grade_pending (v)))
public theorem Grade.conclude_accept (v : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) (__v : Nat) : ((Reasoning.Main.Grade.conclude (v) (__s) = Except.ok (__v)) -> (Reasoning.Main.Grade.accept (v) (__s) = Option.some (__v))) :=
by
  intro llE
  dsimp only [Reasoning.Main.Grade.conclude] at llE
  split at llE
  · cases llE
    assumption
  · split at llE
    · cases llE
    · cases llE
public theorem Grade.conclude_sound (v : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) (__v : Nat) : (Reasoning.Clinic.Consistent (__s) -> ((Reasoning.Main.Grade.conclude (v) (__s) = Except.ok (__v)) -> Reasoning.Main.Graded (v) (__v))) :=
by
  intro llJ llE
  dsimp only [Reasoning.Main.Grade.conclude] at llE
  split at llE
  · cases llE
    exact Reasoning.Main.Grade.accept_sound _ _ _ llJ ‹_›
  · split at llE
    · cases llE
    · cases llE
public theorem Grade.verdict_sound (v : Reasoning.Clinic.Vitals) : (forall (__v : Nat), ((Reasoning.Main.Grade.verdict (v) = Except.ok (__v)) -> Reasoning.Main.Graded (v) (__v))) :=
by
  intro llV llE
  have llJ := Reasoning.Main.Grade.saturate_invariant v
  dsimp only [Reasoning.Main.Grade.verdict] at llE
  generalize llRun : Reasoning.Main.Grade.saturate v = llR at llE llJ
  split at llE
  · exact Reasoning.Main.Grade.conclude_sound _ _ _ llJ llE
  · cases llE
public theorem Grade.explained (v : Reasoning.Clinic.Vitals) : (forall (__v : Nat), (forall (__trace : List (Reasoning.Main.Grade.Step)), ((Reasoning.Main.Grade (v) = Except.ok ((__v, __trace))) -> ((Reasoning.Main.Grade.answer (v) (__trace) = Option.some (__v)) /\ Reasoning.Main.Graded (v) (__v))))) :=
by
  intro llV llT llE
  have llTrace := Reasoning.Main.Grade.run_trace v
  have llJ := Reasoning.Main.Grade.run_invariant v
  dsimp only [Reasoning.Main.Grade] at llE
  generalize llRun : Reasoning.Main.Grade.run v = llR at llE llTrace llJ
  split at llE
  · split at llE
    · rename_i llW llH
      cases llE
      exact And.intro (by dsimp only [Reasoning.Main.Grade.answer]; rw [llTrace]; exact Reasoning.Main.Grade.conclude_accept _ _ _ llH) (Reasoning.Main.Grade.conclude_sound _ _ _ llJ llH)
    · cases llE
  · cases llE

public theorem cap_correct (v : Reasoning.Clinic.Vitals) (chart : Reasoning.Clinic.Chart) (r : Nat) : ((Option.some ((LexLeanRuntime.subtract (3) ((LexLeanRuntime.subtract (3) ((chart).level) : Nat)) : Nat)) = Option.some (r)) -> Graded (v) (r)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Graded, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public inductive Cap.Step where
  | Fever
  | Tachycardia
  | Tachypnea
  | Leukocytosis
  | Sirs
  | Sepsis
  | Shock
  | Escalate
public structure Cap.Ledger where
  iterations : Nat
  attempts : Nat
  firings : Nat
  expansions : Nat
  verifications : Nat
  frontier : Nat
@[expose] public def Cap.observe (v : Reasoning.Clinic.Vitals) : Reasoning.Clinic.Chart := ({ vitals := v, fever := 0, tachycardia := 0, tachypnea := 0, leukocytosis := 0, sirs := 0, sepsis := 0, shock := 0, level := 0 } : Reasoning.Clinic.Chart)
@[expose] public def Cap.fire (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Main.Cap.Step) : Option (Reasoning.Clinic.Chart) := (match __step with | Reasoning.Main.Cap.Step.Fever => Reasoning.Clinic.Fever.apply (__s) | Reasoning.Main.Cap.Step.Tachycardia => Reasoning.Clinic.Tachycardia.apply (__s) | Reasoning.Main.Cap.Step.Tachypnea => Reasoning.Clinic.Tachypnea.apply (__s) | Reasoning.Main.Cap.Step.Leukocytosis => Reasoning.Clinic.Leukocytosis.apply (__s) | Reasoning.Main.Cap.Step.Sirs => Reasoning.Clinic.Sirs.apply (__s) | Reasoning.Main.Cap.Step.Sepsis => Reasoning.Clinic.Sepsis.apply (__s) | Reasoning.Main.Cap.Step.Shock => Reasoning.Clinic.Shock.apply (__s) | Reasoning.Main.Cap.Step.Escalate => Reasoning.Clinic.Escalate.apply (__s))
@[expose] public def Cap.replay (__acc : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart)) (__step : Reasoning.Main.Cap.Step) : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match Reasoning.Main.Cap.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def Cap.follow (v : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Main.Cap.Step)) : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart) := (LexLeanCollections.listFold ((Reasoning.Main.Cap.replay)) (Except.ok (Reasoning.Main.Cap.observe (v))) (__trace) : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart))
@[expose] public def Cap.extract (chart : Reasoning.Clinic.Chart) : Option (Nat) := Option.some ((LexLeanRuntime.subtract (3) ((LexLeanRuntime.subtract (3) ((chart).level) : Nat)) : Nat))
@[expose] public def Cap.accept (_v : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) : Option (Nat) := Reasoning.Main.Cap.extract (__s)
@[expose] public def Cap.answer (v : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Main.Cap.Step)) : Option (Nat) := (match Reasoning.Main.Cap.follow (v) (__trace) with | Except.ok __s => Reasoning.Main.Cap.accept (v) (__s) | Except.error _ => Option.none)
@[expose] public def Cap.select (__s : Reasoning.Clinic.Chart) : Option (Reasoning.Main.Cap.Step) := (match (match (match (if Reasoning.Clinic.Fever.guard (__s) then Option.some (Reasoning.Main.Cap.Step.Fever) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Tachycardia.guard (__s) then Option.some (Reasoning.Main.Cap.Step.Tachycardia) else Option.none)) with | Option.some __found => Option.some (__found) | Option.none => (match (if Reasoning.Clinic.Tachypnea.guard (__s) then Option.some (Reasoning.Main.Cap.Step.Tachypnea) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Leukocytosis.guard (__s) then Option.some (Reasoning.Main.Cap.Step.Leukocytosis) else Option.none))) with | Option.some __found => Option.some (__found) | Option.none => (match (match (if Reasoning.Clinic.Sirs.guard (__s) then Option.some (Reasoning.Main.Cap.Step.Sirs) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Sepsis.guard (__s) then Option.some (Reasoning.Main.Cap.Step.Sepsis) else Option.none)) with | Option.some __found => Option.some (__found) | Option.none => (match (if Reasoning.Clinic.Shock.guard (__s) then Option.some (Reasoning.Main.Cap.Step.Shock) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Escalate.guard (__s) then Option.some (Reasoning.Main.Cap.Step.Escalate) else Option.none))))
@[expose] public def Cap.attempts (__s : Reasoning.Clinic.Chart) : Nat := ((let __scan6l : (Prod (Bool) (Nat)) := (let __scan2l : (Prod (Bool) (Nat)) := (let __scan0l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Fever.guard (__s), 1); (if (__scan0l).1 then __scan0l else (let __scan0r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Tachycardia.guard (__s), 1); ((__scan0r).1, ((__scan0l).2 + (__scan0r).2))))); (if (__scan2l).1 then __scan2l else (let __scan2r : (Prod (Bool) (Nat)) := (let __scan1l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Tachypnea.guard (__s), 1); (if (__scan1l).1 then __scan1l else (let __scan1r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Leukocytosis.guard (__s), 1); ((__scan1r).1, ((__scan1l).2 + (__scan1r).2))))); ((__scan2r).1, ((__scan2l).2 + (__scan2r).2))))); (if (__scan6l).1 then __scan6l else (let __scan6r : (Prod (Bool) (Nat)) := (let __scan5l : (Prod (Bool) (Nat)) := (let __scan3l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Sirs.guard (__s), 1); (if (__scan3l).1 then __scan3l else (let __scan3r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Sepsis.guard (__s), 1); ((__scan3r).1, ((__scan3l).2 + (__scan3r).2))))); (if (__scan5l).1 then __scan5l else (let __scan5r : (Prod (Bool) (Nat)) := (let __scan4l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Shock.guard (__s), 1); (if (__scan4l).1 then __scan4l else (let __scan4r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Escalate.guard (__s), 1); ((__scan4r).1, ((__scan4l).2 + (__scan4r).2))))); ((__scan5r).1, ((__scan5l).2 + (__scan5r).2))))); ((__scan6r).1, ((__scan6l).2 + (__scan6r).2)))))).2
@[expose] public def Cap.next (__s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Chart) := (match Reasoning.Main.Cap.select (__s) with | Option.none => Option.none | Option.some __step => Reasoning.Main.Cap.fire (__s) (__step))
@[expose] public def Cap.saturate (v : Reasoning.Clinic.Vitals) : (Prod (Reasoning.Clinic.Chart) (Bool)) := (LexLeanCollections.iterateUntil ((Reasoning.Main.Cap.next)) (11) (Reasoning.Main.Cap.observe (v)) : (Prod (Reasoning.Clinic.Chart) (Bool)))
public structure Cap.Run where
  state : Reasoning.Clinic.Chart
  trace : List (Reasoning.Main.Cap.Step)
  ledger : Reasoning.Main.Cap.Ledger
@[expose] public def Cap.start (v : Reasoning.Clinic.Vitals) : Reasoning.Main.Cap.Run := ({ state := Reasoning.Main.Cap.observe (v), trace := ([] : List (Reasoning.Main.Cap.Step)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := 0 } : Reasoning.Main.Cap.Ledger) } : Reasoning.Main.Cap.Run)
@[expose] public def Cap.step (__r : Reasoning.Main.Cap.Run) : Option (Reasoning.Main.Cap.Run) := (match Reasoning.Main.Cap.select ((__r).state) with | Option.none => Option.none | Option.some __step => (match Reasoning.Main.Cap.fire ((__r).state) (__step) with | Option.none => Option.none | Option.some __t => Option.some (({ state := __t, trace := (LexLeanRuntime.append ((__r).trace) ((__step :: ([] : List (Reasoning.Main.Cap.Step)))) : List (Reasoning.Main.Cap.Step)), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + Reasoning.Main.Cap.attempts ((__r).state)), firings := (((__r).ledger).firings + 1), expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : Reasoning.Main.Cap.Ledger) } : Reasoning.Main.Cap.Run))))
@[expose] public def Cap.run (v : Reasoning.Clinic.Vitals) : (Prod (Reasoning.Main.Cap.Run) (Bool)) := (LexLeanCollections.iterateUntil ((Reasoning.Main.Cap.step)) (11) (Reasoning.Main.Cap.start (v)) : (Prod (Reasoning.Main.Cap.Run) (Bool)))
@[expose] public def Cap.account (v : Reasoning.Clinic.Vitals) : Reasoning.Main.Cap.Ledger := (let __final : (Prod (Reasoning.Main.Cap.Run) (Bool)) := Reasoning.Main.Cap.run (v); (match (__final).2 with | Bool.true => ({ iterations := (((__final).1).ledger).iterations, attempts := ((((__final).1).ledger).attempts + Reasoning.Main.Cap.attempts (((__final).1).state)), firings := (((__final).1).ledger).firings, expansions := (((__final).1).ledger).expansions, verifications := (((__final).1).ledger).verifications, frontier := (((__final).1).ledger).frontier } : Reasoning.Main.Cap.Ledger) | Bool.false => ((__final).1).ledger))
@[expose] public def Cap.conclude (v : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) : Except ((Prod Bool Bool)) (Nat) := (match Reasoning.Main.Cap.accept (v) (__s) with | Option.some __v => Except.ok (__v) | Option.none => (match Reasoning.Main.Cap.extract (__s) with | Option.some _ => Except.error (((true, false) : Prod Bool Bool)) | Option.none => Except.error (((false, true) : Prod Bool Bool))))
@[expose] public def Cap.verdict (v : Reasoning.Clinic.Vitals) : Except ((Prod Bool Bool)) (Nat) := (let __final : (Prod (Reasoning.Clinic.Chart) (Bool)) := Reasoning.Main.Cap.saturate (v); (match (__final).2 with | Bool.true => Reasoning.Main.Cap.conclude (v) ((__final).1) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
@[expose] public def Cap (v : Reasoning.Clinic.Vitals) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (Reasoning.Main.Cap.Step)))) := (let __final : (Prod (Reasoning.Main.Cap.Run) (Bool)) := Reasoning.Main.Cap.run (v); (match (__final).2 with | Bool.true => (match Reasoning.Main.Cap.conclude (v) (((__final).1).state) with | Except.ok __v => Except.ok ((__v, ((__final).1).trace)) | Except.error __e => Except.error (__e)) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
public theorem Cap.fire_sound (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Main.Cap.Step) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Main.Cap.fire (__s) (__step) = Option.some (__t)) -> Reasoning.Clinic.Justified (__s) (__t)) :=
by
  cases __step with
  | Fever =>
    exact Reasoning.Clinic.Fever.apply_sound __s __t
  | Tachycardia =>
    exact Reasoning.Clinic.Tachycardia.apply_sound __s __t
  | Tachypnea =>
    exact Reasoning.Clinic.Tachypnea.apply_sound __s __t
  | Leukocytosis =>
    exact Reasoning.Clinic.Leukocytosis.apply_sound __s __t
  | Sirs =>
    exact Reasoning.Clinic.Sirs.apply_sound __s __t
  | Sepsis =>
    exact Reasoning.Clinic.Sepsis.apply_sound __s __t
  | Shock =>
    exact Reasoning.Clinic.Shock.apply_sound __s __t
  | Escalate =>
    exact Reasoning.Clinic.Escalate.apply_sound __s __t
public theorem Cap.replay_fire (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Main.Cap.Step) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Main.Cap.fire (__s) (__step) = Option.some (__t)) -> (Reasoning.Main.Cap.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [Reasoning.Main.Cap.replay]
  rw [llE]
public theorem Cap.replay_sound (v : Reasoning.Clinic.Vitals) (__acc : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart)) (__step : Reasoning.Main.Cap.Step) : ((LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Main.Cap.observe (v)) (__acc)) -> (LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Main.Cap.observe (v)) (Reasoning.Main.Cap.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [Reasoning.Main.Cap.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (Reasoning.Main.Cap.fire_sound llS __step _ ‹_›)
public theorem Cap.derivation (v : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Main.Cap.Step)) : (LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Main.Cap.observe (v)) (Reasoning.Main.Cap.follow (v) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((Reasoning.Main.Cap.replay)) (fun (__acc : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart)) => (LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Main.Cap.observe (v)) (__acc))) (Reasoning.Main.Cap.replay_sound (v)) (__trace) (Except.ok (Reasoning.Main.Cap.observe (v))) (LexLeanReasoning.reachesStart ((Reasoning.Clinic.Justified)) (Reasoning.Main.Cap.observe (v))))
public theorem Cap.follow_invariant (v : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Main.Cap.Step)) (__s : Reasoning.Clinic.Chart) : (Reasoning.Clinic.Consistent (Reasoning.Main.Cap.observe (v)) -> ((Reasoning.Main.Cap.follow (v) (__trace) = Except.ok (__s)) -> Reasoning.Clinic.Consistent (__s))) :=
  (fun llI llE => (LexLeanReasoning.starPreserves ((Reasoning.Clinic.Justified)) ((Reasoning.Clinic.Consistent)) Reasoning.Clinic.Findings.preserves (Reasoning.Main.Cap.observe (v)) (__s) (Reasoning.Main.Cap.derivation (v) (__trace) (__s) llE) llI))
public theorem Cap.accept_sound (v : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) (__v : Nat) : ((Reasoning.Main.Cap.accept (v) (__s) = Option.some (__v)) -> Reasoning.Main.Graded (v) (__v)) :=
  (fun llE => (Reasoning.Main.cap_correct (v) (__s) (__v) llE))
public theorem Cap.next_sound (__s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Main.Cap.next (__s) = Option.some (__t)) -> Reasoning.Clinic.Justified (__s) (__t)) :=
by
  intro llE
  dsimp only [Reasoning.Main.Cap.next] at llE
  split at llE
  · cases llE
  · exact Reasoning.Main.Cap.fire_sound __s _ __t llE
public theorem Cap.next_preserves (__s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Main.Cap.next (__s) = Option.some (__t)) -> (Reasoning.Clinic.Consistent (__s) -> Reasoning.Clinic.Consistent (__t))) :=
  (fun llE llH => (Reasoning.Clinic.consistent_preserved (__s) (__t) llH (Reasoning.Main.Cap.next_sound (__s) (__t) llE)))
public theorem Cap.step_none (__r : Reasoning.Main.Cap.Run) : ((Reasoning.Main.Cap.step (__r) = Option.none) -> (Reasoning.Main.Cap.next ((__r).state) = Option.none)) :=
by
  intro llE
  dsimp only [Reasoning.Main.Cap.step] at llE
  split at llE
  · rename_i llH
    dsimp only [Reasoning.Main.Cap.next]
    rw [llH]
  · rename_i llH
    split at llE
    · dsimp only [Reasoning.Main.Cap.next]
      rw [llH]
      assumption
    · cases llE
public theorem Cap.step_some (__r : Reasoning.Main.Cap.Run) (__q : Reasoning.Main.Cap.Run) : ((Reasoning.Main.Cap.step (__r) = Option.some (__q)) -> (Reasoning.Main.Cap.next ((__r).state) = Option.some ((__q).state))) :=
by
  intro llE
  dsimp only [Reasoning.Main.Cap.step] at llE
  split at llE
  · cases llE
  · rename_i llH
    split at llE
    · cases llE
    · cases llE
      dsimp only [Reasoning.Main.Cap.next]
      rw [llH]
      assumption
public theorem Cap.step_trace (v : Reasoning.Clinic.Vitals) (__r : Reasoning.Main.Cap.Run) (__q : Reasoning.Main.Cap.Run) : ((Reasoning.Main.Cap.step (__r) = Option.some (__q)) -> ((Reasoning.Main.Cap.follow (v) ((__r).trace) = Except.ok ((__r).state)) -> (Reasoning.Main.Cap.follow (v) ((__q).trace) = Except.ok ((__q).state)))) :=
by
  intro llE llH
  dsimp only [Reasoning.Main.Cap.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      dsimp only [Reasoning.Main.Cap.follow]
      rw [LexLeanReasoning.foldSnoc]
      dsimp only [Reasoning.Main.Cap.follow] at llH
      rw [llH]
      apply Reasoning.Main.Cap.replay_fire
      assumption
public theorem Cap.step_count (__r : Reasoning.Main.Cap.Run) (__q : Reasoning.Main.Cap.Run) : ((Reasoning.Main.Cap.step (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Main.Cap.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Cap.step_fired (__r : Reasoning.Main.Cap.Run) (__q : Reasoning.Main.Cap.Run) : ((Reasoning.Main.Cap.step (__r) = Option.some (__q)) -> (((__q).ledger).firings = (((__r).ledger).firings + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Main.Cap.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Cap.saturate_derivation (v : Reasoning.Clinic.Vitals) : (LexLeanReasoning.Star ((Reasoning.Clinic.Justified)) (Reasoning.Main.Cap.observe (v)) ((Reasoning.Main.Cap.saturate (v)).1)) :=
by
  dsimp only [Reasoning.Main.Cap.saturate]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Main.Cap.next)) (fun (__s : Reasoning.Clinic.Chart) => (LexLeanReasoning.Star ((Reasoning.Clinic.Justified)) (Reasoning.Main.Cap.observe (v)) (__s))) (fun llA llB llE llH => (LexLeanReasoning.Star.tail _ llA llB llH (Reasoning.Main.Cap.next_sound llA llB llE))) (11) (Reasoning.Main.Cap.observe (v)) (LexLeanReasoning.Star.refl _))
public theorem Cap.run_state (v : Reasoning.Clinic.Vitals) : ((((Reasoning.Main.Cap.run (v)).1).state = (Reasoning.Main.Cap.saturate (v)).1) /\ ((Reasoning.Main.Cap.run (v)).2 = (Reasoning.Main.Cap.saturate (v)).2)) :=
by
  dsimp only [Reasoning.Main.Cap.run, Reasoning.Main.Cap.saturate]
  exact (LexLeanReasoning.iterateUntilSimulate ((Reasoning.Main.Cap.step)) ((Reasoning.Main.Cap.next)) ((fun (__r : Reasoning.Main.Cap.Run) => (__r).state)) Reasoning.Main.Cap.step_none Reasoning.Main.Cap.step_some (11) (Reasoning.Main.Cap.start (v)))
public theorem Cap.run_trace (v : Reasoning.Clinic.Vitals) : (Reasoning.Main.Cap.follow (v) (((Reasoning.Main.Cap.run (v)).1).trace) = Except.ok (((Reasoning.Main.Cap.run (v)).1).state)) :=
by
  dsimp only [Reasoning.Main.Cap.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Main.Cap.step)) (fun (__r : Reasoning.Main.Cap.Run) => (Reasoning.Main.Cap.follow (v) ((__r).trace) = Except.ok ((__r).state))) (Reasoning.Main.Cap.step_trace (v)) (11) (Reasoning.Main.Cap.start (v)) rfl)
public theorem Cap.iterations_bounded (v : Reasoning.Clinic.Vitals) : ((((Reasoning.Main.Cap.run (v)).1).ledger).iterations <= 11) :=
by
  dsimp only [Reasoning.Main.Cap.run]
  exact (LexLeanReasoning.iterateUntilCount ((Reasoning.Main.Cap.step)) ((fun (__r : Reasoning.Main.Cap.Run) => ((__r).ledger).iterations)) Reasoning.Main.Cap.step_count (11) (Reasoning.Main.Cap.start (v)) rfl)
public theorem Cap.firings_bounded (v : Reasoning.Clinic.Vitals) : ((((Reasoning.Main.Cap.run (v)).1).ledger).firings <= 11) :=
by
  dsimp only [Reasoning.Main.Cap.run]
  exact (LexLeanReasoning.iterateUntilCount ((Reasoning.Main.Cap.step)) ((fun (__r : Reasoning.Main.Cap.Run) => ((__r).ledger).firings)) Reasoning.Main.Cap.step_fired (11) (Reasoning.Main.Cap.start (v)) rfl)
public theorem Cap.saturate_invariant (v : Reasoning.Clinic.Vitals) : (Reasoning.Clinic.Consistent (Reasoning.Main.Cap.observe (v)) -> Reasoning.Clinic.Consistent ((Reasoning.Main.Cap.saturate (v)).1)) :=
by
  dsimp only [Reasoning.Main.Cap.saturate]
  exact (fun llI => (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Main.Cap.next)) ((Reasoning.Clinic.Consistent)) Reasoning.Main.Cap.next_preserves (11) (Reasoning.Main.Cap.observe (v)) llI))
public theorem Cap.conclude_accept (v : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) (__v : Nat) : ((Reasoning.Main.Cap.conclude (v) (__s) = Except.ok (__v)) -> (Reasoning.Main.Cap.accept (v) (__s) = Option.some (__v))) :=
by
  intro llE
  dsimp only [Reasoning.Main.Cap.conclude] at llE
  split at llE
  · cases llE
    assumption
  · split at llE
    · cases llE
    · cases llE
public theorem Cap.conclude_sound (v : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) (__v : Nat) : ((Reasoning.Main.Cap.conclude (v) (__s) = Except.ok (__v)) -> Reasoning.Main.Graded (v) (__v)) :=
by
  intro llE
  dsimp only [Reasoning.Main.Cap.conclude] at llE
  split at llE
  · cases llE
    exact Reasoning.Main.Cap.accept_sound _ _ _ ‹_›
  · split at llE
    · cases llE
    · cases llE
public theorem Cap.verdict_sound (v : Reasoning.Clinic.Vitals) : (forall (__v : Nat), ((Reasoning.Main.Cap.verdict (v) = Except.ok (__v)) -> Reasoning.Main.Graded (v) (__v))) :=
by
  intro llV llE
  dsimp only [Reasoning.Main.Cap.verdict] at llE
  generalize llRun : Reasoning.Main.Cap.saturate v = llR at llE
  split at llE
  · exact Reasoning.Main.Cap.conclude_sound _ _ _ llE
  · cases llE
public theorem Cap.explained (v : Reasoning.Clinic.Vitals) : (forall (__v : Nat), (forall (__trace : List (Reasoning.Main.Cap.Step)), ((Reasoning.Main.Cap (v) = Except.ok ((__v, __trace))) -> ((Reasoning.Main.Cap.answer (v) (__trace) = Option.some (__v)) /\ Reasoning.Main.Graded (v) (__v))))) :=
by
  intro llV llT llE
  have llTrace := Reasoning.Main.Cap.run_trace v
  dsimp only [Reasoning.Main.Cap] at llE
  generalize llRun : Reasoning.Main.Cap.run v = llR at llE llTrace
  split at llE
  · split at llE
    · rename_i llW llH
      cases llE
      exact And.intro (by dsimp only [Reasoning.Main.Cap.answer]; rw [llTrace]; exact Reasoning.Main.Cap.conclude_accept _ _ _ llH) (Reasoning.Main.Cap.conclude_sound _ _ _ llH)
    · cases llE
  · cases llE

@[expose] public def triageLevel (temperature : Nat) (heartRate : Nat) (respiratoryRate : Nat) (whiteCells : Nat) (systolic : Nat) (infection : Nat) : Nat := (match Reasoning.Clinic.Triage.verdict (({ temperature := temperature, heartRate := heartRate, respiratoryRate := respiratoryRate, whiteCells := whiteCells, systolic := systolic, infection := infection } : Reasoning.Clinic.Vitals)) with | Except.ok level => level | Except.error _ => 9)

@[expose] public def gatewayLevel (temperature : Nat) (heartRate : Nat) (respiratoryRate : Nat) (whiteCells : Nat) (systolic : Nat) (infection : Nat) : Nat := (match GatewayModel (({ temperature := temperature, heartRate := heartRate, respiratoryRate := respiratoryRate, whiteCells := whiteCells, systolic := systolic, infection := infection } : Reasoning.Clinic.Vitals)) with | Except.ok level => level | Except.error _ => 9)

@[expose] public def triageSteps (temperature : Nat) (heartRate : Nat) (respiratoryRate : Nat) (whiteCells : Nat) (systolic : Nat) (infection : Nat) : Nat := (match Reasoning.Clinic.Triage (({ temperature := temperature, heartRate := heartRate, respiratoryRate := respiratoryRate, whiteCells := whiteCells, systolic := systolic, infection := infection } : Reasoning.Clinic.Vitals)) with | Except.ok explained => (LexLeanRuntime.length ((explained).2) : Nat) | Except.error _ => 0)

@[expose] public def planLeft (target : Nat) : Nat := (match Reasoning.Planner.Plan.verdict (target) with | Except.ok jugs => (jugs).1 | Except.error _ => 9)

@[expose] public def doseLevel (w : Nat) : Nat := (match Reasoning.Screening.Dose.verdict (w) with | Except.ok dose => dose | Except.error _ => 9)

@[expose] public def gradeLevel (temperature : Nat) (heartRate : Nat) (respiratoryRate : Nat) (whiteCells : Nat) (systolic : Nat) (infection : Nat) : Nat := (match Grade.verdict (({ temperature := temperature, heartRate := heartRate, respiratoryRate := respiratoryRate, whiteCells := whiteCells, systolic := systolic, infection := infection } : Reasoning.Clinic.Vitals)) with | Except.ok level => level | Except.error _ => 9)

@[expose] public def screenDose (w : Nat) : Nat := (match Reasoning.Screening.Screen.verdict (w) with | Except.ok dose => dose | Except.error _ => 9)

@[expose] public def spendLeft (x : Nat) : Nat := (match Reasoning.Budget.Spend.verdict (Nat) ((x, x)) with | Except.ok left => left | Except.error _ => 9)

public theorem triage_shock : (Reasoning.Clinic.Triage (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals)) = Except.ok ((3, (Reasoning.Clinic.Triage.Step.Fever :: (Reasoning.Clinic.Triage.Step.Tachycardia :: (Reasoning.Clinic.Triage.Step.Tachypnea :: (Reasoning.Clinic.Triage.Step.Leukocytosis :: (Reasoning.Clinic.Triage.Step.Sirs :: (Reasoning.Clinic.Triage.Step.Sepsis :: (Reasoning.Clinic.Triage.Step.Shock :: (Reasoning.Clinic.Triage.Step.Escalate :: (Reasoning.Clinic.Triage.Step.Escalate :: (Reasoning.Clinic.Triage.Step.Escalate :: ([] : List (Reasoning.Clinic.Triage.Step))))))))))))))) := by
  rfl

public theorem triage_level : (triageLevel (392) (118) (26) (15) (82) (1) = 3) := by
  rfl

public theorem triage_replays : (Reasoning.Clinic.Triage.answer (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals)) ((Reasoning.Clinic.Triage.Step.Fever :: (Reasoning.Clinic.Triage.Step.Tachycardia :: (Reasoning.Clinic.Triage.Step.Tachypnea :: (Reasoning.Clinic.Triage.Step.Leukocytosis :: (Reasoning.Clinic.Triage.Step.Sirs :: (Reasoning.Clinic.Triage.Step.Sepsis :: (Reasoning.Clinic.Triage.Step.Shock :: (Reasoning.Clinic.Triage.Step.Escalate :: (Reasoning.Clinic.Triage.Step.Escalate :: (Reasoning.Clinic.Triage.Step.Escalate :: ([] : List (Reasoning.Clinic.Triage.Step))))))))))))) = Option.some (3)) := by
  rfl

public theorem triage_iterations : ((((Reasoning.Clinic.Triage.run (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).1).ledger).iterations = 10) := by
  rfl

public theorem triage_well : (Reasoning.Clinic.Triage (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals)) = Except.ok ((0, ([] : List (Reasoning.Clinic.Triage.Step))))) := by
  rfl

public theorem triage_forged : (Reasoning.Clinic.Triage.follow (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals)) ((Reasoning.Clinic.Triage.Step.Sepsis :: ([] : List (Reasoning.Clinic.Triage.Step)))) = Except.error (((true, true) : Prod Bool Bool))) := by
  rfl

public theorem triage_inapplicable : (Reasoning.Clinic.Triage.fire (Reasoning.Clinic.Triage.observe (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))) (Reasoning.Clinic.Triage.Step.Shock) = Option.none) := by
  rfl

public theorem review_exhausted : (Reasoning.Clinic.Review.verdict (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals)) = Except.error (((false, false) : Prod Bool Bool))) := by
  rfl

public theorem review_rejected : (Reasoning.Clinic.Review.verdict (({ temperature := 390, heartRate := 100, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals)) = Except.error (((true, false) : Prod Bool Bool))) := by
  rfl

public theorem gateway_level : (gatewayLevel (392) (118) (26) (15) (82) (1) = 3) := by
  rfl

public theorem spend_drained : (Reasoning.Budget.Spend (Nat) ((2, 2)) = Except.ok ((0, (Reasoning.Budget.Spend.Step.Tick :: (Reasoning.Budget.Spend.Step.Tick :: (Reasoning.Budget.Spend.Step.Tick :: (Reasoning.Budget.Spend.Step.Burn (2) :: ([] : List (Reasoning.Budget.Spend.Step (Nat)))))))))) := by
  rfl

public theorem refund_drained : (Reasoning.Budget.Refund.verdict (2) = Except.ok (0)) := by
  rfl

public theorem refund_unsolved : (Reasoning.Budget.Refund.verdict (7) = Except.error (((false, true) : Prod Bool Bool))) := by
  rfl

public theorem plan_found : (Reasoning.Planner.Plan (2) = Except.ok (((2, 2), (Reasoning.Planner.Plan.Step.FillA :: (Reasoning.Planner.Plan.Step.Pour (2) :: ([] : List (Reasoning.Planner.Plan.Step))))))) := by
  rfl

public theorem plan_unsolved : (Reasoning.Planner.Plan.verdict (5) = Except.error (((false, true) : Prod Bool Bool))) := by
  rfl

public theorem plan_frontier : ((((Reasoning.Planner.Plan.run (2)).1).ledger).frontier = 4) := by
  rfl

public theorem screen_found : (Reasoning.Screening.Screen (60) = Except.ok ((30, (Reasoning.Screening.Screen.Step.Propose (30) :: ([] : List (Reasoning.Screening.Screen.Step)))))) := by
  rfl

public theorem grade_shock : (gradeLevel (392) (118) (26) (15) (82) (1) = 3) := by
  rfl

public theorem dose_found : (Reasoning.Screening.Dose (60) = Except.ok ((30, (Reasoning.Screening.Dose.Step.candidate (30) :: ([] : List (Reasoning.Screening.Dose.Step)))))) := by
  rfl

public theorem dose_exhausted : (Reasoning.Screening.Dose.verdict (120) = Except.error (((false, false) : Prod Bool Bool))) := by
  rfl

public theorem screen_truncated : (Reasoning.Screening.Screen.verdict (100) = Except.error (((false, false) : Prod Bool Bool))) := by
  rfl

public theorem triage_ledger : (((Reasoning.Clinic.Triage.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).iterations, ((Reasoning.Clinic.Triage.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).attempts, ((Reasoning.Clinic.Triage.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).firings, ((Reasoning.Clinic.Triage.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).expansions, ((Reasoning.Clinic.Triage.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).verifications, (Reasoning.Clinic.Triage.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).frontier))))) = (10, (60, (10, (0, (1, 0)))))) := by
  rfl

public theorem triage_well_ledger : (((Reasoning.Clinic.Triage.account (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).iterations, ((Reasoning.Clinic.Triage.account (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).attempts, ((Reasoning.Clinic.Triage.account (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).firings, ((Reasoning.Clinic.Triage.account (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).expansions, ((Reasoning.Clinic.Triage.account (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).verifications, (Reasoning.Clinic.Triage.account (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).frontier))))) = (0, (8, (0, (0, (1, 0)))))) := by
  rfl

public theorem review_ledger : (((Reasoning.Clinic.Review.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).iterations, ((Reasoning.Clinic.Review.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).attempts, ((Reasoning.Clinic.Review.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).firings, ((Reasoning.Clinic.Review.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).expansions, ((Reasoning.Clinic.Review.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).verifications, (Reasoning.Clinic.Review.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).frontier))))) = (6, (21, (6, (0, (0, 0)))))) := by
  rfl

public theorem grade_ledger : (((Grade.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).iterations, ((Grade.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).attempts, ((Grade.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).firings, ((Grade.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).expansions, ((Grade.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).verifications, (Grade.account (({ temperature := 392, heartRate := 118, respiratoryRate := 26, whiteCells := 15, systolic := 82, infection := 1 } : Reasoning.Clinic.Vitals))).frontier))))) = (10, (60, (10, (0, (0, 0)))))) := by
  rfl

public theorem grade_well_ledger : (((Grade.account (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).iterations, ((Grade.account (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).attempts, ((Grade.account (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).firings, ((Grade.account (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).expansions, ((Grade.account (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).verifications, (Grade.account (({ temperature := 368, heartRate := 72, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).frontier))))) = (0, (8, (0, (0, (0, 0)))))) := by
  rfl

public theorem review_rejected_ledger : (((Reasoning.Clinic.Review.account (({ temperature := 390, heartRate := 100, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).iterations, ((Reasoning.Clinic.Review.account (({ temperature := 390, heartRate := 100, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).attempts, ((Reasoning.Clinic.Review.account (({ temperature := 390, heartRate := 100, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).firings, ((Reasoning.Clinic.Review.account (({ temperature := 390, heartRate := 100, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).expansions, ((Reasoning.Clinic.Review.account (({ temperature := 390, heartRate := 100, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).verifications, (Reasoning.Clinic.Review.account (({ temperature := 390, heartRate := 100, respiratoryRate := 14, whiteCells := 7, systolic := 120, infection := 0 } : Reasoning.Clinic.Vitals))).frontier))))) = (4, (24, (4, (0, (1, 0)))))) := by
  rfl

public theorem spend_ledger : (((Reasoning.Budget.Spend.account (Nat) ((2, 2))).iterations, ((Reasoning.Budget.Spend.account (Nat) ((2, 2))).attempts, ((Reasoning.Budget.Spend.account (Nat) ((2, 2))).firings, ((Reasoning.Budget.Spend.account (Nat) ((2, 2))).expansions, ((Reasoning.Budget.Spend.account (Nat) ((2, 2))).verifications, (Reasoning.Budget.Spend.account (Nat) ((2, 2))).frontier))))) = (4, (8, (4, (0, (1, 0)))))) := by
  rfl

public theorem refund_ledger : (((Reasoning.Budget.Refund.account (2)).iterations, ((Reasoning.Budget.Refund.account (2)).attempts, ((Reasoning.Budget.Refund.account (2)).firings, ((Reasoning.Budget.Refund.account (2)).expansions, ((Reasoning.Budget.Refund.account (2)).verifications, (Reasoning.Budget.Refund.account (2)).frontier))))) = (4, (8, (4, (0, (1, 0)))))) := by
  rfl

public theorem refund_unsolved_ledger : (((Reasoning.Budget.Refund.account (7)).iterations, ((Reasoning.Budget.Refund.account (7)).attempts, ((Reasoning.Budget.Refund.account (7)).firings, ((Reasoning.Budget.Refund.account (7)).expansions, ((Reasoning.Budget.Refund.account (7)).verifications, (Reasoning.Budget.Refund.account (7)).frontier))))) = (9, (13, (9, (0, (1, 0)))))) := by
  rfl

public theorem plan_ledger : (((((Reasoning.Planner.Plan.run (2)).1).ledger).iterations, ((((Reasoning.Planner.Plan.run (2)).1).ledger).attempts, ((((Reasoning.Planner.Plan.run (2)).1).ledger).firings, ((((Reasoning.Planner.Plan.run (2)).1).ledger).expansions, ((((Reasoning.Planner.Plan.run (2)).1).ledger).verifications, (((Reasoning.Planner.Plan.run (2)).1).ledger).frontier))))) = (4, (15, (8, (3, (4, 4)))))) := by
  rfl

public theorem plan_unsolved_ledger : (((((Reasoning.Planner.Plan.run (5)).1).ledger).iterations, ((((Reasoning.Planner.Plan.run (5)).1).ledger).attempts, ((((Reasoning.Planner.Plan.run (5)).1).ledger).firings, ((((Reasoning.Planner.Plan.run (5)).1).ledger).expansions, ((((Reasoning.Planner.Plan.run (5)).1).ledger).verifications, (((Reasoning.Planner.Plan.run (5)).1).ledger).frontier))))) = (20, (100, (51, (20, (20, 10)))))) := by
  rfl

public theorem screen_ledger : (((((Reasoning.Screening.Screen.run (60)).1).ledger).iterations, ((((Reasoning.Screening.Screen.run (60)).1).ledger).attempts, ((((Reasoning.Screening.Screen.run (60)).1).ledger).firings, ((((Reasoning.Screening.Screen.run (60)).1).ledger).expansions, ((((Reasoning.Screening.Screen.run (60)).1).ledger).verifications, (((Reasoning.Screening.Screen.run (60)).1).ledger).frontier))))) = (4, (12, (4, (3, (4, 3)))))) := by
  rfl

public theorem screen_unsolved_ledger : (((((Reasoning.Screening.Screen.run (0)).1).ledger).iterations, ((((Reasoning.Screening.Screen.run (0)).1).ledger).attempts, ((((Reasoning.Screening.Screen.run (0)).1).ledger).firings, ((((Reasoning.Screening.Screen.run (0)).1).ledger).expansions, ((((Reasoning.Screening.Screen.run (0)).1).ledger).verifications, (((Reasoning.Screening.Screen.run (0)).1).ledger).frontier))))) = (1, (4, (0, (1, (1, 1)))))) := by
  rfl

public theorem screen_truncated_ledger : (((((Reasoning.Screening.Screen.run (100)).1).ledger).iterations, ((((Reasoning.Screening.Screen.run (100)).1).ledger).attempts, ((((Reasoning.Screening.Screen.run (100)).1).ledger).firings, ((((Reasoning.Screening.Screen.run (100)).1).ledger).expansions, ((((Reasoning.Screening.Screen.run (100)).1).ledger).verifications, (((Reasoning.Screening.Screen.run (100)).1).ledger).frontier))))) = (4, (16, (4, (4, (4, 3)))))) := by
  rfl

public theorem dose_ledger : ((((Reasoning.Screening.Dose.run (60)).ledger).iterations, (((Reasoning.Screening.Dose.run (60)).ledger).attempts, (((Reasoning.Screening.Dose.run (60)).ledger).firings, (((Reasoning.Screening.Dose.run (60)).ledger).expansions, (((Reasoning.Screening.Dose.run (60)).ledger).verifications, ((Reasoning.Screening.Dose.run (60)).ledger).frontier))))) = (3, (3, (0, (0, (3, 0)))))) := by
  rfl

public theorem dose_exhausted_ledger : ((((Reasoning.Screening.Dose.run (120)).ledger).iterations, (((Reasoning.Screening.Dose.run (120)).ledger).attempts, (((Reasoning.Screening.Dose.run (120)).ledger).firings, (((Reasoning.Screening.Dose.run (120)).ledger).expansions, (((Reasoning.Screening.Dose.run (120)).ledger).verifications, ((Reasoning.Screening.Dose.run (120)).ledger).frontier))))) = (3, (3, (0, (0, (3, 0)))))) := by
  rfl

end Reasoning.Main
