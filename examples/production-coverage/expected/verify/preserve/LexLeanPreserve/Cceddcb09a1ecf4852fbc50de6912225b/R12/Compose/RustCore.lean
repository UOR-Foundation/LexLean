import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R12
import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R12.RustCore
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R12.Compose.RustCore

theorem __wt_0 : ∀ (__v : (Coverage.Types.Shape)), LexLeanPreservation.Rust.WT LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R12.RustCore.program LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R12.RustCore.krate LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R12.RustCore.flags (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R12.__enc_0 __v) (.adt 0)
  | Coverage.Types.Shape.circle __x0 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat __x0) LexLeanPreservation.Rust.wtl_nil)
  | Coverage.Types.Shape.rect __x0 __x1 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat __x0) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat __x1) LexLeanPreservation.Rust.wtl_nil))
  | Coverage.Types.Shape.empty => LexLeanPreservation.Rust.wt_adt rfl LexLeanPreservation.Rust.wtl_nil

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (w : Nat) (h : Nat) (__e_hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanTarget.TargetSyntax.Value.nat w), (LexLeanTarget.TargetSyntax.Value.nat h)]) : ∃ __e_ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R12.denote w h) __e_ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R12.RustCore.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.nat w), (LexLeanTarget.TargetSyntax.Value.nat h)] __e_ro :=
  LexLeanPreservation.Rust.compose (fun __e_n => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R12.RustCore.root __e_n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat w) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat h) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R12.root w h) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R12.Compose.RustCore
