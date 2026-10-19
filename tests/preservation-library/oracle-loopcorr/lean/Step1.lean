
namespace Tre

def fid (me : Nat) : Ident := Ident.generated IdentKind.function me

/-- What the retargeted tail does, given what the recursive tail does. -/
def Rel (c : Crate) (me : Nat) (ps : List Pos) (X0 : Env) (r r' : ROutcome) : Prop :=
  (∃ d, r = ROutcome.value d ∧ r' = ROutcome.value (TargetSyntax.Value.error d)) ∨
  (Halt r ∧ r' = r) ∨
  (∃ vs ys st k, CarriedV X0 ps vs ys ∧ PackV ys st ∧ r = invoke k c (fid me) vs ∧
      r' = ROutcome.value (TargetSyntax.Value.ok st))

theorem leaf_step {c X X0 x r me ps} {S D : RustType} (hs : r ≠ ROutcome.stuck)
    (h : Ev c X x r) :
    ∃ r', Ev c X (Expr.construct (Ctor.err S D) [x]) r' ∧ Rel c me ps X0 r r' := by
  cases r with
  | value d =>
    exact ⟨_, compConstruct_val (evL_single_val h) (by simp [constructValue]), Or.inl ⟨d, rfl, rfl⟩⟩
  | raise => exact ⟨_, compConstruct_raise (evL_single_raise h), Or.inr (Or.inl ⟨Or.inl rfl, rfl⟩)⟩
  | abort => exact ⟨_, compConstruct_abort (evL_single_abort h), Or.inr (Or.inl ⟨Or.inr rfl, rfl⟩)⟩
  | stuck => exact absurd rfl hs
  | exhausted => exact absurd rfl h.1

theorem carriedV_keep {X X0 : Env} : ∀ {ps vs ys}, CarriedV X ps vs ys →
    (∀ n ∈ keptNames ps, lookup X n = lookup X0 n) → CarriedV X0 ps vs ys := by
  intro ps vs ys h
  induction h with
  | nil => intro _; exact CarriedV.nil
  | changed _ ih => intro hk; exact CarriedV.changed (ih (by simpa [keptNames] using hk))
  | kept hl _ ih =>
    intro hk
    refine CarriedV.kept ?_ (ih (by intro n hn; exact hk n (by simp [keptNames, hn])))
    rw [← hk _ (by simp [keptNames])]; exact hl
  | dropped _ ih => intro hk; exact CarriedV.dropped (ih (by simpa [keptNames] using hk))

end Tre
