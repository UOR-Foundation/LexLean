import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R2
import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R2.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R2.Compose.RustStd

/-- The rendering of the root's entry, invoked on the encoded arguments,
realizes the encoded source result when the arguments satisfy §17.12's
invariants, and refuses them with `none` otherwise. -/
theorem root (m : (List (Prod Nat Nat))) (bonus : Nat) (hrep : LexLeanPreservation.Rust.RepresentableL [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.nat)) m), (LexLeanTarget.TargetSyntax.Value.nat bonus)]) : ∃ ro, LexLeanPreservation.Rust.RCI LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R2.RustStd.krate (LexLeanPreservation.Rust.fnIdent 5) [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.nat)) m), (LexLeanTarget.TargetSyntax.Value.nat bonus)] ro ∧
    (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R2.accepts m bonus → LexLeanPreservation.Rust.RealizesFn true (LexLeanPreservation.someObs (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R2.denote m bonus)) ro) ∧
    (¬ LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R2.accepts m bonus → LexLeanPreservation.Rust.RealizesFn true (LexLeanPreservation.Obs.value LexLeanTarget.TargetSyntax.Value.none) ro) :=
  match LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R2.RustStd.root n 5) rfl (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList (LexLeanPreservation.Rust.wt_encPair LexLeanPreservation.Rust.wt_nat LexLeanPreservation.Rust.wt_nat)) m) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat bonus) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R2.entry m bonus) (LexLeanPreservation.Rust.rel_ne_stuck _ _) with
  | ⟨ro, hr, hc⟩ => ⟨ro, hc, fun h => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R2.entry_accepts m bonus h ▸ hr, fun h => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R2.entry_refuses m bonus h ▸ hr⟩

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R2.Compose.RustStd
