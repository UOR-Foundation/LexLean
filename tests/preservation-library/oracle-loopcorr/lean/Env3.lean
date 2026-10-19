
namespace Tre

theorem loopParams_aligned : ∀ (ps : List Pos) (rc : List Parameter) (fs : List Ident) (pl : List Parameter)
    (os : List Ident), loopParams ps rc fs = some (pl, os) → Aligned ps rc pl fs os := by
  intro ps
  induction ps with
  | nil =>
    intro rc fs pl os h
    cases rc <;> cases fs <;> simp [loopParams] at h
    obtain ⟨rfl, rfl⟩ := h
    exact Aligned.nil
  | cons p ps ih =>
    intro rc fs pl os h
    cases rc with
    | nil => cases p <;> simp [loopParams] at h
    | cons q rest =>
      obtain ⟨pat, ty⟩ := q
      cases p with
      | changed =>
        cases pat <;> cases fs <;> simp [loopParams] at h
        rename_i b f fs'
        obtain ⟨pl', os', hr, rfl, rfl⟩ := h
        exact Aligned.changed (ih _ _ _ _ hr)
      | kept n =>
        cases pat <;> simp [loopParams] at h
        rename_i b
        obtain ⟨hb, pl', hr, rfl⟩ := h
        have := sameIdent_eq _ _ hb
        subst this
        exact Aligned.kept (ih _ _ _ _ hr)
      | dropped =>
        cases pat <;> simp [loopParams] at h
        obtain ⟨pl', hr, rfl⟩ := h
        exact Aligned.dropped (ih _ _ _ _ hr)

theorem orElse_assoc (a b c : Option V) :
    (a.orElse fun _ => b).orElse (fun _ => c) = a.orElse (fun _ => b.orElse fun _ => c) := by
  cases a <;> cases b <;> cases c <;> rfl

theorem looksK_some : ∀ (ps : List Pos) (vs : List V) (i : Ident) (v : V),
    looksK ps vs i = some v → ∃ n ∈ keptNames ps, sameIdent n i = true := by
  intro ps
  induction ps with
  | nil => intro vs i v h; simp [looksK] at h
  | cons p ps ih =>
    intro vs i v h
    cases vs with
    | nil => cases p <;> simp [looksK] at h
    | cons w vs =>
      cases p with
      | changed =>
        simp only [looksK] at h
        obtain ⟨n, hn, hs⟩ := ih vs i v h
        exact ⟨n, by simpa [keptNames] using hn, hs⟩
      | dropped =>
        simp only [looksK] at h
        obtain ⟨n, hn, hs⟩ := ih vs i v h
        exact ⟨n, by simpa [keptNames] using hn, hs⟩
      | kept m =>
        simp only [looksK] at h
        cases hk : looksK ps vs i with
        | some x =>
          obtain ⟨n, hn, hs⟩ := ih vs i x hk
          exact ⟨n, by simp [keptNames, hn], hs⟩
        | none =>
          rw [hk] at h
          simp only [Option.orElse_none] at h
          by_cases hm : sameIdent m i = true
          · exact ⟨m, by simp [keptNames], hm⟩
          · simp [hm] at h

theorem looks_decomp : ∀ {ps rc lp fs os}, Aligned ps rc lp fs os →
    (∀ o ∈ os, ∀ n ∈ keptNames ps, sameIdent o n = false) → ∀ (vs : List V) (i : Ident),
    looks ps os vs i = (looksC os (changedVals ps vs) i).orElse (fun _ => looksK ps vs i) := by
  intro ps rc lp fs os h
  induction h with
  | nil => intro _ vs i; cases vs <;> simp [looks, looksC, looksK, changedVals]
  | changed hA ih =>
    rename_i ps' rc' lp' fs' os' b f ty
    intro hd vs i
    cases vs with
    | nil => simp [looks, looksC, looksK, changedVals]
    | cons v vs =>
      have hd' : ∀ o ∈ os', ∀ n ∈ keptNames ps', sameIdent o n = false := by
        intro o ho n hn
        exact hd o (by simp [ho]) n (by simpa [keptNames] using hn)
      have hbn : ∀ n ∈ keptNames ps', sameIdent b n = false := by
        intro n hn
        exact hd b (by simp) n (by simpa [keptNames] using hn)
      simp only [looks, changedVals, looksC, looksK, ih hd' vs i]
      -- (C.orElse K).orElse H = (C.orElse H).orElse K
      generalize hC : looksC os' (changedVals ps' vs) i = C
      generalize hK : looksK ps' vs i = K
      by_cases hs : sameIdent b i = true
      · simp only [hs, if_true]
        cases K with
        | some k =>
          exfalso
          obtain ⟨n, hn, hni⟩ := looksK_some ps' vs i k hK
          have := sameIdent_trans b i n hs (sameIdent_symm _ _ hni)
          rw [hbn n hn] at this
          exact absurd this (by simp)
        | none => cases C <;> simp
      · simp [hs]
  | kept hA ih =>
    rename_i ps' rc' lp' fs' os' n ty
    intro hd vs i
    cases vs with
    | nil => simp [looks, looksC, looksK, changedVals]
    | cons v vs =>
      have hd' : ∀ o ∈ os', ∀ n ∈ keptNames ps', sameIdent o n = false := by
        intro o ho m hm
        exact hd o ho m (by simp [keptNames, hm])
      simp only [looks, changedVals, looksK, ih hd' vs i]
      rw [orElse_assoc]
  | dropped hA ih =>
    rename_i ps' rc' lp' fs' os' ty
    intro hd vs i
    cases vs with
    | nil => simp [looks, looksC, looksK, changedVals]
    | cons v vs =>
      have hd' : ∀ o ∈ os', ∀ n ∈ keptNames ps', sameIdent o n = false := by
        intro o ho m hm
        exact hd o ho m (by simpa [keptNames] using hm)
      simp only [looks, changedVals, looksK]
      exact ih hd' vs i

theorem looks_rename : ∀ {ps rc lp fs os}, Aligned ps rc lp fs os →
    ∀ (vs : List V) (i : Ident), (∀ f ∈ fs, sameIdent f i = false) → looks ps fs vs i = looksK ps vs i := by
  intro ps rc lp fs os h
  induction h with
  | nil => intro vs i _; cases vs <;> simp [looks, looksK]
  | changed hA ih =>
    rename_i ps' rc' lp' fs' os' b f ty
    intro vs i hf
    cases vs with
    | nil => simp [looks, looksK]
    | cons v vs =>
      have := hf f (by simp)
      simp only [looks, looksK, this]
      rw [ih vs i (fun g hg => hf g (by simp [hg]))]
      simp
  | kept hA ih =>
    intro vs i hf
    cases vs with
    | nil => simp [looks, looksK]
    | cons v vs => simp only [looks, looksK]; rw [ih vs i hf]
  | dropped hA ih =>
    intro vs i hf
    cases vs with
    | nil => simp [looks, looksK]
    | cons v vs => simp only [looks, looksK]; exact ih vs i hf

end Tre
