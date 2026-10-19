
namespace Tre

inductive All2 {α β : Type} (R : α → β → Prop) : List α → List β → Prop
  | nil : All2 R [] []
  | cons {a b as bs} : R a b → All2 R as bs → All2 R (a :: as) (b :: bs)

theorem All2.imp {α β : Type} {R S : α → β → Prop} (h : ∀ a b, R a b → S a b) :
    ∀ {as bs}, All2 R as bs → All2 S as bs
  | _, _, All2.nil => All2.nil
  | _, _, All2.cons hr hs => All2.cons (h _ _ hr) (All2.imp h hs)

theorem All2.imp' {α β : Type} {R S : α → β → Prop} :
    ∀ {as bs}, All2 R as bs → (∀ a b, a ∈ as → R a b → S a b) → All2 S as bs
  | _, _, All2.nil, _ => All2.nil
  | _, _, All2.cons hr hs, h =>
    All2.cons (h _ _ (by simp) hr) (All2.imp' hs (fun a b ha hab => h a b (by simp [ha]) hab))

theorem aligned_swap : ∀ {ps rc lp fs os}, Aligned ps rc lp fs os → Aligned ps lp rc os fs := by
  intro ps rc lp fs os h
  induction h with
  | nil => exact Aligned.nil
  | changed _ ih => exact Aligned.changed ih
  | kept _ ih => exact Aligned.kept ih
  | dropped _ ih => exact Aligned.dropped ih

theorem aligned_simple : ∀ {ps rc lp fs os}, Aligned ps rc lp fs os → ∀ q ∈ patternsOf lp, simplePat q := by
  intro ps rc lp fs os h
  induction h with
  | nil => intro q hq; simp [patternsOf] at hq
  | changed _ ih =>
    intro q hq
    simp only [patternsOf, List.mem_cons] at hq
    rcases hq with rfl | hq
    · simp [simplePat]
    · exact ih q (by simpa [patternsOf] using hq)
  | kept _ ih =>
    intro q hq
    simp only [patternsOf, List.mem_cons] at hq
    rcases hq with rfl | hq
    · simp [simplePat]
    · exact ih q (by simpa [patternsOf] using hq)
  | dropped _ ih =>
    intro q hq
    simp only [patternsOf, List.mem_cons] at hq
    rcases hq with rfl | hq
    · simp [simplePat]
    · exact ih q (by simpa [patternsOf] using hq)

theorem patternsOf_length (ps : List Parameter) : (patternsOf ps).length = ps.length := by
  induction ps with
  | nil => simp [patternsOf]
  | cons p ps ih => simp [patternsOf, ih]

theorem aligned_len : ∀ {ps rc lp fs os}, Aligned ps rc lp fs os → ∀ (vs : List V), vs.length = ps.length →
    (changedVals ps vs).length = os.length ∧ fs.length = os.length ∧ lp.length = ps.length := by
  intro ps rc lp fs os h
  induction h with
  | nil => intro vs _; simp [changedVals]
  | changed _ ih =>
    intro vs hl
    cases vs with
    | nil => simp at hl
    | cons v vs =>
      simp only [List.length_cons, Nat.add_right_cancel_iff] at hl
      obtain ⟨h1, h2, h3⟩ := ih vs hl
      simp [changedVals, h1, h2, h3]
  | kept _ ih =>
    intro vs hl
    cases vs with
    | nil => simp at hl
    | cons v vs =>
      simp only [List.length_cons, Nat.add_right_cancel_iff] at hl
      obtain ⟨h1, h2, h3⟩ := ih vs hl
      simp [changedVals, h1, h2, h3]
  | dropped _ ih =>
    intro vs hl
    cases vs with
    | nil => simp at hl
    | cons v vs =>
      simp only [List.length_cons, Nat.add_right_cancel_iff] at hl
      obtain ⟨h1, h2, h3⟩ := ih vs hl
      simp [changedVals, h1, h2, h3]

def extC : List Ident → List V → Env
  | a :: os, v :: ys => extC os ys ++ [(a, v)]
  | _, _ => []

theorem lookup_extC : ∀ (os : List Ident) (ys : List V) (i : Ident), lookup (extC os ys) i = looksC os ys i := by
  intro os
  induction os with
  | nil => intro ys i; cases ys <;> simp [extC, looksC, lookup]
  | cons a os ih =>
    intro ys i
    cases ys with
    | nil => simp [extC, looksC, lookup]
    | cons v ys =>
      simp only [extC, looksC, lookup_append, ih]
      simp [lookup]

theorem packV_single {v pk : V} (h : PackV [v] pk) : pk = v := by
  cases h with
  | one => rfl
  | cons h' => cases h'

theorem packV_cons2 {a b : V} {rest : List V} {pk : V} (h : PackV (a :: b :: rest) pk) :
    ∃ st, pk = TargetSyntax.Value.pair a st ∧ PackV (b :: rest) st := by
  cases h with
  | cons h' => exact ⟨_, rfl, h'⟩

theorem bind_tuple : ∀ (os : List Ident) (ys : List V) (pat : Pat) (pk : V),
    tuplePat os = some pat → PackV ys pk → os.length = ys.length →
    ∀ Y : Env, bindPattern pat pk Y = some (extC os ys ++ Y) := by
  intro os
  induction os with
  | nil => intro ys pat pk h; simp [tuplePat] at h
  | cons a os ih =>
    intro ys pat pk h hp hl Y
    cases os with
    | nil =>
      simp only [tuplePat, Option.some.injEq] at h
      subst h
      cases ys with
      | nil => simp at hl
      | cons v ys =>
        cases ys with
        | nil =>
          rw [packV_single hp]
          simp [bindPattern, extC]
        | cons w ys' => simp at hl
    | cons b os' =>
      cases hT : tuplePat (b :: os') with
      | none => simp [tuplePat, hT] at h
      | some t =>
        simp only [tuplePat, hT, Option.map_some, Option.some.injEq] at h
        subst h
        cases ys with
        | nil => simp at hl
        | cons v ys =>
          cases ys with
          | nil => simp at hl
          | cons w ys' =>
            obtain ⟨st, rfl, hst⟩ := packV_cons2 hp
            have hl' : (b :: os').length = (w :: ys').length := by simpa using hl
            simp only [bindPattern, bindPatterns]
            rw [ih (w :: ys') t st hT hst hl' ((a, v) :: Y)]
            simp [extC]

theorem packReads_ev {c : Crate} {E : Env} : ∀ (fs : List Ident) (ys : List V) (e : Expr),
    packReads fs = some e → All2 (fun f y => lookup E f = some y) fs ys →
    ∃ pk, PackV ys pk ∧ Ev c E e (ROutcome.value pk) := by
  intro fs
  induction fs with
  | nil => intro ys e h; simp [packReads] at h
  | cons f fs ih =>
    intro ys e h hf
    cases fs with
    | nil =>
      simp only [packReads, Option.some.injEq] at h
      subst h
      cases hf with
      | cons hl hr =>
        cases hr
        refine ⟨_, PackV.one, ⟨by simp, 2, ?_⟩⟩
        rw [eval]; rw [hl]
    | cons g gs =>
      cases hP : packReads (g :: gs) with
      | none => simp [packReads, hP] at h
      | some e' =>
        simp only [packReads, hP, Option.map_some, Option.some.injEq] at h
        subst h
        cases hf with
        | cons hl hr =>
          obtain ⟨pk, hpk, hev⟩ := ih _ e' hP hr
          refine ⟨TargetSyntax.Value.pair _ pk, PackV.cons hpk, compPair_val ?_ hev⟩
          exact ⟨by simp, 2, by rw [eval]; rw [hl]⟩

theorem looksC_none : ∀ (names : List Ident) (ys : List V) (f : Ident),
    (∀ g ∈ names, sameIdent g f = false) → looksC names ys f = none := by
  intro names
  induction names with
  | nil => intro ys f _; cases ys <;> simp [looksC]
  | cons g gs ih =>
    intro ys f h
    cases ys with
    | nil => simp [looksC]
    | cons z zs =>
      simp only [looksC]
      rw [ih zs f (fun a ha => h a (by simp [ha])), h g (by simp)]
      simp

theorem looksC_self : ∀ (names : List Ident) (ys : List V),
    names.Pairwise (fun a b => sameIdent a b = false) → names.length = ys.length →
    All2 (fun f y => looksC names ys f = some y) names ys := by
  intro names
  induction names with
  | nil => intro ys _ hl; cases ys with
    | nil => exact All2.nil
    | cons => simp at hl
  | cons f fs ih =>
    intro ys hd hl
    cases ys with
    | nil => simp at hl
    | cons y ys =>
      simp only [List.pairwise_cons] at hd
      have hl' : fs.length = ys.length := by simpa using hl
      refine All2.cons ?_ ?_
      · have hn : looksC fs ys f = none := by
          apply looksC_none
          intro g hg
          have := hd.1 g hg
          by_cases h : sameIdent g f = true
          · have h2 := sameIdent_symm _ _ h
            rw [this] at h2; simp at h2
          · simpa using h
        simp only [looksC, hn]
        simp [sameIdent_refl]
      · have := ih ys hd.2 hl'
        refine All2.imp (R := fun f y => looksC fs ys f = some y) ?_ this
        intro g z hgz
        simp only [looksC, hgz]
        simp

end Tre
