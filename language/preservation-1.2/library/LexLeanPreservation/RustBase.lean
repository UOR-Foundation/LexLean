import LexLeanTarget.RustSemantics
import LexLeanPreservation.Core
set_option linter.unusedSimpArgs false

/-! The declared Rust machine (SPEC.md §17.17), as certificate B reasons about
it: its fuel stability, the value typing and scopes the correspondence
relates environments by, and convergence of Rust expressions, lists,
blocks, and invocations at some fuel. -/

namespace LexLeanPreservation.Rust
open LexLeanTarget.TargetSyntax LexLeanTarget.RustSyntax LexLeanTarget.RustSemantics

theorem returned_ne {r : ROutcome} {p : Bool} (h : returned r p ≠ .exhausted) : r ≠ .exhausted := by
  intro hr; subst hr; cases p <;> exact h rfl
theorem dispatched_ne {r : ROutcome} {w : Bool} (h : dispatched r w ≠ .exhausted) : r ≠ .exhausted := by
  intro hr; subst hr; cases w <;> exact h rfl
theorem finished_ne {r : ROutcome} {f : Bool} (h : finished r f ≠ .exhausted) : r ≠ .exhausted := by
  intro hr; subst hr; exact h rfl

/-- The declared Rust machine is stable in its fuel: an evaluation that did
not run out gives the same outcome with more fuel. -/
theorem stable : ∀ n,
    (∀ c env e, eval n c env e ≠ .exhausted → eval (n+1) c env e = eval n c env e) ∧
    (∀ c env es, evalList n c env es ≠ .exhausted → evalList (n+1) c env es = evalList n c env es) ∧
    (∀ c env b, evalBlock n c env b ≠ .exhausted → evalBlock (n+1) c env b = evalBlock n c env b) ∧
    (∀ c env v arms, evalArms n c env v arms ≠ .exhausted →
      evalArms (n+1) c env v arms = evalArms n c env v arms) ∧
    (∀ c f args, invoke n c f args ≠ .exhausted → invoke (n+1) c f args = invoke n c f args) := by
  intro n
  induction n with
  | zero => exact ⟨fun _ _ _ h => absurd rfl h, fun _ _ _ h => absurd rfl h,
      fun _ _ _ h => absurd rfl h, fun _ _ _ _ h => absurd rfl h, fun _ _ _ h => absurd rfl h⟩
  | succ n ih =>
    obtain ⟨ihE, ihL, ihB, ihA, ihI⟩ := ih
    refine ⟨?_, ?_, ?_, ?_, ?_⟩
    · intro c env e h
      cases e with
      | lit _ => rfl
      | move _ => rfl
      | clone _ => rfl
      | copy _ => rfl
      | deref _ => rfl
      | unbox _ => rfl
      | uncons _ => rfl
      | isZero _ => rfl
      | nonZero _ => rfl
      | predecessor _ => rfl
      | not o =>
        simp only [eval.eq_7] at h ⊢
        cases hb : eval n c env o with
        | exhausted => rw [hb] at h; exact absurd rfl h
        | _ => rw [ihE _ _ _ (by rw [hb]; exact nofun), hb]
      | box o =>
        simp only [eval.eq_9] at h ⊢
        exact ihE _ _ _ h
      | widen o =>
        simp only [eval.eq_21] at h ⊢
        cases hb : eval n c env o with
        | exhausted => rw [hb] at h; exact absurd rfl h
        | _ => rw [ihE _ _ _ (by rw [hb]; exact nofun), hb]
      | succeed o =>
        simp only [eval.eq_22] at h ⊢
        cases hb : eval n c env o with
        | exhausted => rw [hb] at h; exact absurd rfl h
        | _ => rw [ihE _ _ _ (by rw [hb]; exact nofun), hb]
      | block b =>
        simp only [eval.eq_16] at h ⊢
        exact ihB _ _ _ h
      | call callee ops prop =>
        simp only [eval.eq_10] at h ⊢
        cases hb : evalList n c env ops with
        | values vs =>
          rw [ihL _ _ _ (by rw [hb]; exact nofun), hb]
          rw [hb] at h
          simp only at h ⊢
          cases callee with
          | function i =>
            simp only at h ⊢
            rw [ihI _ _ _ (returned_ne h)]
          | runtime _ => rfl
        | exhausted => rw [hb] at h; exact absurd rfl h
        | _ => rw [ihL _ _ _ (by rw [hb]; exact nofun), hb]
      | apply holder ops prop =>
        simp only [eval.eq_11] at h ⊢
        split
        · rfl
        · rename_i target ht
          rw [ht] at h
          simp only at h ⊢
          cases target with
          | closure f caps =>
            simp only at h ⊢
            cases hb : evalList n c env ops with
            | values vs =>
              rw [ihL _ _ _ (by rw [hb]; exact nofun), hb]
              rw [hb] at h
              simp only at h ⊢
              split
              · rfl
              · rename_i d hd
                rw [hd] at h
                simp only at h ⊢
                rw [ihI _ _ _ (dispatched_ne (returned_ne h))]
            | exhausted => rw [hb] at h; exact absurd rfl h
            | _ => rw [ihL _ _ _ (by rw [hb]; exact nofun), hb]
          | _ => rfl
      | construct k ops =>
        simp only [eval.eq_12] at h ⊢
        cases hb : evalList n c env ops with
        | exhausted => rw [hb] at h; exact absurd rfl h
        | _ => rw [ihL _ _ _ (by rw [hb]; exact nofun), hb]
      | pair l r =>
        simp only [eval.eq_13] at h ⊢
        cases hb : eval n c env l with
        | value lv =>
          rw [ihE _ _ _ (by rw [hb]; exact nofun), hb]
          rw [hb] at h
          simp only at h ⊢
          cases hr : eval n c env r with
          | exhausted => rw [hr] at h; exact absurd rfl h
          | _ => rw [ihE _ _ _ (by rw [hr]; exact nofun), hr]
        | exhausted => rw [hb] at h; exact absurd rfl h
        | _ => rw [ihE _ _ _ (by rw [hb]; exact nofun), hb]
      | cond k t f =>
        simp only [eval.eq_14] at h ⊢
        cases hb : eval n c env k with
        | value kv =>
          rw [ihE _ _ _ (by rw [hb]; exact nofun), hb]
          rw [hb] at h
          cases kv with
          | bool flag =>
            cases flag
            · simp only [Bool.false_eq_true, if_false] at h ⊢
              rw [ihB _ _ _ h]
            · simp only [if_true] at h ⊢
              rw [ihB _ _ _ h]
          | _ => rfl
        | exhausted => rw [hb] at h; exact absurd rfl h
        | _ => rw [ihE _ _ _ (by rw [hb]; exact nofun), hb]
      | matchOn s arms =>
        simp only [eval.eq_15] at h ⊢
        cases hb : eval n c env s with
        | value sv =>
          rw [ihE _ _ _ (by rw [hb]; exact nofun), hb]
          rw [hb] at h
          simp only at h ⊢
          rw [ihA _ _ _ _ h]
        | exhausted => rw [hb] at h; exact absurd rfl h
        | _ => rw [ihE _ _ _ (by rw [hb]; exact nofun), hb]
    · intro c env es h
      cases es with
      | nil => rfl
      | cons e rest =>
        simp only [evalList.eq_3] at h ⊢
        cases hb : eval n c env e with
        | value v =>
          rw [ihE _ _ _ (by rw [hb]; exact nofun), hb]
          rw [hb] at h
          simp only at h ⊢
          cases hl : evalList n c env rest with
          | exhausted => rw [hl] at h; exact absurd rfl h
          | _ => rw [ihL _ _ _ (by rw [hl]; exact nofun), hl]
        | exhausted => rw [hb] at h; exact absurd rfl h
        | _ => rw [ihE _ _ _ (by rw [hb]; exact nofun), hb]
    · intro c env b h
      cases b with
      | mk lets tail =>
        cases lets with
        | nil =>
          simp only [evalBlock.eq_2] at h ⊢
          exact ihE _ _ _ h
        | cons l rest =>
          cases l with
          | mk pat ty bound =>
            simp only [evalBlock.eq_3] at h ⊢
            cases hb : eval n c env bound with
            | value v =>
              rw [ihE _ _ _ (by rw [hb]; exact nofun), hb]
              rw [hb] at h
              simp only at h ⊢
              split
              · rfl
              · rename_i ext hext
                rw [hext] at h
                simp only at h
                rw [ihB _ _ _ h]
            | exhausted => rw [hb] at h; exact absurd rfl h
            | _ => rw [ihE _ _ _ (by rw [hb]; exact nofun), hb]
    · intro c env v arms h
      cases arms with
      | nil => rfl
      | cons a rest =>
        cases a with
        | mk pat body =>
          simp only [evalArms.eq_3] at h ⊢
          split
          · rename_i hd
            rw [hd] at h
            simp only at h
            rw [ihA _ _ _ _ h]
          · rename_i ext hd
            rw [hd] at h
            simp only at h
            rw [ihB _ _ _ h]
    · intro c f args h
      simp only [invoke.eq_2] at h ⊢
      split
      · rfl
      · rename_i fn hfn
        rw [hfn] at h
        simp only at h ⊢
        split
        · rfl
        · rename_i bound hbound
          rw [hbound] at h
          simp only at h
          rw [ihB _ _ _ (finished_ne h)]

end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax

deriving instance DecidableEq for LexLeanTarget.TargetSyntax.IntKind

abbrev RExpr := RustSyntax.Expr
abbrev RLet := RustSyntax.Let
abbrev RBlock := RustSyntax.Block
abbrev RCrate := RustSyntax.Crate
abbrev RIdent := RustSyntax.Ident
abbrev ROut := RustSemantics.ROutcome

/-- The Rust identifier of a program local: `v<m>`. -/
abbrev local_ (m : Nat) : RIdent := .generated .binding m

/-- Types whose values the correspondence needs no fact about: no unit,
ADT, or function type occurs in them. -/
def inert : Ty → Bool
  | .unit => false
  | .adt _ => false
  | .fn _ _ => false
  | .option t => inert t
  | .result a b => inert a && inert b
  | .list t => inert t
  | .pair a b => inert a && inert b
  | .bool | .nat | .int | .fixed _ | .string | .bytes | .ordering => true

def adtFields (p : Program) (i k : Nat) : Option (List Ty) :=
  (p.adts[i]?).bind fun adt => adt.constructors[k]?

/-- The `apply` flag of a function type: whether its dispatch is fallible. -/
abbrev Flags := List ((List Ty × Ty) × Bool)

mutual
def tyBeq : Ty → Ty → Bool
  | .unit, .unit | .bool, .bool | .nat, .nat | .int, .int => true
  | .string, .string | .bytes, .bytes | .ordering, .ordering => true
  | .fixed a, .fixed b => decide (a = b)
  | .option a, .option b | .list a, .list b => tyBeq a b
  | .result a b, .result c d | .pair a b, .pair c d => tyBeq a c && tyBeq b d
  | .adt i, .adt j => i == j
  | .fn ps r, .fn qs s => tysBeq ps qs && tyBeq r s
  | _, _ => false
def tysBeq : List Ty → List Ty → Bool
  | [], [] => true
  | a :: as, b :: bs => tyBeq a b && tysBeq as bs
  | _, _ => false
end

def flagOf (A : Flags) (ps : List Ty) (r : Ty) : Option Bool :=
  (A.find? fun entry => tysBeq entry.1.1 ps && tyBeq entry.1.2 r).map (·.2)

theorem listIndex_eq {α : Type} : ∀ (xs : List α) (i : Nat),
    TargetSemantics.LexLeanRuntime.listIndex xs i = xs[i]?
  | [], _ => rfl
  | _ :: _, 0 => rfl
  | _ :: xs, i + 1 => listIndex_eq xs i

theorem index_eq {α : Type} (xs : List α) (i : Nat) :
    TargetSemantics.LexLeanRuntime.index xs i = xs[i]? := listIndex_eq xs i

def fnIdent (f : Nat) : RIdent := .generated .function f

def fnFallible (c : RCrate) (f : Nat) : Option Bool :=
  (RustSemantics.findFunction c.items (fnIdent f)).map fun fn => RustSemantics.fallibleType fn.2.1

mutual
/-- Value typing: what the correspondence knows of a value of a type. -/
def WT (p : Program) (c : RCrate) (A : Flags) : Value → Ty → Prop
  | .unit, .unit => True
  | .none, .option _ => True
  | .some v, .option t => WT p c A v t
  | .ok v, .result a _ => WT p c A v a
  | .error v, .result _ b => WT p c A v b
  | .list vs, .list t => WTAll p c A vs t
  | .pair a b, .pair s t => WT p c A a s ∧ WT p c A b t
  | .adt k fs, .adt i => ∃ tys, adtFields p i k = some tys ∧ WTL p c A fs tys
  | .closure f cs, .fn ps r => ∃ fn capTys af, p.functions[f]? = some fn ∧
      fn.types = capTys ++ ps ∧ fn.result = r ∧ WTL p c A cs capTys ∧
      flagOf A ps r = some af ∧
      ∃ ff, RustSemantics.findDispatch c.items f cs.length = some (af, ff) ∧
        fnFallible c f = some ff ∧ (!ff || af) = true
  | _, t => inert t = true
def WTAll (p : Program) (c : RCrate) (A : Flags) : List Value → Ty → Prop
  | [], _ => True
  | v :: vs, t => WT p c A v t ∧ WTAll p c A vs t
def WTL (p : Program) (c : RCrate) (A : Flags) : List Value → List Ty → Prop
  | [], [] => True
  | v :: vs, t :: ts => WT p c A v t ∧ WTL p c A vs ts
  | _, _ => False
end

/-- A scope: each program local with its type and the Rust local `v<m>`
holding it, innermost first; `none` when the rendering holds no binding
(a unit, or a local the body never reads). -/
abbrev Ctx := List (Nat × Ty × Option Nat)

def ctxLookup : Ctx → Nat → Option (Ty × Option Nat)
  | [], _ => none
  | (y, t, s) :: rest, x => if y == x then some (t, s) else ctxLookup rest x

/-- Whether no local of the scope is held by `v<m>`. -/
def slotFree (Γ : Ctx) (m : Nat) : Bool := Γ.all fun entry => entry.2.2 != some m

def EnvRel (p : Program) (c : RCrate) (A : Flags) (Γ : Ctx)
    (env : List (Nat × Value)) (renv : List (RIdent × Value)) : Prop :=
  ∀ x t s, ctxLookup Γ x = some (t, s) →
    ∃ v, TargetSemantics.lookup env x = some v ∧ WT p c A v t ∧
      ∀ m, s = some m → RustSemantics.lookup renv (local_ m) = some v

theorem sameIdent_self (i : RIdent) : RustSemantics.sameIdent i i = true := by
  cases i with
  | generated k n => cases k <;> simp [RustSemantics.sameIdent, RustSemantics.kindNumber]
  | exported s => simp [RustSemantics.sameIdent, LexLeanTarget.RustSemantics.LexLeanRuntime.equal]

theorem rlookup_cons_self (i : RIdent) (v : Value) (renv) :
    RustSemantics.lookup ((i, v) :: renv) i = some v := by
  simp [RustSemantics.lookup, sameIdent_self]

theorem sameIdent_kind {k k' : RustSyntax.IdentKind} {a b : Nat} (h : k ≠ k') :
    RustSemantics.sameIdent (.generated k a) (.generated k' b) = false := by
  cases k <;> cases k' <;> simp_all [RustSemantics.sameIdent, RustSemantics.kindNumber]

theorem sameIdent_local {a b : Nat} (h : a ≠ b) :
    RustSemantics.sameIdent (local_ a) (local_ b) = false := by
  simp only [RustSemantics.sameIdent, RustSemantics.kindNumber, Nat.beq_refl, Bool.true_and]
  cases hb : a.beq b
  · rfl
  · exact absurd (Nat.eq_of_beq_eq_true hb) h

theorem rlookup_cons_other {i j : RIdent} {v : Value} {renv}
    (h : RustSemantics.sameIdent i j = false) :
    RustSemantics.lookup ((i, v) :: renv) j = RustSemantics.lookup renv j := by
  simp [RustSemantics.lookup, h]

/-- A temporary of another kind than a local leaves the relation intact. -/
theorem envRel_temp {p c A Γ env renv} (k : RustSyntax.IdentKind) (j : Nat) (w : Value)
    (hk : k ≠ .binding) (h : EnvRel p c A Γ env renv) :
    EnvRel p c A Γ env ((.generated k j, w) :: renv) := by
  intro x t s hx
  obtain ⟨v, hv, hw, hr⟩ := h x t s hx
  refine ⟨v, hv, hw, fun m hm => ?_⟩
  rw [rlookup_cons_other (sameIdent_kind hk)]
  exact hr m hm

theorem ctxLookup_slot {Γ : Ctx} {x t m} (h : ctxLookup Γ x = some (t, some m)) :
    slotFree Γ m = false := by
  induction Γ with
  | nil => simp [ctxLookup] at h
  | cons e rest ih =>
    obtain ⟨y, t', s'⟩ := e
    simp only [ctxLookup] at h
    split at h
    · simp only [Option.some.injEq, Prod.mk.injEq] at h
      obtain ⟨_, rfl⟩ := h
      simp [slotFree]
    · have := ih h
      simp only [slotFree, List.all_cons, Bool.and_eq_false_iff] at this ⊢
      exact Or.inr this

/-- A local bound in both worlds, held by a slot no other local holds. -/
theorem envRel_bind {p c A Γ env renv} (x : Nat) (t : Ty) (m : Nat) (v : Value)
    (hfree : slotFree Γ m = true) (hv : WT p c A v t) (h : EnvRel p c A Γ env renv) :
    EnvRel p c A ((x, t, some m) :: Γ) ((x, v) :: env) ((local_ m, v) :: renv) := by
  intro y t' s hy
  simp only [ctxLookup] at hy
  by_cases hxy : x = y
  · subst hxy
    simp only [beq_self_eq_true, if_true, Option.some.injEq, Prod.mk.injEq] at hy
    obtain ⟨rfl, rfl⟩ := hy
    refine ⟨v, by simp [TargetSemantics.lookup], hv, fun m' hm' => ?_⟩
    cases hm'
    exact rlookup_cons_self _ _ _
  · have hne : (x == y) = false := by simp [hxy]
    simp only [hne, Bool.false_eq_true, if_false] at hy
    obtain ⟨w, hw, hwt, hr⟩ := h y t' s hy
    refine ⟨w, ?_, hwt, fun m' hm' => ?_⟩
    · simp [TargetSemantics.lookup, hxy, hw]
    · subst hm'
      have hm'm : m' ≠ m := by
        intro e; subst e
        have := ctxLookup_slot hy
        rw [hfree] at this; exact absurd this (by decide)
      rw [rlookup_cons_other (sameIdent_local (Ne.symm hm'm))]
      exact hr _ rfl

/-- A local the rendering holds no binding for. -/
theorem envRel_ghost {p c A Γ env renv} (x : Nat) (t : Ty) (v : Value)
    (hv : WT p c A v t) (h : EnvRel p c A Γ env renv) :
    EnvRel p c A ((x, t, none) :: Γ) ((x, v) :: env) renv := by
  intro y t' s hy
  simp only [ctxLookup] at hy
  by_cases hxy : x = y
  · subst hxy
    simp only [beq_self_eq_true, if_true, Option.some.injEq, Prod.mk.injEq] at hy
    obtain ⟨rfl, rfl⟩ := hy
    exact ⟨v, by simp [TargetSemantics.lookup], hv, fun _ hm => by cases hm⟩
  · have hne : (x == y) = false := by simp [hxy]
    simp only [hne, Bool.false_eq_true, if_false] at hy
    obtain ⟨w, hw, hwt, hr⟩ := h y t' s hy
    exact ⟨w, by simp [TargetSemantics.lookup, hxy, hw], hwt, hr⟩

/-- A Rust local no scope entry holds may be bound freely. -/
theorem envRel_extra {p c A Γ env renv} (m : Nat) (w : Value) (hfree : slotFree Γ m = true)
    (h : EnvRel p c A Γ env renv) : EnvRel p c A Γ env ((local_ m, w) :: renv) := by
  intro x t s hx
  obtain ⟨v, hv, hw, hr⟩ := h x t s hx
  refine ⟨v, hv, hw, fun m' hm' => ?_⟩
  subst hm'
  have hm'm : m' ≠ m := by
    intro e; subst e
    have := ctxLookup_slot hx
    rw [hfree] at this; exact absurd this (by decide)
  rw [rlookup_cons_other (sameIdent_local (Ne.symm hm'm))]
  exact hr _ rfl

end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

abbrev REnv := List (RIdent × Value)

theorem reval_le {n m : Nat} {c renv e} (h : RustSemantics.eval n c renv e ≠ .exhausted) (hle : n ≤ m) :
    RustSemantics.eval m c renv e = RustSemantics.eval n c renv e := by
  induction hle with
  | refl => rfl
  | step _ ih => rw [(stable _).1 _ _ _ (by rw [ih]; exact h), ih]
theorem revalList_le {n m : Nat} {c renv es} (h : RustSemantics.evalList n c renv es ≠ .exhausted)
    (hle : n ≤ m) : RustSemantics.evalList m c renv es = RustSemantics.evalList n c renv es := by
  induction hle with
  | refl => rfl
  | step _ ih => rw [(stable _).2.1 _ _ _ (by rw [ih]; exact h), ih]
theorem revalBlock_le {n m : Nat} {c renv b} (h : RustSemantics.evalBlock n c renv b ≠ .exhausted)
    (hle : n ≤ m) : RustSemantics.evalBlock m c renv b = RustSemantics.evalBlock n c renv b := by
  induction hle with
  | refl => rfl
  | step _ ih => rw [(stable _).2.2.1 _ _ _ (by rw [ih]; exact h), ih]
theorem revalArms_le {n m : Nat} {c renv v arms}
    (h : RustSemantics.evalArms n c renv v arms ≠ .exhausted) (hle : n ≤ m) :
    RustSemantics.evalArms m c renv v arms = RustSemantics.evalArms n c renv v arms := by
  induction hle with
  | refl => rfl
  | step _ ih => rw [(stable _).2.2.2.1 _ _ _ _ (by rw [ih]; exact h), ih]
theorem invoke_le {n m : Nat} {c f args} (h : RustSemantics.invoke n c f args ≠ .exhausted)
    (hle : n ≤ m) : RustSemantics.invoke m c f args = RustSemantics.invoke n c f args := by
  induction hle with
  | refl => rfl
  | step _ ih => rw [(stable _).2.2.2.2 _ _ _ (by rw [ih]; exact h), ih]

/-- Rust convergence: some fuel gives the outcome, which is never
exhaustion. -/
def RC (c : RCrate) (renv : REnv) (e : RExpr) (o : ROut) : Prop :=
  o ≠ .exhausted ∧ ∃ n, RustSemantics.eval n c renv e = o
def RCL (c : RCrate) (renv : REnv) (es : List RExpr) (o : ROutcomes) : Prop :=
  o ≠ .exhausted ∧ ∃ n, RustSemantics.evalList n c renv es = o
def RCB (c : RCrate) (renv : REnv) (b : RBlock) (o : ROut) : Prop :=
  o ≠ .exhausted ∧ ∃ n, RustSemantics.evalBlock n c renv b = o
def RCA (c : RCrate) (renv : REnv) (w : Value) (arms : List RustSyntax.Arm) (o : ROut) : Prop :=
  o ≠ .exhausted ∧ ∃ n, RustSemantics.evalArms n c renv w arms = o
def RCI (c : RCrate) (f : RIdent) (args : List Value) (o : ROut) : Prop :=
  o ≠ .exhausted ∧ ∃ n, RustSemantics.invoke n c f args = o

theorem RC.at {c renv e o} (h : RC c renv e o) : ∃ n, ∀ m, n ≤ m → RustSemantics.eval m c renv e = o := by
  obtain ⟨hne, n, hn⟩ := h
  exact ⟨n, fun m hm => by rw [reval_le (by rw [hn]; exact hne) hm, hn]⟩
theorem RCL.at {c renv es o} (h : RCL c renv es o) :
    ∃ n, ∀ m, n ≤ m → RustSemantics.evalList m c renv es = o := by
  obtain ⟨hne, n, hn⟩ := h
  exact ⟨n, fun m hm => by rw [revalList_le (by rw [hn]; exact hne) hm, hn]⟩
theorem RCB.at {c renv b o} (h : RCB c renv b o) :
    ∃ n, ∀ m, n ≤ m → RustSemantics.evalBlock m c renv b = o := by
  obtain ⟨hne, n, hn⟩ := h
  exact ⟨n, fun m hm => by rw [revalBlock_le (by rw [hn]; exact hne) hm, hn]⟩
theorem RCI.at {c f args o} (h : RCI c f args o) :
    ∃ n, ∀ m, n ≤ m → RustSemantics.invoke m c f args = o := by
  obtain ⟨hne, n, hn⟩ := h
  exact ⟨n, fun m hm => by rw [invoke_le (by rw [hn]; exact hne) hm, hn]⟩

/-- Binding a block's lets in order succeeds and ends in `renv'`. -/
inductive LetsOK (c : RCrate) : REnv → List RLet → REnv → Prop where
  | nil {renv} : LetsOK c renv [] renv
  | cons {renv renv1 renv2 pat ty e v ls} :
      RC c renv e (.value v) → RustSemantics.bindPattern pat v renv = some renv1 →
      LetsOK c renv1 ls renv2 → LetsOK c renv (.mk pat ty e :: ls) renv2

/-- An outcome that ends evaluation early: a raise or an abort. -/
def Halt (o : ROut) : Prop := o = .raise ∨ o = .abort

/-- Binding a block's lets halts with `h`. -/
inductive LetsRaise (c : RCrate) : REnv → List RLet → ROut → Prop where
  | here {renv pat ty e ls h} : RC c renv e h → Halt h → LetsRaise c renv (.mk pat ty e :: ls) h
  | later {renv renv1 pat ty e v ls h} :
      RC c renv e (.value v) → RustSemantics.bindPattern pat v renv = some renv1 →
      LetsRaise c renv1 ls h → LetsRaise c renv (.mk pat ty e :: ls) h

theorem block_of_lets {c renv lets renv' tail o} (hl : LetsOK c renv lets renv')
    (ht : RC c renv' tail o) : RCB c renv ⟨lets, tail⟩ o := by
  induction hl with
  | nil =>
    obtain ⟨n, hn⟩ := ht.at
    exact ⟨ht.1, n+1, by rw [RustSemantics.evalBlock.eq_2]; exact hn n (Nat.le_refl _)⟩
  | @cons renv renv1 renv2 pat ty e v ls he hb _ ih =>
    obtain ⟨n1, hn1⟩ := he.at
    obtain ⟨n2, hn2⟩ := (ih ht).at
    refine ⟨ht.1, max n1 n2 + 1, ?_⟩
    rw [RustSemantics.evalBlock.eq_3, hn1 _ (Nat.le_max_left _ _)]
    simp only [hb]
    exact hn2 _ (Nat.le_max_right _ _)

theorem block_of_raise {c renv lets tail h} (hl : LetsRaise c renv lets h) :
    RCB c renv ⟨lets, tail⟩ h := by
  induction hl with
  | @here renv pat ty e ls h he hh =>
    obtain ⟨n, hn⟩ := he.at
    refine ⟨he.1, n+1, ?_⟩
    rw [RustSemantics.evalBlock.eq_3, hn n (Nat.le_refl _)]
    rcases hh with rfl | rfl <;> rfl
  | @later renv renv1 pat ty e v ls h he hb _ ih =>
    obtain ⟨n1, hn1⟩ := he.at
    obtain ⟨n2, hn2⟩ := ih.at
    refine ⟨ih.1, max n1 n2 + 1, ?_⟩
    rw [RustSemantics.evalBlock.eq_3, hn1 _ (Nat.le_max_left _ _)]
    simp only [hb]
    exact hn2 _ (Nat.le_max_right _ _)

theorem RC.block {c renv b o} (h : RCB c renv b o) : RC c renv (.block b) o := by
  obtain ⟨hne, n, hn⟩ := h
  exact ⟨hne, n+1, by rw [RustSemantics.eval.eq_16]; exact hn⟩

theorem LetsOK.append {c renv l1 renv1 l2 renv2} (h1 : LetsOK c renv l1 renv1)
    (h2 : LetsOK c renv1 l2 renv2) : LetsOK c renv (l1 ++ l2) renv2 := by
  induction h1 with
  | nil => exact h2
  | cons he hb _ ih => exact .cons he hb (ih h2)

theorem LetsRaise.append_left {c renv l1 l2 h} (hl : LetsRaise c renv l1 h) :
    LetsRaise c renv (l1 ++ l2) h := by
  induction hl with
  | here he hh => exact .here he hh
  | later he hb _ ih => exact .later he hb ih

theorem LetsRaise.append_right {c renv l1 renv1 l2 h} (h1 : LetsOK c renv l1 renv1)
    (h2 : LetsRaise c renv1 l2 h) : LetsRaise c renv (l1 ++ l2) h := by
  induction h1 with
  | nil => exact h2
  | cons he hb _ ih => exact .later he hb (ih h2)

theorem rcl_nil {c renv} : RCL c renv [] (.values []) := ⟨nofun, 1, rfl⟩

theorem rcl_cons {c renv e es v vs} (h : RC c renv e (.value v)) (hs : RCL c renv es (.values vs)) :
    RCL c renv (e :: es) (.values (v :: vs)) := by
  obtain ⟨n1, hn1⟩ := h.at
  obtain ⟨n2, hn2⟩ := hs.at
  refine ⟨nofun, max n1 n2 + 1, ?_⟩
  rw [RustSemantics.evalList.eq_3, hn1 _ (Nat.le_max_left _ _), hn2 _ (Nat.le_max_right _ _)]

/-- The list outcome a halting element gives. -/
def haltL : ROut → ROutcomes
  | .abort => .abort
  | _ => .raise

theorem rcl_raise_head {c renv e es h} (hr : RC c renv e h) (hh : Halt h) :
    RCL c renv (e :: es) (haltL h) := by
  obtain ⟨n1, hn1⟩ := hr.at
  refine ⟨by rcases hh with rfl | rfl <;> exact nofun, n1 + 1, ?_⟩
  rw [RustSemantics.evalList.eq_3, hn1 _ (Nat.le_refl _)]
  rcases hh with rfl | rfl <;> rfl

theorem rcl_raise_tail {c renv e es v o} (h : RC c renv e (.value v)) (hs : RCL c renv es o)
    (ho : o = .raise ∨ o = .abort) : RCL c renv (e :: es) o := by
  obtain ⟨n1, hn1⟩ := h.at
  obtain ⟨n2, hn2⟩ := hs.at
  refine ⟨hs.1, max n1 n2 + 1, ?_⟩
  rw [RustSemantics.evalList.eq_3, hn1 _ (Nat.le_max_left _ _), hn2 _ (Nat.le_max_right _ _)]
  rcases ho with rfl | rfl <;> rfl

end LexLeanPreservation.Rust
