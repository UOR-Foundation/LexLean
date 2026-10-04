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
namespace LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R1

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [{ constructors := [[.nat], [.nat, .nat]] }], functions := [
    { parameters := [0, 1], types := [.nat, .nat], result := .nat,
      body := (.call 1 [(.build (.adt 1) (.adt 0) [(.var 0), (.var 1)])]) },
    { parameters := [0], types := [(.adt 0)], result := .nat,
      body := (.«match» .nat (.var 0) [(.arm (.adt 0) [1] (.prim .natMul [(.prim .natMul [(.var 1), (.var 1)]), (.value .nat (.nat 3))])), (.arm (.adt 1) [2, 3] (.prim .natMul [(.var 2), (.var 3)]))]) }] }

def __enc_0 : (Production.Kernel.Shape) -> LexLeanTarget.TargetSyntax.Value
  | Production.Kernel.Shape.circle __x0 => LexLeanTarget.TargetSyntax.Value.adt 0 [(LexLeanTarget.TargetSyntax.Value.nat __x0)]
  | Production.Kernel.Shape.rectangle __x0 __x1 => LexLeanTarget.TargetSyntax.Value.adt 1 [(LexLeanTarget.TargetSyntax.Value.nat __x0), (LexLeanTarget.TargetSyntax.Value.nat __x1)]

def __fits_1 (shape : (Production.Kernel.Shape)) : Bool :=
  (true && (match shape with | Production.Kernel.Shape.circle radius => ((((true && (true && true)) && (Nat.blt (Production.Kernel.LexLeanRuntime.multiply (radius) (radius) : Nat) 18446744073709551616)) && (true && true)) && (Nat.blt (Production.Kernel.LexLeanRuntime.multiply ((Production.Kernel.LexLeanRuntime.multiply (radius) (radius) : Nat)) ((3 : Nat)) : Nat) 18446744073709551616)) | Production.Kernel.Shape.rectangle width height => ((true && (true && true)) && (Nat.blt (Production.Kernel.LexLeanRuntime.multiply (width) (height) : Nat) 18446744073709551616))))

theorem __rel_1 (shape : (Production.Kernel.Shape)) : LexLeanPreservation.FunRel __prog 1 [(__enc_0 shape)] (LexLeanPreservation.Rel (__fits_1 shape) (LexLeanTarget.TargetSyntax.Value.nat (Production.Kernel.area shape))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_matchV __enc_0 (fun (__a : (Production.Kernel.Shape)) => (match __a with | Production.Kernel.Shape.circle radius => ((((true && (true && true)) && (Nat.blt (Production.Kernel.LexLeanRuntime.multiply (radius) (radius) : Nat) 18446744073709551616)) && (true && true)) && (Nat.blt (Production.Kernel.LexLeanRuntime.multiply ((Production.Kernel.LexLeanRuntime.multiply (radius) (radius) : Nat)) ((3 : Nat)) : Nat) 18446744073709551616)) | Production.Kernel.Shape.rectangle width height => ((true && (true && true)) && (Nat.blt (Production.Kernel.LexLeanRuntime.multiply (width) (height) : Nat) 18446744073709551616)))) (fun (__a : (Production.Kernel.Shape)) => LexLeanTarget.TargetSyntax.Value.nat (match __a with | Production.Kernel.Shape.circle radius => (Production.Kernel.LexLeanRuntime.multiply ((Production.Kernel.LexLeanRuntime.multiply (radius) (radius) : Nat)) ((3 : Nat)) : Nat) | Production.Kernel.Shape.rectangle width height => (Production.Kernel.LexLeanRuntime.multiply (width) (height) : Nat))) shape (LexLeanPreservation.conv_var rfl) (fun (__a : (Production.Kernel.Shape)) _ => match __a with | Production.Kernel.Shape.circle radius => (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natMul radius radius)) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natMul (Production.Kernel.LexLeanRuntime.multiply (radius) (radius) : Nat) (3 : Nat)))) | Production.Kernel.Shape.rectangle width height => (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natMul width height))))))

def __fits_0 (width : Nat) (height : Nat) : Bool :=
  ((((true && (true && true)) && true) && true) && (__fits_1 ((Production.Kernel.Shape.rectangle (width) (height) : (Production.Kernel.Shape)))))

attribute [local irreducible] Production.Kernel.area in
theorem __rel_0 (width : Nat) (height : Nat) : LexLeanPreservation.FunRel __prog 0 [(LexLeanTarget.TargetSyntax.Value.nat width), (LexLeanTarget.TargetSyntax.Value.nat height)] (LexLeanPreservation.Rel (__fits_0 width height) (LexLeanTarget.TargetSyntax.Value.nat (Production.Main.shapeArea width height))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_adt) LexLeanPreservation.convL_nil) (__rel_1 ((Production.Kernel.Shape.rectangle (width) (height) : (Production.Kernel.Shape)))))

def denote (width : Nat) (height : Nat) : LexLeanPreservation.Obs :=
  cond (__fits_0 width height) (LexLeanPreservation.Obs.value ((LexLeanTarget.TargetSyntax.Value.nat (Production.Main.shapeArea width height)))) LexLeanPreservation.Obs.overflow

theorem root (width : Nat) (height : Nat) : LexLeanPreservation.RunConv __prog 0 [(LexLeanTarget.TargetSyntax.Value.nat width), (LexLeanTarget.TargetSyntax.Value.nat height)] (LexLeanPreservation.Rel (__fits_0 width height) (LexLeanTarget.TargetSyntax.Value.nat (Production.Main.shapeArea width height))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 width height)

end LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R1
