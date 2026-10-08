import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R18
import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R18.RustCore
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R18.Compose.RustCore

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (base : Nat) (offset : Nat) (__e_hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanTarget.TargetSyntax.Value.nat base), (LexLeanTarget.TargetSyntax.Value.nat offset)]) : ∃ __e_ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R18.denote base offset) __e_ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R18.RustCore.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.nat base), (LexLeanTarget.TargetSyntax.Value.nat offset)] __e_ro :=
  LexLeanPreservation.Rust.compose (fun __e_n => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R18.RustCore.root __e_n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat base) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat offset) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R18.root base offset) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R18.Compose.RustCore
