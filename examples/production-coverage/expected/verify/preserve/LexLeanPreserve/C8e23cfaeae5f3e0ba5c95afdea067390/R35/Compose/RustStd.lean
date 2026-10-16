import LexLeanPreserve.C8e23cfaeae5f3e0ba5c95afdea067390.R35
import LexLeanPreserve.C8e23cfaeae5f3e0ba5c95afdea067390.R35.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 0
set_option synthInstance.maxHeartbeats 0
set_option linter.unusedVariables false
namespace LexLeanPreserve.C8e23cfaeae5f3e0ba5c95afdea067390.R35.Compose.RustStd

theorem __wt_0 : ∀ (__v : (Coverage.Syntax.Term)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.C8e23cfaeae5f3e0ba5c95afdea067390.R35.RustStd.program _root_.LexLeanPreserve.C8e23cfaeae5f3e0ba5c95afdea067390.R35.RustStd.krate _root_.LexLeanPreserve.C8e23cfaeae5f3e0ba5c95afdea067390.R35.RustStd.flags (_root_.LexLeanPreserve.C8e23cfaeae5f3e0ba5c95afdea067390.R35.__enc_0 __v) (.adt 0)
  | Coverage.Syntax.Term.literal __x0 => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat __x0) _root_.LexLeanPreservation.Rust.wtl_nil)
  | Coverage.Syntax.Term.plus __x0 __x1 => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (__wt_0 __x0) (_root_.LexLeanPreservation.Rust.wtl_cons (__wt_0 __x1) _root_.LexLeanPreservation.Rust.wtl_nil))

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (t : (Coverage.Syntax.Term)) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanPreserve.C8e23cfaeae5f3e0ba5c95afdea067390.R35.__enc_0 t)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.false (_root_.LexLeanPreserve.C8e23cfaeae5f3e0ba5c95afdea067390.R35.denote t) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.C8e23cfaeae5f3e0ba5c95afdea067390.R35.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanPreserve.C8e23cfaeae5f3e0ba5c95afdea067390.R35.__enc_0 t)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.C8e23cfaeae5f3e0ba5c95afdea067390.R35.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (__wt_0 t) _root_.LexLeanPreservation.Rust.wtl_nil) rfl rfl (_root_.LexLeanPreserve.C8e23cfaeae5f3e0ba5c95afdea067390.R35.root t) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C8e23cfaeae5f3e0ba5c95afdea067390.R35.Compose.RustStd
