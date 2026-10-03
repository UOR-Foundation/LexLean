import LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R0
import LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R0.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R0.Compose.RustStd

theorem __wt_0 : ∀ (__v : (Models.Glyphs.Glyph)), LexLeanPreservation.Rust.WT LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R0.RustStd.program LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R0.RustStd.krate LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R0.RustStd.flags (LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R0.__enc_0 __v) (.adt 0)
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
theorem root (glyph : (Models.Glyphs.Glyph)) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R0.denote glyph) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R0.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R0.__enc_0 glyph)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R0.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (__wt_0 glyph) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R0.root glyph) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C961c4661cb7c4bf48ed6019959062337.R0.Compose.RustStd
