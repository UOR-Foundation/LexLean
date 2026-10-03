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
namespace LexLeanPreserve.C90d90c09a90803e7e527e747b4a162d8.R20

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [(.fixed .i8), (.fixed .i8)], result := (.pair (.option (.fixed .i8)) (.pair (.option (.fixed .i8)) (.pair (.option (.fixed .i8)) (.pair (.option (.fixed .i8)) (.option (.fixed .i8)))))),
      body := (.build .pair (.pair (.option (.fixed .i8)) (.pair (.option (.fixed .i8)) (.pair (.option (.fixed .i8)) (.pair (.option (.fixed .i8)) (.option (.fixed .i8)))))) [(.prim .checkedAdd [(.var 0), (.var 1)]), (.build .pair (.pair (.option (.fixed .i8)) (.pair (.option (.fixed .i8)) (.pair (.option (.fixed .i8)) (.option (.fixed .i8))))) [(.prim .checkedSub [(.var 0), (.var 1)]), (.build .pair (.pair (.option (.fixed .i8)) (.pair (.option (.fixed .i8)) (.option (.fixed .i8)))) [(.prim .checkedMul [(.var 0), (.var 1)]), (.build .pair (.pair (.option (.fixed .i8)) (.option (.fixed .i8))) [(.prim .checkedQuot [(.var 0), (.var 1)]), (.prim .checkedNeg [(.var 0)])])])])]) }] }

def __fits_0 (a : Int8) (b : Int8) : Bool :=
  ((((true && (true && true)) && true) && (((((true && (true && true)) && true) && (((((true && (true && true)) && true) && (((((true && (true && true)) && true) && (((true && true) && true) && true)) && true) && true)) && true) && true)) && true) && true)) && true)

theorem __rel_0 (a : Int8) (b : Int8) : LexLeanPreservation.FunRel __prog 0 [(LexLeanTarget.TargetSyntax.Value.i8 a), (LexLeanTarget.TargetSyntax.Value.i8 b)] (LexLeanPreservation.Rel (__fits_0 a b) ((LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.i8) (LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.i8) (LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.i8) (LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.i8) (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.i8))))) (Coverage.Prims.fixedSmall a b))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_checkedAdd_i8 a b)) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_checkedSub_i8 a b)) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_checkedMul_i8 a b)) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_checkedQuot_i8 a b)) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (LexLeanPreservation.prim_checkedNeg_i8 a)) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair)

def denote (a : Int8) (b : Int8) : LexLeanPreservation.Obs :=
  LexLeanPreservation.Rel (__fits_0 a b) ((LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.i8) (LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.i8) (LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.i8) (LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.i8) (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.i8))))) (Coverage.Prims.fixedSmall a b))

theorem root (a : Int8) (b : Int8) : LexLeanPreservation.RunConv __prog 0 [(LexLeanTarget.TargetSyntax.Value.i8 a), (LexLeanTarget.TargetSyntax.Value.i8 b)] (LexLeanPreservation.Rel (__fits_0 a b) ((LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.i8) (LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.i8) (LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.i8) (LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.i8) (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.i8))))) (Coverage.Prims.fixedSmall a b))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 a b)

end LexLeanPreserve.C90d90c09a90803e7e527e747b4a162d8.R20
