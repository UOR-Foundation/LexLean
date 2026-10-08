import LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R5
import LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R5.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R5.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (values : (List Nat)) (__e_hrep : LexLeanPreservation.Rust.RepresentableL [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) values)]) : ∃ __e_ro, LexLeanPreservation.Rust.RealizesFn false (LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R5.denote values) __e_ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R5.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) values)] __e_ro :=
  LexLeanPreservation.Rust.compose (fun __e_n => LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R5.RustStd.root __e_n 0) rfl (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList LexLeanPreservation.Rust.wt_nat) values) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R5.root values) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R5.Compose.RustStd
