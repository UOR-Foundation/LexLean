import LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R10
import LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R10.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R10.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (a : Int) (b : Int) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R10.denote a b) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R10.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.int a), (LexLeanTarget.TargetSyntax.Value.int b)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R10.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_int a) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_int b) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R10.root a b) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R10.Compose.RustStd
