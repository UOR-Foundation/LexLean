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
namespace LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R20

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [.nat], result := .bool,
      body := (.call 1 [(.var 0)]) },
    { parameters := [0], types := [.nat], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .zero [] (.build .false .bool [])), (.arm .succ [1] (.call 2 [(.var 1)]))]) },
    { parameters := [0], types := [.nat], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .zero [] (.build .true .bool [])), (.arm .succ [1] (.call 1 [(.var 1)]))]) }] }

mutual
def __fits_1 : ∀ (n : Nat), Bool
  | (Nat.zero) => (Bool.true && (Bool.true && Bool.true))
  | (Nat.succ k) => (Bool.true && ((Bool.true && Bool.true) && (__fits_2 (k))))
termination_by structural n => n

def __fits_2 : ∀ (n : Nat), Bool
  | (Nat.zero) => (Bool.true && (Bool.true && Bool.true))
  | (Nat.succ k) => (Bool.true && ((Bool.true && Bool.true) && (__fits_1 (k))))
termination_by structural n => n

end

mutual
theorem __rel_1 : ∀ (n : Nat), _root_.LexLeanPreservation.FunRel __prog 1 [(_root_.LexLeanTarget.TargetSyntax.Value.nat n)] (_root_.LexLeanPreservation.Rel (__fits_1 n) (_root_.LexLeanTarget.TargetSyntax.Value.bool (Coverage.Types.isOdd n)))
  | (Nat.zero) => by rw [__fits_1.eq_def]; exact _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_match (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_build _root_.LexLeanPreservation.convL_nil _root_.LexLeanPreservation.construct_false)))
  | (Nat.succ k) => by rw [__fits_1.eq_def]; exact _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_match (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convA_miss rfl (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__rel_2 (k))))))
termination_by structural n => n

theorem __rel_2 : ∀ (n : Nat), _root_.LexLeanPreservation.FunRel __prog 2 [(_root_.LexLeanTarget.TargetSyntax.Value.nat n)] (_root_.LexLeanPreservation.Rel (__fits_2 n) (_root_.LexLeanTarget.TargetSyntax.Value.bool (Coverage.Types.isEven n)))
  | (Nat.zero) => by rw [__fits_2.eq_def]; exact _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_match (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_build _root_.LexLeanPreservation.convL_nil _root_.LexLeanPreservation.construct_true)))
  | (Nat.succ k) => by rw [__fits_2.eq_def]; exact _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_match (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convA_miss rfl (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__rel_1 (k))))))
termination_by structural n => n

end

def __fits_0 (n : Nat) : Bool :=
  ((Bool.true && Bool.true) && (__fits_1 (n)))

attribute [local irreducible] Coverage.Types.isOdd in
theorem __rel_0 (n : Nat) : _root_.LexLeanPreservation.FunRel __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat n)] (_root_.LexLeanPreservation.Rel (__fits_0 n) (_root_.LexLeanTarget.TargetSyntax.Value.bool (Coverage.Main.parity n))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__rel_1 (n)))

def denote (n : Nat) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 n) (_root_.LexLeanPreservation.Obs.value ((_root_.LexLeanTarget.TargetSyntax.Value.bool (Coverage.Main.parity n)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (n : Nat) : _root_.LexLeanPreservation.RunConv __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat n)] (_root_.LexLeanPreservation.Rel (__fits_0 n) (_root_.LexLeanTarget.TargetSyntax.Value.bool (Coverage.Main.parity n))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 n)

end LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R20
