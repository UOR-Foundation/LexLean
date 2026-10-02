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
namespace LexLeanPreserve.C1b337743df1f8849e737174a3c470d11.R2

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [.nat], result := .nat,
      body := (.call 1 [(.closure 2 []), (.var 0)]) },
    { parameters := [0, 1], types := [(.fn [.nat] .nat), .nat], result := .nat,
      body := (.apply (.var 0) [(.apply (.var 0) [(.var 1)])]) },
    { parameters := [0], types := [.nat], result := .nat,
      body := (.prim .natAdd [(.var 0), (.var 0)]) }] }

def __fits_1 (step : (Nat -> Nat)) (__ff_0 : Nat -> Bool) (value : Nat) : Bool :=
  (true && (((true && ((true && true) && (__ff_0 (value)))) && true) && (__ff_0 ((step (value))))))

theorem __rel_1 (step : (Nat -> Nat)) (__ff_0 : Nat -> Bool) (__fi_0 : Nat) (__fc_0 : List LexLeanTarget.TargetSyntax.Value) (__fh_0 : ∀ (__p0 : Nat), LexLeanPreservation.FunRel __prog __fi_0 (LexLeanTarget.TargetSemantics.LexLeanRuntime.append __fc_0 [(LexLeanTarget.TargetSyntax.Value.nat __p0)]) (LexLeanPreservation.Rel (__ff_0 __p0) (LexLeanTarget.TargetSyntax.Value.nat (step __p0)))) (value : Nat) : LexLeanPreservation.FunRel __prog 1 [(LexLeanTarget.TargetSyntax.Value.closure __fi_0 __fc_0), (LexLeanTarget.TargetSyntax.Value.nat value)] (LexLeanPreservation.Rel (__fits_1 step __ff_0 value) (LexLeanTarget.TargetSyntax.Value.nat (Production.Kernel.applyTwice step value))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_apply (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_apply (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__fh_0 (value))) LexLeanPreservation.convL_nil) (__fh_0 ((step (value)))))

def __fits_2 (number : Nat) : Bool :=
  ((true && (true && true)) && (Nat.blt (number + number) 18446744073709551616))

theorem __rel_2 (number : Nat) : LexLeanPreservation.FunRel __prog 2 [(LexLeanTarget.TargetSyntax.Value.nat number)] (LexLeanPreservation.Rel (__fits_2 number) (LexLeanTarget.TargetSyntax.Value.nat ((number + number)))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd number number))

def __fits_0 (value : Nat) : Bool :=
  ((true && (true && true)) && (__fits_1 ((fun (number : Nat) => (number + number))) (fun (__p0 : Nat) => __fits_2 __p0) (value)))

theorem __rel_0 (value : Nat) : LexLeanPreservation.FunRel __prog 0 [(LexLeanTarget.TargetSyntax.Value.nat value)] (LexLeanPreservation.Rel (__fits_0 value) (LexLeanTarget.TargetSyntax.Value.nat (Production.Main.quadruple value))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_closure LexLeanPreservation.convL_nil) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (__rel_1 ((fun (number : Nat) => (number + number))) (fun (__p0 : Nat) => __fits_2 __p0) (2) ([]) (fun (__p0 : Nat) => __rel_2 __p0) (value)))

def denote (value : Nat) : LexLeanPreservation.Obs :=
  LexLeanPreservation.Rel (__fits_0 value) (LexLeanTarget.TargetSyntax.Value.nat (Production.Main.quadruple value))

theorem root (value : Nat) : LexLeanPreservation.RunConv __prog 0 [(LexLeanTarget.TargetSyntax.Value.nat value)] (LexLeanPreservation.Rel (__fits_0 value) (LexLeanTarget.TargetSyntax.Value.nat (Production.Main.quadruple value))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 value)

end LexLeanPreserve.C1b337743df1f8849e737174a3c470d11.R2
