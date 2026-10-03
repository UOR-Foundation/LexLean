import LexLeanPreserve.Cb9281f8ceb6cf07db1364c9823397fdf.R1
import LexLeanPreserve.Cb9281f8ceb6cf07db1364c9823397fdf.R1.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.Cb9281f8ceb6cf07db1364c9823397fdf.R1.Compose.RustStd

theorem __wt_0 : ∀ (__v : (Models.Glyphs.Glyph)), LexLeanPreservation.Rust.WT LexLeanPreserve.Cb9281f8ceb6cf07db1364c9823397fdf.R1.RustStd.program LexLeanPreserve.Cb9281f8ceb6cf07db1364c9823397fdf.R1.RustStd.krate LexLeanPreserve.Cb9281f8ceb6cf07db1364c9823397fdf.R1.RustStd.flags (LexLeanPreserve.Cb9281f8ceb6cf07db1364c9823397fdf.R1.__enc_0 __v) (.adt 0)
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
theorem root (glyph : (Models.Glyphs.Glyph)) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.Cb9281f8ceb6cf07db1364c9823397fdf.R1.denote glyph) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cb9281f8ceb6cf07db1364c9823397fdf.R1.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanPreserve.Cb9281f8ceb6cf07db1364c9823397fdf.R1.__enc_0 glyph)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.Cb9281f8ceb6cf07db1364c9823397fdf.R1.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (__wt_0 glyph) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.Cb9281f8ceb6cf07db1364c9823397fdf.R1.root glyph) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cb9281f8ceb6cf07db1364c9823397fdf.R1.Compose.RustStd
