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
namespace LexLeanPreserve.Ced5bdb9fbf3cf3c0a410b08dce766c51.R26

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [.string], result := (.pair (.option .int) (.option (.fixed .u32))),
      body := (.build .pair (.pair (.option .int) (.option (.fixed .u32))) [(.prim (.parseDecimal .int) [(.var 0)]), (.prim (.parseDecimal (.fixed .u32)) [(.var 0)])]) }] }

def __fits_0 (s : String) : Bool :=
  ((((true && true) && (LexLeanPreservation.decimalFits s)) && (((true && true) && true) && true)) && true)

theorem __rel_0 (s : String) : LexLeanPreservation.FunRel __prog 0 [(LexLeanTarget.TargetSyntax.Value.string s)] (LexLeanPreservation.Rel (__fits_0 s) ((LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.int) (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.u32)) (Coverage.Prims.decimals s))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (LexLeanPreservation.prim_parseDecimal_int s)) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (LexLeanPreservation.prim_parseDecimal_u32 s)) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair)

def denote (s : String) : LexLeanPreservation.Obs :=
  LexLeanPreservation.Rel (__fits_0 s) ((LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.int) (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.u32)) (Coverage.Prims.decimals s))

theorem root (s : String) : LexLeanPreservation.RunConv __prog 0 [(LexLeanTarget.TargetSyntax.Value.string s)] (LexLeanPreservation.Rel (__fits_0 s) ((LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.int) (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.u32)) (Coverage.Prims.decimals s))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 s)

end LexLeanPreserve.Ced5bdb9fbf3cf3c0a410b08dce766c51.R26
