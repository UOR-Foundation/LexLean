module
public import Init
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Reasoning.Screening

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

@[expose, reducible] public def Short (_w : Nat) (doses : List (Nat)) : Prop := ((LexLeanRuntime.length (doses) : Nat) <= 4)

@[expose] public def shortCheck (_w : Nat) (doses : List (Nat)) : Bool := (Nat.ble ((LexLeanRuntime.length (doses) : Nat)) (4))

public theorem short_sound (w : Nat) (doses : List (Nat)) : ((shortCheck (w) (doses) = true) -> Short (w) (doses)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Short, shortCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem DoseContract.postcondition_sound : LexLeanModels.Sound2 ((Reasoning.Screening.shortCheck)) ((Reasoning.Screening.Short)) := Reasoning.Screening.short_sound

@[expose] public def DoseCandidates (w : Nat) : List (Nat) := ((LexLeanRuntime.subtract (w) (10) : Nat) :: ((LexLeanRuntime.subtract (w) (20) : Nat) :: ((LexLeanRuntime.subtract (w) (30) : Nat) :: ((LexLeanRuntime.subtract (w) (50) : Nat) :: ([] : List (Nat))))))

@[expose] public def DoseModel (__input : Nat) : List (Nat) := Reasoning.Screening.DoseCandidates (__input)

@[expose, reducible] public def Proposed (s : (Prod (Nat) (Nat))) (t : (Prod (Nat) (Nat))) : Prop := (((s).2 = 0) /\ (((t).1 = (s).1) /\ (0 < (t).2)))


public theorem propose_sound (s : (Prod (Nat) (Nat))) (d : Nat) : ((((Nat.beq ((s).2) (0)) && (Nat.blt (0) (d))) = true) -> Proposed (s) (((s).1, d))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Proposed, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose] public def Propose.guard (s : (Prod (Nat) (Nat))) (d : Nat) : Bool := ((Nat.beq ((s).2) (0)) && (Nat.blt (0) (d)))
@[expose] public def Propose.conclusion (s : (Prod (Nat) (Nat))) (d : Nat) : (Prod (Nat) (Nat)) := ((s).1, d)
@[expose] public def Propose.candidates (s : (Prod (Nat) (Nat))) : List (Nat) := (match (let __checked1_input : Nat := (s).1; (let __checked1_output : List (Nat) := Reasoning.Screening.DoseModel (__checked1_input); (if Reasoning.Screening.shortCheck (__checked1_input) (__checked1_output) then Except.ok (__checked1_output) else Except.error (((true, false) : Prod Bool Bool))))) with | Except.ok proposed => proposed | Except.error _ => ([] : List (Nat)))
@[expose] public def Propose.apply (s : (Prod (Nat) (Nat))) (d : Nat) : Option ((Prod (Nat) (Nat))) := (if Reasoning.Screening.Propose.guard (s) (d) then Option.some (Reasoning.Screening.Propose.conclusion (s) (d)) else Option.none)
public theorem Propose.apply_sound (s : (Prod (Nat) (Nat))) (d : Nat) (__t : (Prod (Nat) (Nat))) : ((Reasoning.Screening.Propose.apply (s) (d) = Option.some (__t)) -> Reasoning.Screening.Proposed (s) (__t)) :=
  (LexLeanReasoning.guarded ((Reasoning.Screening.Proposed)) (s) (Reasoning.Screening.Propose.guard (s) (d)) (Reasoning.Screening.Propose.conclusion (s) (d)) (Reasoning.Screening.propose_sound (s) (d)) (__t))

@[expose, reducible] public def Safe (w : Nat) (r : Nat) : Prop := ((r + r) <= w)

@[expose] public def safeCheck (w : Nat) (r : Nat) : Bool := (Nat.ble ((r + r)) (w))

public theorem safe_sound (w : Nat) (r : Nat) : ((safeCheck (w) (r) = true) -> Safe (w) (r)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Safe, safeCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem SafeDose.sound : (LexLeanReasoning.Sound ((Reasoning.Screening.safeCheck)) ((Reasoning.Screening.Safe))) :=
  Reasoning.Screening.safe_sound

public inductive Dose.Step where
  | candidate (_ : Nat)
public structure Dose.Ledger where
  iterations : Nat
  attempts : Nat
  firings : Nat
  expansions : Nat
  verifications : Nat
  frontier : Nat
@[expose] public def Dose.candidates (w : Nat) : List (Nat) := (match (let __checked1_input : Nat := w; (let __checked1_output : List (Nat) := Reasoning.Screening.DoseModel (__checked1_input); (if Reasoning.Screening.shortCheck (__checked1_input) (__checked1_output) then Except.ok (__checked1_output) else Except.error (((true, false) : Prod Bool Bool))))) with | Except.ok proposed => proposed | Except.error _ => ([] : List (Nat)))
@[expose] public def Dose.verify (w : Nat) (__c : Nat) : Option (Nat) := (if Reasoning.Screening.safeCheck (w) (__c) then Option.some (__c) else Option.none)
public structure Dose.Trial where
  found : Option (Nat)
  truncated : Bool
  ledger : Reasoning.Screening.Dose.Ledger
@[expose] public def Dose.«try» (w : Nat) (__t : Reasoning.Screening.Dose.Trial) (__c : Nat) : Reasoning.Screening.Dose.Trial := (match (__t).found with | Option.some _ => __t | Option.none => (match (Nat.blt (((__t).ledger).verifications) (3)) with | Bool.true => ({ found := Reasoning.Screening.Dose.verify (w) (__c), truncated := (__t).truncated, ledger := ({ iterations := (((__t).ledger).iterations + 1), attempts := (((__t).ledger).attempts + 1), firings := ((__t).ledger).firings, expansions := ((__t).ledger).expansions, verifications := (((__t).ledger).verifications + 1), frontier := ((__t).ledger).frontier } : Reasoning.Screening.Dose.Ledger) } : Reasoning.Screening.Dose.Trial) | Bool.false => ({ found := Option.none, truncated := true, ledger := (__t).ledger } : Reasoning.Screening.Dose.Trial)))
@[expose] public def Dose.run (w : Nat) : Reasoning.Screening.Dose.Trial := (LexLeanCollections.listFold ((fun (__t : Reasoning.Screening.Dose.Trial) (__c : Nat) => Reasoning.Screening.Dose.«try» (w) (__t) (__c))) (({ found := Option.none, truncated := false, ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := 0 } : Reasoning.Screening.Dose.Ledger) } : Reasoning.Screening.Dose.Trial)) (Reasoning.Screening.Dose.candidates (w)) : Reasoning.Screening.Dose.Trial)
@[expose] public def Dose.failure (__t : Reasoning.Screening.Dose.Trial) : (Prod Bool Bool) := (match (__t).truncated with | Bool.true => ((false, false) : Prod Bool Bool) | Bool.false => (match (Nat.beq (((__t).ledger).verifications) (0)) with | Bool.true => ((false, true) : Prod Bool Bool) | Bool.false => ((true, false) : Prod Bool Bool)))
@[expose] public def Dose.verdict (w : Nat) : Except ((Prod Bool Bool)) (Nat) := (let __final : Reasoning.Screening.Dose.Trial := Reasoning.Screening.Dose.run (w); (match (__final).found with | Option.some __v => Except.ok (__v) | Option.none => Except.error (Reasoning.Screening.Dose.failure (__final))))
@[expose] public def Dose (w : Nat) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (Reasoning.Screening.Dose.Step)))) := (let __final : Reasoning.Screening.Dose.Trial := Reasoning.Screening.Dose.run (w); (match (__final).found with | Option.some __v => Except.ok ((__v, (Reasoning.Screening.Dose.Step.candidate (__v) :: ([] : List (Reasoning.Screening.Dose.Step))))) | Option.none => Except.error (Reasoning.Screening.Dose.failure (__final))))
@[expose] public def Dose.answer (w : Nat) (__trace : List (Reasoning.Screening.Dose.Step)) : Option (Nat) := (match __trace with | List.nil => Option.none | List.cons __step __rest => (match __rest with | List.nil => (match __step with | Reasoning.Screening.Dose.Step.candidate __c => Reasoning.Screening.Dose.verify (w) (__c)) | List.cons _ _ => Option.none))
public theorem Dose.verify_sound (w : Nat) (__c : Nat) (__v : Nat) : ((Reasoning.Screening.Dose.verify (w) (__c) = Option.some (__v)) -> ((Reasoning.Screening.Dose.verify (w) (__v) = Option.some (__v)) /\ Reasoning.Screening.Safe (w) (__v))) :=
by
  intro llE
  dsimp only [Reasoning.Screening.Dose.verify] at llE
  have llC := LexLeanReasoning.checked _ _ _ llE
  cases llC.right
  exact And.intro (by dsimp only [Reasoning.Screening.Dose.verify]; rw [llC.left]; rfl) (Reasoning.Screening.safe_sound _ _ llC.left)
public theorem Dose.try_sound (w : Nat) (__t : Reasoning.Screening.Dose.Trial) (__c : Nat) : ((forall (__v : Nat), (((__t).found = Option.some (__v)) -> ((Reasoning.Screening.Dose.verify (w) (__v) = Option.some (__v)) /\ Reasoning.Screening.Safe (w) (__v)))) -> (forall (__v : Nat), (((Reasoning.Screening.Dose.«try» (w) (__t) (__c)).found = Option.some (__v)) -> ((Reasoning.Screening.Dose.verify (w) (__v) = Option.some (__v)) /\ Reasoning.Screening.Safe (w) (__v))))) :=
by
  intro llH llV llE
  dsimp only [Reasoning.Screening.Dose.«try»] at llE
  split at llE
  · exact llH llV llE
  · split at llE
    · exact Reasoning.Screening.Dose.verify_sound _ _ _ llE
    · cases llE
public theorem Dose.try_count (w : Nat) (__t : Reasoning.Screening.Dose.Trial) (__c : Nat) : ((((__t).ledger).verifications <= 3) -> (((Reasoning.Screening.Dose.«try» (w) (__t) (__c)).ledger).verifications <= 3)) :=
by
  intro llH
  dsimp only [Reasoning.Screening.Dose.«try»]
  split
  · exact llH
  · split
    · rename_i llB
      exact LexLeanReasoning.bltSucc _ _ llB
    · exact llH
public theorem Dose.run_sound (w : Nat) : (forall (__v : Nat), (((Reasoning.Screening.Dose.run (w)).found = Option.some (__v)) -> ((Reasoning.Screening.Dose.verify (w) (__v) = Option.some (__v)) /\ Reasoning.Screening.Safe (w) (__v)))) :=
by
  dsimp only [Reasoning.Screening.Dose.run]
  exact (LexLeanReasoning.foldInvariant ((fun (__t : Reasoning.Screening.Dose.Trial) (__c : Nat) => Reasoning.Screening.Dose.«try» (w) (__t) (__c))) (fun (__t : Reasoning.Screening.Dose.Trial) => (forall (__v : Nat), (((__t).found = Option.some (__v)) -> ((Reasoning.Screening.Dose.verify (w) (__v) = Option.some (__v)) /\ Reasoning.Screening.Safe (w) (__v))))) (Reasoning.Screening.Dose.try_sound (w)) (Reasoning.Screening.Dose.candidates (w)) (({ found := Option.none, truncated := false, ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := 0 } : Reasoning.Screening.Dose.Ledger) } : Reasoning.Screening.Dose.Trial)) (fun llV llE => (LexLeanReasoning.noneSome _ llE)))
public theorem Dose.verifications_bounded (w : Nat) : (((Reasoning.Screening.Dose.run (w)).ledger).verifications <= 3) :=
by
  dsimp only [Reasoning.Screening.Dose.run]
  exact (LexLeanReasoning.foldInvariant ((fun (__t : Reasoning.Screening.Dose.Trial) (__c : Nat) => Reasoning.Screening.Dose.«try» (w) (__t) (__c))) (fun (__t : Reasoning.Screening.Dose.Trial) => (((__t).ledger).verifications <= 3)) (Reasoning.Screening.Dose.try_count (w)) (Reasoning.Screening.Dose.candidates (w)) (({ found := Option.none, truncated := false, ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := 0 } : Reasoning.Screening.Dose.Ledger) } : Reasoning.Screening.Dose.Trial)) (LexLeanReasoning.zeroLe _))
public theorem Dose.verdict_sound (w : Nat) : (forall (__v : Nat), ((Reasoning.Screening.Dose.verdict (w) = Except.ok (__v)) -> Reasoning.Screening.Safe (w) (__v))) :=
by
  intro llV llE
  have llS := Reasoning.Screening.Dose.run_sound w
  dsimp only [Reasoning.Screening.Dose.verdict] at llE
  generalize llRun : Reasoning.Screening.Dose.run w = llR at llE llS
  split at llE
  · rename_i llHit llF
    cases llE
    exact And.right (llS _ llF)
  · cases llE
public theorem Dose.explained (w : Nat) : (forall (__v : Nat), (forall (__trace : List (Reasoning.Screening.Dose.Step)), ((Reasoning.Screening.Dose (w) = Except.ok ((__v, __trace))) -> ((Reasoning.Screening.Dose.answer (w) (__trace) = Option.some (__v)) /\ Reasoning.Screening.Safe (w) (__v))))) :=
by
  intro llV llT llE
  have llS := Reasoning.Screening.Dose.run_sound w
  dsimp only [Reasoning.Screening.Dose] at llE
  generalize llRun : Reasoning.Screening.Dose.run w = llR at llE llS
  split at llE
  · rename_i llHit llF
    cases llE
    have llOk := llS _ llF
    exact And.intro (by dsimp only [Reasoning.Screening.Dose.answer]; exact And.left llOk) (And.right llOk)
  · cases llE

public inductive Screen.Step where
  | Propose (_ : Nat)
public structure Screen.Ledger where
  iterations : Nat
  attempts : Nat
  firings : Nat
  expansions : Nat
  verifications : Nat
  frontier : Nat
@[expose] public def Screen.observe (w : Nat) : (Prod (Nat) (Nat)) := (w, 0)
@[expose] public def Screen.fire (__s : (Prod (Nat) (Nat))) (__step : Reasoning.Screening.Screen.Step) : Option ((Prod (Nat) (Nat))) := (match __step with | Reasoning.Screening.Screen.Step.Propose __b => Reasoning.Screening.Propose.apply (__s) (__b))
@[expose] public def Screen.replay (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : Reasoning.Screening.Screen.Step) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match Reasoning.Screening.Screen.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def Screen.follow (w : Nat) (__trace : List (Reasoning.Screening.Screen.Step)) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (LexLeanCollections.listFold ((Reasoning.Screening.Screen.replay)) (Except.ok (Reasoning.Screening.Screen.observe (w))) (__trace) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))))
@[expose] public def Screen.extract (pick : (Prod (Nat) (Nat))) : Option (Nat) := (if (Nat.blt (0) ((pick).2)) then Option.some ((pick).2) else Option.none)
@[expose] public def Screen.accept (w : Nat) (__s : (Prod (Nat) (Nat))) : Option (Nat) := (match Reasoning.Screening.Screen.extract (__s) with | Option.none => Option.none | Option.some __v => (if Reasoning.Screening.safeCheck (w) (__v) then Option.some (__v) else Option.none))
@[expose] public def Screen.answer (w : Nat) (__trace : List (Reasoning.Screening.Screen.Step)) : Option (Nat) := (match Reasoning.Screening.Screen.follow (w) (__trace) with | Except.ok __s => Reasoning.Screening.Screen.accept (w) (__s) | Except.error _ => Option.none)
public structure Screen.Node where
  state : (Prod (Nat) (Nat))
  trace : List (Reasoning.Screening.Screen.Step)
public structure Screen.Search where
  frontier : List (Reasoning.Screening.Screen.Node)
  found : Option ((Prod (Nat) (Reasoning.Screening.Screen.Node)))
  truncated : Bool
  ledger : Reasoning.Screening.Screen.Ledger
@[expose] public def Screen.Propose.collect (__node : Reasoning.Screening.Screen.Node) (__acc : List (Reasoning.Screening.Screen.Node)) (__b : Nat) : List (Reasoning.Screening.Screen.Node) := (match Reasoning.Screening.Screen.fire ((__node).state) (Reasoning.Screening.Screen.Step.Propose (__b)) with | Option.none => __acc | Option.some __t => (LexLeanRuntime.append (__acc) ((({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((Reasoning.Screening.Screen.Step.Propose (__b) :: ([] : List (Reasoning.Screening.Screen.Step)))) : List (Reasoning.Screening.Screen.Step)) } : Reasoning.Screening.Screen.Node) :: ([] : List (Reasoning.Screening.Screen.Node)))) : List (Reasoning.Screening.Screen.Node)))
@[expose] public def Screen.Propose.successors (__node : Reasoning.Screening.Screen.Node) : List (Reasoning.Screening.Screen.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Reasoning.Screening.Screen.Node)) (__b : Nat) => Reasoning.Screening.Screen.Propose.collect (__node) (__acc) (__b))) (([] : List (Reasoning.Screening.Screen.Node))) (Reasoning.Screening.Propose.candidates ((__node).state)) : List (Reasoning.Screening.Screen.Node))
@[expose] public def Screen.successors (__node : Reasoning.Screening.Screen.Node) : List (Reasoning.Screening.Screen.Node) := Reasoning.Screening.Screen.Propose.successors (__node)
@[expose] public def Screen.start (w : Nat) : Reasoning.Screening.Screen.Search := (let __first : List (Reasoning.Screening.Screen.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Reasoning.Screening.Screen.Node)) (__n : Reasoning.Screening.Screen.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) (3)) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (Reasoning.Screening.Screen.Node)))) : List (Reasoning.Screening.Screen.Node)) else __acc))) (([] : List (Reasoning.Screening.Screen.Node))) ((({ state := Reasoning.Screening.Screen.observe (w), trace := ([] : List (Reasoning.Screening.Screen.Step)) } : Reasoning.Screening.Screen.Node) :: ([] : List (Reasoning.Screening.Screen.Node)))) : List (Reasoning.Screening.Screen.Node)); ({ frontier := __first, found := Option.none, truncated := (Nat.blt (3) (1)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := (LexLeanRuntime.length (__first) : Nat) } : Reasoning.Screening.Screen.Ledger) } : Reasoning.Screening.Screen.Search))
@[expose] public def Screen.attempts (__s : (Prod (Nat) (Nat))) : Nat := (LexLeanRuntime.length (Reasoning.Screening.Propose.candidates (__s)) : Nat)
@[expose] public def Screen.searchStep (w : Nat) (__r : Reasoning.Screening.Screen.Search) : Option (Reasoning.Screening.Screen.Search) := (match (__r).found with | Option.some _ => Option.none | Option.none => (match (__r).frontier with | List.nil => Option.none | List.cons __node __rest => (match Reasoning.Screening.Screen.accept (w) ((__node).state) with | Option.some __v => Option.some (({ frontier := __rest, found := Option.some ((__v, __node)), truncated := (__r).truncated, ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := ((__r).ledger).attempts, firings := ((__r).ledger).firings, expansions := ((__r).ledger).expansions, verifications := (((__r).ledger).verifications + 1), frontier := ((__r).ledger).frontier } : Reasoning.Screening.Screen.Ledger) } : Reasoning.Screening.Screen.Search)) | Option.none => (let __successors : List (Reasoning.Screening.Screen.Node) := Reasoning.Screening.Screen.successors (__node); (let __fresh : List (Reasoning.Screening.Screen.Node) := __successors; (let __ordered : List (Reasoning.Screening.Screen.Node) := (LexLeanRuntime.append (__fresh) (__rest) : List (Reasoning.Screening.Screen.Node)); (let __next : List (Reasoning.Screening.Screen.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Reasoning.Screening.Screen.Node)) (__n : Reasoning.Screening.Screen.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) (3)) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (Reasoning.Screening.Screen.Node)))) : List (Reasoning.Screening.Screen.Node)) else __acc))) (([] : List (Reasoning.Screening.Screen.Node))) (__ordered) : List (Reasoning.Screening.Screen.Node)); Option.some (({ frontier := __next, found := Option.none, truncated := ((__r).truncated || (Nat.blt (3) ((LexLeanRuntime.length (__ordered) : Nat)))), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + Reasoning.Screening.Screen.attempts ((__node).state)), firings := (((__r).ledger).firings + (LexLeanRuntime.length (__successors) : Nat)), expansions := (((__r).ledger).expansions + 1), verifications := (((__r).ledger).verifications + 1), frontier := (if (Nat.blt (((__r).ledger).frontier) ((LexLeanRuntime.length (__next) : Nat))) then (LexLeanRuntime.length (__next) : Nat) else ((__r).ledger).frontier) } : Reasoning.Screening.Screen.Ledger) } : Reasoning.Screening.Screen.Search)))))))))
@[expose] public def Screen.run (w : Nat) : (Prod (Reasoning.Screening.Screen.Search) (Bool)) := (LexLeanCollections.iterateUntil ((fun (__r : Reasoning.Screening.Screen.Search) => Reasoning.Screening.Screen.searchStep (w) (__r))) (8) (Reasoning.Screening.Screen.start (w)) : (Prod (Reasoning.Screening.Screen.Search) (Bool)))
@[expose] public def Screen.failure (__saturated : Bool) (__truncated : Bool) : (Prod Bool Bool) := (if (__saturated && (!__truncated)) then ((false, true) : Prod Bool Bool) else ((false, false) : Prod Bool Bool))
@[expose] public def Screen (w : Nat) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (Reasoning.Screening.Screen.Step)))) := (let __final : (Prod (Reasoning.Screening.Screen.Search) (Bool)) := Reasoning.Screening.Screen.run (w); (match ((__final).1).found with | Option.some __hit => Except.ok (((__hit).1, ((__hit).2).trace)) | Option.none => Except.error (Reasoning.Screening.Screen.failure ((__final).2) (((__final).1).truncated))))
@[expose] public def Screen.verdict (w : Nat) : Except ((Prod Bool Bool)) (Nat) := (match Reasoning.Screening.Screen (w) with | Except.ok __p => Except.ok ((__p).1) | Except.error __e => Except.error (__e))
public theorem Screen.fire_sound (__s : (Prod (Nat) (Nat))) (__step : Reasoning.Screening.Screen.Step) (__t : (Prod (Nat) (Nat))) : ((Reasoning.Screening.Screen.fire (__s) (__step) = Option.some (__t)) -> Reasoning.Screening.Proposed (__s) (__t)) :=
by
  cases __step with
  | Propose __b =>
    exact Reasoning.Screening.Propose.apply_sound __s __b __t
public theorem Screen.replay_fire (__s : (Prod (Nat) (Nat))) (__step : Reasoning.Screening.Screen.Step) (__t : (Prod (Nat) (Nat))) : ((Reasoning.Screening.Screen.fire (__s) (__step) = Option.some (__t)) -> (Reasoning.Screening.Screen.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [Reasoning.Screening.Screen.replay]
  rw [llE]
public theorem Screen.replay_sound (w : Nat) (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : Reasoning.Screening.Screen.Step) : ((LexLeanReasoning.Reaches ((Reasoning.Screening.Proposed)) (Reasoning.Screening.Screen.observe (w)) (__acc)) -> (LexLeanReasoning.Reaches ((Reasoning.Screening.Proposed)) (Reasoning.Screening.Screen.observe (w)) (Reasoning.Screening.Screen.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [Reasoning.Screening.Screen.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (Reasoning.Screening.Screen.fire_sound llS __step _ ‹_›)
public theorem Screen.derivation (w : Nat) (__trace : List (Reasoning.Screening.Screen.Step)) : (LexLeanReasoning.Reaches ((Reasoning.Screening.Proposed)) (Reasoning.Screening.Screen.observe (w)) (Reasoning.Screening.Screen.follow (w) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((Reasoning.Screening.Screen.replay)) (fun (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) => (LexLeanReasoning.Reaches ((Reasoning.Screening.Proposed)) (Reasoning.Screening.Screen.observe (w)) (__acc))) (Reasoning.Screening.Screen.replay_sound (w)) (__trace) (Except.ok (Reasoning.Screening.Screen.observe (w))) (LexLeanReasoning.reachesStart ((Reasoning.Screening.Proposed)) (Reasoning.Screening.Screen.observe (w))))
public theorem Screen.accept_sound (w : Nat) (__s : (Prod (Nat) (Nat))) (__v : Nat) : ((Reasoning.Screening.Screen.accept (w) (__s) = Option.some (__v)) -> Reasoning.Screening.Safe (w) (__v)) :=
by
  intro llE
  dsimp only [Reasoning.Screening.Screen.accept] at llE
  split at llE
  · cases llE
  · have llC := LexLeanReasoning.checked _ _ _ llE
    exact llC.right ▸ Reasoning.Screening.safe_sound _ _ llC.left
public theorem Screen.extend (w : Nat) (__node : Reasoning.Screening.Screen.Node) (__step : Reasoning.Screening.Screen.Step) (__t : (Prod (Nat) (Nat))) : ((Reasoning.Screening.Screen.follow (w) ((__node).trace) = Except.ok ((__node).state)) -> ((Reasoning.Screening.Screen.fire ((__node).state) (__step) = Option.some (__t)) -> (Reasoning.Screening.Screen.follow (w) ((LexLeanRuntime.append ((__node).trace) ((__step :: ([] : List (Reasoning.Screening.Screen.Step)))) : List (Reasoning.Screening.Screen.Step))) = Except.ok (__t)))) :=
by
  intro llH llE
  dsimp only [Reasoning.Screening.Screen.follow]
  rw [LexLeanReasoning.foldSnoc]
  dsimp only [Reasoning.Screening.Screen.follow] at llH
  rw [llH]
  exact Reasoning.Screening.Screen.replay_fire _ __step __t llE
public theorem Screen.Propose.successors_ok (w : Nat) (__node : Reasoning.Screening.Screen.Node) : ((Reasoning.Screening.Screen.follow (w) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Reasoning.Screening.Screen.Node) => (Reasoning.Screening.Screen.follow (w) ((__n).trace) = Except.ok ((__n).state)))) (Reasoning.Screening.Screen.Propose.successors (__node)))) :=
by
  intro llH
  dsimp only [Reasoning.Screening.Screen.Propose.successors]
  exact LexLeanReasoning.foldInvariant _ (LexLeanReasoning.All (fun (llN : Reasoning.Screening.Screen.Node) => Reasoning.Screening.Screen.follow _ llN.trace = Except.ok llN.state))
    (fun llA llB llP => by
      dsimp only [Reasoning.Screening.Screen.Propose.collect]
      split
      · exact llP
      · exact LexLeanReasoning.allAppend _ _ _ llP (LexLeanReasoning.allSingle _ _ (Reasoning.Screening.Screen.extend _ __node _ _ llH ‹_›)))
    _ _ LexLeanReasoning.All.nil
public theorem Screen.successors_ok (w : Nat) (__node : Reasoning.Screening.Screen.Node) : ((Reasoning.Screening.Screen.follow (w) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Reasoning.Screening.Screen.Node) => (Reasoning.Screening.Screen.follow (w) ((__n).trace) = Except.ok ((__n).state)))) (Reasoning.Screening.Screen.successors (__node)))) :=
  (fun llH => (Reasoning.Screening.Screen.Propose.successors_ok (w) (__node) llH))
public theorem Screen.search_step (w : Nat) (__r : Reasoning.Screening.Screen.Search) (__q : Reasoning.Screening.Screen.Search) : ((Reasoning.Screening.Screen.searchStep (w) (__r) = Option.some (__q)) -> ((LexLeanReasoning.SearchOk ((fun (__n : Reasoning.Screening.Screen.Node) => (Reasoning.Screening.Screen.follow (w) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Reasoning.Screening.Screen.Node) => Reasoning.Screening.Screen.accept (w) ((__n).state))) ((__r).frontier) ((__r).found)) -> (LexLeanReasoning.SearchOk ((fun (__n : Reasoning.Screening.Screen.Node) => (Reasoning.Screening.Screen.follow (w) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Reasoning.Screening.Screen.Node) => Reasoning.Screening.Screen.accept (w) ((__n).state))) ((__q).frontier) ((__q).found)))) :=
by
  intro llE llH
  dsimp only [Reasoning.Screening.Screen.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · rename_i llNode llRest llFr
      have llAll := And.left llH
      rw [llFr] at llAll
      have llNodeOk := LexLeanReasoning.allHead _ llNode llRest llAll
      have llRestOk := LexLeanReasoning.allTail _ llNode llRest llAll
      split at llE
      · rename_i llV llAcc
        cases llE
        exact And.intro llRestOk (And.intro llNodeOk llAcc)
      · cases llE
        exact And.intro (LexLeanReasoning.capAll _ _ _ (LexLeanReasoning.allAppend _ _ _ (Reasoning.Screening.Screen.successors_ok _ llNode llNodeOk) llRestOk)) True.intro
public theorem Screen.search_peak (w : Nat) (__r : Reasoning.Screening.Screen.Search) (__q : Reasoning.Screening.Screen.Search) : ((Reasoning.Screening.Screen.searchStep (w) (__r) = Option.some (__q)) -> ((((__r).ledger).frontier <= 3) -> (((__q).ledger).frontier <= 3))) :=
by
  intro llE llH
  dsimp only [Reasoning.Screening.Screen.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        exact llH
      · cases llE
        exact LexLeanReasoning.peakBound _ _ _ llH (LexLeanReasoning.capBound _ _)
public theorem Screen.search_count (w : Nat) (__r : Reasoning.Screening.Screen.Search) (__q : Reasoning.Screening.Screen.Search) : ((Reasoning.Screening.Screen.searchStep (w) (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Screening.Screen.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        rfl
      · cases llE
        rfl
public theorem Screen.search_verify_growth (w : Nat) (__r : Reasoning.Screening.Screen.Search) (__q : Reasoning.Screening.Screen.Search) : ((Reasoning.Screening.Screen.searchStep (w) (__r) = Option.some (__q)) -> (((__q).ledger).verifications <= (((__r).ledger).verifications + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Screening.Screen.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem Screen.search_expanded (w : Nat) (__r : Reasoning.Screening.Screen.Search) (__q : Reasoning.Screening.Screen.Search) : ((Reasoning.Screening.Screen.searchStep (w) (__r) = Option.some (__q)) -> (((__q).ledger).expansions <= (((__r).ledger).expansions + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Screening.Screen.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem Screen.search_ok (w : Nat) : (LexLeanReasoning.SearchOk ((fun (__n : Reasoning.Screening.Screen.Node) => (Reasoning.Screening.Screen.follow (w) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Reasoning.Screening.Screen.Node) => Reasoning.Screening.Screen.accept (w) ((__n).state))) (((Reasoning.Screening.Screen.run (w)).1).frontier) (((Reasoning.Screening.Screen.run (w)).1).found)) :=
by
  dsimp only [Reasoning.Screening.Screen.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((fun (__r : Reasoning.Screening.Screen.Search) => Reasoning.Screening.Screen.searchStep (w) (__r))) (fun (__r : Reasoning.Screening.Screen.Search) => (LexLeanReasoning.SearchOk ((fun (__n : Reasoning.Screening.Screen.Node) => (Reasoning.Screening.Screen.follow (w) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Reasoning.Screening.Screen.Node) => Reasoning.Screening.Screen.accept (w) ((__n).state))) ((__r).frontier) ((__r).found))) (Reasoning.Screening.Screen.search_step (w)) (8) (Reasoning.Screening.Screen.start (w)) (LexLeanReasoning.searchStart ((fun (__n : Reasoning.Screening.Screen.Node) => (Reasoning.Screening.Screen.follow (w) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Reasoning.Screening.Screen.Node) => Reasoning.Screening.Screen.accept (w) ((__n).state))) _ (LexLeanReasoning.capAll ((fun (__n : Reasoning.Screening.Screen.Node) => (Reasoning.Screening.Screen.follow (w) ((__n).trace) = Except.ok ((__n).state)))) (3) _ (LexLeanReasoning.allSingle ((fun (__n : Reasoning.Screening.Screen.Node) => (Reasoning.Screening.Screen.follow (w) ((__n).trace) = Except.ok ((__n).state)))) (({ state := Reasoning.Screening.Screen.observe (w), trace := ([] : List (Reasoning.Screening.Screen.Step)) } : Reasoning.Screening.Screen.Node)) rfl))))
public theorem Screen.frontier_bounded (w : Nat) : ((((Reasoning.Screening.Screen.run (w)).1).ledger).frontier <= 3) :=
by
  dsimp only [Reasoning.Screening.Screen.run]
  exact (LexLeanReasoning.iterateUntilBound ((fun (__r : Reasoning.Screening.Screen.Search) => Reasoning.Screening.Screen.searchStep (w) (__r))) ((fun (__r : Reasoning.Screening.Screen.Search) => ((__r).ledger).frontier)) (3) (Reasoning.Screening.Screen.search_peak (w)) (8) (Reasoning.Screening.Screen.start (w)) (LexLeanReasoning.capBound (3) _))
public theorem Screen.verifications_bounded (w : Nat) : ((((Reasoning.Screening.Screen.run (w)).1).ledger).verifications <= 8) :=
by
  dsimp only [Reasoning.Screening.Screen.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : Reasoning.Screening.Screen.Search) => Reasoning.Screening.Screen.searchStep (w) (__r))) ((fun (__r : Reasoning.Screening.Screen.Search) => ((__r).ledger).verifications)) (Reasoning.Screening.Screen.search_verify_growth (w)) (8) (Reasoning.Screening.Screen.start (w)) rfl)
public theorem Screen.expansions_bounded (w : Nat) : ((((Reasoning.Screening.Screen.run (w)).1).ledger).expansions <= 8) :=
by
  dsimp only [Reasoning.Screening.Screen.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : Reasoning.Screening.Screen.Search) => Reasoning.Screening.Screen.searchStep (w) (__r))) ((fun (__r : Reasoning.Screening.Screen.Search) => ((__r).ledger).expansions)) (Reasoning.Screening.Screen.search_expanded (w)) (8) (Reasoning.Screening.Screen.start (w)) rfl)
public theorem Screen.iterations_bounded (w : Nat) : ((((Reasoning.Screening.Screen.run (w)).1).ledger).iterations <= 8) :=
by
  dsimp only [Reasoning.Screening.Screen.run]
  exact (LexLeanReasoning.iterateUntilCount ((fun (__r : Reasoning.Screening.Screen.Search) => Reasoning.Screening.Screen.searchStep (w) (__r))) ((fun (__r : Reasoning.Screening.Screen.Search) => ((__r).ledger).iterations)) (Reasoning.Screening.Screen.search_count (w)) (8) (Reasoning.Screening.Screen.start (w)) rfl)
public theorem Screen.explained (w : Nat) : (forall (__v : Nat), (forall (__trace : List (Reasoning.Screening.Screen.Step)), ((Reasoning.Screening.Screen (w) = Except.ok ((__v, __trace))) -> ((Reasoning.Screening.Screen.answer (w) (__trace) = Option.some (__v)) /\ Reasoning.Screening.Safe (w) (__v))))) :=
by
  intro llV llT llE
  have llSearch := Reasoning.Screening.Screen.search_ok w
  dsimp only [Reasoning.Screening.Screen] at llE
  generalize llRun : Reasoning.Screening.Screen.run w = llR at llE llSearch
  split at llE
  · rename_i llHit llF
    cases llE
    have llOk := And.right llSearch
    rw [llF] at llOk
    exact And.intro (by dsimp only [Reasoning.Screening.Screen.answer]; rw [And.left llOk]; exact And.right llOk) (Reasoning.Screening.Screen.accept_sound _ _ _ (And.right llOk))
  · cases llE
public theorem Screen.verdict_sound (w : Nat) : (forall (__v : Nat), ((Reasoning.Screening.Screen.verdict (w) = Except.ok (__v)) -> Reasoning.Screening.Safe (w) (__v))) :=
by
  intro llV llE
  dsimp only [Reasoning.Screening.Screen.verdict] at llE
  generalize llHP : Reasoning.Screening.Screen w = llR at llE
  cases llR with
  | error _ => cases llE
  | ok llP =>
    cases llE
    exact And.right (Reasoning.Screening.Screen.explained _ llP.1 llP.2 llHP)

end Reasoning.Screening
