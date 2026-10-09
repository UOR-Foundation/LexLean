module
public import Init
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
namespace Compiler.PlannerOracle

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

@[expose, reducible] public def Moves (_s : (Prod (Nat) (Nat))) (t : (Prod (Nat) (Nat))) : Prop := (((t).1 <= 4) /\ ((t).2 <= 3))

@[expose, reducible] public def Fits (s : (Prod (Nat) (Nat))) : Prop := (((s).1 <= 4) /\ ((s).2 <= 3))

public theorem fits_kept (s : (Prod (Nat) (Nat))) (t : (Prod (Nat) (Nat))) : (Fits (s) -> (Moves (s) (t) -> Fits (t))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Fits, Moves, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem Pouring.preserves : (LexLeanReasoning.Preserves ((Compiler.PlannerOracle.Moves)) ((Compiler.PlannerOracle.Fits))) :=
  Compiler.PlannerOracle.fits_kept

public theorem filla_sound (s : (Prod (Nat) (Nat))) : ((((Nat.blt ((s).1) (4)) && (Nat.ble ((s).2) (3))) = true) -> Moves (s) ((4, (s).2))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Moves, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose] public def FillA.guard (s : (Prod (Nat) (Nat))) : Bool := ((Nat.blt ((s).1) (4)) && (Nat.ble ((s).2) (3)))
@[expose] public def FillA.conclusion (s : (Prod (Nat) (Nat))) : (Prod (Nat) (Nat)) := (4, (s).2)
@[expose] public def FillA.apply (s : (Prod (Nat) (Nat))) : Option ((Prod (Nat) (Nat))) := (if Compiler.PlannerOracle.FillA.guard (s) then Option.some (Compiler.PlannerOracle.FillA.conclusion (s)) else Option.none)
public theorem FillA.apply_sound (s : (Prod (Nat) (Nat))) (__t : (Prod (Nat) (Nat))) : ((Compiler.PlannerOracle.FillA.apply (s) = Option.some (__t)) -> Compiler.PlannerOracle.Moves (s) (__t)) :=
  (LexLeanReasoning.guarded ((Compiler.PlannerOracle.Moves)) (s) (Compiler.PlannerOracle.FillA.guard (s)) (Compiler.PlannerOracle.FillA.conclusion (s)) (Compiler.PlannerOracle.filla_sound (s)) (__t))

public theorem emptyb_sound (s : (Prod (Nat) (Nat))) : ((((Nat.blt (0) ((s).2)) && (Nat.ble ((s).1) (4))) = true) -> Moves (s) (((s).1, 0))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Moves, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose] public def EmptyB.guard (s : (Prod (Nat) (Nat))) : Bool := ((Nat.blt (0) ((s).2)) && (Nat.ble ((s).1) (4)))
@[expose] public def EmptyB.conclusion (s : (Prod (Nat) (Nat))) : (Prod (Nat) (Nat)) := ((s).1, 0)
@[expose] public def EmptyB.apply (s : (Prod (Nat) (Nat))) : Option ((Prod (Nat) (Nat))) := (if Compiler.PlannerOracle.EmptyB.guard (s) then Option.some (Compiler.PlannerOracle.EmptyB.conclusion (s)) else Option.none)
public theorem EmptyB.apply_sound (s : (Prod (Nat) (Nat))) (__t : (Prod (Nat) (Nat))) : ((Compiler.PlannerOracle.EmptyB.apply (s) = Option.some (__t)) -> Compiler.PlannerOracle.Moves (s) (__t)) :=
  (LexLeanReasoning.guarded ((Compiler.PlannerOracle.Moves)) (s) (Compiler.PlannerOracle.EmptyB.guard (s)) (Compiler.PlannerOracle.EmptyB.conclusion (s)) (Compiler.PlannerOracle.emptyb_sound (s)) (__t))

public theorem pour_sound (s : (Prod (Nat) (Nat))) (k : Nat) : ((((Nat.ble (k) ((s).1)) && ((Nat.ble (((s).2 + k)) (3)) && (Nat.ble ((s).1) (4)))) = true) -> Moves (s) (((LexLeanRuntime.subtract ((s).1) (k) : Nat), ((s).2 + k)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Moves, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose] public def Pour.guard (s : (Prod (Nat) (Nat))) (k : Nat) : Bool := ((Nat.ble (k) ((s).1)) && ((Nat.ble (((s).2 + k)) (3)) && (Nat.ble ((s).1) (4))))
@[expose] public def Pour.conclusion (s : (Prod (Nat) (Nat))) (k : Nat) : (Prod (Nat) (Nat)) := ((LexLeanRuntime.subtract ((s).1) (k) : Nat), ((s).2 + k))
@[expose] public def Pour.candidates (_s : (Prod (Nat) (Nat))) : List (Nat) := (1 :: (2 :: (3 :: ([] : List (Nat)))))
@[expose] public def Pour.apply (s : (Prod (Nat) (Nat))) (k : Nat) : Option ((Prod (Nat) (Nat))) := (if Compiler.PlannerOracle.Pour.guard (s) (k) then Option.some (Compiler.PlannerOracle.Pour.conclusion (s) (k)) else Option.none)
public theorem Pour.apply_sound (s : (Prod (Nat) (Nat))) (k : Nat) (__t : (Prod (Nat) (Nat))) : ((Compiler.PlannerOracle.Pour.apply (s) (k) = Option.some (__t)) -> Compiler.PlannerOracle.Moves (s) (__t)) :=
  (LexLeanReasoning.guarded ((Compiler.PlannerOracle.Moves)) (s) (Compiler.PlannerOracle.Pour.guard (s) (k)) (Compiler.PlannerOracle.Pour.conclusion (s) (k)) (Compiler.PlannerOracle.pour_sound (s) (k)) (__t))

@[expose, reducible] public def Measures (target : Nat) (r : (Prod (Nat) (Nat))) : Prop := (((r).2 = target) /\ ((r).1 <= 4))

@[expose] public def measuresCheck (target : Nat) (r : (Prod (Nat) (Nat))) : Bool := ((Nat.beq ((r).2) (target)) && (Nat.ble ((r).1) (4)))

public theorem measures_sound (target : Nat) (r : (Prod (Nat) (Nat))) : ((measuresCheck (target) (r) = true) -> Measures (target) (r)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Measures, measuresCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem Measured.sound : (LexLeanReasoning.Sound ((Compiler.PlannerOracle.measuresCheck)) ((Compiler.PlannerOracle.Measures))) :=
  Compiler.PlannerOracle.measures_sound

public theorem empty_jugs_fit (_target : Nat) : Fits ((0, 0)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Fits, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public inductive Plan.Step where
  | FillA
  | EmptyB
  | Pour (_ : Nat)
public structure Plan.Ledger where
  iterations : Nat
  attempts : Nat
  firings : Nat
  expansions : Nat
  verifications : Nat
  frontier : Nat
@[expose] public def Plan.observe (_target : Nat) : (Prod (Nat) (Nat)) := (0, 0)
@[expose] public def Plan.fire (__s : (Prod (Nat) (Nat))) (__step : Compiler.PlannerOracle.Plan.Step) : Option ((Prod (Nat) (Nat))) := (match __step with | Compiler.PlannerOracle.Plan.Step.FillA => Compiler.PlannerOracle.FillA.apply (__s) | Compiler.PlannerOracle.Plan.Step.EmptyB => Compiler.PlannerOracle.EmptyB.apply (__s) | Compiler.PlannerOracle.Plan.Step.Pour __b => Compiler.PlannerOracle.Pour.apply (__s) (__b))
@[expose] public def Plan.replay (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : Compiler.PlannerOracle.Plan.Step) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match Compiler.PlannerOracle.Plan.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def Plan.follow (target : Nat) (__trace : List (Compiler.PlannerOracle.Plan.Step)) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (LexLeanCollections.listFold ((Compiler.PlannerOracle.Plan.replay)) (Except.ok (Compiler.PlannerOracle.Plan.observe (target))) (__trace) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))))
@[expose] public def Plan.extract (jugs : (Prod (Nat) (Nat))) : Option ((Prod (Nat) (Nat))) := Option.some (jugs)
@[expose] public def Plan.accept (target : Nat) (__s : (Prod (Nat) (Nat))) : Option ((Prod (Nat) (Nat))) := (match Compiler.PlannerOracle.Plan.extract (__s) with | Option.none => Option.none | Option.some __v => (if Compiler.PlannerOracle.measuresCheck (target) (__v) then Option.some (__v) else Option.none))
@[expose] public def Plan.answer (target : Nat) (__trace : List (Compiler.PlannerOracle.Plan.Step)) : Option ((Prod (Nat) (Nat))) := (match Compiler.PlannerOracle.Plan.follow (target) (__trace) with | Except.ok __s => Compiler.PlannerOracle.Plan.accept (target) (__s) | Except.error _ => Option.none)
public structure Plan.Node where
  state : (Prod (Nat) (Nat))
  trace : List (Compiler.PlannerOracle.Plan.Step)
public structure Plan.Search where
  frontier : List (Compiler.PlannerOracle.Plan.Node)
  visited : List ((Prod (Nat) (Nat)))
  found : Option ((Prod ((Prod (Nat) (Nat))) (Compiler.PlannerOracle.Plan.Node)))
  truncated : Bool
  ledger : Compiler.PlannerOracle.Plan.Ledger
@[expose] public def Plan.FillA.successors (__node : Compiler.PlannerOracle.Plan.Node) : List (Compiler.PlannerOracle.Plan.Node) := (match Compiler.PlannerOracle.Plan.fire ((__node).state) (Compiler.PlannerOracle.Plan.Step.FillA) with | Option.none => ([] : List (Compiler.PlannerOracle.Plan.Node)) | Option.some __t => (({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((Compiler.PlannerOracle.Plan.Step.FillA :: ([] : List (Compiler.PlannerOracle.Plan.Step)))) : List (Compiler.PlannerOracle.Plan.Step)) } : Compiler.PlannerOracle.Plan.Node) :: ([] : List (Compiler.PlannerOracle.Plan.Node))))
@[expose] public def Plan.EmptyB.successors (__node : Compiler.PlannerOracle.Plan.Node) : List (Compiler.PlannerOracle.Plan.Node) := (match Compiler.PlannerOracle.Plan.fire ((__node).state) (Compiler.PlannerOracle.Plan.Step.EmptyB) with | Option.none => ([] : List (Compiler.PlannerOracle.Plan.Node)) | Option.some __t => (({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((Compiler.PlannerOracle.Plan.Step.EmptyB :: ([] : List (Compiler.PlannerOracle.Plan.Step)))) : List (Compiler.PlannerOracle.Plan.Step)) } : Compiler.PlannerOracle.Plan.Node) :: ([] : List (Compiler.PlannerOracle.Plan.Node))))
@[expose] public def Plan.Pour.collect (__node : Compiler.PlannerOracle.Plan.Node) (__acc : List (Compiler.PlannerOracle.Plan.Node)) (__b : Nat) : List (Compiler.PlannerOracle.Plan.Node) := (match Compiler.PlannerOracle.Plan.fire ((__node).state) (Compiler.PlannerOracle.Plan.Step.Pour (__b)) with | Option.none => __acc | Option.some __t => (LexLeanRuntime.append (__acc) ((({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((Compiler.PlannerOracle.Plan.Step.Pour (__b) :: ([] : List (Compiler.PlannerOracle.Plan.Step)))) : List (Compiler.PlannerOracle.Plan.Step)) } : Compiler.PlannerOracle.Plan.Node) :: ([] : List (Compiler.PlannerOracle.Plan.Node)))) : List (Compiler.PlannerOracle.Plan.Node)))
@[expose] public def Plan.Pour.successors (__node : Compiler.PlannerOracle.Plan.Node) : List (Compiler.PlannerOracle.Plan.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Compiler.PlannerOracle.Plan.Node)) (__b : Nat) => Compiler.PlannerOracle.Plan.Pour.collect (__node) (__acc) (__b))) (([] : List (Compiler.PlannerOracle.Plan.Node))) (Compiler.PlannerOracle.Pour.candidates ((__node).state)) : List (Compiler.PlannerOracle.Plan.Node))
@[expose] public def Plan.successors (__node : Compiler.PlannerOracle.Plan.Node) : List (Compiler.PlannerOracle.Plan.Node) := (LexLeanRuntime.append (Compiler.PlannerOracle.Plan.FillA.successors (__node)) ((LexLeanRuntime.append (Compiler.PlannerOracle.Plan.EmptyB.successors (__node)) (Compiler.PlannerOracle.Plan.Pour.successors (__node)) : List (Compiler.PlannerOracle.Plan.Node))) : List (Compiler.PlannerOracle.Plan.Node))
@[expose] public def Plan.fresh (__visited : List ((Prod (Nat) (Nat)))) (__nodes : List (Compiler.PlannerOracle.Plan.Node)) : (Prod (List (Compiler.PlannerOracle.Plan.Node)) (List ((Prod (Nat) (Nat))))) := (LexLeanCollections.listFold ((fun (__acc : (Prod (List (Compiler.PlannerOracle.Plan.Node)) (List ((Prod (Nat) (Nat)))))) (__n : Compiler.PlannerOracle.Plan.Node) => (if (LexLeanCollections.setContains ((__acc).2) ((__n).state) : Bool) then __acc else ((LexLeanRuntime.append ((__acc).1) ((__n :: ([] : List (Compiler.PlannerOracle.Plan.Node)))) : List (Compiler.PlannerOracle.Plan.Node)), (LexLeanCollections.setInsert ((__acc).2) ((__n).state) : List ((Prod (Nat) (Nat)))))))) ((([] : List (Compiler.PlannerOracle.Plan.Node)), __visited)) (__nodes) : (Prod (List (Compiler.PlannerOracle.Plan.Node)) (List ((Prod (Nat) (Nat))))))
@[expose] public def Plan.start (target : Nat) : Compiler.PlannerOracle.Plan.Search := (let __first : List (Compiler.PlannerOracle.Plan.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Compiler.PlannerOracle.Plan.Node)) (__n : Compiler.PlannerOracle.Plan.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) ((target + target))) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (Compiler.PlannerOracle.Plan.Node)))) : List (Compiler.PlannerOracle.Plan.Node)) else __acc))) (([] : List (Compiler.PlannerOracle.Plan.Node))) ((({ state := Compiler.PlannerOracle.Plan.observe (target), trace := ([] : List (Compiler.PlannerOracle.Plan.Step)) } : Compiler.PlannerOracle.Plan.Node) :: ([] : List (Compiler.PlannerOracle.Plan.Node)))) : List (Compiler.PlannerOracle.Plan.Node)); ({ frontier := __first, visited := (LexLeanCollections.setInsert (([] : List ((Prod (Nat) (Nat))))) (Compiler.PlannerOracle.Plan.observe (target)) : List ((Prod (Nat) (Nat)))), found := Option.none, truncated := (Nat.blt ((target + target)) (1)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := (LexLeanRuntime.length (__first) : Nat) } : Compiler.PlannerOracle.Plan.Ledger) } : Compiler.PlannerOracle.Plan.Search))
@[expose] public def Plan.attempts (__s : (Prod (Nat) (Nat))) : Nat := (1 + (1 + (LexLeanRuntime.length (Compiler.PlannerOracle.Pour.candidates (__s)) : Nat)))
@[expose] public def Plan.searchStep (target : Nat) (__r : Compiler.PlannerOracle.Plan.Search) : Option (Compiler.PlannerOracle.Plan.Search) := (match (__r).found with | Option.some _ => Option.none | Option.none => (match (__r).frontier with | List.nil => Option.none | List.cons __node __rest => (match Compiler.PlannerOracle.Plan.accept (target) ((__node).state) with | Option.some __v => Option.some (({ frontier := __rest, visited := (__r).visited, found := Option.some ((__v, __node)), truncated := (__r).truncated, ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := ((__r).ledger).attempts, firings := ((__r).ledger).firings, expansions := ((__r).ledger).expansions, verifications := (((__r).ledger).verifications + 1), frontier := ((__r).ledger).frontier } : Compiler.PlannerOracle.Plan.Ledger) } : Compiler.PlannerOracle.Plan.Search)) | Option.none => (let __successors : List (Compiler.PlannerOracle.Plan.Node) := Compiler.PlannerOracle.Plan.successors (__node); (let __fresh : (Prod (List (Compiler.PlannerOracle.Plan.Node)) (List ((Prod (Nat) (Nat))))) := Compiler.PlannerOracle.Plan.fresh ((__r).visited) (__successors); (let __ordered : List (Compiler.PlannerOracle.Plan.Node) := (LexLeanRuntime.append (__rest) ((__fresh).1) : List (Compiler.PlannerOracle.Plan.Node)); (let __next : List (Compiler.PlannerOracle.Plan.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Compiler.PlannerOracle.Plan.Node)) (__n : Compiler.PlannerOracle.Plan.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) ((target + target))) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (Compiler.PlannerOracle.Plan.Node)))) : List (Compiler.PlannerOracle.Plan.Node)) else __acc))) (([] : List (Compiler.PlannerOracle.Plan.Node))) (__ordered) : List (Compiler.PlannerOracle.Plan.Node)); Option.some (({ frontier := __next, visited := (__fresh).2, found := Option.none, truncated := ((__r).truncated || (Nat.blt ((target + target)) ((LexLeanRuntime.length (__ordered) : Nat)))), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + Compiler.PlannerOracle.Plan.attempts ((__node).state)), firings := (((__r).ledger).firings + (LexLeanRuntime.length (__successors) : Nat)), expansions := (((__r).ledger).expansions + 1), verifications := (((__r).ledger).verifications + 1), frontier := (if (Nat.blt (((__r).ledger).frontier) ((LexLeanRuntime.length (__next) : Nat))) then (LexLeanRuntime.length (__next) : Nat) else ((__r).ledger).frontier) } : Compiler.PlannerOracle.Plan.Ledger) } : Compiler.PlannerOracle.Plan.Search)))))))))
@[expose] public def Plan.run (target : Nat) : (Prod (Compiler.PlannerOracle.Plan.Search) (Bool)) := (LexLeanCollections.iterateUntil ((fun (__r : Compiler.PlannerOracle.Plan.Search) => Compiler.PlannerOracle.Plan.searchStep (target) (__r))) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Compiler.PlannerOracle.Plan.start (target)) : (Prod (Compiler.PlannerOracle.Plan.Search) (Bool)))
@[expose] public def Plan.failure (__saturated : Bool) (__truncated : Bool) : (Prod Bool Bool) := (if (__saturated && (!__truncated)) then ((false, true) : Prod Bool Bool) else ((false, false) : Prod Bool Bool))
@[expose] public def Plan (target : Nat) : Except ((Prod Bool Bool)) ((Prod ((Prod (Nat) (Nat))) (List (Compiler.PlannerOracle.Plan.Step)))) := (let __final : (Prod (Compiler.PlannerOracle.Plan.Search) (Bool)) := Compiler.PlannerOracle.Plan.run (target); (match ((__final).1).found with | Option.some __hit => Except.ok (((__hit).1, ((__hit).2).trace)) | Option.none => Except.error (Compiler.PlannerOracle.Plan.failure ((__final).2) (((__final).1).truncated))))
@[expose] public def Plan.verdict (target : Nat) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (match Compiler.PlannerOracle.Plan (target) with | Except.ok __p => Except.ok ((__p).1) | Except.error __e => Except.error (__e))
public theorem Plan.fire_sound (__s : (Prod (Nat) (Nat))) (__step : Compiler.PlannerOracle.Plan.Step) (__t : (Prod (Nat) (Nat))) : ((Compiler.PlannerOracle.Plan.fire (__s) (__step) = Option.some (__t)) -> Compiler.PlannerOracle.Moves (__s) (__t)) :=
by
  cases __step with
  | FillA =>
    exact Compiler.PlannerOracle.FillA.apply_sound __s __t
  | EmptyB =>
    exact Compiler.PlannerOracle.EmptyB.apply_sound __s __t
  | Pour __b =>
    exact Compiler.PlannerOracle.Pour.apply_sound __s __b __t
public theorem Plan.replay_fire (__s : (Prod (Nat) (Nat))) (__step : Compiler.PlannerOracle.Plan.Step) (__t : (Prod (Nat) (Nat))) : ((Compiler.PlannerOracle.Plan.fire (__s) (__step) = Option.some (__t)) -> (Compiler.PlannerOracle.Plan.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [Compiler.PlannerOracle.Plan.replay]
  rw [llE]
public theorem Plan.replay_sound (target : Nat) (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : Compiler.PlannerOracle.Plan.Step) : ((LexLeanReasoning.Reaches ((Compiler.PlannerOracle.Moves)) (Compiler.PlannerOracle.Plan.observe (target)) (__acc)) -> (LexLeanReasoning.Reaches ((Compiler.PlannerOracle.Moves)) (Compiler.PlannerOracle.Plan.observe (target)) (Compiler.PlannerOracle.Plan.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [Compiler.PlannerOracle.Plan.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (Compiler.PlannerOracle.Plan.fire_sound llS __step _ ‹_›)
public theorem Plan.derivation (target : Nat) (__trace : List (Compiler.PlannerOracle.Plan.Step)) : (LexLeanReasoning.Reaches ((Compiler.PlannerOracle.Moves)) (Compiler.PlannerOracle.Plan.observe (target)) (Compiler.PlannerOracle.Plan.follow (target) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((Compiler.PlannerOracle.Plan.replay)) (fun (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) => (LexLeanReasoning.Reaches ((Compiler.PlannerOracle.Moves)) (Compiler.PlannerOracle.Plan.observe (target)) (__acc))) (Compiler.PlannerOracle.Plan.replay_sound (target)) (__trace) (Except.ok (Compiler.PlannerOracle.Plan.observe (target))) (LexLeanReasoning.reachesStart ((Compiler.PlannerOracle.Moves)) (Compiler.PlannerOracle.Plan.observe (target))))
public theorem Plan.follow_invariant (target : Nat) (__trace : List (Compiler.PlannerOracle.Plan.Step)) (__s : (Prod (Nat) (Nat))) : ((Compiler.PlannerOracle.Plan.follow (target) (__trace) = Except.ok (__s)) -> Compiler.PlannerOracle.Fits (__s)) :=
  (fun llE => (LexLeanReasoning.starPreserves ((Compiler.PlannerOracle.Moves)) ((Compiler.PlannerOracle.Fits)) Compiler.PlannerOracle.Pouring.preserves (Compiler.PlannerOracle.Plan.observe (target)) (__s) (Compiler.PlannerOracle.Plan.derivation (target) (__trace) (__s) llE) (Compiler.PlannerOracle.empty_jugs_fit (target))))
public theorem Plan.accept_sound (target : Nat) (__s : (Prod (Nat) (Nat))) (__v : (Prod (Nat) (Nat))) : ((Compiler.PlannerOracle.Plan.accept (target) (__s) = Option.some (__v)) -> Compiler.PlannerOracle.Measures (target) (__v)) :=
by
  intro llE
  dsimp only [Compiler.PlannerOracle.Plan.accept] at llE
  split at llE
  · cases llE
  · have llC := LexLeanReasoning.checked _ _ _ llE
    exact llC.right ▸ Compiler.PlannerOracle.measures_sound _ _ llC.left
public theorem Plan.extend (target : Nat) (__node : Compiler.PlannerOracle.Plan.Node) (__step : Compiler.PlannerOracle.Plan.Step) (__t : (Prod (Nat) (Nat))) : ((Compiler.PlannerOracle.Plan.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> ((Compiler.PlannerOracle.Plan.fire ((__node).state) (__step) = Option.some (__t)) -> (Compiler.PlannerOracle.Plan.follow (target) ((LexLeanRuntime.append ((__node).trace) ((__step :: ([] : List (Compiler.PlannerOracle.Plan.Step)))) : List (Compiler.PlannerOracle.Plan.Step))) = Except.ok (__t)))) :=
by
  intro llH llE
  dsimp only [Compiler.PlannerOracle.Plan.follow]
  rw [LexLeanReasoning.foldSnoc]
  dsimp only [Compiler.PlannerOracle.Plan.follow] at llH
  rw [llH]
  exact Compiler.PlannerOracle.Plan.replay_fire _ __step __t llE
public theorem Plan.FillA.successors_ok (target : Nat) (__node : Compiler.PlannerOracle.Plan.Node) : ((Compiler.PlannerOracle.Plan.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Compiler.PlannerOracle.Plan.Node) => (Compiler.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Compiler.PlannerOracle.Plan.FillA.successors (__node)))) :=
by
  intro llH
  dsimp only [Compiler.PlannerOracle.Plan.FillA.successors]
  split
  · exact LexLeanReasoning.All.nil
  · exact LexLeanReasoning.allSingle _ _ (Compiler.PlannerOracle.Plan.extend _ __node _ _ llH ‹_›)
public theorem Plan.EmptyB.successors_ok (target : Nat) (__node : Compiler.PlannerOracle.Plan.Node) : ((Compiler.PlannerOracle.Plan.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Compiler.PlannerOracle.Plan.Node) => (Compiler.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Compiler.PlannerOracle.Plan.EmptyB.successors (__node)))) :=
by
  intro llH
  dsimp only [Compiler.PlannerOracle.Plan.EmptyB.successors]
  split
  · exact LexLeanReasoning.All.nil
  · exact LexLeanReasoning.allSingle _ _ (Compiler.PlannerOracle.Plan.extend _ __node _ _ llH ‹_›)
public theorem Plan.Pour.successors_ok (target : Nat) (__node : Compiler.PlannerOracle.Plan.Node) : ((Compiler.PlannerOracle.Plan.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Compiler.PlannerOracle.Plan.Node) => (Compiler.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Compiler.PlannerOracle.Plan.Pour.successors (__node)))) :=
by
  intro llH
  dsimp only [Compiler.PlannerOracle.Plan.Pour.successors]
  exact LexLeanReasoning.foldInvariant _ (LexLeanReasoning.All (fun (llN : Compiler.PlannerOracle.Plan.Node) => Compiler.PlannerOracle.Plan.follow _ llN.trace = Except.ok llN.state))
    (fun llA llB llP => by
      dsimp only [Compiler.PlannerOracle.Plan.Pour.collect]
      split
      · exact llP
      · exact LexLeanReasoning.allAppend _ _ _ llP (LexLeanReasoning.allSingle _ _ (Compiler.PlannerOracle.Plan.extend _ __node _ _ llH ‹_›)))
    _ _ LexLeanReasoning.All.nil
public theorem Plan.successors_ok (target : Nat) (__node : Compiler.PlannerOracle.Plan.Node) : ((Compiler.PlannerOracle.Plan.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Compiler.PlannerOracle.Plan.Node) => (Compiler.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Compiler.PlannerOracle.Plan.successors (__node)))) :=
  (fun llH => (LexLeanReasoning.allAppend _ _ _ (Compiler.PlannerOracle.Plan.FillA.successors_ok (target) (__node) llH) (LexLeanReasoning.allAppend _ _ _ (Compiler.PlannerOracle.Plan.EmptyB.successors_ok (target) (__node) llH) (Compiler.PlannerOracle.Plan.Pour.successors_ok (target) (__node) llH))))
public theorem Plan.fresh_ok (target : Nat) (__visited : List ((Prod (Nat) (Nat)))) (__nodes : List (Compiler.PlannerOracle.Plan.Node)) : ((LexLeanReasoning.All ((fun (__n : Compiler.PlannerOracle.Plan.Node) => (Compiler.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (__nodes)) -> (LexLeanReasoning.All ((fun (__n : Compiler.PlannerOracle.Plan.Node) => (Compiler.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((Compiler.PlannerOracle.Plan.fresh (__visited) (__nodes)).1))) :=
  (LexLeanReasoning.freshAll ((fun (__n : Compiler.PlannerOracle.Plan.Node) => (__n).state)) (fun (__n : Compiler.PlannerOracle.Plan.Node) => (Compiler.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state))) (__visited) (__nodes))
public theorem Plan.search_step (target : Nat) (__r : Compiler.PlannerOracle.Plan.Search) (__q : Compiler.PlannerOracle.Plan.Search) : ((Compiler.PlannerOracle.Plan.searchStep (target) (__r) = Option.some (__q)) -> ((LexLeanReasoning.SearchOk ((fun (__n : Compiler.PlannerOracle.Plan.Node) => (Compiler.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Compiler.PlannerOracle.Plan.Node) => Compiler.PlannerOracle.Plan.accept (target) ((__n).state))) ((__r).frontier) ((__r).found)) -> (LexLeanReasoning.SearchOk ((fun (__n : Compiler.PlannerOracle.Plan.Node) => (Compiler.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Compiler.PlannerOracle.Plan.Node) => Compiler.PlannerOracle.Plan.accept (target) ((__n).state))) ((__q).frontier) ((__q).found)))) :=
by
  intro llE llH
  dsimp only [Compiler.PlannerOracle.Plan.searchStep] at llE
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
        exact And.intro (LexLeanReasoning.capAll _ _ _ (LexLeanReasoning.allAppend _ _ _ llRestOk (Compiler.PlannerOracle.Plan.fresh_ok _ _ _ (Compiler.PlannerOracle.Plan.successors_ok _ llNode llNodeOk)))) True.intro
public theorem Plan.search_peak (target : Nat) (__r : Compiler.PlannerOracle.Plan.Search) (__q : Compiler.PlannerOracle.Plan.Search) : ((Compiler.PlannerOracle.Plan.searchStep (target) (__r) = Option.some (__q)) -> ((((__r).ledger).frontier <= (target + target)) -> (((__q).ledger).frontier <= (target + target)))) :=
by
  intro llE llH
  dsimp only [Compiler.PlannerOracle.Plan.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        exact llH
      · cases llE
        exact LexLeanReasoning.peakBound _ _ _ llH (LexLeanReasoning.capBound _ _)
public theorem Plan.search_count (target : Nat) (__r : Compiler.PlannerOracle.Plan.Search) (__q : Compiler.PlannerOracle.Plan.Search) : ((Compiler.PlannerOracle.Plan.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [Compiler.PlannerOracle.Plan.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        rfl
      · cases llE
        rfl
public theorem Plan.search_verify_growth (target : Nat) (__r : Compiler.PlannerOracle.Plan.Search) (__q : Compiler.PlannerOracle.Plan.Search) : ((Compiler.PlannerOracle.Plan.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).verifications <= (((__r).ledger).verifications + 1))) :=
by
  intro llE
  dsimp only [Compiler.PlannerOracle.Plan.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem Plan.search_expanded (target : Nat) (__r : Compiler.PlannerOracle.Plan.Search) (__q : Compiler.PlannerOracle.Plan.Search) : ((Compiler.PlannerOracle.Plan.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).expansions <= (((__r).ledger).expansions + 1))) :=
by
  intro llE
  dsimp only [Compiler.PlannerOracle.Plan.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem Plan.search_ok (target : Nat) : (LexLeanReasoning.SearchOk ((fun (__n : Compiler.PlannerOracle.Plan.Node) => (Compiler.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Compiler.PlannerOracle.Plan.Node) => Compiler.PlannerOracle.Plan.accept (target) ((__n).state))) (((Compiler.PlannerOracle.Plan.run (target)).1).frontier) (((Compiler.PlannerOracle.Plan.run (target)).1).found)) :=
by
  dsimp only [Compiler.PlannerOracle.Plan.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((fun (__r : Compiler.PlannerOracle.Plan.Search) => Compiler.PlannerOracle.Plan.searchStep (target) (__r))) (fun (__r : Compiler.PlannerOracle.Plan.Search) => (LexLeanReasoning.SearchOk ((fun (__n : Compiler.PlannerOracle.Plan.Node) => (Compiler.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Compiler.PlannerOracle.Plan.Node) => Compiler.PlannerOracle.Plan.accept (target) ((__n).state))) ((__r).frontier) ((__r).found))) (Compiler.PlannerOracle.Plan.search_step (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Compiler.PlannerOracle.Plan.start (target)) (LexLeanReasoning.searchStart ((fun (__n : Compiler.PlannerOracle.Plan.Node) => (Compiler.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Compiler.PlannerOracle.Plan.Node) => Compiler.PlannerOracle.Plan.accept (target) ((__n).state))) _ (LexLeanReasoning.capAll ((fun (__n : Compiler.PlannerOracle.Plan.Node) => (Compiler.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((target + target)) _ (LexLeanReasoning.allSingle ((fun (__n : Compiler.PlannerOracle.Plan.Node) => (Compiler.PlannerOracle.Plan.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (({ state := Compiler.PlannerOracle.Plan.observe (target), trace := ([] : List (Compiler.PlannerOracle.Plan.Step)) } : Compiler.PlannerOracle.Plan.Node)) rfl))))
public theorem Plan.frontier_bounded (target : Nat) : ((((Compiler.PlannerOracle.Plan.run (target)).1).ledger).frontier <= (target + target)) :=
by
  dsimp only [Compiler.PlannerOracle.Plan.run]
  exact (LexLeanReasoning.iterateUntilBound ((fun (__r : Compiler.PlannerOracle.Plan.Search) => Compiler.PlannerOracle.Plan.searchStep (target) (__r))) ((fun (__r : Compiler.PlannerOracle.Plan.Search) => ((__r).ledger).frontier)) ((target + target)) (Compiler.PlannerOracle.Plan.search_peak (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Compiler.PlannerOracle.Plan.start (target)) (LexLeanReasoning.capBound ((target + target)) _))
public theorem Plan.verifications_bounded (target : Nat) : ((((Compiler.PlannerOracle.Plan.run (target)).1).ledger).verifications <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [Compiler.PlannerOracle.Plan.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : Compiler.PlannerOracle.Plan.Search) => Compiler.PlannerOracle.Plan.searchStep (target) (__r))) ((fun (__r : Compiler.PlannerOracle.Plan.Search) => ((__r).ledger).verifications)) (Compiler.PlannerOracle.Plan.search_verify_growth (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Compiler.PlannerOracle.Plan.start (target)) rfl)
public theorem Plan.expansions_bounded (target : Nat) : ((((Compiler.PlannerOracle.Plan.run (target)).1).ledger).expansions <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [Compiler.PlannerOracle.Plan.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : Compiler.PlannerOracle.Plan.Search) => Compiler.PlannerOracle.Plan.searchStep (target) (__r))) ((fun (__r : Compiler.PlannerOracle.Plan.Search) => ((__r).ledger).expansions)) (Compiler.PlannerOracle.Plan.search_expanded (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Compiler.PlannerOracle.Plan.start (target)) rfl)
public theorem Plan.iterations_bounded (target : Nat) : ((((Compiler.PlannerOracle.Plan.run (target)).1).ledger).iterations <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [Compiler.PlannerOracle.Plan.run]
  exact (LexLeanReasoning.iterateUntilCount ((fun (__r : Compiler.PlannerOracle.Plan.Search) => Compiler.PlannerOracle.Plan.searchStep (target) (__r))) ((fun (__r : Compiler.PlannerOracle.Plan.Search) => ((__r).ledger).iterations)) (Compiler.PlannerOracle.Plan.search_count (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Compiler.PlannerOracle.Plan.start (target)) rfl)
public theorem Plan.explained (target : Nat) : (forall (__v : (Prod (Nat) (Nat))), (forall (__trace : List (Compiler.PlannerOracle.Plan.Step)), ((Compiler.PlannerOracle.Plan (target) = Except.ok ((__v, __trace))) -> ((Compiler.PlannerOracle.Plan.answer (target) (__trace) = Option.some (__v)) /\ Compiler.PlannerOracle.Measures (target) (__v))))) :=
by
  intro llV llT llE
  have llSearch := Compiler.PlannerOracle.Plan.search_ok target
  dsimp only [Compiler.PlannerOracle.Plan] at llE
  generalize llRun : Compiler.PlannerOracle.Plan.run target = llR at llE llSearch
  split at llE
  · rename_i llHit llF
    cases llE
    have llOk := And.right llSearch
    rw [llF] at llOk
    exact And.intro (by dsimp only [Compiler.PlannerOracle.Plan.answer]; rw [And.left llOk]; exact And.right llOk) (Compiler.PlannerOracle.Plan.accept_sound _ _ _ (And.right llOk))
  · cases llE
public theorem Plan.verdict_sound (target : Nat) : (forall (__v : (Prod (Nat) (Nat))), ((Compiler.PlannerOracle.Plan.verdict (target) = Except.ok (__v)) -> Compiler.PlannerOracle.Measures (target) (__v))) :=
by
  intro llV llE
  dsimp only [Compiler.PlannerOracle.Plan.verdict] at llE
  generalize llHP : Compiler.PlannerOracle.Plan target = llR at llE
  cases llR with
  | error _ => cases llE
  | ok llP =>
    cases llE
    exact And.right (Compiler.PlannerOracle.Plan.explained _ llP.1 llP.2 llHP)

@[expose, reducible] public def Within (_target : Nat) (r : Nat) : Prop := (r <= 4)

@[expose] public def withinCheck (_target : Nat) (r : Nat) : Bool := (Nat.ble (r) (4))

public theorem within_sound (target : Nat) (r : Nat) : ((withinCheck (target) (r) = true) -> Within (target) (r)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Within, withinCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem Inside.sound : (LexLeanReasoning.Sound ((Compiler.PlannerOracle.withinCheck)) ((Compiler.PlannerOracle.Within))) :=
  Compiler.PlannerOracle.within_sound

@[expose, reducible] public def Whole (_target : Nat) (r : Nat) : Prop := (0 <= r)

@[expose] public def wholeCheck (_target : Nat) (r : Nat) : Bool := (Nat.ble (0) (r))

public theorem whole_sound (target : Nat) (r : Nat) : ((wholeCheck (target) (r) = true) -> Whole (target) (r)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Whole, wholeCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem Anywhere.sound : (LexLeanReasoning.Sound ((Compiler.PlannerOracle.wholeCheck)) ((Compiler.PlannerOracle.Whole))) :=
  Compiler.PlannerOracle.whole_sound

public theorem plan_fit_correct (target : Nat) (s : (Prod (Nat) (Nat))) (v : Nat) : (Fits (s) -> ((Option.some ((s).1) = Option.some (v)) -> Within (target) (v))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Fits, Within, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose, reducible] public def Held (target : Nat) (s : (Prod (Nat) (Nat))) : Prop := (Fits (s) /\ (((s).1 = target) -> (target <= 4)))

public theorem held_initial (target : Nat) : Held (target) ((0, 0)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Fits, Held, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem held_preserved (target : Nat) (s : (Prod (Nat) (Nat))) (t : (Prod (Nat) (Nat))) : (Held (target) (s) -> (Moves (s) (t) -> Held (target) (t))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Fits, Held, Moves, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem plan_held_correct (target : Nat) (s : (Prod (Nat) (Nat))) (v : Nat) : (Held (target) (s) -> ((Option.some ((s).1) = Option.some (v)) -> Within (target) (v))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Fits, Held, Within, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem plan_whole_correct (target : Nat) (s : (Prod (Nat) (Nat))) (v : Nat) : ((Option.some ((s).1) = Option.some (v)) -> Whole (target) (v)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Whole, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public inductive PlanFit.Step where
  | FillA
  | EmptyB
  | Pour (_ : Nat)
public structure PlanFit.Ledger where
  iterations : Nat
  attempts : Nat
  firings : Nat
  expansions : Nat
  verifications : Nat
  frontier : Nat
@[expose] public def PlanFit.observe (_target : Nat) : (Prod (Nat) (Nat)) := (0, 0)
@[expose] public def PlanFit.fire (__s : (Prod (Nat) (Nat))) (__step : Compiler.PlannerOracle.PlanFit.Step) : Option ((Prod (Nat) (Nat))) := (match __step with | Compiler.PlannerOracle.PlanFit.Step.FillA => Compiler.PlannerOracle.FillA.apply (__s) | Compiler.PlannerOracle.PlanFit.Step.EmptyB => Compiler.PlannerOracle.EmptyB.apply (__s) | Compiler.PlannerOracle.PlanFit.Step.Pour __b => Compiler.PlannerOracle.Pour.apply (__s) (__b))
@[expose] public def PlanFit.replay (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : Compiler.PlannerOracle.PlanFit.Step) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match Compiler.PlannerOracle.PlanFit.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def PlanFit.follow (target : Nat) (__trace : List (Compiler.PlannerOracle.PlanFit.Step)) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (LexLeanCollections.listFold ((Compiler.PlannerOracle.PlanFit.replay)) (Except.ok (Compiler.PlannerOracle.PlanFit.observe (target))) (__trace) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))))
@[expose] public def PlanFit.extract (jugs : (Prod (Nat) (Nat))) : Option (Nat) := Option.some ((jugs).1)
@[expose] public def PlanFit.accept (_target : Nat) (__s : (Prod (Nat) (Nat))) : Option (Nat) := Compiler.PlannerOracle.PlanFit.extract (__s)
@[expose] public def PlanFit.answer (target : Nat) (__trace : List (Compiler.PlannerOracle.PlanFit.Step)) : Option (Nat) := (match Compiler.PlannerOracle.PlanFit.follow (target) (__trace) with | Except.ok __s => Compiler.PlannerOracle.PlanFit.accept (target) (__s) | Except.error _ => Option.none)
public structure PlanFit.Node where
  state : (Prod (Nat) (Nat))
  trace : List (Compiler.PlannerOracle.PlanFit.Step)
public structure PlanFit.Search where
  frontier : List (Compiler.PlannerOracle.PlanFit.Node)
  visited : List ((Prod (Nat) (Nat)))
  found : Option ((Prod (Nat) (Compiler.PlannerOracle.PlanFit.Node)))
  truncated : Bool
  ledger : Compiler.PlannerOracle.PlanFit.Ledger
@[expose] public def PlanFit.FillA.successors (__node : Compiler.PlannerOracle.PlanFit.Node) : List (Compiler.PlannerOracle.PlanFit.Node) := (match Compiler.PlannerOracle.PlanFit.fire ((__node).state) (Compiler.PlannerOracle.PlanFit.Step.FillA) with | Option.none => ([] : List (Compiler.PlannerOracle.PlanFit.Node)) | Option.some __t => (({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((Compiler.PlannerOracle.PlanFit.Step.FillA :: ([] : List (Compiler.PlannerOracle.PlanFit.Step)))) : List (Compiler.PlannerOracle.PlanFit.Step)) } : Compiler.PlannerOracle.PlanFit.Node) :: ([] : List (Compiler.PlannerOracle.PlanFit.Node))))
@[expose] public def PlanFit.EmptyB.successors (__node : Compiler.PlannerOracle.PlanFit.Node) : List (Compiler.PlannerOracle.PlanFit.Node) := (match Compiler.PlannerOracle.PlanFit.fire ((__node).state) (Compiler.PlannerOracle.PlanFit.Step.EmptyB) with | Option.none => ([] : List (Compiler.PlannerOracle.PlanFit.Node)) | Option.some __t => (({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((Compiler.PlannerOracle.PlanFit.Step.EmptyB :: ([] : List (Compiler.PlannerOracle.PlanFit.Step)))) : List (Compiler.PlannerOracle.PlanFit.Step)) } : Compiler.PlannerOracle.PlanFit.Node) :: ([] : List (Compiler.PlannerOracle.PlanFit.Node))))
@[expose] public def PlanFit.Pour.collect (__node : Compiler.PlannerOracle.PlanFit.Node) (__acc : List (Compiler.PlannerOracle.PlanFit.Node)) (__b : Nat) : List (Compiler.PlannerOracle.PlanFit.Node) := (match Compiler.PlannerOracle.PlanFit.fire ((__node).state) (Compiler.PlannerOracle.PlanFit.Step.Pour (__b)) with | Option.none => __acc | Option.some __t => (LexLeanRuntime.append (__acc) ((({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((Compiler.PlannerOracle.PlanFit.Step.Pour (__b) :: ([] : List (Compiler.PlannerOracle.PlanFit.Step)))) : List (Compiler.PlannerOracle.PlanFit.Step)) } : Compiler.PlannerOracle.PlanFit.Node) :: ([] : List (Compiler.PlannerOracle.PlanFit.Node)))) : List (Compiler.PlannerOracle.PlanFit.Node)))
@[expose] public def PlanFit.Pour.successors (__node : Compiler.PlannerOracle.PlanFit.Node) : List (Compiler.PlannerOracle.PlanFit.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Compiler.PlannerOracle.PlanFit.Node)) (__b : Nat) => Compiler.PlannerOracle.PlanFit.Pour.collect (__node) (__acc) (__b))) (([] : List (Compiler.PlannerOracle.PlanFit.Node))) (Compiler.PlannerOracle.Pour.candidates ((__node).state)) : List (Compiler.PlannerOracle.PlanFit.Node))
@[expose] public def PlanFit.successors (__node : Compiler.PlannerOracle.PlanFit.Node) : List (Compiler.PlannerOracle.PlanFit.Node) := (LexLeanRuntime.append (Compiler.PlannerOracle.PlanFit.FillA.successors (__node)) ((LexLeanRuntime.append (Compiler.PlannerOracle.PlanFit.EmptyB.successors (__node)) (Compiler.PlannerOracle.PlanFit.Pour.successors (__node)) : List (Compiler.PlannerOracle.PlanFit.Node))) : List (Compiler.PlannerOracle.PlanFit.Node))
@[expose] public def PlanFit.fresh (__visited : List ((Prod (Nat) (Nat)))) (__nodes : List (Compiler.PlannerOracle.PlanFit.Node)) : (Prod (List (Compiler.PlannerOracle.PlanFit.Node)) (List ((Prod (Nat) (Nat))))) := (LexLeanCollections.listFold ((fun (__acc : (Prod (List (Compiler.PlannerOracle.PlanFit.Node)) (List ((Prod (Nat) (Nat)))))) (__n : Compiler.PlannerOracle.PlanFit.Node) => (if (LexLeanCollections.setContains ((__acc).2) ((__n).state) : Bool) then __acc else ((LexLeanRuntime.append ((__acc).1) ((__n :: ([] : List (Compiler.PlannerOracle.PlanFit.Node)))) : List (Compiler.PlannerOracle.PlanFit.Node)), (LexLeanCollections.setInsert ((__acc).2) ((__n).state) : List ((Prod (Nat) (Nat)))))))) ((([] : List (Compiler.PlannerOracle.PlanFit.Node)), __visited)) (__nodes) : (Prod (List (Compiler.PlannerOracle.PlanFit.Node)) (List ((Prod (Nat) (Nat))))))
@[expose] public def PlanFit.start (target : Nat) : Compiler.PlannerOracle.PlanFit.Search := (let __first : List (Compiler.PlannerOracle.PlanFit.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Compiler.PlannerOracle.PlanFit.Node)) (__n : Compiler.PlannerOracle.PlanFit.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) ((target + target))) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (Compiler.PlannerOracle.PlanFit.Node)))) : List (Compiler.PlannerOracle.PlanFit.Node)) else __acc))) (([] : List (Compiler.PlannerOracle.PlanFit.Node))) ((({ state := Compiler.PlannerOracle.PlanFit.observe (target), trace := ([] : List (Compiler.PlannerOracle.PlanFit.Step)) } : Compiler.PlannerOracle.PlanFit.Node) :: ([] : List (Compiler.PlannerOracle.PlanFit.Node)))) : List (Compiler.PlannerOracle.PlanFit.Node)); ({ frontier := __first, visited := (LexLeanCollections.setInsert (([] : List ((Prod (Nat) (Nat))))) (Compiler.PlannerOracle.PlanFit.observe (target)) : List ((Prod (Nat) (Nat)))), found := Option.none, truncated := (Nat.blt ((target + target)) (1)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := (LexLeanRuntime.length (__first) : Nat) } : Compiler.PlannerOracle.PlanFit.Ledger) } : Compiler.PlannerOracle.PlanFit.Search))
@[expose] public def PlanFit.attempts (__s : (Prod (Nat) (Nat))) : Nat := (1 + (1 + (LexLeanRuntime.length (Compiler.PlannerOracle.Pour.candidates (__s)) : Nat)))
@[expose] public def PlanFit.searchStep (target : Nat) (__r : Compiler.PlannerOracle.PlanFit.Search) : Option (Compiler.PlannerOracle.PlanFit.Search) := (match (__r).found with | Option.some _ => Option.none | Option.none => (match (__r).frontier with | List.nil => Option.none | List.cons __node __rest => (match Compiler.PlannerOracle.PlanFit.accept (target) ((__node).state) with | Option.some __v => Option.some (({ frontier := __rest, visited := (__r).visited, found := Option.some ((__v, __node)), truncated := (__r).truncated, ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := ((__r).ledger).attempts, firings := ((__r).ledger).firings, expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : Compiler.PlannerOracle.PlanFit.Ledger) } : Compiler.PlannerOracle.PlanFit.Search)) | Option.none => (let __successors : List (Compiler.PlannerOracle.PlanFit.Node) := Compiler.PlannerOracle.PlanFit.successors (__node); (let __fresh : (Prod (List (Compiler.PlannerOracle.PlanFit.Node)) (List ((Prod (Nat) (Nat))))) := Compiler.PlannerOracle.PlanFit.fresh ((__r).visited) (__successors); (let __ordered : List (Compiler.PlannerOracle.PlanFit.Node) := (LexLeanRuntime.append (__rest) ((__fresh).1) : List (Compiler.PlannerOracle.PlanFit.Node)); (let __next : List (Compiler.PlannerOracle.PlanFit.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Compiler.PlannerOracle.PlanFit.Node)) (__n : Compiler.PlannerOracle.PlanFit.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) ((target + target))) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (Compiler.PlannerOracle.PlanFit.Node)))) : List (Compiler.PlannerOracle.PlanFit.Node)) else __acc))) (([] : List (Compiler.PlannerOracle.PlanFit.Node))) (__ordered) : List (Compiler.PlannerOracle.PlanFit.Node)); Option.some (({ frontier := __next, visited := (__fresh).2, found := Option.none, truncated := ((__r).truncated || (Nat.blt ((target + target)) ((LexLeanRuntime.length (__ordered) : Nat)))), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + Compiler.PlannerOracle.PlanFit.attempts ((__node).state)), firings := (((__r).ledger).firings + (LexLeanRuntime.length (__successors) : Nat)), expansions := (((__r).ledger).expansions + 1), verifications := ((__r).ledger).verifications, frontier := (if (Nat.blt (((__r).ledger).frontier) ((LexLeanRuntime.length (__next) : Nat))) then (LexLeanRuntime.length (__next) : Nat) else ((__r).ledger).frontier) } : Compiler.PlannerOracle.PlanFit.Ledger) } : Compiler.PlannerOracle.PlanFit.Search)))))))))
@[expose] public def PlanFit.run (target : Nat) : (Prod (Compiler.PlannerOracle.PlanFit.Search) (Bool)) := (LexLeanCollections.iterateUntil ((fun (__r : Compiler.PlannerOracle.PlanFit.Search) => Compiler.PlannerOracle.PlanFit.searchStep (target) (__r))) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Compiler.PlannerOracle.PlanFit.start (target)) : (Prod (Compiler.PlannerOracle.PlanFit.Search) (Bool)))
@[expose] public def PlanFit.failure (__saturated : Bool) (__truncated : Bool) : (Prod Bool Bool) := (if (__saturated && (!__truncated)) then ((false, true) : Prod Bool Bool) else ((false, false) : Prod Bool Bool))
@[expose] public def PlanFit (target : Nat) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (Compiler.PlannerOracle.PlanFit.Step)))) := (let __final : (Prod (Compiler.PlannerOracle.PlanFit.Search) (Bool)) := Compiler.PlannerOracle.PlanFit.run (target); (match ((__final).1).found with | Option.some __hit => Except.ok (((__hit).1, ((__hit).2).trace)) | Option.none => Except.error (Compiler.PlannerOracle.PlanFit.failure ((__final).2) (((__final).1).truncated))))
@[expose] public def PlanFit.verdict (target : Nat) : Except ((Prod Bool Bool)) (Nat) := (match Compiler.PlannerOracle.PlanFit (target) with | Except.ok __p => Except.ok ((__p).1) | Except.error __e => Except.error (__e))
public theorem PlanFit.fire_sound (__s : (Prod (Nat) (Nat))) (__step : Compiler.PlannerOracle.PlanFit.Step) (__t : (Prod (Nat) (Nat))) : ((Compiler.PlannerOracle.PlanFit.fire (__s) (__step) = Option.some (__t)) -> Compiler.PlannerOracle.Moves (__s) (__t)) :=
by
  cases __step with
  | FillA =>
    exact Compiler.PlannerOracle.FillA.apply_sound __s __t
  | EmptyB =>
    exact Compiler.PlannerOracle.EmptyB.apply_sound __s __t
  | Pour __b =>
    exact Compiler.PlannerOracle.Pour.apply_sound __s __b __t
public theorem PlanFit.replay_fire (__s : (Prod (Nat) (Nat))) (__step : Compiler.PlannerOracle.PlanFit.Step) (__t : (Prod (Nat) (Nat))) : ((Compiler.PlannerOracle.PlanFit.fire (__s) (__step) = Option.some (__t)) -> (Compiler.PlannerOracle.PlanFit.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [Compiler.PlannerOracle.PlanFit.replay]
  rw [llE]
public theorem PlanFit.replay_sound (target : Nat) (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : Compiler.PlannerOracle.PlanFit.Step) : ((LexLeanReasoning.Reaches ((Compiler.PlannerOracle.Moves)) (Compiler.PlannerOracle.PlanFit.observe (target)) (__acc)) -> (LexLeanReasoning.Reaches ((Compiler.PlannerOracle.Moves)) (Compiler.PlannerOracle.PlanFit.observe (target)) (Compiler.PlannerOracle.PlanFit.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [Compiler.PlannerOracle.PlanFit.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (Compiler.PlannerOracle.PlanFit.fire_sound llS __step _ ‹_›)
public theorem PlanFit.derivation (target : Nat) (__trace : List (Compiler.PlannerOracle.PlanFit.Step)) : (LexLeanReasoning.Reaches ((Compiler.PlannerOracle.Moves)) (Compiler.PlannerOracle.PlanFit.observe (target)) (Compiler.PlannerOracle.PlanFit.follow (target) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((Compiler.PlannerOracle.PlanFit.replay)) (fun (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) => (LexLeanReasoning.Reaches ((Compiler.PlannerOracle.Moves)) (Compiler.PlannerOracle.PlanFit.observe (target)) (__acc))) (Compiler.PlannerOracle.PlanFit.replay_sound (target)) (__trace) (Except.ok (Compiler.PlannerOracle.PlanFit.observe (target))) (LexLeanReasoning.reachesStart ((Compiler.PlannerOracle.Moves)) (Compiler.PlannerOracle.PlanFit.observe (target))))
public theorem PlanFit.follow_invariant (target : Nat) (__trace : List (Compiler.PlannerOracle.PlanFit.Step)) (__s : (Prod (Nat) (Nat))) : ((Compiler.PlannerOracle.PlanFit.follow (target) (__trace) = Except.ok (__s)) -> Compiler.PlannerOracle.Fits (__s)) :=
  (fun llE => (LexLeanReasoning.starPreserves ((Compiler.PlannerOracle.Moves)) ((Compiler.PlannerOracle.Fits)) Compiler.PlannerOracle.Pouring.preserves (Compiler.PlannerOracle.PlanFit.observe (target)) (__s) (Compiler.PlannerOracle.PlanFit.derivation (target) (__trace) (__s) llE) (Compiler.PlannerOracle.empty_jugs_fit (target))))
public theorem PlanFit.accept_sound (target : Nat) (__s : (Prod (Nat) (Nat))) (__v : Nat) : (Compiler.PlannerOracle.Fits (__s) -> ((Compiler.PlannerOracle.PlanFit.accept (target) (__s) = Option.some (__v)) -> Compiler.PlannerOracle.Within (target) (__v))) :=
  (fun llJ llE => (Compiler.PlannerOracle.plan_fit_correct (target) (__s) (__v) llJ llE))
public theorem PlanFit.extend (target : Nat) (__node : Compiler.PlannerOracle.PlanFit.Node) (__step : Compiler.PlannerOracle.PlanFit.Step) (__t : (Prod (Nat) (Nat))) : ((Compiler.PlannerOracle.PlanFit.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> ((Compiler.PlannerOracle.PlanFit.fire ((__node).state) (__step) = Option.some (__t)) -> (Compiler.PlannerOracle.PlanFit.follow (target) ((LexLeanRuntime.append ((__node).trace) ((__step :: ([] : List (Compiler.PlannerOracle.PlanFit.Step)))) : List (Compiler.PlannerOracle.PlanFit.Step))) = Except.ok (__t)))) :=
by
  intro llH llE
  dsimp only [Compiler.PlannerOracle.PlanFit.follow]
  rw [LexLeanReasoning.foldSnoc]
  dsimp only [Compiler.PlannerOracle.PlanFit.follow] at llH
  rw [llH]
  exact Compiler.PlannerOracle.PlanFit.replay_fire _ __step __t llE
public theorem PlanFit.FillA.successors_ok (target : Nat) (__node : Compiler.PlannerOracle.PlanFit.Node) : ((Compiler.PlannerOracle.PlanFit.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Compiler.PlannerOracle.PlanFit.Node) => (Compiler.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Compiler.PlannerOracle.PlanFit.FillA.successors (__node)))) :=
by
  intro llH
  dsimp only [Compiler.PlannerOracle.PlanFit.FillA.successors]
  split
  · exact LexLeanReasoning.All.nil
  · exact LexLeanReasoning.allSingle _ _ (Compiler.PlannerOracle.PlanFit.extend _ __node _ _ llH ‹_›)
public theorem PlanFit.EmptyB.successors_ok (target : Nat) (__node : Compiler.PlannerOracle.PlanFit.Node) : ((Compiler.PlannerOracle.PlanFit.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Compiler.PlannerOracle.PlanFit.Node) => (Compiler.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Compiler.PlannerOracle.PlanFit.EmptyB.successors (__node)))) :=
by
  intro llH
  dsimp only [Compiler.PlannerOracle.PlanFit.EmptyB.successors]
  split
  · exact LexLeanReasoning.All.nil
  · exact LexLeanReasoning.allSingle _ _ (Compiler.PlannerOracle.PlanFit.extend _ __node _ _ llH ‹_›)
public theorem PlanFit.Pour.successors_ok (target : Nat) (__node : Compiler.PlannerOracle.PlanFit.Node) : ((Compiler.PlannerOracle.PlanFit.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Compiler.PlannerOracle.PlanFit.Node) => (Compiler.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Compiler.PlannerOracle.PlanFit.Pour.successors (__node)))) :=
by
  intro llH
  dsimp only [Compiler.PlannerOracle.PlanFit.Pour.successors]
  exact LexLeanReasoning.foldInvariant _ (LexLeanReasoning.All (fun (llN : Compiler.PlannerOracle.PlanFit.Node) => Compiler.PlannerOracle.PlanFit.follow _ llN.trace = Except.ok llN.state))
    (fun llA llB llP => by
      dsimp only [Compiler.PlannerOracle.PlanFit.Pour.collect]
      split
      · exact llP
      · exact LexLeanReasoning.allAppend _ _ _ llP (LexLeanReasoning.allSingle _ _ (Compiler.PlannerOracle.PlanFit.extend _ __node _ _ llH ‹_›)))
    _ _ LexLeanReasoning.All.nil
public theorem PlanFit.successors_ok (target : Nat) (__node : Compiler.PlannerOracle.PlanFit.Node) : ((Compiler.PlannerOracle.PlanFit.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Compiler.PlannerOracle.PlanFit.Node) => (Compiler.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Compiler.PlannerOracle.PlanFit.successors (__node)))) :=
  (fun llH => (LexLeanReasoning.allAppend _ _ _ (Compiler.PlannerOracle.PlanFit.FillA.successors_ok (target) (__node) llH) (LexLeanReasoning.allAppend _ _ _ (Compiler.PlannerOracle.PlanFit.EmptyB.successors_ok (target) (__node) llH) (Compiler.PlannerOracle.PlanFit.Pour.successors_ok (target) (__node) llH))))
public theorem PlanFit.fresh_ok (target : Nat) (__visited : List ((Prod (Nat) (Nat)))) (__nodes : List (Compiler.PlannerOracle.PlanFit.Node)) : ((LexLeanReasoning.All ((fun (__n : Compiler.PlannerOracle.PlanFit.Node) => (Compiler.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (__nodes)) -> (LexLeanReasoning.All ((fun (__n : Compiler.PlannerOracle.PlanFit.Node) => (Compiler.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((Compiler.PlannerOracle.PlanFit.fresh (__visited) (__nodes)).1))) :=
  (LexLeanReasoning.freshAll ((fun (__n : Compiler.PlannerOracle.PlanFit.Node) => (__n).state)) (fun (__n : Compiler.PlannerOracle.PlanFit.Node) => (Compiler.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state))) (__visited) (__nodes))
public theorem PlanFit.search_step (target : Nat) (__r : Compiler.PlannerOracle.PlanFit.Search) (__q : Compiler.PlannerOracle.PlanFit.Search) : ((Compiler.PlannerOracle.PlanFit.searchStep (target) (__r) = Option.some (__q)) -> ((LexLeanReasoning.SearchOk ((fun (__n : Compiler.PlannerOracle.PlanFit.Node) => (Compiler.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Compiler.PlannerOracle.PlanFit.Node) => Compiler.PlannerOracle.PlanFit.accept (target) ((__n).state))) ((__r).frontier) ((__r).found)) -> (LexLeanReasoning.SearchOk ((fun (__n : Compiler.PlannerOracle.PlanFit.Node) => (Compiler.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Compiler.PlannerOracle.PlanFit.Node) => Compiler.PlannerOracle.PlanFit.accept (target) ((__n).state))) ((__q).frontier) ((__q).found)))) :=
by
  intro llE llH
  dsimp only [Compiler.PlannerOracle.PlanFit.searchStep] at llE
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
        exact And.intro (LexLeanReasoning.capAll _ _ _ (LexLeanReasoning.allAppend _ _ _ llRestOk (Compiler.PlannerOracle.PlanFit.fresh_ok _ _ _ (Compiler.PlannerOracle.PlanFit.successors_ok _ llNode llNodeOk)))) True.intro
public theorem PlanFit.search_peak (target : Nat) (__r : Compiler.PlannerOracle.PlanFit.Search) (__q : Compiler.PlannerOracle.PlanFit.Search) : ((Compiler.PlannerOracle.PlanFit.searchStep (target) (__r) = Option.some (__q)) -> ((((__r).ledger).frontier <= (target + target)) -> (((__q).ledger).frontier <= (target + target)))) :=
by
  intro llE llH
  dsimp only [Compiler.PlannerOracle.PlanFit.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        exact llH
      · cases llE
        exact LexLeanReasoning.peakBound _ _ _ llH (LexLeanReasoning.capBound _ _)
public theorem PlanFit.search_count (target : Nat) (__r : Compiler.PlannerOracle.PlanFit.Search) (__q : Compiler.PlannerOracle.PlanFit.Search) : ((Compiler.PlannerOracle.PlanFit.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [Compiler.PlannerOracle.PlanFit.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        rfl
      · cases llE
        rfl
public theorem PlanFit.search_verify_growth (target : Nat) (__r : Compiler.PlannerOracle.PlanFit.Search) (__q : Compiler.PlannerOracle.PlanFit.Search) : ((Compiler.PlannerOracle.PlanFit.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).verifications <= (((__r).ledger).verifications + 1))) :=
by
  intro llE
  dsimp only [Compiler.PlannerOracle.PlanFit.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem PlanFit.search_expanded (target : Nat) (__r : Compiler.PlannerOracle.PlanFit.Search) (__q : Compiler.PlannerOracle.PlanFit.Search) : ((Compiler.PlannerOracle.PlanFit.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).expansions <= (((__r).ledger).expansions + 1))) :=
by
  intro llE
  dsimp only [Compiler.PlannerOracle.PlanFit.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem PlanFit.search_ok (target : Nat) : (LexLeanReasoning.SearchOk ((fun (__n : Compiler.PlannerOracle.PlanFit.Node) => (Compiler.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Compiler.PlannerOracle.PlanFit.Node) => Compiler.PlannerOracle.PlanFit.accept (target) ((__n).state))) (((Compiler.PlannerOracle.PlanFit.run (target)).1).frontier) (((Compiler.PlannerOracle.PlanFit.run (target)).1).found)) :=
by
  dsimp only [Compiler.PlannerOracle.PlanFit.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((fun (__r : Compiler.PlannerOracle.PlanFit.Search) => Compiler.PlannerOracle.PlanFit.searchStep (target) (__r))) (fun (__r : Compiler.PlannerOracle.PlanFit.Search) => (LexLeanReasoning.SearchOk ((fun (__n : Compiler.PlannerOracle.PlanFit.Node) => (Compiler.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Compiler.PlannerOracle.PlanFit.Node) => Compiler.PlannerOracle.PlanFit.accept (target) ((__n).state))) ((__r).frontier) ((__r).found))) (Compiler.PlannerOracle.PlanFit.search_step (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Compiler.PlannerOracle.PlanFit.start (target)) (LexLeanReasoning.searchStart ((fun (__n : Compiler.PlannerOracle.PlanFit.Node) => (Compiler.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Compiler.PlannerOracle.PlanFit.Node) => Compiler.PlannerOracle.PlanFit.accept (target) ((__n).state))) _ (LexLeanReasoning.capAll ((fun (__n : Compiler.PlannerOracle.PlanFit.Node) => (Compiler.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((target + target)) _ (LexLeanReasoning.allSingle ((fun (__n : Compiler.PlannerOracle.PlanFit.Node) => (Compiler.PlannerOracle.PlanFit.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (({ state := Compiler.PlannerOracle.PlanFit.observe (target), trace := ([] : List (Compiler.PlannerOracle.PlanFit.Step)) } : Compiler.PlannerOracle.PlanFit.Node)) rfl))))
public theorem PlanFit.frontier_bounded (target : Nat) : ((((Compiler.PlannerOracle.PlanFit.run (target)).1).ledger).frontier <= (target + target)) :=
by
  dsimp only [Compiler.PlannerOracle.PlanFit.run]
  exact (LexLeanReasoning.iterateUntilBound ((fun (__r : Compiler.PlannerOracle.PlanFit.Search) => Compiler.PlannerOracle.PlanFit.searchStep (target) (__r))) ((fun (__r : Compiler.PlannerOracle.PlanFit.Search) => ((__r).ledger).frontier)) ((target + target)) (Compiler.PlannerOracle.PlanFit.search_peak (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Compiler.PlannerOracle.PlanFit.start (target)) (LexLeanReasoning.capBound ((target + target)) _))
public theorem PlanFit.verifications_bounded (target : Nat) : ((((Compiler.PlannerOracle.PlanFit.run (target)).1).ledger).verifications <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [Compiler.PlannerOracle.PlanFit.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : Compiler.PlannerOracle.PlanFit.Search) => Compiler.PlannerOracle.PlanFit.searchStep (target) (__r))) ((fun (__r : Compiler.PlannerOracle.PlanFit.Search) => ((__r).ledger).verifications)) (Compiler.PlannerOracle.PlanFit.search_verify_growth (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Compiler.PlannerOracle.PlanFit.start (target)) rfl)
public theorem PlanFit.expansions_bounded (target : Nat) : ((((Compiler.PlannerOracle.PlanFit.run (target)).1).ledger).expansions <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [Compiler.PlannerOracle.PlanFit.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : Compiler.PlannerOracle.PlanFit.Search) => Compiler.PlannerOracle.PlanFit.searchStep (target) (__r))) ((fun (__r : Compiler.PlannerOracle.PlanFit.Search) => ((__r).ledger).expansions)) (Compiler.PlannerOracle.PlanFit.search_expanded (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Compiler.PlannerOracle.PlanFit.start (target)) rfl)
public theorem PlanFit.iterations_bounded (target : Nat) : ((((Compiler.PlannerOracle.PlanFit.run (target)).1).ledger).iterations <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [Compiler.PlannerOracle.PlanFit.run]
  exact (LexLeanReasoning.iterateUntilCount ((fun (__r : Compiler.PlannerOracle.PlanFit.Search) => Compiler.PlannerOracle.PlanFit.searchStep (target) (__r))) ((fun (__r : Compiler.PlannerOracle.PlanFit.Search) => ((__r).ledger).iterations)) (Compiler.PlannerOracle.PlanFit.search_count (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Compiler.PlannerOracle.PlanFit.start (target)) rfl)
public theorem PlanFit.explained (target : Nat) : (forall (__v : Nat), (forall (__trace : List (Compiler.PlannerOracle.PlanFit.Step)), ((Compiler.PlannerOracle.PlanFit (target) = Except.ok ((__v, __trace))) -> ((Compiler.PlannerOracle.PlanFit.answer (target) (__trace) = Option.some (__v)) /\ Compiler.PlannerOracle.Within (target) (__v))))) :=
by
  intro llV llT llE
  have llSearch := Compiler.PlannerOracle.PlanFit.search_ok target
  dsimp only [Compiler.PlannerOracle.PlanFit] at llE
  generalize llRun : Compiler.PlannerOracle.PlanFit.run target = llR at llE llSearch
  split at llE
  · rename_i llHit llF
    cases llE
    have llOk := And.right llSearch
    rw [llF] at llOk
    exact And.intro (by dsimp only [Compiler.PlannerOracle.PlanFit.answer]; rw [And.left llOk]; exact And.right llOk) (Compiler.PlannerOracle.PlanFit.accept_sound _ _ _ (Compiler.PlannerOracle.PlanFit.follow_invariant target _ _ (And.left llOk)) (And.right llOk))
  · cases llE
public theorem PlanFit.verdict_sound (target : Nat) : (forall (__v : Nat), ((Compiler.PlannerOracle.PlanFit.verdict (target) = Except.ok (__v)) -> Compiler.PlannerOracle.Within (target) (__v))) :=
by
  intro llV llE
  dsimp only [Compiler.PlannerOracle.PlanFit.verdict] at llE
  generalize llHP : Compiler.PlannerOracle.PlanFit target = llR at llE
  cases llR with
  | error _ => cases llE
  | ok llP =>
    cases llE
    exact And.right (Compiler.PlannerOracle.PlanFit.explained _ llP.1 llP.2 llHP)

public inductive PlanFitR.Step where
  | FillA
  | EmptyB
  | Pour (_ : Nat)
public structure PlanFitR.Ledger where
  iterations : Nat
  attempts : Nat
  firings : Nat
  expansions : Nat
  verifications : Nat
  frontier : Nat
@[expose] public def PlanFitR.observe (_target : Nat) : (Prod (Nat) (Nat)) := (0, 0)
@[expose] public def PlanFitR.fire (__s : (Prod (Nat) (Nat))) (__step : Compiler.PlannerOracle.PlanFitR.Step) : Option ((Prod (Nat) (Nat))) := (match __step with | Compiler.PlannerOracle.PlanFitR.Step.FillA => Compiler.PlannerOracle.FillA.apply (__s) | Compiler.PlannerOracle.PlanFitR.Step.EmptyB => Compiler.PlannerOracle.EmptyB.apply (__s) | Compiler.PlannerOracle.PlanFitR.Step.Pour __b => Compiler.PlannerOracle.Pour.apply (__s) (__b))
@[expose] public def PlanFitR.replay (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : Compiler.PlannerOracle.PlanFitR.Step) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match Compiler.PlannerOracle.PlanFitR.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def PlanFitR.follow (target : Nat) (__trace : List (Compiler.PlannerOracle.PlanFitR.Step)) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (LexLeanCollections.listFold ((Compiler.PlannerOracle.PlanFitR.replay)) (Except.ok (Compiler.PlannerOracle.PlanFitR.observe (target))) (__trace) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))))
@[expose] public def PlanFitR.extract (jugs : (Prod (Nat) (Nat))) : Option (Nat) := Option.some ((jugs).1)
@[expose] public def PlanFitR.accept (_target : Nat) (__s : (Prod (Nat) (Nat))) : Option (Nat) := Compiler.PlannerOracle.PlanFitR.extract (__s)
@[expose] public def PlanFitR.answer (target : Nat) (__trace : List (Compiler.PlannerOracle.PlanFitR.Step)) : Option (Nat) := (match Compiler.PlannerOracle.PlanFitR.follow (target) (__trace) with | Except.ok __s => Compiler.PlannerOracle.PlanFitR.accept (target) (__s) | Except.error _ => Option.none)
public structure PlanFitR.Node where
  state : (Prod (Nat) (Nat))
  trace : List (Compiler.PlannerOracle.PlanFitR.Step)
public structure PlanFitR.Search where
  frontier : List (Compiler.PlannerOracle.PlanFitR.Node)
  visited : List ((Prod (Nat) (Nat)))
  found : Option ((Prod (Nat) (Compiler.PlannerOracle.PlanFitR.Node)))
  truncated : Bool
  ledger : Compiler.PlannerOracle.PlanFitR.Ledger
@[expose] public def PlanFitR.FillA.successors (__node : Compiler.PlannerOracle.PlanFitR.Node) : List (Compiler.PlannerOracle.PlanFitR.Node) := (match Compiler.PlannerOracle.PlanFitR.fire ((__node).state) (Compiler.PlannerOracle.PlanFitR.Step.FillA) with | Option.none => ([] : List (Compiler.PlannerOracle.PlanFitR.Node)) | Option.some __t => (({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((Compiler.PlannerOracle.PlanFitR.Step.FillA :: ([] : List (Compiler.PlannerOracle.PlanFitR.Step)))) : List (Compiler.PlannerOracle.PlanFitR.Step)) } : Compiler.PlannerOracle.PlanFitR.Node) :: ([] : List (Compiler.PlannerOracle.PlanFitR.Node))))
@[expose] public def PlanFitR.EmptyB.successors (__node : Compiler.PlannerOracle.PlanFitR.Node) : List (Compiler.PlannerOracle.PlanFitR.Node) := (match Compiler.PlannerOracle.PlanFitR.fire ((__node).state) (Compiler.PlannerOracle.PlanFitR.Step.EmptyB) with | Option.none => ([] : List (Compiler.PlannerOracle.PlanFitR.Node)) | Option.some __t => (({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((Compiler.PlannerOracle.PlanFitR.Step.EmptyB :: ([] : List (Compiler.PlannerOracle.PlanFitR.Step)))) : List (Compiler.PlannerOracle.PlanFitR.Step)) } : Compiler.PlannerOracle.PlanFitR.Node) :: ([] : List (Compiler.PlannerOracle.PlanFitR.Node))))
@[expose] public def PlanFitR.Pour.collect (__node : Compiler.PlannerOracle.PlanFitR.Node) (__acc : List (Compiler.PlannerOracle.PlanFitR.Node)) (__b : Nat) : List (Compiler.PlannerOracle.PlanFitR.Node) := (match Compiler.PlannerOracle.PlanFitR.fire ((__node).state) (Compiler.PlannerOracle.PlanFitR.Step.Pour (__b)) with | Option.none => __acc | Option.some __t => (LexLeanRuntime.append (__acc) ((({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((Compiler.PlannerOracle.PlanFitR.Step.Pour (__b) :: ([] : List (Compiler.PlannerOracle.PlanFitR.Step)))) : List (Compiler.PlannerOracle.PlanFitR.Step)) } : Compiler.PlannerOracle.PlanFitR.Node) :: ([] : List (Compiler.PlannerOracle.PlanFitR.Node)))) : List (Compiler.PlannerOracle.PlanFitR.Node)))
@[expose] public def PlanFitR.Pour.successors (__node : Compiler.PlannerOracle.PlanFitR.Node) : List (Compiler.PlannerOracle.PlanFitR.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Compiler.PlannerOracle.PlanFitR.Node)) (__b : Nat) => Compiler.PlannerOracle.PlanFitR.Pour.collect (__node) (__acc) (__b))) (([] : List (Compiler.PlannerOracle.PlanFitR.Node))) (Compiler.PlannerOracle.Pour.candidates ((__node).state)) : List (Compiler.PlannerOracle.PlanFitR.Node))
@[expose] public def PlanFitR.successors (__node : Compiler.PlannerOracle.PlanFitR.Node) : List (Compiler.PlannerOracle.PlanFitR.Node) := (LexLeanRuntime.append (Compiler.PlannerOracle.PlanFitR.FillA.successors (__node)) ((LexLeanRuntime.append (Compiler.PlannerOracle.PlanFitR.EmptyB.successors (__node)) (Compiler.PlannerOracle.PlanFitR.Pour.successors (__node)) : List (Compiler.PlannerOracle.PlanFitR.Node))) : List (Compiler.PlannerOracle.PlanFitR.Node))
@[expose] public def PlanFitR.fresh (__visited : List ((Prod (Nat) (Nat)))) (__nodes : List (Compiler.PlannerOracle.PlanFitR.Node)) : (Prod (List (Compiler.PlannerOracle.PlanFitR.Node)) (List ((Prod (Nat) (Nat))))) := (LexLeanCollections.listFold ((fun (__acc : (Prod (List (Compiler.PlannerOracle.PlanFitR.Node)) (List ((Prod (Nat) (Nat)))))) (__n : Compiler.PlannerOracle.PlanFitR.Node) => (if (LexLeanCollections.setContains ((__acc).2) ((__n).state) : Bool) then __acc else ((LexLeanRuntime.append ((__acc).1) ((__n :: ([] : List (Compiler.PlannerOracle.PlanFitR.Node)))) : List (Compiler.PlannerOracle.PlanFitR.Node)), (LexLeanCollections.setInsert ((__acc).2) ((__n).state) : List ((Prod (Nat) (Nat)))))))) ((([] : List (Compiler.PlannerOracle.PlanFitR.Node)), __visited)) (__nodes) : (Prod (List (Compiler.PlannerOracle.PlanFitR.Node)) (List ((Prod (Nat) (Nat))))))
@[expose] public def PlanFitR.start (target : Nat) : Compiler.PlannerOracle.PlanFitR.Search := (let __first : List (Compiler.PlannerOracle.PlanFitR.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Compiler.PlannerOracle.PlanFitR.Node)) (__n : Compiler.PlannerOracle.PlanFitR.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) ((target + target))) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (Compiler.PlannerOracle.PlanFitR.Node)))) : List (Compiler.PlannerOracle.PlanFitR.Node)) else __acc))) (([] : List (Compiler.PlannerOracle.PlanFitR.Node))) ((({ state := Compiler.PlannerOracle.PlanFitR.observe (target), trace := ([] : List (Compiler.PlannerOracle.PlanFitR.Step)) } : Compiler.PlannerOracle.PlanFitR.Node) :: ([] : List (Compiler.PlannerOracle.PlanFitR.Node)))) : List (Compiler.PlannerOracle.PlanFitR.Node)); ({ frontier := __first, visited := (LexLeanCollections.setInsert (([] : List ((Prod (Nat) (Nat))))) (Compiler.PlannerOracle.PlanFitR.observe (target)) : List ((Prod (Nat) (Nat)))), found := Option.none, truncated := (Nat.blt ((target + target)) (1)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := (LexLeanRuntime.length (__first) : Nat) } : Compiler.PlannerOracle.PlanFitR.Ledger) } : Compiler.PlannerOracle.PlanFitR.Search))
@[expose] public def PlanFitR.attempts (__s : (Prod (Nat) (Nat))) : Nat := (1 + (1 + (LexLeanRuntime.length (Compiler.PlannerOracle.Pour.candidates (__s)) : Nat)))
@[expose] public def PlanFitR.searchStep (target : Nat) (__r : Compiler.PlannerOracle.PlanFitR.Search) : Option (Compiler.PlannerOracle.PlanFitR.Search) := (match (__r).found with | Option.some _ => Option.none | Option.none => (match (__r).frontier with | List.nil => Option.none | List.cons __node __rest => (match Compiler.PlannerOracle.PlanFitR.accept (target) ((__node).state) with | Option.some __v => Option.some (({ frontier := __rest, visited := (__r).visited, found := Option.some ((__v, __node)), truncated := (__r).truncated, ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := ((__r).ledger).attempts, firings := ((__r).ledger).firings, expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : Compiler.PlannerOracle.PlanFitR.Ledger) } : Compiler.PlannerOracle.PlanFitR.Search)) | Option.none => (let __successors : List (Compiler.PlannerOracle.PlanFitR.Node) := Compiler.PlannerOracle.PlanFitR.successors (__node); (let __fresh : (Prod (List (Compiler.PlannerOracle.PlanFitR.Node)) (List ((Prod (Nat) (Nat))))) := Compiler.PlannerOracle.PlanFitR.fresh ((__r).visited) (__successors); (let __ordered : List (Compiler.PlannerOracle.PlanFitR.Node) := (LexLeanRuntime.append (__rest) ((__fresh).1) : List (Compiler.PlannerOracle.PlanFitR.Node)); (let __next : List (Compiler.PlannerOracle.PlanFitR.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Compiler.PlannerOracle.PlanFitR.Node)) (__n : Compiler.PlannerOracle.PlanFitR.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) ((target + target))) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (Compiler.PlannerOracle.PlanFitR.Node)))) : List (Compiler.PlannerOracle.PlanFitR.Node)) else __acc))) (([] : List (Compiler.PlannerOracle.PlanFitR.Node))) (__ordered) : List (Compiler.PlannerOracle.PlanFitR.Node)); Option.some (({ frontier := __next, visited := (__fresh).2, found := Option.none, truncated := ((__r).truncated || (Nat.blt ((target + target)) ((LexLeanRuntime.length (__ordered) : Nat)))), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + Compiler.PlannerOracle.PlanFitR.attempts ((__node).state)), firings := (((__r).ledger).firings + (LexLeanRuntime.length (__successors) : Nat)), expansions := (((__r).ledger).expansions + 1), verifications := ((__r).ledger).verifications, frontier := (if (Nat.blt (((__r).ledger).frontier) ((LexLeanRuntime.length (__next) : Nat))) then (LexLeanRuntime.length (__next) : Nat) else ((__r).ledger).frontier) } : Compiler.PlannerOracle.PlanFitR.Ledger) } : Compiler.PlannerOracle.PlanFitR.Search)))))))))
@[expose] public def PlanFitR.run (target : Nat) : (Prod (Compiler.PlannerOracle.PlanFitR.Search) (Bool)) := (LexLeanCollections.iterateUntil ((fun (__r : Compiler.PlannerOracle.PlanFitR.Search) => Compiler.PlannerOracle.PlanFitR.searchStep (target) (__r))) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Compiler.PlannerOracle.PlanFitR.start (target)) : (Prod (Compiler.PlannerOracle.PlanFitR.Search) (Bool)))
@[expose] public def PlanFitR.failure (__saturated : Bool) (__truncated : Bool) : (Prod Bool Bool) := (if (__saturated && (!__truncated)) then ((false, true) : Prod Bool Bool) else ((false, false) : Prod Bool Bool))
@[expose] public def PlanFitR (target : Nat) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (Compiler.PlannerOracle.PlanFitR.Step)))) := (let __final : (Prod (Compiler.PlannerOracle.PlanFitR.Search) (Bool)) := Compiler.PlannerOracle.PlanFitR.run (target); (match ((__final).1).found with | Option.some __hit => Except.ok (((__hit).1, ((__hit).2).trace)) | Option.none => Except.error (Compiler.PlannerOracle.PlanFitR.failure ((__final).2) (((__final).1).truncated))))
@[expose] public def PlanFitR.verdict (target : Nat) : Except ((Prod Bool Bool)) (Nat) := (match Compiler.PlannerOracle.PlanFitR (target) with | Except.ok __p => Except.ok ((__p).1) | Except.error __e => Except.error (__e))
public theorem PlanFitR.fire_sound (__s : (Prod (Nat) (Nat))) (__step : Compiler.PlannerOracle.PlanFitR.Step) (__t : (Prod (Nat) (Nat))) : ((Compiler.PlannerOracle.PlanFitR.fire (__s) (__step) = Option.some (__t)) -> Compiler.PlannerOracle.Moves (__s) (__t)) :=
by
  cases __step with
  | FillA =>
    exact Compiler.PlannerOracle.FillA.apply_sound __s __t
  | EmptyB =>
    exact Compiler.PlannerOracle.EmptyB.apply_sound __s __t
  | Pour __b =>
    exact Compiler.PlannerOracle.Pour.apply_sound __s __b __t
public theorem PlanFitR.replay_fire (__s : (Prod (Nat) (Nat))) (__step : Compiler.PlannerOracle.PlanFitR.Step) (__t : (Prod (Nat) (Nat))) : ((Compiler.PlannerOracle.PlanFitR.fire (__s) (__step) = Option.some (__t)) -> (Compiler.PlannerOracle.PlanFitR.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [Compiler.PlannerOracle.PlanFitR.replay]
  rw [llE]
public theorem PlanFitR.replay_sound (target : Nat) (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : Compiler.PlannerOracle.PlanFitR.Step) : ((LexLeanReasoning.Reaches ((Compiler.PlannerOracle.Moves)) (Compiler.PlannerOracle.PlanFitR.observe (target)) (__acc)) -> (LexLeanReasoning.Reaches ((Compiler.PlannerOracle.Moves)) (Compiler.PlannerOracle.PlanFitR.observe (target)) (Compiler.PlannerOracle.PlanFitR.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [Compiler.PlannerOracle.PlanFitR.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (Compiler.PlannerOracle.PlanFitR.fire_sound llS __step _ ‹_›)
public theorem PlanFitR.derivation (target : Nat) (__trace : List (Compiler.PlannerOracle.PlanFitR.Step)) : (LexLeanReasoning.Reaches ((Compiler.PlannerOracle.Moves)) (Compiler.PlannerOracle.PlanFitR.observe (target)) (Compiler.PlannerOracle.PlanFitR.follow (target) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((Compiler.PlannerOracle.PlanFitR.replay)) (fun (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) => (LexLeanReasoning.Reaches ((Compiler.PlannerOracle.Moves)) (Compiler.PlannerOracle.PlanFitR.observe (target)) (__acc))) (Compiler.PlannerOracle.PlanFitR.replay_sound (target)) (__trace) (Except.ok (Compiler.PlannerOracle.PlanFitR.observe (target))) (LexLeanReasoning.reachesStart ((Compiler.PlannerOracle.Moves)) (Compiler.PlannerOracle.PlanFitR.observe (target))))
public theorem PlanFitR.follow_invariant (target : Nat) (__trace : List (Compiler.PlannerOracle.PlanFitR.Step)) (__s : (Prod (Nat) (Nat))) : (Compiler.PlannerOracle.Fits (Compiler.PlannerOracle.PlanFitR.observe (target)) -> ((Compiler.PlannerOracle.PlanFitR.follow (target) (__trace) = Except.ok (__s)) -> Compiler.PlannerOracle.Fits (__s))) :=
  (fun llI llE => (LexLeanReasoning.starPreserves ((Compiler.PlannerOracle.Moves)) ((Compiler.PlannerOracle.Fits)) Compiler.PlannerOracle.Pouring.preserves (Compiler.PlannerOracle.PlanFitR.observe (target)) (__s) (Compiler.PlannerOracle.PlanFitR.derivation (target) (__trace) (__s) llE) llI))
public theorem PlanFitR.follow_relation (target : Nat) (__trace : List (Compiler.PlannerOracle.PlanFitR.Step)) (__s : (Prod (Nat) (Nat))) : ((Compiler.PlannerOracle.PlanFitR.follow (target) (__trace) = Except.ok (__s)) -> Compiler.PlannerOracle.Held (target) (__s)) :=
  (fun llE => (LexLeanReasoning.starPreserves ((Compiler.PlannerOracle.Moves)) (fun (__s : (Prod (Nat) (Nat))) => Compiler.PlannerOracle.Held (target) (__s)) (Compiler.PlannerOracle.held_preserved (target)) (Compiler.PlannerOracle.PlanFitR.observe (target)) (__s) (Compiler.PlannerOracle.PlanFitR.derivation (target) (__trace) (__s) llE) (Compiler.PlannerOracle.held_initial (target))))
public theorem PlanFitR.accept_sound (target : Nat) (__s : (Prod (Nat) (Nat))) (__v : Nat) : (Compiler.PlannerOracle.Held (target) (__s) -> ((Compiler.PlannerOracle.PlanFitR.accept (target) (__s) = Option.some (__v)) -> Compiler.PlannerOracle.Within (target) (__v))) :=
  (fun llJ llE => (Compiler.PlannerOracle.plan_held_correct (target) (__s) (__v) llJ llE))
public theorem PlanFitR.extend (target : Nat) (__node : Compiler.PlannerOracle.PlanFitR.Node) (__step : Compiler.PlannerOracle.PlanFitR.Step) (__t : (Prod (Nat) (Nat))) : ((Compiler.PlannerOracle.PlanFitR.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> ((Compiler.PlannerOracle.PlanFitR.fire ((__node).state) (__step) = Option.some (__t)) -> (Compiler.PlannerOracle.PlanFitR.follow (target) ((LexLeanRuntime.append ((__node).trace) ((__step :: ([] : List (Compiler.PlannerOracle.PlanFitR.Step)))) : List (Compiler.PlannerOracle.PlanFitR.Step))) = Except.ok (__t)))) :=
by
  intro llH llE
  dsimp only [Compiler.PlannerOracle.PlanFitR.follow]
  rw [LexLeanReasoning.foldSnoc]
  dsimp only [Compiler.PlannerOracle.PlanFitR.follow] at llH
  rw [llH]
  exact Compiler.PlannerOracle.PlanFitR.replay_fire _ __step __t llE
public theorem PlanFitR.FillA.successors_ok (target : Nat) (__node : Compiler.PlannerOracle.PlanFitR.Node) : ((Compiler.PlannerOracle.PlanFitR.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Compiler.PlannerOracle.PlanFitR.Node) => (Compiler.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Compiler.PlannerOracle.PlanFitR.FillA.successors (__node)))) :=
by
  intro llH
  dsimp only [Compiler.PlannerOracle.PlanFitR.FillA.successors]
  split
  · exact LexLeanReasoning.All.nil
  · exact LexLeanReasoning.allSingle _ _ (Compiler.PlannerOracle.PlanFitR.extend _ __node _ _ llH ‹_›)
public theorem PlanFitR.EmptyB.successors_ok (target : Nat) (__node : Compiler.PlannerOracle.PlanFitR.Node) : ((Compiler.PlannerOracle.PlanFitR.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Compiler.PlannerOracle.PlanFitR.Node) => (Compiler.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Compiler.PlannerOracle.PlanFitR.EmptyB.successors (__node)))) :=
by
  intro llH
  dsimp only [Compiler.PlannerOracle.PlanFitR.EmptyB.successors]
  split
  · exact LexLeanReasoning.All.nil
  · exact LexLeanReasoning.allSingle _ _ (Compiler.PlannerOracle.PlanFitR.extend _ __node _ _ llH ‹_›)
public theorem PlanFitR.Pour.successors_ok (target : Nat) (__node : Compiler.PlannerOracle.PlanFitR.Node) : ((Compiler.PlannerOracle.PlanFitR.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Compiler.PlannerOracle.PlanFitR.Node) => (Compiler.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Compiler.PlannerOracle.PlanFitR.Pour.successors (__node)))) :=
by
  intro llH
  dsimp only [Compiler.PlannerOracle.PlanFitR.Pour.successors]
  exact LexLeanReasoning.foldInvariant _ (LexLeanReasoning.All (fun (llN : Compiler.PlannerOracle.PlanFitR.Node) => Compiler.PlannerOracle.PlanFitR.follow _ llN.trace = Except.ok llN.state))
    (fun llA llB llP => by
      dsimp only [Compiler.PlannerOracle.PlanFitR.Pour.collect]
      split
      · exact llP
      · exact LexLeanReasoning.allAppend _ _ _ llP (LexLeanReasoning.allSingle _ _ (Compiler.PlannerOracle.PlanFitR.extend _ __node _ _ llH ‹_›)))
    _ _ LexLeanReasoning.All.nil
public theorem PlanFitR.successors_ok (target : Nat) (__node : Compiler.PlannerOracle.PlanFitR.Node) : ((Compiler.PlannerOracle.PlanFitR.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Compiler.PlannerOracle.PlanFitR.Node) => (Compiler.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Compiler.PlannerOracle.PlanFitR.successors (__node)))) :=
  (fun llH => (LexLeanReasoning.allAppend _ _ _ (Compiler.PlannerOracle.PlanFitR.FillA.successors_ok (target) (__node) llH) (LexLeanReasoning.allAppend _ _ _ (Compiler.PlannerOracle.PlanFitR.EmptyB.successors_ok (target) (__node) llH) (Compiler.PlannerOracle.PlanFitR.Pour.successors_ok (target) (__node) llH))))
public theorem PlanFitR.fresh_ok (target : Nat) (__visited : List ((Prod (Nat) (Nat)))) (__nodes : List (Compiler.PlannerOracle.PlanFitR.Node)) : ((LexLeanReasoning.All ((fun (__n : Compiler.PlannerOracle.PlanFitR.Node) => (Compiler.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (__nodes)) -> (LexLeanReasoning.All ((fun (__n : Compiler.PlannerOracle.PlanFitR.Node) => (Compiler.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((Compiler.PlannerOracle.PlanFitR.fresh (__visited) (__nodes)).1))) :=
  (LexLeanReasoning.freshAll ((fun (__n : Compiler.PlannerOracle.PlanFitR.Node) => (__n).state)) (fun (__n : Compiler.PlannerOracle.PlanFitR.Node) => (Compiler.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state))) (__visited) (__nodes))
public theorem PlanFitR.search_step (target : Nat) (__r : Compiler.PlannerOracle.PlanFitR.Search) (__q : Compiler.PlannerOracle.PlanFitR.Search) : ((Compiler.PlannerOracle.PlanFitR.searchStep (target) (__r) = Option.some (__q)) -> ((LexLeanReasoning.SearchOk ((fun (__n : Compiler.PlannerOracle.PlanFitR.Node) => (Compiler.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Compiler.PlannerOracle.PlanFitR.Node) => Compiler.PlannerOracle.PlanFitR.accept (target) ((__n).state))) ((__r).frontier) ((__r).found)) -> (LexLeanReasoning.SearchOk ((fun (__n : Compiler.PlannerOracle.PlanFitR.Node) => (Compiler.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Compiler.PlannerOracle.PlanFitR.Node) => Compiler.PlannerOracle.PlanFitR.accept (target) ((__n).state))) ((__q).frontier) ((__q).found)))) :=
by
  intro llE llH
  dsimp only [Compiler.PlannerOracle.PlanFitR.searchStep] at llE
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
        exact And.intro (LexLeanReasoning.capAll _ _ _ (LexLeanReasoning.allAppend _ _ _ llRestOk (Compiler.PlannerOracle.PlanFitR.fresh_ok _ _ _ (Compiler.PlannerOracle.PlanFitR.successors_ok _ llNode llNodeOk)))) True.intro
public theorem PlanFitR.search_peak (target : Nat) (__r : Compiler.PlannerOracle.PlanFitR.Search) (__q : Compiler.PlannerOracle.PlanFitR.Search) : ((Compiler.PlannerOracle.PlanFitR.searchStep (target) (__r) = Option.some (__q)) -> ((((__r).ledger).frontier <= (target + target)) -> (((__q).ledger).frontier <= (target + target)))) :=
by
  intro llE llH
  dsimp only [Compiler.PlannerOracle.PlanFitR.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        exact llH
      · cases llE
        exact LexLeanReasoning.peakBound _ _ _ llH (LexLeanReasoning.capBound _ _)
public theorem PlanFitR.search_count (target : Nat) (__r : Compiler.PlannerOracle.PlanFitR.Search) (__q : Compiler.PlannerOracle.PlanFitR.Search) : ((Compiler.PlannerOracle.PlanFitR.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [Compiler.PlannerOracle.PlanFitR.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        rfl
      · cases llE
        rfl
public theorem PlanFitR.search_verify_growth (target : Nat) (__r : Compiler.PlannerOracle.PlanFitR.Search) (__q : Compiler.PlannerOracle.PlanFitR.Search) : ((Compiler.PlannerOracle.PlanFitR.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).verifications <= (((__r).ledger).verifications + 1))) :=
by
  intro llE
  dsimp only [Compiler.PlannerOracle.PlanFitR.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem PlanFitR.search_expanded (target : Nat) (__r : Compiler.PlannerOracle.PlanFitR.Search) (__q : Compiler.PlannerOracle.PlanFitR.Search) : ((Compiler.PlannerOracle.PlanFitR.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).expansions <= (((__r).ledger).expansions + 1))) :=
by
  intro llE
  dsimp only [Compiler.PlannerOracle.PlanFitR.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem PlanFitR.search_ok (target : Nat) : (LexLeanReasoning.SearchOk ((fun (__n : Compiler.PlannerOracle.PlanFitR.Node) => (Compiler.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Compiler.PlannerOracle.PlanFitR.Node) => Compiler.PlannerOracle.PlanFitR.accept (target) ((__n).state))) (((Compiler.PlannerOracle.PlanFitR.run (target)).1).frontier) (((Compiler.PlannerOracle.PlanFitR.run (target)).1).found)) :=
by
  dsimp only [Compiler.PlannerOracle.PlanFitR.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((fun (__r : Compiler.PlannerOracle.PlanFitR.Search) => Compiler.PlannerOracle.PlanFitR.searchStep (target) (__r))) (fun (__r : Compiler.PlannerOracle.PlanFitR.Search) => (LexLeanReasoning.SearchOk ((fun (__n : Compiler.PlannerOracle.PlanFitR.Node) => (Compiler.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Compiler.PlannerOracle.PlanFitR.Node) => Compiler.PlannerOracle.PlanFitR.accept (target) ((__n).state))) ((__r).frontier) ((__r).found))) (Compiler.PlannerOracle.PlanFitR.search_step (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Compiler.PlannerOracle.PlanFitR.start (target)) (LexLeanReasoning.searchStart ((fun (__n : Compiler.PlannerOracle.PlanFitR.Node) => (Compiler.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Compiler.PlannerOracle.PlanFitR.Node) => Compiler.PlannerOracle.PlanFitR.accept (target) ((__n).state))) _ (LexLeanReasoning.capAll ((fun (__n : Compiler.PlannerOracle.PlanFitR.Node) => (Compiler.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((target + target)) _ (LexLeanReasoning.allSingle ((fun (__n : Compiler.PlannerOracle.PlanFitR.Node) => (Compiler.PlannerOracle.PlanFitR.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (({ state := Compiler.PlannerOracle.PlanFitR.observe (target), trace := ([] : List (Compiler.PlannerOracle.PlanFitR.Step)) } : Compiler.PlannerOracle.PlanFitR.Node)) rfl))))
public theorem PlanFitR.frontier_bounded (target : Nat) : ((((Compiler.PlannerOracle.PlanFitR.run (target)).1).ledger).frontier <= (target + target)) :=
by
  dsimp only [Compiler.PlannerOracle.PlanFitR.run]
  exact (LexLeanReasoning.iterateUntilBound ((fun (__r : Compiler.PlannerOracle.PlanFitR.Search) => Compiler.PlannerOracle.PlanFitR.searchStep (target) (__r))) ((fun (__r : Compiler.PlannerOracle.PlanFitR.Search) => ((__r).ledger).frontier)) ((target + target)) (Compiler.PlannerOracle.PlanFitR.search_peak (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Compiler.PlannerOracle.PlanFitR.start (target)) (LexLeanReasoning.capBound ((target + target)) _))
public theorem PlanFitR.verifications_bounded (target : Nat) : ((((Compiler.PlannerOracle.PlanFitR.run (target)).1).ledger).verifications <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [Compiler.PlannerOracle.PlanFitR.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : Compiler.PlannerOracle.PlanFitR.Search) => Compiler.PlannerOracle.PlanFitR.searchStep (target) (__r))) ((fun (__r : Compiler.PlannerOracle.PlanFitR.Search) => ((__r).ledger).verifications)) (Compiler.PlannerOracle.PlanFitR.search_verify_growth (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Compiler.PlannerOracle.PlanFitR.start (target)) rfl)
public theorem PlanFitR.expansions_bounded (target : Nat) : ((((Compiler.PlannerOracle.PlanFitR.run (target)).1).ledger).expansions <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [Compiler.PlannerOracle.PlanFitR.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : Compiler.PlannerOracle.PlanFitR.Search) => Compiler.PlannerOracle.PlanFitR.searchStep (target) (__r))) ((fun (__r : Compiler.PlannerOracle.PlanFitR.Search) => ((__r).ledger).expansions)) (Compiler.PlannerOracle.PlanFitR.search_expanded (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Compiler.PlannerOracle.PlanFitR.start (target)) rfl)
public theorem PlanFitR.iterations_bounded (target : Nat) : ((((Compiler.PlannerOracle.PlanFitR.run (target)).1).ledger).iterations <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [Compiler.PlannerOracle.PlanFitR.run]
  exact (LexLeanReasoning.iterateUntilCount ((fun (__r : Compiler.PlannerOracle.PlanFitR.Search) => Compiler.PlannerOracle.PlanFitR.searchStep (target) (__r))) ((fun (__r : Compiler.PlannerOracle.PlanFitR.Search) => ((__r).ledger).iterations)) (Compiler.PlannerOracle.PlanFitR.search_count (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Compiler.PlannerOracle.PlanFitR.start (target)) rfl)
public theorem PlanFitR.explained (target : Nat) : (forall (__v : Nat), (forall (__trace : List (Compiler.PlannerOracle.PlanFitR.Step)), ((Compiler.PlannerOracle.PlanFitR (target) = Except.ok ((__v, __trace))) -> ((Compiler.PlannerOracle.PlanFitR.answer (target) (__trace) = Option.some (__v)) /\ Compiler.PlannerOracle.Within (target) (__v))))) :=
by
  intro llV llT llE
  have llSearch := Compiler.PlannerOracle.PlanFitR.search_ok target
  dsimp only [Compiler.PlannerOracle.PlanFitR] at llE
  generalize llRun : Compiler.PlannerOracle.PlanFitR.run target = llR at llE llSearch
  split at llE
  · rename_i llHit llF
    cases llE
    have llOk := And.right llSearch
    rw [llF] at llOk
    exact And.intro (by dsimp only [Compiler.PlannerOracle.PlanFitR.answer]; rw [And.left llOk]; exact And.right llOk) (Compiler.PlannerOracle.PlanFitR.accept_sound _ _ _ (Compiler.PlannerOracle.PlanFitR.follow_relation target _ _ (And.left llOk)) (And.right llOk))
  · cases llE
public theorem PlanFitR.verdict_sound (target : Nat) : (forall (__v : Nat), ((Compiler.PlannerOracle.PlanFitR.verdict (target) = Except.ok (__v)) -> Compiler.PlannerOracle.Within (target) (__v))) :=
by
  intro llV llE
  dsimp only [Compiler.PlannerOracle.PlanFitR.verdict] at llE
  generalize llHP : Compiler.PlannerOracle.PlanFitR target = llR at llE
  cases llR with
  | error _ => cases llE
  | ok llP =>
    cases llE
    exact And.right (Compiler.PlannerOracle.PlanFitR.explained _ llP.1 llP.2 llHP)

public inductive PlanFitU.Step where
  | FillA
  | EmptyB
  | Pour (_ : Nat)
public structure PlanFitU.Ledger where
  iterations : Nat
  attempts : Nat
  firings : Nat
  expansions : Nat
  verifications : Nat
  frontier : Nat
@[expose] public def PlanFitU.observe (_target : Nat) : (Prod (Nat) (Nat)) := (0, 0)
@[expose] public def PlanFitU.fire (__s : (Prod (Nat) (Nat))) (__step : Compiler.PlannerOracle.PlanFitU.Step) : Option ((Prod (Nat) (Nat))) := (match __step with | Compiler.PlannerOracle.PlanFitU.Step.FillA => Compiler.PlannerOracle.FillA.apply (__s) | Compiler.PlannerOracle.PlanFitU.Step.EmptyB => Compiler.PlannerOracle.EmptyB.apply (__s) | Compiler.PlannerOracle.PlanFitU.Step.Pour __b => Compiler.PlannerOracle.Pour.apply (__s) (__b))
@[expose] public def PlanFitU.replay (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : Compiler.PlannerOracle.PlanFitU.Step) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (match __acc with | Except.error __e => Except.error (__e) | Except.ok __s => (match Compiler.PlannerOracle.PlanFitU.fire (__s) (__step) with | Option.none => Except.error (((true, true) : Prod Bool Bool)) | Option.some __t => Except.ok (__t)))
@[expose] public def PlanFitU.follow (target : Nat) (__trace : List (Compiler.PlannerOracle.PlanFitU.Step)) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))) := (LexLeanCollections.listFold ((Compiler.PlannerOracle.PlanFitU.replay)) (Except.ok (Compiler.PlannerOracle.PlanFitU.observe (target))) (__trace) : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat))))
@[expose] public def PlanFitU.extract (jugs : (Prod (Nat) (Nat))) : Option (Nat) := Option.some ((jugs).1)
@[expose] public def PlanFitU.accept (_target : Nat) (__s : (Prod (Nat) (Nat))) : Option (Nat) := Compiler.PlannerOracle.PlanFitU.extract (__s)
@[expose] public def PlanFitU.answer (target : Nat) (__trace : List (Compiler.PlannerOracle.PlanFitU.Step)) : Option (Nat) := (match Compiler.PlannerOracle.PlanFitU.follow (target) (__trace) with | Except.ok __s => Compiler.PlannerOracle.PlanFitU.accept (target) (__s) | Except.error _ => Option.none)
public structure PlanFitU.Node where
  state : (Prod (Nat) (Nat))
  trace : List (Compiler.PlannerOracle.PlanFitU.Step)
public structure PlanFitU.Search where
  frontier : List (Compiler.PlannerOracle.PlanFitU.Node)
  visited : List ((Prod (Nat) (Nat)))
  found : Option ((Prod (Nat) (Compiler.PlannerOracle.PlanFitU.Node)))
  truncated : Bool
  ledger : Compiler.PlannerOracle.PlanFitU.Ledger
@[expose] public def PlanFitU.FillA.successors (__node : Compiler.PlannerOracle.PlanFitU.Node) : List (Compiler.PlannerOracle.PlanFitU.Node) := (match Compiler.PlannerOracle.PlanFitU.fire ((__node).state) (Compiler.PlannerOracle.PlanFitU.Step.FillA) with | Option.none => ([] : List (Compiler.PlannerOracle.PlanFitU.Node)) | Option.some __t => (({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((Compiler.PlannerOracle.PlanFitU.Step.FillA :: ([] : List (Compiler.PlannerOracle.PlanFitU.Step)))) : List (Compiler.PlannerOracle.PlanFitU.Step)) } : Compiler.PlannerOracle.PlanFitU.Node) :: ([] : List (Compiler.PlannerOracle.PlanFitU.Node))))
@[expose] public def PlanFitU.EmptyB.successors (__node : Compiler.PlannerOracle.PlanFitU.Node) : List (Compiler.PlannerOracle.PlanFitU.Node) := (match Compiler.PlannerOracle.PlanFitU.fire ((__node).state) (Compiler.PlannerOracle.PlanFitU.Step.EmptyB) with | Option.none => ([] : List (Compiler.PlannerOracle.PlanFitU.Node)) | Option.some __t => (({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((Compiler.PlannerOracle.PlanFitU.Step.EmptyB :: ([] : List (Compiler.PlannerOracle.PlanFitU.Step)))) : List (Compiler.PlannerOracle.PlanFitU.Step)) } : Compiler.PlannerOracle.PlanFitU.Node) :: ([] : List (Compiler.PlannerOracle.PlanFitU.Node))))
@[expose] public def PlanFitU.Pour.collect (__node : Compiler.PlannerOracle.PlanFitU.Node) (__acc : List (Compiler.PlannerOracle.PlanFitU.Node)) (__b : Nat) : List (Compiler.PlannerOracle.PlanFitU.Node) := (match Compiler.PlannerOracle.PlanFitU.fire ((__node).state) (Compiler.PlannerOracle.PlanFitU.Step.Pour (__b)) with | Option.none => __acc | Option.some __t => (LexLeanRuntime.append (__acc) ((({ state := __t, trace := (LexLeanRuntime.append ((__node).trace) ((Compiler.PlannerOracle.PlanFitU.Step.Pour (__b) :: ([] : List (Compiler.PlannerOracle.PlanFitU.Step)))) : List (Compiler.PlannerOracle.PlanFitU.Step)) } : Compiler.PlannerOracle.PlanFitU.Node) :: ([] : List (Compiler.PlannerOracle.PlanFitU.Node)))) : List (Compiler.PlannerOracle.PlanFitU.Node)))
@[expose] public def PlanFitU.Pour.successors (__node : Compiler.PlannerOracle.PlanFitU.Node) : List (Compiler.PlannerOracle.PlanFitU.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Compiler.PlannerOracle.PlanFitU.Node)) (__b : Nat) => Compiler.PlannerOracle.PlanFitU.Pour.collect (__node) (__acc) (__b))) (([] : List (Compiler.PlannerOracle.PlanFitU.Node))) (Compiler.PlannerOracle.Pour.candidates ((__node).state)) : List (Compiler.PlannerOracle.PlanFitU.Node))
@[expose] public def PlanFitU.successors (__node : Compiler.PlannerOracle.PlanFitU.Node) : List (Compiler.PlannerOracle.PlanFitU.Node) := (LexLeanRuntime.append (Compiler.PlannerOracle.PlanFitU.FillA.successors (__node)) ((LexLeanRuntime.append (Compiler.PlannerOracle.PlanFitU.EmptyB.successors (__node)) (Compiler.PlannerOracle.PlanFitU.Pour.successors (__node)) : List (Compiler.PlannerOracle.PlanFitU.Node))) : List (Compiler.PlannerOracle.PlanFitU.Node))
@[expose] public def PlanFitU.fresh (__visited : List ((Prod (Nat) (Nat)))) (__nodes : List (Compiler.PlannerOracle.PlanFitU.Node)) : (Prod (List (Compiler.PlannerOracle.PlanFitU.Node)) (List ((Prod (Nat) (Nat))))) := (LexLeanCollections.listFold ((fun (__acc : (Prod (List (Compiler.PlannerOracle.PlanFitU.Node)) (List ((Prod (Nat) (Nat)))))) (__n : Compiler.PlannerOracle.PlanFitU.Node) => (if (LexLeanCollections.setContains ((__acc).2) ((__n).state) : Bool) then __acc else ((LexLeanRuntime.append ((__acc).1) ((__n :: ([] : List (Compiler.PlannerOracle.PlanFitU.Node)))) : List (Compiler.PlannerOracle.PlanFitU.Node)), (LexLeanCollections.setInsert ((__acc).2) ((__n).state) : List ((Prod (Nat) (Nat)))))))) ((([] : List (Compiler.PlannerOracle.PlanFitU.Node)), __visited)) (__nodes) : (Prod (List (Compiler.PlannerOracle.PlanFitU.Node)) (List ((Prod (Nat) (Nat))))))
@[expose] public def PlanFitU.start (target : Nat) : Compiler.PlannerOracle.PlanFitU.Search := (let __first : List (Compiler.PlannerOracle.PlanFitU.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Compiler.PlannerOracle.PlanFitU.Node)) (__n : Compiler.PlannerOracle.PlanFitU.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) ((target + target))) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (Compiler.PlannerOracle.PlanFitU.Node)))) : List (Compiler.PlannerOracle.PlanFitU.Node)) else __acc))) (([] : List (Compiler.PlannerOracle.PlanFitU.Node))) ((({ state := Compiler.PlannerOracle.PlanFitU.observe (target), trace := ([] : List (Compiler.PlannerOracle.PlanFitU.Step)) } : Compiler.PlannerOracle.PlanFitU.Node) :: ([] : List (Compiler.PlannerOracle.PlanFitU.Node)))) : List (Compiler.PlannerOracle.PlanFitU.Node)); ({ frontier := __first, visited := (LexLeanCollections.setInsert (([] : List ((Prod (Nat) (Nat))))) (Compiler.PlannerOracle.PlanFitU.observe (target)) : List ((Prod (Nat) (Nat)))), found := Option.none, truncated := (Nat.blt ((target + target)) (1)), ledger := ({ iterations := 0, attempts := 0, firings := 0, expansions := 0, verifications := 0, frontier := (LexLeanRuntime.length (__first) : Nat) } : Compiler.PlannerOracle.PlanFitU.Ledger) } : Compiler.PlannerOracle.PlanFitU.Search))
@[expose] public def PlanFitU.attempts (__s : (Prod (Nat) (Nat))) : Nat := (1 + (1 + (LexLeanRuntime.length (Compiler.PlannerOracle.Pour.candidates (__s)) : Nat)))
@[expose] public def PlanFitU.searchStep (target : Nat) (__r : Compiler.PlannerOracle.PlanFitU.Search) : Option (Compiler.PlannerOracle.PlanFitU.Search) := (match (__r).found with | Option.some _ => Option.none | Option.none => (match (__r).frontier with | List.nil => Option.none | List.cons __node __rest => (match Compiler.PlannerOracle.PlanFitU.accept (target) ((__node).state) with | Option.some __v => Option.some (({ frontier := __rest, visited := (__r).visited, found := Option.some ((__v, __node)), truncated := (__r).truncated, ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := ((__r).ledger).attempts, firings := ((__r).ledger).firings, expansions := ((__r).ledger).expansions, verifications := ((__r).ledger).verifications, frontier := ((__r).ledger).frontier } : Compiler.PlannerOracle.PlanFitU.Ledger) } : Compiler.PlannerOracle.PlanFitU.Search)) | Option.none => (let __successors : List (Compiler.PlannerOracle.PlanFitU.Node) := Compiler.PlannerOracle.PlanFitU.successors (__node); (let __fresh : (Prod (List (Compiler.PlannerOracle.PlanFitU.Node)) (List ((Prod (Nat) (Nat))))) := Compiler.PlannerOracle.PlanFitU.fresh ((__r).visited) (__successors); (let __ordered : List (Compiler.PlannerOracle.PlanFitU.Node) := (LexLeanRuntime.append (__rest) ((__fresh).1) : List (Compiler.PlannerOracle.PlanFitU.Node)); (let __next : List (Compiler.PlannerOracle.PlanFitU.Node) := (LexLeanCollections.listFold ((fun (__acc : List (Compiler.PlannerOracle.PlanFitU.Node)) (__n : Compiler.PlannerOracle.PlanFitU.Node) => (if (Nat.blt ((LexLeanRuntime.length (__acc) : Nat)) ((target + target))) then (LexLeanRuntime.append (__acc) ((__n :: ([] : List (Compiler.PlannerOracle.PlanFitU.Node)))) : List (Compiler.PlannerOracle.PlanFitU.Node)) else __acc))) (([] : List (Compiler.PlannerOracle.PlanFitU.Node))) (__ordered) : List (Compiler.PlannerOracle.PlanFitU.Node)); Option.some (({ frontier := __next, visited := (__fresh).2, found := Option.none, truncated := ((__r).truncated || (Nat.blt ((target + target)) ((LexLeanRuntime.length (__ordered) : Nat)))), ledger := ({ iterations := (((__r).ledger).iterations + 1), attempts := (((__r).ledger).attempts + Compiler.PlannerOracle.PlanFitU.attempts ((__node).state)), firings := (((__r).ledger).firings + (LexLeanRuntime.length (__successors) : Nat)), expansions := (((__r).ledger).expansions + 1), verifications := ((__r).ledger).verifications, frontier := (if (Nat.blt (((__r).ledger).frontier) ((LexLeanRuntime.length (__next) : Nat))) then (LexLeanRuntime.length (__next) : Nat) else ((__r).ledger).frontier) } : Compiler.PlannerOracle.PlanFitU.Ledger) } : Compiler.PlannerOracle.PlanFitU.Search)))))))))
@[expose] public def PlanFitU.run (target : Nat) : (Prod (Compiler.PlannerOracle.PlanFitU.Search) (Bool)) := (LexLeanCollections.iterateUntil ((fun (__r : Compiler.PlannerOracle.PlanFitU.Search) => Compiler.PlannerOracle.PlanFitU.searchStep (target) (__r))) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Compiler.PlannerOracle.PlanFitU.start (target)) : (Prod (Compiler.PlannerOracle.PlanFitU.Search) (Bool)))
@[expose] public def PlanFitU.failure (__saturated : Bool) (__truncated : Bool) : (Prod Bool Bool) := (if (__saturated && (!__truncated)) then ((false, true) : Prod Bool Bool) else ((false, false) : Prod Bool Bool))
@[expose] public def PlanFitU (target : Nat) : Except ((Prod Bool Bool)) ((Prod (Nat) (List (Compiler.PlannerOracle.PlanFitU.Step)))) := (let __final : (Prod (Compiler.PlannerOracle.PlanFitU.Search) (Bool)) := Compiler.PlannerOracle.PlanFitU.run (target); (match ((__final).1).found with | Option.some __hit => Except.ok (((__hit).1, ((__hit).2).trace)) | Option.none => Except.error (Compiler.PlannerOracle.PlanFitU.failure ((__final).2) (((__final).1).truncated))))
@[expose] public def PlanFitU.verdict (target : Nat) : Except ((Prod Bool Bool)) (Nat) := (match Compiler.PlannerOracle.PlanFitU (target) with | Except.ok __p => Except.ok ((__p).1) | Except.error __e => Except.error (__e))
public theorem PlanFitU.fire_sound (__s : (Prod (Nat) (Nat))) (__step : Compiler.PlannerOracle.PlanFitU.Step) (__t : (Prod (Nat) (Nat))) : ((Compiler.PlannerOracle.PlanFitU.fire (__s) (__step) = Option.some (__t)) -> Compiler.PlannerOracle.Moves (__s) (__t)) :=
by
  cases __step with
  | FillA =>
    exact Compiler.PlannerOracle.FillA.apply_sound __s __t
  | EmptyB =>
    exact Compiler.PlannerOracle.EmptyB.apply_sound __s __t
  | Pour __b =>
    exact Compiler.PlannerOracle.Pour.apply_sound __s __b __t
public theorem PlanFitU.replay_fire (__s : (Prod (Nat) (Nat))) (__step : Compiler.PlannerOracle.PlanFitU.Step) (__t : (Prod (Nat) (Nat))) : ((Compiler.PlannerOracle.PlanFitU.fire (__s) (__step) = Option.some (__t)) -> (Compiler.PlannerOracle.PlanFitU.replay (Except.ok (__s)) (__step) = Except.ok (__t))) :=
by
  intro llE
  dsimp only [Compiler.PlannerOracle.PlanFitU.replay]
  rw [llE]
public theorem PlanFitU.replay_sound (target : Nat) (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) (__step : Compiler.PlannerOracle.PlanFitU.Step) : ((LexLeanReasoning.Reaches ((Compiler.PlannerOracle.Moves)) (Compiler.PlannerOracle.PlanFitU.observe (target)) (__acc)) -> (LexLeanReasoning.Reaches ((Compiler.PlannerOracle.Moves)) (Compiler.PlannerOracle.PlanFitU.observe (target)) (Compiler.PlannerOracle.PlanFitU.replay (__acc) (__step)))) :=
by
  intro llH llT llE
  cases __acc with
  | error _ => cases llE
  | ok llS =>
    dsimp only [Compiler.PlannerOracle.PlanFitU.replay] at llE
    split at llE
    · cases llE
    · cases llE
      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) (Compiler.PlannerOracle.PlanFitU.fire_sound llS __step _ ‹_›)
public theorem PlanFitU.derivation (target : Nat) (__trace : List (Compiler.PlannerOracle.PlanFitU.Step)) : (LexLeanReasoning.Reaches ((Compiler.PlannerOracle.Moves)) (Compiler.PlannerOracle.PlanFitU.observe (target)) (Compiler.PlannerOracle.PlanFitU.follow (target) (__trace))) :=
  (LexLeanReasoning.foldInvariant ((Compiler.PlannerOracle.PlanFitU.replay)) (fun (__acc : Except ((Prod Bool Bool)) ((Prod (Nat) (Nat)))) => (LexLeanReasoning.Reaches ((Compiler.PlannerOracle.Moves)) (Compiler.PlannerOracle.PlanFitU.observe (target)) (__acc))) (Compiler.PlannerOracle.PlanFitU.replay_sound (target)) (__trace) (Except.ok (Compiler.PlannerOracle.PlanFitU.observe (target))) (LexLeanReasoning.reachesStart ((Compiler.PlannerOracle.Moves)) (Compiler.PlannerOracle.PlanFitU.observe (target))))
public theorem PlanFitU.follow_invariant (target : Nat) (__trace : List (Compiler.PlannerOracle.PlanFitU.Step)) (__s : (Prod (Nat) (Nat))) : (Compiler.PlannerOracle.Fits (Compiler.PlannerOracle.PlanFitU.observe (target)) -> ((Compiler.PlannerOracle.PlanFitU.follow (target) (__trace) = Except.ok (__s)) -> Compiler.PlannerOracle.Fits (__s))) :=
  (fun llI llE => (LexLeanReasoning.starPreserves ((Compiler.PlannerOracle.Moves)) ((Compiler.PlannerOracle.Fits)) Compiler.PlannerOracle.Pouring.preserves (Compiler.PlannerOracle.PlanFitU.observe (target)) (__s) (Compiler.PlannerOracle.PlanFitU.derivation (target) (__trace) (__s) llE) llI))
public theorem PlanFitU.accept_sound (target : Nat) (__s : (Prod (Nat) (Nat))) (__v : Nat) : ((Compiler.PlannerOracle.PlanFitU.accept (target) (__s) = Option.some (__v)) -> Compiler.PlannerOracle.Whole (target) (__v)) :=
  (fun llE => (Compiler.PlannerOracle.plan_whole_correct (target) (__s) (__v) llE))
public theorem PlanFitU.extend (target : Nat) (__node : Compiler.PlannerOracle.PlanFitU.Node) (__step : Compiler.PlannerOracle.PlanFitU.Step) (__t : (Prod (Nat) (Nat))) : ((Compiler.PlannerOracle.PlanFitU.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> ((Compiler.PlannerOracle.PlanFitU.fire ((__node).state) (__step) = Option.some (__t)) -> (Compiler.PlannerOracle.PlanFitU.follow (target) ((LexLeanRuntime.append ((__node).trace) ((__step :: ([] : List (Compiler.PlannerOracle.PlanFitU.Step)))) : List (Compiler.PlannerOracle.PlanFitU.Step))) = Except.ok (__t)))) :=
by
  intro llH llE
  dsimp only [Compiler.PlannerOracle.PlanFitU.follow]
  rw [LexLeanReasoning.foldSnoc]
  dsimp only [Compiler.PlannerOracle.PlanFitU.follow] at llH
  rw [llH]
  exact Compiler.PlannerOracle.PlanFitU.replay_fire _ __step __t llE
public theorem PlanFitU.FillA.successors_ok (target : Nat) (__node : Compiler.PlannerOracle.PlanFitU.Node) : ((Compiler.PlannerOracle.PlanFitU.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Compiler.PlannerOracle.PlanFitU.Node) => (Compiler.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Compiler.PlannerOracle.PlanFitU.FillA.successors (__node)))) :=
by
  intro llH
  dsimp only [Compiler.PlannerOracle.PlanFitU.FillA.successors]
  split
  · exact LexLeanReasoning.All.nil
  · exact LexLeanReasoning.allSingle _ _ (Compiler.PlannerOracle.PlanFitU.extend _ __node _ _ llH ‹_›)
public theorem PlanFitU.EmptyB.successors_ok (target : Nat) (__node : Compiler.PlannerOracle.PlanFitU.Node) : ((Compiler.PlannerOracle.PlanFitU.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Compiler.PlannerOracle.PlanFitU.Node) => (Compiler.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Compiler.PlannerOracle.PlanFitU.EmptyB.successors (__node)))) :=
by
  intro llH
  dsimp only [Compiler.PlannerOracle.PlanFitU.EmptyB.successors]
  split
  · exact LexLeanReasoning.All.nil
  · exact LexLeanReasoning.allSingle _ _ (Compiler.PlannerOracle.PlanFitU.extend _ __node _ _ llH ‹_›)
public theorem PlanFitU.Pour.successors_ok (target : Nat) (__node : Compiler.PlannerOracle.PlanFitU.Node) : ((Compiler.PlannerOracle.PlanFitU.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Compiler.PlannerOracle.PlanFitU.Node) => (Compiler.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Compiler.PlannerOracle.PlanFitU.Pour.successors (__node)))) :=
by
  intro llH
  dsimp only [Compiler.PlannerOracle.PlanFitU.Pour.successors]
  exact LexLeanReasoning.foldInvariant _ (LexLeanReasoning.All (fun (llN : Compiler.PlannerOracle.PlanFitU.Node) => Compiler.PlannerOracle.PlanFitU.follow _ llN.trace = Except.ok llN.state))
    (fun llA llB llP => by
      dsimp only [Compiler.PlannerOracle.PlanFitU.Pour.collect]
      split
      · exact llP
      · exact LexLeanReasoning.allAppend _ _ _ llP (LexLeanReasoning.allSingle _ _ (Compiler.PlannerOracle.PlanFitU.extend _ __node _ _ llH ‹_›)))
    _ _ LexLeanReasoning.All.nil
public theorem PlanFitU.successors_ok (target : Nat) (__node : Compiler.PlannerOracle.PlanFitU.Node) : ((Compiler.PlannerOracle.PlanFitU.follow (target) ((__node).trace) = Except.ok ((__node).state)) -> (LexLeanReasoning.All ((fun (__n : Compiler.PlannerOracle.PlanFitU.Node) => (Compiler.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (Compiler.PlannerOracle.PlanFitU.successors (__node)))) :=
  (fun llH => (LexLeanReasoning.allAppend _ _ _ (Compiler.PlannerOracle.PlanFitU.FillA.successors_ok (target) (__node) llH) (LexLeanReasoning.allAppend _ _ _ (Compiler.PlannerOracle.PlanFitU.EmptyB.successors_ok (target) (__node) llH) (Compiler.PlannerOracle.PlanFitU.Pour.successors_ok (target) (__node) llH))))
public theorem PlanFitU.fresh_ok (target : Nat) (__visited : List ((Prod (Nat) (Nat)))) (__nodes : List (Compiler.PlannerOracle.PlanFitU.Node)) : ((LexLeanReasoning.All ((fun (__n : Compiler.PlannerOracle.PlanFitU.Node) => (Compiler.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (__nodes)) -> (LexLeanReasoning.All ((fun (__n : Compiler.PlannerOracle.PlanFitU.Node) => (Compiler.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((Compiler.PlannerOracle.PlanFitU.fresh (__visited) (__nodes)).1))) :=
  (LexLeanReasoning.freshAll ((fun (__n : Compiler.PlannerOracle.PlanFitU.Node) => (__n).state)) (fun (__n : Compiler.PlannerOracle.PlanFitU.Node) => (Compiler.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state))) (__visited) (__nodes))
public theorem PlanFitU.search_step (target : Nat) (__r : Compiler.PlannerOracle.PlanFitU.Search) (__q : Compiler.PlannerOracle.PlanFitU.Search) : ((Compiler.PlannerOracle.PlanFitU.searchStep (target) (__r) = Option.some (__q)) -> ((LexLeanReasoning.SearchOk ((fun (__n : Compiler.PlannerOracle.PlanFitU.Node) => (Compiler.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Compiler.PlannerOracle.PlanFitU.Node) => Compiler.PlannerOracle.PlanFitU.accept (target) ((__n).state))) ((__r).frontier) ((__r).found)) -> (LexLeanReasoning.SearchOk ((fun (__n : Compiler.PlannerOracle.PlanFitU.Node) => (Compiler.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Compiler.PlannerOracle.PlanFitU.Node) => Compiler.PlannerOracle.PlanFitU.accept (target) ((__n).state))) ((__q).frontier) ((__q).found)))) :=
by
  intro llE llH
  dsimp only [Compiler.PlannerOracle.PlanFitU.searchStep] at llE
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
        exact And.intro (LexLeanReasoning.capAll _ _ _ (LexLeanReasoning.allAppend _ _ _ llRestOk (Compiler.PlannerOracle.PlanFitU.fresh_ok _ _ _ (Compiler.PlannerOracle.PlanFitU.successors_ok _ llNode llNodeOk)))) True.intro
public theorem PlanFitU.search_peak (target : Nat) (__r : Compiler.PlannerOracle.PlanFitU.Search) (__q : Compiler.PlannerOracle.PlanFitU.Search) : ((Compiler.PlannerOracle.PlanFitU.searchStep (target) (__r) = Option.some (__q)) -> ((((__r).ledger).frontier <= (target + target)) -> (((__q).ledger).frontier <= (target + target)))) :=
by
  intro llE llH
  dsimp only [Compiler.PlannerOracle.PlanFitU.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        exact llH
      · cases llE
        exact LexLeanReasoning.peakBound _ _ _ llH (LexLeanReasoning.capBound _ _)
public theorem PlanFitU.search_count (target : Nat) (__r : Compiler.PlannerOracle.PlanFitU.Search) (__q : Compiler.PlannerOracle.PlanFitU.Search) : ((Compiler.PlannerOracle.PlanFitU.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).iterations = (((__r).ledger).iterations + 1))) :=
by
  intro llE
  dsimp only [Compiler.PlannerOracle.PlanFitU.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        rfl
      · cases llE
        rfl
public theorem PlanFitU.search_verify_growth (target : Nat) (__r : Compiler.PlannerOracle.PlanFitU.Search) (__q : Compiler.PlannerOracle.PlanFitU.Search) : ((Compiler.PlannerOracle.PlanFitU.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).verifications <= (((__r).ledger).verifications + 1))) :=
by
  intro llE
  dsimp only [Compiler.PlannerOracle.PlanFitU.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem PlanFitU.search_expanded (target : Nat) (__r : Compiler.PlannerOracle.PlanFitU.Search) (__q : Compiler.PlannerOracle.PlanFitU.Search) : ((Compiler.PlannerOracle.PlanFitU.searchStep (target) (__r) = Option.some (__q)) -> (((__q).ledger).expansions <= (((__r).ledger).expansions + 1))) :=
by
  intro llE
  dsimp only [Compiler.PlannerOracle.PlanFitU.searchStep] at llE
  split at llE
  · cases llE
  · split at llE
    · cases llE
    · split at llE
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
      · cases llE
        first | exact Nat.le_refl _ | exact Nat.le_succ _
public theorem PlanFitU.search_ok (target : Nat) : (LexLeanReasoning.SearchOk ((fun (__n : Compiler.PlannerOracle.PlanFitU.Node) => (Compiler.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Compiler.PlannerOracle.PlanFitU.Node) => Compiler.PlannerOracle.PlanFitU.accept (target) ((__n).state))) (((Compiler.PlannerOracle.PlanFitU.run (target)).1).frontier) (((Compiler.PlannerOracle.PlanFitU.run (target)).1).found)) :=
by
  dsimp only [Compiler.PlannerOracle.PlanFitU.run]
  exact (LexLeanReasoning.iterateUntilInvariant ((fun (__r : Compiler.PlannerOracle.PlanFitU.Search) => Compiler.PlannerOracle.PlanFitU.searchStep (target) (__r))) (fun (__r : Compiler.PlannerOracle.PlanFitU.Search) => (LexLeanReasoning.SearchOk ((fun (__n : Compiler.PlannerOracle.PlanFitU.Node) => (Compiler.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Compiler.PlannerOracle.PlanFitU.Node) => Compiler.PlannerOracle.PlanFitU.accept (target) ((__n).state))) ((__r).frontier) ((__r).found))) (Compiler.PlannerOracle.PlanFitU.search_step (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Compiler.PlannerOracle.PlanFitU.start (target)) (LexLeanReasoning.searchStart ((fun (__n : Compiler.PlannerOracle.PlanFitU.Node) => (Compiler.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((fun (__n : Compiler.PlannerOracle.PlanFitU.Node) => Compiler.PlannerOracle.PlanFitU.accept (target) ((__n).state))) _ (LexLeanReasoning.capAll ((fun (__n : Compiler.PlannerOracle.PlanFitU.Node) => (Compiler.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) ((target + target)) _ (LexLeanReasoning.allSingle ((fun (__n : Compiler.PlannerOracle.PlanFitU.Node) => (Compiler.PlannerOracle.PlanFitU.follow (target) ((__n).trace) = Except.ok ((__n).state)))) (({ state := Compiler.PlannerOracle.PlanFitU.observe (target), trace := ([] : List (Compiler.PlannerOracle.PlanFitU.Step)) } : Compiler.PlannerOracle.PlanFitU.Node)) rfl))))
public theorem PlanFitU.frontier_bounded (target : Nat) : ((((Compiler.PlannerOracle.PlanFitU.run (target)).1).ledger).frontier <= (target + target)) :=
by
  dsimp only [Compiler.PlannerOracle.PlanFitU.run]
  exact (LexLeanReasoning.iterateUntilBound ((fun (__r : Compiler.PlannerOracle.PlanFitU.Search) => Compiler.PlannerOracle.PlanFitU.searchStep (target) (__r))) ((fun (__r : Compiler.PlannerOracle.PlanFitU.Search) => ((__r).ledger).frontier)) ((target + target)) (Compiler.PlannerOracle.PlanFitU.search_peak (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Compiler.PlannerOracle.PlanFitU.start (target)) (LexLeanReasoning.capBound ((target + target)) _))
public theorem PlanFitU.verifications_bounded (target : Nat) : ((((Compiler.PlannerOracle.PlanFitU.run (target)).1).ledger).verifications <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [Compiler.PlannerOracle.PlanFitU.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : Compiler.PlannerOracle.PlanFitU.Search) => Compiler.PlannerOracle.PlanFitU.searchStep (target) (__r))) ((fun (__r : Compiler.PlannerOracle.PlanFitU.Search) => ((__r).ledger).verifications)) (Compiler.PlannerOracle.PlanFitU.search_verify_growth (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Compiler.PlannerOracle.PlanFitU.start (target)) rfl)
public theorem PlanFitU.expansions_bounded (target : Nat) : ((((Compiler.PlannerOracle.PlanFitU.run (target)).1).ledger).expansions <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [Compiler.PlannerOracle.PlanFitU.run]
  exact (LexLeanReasoning.iterateUntilGrowth ((fun (__r : Compiler.PlannerOracle.PlanFitU.Search) => Compiler.PlannerOracle.PlanFitU.searchStep (target) (__r))) ((fun (__r : Compiler.PlannerOracle.PlanFitU.Search) => ((__r).ledger).expansions)) (Compiler.PlannerOracle.PlanFitU.search_expanded (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Compiler.PlannerOracle.PlanFitU.start (target)) rfl)
public theorem PlanFitU.iterations_bounded (target : Nat) : ((((Compiler.PlannerOracle.PlanFitU.run (target)).1).ledger).iterations <= (LexLeanRuntime.multiply (target) (8) : Nat)) :=
by
  dsimp only [Compiler.PlannerOracle.PlanFitU.run]
  exact (LexLeanReasoning.iterateUntilCount ((fun (__r : Compiler.PlannerOracle.PlanFitU.Search) => Compiler.PlannerOracle.PlanFitU.searchStep (target) (__r))) ((fun (__r : Compiler.PlannerOracle.PlanFitU.Search) => ((__r).ledger).iterations)) (Compiler.PlannerOracle.PlanFitU.search_count (target)) ((LexLeanRuntime.multiply (target) (8) : Nat)) (Compiler.PlannerOracle.PlanFitU.start (target)) rfl)
public theorem PlanFitU.explained (target : Nat) : (forall (__v : Nat), (forall (__trace : List (Compiler.PlannerOracle.PlanFitU.Step)), ((Compiler.PlannerOracle.PlanFitU (target) = Except.ok ((__v, __trace))) -> ((Compiler.PlannerOracle.PlanFitU.answer (target) (__trace) = Option.some (__v)) /\ Compiler.PlannerOracle.Whole (target) (__v))))) :=
by
  intro llV llT llE
  have llSearch := Compiler.PlannerOracle.PlanFitU.search_ok target
  dsimp only [Compiler.PlannerOracle.PlanFitU] at llE
  generalize llRun : Compiler.PlannerOracle.PlanFitU.run target = llR at llE llSearch
  split at llE
  · rename_i llHit llF
    cases llE
    have llOk := And.right llSearch
    rw [llF] at llOk
    exact And.intro (by dsimp only [Compiler.PlannerOracle.PlanFitU.answer]; rw [And.left llOk]; exact And.right llOk) (Compiler.PlannerOracle.PlanFitU.accept_sound _ _ _ (And.right llOk))
  · cases llE
public theorem PlanFitU.verdict_sound (target : Nat) : (forall (__v : Nat), ((Compiler.PlannerOracle.PlanFitU.verdict (target) = Except.ok (__v)) -> Compiler.PlannerOracle.Whole (target) (__v))) :=
by
  intro llV llE
  dsimp only [Compiler.PlannerOracle.PlanFitU.verdict] at llE
  generalize llHP : Compiler.PlannerOracle.PlanFitU target = llR at llE
  cases llR with
  | error _ => cases llE
  | ok llP =>
    cases llE
    exact And.right (Compiler.PlannerOracle.PlanFitU.explained _ llP.1 llP.2 llHP)

end Compiler.PlannerOracle
