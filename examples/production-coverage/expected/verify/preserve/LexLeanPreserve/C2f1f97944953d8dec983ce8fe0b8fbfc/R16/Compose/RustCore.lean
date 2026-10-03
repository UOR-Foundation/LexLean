import LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R16
import LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R16.RustCore
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R16.Compose.RustCore

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (p : (Prod Nat Bool)) : ∃ ro, LexLeanPreservation.Rust.RealizesFn false (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R16.denote p) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R16.RustCore.krate (LexLeanPreservation.Rust.fnIdent 0) [((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.bool) p)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R16.RustCore.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encPair LexLeanPreservation.Rust.wt_nat LexLeanPreservation.Rust.wt_bool) p) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R16.root p) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R16.Compose.RustCore
