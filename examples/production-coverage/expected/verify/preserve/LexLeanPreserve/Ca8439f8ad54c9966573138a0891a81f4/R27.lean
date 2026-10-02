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
namespace LexLeanPreserve.Ca8439f8ad54c9966573138a0891a81f4.R27

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [.int, (.fixed .i8)], result := (.pair .string .string),
      body := (.build .pair (.pair .string .string) [(.prim .formatDecimal [(.var 0)]), (.prim .formatDecimal [(.var 1)])]) }] }

def __fits_0 (a : Int) (b : Int8) : Bool :=
  ((((true && true) && true) && (((true && true) && true) && true)) && true)

theorem __rel_0 (a : Int) (b : Int8) : LexLeanPreservation.FunRel __prog 0 [(LexLeanTarget.TargetSyntax.Value.int a), (LexLeanTarget.TargetSyntax.Value.i8 b)] (LexLeanPreservation.Rel (__fits_0 a b) ((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.string LexLeanTarget.TargetSyntax.Value.string) (Coverage.Prims.formats a b))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (LexLeanPreservation.prim_formatDecimal_int a)) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (LexLeanPreservation.prim_formatDecimal_i8 b)) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair)

def denote (a : Int) (b : Int8) : LexLeanPreservation.Obs :=
  LexLeanPreservation.Rel (__fits_0 a b) ((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.string LexLeanTarget.TargetSyntax.Value.string) (Coverage.Prims.formats a b))

theorem root (a : Int) (b : Int8) : LexLeanPreservation.RunConv __prog 0 [(LexLeanTarget.TargetSyntax.Value.int a), (LexLeanTarget.TargetSyntax.Value.i8 b)] (LexLeanPreservation.Rel (__fits_0 a b) ((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.string LexLeanTarget.TargetSyntax.Value.string) (Coverage.Prims.formats a b))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 a b)

end LexLeanPreserve.Ca8439f8ad54c9966573138a0891a81f4.R27
