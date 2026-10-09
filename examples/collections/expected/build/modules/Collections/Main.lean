module
public import Init
public import Collections.Measure
public import Collections.Tables
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Collections.Main

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

@[expose] public def ancestors (parents : List ((Prod (Nat) (Nat)))) : (Prod (List ((Prod (Nat) (Nat)))) (Bool)) := (LexLeanCollections.iterateUntil ((fun (state : List ((Prod (Nat) (Nat)))) => (let derived : List ((Prod (Nat) (Nat))) := (LexLeanCollections.setFold ((fun (outer : List ((Prod (Nat) (Nat)))) (left : (Prod (Nat) (Nat))) => (LexLeanCollections.setFold ((fun (inner : List ((Prod (Nat) (Nat)))) (right : (Prod (Nat) (Nat))) => (if (Nat.beq ((left).2) ((right).1)) then (LexLeanCollections.setInsert (inner) (((left).1, (right).2)) : List ((Prod (Nat) (Nat)))) else inner))) (outer) (state) : List ((Prod (Nat) (Nat)))))) (state) (state) : List ((Prod (Nat) (Nat)))); (if (Nat.beq ((LexLeanCollections.setSize (derived) : Nat)) ((LexLeanCollections.setSize (state) : Nat))) then Option.none else Option.some (derived))))) (8) (parents) : (Prod (List ((Prod (Nat) (Nat)))) (Bool)))

@[expose] public def family : List ((Prod (Nat) (Nat))) := ([(1, 2), (2, 3), (3, 4)] : List ((Prod (Nat) (Nat))))

public theorem table_slots : (Collections.Tables.buildTable (("x" :: ("y" :: ("x2" :: ([] : List (String)))))) = ([("x", 0), ("x2", 2), ("y", 1)] : List (Prod (String) (Nat)))) := by
  decide

public theorem table_lookup : ((LexLeanCollections.mapLookup (Collections.Tables.buildTable (("a" :: ("b" :: ([] : List (String)))))) ("b") : Option (Nat)) = Option.some (1)) := by
  decide

public theorem call_order : (Collections.Tables.callOrder = Option.some (("main" :: ("emit" :: ("parse" :: ("report" :: ("lex" :: ([] : List (String))))))))) := by
  decide

public theorem parse_reaches_lex : (Collections.Tables.reachableFromParse = (["lex", "parse"] : List (String))) := by
  decide

public theorem cycle_has_no_order : ((LexLeanCollections.graphTopological (([(1, [2]), (2, [1])] : List (Prod (Nat) (List (Nat))))) : Option (List (Nat))) = Option.none) := by
  decide

public theorem ancestor_closure : (ancestors (family) = (([(1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)] : List ((Prod (Nat) (Nat)))), true)) := by
  decide

public theorem bounded_iteration : ((LexLeanCollections.iterate ((fun (value : Nat) => (value + 2))) (3) (1) : Nat) = 7) := by
  decide

public theorem reordered_literals_agree : (([(1, true), (2, false)] : List (Prod (Nat) (Bool))) = ([(1, true), (2, false)] : List (Prod (Nat) (Bool)))) := by
  rfl

public theorem op_map_insert : ((LexLeanCollections.mapInsert (([("a", 1), ("b", 2)] : List (Prod (String) (Nat)))) ("c") (3) : List (Prod (String) (Nat))) = ([("a", 1), ("b", 2), ("c", 3)] : List (Prod (String) (Nat)))) := by
  decide

public theorem op_map_insert_replaces : ((LexLeanCollections.mapInsert (([("a", 1), ("b", 2)] : List (Prod (String) (Nat)))) ("a") (9) : List (Prod (String) (Nat))) = ([("a", 9), ("b", 2)] : List (Prod (String) (Nat)))) := by
  decide

public theorem op_map_remove : ((LexLeanCollections.mapRemove (([("a", 1), ("b", 2)] : List (Prod (String) (Nat)))) ("a") : List (Prod (String) (Nat))) = ([("b", 2)] : List (Prod (String) (Nat)))) := by
  decide

public theorem op_map_remove_absent : ((LexLeanCollections.mapRemove (([("a", 1), ("b", 2)] : List (Prod (String) (Nat)))) ("z") : List (Prod (String) (Nat))) = ([("a", 1), ("b", 2)] : List (Prod (String) (Nat)))) := by
  decide

public theorem op_map_lookup : ((LexLeanCollections.mapLookup (([("a", 1), ("b", 2)] : List (Prod (String) (Nat)))) ("b") : Option (Nat)) = Option.some (2)) := by
  decide

public theorem op_map_lookup_absent : ((LexLeanCollections.mapLookup (([("a", 1), ("b", 2)] : List (Prod (String) (Nat)))) ("z") : Option (Nat)) = Option.none) := by
  decide

public theorem op_map_contains : ((LexLeanCollections.mapContains (([("a", 1), ("b", 2)] : List (Prod (String) (Nat)))) ("a") : Bool) = true) := by
  decide

public theorem op_map_size : ((LexLeanCollections.mapSize (([("a", 1), ("b", 2)] : List (Prod (String) (Nat)))) : Nat) = 2) := by
  decide

public theorem op_map_keys : ((LexLeanCollections.mapKeys (([("a", 1), ("b", 2)] : List (Prod (String) (Nat)))) : List (String)) = ("a" :: ("b" :: ([] : List (String))))) := by
  decide

public theorem op_map_values : ((LexLeanCollections.mapValues (([("a", 1), ("b", 2)] : List (Prod (String) (Nat)))) : List (Nat)) = (1 :: (2 :: ([] : List (Nat))))) := by
  decide

public theorem op_map_entries : ((LexLeanCollections.mapEntries (([("a", 1), ("b", 2)] : List (Prod (String) (Nat)))) : List ((Prod (String) (Nat)))) = (("a", 1) :: (("b", 2) :: ([] : List ((Prod (String) (Nat))))))) := by
  decide

public theorem op_map_fold : ((LexLeanCollections.mapFold ((fun (visited : List (String)) (key : String) (_value : Nat) => (key :: visited))) (([] : List (String))) (([("a", 1), ("b", 2)] : List (Prod (String) (Nat)))) : List (String)) = ("b" :: ("a" :: ([] : List (String))))) := by
  decide

public theorem op_set_insert : ((LexLeanCollections.setInsert (([1, 2, 3] : List (Nat))) (0) : List (Nat)) = ([0, 1, 2, 3] : List (Nat))) := by
  decide

public theorem op_set_remove : ((LexLeanCollections.setRemove (([1, 2, 3] : List (Nat))) (2) : List (Nat)) = ([1, 3] : List (Nat))) := by
  decide

public theorem op_set_contains : ((LexLeanCollections.setContains (([1, 2, 3] : List (Nat))) (4) : Bool) = false) := by
  decide

public theorem op_set_size : ((LexLeanCollections.setSize (([1, 2, 3] : List (Nat))) : Nat) = 3) := by
  decide

public theorem op_set_elements : ((LexLeanCollections.setElements (([1, 2, 3] : List (Nat))) : List (Nat)) = (1 :: (2 :: (3 :: ([] : List (Nat)))))) := by
  decide

public theorem op_set_union : ((LexLeanCollections.setUnion (([1, 2, 3] : List (Nat))) (([0, 5] : List (Nat))) : List (Nat)) = ([0, 1, 2, 3, 5] : List (Nat))) := by
  decide

public theorem op_set_intersection : ((LexLeanCollections.setIntersection (([1, 2, 3] : List (Nat))) (([2, 3, 9] : List (Nat))) : List (Nat)) = ([2, 3] : List (Nat))) := by
  decide

public theorem op_set_difference : ((LexLeanCollections.setDifference (([1, 2, 3] : List (Nat))) (([2] : List (Nat))) : List (Nat)) = ([1, 3] : List (Nat))) := by
  decide

public theorem op_set_fold : ((LexLeanCollections.setFold ((fun (visited : List (Nat)) (element : Nat) => (element :: visited))) (([] : List (Nat))) (([1, 2, 3] : List (Nat))) : List (Nat)) = (3 :: (2 :: (1 :: ([] : List (Nat)))))) := by
  decide

public theorem op_list_fold : ((LexLeanCollections.listFold ((fun (visited : List (Nat)) (element : Nat) => (element :: visited))) (([] : List (Nat))) ((5 :: (1 :: (4 :: ([] : List (Nat)))))) : List (Nat)) = (4 :: (1 :: (5 :: ([] : List (Nat)))))) := by
  decide

public theorem op_iterate_zero : ((LexLeanCollections.iterate ((fun (value : Nat) => (value + 2))) (0) (1) : Nat) = 1) := by
  decide

public theorem op_iterate_until_fixed_point : ((LexLeanCollections.iterateUntil ((fun (value : Nat) => (if (Nat.blt (value) (3)) then Option.some ((value + 1)) else Option.none))) (10) (0) : (Prod (Nat) (Bool))) = (3, true)) := by
  decide

public theorem op_iterate_until_exhausted : ((LexLeanCollections.iterateUntil ((fun (value : Nat) => (if (Nat.blt (value) (3)) then Option.some ((value + 1)) else Option.none))) (2) (0) : (Prod (Nat) (Bool))) = (2, false)) := by
  decide

public theorem op_graph_successors : ((LexLeanCollections.graphSuccessors (([(1, [2, 3]), (2, [4]), (3, [4]), (4, [])] : List (Prod (Nat) (List (Nat))))) (1) : List (Nat)) = ([2, 3] : List (Nat))) := by
  decide

public theorem op_graph_successors_absent : ((LexLeanCollections.graphSuccessors (([(1, [2, 3]), (2, [4]), (3, [4]), (4, [])] : List (Prod (Nat) (List (Nat))))) (9) : List (Nat)) = ([] : List (Nat))) := by
  decide

public theorem op_graph_reachable : ((LexLeanCollections.graphReachable (([(1, [2, 3]), (2, [4]), (3, [4]), (4, [])] : List (Prod (Nat) (List (Nat))))) (2) : List (Nat)) = ([2, 4] : List (Nat))) := by
  decide

public theorem op_graph_topological : ((LexLeanCollections.graphTopological (([(1, [2, 3]), (2, [4]), (3, [4]), (4, [])] : List (Prod (Nat) (List (Nat))))) : Option (List (Nat))) = Option.some ((1 :: (2 :: (3 :: (4 :: ([] : List (Nat)))))))) := by
  decide

public theorem key_order_nat : (([0, 9, 10, 11, 100] : List (Nat)) = (LexLeanCollections.listFold ((fun (built : List (Nat)) (element : Nat) => (LexLeanCollections.setInsert (built) (element) : List (Nat)))) (([] : List (Nat))) ((10 :: (9 :: (100 :: (0 :: (11 :: ([] : List (Nat)))))))) : List (Nat))) := by
  decide

public theorem key_order_int : (([(-100 : Int), (-10 : Int), (-2 : Int), (0 : Int), (3 : Int), (9 : Int), (100 : Int)] : List (Int)) = (LexLeanCollections.listFold ((fun (built : List (Int)) (element : Int) => (LexLeanCollections.setInsert (built) (element) : List (Int)))) (([] : List (Int))) (((3 : Int) :: ((-2 : Int) :: ((0 : Int) :: ((-10 : Int) :: ((100 : Int) :: ((-100 : Int) :: ((9 : Int) :: ([] : List (Int)))))))))) : List (Int))) := by
  decide

public theorem key_order_int8 : (([(-128 : Int8), (-9 : Int8), (-1 : Int8), (0 : Int8), (9 : Int8), (127 : Int8)] : List (Int8)) = (LexLeanCollections.listFold ((fun (built : List (Int8)) (element : Int8) => (LexLeanCollections.setInsert (built) (element) : List (Int8)))) (([] : List (Int8))) (((127 : Int8) :: ((-1 : Int8) :: ((0 : Int8) :: ((-128 : Int8) :: ((9 : Int8) :: ((-9 : Int8) :: ([] : List (Int8))))))))) : List (Int8))) := by
  decide

public theorem key_order_int16 : (([(-32768 : Int16), (-10 : Int16), (-1 : Int16), (0 : Int16), (10 : Int16), (32767 : Int16)] : List (Int16)) = (LexLeanCollections.listFold ((fun (built : List (Int16)) (element : Int16) => (LexLeanCollections.setInsert (built) (element) : List (Int16)))) (([] : List (Int16))) (((32767 : Int16) :: ((-1 : Int16) :: ((0 : Int16) :: ((-32768 : Int16) :: ((10 : Int16) :: ((-10 : Int16) :: ([] : List (Int16))))))))) : List (Int16))) := by
  decide

public theorem key_order_int32 : (([(-2147483648 : Int32), (-100 : Int32), (-1 : Int32), (0 : Int32), (100 : Int32), (2147483647 : Int32)] : List (Int32)) = (LexLeanCollections.listFold ((fun (built : List (Int32)) (element : Int32) => (LexLeanCollections.setInsert (built) (element) : List (Int32)))) (([] : List (Int32))) (((2147483647 : Int32) :: ((-1 : Int32) :: ((0 : Int32) :: ((-2147483648 : Int32) :: ((100 : Int32) :: ((-100 : Int32) :: ([] : List (Int32))))))))) : List (Int32))) := by
  decide

public theorem key_order_int64 : (([(-9223372036854775808 : Int64), (-99 : Int64), (-1 : Int64), (0 : Int64), (99 : Int64), (9223372036854775807 : Int64)] : List (Int64)) = (LexLeanCollections.listFold ((fun (built : List (Int64)) (element : Int64) => (LexLeanCollections.setInsert (built) (element) : List (Int64)))) (([] : List (Int64))) (((9223372036854775807 : Int64) :: ((-1 : Int64) :: ((0 : Int64) :: ((-9223372036854775808 : Int64) :: ((99 : Int64) :: ((-99 : Int64) :: ([] : List (Int64))))))))) : List (Int64))) := by
  decide

public theorem key_order_uint8 : (([(0 : UInt8), (7 : UInt8), (70 : UInt8), (255 : UInt8)] : List (UInt8)) = (LexLeanCollections.listFold ((fun (built : List (UInt8)) (element : UInt8) => (LexLeanCollections.setInsert (built) (element) : List (UInt8)))) (([] : List (UInt8))) (((255 : UInt8) :: ((0 : UInt8) :: ((7 : UInt8) :: ((70 : UInt8) :: ([] : List (UInt8))))))) : List (UInt8))) := by
  decide

public theorem key_order_uint16 : (([(0 : UInt16), (9 : UInt16), (10 : UInt16), (65535 : UInt16)] : List (UInt16)) = (LexLeanCollections.listFold ((fun (built : List (UInt16)) (element : UInt16) => (LexLeanCollections.setInsert (built) (element) : List (UInt16)))) (([] : List (UInt16))) (((65535 : UInt16) :: ((0 : UInt16) :: ((9 : UInt16) :: ((10 : UInt16) :: ([] : List (UInt16))))))) : List (UInt16))) := by
  decide

public theorem key_order_uint32 : (([(0 : UInt32), (99 : UInt32), (100 : UInt32), (4294967295 : UInt32)] : List (UInt32)) = (LexLeanCollections.listFold ((fun (built : List (UInt32)) (element : UInt32) => (LexLeanCollections.setInsert (built) (element) : List (UInt32)))) (([] : List (UInt32))) (((4294967295 : UInt32) :: ((0 : UInt32) :: ((99 : UInt32) :: ((100 : UInt32) :: ([] : List (UInt32))))))) : List (UInt32))) := by
  decide

public theorem key_order_uint64 : (([(0 : UInt64), (9 : UInt64), (10 : UInt64), (18446744073709551615 : UInt64)] : List (UInt64)) = (LexLeanCollections.listFold ((fun (built : List (UInt64)) (element : UInt64) => (LexLeanCollections.setInsert (built) (element) : List (UInt64)))) (([] : List (UInt64))) (((18446744073709551615 : UInt64) :: ((0 : UInt64) :: ((9 : UInt64) :: ((10 : UInt64) :: ([] : List (UInt64))))))) : List (UInt64))) := by
  decide

public theorem key_order_bool : (([false, true] : List (Bool)) = (LexLeanCollections.listFold ((fun (built : List (Bool)) (element : Bool) => (LexLeanCollections.setInsert (built) (element) : List (Bool)))) (([] : List (Bool))) ((true :: (false :: ([] : List (Bool))))) : List (Bool))) := by
  decide

public theorem key_order_string : ((["", "Z", "a", "a b", "ab", "z", "ß", "é", "日本", "😀"] : List (String)) = (LexLeanCollections.listFold ((fun (built : List (String)) (element : String) => (LexLeanCollections.setInsert (built) (element) : List (String)))) (([] : List (String))) (("z" :: ("é" :: ("a" :: ("Z" :: ("ß" :: ("日本" :: ("😀" :: ("ab" :: ("" :: ("a b" :: ([] : List (String))))))))))))) : List (String))) := by
  decide

public theorem key_order_product : (([(1, "a"), (1, "z"), (2, "B"), (2, "a")] : List ((Prod (Nat) (String)))) = (LexLeanCollections.listFold ((fun (built : List ((Prod (Nat) (String)))) (element : (Prod (Nat) (String))) => (LexLeanCollections.setInsert (built) (element) : List ((Prod (Nat) (String)))))) (([] : List ((Prod (Nat) (String))))) (((2, "a") :: ((1, "z") :: ((2, "B") :: ((1, "a") :: ([] : List ((Prod (Nat) (String))))))))) : List ((Prod (Nat) (String))))) := by
  decide

end Collections.Main
