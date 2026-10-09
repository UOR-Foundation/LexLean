import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R15
import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R15.RustCore
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R15.Compose.RustCore

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (v : (Option Nat)) (r : (Except Bool Nat)) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [((_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.nat) v), ((_root_.LexLeanPreservation.encExcept _root_.LexLeanTarget.TargetSyntax.Value.bool _root_.LexLeanTarget.TargetSyntax.Value.nat) r)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R15.denote v r) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R15.RustCore.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [((_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.nat) v), ((_root_.LexLeanPreservation.encExcept _root_.LexLeanTarget.TargetSyntax.Value.bool _root_.LexLeanTarget.TargetSyntax.Value.nat) r)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R15.RustCore.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encOption _root_.LexLeanPreservation.Rust.wt_nat) v) (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encExcept _root_.LexLeanPreservation.Rust.wt_bool _root_.LexLeanPreservation.Rust.wt_nat) r) _root_.LexLeanPreservation.Rust.wtl_nil)) rfl rfl (_root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R15.root v r) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R15.Compose.RustCore
