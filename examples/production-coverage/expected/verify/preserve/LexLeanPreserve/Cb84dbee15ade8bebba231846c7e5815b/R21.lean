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
namespace LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R21

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [(.fixed .i8), (.fixed .i8)], result := (.pair (.option (.fixed .i8)) (.pair (.option (.fixed .i8)) (.pair (.option (.fixed .i8)) (.pair (.option (.fixed .i8)) (.option (.fixed .i8)))))),
      body := (.build .pair (.pair (.option (.fixed .i8)) (.pair (.option (.fixed .i8)) (.pair (.option (.fixed .i8)) (.pair (.option (.fixed .i8)) (.option (.fixed .i8)))))) [(.prim .checkedAdd [(.var 0), (.var 1)]), (.build .pair (.pair (.option (.fixed .i8)) (.pair (.option (.fixed .i8)) (.pair (.option (.fixed .i8)) (.option (.fixed .i8))))) [(.prim .checkedSub [(.var 0), (.var 1)]), (.build .pair (.pair (.option (.fixed .i8)) (.pair (.option (.fixed .i8)) (.option (.fixed .i8)))) [(.prim .checkedMul [(.var 0), (.var 1)]), (.build .pair (.pair (.option (.fixed .i8)) (.option (.fixed .i8))) [(.prim .checkedQuot [(.var 0), (.var 1)]), (.prim .checkedNeg [(.var 0)])])])])]) }] }

def __fits_0 (a : Int8) (b : Int8) : Bool :=
  ((((Bool.true && (Bool.true && Bool.true)) && Bool.true) && (((((Bool.true && (Bool.true && Bool.true)) && Bool.true) && (((((Bool.true && (Bool.true && Bool.true)) && Bool.true) && (((((Bool.true && (Bool.true && Bool.true)) && Bool.true) && (((Bool.true && Bool.true) && Bool.true) && Bool.true)) && Bool.true) && Bool.true)) && Bool.true) && Bool.true)) && Bool.true) && Bool.true)) && Bool.true)

theorem __rel_0 (a : Int8) (b : Int8) : _root_.LexLeanPreservation.FunRel __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.i8 a), (_root_.LexLeanTarget.TargetSyntax.Value.i8 b)] (_root_.LexLeanPreservation.Rel (__fits_0 a b) ((_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.i8) (_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.i8) (_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.i8) (_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.i8) (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.i8))))) (Coverage.Prims.fixedSmall a b))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_checkedAdd_i8 a b)) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_checkedSub_i8 a b)) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_checkedMul_i8 a b)) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_checkedQuot_i8 a b)) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (_root_.LexLeanPreservation.prim_checkedNeg_i8 a)) _root_.LexLeanPreservation.convL_nil)) _root_.LexLeanPreservation.construct_pair) _root_.LexLeanPreservation.convL_nil)) _root_.LexLeanPreservation.construct_pair) _root_.LexLeanPreservation.convL_nil)) _root_.LexLeanPreservation.construct_pair) _root_.LexLeanPreservation.convL_nil)) _root_.LexLeanPreservation.construct_pair)

def denote (a : Int8) (b : Int8) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 a b) (_root_.LexLeanPreservation.Obs.value (((_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.i8) (_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.i8) (_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.i8) (_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.i8) (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.i8))))) (Coverage.Prims.fixedSmall a b)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (a : Int8) (b : Int8) : _root_.LexLeanPreservation.RunConv __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.i8 a), (_root_.LexLeanTarget.TargetSyntax.Value.i8 b)] (_root_.LexLeanPreservation.Rel (__fits_0 a b) ((_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.i8) (_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.i8) (_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.i8) (_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.i8) (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.i8))))) (Coverage.Prims.fixedSmall a b))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 a b)

end LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R21
