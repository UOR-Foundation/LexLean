import LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R0
import LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R0.RustCore
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R0.Compose.RustCore

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (left : UInt32) (right : UInt32) (hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanTarget.TargetSyntax.Value.u32 left), (LexLeanTarget.TargetSyntax.Value.u32 right)]) : ∃ ro, LexLeanPreservation.Rust.RealizesFn false (LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R0.denote left right) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R0.RustCore.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.u32 left), (LexLeanTarget.TargetSyntax.Value.u32 right)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R0.RustCore.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_u32 left) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_u32 right) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R0.root left right) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R0.Compose.RustCore
