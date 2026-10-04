import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R14
import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R14.RustCore
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R14.Compose.RustCore

theorem __wt_0 : ∀ (__s : (Coverage.Types.Point)), LexLeanPreservation.Rust.WT LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R14.RustCore.program LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R14.RustCore.krate LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R14.RustCore.flags (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R14.__enc_0 __s) (.adt 0) :=
  fun __s => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat (__s).x) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat (__s).y) LexLeanPreservation.Rust.wtl_nil))

theorem __wt_1 : ∀ (__s : (Coverage.Types.Box Nat)), LexLeanPreservation.Rust.WT LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R14.RustCore.program LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R14.RustCore.krate LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R14.RustCore.flags (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R14.__enc_1 __s) (.adt 1) :=
  fun __s => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat (__s).content) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat (__s).count) LexLeanPreservation.Rust.wtl_nil))

theorem __wt_2 : ∀ (__s : (Coverage.Types.Weights)), LexLeanPreservation.Rust.WT LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R14.RustCore.program LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R14.RustCore.krate LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R14.RustCore.flags (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R14.__enc_2 __s) (.adt 2) :=
  fun __s => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat (__s).base) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat (__s).scale) LexLeanPreservation.Rust.wtl_nil))

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (a : Nat) (b : Nat) (hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanTarget.TargetSyntax.Value.nat a), (LexLeanTarget.TargetSyntax.Value.nat b)]) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R14.denote a b) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R14.RustCore.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.nat a), (LexLeanTarget.TargetSyntax.Value.nat b)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R14.RustCore.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat a) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat b) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R14.root a b) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R14.Compose.RustCore
