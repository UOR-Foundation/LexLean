import LexLeanPreservation.Core
import LexLeanPreservation.Values
import LexLeanPreservation.Primitives
import LexLeanPreservation.Fixed
import LexLeanPreservation.Keys
import LexLeanPreservation.Templates
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
namespace LexLeanPreserve.C5b0ac78067f0d9170f84cb073eec4258.R10

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [.int, .int], result := .int,
      body := (.prim .intSub [(.prim .intMul [(.var 0), (.value .int (.int (-3)))]), (.prim .intNeg [(.prim .intRem [(.prim .intQuot [(.var 0), (.var 1), (.value .int (.int 1))]), (.var 1), (.value .int (.int 0))])])]) }] }

def __fits_0 (a : Int) (b : Int) : Bool :=
  ((((true && (true && true)) && (LexLeanPreservation.intFits (Coverage.Main.LexLeanRuntime.multiply (a) ((-3 : Int)) : Int))) && (((((((true && (true && (true && true))) && (LexLeanPreservation.intFits (Coverage.Main.LexLeanRuntime.quotient (a) (b) ((1 : Int)) : Int))) && (true && (true && true))) && (LexLeanPreservation.intFits (Coverage.Main.LexLeanRuntime.remainder ((Coverage.Main.LexLeanRuntime.quotient (a) (b) ((1 : Int)) : Int)) (b) ((0 : Int)) : Int))) && true) && (LexLeanPreservation.intFits (Coverage.Main.LexLeanRuntime.negate ((Coverage.Main.LexLeanRuntime.remainder ((Coverage.Main.LexLeanRuntime.quotient (a) (b) ((1 : Int)) : Int)) (b) ((0 : Int)) : Int)) : Int))) && true)) && (LexLeanPreservation.intFits (Coverage.Main.LexLeanRuntime.subtract ((Coverage.Main.LexLeanRuntime.multiply (a) ((-3 : Int)) : Int)) ((Coverage.Main.LexLeanRuntime.negate ((Coverage.Main.LexLeanRuntime.remainder ((Coverage.Main.LexLeanRuntime.quotient (a) (b) ((1 : Int)) : Int)) (b) ((0 : Int)) : Int)) : Int)) : Int)))

theorem __rel_0 (a : Int) (b : Int) : LexLeanPreservation.FunRel __prog 0 [(LexLeanTarget.TargetSyntax.Value.int a), (LexLeanTarget.TargetSyntax.Value.int b)] (LexLeanPreservation.Rel (__fits_0 a b) (LexLeanTarget.TargetSyntax.Value.int (Coverage.Main.intOps a b))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_intMul a (-3 : Int))) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil))) (LexLeanPreservation.prim_intQuot a b (1 : Int))) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil))) (LexLeanPreservation.prim_intRem (Coverage.Main.LexLeanRuntime.quotient (a) (b) ((1 : Int)) : Int) b (0 : Int))) LexLeanPreservation.convL_nil) (LexLeanPreservation.prim_intNeg (Coverage.Main.LexLeanRuntime.remainder ((Coverage.Main.LexLeanRuntime.quotient (a) (b) ((1 : Int)) : Int)) (b) ((0 : Int)) : Int))) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_intSub (Coverage.Main.LexLeanRuntime.multiply (a) ((-3 : Int)) : Int) (Coverage.Main.LexLeanRuntime.negate ((Coverage.Main.LexLeanRuntime.remainder ((Coverage.Main.LexLeanRuntime.quotient (a) (b) ((1 : Int)) : Int)) (b) ((0 : Int)) : Int)) : Int)))

def denote (a : Int) (b : Int) : LexLeanPreservation.Obs :=
  LexLeanPreservation.Rel (__fits_0 a b) (LexLeanTarget.TargetSyntax.Value.int (Coverage.Main.intOps a b))

theorem root (a : Int) (b : Int) : LexLeanPreservation.RunConv __prog 0 [(LexLeanTarget.TargetSyntax.Value.int a), (LexLeanTarget.TargetSyntax.Value.int b)] (LexLeanPreservation.Rel (__fits_0 a b) (LexLeanTarget.TargetSyntax.Value.int (Coverage.Main.intOps a b))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 a b)

end LexLeanPreserve.C5b0ac78067f0d9170f84cb073eec4258.R10
