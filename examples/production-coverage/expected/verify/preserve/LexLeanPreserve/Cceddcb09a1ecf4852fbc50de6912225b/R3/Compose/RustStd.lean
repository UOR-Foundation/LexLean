import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R3
import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R3.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R3.Compose.RustStd

/-- The rendering of the root's entry, invoked on the encoded arguments,
realizes the encoded source result when the arguments satisfy §17.12's
invariants, and refuses them with `none` otherwise. -/
theorem root (s : (List Nat)) (t : (List Nat)) (x : Nat) (__e_hrep : LexLeanPreservation.Rust.RepresentableL [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) s), ((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) t), (LexLeanTarget.TargetSyntax.Value.nat x)]) : ∃ __e_ro, LexLeanPreservation.Rust.RCI LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R3.RustStd.krate (LexLeanPreservation.Rust.fnIdent 11) [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) s), ((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) t), (LexLeanTarget.TargetSyntax.Value.nat x)] __e_ro ∧
    (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R3.accepts s t x → LexLeanPreservation.Rust.RealizesFn Bool.false (LexLeanPreservation.someObs (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R3.denote s t x)) __e_ro) ∧
    (¬ LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R3.accepts s t x → LexLeanPreservation.Rust.RealizesFn Bool.false (LexLeanPreservation.Obs.value LexLeanTarget.TargetSyntax.Value.none) __e_ro) :=
  match LexLeanPreservation.Rust.compose (fun __e_n => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R3.RustStd.root __e_n 11) rfl (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList LexLeanPreservation.Rust.wt_nat) s) (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList LexLeanPreservation.Rust.wt_nat) t) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat x) LexLeanPreservation.Rust.wtl_nil))) rfl rfl (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R3.entry s t x) (LexLeanPreservation.Rust.rel_ne_stuck _ _) with
  | ⟨__e_ro, __e_hr, __e_hc⟩ => ⟨__e_ro, __e_hc, fun __e_h => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R3.entry_accepts s t x __e_h ▸ __e_hr, fun __e_h => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R3.entry_refuses s t x __e_h ▸ __e_hr⟩

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R3.Compose.RustStd
