
namespace Tre

theorem invoke_fwd {c : Crate} {f : Ident} {pl : List Parameter} {rty : RustType} {bodyL : Block}
    {vs : List V} {E : Env} {o1 : ROutcome}
    (HF : findFunction c.items f = some (pl, rty, bodyL))
    (hb : bindPatterns (patternsOf pl) vs [] = some E) (hev : EvB c E bodyL o1) :
    EvI c f vs (finished o1 (fallibleType rty)) := by
  obtain ⟨k, hk⟩ := EvB.at hev
  refine ⟨?_, k + 1, ?_⟩
  · have := hev.1
    cases o1 <;> simp [finished] at this ⊢ <;> split <;> simp
  · rw [invoke]
    try simp only
    rw [HF]
    try simp only
    rw [hb]
    try simp only
    rw [hk _ (Nat.le_refl _)]

theorem invoke_inv {c : Crate} {f : Ident} {pl : List Parameter} {rty : RustType} {bodyL : Block}
    {vs : List V} {r : ROutcome}
    (HF : findFunction c.items f = some (pl, rty, bodyL))
    (hI : EvI c f vs r) (hs : r ≠ ROutcome.stuck) :
    ∃ E, bindPatterns (patternsOf pl) vs [] = some E ∧ ∃ o1, EvB c E bodyL o1 ∧
      r = finished o1 (fallibleType rty) := by
  obtain ⟨ne, k, hk⟩ := hI
  cases k with
  | zero => rw [invoke] at hk; exact absurd hk.symm ne
  | succ k =>
    rw [invoke] at hk
    try simp only at hk
    rw [HF] at hk
    try simp only at hk
    cases hb : bindPatterns (patternsOf pl) vs [] with
    | none => rw [hb] at hk; simp at hk; exact absurd hk.symm hs
    | some E =>
      rw [hb] at hk
      try simp only at hk
      refine ⟨E, rfl, evalBlock k c E bodyL, ⟨?_, k, rfl⟩, hk.symm⟩
      intro e
      rw [e] at hk
      simp [finished] at hk
      exact ne hk.symm

theorem finished_idem (o : ROutcome) (F : Bool) (h : o ≠ ROutcome.stuck) :
    finished (finished o F) F = finished o F := by
  cases o <;> cases F <;> simp [finished] at h ⊢

end Tre
