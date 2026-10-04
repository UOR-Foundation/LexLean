import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R20
import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R20.RustCore
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R20.Compose.RustCore

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (n : Nat) (hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanTarget.TargetSyntax.Value.nat n)]) : ∃ ro, LexLeanPreservation.Rust.RealizesFn false (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R20.denote n) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R20.RustCore.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.nat n)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R20.RustCore.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat n) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R20.root n) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R20.Compose.RustCore
