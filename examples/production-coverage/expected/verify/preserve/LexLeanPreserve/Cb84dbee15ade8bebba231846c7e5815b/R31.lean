import LexLeanPreservation.Core
import LexLeanPreservation.Values
import LexLeanPreservation.Primitives
import LexLeanPreservation.Fixed
import LexLeanPreservation.Keys
import LexLeanPreservation.Templates
import LexLeanPreservation.Validate
import Coverage.Boundary
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
namespace LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R31

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
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
def __enc_0 : (Coverage.Syntax.Expr) -> _root_.LexLeanTarget.TargetSyntax.Value
  | Coverage.Syntax.Expr.literal __x0 => _root_.LexLeanTarget.TargetSyntax.Value.adt 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat __x0)]
  | Coverage.Syntax.Expr.plus __x0 __x1 => _root_.LexLeanTarget.TargetSyntax.Value.adt 1 [(__enc_0 __x0), (__enc_0 __x1)]
  | Coverage.Syntax.Expr.block __x0 __x1 => _root_.LexLeanTarget.TargetSyntax.Value.adt 2 [(_root_.LexLeanTarget.TargetSyntax.Value.list (__items_0 __x0)), (__enc_0 __x1)]

def __enc_1 : (Coverage.Syntax.Stmt) -> _root_.LexLeanTarget.TargetSyntax.Value
  | Coverage.Syntax.Stmt.assign __x0 __x1 => _root_.LexLeanTarget.TargetSyntax.Value.adt 0 [(_root_.LexLeanTarget.TargetSyntax.Value.string __x0), (__enc_0 __x1)]
  | Coverage.Syntax.Stmt.sequence __x0 __x1 => _root_.LexLeanTarget.TargetSyntax.Value.adt 1 [(__enc_1 __x0), (__enc_1 __x1)]

def __items_0 : (List (Coverage.Syntax.Stmt)) -> List _root_.LexLeanTarget.TargetSyntax.Value
  | [] => []
  | __x0 :: __x1 => (__enc_1 __x0) :: __items_0 __x1

end

def __L_0 : _root_.LexLeanPreservation.ListEnc (Coverage.Syntax.Stmt) :=
  ⟨__enc_1, __items_0, __items_0.eq_1, __items_0.eq_2⟩

mutual
def __fits_1 : ∀ (expression : (Coverage.Syntax.Expr)), Bool
  | (Coverage.Syntax.Expr.literal value) => (Bool.true && Bool.true)
  | (Coverage.Syntax.Expr.plus left right) => (Bool.true && ((((((Bool.true && Bool.true) && (__fits_1 (left))) && (((Bool.true && Bool.true) && (__fits_1 (right))) && Bool.true)) && (Nat.blt ((Coverage.Recur.exprSize (left)) + (Coverage.Recur.exprSize (right))) 18446744073709551616)) && (Bool.true && Bool.true)) && (Nat.blt (((Coverage.Recur.exprSize (left)) + (Coverage.Recur.exprSize (right))) + (1 : Nat)) 18446744073709551616)))
  | (Coverage.Syntax.Expr.block statements result) => (Bool.true && ((((Bool.true && Bool.true) && (__fits_2 (statements))) && (((Bool.true && Bool.true) && (__fits_1 (result))) && Bool.true)) && (Nat.blt ((Coverage.Recur.statementsSize (statements)) + (Coverage.Recur.exprSize (result))) 18446744073709551616)))
termination_by structural expression => expression

def __fits_2 : ∀ (statements : (List (Coverage.Syntax.Stmt))), Bool
  | (List.nil) => (Bool.true && Bool.true)
  | (List.cons head tail) => (Bool.true && ((((Bool.true && Bool.true) && (__fits_3 (head))) && (((Bool.true && Bool.true) && (__fits_2 (tail))) && Bool.true)) && (Nat.blt ((Coverage.Recur.statementSize (head)) + (Coverage.Recur.statementsSize (tail))) 18446744073709551616)))
termination_by structural statements => statements

def __fits_3 : ∀ (statement : (Coverage.Syntax.Stmt)), Bool
  | (Coverage.Syntax.Stmt.assign target value) => (Bool.true && ((((Bool.true && Bool.true) && (__fits_1 (value))) && (Bool.true && Bool.true)) && (Nat.blt ((Coverage.Recur.exprSize (value)) + (1 : Nat)) 18446744073709551616)))
  | (Coverage.Syntax.Stmt.sequence first second) => (Bool.true && ((((Bool.true && Bool.true) && (__fits_3 (first))) && (((Bool.true && Bool.true) && (__fits_3 (second))) && Bool.true)) && (Nat.blt ((Coverage.Recur.statementSize (first)) + (Coverage.Recur.statementSize (second))) 18446744073709551616)))
termination_by structural statement => statement

end

mutual
theorem __rel_1 : ∀ (expression : (Coverage.Syntax.Expr)), _root_.LexLeanPreservation.FunRel __prog 1 [(__enc_0 expression)] (_root_.LexLeanPreservation.Rel (__fits_1 expression) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Recur.exprSize expression)))
  | (Coverage.Syntax.Expr.literal value) => by rw [__fits_1.eq_def]; exact _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_match (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convA_hit rfl rfl _root_.LexLeanPreservation.conv_value))
  | (Coverage.Syntax.Expr.plus left right) => by rw [__fits_1.eq_def]; exact _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_match (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convA_miss rfl (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__rel_1 (left))) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__rel_1 (right))) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd (Coverage.Recur.exprSize (left)) (Coverage.Recur.exprSize (right)))) (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd ((Coverage.Recur.exprSize (left)) + (Coverage.Recur.exprSize (right))) (1 : Nat))))))
  | (Coverage.Syntax.Expr.block statements result) => by rw [__fits_1.eq_def]; exact _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_match (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convA_miss rfl (_root_.LexLeanPreservation.convA_miss rfl (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__rel_2 (statements))) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__rel_1 (result))) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd (Coverage.Recur.statementsSize (statements)) (Coverage.Recur.exprSize (result))))))))
termination_by structural expression => expression

theorem __rel_2 : ∀ (statements : (List (Coverage.Syntax.Stmt))), _root_.LexLeanPreservation.FunRel __prog 2 [((_root_.LexLeanPreservation.ListEnc.enc __L_0) statements)] (_root_.LexLeanPreservation.Rel (__fits_2 statements) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Recur.statementsSize statements)))
  | (List.nil) => by rw [__fits_2.eq_def]; exact _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_match (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convA_hit rfl rfl _root_.LexLeanPreservation.conv_value))
  | (List.cons head tail) => by rw [__fits_2.eq_def]; exact _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_match (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convA_miss rfl (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__rel_3 (head))) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__rel_2 (tail))) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd (Coverage.Recur.statementSize (head)) (Coverage.Recur.statementsSize (tail)))))))
termination_by structural statements => statements

theorem __rel_3 : ∀ (statement : (Coverage.Syntax.Stmt)), _root_.LexLeanPreservation.FunRel __prog 3 [(__enc_1 statement)] (_root_.LexLeanPreservation.Rel (__fits_3 statement) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Recur.statementSize statement)))
  | (Coverage.Syntax.Stmt.assign target value) => by rw [__fits_3.eq_def]; exact _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_match (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__rel_1 (value))) (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd (Coverage.Recur.exprSize (value)) (1 : Nat)))))
  | (Coverage.Syntax.Stmt.sequence first second) => by rw [__fits_3.eq_def]; exact _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_match (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convA_miss rfl (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__rel_3 (first))) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__rel_3 (second))) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd (Coverage.Recur.statementSize (first)) (Coverage.Recur.statementSize (second)))))))
termination_by structural statement => statement

end

def __fits_0 (e : (Coverage.Syntax.Expr)) : Bool :=
  ((Bool.true && Bool.true) && (__fits_1 (e)))

attribute [local irreducible] Coverage.Recur.exprSize in
theorem __rel_0 (e : (Coverage.Syntax.Expr)) : _root_.LexLeanPreservation.FunRel __prog 0 [(__enc_0 e)] (_root_.LexLeanPreservation.Rel (__fits_0 e) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.RecRoots.syntaxTotal e))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__rel_1 (e)))

def denote (e : (Coverage.Syntax.Expr)) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 e) (_root_.LexLeanPreservation.Obs.value ((_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.RecRoots.syntaxTotal e)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (e : (Coverage.Syntax.Expr)) : _root_.LexLeanPreservation.RunConv __prog 0 [(__enc_0 e)] (_root_.LexLeanPreservation.Rel (__fits_0 e) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.RecRoots.syntaxTotal e))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 e)

end LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R31
