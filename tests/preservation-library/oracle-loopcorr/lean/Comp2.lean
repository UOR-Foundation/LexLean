
namespace Tre

def Halt (r : ROutcome) : Prop := r = ROutcome.raise ∨ r = ROutcome.abort

theorem evBlockCons_val {c X pat ty b rest tail r v X'}
    (hb : Ev c X b (ROutcome.value v)) (hp : bindPattern pat v X = some X')
    (hr : EvB c X' (Block.mk rest tail) r) :
    EvB c X (Block.mk (Let.mk pat ty b :: rest) tail) r := by
  have ne := hr.1
  obtain ⟨kb, hb'⟩ := Ev.at hb
  obtain ⟨kr, hr'⟩ := EvB.at hr
  refine ⟨ne, max kb kr + 1, ?_⟩
  rw [evalBlock, hb' _ (Nat.le_max_left _ _)]
  try simp only
  rw [hp]
  try simp only
  exact hr' _ (Nat.le_max_right _ _)

theorem evBlockCons_halt {c X pat ty b rest tail r}
    (hb : Ev c X b r) (hh : Halt r) :
    EvB c X (Block.mk (Let.mk pat ty b :: rest) tail) r := by
  obtain ⟨kb, hb'⟩ := Ev.at hb
  have ne := hb.1
  refine ⟨ne, kb + 1, ?_⟩
  rw [evalBlock, hb' _ (Nat.le_refl _)]
  rcases hh with rfl | rfl <;> rfl

theorem invCond {c X cd t e r} (hs : r ≠ ROutcome.stuck)
    (h : Ev c X (Expr.cond cd t e) r) :
    (∃ flag, Ev c X cd (ROutcome.value (TargetSyntax.Value.bool flag)) ∧ EvB c X (if flag then t else e) r) ∨
    (Halt r ∧ Ev c X cd r) := by
  obtain ⟨ne, k, hk⟩ := h
  cases k with
  | zero => rw [eval] at hk; exact absurd hk.symm ne
  | succ k =>
    rw [eval] at hk
    generalize hc : eval k c X cd = oc at hk
    cases oc with
    | value v =>
      cases v with
      | bool flag =>
        try simp only at hk
        refine Or.inl ⟨flag, ⟨by simp, k, hc⟩, ne, k, ?_⟩
        cases flag <;> simpa using hk
      | _ => try simp only at hk; exact absurd hk.symm hs
    | raise => try simp only at hk; subst hk; exact Or.inr ⟨Or.inl rfl, ne, k, hc⟩
    | abort => try simp only at hk; subst hk; exact Or.inr ⟨Or.inr rfl, ne, k, hc⟩
    | stuck => try simp only at hk; exact absurd hk.symm hs
    | exhausted => try simp only at hk; exact absurd hk.symm ne

theorem compCond_val {c X cd t e r} {flag : Bool}
    (hc : Ev c X cd (ROutcome.value (TargetSyntax.Value.bool flag))) (hb : EvB c X (if flag then t else e) r) :
    Ev c X (Expr.cond cd t e) r := by
  obtain ⟨kc, hc'⟩ := Ev.at hc
  obtain ⟨kb, hb'⟩ := EvB.at hb
  refine ⟨hb.1, max kc kb + 1, ?_⟩
  rw [eval, hc' _ (Nat.le_max_left _ _)]
  try simp only
  cases flag
  · simpa using hb' _ (Nat.le_max_right _ _)
  · simpa using hb' _ (Nat.le_max_right _ _)

theorem compCond_halt {c X cd t e r} (hc : Ev c X cd r) (hh : Halt r) :
    Ev c X (Expr.cond cd t e) r := by
  obtain ⟨kc, hc'⟩ := Ev.at hc
  refine ⟨hc.1, kc + 1, ?_⟩
  rw [eval, hc' _ (Nat.le_refl _)]
  rcases hh with rfl | rfl <;> rfl

theorem invBlockE {c X b r} : Ev c X (Expr.block b) r ↔ EvB c X b r := by
  constructor
  · rintro ⟨ne, k, hk⟩
    cases k with
    | zero => rw [eval] at hk; exact absurd hk.symm ne
    | succ k => rw [eval] at hk; exact ⟨ne, k, hk⟩
  · rintro ⟨ne, k, hk⟩
    exact ⟨ne, k + 1, by rw [eval]; exact hk⟩

theorem invMatch {c X s arms r} (hs : r ≠ ROutcome.stuck)
    (h : Ev c X (Expr.matchOn s arms) r) :
    (∃ v, Ev c X s (ROutcome.value v) ∧ EvA c X v arms r) ∨ (Halt r ∧ Ev c X s r) := by
  obtain ⟨ne, k, hk⟩ := h
  cases k with
  | zero => rw [eval] at hk; exact absurd hk.symm ne
  | succ k =>
    rw [eval] at hk
    generalize hc : eval k c X s = oc at hk
    cases oc with
    | value v =>
      try simp only at hk
      exact Or.inl ⟨v, ⟨by simp, k, hc⟩, ne, k, hk⟩
    | raise => try simp only at hk; subst hk; exact Or.inr ⟨Or.inl rfl, ne, k, hc⟩
    | abort => try simp only at hk; subst hk; exact Or.inr ⟨Or.inr rfl, ne, k, hc⟩
    | stuck => try simp only at hk; exact absurd hk.symm hs
    | exhausted => try simp only at hk; exact absurd hk.symm ne

theorem compMatch_val {c X s arms r v}
    (hc : Ev c X s (ROutcome.value v)) (ha : EvA c X v arms r) :
    Ev c X (Expr.matchOn s arms) r := by
  obtain ⟨kc, hc'⟩ := Ev.at hc
  obtain ⟨ka, ha'⟩ := EvA.at ha
  refine ⟨ha.1, max kc ka + 1, ?_⟩
  rw [eval, hc' _ (Nat.le_max_left _ _)]
  try simp only
  exact ha' _ (Nat.le_max_right _ _)

theorem compMatch_halt {c X s arms r} (hc : Ev c X s r) (hh : Halt r) :
    Ev c X (Expr.matchOn s arms) r := by
  obtain ⟨kc, hc'⟩ := Ev.at hc
  refine ⟨hc.1, kc + 1, ?_⟩
  rw [eval, hc' _ (Nat.le_refl _)]
  rcases hh with rfl | rfl <;> rfl

theorem invArmsCons {c X v p b rest r} (hs : r ≠ ROutcome.stuck)
    (h : EvA c X v (Arm.mk p b :: rest) r) :
    (∃ X', bindPattern p v X = some X' ∧ EvB c X' b r) ∨
    (bindPattern p v X = none ∧ EvA c X v rest r) := by
  obtain ⟨ne, k, hk⟩ := h
  cases k with
  | zero => rw [evalArms] at hk; exact absurd hk.symm ne
  | succ k =>
    rw [evalArms] at hk
    try simp only at hk
    generalize hp : bindPattern p v X = op at hk
    cases op with
    | none => try simp only at hk; exact Or.inr ⟨rfl, ne, k, hk⟩
    | some X' => try simp only at hk; exact Or.inl ⟨X', rfl, ne, k, hk⟩

theorem compArmsCons_some {c X v p b rest r X'}
    (hp : bindPattern p v X = some X') (hb : EvB c X' b r) :
    EvA c X v (Arm.mk p b :: rest) r := by
  obtain ⟨kb, hb'⟩ := EvB.at hb
  refine ⟨hb.1, kb + 1, ?_⟩
  rw [evalArms]
  try simp only
  rw [hp]
  exact hb' _ (Nat.le_refl _)

theorem compArmsCons_none {c X v p b rest r}
    (hp : bindPattern p v X = none) (hb : EvA c X v rest r) :
    EvA c X v (Arm.mk p b :: rest) r := by
  obtain ⟨kb, hb'⟩ := EvA.at hb
  refine ⟨hb.1, kb + 1, ?_⟩
  rw [evalArms]
  try simp only
  rw [hp]
  exact hb' _ (Nat.le_refl _)

theorem invArmsNil {c X v r} (h : EvA c X v [] r) : r = ROutcome.stuck := by
  obtain ⟨ne, k, hk⟩ := h
  cases k with
  | zero => rw [evalArms] at hk; exact absurd hk.symm ne
  | succ k =>
    rw [evalArms] at hk
    exact hk.symm

end Tre
