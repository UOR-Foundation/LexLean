import LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R5
import LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R5.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R5.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (values : (List Nat)) : ∃ ro, LexLeanPreservation.Rust.RealizesFn false (LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R5.denote values) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R5.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) values)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R5.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList LexLeanPreservation.Rust.wt_nat) values) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R5.root values) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R5.Compose.RustStd
