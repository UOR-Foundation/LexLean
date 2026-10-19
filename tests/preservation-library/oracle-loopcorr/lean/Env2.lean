
namespace Tre

theorem sameIdent_eq (a b : Ident) (h : sameIdent a b = true) : a = b := by
  cases a with
  | generated k n =>
    cases b with
    | generated k' n' =>
      simp only [sameIdent, Bool.and_eq_true] at h
      obtain ⟨h1, h2⟩ := h
      have h2' := Nat.eq_of_beq_eq_true h2
      subst h2'
      have : k = k' := by cases k <;> cases k' <;> simp_all [kindNumber]
      subst this; rfl
    | exported s => simp [sameIdent] at h
  | exported s =>
    cases b with
    | generated k n => simp [sameIdent] at h
    | exported s' => simpa [sameIdent, LexLeanRuntime.equal] using h

theorem sameIdent_refl (a : Ident) : sameIdent a a = true := by
  cases a <;> simp [sameIdent, LexLeanRuntime.equal]

inductive Aligned : List Pos → List Parameter → List Parameter → List Ident → List Ident → Prop
  | nil : Aligned [] [] [] [] []
  | changed {ps rc lp fs os b f ty} : Aligned ps rc lp fs os →
      Aligned (Pos.changed :: ps) (Parameter.mk (Pat.bind b) ty :: rc)
        (Parameter.mk (Pat.bind f) ty :: lp) (f :: fs) (b :: os)
  | kept {ps rc lp fs os n ty} : Aligned ps rc lp fs os →
      Aligned (Pos.kept n :: ps) (Parameter.mk (Pat.bind n) ty :: rc)
        (Parameter.mk (Pat.bind n) ty :: lp) fs os
  | dropped {ps rc lp fs os ty} : Aligned ps rc lp fs os →
      Aligned (Pos.dropped :: ps) (Parameter.mk Pat.wild ty :: rc)
        (Parameter.mk Pat.wild ty :: lp) fs os

def looksC : List Ident → List V → Ident → Option V
  | f :: fs, v :: ys, i => (looksC fs ys i).orElse (fun _ => if sameIdent f i then some v else none)
  | _, _, _ => none

def looksK : List Pos → List V → Ident → Option V
  | Pos.changed :: ps, _ :: vs, i => looksK ps vs i
  | Pos.kept n :: ps, v :: vs, i => (looksK ps vs i).orElse (fun _ => if sameIdent n i then some v else none)
  | Pos.dropped :: ps, _ :: vs, i => looksK ps vs i
  | _, _, _ => none

def changedVals : List Pos → List V → List V
  | Pos.changed :: ps, v :: vs => v :: changedVals ps vs
  | Pos.kept _ :: ps, _ :: vs => changedVals ps vs
  | Pos.dropped :: ps, _ :: vs => changedVals ps vs
  | _, _ => []

def looks : List Pos → List Ident → List V → Ident → Option V
  | Pos.changed :: ps, f :: fs, v :: vs, i =>
      (looks ps fs vs i).orElse (fun _ => if sameIdent f i then some v else none)
  | Pos.kept n :: ps, fs, v :: vs, i =>
      (looks ps fs vs i).orElse (fun _ => if sameIdent n i then some v else none)
  | Pos.dropped :: ps, fs, _ :: vs, i => looks ps fs vs i
  | _, _, _, _ => none

theorem ctx_looks : ∀ {ps rc lp fs os}, Aligned ps rc lp fs os → ∀ (vs : List V), vs.length = ps.length →
    ∀ i, lookup (ctxOf (patternsOf lp) vs) i = looks ps fs vs i := by
  intro ps rc lp fs os h
  induction h with
  | nil => intro vs hl i; cases vs <;> simp_all [patternsOf, ctxOf, lookup, looks]
  | changed _ ih =>
    intro vs hl i
    cases vs with
    | nil => simp at hl
    | cons v vs =>
      simp only [List.length_cons, Nat.add_right_cancel_iff] at hl
      have hl' := hl
      simp only [patternsOf, ctxOf, binderOf, lookup_append, looks, ih vs hl' i]
      simp [lookup]
  | kept _ ih =>
    intro vs hl i
    cases vs with
    | nil => simp at hl
    | cons v vs =>
      simp only [List.length_cons, Nat.add_right_cancel_iff] at hl
      have hl' := hl
      simp only [patternsOf, ctxOf, binderOf, lookup_append, looks, ih vs hl' i]
      simp [lookup]
  | dropped _ ih =>
    intro vs hl i
    cases vs with
    | nil => simp at hl
    | cons v vs =>
      simp only [List.length_cons, Nat.add_right_cancel_iff] at hl
      have hl' := hl
      simp only [patternsOf, ctxOf, binderOf, lookup_append, looks, ih vs hl' i]
      simp [lookup]

end Tre
