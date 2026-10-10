import LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R23
import LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R23.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R23.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (a : Int64) (b : Int64) (c : UInt64) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanTarget.TargetSyntax.Value.i64 a), (_root_.LexLeanTarget.TargetSyntax.Value.i64 b), (_root_.LexLeanTarget.TargetSyntax.Value.u64 c)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.false (_root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R23.denote a b c) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R23.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanTarget.TargetSyntax.Value.i64 a), (_root_.LexLeanTarget.TargetSyntax.Value.i64 b), (_root_.LexLeanTarget.TargetSyntax.Value.u64 c)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R23.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_i64 a) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_i64 b) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_u64 c) _root_.LexLeanPreservation.Rust.wtl_nil))) rfl rfl (_root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R23.root a b c) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R23.Compose.RustStd
