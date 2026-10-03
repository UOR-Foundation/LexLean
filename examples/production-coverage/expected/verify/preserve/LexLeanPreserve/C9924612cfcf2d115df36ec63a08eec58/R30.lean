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
namespace LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R30

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [{ constructors := [[.nat, (.list (.adt 0))]] }], functions := [
    { parameters := [0], types := [.nat], result := .nat,
      body := (.call 1 [(.build (.adt 0) (.adt 0) [(.var 0), (.build .cons (.list (.adt 0)) [(.build (.adt 0) (.adt 0) [(.value .nat (.nat 1)), (.build .nil (.list (.adt 0)) [])]), (.build .nil (.list (.adt 0)) [])])])]) },
    { parameters := [0], types := [(.adt 0)], result := .nat,
      body := (.«match» .nat (.var 0) [(.arm (.adt 0) [1, 2] (.prim .natAdd [(.value .nat (.nat 1)), (.call 2 [(.var 2)])]))]) },
    { parameters := [0], types := [(.list (.adt 0))], result := .nat,
      body := (.«match» .nat (.var 0) [(.arm .nil [] (.value .nat (.nat 0))), (.arm .cons [1, 2] (.prim .natAdd [(.call 1 [(.var 1)]), (.call 2 [(.var 2)])]))]) }] }

mutual
def __enc_0 : (Coverage.Syntax.Rose Nat) -> LexLeanTarget.TargetSyntax.Value
  | Coverage.Syntax.Rose.node __x0 __x1 => LexLeanTarget.TargetSyntax.Value.adt 0 [(LexLeanTarget.TargetSyntax.Value.nat __x0), (LexLeanTarget.TargetSyntax.Value.list (__items_0 __x1))]

def __items_0 : (List (Coverage.Syntax.Rose Nat)) -> List LexLeanTarget.TargetSyntax.Value
  | [] => []
  | __x0 :: __x1 => (__enc_0 __x0) :: __items_0 __x1

end

def __L_0 : LexLeanPreservation.ListEnc (Coverage.Syntax.Rose Nat) :=
  ⟨__enc_0, __items_0, __items_0.eq_1, __items_0.eq_2⟩

mutual
def __fits_1 : ∀ (rose : (Coverage.Syntax.Rose Nat)), Bool
  | (Coverage.Syntax.Rose.node label children) => (true && ((true && (((true && true) && (__fits_2 (children))) && true)) && (Nat.blt ((1 : Nat) + (Coverage.Recur.forestSize Nat (children))) 18446744073709551616)))
termination_by structural rose => rose

def __fits_2 : ∀ (forest : (List (Coverage.Syntax.Rose Nat))), Bool
  | (List.nil) => (true && true)
  | (List.cons head tail) => (true && ((((true && true) && (__fits_1 (head))) && (((true && true) && (__fits_2 (tail))) && true)) && (Nat.blt ((Coverage.Recur.roseSize Nat (head)) + (Coverage.Recur.forestSize Nat (tail))) 18446744073709551616)))
termination_by structural forest => forest

end

mutual
theorem __rel_1 : ∀ (rose : (Coverage.Syntax.Rose Nat)), LexLeanPreservation.FunRel __prog 1 [(__enc_0 rose)] (LexLeanPreservation.Rel (__fits_1 rose) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Recur.roseSize Nat rose)))
  | (Coverage.Syntax.Rose.node label children) => by rw [__fits_1.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_2 (children))) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd (1 : Nat) (Coverage.Recur.forestSize Nat (children))))))
termination_by structural rose => rose

theorem __rel_2 : ∀ (forest : (List (Coverage.Syntax.Rose Nat))), LexLeanPreservation.FunRel __prog 2 [((LexLeanPreservation.ListEnc.enc __L_0) forest)] (LexLeanPreservation.Rel (__fits_2 forest) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Recur.forestSize Nat forest)))
  | (List.nil) => by rw [__fits_2.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_hit rfl rfl LexLeanPreservation.conv_value))
  | (List.cons head tail) => by rw [__fits_2.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_1 (head))) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_2 (tail))) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd (Coverage.Recur.roseSize Nat (head)) (Coverage.Recur.forestSize Nat (tail)))))))
termination_by structural forest => forest

end

def __fits_0 (n : Nat) : Bool :=
  ((((true && (((((true && ((true && true) && true)) && true) && ((true && true) && true)) && true) && true)) && true) && true) && (__fits_1 ((Coverage.Syntax.Rose.node (n) (((Coverage.Syntax.Rose.node ((1 : Nat)) (([] : List (Coverage.Syntax.Rose Nat))) : (Coverage.Syntax.Rose Nat)) :: ([] : List (Coverage.Syntax.Rose Nat)))) : (Coverage.Syntax.Rose Nat)))))

attribute [local irreducible] Coverage.Recur.roseSize in
theorem __rel_0 (n : Nat) : LexLeanPreservation.FunRel __prog 0 [(LexLeanTarget.TargetSyntax.Value.nat n)] (LexLeanPreservation.Rel (__fits_0 n) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.RecRoots.roseTotal n))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_nil) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_adt) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_nil) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_cons) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_adt) LexLeanPreservation.convL_nil) (__rel_1 ((Coverage.Syntax.Rose.node (n) (((Coverage.Syntax.Rose.node ((1 : Nat)) (([] : List (Coverage.Syntax.Rose Nat))) : (Coverage.Syntax.Rose Nat)) :: ([] : List (Coverage.Syntax.Rose Nat)))) : (Coverage.Syntax.Rose Nat)))))

def denote (n : Nat) : LexLeanPreservation.Obs :=
  LexLeanPreservation.Rel (__fits_0 n) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.RecRoots.roseTotal n))

theorem root (n : Nat) : LexLeanPreservation.RunConv __prog 0 [(LexLeanTarget.TargetSyntax.Value.nat n)] (LexLeanPreservation.Rel (__fits_0 n) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.RecRoots.roseTotal n))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 n)

end LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R30
