import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R24
import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R24.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R24.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (a : UInt16) (b : UInt16) (s : UInt32) (__e_hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanTarget.TargetSyntax.Value.u16 a), (LexLeanTarget.TargetSyntax.Value.u16 b), (LexLeanTarget.TargetSyntax.Value.u32 s)]) : ∃ __e_ro, LexLeanPreservation.Rust.RealizesFn Bool.false (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R24.denote a b s) __e_ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R24.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.u16 a), (LexLeanTarget.TargetSyntax.Value.u16 b), (LexLeanTarget.TargetSyntax.Value.u32 s)] __e_ro :=
  LexLeanPreservation.Rust.compose (fun __e_n => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R24.RustStd.root __e_n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_u16 a) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_u16 b) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_u32 s) LexLeanPreservation.Rust.wtl_nil))) rfl rfl (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R24.root a b s) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R24.Compose.RustStd
