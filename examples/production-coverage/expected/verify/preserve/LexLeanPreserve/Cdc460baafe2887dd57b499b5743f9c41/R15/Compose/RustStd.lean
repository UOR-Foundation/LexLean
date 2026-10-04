import LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R15
import LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R15.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R15.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (v : (Option Nat)) (r : (Except Bool Nat)) (hrep : LexLeanPreservation.Rust.RepresentableL [((LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.nat) v), ((LexLeanPreservation.encExcept LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.nat) r)]) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R15.denote v r) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R15.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [((LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.nat) v), ((LexLeanPreservation.encExcept LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.nat) r)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R15.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encOption LexLeanPreservation.Rust.wt_nat) v) (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encExcept LexLeanPreservation.Rust.wt_bool LexLeanPreservation.Rust.wt_nat) r) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R15.root v r) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R15.Compose.RustStd
