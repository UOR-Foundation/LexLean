import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33
import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33.Compose.RustStd

theorem __wt_0 : ∀ (__v : (Coverage.Syntax.Term)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33.RustStd.program _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33.RustStd.krate _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33.RustStd.flags (_root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33.__enc_0 __v) (.adt 0)
  | Coverage.Syntax.Term.literal __x0 => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat __x0) _root_.LexLeanPreservation.Rust.wtl_nil)
  | Coverage.Syntax.Term.plus __x0 __x1 => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (__wt_0 __x0) (_root_.LexLeanPreservation.Rust.wtl_cons (__wt_0 __x1) _root_.LexLeanPreservation.Rust.wtl_nil))

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (t : (Coverage.Syntax.Term)) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33.__enc_0 t)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.false (_root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33.denote t) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33.__enc_0 t)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (__wt_0 t) _root_.LexLeanPreservation.Rust.wtl_nil) rfl rfl (_root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33.root t) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33.Compose.RustStd
