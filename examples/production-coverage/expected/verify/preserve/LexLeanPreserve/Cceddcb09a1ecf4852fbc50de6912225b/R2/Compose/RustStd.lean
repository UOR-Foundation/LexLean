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
theorem root (m : (List (Prod Nat Nat))) (bonus : Nat) (__e_hrep : LexLeanPreservation.Rust.RepresentableL [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.nat)) m), (LexLeanTarget.TargetSyntax.Value.nat bonus)]) : ∃ __e_ro, LexLeanPreservation.Rust.RCI LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R2.RustStd.krate (LexLeanPreservation.Rust.fnIdent 5) [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.nat)) m), (LexLeanTarget.TargetSyntax.Value.nat bonus)] __e_ro ∧
    (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R2.accepts m bonus → LexLeanPreservation.Rust.RealizesFn true (LexLeanPreservation.someObs (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R2.denote m bonus)) __e_ro) ∧
    (¬ LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R2.accepts m bonus → LexLeanPreservation.Rust.RealizesFn true (LexLeanPreservation.Obs.value LexLeanTarget.TargetSyntax.Value.none) __e_ro) :=
  match LexLeanPreservation.Rust.compose (fun __e_n => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R2.RustStd.root __e_n 5) rfl (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList (LexLeanPreservation.Rust.wt_encPair LexLeanPreservation.Rust.wt_nat LexLeanPreservation.Rust.wt_nat)) m) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat bonus) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R2.entry m bonus) (LexLeanPreservation.Rust.rel_ne_stuck _ _) with
  | ⟨__e_ro, __e_hr, __e_hc⟩ => ⟨__e_ro, __e_hc, fun __e_h => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R2.entry_accepts m bonus __e_h ▸ __e_hr, fun __e_h => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R2.entry_refuses m bonus __e_h ▸ __e_hr⟩

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R2.Compose.RustStd
