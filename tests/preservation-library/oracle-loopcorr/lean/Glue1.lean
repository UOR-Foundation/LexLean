
namespace Tre

theorem Ev_det {c X e o1 o2} (h1 : Ev c X e o1) (h2 : Ev c X e o2) : o1 = o2 := by
  obtain ⟨k1, a1⟩ := Ev.at h1
  obtain ⟨k2, a2⟩ := Ev.at h2
  rw [← a1 (max k1 k2) (Nat.le_max_left _ _), ← a2 (max k1 k2) (Nat.le_max_right _ _)]

theorem EvB_det {c X e o1 o2} (h1 : EvB c X e o1) (h2 : EvB c X e o2) : o1 = o2 := by
  obtain ⟨k1, a1⟩ := EvB.at h1
  obtain ⟨k2, a2⟩ := EvB.at h2
  rw [← a1 (max k1 k2) (Nat.le_max_left _ _), ← a2 (max k1 k2) (Nat.le_max_right _ _)]

theorem compLoop_err {c Y s body} {d : V}
    (h : EvB c Y body (ROutcome.value (TargetSyntax.Value.error d))) :
    Ev c Y (Expr.loop s body) (ROutcome.value d) := by
  obtain ⟨k, hk⟩ := EvB.at h
  refine ⟨by simp, k + 1, ?_⟩
  rw [eval, hk _ (Nat.le_refl _)]

theorem compLoop_halt {c Y s body o} (h : EvB c Y body o) (hh : Halt o) :
    Ev c Y (Expr.loop s body) o := by
  obtain ⟨k, hk⟩ := EvB.at h
  refine ⟨h.1, k + 1, ?_⟩
  rw [eval, hk _ (Nat.le_refl _)]
  rcases hh with rfl | rfl <;> rfl

theorem compLoop_ok {c Y s body o} {st : V}
    (h : EvB c Y body (ROutcome.value (TargetSyntax.Value.ok st)))
    (hr : Ev c ((s, st) :: Y) (Expr.loop s body) o) : Ev c Y (Expr.loop s body) o := by
  obtain ⟨k, hk⟩ := EvB.at h
  obtain ⟨k2, hk2⟩ := Ev.at hr
  refine ⟨hr.1, max k k2 + 1, ?_⟩
  rw [eval, hk _ (Nat.le_max_left _ _)]
  try simp only
  exact hk2 _ (Nat.le_max_right _ _)

theorem invLoop {c Y s body o} (h : Ev c Y (Expr.loop s body) o) (hs : o ≠ ROutcome.stuck) :
    (∃ d, EvB c Y body (ROutcome.value (TargetSyntax.Value.error d)) ∧ o = ROutcome.value d) ∨
    (Halt o ∧ EvB c Y body o) ∨
    (∃ st, EvB c Y body (ROutcome.value (TargetSyntax.Value.ok st)) ∧ Ev c ((s, st) :: Y) (Expr.loop s body) o) := by
  obtain ⟨ne, k, hk⟩ := h
  cases k with
  | zero => rw [eval] at hk; exact absurd hk.symm ne
  | succ k =>
    rw [eval] at hk
    generalize hb : evalBlock k c Y body = ob at hk
    cases ob with
    | value st =>
      cases st with
      | ok next =>
        try simp only at hk
        exact Or.inr (Or.inr ⟨next, ⟨by simp, k, hb⟩, ne, k, hk⟩)
      | error d =>
        try simp only at hk
        subst hk
        exact Or.inl ⟨d, ⟨by simp, k, hb⟩, rfl⟩
      | _ => try simp only at hk; exact absurd hk.symm hs
    | raise => try simp only at hk; subst hk; exact Or.inr (Or.inl ⟨Or.inl rfl, by simp, k, hb⟩)
    | abort => try simp only at hk; subst hk; exact Or.inr (Or.inl ⟨Or.inr rfl, by simp, k, hb⟩)
    | stuck => try simp only at hk; exact absurd hk.symm hs
    | exhausted => try simp only at hk; exact absurd hk.symm ne

/-- The single entry binding of a loop form's body. -/
theorem entry_block {c E h pe tail pk o} (hp : Ev c E pe (ROutcome.value pk)) (hs : o ≠ ROutcome.stuck) :
    EvB c E (Block.mk [Let.mk (Pat.bind h) none pe] tail) o ↔ Ev c ((h, pk) :: E) tail o := by
  constructor
  · intro hev
    rcases invBlockCons hs hev with ⟨v, X', hv, hb, hr⟩ | ⟨hh, hv⟩
    · have := Ev_det hv hp
      injection this with this
      subst this
      simp only [bindPattern, Option.some.injEq] at hb
      subst hb
      exact evBlockNil.1 hr
    · have := Ev_det hv hp
      rcases hh with rfl | rfl <;> cases this
  · intro hev
    exact evBlockCons_val hp (by simp [bindPattern]) (evBlockNil.2 hev)

end Tre
