import LexLeanTarget.TargetSemantics
set_option linter.unusedSimpArgs false
namespace LexLeanPreservation
open LexLeanTarget.TargetSyntax LexLeanTarget.TargetSemantics

/-! The relations of semantic preservation (SPEC.md §17.17): the realization
calculus observed through fuel (`Obs`, `Conv`, `FunRel`), stability of the
fuelled evaluator, and one compatibility lemma per calculus construct. A
certificate composes these lemmas along the source term, so its proof is
derived from the source and checked against the lowered program. -/

theorem addSteps_ne {r : Outcome} {k : Nat} (h : addSteps r k ≠ .exhausted) : r ≠ .exhausted := by
  intro hr; subst hr; exact h rfl
theorem chargeResult_ne {r : Outcome} {k : Nat} (h : chargeResult r k ≠ .exhausted) : r ≠ .exhausted := by
  intro hr; subst hr; exact h rfl

theorem stable : ∀ n,
    (∀ p env e, eval n p env e ≠ .exhausted → eval (n+1) p env e = eval n p env e) ∧
    (∀ p env es, evalList n p env es ≠ .exhausted → evalList (n+1) p env es = evalList n p env es) ∧
    (∀ p env v arms, evalArms n p env v arms ≠ .exhausted → evalArms (n+1) p env v arms = evalArms n p env v arms) := by
  intro n
  induction n with
  | zero => exact ⟨fun _ _ _ h => absurd rfl h, fun _ _ _ h => absurd rfl h, fun _ _ _ _ h => absurd rfl h⟩
  | succ n ih =>
    obtain ⟨ihE, ihL, ihA⟩ := ih
    refine ⟨?_, ?_, ?_⟩
    · intro p env e h
      cases e with
      | value t v => rfl
      | var x => rfl
      | «let» x t b body =>
        simp only [eval.eq_4] at h ⊢
        cases hb : eval n p env b with
        | value bv bs =>
          rw [ihE _ _ _ (by rw [hb]; exact nofun), hb]
          rw [hb] at h
          simp only at h ⊢
          rw [ihE _ _ _ (addSteps_ne h)]
        | exhausted => rw [hb] at h; exact absurd rfl h
        | _ => rw [ihE _ _ _ (by rw [hb]; exact nofun), hb]
      | cond c t f =>
        simp only [eval.eq_5] at h ⊢
        cases hb : eval n p env c with
        | value cv cs =>
          rw [ihE _ _ _ (by rw [hb]; exact nofun), hb]
          rw [hb] at h
          cases cv with
          | bool flag =>
            cases flag
            · simp only [Bool.false_eq_true, if_false] at h ⊢
              rw [ihE _ _ _ (addSteps_ne h)]
            · simp only [if_true] at h ⊢
              rw [ihE _ _ _ (addSteps_ne h)]
          | _ => rfl
        | exhausted => rw [hb] at h; exact absurd rfl h
        | _ => rw [ihE _ _ _ (by rw [hb]; exact nofun), hb]
      | «match» t s arms =>
        simp only [eval.eq_6] at h ⊢
        cases hb : eval n p env s with
        | value sv ss =>
          rw [ihE _ _ _ (by rw [hb]; exact nofun), hb]
          rw [hb] at h
          simp only at h ⊢
          rw [ihA _ _ _ _ (addSteps_ne h)]
        | exhausted => rw [hb] at h; exact absurd rfl h
        | _ => rw [ihE _ _ _ (by rw [hb]; exact nofun), hb]
      | build shape t ops =>
        simp only [eval.eq_7] at h ⊢
        cases hb : evalList n p env ops with
        | exhausted => rw [hb] at h; exact absurd rfl h
        | _ => rw [ihL _ _ _ (by rw [hb]; exact nofun), hb]
      | call callee ops =>
        simp only [eval.eq_8] at h ⊢
        cases hb : evalList n p env ops with
        | values vs ss =>
          rw [ihL _ _ _ (by rw [hb]; exact nofun), hb]
          rw [hb] at h
          simp only at h ⊢
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
              rw [ihE _ _ _ (addSteps_ne h)]
        | exhausted => rw [hb] at h; exact absurd rfl h
        | _ => rw [ihL _ _ _ (by rw [hb]; exact nofun), hb]
      | closure callee caps =>
        simp only [eval.eq_9] at h ⊢
        cases hb : evalList n p env caps with
        | exhausted => rw [hb] at h; exact absurd rfl h
        | _ => rw [ihL _ _ _ (by rw [hb]; exact nofun), hb]
      | apply t ops =>
        simp only [eval.eq_10] at h ⊢
        cases hb : eval n p env t with
        | value tv ts =>
          rw [ihE _ _ _ (by rw [hb]; exact nofun), hb]
          rw [hb] at h
          cases tv with
          | closure callee captured =>
            simp only at h ⊢
            cases hl : evalList n p env ops with
            | values vs ss =>
              rw [ihL _ _ _ (by rw [hl]; exact nofun), hl]
              rw [hl] at h
              simp only at h ⊢
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
                  rw [ihE _ _ _ (addSteps_ne h)]
            | exhausted => rw [hl] at h; exact absurd rfl h
            | _ => rw [ihL _ _ _ (by rw [hl]; exact nofun), hl]
          | _ => rfl
        | exhausted => rw [hb] at h; exact absurd rfl h
        | _ => rw [ihE _ _ _ (by rw [hb]; exact nofun), hb]
      | prim op ops =>
        simp only [eval.eq_11] at h ⊢
        cases hb : evalList n p env ops with
        | exhausted => rw [hb] at h; exact absurd rfl h
        | _ => rw [ihL _ _ _ (by rw [hb]; exact nofun), hb]
      | first i =>
        simp only [eval.eq_12] at h ⊢
        cases hb : eval n p env i with
        | exhausted => rw [hb] at h; exact absurd rfl h
        | _ => rw [ihE _ _ _ (by rw [hb]; exact nofun), hb]
      | second i =>
        simp only [eval.eq_13] at h ⊢
        cases hb : eval n p env i with
        | exhausted => rw [hb] at h; exact absurd rfl h
        | _ => rw [ihE _ _ _ (by rw [hb]; exact nofun), hb]
      | field i k =>
        simp only [eval.eq_14] at h ⊢
        cases hb : eval n p env i with
        | exhausted => rw [hb] at h; exact absurd rfl h
        | _ => rw [ihE _ _ _ (by rw [hb]; exact nofun), hb]
    · intro p env es h
      cases es with
      | nil => rfl
      | cons e rest =>
        simp only [evalList.eq_3] at h ⊢
        cases hb : eval n p env e with
        | value v s =>
          rw [ihE _ _ _ (by rw [hb]; exact nofun), hb]
          rw [hb] at h
          simp only at h ⊢
          cases hl : evalList n p env rest with
          | exhausted => rw [hl] at h; exact absurd rfl h
          | _ => rw [ihL _ _ _ (by rw [hl]; exact nofun), hl]
        | exhausted => rw [hb] at h; exact absurd rfl h
        | _ => rw [ihE _ _ _ (by rw [hb]; exact nofun), hb]
    · intro p env v arms h
      cases arms with
      | nil => rfl
      | cons a rest =>
        cases a with
        | arm shape binders body =>
          simp only [evalArms.eq_3] at h ⊢
          split
          · rename_i hd
            rw [hd] at h
            simp only at h
            rw [ihA _ _ _ _ (addSteps_ne h)]
          · rename_i fields hd
            rw [hd] at h
            simp only at h ⊢
            split
            · rfl
            · rename_i ext hext
              rw [hext] at h
              simp only at h
              rw [ihE _ _ _ (addSteps_ne h)]

theorem eval_le {n m : Nat} {p env e} (h : eval n p env e ≠ .exhausted) (hle : n ≤ m) :
    eval m p env e = eval n p env e := by
  induction hle with
  | refl => rfl
  | step _ ih => rw [(stable _).1 _ _ _ (by rw [ih]; exact h), ih]
theorem evalList_le {n m : Nat} {p env es} (h : evalList n p env es ≠ .exhausted) (hle : n ≤ m) :
    evalList m p env es = evalList n p env es := by
  induction hle with
  | refl => rfl
  | step _ ih => rw [(stable _).2.1 _ _ _ (by rw [ih]; exact h), ih]
theorem evalArms_le {n m : Nat} {p env v arms} (h : evalArms n p env v arms ≠ .exhausted) (hle : n ≤ m) :
    evalArms m p env v arms = evalArms n p env v arms := by
  induction hle with
  | refl => rfl
  | step _ ih => rw [(stable _).2.2 _ _ _ _ (by rw [ih]; exact h), ih]

inductive Obs where
  | value (v : Value)
  | overflow
  | stuck

inductive ObsL where
  | values (vs : List Value)
  | overflow
  | stuck

def obs : Outcome → Option Obs
  | .value v _ => some (.value v)
  | .overflow _ => some .overflow
  | .stuck => some .stuck
  | .exhausted => none

def obsL : Outcomes → Option ObsL
  | .values vs _ => some (.values vs)
  | .overflow _ => some .overflow
  | .stuck => some .stuck
  | .exhausted => none

def Rel (fits : Bool) (v : Value) : Obs := cond fits (.value v) .overflow
def RelL (fits : Bool) (vs : List Value) : ObsL := cond fits (.values vs) .overflow

def Conv (p : Program) (env : List (Nat × Value)) (e : Expr) (o : Obs) : Prop :=
  ∃ n, obs (eval n p env e) = some o
def ConvL (p : Program) (env : List (Nat × Value)) (es : List Expr) (o : ObsL) : Prop :=
  ∃ n, obsL (evalList n p env es) = some o
def ConvA (p : Program) (env : List (Nat × Value)) (v : Value) (arms : List Arm) (o : Obs) : Prop :=
  ∃ n, obs (evalArms n p env v arms) = some o
def FunRel (p : Program) (f : Nat) (args : List Value) (o : Obs) : Prop :=
  ∃ fn : Function, LexLeanRuntime.index p.functions f = some fn ∧
    ∃ env, bindAll fn.parameters args [] = some env ∧ Conv p env fn.body o
def RunConv (p : Program) (entry : Nat) (args : List Value) (o : Obs) : Prop :=
  ∃ n, obs (run n p entry args) = some o

theorem obs_ne {r : Outcome} {o : Obs} (h : obs r = some o) : r ≠ .exhausted := by
  intro hr; subst hr; cases h
theorem obsL_ne {r : Outcomes} {o : ObsL} (h : obsL r = some o) : r ≠ .exhausted := by
  intro hr; subst hr; cases h

theorem conv_at {p env e o} (h : Conv p env e o) : ∃ n, ∀ m, n ≤ m → obs (eval m p env e) = some o := by
  obtain ⟨n, hn⟩ := h
  exact ⟨n, fun m hm => by rw [eval_le (obs_ne hn) hm]; exact hn⟩
theorem convL_at {p env es o} (h : ConvL p env es o) : ∃ n, ∀ m, n ≤ m → obsL (evalList m p env es) = some o := by
  obtain ⟨n, hn⟩ := h
  exact ⟨n, fun m hm => by rw [evalList_le (obsL_ne hn) hm]; exact hn⟩
theorem convA_at {p env v arms o} (h : ConvA p env v arms o) : ∃ n, ∀ m, n ≤ m → obs (evalArms m p env v arms) = some o := by
  obtain ⟨n, hn⟩ := h
  exact ⟨n, fun m hm => by rw [evalArms_le (obs_ne hn) hm]; exact hn⟩

theorem obs_addSteps (r : Outcome) (k : Nat) : obs (addSteps r k) = obs r := by cases r <;> rfl
theorem obs_chargeResult (r : Outcome) (k : Nat) : obs (chargeResult r k) = obs r := by cases r <;> rfl

theorem conv_value {p env t v} : Conv p env (.value t v) (Rel true v) := ⟨1, rfl⟩
theorem conv_var {p env x v} (h : lookup env x = some v) : Conv p env (.var x) (Rel true v) :=
  ⟨1, by simp only [eval.eq_3, h]; rfl⟩

theorem convL_nil {p env} : ConvL p env [] (RelL true []) := ⟨1, rfl⟩
theorem convL_cons {p env e es f g v vs} (he : Conv p env e (Rel f v)) (hes : ConvL p env es (RelL g vs)) :
    ConvL p env (e :: es) (RelL (f && g) (v :: vs)) := by
  obtain ⟨n1, h1⟩ := conv_at he
  obtain ⟨n2, h2⟩ := convL_at hes
  refine ⟨n1 + n2 + 1, ?_⟩
  have e1 := h1 (n1 + n2) (by omega)
  have e2 := h2 (n1 + n2) (by omega)
  simp only [evalList.eq_3]
  cases f with
  | false =>
    cases hr : eval (n1 + n2) p env e <;> rw [hr] at e1 <;> simp [obs, obsL, Rel, RelL, cond] at e1 ⊢
  | true =>
    cases hr : eval (n1 + n2) p env e <;> rw [hr] at e1 <;> simp [obs, obsL, Rel, RelL, cond] at e1
    subst e1
    cases g with
    | false => cases hl : evalList (n1 + n2) p env es <;> rw [hl] at e2 <;> simp [obs, obsL, Rel, RelL, cond] at e2 ⊢
    | true =>
      cases hl : evalList (n1 + n2) p env es <;> rw [hl] at e2 <;> simp [obs, obsL, Rel, RelL, cond] at e2 ⊢
      exact e2

theorem conv_prim {p env op es f g vs w} (hes : ConvL p env es (RelL f vs))
    (hp : obs (primitive op vs) = some (Rel g w)) : Conv p env (.prim op es) (Rel (f && g) w) := by
  obtain ⟨n, h⟩ := convL_at hes
  refine ⟨n + 1, ?_⟩
  have e := h n (Nat.le_refl _)
  simp only [eval.eq_11]
  cases f with
  | false => cases hl : evalList n p env es <;> rw [hl] at e <;> simp [obs, obsL, Rel, RelL, cond] at e ⊢ <;> rfl
  | true =>
    cases hl : evalList n p env es with
    | values vs' s =>
      rw [hl] at e; simp [obsL, RelL, cond] at e; subst e
      show obs (chargeResult _ _) = _
      rw [obs_chargeResult]; exact hp
    | _ => rw [hl] at e; simp [obsL, RelL, cond] at e

theorem conv_build {p env shape t es f g vs w} (hes : ConvL p env es (RelL f vs))
    (hc : obs (construct shape vs) = some (Rel g w)) : Conv p env (.build shape t es) (Rel (f && g) w) := by
  obtain ⟨n, h⟩ := convL_at hes
  refine ⟨n + 1, ?_⟩
  have e := h n (Nat.le_refl _)
  simp only [eval.eq_7]
  cases f with
  | false => cases hl : evalList n p env es <;> rw [hl] at e <;> simp [obs, obsL, Rel, RelL, cond] at e ⊢ <;> rfl
  | true =>
    cases hl : evalList n p env es with
    | values vs' s =>
      rw [hl] at e; simp [obsL, RelL, cond] at e; subst e
      show obs (addSteps _ _) = _
      rw [obs_addSteps]; exact hc

    | _ => rw [hl] at e; simp [obsL, RelL, cond] at e

theorem conv_closure {p env fi es f vs} (hes : ConvL p env es (RelL f vs)) :
    Conv p env (.closure fi es) (Rel f (.closure fi vs)) := by
  obtain ⟨n, h⟩ := convL_at hes
  refine ⟨n + 1, ?_⟩
  have e := h n (Nat.le_refl _)
  simp only [eval.eq_9]
  cases f with
  | false => cases hl : evalList n p env es <;> rw [hl] at e <;> simp [obs, obsL, Rel, RelL, cond] at e ⊢ <;> rfl
  | true =>
    cases hl : evalList n p env es with
    | values vs' s => rw [hl] at e; simp [obsL, RelL, cond] at e; subst e; rfl
    | _ => rw [hl] at e; simp [obsL, RelL, cond] at e

theorem conv_let {p env x t b body f g bv w} (hb : Conv p env b (Rel f bv))
    (hbody : Conv p ((x, bv) :: env) body (Rel g w)) : Conv p env (.let x t b body) (Rel (f && g) w) := by
  obtain ⟨n1, h1⟩ := conv_at hb
  obtain ⟨n2, h2⟩ := conv_at hbody
  refine ⟨n1 + n2 + 1, ?_⟩
  have e1 := h1 (n1 + n2) (by omega)
  have e2 := h2 (n1 + n2) (by omega)
  simp only [eval.eq_4]
  cases f with
  | false => cases hr : eval (n1 + n2) p env b <;> rw [hr] at e1 <;> simp [obs, Rel, cond] at e1 ⊢ <;> rfl
  | true =>
    cases hr : eval (n1 + n2) p env b with
    | value bv' s =>
      rw [hr] at e1; simp [obs, Rel, cond] at e1; subst e1
      show obs (addSteps _ _) = _
      rw [obs_addSteps]; exact e2
    | _ => rw [hr] at e1; simp [obs, Rel, cond] at e1

theorem conv_cond {p env c t e f} (P : Bool → Bool) (V : Bool → Value) (b : Bool)
    (hc : Conv p env c (Rel f (.bool b)))
    (ht : b = true → Conv p env t (Rel (P true) (V true)))
    (he : b = false → Conv p env e (Rel (P false) (V false))) :
    Conv p env (.cond c t e) (Rel (f && P b) (V b)) := by
  obtain ⟨n1, h1⟩ := conv_at hc
  have hbr : Conv p env (cond b t e) (Rel (P b) (V b)) := by
    cases b
    · exact he rfl
    · exact ht rfl
  obtain ⟨n2, h2⟩ := conv_at hbr
  refine ⟨n1 + n2 + 1, ?_⟩
  have e1 := h1 (n1 + n2) (by omega)
  have e2 := h2 (n1 + n2) (by omega)
  simp only [eval.eq_5]
  cases f with
  | false => cases hr : eval (n1 + n2) p env c <;> rw [hr] at e1 <;> simp [obs, Rel, cond] at e1 ⊢ <;> rfl
  | true =>
    cases hr : eval (n1 + n2) p env c with
    | value cv s =>
      rw [hr] at e1; simp [obs, Rel, cond] at e1; subst e1
      show obs (addSteps _ _) = _
      rw [obs_addSteps]
      cases b <;> exact e2
    | _ => rw [hr] at e1; simp [obs, Rel, cond] at e1

theorem conv_match {p env t s arms f g v w} (hs : Conv p env s (Rel f v))
    (ha : ConvA p env v arms (Rel g w)) : Conv p env (.match t s arms) (Rel (f && g) w) := by
  obtain ⟨n1, h1⟩ := conv_at hs
  obtain ⟨n2, h2⟩ := convA_at ha
  refine ⟨n1 + n2 + 1, ?_⟩
  have e1 := h1 (n1 + n2) (by omega)
  have e2 := h2 (n1 + n2) (by omega)
  simp only [eval.eq_6]
  cases f with
  | false => cases hr : eval (n1 + n2) p env s <;> rw [hr] at e1 <;> simp [obs, Rel, cond] at e1 ⊢ <;> rfl
  | true =>
    cases hr : eval (n1 + n2) p env s with
    | value sv st =>
      rw [hr] at e1; simp [obs, Rel, cond] at e1; subst e1
      show obs (addSteps _ _) = _
      rw [obs_addSteps]; exact e2
    | _ => rw [hr] at e1; simp [obs, Rel, cond] at e1

theorem convA_hit {p env v shape binders body rest fields env' o}
    (hd : destruct shape v = some fields) (hb : bindAll binders fields env = some env')
    (hbody : Conv p env' body o) : ConvA p env v (.arm shape binders body :: rest) o := by
  obtain ⟨n, h⟩ := conv_at hbody
  refine ⟨n + 1, ?_⟩
  simp only [evalArms.eq_3, hd, hb]
  show obs (addSteps _ _) = _
  rw [obs_addSteps]; exact h n (Nat.le_refl _)

theorem convA_miss {p env v shape binders body rest o}
    (hd : destruct shape v = none) (hr : ConvA p env v rest o) :
    ConvA p env v (.arm shape binders body :: rest) o := by
  obtain ⟨n, h⟩ := convA_at hr
  refine ⟨n + 1, ?_⟩
  simp only [evalArms.eq_3, hd]
  show obs (addSteps _ _) = _
  rw [obs_addSteps]; exact h n (Nat.le_refl _)

theorem funRel_intro {p fi args o} {fn : Function} {env}
    (hf : LexLeanRuntime.index p.functions fi = some fn)
    (hb : bindAll fn.parameters args [] = some env) (hbody : Conv p env fn.body o) : FunRel p fi args o :=
  ⟨fn, hf, env, hb, hbody⟩

theorem conv_call {p env fi es f g vs w} (hes : ConvL p env es (RelL f vs))
    (hf : FunRel p fi vs (Rel g w)) : Conv p env (.call fi es) (Rel (f && g) w) := by
  obtain ⟨fn, hfn, benv, hb, hbody⟩ := hf
  obtain ⟨n1, h1⟩ := convL_at hes
  obtain ⟨n2, h2⟩ := conv_at hbody
  refine ⟨n1 + n2 + 1, ?_⟩
  have e1 := h1 (n1 + n2) (by omega)
  have e2 := h2 (n1 + n2) (by omega)
  simp only [eval.eq_8]
  cases f with
  | false => cases hl : evalList (n1 + n2) p env es <;> rw [hl] at e1 <;> simp [obs, obsL, Rel, RelL, cond] at e1 ⊢ <;> rfl
  | true =>
    cases hl : evalList (n1 + n2) p env es with
    | values vs' s =>
      rw [hl] at e1; simp [obsL, RelL, cond] at e1; subst e1
      simp only [hfn, hb]
      show obs (addSteps _ _) = _
      rw [obs_addSteps]; exact e2
    | _ => rw [hl] at e1; simp [obsL, RelL, cond] at e1

theorem conv_apply {p env t es f g h fi caps vs w} (ht : Conv p env t (Rel f (.closure fi caps)))
    (hes : ConvL p env es (RelL g vs))
    (hf : FunRel p fi (LexLeanRuntime.append caps vs) (Rel h w)) :
    Conv p env (.apply t es) (Rel (f && (g && h)) w) := by
  obtain ⟨fn, hfn, benv, hb, hbody⟩ := hf
  obtain ⟨n0, h0⟩ := conv_at ht
  obtain ⟨n1, h1⟩ := convL_at hes
  obtain ⟨n2, h2⟩ := conv_at hbody
  refine ⟨n0 + n1 + n2 + 1, ?_⟩
  have e0 := h0 (n0 + n1 + n2) (by omega)
  have e1 := h1 (n0 + n1 + n2) (by omega)
  have e2 := h2 (n0 + n1 + n2) (by omega)
  simp only [eval.eq_10]
  cases f with
  | false => cases hr : eval (n0 + n1 + n2) p env t <;> rw [hr] at e0 <;> simp [obs, Rel, cond] at e0 ⊢ <;> rfl
  | true =>
    cases hr : eval (n0 + n1 + n2) p env t with
    | value tv ts =>
      rw [hr] at e0; simp [obs, Rel, cond] at e0; subst e0
      simp only
      cases g with
      | false => cases hl : evalList (n0 + n1 + n2) p env es <;> rw [hl] at e1 <;> simp [obs, obsL, Rel, RelL, cond] at e1 ⊢ <;> rfl
      | true =>
        cases hl : evalList (n0 + n1 + n2) p env es with
        | values vs' s =>
          rw [hl] at e1; simp [obsL, RelL, cond] at e1; subst e1
          simp only [hfn, hb]
          show obs (addSteps _ _) = _
          rw [obs_addSteps]; exact e2
        | _ => rw [hl] at e1; simp [obsL, RelL, cond] at e1
    | _ => rw [hr] at e0; simp [obs, Rel, cond] at e0

theorem conv_first {p env e f a b} (h : Conv p env e (Rel f (.pair a b))) :
    Conv p env (.first e) (Rel f a) := by
  obtain ⟨n, hn⟩ := h
  refine ⟨n + 1, ?_⟩
  simp only [eval.eq_12]
  cases f with
  | false => cases hr : eval n p env e <;> rw [hr] at hn <;> simp [obs, Rel, cond] at hn ⊢ <;> rfl
  | true =>
    cases hr : eval n p env e with
    | value v s => rw [hr] at hn; simp [obs, Rel, cond] at hn; subst hn; rfl
    | _ => rw [hr] at hn; simp [obs, Rel, cond] at hn

theorem conv_second {p env e f a b} (h : Conv p env e (Rel f (.pair a b))) :
    Conv p env (.second e) (Rel f b) := by
  obtain ⟨n, hn⟩ := h
  refine ⟨n + 1, ?_⟩
  simp only [eval.eq_13]
  cases f with
  | false => cases hr : eval n p env e <;> rw [hr] at hn <;> simp [obs, Rel, cond] at hn ⊢ <;> rfl
  | true =>
    cases hr : eval n p env e with
    | value v s => rw [hr] at hn; simp [obs, Rel, cond] at hn; subst hn; rfl
    | _ => rw [hr] at hn; simp [obs, Rel, cond] at hn

theorem conv_field {p env e f k fields ctor v} (h : Conv p env e (Rel f (.adt ctor fields)))
    (hi : LexLeanRuntime.index fields k = some v) :
    Conv p env (.field e k) (Rel f v) := by
  obtain ⟨n, hn⟩ := h
  refine ⟨n + 1, ?_⟩
  simp only [eval.eq_14]
  cases f with
  | false => cases hr : eval n p env e <;> rw [hr] at hn <;> simp [obs, Rel, cond] at hn ⊢ <;> rfl
  | true =>
    cases hr : eval n p env e with
    | value v' s => rw [hr] at hn; simp [obs, Rel, cond] at hn; subst hn; simp only [hi]; rfl
    | _ => rw [hr] at hn; simp [obs, Rel, cond] at hn

theorem run_of_funRel {p fi args o} (h : FunRel p fi args o) : RunConv p fi args o := by
  obtain ⟨fn, hfn, env, hb, n, hn⟩ := h
  exact ⟨n, by simp only [run, hfn, hb]; exact hn⟩

theorem conv_matchV {α : Type} {p env t s arms f} (enc : α → Value) (G : α → Bool) (W : α → Value) (sv : α)
    (hs : Conv p env s (Rel f (enc sv)))
    (ha : ∀ a, sv = a → ConvA p env (enc a) arms (Rel (G a) (W a))) :
    Conv p env (.match t s arms) (Rel (f && G sv) (W sv)) :=
  conv_match hs (ha sv rfl)

end LexLeanPreservation
