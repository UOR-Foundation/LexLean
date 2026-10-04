import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33
import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33.Compose.RustStd

theorem __wt_0 : ∀ (__v : (Coverage.Syntax.Term)), LexLeanPreservation.Rust.WT LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33.RustStd.program LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33.RustStd.krate LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33.RustStd.flags (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33.__enc_0 __v) (.adt 0)
  | Coverage.Syntax.Term.literal __x0 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat __x0) LexLeanPreservation.Rust.wtl_nil)
  | Coverage.Syntax.Term.plus __x0 __x1 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (__wt_0 __x0) (LexLeanPreservation.Rust.wtl_cons (__wt_0 __x1) LexLeanPreservation.Rust.wtl_nil))

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (t : (Coverage.Syntax.Term)) (hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33.__enc_0 t)]) : ∃ ro, LexLeanPreservation.Rust.RealizesFn false (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33.denote t) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33.__enc_0 t)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (__wt_0 t) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33.root t) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R33.Compose.RustStd
