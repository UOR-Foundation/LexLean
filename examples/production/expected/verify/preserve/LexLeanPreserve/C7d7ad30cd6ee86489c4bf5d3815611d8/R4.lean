import LexLeanPreservation.Core
import LexLeanPreservation.Values
import LexLeanPreservation.Primitives
import LexLeanPreservation.Fixed
import LexLeanPreservation.Keys
import LexLeanPreservation.Templates
import Production.Kernel
import Production.Main
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.C7d7ad30cd6ee86489c4bf5d3815611d8.R4

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [(.list .nat)], result := .nat,
      body := (.call 1 [(.var 0)]) },
    { parameters := [0], types := [(.list .nat)], result := .nat,
      body := (.«match» .nat (.var 0) [(.arm .nil [] (.value .nat (.nat 0))), (.arm .cons [1, 2] (.prim .natAdd [(.var 1), (.call 1 [(.var 2)])]))]) }] }

def __fits_1 : ∀ (values : (List Nat)), Bool
  | (List.nil) => (true && true)
  | (List.cons head tail) => (true && ((true && (((true && true) && (__fits_1 (tail))) && true)) && (Nat.blt (head + (Production.Kernel.total (tail))) 18446744073709551616)))

theorem __rel_1 : ∀ (values : (List Nat)), LexLeanPreservation.FunRel __prog 1 [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) values)] (LexLeanPreservation.Rel (__fits_1 values) (LexLeanTarget.TargetSyntax.Value.nat (Production.Kernel.total values)))
  | (List.nil) => by rw [__fits_1.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_hit rfl rfl LexLeanPreservation.conv_value))
  | (List.cons head tail) => by rw [__fits_1.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_1 (tail))) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd head (Production.Kernel.total (tail)))))))

def __fits_0 (values : (List Nat)) : Bool :=
  ((true && true) && (__fits_1 (values)))

attribute [local irreducible] Production.Kernel.total in
theorem __rel_0 (values : (List Nat)) : LexLeanPreservation.FunRel __prog 0 [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) values)] (LexLeanPreservation.Rel (__fits_0 values) (LexLeanTarget.TargetSyntax.Value.nat (Production.Main.sumAll values))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_1 (values)))

def denote (values : (List Nat)) : LexLeanPreservation.Obs :=
  LexLeanPreservation.Rel (__fits_0 values) (LexLeanTarget.TargetSyntax.Value.nat (Production.Main.sumAll values))

theorem root (values : (List Nat)) : LexLeanPreservation.RunConv __prog 0 [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) values)] (LexLeanPreservation.Rel (__fits_0 values) (LexLeanTarget.TargetSyntax.Value.nat (Production.Main.sumAll values))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 values)

end LexLeanPreserve.C7d7ad30cd6ee86489c4bf5d3815611d8.R4
