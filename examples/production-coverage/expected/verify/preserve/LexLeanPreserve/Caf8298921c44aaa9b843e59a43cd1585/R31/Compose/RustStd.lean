import LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R31
import LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R31.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R31.Compose.RustStd

mutual
theorem __wt_0 : ∀ (__v : (Coverage.Syntax.Expr)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R31.RustStd.program _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R31.RustStd.krate _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R31.RustStd.flags (_root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R31.__enc_0 __v) (.adt 0)
  | Coverage.Syntax.Expr.literal __x0 => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat __x0) _root_.LexLeanPreservation.Rust.wtl_nil)
  | Coverage.Syntax.Expr.plus __x0 __x1 => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (__wt_0 __x0) (_root_.LexLeanPreservation.Rust.wtl_cons (__wt_0 __x1) _root_.LexLeanPreservation.Rust.wtl_nil))
  | Coverage.Syntax.Expr.block __x0 __x1 => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_list (__wtItems_0 __x0)) (_root_.LexLeanPreservation.Rust.wtl_cons (__wt_0 __x1) _root_.LexLeanPreservation.Rust.wtl_nil))

theorem __wt_1 : ∀ (__v : (Coverage.Syntax.Stmt)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R31.RustStd.program _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R31.RustStd.krate _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R31.RustStd.flags (_root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R31.__enc_1 __v) (.adt 1)
  | Coverage.Syntax.Stmt.assign __x0 __x1 => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_string __x0) (_root_.LexLeanPreservation.Rust.wtl_cons (__wt_0 __x1) _root_.LexLeanPreservation.Rust.wtl_nil))
  | Coverage.Syntax.Stmt.sequence __x0 __x1 => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (__wt_1 __x0) (_root_.LexLeanPreservation.Rust.wtl_cons (__wt_1 __x1) _root_.LexLeanPreservation.Rust.wtl_nil))

theorem __wtItems_0 : ∀ (__v : (List (Coverage.Syntax.Stmt))), _root_.LexLeanPreservation.Rust.WTAll _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R31.RustStd.program _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R31.RustStd.krate _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R31.RustStd.flags (_root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R31.__items_0 __v) (.adt 1)
  | [] => _root_.LexLeanPreservation.Rust.wtAll_nil
  | __x0 :: __x1 => _root_.LexLeanPreservation.Rust.wtAll_cons (__wt_1 __x0) (__wtItems_0 __x1)

end

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (e : (Coverage.Syntax.Expr)) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R31.__enc_0 e)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R31.denote e) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R31.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R31.__enc_0 e)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R31.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (__wt_0 e) _root_.LexLeanPreservation.Rust.wtl_nil) rfl rfl (_root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R31.root e) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R31.Compose.RustStd
