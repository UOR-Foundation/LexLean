<<<<<<<< HEAD:examples/production/expected/verify/preserve/LexLeanPreserve/C8b4ddd633568e02cb00e48501ea9dbc5/R3/Compose/RustStd.lean
import LexLeanPreserve.C8b4ddd633568e02cb00e48501ea9dbc5.R3
import LexLeanPreserve.C8b4ddd633568e02cb00e48501ea9dbc5.R3.RustStd
========
import LexLeanPreserve.C3950237c975b30f5dd9298b8e09919c9.R3
import LexLeanPreserve.C3950237c975b30f5dd9298b8e09919c9.R3.RustStd
>>>>>>>> gh/main:examples/production/expected/verify/preserve/LexLeanPreserve/C3950237c975b30f5dd9298b8e09919c9/R3/Compose/RustStd.lean
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 0
set_option synthInstance.maxHeartbeats 0
set_option linter.unusedVariables false
<<<<<<<< HEAD:examples/production/expected/verify/preserve/LexLeanPreserve/C8b4ddd633568e02cb00e48501ea9dbc5/R3/Compose/RustStd.lean
namespace LexLeanPreserve.C8b4ddd633568e02cb00e48501ea9dbc5.R3.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (number : Nat) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanTarget.TargetSyntax.Value.nat number)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.C8b4ddd633568e02cb00e48501ea9dbc5.R3.denote number) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.C8b4ddd633568e02cb00e48501ea9dbc5.R3.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanTarget.TargetSyntax.Value.nat number)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.C8b4ddd633568e02cb00e48501ea9dbc5.R3.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat number) _root_.LexLeanPreservation.Rust.wtl_nil) rfl rfl (_root_.LexLeanPreserve.C8b4ddd633568e02cb00e48501ea9dbc5.R3.root number) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C8b4ddd633568e02cb00e48501ea9dbc5.R3.Compose.RustStd
========
namespace LexLeanPreserve.C3950237c975b30f5dd9298b8e09919c9.R3.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (number : Nat) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanTarget.TargetSyntax.Value.nat number)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.C3950237c975b30f5dd9298b8e09919c9.R3.denote number) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.C3950237c975b30f5dd9298b8e09919c9.R3.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanTarget.TargetSyntax.Value.nat number)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.C3950237c975b30f5dd9298b8e09919c9.R3.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat number) _root_.LexLeanPreservation.Rust.wtl_nil) rfl rfl (_root_.LexLeanPreserve.C3950237c975b30f5dd9298b8e09919c9.R3.root number) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C3950237c975b30f5dd9298b8e09919c9.R3.Compose.RustStd
>>>>>>>> gh/main:examples/production/expected/verify/preserve/LexLeanPreserve/C3950237c975b30f5dd9298b8e09919c9/R3/Compose/RustStd.lean
