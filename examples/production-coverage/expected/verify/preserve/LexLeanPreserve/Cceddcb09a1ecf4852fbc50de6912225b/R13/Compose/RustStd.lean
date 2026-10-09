import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R13
import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R13.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R13.Compose.RustStd

theorem __wt_0 : ∀ (__v : (Coverage.Types.Tree)), LexLeanPreservation.Rust.WT LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R13.RustStd.program LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R13.RustStd.krate LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R13.RustStd.flags (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R13.__enc_0 __v) (.adt 0)
  | Coverage.Types.Tree.leaf => LexLeanPreservation.Rust.wt_adt rfl LexLeanPreservation.Rust.wtl_nil
  | Coverage.Types.Tree.node __x0 __x1 __x2 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (__wt_0 __x0) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat __x1) (LexLeanPreservation.Rust.wtl_cons (__wt_0 __x2) LexLeanPreservation.Rust.wtl_nil)))

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (n : Nat) (__e_hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanTarget.TargetSyntax.Value.nat n)]) : ∃ __e_ro, LexLeanPreservation.Rust.RealizesFn Bool.true (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R13.denote n) __e_ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R13.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.nat n)] __e_ro :=
  LexLeanPreservation.Rust.compose (fun __e_n => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R13.RustStd.root __e_n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat n) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R13.root n) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R13.Compose.RustStd
