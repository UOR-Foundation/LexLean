
namespace Tre

theorem carriedV_len {X ps vs ys} (h : CarriedV X ps vs ys) : vs.length = ps.length := by
  induction h with
  | nil => rfl
  | changed _ ih => simp [ih]
  | kept _ _ ih => simp [ih]
  | dropped _ ih => simp [ih]

theorem Ev_agree {c : Crate} {E E' : Env} {x : Expr} {o : ROutcome} (h : Ev c E x o)
    (hA : Agree (occE x) E E') : Ev c E' x o := by
  obtain ⟨ne, k, hk⟩ := h
  refine ⟨ne, k, ?_⟩
  rw [← (relevance c k).1 x E E' hA]
  exact hk

theorem glue {c : Crate} {me : Nat} {ps : List Pos} {S D : RustType} {h : Ident}
    {rc pl : List Parameter} {fresh os : List Ident} {lets0 : List Let} {tail0 tail' : Expr}
    {pe : Expr} {tp : Pat} {rty : RustType} {B1 : Block}
    (hhk : ∀ m, sameIdent h (loc m) = false)
    (hB1 : B1 = Block.mk (Let.mk tp none (Expr.move h) :: lets0) tail')
    (hRT : rtB me ps S D (Block.mk lets0 tail0) = some (Block.mk lets0 tail'))
    (hPR : packReads fresh = some pe) (hTP : tuplePat os = some tp)
    (W : LoopWF ps rc pl fresh os B1)
    (HF : findFunction c.items (Ident.generated IdentKind.function me) = some (pl, rty,
        Block.mk [Let.mk (Pat.bind h) none pe] (Expr.loop h B1)))
    (vs0 : List V) (hl : vs0.length = ps.length) :
    ∃ X1 : Env, (∀ m, lookup X1 (loc m) = lookup (ctxOf (patternsOf rc) vs0) (loc m)) ∧
      (∀ r, r ≠ ROutcome.stuck → EvB c X1 (Block.mk lets0 tail0) r →
        EvI c (Ident.generated IdentKind.function me) vs0 (finished r (fallibleType rty))) := by
  obtain ⟨hy, hf, hp⟩ := aligned_len W.aligned vs0 hl
  have hbind : ∀ vs : List V, vs.length = ps.length →
      bindPatterns (patternsOf pl) vs [] = some (ctxOf (patternsOf pl) vs) := by
    intro vs hvs
    have := bindPatterns_ctx (patternsOf pl) vs [] (aligned_simple W.aligned) (by rw [patternsOf_length]; omega)
    simpa using this
  obtain ⟨pk0, hPV0, hpe0⟩ := entry_pack (c := c) W hPR vs0 hl
  have hX1 := bind_tuple os (changedVals ps vs0) tp pk0 hTP hPV0 hy.symm ((h, pk0) :: ctxOf (patternsOf pl) vs0)
  refine ⟨extC os (changedVals ps vs0) ++ ((h, pk0) :: ctxOf (patternsOf pl) vs0), ?_, ?_⟩
  · intro m
    have hdec := looks_decomp W.aligned (by
      intro o ho n hn'; exact W.disj o ho n hn') vs0
    have hnf : ∀ f ∈ fresh, sameIdent f (loc m) = false := by
      intro f hf'
      obtain ⟨k, rfl⟩ := W.freshKind f hf'
      exact same_param_loc k m
    have e1 : lookup ((h, pk0) :: ctxOf (patternsOf pl) vs0) (loc m) = looksK ps vs0 (loc m) := by
      simp only [lookup, hhk m]
      simp only [Bool.false_eq_true, if_false]
      rw [ctx_looks W.aligned vs0 hl (loc m), looks_fresh_K W vs0 (loc m) hnf]
    rw [ctx_looks (aligned_swap W.aligned) vs0 hl (loc m), hdec (loc m), lookup_append, lookup_extC]
    rw [e1]
  · intro r hs hev
    generalize hX : extC os (changedVals ps vs0) ++ ((h, pk0) :: ctxOf (patternsOf pl) vs0) = X1 at hX1 hev
    have hEh : Ev c ((h, pk0) :: ctxOf (patternsOf pl) vs0) (Expr.move h) (ROutcome.value pk0) :=
      ⟨by simp, 2, by rw [eval]; simp [lookup, sameIdent_refl]⟩
    have hKEPT0 : ∀ n ∈ keptNames ps, lookup X1 n = looksK ps vs0 n := by
      intro n hn
      obtain ⟨m, rfl⟩ := W.keptLoc n hn
      rw [← hX, lookup_append, lookup_extC]
      have hn0 : looksC os (changedVals ps vs0) (loc m) = none :=
        looksC_none _ _ _ (fun g hg => by
          have := W.disj g hg (loc m) hn
          exact this)
      rw [hn0]
      simp only [Option.orElse_none, lookup, hhk m]
      simp only [Bool.false_eq_true, if_false]
      rw [ctx_looks W.aligned vs0 hl (loc m), looks_fresh_K W vs0 (loc m) (fun f hf' => by
        exact fresh_not_kept W f hf' (loc m) hn)]
    obtain ⟨r', hr', hrel⟩ := stepB c me ps S D (Block.mk lets0 tail0) (Block.mk lets0 tail') hRT X1 X1
      (fun _ _ => rfl) r hs hev
    have hEloop : EvB c ((h, pk0) :: ctxOf (patternsOf pl) vs0) B1 r' := by
      rw [hB1]
      exact evBlockCons_val hEh hX1 hr'
    have hfinal : ∀ o, o ≠ ROutcome.stuck → Ev c ((h, pk0) :: ctxOf (patternsOf pl) vs0) (Expr.loop h B1) o →
        EvI c (Ident.generated IdentKind.function me) vs0 (finished o (fallibleType rty)) := by
      intro o hso hl'
      exact invoke_fwd HF (hbind vs0 hl) ((entry_block hpe0 hso).2 hl')
    rcases hrel with ⟨d, hd, hd'⟩ | ⟨hh, hh'⟩ | ⟨vs1, ys1, st, k, hCar, hPV, hrk, hr'eq⟩
    · subst hd; subst hd'
      have := hfinal (ROutcome.value d) (by simp) (compLoop_err hEloop)
      exact this
    · rw [hh'] at hEloop
      exact hfinal r hs (compLoop_halt hEloop hh)
    · subst hr'eq
      have hne : r ≠ ROutcome.exhausted := hev.1
      have hI : EvI c (Ident.generated IdentKind.function me) vs1 r := ⟨hne, k, hrk.symm⟩
      have hl1 : vs1.length = ps.length := carriedV_len hCar
      obtain ⟨E1, hb1, oo, hoo, hroo⟩ := invoke_inv HF hI hs
      have hE1 : E1 = ctxOf (patternsOf pl) vs1 := by
        have := hbind vs1 hl1
        rw [hb1] at this
        exact Option.some.inj this
      subst hE1
      obtain ⟨pk1, hPV1, hpe1⟩ := entry_pack (c := c) W hPR vs1 hl1
      have hys1 := carriedV_changed hCar
      rw [← hys1] at hPV1
      have hst : pk1 = st := packV_det hPV1 hPV
      subst hst
      have hoos : oo ≠ ROutcome.stuck := by
        intro e; rw [e] at hroo; simp [finished] at hroo; exact hs hroo
      have hL1 : Ev c ((h, pk1) :: ctxOf (patternsOf pl) vs1) (Expr.loop h B1) oo :=
        (entry_block hpe1 hoos).1 hoo
      have hAg : Agree (occE (Expr.loop h B1)) ((h, pk1) :: ctxOf (patternsOf pl) vs1)
          ((h, pk1) :: ((h, pk0) :: ctxOf (patternsOf pl) vs0)) := by
        intro i hi
        by_cases hsi : sameIdent h i = true
        · simp [lookup, hsi]
        · simp only [lookup, hsi]
          simp only [Bool.not_eq_true] at hsi
          simp only [Bool.false_eq_true, if_false]
          have hi' : i ∈ occB B1 := by
            simp only [occE, List.mem_cons] at hi
            rcases hi with hi | hi
            · rw [hi] at hsi; simp [sameIdent_refl] at hsi
            · exact hi
          have hfr : ∀ f ∈ fresh, sameIdent f i = false := fun f hf' => W.free f hf' i hi'
          rw [ctx_looks W.aligned vs1 hl1 i, ctx_looks W.aligned vs0 hl i,
            looks_fresh_K W vs1 i hfr, looks_fresh_K W vs0 i hfr]
          exact looksK_agree hCar W.keptDist hKEPT0 i
      have hL0 := Ev_agree hL1 hAg
      have hL := compLoop_ok hEloop hL0
      have hfin := hfinal oo hoos hL
      rw [hroo, finished_idem oo _ hoos]
      exact hfin

end Tre
