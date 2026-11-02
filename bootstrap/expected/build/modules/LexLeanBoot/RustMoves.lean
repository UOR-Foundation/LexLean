module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanBoot.RustAst
public import LexLeanBoot.RustValidate
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.RustMoves

public structure Later where
  whole : Nat
  parts : List ((Prod (Nat) (Nat)))

@[expose] public def idKey (ident : LexLeanBoot.RustValidate.Id) : (Prod (Nat) (Nat)) := (match ident with | LexLeanBoot.RustValidate.Id.IdLocal n => (0, n) | LexLeanBoot.RustValidate.Id.IdHolder n => (1, n) | LexLeanBoot.RustValidate.Id.IdOperand n => (2, n) | LexLeanBoot.RustValidate.Id.IdCallee n => (3, n) | LexLeanBoot.RustValidate.Id.IdBoxed n => (4, n) | LexLeanBoot.RustValidate.Id.IdPart n => (5, n) | LexLeanBoot.RustValidate.Id.IdCapture n => (6, n) | LexLeanBoot.RustValidate.Id.IdParam n => (7, n) | LexLeanBoot.RustValidate.Id.IdFunction n => (8, n) | LexLeanBoot.RustValidate.Id.IdExport _ => (9, 0))

@[expose] public def idEq (left : LexLeanBoot.RustValidate.Id) (right : LexLeanBoot.RustValidate.Id) : Bool := (let one : (Prod (Nat) (Nat)) := idKey (left); (let two : (Prod (Nat) (Nat)) := idKey (right); ((Nat.beq ((one).1) ((two).1)) && (Nat.beq ((one).2) ((two).2)))))

@[expose] public def natMax (left : Nat) (right : Nat) : Nat := (if (Nat.blt (left) (right)) then right else left)

@[expose] public def partCount : (parts : List ((Prod (Nat) (Nat)))) -> (part : Nat) -> Nat
  | List.nil, _part => 0
  | List.cons head tail, part => (if (Nat.beq ((head).1) (part)) then (head).2 else partCount (tail) (part))

@[expose] public def partSet : (parts : List ((Prod (Nat) (Nat)))) -> (part : Nat) -> (count : Nat) -> List ((Prod (Nat) (Nat)))
  | List.nil, part, count => ((part, count) :: ([] : List ((Prod (Nat) (Nat)))))
  | List.cons head tail, part, count => (if (Nat.beq ((head).1) (part)) then ((part, count) :: tail) else (head :: partSet (tail) (part) (count)))

@[expose] public def mergeParts : (left : List ((Prod (Nat) (Nat)))) -> (right : List ((Prod (Nat) (Nat)))) -> List ((Prod (Nat) (Nat)))
  | left, List.nil => left
  | left, List.cons head tail => mergeParts (partSet (left) ((head).1) (natMax ((head).2) (partCount (left) ((head).1)))) (tail)

@[expose] public def atMostOne : (parts : List ((Prod (Nat) (Nat)))) -> Bool
  | List.nil => true
  | List.cons head tail => (if (Nat.blt ((head).2) (2)) then atMostOne (tail) else false)

@[expose] public def liveFind : (live : List ((Prod (LexLeanBoot.RustValidate.Id) (Later)))) -> (ident : LexLeanBoot.RustValidate.Id) -> Option (Later)
  | List.nil, _ident => Option.none
  | List.cons head tail, ident => (if idEq ((head).1) (ident) then Option.some ((head).2) else liveFind (tail) (ident))

@[expose] public def liveSet : (live : List ((Prod (LexLeanBoot.RustValidate.Id) (Later)))) -> (ident : LexLeanBoot.RustValidate.Id) -> (later : Later) -> List ((Prod (LexLeanBoot.RustValidate.Id) (Later)))
  | List.nil, ident, later => ((ident, later) :: ([] : List ((Prod (LexLeanBoot.RustValidate.Id) (Later)))))
  | List.cons head tail, ident, later => (if idEq ((head).1) (ident) then ((ident, later) :: tail) else (head :: liveSet (tail) (ident) (later)))

@[expose] public def liveRemove : (live : List ((Prod (LexLeanBoot.RustValidate.Id) (Later)))) -> (ident : LexLeanBoot.RustValidate.Id) -> List ((Prod (LexLeanBoot.RustValidate.Id) (Later)))
  | List.nil, _ident => ([] : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))))
  | List.cons head tail, ident => (if idEq ((head).1) (ident) then tail else (head :: liveRemove (tail) (ident)))

@[expose] public def entryOf (live : List ((Prod (LexLeanBoot.RustValidate.Id) (Later)))) (ident : LexLeanBoot.RustValidate.Id) : Later := (match liveFind (live) (ident) with | Option.some found => found | Option.none => ({ whole := 0, parts := ([] : List ((Prod (Nat) (Nat)))) } : Later))

@[expose] public def mergeLive : (left : List ((Prod (LexLeanBoot.RustValidate.Id) (Later)))) -> (right : List ((Prod (LexLeanBoot.RustValidate.Id) (Later)))) -> List ((Prod (LexLeanBoot.RustValidate.Id) (Later)))
  | left, List.nil => left
  | left, List.cons head tail => (let current : Later := entryOf (left) ((head).1); mergeLive (liveSet (left) ((head).1) (({ whole := natMax ((current).whole) (((head).2).whole), parts := mergeParts ((current).parts) (((head).2).parts) } : Later))) (tail))

@[expose] public def readWhole (live : List ((Prod (LexLeanBoot.RustValidate.Id) (Later)))) (ident : LexLeanBoot.RustValidate.Id) : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := (let current : Later := entryOf (live) (ident); liveSet (live) (ident) (({ whole := ((current).whole + 1), parts := (current).parts } : Later)))

@[expose] public def readPart (live : List ((Prod (LexLeanBoot.RustValidate.Id) (Later)))) (ident : LexLeanBoot.RustValidate.Id) (part : Nat) : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := (let current : Later := entryOf (live) (ident); liveSet (live) (ident) (({ whole := (current).whole, parts := partSet ((current).parts) (part) ((partCount ((current).parts) (part) + 1)) } : Later)))

@[expose] public def mayMove (live : List ((Prod (LexLeanBoot.RustValidate.Id) (Later)))) (ident : LexLeanBoot.RustValidate.Id) : Bool := (match liveFind (live) (ident) with | Option.some found => ((Nat.beq ((found).whole) (0)) && (Nat.beq ((LexLeanRuntime.length ((found).parts) : Nat)) (0))) | Option.none => true)

@[expose] public def mayMovePart (live : List ((Prod (LexLeanBoot.RustValidate.Id) (Later)))) (ident : LexLeanBoot.RustValidate.Id) (part : Nat) : Bool := (match liveFind (live) (ident) with | Option.some found => ((Nat.beq ((found).whole) (0)) && ((Nat.beq (partCount ((found).parts) (part)) (0)) && atMostOne ((found).parts))) | Option.none => true)

@[expose] public def unbind : (pattern : LexLeanBoot.RustAst.Pat) -> (live : List ((Prod (LexLeanBoot.RustValidate.Id) (Later)))) -> List ((Prod (LexLeanBoot.RustValidate.Id) (Later)))
  | LexLeanBoot.RustAst.Pat.PLeaf _, live => live
  | LexLeanBoot.RustAst.Pat.PBind ident, live => liveRemove (live) (ident)
  | LexLeanBoot.RustAst.Pat.PTuple parts, live => unbind (parts) (live)
  | LexLeanBoot.RustAst.Pat.PWrap _ inner, live => unbind (inner) (live)
  | LexLeanBoot.RustAst.Pat.PAdt _ _ parts, live => unbind (parts) (live)
  | LexLeanBoot.RustAst.Pat.PNil, live => live
  | LexLeanBoot.RustAst.Pat.PCons head tail, live => (let one : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := unbind (head) (live); unbind (tail) (one))
  | LexLeanBoot.RustAst.Pat.POrdering _, live => live

@[expose] public def isWild (pattern : LexLeanBoot.RustAst.Pat) : Bool := (match pattern with | LexLeanBoot.RustAst.Pat.PLeaf leaf => (match leaf with | LexLeanBoot.RustAst.PatLeaf.PlWild => true | LexLeanBoot.RustAst.PatLeaf.PlUnit => false | LexLeanBoot.RustAst.PatLeaf.PlNone => false) | LexLeanBoot.RustAst.Pat.PBind _ => false | LexLeanBoot.RustAst.Pat.PTuple _ => false | LexLeanBoot.RustAst.Pat.PWrap _ _ => false | LexLeanBoot.RustAst.Pat.PAdt _ _ _ => false | LexLeanBoot.RustAst.Pat.PNil => false | LexLeanBoot.RustAst.Pat.PCons _ _ => false | LexLeanBoot.RustAst.Pat.POrdering _ => false)

@[expose] public def boundCount : (parts : LexLeanBoot.RustAst.Pat) -> Nat
  | LexLeanBoot.RustAst.Pat.PLeaf _ => 0
  | LexLeanBoot.RustAst.Pat.PBind _ => 0
  | LexLeanBoot.RustAst.Pat.PTuple _ => 0
  | LexLeanBoot.RustAst.Pat.PWrap _ _ => 0
  | LexLeanBoot.RustAst.Pat.PAdt _ _ _ => 0
  | LexLeanBoot.RustAst.Pat.PNil => 0
  | LexLeanBoot.RustAst.Pat.PCons head tail => (if isWild (head) then boundCount (tail) else (1 + boundCount (tail)))
  | LexLeanBoot.RustAst.Pat.POrdering _ => 0

@[expose] public def partsLength : (parts : LexLeanBoot.RustAst.Pat) -> Nat
  | LexLeanBoot.RustAst.Pat.PLeaf _ => 0
  | LexLeanBoot.RustAst.Pat.PBind _ => 0
  | LexLeanBoot.RustAst.Pat.PTuple _ => 0
  | LexLeanBoot.RustAst.Pat.PWrap _ _ => 0
  | LexLeanBoot.RustAst.Pat.PAdt _ _ _ => 0
  | LexLeanBoot.RustAst.Pat.PNil => 0
  | LexLeanBoot.RustAst.Pat.PCons _ tail => (1 + partsLength (tail))
  | LexLeanBoot.RustAst.Pat.POrdering _ => 0

@[expose] public def firstBound : (parts : LexLeanBoot.RustAst.Pat) -> («at» : Nat) -> Nat
  | LexLeanBoot.RustAst.Pat.PLeaf _, «at» => «at»
  | LexLeanBoot.RustAst.Pat.PBind _, «at» => «at»
  | LexLeanBoot.RustAst.Pat.PTuple _, «at» => «at»
  | LexLeanBoot.RustAst.Pat.PWrap _ _, «at» => «at»
  | LexLeanBoot.RustAst.Pat.PAdt _ _ _, «at» => «at»
  | LexLeanBoot.RustAst.Pat.PNil, «at» => «at»
  | LexLeanBoot.RustAst.Pat.PCons head tail, «at» => (if isWild (head) then firstBound (tail) ((«at» + 1)) else «at»)
  | LexLeanBoot.RustAst.Pat.POrdering _, «at» => «at»

@[expose] public def cloneRead (value : LexLeanBoot.RustAst.Expr) : Option ((Prod (LexLeanBoot.RustValidate.Id) (LexLeanBoot.RustValidate.Origin))) := (match value with | LexLeanBoot.RustAst.Expr.ELit _ _ => Option.none | LexLeanBoot.RustAst.Expr.ERead kind ident o => (match kind with | LexLeanBoot.RustValidate.ReadKind.RkClone => Option.some ((ident, o)) | LexLeanBoot.RustValidate.ReadKind.RkMove => Option.none | LexLeanBoot.RustValidate.ReadKind.RkCopy => Option.none | LexLeanBoot.RustValidate.ReadKind.RkDeref => Option.none | LexLeanBoot.RustValidate.ReadKind.RkUnbox => Option.none) | LexLeanBoot.RustAst.Expr.EWrap _ _ _ => Option.none | LexLeanBoot.RustAst.Expr.ECall _ _ _ _ => Option.none | LexLeanBoot.RustAst.Expr.EApply _ _ _ _ => Option.none | LexLeanBoot.RustAst.Expr.EConstruct _ _ _ => Option.none | LexLeanBoot.RustAst.Expr.EPair _ _ _ => Option.none | LexLeanBoot.RustAst.Expr.EIf _ _ _ _ => Option.none | LexLeanBoot.RustAst.Expr.EMatch _ _ _ => Option.none | LexLeanBoot.RustAst.Expr.EBlockE _ => Option.none | LexLeanBoot.RustAst.Expr.EProbe _ _ _ => Option.none | LexLeanBoot.RustAst.Expr.EBlock _ _ => Option.none | LexLeanBoot.RustAst.Expr.ENil => Option.none | LexLeanBoot.RustAst.Expr.ECons _ _ => Option.none | LexLeanBoot.RustAst.Expr.ELet _ _ _ _ _ _ => Option.none | LexLeanBoot.RustAst.Expr.EArm _ _ _ => Option.none | LexLeanBoot.RustAst.Expr.ELoop _ _ => Option.none)

@[expose] public def projectionOf (pattern : LexLeanBoot.RustAst.Pat) (value : LexLeanBoot.RustAst.Expr) : Option ((Prod (Nat) ((Prod (LexLeanBoot.RustValidate.Id) (LexLeanBoot.RustValidate.Origin))))) := (match pattern with | LexLeanBoot.RustAst.Pat.PLeaf _ => Option.none | LexLeanBoot.RustAst.Pat.PBind _ => Option.none | LexLeanBoot.RustAst.Pat.PTuple parts => (match cloneRead (value) with | Option.some found => (if ((Nat.beq (boundCount (parts)) (1)) && (Nat.blt (1) (partsLength (parts)))) then Option.some ((firstBound (parts) (0), found)) else Option.none) | Option.none => Option.none) | LexLeanBoot.RustAst.Pat.PWrap _ _ => Option.none | LexLeanBoot.RustAst.Pat.PAdt _ _ _ => Option.none | LexLeanBoot.RustAst.Pat.PNil => Option.none | LexLeanBoot.RustAst.Pat.PCons _ _ => Option.none | LexLeanBoot.RustAst.Pat.POrdering _ => Option.none)

@[expose] public def isNilExpr (e : LexLeanBoot.RustAst.Expr) : Bool := (match e with | LexLeanBoot.RustAst.Expr.ELit _ _ => false | LexLeanBoot.RustAst.Expr.ERead _ _ _ => false | LexLeanBoot.RustAst.Expr.EWrap _ _ _ => false | LexLeanBoot.RustAst.Expr.ECall _ _ _ _ => false | LexLeanBoot.RustAst.Expr.EApply _ _ _ _ => false | LexLeanBoot.RustAst.Expr.EConstruct _ _ _ => false | LexLeanBoot.RustAst.Expr.EPair _ _ _ => false | LexLeanBoot.RustAst.Expr.EIf _ _ _ _ => false | LexLeanBoot.RustAst.Expr.EMatch _ _ _ => false | LexLeanBoot.RustAst.Expr.EBlockE _ => false | LexLeanBoot.RustAst.Expr.EProbe _ _ _ => false | LexLeanBoot.RustAst.Expr.EBlock _ _ => false | LexLeanBoot.RustAst.Expr.ENil => true | LexLeanBoot.RustAst.Expr.ECons _ _ => false | LexLeanBoot.RustAst.Expr.ELet _ _ _ _ _ _ => false | LexLeanBoot.RustAst.Expr.EArm _ _ _ => false | LexLeanBoot.RustAst.Expr.ELoop _ _ => false)

@[expose] public def isCloneKind (kind : LexLeanBoot.RustValidate.ReadKind) : Bool := (match kind with | LexLeanBoot.RustValidate.ReadKind.RkClone => true | LexLeanBoot.RustValidate.ReadKind.RkMove => false | LexLeanBoot.RustValidate.ReadKind.RkCopy => false | LexLeanBoot.RustValidate.ReadKind.RkDeref => false | LexLeanBoot.RustValidate.ReadKind.RkUnbox => false)

@[expose] public def readsE : (e : LexLeanBoot.RustAst.Expr) -> (live : List ((Prod (LexLeanBoot.RustValidate.Id) (Later)))) -> (mode : Nat) -> List ((Prod (LexLeanBoot.RustValidate.Id) (Later)))
  | LexLeanBoot.RustAst.Expr.ELit _ _, live, _mode => live
  | LexLeanBoot.RustAst.Expr.ERead _ ident _, live, _mode => readWhole (live) (ident)
  | LexLeanBoot.RustAst.Expr.EWrap _ inner _, live, _mode => readsE (inner) (live) (0)
  | LexLeanBoot.RustAst.Expr.ECall _ args _ _, live, _mode => readsE (args) (live) (0)
  | LexLeanBoot.RustAst.Expr.EApply holder args _ _, live, _mode => (let held : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := readWhole (live) (holder); readsE (args) (held) (0))
  | LexLeanBoot.RustAst.Expr.EConstruct _ args _, live, _mode => readsE (args) (live) (0)
  | LexLeanBoot.RustAst.Expr.EPair left right _, live, _mode => (let after : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := readsE (right) (live) (0); readsE (left) (after) (0))
  | LexLeanBoot.RustAst.Expr.EIf condition thenBranch elseBranch _, live, _mode => (let one : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := readsE (thenBranch) (live) (0); (let two : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := readsE (elseBranch) (live) (0); readsE (condition) (mergeLive (one) (two)) (0)))
  | LexLeanBoot.RustAst.Expr.EMatch scrutinee arms _, live, _mode => (let through : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := (if isNilExpr (arms) then live else readsE (arms) (live) (1)); readsE (scrutinee) (through) (0))
  | LexLeanBoot.RustAst.Expr.EBlockE inner, live, _mode => readsE (inner) (live) (0)
  | LexLeanBoot.RustAst.Expr.EProbe _ ident _, live, _mode => readWhole (live) (ident)
  | LexLeanBoot.RustAst.Expr.EBlock lets tail, live, _mode => (let after : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := readsE (tail) (live) (0); readsE (lets) (after) (0))
  | LexLeanBoot.RustAst.Expr.ENil, live, mode => (if (Nat.beq (mode) (1)) then ([] : List ((Prod (LexLeanBoot.RustValidate.Id) (Later)))) else live)
  | LexLeanBoot.RustAst.Expr.ECons head tail, live, _mode => (let after : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := readsE (tail) (live) (0); readsE (head) (after) (0))
  | LexLeanBoot.RustAst.Expr.ELet pat _ _ value _ rest, live, _mode => (let inner : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := readsE (rest) (live) (0); (let bound : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := unbind (pat) (inner); (match projectionOf (pat) (value) with | Option.some found => readPart (bound) (((found).2).1) ((found).1) | Option.none => readsE (value) (bound) (0))))
  | LexLeanBoot.RustAst.Expr.EArm pat body rest, live, _mode => (let seen : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := readsE (body) (live) (0); (let later : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := readsE (rest) (live) (1); mergeLive (unbind (pat) (seen)) (later)))
  | LexLeanBoot.RustAst.Expr.ELoop state body, live, _mode => (let bare : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := readsE (body) (([] : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))))) (0); (let reads : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := liveRemove (bare) (state); (let inside : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := liveRemove (mergeLive (live) (reads)) (state); (let ran : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := readsE (body) (inside) (0); readWhole (ran) (state)))))

@[expose] public def moveE : (e : LexLeanBoot.RustAst.Expr) -> (live : List ((Prod (LexLeanBoot.RustValidate.Id) (Later)))) -> LexLeanBoot.RustAst.Expr
  | LexLeanBoot.RustAst.Expr.ELit lit o, _live => LexLeanBoot.RustAst.Expr.ELit (lit) (o)
  | LexLeanBoot.RustAst.Expr.ERead kind ident o, live => (if isCloneKind (kind) then (if mayMove (live) (ident) then LexLeanBoot.RustAst.Expr.ERead (LexLeanBoot.RustValidate.ReadKind.RkMove) (ident) (o) else LexLeanBoot.RustAst.Expr.ERead (kind) (ident) (o)) else LexLeanBoot.RustAst.Expr.ERead (kind) (ident) (o))
  | LexLeanBoot.RustAst.Expr.EWrap kind inner o, live => LexLeanBoot.RustAst.Expr.EWrap (kind) (moveE (inner) (live)) (o)
  | LexLeanBoot.RustAst.Expr.ECall callee args propagate o, live => LexLeanBoot.RustAst.Expr.ECall (callee) (moveE (args) (live)) (propagate) (o)
  | LexLeanBoot.RustAst.Expr.EApply holder args propagate o, live => (let held : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := readWhole (live) (holder); LexLeanBoot.RustAst.Expr.EApply (holder) (moveE (args) (held)) (propagate) (o))
  | LexLeanBoot.RustAst.Expr.EConstruct con args o, live => LexLeanBoot.RustAst.Expr.EConstruct (con) (moveE (args) (live)) (o)
  | LexLeanBoot.RustAst.Expr.EPair left right o, live => (let after : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := readsE (right) (live) (0); LexLeanBoot.RustAst.Expr.EPair (moveE (left) (after)) (moveE (right) (live)) (o))
  | LexLeanBoot.RustAst.Expr.EIf condition thenBranch elseBranch o, live => (let one : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := readsE (thenBranch) (live) (0); (let two : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := readsE (elseBranch) (live) (0); LexLeanBoot.RustAst.Expr.EIf (moveE (condition) (mergeLive (one) (two))) (moveE (thenBranch) (live)) (moveE (elseBranch) (live)) (o)))
  | LexLeanBoot.RustAst.Expr.EMatch scrutinee arms o, live => (let through : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := (if isNilExpr (arms) then live else readsE (arms) (live) (1)); LexLeanBoot.RustAst.Expr.EMatch (moveE (scrutinee) (through)) (moveE (arms) (live)) (o))
  | LexLeanBoot.RustAst.Expr.EBlockE inner, live => LexLeanBoot.RustAst.Expr.EBlockE (moveE (inner) (live))
  | LexLeanBoot.RustAst.Expr.EProbe kind ident o, _live => LexLeanBoot.RustAst.Expr.EProbe (kind) (ident) (o)
  | LexLeanBoot.RustAst.Expr.EBlock lets tail, live => (let after : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := readsE (tail) (live) (0); LexLeanBoot.RustAst.Expr.EBlock (moveE (lets) (after)) (moveE (tail) (live)))
  | LexLeanBoot.RustAst.Expr.ENil, _live => LexLeanBoot.RustAst.Expr.ENil
  | LexLeanBoot.RustAst.Expr.ECons head tail, live => (let after : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := readsE (tail) (live) (0); LexLeanBoot.RustAst.Expr.ECons (moveE (head) (after)) (moveE (tail) (live)))
  | LexLeanBoot.RustAst.Expr.ELet pat mutable ty value o rest, live => (let inner : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := readsE (rest) (live) (0); (let bound : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := unbind (pat) (inner); LexLeanBoot.RustAst.Expr.ELet (pat) (mutable) (ty) ((match projectionOf (pat) (value) with | Option.some found => (if mayMovePart (bound) (((found).2).1) ((found).1) then LexLeanBoot.RustAst.Expr.ERead (LexLeanBoot.RustValidate.ReadKind.RkMove) (((found).2).1) (((found).2).2) else value) | Option.none => moveE (value) (bound))) (o) (moveE (rest) (live))))
  | LexLeanBoot.RustAst.Expr.EArm pat body rest, live => LexLeanBoot.RustAst.Expr.EArm (pat) (moveE (body) (live)) (moveE (rest) (live))
  | LexLeanBoot.RustAst.Expr.ELoop state body, live => (let bare : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := readsE (body) (([] : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))))) (0); (let reads : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := liveRemove (bare) (state); (let inside : List ((Prod (LexLeanBoot.RustValidate.Id) (Later))) := liveRemove (mergeLive (live) (reads)) (state); LexLeanBoot.RustAst.Expr.ELoop (state) (moveE (body) (inside)))))

@[expose] public def moveItems : (items : LexLeanBoot.RustAst.Def) -> LexLeanBoot.RustAst.Def
  | LexLeanBoot.RustAst.Def.DEnum ty row o => LexLeanBoot.RustAst.Def.DEnum (ty) (row) (o)
  | LexLeanBoot.RustAst.Def.DApply number parameters result fallible dispatch o => LexLeanBoot.RustAst.Def.DApply (number) (parameters) (result) (fallible) (dispatch) (o)
  | LexLeanBoot.RustAst.Def.DFunction name parameters result body o => LexLeanBoot.RustAst.Def.DFunction (name) (parameters) (result) (moveE (body) (([] : List ((Prod (LexLeanBoot.RustValidate.Id) (Later)))))) (o)
  | LexLeanBoot.RustAst.Def.DNil => LexLeanBoot.RustAst.Def.DNil
  | LexLeanBoot.RustAst.Def.DCons item rest => LexLeanBoot.RustAst.Def.DCons (moveItems (item)) (moveItems (rest))

@[expose] public def applyMoves (items : LexLeanBoot.RustAst.Def) : LexLeanBoot.RustAst.Def := moveItems (items)

end LexLeanBoot.RustMoves
