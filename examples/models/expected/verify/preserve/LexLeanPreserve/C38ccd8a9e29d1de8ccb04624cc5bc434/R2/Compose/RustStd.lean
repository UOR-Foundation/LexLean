<<<<<<<< HEAD:examples/models/expected/verify/preserve/LexLeanPreserve/C38ccd8a9e29d1de8ccb04624cc5bc434/R2/Compose/RustStd.lean
import LexLeanPreserve.C38ccd8a9e29d1de8ccb04624cc5bc434.R2
import LexLeanPreserve.C38ccd8a9e29d1de8ccb04624cc5bc434.R2.RustStd
========
import LexLeanPreserve.C6e8361ce3941a9dbed4a89b310886cf8.R2
import LexLeanPreserve.C6e8361ce3941a9dbed4a89b310886cf8.R2.RustStd
>>>>>>>> gh/main:examples/models/expected/verify/preserve/LexLeanPreserve/C6e8361ce3941a9dbed4a89b310886cf8/R2/Compose/RustStd.lean
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 0
set_option synthInstance.maxHeartbeats 0
set_option linter.unusedVariables false
<<<<<<<< HEAD:examples/models/expected/verify/preserve/LexLeanPreserve/C38ccd8a9e29d1de8ccb04624cc5bc434/R2/Compose/RustStd.lean
namespace LexLeanPreserve.C38ccd8a9e29d1de8ccb04624cc5bc434.R2.Compose.RustStd

theorem __wt_0 : ∀ (__v : (Models.Triage.Presentation)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.C38ccd8a9e29d1de8ccb04624cc5bc434.R2.RustStd.program _root_.LexLeanPreserve.C38ccd8a9e29d1de8ccb04624cc5bc434.R2.RustStd.krate _root_.LexLeanPreserve.C38ccd8a9e29d1de8ccb04624cc5bc434.R2.RustStd.flags (_root_.LexLeanPreserve.C38ccd8a9e29d1de8ccb04624cc5bc434.R2.__enc_0 __v) (.adt 0)
========
namespace LexLeanPreserve.C6e8361ce3941a9dbed4a89b310886cf8.R2.Compose.RustStd

theorem __wt_0 : ∀ (__v : (Models.Triage.Presentation)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.C6e8361ce3941a9dbed4a89b310886cf8.R2.RustStd.program _root_.LexLeanPreserve.C6e8361ce3941a9dbed4a89b310886cf8.R2.RustStd.krate _root_.LexLeanPreserve.C6e8361ce3941a9dbed4a89b310886cf8.R2.RustStd.flags (_root_.LexLeanPreserve.C6e8361ce3941a9dbed4a89b310886cf8.R2.__enc_0 __v) (.adt 0)
>>>>>>>> gh/main:examples/models/expected/verify/preserve/LexLeanPreserve/C6e8361ce3941a9dbed4a89b310886cf8/R2/Compose/RustStd.lean
  | Models.Triage.Presentation.p0 => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Presentation.p1 => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Presentation.p2 => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Presentation.p3 => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Presentation.p4 => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Presentation.p5 => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Presentation.p6 => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Presentation.p7 => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil

<<<<<<<< HEAD:examples/models/expected/verify/preserve/LexLeanPreserve/C38ccd8a9e29d1de8ccb04624cc5bc434/R2/Compose/RustStd.lean
theorem __wt_1 : ∀ (__v : (Models.Triage.Level)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.C38ccd8a9e29d1de8ccb04624cc5bc434.R2.RustStd.program _root_.LexLeanPreserve.C38ccd8a9e29d1de8ccb04624cc5bc434.R2.RustStd.krate _root_.LexLeanPreserve.C38ccd8a9e29d1de8ccb04624cc5bc434.R2.RustStd.flags (_root_.LexLeanPreserve.C38ccd8a9e29d1de8ccb04624cc5bc434.R2.__enc_1 __v) (.adt 1)
========
theorem __wt_1 : ∀ (__v : (Models.Triage.Level)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.C6e8361ce3941a9dbed4a89b310886cf8.R2.RustStd.program _root_.LexLeanPreserve.C6e8361ce3941a9dbed4a89b310886cf8.R2.RustStd.krate _root_.LexLeanPreserve.C6e8361ce3941a9dbed4a89b310886cf8.R2.RustStd.flags (_root_.LexLeanPreserve.C6e8361ce3941a9dbed4a89b310886cf8.R2.__enc_1 __v) (.adt 1)
>>>>>>>> gh/main:examples/models/expected/verify/preserve/LexLeanPreserve/C6e8361ce3941a9dbed4a89b310886cf8/R2/Compose/RustStd.lean
  | Models.Triage.Level.routine => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Level.urgent => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Level.emergency => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
<<<<<<<< HEAD:examples/models/expected/verify/preserve/LexLeanPreserve/C38ccd8a9e29d1de8ccb04624cc5bc434/R2/Compose/RustStd.lean
theorem root (presentation : (Models.Triage.Presentation)) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanPreserve.C38ccd8a9e29d1de8ccb04624cc5bc434.R2.__enc_0 presentation)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.C38ccd8a9e29d1de8ccb04624cc5bc434.R2.denote presentation) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.C38ccd8a9e29d1de8ccb04624cc5bc434.R2.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanPreserve.C38ccd8a9e29d1de8ccb04624cc5bc434.R2.__enc_0 presentation)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.C38ccd8a9e29d1de8ccb04624cc5bc434.R2.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (__wt_0 presentation) _root_.LexLeanPreservation.Rust.wtl_nil) rfl rfl (_root_.LexLeanPreserve.C38ccd8a9e29d1de8ccb04624cc5bc434.R2.root presentation) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C38ccd8a9e29d1de8ccb04624cc5bc434.R2.Compose.RustStd
========
theorem root (presentation : (Models.Triage.Presentation)) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanPreserve.C6e8361ce3941a9dbed4a89b310886cf8.R2.__enc_0 presentation)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.C6e8361ce3941a9dbed4a89b310886cf8.R2.denote presentation) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.C6e8361ce3941a9dbed4a89b310886cf8.R2.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanPreserve.C6e8361ce3941a9dbed4a89b310886cf8.R2.__enc_0 presentation)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.C6e8361ce3941a9dbed4a89b310886cf8.R2.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (__wt_0 presentation) _root_.LexLeanPreservation.Rust.wtl_nil) rfl rfl (_root_.LexLeanPreserve.C6e8361ce3941a9dbed4a89b310886cf8.R2.root presentation) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C6e8361ce3941a9dbed4a89b310886cf8.R2.Compose.RustStd
>>>>>>>> gh/main:examples/models/expected/verify/preserve/LexLeanPreserve/C6e8361ce3941a9dbed4a89b310886cf8/R2/Compose/RustStd.lean
