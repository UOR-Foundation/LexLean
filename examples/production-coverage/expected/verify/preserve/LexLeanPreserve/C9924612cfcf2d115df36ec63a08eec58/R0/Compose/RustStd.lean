import LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0
import LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.Compose.RustStd

mutual
theorem __wt_0 : ∀ (__v : (Coverage.Boundary.Grove)), LexLeanPreservation.Rust.WT LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.RustStd.program LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.RustStd.krate LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.RustStd.flags (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.__enc_0 __v) (.adt 0)
  | Coverage.Boundary.Grove.node __x0 __x1 __x2 __x3 __x4 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList LexLeanPreservation.Rust.wt_nat) __x0) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_list (__wtItems_0 __x1)) (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList (LexLeanPreservation.Rust.wt_encPair LexLeanPreservation.Rust.wt_string LexLeanPreservation.Rust.wt_int)) __x2) (LexLeanPreservation.Rust.wtl_cons (__wtAux_1 __x3) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_list (__wtItems_3 __x4)) LexLeanPreservation.Rust.wtl_nil)))))
  | Coverage.Boundary.Grove.leaf __x0 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList (LexLeanPreservation.Rust.wt_encPair LexLeanPreservation.Rust.wt_nat LexLeanPreservation.Rust.wt_int)) __x0) LexLeanPreservation.Rust.wtl_nil)
  | Coverage.Boundary.Grove.settled __x0 => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons (__wtAux_4 __x0) LexLeanPreservation.Rust.wtl_nil)

theorem __wtItems_0 : ∀ (__v : (List (Coverage.Boundary.Grove))), LexLeanPreservation.Rust.WTAll LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.RustStd.program LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.RustStd.krate LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.RustStd.flags (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.__items_0 __v) (.adt 0)
  | [] => LexLeanPreservation.Rust.wtAll_nil
  | __x0 :: __x1 => LexLeanPreservation.Rust.wtAll_cons (__wt_0 __x0) (__wtItems_0 __x1)

theorem __wtAux_1 : ∀ (__v : (Option (Coverage.Boundary.Grove))), LexLeanPreservation.Rust.WT LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.RustStd.program LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.RustStd.krate LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.RustStd.flags (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.__aux_1 __v) (.option (.adt 0))
  | none => LexLeanPreservation.Rust.wt_noneV
  | some __x0 => LexLeanPreservation.Rust.wt_someV (__wt_0 __x0)

theorem __wtAux_2 : ∀ (__v : (Prod Nat (Coverage.Boundary.Grove))), LexLeanPreservation.Rust.WT LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.RustStd.program LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.RustStd.krate LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.RustStd.flags (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.__aux_2 __v) (.pair .nat (.adt 0))
  | (__x0, __x1) => LexLeanPreservation.Rust.wt_pairV (LexLeanPreservation.Rust.wt_nat __x0) (__wt_0 __x1)

theorem __wtItems_3 : ∀ (__v : (List (Prod Nat (Coverage.Boundary.Grove)))), LexLeanPreservation.Rust.WTAll LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.RustStd.program LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.RustStd.krate LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.RustStd.flags (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.__items_3 __v) (.pair .nat (.adt 0))
  | [] => LexLeanPreservation.Rust.wtAll_nil
  | __x0 :: __x1 => LexLeanPreservation.Rust.wtAll_cons (__wtAux_2 __x0) (__wtItems_3 __x1)

theorem __wtAux_4 : ∀ (__v : (Except Nat (Coverage.Boundary.Grove))), LexLeanPreservation.Rust.WT LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.RustStd.program LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.RustStd.krate LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.RustStd.flags (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.__aux_4 __v) (.result (.adt 0) .nat)
  | Except.error __x0 => LexLeanPreservation.Rust.wt_errorV (LexLeanPreservation.Rust.wt_nat __x0)
  | Except.ok __x0 => LexLeanPreservation.Rust.wt_okV (__wt_0 __x0)

end

theorem __wt_1 : ∀ (__s : (Coverage.Boundary.Ledger)), LexLeanPreservation.Rust.WT LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.RustStd.program LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.RustStd.krate LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.RustStd.flags (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.__enc_1 __s) (.adt 1) :=
  fun __s => LexLeanPreservation.Rust.wt_adt rfl (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList (LexLeanPreservation.Rust.wt_encPair LexLeanPreservation.Rust.wt_string LexLeanPreservation.Rust.wt_int)) (__s).entries) (LexLeanPreservation.Rust.wtl_cons (LexLeanPreservation.Rust.wt_string (__s).owner) LexLeanPreservation.Rust.wtl_nil))

/-- The rendering of the root's entry, invoked on the encoded arguments,
realizes the encoded source result when the arguments satisfy §17.12's
invariants, and refuses them with `none` otherwise. -/
theorem root (g : (Coverage.Boundary.Grove)) (ledger : (Coverage.Boundary.Ledger)) (picks : (Option (List Nat))) (spare : (List (List (Prod Nat Bool)))) : ∃ ro, LexLeanPreservation.Rust.RCI LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.RustStd.krate (LexLeanPreservation.Rust.fnIdent 18) [(LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.__enc_0 g), (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.__enc_1 ledger), ((LexLeanPreservation.encOption (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat)) picks), ((LexLeanPreservation.encList (LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.bool))) spare)] ro ∧
    (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.accepts g ledger picks spare → LexLeanPreservation.Rust.RealizesFn false (LexLeanPreservation.someObs (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.denote g ledger picks spare)) ro) ∧
    (¬ LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.accepts g ledger picks spare → LexLeanPreservation.Rust.RealizesFn false (LexLeanPreservation.Obs.value LexLeanTarget.TargetSyntax.Value.none) ro) :=
  match LexLeanPreservation.Rust.compose (fun n => LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.RustStd.root n 18) rfl (LexLeanPreservation.Rust.wtl_cons (__wt_0 g) (LexLeanPreservation.Rust.wtl_cons (__wt_1 ledger) (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encOption (LexLeanPreservation.Rust.wt_encList LexLeanPreservation.Rust.wt_nat)) picks) (LexLeanPreservation.Rust.wtl_cons ((LexLeanPreservation.Rust.wt_encList (LexLeanPreservation.Rust.wt_encList (LexLeanPreservation.Rust.wt_encPair LexLeanPreservation.Rust.wt_nat LexLeanPreservation.Rust.wt_bool))) spare) LexLeanPreservation.Rust.wtl_nil)))) rfl rfl (LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.entry g ledger picks spare) (LexLeanPreservation.Rust.rel_ne_stuck _ _) with
  | ⟨ro, hr, hc⟩ => ⟨ro, hc, fun h => LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.entry_accepts g ledger picks spare h ▸ hr, fun h => LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.entry_refuses g ledger picks spare h ▸ hr⟩

end LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R0.Compose.RustStd
