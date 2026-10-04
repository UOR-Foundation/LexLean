import LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R3
import LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R3.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R3.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (number : Nat) (hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanTarget.TargetSyntax.Value.nat number)]) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R3.denote number) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R3.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.nat number)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R3.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat number) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R3.root number) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R3.Compose.RustStd
