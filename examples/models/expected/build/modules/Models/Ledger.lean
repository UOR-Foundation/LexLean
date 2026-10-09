module
public import Init
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Models.Ledger

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

@[expose, reducible] public def Capped (s : Nat) : Prop := (s <= 100)

@[expose] public def cappedCheck (s : Nat) : Bool := (Nat.ble (s) (100))

public theorem capped_check_sound (s : Nat) : ((cappedCheck (s) = true) -> Capped (s)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Capped, cappedCheck, ← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem capped_check_complete (s : Nat) : (Capped (s) -> (cappedCheck (s) = true)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Capped, cappedCheck, ← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose, reducible] public def Small (_s : Nat) (x : Nat) : Prop := (x <= 10)

@[expose] public def smallCheck (_s : Nat) (x : Nat) : Bool := (Nat.ble (x) (10))

public theorem small_check_sound (s : Nat) (x : Nat) : ((smallCheck (s) (x) = true) -> Small (s) (x)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Small, smallCheck, ← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose, reducible] public def Grows (s : Nat) (_x : Nat) (t : Nat) (_y : Nat) : Prop := (s <= t)

@[expose] public def growsCheck (s : Nat) (_x : Nat) (t : Nat) (_y : Nat) : Bool := (Nat.ble (s) (t))

public theorem grows_check_sound (s : Nat) (x : Nat) (t : Nat) (y : Nat) : ((growsCheck (s) (x) (t) (y) = true) -> Grows (s) (x) (t) (y)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Grows, growsCheck, ← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem grows_check_complete (s : Nat) (x : Nat) (t : Nat) (y : Nat) : (Grows (s) (x) (t) (y) -> (growsCheck (s) (x) (t) (y) = true)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Grows, growsCheck, ← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem LedgerContract.invariant_sound : LexLeanModels.Sound1 ((Models.Ledger.cappedCheck)) ((Models.Ledger.Capped)) := Models.Ledger.capped_check_sound
public theorem LedgerContract.invariant_complete : LexLeanModels.Complete1 ((Models.Ledger.cappedCheck)) ((Models.Ledger.Capped)) := Models.Ledger.capped_check_complete
public theorem LedgerContract.postcondition_sound : LexLeanModels.Sound4 ((Models.Ledger.growsCheck)) ((Models.Ledger.Grows)) := Models.Ledger.grows_check_sound
public theorem LedgerContract.postcondition_complete : LexLeanModels.Complete4 ((Models.Ledger.growsCheck)) ((Models.Ledger.Grows)) := Models.Ledger.grows_check_complete
public theorem LedgerContract.precondition_sound : LexLeanModels.Sound2 ((Models.Ledger.smallCheck)) ((Models.Ledger.Small)) := Models.Ledger.small_check_sound

@[expose] public def Spill.initial : Nat := 0
@[expose] public def Spill (s : Nat) (x : Nat) : (Prod (Nat) (Nat)) := ((s + x), x)

@[expose] public def spillRef (s : Nat) (x : Nat) : (Prod (Nat) (Nat)) := ((s + x), x)

public theorem spill_grows (s : Nat) (x : Nat) : (Capped (s) -> (Small (s) (x) -> Grows (s) (x) ((Spill (s) (x)).1) ((Spill (s) (x)).2))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Capped, Grows, Small, Spill, ← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem spill_reference (s : Nat) (x : Nat) : (Capped (s) -> (Small (s) (x) -> (Spill (s) (x) = spillRef (s) (x)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Spill, spillRef, ← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem SpillEvidence.spill_grows : LexLeanModels.SatisfiesStep ((Models.Ledger.Capped)) ((Models.Ledger.Small)) ((Models.Ledger.Grows)) ((Models.Ledger.Spill)) := Models.Ledger.spill_grows

@[expose] public def SpillModel.initial : Nat := Models.Ledger.Spill.initial
@[expose] public def SpillModel (__state : Nat) (__input : Nat) : (Prod (Nat) (Nat)) := Models.Ledger.Spill (__state) (__input)

public theorem SpillReference.spill_reference : LexLeanModels.EquivalentStep ((Models.Ledger.Capped)) ((Models.Ledger.Small)) ((Models.Ledger.Spill)) ((Models.Ledger.spillRef)) := Models.Ledger.spill_reference

@[expose] public def SpillRawModel.initial : Nat := Models.Ledger.Spill.initial
@[expose] public def SpillRawModel (__state : Nat) (__input : Nat) : (Prod (Nat) (Nat)) := Models.Ledger.Spill (__state) (__input)

public theorem TallyContract.postcondition_sound : LexLeanModels.Sound4 ((Models.Ledger.growsCheck)) ((Models.Ledger.Grows)) := Models.Ledger.grows_check_sound
public theorem TallyContract.precondition_sound : LexLeanModels.Sound2 ((Models.Ledger.smallCheck)) ((Models.Ledger.Small)) := Models.Ledger.small_check_sound

@[expose] public def Tally.initial : Nat := 0
@[expose] public def Tally (s : Nat) (x : Nat) : (Prod (Nat) (Nat)) := ((s + x), s)

@[expose] public def tallyRef (s : Nat) (x : Nat) : (Prod (Nat) (Nat)) := ((s + x), s)

public theorem tally_grows (s : Nat) (x : Nat) : (Small (s) (x) -> Grows (s) (x) ((Tally (s) (x)).1) ((Tally (s) (x)).2)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Grows, Small, Tally, ← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem tally_reference (s : Nat) (x : Nat) : (Small (s) (x) -> (Tally (s) (x) = tallyRef (s) (x))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Tally, tallyRef, ← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem TallyEvidence.tally_reference : LexLeanModels.EquivalentStepNoInvariant ((Models.Ledger.Small)) ((Models.Ledger.Tally)) ((Models.Ledger.tallyRef)) := Models.Ledger.tally_reference
public theorem TallyEvidence.tally_grows : LexLeanModels.SatisfiesStepNoInvariant ((Models.Ledger.Small)) ((Models.Ledger.Grows)) ((Models.Ledger.Tally)) := Models.Ledger.tally_grows

@[expose] public def TallyModel.initial : Nat := Models.Ledger.Tally.initial
@[expose] public def TallyModel (__state : Nat) (__input : Nat) : (Prod (Nat) (Nat)) := Models.Ledger.Tally (__state) (__input)

public theorem CapContract.invariant_sound : LexLeanModels.Sound1 ((Models.Ledger.cappedCheck)) ((Models.Ledger.Capped)) := Models.Ledger.capped_check_sound
public theorem CapContract.invariant_complete : LexLeanModels.Complete1 ((Models.Ledger.cappedCheck)) ((Models.Ledger.Capped)) := Models.Ledger.capped_check_complete
public theorem CapContract.postcondition_sound : LexLeanModels.Sound4 ((Models.Ledger.growsCheck)) ((Models.Ledger.Grows)) := Models.Ledger.grows_check_sound

@[expose] public def Clamp.initial : Nat := 0
@[expose] public def Clamp (s : Nat) (x : Nat) : (Prod (Nat) (Nat)) := (let total : Nat := (s + x); ((LexLeanRuntime.subtract (total) ((LexLeanRuntime.subtract (total) (100) : Nat)) : Nat), (LexLeanRuntime.subtract (total) ((LexLeanRuntime.subtract (total) (100) : Nat)) : Nat)))

@[expose] public def clampRef (s : Nat) (x : Nat) : (Prod (Nat) (Nat)) := (let total : Nat := (s + x); ((LexLeanRuntime.subtract (total) ((LexLeanRuntime.subtract (total) (100) : Nat)) : Nat), (LexLeanRuntime.subtract (total) ((LexLeanRuntime.subtract (total) (100) : Nat)) : Nat)))

public theorem clamp_initial : Capped (Clamp.initial) := by
  decide

public theorem clamp_preserves (s : Nat) (x : Nat) : (Capped (s) -> Capped ((Clamp (s) (x)).1)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Capped, Clamp, ← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem clamp_grows (s : Nat) (x : Nat) : (Capped (s) -> Grows (s) (x) ((Clamp (s) (x)).1) ((Clamp (s) (x)).2)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Capped, Clamp, Grows, ← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem clamp_reference (s : Nat) (x : Nat) : (Capped (s) -> (Clamp (s) (x) = clampRef (s) (x))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Clamp, clampRef, ← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem ClampEvidence.clamp_reference : LexLeanModels.EquivalentStepNoPrecondition ((Models.Ledger.Capped)) ((Models.Ledger.Clamp)) ((Models.Ledger.clampRef)) := Models.Ledger.clamp_reference
public theorem ClampEvidence.clamp_initial : LexLeanModels.Initial ((Models.Ledger.Capped)) (Models.Ledger.Clamp.initial) := Models.Ledger.clamp_initial
public theorem ClampEvidence.clamp_preserves : LexLeanModels.PreservesTotal ((Models.Ledger.Capped)) ((Models.Ledger.Clamp)) := Models.Ledger.clamp_preserves
public theorem ClampEvidence.clamp_grows : LexLeanModels.SatisfiesStepNoPrecondition ((Models.Ledger.Capped)) ((Models.Ledger.Grows)) ((Models.Ledger.Clamp)) := Models.Ledger.clamp_grows

@[expose] public def ClampModel.initial : Nat := Models.Ledger.Clamp.initial
@[expose] public def ClampModel (__state : Nat) (__input : Nat) : (Prod (Nat) (Nat)) := Models.Ledger.Clamp (__state) (__input)

public theorem FreeContract.postcondition_sound : LexLeanModels.Sound4 ((Models.Ledger.growsCheck)) ((Models.Ledger.Grows)) := Models.Ledger.grows_check_sound

public theorem tally_free_grows (s : Nat) (x : Nat) : Grows (s) (x) ((Tally (s) (x)).1) ((Tally (s) (x)).2) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Grows, Tally, ← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem tally_free_reference (s : Nat) (x : Nat) : (Tally (s) (x) = tallyRef (s) (x)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Tally, tallyRef, ← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem FreeEvidence.tally_free_reference : LexLeanModels.EquivalentStepTotal ((Models.Ledger.Tally)) ((Models.Ledger.tallyRef)) := Models.Ledger.tally_free_reference
public theorem FreeEvidence.tally_free_grows : LexLeanModels.SatisfiesStepTotal ((Models.Ledger.Grows)) ((Models.Ledger.Tally)) := Models.Ledger.tally_free_grows

@[expose] public def FreeModel.initial : Nat := Models.Ledger.Tally.initial
@[expose] public def FreeModel (__state : Nat) (__input : Nat) : (Prod (Nat) (Nat)) := Models.Ledger.Tally (__state) (__input)

@[expose, reducible] public def Within (x : Nat) : Prop := (x <= 10)

@[expose] public def withinCheck (x : Nat) : Bool := (Nat.ble (x) (10))

public theorem within_check_sound (x : Nat) : ((withinCheck (x) = true) -> Within (x)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Within, withinCheck, ← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem within_check_complete (x : Nat) : (Within (x) -> (withinCheck (x) = true)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Within, withinCheck, ← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose, reducible] public def Below (x : Nat) (y : Nat) : Prop := (y <= x)

@[expose] public def belowCheck (x : Nat) (y : Nat) : Bool := (Nat.ble (y) (x))

public theorem below_check_sound (x : Nat) (y : Nat) : ((belowCheck (x) (y) = true) -> Below (x) (y)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Below, belowCheck, ← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem GuessContract.postcondition_sound : LexLeanModels.Sound2 ((Models.Ledger.belowCheck)) ((Models.Ledger.Below)) := Models.Ledger.below_check_sound
public theorem GuessContract.precondition_sound : LexLeanModels.Sound1 ((Models.Ledger.withinCheck)) ((Models.Ledger.Within)) := Models.Ledger.within_check_sound
public theorem GuessContract.precondition_complete : LexLeanModels.Complete1 ((Models.Ledger.withinCheck)) ((Models.Ledger.Within)) := Models.Ledger.within_check_complete

@[expose] public def Guess (x : Nat) : Nat := (LexLeanRuntime.subtract (x) (1) : Nat)

@[expose] public def guessRef (x : Nat) : Nat := (LexLeanRuntime.subtract (x) (1) : Nat)

public theorem guess_reference (x : Nat) : (Within (x) -> (Guess (x) = guessRef (x))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Guess, guessRef, ← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose] public def guessSamples : List ((Prod (Nat) (Nat))) := ((0, 0) :: ((1, 0) :: ((2, 1) :: ((5, 4) :: ((9, 9) :: ([] : List ((Prod (Nat) (Nat)))))))))

@[expose] public def sameNat (expected : Nat) (observed : Nat) : Bool := (Nat.beq (expected) (observed))

public theorem same_nat_sound (expected : Nat) (observed : Nat) : ((sameNat (expected) (observed) = true) -> (expected = observed)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [sameNat, ← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem guess_agreement : (((LexLeanRuntime.length (guessSamples) : Nat) = 5) /\ ((LexLeanCollections.listFold ((fun (count : Nat) («example» : (Prod (Nat) (Nat))) => (if sameNat (Guess ((«example»).1)) ((«example»).2) then (count + 1) else count))) (0) (guessSamples) : Nat) = 4)) := by
  decide

public theorem GuessSample.guess_agreement : LexLeanModels.Agreement ((Models.Ledger.Guess)) ((Models.Ledger.sameNat)) (Models.Ledger.guessSamples) (5) (4) := by decide

@[expose] public def GuessModel (__input : Nat) : Nat := Models.Ledger.Guess (__input)

public theorem GuessEquivalence.guess_reference : LexLeanModels.Equivalent ((Models.Ledger.Within)) ((Models.Ledger.Guess)) ((Models.Ledger.guessRef)) := Models.Ledger.guess_reference

@[expose] public def GuessExactModel (__input : Nat) : Nat := Models.Ledger.Guess (__input)

@[expose] public def Overshoot (x : Nat) : Nat := (x + 1)

@[expose] public def OvershootModel (__input : Nat) : Nat := Models.Ledger.Overshoot (__input)

@[expose] public def violationCode (violation : (Prod Bool Bool)) : Nat := (match violation with | (false, false) => 1 | (false, true) => 2 | (true, false) => 3 | (true, true) => 4)

end Models.Ledger
