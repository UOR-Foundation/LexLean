import LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R4
import LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R4.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R4.Compose.RustStd

theorem __wt_0 : ∀ (__s : (Models.Session.Window)), LexLeanPreservation.Rust.WT LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R4.RustStd.program LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R4.RustStd.krate LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R4.RustStd.flags (LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R4.__enc_0 __s) (.adt 0) :=
  fun __s => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat (__s).used) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat (__s).items) LexLeanPreservation.Rust.wtl_nil))

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (window : (Models.Session.Window)) (cost : Nat) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R4.denote window cost) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R4.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R4.__enc_0 window), (LexLeanTarget.TargetSyntax.Value.nat cost)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R4.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (__wt_0 window) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat cost) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R4.root window cost) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R4.Compose.RustStd
