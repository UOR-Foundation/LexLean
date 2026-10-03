import LexLeanPreservation.Core
import LexLeanPreservation.Values
import LexLeanPreservation.Primitives
import LexLeanPreservation.Fixed
import LexLeanPreservation.Keys
import LexLeanPreservation.Templates
import Production.Kernel
import Production.Main
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.C5cefcb733d9f1b6cd44c798480b543f0.R0

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [(.fixed .u32), (.fixed .u32)], result := (.option (.fixed .u32)),
      body := (.prim .checkedAdd [(.var 0), (.var 1)]) }] }

def __fits_0 (left : UInt32) (right : UInt32) : Bool :=
  ((true && (true && true)) && true)

theorem __rel_0 (left : UInt32) (right : UInt32) : LexLeanPreservation.FunRel __prog 0 [(LexLeanTarget.TargetSyntax.Value.u32 left), (LexLeanTarget.TargetSyntax.Value.u32 right)] (LexLeanPreservation.Rel (__fits_0 left right) ((LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.u32) (Production.Main.checkedSum left right))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_checkedAdd_u32 left right))

def denote (left : UInt32) (right : UInt32) : LexLeanPreservation.Obs :=
  LexLeanPreservation.Rel (__fits_0 left right) ((LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.u32) (Production.Main.checkedSum left right))

theorem root (left : UInt32) (right : UInt32) : LexLeanPreservation.RunConv __prog 0 [(LexLeanTarget.TargetSyntax.Value.u32 left), (LexLeanTarget.TargetSyntax.Value.u32 right)] (LexLeanPreservation.Rel (__fits_0 left right) ((LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.u32) (Production.Main.checkedSum left right))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 left right)

end LexLeanPreserve.C5cefcb733d9f1b6cd44c798480b543f0.R0
