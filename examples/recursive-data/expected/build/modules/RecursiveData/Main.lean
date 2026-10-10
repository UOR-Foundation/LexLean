module
public import Init
public import RecursiveData.Types
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace RecursiveData.Main

@[expose] public def treeSize : (tree : RecursiveData.Types.Tree (Nat)) -> Nat
  | RecursiveData.Types.Tree.leaf => 0
  | RecursiveData.Types.Tree.node left _ right => ((treeSize (left) + 1) + treeSize (right))

@[expose] public def mirror : (tree : RecursiveData.Types.Tree (Nat)) -> RecursiveData.Types.Tree (Nat)
  | RecursiveData.Types.Tree.leaf => RecursiveData.Types.Tree.leaf
  | RecursiveData.Types.Tree.node left value right => RecursiveData.Types.Tree.node (mirror (right)) (value) (mirror (left))

@[expose] public def sampleTree : RecursiveData.Types.Tree (Nat) := RecursiveData.Types.Tree.node (RecursiveData.Types.Tree.node (RecursiveData.Types.Tree.leaf) (1) (RecursiveData.Types.Tree.leaf)) (2) (RecursiveData.Types.Tree.leaf)

public theorem sample_tree_size : (treeSize (sampleTree) = 2) := by
  decide

public theorem mirror_involutive (tree : RecursiveData.Types.Tree (Nat)) : (mirror (mirror (tree)) = tree) := by
  induction tree with
  | leaf =>
    simp only [mirror]
  | node left value right leftHypothesis rightHypothesis =>
    simp only [leftHypothesis, mirror, rightHypothesis]

@[expose] public def roseLabel (rose : RecursiveData.Types.Rose (Nat)) : Nat := (match rose with | RecursiveData.Types.Rose.node label _ => label)

public theorem rose_label : (roseLabel (RecursiveData.Types.Rose.node (7) (([] : List (RecursiveData.Types.Rose (Nat))))) = 7) := by
  rfl

@[expose] public def isLiteral (expression : RecursiveData.Types.Expr) : Bool := (match expression with | RecursiveData.Types.Expr.literal _ => true | RecursiveData.Types.Expr.plus _ _ => false | RecursiveData.Types.Expr.block _ _ => false)

public theorem block_is_not_literal : (isLiteral (RecursiveData.Types.Expr.block ((RecursiveData.Types.Stmt.assign ("x") (RecursiveData.Types.Expr.literal (1)) :: ([] : List (RecursiveData.Types.Stmt)))) (RecursiveData.Types.Expr.literal (0))) = false) := by
  rfl

@[expose] public def outcomeOrZero (outcome : RecursiveData.Types.Outcome (String) (Nat)) : Nat := (match outcome with | RecursiveData.Types.Outcome.failure _ => 0 | RecursiveData.Types.Outcome.success value => value)

public theorem success_value : (outcomeOrZero (RecursiveData.Types.Outcome.success (5)) = 5) := by
  rfl

@[expose] public def swap (pair : (Prod (Nat) (Bool))) : (Prod (Bool) (Nat)) := ((pair).2, (pair).1)

@[expose] public def sumPair (pair : (Prod (Nat) (Nat))) : Nat := (match pair with | Prod.mk left right => (left + right))

public theorem swap_twice_first : ((swap ((3, true))).1 = true) := by
  rfl

public theorem sum_pair : (sumPair ((3, 4)) = 7) := by
  rfl

public theorem located_row : (((({ value := true, position := (1, 2) } : RecursiveData.Types.Located (Bool))).position).1 = 1) := by
  rfl

end RecursiveData.Main
