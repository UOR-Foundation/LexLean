module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanBoot.CalculusCheck
public import LexLeanBoot.RustAst
public import LexLeanBoot.RustValidate
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.RustLower

public inductive Clos where
  | ClNil
  | ClCons (_ : LexLeanBoot.CalculusCheck.Ty) (_ : Nat) (_ : LexLeanBoot.CalculusCheck.Ty) (_ : Clos)

public inductive Edges where
  | EdgeNil
  | EdgeCons (_ : LexLeanBoot.CalculusCheck.Ty) (_ : LexLeanBoot.CalculusCheck.Ty) (_ : Edges)

public inductive Facts where
  | FactsNil
  | FactsCons (_ : Bool) (_ : List (Nat)) (_ : LexLeanBoot.CalculusCheck.Ty) (_ : Facts)

public structure Collect where
  known : LexLeanBoot.CalculusCheck.Ty
  closures : Clos

public structure Reach where
  own : Bool
  calls : List (Nat)
  applies : LexLeanBoot.CalculusCheck.Ty

@[expose] public def tyMember : (chain : LexLeanBoot.CalculusCheck.Ty) -> (item : LexLeanBoot.CalculusCheck.Ty) -> Bool
  | LexLeanBoot.CalculusCheck.Ty.TUnit, _item => false
  | LexLeanBoot.CalculusCheck.Ty.TBool, _item => false
  | LexLeanBoot.CalculusCheck.Ty.TNat, _item => false
  | LexLeanBoot.CalculusCheck.Ty.TInt, _item => false
  | LexLeanBoot.CalculusCheck.Ty.TFixed _, _item => false
  | LexLeanBoot.CalculusCheck.Ty.TString, _item => false
  | LexLeanBoot.CalculusCheck.Ty.TBytes, _item => false
  | LexLeanBoot.CalculusCheck.Ty.TOrdering, _item => false
  | LexLeanBoot.CalculusCheck.Ty.TOption _, _item => false
  | LexLeanBoot.CalculusCheck.Ty.TResult _ _, _item => false
  | LexLeanBoot.CalculusCheck.Ty.TList _, _item => false
  | LexLeanBoot.CalculusCheck.Ty.TPair _ _, _item => false
  | LexLeanBoot.CalculusCheck.Ty.TAdt _, _item => false
  | LexLeanBoot.CalculusCheck.Ty.TFn _ _, _item => false
  | LexLeanBoot.CalculusCheck.Ty.TNil, _item => false
  | LexLeanBoot.CalculusCheck.Ty.TCons head tail, item => (if LexLeanBoot.CalculusCheck.tyEq (head) (item) then true else tyMember (tail) (item))

@[expose] public def tyAppend : (left : LexLeanBoot.CalculusCheck.Ty) -> (right : LexLeanBoot.CalculusCheck.Ty) -> LexLeanBoot.CalculusCheck.Ty
  | LexLeanBoot.CalculusCheck.Ty.TUnit, right => right
  | LexLeanBoot.CalculusCheck.Ty.TBool, right => right
  | LexLeanBoot.CalculusCheck.Ty.TNat, right => right
  | LexLeanBoot.CalculusCheck.Ty.TInt, right => right
  | LexLeanBoot.CalculusCheck.Ty.TFixed _, right => right
  | LexLeanBoot.CalculusCheck.Ty.TString, right => right
  | LexLeanBoot.CalculusCheck.Ty.TBytes, right => right
  | LexLeanBoot.CalculusCheck.Ty.TOrdering, right => right
  | LexLeanBoot.CalculusCheck.Ty.TOption _, right => right
  | LexLeanBoot.CalculusCheck.Ty.TResult _ _, right => right
  | LexLeanBoot.CalculusCheck.Ty.TList _, right => right
  | LexLeanBoot.CalculusCheck.Ty.TPair _ _, right => right
  | LexLeanBoot.CalculusCheck.Ty.TAdt _, right => right
  | LexLeanBoot.CalculusCheck.Ty.TFn _ _, right => right
  | LexLeanBoot.CalculusCheck.Ty.TNil, right => right
  | LexLeanBoot.CalculusCheck.Ty.TCons head tail, right => LexLeanBoot.CalculusCheck.Ty.TCons (head) (tyAppend (tail) (right))

@[expose] public def tyAddNew (chain : LexLeanBoot.CalculusCheck.Ty) (item : LexLeanBoot.CalculusCheck.Ty) : LexLeanBoot.CalculusCheck.Ty := (if tyMember (chain) (item) then chain else tyAppend (chain) (LexLeanBoot.CalculusCheck.Ty.TCons (item) (LexLeanBoot.CalculusCheck.Ty.TNil)))

@[expose] public def tyDrop : (chain : LexLeanBoot.CalculusCheck.Ty) -> (count : Nat) -> LexLeanBoot.CalculusCheck.Ty
  | LexLeanBoot.CalculusCheck.Ty.TUnit, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TBool, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TNat, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TInt, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TFixed _, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TString, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TBytes, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TOrdering, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TOption _, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TResult _ _, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TList _, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TPair _ _, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TAdt _, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TFn _ _, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TNil, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TCons head tail, count => (match count with | Nat.zero => LexLeanBoot.CalculusCheck.Ty.TCons (head) (tail) | Nat.succ previous => tyDrop (tail) (previous))

@[expose] public def tyTake : (chain : LexLeanBoot.CalculusCheck.Ty) -> (count : Nat) -> LexLeanBoot.CalculusCheck.Ty
  | LexLeanBoot.CalculusCheck.Ty.TUnit, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TBool, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TNat, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TInt, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TFixed _, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TString, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TBytes, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TOrdering, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TOption _, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TResult _ _, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TList _, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TPair _ _, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TAdt _, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TFn _ _, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TNil, _count => LexLeanBoot.CalculusCheck.Ty.TNil
  | LexLeanBoot.CalculusCheck.Ty.TCons head tail, count => (match count with | Nat.zero => LexLeanBoot.CalculusCheck.Ty.TNil | Nat.succ previous => LexLeanBoot.CalculusCheck.Ty.TCons (head) (tyTake (tail) (previous)))

@[expose] public def register : (known : LexLeanBoot.CalculusCheck.Ty) -> (ty : LexLeanBoot.CalculusCheck.Ty) -> LexLeanBoot.CalculusCheck.Ty
  | known, LexLeanBoot.CalculusCheck.Ty.TUnit => known
  | known, LexLeanBoot.CalculusCheck.Ty.TBool => known
  | known, LexLeanBoot.CalculusCheck.Ty.TNat => known
  | known, LexLeanBoot.CalculusCheck.Ty.TInt => known
  | known, LexLeanBoot.CalculusCheck.Ty.TFixed _ => known
  | known, LexLeanBoot.CalculusCheck.Ty.TString => known
  | known, LexLeanBoot.CalculusCheck.Ty.TBytes => known
  | known, LexLeanBoot.CalculusCheck.Ty.TOrdering => known
  | known, LexLeanBoot.CalculusCheck.Ty.TOption value => register (known) (value)
  | known, LexLeanBoot.CalculusCheck.Ty.TResult ok error => register (register (known) (ok)) (error)
  | known, LexLeanBoot.CalculusCheck.Ty.TList element => register (known) (element)
  | known, LexLeanBoot.CalculusCheck.Ty.TPair left right => register (register (known) (left)) (right)
  | known, LexLeanBoot.CalculusCheck.Ty.TAdt _ => known
  | known, LexLeanBoot.CalculusCheck.Ty.TFn parameters result => (let added : LexLeanBoot.CalculusCheck.Ty := tyAddNew (known) (LexLeanBoot.CalculusCheck.Ty.TFn (parameters) (result)); register (register (added) (parameters)) (result))
  | known, LexLeanBoot.CalculusCheck.Ty.TNil => known
  | known, LexLeanBoot.CalculusCheck.Ty.TCons head tail => register (register (known) (head)) (tail)

@[expose] public def primFails (operation : LexLeanBoot.CalculusCheck.Prim) : Bool := (match operation with | LexLeanBoot.CalculusCheck.Prim.NatAdd => true | LexLeanBoot.CalculusCheck.Prim.NatSub => false | LexLeanBoot.CalculusCheck.Prim.NatMul => true | LexLeanBoot.CalculusCheck.Prim.NatQuot => false | LexLeanBoot.CalculusCheck.Prim.NatRem => false | LexLeanBoot.CalculusCheck.Prim.NatEq => false | LexLeanBoot.CalculusCheck.Prim.NatLe => false | LexLeanBoot.CalculusCheck.Prim.NatLt => false | LexLeanBoot.CalculusCheck.Prim.IntAdd => true | LexLeanBoot.CalculusCheck.Prim.IntSub => true | LexLeanBoot.CalculusCheck.Prim.IntMul => true | LexLeanBoot.CalculusCheck.Prim.IntNeg => true | LexLeanBoot.CalculusCheck.Prim.IntQuot => true | LexLeanBoot.CalculusCheck.Prim.IntRem => false | LexLeanBoot.CalculusCheck.Prim.CheckedAdd => false | LexLeanBoot.CalculusCheck.Prim.CheckedSub => false | LexLeanBoot.CalculusCheck.Prim.CheckedMul => false | LexLeanBoot.CalculusCheck.Prim.CheckedNeg => false | LexLeanBoot.CalculusCheck.Prim.CheckedQuot => false | LexLeanBoot.CalculusCheck.Prim.BitAnd => false | LexLeanBoot.CalculusCheck.Prim.BitOr => false | LexLeanBoot.CalculusCheck.Prim.BitXor => false | LexLeanBoot.CalculusCheck.Prim.BitNot => false | LexLeanBoot.CalculusCheck.Prim.ShiftLeft => false | LexLeanBoot.CalculusCheck.Prim.ShiftRight => false | LexLeanBoot.CalculusCheck.Prim.Equal => false | LexLeanBoot.CalculusCheck.Prim.BoolNot => false | LexLeanBoot.CalculusCheck.Prim.BoolAnd => false | LexLeanBoot.CalculusCheck.Prim.BoolOr => false | LexLeanBoot.CalculusCheck.Prim.Append => false | LexLeanBoot.CalculusCheck.Prim.Length => false | LexLeanBoot.CalculusCheck.Prim.Index => false | LexLeanBoot.CalculusCheck.Prim.Slice => false | LexLeanBoot.CalculusCheck.Prim.Utf8Encode => false | LexLeanBoot.CalculusCheck.Prim.Utf8Decode => false | LexLeanBoot.CalculusCheck.Prim.CompareBytes => false | LexLeanBoot.CalculusCheck.Prim.SplitExact => false | LexLeanBoot.CalculusCheck.Prim.Join => false | LexLeanBoot.CalculusCheck.Prim.FormatDecimal => false | LexLeanBoot.CalculusCheck.Prim.Compare => false | LexLeanBoot.CalculusCheck.Prim.Convert _ => false | LexLeanBoot.CalculusCheck.Prim.ParseDecimal target => (Nat.beq (LexLeanBoot.CalculusCheck.tyTag (target)) (3)))

@[expose] public def closHas : (closures : Clos) -> (fnTy : LexLeanBoot.CalculusCheck.Ty) -> (function : Nat) -> Bool
  | Clos.ClNil, _fnTy, _function => false
  | Clos.ClCons known member _ rest, fnTy, function => (if (Nat.beq (member) (function)) then (if LexLeanBoot.CalculusCheck.tyEq (known) (fnTy) then true else closHas (rest) (fnTy) (function)) else closHas (rest) (fnTy) (function))

@[expose] public def closAdd (closures : Clos) (fnTy : LexLeanBoot.CalculusCheck.Ty) (function : Nat) (captured : LexLeanBoot.CalculusCheck.Ty) : Clos := (if closHas (closures) (fnTy) (function) then closures else Clos.ClCons (fnTy) (function) (captured) (closures))

@[expose] public def closVariants : (closures : Clos) -> (fnTy : LexLeanBoot.CalculusCheck.Ty) -> LexLeanBoot.CalculusCheck.Ty
  | Clos.ClNil, _fnTy => LexLeanBoot.CalculusCheck.Ty.TNil
  | Clos.ClCons known _ captured rest, fnTy => (if LexLeanBoot.CalculusCheck.tyEq (known) (fnTy) then LexLeanBoot.CalculusCheck.Ty.TCons (captured) (closVariants (rest) (fnTy)) else closVariants (rest) (fnTy))

@[expose] public def registerChain : (known : LexLeanBoot.CalculusCheck.Ty) -> (tys : LexLeanBoot.CalculusCheck.Ty) -> LexLeanBoot.CalculusCheck.Ty
  | known, LexLeanBoot.CalculusCheck.Ty.TCons head tail => registerChain (register (known) (head)) (tail)
  | known, LexLeanBoot.CalculusCheck.Ty.TUnit => known
  | known, LexLeanBoot.CalculusCheck.Ty.TBool => known
  | known, LexLeanBoot.CalculusCheck.Ty.TNat => known
  | known, LexLeanBoot.CalculusCheck.Ty.TInt => known
  | known, LexLeanBoot.CalculusCheck.Ty.TFixed _ => known
  | known, LexLeanBoot.CalculusCheck.Ty.TString => known
  | known, LexLeanBoot.CalculusCheck.Ty.TBytes => known
  | known, LexLeanBoot.CalculusCheck.Ty.TOrdering => known
  | known, LexLeanBoot.CalculusCheck.Ty.TOption _ => known
  | known, LexLeanBoot.CalculusCheck.Ty.TResult _ _ => known
  | known, LexLeanBoot.CalculusCheck.Ty.TList _ => known
  | known, LexLeanBoot.CalculusCheck.Ty.TPair _ _ => known
  | known, LexLeanBoot.CalculusCheck.Ty.TAdt _ => known
  | known, LexLeanBoot.CalculusCheck.Ty.TFn _ _ => known
  | known, LexLeanBoot.CalculusCheck.Ty.TNil => known

@[expose] public def registerSignature (program : LexLeanBoot.CalculusCheck.Program) (known : LexLeanBoot.CalculusCheck.Ty) (index : Nat) : LexLeanBoot.CalculusCheck.Ty := register (registerChain (known) (LexLeanBoot.CalculusCheck.functionTypesOr (program) (index))) (LexLeanBoot.CalculusCheck.functionResult (program) (index))

@[expose] public def siteOwn (sites : LexLeanBoot.CalculusCheck.Ty) : LexLeanBoot.CalculusCheck.Ty := LexLeanBoot.CalculusCheck.tyFirst (sites)

@[expose] public def kidA (sites : LexLeanBoot.CalculusCheck.Ty) : LexLeanBoot.CalculusCheck.Ty := LexLeanBoot.CalculusCheck.tyFirst (LexLeanBoot.CalculusCheck.tySecond (sites))

@[expose] public def kidB (sites : LexLeanBoot.CalculusCheck.Ty) : LexLeanBoot.CalculusCheck.Ty := LexLeanBoot.CalculusCheck.tyFirst (LexLeanBoot.CalculusCheck.tySecond (LexLeanBoot.CalculusCheck.tySecond (sites)))

@[expose] public def kidC (sites : LexLeanBoot.CalculusCheck.Ty) : LexLeanBoot.CalculusCheck.Ty := LexLeanBoot.CalculusCheck.tyFirst (LexLeanBoot.CalculusCheck.tySecond (LexLeanBoot.CalculusCheck.tySecond (LexLeanBoot.CalculusCheck.tySecond (sites))))

@[expose] public def collectTerm : (program : LexLeanBoot.CalculusCheck.Program) -> (state : Collect) -> (expr : LexLeanBoot.CalculusCheck.Expr) -> (sites : LexLeanBoot.CalculusCheck.Ty) -> Collect
  | _program, state, LexLeanBoot.CalculusCheck.Expr.EValue ty _, _sites => ({ known := register ((state).known) (ty), closures := (state).closures } : Collect)
  | _program, state, LexLeanBoot.CalculusCheck.Expr.EVar _, _sites => state
  | program, state, LexLeanBoot.CalculusCheck.Expr.ELet _ ty bound body, sites => (let registered : Collect := ({ known := register ((state).known) (ty), closures := (state).closures } : Collect); (let inner : Collect := collectTerm (program) (registered) (bound) (kidA (sites)); collectTerm (program) (inner) (body) (kidB (sites))))
  | program, state, LexLeanBoot.CalculusCheck.Expr.ECond condition thenBranch elseBranch, sites => (let one : Collect := collectTerm (program) (state) (condition) (kidA (sites)); (let two : Collect := collectTerm (program) (one) (thenBranch) (kidB (sites)); collectTerm (program) (two) (elseBranch) (kidC (sites))))
  | program, state, LexLeanBoot.CalculusCheck.Expr.EMatch ty scrutinee arms, sites => (let registered : Collect := ({ known := register ((state).known) (ty), closures := (state).closures } : Collect); (let inner : Collect := collectTerm (program) (registered) (scrutinee) (kidA (sites)); collectTerm (program) (inner) (arms) (kidB (sites))))
  | program, state, LexLeanBoot.CalculusCheck.Expr.EBuild _ ty operands, sites => (let registered : Collect := ({ known := register ((state).known) (ty), closures := (state).closures } : Collect); collectTerm (program) (registered) (operands) (kidA (sites)))
  | program, state, LexLeanBoot.CalculusCheck.Expr.ECall _ operands, sites => collectTerm (program) (state) (operands) (kidA (sites))
  | program, state, LexLeanBoot.CalculusCheck.Expr.EClosure function captures, sites => (let declared : LexLeanBoot.CalculusCheck.Ty := LexLeanBoot.CalculusCheck.functionTypesOr (program) (function); (let count : Nat := LexLeanBoot.CalculusCheck.exprChainLength (captures); (let closureType : LexLeanBoot.CalculusCheck.Ty := LexLeanBoot.CalculusCheck.Ty.TFn (tyDrop (declared) (count)) (LexLeanBoot.CalculusCheck.functionResult (program) (function)); (let registered : Collect := ({ known := register ((state).known) (closureType), closures := closAdd ((state).closures) (closureType) (function) (tyTake (declared) (count)) } : Collect); collectTerm (program) (registered) (captures) (kidA (sites))))))
  | program, state, LexLeanBoot.CalculusCheck.Expr.EApply target operands, sites => (let inner : Collect := collectTerm (program) (state) (target) (kidA (sites)); collectTerm (program) (inner) (operands) (kidB (sites)))
  | program, state, LexLeanBoot.CalculusCheck.Expr.EPrim _ operands, sites => collectTerm (program) (state) (operands) (kidA (sites))
  | program, state, LexLeanBoot.CalculusCheck.Expr.EFirst value, sites => collectTerm (program) (state) (value) (kidA (sites))
  | program, state, LexLeanBoot.CalculusCheck.Expr.ESecond value, sites => collectTerm (program) (state) (value) (kidA (sites))
  | program, state, LexLeanBoot.CalculusCheck.Expr.EField value _, sites => collectTerm (program) (state) (value) (kidA (sites))
  | _program, state, LexLeanBoot.CalculusCheck.Expr.ENil, _sites => state
  | program, state, LexLeanBoot.CalculusCheck.Expr.ECons head tail, sites => (let inner : Collect := collectTerm (program) (state) (head) (kidA (sites)); collectTerm (program) (inner) (tail) (kidB (sites)))
  | program, state, LexLeanBoot.CalculusCheck.Expr.EArm _ _ body, sites => collectTerm (program) (state) (body) (kidA (sites))

@[expose] public def collectFunctions : (program : LexLeanBoot.CalculusCheck.Program) -> (state : Collect) -> (bodies : LexLeanBoot.CalculusCheck.Expr) -> (index : Nat) -> (dead : List (Bool)) -> (sites : LexLeanBoot.CalculusCheck.Ty) -> Collect
  | _program, state, LexLeanBoot.CalculusCheck.Expr.EValue _ _, _index, _dead, _sites => state
  | _program, state, LexLeanBoot.CalculusCheck.Expr.EVar _, _index, _dead, _sites => state
  | _program, state, LexLeanBoot.CalculusCheck.Expr.ELet _ _ _ _, _index, _dead, _sites => state
  | _program, state, LexLeanBoot.CalculusCheck.Expr.ECond _ _ _, _index, _dead, _sites => state
  | _program, state, LexLeanBoot.CalculusCheck.Expr.EMatch _ _ _, _index, _dead, _sites => state
  | _program, state, LexLeanBoot.CalculusCheck.Expr.EBuild _ _ _, _index, _dead, _sites => state
  | _program, state, LexLeanBoot.CalculusCheck.Expr.ECall _ _, _index, _dead, _sites => state
  | _program, state, LexLeanBoot.CalculusCheck.Expr.EClosure _ _, _index, _dead, _sites => state
  | _program, state, LexLeanBoot.CalculusCheck.Expr.EApply _ _, _index, _dead, _sites => state
  | _program, state, LexLeanBoot.CalculusCheck.Expr.EPrim _ _, _index, _dead, _sites => state
  | _program, state, LexLeanBoot.CalculusCheck.Expr.EFirst _, _index, _dead, _sites => state
  | _program, state, LexLeanBoot.CalculusCheck.Expr.ESecond _, _index, _dead, _sites => state
  | _program, state, LexLeanBoot.CalculusCheck.Expr.EField _ _, _index, _dead, _sites => state
  | _program, state, LexLeanBoot.CalculusCheck.Expr.ENil, _index, _dead, _sites => state
  | program, state, LexLeanBoot.CalculusCheck.Expr.ECons body rest, index, dead, sites => (let live : Bool := (match LexLeanBoot.CalculusCheck.listAt (Bool) (dead) (index) with | Option.some flag => (!flag) | Option.none => true); (let signed : Collect := ({ known := registerSignature (program) ((state).known) (index), closures := (state).closures } : Collect); (let inner : Collect := (if live then collectTerm (program) (signed) (body) (kidA (sites)) else signed); collectFunctions (program) (inner) (rest) ((index + 1)) (dead) (kidB (sites)))))
  | _program, state, LexLeanBoot.CalculusCheck.Expr.EArm _ _ _, _index, _dead, _sites => state

@[expose] public def natMember : (items : List (Nat)) -> (item : Nat) -> Bool
  | List.nil, _item => false
  | List.cons head tail, item => (if (Nat.beq (head) (item)) then true else natMember (tail) (item))

@[expose] public def reachTerm : (program : LexLeanBoot.CalculusCheck.Program) -> (found : Reach) -> (expr : LexLeanBoot.CalculusCheck.Expr) -> (sites : LexLeanBoot.CalculusCheck.Ty) -> Reach
  | _program, found, LexLeanBoot.CalculusCheck.Expr.EValue _ _, _sites => found
  | _program, found, LexLeanBoot.CalculusCheck.Expr.EVar _, _sites => found
  | program, found, LexLeanBoot.CalculusCheck.Expr.ELet _ _ bound body, sites => (let inner : Reach := reachTerm (program) (found) (bound) (kidA (sites)); reachTerm (program) (inner) (body) (kidB (sites)))
  | program, found, LexLeanBoot.CalculusCheck.Expr.ECond condition thenBranch elseBranch, sites => (let one : Reach := reachTerm (program) (found) (condition) (kidA (sites)); (let two : Reach := reachTerm (program) (one) (thenBranch) (kidB (sites)); reachTerm (program) (two) (elseBranch) (kidC (sites))))
  | program, found, LexLeanBoot.CalculusCheck.Expr.EMatch _ scrutinee arms, sites => (let inner : Reach := reachTerm (program) (found) (scrutinee) (kidA (sites)); reachTerm (program) (inner) (arms) (kidB (sites)))
  | program, found, LexLeanBoot.CalculusCheck.Expr.EBuild shape _ operands, sites => (let inner : Reach := reachTerm (program) (found) (operands) (kidA (sites)); (match shape with | LexLeanBoot.CalculusCheck.Shape.SSucc => ({ own := true, calls := (inner).calls, applies := (inner).applies } : Reach) | LexLeanBoot.CalculusCheck.Shape.SNone => inner | LexLeanBoot.CalculusCheck.Shape.SSome => inner | LexLeanBoot.CalculusCheck.Shape.SOk => inner | LexLeanBoot.CalculusCheck.Shape.SError => inner | LexLeanBoot.CalculusCheck.Shape.SNil => inner | LexLeanBoot.CalculusCheck.Shape.SCons => inner | LexLeanBoot.CalculusCheck.Shape.SZero => inner | LexLeanBoot.CalculusCheck.Shape.SPair => inner | LexLeanBoot.CalculusCheck.Shape.STrue => inner | LexLeanBoot.CalculusCheck.Shape.SFalse => inner | LexLeanBoot.CalculusCheck.Shape.SUnit => inner | LexLeanBoot.CalculusCheck.Shape.SLt => inner | LexLeanBoot.CalculusCheck.Shape.SEq => inner | LexLeanBoot.CalculusCheck.Shape.SGt => inner | LexLeanBoot.CalculusCheck.Shape.SAdt _ => inner))
  | program, found, LexLeanBoot.CalculusCheck.Expr.ECall function operands, sites => (let inner : Reach := reachTerm (program) (found) (operands) (kidA (sites)); ({ own := (inner).own, calls := (if natMember ((inner).calls) (function) then (inner).calls else (function :: (inner).calls)), applies := (inner).applies } : Reach))
  | program, found, LexLeanBoot.CalculusCheck.Expr.EClosure _ captures, sites => reachTerm (program) (found) (captures) (kidA (sites))
  | program, found, LexLeanBoot.CalculusCheck.Expr.EApply target operands, sites => (let one : Reach := reachTerm (program) (found) (target) (kidA (sites)); (let two : Reach := reachTerm (program) (one) (operands) (kidB (sites)); ({ own := (two).own, calls := (two).calls, applies := tyAddNew ((two).applies) (siteOwn (sites)) } : Reach)))
  | program, found, LexLeanBoot.CalculusCheck.Expr.EPrim operation operands, sites => (let inner : Reach := reachTerm (program) (found) (operands) (kidA (sites)); ({ own := (if (inner).own then true else primFails (operation)), calls := (inner).calls, applies := (inner).applies } : Reach))
  | program, found, LexLeanBoot.CalculusCheck.Expr.EFirst value, sites => reachTerm (program) (found) (value) (kidA (sites))
  | program, found, LexLeanBoot.CalculusCheck.Expr.ESecond value, sites => reachTerm (program) (found) (value) (kidA (sites))
  | program, found, LexLeanBoot.CalculusCheck.Expr.EField value _, sites => reachTerm (program) (found) (value) (kidA (sites))
  | _program, found, LexLeanBoot.CalculusCheck.Expr.ENil, _sites => found
  | program, found, LexLeanBoot.CalculusCheck.Expr.ECons head tail, sites => (let inner : Reach := reachTerm (program) (found) (head) (kidA (sites)); reachTerm (program) (inner) (tail) (kidB (sites)))
  | program, found, LexLeanBoot.CalculusCheck.Expr.EArm _ _ body, sites => reachTerm (program) (found) (body) (kidA (sites))

@[expose] public def reachFunctions : (program : LexLeanBoot.CalculusCheck.Program) -> (bodies : LexLeanBoot.CalculusCheck.Expr) -> (index : Nat) -> (sites : LexLeanBoot.CalculusCheck.Ty) -> Facts
  | _program, LexLeanBoot.CalculusCheck.Expr.EValue _ _, _index, _sites => Facts.FactsNil
  | _program, LexLeanBoot.CalculusCheck.Expr.EVar _, _index, _sites => Facts.FactsNil
  | _program, LexLeanBoot.CalculusCheck.Expr.ELet _ _ _ _, _index, _sites => Facts.FactsNil
  | _program, LexLeanBoot.CalculusCheck.Expr.ECond _ _ _, _index, _sites => Facts.FactsNil
  | _program, LexLeanBoot.CalculusCheck.Expr.EMatch _ _ _, _index, _sites => Facts.FactsNil
  | _program, LexLeanBoot.CalculusCheck.Expr.EBuild _ _ _, _index, _sites => Facts.FactsNil
  | _program, LexLeanBoot.CalculusCheck.Expr.ECall _ _, _index, _sites => Facts.FactsNil
  | _program, LexLeanBoot.CalculusCheck.Expr.EClosure _ _, _index, _sites => Facts.FactsNil
  | _program, LexLeanBoot.CalculusCheck.Expr.EApply _ _, _index, _sites => Facts.FactsNil
  | _program, LexLeanBoot.CalculusCheck.Expr.EPrim _ _, _index, _sites => Facts.FactsNil
  | _program, LexLeanBoot.CalculusCheck.Expr.EFirst _, _index, _sites => Facts.FactsNil
  | _program, LexLeanBoot.CalculusCheck.Expr.ESecond _, _index, _sites => Facts.FactsNil
  | _program, LexLeanBoot.CalculusCheck.Expr.EField _ _, _index, _sites => Facts.FactsNil
  | _program, LexLeanBoot.CalculusCheck.Expr.ENil, _index, _sites => Facts.FactsNil
  | program, LexLeanBoot.CalculusCheck.Expr.ECons body rest, index, sites => (let found : Reach := reachTerm (program) (({ own := false, calls := ([] : List (Nat)), applies := LexLeanBoot.CalculusCheck.Ty.TNil } : Reach)) (body) (kidA (sites)); Facts.FactsCons ((found).own) ((found).calls) ((found).applies) (reachFunctions (program) (rest) ((index + 1)) (kidB (sites))))
  | _program, LexLeanBoot.CalculusCheck.Expr.EArm _ _ _, _index, _sites => Facts.FactsNil

@[expose] public def adtOwners : (count : Nat) -> LexLeanBoot.CalculusCheck.Ty
  | Nat.zero => LexLeanBoot.CalculusCheck.Ty.TNil
  | Nat.succ previous => LexLeanBoot.CalculusCheck.Ty.TCons (LexLeanBoot.CalculusCheck.Ty.TAdt (previous)) (adtOwners (previous))

@[expose] public def inPlace : (known : LexLeanBoot.CalculusCheck.Ty) -> (held : LexLeanBoot.CalculusCheck.Ty) -> (ty : LexLeanBoot.CalculusCheck.Ty) -> LexLeanBoot.CalculusCheck.Ty
  | _known, held, LexLeanBoot.CalculusCheck.Ty.TUnit => held
  | _known, held, LexLeanBoot.CalculusCheck.Ty.TBool => held
  | _known, held, LexLeanBoot.CalculusCheck.Ty.TNat => held
  | _known, held, LexLeanBoot.CalculusCheck.Ty.TInt => held
  | _known, held, LexLeanBoot.CalculusCheck.Ty.TFixed _ => held
  | _known, held, LexLeanBoot.CalculusCheck.Ty.TString => held
  | _known, held, LexLeanBoot.CalculusCheck.Ty.TBytes => held
  | _known, held, LexLeanBoot.CalculusCheck.Ty.TOrdering => held
  | known, held, LexLeanBoot.CalculusCheck.Ty.TOption value => inPlace (known) (held) (value)
  | known, held, LexLeanBoot.CalculusCheck.Ty.TResult ok error => inPlace (known) (inPlace (known) (held) (ok)) (error)
  | _known, held, LexLeanBoot.CalculusCheck.Ty.TList _ => held
  | known, held, LexLeanBoot.CalculusCheck.Ty.TPair left right => inPlace (known) (inPlace (known) (held) (left)) (right)
  | _known, held, LexLeanBoot.CalculusCheck.Ty.TAdt index => tyAddNew (held) (LexLeanBoot.CalculusCheck.Ty.TAdt (index))
  | known, held, LexLeanBoot.CalculusCheck.Ty.TFn parameters result => (if tyMember (known) (LexLeanBoot.CalculusCheck.Ty.TFn (parameters) (result)) then tyAddNew (held) (LexLeanBoot.CalculusCheck.Ty.TFn (parameters) (result)) else held)
  | _known, held, LexLeanBoot.CalculusCheck.Ty.TNil => held
  | _known, held, LexLeanBoot.CalculusCheck.Ty.TCons _ _ => held

@[expose] public def edgeHas : (edges : Edges) -> («from» : LexLeanBoot.CalculusCheck.Ty) -> (to : LexLeanBoot.CalculusCheck.Ty) -> Bool
  | Edges.EdgeNil, _from, _to => false
  | Edges.EdgeCons left right rest, «from», to => (if LexLeanBoot.CalculusCheck.tyEq (left) («from») then (if LexLeanBoot.CalculusCheck.tyEq (right) (to) then true else edgeHas (rest) («from») (to)) else edgeHas (rest) («from») (to))

@[expose] public def edgeCount : (edges : Edges) -> Nat
  | Edges.EdgeNil => 0
  | Edges.EdgeCons _ _ rest => (1 + edgeCount (rest))

@[expose] public def edgeAdd (edges : Edges) («from» : LexLeanBoot.CalculusCheck.Ty) (to : LexLeanBoot.CalculusCheck.Ty) : Edges := (if edgeHas (edges) («from») (to) then edges else Edges.EdgeCons («from») (to) (edges))

@[expose] public def edgesToHeld : (owner : LexLeanBoot.CalculusCheck.Ty) -> (held : LexLeanBoot.CalculusCheck.Ty) -> (edges : Edges) -> Edges
  | _owner, LexLeanBoot.CalculusCheck.Ty.TUnit, edges => edges
  | _owner, LexLeanBoot.CalculusCheck.Ty.TBool, edges => edges
  | _owner, LexLeanBoot.CalculusCheck.Ty.TNat, edges => edges
  | _owner, LexLeanBoot.CalculusCheck.Ty.TInt, edges => edges
  | _owner, LexLeanBoot.CalculusCheck.Ty.TFixed _, edges => edges
  | _owner, LexLeanBoot.CalculusCheck.Ty.TString, edges => edges
  | _owner, LexLeanBoot.CalculusCheck.Ty.TBytes, edges => edges
  | _owner, LexLeanBoot.CalculusCheck.Ty.TOrdering, edges => edges
  | _owner, LexLeanBoot.CalculusCheck.Ty.TOption _, edges => edges
  | _owner, LexLeanBoot.CalculusCheck.Ty.TResult _ _, edges => edges
  | _owner, LexLeanBoot.CalculusCheck.Ty.TList _, edges => edges
  | _owner, LexLeanBoot.CalculusCheck.Ty.TPair _ _, edges => edges
  | _owner, LexLeanBoot.CalculusCheck.Ty.TAdt _, edges => edges
  | _owner, LexLeanBoot.CalculusCheck.Ty.TFn _ _, edges => edges
  | _owner, LexLeanBoot.CalculusCheck.Ty.TNil, edges => edges
  | owner, LexLeanBoot.CalculusCheck.Ty.TCons head tail, edges => edgesToHeld (owner) (tail) (edgeAdd (edges) (owner) (head))

@[expose] public def edgesOfFields : (known : LexLeanBoot.CalculusCheck.Ty) -> (owner : LexLeanBoot.CalculusCheck.Ty) -> (fields : LexLeanBoot.CalculusCheck.Ty) -> (edges : Edges) -> Edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TUnit, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TBool, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TNat, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TInt, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TFixed _, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TString, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TBytes, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TOrdering, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TOption _, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TResult _ _, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TList _, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TPair _ _, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TAdt _, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TFn _ _, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TNil, edges => edges
  | known, owner, LexLeanBoot.CalculusCheck.Ty.TCons head tail, edges => edgesOfFields (known) (owner) (tail) (edgesToHeld (owner) (inPlace (known) (LexLeanBoot.CalculusCheck.Ty.TNil) (head)) (edges))

@[expose] public def edgesOfVariants : (known : LexLeanBoot.CalculusCheck.Ty) -> (owner : LexLeanBoot.CalculusCheck.Ty) -> (variants : LexLeanBoot.CalculusCheck.Ty) -> (edges : Edges) -> Edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TUnit, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TBool, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TNat, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TInt, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TFixed _, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TString, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TBytes, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TOrdering, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TOption _, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TResult _ _, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TList _, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TPair _ _, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TAdt _, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TFn _ _, edges => edges
  | _known, _owner, LexLeanBoot.CalculusCheck.Ty.TNil, edges => edges
  | known, owner, LexLeanBoot.CalculusCheck.Ty.TCons head tail, edges => edgesOfVariants (known) (owner) (tail) (edgesOfFields (known) (owner) (head) (edges))

@[expose] public def variantsOf (program : LexLeanBoot.CalculusCheck.Program) (closures : Clos) (owner : LexLeanBoot.CalculusCheck.Ty) : LexLeanBoot.CalculusCheck.Ty := (match owner with | LexLeanBoot.CalculusCheck.Ty.TUnit => LexLeanBoot.CalculusCheck.Ty.TNil | LexLeanBoot.CalculusCheck.Ty.TBool => LexLeanBoot.CalculusCheck.Ty.TNil | LexLeanBoot.CalculusCheck.Ty.TNat => LexLeanBoot.CalculusCheck.Ty.TNil | LexLeanBoot.CalculusCheck.Ty.TInt => LexLeanBoot.CalculusCheck.Ty.TNil | LexLeanBoot.CalculusCheck.Ty.TFixed _ => LexLeanBoot.CalculusCheck.Ty.TNil | LexLeanBoot.CalculusCheck.Ty.TString => LexLeanBoot.CalculusCheck.Ty.TNil | LexLeanBoot.CalculusCheck.Ty.TBytes => LexLeanBoot.CalculusCheck.Ty.TNil | LexLeanBoot.CalculusCheck.Ty.TOrdering => LexLeanBoot.CalculusCheck.Ty.TNil | LexLeanBoot.CalculusCheck.Ty.TOption _ => LexLeanBoot.CalculusCheck.Ty.TNil | LexLeanBoot.CalculusCheck.Ty.TResult _ _ => LexLeanBoot.CalculusCheck.Ty.TNil | LexLeanBoot.CalculusCheck.Ty.TList _ => LexLeanBoot.CalculusCheck.Ty.TNil | LexLeanBoot.CalculusCheck.Ty.TPair _ _ => LexLeanBoot.CalculusCheck.Ty.TNil | LexLeanBoot.CalculusCheck.Ty.TAdt index => (match LexLeanBoot.CalculusCheck.adtAt (program) (index) with | Option.some constructors => constructors | Option.none => LexLeanBoot.CalculusCheck.Ty.TNil) | LexLeanBoot.CalculusCheck.Ty.TFn parameters result => closVariants (closures) (LexLeanBoot.CalculusCheck.Ty.TFn (parameters) (result)) | LexLeanBoot.CalculusCheck.Ty.TNil => LexLeanBoot.CalculusCheck.Ty.TNil | LexLeanBoot.CalculusCheck.Ty.TCons _ _ => LexLeanBoot.CalculusCheck.Ty.TNil)

@[expose] public def allEdges : (program : LexLeanBoot.CalculusCheck.Program) -> (known : LexLeanBoot.CalculusCheck.Ty) -> (closures : Clos) -> (owners : LexLeanBoot.CalculusCheck.Ty) -> (edges : Edges) -> Edges
  | _program, _known, _closures, LexLeanBoot.CalculusCheck.Ty.TUnit, edges => edges
  | _program, _known, _closures, LexLeanBoot.CalculusCheck.Ty.TBool, edges => edges
  | _program, _known, _closures, LexLeanBoot.CalculusCheck.Ty.TNat, edges => edges
  | _program, _known, _closures, LexLeanBoot.CalculusCheck.Ty.TInt, edges => edges
  | _program, _known, _closures, LexLeanBoot.CalculusCheck.Ty.TFixed _, edges => edges
  | _program, _known, _closures, LexLeanBoot.CalculusCheck.Ty.TString, edges => edges
  | _program, _known, _closures, LexLeanBoot.CalculusCheck.Ty.TBytes, edges => edges
  | _program, _known, _closures, LexLeanBoot.CalculusCheck.Ty.TOrdering, edges => edges
  | _program, _known, _closures, LexLeanBoot.CalculusCheck.Ty.TOption _, edges => edges
  | _program, _known, _closures, LexLeanBoot.CalculusCheck.Ty.TResult _ _, edges => edges
  | _program, _known, _closures, LexLeanBoot.CalculusCheck.Ty.TList _, edges => edges
  | _program, _known, _closures, LexLeanBoot.CalculusCheck.Ty.TPair _ _, edges => edges
  | _program, _known, _closures, LexLeanBoot.CalculusCheck.Ty.TAdt _, edges => edges
  | _program, _known, _closures, LexLeanBoot.CalculusCheck.Ty.TFn _ _, edges => edges
  | _program, _known, _closures, LexLeanBoot.CalculusCheck.Ty.TNil, edges => edges
  | program, known, closures, LexLeanBoot.CalculusCheck.Ty.TCons head tail, edges => allEdges (program) (known) (closures) (tail) (edgesOfVariants (known) (head) (variantsOf (program) (closures) (head)) (edges))

@[expose] public def composeFrom : (right : Edges) -> (middle : LexLeanBoot.CalculusCheck.Ty) -> («from» : LexLeanBoot.CalculusCheck.Ty) -> (edges : Edges) -> Edges
  | Edges.EdgeNil, _middle, _from, edges => edges
  | Edges.EdgeCons start to rest, middle, «from», edges => composeFrom (rest) (middle) («from») ((if LexLeanBoot.CalculusCheck.tyEq (start) (middle) then edgeAdd (edges) («from») (to) else edges))

@[expose] public def compose : (all : Edges) -> (left : Edges) -> (edges : Edges) -> Edges
  | _all, Edges.EdgeNil, edges => edges
  | all, Edges.EdgeCons «from» middle rest, edges => compose (all) (rest) (composeFrom (all) (middle) («from») (edges))

@[expose] public def closeStep (edges : Edges) : Option (Edges) := (let wider : Edges := compose (edges) (edges) (edges); (if (Nat.beq (edgeCount (wider)) (edgeCount (edges))) then Option.none else Option.some (wider)))

@[expose] public def closeEdges (edges : Edges) (bound : Nat) : Edges := ((LexLeanCollections.iterateUntil ((fun (current : Edges) => closeStep (current))) (bound) (edges) : (Prod (Edges) (Bool)))).1

@[expose] public def visible : (ty : LexLeanBoot.CalculusCheck.Ty) -> (inhabited : LexLeanBoot.CalculusCheck.Ty) -> Bool
  | LexLeanBoot.CalculusCheck.Ty.TUnit, _inhabited => true
  | LexLeanBoot.CalculusCheck.Ty.TBool, _inhabited => true
  | LexLeanBoot.CalculusCheck.Ty.TNat, _inhabited => true
  | LexLeanBoot.CalculusCheck.Ty.TInt, _inhabited => true
  | LexLeanBoot.CalculusCheck.Ty.TFixed _, _inhabited => true
  | LexLeanBoot.CalculusCheck.Ty.TString, _inhabited => true
  | LexLeanBoot.CalculusCheck.Ty.TBytes, _inhabited => true
  | LexLeanBoot.CalculusCheck.Ty.TOrdering, _inhabited => true
  | LexLeanBoot.CalculusCheck.Ty.TOption _, _inhabited => true
  | LexLeanBoot.CalculusCheck.Ty.TResult ok error, inhabited => (if visible (ok) (inhabited) then true else visible (error) (inhabited))
  | LexLeanBoot.CalculusCheck.Ty.TList _, _inhabited => true
  | LexLeanBoot.CalculusCheck.Ty.TPair left right, inhabited => (if visible (left) (inhabited) then visible (right) (inhabited) else false)
  | LexLeanBoot.CalculusCheck.Ty.TAdt index, inhabited => tyMember (inhabited) (LexLeanBoot.CalculusCheck.Ty.TAdt (index))
  | LexLeanBoot.CalculusCheck.Ty.TFn parameters result, inhabited => tyMember (inhabited) (LexLeanBoot.CalculusCheck.Ty.TFn (parameters) (result))
  | LexLeanBoot.CalculusCheck.Ty.TNil, _inhabited => true
  | LexLeanBoot.CalculusCheck.Ty.TCons _ _, _inhabited => true

@[expose] public def anyHeld : (owner : LexLeanBoot.CalculusCheck.Ty) -> (edges : Edges) -> (held : LexLeanBoot.CalculusCheck.Ty) -> Bool
  | _owner, _edges, LexLeanBoot.CalculusCheck.Ty.TUnit => false
  | _owner, _edges, LexLeanBoot.CalculusCheck.Ty.TBool => false
  | _owner, _edges, LexLeanBoot.CalculusCheck.Ty.TNat => false
  | _owner, _edges, LexLeanBoot.CalculusCheck.Ty.TInt => false
  | _owner, _edges, LexLeanBoot.CalculusCheck.Ty.TFixed _ => false
  | _owner, _edges, LexLeanBoot.CalculusCheck.Ty.TString => false
  | _owner, _edges, LexLeanBoot.CalculusCheck.Ty.TBytes => false
  | _owner, _edges, LexLeanBoot.CalculusCheck.Ty.TOrdering => false
  | _owner, _edges, LexLeanBoot.CalculusCheck.Ty.TOption _ => false
  | _owner, _edges, LexLeanBoot.CalculusCheck.Ty.TResult _ _ => false
  | _owner, _edges, LexLeanBoot.CalculusCheck.Ty.TList _ => false
  | _owner, _edges, LexLeanBoot.CalculusCheck.Ty.TPair _ _ => false
  | _owner, _edges, LexLeanBoot.CalculusCheck.Ty.TAdt _ => false
  | _owner, _edges, LexLeanBoot.CalculusCheck.Ty.TFn _ _ => false
  | _owner, _edges, LexLeanBoot.CalculusCheck.Ty.TNil => false
  | owner, edges, LexLeanBoot.CalculusCheck.Ty.TCons head tail => (if LexLeanBoot.CalculusCheck.tyEq (head) (owner) then true else (if edgeHas (edges) (head) (owner) then true else anyHeld (owner) (edges) (tail)))

@[expose] public def cyclic (known : LexLeanBoot.CalculusCheck.Ty) (edges : Edges) (owner : LexLeanBoot.CalculusCheck.Ty) (ty : LexLeanBoot.CalculusCheck.Ty) : Bool := anyHeld (owner) (edges) (inPlace (known) (LexLeanBoot.CalculusCheck.Ty.TNil) (ty))

@[expose] public def fieldsOk : (known : LexLeanBoot.CalculusCheck.Ty) -> (edges : Edges) -> (inhabited : LexLeanBoot.CalculusCheck.Ty) -> (owner : LexLeanBoot.CalculusCheck.Ty) -> (fields : LexLeanBoot.CalculusCheck.Ty) -> Bool
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TUnit => true
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TBool => true
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TNat => true
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TInt => true
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TFixed _ => true
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TString => true
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TBytes => true
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TOrdering => true
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TOption _ => true
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TResult _ _ => true
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TList _ => true
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TPair _ _ => true
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TAdt _ => true
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TFn _ _ => true
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TNil => true
  | known, edges, inhabited, owner, LexLeanBoot.CalculusCheck.Ty.TCons head tail => (if (if cyclic (known) (edges) (owner) (head) then true else visible (head) (inhabited)) then fieldsOk (known) (edges) (inhabited) (owner) (tail) else false)

@[expose] public def someVariantOk : (known : LexLeanBoot.CalculusCheck.Ty) -> (edges : Edges) -> (inhabited : LexLeanBoot.CalculusCheck.Ty) -> (owner : LexLeanBoot.CalculusCheck.Ty) -> (variants : LexLeanBoot.CalculusCheck.Ty) -> Bool
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TUnit => false
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TBool => false
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TNat => false
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TInt => false
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TFixed _ => false
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TString => false
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TBytes => false
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TOrdering => false
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TOption _ => false
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TResult _ _ => false
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TList _ => false
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TPair _ _ => false
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TAdt _ => false
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TFn _ _ => false
  | _known, _edges, _inhabited, _owner, LexLeanBoot.CalculusCheck.Ty.TNil => false
  | known, edges, inhabited, owner, LexLeanBoot.CalculusCheck.Ty.TCons head tail => (if fieldsOk (known) (edges) (inhabited) (owner) (head) then true else someVariantOk (known) (edges) (inhabited) (owner) (tail))

@[expose] public def inhabitRound : (program : LexLeanBoot.CalculusCheck.Program) -> (known : LexLeanBoot.CalculusCheck.Ty) -> (closures : Clos) -> (edges : Edges) -> (owners : LexLeanBoot.CalculusCheck.Ty) -> (inhabited : LexLeanBoot.CalculusCheck.Ty) -> LexLeanBoot.CalculusCheck.Ty
  | _program, _known, _closures, _edges, LexLeanBoot.CalculusCheck.Ty.TUnit, inhabited => inhabited
  | _program, _known, _closures, _edges, LexLeanBoot.CalculusCheck.Ty.TBool, inhabited => inhabited
  | _program, _known, _closures, _edges, LexLeanBoot.CalculusCheck.Ty.TNat, inhabited => inhabited
  | _program, _known, _closures, _edges, LexLeanBoot.CalculusCheck.Ty.TInt, inhabited => inhabited
  | _program, _known, _closures, _edges, LexLeanBoot.CalculusCheck.Ty.TFixed _, inhabited => inhabited
  | _program, _known, _closures, _edges, LexLeanBoot.CalculusCheck.Ty.TString, inhabited => inhabited
  | _program, _known, _closures, _edges, LexLeanBoot.CalculusCheck.Ty.TBytes, inhabited => inhabited
  | _program, _known, _closures, _edges, LexLeanBoot.CalculusCheck.Ty.TOrdering, inhabited => inhabited
  | _program, _known, _closures, _edges, LexLeanBoot.CalculusCheck.Ty.TOption _, inhabited => inhabited
  | _program, _known, _closures, _edges, LexLeanBoot.CalculusCheck.Ty.TResult _ _, inhabited => inhabited
  | _program, _known, _closures, _edges, LexLeanBoot.CalculusCheck.Ty.TList _, inhabited => inhabited
  | _program, _known, _closures, _edges, LexLeanBoot.CalculusCheck.Ty.TPair _ _, inhabited => inhabited
  | _program, _known, _closures, _edges, LexLeanBoot.CalculusCheck.Ty.TAdt _, inhabited => inhabited
  | _program, _known, _closures, _edges, LexLeanBoot.CalculusCheck.Ty.TFn _ _, inhabited => inhabited
  | _program, _known, _closures, _edges, LexLeanBoot.CalculusCheck.Ty.TNil, inhabited => inhabited
  | program, known, closures, edges, LexLeanBoot.CalculusCheck.Ty.TCons head tail, inhabited => inhabitRound (program) (known) (closures) (edges) (tail) ((if tyMember (inhabited) (head) then inhabited else (if someVariantOk (known) (edges) (inhabited) (head) (variantsOf (program) (closures) (head)) then tyAddNew (inhabited) (head) else inhabited)))

@[expose] public def inhabitStep (program : LexLeanBoot.CalculusCheck.Program) (known : LexLeanBoot.CalculusCheck.Ty) (closures : Clos) (edges : Edges) (owners : LexLeanBoot.CalculusCheck.Ty) (inhabited : LexLeanBoot.CalculusCheck.Ty) : Option (LexLeanBoot.CalculusCheck.Ty) := (let next : LexLeanBoot.CalculusCheck.Ty := inhabitRound (program) (known) (closures) (edges) (owners) (inhabited); (if (Nat.beq (LexLeanBoot.CalculusCheck.tyChainLength (next)) (LexLeanBoot.CalculusCheck.tyChainLength (inhabited))) then Option.none else Option.some (next)))

@[expose] public def inhabitation (program : LexLeanBoot.CalculusCheck.Program) (known : LexLeanBoot.CalculusCheck.Ty) (closures : Clos) (edges : Edges) (owners : LexLeanBoot.CalculusCheck.Ty) : LexLeanBoot.CalculusCheck.Ty := ((LexLeanCollections.iterateUntil ((fun (inhabited : LexLeanBoot.CalculusCheck.Ty) => inhabitStep (program) (known) (closures) (edges) (owners) (inhabited))) ((1 + LexLeanBoot.CalculusCheck.tyChainLength (owners))) (LexLeanBoot.CalculusCheck.Ty.TNil) : (Prod (LexLeanBoot.CalculusCheck.Ty) (Bool)))).1

@[expose] public def anyInvisible : (params : LexLeanBoot.CalculusCheck.Ty) -> (inhabited : LexLeanBoot.CalculusCheck.Ty) -> Bool
  | LexLeanBoot.CalculusCheck.Ty.TUnit, _inhabited => false
  | LexLeanBoot.CalculusCheck.Ty.TBool, _inhabited => false
  | LexLeanBoot.CalculusCheck.Ty.TNat, _inhabited => false
  | LexLeanBoot.CalculusCheck.Ty.TInt, _inhabited => false
  | LexLeanBoot.CalculusCheck.Ty.TFixed _, _inhabited => false
  | LexLeanBoot.CalculusCheck.Ty.TString, _inhabited => false
  | LexLeanBoot.CalculusCheck.Ty.TBytes, _inhabited => false
  | LexLeanBoot.CalculusCheck.Ty.TOrdering, _inhabited => false
  | LexLeanBoot.CalculusCheck.Ty.TOption _, _inhabited => false
  | LexLeanBoot.CalculusCheck.Ty.TResult _ _, _inhabited => false
  | LexLeanBoot.CalculusCheck.Ty.TList _, _inhabited => false
  | LexLeanBoot.CalculusCheck.Ty.TPair _ _, _inhabited => false
  | LexLeanBoot.CalculusCheck.Ty.TAdt _, _inhabited => false
  | LexLeanBoot.CalculusCheck.Ty.TFn _ _, _inhabited => false
  | LexLeanBoot.CalculusCheck.Ty.TNil, _inhabited => false
  | LexLeanBoot.CalculusCheck.Ty.TCons head tail, inhabited => (if visible (head) (inhabited) then anyInvisible (tail) (inhabited) else true)

@[expose] public def deadFlags : (types : LexLeanBoot.CalculusCheck.Ty) -> (inhabited : LexLeanBoot.CalculusCheck.Ty) -> List (Bool)
  | LexLeanBoot.CalculusCheck.Ty.TUnit, _inhabited => ([] : List (Bool))
  | LexLeanBoot.CalculusCheck.Ty.TBool, _inhabited => ([] : List (Bool))
  | LexLeanBoot.CalculusCheck.Ty.TNat, _inhabited => ([] : List (Bool))
  | LexLeanBoot.CalculusCheck.Ty.TInt, _inhabited => ([] : List (Bool))
  | LexLeanBoot.CalculusCheck.Ty.TFixed _, _inhabited => ([] : List (Bool))
  | LexLeanBoot.CalculusCheck.Ty.TString, _inhabited => ([] : List (Bool))
  | LexLeanBoot.CalculusCheck.Ty.TBytes, _inhabited => ([] : List (Bool))
  | LexLeanBoot.CalculusCheck.Ty.TOrdering, _inhabited => ([] : List (Bool))
  | LexLeanBoot.CalculusCheck.Ty.TOption _, _inhabited => ([] : List (Bool))
  | LexLeanBoot.CalculusCheck.Ty.TResult _ _, _inhabited => ([] : List (Bool))
  | LexLeanBoot.CalculusCheck.Ty.TList _, _inhabited => ([] : List (Bool))
  | LexLeanBoot.CalculusCheck.Ty.TPair _ _, _inhabited => ([] : List (Bool))
  | LexLeanBoot.CalculusCheck.Ty.TAdt _, _inhabited => ([] : List (Bool))
  | LexLeanBoot.CalculusCheck.Ty.TFn _ _, _inhabited => ([] : List (Bool))
  | LexLeanBoot.CalculusCheck.Ty.TNil, _inhabited => ([] : List (Bool))
  | LexLeanBoot.CalculusCheck.Ty.TCons head tail, inhabited => (anyInvisible (head) (inhabited) :: deadFlags (tail) (inhabited))

@[expose] public def boolsEq : (left : List (Bool)) -> (right : List (Bool)) -> Bool
  | List.nil, right => (match right with | List.nil => true | List.cons _ _ => false)
  | List.cons lh lt, right => (match right with | List.nil => false | List.cons rh rt => (if (LexLeanRuntime.equal (lh) (rh) : Bool) then boolsEq (lt) (rt) else false))

public structure Liveness where
  dead : List (Bool)
  closures : Clos

@[expose] public def livenessStep (program : LexLeanBoot.CalculusCheck.Program) (bodies : LexLeanBoot.CalculusCheck.Expr) (known : LexLeanBoot.CalculusCheck.Ty) (edges : Edges) (owners : LexLeanBoot.CalculusCheck.Ty) (sites : LexLeanBoot.CalculusCheck.Ty) (state : Liveness) : Option (Liveness) := (let inhabited : LexLeanBoot.CalculusCheck.Ty := inhabitation (program) (known) ((state).closures) (edges) (owners); (let dead : List (Bool) := deadFlags ((program).types) (inhabited); (if boolsEq (dead) ((state).dead) then Option.none else Option.some (({ dead := dead, closures := (collectFunctions (program) (({ known := known, closures := Clos.ClNil } : Collect)) (bodies) (0) (dead) (sites)).closures } : Liveness)))))

@[expose] public def falses : (count : Nat) -> List (Bool)
  | Nat.zero => ([] : List (Bool))
  | Nat.succ previous => (false :: falses (previous))

@[expose] public def flagAt (flags : List (Bool)) (index : Nat) : Bool := (match LexLeanBoot.CalculusCheck.listAt (Bool) (flags) (index) with | Option.some flag => flag | Option.none => false)

@[expose] public def callsFail : (calls : List (Nat)) -> (flags : List (Bool)) -> Bool
  | List.nil, _flags => false
  | List.cons head tail, flags => (if flagAt (flags) (head) then true else callsFail (tail) (flags))

@[expose] public def closuresFail : (closures : Clos) -> (fnTy : LexLeanBoot.CalculusCheck.Ty) -> (flags : List (Bool)) -> Bool
  | Clos.ClNil, _fnTy, _flags => false
  | Clos.ClCons known member _ rest, fnTy, flags => (if LexLeanBoot.CalculusCheck.tyEq (known) (fnTy) then (if flagAt (flags) (member) then true else closuresFail (rest) (fnTy) (flags)) else closuresFail (rest) (fnTy) (flags))

@[expose] public def appliesFail : (applies : LexLeanBoot.CalculusCheck.Ty) -> (closures : Clos) -> (flags : List (Bool)) -> Bool
  | LexLeanBoot.CalculusCheck.Ty.TUnit, _closures, _flags => false
  | LexLeanBoot.CalculusCheck.Ty.TBool, _closures, _flags => false
  | LexLeanBoot.CalculusCheck.Ty.TNat, _closures, _flags => false
  | LexLeanBoot.CalculusCheck.Ty.TInt, _closures, _flags => false
  | LexLeanBoot.CalculusCheck.Ty.TFixed _, _closures, _flags => false
  | LexLeanBoot.CalculusCheck.Ty.TString, _closures, _flags => false
  | LexLeanBoot.CalculusCheck.Ty.TBytes, _closures, _flags => false
  | LexLeanBoot.CalculusCheck.Ty.TOrdering, _closures, _flags => false
  | LexLeanBoot.CalculusCheck.Ty.TOption _, _closures, _flags => false
  | LexLeanBoot.CalculusCheck.Ty.TResult _ _, _closures, _flags => false
  | LexLeanBoot.CalculusCheck.Ty.TList _, _closures, _flags => false
  | LexLeanBoot.CalculusCheck.Ty.TPair _ _, _closures, _flags => false
  | LexLeanBoot.CalculusCheck.Ty.TAdt _, _closures, _flags => false
  | LexLeanBoot.CalculusCheck.Ty.TFn _ _, _closures, _flags => false
  | LexLeanBoot.CalculusCheck.Ty.TNil, _closures, _flags => false
  | LexLeanBoot.CalculusCheck.Ty.TCons head tail, closures, flags => (if closuresFail (closures) (head) (flags) then true else appliesFail (tail) (closures) (flags))

@[expose] public def failStep : (facts : Facts) -> (closures : Clos) -> (flags : List (Bool)) -> (index : Nat) -> List (Bool)
  | Facts.FactsNil, _closures, _flags, _index => ([] : List (Bool))
  | Facts.FactsCons own calls applies rest, closures, flags, index => ((if (if own then true else (if flagAt (flags) (index) then true else callsFail (calls) (flags))) then true else appliesFail (applies) (closures) (flags)) :: failStep (rest) (closures) (flags) ((index + 1)))

@[expose] public def failNext (facts : Facts) (closures : Clos) (flags : List (Bool)) : Option (List (Bool)) := (let next : List (Bool) := failStep (facts) (closures) (flags) (0); (if boolsEq (next) (flags) then Option.none else Option.some (next)))

@[expose] public def deadFunctions (spec : ByteArray) (adts : LexLeanBoot.CalculusCheck.Ty) (parameters : List (List (Nat))) (types : LexLeanBoot.CalculusCheck.Ty) (results : LexLeanBoot.CalculusCheck.Ty) (bodies : LexLeanBoot.CalculusCheck.Expr) (sites : LexLeanBoot.CalculusCheck.Ty) : List (Bool) := (let program : LexLeanBoot.CalculusCheck.Program := ({ spec := spec, adts := adts, parameters := parameters, types := types, results := results, bodies := bodies } : LexLeanBoot.CalculusCheck.Program); (let count : Nat := LexLeanBoot.CalculusCheck.exprChainLength (bodies); (let registered : LexLeanBoot.CalculusCheck.Ty := register (register (register (LexLeanBoot.CalculusCheck.Ty.TNil) (types)) (results)) (adts); (let initial : Collect := collectFunctions (program) (({ known := registered, closures := Clos.ClNil } : Collect)) (bodies) (0) (([] : List (Bool))) (sites); (let known : LexLeanBoot.CalculusCheck.Ty := (initial).known; (let owners : LexLeanBoot.CalculusCheck.Ty := tyAppend (adtOwners (LexLeanBoot.CalculusCheck.tyChainLength (adts))) (known); (let direct : Edges := allEdges (program) (known) ((initial).closures) (owners) (Edges.EdgeNil); (let edges : Edges := closeEdges (direct) ((1 + (LexLeanRuntime.multiply (LexLeanBoot.CalculusCheck.tyChainLength (owners)) (LexLeanBoot.CalculusCheck.tyChainLength (owners)) : Nat))); (let live : Liveness := ((LexLeanCollections.iterateUntil ((fun (state : Liveness) => livenessStep (program) (bodies) (known) (edges) (owners) (sites) (state))) ((1 + count)) (({ dead := falses (count), closures := (initial).closures } : Liveness)) : (Prod (Liveness) (Bool)))).1; (live).dead)))))))))

@[expose] public def fallibleFunctions (spec : ByteArray) (adts : LexLeanBoot.CalculusCheck.Ty) (parameters : List (List (Nat))) (types : LexLeanBoot.CalculusCheck.Ty) (results : LexLeanBoot.CalculusCheck.Ty) (bodies : LexLeanBoot.CalculusCheck.Expr) (sites : LexLeanBoot.CalculusCheck.Ty) (dead : List (Bool)) : List (Bool) := (let program : LexLeanBoot.CalculusCheck.Program := ({ spec := spec, adts := adts, parameters := parameters, types := types, results := results, bodies := bodies } : LexLeanBoot.CalculusCheck.Program); (let count : Nat := LexLeanBoot.CalculusCheck.exprChainLength (bodies); (let registered : LexLeanBoot.CalculusCheck.Ty := register (register (register (LexLeanBoot.CalculusCheck.Ty.TNil) (types)) (results)) (adts); (let initial : Collect := collectFunctions (program) (({ known := registered, closures := Clos.ClNil } : Collect)) (bodies) (0) (dead) (sites); (let facts : Facts := reachFunctions (program) (bodies) (0) (sites); ((LexLeanCollections.iterateUntil ((fun (flags : List (Bool)) => failNext (facts) ((initial).closures) (flags))) ((1 + count)) (falses (count)) : (Prod (List (Bool)) (Bool)))).1)))))

@[expose] public def kindOf (kind : LexLeanBoot.CalculusCheck.IntKind) : LexLeanBoot.RustValidate.IntKind := (match kind with | LexLeanBoot.CalculusCheck.IntKind.U8 => LexLeanBoot.RustValidate.IntKind.U8 | LexLeanBoot.CalculusCheck.IntKind.U16 => LexLeanBoot.RustValidate.IntKind.U16 | LexLeanBoot.CalculusCheck.IntKind.U32 => LexLeanBoot.RustValidate.IntKind.U32 | LexLeanBoot.CalculusCheck.IntKind.U64 => LexLeanBoot.RustValidate.IntKind.U64 | LexLeanBoot.CalculusCheck.IntKind.I8 => LexLeanBoot.RustValidate.IntKind.I8 | LexLeanBoot.CalculusCheck.IntKind.I16 => LexLeanBoot.RustValidate.IntKind.I16 | LexLeanBoot.CalculusCheck.IntKind.I32 => LexLeanBoot.RustValidate.IntKind.I32 | LexLeanBoot.CalculusCheck.IntKind.I64 => LexLeanBoot.RustValidate.IntKind.I64)

@[expose] public def fnPosition : (known : LexLeanBoot.CalculusCheck.Ty) -> (ty : LexLeanBoot.CalculusCheck.Ty) -> («at» : Nat) -> Nat
  | LexLeanBoot.CalculusCheck.Ty.TUnit, _ty, «at» => «at»
  | LexLeanBoot.CalculusCheck.Ty.TBool, _ty, «at» => «at»
  | LexLeanBoot.CalculusCheck.Ty.TNat, _ty, «at» => «at»
  | LexLeanBoot.CalculusCheck.Ty.TInt, _ty, «at» => «at»
  | LexLeanBoot.CalculusCheck.Ty.TFixed _, _ty, «at» => «at»
  | LexLeanBoot.CalculusCheck.Ty.TString, _ty, «at» => «at»
  | LexLeanBoot.CalculusCheck.Ty.TBytes, _ty, «at» => «at»
  | LexLeanBoot.CalculusCheck.Ty.TOrdering, _ty, «at» => «at»
  | LexLeanBoot.CalculusCheck.Ty.TOption _, _ty, «at» => «at»
  | LexLeanBoot.CalculusCheck.Ty.TResult _ _, _ty, «at» => «at»
  | LexLeanBoot.CalculusCheck.Ty.TList _, _ty, «at» => «at»
  | LexLeanBoot.CalculusCheck.Ty.TPair _ _, _ty, «at» => «at»
  | LexLeanBoot.CalculusCheck.Ty.TAdt _, _ty, «at» => «at»
  | LexLeanBoot.CalculusCheck.Ty.TFn _ _, _ty, «at» => «at»
  | LexLeanBoot.CalculusCheck.Ty.TNil, _ty, «at» => «at»
  | LexLeanBoot.CalculusCheck.Ty.TCons head tail, ty, «at» => (if LexLeanBoot.CalculusCheck.tyEq (head) (ty) then «at» else fnPosition (tail) (ty) ((«at» + 1)))

@[expose] public def vty : (known : LexLeanBoot.CalculusCheck.Ty) -> (ty : LexLeanBoot.CalculusCheck.Ty) -> LexLeanBoot.RustValidate.Ty
  | _known, LexLeanBoot.CalculusCheck.Ty.TUnit => LexLeanBoot.RustValidate.Ty.TyLeaf (LexLeanBoot.RustValidate.LeafKind.LkUnit)
  | _known, LexLeanBoot.CalculusCheck.Ty.TBool => LexLeanBoot.RustValidate.Ty.TyLeaf (LexLeanBoot.RustValidate.LeafKind.LkBool)
  | _known, LexLeanBoot.CalculusCheck.Ty.TNat => LexLeanBoot.RustValidate.Ty.TyLeaf (LexLeanBoot.RustValidate.LeafKind.LkNat)
  | _known, LexLeanBoot.CalculusCheck.Ty.TInt => LexLeanBoot.RustValidate.Ty.TyLeaf (LexLeanBoot.RustValidate.LeafKind.LkInt)
  | _known, LexLeanBoot.CalculusCheck.Ty.TFixed width => LexLeanBoot.RustValidate.Ty.TyFixed (kindOf (width))
  | _known, LexLeanBoot.CalculusCheck.Ty.TString => LexLeanBoot.RustValidate.Ty.TyLeaf (LexLeanBoot.RustValidate.LeafKind.LkStr)
  | _known, LexLeanBoot.CalculusCheck.Ty.TBytes => LexLeanBoot.RustValidate.Ty.TyLeaf (LexLeanBoot.RustValidate.LeafKind.LkBytes)
  | _known, LexLeanBoot.CalculusCheck.Ty.TOrdering => LexLeanBoot.RustValidate.Ty.TyLeaf (LexLeanBoot.RustValidate.LeafKind.LkOrdering)
  | known, LexLeanBoot.CalculusCheck.Ty.TOption value => LexLeanBoot.RustValidate.Ty.TyUnary (LexLeanBoot.RustValidate.UnaryKind.UkOption) (vty (known) (value))
  | known, LexLeanBoot.CalculusCheck.Ty.TResult ok error => LexLeanBoot.RustValidate.Ty.TyBinary (LexLeanBoot.RustValidate.BinaryKind.BkResult) (vty (known) (ok)) (vty (known) (error))
  | known, LexLeanBoot.CalculusCheck.Ty.TList element => LexLeanBoot.RustValidate.Ty.TyUnary (LexLeanBoot.RustValidate.UnaryKind.UkList) (vty (known) (element))
  | known, LexLeanBoot.CalculusCheck.Ty.TPair left right => LexLeanBoot.RustValidate.Ty.TyBinary (LexLeanBoot.RustValidate.BinaryKind.BkPair) (vty (known) (left)) (vty (known) (right))
  | _known, LexLeanBoot.CalculusCheck.Ty.TAdt index => LexLeanBoot.RustValidate.Ty.TyNamed (LexLeanBoot.RustValidate.NameKind.NkAdt) (index)
  | known, LexLeanBoot.CalculusCheck.Ty.TFn parameters result => LexLeanBoot.RustValidate.Ty.TyNamed (LexLeanBoot.RustValidate.NameKind.NkFn) (fnPosition (known) (LexLeanBoot.CalculusCheck.Ty.TFn (parameters) (result)) (0))
  | _known, LexLeanBoot.CalculusCheck.Ty.TNil => LexLeanBoot.RustValidate.Ty.TyNil
  | known, LexLeanBoot.CalculusCheck.Ty.TCons head tail => LexLeanBoot.RustValidate.Ty.TyCons (vty (known) (head)) (vty (known) (tail))

@[expose] public def fieldTy (known : LexLeanBoot.CalculusCheck.Ty) (edges : Edges) (owner : LexLeanBoot.CalculusCheck.Ty) (ty : LexLeanBoot.CalculusCheck.Ty) : LexLeanBoot.RustValidate.Ty := (if cyclic (known) (edges) (owner) (ty) then LexLeanBoot.RustValidate.Ty.TyUnary (LexLeanBoot.RustValidate.UnaryKind.UkRc) (vty (known) (ty)) else vty (known) (ty))

@[expose] public def fieldChain : (known : LexLeanBoot.CalculusCheck.Ty) -> (edges : Edges) -> (owner : LexLeanBoot.CalculusCheck.Ty) -> (tys : LexLeanBoot.CalculusCheck.Ty) -> LexLeanBoot.RustValidate.Ty
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TUnit => LexLeanBoot.RustValidate.Ty.TyNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TBool => LexLeanBoot.RustValidate.Ty.TyNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TNat => LexLeanBoot.RustValidate.Ty.TyNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TInt => LexLeanBoot.RustValidate.Ty.TyNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TFixed _ => LexLeanBoot.RustValidate.Ty.TyNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TString => LexLeanBoot.RustValidate.Ty.TyNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TBytes => LexLeanBoot.RustValidate.Ty.TyNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TOrdering => LexLeanBoot.RustValidate.Ty.TyNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TOption _ => LexLeanBoot.RustValidate.Ty.TyNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TResult _ _ => LexLeanBoot.RustValidate.Ty.TyNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TList _ => LexLeanBoot.RustValidate.Ty.TyNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TPair _ _ => LexLeanBoot.RustValidate.Ty.TyNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TAdt _ => LexLeanBoot.RustValidate.Ty.TyNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TFn _ _ => LexLeanBoot.RustValidate.Ty.TyNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TNil => LexLeanBoot.RustValidate.Ty.TyNil
  | known, edges, owner, LexLeanBoot.CalculusCheck.Ty.TCons head tail => LexLeanBoot.RustValidate.Ty.TyCons (fieldTy (known) (edges) (owner) (head)) (fieldChain (known) (edges) (owner) (tail))

@[expose] public def copyTy : (ty : LexLeanBoot.CalculusCheck.Ty) -> Bool
  | LexLeanBoot.CalculusCheck.Ty.TUnit => true
  | LexLeanBoot.CalculusCheck.Ty.TBool => true
  | LexLeanBoot.CalculusCheck.Ty.TNat => true
  | LexLeanBoot.CalculusCheck.Ty.TInt => true
  | LexLeanBoot.CalculusCheck.Ty.TFixed _ => true
  | LexLeanBoot.CalculusCheck.Ty.TString => false
  | LexLeanBoot.CalculusCheck.Ty.TBytes => false
  | LexLeanBoot.CalculusCheck.Ty.TOrdering => true
  | LexLeanBoot.CalculusCheck.Ty.TOption value => copyTy (value)
  | LexLeanBoot.CalculusCheck.Ty.TResult ok error => (if copyTy (ok) then copyTy (error) else false)
  | LexLeanBoot.CalculusCheck.Ty.TList _ => false
  | LexLeanBoot.CalculusCheck.Ty.TPair left right => (if copyTy (left) then copyTy (right) else false)
  | LexLeanBoot.CalculusCheck.Ty.TAdt _ => false
  | LexLeanBoot.CalculusCheck.Ty.TFn _ _ => false
  | LexLeanBoot.CalculusCheck.Ty.TNil => false
  | LexLeanBoot.CalculusCheck.Ty.TCons _ _ => false

@[expose] public def capOf (known : LexLeanBoot.CalculusCheck.Ty) (edges : Edges) (owner : LexLeanBoot.CalculusCheck.Ty) (ty : LexLeanBoot.CalculusCheck.Ty) : LexLeanBoot.RustValidate.Cap := (if LexLeanBoot.CalculusCheck.tyEq (ty) (LexLeanBoot.CalculusCheck.Ty.TUnit) then LexLeanBoot.RustValidate.Cap.CapUnit else (if cyclic (known) (edges) (owner) (ty) then LexLeanBoot.RustValidate.Cap.CapUnbox else (if copyTy (ty) then LexLeanBoot.RustValidate.Cap.CapCopy else LexLeanBoot.RustValidate.Cap.CapClone)))

@[expose] public def capList : (known : LexLeanBoot.CalculusCheck.Ty) -> (edges : Edges) -> (owner : LexLeanBoot.CalculusCheck.Ty) -> (tys : LexLeanBoot.CalculusCheck.Ty) -> List (LexLeanBoot.RustValidate.Cap)
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TUnit => ([] : List (LexLeanBoot.RustValidate.Cap))
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TBool => ([] : List (LexLeanBoot.RustValidate.Cap))
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TNat => ([] : List (LexLeanBoot.RustValidate.Cap))
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TInt => ([] : List (LexLeanBoot.RustValidate.Cap))
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TFixed _ => ([] : List (LexLeanBoot.RustValidate.Cap))
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TString => ([] : List (LexLeanBoot.RustValidate.Cap))
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TBytes => ([] : List (LexLeanBoot.RustValidate.Cap))
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TOrdering => ([] : List (LexLeanBoot.RustValidate.Cap))
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TOption _ => ([] : List (LexLeanBoot.RustValidate.Cap))
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TResult _ _ => ([] : List (LexLeanBoot.RustValidate.Cap))
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TList _ => ([] : List (LexLeanBoot.RustValidate.Cap))
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TPair _ _ => ([] : List (LexLeanBoot.RustValidate.Cap))
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TAdt _ => ([] : List (LexLeanBoot.RustValidate.Cap))
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TFn _ _ => ([] : List (LexLeanBoot.RustValidate.Cap))
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TNil => ([] : List (LexLeanBoot.RustValidate.Cap))
  | known, edges, owner, LexLeanBoot.CalculusCheck.Ty.TCons head tail => (capOf (known) (edges) (owner) (head) :: capList (known) (edges) (owner) (tail))

@[expose] public def revClos : (closures : Clos) -> (acc : Clos) -> Clos
  | Clos.ClNil, acc => acc
  | Clos.ClCons fnTy member captured rest, acc => revClos (rest) (Clos.ClCons (fnTy) (member) (captured) (acc))

@[expose] public def rowsFor : (known : LexLeanBoot.CalculusCheck.Ty) -> (edges : Edges) -> (owner : LexLeanBoot.CalculusCheck.Ty) -> (target : LexLeanBoot.CalculusCheck.Ty) -> (closures : Clos) -> LexLeanBoot.RustValidate.Row
  | _known, _edges, _owner, _target, Clos.ClNil => LexLeanBoot.RustValidate.Row.RowNil
  | known, edges, owner, target, Clos.ClCons fnTy member captured rest => (if LexLeanBoot.CalculusCheck.tyEq (fnTy) (target) then LexLeanBoot.RustValidate.Row.RowCons (member) (fieldChain (known) (edges) (owner) (captured)) (rowsFor (known) (edges) (owner) (target) (rest)) else rowsFor (known) (edges) (owner) (target) (rest))

@[expose] public def dispatchFor : (known : LexLeanBoot.CalculusCheck.Ty) -> (edges : Edges) -> (owner : LexLeanBoot.CalculusCheck.Ty) -> (target : LexLeanBoot.CalculusCheck.Ty) -> (fallible : List (Bool)) -> (closures : Clos) -> LexLeanBoot.RustValidate.Dis
  | _known, _edges, _owner, _target, _fallible, Clos.ClNil => LexLeanBoot.RustValidate.Dis.DisNil
  | known, edges, owner, target, fallible, Clos.ClCons fnTy member captured rest => (if LexLeanBoot.CalculusCheck.tyEq (fnTy) (target) then LexLeanBoot.RustValidate.Dis.DisCons (member) (capList (known) (edges) (owner) (captured)) (flagAt (fallible) (member)) (dispatchFor (known) (edges) (owner) (target) (fallible) (rest)) else dispatchFor (known) (edges) (owner) (target) (fallible) (rest))

@[expose] public def applyFails : (target : LexLeanBoot.CalculusCheck.Ty) -> (fallible : List (Bool)) -> (closures : Clos) -> Bool
  | _target, _fallible, Clos.ClNil => false
  | target, fallible, Clos.ClCons fnTy member _ rest => (if LexLeanBoot.CalculusCheck.tyEq (fnTy) (target) then (if flagAt (fallible) (member) then true else applyFails (target) (fallible) (rest)) else applyFails (target) (fallible) (rest))

@[expose] public def closureOf : (target : LexLeanBoot.CalculusCheck.Ty) -> (closures : Clos) -> Bool
  | _target, Clos.ClNil => false
  | target, Clos.ClCons fnTy _ _ rest => (if LexLeanBoot.CalculusCheck.tyEq (fnTy) (target) then true else closureOf (target) (rest))

@[expose] public def fnOrigin (closed : Bool) : LexLeanBoot.RustValidate.Origin := (if closed then ({ element := (LexLeanRuntime.utf8Encode ("expr:closure") : ByteArray), width := Option.none } : LexLeanBoot.RustValidate.Origin) else ({ element := (LexLeanRuntime.utf8Encode ("type:fn") : ByteArray), width := Option.none } : LexLeanBoot.RustValidate.Origin))

@[expose] public def adtRows : (known : LexLeanBoot.CalculusCheck.Ty) -> (edges : Edges) -> (owner : LexLeanBoot.CalculusCheck.Ty) -> (constructors : LexLeanBoot.CalculusCheck.Ty) -> (number : Nat) -> LexLeanBoot.RustValidate.Row
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TUnit, _number => LexLeanBoot.RustValidate.Row.RowNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TBool, _number => LexLeanBoot.RustValidate.Row.RowNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TNat, _number => LexLeanBoot.RustValidate.Row.RowNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TInt, _number => LexLeanBoot.RustValidate.Row.RowNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TFixed _, _number => LexLeanBoot.RustValidate.Row.RowNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TString, _number => LexLeanBoot.RustValidate.Row.RowNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TBytes, _number => LexLeanBoot.RustValidate.Row.RowNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TOrdering, _number => LexLeanBoot.RustValidate.Row.RowNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TOption _, _number => LexLeanBoot.RustValidate.Row.RowNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TResult _ _, _number => LexLeanBoot.RustValidate.Row.RowNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TList _, _number => LexLeanBoot.RustValidate.Row.RowNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TPair _ _, _number => LexLeanBoot.RustValidate.Row.RowNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TAdt _, _number => LexLeanBoot.RustValidate.Row.RowNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TFn _ _, _number => LexLeanBoot.RustValidate.Row.RowNil
  | _known, _edges, _owner, LexLeanBoot.CalculusCheck.Ty.TNil, _number => LexLeanBoot.RustValidate.Row.RowNil
  | known, edges, owner, LexLeanBoot.CalculusCheck.Ty.TCons fields rest, number => LexLeanBoot.RustValidate.Row.RowCons (number) (fieldChain (known) (edges) (owner) (fields)) (adtRows (known) (edges) (owner) (rest) ((number + 1)))

@[expose] public def adtDefs : (known : LexLeanBoot.CalculusCheck.Ty) -> (edges : Edges) -> (adts : LexLeanBoot.CalculusCheck.Ty) -> (position : Nat) -> (tail : LexLeanBoot.RustAst.Def) -> LexLeanBoot.RustAst.Def
  | _known, _edges, LexLeanBoot.CalculusCheck.Ty.TUnit, _position, tail => tail
  | _known, _edges, LexLeanBoot.CalculusCheck.Ty.TBool, _position, tail => tail
  | _known, _edges, LexLeanBoot.CalculusCheck.Ty.TNat, _position, tail => tail
  | _known, _edges, LexLeanBoot.CalculusCheck.Ty.TInt, _position, tail => tail
  | _known, _edges, LexLeanBoot.CalculusCheck.Ty.TFixed _, _position, tail => tail
  | _known, _edges, LexLeanBoot.CalculusCheck.Ty.TString, _position, tail => tail
  | _known, _edges, LexLeanBoot.CalculusCheck.Ty.TBytes, _position, tail => tail
  | _known, _edges, LexLeanBoot.CalculusCheck.Ty.TOrdering, _position, tail => tail
  | _known, _edges, LexLeanBoot.CalculusCheck.Ty.TOption _, _position, tail => tail
  | _known, _edges, LexLeanBoot.CalculusCheck.Ty.TResult _ _, _position, tail => tail
  | _known, _edges, LexLeanBoot.CalculusCheck.Ty.TList _, _position, tail => tail
  | _known, _edges, LexLeanBoot.CalculusCheck.Ty.TPair _ _, _position, tail => tail
  | _known, _edges, LexLeanBoot.CalculusCheck.Ty.TAdt _, _position, tail => tail
  | _known, _edges, LexLeanBoot.CalculusCheck.Ty.TFn _ _, _position, tail => tail
  | _known, _edges, LexLeanBoot.CalculusCheck.Ty.TNil, _position, tail => tail
  | known, edges, LexLeanBoot.CalculusCheck.Ty.TCons constructors rest, position, tail => LexLeanBoot.RustAst.Def.DCons (LexLeanBoot.RustAst.Def.DEnum (LexLeanBoot.RustValidate.Ty.TyNamed (LexLeanBoot.RustValidate.NameKind.NkAdt) (position)) (adtRows (known) (edges) (LexLeanBoot.CalculusCheck.Ty.TAdt (position)) (constructors) (0)) (({ element := (LexLeanRuntime.utf8Encode ("type:adt") : ByteArray), width := Option.none } : LexLeanBoot.RustValidate.Origin))) (adtDefs (known) (edges) (rest) ((position + 1)) (tail))

@[expose] public def fnDefs : (known : LexLeanBoot.CalculusCheck.Ty) -> (edges : Edges) -> (ordered : Clos) -> (every : Clos) -> (fallible : List (Bool)) -> (types : LexLeanBoot.CalculusCheck.Ty) -> (position : Nat) -> (tail : LexLeanBoot.RustAst.Def) -> LexLeanBoot.RustAst.Def
  | _known, _edges, _ordered, _every, _fallible, LexLeanBoot.CalculusCheck.Ty.TUnit, _position, tail => tail
  | _known, _edges, _ordered, _every, _fallible, LexLeanBoot.CalculusCheck.Ty.TBool, _position, tail => tail
  | _known, _edges, _ordered, _every, _fallible, LexLeanBoot.CalculusCheck.Ty.TNat, _position, tail => tail
  | _known, _edges, _ordered, _every, _fallible, LexLeanBoot.CalculusCheck.Ty.TInt, _position, tail => tail
  | _known, _edges, _ordered, _every, _fallible, LexLeanBoot.CalculusCheck.Ty.TFixed _, _position, tail => tail
  | _known, _edges, _ordered, _every, _fallible, LexLeanBoot.CalculusCheck.Ty.TString, _position, tail => tail
  | _known, _edges, _ordered, _every, _fallible, LexLeanBoot.CalculusCheck.Ty.TBytes, _position, tail => tail
  | _known, _edges, _ordered, _every, _fallible, LexLeanBoot.CalculusCheck.Ty.TOrdering, _position, tail => tail
  | _known, _edges, _ordered, _every, _fallible, LexLeanBoot.CalculusCheck.Ty.TOption _, _position, tail => tail
  | _known, _edges, _ordered, _every, _fallible, LexLeanBoot.CalculusCheck.Ty.TResult _ _, _position, tail => tail
  | _known, _edges, _ordered, _every, _fallible, LexLeanBoot.CalculusCheck.Ty.TList _, _position, tail => tail
  | _known, _edges, _ordered, _every, _fallible, LexLeanBoot.CalculusCheck.Ty.TPair _ _, _position, tail => tail
  | _known, _edges, _ordered, _every, _fallible, LexLeanBoot.CalculusCheck.Ty.TAdt _, _position, tail => tail
  | _known, _edges, _ordered, _every, _fallible, LexLeanBoot.CalculusCheck.Ty.TFn _ _, _position, tail => tail
  | _known, _edges, _ordered, _every, _fallible, LexLeanBoot.CalculusCheck.Ty.TNil, _position, tail => tail
  | known, edges, ordered, every, fallible, LexLeanBoot.CalculusCheck.Ty.TCons head rest, position, tail => (match head with | LexLeanBoot.CalculusCheck.Ty.TUnit => fnDefs (known) (edges) (ordered) (every) (fallible) (rest) ((position + 1)) (tail) | LexLeanBoot.CalculusCheck.Ty.TBool => fnDefs (known) (edges) (ordered) (every) (fallible) (rest) ((position + 1)) (tail) | LexLeanBoot.CalculusCheck.Ty.TNat => fnDefs (known) (edges) (ordered) (every) (fallible) (rest) ((position + 1)) (tail) | LexLeanBoot.CalculusCheck.Ty.TInt => fnDefs (known) (edges) (ordered) (every) (fallible) (rest) ((position + 1)) (tail) | LexLeanBoot.CalculusCheck.Ty.TFixed _ => fnDefs (known) (edges) (ordered) (every) (fallible) (rest) ((position + 1)) (tail) | LexLeanBoot.CalculusCheck.Ty.TString => fnDefs (known) (edges) (ordered) (every) (fallible) (rest) ((position + 1)) (tail) | LexLeanBoot.CalculusCheck.Ty.TBytes => fnDefs (known) (edges) (ordered) (every) (fallible) (rest) ((position + 1)) (tail) | LexLeanBoot.CalculusCheck.Ty.TOrdering => fnDefs (known) (edges) (ordered) (every) (fallible) (rest) ((position + 1)) (tail) | LexLeanBoot.CalculusCheck.Ty.TOption _ => fnDefs (known) (edges) (ordered) (every) (fallible) (rest) ((position + 1)) (tail) | LexLeanBoot.CalculusCheck.Ty.TResult _ _ => fnDefs (known) (edges) (ordered) (every) (fallible) (rest) ((position + 1)) (tail) | LexLeanBoot.CalculusCheck.Ty.TList _ => fnDefs (known) (edges) (ordered) (every) (fallible) (rest) ((position + 1)) (tail) | LexLeanBoot.CalculusCheck.Ty.TPair _ _ => fnDefs (known) (edges) (ordered) (every) (fallible) (rest) ((position + 1)) (tail) | LexLeanBoot.CalculusCheck.Ty.TAdt _ => fnDefs (known) (edges) (ordered) (every) (fallible) (rest) ((position + 1)) (tail) | LexLeanBoot.CalculusCheck.Ty.TFn parameters result => (let target : LexLeanBoot.CalculusCheck.Ty := LexLeanBoot.CalculusCheck.Ty.TFn (parameters) (result); LexLeanBoot.RustAst.Def.DCons (LexLeanBoot.RustAst.Def.DEnum (LexLeanBoot.RustValidate.Ty.TyNamed (LexLeanBoot.RustValidate.NameKind.NkFn) (position)) (rowsFor (known) (edges) (target) (target) (ordered)) (fnOrigin (closureOf (target) (every)))) (LexLeanBoot.RustAst.Def.DCons (LexLeanBoot.RustAst.Def.DApply (position) (vty (known) (parameters)) (vty (known) (result)) (applyFails (target) (fallible) (ordered)) (dispatchFor (known) (edges) (target) (target) (fallible) (ordered)) (fnOrigin (closureOf (target) (every)))) (fnDefs (known) (edges) (ordered) (every) (fallible) (rest) ((position + 1)) (tail)))) | LexLeanBoot.CalculusCheck.Ty.TNil => fnDefs (known) (edges) (ordered) (every) (fallible) (rest) ((position + 1)) (tail) | LexLeanBoot.CalculusCheck.Ty.TCons _ _ => fnDefs (known) (edges) (ordered) (every) (fallible) (rest) ((position + 1)) (tail))

@[expose] public def registerCtors : (known : LexLeanBoot.CalculusCheck.Ty) -> (constructors : LexLeanBoot.CalculusCheck.Ty) -> LexLeanBoot.CalculusCheck.Ty
  | known, LexLeanBoot.CalculusCheck.Ty.TUnit => known
  | known, LexLeanBoot.CalculusCheck.Ty.TBool => known
  | known, LexLeanBoot.CalculusCheck.Ty.TNat => known
  | known, LexLeanBoot.CalculusCheck.Ty.TInt => known
  | known, LexLeanBoot.CalculusCheck.Ty.TFixed _ => known
  | known, LexLeanBoot.CalculusCheck.Ty.TString => known
  | known, LexLeanBoot.CalculusCheck.Ty.TBytes => known
  | known, LexLeanBoot.CalculusCheck.Ty.TOrdering => known
  | known, LexLeanBoot.CalculusCheck.Ty.TOption _ => known
  | known, LexLeanBoot.CalculusCheck.Ty.TResult _ _ => known
  | known, LexLeanBoot.CalculusCheck.Ty.TList _ => known
  | known, LexLeanBoot.CalculusCheck.Ty.TPair _ _ => known
  | known, LexLeanBoot.CalculusCheck.Ty.TAdt _ => known
  | known, LexLeanBoot.CalculusCheck.Ty.TFn _ _ => known
  | known, LexLeanBoot.CalculusCheck.Ty.TNil => known
  | known, LexLeanBoot.CalculusCheck.Ty.TCons fields rest => registerCtors (registerChain (known) (fields)) (rest)

@[expose] public def registerAdts : (known : LexLeanBoot.CalculusCheck.Ty) -> (adts : LexLeanBoot.CalculusCheck.Ty) -> LexLeanBoot.CalculusCheck.Ty
  | known, LexLeanBoot.CalculusCheck.Ty.TUnit => known
  | known, LexLeanBoot.CalculusCheck.Ty.TBool => known
  | known, LexLeanBoot.CalculusCheck.Ty.TNat => known
  | known, LexLeanBoot.CalculusCheck.Ty.TInt => known
  | known, LexLeanBoot.CalculusCheck.Ty.TFixed _ => known
  | known, LexLeanBoot.CalculusCheck.Ty.TString => known
  | known, LexLeanBoot.CalculusCheck.Ty.TBytes => known
  | known, LexLeanBoot.CalculusCheck.Ty.TOrdering => known
  | known, LexLeanBoot.CalculusCheck.Ty.TOption _ => known
  | known, LexLeanBoot.CalculusCheck.Ty.TResult _ _ => known
  | known, LexLeanBoot.CalculusCheck.Ty.TList _ => known
  | known, LexLeanBoot.CalculusCheck.Ty.TPair _ _ => known
  | known, LexLeanBoot.CalculusCheck.Ty.TAdt _ => known
  | known, LexLeanBoot.CalculusCheck.Ty.TFn _ _ => known
  | known, LexLeanBoot.CalculusCheck.Ty.TNil => known
  | known, LexLeanBoot.CalculusCheck.Ty.TCons constructors rest => registerAdts (registerCtors (known) (constructors)) (rest)

@[expose] public def lowerTypes (spec : ByteArray) (adts : LexLeanBoot.CalculusCheck.Ty) (parameters : List (List (Nat))) (types : LexLeanBoot.CalculusCheck.Ty) (results : LexLeanBoot.CalculusCheck.Ty) (bodies : LexLeanBoot.CalculusCheck.Expr) (sites : LexLeanBoot.CalculusCheck.Ty) (dead : List (Bool)) (fallible : List (Bool)) : LexLeanBoot.RustAst.Def := (let program : LexLeanBoot.CalculusCheck.Program := ({ spec := spec, adts := adts, parameters := parameters, types := types, results := results, bodies := bodies } : LexLeanBoot.CalculusCheck.Program); (let numbered : Collect := collectFunctions (program) (({ known := LexLeanBoot.CalculusCheck.Ty.TNil, closures := Clos.ClNil } : Collect)) (bodies) (0) (([] : List (Bool))) (sites); (let known : LexLeanBoot.CalculusCheck.Ty := registerAdts ((numbered).known) (adts); (let live : Collect := collectFunctions (program) (({ known := known, closures := Clos.ClNil } : Collect)) (bodies) (0) (dead) (sites); (let owners : LexLeanBoot.CalculusCheck.Ty := tyAppend (adtOwners (LexLeanBoot.CalculusCheck.tyChainLength (adts))) (known); (let direct : Edges := allEdges (program) (known) ((numbered).closures) (owners) (Edges.EdgeNil); (let edges : Edges := closeEdges (direct) ((1 + (LexLeanRuntime.multiply (LexLeanBoot.CalculusCheck.tyChainLength (owners)) (LexLeanBoot.CalculusCheck.tyChainLength (owners)) : Nat))); adtDefs (known) (edges) (adts) (0) (fnDefs (known) (edges) (revClos ((live).closures) (Clos.ClNil)) ((numbered).closures) (fallible) (known) (0) (LexLeanBoot.RustAst.Def.DNil)))))))))

end LexLeanBoot.RustLower
