import LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1
import LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1.Compose.RustStd

theorem __wt_0 : ∀ (__v : (Models.Glyphs.Glyph)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1.RustStd.program _root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1.RustStd.krate _root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1.RustStd.flags (_root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1.__enc_0 __v) (.adt 0)
  | Models.Glyphs.Glyph.g0 => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Glyphs.Glyph.g1 => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Glyphs.Glyph.g2 => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Glyphs.Glyph.g3 => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Glyphs.Glyph.g4 => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Glyphs.Glyph.g5 => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Glyphs.Glyph.g6 => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Glyphs.Glyph.g7 => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Glyphs.Glyph.g8 => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil
  | Models.Glyphs.Glyph.g9 => _root_.LexLeanPreservation.Rust.wt_adt rfl _root_.LexLeanPreservation.Rust.wtl_nil

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (glyph : (Models.Glyphs.Glyph)) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1.__enc_0 glyph)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1.denote glyph) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1.__enc_0 glyph)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (__wt_0 glyph) _root_.LexLeanPreservation.Rust.wtl_nil) rfl rfl (_root_.LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1.root glyph) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R1.Compose.RustStd
