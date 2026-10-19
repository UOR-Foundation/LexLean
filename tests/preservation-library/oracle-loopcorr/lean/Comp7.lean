
namespace Tre

/-- The values a self call hands the state, given the values of its arguments. -/
inductive CarriedV (X : Env) : List Pos → List V → List V → Prop
  | nil : CarriedV X [] [] []
  | changed {ps vs ys v} : CarriedV X ps vs ys → CarriedV X (Pos.changed :: ps) (v :: vs) (v :: ys)
  | kept {ps vs ys v n} : lookup X n = some v → CarriedV X ps vs ys →
      CarriedV X (Pos.kept n :: ps) (v :: vs) ys
  | dropped {ps vs ys v} : CarriedV X ps vs ys → CarriedV X (Pos.dropped :: ps) (v :: vs) ys

inductive PackV : List V → V → Prop
  | one {a} : PackV [a] a
  | cons {a vs st} : PackV vs st → PackV (a :: vs) (TargetSyntax.Value.pair a st)

theorem carried_ev {c X} : ∀ (ps : List Pos) (args xs : List Expr) (o : ROutcomes),
    carried ps args = some xs → EvL c X args o → o ≠ ROutcomes.stuck →
    (∃ vs ys, o = ROutcomes.values vs ∧ CarriedV X ps vs ys ∧ EvL c X xs (ROutcomes.values ys)) ∨
    (o = ROutcomes.raise ∧ EvL c X xs ROutcomes.raise) ∨
    (o = ROutcomes.abort ∧ EvL c X xs ROutcomes.abort) := by
  intro ps
  induction ps with
  | nil =>
    intro args xs o hc h hs
    cases args with
    | nil =>
      simp only [carried, Option.some.injEq] at hc
      subst hc
      obtain ⟨ne, k, hk⟩ := h
      cases k with
      | zero => rw [evalList] at hk; exact absurd hk.symm ne
      | succ k =>
        rw [evalList] at hk
        subst hk
        exact Or.inl ⟨[], [], rfl, CarriedV.nil, evL_nil⟩
    | cons a as => simp [carried] at hc
  | cons p ps ih =>
    intro args xs o hc h hs
    cases args with
    | nil => cases p <;> simp [carried] at hc
    | cons a as =>
      rcases invEvL_cons h with ⟨v, o', ha, has, rfl⟩ | ⟨rfl, ha⟩ | ⟨rfl, ha⟩ | ⟨rfl, _⟩
      · cases p with
        | changed =>
          simp only [carried] at hc
          cases hc' : carried ps as with
          | none => rw [hc'] at hc; simp at hc
          | some xs' =>
            rw [hc'] at hc
            simp only [Option.map_some, Option.some.injEq] at hc
            subst hc
            have hs' : o' ≠ ROutcomes.stuck := by
              intro e; subst e; exact hs rfl
            rcases ih as xs' o' hc' has hs' with ⟨vs, ys, rfl, hcv, hev⟩ | ⟨rfl, hev⟩ | ⟨rfl, hev⟩
            · exact Or.inl ⟨v :: vs, v :: ys, rfl, CarriedV.changed hcv, compEvL_cons_val ha hev⟩
            · exact Or.inr (Or.inl ⟨rfl, by
                simpa [consL] using compEvL_cons_val (c := c) (X := X) (a := a) (as := xs') ha hev⟩)
            · exact Or.inr (Or.inr ⟨rfl, by
                simpa [consL] using compEvL_cons_val (c := c) (X := X) (a := a) (as := xs') ha hev⟩)
        | kept n =>
          simp only [carried] at hc
          by_cases hp : passes a n = true
          · rw [if_pos hp] at hc
            have hs' : o' ≠ ROutcomes.stuck := by
              intro e; subst e; exact hs rfl
            have hrn : readName a ≠ none := by
              intro e; simp [passes, e] at hp
            obtain ⟨i, hi⟩ := Option.ne_none_iff_exists'.1 hrn
            have hlook : lookup X n = some v := by
              rw [← lookup_same X i n (by simpa [passes, hi] using hp)]
              exact read_ev hi ha
            rcases ih as xs o' hc has hs' with ⟨vs, ys, rfl, hcv, hev⟩ | ⟨rfl, hev⟩ | ⟨rfl, hev⟩
            · exact Or.inl ⟨v :: vs, ys, rfl, CarriedV.kept hlook hcv, hev⟩
            · exact Or.inr (Or.inl ⟨rfl, hev⟩)
            · exact Or.inr (Or.inr ⟨rfl, hev⟩)
          · rw [if_neg hp] at hc; simp at hc
        | dropped =>
          simp only [carried] at hc
          by_cases hp : isPure a = true
          · rw [if_pos hp] at hc
            have hs' : o' ≠ ROutcomes.stuck := by
              intro e; subst e; exact hs rfl
            rcases ih as xs o' hc has hs' with ⟨vs, ys, rfl, hcv, hev⟩ | ⟨rfl, hev⟩ | ⟨rfl, hev⟩
            · exact Or.inl ⟨v :: vs, ys, rfl, CarriedV.dropped hcv, hev⟩
            · exact Or.inr (Or.inl ⟨rfl, hev⟩)
            · exact Or.inr (Or.inr ⟨rfl, hev⟩)
          · rw [if_neg hp] at hc; simp at hc
      · -- the first argument raises
        cases p with
        | changed =>
          simp only [carried] at hc
          cases hc' : carried ps as with
          | none => rw [hc'] at hc; simp at hc
          | some xs' =>
            rw [hc'] at hc
            simp only [Option.map_some, Option.some.injEq] at hc
            subst hc
            exact Or.inr (Or.inl ⟨rfl, compEvL_cons_halt ha (Or.inl rfl)⟩)
        | kept n =>
          simp only [carried] at hc
          by_cases hp : passes a n = true
          · have hrn : readName a ≠ none := by
              intro e; simp [passes, e] at hp
            obtain ⟨i, hi⟩ := Option.ne_none_iff_exists'.1 hrn
            exfalso
            cases a <;> simp [readName] at hi <;> exact pure_no_halt (a := _) (by simp [isPure]) (Or.inl rfl) ha
          · rw [if_neg hp] at hc; simp at hc
        | dropped =>
          simp only [carried] at hc
          by_cases hp : isPure a = true
          · exact absurd ha (fun h => pure_no_halt hp (Or.inl rfl) h)
          · rw [if_neg hp] at hc; simp at hc
      · cases p with
        | changed =>
          simp only [carried] at hc
          cases hc' : carried ps as with
          | none => rw [hc'] at hc; simp at hc
          | some xs' =>
            rw [hc'] at hc
            simp only [Option.map_some, Option.some.injEq] at hc
            subst hc
            exact Or.inr (Or.inr ⟨rfl, compEvL_cons_halt ha (Or.inr rfl)⟩)
        | kept n =>
          simp only [carried] at hc
          by_cases hp : passes a n = true
          · have hrn : readName a ≠ none := by
              intro e; simp [passes, e] at hp
            obtain ⟨i, hi⟩ := Option.ne_none_iff_exists'.1 hrn
            exfalso
            cases a <;> simp [readName] at hi <;> exact pure_no_halt (a := _) (by simp [isPure]) (Or.inr rfl) ha
          · rw [if_neg hp] at hc; simp at hc
        | dropped =>
          simp only [carried] at hc
          by_cases hp : isPure a = true
          · exact absurd ha (fun h => pure_no_halt hp (Or.inr rfl) h)
          · rw [if_neg hp] at hc; simp at hc
      · exact absurd rfl hs

end Tre
