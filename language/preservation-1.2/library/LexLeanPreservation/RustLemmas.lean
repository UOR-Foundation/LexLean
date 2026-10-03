import LexLeanPreservation.RustCorr
set_option linter.unusedSimpArgs false

/-! Soundness of the correspondence's rules for values, operands, calls,
primitives, constructions, lets, conditionals, and the matches a holder
dispatches on: one lemma per rule, each from the semantics of its
premises. -/

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

/-! Small facts. -/

theorem rc_read {c renv r i v} (hr : readOf r = some i) (hl : RustSemantics.lookup renv i = some v) :
    RC c renv r (.value v) := by
  refine ⟨nofun, 1, ?_⟩
  cases r <;> simp [readOf] at hr <;> subst hr <;>
    simp [RustSemantics.eval.eq_3, RustSemantics.eval.eq_4, RustSemantics.eval.eq_5,
      RustSemantics.eval.eq_6, RustSemantics.eval.eq_8, hl]

theorem rc_lit {c renv l v} (h : RustSemantics.literal l = some v) : RC c renv (.lit l) (.value v) :=
  ⟨nofun, 1, by simp [RustSemantics.eval.eq_2, h]⟩

theorem wt_unit {p c A v} (h : WT p c A v .unit) : v = .unit := by
  cases v <;> simp_all [WT, inert]

theorem fresh_nil {Γ} : Fresh Γ [] := fun _ _ h => by simp at h

theorem obs_eval_zero {p env e} : obs (TargetSemantics.eval 0 p env e) = none := rfl
theorem obsL_evalList_zero {p env es} : obsL (TargetSemantics.evalList 0 p env es) = none := rfl

theorem realizes_halt {fl ro} (h : Realizes fl .overflow ro) : Halt ro := by
  cases ro <;> simp [Realizes] at h <;> simp [Halt]

theorem realizesL_halt {fl ro} (h : RealizesL fl .overflow ro) : ro = .raise ∨ ro = .abort := by
  cases ro <;> simp [RealizesL] at h <;> simp

theorem realizesL_haltL {fl h} (hr : Realizes fl .overflow h) : RealizesL fl .overflow (haltL h) := by
  cases h <;> simp_all [Realizes, RealizesL, haltL]

theorem sem_zero {p c A Γ fl} (j : Jd) : Sem p c A 0 Γ fl j := by
  cases j with
  | k => intro env renv _ β env' _ o ho; simp [obs_eval_zero] at ho
  | m => intro env renv _ v _ j shape xs body _ _ fields env' _ _ o ho; simp [obs_eval_zero] at ho
  | _ => intro env renv _ o ho; simp [obs_eval_zero, obsL_evalList_zero] at ho

/-! Function parameters. -/

theorem param_env {p c A} : ∀ (acc : Ctx) (xs : List Nat) (ts : List Ty) (qs : List RustSyntax.Pat)
    (Γ : Ctx) (args : List Value) (env env' : List (Nat × Value)) (renv : REnv),
    paramCtx acc xs ts qs = some Γ → EnvRel p c A acc env renv → WTL p c A args ts →
    TargetSemantics.bindAll xs args env = some env' →
    ∃ renv', RustSemantics.bindPatterns qs args renv = some renv' ∧ EnvRel p c A Γ env' renv'
  | acc, [], [], [], Γ, args, env, env', renv, hp, he, hw, hb => by
    simp only [paramCtx, Option.some.injEq] at hp; subst hp
    cases args with
    | nil =>
      simp [TargetSemantics.bindAll.eq_1] at hb; subst hb
      exact ⟨renv, by simp [RustSemantics.bindPatterns.eq_1], he⟩
    | cons _ _ => simp [TargetSemantics.bindAll.eq_2] at hb
  | acc, x :: xs, t :: ts, q :: qs, Γ, args, env, env', renv, hp, he, hw, hb => by
    cases args with
    | nil => simp [WTL] at hw
    | cons v vs =>
      simp only [WTL] at hw
      simp only [TargetSemantics.bindAll] at hb
      cases q with
      | wild =>
        simp only [paramCtx] at hp
        obtain ⟨renv', h1, h2⟩ := param_env _ xs ts qs Γ vs _ _ renv hp
          (envRel_ghost x t v hw.1 he) hw.2 hb
        exact ⟨renv', by simpa [RustSemantics.bindPatterns, RustSemantics.bindPattern.eq_1] using h1, h2⟩
      | bind i =>
        cases i with
        | generated k m =>
          cases k with
          | binding =>
            simp only [paramCtx] at hp
            split at hp
            · rename_i hfree
              obtain ⟨renv', h1, h2⟩ := param_env _ xs ts qs Γ vs _ _ _ hp
                (envRel_bind x t m v hfree hw.1 he) hw.2 hb
              exact ⟨renv', by simpa [RustSemantics.bindPatterns, RustSemantics.bindPattern.eq_2] using h1, h2⟩
            · simp at hp
          | _ => simp [paramCtx] at hp
        | exported _ => simp [paramCtx] at hp
      | _ => simp [paramCtx] at hp
  | _, [], [], _ :: _, _, _, _, _, _, hp, _, _, _ => by simp [paramCtx] at hp
  | _, [], _ :: _, _, _, _, _, _, _, hp, _, _, _ => by simp [paramCtx] at hp
  | _, _ :: _, [], _, _, _, _, _, _, hp, _, _, _ => by simp [paramCtx] at hp
  | _, _ :: _, _ :: _, [], _, _, _, _, _, hp, _, _, _ => by simp [paramCtx] at hp

end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

section rules
variable {p : Program} {c : RCrate} {A : Flags}

theorem sem_var {n Γ fl x t m r} (hx : ctxLookup Γ x = some (t, some m))
    (hr : readOf r = some (local_ m)) : Sem p c A (n+1) Γ fl (.e (.var x) t r) := by
  intro env renv he o ho hs
  obtain ⟨v, hv, hwt, hrl⟩ := he x t _ hx
  simp only [TargetSemantics.eval.eq_3, hv, obs, Option.some.injEq] at ho
  subst ho
  refine ⟨(fun w hw => by cases hw; exact hwt), .value v, rfl, rc_read hr (hrl m rfl)⟩

theorem sem_varUnit {n Γ fl x s} (hx : ctxLookup Γ x = some (.unit, s)) :
    Sem p c A (n+1) Γ fl (.e (.var x) .unit (.lit .unit)) := by
  intro env renv he o ho hs
  obtain ⟨v, hv, hwt, _⟩ := he x .unit s hx
  simp only [TargetSemantics.eval.eq_3, hv, obs, Option.some.injEq] at ho
  subst ho
  have := wt_unit hwt; subst this
  exact ⟨(fun w hw => by cases hw; exact hwt), .value .unit, rfl, rc_lit rfl⟩

theorem sem_bOfE {n Γ fl e t r} (h : Sem p c A n Γ fl (.e e t r)) : Sem p c A n Γ fl (.b false e t [] r) := by
  intro env renv he o ho hs
  obtain ⟨hw, ro, hro, hrc⟩ := h env renv he o ho hs
  exact ⟨hw, .inl ⟨[], fresh_nil, .nil, ro, hro, by simpa using hrc⟩⟩

theorem sem_eOfB {n Γ fl e t lets tail} (h : Sem p c A n Γ fl (.b false e t lets tail)) :
    Sem p c A n Γ fl (.e e t (.block ⟨lets, tail⟩)) := by
  intro env renv he o ho hs
  obtain ⟨hw, hcase⟩ := h env renv he o ho hs
  refine ⟨hw, ?_⟩
  rcases hcase with ⟨new, _, hl, ro, hro, hrc⟩ | ⟨rfl, hh, hr, hraise⟩
  · exact ⟨ro, hro, RC.block (block_of_lets hl hrc)⟩
  · exact ⟨hh, hr, RC.block (block_of_raise hraise)⟩

theorem sem_lNil {n Γ fl} : Sem p c A (n+1) Γ fl (.l [] [] []) := by
  intro env renv he o ho hs
  simp only [TargetSemantics.evalList.eq_2, obsL, Option.some.injEq] at ho
  subst ho
  exact ⟨(fun vs h => by cases h; simp [WTL]), .values [], rfl, rcl_nil⟩

theorem sem_lCons {n Γ fl e es t ts r rs} (he : Sem p c A n Γ fl (.e e t r))
    (hes : Sem p c A n Γ fl (.l es ts rs)) :
    Sem p c A (n+1) Γ fl (.l (e :: es) (t :: ts) (r :: rs)) := by
  intro env renv hrel o ho hs
  simp only [TargetSemantics.evalList.eq_3] at ho
  cases h1 : TargetSemantics.eval n p env e with
  | value v s1 =>
    rw [h1] at ho
    simp only at ho
    obtain ⟨hw1, ro1, hro1, hrc1⟩ := he env renv hrel (.value v) (by rw [h1]; rfl) nofun
    cases ro1 with
    | value w =>
      simp only [Realizes] at hro1; subst hro1
      cases h2 : TargetSemantics.evalList n p env es with
      | values vs s2 =>
        rw [h2] at ho
        simp only [obsL, Option.some.injEq] at ho; subst ho
        obtain ⟨hw2, ro2, hro2, hrc2⟩ := hes env renv hrel (.values vs) (by rw [h2]; rfl) nofun
        cases ro2 with
        | values ws =>
          simp only [RealizesL] at hro2; subst hro2
          exact ⟨(fun xs hx => by cases hx; exact ⟨hw1 _ rfl, hw2 _ rfl⟩), .values (w :: ws), rfl,
            rcl_cons hrc1 hrc2⟩
        | _ => simp [RealizesL] at hro2
      | overflow s2 =>
        rw [h2] at ho
        simp only [obsL, Option.some.injEq] at ho; subst ho
        obtain ⟨_, ro2, hro2, hrc2⟩ := hes env renv hrel .overflow (by rw [h2]; rfl) nofun
        exact ⟨(fun _ h => by cases h), ro2, hro2, rcl_raise_tail hrc1 hrc2 (realizesL_halt hro2)⟩
      | stuck => rw [h2] at ho; simp [obsL] at ho; subst ho; exact absurd rfl hs
      | exhausted => rw [h2] at ho; simp [obsL] at ho
    | _ => simp [Realizes] at hro1
  | overflow s1 =>
    rw [h1] at ho
    simp only [obsL, Option.some.injEq] at ho; subst ho
    obtain ⟨_, ro1, hro1, hrc1⟩ := he env renv hrel .overflow (by rw [h1]; rfl) nofun
    exact ⟨(fun _ h => by cases h), haltL ro1, realizesL_haltL hro1,
      rcl_raise_head hrc1 (realizes_halt hro1)⟩
  | stuck => rw [h1] at ho; simp [obsL] at ho; subst ho; exact absurd rfl hs
  | exhausted => rw [h1] at ho; simp [obsL] at ho

end rules
end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

/-- What runtime items do, read off their declaration. -/
def itemResult (fallible : Bool) : TargetSemantics.Outcome → ROut
  | .value w _ => if fallible then .value (.ok w) else .value w
  | .overflow _ => if fallible then .value (.error .unit) else .abort
  | .stuck => .stuck
  | .exhausted => .stuck

theorem itemOperands_eq {item : RustSyntax.Item} (h : item ≠ .natSucc) (vs : List Value) :
    RustSemantics.itemOperands item vs = vs := by
  cases item <;> simp_all [RustSemantics.itemOperands]

theorem runItem_eq {c : RCrate} {item vs} (hok : itemOK c item = true) (hs : item ≠ .natSucc)
    (hacc : RustSemantics.itemAccepts item vs = true) :
    RustSemantics.runItem c.profile item vs =
      itemResult (RustSemantics.itemFallible item)
        (TargetSemantics.primitive (RustSemantics.itemPrimitive item) vs) := by
  unfold RustSemantics.runItem
  simp only [hacc, if_true]
  rw [itemOperands_eq hs vs]
  cases hp : c.profile with
  | core =>
    simp only [itemOK, hp, Bool.not_eq_true'] at hok
    simp only [hok, Bool.false_eq_true, if_false]
    cases TargetSemantics.primitive (RustSemantics.itemPrimitive item) vs <;> simp [itemResult]
  | std =>
    cases TargetSemantics.primitive (RustSemantics.itemPrimitive item) vs <;> simp [itemResult]

/-! Widths: a width-indexed item's guard holds on well-typed operands, and
a primitive whose value has a fixed-width type gives a value of that
width. -/

mutual
theorem tyBeq_eq : ∀ a b : Ty, tyBeq a b = true → a = b
  | .unit, b, h | .bool, b, h | .nat, b, h | .int, b, h | .string, b, h | .bytes, b, h
  | .ordering, b, h => by cases b <;> simp_all [tyBeq]
  | .fixed k, b, h => by cases b <;> simp_all [tyBeq]
  | .option a, b, h => by
    cases b <;> simp [tyBeq] at h
    rw [tyBeq_eq a _ h]
  | .list a, b, h => by
    cases b <;> simp [tyBeq] at h
    rw [tyBeq_eq a _ h]
  | .result a1 a2, b, h => by
    cases b <;> simp [tyBeq] at h
    rw [tyBeq_eq a1 _ h.1, tyBeq_eq a2 _ h.2]
  | .pair a1 a2, b, h => by
    cases b <;> simp [tyBeq] at h
    rw [tyBeq_eq a1 _ h.1, tyBeq_eq a2 _ h.2]
  | .adt i, b, h => by cases b <;> simp_all [tyBeq]
  | .fn ps r, b, h => by
    cases b <;> simp [tyBeq] at h
    rw [tysBeq_eq ps _ h.1, tyBeq_eq r _ h.2]
theorem tysBeq_eq : ∀ a b : List Ty, tysBeq a b = true → a = b
  | [], b, h => by cases b <;> simp_all [tysBeq]
  | a :: as, b, h => by
    cases b <;> simp [tysBeq] at h
    rw [tyBeq_eq a _ h.1, tysBeq_eq as _ h.2]
end

theorem hasWidth_u8 {v : Value} (h : RustSemantics.hasWidth .u8 v = true) : ∃ x, v = .u8 x := by
  cases v <;> simp_all [RustSemantics.hasWidth]
theorem hasWidth_u16 {v : Value} (h : RustSemantics.hasWidth .u16 v = true) : ∃ x, v = .u16 x := by
  cases v <;> simp_all [RustSemantics.hasWidth]
theorem hasWidth_u32 {v : Value} (h : RustSemantics.hasWidth .u32 v = true) : ∃ x, v = .u32 x := by
  cases v <;> simp_all [RustSemantics.hasWidth]
theorem hasWidth_u64 {v : Value} (h : RustSemantics.hasWidth .u64 v = true) : ∃ x, v = .u64 x := by
  cases v <;> simp_all [RustSemantics.hasWidth]
theorem hasWidth_i8 {v : Value} (h : RustSemantics.hasWidth .i8 v = true) : ∃ x, v = .i8 x := by
  cases v <;> simp_all [RustSemantics.hasWidth]
theorem hasWidth_i16 {v : Value} (h : RustSemantics.hasWidth .i16 v = true) : ∃ x, v = .i16 x := by
  cases v <;> simp_all [RustSemantics.hasWidth]
theorem hasWidth_i32 {v : Value} (h : RustSemantics.hasWidth .i32 v = true) : ∃ x, v = .i32 x := by
  cases v <;> simp_all [RustSemantics.hasWidth]
theorem hasWidth_i64 {v : Value} (h : RustSemantics.hasWidth .i64 v = true) : ∃ x, v = .i64 x := by
  cases v <;> simp_all [RustSemantics.hasWidth]

theorem wt_fixed {p c A} {v : Value} {w : IntKind} :
    WT p c A v (.fixed w) ↔ RustSemantics.hasWidth w v = true := by
  cases v <;> simp [WT]

theorem allWidth_of {p c A w} : ∀ (vs : List Value) (ts : List Ty),
    (ts.all fun t => tyBeq t (.fixed w)) = true → WTL p c A vs ts → RustSemantics.allWidth w vs = true
  | [], [], _, _ => by simp [RustSemantics.allWidth]
  | [], _ :: _, _, h => by simp [WTL] at h
  | _ :: _, [], _, h => by simp [WTL] at h
  | v :: vs, t :: ts, ht, hw => by
    simp only [List.all_cons, Bool.and_eq_true] at ht
    simp only [WTL] at hw
    rw [tyBeq_eq _ _ ht.1] at hw
    simp only [RustSemantics.allWidth, wt_fixed.mp hw.1, if_true]
    exact allWidth_of vs ts ht.2 hw.2

theorem wtAll_iff {p c A} {t : Ty} : ∀ vs : List Value, WTAll p c A vs t ↔ ∀ v ∈ vs, WT p c A v t
  | [] => by simp [WTAll]
  | v :: vs => by simp [WTAll, wtAll_iff vs]

theorem wt_list_inv {p c A} {v : Value} {e : Ty} (h : WT p c A v (.list e)) :
    ∃ xs, v = .list xs ∧ WTAll p c A xs e := by
  cases v <;> simp [WT, inert] at h
  exact ⟨_, rfl, h⟩

theorem wt_bytes_inv {p c A} {v : Value} (h : WT p c A v .bytes) : ∃ b, v = .bytes b := by
  cases v <;> simp [WT, inert] at h
  exact ⟨_, rfl⟩

theorem wt_string_inv {p c A} {v : Value} (h : WT p c A v .string) : ∃ b, v = .string b := by
  cases v <;> simp [WT, inert] at h
  exact ⟨_, rfl⟩

theorem isSeq_eq {a : Ty} (h : isSeq a = true) : (∃ e, a = .list e) ∨ a = .bytes := by
  cases a <;> simp_all [isSeq]

theorem wtl_three {p c A} {vs : List Value} {a b d : Ty} (h : WTL p c A vs [a, b, d]) :
    ∃ x y z, vs = [x, y, z] ∧ WT p c A x a := by
  match vs, h with
  | [x, y, z], h => simp only [WTL] at h; exact ⟨x, y, z, rfl, h.1⟩

theorem takes_of_typed {p c A item t v} (h : RustSemantics.itemTakes item (shapeOf t) = true)
    (hw : WT p c A v t) : RustSemantics.itemTakes item v = true := by
  cases t
  case list e =>
    obtain ⟨xs, rfl, -⟩ := wt_list_inv hw
    cases item <;> simp_all [RustSemantics.itemTakes, RustSemantics.isList,
      RustSemantics.isBytes, RustSemantics.isString, shapeOf]
  case bytes =>
    obtain ⟨b, rfl⟩ := wt_bytes_inv hw
    cases item <;> simp_all [RustSemantics.itemTakes, RustSemantics.isList,
      RustSemantics.isBytes, RustSemantics.isString, shapeOf]
  case string =>
    obtain ⟨b, rfl⟩ := wt_string_inv hw
    cases item <;> simp_all [RustSemantics.itemTakes, RustSemantics.isList,
      RustSemantics.isBytes, RustSemantics.isString, shapeOf]
  all_goals cases item <;> simp_all [RustSemantics.itemTakes, RustSemantics.isList,
    RustSemantics.isBytes, RustSemantics.isString, shapeOf]

theorem width_of_typed {p c A item ts vs}
    (h : (match RustSemantics.itemWidth item with
      | none => true
      | some w => if RustSemantics.itemShifts item then
          (match ts with
           | t :: _ => tyBeq t (.fixed w)
           | [] => false)
        else ts.all fun t => tyBeq t (.fixed w)) = true)
    (hw : WTL p c A vs ts) :
    (match RustSemantics.itemWidth item with
      | none => true
      | some kind => if RustSemantics.itemShifts item then
          (match vs with
           | [] => false
           | head :: _ => RustSemantics.hasWidth kind head)
        else RustSemantics.allWidth kind vs) = true := by
  cases hi : RustSemantics.itemWidth item with
  | none => rfl
  | some w =>
    rw [hi] at h
    simp only at h ⊢
    cases hs : RustSemantics.itemShifts item
    · simp only [hs, Bool.false_eq_true, if_false] at h ⊢
      exact allWidth_of vs ts h hw
    · simp only [hs, if_true] at h ⊢
      cases ts with
      | nil => simp at h
      | cons t ts =>
        cases vs with
        | nil => simp [WTL] at hw
        | cons v vs =>
          simp only [WTL] at hw
          simp only at h
          rw [tyBeq_eq _ _ h] at hw
          exact wt_fixed.mp hw.1

theorem accepts_of_typed {p c A item ts vs} (h : itemTyped item ts = true) (hw : WTL p c A vs ts) :
    RustSemantics.itemAccepts item vs = true := by
  unfold itemTyped at h
  simp only [Bool.and_eq_true] at h
  obtain ⟨hk, h⟩ := h
  have hwid := width_of_typed h hw
  unfold RustSemantics.itemAccepts
  cases vs with
  | nil => simp only [if_true]; exact hwid
  | cons v rest =>
    cases ts with
    | nil => simp [WTL] at hw
    | cons t ts' =>
      simp only at hk
      have ht := takes_of_typed hk (by simp only [WTL] at hw; exact hw.1)
      simp only [ht, if_true]
      exact hwid

theorem isFixed_eq {a : Ty} (h : isFixed a = true) : ∃ k, a = .fixed k := by
  cases a <;> simp_all [isFixed]

theorem wtl_two {p c A} {vs : List Value} {a b : Ty} (h : WTL p c A vs [a, b]) :
    ∃ x y, vs = [x, y] ∧ WT p c A x a ∧ WT p c A y b := by
  match vs, h with
  | [x, y], h => simp only [WTL] at h; exact ⟨x, y, rfl, h.1, h.2.1⟩

theorem wtl_one {p c A} {vs : List Value} {a : Ty} (h : WTL p c A vs [a]) :
    ∃ x, vs = [x] ∧ WT p c A x a := by
  match vs, h with
  | [x], h => simp only [WTL] at h; exact ⟨x, rfl, h.1⟩

set_option maxHeartbeats 2000000 in
/-- A checked operation on two operands of a width gives a value of that
width, or none. -/
theorem wt_checked {p c A op k x y w s}
    (hop : op = .checkedAdd ∨ op = .checkedSub ∨ op = .checkedMul ∨ op = .checkedQuot)
    (hx : RustSemantics.hasWidth k x = true) (hy : RustSemantics.hasWidth k y = true)
    (hp : TargetSemantics.primitive op [x, y] = .value w s) : WT p c A w (.option (.fixed k)) := by
  rcases hop with rfl | rfl | rfl | rfl
  all_goals cases k
  all_goals first
    | (obtain ⟨x, rfl⟩ := hasWidth_u8 hx; obtain ⟨y, rfl⟩ := hasWidth_u8 hy)
    | (obtain ⟨x, rfl⟩ := hasWidth_u16 hx; obtain ⟨y, rfl⟩ := hasWidth_u16 hy)
    | (obtain ⟨x, rfl⟩ := hasWidth_u32 hx; obtain ⟨y, rfl⟩ := hasWidth_u32 hy)
    | (obtain ⟨x, rfl⟩ := hasWidth_u64 hx; obtain ⟨y, rfl⟩ := hasWidth_u64 hy)
    | (obtain ⟨x, rfl⟩ := hasWidth_i8 hx; obtain ⟨y, rfl⟩ := hasWidth_i8 hy)
    | (obtain ⟨x, rfl⟩ := hasWidth_i16 hx; obtain ⟨y, rfl⟩ := hasWidth_i16 hy)
    | (obtain ⟨x, rfl⟩ := hasWidth_i32 hx; obtain ⟨y, rfl⟩ := hasWidth_i32 hy)
    | (obtain ⟨x, rfl⟩ := hasWidth_i64 hx; obtain ⟨y, rfl⟩ := hasWidth_i64 hy)
  all_goals rw [TargetSemantics.primitive.eq_def] at hp
  all_goals dsimp only at hp
  all_goals (repeat' (split at hp))
  all_goals first
    | (simp only [TargetSemantics.Outcome.value.injEq] at hp
       obtain ⟨rfl, -⟩ := hp
       simp [WT, RustSemantics.hasWidth])
    | simp at hp

set_option maxHeartbeats 2000000 in
/-- A bitwise operation on two operands of a width gives a value of that
width. -/
theorem wt_bitwise {p c A op k x y w s}
    (hop : op = .bitAnd ∨ op = .bitOr ∨ op = .bitXor)
    (hx : RustSemantics.hasWidth k x = true) (hy : RustSemantics.hasWidth k y = true)
    (hp : TargetSemantics.primitive op [x, y] = .value w s) : WT p c A w (.fixed k) := by
  rcases hop with rfl | rfl | rfl
  all_goals cases k
  all_goals first
    | (obtain ⟨x, rfl⟩ := hasWidth_u8 hx; obtain ⟨y, rfl⟩ := hasWidth_u8 hy)
    | (obtain ⟨x, rfl⟩ := hasWidth_u16 hx; obtain ⟨y, rfl⟩ := hasWidth_u16 hy)
    | (obtain ⟨x, rfl⟩ := hasWidth_u32 hx; obtain ⟨y, rfl⟩ := hasWidth_u32 hy)
    | (obtain ⟨x, rfl⟩ := hasWidth_u64 hx; obtain ⟨y, rfl⟩ := hasWidth_u64 hy)
    | (obtain ⟨x, rfl⟩ := hasWidth_i8 hx; obtain ⟨y, rfl⟩ := hasWidth_i8 hy)
    | (obtain ⟨x, rfl⟩ := hasWidth_i16 hx; obtain ⟨y, rfl⟩ := hasWidth_i16 hy)
    | (obtain ⟨x, rfl⟩ := hasWidth_i32 hx; obtain ⟨y, rfl⟩ := hasWidth_i32 hy)
    | (obtain ⟨x, rfl⟩ := hasWidth_i64 hx; obtain ⟨y, rfl⟩ := hasWidth_i64 hy)
  all_goals rw [TargetSemantics.primitive.eq_def] at hp
  all_goals dsimp only at hp
  all_goals (repeat' (split at hp))
  all_goals first
    | (simp only [TargetSemantics.Outcome.value.injEq] at hp
       obtain ⟨rfl, -⟩ := hp
       simp [WT, RustSemantics.hasWidth])
    | simp at hp

/-- A checked negation or a complement gives a value of its operand's
width. -/
theorem wt_unary {p c A op k x w s t}
    (hop : op = .checkedNeg ∧ t = .option (.fixed k) ∨ op = .bitNot ∧ t = .fixed k)
    (hx : RustSemantics.hasWidth k x = true)
    (hp : TargetSemantics.primitive op [x] = .value w s) : WT p c A w t := by
  rcases hop with ⟨rfl, rfl⟩ | ⟨rfl, rfl⟩
  all_goals cases k
  all_goals first
    | obtain ⟨x, rfl⟩ := hasWidth_u8 hx
    | obtain ⟨x, rfl⟩ := hasWidth_u16 hx
    | obtain ⟨x, rfl⟩ := hasWidth_u32 hx
    | obtain ⟨x, rfl⟩ := hasWidth_u64 hx
    | obtain ⟨x, rfl⟩ := hasWidth_i8 hx
    | obtain ⟨x, rfl⟩ := hasWidth_i16 hx
    | obtain ⟨x, rfl⟩ := hasWidth_i32 hx
    | obtain ⟨x, rfl⟩ := hasWidth_i64 hx
  all_goals rw [TargetSemantics.primitive.eq_def] at hp
  all_goals dsimp only at hp
  all_goals (repeat' (split at hp))
  all_goals first
    | (simp only [TargetSemantics.Outcome.value.injEq] at hp
       obtain ⟨rfl, -⟩ := hp
       simp [WT, RustSemantics.hasWidth])
    | simp at hp

/-- A shift gives a value of its shifted operand's width. -/
theorem wt_shift {p c A op k x a w s} (hop : op = .shiftLeft ∨ op = .shiftRight)
    (hx : RustSemantics.hasWidth k x = true)
    (hp : TargetSemantics.primitive op [x, .u32 a] = .value w s) :
    WT p c A w (.option (.fixed k)) := by
  rcases hop with rfl | rfl
  all_goals cases k
  all_goals first
    | obtain ⟨x, rfl⟩ := hasWidth_u8 hx
    | obtain ⟨x, rfl⟩ := hasWidth_u16 hx
    | obtain ⟨x, rfl⟩ := hasWidth_u32 hx
    | obtain ⟨x, rfl⟩ := hasWidth_u64 hx
    | obtain ⟨x, rfl⟩ := hasWidth_i8 hx
    | obtain ⟨x, rfl⟩ := hasWidth_i16 hx
    | obtain ⟨x, rfl⟩ := hasWidth_i32 hx
    | obtain ⟨x, rfl⟩ := hasWidth_i64 hx
  all_goals rw [TargetSemantics.primitive.eq_def] at hp
  all_goals dsimp only at hp
  all_goals (repeat' (split at hp))
  all_goals first
    | (simp only [TargetSemantics.Outcome.value.injEq] at hp
       obtain ⟨rfl, -⟩ := hp
       simp [WT, RustSemantics.hasWidth])
    | simp at hp

/-- A conversion gives a value of its target width. -/
theorem wt_convert {p c A k x w s} (hp : TargetSemantics.primitive (.convert k) [x] = .value w s) :
    WT p c A w (.option (.fixed k)) := by
  cases k
  all_goals rw [TargetSemantics.primitive.eq_def] at hp
  all_goals dsimp only at hp
  all_goals (repeat' (split at hp))
  all_goals first
    | (simp only [TargetSemantics.Outcome.value.injEq] at hp
       obtain ⟨rfl, -⟩ := hp
       simp [WT, RustSemantics.hasWidth])
    | simp at hp

/-- A parse gives a value of its target width. -/
theorem wt_parse {p c A k x w s}
    (hp : TargetSemantics.primitive (.parseDecimal (.fixed k)) [x] = .value w s) :
    WT p c A w (.option (.fixed k)) := by
  cases k
  all_goals rw [TargetSemantics.primitive.eq_def] at hp
  all_goals dsimp only at hp
  all_goals (repeat' (split at hp))
  all_goals first
    | (simp only [TargetSemantics.Outcome.value.injEq] at hp
       obtain ⟨rfl, -⟩ := hp
       simp [WT, RustSemantics.hasWidth])
    | simp at hp

theorem slice_eq (xs : List Value) (start count : Nat) :
    TargetSemantics.LexLeanRuntime.slice xs start count =
      if start + count ≤ xs.length then some ((xs.drop start).take count) else none := rfl

/-- An append keeps the elements of the lists it appends. -/
theorem wt_append {p c A x y w s t} (hx : WT p c A x t) (hy : WT p c A y t) (hs : isSeq t = true)
    (hp : TargetSemantics.primitive .append [x, y] = .value w s) : WT p c A w t := by
  rw [TargetSemantics.primitive.eq_def] at hp
  rcases isSeq_eq hs with ⟨e, rfl⟩ | rfl
  · obtain ⟨xs, rfl, hxs⟩ := wt_list_inv hx
    obtain ⟨ys, rfl, hys⟩ := wt_list_inv hy
    dsimp only at hp
    simp only [TargetSemantics.Outcome.value.injEq] at hp
    obtain ⟨rfl, -⟩ := hp
    simp only [WT]
    rw [wtAll_iff] at hxs hys ⊢
    intro v hv
    change v ∈ xs ++ ys at hv
    rcases List.mem_append.mp hv with hv | hv
    · exact hxs v hv
    · exact hys v hv
  · obtain ⟨b1, rfl⟩ := wt_bytes_inv hx
    obtain ⟨b2, rfl⟩ := wt_bytes_inv hy
    dsimp only at hp
    simp only [TargetSemantics.Outcome.value.injEq] at hp
    obtain ⟨rfl, -⟩ := hp
    simp [WT]

/-- A slice keeps the elements of the list it slices. -/
theorem wt_slice {p c A x y z w s t} (hx : WT p c A x t) (hs : isSeq t = true)
    (hp : TargetSemantics.primitive .slice [x, y, z] = .value w s) : WT p c A w (.option t) := by
  rw [TargetSemantics.primitive.eq_def] at hp
  rcases isSeq_eq hs with ⟨e, rfl⟩ | rfl
  · obtain ⟨xs, rfl, hxs⟩ := wt_list_inv hx
    dsimp only at hp
    repeat' (split at hp)
    all_goals first
      | (simp only [TargetSemantics.Outcome.value.injEq] at hp
         obtain ⟨rfl, -⟩ := hp
         simp [WT]; done)
      | (rename_i part hpart
         simp only [TargetSemantics.Outcome.value.injEq] at hp
         obtain ⟨rfl, -⟩ := hp
         simp only [WT]
         rw [wtAll_iff] at hxs ⊢
         intro v hv
         rw [slice_eq] at hpart
         split at hpart
         · simp only [Option.some.injEq] at hpart
           subst hpart
           exact hxs v (List.mem_of_mem_drop (List.mem_of_mem_take hv))
         · simp at hpart)
      | simp at hp
  · obtain ⟨b1, rfl⟩ := wt_bytes_inv hx
    dsimp only at hp
    repeat' (split at hp)
    all_goals first
      | (simp only [TargetSemantics.Outcome.value.injEq] at hp
         obtain ⟨rfl, -⟩ := hp
         simp [WT])
      | simp at hp

/-- A read of a list gives one of its elements, and of bytes a byte. -/
theorem wt_index {p c A x y w s e e'}
    (hx : WT p c A x (.list e) ∧ e' = e ∨ WT p c A x .bytes ∧ e' = .fixed .u8)
    (hp : TargetSemantics.primitive .index [x, y] = .value w s) : WT p c A w (.option e') := by
  rw [TargetSemantics.primitive.eq_def] at hp
  rcases hx with ⟨hx, rfl⟩ | ⟨hx, rfl⟩
  · obtain ⟨xs, rfl, hxs⟩ := wt_list_inv hx
    dsimp only at hp
    repeat' (split at hp)
    all_goals first
      | (simp only [TargetSemantics.Outcome.value.injEq] at hp
         obtain ⟨rfl, -⟩ := hp
         simp [WT]; done)
      | (rename_i item hitem
         simp only [TargetSemantics.Outcome.value.injEq] at hp
         obtain ⟨rfl, -⟩ := hp
         simp only [WT]
         rw [index_eq] at hitem
         exact (wtAll_iff xs).mp hxs item (List.mem_of_getElem? hitem))
      | simp at hp
  · obtain ⟨b1, rfl⟩ := wt_bytes_inv hx
    dsimp only at hp
    repeat' (split at hp)
    all_goals first
      | (simp only [TargetSemantics.Outcome.value.injEq] at hp
         obtain ⟨rfl, -⟩ := hp
         simp [WT, RustSemantics.hasWidth])
      | simp at hp

theorem fromStrings_wt {p c A} : ∀ texts : List String,
    WTAll p c A (TargetSemantics.fromStrings texts) .string
  | [] => by simp [TargetSemantics.fromStrings, WTAll]
  | t :: ts => by simp [TargetSemantics.fromStrings, WTAll, WT, fromStrings_wt ts]

/-- Text operations build text, bytes, or lists of text. -/
theorem wt_encode {p c A b w s} (hp : TargetSemantics.primitive .utf8Encode [.string b] = .value w s) :
    WT p c A w .bytes := by
  all_goals rw [TargetSemantics.primitive.eq_def] at hp
  all_goals dsimp only at hp
  all_goals (repeat' (split at hp))
  all_goals first
    | (simp only [TargetSemantics.Outcome.value.injEq] at hp
       obtain ⟨rfl, -⟩ := hp
       simp [WT, RustSemantics.hasWidth])
    | simp at hp

theorem wt_decode {p c A b w s} (hp : TargetSemantics.primitive .utf8Decode [.bytes b] = .value w s) :
    WT p c A w (.option .string) := by
  all_goals rw [TargetSemantics.primitive.eq_def] at hp
  all_goals dsimp only at hp
  all_goals (repeat' (split at hp))
  all_goals first
    | (simp only [TargetSemantics.Outcome.value.injEq] at hp
       obtain ⟨rfl, -⟩ := hp
       simp [WT, RustSemantics.hasWidth])
    | simp at hp

theorem wt_join {p c A xs d w s} (hp : TargetSemantics.primitive .join [.list xs, .string d] = .value w s) :
    WT p c A w .string := by
  all_goals rw [TargetSemantics.primitive.eq_def] at hp
  all_goals dsimp only at hp
  all_goals (repeat' (split at hp))
  all_goals first
    | (simp only [TargetSemantics.Outcome.value.injEq] at hp
       obtain ⟨rfl, -⟩ := hp
       simp [WT, RustSemantics.hasWidth])
    | simp at hp

theorem wt_format {p c A x w s} (hp : TargetSemantics.primitive .formatDecimal [x] = .value w s) :
    WT p c A w .string := by
  all_goals rw [TargetSemantics.primitive.eq_def] at hp
  all_goals dsimp only at hp
  all_goals (repeat' (split at hp))
  all_goals first
    | (simp only [TargetSemantics.Outcome.value.injEq] at hp
       obtain ⟨rfl, -⟩ := hp
       simp [WT, RustSemantics.hasWidth])
    | simp at hp

theorem wt_split {p c A a b d w s}
    (hp : TargetSemantics.primitive .splitExact [.string a, .string b, .u32 d] = .value w s) :
    WT p c A w (.option (.list .string)) := by
  all_goals rw [TargetSemantics.primitive.eq_def] at hp
  all_goals dsimp only at hp
  all_goals (repeat' (split at hp))
  all_goals first
    | (simp only [TargetSemantics.Outcome.value.injEq] at hp
       obtain ⟨rfl, -⟩ := hp
       first
         | (simp [WT]; done)
         | (simp only [WT]; exact fromStrings_wt _))
    | simp at hp

/-- A primitive whose value's type value typing constrains gives a value of
that type. -/
theorem wt_fixedTyped {p c A op ts t vs w s} (h : fixedTyped op ts t = true)
    (hw : WTL p c A vs ts) (hp : TargetSemantics.primitive op vs = .value w s) :
    WT p c A w t := by
  cases op <;> simp only [fixedTyped] at h
  all_goals first | exact absurd h Bool.false_ne_true | skip
  case checkedAdd | checkedSub | checkedMul | checkedQuot | bitAnd | bitOr | bitXor =>
    split at h
    · simp only [Bool.and_eq_true] at h
      obtain ⟨⟨ha, hb⟩, ht⟩ := h
      obtain ⟨k, rfl⟩ := isFixed_eq ha
      rw [tyBeq_eq _ _ hb] at hw
      obtain ⟨x, y, rfl, hx, hy⟩ := wtl_two hw
      rw [wt_fixed] at hx hy
      rw [tyBeq_eq _ _ ht]
      first
        | exact wt_checked (by simp) hx hy hp
        | exact wt_bitwise (by simp) hx hy hp
    · exact absurd h Bool.false_ne_true
  case checkedNeg | bitNot =>
    split at h
    · simp only [Bool.and_eq_true] at h
      obtain ⟨ha, ht⟩ := h
      obtain ⟨k, rfl⟩ := isFixed_eq ha
      obtain ⟨x, rfl, hx⟩ := wtl_one hw
      rw [wt_fixed] at hx
      rw [tyBeq_eq _ _ ht]
      exact wt_unary (by simp) hx hp
    · exact absurd h Bool.false_ne_true
  case shiftLeft | shiftRight =>
    split at h
    · simp only [Bool.and_eq_true] at h
      obtain ⟨⟨ha, hb⟩, ht⟩ := h
      obtain ⟨k, rfl⟩ := isFixed_eq ha
      rw [tyBeq_eq _ _ hb] at hw
      rw [tyBeq_eq _ _ ht]
      obtain ⟨x, y, rfl, hx, hy⟩ := wtl_two hw
      rw [wt_fixed] at hx hy
      obtain ⟨a, rfl⟩ := hasWidth_u32 hy
      exact wt_shift (by simp) hx hp
    · exact absurd h Bool.false_ne_true
  case convert k =>
    rw [tyBeq_eq _ _ h]
    rcases vs with _ | ⟨x, _ | ⟨y, rest⟩⟩
    · rw [TargetSemantics.primitive.eq_def] at hp; simp at hp
    · exact wt_convert hp
    · rw [TargetSemantics.primitive.eq_def] at hp; simp at hp
  case parseDecimal target =>
    simp only [Bool.and_eq_true] at h
    obtain ⟨ha, ht⟩ := h
    obtain ⟨k, rfl⟩ := isFixed_eq ha
    rw [tyBeq_eq _ _ ht]
    rcases vs with _ | ⟨x, _ | ⟨y, rest⟩⟩
    · rw [TargetSemantics.primitive.eq_def] at hp; simp at hp
    · exact wt_parse hp
    · rw [TargetSemantics.primitive.eq_def] at hp; simp at hp
  case append =>
    split at h
    · simp only [Bool.and_eq_true] at h
      obtain ⟨⟨ha, hb⟩, ht⟩ := h
      rw [tyBeq_eq _ _ hb] at hw
      rw [tyBeq_eq _ _ ht]
      obtain ⟨x, y, rfl, hx, hy⟩ := wtl_two hw
      exact wt_append hx hy ha hp
    · exact absurd h Bool.false_ne_true
  case slice =>
    split at h
    · simp only [Bool.and_eq_true] at h
      obtain ⟨ha, ht⟩ := h
      rw [tyBeq_eq _ _ ht]
      obtain ⟨x, y, z, rfl, hx⟩ := wtl_three hw
      exact wt_slice hx ha hp
    · exact absurd h Bool.false_ne_true
  case index =>
    split at h
    · obtain ⟨x, y, rfl, hx, -⟩ := wtl_two hw
      simp only [Bool.or_eq_true, Bool.and_eq_true] at h
      rcases h with ⟨⟨-, hl⟩, ht⟩ | ⟨hb, ht⟩
      · rw [tyBeq_eq _ _ hl] at hx
        rw [tyBeq_eq _ _ ht]
        exact wt_index (.inl ⟨hx, rfl⟩) hp
      · rw [tyBeq_eq _ _ hb] at hx
        rw [tyBeq_eq _ _ ht]
        exact wt_index (e := .unit) (.inr ⟨hx, rfl⟩) hp
    · exact absurd h Bool.false_ne_true
  case utf8Encode | utf8Decode =>
    split at h
    · simp only [Bool.and_eq_true] at h
      obtain ⟨ha, ht⟩ := h
      rw [tyBeq_eq _ _ ht]
      rw [tyBeq_eq _ _ ha] at hw
      obtain ⟨x, rfl, hx⟩ := wtl_one hw
      first
        | (obtain ⟨b, rfl⟩ := wt_string_inv hx; exact wt_encode hp)
        | (obtain ⟨b, rfl⟩ := wt_bytes_inv hx; exact wt_decode hp)
    · exact absurd h Bool.false_ne_true
  case join =>
    split at h
    · simp only [Bool.and_eq_true] at h
      obtain ⟨⟨ha, hb⟩, ht⟩ := h
      rw [tyBeq_eq _ _ ht]
      rw [tyBeq_eq _ _ ha, tyBeq_eq _ _ hb] at hw
      obtain ⟨x, y, rfl, hx, hy⟩ := wtl_two hw
      obtain ⟨xs, rfl, -⟩ := wt_list_inv hx
      obtain ⟨d, rfl⟩ := wt_string_inv hy
      exact wt_join hp
    · exact absurd h Bool.false_ne_true
  case formatDecimal =>
    split at h
    · rw [tyBeq_eq _ _ h]
      obtain ⟨x, rfl, -⟩ := wtl_one hw
      exact wt_format hp
    · exact absurd h Bool.false_ne_true
  case splitExact =>
    split at h
    · simp only [Bool.and_eq_true] at h
      obtain ⟨⟨⟨ha, hb⟩, hd⟩, ht⟩ := h
      rw [tyBeq_eq _ _ ht]
      rw [tyBeq_eq _ _ ha, tyBeq_eq _ _ hb, tyBeq_eq _ _ hd] at hw
      match vs, hw with
      | [x, y, z], hw =>
        simp only [WTL] at hw
        obtain ⟨hx, hy, hz, -⟩ := hw
        obtain ⟨a1, rfl⟩ := wt_string_inv hx
        obtain ⟨a2, rfl⟩ := wt_string_inv hy
        rw [wt_fixed] at hz
        obtain ⟨a3, rfl⟩ := hasWidth_u32 hz
        exact wt_split hp
    · exact absurd h Bool.false_ne_true

theorem wt_prim {p c A op ts t vs w s} (h : primTyped op ts t = true) (hw : WTL p c A vs ts)
    (hp : TargetSemantics.primitive op vs = .value w s) : WT p c A w t := by
  simp only [primTyped, Bool.or_eq_true] at h
  rcases h with h | h
  · exact wt_of_inert w t h
  · exact wt_fixedTyped h hw hp

mutual
/-- The identifiers a pattern binds are the only ones it adds. -/
theorem bindPattern_shape : ∀ (q : RustSyntax.Pat) (v : Value) (renv renv' : REnv),
    RustSemantics.bindPattern q v renv = some renv' →
    ∃ bs, renv' = bs ++ renv ∧ ∀ i w, (i, w) ∈ bs → i ∈ patBinds q
  | q, v, renv, renv', h => by
    cases q with
    | wild => simp [RustSemantics.bindPattern.eq_1] at h; subst h; exact ⟨[], rfl, by simp⟩
    | bind i => simp [RustSemantics.bindPattern.eq_2] at h; subst h; exact ⟨[(i, v)], rfl, by simp [patBinds]⟩
    | unit =>
      cases v <;> simp [RustSemantics.bindPattern] at h
      subst h; exact ⟨[], rfl, by simp⟩
    | none =>
      cases v <;> simp [RustSemantics.bindPattern] at h
      subst h; exact ⟨[], rfl, by simp⟩
    | ordering o =>
      cases v <;> simp [RustSemantics.bindPattern] at h
      obtain ⟨_, h⟩ := h
      subst h; exact ⟨[], rfl, by simp⟩
    | some q =>
      cases v <;> simp [RustSemantics.bindPattern] at h
      rename_i w
      obtain ⟨bs, hb, hi⟩ := bindPattern_shape q w renv renv' h
      exact ⟨bs, hb, by simpa [patBinds] using hi⟩
    | ok q =>
      cases v <;> simp [RustSemantics.bindPattern] at h
      rename_i w
      obtain ⟨bs, hb, hi⟩ := bindPattern_shape q w renv renv' h
      exact ⟨bs, hb, by simpa [patBinds] using hi⟩
    | err q =>
      cases v <;> simp [RustSemantics.bindPattern] at h
      rename_i w
      obtain ⟨bs, hb, hi⟩ := bindPattern_shape q w renv renv' h
      exact ⟨bs, hb, by simpa [patBinds] using hi⟩
    | tuple qs =>
      cases v <;> simp [RustSemantics.bindPattern] at h
      rename_i a b
      obtain ⟨bs, hb, hi⟩ := bindPatterns_shape qs [a, b] renv renv' h
      exact ⟨bs, hb, by simpa [patBinds] using hi⟩
    | adt i k qs =>
      cases v <;> simp [RustSemantics.bindPattern] at h
      rename_i tag fields
      obtain ⟨_, h⟩ := h
      obtain ⟨bs, hb, hi⟩ := bindPatterns_shape qs fields renv renv' h
      exact ⟨bs, hb, by simpa [patBinds] using hi⟩
theorem bindPatterns_shape : ∀ (qs : List RustSyntax.Pat) (vs : List Value) (renv renv' : REnv),
    RustSemantics.bindPatterns qs vs renv = some renv' →
    ∃ bs, renv' = bs ++ renv ∧ ∀ i w, (i, w) ∈ bs → i ∈ patsBinds qs
  | [], vs, renv, renv', h => by
    cases vs with
    | nil => simp [RustSemantics.bindPatterns.eq_1] at h; subst h; exact ⟨[], rfl, by simp⟩
    | cons _ _ => simp [RustSemantics.bindPatterns.eq_2] at h
  | q :: qs, vs, renv, renv', h => by
    cases vs with
    | nil => simp [RustSemantics.bindPatterns] at h
    | cons v vs =>
      simp only [RustSemantics.bindPatterns] at h
      cases h1 : RustSemantics.bindPattern q v renv with
      | none => rw [h1] at h; simp at h
      | some r1 =>
        rw [h1] at h
        simp only at h
        obtain ⟨b1, hb1, hi1⟩ := bindPattern_shape q v renv r1 h1
        obtain ⟨b2, hb2, hi2⟩ := bindPatterns_shape qs vs r1 renv' h
        subst hb1
        exact ⟨b2 ++ b1, by simp [hb2], fun i w hm => by
          simp only [List.mem_append] at hm
          simp only [patsBinds, List.mem_append]
          rcases hm with hm | hm
          · exact .inr (hi2 i w hm)
          · exact .inl (hi1 i w hm)⟩
end

theorem letsOK_shape {c renv lets renv'} (h : LetsOK c renv lets renv') :
    ∃ bs, renv' = bs ++ renv ∧ ∀ i w, (i, w) ∈ bs → i ∈ letsBind lets := by
  induction h with
  | nil => exact ⟨[], rfl, by simp⟩
  | @cons renv renv1 renv2 pat ty e v ls _ hb _ ih =>
    obtain ⟨b1, hb1, hi1⟩ := bindPattern_shape pat v renv renv1 hb
    obtain ⟨b2, hb2, hi2⟩ := ih
    subst hb1
    exact ⟨b2 ++ b1, by simp [hb2], fun i w hm => by
      simp only [List.mem_append] at hm
      simp only [letsBind, List.mem_append]
      rcases hm with hm | hm
      · exact .inr (hi2 i w hm)
      · exact .inl (hi1 i w hm)⟩

theorem rlookup_append_fresh {bs renv : REnv} {i : RIdent}
    (h : ∀ j w, (j, w) ∈ bs → RustSemantics.sameIdent j i = false) :
    RustSemantics.lookup (bs ++ renv) i = RustSemantics.lookup renv i := by
  induction bs with
  | nil => rfl
  | cons b rest ih =>
    obtain ⟨j, w⟩ := b
    simp only [List.cons_append]
    rw [rlookup_cons_other (h j w (by simp))]
    exact ih (fun j w hm => h j w (by simp [hm]))

section rules
variable {p : Program} {c : RCrate} {A : Flags}

theorem sem_opsNil {n Γ fl} : Sem p c A (n+1) Γ fl (.ops [] [] [] []) := by
  intro env renv he o ho hs
  simp only [TargetSemantics.evalList.eq_2, obsL, Option.some.injEq] at ho
  subst ho
  refine ⟨fun vs h => ?_, fun h => by cases h⟩
  cases h
  exact ⟨by simp [WTL], [], fresh_nil, .nil, rcl_nil⟩

theorem fresh_temp {Γ new k j v} (hk : k ≠ RustSyntax.IdentKind.binding) (h : Fresh Γ new) :
    Fresh Γ (new ++ [(.generated k j, v)]) := by
  intro m w hm
  simp only [List.mem_append, List.mem_singleton, Prod.mk.injEq] at hm
  rcases hm with hm | ⟨hm, _⟩
  · exact h m w hm
  · simp [local_] at hm; exact absurd hm.1.symm hk

theorem sem_opsCons {n Γ fl e es t ts r lets args k j ty} (he : Sem p c A n Γ fl (.e e t r))
    (hes : Sem p c A n Γ fl (.ops es ts lets args)) (hk : k ≠ .binding)
    (hfresh : tempFresh (.generated k j) lets = true) :
    Sem p c A (n+1) Γ fl (.ops (e :: es) (t :: ts) (.mk (.bind (.generated k j)) ty r :: lets)
      (.move (.generated k j) :: args)) := by
  intro env renv hrel o ho hs
  simp only [TargetSemantics.evalList.eq_3] at ho
  cases h1 : TargetSemantics.eval n p env e with
  | value v s1 =>
    rw [h1] at ho
    simp only at ho
    obtain ⟨hw1, ro1, hro1, hrc1⟩ := he env renv hrel (.value v) (by rw [h1]; rfl) nofun
    cases ro1 with
    | value w =>
      simp only [Realizes] at hro1; subst hro1
      have hrel1 := envRel_temp k j w hk hrel
      cases h2 : TargetSemantics.evalList n p env es with
      | values vs s2 =>
        rw [h2] at ho
        simp only [obsL, Option.some.injEq] at ho; subst ho
        obtain ⟨hv2, _⟩ := hes env _ hrel1 (.values vs) (by rw [h2]; rfl) nofun
        obtain ⟨hw2, new, hnew, hl, hargs⟩ := hv2 vs rfl
        refine ⟨fun xs hx => ?_, fun h => by cases h⟩
        cases hx
        refine ⟨⟨hw1 _ rfl, hw2⟩, new ++ [(.generated k j, w)], fresh_temp hk hnew, ?_, ?_⟩
        · simp only [List.append_assoc, List.singleton_append]
          exact .cons hrc1 (by simp [RustSemantics.bindPattern.eq_2]) hl
        · simp only [List.append_assoc, List.singleton_append]
          obtain ⟨bs, hbs, hbi⟩ := letsOK_shape hl
          have hnb : new ++ (RustSyntax.Ident.generated k j, w) :: renv =
              bs ++ (RustSyntax.Ident.generated k j, w) :: renv := by
            rw [← hbs]
          have hread : RustSemantics.lookup (new ++ (RustSyntax.Ident.generated k j, w) :: renv)
              (.generated k j) = some w := by
            rw [hnb, rlookup_append_fresh]
            · exact rlookup_cons_self _ _ _
            · intro i' w' hm
              have hi := hbi i' w' hm
              simp only [tempFresh, Bool.not_eq_true', List.any_eq_false] at hfresh
              cases hs' : RustSemantics.sameIdent i' (.generated k j)
              · rfl
              · exact absurd hs' (by simpa using hfresh i' hi)
          exact rcl_cons (rc_read (r := .move (.generated k j)) rfl hread) hargs
      | overflow s2 =>
        rw [h2] at ho
        simp only [obsL, Option.some.injEq] at ho; subst ho
        obtain ⟨_, hov⟩ := hes env _ hrel1 .overflow (by rw [h2]; rfl) nofun
        refine ⟨(fun _ h => by cases h), fun _ => ?_⟩
        rcases hov rfl with ⟨hh, hrh, hraise⟩ | ⟨new, hnew, hl, ro, hro, hargs⟩
        · exact .inl ⟨hh, hrh, .later hrc1 (by simp [RustSemantics.bindPattern.eq_2]) hraise⟩
        · refine .inr ⟨new ++ [(.generated k j, w)], fresh_temp hk hnew, ?_, ro, hro, ?_⟩
          · simp only [List.append_assoc, List.singleton_append]
            exact .cons hrc1 (by simp [RustSemantics.bindPattern.eq_2]) hl
          · simp only [List.append_assoc, List.singleton_append]
            obtain ⟨bs, hbs, hbi⟩ := letsOK_shape hl
            have hnb : new ++ (RustSyntax.Ident.generated k j, w) :: renv =
                bs ++ (RustSyntax.Ident.generated k j, w) :: renv := by
              rw [← hbs]
            have hread : RustSemantics.lookup (new ++ (RustSyntax.Ident.generated k j, w) :: renv)
                (.generated k j) = some w := by
              rw [hnb, rlookup_append_fresh]
              · exact rlookup_cons_self _ _ _
              · intro i' w' hm
                have hi := hbi i' w' hm
                simp only [tempFresh, Bool.not_eq_true', List.any_eq_false] at hfresh
                cases hs' : RustSemantics.sameIdent i' (.generated k j)
                · rfl
                · exact absurd hs' (by simpa using hfresh i' hi)
            exact rcl_raise_tail (rc_read (r := .move (.generated k j)) rfl hread) hargs
              (realizesL_halt hro)
      | stuck => rw [h2] at ho; simp [obsL] at ho; subst ho; exact absurd rfl hs
      | exhausted => rw [h2] at ho; simp [obsL] at ho
    | _ => simp [Realizes] at hro1
  | overflow s1 =>
    rw [h1] at ho
    simp only [obsL, Option.some.injEq] at ho; subst ho
    obtain ⟨_, ro1, hro1, hrc1⟩ := he env renv hrel .overflow (by rw [h1]; rfl) nofun
    exact ⟨(fun _ h => by cases h), fun _ => .inl ⟨ro1, hro1, .here hrc1 (realizes_halt hro1)⟩⟩
  | stuck => rw [h1] at ho; simp [obsL] at ho; subst ho; exact absurd rfl hs
  | exhausted => rw [h1] at ho; simp [obsL] at ho

end rules
end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

theorem rc_call_runtime {c renv args item F vs} (h : RCL c renv args (.values vs))
    (hne : RustSemantics.returned (RustSemantics.runItem c.profile item vs) F ≠ .exhausted) :
    RC c renv (.call (.runtime item) args F)
      (RustSemantics.returned (RustSemantics.runItem c.profile item vs) F) := by
  obtain ⟨n, hn⟩ := h.at
  exact ⟨hne, n+1, by rw [RustSemantics.eval.eq_10, hn n (Nat.le_refl _)]⟩

theorem rc_call_function {c renv args f F vs ro} (h : RCL c renv args (.values vs))
    (hi : RCI c (fnIdent f) vs ro)
    (hne : RustSemantics.returned ro F ≠ .exhausted) :
    RC c renv (.call (.function f) args F) (RustSemantics.returned ro F) := by
  obtain ⟨n1, hn1⟩ := h.at
  obtain ⟨n2, hn2⟩ := hi.at
  refine ⟨hne, max n1 n2 + 1, ?_⟩
  rw [RustSemantics.eval.eq_10, hn1 _ (Nat.le_max_left _ _)]
  simp only
  rw [show RustSyntax.Ident.generated RustSyntax.IdentKind.function f = fnIdent f from rfl,
    hn2 _ (Nat.le_max_right _ _)]

theorem rc_call_halt {c renv args callee F o} (h : RCL c renv args o) (ho : o = .raise ∨ o = .abort) :
    RC c renv (.call callee args F) (match o with | .abort => .abort | _ => .raise) := by
  obtain ⟨n, hn⟩ := h.at
  refine ⟨by rcases ho with rfl | rfl <;> exact nofun, n+1, ?_⟩
  rw [RustSemantics.eval.eq_10, hn n (Nat.le_refl _)]
  rcases ho with rfl | rfl <;> rfl

theorem rc_succeed {c renv r o} (h : RC c renv r o) :
    RC c renv (.succeed r) (match o with
      | .value v => .value (.ok v) | .raise => .raise | .abort => .abort
      | .stuck => .stuck | .exhausted => .exhausted) := by
  obtain ⟨n, hn⟩ := h.at
  refine ⟨?_, n+1, ?_⟩
  · cases o with
    | exhausted => exact h.1
    | _ => exact nofun
  rw [RustSemantics.eval.eq_22, hn n (Nat.le_refl _)]
  cases o <;> rfl

section rules
variable {p : Program} {c : RCrate} {A : Flags}

theorem sem_prim {n Γ fl op es ts t lets args item} (hops : Sem p c A n Γ fl (.ops es ts lets args))
    (hop : RustSemantics.itemPrimitive item = op) (hs : item ≠ .natSucc) (hok : itemOK c item = true)
    (hty : itemTyped item ts = true) (hpt : primTyped op ts t = true)
    (hfl : RustSemantics.itemFallible item = true → fl = true) :
    Sem p c A (n+1) Γ fl (.b false (.prim op es) t lets
      (.call (.runtime item) args (RustSemantics.itemFallible item))) := by
  intro env renv hrel o ho hst
  have hwt : ∀ v, o = .value v → WT p c A v t := by
    intro v hv
    subst hv
    simp only [TargetSemantics.eval.eq_11] at ho
    cases h1 : TargetSemantics.evalList n p env es with
    | values vs s1 =>
      rw [h1] at ho
      simp only [obs_chargeResult] at ho
      obtain ⟨hv, _⟩ := hops env renv hrel (.values vs) (by rw [h1]; rfl) nofun
      obtain ⟨hwl, -⟩ := hv vs rfl
      cases hpr : TargetSemantics.primitive op vs with
      | value w s2 =>
        rw [hpr] at ho; simp only [obs, Option.some.injEq, Obs.value.injEq] at ho; subst ho
        exact wt_prim hpt hwl hpr
      | _ => rw [hpr] at ho; simp [obs] at ho
    | _ => rw [h1] at ho; simp [obs] at ho
  refine ⟨hwt, ?_⟩
  simp only [TargetSemantics.eval.eq_11] at ho
  cases h1 : TargetSemantics.evalList n p env es with
  | values vs s1 =>
    rw [h1] at ho
    simp only [obs_chargeResult] at ho
    obtain ⟨hv, _⟩ := hops env renv hrel (.values vs) (by rw [h1]; rfl) nofun
    obtain ⟨hwl, new, hnew, hl, hargs⟩ := hv vs rfl
    left
    refine ⟨new, hnew, hl, ?_⟩
    have hrun := runItem_eq (vs := vs) hok hs (accepts_of_typed hty hwl)
    rw [hop] at hrun
    cases hpr : TargetSemantics.primitive op vs with
    | value w s2 =>
      rw [hpr] at ho; simp only [obs, Option.some.injEq] at ho; subst ho
      refine ⟨.value w, rfl, ?_⟩
      have := rc_call_runtime (item := item) (F := RustSemantics.itemFallible item) hargs
      rw [hrun, hpr] at this
      cases hF : RustSemantics.itemFallible item <;>
        simp [itemResult, hF, RustSemantics.returned, RustSemantics.propagated] at this ⊢ <;>
        first | exact this | exact this (by simp)
    | overflow s2 =>
      rw [hpr] at ho; simp only [obs, Option.some.injEq] at ho; subst ho
      have := rc_call_runtime (item := item) (F := RustSemantics.itemFallible item) hargs
      rw [hrun, hpr] at this
      cases hF : RustSemantics.itemFallible item
      · refine ⟨.abort, trivial, ?_⟩
        simp [itemResult, hF, RustSemantics.returned] at this
        first | exact this | exact this (by simp)
      · refine ⟨.raise, hfl hF, ?_⟩
        simp [itemResult, hF, RustSemantics.returned, RustSemantics.propagated] at this
        first | exact this | exact this (by simp)
    | stuck => rw [hpr] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
    | exhausted => rw [hpr] at ho; simp [obs] at ho
  | overflow s1 =>
    rw [h1] at ho
    simp only [obs, Option.some.injEq] at ho; subst ho
    obtain ⟨_, hov⟩ := hops env renv hrel .overflow (by rw [h1]; rfl) nofun
    rcases hov rfl with ⟨hh, hrh, hraise⟩ | ⟨new, hnew, hl, ro, hro, hargs⟩
    · exact .inr ⟨rfl, hh, hrh, hraise⟩
    · refine .inl ⟨new, hnew, hl, _, ?_, rc_call_halt (callee := .runtime item)
        (F := RustSemantics.itemFallible item) hargs (realizesL_halt hro)⟩
      rcases realizesL_halt hro with rfl | rfl <;> simp_all [RealizesM, Realizes, RealizesL]
  | stuck => rw [h1] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
  | exhausted => rw [h1] at ho; simp [obs] at ho

theorem realizesM_overflow {fm fl h} (hr : Realizes fl .overflow h) : RealizesM fm fl .overflow h := by
  cases fm with
  | false => exact hr
  | true => rcases realizes_halt hr with rfl | rfl <;> trivial

theorem sem_fOfB {n Γ e t lets tail} (h : Sem p c A n Γ true (.b false e t lets tail)) :
    Sem p c A n Γ true (.b true e t lets (.succeed tail)) := by
  intro env renv he o ho hs
  obtain ⟨hw, hcase⟩ := h env renv he o ho hs
  refine ⟨hw, ?_⟩
  rcases hcase with ⟨new, hnew, hl, ro, hro, hrc⟩ | ⟨rfl, hh, hr, hraise⟩
  · left
    refine ⟨new, hnew, hl, _, ?_, rc_succeed hrc⟩
    cases o <;> cases ro <;> simp_all [RealizesM, Realizes, RealizesF]
  · exact .inr ⟨rfl, hh, realizesM_overflow hr, hraise⟩

end rules
end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

theorem LetsRaise.halt {c renv lets h} (hl : LetsRaise c renv lets h) : Halt h := by
  induction hl with
  | here _ hh => exact hh
  | later _ _ _ ih => exact ih

section rules
variable {p : Program} {c : RCrate} {A : Flags}

end rules
end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

/-! Block inversion and the fold. -/

theorem rcb_nil_inv {c renv tail o} (h : RCB c renv ⟨[], tail⟩ o) : RC c renv tail o := by
  obtain ⟨hne, n, hn⟩ := h
  cases n with
  | zero => simp [RustSemantics.evalBlock.eq_1] at hn; exact absurd hn.symm hne
  | succ n => rw [RustSemantics.evalBlock.eq_2] at hn; exact ⟨hne, n, hn⟩

theorem rcb_nil {c renv tail o} (h : RC c renv tail o) : RCB c renv ⟨[], tail⟩ o :=
  block_of_lets .nil h

/-- A block that begins with a let: the let's value is bound and the rest
runs, or the value ends the block. -/
theorem rcb_cons_inv {c renv pat ty e ls tail o} (h : RCB c renv ⟨.mk pat ty e :: ls, tail⟩ o) :
    (∃ v renv1, RC c renv e (.value v) ∧ RustSemantics.bindPattern pat v renv = some renv1 ∧
        RCB c renv1 ⟨ls, tail⟩ o) ∨
    (RC c renv e o ∧ ∀ v, o ≠ .value v) ∨
    (∃ v, RC c renv e (.value v) ∧ RustSemantics.bindPattern pat v renv = none ∧ o = .stuck) := by
  obtain ⟨hne, n, hn⟩ := h
  cases n with
  | zero => simp [RustSemantics.evalBlock.eq_1] at hn; exact absurd hn.symm hne
  | succ n =>
    rw [RustSemantics.evalBlock.eq_3] at hn
    cases he : RustSemantics.eval n c renv e with
    | value v =>
      rw [he] at hn; simp only at hn
      cases hb : RustSemantics.bindPattern pat v renv with
      | none => rw [hb] at hn; exact .inr (.inr ⟨v, ⟨nofun, n, he⟩, hb, hn.symm⟩)
      | some renv1 => rw [hb] at hn; exact .inl ⟨v, renv1, ⟨nofun, n, he⟩, hb, hne, n, hn⟩
    | raise => rw [he] at hn; subst hn; exact .inr (.inl ⟨⟨nofun, n, he⟩, nofun⟩)
    | abort => rw [he] at hn; subst hn; exact .inr (.inl ⟨⟨nofun, n, he⟩, nofun⟩)
    | stuck => rw [he] at hn; subst hn; exact .inr (.inl ⟨⟨nofun, n, he⟩, nofun⟩)
    | exhausted => rw [he] at hn; exact absurd hn.symm hne

theorem rcb_cons_value {c renv pat ty e ls tail o v renv1} (he : RC c renv e (.value v))
    (hb : RustSemantics.bindPattern pat v renv = some renv1) (hr : RCB c renv1 ⟨ls, tail⟩ o) :
    RCB c renv ⟨.mk pat ty e :: ls, tail⟩ o := by
  obtain ⟨n1, hn1⟩ := he.at
  obtain ⟨n2, hn2⟩ := hr.at
  refine ⟨hr.1, max n1 n2 + 1, ?_⟩
  rw [RustSemantics.evalBlock.eq_3, hn1 _ (Nat.le_max_left _ _)]
  simp only [hb]
  exact hn2 _ (Nat.le_max_right _ _)

theorem rcb_cons_stop {c renv pat ty e ls tail o} (he : RC c renv e o) (hv : ∀ v, o ≠ .value v) :
    RCB c renv ⟨.mk pat ty e :: ls, tail⟩ o := by
  obtain ⟨n, hn⟩ := he.at
  refine ⟨he.1, n+1, ?_⟩
  rw [RustSemantics.evalBlock.eq_3, hn n (Nat.le_refl _)]
  cases o with
  | value v => exact absurd rfl (hv v)
  | exhausted => exact absurd rfl he.1
  | _ => rfl

theorem rcb_cons_nomatch {c renv pat ty e ls tail v} (he : RC c renv e (.value v))
    (hb : RustSemantics.bindPattern pat v renv = none) :
    RCB c renv ⟨.mk pat ty e :: ls, tail⟩ .stuck := by
  obtain ⟨n, hn⟩ := he.at
  refine ⟨nofun, n+1, ?_⟩
  rw [RustSemantics.evalBlock.eq_3, hn n (Nat.le_refl _)]
  simp only [hb]

/-- Lets in front of a block run first. -/
theorem rcb_prefix {c renv pre ls tail ls' tail' o}
    (hrest : ∀ renv1 o, RCB c renv1 ⟨ls, tail⟩ o → RCB c renv1 ⟨ls', tail'⟩ o)
    (h : RCB c renv ⟨pre ++ ls, tail⟩ o) : RCB c renv ⟨pre ++ ls', tail'⟩ o := by
  induction pre generalizing renv with
  | nil => exact hrest _ _ h
  | cons l pre ih =>
    obtain ⟨pat, ty, e⟩ := l
    rcases rcb_cons_inv h with ⟨v, renv1, he, hb, hr⟩ | ⟨he, hv⟩ | ⟨v, he, hb, rfl⟩
    · exact rcb_cons_value he hb (ih hr)
    · exact rcb_cons_stop he hv
    · exact rcb_cons_nomatch he hb

theorem RC.block_inv {c renv b o} (h : RC c renv (.block b) o) : RCB c renv b o := by
  obtain ⟨hne, n, hn⟩ := h
  cases n with
  | zero => simp [RustSemantics.eval.eq_1] at hn; exact absurd hn.symm hne
  | succ n => rw [RustSemantics.eval.eq_16] at hn; exact ⟨hne, n, hn⟩

theorem rlookup_cons_same {i j : RIdent} {w : Value} {renv : REnv}
    (h : RustSemantics.sameIdent i j = true) : RustSemantics.lookup ((i, w) :: renv) j = some w := by
  simp [RustSemantics.lookup.eq_2, h]

theorem rc_read_inv {c renv i o} {r : RExpr} (hr : r = .move i ∨ r = .copy i) (h : RC c renv r o) :
    o = .stuck ∨ ∃ v, RustSemantics.lookup renv i = some v ∧ o = .value v := by
  obtain ⟨hne, n, hn⟩ := h
  cases n with
  | zero => simp [RustSemantics.eval.eq_1] at hn; exact absurd hn.symm hne
  | succ n =>
    rcases hr with rfl | rfl
    · rw [RustSemantics.eval.eq_3] at hn
      cases hl : RustSemantics.lookup renv i with
      | none => rw [hl] at hn; exact .inl hn.symm
      | some v => rw [hl] at hn; exact .inr ⟨v, rfl, hn.symm⟩
    · rw [RustSemantics.eval.eq_5] at hn
      cases hl : RustSemantics.lookup renv i with
      | none => rw [hl] at hn; exact .inl hn.symm
      | some v => rw [hl] at hn; exact .inr ⟨v, rfl, hn.symm⟩

/-- The last binding read back is the bound value. -/
theorem rcb_fold_last {c renv i j v tail ty o} (hij : RustSemantics.sameIdent i j = true)
    (ht : tail = .move j ∨ tail = .copy j) (h : RCB c renv ⟨[.mk (.bind i) ty v], tail⟩ o) :
    RC c renv v o := by
  rcases rcb_cons_inv h with ⟨w, renv1, he, hb, hr⟩ | ⟨he, _⟩ | ⟨w, _, hb, _⟩
  · simp [RustSemantics.bindPattern.eq_2] at hb; subst hb
    rcases rc_read_inv ht (rcb_nil_inv hr) with rfl | ⟨w', hl, rfl⟩
    · exact absurd (rlookup_cons_same (w := w) (renv := renv) hij) (by
        intro hs
        obtain ⟨_, n, hn⟩ := rcb_nil_inv hr
        cases n with
        | zero => simp [RustSemantics.eval.eq_1] at hn
        | succ n =>
          rcases ht with rfl | rfl
          · rw [RustSemantics.eval.eq_3, hs] at hn; cases hn
          · rw [RustSemantics.eval.eq_5, hs] at hn; cases hn)
    · rw [rlookup_cons_same hij] at hl; cases hl; exact he
  · exact he
  · simp [RustSemantics.bindPattern.eq_2] at hb

theorem split_last {α} : ∀ (l : List α) (a : α), l.getLast? = some a → l = l.dropLast ++ [a]
  | [], _, h => by simp at h
  | [x], a, h => by simp at h; simp [h]
  | x :: y :: rest, a, h => by
    have := split_last (y :: rest) a (by simpa [List.getLast?_cons_cons] using h)
    simp only [List.dropLast_cons_cons, List.cons_append]
    exact congrArg (x :: ·) this

/-- What a fold step does. -/
theorem foldStep_spec {b b' : RBlock} (h : foldStep b = some b') :
    ∃ L i v j, b.1 = L ++ [.mk (.bind i) none v] ∧ (b.2 = .move j ∨ b.2 = .copy j) ∧
      RustSemantics.sameIdent i j = true ∧
      b' = flatB L v := by
  obtain ⟨lets, tail⟩ := b
  unfold foldStep at h
  cases hlast : lets.getLast? with
  | none => simp [hlast] at h
  | some l =>
    obtain ⟨pat, ty, v⟩ := l
    have hsplit : lets = lets.dropLast ++ [.mk pat ty v] := by
      exact split_last _ _ hlast
    cases pat <;> cases ty <;> cases tail <;> simp [hlast] at h
    all_goals
      first
      | (obtain ⟨hij, rfl⟩ := h
         exact ⟨lets.dropLast, _, v, _, hsplit, by simp, hij, rfl⟩)
      | skip

theorem rcb_flat {c renv L v o} (h : RCB c renv ⟨L, v⟩ o) : RCB c renv (flatB L v) o := by
  cases v with
  | block b =>
    obtain ⟨inner, last⟩ := b
    simp only [flatB]
    have := rcb_prefix (pre := L) (ls := []) (tail := .block ⟨inner, last⟩) (ls' := inner) (tail' := last)
      (fun renv1 o h1 => (rcb_nil_inv h1).block_inv) (by simpa using h)
    exact this
  | _ => exact h

theorem foldStep_sound {c b b' renv o} (hf : foldStep b = some b') (h : RCB c renv b o) :
    RCB c renv b' o := by
  obtain ⟨L, i, v, j, hl, ht, hij, rfl⟩ := foldStep_spec hf
  obtain ⟨lets, tail⟩ := b
  simp only at hl ht; subst hl
  apply rcb_flat
  have := rcb_prefix (pre := L) (ls := [.mk (.bind i) none v]) (tail := tail) (ls' := []) (tail' := v)
    (fun renv1 o h1 => rcb_nil (rcb_fold_last hij ht h1)) h
  simpa using this

theorem foldN_sound {c renv o} : ∀ n (b : RBlock), RCB c renv b o → RCB c renv (foldN n b) o
  | 0, _, h => h
  | n+1, b, h => by
    simp only [foldN]
    cases hs : foldStep b with
    | none => exact h
    | some b' => exact foldN_sound n b' (foldStep_sound hs h)

theorem foldB_sound {c renv o} {b : RBlock} (h : RCB c renv b o) : RCB c renv (foldB b) o := by
  obtain ⟨lets, tail⟩ := b
  exact foldN_sound _ _ h

end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

theorem rc_construct {c renv rs k vs w} (h : RCL c renv rs (.values vs))
    (hk : RustSemantics.constructValue k vs = some w) : RC c renv (.construct k rs) (.value w) := by
  obtain ⟨n, hn⟩ := h.at
  exact ⟨nofun, n+1, by rw [RustSemantics.eval.eq_12, hn n (Nat.le_refl _)]; simp [hk]⟩

theorem rc_construct_halt {c renv rs k o} (h : RCL c renv rs o) (ho : o = .raise ∨ o = .abort) :
    RC c renv (.construct k rs) (match o with | .abort => .abort | _ => .raise) := by
  obtain ⟨n, hn⟩ := h.at
  refine ⟨by rcases ho with rfl | rfl <;> exact nofun, n+1, ?_⟩
  rw [RustSemantics.eval.eq_12, hn n (Nat.le_refl _)]
  rcases ho with rfl | rfl <;> rfl

theorem rc_pair {c renv a b x y} (ha : RC c renv a (.value x)) (hb : RC c renv b (.value y)) :
    RC c renv (.pair a b) (.value (.pair x y)) := by
  obtain ⟨n1, hn1⟩ := ha.at
  obtain ⟨n2, hn2⟩ := hb.at
  exact ⟨nofun, max n1 n2 + 1, by
    rw [RustSemantics.eval.eq_13, hn1 _ (Nat.le_max_left _ _), hn2 _ (Nat.le_max_right _ _)]⟩

theorem RC.boxed {c renv r o} (h : RC c renv r o) : RC c renv (.box r) o := by
  obtain ⟨n, hn⟩ := h.at
  exact ⟨h.1, n+1, by rw [RustSemantics.eval.eq_9]; exact hn n (Nat.le_refl _)⟩

/-- The single outcome a halting list outcome is. -/
def oneOf : ROutcomes → ROut
  | .abort => .abort
  | .raise => .raise
  | .values _ => .stuck
  | .stuck => .stuck
  | .exhausted => .exhausted

/-- An evaluated list, taken apart at its head. -/
theorem rcl_cons_inv {c renv r rs o} (h : RCL c renv (r :: rs) o) (hs : o ≠ .stuck) :
    (∃ v vs, o = .values (v :: vs) ∧ RC c renv r (.value v) ∧ RCL c renv rs (.values vs)) ∨
    ((o = .raise ∨ o = .abort) ∧
      (RC c renv r (oneOf o) ∨ ∃ v, RC c renv r (.value v) ∧ RCL c renv rs o)) := by
  obtain ⟨hne, n, hn⟩ := h
  cases n with
  | zero => exact absurd hn.symm hne
  | succ n =>
    rw [RustSemantics.evalList.eq_3] at hn
    cases ha : RustSemantics.eval n c renv r with
    | value x =>
      rw [ha] at hn
      have ha' : RC c renv r (.value x) := ⟨nofun, _, ha⟩
      cases hb : RustSemantics.evalList n c renv rs with
      | values xs =>
        rw [hb] at hn; simp only at hn; subst hn
        exact .inl ⟨x, xs, rfl, ha', ⟨nofun, _, hb⟩⟩
      | raise =>
        rw [hb] at hn; simp only at hn; subst hn
        exact .inr ⟨.inl rfl, .inr ⟨x, ha', ⟨nofun, _, hb⟩⟩⟩
      | abort =>
        rw [hb] at hn; simp only at hn; subst hn
        exact .inr ⟨.inr rfl, .inr ⟨x, ha', ⟨nofun, _, hb⟩⟩⟩
      | stuck => rw [hb] at hn; simp only at hn; subst hn; exact absurd rfl hs
      | exhausted => rw [hb] at hn; simp only at hn; exact absurd hn.symm hne
    | raise => rw [ha] at hn; simp only at hn; subst hn; exact .inr ⟨.inl rfl, .inl ⟨nofun, _, ha⟩⟩
    | abort => rw [ha] at hn; simp only at hn; subst hn; exact .inr ⟨.inr rfl, .inl ⟨nofun, _, ha⟩⟩
    | stuck => rw [ha] at hn; simp only at hn; subst hn; exact absurd rfl hs
    | exhausted => rw [ha] at hn; simp only at hn; exact absurd hn.symm hne

end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

mutual
theorem rc_litValue {c renv} : ∀ (r : RExpr) (v : Value), litValue r = some v → RC c renv r (.value v)
  | .lit l, v, h => by simp only [litValue] at h; exact rc_lit h
  | .construct k rs, v, h => by
    simp only [litValue, Option.bind_eq_some_iff] at h
    obtain ⟨vs, hvs, hk⟩ := h
    exact rc_construct (rcl_litValues rs vs hvs) hk
  | .pair a b, v, h => by
    simp only [litValue, Option.bind_eq_some_iff, Option.map_eq_some_iff] at h
    obtain ⟨x, hx, y, hy, rfl⟩ := h
    exact rc_pair (rc_litValue a x hx) (rc_litValue b y hy)
  | .box e, v, h => by simp only [litValue] at h; exact (rc_litValue e v h).boxed
  | .move _, _, h | .clone _, _, h | .copy _, _, h | .deref _, _, h | .not _, _, h
  | .unbox _, _, h | .call _ _ _, _, h | .apply _ _ _, _, h | .cond _ _ _, _, h
  | .matchOn _ _, _, h | .block _, _, h | .uncons _, _, h | .isZero _, _, h
  | .nonZero _, _, h | .predecessor _, _, h | .widen _, _, h | .succeed _, _, h => by
    simp [litValue] at h
theorem rcl_litValues {c renv} : ∀ (rs : List RExpr) (vs : List Value), litValues rs = some vs →
    RCL c renv rs (.values vs)
  | [], vs, h => by simp only [litValues, Option.some.injEq] at h; subst h; exact rcl_nil
  | r :: rs, vs, h => by
    simp only [litValues, Option.bind_eq_some_iff, Option.map_eq_some_iff] at h
    obtain ⟨v, hv, ws, hws, rfl⟩ := h
    exact rcl_cons (rc_litValue r v hv) (rcl_litValues rs ws hws)
end

mutual
theorem typedLit_wt {p c A} : ∀ (v : Value) (t : Ty), typedLit p v t = true → WT p c A v t
  | v, t, h => by
    cases v with
    | unit => cases t <;> simp_all [typedLit, WT, inert]
    | none => cases t <;> simp_all [typedLit, WT, inert]
    | some x =>
      cases t with
      | option t => simp only [typedLit] at h; simp only [WT]; exact typedLit_wt x t h
      | _ => simp_all [typedLit, WT, inert]
    | ok x =>
      cases t with
      | result a b => simp only [typedLit] at h; simp only [WT]; exact typedLit_wt x a h
      | _ => simp_all [typedLit, WT, inert]
    | error x =>
      cases t with
      | result a b => simp only [typedLit] at h; simp only [WT]; exact typedLit_wt x b h
      | _ => simp_all [typedLit, WT, inert]
    | list xs =>
      cases t with
      | list t => simp only [typedLit] at h; simp only [WT]; exact typedLitAll_wt xs t h
      | _ => simp_all [typedLit, WT, inert]
    | pair x y =>
      cases t with
      | pair s t =>
        simp only [typedLit, Bool.and_eq_true] at h; simp only [WT]
        exact ⟨typedLit_wt x s h.1, typedLit_wt y t h.2⟩
      | _ => simp_all [typedLit, WT, inert]
    | adt k fs =>
      cases t with
      | adt i =>
        simp only [typedLit] at h; simp only [WT]
        cases hf : adtFields p i k with
        | none => rw [hf] at h; simp at h
        | some ts => rw [hf] at h; exact ⟨ts, rfl, typedLits_wt fs ts h⟩
      | _ => simp_all [typedLit, WT, inert]
    | closure _ _ => simp [typedLit] at h
    | _ => cases t <;> simp_all [typedLit, WT, inert]
theorem typedLitAll_wt {p c A} : ∀ (vs : List Value) (t : Ty), typedLitAll p vs t = true → WTAll p c A vs t
  | [], _, _ => by simp [WTAll]
  | v :: vs, t, h => by
    simp only [typedLitAll, Bool.and_eq_true] at h; simp only [WTAll]
    exact ⟨typedLit_wt v t h.1, typedLitAll_wt vs t h.2⟩
theorem typedLits_wt {p c A} : ∀ (vs : List Value) (ts : List Ty), typedLits p vs ts = true → WTL p c A vs ts
  | [], [], _ => by simp [WTL]
  | v :: vs, t :: ts, h => by
    simp only [typedLits, Bool.and_eq_true] at h; simp only [WTL]
    exact ⟨typedLit_wt v t h.1, typedLits_wt vs ts h.2⟩
  | [], _ :: _, h => by simp [typedLits] at h
  | _ :: _, [], h => by simp [typedLits] at h
end

end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

theorem build_sound {p c A shape ty k ts vs w s} (hb : buildShape p shape ty k = some ts)
    (hw : WTL p c A vs ts) (hc : TargetSemantics.construct shape vs = .value w s) :
    RustSemantics.constructValue k vs = some w ∧ WT p c A w ty := by
  cases shape <;> cases ty <;> cases k <;> simp [buildShape] at hb
  case none.option.none =>
    subst hb
    cases vs with
    | nil => simp [TargetSemantics.construct] at hc; obtain ⟨rfl, -⟩ := hc; exact ⟨rfl, by simp [WT]⟩
    | cons _ _ => simp [WTL] at hw
  case some.option.some t =>
    subst hb
    match vs, hw with
    | [v], hw =>
      simp [TargetSemantics.construct] at hc; obtain ⟨rfl, -⟩ := hc
      simp [WTL] at hw
      exact ⟨rfl, by simp only [WT]; exact hw⟩
  case ok.result.ok a b _ _ =>
    subst hb
    match vs, hw with
    | [v], hw =>
      simp [TargetSemantics.construct] at hc; obtain ⟨rfl, -⟩ := hc
      simp [WTL] at hw
      exact ⟨rfl, by simp only [WT]; exact hw⟩
  case error.result.err a b _ _ =>
    subst hb
    match vs, hw with
    | [v], hw =>
      simp [TargetSemantics.construct] at hc; obtain ⟨rfl, -⟩ := hc
      simp [WTL] at hw
      exact ⟨rfl, by simp only [WT]; exact hw⟩
  case nil.list.nil t _ =>
    subst hb
    cases vs with
    | nil => simp [TargetSemantics.construct] at hc; obtain ⟨rfl, -⟩ := hc
             exact ⟨rfl, by simp [WT, WTAll]⟩
    | cons _ _ => simp [WTL] at hw
  case cons.list.cons t =>
    subst hb
    match vs, hw with
    | [h, tl], hw =>
      simp only [WTL] at hw
      cases tl with
      | list items =>
        simp [TargetSemantics.construct] at hc; obtain ⟨rfl, -⟩ := hc
        simp only [WT] at hw
        exact ⟨rfl, by simp only [WT, WTAll]; exact ⟨hw.1, hw.2.1⟩⟩
      | _ => simp [TargetSemantics.construct] at hc
  case adt.adt.adt k i i' k' =>
    obtain ⟨⟨rfl, rfl⟩, hf⟩ := hb
    simp [TargetSemantics.construct] at hc; obtain ⟨rfl, -⟩ := hc
    exact ⟨rfl, by simp only [WT]; exact ⟨ts, hf, hw⟩⟩

end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

section rules
variable {p : Program} {c : RCrate} {A : Flags}

theorem sem_value {n Γ fl t v r} (hl : litValue r = some v) (ht : typedLit p v t = true) :
    Sem p c A (n+1) Γ fl (.e (.value t v) t r) := by
  intro env renv he o ho hs
  simp only [TargetSemantics.eval.eq_2, obs, Option.some.injEq] at ho
  subst ho
  exact ⟨fun w hw => by cases hw; exact typedLit_wt _ _ ht, .value v, rfl, rc_litValue r v hl⟩

theorem letsOK_nil {c renv renv'} (h : LetsOK c renv [] renv') : renv' = renv := by
  cases h; rfl

theorem sem_eOfBNil {n Γ fl e t r} (h : Sem p c A n Γ fl (.b false e t [] r)) : Sem p c A n Γ fl (.e e t r) := by
  intro env renv he o ho hs
  obtain ⟨hw, hcase⟩ := h env renv he o ho hs
  refine ⟨hw, ?_⟩
  rcases hcase with ⟨new, _, hl, ro, hro, hrc⟩ | ⟨_, _, _, hraise⟩
  · have := letsOK_nil hl
    have hnew : new = [] := by simpa using this
    subst hnew
    exact ⟨ro, hro, by simpa using hrc⟩
  · cases hraise

theorem sem_opsOfL {n Γ fl es ts rs} (h : Sem p c A n Γ fl (.l es ts rs)) :
    Sem p c A n Γ fl (.ops es ts [] rs) := by
  intro env renv he o ho hs
  obtain ⟨hw, ro, hro, hrc⟩ := h env renv he o ho hs
  refine ⟨fun vs hv => ?_, fun hov => ?_⟩
  · subst hv
    cases ro <;> simp [RealizesL] at hro
    subst hro
    exact ⟨hw _ rfl, [], fresh_nil, .nil, by simpa using hrc⟩
  · subst hov
    exact .inr ⟨[], fresh_nil, .nil, ro, hro, by simpa using hrc⟩

theorem sem_opsUnit {n Γ fl e es ts r lets args ty} (he : Sem p c A n Γ fl (.e e .unit r))
    (hes : Sem p c A n Γ fl (.ops es ts lets args)) :
    Sem p c A (n+1) Γ fl (.ops (e :: es) (.unit :: ts) (.mk .unit ty r :: lets)
      (.lit .unit :: args)) := by
  intro env renv hrel o ho hs
  simp only [TargetSemantics.evalList.eq_3] at ho
  cases h1 : TargetSemantics.eval n p env e with
  | value v s1 =>
    rw [h1] at ho
    simp only at ho
    obtain ⟨hw1, ro1, hro1, hrc1⟩ := he env renv hrel (.value v) (by rw [h1]; rfl) nofun
    have hv : v = .unit := wt_unit (hw1 v rfl)
    subst hv
    cases ro1 with
    | value w =>
      simp only [Realizes] at hro1; subst hro1
      have hbind : RustSemantics.bindPattern .unit Value.unit renv = some renv := by
        simp [RustSemantics.bindPattern]
      cases h2 : TargetSemantics.evalList n p env es with
      | values vs s2 =>
        rw [h2] at ho
        simp only [obsL, Option.some.injEq] at ho; subst ho
        obtain ⟨hv2, _⟩ := hes env renv hrel (.values vs) (by rw [h2]; rfl) nofun
        obtain ⟨hw2, new, hnew, hl, hargs⟩ := hv2 vs rfl
        refine ⟨fun xs hx => ?_, fun h => by cases h⟩
        cases hx
        exact ⟨⟨hw1 _ rfl, hw2⟩, new, hnew, .cons hrc1 hbind hl,
          rcl_cons (rc_lit (by simp [RustSemantics.literal])) hargs⟩
      | overflow s2 =>
        rw [h2] at ho
        simp only [obsL, Option.some.injEq] at ho; subst ho
        obtain ⟨_, hov⟩ := hes env renv hrel .overflow (by rw [h2]; rfl) nofun
        refine ⟨(fun _ h => by cases h), fun _ => ?_⟩
        rcases hov rfl with ⟨hh, hrh, hraise⟩ | ⟨new, hnew, hl, ro, hro, hargs⟩
        · exact .inl ⟨hh, hrh, .later hrc1 hbind hraise⟩
        · exact .inr ⟨new, hnew, .cons hrc1 hbind hl, ro, hro,
            rcl_raise_tail (rc_lit (v := Value.unit) (by simp [RustSemantics.literal])) hargs
              (realizesL_halt hro)⟩
      | stuck => rw [h2] at ho; simp [obsL] at ho; subst ho; exact absurd rfl hs
      | exhausted => rw [h2] at ho; simp [obsL] at ho
    | _ => simp [Realizes] at hro1
  | overflow s1 =>
    rw [h1] at ho
    simp only [obsL, Option.some.injEq] at ho; subst ho
    obtain ⟨_, ro1, hro1, hrc1⟩ := he env renv hrel .overflow (by rw [h1]; rfl) nofun
    exact ⟨(fun _ h => by cases h), fun _ => .inl ⟨ro1, hro1, .here hrc1 (realizes_halt hro1)⟩⟩
  | stuck => rw [h1] at ho; simp [obsL] at ho; subst ho; exact absurd rfl hs
  | exhausted => rw [h1] at ho; simp [obsL] at ho

theorem sem_build {n Γ fl shape ty es ts lets args k} (hops : Sem p c A n Γ fl (.ops es ts lets args))
    (hb : buildShape p shape ty k = some ts) :
    Sem p c A (n+1) Γ fl (.b false (.build shape ty es) ty lets (.construct k args)) := by
  intro env renv hrel o ho hst
  simp only [TargetSemantics.eval.eq_7] at ho
  cases h1 : TargetSemantics.evalList n p env es with
  | values vs s1 =>
    rw [h1] at ho
    simp only [obs_addSteps] at ho
    obtain ⟨hv, _⟩ := hops env renv hrel (.values vs) (by rw [h1]; rfl) nofun
    obtain ⟨hw, new, hnew, hl, hargs⟩ := hv vs rfl
    cases hc : TargetSemantics.construct shape vs with
    | value w s2 =>
      rw [hc] at ho; simp only [obs, Option.some.injEq] at ho; subst ho
      obtain ⟨hk, hwt⟩ := build_sound hb hw hc
      exact ⟨fun x hx => by cases hx; exact hwt, .inl ⟨new, hnew, hl, .value w, rfl, rc_construct hargs hk⟩⟩
    | overflow s2 =>
      exfalso
      have hns : shape ≠ .succ := by rintro rfl; cases ty <;> cases k <;> simp [buildShape] at hb
      cases shape <;> simp only [TargetSemantics.construct] at hc
      all_goals first
        | exact absurd rfl hns
        | (repeat' split at hc) <;> simp at hc
    | stuck => rw [hc] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
    | exhausted => rw [hc] at ho; simp [obs] at ho
  | overflow s1 =>
    rw [h1] at ho
    simp only [obs, Option.some.injEq] at ho; subst ho
    obtain ⟨_, hov⟩ := hops env renv hrel .overflow (by rw [h1]; rfl) nofun
    refine ⟨(fun _ h => by cases h), ?_⟩
    rcases hov rfl with ⟨hh, hrh, hraise⟩ | ⟨new, hnew, hl, ro, hro, hargs⟩
    · exact .inr ⟨rfl, hh, hrh, hraise⟩
    · refine .inl ⟨new, hnew, hl, _, ?_, rc_construct_halt (k := k) hargs (realizesL_halt hro)⟩
      rcases realizesL_halt hro with rfl | rfl <;> simp_all [RealizesM, Realizes, RealizesL]
  | stuck => rw [h1] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
  | exhausted => rw [h1] at ho; simp [obs] at ho

end rules
end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

theorem rc_pair_halt_left {c renv a b h} (ha : RC c renv a h) (hh : Halt h) : RC c renv (.pair a b) h := by
  obtain ⟨n, hn⟩ := ha.at
  refine ⟨ha.1, n+1, ?_⟩
  rw [RustSemantics.eval.eq_13, hn n (Nat.le_refl _)]
  rcases hh with rfl | rfl <;> rfl

theorem rc_pair_halt_right {c renv a b x h} (ha : RC c renv a (.value x)) (hb : RC c renv b h)
    (hh : Halt h) : RC c renv (.pair a b) h := by
  obtain ⟨n1, hn1⟩ := ha.at
  obtain ⟨n2, hn2⟩ := hb.at
  refine ⟨hb.1, max n1 n2 + 1, ?_⟩
  rw [RustSemantics.eval.eq_13, hn1 _ (Nat.le_max_left _ _), hn2 _ (Nat.le_max_right _ _)]
  rcases hh with rfl | rfl <;> rfl

theorem oneOf_halt {o : ROutcomes} (h : o = .raise ∨ o = .abort) : Halt (oneOf o) := by
  rcases h with rfl | rfl <;> simp [oneOf, Halt]

theorem rcl_single_halt {c renv r o} (h : RCL c renv [r] o) (ho : o = .raise ∨ o = .abort) :
    RC c renv r (oneOf o) := by
  rcases rcl_cons_inv h (by rcases ho with rfl | rfl <;> exact nofun) with
    ⟨_, _, rfl, _⟩ | ⟨_, h1 | ⟨_, _, h2⟩⟩
  · rcases ho with h | h <;> cases h
  · exact h1
  · obtain ⟨hne, n, hn⟩ := h2
    cases n with
    | zero => exact absurd hn.symm hne
    | succ n => simp [RustSemantics.evalList.eq_2] at hn; subst hn; rcases ho with h | h <;> cases h

theorem realizes_oneOf {fl o} (h : RealizesL fl .overflow o) : Realizes fl .overflow (oneOf o) := by
  cases o <;> simp_all [RealizesL, Realizes, oneOf]

theorem const_sound {p c A shape ty l w s} (hl : constShape shape ty = some l)
    (hc : TargetSemantics.construct shape [] = .value w s) :
    RustSemantics.literal l = some w ∧ WT p c A w ty := by
  cases shape <;> cases ty <;> simp [constShape] at hl <;> subst hl <;>
    simp [TargetSemantics.construct] at hc <;> obtain ⟨rfl, -⟩ := hc <;>
    exact ⟨rfl, by simp [WT, inert]⟩

theorem runItem_succ {c : RCrate} (k : Nat) :
    RustSemantics.runItem c.profile .natSucc [.nat k] =
      itemResult true (TargetSemantics.natResult (k + 1)) := by
  have hop : RustSemantics.itemOperands .natSucc [.nat k] = [.nat k, .nat 1] := rfl
  have hprim : TargetSemantics.primitive .natAdd [.nat k, .nat 1] = TargetSemantics.natResult (k + 1) := by
    simp [TargetSemantics.primitive]
  unfold RustSemantics.runItem
  cases c.profile <;> simp only [hop] <;>
    simp [RustSemantics.itemAccepts, RustSemantics.itemWidth, RustSemantics.itemHeap,
      RustSemantics.itemPrimitive, RustSemantics.itemFallible, hprim] <;>
    cases TargetSemantics.natResult (k + 1) <;> rfl

section rules
variable {p : Program} {c : RCrate} {A : Flags}

theorem sem_buildPair {n Γ fl a b s t ra rb} (hl : Sem p c A n Γ fl (.l [a, b] [s, t] [ra, rb])) :
    Sem p c A (n+1) Γ fl (.e (.build .pair (.pair s t) [a, b]) (.pair s t) (.pair ra rb)) := by
  intro env renv hrel o ho hst
  simp only [TargetSemantics.eval.eq_7] at ho
  cases h1 : TargetSemantics.evalList n p env [a, b] with
  | values vs s1 =>
    rw [h1] at ho
    simp only [obs_addSteps] at ho
    obtain ⟨hw, ro, hro, hrc⟩ := hl env renv hrel (.values vs) (by rw [h1]; rfl) nofun
    cases ro with
    | values ws =>
    simp only [RealizesL] at hro
    have hv : vs = ws := hro.symm
    subst hv
    rcases rcl_cons_inv hrc nofun with ⟨x, ws, hxs, hx, hrest⟩ | ⟨hh, _⟩
    · cases hxs
      rcases rcl_cons_inv hrest nofun with ⟨y, zs, hys, hy, hz⟩ | ⟨hh, _⟩
      · cases hys
        obtain ⟨hne, m, hm⟩ := hz
        cases m with
        | zero => exact absurd hm.symm hne
        | succ m =>
          simp [RustSemantics.evalList.eq_2] at hm; subst hm
          simp [TargetSemantics.construct, obs] at ho; subst ho
          have hw' := hw _ rfl
          simp [WTL] at hw'
          exact ⟨fun v hv => by cases hv; simp only [WT]; exact ⟨hw'.1, hw'.2⟩, .value (.pair x y), rfl,
            rc_pair hx hy⟩
      · rcases hh with h | h <;> cases h
    · rcases hh with h | h <;> cases h
    | _ => simp [RealizesL] at hro
  | overflow s1 =>
    rw [h1] at ho
    simp only [obs, Option.some.injEq] at ho; subst ho
    obtain ⟨_, ro, hro, hrc⟩ := hl env renv hrel .overflow (by rw [h1]; rfl) nofun
    have hh := realizesL_halt hro
    refine ⟨(fun _ h => by cases h), oneOf ro, realizes_oneOf hro, ?_⟩
    rcases rcl_cons_inv hrc (by rcases hh with rfl | rfl <;> exact nofun) with
      ⟨_, _, rfl, _⟩ | ⟨_, h1 | ⟨x, hx, h2⟩⟩
    · rcases hh with h | h <;> cases h
    · exact rc_pair_halt_left h1 (oneOf_halt hh)
    · exact rc_pair_halt_right hx (rcl_single_halt h2 hh) (oneOf_halt hh)
  | stuck => rw [h1] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
  | exhausted => rw [h1] at ho; simp [obs] at ho

theorem sem_buildConst {n Γ fl shape ty l} (hl : constShape shape ty = some l) :
    Sem p c A (n+1) Γ fl (.e (.build shape ty []) ty (.lit l)) := by
  intro env renv hrel o ho hst
  simp only [TargetSemantics.eval.eq_7] at ho
  cases n with
  | zero => simp [TargetSemantics.evalList, obs] at ho
  | succ n =>
    simp only [TargetSemantics.evalList.eq_2, obs_addSteps] at ho
    cases hc : TargetSemantics.construct shape [] with
    | value w s2 =>
      rw [hc] at ho; simp only [obs, Option.some.injEq] at ho; subst ho
      obtain ⟨hlit, hwt⟩ := const_sound (p := p) (c := c) (A := A) hl hc
      exact ⟨fun v hv => by cases hv; exact hwt, .value w, rfl, rc_lit hlit⟩
    | overflow s2 =>
      exfalso
      cases shape <;> cases ty <;> simp [constShape] at hl <;> simp [TargetSemantics.construct] at hc
    | stuck => rw [hc] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
    | exhausted => rw [hc] at ho; simp [obs] at ho

theorem sem_buildSucc {n Γ e r} (hl : Sem p c A n Γ true (.l [e] [.nat] [r])) :
    Sem p c A (n+1) Γ true (.e (.build .succ .nat [e]) .nat (.call (.runtime .natSucc) [r] true)) := by
  intro env renv hrel o ho hst
  refine ⟨fun v _ => wt_of_inert v .nat rfl, ?_⟩
  simp only [TargetSemantics.eval.eq_7] at ho
  cases h1 : TargetSemantics.evalList n p env [e] with
  | values vs s1 =>
    rw [h1] at ho
    simp only [obs_addSteps] at ho
    obtain ⟨_, ro, hro, hrc⟩ := hl env renv hrel (.values vs) (by rw [h1]; rfl) nofun
    cases ro with
    | values ws =>
      simp only [RealizesL] at hro
      rw [hro] at hrc
      rcases vs with _ | ⟨v, _ | ⟨_, _⟩⟩
      · simp [TargetSemantics.construct, obs] at ho; subst ho; exact absurd rfl hst
      · cases v with
        | nat k =>
          simp only [TargetSemantics.construct] at ho
          have hrun := rc_call_runtime (item := .natSucc) (F := true) hrc
          rw [runItem_succ] at hrun
          unfold TargetSemantics.natResult at ho hrun
          by_cases hlt : (k + 1).blt 18446744073709551616 = true
          · simp only [hlt, if_true, obs, Option.some.injEq] at ho; subst ho
            refine ⟨.value (.nat (k+1)), rfl, ?_⟩
            simp [hlt, itemResult, RustSemantics.returned, RustSemantics.propagated] at hrun
            first | exact hrun | exact hrun (by simp)
          · simp only [hlt, Bool.false_eq_true, if_false, obs, Option.some.injEq] at ho; subst ho
            refine ⟨.raise, rfl, ?_⟩
            simp [hlt, itemResult, RustSemantics.returned, RustSemantics.propagated] at hrun
            first | exact hrun | exact hrun (by simp)
        | _ => simp [TargetSemantics.construct, obs] at ho; subst ho; exact absurd rfl hst
      · simp [TargetSemantics.construct, obs] at ho; subst ho; exact absurd rfl hst
    | _ => simp [RealizesL] at hro
  | overflow s1 =>
    rw [h1] at ho
    simp only [obs, Option.some.injEq] at ho; subst ho
    obtain ⟨_, ro, hro, hrc⟩ := hl env renv hrel .overflow (by rw [h1]; rfl) nofun
    have hh := realizesL_halt hro
    refine ⟨_, ?_, rc_call_halt (callee := .runtime .natSucc) (F := true) hrc hh⟩
    rcases hh with rfl | rfl <;> simp_all [RealizesM, Realizes, RealizesL]
  | stuck => rw [h1] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
  | exhausted => rw [h1] at ho; simp [obs] at ho

end rules
end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

theorem slotFree_cons {Γ : Ctx} {e m} (h : slotFree (e :: Γ) m = true) : slotFree Γ m = true := by
  simp only [slotFree, List.all_cons, Bool.and_eq_true] at h ⊢; exact h.2

theorem fresh_weaken {Γ e new} (h : Fresh (e :: Γ) new) : Fresh Γ new :=
  fun m w hm => slotFree_cons (h m w hm)

theorem fresh_bind {Γ new m v} (h : Fresh Γ new) (hm : slotFree Γ m = true) :
    Fresh Γ (new ++ [(local_ m, v)]) := by
  intro m' w hm'
  simp only [List.mem_append, List.mem_singleton, Prod.mk.injEq] at hm'
  rcases hm' with hm' | ⟨hm', _⟩
  · exact h m' w hm'
  · simp [local_] at hm'; subst hm'; exact hm

theorem fresh_append {Γ a b} (ha : Fresh Γ a) (hb : Fresh Γ b) : Fresh Γ (a ++ b) := by
  intro m w hm
  simp only [List.mem_append] at hm
  rcases hm with hm | hm
  · exact ha m w hm
  · exact hb m w hm

/-- Bindings no scope entry holds leave the relation intact. -/
theorem envRel_fresh {p c A Γ env renv new} (hf : Fresh Γ new) (h : EnvRel p c A Γ env renv) :
    EnvRel p c A Γ env (new ++ renv) := by
  intro x t s hx
  obtain ⟨v, hv, hw, hr⟩ := h x t s hx
  refine ⟨v, hv, hw, fun m hm => ?_⟩
  subst hm
  rw [rlookup_append_fresh]
  · exact hr m rfl
  · intro j w hj
    cases j with
    | generated k i =>
      cases k with
      | binding =>
        have hfree := hf i w hj
        have hne : i ≠ m := by
          intro e; subst e
          have := ctxLookup_slot hx
          rw [hfree] at this; exact absurd this (by decide)
        exact sameIdent_local hne
      | _ => exact sameIdent_kind (by intro h; cases h)
    | exported _ => simp [RustSemantics.sameIdent, local_]

section rules
variable {p : Program} {c : RCrate} {A : Flags}

/-- The bound value of a let, and the body's semantics, give the let's. -/
theorem sem_letBind {n Γ fl fm x t b body τ rb m ty lets tail}
    (hb : Sem p c A n Γ fl (.e b t rb)) (hfree : slotFree Γ m = true)
    (hbody : Sem p c A n ((x, t, some m) :: Γ) fl (.b fm body τ lets tail)) :
    Sem p c A (n+1) Γ fl (.b fm (.let x t b body) τ (.mk (.bind (local_ m)) ty rb :: lets) tail) := by
  intro env renv hrel o ho hst
  simp only [TargetSemantics.eval.eq_4] at ho
  cases h1 : TargetSemantics.eval n p env b with
  | value v s1 =>
    rw [h1] at ho
    simp only [obs_addSteps] at ho
    obtain ⟨hw1, ro1, hro1, hrc1⟩ := hb env renv hrel (.value v) (by rw [h1]; rfl) nofun
    cases ro1 with
    | value w =>
      simp only [Realizes] at hro1; subst hro1
      have hrel1 := envRel_bind x t m w hfree (hw1 w rfl) hrel
      obtain ⟨hwt, hcase⟩ := hbody _ _ hrel1 o ho hst
      refine ⟨hwt, ?_⟩
      have hbind : RustSemantics.bindPattern (.bind (local_ m)) w renv = some ((local_ m, w) :: renv) := by
        simp [RustSemantics.bindPattern.eq_2]
      rcases hcase with ⟨new, hnew, hl, ro, hro, hrc⟩ | ⟨rfl, h, hrh, hraise⟩
      · refine .inl ⟨new ++ [(local_ m, w)], fresh_bind (fresh_weaken hnew) hfree, ?_, ro, hro, ?_⟩
        · simp only [List.append_assoc, List.singleton_append]; exact .cons hrc1 hbind hl
        · simpa using hrc
      · exact .inr ⟨rfl, h, hrh, .later hrc1 hbind hraise⟩
    | _ => simp [Realizes] at hro1
  | overflow s1 =>
    rw [h1] at ho
    simp only [obs, Option.some.injEq] at ho; subst ho
    obtain ⟨_, ro1, hro1, hrc1⟩ := hb env renv hrel .overflow (by rw [h1]; rfl) nofun
    exact ⟨(fun _ h => by cases h), .inr ⟨rfl, ro1, realizesM_overflow hro1, .here hrc1 (realizes_halt hro1)⟩⟩
  | stuck => rw [h1] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
  | exhausted => rw [h1] at ho; simp [obs] at ho

theorem sem_letWild {n Γ fl fm x t b body τ rb ty lets tail}
    (hb : Sem p c A n Γ fl (.e b t rb))
    (hbody : Sem p c A n ((x, t, none) :: Γ) fl (.b fm body τ lets tail)) :
    Sem p c A (n+1) Γ fl (.b fm (.let x t b body) τ (.mk .wild ty rb :: lets) tail) := by
  intro env renv hrel o ho hst
  simp only [TargetSemantics.eval.eq_4] at ho
  cases h1 : TargetSemantics.eval n p env b with
  | value v s1 =>
    rw [h1] at ho
    simp only [obs_addSteps] at ho
    obtain ⟨hw1, ro1, hro1, hrc1⟩ := hb env renv hrel (.value v) (by rw [h1]; rfl) nofun
    cases ro1 with
    | value w =>
      simp only [Realizes] at hro1; subst hro1
      have hrel1 := envRel_ghost x t w (hw1 w rfl) hrel
      obtain ⟨hwt, hcase⟩ := hbody _ _ hrel1 o ho hst
      refine ⟨hwt, ?_⟩
      have hbind : RustSemantics.bindPattern .wild w renv = some renv := by
        simp [RustSemantics.bindPattern.eq_1]
      rcases hcase with ⟨new, hnew, hl, ro, hro, hrc⟩ | ⟨rfl, h, hrh, hraise⟩
      · exact .inl ⟨new, fresh_weaken hnew, .cons hrc1 hbind hl, ro, hro, hrc⟩
      · exact .inr ⟨rfl, h, hrh, .later hrc1 hbind hraise⟩
    | _ => simp [Realizes] at hro1
  | overflow s1 =>
    rw [h1] at ho
    simp only [obs, Option.some.injEq] at ho; subst ho
    obtain ⟨_, ro1, hro1, hrc1⟩ := hb env renv hrel .overflow (by rw [h1]; rfl) nofun
    exact ⟨(fun _ h => by cases h), .inr ⟨rfl, ro1, realizesM_overflow hro1, .here hrc1 (realizes_halt hro1)⟩⟩
  | stuck => rw [h1] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
  | exhausted => rw [h1] at ho; simp [obs] at ho

end rules
end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

section rules
variable {p : Program} {c : RCrate} {A : Flags}

theorem sem_eOfBFold {n Γ fl e t lets tail} (h : Sem p c A n Γ fl (.b false e t lets tail)) :
    Sem p c A n Γ fl (.e e t (.block (foldB ⟨lets, tail⟩))) := by
  intro env renv he o ho hs
  obtain ⟨hw, hcase⟩ := h env renv he o ho hs
  refine ⟨hw, ?_⟩
  rcases hcase with ⟨new, _, hl, ro, hro, hrc⟩ | ⟨rfl, hh, hr, hraise⟩
  · exact ⟨ro, hro, RC.block (foldB_sound (block_of_lets hl hrc))⟩
  · exact ⟨hh, hr, RC.block (foldB_sound (block_of_raise hraise))⟩

/-- A call's result: in value position, propagated; in a fallible tail,
returned. -/
theorem call_result {n fl f F ws env' o renv' args} {fn : TargetSyntax.Function}
    (hfun : FunSem p c A n f) (hidx : TargetSemantics.LexLeanRuntime.index p.functions f = some fn)
    (hw : WTL p c A ws fn.types) (hb : TargetSemantics.bindAll fn.parameters ws [] = some env')
    (hF : fnFallible c f = some F) (ho : obs (TargetSemantics.eval n p env' fn.body) = some o)
    (hst : o ≠ .stuck) (hargs : RCL c renv' args (.values ws)) :
    (∀ v, o = .value v → WT p c A v fn.result) ∧
    ((F = true → fl = true) → ∃ ro, Realizes fl o ro ∧ RC c renv' (.call (.function f) args F) ro) ∧
    (F = true → ∃ ro, RealizesF o ro ∧ RC c renv' (.call (.function f) args false) ro) := by
  obtain ⟨hwr, ro, hro, hri⟩ := hfun fn ws env' F hidx hw hb hF o ho hst
  refine ⟨hwr, fun hfl => ?_, fun hT => ?_⟩
  · cases o with
    | value v =>
      cases ro <;> simp [RealizesFn] at hro
      subst hro
      refine ⟨.value v, rfl, ?_⟩
      have := rc_call_function (F := F) hargs hri
      cases F <;> simp [RustSemantics.returned, RustSemantics.propagated] at this ⊢ <;>
        first | exact this | exact this (by simp)
    | overflow =>
      cases ro with
      | value w =>
        simp only [RealizesFn] at hro
        obtain ⟨rfl, rfl⟩ := hro
        refine ⟨.raise, hfl rfl, ?_⟩
        have := rc_call_function (F := true) hargs hri
        simp [RustSemantics.returned, RustSemantics.propagated] at this
        first | exact this | exact this (by simp)
      | abort =>
        refine ⟨.abort, trivial, ?_⟩
        have := rc_call_function (F := F) hargs hri
        cases F <;> simp [RustSemantics.returned, RustSemantics.propagated] at this <;>
          first | exact this | exact this (by simp)
      | _ => simp [RealizesFn] at hro
    | stuck => exact absurd rfl hst
  · subst hT
    have := rc_call_function (F := false) hargs hri
    simp only [RustSemantics.returned, Bool.false_eq_true, if_false] at this
    refine ⟨ro, ?_, this (by intro h; subst h; exact hri.1 rfl)⟩
    cases o <;> cases ro <;> simp_all [RealizesFn, RealizesF]

theorem sem_callOps {n Γ fl f es ts t lets args F} {fn : TargetSyntax.Function}
    (hops : Sem p c A n Γ fl (.ops es ts lets args)) (hfun : FunSem p c A n f)
    (hidx : TargetSemantics.LexLeanRuntime.index p.functions f = some fn) (hts : fn.types = ts)
    (hres : fn.result = t) (hF : fnFallible c f = some F) (hfl : F = true → fl = true) :
    Sem p c A (n+1) Γ fl (.b false (.call f es) t lets (.call (.function f) args F)) := by
  intro env renv hrel o ho hst
  simp only [TargetSemantics.eval.eq_8] at ho
  cases h1 : TargetSemantics.evalList n p env es with
  | values vs s1 =>
    rw [h1] at ho
    simp only [hidx] at ho
    obtain ⟨hv, _⟩ := hops env renv hrel (.values vs) (by rw [h1]; rfl) nofun
    obtain ⟨hw, new, hnew, hl, hargs⟩ := hv vs rfl
    cases hb : TargetSemantics.bindAll fn.parameters vs [] with
    | none => rw [hb] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
    | some env' =>
      rw [hb] at ho
      simp only [obs_addSteps] at ho
      obtain ⟨hwr, hval, _⟩ := call_result hfun hidx (hts ▸ hw) hb hF ho hst hargs
      obtain ⟨ro, hro, hrc⟩ := hval hfl
      exact ⟨fun v hv => hres ▸ hwr v hv, .inl ⟨new, hnew, hl, ro, hro, hrc⟩⟩
  | overflow s1 =>
    rw [h1] at ho
    simp only [obs, Option.some.injEq] at ho; subst ho
    obtain ⟨_, hov⟩ := hops env renv hrel .overflow (by rw [h1]; rfl) nofun
    refine ⟨(fun _ h => by cases h), ?_⟩
    rcases hov rfl with ⟨hh, hrh, hraise⟩ | ⟨new, hnew, hl, ro, hro, hargs⟩
    · exact .inr ⟨rfl, hh, hrh, hraise⟩
    · refine .inl ⟨new, hnew, hl, _, ?_, rc_call_halt (callee := .function f) (F := F) hargs
        (realizesL_halt hro)⟩
      rcases realizesL_halt hro with rfl | rfl <;> simp_all [RealizesM, Realizes, RealizesL]
  | stuck => rw [h1] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
  | exhausted => rw [h1] at ho; simp [obs] at ho

theorem sem_callF {n Γ f es ts t lets args} {fn : TargetSyntax.Function}
    (hops : Sem p c A n Γ true (.ops es ts lets args)) (hfun : FunSem p c A n f)
    (hidx : TargetSemantics.LexLeanRuntime.index p.functions f = some fn) (hts : fn.types = ts)
    (hres : fn.result = t) (hF : fnFallible c f = some true) :
    Sem p c A (n+1) Γ true (.b true (.call f es) t lets (.call (.function f) args false)) := by
  intro env renv hrel o ho hst
  simp only [TargetSemantics.eval.eq_8] at ho
  cases h1 : TargetSemantics.evalList n p env es with
  | values vs s1 =>
    rw [h1] at ho
    simp only [hidx] at ho
    obtain ⟨hv, _⟩ := hops env renv hrel (.values vs) (by rw [h1]; rfl) nofun
    obtain ⟨hw, new, hnew, hl, hargs⟩ := hv vs rfl
    cases hb : TargetSemantics.bindAll fn.parameters vs [] with
    | none => rw [hb] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
    | some env' =>
      rw [hb] at ho
      simp only [obs_addSteps] at ho
      obtain ⟨hwr, _, hf⟩ := call_result (fl := true) hfun hidx (hts ▸ hw) hb hF ho hst hargs
      obtain ⟨ro, hro, hrc⟩ := hf rfl
      exact ⟨fun v hv => hres ▸ hwr v hv, .inl ⟨new, hnew, hl, ro, hro, hrc⟩⟩
  | overflow s1 =>
    rw [h1] at ho
    simp only [obs, Option.some.injEq] at ho; subst ho
    obtain ⟨_, hov⟩ := hops env renv hrel .overflow (by rw [h1]; rfl) nofun
    refine ⟨(fun _ h => by cases h), ?_⟩
    rcases hov rfl with ⟨hh, hrh, hraise⟩ | ⟨new, hnew, hl, ro, hro, hargs⟩
    · exact .inr ⟨rfl, hh, realizesM_overflow hrh, hraise⟩
    · refine .inl ⟨new, hnew, hl, _, ?_, rc_call_halt (callee := .function f) (F := false) hargs
        (realizesL_halt hro)⟩
      rcases realizesL_halt hro with rfl | rfl <;> simp_all [RealizesM, RealizesF]
  | stuck => rw [h1] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
  | exhausted => rw [h1] at ho; simp [obs] at ho

theorem sem_primF {n Γ op es ts t lets args item} (hops : Sem p c A n Γ true (.ops es ts lets args))
    (hop : RustSemantics.itemPrimitive item = op) (hs : item ≠ .natSucc) (hok : itemOK c item = true)
    (hty : itemTyped item ts = true) (hpt : primTyped op ts t = true)
    (hfal : RustSemantics.itemFallible item = true) :
    Sem p c A (n+1) Γ true (.b true (.prim op es) t lets (.call (.runtime item) args false)) := by
  intro env renv hrel o ho hst
  have hwt : ∀ v, o = .value v → WT p c A v t := by
    intro v hv
    subst hv
    simp only [TargetSemantics.eval.eq_11] at ho
    cases h1 : TargetSemantics.evalList n p env es with
    | values vs s1 =>
      rw [h1] at ho
      simp only [obs_chargeResult] at ho
      obtain ⟨hv, _⟩ := hops env renv hrel (.values vs) (by rw [h1]; rfl) nofun
      obtain ⟨hwl, -⟩ := hv vs rfl
      cases hpr : TargetSemantics.primitive op vs with
      | value w s2 =>
        rw [hpr] at ho; simp only [obs, Option.some.injEq, Obs.value.injEq] at ho; subst ho
        exact wt_prim hpt hwl hpr
      | _ => rw [hpr] at ho; simp [obs] at ho
    | _ => rw [h1] at ho; simp [obs] at ho
  refine ⟨hwt, ?_⟩
  simp only [TargetSemantics.eval.eq_11] at ho
  cases h1 : TargetSemantics.evalList n p env es with
  | values vs s1 =>
    rw [h1] at ho
    simp only [obs_chargeResult] at ho
    obtain ⟨hv, _⟩ := hops env renv hrel (.values vs) (by rw [h1]; rfl) nofun
    obtain ⟨hwl, new, hnew, hl, hargs⟩ := hv vs rfl
    left
    refine ⟨new, hnew, hl, ?_⟩
    have hrun := runItem_eq (vs := vs) hok hs (accepts_of_typed hty hwl)
    rw [hop, hfal] at hrun
    have := rc_call_runtime (item := item) (F := false) hargs
    rw [hrun] at this
    simp only [RustSemantics.returned, Bool.false_eq_true, if_false] at this
    cases hpr : TargetSemantics.primitive op vs with
    | value w s2 =>
      rw [hpr] at ho; simp only [obs, Option.some.injEq] at ho; subst ho
      rw [hpr] at this
      exact ⟨.value (.ok w), rfl, this nofun⟩
    | overflow s2 =>
      rw [hpr] at ho; simp only [obs, Option.some.injEq] at ho; subst ho
      rw [hpr] at this
      exact ⟨.value (.error .unit), rfl, this nofun⟩
    | stuck => rw [hpr] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
    | exhausted => rw [hpr] at ho; simp [obs] at ho
  | overflow s1 =>
    rw [h1] at ho
    simp only [obs, Option.some.injEq] at ho; subst ho
    obtain ⟨_, hov⟩ := hops env renv hrel .overflow (by rw [h1]; rfl) nofun
    rcases hov rfl with ⟨hh, hrh, hraise⟩ | ⟨new, hnew, hl, ro, hro, hargs⟩
    · exact .inr ⟨rfl, hh, realizesM_overflow hrh, hraise⟩
    · refine .inl ⟨new, hnew, hl, _, ?_, rc_call_halt (callee := .runtime item) (F := false) hargs
        (realizesL_halt hro)⟩
      rcases realizesL_halt hro with rfl | rfl <;> simp_all [RealizesM, RealizesF]
  | stuck => rw [h1] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
  | exhausted => rw [h1] at ho; simp [obs] at ho

theorem convert_args (k : IntKind) : ∀ vs : List Value,
    TargetSemantics.primitive (.convert k) vs ≠ .stuck → ∃ v, vs = [v] ∧ RustSemantics.widened v = some v
  | [], h => absurd (by cases k <;> rfl) h
  | [v], h => by
    cases v <;> first | exact ⟨_, rfl, rfl⟩ | (exfalso; apply h; cases k <;> rfl)
  | _ :: _ :: _, h => absurd (by cases k <;> rfl) h

theorem rc_widen {c renv r v} (h : RC c renv r (.value v)) (hw : RustSemantics.widened v = some v) :
    RC c renv (.widen r) (.value v) := by
  obtain ⟨n, hn⟩ := h.at
  exact ⟨nofun, n+1, by rw [RustSemantics.eval.eq_21, hn n (Nat.le_refl _)]; simp [hw]⟩

theorem rc_widen_halt {c renv r h} (hr : RC c renv r h) (hh : Halt h) : RC c renv (.widen r) h := by
  obtain ⟨n, hn⟩ := hr.at
  refine ⟨hr.1, n+1, ?_⟩
  rw [RustSemantics.eval.eq_21, hn n (Nat.le_refl _)]
  rcases hh with rfl | rfl <;> rfl

theorem rcl_nil_inv {c renv o} (h : RCL c renv [] o) : o = .values [] := by
  obtain ⟨hne, n, hn⟩ := h
  cases n with
  | zero => exact absurd hn.symm hne
  | succ n => rw [RustSemantics.evalList.eq_2] at hn; exact hn.symm

theorem rcl_widen_one {c renv r o} (h : RCL c renv [r] o) (hs : o ≠ .stuck)
    (hw : ∀ vs, o = .values vs → ∀ v ∈ vs, RustSemantics.widened v = some v) :
    RCL c renv [.widen r] o := by
  rcases rcl_cons_inv h hs with ⟨v, vs, rfl, hv, hvs⟩ | ⟨hh, h1 | ⟨x, hx, h2⟩⟩
  · have := rcl_nil_inv hvs; cases this
    exact rcl_cons (rc_widen hv (hw _ rfl v (by simp))) rcl_nil
  · have := rcl_raise_head (es := []) (rc_widen_halt h1 (oneOf_halt hh)) (oneOf_halt hh)
    rcases hh with rfl | rfl <;> exact this
  · have := rcl_nil_inv h2; rcases hh with rfl | rfl <;> cases this

theorem sem_primWiden {n Γ fl k e t0 t lets a} (hops : Sem p c A n Γ fl (.ops [e] [t0] lets [a]))
    (hok : itemOK c (.convert k) = true) (hpt : primTyped (.convert k) [t0] t = true) :
    Sem p c A (n+1) Γ fl (.b false (.prim (.convert k) [e]) t lets
      (.call (.runtime (.convert k)) [.widen a] false)) := by
  intro env renv hrel o ho hst
  have hwt : ∀ v, o = .value v → WT p c A v t := by
    intro v hv
    subst hv
    simp only [TargetSemantics.eval.eq_11] at ho
    cases h1 : TargetSemantics.evalList n p env [e] with
    | values vs s1 =>
      rw [h1] at ho
      simp only [obs_chargeResult] at ho
      obtain ⟨hv, _⟩ := hops env renv hrel (.values vs) (by rw [h1]; rfl) nofun
      obtain ⟨hwl, -⟩ := hv vs rfl
      cases hpr : TargetSemantics.primitive (.convert k) vs with
      | value w s2 =>
        rw [hpr] at ho; simp only [obs, Option.some.injEq, Obs.value.injEq] at ho; subst ho
        exact wt_prim hpt hwl hpr
      | _ => rw [hpr] at ho; simp [obs] at ho
    | _ => rw [h1] at ho; simp [obs] at ho
  refine ⟨hwt, ?_⟩
  simp only [TargetSemantics.eval.eq_11] at ho
  cases h1 : TargetSemantics.evalList n p env [e] with
  | values vs s1 =>
    rw [h1] at ho
    simp only [obs_chargeResult] at ho
    obtain ⟨hv, _⟩ := hops env renv hrel (.values vs) (by rw [h1]; rfl) nofun
    obtain ⟨_, new, hnew, hl, hargs⟩ := hv vs rfl
    left
    refine ⟨new, hnew, hl, ?_⟩
    have hpst : TargetSemantics.primitive (.convert k) vs ≠ .stuck := by
      intro h; rw [h] at ho; simp [obs] at ho; subst ho; exact hst rfl
    obtain ⟨v, rfl, hw⟩ := convert_args k vs hpst
    have hargs' := rcl_widen_one hargs nofun (fun ws hws w hm => by cases hws; simp at hm; subst hm; exact hw)
    have hrun := runItem_eq (vs := [v]) hok (by intro h; cases h) rfl
    have := rc_call_runtime (item := .convert k) (F := false) hargs'
    rw [hrun] at this
    simp only [RustSemantics.returned, Bool.false_eq_true, if_false, RustSemantics.itemFallible,
      RustSemantics.itemPrimitive] at this
    cases hpr : TargetSemantics.primitive (.convert k) [v] with
    | value w s2 =>
      rw [hpr] at ho; simp only [obs, Option.some.injEq] at ho; subst ho
      rw [hpr] at this
      exact ⟨.value w, rfl, this nofun⟩
    | overflow s2 =>
      rw [hpr] at ho; simp only [obs, Option.some.injEq] at ho; subst ho
      rw [hpr] at this
      exact ⟨.abort, trivial, this nofun⟩
    | stuck => exact absurd hpr hpst
    | exhausted => rw [hpr] at ho; simp [obs] at ho
  | overflow s1 =>
    rw [h1] at ho
    simp only [obs, Option.some.injEq] at ho; subst ho
    obtain ⟨_, hov⟩ := hops env renv hrel .overflow (by rw [h1]; rfl) nofun
    rcases hov rfl with ⟨hh, hrh, hraise⟩ | ⟨new, hnew, hl, ro, hro, hargs⟩
    · exact .inr ⟨rfl, hh, hrh, hraise⟩
    · have hh := realizesL_halt hro
      have hargs' := rcl_widen_one hargs (by rcases hh with rfl | rfl <;> exact nofun)
        (fun ws hws => by rcases hh with rfl | rfl <;> cases hws)
      refine .inl ⟨new, hnew, hl, _, ?_, rc_call_halt (callee := .runtime (.convert k)) (F := false)
        hargs' hh⟩
      rcases hh with rfl | rfl <;> simp_all [RealizesM, Realizes, RealizesL]
  | stuck => rw [h1] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
  | exhausted => rw [h1] at ho; simp [obs] at ho

theorem sem_buildSuccF {n Γ e r} (hl : Sem p c A n Γ true (.l [e] [.nat] [r])) :
    Sem p c A (n+1) Γ true (.b true (.build .succ .nat [e]) .nat [] (.call (.runtime .natSucc) [r] false)) := by
  intro env renv hrel o ho hst
  refine ⟨fun v _ => wt_of_inert v .nat rfl, ?_⟩
  simp only [TargetSemantics.eval.eq_7] at ho
  cases h1 : TargetSemantics.evalList n p env [e] with
  | values vs s1 =>
    rw [h1] at ho
    simp only [obs_addSteps] at ho
    obtain ⟨_, ro, hro, hrc⟩ := hl env renv hrel (.values vs) (by rw [h1]; rfl) nofun
    cases ro with
    | values ws =>
      simp only [RealizesL] at hro
      rw [hro] at hrc
      rcases vs with _ | ⟨v, _ | ⟨_, _⟩⟩
      · simp [TargetSemantics.construct, obs] at ho; subst ho; exact absurd rfl hst
      · cases v with
        | nat k =>
          simp only [TargetSemantics.construct] at ho
          have hrun := rc_call_runtime (item := .natSucc) (F := false) hrc
          rw [runItem_succ] at hrun
          unfold TargetSemantics.natResult at ho hrun
          refine .inl ⟨[], fresh_nil, .nil, ?_⟩
          by_cases hlt : (k + 1).blt 18446744073709551616 = true
          · simp only [hlt, if_true, obs, Option.some.injEq] at ho; subst ho
            refine ⟨.value (.ok (.nat (k+1))), rfl, ?_⟩
            simp [hlt, itemResult, RustSemantics.returned] at hrun
            first | exact hrun | exact hrun (by simp)
          · simp only [hlt, Bool.false_eq_true, if_false, obs, Option.some.injEq] at ho; subst ho
            refine ⟨.value (.error .unit), rfl, ?_⟩
            simp [hlt, itemResult, RustSemantics.returned] at hrun
            first | exact hrun | exact hrun (by simp)
        | _ => simp [TargetSemantics.construct, obs] at ho; subst ho; exact absurd rfl hst
      · simp [TargetSemantics.construct, obs] at ho; subst ho; exact absurd rfl hst
    | _ => simp [RealizesL] at hro
  | overflow s1 =>
    rw [h1] at ho
    simp only [obs, Option.some.injEq] at ho; subst ho
    obtain ⟨_, ro, hro, hrc⟩ := hl env renv hrel .overflow (by rw [h1]; rfl) nofun
    have hh := realizesL_halt hro
    refine .inl ⟨[], fresh_nil, .nil, _, ?_, rc_call_halt (callee := .runtime .natSucc) (F := false) hrc hh⟩
    rcases hh with rfl | rfl <;> simp_all [RealizesM, RealizesF]
  | stuck => rw [h1] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
  | exhausted => rw [h1] at ho; simp [obs] at ho

theorem sem_fUnit {n Γ e lets tail} (h : Sem p c A n Γ true (.b false e .unit lets tail)) :
    Sem p c A n Γ true (.b true e .unit (lets ++ [.mk .unit none tail]) (.succeed (.lit .unit))) := by
  intro env renv hrel o ho hst
  obtain ⟨hw, hcase⟩ := h env renv hrel o ho hst
  refine ⟨hw, ?_⟩
  rcases hcase with ⟨new, hnew, hl, ro, hro, hrc⟩ | ⟨rfl, hh, hr, hraise⟩
  · cases o with
    | value v =>
      cases ro with
      | value w => ?_
      | _ => simp [RealizesM, Realizes] at hro
      simp only [RealizesM, Realizes] at hro
      rw [hro] at hrc
      have hv : v = .unit := wt_unit (hw v rfl)
      subst hv
      refine .inl ⟨new, hnew, hl.append (.cons hrc (by simp [RustSemantics.bindPattern]) .nil),
        .value (.ok .unit), rfl, ?_⟩
      have := rc_succeed (rc_lit (c := c) (renv := new ++ renv) (l := .unit) (v := .unit)
        (by simp [RustSemantics.literal]))
      exact this
    | overflow =>
      exact .inr ⟨rfl, ro, by rcases realizes_halt hro with rfl | rfl <;> trivial,
        LetsRaise.append_right hl (.here hrc (realizes_halt hro))⟩
    | stuck => exact absurd rfl hst
  · exact .inr ⟨rfl, hh, realizesM_overflow hr, LetsRaise.append_left hraise⟩

end rules
end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

theorem wtl_length {p c A} : ∀ {vs : List Value} {ts : List Ty}, WTL p c A vs ts → vs.length = ts.length
  | [], [], _ => rfl
  | _ :: _, _ :: _, h => by simp only [WTL] at h; simp [wtl_length h.2]
  | [], _ :: _, h => by simp [WTL] at h
  | _ :: _, [], h => by simp [WTL] at h

theorem wtl_append {p c A} : ∀ {as : List Value} {ts : List Ty} {bs us}, WTL p c A as ts →
    WTL p c A bs us → WTL p c A (as ++ bs) (ts ++ us)
  | [], [], _, _, _, h => h
  | _ :: _, _ :: _, _, _, h, h' => by simp only [WTL, List.cons_append] at h ⊢; exact ⟨h.1, wtl_append h.2 h'⟩
  | [], _ :: _, _, _, h, _ => by simp [WTL] at h
  | _ :: _, [], _, _, h, _ => by simp [WTL] at h

theorem rcl_boxWith : ∀ {c renv} (mask : List Bool) (rs : List RExpr) (o : ROutcomes),
    RCL c renv rs o → o ≠ .stuck → RCL c renv (boxWith mask rs) o
  | _, _, [], rs, o, h, _ => by cases rs <;> exact h
  | _, _, _ :: _, [], o, h, _ => h
  | _, _, b :: mask, r :: rs, o, h, hs => by
    simp only [boxWith]
    rcases rcl_cons_inv h hs with ⟨v, vs, rfl, hv, hvs⟩ | ⟨hh, h1 | ⟨x, hx, h2⟩⟩
    · exact rcl_cons (by cases b <;> simp <;> first | exact hv.boxed | exact hv)
        (rcl_boxWith mask rs _ hvs nofun)
    · have := rcl_raise_head (es := boxWith mask rs)
        (show RC _ _ (if b then .box r else r) (oneOf o) by
          cases b <;> simp <;> first | exact h1.boxed | exact h1) (oneOf_halt hh)
      rcases hh with rfl | rfl <;> exact this
    · exact rcl_raise_tail (by cases b <;> simp <;> first | exact hx.boxed | exact hx)
        (rcl_boxWith mask rs _ h2 (by rcases hh with rfl | rfl <;> exact nofun)) hh

section rules
variable {p : Program} {c : RCrate} {A : Flags}

/-- Operands stored boxed evaluate as they do unboxed. -/
theorem sem_ops_box {n Γ fl es ts lets args} (mask : List Bool)
    (h : Sem p c A n Γ fl (.ops es ts lets args)) : Sem p c A n Γ fl (.ops es ts lets (boxWith mask args)) := by
  intro env renv hrel o ho hst
  obtain ⟨hv, hov⟩ := h env renv hrel o ho hst
  refine ⟨fun vs hvs => ?_, fun hof => ?_⟩
  · obtain ⟨hw, new, hnew, hl, hargs⟩ := hv vs hvs
    exact ⟨hw, new, hnew, hl, rcl_boxWith mask args _ hargs nofun⟩
  · rcases hov hof with h1 | ⟨new, hnew, hl, ro, hro, hargs⟩
    · exact .inl h1
    · exact .inr ⟨new, hnew, hl, ro, hro, rcl_boxWith mask args _ hargs
        (by rcases realizesL_halt hro with rfl | rfl <;> exact nofun)⟩

end rules

theorem rc_apply {c renv hc args P f caps vs af ff ro}
    (hl : RustSemantics.lookup renv hc = some (.closure f caps)) (hargs : RCL c renv args (.values vs))
    (hd : RustSemantics.findDispatch c.items f caps.length = some (af, ff))
    (hi : RCI c (fnIdent f) (caps ++ vs) ro)
    (hne : RustSemantics.returned (RustSemantics.dispatched ro (af && !ff)) P ≠ .exhausted) :
    RC c renv (.apply hc args P) (RustSemantics.returned (RustSemantics.dispatched ro (af && !ff)) P) := by
  obtain ⟨n1, hn1⟩ := hargs.at
  obtain ⟨n2, hn2⟩ := hi.at
  refine ⟨hne, max n1 n2 + 1, ?_⟩
  rw [RustSemantics.eval.eq_11, hl]
  simp only
  rw [hn1 _ (Nat.le_max_left _ _)]
  have e1 : (RustSemantics.LexLeanRuntime.length caps : Nat) = caps.length := rfl
  have e2 : (RustSemantics.LexLeanRuntime.append caps vs : List Value) = caps ++ vs := rfl
  simp only
  rw [e1, e2, hd]
  simp only
  rw [show RustSyntax.Ident.generated RustSyntax.IdentKind.function f = fnIdent f from rfl,
    hn2 _ (Nat.le_max_right _ _)]

theorem rc_apply_halt {c renv hc args P f caps o}
    (hl : RustSemantics.lookup renv hc = some (.closure f caps)) (hargs : RCL c renv args o)
    (ho : o = .raise ∨ o = .abort) :
    RC c renv (.apply hc args P) (oneOf o) := by
  obtain ⟨n, hn⟩ := hargs.at
  refine ⟨by rcases ho with rfl | rfl <;> exact nofun, n+1, ?_⟩
  rw [RustSemantics.eval.eq_11, hl]
  simp only
  rw [hn n (Nat.le_refl _)]
  rcases ho with rfl | rfl <;> rfl

section rules
variable {p : Program} {c : RCrate} {A : Flags}

/-- An application's result, given the closure's typing. -/
theorem apply_result {n fl f caps vs env' o renv' hc args ps r af ff} {fn : TargetSyntax.Function}
    {capTys : List Ty} (hfun : FunSem p c A n f)
    (hidx : TargetSemantics.LexLeanRuntime.index p.functions f = some fn)
    (htys : fn.types = capTys ++ ps) (hres : fn.result = r)
    (hwc : WTL p c A caps capTys) (hwv : WTL p c A vs ps)
    (hd : RustSemantics.findDispatch c.items f caps.length = some (af, ff))
    (hF : fnFallible c f = some ff) (hfa : (!ff || af) = true)
    (hb : TargetSemantics.bindAll fn.parameters (caps ++ vs) [] = some env')
    (ho : obs (TargetSemantics.eval n p env' fn.body) = some o) (hst : o ≠ .stuck)
    (hl : RustSemantics.lookup renv' hc = some (.closure f caps))
    (hargs : RCL c renv' args (.values vs)) :
    (∀ v, o = .value v → WT p c A v r) ∧
    ((af = true → fl = true) → ∃ ro, Realizes fl o ro ∧ RC c renv' (.apply hc args af) ro) ∧
    (af = true → ∃ ro, RealizesF o ro ∧ RC c renv' (.apply hc args false) ro) := by
  have hw : WTL p c A (caps ++ vs) fn.types := htys ▸ wtl_append hwc hwv
  obtain ⟨hwr, ro, hro, hri⟩ := hfun fn (caps ++ vs) env' ff hidx hw hb hF o ho hst
  refine ⟨fun v hv => hres ▸ hwr v hv, fun hfl => ?_, fun haf => ?_⟩
  · cases o with
    | value v =>
      cases ro <;> simp [RealizesFn] at hro
      subst hro
      refine ⟨.value v, rfl, ?_⟩
      have := rc_apply (P := af) hl hargs hd hri
      cases ff <;> cases af <;> simp_all [RustSemantics.returned, RustSemantics.propagated,
        RustSemantics.dispatched]
    | overflow =>
      cases ro with
      | value w =>
        simp only [RealizesFn] at hro
        obtain ⟨rfl, rfl⟩ := hro
        simp at hfa; subst hfa
        refine ⟨.raise, hfl rfl, ?_⟩
        have := rc_apply (P := true) hl hargs hd hri
        simp_all [RustSemantics.returned, RustSemantics.propagated, RustSemantics.dispatched]
      | abort =>
        refine ⟨.abort, trivial, ?_⟩
        have := rc_apply (P := af) hl hargs hd hri
        cases ff <;> cases af <;> simp_all [RustSemantics.returned, RustSemantics.propagated,
          RustSemantics.dispatched]
      | _ => simp [RealizesFn] at hro
    | stuck => exact absurd rfl hst
  · subst haf
    cases o with
    | value v =>
      cases ro <;> simp [RealizesFn] at hro
      subst hro
      refine ⟨.value (.ok v), rfl, ?_⟩
      have := rc_apply (P := false) hl hargs hd hri
      cases ff <;> simp_all [RustSemantics.returned, RustSemantics.dispatched]
    | overflow =>
      cases ro with
      | value w =>
        simp only [RealizesFn] at hro
        obtain ⟨rfl, rfl⟩ := hro
        refine ⟨.value (.error .unit), rfl, ?_⟩
        have := rc_apply (P := false) hl hargs hd hri
        simp_all [RustSemantics.returned, RustSemantics.dispatched]
      | abort =>
        refine ⟨.abort, trivial, ?_⟩
        have := rc_apply (P := false) hl hargs hd hri
        cases ff <;> simp_all [RustSemantics.returned, RustSemantics.dispatched]
      | _ => simp [RealizesFn] at hro
    | stuck => exact absurd rfl hst

/-- A temporary bound before lets that never rebind it is still bound
after them. -/
theorem lookup_through {c renv lets new i w} (hl : LetsOK c ((i, w) :: renv) lets new)
    (hfresh : tempFresh i lets = true) : ∃ bs, new = bs ++ (i, w) :: renv ∧
      RustSemantics.lookup new i = some w := by
  obtain ⟨bs, hbs, hbi⟩ := letsOK_shape hl
  refine ⟨bs, hbs, ?_⟩
  rw [hbs, rlookup_append_fresh]
  · exact rlookup_cons_self _ _ _
  · intro i' w' hm
    have hi := hbi i' w' hm
    simp only [tempFresh, Bool.not_eq_true', List.any_eq_false] at hfresh
    cases hs' : RustSemantics.sameIdent i' i
    · rfl
    · exact absurd hs' (by simpa using hfresh i' hi)

theorem sem_applyGen {n Γ fl fm tg ps r rt es lets args af k j ty P}
    (htg : Sem p c A n Γ fl (.e tg (.fn ps r) rt)) (hops : Sem p c A n Γ fl (.ops es ps lets args))
    (hfun : ∀ f, FunSem p c A n f) (hflag : flagOf A ps r = some af) (hk : k ≠ .binding)
    (hfresh : tempFresh (.generated k j) lets = true)
    (hres : ∀ {renv' f caps vs env' o} {fn : TargetSyntax.Function} {capTys : List Ty} {ff},
      TargetSemantics.LexLeanRuntime.index p.functions f = some fn →
      fn.types = capTys ++ ps → fn.result = r → WTL p c A caps capTys → WTL p c A vs ps →
      RustSemantics.findDispatch c.items f caps.length = some (af, ff) →
      fnFallible c f = some ff → (!ff || af) = true →
      TargetSemantics.bindAll fn.parameters (caps ++ vs) [] = some env' →
      obs (TargetSemantics.eval n p env' fn.body) = some o → o ≠ .stuck →
      RustSemantics.lookup renv' (.generated k j) = some (.closure f caps) →
      RCL c renv' args (.values vs) →
      ∃ ro, RealizesM fm fl o ro ∧ RC c renv' (.apply (.generated k j) args P) ro)
    (hhalt : ∀ h, Realizes fl .overflow h → RealizesM fm fl .overflow h) :
    Sem p c A (n+1) Γ fl (.b fm (.apply tg es) r (.mk (.bind (.generated k j)) ty rt :: lets)
      (.apply (.generated k j) args P)) := by
  intro env renv hrel o ho hst
  simp only [TargetSemantics.eval.eq_10] at ho
  cases h1 : TargetSemantics.eval n p env tg with
  | value v s1 =>
    rw [h1] at ho
    simp only at ho
    obtain ⟨hw1, ro1, hro1, hrc1⟩ := htg env renv hrel (.value v) (by rw [h1]; rfl) nofun
    cases ro1 with
    | value w =>
      simp only [Realizes] at hro1; subst hro1
      cases w with
      | closure f caps =>
        obtain ⟨fn, capTys, af', hfn, htys, hres', hwc, hflag', ff, hd, hF, hfa⟩ := hw1 _ rfl
        rw [hflag] at hflag'; cases hflag'
        have hrel1 := envRel_temp k j (.closure f caps) hk hrel
        have hbind : RustSemantics.bindPattern (.bind (.generated k j)) (.closure f caps) renv =
            some ((.generated k j, .closure f caps) :: renv) := by
          simp [RustSemantics.bindPattern.eq_2]
        have hidx : TargetSemantics.LexLeanRuntime.index p.functions f = some fn := by
          rw [index_eq]; exact hfn
        cases h2 : TargetSemantics.evalList n p env es with
        | values vs s2 =>
          rw [h2] at ho
          simp only [hidx] at ho
          obtain ⟨hv, _⟩ := hops env _ hrel1 (.values vs) (by rw [h2]; rfl) nofun
          obtain ⟨hwv, new, hnew, hl, hargs⟩ := hv vs rfl
          obtain ⟨bs, hbs, hread⟩ := lookup_through hl hfresh
          cases hb : TargetSemantics.bindAll fn.parameters (TargetSemantics.LexLeanRuntime.append caps vs) [] with
          | none => rw [hb] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
          | some env' =>
            rw [hb] at ho
            simp only [obs_addSteps] at ho
            have hfw := hfun f fn (caps ++ vs) env' ff hidx (htys ▸ wtl_append hwc hwv) hb hF o ho hst
            refine ⟨fun x hx => hres' ▸ hfw.1 x hx, ?_⟩
            obtain ⟨ro, hro, hrc⟩ := hres hidx htys hres' hwc hwv hd hF hfa hb ho hst hread hargs
            refine .inl ⟨new ++ [(.generated k j, .closure f caps)], fresh_temp hk hnew, ?_, ro, hro, ?_⟩
            · simp only [List.append_assoc, List.singleton_append]; exact .cons hrc1 hbind hl
            · simpa using hrc
        | overflow s2 =>
          rw [h2] at ho
          simp only [obs, Option.some.injEq] at ho; subst ho
          obtain ⟨_, hov⟩ := hops env _ hrel1 .overflow (by rw [h2]; rfl) nofun
          refine ⟨(fun _ h => by cases h), ?_⟩
          rcases hov rfl with ⟨hh, hrh, hraise⟩ | ⟨new, hnew, hl, ro, hro, hargs⟩
          · exact .inr ⟨rfl, hh, hhalt _ hrh, .later hrc1 hbind hraise⟩
          · obtain ⟨bs, hbs, hread⟩ := lookup_through hl hfresh
            refine .inl ⟨new ++ [(.generated k j, .closure f caps)], fresh_temp hk hnew, ?_,
              oneOf ro, ?_, ?_⟩
            · simp only [List.append_assoc, List.singleton_append]; exact .cons hrc1 hbind hl
            · rcases realizesL_halt hro with rfl | rfl
              · exact hhalt _ (by simpa [Realizes, RealizesL, oneOf] using hro)
              · exact hhalt _ trivial
            · simpa using rc_apply_halt (P := P) hread hargs (realizesL_halt hro)
        | stuck => rw [h2] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
        | exhausted => rw [h2] at ho; simp [obs] at ho
      | _ => simp [obs] at ho; subst ho; exact absurd rfl hst
    | _ => simp [Realizes] at hro1
  | overflow s1 =>
    rw [h1] at ho
    simp only [obs, Option.some.injEq] at ho; subst ho
    obtain ⟨_, ro1, hro1, hrc1⟩ := htg env renv hrel .overflow (by rw [h1]; rfl) nofun
    exact ⟨(fun _ h => by cases h), .inr ⟨rfl, ro1, hhalt _ hro1, .here hrc1 (realizes_halt hro1)⟩⟩
  | stuck => rw [h1] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
  | exhausted => rw [h1] at ho; simp [obs] at ho

theorem sem_apply {n Γ fl tg ps r rt es lets args af k j ty}
    (htg : Sem p c A n Γ fl (.e tg (.fn ps r) rt)) (hops : Sem p c A n Γ fl (.ops es ps lets args))
    (hfun : ∀ f, FunSem p c A n f) (hflag : flagOf A ps r = some af) (hfl : af = true → fl = true)
    (hk : k ≠ .binding) (hfresh : tempFresh (.generated k j) lets = true) :
    Sem p c A (n+1) Γ fl (.b false (.apply tg es) r (.mk (.bind (.generated k j)) ty rt :: lets)
      (.apply (.generated k j) args af)) :=
  sem_applyGen htg hops hfun hflag hk hfresh
    (fun hidx htys hres hwc hwv hd hF hfa hb ho hst hl hargs =>
      (apply_result (fl := fl) (hfun _) hidx htys hres hwc hwv hd hF hfa hb ho hst hl hargs).2.1 hfl)
    (fun _ h => h)

theorem sem_applyF {n Γ tg ps r rt es lets args k j ty}
    (htg : Sem p c A n Γ true (.e tg (.fn ps r) rt)) (hops : Sem p c A n Γ true (.ops es ps lets args))
    (hfun : ∀ f, FunSem p c A n f) (hflag : flagOf A ps r = some true)
    (hk : k ≠ .binding) (hfresh : tempFresh (.generated k j) lets = true) :
    Sem p c A (n+1) Γ true (.b true (.apply tg es) r (.mk (.bind (.generated k j)) ty rt :: lets)
      (.apply (.generated k j) args false)) :=
  sem_applyGen htg hops hfun hflag hk hfresh
    (fun hidx htys hres hwc hwv hd hF hfa hb ho hst hl hargs =>
      (apply_result (fl := true) (hfun _) hidx htys hres hwc hwv hd hF hfa hb ho hst hl hargs).2.2 rfl)
    (fun _ h => realizesM_overflow h)

theorem sem_closure {n Γ fl f es caps ps r lets args mask q af ff} {fn : TargetSyntax.Function}
    (hops : Sem p c A n Γ fl (.ops es caps lets args)) (hfn : p.functions[f]? = some fn)
    (htys : fn.types = caps ++ ps) (hres : fn.result = r) (hflag : flagOf A ps r = some af)
    (hd : RustSemantics.findDispatch c.items f caps.length = some (af, ff))
    (hF : fnFallible c f = some ff) (hfa : (!ff || af) = true) :
    Sem p c A (n+1) Γ fl (.b false (.closure f es) (.fn ps r) lets
      (.construct (.closure q f) (boxWith mask args))) := by
  intro env renv hrel o ho hst
  simp only [TargetSemantics.eval.eq_9] at ho
  cases h1 : TargetSemantics.evalList n p env es with
  | values vs s1 =>
    rw [h1] at ho
    simp only [obs, Option.some.injEq] at ho; subst ho
    obtain ⟨hv, _⟩ := hops env renv hrel (.values vs) (by rw [h1]; rfl) nofun
    obtain ⟨hw, new, hnew, hl, hargs⟩ := hv vs rfl
    refine ⟨fun v hv => ?_, .inl ⟨new, hnew, hl, .value (.closure f vs), rfl, ?_⟩⟩
    · cases hv
      simp only [WT]
      refine ⟨fn, caps, af, hfn, htys, hres, hw, hflag, ff, ?_, hF, hfa⟩
      rw [wtl_length hw]; exact hd
    · exact rc_construct (rcl_boxWith mask args _ hargs nofun) rfl
  | overflow s1 =>
    rw [h1] at ho
    simp only [obs, Option.some.injEq] at ho; subst ho
    obtain ⟨_, hov⟩ := hops env renv hrel .overflow (by rw [h1]; rfl) nofun
    refine ⟨(fun _ h => by cases h), ?_⟩
    rcases hov rfl with ⟨hh, hrh, hraise⟩ | ⟨new, hnew, hl, ro, hro, hargs⟩
    · exact .inr ⟨rfl, hh, hrh, hraise⟩
    · have hh := realizesL_halt hro
      refine .inl ⟨new, hnew, hl, _, ?_, rc_construct_halt (k := .closure q f)
        (rcl_boxWith mask args _ hargs (by rcases hh with rfl | rfl <;> exact nofun)) hh⟩
      rcases hh with rfl | rfl
      · exact (show fl = true from hro)
      · trivial
  | stuck => rw [h1] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
  | exhausted => rw [h1] at ho; simp [obs] at ho

theorem sem_part {n Γ fl s0 ta tb rs k j ty} (first : Bool)
    (hs : Sem p c A n Γ fl (.e s0 (.pair ta tb) rs)) (hk : k ≠ .binding) :
    Sem p c A (n+1) Γ fl (.b false (if first then .first s0 else .second s0) (if first then ta else tb)
      [.mk (.tuple (if first then [.bind (.generated k j), .wild] else [.wild, .bind (.generated k j)])) ty rs]
      (.move (.generated k j))) := by
  intro env renv hrel o ho hst
  have hev : TargetSemantics.eval (n+1) p env (if first then .first s0 else .second s0) =
      match TargetSemantics.eval n p env s0 with
      | .value (.pair a b) st => .value (if first then a else b) (st + 1)
      | .value _ _ => .stuck
      | .overflow st => .overflow (st + 1)
      | .stuck => .stuck
      | .exhausted => .exhausted := by
    cases first <;> simp only [Bool.false_eq_true, if_false, if_true, TargetSemantics.eval.eq_12,
      TargetSemantics.eval.eq_13] <;>
      cases TargetSemantics.eval n p env s0 <;> try rfl
    all_goals rename_i v _; cases v <;> rfl
  rw [hev] at ho
  cases h1 : TargetSemantics.eval n p env s0 with
  | value v s1 =>
    rw [h1] at ho
    obtain ⟨hw1, ro1, hro1, hrc1⟩ := hs env renv hrel (.value v) (by rw [h1]; rfl) nofun
    cases ro1 with
    | value w =>
      simp only [Realizes] at hro1; subst hro1
      cases w with
      | pair a b =>
        simp only [obs, Option.some.injEq] at ho; subst ho
        have hw := hw1 _ rfl
        simp only [WT] at hw
        have hbind : RustSemantics.bindPattern
            (.tuple (if first then [.bind (.generated k j), .wild] else [.wild, .bind (.generated k j)]))
            (.pair a b) renv = some ((.generated k j, if first then a else b) :: renv) := by
          cases first <;> simp [RustSemantics.bindPattern, RustSemantics.bindPatterns]
        refine ⟨fun v hv => by cases hv; cases first <;> simp [hw.1, hw.2], .inl ⟨[(.generated k j, if first then a else b)],
          fresh_temp (new := []) hk fresh_nil, .cons hrc1 hbind .nil, .value (if first then a else b), rfl, ?_⟩⟩
        exact rc_read (r := .move (.generated k j)) rfl (rlookup_cons_self _ _ _)
      | _ => simp [obs] at ho; subst ho; exact absurd rfl hst
    | _ => simp [Realizes] at hro1
  | overflow s1 =>
    rw [h1] at ho
    simp only [obs, Option.some.injEq] at ho; subst ho
    obtain ⟨_, ro1, hro1, hrc1⟩ := hs env renv hrel .overflow (by rw [h1]; rfl) nofun
    exact ⟨(fun _ h => by cases h), .inr ⟨rfl, ro1, hro1, .here hrc1 (realizes_halt hro1)⟩⟩
  | stuck => rw [h1] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
  | exhausted => rw [h1] at ho; simp [obs] at ho

end rules
end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

/-! Conditionals. -/

theorem rc_not_inv {c renv e o} (h : RC c renv (.not e) o) :
    (∃ β, o = .value (.bool β) ∧ RC c renv e (.value (.bool !β))) ∨ (Halt o ∧ RC c renv e o) ∨ o = .stuck := by
  obtain ⟨hne, n, hn⟩ := h
  cases n with
  | zero => simp [RustSemantics.eval.eq_1] at hn; exact absurd hn.symm hne
  | succ n =>
    rw [RustSemantics.eval.eq_7] at hn
    cases he : RustSemantics.eval n c renv e with
    | value v =>
      rw [he] at hn
      cases v with
      | bool β =>
        simp only at hn; subst hn
        refine .inl ⟨!β, rfl, ?_⟩
        simp only [Bool.not_not]; exact ⟨nofun, n, he⟩
      | _ => simp only at hn; exact .inr (.inr hn.symm)
    | raise => rw [he] at hn; subst hn; exact .inr (.inl ⟨.inl rfl, nofun, n, he⟩)
    | abort => rw [he] at hn; subst hn; exact .inr (.inl ⟨.inr rfl, nofun, n, he⟩)
    | stuck => rw [he] at hn; exact .inr (.inr hn.symm)
    | exhausted => rw [he] at hn; exact absurd hn.symm hne

theorem rc_not {c renv e o} (h : RC c renv e o) :
    RC c renv (.not e) (match o with
      | .value (.bool β) => .value (.bool !β) | .value _ => .stuck
      | .raise => .raise | .abort => .abort | .stuck => .stuck | .exhausted => .exhausted) := by
  obtain ⟨n, hn⟩ := h.at
  refine ⟨?_, n+1, ?_⟩
  · cases o with
    | value v => cases v <;> exact nofun
    | exhausted => exact h.1
    | _ => exact nofun
  · rw [RustSemantics.eval.eq_7, hn n (Nat.le_refl _)]
    cases o with
    | value v => cases v <;> rfl
    | _ => rfl

theorem rc_isZero_inv {c renv i o} (h : RC c renv (.isZero i) o) (hs : o ≠ .stuck) :
    ∃ k, RustSemantics.lookup renv i = some (.nat k) ∧ o = .value (.bool (Nat.beq k 0)) := by
  obtain ⟨hne, n, hn⟩ := h
  cases n with
  | zero => simp [RustSemantics.eval.eq_1] at hn; exact absurd hn.symm hne
  | succ n =>
    rw [RustSemantics.eval.eq_18] at hn
    cases hl : RustSemantics.lookup renv i with
    | none => rw [hl] at hn; exact absurd hn.symm hs
    | some v =>
      rw [hl] at hn
      cases v with
      | nat k => simp only at hn; exact ⟨k, rfl, hn.symm⟩
      | _ => simp only at hn; exact absurd hn.symm hs

theorem rc_nonZero_inv {c renv i o} (h : RC c renv (.nonZero i) o) (hs : o ≠ .stuck) :
    ∃ k, RustSemantics.lookup renv i = some (.nat k) ∧ o = .value (.bool !(Nat.beq k 0)) := by
  obtain ⟨hne, n, hn⟩ := h
  cases n with
  | zero => simp [RustSemantics.eval.eq_1] at hn; exact absurd hn.symm hne
  | succ n =>
    rw [RustSemantics.eval.eq_19] at hn
    cases hl : RustSemantics.lookup renv i with
    | none => rw [hl] at hn; exact absurd hn.symm hs
    | some v =>
      rw [hl] at hn
      cases v with
      | nat k => simp only at hn; exact ⟨k, rfl, hn.symm⟩
      | _ => simp only at hn; exact absurd hn.symm hs

theorem rc_isZero {c renv i k} (h : RustSemantics.lookup renv i = some (.nat k)) :
    RC c renv (.isZero i) (.value (.bool (Nat.beq k 0))) :=
  ⟨nofun, 1, by rw [RustSemantics.eval.eq_18, h]⟩

theorem rc_nonZero {c renv i k} (h : RustSemantics.lookup renv i = some (.nat k)) :
    RC c renv (.nonZero i) (.value (.bool !(Nat.beq k 0))) :=
  ⟨nofun, 1, by rw [RustSemantics.eval.eq_19, h]⟩

theorem rc_lit_inv {c renv l o} (h : RC c renv (.lit l) o) (hs : o ≠ .stuck) :
    ∃ v, RustSemantics.literal l = some v ∧ o = .value v := by
  obtain ⟨hne, n, hn⟩ := h
  cases n with
  | zero => simp [RustSemantics.eval.eq_1] at hn; exact absurd hn.symm hne
  | succ n =>
    rw [RustSemantics.eval.eq_2] at hn
    cases hl : RustSemantics.literal l with
    | none => rw [hl] at hn; exact absurd hn.symm hs
    | some v => rw [hl] at hn; exact ⟨v, rfl, hn.symm⟩

/-- The negation the rendering writes negates. -/
theorem rc_negate {c renv ce β} (h : RC c renv ce (.value (.bool β))) :
    RC c renv (negateR ce) (.value (.bool !β)) := by
  have hnot := rc_not h
  simp only at hnot
  cases ce with
  | lit l =>
    cases l with
    | bool b =>
      obtain ⟨v, hl, hv⟩ := rc_lit_inv h nofun
      cases hv
      simp [RustSemantics.literal] at hl
      subst hl
      exact rc_lit (by simp [RustSemantics.literal])
    | _ => exact hnot
  | isZero i =>
    obtain ⟨k, hl, hv⟩ := rc_isZero_inv h nofun
    cases hv
    exact rc_nonZero hl
  | nonZero i =>
    obtain ⟨k, hl, hv⟩ := rc_nonZero_inv h nofun
    cases hv
    show RC c renv (.isZero i) _
    simpa using rc_isZero (c := c) hl
  | not e =>
    rcases rc_not_inv h with ⟨β', hv, he⟩ | ⟨hh, _⟩ | hs
    · cases hv; show RC c renv e _; simpa using he
    · rcases hh with h | h <;> cases h
    · cases hs
  | _ => exact hnot

theorem rc_negate_halt {c renv ce h} (hr : RC c renv ce h) (hh : Halt h) : RC c renv (negateR ce) h := by
  have hnot := rc_not hr
  have hne : h ≠ .stuck := by rcases hh with rfl | rfl <;> exact nofun
  cases ce with
  | lit l =>
    obtain ⟨v, _, rfl⟩ := rc_lit_inv hr hne
    rcases hh with h | h <;> cases h
  | isZero i =>
    obtain ⟨k, _, rfl⟩ := rc_isZero_inv hr hne
    rcases hh with h | h <;> cases h
  | nonZero i =>
    obtain ⟨k, _, rfl⟩ := rc_nonZero_inv hr hne
    rcases hh with h | h <;> cases h
  | not e =>
    rcases rc_not_inv hr with ⟨β', rfl, _⟩ | ⟨_, he⟩ | rfl
    · rcases hh with h | h <;> cases h
    · show RC c renv e _; exact he
    · exact absurd rfl hne
  | _ => rcases hh with rfl | rfl <;> exact hnot

theorem rc_cond {c renv ce tb eb β o} (hc : RC c renv ce (.value (.bool β)))
    (hb : RCB c renv (if β then tb else eb) o) : RC c renv (.cond ce tb eb) o := by
  obtain ⟨n1, hn1⟩ := hc.at
  obtain ⟨n2, hn2⟩ := hb.at
  refine ⟨hb.1, max n1 n2 + 1, ?_⟩
  rw [RustSemantics.eval.eq_14, hn1 _ (Nat.le_max_left _ _)]
  cases β <;> simp only [Bool.false_eq_true, if_false, if_true] at hn2 ⊢ <;>
    exact hn2 _ (Nat.le_max_right _ _)

theorem rc_cond_halt {c renv ce tb eb h} (hc : RC c renv ce h) (hh : Halt h) :
    RC c renv (.cond ce tb eb) h := by
  obtain ⟨n, hn⟩ := hc.at
  refine ⟨hc.1, n+1, ?_⟩
  rw [RustSemantics.eval.eq_14, hn n (Nat.le_refl _)]
  rcases hh with rfl | rfl <;> rfl

theorem rcb_of_lets {c renv pre renv' ls tail o} (hl : LetsOK c renv pre renv')
    (h : RCB c renv' ⟨ls, tail⟩ o) : RCB c renv ⟨pre ++ ls, tail⟩ o := by
  induction hl with
  | nil => exact h
  | cons he hb _ ih => exact rcb_cons_value he hb (ih h)

/-- A conditional's Rust halts when its condition does. -/
theorem k_halt {p c A Γ fl fm ce a b t lets tail}
    (h : Corr p c A Γ fl (.k .plain fm ce a b t lets tail)) :
    ∀ renv h', Halt h' → RC c renv ce h' → (lets = [] ∧ RC c renv tail h') ∨ LetsRaise c renv lets h' := by
  intro renv h' hh hc
  cases h with
  | kId => exact .inl ⟨rfl, hc⟩
  | kNeg => exact .inl ⟨rfl, rc_negate_halt hc hh⟩
  | kSame => exact .inr (.here hc hh)
  | kIf => exact .inl ⟨rfl, rc_cond_halt hc hh⟩
  | kHeld => exact .inr (.here hc hh)

end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

theorem rc_pred {c renv i k} (h : RustSemantics.lookup renv i = some (.nat k)) (hk : k ≠ 0) :
    RC c renv (.predecessor i) (.value (.nat (k - 1))) := by
  obtain ⟨k', rfl⟩ : ∃ k', k = k' + 1 := ⟨k - 1, by omega⟩
  exact ⟨nofun, 1, by rw [RustSemantics.eval.eq_20, h]; rfl⟩

theorem kpre_cond {c Γ kind renv ce β env env'} (h : KPre c Γ kind renv ce β env env') :
    RC c renv ce (.value (.bool β)) := by
  cases kind with
  | plain => exact h.1
  | nat hv x s => obtain ⟨_, k, _, hc, _⟩ := h; exact hc

theorem kpre_then {c Γ kind renv ce env env'} (h : KPre c Γ kind renv ce true env env') : env' = env := by
  cases kind with
  | plain => exact h.2
  | nat hv x s => obtain ⟨_, k, _, _, _, he⟩ := h; simpa using he

theorem fresh_elseCtx {Γ kind new} (h : Fresh (elseCtx kind Γ) new) : Fresh Γ new := by
  cases kind with
  | plain => exact h
  | nat hv x s => exact fresh_weaken h

theorem kpre_else {p c A Γ kind renv ce env env'} (hrel : EnvRel p c A Γ env renv)
    (h : KPre c Γ kind renv ce false env env') :
    ∃ renv', LetsOK c renv (elseLets kind) renv' ∧ EnvRel p c A (elseCtx kind Γ) env' renv' := by
  cases kind with
  | plain => obtain ⟨_, rfl⟩ := h; exact ⟨renv, .nil, hrel⟩
  | nat hv x s =>
    obtain ⟨hs, k, hl, _, hβ, rfl⟩ := h
    have hk : k ≠ 0 := by intro h0; subst h0; simp at hβ
    simp only [Bool.false_eq_true, if_false]
    cases s with
    | none => exact ⟨renv, .nil, envRel_ghost x .nat _ (wt_of_inert _ .nat rfl) hrel⟩
    | some m =>
      refine ⟨(local_ m, .nat (k - 1)) :: renv, .cons (rc_pred hl hk)
        (by simp [RustSemantics.bindPattern.eq_2]) .nil, ?_⟩
      exact envRel_bind x .nat m _ hs (wt_of_inert _ .nat rfl) hrel

theorem kpre_held {c Γ kind renv ce β env env' k j}
    (h : KPre c Γ kind renv ce β env env') (hh : heldOK kind (.generated k j) = true) :
    KPre c Γ kind ((.generated k j, .bool β) :: renv) (.move (.generated k j)) β env env' := by
  have hread : RC c ((.generated k j, .bool β) :: renv) (.move (.generated k j)) (.value (.bool β)) :=
    rc_read rfl (rlookup_cons_self _ _ _)
  cases kind with
  | plain => exact ⟨hread, h.2⟩
  | nat hv x s =>
    obtain ⟨hs, k', hl, _, hβ, he⟩ := h
    simp only [heldOK, Bool.not_eq_true'] at hh
    exact ⟨hs, k', by rw [rlookup_cons_other hh]; exact hl, hread, hβ, he⟩

section rules
variable {p : Program} {c : RCrate} {A : Flags}

/-- A block's semantics as one outcome of the whole block. -/
theorem sem_b_rcb {n Γ fl fm e t lets tail env renv o}
    (h : Sem p c A n Γ fl (.b fm e t lets tail)) (hrel : EnvRel p c A Γ env renv)
    (ho : obs (TargetSemantics.eval n p env e) = some o) (hst : o ≠ .stuck) :
    (∀ v, o = .value v → WT p c A v t) ∧ ∃ ro, RealizesM fm fl o ro ∧ RCB c renv ⟨lets, tail⟩ ro := by
  obtain ⟨hw, hcase⟩ := h env renv hrel o ho hst
  refine ⟨hw, ?_⟩
  rcases hcase with ⟨new, _, hl, ro, hro, hrc⟩ | ⟨rfl, hh, hr, hraise⟩
  · exact ⟨ro, hro, block_of_lets hl hrc⟩
  · exact ⟨hh, hr, block_of_raise hraise⟩

/-- A branch rendered as a Boolean literal computes it. -/
theorem lit_branch {n Γ fl e b0 env renv o}
    (h : Sem p c A n Γ fl (.b false e .bool [] (.lit (.bool b0)))) (hrel : EnvRel p c A Γ env renv)
    (ho : obs (TargetSemantics.eval n p env e) = some o) (hst : o ≠ .stuck) : o = .value (.bool b0) := by
  obtain ⟨_, hcase⟩ := h env renv hrel o ho hst
  rcases hcase with ⟨new, _, hl, ro, hro, hrc⟩ | ⟨rfl, hh, hr, hraise⟩
  · obtain ⟨v, hv, rfl⟩ := rc_lit_inv hrc (by intro h; subst h; cases o <;> simp [RealizesM, Realizes] at hro)
    simp [RustSemantics.literal] at hv; subst hv
    cases o <;> simp [RealizesM, Realizes] at hro
    rw [hro]
  · cases hraise

/-- The branch a conditional takes, as the Rust `if` runs it. -/
theorem k_branch {n Γ fl fm kind ce' a b t la ta lb tb env renv0 β env' o}
    (ha : Sem p c A n Γ fl (.b fm a t la ta)) (hb : Sem p c A n (elseCtx kind Γ) fl (.b fm b t lb tb))
    (hrel : EnvRel p c A Γ env renv0) (hpre : KPre c Γ kind renv0 ce' β env env')
    (ho : obs (TargetSemantics.eval n p env' (if β then a else b)) = some o) (hst : o ≠ .stuck) :
    (∀ v, o = .value v → WT p c A v t) ∧ ∃ ro, RealizesM fm fl o ro ∧
      RC c renv0 (.cond ce' (foldB ⟨la, ta⟩) (foldB ⟨elseLets kind ++ lb, tb⟩)) ro := by
  have hc := kpre_cond hpre
  cases β with
  | true =>
    have he := kpre_then hpre; subst he
    simp only [if_true] at ho
    obtain ⟨hw, ro, hro, hrcb⟩ := sem_b_rcb ha hrel ho hst
    exact ⟨hw, ro, hro, rc_cond hc (by simpa using foldB_sound hrcb)⟩
  | false =>
    obtain ⟨renv', hl, hrel'⟩ := kpre_else hrel hpre
    simp only [Bool.false_eq_true, if_false] at ho
    obtain ⟨hw, ro, hro, hrcb⟩ := sem_b_rcb hb hrel' ho hst
    exact ⟨hw, ro, hro, rc_cond hc (by simpa using foldB_sound (rcb_of_lets hl hrcb))⟩

theorem sem_kIf {n Γ fl fm kind ce a b t la ta lb tb}
    (ha : Sem p c A n Γ fl (.b fm a t la ta)) (hb : Sem p c A n (elseCtx kind Γ) fl (.b fm b t lb tb)) :
    Sem p c A n Γ fl (.k kind fm ce a b t []
      (.cond ce (foldB ⟨la, ta⟩) (foldB ⟨elseLets kind ++ lb, tb⟩))) := by
  intro env renv hrel β env' hpre o ho hst
  obtain ⟨hw, ro, hro, hrc⟩ := k_branch ha hb hrel hpre ho hst
  exact ⟨hw, .inl ⟨[], fresh_nil, .nil, ro, hro, by simpa using hrc⟩⟩

theorem sem_kHeld {n Γ fl fm kind ce a b t la ta lb tb k j ty}
    (ha : Sem p c A n Γ fl (.b fm a t la ta)) (hb : Sem p c A n (elseCtx kind Γ) fl (.b fm b t lb tb))
    (hk : k ≠ .binding) (hh : heldOK kind (.generated k j) = true) :
    Sem p c A n Γ fl (.k kind fm ce a b t [.mk (.bind (.generated k j)) ty ce]
      (.cond (.move (.generated k j)) (foldB ⟨la, ta⟩) (foldB ⟨elseLets kind ++ lb, tb⟩))) := by
  intro env renv hrel β env' hpre o ho hst
  have hrel1 := envRel_temp k j (.bool β) hk hrel
  obtain ⟨hw, ro, hro, hrc⟩ := k_branch ha hb hrel1 (kpre_held hpre hh) ho hst
  refine ⟨hw, .inl ⟨[(.generated k j, .bool β)], by simpa using fresh_temp (new := []) (v := .bool β) (j := j) hk fresh_nil,
    .cons (kpre_cond hpre) (by simp [RustSemantics.bindPattern.eq_2]) .nil, ro, hro, by simpa using hrc⟩⟩

theorem sem_kSame {n Γ fl fm kind ce a b t lets tail ty}
    (ha : Sem p c A n Γ fl (.b fm a t lets tail))
    (hb : Sem p c A n (elseCtx kind Γ) fl (.b fm b t lets tail)) (he : elseLets kind = []) :
    Sem p c A n Γ fl (.k kind fm ce a b t (.mk .wild ty ce :: lets) tail) := by
  intro env renv hrel β env' hpre o ho hst
  have hc := kpre_cond hpre
  have hbind : RustSemantics.bindPattern .wild (.bool β) renv = some renv := by
    simp [RustSemantics.bindPattern.eq_1]
  have conv : ∀ {Γ'}, (Fresh Γ' = Fresh Γ' → True) →
      ((∀ v, o = .value v → WT p c A v t) ∧
        ((∃ new, Fresh Γ' new ∧ LetsOK c renv lets (new ++ renv) ∧
          ∃ ro, RealizesM fm fl o ro ∧ RC c (new ++ renv) tail ro) ∨
        (o = .overflow ∧ ∃ h, RealizesM fm fl .overflow h ∧ LetsRaise c renv lets h))) →
      (∀ {new}, Fresh Γ' new → Fresh Γ new) →
      (∀ v, o = .value v → WT p c A v t) ∧
        ((∃ new, Fresh Γ new ∧ LetsOK c renv (.mk .wild ty ce :: lets) (new ++ renv) ∧
          ∃ ro, RealizesM fm fl o ro ∧ RC c (new ++ renv) tail ro) ∨
        (o = .overflow ∧ ∃ h, RealizesM fm fl .overflow h ∧ LetsRaise c renv (.mk .wild ty ce :: lets) h)) := by
    intro Γ' _ hres hfr
    obtain ⟨hw, hcase⟩ := hres
    refine ⟨hw, ?_⟩
    rcases hcase with ⟨new, hnew, hl, ro, hro, hrc⟩ | ⟨rfl, h, hrh, hraise⟩
    · exact .inl ⟨new, hfr hnew, .cons hc hbind hl, ro, hro, hrc⟩
    · exact .inr ⟨rfl, h, hrh, .later hc hbind hraise⟩
  cases β with
  | true =>
    have he' := kpre_then hpre; rw [he'] at ho
    simp only [if_true] at ho
    exact conv (fun _ => trivial) (ha env renv hrel o ho hst) id
  | false =>
    obtain ⟨renv', hl, hrel'⟩ := kpre_else hrel hpre
    rw [he] at hl; cases letsOK_nil hl
    simp only [Bool.false_eq_true, if_false] at ho
    exact conv (fun _ => trivial) (hb env' renv hrel' o ho hst) fresh_elseCtx

theorem sem_kLit {n Γ fl kind ce a b b0 tail}
    (ha : Sem p c A n Γ fl (.b false a .bool [] (.lit (.bool b0))))
    (hb : Sem p c A n (elseCtx kind Γ) fl (.b false b .bool [] (.lit (.bool !b0))))
    (htail : ∀ renv β, RC c renv ce (.value (.bool β)) → RC c renv tail (.value (.bool (if b0 then β else !β)))) :
    Sem p c A n Γ fl (.k kind false ce a b .bool [] tail) := by
  intro env renv hrel β env' hpre o ho hst
  have hc := kpre_cond hpre
  have hv : o = .value (.bool (if b0 then β else !β)) := by
    cases β with
    | true =>
      have he' := kpre_then hpre; subst he'
      simp only [if_true] at ho
      rw [lit_branch ha hrel ho hst]; cases b0 <;> rfl
    | false =>
      obtain ⟨renv', hl, hrel'⟩ := kpre_else hrel hpre
      simp only [Bool.false_eq_true, if_false] at ho
      rw [lit_branch hb hrel' ho hst]; cases b0 <;> rfl
  subst hv
  exact ⟨fun v _ => wt_of_inert v .bool rfl, .inl ⟨[], fresh_nil, .nil,
    .value (.bool (if b0 then β else !β)), rfl, by simpa using htail renv β hc⟩⟩

theorem sem_kId {n Γ fl kind ce a b}
    (ha : Sem p c A n Γ fl (.b false a .bool [] (.lit (.bool true))))
    (hb : Sem p c A n (elseCtx kind Γ) fl (.b false b .bool [] (.lit (.bool false)))) :
    Sem p c A n Γ fl (.k kind false ce a b .bool [] ce) :=
  sem_kLit (b0 := true) ha hb (fun _ _ h => by simpa using h)

theorem sem_kNeg {n Γ fl kind ce a b}
    (ha : Sem p c A n Γ fl (.b false a .bool [] (.lit (.bool false))))
    (hb : Sem p c A n (elseCtx kind Γ) fl (.b false b .bool [] (.lit (.bool true)))) :
    Sem p c A n Γ fl (.k kind false ce a b .bool [] (negateR ce)) :=
  sem_kLit (b0 := false) ha hb (fun _ _ h => by simpa using rc_negate h)

theorem eval_cond {n env c0 a b} :
    TargetSemantics.eval (n+1) p env (.cond c0 a b) =
      match TargetSemantics.eval n p env c0 with
      | .value (.bool β) st => TargetSemantics.addSteps (TargetSemantics.eval n p env (if β then a else b)) (st + 1)
      | .value _ _ => .stuck
      | .overflow st => .overflow (st + 1)
      | .stuck => .stuck
      | .exhausted => .exhausted := by
  rw [TargetSemantics.eval.eq_5]
  cases TargetSemantics.eval n p env c0 with
  | value v st => cases v <;> try rfl
                  rename_i β; cases β <;> rfl
  | _ => rfl

theorem sem_cond {n Γ fl fm c0 a b t lc ce lk tk}
    (hc : Sem p c A n Γ fl (.b false c0 .bool lc ce)) (hk : Sem p c A n Γ fl (.k .plain fm ce a b t lk tk))
    (hkh : ∀ renv h, Halt h → RC c renv ce h → (lk = [] ∧ RC c renv tk h) ∨ LetsRaise c renv lk h) :
    Sem p c A (n+1) Γ fl (.b fm (.cond c0 a b) t (lc ++ lk) tk) := by
  intro env renv hrel o ho hst
  rw [eval_cond] at ho
  cases h1 : TargetSemantics.eval n p env c0 with
  | value v s1 =>
    rw [h1] at ho
    obtain ⟨_, hcase⟩ := hc env renv hrel (.value v) (by rw [h1]; rfl) nofun
    rcases hcase with ⟨new, hnew, hl, ro, hro, hrc⟩ | ⟨h, _⟩
    · cases ro with
      | value w =>
        simp only [RealizesM, Realizes] at hro; subst hro
        cases w with
        | bool β =>
          simp only [obs_addSteps] at ho
          obtain ⟨hw, hcase2⟩ := hk env (new ++ renv) (envRel_fresh hnew hrel) β env ⟨hrc, rfl⟩ o ho hst
          refine ⟨hw, ?_⟩
          rcases hcase2 with ⟨new2, hnew2, hl2, ro2, hro2, hrc2⟩ | ⟨rfl, h2, hrh2, hraise2⟩
          · refine .inl ⟨new2 ++ new, fresh_append hnew2 hnew, ?_, ro2, hro2, by simpa using hrc2⟩
            simpa using hl.append hl2
          · exact .inr ⟨rfl, h2, hrh2, LetsRaise.append_right hl hraise2⟩
        | _ => simp [obs] at ho; subst ho; exact absurd rfl hst
      | _ => simp [RealizesM, Realizes] at hro
    · cases h
  | overflow s1 =>
    rw [h1] at ho
    simp only [obs, Option.some.injEq] at ho; subst ho
    obtain ⟨_, hcase⟩ := hc env renv hrel .overflow (by rw [h1]; rfl) nofun
    refine ⟨(fun _ h => by cases h), ?_⟩
    rcases hcase with ⟨new, hnew, hl, ro, hro, hrc⟩ | ⟨_, h, hrh, hraise⟩
    · have hro' : Realizes fl .overflow ro := hro
      rcases hkh _ ro (realizes_halt hro') hrc with ⟨rfl, hrt⟩ | hraise2
      · exact .inl ⟨new, hnew, by simpa using hl, ro, realizesM_overflow hro', hrt⟩
      · exact .inr ⟨rfl, ro, realizesM_overflow hro', LetsRaise.append_right hl hraise2⟩
    · exact .inr ⟨rfl, h, realizesM_overflow hrh, LetsRaise.append_left hraise⟩
  | stuck => rw [h1] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
  | exhausted => rw [h1] at ho; simp [obs] at ho

theorem sem_condId {n Γ fl fm c0 a b lc tc}
    (hc : Sem p c A n Γ fl (.b fm c0 .bool lc tc))
    (ha : Sem p c A n Γ fl (.b false a .bool [] (.lit (.bool true))))
    (hb : Sem p c A n Γ fl (.b false b .bool [] (.lit (.bool false)))) :
    Sem p c A (n+1) Γ fl (.b fm (.cond c0 a b) .bool lc tc) := by
  intro env renv hrel o ho hst
  rw [eval_cond] at ho
  cases h1 : TargetSemantics.eval n p env c0 with
  | value v s1 =>
    rw [h1] at ho
    cases v with
    | bool β =>
      simp only [obs_addSteps] at ho
      have hv : o = .value (.bool β) := by
        cases β with
        | true => simpa using lit_branch ha hrel (by simpa using ho) hst
        | false => simpa using lit_branch hb hrel (by simpa using ho) hst
      subst hv
      exact hc env renv hrel _ (by rw [h1]; rfl) hst
    | _ => simp [obs] at ho; subst ho; exact absurd rfl hst
  | overflow s1 =>
    rw [h1] at ho
    simp only [obs, Option.some.injEq] at ho; subst ho
    exact hc env renv hrel .overflow (by rw [h1]; rfl) nofun
  | stuck => rw [h1] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
  | exhausted => rw [h1] at ho; simp [obs] at ho

end rules
end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

/-! The calculus side of a match. -/

/-- A match that is neither stuck nor exhausted took an arm: the first whose
shape destructs the value. -/
theorem evalArms_find {p env v} : ∀ (m : Nat) (arms : List Arm) (o : Obs),
    obs (TargetSemantics.evalArms m p env v arms) = some o → o ≠ .stuck →
    ∃ (j : Nat) (shape : Shape) (xs : List Nat) (body : Expr) (fields : List Value)
      (env' : List (Nat × Value)) (m' : Nat), m' < m ∧ arms[j]? = some (Arm.arm shape xs body) ∧
      TargetSemantics.destruct shape v = some fields ∧ TargetSemantics.bindAll xs fields env = some env' ∧
      obs (TargetSemantics.eval m' p env' body) = some o ∧
      ∀ (i : Nat) (a : Arm), i < j → arms[i]? = some a → TargetSemantics.destruct (armShape a) v = none
  | 0, _, _, h, _ => by simp [TargetSemantics.evalArms.eq_1, obs] at h
  | m+1, [], o, h, hs => by
    simp [TargetSemantics.evalArms.eq_2, obs] at h; subst h; exact absurd rfl hs
  | m+1, .arm shape xs body :: rest, o, h, hs => by
    rw [TargetSemantics.evalArms.eq_3] at h
    cases hd : TargetSemantics.destruct shape v with
    | none =>
      rw [hd] at h; simp only [obs_addSteps] at h
      obtain ⟨j, sh, ys, bd, fs, env', m', hm, hj, hd', hb, ho, hprev⟩ := evalArms_find m rest o h hs
      refine ⟨j + 1, sh, ys, bd, fs, env', m', by omega, by simpa using hj, hd', hb, ho, ?_⟩
      intro i a hi ha
      cases i with
      | zero => simp at ha; subst ha; exact hd
      | succ i => exact hprev i a (by omega) (by simpa using ha)
    | some fields =>
      rw [hd] at h; simp only at h
      cases hb : TargetSemantics.bindAll xs fields env with
      | none => rw [hb] at h; simp [obs] at h; subst h; exact absurd rfl hs
      | some env' =>
        rw [hb] at h; simp only [obs_addSteps] at h
        exact ⟨0, shape, xs, body, fields, env', m, by omega, rfl, hd, hb, h, fun i _ hi => by omega⟩

theorem natArms_spec {arms a x b} (h : natArms arms = some (a, x, b)) :
    arms = [.arm .zero [] a, .arm .succ [x] b] ∨ arms = [.arm .succ [x] b, .arm .zero [] a] := by
  unfold natArms at h
  split at h
  · simp only [Option.some.injEq, Prod.mk.injEq] at h; obtain ⟨rfl, rfl, rfl⟩ := h; exact .inl rfl
  · simp only [Option.some.injEq, Prod.mk.injEq] at h; obtain ⟨rfl, rfl, rfl⟩ := h; exact .inr rfl
  · cases h

theorem boolArms_spec {arms a b} (h : boolArms arms = some (a, b)) :
    arms = [.arm .true [] a, .arm .false [] b] ∨ arms = [.arm .false [] b, .arm .true [] a] := by
  unfold boolArms at h
  split at h
  · simp only [Option.some.injEq, Prod.mk.injEq] at h; obtain ⟨rfl, rfl⟩ := h; exact .inl rfl
  · simp only [Option.some.injEq, Prod.mk.injEq] at h; obtain ⟨rfl, rfl⟩ := h; exact .inr rfl
  · cases h

/-- The arm a match on a natural takes. -/
theorem natArms_eval {p env v arms a x b m o} (h : natArms arms = some (a, x, b))
    (ho : obs (TargetSemantics.evalArms m p env v arms) = some o) (hs : o ≠ .stuck) :
    ∃ k m', m' < m ∧ v = .nat k ∧
      obs (TargetSemantics.eval m' p (if Nat.beq k 0 then env else (x, .nat (k - 1)) :: env)
        (if Nat.beq k 0 then a else b)) = some o := by
  obtain ⟨j, shape, xs, body, fields, env', m', hm, hj, hd, hb, hbody, _⟩ := evalArms_find m arms o ho hs
  have zero_case : ∀ k, TargetSemantics.destruct .zero v = some fields → v = .nat k →
      TargetSemantics.bindAll [] fields env = some env' → Nat.beq k 0 = true ∧ env' = env := by
    intro k hd hv hb; subst hv
    simp only [TargetSemantics.destruct] at hd
    cases hk : Nat.beq k 0 <;> simp [hk] at hd
    subst hd; simp [TargetSemantics.bindAll] at hb; exact ⟨rfl, hb.symm⟩
  have succ_case : ∀ k, TargetSemantics.destruct .succ v = some fields → v = .nat k →
      TargetSemantics.bindAll [x] fields env = some env' →
      Nat.beq k 0 = false ∧ env' = (x, .nat (k - 1)) :: env := by
    intro k hd hv hb; subst hv
    simp only [TargetSemantics.destruct] at hd
    cases hk : Nat.beq k 0 <;> simp [hk] at hd
    subst hd; simp [TargetSemantics.bindAll] at hb
    exact ⟨rfl, by rw [← hb]; rfl⟩
  have hnat : ∀ s, (s = Shape.zero ∨ s = Shape.succ) → TargetSemantics.destruct s v = some fields →
      ∃ k, v = .nat k := by
    intro s hs hd
    rcases hs with rfl | rfl <;> cases v <;> simp [TargetSemantics.destruct] at hd <;> exact ⟨_, rfl⟩
  rcases natArms_spec h with rfl | rfl <;>
    rcases j with _ | _ | j <;> simp at hj <;> obtain ⟨rfl, rfl, rfl⟩ := hj
  · obtain ⟨k, rfl⟩ := hnat _ (.inl rfl) hd
    obtain ⟨hk, rfl⟩ := zero_case k hd rfl hb
    exact ⟨k, m', hm, rfl, by simpa [hk] using hbody⟩
  · obtain ⟨k, rfl⟩ := hnat _ (.inr rfl) hd
    obtain ⟨hk, rfl⟩ := succ_case k hd rfl hb
    exact ⟨k, m', hm, rfl, by simpa [hk] using hbody⟩
  · obtain ⟨k, rfl⟩ := hnat _ (.inr rfl) hd
    obtain ⟨hk, rfl⟩ := succ_case k hd rfl hb
    exact ⟨k, m', hm, rfl, by simpa [hk] using hbody⟩
  · obtain ⟨k, rfl⟩ := hnat _ (.inl rfl) hd
    obtain ⟨hk, rfl⟩ := zero_case k hd rfl hb
    exact ⟨k, m', hm, rfl, by simpa [hk] using hbody⟩

/-- The arm a match on a Boolean takes. -/
theorem boolArms_eval {p env v arms a b m o} (h : boolArms arms = some (a, b))
    (ho : obs (TargetSemantics.evalArms m p env v arms) = some o) (hs : o ≠ .stuck) :
    ∃ β m', m' < m ∧ v = .bool β ∧ obs (TargetSemantics.eval m' p env (if β then a else b)) = some o := by
  obtain ⟨j, shape, xs, body, fields, env', m', hm, hj, hd, hb, hbody, _⟩ := evalArms_find m arms o ho hs
  rcases boolArms_spec h with rfl | rfl <;>
    rcases j with _ | _ | j <;> simp at hj <;> obtain ⟨rfl, rfl, rfl⟩ := hj <;>
    cases v <;> simp [TargetSemantics.destruct] at hd <;>
    obtain ⟨rfl, rfl⟩ := hd <;>
    simp [TargetSemantics.bindAll] at hb <;> subst hb <;>
    first | exact ⟨true, m', hm, rfl, by simpa using hbody⟩ | exact ⟨false, m', hm, rfl, by simpa using hbody⟩

end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

section rules
variable {p : Program} {c : RCrate} {A : Flags}

theorem eval_match {n env τ s0 arms} :
    TargetSemantics.eval (n+1) p env (.match τ s0 arms) =
      match TargetSemantics.eval n p env s0 with
      | .value v st => TargetSemantics.addSteps (TargetSemantics.evalArms n p env v arms) (st + 1)
      | .overflow st => .overflow (st + 1)
      | .stuck => .stuck
      | .exhausted => .exhausted := by
  rw [TargetSemantics.eval.eq_6]
  cases TargetSemantics.eval n p env s0 <;> rfl

/-- A match whose scrutinee the rendering holds in `h` first. -/
theorem sem_matchHolder {n Γ fl fm τ s0 arms st rs k j ty lets tail}
    (hs : Sem p c A n Γ fl (.e s0 st rs)) (hk : k ≠ .binding)
    (hcont : ∀ env renv v o, EnvRel p c A Γ env renv → WT p c A v st →
      RustSemantics.lookup renv (.generated k j) = some v →
      obs (TargetSemantics.evalArms n p env v arms) = some o → o ≠ .stuck →
      (∀ w, o = .value w → WT p c A w τ) ∧
      ((∃ new, Fresh Γ new ∧ LetsOK c renv lets (new ++ renv) ∧
          ∃ ro, RealizesM fm fl o ro ∧ RC c (new ++ renv) tail ro) ∨
        (o = .overflow ∧ ∃ h, RealizesM fm fl .overflow h ∧ LetsRaise c renv lets h))) :
    Sem p c A (n+1) Γ fl (.b fm (.match τ s0 arms) τ (.mk (.bind (.generated k j)) ty rs :: lets) tail) := by
  intro env renv hrel o ho hst
  rw [eval_match] at ho
  cases h1 : TargetSemantics.eval n p env s0 with
  | value v s1 =>
    rw [h1] at ho
    simp only [obs_addSteps] at ho
    obtain ⟨hw1, ro1, hro1, hrc1⟩ := hs env renv hrel (.value v) (by rw [h1]; rfl) nofun
    cases ro1 with
    | value w =>
      simp only [Realizes] at hro1; subst hro1
      have hrel1 := envRel_temp k j w hk hrel
      have hbind : RustSemantics.bindPattern (.bind (.generated k j)) w renv =
          some ((.generated k j, w) :: renv) := by simp [RustSemantics.bindPattern.eq_2]
      obtain ⟨hwt, hcase⟩ := hcont env _ w o hrel1 (hw1 w rfl) (rlookup_cons_self _ _ _) ho hst
      refine ⟨hwt, ?_⟩
      rcases hcase with ⟨new, hnew, hl, ro, hro, hrc⟩ | ⟨rfl, h, hrh, hraise⟩
      · refine .inl ⟨new ++ [(.generated k j, w)], fresh_temp hk hnew, ?_, ro, hro, by simpa using hrc⟩
        simp only [List.append_assoc, List.singleton_append]; exact .cons hrc1 hbind hl
      · exact .inr ⟨rfl, h, hrh, .later hrc1 hbind hraise⟩
    | _ => simp [Realizes] at hro1
  | overflow s1 =>
    rw [h1] at ho
    simp only [obs, Option.some.injEq] at ho; subst ho
    obtain ⟨_, ro1, hro1, hrc1⟩ := hs env renv hrel .overflow (by rw [h1]; rfl) nofun
    exact ⟨(fun _ h => by cases h), .inr ⟨rfl, ro1, realizesM_overflow hro1, .here hrc1 (realizes_halt hro1)⟩⟩
  | stuck => rw [h1] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
  | exhausted => rw [h1] at ho; simp [obs] at ho

theorem sem_matchNat {n Γ fl fm τ s0 arms rs a x b sl k j ty lets tail}
    (hs : Sem p c A n Γ fl (.e s0 .nat rs)) (harms : natArms arms = some (a, x, b))
    (hsl : slotOK Γ sl = true) (hk : k ≠ .binding)
    (hkd : ∀ m ≤ n, Sem p c A m Γ fl
      (.k (.nat (.generated k j) x sl) fm (.isZero (.generated k j)) a b τ lets tail)) :
    Sem p c A (n+1) Γ fl (.b fm (.match τ s0 arms) τ (.mk (.bind (.generated k j)) ty rs :: lets) tail) := by
  refine sem_matchHolder hs hk ?_
  intro env renv v o hrel _ hl ho hst
  obtain ⟨k', m', hm, rfl, ho'⟩ := natArms_eval harms ho hst
  exact hkd m' (by omega) env renv hrel _ _ ⟨hsl, k', hl, rc_isZero hl, rfl, rfl⟩ o ho' hst

theorem sem_matchBool {n Γ fl fm τ s0 arms rs a b k j ty lets tail}
    (hs : Sem p c A n Γ fl (.e s0 .bool rs)) (harms : boolArms arms = some (a, b)) (hk : k ≠ .binding)
    (hkd : ∀ m ≤ n, Sem p c A m Γ fl (.k .plain fm (.move (.generated k j)) a b τ lets tail)) :
    Sem p c A (n+1) Γ fl (.b fm (.match τ s0 arms) τ (.mk (.bind (.generated k j)) ty rs :: lets) tail) := by
  refine sem_matchHolder hs hk ?_
  intro env renv v o hrel _ hl ho hst
  obtain ⟨β, m', hm, rfl, ho'⟩ := boolArms_eval harms ho hst
  exact hkd m' (by omega) env renv hrel β env ⟨rc_read rfl hl, rfl⟩ o ho' hst

theorem sem_matchBoolId {n Γ fl fm s0 arms a b lets tail}
    (hs : Sem p c A n Γ fl (.b fm s0 .bool lets tail)) (harms : boolArms arms = some (a, b))
    (ha : ∀ m ≤ n, Sem p c A m Γ fl (.b false a .bool [] (.lit (.bool true))))
    (hb : ∀ m ≤ n, Sem p c A m Γ fl (.b false b .bool [] (.lit (.bool false)))) :
    Sem p c A (n+1) Γ fl (.b fm (.match .bool s0 arms) .bool lets tail) := by
  intro env renv hrel o ho hst
  rw [eval_match] at ho
  cases h1 : TargetSemantics.eval n p env s0 with
  | value v s1 =>
    rw [h1] at ho
    simp only [obs_addSteps] at ho
    obtain ⟨β, m', hm, rfl, ho'⟩ := boolArms_eval harms ho hst
    have hv : o = .value (.bool β) := by
      cases β with
      | true => simpa using lit_branch (ha m' (by omega)) hrel (by simpa using ho') hst
      | false => simpa using lit_branch (hb m' (by omega)) hrel (by simpa using ho') hst
    subst hv
    exact hs env renv hrel _ (by rw [h1]; rfl) hst
  | overflow s1 =>
    rw [h1] at ho
    simp only [obs, Option.some.injEq] at ho; subst ho
    exact hs env renv hrel .overflow (by rw [h1]; rfl) nofun
  | stuck => rw [h1] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
  | exhausted => rw [h1] at ho; simp [obs] at ho

theorem sem_matchUnit {n Γ fl fm τ s0 body rs ty lets tail}
    (hs : Sem p c A n Γ fl (.e s0 .unit rs))
    (hbody : ∀ m ≤ n, Sem p c A m Γ fl (.b fm body τ lets tail)) :
    Sem p c A (n+1) Γ fl (.b fm (.match τ s0 [.arm .unit [] body]) τ (.mk .unit ty rs :: lets) tail) := by
  intro env renv hrel o ho hst
  rw [eval_match] at ho
  cases h1 : TargetSemantics.eval n p env s0 with
  | value v s1 =>
    rw [h1] at ho
    simp only [obs_addSteps] at ho
    obtain ⟨hw1, ro1, hro1, hrc1⟩ := hs env renv hrel (.value v) (by rw [h1]; rfl) nofun
    have hv : v = .unit := wt_unit (hw1 v rfl)
    subst hv
    cases ro1 with
    | value w =>
      simp only [Realizes] at hro1; subst hro1
      obtain ⟨j', shape, xs, bd, fields, env', m', hm, hj, hd, hb, hbody', _⟩ := evalArms_find n _ o ho hst
      rcases j' with _ | j' <;> simp at hj
      obtain ⟨rfl, rfl, rfl⟩ := hj
      simp [TargetSemantics.destruct] at hd; subst hd
      simp [TargetSemantics.bindAll] at hb; subst hb
      obtain ⟨hwt, hcase⟩ := hbody m' (by omega) env renv hrel o hbody' hst
      have hbind : RustSemantics.bindPattern .unit Value.unit renv = some renv := by
        simp [RustSemantics.bindPattern]
      refine ⟨hwt, ?_⟩
      rcases hcase with ⟨new, hnew, hl, ro, hro, hrc⟩ | ⟨rfl, h, hrh, hraise⟩
      · exact .inl ⟨new, hnew, .cons hrc1 hbind hl, ro, hro, hrc⟩
      · exact .inr ⟨rfl, h, hrh, .later hrc1 hbind hraise⟩
    | _ => simp [Realizes] at hro1
  | overflow s1 =>
    rw [h1] at ho
    simp only [obs, Option.some.injEq] at ho; subst ho
    obtain ⟨_, ro1, hro1, hrc1⟩ := hs env renv hrel .overflow (by rw [h1]; rfl) nofun
    exact ⟨(fun _ h => by cases h), .inr ⟨rfl, ro1, realizesM_overflow hro1, .here hrc1 (realizes_halt hro1)⟩⟩
  | stuck => rw [h1] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
  | exhausted => rw [h1] at ho; simp [obs] at ho

end rules
end LexLeanPreservation.Rust
