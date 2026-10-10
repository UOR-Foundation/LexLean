import LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R6
import LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R6.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R6.Compose.RustStd

theorem __wt_0 : ∀ (__v : (Reasoning.Screening.Screen.Step)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R6.RustStd.program _root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R6.RustStd.krate _root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R6.RustStd.flags (_root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R6.__enc_0 __v) (.adt 0)
  | Reasoning.Screening.Screen.Step.Propose __x0 => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat __x0) _root_.LexLeanPreservation.Rust.wtl_nil)

theorem __wt_2 : ∀ (__s : (Reasoning.Screening.Screen.Node)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R6.RustStd.program _root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R6.RustStd.krate _root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R6.RustStd.flags (_root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R6.__enc_2 __s) (.adt 2) :=
  fun __s => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encPair _root_.LexLeanPreservation.Rust.wt_nat _root_.LexLeanPreservation.Rust.wt_nat) (__s).state) (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encList __wt_0) (__s).trace) _root_.LexLeanPreservation.Rust.wtl_nil))

theorem __wt_3 : ∀ (__s : (Reasoning.Screening.Screen.Ledger)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R6.RustStd.program _root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R6.RustStd.krate _root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R6.RustStd.flags (_root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R6.__enc_3 __s) (.adt 3) :=
  fun __s => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat (__s).iterations) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat (__s).attempts) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat (__s).firings) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat (__s).expansions) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat (__s).verifications) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat (__s).frontier) _root_.LexLeanPreservation.Rust.wtl_nil))))))

theorem __wt_1 : ∀ (__s : (Reasoning.Screening.Screen.Search)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R6.RustStd.program _root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R6.RustStd.krate _root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R6.RustStd.flags (_root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R6.__enc_1 __s) (.adt 1) :=
  fun __s => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encList __wt_2) (__s).frontier) (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encOption (_root_.LexLeanPreservation.Rust.wt_encPair _root_.LexLeanPreservation.Rust.wt_nat __wt_2)) (__s).found) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_bool (__s).truncated) (_root_.LexLeanPreservation.Rust.wtl_cons (__wt_3 (__s).ledger) _root_.LexLeanPreservation.Rust.wtl_nil))))

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (w : Nat) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanTarget.TargetSyntax.Value.nat w)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R6.denote w) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R6.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanTarget.TargetSyntax.Value.nat w)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R6.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_nat w) _root_.LexLeanPreservation.Rust.wtl_nil) rfl rfl (_root_.LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R6.root w) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R6.Compose.RustStd
