import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4
import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.Compose.RustStd

/-- The rendering of the root's entry, invoked on the encoded arguments,
realizes the encoded source result when the arguments satisfy §17.12's
invariants, and refuses them with `none` otherwise. -/
theorem root (xs : (List Nat)) (s : (List Int)) (__e_hrep : LexLeanPreservation.Rust.RepresentableL [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) xs), ((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.int) s)]) : ∃ __e_ro, LexLeanPreservation.Rust.RCI LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.RustStd.krate (LexLeanPreservation.Rust.fnIdent 6) [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) xs), ((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.int) s)] __e_ro ∧
    (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.accepts xs s → LexLeanPreservation.Rust.RealizesFn true (LexLeanPreservation.someObs (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.denote xs s)) __e_ro) ∧
    (¬ LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.accepts xs s → LexLeanPreservation.Rust.RealizesFn true (LexLeanPreservation.Obs.value LexLeanTarget.TargetSyntax.Value.none) __e_ro) :=
  match LexLeanPreservation.Rust.compose (fun __e_n => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.RustStd.root __e_n 6) rfl (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList LexLeanPreservation.Rust.wt_nat) xs) (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList LexLeanPreservation.Rust.wt_int) s) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.entry xs s) (LexLeanPreservation.Rust.rel_ne_stuck _ _) with
  | ⟨__e_ro, __e_hr, __e_hc⟩ => ⟨__e_ro, __e_hc, fun __e_h => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.entry_accepts xs s __e_h ▸ __e_hr, fun __e_h => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.entry_refuses xs s __e_h ▸ __e_hr⟩

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.Compose.RustStd
