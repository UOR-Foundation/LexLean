import LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R26
import LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R26.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R26.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (xs : (List Nat)) (b : ByteArray) (i : Nat) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R26.denote xs b i) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R26.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) xs), (LexLeanTarget.TargetSyntax.Value.bytes b), (LexLeanTarget.TargetSyntax.Value.nat i)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R26.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList LexLeanPreservation.Rust.wt_nat) xs) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_bytes b) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat i) LexLeanPreservation.Rust.wtl_nil))) rfl rfl (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R26.root xs b i) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R26.Compose.RustStd
