import LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R22
import LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R22.RustCore
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R22.Compose.RustCore

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (a : Int64) (b : Int64) (c : UInt64) : ∃ ro, LexLeanPreservation.Rust.RealizesFn false (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R22.denote a b c) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R22.RustCore.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.i64 a), (LexLeanTarget.TargetSyntax.Value.i64 b), (LexLeanTarget.TargetSyntax.Value.u64 c)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R22.RustCore.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_i64 a) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_i64 b) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_u64 c) LexLeanPreservation.Rust.wtl_nil))) rfl rfl (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R22.root a b c) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R22.Compose.RustCore
