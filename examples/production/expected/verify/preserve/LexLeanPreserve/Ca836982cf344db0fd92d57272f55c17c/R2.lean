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
namespace LexLeanPreserve.Ca836982cf344db0fd92d57272f55c17c.R2

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [.nat], result := .nat,
      body := (.call 1 [(.closure 2 []), (.var 0)]) },
    { parameters := [0, 1], types := [(.fn [.nat] .nat), .nat], result := .nat,
      body := (.apply (.var 0) [(.apply (.var 0) [(.var 1)])]) },
    { parameters := [0], types := [.nat], result := .nat,
      body := (.prim .natAdd [(.var 0), (.var 0)]) }] }

def __fits_1 (step : (Nat -> Nat)) (__ff_0 : Nat -> Bool) (value : Nat) : Bool :=
  (Bool.true && (((Bool.true && ((Bool.true && Bool.true) && (__ff_0 (value)))) && Bool.true) && (__ff_0 ((step (value))))))

theorem __rel_1 (step : (Nat -> Nat)) (__ff_0 : Nat -> Bool) (__fi_0 : Nat) (__fc_0 : List _root_.LexLeanTarget.TargetSyntax.Value) (__fh_0 : ∀ (__p0 : Nat), _root_.LexLeanPreservation.FunRel __prog __fi_0 (_root_.LexLeanTarget.TargetSemantics.LexLeanRuntime.append __fc_0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat __p0)]) (_root_.LexLeanPreservation.Rel (__ff_0 __p0) (_root_.LexLeanTarget.TargetSyntax.Value.nat (step __p0)))) (value : Nat) : _root_.LexLeanPreservation.FunRel __prog 1 [(_root_.LexLeanTarget.TargetSyntax.Value.closure __fi_0 __fc_0), (_root_.LexLeanTarget.TargetSyntax.Value.nat value)] (_root_.LexLeanPreservation.Rel (__fits_1 step __ff_0 value) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Production.Kernel.applyTwice step value))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_apply (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_apply (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__fh_0 (value))) _root_.LexLeanPreservation.convL_nil) (__fh_0 ((step (value)))))

def __fits_2 (number : Nat) : Bool :=
  ((Bool.true && (Bool.true && Bool.true)) && (Nat.blt (number + number) 18446744073709551616))

theorem __rel_2 (number : Nat) : _root_.LexLeanPreservation.FunRel __prog 2 [(_root_.LexLeanTarget.TargetSyntax.Value.nat number)] (_root_.LexLeanPreservation.Rel (__fits_2 number) (_root_.LexLeanTarget.TargetSyntax.Value.nat ((number + number)))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd number number))

def __fits_0 (value : Nat) : Bool :=
  ((Bool.true && (Bool.true && Bool.true)) && (__fits_1 ((fun (number : Nat) => (number + number))) (fun (__p0 : Nat) => __fits_2 __p0) (value)))

attribute [local irreducible] Production.Kernel.applyTwice in
theorem __rel_0 (value : Nat) : _root_.LexLeanPreservation.FunRel __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat value)] (_root_.LexLeanPreservation.Rel (__fits_0 value) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Production.Main.quadruple value))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_closure _root_.LexLeanPreservation.convL_nil) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (__rel_1 ((fun (number : Nat) => (number + number))) (fun (__p0 : Nat) => __fits_2 __p0) (2) ([]) (fun (__p0 : Nat) => __rel_2 __p0) (value)))

def denote (value : Nat) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 value) (_root_.LexLeanPreservation.Obs.value ((_root_.LexLeanTarget.TargetSyntax.Value.nat (Production.Main.quadruple value)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (value : Nat) : _root_.LexLeanPreservation.RunConv __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat value)] (_root_.LexLeanPreservation.Rel (__fits_0 value) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Production.Main.quadruple value))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 value)

end LexLeanPreserve.Ca836982cf344db0fd92d57272f55c17c.R2
