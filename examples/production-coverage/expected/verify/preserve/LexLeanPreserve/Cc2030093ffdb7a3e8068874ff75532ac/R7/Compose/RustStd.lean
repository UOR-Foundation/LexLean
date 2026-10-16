<<<<<<<< HEAD:examples/production-coverage/expected/verify/preserve/LexLeanPreserve/C8d803bc64ccef7b33075ee3a67640390/R7/Compose/RustStd.lean
import LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R7
import LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R7.RustStd
========
import LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R7
import LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R7.RustStd
>>>>>>>> gh/main:examples/production-coverage/expected/verify/preserve/LexLeanPreserve/Cc2030093ffdb7a3e8068874ff75532ac/R7/Compose/RustStd.lean
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 0
set_option synthInstance.maxHeartbeats 0
set_option linter.unusedVariables false
<<<<<<<< HEAD:examples/production-coverage/expected/verify/preserve/LexLeanPreserve/C8d803bc64ccef7b33075ee3a67640390/R7/Compose/RustStd.lean
namespace LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R7.Compose.RustStd
========
namespace LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R7.Compose.RustStd
>>>>>>>> gh/main:examples/production-coverage/expected/verify/preserve/LexLeanPreserve/Cc2030093ffdb7a3e8068874ff75532ac/R7/Compose/RustStd.lean

/-- The rendering of the root's entry, invoked on the encoded arguments,
realizes the encoded source result when the arguments satisfy §17.12's
invariants, and refuses them with `none` otherwise. -/
<<<<<<<< HEAD:examples/production-coverage/expected/verify/preserve/LexLeanPreserve/C8d803bc64ccef7b33075ee3a67640390/R7/Compose/RustStd.lean
theorem root (g : (List (Prod String (List String)))) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [((_root_.LexLeanPreservation.encList (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.string (_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.string))) g)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R7.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 15) [((_root_.LexLeanPreservation.encList (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.string (_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.string))) g)] __e_ro ∧
    (_root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R7.accepts g → _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreservation.someObs (_root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R7.denote g)) __e_ro) ∧
    (¬ _root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R7.accepts g → _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreservation.Obs.value _root_.LexLeanTarget.TargetSyntax.Value.none) __e_ro) :=
  match _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R7.RustStd.root __e_n 15) rfl (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encList (_root_.LexLeanPreservation.Rust.wt_encPair _root_.LexLeanPreservation.Rust.wt_string (_root_.LexLeanPreservation.Rust.wt_encList _root_.LexLeanPreservation.Rust.wt_string))) g) _root_.LexLeanPreservation.Rust.wtl_nil) rfl rfl (_root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R7.entry g) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _) with
  | ⟨__e_ro, __e_hr, __e_hc⟩ => ⟨__e_ro, __e_hc, fun __e_h => _root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R7.entry_accepts g __e_h ▸ __e_hr, fun __e_h => _root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R7.entry_refuses g __e_h ▸ __e_hr⟩

end LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R7.Compose.RustStd
========
theorem root (g : (List (Prod String (List String)))) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [((_root_.LexLeanPreservation.encList (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.string (_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.string))) g)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R7.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 15) [((_root_.LexLeanPreservation.encList (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.string (_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.string))) g)] __e_ro ∧
    (_root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R7.accepts g → _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreservation.someObs (_root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R7.denote g)) __e_ro) ∧
    (¬ _root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R7.accepts g → _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreservation.Obs.value _root_.LexLeanTarget.TargetSyntax.Value.none) __e_ro) :=
  match _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R7.RustStd.root __e_n 15) rfl (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encList (_root_.LexLeanPreservation.Rust.wt_encPair _root_.LexLeanPreservation.Rust.wt_string (_root_.LexLeanPreservation.Rust.wt_encList _root_.LexLeanPreservation.Rust.wt_string))) g) _root_.LexLeanPreservation.Rust.wtl_nil) rfl rfl (_root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R7.entry g) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _) with
  | ⟨__e_ro, __e_hr, __e_hc⟩ => ⟨__e_ro, __e_hc, fun __e_h => _root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R7.entry_accepts g __e_h ▸ __e_hr, fun __e_h => _root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R7.entry_refuses g __e_h ▸ __e_hr⟩

end LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R7.Compose.RustStd
>>>>>>>> gh/main:examples/production-coverage/expected/verify/preserve/LexLeanPreserve/Cc2030093ffdb7a3e8068874ff75532ac/R7/Compose/RustStd.lean
