import LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R21
import LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R21.RustCore
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R21.Compose.RustCore

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (a : Int8) (b : Int8) : ∃ ro, LexLeanPreservation.Rust.RealizesFn false (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R21.denote a b) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R21.RustCore.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.i8 a), (LexLeanTarget.TargetSyntax.Value.i8 b)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R21.RustCore.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_i8 a) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_i8 b) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R21.root a b) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R21.Compose.RustCore
