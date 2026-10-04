import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R29
import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R29.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R29.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (s : String) (d : String) (hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanTarget.TargetSyntax.Value.string s), (LexLeanTarget.TargetSyntax.Value.string d)]) : ∃ ro, LexLeanPreservation.Rust.RealizesFn false (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R29.denote s d) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R29.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.string s), (LexLeanTarget.TargetSyntax.Value.string d)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R29.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_string s) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_string d) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R29.root s d) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R29.Compose.RustStd
