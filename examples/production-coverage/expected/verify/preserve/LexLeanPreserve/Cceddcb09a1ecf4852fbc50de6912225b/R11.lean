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
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R11

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [.int, .int], result := .int,
      body := (.prim .intSub [(.prim .intMul [(.var 0), (.value .int (.int (-3)))]), (.prim .intNeg [(.prim .intRem [(.prim .intQuot [(.var 0), (.var 1), (.value .int (.int 1))]), (.var 1), (.value .int (.int 0))])])]) }] }

def __fits_0 (a : Int) (b : Int) : Bool :=
  ((((Bool.true && (Bool.true && Bool.true)) && (_root_.LexLeanPreservation.intFits (Coverage.Main.LexLeanRuntime.multiply (a) ((-3 : Int)) : Int))) && (((((((Bool.true && (Bool.true && (Bool.true && Bool.true))) && (_root_.LexLeanPreservation.intFits (Coverage.Main.LexLeanRuntime.quotient (a) (b) ((1 : Int)) : Int))) && (Bool.true && (Bool.true && Bool.true))) && (_root_.LexLeanPreservation.intFits (Coverage.Main.LexLeanRuntime.remainder ((Coverage.Main.LexLeanRuntime.quotient (a) (b) ((1 : Int)) : Int)) (b) ((0 : Int)) : Int))) && Bool.true) && (_root_.LexLeanPreservation.intFits (Coverage.Main.LexLeanRuntime.negate ((Coverage.Main.LexLeanRuntime.remainder ((Coverage.Main.LexLeanRuntime.quotient (a) (b) ((1 : Int)) : Int)) (b) ((0 : Int)) : Int)) : Int))) && Bool.true)) && (_root_.LexLeanPreservation.intFits (Coverage.Main.LexLeanRuntime.subtract ((Coverage.Main.LexLeanRuntime.multiply (a) ((-3 : Int)) : Int)) ((Coverage.Main.LexLeanRuntime.negate ((Coverage.Main.LexLeanRuntime.remainder ((Coverage.Main.LexLeanRuntime.quotient (a) (b) ((1 : Int)) : Int)) (b) ((0 : Int)) : Int)) : Int)) : Int)))

theorem __rel_0 (a : Int) (b : Int) : _root_.LexLeanPreservation.FunRel __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.int a), (_root_.LexLeanTarget.TargetSyntax.Value.int b)] (_root_.LexLeanPreservation.Rel (__fits_0 a b) (_root_.LexLeanTarget.TargetSyntax.Value.int (Coverage.Main.intOps a b))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_intMul a (-3 : Int))) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value _root_.LexLeanPreservation.convL_nil))) (_root_.LexLeanPreservation.prim_intQuot a b (1 : Int))) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value _root_.LexLeanPreservation.convL_nil))) (_root_.LexLeanPreservation.prim_intRem (Coverage.Main.LexLeanRuntime.quotient (a) (b) ((1 : Int)) : Int) b (0 : Int))) _root_.LexLeanPreservation.convL_nil) (_root_.LexLeanPreservation.prim_intNeg (Coverage.Main.LexLeanRuntime.remainder ((Coverage.Main.LexLeanRuntime.quotient (a) (b) ((1 : Int)) : Int)) (b) ((0 : Int)) : Int))) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_intSub (Coverage.Main.LexLeanRuntime.multiply (a) ((-3 : Int)) : Int) (Coverage.Main.LexLeanRuntime.negate ((Coverage.Main.LexLeanRuntime.remainder ((Coverage.Main.LexLeanRuntime.quotient (a) (b) ((1 : Int)) : Int)) (b) ((0 : Int)) : Int)) : Int)))

def denote (a : Int) (b : Int) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 a b) (_root_.LexLeanPreservation.Obs.value ((_root_.LexLeanTarget.TargetSyntax.Value.int (Coverage.Main.intOps a b)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (a : Int) (b : Int) : _root_.LexLeanPreservation.RunConv __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.int a), (_root_.LexLeanTarget.TargetSyntax.Value.int b)] (_root_.LexLeanPreservation.Rel (__fits_0 a b) (_root_.LexLeanTarget.TargetSyntax.Value.int (Coverage.Main.intOps a b))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 a b)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R11
