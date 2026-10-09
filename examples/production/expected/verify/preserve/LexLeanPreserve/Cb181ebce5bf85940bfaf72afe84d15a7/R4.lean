import LexLeanPreservation.Core
import LexLeanPreservation.Values
import LexLeanPreservation.Primitives
import LexLeanPreservation.Fixed
import LexLeanPreservation.Keys
import LexLeanPreservation.Templates
import LexLeanPreservation.Validate
import Production.Kernel
import Production.Main
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R4

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [(.list .nat)], result := .nat,
      body := (.call 1 [(.var 0)]) },
    { parameters := [0], types := [(.list .nat)], result := .nat,
      body := (.«match» .nat (.var 0) [(.arm .nil [] (.value .nat (.nat 0))), (.arm .cons [1, 2] (.prim .natAdd [(.var 1), (.call 1 [(.var 2)])]))]) }] }

def __fits_1 : ∀ (values : (List Nat)), Bool
  | (List.nil) => (Bool.true && Bool.true)
  | (List.cons head tail) => (Bool.true && ((Bool.true && (((Bool.true && Bool.true) && (__fits_1 (tail))) && Bool.true)) && (Nat.blt (head + (Production.Kernel.total (tail))) 18446744073709551616)))

theorem __rel_1 : ∀ (values : (List Nat)), _root_.LexLeanPreservation.FunRel __prog 1 [((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.nat) values)] (_root_.LexLeanPreservation.Rel (__fits_1 values) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Production.Kernel.total values)))
  | (List.nil) => by rw [__fits_1.eq_def]; exact _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_match (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convA_hit rfl rfl _root_.LexLeanPreservation.conv_value))
  | (List.cons head tail) => by rw [__fits_1.eq_def]; exact _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_match (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convA_miss rfl (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__rel_1 (tail))) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd head (Production.Kernel.total (tail)))))))

def __fits_0 (values : (List Nat)) : Bool :=
  ((Bool.true && Bool.true) && (__fits_1 (values)))

attribute [local irreducible] Production.Kernel.total in
theorem __rel_0 (values : (List Nat)) : _root_.LexLeanPreservation.FunRel __prog 0 [((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.nat) values)] (_root_.LexLeanPreservation.Rel (__fits_0 values) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Production.Main.sumAll values))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__rel_1 (values)))

def denote (values : (List Nat)) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 values) (_root_.LexLeanPreservation.Obs.value ((_root_.LexLeanTarget.TargetSyntax.Value.nat (Production.Main.sumAll values)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (values : (List Nat)) : _root_.LexLeanPreservation.RunConv __prog 0 [((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.nat) values)] (_root_.LexLeanPreservation.Rel (__fits_0 values) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Production.Main.sumAll values))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 values)

end LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R4
