<<<<<<<< HEAD:examples/models/expected/verify/preserve/LexLeanPreserve/C38ccd8a9e29d1de8ccb04624cc5bc434/R8/Compose/RustStd.lean
import LexLeanPreserve.C38ccd8a9e29d1de8ccb04624cc5bc434.R8
import LexLeanPreserve.C38ccd8a9e29d1de8ccb04624cc5bc434.R8.RustStd
========
import LexLeanPreserve.C6e8361ce3941a9dbed4a89b310886cf8.R8
import LexLeanPreserve.C6e8361ce3941a9dbed4a89b310886cf8.R8.RustStd
>>>>>>>> gh/main:examples/models/expected/verify/preserve/LexLeanPreserve/C6e8361ce3941a9dbed4a89b310886cf8/R8/Compose/RustStd.lean
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 0
set_option synthInstance.maxHeartbeats 0
set_option linter.unusedVariables false
<<<<<<<< HEAD:examples/models/expected/verify/preserve/LexLeanPreserve/C38ccd8a9e29d1de8ccb04624cc5bc434/R8/Compose/RustStd.lean
namespace LexLeanPreserve.C38ccd8a9e29d1de8ccb04624cc5bc434.R8.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (s : Nat) (x : Nat) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanTarget.TargetSyntax.Value.nat s), (_root_.LexLeanTarget.TargetSyntax.Value.nat x)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.C38ccd8a9e29d1de8ccb04624cc5bc434.R8.denote s x) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.C38ccd8a9e29d1de8ccb04624cc5bc434.R8.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanTarget.TargetSyntax.Value.nat s), (_root_.LexLeanTarget.TargetSyntax.Value.nat x)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.C38ccd8a9e29d1de8ccb04624cc5bc434.R8.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat s) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat x) _root_.LexLeanPreservation.Rust.wtl_nil)) rfl rfl (_root_.LexLeanPreserve.C38ccd8a9e29d1de8ccb04624cc5bc434.R8.root s x) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C38ccd8a9e29d1de8ccb04624cc5bc434.R8.Compose.RustStd
========
namespace LexLeanPreserve.C6e8361ce3941a9dbed4a89b310886cf8.R8.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (s : Nat) (x : Nat) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanTarget.TargetSyntax.Value.nat s), (_root_.LexLeanTarget.TargetSyntax.Value.nat x)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.C6e8361ce3941a9dbed4a89b310886cf8.R8.denote s x) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.C6e8361ce3941a9dbed4a89b310886cf8.R8.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanTarget.TargetSyntax.Value.nat s), (_root_.LexLeanTarget.TargetSyntax.Value.nat x)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.C6e8361ce3941a9dbed4a89b310886cf8.R8.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat s) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat x) _root_.LexLeanPreservation.Rust.wtl_nil)) rfl rfl (_root_.LexLeanPreserve.C6e8361ce3941a9dbed4a89b310886cf8.R8.root s x) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C6e8361ce3941a9dbed4a89b310886cf8.R8.Compose.RustStd
>>>>>>>> gh/main:examples/models/expected/verify/preserve/LexLeanPreserve/C6e8361ce3941a9dbed4a89b310886cf8/R8/Compose/RustStd.lean
