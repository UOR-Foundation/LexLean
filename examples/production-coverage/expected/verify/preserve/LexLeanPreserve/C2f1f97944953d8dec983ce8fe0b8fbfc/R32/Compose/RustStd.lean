import LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R32
import LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R32.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R32.Compose.RustStd

theorem __wt_0 : ∀ (__v : (Coverage.Syntax.Term)), LexLeanPreservation.Rust.WT LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R32.RustStd.program LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R32.RustStd.krate LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R32.RustStd.flags (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R32.__enc_0 __v) (.adt 0)
  | Coverage.Syntax.Term.literal __x0 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat __x0) LexLeanPreservation.Rust.wtl_nil)
  | Coverage.Syntax.Term.plus __x0 __x1 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (__wt_0 __x0) (LexLeanPreservation.Rust.wtl_cons (__wt_0 __x1) LexLeanPreservation.Rust.wtl_nil))

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (t : (Coverage.Syntax.Term)) : ∃ ro, LexLeanPreservation.Rust.RealizesFn false (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R32.denote t) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R32.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R32.__enc_0 t)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R32.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (__wt_0 t) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R32.root t) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R32.Compose.RustStd
