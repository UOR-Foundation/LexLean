
namespace Tre

theorem rtB_inv {me ps S D lets tail b'} (h : rtB me ps S D (Block.mk lets tail) = some b') :
    hygLets (keptNames ps) lets = true ∧ ∃ tail', rtE me ps S D tail = some tail' ∧
      b' = Block.mk lets tail' := by
  rw [rtB] at h
  by_cases hy : hygLets (keptNames ps) lets = true
  · rw [if_pos hy] at h
    cases ht : rtE me ps S D tail with
    | none => rw [ht] at h; simp at h
    | some t' =>
      rw [ht] at h
      simp only [Option.some.injEq] at h
      exact ⟨hy, t', rfl, h.symm⟩
  · rw [if_neg hy] at h; simp at h

theorem rtArms_inv {me ps S D p b rest arms'} (h : rtArms me ps S D (Arm.mk p b :: rest) = some arms') :
    hygP (keptNames ps) p = true ∧ ∃ b' rest', rtB me ps S D b = some b' ∧ rtArms me ps S D rest = some rest' ∧
      arms' = Arm.mk p b' :: rest' := by
  rw [rtArms] at h
  by_cases hy : hygP (keptNames ps) p = true
  · rw [if_pos hy] at h
    cases hb : rtB me ps S D b with
    | none => rw [hb] at h; simp at h
    | some b' =>
      cases hr : rtArms me ps S D rest with
      | none => rw [hb, hr] at h; simp at h
      | some r' =>
        rw [hb, hr] at h
        simp only [Option.some.injEq] at h
        exact ⟨hy, b', r', rfl, rfl, h.symm⟩
  · rw [if_neg hy] at h; simp at h

theorem stepLets {c : Crate} {me : Nat} {ps : List Pos} {X0 : Env} {tail tail' : Expr}
    (tailStep : ∀ X : Env, (∀ n ∈ keptNames ps, lookup X n = lookup X0 n) → ∀ r, r ≠ ROutcome.stuck →
      Ev c X tail r → ∃ r', Ev c X tail' r' ∧ Rel c me ps X0 r r') :
    ∀ (lets : List Let), hygLets (keptNames ps) lets = true → ∀ X : Env,
      (∀ n ∈ keptNames ps, lookup X n = lookup X0 n) → ∀ r, r ≠ ROutcome.stuck →
      EvB c X (Block.mk lets tail) r → ∃ r', EvB c X (Block.mk lets tail') r' ∧ Rel c me ps X0 r r' := by
  intro lets
  induction lets with
  | nil =>
    intro _ X hK r hs h
    obtain ⟨r', h1, h2⟩ := tailStep X hK r hs (evBlockNil.1 h)
    exact ⟨r', evBlockNil.2 h1, h2⟩
  | cons l rest ih =>
    intro hy X hK r hs h
    obtain ⟨pat, ty, b⟩ := l
    simp only [hygLets, Bool.and_eq_true] at hy
    rcases invBlockCons hs h with ⟨v, X', hb, hp, hr⟩ | ⟨hh, hb⟩
    · have hK' : ∀ n ∈ keptNames ps, lookup X' n = lookup X0 n := by
        intro n hn
        rw [bindPattern_lookup hy.1 hp n hn]
        exact hK n hn
      obtain ⟨r', h1, h2⟩ := ih hy.2 X' hK' r hs hr
      exact ⟨r', evBlockCons_val hb hp h1, h2⟩
    · exact ⟨r, evBlockCons_halt hb hh, Or.inr (Or.inl ⟨hh, rfl⟩)⟩

end Tre
