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
namespace LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R22

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [(.fixed .i16), (.fixed .i32)], result := (.pair (.option (.fixed .i16)) (.pair (.option (.fixed .i32)) (.pair (.option (.fixed .i32)) .bool))),
      body := (.build .pair (.pair (.option (.fixed .i16)) (.pair (.option (.fixed .i32)) (.pair (.option (.fixed .i32)) .bool))) [(.prim .checkedMul [(.var 0), (.var 0)]), (.build .pair (.pair (.option (.fixed .i32)) (.pair (.option (.fixed .i32)) .bool)) [(.prim .checkedAdd [(.var 1), (.value (.fixed .i32) (.i32 (-7)))]), (.build .pair (.pair (.option (.fixed .i32)) .bool) [(.prim (.convert .i32) [(.var 0)]), (.prim .equal [(.var 1), (.value (.fixed .i32) (.i32 3))])])])]) }] }

def __fits_0 (a : Int16) (b : Int32) : Bool :=
  ((((Bool.true && (Bool.true && Bool.true)) && Bool.true) && (((((Bool.true && (Bool.true && Bool.true)) && Bool.true) && (((((Bool.true && Bool.true) && Bool.true) && (((Bool.true && (Bool.true && Bool.true)) && Bool.true) && Bool.true)) && Bool.true) && Bool.true)) && Bool.true) && Bool.true)) && Bool.true)

theorem __rel_0 (a : Int16) (b : Int32) : _root_.LexLeanPreservation.FunRel __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.i16 a), (_root_.LexLeanTarget.TargetSyntax.Value.i32 b)] (_root_.LexLeanPreservation.Rel (__fits_0 a b) ((_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.i16) (_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.i32) (_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.i32) _root_.LexLeanTarget.TargetSyntax.Value.bool))) (Coverage.Prims.fixedMiddle a b))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_checkedMul_i16 a a)) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_checkedAdd_i32 b (-7 : Int32))) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (_root_.LexLeanPreservation.prim_convert_i16_i32 a)) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_equal_i32 b (3 : Int32))) _root_.LexLeanPreservation.convL_nil)) _root_.LexLeanPreservation.construct_pair) _root_.LexLeanPreservation.convL_nil)) _root_.LexLeanPreservation.construct_pair) _root_.LexLeanPreservation.convL_nil)) _root_.LexLeanPreservation.construct_pair)

def denote (a : Int16) (b : Int32) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 a b) (_root_.LexLeanPreservation.Obs.value (((_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.i16) (_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.i32) (_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.i32) _root_.LexLeanTarget.TargetSyntax.Value.bool))) (Coverage.Prims.fixedMiddle a b)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (a : Int16) (b : Int32) : _root_.LexLeanPreservation.RunConv __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.i16 a), (_root_.LexLeanTarget.TargetSyntax.Value.i32 b)] (_root_.LexLeanPreservation.Rel (__fits_0 a b) ((_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.i16) (_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.i32) (_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.i32) _root_.LexLeanTarget.TargetSyntax.Value.bool))) (Coverage.Prims.fixedMiddle a b))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 a b)

end LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R22
