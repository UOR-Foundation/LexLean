import LexLeanPreserve.C8e23cfaeae5f3e0ba5c95afdea067390.R30
import LexLeanPreserve.C8e23cfaeae5f3e0ba5c95afdea067390.R30.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 0
set_option synthInstance.maxHeartbeats 0
set_option linter.unusedVariables false
namespace LexLeanPreserve.C8e23cfaeae5f3e0ba5c95afdea067390.R30.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (u8 : UInt8) (i8 : Int8) (u16 : UInt16) (i16 : Int16) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanTarget.TargetSyntax.Value.u8 u8), (_root_.LexLeanTarget.TargetSyntax.Value.i8 i8), (_root_.LexLeanTarget.TargetSyntax.Value.u16 u16), (_root_.LexLeanTarget.TargetSyntax.Value.i16 i16)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.C8e23cfaeae5f3e0ba5c95afdea067390.R30.denote u8 i8 u16 i16) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.C8e23cfaeae5f3e0ba5c95afdea067390.R30.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanTarget.TargetSyntax.Value.u8 u8), (_root_.LexLeanTarget.TargetSyntax.Value.i8 i8), (_root_.LexLeanTarget.TargetSyntax.Value.u16 u16), (_root_.LexLeanTarget.TargetSyntax.Value.i16 i16)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.C8e23cfaeae5f3e0ba5c95afdea067390.R30.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_u8 u8) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_i8 i8) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_u16 u16) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_i16 i16) _root_.LexLeanPreservation.Rust.wtl_nil)))) rfl rfl (_root_.LexLeanPreserve.C8e23cfaeae5f3e0ba5c95afdea067390.R30.root u8 i8 u16 i16) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C8e23cfaeae5f3e0ba5c95afdea067390.R30.Compose.RustStd
