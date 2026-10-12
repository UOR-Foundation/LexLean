module
public meta import Lean
set_option autoImplicit false
set_option linter.unusedVariables false
universe u_1 u_2

public meta partial def Core.sum (xs : (List Nat)) : Nat :=
  (match xs with | .nil => 0 | .cons h t => (Nat.add h (Core.sum t)))

public meta partial def Core.size {α : Type} (xs : (List α)) : Nat :=
  (match xs with | .nil => 0 | .cons h t => (Nat.add 1 (Core.size (α := α) t)))

public meta def Core.describe (n : Nat) : String :=
  (let big : Bool := (Nat.ble 10 n); (String.append (if big then "big" else "small") "!"))

public meta def Core.double (xs : (List Nat)) : (List Nat) :=
  (List.map (α := Nat) (β := Nat) (fun (n : Nat) => (Nat.add n n)) xs)

public meta def Core.first (p : (Prod Nat String)) : Nat :=
  (p).fst

public meta def Core.pair : (Prod Nat String) :=
  (Prod.mk (α := Nat) (β := String) 1 "a")

public meta def Core.array : (Array Nat) :=
  ((#[1, 2, 3] : Array Nat))

public meta def Core.list : (List Nat) :=
  (([1, 2] : List Nat))

public meta def Core.twice (f : (Nat → Nat)) (n : Nat) : Nat :=
  (f (f n))

public meta def Core.orZero (o : (Option Nat)) : Nat :=
  (match o with | .none => 0 | .some v => v)

public meta def Core.isSmall (n : Nat) : Bool :=
  (match n with | .zero => (Bool.true) | _ => (Bool.false))

public meta def Core.predecessor (n : Nat) : Nat :=
  (Nat.pred n)

mutual
public meta partial def Core.isEven (n : Nat) : Bool :=
  (match n with | .zero => (Bool.true) | .succ m => (Core.isOdd m))

public meta partial def Core.isOdd (n : Nat) : Bool :=
  (match n with | .zero => (Bool.false) | .succ m => (Core.isEven m))
end
