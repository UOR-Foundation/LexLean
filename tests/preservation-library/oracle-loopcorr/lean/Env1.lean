
namespace Tre

def binderOf : Pat → V → Env
  | Pat.bind b, v => [(b, v)]
  | _, _ => []

def ctxOf : List Pat → List V → Env
  | [], [] => []
  | p :: ps, v :: vs => ctxOf ps vs ++ binderOf p v
  | _, _ => []

def simplePat : Pat → Prop
  | Pat.wild => True
  | Pat.bind _ => True
  | _ => False

theorem bindPatterns_ctx : ∀ (pats : List Pat) (vs : List V) (E : Env),
    (∀ p ∈ pats, simplePat p) → pats.length = vs.length →
    bindPatterns pats vs E = some (ctxOf pats vs ++ E) := by
  intro pats
  induction pats with
  | nil =>
    intro vs E _ hl
    cases vs with
    | nil => simp [bindPatterns, ctxOf]
    | cons => simp at hl
  | cons p ps ih =>
    intro vs E hs hl
    cases vs with
    | nil => simp at hl
    | cons v vs =>
      have hp := hs p (by simp)
      have hs' : ∀ q ∈ ps, simplePat q := fun q hq => hs q (by simp [hq])
      have hl' : ps.length = vs.length := by simpa using hl
      simp only [bindPatterns]
      cases p with
      | wild =>
        simp only [bindPattern]
        rw [ih vs E hs' hl']
        simp [ctxOf, binderOf]
      | bind b =>
        simp only [bindPattern]
        rw [ih vs ((b, v) :: E) hs' hl']
        simp [ctxOf, binderOf]
      | _ => exact absurd hp (by simp [simplePat])

theorem lookup_append (A B : Env) (i : Ident) :
    lookup (A ++ B) i = (lookup A i).orElse (fun _ => lookup B i) := by
  induction A with
  | nil => simp [lookup]
  | cons e rest ih =>
    obtain ⟨a, v⟩ := e
    simp only [List.cons_append, lookup]
    by_cases h : sameIdent a i = true
    · simp [h]
    · simp [h, ih]

end Tre
