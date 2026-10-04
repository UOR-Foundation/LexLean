import LexLeanPreservation.RustSound
import LexLeanPreservation.Validate
/-!
Certificate E (SPEC.md §17.17): certificates A and B composed. Certificate
A states a root's calculus observation on its encoded arguments;
certificate B states that every function of the program is simulated by
its rendering on well-typed arguments; the encodings are well typed, so the
rendering of the root, invoked on the encoded arguments, realizes the
encoded source result.
-/
set_option autoImplicit false
namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax

theorem rel_ne_stuck (f : Bool) (v : Value) : Rel f v ≠ .stuck := by
  cases f <;> simp [Rel]

/-- Certificates A and B composed: a function's calculus observation on
`args`, which certificate A states, is realized by its rendering. -/
theorem compose {p : Program} {c : RCrate} {A : Flags} {f : Nat}
    {fn : Function} {args : List Value}
    {env : List (Nat × Value)} {F : Bool} {o : Obs}
    (hB : ∀ n, FunSem p c A n f)
    (hfn : TargetSemantics.LexLeanRuntime.index p.functions f = some fn)
    (hwt : WTL p c A args fn.types)
    (hbind : TargetSemantics.bindAll fn.parameters args [] = some env)
    (hF : fnFallible c f = some F)
    (hA : RunConv p f args o) (hne : o ≠ .stuck) :
    ∃ ro, RealizesFn F o ro ∧ RCI c (fnIdent f) args ro := by
  obtain ⟨n, hn⟩ := hA
  simp only [TargetSemantics.run, hfn, hbind] at hn
  exact (hB n fn args env F hfn hwt hbind hF o hn hne).2

/-! Encoded source values are well typed. -/

section wt
variable {p : Program} {c : RCrate} {A : Flags}

theorem wtl_nil : WTL p c A [] [] := by simp [WTL]
theorem wtl_cons {v : Value} {t : Ty} {vs : List Value} {ts : List Ty} (h : WT p c A v t)
    (hs : WTL p c A vs ts) : WTL p c A (v :: vs) (t :: ts) := by simp only [WTL]; exact ⟨h, hs⟩
theorem wtAll_nil {t : Ty} : WTAll p c A [] t := by simp [WTAll]
theorem wtAll_cons {v : Value} {vs : List Value} {t : Ty} (h : WT p c A v t) (hs : WTAll p c A vs t) :
    WTAll p c A (v :: vs) t := by simp only [WTAll]; exact ⟨h, hs⟩

theorem wt_nat (n : Nat) : WT p c A (.nat n) .nat := by simp [WT, inert]
theorem wt_int (n : Int) : WT p c A (.int n) .int := by simp [WT, inert]
theorem wt_bool (b : Bool) : WT p c A (.bool b) .bool := by simp [WT, inert]
theorem wt_string (s : String) : WT p c A (.string s) .string := by simp [WT]
theorem wt_bytes (b : ByteArray) : WT p c A (.bytes b) .bytes := by simp [WT]
theorem wt_unitValue : WT p c A .unit .unit := by simp [WT]
theorem wt_ordering (o : Order) : WT p c A (.ordering o) .ordering := by simp [WT, inert]
theorem wt_u8 (x : UInt8) : WT p c A (.u8 x) (.fixed .u8) := by simp [WT, RustSemantics.hasWidth]
theorem wt_u16 (x : UInt16) : WT p c A (.u16 x) (.fixed .u16) := by simp [WT, RustSemantics.hasWidth]
theorem wt_u32 (x : UInt32) : WT p c A (.u32 x) (.fixed .u32) := by simp [WT, RustSemantics.hasWidth]
theorem wt_u64 (x : UInt64) : WT p c A (.u64 x) (.fixed .u64) := by simp [WT, RustSemantics.hasWidth]
theorem wt_i8 (x : Int8) : WT p c A (.i8 x) (.fixed .i8) := by simp [WT, RustSemantics.hasWidth]
theorem wt_i16 (x : Int16) : WT p c A (.i16 x) (.fixed .i16) := by simp [WT, RustSemantics.hasWidth]
theorem wt_i32 (x : Int32) : WT p c A (.i32 x) (.fixed .i32) := by simp [WT, RustSemantics.hasWidth]
theorem wt_i64 (x : Int64) : WT p c A (.i64 x) (.fixed .i64) := by simp [WT, RustSemantics.hasWidth]

theorem wt_list {vs : List Value} {t : Ty} (h : WTAll p c A vs t) : WT p c A (.list vs) (.list t) := by
  simp only [WT]; exact h
theorem wt_adt {i k : Nat} {fs : List Value} {tys : List Ty} (h : adtFields p i k = some tys)
    (hf : WTL p c A fs tys) : WT p c A (.adt k fs) (.adt i) := by
  simp only [WT]; exact ⟨tys, h, hf⟩

theorem wt_encUnit (u : Unit) : WT p c A (encUnit u) .unit := by simp [encUnit, WT]
theorem wt_encOrdering (o : Ordering) : WT p c A (encOrdering o) .ordering := by
  cases o <;> simp [encOrdering, WT, inert]
theorem wt_noneV {t : Ty} : WT p c A .none (.option t) := by simp [WT]
theorem wt_someV {v : Value} {t : Ty} (h : WT p c A v t) : WT p c A (.some v) (.option t) := by
  simp only [WT]; exact h
theorem wt_pairV {a b : Value} {s t : Ty} (ha : WT p c A a s) (hb : WT p c A b t) :
    WT p c A (.pair a b) (.pair s t) := by simp only [WT]; exact ⟨ha, hb⟩
theorem wt_okV {v : Value} {s t : Ty} (h : WT p c A v s) : WT p c A (.ok v) (.result s t) := by
  simp only [WT]; exact h
theorem wt_errorV {v : Value} {s t : Ty} (h : WT p c A v t) : WT p c A (.error v) (.result s t) := by
  simp only [WT]; exact h

theorem wtAll_map {α : Type} {e : α → Value} {t : Ty} (h : ∀ x, WT p c A (e x) t) :
    ∀ xs : List α, WTAll p c A (xs.map e) t
  | [] => wtAll_nil
  | x :: xs => wtAll_cons (h x) (wtAll_map h xs)

theorem wt_encList {α : Type} {e : α → Value} {t : Ty} (h : ∀ x, WT p c A (e x) t) (xs : List α) :
    WT p c A (encList e xs) (.list t) := wt_list (wtAll_map h xs)
theorem wt_listEnc {α : Type} (L : ListEnc α) {t : Ty} (h : ∀ x, WT p c A (L.elem x) t) (xs : List α) :
    WT p c A (L.enc xs) (.list t) := by
  simp only [ListEnc.enc, ListEnc.items_eq]; exact wt_list (wtAll_map h xs)
theorem wt_optEnc {α : Type} (O : OptEnc α) {t : Ty} (h : ∀ x, WT p c A (O.elem x) t) :
    ∀ o : Option α, WT p c A (O.enc o) (.option t)
  | none => by rw [O.enc_none]; exact wt_noneV
  | some a => by rw [O.enc_some]; exact wt_someV (h a)
theorem wt_encOption {α : Type} {e : α → Value} {t : Ty} (h : ∀ x, WT p c A (e x) t) :
    ∀ o : Option α, WT p c A (encOption e o) (.option t)
  | none => by simp [encOption, WT]
  | some a => by simp only [encOption, WT]; exact h a
theorem wt_encPair {α β : Type} {ea : α → Value} {eb : β → Value} {s t : Ty}
    (ha : ∀ x, WT p c A (ea x) s) (hb : ∀ x, WT p c A (eb x) t) (x : α × β) :
    WT p c A (encPair ea eb x) (.pair s t) := by
  simp only [encPair, WT]; exact ⟨ha x.1, hb x.2⟩
theorem wt_encExcept {ε α : Type} {ee : ε → Value} {ea : α → Value} {s t : Ty}
    (he : ∀ x, WT p c A (ee x) t) (ha : ∀ x, WT p c A (ea x) s) :
    ∀ x : Except ε α, WT p c A (encExcept ee ea x) (.result s t)
  | .ok a => by simp only [encExcept, WT]; exact ha a
  | .error e => by simp only [encExcept, WT]; exact he e
end wt

/-! The arguments a Rust caller can pass: a natural number is a `u64` and an
integer an `i64`, wherever it occurs in the argument. Certificate E quantifies
over these only. -/

mutual
def Representable : Value → Prop
  | .nat n => n < 18446744073709551616
  | .int i => -9223372036854775808 ≤ i ∧ i ≤ 9223372036854775807
  | .some v => Representable v
  | .ok v => Representable v
  | .error v => Representable v
  | .list vs => RepresentableL vs
  | .pair a b => Representable a ∧ Representable b
  | .adt _ vs => RepresentableL vs
  | .closure _ vs => RepresentableL vs
  | _ => True
def RepresentableL : List Value → Prop
  | [] => True
  | v :: vs => Representable v ∧ RepresentableL vs
end

theorem repL_nil : RepresentableL [] := trivial
theorem repL_cons {v : Value} {vs : List Value} (hv : Representable v) (hs : RepresentableL vs) :
    RepresentableL (v :: vs) := ⟨hv, hs⟩

/-- The machine aborts only in an item that cannot fail: an item whose Rust
function returns `R<T>` reports an overflow as `Err(Overflow)`. The suite
checks that the only item of the table that can overflow while infallible is
a length (SPEC.md §17.17). -/
theorem runItem_abort_infallible (profile : LexLeanTarget.RustSyntax.Profile)
    (item : LexLeanTarget.RustSyntax.Item) (values : List Value)
    (h : RustSemantics.runItem profile item values = .abort) :
    RustSemantics.itemFallible item = false := by
  cases hf : RustSemantics.itemFallible item
  · rfl
  · exfalso
    unfold RustSemantics.runItem at h
    simp only [hf, if_true] at h
    repeat split at h
    all_goals (first | contradiction | simp at h | skip)
    all_goals
      generalize TargetSemantics.primitive (RustSemantics.itemPrimitive item)
        (RustSemantics.itemOperands item values) = r at h
      cases r <;> simp at h

end LexLeanPreservation.Rust
