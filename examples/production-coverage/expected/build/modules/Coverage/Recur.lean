module
public import Init
public import Coverage.Syntax
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Coverage.Recur

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

mutual
@[expose] public def roseSize (Item : Type) : (rose : Coverage.Syntax.Rose (Item)) -> Nat
  | Coverage.Syntax.Rose.node _ children => (1 + forestSize (Item) (children))
termination_by structural rose => rose

@[expose] public def forestSize (Item : Type) : (forest : List (Coverage.Syntax.Rose (Item))) -> Nat
  | List.nil => 0
  | List.cons head tail => (roseSize (Item) (head) + forestSize (Item) (tail))
termination_by structural forest => forest
end

mutual
@[expose] public def exprSize : (expression : Coverage.Syntax.Expr) -> Nat
  | Coverage.Syntax.Expr.literal _ => 1
  | Coverage.Syntax.Expr.plus left right => ((exprSize (left) + exprSize (right)) + 1)
  | Coverage.Syntax.Expr.block statements result => (statementsSize (statements) + exprSize (result))
termination_by structural expression => expression

@[expose] public def statementsSize : (statements : List (Coverage.Syntax.Stmt)) -> Nat
  | List.nil => 0
  | List.cons head tail => (statementSize (head) + statementsSize (tail))
termination_by structural statements => statements

@[expose] public def statementSize : (statement : Coverage.Syntax.Stmt) -> Nat
  | Coverage.Syntax.Stmt.assign _ value => (exprSize (value) + 1)
  | Coverage.Syntax.Stmt.sequence first second => (statementSize (first) + statementSize (second))
termination_by structural statement => statement
end

mutual
@[expose] public def isEven : (number : Nat) -> Bool
  | Nat.zero => true
  | Nat.succ previous => isOdd (previous)
termination_by structural number => number

@[expose] public def isOdd : (number : Nat) -> Bool
  | Nat.zero => false
  | Nat.succ previous => isEven (previous)
termination_by structural number => number
end

public theorem countdown_decreases (number : Nat) (_steps : Nat) : (((Nat.blt (number) (2)) = false) -> ((LexLeanRuntime.subtract (number) (2) : Nat) < number)) := by
  intros
  try set_option linter.unusedSimpArgs false in simp only [← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose, semireducible] public def countdown (number : Nat) (steps : Nat) : Nat := (match (generalizing := false) __decrease0 : (Nat.blt (number) (2)) with | true => steps | false => countdown ((LexLeanRuntime.subtract (number) (2) : Nat)) ((steps + 1)))
termination_by number
decreasing_by all_goals first | (have __evidence := countdown_decreases (number) (steps) (__decrease0); subst_vars; exact __evidence)

public theorem reduce_decreases (value : Nat) (bound : Nat) : (((Nat.beq (bound) (0)) = false) -> (((Nat.blt (value) (bound)) = false) -> ((LexLeanRuntime.subtract (value) (bound) : Nat) < value))) := by
  intros
  try set_option linter.unusedSimpArgs false in simp only [← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose, semireducible] public def reduce (value : Nat) (bound : Nat) : Nat := (match (generalizing := false) __decrease0 : (Nat.beq (bound) (0)) with | true => value | false => (match (generalizing := false) __decrease1 : (Nat.blt (value) (bound)) with | true => value | false => reduce ((LexLeanRuntime.subtract (value) (bound) : Nat)) (bound)))
termination_by value
decreasing_by all_goals first | (have __evidence := reduce_decreases (value) (bound) (__decrease0) (__decrease1); subst_vars; exact __evidence)

public theorem search_decreases (target : Nat) (low : Nat) (high : Nat) : (((Nat.blt (low) (high)) = true) -> (((Nat.ble (target) ((low + low))) = false) -> ((LexLeanRuntime.subtract (high) ((low + 1)) : Nat) < (LexLeanRuntime.subtract (high) (low) : Nat)))) := by
  intros
  try set_option linter.unusedSimpArgs false in simp only [← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose, semireducible] public def search (target : Nat) (low : Nat) (high : Nat) : Nat := (match (generalizing := false) __decrease0 : (Nat.blt (low) (high)) with | true => (match (generalizing := false) __decrease1 : (Nat.ble (target) ((low + low))) with | true => low | false => search (target) ((low + 1)) (high)) | false => high)
termination_by (LexLeanRuntime.subtract (high) (low) : Nat)
decreasing_by all_goals first | (have __evidence := search_decreases (target) (low) (high) (__decrease0) (__decrease1); subst_vars; exact __evidence)

@[expose] public def weight : (term : Coverage.Syntax.Term) -> Nat
  | Coverage.Syntax.Term.literal _ => 1
  | Coverage.Syntax.Term.plus left right => (((weight (left) + weight (left)) + weight (right)) + 1)

public theorem reassociate_literal (term : Coverage.Syntax.Term) (left : Coverage.Syntax.Term) (right : Coverage.Syntax.Term) (value : Nat) : ((term = Coverage.Syntax.Term.plus (left) (right)) -> ((left = Coverage.Syntax.Term.literal (value)) -> (weight (right) < weight (term)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [weight, ← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem reassociate_plus (term : Coverage.Syntax.Term) (left : Coverage.Syntax.Term) (right : Coverage.Syntax.Term) (inner : Coverage.Syntax.Term) (rest : Coverage.Syntax.Term) : ((term = Coverage.Syntax.Term.plus (left) (right)) -> ((left = Coverage.Syntax.Term.plus (inner) (rest)) -> (weight (Coverage.Syntax.Term.plus (inner) (Coverage.Syntax.Term.plus (rest) (right))) < weight (term)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [weight, ← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose, semireducible] public def reassociate (term : Coverage.Syntax.Term) : Coverage.Syntax.Term := (match (generalizing := false) __decrease0 : term with | Coverage.Syntax.Term.literal value => Coverage.Syntax.Term.literal (value) | Coverage.Syntax.Term.plus left right => (match (generalizing := false) __decrease1 : left with | Coverage.Syntax.Term.literal value => Coverage.Syntax.Term.plus (Coverage.Syntax.Term.literal (value)) (reassociate (right)) | Coverage.Syntax.Term.plus inner rest => reassociate (Coverage.Syntax.Term.plus (inner) (Coverage.Syntax.Term.plus (rest) (right)))))
termination_by weight (term)
decreasing_by all_goals first | (have __evidence := reassociate_literal (term) _ _ _ (__decrease0) (__decrease1); subst_vars; exact __evidence) | (have __evidence := reassociate_plus (term) _ _ _ _ (__decrease0) (__decrease1); subst_vars; exact __evidence)

public theorem prune_plus (term : Coverage.Syntax.Term) (left : Coverage.Syntax.Term) (right : Coverage.Syntax.Term) (inner : Coverage.Syntax.Term) (rest : Coverage.Syntax.Term) : ((term = Coverage.Syntax.Term.plus (left) (right)) -> ((left = Coverage.Syntax.Term.plus (inner) (rest)) -> (weight (Coverage.Syntax.Term.plus (inner) (right)) < weight (term)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [weight, ← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose, semireducible] public def prune (term : Coverage.Syntax.Term) : Coverage.Syntax.Term := (match (generalizing := false) __decrease0 : term with | Coverage.Syntax.Term.literal value => Coverage.Syntax.Term.literal (value) | Coverage.Syntax.Term.plus left right => (match (generalizing := false) __decrease1 : left with | Coverage.Syntax.Term.literal _ => prune (right) | Coverage.Syntax.Term.plus inner _ => prune (Coverage.Syntax.Term.plus (inner) (right))))
termination_by weight (term)
decreasing_by all_goals first | (have __evidence := reassociate_literal (term) _ _ _ (__decrease0) (__decrease1); subst_vars; exact __evidence) | (have __evidence := prune_plus (term) _ _ _ _ (__decrease0) (__decrease1); subst_vars; exact __evidence)

mutual
@[expose] public def foldExpr : (expression : Coverage.Syntax.Expr) -> Coverage.Syntax.Expr
  | Coverage.Syntax.Expr.literal value => Coverage.Syntax.Expr.literal (value)
  | Coverage.Syntax.Expr.plus left right => (let foldedLeft : Coverage.Syntax.Expr := foldExpr (left); (let foldedRight : Coverage.Syntax.Expr := foldExpr (right); (match foldedLeft with | Coverage.Syntax.Expr.literal leftValue => (match foldedRight with | Coverage.Syntax.Expr.literal rightValue => Coverage.Syntax.Expr.literal ((leftValue + rightValue)) | Coverage.Syntax.Expr.plus _ _ => Coverage.Syntax.Expr.plus (foldedLeft) (foldedRight) | Coverage.Syntax.Expr.block _ _ => Coverage.Syntax.Expr.plus (foldedLeft) (foldedRight)) | Coverage.Syntax.Expr.plus _ _ => Coverage.Syntax.Expr.plus (foldedLeft) (foldedRight) | Coverage.Syntax.Expr.block _ _ => Coverage.Syntax.Expr.plus (foldedLeft) (foldedRight))))
  | Coverage.Syntax.Expr.block statements result => Coverage.Syntax.Expr.block (foldStatements (statements)) (foldExpr (result))
termination_by structural expression => expression

@[expose] public def foldStatements : (statements : List (Coverage.Syntax.Stmt)) -> List (Coverage.Syntax.Stmt)
  | List.nil => ([] : List (Coverage.Syntax.Stmt))
  | List.cons head tail => (foldStatement (head) :: foldStatements (tail))
termination_by structural statements => statements

@[expose] public def foldStatement : (statement : Coverage.Syntax.Stmt) -> Coverage.Syntax.Stmt
  | Coverage.Syntax.Stmt.assign target value => Coverage.Syntax.Stmt.assign (target) (foldExpr (value))
  | Coverage.Syntax.Stmt.sequence first second => Coverage.Syntax.Stmt.sequence (foldStatement (first)) (foldStatement (second))
termination_by structural statement => statement
end

public theorem ping_to_pong (number : Nat) : (((Nat.beq (number) (0)) = false) -> ((number + number) < ((number + number) + 1))) := by
  intros
  try set_option linter.unusedSimpArgs false in simp only [← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem pong_to_ping (number : Nat) : (((Nat.beq (number) (0)) = false) -> ((((LexLeanRuntime.subtract (number) (1) : Nat) + (LexLeanRuntime.subtract (number) (1) : Nat)) + 1) < (number + number))) := by
  intros
  try set_option linter.unusedSimpArgs false in simp only [← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

mutual
@[expose, semireducible] public def ping (number : Nat) : Nat := (match (generalizing := false) __decrease0 : (Nat.beq (number) (0)) with | true => 0 | false => (pong (number) + 1))
termination_by ((number + number) + 1)
decreasing_by all_goals first | (have __evidence := ping_to_pong (number) (__decrease0); subst_vars; exact __evidence)

@[expose, semireducible] public def pong (number : Nat) : Nat := (match (generalizing := false) __decrease0 : (Nat.beq (number) (0)) with | true => 0 | false => (ping ((LexLeanRuntime.subtract (number) (1) : Nat)) + 1))
termination_by (number + number)
decreasing_by all_goals first | (have __evidence := pong_to_ping (number) (__decrease0); subst_vars; exact __evidence)
end

public theorem ping_seven : (ping (7) = 14) := by
  decide

end Coverage.Recur
