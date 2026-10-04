import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7
import LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.Compose.RustStd

/-- The rendering of the root's entry, invoked on the encoded arguments,
realizes the encoded source result when the arguments satisfy §17.12's
invariants, and refuses them with `none` otherwise. -/
theorem root (g : (List (Prod String (List String)))) (hrep : LexLeanPreservation.Rust.RepresentableL [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.string (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.string))) g)]) : ∃ ro, LexLeanPreservation.Rust.RCI LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.RustStd.krate (LexLeanPreservation.Rust.fnIdent 15) [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.string (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.string))) g)] ro ∧
    (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.accepts g → LexLeanPreservation.Rust.RealizesFn true (LexLeanPreservation.someObs (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.denote g)) ro) ∧
    (¬ LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.accepts g → LexLeanPreservation.Rust.RealizesFn true (LexLeanPreservation.Obs.value LexLeanTarget.TargetSyntax.Value.none) ro) :=
  match LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.RustStd.root n 15) rfl (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList (LexLeanPreservation.Rust.wt_encPair LexLeanPreservation.Rust.wt_string (LexLeanPreservation.Rust.wt_encList LexLeanPreservation.Rust.wt_string))) g) LexLeanPreservation.Rust.wtl_nil) rfl rfl (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.entry g) (LexLeanPreservation.Rust.rel_ne_stuck _ _) with
  | ⟨ro, hr, hc⟩ => ⟨ro, hc, fun h => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.entry_accepts g h ▸ hr, fun h => LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.entry_refuses g h ▸ hr⟩

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.Compose.RustStd
