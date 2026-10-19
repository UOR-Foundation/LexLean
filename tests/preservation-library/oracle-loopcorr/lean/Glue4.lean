
namespace Tre

structure LoopWF (ps : List Pos) (rc pl : List Parameter) (fresh os : List Ident) (B1 : Block) : Prop where
  aligned : Aligned ps rc pl fresh os
  freshDist : fresh.Pairwise (fun a b => sameIdent a b = false)
  disj : ∀ o ∈ os, ∀ n ∈ keptNames ps, sameIdent o n = false
  keptDist : (keptNames ps).Pairwise (fun a b => sameIdent a b = false)
  keptLoc : ∀ n ∈ keptNames ps, ∃ m, n = loc m
  freshKind : ∀ f ∈ fresh, ∃ k, f = paramId k
  free : ∀ f ∈ fresh, ∀ i ∈ occB B1, sameIdent f i = false

theorem fresh_not_kept {ps rc pl fresh os B1} (W : LoopWF ps rc pl fresh os B1) :
    ∀ f ∈ fresh, ∀ n ∈ keptNames ps, sameIdent f n = false := by
  intro f hf n hn
  obtain ⟨k, rfl⟩ := W.freshKind f hf
  obtain ⟨m, rfl⟩ := W.keptLoc n hn
  exact same_param_loc k m

theorem looks_fresh_K {ps rc pl fresh os B1} (W : LoopWF ps rc pl fresh os B1) (vs : List V) (i : Ident)
    (hi : ∀ f ∈ fresh, sameIdent f i = false) : looks ps fresh vs i = looksK ps vs i :=
  looks_rename W.aligned vs i hi

/-- The environment a loop form's function starts with, and the facts the iteration needs of it. -/
theorem entry_pack {c : Crate} {ps rc pl fresh os B1} {pe : Expr} (W : LoopWF ps rc pl fresh os B1)
    (hPR : packReads fresh = some pe) (vs : List V) (hl : vs.length = ps.length) :
    ∃ pk, PackV (changedVals ps vs) pk ∧ Ev c (ctxOf (patternsOf pl) vs) pe (ROutcome.value pk) := by
  obtain ⟨hy, hf, hp⟩ := aligned_len W.aligned vs hl
  have hdisj : ∀ o ∈ fresh, ∀ n ∈ keptNames ps, sameIdent o n = false := fresh_not_kept W
  have hdec := looks_decomp (aligned_swap W.aligned) (by
    intro o ho n hn; exact hdisj o ho n hn) vs
  -- lookups of the fresh names in the entry environment
  have hall : All2 (fun f y => lookup (ctxOf (patternsOf pl) vs) f = some y) fresh (changedVals ps vs) := by
    have hs := looksC_self fresh (changedVals ps vs) W.freshDist (by omega)
    refine All2.imp' hs ?_
    intro f y hf hfy
    rw [ctx_looks W.aligned vs hl, hdec f, hfy]
    have hK : looksK ps vs f = none := by
      cases hk : looksK ps vs f with
      | none => rfl
      | some w =>
        obtain ⟨n, hn, hs'⟩ := looksK_some ps vs f w hk
        have h1 := hdisj f hf n hn
        have h2 := sameIdent_symm _ _ hs'
        rw [h1] at h2
        exact absurd h2 (by simp)
    rw [hK]; simp
  exact packReads_ev fresh _ pe hPR hall

end Tre
