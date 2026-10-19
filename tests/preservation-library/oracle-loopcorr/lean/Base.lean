import LexLeanPreservation.RustSound
open LexLeanTarget LexLeanTarget.RustSyntax LexLeanTarget.RustSemantics
set_option pp.funBinderTypes false

namespace Tre
abbrev Env := List (Ident × TargetSyntax.Value)
abbrev V := TargetSyntax.Value

theorem eval_mono (n d : Nat) (c : Crate) (X : Env) (e : Expr)
    (h : eval n c X e ≠ ROutcome.exhausted) : eval (n + d) c X e = eval n c X e := by
  induction d with
  | zero => rfl
  | succ d ih =>
    have := (LexLeanPreservation.Rust.stable (n + d)).1 c X e (by rw [ih]; exact h)
    rw [← Nat.add_assoc, this, ih]

theorem evalList_mono (n d : Nat) (c : Crate) (X : Env) (e : List Expr)
    (h : evalList n c X e ≠ ROutcomes.exhausted) : evalList (n + d) c X e = evalList n c X e := by
  induction d with
  | zero => rfl
  | succ d ih =>
    have := (LexLeanPreservation.Rust.stable (n + d)).2.1 c X e (by rw [ih]; exact h)
    rw [← Nat.add_assoc, this, ih]

theorem evalBlock_mono (n d : Nat) (c : Crate) (X : Env) (e : Block)
    (h : evalBlock n c X e ≠ ROutcome.exhausted) : evalBlock (n + d) c X e = evalBlock n c X e := by
  induction d with
  | zero => rfl
  | succ d ih =>
    have := (LexLeanPreservation.Rust.stable (n + d)).2.2.1 c X e (by rw [ih]; exact h)
    rw [← Nat.add_assoc, this, ih]

theorem evalArms_mono (n d : Nat) (c : Crate) (X : Env) (v : V) (e : List Arm)
    (h : evalArms n c X v e ≠ ROutcome.exhausted) : evalArms (n + d) c X v e = evalArms n c X v e := by
  induction d with
  | zero => rfl
  | succ d ih =>
    have := (LexLeanPreservation.Rust.stable (n + d)).2.2.2.1 c X v e (by rw [ih]; exact h)
    rw [← Nat.add_assoc, this, ih]

theorem invoke_mono (n d : Nat) (c : Crate) (f : Ident) (a : List V)
    (h : invoke n c f a ≠ ROutcome.exhausted) : invoke (n + d) c f a = invoke n c f a := by
  induction d with
  | zero => rfl
  | succ d ih =>
    have := (LexLeanPreservation.Rust.stable (n + d)).2.2.2.2 c f a (by rw [ih]; exact h)
    rw [← Nat.add_assoc, this, ih]

/-- Evaluation to a settled outcome at some fuel. -/
def Ev (c : Crate) (X : Env) (e : Expr) (o : ROutcome) : Prop :=
  o ≠ ROutcome.exhausted ∧ ∃ k, eval k c X e = o
def EvB (c : Crate) (X : Env) (e : Block) (o : ROutcome) : Prop :=
  o ≠ ROutcome.exhausted ∧ ∃ k, evalBlock k c X e = o
def EvL (c : Crate) (X : Env) (e : List Expr) (o : ROutcomes) : Prop :=
  o ≠ ROutcomes.exhausted ∧ ∃ k, evalList k c X e = o
def EvA (c : Crate) (X : Env) (v : V) (e : List Arm) (o : ROutcome) : Prop :=
  o ≠ ROutcome.exhausted ∧ ∃ k, evalArms k c X v e = o
def EvI (c : Crate) (f : Ident) (a : List V) (o : ROutcome) : Prop :=
  o ≠ ROutcome.exhausted ∧ ∃ k, invoke k c f a = o

theorem Ev.at {c X e o} (h : Ev c X e o) : ∃ k, ∀ j, k ≤ j → eval j c X e = o := by
  obtain ⟨ne, k, hk⟩ := h
  refine ⟨k, fun j hj => ?_⟩
  obtain ⟨d, rfl⟩ := Nat.exists_eq_add_of_le hj
  rw [eval_mono k d c X e (by rw [hk]; exact ne), hk]
theorem EvB.at {c X e o} (h : EvB c X e o) : ∃ k, ∀ j, k ≤ j → evalBlock j c X e = o := by
  obtain ⟨ne, k, hk⟩ := h
  refine ⟨k, fun j hj => ?_⟩
  obtain ⟨d, rfl⟩ := Nat.exists_eq_add_of_le hj
  rw [evalBlock_mono k d c X e (by rw [hk]; exact ne), hk]
theorem EvL.at {c X e o} (h : EvL c X e o) : ∃ k, ∀ j, k ≤ j → evalList j c X e = o := by
  obtain ⟨ne, k, hk⟩ := h
  refine ⟨k, fun j hj => ?_⟩
  obtain ⟨d, rfl⟩ := Nat.exists_eq_add_of_le hj
  rw [evalList_mono k d c X e (by rw [hk]; exact ne), hk]
theorem EvA.at {c X v e o} (h : EvA c X v e o) : ∃ k, ∀ j, k ≤ j → evalArms j c X v e = o := by
  obtain ⟨ne, k, hk⟩ := h
  refine ⟨k, fun j hj => ?_⟩
  obtain ⟨d, rfl⟩ := Nat.exists_eq_add_of_le hj
  rw [evalArms_mono k d c X v e (by rw [hk]; exact ne), hk]
theorem EvI.at {c f a o} (h : EvI c f a o) : ∃ k, ∀ j, k ≤ j → invoke j c f a = o := by
  obtain ⟨ne, k, hk⟩ := h
  refine ⟨k, fun j hj => ?_⟩
  obtain ⟨d, rfl⟩ := Nat.exists_eq_add_of_le hj
  rw [invoke_mono k d c f a (by rw [hk]; exact ne), hk]


theorem evBlockNil {c X tail r} : EvB c X (Block.mk [] tail) r ↔ Ev c X tail r := by
  constructor
  · rintro ⟨ne, k, hk⟩
    cases k with
    | zero => rw [evalBlock] at hk; exact absurd hk.symm ne
    | succ k => rw [evalBlock] at hk; exact ⟨ne, k, hk⟩
  · rintro ⟨ne, k, hk⟩
    exact ⟨ne, k + 1, by rw [evalBlock]; exact hk⟩

theorem invBlockCons {c X pat ty b rest tail r} (hs : r ≠ ROutcome.stuck)
    (h : EvB c X (Block.mk (Let.mk pat ty b :: rest) tail) r) :
    (∃ v X', Ev c X b (ROutcome.value v) ∧ bindPattern pat v X = some X' ∧
        EvB c X' (Block.mk rest tail) r) ∨
    ((r = ROutcome.raise ∨ r = ROutcome.abort) ∧ Ev c X b r) := by
  obtain ⟨ne, k, hk⟩ := h
  cases k with
  | zero => rw [evalBlock] at hk; exact absurd hk.symm ne
  | succ k =>
    rw [evalBlock] at hk
    try simp only at hk
    generalize hb : eval k c X b = ob at hk
    cases ob with
    | value v =>
      try simp only at hk
      generalize hp : bindPattern pat v X = op at hk
      cases op with
      | none => try simp only at hk; exact absurd hk.symm hs
      | some X' =>
        try simp only at hk
        exact Or.inl ⟨v, X', ⟨by simp, k, hb⟩, hp, ne, k, hk⟩
    | raise => try simp only at hk; subst hk; exact Or.inr ⟨Or.inl rfl, ne, k, hb⟩
    | abort => try simp only at hk; subst hk; exact Or.inr ⟨Or.inr rfl, ne, k, hb⟩
    | stuck => try simp only at hk; exact absurd hk.symm hs
    | exhausted => try simp only at hk; exact absurd hk.symm ne


end Tre
