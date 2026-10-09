import LexLeanPreservation.Core
import LexLeanPreservation.Values
import LexLeanPreservation.Primitives
import LexLeanPreservation.Fixed
import LexLeanPreservation.Keys
import LexLeanPreservation.Templates
import LexLeanPreservation.Validate
import Production.Kernel
import Production.Main
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R0

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [(.fixed .u32), (.fixed .u32)], result := (.option (.fixed .u32)),
      body := (.prim .checkedAdd [(.var 0), (.var 1)]) }] }

def __fits_0 (left : UInt32) (right : UInt32) : Bool :=
  ((Bool.true && (Bool.true && Bool.true)) && Bool.true)

theorem __rel_0 (left : UInt32) (right : UInt32) : _root_.LexLeanPreservation.FunRel __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.u32 left), (_root_.LexLeanTarget.TargetSyntax.Value.u32 right)] (_root_.LexLeanPreservation.Rel (__fits_0 left right) ((_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.u32) (Production.Main.checkedSum left right))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_checkedAdd_u32 left right))

def denote (left : UInt32) (right : UInt32) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 left right) (_root_.LexLeanPreservation.Obs.value (((_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.u32) (Production.Main.checkedSum left right)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (left : UInt32) (right : UInt32) : _root_.LexLeanPreservation.RunConv __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.u32 left), (_root_.LexLeanTarget.TargetSyntax.Value.u32 right)] (_root_.LexLeanPreservation.Rel (__fits_0 left right) ((_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.u32) (Production.Main.checkedSum left right))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 left right)

end LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R0
