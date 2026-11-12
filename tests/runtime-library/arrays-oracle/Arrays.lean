module
public import Init
public import LexLeanRuntime.Portable
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
namespace LexLeanArrays

/-! Arrays answer the portable `length` and `index` primitives. -/

public instance arrayLengthable {α : Type} : LexLeanRuntime.Lengthable (Array α) where
  length := Array.size

public instance arrayIndexable {α : Type} : LexLeanRuntime.Indexable (Array α) α where
  index := fun value offset => value[offset]?

/-! The array primitives of language 1.2. -/

@[expose] public def fromList {α : Type} (values : List α) : Array α := List.toArray values

@[expose] public def toList {α : Type} (value : Array α) : List α := Array.toList value

@[expose] public def set {α : Type} (value : Array α) (offset : Nat) (element : α) : Array α :=
  Array.setIfInBounds value offset element

/-! The laws a `law` proof applies: an array is the list of its elements. -/

public theorem to_list_from_list {α : Type} (values : List α) :
    toList (fromList values) = values := rfl

public theorem from_list_to_list {α : Type} (value : Array α) :
    fromList (toList value) = value := rfl

public theorem length_to_list {α : Type} (value : Array α) :
    LexLeanRuntime.length value = LexLeanRuntime.length (toList value) := rfl

public theorem list_index {α : Type} (values : List α) (offset : Nat) :
    (LexLeanRuntime.index values offset : Option α) = values[offset]? := by
  induction values generalizing offset with
  | nil => rfl
  | cons head tail hypothesis =>
    cases offset with
    | zero => rfl
    | succ offset => exact hypothesis offset

public theorem index_to_list {α : Type} (value : Array α) (offset : Nat) :
    (LexLeanRuntime.index value offset : Option α) = LexLeanRuntime.index (toList value) offset := by
  rw [list_index]
  exact (Array.getElem?_toList).symm

public theorem index_beyond {α : Type} (value : Array α) (offset : Nat) :
    LexLeanRuntime.length value ≤ offset →
      (LexLeanRuntime.index value offset : Option α) = none := by
  intro beyond
  exact Array.getElem?_eq_none beyond

public theorem length_set {α : Type} (value : Array α) (offset : Nat) (element : α) :
    LexLeanRuntime.length (set value offset element) = LexLeanRuntime.length value :=
  Array.size_setIfInBounds

public theorem index_set_same {α : Type} (value : Array α) (offset : Nat) (element : α) :
    offset < LexLeanRuntime.length value →
      (LexLeanRuntime.index (set value offset element) offset : Option α) = some element := by
  intro bound
  exact Array.getElem?_setIfInBounds_self_of_lt bound

public theorem index_set_other {α : Type} (value : Array α) (offset other : Nat) (element : α) :
    Nat.beq offset other = false →
      (LexLeanRuntime.index (set value offset element) other : Option α) =
        LexLeanRuntime.index value other := by
  intro distinct
  exact Array.getElem?_setIfInBounds_ne (Nat.ne_of_beq_eq_false distinct)

public theorem set_beyond {α : Type} (value : Array α) (offset : Nat) (element : α) :
    LexLeanRuntime.length value ≤ offset → set value offset element = value := by
  intro beyond
  exact Array.setIfInBounds_eq_of_size_le beyond

end LexLeanArrays
