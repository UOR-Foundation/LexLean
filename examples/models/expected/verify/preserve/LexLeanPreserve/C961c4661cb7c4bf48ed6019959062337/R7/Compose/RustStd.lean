import LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R7
import LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R7.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R7.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (x : Nat) : ∃ ro, LexLeanPreservation.Rust.RealizesFn false (LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R7.denote x) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R7.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.nat x)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R7.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat x) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R7.root x) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R7.Compose.RustStd
