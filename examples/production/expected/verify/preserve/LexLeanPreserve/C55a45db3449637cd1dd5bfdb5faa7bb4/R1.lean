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
namespace LexLeanPreserve.C55a45db3449637cd1dd5bfdb5faa7bb4.R1

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
  { adts := [{ constructors := [[.nat], [.nat, .nat]] }], functions := [
    { parameters := [0, 1], types := [.nat, .nat], result := .nat,
      body := (.call 1 [(.build (.adt 1) (.adt 0) [(.var 0), (.var 1)])]) },
    { parameters := [0], types := [(.adt 0)], result := .nat,
      body := (.«match» .nat (.var 0) [(.arm (.adt 0) [1] (.prim .natMul [(.prim .natMul [(.var 1), (.var 1)]), (.value .nat (.nat 3))])), (.arm (.adt 1) [2, 3] (.prim .natMul [(.var 2), (.var 3)]))]) }] }

def __enc_0 : (Production.Kernel.Shape) -> _root_.LexLeanTarget.TargetSyntax.Value
  | Production.Kernel.Shape.circle __x0 => _root_.LexLeanTarget.TargetSyntax.Value.adt 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat __x0)]
  | Production.Kernel.Shape.rectangle __x0 __x1 => _root_.LexLeanTarget.TargetSyntax.Value.adt 1 [(_root_.LexLeanTarget.TargetSyntax.Value.nat __x0), (_root_.LexLeanTarget.TargetSyntax.Value.nat __x1)]

def __fits_1 (shape : (Production.Kernel.Shape)) : Bool :=
  (Bool.true && (match shape with | Production.Kernel.Shape.circle radius => ((((Bool.true && (Bool.true && Bool.true)) && (Nat.blt (Production.Kernel.LexLeanRuntime.multiply (radius) (radius) : Nat) 18446744073709551616)) && (Bool.true && Bool.true)) && (Nat.blt (Production.Kernel.LexLeanRuntime.multiply ((Production.Kernel.LexLeanRuntime.multiply (radius) (radius) : Nat)) ((3 : Nat)) : Nat) 18446744073709551616)) | Production.Kernel.Shape.rectangle width height => ((Bool.true && (Bool.true && Bool.true)) && (Nat.blt (Production.Kernel.LexLeanRuntime.multiply (width) (height) : Nat) 18446744073709551616))))

theorem __rel_1 (shape : (Production.Kernel.Shape)) : _root_.LexLeanPreservation.FunRel __prog 1 [(__enc_0 shape)] (_root_.LexLeanPreservation.Rel (__fits_1 shape) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Production.Kernel.area shape))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_matchV __enc_0 (fun (__a : (Production.Kernel.Shape)) => (match __a with | Production.Kernel.Shape.circle radius => ((((Bool.true && (Bool.true && Bool.true)) && (Nat.blt (Production.Kernel.LexLeanRuntime.multiply (radius) (radius) : Nat) 18446744073709551616)) && (Bool.true && Bool.true)) && (Nat.blt (Production.Kernel.LexLeanRuntime.multiply ((Production.Kernel.LexLeanRuntime.multiply (radius) (radius) : Nat)) ((3 : Nat)) : Nat) 18446744073709551616)) | Production.Kernel.Shape.rectangle width height => ((Bool.true && (Bool.true && Bool.true)) && (Nat.blt (Production.Kernel.LexLeanRuntime.multiply (width) (height) : Nat) 18446744073709551616)))) (fun (__a : (Production.Kernel.Shape)) => _root_.LexLeanTarget.TargetSyntax.Value.nat (match __a with | Production.Kernel.Shape.circle radius => (Production.Kernel.LexLeanRuntime.multiply ((Production.Kernel.LexLeanRuntime.multiply (radius) (radius) : Nat)) ((3 : Nat)) : Nat) | Production.Kernel.Shape.rectangle width height => (Production.Kernel.LexLeanRuntime.multiply (width) (height) : Nat))) shape (_root_.LexLeanPreservation.conv_var rfl) (fun (__a : (Production.Kernel.Shape)) _ => match __a with | Production.Kernel.Shape.circle radius => (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natMul radius radius)) (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natMul (Production.Kernel.LexLeanRuntime.multiply (radius) (radius) : Nat) (3 : Nat)))) | Production.Kernel.Shape.rectangle width height => (_root_.LexLeanPreservation.convA_miss rfl (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natMul width height))))))

def __fits_0 (width : Nat) (height : Nat) : Bool :=
  ((((Bool.true && (Bool.true && Bool.true)) && Bool.true) && Bool.true) && (__fits_1 ((Production.Kernel.Shape.rectangle (width) (height) : (Production.Kernel.Shape)))))

attribute [local irreducible] Production.Kernel.area in
theorem __rel_0 (width : Nat) (height : Nat) : _root_.LexLeanPreservation.FunRel __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat width), (_root_.LexLeanTarget.TargetSyntax.Value.nat height)] (_root_.LexLeanPreservation.Rel (__fits_0 width height) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Production.Main.shapeArea width height))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) _root_.LexLeanPreservation.construct_adt) _root_.LexLeanPreservation.convL_nil) (__rel_1 ((Production.Kernel.Shape.rectangle (width) (height) : (Production.Kernel.Shape)))))

def denote (width : Nat) (height : Nat) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 width height) (_root_.LexLeanPreservation.Obs.value ((_root_.LexLeanTarget.TargetSyntax.Value.nat (Production.Main.shapeArea width height)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (width : Nat) (height : Nat) : _root_.LexLeanPreservation.RunConv __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat width), (_root_.LexLeanTarget.TargetSyntax.Value.nat height)] (_root_.LexLeanPreservation.Rel (__fits_0 width height) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Production.Main.shapeArea width height))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 width height)

end LexLeanPreserve.C55a45db3449637cd1dd5bfdb5faa7bb4.R1
