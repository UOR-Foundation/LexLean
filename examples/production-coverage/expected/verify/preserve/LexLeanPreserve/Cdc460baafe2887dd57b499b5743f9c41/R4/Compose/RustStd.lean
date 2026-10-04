import LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R4
import LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R4.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R4.Compose.RustStd

/-- The rendering of the root's entry, invoked on the encoded arguments,
realizes the encoded source result when the arguments satisfy §17.12's
invariants, and refuses them with `none` otherwise. -/
theorem root (xs : (List Nat)) (s : (List Int)) (hrep : LexLeanPreservation.Rust.RepresentableL [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) xs), ((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.int) s)]) : ∃ ro, LexLeanPreservation.Rust.RCI LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R4.RustStd.krate (LexLeanPreservation.Rust.fnIdent 6) [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) xs), ((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.int) s)] ro ∧
    (LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R4.accepts xs s → LexLeanPreservation.Rust.RealizesFn true (LexLeanPreservation.someObs (LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R4.denote xs s)) ro) ∧
    (¬ LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R4.accepts xs s → LexLeanPreservation.Rust.RealizesFn true (LexLeanPreservation.Obs.value LexLeanTarget.TargetSyntax.Value.none) ro) :=
  match LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R4.RustStd.root n 6) rfl (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList LexLeanPreservation.Rust.wt_nat) xs) (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList LexLeanPreservation.Rust.wt_int) s) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R4.entry xs s) (LexLeanPreservation.Rust.rel_ne_stuck _ _) with
  | ⟨ro, hr, hc⟩ => ⟨ro, hc, fun h => LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R4.entry_accepts xs s h ▸ hr, fun h => LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R4.entry_refuses xs s h ▸ hr⟩

end LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R4.Compose.RustStd
