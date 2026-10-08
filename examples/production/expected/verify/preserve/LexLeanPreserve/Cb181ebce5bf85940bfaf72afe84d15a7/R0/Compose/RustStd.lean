import LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R0
import LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R0.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R0.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (left : UInt32) (right : UInt32) (__e_hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanTarget.TargetSyntax.Value.u32 left), (LexLeanTarget.TargetSyntax.Value.u32 right)]) : ∃ __e_ro, LexLeanPreservation.Rust.RealizesFn false (LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R0.denote left right) __e_ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R0.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.u32 left), (LexLeanTarget.TargetSyntax.Value.u32 right)] __e_ro :=
  LexLeanPreservation.Rust.compose (fun __e_n => LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R0.RustStd.root __e_n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_u32 left) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_u32 right) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R0.root left right) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R0.Compose.RustStd
