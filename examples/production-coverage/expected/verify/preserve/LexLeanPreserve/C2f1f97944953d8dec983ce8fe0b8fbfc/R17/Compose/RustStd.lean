import LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R17
import LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R17.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R17.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (base : Nat) (offset : Nat) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R17.denote base offset) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R17.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.nat base), (LexLeanTarget.TargetSyntax.Value.nat offset)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R17.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat base) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat offset) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R17.root base offset) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R17.Compose.RustStd
