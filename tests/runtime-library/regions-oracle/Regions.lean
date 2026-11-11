module
public import Init
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
@[expose] public section
namespace LexLeanRegions

/-! Decision fragments and their local box obligations (UOR-Foundation/PrismPM#605).

A decision fragment is a first-match judge over a request of natural-number fields: rules, each a
conjunction of comparisons of one field with one constant, and an `otherwise` answer. A property
names a conjunction of comparisons and an answer the judge must (`always`) or must not give there.

The public definitions mirror, shape for shape, the Lean the LexLean backend emits for such a
module: `Nat.blt`, `Nat.ble`, `Nat.beq` and `!` for comparisons, `&&` folded from the left for
conjunctions, `if b then` for a rule's test. So a module's `breaks` definition is definitionally
`breaks` of its lowered data, and `holds` closes a property with one Boolean obligation the
kernel decides. The proofs run through `Spec`, the same semantics written with `if`. -/

structure Cond where
  field : Nat
  op : Nat
  value : Nat

structure Rule where
  when : List Cond
  outcome : Nat

structure Property where
  always : Bool
  when : List Cond
  outcome : Nat

def get : List Nat → Nat → Nat
  | [], _ => 0
  | v :: _, 0 => v
  | _ :: vs, n + 1 => get vs n

/-- Operator codes, `x` the field's value and `c` the constant: 0 `x < c`, 1 `x ≤ c`,
2 `c < x`, 3 `c ≤ x`, 4 `x = c`, 5 `c = x`, 6 `x ≠ c`, 7 and above `c ≠ x`. -/
def opHolds : Nat → Nat → Nat → Bool
  | 0, c, x => Nat.blt x c
  | 1, c, x => Nat.ble x c
  | 2, c, x => Nat.blt c x
  | 3, c, x => Nat.ble c x
  | 4, c, x => Nat.beq x c
  | 5, c, x => Nat.beq c x
  | 6, c, x => !(Nat.beq x c)
  | _ + 7, c, x => !(Nat.beq c x)

def condHolds (c : Cond) (x : List Nat) : Bool := opHolds c.op c.value (get x c.field)

def allHoldFrom (x : List Nat) : Bool → List Cond → Bool
  | acc, [] => acc
  | acc, c :: cs => allHoldFrom x (acc && condHolds c x) cs

/-- The conjunction, folded from the left as the backend writes it. -/
def allHold (cs : List Cond) (x : List Nat) : Bool := allHoldFrom x true cs

/-- First match decides; `d` is `otherwise`. -/
def judge : List Rule → Nat → List Nat → Nat
  | [], d, _ => d
  | r :: rs, d, x => if allHold r.when x then r.outcome else judge rs d x

/-- Answer `o` breaks property `p` where `p` speaks. -/
def bad (p : Property) (o : Nat) : Bool :=
  match p.always with
  | true => !(Nat.beq o p.outcome)
  | false => Nat.beq o p.outcome

def breaks (rules : List Rule) (d : Nat) (p : Property) (x : List Nat) : Bool :=
  allHold p.when x && bad p (judge rules d x)

namespace Spec

/-- The same comparisons on the six codes of the proofs: 0 `<`, 1 `≤`, 2 `>`, 3 `≥`, 4 `=`,
5 and above `≠`. -/
def opHolds (op c x : Nat) : Bool :=
  if op = 0 then (if x < c then true else false)
  else if op = 1 then (if x ≤ c then true else false)
  else if op = 2 then (if c < x then true else false)
  else if op = 3 then (if c ≤ x then true else false)
  else if op = 4 then (if x = c then true else false)
  else (if x = c then false else true)

def condHolds (c : Cond) (x : List Nat) : Bool := opHolds c.op c.value (get x c.field)

def allHold : List Cond → List Nat → Bool
  | [], _ => true
  | c :: cs, x => if condHolds c x then allHold cs x else false

def judge : List Rule → Nat → List Nat → Nat
  | [], d, _ => d
  | r :: rs, d, x => if allHold r.when x then r.outcome else judge rs d x

def breaksWith (p : Property) (x : List Nat) (o : Nat) : Bool :=
  if allHold p.when x then
    (if p.always then (if o = p.outcome then false else true)
     else (if o = p.outcome then true else false))
  else false

def breaks (rules : List Rule) (d : Nat) (p : Property) (x : List Nat) : Bool :=
  breaksWith p x (judge rules d x)

/-! ## Boxes

For every property `p` and every rule `r` whose answer would break `p`, one obligation: the part
of `r`'s box inside `p`'s box is covered by the boxes of the rules before `r`. One more obligation
covers `otherwise`. Each is box subtraction over a few intervals, so the kernel decides it quickly.
Rules use only the exact comparisons (`<`, `≤`, `>`, `≥`, `=`), whose box is exactly where they
hold; a property may also use `≠`, whose box over-covers. -/

/-- A box: one closed interval per field. -/
abbrev Box := List (Nat × Nat)

def inBox : Box → List Nat → Bool
  | [], [] => true
  | [], _ :: _ => false
  | _ :: _, [] => false
  | (l, h) :: bs, v :: vs => if l ≤ v then (if v ≤ h then inBox bs vs else false) else false

def nonEmpty : Box → Bool
  | [] => true
  | (l, h) :: bs => if l ≤ h then nonEmpty bs else false

theorem inBox_cons (l h : Nat) (bs : Box) (v : Nat) (vs : List Nat) :
    inBox ((l, h) :: bs) (v :: vs) = (Decidable.decide (l ≤ v) && Decidable.decide (v ≤ h) && inBox bs vs) := by
  simp only [inBox]; by_cases a : l ≤ v <;> by_cases b : v ≤ h <;> simp [a, b]

theorem inBox_nil_cons (v : Nat) (vs : List Nat) : inBox [] (v :: vs) = false := rfl

theorem inBox_cons_nil (p : Nat × Nat) (bs : Box) : inBox (p :: bs) [] = false := by
  obtain ⟨l, h⟩ := p; rfl

theorem nonEmpty_of_inBox (b : Box) : ∀ (x : List Nat), inBox b x = true → nonEmpty b = true := by
  induction b with
  | nil => intro _ _; rfl
  | cons p bs ih =>
    intro x hx
    obtain ⟨l, h⟩ := p
    cases x with
    | nil => rw [inBox_cons_nil] at hx; exact absurd hx Bool.false_ne_true
    | cons v vs =>
      simp only [inBox_cons, Bool.and_eq_true, decide_eq_true_eq] at hx
      simp only [nonEmpty]; rw [if_pos (by omega)]; exact ih vs hx.2

/-! ## Subtraction -/

/-- Boxes that together cover every point of `A` outside `B`. -/
def sub : Box → Box → List Box
  | [], [] => []
  | [], _ :: _ => [[]]
  | a :: as, [] => [a :: as]
  | (al, ah) :: as, (bl, bh) :: bs =>
    if ah < bl then [(al, ah) :: as] else if bh < al then [(al, ah) :: as]
    else (if al < bl then [(al, bl - 1) :: as] else []) ++
         (if bh < ah then [(bh + 1, ah) :: as] else []) ++
         (sub as bs).map (fun p => (max al bl, min ah bh) :: p)

theorem sub_cover (A : Box) : ∀ (B : Box) (x : List Nat), inBox A x = true → inBox B x = false →
    ∃ p ∈ sub A B, inBox p x = true := by
  induction A with
  | nil =>
    intro B x ha hb
    cases x with
    | cons v vs => rw [inBox_nil_cons] at ha; exact absurd ha Bool.false_ne_true
    | nil =>
      cases B with
      | nil => exact absurd hb (by simp [inBox])
      | cons b bs => exact ⟨[], by simp [sub], rfl⟩
  | cons a as ih =>
    intro B x ha hb
    cases B with
    | nil => exact ⟨a :: as, by simp [sub], ha⟩
    | cons b bs =>
      obtain ⟨al, ah⟩ := a
      obtain ⟨bl, bh⟩ := b
      cases x with
      | nil => rw [inBox_cons_nil] at ha; exact absurd ha Bool.false_ne_true
      | cons v vs =>
        have hA := ha
        simp only [inBox_cons, Bool.and_eq_true, decide_eq_true_eq] at hA
        obtain ⟨⟨h1, h2⟩, ha'⟩ := hA
        simp only [sub]
        by_cases c1 : ah < bl
        · rw [if_pos c1]; exact ⟨_, List.mem_singleton_self _, ha⟩
        · rw [if_neg c1]
          by_cases c2 : bh < al
          · rw [if_pos c2]; exact ⟨_, List.mem_singleton_self _, ha⟩
          · rw [if_neg c2]
            by_cases lv : v < bl
            · refine ⟨(al, bl - 1) :: as, ?_, ?_⟩
              · rw [if_pos (show al < bl by omega)]; simp
              · simp only [inBox_cons, Bool.and_eq_true, decide_eq_true_eq]; exact ⟨⟨h1, by omega⟩, ha'⟩
            · by_cases rv : bh < v
              · refine ⟨(bh + 1, ah) :: as, ?_, ?_⟩
                · rw [if_pos (show bh < ah by omega)]; simp
                · simp only [inBox_cons, Bool.and_eq_true, decide_eq_true_eq]; exact ⟨⟨by omega, h2⟩, ha'⟩
              · have hbs : inBox bs vs = false := by
                  cases h : inBox bs vs
                  · rfl
                  · simp only [inBox_cons, h, Bool.and_true, Bool.and_eq_false_iff, decide_eq_false_iff_not] at hb
                    omega
                obtain ⟨p, hp, hin⟩ := ih bs vs ha' hbs
                refine ⟨(max al bl, min ah bh) :: p, ?_, ?_⟩
                · simp only [List.mem_append, List.mem_map]; exact Or.inr ⟨p, hp, rfl⟩
                · simp only [inBox_cons, Bool.and_eq_true, decide_eq_true_eq]
                  exact ⟨⟨Nat.max_le.mpr ⟨h1, by omega⟩, Nat.le_min.mpr ⟨h2, by omega⟩⟩, hin⟩

/-- `A` minus every box of `Bs`, as boxes, empty ones dropped. -/
def subAll (A : Box) : List Box → List Box
  | [] => if nonEmpty A then [A] else []
  | B :: Bs => (subAll A Bs).flatMap (fun p => (sub p B).filter nonEmpty)

theorem subAll_cover (A : Box) (x : List Nat) (ha : inBox A x = true) (Bs : List Box) :
    (∀ B ∈ Bs, inBox B x = false) → ∃ p ∈ subAll A Bs, inBox p x = true := by
  induction Bs with
  | nil => intro _; exact ⟨A, by simp [subAll, nonEmpty_of_inBox A x ha], ha⟩
  | cons B Bs ih =>
    intro h
    obtain ⟨p, hp, hin⟩ := ih (fun B' hB' => h B' (List.mem_cons_of_mem _ hB'))
    obtain ⟨q, hq, hqin⟩ := sub_cover p B x hin (h B (List.mem_cons_self ..))
    exact ⟨q, List.mem_flatMap.mpr ⟨p, hp, List.mem_filter.mpr ⟨hq, nonEmpty_of_inBox q x hqin⟩⟩, hqin⟩

/-- Empty subtraction: every point of `A` is in some box of `Bs`. -/
theorem covered_of_subAll (A : Box) (Bs : List Box) (h : subAll A Bs = []) (x : List Nat)
    (ha : inBox A x = true) : ∃ B ∈ Bs, inBox B x = true := by
  by_cases hc : ∃ B ∈ Bs, inBox B x = true
  · exact hc
  · have : ∀ B ∈ Bs, inBox B x = false := fun B hB => by
      cases hx : inBox B x
      · rfl
      · exact absurd ⟨B, hB, hx⟩ hc
    obtain ⟨p, hp, _⟩ := subAll_cover A x ha Bs this
    rw [h] at hp; exact absurd hp List.not_mem_nil

def meet : Box → Box → Box
  | [], _ => []
  | _ :: _, [] => []
  | (al, ah) :: as, (bl, bh) :: bs => (max al bl, min ah bh) :: meet as bs

theorem inBox_meet (A : Box) : ∀ (B : Box) (x : List Nat), inBox A x = true → inBox B x = true →
    inBox (meet A B) x = true := by
  induction A with
  | nil => intro B x ha _; exact ha
  | cons a as ih =>
    intro B x ha hb
    obtain ⟨al, ah⟩ := a
    cases x with
    | nil => rw [inBox_cons_nil] at ha; exact absurd ha Bool.false_ne_true
    | cons v vs =>
      cases B with
      | nil => rw [inBox_nil_cons] at hb; exact absurd hb Bool.false_ne_true
      | cons b bs =>
        obtain ⟨bl, bh⟩ := b
        simp only [inBox_cons, Bool.and_eq_true, decide_eq_true_eq] at ha hb
        simp only [meet, inBox_cons, Bool.and_eq_true, decide_eq_true_eq]
        exact ⟨⟨Nat.max_le.mpr ⟨ha.1.1, hb.1.1⟩, Nat.le_min.mpr ⟨ha.1.2, hb.1.2⟩⟩, ih bs vs ha.2 hb.2⟩

/-! ## The box of a list of conditions -/

def setLo : Box → Nat → Nat → Box
  | [], _, _ => []
  | (l, h) :: bs, 0, lo => (max l lo, h) :: bs
  | b :: bs, f + 1, lo => b :: setLo bs f lo

def setHi : Box → Nat → Nat → Box
  | [], _, _ => []
  | (l, h) :: bs, 0, hi => (l, min h hi) :: bs
  | b :: bs, f + 1, hi => b :: setHi bs f hi

theorem inBox_setLo (B : Box) : ∀ (f lo : Nat) (x : List Nat), f < B.length →
    (inBox (setLo B f lo) x = true ↔ inBox B x = true ∧ lo ≤ get x f) := by
  induction B with
  | nil => intro _ _ _ hf; exact absurd hf (Nat.not_lt_zero _)
  | cons b bs ih =>
    intro f lo x hf
    obtain ⟨l, h⟩ := b
    cases x with
    | nil => cases f <;> simp [setLo, inBox]
    | cons v vs =>
      cases f with
      | zero =>
        simp only [setLo, inBox_cons, get, Bool.and_eq_true, decide_eq_true_eq]
        constructor
        · intro ⟨⟨a, b⟩, c⟩; exact ⟨⟨⟨by omega, b⟩, c⟩, by omega⟩
        · intro ⟨⟨⟨a, b⟩, c⟩, d⟩; exact ⟨⟨by omega, b⟩, c⟩
      | succ f =>
        simp only [setLo, inBox_cons, get, Bool.and_eq_true, decide_eq_true_eq]
        have ihf := ih f lo vs (by simp at hf; omega)
        constructor
        · intro ⟨a, c⟩; exact ⟨⟨a, (ihf.mp c).1⟩, (ihf.mp c).2⟩
        · intro ⟨⟨a, c⟩, d⟩; exact ⟨a, ihf.mpr ⟨c, d⟩⟩

theorem inBox_setHi (B : Box) : ∀ (f hi : Nat) (x : List Nat), f < B.length →
    (inBox (setHi B f hi) x = true ↔ inBox B x = true ∧ get x f ≤ hi) := by
  induction B with
  | nil => intro _ _ _ hf; exact absurd hf (Nat.not_lt_zero _)
  | cons b bs ih =>
    intro f hi x hf
    obtain ⟨l, h⟩ := b
    cases x with
    | nil => cases f <;> simp [setHi, inBox]
    | cons v vs =>
      cases f with
      | zero =>
        simp only [setHi, inBox_cons, get, Bool.and_eq_true, decide_eq_true_eq]
        constructor
        · intro ⟨⟨a, b⟩, c⟩; exact ⟨⟨⟨a, by omega⟩, c⟩, by omega⟩
        · intro ⟨⟨⟨a, b⟩, c⟩, d⟩; exact ⟨⟨a, by omega⟩, c⟩
      | succ f =>
        simp only [setHi, inBox_cons, get, Bool.and_eq_true, decide_eq_true_eq]
        have ihf := ih f hi vs (by simp at hf; omega)
        constructor
        · intro ⟨a, c⟩; exact ⟨⟨a, (ihf.mp c).1⟩, (ihf.mp c).2⟩
        · intro ⟨⟨a, c⟩, d⟩; exact ⟨a, ihf.mpr ⟨c, d⟩⟩

theorem setLo_length (B : Box) : ∀ (f lo : Nat), (setLo B f lo).length = B.length := by
  induction B with
  | nil => intro _ _; rfl
  | cons b bs ih =>
    intro f lo
    obtain ⟨l, h⟩ := b
    cases f with
    | zero => rfl
    | succ f => simp [setLo, ih f lo]

theorem setHi_length (B : Box) : ∀ (f hi : Nat), (setHi B f hi).length = B.length := by
  induction B with
  | nil => intro _ _; rfl
  | cons b bs ih =>
    intro f hi
    obtain ⟨l, h⟩ := b
    cases f with
    | zero => rfl
    | succ f => simp [setHi, ih f hi]

/-- Narrow a box by one condition. `!=` (op 5 and above) does not narrow. -/
def condBox (B : Box) (c : Cond) : Box :=
  if c.op = 0 then (if c.value = 0 then setHi (setLo B c.field 1) c.field 0 else setHi B c.field (c.value - 1))
  else if c.op = 1 then setHi B c.field c.value
  else if c.op = 2 then setLo B c.field (c.value + 1)
  else if c.op = 3 then setLo B c.field c.value
  else if c.op = 4 then setHi (setLo B c.field c.value) c.field c.value
  else B

theorem condBox_length (B : Box) (c : Cond) : (condBox B c).length = B.length := by
  unfold condBox; repeat' split
  all_goals simp [setLo_length, setHi_length]

/-- A point in the box that meets the condition stays in the narrowed box. -/
theorem condBox_hull (B : Box) (c : Cond) (x : List Nat) (hf : c.field < B.length)
    (hx : inBox B x = true) (hc : condHolds c x = true) : inBox (condBox B c) x = true := by
  simp only [condHolds, opHolds] at hc
  unfold condBox
  have hl := hf; have hl2 : c.field < (setLo B c.field 1).length := by rw [setLo_length]; exact hf
  have hl3 : c.field < (setLo B c.field c.value).length := by rw [setLo_length]; exact hf
  by_cases h0 : c.op = 0
  · simp only [h0, if_true] at hc ⊢
    split at hc
    · rename_i hv
      by_cases hz : c.value = 0
      · omega
      · rw [if_neg hz]; exact (inBox_setHi B _ _ x hf).mpr ⟨hx, by omega⟩
    · simp at hc
  · simp only [h0, if_false] at hc ⊢
    by_cases h1 : c.op = 1
    · simp only [h1, if_true] at hc ⊢
      split at hc
      · exact (inBox_setHi B _ _ x hf).mpr ⟨hx, by omega⟩
      · simp at hc
    · simp only [h1, if_false] at hc ⊢
      by_cases h2 : c.op = 2
      · simp only [h2, if_true] at hc ⊢
        split at hc
        · exact (inBox_setLo B _ _ x hf).mpr ⟨hx, by omega⟩
        · simp at hc
      · simp only [h2, if_false] at hc ⊢
        by_cases h3 : c.op = 3
        · simp only [h3, if_true] at hc ⊢
          split at hc
          · exact (inBox_setLo B _ _ x hf).mpr ⟨hx, by omega⟩
          · simp at hc
        · simp only [h3, if_false] at hc ⊢
          by_cases h4 : c.op = 4
          · simp only [h4, if_true] at hc ⊢
            split at hc
            · exact (inBox_setHi _ _ _ x hl3).mpr ⟨(inBox_setLo B _ _ x hf).mpr ⟨hx, by omega⟩, by omega⟩
            · simp at hc
          · simp only [h4, if_false]; exact hx

/-- For `<`, `<=`, `>`, `>=` and `=`, a point in the narrowed box meets the condition. -/
theorem condBox_exact (B : Box) (c : Cond) (x : List Nat) (hf : c.field < B.length) (hop : c.op < 5)
    (hx : inBox (condBox B c) x = true) : inBox B x = true ∧ condHolds c x = true := by
  simp only [condHolds, opHolds]
  unfold condBox at hx
  have hl2 : c.field < (setLo B c.field 1).length := by rw [setLo_length]; exact hf
  have hl3 : c.field < (setLo B c.field c.value).length := by rw [setLo_length]; exact hf
  by_cases h0 : c.op = 0
  · simp only [h0, if_true] at hx ⊢
    by_cases hz : c.value = 0
    · rw [if_pos hz] at hx
      have := (inBox_setHi _ _ _ x hl2).mp hx
      have := (inBox_setLo B _ _ x hf).mp this.1
      omega
    · rw [if_neg hz] at hx
      have := (inBox_setHi B _ _ x hf).mp hx
      exact ⟨this.1, by rw [if_pos (by omega)]⟩
  · simp only [h0, if_false] at hx ⊢
    by_cases h1 : c.op = 1
    · simp only [h1, if_true] at hx ⊢
      have := (inBox_setHi B _ _ x hf).mp hx
      exact ⟨this.1, by rw [if_pos this.2]⟩
    · simp only [h1, if_false] at hx ⊢
      by_cases h2 : c.op = 2
      · simp only [h2, if_true] at hx ⊢
        have := (inBox_setLo B _ _ x hf).mp hx
        exact ⟨this.1, by rw [if_pos (by omega)]⟩
      · simp only [h2, if_false] at hx ⊢
        by_cases h3 : c.op = 3
        · simp only [h3, if_true] at hx ⊢
          have := (inBox_setLo B _ _ x hf).mp hx
          exact ⟨this.1, by rw [if_pos this.2]⟩
        · simp only [h3, if_false] at hx ⊢
          have h4 : c.op = 4 := by omega
          simp only [h4, if_true] at hx ⊢
          have a := (inBox_setHi _ _ _ x hl3).mp hx
          have b := (inBox_setLo B _ _ x hf).mp a.1
          exact ⟨b.1, by rw [if_pos (by omega)]⟩

def boxOf (B : Box) : List Cond → Box
  | [] => B
  | c :: cs => boxOf (condBox B c) cs

def condsOk (n : Nat) (strict : Bool) : List Cond → Bool
  | [] => true
  | c :: cs => if c.field < n then (if strict then (if c.op < 5 then condsOk n strict cs else false) else condsOk n strict cs) else false

theorem condsOk_weaken (n : Nat) (cs : List Cond) : condsOk n true cs = true → condsOk n false cs = true := by
  induction cs with
  | nil => intro _; rfl
  | cons c cs ih =>
    intro h
    by_cases hf : c.field < n
    · by_cases ho : c.op < 5
      · simp [condsOk, hf, ho] at h ⊢; exact ih h
      · simp [condsOk, hf, ho] at h
    · simp [condsOk, hf] at h

theorem boxOf_hull (B : Box) (cs : List Cond) (x : List Nat) : condsOk B.length false cs = true →
    inBox B x = true → allHold cs x = true → inBox (boxOf B cs) x = true := by
  induction cs generalizing B with
  | nil => intro _ hx _; exact hx
  | cons c cs ih =>
    intro hok hx hall
    simp only [condsOk] at hok
    split at hok
    · rename_i hf
      simp only [allHold] at hall
      split at hall
      · rename_i hc
        exact ih (condBox B c) (by rw [condBox_length]; exact hok) (condBox_hull B c x hf hx hc) hall
      · simp at hall
    · simp at hok

theorem boxOf_exact (B : Box) (cs : List Cond) (x : List Nat) : condsOk B.length true cs = true →
    inBox (boxOf B cs) x = true → inBox B x = true ∧ allHold cs x = true := by
  induction cs generalizing B with
  | nil => intro _ hx; exact ⟨hx, rfl⟩
  | cons c cs ih =>
    intro hok hx
    simp only [condsOk] at hok
    split at hok
    · rename_i hf
      rw [if_true] at hok
      split at hok
      · rename_i hop
        have ihB := ih (condBox B c) (by rw [condBox_length]; exact hok) hx
        have := condBox_exact B c x hf hop ihB.1
        simp only [allHold, this.2, if_true]
        exact ⟨this.1, ihB.2⟩
      · simp at hok
    · simp at hok

/-! ## The obligations -/

/-- Outcome `o` would break property `p`. -/
def bad (p : Property) (o : Nat) : Bool :=
  if p.always then (if o = p.outcome then false else true) else (if o = p.outcome then true else false)

theorem breaksWith_eq (p : Property) (x : List Nat) (o : Nat) :
    breaksWith p x o = (allHold p.when x && bad p o) := by
  simp only [breaksWith, bad]; split <;> simp_all

/-- Rule `r`, after the rules `pre`, never first-matches a request that `p` speaks about with an
answer that breaks `p`. -/
def oblRule (B : Box) (pre : List Rule) (r : Rule) (p : Property) : Bool :=
  if bad p r.outcome then (subAll (meet (boxOf B r.when) (boxOf B p.when)) (pre.map (fun q => boxOf B q.when))).isEmpty
  else true

def oblRules (B : Box) (p : Property) : List Rule → List Rule → Bool
  | _, [] => true
  | pre, r :: rs => oblRule B pre r p && oblRules B p (pre ++ [r]) rs

/-- `otherwise` never answers, against `p`, a request no rule matches. -/
def oblOther (B : Box) (rules : List Rule) (d : Nat) (p : Property) : Bool :=
  if bad p d then (subAll (meet B (boxOf B p.when)) (rules.map (fun q => boxOf B q.when))).isEmpty else true

def regionCheck (B : Box) (rules : List Rule) (props : List Property) (d : Nat) : Bool :=
  rules.all (fun r => condsOk B.length true r.when) &&
  props.all (fun p => condsOk B.length false p.when && oblRules B p [] rules && oblOther B rules d p)

/-! ## Soundness -/

theorem judge_cases (d : Nat) (x : List Nat) (rules : List Rule) :
    ((∀ r ∈ rules, allHold r.when x = false) ∧ judge rules d x = d) ∨
    ∃ pre r suf, rules = pre ++ r :: suf ∧ (∀ q ∈ pre, allHold q.when x = false) ∧
      allHold r.when x = true ∧ judge rules d x = r.outcome := by
  induction rules with
  | nil => exact Or.inl ⟨fun _ h => absurd h List.not_mem_nil, rfl⟩
  | cons r rs ih =>
    cases hr : allHold r.when x
    · rcases ih with ⟨hn, hj⟩ | ⟨pre, r', suf, he, hp, hm, hj⟩
      · left; refine ⟨fun q hq => ?_, ?_⟩
        · cases List.mem_cons.mp hq with
          | inl h => subst h; exact hr
          | inr h => exact hn q h
        · simp [judge, hr, hj]
      · right; refine ⟨r :: pre, r', suf, by simp [he], fun q hq => ?_, hm, by simp [judge, hr, hj]⟩
        cases List.mem_cons.mp hq with
        | inl h => subst h; exact hr
        | inr h => exact hp q h
    · right; exact ⟨[], r, rs, rfl, fun _ h => absurd h List.not_mem_nil, hr, by simp [judge, hr]⟩

theorem oblRules_at (B : Box) (p : Property) (acc pre : List Rule) (r : Rule) (suf : List Rule) :
    oblRules B p acc (pre ++ r :: suf) = true → oblRule B (acc ++ pre) r p = true := by
  induction pre generalizing acc with
  | nil => intro h; simp only [List.nil_append, oblRules, Bool.and_eq_true] at h; simpa using h.1
  | cons q pre ih =>
    intro h
    simp only [List.cons_append, oblRules, Bool.and_eq_true] at h
    have := ih (acc ++ [q]) h.2
    simpa [List.append_assoc] using this

/-- Passing every obligation means every property holds on every request in the bounds. -/
theorem regionCheck_sound (B : Box) (rules : List Rule) (props : List Property) (d : Nat)
    (hc : regionCheck B rules props d = true) (p : Property) (hp : p ∈ props)
    (x : List Nat) (hx : inBox B x = true) : breaks rules d p x = false := by
  simp only [regionCheck, Bool.and_eq_true, List.all_eq_true] at hc
  obtain ⟨hrules, hprops⟩ := hc
  obtain ⟨⟨hpok, hobl⟩, hother⟩ := hprops p hp
  simp only [breaks, breaksWith_eq]
  cases hw : allHold p.when x
  · rfl
  · have hW := boxOf_hull B p.when x hpok hx hw
    rcases judge_cases d x rules with ⟨hn, hj⟩ | ⟨pre, r, suf, he, hpre, hm, hj⟩
    · rw [hj]
      cases hb : bad p d
      · rfl
      · simp only [oblOther, hb, if_true, List.isEmpty_iff] at hother
        obtain ⟨Q, hQ, hin⟩ := covered_of_subAll _ _ hother x (inBox_meet _ _ x hx hW)
        obtain ⟨q, hq, rfl⟩ := List.mem_map.mp hQ
        have := (boxOf_exact B q.when x (hrules q hq) hin).2
        rw [hn q hq] at this; exact absurd this (by simp)
    · rw [hj]
      cases hb : bad p r.outcome
      · rfl
      · have hr := oblRules_at B p [] pre r suf (he ▸ hobl)
        simp only [List.nil_append, oblRule, hb, if_true, List.isEmpty_iff] at hr
        have hR := boxOf_hull B r.when x (condsOk_weaken _ _ (hrules r (by rw [he]; simp))) hx hm
        obtain ⟨Q, hQ, hin⟩ := covered_of_subAll _ _ hr x (inBox_meet _ _ x hR hW)
        obtain ⟨q, hq, rfl⟩ := List.mem_map.mp hQ
        have hqr : q ∈ rules := by rw [he]; simp [hq]
        have := (boxOf_exact B q.when x (hrules q hqr) hin).2
        rw [hpre q hq] at this; exact absurd this (by simp)


/-! ## Bounds: the property's own comparisons confine every field to the box -/

/-- `c` bounds field `i` above by `h`. -/
def upper (i h : Nat) (c : Cond) : Bool :=
  decide (c.field = i) &&
    ((decide (c.op = 0) && decide (c.value ≤ h + 1)) ||
     (decide (c.op = 1) && decide (c.value ≤ h)) ||
     (decide (c.op = 4) && decide (c.value ≤ h)))

/-- Every interval of the box starts at 0 and some comparison of `ws` bounds its field. -/
def hull (ws : List Cond) : Nat → Box → Bool
  | _, [] => true
  | i, (l, h) :: bs => decide (l = 0) && ws.any (upper i h) && hull ws (i + 1) bs

theorem allHold_mem (cs : List Cond) (x : List Nat) : allHold cs x = true →
    ∀ c ∈ cs, condHolds c x = true := by
  induction cs with
  | nil => intro _ _ hc; exact absurd hc List.not_mem_nil
  | cons c cs ih =>
    intro h c' hc
    simp only [allHold] at h
    split at h
    · rename_i hh
      cases List.mem_cons.mp hc with
      | inl e => subst e; exact hh
      | inr m => exact ih h c' m
    · simp at h

theorem upper_bound (i h : Nat) (c : Cond) (x : List Nat) (hu : upper i h c = true)
    (hc : condHolds c x = true) : get x i ≤ h := by
  simp only [upper, Bool.and_eq_true, Bool.or_eq_true, decide_eq_true_eq] at hu
  obtain ⟨hf, hu⟩ := hu
  simp only [condHolds, opHolds, hf] at hc
  rcases hu with (⟨h0, hv⟩ | ⟨h1, hv⟩) | ⟨h4, hv⟩
  · simp only [h0, if_true] at hc; split at hc <;> simp_all; omega
  · simp only [h1] at hc; split at hc <;> simp_all; omega
  · simp only [h4] at hc; split at hc <;> simp_all

theorem inBox_of_hull (ws : List Cond) (y : List Nat) (hy : allHold ws y = true) (B : Box) :
    ∀ (i : Nat) (x : List Nat), hull ws i B = true →
      (∀ j, get x j = get y (i + j)) → x.length = B.length → inBox B x = true := by
  induction B with
  | nil =>
    intro i x _ _ hl
    cases x with
    | nil => rfl
    | cons _ _ => exact nomatch hl
  | cons b bs ih =>
    intro i x hh hx hl
    obtain ⟨l, h⟩ := b
    cases x with
    | nil => exact nomatch hl
    | cons v vs =>
      simp only [hull, Bool.and_eq_true, decide_eq_true_eq, List.any_eq_true] at hh
      obtain ⟨⟨hl0, c, hc, hu⟩, hrest⟩ := hh
      have hv : get (v :: vs) 0 = get y (i + 0) := hx 0
      have hb := upper_bound i h c y hu (allHold_mem ws y hy c hc)
      simp only [get, Nat.add_zero] at hv
      simp only [inBox_cons, Bool.and_eq_true, decide_eq_true_eq]
      refine ⟨⟨hl0 ▸ Nat.zero_le v, hv ▸ hb⟩, ?_⟩
      exact ih (i + 1) vs hrest
        (fun j => by
          have := hx (j + 1)
          show get vs j = get y (i + 1 + j)
          rw [Nat.add_assoc, Nat.add_comm 1 j]; exact this)
        (Nat.succ.inj hl)

end Spec

/-! ## The bridge from the backend's shapes to `Spec` -/

/-- An operator code in `Spec`'s six. -/
def opSpec : Nat → Nat
  | 0 => 0 | 1 => 1 | 2 => 2 | 3 => 3 | 4 => 4 | 5 => 4 | _ + 6 => 5

def condSpec (c : Cond) : Cond := ⟨c.field, opSpec c.op, c.value⟩
def ruleSpec (r : Rule) : Rule := ⟨r.when.map condSpec, r.outcome⟩
def propSpec (p : Property) : Property := ⟨p.always, p.when.map condSpec, p.outcome⟩

theorem blt_decide (a b : Nat) : Nat.blt a b = decide (a < b) :=
  Bool.eq_iff_iff.mpr (by simp [Nat.blt_eq])

theorem ble_decide (a b : Nat) : Nat.ble a b = decide (a ≤ b) :=
  Bool.eq_iff_iff.mpr (by simp [Nat.ble_eq])

theorem beq_decide (a b : Nat) : Nat.beq a b = decide (a = b) :=
  Bool.eq_iff_iff.mpr (by simp [Nat.beq_eq])

theorem opHolds_spec (op c x : Nat) : opHolds op c x = Spec.opHolds (opSpec op) c x := by
  rcases op with _ | _ | _ | _ | _ | _ | _ | op
  all_goals
    simp only [opHolds, opSpec, Spec.opHolds, blt_decide, ble_decide, beq_decide]
    (repeat' split) <;> simp_all <;> omega

theorem condHolds_spec (c : Cond) (x : List Nat) : condHolds c x = Spec.condHolds (condSpec c) x := by
  simp only [condHolds, Spec.condHolds, condSpec, opHolds_spec]

theorem allHoldFrom_spec (x : List Nat) (cs : List Cond) : ∀ (acc : Bool),
    allHoldFrom x acc cs = (acc && Spec.allHold (cs.map condSpec) x) := by
  induction cs with
  | nil => intro acc; simp [allHoldFrom, Spec.allHold]
  | cons c cs ih =>
    intro acc
    simp only [allHoldFrom, ih, List.map_cons, Spec.allHold, condHolds_spec]
    cases Spec.condHolds (condSpec c) x <;> cases acc <;> simp

theorem allHold_spec (cs : List Cond) (x : List Nat) :
    allHold cs x = Spec.allHold (cs.map condSpec) x := by
  simp [allHold, allHoldFrom_spec]

theorem judge_spec (d : Nat) (x : List Nat) (rules : List Rule) :
    judge rules d x = Spec.judge (rules.map ruleSpec) d x := by
  induction rules with
  | nil => rfl
  | cons r rs ih => simp only [judge, List.map_cons, Spec.judge, ruleSpec, allHold_spec, ih]

theorem breaks_spec (rules : List Rule) (d : Nat) (p : Property) (x : List Nat) :
    breaks rules d p x = Spec.breaks (rules.map ruleSpec) d (propSpec p) x := by
  obtain ⟨al, ws, out⟩ := p
  show (allHold ws x && bad ⟨al, ws, out⟩ (judge rules d x)) =
    Spec.breaksWith ⟨al, ws.map condSpec, out⟩ x (Spec.judge (rules.map ruleSpec) d x)
  rw [allHold_spec, judge_spec]
  generalize Spec.judge (List.map ruleSpec rules) d x = o
  unfold Spec.breaksWith
  dsimp only
  cases Spec.allHold (List.map condSpec ws) x <;> cases al <;> by_cases ho : o = out <;>
    simp [bad, ho, beq_decide]

/-! ## The proof form's obligation and its soundness -/

/-- One property's obligation: the box is the property's own hull, and every local box
obligation of `Spec.regionCheck` passes. -/
def obligation (B : Spec.Box) (rules : List Rule) (d : Nat) (p : Property) : Bool :=
  Spec.hull (propSpec p).when 0 B && Spec.regionCheck B (rules.map ruleSpec) [propSpec p] d

/-- If the obligation passes, `p` holds on every request of the box's width. -/
theorem holds (rules : List Rule) (d : Nat) (p : Property) (B : Spec.Box)
    (h : obligation B rules d p = true) (x : List Nat) (hl : x.length = B.length) :
    breaks rules d p x = false := by
  simp only [obligation, Bool.and_eq_true] at h
  rw [breaks_spec]
  cases hw : Spec.allHold (propSpec p).when x
  · simp [Spec.breaks, Spec.breaksWith, hw]
  · have hx := Spec.inBox_of_hull _ x hw B 0 x h.1 (fun j => by simp) hl
    exact Spec.regionCheck_sound B _ _ d h.2 (propSpec p) (List.mem_singleton_self _) x hx

end LexLeanRegions
end
