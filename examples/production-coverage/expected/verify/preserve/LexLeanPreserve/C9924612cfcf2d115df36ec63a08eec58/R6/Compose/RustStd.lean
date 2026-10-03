import LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R6
import LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R6.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R6.Compose.RustStd

/-- The rendering of the root's entry, invoked on the encoded arguments,
realizes the encoded source result when the arguments satisfy §17.12's
invariants, and refuses them with `none` otherwise. -/
theorem root (g : (List (Prod Nat (List Nat)))) (start : Nat) : ∃ ro, LexLeanPreservation.Rust.RCI LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R6.RustStd.krate (LexLeanPreservation.Rust.fnIdent 30) [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat))) g), (LexLeanTarget.TargetSyntax.Value.nat start)] ro ∧
    (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R6.accepts g start → LexLeanPreservation.Rust.RealizesFn true (LexLeanPreservation.someObs (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R6.denote g start)) ro) ∧
    (¬ LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R6.accepts g start → LexLeanPreservation.Rust.RealizesFn true (LexLeanPreservation.Obs.value LexLeanTarget.TargetSyntax.Value.none) ro) :=
  match LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R6.RustStd.root n 30) rfl (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList (LexLeanPreservation.Rust.wt_encPair LexLeanPreservation.Rust.wt_nat (LexLeanPreservation.Rust.wt_encList LexLeanPreservation.Rust.wt_nat))) g) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat start) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R6.entry g start) (LexLeanPreservation.Rust.rel_ne_stuck _ _) with
  | ⟨ro, hr, hc⟩ => ⟨ro, hc, fun h => LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R6.entry_accepts g start h ▸ hr, fun h => LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R6.entry_refuses g start h ▸ hr⟩

end LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R6.Compose.RustStd
