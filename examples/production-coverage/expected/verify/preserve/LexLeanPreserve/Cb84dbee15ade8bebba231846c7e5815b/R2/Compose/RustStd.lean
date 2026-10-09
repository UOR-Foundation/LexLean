import LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2
import LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.Compose.RustStd

/-- The rendering of the root's entry, invoked on the encoded arguments,
realizes the encoded source result when the arguments satisfy §17.12's
invariants, and refuses them with `none` otherwise. -/
theorem root (m : (List (Prod Nat Nat))) (bonus : Nat) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [((_root_.LexLeanPreservation.encList (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.nat _root_.LexLeanTarget.TargetSyntax.Value.nat)) m), (_root_.LexLeanTarget.TargetSyntax.Value.nat bonus)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 5) [((_root_.LexLeanPreservation.encList (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.nat _root_.LexLeanTarget.TargetSyntax.Value.nat)) m), (_root_.LexLeanTarget.TargetSyntax.Value.nat bonus)] __e_ro ∧
    (_root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.accepts m bonus → _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreservation.someObs (_root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.denote m bonus)) __e_ro) ∧
    (¬ _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.accepts m bonus → _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreservation.Obs.value _root_.LexLeanTarget.TargetSyntax.Value.none) __e_ro) :=
  match _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.RustStd.root __e_n 5) rfl (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encList (_root_.LexLeanPreservation.Rust.wt_encPair _root_.LexLeanPreservation.Rust.wt_nat _root_.LexLeanPreservation.Rust.wt_nat)) m) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat bonus) _root_.LexLeanPreservation.Rust.wtl_nil)) rfl rfl (_root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.entry m bonus) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _) with
  | ⟨__e_ro, __e_hr, __e_hc⟩ => ⟨__e_ro, __e_hc, fun __e_h => _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.entry_accepts m bonus __e_h ▸ __e_hr, fun __e_h => _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.entry_refuses m bonus __e_h ▸ __e_hr⟩

end LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.Compose.RustStd
