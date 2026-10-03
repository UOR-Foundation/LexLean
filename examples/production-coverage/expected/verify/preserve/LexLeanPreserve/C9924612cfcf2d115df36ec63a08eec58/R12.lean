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
namespace LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R12

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [{ constructors := [[.nat], [.nat, .nat], []] }], functions := [
    { parameters := [0, 1], types := [.nat, .nat], result := .nat,
      body := (.prim .natAdd [(.call 1 [(.build (.adt 1) (.adt 0) [(.var 0), (.var 1)])]), (.prim .natAdd [(.call 1 [(.build (.adt 0) (.adt 0) [(.var 0)])]), (.call 1 [(.build (.adt 2) (.adt 0) [])])])]) },
    { parameters := [0], types := [(.adt 0)], result := .nat,
      body := (.«match» .nat (.var 0) [(.arm (.adt 0) [1] (.prim .natMul [(.prim .natMul [(.var 1), (.var 1)]), (.value .nat (.nat 3))])), (.arm (.adt 1) [2, 3] (.prim .natMul [(.var 2), (.var 3)])), (.arm (.adt 2) [] (.value .nat (.nat 0)))]) }] }

def __enc_0 : (Coverage.Types.Shape) -> LexLeanTarget.TargetSyntax.Value
  | Coverage.Types.Shape.circle __x0 => LexLeanTarget.TargetSyntax.Value.adt 0 [(LexLeanTarget.TargetSyntax.Value.nat __x0)]
  | Coverage.Types.Shape.rect __x0 __x1 => LexLeanTarget.TargetSyntax.Value.adt 1 [(LexLeanTarget.TargetSyntax.Value.nat __x0), (LexLeanTarget.TargetSyntax.Value.nat __x1)]
  | Coverage.Types.Shape.empty => LexLeanTarget.TargetSyntax.Value.adt 2 []

def __fits_1 (shape : (Coverage.Types.Shape)) : Bool :=
  (true && (match shape with | Coverage.Types.Shape.circle r => ((((true && (true && true)) && (Nat.blt (Coverage.Types.LexLeanRuntime.multiply (r) (r) : Nat) 18446744073709551616)) && (true && true)) && (Nat.blt (Coverage.Types.LexLeanRuntime.multiply ((Coverage.Types.LexLeanRuntime.multiply (r) (r) : Nat)) ((3 : Nat)) : Nat) 18446744073709551616)) | Coverage.Types.Shape.rect w h => ((true && (true && true)) && (Nat.blt (Coverage.Types.LexLeanRuntime.multiply (w) (h) : Nat) 18446744073709551616)) | Coverage.Types.Shape.empty => true))

theorem __rel_1 (shape : (Coverage.Types.Shape)) : LexLeanPreservation.FunRel __prog 1 [(__enc_0 shape)] (LexLeanPreservation.Rel (__fits_1 shape) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Types.area shape))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_matchV __enc_0 (fun (__a : (Coverage.Types.Shape)) => (match __a with | Coverage.Types.Shape.circle r => ((((true && (true && true)) && (Nat.blt (Coverage.Types.LexLeanRuntime.multiply (r) (r) : Nat) 18446744073709551616)) && (true && true)) && (Nat.blt (Coverage.Types.LexLeanRuntime.multiply ((Coverage.Types.LexLeanRuntime.multiply (r) (r) : Nat)) ((3 : Nat)) : Nat) 18446744073709551616)) | Coverage.Types.Shape.rect w h => ((true && (true && true)) && (Nat.blt (Coverage.Types.LexLeanRuntime.multiply (w) (h) : Nat) 18446744073709551616)) | Coverage.Types.Shape.empty => true)) (fun (__a : (Coverage.Types.Shape)) => LexLeanTarget.TargetSyntax.Value.nat (match __a with | Coverage.Types.Shape.circle r => (Coverage.Types.LexLeanRuntime.multiply ((Coverage.Types.LexLeanRuntime.multiply (r) (r) : Nat)) ((3 : Nat)) : Nat) | Coverage.Types.Shape.rect w h => (Coverage.Types.LexLeanRuntime.multiply (w) (h) : Nat) | Coverage.Types.Shape.empty => (0 : Nat))) shape (LexLeanPreservation.conv_var rfl) (fun (__a : (Coverage.Types.Shape)) _ => match __a with | Coverage.Types.Shape.circle r => (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natMul r r)) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natMul (Coverage.Types.LexLeanRuntime.multiply (r) (r) : Nat) (3 : Nat)))) | Coverage.Types.Shape.rect w h => (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natMul w h)))) | Coverage.Types.Shape.empty => (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_hit rfl rfl LexLeanPreservation.conv_value)))))

def __fits_0 (w : Nat) (h : Nat) : Bool :=
  ((((((true && (true && true)) && true) && true) && (__fits_1 ((Coverage.Types.Shape.rect (w) (h) : (Coverage.Types.Shape))))) && (((((((true && true) && true) && true) && (__fits_1 ((Coverage.Types.Shape.circle (w) : (Coverage.Types.Shape))))) && ((((true && true) && true) && (__fits_1 ((Coverage.Types.Shape.empty : (Coverage.Types.Shape))))) && true)) && (Nat.blt ((Coverage.Types.area ((Coverage.Types.Shape.circle (w) : (Coverage.Types.Shape)))) + (Coverage.Types.area ((Coverage.Types.Shape.empty : (Coverage.Types.Shape))))) 18446744073709551616)) && true)) && (Nat.blt ((Coverage.Types.area ((Coverage.Types.Shape.rect (w) (h) : (Coverage.Types.Shape)))) + ((Coverage.Types.area ((Coverage.Types.Shape.circle (w) : (Coverage.Types.Shape)))) + (Coverage.Types.area ((Coverage.Types.Shape.empty : (Coverage.Types.Shape)))))) 18446744073709551616))

attribute [local irreducible] Coverage.Types.area in
theorem __rel_0 (w : Nat) (h : Nat) : LexLeanPreservation.FunRel __prog 0 [(LexLeanTarget.TargetSyntax.Value.nat w), (LexLeanTarget.TargetSyntax.Value.nat h)] (LexLeanPreservation.Rel (__fits_0 w h) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.shapes w h))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_adt) LexLeanPreservation.convL_nil) (__rel_1 ((Coverage.Types.Shape.rect (w) (h) : (Coverage.Types.Shape))))) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) LexLeanPreservation.construct_adt) LexLeanPreservation.convL_nil) (__rel_1 ((Coverage.Types.Shape.circle (w) : (Coverage.Types.Shape))))) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_adt) LexLeanPreservation.convL_nil) (__rel_1 ((Coverage.Types.Shape.empty : (Coverage.Types.Shape))))) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd (Coverage.Types.area ((Coverage.Types.Shape.circle (w) : (Coverage.Types.Shape)))) (Coverage.Types.area ((Coverage.Types.Shape.empty : (Coverage.Types.Shape)))))) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd (Coverage.Types.area ((Coverage.Types.Shape.rect (w) (h) : (Coverage.Types.Shape)))) ((Coverage.Types.area ((Coverage.Types.Shape.circle (w) : (Coverage.Types.Shape)))) + (Coverage.Types.area ((Coverage.Types.Shape.empty : (Coverage.Types.Shape)))))))

def denote (w : Nat) (h : Nat) : LexLeanPreservation.Obs :=
  LexLeanPreservation.Rel (__fits_0 w h) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.shapes w h))

theorem root (w : Nat) (h : Nat) : LexLeanPreservation.RunConv __prog 0 [(LexLeanTarget.TargetSyntax.Value.nat w), (LexLeanTarget.TargetSyntax.Value.nat h)] (LexLeanPreservation.Rel (__fits_0 w h) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.shapes w h))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 w h)

end LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R12
