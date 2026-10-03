import LexLeanPreservation.Keys
set_option linter.unusedSimpArgs false
namespace LexLeanPreservation
open LexLeanTarget.TargetSyntax LexLeanTarget.TargetSemantics

/-! The collection templates: each calculus template function transcribed,
and one theorem per template relating its instance to the source collection
operation, given the defining clauses of that operation's runtime functions. -/

theorem ListEnc.enc_nil {α : Type} (L : ListEnc α) : L.enc [] = .list [] := by
  rw [ListEnc.enc, L.items_nil]
theorem ListEnc.enc_cons {α : Type} (L : ListEnc α) (x : α) (xs : List α) :
    L.enc (x :: xs) = .list (L.elem x :: L.items xs) := by
  rw [ListEnc.enc, L.items_cons]

/-! ## The collection templates, transcribed

Each definition is the Lean transcription of one function of the calculus
collection library (`calculus/library.rs`), placed at index `this`; a
certificate discharges `LexLeanRuntime.index p.functions fi = some (…)` by
`rfl`, so a transcription that drifts from the builder fails the kernel. -/
namespace Tpl

def lst (t : Ty) : Ty := .list t
def pr (a b : Ty) : Ty := .pair a b
def v (n : Nat) : Expr := .var n
def cns (t : Ty) (h tl : Expr) : Expr := .build .cons t [h, tl]
def nl (t : Ty) : Expr := .build .nil t []
def boolE (b : Bool) : Expr := .build (if b then .true else .false) .bool []
def byOrder (r : Ty) (k o less same more : Expr) : Expr :=
  .match r (.prim .compare [k, o]) [.arm .lt [] less, .arm .eq [] same, .arm .gt [] more]
def fn (types : List Ty) (result : Ty) (body : Expr) : Function :=
  { parameters := (List.range types.length), types := types, result := result, body := body }
def natOne : Expr := .value .nat (.nat 1)

def mapInsertFn (this : Nat) (k w : Ty) : Function :=
  let entries := lst (pr k w)
  let newEntry := Expr.build .pair (pr k w) [v 1, v 2]
  fn [entries, k, w] entries
    (.match entries (v 0)
      [.arm .nil [] (cns entries newEntry (nl entries)),
       .arm .cons [3, 4] (byOrder entries (v 1) (.first (v 3))
          (cns entries newEntry (v 0))
          (cns entries newEntry (v 4))
          (cns entries (v 3) (.call this [v 4, v 1, v 2])))])

def mapRemoveFn (this : Nat) (k w : Ty) : Function :=
  let entries := lst (pr k w)
  fn [entries, k] entries
    (.match entries (v 0)
      [.arm .nil [] (nl entries),
       .arm .cons [2, 3] (byOrder entries (v 1) (.first (v 2))
          (v 0) (v 3) (cns entries (v 2) (.call this [v 3, v 1])))])

def mapLookupFn (this : Nat) (k w : Ty) : Function :=
  let found := Ty.option w
  fn [lst (pr k w), k] found
    (.match found (v 0)
      [.arm .nil [] (.build .none found []),
       .arm .cons [2, 3] (byOrder found (v 1) (.first (v 2))
          (.build .none found []) (.build .some found [.second (v 2)])
          (.call this [v 3, v 1]))])

def mapContainsFn (lookup : Nat) (k w : Ty) : Function :=
  fn [lst (pr k w), k] .bool
    (.match .bool (.call lookup [v 0, v 1])
      [.arm .none [] (boolE false), .arm .some [2] (boolE true)])

def mapProjectFn (this : Nat) (k w : Ty) (keys : Bool) : Function :=
  let result := lst (if keys then k else w)
  fn [lst (pr k w)] result
    (.match result (v 0)
      [.arm .nil [] (nl result),
       .arm .cons [1, 2] (cns result (if keys then .first (v 1) else .second (v 1)) (.call this [v 2]))])

def mapFoldFn (this : Nat) (k w s : Ty) : Function :=
  fn [.fn [s, k, w] s, s, lst (pr k w)] s
    (.match s (v 2)
      [.arm .nil [] (v 1),
       .arm .cons [3, 4] (.call this [v 0, .apply (v 0) [v 1, .first (v 3), .second (v 3)], v 4])])

def setInsertFn (this : Nat) (k : Ty) : Function :=
  fn [lst k, k] (lst k)
    (.match (lst k) (v 0)
      [.arm .nil [] (cns (lst k) (v 1) (nl (lst k))),
       .arm .cons [2, 3] (byOrder (lst k) (v 1) (v 2)
          (cns (lst k) (v 1) (v 0)) (v 0) (cns (lst k) (v 2) (.call this [v 3, v 1])))])

def setRemoveFn (this : Nat) (k : Ty) : Function :=
  fn [lst k, k] (lst k)
    (.match (lst k) (v 0)
      [.arm .nil [] (nl (lst k)),
       .arm .cons [2, 3] (byOrder (lst k) (v 1) (v 2)
          (v 0) (v 3) (cns (lst k) (v 2) (.call this [v 3, v 1])))])

def setContainsFn (this : Nat) (k : Ty) : Function :=
  fn [lst k, k] .bool
    (.match .bool (v 0)
      [.arm .nil [] (boolE false),
       .arm .cons [2, 3] (byOrder .bool (v 1) (v 2)
          (boolE false) (boolE true) (.call this [v 3, v 1]))])

def setUnionFn (this insert : Nat) (k : Ty) : Function :=
  fn [lst k, lst k] (lst k)
    (.match (lst k) (v 1)
      [.arm .nil [] (v 0),
       .arm .cons [2, 3] (.call this [.call insert [v 0, v 2], v 3])])

def setFilterFn (this contains : Nat) (k : Ty) (keep : Bool) : Function :=
  let rest := Expr.call this [v 3, v 1]
  let kept := cns (lst k) (v 2) rest
  fn [lst k, lst k] (lst k)
    (.match (lst k) (v 0)
      [.arm .nil [] (nl (lst k)),
       .arm .cons [2, 3] (.cond (.call contains [v 1, v 2])
          (if keep then kept else rest) (if keep then rest else kept))])

def listFoldFn (this : Nat) (e s : Ty) : Function :=
  fn [.fn [s, e] s, s, lst e] s
    (.match s (v 2)
      [.arm .nil [] (v 1),
       .arm .cons [3, 4] (.call this [v 0, .apply (v 0) [v 1, v 3], v 4])])

def iterateFn (this : Nat) (s : Ty) : Function :=
  fn [.fn [s] s, .nat, s] s
    (.match s (v 1)
      [.arm .zero [] (v 2),
       .arm .succ [3] (.call this [v 0, v 3, .apply (v 0) [v 2]])])

def iterateUntilFn (this : Nat) (s : Ty) : Function :=
  let result := pr s .bool
  let finished := fun (reached : Bool) => Expr.build .pair result [v 2, boolE reached]
  fn [.fn [s] (.option s), .nat, s] result
    (.match result (v 1)
      [.arm .zero [] (finished false),
       .arm .succ [3] (.match result (.apply (v 0) [v 2])
          [.arm .none [] (finished true),
           .arm .some [4] (.call this [v 0, v 3, v 4])])])

def graphSuccessorsFn (lookup : Nat) (k : Ty) : Function :=
  fn [lst (pr k (lst k)), k] (lst k)
    (.match (lst k) (.call lookup [v 0, v 1])
      [.arm .none [] (nl (lst k)), .arm .some [2] (v 2)])

def graphNodesFn (this inner insert : Nat) (k : Ty) : Function :=
  fn [lst (pr k (lst k)), lst k] (lst k)
    (.match (lst k) (v 0)
      [.arm .nil [] (v 1),
       .arm .cons [2, 3] (.call this [v 3, .call inner [.second (v 2), .call insert [v 1, .first (v 2)]]])])

def graphInnerFn (this insert : Nat) (k : Ty) : Function :=
  fn [lst k, lst k] (lst k)
    (.match (lst k) (v 0)
      [.arm .nil [] (v 1),
       .arm .cons [2, 3] (.call this [v 3, .call insert [v 1, v 2]])])

def graphReachableFn (rounds allNodes : Nat) (k : Ty) : Function :=
  let single := cns (lst k) (v 1) (nl (lst k))
  fn [lst (pr k (lst k)), k] (lst k)
    (.call rounds [v 0, .prim .natAdd [.prim .length [.call allNodes [v 0, nl (lst k)]], natOne], single, single])

def reachRoundsFn (this frontier union : Nat) (k : Ty) : Function :=
  fn [lst (pr k (lst k)), .nat, lst k, lst k] (lst k)
    (.match (lst k) (v 1)
      [.arm .zero [] (v 3),
       .arm .succ [4] (.let 5 (lst k) (.call frontier [v 0, v 3, v 2, nl (lst k)])
          (.match (lst k) (v 5)
            [.arm .nil [] (v 3),
             .arm .cons [6, 7] (.call this [v 0, v 4, v 5, .call union [v 3, v 5]])]))])

def reachFrontierFn (this into successors : Nat) (k : Ty) : Function :=
  fn [lst (pr k (lst k)), lst k, lst k, lst k] (lst k)
    (.match (lst k) (v 2)
      [.arm .nil [] (v 3),
       .arm .cons [4, 5] (.call this [v 0, v 1, v 5, .call into [v 1, .call successors [v 0, v 4], v 3]])])

def reachIntoFn (this contains insert : Nat) (k : Ty) : Function :=
  fn [lst k, lst k, lst k] (lst k)
    (.match (lst k) (v 1)
      [.arm .nil [] (v 2),
       .arm .cons [3, 4] (.call this [v 0, v 4,
          .cond (.call contains [v 0, v 3]) (v 2)
            (.cond (.call contains [v 2, v 3]) (v 2) (.call insert [v 2, v 3]))])])

def graphTopologicalFn (rounds allNodes : Nat) (k : Ty) : Function :=
  fn [lst (pr k (lst k))] (.option (lst k))
    (.let 1 (lst k) (.call allNodes [v 0, nl (lst k)])
      (.call rounds [v 0, .prim .natAdd [.prim .length [v 1], natOne], v 1, nl (lst k)]))

def topoFinished (reverse first : Nat) (k : Ty) : Expr :=
  .match (.option (lst k)) (v 2)
    [.arm .nil [] (.build .some (.option (lst k)) [.call reverse [v 3, nl (lst k)]]),
     .arm .cons [first, first + 1] (.build .none (.option (lst k)) [])]

def topoRoundsFn (this ready remove reverse : Nat) (k : Ty) : Function :=
  fn [lst (pr k (lst k)), .nat, lst k, lst k] (.option (lst k))
    (.match (.option (lst k)) (v 1)
      [.arm .zero [] (topoFinished reverse 4 k),
       .arm .succ [6] (.match (.option (lst k)) (.call ready [v 0, v 2, v 2])
          [.arm .nil [] (topoFinished reverse 7 k),
           .arm .cons [9, 10] (.call this [v 0, v 6, .call remove [v 2, v 9], cns (lst k) (v 9) (v 3)])])])

def topoReadyFn (this unreached : Nat) (k : Ty) : Function :=
  let rest := Expr.call this [v 0, v 1, v 4]
  fn [lst (pr k (lst k)), lst k, lst k] (lst k)
    (.match (lst k) (v 2)
      [.arm .nil [] (nl (lst k)),
       .arm .cons [3, 4] (.cond (.call unreached [v 0, v 3, v 1]) (cns (lst k) (v 3) rest) rest)])

def topoUnreachedFn (this contains successors : Nat) (k : Ty) : Function :=
  fn [lst (pr k (lst k)), k, lst k] .bool
    (.match .bool (v 2)
      [.arm .nil [] (boolE true),
       .arm .cons [3, 4] (.cond (.call contains [.call successors [v 0, v 3], v 1])
          (boolE false) (.call this [v 0, v 1, v 4]))])

def reverseFn (this : Nat) (k : Ty) : Function :=
  fn [lst k, lst k] (lst k)
    (.match (lst k) (v 0)
      [.arm .nil [] (v 1),
       .arm .cons [2, 3] (.call this [v 3, cns (lst k) (v 2) (v 1)])])

end Tpl
open Tpl

/-- A comparison whose source order is known. -/
theorem prim_compare_at {κ : Type} {enc : κ → Value} {cmp : κ → κ → Ordering}
    (K : KeySpec enc cmp) {a b : κ} {o : Ordering} (h : cmp a b = o) :
    obs (primitive .compare [enc a, enc b]) = some (Rel true (.ordering (ordOf o))) := by
  rw [prim_compare_key K, h]

theorem tpl_mapInsert {κ ν : Type} {ek : κ → Value} {cmp : κ → κ → Ordering} (K : KeySpec ek cmp)
    (ev : ν → Value) (L : ListEnc (κ × ν)) (hL : ∀ e, L.elem e = .pair (ek e.1) (ev e.2))
    (ins : κ → ν → List (κ × ν) → List (κ × ν))
    (i0 : ∀ k w, ins k w [] = [(k, w)])
    (ilt : ∀ k w o s r, cmp k o = .lt → ins k w ((o, s) :: r) = (k, w) :: (o, s) :: r)
    (ieq : ∀ k w o s r, cmp k o = .eq → ins k w ((o, s) :: r) = (k, w) :: r)
    (igt : ∀ k w o s r, cmp k o = .gt → ins k w ((o, s) :: r) = (o, s) :: ins k w r)
    {p : Program} {fi : Nat} {tk tw : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (mapInsertFn fi tk tw)) :
    ∀ m k w, FunRel p fi [L.enc m, ek k, ev w] (Rel true (L.enc (ins k w m)))
  | [], k, w => by
    rw [i0, L.enc_nil, L.enc_cons, L.items_nil, hL]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl
      (conv_build (convL_cons (conv_build (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil))
        construct_pair) (convL_cons (conv_build convL_nil construct_nil) convL_nil)) construct_cons)))
  | (o, s) :: r, k, w => by
    have hm := L.enc_cons (o, s) r
    rw [hL] at hm
    rw [hm]
    cases h : cmp k o
    · rw [ilt k w o s r h, L.enc_cons, L.items_cons, hL, hL]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_match (conv_prim (convL_cons (conv_var rfl) (convL_cons (conv_first (conv_var rfl)) convL_nil))
          (prim_compare_at K h))
          (convA_hit rfl rfl (conv_build (convL_cons (conv_build (convL_cons (conv_var rfl)
            (convL_cons (conv_var rfl) convL_nil)) construct_pair) (convL_cons (conv_var rfl) convL_nil))
            construct_cons))))))
    · rw [ieq k w o s r h, L.enc_cons, hL]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_match (conv_prim (convL_cons (conv_var rfl) (convL_cons (conv_first (conv_var rfl)) convL_nil))
          (prim_compare_at K h))
          (convA_miss rfl (convA_hit rfl rfl (conv_build (convL_cons (conv_build (convL_cons (conv_var rfl)
            (convL_cons (conv_var rfl) convL_nil)) construct_pair) (convL_cons (conv_var rfl) convL_nil))
            construct_cons)))))))
    · rw [igt k w o s r h, L.enc_cons, hL]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_match (conv_prim (convL_cons (conv_var rfl) (convL_cons (conv_first (conv_var rfl)) convL_nil))
          (prim_compare_at K h))
          (convA_miss rfl (convA_miss rfl (convA_hit rfl rfl (conv_build (convL_cons (conv_var rfl)
            (convL_cons (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil)))
              (tpl_mapInsert K ev L hL ins i0 ilt ieq igt hf r k w)) convL_nil))
            construct_cons))))))))

/-- The cons case of an entry list, with the entry's pair exposed. -/
theorem enc_entry {κ ν : Type} {ek : κ → Value} {ev : ν → Value} (L : ListEnc (κ × ν))
    (hL : ∀ e, L.elem e = .pair (ek e.1) (ev e.2)) (o : κ) (s : ν) (r : List (κ × ν)) :
    L.enc ((o, s) :: r) = .list (.pair (ek o) (ev s) :: L.items r) := by
  rw [L.enc_cons, hL]

theorem enc_elem {κ : Type} {ek : κ → Value} (L : ListEnc κ) (hL : ∀ e, L.elem e = ek e)
    (o : κ) (r : List κ) : L.enc (o :: r) = .list (ek o :: L.items r) := by
  rw [L.enc_cons, hL]

theorem tpl_mapRemove {κ ν : Type} {ek : κ → Value} {cmp : κ → κ → Ordering} (K : KeySpec ek cmp)
    (ev : ν → Value) (L : ListEnc (κ × ν)) (hL : ∀ e, L.elem e = .pair (ek e.1) (ev e.2))
    (rem : κ → List (κ × ν) → List (κ × ν))
    (r0 : ∀ k, rem k [] = [])
    (rlt : ∀ k o s r, cmp k o = .lt → rem k ((o, s) :: r) = (o, s) :: r)
    (req : ∀ k o s r, cmp k o = .eq → rem k ((o, s) :: r) = r)
    (rgt : ∀ k o s r, cmp k o = .gt → rem k ((o, s) :: r) = (o, s) :: rem k r)
    {p : Program} {fi : Nat} {tk tw : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (mapRemoveFn fi tk tw)) :
    ∀ m k, FunRel p fi [L.enc m, ek k] (Rel true (L.enc (rem k m)))
  | [], k => by
    rw [r0, L.enc_nil]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl
      (conv_build convL_nil construct_nil)))
  | (o, s) :: r, k => by
    rw [enc_entry L hL]
    cases h : cmp k o
    · rw [rlt k o s r h, enc_entry L hL]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_match (conv_prim (convL_cons (conv_var rfl) (convL_cons (conv_first (conv_var rfl)) convL_nil))
          (prim_compare_at K h)) (convA_hit rfl rfl (conv_var rfl))))))
    · rw [req k o s r h]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_match (conv_prim (convL_cons (conv_var rfl) (convL_cons (conv_first (conv_var rfl)) convL_nil))
          (prim_compare_at K h)) (convA_miss rfl (convA_hit rfl rfl (conv_var rfl)))))))
    · rw [rgt k o s r h, enc_entry L hL]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_match (conv_prim (convL_cons (conv_var rfl) (convL_cons (conv_first (conv_var rfl)) convL_nil))
          (prim_compare_at K h))
          (convA_miss rfl (convA_miss rfl (convA_hit rfl rfl (conv_build (convL_cons (conv_var rfl)
            (convL_cons (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil))
              (tpl_mapRemove K ev L hL rem r0 rlt req rgt hf r k)) convL_nil))
            construct_cons))))))))

theorem tpl_mapLookup {κ ν : Type} {ek : κ → Value} {cmp : κ → κ → Ordering} (K : KeySpec ek cmp)
    (ev : ν → Value) (L : ListEnc (κ × ν)) (hL : ∀ e, L.elem e = .pair (ek e.1) (ev e.2))
    (O : OptEnc ν) (hO : ∀ w, O.elem w = ev w)
    (look : κ → List (κ × ν) → Option ν)
    (l0 : ∀ k, look k [] = none)
    (llt : ∀ k o s r, cmp k o = .lt → look k ((o, s) :: r) = none)
    (leq : ∀ k o s r, cmp k o = .eq → look k ((o, s) :: r) = some s)
    (lgt : ∀ k o s r, cmp k o = .gt → look k ((o, s) :: r) = look k r)
    {p : Program} {fi : Nat} {tk tw : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (mapLookupFn fi tk tw)) :
    ∀ m k, FunRel p fi [L.enc m, ek k] (Rel true (O.enc (look k m)))
  | [], k => by
    rw [l0, L.enc_nil, O.enc_none]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl
      (conv_build convL_nil construct_none)))
  | (o, s) :: r, k => by
    rw [enc_entry L hL]
    cases h : cmp k o
    · rw [llt k o s r h, O.enc_none]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_match (conv_prim (convL_cons (conv_var rfl) (convL_cons (conv_first (conv_var rfl)) convL_nil))
          (prim_compare_at K h)) (convA_hit rfl rfl (conv_build convL_nil construct_none))))))
    · rw [leq k o s r h, O.enc_some, hO]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_match (conv_prim (convL_cons (conv_var rfl) (convL_cons (conv_first (conv_var rfl)) convL_nil))
          (prim_compare_at K h)) (convA_miss rfl (convA_hit rfl rfl
            (conv_build (convL_cons (conv_second (conv_var rfl)) convL_nil) construct_some)))))))
    · rw [lgt k o s r h]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_match (conv_prim (convL_cons (conv_var rfl) (convL_cons (conv_first (conv_var rfl)) convL_nil))
          (prim_compare_at K h))
          (convA_miss rfl (convA_miss rfl (convA_hit rfl rfl
            (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil))
              (tpl_mapLookup K ev L hL O hO look l0 llt leq lgt hf r k)))))))))

theorem tpl_mapContains {κ ν : Type} {ek : κ → Value} {cmp : κ → κ → Ordering} (K : KeySpec ek cmp)
    (ev : ν → Value) (L : ListEnc (κ × ν)) (hL : ∀ e, L.elem e = .pair (ek e.1) (ev e.2))
    (look : κ → List (κ × ν) → Option ν)
    (l0 : ∀ k, look k [] = none)
    (llt : ∀ k o s r, cmp k o = .lt → look k ((o, s) :: r) = none)
    (leq : ∀ k o s r, cmp k o = .eq → look k ((o, s) :: r) = some s)
    (lgt : ∀ k o s r, cmp k o = .gt → look k ((o, s) :: r) = look k r)
    {p : Program} {fi : Nat} {tk tw : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (mapContainsFn (fi + 1) tk tw))
    (hl : LexLeanRuntime.index p.functions (fi + 1) = some (mapLookupFn (fi + 1) tk tw))
    (m : List (κ × ν)) (k : κ) :
    FunRel p fi [L.enc m, ek k] (Rel true (.bool (look k m).isSome)) := by
  have h := tpl_mapLookup K ev L hL (optEnc ev) (fun _ => rfl) look l0 llt leq lgt hl m k
  cases e : look k m
  · rw [e] at h
    exact funRel_intro hf rfl (conv_match (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil)) h)
      (convA_hit rfl rfl (conv_build convL_nil construct_false)))
  · rw [e] at h
    exact funRel_intro hf rfl (conv_match (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil)) h)
      (convA_miss rfl (convA_hit rfl rfl (conv_build convL_nil construct_true))))

theorem tpl_mapKeys {κ ν : Type} {ek : κ → Value} {ev : ν → Value}
    (L : ListEnc (κ × ν)) (hL : ∀ e, L.elem e = .pair (ek e.1) (ev e.2))
    (R : ListEnc κ) (hR : ∀ x, R.elem x = ek x)
    {p : Program} {fi : Nat} {tk tw : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (mapProjectFn fi tk tw true)) :
    ∀ m, FunRel p fi [L.enc m] (Rel true (R.enc (m.map Prod.fst)))
  | [] => by
    rw [List.map_nil, L.enc_nil, R.enc_nil]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl
      (conv_build convL_nil construct_nil)))
  | (o, s) :: r => by
    rw [enc_entry L hL, List.map_cons, enc_elem R hR]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
      (conv_build (convL_cons (conv_first (conv_var rfl)) (convL_cons (conv_call (convL_cons (conv_var rfl) convL_nil)
        (tpl_mapKeys L hL R hR hf r)) convL_nil)) construct_cons))))

theorem tpl_mapValues {κ ν : Type} {ek : κ → Value} {ev : ν → Value}
    (L : ListEnc (κ × ν)) (hL : ∀ e, L.elem e = .pair (ek e.1) (ev e.2))
    (R : ListEnc ν) (hR : ∀ x, R.elem x = ev x)
    {p : Program} {fi : Nat} {tk tw : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (mapProjectFn fi tk tw false)) :
    ∀ m, FunRel p fi [L.enc m] (Rel true (R.enc (m.map Prod.snd)))
  | [] => by
    rw [List.map_nil, L.enc_nil, R.enc_nil]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl
      (conv_build convL_nil construct_nil)))
  | (o, s) :: r => by
    rw [enc_entry L hL, List.map_cons, enc_elem R hR]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
      (conv_build (convL_cons (conv_second (conv_var rfl)) (convL_cons (conv_call (convL_cons (conv_var rfl) convL_nil)
        (tpl_mapValues L hL R hR hf r)) convL_nil)) construct_cons))))

theorem tpl_setInsert {κ : Type} {ek : κ → Value} {cmp : κ → κ → Ordering} (K : KeySpec ek cmp)
    (L : ListEnc κ) (hL : ∀ e, L.elem e = ek e)
    (ins : κ → List κ → List κ)
    (i0 : ∀ k, ins k [] = [k])
    (ilt : ∀ k o r, cmp k o = .lt → ins k (o :: r) = k :: o :: r)
    (ieq : ∀ k o r, cmp k o = .eq → ins k (o :: r) = o :: r)
    (igt : ∀ k o r, cmp k o = .gt → ins k (o :: r) = o :: ins k r)
    {p : Program} {fi : Nat} {tk : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (setInsertFn fi tk)) :
    ∀ s k, FunRel p fi [L.enc s, ek k] (Rel true (L.enc (ins k s)))
  | [], k => by
    rw [i0, L.enc_nil, enc_elem L hL, L.items_nil]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl
      (conv_build (convL_cons (conv_var rfl) (convL_cons (conv_build convL_nil construct_nil) convL_nil)) construct_cons)))
  | o :: r, k => by
    rw [enc_elem L hL]
    cases h : cmp k o
    · rw [ilt k o r h, enc_elem L hL, L.items_cons, hL]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_match (conv_prim (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil))
          (prim_compare_at K h))
          (convA_hit rfl rfl (conv_build (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil))
            construct_cons))))))
    · rw [ieq k o r h, enc_elem L hL]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_match (conv_prim (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil))
          (prim_compare_at K h)) (convA_miss rfl (convA_hit rfl rfl (conv_var rfl)))))))
    · rw [igt k o r h, enc_elem L hL]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_match (conv_prim (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil))
          (prim_compare_at K h))
          (convA_miss rfl (convA_miss rfl (convA_hit rfl rfl (conv_build (convL_cons (conv_var rfl)
            (convL_cons (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil))
              (tpl_setInsert K L hL ins i0 ilt ieq igt hf r k)) convL_nil))
            construct_cons))))))))

theorem tpl_setRemove {κ : Type} {ek : κ → Value} {cmp : κ → κ → Ordering} (K : KeySpec ek cmp)
    (L : ListEnc κ) (hL : ∀ e, L.elem e = ek e)
    (rem : κ → List κ → List κ)
    (r0 : ∀ k, rem k [] = [])
    (rlt : ∀ k o r, cmp k o = .lt → rem k (o :: r) = o :: r)
    (req : ∀ k o r, cmp k o = .eq → rem k (o :: r) = r)
    (rgt : ∀ k o r, cmp k o = .gt → rem k (o :: r) = o :: rem k r)
    {p : Program} {fi : Nat} {tk : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (setRemoveFn fi tk)) :
    ∀ s k, FunRel p fi [L.enc s, ek k] (Rel true (L.enc (rem k s)))
  | [], k => by
    rw [r0, L.enc_nil]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl
      (conv_build convL_nil construct_nil)))
  | o :: r, k => by
    rw [enc_elem L hL]
    cases h : cmp k o
    · rw [rlt k o r h, enc_elem L hL]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_match (conv_prim (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil))
          (prim_compare_at K h)) (convA_hit rfl rfl (conv_var rfl))))))
    · rw [req k o r h]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_match (conv_prim (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil))
          (prim_compare_at K h)) (convA_miss rfl (convA_hit rfl rfl (conv_var rfl)))))))
    · rw [rgt k o r h, enc_elem L hL]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_match (conv_prim (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil))
          (prim_compare_at K h))
          (convA_miss rfl (convA_miss rfl (convA_hit rfl rfl (conv_build (convL_cons (conv_var rfl)
            (convL_cons (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil))
              (tpl_setRemove K L hL rem r0 rlt req rgt hf r k)) convL_nil))
            construct_cons))))))))

theorem tpl_setContains {κ : Type} {ek : κ → Value} {cmp : κ → κ → Ordering} (K : KeySpec ek cmp)
    (L : ListEnc κ) (hL : ∀ e, L.elem e = ek e)
    (con : κ → List κ → Bool)
    (c0 : ∀ k, con k [] = false)
    (clt : ∀ k o r, cmp k o = .lt → con k (o :: r) = false)
    (ceq : ∀ k o r, cmp k o = .eq → con k (o :: r) = true)
    (cgt : ∀ k o r, cmp k o = .gt → con k (o :: r) = con k r)
    {p : Program} {fi : Nat} {tk : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (setContainsFn fi tk)) :
    ∀ s k, FunRel p fi [L.enc s, ek k] (Rel true (.bool (con k s)))
  | [], k => by
    rw [c0, L.enc_nil]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl
      (conv_build convL_nil construct_false)))
  | o :: r, k => by
    rw [enc_elem L hL]
    cases h : cmp k o
    · rw [clt k o r h]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_match (conv_prim (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil))
          (prim_compare_at K h)) (convA_hit rfl rfl (conv_build convL_nil construct_false))))))
    · rw [ceq k o r h]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_match (conv_prim (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil))
          (prim_compare_at K h)) (convA_miss rfl (convA_hit rfl rfl (conv_build convL_nil construct_true)))))))
    · rw [cgt k o r h]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_match (conv_prim (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil))
          (prim_compare_at K h))
          (convA_miss rfl (convA_miss rfl (convA_hit rfl rfl
            (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil))
              (tpl_setContains K L hL con c0 clt ceq cgt hf r k)))))))))

theorem conv_cond_true {p env c t e f g w} (hc : Conv p env c (Rel f (.bool true)))
    (ht : Conv p env t (Rel g w)) : Conv p env (.cond c t e) (Rel (f && g) w) :=
  conv_cond (fun _ => g) (fun _ => w) true hc (fun _ => ht) (fun h => absurd h (by decide))

theorem conv_cond_false {p env c t e f g w} (hc : Conv p env c (Rel f (.bool false)))
    (he : Conv p env e (Rel g w)) : Conv p env (.cond c t e) (Rel (f && g) w) :=
  conv_cond (fun _ => g) (fun _ => w) false hc (fun h => absurd h (by decide)) (fun _ => he)

theorem Conv.bool_eq {p env c f} {b b' : Bool} (h : Conv p env c (Rel f (.bool b))) (e : b = b') :
    Conv p env c (Rel f (.bool b')) := e ▸ h

theorem Conv.fits_eq {p env c f f' w} (h : Conv p env c (Rel f w)) (e : f = f') :
    Conv p env c (Rel f' w) := e ▸ h

theorem FunRel.fits_eq {p fi args f f' w} (h : FunRel p fi args (Rel f w)) (e : f = f') :
    FunRel p fi args (Rel f' w) := e ▸ h

theorem tpl_setUnion {κ : Type} {ek : κ → Value} {cmp : κ → κ → Ordering} (K : KeySpec ek cmp)
    (L : ListEnc κ) (hL : ∀ e, L.elem e = ek e)
    (ins : κ → List κ → List κ)
    (i0 : ∀ k, ins k [] = [k])
    (ilt : ∀ k o r, cmp k o = .lt → ins k (o :: r) = k :: o :: r)
    (ieq : ∀ k o r, cmp k o = .eq → ins k (o :: r) = o :: r)
    (igt : ∀ k o r, cmp k o = .gt → ins k (o :: r) = o :: ins k r)
    {p : Program} {fi : Nat} {tk : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (setUnionFn fi (fi + 1) tk))
    (hi : LexLeanRuntime.index p.functions (fi + 1) = some (setInsertFn (fi + 1) tk)) :
    ∀ r l, FunRel p fi [L.enc l, L.enc r] (Rel true (L.enc (r.foldl (fun acc key => ins key acc) l)))
  | [], l => by
    rw [L.enc_nil, List.foldl_nil]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl (conv_var rfl)))
  | o :: r, l => by
    rw [enc_elem L hL, List.foldl_cons]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
      (conv_call (convL_cons (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil))
        (tpl_setInsert K L hL ins i0 ilt ieq igt hi l o)) (convL_cons (conv_var rfl) convL_nil))
        (tpl_setUnion K L hL ins i0 ilt ieq igt hf hi r (ins o l))))))

theorem tpl_setIntersection {κ : Type} {ek : κ → Value} {cmp : κ → κ → Ordering} (K : KeySpec ek cmp)
    (L : ListEnc κ) (hL : ∀ e, L.elem e = ek e)
    (con : κ → List κ → Bool)
    (c0 : ∀ k, con k [] = false)
    (clt : ∀ k o r, cmp k o = .lt → con k (o :: r) = false)
    (ceq : ∀ k o r, cmp k o = .eq → con k (o :: r) = true)
    (cgt : ∀ k o r, cmp k o = .gt → con k (o :: r) = con k r)
    {p : Program} {fi : Nat} {tk : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (setFilterFn fi (fi + 1) tk true))
    (hc : LexLeanRuntime.index p.functions (fi + 1) = some (setContainsFn (fi + 1) tk)) :
    ∀ l r, FunRel p fi [L.enc l, L.enc r] (Rel true (L.enc (l.filter (fun key => con key r))))
  | [], r => by
    rw [List.filter_nil, L.enc_nil]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl (conv_build convL_nil construct_nil)))
  | o :: l, r => by
    rw [enc_elem L hL]
    have hcall := tpl_setContains K L hL con c0 clt ceq cgt hc r o
    have ih := tpl_setIntersection K L hL con c0 clt ceq cgt hf hc l r
    cases e : con o r
    · simp only [List.filter_cons, e, Bool.not_false, Bool.not_true, reduceIte]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_cond_false (Conv.bool_eq (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil)) hcall) e)
          (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil)) ih)))))
    · simp only [List.filter_cons, e, Bool.not_false, Bool.not_true, reduceIte]
      rw [enc_elem L hL]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_cond_true (Conv.bool_eq (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil)) hcall) e)
          (conv_build (convL_cons (conv_var rfl) (convL_cons (conv_call (convL_cons (conv_var rfl)
            (convL_cons (conv_var rfl) convL_nil)) ih) convL_nil)) construct_cons)))))

theorem tpl_setDifference {κ : Type} {ek : κ → Value} {cmp : κ → κ → Ordering} (K : KeySpec ek cmp)
    (L : ListEnc κ) (hL : ∀ e, L.elem e = ek e)
    (con : κ → List κ → Bool)
    (c0 : ∀ k, con k [] = false)
    (clt : ∀ k o r, cmp k o = .lt → con k (o :: r) = false)
    (ceq : ∀ k o r, cmp k o = .eq → con k (o :: r) = true)
    (cgt : ∀ k o r, cmp k o = .gt → con k (o :: r) = con k r)
    {p : Program} {fi : Nat} {tk : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (setFilterFn fi (fi + 1) tk false))
    (hc : LexLeanRuntime.index p.functions (fi + 1) = some (setContainsFn (fi + 1) tk)) :
    ∀ l r, FunRel p fi [L.enc l, L.enc r] (Rel true (L.enc (l.filter (fun key => !con key r))))
  | [], r => by
    rw [List.filter_nil, L.enc_nil]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl (conv_build convL_nil construct_nil)))
  | o :: l, r => by
    rw [enc_elem L hL]
    have hcall := tpl_setContains K L hL con c0 clt ceq cgt hc r o
    have ih := tpl_setDifference K L hL con c0 clt ceq cgt hf hc l r
    cases e : con o r
    · simp only [List.filter_cons, e, Bool.not_false, Bool.not_true, reduceIte]
      rw [enc_elem L hL]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_cond_false (Conv.bool_eq (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil)) hcall) e)
          (conv_build (convL_cons (conv_var rfl) (convL_cons (conv_call (convL_cons (conv_var rfl)
            (convL_cons (conv_var rfl) convL_nil)) ih) convL_nil)) construct_cons)))))
    · simp only [List.filter_cons, e, Bool.not_false, Bool.not_true, reduceIte]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_cond_true (Conv.bool_eq (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil)) hcall) e)
          (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil)) ih)))))

/-- Whether every step of a fold fits. -/
def foldFits {σ α : Type} (sf : σ → α → Bool) (step : σ → α → σ) : σ → List α → Bool
  | _, [] => true
  | s, x :: xs => sf s x && foldFits sf step (step s x) xs

theorem tpl_listFold {α σ : Type} {ea : α → Value} {es : σ → Value}
    (L : ListEnc α) (hL : ∀ e, L.elem e = ea e)
    (step : σ → α → σ) (sf : σ → α → Bool) (sfi : Nat) (caps : List Value) {p : Program}
    (hs : ∀ s x, FunRel p sfi (LexLeanRuntime.append caps [es s, ea x]) (Rel (sf s x) (es (step s x))))
    {fi : Nat} {ta ts : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (listFoldFn fi ta ts)) :
    ∀ s xs, FunRel p fi [.closure sfi caps, es s, L.enc xs] (Rel (foldFits sf step s xs) (es (xs.foldl step s)))
  | s, [] => by
    rw [L.enc_nil]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl (conv_var rfl)))
  | s, x :: xs => by
    rw [enc_elem L hL]
    refine FunRel.fits_eq (funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
      (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_apply (conv_var rfl)
        (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil)) (hs s x))
        (convL_cons (conv_var rfl) convL_nil)))
        (tpl_listFold L hL step sf sfi caps hs hf (step s x) xs)))))) ?_
    simp only [foldFits, Bool.true_and, Bool.and_true]

theorem tpl_mapFold {κ ν σ : Type} {ek : κ → Value} {ev : ν → Value} {es : σ → Value}
    (L : ListEnc (κ × ν)) (hL : ∀ e, L.elem e = .pair (ek e.1) (ev e.2))
    (step : σ → κ → ν → σ) (sf : σ → κ → ν → Bool) (sfi : Nat) (caps : List Value) {p : Program}
    (hs : ∀ s k w, FunRel p sfi (LexLeanRuntime.append caps [es s, ek k, ev w]) (Rel (sf s k w) (es (step s k w))))
    {fi : Nat} {tk tw ts : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (mapFoldFn fi tk tw ts)) :
    ∀ s m, FunRel p fi [.closure sfi caps, es s, L.enc m]
      (Rel (foldFits (fun st e => sf st e.1 e.2) (fun st e => step st e.1 e.2) s m)
        (es (m.foldl (fun st e => step st e.1 e.2) s)))
  | s, [] => by
    rw [L.enc_nil]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl (conv_var rfl)))
  | s, (k, w) :: m => by
    rw [enc_entry L hL]
    refine FunRel.fits_eq (funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
      (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_apply (conv_var rfl)
        (convL_cons (conv_var rfl) (convL_cons (conv_first (conv_var rfl)) (convL_cons (conv_second (conv_var rfl)) convL_nil)))
        (hs s k w))
        (convL_cons (conv_var rfl) convL_nil)))
        (tpl_mapFold L hL step sf sfi caps hs hf (step s k w) m)))))) ?_
    simp only [foldFits, Bool.true_and, Bool.and_true]

/-- Whether every step of a bounded iteration fits. -/
def iterateFits {σ : Type} (sf : σ → Bool) (step : σ → σ) : Nat → σ → Bool
  | 0, _ => true
  | n + 1, s => sf s && iterateFits sf step n (step s)

theorem tpl_iterate {σ : Type} {es : σ → Value}
    (step : σ → σ) (sf : σ → Bool) (sfi : Nat) (caps : List Value) {p : Program}
    (hs : ∀ s, FunRel p sfi (LexLeanRuntime.append caps [es s]) (Rel (sf s) (es (step s))))
    (it : Nat → σ → σ) (t0 : ∀ s, it 0 s = s) (t1 : ∀ n s, it (n + 1) s = it n (step s))
    {fi : Nat} {ts : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (iterateFn fi ts)) :
    ∀ n s, FunRel p fi [.closure sfi caps, .nat n, es s] (Rel (iterateFits sf step n s) (es (it n s)))
  | 0, s => by
    rw [t0]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl (conv_var rfl)))
  | n + 1, s => by
    rw [t1]
    refine FunRel.fits_eq (funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
      (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) (convL_cons (conv_apply (conv_var rfl)
        (convL_cons (conv_var rfl) convL_nil) (hs s)) convL_nil)))
        (tpl_iterate step sf sfi caps hs it t0 t1 hf n (step s))))))) ?_
    simp only [iterateFits, Bool.true_and, Bool.and_true]

/-- Whether every step of a fuelled search fits. -/
def untilFits {σ : Type} (sf : σ → Bool) (step : σ → Option σ) : Nat → σ → Bool
  | 0, _ => true
  | n + 1, s => sf s && (match step s with
    | none => true
    | some t => untilFits sf step n t)

theorem tpl_iterateUntil {σ : Type} {es : σ → Value}
    (O : OptEnc σ) (hO : ∀ s, O.elem s = es s)
    (P : σ × Bool → Value) (hP : ∀ x, P x = .pair (es x.1) (.bool x.2))
    (step : σ → Option σ) (sf : σ → Bool) (sfi : Nat) (caps : List Value) {p : Program}
    (hs : ∀ s, FunRel p sfi (LexLeanRuntime.append caps [es s]) (Rel (sf s) (O.enc (step s))))
    (it : Nat → σ → σ × Bool) (u0 : ∀ s, it 0 s = (s, false))
    (un : ∀ n s, step s = none → it (n + 1) s = (s, true))
    (us : ∀ n s t, step s = some t → it (n + 1) s = it n t)
    {fi : Nat} {ts : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (iterateUntilFn fi ts)) :
    ∀ n s, FunRel p fi [.closure sfi caps, .nat n, es s] (Rel (untilFits sf step n s) (P (it n s)))
  | 0, s => by
    rw [u0, hP]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl
      (conv_build (convL_cons (conv_var rfl) (convL_cons (conv_build convL_nil construct_false) convL_nil)) construct_pair)))
  | n + 1, s => by
    have h := hs s
    cases e : step s with
    | none =>
      rw [un n s e, hP]
      rw [e, O.enc_none] at h
      refine FunRel.fits_eq (funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_match (conv_apply (conv_var rfl) (convL_cons (conv_var rfl) convL_nil) h)
          (convA_hit rfl rfl (conv_build (convL_cons (conv_var rfl) (convL_cons (conv_build convL_nil construct_true)
            convL_nil)) construct_pair))))))) ?_
      simp only [untilFits, e, Bool.true_and, Bool.and_true]
    | some t =>
      rw [us n s t e]
      rw [e, O.enc_some, hO] at h
      refine FunRel.fits_eq (funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_match (conv_apply (conv_var rfl) (convL_cons (conv_var rfl) convL_nil) h)
          (convA_miss rfl (convA_hit rfl rfl (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl)
            (convL_cons (conv_var rfl) convL_nil)))
            (tpl_iterateUntil O hO P hP step sf sfi caps hs it u0 un us hf n t))))))))) ?_
      simp only [untilFits, e, Bool.true_and, Bool.and_true]

theorem tpl_graphSuccessors {κ : Type} {ek : κ → Value} {cmp : κ → κ → Ordering} (K : KeySpec ek cmp)
    (LK : ListEnc κ) (L : ListEnc (κ × List κ)) (hL : ∀ e, L.elem e = .pair (ek e.1) (LK.enc e.2))
    (look : κ → List (κ × List κ) → Option (List κ))
    (l0 : ∀ k, look k [] = none)
    (llt : ∀ k o s r, cmp k o = .lt → look k ((o, s) :: r) = none)
    (leq : ∀ k o s r, cmp k o = .eq → look k ((o, s) :: r) = some s)
    (lgt : ∀ k o s r, cmp k o = .gt → look k ((o, s) :: r) = look k r)
    {p : Program} {fi : Nat} {tk : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (graphSuccessorsFn (fi + 1) tk))
    (hl : LexLeanRuntime.index p.functions (fi + 1) = some (mapLookupFn (fi + 1) tk (lst tk)))
    (g : List (κ × List κ)) (k : κ) :
    FunRel p fi [L.enc g, ek k] (Rel true (LK.enc ((look k g).getD []))) := by
  have h := tpl_mapLookup K LK.enc L hL (optEnc LK.enc) (fun _ => rfl) look l0 llt leq lgt hl g k
  cases e : look k g
  · rw [e] at h
    rw [Option.getD_none, LK.enc_nil]
    exact funRel_intro hf rfl (conv_match (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil)) h)
      (convA_hit rfl rfl (conv_build convL_nil construct_nil)))
  · rw [e] at h
    rw [Option.getD_some]
    exact funRel_intro hf rfl (conv_match (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil)) h)
      (convA_miss rfl (convA_hit rfl rfl (conv_var rfl))))

theorem tpl_graphInner {κ : Type} {ek : κ → Value} {cmp : κ → κ → Ordering} (K : KeySpec ek cmp)
    (LK : ListEnc κ) (hK : ∀ e, LK.elem e = ek e)
    (ins : κ → List κ → List κ)
    (i0 : ∀ k, ins k [] = [k])
    (ilt : ∀ k o r, cmp k o = .lt → ins k (o :: r) = k :: o :: r)
    (ieq : ∀ k o r, cmp k o = .eq → ins k (o :: r) = o :: r)
    (igt : ∀ k o r, cmp k o = .gt → ins k (o :: r) = o :: ins k r)
    {p : Program} {fi ii : Nat} {tk : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (graphInnerFn fi ii tk))
    (hi : LexLeanRuntime.index p.functions ii = some (setInsertFn ii tk)) :
    ∀ xs acc, FunRel p fi [LK.enc xs, LK.enc acc] (Rel true (LK.enc (xs.foldl (fun inner node => ins node inner) acc)))
  | [], acc => by
    rw [LK.enc_nil, List.foldl_nil]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl (conv_var rfl)))
  | x :: xs, acc => by
    rw [enc_elem LK hK, List.foldl_cons]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
      (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_call (convL_cons (conv_var rfl)
        (convL_cons (conv_var rfl) convL_nil)) (tpl_setInsert K LK hK ins i0 ilt ieq igt hi acc x)) convL_nil))
        (tpl_graphInner K LK hK ins i0 ilt ieq igt hf hi xs (ins x acc))))))

theorem tpl_graphNodes {κ : Type} {ek : κ → Value} {cmp : κ → κ → Ordering} (K : KeySpec ek cmp)
    (LK : ListEnc κ) (hK : ∀ e, LK.elem e = ek e)
    (L : ListEnc (κ × List κ)) (hL : ∀ e, L.elem e = .pair (ek e.1) (LK.enc e.2))
    (ins : κ → List κ → List κ)
    (i0 : ∀ k, ins k [] = [k])
    (ilt : ∀ k o r, cmp k o = .lt → ins k (o :: r) = k :: o :: r)
    (ieq : ∀ k o r, cmp k o = .eq → ins k (o :: r) = o :: r)
    (igt : ∀ k o r, cmp k o = .gt → ins k (o :: r) = o :: ins k r)
    {p : Program} {fi : Nat} {tk : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (graphNodesFn fi (fi + 1) (fi + 2) tk))
    (hn : LexLeanRuntime.index p.functions (fi + 1) = some (graphInnerFn (fi + 1) (fi + 2) tk))
    (hi : LexLeanRuntime.index p.functions (fi + 2) = some (setInsertFn (fi + 2) tk)) :
    ∀ g acc, FunRel p fi [L.enc g, LK.enc acc] (Rel true (LK.enc
      (g.foldl (fun acc entry => entry.2.foldl (fun inner node => ins node inner) (ins entry.1 acc)) acc)))
  | [], acc => by
    rw [L.enc_nil, List.foldl_nil]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl (conv_var rfl)))
  | (o, s) :: g, acc => by
    rw [enc_entry L hL, List.foldl_cons]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
      (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_call (convL_cons (conv_second (conv_var rfl))
        (convL_cons (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_first (conv_var rfl)) convL_nil))
          (tpl_setInsert K LK hK ins i0 ilt ieq igt hi acc o)) convL_nil))
        (tpl_graphInner K LK hK ins i0 ilt ieq igt hn hi s (ins o acc))) convL_nil))
        (tpl_graphNodes K LK hK L hL ins i0 ilt ieq igt hf hn hi g _)))))

/-- The node bound of a graph traversal fits. -/
def graphFits {κ : Type} (ins : κ → List κ → List κ) (g : List (κ × List κ)) : Bool :=
  Nat.blt ((g.foldl (fun acc entry => entry.2.foldl (fun inner node => ins node inner) (ins entry.1 acc)) []).length + 1)
    18446744073709551616

theorem blt_succ_fits (l : Nat) :
    (Nat.blt l 18446744073709551616 && Nat.blt (l + 1) 18446744073709551616) = Nat.blt (l + 1) 18446744073709551616 := by
  cases h : Nat.blt (l + 1) 18446744073709551616
  · simp
  · simp only [Bool.and_true]
    simp only [Nat.blt_eq] at h ⊢
    omega

theorem tpl_reachInto {κ : Type} {ek : κ → Value} {cmp : κ → κ → Ordering} (K : KeySpec ek cmp)
    (LK : ListEnc κ) (hK : ∀ e, LK.elem e = ek e)
    (ins : κ → List κ → List κ)
    (i0 : ∀ k, ins k [] = [k])
    (ilt : ∀ k o r, cmp k o = .lt → ins k (o :: r) = k :: o :: r)
    (ieq : ∀ k o r, cmp k o = .eq → ins k (o :: r) = o :: r)
    (igt : ∀ k o r, cmp k o = .gt → ins k (o :: r) = o :: ins k r)
    (con : κ → List κ → Bool)
    (c0 : ∀ k, con k [] = false)
    (clt : ∀ k o r, cmp k o = .lt → con k (o :: r) = false)
    (ceq : ∀ k o r, cmp k o = .eq → con k (o :: r) = true)
    (cgt : ∀ k o r, cmp k o = .gt → con k (o :: r) = con k r)
    {p : Program} {fi ci ii : Nat} {tk : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (reachIntoFn fi ci ii tk))
    (hc : LexLeanRuntime.index p.functions ci = some (setContainsFn ci tk))
    (hi : LexLeanRuntime.index p.functions ii = some (setInsertFn ii tk)) (seen : List κ) :
    ∀ ys acc, FunRel p fi [LK.enc seen, LK.enc ys, LK.enc acc]
      (Rel true (LK.enc (ys.foldl (fun acc2 y => if con y seen || con y acc2 then acc2 else ins y acc2) acc)))
  | [], acc => by
    rw [LK.enc_nil, List.foldl_nil]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl (conv_var rfl)))
  | y :: ys, acc => by
    rw [enc_elem LK hK, List.foldl_cons]
    have ih := tpl_reachInto K LK hK ins i0 ilt ieq igt con c0 clt ceq cgt hf hc hi seen ys
    have h1 := tpl_setContains K LK hK con c0 clt ceq cgt hc seen y
    have h2 := tpl_setContains K LK hK con c0 clt ceq cgt hc acc y
    cases e1 : con y seen
    · cases e2 : con y acc
      · simp only [Bool.false_or, e2, Bool.false_eq_true, reduceIte]
        exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
          (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) (convL_cons
            (conv_cond_false (Conv.bool_eq (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil)) h1) e1)
              (conv_cond_false (Conv.bool_eq (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil)) h2) e2)
                (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil))
                  (tpl_setInsert K LK hK ins i0 ilt ieq igt hi acc y)))) convL_nil)))
            (ih (ins y acc))))))
      · simp only [Bool.false_or, e2, reduceIte]
        exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
          (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) (convL_cons
            (conv_cond_false (Conv.bool_eq (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil)) h1) e1)
              (conv_cond_true (Conv.bool_eq (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil)) h2) e2)
                (conv_var rfl))) convL_nil)))
            (ih acc)))))
    · simp only [Bool.true_or, reduceIte]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) (convL_cons
          (conv_cond_true (Conv.bool_eq (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil)) h1) e1)
            (conv_var rfl)) convL_nil)))
          (ih acc)))))

/-- The clauses a source key order and its collection runtime satisfy, bundled
for the graph traversals, which use all of them. -/
structure SetOps {κ : Type} (cmp : κ → κ → Ordering) (ins : κ → List κ → List κ)
    (rem : κ → List κ → List κ) (con : κ → List κ → Bool) : Prop where
  i0 : ∀ k, ins k [] = [k]
  ilt : ∀ k o r, cmp k o = .lt → ins k (o :: r) = k :: o :: r
  ieq : ∀ k o r, cmp k o = .eq → ins k (o :: r) = o :: r
  igt : ∀ k o r, cmp k o = .gt → ins k (o :: r) = o :: ins k r
  r0 : ∀ k, rem k [] = []
  rlt : ∀ k o r, cmp k o = .lt → rem k (o :: r) = o :: r
  req : ∀ k o r, cmp k o = .eq → rem k (o :: r) = r
  rgt : ∀ k o r, cmp k o = .gt → rem k (o :: r) = o :: rem k r
  c0 : ∀ k, con k [] = false
  clt : ∀ k o r, cmp k o = .lt → con k (o :: r) = false
  ceq : ∀ k o r, cmp k o = .eq → con k (o :: r) = true
  cgt : ∀ k o r, cmp k o = .gt → con k (o :: r) = con k r

structure LookOps {κ ν : Type} (cmp : κ → κ → Ordering) (look : κ → List (κ × ν) → Option ν) : Prop where
  l0 : ∀ k, look k [] = none
  llt : ∀ k o s r, cmp k o = .lt → look k ((o, s) :: r) = none
  leq : ∀ k o s r, cmp k o = .eq → look k ((o, s) :: r) = some s
  lgt : ∀ k o s r, cmp k o = .gt → look k ((o, s) :: r) = look k r

theorem tpl_reachFrontier {κ : Type} {ek : κ → Value} {cmp : κ → κ → Ordering} (K : KeySpec ek cmp)
    (LK : ListEnc κ) (hK : ∀ e, LK.elem e = ek e)
    (L : ListEnc (κ × List κ)) (hL : ∀ e, L.elem e = .pair (ek e.1) (LK.enc e.2))
    {ins rem : κ → List κ → List κ} {con : κ → List κ → Bool} (S : SetOps cmp ins rem con)
    {look : κ → List (κ × List κ) → Option (List κ)} (Lk : LookOps cmp look)
    (g : List (κ × List κ)) (sx : κ → List κ) (hsx : ∀ k, sx k = (look k g).getD [])
    {p : Program} {fi ii si ci ni : Nat} {tk : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (reachFrontierFn fi ii si tk))
    (hn : LexLeanRuntime.index p.functions ii = some (reachIntoFn ii ci ni tk))
    (hs : LexLeanRuntime.index p.functions si = some (graphSuccessorsFn (si + 1) tk))
    (hl : LexLeanRuntime.index p.functions (si + 1) = some (mapLookupFn (si + 1) tk (lst tk)))
    (hc : LexLeanRuntime.index p.functions ci = some (setContainsFn ci tk))
    (hi : LexLeanRuntime.index p.functions ni = some (setInsertFn ni tk)) (seen : List κ) :
    ∀ f acc, FunRel p fi [L.enc g, LK.enc seen, LK.enc f, LK.enc acc]
      (Rel true (LK.enc (f.foldl (fun acc node =>
        (sx node).foldl (fun acc2 y => if con y seen || con y acc2 then acc2 else ins y acc2) acc) acc)))
  | [], acc => by
    rw [LK.enc_nil, List.foldl_nil]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl (conv_var rfl)))
  | x :: f, acc => by
    rw [enc_elem LK hK, List.foldl_cons]
    have hsucc := tpl_graphSuccessors K LK L hL look Lk.l0 Lk.llt Lk.leq Lk.lgt hs hl g x
    rw [← hsx] at hsucc
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
      (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) (convL_cons
        (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_call (convL_cons (conv_var rfl)
          (convL_cons (conv_var rfl) convL_nil)) hsucc) (convL_cons (conv_var rfl) convL_nil)))
          (tpl_reachInto K LK hK ins S.i0 S.ilt S.ieq S.igt con S.c0 S.clt S.ceq S.cgt hn hc hi seen (sx x) acc))
        convL_nil))))
        (tpl_reachFrontier K LK hK L hL S Lk g sx hsx hf hn hs hl hc hi seen f _)))))

/-- One round's newly reached nodes. -/
def reachNext {κ : Type} (sx : κ → List κ) (con : κ → List κ → Bool) (ins : κ → List κ → List κ)
    (f seen : List κ) : List κ :=
  f.foldl (fun acc node => (sx node).foldl (fun acc2 y => if con y seen || con y acc2 then acc2 else ins y acc2) acc) []

theorem tpl_reachRounds {κ : Type} {ek : κ → Value} {cmp : κ → κ → Ordering} (K : KeySpec ek cmp)
    (LK : ListEnc κ) (hK : ∀ e, LK.elem e = ek e)
    (L : ListEnc (κ × List κ)) (hL : ∀ e, L.elem e = .pair (ek e.1) (LK.enc e.2))
    {ins rem : κ → List κ → List κ} {con : κ → List κ → Bool} (S : SetOps cmp ins rem con)
    {look : κ → List (κ × List κ) → Option (List κ)} (Lk : LookOps cmp look)
    (g : List (κ × List κ)) (sx : κ → List κ) (hsx : ∀ k, sx k = (look k g).getD [])
    (reach : Nat → List κ → List κ → List κ)
    (h0 : ∀ f s, reach 0 f s = s)
    (hn : ∀ n f s, f.foldl (fun acc node => (sx node).foldl
        (fun acc2 y => if con y s || con y acc2 then acc2 else ins y acc2) acc) [] = [] →
      reach (n + 1) f s = s)
    (hc : ∀ n f s x xs, f.foldl (fun acc node => (sx node).foldl
        (fun acc2 y => if con y s || con y acc2 then acc2 else ins y acc2) acc) [] = x :: xs →
      reach (n + 1) f s = reach n (x :: xs) ((x :: xs).foldl (fun acc key => ins key acc) s))
    {p : Program} {fi : Nat} {tk : Ty}
    (hr : LexLeanRuntime.index p.functions (fi + 1) = some (reachRoundsFn (fi + 1) (fi + 2) (fi + 8) tk))
    (hfr : LexLeanRuntime.index p.functions (fi + 2) = some (reachFrontierFn (fi + 2) (fi + 3) (fi + 4) tk))
    (hin : LexLeanRuntime.index p.functions (fi + 3) = some (reachIntoFn (fi + 3) (fi + 6) (fi + 7) tk))
    (hs : LexLeanRuntime.index p.functions (fi + 4) = some (graphSuccessorsFn (fi + 4 + 1) tk))
    (hl : LexLeanRuntime.index p.functions (fi + 4 + 1) = some (mapLookupFn (fi + 4 + 1) tk (lst tk)))
    (hco : LexLeanRuntime.index p.functions (fi + 6) = some (setContainsFn (fi + 6) tk))
    (hi : LexLeanRuntime.index p.functions (fi + 7) = some (setInsertFn (fi + 7) tk))
    (hu : LexLeanRuntime.index p.functions (fi + 8) = some (setUnionFn (fi + 8) (fi + 8 + 1) tk))
    (hui : LexLeanRuntime.index p.functions (fi + 8 + 1) = some (setInsertFn (fi + 8 + 1) tk)) :
    ∀ n f s, FunRel p (fi + 1) [L.enc g, .nat n, LK.enc f, LK.enc s] (Rel true (LK.enc (reach n f s)))
  | 0, f, s => by
    rw [h0]
    exact funRel_intro hr rfl (conv_match (conv_var rfl) (convA_hit rfl rfl (conv_var rfl)))
  | n + 1, f, s => by
    have hnext := tpl_reachFrontier K LK hK L hL S Lk g sx hsx hfr hin hs hl hco hi s f []
    cases e : reachNext sx con ins f s with
    | nil =>
      rw [hn n f s e]
      rw [show f.foldl _ [] = reachNext sx con ins f s from rfl, e, LK.enc_nil] at hnext
      exact funRel_intro hr rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_let (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) (convL_cons (conv_var rfl)
          (convL_cons (conv_build convL_nil construct_nil) convL_nil)))) hnext)
          (conv_match (conv_var rfl) (convA_hit rfl rfl (conv_var rfl)))))))
    | cons x xs =>
      rw [hc n f s x xs e]
      rw [show f.foldl _ [] = reachNext sx con ins f s from rfl, e, LK.enc_nil] at hnext
      have hun := tpl_setUnion K LK hK ins S.i0 S.ilt S.ieq S.igt hu hui (x :: xs) s
      have ih := tpl_reachRounds K LK hK L hL S Lk g sx hsx reach h0 hn hc hr hfr hin hs hl hco hi hu hui n (x :: xs)
        ((x :: xs).foldl (fun acc key => ins key acc) s)
      rw [enc_elem LK hK] at hnext hun ih
      exact funRel_intro hr rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_let (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) (convL_cons (conv_var rfl)
          (convL_cons (conv_build convL_nil construct_nil) convL_nil)))) hnext)
          (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
            (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) (convL_cons (conv_var rfl)
              (convL_cons (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil)) hun) convL_nil))))
              ih))))))))

theorem length_eq {α : Type} (xs : List α) : LexLeanRuntime.length xs = xs.length := rfl

theorem tpl_graphReachable {κ : Type} {ek : κ → Value} {cmp : κ → κ → Ordering} (K : KeySpec ek cmp)
    (LK : ListEnc κ) (hK : ∀ e, LK.elem e = ek e)
    (L : ListEnc (κ × List κ)) (hL : ∀ e, L.elem e = .pair (ek e.1) (LK.enc e.2))
    {ins rem : κ → List κ → List κ} {con : κ → List κ → Bool} (S : SetOps cmp ins rem con)
    {look : κ → List (κ × List κ) → Option (List κ)} (Lk : LookOps cmp look)
    (g : List (κ × List κ)) (sx : κ → List κ) (hsx : ∀ k, sx k = (look k g).getD [])
    (reach : Nat → List κ → List κ → List κ)
    (h0 : ∀ f s, reach 0 f s = s)
    (hn : ∀ n f s, f.foldl (fun acc node => (sx node).foldl
        (fun acc2 y => if con y s || con y acc2 then acc2 else ins y acc2) acc) [] = [] →
      reach (n + 1) f s = s)
    (hc : ∀ n f s x xs, f.foldl (fun acc node => (sx node).foldl
        (fun acc2 y => if con y s || con y acc2 then acc2 else ins y acc2) acc) [] = x :: xs →
      reach (n + 1) f s = reach n (x :: xs) ((x :: xs).foldl (fun acc key => ins key acc) s))
    {p : Program} {fi : Nat} {tk : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (graphReachableFn (fi + 1) (fi + 10) tk))
    (hr : LexLeanRuntime.index p.functions (fi + 1) = some (reachRoundsFn (fi + 1) (fi + 2) (fi + 8) tk))
    (hfr : LexLeanRuntime.index p.functions (fi + 2) = some (reachFrontierFn (fi + 2) (fi + 3) (fi + 4) tk))
    (hin : LexLeanRuntime.index p.functions (fi + 3) = some (reachIntoFn (fi + 3) (fi + 6) (fi + 7) tk))
    (hs : LexLeanRuntime.index p.functions (fi + 4) = some (graphSuccessorsFn (fi + 4 + 1) tk))
    (hl : LexLeanRuntime.index p.functions (fi + 4 + 1) = some (mapLookupFn (fi + 4 + 1) tk (lst tk)))
    (hco : LexLeanRuntime.index p.functions (fi + 6) = some (setContainsFn (fi + 6) tk))
    (hi : LexLeanRuntime.index p.functions (fi + 7) = some (setInsertFn (fi + 7) tk))
    (hu : LexLeanRuntime.index p.functions (fi + 8) = some (setUnionFn (fi + 8) (fi + 8 + 1) tk))
    (hui : LexLeanRuntime.index p.functions (fi + 8 + 1) = some (setInsertFn (fi + 8 + 1) tk))
    (hno : LexLeanRuntime.index p.functions (fi + 10) = some (graphNodesFn (fi + 10) (fi + 10 + 1) (fi + 10 + 2) tk))
    (hni : LexLeanRuntime.index p.functions (fi + 10 + 1) = some (graphInnerFn (fi + 10 + 1) (fi + 10 + 2) tk))
    (hnn : LexLeanRuntime.index p.functions (fi + 10 + 2) = some (setInsertFn (fi + 10 + 2) tk))
    (start : κ) :
    FunRel p fi [L.enc g, ek start] (Rel (graphFits ins g) (LK.enc (reach
      ((g.foldl (fun acc entry => entry.2.foldl (fun inner node => ins node inner) (ins entry.1 acc)) []).length + 1)
      [start] [start]))) := by
  have hnodes := tpl_graphNodes K LK hK L hL ins S.i0 S.ilt S.ieq S.igt hno hni hnn g []
  rw [LK.enc_nil] at hnodes
  have hround := tpl_reachRounds K LK hK L hL S Lk g sx hsx reach h0 hn hc hr hfr hin hs hl hco hi hu hui
    ((g.foldl (fun acc entry => entry.2.foldl (fun inner node => ins node inner) (ins entry.1 acc)) []).length + 1)
    [start] [start]
  rw [enc_elem LK hK, LK.items_nil] at hround
  refine FunRel.fits_eq (funRel_intro hf rfl (conv_call (convL_cons (conv_var rfl) (convL_cons
    (conv_prim (convL_cons (conv_prim (convL_cons (conv_call (convL_cons (conv_var rfl)
      (convL_cons (conv_build convL_nil construct_nil) convL_nil)) hnodes) convL_nil) (prim_length_list LK _))
      (convL_cons conv_value convL_nil)) (prim_natAdd _ 1))
    (convL_cons (conv_build (convL_cons (conv_var rfl) (convL_cons (conv_build convL_nil construct_nil) convL_nil)) construct_cons)
      (convL_cons (conv_build (convL_cons (conv_var rfl) (convL_cons (conv_build convL_nil construct_nil) convL_nil)) construct_cons)
        convL_nil)))) hround)) ?_
  simp only [graphFits, length_eq, Bool.true_and, Bool.and_true]
  exact blt_succ_fits _

theorem tpl_reverse {κ : Type} {ek : κ → Value} (LK : ListEnc κ) (hK : ∀ e, LK.elem e = ek e)
    {p : Program} {fi : Nat} {tk : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (reverseFn fi tk)) :
    ∀ xs acc, FunRel p fi [LK.enc xs, LK.enc acc] (Rel true (LK.enc (List.reverseAux xs acc)))
  | [], acc => by
    rw [LK.enc_nil, List.reverseAux_nil]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl (conv_var rfl)))
  | x :: xs, acc => by
    rw [enc_elem LK hK, List.reverseAux_cons]
    have ih := tpl_reverse LK hK hf xs (x :: acc)
    rw [enc_elem LK hK] at ih
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
      (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_build (convL_cons (conv_var rfl)
        (convL_cons (conv_var rfl) convL_nil)) construct_cons) convL_nil)) ih))))

theorem tpl_topoUnreached {κ : Type} {ek : κ → Value} {cmp : κ → κ → Ordering} (K : KeySpec ek cmp)
    (LK : ListEnc κ) (hK : ∀ e, LK.elem e = ek e)
    (L : ListEnc (κ × List κ)) (hL : ∀ e, L.elem e = .pair (ek e.1) (LK.enc e.2))
    {ins rem : κ → List κ → List κ} {con : κ → List κ → Bool} (S : SetOps cmp ins rem con)
    {look : κ → List (κ × List κ) → Option (List κ)} (Lk : LookOps cmp look)
    (g : List (κ × List κ)) (sx : κ → List κ) (hsx : ∀ k, sx k = (look k g).getD [])
    {p : Program} {fi ci si : Nat} {tk : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (topoUnreachedFn fi ci si tk))
    (hc : LexLeanRuntime.index p.functions ci = some (setContainsFn ci tk))
    (hs : LexLeanRuntime.index p.functions si = some (graphSuccessorsFn (si + 1) tk))
    (hl : LexLeanRuntime.index p.functions (si + 1) = some (mapLookupFn (si + 1) tk (lst tk))) (node : κ) :
    ∀ others, FunRel p fi [L.enc g, ek node, LK.enc others]
      (Rel true (.bool (others.all (fun other => !con node (sx other)))))
  | [] => by
    rw [LK.enc_nil, List.all_nil]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl (conv_build convL_nil construct_true)))
  | o :: os => by
    rw [enc_elem LK hK, List.all_cons]
    have hsucc := tpl_graphSuccessors K LK L hL look Lk.l0 Lk.llt Lk.leq Lk.lgt hs hl g o
    rw [← hsx] at hsucc
    have hcon := tpl_setContains K LK hK con S.c0 S.clt S.ceq S.cgt hc (sx o) node
    have ih := tpl_topoUnreached K LK hK L hL S Lk g sx hsx hf hc hs hl node os
    cases e : con node (sx o)
    · rw [Bool.not_false, Bool.true_and]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_cond_false (Conv.bool_eq (conv_call (convL_cons (conv_call (convL_cons (conv_var rfl)
          (convL_cons (conv_var rfl) convL_nil)) hsucc) (convL_cons (conv_var rfl) convL_nil)) hcon) e)
          (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil))) ih)))))
    · rw [Bool.not_true, Bool.false_and]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_cond_true (Conv.bool_eq (conv_call (convL_cons (conv_call (convL_cons (conv_var rfl)
          (convL_cons (conv_var rfl) convL_nil)) hsucc) (convL_cons (conv_var rfl) convL_nil)) hcon) e)
          (conv_build convL_nil construct_false)))))

theorem tpl_topoReady {κ : Type} {ek : κ → Value} {cmp : κ → κ → Ordering} (K : KeySpec ek cmp)
    (LK : ListEnc κ) (hK : ∀ e, LK.elem e = ek e)
    (L : ListEnc (κ × List κ)) (hL : ∀ e, L.elem e = .pair (ek e.1) (LK.enc e.2))
    {ins rem : κ → List κ → List κ} {con : κ → List κ → Bool} (S : SetOps cmp ins rem con)
    {look : κ → List (κ × List κ) → Option (List κ)} (Lk : LookOps cmp look)
    (g : List (κ × List κ)) (sx : κ → List κ) (hsx : ∀ k, sx k = (look k g).getD [])
    {p : Program} {fi ui ci si : Nat} {tk : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (topoReadyFn fi ui tk))
    (hu : LexLeanRuntime.index p.functions ui = some (topoUnreachedFn ui ci si tk))
    (hc : LexLeanRuntime.index p.functions ci = some (setContainsFn ci tk))
    (hs : LexLeanRuntime.index p.functions si = some (graphSuccessorsFn (si + 1) tk))
    (hl : LexLeanRuntime.index p.functions (si + 1) = some (mapLookupFn (si + 1) tk (lst tk)))
    (remaining : List κ) :
    ∀ cands, FunRel p fi [L.enc g, LK.enc remaining, LK.enc cands]
      (Rel true (LK.enc (cands.filter (fun node => remaining.all (fun other => !con node (sx other))))))
  | [] => by
    rw [List.filter_nil, LK.enc_nil]
    exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_hit rfl rfl (conv_build convL_nil construct_nil)))
  | x :: xs => by
    rw [enc_elem LK hK]
    have hun := tpl_topoUnreached K LK hK L hL S Lk g sx hsx hu hc hs hl x remaining
    have ih := tpl_topoReady K LK hK L hL S Lk g sx hsx hf hu hc hs hl remaining xs
    cases e : remaining.all (fun other => !con x (sx other))
    · simp only [List.filter_cons, e, Bool.false_eq_true, reduceIte]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_cond_false (Conv.bool_eq (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl)
          (convL_cons (conv_var rfl) convL_nil))) hun) e)
          (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil))) ih)))))
    · simp only [List.filter_cons, e, reduceIte]
      rw [enc_elem LK hK]
      exact funRel_intro hf rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_cond_true (Conv.bool_eq (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl)
          (convL_cons (conv_var rfl) convL_nil))) hun) e)
          (conv_build (convL_cons (conv_var rfl) (convL_cons (conv_call (convL_cons (conv_var rfl)
            (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil))) ih) convL_nil)) construct_cons)))))

/-- The defining clauses of a source topological ordering by rounds. -/
structure TopoOps {κ : Type} (sx : κ → List κ) (con : κ → List κ → Bool) (rem : κ → List κ → List κ)
    (topo : Nat → List κ → List κ → Option (List κ)) : Prop where
  z_nil : ∀ ord, topo 0 [] ord = some ord.reverse
  z_cons : ∀ x xs ord, topo 0 (x :: xs) ord = none
  s_nil : ∀ n ord, topo (n + 1) [] ord = some ord.reverse
  s_stuck : ∀ n x xs ord,
    (x :: xs).filter (fun node => (x :: xs).all (fun other => !con node (sx other))) = [] →
    topo (n + 1) (x :: xs) ord = none
  s_next : ∀ n remaining ord r rs,
    remaining.filter (fun node => remaining.all (fun other => !con node (sx other))) = r :: rs →
    topo (n + 1) remaining ord = topo n (rem r remaining) (r :: ord)

theorem tpl_topoRounds {κ : Type} {ek : κ → Value} {cmp : κ → κ → Ordering} (K : KeySpec ek cmp)
    (LK : ListEnc κ) (hK : ∀ e, LK.elem e = ek e)
    (L : ListEnc (κ × List κ)) (hL : ∀ e, L.elem e = .pair (ek e.1) (LK.enc e.2))
    (OL : OptEnc (List κ)) (hOL : ∀ x, OL.elem x = LK.enc x)
    {ins rem : κ → List κ → List κ} {con : κ → List κ → Bool} (S : SetOps cmp ins rem con)
    {look : κ → List (κ × List κ) → Option (List κ)} (Lk : LookOps cmp look)
    (g : List (κ × List κ)) (sx : κ → List κ) (hsx : ∀ k, sx k = (look k g).getD [])
    {topo : Nat → List κ → List κ → Option (List κ)} (T : TopoOps sx con rem topo)
    {p : Program} {fi : Nat} {tk : Ty}
    (hr : LexLeanRuntime.index p.functions (fi + 1) = some (topoRoundsFn (fi + 1) (fi + 2) (fi + 11) (fi + 4) tk))
    (hrd : LexLeanRuntime.index p.functions (fi + 2) = some (topoReadyFn (fi + 2) (fi + 3) tk))
    (hu : LexLeanRuntime.index p.functions (fi + 3) = some (topoUnreachedFn (fi + 3) (fi + 10) (fi + 8) tk))
    (hrv : LexLeanRuntime.index p.functions (fi + 4) = some (reverseFn (fi + 4) tk))
    (hs : LexLeanRuntime.index p.functions (fi + 8) = some (graphSuccessorsFn (fi + 8 + 1) tk))
    (hl : LexLeanRuntime.index p.functions (fi + 8 + 1) = some (mapLookupFn (fi + 8 + 1) tk (lst tk)))
    (hc : LexLeanRuntime.index p.functions (fi + 10) = some (setContainsFn (fi + 10) tk))
    (hm : LexLeanRuntime.index p.functions (fi + 11) = some (setRemoveFn (fi + 11) tk)) :
    ∀ n remaining ord, FunRel p (fi + 1) [L.enc g, .nat n, LK.enc remaining, LK.enc ord]
      (Rel true (OL.enc (topo n remaining ord)))
  | 0, [], ord => by
    rw [T.z_nil, OL.enc_some, hOL, LK.enc_nil]
    have hrev := tpl_reverse LK hK hrv ord []
    rw [LK.enc_nil] at hrev
    exact funRel_intro hr rfl (conv_match (conv_var rfl) (convA_hit rfl rfl
      (conv_match (conv_var rfl) (convA_hit rfl rfl (conv_build (convL_cons (conv_call (convL_cons (conv_var rfl)
        (convL_cons (conv_build convL_nil construct_nil) convL_nil)) hrev) convL_nil) construct_some)))))
  | 0, x :: xs, ord => by
    rw [T.z_cons, OL.enc_none, enc_elem LK hK]
    exact funRel_intro hr rfl (conv_match (conv_var rfl) (convA_hit rfl rfl
      (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl (conv_build convL_nil construct_none))))))
  | n + 1, remaining, ord => by
    have hready := tpl_topoReady K LK hK L hL S Lk g sx hsx hrd hu hc hs hl remaining remaining
    cases e : remaining.filter (fun node => remaining.all (fun other => !con node (sx other))) with
    | nil =>
      rw [e, LK.enc_nil] at hready
      cases remaining with
      | nil =>
        rw [T.s_nil, OL.enc_some, hOL, LK.enc_nil]
        rw [LK.enc_nil] at hready
        have hrev := tpl_reverse LK hK hrv ord []
        rw [LK.enc_nil] at hrev
        exact funRel_intro hr rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
          (conv_match (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) (convL_cons (conv_var rfl)
            convL_nil))) hready)
            (convA_hit rfl rfl (conv_match (conv_var rfl) (convA_hit rfl rfl (conv_build (convL_cons
              (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_build convL_nil construct_nil) convL_nil)) hrev)
              convL_nil) construct_some))))))))
      | cons x xs =>
        rw [T.s_stuck n x xs ord e, OL.enc_none, enc_elem LK hK]
        rw [enc_elem LK hK] at hready
        exact funRel_intro hr rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
          (conv_match (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) (convL_cons (conv_var rfl)
            convL_nil))) hready)
            (convA_hit rfl rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
              (conv_build convL_nil construct_none)))))))))
    | cons r rs =>
      rw [e, enc_elem LK hK] at hready
      rw [T.s_next n remaining ord r rs e]
      have hrm := tpl_setRemove K LK hK rem S.r0 S.rlt S.req S.rgt hm remaining r
      have ih := tpl_topoRounds K LK hK L hL OL hOL S Lk g sx hsx T hr hrd hu hrv hs hl hc hm n (rem r remaining) (r :: ord)
      rw [enc_elem LK hK (o := r) (r := ord)] at ih
      exact funRel_intro hr rfl (conv_match (conv_var rfl) (convA_miss rfl (convA_hit rfl rfl
        (conv_match (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) (convL_cons (conv_var rfl)
          convL_nil))) hready)
          (convA_miss rfl (convA_hit rfl rfl (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl)
            (convL_cons (conv_call (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil)) hrm)
              (convL_cons (conv_build (convL_cons (conv_var rfl) (convL_cons (conv_var rfl) convL_nil)) construct_cons)
                convL_nil)))) ih)))))))

theorem tpl_graphTopological {κ : Type} {ek : κ → Value} {cmp : κ → κ → Ordering} (K : KeySpec ek cmp)
    (LK : ListEnc κ) (hK : ∀ e, LK.elem e = ek e)
    (L : ListEnc (κ × List κ)) (hL : ∀ e, L.elem e = .pair (ek e.1) (LK.enc e.2))
    (OL : OptEnc (List κ)) (hOL : ∀ x, OL.elem x = LK.enc x)
    {ins rem : κ → List κ → List κ} {con : κ → List κ → Bool} (S : SetOps cmp ins rem con)
    {look : κ → List (κ × List κ) → Option (List κ)} (Lk : LookOps cmp look)
    (g : List (κ × List κ)) (sx : κ → List κ) (hsx : ∀ k, sx k = (look k g).getD [])
    {topo : Nat → List κ → List κ → Option (List κ)} (T : TopoOps sx con rem topo)
    {p : Program} {fi : Nat} {tk : Ty}
    (hf : LexLeanRuntime.index p.functions fi = some (graphTopologicalFn (fi + 1) (fi + 5) tk))
    (hr : LexLeanRuntime.index p.functions (fi + 1) = some (topoRoundsFn (fi + 1) (fi + 2) (fi + 11) (fi + 4) tk))
    (hrd : LexLeanRuntime.index p.functions (fi + 2) = some (topoReadyFn (fi + 2) (fi + 3) tk))
    (hu : LexLeanRuntime.index p.functions (fi + 3) = some (topoUnreachedFn (fi + 3) (fi + 10) (fi + 8) tk))
    (hrv : LexLeanRuntime.index p.functions (fi + 4) = some (reverseFn (fi + 4) tk))
    (hno : LexLeanRuntime.index p.functions (fi + 5) = some (graphNodesFn (fi + 5) (fi + 5 + 1) (fi + 5 + 2) tk))
    (hni : LexLeanRuntime.index p.functions (fi + 5 + 1) = some (graphInnerFn (fi + 5 + 1) (fi + 5 + 2) tk))
    (hnn : LexLeanRuntime.index p.functions (fi + 5 + 2) = some (setInsertFn (fi + 5 + 2) tk))
    (hs : LexLeanRuntime.index p.functions (fi + 8) = some (graphSuccessorsFn (fi + 8 + 1) tk))
    (hl : LexLeanRuntime.index p.functions (fi + 8 + 1) = some (mapLookupFn (fi + 8 + 1) tk (lst tk)))
    (hc : LexLeanRuntime.index p.functions (fi + 10) = some (setContainsFn (fi + 10) tk))
    (hm : LexLeanRuntime.index p.functions (fi + 11) = some (setRemoveFn (fi + 11) tk)) :
    FunRel p fi [L.enc g] (Rel (graphFits ins g) (OL.enc (topo
      ((g.foldl (fun acc entry => entry.2.foldl (fun inner node => ins node inner) (ins entry.1 acc)) []).length + 1)
      (g.foldl (fun acc entry => entry.2.foldl (fun inner node => ins node inner) (ins entry.1 acc)) []) []))) := by
  have hnodes := tpl_graphNodes K LK hK L hL ins S.i0 S.ilt S.ieq S.igt hno hni hnn g []
  rw [LK.enc_nil] at hnodes
  have hround := tpl_topoRounds K LK hK L hL OL hOL S Lk g sx hsx T hr hrd hu hrv hs hl hc hm
    ((g.foldl (fun acc entry => entry.2.foldl (fun inner node => ins node inner) (ins entry.1 acc)) []).length + 1)
    (g.foldl (fun acc entry => entry.2.foldl (fun inner node => ins node inner) (ins entry.1 acc)) []) []
  rw [LK.enc_nil] at hround
  refine FunRel.fits_eq (funRel_intro hf rfl (conv_let (conv_call (convL_cons (conv_var rfl)
    (convL_cons (conv_build convL_nil construct_nil) convL_nil)) hnodes)
    (conv_call (convL_cons (conv_var rfl) (convL_cons
      (conv_prim (convL_cons (conv_prim (convL_cons (conv_var rfl) convL_nil) (prim_length_list LK _))
        (convL_cons conv_value convL_nil)) (prim_natAdd _ 1))
      (convL_cons (conv_var rfl) (convL_cons (conv_build convL_nil construct_nil) convL_nil)))) hround))) ?_
  simp only [graphFits, length_eq, Bool.true_and, Bool.and_true]
  exact blt_succ_fits _

end LexLeanPreservation
