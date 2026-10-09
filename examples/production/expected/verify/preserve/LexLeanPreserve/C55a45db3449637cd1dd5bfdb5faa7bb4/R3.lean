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
namespace LexLeanPreserve.C55a45db3449637cd1dd5bfdb5faa7bb4.R3

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [.nat], result := .nat,
      body := (.call 1 [(.var 0), (.value .nat (.nat 0))]) },
    { parameters := [0, 1], types := [.nat, .nat], result := .nat,
      body := (.cond (.prim .natLt [(.var 0), (.value .nat (.nat 2))]) (.var 1) (.call 1 [(.prim .natSub [(.var 0), (.value .nat (.nat 2))]), (.prim .natAdd [(.var 1), (.value .nat (.nat 1))])])) }] }

def __fits_1 (number : Nat) (steps : Nat) : Bool :=
  (((Bool.true && (Bool.true && Bool.true)) && Bool.true) && (match (generalizing := false) __decrease0 : (Nat.blt number (2 : Nat)) with | true => Bool.true | false => ((((Bool.true && (Bool.true && Bool.true)) && Bool.true) && (((Bool.true && (Bool.true && Bool.true)) && (Nat.blt (steps + (1 : Nat)) 18446744073709551616)) && Bool.true)) && (__fits_1 ((Production.Kernel.LexLeanRuntime.subtract (number) ((2 : Nat)) : Nat)) ((steps + (1 : Nat)))))))
termination_by number
decreasing_by all_goals first | (have __evidence := Production.Kernel.countdown_decreases (number) (steps) (__decrease0); subst_vars; exact __evidence)

theorem __rel_1 (number : Nat) (steps : Nat) : _root_.LexLeanPreservation.FunRel __prog 1 [(_root_.LexLeanTarget.TargetSyntax.Value.nat number), (_root_.LexLeanTarget.TargetSyntax.Value.nat steps)] (_root_.LexLeanPreservation.Rel (__fits_1 number steps) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Production.Kernel.countdown number steps))) := by
  rw [Production.Kernel.countdown.eq_def, __fits_1.eq_def]
  exact _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_cond (fun (__b : Bool) => (match (generalizing := false) __decrease0 : __b with | true => Bool.true | false => ((((Bool.true && (Bool.true && Bool.true)) && Bool.true) && (((Bool.true && (Bool.true && Bool.true)) && (Nat.blt (steps + (1 : Nat)) 18446744073709551616)) && Bool.true)) && (__fits_1 ((Production.Kernel.LexLeanRuntime.subtract (number) ((2 : Nat)) : Nat)) ((steps + (1 : Nat))))))) (fun (__b : Bool) => _root_.LexLeanTarget.TargetSyntax.Value.nat (match (generalizing := false) __decrease0 : __b with | true => steps | false => (Production.Kernel.countdown ((Production.Kernel.LexLeanRuntime.subtract (number) ((2 : Nat)) : Nat)) ((steps + (1 : Nat)))))) (Nat.blt number (2 : Nat)) (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natLt number (2 : Nat))) (fun __decrease0 => (_root_.LexLeanPreservation.conv_var rfl)) (fun __decrease0 => (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natSub number (2 : Nat))) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd steps (1 : Nat))) _root_.LexLeanPreservation.convL_nil)) (__rel_1 ((Production.Kernel.LexLeanRuntime.subtract (number) ((2 : Nat)) : Nat)) ((steps + (1 : Nat)))))))
termination_by number
decreasing_by all_goals first | (have __evidence := Production.Kernel.countdown_decreases (number) (steps) (__decrease0); subst_vars; exact __evidence)

def __fits_0 (number : Nat) : Bool :=
  ((Bool.true && (Bool.true && Bool.true)) && (__fits_1 (number) ((0 : Nat))))

attribute [local irreducible] Production.Kernel.countdown in
theorem __rel_0 (number : Nat) : _root_.LexLeanPreservation.FunRel __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat number)] (_root_.LexLeanPreservation.Rel (__fits_0 number) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Production.Main.halvings number))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value _root_.LexLeanPreservation.convL_nil)) (__rel_1 (number) ((0 : Nat))))

def denote (number : Nat) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 number) (_root_.LexLeanPreservation.Obs.value ((_root_.LexLeanTarget.TargetSyntax.Value.nat (Production.Main.halvings number)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (number : Nat) : _root_.LexLeanPreservation.RunConv __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat number)] (_root_.LexLeanPreservation.Rel (__fits_0 number) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Production.Main.halvings number))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 number)

end LexLeanPreserve.C55a45db3449637cd1dd5bfdb5faa7bb4.R3
