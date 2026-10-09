import LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R2
import LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R2.RustCore
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R2.Compose.RustCore

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (value : Nat) (__e_hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanTarget.TargetSyntax.Value.nat value)]) : ∃ __e_ro, LexLeanPreservation.Rust.RealizesFn Bool.true (LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R2.denote value) __e_ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R2.RustCore.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.nat value)] __e_ro :=
  LexLeanPreservation.Rust.compose (fun __e_n => LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R2.RustCore.root __e_n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat value) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R2.root value) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R2.Compose.RustCore
