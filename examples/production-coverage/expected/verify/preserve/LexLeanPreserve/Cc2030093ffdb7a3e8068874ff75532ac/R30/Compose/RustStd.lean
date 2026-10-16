<<<<<<<< HEAD:examples/production-coverage/expected/verify/preserve/LexLeanPreserve/C8d803bc64ccef7b33075ee3a67640390/R30/Compose/RustStd.lean
import LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R30
import LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R30.RustStd
========
import LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R30
import LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R30.RustStd
>>>>>>>> gh/main:examples/production-coverage/expected/verify/preserve/LexLeanPreserve/Cc2030093ffdb7a3e8068874ff75532ac/R30/Compose/RustStd.lean
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 0
set_option synthInstance.maxHeartbeats 0
set_option linter.unusedVariables false
<<<<<<<< HEAD:examples/production-coverage/expected/verify/preserve/LexLeanPreserve/C8d803bc64ccef7b33075ee3a67640390/R30/Compose/RustStd.lean
namespace LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R30.Compose.RustStd

mutual
theorem __wt_0 : ∀ (__v : (Coverage.Syntax.Rose Nat)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R30.RustStd.program _root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R30.RustStd.krate _root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R30.RustStd.flags (_root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R30.__enc_0 __v) (.adt 0)
  | Coverage.Syntax.Rose.node __x0 __x1 => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat __x0) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_list (__wtItems_0 __x1)) _root_.LexLeanPreservation.Rust.wtl_nil))

theorem __wtItems_0 : ∀ (__v : (List (Coverage.Syntax.Rose Nat))), _root_.LexLeanPreservation.Rust.WTAll _root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R30.RustStd.program _root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R30.RustStd.krate _root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R30.RustStd.flags (_root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R30.__items_0 __v) (.adt 0)
========
namespace LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R30.Compose.RustStd

mutual
theorem __wt_0 : ∀ (__v : (Coverage.Syntax.Rose Nat)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R30.RustStd.program _root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R30.RustStd.krate _root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R30.RustStd.flags (_root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R30.__enc_0 __v) (.adt 0)
  | Coverage.Syntax.Rose.node __x0 __x1 => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat __x0) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_list (__wtItems_0 __x1)) _root_.LexLeanPreservation.Rust.wtl_nil))

theorem __wtItems_0 : ∀ (__v : (List (Coverage.Syntax.Rose Nat))), _root_.LexLeanPreservation.Rust.WTAll _root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R30.RustStd.program _root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R30.RustStd.krate _root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R30.RustStd.flags (_root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R30.__items_0 __v) (.adt 0)
>>>>>>>> gh/main:examples/production-coverage/expected/verify/preserve/LexLeanPreserve/Cc2030093ffdb7a3e8068874ff75532ac/R30/Compose/RustStd.lean
  | [] => _root_.LexLeanPreservation.Rust.wtAll_nil
  | __x0 :: __x1 => _root_.LexLeanPreservation.Rust.wtAll_cons (__wt_0 __x0) (__wtItems_0 __x1)

end

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
<<<<<<<< HEAD:examples/production-coverage/expected/verify/preserve/LexLeanPreserve/C8d803bc64ccef7b33075ee3a67640390/R30/Compose/RustStd.lean
theorem root (n : Nat) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanTarget.TargetSyntax.Value.nat n)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R30.denote n) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R30.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanTarget.TargetSyntax.Value.nat n)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R30.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat n) _root_.LexLeanPreservation.Rust.wtl_nil) rfl rfl (_root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R30.root n) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R30.Compose.RustStd
========
theorem root (n : Nat) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanTarget.TargetSyntax.Value.nat n)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R30.denote n) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R30.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanTarget.TargetSyntax.Value.nat n)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R30.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat n) _root_.LexLeanPreservation.Rust.wtl_nil) rfl rfl (_root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R30.root n) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R30.Compose.RustStd
>>>>>>>> gh/main:examples/production-coverage/expected/verify/preserve/LexLeanPreserve/Cc2030093ffdb7a3e8068874ff75532ac/R30/Compose/RustStd.lean
