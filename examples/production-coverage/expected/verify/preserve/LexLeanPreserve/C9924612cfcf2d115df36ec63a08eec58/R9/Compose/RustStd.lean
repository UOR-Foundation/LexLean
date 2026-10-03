import LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R9
import LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R9.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R9.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (x : Nat) (s : String) : ∃ ro, LexLeanPreservation.Rust.RealizesFn false (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R9.denote x s) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R9.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.nat x), (LexLeanTarget.TargetSyntax.Value.string s)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R9.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat x) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_string s) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R9.root x s) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R9.Compose.RustStd
