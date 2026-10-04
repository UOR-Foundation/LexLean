import LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R2
import LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R2.RustCore
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R2.Compose.RustCore

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (value : Nat) (hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanTarget.TargetSyntax.Value.nat value)]) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R2.denote value) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R2.RustCore.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.nat value)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R2.RustCore.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat value) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R2.root value) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R2.Compose.RustCore
