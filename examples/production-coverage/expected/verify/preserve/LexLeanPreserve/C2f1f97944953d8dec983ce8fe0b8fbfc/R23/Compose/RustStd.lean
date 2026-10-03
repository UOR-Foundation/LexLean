import LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R23
import LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R23.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R23.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (a : UInt16) (b : UInt16) (s : UInt32) : ∃ ro, LexLeanPreservation.Rust.RealizesFn false (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R23.denote a b s) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R23.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.u16 a), (LexLeanTarget.TargetSyntax.Value.u16 b), (LexLeanTarget.TargetSyntax.Value.u32 s)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R23.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_u16 a) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_u16 b) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_u32 s) LexLeanPreservation.Rust.wtl_nil))) rfl rfl (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R23.root a b s) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R23.Compose.RustStd
