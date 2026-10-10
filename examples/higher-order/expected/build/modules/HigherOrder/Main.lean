module
public import Init
public import HigherOrder.Combinators
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace HigherOrder.Main

@[expose] public def addAll (offset : Nat) (values : List (Nat)) : List (Nat) := HigherOrder.Combinators.mapList (Nat) (Nat) ((fun (value : Nat) => (value + offset))) (values)

@[expose] public def total (values : List (Nat)) : Nat := HigherOrder.Combinators.foldList (Nat) (Nat) ((fun (sum : Nat) (value : Nat) => (sum + value))) (0) (values)

@[expose] public def bumpAll (values : List (Nat)) : List (Nat) := HigherOrder.Combinators.mapList (Nat) (Nat) ((HigherOrder.Combinators.increment)) (values)

@[expose] public def addThenDouble : ((Nat) -> (Nat)) := HigherOrder.Combinators.compose (Nat) (Nat) (Nat) ((fun (value : Nat) => (value + value))) ((HigherOrder.Combinators.increment))

@[expose] public def evaluator : HigherOrder.Combinators.Visitor := ({ onLiteral := (fun (value : Nat) => value), onPlus := (fun (left : Nat) (right : Nat) => (left + right)), onTwice := (fun (value : Nat) => (value + value)) } : HigherOrder.Combinators.Visitor)

public theorem add_all : (addAll (10) ((1 :: (2 :: (3 :: ([] : List (Nat)))))) = (11 :: (12 :: (13 :: ([] : List (Nat)))))) := by
  rfl

public theorem total_sum : (total ((1 :: (2 :: (3 :: (4 :: ([] : List (Nat))))))) = 10) := by
  rfl

public theorem bump_all : (bumpAll ((0 :: (4 :: ([] : List (Nat))))) = (1 :: (5 :: ([] : List (Nat))))) := by
  rfl

public theorem composed : ((addThenDouble (4)) = 10) := by
  rfl

public theorem visitor_agrees : (HigherOrder.Combinators.visit (evaluator) (HigherOrder.Combinators.Arith.plus (HigherOrder.Combinators.Arith.literal (2)) (HigherOrder.Combinators.Arith.twice (HigherOrder.Combinators.Arith.literal (3)))) = HigherOrder.Combinators.evaluate (HigherOrder.Combinators.Arith.plus (HigherOrder.Combinators.Arith.literal (2)) (HigherOrder.Combinators.Arith.twice (HigherOrder.Combinators.Arith.literal (3))))) := by
  rfl

public theorem map_identity (Input : Type) (values : List (Input)) : (HigherOrder.Combinators.mapList (Input) (Input) ((fun (value : Input) => value)) (values) = values) := by
  induction values with
  | nil =>
    simp only [HigherOrder.Combinators.mapList]
  | cons head tail hypothesis =>
    simp only [hypothesis, HigherOrder.Combinators.mapList]

public theorem map_identity_numbers : (HigherOrder.Combinators.mapList (Nat) (Nat) ((fun (value : Nat) => value)) ((7 :: (8 :: ([] : List (Nat))))) = (7 :: (8 :: ([] : List (Nat))))) := by
  exact map_identity (Nat) ((7 :: (8 :: ([] : List (Nat)))))

end HigherOrder.Main
