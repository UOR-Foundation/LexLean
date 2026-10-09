import LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R2
import LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R2.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R2.Compose.RustStd

theorem __wt_0 : ∀ (__v : (Models.Triage.Presentation)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R2.RustStd.program _root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R2.RustStd.krate _root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R2.RustStd.flags (_root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R2.__enc_0 __v) (.adt 0)
  | Models.Triage.Presentation.p0 => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Presentation.p1 => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Presentation.p2 => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Presentation.p3 => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Presentation.p4 => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Presentation.p5 => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Presentation.p6 => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Presentation.p7 => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil

theorem __wt_1 : ∀ (__v : (Models.Triage.Level)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R2.RustStd.program _root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R2.RustStd.krate _root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R2.RustStd.flags (_root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R2.__enc_1 __v) (.adt 1)
  | Models.Triage.Level.routine => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Level.urgent => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Level.emergency => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (presentation : (Models.Triage.Presentation)) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R2.__enc_0 presentation)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R2.denote presentation) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R2.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R2.__enc_0 presentation)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R2.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (__wt_0 presentation) _root_.LexLeanPreservation.Rust.wtl_nil) rfl rfl (_root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R2.root presentation) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R2.Compose.RustStd
