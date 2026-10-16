<<<<<<<< HEAD:examples/production-coverage/expected/verify/preserve/LexLeanPreserve/C8d803bc64ccef7b33075ee3a67640390/R14/Compose/RustCore.lean
import LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R14
import LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R14.RustCore
========
import LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R14
import LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R14.RustCore
>>>>>>>> gh/main:examples/production-coverage/expected/verify/preserve/LexLeanPreserve/Cc2030093ffdb7a3e8068874ff75532ac/R14/Compose/RustCore.lean
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 0
set_option synthInstance.maxHeartbeats 0
set_option linter.unusedVariables false
<<<<<<<< HEAD:examples/production-coverage/expected/verify/preserve/LexLeanPreserve/C8d803bc64ccef7b33075ee3a67640390/R14/Compose/RustCore.lean
namespace LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R14.Compose.RustCore

theorem __wt_0 : ∀ (__s : (Coverage.Types.Point)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R14.RustCore.program _root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R14.RustCore.krate _root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R14.RustCore.flags (_root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R14.__enc_0 __s) (.adt 0) :=
  fun __s => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat (__s).x) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat (__s).y) _root_.LexLeanPreservation.Rust.wtl_nil))

theorem __wt_1 : ∀ (__s : (Coverage.Types.Box Nat)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R14.RustCore.program _root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R14.RustCore.krate _root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R14.RustCore.flags (_root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R14.__enc_1 __s) (.adt 1) :=
  fun __s => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat (__s).content) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat (__s).count) _root_.LexLeanPreservation.Rust.wtl_nil))

theorem __wt_2 : ∀ (__s : (Coverage.Types.Weights)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R14.RustCore.program _root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R14.RustCore.krate _root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R14.RustCore.flags (_root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R14.__enc_2 __s) (.adt 2) :=
========
namespace LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R14.Compose.RustCore

theorem __wt_0 : ∀ (__s : (Coverage.Types.Point)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R14.RustCore.program _root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R14.RustCore.krate _root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R14.RustCore.flags (_root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R14.__enc_0 __s) (.adt 0) :=
  fun __s => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat (__s).x) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat (__s).y) _root_.LexLeanPreservation.Rust.wtl_nil))

theorem __wt_1 : ∀ (__s : (Coverage.Types.Box Nat)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R14.RustCore.program _root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R14.RustCore.krate _root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R14.RustCore.flags (_root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R14.__enc_1 __s) (.adt 1) :=
  fun __s => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat (__s).content) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat (__s).count) _root_.LexLeanPreservation.Rust.wtl_nil))

theorem __wt_2 : ∀ (__s : (Coverage.Types.Weights)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R14.RustCore.program _root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R14.RustCore.krate _root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R14.RustCore.flags (_root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R14.__enc_2 __s) (.adt 2) :=
>>>>>>>> gh/main:examples/production-coverage/expected/verify/preserve/LexLeanPreserve/Cc2030093ffdb7a3e8068874ff75532ac/R14/Compose/RustCore.lean
  fun __s => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat (__s).base) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat (__s).scale) _root_.LexLeanPreservation.Rust.wtl_nil))

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
<<<<<<<< HEAD:examples/production-coverage/expected/verify/preserve/LexLeanPreserve/C8d803bc64ccef7b33075ee3a67640390/R14/Compose/RustCore.lean
theorem root (a : Nat) (b : Nat) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanTarget.TargetSyntax.Value.nat a), (_root_.LexLeanTarget.TargetSyntax.Value.nat b)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R14.denote a b) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R14.RustCore.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanTarget.TargetSyntax.Value.nat a), (_root_.LexLeanTarget.TargetSyntax.Value.nat b)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R14.RustCore.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat a) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat b) _root_.LexLeanPreservation.Rust.wtl_nil)) rfl rfl (_root_.LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R14.root a b) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C8d803bc64ccef7b33075ee3a67640390.R14.Compose.RustCore
========
theorem root (a : Nat) (b : Nat) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanTarget.TargetSyntax.Value.nat a), (_root_.LexLeanTarget.TargetSyntax.Value.nat b)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R14.denote a b) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R14.RustCore.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanTarget.TargetSyntax.Value.nat a), (_root_.LexLeanTarget.TargetSyntax.Value.nat b)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R14.RustCore.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat a) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat b) _root_.LexLeanPreservation.Rust.wtl_nil)) rfl rfl (_root_.LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R14.root a b) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cc2030093ffdb7a3e8068874ff75532ac.R14.Compose.RustCore
>>>>>>>> gh/main:examples/production-coverage/expected/verify/preserve/LexLeanPreserve/Cc2030093ffdb7a3e8068874ff75532ac/R14/Compose/RustCore.lean
