import LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R2
import LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R2.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R2.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (value : Nat) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R2.denote value) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R2.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.nat value)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R2.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat value) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R2.root value) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R2.Compose.RustStd
