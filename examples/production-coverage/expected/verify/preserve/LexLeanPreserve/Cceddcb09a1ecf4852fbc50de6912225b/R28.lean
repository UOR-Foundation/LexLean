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
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R28

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [.int, (.fixed .i8)], result := (.pair .string .string),
      body := (.build .pair (.pair .string .string) [(.prim .formatDecimal [(.var 0)]), (.prim .formatDecimal [(.var 1)])]) }] }

def __fits_0 (a : Int) (b : Int8) : Bool :=
  ((((Bool.true && Bool.true) && Bool.true) && (((Bool.true && Bool.true) && Bool.true) && Bool.true)) && Bool.true)

theorem __rel_0 (a : Int) (b : Int8) : _root_.LexLeanPreservation.FunRel __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.int a), (_root_.LexLeanTarget.TargetSyntax.Value.i8 b)] (_root_.LexLeanPreservation.Rel (__fits_0 a b) ((_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.string _root_.LexLeanTarget.TargetSyntax.Value.string) (Coverage.Prims.formats a b))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (_root_.LexLeanPreservation.prim_formatDecimal_int a)) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (_root_.LexLeanPreservation.prim_formatDecimal_i8 b)) _root_.LexLeanPreservation.convL_nil)) _root_.LexLeanPreservation.construct_pair)

def denote (a : Int) (b : Int8) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 a b) (_root_.LexLeanPreservation.Obs.value (((_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.string _root_.LexLeanTarget.TargetSyntax.Value.string) (Coverage.Prims.formats a b)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (a : Int) (b : Int8) : _root_.LexLeanPreservation.RunConv __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.int a), (_root_.LexLeanTarget.TargetSyntax.Value.i8 b)] (_root_.LexLeanPreservation.Rel (__fits_0 a b) ((_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.string _root_.LexLeanTarget.TargetSyntax.Value.string) (Coverage.Prims.formats a b))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 a b)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R28
