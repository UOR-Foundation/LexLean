import LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R22
import LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R22.RustCore
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R22.Compose.RustCore

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (a : Int16) (b : Int32) : ∃ ro, LexLeanPreservation.Rust.RealizesFn false (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R22.denote a b) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R22.RustCore.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.i16 a), (LexLeanTarget.TargetSyntax.Value.i32 b)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R22.RustCore.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_i16 a) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_i32 b) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R22.root a b) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R22.Compose.RustCore
