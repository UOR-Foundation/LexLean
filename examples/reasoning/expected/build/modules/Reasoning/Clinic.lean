module
public import Init
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
namespace Reasoning.Clinic

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

public structure Vitals where
  temperature : Nat
  heartRate : Nat
  respiratoryRate : Nat
  whiteCells : Nat
  systolic : Nat
  infection : Nat

public structure Chart where
  vitals : Vitals
  fever : Nat
  tachycardia : Nat
  tachypnea : Nat
  leukocytosis : Nat
  sirs : Nat
  sepsis : Nat
  shock : Nat
  level : Nat

@[expose] public def feverish (v : Vitals) : Bool := (Nat.blt (380) ((v).temperature))

@[expose] public def tachycardic (v : Vitals) : Bool := (Nat.blt (90) ((v).heartRate))

@[expose] public def tachypneic (v : Vitals) : Bool := (Nat.blt (20) ((v).respiratoryRate))

@[expose] public def leukocytic (v : Vitals) : Bool := (Nat.blt (12) ((v).whiteCells))

@[expose] public def hypotensive (v : Vitals) : Bool := (Nat.blt ((v).systolic) (90))

@[expose, reducible] public def Febrile (v : Vitals) : Prop := (380 < (v).temperature)

@[expose, reducible] public def Tachycardic (v : Vitals) : Prop := (90 < (v).heartRate)

@[expose, reducible] public def Tachypneic (v : Vitals) : Prop := (20 < (v).respiratoryRate)

@[expose, reducible] public def Leukocytic (v : Vitals) : Prop := (12 < (v).whiteCells)

@[expose, reducible] public def Hypotensive (v : Vitals) : Prop := ((v).systolic < 90)

@[expose, reducible] public def Warranted (c : Chart) : Prop := (((c).level <= 3) /\ ((((c).level = 1) -> ((c).sirs = 1)) /\ ((((c).level = 2) -> ((c).sepsis = 1)) /\ (((c).level = 3) -> ((c).shock = 1)))))

@[expose, reducible] public def Justified (s : Chart) (t : Chart) : Prop := ((((t).vitals).temperature = ((s).vitals).temperature) /\ ((((t).vitals).heartRate = ((s).vitals).heartRate) /\ ((((t).vitals).respiratoryRate = ((s).vitals).respiratoryRate) /\ ((((t).vitals).whiteCells = ((s).vitals).whiteCells) /\ ((((t).vitals).systolic = ((s).vitals).systolic) /\ ((((t).vitals).infection = ((s).vitals).infection) /\ (((s).fever <= (t).fever) /\ ((((s).fever < (t).fever) -> (((t).fever = 1) /\ Febrile ((t).vitals))) /\ (((s).tachycardia <= (t).tachycardia) /\ ((((s).tachycardia < (t).tachycardia) -> (((t).tachycardia = 1) /\ Tachycardic ((t).vitals))) /\ (((s).tachypnea <= (t).tachypnea) /\ ((((s).tachypnea < (t).tachypnea) -> (((t).tachypnea = 1) /\ Tachypneic ((t).vitals))) /\ (((s).leukocytosis <= (t).leukocytosis) /\ ((((s).leukocytosis < (t).leukocytosis) -> (((t).leukocytosis = 1) /\ Leukocytic ((t).vitals))) /\ (((s).sirs <= (t).sirs) /\ ((((s).sirs < (t).sirs) -> (((t).sirs = 1) /\ (2 <= ((((t).fever + (t).tachycardia) + (t).tachypnea) + (t).leukocytosis)))) /\ (((s).sepsis <= (t).sepsis) /\ ((((s).sepsis < (t).sepsis) -> (((t).sepsis = 1) /\ (((t).sirs = 1) /\ (((t).vitals).infection = 1)))) /\ (((s).shock <= (t).shock) /\ ((((s).shock < (t).shock) -> (((t).shock = 1) /\ (((t).sepsis = 1) /\ Hypotensive ((t).vitals)))) /\ (((s).level <= (t).level) /\ (((s).level < (t).level) -> (((t).level = ((s).level + 1)) /\ Warranted (t))))))))))))))))))))))))

@[expose, reducible] public def Consistent (c : Chart) : Prop := (((c).fever <= 1) /\ (((c).tachycardia <= 1) /\ (((c).tachypnea <= 1) /\ (((c).leukocytosis <= 1) /\ (((c).sirs <= 1) /\ (((c).sepsis <= 1) /\ (((c).shock <= 1) /\ ((c).level <= 3))))))))

@[expose] public def Pending (c : Chart) : Nat := ((LexLeanRuntime.subtract (1) ((c).fever) : Nat) + ((LexLeanRuntime.subtract (1) ((c).tachycardia) : Nat) + ((LexLeanRuntime.subtract (1) ((c).tachypnea) : Nat) + ((LexLeanRuntime.subtract (1) ((c).leukocytosis) : Nat) + ((LexLeanRuntime.subtract (1) ((c).sirs) : Nat) + ((LexLeanRuntime.subtract (1) ((c).sepsis) : Nat) + ((LexLeanRuntime.subtract (1) ((c).shock) : Nat) + (LexLeanRuntime.subtract (3) ((c).level) : Nat))))))))

public theorem consistent_preserved (s : Chart) (t : Chart) : (Consistent (s) -> (Justified (s) (t) -> Consistent (t))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Consistent, Febrile, Hypotensive, Justified, Leukocytic, Tachycardic, Tachypneic, Warranted, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem Findings.preserves : (LexLeanReasoning.Preserves ((Reasoning.Clinic.Justified)) ((Reasoning.Clinic.Consistent))) :=
  Reasoning.Clinic.consistent_preserved

public theorem fever_sound (s : Chart) : ((((Nat.beq ((s).fever) (0)) && feverish ((s).vitals)) = true) -> Justified (s) (({ vitals := (s).vitals, fever := 1, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Chart))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Febrile, Hypotensive, Justified, Leukocytic, Tachycardic, Tachypneic, Warranted, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem fever_progress (s : Chart) : (Consistent (s) -> ((((Nat.beq ((s).fever) (0)) && feverish ((s).vitals)) = true) -> (Pending (({ vitals := (s).vitals, fever := 1, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Chart)) < Pending (s)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Consistent, Pending, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose] public def Fever.guard (s : Reasoning.Clinic.Chart) : Bool := ((Nat.beq ((s).fever) (0)) && Reasoning.Clinic.feverish ((s).vitals))
@[expose] public def Fever.conclusion (s : Reasoning.Clinic.Chart) : Reasoning.Clinic.Chart := ({ vitals := (s).vitals, fever := 1, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Reasoning.Clinic.Chart)
@[expose] public def Fever.apply (s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Chart) := (if Reasoning.Clinic.Fever.guard (s) then Option.some (Reasoning.Clinic.Fever.conclusion (s)) else Option.none)
public theorem Fever.apply_sound (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Fever.apply (s) = Option.some (__t)) -> Reasoning.Clinic.Justified (s) (__t)) :=
  (LexLeanReasoning.guarded ((Reasoning.Clinic.Justified)) (s) (Reasoning.Clinic.Fever.guard (s)) (Reasoning.Clinic.Fever.conclusion (s)) (Reasoning.Clinic.fever_sound (s)) (__t))
public theorem Fever.apply_progress (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : (Reasoning.Clinic.Consistent (s) -> ((Reasoning.Clinic.Fever.apply (s) = Option.some (__t)) -> (Reasoning.Clinic.Pending (__t) < Reasoning.Clinic.Pending (s)))) :=
  (fun llJ => (LexLeanReasoning.guardedRank ((Reasoning.Clinic.Pending)) (s) (Reasoning.Clinic.Fever.guard (s)) (Reasoning.Clinic.Fever.conclusion (s)) (Reasoning.Clinic.fever_progress (s) llJ) (__t)))

public theorem tachycardia_sound (s : Chart) : ((((Nat.beq ((s).tachycardia) (0)) && tachycardic ((s).vitals)) = true) -> Justified (s) (({ vitals := (s).vitals, fever := (s).fever, tachycardia := 1, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Chart))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Febrile, Hypotensive, Justified, Leukocytic, Tachycardic, Tachypneic, Warranted, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem tachycardia_progress (s : Chart) : (Consistent (s) -> ((((Nat.beq ((s).tachycardia) (0)) && tachycardic ((s).vitals)) = true) -> (Pending (({ vitals := (s).vitals, fever := (s).fever, tachycardia := 1, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Chart)) < Pending (s)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Consistent, Pending, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose] public def Tachycardia.guard (s : Reasoning.Clinic.Chart) : Bool := ((Nat.beq ((s).tachycardia) (0)) && Reasoning.Clinic.tachycardic ((s).vitals))
@[expose] public def Tachycardia.conclusion (s : Reasoning.Clinic.Chart) : Reasoning.Clinic.Chart := ({ vitals := (s).vitals, fever := (s).fever, tachycardia := 1, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Reasoning.Clinic.Chart)
@[expose] public def Tachycardia.apply (s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Chart) := (if Reasoning.Clinic.Tachycardia.guard (s) then Option.some (Reasoning.Clinic.Tachycardia.conclusion (s)) else Option.none)
public theorem Tachycardia.apply_sound (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Tachycardia.apply (s) = Option.some (__t)) -> Reasoning.Clinic.Justified (s) (__t)) :=
  (LexLeanReasoning.guarded ((Reasoning.Clinic.Justified)) (s) (Reasoning.Clinic.Tachycardia.guard (s)) (Reasoning.Clinic.Tachycardia.conclusion (s)) (Reasoning.Clinic.tachycardia_sound (s)) (__t))
public theorem Tachycardia.apply_progress (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : (Reasoning.Clinic.Consistent (s) -> ((Reasoning.Clinic.Tachycardia.apply (s) = Option.some (__t)) -> (Reasoning.Clinic.Pending (__t) < Reasoning.Clinic.Pending (s)))) :=
  (fun llJ => (LexLeanReasoning.guardedRank ((Reasoning.Clinic.Pending)) (s) (Reasoning.Clinic.Tachycardia.guard (s)) (Reasoning.Clinic.Tachycardia.conclusion (s)) (Reasoning.Clinic.tachycardia_progress (s) llJ) (__t)))

public theorem tachypnea_sound (s : Chart) : ((((Nat.beq ((s).tachypnea) (0)) && tachypneic ((s).vitals)) = true) -> Justified (s) (({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := 1, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Chart))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Febrile, Hypotensive, Justified, Leukocytic, Tachycardic, Tachypneic, Warranted, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem tachypnea_progress (s : Chart) : (Consistent (s) -> ((((Nat.beq ((s).tachypnea) (0)) && tachypneic ((s).vitals)) = true) -> (Pending (({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := 1, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Chart)) < Pending (s)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Consistent, Pending, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose] public def Tachypnea.guard (s : Reasoning.Clinic.Chart) : Bool := ((Nat.beq ((s).tachypnea) (0)) && Reasoning.Clinic.tachypneic ((s).vitals))
@[expose] public def Tachypnea.conclusion (s : Reasoning.Clinic.Chart) : Reasoning.Clinic.Chart := ({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := 1, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Reasoning.Clinic.Chart)
@[expose] public def Tachypnea.apply (s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Chart) := (if Reasoning.Clinic.Tachypnea.guard (s) then Option.some (Reasoning.Clinic.Tachypnea.conclusion (s)) else Option.none)
public theorem Tachypnea.apply_sound (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Tachypnea.apply (s) = Option.some (__t)) -> Reasoning.Clinic.Justified (s) (__t)) :=
  (LexLeanReasoning.guarded ((Reasoning.Clinic.Justified)) (s) (Reasoning.Clinic.Tachypnea.guard (s)) (Reasoning.Clinic.Tachypnea.conclusion (s)) (Reasoning.Clinic.tachypnea_sound (s)) (__t))
public theorem Tachypnea.apply_progress (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : (Reasoning.Clinic.Consistent (s) -> ((Reasoning.Clinic.Tachypnea.apply (s) = Option.some (__t)) -> (Reasoning.Clinic.Pending (__t) < Reasoning.Clinic.Pending (s)))) :=
  (fun llJ => (LexLeanReasoning.guardedRank ((Reasoning.Clinic.Pending)) (s) (Reasoning.Clinic.Tachypnea.guard (s)) (Reasoning.Clinic.Tachypnea.conclusion (s)) (Reasoning.Clinic.tachypnea_progress (s) llJ) (__t)))

public theorem leukocytosis_sound (s : Chart) : ((((Nat.beq ((s).leukocytosis) (0)) && leukocytic ((s).vitals)) = true) -> Justified (s) (({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := 1, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Chart))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Febrile, Hypotensive, Justified, Leukocytic, Tachycardic, Tachypneic, Warranted, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem leukocytosis_progress (s : Chart) : (Consistent (s) -> ((((Nat.beq ((s).leukocytosis) (0)) && leukocytic ((s).vitals)) = true) -> (Pending (({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := 1, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Chart)) < Pending (s)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Consistent, Pending, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose] public def Leukocytosis.guard (s : Reasoning.Clinic.Chart) : Bool := ((Nat.beq ((s).leukocytosis) (0)) && Reasoning.Clinic.leukocytic ((s).vitals))
@[expose] public def Leukocytosis.conclusion (s : Reasoning.Clinic.Chart) : Reasoning.Clinic.Chart := ({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := 1, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Reasoning.Clinic.Chart)
@[expose] public def Leukocytosis.apply (s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Chart) := (if Reasoning.Clinic.Leukocytosis.guard (s) then Option.some (Reasoning.Clinic.Leukocytosis.conclusion (s)) else Option.none)
public theorem Leukocytosis.apply_sound (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Leukocytosis.apply (s) = Option.some (__t)) -> Reasoning.Clinic.Justified (s) (__t)) :=
  (LexLeanReasoning.guarded ((Reasoning.Clinic.Justified)) (s) (Reasoning.Clinic.Leukocytosis.guard (s)) (Reasoning.Clinic.Leukocytosis.conclusion (s)) (Reasoning.Clinic.leukocytosis_sound (s)) (__t))
public theorem Leukocytosis.apply_progress (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : (Reasoning.Clinic.Consistent (s) -> ((Reasoning.Clinic.Leukocytosis.apply (s) = Option.some (__t)) -> (Reasoning.Clinic.Pending (__t) < Reasoning.Clinic.Pending (s)))) :=
  (fun llJ => (LexLeanReasoning.guardedRank ((Reasoning.Clinic.Pending)) (s) (Reasoning.Clinic.Leukocytosis.guard (s)) (Reasoning.Clinic.Leukocytosis.conclusion (s)) (Reasoning.Clinic.leukocytosis_progress (s) llJ) (__t)))

public theorem sirs_sound (s : Chart) : ((((Nat.beq ((s).sirs) (0)) && (Nat.ble (2) (((((s).fever + (s).tachycardia) + (s).tachypnea) + (s).leukocytosis)))) = true) -> Justified (s) (({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := 1, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Chart))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Febrile, Hypotensive, Justified, Leukocytic, Tachycardic, Tachypneic, Warranted, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem sirs_progress (s : Chart) : (Consistent (s) -> ((((Nat.beq ((s).sirs) (0)) && (Nat.ble (2) (((((s).fever + (s).tachycardia) + (s).tachypnea) + (s).leukocytosis)))) = true) -> (Pending (({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := 1, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Chart)) < Pending (s)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Consistent, Pending, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose] public def Sirs.guard (s : Reasoning.Clinic.Chart) : Bool := ((Nat.beq ((s).sirs) (0)) && (Nat.ble (2) (((((s).fever + (s).tachycardia) + (s).tachypnea) + (s).leukocytosis))))
@[expose] public def Sirs.conclusion (s : Reasoning.Clinic.Chart) : Reasoning.Clinic.Chart := ({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := 1, sepsis := (s).sepsis, shock := (s).shock, level := (s).level } : Reasoning.Clinic.Chart)
@[expose] public def Sirs.apply (s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Chart) := (if Reasoning.Clinic.Sirs.guard (s) then Option.some (Reasoning.Clinic.Sirs.conclusion (s)) else Option.none)
public theorem Sirs.apply_sound (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Sirs.apply (s) = Option.some (__t)) -> Reasoning.Clinic.Justified (s) (__t)) :=
  (LexLeanReasoning.guarded ((Reasoning.Clinic.Justified)) (s) (Reasoning.Clinic.Sirs.guard (s)) (Reasoning.Clinic.Sirs.conclusion (s)) (Reasoning.Clinic.sirs_sound (s)) (__t))
public theorem Sirs.apply_progress (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : (Reasoning.Clinic.Consistent (s) -> ((Reasoning.Clinic.Sirs.apply (s) = Option.some (__t)) -> (Reasoning.Clinic.Pending (__t) < Reasoning.Clinic.Pending (s)))) :=
  (fun llJ => (LexLeanReasoning.guardedRank ((Reasoning.Clinic.Pending)) (s) (Reasoning.Clinic.Sirs.guard (s)) (Reasoning.Clinic.Sirs.conclusion (s)) (Reasoning.Clinic.sirs_progress (s) llJ) (__t)))

public theorem sepsis_sound (s : Chart) : ((((Nat.beq ((s).sepsis) (0)) && ((Nat.beq ((s).sirs) (1)) && (Nat.beq (((s).vitals).infection) (1)))) = true) -> Justified (s) (({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := 1, shock := (s).shock, level := (s).level } : Chart))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Febrile, Hypotensive, Justified, Leukocytic, Tachycardic, Tachypneic, Warranted, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem sepsis_progress (s : Chart) : (Consistent (s) -> ((((Nat.beq ((s).sepsis) (0)) && ((Nat.beq ((s).sirs) (1)) && (Nat.beq (((s).vitals).infection) (1)))) = true) -> (Pending (({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := 1, shock := (s).shock, level := (s).level } : Chart)) < Pending (s)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Consistent, Pending, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose] public def Sepsis.guard (s : Reasoning.Clinic.Chart) : Bool := ((Nat.beq ((s).sepsis) (0)) && ((Nat.beq ((s).sirs) (1)) && (Nat.beq (((s).vitals).infection) (1))))
@[expose] public def Sepsis.conclusion (s : Reasoning.Clinic.Chart) : Reasoning.Clinic.Chart := ({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := 1, shock := (s).shock, level := (s).level } : Reasoning.Clinic.Chart)
@[expose] public def Sepsis.apply (s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Chart) := (if Reasoning.Clinic.Sepsis.guard (s) then Option.some (Reasoning.Clinic.Sepsis.conclusion (s)) else Option.none)
public theorem Sepsis.apply_sound (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Sepsis.apply (s) = Option.some (__t)) -> Reasoning.Clinic.Justified (s) (__t)) :=
  (LexLeanReasoning.guarded ((Reasoning.Clinic.Justified)) (s) (Reasoning.Clinic.Sepsis.guard (s)) (Reasoning.Clinic.Sepsis.conclusion (s)) (Reasoning.Clinic.sepsis_sound (s)) (__t))
public theorem Sepsis.apply_progress (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : (Reasoning.Clinic.Consistent (s) -> ((Reasoning.Clinic.Sepsis.apply (s) = Option.some (__t)) -> (Reasoning.Clinic.Pending (__t) < Reasoning.Clinic.Pending (s)))) :=
  (fun llJ => (LexLeanReasoning.guardedRank ((Reasoning.Clinic.Pending)) (s) (Reasoning.Clinic.Sepsis.guard (s)) (Reasoning.Clinic.Sepsis.conclusion (s)) (Reasoning.Clinic.sepsis_progress (s) llJ) (__t)))

public theorem shock_sound (s : Chart) : ((((Nat.beq ((s).shock) (0)) && ((Nat.beq ((s).sepsis) (1)) && hypotensive ((s).vitals))) = true) -> Justified (s) (({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := 1, level := (s).level } : Chart))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Febrile, Hypotensive, Justified, Leukocytic, Tachycardic, Tachypneic, Warranted, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem shock_progress (s : Chart) : (Consistent (s) -> ((((Nat.beq ((s).shock) (0)) && ((Nat.beq ((s).sepsis) (1)) && hypotensive ((s).vitals))) = true) -> (Pending (({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := 1, level := (s).level } : Chart)) < Pending (s)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Consistent, Pending, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose] public def Shock.guard (s : Reasoning.Clinic.Chart) : Bool := ((Nat.beq ((s).shock) (0)) && ((Nat.beq ((s).sepsis) (1)) && Reasoning.Clinic.hypotensive ((s).vitals)))
@[expose] public def Shock.conclusion (s : Reasoning.Clinic.Chart) : Reasoning.Clinic.Chart := ({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := 1, level := (s).level } : Reasoning.Clinic.Chart)
@[expose] public def Shock.apply (s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Chart) := (if Reasoning.Clinic.Shock.guard (s) then Option.some (Reasoning.Clinic.Shock.conclusion (s)) else Option.none)
public theorem Shock.apply_sound (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Shock.apply (s) = Option.some (__t)) -> Reasoning.Clinic.Justified (s) (__t)) :=
  (LexLeanReasoning.guarded ((Reasoning.Clinic.Justified)) (s) (Reasoning.Clinic.Shock.guard (s)) (Reasoning.Clinic.Shock.conclusion (s)) (Reasoning.Clinic.shock_sound (s)) (__t))
public theorem Shock.apply_progress (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : (Reasoning.Clinic.Consistent (s) -> ((Reasoning.Clinic.Shock.apply (s) = Option.some (__t)) -> (Reasoning.Clinic.Pending (__t) < Reasoning.Clinic.Pending (s)))) :=
  (fun llJ => (LexLeanReasoning.guardedRank ((Reasoning.Clinic.Pending)) (s) (Reasoning.Clinic.Shock.guard (s)) (Reasoning.Clinic.Shock.conclusion (s)) (Reasoning.Clinic.shock_progress (s) llJ) (__t)))

public theorem escalate_sound (s : Chart) : ((((Nat.blt ((s).level) (3)) && (((Nat.beq ((s).level) (0)) && (Nat.beq ((s).sirs) (1))) || (((Nat.beq ((s).level) (1)) && (Nat.beq ((s).sepsis) (1))) || ((Nat.beq ((s).level) (2)) && (Nat.beq ((s).shock) (1)))))) = true) -> Justified (s) (({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := ((s).level + 1) } : Chart))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Febrile, Hypotensive, Justified, Leukocytic, Tachycardic, Tachypneic, Warranted, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem escalate_progress (s : Chart) : (Consistent (s) -> ((((Nat.blt ((s).level) (3)) && (((Nat.beq ((s).level) (0)) && (Nat.beq ((s).sirs) (1))) || (((Nat.beq ((s).level) (1)) && (Nat.beq ((s).sepsis) (1))) || ((Nat.beq ((s).level) (2)) && (Nat.beq ((s).shock) (1)))))) = true) -> (Pending (({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := ((s).level + 1) } : Chart)) < Pending (s)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Consistent, Pending, feverish, hypotensive, leukocytic, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose] public def Escalate.guard (s : Reasoning.Clinic.Chart) : Bool := ((Nat.blt ((s).level) (3)) && (((Nat.beq ((s).level) (0)) && (Nat.beq ((s).sirs) (1))) || (((Nat.beq ((s).level) (1)) && (Nat.beq ((s).sepsis) (1))) || ((Nat.beq ((s).level) (2)) && (Nat.beq ((s).shock) (1))))))
@[expose] public def Escalate.conclusion (s : Reasoning.Clinic.Chart) : Reasoning.Clinic.Chart := ({ vitals := (s).vitals, fever := (s).fever, tachycardia := (s).tachycardia, tachypnea := (s).tachypnea, leukocytosis := (s).leukocytosis, sirs := (s).sirs, sepsis := (s).sepsis, shock := (s).shock, level := ((s).level + 1) } : Reasoning.Clinic.Chart)
@[expose] public def Escalate.apply (s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Chart) := (if Reasoning.Clinic.Escalate.guard (s) then Option.some (Reasoning.Clinic.Escalate.conclusion (s)) else Option.none)
public theorem Escalate.apply_sound (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Escalate.apply (s) = Option.some (__t)) -> Reasoning.Clinic.Justified (s) (__t)) :=
  (LexLeanReasoning.guarded ((Reasoning.Clinic.Justified)) (s) (Reasoning.Clinic.Escalate.guard (s)) (Reasoning.Clinic.Escalate.conclusion (s)) (Reasoning.Clinic.escalate_sound (s)) (__t))
public theorem Escalate.apply_progress (s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : (Reasoning.Clinic.Consistent (s) -> ((Reasoning.Clinic.Escalate.apply (s) = Option.some (__t)) -> (Reasoning.Clinic.Pending (__t) < Reasoning.Clinic.Pending (s)))) :=
  (fun llJ => (LexLeanReasoning.guardedRank ((Reasoning.Clinic.Pending)) (s) (Reasoning.Clinic.Escalate.guard (s)) (Reasoning.Clinic.Escalate.conclusion (s)) (Reasoning.Clinic.escalate_progress (s) llJ) (__t)))

@[expose, reducible] public def Indicated (v : Vitals) (r : Nat) : Prop := ((r <= 3) /\ ((Hypotensive (v) /\ (((v).infection = 1) /\ (Febrile (v) /\ Tachycardic (v)))) -> (r = 3)))

@[expose] public def recommendationCheck (v : Vitals) (r : Nat) : Bool := ((Nat.ble (r) (3)) && ((Nat.beq (r) (3)) || (!(hypotensive (v) && ((Nat.beq ((v).infection) (1)) && (feverish (v) && tachycardic (v)))))))

public theorem recommendation_sound (v : Vitals) (r : Nat) : ((recommendationCheck (v) (r) = true) -> Indicated (v) (r)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Febrile, Hypotensive, Indicated, Leukocytic, Tachycardic, Tachypneic, feverish, hypotensive, leukocytic, recommendationCheck, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem recommendation_complete (v : Vitals) (r : Nat) : (Indicated (v) (r) -> (recommendationCheck (v) (r) = true)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Febrile, Hypotensive, Indicated, Leukocytic, Tachycardic, Tachypneic, feverish, hypotensive, leukocytic, recommendationCheck, tachycardic, tachypneic, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem Recommendation.sound : (LexLeanReasoning.Sound ((Reasoning.Clinic.recommendationCheck)) ((Reasoning.Clinic.Indicated))) :=
  Reasoning.Clinic.recommendation_sound
public theorem Recommendation.complete : (LexLeanReasoning.Complete ((Reasoning.Clinic.recommendationCheck)) ((Reasoning.Clinic.Indicated))) :=
  Reasoning.Clinic.recommendation_complete

@[expose, reducible] public def Discharged (_v : Vitals) (r : Nat) : Prop := (r = 0)

@[expose] public def dischargeCheck (_v : Vitals) (r : Nat) : Bool := (Nat.beq (r) (0))

public theorem discharge_sound (v : Vitals) (r : Nat) : ((dischargeCheck (v) (r) = true) -> Discharged (v) (r)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Discharged, dischargeCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem Outpatient.sound : (LexLeanReasoning.Sound ((Reasoning.Clinic.dischargeCheck)) ((Reasoning.Clinic.Discharged))) :=
  Reasoning.Clinic.discharge_sound

public theorem admitted_consistent (patient : Vitals) : Consistent (({ vitals := patient, fever := 0, tachycardia := 0, tachypnea := 0, leukocytosis := 0, sirs := 0, sepsis := 0, shock := 0, level := 0 } : Chart)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Consistent, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem admitted_pending (patient : Vitals) : (Pending (({ vitals := patient, fever := 0, tachycardia := 0, tachypnea := 0, leukocytosis := 0, sirs := 0, sepsis := 0, shock := 0, level := 0 } : Chart)) < 11) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Pending, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public inductive Triage.Step where
  | Fever
  | Tachycardia
  | Tachypnea
  | Leukocytosis
  | Sirs
  | Sepsis
  | Shock
  | Escalate
public structure Triage.Ledger where
  iterations : Nat
  attempts : Nat
  firings : Nat
  expansions : Nat
  verifications : Nat
  frontier : Nat
@[expose] public def Triage.observe (patient : Reasoning.Clinic.Vitals) : Reasoning.Clinic.Chart := ({ vitals := patient, fever := 0, tachycardia := 0, tachypnea := 0, leukocytosis := 0, sirs := 0, sepsis := 0, shock := 0, level := 0 } : Reasoning.Clinic.Chart)
@[expose] public def Triage.fire (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Clinic.Triage.Step) : Option (Reasoning.Clinic.Chart) := (match __step with | Reasoning.Clinic.Triage.Step.Fever => Reasoning.Clinic.Fever.apply (__s) | Reasoning.Clinic.Triage.Step.Tachycardia => Reasoning.Clinic.Tachycardia.apply (__s) | Reasoning.Clinic.Triage.Step.Tachypnea => Reasoning.Clinic.Tachypnea.apply (__s) | Reasoning.Clinic.Triage.Step.Leukocytosis => Reasoning.Clinic.Leukocytosis.apply (__s) | Reasoning.Clinic.Triage.Step.Sirs => Reasoning.Clinic.Sirs.apply (__s) | Reasoning.Clinic.Triage.Step.Sepsis => Reasoning.Clinic.Sepsis.apply (__s) | Reasoning.Clinic.Triage.Step.Shock => Reasoning.Clinic.Shock.apply (__s) | Reasoning.Clinic.Triage.Step.Escalate => Reasoning.Clinic.Escalate.apply (__s))
@[expose] public def Triage.replay (__acc : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart)) (__step : Reasoning.Clinic.Triage.Step) : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match Reasoning.Clinic.Triage.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def Triage.follow (patient : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Clinic.Triage.Step)) : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart) := (LexLeanCollections.listFold ((Reasoning.Clinic.Triage.replay)) (Except.ok (Reasoning.Clinic.Triage.observe (patient))) (__trace) : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart))
@[expose] public def Triage.extract (chart : Reasoning.Clinic.Chart) : Option (Nat) := Option.some ((chart).level)
@[expose] public def Triage.accept (patient : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) : Option (Nat) := (match Reasoning.Clinic.Triage.extract (__s) with | Option.none => Option.none | Option.some __v => (if Reasoning.Clinic.recommendationCheck (patient) (__v) then Option.some (__v) else Option.none))
@[expose] public def Triage.answer (patient : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Clinic.Triage.Step)) : Option (Nat) := (match Reasoning.Clinic.Triage.follow (patient) (__trace) with | Except.ok __s => Reasoning.Clinic.Triage.accept (patient) (__s) | Except.error _ => Option.none)
@[expose] public def Triage.select (__s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Triage.Step) := (match (match (match (if Reasoning.Clinic.Fever.guard (__s) then Option.some (Reasoning.Clinic.Triage.Step.Fever) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Tachycardia.guard (__s) then Option.some (Reasoning.Clinic.Triage.Step.Tachycardia) else Option.none)) with | Option.some __found => Option.some (__found) | Option.none => (match (if Reasoning.Clinic.Tachypnea.guard (__s) then Option.some (Reasoning.Clinic.Triage.Step.Tachypnea) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Leukocytosis.guard (__s) then Option.some (Reasoning.Clinic.Triage.Step.Leukocytosis) else Option.none))) with | Option.some __found => Option.some (__found) | Option.none => (match (match (if Reasoning.Clinic.Sirs.guard (__s) then Option.some (Reasoning.Clinic.Triage.Step.Sirs) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Sepsis.guard (__s) then Option.some (Reasoning.Clinic.Triage.Step.Sepsis) else Option.none)) with | Option.some __found => Option.some (__found) | Option.none => (match (if Reasoning.Clinic.Shock.guard (__s) then Option.some (Reasoning.Clinic.Triage.Step.Shock) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Escalate.guard (__s) then Option.some (Reasoning.Clinic.Triage.Step.Escalate) else Option.none))))
@[expose] public def Triage.attempts (__s : Reasoning.Clinic.Chart) : Nat := ((let __scan6l : (Prod (Bool) (Nat)) := (let __scan2l : (Prod (Bool) (Nat)) := (let __scan0l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Fever.guard (__s), 1); (if (__scan0l).1 then __scan0l else (let __scan0r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Tachycardia.guard (__s), 1); ((__scan0r).1, ((__scan0l).2 + (__scan0r).2))))); (if (__scan2l).1 then __scan2l else (let __scan2r : (Prod (Bool) (Nat)) := (let __scan1l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Tachypnea.guard (__s), 1); (if (__scan1l).1 then __scan1l else (let __scan1r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Leukocytosis.guard (__s), 1); ((__scan1r).1, ((__scan1l).2 + (__scan1r).2))))); ((__scan2r).1, ((__scan2l).2 + (__scan2r).2))))); (if (__scan6l).1 then __scan6l else (let __scan6r : (Prod (Bool) (Nat)) := (let __scan5l : (Prod (Bool) (Nat)) := (let __scan3l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Sirs.guard (__s), 1); (if (__scan3l).1 then __scan3l else (let __scan3r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Sepsis.guard (__s), 1); ((__scan3r).1, ((__scan3l).2 + (__scan3r).2))))); (if (__scan5l).1 then __scan5l else (let __scan5r : (Prod (Bool) (Nat)) := (let __scan4l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Shock.guard (__s), 1); (if (__scan4l).1 then __scan4l else (let __scan4r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Escalate.guard (__s), 1); ((__scan4r).1, ((__scan4l).2 + (__scan4r).2))))); ((__scan5r).1, ((__scan5l).2 + (__scan5r).2))))); ((__scan6r).1, ((__scan6l).2 + (__scan6r).2)))))).2
@[expose] public def Triage.next (__s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Chart) := (match Reasoning.Clinic.Triage.select (__s) with | Option.none => Option.none | Option.some __step => Reasoning.Clinic.Triage.fire (__s) (__step))
@[expose] public def Triage.saturate (patient : Reasoning.Clinic.Vitals) : (Prod (Reasoning.Clinic.Chart) (Bool)) := (LexLeanCollections.iterateUntil ((Reasoning.Clinic.Triage.next)) (11) (Reasoning.Clinic.Triage.observe (patient)) : (Prod (Reasoning.Clinic.Chart) (Bool)))
public structure Triage.Run where
  state : Reasoning.Clinic.Chart
  trace : List (Reasoning.Clinic.Triage.Step)
  ledger : Reasoning.Clinic.Triage.Ledger
@[expose] public def Triage.start (patient : Reasoning.Clinic.Vitals) : Reasoning.Clinic.Triage.Run := ({ state := Reasoning.Clinic.Triage.observe (patient), trace := ([] : List (Reasoning.Clinic.Triage.Step)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := 0 } : Reasoning.Clinic.Triage.Ledger) } : Reasoning.Clinic.Triage.Run)
@[expose] public def Triage.step (__r : Reasoning.Clinic.Triage.Run) : Option (Reasoning.Clinic.Triage.Run) := (match Reasoning.Clinic.Triage.select ((__r).state) with | Option.none => Option.none | Option.some __step => (match Reasoning.Clinic.Triage.fire ((__r).state) (__step) with | Option.none => Option.none | Option.some __t => Option.some (({ state := __t, trace := (LexLeanRuntime.append ((__r).trace) ((__step :: ([] : List (Reasoning.Clinic.Triage.Step)))) : List (Reasoning.Clinic.Triage.Step)), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + Reasoning.Clinic.Triage.attempts ((__r).state)), firings := (((__r).ledger).firings + 1), expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : Reasoning.Clinic.Triage.Ledger) } : Reasoning.Clinic.Triage.Run))))
@[expose] public def Triage.run (patient : Reasoning.Clinic.Vitals) : (Prod (Reasoning.Clinic.Triage.Run) (Bool)) := (LexLeanCollections.iterateUntil ((Reasoning.Clinic.Triage.step)) (11) (Reasoning.Clinic.Triage.start (patient)) : (Prod (Reasoning.Clinic.Triage.Run) (Bool)))
@[expose] public def Triage.account (patient : Reasoning.Clinic.Vitals) : Reasoning.Clinic.Triage.Ledger := (let __final : (Prod (Reasoning.Clinic.Triage.Run) (Bool)) := Reasoning.Clinic.Triage.run (patient); (match (__final).2 with | Bool.true => ({ iterations := (((__final).1).ledger).iterations, attempts := ((((__final).1).ledger).attempts + Reasoning.Clinic.Triage.attempts (((__final).1).state)), firings := (((__final).1).ledger).firings, expansions := (((__final).1).ledger).expansions, verifications := ((((__final).1).ledger).verifications + 1), frontier := (((__final).1).ledger).frontier } : Reasoning.Clinic.Triage.Ledger) | Bool.false => ((__final).1).ledger))
@[expose] public def Triage.conclude (patient : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) : Except ((Prod Bool Bool)) (Nat) := (match Reasoning.Clinic.Triage.accept (patient) (__s) with | Option.some __v => Except.ok (__v) | Option.none => (match Reasoning.Clinic.Triage.extract (__s) with | Option.some _ => Except.error (((true, false) : Prod Bool Bool)) | Option.none => Except.error (((false, true) : Prod Bool Bool))))
@[expose] public def Triage.verdict (patient : Reasoning.Clinic.Vitals) : Except ((Prod Bool Bool)) (Nat) := (let __final : (Prod (Reasoning.Clinic.Chart) (Bool)) := Reasoning.Clinic.Triage.saturate (patient); (match (__final).2 with | Bool.true => Reasoning.Clinic.Triage.conclude (patient) ((__final).1) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
@[expose] public def Triage (patient : Reasoning.Clinic.Vitals) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (Reasoning.Clinic.Triage.Step)))) := (let __final : (Prod (Reasoning.Clinic.Triage.Run) (Bool)) := Reasoning.Clinic.Triage.run (patient); (match (__final).2 with | Bool.true => (match Reasoning.Clinic.Triage.conclude (patient) (((__final).1).state) with | Except.ok __v => Except.ok ((__v, ((__final).1).trace)) | Except.error __e => Except.error (__e)) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
public theorem Triage.fire_sound (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Clinic.Triage.Step) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Triage.fire (__s) (__step) = Option.some (__t)) -> Reasoning.Clinic.Justified (__s) (__t)) :=
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
public theorem Triage.fire_progress (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Clinic.Triage.Step) (__t : Reasoning.Clinic.Chart) : (Reasoning.Clinic.Consistent (__s) -> ((Reasoning.Clinic.Triage.fire (__s) (__step) = Option.some (__t)) -> (Reasoning.Clinic.Pending (__t) < Reasoning.Clinic.Pending (__s)))) :=
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
public theorem Triage.replay_fire (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Clinic.Triage.Step) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Triage.fire (__s) (__step) = Option.some (__t)) -> (Reasoning.Clinic.Triage.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Triage.replay]
  rw [llE]
public theorem Triage.replay_sound (patient : Reasoning.Clinic.Vitals) (__acc : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart)) (__step : Reasoning.Clinic.Triage.Step) : ((LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Triage.observe (patient)) (__acc)) -> (LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Triage.observe (patient)) (Reasoning.Clinic.Triage.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [Reasoning.Clinic.Triage.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (Reasoning.Clinic.Triage.fire_sound llS __step _ ‹_›)
public theorem Triage.derivation (patient : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Clinic.Triage.Step)) : (LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Triage.observe (patient)) (Reasoning.Clinic.Triage.follow (patient) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((Reasoning.Clinic.Triage.replay)) (fun (__acc : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart)) => (LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Triage.observe (patient)) (__acc))) (Reasoning.Clinic.Triage.replay_sound (patient)) (__trace) (Except.ok (Reasoning.Clinic.Triage.observe (patient))) (LexLeanReasoning.reachesStart ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Triage.observe (patient))))
public theorem Triage.follow_invariant (patient : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Clinic.Triage.Step)) (__s : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Triage.follow (patient) (__trace) = Except.ok (__s)) -> Reasoning.Clinic.Consistent (__s)) :=
  (fun llE => (LexLeanReasoning.starPreserves ((Reasoning.Clinic.Justified)) ((Reasoning.Clinic.Consistent)) Reasoning.Clinic.Findings.preserves (Reasoning.Clinic.Triage.observe (patient)) (__s) (Reasoning.Clinic.Triage.derivation (patient) (__trace) (__s) llE) (Reasoning.Clinic.admitted_consistent (patient))))
public theorem Triage.accept_sound (patient : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) (__v : Nat) : ((Reasoning.Clinic.Triage.accept (patient) (__s) = Option.some (__v)) -> Reasoning.Clinic.Indicated (patient) (__v)) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Triage.accept] at llE
  split at llE
  · cases llE
  · have llC := LexLeanReasoning.checked _ _ _ llE
    exact llC.right ▸ Reasoning.Clinic.recommendation_sound _ _ llC.left
public theorem Triage.next_sound (__s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Triage.next (__s) = Option.some (__t)) -> Reasoning.Clinic.Justified (__s) (__t)) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Triage.next] at llE
  split at llE
  · cases llE
  · exact Reasoning.Clinic.Triage.fire_sound __s _ __t llE
public theorem Triage.next_preserves (__s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Triage.next (__s) = Option.some (__t)) -> (Reasoning.Clinic.Consistent (__s) -> Reasoning.Clinic.Consistent (__t))) :=
  (fun llE llH => (Reasoning.Clinic.consistent_preserved (__s) (__t) llH (Reasoning.Clinic.Triage.next_sound (__s) (__t) llE)))
public theorem Triage.next_progress (__s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Triage.next (__s) = Option.some (__t)) -> (Reasoning.Clinic.Consistent (__s) -> (Reasoning.Clinic.Pending (__t) < Reasoning.Clinic.Pending (__s)))) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Triage.next] at llE
  split at llE
  · cases llE
  · exact fun llJ => Reasoning.Clinic.Triage.fire_progress __s _ __t llJ llE
public theorem Triage.step_none (__r : Reasoning.Clinic.Triage.Run) : ((Reasoning.Clinic.Triage.step (__r) = Option.none) -> (Reasoning.Clinic.Triage.next ((__r).state) = Option.none)) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Triage.step] at llE
  split at llE
  · rename_i llH
    dsimp only [Reasoning.Clinic.Triage.next]
    rw [llH]
  · rename_i llH
    split at llE
    · dsimp only [Reasoning.Clinic.Triage.next]
      rw [llH]
      assumption
    · cases llE
public theorem Triage.step_some (__r : Reasoning.Clinic.Triage.Run) (__q : Reasoning.Clinic.Triage.Run) : ((Reasoning.Clinic.Triage.step (__r) = Option.some (__q)) -> (Reasoning.Clinic.Triage.next ((__r).state) = Option.some ((__q).state))) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Triage.step] at llE
  split at llE
  · cases llE
  · rename_i llH
    split at llE
    · cases llE
    · cases llE
      dsimp only [Reasoning.Clinic.Triage.next]
      rw [llH]
      assumption
public theorem Triage.step_trace (patient : Reasoning.Clinic.Vitals) (__r : Reasoning.Clinic.Triage.Run) (__q : Reasoning.Clinic.Triage.Run) : ((Reasoning.Clinic.Triage.step (__r) = Option.some (__q)) -> ((Reasoning.Clinic.Triage.follow (patient) ((__r).trace) = Except.ok ((__r).state)) -> (Reasoning.Clinic.Triage.follow (patient) ((__q).trace) = Except.ok ((__q).state)))) :=
by
  intro llE llH
  dsimp only [Reasoning.Clinic.Triage.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      dsimp only [Reasoning.Clinic.Triage.follow]
      rw [LexLeanReasoning.foldSnoc]
      dsimp only [Reasoning.Clinic.Triage.follow] at llH
      rw [llH]
      apply Reasoning.Clinic.Triage.replay_fire
      assumption
public theorem Triage.step_count (__r : Reasoning.Clinic.Triage.Run) (__q : Reasoning.Clinic.Triage.Run) : ((Reasoning.Clinic.Triage.step (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Triage.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Triage.step_fired (__r : Reasoning.Clinic.Triage.Run) (__q : Reasoning.Clinic.Triage.Run) : ((Reasoning.Clinic.Triage.step (__r) = Option.some (__q)) -> (((__q).ledger).firings = (((__r).ledger).firings + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Triage.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Triage.saturate_derivation (patient : Reasoning.Clinic.Vitals) : (LexLeanReasoning.Star ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Triage.observe (patient)) ((Reasoning.Clinic.Triage.saturate (patient)).1)) :=
by
  dsimp only [Reasoning.Clinic.Triage.saturate]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Clinic.Triage.next)) (fun (__s : Reasoning.Clinic.Chart) => (LexLeanReasoning.Star ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Triage.observe (patient)) (__s))) (fun llA llB llE llH => (LexLeanReasoning.Star.tail _ llA llB llH (Reasoning.Clinic.Triage.next_sound llA llB llE))) (11) (Reasoning.Clinic.Triage.observe (patient)) (LexLeanReasoning.Star.refl _))
public theorem Triage.run_state (patient : Reasoning.Clinic.Vitals) : ((((Reasoning.Clinic.Triage.run (patient)).1).state = (Reasoning.Clinic.Triage.saturate (patient)).1) /\ ((Reasoning.Clinic.Triage.run (patient)).2 = (Reasoning.Clinic.Triage.saturate (patient)).2)) :=
by
  dsimp only [Reasoning.Clinic.Triage.run, Reasoning.Clinic.Triage.saturate]
  exact (LexLeanReasoning.iterateUntilSimulate ((Reasoning.Clinic.Triage.step)) ((Reasoning.Clinic.Triage.next)) ((fun (__r : Reasoning.Clinic.Triage.Run) => (__r).state)) Reasoning.Clinic.Triage.step_none Reasoning.Clinic.Triage.step_some (11) (Reasoning.Clinic.Triage.start (patient)))
public theorem Triage.run_trace (patient : Reasoning.Clinic.Vitals) : (Reasoning.Clinic.Triage.follow (patient) (((Reasoning.Clinic.Triage.run (patient)).1).trace) = Except.ok (((Reasoning.Clinic.Triage.run (patient)).1).state)) :=
by
  dsimp only [Reasoning.Clinic.Triage.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Clinic.Triage.step)) (fun (__r : Reasoning.Clinic.Triage.Run) => (Reasoning.Clinic.Triage.follow (patient) ((__r).trace) = Except.ok ((__r).state))) (Reasoning.Clinic.Triage.step_trace (patient)) (11) (Reasoning.Clinic.Triage.start (patient)) rfl)
public theorem Triage.iterations_bounded (patient : Reasoning.Clinic.Vitals) : ((((Reasoning.Clinic.Triage.run (patient)).1).ledger).iterations <= 11) :=
by
  dsimp only [Reasoning.Clinic.Triage.run]
  exact (LexLeanReasoning.iterateUntilCount ((Reasoning.Clinic.Triage.step)) ((fun (__r : Reasoning.Clinic.Triage.Run) => ((__r).ledger).iterations)) Reasoning.Clinic.Triage.step_count (11) (Reasoning.Clinic.Triage.start (patient)) rfl)
public theorem Triage.firings_bounded (patient : Reasoning.Clinic.Vitals) : ((((Reasoning.Clinic.Triage.run (patient)).1).ledger).firings <= 11) :=
by
  dsimp only [Reasoning.Clinic.Triage.run]
  exact (LexLeanReasoning.iterateUntilCount ((Reasoning.Clinic.Triage.step)) ((fun (__r : Reasoning.Clinic.Triage.Run) => ((__r).ledger).firings)) Reasoning.Clinic.Triage.step_fired (11) (Reasoning.Clinic.Triage.start (patient)) rfl)
public theorem Triage.saturate_invariant (patient : Reasoning.Clinic.Vitals) : Reasoning.Clinic.Consistent ((Reasoning.Clinic.Triage.saturate (patient)).1) :=
by
  dsimp only [Reasoning.Clinic.Triage.saturate]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Clinic.Triage.next)) ((Reasoning.Clinic.Consistent)) Reasoning.Clinic.Triage.next_preserves (11) (Reasoning.Clinic.Triage.observe (patient)) (Reasoning.Clinic.admitted_consistent (patient)))
public theorem Triage.saturates (patient : Reasoning.Clinic.Vitals) : ((Reasoning.Clinic.Triage.saturate (patient)).2 = true) :=
by
  dsimp only [Reasoning.Clinic.Triage.saturate]
  exact (LexLeanReasoning.iterateUntilStops ((Reasoning.Clinic.Triage.next)) ((Reasoning.Clinic.Consistent)) ((Reasoning.Clinic.Pending)) (fun llA llB llE llH => (And.intro (Reasoning.Clinic.Triage.next_preserves llA llB llE llH) (Reasoning.Clinic.Triage.next_progress llA llB llE llH))) (11) (Reasoning.Clinic.Triage.observe (patient)) (Reasoning.Clinic.admitted_consistent (patient)) (Reasoning.Clinic.admitted_pending (patient)))
public theorem Triage.conclude_accept (patient : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) (__v : Nat) : ((Reasoning.Clinic.Triage.conclude (patient) (__s) = Except.ok (__v)) -> (Reasoning.Clinic.Triage.accept (patient) (__s) = Option.some (__v))) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Triage.conclude] at llE
  split at llE
  · cases llE
    assumption
  · split at llE
    · cases llE
    · cases llE
public theorem Triage.conclude_sound (patient : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) (__v : Nat) : ((Reasoning.Clinic.Triage.conclude (patient) (__s) = Except.ok (__v)) -> Reasoning.Clinic.Indicated (patient) (__v)) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Triage.conclude] at llE
  split at llE
  · cases llE
    exact Reasoning.Clinic.Triage.accept_sound _ _ _ ‹_›
  · split at llE
    · cases llE
    · cases llE
public theorem Triage.verdict_sound (patient : Reasoning.Clinic.Vitals) : (forall (__v : Nat), ((Reasoning.Clinic.Triage.verdict (patient) = Except.ok (__v)) -> Reasoning.Clinic.Indicated (patient) (__v))) :=
by
  intro llV llE
  dsimp only [Reasoning.Clinic.Triage.verdict] at llE
  generalize llRun : Reasoning.Clinic.Triage.saturate patient = llR at llE
  split at llE
  · exact Reasoning.Clinic.Triage.conclude_sound _ _ _ llE
  · cases llE
public theorem Triage.explained (patient : Reasoning.Clinic.Vitals) : (forall (__v : Nat), (forall (__trace : List (Reasoning.Clinic.Triage.Step)), ((Reasoning.Clinic.Triage (patient) = Except.ok ((__v, __trace))) -> ((Reasoning.Clinic.Triage.answer (patient) (__trace) = Option.some (__v)) /\ Reasoning.Clinic.Indicated (patient) (__v))))) :=
by
  intro llV llT llE
  have llTrace := Reasoning.Clinic.Triage.run_trace patient
  dsimp only [Reasoning.Clinic.Triage] at llE
  generalize llRun : Reasoning.Clinic.Triage.run patient = llR at llE llTrace
  split at llE
  · split at llE
    · rename_i llW llH
      cases llE
      exact And.intro (by dsimp only [Reasoning.Clinic.Triage.answer]; rw [llTrace]; exact Reasoning.Clinic.Triage.conclude_accept _ _ _ llH) (Reasoning.Clinic.Triage.conclude_sound _ _ _ llH)
    · cases llE
  · cases llE

public inductive Review.Step where
  | Fever
  | Tachycardia
  | Tachypnea
  | Leukocytosis
  | Sirs
  | Sepsis
  | Shock
  | Escalate
public structure Review.Ledger where
  iterations : Nat
  attempts : Nat
  firings : Nat
  expansions : Nat
  verifications : Nat
  frontier : Nat
@[expose] public def Review.observe (patient : Reasoning.Clinic.Vitals) : Reasoning.Clinic.Chart := ({ vitals := patient, fever := 0, tachycardia := 0, tachypnea := 0, leukocytosis := 0, sirs := 0, sepsis := 0, shock := 0, level := 0 } : Reasoning.Clinic.Chart)
@[expose] public def Review.fire (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Clinic.Review.Step) : Option (Reasoning.Clinic.Chart) := (match __step with | Reasoning.Clinic.Review.Step.Fever => Reasoning.Clinic.Fever.apply (__s) | Reasoning.Clinic.Review.Step.Tachycardia => Reasoning.Clinic.Tachycardia.apply (__s) | Reasoning.Clinic.Review.Step.Tachypnea => Reasoning.Clinic.Tachypnea.apply (__s) | Reasoning.Clinic.Review.Step.Leukocytosis => Reasoning.Clinic.Leukocytosis.apply (__s) | Reasoning.Clinic.Review.Step.Sirs => Reasoning.Clinic.Sirs.apply (__s) | Reasoning.Clinic.Review.Step.Sepsis => Reasoning.Clinic.Sepsis.apply (__s) | Reasoning.Clinic.Review.Step.Shock => Reasoning.Clinic.Shock.apply (__s) | Reasoning.Clinic.Review.Step.Escalate => Reasoning.Clinic.Escalate.apply (__s))
@[expose] public def Review.replay (__acc : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart)) (__step : Reasoning.Clinic.Review.Step) : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match Reasoning.Clinic.Review.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def Review.follow (patient : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Clinic.Review.Step)) : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart) := (LexLeanCollections.listFold ((Reasoning.Clinic.Review.replay)) (Except.ok (Reasoning.Clinic.Review.observe (patient))) (__trace) : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart))
@[expose] public def Review.extract (chart : Reasoning.Clinic.Chart) : Option (Nat) := Option.some ((chart).level)
@[expose] public def Review.accept (patient : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) : Option (Nat) := (match Reasoning.Clinic.Review.extract (__s) with | Option.none => Option.none | Option.some __v => (if Reasoning.Clinic.dischargeCheck (patient) (__v) then Option.some (__v) else Option.none))
@[expose] public def Review.answer (patient : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Clinic.Review.Step)) : Option (Nat) := (match Reasoning.Clinic.Review.follow (patient) (__trace) with | Except.ok __s => Reasoning.Clinic.Review.accept (patient) (__s) | Except.error _ => Option.none)
@[expose] public def Review.select (__s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Review.Step) := (match (match (match (if Reasoning.Clinic.Fever.guard (__s) then Option.some (Reasoning.Clinic.Review.Step.Fever) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Tachycardia.guard (__s) then Option.some (Reasoning.Clinic.Review.Step.Tachycardia) else Option.none)) with | Option.some __found => Option.some (__found) | Option.none => (match (if Reasoning.Clinic.Tachypnea.guard (__s) then Option.some (Reasoning.Clinic.Review.Step.Tachypnea) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Leukocytosis.guard (__s) then Option.some (Reasoning.Clinic.Review.Step.Leukocytosis) else Option.none))) with | Option.some __found => Option.some (__found) | Option.none => (match (match (if Reasoning.Clinic.Sirs.guard (__s) then Option.some (Reasoning.Clinic.Review.Step.Sirs) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Sepsis.guard (__s) then Option.some (Reasoning.Clinic.Review.Step.Sepsis) else Option.none)) with | Option.some __found => Option.some (__found) | Option.none => (match (if Reasoning.Clinic.Shock.guard (__s) then Option.some (Reasoning.Clinic.Review.Step.Shock) else Option.none) with | Option.some __found => Option.some (__found) | Option.none => (if Reasoning.Clinic.Escalate.guard (__s) then Option.some (Reasoning.Clinic.Review.Step.Escalate) else Option.none))))
@[expose] public def Review.attempts (__s : Reasoning.Clinic.Chart) : Nat := ((let __scan6l : (Prod (Bool) (Nat)) := (let __scan2l : (Prod (Bool) (Nat)) := (let __scan0l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Fever.guard (__s), 1); (if (__scan0l).1 then __scan0l else (let __scan0r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Tachycardia.guard (__s), 1); ((__scan0r).1, ((__scan0l).2 + (__scan0r).2))))); (if (__scan2l).1 then __scan2l else (let __scan2r : (Prod (Bool) (Nat)) := (let __scan1l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Tachypnea.guard (__s), 1); (if (__scan1l).1 then __scan1l else (let __scan1r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Leukocytosis.guard (__s), 1); ((__scan1r).1, ((__scan1l).2 + (__scan1r).2))))); ((__scan2r).1, ((__scan2l).2 + (__scan2r).2))))); (if (__scan6l).1 then __scan6l else (let __scan6r : (Prod (Bool) (Nat)) := (let __scan5l : (Prod (Bool) (Nat)) := (let __scan3l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Sirs.guard (__s), 1); (if (__scan3l).1 then __scan3l else (let __scan3r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Sepsis.guard (__s), 1); ((__scan3r).1, ((__scan3l).2 + (__scan3r).2))))); (if (__scan5l).1 then __scan5l else (let __scan5r : (Prod (Bool) (Nat)) := (let __scan4l : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Shock.guard (__s), 1); (if (__scan4l).1 then __scan4l else (let __scan4r : (Prod (Bool) (Nat)) := (Reasoning.Clinic.Escalate.guard (__s), 1); ((__scan4r).1, ((__scan4l).2 + (__scan4r).2))))); ((__scan5r).1, ((__scan5l).2 + (__scan5r).2))))); ((__scan6r).1, ((__scan6l).2 + (__scan6r).2)))))).2
@[expose] public def Review.next (__s : Reasoning.Clinic.Chart) : Option (Reasoning.Clinic.Chart) := (match Reasoning.Clinic.Review.select (__s) with | Option.none => Option.none | Option.some __step => Reasoning.Clinic.Review.fire (__s) (__step))
@[expose] public def Review.saturate (patient : Reasoning.Clinic.Vitals) : (Prod (Reasoning.Clinic.Chart) (Bool)) := (LexLeanCollections.iterateUntil ((Reasoning.Clinic.Review.next)) (6) (Reasoning.Clinic.Review.observe (patient)) : (Prod (Reasoning.Clinic.Chart) (Bool)))
public structure Review.Run where
  state : Reasoning.Clinic.Chart
  trace : List (Reasoning.Clinic.Review.Step)
  ledger : Reasoning.Clinic.Review.Ledger
@[expose] public def Review.start (patient : Reasoning.Clinic.Vitals) : Reasoning.Clinic.Review.Run := ({ state := Reasoning.Clinic.Review.observe (patient), trace := ([] : List (Reasoning.Clinic.Review.Step)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := 0 } : Reasoning.Clinic.Review.Ledger) } : Reasoning.Clinic.Review.Run)
@[expose] public def Review.step (__r : Reasoning.Clinic.Review.Run) : Option (Reasoning.Clinic.Review.Run) := (match Reasoning.Clinic.Review.select ((__r).state) with | Option.none => Option.none | Option.some __step => (match Reasoning.Clinic.Review.fire ((__r).state) (__step) with | Option.none => Option.none | Option.some __t => Option.some (({ state := __t, trace := (LexLeanRuntime.append ((__r).trace) ((__step :: ([] : List (Reasoning.Clinic.Review.Step)))) : List (Reasoning.Clinic.Review.Step)), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + Reasoning.Clinic.Review.attempts ((__r).state)), firings := (((__r).ledger).firings + 1), expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : Reasoning.Clinic.Review.Ledger) } : Reasoning.Clinic.Review.Run))))
@[expose] public def Review.run (patient : Reasoning.Clinic.Vitals) : (Prod (Reasoning.Clinic.Review.Run) (Bool)) := (LexLeanCollections.iterateUntil ((Reasoning.Clinic.Review.step)) (6) (Reasoning.Clinic.Review.start (patient)) : (Prod (Reasoning.Clinic.Review.Run) (Bool)))
@[expose] public def Review.account (patient : Reasoning.Clinic.Vitals) : Reasoning.Clinic.Review.Ledger := (let __final : (Prod (Reasoning.Clinic.Review.Run) (Bool)) := Reasoning.Clinic.Review.run (patient); (match (__final).2 with | Bool.true => ({ iterations := (((__final).1).ledger).iterations, attempts := ((((__final).1).ledger).attempts + Reasoning.Clinic.Review.attempts (((__final).1).state)), firings := (((__final).1).ledger).firings, expansions := (((__final).1).ledger).expansions, verifications := ((((__final).1).ledger).verifications + 1), frontier := (((__final).1).ledger).frontier } : Reasoning.Clinic.Review.Ledger) | Bool.false => ((__final).1).ledger))
@[expose] public def Review.conclude (patient : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) : Except ((Prod Bool Bool)) (Nat) := (match Reasoning.Clinic.Review.accept (patient) (__s) with | Option.some __v => Except.ok (__v) | Option.none => (match Reasoning.Clinic.Review.extract (__s) with | Option.some _ => Except.error (((true, false) : Prod Bool Bool)) | Option.none => Except.error (((false, true) : Prod Bool Bool))))
@[expose] public def Review.verdict (patient : Reasoning.Clinic.Vitals) : Except ((Prod Bool Bool)) (Nat) := (let __final : (Prod (Reasoning.Clinic.Chart) (Bool)) := Reasoning.Clinic.Review.saturate (patient); (match (__final).2 with | Bool.true => Reasoning.Clinic.Review.conclude (patient) ((__final).1) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
@[expose] public def Review (patient : Reasoning.Clinic.Vitals) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (Reasoning.Clinic.Review.Step)))) := (let __final : (Prod (Reasoning.Clinic.Review.Run) (Bool)) := Reasoning.Clinic.Review.run (patient); (match (__final).2 with | Bool.true => (match Reasoning.Clinic.Review.conclude (patient) (((__final).1).state) with | Except.ok __v => Except.ok ((__v, ((__final).1).trace)) | Except.error __e => Except.error (__e)) | Bool.false => Except.error (((false, false) : Prod Bool Bool))))
public theorem Review.fire_sound (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Clinic.Review.Step) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Review.fire (__s) (__step) = Option.some (__t)) -> Reasoning.Clinic.Justified (__s) (__t)) :=
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
public theorem Review.replay_fire (__s : Reasoning.Clinic.Chart) (__step : Reasoning.Clinic.Review.Step) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Review.fire (__s) (__step) = Option.some (__t)) -> (Reasoning.Clinic.Review.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Review.replay]
  rw [llE]
public theorem Review.replay_sound (patient : Reasoning.Clinic.Vitals) (__acc : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart)) (__step : Reasoning.Clinic.Review.Step) : ((LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Review.observe (patient)) (__acc)) -> (LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Review.observe (patient)) (Reasoning.Clinic.Review.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [Reasoning.Clinic.Review.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (Reasoning.Clinic.Review.fire_sound llS __step _ ‹_›)
public theorem Review.derivation (patient : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Clinic.Review.Step)) : (LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Review.observe (patient)) (Reasoning.Clinic.Review.follow (patient) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((Reasoning.Clinic.Review.replay)) (fun (__acc : Except ((Prod Bool Bool)) (Reasoning.Clinic.Chart)) => (LexLeanReasoning.Reaches ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Review.observe (patient)) (__acc))) (Reasoning.Clinic.Review.replay_sound (patient)) (__trace) (Except.ok (Reasoning.Clinic.Review.observe (patient))) (LexLeanReasoning.reachesStart ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Review.observe (patient))))
public theorem Review.follow_invariant (patient : Reasoning.Clinic.Vitals) (__trace : List (Reasoning.Clinic.Review.Step)) (__s : Reasoning.Clinic.Chart) : (Reasoning.Clinic.Consistent (Reasoning.Clinic.Review.observe (patient)) -> ((Reasoning.Clinic.Review.follow (patient) (__trace) = Except.ok (__s)) -> Reasoning.Clinic.Consistent (__s))) :=
  (fun llI llE => (LexLeanReasoning.starPreserves ((Reasoning.Clinic.Justified)) ((Reasoning.Clinic.Consistent)) Reasoning.Clinic.Findings.preserves (Reasoning.Clinic.Review.observe (patient)) (__s) (Reasoning.Clinic.Review.derivation (patient) (__trace) (__s) llE) llI))
public theorem Review.accept_sound (patient : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) (__v : Nat) : ((Reasoning.Clinic.Review.accept (patient) (__s) = Option.some (__v)) -> Reasoning.Clinic.Discharged (patient) (__v)) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Review.accept] at llE
  split at llE
  · cases llE
  · have llC := LexLeanReasoning.checked _ _ _ llE
    exact llC.right ▸ Reasoning.Clinic.discharge_sound _ _ llC.left
public theorem Review.next_sound (__s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Review.next (__s) = Option.some (__t)) -> Reasoning.Clinic.Justified (__s) (__t)) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Review.next] at llE
  split at llE
  · cases llE
  · exact Reasoning.Clinic.Review.fire_sound __s _ __t llE
public theorem Review.next_preserves (__s : Reasoning.Clinic.Chart) (__t : Reasoning.Clinic.Chart) : ((Reasoning.Clinic.Review.next (__s) = Option.some (__t)) -> (Reasoning.Clinic.Consistent (__s) -> Reasoning.Clinic.Consistent (__t))) :=
  (fun llE llH => (Reasoning.Clinic.consistent_preserved (__s) (__t) llH (Reasoning.Clinic.Review.next_sound (__s) (__t) llE)))
public theorem Review.step_none (__r : Reasoning.Clinic.Review.Run) : ((Reasoning.Clinic.Review.step (__r) = Option.none) -> (Reasoning.Clinic.Review.next ((__r).state) = Option.none)) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Review.step] at llE
  split at llE
  · rename_i llH
    dsimp only [Reasoning.Clinic.Review.next]
    rw [llH]
  · rename_i llH
    split at llE
    · dsimp only [Reasoning.Clinic.Review.next]
      rw [llH]
      assumption
    · cases llE
public theorem Review.step_some (__r : Reasoning.Clinic.Review.Run) (__q : Reasoning.Clinic.Review.Run) : ((Reasoning.Clinic.Review.step (__r) = Option.some (__q)) -> (Reasoning.Clinic.Review.next ((__r).state) = Option.some ((__q).state))) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Review.step] at llE
  split at llE
  · cases llE
  · rename_i llH
    split at llE
    · cases llE
    · cases llE
      dsimp only [Reasoning.Clinic.Review.next]
      rw [llH]
      assumption
public theorem Review.step_trace (patient : Reasoning.Clinic.Vitals) (__r : Reasoning.Clinic.Review.Run) (__q : Reasoning.Clinic.Review.Run) : ((Reasoning.Clinic.Review.step (__r) = Option.some (__q)) -> ((Reasoning.Clinic.Review.follow (patient) ((__r).trace) = Except.ok ((__r).state)) -> (Reasoning.Clinic.Review.follow (patient) ((__q).trace) = Except.ok ((__q).state)))) :=
by
  intro llE llH
  dsimp only [Reasoning.Clinic.Review.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      dsimp only [Reasoning.Clinic.Review.follow]
      rw [LexLeanReasoning.foldSnoc]
      dsimp only [Reasoning.Clinic.Review.follow] at llH
      rw [llH]
      apply Reasoning.Clinic.Review.replay_fire
      assumption
public theorem Review.step_count (__r : Reasoning.Clinic.Review.Run) (__q : Reasoning.Clinic.Review.Run) : ((Reasoning.Clinic.Review.step (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Review.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Review.step_fired (__r : Reasoning.Clinic.Review.Run) (__q : Reasoning.Clinic.Review.Run) : ((Reasoning.Clinic.Review.step (__r) = Option.some (__q)) -> (((__q).ledger).firings = (((__r).ledger).firings + 1))) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Review.step] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · cases llE
      rfl
public theorem Review.saturate_derivation (patient : Reasoning.Clinic.Vitals) : (LexLeanReasoning.Star ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Review.observe (patient)) ((Reasoning.Clinic.Review.saturate (patient)).1)) :=
by
  dsimp only [Reasoning.Clinic.Review.saturate]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Clinic.Review.next)) (fun (__s : Reasoning.Clinic.Chart) => (LexLeanReasoning.Star ((Reasoning.Clinic.Justified)) (Reasoning.Clinic.Review.observe (patient)) (__s))) (fun llA llB llE llH => (LexLeanReasoning.Star.tail _ llA llB llH (Reasoning.Clinic.Review.next_sound llA llB llE))) (6) (Reasoning.Clinic.Review.observe (patient)) (LexLeanReasoning.Star.refl _))
public theorem Review.run_state (patient : Reasoning.Clinic.Vitals) : ((((Reasoning.Clinic.Review.run (patient)).1).state = (Reasoning.Clinic.Review.saturate (patient)).1) /\ ((Reasoning.Clinic.Review.run (patient)).2 = (Reasoning.Clinic.Review.saturate (patient)).2)) :=
by
  dsimp only [Reasoning.Clinic.Review.run, Reasoning.Clinic.Review.saturate]
  exact (LexLeanReasoning.iterateUntilSimulate ((Reasoning.Clinic.Review.step)) ((Reasoning.Clinic.Review.next)) ((fun (__r : Reasoning.Clinic.Review.Run) => (__r).state)) Reasoning.Clinic.Review.step_none Reasoning.Clinic.Review.step_some (6) (Reasoning.Clinic.Review.start (patient)))
public theorem Review.run_trace (patient : Reasoning.Clinic.Vitals) : (Reasoning.Clinic.Review.follow (patient) (((Reasoning.Clinic.Review.run (patient)).1).trace) = Except.ok (((Reasoning.Clinic.Review.run (patient)).1).state)) :=
by
  dsimp only [Reasoning.Clinic.Review.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Clinic.Review.step)) (fun (__r : Reasoning.Clinic.Review.Run) => (Reasoning.Clinic.Review.follow (patient) ((__r).trace) = Except.ok ((__r).state))) (Reasoning.Clinic.Review.step_trace (patient)) (6) (Reasoning.Clinic.Review.start (patient)) rfl)
public theorem Review.iterations_bounded (patient : Reasoning.Clinic.Vitals) : ((((Reasoning.Clinic.Review.run (patient)).1).ledger).iterations <= 6) :=
by
  dsimp only [Reasoning.Clinic.Review.run]
  exact (LexLeanReasoning.iterateUntilCount ((Reasoning.Clinic.Review.step)) ((fun (__r : Reasoning.Clinic.Review.Run) => ((__r).ledger).iterations)) Reasoning.Clinic.Review.step_count (6) (Reasoning.Clinic.Review.start (patient)) rfl)
public theorem Review.firings_bounded (patient : Reasoning.Clinic.Vitals) : ((((Reasoning.Clinic.Review.run (patient)).1).ledger).firings <= 6) :=
by
  dsimp only [Reasoning.Clinic.Review.run]
  exact (LexLeanReasoning.iterateUntilCount ((Reasoning.Clinic.Review.step)) ((fun (__r : Reasoning.Clinic.Review.Run) => ((__r).ledger).firings)) Reasoning.Clinic.Review.step_fired (6) (Reasoning.Clinic.Review.start (patient)) rfl)
public theorem Review.saturate_invariant (patient : Reasoning.Clinic.Vitals) : (Reasoning.Clinic.Consistent (Reasoning.Clinic.Review.observe (patient)) -> Reasoning.Clinic.Consistent ((Reasoning.Clinic.Review.saturate (patient)).1)) :=
by
  dsimp only [Reasoning.Clinic.Review.saturate]
  exact (fun llI => (LexLeanReasoning.iterateUntilInvariant ((Reasoning.Clinic.Review.next)) ((Reasoning.Clinic.Consistent)) Reasoning.Clinic.Review.next_preserves (6) (Reasoning.Clinic.Review.observe (patient)) llI))
public theorem Review.conclude_accept (patient : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) (__v : Nat) : ((Reasoning.Clinic.Review.conclude (patient) (__s) = Except.ok (__v)) -> (Reasoning.Clinic.Review.accept (patient) (__s) = Option.some (__v))) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Review.conclude] at llE
  split at llE
  · cases llE
    assumption
  · split at llE
    · cases llE
    · cases llE
public theorem Review.conclude_sound (patient : Reasoning.Clinic.Vitals) (__s : Reasoning.Clinic.Chart) (__v : Nat) : ((Reasoning.Clinic.Review.conclude (patient) (__s) = Except.ok (__v)) -> Reasoning.Clinic.Discharged (patient) (__v)) :=
by
  intro llE
  dsimp only [Reasoning.Clinic.Review.conclude] at llE
  split at llE
  · cases llE
    exact Reasoning.Clinic.Review.accept_sound _ _ _ ‹_›
  · split at llE
    · cases llE
    · cases llE
public theorem Review.verdict_sound (patient : Reasoning.Clinic.Vitals) : (forall (__v : Nat), ((Reasoning.Clinic.Review.verdict (patient) = Except.ok (__v)) -> Reasoning.Clinic.Discharged (patient) (__v))) :=
by
  intro llV llE
  dsimp only [Reasoning.Clinic.Review.verdict] at llE
  generalize llRun : Reasoning.Clinic.Review.saturate patient = llR at llE
  split at llE
  · exact Reasoning.Clinic.Review.conclude_sound _ _ _ llE
  · cases llE
public theorem Review.explained (patient : Reasoning.Clinic.Vitals) : (forall (__v : Nat), (forall (__trace : List (Reasoning.Clinic.Review.Step)), ((Reasoning.Clinic.Review (patient) = Except.ok ((__v, __trace))) -> ((Reasoning.Clinic.Review.answer (patient) (__trace) = Option.some (__v)) /\ Reasoning.Clinic.Discharged (patient) (__v))))) :=
by
  intro llV llT llE
  have llTrace := Reasoning.Clinic.Review.run_trace patient
  dsimp only [Reasoning.Clinic.Review] at llE
  generalize llRun : Reasoning.Clinic.Review.run patient = llR at llE llTrace
  split at llE
  · split at llE
    · rename_i llW llH
      cases llE
      exact And.intro (by dsimp only [Reasoning.Clinic.Review.answer]; rw [llTrace]; exact Reasoning.Clinic.Review.conclude_accept _ _ _ llH) (Reasoning.Clinic.Review.conclude_sound _ _ _ llH)
    · cases llE
  · cases llE

end Reasoning.Clinic
