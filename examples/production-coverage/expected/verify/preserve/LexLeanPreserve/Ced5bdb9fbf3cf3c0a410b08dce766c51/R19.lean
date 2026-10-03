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
namespace LexLeanPreserve.Ced5bdb9fbf3cf3c0a410b08dce766c51.R19

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [.nat], result := .bool,
      body := (.call 1 [(.var 0)]) },
    { parameters := [0], types := [.nat], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .zero [] (.build .false .bool [])), (.arm .succ [1] (.call 2 [(.var 1)]))]) },
    { parameters := [0], types := [.nat], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .zero [] (.build .true .bool [])), (.arm .succ [1] (.call 1 [(.var 1)]))]) }] }

mutual
def __fits_1 : ∀ (n : Nat), Bool
  | (Nat.zero) => (true && (true && true))
  | (Nat.succ k) => (true && ((true && true) && (__fits_2 (k))))
termination_by structural n => n

def __fits_2 : ∀ (n : Nat), Bool
  | (Nat.zero) => (true && (true && true))
  | (Nat.succ k) => (true && ((true && true) && (__fits_1 (k))))
termination_by structural n => n

end

mutual
theorem __rel_1 : ∀ (n : Nat), LexLeanPreservation.FunRel __prog 1 [(LexLeanTarget.TargetSyntax.Value.nat n)] (LexLeanPreservation.Rel (__fits_1 n) (LexLeanTarget.TargetSyntax.Value.bool (Coverage.Types.isOdd n)))
  | (Nat.zero) => by rw [__fits_1.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_false)))
  | (Nat.succ k) => by rw [__fits_1.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_2 (k))))))
termination_by structural n => n

theorem __rel_2 : ∀ (n : Nat), LexLeanPreservation.FunRel __prog 2 [(LexLeanTarget.TargetSyntax.Value.nat n)] (LexLeanPreservation.Rel (__fits_2 n) (LexLeanTarget.TargetSyntax.Value.bool (Coverage.Types.isEven n)))
  | (Nat.zero) => by rw [__fits_2.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_true)))
  | (Nat.succ k) => by rw [__fits_2.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_1 (k))))))
termination_by structural n => n

end

def __fits_0 (n : Nat) : Bool :=
  ((true && true) && (__fits_1 (n)))

theorem __rel_0 (n : Nat) : LexLeanPreservation.FunRel __prog 0 [(LexLeanTarget.TargetSyntax.Value.nat n)] (LexLeanPreservation.Rel (__fits_0 n) (LexLeanTarget.TargetSyntax.Value.bool (Coverage.Main.parity n))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_1 (n)))

def denote (n : Nat) : LexLeanPreservation.Obs :=
  LexLeanPreservation.Rel (__fits_0 n) (LexLeanTarget.TargetSyntax.Value.bool (Coverage.Main.parity n))

theorem root (n : Nat) : LexLeanPreservation.RunConv __prog 0 [(LexLeanTarget.TargetSyntax.Value.nat n)] (LexLeanPreservation.Rel (__fits_0 n) (LexLeanTarget.TargetSyntax.Value.bool (Coverage.Main.parity n))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 n)

end LexLeanPreserve.Ced5bdb9fbf3cf3c0a410b08dce766c51.R19
