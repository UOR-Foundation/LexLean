import LexLeanPreservation.RustLemmas
set_option linter.unusedSimpArgs false

/-! Soundness of the rules for matches on rendered arms: the shapes the
patterns realize, boxed fields loaded after the match, empty types whose
arms the rendering omits, record fields, and arms that rebuild their
scrutinee. -/

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

/-! Shapes and the patterns that realize them. -/

/-- The shape a value is built by. -/
def canon : Value → Option Shape
  | .none => some .none
  | .some _ => some .some
  | .ok _ => some .ok
  | .error _ => some .error
  | .list [] => some .nil
  | .list (_ :: _) => some .cons
  | .nat 0 => some .zero
  | .nat (_ + 1) => some .succ
  | .pair _ _ => some .pair
  | .bool true => some .true
  | .bool false => some .false
  | .unit => some .unit
  | .ordering .less => some .lt
  | .ordering .same => some .eq
  | .ordering .more => some .gt
  | .adt k _ => some (.adt k)
  | _ => none

theorem destruct_canon {s v fs} (h : TargetSemantics.destruct s v = some fs) : canon v = some s := by
  cases s <;> cases v <;> simp [TargetSemantics.destruct] at h <;> simp [canon]
  all_goals first
    | (rename_i o; cases o <;> simp [TargetSemantics.sameOrder] at h ⊢; done)
    | skip
  all_goals first
    | (rename_i l; cases l <;> simp at h ⊢)
    | (rename_i k; cases k <;> simp at h ⊢)
    | (rename_i b; cases b <;> simp at h ⊢)
    | (rename_i o; cases o <;> simp [TargetSemantics.sameOrder] at h ⊢ <;> exact absurd h.1 (by decide))
    | (obtain ⟨rfl, _⟩ := h; rfl)
    | skip

theorem shapeBeq_refl : ∀ s : Shape, shapeBeq s s = true := by
  intro s; cases s <;> simp [shapeBeq]

theorem destruct_excl {s1 s2 v f1 f2} (h1 : TargetSemantics.destruct s1 v = some f1)
    (h2 : TargetSemantics.destruct s2 v = some f2) : shapeBeq s1 s2 = true := by
  have e1 := destruct_canon h1
  have e2 := destruct_canon h2
  rw [e1] at e2; cases e2; exact shapeBeq_refl _

/-- The fields a shape destructs have the shape's field types. -/
theorem fields_wt {p c A st shape v fields ts} (hw : WT p c A v st)
    (hd : TargetSemantics.destruct shape v = some fields) (hs : shapeFields p st shape = some ts) :
    WTL p c A fields ts := by
  cases shape with
  | adt k =>
    cases st <;> simp [shapeFields] at hs
    cases v <;> simp [TargetSemantics.destruct] at hd
    obtain ⟨rfl, rfl⟩ := hd
    simp only [WT] at hw
    obtain ⟨tys, htys, hwl⟩ := hw
    rw [hs] at htys; cases htys; exact hwl
  | _ =>
    cases st <;> simp [shapeFields] at hs <;> subst hs <;>
      cases v <;> simp [TargetSemantics.destruct] at hd
    all_goals first
      | (obtain ⟨_, rfl⟩ := hd; simp [WTL]; done)
      | (subst hd; simp [WTL]; done)
      | (subst hd; simp only [WT] at hw; simp [WTL, hw]; done)
      | (obtain ⟨_, rfl⟩ := hd; simp [WTL, WT, inert]; done)
      | (rename_i l; cases l <;> simp at hd <;> subst hd <;>
          simp only [WT, WTAll] at hw <;> simp only [WTL, WT] <;> exact ⟨hw.1, hw.2, trivial⟩)
      | skip

theorem bindPatterns_single (q : RustSyntax.Pat) (x : Value) (r : REnv) :
    RustSemantics.bindPatterns [q] [x] r = RustSemantics.bindPattern q x r := by
  cases h : RustSemantics.bindPattern q x r <;> simp [RustSemantics.bindPatterns, h]

theorem outer_bind {st view shape q inner v fields renv}
    (h : armOuter st view shape q = some inner) (hd : TargetSemantics.destruct shape v = some fields) :
    RustSemantics.bindPattern q (viewVal view v) renv = RustSemantics.bindPatterns inner fields renv := by
  unfold armOuter at h
  split at h
  all_goals first
    | (simp only [Option.some.injEq] at h; subst h
       cases v <;> simp [TargetSemantics.destruct, TargetSemantics.sameOrder] at hd <;>
        (try obtain ⟨_, rfl⟩ := hd) <;> (try subst hd) <;>
        simp [viewVal, RustSemantics.bindPattern, RustSemantics.bindPatterns.eq_1, TargetSemantics.sameOrder,
          bindPatterns_single] <;> done)
    | (simp only [Option.some.injEq] at h; subst h
       cases v <;> simp [TargetSemantics.destruct] at hd
       rename_i o; cases o <;> simp [TargetSemantics.sameOrder] at hd <;> subst hd <;>
        simp [viewVal, RustSemantics.bindPattern, RustSemantics.bindPatterns.eq_1, TargetSemantics.sameOrder]
       done)
    | skip
  · split at h
    · simp only [Option.some.injEq] at h; subst h
      cases v <;> simp [TargetSemantics.destruct] at hd
      rename_i l; cases l <;> simp at hd; subst hd
      simp [viewVal, RustSemantics.bindPattern]
    · cases h
  · split at h
    · rename_i hik
      simp only [Option.some.injEq] at h; subst h
      simp at hik; obtain ⟨rfl, rfl⟩ := hik
      cases v <;> simp [TargetSemantics.destruct] at hd
      obtain ⟨rfl, rfl⟩ := hd
      simp [viewVal, RustSemantics.bindPattern]
    · cases h
  all_goals cases h

theorem outer_miss {p st view shape shape0 q inner v f0 renv}
    (h : armOuter st view shape q = some inner) (hv : (shapeFields p st shape0).isSome = true)
    (hd : TargetSemantics.destruct shape0 v = some f0) (hne : shapeBeq shape0 shape = false) :
    RustSemantics.bindPattern q (viewVal view v) renv = none := by
  unfold armOuter at h
  split at h
  all_goals first
    | (cases shape0 <;> simp [shapeFields] at hv <;> simp [shapeBeq] at hne <;>
        cases v <;> simp [TargetSemantics.destruct] at hd <;>
        (try (rename_i o; cases o <;> simp [TargetSemantics.sameOrder] at hd)) <;>
        (try (rename_i l; cases l <;> simp at hd)) <;>
        simp [viewVal, RustSemantics.bindPattern, TargetSemantics.sameOrder] <;> done)
    | skip
  · split at h
    · rename_i hik
      simp at hik; obtain ⟨rfl, rfl⟩ := hik
      cases shape0 <;> simp [shapeFields] at hv
      simp [shapeBeq] at hne
      cases v <;> simp [TargetSemantics.destruct] at hd
      obtain ⟨rfl, rfl⟩ := hd
      simp [viewVal, RustSemantics.bindPattern, hne]
    · cases h
  · cases h

end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

/-! Field patterns whose boxed fields are loaded after the match. -/

/-- The bindings binder patterns add, latest first. -/
def entriesOf : List RustSyntax.Pat → List Value → REnv
  | q :: qs, f :: fs => entriesOf qs fs ++ (match bindOf q with | some i => [(i, f)] | none => [])
  | _, _ => []

def boxedOne (q : RustSyntax.Pat) : List Nat :=
  match (bindOf q).bind identBoxed with
  | some b => [b]
  | none => []

/-- The boxed temporaries field patterns bind. -/
def boxedOf : List RustSyntax.Pat → List Nat
  | [] => []
  | q :: qs => boxedOne q ++ boxedOf qs

def simplePat (q : RustSyntax.Pat) : Bool := isWild q || (bindOf q).isSome

theorem isWild_eq {q} (h : isWild q = true) : q = .wild := by
  cases q <;> simp [isWild] at h ⊢

theorem bindOf_eq {q i} (h : bindOf q = some i) : q = .bind i := by
  cases q <;> simp [bindOf] at h ⊢; exact h

theorem identLocal_eq {i m} (h : identLocal i = some m) : i = local_ m := by
  cases i with
  | generated k n =>
    simp only [identLocal] at h
    split at h
    · rename_i hk; cases h; cases k <;> simp [RustSemantics.kindNumber] at hk; rfl
    · cases h
  | exported _ => simp [identLocal] at h

theorem identBoxed_eq {i b} (h : identBoxed i = some b) : i = .generated .boxed b := by
  cases i with
  | generated k n =>
    simp only [identBoxed] at h
    split at h
    · rename_i hk; cases h; cases k <;> simp [RustSemantics.kindNumber] at hk; rfl
    · cases h
  | exported _ => simp [identBoxed] at h

theorem rlookup_append (A B : REnv) (i : RIdent) :
    RustSemantics.lookup (A ++ B) i = (RustSemantics.lookup A i).or (RustSemantics.lookup B i) := by
  induction A with
  | nil => simp [RustSemantics.lookup]
  | cons e rest ih =>
    obtain ⟨j, w⟩ := e
    simp only [List.cons_append, RustSemantics.lookup]
    split <;> simp [ih]

theorem bp_entries : ∀ (qs : List RustSyntax.Pat) (fs : List Value) (R : REnv),
    qs.all simplePat = true → qs.length = fs.length →
    RustSemantics.bindPatterns qs fs R = some (entriesOf qs fs ++ R)
  | [], [], R, _, _ => by simp [RustSemantics.bindPatterns, entriesOf]
  | q :: qs, f :: fs, R, hs, hl => by
    simp only [List.all_cons, Bool.and_eq_true] at hs
    simp only [List.length_cons, Nat.add_right_cancel_iff] at hl
    simp only [simplePat, Bool.or_eq_true] at hs
    rcases hs.1 with hw | hb
    · have := isWild_eq hw; subst this
      simp [RustSemantics.bindPatterns, RustSemantics.bindPattern.eq_1, entriesOf, bindOf,
        bp_entries qs fs R hs.2 hl]
    · obtain ⟨i, hi⟩ := Option.isSome_iff_exists.mp hb
      have := bindOf_eq hi; subst this
      simp [RustSemantics.bindPatterns, RustSemantics.bindPattern.eq_2, entriesOf, bindOf,
        bp_entries qs fs _ hs.2 hl]
  | [], _ :: _, _, _, hl => by simp at hl
  | _ :: _, [], _, _, hl => by simp at hl

theorem same_local_local (m m' : Nat) : RustSemantics.sameIdent (local_ m) (local_ m') = Nat.beq m m' := by
  simp [RustSemantics.sameIdent, RustSemantics.kindNumber, local_]

theorem same_boxed_boxed (b b' : Nat) :
    RustSemantics.sameIdent (.generated .boxed b) (.generated .boxed b') = Nat.beq b b' := by
  simp [RustSemantics.sameIdent, RustSemantics.kindNumber]

/-- A local no binder pattern binds is not among the entries. -/
theorem entries_local_none : ∀ (qs : List RustSyntax.Pat) (fs : List Value) (m : Nat),
    m ∉ patLocals qs → RustSemantics.lookup (entriesOf qs fs) (local_ m) = none
  | [], _, _, _ => by simp [entriesOf, RustSemantics.lookup]
  | _ :: _, [], _, _ => by simp [entriesOf, RustSemantics.lookup]
  | q :: qs, f :: fs, m, hm => by
    simp only [patLocals, List.mem_append, not_or] at hm
    simp only [entriesOf, rlookup_append, entries_local_none qs fs m hm.2, Option.none_or]
    cases hq : bindOf q with
    | none => simp [RustSemantics.lookup]
    | some i =>
      simp only [RustSemantics.lookup]
      cases hl : identLocal i with
      | some n =>
        have := identLocal_eq hl; subst this
        simp [patLocal, hq, hl] at hm
        simp [same_local_local, Nat.beq_eq, Ne.symm hm.1]
      | none =>
        cases i with
        | generated k n =>
          have : RustSemantics.sameIdent (.generated k n) (local_ m) = false := by
            simp only [identLocal] at hl
            split at hl
            · cases hl
            · rename_i hk
              simp only [RustSemantics.sameIdent, local_, RustSemantics.kindNumber] at hk ⊢
              cases k <;> simp_all [RustSemantics.kindNumber]
          simp [this]
        | exported _ => simp [RustSemantics.sameIdent, local_]

/-- A boxed temporary no binder pattern binds is not among the entries. -/
theorem entries_boxed_none : ∀ (qs : List RustSyntax.Pat) (fs : List Value) (b : Nat),
    b ∉ boxedOf qs → RustSemantics.lookup (entriesOf qs fs) (.generated .boxed b) = none
  | [], _, _, _ => by simp [entriesOf, RustSemantics.lookup]
  | _ :: _, [], _, _ => by simp [entriesOf, RustSemantics.lookup]
  | q :: qs, f :: fs, b, hb => by
    simp only [boxedOf, List.mem_append, not_or] at hb
    simp only [entriesOf, rlookup_append, entries_boxed_none qs fs b hb.2, Option.none_or]
    cases hq : bindOf q with
    | none => simp [RustSemantics.lookup]
    | some i =>
      simp only [RustSemantics.lookup]
      cases hl : identBoxed i with
      | some n =>
        have := identBoxed_eq hl; subst this
        simp [boxedOne, hq, hl] at hb
        simp [same_boxed_boxed, Nat.beq_eq, Ne.symm hb.1]
      | none =>
        cases i with
        | generated k n =>
          have : RustSemantics.sameIdent (.generated k n) (.generated .boxed b) = false := by
            simp only [identBoxed] at hl
            split at hl
            · cases hl
            · rename_i hk
              simp only [RustSemantics.sameIdent, RustSemantics.kindNumber] at hk ⊢
              cases k <;> simp_all [RustSemantics.kindNumber]
          simp [this]
        | exported _ => simp [RustSemantics.sameIdent]

theorem loadOf_spec {l : RLet} {m b} (h : loadOf l = some (m, b)) :
    l = .mk (.bind (local_ m)) none (.unbox (.generated .boxed b)) := by
  obtain ⟨q, ty, e⟩ := l
  simp only [loadOf] at h
  cases hq : bindOf q with
  | none => simp [hq] at h
  | some i =>
    rw [hq] at h
    cases hi : identLocal i with
    | none => simp [hi] at h
    | some m' =>
      simp only [hi, Option.bind_some] at h
      cases ty with
      | some _ => simp at h
      | none =>
        cases he : unboxOf e with
        | none => simp [he] at h
        | some j =>
          cases hj : identBoxed j with
          | none => simp [he, hj] at h
          | some b' =>
            simp [he, hj] at h
            obtain ⟨rfl, rfl⟩ := h
            have he' : e = .unbox j := by cases e <;> simp [unboxOf] at he; rw [he]
            rw [bindOf_eq hq, identLocal_eq hi, he', identBoxed_eq hj]

/-- The three ways a field pattern direct-ifies. -/
theorem directPat_cons {q qs loads seen qsD} (h : directPat (q :: qs) loads seen = some qsD) :
    (q = .wild ∧ ∃ qs', directPat qs loads seen = some qs' ∧ qsD = .wild :: qs') ∨
    (∃ n, q = .bind (local_ n) ∧ ∃ qs', directPat qs loads seen = some qs' ∧ qsD = .bind (local_ n) :: qs') ∨
    (∃ n m rest, q = .bind (.generated .boxed n) ∧ loads = .mk (.bind (local_ m)) none
        (.unbox (.generated .boxed n)) :: rest ∧ n ∉ seen ∧
      ∃ qs', directPat qs rest (n :: seen) = some qs' ∧ qsD = .bind (local_ m) :: qs') := by
  simp only [directPat] at h
  split at h
  · rename_i hw
    simp only [Option.map_eq_some_iff] at h
    obtain ⟨qs', h', rfl⟩ := h
    exact .inl ⟨isWild_eq hw, qs', h', rfl⟩
  · split at h
    · cases h
    · rename_i i hi
      split at h
      · rename_i n hn
        simp only [Option.map_eq_some_iff] at h
        obtain ⟨qs', h', rfl⟩ := h
        exact .inr (.inl ⟨n, by rw [bindOf_eq hi, identLocal_eq hn], qs', h', rfl⟩)
      · split at h
        · cases h
        · rename_i n hb
          split at h
          · cases h
          · rename_i l rest
            split at h
            · cases h
            · rename_i m b hl
              split at h
              · rename_i hc
                simp only [Option.map_eq_some_iff] at h
                obtain ⟨qs', h', rfl⟩ := h
                simp at hc
                obtain ⟨rfl, hc⟩ := hc
                refine .inr (.inr ⟨n, m, rest, by rw [bindOf_eq hi, identBoxed_eq hb], ?_, hc, qs', h', rfl⟩)
                rw [loadOf_spec hl]
              · cases h

theorem directPat_nil {loads seen qsD} (h : directPat [] loads seen = some qsD) : loads = [] ∧ qsD = [] := by
  simp only [directPat] at h
  split at h
  · rename_i he; simp at h; exact ⟨List.isEmpty_iff.mp he, h⟩
  · cases h

theorem rlookup_nil (i : RIdent) : RustSemantics.lookup [] i = none := rfl

theorem rlookup_local_single (m m' : Nat) (f : Value) :
    RustSemantics.lookup [(local_ m', f)] (local_ m) = if m' = m then some f else none := by
  simp only [RustSemantics.lookup, same_local_local]
  by_cases h : m' = m <;> simp [h, Nat.beq_eq]

theorem rlookup_boxed_local (b m : Nat) (f : Value) :
    RustSemantics.lookup [(.generated .boxed b, f)] (local_ m) = none := by
  simp [RustSemantics.lookup, RustSemantics.sameIdent, RustSemantics.kindNumber, local_]

theorem rlookup_local_boxed (m b : Nat) (f : Value) :
    RustSemantics.lookup [(local_ m, f)] (.generated .boxed b) = none := by
  simp [RustSemantics.lookup, RustSemantics.sameIdent, RustSemantics.kindNumber, local_]

/-- Binding the field patterns, then loading the boxed fields, binds every
local the direct patterns bind to the same value. -/
theorem directPat_loads {c : RCrate} : ∀ (inner : List RustSyntax.Pat) (loads : List RLet)
    (seen : List Nat) (qsD : List RustSyntax.Pat) (fs : List Value),
    directPat inner loads seen = some qsD → (patLocals qsD).Nodup → inner.length = fs.length →
    (∀ b ∈ boxedOf inner, b ∉ seen) ∧ inner.length = qsD.length ∧ inner.all simplePat = true ∧
    qsD.all simplePat = true ∧
    ∃ Lb : REnv,
      (∀ X : REnv, (∀ b ∈ boxedOf inner, RustSemantics.lookup X (.generated .boxed b) =
          RustSemantics.lookup (entriesOf inner fs) (.generated .boxed b)) →
        LetsOK c X loads (Lb ++ X)) ∧
      (∀ m, (RustSemantics.lookup Lb (local_ m)).or (RustSemantics.lookup (entriesOf inner fs) (local_ m)) =
        RustSemantics.lookup (entriesOf qsD fs) (local_ m))
  | [], loads, seen, qsD, fs, h, _, hl => by
    obtain ⟨rfl, rfl⟩ := directPat_nil h
    cases fs with
    | cons _ _ => simp at hl
    | nil =>
      refine ⟨by simp [boxedOf], rfl, rfl, rfl, [], fun X _ => .nil, fun m => ?_⟩
      simp [entriesOf, rlookup_nil]
  | q :: inner, loads, seen, qsD, fs, h, hnd, hl => by
    cases fs with
    | nil => simp at hl
    | cons f fs =>
    simp only [List.length_cons, Nat.add_right_cancel_iff] at hl
    rcases directPat_cons h with ⟨rfl, qs', h', rfl⟩ | ⟨n, rfl, qs', h', rfl⟩ |
      ⟨n, m0, rest, rfl, rfl, hns, qs', h', rfl⟩
    · have hnd' : (patLocals qs').Nodup := by simpa [patLocals, patLocal, bindOf] using hnd
      obtain ⟨hb, hlen, hs1, hs2, Lb, hlets, heq⟩ := directPat_loads inner loads seen qs' fs h' hnd' hl
      refine ⟨by simpa [boxedOf, boxedOne, bindOf] using hb, by simp [hlen],
        by simp [simplePat, isWild, hs1], by simp [simplePat, isWild, hs2], Lb, fun X hX => hlets X ?_, ?_⟩
      · intro b hbm; simpa [entriesOf, bindOf] using hX b (by simpa [boxedOf, boxedOne, bindOf] using hbm)
      · intro m; simpa [entriesOf, bindOf] using heq m
    · have hnd' : (patLocals qs').Nodup := by
        simp [patLocals, patLocal, bindOf, identLocal, local_, RustSemantics.kindNumber] at hnd; exact hnd.2
      obtain ⟨hb, hlen, hs1, hs2, Lb, hlets, heq⟩ := directPat_loads inner loads seen qs' fs h' hnd' hl
      have hbo : boxedOne (.bind (local_ n)) = [] := by
        simp [boxedOne, bindOf, identBoxed, local_, RustSemantics.kindNumber]
      refine ⟨by simpa [boxedOf, hbo] using hb, by simp [hlen],
        by simp [simplePat, bindOf, hs1], by simp [simplePat, bindOf, hs2], Lb, fun X hX => hlets X ?_, ?_⟩
      · intro b hbm
        have := hX b (by simp [boxedOf, hbo, hbm])
        rw [this]; simp [entriesOf, bindOf, rlookup_append, rlookup_local_boxed]
      · intro m
        simp only [entriesOf, bindOf, rlookup_append]
        rw [← heq m, Option.or_assoc]
    · have hnd' : (patLocals qs').Nodup := by
        simp [patLocals, patLocal, bindOf, identLocal, local_, RustSemantics.kindNumber] at hnd; exact hnd.2
      have hm0 : m0 ∉ patLocals qs' := by
        simp [patLocals, patLocal, bindOf, identLocal, local_, RustSemantics.kindNumber] at hnd; exact hnd.1
      obtain ⟨hb, hlen, hs1, hs2, Lb, hlets, heq⟩ := directPat_loads inner rest (n :: seen) qs' fs h' hnd' hl
      have hbo : boxedOne (.bind (.generated .boxed n)) = [n] := by
        simp [boxedOne, bindOf, identBoxed, RustSemantics.kindNumber]
      have hnin : n ∉ boxedOf inner := fun hm => by have := hb n hm; simp at this
      have hpi : RustSemantics.lookup (entriesOf inner fs) (.generated .boxed n) = none :=
        entries_boxed_none inner fs n hnin
      have hpd0 : RustSemantics.lookup (entriesOf qs' fs) (local_ m0) = none :=
        entries_local_none qs' fs m0 hm0
      refine ⟨?_, by simp [hlen], by simp [simplePat, bindOf, hs1], by simp [simplePat, bindOf, hs2],
        Lb ++ [(local_ m0, f)], fun X hX => ?_, fun m => ?_⟩
      · intro b hbm
        simp only [boxedOf, hbo, List.cons_append, List.nil_append, List.mem_cons] at hbm
        rcases hbm with rfl | hbm
        · exact hns
        · have := hb b hbm; simp at this; exact this.2
      · have hX0 := hX n (by simp [boxedOf, hbo])
        simp only [entriesOf, bindOf, rlookup_append, hpi, Option.none_or] at hX0
        have hread : RustSemantics.lookup X (.generated .boxed n) = some f := by
          rw [hX0]; simp [RustSemantics.lookup, same_boxed_boxed]
        have h1 := hlets ((local_ m0, f) :: X) (by
          intro b hbm
          have hbn : b ≠ n := fun e => hnin (e ▸ hbm)
          have := hX b (by simp [boxedOf, hbo, hbm])
          rw [rlookup_cons_other (by simp [RustSemantics.sameIdent, RustSemantics.kindNumber, local_]), this]
          simp [entriesOf, bindOf, rlookup_append, RustSemantics.lookup, same_boxed_boxed, Nat.beq_eq, Ne.symm hbn])
        simp only [List.append_assoc, List.singleton_append]
        exact .cons (rc_read (r := .unbox (.generated .boxed n)) rfl hread)
          (by simp [RustSemantics.bindPattern.eq_2]) h1
      · have he := heq m
        simp only [entriesOf, bindOf, rlookup_append, rlookup_boxed_local, rlookup_local_single,
          Option.or_none] at he ⊢
        by_cases hmm : m0 = m
        · subst hmm
          rw [hpd0] at he ⊢
          have h1 : RustSemantics.lookup Lb (local_ m0) = none := by
            cases h' : RustSemantics.lookup Lb (local_ m0) <;> simp_all
          have h2 : RustSemantics.lookup (entriesOf inner fs) (local_ m0) = none := by
            cases h' : RustSemantics.lookup (entriesOf inner fs) (local_ m0) <;> simp_all
          simp [h1, h2]
        · simp [hmm, he]

theorem entries_boxed_some : ∀ (qs : List RustSyntax.Pat) (fs : List Value) (b : Nat),
    qs.length = fs.length → b ∈ boxedOf qs →
    ∃ w, RustSemantics.lookup (entriesOf qs fs) (.generated .boxed b) = some w
  | [], _, _, _, hb => by simp [boxedOf] at hb
  | _ :: _, [], _, hl, _ => by simp at hl
  | q :: qs, f :: fs, b, hl, hb => by
    simp only [List.length_cons, Nat.add_right_cancel_iff] at hl
    simp only [entriesOf, rlookup_append]
    simp only [boxedOf, List.mem_append] at hb
    rcases hb with hb | hb
    · cases h : RustSemantics.lookup (entriesOf qs fs) (.generated .boxed b) with
      | some w => exact ⟨w, rfl⟩
      | none =>
        simp only [boxedOne] at hb
        cases hq : bindOf q with
        | none => simp [hq] at hb
        | some i =>
          cases hi : identBoxed i with
          | none => simp [hq, hi] at hb
          | some b' =>
            simp [hq, hi] at hb; subst hb
            rw [identBoxed_eq hi]
            exact ⟨f, by simp [RustSemantics.lookup, same_boxed_boxed]⟩
    · obtain ⟨w, hw⟩ := entries_boxed_some qs fs b hl hb
      exact ⟨w, by rw [hw]; rfl⟩

theorem paramCtx_length : ∀ (acc : Ctx) (xs : List Nat) (ts : List Ty) (qs : List RustSyntax.Pat) (Γ : Ctx),
    paramCtx acc xs ts qs = some Γ → xs.length = ts.length ∧ ts.length = qs.length
  | _, [], [], [], _, _ => ⟨rfl, rfl⟩
  | acc, x :: xs, t :: ts, q :: qs, Γ, h => by
    cases q with
    | wild =>
      simp only [paramCtx] at h
      have := paramCtx_length _ xs ts qs Γ h; simp [this.1, this.2]
    | bind i =>
      cases i with
      | generated k m =>
        cases k with
        | binding =>
          simp only [paramCtx] at h
          split at h
          · have := paramCtx_length _ xs ts qs Γ h; simp [this.1, this.2]
          · cases h
        | _ => simp [paramCtx] at h
      | exported _ => simp [paramCtx] at h
    | _ => simp [paramCtx] at h
  | _, [], [], _ :: _, _, h => by simp [paramCtx] at h
  | _, [], _ :: _, _, _, h => by simp [paramCtx] at h
  | _, _ :: _, [], _, _, h => by simp [paramCtx] at h
  | _, _ :: _, _ :: _, [], _, h => by simp [paramCtx] at h

theorem directPat_len : ∀ (inner : List RustSyntax.Pat) (loads : List RLet) (seen : List Nat)
    (qsD : List RustSyntax.Pat), directPat inner loads seen = some qsD → inner.length = qsD.length
  | [], _, _, _, h => by obtain ⟨_, rfl⟩ := directPat_nil h; rfl
  | q :: inner, loads, seen, qsD, h => by
    rcases directPat_cons h with ⟨_, qs', h', rfl⟩ | ⟨_, _, qs', h', rfl⟩ | ⟨_, _, _, _, _, _, qs', h', rfl⟩ <;>
      simp [directPat_len _ _ _ _ h']

/-- Relations about locals hold of any environment binding the same locals. -/
theorem envRel_locals {p c A Γ env R S}
    (h : EnvRel p c A Γ env R)
    (hloc : ∀ m, RustSemantics.lookup S (local_ m) = RustSemantics.lookup R (local_ m)) :
    EnvRel p c A Γ env S := by
  intro x t s hx
  obtain ⟨v, hv, hw, hr⟩ := h x t s hx
  exact ⟨v, hv, hw, fun m hm => by rw [hloc]; exact hr m hm⟩

/-- An arm's pattern and loads bind the arm's binders as its scope says. -/
theorem armPat_sound {p c A Γ Γ' st view shape xs q loads env renv v fields env'}
    (h : armPat p Γ st view shape xs q loads = some Γ') (hrel : EnvRel p c A Γ env renv)
    (hw : WT p c A v st) (hd : TargetSemantics.destruct shape v = some fields)
    (hb : TargetSemantics.bindAll xs fields env = some env') :
    ∃ renv1 renv2, RustSemantics.bindPattern q (viewVal view v) renv = some renv1 ∧
      LetsOK c renv1 loads renv2 ∧ EnvRel p c A Γ' env' renv2 := by
  simp only [armPat] at h
  cases ho : armOuter st view shape q with
  | none => simp [ho] at h
  | some inner =>
  cases hs : shapeFields p st shape with
  | none => simp [ho, hs] at h
  | some ts =>
  cases hdp : directPat inner loads [] with
  | none => simp [ho, hs, hdp] at h
  | some qsD =>
  simp only [ho, hs, hdp, Option.bind_some] at h
  split at h
  · rename_i hnd
    have hwl := fields_wt hw hd hs
    obtain ⟨hl1, hl2⟩ := paramCtx_length _ _ _ _ _ h
    have hfl := wtl_length hwl
    have hil : inner.length = fields.length := by
      rw [directPat_len _ _ _ _ hdp, ← hl2, hfl]
    obtain ⟨_, _, hs1, hs2, Lb, hlets, heq⟩ :=
      directPat_loads (c := c) inner loads [] qsD fields hdp (by simpa using hnd) hil
    have hbp := bp_entries inner fields renv hs1 hil
    obtain ⟨RD, hRD, hrelD⟩ := param_env Γ xs ts qsD Γ' fields env env' renv h hrel hwl hb
    have hRD' := bp_entries qsD fields renv hs2 (by rw [← hl2, hfl])
    rw [hRD'] at hRD; cases hRD
    refine ⟨entriesOf inner fields ++ renv, Lb ++ (entriesOf inner fields ++ renv), ?_, ?_, ?_⟩
    · rw [outer_bind ho hd, hbp]
    · apply hlets
      intro b hbm
      obtain ⟨w, hwb⟩ := entries_boxed_some inner fields b hil hbm
      rw [rlookup_append, hwb]; rfl
    · refine envRel_locals hrelD (fun m => ?_)
      rw [rlookup_append, rlookup_append, rlookup_append, ← heq m, Option.or_assoc]
  · cases h

end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

/-! Matches on rendered arms. -/

theorem rca_hit {c renv w q blk rest renv1 o}
    (hb : RustSemantics.bindPattern q w renv = some renv1) (h : RCB c renv1 blk o) :
    RCA c renv w (.mk q blk :: rest) o := by
  obtain ⟨n, hn⟩ := h.at
  exact ⟨h.1, n+1, by rw [RustSemantics.evalArms.eq_3]; simp only [hb]; exact hn n (Nat.le_refl _)⟩

theorem rca_miss {c renv w q blk rest o}
    (hb : RustSemantics.bindPattern q w renv = none) (h : RCA c renv w rest o) :
    RCA c renv w (.mk q blk :: rest) o := by
  obtain ⟨hne, n, hn⟩ := h
  exact ⟨hne, n+1, by rw [RustSemantics.evalArms.eq_3]; simp only [hb]; exact hn⟩

theorem rc_matchOn {c renv sc w rarms o} (hs : RC c renv sc (.value w)) (h : RCA c renv w rarms o) :
    RC c renv (.matchOn sc rarms) o := by
  obtain ⟨n1, hn1⟩ := hs.at
  obtain ⟨hne, n2, hn2⟩ := h
  have hn2' : ∀ m, n2 ≤ m → RustSemantics.evalArms m c renv w rarms = o := fun m hm => by
    rw [revalArms_le (by rw [hn2]; exact hne) hm, hn2]
  exact ⟨hne, max n1 n2 + 1, by
    rw [RustSemantics.eval.eq_15, hn1 _ (Nat.le_max_left _ _)]; exact hn2' _ (Nat.le_max_right _ _)⟩

theorem rc_uncons {c renv h items} (hl : RustSemantics.lookup renv h = some (.list items)) :
    RC c renv (.uncons h) (.value (viewVal true (.list items))) := by
  refine ⟨nofun, 1, ?_⟩
  rw [RustSemantics.eval.eq_17, hl]
  cases items <;> rfl

theorem armPat_outer {p Γ st view shape xs q loads Γ'} (h : armPat p Γ st view shape xs q loads = some Γ') :
    ∃ inner, armOuter st view shape q = some inner := by
  simp only [armPat] at h
  cases ho : armOuter st view shape q with
  | none => simp [ho] at h
  | some inner => exact ⟨inner, rfl⟩

theorem outer_view {st shape q inner v fields} (h : armOuter st true shape q = some inner)
    (hd : TargetSemantics.destruct shape v = some fields) : ∃ items, v = .list items := by
  cases shape <;> cases st <;> simp [armOuter] at h <;>
    cases v <;> simp [TargetSemantics.destruct] at hd <;> exact ⟨_, rfl⟩

theorem otherArms_mem {p st arms idx s j a} (h : otherArms p st arms idx s = true) (hj : j ∈ idx)
    (ha : arms[j]? = some a) : shapeBeq (armShape a) s = false ∧ (shapeFields p st (armShape a)).isSome = true := by
  simp only [otherArms, List.all_eq_true] at h
  have := h j hj
  simp only [ha, Bool.and_eq_true, Bool.not_eq_true'] at this
  exact this

section rules
variable {p : Program} {c : RCrate} {A : Flags}

theorem sem_mNil {n Γ fl st view arms fm τ} : Sem p c A n Γ fl (.m st view arms [] fm τ []) := by
  intro env renv hrel v hw j shape xs body hj
  simp at hj

theorem sem_mCons {n Γ Γ' fl st view arms idx fm τ rarms j shape xs body q loads lets tail}
    (hrest : Sem p c A n Γ fl (.m st view arms idx fm τ rarms))
    (hj : arms[j]? = some (.arm shape xs body)) (hoth : otherArms p st arms idx shape = true)
    (hpat : armPat p Γ st view shape xs q loads = some Γ')
    (hbody : Sem p c A n Γ' fl (.b fm body τ lets tail)) :
    Sem p c A n Γ fl (.m st view arms (j :: idx) fm τ (.mk q (foldB ⟨loads ++ lets, tail⟩) :: rarms)) := by
  intro env renv hrel v hw j0 shape0 xs0 body0 hmem hj0 fields env' hd hb o ho hst
  obtain ⟨inner, hout⟩ := armPat_outer hpat
  simp only [List.mem_cons] at hmem
  rcases hmem with rfl | hmem
  · rw [hj] at hj0; cases hj0
    obtain ⟨renv1, renv2, hbp, hl, hrel'⟩ := armPat_sound hpat hrel hw hd hb
    obtain ⟨hwt, ro, hro, hrcb⟩ := sem_b_rcb hbody hrel' ho hst
    refine ⟨hwt, fun hv => ?_, ro, hro, rca_hit hbp (foldB_sound (rcb_of_lets hl hrcb))⟩
    subst hv; exact outer_view hout hd
  · obtain ⟨hwt, hview, ro, hro, hrca⟩ := hrest env renv hrel v hw j0 shape0 xs0 body0 hmem hj0 fields env' hd hb o ho hst
    obtain ⟨hne, hvalid⟩ := otherArms_mem hoth hmem hj0
    refine ⟨hwt, hview, ro, hro, rca_miss ?_ hrca⟩
    exact outer_miss hout hvalid hd hne

end rules
end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

/-! Types no value inhabits. -/

mutual
theorem tyBeq_refl : ∀ t : Ty, tyBeq t t = true
  | .unit | .bool | .nat | .int | .string | .bytes | .ordering => by simp [tyBeq]
  | .fixed k => by simp [tyBeq]
  | .option t | .list t => by simp [tyBeq, tyBeq_refl t]
  | .result a b | .pair a b => by simp [tyBeq, tyBeq_refl a, tyBeq_refl b]
  | .adt i => by simp [tyBeq]
  | .fn ps r => by simp [tyBeq, tysBeq_refl ps, tyBeq_refl r]
theorem tysBeq_refl : ∀ ts : List Ty, tysBeq ts ts = true
  | [] => by simp [tysBeq]
  | t :: ts => by simp [tysBeq, tyBeq_refl t, tysBeq_refl ts]
end

theorem findArm_some : ∀ {arms : List RustSyntax.Dispatch} {f k x}, RustSemantics.findArm arms f k = some x →
    ∃ d ∈ arms, d.function = f ∧ d.captures.length = k
  | [], _, _, _, h => by simp [RustSemantics.findArm] at h
  | d :: rest, f, k, x, h => by
    simp only [RustSemantics.findArm] at h
    split at h
    · rename_i hc
      simp [Nat.beq_eq] at hc
      exact ⟨d, by simp, hc.1, hc.2⟩
    · obtain ⟨d', hd', h1, h2⟩ := findArm_some h
      exact ⟨d', by simp [hd'], h1, h2⟩

theorem findDispatch_some : ∀ {items : List RustSyntax.ItemDef} {f k x},
    RustSemantics.findDispatch items f k = some x →
    ∃ ft ps r fal arms, RustSyntax.ItemDef.apply ft ps r fal arms ∈ items ∧
      ∃ d ∈ arms, d.function = f ∧ d.captures.length = k
  | [], _, _, _, h => by simp [RustSemantics.findDispatch] at h
  | item :: rest, f, k, x, h => by
    cases item with
    | enum _ _ =>
      obtain ⟨ft, ps, r, fal, arms, hm, hd⟩ := findDispatch_some (by simpa [RustSemantics.findDispatch] using h)
      exact ⟨ft, ps, r, fal, arms, by simp [hm], hd⟩
    | function _ _ _ _ =>
      obtain ⟨ft, ps, r, fal, arms, hm, hd⟩ := findDispatch_some (by simpa [RustSemantics.findDispatch] using h)
      exact ⟨ft, ps, r, fal, arms, by simp [hm], hd⟩
    | apply ft ps r fal arms =>
      simp only [RustSemantics.findDispatch] at h
      cases ha : RustSemantics.findArm arms f k with
      | some y => exact ⟨ft, ps, r, fal, arms, by simp, findArm_some ha⟩
      | none =>
        rw [ha] at h
        obtain ⟨ft', ps', r', fal', arms', hm, hd⟩ := findDispatch_some h
        exact ⟨ft', ps', r', fal', arms', by simp [hm], hd⟩

theorem uninh_of_inert {p c U} : ∀ t : Ty, inert t = true → uninh p c U t = false
  | .option _, _ | .list _, _ | .unit, _ | .bool, _ | .nat, _ | .int, _ | .fixed _, _ => by simp [uninh]
  | .string, _ | .bytes, _ | .ordering, _ => by simp [uninh]
  | .adt _, h | .fn _ _, h => by simp [inert] at h
  | .result a b, h => by
    simp [inert] at h; simp [uninh, uninh_of_inert a h.1, uninh_of_inert b h.2]
  | .pair a b, h => by
    simp [inert] at h; simp [uninh, uninh_of_inert a h.1, uninh_of_inert b h.2]

mutual
/-- No value of a type claimed empty has the type. -/
theorem wt_uninh {p c A U} (hU : validU p c U = true) : ∀ (v : Value) (t : Ty), WT p c A v t →
    uninh p c U t = false
  | v, t, hw => by
    cases t with
    | adt i =>
      cases v with
      | adt k fs =>
        simp only [WT] at hw
        obtain ⟨tys, htys, hwl⟩ := hw
        simp only [uninh]
        cases hc : U.contains i with
        | false => rfl
        | true =>
          exfalso
          have hU' := hU
          simp only [validU, List.all_eq_true] at hU'
          have hi := hU' i (List.contains_iff_mem.mp hc)
          simp only [adtFields] at htys
          cases ha : p.adts[i]? with
          | none => simp [ha] at htys
          | some adt =>
            simp only [ha, Option.bind_some] at htys hi
            simp only [List.all_eq_true] at hi
            have hany := hi tys (List.mem_of_getElem? htys)
            have := wtl_uninh hU fs tys hwl
            rw [this] at hany; cases hany
      | _ => simp [WT, inert] at hw
    | pair a b =>
      simp only [uninh]
      cases v with
      | pair x y =>
        simp only [WT] at hw
        rw [wt_uninh hU x a hw.1, wt_uninh hU y b hw.2]; rfl
      | _ =>
        simp only [WT] at hw
        have := uninh_of_inert (p := p) (c := c) (U := U) _ hw
        simpa [uninh] using this
    | result a b =>
      cases v with
      | ok x => simp only [WT] at hw; simp [uninh, wt_uninh hU x a hw]
      | error y => simp only [WT] at hw; simp [uninh, wt_uninh hU y b hw]
      | _ => simp only [WT] at hw; rw [uninh_of_inert _ hw]
    | fn ps r =>
      cases v with
      | closure f cs =>
        simp only [WT] at hw
        obtain ⟨fn, capTys, af, hfn, htys, hres, hwc, _, ff, hd, _, _⟩ := hw
        simp only [uninh, noClosure]
        cases hnc : c.items.all _ with
        | false => rfl
        | true =>
          exfalso
          simp only [List.all_eq_true] at hnc
          obtain ⟨ft, ps', r', fal, arms, hm, d, hdm, hdf, hdl⟩ := findDispatch_some hd
          have h1 := hnc _ hm
          simp only [List.all_eq_true] at h1
          have h2 := h1 d hdm
          rw [hdf, hfn] at h2
          simp only at h2
          have hdrop : fn.types.drop d.captures.length = ps := by
            rw [htys, hdl, wtl_length hwc]; simp
          rw [hdrop, hres, tysBeq_refl, tyBeq_refl] at h2
          cases h2
      | _ => simp [WT, inert] at hw
    | _ => simp [uninh]
theorem wtl_uninh {p c A U} (hU : validU p c U = true) : ∀ (vs : List Value) (ts : List Ty),
    WTL p c A vs ts → ts.any (uninh p c U) = false
  | [], [], _ => rfl
  | v :: vs, t :: ts, hw => by
    simp only [WTL] at hw
    simp [wt_uninh hU v t hw.1, wtl_uninh hU vs ts hw.2]
  | [], _ :: _, hw => by simp [WTL] at hw
  | _ :: _, [], hw => by simp [WTL] at hw
end

end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

theorem covered_mem {p c U st arms idx j a} (h : covered p c U st arms idx = true)
    (ha : arms[j]? = some a) :
    j ∈ idx ∨ ∃ ts, shapeFields p st (armShape a) = some ts ∧ ts.any (uninh p c U) = true := by
  simp only [covered, List.all_eq_true, List.mem_range] at h
  have hj : j < arms.length := by
    rcases Nat.lt_or_ge j arms.length with h' | h'
    · exact h'
    · rw [List.getElem?_eq_none h'] at ha; cases ha
  have := h j hj
  simp only [ha, Bool.or_eq_true] at this
  rcases this with h1 | h1
  · exact .inl (List.contains_iff_mem.mp h1)
  · cases hs : shapeFields p st (armShape a) with
    | none => simp [hs] at h1
    | some ts => simp [hs] at h1; exact .inr ⟨ts, rfl, by simpa using h1⟩

theorem rc_scrut {c renv h v view} (hl : RustSemantics.lookup renv h = some v)
    (hv : view = true → ∃ items, v = .list items) :
    RC c renv (scrut view h) (.value (viewVal view v)) := by
  cases view with
  | false => exact rc_read (r := .move h) rfl hl
  | true =>
    obtain ⟨items, rfl⟩ := hv rfl
    exact rc_uncons hl

section rules
variable {p : Program} {c : RCrate} {A : Flags}

/-- The arm a match takes is one the rendering has: an arm it omits holds a
value of an empty type. -/
theorem taken_arm {n env v st arms idx o U} (hU : validU p c U = true)
    (hcov : covered p c U st arms idx = true) (hw : WT p c A v st)
    (ho : obs (TargetSemantics.evalArms n p env v arms) = some o) (hst : o ≠ .stuck) :
    ∃ j shape xs body fields env' m', m' < n ∧ j ∈ idx ∧ arms[j]? = some (.arm shape xs body) ∧
      TargetSemantics.destruct shape v = some fields ∧ TargetSemantics.bindAll xs fields env = some env' ∧
      obs (TargetSemantics.eval m' p env' body) = some o := by
  obtain ⟨j, shape, xs, body, fields, env', m', hm, hj, hd, hb, hbody, _⟩ := evalArms_find n arms o ho hst
  rcases covered_mem hcov hj with hmem | ⟨ts, hs, hany⟩
  · exact ⟨j, shape, xs, body, fields, env', m', hm, hmem, hj, hd, hb, hbody⟩
  · exfalso
    have hwl := fields_wt hw hd hs
    rw [wtl_uninh hU fields ts hwl] at hany; cases hany

theorem sem_matchArms {n Γ fl fm τ st view s0 arms idx rs rarms U k j ty}
    (hs : Sem p c A n Γ fl (.e s0 st rs))
    (hm : ∀ m ≤ n, Sem p c A m Γ fl (.m st view arms idx fm τ rarms))
    (hU : validU p c U = true) (hcov : covered p c U st arms idx = true) (hk : k ≠ .binding) :
    Sem p c A (n+1) Γ fl (.b fm (.match τ s0 arms) τ [.mk (.bind (.generated k j)) ty rs]
      (.matchOn (scrut view (.generated k j)) rarms)) := by
  refine sem_matchHolder hs hk ?_
  intro env renv v o hrel hw hl ho hst
  obtain ⟨j0, shape, xs, body, fields, env', m', hm', hmem, hj, hd, hb, hbody⟩ :=
    taken_arm hU hcov hw ho hst
  obtain ⟨hwt, hview, ro, hro, hrca⟩ :=
    hm m' (by omega) env renv hrel v hw j0 shape xs body hmem hj fields env' hd hb o hbody hst
  exact ⟨hwt, .inl ⟨[], fresh_nil, .nil, ro, hro, by simpa using rc_matchOn (rc_scrut hl hview) hrca⟩⟩

theorem paramCtx_ext : ∀ (acc : Ctx) (xs : List Nat) (ts : List Ty) (qs : List RustSyntax.Pat) (Γ : Ctx),
    paramCtx acc xs ts qs = some Γ → ∃ ext, Γ = ext ++ acc
  | acc, [], [], [], Γ, h => by simp [paramCtx] at h; exact ⟨[], by simp [h]⟩
  | acc, x :: xs, t :: ts, q :: qs, Γ, h => by
    cases q with
    | wild =>
      simp only [paramCtx] at h
      obtain ⟨ext, rfl⟩ := paramCtx_ext _ xs ts qs Γ h
      exact ⟨ext ++ [(x, t, none)], by simp⟩
    | bind i =>
      cases i with
      | generated k m =>
        cases k with
        | binding =>
          simp only [paramCtx] at h
          split at h
          · obtain ⟨ext, rfl⟩ := paramCtx_ext _ xs ts qs Γ h
            exact ⟨ext ++ [(x, t, some m)], by simp⟩
          · cases h
        | _ => simp [paramCtx] at h
      | exported _ => simp [paramCtx] at h
    | _ => simp [paramCtx] at h
  | _, [], [], _ :: _, _, h => by simp [paramCtx] at h
  | _, [], _ :: _, _, _, h => by simp [paramCtx] at h
  | _, _ :: _, [], _, _, h => by simp [paramCtx] at h
  | _, _ :: _, _ :: _, [], _, h => by simp [paramCtx] at h

theorem fresh_ext {Γ ext new} (h : Fresh (ext ++ Γ) new) : Fresh Γ new := by
  intro m w hm
  have := h m w hm
  simp only [slotFree, List.all_append, Bool.and_eq_true] at this
  exact this.2

/-- The locals parameter patterns bind are free in the scope they extend. -/
theorem paramCtx_fresh : ∀ (acc : Ctx) (xs : List Nat) (ts : List Ty) (qs : List RustSyntax.Pat) (Γ : Ctx)
    (vs : List Value) (R R' : REnv),
    paramCtx acc xs ts qs = some Γ → RustSemantics.bindPatterns qs vs R = some R' →
    ∃ bs, R' = bs ++ R ∧ Fresh acc bs
  | acc, [], [], [], Γ, vs, R, R', h, hb => by
    cases vs with
    | nil => simp [RustSemantics.bindPatterns] at hb; subst hb; exact ⟨[], rfl, fresh_nil⟩
    | cons _ _ => simp [RustSemantics.bindPatterns] at hb
  | acc, x :: xs, t :: ts, q :: qs, Γ, vs, R, R', h, hb => by
    cases vs with
    | nil => simp [RustSemantics.bindPatterns] at hb
    | cons v vs =>
    cases q with
    | wild =>
      simp only [paramCtx] at h
      simp only [RustSemantics.bindPatterns, RustSemantics.bindPattern.eq_1] at hb
      obtain ⟨bs, rfl, hf⟩ := paramCtx_fresh _ xs ts qs Γ vs R R' h hb
      exact ⟨bs, rfl, fresh_weaken hf⟩
    | bind i =>
      cases i with
      | generated k m =>
        cases k with
        | binding =>
          simp only [paramCtx] at h
          split at h
          · rename_i hfree
            simp only [RustSemantics.bindPatterns, RustSemantics.bindPattern.eq_2] at hb
            obtain ⟨bs, rfl, hf⟩ := paramCtx_fresh _ xs ts qs Γ vs _ R' h hb
            refine ⟨bs ++ [(local_ m, v)], by simp, fresh_bind (fresh_weaken hf) hfree⟩
          · cases h
        | _ => simp [paramCtx] at h
      | exported _ => simp [paramCtx] at h
    | _ => simp [paramCtx] at h
  | _, [], [], _ :: _, _, _, _, _, h, _ => by simp [paramCtx] at h
  | _, [], _ :: _, _, _, _, _, _, h, _ => by simp [paramCtx] at h
  | _, _ :: _, [], _, _, _, _, _, h, _ => by simp [paramCtx] at h
  | _, _ :: _, _ :: _, [], _, _, _, _, h, _ => by simp [paramCtx] at h

theorem sem_matchPair {n Γ Γ'' fl fm τ s0 x y body ta tb rs q r k j ty ty' lets tail}
    (hs : Sem p c A n Γ fl (.e s0 (.pair ta tb) rs))
    (hpc : paramCtx Γ [x, y] [ta, tb] [q, r] = some Γ'')
    (hbody : ∀ m ≤ n, Sem p c A m Γ'' fl (.b fm body τ lets tail)) (hk : k ≠ .binding) :
    Sem p c A (n+1) Γ fl (.b fm (.match τ s0 [.arm .pair [x, y] body]) τ
      (.mk (.bind (.generated k j)) ty rs :: .mk (.tuple [q, r]) ty' (.move (.generated k j)) :: lets) tail) := by
  refine sem_matchHolder hs hk ?_
  intro env renv v o hrel hw hl ho hst
  obtain ⟨j0, shape, xs, bd, fields, env', m', hm, hj, hd, hb, hbody', hprev⟩ := evalArms_find n _ o ho hst
  rcases j0 with _ | j0 <;> simp at hj
  obtain ⟨rfl, rfl, rfl⟩ := hj
  cases v with
  | pair a b => ?_
  | _ => simp [TargetSemantics.destruct] at hd
  simp [TargetSemantics.destruct] at hd
  subst hd
  simp only [WT] at hw
  obtain ⟨renv', hbp, hrel'⟩ := param_env Γ [x, y] [ta, tb] [q, r] Γ'' [a, b] env env' renv hpc hrel
    (by simp [WTL, hw.1, hw.2]) hb
  obtain ⟨bs, rfl, hfr⟩ := paramCtx_fresh Γ [x, y] [ta, tb] [q, r] Γ'' [a, b] renv _ hpc hbp
  obtain ⟨ext, rfl⟩ := paramCtx_ext Γ [x, y] [ta, tb] [q, r] Γ'' hpc
  have hpat : RustSemantics.bindPattern (.tuple [q, r]) (.pair a b) renv = some (bs ++ renv) := by
    simp only [RustSemantics.bindPattern]; exact hbp
  have hmove : RC c renv (.move (.generated k j)) (.value (.pair a b)) := rc_read rfl hl
  obtain ⟨hwt, hcase⟩ := hbody m' (by omega) env' _ hrel' o hbody' hst
  refine ⟨hwt, ?_⟩
  rcases hcase with ⟨new, hnew, hlets, ro, hro, hrc⟩ | ⟨rfl, h, hrh, hraise⟩
  · refine .inl ⟨new ++ bs, fresh_append (fresh_ext hnew) hfr, ?_, ro, hro, by simpa using hrc⟩
    simpa using LetsOK.cons hmove hpat hlets
  · exact .inr ⟨rfl, h, hrh, .later hmove hpat hraise⟩

end rules
end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

/-! Record fields. -/

theorem fieldPats_none : ∀ (n : Nat) (i : RIdent) (fs : List Value) (renv : REnv), fs.length = n →
    RustSemantics.bindPatterns (fieldPats none n i) fs renv = some renv
  | 0, _, [], _, _ => by simp [fieldPats, RustSemantics.bindPatterns]
  | n+1, i, f :: fs, renv, h => by
    simp only [List.length_cons, Nat.add_right_cancel_iff] at h
    simp [fieldPats, RustSemantics.bindPatterns, RustSemantics.bindPattern.eq_1, fieldPats_none n i fs renv h]
  | 0, _, _ :: _, _, h => by simp at h
  | _+1, _, [], _, h => by simp at h

theorem fieldPats_some : ∀ (pos n : Nat) (i : RIdent) (fs : List Value) (renv : REnv) (sel : Value),
    fs.length = n → fs[pos]? = some sel →
    RustSemantics.bindPatterns (fieldPats (some pos) n i) fs renv = some ((i, sel) :: renv)
  | _, 0, _, [], _, _, _, h => by simp at h
  | 0, n+1, i, f :: fs, renv, sel, hl, hs => by
    simp only [List.length_cons, Nat.add_right_cancel_iff] at hl
    simp at hs; subst hs
    simp [fieldPats, RustSemantics.bindPatterns, RustSemantics.bindPattern.eq_2, fieldPats_none n i fs _ hl]
  | pos+1, n+1, i, f :: fs, renv, sel, hl, hs => by
    simp only [List.length_cons, Nat.add_right_cancel_iff] at hl
    simp at hs
    simp [fieldPats, RustSemantics.bindPatterns, RustSemantics.bindPattern.eq_1,
      fieldPats_some pos n i fs renv sel hl hs]
  | _, 0, _, _ :: _, _, _, h, _ => by simp at h
  | _, _+1, _, [], _, _, h, _ => by simp at h

theorem tyBeq_unit {t : Ty} (h : tyBeq t .unit = true) : t = .unit := by
  cases t <;> simp [tyBeq] at h ⊢

theorem rc_partRead {c renv rd j w} (h : partRead rd j = true) :
    RC c ((j, w) :: renv) rd (.value w) := by
  cases rd <;> simp [partRead] at h
  · exact rc_read (r := .move _) rfl (rlookup_cons_same h)
  · exact rc_read (r := .unbox _) rfl (rlookup_cons_same h)

theorem rc_matchOn_halt {c renv sc rarms h} (hs : RC c renv sc h) (hh : Halt h) :
    RC c renv (.matchOn sc rarms) h := by
  obtain ⟨n, hn⟩ := hs.at
  refine ⟨hs.1, n+1, ?_⟩
  rw [RustSemantics.eval.eq_15, hn n (Nat.le_refl _)]
  rcases hh with rfl | rfl <;> rfl

theorem realizesM_value {fm fl v ro} (h : RealizesM fm fl (.value v) ro) :
    ro = .value (cond fm (.ok v) v) := by
  cases fm <;> cases ro <;> simp_all [RealizesM, Realizes, RealizesF]

theorem wtl_get {p c A} : ∀ {vs : List Value} {ts : List Ty} {pos : Nat} {v : Value} {t : Ty}, WTL p c A vs ts →
    vs[pos]? = some v → ts[pos]? = some t → WT p c A v t
  | [], _, _, _, _, _, h, _ => by simp at h
  | _ :: _, [], _, _, _, hw, _, _ => by simp [WTL] at hw
  | x :: vs, t0 :: ts, 0, v, t, hw, h1, h2 => by
    simp at h1 h2; subst h1; subst h2; simp only [WTL] at hw; exact hw.1
  | x :: vs, t0 :: ts, pos+1, v, t, hw, h1, h2 => by
    simp at h1 h2; simp only [WTL] at hw; exact wtl_get hw.2 h1 h2

section rules
variable {p : Program} {c : RCrate} {A : Flags}

/-- A field read: the record's value, and the field the rendering's one-arm
match binds. -/
theorem field_out {fl fm t rd j sel} (hfm : (!fm || fl) = true) (hw : WT p c A sel t)
    (hpr : partRead rd j = true) (renv : REnv) :
    ∃ ro, RealizesM fm fl (.value sel) ro ∧ RCB c ((j, sel) :: renv) (okBlock fm (tyBeq t .unit) rd) ro := by
  have hrd := rc_partRead (c := c) (renv := renv) (w := sel) hpr
  cases fm with
  | false => exact ⟨.value sel, rfl, by simpa [okBlock] using rcb_nil hrd⟩
  | true =>
    cases hu : tyBeq t .unit with
    | false =>
      refine ⟨.value (.ok sel), rfl, ?_⟩
      simpa [okBlock, hu] using rcb_nil (rc_succeed hrd)
    | true =>
      have := tyBeq_unit hu; subst this
      have hsel := wt_unit hw; subst hsel
      refine ⟨.value (.ok .unit), rfl, ?_⟩
      simp only [okBlock, hu, if_true]
      exact block_of_lets (.cons hrd rfl .nil)
        (rc_succeed (rc_lit (v := .unit) (by simp [RustSemantics.literal])))

theorem eval_field {n env r0 pos} :
    TargetSemantics.eval (n+1) p env (.field r0 pos) =
      match TargetSemantics.eval n p env r0 with
      | .value (.adt _ fs) st => match fs[pos]? with
        | none => .stuck
        | some sel => .value sel (st + 1 + pos)
      | .value _ _ => .stuck
      | .overflow st => .overflow (st + 1)
      | .stuck => .stuck
      | .exhausted => .exhausted := by
  rw [TargetSemantics.eval.eq_14]
  cases TargetSemantics.eval n p env r0 with
  | value v st =>
    cases v <;> try rfl
    rename_i k fs
    simp only [index_eq]
    rfl
  | _ => rfl

/-- What reading field `pos` of a record computes, given the record's
semantics in Rust. -/
theorem sem_fieldGen {n Γ fl fm r0 i pos ts t rr k j rd lets tail}
    (hr : Sem p c A n Γ fl (.e r0 (.adt i) rr))
    (hone : (p.adts[i]?).map (·.constructors.length) = some 1) (hts : adtFields p i 0 = some ts)
    (hpos : ts[pos]? = some t) (hpr : partRead rd (.generated k j) = true) (hfm : (!fm || fl) = true)
    (hcode : ∀ renv v ro, RC c renv rr (.value v) →
      (∀ R, RCA c R v [.mk (.adt i 0 (fieldPats (some pos) ts.length (.generated k j)))
        (okBlock fm (tyBeq t .unit) rd)] ro) →
      ∃ new, Fresh Γ new ∧ LetsOK c renv lets (new ++ renv) ∧ RC c (new ++ renv) tail ro)
    (hhalt : ∀ renv h, RC c renv rr h → Halt h →
      (∃ new, Fresh Γ new ∧ LetsOK c renv lets (new ++ renv) ∧ RC c (new ++ renv) tail h) ∨
        LetsRaise c renv lets h) :
    Sem p c A (n+1) Γ fl (.b fm (.field r0 pos) t lets tail) := by
  intro env renv hrel o ho hst
  rw [eval_field] at ho
  cases h1 : TargetSemantics.eval n p env r0 with
  | value v s1 =>
    rw [h1] at ho
    obtain ⟨hw1, ro1, hro1, hrc1⟩ := hr env renv hrel (.value v) (by rw [h1]; rfl) nofun
    cases ro1 with
    | value w =>
      simp only [Realizes] at hro1; subst hro1
      cases w with
      | adt k' fs =>
        have hw := hw1 _ rfl
        simp only [WT] at hw
        obtain ⟨tys, htys, hwl⟩ := hw
        have hk0 : k' = 0 := by
          simp only [adtFields] at htys
          cases ha : p.adts[i]? with
          | none => simp [ha] at htys
          | some adt =>
            simp [ha] at hone htys
            have := (List.getElem?_eq_some_iff.mp htys).1
            omega
        subst hk0
        rw [hts] at htys; cases htys
        cases hsel : fs[pos]? with
        | none => simp [hsel, obs] at ho; subst ho; exact absurd rfl hst
        | some sel =>
          simp only [hsel, obs, Option.some.injEq] at ho; subst ho
          have hwsel : WT p c A sel t := wtl_get hwl hsel hpos
          obtain ⟨ro, hro, _⟩ := field_out (renv := renv) hfm hwsel hpr
          have hrca : ∀ R, RCA c R (.adt 0 fs) [.mk (.adt i 0 (fieldPats (some pos) ts.length (.generated k j)))
              (okBlock fm (tyBeq t .unit) rd)] ro := by
            intro R
            obtain ⟨ro', hro', hrcb⟩ := field_out (renv := R) hfm hwsel hpr
            have hbind := fieldPats_some pos ts.length (.generated k j) fs R sel (wtl_length hwl) hsel
            have : ro' = ro := by rw [realizesM_value hro, realizesM_value hro']
            subst this
            exact rca_hit (by simp [RustSemantics.bindPattern, hbind]) hrcb
          obtain ⟨new, hnew, hl, hrc⟩ := hcode renv _ ro hrc1 hrca
          exact ⟨fun v hv => by cases hv; exact hwsel, .inl ⟨new, hnew, hl, ro, hro, hrc⟩⟩
      | _ => simp [obs] at ho; subst ho; exact absurd rfl hst
    | _ => simp [Realizes] at hro1
  | overflow s1 =>
    rw [h1] at ho
    simp only [obs, Option.some.injEq] at ho; subst ho
    obtain ⟨_, ro1, hro1, hrc1⟩ := hr env renv hrel .overflow (by rw [h1]; rfl) nofun
    refine ⟨(fun _ h => by cases h), ?_⟩
    rcases hhalt renv ro1 hrc1 (realizes_halt hro1) with ⟨new, hnew, hl, hrc⟩ | hraise
    · exact .inl ⟨new, hnew, hl, ro1, realizesM_overflow hro1, hrc⟩
    · exact .inr ⟨rfl, ro1, realizesM_overflow hro1, hraise⟩
  | stuck => rw [h1] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
  | exhausted => rw [h1] at ho; simp [obs] at ho

theorem sem_fieldRead {n Γ fl fm r0 i pos ts t rr k j rd}
    (hr : Sem p c A n Γ fl (.e r0 (.adt i) rr))
    (hone : (p.adts[i]?).map (·.constructors.length) = some 1) (hts : adtFields p i 0 = some ts)
    (hpos : ts[pos]? = some t) (hpr : partRead rd (.generated k j) = true) (hfm : (!fm || fl) = true) :
    Sem p c A (n+1) Γ fl (.b fm (.field r0 pos) t []
      (.matchOn rr [.mk (.adt i 0 (fieldPats (some pos) ts.length (.generated k j)))
        (okBlock fm (tyBeq t .unit) rd)])) :=
  sem_fieldGen hr hone hts hpos hpr hfm
    (fun renv _ _ hrc hrca => ⟨[], fresh_nil, .nil, by simpa using rc_matchOn hrc (hrca renv)⟩)
    (fun _ _ hrc hh => .inl ⟨[], fresh_nil, .nil, by simpa using rc_matchOn_halt hrc hh⟩)

theorem sem_fieldHeld {n Γ fl fm r0 i pos ts t rr k j rd k' j' ty}
    (hr : Sem p c A n Γ fl (.e r0 (.adt i) rr))
    (hone : (p.adts[i]?).map (·.constructors.length) = some 1) (hts : adtFields p i 0 = some ts)
    (hpos : ts[pos]? = some t) (hpr : partRead rd (.generated k j) = true) (hk' : k' ≠ .binding)
    (hfm : (!fm || fl) = true) :
    Sem p c A (n+1) Γ fl (.b fm (.field r0 pos) t [.mk (.bind (.generated k' j')) ty rr]
      (.matchOn (.move (.generated k' j')) [.mk (.adt i 0 (fieldPats (some pos) ts.length (.generated k j)))
        (okBlock fm (tyBeq t .unit) rd)])) :=
  sem_fieldGen hr hone hts hpos hpr hfm
    (fun _ v _ hrc hrca => ⟨[(.generated k' j', v)], by simpa using fresh_temp (new := []) (j := j') (v := v) hk' fresh_nil,
      .cons hrc (by simp [RustSemantics.bindPattern.eq_2]) .nil,
      by simpa using rc_matchOn (rc_read (r := .move (.generated k' j')) rfl (rlookup_cons_self _ _ _)) (hrca _)⟩)
    (fun _ _ hrc hh => .inr (.here hrc hh))

end rules
end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

/-! Arms that rebuild what they matched. -/

theorem sameIdent_iff (a b : RIdent) : RustSemantics.sameIdent a b = true ↔ a = b := by
  cases a with
  | generated k n =>
    cases b with
    | generated k' n' =>
      cases k <;> cases k' <;> simp [RustSemantics.sameIdent, RustSemantics.kindNumber, Nat.beq_eq]
    | exported _ => simp [RustSemantics.sameIdent]
  | exported s =>
    cases b with
    | generated _ _ => simp [RustSemantics.sameIdent]
    | exported s' => simp [RustSemantics.sameIdent, RustSemantics.LexLeanRuntime.equal]

theorem rlookup_eq_of_same {renv : REnv} {i j : RIdent} (h : RustSemantics.sameIdent i j = true) :
    RustSemantics.lookup renv j = RustSemantics.lookup renv i := by
  rw [(sameIdent_iff i j).mp h]

theorem readOf_rc {c renv a j v} (h : readOf a = some j) (hl : RustSemantics.lookup renv j = some v) :
    RC c renv a (.value v) := rc_read h hl

theorem entries_none_of {i : RIdent} : ∀ (qs : List RustSyntax.Pat) (fs : List Value),
    (qs.filterMap bindOf).any (RustSemantics.sameIdent i) = false →
    RustSemantics.lookup (entriesOf qs fs) i = none
  | [], _, _ => by simp [entriesOf, RustSemantics.lookup]
  | _ :: _, [], _ => by simp [entriesOf, RustSemantics.lookup]
  | q :: qs, f :: fs, h => by
    simp only [entriesOf, rlookup_append]
    cases hq : bindOf q with
    | none =>
      simp only [List.filterMap_cons, hq] at h
      simp [entries_none_of qs fs h, RustSemantics.lookup]
    | some i' =>
      simp only [List.filterMap_cons, hq, List.any_cons, Bool.or_eq_false_iff] at h
      rw [entries_none_of qs fs h.2]
      have : RustSemantics.sameIdent i' i = false := by
        cases hs : RustSemantics.sameIdent i' i
        · rfl
        · have := (sameIdent_iff _ _).mp hs; subst this; rw [sameIdent_self] at h; simp at h
      simp [RustSemantics.lookup, this]

theorem entries_lookup : ∀ (qs : List RustSyntax.Pat) (fs : List Value) (j : Nat) (i : RIdent) (f : Value),
    identsDistinct (qs.filterMap bindOf) = true → qs[j]? = some (.bind i) → fs[j]? = some f →
    RustSemantics.lookup (entriesOf qs fs) i = some f
  | [], _, _, _, _, _, h, _ => by simp at h
  | _ :: _, [], _, _, _, _, _, h => by simp at h
  | q :: qs, f0 :: fs, 0, i, f, hd, hq, hf => by
    simp at hq hf; subst hq; subst hf
    simp only [List.filterMap_cons, bindOf, identsDistinct, Bool.and_eq_true, Bool.not_eq_true'] at hd
    simp only [entriesOf, bindOf, rlookup_append, entries_none_of qs fs hd.1, Option.none_or]
    simp [RustSemantics.lookup, sameIdent_self]
  | q :: qs, f0 :: fs, j+1, i, f, hd, hq, hf => by
    simp at hq hf
    have hd' : identsDistinct (qs.filterMap bindOf) = true := by
      cases hb : bindOf q with
      | none => simpa [List.filterMap_cons, hb] using hd
      | some _ =>
        simp only [List.filterMap_cons, hb, identsDistinct, Bool.and_eq_true] at hd; exact hd.2
    simp only [entriesOf, rlookup_append, entries_lookup qs fs j i f hd' hq hf]
    rfl

theorem rebuildArg_rc {p c A t q a x E} (h : rebuildArg t q a = true) (hw : WT p c A x t)
    (hl : ∀ i, bindOf q = some i → RustSemantics.lookup E i = some x) : RC c E a (.value x) := by
  simp only [rebuildArg] at h
  cases hq : bindOf q with
  | some i =>
    rw [hq] at h; simp only at h
    cases hr : readOf a with
    | none => rw [hr] at h; cases h
    | some j =>
      rw [hr] at h; simp only at h
      exact rc_read hr (by rw [rlookup_eq_of_same h]; exact hl i hq)
  | none =>
    rw [hq] at h; simp only [Bool.and_eq_true] at h
    obtain ⟨⟨_, ht⟩, hlit⟩ := h
    have := tyBeq_unit ht; subst this
    have hx := wt_unit hw; subst hx
    cases ha : litOf a with
    | none => rw [ha] at hlit; cases hlit
    | some l =>
      rw [ha] at hlit; simp only at hlit
      cases hv : RustSemantics.literal l with
      | none => rw [hv] at hlit; cases hlit
      | some v =>
        rw [hv] at hlit; simp only at hlit
        cases v <;> simp at hlit
        have : a = .lit l := by cases a <;> simp [litOf] at ha; rw [ha]
        subst this
        exact rc_lit hv

theorem rcl_rebuild {p c A E} : ∀ (ts : List Ty) (qs : List RustSyntax.Pat) (args : List RExpr) (fs : List Value),
    rebuildArgs ts qs args = true → WTL p c A fs ts →
    (∀ (j : Nat) (i : RIdent) (f : Value), qs[j]? = some (RustSyntax.Pat.bind i) → fs[j]? = some f →
      RustSemantics.lookup E i = some f) →
    RCL c E args (.values fs)
  | [], [], [], [], _, _, _ => rcl_nil
  | t :: ts, q :: qs, a :: as, f :: fs, h, hw, hl => by
    simp only [rebuildArgs, Bool.and_eq_true] at h
    simp only [WTL] at hw
    refine rcl_cons (rebuildArg_rc h.1 hw.1 (fun i hi => hl 0 i f (by simp [bindOf_eq hi]) rfl))
      (rcl_rebuild ts qs as fs h.2 hw.2 (fun j i f' hq hf => hl (j+1) i f' (by simpa using hq) (by simpa using hf)))
  | [], [], [], _ :: _, _, hw, _ => by simp [WTL] at hw
  | _ :: _, _, _, [], _, hw, _ => by simp [WTL] at hw
  | [], _ :: _, _, _, h, _, _ => by simp [rebuildArgs] at h
  | [], [], _ :: _, _, h, _, _ => by simp [rebuildArgs] at h
  | _ :: _, [], _, _ :: _, h, _, _ => by simp [rebuildArgs] at h
  | _ :: _, _ :: _, [], _ :: _, h, _, _ => by simp [rebuildArgs] at h

theorem rebuildArg_simple {t q a} (h : rebuildArg t q a = true) : simplePat q = true := by
  simp only [rebuildArg] at h
  cases hq : bindOf q with
  | some i => simp [simplePat, hq]
  | none => rw [hq] at h; simp only [Bool.and_eq_true] at h; simp [simplePat, h.1.1]

theorem rebuildArgs_simple : ∀ {ts qs args}, rebuildArgs ts qs args = true →
    qs.all simplePat = true ∧ qs.length = ts.length
  | [], [], [], _ => ⟨rfl, rfl⟩
  | _ :: _, _ :: _, _ :: _, h => by
    simp only [rebuildArgs, Bool.and_eq_true] at h
    obtain ⟨h1, h2⟩ := rebuildArgs_simple h.2
    simp [rebuildArg_simple h.1, h1, h2]
  | [], _ :: _, _, h | [], [], _ :: _, h | _ :: _, [], _, h | _ :: _, _ :: _, [], h => by
    simp [rebuildArgs] at h

theorem ctorOf_eq {t k args} (h : ctorOf t = some (k, args)) : t = .construct k args := by
  cases t <;> simp [ctorOf] at h; obtain ⟨rfl, rfl⟩ := h; rfl

theorem single_rebuild {p c A t q a x renv renv1}
    (h : rebuildArg t q a = true) (hw : WT p c A x t)
    (hb : RustSemantics.bindPattern q x renv = some renv1) : RC c renv1 a (.value x) := by
  refine rebuildArg_rc h hw (fun i hi => ?_)
  rw [bindOf_eq hi] at hb
  simp [RustSemantics.bindPattern.eq_2] at hb; subst hb
  exact rlookup_cons_self _ _ _

/-- A rebuilding arm's value is the value it matched. -/
theorem rebuild_value {p c A st q tail v renv renv1} (h : rebuildTail p st q tail = true)
    (hw : WT p c A v st) (hb : RustSemantics.bindPattern q v renv = some renv1) :
    RC c renv1 tail (.value v) := by
  unfold rebuildTail at h
  split at h
  · -- none
    cases hc : ctorOf tail with
    | none => simp [hc] at h
    | some ka =>
      obtain ⟨k, args⟩ := ka
      rw [hc] at h
      split at h
      · rename_i ty heq
        cases heq
        rw [ctorOf_eq hc]
        cases v <;> simp [RustSemantics.bindPattern] at hb
        exact rc_construct rcl_nil rfl
      · cases h
  · -- some
    rename_i q' t
    cases hc : ctorOf tail with
    | none => simp [hc] at h
    | some ka =>
      obtain ⟨k, args⟩ := ka
      rw [hc] at h
      split at h
      · rename_i a heq
        cases heq
        rw [ctorOf_eq hc]
        cases v <;> simp [RustSemantics.bindPattern] at hb
        rename_i x
        simp only [WT] at hw
        exact rc_construct (rcl_cons (single_rebuild h hw hb) rcl_nil) rfl
      · cases h
  · -- ok
    rename_i q' t _
    cases hc : ctorOf tail with
    | none => simp [hc] at h
    | some ka =>
      obtain ⟨k, args⟩ := ka
      rw [hc] at h
      split at h
      · rename_i _ _ a heq
        cases heq
        rw [ctorOf_eq hc]
        cases v <;> simp [RustSemantics.bindPattern] at hb
        simp only [WT] at hw
        exact rc_construct (rcl_cons (single_rebuild h hw hb) rcl_nil) rfl
      · cases h
  · -- err
    rename_i q' _ t
    cases hc : ctorOf tail with
    | none => simp [hc] at h
    | some ka =>
      obtain ⟨k, args⟩ := ka
      rw [hc] at h
      split at h
      · rename_i _ _ a heq
        cases heq
        rw [ctorOf_eq hc]
        cases v <;> simp [RustSemantics.bindPattern] at hb
        simp only [WT] at hw
        exact rc_construct (rcl_cons (single_rebuild h hw hb) rcl_nil) rfl
      · cases h
  · -- ordering
    rename_i o
    cases hl : litOf tail with
    | none => simp [hl] at h
    | some l =>
      rw [hl] at h
      split at h
      · rename_i o' heq
        cases heq
        have : tail = .lit (.ordering o') := by cases tail <;> simp [litOf] at hl; rw [hl]
        subst this
        cases v <;> simp [RustSemantics.bindPattern] at hb
        rename_i o0
        obtain ⟨hs, rfl⟩ := hb
        have : o' = o0 := by cases o <;> cases o' <;> cases o0 <;> simp_all [TargetSemantics.sameOrder]
        subst this
        exact rc_lit rfl
      · cases h
  · -- adt
    rename_i i k qs i0
    cases hc : ctorOf tail with
    | none => simp [hc] at h
    | some ka =>
      obtain ⟨kk, args⟩ := ka
      rw [hc] at h
      split at h
      · rename_i i' k' args' heq
        cases heq
        simp only [Bool.and_eq_true, beq_iff_eq] at h
        obtain ⟨⟨⟨⟨rfl, rfl⟩, rfl⟩, hdist⟩, hargs⟩ := h
        cases hts : adtFields p i k with
        | none => rw [hts] at hargs; cases hargs
        | some ts =>
          rw [hts] at hargs; simp only at hargs
          rw [ctorOf_eq hc]
          cases v <;> simp [RustSemantics.bindPattern] at hb
          rename_i k0 fs
          obtain ⟨rfl, hbp⟩ := hb
          simp only [WT] at hw
          obtain ⟨tys, htys, hwl⟩ := hw
          rw [hts] at htys; cases htys
          obtain ⟨hsimple, hlen⟩ := rebuildArgs_simple hargs
          rw [bp_entries qs fs renv hsimple (by rw [hlen, wtl_length hwl])] at hbp
          cases hbp
          refine rc_construct (rcl_rebuild ts qs args fs hargs hwl (fun j i f hq hf => ?_)) rfl
          rw [rlookup_append, entries_lookup qs fs j i f hdist hq hf]; rfl
      · cases h
  · cases h

end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

theorem rcb_det {c renv b o1 o2} (h1 : RCB c renv b o1) (h2 : RCB c renv b o2) : o1 = o2 := by
  obtain ⟨n1, hn1⟩ := h1.at
  obtain ⟨n2, hn2⟩ := h2.at
  rw [← hn1 (max n1 n2) (Nat.le_max_left _ _), ← hn2 (max n1 n2) (Nat.le_max_right _ _)]

theorem rebuild_rca {p c A st v renv} : ∀ (rarms : List RustSyntax.Arm) (ro : ROut),
    rarms.all (rebuildsArm p st) = true → WT p c A v st → RCA c renv v rarms ro →
    ro = .value v ∨ ro = .stuck
  | [], ro, _, _, ⟨hne, n, hn⟩ => by
    cases n with
    | zero => simp [RustSemantics.evalArms.eq_1] at hn; exact absurd hn.symm hne
    | succ n => rw [RustSemantics.evalArms.eq_2] at hn; exact .inr hn.symm
  | .mk q blk :: rest, ro, hall, hw, ⟨hne, n, hn⟩ => by
    simp only [List.all_cons, Bool.and_eq_true] at hall
    cases n with
    | zero => simp [RustSemantics.evalArms.eq_1] at hn; exact absurd hn.symm hne
    | succ n =>
      rw [RustSemantics.evalArms.eq_3] at hn
      cases hb : RustSemantics.bindPattern q v renv with
      | none =>
        rw [hb] at hn
        exact rebuild_rca rest ro hall.2 hw ⟨hne, n, hn⟩
      | some renv1 =>
        rw [hb] at hn; simp only at hn
        obtain ⟨lets, tail⟩ := blk
        simp only [rebuildsArm, Bool.and_eq_true, List.isEmpty_iff] at hall
        obtain ⟨⟨rfl, hrt⟩, _⟩ := hall
        have hv := rcb_nil (rebuild_value hrt hw hb)
        exact .inl (rcb_det ⟨hne, n, hn⟩ hv)

section rules
variable {p : Program} {c : RCrate} {A : Flags}

theorem sem_matchRebuilt {n Γ fl fm st s0 arms idx rarms U lets tail}
    (hs : Sem p c A n Γ fl (.b fm s0 st lets tail))
    (hm : ∀ m ≤ n, Sem p c A m Γ fl (.m st false arms idx false st rarms))
    (hreb : rarms.all (rebuildsArm p st) = true)
    (hU : validU p c U = true) (hcov : covered p c U st arms idx = true) :
    Sem p c A (n+1) Γ fl (.b fm (.match st s0 arms) st lets tail) := by
  intro env renv hrel o ho hst
  rw [eval_match] at ho
  cases h1 : TargetSemantics.eval n p env s0 with
  | value v s1 =>
    rw [h1] at ho
    simp only [obs_addSteps] at ho
    obtain ⟨hw0, _⟩ := hs env renv hrel (.value v) (by rw [h1]; rfl) nofun
    have hw := hw0 v rfl
    obtain ⟨j, shape, xs, body, fields, env', m', hm', hmem, hj, hd, hb, hbody⟩ :=
      taken_arm hU hcov hw ho hst
    obtain ⟨_, _, ro, hro, hrca⟩ :=
      hm m' (by omega) env renv hrel v hw j shape xs body hmem hj fields env' hd hb o hbody hst
    have hov : o = .value v := by
      rcases rebuild_rca rarms ro hreb hw hrca with rfl | rfl
      · cases o <;> simp [RealizesM, Realizes] at hro; rw [hro]
      · cases o <;> simp [RealizesM, Realizes] at hro
    subst hov
    exact hs env renv hrel _ (by rw [h1]; rfl) hst
  | overflow s1 =>
    rw [h1] at ho
    simp only [obs, Option.some.injEq] at ho; subst ho
    exact hs env renv hrel .overflow (by rw [h1]; rfl) nofun
  | stuck => rw [h1] at ho; simp [obs] at ho; subst ho; exact absurd rfl hst
  | exhausted => rw [h1] at ho; simp [obs] at ho

end rules
end LexLeanPreservation.Rust
