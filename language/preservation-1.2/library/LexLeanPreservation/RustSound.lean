import LexLeanPreservation.RustMatches
set_option linter.unusedSimpArgs false

/-! The soundness of the correspondence (SPEC.md §17.17): every derivation's
judgment holds at every fuel, so a crate every function of which
corresponds to the program's simulates it. Certificate B applies
`simulate` to its derivations. -/

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

section rules
variable {p : Program} {c : RCrate} {A : Flags}

/-- Every function is simulated at fuel `n`, given the correspondence of
its body at `n`. -/
theorem funSem_of {n} (ih : ∀ Γ fl j, Corr p c A Γ fl j → Sem p c A n Γ fl j)
    (hcrate : CrateOK p c A) (f : Nat) : FunSem p c A n f := by
  intro fn args env F hidx hw hb hF o ho hst
  obtain ⟨fn', hidx', hcase⟩ := hcrate f fn hidx
  rw [hidx] at hidx'; cases hidx'
  rcases hcase with ⟨params, rty, lets, tail, Γ, hfind, hctx, hcorr⟩ | ⟨U, t, hU, hmem, hun⟩
  · have hF' : RustSemantics.fallibleType rty = F := by
      simp [fnFallible, hfind] at hF; exact hF
    obtain ⟨renv, hbp, hrel⟩ := param_env [] fn.parameters fn.types _ Γ args [] env []
      hctx (by intro x t s h; simp [ctxLookup] at h) hw hb
    have hinvoke : ∀ ro, RCB c renv ⟨lets, tail⟩ ro →
        RustSemantics.finished ro F ≠ .exhausted →
        RCI c (fnIdent f) args (RustSemantics.finished ro F) := by
      intro ro hro hne
      obtain ⟨m, hm⟩ := (foldB_sound hro).at
      refine ⟨hne, m+1, ?_⟩
      rw [RustSemantics.invoke.eq_2, hfind]
      simp only [hbp, hF', hm m (Nat.le_refl _)]
    rw [hF'] at hcorr
    cases F with
    | false =>
      obtain ⟨hwt, hcase⟩ := ih Γ false _ hcorr env renv hrel o ho hst
      refine ⟨hwt, ?_⟩
      rcases hcase with ⟨new, _, hl, ro, hro, hrc⟩ | ⟨rfl, h, hrh, hraise⟩
      · cases o with
        | value v =>
          cases ro with
          | value w =>
            simp only [RealizesM, Realizes] at hro; subst hro
            exact ⟨.value w, rfl, hinvoke _ (block_of_lets hl hrc) nofun⟩
          | _ => simp [RealizesM, Realizes] at hro
        | overflow =>
          cases ro with
          | abort => exact ⟨.abort, trivial, hinvoke _ (block_of_lets hl hrc) nofun⟩
          | _ => simp [RealizesM, Realizes] at hro
        | stuck => exact absurd rfl hst
      · cases h with
        | abort => exact ⟨.abort, trivial, hinvoke _ (block_of_raise hraise) nofun⟩
        | _ => simp [RealizesM, Realizes] at hrh
    | true =>
      obtain ⟨hwt, hcase⟩ := ih Γ true _ hcorr env renv hrel o ho hst
      refine ⟨hwt, ?_⟩
      rcases hcase with ⟨new, _, hl, ro, hro, hrc⟩ | ⟨rfl, _, _, hraise⟩
      · cases o with
        | value v =>
          cases ro <;> simp [RealizesM, RealizesF] at hro
          subst hro
          exact ⟨.value (.ok v), rfl, hinvoke _ (block_of_lets hl hrc) nofun⟩
        | overflow =>
          cases ro with
          | raise => exact ⟨.value (.error .unit), ⟨rfl, rfl⟩, hinvoke _ (block_of_lets hl hrc) nofun⟩
          | abort => exact ⟨.abort, trivial, hinvoke _ (block_of_lets hl hrc) nofun⟩
          | value w =>
            simp [RealizesM, RealizesF] at hro; subst hro
            exact ⟨.value (.error .unit), ⟨rfl, rfl⟩, hinvoke _ (block_of_lets hl hrc) nofun⟩
          | _ => simp [RealizesM, RealizesF] at hro
        | stuck => exact absurd rfl hst
      · rcases hraise.halt with rfl | rfl
        · exact ⟨.value (.error .unit), ⟨rfl, rfl⟩, hinvoke _ (block_of_raise hraise) nofun⟩
        · exact ⟨.abort, trivial, hinvoke _ (block_of_raise hraise) nofun⟩
  · exfalso
    have := wtl_uninh hU args fn.types hw
    simp only [List.any_eq_false] at this
    exact absurd hun (by simpa using this t hmem)

theorem kind_ne {k : RustSyntax.IdentKind} (h : isTemp k = true) : k ≠ .binding := by
  intro e; subst e; simp [isTemp] at h

theorem item_ne {item : RustSyntax.Item} (h : plainItem item = true) : item ≠ .natSucc := by
  intro e; subst e; simp [plainItem] at h

theorem fl_of {F fl : Bool} (h : (!F || fl) = true) : F = true → fl = true := by
  intro hF; subst hF; simpa using h

/-- The correspondence is sound: every judgment it derives holds at every
fuel. -/
theorem sound (hcrate : CrateOK p c A) :
    ∀ n Γ fl j, Corr p c A Γ fl j → Sem p c A n Γ fl j := by
  have key : ∀ n, ∀ m ≤ n, ∀ Γ fl j, Corr p c A Γ fl j → Sem p c A m Γ fl j := by
    intro n
    induction n with
    | zero => intro m hm Γ fl j _; rw [Nat.le_zero.mp hm]; exact sem_zero j
    | succ n ihn =>
      intro m hm
      rcases Nat.lt_or_ge m (n+1) with hlt | hge
      · exact ihn m (by omega)
      · have : m = n + 1 := by omega
        subst this
        have hfun := fun f => funSem_of (ihn n (Nat.le_refl _)) hcrate f
        have low := ihn n (Nat.le_refl _)
        intro Γ fl j h
        induction h with
        | var hx hr => exact sem_var hx hr
        | varUnit hx => exact sem_varUnit hx
        | bOfE _ ih' => exact sem_bOfE ih'
        | eOfB _ ih' => exact sem_eOfBFold ih'
        | lNil => exact sem_lNil
        | lCons he hes _ _ => exact sem_lCons (low _ _ _ he) (low _ _ _ hes)
        | opsNil => exact sem_opsNil
        | opsCons he hes hk hf _ _ => exact sem_opsCons (low _ _ _ he) (low _ _ _ hes) (kind_ne hk) hf
        | prim hops hop hs hok hin hfl _ =>
          exact sem_prim (low _ _ _ hops) hop (item_ne hs) hok hin (fl_of hfl)
        | fOfB _ ih' => exact sem_fOfB ih'
        | value hl ht => exact sem_value hl ht
        | eOfBNil _ ih' => exact sem_eOfBNil ih'
        | opsOfL _ ih' => exact sem_opsOfL ih'
        | opsUnit he hes _ _ => exact sem_opsUnit (low _ _ _ he) (low _ _ _ hes)
        | build hops hb _ => exact sem_build (sem_ops_box _ (low _ _ _ hops)) hb
        | buildPair hl _ => exact sem_buildPair (low _ _ _ hl)
        | buildConst hl => exact sem_buildConst hl
        | letBind hb hf hbody _ _ => exact sem_letBind (low _ _ _ hb) hf (low _ _ _ hbody)
        | letWild hb hbody _ _ => exact sem_letWild (low _ _ _ hb) (low _ _ _ hbody)
        | buildSucc hl _ => exact sem_buildSucc (low _ _ _ hl)
        | primWiden hops hok hin _ => exact sem_primWiden (low _ _ _ hops) hok hin
        | primF hops hop hs hok hin hfal _ =>
          exact sem_primF (low _ _ _ hops) hop (item_ne hs) hok hin hfal
        | callOps hops hidx hts hres hF hfl _ =>
          exact sem_callOps (low _ _ _ hops) (hfun _) hidx hts hres hF (fl_of hfl)
        | callF hops hidx hts hres hF _ => exact sem_callF (low _ _ _ hops) (hfun _) hidx hts hres hF
        | buildSuccF hl _ => exact sem_buildSuccF (low _ _ _ hl)
        | fUnit _ ih' => exact sem_fUnit ih'
        | cond hc hk _ _ => exact sem_cond (low _ _ _ hc) (low _ _ _ hk) (k_halt hk)
        | condId hc ha hb _ _ _ => exact sem_condId (low _ _ _ hc) (low _ _ _ ha) (low _ _ _ hb)
        | kId _ _ _ iha ihb => exact sem_kId iha ihb
        | kNeg _ _ _ iha ihb => exact sem_kNeg iha ihb
        | kSame _ _ he iha ihb => exact sem_kSame iha ihb he
        | kIf _ _ iha ihb => exact sem_kIf iha ihb
        | kHeld _ _ hk hh iha ihb => exact sem_kHeld iha ihb (kind_ne hk) hh
        | matchNat hs harms hsl hk hkd _ _ =>
          exact sem_matchNat (low _ _ _ hs) harms hsl (kind_ne hk)
            (fun m hm => ihn m hm _ _ _ hkd)
        | matchBool hs harms hk hkd _ _ =>
          exact sem_matchBool (low _ _ _ hs) harms (kind_ne hk) (fun m hm => ihn m hm _ _ _ hkd)
        | matchBoolId hs harms ha hb _ _ _ =>
          exact sem_matchBoolId (low _ _ _ hs) harms (fun m hm => ihn m hm _ _ _ ha)
            (fun m hm => ihn m hm _ _ _ hb)
        | matchArms hs hm hU hcov hk _ _ =>
          exact sem_matchArms (low _ _ _ hs) (fun m hm' => ihn m hm' _ _ _ hm) hU hcov (kind_ne hk)
        | matchRebuilt hs hm hreb hU hcov _ _ =>
          exact sem_matchRebuilt (low _ _ _ hs) (fun m hm' => ihn m hm' _ _ _ hm) hreb hU hcov
        | mNil => exact sem_mNil
        | mCons _ hj hoth hpat _ ihrest ihbody => exact sem_mCons ihrest hj hoth hpat ihbody
        | matchPair hs hpc hbody hk _ _ =>
          exact sem_matchPair (low _ _ _ hs) hpc (fun m hm => ihn m hm _ _ _ hbody) (kind_ne hk)
        | matchUnit hs hbody _ _ =>
          exact sem_matchUnit (low _ _ _ hs) (fun m hm => ihn m hm _ _ _ hbody)
        | first hs hk _ => exact sem_part true (low _ _ _ hs) (kind_ne hk)
        | second hs hk _ => exact sem_part false (low _ _ _ hs) (kind_ne hk)
        | fieldRead hr hone hts hpos _ hpr hfm _ =>
          exact sem_fieldRead (low _ _ _ hr) hone hts hpos hpr hfm
        | fieldHeld hr hone hts hpos _ hpr hk' hfm _ =>
          exact sem_fieldHeld (low _ _ _ hr) hone hts hpos hpr (kind_ne hk') hfm
        | closure hops hfn htys hres hflag hd hF hfa _ =>
          exact sem_closure (low _ _ _ hops) hfn htys hres hflag hd hF hfa
        | apply htg hops hflag hfl hk hfresh _ _ =>
          exact sem_apply (low _ _ _ htg) (low _ _ _ hops) hfun hflag (fl_of hfl) (kind_ne hk) hfresh
        | applyF htg hops hflag hk hfresh _ _ =>
          exact sem_applyF (low _ _ _ htg) (low _ _ _ hops) hfun hflag (kind_ne hk) hfresh
  exact fun n => key n n (Nat.le_refl _)

/-- Every function of a corresponding program and crate is simulated. -/
theorem simulate (hcrate : CrateOK p c A) (n f : Nat) : FunSem p c A n f :=
  funSem_of (sound hcrate n) hcrate f

end rules
end LexLeanPreservation.Rust
