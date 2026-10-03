import LexLeanPreserve.C7d7ad30cd6ee86489c4bf5d3815611d8.R0
import LexLeanPreserve.C7d7ad30cd6ee86489c4bf5d3815611d8.R0.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C7d7ad30cd6ee86489c4bf5d3815611d8.R0.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (left : UInt32) (right : UInt32) : ∃ ro, LexLeanPreservation.Rust.RealizesFn false (LexLeanPreserve.C7d7ad30cd6ee86489c4bf5d3815611d8.R0.denote left right) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C7d7ad30cd6ee86489c4bf5d3815611d8.R0.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.u32 left), (LexLeanTarget.TargetSyntax.Value.u32 right)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C7d7ad30cd6ee86489c4bf5d3815611d8.R0.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_u32 left) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_u32 right) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.C7d7ad30cd6ee86489c4bf5d3815611d8.R0.root left right) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C7d7ad30cd6ee86489c4bf5d3815611d8.R0.Compose.RustStd
