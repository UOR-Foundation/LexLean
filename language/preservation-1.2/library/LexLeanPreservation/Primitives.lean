import LexLeanPreservation.Values
set_option linter.unusedSimpArgs false
namespace LexLeanPreservation
open LexLeanTarget.TargetSyntax LexLeanTarget.TargetSemantics

/-! One relation per primitive: what the calculus primitive observes on
encoded operands, with the exact width predicate its result check imposes. -/

/-- A natural-number result fits the 64-bit realization. -/
theorem obs_natResult (n : Nat) : obs (natResult n) = some (Rel (Nat.blt n 18446744073709551616) (.nat n)) := by
  unfold natResult
  cases Nat.blt n 18446744073709551616 <;> rfl

/-- An integer result fits the 64-bit realization. -/
def intFits (n : Int) : Bool := (LexLeanRuntime.checkedConvert n : Option Int64).isSome

theorem obs_intResult (n : Int) : obs (intResult n) = some (Rel (intFits n) (.int n)) := by
  unfold intResult intFits
  cases (LexLeanRuntime.checkedConvert n : Option Int64) <;> rfl

theorem prim_natAdd (a b : Nat) :
    obs (primitive .natAdd [.nat a, .nat b]) = some (Rel (Nat.blt (a + b) 18446744073709551616) (.nat (a + b))) :=
  obs_natResult (a + b)
theorem prim_natSub (a b : Nat) :
    obs (primitive .natSub [.nat a, .nat b]) = some (Rel true (.nat (LexLeanRuntime.subtract a b))) := rfl
theorem prim_natMul (a b : Nat) :
    obs (primitive .natMul [.nat a, .nat b]) =
      some (Rel (Nat.blt (LexLeanRuntime.multiply a b) 18446744073709551616) (.nat (LexLeanRuntime.multiply a b))) :=
  obs_natResult (LexLeanRuntime.multiply a b)
theorem prim_natQuot (a b z : Nat) :
    obs (primitive .natQuot [.nat a, .nat b, .nat z]) = some (Rel true (.nat (LexLeanRuntime.quotient a b z))) := rfl
theorem prim_natRem (a b z : Nat) :
    obs (primitive .natRem [.nat a, .nat b, .nat z]) = some (Rel true (.nat (LexLeanRuntime.remainder a b z))) := rfl
theorem prim_natEq (a b : Nat) :
    obs (primitive .natEq [.nat a, .nat b]) = some (Rel true (.bool (Nat.beq a b))) := rfl
theorem prim_natLe (a b : Nat) :
    obs (primitive .natLe [.nat a, .nat b]) = some (Rel true (.bool (Nat.ble a b))) := rfl
theorem prim_natLt (a b : Nat) :
    obs (primitive .natLt [.nat a, .nat b]) = some (Rel true (.bool (Nat.blt a b))) := rfl

theorem prim_intSub (a b : Int) :
    obs (primitive .intSub [.int a, .int b]) =
      some (Rel (intFits (LexLeanRuntime.subtract a b)) (.int (LexLeanRuntime.subtract a b))) :=
  obs_intResult (LexLeanRuntime.subtract a b)
theorem prim_intMul (a b : Int) :
    obs (primitive .intMul [.int a, .int b]) =
      some (Rel (intFits (LexLeanRuntime.multiply a b)) (.int (LexLeanRuntime.multiply a b))) :=
  obs_intResult (LexLeanRuntime.multiply a b)
theorem prim_intNeg (a : Int) :
    obs (primitive .intNeg [.int a]) =
      some (Rel (intFits (LexLeanRuntime.negate a)) (.int (LexLeanRuntime.negate a))) :=
  obs_intResult (LexLeanRuntime.negate a)
theorem prim_intQuot (a b z : Int) :
    obs (primitive .intQuot [.int a, .int b, .int z]) =
      some (Rel (intFits (LexLeanRuntime.quotient a b z)) (.int (LexLeanRuntime.quotient a b z))) :=
  obs_intResult (LexLeanRuntime.quotient a b z)
theorem prim_intRem (a b z : Int) :
    obs (primitive .intRem [.int a, .int b, .int z]) =
      some (Rel (intFits (LexLeanRuntime.remainder a b z)) (.int (LexLeanRuntime.remainder a b z))) :=
  obs_intResult (LexLeanRuntime.remainder a b z)

theorem prim_boolNot (b : Bool) :
    obs (primitive .boolNot [.bool b]) = some (Rel true (.bool (!b))) := rfl

theorem prim_equal_nat (a b : Nat) :
    obs (primitive .equal [.nat a, .nat b]) = some (Rel true (.bool (LexLeanRuntime.equal a b))) := rfl
theorem prim_equal_bool (a b : Bool) :
    obs (primitive .equal [.bool a, .bool b]) = some (Rel true (.bool (LexLeanRuntime.equal a b))) := rfl
theorem prim_equal_string (a b : String) :
    obs (primitive .equal [.string a, .string b]) = some (Rel true (.bool (LexLeanRuntime.equal a b))) := rfl
theorem prim_equal_bytes (a b : ByteArray) :
    obs (primitive .equal [.bytes a, .bytes b]) = some (Rel true (.bool (LexLeanRuntime.equal a b))) := rfl
theorem prim_equal_ordering (a b : Ordering) :
    obs (primitive .equal [encOrdering a, encOrdering b]) = some (Rel true (.bool (LexLeanRuntime.equal a b))) := by
  cases a <;> cases b <;> rfl

theorem prim_append_bytes (a b : ByteArray) :
    obs (primitive .append [.bytes a, .bytes b]) = some (Rel true (.bytes (LexLeanRuntime.append a b))) := rfl
theorem prim_append_list {α : Type} (L : ListEnc α) (xs ys : List α) :
    obs (primitive .append [L.enc xs, L.enc ys]) =
      some (Rel true (L.enc (LexLeanRuntime.append xs ys))) := by
  show some (Obs.value (.list ((L.items xs).append (L.items ys)))) = some (Obs.value (.list (L.items (xs.append ys))))
  rw [L.items_eq, L.items_eq, L.items_eq, List.append_eq, List.append_eq, List.map_append]

theorem prim_length_bytes (a : ByteArray) :
    obs (primitive .length [.bytes a]) =
      some (Rel (Nat.blt (LexLeanRuntime.length a) 18446744073709551616) (.nat (LexLeanRuntime.length a))) :=
  obs_natResult _
theorem prim_length_string (a : String) :
    obs (primitive .length [.string a]) =
      some (Rel (Nat.blt (LexLeanRuntime.length a) 18446744073709551616) (.nat (LexLeanRuntime.length a))) :=
  obs_natResult _
theorem prim_length_list {α : Type} (L : ListEnc α) (xs : List α) :
    obs (primitive .length [L.enc xs]) =
      some (Rel (Nat.blt (LexLeanRuntime.length xs) 18446744073709551616) (.nat (LexLeanRuntime.length xs))) := by
  show obs (natResult (LexLeanRuntime.length (L.items xs))) = _
  have h : LexLeanRuntime.length (L.items xs) = LexLeanRuntime.length xs := by
    rw [L.items_eq]
    simp only [LexLeanRuntime.length, LexLeanRuntime.Lengthable.length, List.length_map]
  rw [h]
  exact obs_natResult _

theorem listIndex_map {α : Type} (e : α → Value) : ∀ (xs : List α) (i : Nat),
    LexLeanRuntime.listIndex (xs.map e) i = (LexLeanRuntime.listIndex xs i).map e
  | [], _ => rfl
  | _ :: _, 0 => rfl
  | _ :: xs, i + 1 => listIndex_map e xs i

/-- Any function satisfying `listIndex`'s clauses is `listIndex`: the
relation lemma takes the user module's own copy by its clauses. -/
theorem listIndex_unique {α : Type} (li : List α → Nat → Option α)
    (h0 : ∀ i, li [] i = none) (h1 : ∀ x xs, li (x :: xs) 0 = some x)
    (h2 : ∀ x xs i, li (x :: xs) (i + 1) = li xs i) :
    ∀ xs i, li xs i = LexLeanRuntime.listIndex xs i
  | [], i => h0 i
  | x :: xs, 0 => h1 x xs
  | x :: xs, i + 1 => (h2 x xs i).trans (listIndex_unique li h0 h1 h2 xs i)

theorem prim_index_list {α : Type} (L : ListEnc α) (O : OptEnc α) (hO : ∀ a, O.elem a = L.elem a)
    (li : List α → Nat → Option α)
    (h0 : ∀ i, li [] i = none) (h1 : ∀ x xs, li (x :: xs) 0 = some x)
    (h2 : ∀ x xs i, li (x :: xs) (i + 1) = li xs i) (xs : List α) (i : Nat) :
    obs (primitive .index [L.enc xs, .nat i]) = some (Rel true (O.enc (li xs i))) := by
  show obs (match (LexLeanRuntime.index (L.items xs) i : Option Value) with
    | none => Outcome.value Value.none 0 | some item => Outcome.value (Value.some item) 0) = _
  have h : (LexLeanRuntime.index (L.items xs) i : Option Value) = (li xs i).map L.elem := by
    rw [listIndex_unique li h0 h1 h2, L.items_eq]
    exact listIndex_map L.elem xs i
  rw [h, O.enc_map]
  cases li xs i with
  | none => rfl
  | some a => show some (Obs.value (Value.some (L.elem a))) = some (Obs.value (Value.some (O.elem a))); rw [hO]

theorem prim_index_bytes (a : ByteArray) (i : Nat) :
    obs (primitive .index [.bytes a, .nat i]) =
      some (Rel true (encOption Value.u8 (LexLeanRuntime.index a i : Option UInt8))) := by
  show obs (match (LexLeanRuntime.index a i : Option UInt8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u8 r)) 0) = _
  cases (LexLeanRuntime.index a i : Option UInt8) <;> rfl

theorem prim_slice_bytes (a : ByteArray) (s c : Nat) :
    obs (primitive .slice [.bytes a, .nat s, .nat c]) =
      some (Rel true (encOption Value.bytes (LexLeanRuntime.slice a s c))) := by
  show obs (match (LexLeanRuntime.slice a s c : Option ByteArray) with
    | none => Outcome.value Value.none 0 | some part => Outcome.value (Value.some (Value.bytes part)) 0) = _
  cases (LexLeanRuntime.slice a s c : Option ByteArray) <;> rfl

theorem prim_slice_list {α : Type} (L : ListEnc α) (O : OptEnc (List α)) (hO : ∀ xs, O.elem xs = L.enc xs)
    (xs : List α) (s c : Nat) :
    obs (primitive .slice [L.enc xs, .nat s, .nat c]) =
      some (Rel true (O.enc (LexLeanRuntime.slice xs s c))) := by
  show obs (match (LexLeanRuntime.slice (L.items xs) s c : Option (List Value)) with
    | none => Outcome.value Value.none 0 | some part => Outcome.value (Value.some (Value.list part)) 0) = _
  have h : (LexLeanRuntime.slice (L.items xs) s c : Option (List Value)) =
      (LexLeanRuntime.slice xs s c).map L.items := by
    simp only [L.items_eq, LexLeanRuntime.slice, LexLeanRuntime.Sliceable.slice, List.length_map]
    split <;> simp [L.items_eq, List.map_take, List.map_drop]
  rw [h, O.enc_map]
  cases (LexLeanRuntime.slice xs s c : Option (List α)) with
  | none => rfl
  | some part => show some (Obs.value (Value.some (Value.list (L.items part)))) = some (Obs.value (Value.some (O.elem part))); rw [hO]; rfl

theorem prim_utf8Encode (s : String) :
    obs (primitive .utf8Encode [.string s]) = some (Rel true (.bytes (LexLeanRuntime.utf8Encode s))) := rfl
theorem prim_utf8Decode (b : ByteArray) :
    obs (primitive .utf8Decode [.bytes b]) = some (Rel true (encOption Value.string (LexLeanRuntime.utf8Decode b))) := by
  show obs (match (LexLeanRuntime.utf8Decode b : Option String) with
    | none => Outcome.value Value.none 0 | some text => Outcome.value (Value.some (Value.string text)) 0) = _
  cases (LexLeanRuntime.utf8Decode b : Option String) <;> rfl

theorem bytes_zero : (ByteArray.mk #[0]).toList = [0] := by
  rw [ByteArray.toList, ByteArray.toList.loop, ByteArray.toList.loop]
  simp [ByteArray.size, ByteArray.get!]
theorem bytes_one : (ByteArray.mk #[1]).toList = [1] := by
  rw [ByteArray.toList, ByteArray.toList.loop, ByteArray.toList.loop]
  simp [ByteArray.size, ByteArray.get!]
theorem bytes_empty : (ByteArray.mk #[]).toList = [] := by
  rw [ByteArray.toList, ByteArray.toList.loop]
  simp [ByteArray.size]

theorem fromOrdering_encOrdering (o : Ordering) : Value.ordering (fromOrdering o) = encOrdering o := by
  cases o <;> simp [fromOrdering, LexLeanRuntime.equal, LexLeanRuntime.compareBytes, bytes_zero, bytes_one,
    bytes_empty, encOrdering] <;> decide

theorem prim_compareBytes (a b : ByteArray) :
    obs (primitive .compareBytes [.bytes a, .bytes b]) =
      some (Rel true (encOrdering (LexLeanRuntime.compareBytes a b))) := by
  show some (Obs.value (Value.ordering (fromOrdering (LexLeanRuntime.compareBytes a b)))) = _
  rw [fromOrdering_encOrdering]
  rfl

theorem fromStrings_map : ∀ (xs : List String), fromStrings xs = xs.map Value.string
  | [] => rfl
  | x :: xs => by
    show Value.string x :: fromStrings xs = Value.string x :: xs.map Value.string
    rw [fromStrings_map xs]

theorem toStrings_map : ∀ (xs : List String), toStrings (xs.map Value.string) = some xs
  | [] => rfl
  | x :: xs => by
    show (match toStrings (xs.map Value.string) with
      | none => none | some rest => some (x :: rest)) = some (x :: xs)
    rw [toStrings_map xs]

theorem prim_splitExact (s d : String) (m : UInt32) :
    obs (primitive .splitExact [.string s, .string d, .u32 m]) =
      some (Rel true (encOption (encList Value.string) (LexLeanRuntime.splitExact s d m))) := by
  show obs (match (LexLeanRuntime.splitExact s d m : Option (List String)) with
    | none => Outcome.value Value.none 0
    | some parts => Outcome.value (Value.some (Value.list (fromStrings parts))) 0) = _
  cases (LexLeanRuntime.splitExact s d m : Option (List String)) with
  | none => rfl
  | some parts =>
    show obs (Outcome.value (Value.some (Value.list (fromStrings parts))) 0) = _
    rw [fromStrings_map]
    rfl

theorem prim_join (xs : List String) (d : String) :
    obs (primitive .join [encList Value.string xs, .string d]) =
      some (Rel true (.string (LexLeanRuntime.join xs d))) := by
  show obs (match toStrings (xs.map Value.string) with
    | none => Outcome.stuck
    | some texts => Outcome.value (Value.string (LexLeanRuntime.join texts d)) 0) = _
  rw [toStrings_map]
  rfl

end LexLeanPreservation
