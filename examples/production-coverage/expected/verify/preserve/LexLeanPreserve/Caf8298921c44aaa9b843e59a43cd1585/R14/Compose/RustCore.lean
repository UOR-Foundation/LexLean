import LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R14
import LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R14.RustCore
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R14.Compose.RustCore

theorem __wt_0 : ∀ (__s : (Coverage.Types.Point)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R14.RustCore.program _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R14.RustCore.krate _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R14.RustCore.flags (_root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R14.__enc_0 __s) (.adt 0) :=
  fun __s => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat (__s).x) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat (__s).y) _root_.LexLeanPreservation.Rust.wtl_nil))

theorem __wt_1 : ∀ (__s : (Coverage.Types.Box Nat)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R14.RustCore.program _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R14.RustCore.krate _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R14.RustCore.flags (_root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R14.__enc_1 __s) (.adt 1) :=
  fun __s => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat (__s).content) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat (__s).count) _root_.LexLeanPreservation.Rust.wtl_nil))

theorem __wt_2 : ∀ (__s : (Coverage.Types.Weights)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R14.RustCore.program _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R14.RustCore.krate _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R14.RustCore.flags (_root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R14.__enc_2 __s) (.adt 2) :=
  fun __s => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat (__s).base) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat (__s).scale) _root_.LexLeanPreservation.Rust.wtl_nil))

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (a : Nat) (b : Nat) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanTarget.TargetSyntax.Value.nat a), (_root_.LexLeanTarget.TargetSyntax.Value.nat b)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R14.denote a b) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R14.RustCore.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanTarget.TargetSyntax.Value.nat a), (_root_.LexLeanTarget.TargetSyntax.Value.nat b)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R14.RustCore.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat a) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat b) _root_.LexLeanPreservation.Rust.wtl_nil)) rfl rfl (_root_.LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R14.root a b) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R14.Compose.RustCore
