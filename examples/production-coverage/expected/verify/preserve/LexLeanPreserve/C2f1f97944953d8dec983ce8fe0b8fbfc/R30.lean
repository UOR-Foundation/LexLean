import LexLeanPreservation.Core
import LexLeanPreservation.Values
import LexLeanPreservation.Primitives
import LexLeanPreservation.Fixed
import LexLeanPreservation.Keys
import LexLeanPreservation.Templates
import Coverage.Colls
import Coverage.Main
import Coverage.Prims
import Coverage.RecRoots
import Coverage.Recur
import Coverage.Syntax
import Coverage.Types
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R30

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [{ constructors := [[.nat], [(.adt 0), (.adt 0)], [(.list (.adt 1)), (.adt 0)]] },
    { constructors := [[.string, (.adt 0)], [(.adt 1), (.adt 1)]] }], functions := [
    { parameters := [0], types := [(.adt 0)], result := .nat,
      body := (.call 1 [(.var 0)]) },
    { parameters := [0], types := [(.adt 0)], result := .nat,
      body := (.«match» .nat (.var 0) [(.arm (.adt 0) [1] (.value .nat (.nat 1))), (.arm (.adt 1) [2, 3] (.prim .natAdd [(.prim .natAdd [(.call 1 [(.var 2)]), (.call 1 [(.var 3)])]), (.value .nat (.nat 1))])), (.arm (.adt 2) [4, 5] (.prim .natAdd [(.call 2 [(.var 4)]), (.call 1 [(.var 5)])]))]) },
    { parameters := [0], types := [(.list (.adt 1))], result := .nat,
      body := (.«match» .nat (.var 0) [(.arm .nil [] (.value .nat (.nat 0))), (.arm .cons [1, 2] (.prim .natAdd [(.call 3 [(.var 1)]), (.call 2 [(.var 2)])]))]) },
    { parameters := [0], types := [(.adt 1)], result := .nat,
      body := (.«match» .nat (.var 0) [(.arm (.adt 0) [1, 2] (.prim .natAdd [(.call 1 [(.var 2)]), (.value .nat (.nat 1))])), (.arm (.adt 1) [3, 4] (.prim .natAdd [(.call 3 [(.var 3)]), (.call 3 [(.var 4)])]))]) }] }

mutual
def __enc_0 : (Coverage.Syntax.Expr) -> LexLeanTarget.TargetSyntax.Value
  | Coverage.Syntax.Expr.literal __x0 => LexLeanTarget.TargetSyntax.Value.adt 0 [(LexLeanTarget.TargetSyntax.Value.nat __x0)]
  | Coverage.Syntax.Expr.plus __x0 __x1 => LexLeanTarget.TargetSyntax.Value.adt 1 [(__enc_0 __x0), (__enc_0 __x1)]
  | Coverage.Syntax.Expr.block __x0 __x1 => LexLeanTarget.TargetSyntax.Value.adt 2 [(LexLeanTarget.TargetSyntax.Value.list (__items_0 __x0)), (__enc_0 __x1)]

def __enc_1 : (Coverage.Syntax.Stmt) -> LexLeanTarget.TargetSyntax.Value
  | Coverage.Syntax.Stmt.assign __x0 __x1 => LexLeanTarget.TargetSyntax.Value.adt 0 [(LexLeanTarget.TargetSyntax.Value.string __x0), (__enc_0 __x1)]
  | Coverage.Syntax.Stmt.sequence __x0 __x1 => LexLeanTarget.TargetSyntax.Value.adt 1 [(__enc_1 __x0), (__enc_1 __x1)]

def __items_0 : (List (Coverage.Syntax.Stmt)) -> List LexLeanTarget.TargetSyntax.Value
  | [] => []
  | __x0 :: __x1 => (__enc_1 __x0) :: __items_0 __x1

end

def __L_0 : LexLeanPreservation.ListEnc (Coverage.Syntax.Stmt) :=
  ⟨__enc_1, __items_0, __items_0.eq_1, __items_0.eq_2⟩

mutual
def __fits_1 : ∀ (expression : (Coverage.Syntax.Expr)), Bool
  | (Coverage.Syntax.Expr.literal value) => (true && true)
  | (Coverage.Syntax.Expr.plus left right) => (true && ((((((true && true) && (__fits_1 (left))) && (((true && true) && (__fits_1 (right))) && true)) && (Nat.blt ((Coverage.Recur.exprSize (left)) + (Coverage.Recur.exprSize (right))) 18446744073709551616)) && (true && true)) && (Nat.blt (((Coverage.Recur.exprSize (left)) + (Coverage.Recur.exprSize (right))) + (1 : Nat)) 18446744073709551616)))
  | (Coverage.Syntax.Expr.block statements result) => (true && ((((true && true) && (__fits_2 (statements))) && (((true && true) && (__fits_1 (result))) && true)) && (Nat.blt ((Coverage.Recur.statementsSize (statements)) + (Coverage.Recur.exprSize (result))) 18446744073709551616)))
termination_by structural expression => expression

def __fits_2 : ∀ (statements : (List (Coverage.Syntax.Stmt))), Bool
  | (List.nil) => (true && true)
  | (List.cons head tail) => (true && ((((true && true) && (__fits_3 (head))) && (((true && true) && (__fits_2 (tail))) && true)) && (Nat.blt ((Coverage.Recur.statementSize (head)) + (Coverage.Recur.statementsSize (tail))) 18446744073709551616)))
termination_by structural statements => statements

def __fits_3 : ∀ (statement : (Coverage.Syntax.Stmt)), Bool
  | (Coverage.Syntax.Stmt.assign target value) => (true && ((((true && true) && (__fits_1 (value))) && (true && true)) && (Nat.blt ((Coverage.Recur.exprSize (value)) + (1 : Nat)) 18446744073709551616)))
  | (Coverage.Syntax.Stmt.sequence first second) => (true && ((((true && true) && (__fits_3 (first))) && (((true && true) && (__fits_3 (second))) && true)) && (Nat.blt ((Coverage.Recur.statementSize (first)) + (Coverage.Recur.statementSize (second))) 18446744073709551616)))
termination_by structural statement => statement

end

mutual
theorem __rel_1 : ∀ (expression : (Coverage.Syntax.Expr)), LexLeanPreservation.FunRel __prog 1 [(__enc_0 expression)] (LexLeanPreservation.Rel (__fits_1 expression) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Recur.exprSize expression)))
  | (Coverage.Syntax.Expr.literal value) => by rw [__fits_1.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_hit rfl rfl LexLeanPreservation.conv_value))
  | (Coverage.Syntax.Expr.plus left right) => by rw [__fits_1.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_1 (left))) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_1 (right))) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd (Coverage.Recur.exprSize (left)) (Coverage.Recur.exprSize (right)))) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd ((Coverage.Recur.exprSize (left)) + (Coverage.Recur.exprSize (right))) (1 : Nat))))))
  | (Coverage.Syntax.Expr.block statements result) => by rw [__fits_1.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_2 (statements))) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_1 (result))) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd (Coverage.Recur.statementsSize (statements)) (Coverage.Recur.exprSize (result))))))))
termination_by structural expression => expression

theorem __rel_2 : ∀ (statements : (List (Coverage.Syntax.Stmt))), LexLeanPreservation.FunRel __prog 2 [((LexLeanPreservation.ListEnc.enc __L_0) statements)] (LexLeanPreservation.Rel (__fits_2 statements) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Recur.statementsSize statements)))
  | (List.nil) => by rw [__fits_2.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_hit rfl rfl LexLeanPreservation.conv_value))
  | (List.cons head tail) => by rw [__fits_2.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_3 (head))) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_2 (tail))) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd (Coverage.Recur.statementSize (head)) (Coverage.Recur.statementsSize (tail)))))))
termination_by structural statements => statements

theorem __rel_3 : ∀ (statement : (Coverage.Syntax.Stmt)), LexLeanPreservation.FunRel __prog 3 [(__enc_1 statement)] (LexLeanPreservation.Rel (__fits_3 statement) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Recur.statementSize statement)))
  | (Coverage.Syntax.Stmt.assign target value) => by rw [__fits_3.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_1 (value))) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd (Coverage.Recur.exprSize (value)) (1 : Nat)))))
  | (Coverage.Syntax.Stmt.sequence first second) => by rw [__fits_3.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_3 (first))) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_3 (second))) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd (Coverage.Recur.statementSize (first)) (Coverage.Recur.statementSize (second)))))))
termination_by structural statement => statement

end

def __fits_0 (e : (Coverage.Syntax.Expr)) : Bool :=
  ((true && true) && (__fits_1 (e)))

attribute [local irreducible] Coverage.Recur.exprSize in
theorem __rel_0 (e : (Coverage.Syntax.Expr)) : LexLeanPreservation.FunRel __prog 0 [(__enc_0 e)] (LexLeanPreservation.Rel (__fits_0 e) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.RecRoots.syntaxTotal e))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_1 (e)))

def denote (e : (Coverage.Syntax.Expr)) : LexLeanPreservation.Obs :=
  LexLeanPreservation.Rel (__fits_0 e) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.RecRoots.syntaxTotal e))

theorem root (e : (Coverage.Syntax.Expr)) : LexLeanPreservation.RunConv __prog 0 [(__enc_0 e)] (LexLeanPreservation.Rel (__fits_0 e) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.RecRoots.syntaxTotal e))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 e)

end LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R30
