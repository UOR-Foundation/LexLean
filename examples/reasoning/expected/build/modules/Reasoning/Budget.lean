module
public import Init
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
namespace Reasoning.Budget

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

@[expose, reducible] public def Spent (T : Type) (s : (Prod (T) (Nat))) (t : (Prod (T) (Nat))) : Prop := (((t).1 = (s).1) /\ ((t).2 < (s).2))

@[expose] public def Remaining (T : Type) (s : (Prod (T) (Nat))) : Nat := (s).2


public theorem tick_sound (T : Type) (s : (Prod (T) (Nat))) : (((Nat.blt (2) ((s).2)) = true) -> Spent (T) (s) (((s).1, (LexLeanRuntime.subtract ((s).2) (1) : Nat)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Spent, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem tick_progress (T : Type) (s : (Prod (T) (Nat))) : (((Nat.blt (2) ((s).2)) = true) -> (Remaining (T) (((s).1, (LexLeanRuntime.subtract ((s).2) (1) : Nat))) < Remaining (T) (s))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Remaining, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose] public def Tick.guard (T : Type) (s : (Prod (T) (Nat))) : Bool := (Nat.blt (2) ((s).2))
@[expose] public def Tick.conclusion (T : Type) (s : (Prod (T) (Nat))) : (Prod (T) (Nat)) := ((s).1, (LexLeanRuntime.subtract ((s).2) (1) : Nat))
@[expose] public def Tick.apply (T : Type) (s : (Prod (T) (Nat))) : Option ((Prod (T) (Nat))) := (if Reasoning.Budget.Tick.guard (T) (s) then Option.some (Reasoning.Budget.Tick.conclusion (T) (s)) else Option.none)
public theorem Tick.apply_sound (T : Type) (s : (Prod (T) (Nat))) (__t : (Prod (T) (Nat))) : ((Reasoning.Budget.Tick.apply (T) (s) = Option.some (__t)) -> Reasoning.Budget.Spent (T) (s) (__t)) :=
  (LexLeanReasoning.guarded ((Reasoning.Budget.Spent (T))) (s) (Reasoning.Budget.Tick.guard (T) (s)) (Reasoning.Budget.Tick.conclusion (T) (s)) ((Reasoning.Budget.tick_sound (T)) (s)) (__t))
public theorem Tick.apply_progress (T : Type) (s : (Prod (T) (Nat))) (__t : (Prod (T) (Nat))) : ((Reasoning.Budget.Tick.apply (T) (s) = Option.some (__t)) -> (Reasoning.Budget.Remaining (T) (__t) < Reasoning.Budget.Remaining (T) (s))) :=
  (LexLeanReasoning.guardedRank ((Reasoning.Budget.Remaining (T))) (s) (Reasoning.Budget.Tick.guard (T) (s)) (Reasoning.Budget.Tick.conclusion (T) (s)) ((Reasoning.Budget.tick_progress (T)) (s)) (__t))

public theorem burn_sound (T : Type) (s : (Prod (T) (Nat))) (k : Nat) : ((((Nat.ble (k) ((s).2)) && (Nat.blt (0) (k))) = true) -> Spent (T) (s) (((s).1, (LexLeanRuntime.subtract ((s).2) (k) : Nat)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Spent, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem burn_progress (T : Type) (s : (Prod (T) (Nat))) (k : Nat) : ((((Nat.ble (k) ((s).2)) && (Nat.blt (0) (k))) = true) -> (Remaining (T) (((s).1, (LexLeanRuntime.subtract ((s).2) (k) : Nat))) < Remaining (T) (s))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Remaining, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose] public def Burn.guard (T : Type) (s : (Prod (T) (Nat))) (k : Nat) : Bool := ((Nat.ble (k) ((s).2)) && (Nat.blt (0) (k)))
@[expose] public def Burn.conclusion (T : Type) (s : (Prod (T) (Nat))) (k : Nat) : (Prod (T) (Nat)) := ((s).1, (LexLeanRuntime.subtract ((s).2) (k) : Nat))
@[expose] public def Burn.candidates (T : Type) (_s : (Prod (T) (Nat))) : List (Nat) := (2 :: (1 :: ([] : List (Nat))))
@[expose] public def Burn.apply (T : Type) (s : (Prod (T) (Nat))) (k : Nat) : Option ((Prod (T) (Nat))) := (if Reasoning.Budget.Burn.guard (T) (s) (k) then Option.some (Reasoning.Budget.Burn.conclusion (T) (s) (k)) else Option.none)
public theorem Burn.apply_sound (T : Type) (s : (Prod (T) (Nat))) (k : Nat) (__t : (Prod (T) (Nat))) : ((Reasoning.Budget.Burn.apply (T) (s) (k) = Option.some (__t)) -> Reasoning.Budget.Spent (T) (s) (__t)) :=
  (LexLeanReasoning.guarded ((Reasoning.Budget.Spent (T))) (s) (Reasoning.Budget.Burn.guard (T) (s) (k)) (Reasoning.Budget.Burn.conclusion (T) (s) (k)) ((Reasoning.Budget.burn_sound (T)) (s) (k)) (__t))
public theorem Burn.apply_progress (T : Type) (s : (Prod (T) (Nat))) (k : Nat) (__t : (Prod (T) (Nat))) : ((Reasoning.Budget.Burn.apply (T) (s) (k) = Option.some (__t)) -> (Reasoning.Budget.Remaining (T) (__t) < Reasoning.Budget.Remaining (T) (s))) :=
  (LexLeanReasoning.guardedRank ((Reasoning.Budget.Remaining (T))) (s) (Reasoning.Budget.Burn.guard (T) (s) (k)) (Reasoning.Budget.Burn.conclusion (T) (s) (k)) ((Reasoning.Budget.burn_progress (T)) (s) (k)) (__t))

@[expose, reducible] public def Empty (T : Type) (_x : T) (r : Nat) : Prop := (r = 0)

@[expose] public def emptyCheck (T : Type) (_x : T) (r : Nat) : Bool := (Nat.beq (r) (0))

public theorem empty_sound (T : Type) (x : T) (r : Nat) : ((emptyCheck (T) (x) (r) = true) -> Empty (T) (x) (r)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Empty, emptyCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem Drained.sound (T : Type) : (LexLeanReasoning.Sound ((Reasoning.Budget.emptyCheck (T))) ((Reasoning.Budget.Empty (T)))) :=
  (Reasoning.Budget.empty_sound (T))

public theorem allowance_bound (T : Type) (x : (Prod (T) (Nat))) : (Remaining (T) (((x).1, ((x).2 + 3))) < ((x).2 + 4)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Remaining, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public inductive Spend.Step (T : Type) where
  | Tick
  | Burn (_ : Nat)
public structure Spend.Ledger (T : Type) where
  iterations : Nat
  attempts : Nat
  firings : Nat
  expansions : Nat
  verifications : Nat
  frontier : Nat
@[expose] public def Spend.observe (T : Type) (x : (Prod (T) (Nat))) : (Prod (T) (Nat)) := ((x).1, ((x).2 + 3))
@[expose] public def Spend.fire (T : Type) (__s : (Prod (T) (Nat))) (__step : Reasoning.Budget.Spend.Step (T)) : Option ((Prod (T) (Nat))) := (match __step with | Reasoning.Budget.Spend.Step.Tick => Reasoning.Budget.Tick.apply (T) (__s) | Reasoning.Budget.Spend.Step.Burn __b => Reasoning.Budget.Burn.apply (T) (__s) (__b))
@[expose] public def Spend.replay (T : Type) (__acc : Except ((Prod Bool Bool)) ((Prod (T) (Nat)))) (__step : Reasoning.Budget.Spend.Step (T)) : Except ((Prod Bool Bool)) ((Prod (T) (Nat))) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match Reasoning.Budget.Spend.fire (T) (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def Spend.follow (T : Type) (x : (Prod (T) (Nat))) (__trace : List (Reasoning.Budget.Spend.Step (T))) : Except ((Prod Bool Bool)) ((Prod (T) (Nat))) := (LexLeanCollections.listFold ((Reasoning.Budget.Spend.replay (T))) (Except.ok (Reasoning.Budget.Spend.observe (T) (x))) (__trace) : Except ((Prod Bool Bool)) ((Prod (T) (Nat))))
@[expose] public def Spend.extract (T : Type) (account : (Prod (T) (Nat))) : Option (Nat) := Option.some ((account).2)
@[expose] public def Spend.accept (T : Type) (x : (Prod (T) (Nat))) (__s : (Prod (T) (Nat))) : Option (Nat) := (match Reasoning.Budget.Spend.extract (T) (__s) with | Option.none => Option.none | Option.some __v => (if Reasoning.Budget.emptyCheck ((Prod (T) (Nat))) (x) (__v) then Option.some (__v) else Option.none))
@[expose] public def Spend.answer (T : Type) (x : (Prod (T) (Nat))) (__trace : List (Reasoning.Budget.Spend.Step (T))) : Option (Nat) := (match Reasoning.Budget.Spend.follow (T) (x) (__trace) with | Except.ok __s => Reasoning.Budget.Spend.accept (T) (x) (__s) | Except.error _ => Option.none)
@[expose] public def Spend.select (T : Type) (__s : (Prod (T) (Nat))) : Option (Reasoning.Budget.Spend.Step (T)) := (match (if Reasoning.Budget.Tick.guard (T) (__s) then Option.some (Reasoning.Budget.Spend.Step.Tick) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (LexLeanCollections.listFold ((fun (__acc : Option (Reasoning.Budget.Spend.Step (T))) (__b : Nat) => (match __acc with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Budget.Burn.guard (T) (__s) (__b) then Option.some (Reasoning.Budget.Spend.Step.Burn (__b)) else Option.none)))) (Option.none) (Reasoning.Budget.Burn.candidates (T) (__s)) : Option (Reasoning.Budget.Spend.Step (T))))
@[expose] public def Spend.attempts (T : Type) (__s : (Prod (T) (Nat))) : Nat := ((let __scan0l : (Prod (Bool) (Nat)) := (Reasoning.Budget.Tick.guard (T) (__s), 1); (if (__scan0l).1 then __scan0l else (let __scan0r : (Prod (Bool) (Nat)) := (LexLeanCollections.listFold ((fun (__acc : (Prod (Bool) (Nat))) (__b : Nat) => (if (__acc).1 then __acc else (Reasoning.Budget.Burn.guard (T) (__s) (__b), ((__acc).2 + 1))))) ((false, 0)) (Reasoning.Budget.Burn.candidates (T) (__s)) : (Prod (Bool) (Nat))); ((__scan0r).1, ((__scan0l).2 + (__scan0r).2)))))).2
@[expose] public def Spend.next (T : Type) (__s : (Prod (T) (Nat))) : Option ((Prod (T) (Nat))) := (match Reasoning.Budget.Spend.select (T) (__s) with | Option.none => Option.none | Option.some __step => Reasoning.Budget.Spend.fire (T) (__s) (__step))
@[expose] public def Spend.saturate (T : Type) (x : (Prod (T) (Nat))) : (Prod ((Prod (T) (Nat))) (Bool)) := (LexLeanCollections.iterateUntil ((Reasoning.Budget.Spend.next (T))) (((x).2 + 4)) (Reasoning.Budget.Spend.observe (T) (x)) : (Prod ((Prod (T) (Nat))) (Bool)))
public structure Spend.Run (T : Type) where
  state : (Prod (T) (Nat))
  trace : List (Reasoning.Budget.Spend.Step (T))
  ledger : Reasoning.Budget.Spend.Ledger (T)
@[expose] public def Spend.start (T : Type) (x : (Prod (T) (Nat))) : Reasoning.Budget.Spend.Run (T) := ({ state := Reasoning.Budget.Spend.observe (T) (x), trace := ([] : List (Reasoning.Budget.Spend.Step (T))), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := 0 } : Reasoning.Budget.Spend.Ledger (T)) } : Reasoning.Budget.Spend.Run (T))
@[expose] public def Spend.step (T : Type) (__r : Reasoning.Budget.Spend.Run (T)) : Option (Reasoning.Budget.Spend.Run (T)) := (match Reasoning.Budget.Spend.select (T) ((__r).state) with | Option.none => Option.none | Option.some __step => (match Reasoning.Budget.Spend.fire (T) ((__r).state) (__step) with | Option.none => Option.none | Option.some __t => Option.some (({ state := __t, trace := (LexLeanRuntime.append ((__r).trace) ((__step :: ([] : List (Reasoning.Budget.Spend.Step (T))))) : List (Reasoning.Budget.Spend.Step (T))), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + Reasoning.Budget.Spend.attempts (T) ((__r).state)), firings := (((__r).ledger).firings + 1), expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : Reasoning.Budget.Spend.Ledger (T)) } : Reasoning.Budget.Spend.Run (T)))))
@[expose] public def Spend.run (T : Type) (x : (Prod (T) (Nat))) : (Prod (Reasoning.Budget.Spend.Run (T)) (Bool)) := (LexLeanCollections.iterateUntil ((Reasoning.Budget.Spend.step (T))) (((x).2 + 4)) (Reasoning.Budget.Spend.start (T) (x)) : (Prod (Reasoning.Budget.Spend.Run (T)) (Bool)))
@[expose] public def Spend.account (T : Type) (x : (Prod (T) (Nat))) : Reasoning.Budget.Spend.Ledger (T) := (let __final : (Prod (Reasoning.Budget.Spend.Run (T)) (Bool)) := Reasoning.Budget.Spend.run (T) (x); (match (__final).2 with | Bool.true => ({ iterations := (((__final).1).ledger).iterations, attempts := ((((__final).1).ledger).attempts + Reasoning.Budget.Spend.attempts (T) (((__final).1).state)), firings := (((__final).1).ledger).firings, expansions := (((__final).1).ledger).expansions, verifications := ((((__final).1).ledger).verifications + 1), frontier := (((__final).1).ledger).frontier } : Reasoning.Budget.Spend.Ledger (T)) | Bool.false => ((__final).1).ledger))
@[expose] public def Spend.conclude (T : Type) (x : (Prod (T) (Nat))) (__s : (Prod (T) (Nat))) : Except ((Prod Bool Bool)) (Nat) := (match Reasoning.Budget.Spend.accept (T) (x) (__s) with | Option.some __v => Except.ok (__v) | Option.none => (match Reasoning.Budget.Spend.extract (T) (__s) with | Option.some _ => Except.error (((true, false) : Prod Bool Bool)) | Option.none => Except.error (((false, true) : Prod Bool Bool))))
@[expose] public def Spend.verdict (T : Type) (x : (Prod (T) (Nat))) : Except ((Prod Bool Bool)) (Nat) := (let __final : (Prod ((Prod (T) (Nat))) (Bool)) := Reasoning.Budget.Spend.saturate (T) (x); (match (__final).2 with | Bool.true => Reasoning.Budget.Spend.conclude (T) (x) ((__final).1) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
@[expose] public def Spend (T : Type) (x : (Prod (T) (Nat))) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (Reasoning.Budget.Spend.Step (T))))) := (let __final : (Prod (Reasoning.Budget.Spend.Run (T)) (Bool)) := Reasoning.Budget.Spend.run (T) (x); (match (__final).2 with | Bool.true => (match Reasoning.Budget.Spend.conclude (T) (x) (((__final).1).state) with | Except.ok __v => Except.ok ((__v, ((__final).1).trace)) | Except.error __e => Except.error (__e)) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
public theorem Spend.fire_sound (T : Type) (__s : (Prod (T) (Nat))) (__step : Reasoning.Budget.Spend.Step (T)) (__t : (Prod (T) (Nat))) : ((Reasoning.Budget.Spend.fire (T) (__s) (__step) = Option.some (__t)) -> Reasoning.Budget.Spent (T) (__s) (__t)) :=
by
  cases __step with
  | Tick =>
    exact Reasoning.Budget.Tick.apply_sound (T) __s __t
  | Burn __b =>
    exact Reasoning.Budget.Burn.apply_sound (T) __s __b __t
public theorem Spend.fire_progress (T : Type) (__s : (Prod (T) (Nat))) (__step : Reasoning.Budget.Spend.Step (T)) (__t : (Prod (T) (Nat))) : ((Reasoning.Budget.Spend.fire (T) (__s) (__step) = Option.some (__t)) -> (Reasoning.Budget.Remaining (T) (__t) < Reasoning.Budget.Remaining (T) (__s))) :=
by
  cases __step with
  | Tick =>
    exact Reasoning.Budget.Tick.apply_progress (T) __s __t
  | Burn __b =>
    exact Reasoning.Budget.Burn.apply_progress (T) __s __b __t
public theorem Spend.replay_fire (T : Type) (__s : (Prod (T) (Nat))) (__step : Reasoning.Budget.Spend.Step (T)) (__t : (Prod (T) (Nat))) : ((Reasoning.Budget.Spend.fire (T) (__s) (__step) = Option.some (__t)) -> (Reasoning.Budget.Spend.replay (T) (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Spend.replay]
  rw [llE]
public theorem Spend.replay_sound (T : Type) (x : (Prod (T) (Nat))) (__acc : Except ((Prod Bool Bool)) ((Prod (T) (Nat)))) (__step : Reasoning.Budget.Spend.Step (T)) : ((LexLeanReasoning.Reaches ((Reasoning.Budget.Spent (T))) (Reasoning.Budget.Spend.observe (T) (x)) (__acc)) -> (LexLeanReasoning.Reaches ((Reasoning.Budget.Spent (T))) (Reasoning.Budget.Spend.observe (T) (x)) (Reasoning.Budget.Spend.replay (T) (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [Reasoning.Budget.Spend.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) ((Reasoning.Budget.Spend.fire_sound (T)) llS __step _ ‹_›)
public theorem Spend.derivation (T : Type) (x : (Prod (T) (Nat))) (__trace : List (Reasoning.Budget.Spend.Step (T))) : (LexLeanReasoning.Reaches ((Reasoning.Budget.Spent (T))) (Reasoning.Budget.Spend.observe (T) (x)) (Reasoning.Budget.Spend.follow (T) (x) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((Reasoning.Budget.Spend.replay (T))) (fun (__acc : Except ((Prod Bool Bool)) ((Prod (T) (Nat)))) => (LexLeanReasoning.Reaches ((Reasoning.Budget.Spent (T))) (Reasoning.Budget.Spend.observe (T) (x)) (__acc))) ((Reasoning.Budget.Spend.replay_sound (T)) (x)) (__trace) (Except.ok (Reasoning.Budget.Spend.observe (T) (x))) (LexLeanReasoning.reachesStart ((Reasoning.Budget.Spent (T))) (Reasoning.Budget.Spend.observe (T) (x))))
public theorem Spend.accept_sound (T : Type) (x : (Prod (T) (Nat))) (__s : (Prod (T) (Nat))) (__v : Nat) : ((Reasoning.Budget.Spend.accept (T) (x) (__s) = Option.some (__v)) -> Reasoning.Budget.Empty ((Prod (T) (Nat))) (x) (__v)) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Spend.accept] at llE
  split at llE
  · cases llE
  · have llC := LexLeanReasoning.checked _ _ _ llE
    exact llC.right ▸ Reasoning.Budget.empty_sound ((Prod (T) (Nat))) _ _ llC.left
public theorem Spend.next_sound (T : Type) (__s : (Prod (T) (Nat))) (__t : (Prod (T) (Nat))) : ((Reasoning.Budget.Spend.next (T) (__s) = Option.some (__t)) -> Reasoning.Budget.Spent (T) (__s) (__t)) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Spend.next] at llE
  split at llE
  · cases llE
  · exact (Reasoning.Budget.Spend.fire_sound (T)) __s _ __t llE
public theorem Spend.next_progress (T : Type) (__s : (Prod (T) (Nat))) (__t : (Prod (T) (Nat))) : ((Reasoning.Budget.Spend.next (T) (__s) = Option.some (__t)) -> (Reasoning.Budget.Remaining (T) (__t) < Reasoning.Budget.Remaining (T) (__s))) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Spend.next] at llE
  split at llE
  · cases llE
  · exact (Reasoning.Budget.Spend.fire_progress (T)) __s _ __t llE
public theorem Spend.step_none (T : Type) (__r : Reasoning.Budget.Spend.Run (T)) : ((Reasoning.Budget.Spend.step (T) (__r) = Option.none) -> (Reasoning.Budget.Spend.next (T) ((__r).state) = Option.none)) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Spend.step] at llE
  split at llE
  · rename_i llH
    dsimp only [Reasoning.Budget.Spend.next]
    rw [llH]
  · rename_i llH
    split at llE
    · dsimp only [Reasoning.Budget.Spend.next]
      rw [llH]
      assumption
    · cases llE
public theorem Spend.step_some (T : Type) (__r : Reasoning.Budget.Spend.Run (T)) (__q : Reasoning.Budget.Spend.Run (T)) : ((Reasoning.Budget.Spend.step (T) (__r) = Option.some (__q)) -> (Reasoning.Budget.Spend.next (T) ((__r).state) = Option.some ((__q).state))) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Spend.step] at llE
  split at llE
  · cases llE
  · rename_i llH
    split at llE
    · cases llE
    · cases llE
      dsimp only [Reasoning.Budget.Spend.next]
      rw [llH]
      assumption
public theorem Spend.step_trace (T : Type) (x : (Prod (T) (Nat))) (__r : Reasoning.Budget.Spend.Run (T)) (__q : Reasoning.Budget.Spend.Run (T)) : ((Reasoning.Budget.Spend.step (T) (__r) = Option.some (__q)) -> ((Reasoning.Budget.Spend.follow (T) (x) ((__r).trace) = Except.ok ((__r).state)) -> (Reasoning.Budget.Spend.follow (T) (x) ((__q).trace) = Except.ok ((__q).state)))) :=
by
  intro llE llH
  dsimp only [Reasoning.Budget.Spend.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      dsimp only [Reasoning.Budget.Spend.follow]
      rw [LexLeanReasoning.foldSnoc]
      dsimp only [Reasoning.Budget.Spend.follow] at llH
      rw [llH]
      apply (Reasoning.Budget.Spend.replay_fire (T))
      assumption
public theorem Spend.step_count (T : Type) (__r : Reasoning.Budget.Spend.Run (T)) (__q : Reasoning.Budget.Spend.Run (T)) : ((Reasoning.Budget.Spend.step (T) (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Spend.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Spend.step_fired (T : Type) (__r : Reasoning.Budget.Spend.Run (T)) (__q : Reasoning.Budget.Spend.Run (T)) : ((Reasoning.Budget.Spend.step (T) (__r) = Option.some (__q)) -> (((__q).ledger).firings = (((__r).ledger).firings + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Spend.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Spend.saturate_derivation (T : Type) (x : (Prod (T) (Nat))) : (LexLeanReasoning.Star ((Reasoning.Budget.Spent (T))) (Reasoning.Budget.Spend.observe (T) (x)) ((Reasoning.Budget.Spend.saturate (T) (x)).1)) :=
by
  dsimp only [Reasoning.Budget.Spend.saturate]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Budget.Spend.next (T))) (fun (__s : (Prod (T) (Nat))) => (LexLeanReasoning.Star ((Reasoning.Budget.Spent (T))) (Reasoning.Budget.Spend.observe (T) (x)) (__s))) (fun llA llB llE llH => (LexLeanReasoning.Star.tail _ llA llB llH ((Reasoning.Budget.Spend.next_sound (T)) llA llB llE))) (((x).2 + 4)) (Reasoning.Budget.Spend.observe (T) (x)) (LexLeanReasoning.Star.refl _))
public theorem Spend.run_state (T : Type) (x : (Prod (T) (Nat))) : ((((Reasoning.Budget.Spend.run (T) (x)).1).state = (Reasoning.Budget.Spend.saturate (T) (x)).1) /\ ((Reasoning.Budget.Spend.run (T) (x)).2 = (Reasoning.Budget.Spend.saturate (T) (x)).2)) :=
by
  dsimp only [Reasoning.Budget.Spend.run, Reasoning.Budget.Spend.saturate]
  exact (LexLeanReasoning.iterateUntilSimulate ((Reasoning.Budget.Spend.step (T))) ((Reasoning.Budget.Spend.next (T))) ((fun (__r : Reasoning.Budget.Spend.Run (T)) => (__r).state)) (Reasoning.Budget.Spend.step_none (T)) (Reasoning.Budget.Spend.step_some (T)) (((x).2 + 4)) (Reasoning.Budget.Spend.start (T) (x)))
public theorem Spend.run_trace (T : Type) (x : (Prod (T) (Nat))) : (Reasoning.Budget.Spend.follow (T) (x) (((Reasoning.Budget.Spend.run (T) (x)).1).trace) = Except.ok (((Reasoning.Budget.Spend.run (T) (x)).1).state)) :=
by
  dsimp only [Reasoning.Budget.Spend.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Budget.Spend.step (T))) (fun (__r : Reasoning.Budget.Spend.Run (T)) => (Reasoning.Budget.Spend.follow (T) (x) ((__r).trace) = Except.ok ((__r).state))) ((Reasoning.Budget.Spend.step_trace (T)) (x)) (((x).2 + 4)) (Reasoning.Budget.Spend.start (T) (x)) rfl)
public theorem Spend.iterations_bounded (T : Type) (x : (Prod (T) (Nat))) : ((((Reasoning.Budget.Spend.run (T) (x)).1).ledger).iterations <= ((x).2 + 4)) :=
by
  dsimp only [Reasoning.Budget.Spend.run]
  exact (LexLeanReasoning.iterateUntilCount ((Reasoning.Budget.Spend.step (T))) ((fun (__r : Reasoning.Budget.Spend.Run (T)) => ((__r).ledger).iterations)) (Reasoning.Budget.Spend.step_count (T)) (((x).2 + 4)) (Reasoning.Budget.Spend.start (T) (x)) rfl)
public theorem Spend.firings_bounded (T : Type) (x : (Prod (T) (Nat))) : ((((Reasoning.Budget.Spend.run (T) (x)).1).ledger).firings <= ((x).2 + 4)) :=
by
  dsimp only [Reasoning.Budget.Spend.run]
  exact (LexLeanReasoning.iterateUntilCount ((Reasoning.Budget.Spend.step (T))) ((fun (__r : Reasoning.Budget.Spend.Run (T)) => ((__r).ledger).firings)) (Reasoning.Budget.Spend.step_fired (T)) (((x).2 + 4)) (Reasoning.Budget.Spend.start (T) (x)) rfl)
public theorem Spend.saturates (T : Type) (x : (Prod (T) (Nat))) : ((Reasoning.Budget.Spend.saturate (T) (x)).2 = true) :=
by
  dsimp only [Reasoning.Budget.Spend.saturate]
  exact (LexLeanReasoning.iterateUntilStops ((Reasoning.Budget.Spend.next (T))) (fun (_ : (Prod (T) (Nat))) => True) ((Reasoning.Budget.Remaining (T))) (fun llA llB llE _llH => (And.intro True.intro ((Reasoning.Budget.Spend.next_progress (T)) llA llB llE))) (((x).2 + 4)) (Reasoning.Budget.Spend.observe (T) (x)) True.intro ((Reasoning.Budget.allowance_bound (T)) (x)))
public theorem Spend.conclude_accept (T : Type) (x : (Prod (T) (Nat))) (__s : (Prod (T) (Nat))) (__v : Nat) : ((Reasoning.Budget.Spend.conclude (T) (x) (__s) = Except.ok (__v)) -> (Reasoning.Budget.Spend.accept (T) (x) (__s) = Option.some (__v))) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Spend.conclude] at llE
  split at llE
  · cases llE
    assumption
  · split at llE
    · cases llE
    · cases llE
public theorem Spend.conclude_sound (T : Type) (x : (Prod (T) (Nat))) (__s : (Prod (T) (Nat))) (__v : Nat) : ((Reasoning.Budget.Spend.conclude (T) (x) (__s) = Except.ok (__v)) -> Reasoning.Budget.Empty ((Prod (T) (Nat))) (x) (__v)) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Spend.conclude] at llE
  split at llE
  · cases llE
    exact (Reasoning.Budget.Spend.accept_sound (T)) _ _ _ ‹_›
  · split at llE
    · cases llE
    · cases llE
public theorem Spend.verdict_sound (T : Type) (x : (Prod (T) (Nat))) : (forall (__v : Nat), ((Reasoning.Budget.Spend.verdict (T) (x) = Except.ok (__v)) -> Reasoning.Budget.Empty ((Prod (T) (Nat))) (x) (__v))) :=
by
  intro llV llE
  dsimp only [Reasoning.Budget.Spend.verdict] at llE
  generalize llRun : (Reasoning.Budget.Spend.saturate (T)) x = llR at llE
  split at llE
  · exact (Reasoning.Budget.Spend.conclude_sound (T)) _ _ _ llE
  · cases llE
public theorem Spend.explained (T : Type) (x : (Prod (T) (Nat))) : (forall (__v : Nat), (forall (__trace : List (Reasoning.Budget.Spend.Step (T))), ((Reasoning.Budget.Spend (T) (x) = Except.ok ((__v, __trace))) -> ((Reasoning.Budget.Spend.answer (T) (x) (__trace) = Option.some (__v)) /\ Reasoning.Budget.Empty ((Prod (T) (Nat))) (x) (__v))))) :=
by
  intro llV llT llE
  have llTrace := (Reasoning.Budget.Spend.run_trace (T)) x
  dsimp only [Reasoning.Budget.Spend] at llE
  generalize llRun : (Reasoning.Budget.Spend.run (T)) x = llR at llE llTrace
  split at llE
  · split at llE
    · rename_i llW llH
      cases llE
      exact And.intro (by dsimp only [Reasoning.Budget.Spend.answer]; rw [llTrace]; exact (Reasoning.Budget.Spend.conclude_accept (T)) _ _ _ llH) ((Reasoning.Budget.Spend.conclude_sound (T)) _ _ _ llH)
    · cases llE
  · cases llE

public inductive Refund.Step where
  | Tick
  | Burn (_ : Nat)
public structure Refund.Ledger where
  iterations : Nat
  attempts : Nat
  firings : Nat
  expansions : Nat
  verifications : Nat
  frontier : Nat
@[expose] public def Refund.observe (x : Nat) : (Prod (Nat) (Nat)) := (x, (x + 3))
@[expose] public def Refund.fire (__s : (Prod (Nat) (Nat))) (__step : Reasoning.Budget.Refund.Step) : Option ((Prod (Nat) (Nat))) := (match __step with | Reasoning.Budget.Refund.Step.Tick => Reasoning.Budget.Tick.apply (Nat) (__s) | Reasoning.Budget.Refund.Step.Burn __b => Reasoning.Budget.Burn.apply (Nat) (__s) (__b))
@[expose] public def Refund.replay (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : Reasoning.Budget.Refund.Step) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match Reasoning.Budget.Refund.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def Refund.follow (x : Nat) (__trace : List (Reasoning.Budget.Refund.Step)) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (LexLeanCollections.listFold ((Reasoning.Budget.Refund.replay)) (Except.ok (Reasoning.Budget.Refund.observe (x))) (__trace) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))))
@[expose] public def Refund.extract (account : (Prod (Nat) (Nat))) : Option (Nat) := (if (Nat.blt ((account).1) (5)) then Option.some ((account).2) else Option.none)
@[expose] public def Refund.accept (x : Nat) (__s : (Prod (Nat) (Nat))) : Option (Nat) := (match Reasoning.Budget.Refund.extract (__s) with | Option.none => Option.none | Option.some __v => (if Reasoning.Budget.emptyCheck (Nat) (x) (__v) then Option.some (__v) else Option.none))
@[expose] public def Refund.answer (x : Nat) (__trace : List (Reasoning.Budget.Refund.Step)) : Option (Nat) := (match Reasoning.Budget.Refund.follow (x) (__trace) with | Except.ok __s => Reasoning.Budget.Refund.accept (x) (__s) | Except.error _ => Option.none)
@[expose] public def Refund.select (__s : (Prod (Nat) (Nat))) : Option (Reasoning.Budget.Refund.Step) := (match (if Reasoning.Budget.Tick.guard (Nat) (__s) then Option.some (Reasoning.Budget.Refund.Step.Tick) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (LexLeanCollections.listFold ((fun (__acc : Option (Reasoning.Budget.Refund.Step)) (__b : Nat) => (match __acc with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Budget.Burn.guard (Nat) (__s) (__b) then Option.some (Reasoning.Budget.Refund.Step.Burn (__b)) else Option.none)))) (Option.none) (Reasoning.Budget.Burn.candidates (Nat) (__s)) : Option (Reasoning.Budget.Refund.Step)))
@[expose] public def Refund.attempts (__s : (Prod (Nat) (Nat))) : Nat := ((let __scan0l : (Prod (Bool) (Nat)) := (Reasoning.Budget.Tick.guard (Nat) (__s), 1); (if (__scan0l).1 then __scan0l else (let __scan0r : (Prod (Bool) (Nat)) := (LexLeanCollections.listFold ((fun (__acc : (Prod (Bool) (Nat))) (__b : Nat) => (if (__acc).1 then __acc else (Reasoning.Budget.Burn.guard (Nat) (__s) (__b), ((__acc).2 + 1))))) ((false, 0)) (Reasoning.Budget.Burn.candidates (Nat) (__s)) : (Prod (Bool) (Nat))); ((__scan0r).1, ((__scan0l).2 + (__scan0r).2)))))).2
@[expose] public def Refund.next (__s : (Prod (Nat) (Nat))) : Option ((Prod (Nat) (Nat))) := (match Reasoning.Budget.Refund.select (__s) with | Option.none => Option.none | Option.some __step => Reasoning.Budget.Refund.fire (__s) (__step))
@[expose] public def Refund.saturate (x : Nat) : (Prod ((Prod (Nat) (Nat))) (Bool)) := (LexLeanCollections.iterateUntil ((Reasoning.Budget.Refund.next)) ((x + 4)) (Reasoning.Budget.Refund.observe (x)) : (Prod ((Prod (Nat) (Nat))) (Bool)))
public structure Refund.Run where
  state : (Prod (Nat) (Nat))
  trace : List (Reasoning.Budget.Refund.Step)
  ledger : Reasoning.Budget.Refund.Ledger
@[expose] public def Refund.start (x : Nat) : Reasoning.Budget.Refund.Run := ({ state := Reasoning.Budget.Refund.observe (x), trace := ([] : List (Reasoning.Budget.Refund.Step)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := 0 } : Reasoning.Budget.Refund.Ledger) } : Reasoning.Budget.Refund.Run)
@[expose] public def Refund.step (__r : Reasoning.Budget.Refund.Run) : Option (Reasoning.Budget.Refund.Run) := (match Reasoning.Budget.Refund.select ((__r).state) with | Option.none => Option.none | Option.some __step => (match Reasoning.Budget.Refund.fire ((__r).state) (__step) with | Option.none => Option.none | Option.some __t => Option.some (({ state := __t, trace := (LexLeanRuntime.append ((__r).trace) ((__step :: ([] : List (Reasoning.Budget.Refund.Step)))) : List (Reasoning.Budget.Refund.Step)), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + Reasoning.Budget.Refund.attempts ((__r).state)), firings := (((__r).ledger).firings + 1), expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : Reasoning.Budget.Refund.Ledger) } : Reasoning.Budget.Refund.Run))))
@[expose] public def Refund.run (x : Nat) : (Prod (Reasoning.Budget.Refund.Run) (Bool)) := (LexLeanCollections.iterateUntil ((Reasoning.Budget.Refund.step)) ((x + 4)) (Reasoning.Budget.Refund.start (x)) : (Prod (Reasoning.Budget.Refund.Run) (Bool)))
@[expose] public def Refund.account (x : Nat) : Reasoning.Budget.Refund.Ledger := (let __final : (Prod (Reasoning.Budget.Refund.Run) (Bool)) := Reasoning.Budget.Refund.run (x); (match (__final).2 with | Bool.true => ({ iterations := (((__final).1).ledger).iterations, attempts := ((((__final).1).ledger).attempts + Reasoning.Budget.Refund.attempts (((__final).1).state)), firings := (((__final).1).ledger).firings, expansions := (((__final).1).ledger).expansions, verifications := ((((__final).1).ledger).verifications + 1), frontier := (((__final).1).ledger).frontier } : Reasoning.Budget.Refund.Ledger) | Bool.false => ((__final).1).ledger))
@[expose] public def Refund.conclude (x : Nat) (__s : (Prod (Nat) (Nat))) : Except ((Prod Bool Bool)) (Nat) := (match Reasoning.Budget.Refund.accept (x) (__s) with | Option.some __v => Except.ok (__v) | Option.none => (match Reasoning.Budget.Refund.extract (__s) with | Option.some _ => Except.error (((true, false) : Prod Bool Bool)) | Option.none => Except.error (((false, true) : Prod Bool Bool))))
@[expose] public def Refund.verdict (x : Nat) : Except ((Prod Bool Bool)) (Nat) := (let __final : (Prod ((Prod (Nat) (Nat))) (Bool)) := Reasoning.Budget.Refund.saturate (x); (match (__final).2 with | Bool.true => Reasoning.Budget.Refund.conclude (x) ((__final).1) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
@[expose] public def Refund (x : Nat) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (Reasoning.Budget.Refund.Step)))) := (let __final : (Prod (Reasoning.Budget.Refund.Run) (Bool)) := Reasoning.Budget.Refund.run (x); (match (__final).2 with | Bool.true => (match Reasoning.Budget.Refund.conclude (x) (((__final).1).state) with | Except.ok __v => Except.ok ((__v, ((__final).1).trace)) | Except.error __e => Except.error (__e)) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
public theorem Refund.fire_sound (__s : (Prod (Nat) (Nat))) (__step : Reasoning.Budget.Refund.Step) (__t : (Prod (Nat) (Nat))) : ((Reasoning.Budget.Refund.fire (__s) (__step) = Option.some (__t)) -> Reasoning.Budget.Spent (Nat) (__s) (__t)) :=
by
  cases __step with
  | Tick =>
    exact Reasoning.Budget.Tick.apply_sound (Nat) __s __t
  | Burn __b =>
    exact Reasoning.Budget.Burn.apply_sound (Nat) __s __b __t
public theorem Refund.replay_fire (__s : (Prod (Nat) (Nat))) (__step : Reasoning.Budget.Refund.Step) (__t : (Prod (Nat) (Nat))) : ((Reasoning.Budget.Refund.fire (__s) (__step) = Option.some (__t)) -> (Reasoning.Budget.Refund.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Refund.replay]
  rw [llE]
public theorem Refund.replay_sound (x : Nat) (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : Reasoning.Budget.Refund.Step) : ((LexLeanReasoning.Reaches ((Reasoning.Budget.Spent (Nat))) (Reasoning.Budget.Refund.observe (x)) (__acc)) -> (LexLeanReasoning.Reaches ((Reasoning.Budget.Spent (Nat))) (Reasoning.Budget.Refund.observe (x)) (Reasoning.Budget.Refund.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [Reasoning.Budget.Refund.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (Reasoning.Budget.Refund.fire_sound llS __step _ ‹_›)
public theorem Refund.derivation (x : Nat) (__trace : List (Reasoning.Budget.Refund.Step)) : (LexLeanReasoning.Reaches ((Reasoning.Budget.Spent (Nat))) (Reasoning.Budget.Refund.observe (x)) (Reasoning.Budget.Refund.follow (x) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((Reasoning.Budget.Refund.replay)) (fun (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) => (LexLeanReasoning.Reaches ((Reasoning.Budget.Spent (Nat))) (Reasoning.Budget.Refund.observe (x)) (__acc))) (Reasoning.Budget.Refund.replay_sound (x)) (__trace) (Except.ok (Reasoning.Budget.Refund.observe (x))) (LexLeanReasoning.reachesStart ((Reasoning.Budget.Spent (Nat))) (Reasoning.Budget.Refund.observe (x))))
public theorem Refund.accept_sound (x : Nat) (__s : (Prod (Nat) (Nat))) (__v : Nat) : ((Reasoning.Budget.Refund.accept (x) (__s) = Option.some (__v)) -> Reasoning.Budget.Empty (Nat) (x) (__v)) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Refund.accept] at llE
  split at llE
  · cases llE
  · have llC := LexLeanReasoning.checked _ _ _ llE
    exact llC.right ▸ Reasoning.Budget.empty_sound (Nat) _ _ llC.left
public theorem Refund.next_sound (__s : (Prod (Nat) (Nat))) (__t : (Prod (Nat) (Nat))) : ((Reasoning.Budget.Refund.next (__s) = Option.some (__t)) -> Reasoning.Budget.Spent (Nat) (__s) (__t)) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Refund.next] at llE
  split at llE
  · cases llE
  · exact Reasoning.Budget.Refund.fire_sound __s _ __t llE
public theorem Refund.step_none (__r : Reasoning.Budget.Refund.Run) : ((Reasoning.Budget.Refund.step (__r) = Option.none) -> (Reasoning.Budget.Refund.next ((__r).state) = Option.none)) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Refund.step] at llE
  split at llE
  · rename_i llH
    dsimp only [Reasoning.Budget.Refund.next]
    rw [llH]
  · rename_i llH
    split at llE
    · dsimp only [Reasoning.Budget.Refund.next]
      rw [llH]
      assumption
    · cases llE
public theorem Refund.step_some (__r : Reasoning.Budget.Refund.Run) (__q : Reasoning.Budget.Refund.Run) : ((Reasoning.Budget.Refund.step (__r) = Option.some (__q)) -> (Reasoning.Budget.Refund.next ((__r).state) = Option.some ((__q).state))) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Refund.step] at llE
  split at llE
  · cases llE
  · rename_i llH
    split at llE
    · cases llE
    · cases llE
      dsimp only [Reasoning.Budget.Refund.next]
      rw [llH]
      assumption
public theorem Refund.step_trace (x : Nat) (__r : Reasoning.Budget.Refund.Run) (__q : Reasoning.Budget.Refund.Run) : ((Reasoning.Budget.Refund.step (__r) = Option.some (__q)) -> ((Reasoning.Budget.Refund.follow (x) ((__r).trace) = Except.ok ((__r).state)) -> (Reasoning.Budget.Refund.follow (x) ((__q).trace) = Except.ok ((__q).state)))) :=
by
  intro llE llH
  dsimp only [Reasoning.Budget.Refund.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      dsimp only [Reasoning.Budget.Refund.follow]
      rw [LexLeanReasoning.foldSnoc]
      dsimp only [Reasoning.Budget.Refund.follow] at llH
      rw [llH]
      apply Reasoning.Budget.Refund.replay_fire
      assumption
public theorem Refund.step_count (__r : Reasoning.Budget.Refund.Run) (__q : Reasoning.Budget.Refund.Run) : ((Reasoning.Budget.Refund.step (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Refund.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Refund.step_fired (__r : Reasoning.Budget.Refund.Run) (__q : Reasoning.Budget.Refund.Run) : ((Reasoning.Budget.Refund.step (__r) = Option.some (__q)) -> (((__q).ledger).firings = (((__r).ledger).firings + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Refund.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Refund.saturate_derivation (x : Nat) : (LexLeanReasoning.Star ((Reasoning.Budget.Spent (Nat))) (Reasoning.Budget.Refund.observe (x)) ((Reasoning.Budget.Refund.saturate (x)).1)) :=
by
  dsimp only [Reasoning.Budget.Refund.saturate]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Budget.Refund.next)) (fun (__s : (Prod (Nat) (Nat))) => (LexLeanReasoning.Star ((Reasoning.Budget.Spent (Nat))) (Reasoning.Budget.Refund.observe (x)) (__s))) (fun llA llB llE llH => (LexLeanReasoning.Star.tail _ llA llB llH (Reasoning.Budget.Refund.next_sound llA llB llE))) ((x + 4)) (Reasoning.Budget.Refund.observe (x)) (LexLeanReasoning.Star.refl _))
public theorem Refund.run_state (x : Nat) : ((((Reasoning.Budget.Refund.run (x)).1).state = (Reasoning.Budget.Refund.saturate (x)).1) /\ ((Reasoning.Budget.Refund.run (x)).2 = (Reasoning.Budget.Refund.saturate (x)).2)) :=
by
  dsimp only [Reasoning.Budget.Refund.run, Reasoning.Budget.Refund.saturate]
  exact (LexLeanReasoning.iterateUntilSimulate ((Reasoning.Budget.Refund.step)) ((Reasoning.Budget.Refund.next)) ((fun (__r : Reasoning.Budget.Refund.Run) => (__r).state)) Reasoning.Budget.Refund.step_none Reasoning.Budget.Refund.step_some ((x + 4)) (Reasoning.Budget.Refund.start (x)))
public theorem Refund.run_trace (x : Nat) : (Reasoning.Budget.Refund.follow (x) (((Reasoning.Budget.Refund.run (x)).1).trace) = Except.ok (((Reasoning.Budget.Refund.run (x)).1).state)) :=
by
  dsimp only [Reasoning.Budget.Refund.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Budget.Refund.step)) (fun (__r : Reasoning.Budget.Refund.Run) => (Reasoning.Budget.Refund.follow (x) ((__r).trace) = Except.ok ((__r).state))) (Reasoning.Budget.Refund.step_trace (x)) ((x + 4)) (Reasoning.Budget.Refund.start (x)) rfl)
public theorem Refund.iterations_bounded (x : Nat) : ((((Reasoning.Budget.Refund.run (x)).1).ledger).iterations <= (x + 4)) :=
by
  dsimp only [Reasoning.Budget.Refund.run]
  exact (LexLeanReasoning.iterateUntilCount ((Reasoning.Budget.Refund.step)) ((fun (__r : Reasoning.Budget.Refund.Run) => ((__r).ledger).iterations)) Reasoning.Budget.Refund.step_count ((x + 4)) (Reasoning.Budget.Refund.start (x)) rfl)
public theorem Refund.firings_bounded (x : Nat) : ((((Reasoning.Budget.Refund.run (x)).1).ledger).firings <= (x + 4)) :=
by
  dsimp only [Reasoning.Budget.Refund.run]
  exact (LexLeanReasoning.iterateUntilCount ((Reasoning.Budget.Refund.step)) ((fun (__r : Reasoning.Budget.Refund.Run) => ((__r).ledger).firings)) Reasoning.Budget.Refund.step_fired ((x + 4)) (Reasoning.Budget.Refund.start (x)) rfl)
public theorem Refund.conclude_accept (x : Nat) (__s : (Prod (Nat) (Nat))) (__v : Nat) : ((Reasoning.Budget.Refund.conclude (x) (__s) = Except.ok (__v)) -> (Reasoning.Budget.Refund.accept (x) (__s) = Option.some (__v))) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Refund.conclude] at llE
  split at llE
  · cases llE
    assumption
  · split at llE
    · cases llE
    · cases llE
public theorem Refund.conclude_sound (x : Nat) (__s : (Prod (Nat) (Nat))) (__v : Nat) : ((Reasoning.Budget.Refund.conclude (x) (__s) = Except.ok (__v)) -> Reasoning.Budget.Empty (Nat) (x) (__v)) :=
by
  intro llE
  dsimp only [Reasoning.Budget.Refund.conclude] at llE
  split at llE
  · cases llE
    exact Reasoning.Budget.Refund.accept_sound _ _ _ ‹_›
  · split at llE
    · cases llE
    · cases llE
public theorem Refund.verdict_sound (x : Nat) : (forall (__v : Nat), ((Reasoning.Budget.Refund.verdict (x) = Except.ok (__v)) -> Reasoning.Budget.Empty (Nat) (x) (__v))) :=
by
  intro llV llE
  dsimp only [Reasoning.Budget.Refund.verdict] at llE
  generalize llRun : Reasoning.Budget.Refund.saturate x = llR at llE
  split at llE
  · exact Reasoning.Budget.Refund.conclude_sound _ _ _ llE
  · cases llE
public theorem Refund.explained (x : Nat) : (forall (__v : Nat), (forall (__trace : List (Reasoning.Budget.Refund.Step)), ((Reasoning.Budget.Refund (x) = Except.ok ((__v, __trace))) -> ((Reasoning.Budget.Refund.answer (x) (__trace) = Option.some (__v)) /\ Reasoning.Budget.Empty (Nat) (x) (__v))))) :=
by
  intro llV llT llE
  have llTrace := Reasoning.Budget.Refund.run_trace x
  dsimp only [Reasoning.Budget.Refund] at llE
  generalize llRun : Reasoning.Budget.Refund.run x = llR at llE llTrace
  split at llE
  · split at llE
    · rename_i llW llH
      cases llE
      exact And.intro (by dsimp only [Reasoning.Budget.Refund.answer]; rw [llTrace]; exact Reasoning.Budget.Refund.conclude_accept _ _ _ llH) (Reasoning.Budget.Refund.conclude_sound _ _ _ llH)
    · cases llE
  · cases llE

end Reasoning.Budget
