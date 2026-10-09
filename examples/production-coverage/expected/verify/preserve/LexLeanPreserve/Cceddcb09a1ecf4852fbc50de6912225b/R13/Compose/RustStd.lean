import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R13
import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R13.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R13.Compose.RustStd

theorem __wt_0 : ∀ (__v : (Coverage.Types.Tree)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R13.RustStd.program _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R13.RustStd.krate _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R13.RustStd.flags (_root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R13.__enc_0 __v) (.adt 0)
  | Coverage.Types.Tree.leaf => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Coverage.Types.Tree.node __x0 __x1 __x2 => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (__wt_0 __x0) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat __x1) (_root_.LexLeanPreservation.Rust.wtl_cons (__wt_0 __x2) _root_.LexLeanPreservation.Rust.wtl_nil)))

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (n : Nat) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanTarget.TargetSyntax.Value.nat n)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R13.denote n) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R13.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanTarget.TargetSyntax.Value.nat n)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R13.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat n) _root_.LexLeanPreservation.Rust.wtl_nil) rfl rfl (_root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R13.root n) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R13.Compose.RustStd
