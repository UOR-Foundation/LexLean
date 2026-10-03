import LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R6
import LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R6.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R6.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (g : (List (Prod String (List String)))) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R6.denote g) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R6.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.string (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.string))) g)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R6.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList (LexLeanPreservation.Rust.wt_encPair LexLeanPreservation.Rust.wt_string (LexLeanPreservation.Rust.wt_encList LexLeanPreservation.Rust.wt_string))) g) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R6.root g) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R6.Compose.RustStd
