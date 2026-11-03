module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanBoot.CalculusCheck
public import LexLeanBoot.RustAst
public import LexLeanBoot.RustBodies
public import LexLeanBoot.RustValidate
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.RustLoops

public structure LoopOut where
  found : Bool
  params : LexLeanBoot.RustAst.Par
  body : LexLeanBoot.RustAst.Expr
  next : Nat

public structure Tails where
  ok : Bool
  calls : LexLeanBoot.RustAst.Expr

public structure Parts where
  ok : Bool
  count : Nat
  ty : LexLeanBoot.RustValidate.Ty
  tuple : LexLeanBoot.RustAst.Pat
  start : LexLeanBoot.RustAst.Expr
  renamed : LexLeanBoot.RustAst.Par

public structure Retarget where
  me : Nat
  params : LexLeanBoot.RustAst.Par
  calls : LexLeanBoot.RustAst.Expr
  state : LexLeanBoot.RustValidate.Ty
  done : LexLeanBoot.RustValidate.Ty

@[expose] public def callAt (_seed : Nat) : LexLeanBoot.RustValidate.Origin := ({ element := (LexLeanRuntime.utf8Encode ("expr:call") : ByteArray), width := Option.none } : LexLeanBoot.RustValidate.Origin)

@[expose] public def occurs : (subject : LexLeanBoot.RustAst.Expr) -> (me : Nat) -> Bool
  | LexLeanBoot.RustAst.Expr.ELit _ _, _me => false
  | LexLeanBoot.RustAst.Expr.ERead _ _ _, _me => false
  | LexLeanBoot.RustAst.Expr.EWrap _ inner _, me => occurs (inner) (me)
  | LexLeanBoot.RustAst.Expr.ECall callee args _ _, me => (match callee with | LexLeanBoot.RustAst.Callee.CalFn called => (if (Nat.beq (called) (me)) then true else occurs (args) (me)) | LexLeanBoot.RustAst.Callee.CalRt _ => occurs (args) (me) | LexLeanBoot.RustAst.Callee.CalMachine _ _ => occurs (args) (me))
  | LexLeanBoot.RustAst.Expr.EApply _ args _ _, me => occurs (args) (me)
  | LexLeanBoot.RustAst.Expr.EConstruct _ args _, me => occurs (args) (me)
  | LexLeanBoot.RustAst.Expr.EPair left right _, me => (if occurs (left) (me) then true else occurs (right) (me))
  | LexLeanBoot.RustAst.Expr.EIf c t e _, me => (if occurs (c) (me) then true else (if occurs (t) (me) then true else occurs (e) (me)))
  | LexLeanBoot.RustAst.Expr.EMatch s arms _, me => (if occurs (s) (me) then true else occurs (arms) (me))
  | LexLeanBoot.RustAst.Expr.EBlockE inner, me => occurs (inner) (me)
  | LexLeanBoot.RustAst.Expr.EProbe _ _ _, _me => false
  | LexLeanBoot.RustAst.Expr.EBlock lets tail, me => (if occurs (lets) (me) then true else occurs (tail) (me))
  | LexLeanBoot.RustAst.Expr.ENil, _me => false
  | LexLeanBoot.RustAst.Expr.ECons head tail, me => (if occurs (head) (me) then true else occurs (tail) (me))
  | LexLeanBoot.RustAst.Expr.ELet _ _ _ value _ rest, me => (if occurs (value) (me) then true else occurs (rest) (me))
  | LexLeanBoot.RustAst.Expr.EArm _ block rest, me => (if occurs (block) (me) then true else occurs (rest) (me))
  | LexLeanBoot.RustAst.Expr.ELoop _ body, me => occurs (body) (me)

@[expose] public def appendCalls : (left : LexLeanBoot.RustAst.Expr) -> (right : LexLeanBoot.RustAst.Expr) -> LexLeanBoot.RustAst.Expr
  | LexLeanBoot.RustAst.Expr.ELit _ _, right => right
  | LexLeanBoot.RustAst.Expr.ERead _ _ _, right => right
  | LexLeanBoot.RustAst.Expr.EWrap _ _ _, right => right
  | LexLeanBoot.RustAst.Expr.ECall _ _ _ _, right => right
  | LexLeanBoot.RustAst.Expr.EApply _ _ _ _, right => right
  | LexLeanBoot.RustAst.Expr.EConstruct _ _ _, right => right
  | LexLeanBoot.RustAst.Expr.EPair _ _ _, right => right
  | LexLeanBoot.RustAst.Expr.EIf _ _ _ _, right => right
  | LexLeanBoot.RustAst.Expr.EMatch _ _ _, right => right
  | LexLeanBoot.RustAst.Expr.EBlockE _, right => right
  | LexLeanBoot.RustAst.Expr.EProbe _ _ _, right => right
  | LexLeanBoot.RustAst.Expr.EBlock _ _, right => right
  | LexLeanBoot.RustAst.Expr.ENil, right => right
  | LexLeanBoot.RustAst.Expr.ECons head tail, right => LexLeanBoot.RustAst.Expr.ECons (head) (appendCalls (tail) (right))
  | LexLeanBoot.RustAst.Expr.ELet _ _ _ _ _ _, right => right
  | LexLeanBoot.RustAst.Expr.EArm _ _ _, right => right
  | LexLeanBoot.RustAst.Expr.ELoop _ _, right => right

@[expose] public def tails : (subject : LexLeanBoot.RustAst.Expr) -> (whole : LexLeanBoot.RustAst.Expr) -> (me : Nat) -> (mode : Nat) -> Tails
  | LexLeanBoot.RustAst.Expr.ELit _ _, whole, me, mode => (if (Nat.beq (mode) (0)) then (if occurs (whole) (me) then ({ ok := false, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails) else ({ ok := true, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails)) else ({ ok := false, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails))
  | LexLeanBoot.RustAst.Expr.ERead _ _ _, whole, me, mode => (if (Nat.beq (mode) (0)) then (if occurs (whole) (me) then ({ ok := false, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails) else ({ ok := true, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails)) else ({ ok := false, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails))
  | LexLeanBoot.RustAst.Expr.EWrap _ _ _, whole, me, mode => (if (Nat.beq (mode) (0)) then (if occurs (whole) (me) then ({ ok := false, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails) else ({ ok := true, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails)) else ({ ok := false, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails))
  | LexLeanBoot.RustAst.Expr.ECall callee args _ _, whole, me, _mode => (match callee with | LexLeanBoot.RustAst.Callee.CalFn called => (if (Nat.beq (called) (me)) then (if occurs (args) (me) then ({ ok := false, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails) else ({ ok := true, calls := LexLeanBoot.RustAst.Expr.ECons (args) (LexLeanBoot.RustAst.Expr.ENil) } : Tails)) else (if occurs (whole) (me) then ({ ok := false, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails) else ({ ok := true, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails))) | LexLeanBoot.RustAst.Callee.CalRt _ => (if occurs (whole) (me) then ({ ok := false, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails) else ({ ok := true, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails)) | LexLeanBoot.RustAst.Callee.CalMachine _ _ => (if occurs (whole) (me) then ({ ok := false, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails) else ({ ok := true, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails)))
  | LexLeanBoot.RustAst.Expr.EApply _ _ _ _, whole, me, mode => (if (Nat.beq (mode) (0)) then (if occurs (whole) (me) then ({ ok := false, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails) else ({ ok := true, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails)) else ({ ok := false, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails))
  | LexLeanBoot.RustAst.Expr.EConstruct _ _ _, whole, me, mode => (if (Nat.beq (mode) (0)) then (if occurs (whole) (me) then ({ ok := false, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails) else ({ ok := true, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails)) else ({ ok := false, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails))
  | LexLeanBoot.RustAst.Expr.EPair _ _ _, whole, me, mode => (if (Nat.beq (mode) (0)) then (if occurs (whole) (me) then ({ ok := false, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails) else ({ ok := true, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails)) else ({ ok := false, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails))
  | LexLeanBoot.RustAst.Expr.EIf _ t e _, _whole, me, _mode => (let one : Tails := tails (t) (t) (me) (1); (let two : Tails := tails (e) (e) (me) (1); ({ ok := ((one).ok && (two).ok), calls := appendCalls ((one).calls) ((two).calls) } : Tails)))
  | LexLeanBoot.RustAst.Expr.EMatch _ arms _, _whole, me, _mode => (let all : Tails := tails (arms) (arms) (me) (2); ({ ok := (all).ok, calls := (all).calls } : Tails))
  | LexLeanBoot.RustAst.Expr.EBlockE _, whole, me, mode => (if (Nat.beq (mode) (0)) then (if occurs (whole) (me) then ({ ok := false, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails) else ({ ok := true, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails)) else ({ ok := false, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails))
  | LexLeanBoot.RustAst.Expr.EProbe _ _ _, whole, me, mode => (if (Nat.beq (mode) (0)) then (if occurs (whole) (me) then ({ ok := false, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails) else ({ ok := true, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails)) else ({ ok := false, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails))
  | LexLeanBoot.RustAst.Expr.EBlock lets tail, _whole, me, _mode => (let last : Tails := tails (tail) (tail) (me) (0); ({ ok := ((!occurs (lets) (me)) && (last).ok), calls := (last).calls } : Tails))
  | LexLeanBoot.RustAst.Expr.ENil, _whole, _me, _mode => ({ ok := true, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails)
  | LexLeanBoot.RustAst.Expr.ECons _ _, whole, me, mode => (if (Nat.beq (mode) (0)) then (if occurs (whole) (me) then ({ ok := false, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails) else ({ ok := true, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails)) else ({ ok := false, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails))
  | LexLeanBoot.RustAst.Expr.ELet _ _ _ _ _ _, whole, me, mode => (if (Nat.beq (mode) (0)) then (if occurs (whole) (me) then ({ ok := false, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails) else ({ ok := true, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails)) else ({ ok := false, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails))
  | LexLeanBoot.RustAst.Expr.EArm _ block rest, _whole, me, _mode => (let one : Tails := tails (block) (block) (me) (1); (let two : Tails := tails (rest) (rest) (me) (2); ({ ok := ((one).ok && (two).ok), calls := appendCalls ((one).calls) ((two).calls) } : Tails)))
  | LexLeanBoot.RustAst.Expr.ELoop _ _, whole, me, mode => (if (Nat.beq (mode) (0)) then (if occurs (whole) (me) then ({ ok := false, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails) else ({ ok := true, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails)) else ({ ok := false, calls := LexLeanBoot.RustAst.Expr.ENil } : Tails))

@[expose] public def passes (argument : LexLeanBoot.RustAst.Expr) (name : LexLeanBoot.RustValidate.Id) : Bool := (match argument with | LexLeanBoot.RustAst.Expr.ELit _ _ => false | LexLeanBoot.RustAst.Expr.ERead kind read _ => (match kind with | LexLeanBoot.RustValidate.ReadKind.RkMove => LexLeanBoot.RustBodies.idEq (read) (name) | LexLeanBoot.RustValidate.ReadKind.RkClone => LexLeanBoot.RustBodies.idEq (read) (name) | LexLeanBoot.RustValidate.ReadKind.RkCopy => LexLeanBoot.RustBodies.idEq (read) (name) | LexLeanBoot.RustValidate.ReadKind.RkDeref => LexLeanBoot.RustBodies.idEq (read) (name) | LexLeanBoot.RustValidate.ReadKind.RkUnbox => false) | LexLeanBoot.RustAst.Expr.EWrap _ _ _ => false | LexLeanBoot.RustAst.Expr.ECall _ _ _ _ => false | LexLeanBoot.RustAst.Expr.EApply _ _ _ _ => false | LexLeanBoot.RustAst.Expr.EConstruct _ _ _ => false | LexLeanBoot.RustAst.Expr.EPair _ _ _ => false | LexLeanBoot.RustAst.Expr.EIf _ _ _ _ => false | LexLeanBoot.RustAst.Expr.EMatch _ _ _ => false | LexLeanBoot.RustAst.Expr.EBlockE _ => false | LexLeanBoot.RustAst.Expr.EProbe _ _ _ => false | LexLeanBoot.RustAst.Expr.EBlock _ _ => false | LexLeanBoot.RustAst.Expr.ENil => false | LexLeanBoot.RustAst.Expr.ECons _ _ => false | LexLeanBoot.RustAst.Expr.ELet _ _ _ _ _ _ => false | LexLeanBoot.RustAst.Expr.EArm _ _ _ => false | LexLeanBoot.RustAst.Expr.ELoop _ _ => false)

@[expose] public def passesAll : (calls : LexLeanBoot.RustAst.Expr) -> (position : Nat) -> (name : LexLeanBoot.RustValidate.Id) -> Bool
  | LexLeanBoot.RustAst.Expr.ELit _ _, _position, _name => true
  | LexLeanBoot.RustAst.Expr.ERead _ _ _, _position, _name => true
  | LexLeanBoot.RustAst.Expr.EWrap _ _ _, _position, _name => true
  | LexLeanBoot.RustAst.Expr.ECall _ _ _ _, _position, _name => true
  | LexLeanBoot.RustAst.Expr.EApply _ _ _ _, _position, _name => true
  | LexLeanBoot.RustAst.Expr.EConstruct _ _ _, _position, _name => true
  | LexLeanBoot.RustAst.Expr.EPair _ _ _, _position, _name => true
  | LexLeanBoot.RustAst.Expr.EIf _ _ _ _, _position, _name => true
  | LexLeanBoot.RustAst.Expr.EMatch _ _ _, _position, _name => true
  | LexLeanBoot.RustAst.Expr.EBlockE _, _position, _name => true
  | LexLeanBoot.RustAst.Expr.EProbe _ _ _, _position, _name => true
  | LexLeanBoot.RustAst.Expr.EBlock _ _, _position, _name => true
  | LexLeanBoot.RustAst.Expr.ENil, _position, _name => true
  | LexLeanBoot.RustAst.Expr.ECons args rest, position, name => (if passes (LexLeanBoot.RustBodies.argAt (args) (position)) (name) then passesAll (rest) (position) (name) else false)
  | LexLeanBoot.RustAst.Expr.ELet _ _ _ _ _ _, _position, _name => true
  | LexLeanBoot.RustAst.Expr.EArm _ _ _, _position, _name => true
  | LexLeanBoot.RustAst.Expr.ELoop _ _, _position, _name => true

@[expose] public def isChanged (subject : LexLeanBoot.RustAst.Pat) (calls : LexLeanBoot.RustAst.Expr) (position : Nat) : Bool := (match subject with | LexLeanBoot.RustAst.Pat.PLeaf _ => false | LexLeanBoot.RustAst.Pat.PBind name => (!passesAll (calls) (position) (name)) | LexLeanBoot.RustAst.Pat.PTuple _ => false | LexLeanBoot.RustAst.Pat.PWrap _ _ => false | LexLeanBoot.RustAst.Pat.PAdt _ _ _ => false | LexLeanBoot.RustAst.Pat.PNil => false | LexLeanBoot.RustAst.Pat.PCons _ _ => false | LexLeanBoot.RustAst.Pat.POrdering _ => false)

@[expose] public def partsCons (subject : LexLeanBoot.RustAst.Pat) (whole : LexLeanBoot.RustAst.Pat) (ty : LexLeanBoot.RustValidate.Ty) (more : Parts) (changed : Bool) (made : Nat) : Parts := (match subject with | LexLeanBoot.RustAst.Pat.PLeaf _ => ({ ok := false, count := 0, ty := LexLeanBoot.RustValidate.Ty.TyNil, tuple := LexLeanBoot.RustAst.Pat.PNil, start := LexLeanBoot.RustAst.Expr.ENil, renamed := LexLeanBoot.RustAst.Par.ParNil } : Parts) | LexLeanBoot.RustAst.Pat.PBind _ => (if changed then ({ ok := (more).ok, count := ((more).count + 1), ty := (if (Nat.beq ((more).count) (0)) then ty else LexLeanBoot.RustValidate.Ty.TyBinary (LexLeanBoot.RustValidate.BinaryKind.BkPair) (ty) ((more).ty)), tuple := (if (Nat.beq ((more).count) (0)) then whole else LexLeanBoot.RustAst.Pat.PTuple (LexLeanBoot.RustAst.Pat.PCons (whole) (LexLeanBoot.RustAst.Pat.PCons ((more).tuple) (LexLeanBoot.RustAst.Pat.PNil)))), start := (if (Nat.beq ((more).count) (0)) then LexLeanBoot.RustAst.Expr.ERead (LexLeanBoot.RustValidate.ReadKind.RkMove) (LexLeanBoot.RustValidate.Id.IdParam (made)) (callAt (0)) else LexLeanBoot.RustAst.Expr.EPair (LexLeanBoot.RustAst.Expr.ERead (LexLeanBoot.RustValidate.ReadKind.RkMove) (LexLeanBoot.RustValidate.Id.IdParam (made)) (callAt (0))) ((more).start) (callAt (0))), renamed := LexLeanBoot.RustAst.Par.ParCons (LexLeanBoot.RustAst.Pat.PBind (LexLeanBoot.RustValidate.Id.IdParam (made))) (ty) ((more).renamed) } : Parts) else ({ ok := (more).ok, count := (more).count, ty := (more).ty, tuple := (more).tuple, start := (more).start, renamed := LexLeanBoot.RustAst.Par.ParCons (whole) (ty) ((more).renamed) } : Parts)) | LexLeanBoot.RustAst.Pat.PTuple _ => ({ ok := false, count := 0, ty := LexLeanBoot.RustValidate.Ty.TyNil, tuple := LexLeanBoot.RustAst.Pat.PNil, start := LexLeanBoot.RustAst.Expr.ENil, renamed := LexLeanBoot.RustAst.Par.ParNil } : Parts) | LexLeanBoot.RustAst.Pat.PWrap _ _ => ({ ok := false, count := 0, ty := LexLeanBoot.RustValidate.Ty.TyNil, tuple := LexLeanBoot.RustAst.Pat.PNil, start := LexLeanBoot.RustAst.Expr.ENil, renamed := LexLeanBoot.RustAst.Par.ParNil } : Parts) | LexLeanBoot.RustAst.Pat.PAdt _ _ _ => ({ ok := false, count := 0, ty := LexLeanBoot.RustValidate.Ty.TyNil, tuple := LexLeanBoot.RustAst.Pat.PNil, start := LexLeanBoot.RustAst.Expr.ENil, renamed := LexLeanBoot.RustAst.Par.ParNil } : Parts) | LexLeanBoot.RustAst.Pat.PNil => ({ ok := false, count := 0, ty := LexLeanBoot.RustValidate.Ty.TyNil, tuple := LexLeanBoot.RustAst.Pat.PNil, start := LexLeanBoot.RustAst.Expr.ENil, renamed := LexLeanBoot.RustAst.Par.ParNil } : Parts) | LexLeanBoot.RustAst.Pat.PCons _ _ => ({ ok := false, count := 0, ty := LexLeanBoot.RustValidate.Ty.TyNil, tuple := LexLeanBoot.RustAst.Pat.PNil, start := LexLeanBoot.RustAst.Expr.ENil, renamed := LexLeanBoot.RustAst.Par.ParNil } : Parts) | LexLeanBoot.RustAst.Pat.POrdering _ => ({ ok := false, count := 0, ty := LexLeanBoot.RustValidate.Ty.TyNil, tuple := LexLeanBoot.RustAst.Pat.PNil, start := LexLeanBoot.RustAst.Expr.ENil, renamed := LexLeanBoot.RustAst.Par.ParNil } : Parts))

@[expose] public def parts : (params : LexLeanBoot.RustAst.Par) -> (calls : LexLeanBoot.RustAst.Expr) -> (position : Nat) -> (start : Nat) -> (j : Nat) -> Parts
  | LexLeanBoot.RustAst.Par.ParNil, _calls, _position, _start, _j => ({ ok := true, count := 0, ty := LexLeanBoot.RustValidate.Ty.TyNil, tuple := LexLeanBoot.RustAst.Pat.PNil, start := LexLeanBoot.RustAst.Expr.ENil, renamed := LexLeanBoot.RustAst.Par.ParNil } : Parts)
  | LexLeanBoot.RustAst.Par.ParCons pat ty rest, calls, position, start, j => (let changed : Bool := isChanged (pat) (calls) (position); partsCons (pat) (pat) (ty) (parts (rest) (calls) ((position + 1)) (start) ((if changed then (j + 1) else j))) (changed) ((start + (2 + j))))

@[expose] public def nextHere (changed : Bool) (args : LexLeanBoot.RustAst.Expr) (position : Nat) (more : LexLeanBoot.RustAst.Expr) («at» : LexLeanBoot.RustValidate.Origin) : LexLeanBoot.RustAst.Expr := (if changed then (if LexLeanBoot.RustBodies.isNilE (more) then LexLeanBoot.RustBodies.argAt (args) (position) else LexLeanBoot.RustAst.Expr.EPair (LexLeanBoot.RustBodies.argAt (args) (position)) (more) («at»)) else more)

@[expose] public def nextState : (params : LexLeanBoot.RustAst.Par) -> (calls : LexLeanBoot.RustAst.Expr) -> (args : LexLeanBoot.RustAst.Expr) -> (position : Nat) -> («at» : LexLeanBoot.RustValidate.Origin) -> LexLeanBoot.RustAst.Expr
  | LexLeanBoot.RustAst.Par.ParNil, _calls, _args, _position, _at => LexLeanBoot.RustAst.Expr.ENil
  | LexLeanBoot.RustAst.Par.ParCons pat _ rest, calls, args, position, «at» => nextHere (isChanged (pat) (calls) (position)) (args) (position) (nextState (rest) (calls) (args) ((position + 1)) («at»)) («at»)

@[expose] public def retarget : (subject : LexLeanBoot.RustAst.Expr) -> (whole : LexLeanBoot.RustAst.Expr) -> (plan : Retarget) -> (mode : Nat) -> LexLeanBoot.RustAst.Expr
  | LexLeanBoot.RustAst.Expr.ELit _ _, whole, plan, mode => (if (Nat.beq (mode) (0)) then LexLeanBoot.RustAst.Expr.EConstruct (LexLeanBoot.RustValidate.Con.ConErr ((plan).state) ((plan).done)) (LexLeanBoot.RustAst.Expr.ECons (whole) (LexLeanBoot.RustAst.Expr.ENil)) (callAt (0)) else whole)
  | LexLeanBoot.RustAst.Expr.ERead _ _ _, whole, plan, mode => (if (Nat.beq (mode) (0)) then LexLeanBoot.RustAst.Expr.EConstruct (LexLeanBoot.RustValidate.Con.ConErr ((plan).state) ((plan).done)) (LexLeanBoot.RustAst.Expr.ECons (whole) (LexLeanBoot.RustAst.Expr.ENil)) (callAt (0)) else whole)
  | LexLeanBoot.RustAst.Expr.EWrap _ _ _, whole, plan, mode => (if (Nat.beq (mode) (0)) then LexLeanBoot.RustAst.Expr.EConstruct (LexLeanBoot.RustValidate.Con.ConErr ((plan).state) ((plan).done)) (LexLeanBoot.RustAst.Expr.ECons (whole) (LexLeanBoot.RustAst.Expr.ENil)) (callAt (0)) else whole)
  | LexLeanBoot.RustAst.Expr.ECall callee args _ «at», whole, plan, _mode => (match callee with | LexLeanBoot.RustAst.Callee.CalFn called => (if (Nat.beq (called) ((plan).me)) then LexLeanBoot.RustAst.Expr.EConstruct (LexLeanBoot.RustValidate.Con.ConOk ((plan).state) ((plan).done)) (LexLeanBoot.RustAst.Expr.ECons (nextState ((plan).params) ((plan).calls) (args) (0) («at»)) (LexLeanBoot.RustAst.Expr.ENil)) («at») else LexLeanBoot.RustAst.Expr.EConstruct (LexLeanBoot.RustValidate.Con.ConErr ((plan).state) ((plan).done)) (LexLeanBoot.RustAst.Expr.ECons (whole) (LexLeanBoot.RustAst.Expr.ENil)) (callAt (0))) | LexLeanBoot.RustAst.Callee.CalRt _ => LexLeanBoot.RustAst.Expr.EConstruct (LexLeanBoot.RustValidate.Con.ConErr ((plan).state) ((plan).done)) (LexLeanBoot.RustAst.Expr.ECons (whole) (LexLeanBoot.RustAst.Expr.ENil)) (callAt (0)) | LexLeanBoot.RustAst.Callee.CalMachine _ _ => LexLeanBoot.RustAst.Expr.EConstruct (LexLeanBoot.RustValidate.Con.ConErr ((plan).state) ((plan).done)) (LexLeanBoot.RustAst.Expr.ECons (whole) (LexLeanBoot.RustAst.Expr.ENil)) (callAt (0)))
  | LexLeanBoot.RustAst.Expr.EApply _ _ _ _, whole, plan, mode => (if (Nat.beq (mode) (0)) then LexLeanBoot.RustAst.Expr.EConstruct (LexLeanBoot.RustValidate.Con.ConErr ((plan).state) ((plan).done)) (LexLeanBoot.RustAst.Expr.ECons (whole) (LexLeanBoot.RustAst.Expr.ENil)) (callAt (0)) else whole)
  | LexLeanBoot.RustAst.Expr.EConstruct _ _ _, whole, plan, mode => (if (Nat.beq (mode) (0)) then LexLeanBoot.RustAst.Expr.EConstruct (LexLeanBoot.RustValidate.Con.ConErr ((plan).state) ((plan).done)) (LexLeanBoot.RustAst.Expr.ECons (whole) (LexLeanBoot.RustAst.Expr.ENil)) (callAt (0)) else whole)
  | LexLeanBoot.RustAst.Expr.EPair _ _ _, whole, plan, mode => (if (Nat.beq (mode) (0)) then LexLeanBoot.RustAst.Expr.EConstruct (LexLeanBoot.RustValidate.Con.ConErr ((plan).state) ((plan).done)) (LexLeanBoot.RustAst.Expr.ECons (whole) (LexLeanBoot.RustAst.Expr.ENil)) (callAt (0)) else whole)
  | LexLeanBoot.RustAst.Expr.EIf c t e «at», _whole, plan, _mode => LexLeanBoot.RustAst.Expr.EIf (c) (retarget (t) (t) (plan) (1)) (retarget (e) (e) (plan) (1)) («at»)
  | LexLeanBoot.RustAst.Expr.EMatch s arms «at», _whole, plan, _mode => LexLeanBoot.RustAst.Expr.EMatch (s) (retarget (arms) (arms) (plan) (2)) («at»)
  | LexLeanBoot.RustAst.Expr.EBlockE _, whole, plan, mode => (if (Nat.beq (mode) (0)) then LexLeanBoot.RustAst.Expr.EConstruct (LexLeanBoot.RustValidate.Con.ConErr ((plan).state) ((plan).done)) (LexLeanBoot.RustAst.Expr.ECons (whole) (LexLeanBoot.RustAst.Expr.ENil)) (callAt (0)) else whole)
  | LexLeanBoot.RustAst.Expr.EProbe _ _ _, whole, plan, mode => (if (Nat.beq (mode) (0)) then LexLeanBoot.RustAst.Expr.EConstruct (LexLeanBoot.RustValidate.Con.ConErr ((plan).state) ((plan).done)) (LexLeanBoot.RustAst.Expr.ECons (whole) (LexLeanBoot.RustAst.Expr.ENil)) (callAt (0)) else whole)
  | LexLeanBoot.RustAst.Expr.EBlock lets tail, whole, plan, mode => (if (Nat.beq (mode) (1)) then LexLeanBoot.RustAst.Expr.EBlock (lets) (retarget (tail) (tail) (plan) (0)) else LexLeanBoot.RustAst.Expr.EConstruct (LexLeanBoot.RustValidate.Con.ConErr ((plan).state) ((plan).done)) (LexLeanBoot.RustAst.Expr.ECons (whole) (LexLeanBoot.RustAst.Expr.ENil)) (callAt (0)))
  | LexLeanBoot.RustAst.Expr.ENil, whole, plan, mode => (if (Nat.beq (mode) (0)) then LexLeanBoot.RustAst.Expr.EConstruct (LexLeanBoot.RustValidate.Con.ConErr ((plan).state) ((plan).done)) (LexLeanBoot.RustAst.Expr.ECons (whole) (LexLeanBoot.RustAst.Expr.ENil)) (callAt (0)) else whole)
  | LexLeanBoot.RustAst.Expr.ECons _ _, whole, plan, mode => (if (Nat.beq (mode) (0)) then LexLeanBoot.RustAst.Expr.EConstruct (LexLeanBoot.RustValidate.Con.ConErr ((plan).state) ((plan).done)) (LexLeanBoot.RustAst.Expr.ECons (whole) (LexLeanBoot.RustAst.Expr.ENil)) (callAt (0)) else whole)
  | LexLeanBoot.RustAst.Expr.ELet _ _ _ _ _ _, whole, plan, mode => (if (Nat.beq (mode) (0)) then LexLeanBoot.RustAst.Expr.EConstruct (LexLeanBoot.RustValidate.Con.ConErr ((plan).state) ((plan).done)) (LexLeanBoot.RustAst.Expr.ECons (whole) (LexLeanBoot.RustAst.Expr.ENil)) (callAt (0)) else whole)
  | LexLeanBoot.RustAst.Expr.EArm pats block rest, whole, plan, mode => (if (Nat.beq (mode) (2)) then LexLeanBoot.RustAst.Expr.EArm (pats) (retarget (block) (block) (plan) (1)) (retarget (rest) (rest) (plan) (2)) else whole)
  | LexLeanBoot.RustAst.Expr.ELoop _ _, whole, plan, mode => (if (Nat.beq (mode) (0)) then LexLeanBoot.RustAst.Expr.EConstruct (LexLeanBoot.RustValidate.Con.ConErr ((plan).state) ((plan).done)) (LexLeanBoot.RustAst.Expr.ECons (whole) (LexLeanBoot.RustAst.Expr.ENil)) (callAt (0)) else whole)

@[expose] public def loopFunction (me : Nat) (params : LexLeanBoot.RustAst.Par) (done : LexLeanBoot.RustValidate.Ty) (body : LexLeanBoot.RustAst.Expr) (start : Nat) : LoopOut := (let seen : Tails := tails (body) (body) (me) (1); (if ((!(seen).ok) || LexLeanBoot.RustBodies.isNilE ((seen).calls)) then ({ found := false, params := params, body := body, next := start } : LoopOut) else (let made : Parts := parts (params) ((seen).calls) (0) (start) (0); (if ((!(made).ok) || (Nat.beq ((made).count) (0))) then ({ found := false, params := params, body := body, next := start } : LoopOut) else (let inner : LexLeanBoot.RustAst.Expr := retarget (body) (body) (({ me := me, params := params, calls := (seen).calls, state := (made).ty, done := done } : Retarget)) (1); ({ found := true, params := (made).renamed, body := LexLeanBoot.RustAst.Expr.EBlock (LexLeanBoot.RustAst.Expr.ELet (LexLeanBoot.RustAst.Pat.PBind (LexLeanBoot.RustValidate.Id.IdHolder ((start + 1)))) (false) (LexLeanBoot.RustBodies.unitTy (0)) ((made).start) (callAt (0)) (LexLeanBoot.RustAst.Expr.ENil)) (LexLeanBoot.RustAst.Expr.ELoop (LexLeanBoot.RustValidate.Id.IdHolder ((start + 1))) (LexLeanBoot.RustAst.Expr.EBlock (LexLeanBoot.RustAst.Expr.ELet ((made).tuple) (false) (LexLeanBoot.RustBodies.unitTy (0)) (LexLeanBoot.RustAst.Expr.ERead (LexLeanBoot.RustValidate.ReadKind.RkMove) (LexLeanBoot.RustValidate.Id.IdHolder ((start + 1))) (callAt (0))) (callAt (0)) (LexLeanBoot.RustBodies.blockLets (inner))) (LexLeanBoot.RustBodies.blockTail (inner)))), next := ((start + 1) + (made).count) } : LoopOut))))))

end LexLeanBoot.RustLoops
