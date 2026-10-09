import LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1
import LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1.Compose.RustStd

theorem __wt_0 : ∀ (__v : (Models.Glyphs.Glyph)), LexLeanPreservation.Rust.WT LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1.RustStd.program LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1.RustStd.krate LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1.RustStd.flags (LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1.__enc_0 __v) (.adt 0)
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
theorem root (glyph : (Models.Glyphs.Glyph)) (__e_hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1.__enc_0 glyph)]) : ∃ __e_ro, LexLeanPreservation.Rust.RealizesFn Bool.true (LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1.denote glyph) __e_ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1.__enc_0 glyph)] __e_ro :=
  LexLeanPreservation.Rust.compose (fun __e_n => LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1.RustStd.root __e_n 0) rfl (LexLeanPreservation.Rust.wtl_cons (__wt_0 glyph) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1.root glyph) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1.Compose.RustStd
