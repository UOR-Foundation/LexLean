import LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R5
import LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R5.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R5.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (values : (List Nat)) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.nat) values)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.false (_root_.LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R5.denote values) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R5.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.nat) values)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R5.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encList _root_.LexLeanPreservation.Rust.wt_nat) values) _root_.LexLeanPreservation.Rust.wtl_nil) rfl rfl (_root_.LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R5.root values) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R5.Compose.RustStd
