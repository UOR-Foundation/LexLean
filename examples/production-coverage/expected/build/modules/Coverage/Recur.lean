module
public import Init
public import LexLeanRuntime.Portable
public import Coverage.Syntax
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Coverage.Recur

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
  try set_option linter.unusedSimpArgs false in simp only [← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose, semireducible] public def countdown (number : Nat) (steps : Nat) : Nat := (match (generalizing := false) __decrease0 : (Nat.blt (number) (2)) with | true => steps | false => countdown ((LexLeanRuntime.subtract (number) (2) : Nat)) ((steps + 1)))
termination_by number
decreasing_by all_goals first | (have __evidence := countdown_decreases (number) (steps) (__decrease0); subst_vars; exact __evidence)

public theorem reduce_decreases (value : Nat) (bound : Nat) : (((Nat.beq (bound) (0)) = false) -> (((Nat.blt (value) (bound)) = false) -> ((LexLeanRuntime.subtract (value) (bound) : Nat) < value))) := by
  intros
  try set_option linter.unusedSimpArgs false in simp only [← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose, semireducible] public def reduce (value : Nat) (bound : Nat) : Nat := (match (generalizing := false) __decrease0 : (Nat.beq (bound) (0)) with | true => value | false => (match (generalizing := false) __decrease1 : (Nat.blt (value) (bound)) with | true => value | false => reduce ((LexLeanRuntime.subtract (value) (bound) : Nat)) (bound)))
termination_by value
decreasing_by all_goals first | (have __evidence := reduce_decreases (value) (bound) (__decrease0) (__decrease1); subst_vars; exact __evidence)

public theorem search_decreases (target : Nat) (low : Nat) (high : Nat) : (((Nat.blt (low) (high)) = true) -> (((Nat.ble (target) ((low + low))) = false) -> ((LexLeanRuntime.subtract (high) ((low + 1)) : Nat) < (LexLeanRuntime.subtract (high) (low) : Nat)))) := by
  intros
  try set_option linter.unusedSimpArgs false in simp only [← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
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
  try set_option linter.unusedSimpArgs false in simp only [weight, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem reassociate_plus (term : Coverage.Syntax.Term) (left : Coverage.Syntax.Term) (right : Coverage.Syntax.Term) (inner : Coverage.Syntax.Term) (rest : Coverage.Syntax.Term) : ((term = Coverage.Syntax.Term.plus (left) (right)) -> ((left = Coverage.Syntax.Term.plus (inner) (rest)) -> (weight (Coverage.Syntax.Term.plus (inner) (Coverage.Syntax.Term.plus (rest) (right))) < weight (term)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [weight, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

@[expose, semireducible] public def reassociate (term : Coverage.Syntax.Term) : Coverage.Syntax.Term := (match (generalizing := false) __decrease0 : term with | Coverage.Syntax.Term.literal value => Coverage.Syntax.Term.literal (value) | Coverage.Syntax.Term.plus left right => (match (generalizing := false) __decrease1 : left with | Coverage.Syntax.Term.literal value => Coverage.Syntax.Term.plus (Coverage.Syntax.Term.literal (value)) (reassociate (right)) | Coverage.Syntax.Term.plus inner rest => reassociate (Coverage.Syntax.Term.plus (inner) (Coverage.Syntax.Term.plus (rest) (right)))))
termination_by weight (term)
decreasing_by all_goals first | (have __evidence := reassociate_literal (term) _ _ _ (__decrease0) (__decrease1); subst_vars; exact __evidence) | (have __evidence := reassociate_plus (term) _ _ _ _ (__decrease0) (__decrease1); subst_vars; exact __evidence)

public theorem prune_plus (term : Coverage.Syntax.Term) (left : Coverage.Syntax.Term) (right : Coverage.Syntax.Term) (inner : Coverage.Syntax.Term) (rest : Coverage.Syntax.Term) : ((term = Coverage.Syntax.Term.plus (left) (right)) -> ((left = Coverage.Syntax.Term.plus (inner) (rest)) -> (weight (Coverage.Syntax.Term.plus (inner) (right)) < weight (term)))) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [weight, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
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
  try set_option linter.unusedSimpArgs false in simp only [← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem pong_to_ping (number : Nat) : (((Nat.beq (number) (0)) = false) -> ((((LexLeanRuntime.subtract (number) (1) : Nat) + (LexLeanRuntime.subtract (number) (1) : Nat)) + 1) < (number + number))) := by
  intros
  try set_option linter.unusedSimpArgs false in simp only [← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
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
