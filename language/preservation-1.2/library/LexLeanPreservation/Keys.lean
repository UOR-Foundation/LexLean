import LexLeanPreservation.Primitives
namespace LexLeanPreservation
open LexLeanTarget.TargetSyntax LexLeanTarget.TargetSemantics

/-! Key orders: the calculus `compare` agrees with every source key order,
each stated over the order the certificate names, so the collection lemmas'
clauses mention the user module's own `Key.compare`. -/

/-- The realization's order of a source comparison. -/
def ordOf : Ordering → Order
  | .lt => .less
  | .eq => .same
  | .gt => .more

/-- The realization's `compare` agrees with a source key order on encoded
keys. A collection lemma takes the order as a parameter, so its clauses can
name the user module's own `Key.compare`. -/
structure KeySpec {κ : Type} (enc : κ → Value) (cmp : κ → κ → Ordering) : Prop where
  spec : ∀ a b, compareValue (enc a) (enc b) = some (ordOf (cmp a b))

theorem prim_compare_key {κ : Type} {enc : κ → Value} {cmp : κ → κ → Ordering}
    (K : KeySpec enc cmp) (a b : κ) :
    obs (primitive .compare [enc a, enc b]) = some (Rel true (.ordering (ordOf (cmp a b)))) := by
  show obs (match compareValue (enc a) (enc b) with
    | none => Outcome.stuck | some order => Outcome.value (Value.ordering order) 0) = _
  rw [K.spec]
  rfl

/-- The realization orders two keys by inserting them into a two-entry map:
the order is read off the position of the second key. -/
theorem orderOf_insert {κ : Type} [LexLeanCollections.Key κ] (a b : κ) :
    orderOf (LexLeanCollections.mapValues (LexLeanCollections.mapInsert
      (LexLeanCollections.mapInsert ([] : List (κ × Nat)) a 0) b 1)) =
      ordOf (LexLeanCollections.Key.compare b a).swap := by
  show orderOf (LexLeanCollections.mapValues (LexLeanCollections.insertEntry b 1 [(a, 0)])) = _
  simp only [LexLeanCollections.insertEntry]
  cases LexLeanCollections.Key.compare b a <;> rfl

theorem keySpec_nat (cmp : Nat → Nat → Ordering) (h : ∀ a b, cmp a b = compare a b) :
    KeySpec Value.nat cmp := ⟨fun a b => by
  show some (orderNat a b) = _
  rw [orderNat, orderOf_insert, h]
  show some (ordOf (compare b a).swap) = _
  rw [← Nat.compare_swap, Ordering.swap_swap]⟩

theorem keySpec_int (cmp : Int → Int → Ordering) (h : ∀ a b, cmp a b = compare a b) :
    KeySpec Value.int cmp := ⟨fun a b => by
  show some (orderInt a b) = _
  rw [orderInt, orderOf_insert, h]
  show some (ordOf (compare b a).swap) = _
  rw [← Int.compare_swap, Ordering.swap_swap]⟩

theorem keySpec_bool (cmp : Bool → Bool → Ordering) (h : ∀ a b, cmp a b = compare a b) :
    KeySpec Value.bool cmp := ⟨fun a b => by
  show some (orderBool a b) = _
  rw [orderBool, orderOf_insert, h]
  cases a <;> cases b <;> rfl⟩

theorem keySpec_i8 (cmp : Int8 → Int8 → Ordering) (h : ∀ a b, cmp a b = compare a.toInt b.toInt) :
    KeySpec Value.i8 cmp := ⟨fun a b => by
  show some (orderI8 a b) = _
  rw [orderI8, orderOf_insert, h]
  show some (ordOf (compare b.toInt a.toInt).swap) = _
  rw [← Int.compare_swap, Ordering.swap_swap]⟩

theorem keySpec_i16 (cmp : Int16 → Int16 → Ordering) (h : ∀ a b, cmp a b = compare a.toInt b.toInt) :
    KeySpec Value.i16 cmp := ⟨fun a b => by
  show some (orderI16 a b) = _
  rw [orderI16, orderOf_insert, h]
  show some (ordOf (compare b.toInt a.toInt).swap) = _
  rw [← Int.compare_swap, Ordering.swap_swap]⟩

theorem keySpec_i32 (cmp : Int32 → Int32 → Ordering) (h : ∀ a b, cmp a b = compare a.toInt b.toInt) :
    KeySpec Value.i32 cmp := ⟨fun a b => by
  show some (orderI32 a b) = _
  rw [orderI32, orderOf_insert, h]
  show some (ordOf (compare b.toInt a.toInt).swap) = _
  rw [← Int.compare_swap, Ordering.swap_swap]⟩

theorem keySpec_i64 (cmp : Int64 → Int64 → Ordering) (h : ∀ a b, cmp a b = compare a.toInt b.toInt) :
    KeySpec Value.i64 cmp := ⟨fun a b => by
  show some (orderI64 a b) = _
  rw [orderI64, orderOf_insert, h]
  show some (ordOf (compare b.toInt a.toInt).swap) = _
  rw [← Int.compare_swap, Ordering.swap_swap]⟩

theorem keySpec_u8 (cmp : UInt8 → UInt8 → Ordering) (h : ∀ a b, cmp a b = compare a.toNat b.toNat) :
    KeySpec Value.u8 cmp := ⟨fun a b => by
  show some (orderU8 a b) = _
  rw [orderU8, orderOf_insert, h]
  show some (ordOf (compare b.toNat a.toNat).swap) = _
  rw [← Nat.compare_swap, Ordering.swap_swap]⟩

theorem keySpec_u16 (cmp : UInt16 → UInt16 → Ordering) (h : ∀ a b, cmp a b = compare a.toNat b.toNat) :
    KeySpec Value.u16 cmp := ⟨fun a b => by
  show some (orderU16 a b) = _
  rw [orderU16, orderOf_insert, h]
  show some (ordOf (compare b.toNat a.toNat).swap) = _
  rw [← Nat.compare_swap, Ordering.swap_swap]⟩

theorem keySpec_u32 (cmp : UInt32 → UInt32 → Ordering) (h : ∀ a b, cmp a b = compare a.toNat b.toNat) :
    KeySpec Value.u32 cmp := ⟨fun a b => by
  show some (orderU32 a b) = _
  rw [orderU32, orderOf_insert, h]
  show some (ordOf (compare b.toNat a.toNat).swap) = _
  rw [← Nat.compare_swap, Ordering.swap_swap]⟩

theorem keySpec_u64 (cmp : UInt64 → UInt64 → Ordering) (h : ∀ a b, cmp a b = compare a.toNat b.toNat) :
    KeySpec Value.u64 cmp := ⟨fun a b => by
  show some (orderU64 a b) = _
  rw [orderU64, orderOf_insert, h]
  show some (ordOf (compare b.toNat a.toNat).swap) = _
  rw [← Nat.compare_swap, Ordering.swap_swap]⟩

/-- Code-point comparison of character lists, swapped. -/
theorem compareCodes_swap : ∀ (l r : List Char),
    LexLeanCollections.compareCodes r l = (LexLeanCollections.compareCodes l r).swap
  | [], [] => rfl
  | [], _ :: _ => rfl
  | _ :: _, [] => rfl
  | a :: as, b :: bs => by
    simp only [LexLeanCollections.compareCodes]
    rw [← Nat.compare_swap a.val.toNat b.val.toNat, compareCodes_swap as bs]
    cases compare a.val.toNat b.val.toNat <;> rfl

/-- A source code-point comparison given by its defining clauses is the
realization's. -/
theorem compareCodes_eq (cc : List Char → List Char → Ordering)
    (c0 : cc [] [] = .eq) (c1 : ∀ r rs, cc [] (r :: rs) = .lt) (c2 : ∀ l ls, cc (l :: ls) [] = .gt)
    (c3 : ∀ l ls r rs, compare l.val.toNat r.val.toNat = .eq → cc (l :: ls) (r :: rs) = cc ls rs)
    (c4 : ∀ l ls r rs, compare l.val.toNat r.val.toNat = .lt → cc (l :: ls) (r :: rs) = .lt)
    (c5 : ∀ l ls r rs, compare l.val.toNat r.val.toNat = .gt → cc (l :: ls) (r :: rs) = .gt) :
    ∀ l r, cc l r = LexLeanCollections.compareCodes l r
  | [], [] => c0
  | [], r :: rs => c1 r rs
  | l :: ls, [] => c2 l ls
  | l :: ls, r :: rs => by
    simp only [LexLeanCollections.compareCodes]
    cases h : compare l.val.toNat r.val.toNat
    · rw [c4 l ls r rs h]
    · rw [c3 l ls r rs h]; exact compareCodes_eq cc c0 c1 c2 c3 c4 c5 ls rs
    · rw [c5 l ls r rs h]

theorem keySpec_string (cc : List Char → List Char → Ordering)
    (c0 : cc [] [] = .eq) (c1 : ∀ r rs, cc [] (r :: rs) = .lt) (c2 : ∀ l ls, cc (l :: ls) [] = .gt)
    (c3 : ∀ l ls r rs, compare l.val.toNat r.val.toNat = .eq → cc (l :: ls) (r :: rs) = cc ls rs)
    (c4 : ∀ l ls r rs, compare l.val.toNat r.val.toNat = .lt → cc (l :: ls) (r :: rs) = .lt)
    (c5 : ∀ l ls r rs, compare l.val.toNat r.val.toNat = .gt → cc (l :: ls) (r :: rs) = .gt)
    (cmp : String → String → Ordering) (h : ∀ a b, cmp a b = cc a.toList b.toList) :
    KeySpec Value.string cmp := ⟨fun a b => by
  show some (orderString a b) = _
  rw [orderString, orderOf_insert, h, compareCodes_eq cc c0 c1 c2 c3 c4 c5]
  show some (ordOf (LexLeanCollections.compareCodes b.toList a.toList).swap) = _
  rw [compareCodes_swap a.toList b.toList, Ordering.swap_swap]⟩

/-- Pairs compare lexicographically. -/
theorem keySpec_pair {α β : Type} {ea : α → Value} {eb : β → Value}
    {ca : α → α → Ordering} {cb : β → β → Ordering}
    (A : KeySpec ea ca) (B : KeySpec eb cb) (cmp : α × β → α × β → Ordering)
    (hl : ∀ a b, ca a.1 b.1 = .lt → cmp a b = .lt)
    (he : ∀ a b, ca a.1 b.1 = .eq → cmp a b = cb a.2 b.2)
    (hg : ∀ a b, ca a.1 b.1 = .gt → cmp a b = .gt) :
    KeySpec (encPair ea eb) cmp := ⟨fun a b => by
  show compareValue (.pair (ea a.1) (eb a.2)) (.pair (ea b.1) (eb b.2)) = _
  simp only [compareValue, A.spec]
  cases h : ca a.1 b.1
  · rw [hl a b h]; rfl
  · rw [he a b h]; exact B.spec a.2 b.2
  · rw [hg a b h]; rfl⟩

/-- A key comparison as the realization writes it: `compare`, then a match
on the order it returns (`primitive.less_than`). -/
def lessThanE (a b : Expr) : Expr :=
  .match .bool (.prim .compare [a, b])
    [.arm .lt [] (.build .true .bool []), .arm .eq [] (.build .false .bool []),
      .arm .gt [] (.build .false .bool [])]

/-- A key comparison relates to the source's `lessThan`, given the clauses
by which it unfolds to the key order. -/
theorem conv_lessThan {κ : Type} {enc : κ → Value} {cmp : κ → κ → Ordering} (K : KeySpec enc cmp)
    (lt : κ → κ → Bool)
    (hlt : ∀ x y, cmp x y = .lt → lt x y = true)
    (heq : ∀ x y, cmp x y = .eq → lt x y = false)
    (hgt : ∀ x y, cmp x y = .gt → lt x y = false)
    {p env ea eb f} {a b : κ} (hl : ConvL p env [ea, eb] (RelL f [enc a, enc b])) :
    Conv p env (lessThanE ea eb) (Rel f (.bool (lt a b))) := by
  have hs := conv_prim hl (prim_compare_key K a b)
  simp only [Bool.and_true] at hs
  show Conv p env (.match .bool (.prim .compare [ea, eb])
    [.arm .lt [] (.build .true .bool []), .arm .eq [] (.build .false .bool []),
      .arm .gt [] (.build .false .bool [])]) _
  cases h : cmp a b
  · rw [hlt a b h]
    rw [h] at hs
    rw [show f = (f && (true && true)) by simp]
    exact conv_match hs (convA_hit rfl rfl (conv_build convL_nil construct_true))
  · rw [heq a b h]
    rw [h] at hs
    rw [show f = (f && (true && true)) by simp]
    exact conv_match hs (convA_miss rfl (convA_hit rfl rfl (conv_build convL_nil construct_false)))
  · rw [hgt a b h]
    rw [h] at hs
    rw [show f = (f && (true && true)) by simp]
    exact conv_match hs
      (convA_miss rfl (convA_miss rfl (convA_hit rfl rfl (conv_build convL_nil construct_false))))

end LexLeanPreservation
