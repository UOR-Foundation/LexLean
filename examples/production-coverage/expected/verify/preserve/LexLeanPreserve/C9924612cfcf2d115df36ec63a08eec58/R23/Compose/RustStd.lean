import LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R23
import LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R23.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R23.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (a : Int64) (b : Int64) (c : UInt64) : ∃ ro, LexLeanPreservation.Rust.RealizesFn false (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R23.denote a b c) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R23.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.i64 a), (LexLeanTarget.TargetSyntax.Value.i64 b), (LexLeanTarget.TargetSyntax.Value.u64 c)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R23.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_i64 a) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_i64 b) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_u64 c) LexLeanPreservation.Rust.wtl_nil))) rfl rfl (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R23.root a b c) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R23.Compose.RustStd
