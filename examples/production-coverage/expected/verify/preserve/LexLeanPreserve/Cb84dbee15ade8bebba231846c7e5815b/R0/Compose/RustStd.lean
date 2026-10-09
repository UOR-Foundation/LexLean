import LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0
import LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.Compose.RustStd

mutual
theorem __wt_0 : ∀ (__v : (Coverage.Boundary.Grove)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.RustStd.program _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.RustStd.krate _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.RustStd.flags (_root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.__enc_0 __v) (.adt 0)
  | Coverage.Boundary.Grove.node __x0 __x1 __x2 __x3 __x4 => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encList _root_.LexLeanPreservation.Rust.wt_nat) __x0) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_list (__wtItems_0 __x1)) (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encList (_root_.LexLeanPreservation.Rust.wt_encPair _root_.LexLeanPreservation.Rust.wt_string _root_.LexLeanPreservation.Rust.wt_int)) __x2) (_root_.LexLeanPreservation.Rust.wtl_cons (__wtAux_1 __x3) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_list (__wtItems_3 __x4)) _root_.LexLeanPreservation.Rust.wtl_nil)))))
  | Coverage.Boundary.Grove.leaf __x0 => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encList (_root_.LexLeanPreservation.Rust.wt_encPair _root_.LexLeanPreservation.Rust.wt_nat _root_.LexLeanPreservation.Rust.wt_int)) __x0) _root_.LexLeanPreservation.Rust.wtl_nil)
  | Coverage.Boundary.Grove.settled __x0 => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (__wtAux_4 __x0) _root_.LexLeanPreservation.Rust.wtl_nil)

theorem __wtItems_0 : ∀ (__v : (List (Coverage.Boundary.Grove))), _root_.LexLeanPreservation.Rust.WTAll _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.RustStd.program _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.RustStd.krate _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.RustStd.flags (_root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.__items_0 __v) (.adt 0)
  | [] => _root_.LexLeanPreservation.Rust.wtAll_nil
  | __x0 :: __x1 => _root_.LexLeanPreservation.Rust.wtAll_cons (__wt_0 __x0) (__wtItems_0 __x1)

theorem __wtAux_1 : ∀ (__v : (Option (Coverage.Boundary.Grove))), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.RustStd.program _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.RustStd.krate _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.RustStd.flags (_root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.__aux_1 __v) (.option (.adt 0))
  | none => _root_.LexLeanPreservation.Rust.wt_noneV
  | some __x0 => _root_.LexLeanPreservation.Rust.wt_someV (__wt_0 __x0)

theorem __wtAux_2 : ∀ (__v : (Prod Nat (Coverage.Boundary.Grove))), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.RustStd.program _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.RustStd.krate _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.RustStd.flags (_root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.__aux_2 __v) (.pair .nat (.adt 0))
  | (__x0, __x1) => _root_.LexLeanPreservation.Rust.wt_pairV (_root_.LexLeanPreservation.Rust.wt_nat __x0) (__wt_0 __x1)

theorem __wtItems_3 : ∀ (__v : (List (Prod Nat (Coverage.Boundary.Grove)))), _root_.LexLeanPreservation.Rust.WTAll _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.RustStd.program _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.RustStd.krate _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.RustStd.flags (_root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.__items_3 __v) (.pair .nat (.adt 0))
  | [] => _root_.LexLeanPreservation.Rust.wtAll_nil
  | __x0 :: __x1 => _root_.LexLeanPreservation.Rust.wtAll_cons (__wtAux_2 __x0) (__wtItems_3 __x1)

theorem __wtAux_4 : ∀ (__v : (Except Nat (Coverage.Boundary.Grove))), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.RustStd.program _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.RustStd.krate _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.RustStd.flags (_root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.__aux_4 __v) (.result (.adt 0) .nat)
  | Except.error __x0 => _root_.LexLeanPreservation.Rust.wt_errorV (_root_.LexLeanPreservation.Rust.wt_nat __x0)
  | Except.ok __x0 => _root_.LexLeanPreservation.Rust.wt_okV (__wt_0 __x0)

end

theorem __wt_1 : ∀ (__s : (Coverage.Boundary.Ledger)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.RustStd.program _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.RustStd.krate _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.RustStd.flags (_root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.__enc_1 __s) (.adt 1) :=
  fun __s => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encList (_root_.LexLeanPreservation.Rust.wt_encPair _root_.LexLeanPreservation.Rust.wt_string _root_.LexLeanPreservation.Rust.wt_int)) (__s).entries) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_string (__s).owner) _root_.LexLeanPreservation.Rust.wtl_nil))

/-- The rendering of the root's entry, invoked on the encoded arguments,
realizes the encoded source result when the arguments satisfy §17.12's
invariants, and refuses them with `none` otherwise. -/
theorem root (g : (Coverage.Boundary.Grove)) (ledger : (Coverage.Boundary.Ledger)) (picks : (Option (List Nat))) (spare : (List (List (Prod Nat Bool)))) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.__enc_0 g), (_root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.__enc_1 ledger), ((_root_.LexLeanPreservation.encOption (_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.nat)) picks), ((_root_.LexLeanPreservation.encList (_root_.LexLeanPreservation.encList (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.nat _root_.LexLeanTarget.TargetSyntax.Value.bool))) spare)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 18) [(_root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.__enc_0 g), (_root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.__enc_1 ledger), ((_root_.LexLeanPreservation.encOption (_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.nat)) picks), ((_root_.LexLeanPreservation.encList (_root_.LexLeanPreservation.encList (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.nat _root_.LexLeanTarget.TargetSyntax.Value.bool))) spare)] __e_ro ∧
    (_root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.accepts g ledger picks spare → _root_.LexLeanPreservation.Rust.RealizesFn Bool.false (_root_.LexLeanPreservation.someObs (_root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.denote g ledger picks spare)) __e_ro) ∧
    (¬ _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.accepts g ledger picks spare → _root_.LexLeanPreservation.Rust.RealizesFn Bool.false (_root_.LexLeanPreservation.Obs.value _root_.LexLeanTarget.TargetSyntax.Value.none) __e_ro) :=
  match _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.RustStd.root __e_n 18) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (__wt_0 g) (_root_.LexLeanPreservation.Rust.wtl_cons (__wt_1 ledger) (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encOption (_root_.LexLeanPreservation.Rust.wt_encList _root_.LexLeanPreservation.Rust.wt_nat)) picks) (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encList (_root_.LexLeanPreservation.Rust.wt_encList (_root_.LexLeanPreservation.Rust.wt_encPair _root_.LexLeanPreservation.Rust.wt_nat _root_.LexLeanPreservation.Rust.wt_bool))) spare) _root_.LexLeanPreservation.Rust.wtl_nil)))) rfl rfl (_root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.entry g ledger picks spare) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _) with
  | ⟨__e_ro, __e_hr, __e_hc⟩ => ⟨__e_ro, __e_hc, fun __e_h => _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.entry_accepts g ledger picks spare __e_h ▸ __e_hr, fun __e_h => _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.entry_refuses g ledger picks spare __e_h ▸ __e_hr⟩

end LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R0.Compose.RustStd
