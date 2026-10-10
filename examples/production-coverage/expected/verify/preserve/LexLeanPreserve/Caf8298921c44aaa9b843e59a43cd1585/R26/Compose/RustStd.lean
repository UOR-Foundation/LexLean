import LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R26
import LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R26.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R26.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (xs : (List Nat)) (b : ByteArray) (i : Nat) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.nat) xs), (_root_.LexLeanTarget.TargetSyntax.Value.bytes b), (_root_.LexLeanTarget.TargetSyntax.Value.nat i)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R26.denote xs b i) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R26.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.nat) xs), (_root_.LexLeanTarget.TargetSyntax.Value.bytes b), (_root_.LexLeanTarget.TargetSyntax.Value.nat i)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R26.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encList _root_.LexLeanPreservation.Rust.wt_nat) xs) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_bytes b) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat i) _root_.LexLeanPreservation.Rust.wtl_nil))) rfl rfl (_root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R26.root xs b i) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R26.Compose.RustStd
