import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R17
import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R17.RustCore
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R17.Compose.RustCore

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (p : (Prod Nat Bool)) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [((_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.nat _root_.LexLeanTarget.TargetSyntax.Value.bool) p)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.false (_root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R17.denote p) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R17.RustCore.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [((_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.nat _root_.LexLeanTarget.TargetSyntax.Value.bool) p)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R17.RustCore.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encPair _root_.LexLeanPreservation.Rust.wt_nat _root_.LexLeanPreservation.Rust.wt_bool) p) _root_.LexLeanPreservation.Rust.wtl_nil) rfl rfl (_root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R17.root p) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R17.Compose.RustCore
