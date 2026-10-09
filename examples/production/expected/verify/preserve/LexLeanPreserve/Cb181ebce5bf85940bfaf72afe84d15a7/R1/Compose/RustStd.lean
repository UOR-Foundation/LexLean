import LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R1
import LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R1.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R1.Compose.RustStd

theorem __wt_0 : ∀ (__v : (Production.Kernel.Shape)), LexLeanPreservation.Rust.WT LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R1.RustStd.program LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R1.RustStd.krate LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R1.RustStd.flags (LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R1.__enc_0 __v) (.adt 0)
  | Production.Kernel.Shape.circle __x0 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat __x0) LexLeanPreservation.Rust.wtl_nil)
  | Production.Kernel.Shape.rectangle __x0 __x1 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat __x0) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat __x1) LexLeanPreservation.Rust.wtl_nil))

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (width : Nat) (height : Nat) (__e_hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanTarget.TargetSyntax.Value.nat width), (LexLeanTarget.TargetSyntax.Value.nat height)]) : ∃ __e_ro, LexLeanPreservation.Rust.RealizesFn Bool.true (LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R1.denote width height) __e_ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R1.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.nat width), (LexLeanTarget.TargetSyntax.Value.nat height)] __e_ro :=
  LexLeanPreservation.Rust.compose (fun __e_n => LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R1.RustStd.root __e_n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat width) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat height) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R1.root width height) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R1.Compose.RustStd
