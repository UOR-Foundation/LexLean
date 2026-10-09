import LexLeanPreserve.C55a45db3449637cd1dd5bfdb5faa7bb4.R3
import LexLeanPreserve.C55a45db3449637cd1dd5bfdb5faa7bb4.R3.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.C55a45db3449637cd1dd5bfdb5faa7bb4.R3.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (number : Nat) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanTarget.TargetSyntax.Value.nat number)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.C55a45db3449637cd1dd5bfdb5faa7bb4.R3.denote number) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.C55a45db3449637cd1dd5bfdb5faa7bb4.R3.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanTarget.TargetSyntax.Value.nat number)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.C55a45db3449637cd1dd5bfdb5faa7bb4.R3.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat number) _root_.LexLeanPreservation.Rust.wtl_nil) rfl rfl (_root_.LexLeanPreserve.C55a45db3449637cd1dd5bfdb5faa7bb4.R3.root number) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C55a45db3449637cd1dd5bfdb5faa7bb4.R3.Compose.RustStd
