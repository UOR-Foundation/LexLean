import LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R3
import LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R3.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R3.Compose.RustStd

theorem __wt_0 : ∀ (__s : (Models.Session.Window)), LexLeanPreservation.Rust.WT LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R3.RustStd.program LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R3.RustStd.krate LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R3.RustStd.flags (LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R3.__enc_0 __s) (.adt 0) :=
  fun __s => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat (__s).used) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat (__s).items) LexLeanPreservation.Rust.wtl_nil))

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (costs : (List Nat)) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R3.denote costs) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R3.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) costs)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R3.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList LexLeanPreservation.Rust.wt_nat) costs) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R3.root costs) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R3.Compose.RustStd
