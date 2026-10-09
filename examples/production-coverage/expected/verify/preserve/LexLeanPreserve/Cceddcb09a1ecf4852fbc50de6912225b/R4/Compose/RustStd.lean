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
theorem root (xs : (List Nat)) (s : (List Int)) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.nat) xs), ((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.int) s)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 6) [((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.nat) xs), ((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.int) s)] __e_ro ∧
    (_root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.accepts xs s → _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreservation.someObs (_root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.denote xs s)) __e_ro) ∧
    (¬ _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.accepts xs s → _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreservation.Obs.value _root_.LexLeanTarget.TargetSyntax.Value.none) __e_ro) :=
  match _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.RustStd.root __e_n 6) rfl (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encList _root_.LexLeanPreservation.Rust.wt_nat) xs) (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encList _root_.LexLeanPreservation.Rust.wt_int) s) _root_.LexLeanPreservation.Rust.wtl_nil)) rfl rfl (_root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.entry xs s) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _) with
  | ⟨__e_ro, __e_hr, __e_hc⟩ => ⟨__e_ro, __e_hc, fun __e_h => _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.entry_accepts xs s __e_h ▸ __e_hr, fun __e_h => _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.entry_refuses xs s __e_h ▸ __e_hr⟩

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.Compose.RustStd
