import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R21
import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R21.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R21.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (a : Int8) (b : Int8) (hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanTarget.TargetSyntax.Value.i8 a), (LexLeanTarget.TargetSyntax.Value.i8 b)]) : ∃ ro, LexLeanPreservation.Rust.RealizesFn false (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R21.denote a b) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R21.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.i8 a), (LexLeanTarget.TargetSyntax.Value.i8 b)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R21.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_i8 a) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_i8 b) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R21.root a b) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R21.Compose.RustStd
