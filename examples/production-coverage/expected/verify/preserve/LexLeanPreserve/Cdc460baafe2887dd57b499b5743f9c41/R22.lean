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
namespace LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R22

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [(.fixed .i16), (.fixed .i32)], result := (.pair (.option (.fixed .i16)) (.pair (.option (.fixed .i32)) (.pair (.option (.fixed .i32)) .bool))),
      body := (.build .pair (.pair (.option (.fixed .i16)) (.pair (.option (.fixed .i32)) (.pair (.option (.fixed .i32)) .bool))) [(.prim .checkedMul [(.var 0), (.var 0)]), (.build .pair (.pair (.option (.fixed .i32)) (.pair (.option (.fixed .i32)) .bool)) [(.prim .checkedAdd [(.var 1), (.value (.fixed .i32) (.i32 (-7)))]), (.build .pair (.pair (.option (.fixed .i32)) .bool) [(.prim (.convert .i32) [(.var 0)]), (.prim .equal [(.var 1), (.value (.fixed .i32) (.i32 3))])])])]) }] }

def __fits_0 (a : Int16) (b : Int32) : Bool :=
  ((((true && (true && true)) && true) && (((((true && (true && true)) && true) && (((((true && true) && true) && (((true && (true && true)) && true) && true)) && true) && true)) && true) && true)) && true)

theorem __rel_0 (a : Int16) (b : Int32) : LexLeanPreservation.FunRel __prog 0 [(LexLeanTarget.TargetSyntax.Value.i16 a), (LexLeanTarget.TargetSyntax.Value.i32 b)] (LexLeanPreservation.Rel (__fits_0 a b) ((LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.i16) (LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.i32) (LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.i32) LexLeanTarget.TargetSyntax.Value.bool))) (Coverage.Prims.fixedMiddle a b))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_checkedMul_i16 a a)) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_checkedAdd_i32 b (-7 : Int32))) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (LexLeanPreservation.prim_convert_i16_i32 a)) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_equal_i32 b (3 : Int32))) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair)

def denote (a : Int16) (b : Int32) : LexLeanPreservation.Obs :=
  cond (__fits_0 a b) (LexLeanPreservation.Obs.value (((LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.i16) (LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.i32) (LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.i32) LexLeanTarget.TargetSyntax.Value.bool))) (Coverage.Prims.fixedMiddle a b)))) LexLeanPreservation.Obs.overflow

theorem root (a : Int16) (b : Int32) : LexLeanPreservation.RunConv __prog 0 [(LexLeanTarget.TargetSyntax.Value.i16 a), (LexLeanTarget.TargetSyntax.Value.i32 b)] (LexLeanPreservation.Rel (__fits_0 a b) ((LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.i16) (LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.i32) (LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.i32) LexLeanTarget.TargetSyntax.Value.bool))) (Coverage.Prims.fixedMiddle a b))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 a b)

end LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R22
