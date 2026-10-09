import LexLeanPreserve.C55a45db3449637cd1dd5bfdb5faa7bb4.R0
import LexLeanPreserve.C55a45db3449637cd1dd5bfdb5faa7bb4.R0.RustCore
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.C55a45db3449637cd1dd5bfdb5faa7bb4.R0.Compose.RustCore

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (left : UInt32) (right : UInt32) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanTarget.TargetSyntax.Value.u32 left), (_root_.LexLeanTarget.TargetSyntax.Value.u32 right)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.false (_root_.LexLeanPreserve.C55a45db3449637cd1dd5bfdb5faa7bb4.R0.denote left right) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.C55a45db3449637cd1dd5bfdb5faa7bb4.R0.RustCore.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanTarget.TargetSyntax.Value.u32 left), (_root_.LexLeanTarget.TargetSyntax.Value.u32 right)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.C55a45db3449637cd1dd5bfdb5faa7bb4.R0.RustCore.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_u32 left) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_u32 right) _root_.LexLeanPreservation.Rust.wtl_nil)) rfl rfl (_root_.LexLeanPreserve.C55a45db3449637cd1dd5bfdb5faa7bb4.R0.root left right) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C55a45db3449637cd1dd5bfdb5faa7bb4.R0.Compose.RustCore
