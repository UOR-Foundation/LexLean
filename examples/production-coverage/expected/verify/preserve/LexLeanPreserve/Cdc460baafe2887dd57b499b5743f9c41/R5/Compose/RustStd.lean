import LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R5
import LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R5.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R5.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (n : Nat) (start : Nat) (hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanTarget.TargetSyntax.Value.nat n), (LexLeanTarget.TargetSyntax.Value.nat start)]) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R5.denote n start) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R5.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.nat n), (LexLeanTarget.TargetSyntax.Value.nat start)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R5.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat n) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat start) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R5.root n start) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R5.Compose.RustStd
