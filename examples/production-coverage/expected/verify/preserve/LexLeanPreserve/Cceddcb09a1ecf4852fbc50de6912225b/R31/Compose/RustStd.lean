import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R31
import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R31.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R31.Compose.RustStd

mutual
theorem __wt_0 : ∀ (__v : (Coverage.Syntax.Expr)), LexLeanPreservation.Rust.WT LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R31.RustStd.program LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R31.RustStd.krate LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R31.RustStd.flags (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R31.__enc_0 __v) (.adt 0)
  | Coverage.Syntax.Expr.literal __x0 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat __x0) LexLeanPreservation.Rust.wtl_nil)
  | Coverage.Syntax.Expr.plus __x0 __x1 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (__wt_0 __x0) (LexLeanPreservation.Rust.wtl_cons (__wt_0 __x1) LexLeanPreservation.Rust.wtl_nil))
  | Coverage.Syntax.Expr.block __x0 __x1 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_list (__wtItems_0 __x0)) (LexLeanPreservation.Rust.wtl_cons (__wt_0 __x1) LexLeanPreservation.Rust.wtl_nil))

theorem __wt_1 : ∀ (__v : (Coverage.Syntax.Stmt)), LexLeanPreservation.Rust.WT LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R31.RustStd.program LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R31.RustStd.krate LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R31.RustStd.flags (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R31.__enc_1 __v) (.adt 1)
  | Coverage.Syntax.Stmt.assign __x0 __x1 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_string __x0) (LexLeanPreservation.Rust.wtl_cons (__wt_0 __x1) LexLeanPreservation.Rust.wtl_nil))
  | Coverage.Syntax.Stmt.sequence __x0 __x1 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (__wt_1 __x0) (LexLeanPreservation.Rust.wtl_cons (__wt_1 __x1) LexLeanPreservation.Rust.wtl_nil))

theorem __wtItems_0 : ∀ (__v : (List (Coverage.Syntax.Stmt))), LexLeanPreservation.Rust.WTAll LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R31.RustStd.program LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R31.RustStd.krate LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R31.RustStd.flags (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R31.__items_0 __v) (.adt 1)
  | [] => LexLeanPreservation.Rust.wtAll_nil
  | __x0 :: __x1 => LexLeanPreservation.Rust.wtAll_cons (__wt_1 __x0) (__wtItems_0 __x1)

end

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (e : (Coverage.Syntax.Expr)) (__e_hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R31.__enc_0 e)]) : ∃ __e_ro, LexLeanPreservation.Rust.RealizesFn Bool.true (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R31.denote e) __e_ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R31.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R31.__enc_0 e)] __e_ro :=
  LexLeanPreservation.Rust.compose (fun __e_n => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R31.RustStd.root __e_n 0) rfl (LexLeanPreservation.Rust.wtl_cons (__wt_0 e) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R31.root e) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R31.Compose.RustStd
