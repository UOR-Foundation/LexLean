
namespace Tre
mutual
theorem bindPattern_ext : ∀ (pat : Pat) (v : V) (X X' : Env), bindPattern pat v X = some X' →
    ∃ ext : Env, X' = ext ++ X ∧ ∀ e ∈ ext, e.1 ∈ patBinders pat := by
  intro pat
  cases pat with
  | wild => intro v X X' h; simp [bindPattern] at h; subst h; exact ⟨[], rfl, by simp⟩
  | bind i => intro v X X' h; simp [bindPattern] at h; subst h; exact ⟨[(i, v)], rfl, by simp [patBinders]⟩
  | unit => intro v X X' h; cases v <;> simp [bindPattern] at h; subst h; exact ⟨[], rfl, by simp⟩
  | none => intro v X X' h; cases v <;> simp [bindPattern] at h; subst h; exact ⟨[], rfl, by simp⟩
  | ordering o =>
    intro v X X' h
    cases v <;> simp [bindPattern] at h
    obtain ⟨_, h⟩ := h
    subst h; exact ⟨[], rfl, by simp⟩
  | tuple ps =>
    intro v X X' h
    cases v <;> simp [bindPattern] at h
    obtain ⟨ext, he, hb⟩ := bindPatterns_ext ps _ X X' h
    exact ⟨ext, he, by simpa [patBinders] using hb⟩
  | some p =>
    intro v X X' h
    cases v <;> simp [bindPattern] at h
    obtain ⟨ext, he, hb⟩ := bindPattern_ext p _ X X' h
    exact ⟨ext, he, by simpa [patBinders] using hb⟩
  | ok p =>
    intro v X X' h
    cases v <;> simp [bindPattern] at h
    obtain ⟨ext, he, hb⟩ := bindPattern_ext p _ X X' h
    exact ⟨ext, he, by simpa [patBinders] using hb⟩
  | err p =>
    intro v X X' h
    cases v <;> simp [bindPattern] at h
    obtain ⟨ext, he, hb⟩ := bindPattern_ext p _ X X' h
    exact ⟨ext, he, by simpa [patBinders] using hb⟩
  | adt a n ps =>
    intro v X X' h
    cases v <;> simp [bindPattern] at h
    obtain ⟨_, h⟩ := h
    obtain ⟨ext, he, hb⟩ := bindPatterns_ext ps _ X X' h
    exact ⟨ext, he, by simpa [patBinders] using hb⟩
theorem bindPatterns_ext : ∀ (pats : List Pat) (vs : List V) (X X' : Env), bindPatterns pats vs X = some X' →
    ∃ ext : Env, X' = ext ++ X ∧ ∀ e ∈ ext, e.1 ∈ patBindersList pats := by
  intro pats
  cases pats with
  | nil =>
    intro vs X X' h
    cases vs <;> simp [bindPatterns] at h
    subst h; exact ⟨[], rfl, by simp⟩
  | cons p ps =>
    intro vs X X' h
    cases vs with
    | nil => simp [bindPatterns] at h
    | cons v vs =>
      simp only [bindPatterns] at h
      cases hb : bindPattern p v X with
      | none => rw [hb] at h; simp at h
      | some X1 =>
        rw [hb] at h
        simp only at h
        obtain ⟨e1, he1, hb1⟩ := bindPattern_ext p v X X1 hb
        obtain ⟨e2, he2, hb2⟩ := bindPatterns_ext ps vs X1 X' h
        refine ⟨e2 ++ e1, by rw [he2, he1]; simp, ?_⟩
        intro e he
        simp only [List.mem_append] at he
        simp only [patBindersList, List.mem_append]
        rcases he with he | he
        · exact Or.inr (hb2 e he)
        · exact Or.inl (hb1 e he)
end

theorem lookup_append_avoid (ext X : Env) (n : Ident)
    (h : ∀ e ∈ ext, sameIdent e.1 n = false) : lookup (ext ++ X) n = lookup X n := by
  induction ext with
  | nil => rfl
  | cons e rest ih =>
    obtain ⟨a, v⟩ := e
    have h1 : sameIdent a n = false := h (a, v) (by simp)
    simp only [List.cons_append, lookup, h1]
    simp only [Bool.false_eq_true, if_false]
    exact ih (fun e he => h e (by simp [he]))

theorem bindPattern_lookup {pat v X X'} {K : List Ident} (hy : hygP K pat = true)
    (h : bindPattern pat v X = some X') : ∀ n ∈ K, lookup X' n = lookup X n := by
  obtain ⟨ext, rfl, hb⟩ := bindPattern_ext pat v X X' h
  intro n hn
  apply lookup_append_avoid
  intro e he
  have := hb e he
  have h2 := (List.all_eq_true.1 hy) e.1 this
  have h3 := (List.all_eq_true.1 h2) n hn
  simpa using h3

end Tre
