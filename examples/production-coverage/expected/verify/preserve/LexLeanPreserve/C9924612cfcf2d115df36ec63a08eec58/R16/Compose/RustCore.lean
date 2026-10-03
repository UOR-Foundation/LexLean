import LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R16
import LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R16.RustCore
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R16.Compose.RustCore

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (n : Nat) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R16.denote n) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R16.RustCore.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.nat n)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R16.RustCore.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat n) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R16.root n) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R16.Compose.RustCore
