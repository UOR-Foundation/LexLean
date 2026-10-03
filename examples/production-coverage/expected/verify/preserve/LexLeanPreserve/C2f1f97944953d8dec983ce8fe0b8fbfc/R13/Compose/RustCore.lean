import LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R13
import LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R13.RustCore
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R13.Compose.RustCore

theorem __wt_0 : ∀ (__s : (Coverage.Types.Point)), LexLeanPreservation.Rust.WT LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R13.RustCore.program LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R13.RustCore.krate LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R13.RustCore.flags (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R13.__enc_0 __s) (.adt 0) :=
  fun __s => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat (__s).x) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat (__s).y) LexLeanPreservation.Rust.wtl_nil))

theorem __wt_1 : ∀ (__s : (Coverage.Types.Box Nat)), LexLeanPreservation.Rust.WT LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R13.RustCore.program LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R13.RustCore.krate LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R13.RustCore.flags (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R13.__enc_1 __s) (.adt 1) :=
  fun __s => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat (__s).content) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat (__s).count) LexLeanPreservation.Rust.wtl_nil))

theorem __wt_2 : ∀ (__s : (Coverage.Types.Weights)), LexLeanPreservation.Rust.WT LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R13.RustCore.program LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R13.RustCore.krate LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R13.RustCore.flags (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R13.__enc_2 __s) (.adt 2) :=
  fun __s => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat (__s).base) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat (__s).scale) LexLeanPreservation.Rust.wtl_nil))

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (a : Nat) (b : Nat) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R13.denote a b) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R13.RustCore.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.nat a), (LexLeanTarget.TargetSyntax.Value.nat b)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R13.RustCore.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat a) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat b) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R13.root a b) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R13.Compose.RustCore
