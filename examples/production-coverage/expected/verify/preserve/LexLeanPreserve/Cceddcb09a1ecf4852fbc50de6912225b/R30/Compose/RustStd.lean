import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R30
import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R30.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R30.Compose.RustStd

mutual
theorem __wt_0 : ∀ (__v : (Coverage.Syntax.Rose Nat)), LexLeanPreservation.Rust.WT LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R30.RustStd.program LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R30.RustStd.krate LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R30.RustStd.flags (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R30.__enc_0 __v) (.adt 0)
  | Coverage.Syntax.Rose.node __x0 __x1 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat __x0) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_list (__wtItems_0 __x1)) LexLeanPreservation.Rust.wtl_nil))

theorem __wtItems_0 : ∀ (__v : (List (Coverage.Syntax.Rose Nat))), LexLeanPreservation.Rust.WTAll LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R30.RustStd.program LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R30.RustStd.krate LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R30.RustStd.flags (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R30.__items_0 __v) (.adt 0)
  | [] => LexLeanPreservation.Rust.wtAll_nil
  | __x0 :: __x1 => LexLeanPreservation.Rust.wtAll_cons (__wt_0 __x0) (__wtItems_0 __x1)

end

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (n : Nat) (__e_hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanTarget.TargetSyntax.Value.nat n)]) : ∃ __e_ro, LexLeanPreservation.Rust.RealizesFn Bool.true (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R30.denote n) __e_ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R30.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.nat n)] __e_ro :=
  LexLeanPreservation.Rust.compose (fun __e_n => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R30.RustStd.root __e_n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat n) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R30.root n) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R30.Compose.RustStd
