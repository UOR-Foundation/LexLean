<<<<<<<< HEAD:examples/production-coverage/expected/verify/preserve/LexLeanPreserve/C8d803bc64ccef7b33075ee3a67640390/R25/Compose/RustStd.lean
import LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R25
import LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R25.RustStd
========
import LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R25
import LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R25.RustStd
>>>>>>>> gh/main:examples/production-coverage/expected/verify/preserve/LexLeanPreserve/Cc2030093ffdb7a3e8068874ff75532ac/R25/Compose/RustStd.lean
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 0
set_option synthInstance.maxHeartbeats 0
set_option linter.unusedVariables false
<<<<<<<< HEAD:examples/production-coverage/expected/verify/preserve/LexLeanPreserve/C8d803bc64ccef7b33075ee3a67640390/R25/Compose/RustStd.lean
namespace LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R25.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (a : Int) (b : Int8) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanTarget.TargetSyntax.Value.int a), (_root_.LexLeanTarget.TargetSyntax.Value.i8 b)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.false (_root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R25.denote a b) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R25.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanTarget.TargetSyntax.Value.int a), (_root_.LexLeanTarget.TargetSyntax.Value.i8 b)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R25.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_int a) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_i8 b) _root_.LexLeanPreservation.Rust.wtl_nil)) rfl rfl (_root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R25.root a b) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R25.Compose.RustStd
========
namespace LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R25.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (a : Int) (b : Int8) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanTarget.TargetSyntax.Value.int a), (_root_.LexLeanTarget.TargetSyntax.Value.i8 b)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.false (_root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R25.denote a b) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R25.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanTarget.TargetSyntax.Value.int a), (_root_.LexLeanTarget.TargetSyntax.Value.i8 b)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R25.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_int a) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_i8 b) _root_.LexLeanPreservation.Rust.wtl_nil)) rfl rfl (_root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R25.root a b) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R25.Compose.RustStd
>>>>>>>> gh/main:examples/production-coverage/expected/verify/preserve/LexLeanPreserve/Cc2030093ffdb7a3e8068874ff75532ac/R25/Compose/RustStd.lean
