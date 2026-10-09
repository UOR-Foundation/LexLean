import LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R5
import LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R5.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R5.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (s : Nat) (x : Nat) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanTarget.TargetSyntax.Value.nat s), (_root_.LexLeanTarget.TargetSyntax.Value.nat x)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R5.denote s x) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R5.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanTarget.TargetSyntax.Value.nat s), (_root_.LexLeanTarget.TargetSyntax.Value.nat x)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R5.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat s) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat x) _root_.LexLeanPreservation.Rust.wtl_nil)) rfl rfl (_root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R5.root s x) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R5.Compose.RustStd
