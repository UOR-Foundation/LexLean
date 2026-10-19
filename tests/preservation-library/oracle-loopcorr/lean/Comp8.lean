
namespace Tre

theorem evL_nil_inv {c X o} (h : EvL c X [] o) : o = ROutcomes.values [] := by
  obtain ⟨ne, k, hk⟩ := h
  cases k with
  | zero => rw [evalList] at hk; exact absurd hk.symm ne
  | succ k => rw [evalList] at hk; exact hk.symm

theorem pack_ev {c X} : ∀ (xs : List Expr) (e : Expr), pack xs = some e → ∀ ys,
    EvL c X xs (ROutcomes.values ys) → ∃ st, PackV ys st ∧ Ev c X e (ROutcome.value st) := by
  intro xs
  induction xs with
  | nil => intro e he; simp [pack] at he
  | cons a rest ih =>
    intro e he ys h
    cases rest with
    | nil =>
      simp only [pack, Option.some.injEq] at he
      subst he
      obtain ⟨v, vs', rfl, ha, hr⟩ := evL_cons_values.1 h
      have := evL_nil_inv hr
      injection this with this
      subst this
      exact ⟨v, PackV.one, ha⟩
    | cons b rest' =>
      simp only [pack] at he
      cases hp : pack (b :: rest') with
      | none => rw [hp] at he; simp at he
      | some e' =>
        rw [hp] at he
        simp only [Option.map_some, Option.some.injEq] at he
        subst he
        obtain ⟨v, vs', rfl, ha, hr⟩ := evL_cons_values.1 h
        obtain ⟨st, hst, hev⟩ := ih e' hp vs' hr
        exact ⟨TargetSyntax.Value.pair v st, PackV.cons hst, compPair_val ha hev⟩

inductive HP : ROutcome → ROutcomes → Prop
  | raise : HP ROutcome.raise ROutcomes.raise
  | abort : HP ROutcome.abort ROutcomes.abort

theorem pack_halt {c X} : ∀ (xs : List Expr) (e : Expr), pack xs = some e → ∀ (o : ROutcome) (ob : ROutcomes),
    HP o ob → EvL c X xs ob → Ev c X e o := by
  intro xs
  induction xs with
  | nil => intro e he; simp [pack] at he
  | cons a rest ih =>
    intro e he o ob ho h
    cases rest with
    | nil =>
      simp only [pack, Option.some.injEq] at he
      subst he
      rcases invEvL_cons h with ⟨v, o', ha, hr, hh⟩ | ⟨hh, ha⟩ | ⟨hh, ha⟩ | ⟨hh, _⟩
      · have := evL_nil_inv hr
        subst this
        cases ho <;> simp [consL] at hh
      · cases ho
        · exact ha
        · simp at hh
      · cases ho
        · simp at hh
        · exact ha
      · cases ho <;> simp at hh
    | cons b rest' =>
      simp only [pack] at he
      cases hp : pack (b :: rest') with
      | none => rw [hp] at he; simp at he
      | some e' =>
        rw [hp] at he
        simp only [Option.map_some, Option.some.injEq] at he
        subst he
        rcases invEvL_cons h with ⟨v, o', ha, hr, hh⟩ | ⟨hh, ha⟩ | ⟨hh, ha⟩ | ⟨hh, _⟩
        · cases ho with
          | raise =>
            have : o' = ROutcomes.raise := by cases o' <;> simp [consL] at hh <;> rfl
            subst this
            exact compPair_haltR ha (ih e' hp _ _ HP.raise hr) (Or.inl rfl)
          | abort =>
            have : o' = ROutcomes.abort := by cases o' <;> simp [consL] at hh <;> rfl
            subst this
            exact compPair_haltR ha (ih e' hp _ _ HP.abort hr) (Or.inr rfl)
        · cases ho
          · exact compPair_haltL ha (Or.inl rfl)
          · simp at hh
        · cases ho
          · simp at hh
          · exact compPair_haltL ha (Or.inr rfl)
        · cases ho <;> simp at hh

end Tre
