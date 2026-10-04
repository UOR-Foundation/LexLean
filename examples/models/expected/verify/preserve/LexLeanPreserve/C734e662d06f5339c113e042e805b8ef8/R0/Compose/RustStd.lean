import LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R0
import LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R0.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R0.Compose.RustStd

theorem __wt_0 : ∀ (__v : (Models.Glyphs.Glyph)), LexLeanPreservation.Rust.WT LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R0.RustStd.program LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R0.RustStd.krate LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R0.RustStd.flags (LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R0.__enc_0 __v) (.adt 0)
  | Models.Glyphs.Glyph.g0 => LexLeanPreservation.Rust.wt_adt rfl LexLeanPreservation.Rust.wtl_nil
  | Models.Glyphs.Glyph.g1 => LexLeanPreservation.Rust.wt_adt rfl LexLeanPreservation.Rust.wtl_nil
  | Models.Glyphs.Glyph.g2 => LexLeanPreservation.Rust.wt_adt rfl LexLeanPreservation.Rust.wtl_nil
  | Models.Glyphs.Glyph.g3 => LexLeanPreservation.Rust.wt_adt rfl LexLeanPreservation.Rust.wtl_nil
  | Models.Glyphs.Glyph.g4 => LexLeanPreservation.Rust.wt_adt rfl LexLeanPreservation.Rust.wtl_nil
  | Models.Glyphs.Glyph.g5 => LexLeanPreservation.Rust.wt_adt rfl LexLeanPreservation.Rust.wtl_nil
  | Models.Glyphs.Glyph.g6 => LexLeanPreservation.Rust.wt_adt rfl LexLeanPreservation.Rust.wtl_nil
  | Models.Glyphs.Glyph.g7 => LexLeanPreservation.Rust.wt_adt rfl LexLeanPreservation.Rust.wtl_nil
  | Models.Glyphs.Glyph.g8 => LexLeanPreservation.Rust.wt_adt rfl LexLeanPreservation.Rust.wtl_nil
  | Models.Glyphs.Glyph.g9 => LexLeanPreservation.Rust.wt_adt rfl LexLeanPreservation.Rust.wtl_nil

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (glyph : (Models.Glyphs.Glyph)) (hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R0.__enc_0 glyph)]) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R0.denote glyph) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R0.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R0.__enc_0 glyph)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R0.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (__wt_0 glyph) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R0.root glyph) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R0.Compose.RustStd
