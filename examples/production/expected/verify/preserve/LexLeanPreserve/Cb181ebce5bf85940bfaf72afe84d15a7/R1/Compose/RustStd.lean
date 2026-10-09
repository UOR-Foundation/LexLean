import LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R1
import LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R1.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R1.Compose.RustStd

theorem __wt_0 : ∀ (__v : (Production.Kernel.Shape)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R1.RustStd.program _root_.LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R1.RustStd.krate _root_.LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R1.RustStd.flags (_root_.LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R1.__enc_0 __v) (.adt 0)
  | Production.Kernel.Shape.circle __x0 => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat __x0) _root_.LexLeanPreservation.Rust.wtl_nil)
  | Production.Kernel.Shape.rectangle __x0 __x1 => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat __x0) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat __x1) _root_.LexLeanPreservation.Rust.wtl_nil))

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (width : Nat) (height : Nat) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanTarget.TargetSyntax.Value.nat width), (_root_.LexLeanTarget.TargetSyntax.Value.nat height)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R1.denote width height) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R1.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanTarget.TargetSyntax.Value.nat width), (_root_.LexLeanTarget.TargetSyntax.Value.nat height)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R1.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat width) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat height) _root_.LexLeanPreservation.Rust.wtl_nil)) rfl rfl (_root_.LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R1.root width height) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R1.Compose.RustStd
