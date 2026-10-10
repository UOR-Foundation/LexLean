import LexLeanPreservation.Templates
/-!
Boundary validators (SPEC.md §17.17): the calculus functions a root's entry
calls to decide, at run time, the invariants §17.12 states of the values it
receives — a map's keys and a set's elements strictly ascending in the key
order, recursively through containers — and the theorems linking each
validator to its proposition.
-/
set_option autoImplicit false
namespace LexLeanPreservation
open LexLeanTarget.TargetSyntax LexLeanTarget.TargetSemantics

/-! The propositions and their decisions. -/

/-- Whether a key order puts `a` strictly before `b`. -/
def lessB {κ : Type} (cmp : κ → κ → Ordering) (a b : κ) : Bool :=
  match cmp a b with
  | .lt => true
  | .eq => false
  | .gt => false

/-- §17.12: a list strictly ascending in a key order. -/
def StrictAsc {κ : Type} (cmp : κ → κ → Ordering) : List κ → Prop
  | [] => True
  | [_] => True
  | a :: b :: r => cmp a b = .lt ∧ StrictAsc cmp (b :: r)

/-- Every member of a list satisfies `valid`. -/
def validList {α : Type} (valid : α → Bool) : List α → Bool
  | [] => true
  | x :: xs => if valid x then validList valid xs else false

/-- A set: its elements strictly ascending. -/
def ascSet {κ : Type} (cmp : κ → κ → Ordering) : List κ → Bool
  | [] => true
  | [_] => true
  | a :: b :: r => if lessB cmp a b then ascSet cmp (b :: r) else false

/-- A map: its keys strictly ascending, each value valid. -/
def ascMap {κ ν : Type} (cmp : κ → κ → Ordering) (valid : ν → Bool) : List (κ × ν) → Bool
  | [] => true
  | [e] => if valid e.2 then true else false
  | e :: e2 :: r => if valid e.2 then (if lessB cmp e.1 e2.1 then ascMap cmp valid (e2 :: r) else false)
      else false

def validOption {α : Type} (valid : α → Bool) : Option α → Bool
  | none => true
  | some a => valid a

def validPair {α β : Type} (va : α → Bool) (vb : β → Bool) (x : α × β) : Bool :=
  if va x.1 then vb x.2 else false

def validExcept {ε α : Type} (ve : ε → Bool) (va : α → Bool) : Except ε α → Bool
  | .ok a => va a
  | .error e => ve e

theorem lessB_iff {κ : Type} (cmp : κ → κ → Ordering) (a b : κ) : lessB cmp a b = true ↔ cmp a b = .lt := by
  unfold lessB; cases cmp a b <;> simp

theorem validList_iff {α : Type} (valid : α → Bool) :
    ∀ xs : List α, validList valid xs = true ↔ ∀ x ∈ xs, valid x = true
  | [] => by simp [validList]
  | x :: xs => by
    cases h : valid x <;> simp [validList, h, validList_iff valid xs]

/-- The validator of a set decides exactly that its elements ascend. -/
theorem ascSet_iff {κ : Type} (cmp : κ → κ → Ordering) :
    ∀ xs : List κ, ascSet cmp xs = true ↔ StrictAsc cmp xs
  | [] => by simp [ascSet, StrictAsc]
  | [_] => by simp [ascSet, StrictAsc]
  | a :: b :: r => by
    have ih := ascSet_iff cmp (b :: r)
    cases h : lessB cmp a b
    · have : cmp a b ≠ .lt := fun e => by rw [(lessB_iff cmp a b).mpr e] at h; cases h
      simp [ascSet, StrictAsc, h, this]
    · simp [ascSet, StrictAsc, h, ih, (lessB_iff cmp a b).mp h]

/-- The validator of a map decides exactly that its keys ascend and every
value is valid. -/
theorem ascMap_iff {κ ν : Type} (cmp : κ → κ → Ordering) (valid : ν → Bool) :
    ∀ m : List (κ × ν), ascMap cmp valid m = true ↔
      StrictAsc cmp (m.map Prod.fst) ∧ ∀ e ∈ m, valid e.2 = true
  | [] => by simp [ascMap, StrictAsc]
  | [e] => by cases h : valid e.2 <;> simp [ascMap, StrictAsc, h]
  | e :: e2 :: r => by
    have ih := ascMap_iff cmp valid (e2 :: r)
    cases hv : valid e.2
    · simp [ascMap, hv]
    · cases h : lessB cmp e.1 e2.1
      · have : cmp e.1 e2.1 ≠ .lt := fun x => by rw [(lessB_iff cmp _ _).mpr x] at h; cases h
        simp [ascMap, StrictAsc, hv, h, this]
      · simp only [ascMap, hv, h, if_true, ih]
        simp [StrictAsc, (lessB_iff cmp _ _).mp h, hv]

/-- A root's observation as its entry returns it when every invariant
holds: a value as `some`; an overflow stays one. -/
def someObs : Obs → Obs
  | .value v => .value (.some v)
  | .overflow => .overflow
  | .stuck => .stuck

/-! §17.12's propositions, compositionally: a validated type's invariant is
built from these, and a document type's from its fields'. -/

/-- Every member of a list satisfies `inv`. -/
def InvList {α : Type} (inv : α → Prop) (xs : List α) : Prop := ∀ x ∈ xs, inv x

/-- A set: its elements strictly ascending in the key order. -/
def InvSet {κ : Type} (cmp : κ → κ → Ordering) (xs : List κ) : Prop := StrictAsc cmp xs

/-- A map: its keys strictly ascending in the key order, each value
satisfying `inv`. -/
def InvMap {κ ν : Type} (cmp : κ → κ → Ordering) (inv : ν → Prop) (m : List (κ × ν)) : Prop :=
  StrictAsc cmp (m.map Prod.fst) ∧ ∀ e ∈ m, inv e.2

def InvOption {α : Type} (inv : α → Prop) : Option α → Prop
  | none => True
  | some a => inv a

def InvPair {α β : Type} (ia : α → Prop) (ib : β → Prop) (x : α × β) : Prop := ia x.1 ∧ ib x.2

def InvExcept {ε α : Type} (ie : ε → Prop) (ia : α → Prop) : Except ε α → Prop
  | .ok a => ia a
  | .error e => ie e

theorem trueIff : true = true ↔ True := ⟨fun _ => trivial, fun _ => rfl⟩

/-- A conditional conjunction, as a validator's chain of field checks
computes it, decides the conjunction of the fields' propositions. -/
theorem condAnd_iff {c b : Bool} {P Q : Prop} (hc : c = true ↔ P) (hb : b = true ↔ Q) :
    (if c then b else false) = true ↔ P ∧ Q := by
  cases c <;> cases b <;> simp_all

/-- A list invariant defined by structural recursion, as a recursive
group's validator states it, is §17.12's proposition of the list. -/
theorem invList_eqs {α : Type} (inv : α → Prop) (I : List α → Prop) (h0 : I [] ↔ True)
    (h1 : ∀ x xs, I (x :: xs) ↔ inv x ∧ I xs) : ∀ xs, I xs ↔ InvList inv xs
  | [] => by rw [h0]; simp [InvList]
  | x :: xs => by
    rw [h1, invList_eqs inv I h0 h1 xs]; simp [InvList]

theorem validTrue_inv {α : Type} (x : α) : (fun _ : α => true) x = true ↔ (fun _ : α => True) x :=
  trueIff

theorem validList_inv {α : Type} (valid : α → Bool) (inv : α → Prop) (h : ∀ x, valid x = true ↔ inv x)
    (xs : List α) : validList valid xs = true ↔ InvList inv xs := by
  rw [validList_iff]; unfold InvList
  exact forall_congr' fun x => imp_congr_right fun _ => h x

theorem ascSet_inv {κ : Type} (cmp : κ → κ → Ordering) (xs : List κ) :
    ascSet cmp xs = true ↔ InvSet cmp xs := ascSet_iff cmp xs

theorem ascMap_inv {κ ν : Type} (cmp : κ → κ → Ordering) (valid : ν → Bool) (inv : ν → Prop)
    (h : ∀ w, valid w = true ↔ inv w) (m : List (κ × ν)) :
    ascMap cmp valid m = true ↔ InvMap cmp inv m := by
  rw [ascMap_iff]; unfold InvMap
  exact and_congr Iff.rfl (forall_congr' fun e => imp_congr_right fun _ => h e.2)

theorem validOption_inv {α : Type} (valid : α → Bool) (inv : α → Prop) (h : ∀ x, valid x = true ↔ inv x) :
    ∀ o, validOption valid o = true ↔ InvOption inv o
  | none => by simp [validOption, InvOption]
  | some a => h a

theorem validPair_inv {α β : Type} (va : α → Bool) (vb : β → Bool) (ia : α → Prop) (ib : β → Prop)
    (ha : ∀ x, va x = true ↔ ia x) (hb : ∀ x, vb x = true ↔ ib x) (x : α × β) :
    validPair va vb x = true ↔ InvPair ia ib x :=
  condAnd_iff (ha x.1) (hb x.2)

theorem validExcept_inv {ε α : Type} (ve : ε → Bool) (va : α → Bool) (ie : ε → Prop) (ia : α → Prop)
    (he : ∀ x, ve x = true ↔ ie x) (ha : ∀ x, va x = true ↔ ia x) :
    ∀ x, validExcept ve va x = true ↔ InvExcept ie ia x
  | .ok a => ha a
  | .error e => he e

namespace Tpl

def validTrueFn (t : Ty) : Function := fn [t] .bool (boolE true)

def validListFn (this f : Nat) (t : Ty) : Function :=
  fn [lst t] .bool (.match .bool (v 0) [.arm .nil [] (boolE true),
    .arm .cons [1, 2] (.cond (.call f [v 1]) (.call this [v 2]) (boolE false))])

def validSetFn (this : Nat) (k : Ty) : Function :=
  fn [lst k] .bool (.match .bool (v 0) [.arm .nil [] (boolE true),
    .arm .cons [1, 2] (.match .bool (v 2) [.arm .nil [] (boolE true),
      .arm .cons [3, 4] (.cond (lessThanE (v 1) (v 3)) (.call this [v 2]) (boolE false))])])

def validMapFn (this f : Nat) (k w : Ty) : Function :=
  fn [lst (pr k w)] .bool (.match .bool (v 0) [.arm .nil [] (boolE true),
    .arm .cons [1, 2] (.cond (.call f [.second (v 1)])
      (.match .bool (v 2) [.arm .nil [] (boolE true),
        .arm .cons [3, 4] (.cond (lessThanE (.first (v 1)) (.first (v 3))) (.call this [v 2])
          (boolE false))])
      (boolE false))])

def validOptionFn (f : Nat) (t : Ty) : Function :=
  fn [.option t] .bool (.match .bool (v 0) [.arm .none [] (boolE true), .arm .some [1] (.call f [v 1])])

def validPairFn (fa fb : Nat) (a b : Ty) : Function :=
  fn [pr a b] .bool (.cond (.call fa [.first (v 0)]) (.call fb [.second (v 0)]) (boolE false))

def validResultFn (fa fb : Nat) (a b : Ty) : Function :=
  fn [.result a b] .bool (.match .bool (v 0)
    [.arm .ok [1] (.call fa [v 1]), .arm .error [2] (.call fb [v 2])])

end Tpl

open Tpl

theorem tpl_validTrue {α : Type} (e : α → Value) {p : Program} {fi : Nat} {t : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (validTrueFn t)) :
    ∀ x, FunRel p fi [e x] (Rel true (.bool ((fun _ => true) x))) :=
  fun _ => funRel_intro hf rfl (conv_build convL_nil construct_true)

theorem tpl_validList {α : Type} {ea : α → Value} (valid : α → Bool)
    (L : ListEnc α) (hL : ∀ x, L.elem x = ea x) {p : Program} {fv fi : Nat} {t : Ty}
    (hv : ∀ x, FunRel p fv [ea x] (Rel true (.bool (valid x))))
    (hf : LexLeanRuntime.index p.functions fi = some (validListFn fi fv t)) :
    ∀ xs, FunRel p fi [L.enc xs] (Rel true (.bool (validList valid xs)))
  | [] => by
    rw [L.enc_nil]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl
      (conv_build convL_nil construct_true)))
  | x :: xs => by
    rw [enc_elem L hL]
    refine FunRel.fits_eq (funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
      (conv_cond (fun _ => true) (fun b => .bool (if b then validList valid xs else false)) (valid x)
        (conv_call (convL_cons (conv_var rfl) convL_nil) (hv x))
        (fun _ => conv_call (convL_cons (conv_var rfl) convL_nil) (tpl_validList valid L hL hv hf xs))
        (fun _ => conv_build convL_nil construct_false)))))) ?_
    simp
  termination_by xs => xs.length

theorem lessB_lt {κ : Type} {cmp : κ → κ → Ordering} : ∀ x y, cmp x y = .lt → lessB cmp x y = true :=
  fun x y h => by simp [lessB, h]
theorem lessB_eq {κ : Type} {cmp : κ → κ → Ordering} : ∀ x y, cmp x y = .eq → lessB cmp x y = false :=
  fun x y h => by simp [lessB, h]
theorem lessB_gt {κ : Type} {cmp : κ → κ → Ordering} : ∀ x y, cmp x y = .gt → lessB cmp x y = false :=
  fun x y h => by simp [lessB, h]

theorem tpl_validSet {κ : Type} {ek : κ → Value} {cmp : κ → κ → Ordering} (K : KeySpec ek cmp)
    (L : ListEnc κ) (hL : ∀ x, L.elem x = ek x) {p : Program} {fi : Nat} {t : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (validSetFn fi t)) :
    ∀ xs, FunRel p fi [L.enc xs] (Rel true (.bool (ascSet cmp xs)))
  | [] => by
    rw [L.enc_nil]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl
      (conv_build convL_nil construct_true)))
  | [a] => by
    rw [enc_elem L hL, L.items_nil]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
      (conv_match (conv_var rfl) (convA_hit rfl rfl (conv_build convL_nil construct_true))))))
  | a :: b :: r => by
    have e : L.enc (a :: b :: r) = .list (ek a :: ek b :: L.items r) := by
      rw [enc_elem L hL, L.items_cons, hL]
    rw [e]
    have hrest : L.enc (b :: r) = .list (ek b :: L.items r) := enc_elem L hL b r
    refine FunRel.fits_eq (funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
      (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_cond (fun _ => true) (fun c => .bool (if c then ascSet cmp (b :: r) else false))
          (lessB cmp a b)
          (conv_lessThan K (lessB cmp) lessB_lt lessB_eq lessB_gt
            (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil)))
          (fun _ => conv_call (convL_cons (conv_var rfl) convL_nil)
            (hrest ▸ tpl_validSet K L hL hf (b :: r)))
          (fun _ => conv_build convL_nil construct_false))))))))) ?_
    simp

theorem tpl_validMap {κ ν : Type} {ek : κ → Value} {ev : ν → Value} {cmp : κ → κ → Ordering}
    (K : KeySpec ek cmp) (valid : ν → Bool)
    (L : ListEnc (κ × ν)) (hL : ∀ e, L.elem e = .pair (ek e.1) (ev e.2)) {p : Program} {fv fi : Nat}
    {tk tw : Ty}
    (hv : ∀ w, FunRel p fv [ev w] (Rel true (.bool (valid w))))
    (hf : LexLeanRuntime.index p.functions fi = some (validMapFn fi fv tk tw)) :
    ∀ m, FunRel p fi [L.enc m] (Rel true (.bool (ascMap cmp valid m)))
  | [] => by
    rw [L.enc_nil]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl
      (conv_build convL_nil construct_true)))
  | [(k, w)] => by
    rw [enc_entry L hL, L.items_nil]
    refine FunRel.fits_eq (funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
      (conv_cond (fun _ => true) (fun c => .bool (if c then true else false)) (valid w)
        (conv_call (convL_cons (conv_second (conv_var rfl)) convL_nil) (hv w))
        (fun _ => conv_match (conv_var rfl) (convA_hit rfl rfl (conv_build convL_nil construct_true)))
        (fun _ => conv_build convL_nil construct_false)))))) ?_
    simp
  | (k, w) :: (k2, w2) :: r => by
    have e : L.enc ((k, w) :: (k2, w2) :: r) =
        .list (.pair (ek k) (ev w) :: .pair (ek k2) (ev w2) :: L.items r) := by
      rw [enc_entry L hL, L.items_cons, hL]
    rw [e]
    have hrest : L.enc ((k2, w2) :: r) = .list (.pair (ek k2) (ev w2) :: L.items r) :=
      enc_entry L hL k2 w2 r
    refine FunRel.fits_eq (funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
      (conv_cond (fun _ => true)
        (fun c => .bool (if c then (if lessB cmp k k2 then ascMap cmp valid ((k2, w2) :: r) else false)
          else false)) (valid w)
        (conv_call (convL_cons (conv_second (conv_var rfl)) convL_nil) (hv w))
        (fun _ => conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
          (conv_cond (fun _ => true) (fun c => .bool (if c then ascMap cmp valid ((k2, w2) :: r) else false))
            (lessB cmp k k2)
            (conv_lessThan K (lessB cmp) lessB_lt lessB_eq lessB_gt
              (convL_cons (conv_first (conv_var rfl)) (convL_cons (conv_first (conv_var rfl)) convL_nil)))
            (fun _ => conv_call (convL_cons (conv_var rfl) convL_nil)
              (hrest ▸ tpl_validMap K valid L hL hv hf ((k2, w2) :: r)))
            (fun _ => conv_build convL_nil construct_false)))))
        (fun _ => conv_build convL_nil construct_false)))))) ?_
    simp

theorem tpl_validOption {α : Type} {ea : α → Value} (valid : α → Bool) {p : Program} {fv fi : Nat}
    {t : Ty} (hv : ∀ x, FunRel p fv [ea x] (Rel true (.bool (valid x))))
    (hf : LexLeanRuntime.index p.functions fi = some (validOptionFn fv t)) :
    ∀ o, FunRel p fi [encOption ea o] (Rel true (.bool (validOption valid o)))
  | none => funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl
      (conv_build convL_nil construct_true)))
  | some a => funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
      (conv_call (convL_cons (conv_var rfl) convL_nil) (hv a)))))

theorem tpl_validPair {α β : Type} {ea : α → Value} {eb : β → Value} (va : α → Bool) (vb : β → Bool)
    {p : Program} {fa fb fi : Nat} {ta tb : Ty}
    (ha : ∀ x, FunRel p fa [ea x] (Rel true (.bool (va x))))
    (hb : ∀ x, FunRel p fb [eb x] (Rel true (.bool (vb x))))
    (hf : LexLeanRuntime.index p.functions fi = some (validPairFn fa fb ta tb)) :
    ∀ x, FunRel p fi [encPair ea eb x] (Rel true (.bool (validPair va vb x)))
  | (a, b) => by
    refine FunRel.fits_eq (funRel_intro hf rfl
      (conv_cond (fun _ => true) (fun c => .bool (if c then vb b else false)) (va a)
        (conv_call (convL_cons (conv_first (conv_var rfl)) convL_nil) (ha a))
        (fun _ => conv_call (convL_cons (conv_second (conv_var rfl)) convL_nil) (hb b))
        (fun _ => conv_build convL_nil construct_false))) ?_
    simp

theorem tpl_validResult {ε α : Type} {ee : ε → Value} {ea : α → Value} (ve : ε → Bool) (va : α → Bool)
    {p : Program} {fa fe fi : Nat} {ta te : Ty}
    (ha : ∀ x, FunRel p fa [ea x] (Rel true (.bool (va x))))
    (he : ∀ x, FunRel p fe [ee x] (Rel true (.bool (ve x))))
    (hf : LexLeanRuntime.index p.functions fi = some (validResultFn fa fe ta te)) :
    ∀ x, FunRel p fi [encExcept ee ea x] (Rel true (.bool (validExcept ve va x)))
  | .ok a => funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl
      (conv_call (convL_cons (conv_var rfl) convL_nil) (ha a))))
  | .error e => funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
      (conv_call (convL_cons (conv_var rfl) convL_nil) (he e)))))

end LexLeanPreservation
