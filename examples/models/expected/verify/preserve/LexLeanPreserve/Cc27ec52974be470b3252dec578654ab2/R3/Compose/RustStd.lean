import LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R3
import LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R3.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R3.Compose.RustStd

theorem __wt_0 : ∀ (__s : (Models.Session.Window)), LexLeanPreservation.Rust.WT LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R3.RustStd.program LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R3.RustStd.krate LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R3.RustStd.flags (LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R3.__enc_0 __s) (.adt 0) :=
  fun __s => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat (__s).used) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat (__s).items) LexLeanPreservation.Rust.wtl_nil))

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (costs : (List Nat)) (__e_hrep : LexLeanPreservation.Rust.RepresentableL [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) costs)]) : ∃ __e_ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R3.denote costs) __e_ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R3.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) costs)] __e_ro :=
  LexLeanPreservation.Rust.compose (fun __e_n => LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R3.RustStd.root __e_n 0) rfl (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList LexLeanPreservation.Rust.wt_nat) costs) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R3.root costs) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R3.Compose.RustStd
