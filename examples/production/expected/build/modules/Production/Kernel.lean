module
public import Init
public import LexLeanRuntime.Portable
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Production.Kernel

public inductive Shape where
  | circle (_ : Nat)
  | rectangle (_ : Nat) (_ : Nat)

@[expose] public def area (shape : Shape) : Nat := (match shape with | Shape.circle radius => (LexLeanRuntime.multiply ((LexLeanRuntime.multiply (radius) (radius) : Nat)) (3) : Nat) | Shape.rectangle width height => (LexLeanRuntime.multiply (width) (height) : Nat))

@[expose] public def applyTwice (step : ((Nat) -> (Nat))) (value : Nat) : Nat := (step ((step (value))))

@[expose] public def total : (values : List (Nat)) -> Nat
  | List.nil => 0
  | List.cons head tail => (head + total (tail))

@[expose] public def firstOr (Item : Type) (values : List (Item)) (fallback : Item) : Item := (match values with | List.nil => fallback | List.cons head _ => head)

public theorem countdown_decreases (number : Nat) (_steps : Nat) : (((Nat.blt (number) (2)) = false) -> ((LexLeanRuntime.subtract (number) (2) : Nat) < number)) := by
  intros
  try set_option linter.unusedSimpArgs false in simp only [← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose, semireducible] public def countdown (number : Nat) (steps : Nat) : Nat := (match (generalizing := false) __decrease0 : (Nat.blt (number) (2)) with | true => steps | false => countdown ((LexLeanRuntime.subtract (number) (2) : Nat)) ((steps + 1)))
termination_by number
decreasing_by all_goals first | (have __evidence := countdown_decreases (number) (steps) (__decrease0); subst_vars; exact __evidence)

@[expose, reducible] public def positive (value : Nat) : Prop := (0 < value)

public theorem area_square : (area (Shape.rectangle (3) (3)) = 9) := by
  decide

end Production.Kernel
