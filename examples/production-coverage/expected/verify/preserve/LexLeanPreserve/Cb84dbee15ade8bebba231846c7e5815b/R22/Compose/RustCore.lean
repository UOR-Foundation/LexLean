import LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R22
import LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R22.RustCore
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R22.Compose.RustCore

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (a : Int16) (b : Int32) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanTarget.TargetSyntax.Value.i16 a), (_root_.LexLeanTarget.TargetSyntax.Value.i32 b)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.false (_root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R22.denote a b) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R22.RustCore.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanTarget.TargetSyntax.Value.i16 a), (_root_.LexLeanTarget.TargetSyntax.Value.i32 b)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R22.RustCore.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_i16 a) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_i32 b) _root_.LexLeanPreservation.Rust.wtl_nil)) rfl rfl (_root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R22.root a b) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R22.Compose.RustCore
