import LexLeanPreserve.C9b67141c19e48429ed1a6443d832fde7.R4
import LexLeanPreserve.C9b67141c19e48429ed1a6443d832fde7.R4.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.C9b67141c19e48429ed1a6443d832fde7.R4.Compose.RustStd

theorem __wt_0 : ∀ (__s : (Models.Session.Window)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.C9b67141c19e48429ed1a6443d832fde7.R4.RustStd.program _root_.LexLeanPreserve.C9b67141c19e48429ed1a6443d832fde7.R4.RustStd.krate _root_.LexLeanPreserve.C9b67141c19e48429ed1a6443d832fde7.R4.RustStd.flags (_root_.LexLeanPreserve.C9b67141c19e48429ed1a6443d832fde7.R4.__enc_0 __s) (.adt 0) :=
  fun __s => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat (__s).used) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat (__s).items) _root_.LexLeanPreservation.Rust.wtl_nil))

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (window : (Models.Session.Window)) (cost : Nat) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanPreserve.C9b67141c19e48429ed1a6443d832fde7.R4.__enc_0 window), (_root_.LexLeanTarget.TargetSyntax.Value.nat cost)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.C9b67141c19e48429ed1a6443d832fde7.R4.denote window cost) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.C9b67141c19e48429ed1a6443d832fde7.R4.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanPreserve.C9b67141c19e48429ed1a6443d832fde7.R4.__enc_0 window), (_root_.LexLeanTarget.TargetSyntax.Value.nat cost)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.C9b67141c19e48429ed1a6443d832fde7.R4.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (__wt_0 window) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat cost) _root_.LexLeanPreservation.Rust.wtl_nil)) rfl rfl (_root_.LexLeanPreserve.C9b67141c19e48429ed1a6443d832fde7.R4.root window cost) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C9b67141c19e48429ed1a6443d832fde7.R4.Compose.RustStd
