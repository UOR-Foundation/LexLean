
namespace Tre

theorem pure_no_halt {c X a o} (hp : isPure a = true) (ho : Halt o) (h : Ev c X a o) : False := by
  obtain ⟨_, k, hk⟩ := h
  cases k with
  | zero => rw [eval] at hk; subst hk; rcases ho with h | h <;> cases h
  | succ k =>
    cases a <;> simp only [isPure, Bool.false_eq_true] at hp <;> rw [eval] at hk <;>
    · split at hk <;> (subst hk; rcases ho with h | h <;> cases h)

end Tre
