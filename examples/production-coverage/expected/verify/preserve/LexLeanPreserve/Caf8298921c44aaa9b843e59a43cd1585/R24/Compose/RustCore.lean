import LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R24
import LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R24.RustCore
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R24.Compose.RustCore

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (a : UInt16) (b : UInt16) (s : UInt32) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanTarget.TargetSyntax.Value.u16 a), (_root_.LexLeanTarget.TargetSyntax.Value.u16 b), (_root_.LexLeanTarget.TargetSyntax.Value.u32 s)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.false (_root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R24.denote a b s) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R24.RustCore.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanTarget.TargetSyntax.Value.u16 a), (_root_.LexLeanTarget.TargetSyntax.Value.u16 b), (_root_.LexLeanTarget.TargetSyntax.Value.u32 s)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R24.RustCore.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_u16 a) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_u16 b) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_u32 s) _root_.LexLeanPreservation.Rust.wtl_nil))) rfl rfl (_root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R24.root a b s) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R24.Compose.RustCore
