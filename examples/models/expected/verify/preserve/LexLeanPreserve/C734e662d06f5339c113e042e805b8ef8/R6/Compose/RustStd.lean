import LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R6
import LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R6.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R6.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (s : Nat) (x : Nat) (hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanTarget.TargetSyntax.Value.nat s), (LexLeanTarget.TargetSyntax.Value.nat x)]) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R6.denote s x) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R6.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.nat s), (LexLeanTarget.TargetSyntax.Value.nat x)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R6.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat s) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat x) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R6.root s x) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R6.Compose.RustStd
