
namespace Tre

macro "sub " h:term : term => `(Agree.mono $h (fun i hi => by
  simp [occE, occL, occB, occLets, occArms, hi]))

theorem relevance (c : Crate) : ∀ n,
    (∀ x E E', Agree (occE x) E E' → eval n c E x = eval n c E' x) ∧
    (∀ xs E E', Agree (occL xs) E E' → evalList n c E xs = evalList n c E' xs) ∧
    (∀ b E E', Agree (occB b) E E' → evalBlock n c E b = evalBlock n c E' b) ∧
    (∀ v arms E E', Agree (occArms arms) E E' → evalArms n c E v arms = evalArms n c E' v arms) := by
  intro n
  induction n with
  | zero =>
    refine ⟨?_, ?_, ?_, ?_⟩ <;> intros <;> simp [eval, evalList, evalBlock, evalArms]
  | succ n ih =>
    obtain ⟨ihE, ihL, ihB, ihA⟩ := ih
    refine ⟨?_, ?_, ?_, ?_⟩
    · intro x E E' h
      cases x with
      | lit l => rw [eval, eval]
      | move i => rw [eval, eval, h i (by simp [occE])]
      | clone i => rw [eval, eval, h i (by simp [occE])]
      | copy i => rw [eval, eval, h i (by simp [occE])]
      | deref i => rw [eval, eval, h i (by simp [occE])]
      | unbox i => rw [eval, eval, h i (by simp [occE])]
      | not e => rw [eval, eval, ihE e E E' (sub h)]
      | box e => rw [eval, eval, ihE e E E' (sub h)]
      | widen e => rw [eval, eval, ihE e E E' (sub h)]
      | succeed e => rw [eval, eval, ihE e E E' (sub h)]
      | call callee args prop => rw [eval, eval, ihL args E E' (sub h)]
      | apply hd args prop =>
        rw [eval, eval, h hd (by simp [occE]), ihL args E E' (sub h)]
      | construct ctor args => rw [eval, eval, ihL args E E' (sub h)]
      | pair a b => rw [eval, eval, ihE a E E' (sub h), ihE b E E' (sub h)]
      | cond cd t e =>
        rw [eval, eval, ihE cd E E' (sub h), ihB t E E' (sub h), ihB e E E' (sub h)]
      | matchOn s arms =>
        rw [eval, eval, ihE s E E' (sub h)]
        generalize eval n c E' s = os
        cases os <;> simp only
        rename_i v
        exact ihA v arms E E' (sub h)
      | block b => rw [eval, eval, ihB b E E' (sub h)]
      | uncons i => rw [eval, eval, h i (by simp [occE])]
      | isZero i => rw [eval, eval, h i (by simp [occE])]
      | nonZero i => rw [eval, eval, h i (by simp [occE])]
      | predecessor i => rw [eval, eval, h i (by simp [occE])]
      | loop s b =>
        rw [eval, eval, ihB b E E' (sub h)]
        generalize evalBlock n c E' b = ob
        cases ob with
        | value st =>
          cases st <;> simp only
          rename_i next
          exact ihE _ ((s, next) :: E) ((s, next) :: E') (by
            intro i hi
            simp only [lookup]
            by_cases hs : sameIdent s i = true
            · simp [hs]
            · simp only [hs]
              exact h i (by simp [occE] at hi ⊢; first | exact hi | skip))
        | _ => simp only
    · intro xs E E' h
      cases xs with
      | nil => rw [evalList, evalList]
      | cons a as =>
        rw [evalList, evalList, ihE a E E' (sub h), ihL as E E' (sub h)]
    · intro b E E' h
      cases b with
      | mk lets tail =>
        cases lets with
        | nil => rw [evalBlock, evalBlock, ihE tail E E' (sub h)]
        | cons l rest =>
          obtain ⟨pat, ty, bd⟩ := l
          rw [evalBlock, evalBlock, ihE bd E E' (sub h)]
          generalize eval n c E' bd = ob
          cases ob with
          | value v =>
            simp only
            rw [bindPattern_map pat v E, bindPattern_map pat v E']
            cases hb : bindPattern pat v [] with
            | none => simp
            | some ext =>
              simp only [Option.map_some]
              exact ihB (Block.mk rest tail) (ext ++ E) (ext ++ E') (Agree.ext ext (Agree.mono h (by
                intro i hi
                simp only [occB, occLets, List.mem_append] at hi ⊢
                rcases hi with hi | hi
                · exact Or.inl (Or.inr hi)
                · exact Or.inr hi)))
          | _ => simp only
    · intro v arms E E' h
      cases arms with
      | nil => rw [evalArms, evalArms]
      | cons a rest =>
        obtain ⟨pat, bd⟩ := a
        rw [evalArms, evalArms]
        rw [bindPattern_map pat v E, bindPattern_map pat v E']
        cases hb : bindPattern pat v [] with
        | none => simp only [Option.map_none]; exact ihA v rest E E' (sub h)
        | some ext =>
          simp only [Option.map_some]
          exact ihB bd (ext ++ E) (ext ++ E') (Agree.ext ext (sub h))

end Tre
