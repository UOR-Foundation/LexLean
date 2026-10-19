
namespace Tre

/-- the outcome of a list of expressions is the values of each -/
theorem evL_nil {c X} : EvL c X [] (ROutcomes.values []) :=
  ⟨by simp, 1, by rw [evalList]⟩

theorem evL_cons_values {c X a rest vs} :
    EvL c X (a :: rest) (ROutcomes.values vs) ↔
      ∃ v vs', vs = v :: vs' ∧ Ev c X a (ROutcome.value v) ∧ EvL c X rest (ROutcomes.values vs') := by
  constructor
  · rintro ⟨ne, k, hk⟩
    cases k with
    | zero => rw [evalList] at hk; exact absurd hk.symm ne
    | succ k =>
      rw [evalList] at hk
      generalize ha : eval k c X a = oa at hk
      cases oa with
      | value v =>
        try simp only at hk
        generalize hr : evalList k c X rest = orr at hk
        cases orr with
        | values vs' =>
          try simp only at hk
          injection hk with hk
          subst hk
          exact ⟨v, vs', rfl, ⟨by simp, k, ha⟩, ⟨by simp, k, hr⟩⟩
        | raise => try simp only at hk; cases hk
        | abort => try simp only at hk; cases hk
        | stuck => try simp only at hk; cases hk
        | exhausted => try simp only at hk; cases hk
      | raise => try simp only at hk; cases hk
      | abort => try simp only at hk; cases hk
      | stuck => try simp only at hk; cases hk
      | exhausted => try simp only at hk; cases hk
  · rintro ⟨v, vs', rfl, ⟨_, ka, ha⟩, ⟨_, kr, hr⟩⟩
    have ha' := Ev.at ⟨by simp, ka, ha⟩
    have hr' := EvL.at ⟨by simp, kr, hr⟩
    obtain ⟨ka, ha'⟩ := ha'
    obtain ⟨kr, hr'⟩ := hr'
    refine ⟨by simp, max ka kr + 1, ?_⟩
    rw [evalList, ha' _ (Nat.le_max_left _ _), hr' _ (Nat.le_max_right _ _)]

end Tre
