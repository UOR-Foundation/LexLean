import LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R30
import LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R30.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R30.Compose.RustStd

mutual
theorem __wt_0 : ∀ (__v : (Coverage.Syntax.Rose Nat)), LexLeanPreservation.Rust.WT LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R30.RustStd.program LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R30.RustStd.krate LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R30.RustStd.flags (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R30.__enc_0 __v) (.adt 0)
  | Coverage.Syntax.Rose.node __x0 __x1 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat __x0) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_list (__wtItems_0 __x1)) LexLeanPreservation.Rust.wtl_nil))

theorem __wtItems_0 : ∀ (__v : (List (Coverage.Syntax.Rose Nat))), LexLeanPreservation.Rust.WTAll LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R30.RustStd.program LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R30.RustStd.krate LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R30.RustStd.flags (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R30.__items_0 __v) (.adt 0)
  | [] => LexLeanPreservation.Rust.wtAll_nil
  | __x0 :: __x1 => LexLeanPreservation.Rust.wtAll_cons (__wt_0 __x0) (__wtItems_0 __x1)

end

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (n : Nat) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R30.denote n) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R30.RustStd.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.nat n)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R30.RustStd.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat n) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R30.root n) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R30.Compose.RustStd
