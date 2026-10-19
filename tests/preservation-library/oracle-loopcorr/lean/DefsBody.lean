
namespace Tre


/-- How a parameter position is carried by the loop. -/
inductive Pos where
  | changed : Pos
  | kept : Ident → Pos
  | dropped : Pos

def readName : Expr → Option Ident
  | .move i => some i
  | .clone i => some i
  | .copy i => some i
  | .deref i => some i
  | _ => none

def isPure : Expr → Bool
  | .lit _ => true
  | .move _ => true
  | .clone _ => true
  | .copy _ => true
  | .deref _ => true
  | .unbox _ => true
  | _ => false

def passes (arg : Expr) (name : Ident) : Bool :=
  match readName arg with
  | some i => sameIdent i name
  | none => false

/-- The arguments of a self call that the state carries, or none when a kept
parameter is not passed on or a dropped argument may have an effect. -/
def carried : List Pos → List Expr → Option (List Expr)
  | [], [] => some []
  | Pos.changed :: ps, a :: as => (carried ps as).map (a :: ·)
  | Pos.kept n :: ps, a :: as => if passes a n then carried ps as else none
  | Pos.dropped :: ps, a :: as => if isPure a then carried ps as else none
  | _, _ => none

def pack : List Expr → Option Expr
  | [] => none
  | [a] => some a
  | a :: rest => (pack rest).map (Expr.pair a)


mutual
def patBinders : Pat → List Ident
  | Pat.wild => []
  | Pat.bind i => [i]
  | Pat.unit => []
  | Pat.tuple ps => patBindersList ps
  | Pat.none => []
  | Pat.some p => patBinders p
  | Pat.ok p => patBinders p
  | Pat.err p => patBinders p
  | Pat.ordering _ => []
  | Pat.adt _ _ ps => patBindersList ps
def patBindersList : List Pat → List Ident
  | [] => []
  | p :: ps => patBinders p ++ patBindersList ps
end

def keptNames : List Pos → List Ident
  | [] => []
  | Pos.kept n :: ps => n :: keptNames ps
  | _ :: ps => keptNames ps

/-- A pattern binds none of the kept parameters. -/
def hygP (K : List Ident) (p : Pat) : Bool :=
  (patBinders p).all fun b => K.all fun n => !sameIdent b n

def hygLets (K : List Ident) : List Let → Bool
  | [] => true
  | Let.mk p _ _ :: rest => hygP K p && hygLets K rest

mutual
def rtE (me : Nat) (ps : List Pos) (S D : RustType) : Expr → Option Expr
  | Expr.call (Callee.function m) args false =>
      if m == me then
        match carried ps args with
        | none => none
        | some xs => match pack xs with
          | none => none
          | some st => some (Expr.construct (Ctor.ok S D) [st])
      else some (Expr.construct (Ctor.err S D) [Expr.call (Callee.function m) args false])
  | Expr.cond c t e =>
      match rtB me ps S D t, rtB me ps S D e with
      | some t', some e' => some (Expr.cond c t' e')
      | _, _ => none
  | Expr.matchOn s arms =>
      match rtArms me ps S D arms with
      | some arms' => some (Expr.matchOn s arms')
      | none => none
  | Expr.block b =>
      match rtB me ps S D b with
      | some b' => some (Expr.block b')
      | none => none
  | x => some (Expr.construct (Ctor.err S D) [x])

def rtB (me : Nat) (ps : List Pos) (S D : RustType) : Block → Option Block
  | Block.mk lets tail =>
      if hygLets (keptNames ps) lets then
        match rtE me ps S D tail with
        | some tail' => some (Block.mk lets tail')
        | none => none
      else none

def rtArms (me : Nat) (ps : List Pos) (S D : RustType) : List Arm → Option (List Arm)
  | [] => some []
  | Arm.mk p b :: rest =>
      if hygP (keptNames ps) p then
        match rtB me ps S D b, rtArms me ps S D rest with
        | some b', some rest' => some (Arm.mk p b' :: rest')
        | _, _ => none
      else none
end


end Tre
