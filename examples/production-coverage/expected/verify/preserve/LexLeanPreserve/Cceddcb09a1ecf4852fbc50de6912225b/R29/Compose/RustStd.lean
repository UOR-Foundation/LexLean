import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R29
import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R29.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R29.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (s : String) (d : String) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanTarget.TargetSyntax.Value.string s), (_root_.LexLeanTarget.TargetSyntax.Value.string d)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.false (_root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R29.denote s d) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R29.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanTarget.TargetSyntax.Value.string s), (_root_.LexLeanTarget.TargetSyntax.Value.string d)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R29.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_string s) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_string d) _root_.LexLeanPreservation.Rust.wtl_nil)) rfl rfl (_root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R29.root s d) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R29.Compose.RustStd
