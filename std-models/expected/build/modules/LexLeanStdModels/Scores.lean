module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanRuntime.Models
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanStdModels.Scores

@[expose] public def leaderStep (state : (Prod (Nat) ((Prod (Nat) (Int))))) (score : Int) : (Prod (Nat) ((Prod (Nat) (Int)))) := (if (Nat.beq ((state).1) (0)) then (1, (0, score)) else (if (LexLeanCollections.lessThan (((state).2).2) (score) : Bool) then (((state).1 + 1), ((state).1, score)) else (((state).1 + 1), (state).2)))

@[expose] public def best (scores : List (Int)) : Nat := (((LexLeanCollections.listFold ((fun (state : (Prod (Nat) ((Prod (Nat) (Int))))) (score : Int) => leaderStep (state) (score))) ((0, (0, (0 : Int)))) (scores) : (Prod (Nat) ((Prod (Nat) (Int)))))).2).1

@[expose] public def runnerStep (state : (Prod (Nat) ((Prod (Int) (Int))))) (score : Int) : (Prod (Nat) ((Prod (Int) (Int)))) := (if (Nat.beq ((state).1) (0)) then (1, (score, (0 : Int))) else (if (Nat.beq ((state).1) (1)) then (if (LexLeanCollections.lessThan (((state).2).1) (score) : Bool) then (2, (score, ((state).2).1)) else (2, (((state).2).1, score))) else (if (LexLeanCollections.lessThan (((state).2).1) (score) : Bool) then (((state).1 + 1), (score, ((state).2).1)) else (((state).1 + 1), (((state).2).1, (if (LexLeanCollections.lessThan (((state).2).2) (score) : Bool) then score else ((state).2).2))))))

@[expose] public def leaders (scores : List (Int)) : (Prod (Int) (Int)) := ((LexLeanCollections.listFold ((fun (state : (Prod (Nat) ((Prod (Int) (Int))))) (score : Int) => runnerStep (state) (score))) ((0, ((0 : Int), (0 : Int)))) (scores) : (Prod (Nat) ((Prod (Int) (Int)))))).2

@[expose] public def margin (scores : List (Int)) : Int := (if (Nat.blt ((LexLeanRuntime.length (scores) : Nat)) (2)) then (0 : Int) else (LexLeanRuntime.subtract ((leaders (scores)).1) ((leaders (scores)).2) : Int))

@[expose] public def decideOr (threshold : Int) (scores : List (Int)) : Option (Nat) := (if (Nat.blt ((LexLeanRuntime.length (scores) : Nat)) (2)) then Option.none else (if (LexLeanCollections.lessThan (margin (scores)) (threshold) : Bool) then Option.none else Option.some (best (scores))))

public theorem best_first_maximal : (best (((3 : Int) :: ((9 : Int) :: ((9 : Int) :: ((4 : Int) :: ([] : List (Int))))))) = 1) := by
  decide

public theorem margin_example : (margin (((3 : Int) :: ((9 : Int) :: ((4 : Int) :: ([] : List (Int)))))) = (5 : Int)) := by
  decide

public theorem margin_tie : (margin (((5 : Int) :: ((5 : Int) :: ([] : List (Int))))) = (0 : Int)) := by
  decide

public theorem decide_or_commits : (decideOr ((2 : Int)) (((3 : Int) :: ((9 : Int) :: ((4 : Int) :: ([] : List (Int)))))) = Option.some (1)) := by
  decide

public theorem decide_or_tie_abstains : (decideOr ((1 : Int)) (((5 : Int) :: ((5 : Int) :: ([] : List (Int))))) = Option.none) := by
  decide

public theorem decide_or_single_abstains : (decideOr ((0 : Int)) (((7 : Int) :: ([] : List (Int)))) = Option.none) := by
  decide

public theorem decide_or_empty_abstains : (decideOr ((0 : Int)) (([] : List (Int))) = Option.none) := by
  decide

public inductive Candidate (T : Type) where
  | confident (_ : T)
  | unsure (_ : T)

@[expose] public def isConfident (T : Type) (candidate : Candidate (T)) : Bool := (match candidate with | Candidate.confident _ => true | Candidate.unsure _ => false)

@[expose] public def verdictOf (T : Type) (candidate : Candidate (T)) : Option (T) := (match candidate with | Candidate.confident value => Option.some (value) | Candidate.unsure _ => Option.none)

@[expose] public def isCommitted (T : Type) (verdict : Option (T)) : Bool := (match verdict with | Option.some _ => true | Option.none => false)

@[expose, reducible] public def Faithful (T : Type) (candidate : Candidate (T)) (verdict : Option (T)) : Prop := (isCommitted (T) (verdict) = isConfident (T) (candidate))

@[expose] public def Gate (T : Type) (candidate : LexLeanStdModels.Scores.Candidate (T)) : Option (T) := LexLeanStdModels.Scores.verdictOf (T) (candidate)

public theorem gate_faithful (T : Type) (candidate : Candidate (T)) : Faithful (T) (candidate) (Gate (T) (candidate)) := by
  cases candidate with
  | confident value =>
    simp only [Faithful, Gate, isCommitted, isConfident, verdictOf]
  | unsure value =>
    simp only [Faithful, Gate, isCommitted, isConfident, verdictOf]


public theorem GateEvidence.gate_faithful (T : Type) : LexLeanModels.SatisfiesTotal ((LexLeanStdModels.Scores.Faithful (T))) ((LexLeanStdModels.Scores.Gate (T))) := LexLeanStdModels.Scores.gate_faithful (T)

@[expose] public def GateModel (T : Type) (__input : LexLeanStdModels.Scores.Candidate (T)) : Option (T) := LexLeanStdModels.Scores.Gate (T) (__input)

end LexLeanStdModels.Scores
