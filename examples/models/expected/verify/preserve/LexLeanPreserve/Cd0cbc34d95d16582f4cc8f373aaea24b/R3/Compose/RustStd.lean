import LexLeanPreserve.Cd0cbc34d95d16582f4cc8f373aaea24b.R3
import LexLeanPreserve.Cd0cbc34d95d16582f4cc8f373aaea24b.R3.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cd0cbc34d95d16582f4cc8f373aaea24b.R3.Compose.RustStd

theorem __wt_0 : ∀ (__s : (Models.Session.Window)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.Cd0cbc34d95d16582f4cc8f373aaea24b.R3.RustStd.program _root_.LexLeanPreserve.Cd0cbc34d95d16582f4cc8f373aaea24b.R3.RustStd.krate _root_.LexLeanPreserve.Cd0cbc34d95d16582f4cc8f373aaea24b.R3.RustStd.flags (_root_.LexLeanPreserve.Cd0cbc34d95d16582f4cc8f373aaea24b.R3.__enc_0 __s) (.adt 0) :=
  fun __s => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat (__s).used) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat (__s).items) _root_.LexLeanPreservation.Rust.wtl_nil))

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (costs : (List Nat)) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.nat) costs)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.Cd0cbc34d95d16582f4cc8f373aaea24b.R3.denote costs) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Cd0cbc34d95d16582f4cc8f373aaea24b.R3.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.nat) costs)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Cd0cbc34d95d16582f4cc8f373aaea24b.R3.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encList _root_.LexLeanPreservation.Rust.wt_nat) costs) _root_.LexLeanPreservation.Rust.wtl_nil) rfl rfl (_root_.LexLeanPreserve.Cd0cbc34d95d16582f4cc8f373aaea24b.R3.root costs) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cd0cbc34d95d16582f4cc8f373aaea24b.R3.Compose.RustStd
