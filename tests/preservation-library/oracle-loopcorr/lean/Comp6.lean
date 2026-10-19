
namespace Tre

def consL (v : V) : ROutcomes → ROutcomes
  | .values vs => .values (v :: vs)
  | .raise => .raise
  | .abort => .abort
  | .stuck => .stuck
  | .exhausted => .exhausted

theorem consL_ne {v o} (h : consL v o ≠ ROutcomes.exhausted) : o ≠ ROutcomes.exhausted := by
  intro e; subst e; exact h rfl

theorem invEvL_cons {c X a as o} (h : EvL c X (a :: as) o) :
    (∃ v o', Ev c X a (ROutcome.value v) ∧ EvL c X as o' ∧ o = consL v o') ∨
    (o = ROutcomes.raise ∧ Ev c X a ROutcome.raise) ∨
    (o = ROutcomes.abort ∧ Ev c X a ROutcome.abort) ∨
    (o = ROutcomes.stuck ∧ Ev c X a ROutcome.stuck) := by
  obtain ⟨ne, k, hk⟩ := h
  cases k with
  | zero => rw [evalList] at hk; exact absurd hk.symm ne
  | succ k =>
    rw [evalList] at hk
    generalize ha : eval k c X a = oa at hk
    cases oa with
    | value v =>
      try simp only at hk
      generalize hr : evalList k c X as = orr at hk
      subst hk
      have ne' := consL_ne (v := v) (o := orr) ne
      exact Or.inl ⟨v, orr, ⟨by simp, k, ha⟩, ⟨ne', k, hr⟩, by cases orr <;> rfl⟩
    | raise => try simp only at hk; subst hk; exact Or.inr (Or.inl ⟨rfl, by simp, k, ha⟩)
    | abort => try simp only at hk; subst hk; exact Or.inr (Or.inr (Or.inl ⟨rfl, by simp, k, ha⟩))
    | stuck => try simp only at hk; subst hk; exact Or.inr (Or.inr (Or.inr ⟨rfl, by simp, k, ha⟩))
    | exhausted => try simp only at hk; exact absurd hk.symm ne

theorem compEvL_cons_val {c X a as o'} {v : V} (ha : Ev c X a (ROutcome.value v))
    (hs : EvL c X as o') : EvL c X (a :: as) (consL v o') := by
  obtain ⟨ka, ha'⟩ := Ev.at ha
  obtain ⟨ks, hs'⟩ := EvL.at hs
  refine ⟨?_, max ka ks + 1, ?_⟩
  · have := hs.1; cases o' <;> simp [consL] <;> exact absurd rfl this
  · rw [evalList, ha' _ (Nat.le_max_left _ _), hs' _ (Nat.le_max_right _ _)]
    cases o' <;> rfl

theorem compEvL_cons_halt {c X a as} {o : ROutcome} (ha : Ev c X a o) (ho : Halt o) :
    EvL c X (a :: as) (match o with | .raise => ROutcomes.raise | _ => ROutcomes.abort) := by
  obtain ⟨ka, ha'⟩ := Ev.at ha
  rcases ho with rfl | rfl
  · refine ⟨by simp, ka + 1, ?_⟩
    rw [evalList, ha' _ (Nat.le_refl _)]
  · refine ⟨by simp, ka + 1, ?_⟩
    rw [evalList, ha' _ (Nat.le_refl _)]

end Tre
