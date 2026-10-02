import LexLeanPreservation.Core
namespace LexLeanPreservation
open LexLeanTarget.TargetSyntax LexLeanTarget.TargetSemantics

/-! The encoding of source values into calculus values: one encoder per
source type, and the list and option encoder bundles a nested inductive's
auxiliary encoders instantiate. -/

def encUnit (_ : Unit) : Value := .unit

def encOrdering : Ordering → Value
  | .lt => .ordering .less
  | .eq => .ordering .same
  | .gt => .ordering .more

def encOption {α : Type} (e : α → Value) : Option α → Value
  | none => .none
  | some a => .some (e a)

def encExcept {ε α : Type} (ee : ε → Value) (ea : α → Value) : Except ε α → Value
  | .error x => .error (ee x)
  | .ok a => .ok (ea a)

def encList {α : Type} (e : α → Value) (xs : List α) : Value := .list (xs.map e)

def encPair {α β : Type} (ea : α → Value) (eb : β → Value) (p : α × β) : Value := .pair (ea p.1) (eb p.2)

/-- A list encoder given by its element encoder and an item function that
follows the list structurally. A list of a nested inductive is encoded by an
auxiliary of the type's own mutual block, not by `List.map`, and every lemma
about lists takes the encoder in this form. -/
structure ListEnc (α : Type) where
  elem : α → Value
  items : List α → List Value
  items_nil : items [] = []
  items_cons : ∀ x xs, items (x :: xs) = elem x :: items xs

def ListEnc.enc {α : Type} (L : ListEnc α) (xs : List α) : Value := .list (L.items xs)

theorem ListEnc.items_eq {α : Type} (L : ListEnc α) : ∀ xs, L.items xs = xs.map L.elem
  | [] => L.items_nil
  | x :: xs => by rw [L.items_cons, ListEnc.items_eq L xs]; rfl

def listEnc {α : Type} (e : α → Value) : ListEnc α := ⟨e, fun xs => xs.map e, rfl, fun _ _ => rfl⟩

/-- An option encoder given by its element encoder. -/
structure OptEnc (α : Type) where
  elem : α → Value
  enc : Option α → Value
  enc_none : enc none = .none
  enc_some : ∀ a, enc (some a) = .some (elem a)

def optEnc {α : Type} (e : α → Value) : OptEnc α := ⟨e, encOption e, rfl, fun _ => rfl⟩

theorem OptEnc.enc_map {α : Type} (O : OptEnc α) (o : Option α) :
    O.enc o = encOption O.elem o := by
  cases o
  · exact O.enc_none
  · exact O.enc_some _

theorem construct_true : obs (construct .true []) = some (Rel true (.bool true)) := rfl
theorem construct_false : obs (construct .false []) = some (Rel true (.bool false)) := rfl
theorem construct_unit : obs (construct .unit []) = some (Rel true .unit) := rfl
theorem construct_nil : obs (construct .nil []) = some (Rel true (.list [])) := rfl
theorem construct_cons {h : Value} {t : List Value} : obs (construct .cons [h, .list t]) = some (Rel true (.list (h :: t))) := rfl
theorem construct_pair {a b : Value} : obs (construct .pair [a, b]) = some (Rel true (.pair a b)) := rfl
theorem construct_none : obs (construct .none []) = some (Rel true .none) := rfl
theorem construct_some {a : Value} : obs (construct .some [a]) = some (Rel true (.some a)) := rfl
theorem construct_ok {a : Value} : obs (construct .ok [a]) = some (Rel true (.ok a)) := rfl
theorem construct_error {a : Value} : obs (construct .error [a]) = some (Rel true (.error a)) := rfl
theorem construct_zero : obs (construct .zero []) = some (Rel true (.nat 0)) := rfl
theorem construct_succ {n : Nat} : obs (construct .succ [.nat n]) = some (Rel (Nat.blt (n + 1) 18446744073709551616) (.nat (n + 1))) := by
  show obs (natResult (n + 1)) = _
  unfold natResult
  cases Nat.blt (n + 1) 18446744073709551616 <;> rfl
theorem construct_adt {k : Nat} {fs : List Value} : obs (construct (.adt k) fs) = some (Rel true (.adt k fs)) := rfl

end LexLeanPreservation
