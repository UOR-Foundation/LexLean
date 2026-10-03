import LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R18
import LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R18.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R18.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (values : (List Nat)) : ∃ ro, LexLeanPreservation.Rust.RealizesFn false (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R18.denote values) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R18.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) values)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R18.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList LexLeanPreservation.Rust.wt_nat) values) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R18.root values) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R18.Compose.RustStd
