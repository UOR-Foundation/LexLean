import LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R1
import LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R1.RustCore
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R1.Compose.RustCore

theorem __wt_0 : ∀ (__v : (Production.Kernel.Shape)), LexLeanPreservation.Rust.WT LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R1.RustCore.program LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R1.RustCore.krate LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R1.RustCore.flags (LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R1.__enc_0 __v) (.adt 0)
  | Production.Kernel.Shape.circle __x0 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat __x0) LexLeanPreservation.Rust.wtl_nil)
  | Production.Kernel.Shape.rectangle __x0 __x1 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat __x0) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat __x1) LexLeanPreservation.Rust.wtl_nil))

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (width : Nat) (height : Nat) : ∃ ro, LexLeanPreservation.Rust.RealizesFn true (LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R1.denote width height) ro ∧ LexLeanPreservation.Rust.RCI LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R1.RustCore.krate (LexLeanPreservation.Rust.fnIdent 0) [(LexLeanTarget.TargetSyntax.Value.nat width), (LexLeanTarget.TargetSyntax.Value.nat height)] ro :=
  LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R1.RustCore.root n 0) rfl (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat width) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat height) LexLeanPreservation.Rust.wtl_nil)) rfl rfl (LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R1.root width height) (LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R1.Compose.RustCore
