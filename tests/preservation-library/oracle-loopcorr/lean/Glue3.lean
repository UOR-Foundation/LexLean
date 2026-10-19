
namespace Tre

def loc (m : Nat) : Ident := Ident.generated IdentKind.binding m

theorem same_param_loc (k m : Nat) : sameIdent (paramId k) (loc m) = false := by
  simp [sameIdent, paramId, loc, kindNumber]
theorem same_loc_param (m k : Nat) : sameIdent (loc m) (paramId k) = false := by
  simp [sameIdent, paramId, loc, kindNumber]
theorem same_holder_loc (k m : Nat) : sameIdent (holderId k) (loc m) = false := by
  simp [sameIdent, holderId, loc, kindNumber]

theorem packV_det : ∀ {ys : List V} {a b : V}, PackV ys a → PackV ys b → a = b := by
  intro ys a b h1
  induction h1 generalizing b with
  | one => intro h2; exact (packV_single h2).symm
  | cons h ih =>
    rename_i x vs st
    intro h2
    cases vs with
    | nil => cases h
    | cons y ys' =>
      obtain ⟨st2, rfl, hst2⟩ := packV_cons2 h2
      rw [ih hst2]

theorem carriedV_changed {X ps vs ys} (h : CarriedV X ps vs ys) : ys = changedVals ps vs := by
  induction h with
  | nil => rfl
  | changed _ ih => simp [changedVals, ih]
  | kept _ _ ih => simp [changedVals, ih]
  | dropped _ ih => simp [changedVals, ih]

theorem looksK_carried {X : Env} : ∀ {ps vs ys}, CarriedV X ps vs ys →
    (keptNames ps).Pairwise (fun a b => sameIdent a b = false) →
    ∀ n ∈ keptNames ps, looksK ps vs n = lookup X n := by
  intro ps vs ys h
  induction h with
  | nil => intro _ n hn; simp [keptNames] at hn
  | changed _ ih => intro hd n hn; simp only [keptNames] at hd hn; simpa [looksK] using ih hd n hn
  | dropped _ ih => intro hd n hn; simp only [keptNames] at hd hn; simpa [looksK] using ih hd n hn
  | kept hl hr ih =>
    rename_i ps vs ys v n0
    intro hd n hn
    simp only [keptNames, List.pairwise_cons] at hd hn
    simp only [looksK]
    rcases List.mem_cons.1 hn with rfl | hn'
    · have hnone : looksK ps vs n = none := by
        cases hk : looksK ps vs n with
        | none => rfl
        | some w =>
          obtain ⟨m, hm, hs⟩ := looksK_some ps vs n w hk
          have := hd.1 m hm
          have := sameIdent_eq _ _ hs
          subst this
          simp [sameIdent_refl] at *
      rw [hnone, hl]; simp [sameIdent_refl]
    · rw [ih hd.2 n hn']
      cases hk : lookup X n with
      | some w => rfl
      | none =>
        have := hd.1 n hn'
        simp [this]

theorem looksK_agree {X : Env} {ps : List Pos} {vs1 vs0 : List V} {ys : List V}
    (hC : CarriedV X ps vs1 ys)
    (hd : (keptNames ps).Pairwise (fun a b => sameIdent a b = false))
    (h0 : ∀ n ∈ keptNames ps, lookup X n = looksK ps vs0 n) (i : Ident) :
    looksK ps vs1 i = looksK ps vs0 i := by
  by_cases hk : ∃ n ∈ keptNames ps, sameIdent n i = true
  · obtain ⟨n, hn, hs⟩ := hk
    have := sameIdent_eq _ _ hs
    subst this
    rw [looksK_carried hC hd n hn, h0 n hn]
  · cases h1 : looksK ps vs1 i with
    | some w => exact absurd (looksK_some ps vs1 i w h1) hk
    | none =>
      cases h2 : looksK ps vs0 i with
      | none => rfl
      | some w => exact absurd (looksK_some ps vs0 i w h2) hk

end Tre
