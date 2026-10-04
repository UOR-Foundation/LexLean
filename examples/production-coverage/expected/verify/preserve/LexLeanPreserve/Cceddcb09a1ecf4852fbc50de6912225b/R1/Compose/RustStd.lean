import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R1
import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R1.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R1.Compose.RustStd

/-- The rendering of the root's entry, invoked on the encoded arguments,
realizes the encoded source result when the arguments satisfy §17.12's
invariants, and refuses them with `none` otherwise. -/
theorem root (m : (List (Prod String Nat))) (k : String) (v : Nat) (hrep : LexLeanPreservation.Rust.RepresentableL [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.string LexLeanTarget.TargetSyntax.Value.nat)) m), (LexLeanTarget.TargetSyntax.Value.string k), (LexLeanTarget.TargetSyntax.Value.nat v)]) : ∃ ro, LexLeanPreservation.Rust.RCI LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R1.RustStd.krate (LexLeanPreservation.Rust.fnIdent 10) [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.string LexLeanTarget.TargetSyntax.Value.nat)) m), (LexLeanTarget.TargetSyntax.Value.string k), (LexLeanTarget.TargetSyntax.Value.nat v)] ro ∧
    (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R1.accepts m k v → LexLeanPreservation.Rust.RealizesFn false (LexLeanPreservation.someObs (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R1.denote m k v)) ro) ∧
    (¬ LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R1.accepts m k v → LexLeanPreservation.Rust.RealizesFn false (LexLeanPreservation.Obs.value LexLeanTarget.TargetSyntax.Value.none) ro) :=
  match LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R1.RustStd.root n 10) rfl (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList (LexLeanPreservation.Rust.wt_encPair LexLeanPreservation.Rust.wt_string LexLeanPreservation.Rust.wt_nat)) m) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_string k) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_nat v) LexLeanPreservation.Rust.wtl_nil))) rfl rfl (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R1.entry m k v) (LexLeanPreservation.Rust.rel_ne_stuck _ _) with
  | ⟨ro, hr, hc⟩ => ⟨ro, hc, fun h => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R1.entry_accepts m k v h ▸ hr, fun h => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R1.entry_refuses m k v h ▸ hr⟩

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R1.Compose.RustStd
