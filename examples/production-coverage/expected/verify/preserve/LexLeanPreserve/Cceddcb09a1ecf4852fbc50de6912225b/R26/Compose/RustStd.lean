import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R26
import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R26.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R26.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (xs : (List Nat)) (b : ByteArray) (i : Nat) (__e_hrep : LexLeanPreservation.Rust.RepresentableL [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) xs), (LexLeanTarget.TargetSyntax.Value.bytes b), (LexLeanTarget.TargetSyntax.Value.nat i)]) : ∃ __e_ro, LexLeanPreservation.Rust.RealizesFn Bool.true (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R26.denote xs b i) __e_ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R26.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) xs), (LexLeanTarget.TargetSyntax.Value.bytes b), (LexLeanTarget.TargetSyntax.Value.nat i)] __e_ro :=
  LexLeanPreservation.Rust.compose (fun __e_n => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R26.RustStd.root __e_n 0) rfl (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList LexLeanPreservation.Rust.wt_nat) xs) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_bytes b) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat i) LexLeanPreservation.Rust.wtl_nil))) rfl rfl (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R26.root xs b i) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R26.Compose.RustStd
