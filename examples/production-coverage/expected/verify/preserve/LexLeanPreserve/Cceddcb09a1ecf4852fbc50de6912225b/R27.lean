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
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R27

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [.string], result := (.pair (.option .int) (.option (.fixed .u32))),
      body := (.build .pair (.pair (.option .int) (.option (.fixed .u32))) [(.prim (.parseDecimal .int) [(.var 0)]), (.prim (.parseDecimal (.fixed .u32)) [(.var 0)])]) }] }

def __fits_0 (s : String) : Bool :=
  ((((Bool.true && Bool.true) && (_root_.LexLeanPreservation.decimalFits s)) && (((Bool.true && Bool.true) && Bool.true) && Bool.true)) && Bool.true)

theorem __rel_0 (s : String) : _root_.LexLeanPreservation.FunRel __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.string s)] (_root_.LexLeanPreservation.Rel (__fits_0 s) ((_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.int) (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.u32)) (Coverage.Prims.decimals s))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (_root_.LexLeanPreservation.prim_parseDecimal_int s)) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (_root_.LexLeanPreservation.prim_parseDecimal_u32 s)) _root_.LexLeanPreservation.convL_nil)) _root_.LexLeanPreservation.construct_pair)

def denote (s : String) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 s) (_root_.LexLeanPreservation.Obs.value (((_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.int) (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.u32)) (Coverage.Prims.decimals s)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (s : String) : _root_.LexLeanPreservation.RunConv __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.string s)] (_root_.LexLeanPreservation.Rel (__fits_0 s) ((_root_.LexLeanPreservation.encPair (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.int) (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.u32)) (Coverage.Prims.decimals s))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 s)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R27
