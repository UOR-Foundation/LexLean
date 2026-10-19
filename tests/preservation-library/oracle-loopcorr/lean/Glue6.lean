
namespace Tre

def distinctI : List Ident → Bool
  | [] => true
  | a :: rest => (rest.all fun b => !sameIdent a b) && distinctI rest

theorem distinctI_pairwise : ∀ {l : List Ident}, distinctI l = true →
    l.Pairwise (fun a b => sameIdent a b = false) := by
  intro l
  induction l with
  | nil => intro _; exact List.Pairwise.nil
  | cons a rest ih =>
    intro h
    simp only [distinctI, Bool.and_eq_true, List.all_eq_true, Bool.not_eq_true'] at h
    refine List.Pairwise.cons ?_ (ih h.2)
    intro b hb
    simpa using h.1 b hb

def isLoc : Ident → Bool
  | Ident.generated IdentKind.binding _ => true
  | _ => false

theorem isLoc_eq {n : Ident} (h : isLoc n = true) : ∃ m, n = loc m := by
  cases n with
  | generated k m => cases k <;> simp [isLoc] at h; exact ⟨m, rfl⟩
  | exported s => simp [isLoc] at h

def loopCheck (ps : List Pos) (pns : List Nat) (os : List Ident) (B1 : Block) : Bool :=
  distinctI (pns.map paramId) && distinctI (keptNames ps) &&
  (os.all fun o => (keptNames ps).all fun n => !sameIdent o n) &&
  (keptNames ps).all isLoc &&
  ((pns.map paramId).all fun f => (occB B1).all fun i => !sameIdent f i)

theorem wf_of_loopCheck {ps rc pl pns os B1}
    (hLP : loopParams ps rc (pns.map paramId) = some (pl, os))
    (hC : loopCheck ps pns os B1 = true) : LoopWF ps rc pl (pns.map paramId) os B1 := by
  simp only [loopCheck, Bool.and_eq_true, List.all_eq_true, Bool.not_eq_true'] at hC
  obtain ⟨⟨⟨⟨h1, h2⟩, h3⟩, h4⟩, h5⟩ := hC
  refine ⟨loopParams_aligned _ _ _ _ _ hLP, distinctI_pairwise h1, ?_, distinctI_pairwise h2, ?_, ?_, ?_⟩
  · intro o ho n hn
    simpa using h3 o ho n hn
  · intro n hn; exact isLoc_eq (h4 n hn)
  · intro f hf
    obtain ⟨k, hk⟩ := List.mem_map.1 hf
    exact ⟨k, hk.2.symm⟩
  · intro f hf i hi
    simpa using h5 f hf i hi

def loopCheckOf (me : Nat) (ps : List Pos) (S D : RustType) (hn : Nat) (pns : List Nat)
    (params : List Parameter) (body : Block) : Bool :=
  match loopParts me ps S D hn pns params body with
  | some (_, os, _, B1) => loopCheck ps pns os B1
  | none => false

theorem loopParts_inv {me ps S D hn pns rc lets0 tail0 pl os pe B1}
    (h : loopParts me ps S D hn pns rc (Block.mk lets0 tail0) = some (pl, os, pe, B1)) :
    ∃ tp tail', loopParams ps rc (pns.map paramId) = some (pl, os) ∧
      rtB me ps S D (Block.mk lets0 tail0) = some (Block.mk lets0 tail') ∧
      packReads (pns.map paramId) = some pe ∧ tuplePat os = some tp ∧
      B1 = Block.mk (Let.mk tp none (Expr.move (holderId hn)) :: lets0) tail' := by
  unfold loopParts at h
  cases hLP : loopParams ps rc (pns.map paramId) with
  | none => rw [hLP] at h; simp at h
  | some r =>
    obtain ⟨pl', os'⟩ := r
    cases hRT : rtB me ps S D (Block.mk lets0 tail0) with
    | none => rw [hLP, hRT] at h; simp at h
    | some b' =>
      obtain ⟨l', tail'⟩ := b'
      have hl' : l' = lets0 := by
        have := rtB_inv hRT
        obtain ⟨_, t', _, hb⟩ := this
        exact (by simpa using hb : l' = lets0 ∧ tail' = t').1
      subst hl'
      rw [hLP, hRT] at h
      dsimp only at h
      cases hPR : packReads (pns.map paramId) with
      | none => rw [hPR] at h; simp at h
      | some pe' =>
        cases hTP : tuplePat os' with
        | none => rw [hPR, hTP] at h; simp at h
        | some tp =>
          rw [hPR, hTP] at h
          simp only [Option.some.injEq, Prod.mk.injEq] at h
          obtain ⟨rfl, rfl, rfl, rfl⟩ := h
          exact ⟨tp, tail', rfl, rfl, rfl, hTP, rfl⟩

end Tre
