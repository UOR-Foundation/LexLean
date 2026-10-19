
namespace Tre

theorem sameIdent_symm (a b : Ident) (h : sameIdent a b = true) : sameIdent b a = true := by
  cases a <;> cases b <;> simp [sameIdent, LexLeanRuntime.equal] at h ⊢
  all_goals first | omega | (try subst_vars; simp_all)

theorem sameIdent_trans (a b c : Ident) (h1 : sameIdent a b = true) (h2 : sameIdent b c = true) :
    sameIdent a c = true := by
  cases a <;> cases b <;> cases c <;> simp [sameIdent, LexLeanRuntime.equal] at h1 h2 ⊢
  all_goals first | omega | (try subst_vars; simp_all)

theorem lookup_same (X : Env) (i n : Ident) (h : sameIdent i n = true) : lookup X i = lookup X n := by
  induction X with
  | nil => rfl
  | cons e rest ih =>
    obtain ⟨a, v⟩ := e
    have h1 : sameIdent a i = sameIdent a n := by
      by_cases h3 : sameIdent a i = true
      · rw [h3, sameIdent_trans a i n h3 h]
      · by_cases h4 : sameIdent a n = true
        · exact absurd (sameIdent_trans a n i h4 (sameIdent_symm _ _ h)) h3
        · simp only [Bool.not_eq_true] at h3 h4
          rw [h3, h4]
    simp only [lookup, h1, ih]

theorem read_aux {c X} {j i : Ident} {v : V} (e : Expr) (k : Nat)
    (he : e = Expr.move j ∨ e = Expr.clone j ∨ e = Expr.copy j ∨ e = Expr.deref j)
    (hj : j = i) (hk : eval (k + 1) c X e = ROutcome.value v) : lookup X i = some v := by
  subst hj
  rcases he with rfl | rfl | rfl | rfl <;> rw [eval] at hk <;>
  · generalize hl : lookup X j = ol at hk
    cases ol with
    | none => simp at hk
    | some w => simp at hk; subst hk; rfl

theorem read_ev {c X a i} {v : V} (hr : readName a = some i)
    (h : Ev c X a (ROutcome.value v)) : lookup X i = some v := by
  obtain ⟨_, k, hk⟩ := h
  cases k with
  | zero => rw [eval] at hk; cases hk
  | succ k =>
    cases a with
    | move j => exact read_aux _ k (Or.inl rfl) (Option.some.inj hr) hk
    | clone j => exact read_aux _ k (Or.inr (Or.inl rfl)) (Option.some.inj hr) hk
    | copy j => exact read_aux _ k (Or.inr (Or.inr (Or.inl rfl))) (Option.some.inj hr) hk
    | deref j => exact read_aux _ k (Or.inr (Or.inr (Or.inr rfl))) (Option.some.inj hr) hk
    | _ => simp [readName] at hr

end Tre
