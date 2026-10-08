import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R25
import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R25.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R25.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (a : Int) (b : Int8) (__e_hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanTarget.TargetSyntax.Value.int a), (LexLeanTarget.TargetSyntax.Value.i8 b)]) : ∃ __e_ro, LexLeanPreservation.Rust.RealizesFn false (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R25.denote a b) __e_ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R25.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.int a), (LexLeanTarget.TargetSyntax.Value.i8 b)] __e_ro :=
  LexLeanPreservation.Rust.compose (fun __e_n => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R25.RustStd.root __e_n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_int a) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_i8 b) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R25.root a b) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R25.Compose.RustStd
