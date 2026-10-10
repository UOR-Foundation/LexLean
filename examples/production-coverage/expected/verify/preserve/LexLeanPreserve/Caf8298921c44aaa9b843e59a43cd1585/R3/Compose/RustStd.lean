import LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R3
import LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R3.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R3.Compose.RustStd

/-- The rendering of the root's entry, invoked on the encoded arguments,
realizes the encoded source result when the arguments satisfy §17.12's
invariants, and refuses them with `none` otherwise. -/
theorem root (s : (List Nat)) (t : (List Nat)) (x : Nat) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.nat) s), ((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.nat) t), (_root_.LexLeanTarget.TargetSyntax.Value.nat x)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R3.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 11) [((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.nat) s), ((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.nat) t), (_root_.LexLeanTarget.TargetSyntax.Value.nat x)] __e_ro ∧
    (_root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R3.accepts s t x → _root_.LexLeanPreservation.Rust.RealizesFn Bool.false (_root_.LexLeanPreservation.someObs (_root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R3.denote s t x)) __e_ro) ∧
    (¬ _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R3.accepts s t x → _root_.LexLeanPreservation.Rust.RealizesFn Bool.false (_root_.LexLeanPreservation.Obs.value _root_.LexLeanTarget.TargetSyntax.Value.none) __e_ro) :=
  match _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R3.RustStd.root __e_n 11) rfl (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encList _root_.LexLeanPreservation.Rust.wt_nat) s) (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encList _root_.LexLeanPreservation.Rust.wt_nat) t) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat x) _root_.LexLeanPreservation.Rust.wtl_nil))) rfl rfl (_root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R3.entry s t x) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _) with
  | ⟨__e_ro, __e_hr, __e_hc⟩ => ⟨__e_ro, __e_hc, fun __e_h => _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R3.entry_accepts s t x __e_h ▸ __e_hr, fun __e_h => _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R3.entry_refuses s t x __e_h ▸ __e_hr⟩

end LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R3.Compose.RustStd
