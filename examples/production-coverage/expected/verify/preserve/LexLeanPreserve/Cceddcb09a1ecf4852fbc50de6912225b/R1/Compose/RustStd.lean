import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R1
import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R1.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R1.Compose.RustStd

/-- The rendering of the root's entry, invoked on the encoded arguments,
realizes the encoded source result when the arguments satisfy §17.12's
invariants, and refuses them with `none` otherwise. -/
theorem root (m : (List (Prod String Nat))) (k : String) (v : Nat) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [((_root_.LexLeanPreservation.encList (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.string _root_.LexLeanTarget.TargetSyntax.Value.nat)) m), (_root_.LexLeanTarget.TargetSyntax.Value.string k), (_root_.LexLeanTarget.TargetSyntax.Value.nat v)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R1.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 10) [((_root_.LexLeanPreservation.encList (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.string _root_.LexLeanTarget.TargetSyntax.Value.nat)) m), (_root_.LexLeanTarget.TargetSyntax.Value.string k), (_root_.LexLeanTarget.TargetSyntax.Value.nat v)] __e_ro ∧
    (_root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R1.accepts m k v → _root_.LexLeanPreservation.Rust.RealizesFn Bool.false (_root_.LexLeanPreservation.someObs (_root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R1.denote m k v)) __e_ro) ∧
    (¬ _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R1.accepts m k v → _root_.LexLeanPreservation.Rust.RealizesFn Bool.false (_root_.LexLeanPreservation.Obs.value _root_.LexLeanTarget.TargetSyntax.Value.none) __e_ro) :=
  match _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R1.RustStd.root __e_n 10) rfl (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encList (_root_.LexLeanPreservation.Rust.wt_encPair _root_.LexLeanPreservation.Rust.wt_string _root_.LexLeanPreservation.Rust.wt_nat)) m) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_string k) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat v) _root_.LexLeanPreservation.Rust.wtl_nil))) rfl rfl (_root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R1.entry m k v) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _) with
  | ⟨__e_ro, __e_hr, __e_hc⟩ => ⟨__e_ro, __e_hc, fun __e_h => _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R1.entry_accepts m k v __e_h ▸ __e_hr, fun __e_h => _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R1.entry_refuses m k v __e_h ▸ __e_hr⟩

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R1.Compose.RustStd
