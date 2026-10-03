import LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R15
import LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R15.RustCore
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R15.Compose.RustCore

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (v : (Option Nat)) (r : (Except Bool Nat)) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R15.denote v r) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R15.RustCore.krate (LexLeanPreservation.Rust.fnIdent 0) [((LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.nat) v), ((LexLeanPreservation.encExcept LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.nat) r)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R15.RustCore.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encOption LexLeanPreservation.Rust.wt_nat) v) (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encExcept LexLeanPreservation.Rust.wt_bool LexLeanPreservation.Rust.wt_nat) r) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R15.root v r) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R15.Compose.RustCore
