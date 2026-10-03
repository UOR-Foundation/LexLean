import LexLeanPreserve.C7d7ad30cd6ee86489c4bf5d3815611d8.R4
import LexLeanPreserve.C7d7ad30cd6ee86489c4bf5d3815611d8.R4.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C7d7ad30cd6ee86489c4bf5d3815611d8.R4.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (values : (List Nat)) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.C7d7ad30cd6ee86489c4bf5d3815611d8.R4.denote values) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C7d7ad30cd6ee86489c4bf5d3815611d8.R4.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) values)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C7d7ad30cd6ee86489c4bf5d3815611d8.R4.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList LexLeanPreservation.Rust.wt_nat) values) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.C7d7ad30cd6ee86489c4bf5d3815611d8.R4.root values) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C7d7ad30cd6ee86489c4bf5d3815611d8.R4.Compose.RustStd
