import LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R12
import LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R12.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R12.Compose.RustStd

theorem __wt_0 : ∀ (__v : (Coverage.Types.Tree)), LexLeanPreservation.Rust.WT LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R12.RustStd.program LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R12.RustStd.krate LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R12.RustStd.flags (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R12.__enc_0 __v) (.adt 0)
  | Coverage.Types.Tree.leaf => LexLeanPreservation.Rust.wt_adt rfl LexLeanPreservation.Rust.wtl_nil
  | Coverage.Types.Tree.node __x0 __x1 __x2 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (__wt_0 __x0) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat __x1) (LexLeanPreservation.Rust.wtl_cons (__wt_0 __x2) LexLeanPreservation.Rust.wtl_nil)))

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (n : Nat) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R12.denote n) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R12.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.nat n)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R12.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat n) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R12.root n) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R12.Compose.RustStd
