import LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R2
import LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R2.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R2.Compose.RustStd

theorem __wt_0 : ∀ (__v : (Models.Triage.Presentation)), LexLeanPreservation.Rust.WT LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R2.RustStd.program LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R2.RustStd.krate LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R2.RustStd.flags (LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R2.__enc_0 __v) (.adt 0)
  | Models.Triage.Presentation.p0 => LexLeanPreservation.Rust.wt_adt rfl LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Presentation.p1 => LexLeanPreservation.Rust.wt_adt rfl LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Presentation.p2 => LexLeanPreservation.Rust.wt_adt rfl LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Presentation.p3 => LexLeanPreservation.Rust.wt_adt rfl LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Presentation.p4 => LexLeanPreservation.Rust.wt_adt rfl LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Presentation.p5 => LexLeanPreservation.Rust.wt_adt rfl LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Presentation.p6 => LexLeanPreservation.Rust.wt_adt rfl LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Presentation.p7 => LexLeanPreservation.Rust.wt_adt rfl LexLeanPreservation.Rust.wtl_nil

theorem __wt_1 : ∀ (__v : (Models.Triage.Level)), LexLeanPreservation.Rust.WT LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R2.RustStd.program LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R2.RustStd.krate LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R2.RustStd.flags (LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R2.__enc_1 __v) (.adt 1)
  | Models.Triage.Level.routine => LexLeanPreservation.Rust.wt_adt rfl LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Level.urgent => LexLeanPreservation.Rust.wt_adt rfl LexLeanPreservation.Rust.wtl_nil
  | Models.Triage.Level.emergency => LexLeanPreservation.Rust.wt_adt rfl LexLeanPreservation.Rust.wtl_nil

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (presentation : (Models.Triage.Presentation)) (hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R2.__enc_0 presentation)]) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R2.denote presentation) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R2.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R2.__enc_0 presentation)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R2.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (__wt_0 presentation) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R2.root presentation) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R2.Compose.RustStd
