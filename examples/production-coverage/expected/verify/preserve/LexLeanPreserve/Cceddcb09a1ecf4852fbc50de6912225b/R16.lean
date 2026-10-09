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
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R16

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [.nat], result := (.pair (.option .nat) (.result .nat .unit)),
      body := (.build .pair (.pair (.option .nat) (.result .nat .unit)) [(.build .some (.option .nat) [(.build .succ .nat [(.var 0)])]), (.cond (.prim .natLt [(.var 0), (.value .nat (.nat 3))]) (.build .ok (.result .nat .unit) [(.build .zero .nat [])]) (.build .error (.result .nat .unit) [(.build .unit .unit [])]))]) }] }

def __fits_0 (n : Nat) : Bool :=
  ((((((Bool.true && Bool.true) && (Nat.blt (Nat.succ (n) : Nat) 18446744073709551616)) && Bool.true) && Bool.true) && ((((Bool.true && (Bool.true && Bool.true)) && Bool.true) && (if (Nat.blt n (3 : Nat)) then (((Bool.true && Bool.true) && Bool.true) && Bool.true) else (((Bool.true && Bool.true) && Bool.true) && Bool.true))) && Bool.true)) && Bool.true)

theorem __rel_0 (n : Nat) : _root_.LexLeanPreservation.FunRel __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat n)] (_root_.LexLeanPreservation.Rel (__fits_0 n) ((_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.nat) (_root_.LexLeanPreservation.encExcept _root_.LexLeanPreservation.encUnit _root_.LexLeanTarget.TargetSyntax.Value.nat)) (Coverage.Main.builders n))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) _root_.LexLeanPreservation.construct_succ) _root_.LexLeanPreservation.convL_nil) _root_.LexLeanPreservation.construct_some) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_cond (fun (__b : Bool) => (if __b then (((Bool.true && Bool.true) && Bool.true) && Bool.true) else (((Bool.true && Bool.true) && Bool.true) && Bool.true))) (fun (__b : Bool) => (_root_.LexLeanPreservation.encExcept _root_.LexLeanPreservation.encUnit _root_.LexLeanTarget.TargetSyntax.Value.nat) (if __b then (Except.ok ((Nat.zero : Nat)) : (Except Unit Nat)) else (Except.error (()) : (Except Unit Nat)))) (Nat.blt n (3 : Nat)) (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natLt n (3 : Nat))) (fun _ => (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build _root_.LexLeanPreservation.convL_nil _root_.LexLeanPreservation.construct_zero) _root_.LexLeanPreservation.convL_nil) _root_.LexLeanPreservation.construct_ok)) (fun _ => (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build _root_.LexLeanPreservation.convL_nil _root_.LexLeanPreservation.construct_unit) _root_.LexLeanPreservation.convL_nil) _root_.LexLeanPreservation.construct_error))) _root_.LexLeanPreservation.convL_nil)) _root_.LexLeanPreservation.construct_pair)

def denote (n : Nat) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 n) (_root_.LexLeanPreservation.Obs.value (((_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.nat) (_root_.LexLeanPreservation.encExcept _root_.LexLeanPreservation.encUnit _root_.LexLeanTarget.TargetSyntax.Value.nat)) (Coverage.Main.builders n)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (n : Nat) : _root_.LexLeanPreservation.RunConv __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat n)] (_root_.LexLeanPreservation.Rel (__fits_0 n) ((_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.nat) (_root_.LexLeanPreservation.encExcept _root_.LexLeanPreservation.encUnit _root_.LexLeanTarget.TargetSyntax.Value.nat)) (Coverage.Main.builders n))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 n)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R16
