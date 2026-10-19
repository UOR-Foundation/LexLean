
namespace Tre

mutual
theorem bindPattern_map : ∀ (pat : Pat) (v : V) (X : Env),
    bindPattern pat v X = Option.map (fun ext => ext ++ X) (bindPattern pat v []) := by
  intro pat
  cases pat with
  | wild => intro v X; simp [bindPattern]
  | bind i => intro v X; simp [bindPattern]
  | unit => intro v X; cases v <;> simp [bindPattern]
  | none => intro v X; cases v <;> simp [bindPattern]
  | ordering o => intro v X; cases v <;> simp [bindPattern]
  | tuple ps =>
    intro v X
    cases v <;> simp [bindPattern]
    exact bindPatterns_map ps _ X
  | some p =>
    intro v X
    cases v <;> simp [bindPattern]
    exact bindPattern_map p _ X
  | ok p =>
    intro v X
    cases v <;> simp [bindPattern]
    exact bindPattern_map p _ X
  | err p =>
    intro v X
    cases v <;> simp [bindPattern]
    exact bindPattern_map p _ X
  | adt a n ps =>
    intro v X
    cases v <;> simp [bindPattern]
    rename_i tag fields
    by_cases ht : tag = n
    · simp only [ht, if_true]
      exact bindPatterns_map ps fields X
    · simp [ht]
theorem bindPatterns_map : ∀ (pats : List Pat) (vs : List V) (X : Env),
    bindPatterns pats vs X = Option.map (fun ext => ext ++ X) (bindPatterns pats vs []) := by
  intro pats
  cases pats with
  | nil => intro vs X; cases vs <;> simp [bindPatterns]
  | cons p ps =>
    intro vs X
    cases vs with
    | nil => simp [bindPatterns]
    | cons v vs =>
      simp only [bindPatterns]
      rw [bindPattern_map p v X, bindPattern_map p v []]
      cases hb : bindPattern p v [] with
      | none => simp
      | some e1 =>
        simp only [Option.map_some, List.append_nil]
        rw [bindPatterns_map ps vs (e1 ++ X), bindPatterns_map ps vs e1]
        cases bindPatterns ps vs [] <;> simp
end

end Tre
