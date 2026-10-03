import LexLeanPreservation.RustBase
set_option linter.unusedSimpArgs false

/-! The correspondence between a realization program and its rendered crate
(SPEC.md §17.17): the closed rule set `Corr` certificate B derives, the
decidable side conditions its rules check (a rendering's folds, patterns,
loads, rebuilt arms, empty types), and the semantics `Sem` every judgment
of it is sound for. -/

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

/-! The correspondence between a realization program and its Rust crate. -/

/-- A read of a binding: a move, a clone, a copy, a dereference, or an
unboxing. -/
def readOf : RExpr → Option RIdent
  | .move i | .clone i | .copy i | .deref i | .unbox i => some i
  | _ => none

mutual
/-- The identifiers a pattern binds. -/
def patBinds : RustSyntax.Pat → List RIdent
  | .bind i => [i]
  | .tuple ps => patsBinds ps
  | .some q | .ok q | .err q => patBinds q
  | .adt _ _ ps => patsBinds ps
  | .wild | .unit | .none | .ordering _ => []
def patsBinds : List RustSyntax.Pat → List RIdent
  | [] => []
  | q :: qs => patBinds q ++ patsBinds qs
end

def letsBind : List RLet → List RIdent
  | [] => []
  | .mk q _ _ :: rest => patBinds q ++ letsBind rest

/-- Whether no let of `lets` rebinds `i`. -/
def tempFresh (i : RIdent) (lets : List RLet) : Bool :=
  !(letsBind lets).any fun j => RustSemantics.sameIdent j i

/-- Whether an identifier kind is a temporary's: anything but a local. -/
def isTemp : RustSyntax.IdentKind → Bool
  | .binding => false
  | _ => true

/-- Whether a runtime item hands its primitive its operands unchanged:
every item but the successor. -/
def plainItem : RustSyntax.Item → Bool
  | .natSucc => false
  | _ => true

/-- Whether a runtime item exists in the crate's profile. -/
def itemOK (c : RCrate) (item : RustSyntax.Item) : Bool :=
  match c.profile with
  | .std => true
  | .core => !RustSemantics.itemHeap item

/-- A value of a type's sequence or text shape, as an item's sequence
guard reads it; any other type's is the unit. -/
def shapeOf : Ty → Value
  | .list _ => .list []
  | .bytes => .bytes .empty
  | .string => .string ""
  | .unit | .bool | .nat | .int | .fixed _ | .ordering | .option _ | .result _ _
  | .pair _ _ | .adt _ | .fn _ _ => .unit

/-- Whether operand types meet an item's guard: a sequence or text item's
first operand has its sequence or text, and every operand of an item of a
width, or a shift's shifted operand, has the item's width. -/
def itemTyped (item : RustSyntax.Item) (ts : List Ty) : Bool :=
  (match ts with
   | t :: _ => RustSemantics.itemTakes item (shapeOf t)
   | [] => true) &&
  match RustSemantics.itemWidth item with
  | none => true
  | some w => if RustSemantics.itemShifts item then
      (match ts with
       | t :: _ => tyBeq t (.fixed w)
       | [] => false)
    else ts.all fun t => tyBeq t (.fixed w)

/-- A fixed-width type. -/
def isFixed : Ty → Bool
  | .fixed _ => true
  | _ => false

/-- A sequence type: a list or bytes. -/
def isSeq : Ty → Bool
  | .list _ => true
  | .bytes => true
  | _ => false

/-- A list type's element type. -/
def elemOf : Ty → Ty
  | .list t => t
  | _ => .unit

/-- The primitives whose value's type value typing constrains, at their
operand types: the checked, bitwise, and shift operations at their
operands' width, a conversion and a parse at their target's, and the
sequence and text operations at the sequence or text they build. -/
def fixedTyped (op : Prim) (ts : List Ty) (t : Ty) : Bool :=
  match op with
  | .checkedAdd | .checkedSub | .checkedMul | .checkedQuot =>
    match ts with
    | [a, b] => isFixed a && tyBeq b a && tyBeq t (.option a)
    | _ => false
  | .checkedNeg =>
    match ts with
    | [a] => isFixed a && tyBeq t (.option a)
    | _ => false
  | .bitAnd | .bitOr | .bitXor =>
    match ts with
    | [a, b] => isFixed a && tyBeq b a && tyBeq t a
    | _ => false
  | .bitNot =>
    match ts with
    | [a] => isFixed a && tyBeq t a
    | _ => false
  | .shiftLeft | .shiftRight =>
    match ts with
    | [a, b] => isFixed a && tyBeq b (.fixed .u32) && tyBeq t (.option a)
    | _ => false
  | .convert k => tyBeq t (.option (.fixed k))
  | .parseDecimal target => isFixed target && tyBeq t (.option target)
  | .append =>
    match ts with
    | [a, b] => isSeq a && tyBeq b a && tyBeq t a
    | _ => false
  | .slice =>
    match ts with
    | [a, _, _] => isSeq a && tyBeq t (.option a)
    | _ => false
  | .index =>
    match ts with
    | [a, _] => (isSeq a && tyBeq a (.list (elemOf a)) && tyBeq t (.option (elemOf a))) ||
        (tyBeq a .bytes && tyBeq t (.option (.fixed .u8)))
    | _ => false
  | .utf8Encode =>
    match ts with
    | [a] => tyBeq a .string && tyBeq t .bytes
    | _ => false
  | .utf8Decode =>
    match ts with
    | [a] => tyBeq a .bytes && tyBeq t (.option .string)
    | _ => false
  | .join =>
    match ts with
    | [a, b] => tyBeq a (.list .string) && tyBeq b .string && tyBeq t .string
    | _ => false
  | .formatDecimal =>
    match ts with
    | [_] => tyBeq t .string
    | _ => false
  | .splitExact =>
    match ts with
    | [a, b, d] => tyBeq a .string && tyBeq b .string && tyBeq d (.fixed .u32) &&
        tyBeq t (.option (.list .string))
    | _ => false
  | .natAdd | .natSub | .natMul | .natQuot | .natRem | .natEq | .natLe | .natLt
  | .intAdd | .intSub | .intMul | .intNeg | .intQuot | .intRem | .equal | .boolNot
  | .boolAnd | .boolOr | .length | .compareBytes | .compare => false

/-- The type a primitive's value has, as the correspondence knows it:
inert, or fixed by its operands' widths. -/
def primTyped (op : Prim) (ts : List Ty) (t : Ty) : Bool :=
  inert t || fixedTyped op ts t

mutual
/-- The value a Rust literal expression builds, if it is one. -/
def litValue : RExpr → Option Value
  | .lit l => RustSemantics.literal l
  | .construct k rs => (litValues rs).bind (RustSemantics.constructValue k)
  | .pair a b => (litValue a).bind fun x => (litValue b).map fun y => .pair x y
  | .box e => litValue e
  | _ => none
def litValues : List RExpr → Option (List Value)
  | [] => some []
  | r :: rs => (litValue r).bind fun v => (litValues rs).map (v :: ·)
end

mutual
/-- Whether a literal value has its type; a literal holds no closure. -/
def typedLit (p : Program) : Value → Ty → Bool
  | .unit, .unit => true
  | .none, .option _ => true
  | .some v, .option t => typedLit p v t
  | .ok v, .result a _ => typedLit p v a
  | .error v, .result _ b => typedLit p v b
  | .list vs, .list t => typedLitAll p vs t
  | .pair a b, .pair s t => typedLit p a s && typedLit p b t
  | .adt k fs, .adt i => match adtFields p i k with
    | some ts => typedLits p fs ts
    | none => false
  | .closure _ _, _ => false
  | v, .fixed w => RustSemantics.hasWidth w v
  | .string _, .string => true
  | .bytes _, .bytes => true
  | _, t => inert t
def typedLitAll (p : Program) : List Value → Ty → Bool
  | [], _ => true
  | v :: vs, t => typedLit p v t && typedLitAll p vs t
def typedLits (p : Program) : List Value → List Ty → Bool
  | [], [] => true
  | v :: vs, t :: ts => typedLit p v t && typedLits p vs ts
  | _, _ => false
end

/-- The operand types a constructor shape builds a value of type `ty` from,
when the rendering's constructor `k` is that shape's. -/
def buildShape (p : Program) : Shape → Ty → RustSyntax.Ctor → Option (List Ty)
  | .none, .option _, .none _ => some []
  | .some, .option t, .some => some [t]
  | .ok, .result a _, .ok _ _ => some [a]
  | .error, .result _ b, .err _ _ => some [b]
  | .nil, .list _, .nil _ => some []
  | .cons, .list t, .cons => some [t, .list t]
  | .adt k, .adt i, .adt i' k' => if i == i' && k == k' then adtFields p i k else none
  | _, _, _ => none

/-- The literal a nullary shape of type `ty` is rendered as. -/
def constShape : Shape → Ty → Option RustSyntax.Lit
  | .true, .bool => some (.bool true)
  | .false, .bool => some (.bool false)
  | .unit, .unit => some .unit
  | .lt, .ordering => some (.ordering .less)
  | .eq, .ordering => some (.ordering .same)
  | .gt, .ordering => some (.ordering .more)
  | .zero, .nat => some (.nat 0)
  | _, _ => none


/-! Helpers for conditionals, matches, and blocks. -/

/-- The negation the rendering writes for a condition. -/
def negateR : RExpr → RExpr
  | .lit (.bool b) => .lit (.bool (!b))
  | .isZero i => .nonZero i
  | .nonZero i => .isZero i
  | .not e => e
  | e => .not e

/-- `lets` then `v`, a block's lets joining them. -/
def flatB (lets : List RLet) : RExpr → RBlock
  | .block ⟨inner, last⟩ => ⟨lets ++ inner, last⟩
  | other => ⟨lets, other⟩

/-- One fold step: a block whose last let binds `i` (with no stated type)
and whose value reads `i` back is the bound value itself, its block
flattened. -/
def foldStep : RBlock → Option RBlock
  | ⟨lets, tail⟩ =>
    match lets.getLast?, tail with
    | some (.mk (.bind i) none v), .move j | some (.mk (.bind i) none v), .copy j =>
      if RustSemantics.sameIdent i j then some (flatB lets.dropLast v) else none
    | _, _ => none

/-- Fold steps until none applies. -/
def foldN : Nat → RBlock → RBlock
  | 0, b => b
  | n+1, b => match foldStep b with
    | some b' => foldN n b'
    | none => b

/-- A block as the rendering folds it. -/
def foldB (b : RBlock) : RBlock := match b with
  | ⟨lets, _⟩ => foldN lets.length b

/-- How a conditional's branches are entered: on a Boolean, or on the
holder `h` of a natural whose successor arm binds `x` to `v<s>`. -/
inductive Kind where
  | plain
  | nat (h : RIdent) (x : Nat) (s : Option Nat)

/-- The scope of the else branch. -/
def elseCtx : Kind → Ctx → Ctx
  | .plain, Γ => Γ
  | .nat _ x s, Γ => (x, .nat, s) :: Γ

/-- The predecessor's binding the successor branch begins with. -/
def elseLets : Kind → List RLet
  | .plain => []
  | .nat _ _ none => []
  | .nat h _ (some m) => [.mk (.bind (local_ m)) none (.predecessor h)]

/-- A holder may be bound without hiding the natural's. -/
def heldOK : Kind → RIdent → Bool
  | .plain, _ => true
  | .nat h _ _, i => !RustSemantics.sameIdent i h

def slotOK (Γ : Ctx) : Option Nat → Bool
  | none => true
  | some m => slotFree Γ m

/-- The arms of a match on a natural: the zero arm and the successor arm
binding `x`. -/
def natArms : List Arm → Option (Expr × Nat × Expr)
  | [.arm .zero [] a, .arm .succ [x] b] => some (a, x, b)
  | [.arm .succ [x] b, .arm .zero [] a] => some (a, x, b)
  | _ => none

/-- The arms of a match on a Boolean. -/
def boolArms : List Arm → Option (Expr × Expr)
  | [.arm .true [] a, .arm .false [] b] => some (a, b)
  | [.arm .false [] b, .arm .true [] a] => some (a, b)
  | _ => none

def shapeBeq : Shape → Shape → Bool
  | .none, .none | .some, .some | .ok, .ok | .error, .error => true
  | .nil, .nil | .cons, .cons | .zero, .zero | .succ, .succ | .pair, .pair => true
  | .true, .true | .false, .false | .unit, .unit | .lt, .lt | .eq, .eq | .gt, .gt => true
  | .adt i, .adt j => i == j
  | _, _ => false

/-- The shape of an arm. -/
def armShape : Arm → Shape
  | .arm s _ _ => s

/-- The Rust scrutinee of a match on the holder `h`: the value, or for a
list its first cell. -/
def scrut (view : Bool) (h : RIdent) : RExpr := if view then .uncons h else .move h

/-- The types of the fields a shape destructs at a type. -/
def shapeFields (p : Program) : Ty → Shape → Option (List Ty)
  | .option _, .none => some []
  | .option t, .some => some [t]
  | .result a _, .ok => some [a]
  | .result _ b, .error => some [b]
  | .list _, .nil => some []
  | .list t, .cons => some [t, .list t]
  | .nat, .zero => some []
  | .nat, .succ => some [.nat]
  | .pair a b, .pair => some [a, b]
  | .bool, .true | .bool, .false | .unit, .unit => some []
  | .ordering, .lt | .ordering, .eq | .ordering, .gt => some []
  | .adt i, .adt k => adtFields p i k
  | _, _ => none

/-- Every arm at the positions `idx` has a shape of the scrutinee's type
other than `s`. -/
def otherArms (p : Program) (st : Ty) (arms : List Arm) (idx : List Nat) (s : Shape) : Bool :=
  idx.all fun i => match arms[i]? with
    | some a => !shapeBeq (armShape a) s && (shapeFields p st (armShape a)).isSome
    | none => true

/-- The scope a function's parameters open, innermost (last) first: each
parameter pattern binds its local, or nothing. -/
def paramCtx : Ctx → List Nat → List Ty → List RustSyntax.Pat → Option Ctx
  | acc, [], [], [] => some acc
  | acc, x :: xs, t :: ts, .wild :: qs => paramCtx ((x, t, none) :: acc) xs ts qs
  | acc, x :: xs, t :: ts, .bind (.generated .binding m) :: qs =>
      if slotFree acc m then paramCtx ((x, t, some m) :: acc) xs ts qs else none
  | _, _, _, _ => none

/-- The patterns of the fields of an arm's shape, inside the pattern that
matches the shape at a scrutinee type (or, with `view`, at a list's first
cell). -/
def armOuter : Ty → Bool → Shape → RustSyntax.Pat → Option (List RustSyntax.Pat)
  | .option _, false, .none, .none => some []
  | .option _, false, .some, .some q => some [q]
  | .result _ _, false, .ok, .ok q => some [q]
  | .result _ _, false, .error, .err q => some [q]
  | .ordering, false, .lt, .ordering .less => some []
  | .ordering, false, .eq, .ordering .same => some []
  | .ordering, false, .gt, .ordering .more => some []
  | .list _, true, .nil, .none => some []
  | .list _, true, .cons, .some (.tuple qs) => if qs.length == 2 then some qs else none
  | .adt i, false, .adt k, .adt i' k' qs => if i == i' && k == k' then some qs else none
  | _, _, _, _ => none

/-- The field patterns with each boxed field read back by its load: a
boxed temporary bound by the pattern and loaded, in order, into a local. -/
def bindOf : RustSyntax.Pat → Option RIdent
  | .bind i => some i
  | _ => none

def isWild : RustSyntax.Pat → Bool
  | .wild => true
  | _ => false

def unboxOf : RExpr → Option RIdent
  | .unbox i => some i
  | _ => none

def identLocal : RIdent → Option Nat
  | .generated k m => if RustSemantics.kindNumber k == 0 then some m else none
  | .exported _ => none

def identBoxed : RIdent → Option Nat
  | .generated k m => if RustSemantics.kindNumber k == 4 then some m else none
  | .exported _ => none

/-- A load: the local `v<m>` bound to the boxed temporary `r<b>` read back. -/
def loadOf : RLet → Option (Nat × Nat)
  | .mk q ty e =>
    (bindOf q).bind fun i => (identLocal i).bind fun m =>
    if ty.isNone then (unboxOf e).bind fun j => (identBoxed j).map fun b => (m, b) else none

def directPat : List RustSyntax.Pat → List RLet → List Nat → Option (List RustSyntax.Pat)
  | [], loads, _ => if loads.isEmpty then some [] else none
  | q :: qs, loads, seen =>
    if isWild q then (directPat qs loads seen).map (.wild :: ·) else
    match bindOf q with
    | none => none
    | some i => match identLocal i with
      | some n => (directPat qs loads seen).map (.bind (local_ n) :: ·)
      | none => match identBoxed i with
        | none => none
        | some n => match loads with
          | [] => none
          | l :: rest => match loadOf l with
            | none => none
            | some (m, b) =>
              if n == b && !seen.contains n then
                (directPat qs rest (n :: seen)).map (.bind (local_ m) :: ·)
              else none

/-- The locals binder patterns bind. -/
def patLocal (q : RustSyntax.Pat) : List Nat :=
  match (bindOf q).bind identLocal with
  | some m => [m]
  | none => []

def patLocals : List RustSyntax.Pat → List Nat
  | [] => []
  | q :: qs => patLocal q ++ patLocals qs

/-- The scope an arm's body is rendered in, given the arm's pattern and the
loads its body begins with, when the pattern realizes the arm's shape at
the scrutinee's type. -/
def armPat (p : Program) (Γ : Ctx) (st : Ty) (view : Bool) (shape : Shape) (xs : List Nat)
    (q : RustSyntax.Pat) (loads : List RLet) : Option Ctx :=
  (armOuter st view shape q).bind fun inner =>
  (shapeFields p st shape).bind fun ts =>
  (directPat inner loads []).bind fun qsD =>
    if decide (patLocals qsD).Nodup then paramCtx Γ xs ts qsD else none

/-- Whether a function type has no closure: no dispatch arm of the crate
calls a function of that type. -/
def noClosure (p : Program) (c : RCrate) (ps : List Ty) (r : Ty) : Bool :=
  c.items.all fun item => match item with
    | .apply _ _ _ _ arms => arms.all fun d => match p.functions[d.function]? with
      | some fn => !(tysBeq (fn.types.drop d.captures.length) ps && tyBeq fn.result r)
      | none => true
    | _ => true

/-- Types no value inhabits, given ADTs `U` claimed empty. -/
def uninh (p : Program) (c : RCrate) (U : List Nat) : Ty → Bool
  | .adt i => U.contains i
  | .pair a b => uninh p c U a || uninh p c U b
  | .result a b => uninh p c U a && uninh p c U b
  | .fn ps r => noClosure p c ps r
  | _ => false

/-- Every constructor of every ADT claimed empty has a field of an empty
type. -/
def validU (p : Program) (c : RCrate) (U : List Nat) : Bool :=
  U.all fun i => match p.adts[i]? with
    | some adt => adt.constructors.all fun fields => fields.any (uninh p c U)
    | none => true

/-- Every arm is rendered, or never taken: its shape holds a field of an
empty type. -/
def covered (p : Program) (c : RCrate) (U : List Nat) (st : Ty) (arms : List Arm)
    (idx : List Nat) : Bool :=
  (List.range arms.length).all fun j => idx.contains j || match arms[j]? with
    | some a => match shapeFields p st (armShape a) with
      | some ts => ts.any (uninh p c U)
      | none => false
    | none => false

def ctorOf : RExpr → Option (RustSyntax.Ctor × List RExpr)
  | .construct k args => some (k, args)
  | _ => none

def litOf : RExpr → Option RustSyntax.Lit
  | .lit l => some l
  | _ => none

/-- Identifiers pairwise distinct. -/
def identsDistinct : List RIdent → Bool
  | [] => true
  | i :: is => !(is.any (RustSemantics.sameIdent i)) && identsDistinct is

/-- A field of type `t` rebuilt: read back from the binding the pattern
made, or a unit the pattern ignored. -/
def rebuildArg (t : Ty) (q : RustSyntax.Pat) (a : RExpr) : Bool :=
  match bindOf q with
  | some i => match readOf a with
    | some j => RustSemantics.sameIdent i j
    | none => false
  | none => isWild q && tyBeq t .unit && (match litOf a with
    | some l => match RustSemantics.literal l with
      | some v => match v with
        | .unit => true
        | _ => false
      | none => false
    | none => false)

def rebuildArgs : List Ty → List RustSyntax.Pat → List RExpr → Bool
  | t :: ts, q :: qs, a :: as => rebuildArg t q a && rebuildArgs ts qs as
  | [], [], [] => true
  | _, _, _ => false

/-- Whether a match arm's value is exactly the scrutinee it matched at type
`st`: the same variant rebuilt from the same bindings. -/
def rebuildTail (p : Program) (st : Ty) (q : RustSyntax.Pat) (tail : RExpr) : Bool :=
  match q, st with
  | .none, .option _ => match ctorOf tail with
    | some (.none _, []) => true
    | _ => false
  | .some q', .option t => match ctorOf tail with
    | some (.some, [a]) => rebuildArg t q' a
    | _ => false
  | .ok q', .result t _ => match ctorOf tail with
    | some (.ok _ _, [a]) => rebuildArg t q' a
    | _ => false
  | .err q', .result _ t => match ctorOf tail with
    | some (.err _ _, [a]) => rebuildArg t q' a
    | _ => false
  | .ordering o, .ordering => match litOf tail with
    | some (.ordering o') => TargetSemantics.sameOrder o o'
    | _ => false
  | .adt i k qs, .adt i0 => match ctorOf tail with
    | some (.adt i' k', args) => i == i0 && i == i' && k == k' &&
        identsDistinct (qs.filterMap bindOf) && match adtFields p i k with
          | some ts => rebuildArgs ts qs args
          | none => false
    | _ => false
  | _, _ => false

def rebuildsArm (p : Program) (st : Ty) (a : RustSyntax.Arm) : Bool :=
  match a with
  | .mk q ⟨lets, tail⟩ => lets.isEmpty && rebuildTail p st q tail

/-- The arm a record's field is read by: the selected field bound to `j`,
every other one ignored. -/
def fieldPats : Option Nat → Nat → RIdent → List RustSyntax.Pat
  | _, 0, _ => []
  | some 0, n+1, i => .bind i :: fieldPats none n i
  | some (s+1), n+1, i => .wild :: fieldPats (some s) n i
  | none, n+1, i => .wild :: fieldPats none n i

/-- The Rust tail of a value read `r`, in a fallible function's tail when
`fm`: `Ok` of it, or for a unit, the unit bound and `Ok(())`. -/
def okBlock (fm unit : Bool) (r : RExpr) : RBlock :=
  if fm then
    if unit then ⟨[.mk .unit none r], .succeed (.lit .unit)⟩ else ⟨[], .succeed r⟩
  else ⟨[], r⟩

/-- A read of a record's field: moved, or unboxed. -/
def partRead : RExpr → RIdent → Bool
  | .move i, j | .unbox i, j => RustSemantics.sameIdent j i
  | _, _ => false

/-- Arguments as stored: each operand, boxed where the mask says. -/
def boxWith : List Bool → List RExpr → List RExpr
  | b :: bs, a :: as => (if b then .box a else a) :: boxWith bs as
  | _, as => as

/-- The judgments of the correspondence. -/
inductive Jd where
  /-- `e` of type `t` is computed by `r` in value position. -/
  | e (e : Expr) (t : Ty) (r : RExpr)
  /-- `e` is computed by binding `lets`, then evaluating `tail`; with `fm`,
  in tail position of a fallible function, as `Ok` of its value. -/
  | b (fm : Bool) (e : Expr) (t : Ty) (lets : List RLet) (tail : RExpr)
  /-- Operands evaluated left to right. -/
  | l (es : List Expr) (ts : List Ty) (rs : List RExpr)
  /-- Operands bound to temporaries in order, then read back. -/
  | ops (es : List Expr) (ts : List Ty) (lets : List RLet) (args : List RExpr)
  /-- A conditional entered on `ce` (a Boolean, or a natural's zero test):
  `a` when it holds, else `b`, computed by `lets` then `tail`. -/
  | k (kind : Kind) (fm : Bool) (ce : RExpr) (a b : Expr) (t : Ty) (lets : List RLet) (tail : RExpr)
  /-- The arms at positions `idx` of a match on a value of type `st` are
  realized by the Rust arms `rarms`, matching the value (or, with `view`,
  its first cell). -/
  | m (st : Ty) (view : Bool) (arms : List Arm) (idx : List Nat) (fm : Bool) (τ : Ty)
      (rarms : List RustSyntax.Arm)

inductive Corr (p : Program) (c : RCrate) (A : Flags) : Ctx → Bool → Jd → Prop where
  | var {Γ fl x t m r} : ctxLookup Γ x = some (t, some m) → readOf r = some (local_ m) →
      Corr p c A Γ fl (.e (.var x) t r)
  | varUnit {Γ fl x s} : ctxLookup Γ x = some (.unit, s) →
      Corr p c A Γ fl (.e (.var x) .unit (.lit .unit))
  | bOfE {Γ fl e t r} : Corr p c A Γ fl (.e e t r) → Corr p c A Γ fl (.b false e t [] r)
  | eOfB {Γ fl e t lets tail} : Corr p c A Γ fl (.b false e t lets tail) →
      Corr p c A Γ fl (.e e t (.block (foldB ⟨lets, tail⟩)))
  | lNil {Γ fl} : Corr p c A Γ fl (.l [] [] [])
  | lCons {Γ fl e es t ts r rs} : Corr p c A Γ fl (.e e t r) → Corr p c A Γ fl (.l es ts rs) →
      Corr p c A Γ fl (.l (e :: es) (t :: ts) (r :: rs))
  | opsNil {Γ fl} : Corr p c A Γ fl (.ops [] [] [] [])
  | opsCons {Γ fl e es t ts r lets args k j ty} : Corr p c A Γ fl (.e e t r) →
      Corr p c A Γ fl (.ops es ts lets args) → isTemp k = true →
      tempFresh (.generated k j) lets = true →
      Corr p c A Γ fl (.ops (e :: es) (t :: ts) (.mk (.bind (.generated k j)) ty r :: lets)
        (.move (.generated k j) :: args))
  | prim {Γ fl op es ts t lets args item} : Corr p c A Γ fl (.ops es ts lets args) →
      RustSemantics.itemPrimitive item = op → plainItem item = true → itemOK c item = true →
      itemTyped item ts = true → primTyped op ts t = true →
      (!RustSemantics.itemFallible item || fl) = true →
      Corr p c A Γ fl (.b false (.prim op es) t lets
        (.call (.runtime item) args (RustSemantics.itemFallible item)))
  | fOfB {Γ e t lets tail} : Corr p c A Γ true (.b false e t lets tail) →
      Corr p c A Γ true (.b true e t lets (.succeed tail))
  | value {Γ fl t v r} : litValue r = some v → typedLit p v t = true →
      Corr p c A Γ fl (.e (.value t v) t r)
  | eOfBNil {Γ fl e t r} : Corr p c A Γ fl (.b false e t [] r) → Corr p c A Γ fl (.e e t r)
  | opsOfL {Γ fl es ts rs} : Corr p c A Γ fl (.l es ts rs) → Corr p c A Γ fl (.ops es ts [] rs)
  | opsUnit {Γ fl e es ts r lets args ty} : Corr p c A Γ fl (.e e .unit r) →
      Corr p c A Γ fl (.ops es ts lets args) →
      Corr p c A Γ fl (.ops (e :: es) (.unit :: ts) (.mk .unit ty r :: lets) (.lit .unit :: args))
  | build {Γ fl shape ty es ts lets args k mask} : Corr p c A Γ fl (.ops es ts lets args) →
      buildShape p shape ty k = some ts →
      Corr p c A Γ fl (.b false (.build shape ty es) ty lets (.construct k (boxWith mask args)))
  | buildPair {Γ fl a b s t ra rb} : Corr p c A Γ fl (.l [a, b] [s, t] [ra, rb]) →
      Corr p c A Γ fl (.e (.build .pair (.pair s t) [a, b]) (.pair s t) (.pair ra rb))
  | buildConst {Γ fl shape ty l} : constShape shape ty = some l →
      Corr p c A Γ fl (.e (.build shape ty []) ty (.lit l))
  | letBind {Γ fl fm x t b body τ rb m ty lets tail} : Corr p c A Γ fl (.e b t rb) →
      slotFree Γ m = true → Corr p c A ((x, t, some m) :: Γ) fl (.b fm body τ lets tail) →
      Corr p c A Γ fl (.b fm (.let x t b body) τ (.mk (.bind (local_ m)) ty rb :: lets) tail)
  | letWild {Γ fl fm x t b body τ rb ty lets tail} : Corr p c A Γ fl (.e b t rb) →
      Corr p c A ((x, t, none) :: Γ) fl (.b fm body τ lets tail) →
      Corr p c A Γ fl (.b fm (.let x t b body) τ (.mk .wild ty rb :: lets) tail)
  | buildSucc {Γ e r} : Corr p c A Γ true (.l [e] [.nat] [r]) →
      Corr p c A Γ true (.e (.build .succ .nat [e]) .nat (.call (.runtime .natSucc) [r] true))
  | primWiden {Γ fl k e t0 t lets a} : Corr p c A Γ fl (.ops [e] [t0] lets [a]) →
      itemOK c (.convert k) = true → primTyped (.convert k) [t0] t = true →
      Corr p c A Γ fl (.b false (.prim (.convert k) [e]) t lets
        (.call (.runtime (.convert k)) [.widen a] false))
  | primF {Γ op es ts t lets args item} : Corr p c A Γ true (.ops es ts lets args) →
      RustSemantics.itemPrimitive item = op → plainItem item = true → itemOK c item = true →
      itemTyped item ts = true → primTyped op ts t = true →
      RustSemantics.itemFallible item = true →
      Corr p c A Γ true (.b true (.prim op es) t lets (.call (.runtime item) args false))
  | callOps {Γ fl f es ts t lets args F} {fn : TargetSyntax.Function} :
      Corr p c A Γ fl (.ops es ts lets args) →
      TargetSemantics.LexLeanRuntime.index p.functions f = some fn → fn.types = ts →
      fn.result = t → fnFallible c f = some F → (!F || fl) = true →
      Corr p c A Γ fl (.b false (.call f es) t lets (.call (.function f) args F))
  | callF {Γ f es ts t lets args} {fn : TargetSyntax.Function} :
      Corr p c A Γ true (.ops es ts lets args) →
      TargetSemantics.LexLeanRuntime.index p.functions f = some fn → fn.types = ts →
      fn.result = t → fnFallible c f = some true →
      Corr p c A Γ true (.b true (.call f es) t lets (.call (.function f) args false))
  | buildSuccF {Γ e r} : Corr p c A Γ true (.l [e] [.nat] [r]) →
      Corr p c A Γ true (.b true (.build .succ .nat [e]) .nat [] (.call (.runtime .natSucc) [r] false))
  | fUnit {Γ e lets tail} : Corr p c A Γ true (.b false e .unit lets tail) →
      Corr p c A Γ true (.b true e .unit (lets ++ [.mk .unit none tail]) (.succeed (.lit .unit)))
  | cond {Γ fl fm c0 a b t lc ce lk tk} : Corr p c A Γ fl (.b false c0 .bool lc ce) →
      Corr p c A Γ fl (.k .plain fm ce a b t lk tk) →
      Corr p c A Γ fl (.b fm (.cond c0 a b) t (lc ++ lk) tk)
  | condId {Γ fl fm c0 a b lc tc} : Corr p c A Γ fl (.b fm c0 .bool lc tc) →
      Corr p c A Γ fl (.b false a .bool [] (.lit (.bool true))) →
      Corr p c A Γ fl (.b false b .bool [] (.lit (.bool false))) →
      Corr p c A Γ fl (.b fm (.cond c0 a b) .bool lc tc)
  | kId {Γ fl kind ce a b} : Corr p c A Γ fl (.b false a .bool [] (.lit (.bool true))) →
      Corr p c A (elseCtx kind Γ) fl (.b false b .bool [] (.lit (.bool false))) →
      elseLets kind = [] → Corr p c A Γ fl (.k kind false ce a b .bool [] ce)
  | kNeg {Γ fl kind ce a b} : Corr p c A Γ fl (.b false a .bool [] (.lit (.bool false))) →
      Corr p c A (elseCtx kind Γ) fl (.b false b .bool [] (.lit (.bool true))) →
      elseLets kind = [] → Corr p c A Γ fl (.k kind false ce a b .bool [] (negateR ce))
  | kSame {Γ fl fm kind ce a b t lets tail ty} : Corr p c A Γ fl (.b fm a t lets tail) →
      Corr p c A (elseCtx kind Γ) fl (.b fm b t lets tail) → elseLets kind = [] →
      Corr p c A Γ fl (.k kind fm ce a b t (.mk .wild ty ce :: lets) tail)
  | kIf {Γ fl fm kind ce a b t la ta lb tb} : Corr p c A Γ fl (.b fm a t la ta) →
      Corr p c A (elseCtx kind Γ) fl (.b fm b t lb tb) →
      Corr p c A Γ fl (.k kind fm ce a b t []
        (.cond ce (foldB ⟨la, ta⟩) (foldB ⟨elseLets kind ++ lb, tb⟩)))
  | kHeld {Γ fl fm kind ce a b t la ta lb tb k j ty} : Corr p c A Γ fl (.b fm a t la ta) →
      Corr p c A (elseCtx kind Γ) fl (.b fm b t lb tb) → isTemp k = true →
      heldOK kind (.generated k j) = true →
      Corr p c A Γ fl (.k kind fm ce a b t [.mk (.bind (.generated k j)) ty ce]
        (.cond (.move (.generated k j)) (foldB ⟨la, ta⟩) (foldB ⟨elseLets kind ++ lb, tb⟩)))
  | matchNat {Γ fl fm τ s0 arms rs a x b sl k j ty lets tail} : Corr p c A Γ fl (.e s0 .nat rs) →
      natArms arms = some (a, x, b) → slotOK Γ sl = true → isTemp k = true →
      Corr p c A Γ fl (.k (.nat (.generated k j) x sl) fm (.isZero (.generated k j)) a b τ lets tail) →
      Corr p c A Γ fl (.b fm (.match τ s0 arms) τ (.mk (.bind (.generated k j)) ty rs :: lets) tail)
  | matchBool {Γ fl fm τ s0 arms rs a b k j ty lets tail} : Corr p c A Γ fl (.e s0 .bool rs) →
      boolArms arms = some (a, b) → isTemp k = true →
      Corr p c A Γ fl (.k .plain fm (.move (.generated k j)) a b τ lets tail) →
      Corr p c A Γ fl (.b fm (.match τ s0 arms) τ (.mk (.bind (.generated k j)) ty rs :: lets) tail)
  | matchBoolId {Γ fl fm s0 arms a b lets tail} : Corr p c A Γ fl (.b fm s0 .bool lets tail) →
      boolArms arms = some (a, b) →
      Corr p c A Γ fl (.b false a .bool [] (.lit (.bool true))) →
      Corr p c A Γ fl (.b false b .bool [] (.lit (.bool false))) →
      Corr p c A Γ fl (.b fm (.match .bool s0 arms) .bool lets tail)
  | matchArms {Γ fl fm τ st view s0 arms idx rs rarms U k j ty} : Corr p c A Γ fl (.e s0 st rs) →
      Corr p c A Γ fl (.m st view arms idx fm τ rarms) →
      validU p c U = true → covered p c U st arms idx = true → isTemp k = true →
      Corr p c A Γ fl (.b fm (.match τ s0 arms) τ [.mk (.bind (.generated k j)) ty rs]
        (.matchOn (scrut view (.generated k j)) rarms))
  | matchRebuilt {Γ fl fm st s0 arms idx rarms U lets tail} : Corr p c A Γ fl (.b fm s0 st lets tail) →
      Corr p c A Γ fl (.m st false arms idx false st rarms) → rarms.all (rebuildsArm p st) = true →
      validU p c U = true → covered p c U st arms idx = true →
      Corr p c A Γ fl (.b fm (.match st s0 arms) st lets tail)
  | mNil {Γ fl st view arms fm τ} : Corr p c A Γ fl (.m st view arms [] fm τ [])
  | mCons {Γ Γ' fl st view arms idx fm τ rarms j shape xs body q loads lets tail} :
      Corr p c A Γ fl (.m st view arms idx fm τ rarms) →
      arms[j]? = some (.arm shape xs body) → otherArms p st arms idx shape = true →
      armPat p Γ st view shape xs q loads = some Γ' →
      Corr p c A Γ' fl (.b fm body τ lets tail) →
      Corr p c A Γ fl (.m st view arms (j :: idx) fm τ (.mk q (foldB ⟨loads ++ lets, tail⟩) :: rarms))
  | matchPair {Γ Γ'' fl fm τ s0 x y body ta tb rs q r k j ty ty' lets tail} :
      Corr p c A Γ fl (.e s0 (.pair ta tb) rs) → paramCtx Γ [x, y] [ta, tb] [q, r] = some Γ'' →
      Corr p c A Γ'' fl (.b fm body τ lets tail) → isTemp k = true →
      Corr p c A Γ fl (.b fm (.match τ s0 [.arm .pair [x, y] body]) τ
        (.mk (.bind (.generated k j)) ty rs :: .mk (.tuple [q, r]) ty' (.move (.generated k j)) :: lets) tail)
  | matchUnit {Γ fl fm τ s0 body rs ty lets tail} : Corr p c A Γ fl (.e s0 .unit rs) →
      Corr p c A Γ fl (.b fm body τ lets tail) →
      Corr p c A Γ fl (.b fm (.match τ s0 [.arm .unit [] body]) τ (.mk .unit ty rs :: lets) tail)
  | first {Γ fl s0 ta tb rs k j ty} : Corr p c A Γ fl (.e s0 (.pair ta tb) rs) → isTemp k = true →
      Corr p c A Γ fl (.b false (.first s0) ta [.mk (.tuple [.bind (.generated k j), .wild]) ty rs]
        (.move (.generated k j)))
  | second {Γ fl s0 ta tb rs k j ty} : Corr p c A Γ fl (.e s0 (.pair ta tb) rs) → isTemp k = true →
      Corr p c A Γ fl (.b false (.second s0) tb [.mk (.tuple [.wild, .bind (.generated k j)]) ty rs]
        (.move (.generated k j)))
  | fieldRead {Γ fl fm r0 i pos ts t rr k j rd} : Corr p c A Γ fl (.e r0 (.adt i) rr) →
      (p.adts[i]?).map (·.constructors.length) = some 1 → adtFields p i 0 = some ts →
      ts[pos]? = some t → isTemp k = true → partRead rd (.generated k j) = true →
      (!fm || fl) = true →
      Corr p c A Γ fl (.b fm (.field r0 pos) t []
        (.matchOn rr [.mk (.adt i 0 (fieldPats (some pos) ts.length (.generated k j)))
          (okBlock fm (tyBeq t .unit) rd)]))
  | fieldHeld {Γ fl fm r0 i pos ts t rr k j rd k' j' ty} : Corr p c A Γ fl (.e r0 (.adt i) rr) →
      (p.adts[i]?).map (·.constructors.length) = some 1 → adtFields p i 0 = some ts →
      ts[pos]? = some t → isTemp k = true → partRead rd (.generated k j) = true →
      isTemp k' = true → (!fm || fl) = true →
      Corr p c A Γ fl (.b fm (.field r0 pos) t [.mk (.bind (.generated k' j')) ty rr]
        (.matchOn (.move (.generated k' j')) [.mk (.adt i 0 (fieldPats (some pos) ts.length (.generated k j)))
          (okBlock fm (tyBeq t .unit) rd)]))
  | closure {Γ fl f es caps ps r lets args mask q af ff} {fn : TargetSyntax.Function} :
      Corr p c A Γ fl (.ops es caps lets args) →
      p.functions[f]? = some fn → fn.types = caps ++ ps → fn.result = r →
      flagOf A ps r = some af → RustSemantics.findDispatch c.items f caps.length = some (af, ff) →
      fnFallible c f = some ff → (!ff || af) = true →
      Corr p c A Γ fl (.b false (.closure f es) (.fn ps r) lets (.construct (.closure q f) (boxWith mask args)))
  | apply {Γ fl tg ps r rt es lets args af k j ty} : Corr p c A Γ fl (.e tg (.fn ps r) rt) →
      Corr p c A Γ fl (.ops es ps lets args) → flagOf A ps r = some af → (!af || fl) = true →
      isTemp k = true → tempFresh (.generated k j) lets = true →
      Corr p c A Γ fl (.b false (.apply tg es) r (.mk (.bind (.generated k j)) ty rt :: lets)
        (.apply (.generated k j) args af))
  | applyF {Γ tg ps r rt es lets args k j ty} : Corr p c A Γ true (.e tg (.fn ps r) rt) →
      Corr p c A Γ true (.ops es ps lets args) → flagOf A ps r = some true →
      isTemp k = true → tempFresh (.generated k j) lets = true →
      Corr p c A Γ true (.b true (.apply tg es) r (.mk (.bind (.generated k j)) ty rt :: lets)
        (.apply (.generated k j) args false))

end LexLeanPreservation.Rust

namespace LexLeanPreservation.Rust
open LexLeanTarget LexLeanTarget.TargetSyntax
open RustSemantics (ROutcome ROutcomes)

section
variable (p : Program) (c : RCrate) (A : Flags)

/-- How the rendering realizes an observation in value position: the same
value; an overflow raised, in a scope that may fail, or the machine's
abort. -/
def Realizes (fl : Bool) : Obs → ROut → Prop
  | .value v, .value w => w = v
  | .overflow, .raise => fl = true
  | .overflow, .abort => True
  | _, _ => False

def RealizesL (fl : Bool) : ObsL → ROutcomes → Prop
  | .values vs, .values ws => ws = vs
  | .overflow, .raise => fl = true
  | .overflow, .abort => True
  | _, _ => False

/-- The tail of a fallible function: `Ok` of the value, or the error, raised
or returned. -/
def RealizesF : Obs → ROut → Prop
  | .value v, .value w => w = .ok v
  | .overflow, .raise => True
  | .overflow, .abort => True
  | .overflow, .value w => w = .error .unit
  | _, _ => False

/-- A function's result: its value, or, for a fallible one, `Ok` of it and
`Err(Overflow)` for an overflow. -/
def RealizesFn (fallible : Bool) : Obs → ROut → Prop
  | .value v, .value w => w = cond fallible (.ok v) v
  | .overflow, .value w => fallible = true ∧ w = .error .unit
  | .overflow, .abort => True
  | _, _ => False

/-- How a block realizes an observation: in value position, or, with `fm`,
in the tail of a fallible function. -/
def RealizesM : Bool → Bool → Obs → ROut → Prop
  | true, _ => RealizesF
  | false, fl => Realizes fl

/-- The value a match's Rust arms see: the scrutinee, or with `view`, its
first cell. -/
def viewVal : Bool → Value → Value
  | false, v => v
  | true, .list [] => .none
  | true, .list (x :: xs) => .some (.pair x (.list xs))
  | true, v => v

/-- What entering a conditional on `ce` gives: the Boolean `β`, and the
scope of the branch it chooses. -/
def KPre (c : RCrate) (Γ : Ctx) : Kind → REnv → RExpr → Bool → List (Nat × Value) →
    List (Nat × Value) → Prop
  | .plain, renv, ce, β, env, env' => RC c renv ce (.value (.bool β)) ∧ env' = env
  | .nat hv x s, renv, ce, β, env, env' => slotOK Γ s = true ∧
      ∃ k, RustSemantics.lookup renv hv = some (.nat k) ∧
        RC c renv ce (.value (.bool β)) ∧ β = (Nat.beq k 0) ∧
        env' = (if β then env else (x, .nat (k - 1)) :: env)

def Fresh (Γ : Ctx) (new : REnv) : Prop := ∀ m w, (local_ m, w) ∈ new → slotFree Γ m = true

def Sem (n : Nat) (Γ : Ctx) (fl : Bool) : Jd → Prop
  | .e e t r => ∀ env renv, EnvRel p c A Γ env renv → ∀ o,
      obs (TargetSemantics.eval n p env e) = some o → o ≠ .stuck →
      (∀ v, o = .value v → WT p c A v t) ∧ ∃ ro, Realizes fl o ro ∧ RC c renv r ro
  | .b fm e t lets tail => ∀ env renv, EnvRel p c A Γ env renv → ∀ o,
      obs (TargetSemantics.eval n p env e) = some o → o ≠ .stuck →
      (∀ v, o = .value v → WT p c A v t) ∧
      ((∃ new, Fresh Γ new ∧ LetsOK c renv lets (new ++ renv) ∧
          ∃ ro, RealizesM fm fl o ro ∧ RC c (new ++ renv) tail ro) ∨
        (o = .overflow ∧ ∃ h, RealizesM fm fl .overflow h ∧ LetsRaise c renv lets h))
  | .k kind fm ce a b t lets tail => ∀ env renv, EnvRel p c A Γ env renv →
      ∀ (β : Bool) (env' : List (Nat × Value)), KPre c Γ kind renv ce β env env' →
      ∀ o, obs (TargetSemantics.eval n p env' (if β then a else b)) = some o → o ≠ .stuck →
      (∀ v, o = .value v → WT p c A v t) ∧
      ((∃ new, Fresh Γ new ∧ LetsOK c renv lets (new ++ renv) ∧
          ∃ ro, RealizesM fm fl o ro ∧ RC c (new ++ renv) tail ro) ∨
        (o = .overflow ∧ ∃ h, RealizesM fm fl .overflow h ∧ LetsRaise c renv lets h))
  | .m st view arms idx fm τ rarms => ∀ env renv, EnvRel p c A Γ env renv → ∀ v, WT p c A v st →
      ∀ j shape xs body, j ∈ idx → arms[j]? = some (.arm shape xs body) →
      ∀ fields env', TargetSemantics.destruct shape v = some fields →
      TargetSemantics.bindAll xs fields env = some env' →
      ∀ o, obs (TargetSemantics.eval n p env' body) = some o → o ≠ .stuck →
      (∀ w, o = .value w → WT p c A w τ) ∧ (view = true → ∃ items, v = .list items) ∧
      ∃ ro, RealizesM fm fl o ro ∧ RCA c renv (viewVal view v) rarms ro
  | .l es ts rs => ∀ env renv, EnvRel p c A Γ env renv → ∀ o,
      obsL (TargetSemantics.evalList n p env es) = some o → o ≠ .stuck →
      (∀ vs, o = .values vs → WTL p c A vs ts) ∧ ∃ ro, RealizesL fl o ro ∧ RCL c renv rs ro
  | .ops es ts lets args => ∀ env renv, EnvRel p c A Γ env renv → ∀ o,
      obsL (TargetSemantics.evalList n p env es) = some o → o ≠ .stuck →
      (∀ vs, o = .values vs → WTL p c A vs ts ∧ ∃ new, Fresh Γ new ∧
          LetsOK c renv lets (new ++ renv) ∧ RCL c (new ++ renv) args (.values vs)) ∧
      (o = .overflow → (∃ h, Realizes fl .overflow h ∧ LetsRaise c renv lets h) ∨
        (∃ new, Fresh Γ new ∧ LetsOK c renv lets (new ++ renv) ∧
          ∃ ro, RealizesL fl .overflow ro ∧ RCL c (new ++ renv) args ro))

/-- A function and its rendering correspond: the parameter patterns open a
scope in which the body corresponds to the rendered body (as the rendering
folds it), as a fallible function's tail when the rendering returns `R<T>`;
or no value of some parameter's type exists, so the function is never
called. -/
def FunOK (f : Nat) : Prop :=
  ∃ fn : TargetSyntax.Function, TargetSemantics.LexLeanRuntime.index p.functions f = some fn ∧
    ((∃ params rty lets tail Γ,
      RustSemantics.findFunction c.items (fnIdent f) = some (params, rty, foldB ⟨lets, tail⟩) ∧
      paramCtx [] fn.parameters fn.types (RustSemantics.patternsOf params) = some Γ ∧
      Corr p c A Γ (RustSemantics.fallibleType rty)
        (.b (RustSemantics.fallibleType rty) fn.body fn.result lets tail)) ∨
     (∃ U t, validU p c U = true ∧ t ∈ fn.types ∧ uninh p c U t = true))

def CrateOK : Prop := ∀ f (fn : TargetSyntax.Function),
  TargetSemantics.LexLeanRuntime.index p.functions f = some fn → FunOK p c A f

/-- A function of the program, called at fuel `n`, is simulated by its
rendering. -/
def FunSem (n : Nat) (f : Nat) : Prop :=
  ∀ (fn : TargetSyntax.Function) args env F,
    TargetSemantics.LexLeanRuntime.index p.functions f = some fn → WTL p c A args fn.types →
    TargetSemantics.bindAll fn.parameters args [] = some env → fnFallible c f = some F →
    ∀ o, obs (TargetSemantics.eval n p env fn.body) = some o → o ≠ .stuck →
      (∀ v, o = .value v → WT p c A v fn.result) ∧
      ∃ ro, RealizesFn F o ro ∧ RCI c (fnIdent f) args ro
end

/-! Facts about the definitions. -/

mutual
theorem wt_of_inert {p c A} : ∀ (v : Value) (t : Ty), inert t = true → WT p c A v t
  | v, t, h => by
    cases t with
    | unit | adt _ | fn _ _ | list _ => simp [inert] at h
    | option t =>
      cases v with
      | some x => simp only [WT]; exact wt_of_inert x t (by simpa [inert] using h)
      | _ => simp [WT] <;> simpa using h
    | result a b =>
      simp [inert] at h
      cases v with
      | ok x => simp only [WT]; exact wt_of_inert x a h.1
      | error x => simp only [WT]; exact wt_of_inert x b h.2
      | _ => simp [WT, inert, h]
    | pair a b =>
      simp [inert] at h
      cases v with
      | pair x y => simp only [WT]; exact ⟨wt_of_inert x a h.1, wt_of_inert y b h.2⟩
      | _ => simp [WT, inert, h]
    | _ => cases v <;> simp_all [WT, inert]
theorem wtAll_of_inert {p c A} : ∀ (vs : List Value) (t : Ty), inert t = true → WTAll p c A vs t
  | [], _, _ => by simp [WTAll]
  | v :: vs, t, h => by simp only [WTAll]; exact ⟨wt_of_inert v t h, wtAll_of_inert vs t h⟩
end

end LexLeanPreservation.Rust
