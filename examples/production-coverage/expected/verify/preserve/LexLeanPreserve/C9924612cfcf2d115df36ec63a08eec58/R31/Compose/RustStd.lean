import LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R31
import LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R31.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R31.Compose.RustStd

mutual
theorem __wt_0 : ∀ (__v : (Coverage.Syntax.Expr)), LexLeanPreservation.Rust.WT LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R31.RustStd.program LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R31.RustStd.krate LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R31.RustStd.flags (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R31.__enc_0 __v) (.adt 0)
  | Coverage.Syntax.Expr.literal __x0 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat __x0) LexLeanPreservation.Rust.wtl_nil)
  | Coverage.Syntax.Expr.plus __x0 __x1 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (__wt_0 __x0) (LexLeanPreservation.Rust.wtl_cons (__wt_0 __x1) LexLeanPreservation.Rust.wtl_nil))
  | Coverage.Syntax.Expr.block __x0 __x1 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_list (__wtItems_0 __x0)) (LexLeanPreservation.Rust.wtl_cons (__wt_0 __x1) LexLeanPreservation.Rust.wtl_nil))

theorem __wt_1 : ∀ (__v : (Coverage.Syntax.Stmt)), LexLeanPreservation.Rust.WT LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R31.RustStd.program LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R31.RustStd.krate LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R31.RustStd.flags (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R31.__enc_1 __v) (.adt 1)
  | Coverage.Syntax.Stmt.assign __x0 __x1 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_string __x0) (LexLeanPreservation.Rust.wtl_cons (__wt_0 __x1) LexLeanPreservation.Rust.wtl_nil))
  | Coverage.Syntax.Stmt.sequence __x0 __x1 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (__wt_1 __x0) (LexLeanPreservation.Rust.wtl_cons (__wt_1 __x1) LexLeanPreservation.Rust.wtl_nil))

theorem __wtItems_0 : ∀ (__v : (List (Coverage.Syntax.Stmt))), LexLeanPreservation.Rust.WTAll LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R31.RustStd.program LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R31.RustStd.krate LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R31.RustStd.flags (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R31.__items_0 __v) (.adt 1)
  | [] => LexLeanPreservation.Rust.wtAll_nil
  | __x0 :: __x1 => LexLeanPreservation.Rust.wtAll_cons (__wt_1 __x0) (__wtItems_0 __x1)

end

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (e : (Coverage.Syntax.Expr)) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R31.denote e) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R31.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R31.__enc_0 e)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R31.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (__wt_0 e) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R31.root e) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R31.Compose.RustStd
