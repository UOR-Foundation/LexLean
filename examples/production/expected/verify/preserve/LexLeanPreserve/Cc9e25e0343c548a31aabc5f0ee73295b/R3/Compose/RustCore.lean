import LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R3
import LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R3.RustCore
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R3.Compose.RustCore

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (number : Nat) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R3.denote number) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R3.RustCore.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.nat number)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R3.RustCore.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat number) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R3.root number) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R3.Compose.RustCore
