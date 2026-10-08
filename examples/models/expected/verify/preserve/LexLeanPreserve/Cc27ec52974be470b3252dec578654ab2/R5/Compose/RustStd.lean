import LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R5
import LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R5.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R5.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (s : Nat) (x : Nat) (__e_hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanTarget.TargetSyntax.Value.nat s), (LexLeanTarget.TargetSyntax.Value.nat x)]) : ∃ __e_ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R5.denote s x) __e_ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R5.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.nat s), (LexLeanTarget.TargetSyntax.Value.nat x)] __e_ro :=
  LexLeanPreservation.Rust.compose (fun __e_n => LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R5.RustStd.root __e_n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat s) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat x) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R5.root s x) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R5.Compose.RustStd
