
namespace Tre

theorem ctor_ne_call_aux (m : Nat) (args : List Expr) (x : Expr) : True := trivial

mutual
theorem stepE (c : Crate) (me : Nat) (ps : List Pos) (S D : RustType) : ∀ (x x' : Expr),
    rtE me ps S D x = some x' → ∀ (X X0 : Env),
    (∀ n ∈ keptNames ps, lookup X n = lookup X0 n) → ∀ r, r ≠ ROutcome.stuck →
    Ev c X x r → ∃ r', Ev c X x' r' ∧ Rel c me ps X0 r r' := by
  intro x
  cases x with
  | cond cd t e =>
    intro x' h X X0 hK r hs hev
    rw [rtE_cond] at h
    cases ht : rtB me ps S D t with
    | none => rw [ht] at h; simp at h
    | some t' =>
      cases he : rtB me ps S D e with
      | none => rw [ht, he] at h; simp at h
      | some e' =>
        rw [ht, he] at h
        simp only [Option.some.injEq] at h
        subst h
        rcases invCond hs hev with ⟨flag, hc, hb⟩ | ⟨hh, hc⟩
        · cases flag
          · obtain ⟨r', h1, h2⟩ := stepB c me ps S D e e' he X X0 hK r hs hb
            exact ⟨r', compCond_val (flag := false) hc h1, h2⟩
          · obtain ⟨r', h1, h2⟩ := stepB c me ps S D t t' ht X X0 hK r hs hb
            exact ⟨r', compCond_val (flag := true) hc h1, h2⟩
        · exact ⟨r, compCond_halt hc hh, Or.inr (Or.inl ⟨hh, rfl⟩)⟩
  | matchOn s arms =>
    intro x' h X X0 hK r hs hev
    rw [rtE_match] at h
    cases ha : rtArms me ps S D arms with
    | none => rw [ha] at h; simp at h
    | some arms' =>
      rw [ha] at h
      simp only [Option.some.injEq] at h
      subst h
      rcases invMatch hs hev with ⟨v, hc, hb⟩ | ⟨hh, hc⟩
      · obtain ⟨r', h1, h2⟩ := stepA c me ps S D arms arms' ha X X0 hK v r hs hb
        exact ⟨r', compMatch_val hc h1, h2⟩
      · exact ⟨r, compMatch_halt hc hh, Or.inr (Or.inl ⟨hh, rfl⟩)⟩
  | block b =>
    intro x' h X X0 hK r hs hev
    rw [rtE_block] at h
    cases hb : rtB me ps S D b with
    | none => rw [hb] at h; simp at h
    | some b' =>
      rw [hb] at h
      simp only [Option.some.injEq] at h
      subst h
      obtain ⟨r', h1, h2⟩ := stepB c me ps S D b b' hb X X0 hK r hs (invBlockE.1 hev)
      exact ⟨r', invBlockE.2 h1, h2⟩
  | call callee args prop =>
    intro x' h X X0 hK r hs hev
    cases callee with
    | function m =>
      cases prop with
      | true =>
        rw [rtE_call_prop] at h
        simp only [Option.some.injEq] at h
        subst h
        exact leaf_step hs hev
      | false =>
        by_cases hm : (m == me) = true
        · have hm' : m = me := by simpa using hm
          subst hm'
          rw [rtE_self] at h
          cases hc : carried ps args with
          | none => rw [hc] at h; simp at h
          | some xs =>
            rw [hc] at h
            dsimp only at h
            cases hp : pack xs with
            | none => rw [hp] at h; simp at h
            | some st' =>
              rw [hp] at h
              simp only [Option.some.injEq] at h
              subst h
              rcases invCall hs hev with ⟨vs, hl, ⟨hne, k, hk⟩⟩ | ⟨rfl, hl⟩ | ⟨rfl, hl⟩
              · rcases carried_ev ps args xs _ hc hl (by simp) with
                  ⟨vs', ys, hvs, hcv, hev'⟩ | ⟨h1, _⟩ | ⟨h1, _⟩
                · injection hvs with hvs
                  subst hvs
                  obtain ⟨st, hst, hevp⟩ := pack_ev xs st' hp ys hev'
                  refine ⟨ROutcome.value (TargetSyntax.Value.ok st), compConstruct_val (w := TargetSyntax.Value.ok st) (evL_single_val hevp) (by simp [constructValue]), ?_⟩
                  exact Or.inr (Or.inr ⟨vs, ys, st, k, carriedV_keep hcv (by
                    intro n hn; exact hK n hn), hst, hk.symm, rfl⟩)
                · cases h1
                · cases h1
              · rcases carried_ev ps args xs _ hc hl (by simp) with
                  ⟨vs', ys, hvs, _, _⟩ | ⟨_, hev'⟩ | ⟨h1, _⟩
                · cases hvs
                · exact ⟨_, compConstruct_raise (evL_single_raise (pack_halt xs st' hp ROutcome.raise _ HP.raise hev')),
                    Or.inr (Or.inl ⟨Or.inl rfl, rfl⟩)⟩
                · cases h1
              · rcases carried_ev ps args xs _ hc hl (by simp) with
                  ⟨vs', ys, hvs, _, _⟩ | ⟨h1, _⟩ | ⟨_, hev'⟩
                · cases hvs
                · cases h1
                · exact ⟨_, compConstruct_abort (evL_single_abort (pack_halt xs st' hp ROutcome.abort _ HP.abort hev')),
                    Or.inr (Or.inl ⟨Or.inr rfl, rfl⟩)⟩
        · have hm' : (m == me) = false := by simpa using hm
          rw [rtE_call_other me ps S D m args hm'] at h
          simp only [Option.some.injEq] at h
          subst h
          exact leaf_step hs hev
    | runtime item =>
      rw [rtE_other me ps S D _ (by intros; intro h; cases h) (by intros; intro h; cases h)
        (by intros; intro h; cases h) (by intros; intro h; cases h)] at h
      simp only [Option.some.injEq] at h
      subst h
      exact leaf_step hs hev
    | machine item =>
      rw [rtE_other me ps S D _ (by intros; intro h; cases h) (by intros; intro h; cases h)
        (by intros; intro h; cases h) (by intros; intro h; cases h)] at h
      simp only [Option.some.injEq] at h
      subst h
      exact leaf_step hs hev
  | _ =>
    intro x' h X X0 hK r hs hev
    rw [rtE_other me ps S D _ (by intros; intro h; cases h) (by intros; intro h; cases h)
      (by intros; intro h; cases h) (by intros; intro h; cases h)] at h
    simp only [Option.some.injEq] at h
    subst h
    exact leaf_step hs hev

theorem stepB (c : Crate) (me : Nat) (ps : List Pos) (S D : RustType) : ∀ (b b' : Block),
    rtB me ps S D b = some b' → ∀ (X X0 : Env),
    (∀ n ∈ keptNames ps, lookup X n = lookup X0 n) → ∀ r, r ≠ ROutcome.stuck →
    EvB c X b r → ∃ r', EvB c X b' r' ∧ Rel c me ps X0 r r' := by
  intro b
  cases b with
  | mk lets tail =>
    intro b' h X X0 hK r hs hev
    obtain ⟨hy, tail', ht, rfl⟩ := rtB_inv h
    exact stepLets (fun X hK r hs hev => stepE c me ps S D tail tail' ht X X0 hK r hs hev)
      lets hy X hK r hs hev

theorem stepA (c : Crate) (me : Nat) (ps : List Pos) (S D : RustType) : ∀ (arms arms' : List Arm),
    rtArms me ps S D arms = some arms' → ∀ (X X0 : Env),
    (∀ n ∈ keptNames ps, lookup X n = lookup X0 n) → ∀ (v : V) r, r ≠ ROutcome.stuck →
    EvA c X v arms r → ∃ r', EvA c X v arms' r' ∧ Rel c me ps X0 r r' := by
  intro arms
  cases arms with
  | nil =>
    intro arms' h X X0 hK v r hs hev
    exact absurd (invArmsNil hev) hs
  | cons a rest =>
    intro arms' h X X0 hK v r hs hev
    obtain ⟨p, b⟩ := a
    obtain ⟨hy, b', rest', hb, hr, rfl⟩ := rtArms_inv h
    rcases invArmsCons hs hev with ⟨X', hp, hbev⟩ | ⟨hp, hrest⟩
    · have hK' : ∀ n ∈ keptNames ps, lookup X' n = lookup X0 n := by
        intro n hn
        rw [bindPattern_lookup hy hp n hn]
        exact hK n hn
      obtain ⟨r', h1, h2⟩ := stepB c me ps S D b b' hb X' X0 hK' r hs hbev
      exact ⟨r', compArmsCons_some hp h1, h2⟩
    · obtain ⟨r', h1, h2⟩ := stepA c me ps S D rest rest' hr X X0 hK v r hs hrest
      exact ⟨r', compArmsCons_none hp h1, h2⟩
end

end Tre
