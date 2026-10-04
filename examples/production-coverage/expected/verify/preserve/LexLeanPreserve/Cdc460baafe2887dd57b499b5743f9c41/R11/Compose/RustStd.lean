import LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R11
import LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R11.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R11.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (a : Int) (b : Int) (hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanTarget.TargetSyntax.Value.int a), (LexLeanTarget.TargetSyntax.Value.int b)]) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R11.denote a b) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R11.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.int a), (LexLeanTarget.TargetSyntax.Value.int b)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R11.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_int a) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_int b) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R11.root a b) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R11.Compose.RustStd
