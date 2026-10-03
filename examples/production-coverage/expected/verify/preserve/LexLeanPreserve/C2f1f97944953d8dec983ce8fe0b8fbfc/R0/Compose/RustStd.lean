import LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R0
import LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R0.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R0.Compose.RustStd

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (m : (List (Prod String Nat))) (k : String) (v : Nat) : ∃ ro, LexLeanPreservation.Rust.RealizesFn false (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R0.denote m k v) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R0.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.string LexLeanTarget.TargetSyntax.Value.nat)) m), (LexLeanTarget.TargetSyntax.Value.string k), (LexLeanTarget.TargetSyntax.Value.nat v)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R0.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList (LexLeanPreservation.Rust.wt_encPair LexLeanPreservation.Rust.wt_string LexLeanPreservation.Rust.wt_nat)) m) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_string k) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat v) LexLeanPreservation.Rust.wtl_nil))) rfl rfl (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R0.root m k v) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R0.Compose.RustStd
