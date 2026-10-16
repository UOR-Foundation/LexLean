module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanLaws

open LexLeanCollections

/-! Sums of a function over a list: the normal form of an additive fold. -/

@[expose] public def sumNat {α : Type} (k : α -> Nat) : List α -> Nat
  | [] => 0
  | x :: xs => k x + sumNat k xs

@[expose] public def sumInt {α : Type} (k : α -> Int) : List α -> Int
  | [] => 0
  | x :: xs => k x + sumInt k xs

/-! Bridges from the runtime to the propositions `omega` reads. -/

public theorem lessThan_nat (a b : Nat) : lessThan a b = decide (a < b) := by
  simp only [lessThan, Key.compare]
  cases h : compare a b
  · simp only [Nat.compare_eq_lt.mp h, decide_true]
  · simp only [Nat.compare_eq_eq.mp h, Nat.lt_irrefl, decide_false]
  · simp only [Nat.not_lt.mpr (Nat.le_of_lt (Nat.compare_eq_gt.mp h)), decide_false]

public theorem lessThan_int (a b : Int) : lessThan a b = decide (a < b) := by
  simp only [lessThan, Key.compare]
  cases h : compare a b
  · simp only [Int.compare_eq_lt.mp h, decide_true]
  · simp only [Int.compare_eq_eq.mp h, Int.lt_irrefl, decide_false]
  · simp only [Int.not_lt.mpr (Int.le_of_lt (Int.compare_eq_gt.mp h)), decide_false]

public theorem lessThan_int8 (a b : Int8) : lessThan a b = decide (a.toInt < b.toInt) := by
  simp only [lessThan, Key.compare]
  cases h : compare a.toInt b.toInt
  · simp only [Int.compare_eq_lt.mp h, decide_true]
  · simp only [Int.compare_eq_eq.mp h, Int.lt_irrefl, decide_false]
  · simp only [Int.not_lt.mpr (Int.le_of_lt (Int.compare_eq_gt.mp h)), decide_false]

public theorem lessThan_int16 (a b : Int16) : lessThan a b = decide (a.toInt < b.toInt) := by
  simp only [lessThan, Key.compare]
  cases h : compare a.toInt b.toInt
  · simp only [Int.compare_eq_lt.mp h, decide_true]
  · simp only [Int.compare_eq_eq.mp h, Int.lt_irrefl, decide_false]
  · simp only [Int.not_lt.mpr (Int.le_of_lt (Int.compare_eq_gt.mp h)), decide_false]

public theorem lessThan_int32 (a b : Int32) : lessThan a b = decide (a.toInt < b.toInt) := by
  simp only [lessThan, Key.compare]
  cases h : compare a.toInt b.toInt
  · simp only [Int.compare_eq_lt.mp h, decide_true]
  · simp only [Int.compare_eq_eq.mp h, Int.lt_irrefl, decide_false]
  · simp only [Int.not_lt.mpr (Int.le_of_lt (Int.compare_eq_gt.mp h)), decide_false]

public theorem lessThan_int64 (a b : Int64) : lessThan a b = decide (a.toInt < b.toInt) := by
  simp only [lessThan, Key.compare]
  cases h : compare a.toInt b.toInt
  · simp only [Int.compare_eq_lt.mp h, decide_true]
  · simp only [Int.compare_eq_eq.mp h, Int.lt_irrefl, decide_false]
  · simp only [Int.not_lt.mpr (Int.le_of_lt (Int.compare_eq_gt.mp h)), decide_false]

public theorem lessThan_uint8 (a b : UInt8) : lessThan a b = decide (a.toNat < b.toNat) := by
  simp only [lessThan, Key.compare]
  cases h : compare a.toNat b.toNat
  · simp only [Nat.compare_eq_lt.mp h, decide_true]
  · simp only [Nat.compare_eq_eq.mp h, Nat.lt_irrefl, decide_false]
  · simp only [Nat.not_lt.mpr (Nat.le_of_lt (Nat.compare_eq_gt.mp h)), decide_false]

public theorem lessThan_uint16 (a b : UInt16) : lessThan a b = decide (a.toNat < b.toNat) := by
  simp only [lessThan, Key.compare]
  cases h : compare a.toNat b.toNat
  · simp only [Nat.compare_eq_lt.mp h, decide_true]
  · simp only [Nat.compare_eq_eq.mp h, Nat.lt_irrefl, decide_false]
  · simp only [Nat.not_lt.mpr (Nat.le_of_lt (Nat.compare_eq_gt.mp h)), decide_false]

public theorem lessThan_uint32 (a b : UInt32) : lessThan a b = decide (a.toNat < b.toNat) := by
  simp only [lessThan, Key.compare]
  cases h : compare a.toNat b.toNat
  · simp only [Nat.compare_eq_lt.mp h, decide_true]
  · simp only [Nat.compare_eq_eq.mp h, Nat.lt_irrefl, decide_false]
  · simp only [Nat.not_lt.mpr (Nat.le_of_lt (Nat.compare_eq_gt.mp h)), decide_false]

public theorem lessThan_uint64 (a b : UInt64) : lessThan a b = decide (a.toNat < b.toNat) := by
  simp only [lessThan, Key.compare]
  cases h : compare a.toNat b.toNat
  · simp only [Nat.compare_eq_lt.mp h, decide_true]
  · simp only [Nat.compare_eq_eq.mp h, Nat.lt_irrefl, decide_false]
  · simp only [Nat.not_lt.mpr (Nat.le_of_lt (Nat.compare_eq_gt.mp h)), decide_false]

public theorem quotient_nat (a b z : Nat) :
    LexLeanRuntime.quotient a b z = if b = 0 then z else a / b := by
  simp only [LexLeanRuntime.quotient, LexLeanRuntime.Quotient.isZero, LexLeanRuntime.Quotient.quotient, beq_iff_eq]
  rfl

public theorem quotient_int (a b z : Int) :
    LexLeanRuntime.quotient a b z = if b = 0 then z else a.tdiv b := by
  simp only [LexLeanRuntime.quotient, LexLeanRuntime.Quotient.isZero, LexLeanRuntime.Quotient.quotient, beq_iff_eq]

public theorem remainder_nat (a b z : Nat) :
    LexLeanRuntime.remainder a b z = if b = 0 then z else a % b := by
  simp only [LexLeanRuntime.remainder, LexLeanRuntime.Quotient.isZero, LexLeanRuntime.Quotient.remainder, beq_iff_eq]
  rfl

public theorem remainder_int (a b z : Int) :
    LexLeanRuntime.remainder a b z = if b = 0 then z else a.tmod b := by
  simp only [LexLeanRuntime.remainder, LexLeanRuntime.Quotient.isZero, LexLeanRuntime.Quotient.remainder, beq_iff_eq]

-- Truncating division by a nonzero divisor, as Euclidean division and a sign case.
public theorem tdiv_ediv (a b : Int) :
    a.tdiv b = if 0 ≤ a then (if 0 ≤ b then a / b else -(a / -b)) else
      (if 0 ≤ b then -((-a) / b) else (-a) / -b) := by
  split <;> split
  · exact Int.tdiv_eq_ediv_of_nonneg ‹_›
  · rw [show b = -(-b) by omega, Int.tdiv_neg, Int.tdiv_eq_ediv_of_nonneg ‹_›, Int.neg_neg]
  · rw [show a = -(-a) by omega, Int.neg_tdiv, Int.tdiv_eq_ediv_of_nonneg (by omega), Int.neg_neg]
  · rw [show a = -(-a) by omega, show b = -(-b) by omega, Int.neg_tdiv, Int.tdiv_neg,
      Int.tdiv_eq_ediv_of_nonneg (by omega), Int.neg_neg, Int.neg_neg, Int.neg_neg]

public theorem tmod_emod (a b : Int) :
    a.tmod b = if 0 ≤ a then a % b else -((-a) % b) := by
  split
  · exact Int.tmod_eq_emod_of_nonneg ‹_›
  · rw [show a = -(-a) by omega, Int.neg_tmod, Int.tmod_eq_emod_of_nonneg (by omega), Int.neg_neg]

public theorem length_list {α : Type} (xs : List α) : LexLeanRuntime.length xs = xs.length := rfl

public theorem append_list {α : Type} (xs ys : List α) : LexLeanRuntime.append xs ys = xs ++ ys := rfl

public theorem setFold_listFold {κ σ : Type} (f : σ -> κ -> σ) (z : σ) (s : List κ) :
    setFold f z s = listFold f z s := rfl

public theorem mapFold_listFold {κ ν σ : Type} (f : σ -> κ -> ν -> σ) (z : σ) (m : List (Prod κ ν)) :
    mapFold f z m = listFold (fun a e => f a e.1 e.2) z m := rfl

public theorem setSize_length {κ : Type} (s : List κ) : setSize s = s.length := rfl

public theorem mapSize_length {κ ν : Type} (m : List (Prod κ ν)) : mapSize m = m.length := rfl

public theorem setElements_self {κ : Type} (s : List κ) : setElements s = s := rfl

public theorem mapEntries_self {κ ν : Type} (m : List (Prod κ ν)) : mapEntries m = m := rfl

/-! Folds. -/

public theorem listFold_nil {α σ : Type} (f : σ -> α -> σ) (z : σ) : listFold f z [] = z := rfl

public theorem listFold_cons {α σ : Type} (f : σ -> α -> σ) (z : σ) (x : α) (xs : List α) :
    listFold f z (x :: xs) = listFold f (f z x) xs := rfl

public theorem listFold_append {α σ : Type} (f : σ -> α -> σ) (z : σ) (xs ys : List α) :
    listFold f z (xs ++ ys) = listFold f (listFold f z xs) ys := List.foldl_append

-- The fold that conses every element onto its state reverses the list onto it.
public theorem listFold_prepend {α : Type} (z xs : List α) :
    listFold (fun a x => x :: a) z xs = xs.reverse ++ z := by
  induction xs generalizing z with
  | nil => rfl
  | cons x xs ih => simp only [listFold, List.foldl, List.reverse_cons, List.append_assoc] at *; rw [ih]; rfl

-- The fold that appends every element to its state copies the list after it.
public theorem listFold_extend {α : Type} (z xs : List α) :
    listFold (fun a x => a ++ [x]) z xs = z ++ xs := by
  induction xs generalizing z with
  | nil => simp [listFold]
  | cons x xs ih => simp only [listFold, List.foldl] at *; rw [ih]; simp

-- An additive fold is its initial value plus the sum of what each element adds.
public theorem listFold_add_nat {α : Type} (k : α -> Nat) (z : Nat) (xs : List α) :
    listFold (fun a x => a + k x) z xs = z + sumNat k xs := by
  induction xs generalizing z with
  | nil => simp [listFold, sumNat]
  | cons x xs ih => simp only [listFold, List.foldl, sumNat] at *; rw [ih]; omega

public theorem listFold_add_nat_self (z : Nat) (xs : List Nat) :
    listFold (fun a x => a + x) z xs = z + sumNat (fun x => x) xs :=
  listFold_add_nat (fun x => x) z xs

public theorem listFold_add_int {α : Type} (k : α -> Int) (z : Int) (xs : List α) :
    listFold (fun a x => a + k x) z xs = z + sumInt k xs := by
  induction xs generalizing z with
  | nil => simp [listFold, sumInt]
  | cons x xs ih => simp only [listFold, List.foldl, sumInt] at *; rw [ih]; omega

public theorem listFold_add_int_self (z : Int) (xs : List Int) :
    listFold (fun a x => a + x) z xs = z + sumInt (fun x => x) xs :=
  listFold_add_int (fun x => x) z xs

public theorem listFold_sub_int {α : Type} (k : α -> Int) (z : Int) (xs : List α) :
    listFold (fun a x => a - k x) z xs = z - sumInt k xs := by
  induction xs generalizing z with
  | nil => simp [listFold, sumInt]
  | cons x xs ih => simp only [listFold, List.foldl, sumInt] at *; rw [ih]; omega

public theorem listFold_sub_int_self (z : Int) (xs : List Int) :
    listFold (fun a x => a - x) z xs = z - sumInt (fun x => x) xs :=
  listFold_sub_int (fun x => x) z xs

/-! Sums: linearity, and invariance under permutation. -/

public theorem sumNat_nil {α : Type} (k : α -> Nat) : sumNat k [] = 0 := rfl

public theorem sumNat_cons {α : Type} (k : α -> Nat) (x : α) (xs : List α) :
    sumNat k (x :: xs) = k x + sumNat k xs := rfl

public theorem sumInt_nil {α : Type} (k : α -> Int) : sumInt k [] = 0 := rfl

public theorem sumInt_cons {α : Type} (k : α -> Int) (x : α) (xs : List α) :
    sumInt k (x :: xs) = k x + sumInt k xs := rfl

public theorem sumNat_add {α : Type} (k l : α -> Nat) (xs : List α) :
    sumNat (fun x => k x + l x) xs = sumNat k xs + sumNat l xs := by
  induction xs with
  | nil => rfl
  | cons x xs ih => simp only [sumNat] at *; rw [ih]; omega

public theorem sumInt_add {α : Type} (k l : α -> Int) (xs : List α) :
    sumInt (fun x => k x + l x) xs = sumInt k xs + sumInt l xs := by
  induction xs with
  | nil => rfl
  | cons x xs ih => simp only [sumInt] at *; rw [ih]; omega

public theorem sumInt_sub {α : Type} (k l : α -> Int) (xs : List α) :
    sumInt (fun x => k x - l x) xs = sumInt k xs - sumInt l xs := by
  induction xs with
  | nil => rfl
  | cons x xs ih => simp only [sumInt] at *; rw [ih]; omega

public theorem sumInt_neg {α : Type} (k : α -> Int) (xs : List α) :
    sumInt (fun x => -k x) xs = -sumInt k xs := by
  induction xs with
  | nil => rfl
  | cons x xs ih => simp only [sumInt] at *; rw [ih]; omega

public theorem sumNat_mul {α : Type} (c : Nat) (k : α -> Nat) (xs : List α) :
    sumNat (fun x => c * k x) xs = c * sumNat k xs := by
  induction xs with
  | nil => rfl
  | cons x xs ih => simp only [sumNat] at *; rw [ih, Nat.mul_add]

public theorem sumInt_mul {α : Type} (c : Int) (k : α -> Int) (xs : List α) :
    sumInt (fun x => c * k x) xs = c * sumInt k xs := by
  induction xs with
  | nil => simp [sumInt]
  | cons x xs ih => simp only [sumInt] at *; rw [ih, Int.mul_add]

public theorem sumNat_mul_self (c : Nat) (xs : List Nat) :
    sumNat (fun x => c * x) xs = c * sumNat (fun x => x) xs := sumNat_mul c (fun x => x) xs

public theorem sumInt_mul_self (c : Int) (xs : List Int) :
    sumInt (fun x => c * x) xs = c * sumInt (fun x => x) xs := sumInt_mul c (fun x => x) xs

public theorem sumNat_mul_right {α : Type} (c : Nat) (k : α -> Nat) (xs : List α) :
    sumNat (fun x => k x * c) xs = sumNat k xs * c := by
  rw [Nat.mul_comm, ← sumNat_mul]; simp only [Nat.mul_comm]

public theorem sumInt_mul_right {α : Type} (c : Int) (k : α -> Int) (xs : List α) :
    sumInt (fun x => k x * c) xs = sumInt k xs * c := by
  rw [Int.mul_comm, ← sumInt_mul]; simp only [Int.mul_comm]

public theorem sumInt_neg_self (xs : List Int) :
    sumInt (fun x => -x) xs = -sumInt (fun x => x) xs := sumInt_neg (fun x => x) xs

public theorem sumNat_const {α : Type} (c : Nat) (xs : List α) :
    sumNat (fun _ => c) xs = c * xs.length := by
  induction xs with
  | nil => rfl
  | cons x xs ih => simp only [sumNat, List.length_cons] at *; rw [ih, Nat.mul_succ]; omega

public theorem sumInt_const {α : Type} (c : Int) (xs : List α) :
    sumInt (fun _ => c) xs = c * xs.length := by
  induction xs with
  | nil => simp [sumInt]
  | cons x xs ih => simp only [sumInt, List.length_cons] at *; rw [ih]; simp [Int.mul_add]; omega

public theorem sumNat_append {α : Type} (k : α -> Nat) (xs ys : List α) :
    sumNat k (xs ++ ys) = sumNat k xs + sumNat k ys := by
  induction xs with
  | nil => simp [sumNat]
  | cons x xs ih => simp only [sumNat, List.cons_append] at *; rw [ih]; omega

public theorem sumInt_append {α : Type} (k : α -> Int) (xs ys : List α) :
    sumInt k (xs ++ ys) = sumInt k xs + sumInt k ys := by
  induction xs with
  | nil => simp [sumInt]
  | cons x xs ih => simp only [sumInt, List.cons_append] at *; rw [ih]; omega

-- A sum is invariant under every permutation of the list.
public theorem sumNat_perm {α : Type} (k : α -> Nat) {xs ys : List α} (h : xs.Perm ys) :
    sumNat k xs = sumNat k ys := by
  induction h with
  | nil => rfl
  | cons x _ ih => simp only [sumNat]; rw [ih]
  | swap x y l => simp only [sumNat]; omega
  | trans _ _ ih₁ ih₂ => exact ih₁.trans ih₂

public theorem sumInt_perm {α : Type} (k : α -> Int) {xs ys : List α} (h : xs.Perm ys) :
    sumInt k xs = sumInt k ys := by
  induction h with
  | nil => rfl
  | cons x _ ih => simp only [sumInt]; rw [ih]
  | swap x y l => simp only [sumInt]; omega
  | trans _ _ ih₁ ih₂ => exact ih₁.trans ih₂

public theorem sumNat_reverse {α : Type} (k : α -> Nat) (xs : List α) :
    sumNat k xs.reverse = sumNat k xs := sumNat_perm k (List.reverse_perm xs)

public theorem sumInt_reverse {α : Type} (k : α -> Int) (xs : List α) :
    sumInt k xs.reverse = sumInt k xs := sumInt_perm k (List.reverse_perm xs)

/-! Fusion. -/

-- A function that commutes with the step carries the fold of one step to the fold of the other.
public theorem listFold_fusion {α σ τ : Type} (f : σ -> α -> σ) (g : τ -> α -> τ) (h : σ -> τ)
    (z : σ) (xs : List α) (step : ∀ (a : σ) (x : α), h (f a x) = g (h a) x) :
    h (listFold f z xs) = listFold g (h z) xs := by
  induction xs generalizing z with
  | nil => rfl
  | cons x xs ih => simp only [listFold, List.foldl] at *; rw [ih, step]

-- Two folds over one list are one fold over the pair of their states.
public theorem listFold_pair {α σ τ : Type} (f : σ -> α -> σ) (g : τ -> α -> τ) (z : σ) (w : τ)
    (xs : List α) :
    listFold (fun p x => (f p.1 x, g p.2 x)) (z, w) xs = (listFold f z xs, listFold g w xs) := by
  induction xs generalizing z w with
  | nil => rfl
  | cons x xs ih => simp only [listFold, List.foldl] at *; rw [ih]

end LexLeanLaws
