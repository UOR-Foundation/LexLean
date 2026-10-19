
namespace Tre

theorem compPair_val {c X l r} {a b : V} (hl : Ev c X l (ROutcome.value a))
    (hr : Ev c X r (ROutcome.value b)) :
    Ev c X (Expr.pair l r) (ROutcome.value (TargetSyntax.Value.pair a b)) := by
  obtain ⟨kl, hl'⟩ := Ev.at hl
  obtain ⟨kr, hr'⟩ := Ev.at hr
  refine ⟨by simp, max kl kr + 1, ?_⟩
  rw [eval, hl' _ (Nat.le_max_left _ _)]
  try simp only
  rw [hr' _ (Nat.le_max_right _ _)]

theorem compPair_haltL {c X l r o} (hl : Ev c X l o) (hh : Halt o) :
    Ev c X (Expr.pair l r) o := by
  obtain ⟨kl, hl'⟩ := Ev.at hl
  refine ⟨hl.1, kl + 1, ?_⟩
  rw [eval, hl' _ (Nat.le_refl _)]
  rcases hh with rfl | rfl <;> rfl

theorem compPair_haltR {c X l r o} {a : V} (hl : Ev c X l (ROutcome.value a))
    (hr : Ev c X r o) (hh : Halt o) : Ev c X (Expr.pair l r) o := by
  obtain ⟨kl, hl'⟩ := Ev.at hl
  obtain ⟨kr, hr'⟩ := Ev.at hr
  refine ⟨hr.1, max kl kr + 1, ?_⟩
  rw [eval, hl' _ (Nat.le_max_left _ _)]
  try simp only
  rw [hr' _ (Nat.le_max_right _ _)]
  rcases hh with rfl | rfl <;> rfl

theorem evL_single_val {c X e} {v : V} (h : Ev c X e (ROutcome.value v)) :
    EvL c X [e] (ROutcomes.values [v]) :=
  evL_cons_values.2 ⟨v, [], rfl, h, evL_nil⟩

theorem evL_single_raise {c X e} (h : Ev c X e ROutcome.raise) :
    EvL c X [e] ROutcomes.raise := by
  obtain ⟨kc, hc'⟩ := Ev.at h
  refine ⟨by simp, kc + 1, ?_⟩
  rw [evalList, hc' _ (Nat.le_refl _)]

theorem evL_single_abort {c X e} (h : Ev c X e ROutcome.abort) :
    EvL c X [e] ROutcomes.abort := by
  obtain ⟨kc, hc'⟩ := Ev.at h
  refine ⟨by simp, kc + 1, ?_⟩
  rw [evalList, hc' _ (Nat.le_refl _)]

theorem compConstruct_val {c X ctor args vs} {w : V} (h : EvL c X args (ROutcomes.values vs))
    (hw : constructValue ctor vs = some w) :
    Ev c X (Expr.construct ctor args) (ROutcome.value w) := by
  obtain ⟨k, hk'⟩ := EvL.at h
  refine ⟨by simp, k + 1, ?_⟩
  rw [eval, hk' _ (Nat.le_refl _)]
  try simp only
  rw [hw]

theorem compConstruct_raise {c X ctor args} (h : EvL c X args ROutcomes.raise) :
    Ev c X (Expr.construct ctor args) ROutcome.raise := by
  obtain ⟨k, hk'⟩ := EvL.at h
  refine ⟨by simp, k + 1, ?_⟩
  rw [eval, hk' _ (Nat.le_refl _)]

theorem compConstruct_abort {c X ctor args} (h : EvL c X args ROutcomes.abort) :
    Ev c X (Expr.construct ctor args) ROutcome.abort := by
  obtain ⟨k, hk'⟩ := EvL.at h
  refine ⟨by simp, k + 1, ?_⟩
  rw [eval, hk' _ (Nat.le_refl _)]

/-- Inversion of a call of a crate function whose result is not propagated. -/
theorem invCall {c X m args r} (hs : r ≠ ROutcome.stuck)
    (h : Ev c X (Expr.call (Callee.function m) args false) r) :
    (∃ vs, EvL c X args (ROutcomes.values vs) ∧
        EvI c (Ident.generated IdentKind.function m) vs r) ∨
    (r = ROutcome.raise ∧ EvL c X args ROutcomes.raise) ∨
    (r = ROutcome.abort ∧ EvL c X args ROutcomes.abort) := by
  obtain ⟨ne, k, hk⟩ := h
  cases k with
  | zero => rw [eval] at hk; exact absurd hk.symm ne
  | succ k =>
    rw [eval] at hk
    generalize hl : evalList k c X args = ol at hk
    cases ol with
    | values vs =>
      try simp only at hk
      exact Or.inl ⟨vs, ⟨by simp, k, hl⟩, ne, k, hk⟩
    | raise => try simp only at hk; subst hk; exact Or.inr (Or.inl ⟨rfl, by simp, k, hl⟩)
    | abort => try simp only at hk; subst hk; exact Or.inr (Or.inr ⟨rfl, by simp, k, hl⟩)
    | stuck => try simp only at hk; exact absurd hk.symm hs
    | exhausted => try simp only at hk; exact absurd hk.symm ne

end Tre
