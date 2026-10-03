import LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R27
import LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R27.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R27.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (s : String) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R27.denote s) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R27.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.string s)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R27.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_string s) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R27.root s) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R27.Compose.RustStd
