import LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R11
import LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R11.RustCore
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R11.Compose.RustCore

theorem __wt_0 : ∀ (__v : (Coverage.Types.Shape)), LexLeanPreservation.Rust.WT LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R11.RustCore.program LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R11.RustCore.krate LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R11.RustCore.flags (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R11.__enc_0 __v) (.adt 0)
  | Coverage.Types.Shape.circle __x0 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat __x0) LexLeanPreservation.Rust.wtl_nil)
  | Coverage.Types.Shape.rect __x0 __x1 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat __x0) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat __x1) LexLeanPreservation.Rust.wtl_nil))
  | Coverage.Types.Shape.empty => LexLeanPreservation.Rust.wt_adt rfl LexLeanPreservation.Rust.wtl_nil

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (w : Nat) (h : Nat) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R11.denote w h) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R11.RustCore.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.nat w), (LexLeanTarget.TargetSyntax.Value.nat h)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R11.RustCore.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat w) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat h) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R11.root w h) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R11.Compose.RustCore
