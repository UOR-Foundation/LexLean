import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R15
import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R15.RustCore
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R15.Compose.RustCore

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (v : (Option Nat)) (r : (Except Bool Nat)) (__e_hrep : LexLeanPreservation.Rust.RepresentableL [((LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.nat) v), ((LexLeanPreservation.encExcept LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.nat) r)]) : ∃ __e_ro, LexLeanPreservation.Rust.RealizesFn Bool.true (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R15.denote v r) __e_ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R15.RustCore.krate (LexLeanPreservation.Rust.fnIdent 0) [((LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.nat) v), ((LexLeanPreservation.encExcept LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.nat) r)] __e_ro :=
  LexLeanPreservation.Rust.compose (fun __e_n => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R15.RustCore.root __e_n 0) rfl (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encOption LexLeanPreservation.Rust.wt_nat) v) (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encExcept LexLeanPreservation.Rust.wt_bool LexLeanPreservation.Rust.wt_nat) r) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R15.root v r) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R15.Compose.RustCore
