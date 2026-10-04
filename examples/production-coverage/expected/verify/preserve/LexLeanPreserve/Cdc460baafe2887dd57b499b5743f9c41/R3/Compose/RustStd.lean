import LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R3
import LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R3.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R3.Compose.RustStd

/-- The rendering of the root's entry, invoked on the encoded arguments,
realizes the encoded source result when the arguments satisfy §17.12's
invariants, and refuses them with `none` otherwise. -/
theorem root (s : (List Nat)) (t : (List Nat)) (x : Nat) (hrep : LexLeanPreservation.Rust.RepresentableL [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) s), ((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) t), (LexLeanTarget.TargetSyntax.Value.nat x)]) : ∃ ro, LexLeanPreservation.Rust.RCI LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R3.RustStd.krate (LexLeanPreservation.Rust.fnIdent 11) [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) s), ((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) t), (LexLeanTarget.TargetSyntax.Value.nat x)] ro ∧
    (LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R3.accepts s t x → LexLeanPreservation.Rust.RealizesFn false (LexLeanPreservation.someObs (LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R3.denote s t x)) ro) ∧
    (¬ LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R3.accepts s t x → LexLeanPreservation.Rust.RealizesFn false (LexLeanPreservation.Obs.value LexLeanTarget.TargetSyntax.Value.none) ro) :=
  match LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R3.RustStd.root n 11) rfl (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList LexLeanPreservation.Rust.wt_nat) s) (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList LexLeanPreservation.Rust.wt_nat) t) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat x) LexLeanPreservation.Rust.wtl_nil))) rfl rfl (LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R3.entry s t x) (LexLeanPreservation.Rust.rel_ne_stuck _ _) with
  | ⟨ro, hr, hc⟩ => ⟨ro, hc, fun h => LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R3.entry_accepts s t x h ▸ hr, fun h => LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R3.entry_refuses s t x h ▸ hr⟩

end LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R3.Compose.RustStd
