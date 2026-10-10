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
namespace LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R12

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
  { adts := [{ constructors := [[.nat], [.nat, .nat], []] }], functions := [
    { parameters := [0, 1], types := [.nat, .nat], result := .nat,
      body := (.prim .natAdd [(.call 1 [(.build (.adt 1) (.adt 0) [(.var 0), (.var 1)])]), (.prim .natAdd [(.call 1 [(.build (.adt 0) (.adt 0) [(.var 0)])]), (.call 1 [(.build (.adt 2) (.adt 0) [])])])]) },
    { parameters := [0], types := [(.adt 0)], result := .nat,
      body := (.«match» .nat (.var 0) [(.arm (.adt 0) [1] (.prim .natMul [(.prim .natMul [(.var 1), (.var 1)]), (.value .nat (.nat 3))])), (.arm (.adt 1) [2, 3] (.prim .natMul [(.var 2), (.var 3)])), (.arm (.adt 2) [] (.value .nat (.nat 0)))]) }] }

def __enc_0 : (Coverage.Types.Shape) -> _root_.LexLeanTarget.TargetSyntax.Value
  | Coverage.Types.Shape.circle __x0 => _root_.LexLeanTarget.TargetSyntax.Value.adt 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat __x0)]
  | Coverage.Types.Shape.rect __x0 __x1 => _root_.LexLeanTarget.TargetSyntax.Value.adt 1 [(_root_.LexLeanTarget.TargetSyntax.Value.nat __x0), (_root_.LexLeanTarget.TargetSyntax.Value.nat __x1)]
  | Coverage.Types.Shape.empty => _root_.LexLeanTarget.TargetSyntax.Value.adt 2 []

def __fits_1 (shape : (Coverage.Types.Shape)) : Bool :=
  (Bool.true && (match shape with | Coverage.Types.Shape.circle r => ((((Bool.true && (Bool.true && Bool.true)) && (Nat.blt (Coverage.Types.LexLeanRuntime.multiply (r) (r) : Nat) 18446744073709551616)) && (Bool.true && Bool.true)) && (Nat.blt (Coverage.Types.LexLeanRuntime.multiply ((Coverage.Types.LexLeanRuntime.multiply (r) (r) : Nat)) ((3 : Nat)) : Nat) 18446744073709551616)) | Coverage.Types.Shape.rect w h => ((Bool.true && (Bool.true && Bool.true)) && (Nat.blt (Coverage.Types.LexLeanRuntime.multiply (w) (h) : Nat) 18446744073709551616)) | Coverage.Types.Shape.empty => Bool.true))

theorem __rel_1 (shape : (Coverage.Types.Shape)) : _root_.LexLeanPreservation.FunRel __prog 1 [(__enc_0 shape)] (_root_.LexLeanPreservation.Rel (__fits_1 shape) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Types.area shape))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_matchV __enc_0 (fun (__a : (Coverage.Types.Shape)) => (match __a with | Coverage.Types.Shape.circle r => ((((Bool.true && (Bool.true && Bool.true)) && (Nat.blt (Coverage.Types.LexLeanRuntime.multiply (r) (r) : Nat) 18446744073709551616)) && (Bool.true && Bool.true)) && (Nat.blt (Coverage.Types.LexLeanRuntime.multiply ((Coverage.Types.LexLeanRuntime.multiply (r) (r) : Nat)) ((3 : Nat)) : Nat) 18446744073709551616)) | Coverage.Types.Shape.rect w h => ((Bool.true && (Bool.true && Bool.true)) && (Nat.blt (Coverage.Types.LexLeanRuntime.multiply (w) (h) : Nat) 18446744073709551616)) | Coverage.Types.Shape.empty => Bool.true)) (fun (__a : (Coverage.Types.Shape)) => _root_.LexLeanTarget.TargetSyntax.Value.nat (match __a with | Coverage.Types.Shape.circle r => (Coverage.Types.LexLeanRuntime.multiply ((Coverage.Types.LexLeanRuntime.multiply (r) (r) : Nat)) ((3 : Nat)) : Nat) | Coverage.Types.Shape.rect w h => (Coverage.Types.LexLeanRuntime.multiply (w) (h) : Nat) | Coverage.Types.Shape.empty => (0 : Nat))) shape (_root_.LexLeanPreservation.conv_var rfl) (fun (__a : (Coverage.Types.Shape)) _ => match __a with | Coverage.Types.Shape.circle r => (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natMul r r)) (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natMul (Coverage.Types.LexLeanRuntime.multiply (r) (r) : Nat) (3 : Nat)))) | Coverage.Types.Shape.rect w h => (_root_.LexLeanPreservation.convA_miss rfl (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natMul w h)))) | Coverage.Types.Shape.empty => (_root_.LexLeanPreservation.convA_miss rfl (_root_.LexLeanPreservation.convA_miss rfl (_root_.LexLeanPreservation.convA_hit rfl rfl _root_.LexLeanPreservation.conv_value)))))

def __fits_0 (w : Nat) (h : Nat) : Bool :=
  ((((((Bool.true && (Bool.true && Bool.true)) && Bool.true) && Bool.true) && (__fits_1 ((Coverage.Types.Shape.rect (w) (h) : (Coverage.Types.Shape))))) && (((((((Bool.true && Bool.true) && Bool.true) && Bool.true) && (__fits_1 ((Coverage.Types.Shape.circle (w) : (Coverage.Types.Shape))))) && ((((Bool.true && Bool.true) && Bool.true) && (__fits_1 ((Coverage.Types.Shape.empty : (Coverage.Types.Shape))))) && Bool.true)) && (Nat.blt ((Coverage.Types.area ((Coverage.Types.Shape.circle (w) : (Coverage.Types.Shape)))) + (Coverage.Types.area ((Coverage.Types.Shape.empty : (Coverage.Types.Shape))))) 18446744073709551616)) && Bool.true)) && (Nat.blt ((Coverage.Types.area ((Coverage.Types.Shape.rect (w) (h) : (Coverage.Types.Shape)))) + ((Coverage.Types.area ((Coverage.Types.Shape.circle (w) : (Coverage.Types.Shape)))) + (Coverage.Types.area ((Coverage.Types.Shape.empty : (Coverage.Types.Shape)))))) 18446744073709551616))

attribute [local irreducible] Coverage.Types.area in
theorem __rel_0 (w : Nat) (h : Nat) : _root_.LexLeanPreservation.FunRel __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat w), (_root_.LexLeanTarget.TargetSyntax.Value.nat h)] (_root_.LexLeanPreservation.Rel (__fits_0 w h) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.shapes w h))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) _root_.LexLeanPreservation.construct_adt) _root_.LexLeanPreservation.convL_nil) (__rel_1 ((Coverage.Types.Shape.rect (w) (h) : (Coverage.Types.Shape))))) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) _root_.LexLeanPreservation.construct_adt) _root_.LexLeanPreservation.convL_nil) (__rel_1 ((Coverage.Types.Shape.circle (w) : (Coverage.Types.Shape))))) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build _root_.LexLeanPreservation.convL_nil _root_.LexLeanPreservation.construct_adt) _root_.LexLeanPreservation.convL_nil) (__rel_1 ((Coverage.Types.Shape.empty : (Coverage.Types.Shape))))) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd (Coverage.Types.area ((Coverage.Types.Shape.circle (w) : (Coverage.Types.Shape)))) (Coverage.Types.area ((Coverage.Types.Shape.empty : (Coverage.Types.Shape)))))) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd (Coverage.Types.area ((Coverage.Types.Shape.rect (w) (h) : (Coverage.Types.Shape)))) ((Coverage.Types.area ((Coverage.Types.Shape.circle (w) : (Coverage.Types.Shape)))) + (Coverage.Types.area ((Coverage.Types.Shape.empty : (Coverage.Types.Shape)))))))

def denote (w : Nat) (h : Nat) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 w h) (_root_.LexLeanPreservation.Obs.value ((_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.shapes w h)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (w : Nat) (h : Nat) : _root_.LexLeanPreservation.RunConv __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat w), (_root_.LexLeanTarget.TargetSyntax.Value.nat h)] (_root_.LexLeanPreservation.Rel (__fits_0 w h) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.shapes w h))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 w h)

end LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R12
