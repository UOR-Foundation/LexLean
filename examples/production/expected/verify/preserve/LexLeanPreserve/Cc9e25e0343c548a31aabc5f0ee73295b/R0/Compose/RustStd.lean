import LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R0
import LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R0.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R0.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (left : UInt32) (right : UInt32) : ∃ ro, LexLeanPreservation.Rust.RealizesFn false (LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R0.denote left right) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R0.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.u32 left), (LexLeanTarget.TargetSyntax.Value.u32 right)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R0.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_u32 left) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_u32 right) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R0.root left right) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R0.Compose.RustStd
