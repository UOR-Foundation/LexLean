import LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R23
import LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R23.RustCore
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R23.Compose.RustCore

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (a : Int64) (b : Int64) (c : UInt64) (hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanTarget.TargetSyntax.Value.i64 a), (LexLeanTarget.TargetSyntax.Value.i64 b), (LexLeanTarget.TargetSyntax.Value.u64 c)]) : ∃ ro, LexLeanPreservation.Rust.RealizesFn false (LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R23.denote a b c) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R23.RustCore.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.i64 a), (LexLeanTarget.TargetSyntax.Value.i64 b), (LexLeanTarget.TargetSyntax.Value.u64 c)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R23.RustCore.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_i64 a) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_i64 b) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_u64 c) LexLeanPreservation.Rust.wtl_nil))) rfl rfl (LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R23.root a b c) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R23.Compose.RustCore
