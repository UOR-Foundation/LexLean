
namespace LexLeanPreservation.Rust

theorem letsRaise_halt {c : RCrate} {renv : REnv} {lets : List RLet} {hh : ROut}
    (h : LetsRaise c renv lets hh) : Halt hh := by
  induction h with
  | here _ hh => exact hh
  | later _ _ _ ih => exact ih

theorem funSem_loop {p : LexLeanTarget.TargetSyntax.Program} {c : RCrate} {A : Flags} {n f : Nat}
    (hS : ∀ (Γ : Ctx) (fl : Bool) (j : Jd), Corr p c A Γ fl j → Sem p c A n Γ fl j)
    (h : FunLoop p c A f) : FunSem p c A n f := by
  intro fn args env F hidx hW hbind hfF o hobs hnS
  obtain ⟨fn', rc, rty, lets0, tail0, Γ, ps, S, D, hn, pns, pl, bodyL, hidx', HF, hLoop, hChk, hCtx, hCorr⟩ := h
  have hfn : fn' = fn := Option.some.inj (hidx'.symm.trans hidx)
  rw [hfn] at hCtx hCorr
  have hF : F = LexLeanTarget.RustSemantics.fallibleType rty := by
    unfold fnFallible at hfF
    rw [HF] at hfF
    simp only [Option.map_some, Option.some.injEq] at hfF
    exact hfF.symm
  subst hF
  unfold Tre.loopOf at hLoop
  cases hLP : Tre.loopParts f ps S D hn pns rc (LexLeanTarget.RustSyntax.Block.mk lets0 tail0) with
  | none => rw [hLP] at hLoop; simp at hLoop
  | some r =>
    obtain ⟨pl', os, pe, B1⟩ := r
    rw [hLP] at hLoop
    simp only [Option.map_some, Option.some.injEq, Prod.mk.injEq] at hLoop
    obtain ⟨rfl, rfl⟩ := hLoop
    obtain ⟨tp, tail', hLP', hRT, hPR, hTP, hB1⟩ := Tre.loopParts_inv hLP
    have hC : Tre.loopCheck ps pns os B1 = true := by
      unfold Tre.loopCheckOf at hChk
      rw [hLP] at hChk
      exact hChk
    have W := Tre.wf_of_loopCheck hLP' hC
    have hlt := paramCtx_length [] fn.parameters fn.types (LexLeanTarget.RustSemantics.patternsOf rc) Γ hCtx
    have hl : args.length = ps.length := by
      rw [wtl_length hW, hlt.2, Tre.patternsOf_length, Tre.aligned_rc_len W.aligned]
    have hE0 : EnvRel p c A [] [] [] := by
      intro x t s hx
      simp [ctxLookup] at hx
    obtain ⟨Erec, hErec, hRel⟩ := param_env [] fn.parameters fn.types
      (LexLeanTarget.RustSemantics.patternsOf rc) Γ args [] env [] hCtx hE0 hW hbind
    have hrcs : ∀ q ∈ LexLeanTarget.RustSemantics.patternsOf rc, Tre.simplePat q :=
      Tre.aligned_simple (Tre.aligned_swap W.aligned)
    have hErec' : Erec = Tre.ctxOf (LexLeanTarget.RustSemantics.patternsOf rc) args := by
      have := Tre.bindPatterns_ctx (LexLeanTarget.RustSemantics.patternsOf rc) args [] hrcs
        (by rw [Tre.patternsOf_length, Tre.aligned_rc_len W.aligned]; omega)
      rw [hErec] at this
      simpa using this
    obtain ⟨X1, hloc, hr⟩ := Tre.glue (h := Tre.holderId hn) (Tre.same_holder_loc hn) hB1 hRT hPR hTP W HF args hl
    have hEnv : EnvRel p c A Γ env X1 := by
      refine envRel_locals hRel ?_
      intro m
      rw [hErec']
      exact hloc m
    have hSem := hS Γ _ _ hCorr
    unfold Sem at hSem
    obtain ⟨hWT, hcase⟩ := hSem env X1 hEnv o hobs hnS
    refine ⟨hWT, ?_⟩
    rcases hcase with ⟨new, _, hlets, ro, hreal, hrc⟩ | ⟨ho, hh, hreal, hlr⟩
    · exact ⟨_, Tre.realizesFn_of hreal, hr ro (Tre.realizes_stuck hreal) (Tre.lets_ev hlets hrc)⟩
    · subst ho
      exact ⟨_, Tre.realizesFn_of hreal, hr hh (Tre.realizes_stuck hreal)
        (Tre.lets_raise hlr (letsRaise_halt hlr))⟩

end LexLeanPreservation.Rust
