import LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R15
import LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R15.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R15.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (v : (Option Nat)) (r : (Except Bool Nat)) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [((_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.nat) v), ((_root_.LexLeanPreservation.encExcept _root_.LexLeanTarget.TargetSyntax.Value.bool _root_.LexLeanTarget.TargetSyntax.Value.nat) r)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R15.denote v r) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R15.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [((_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.nat) v), ((_root_.LexLeanPreservation.encExcept _root_.LexLeanTarget.TargetSyntax.Value.bool _root_.LexLeanTarget.TargetSyntax.Value.nat) r)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R15.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encOption _root_.LexLeanPreservation.Rust.wt_nat) v) (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encExcept _root_.LexLeanPreservation.Rust.wt_bool _root_.LexLeanPreservation.Rust.wt_nat) r) _root_.LexLeanPreservation.Rust.wtl_nil)) rfl rfl (_root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R15.root v r) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R15.Compose.RustStd
