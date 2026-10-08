import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R27
import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R27.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R27.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (s : String) (__e_hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanTarget.TargetSyntax.Value.string s)]) : ∃ __e_ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R27.denote s) __e_ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R27.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.string s)] __e_ro :=
  LexLeanPreservation.Rust.compose (fun __e_n => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R27.RustStd.root __e_n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_string s) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R27.root s) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R27.Compose.RustStd
