import LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R7
import LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R7.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R7.Compose.RustStd

theorem __wt_0 : ∀ (__v : (Reasoning.Budget.Spend.Step Nat)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R7.RustStd.program _root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R7.RustStd.krate _root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R7.RustStd.flags (_root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R7.__enc_0 __v) (.adt 0)
  | Reasoning.Budget.Spend.Step.Tick => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Reasoning.Budget.Spend.Step.Burn __x0 => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat __x0) _root_.LexLeanPreservation.Rust.wtl_nil)

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (x : Nat) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanTarget.TargetSyntax.Value.nat x)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R7.denote x) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R7.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanTarget.TargetSyntax.Value.nat x)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R7.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat x) _root_.LexLeanPreservation.Rust.wtl_nil) rfl rfl (_root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R7.root x) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R7.Compose.RustStd
