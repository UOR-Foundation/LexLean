module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Text
public import LexLeanBoot.RustValidate
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.RuntimeValidate

public inductive Ty where
  | TyUnit
  | TyBool
  | TyNat
  | TyFixed
  | TyOrdering
  | TySelf
  | TyParam (_ : Nat)
  | TyRef (_ : Ty)
  | TyRefMut (_ : Ty)
  | TyPair (_ : Ty) (_ : Ty)
  | TyNamed (_ : ByteArray) (_ : Ty)
  | TyWide
  | TySize
  | TyChar
  | TyStr
  | TySlice (_ : Ty)
  | TyUnknown
  | TyAbsent
  | TyNil
  | TyCons (_ : Ty) (_ : Ty)

public inductive Pat where
  | PWild
  | PBind (_ : ByteArray)
  | PBindMut (_ : ByteArray)
  | PPath (_ : List (ByteArray))
  | PTuple (_ : Pat)
  | PTupleStruct (_ : List (ByteArray)) (_ : Pat)
  | PStruct (_ : List (ByteArray)) (_ : List (ByteArray)) (_ : Pat)
  | PLit (_ : Nat)
  | PNil
  | PCons (_ : Pat) (_ : Pat)

public inductive Expr where
  | EVar (_ : ByteArray)
  | ELit (_ : Nat)
  | EPath (_ : List (ByteArray))
  | ECall (_ : List (ByteArray)) (_ : Expr)
  | EMethod (_ : Expr) (_ : ByteArray) (_ : Expr)
  | EField (_ : Expr) (_ : Nat)
  | EFieldNamed (_ : Expr) (_ : ByteArray)
  | EBorrow (_ : Expr)
  | EEquals (_ : Expr) (_ : Expr)
  | EMatch (_ : Expr) (_ : Expr)
  | ELet (_ : Pat) (_ : Ty) (_ : Expr)
  | EAssign (_ : Expr) (_ : Expr)
  | EIf (_ : Expr) (_ : Expr) (_ : Expr)
  | EWhile (_ : Expr) (_ : Expr)
  | EWhileLet (_ : Pat) (_ : Expr) (_ : Expr)
  | ELoop (_ : Expr)
  | EFor (_ : Pat) (_ : Expr) (_ : Expr)
  | EBreak
  | EReturn (_ : Expr)
  | ETuple (_ : Expr)
  | EStructLit (_ : List (ByteArray)) (_ : List (ByteArray)) (_ : Expr)
  | EBinary (_ : Nat) (_ : Expr) (_ : Expr)
  | ENot (_ : Expr)
  | ECast (_ : Expr) (_ : Ty)
  | EIndex (_ : Expr) (_ : Expr)
  | ERange (_ : Expr) (_ : Expr) (_ : Expr)
  | EDeref (_ : Expr)
  | EBorrowMut (_ : Expr)
  | EMethodOf (_ : Expr) (_ : ByteArray) (_ : Ty) (_ : Expr)
  | EBlock (_ : Expr) (_ : Expr)
  | ENone
  | ENil
  | ECons (_ : Expr) (_ : Expr)
  | ELetS (_ : Pat) (_ : Ty) (_ : Expr) (_ : Expr)
  | EArm (_ : Pat) (_ : Expr) (_ : Expr) (_ : Expr)

public structure Generic where
  name : ByteArray
  bounds : List (ByteArray)

public structure Param where
  name : ByteArray
  mutable : Bool
  ty : Ty

public structure Sig where
  name : ByteArray
  receiver : Bool
  exclusive : Bool
  params : List (Param)
  result : Ty

public structure FnDef where
  sig : Sig
  generics : List (Generic)
  «public» : Bool
  body : Expr

public inductive Variant where
  | VUnit (_ : ByteArray)
  | VTuple (_ : ByteArray) (_ : Ty)
  | VRecord (_ : ByteArray) (_ : List (ByteArray)) (_ : Ty)

public inductive Item where
  | IUse (_ : List (ByteArray)) (_ : List (ByteArray))
  | IUnit (_ : ByteArray) (_ : List (ByteArray))
  | IAlias (_ : ByteArray) (_ : List (ByteArray)) (_ : Ty)
  | IStatic (_ : ByteArray) (_ : Ty) (_ : Expr)
  | ITrait (_ : ByteArray) (_ : List (Sig))
  | IImpl (_ : List (Generic)) (_ : List (ByteArray)) (_ : Ty) (_ : List (FnDef))
  | IFn (_ : FnDef)
  | ITuple (_ : ByteArray) (_ : List (ByteArray)) (_ : List (ByteArray)) (_ : Ty)
  | IRecord (_ : ByteArray) (_ : List (ByteArray)) (_ : List (ByteArray)) (_ : List (ByteArray)) (_ : Ty)
  | IEnum (_ : ByteArray) (_ : List (ByteArray)) (_ : List (ByteArray)) (_ : List (Variant))

public structure Crate where
  core : Bool
  items : List (Item)

public structure Entry where
  name : ByteArray
  n : Nat

public structure TraitE where
  name : ByteArray
  sigs : List (Sig)

public structure StructE where
  name : ByteArray
  fields : List (ByteArray)
  gcount : Nat

public structure EnumE where
  name : ByteArray
  variants : List (Variant)
  gcount : Nat

public structure Sc where
  core : Bool
  declared : List (ByteArray)
  traits : List (TraitE)
  structs : List (StructE)
  enums : List (EnumE)
  aliases : List (Entry)
  statics : List (ByteArray)
  functions : List (Entry)
  methods : List (Entry)
  imported : List (ByteArray)

public structure Var where
  name : ByteArray
  mutable : Bool
  ty : Ty

public structure Fr where
  generics : List (Generic)
  vars : List (Var)
  selfTy : Ty
  inLoop : Bool

@[expose] public def rtext (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def rcat (left : ByteArray) (right : ByteArray) : ByteArray := (LexLeanRuntime.append (left) (right) : ByteArray)

@[expose] public def rtick (name : ByteArray) : ByteArray := rcat (rtext ("`")) (rcat (name) (rtext ("`")))

@[expose] public def rdec (number : Nat) : ByteArray := LexLeanBoot.RustValidate.decimal (number)

@[expose] public def bEq (left : ByteArray) (right : ByteArray) : Bool := (LexLeanRuntime.equal (left) (right) : Bool)

@[expose] public def inList (names : List (ByteArray)) (wanted : ByteArray) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (name : ByteArray) => (if found then true else bEq (name) (wanted)))) (false) (names) : Bool)

@[expose] public def joinWith (names : List (ByteArray)) (glue : String) : ByteArray := LexLeanStd.Text.joinWith (names) (rtext (glue))

@[expose] public def lastName (names : List (ByteArray)) : ByteArray := (LexLeanCollections.listFold ((fun (_last : ByteArray) (name : ByteArray) => name)) (rtext ("")) (names) : ByteArray)

@[expose] public def headName (names : List (ByteArray)) : ByteArray := (match names with | List.nil => rtext ("") | List.cons head _ => head)

@[expose] public def nthName : (names : List (ByteArray)) -> («at» : Nat) -> Option (ByteArray)
  | List.nil, _at => Option.none
  | List.cons head rest, «at» => (match «at» with | Nat.zero => Option.some (head) | Nat.succ below => nthName (rest) (below))

@[expose] public def orElse (lead : Option (ByteArray)) (other : Option (ByteArray)) : Option (ByteArray) := (match lead with | Option.some stop => Option.some (stop) | Option.none => other)

@[expose] public def inRangeB (byte : UInt8) (low : UInt8) (high : UInt8) : Bool := (match (LexLeanRuntime.checkedSubtract (byte) (low) : Option (UInt8)) with | Option.some _ => (match (LexLeanRuntime.checkedSubtract (high) (byte) : Option (UInt8)) with | Option.some _ => true | Option.none => false) | Option.none => false)

@[expose] public def identByte (raw : ByteArray) («at» : Nat) : Bool := (match LexLeanStd.Text.byteAt (raw) («at») with | Option.some byte => (if inRangeB (byte) ((97 : UInt8)) ((122 : UInt8)) then true else (if inRangeB (byte) ((65 : UInt8)) ((90 : UInt8)) then true else (if (LexLeanRuntime.equal (byte) ((95 : UInt8)) : Bool) then true else (if (Nat.blt (0) («at»)) then inRangeB (byte) ((48 : UInt8)) ((57 : UInt8)) else false)))) | Option.none => false)

@[expose] public def identShape (raw : ByteArray) : Bool := (if (Nat.blt (0) ((LexLeanRuntime.length (raw) : Nat))) then (Nat.beq (((LexLeanCollections.iterateUntil ((fun («at» : Nat) => (if identByte (raw) («at») then Option.some ((«at» + 1)) else Option.none))) ((LexLeanRuntime.length (raw) : Nat)) (0) : (Prod (Nat) (Bool)))).1) ((LexLeanRuntime.length (raw) : Nat))) else false)

@[expose] public def keywords : List (ByteArray) := (rtext ("abstract") :: (rtext ("as") :: (rtext ("async") :: (rtext ("await") :: (rtext ("become") :: (rtext ("box") :: (rtext ("break") :: (rtext ("const") :: (rtext ("continue") :: (rtext ("crate") :: (rtext ("do") :: (rtext ("dyn") :: (rtext ("else") :: (rtext ("enum") :: (rtext ("extern") :: (rtext ("false") :: (rtext ("final") :: (rtext ("fn") :: (rtext ("for") :: (rtext ("if") :: (rtext ("impl") :: (rtext ("in") :: (rtext ("let") :: (rtext ("loop") :: (rtext ("macro") :: (rtext ("match") :: (rtext ("mod") :: (rtext ("move") :: (rtext ("mut") :: (rtext ("override") :: (rtext ("priv") :: (rtext ("pub") :: (rtext ("ref") :: (rtext ("return") :: (rtext ("self") :: (rtext ("Self") :: (rtext ("static") :: (rtext ("struct") :: (rtext ("super") :: (rtext ("trait") :: (rtext ("true") :: (rtext ("try") :: (rtext ("type") :: (rtext ("typeof") :: (rtext ("unsafe") :: (rtext ("unsized") :: (rtext ("use") :: (rtext ("virtual") :: (rtext ("where") :: (rtext ("while") :: (rtext ("yield") :: (rtext ("_") :: ([] : List (ByteArray))))))))))))))))))))))))))))))))))))))))))))))))))))))

@[expose] public def identErr (what : String) (name : ByteArray) : Option (ByteArray) := (if (if identShape (name) then (!inList (keywords) (name)) else false) then Option.none else Option.some (rcat (rtext ("runtime support: identifier: ")) (rcat (rtext (what)) (rcat (rtext (" ")) (rcat (rtick (name)) (rtext (" is not an identifier")))))))

@[expose] public def identsErr (what : String) (names : List (ByteArray)) : Option (ByteArray) := (LexLeanCollections.listFold ((fun (found : Option (ByteArray)) (name : ByteArray) => (match found with | Option.some stop => Option.some (stop) | Option.none => identErr (what) (name)))) (Option.none) (names) : Option (ByteArray))

@[expose] public def segmentsErr (what : String) (path : List (ByteArray)) : Option (ByteArray) := (LexLeanCollections.listFold ((fun (found : Option (ByteArray)) (segment : ByteArray) => (match found with | Option.some stop => Option.some (stop) | Option.none => (if (if bEq (segment) (rtext ("Self")) then true else (if bEq (segment) (rtext ("self")) then true else (if bEq (segment) (rtext ("crate")) then true else bEq (segment) (rtext ("super"))))) then Option.none else identErr (what) (segment))))) (Option.none) (path) : Option (ByteArray))

@[expose] public def derivesErr (what : String) (names : List (ByteArray)) : Option (ByteArray) := (LexLeanCollections.listFold ((fun (found : Option (ByteArray)) (name : ByteArray) => (match found with | Option.some stop => Option.some (stop) | Option.none => (if (if bEq (name) (rtext ("Clone")) then true else (if bEq (name) (rtext ("Copy")) then true else (if bEq (name) (rtext ("Debug")) then true else (if bEq (name) (rtext ("Eq")) then true else bEq (name) (rtext ("PartialEq")))))) then Option.none else Option.some (rcat (rtext ("runtime support: derive: ")) (rcat (rtext (what)) (rcat (rtext (" derives ")) (rcat (rtick (name)) (rtext (", which is not in the closed table")))))))))) (Option.none) (names) : Option (ByteArray))

@[expose] public def literalErr (scalars : Nat) : Option (ByteArray) := (if (Nat.beq (scalars) (1)) then Option.none else Option.some (rcat (rtext ("runtime support: char-literal: ")) (rcat (rtext ("a character literal of ")) (rcat (rdec (scalars)) (rtext (" scalar values"))))))

@[expose] public def opPrec (op : Nat) : Nat := (if (Nat.beq (op) (6)) then 3 else (if (Nat.beq (op) (5)) then 4 else (if (Nat.beq (op) (7)) then 5 else (if (Nat.beq (op) (8)) then 5 else (if (Nat.beq (op) (9)) then 5 else (if (Nat.beq (op) (10)) then 5 else (if (Nat.beq (op) (11)) then 5 else (if (Nat.beq (op) (0)) then 9 else (if (Nat.beq (op) (1)) then 9 else 10)))))))))

@[expose] public def exprPrec (e : Expr) : Nat := (match e with | Expr.EVar _ => 14 | Expr.ELit _ => 14 | Expr.EPath _ => 14 | Expr.ECall _ _ => 13 | Expr.EMethod _ _ _ => 13 | Expr.EField _ _ => 13 | Expr.EFieldNamed _ _ => 13 | Expr.EBorrow _ => 12 | Expr.EEquals _ _ => 5 | Expr.EMatch _ _ => 14 | Expr.ELet _ _ _ => 1 | Expr.EAssign _ _ => 1 | Expr.EIf _ _ _ => 14 | Expr.EWhile _ _ => 14 | Expr.EWhileLet _ _ _ => 14 | Expr.ELoop _ => 14 | Expr.EFor _ _ _ => 14 | Expr.EBreak => 1 | Expr.EReturn _ => 1 | Expr.ETuple _ => 14 | Expr.EStructLit _ _ _ => 14 | Expr.EBinary op _ _ => opPrec (op) | Expr.ENot _ => 12 | Expr.ECast _ _ => 11 | Expr.EIndex _ _ => 13 | Expr.ERange _ _ _ => 14 | Expr.EDeref _ => 12 | Expr.EBorrowMut _ => 12 | Expr.EMethodOf _ _ _ _ => 13 | Expr.EBlock _ _ => 14 | Expr.ENone => 14 | Expr.ENil => 14 | Expr.ECons _ _ => 14 | Expr.ELetS _ _ _ _ => 14 | Expr.EArm _ _ _ _ => 14)

@[expose] public def endsInCast : (e : Expr) -> Bool
  | Expr.EVar _ => false
  | Expr.ELit _ => false
  | Expr.EPath _ => false
  | Expr.ECall _ _ => false
  | Expr.EMethod _ _ _ => false
  | Expr.EField _ _ => false
  | Expr.EFieldNamed _ _ => false
  | Expr.EBorrow _ => false
  | Expr.EEquals _ _ => false
  | Expr.EMatch _ _ => false
  | Expr.ELet _ _ _ => false
  | Expr.EAssign _ _ => false
  | Expr.EIf _ _ _ => false
  | Expr.EWhile _ _ => false
  | Expr.EWhileLet _ _ _ => false
  | Expr.ELoop _ => false
  | Expr.EFor _ _ _ => false
  | Expr.EBreak => false
  | Expr.EReturn _ => false
  | Expr.ETuple _ => false
  | Expr.EStructLit _ _ _ => false
  | Expr.EBinary op _ right => (if (Nat.blt (opPrec (op)) (exprPrec (right))) then endsInCast (right) else false)
  | Expr.ENot _ => false
  | Expr.ECast _ _ => true
  | Expr.EIndex _ _ => false
  | Expr.ERange _ _ _ => false
  | Expr.EDeref _ => false
  | Expr.EBorrowMut _ => false
  | Expr.EMethodOf _ _ _ _ => false
  | Expr.EBlock _ _ => false
  | Expr.ENone => false
  | Expr.ENil => false
  | Expr.ECons _ _ => false
  | Expr.ELetS _ _ _ _ => false
  | Expr.EArm _ _ _ _ => false

@[expose] public def leftParenthesized (op : Nat) (left : Expr) : Bool := (if (if (Nat.beq (op) (8)) then endsInCast (left) else false) then true else (Nat.blt (exprPrec (left)) ((if (Nat.beq (opPrec (op)) (5)) then 6 else opPrec (op)))))

@[expose] public def exposed : (e : Expr) -> Bool
  | Expr.EVar _ => false
  | Expr.ELit _ => false
  | Expr.EPath _ => false
  | Expr.ECall _ _ => false
  | Expr.EMethod receiver _ _ => (if (Nat.ble (13) (exprPrec (receiver))) then exposed (receiver) else false)
  | Expr.EField receiver _ => (if (Nat.ble (13) (exprPrec (receiver))) then exposed (receiver) else false)
  | Expr.EFieldNamed receiver _ => (if (Nat.ble (13) (exprPrec (receiver))) then exposed (receiver) else false)
  | Expr.EBorrow inner => (if (Nat.ble (12) (exprPrec (inner))) then exposed (inner) else false)
  | Expr.EEquals a b => (if (if (Nat.ble (6) (exprPrec (a))) then exposed (a) else false) then true else (if (Nat.ble (6) (exprPrec (b))) then exposed (b) else false))
  | Expr.EMatch _ _ => false
  | Expr.ELet _ _ _ => false
  | Expr.EAssign place value => (if exposed (place) then true else exposed (value))
  | Expr.EIf _ _ _ => false
  | Expr.EWhile _ _ => false
  | Expr.EWhileLet _ _ _ => false
  | Expr.ELoop _ => false
  | Expr.EFor _ _ _ => false
  | Expr.EBreak => false
  | Expr.EReturn value => exposed (value)
  | Expr.ETuple _ => false
  | Expr.EStructLit _ _ _ => true
  | Expr.EBinary op a b => (if (if leftParenthesized (op) (a) then false else exposed (a)) then true else (if (Nat.ble ((opPrec (op) + 1)) (exprPrec (b))) then exposed (b) else false))
  | Expr.ENot inner => (if (Nat.ble (12) (exprPrec (inner))) then exposed (inner) else false)
  | Expr.ECast inner _ => (if (Nat.ble (11) (exprPrec (inner))) then exposed (inner) else false)
  | Expr.EIndex receiver _ => (if (Nat.ble (13) (exprPrec (receiver))) then exposed (receiver) else false)
  | Expr.ERange receiver _ _ => (if (Nat.ble (13) (exprPrec (receiver))) then exposed (receiver) else false)
  | Expr.EDeref inner => (if (Nat.ble (12) (exprPrec (inner))) then exposed (inner) else false)
  | Expr.EBorrowMut inner => (if (Nat.ble (12) (exprPrec (inner))) then exposed (inner) else false)
  | Expr.EMethodOf receiver _ _ _ => (if (Nat.ble (13) (exprPrec (receiver))) then exposed (receiver) else false)
  | Expr.EBlock _ _ => false
  | Expr.ENone => false
  | Expr.ENil => false
  | Expr.ECons _ _ => false
  | Expr.ELetS _ _ _ _ => false
  | Expr.EArm _ _ _ _ => false

@[expose] public def headErr (e : Expr) : Option (ByteArray) := (if exposed (e) then Option.some (rcat (rtext ("runtime support: struct-head: ")) (rtext ("a struct literal outside parentheses in the head of a condition, a loop, or a match"))) else Option.none)

@[expose] public def unscopedErr : (what : String) -> (ty : Ty) -> Option (ByteArray)
  | _what, Ty.TyUnit => Option.none
  | _what, Ty.TyBool => Option.none
  | _what, Ty.TyNat => Option.none
  | _what, Ty.TyFixed => Option.none
  | _what, Ty.TyOrdering => Option.none
  | _what, Ty.TySelf => Option.none
  | what, Ty.TyParam _ => Option.some (rcat (rtext ("runtime support: type-scope: ")) (rcat (rtext (what)) (rtext (" names a generic parameter, which the printer writes without the generics in scope"))))
  | what, Ty.TyRef inner => unscopedErr (what) (inner)
  | what, Ty.TyRefMut inner => unscopedErr (what) (inner)
  | what, Ty.TyPair a b => (match unscopedErr (what) (a) with | Option.some stop => Option.some (stop) | Option.none => unscopedErr (what) (b))
  | what, Ty.TyNamed _ arguments => unscopedErr (what) (arguments)
  | _what, Ty.TyWide => Option.none
  | _what, Ty.TySize => Option.none
  | _what, Ty.TyChar => Option.none
  | _what, Ty.TyStr => Option.none
  | what, Ty.TySlice inner => unscopedErr (what) (inner)
  | _what, Ty.TyUnknown => Option.none
  | _what, Ty.TyAbsent => Option.none
  | _what, Ty.TyNil => Option.none
  | what, Ty.TyCons head tail => (match unscopedErr (what) (head) with | Option.some stop => Option.some (stop) | Option.none => unscopedErr (what) (tail))

@[expose] public def tyLen : (chain : Ty) -> Nat
  | Ty.TyUnit => 0
  | Ty.TyBool => 0
  | Ty.TyNat => 0
  | Ty.TyFixed => 0
  | Ty.TyOrdering => 0
  | Ty.TySelf => 0
  | Ty.TyParam _ => 0
  | Ty.TyRef _ => 0
  | Ty.TyRefMut _ => 0
  | Ty.TyPair _ _ => 0
  | Ty.TyNamed _ _ => 0
  | Ty.TyWide => 0
  | Ty.TySize => 0
  | Ty.TyChar => 0
  | Ty.TyStr => 0
  | Ty.TySlice _ => 0
  | Ty.TyUnknown => 0
  | Ty.TyAbsent => 0
  | Ty.TyNil => 0
  | Ty.TyCons _ tail => (tyLen (tail) + 1)

@[expose] public def patLen : (chain : Pat) -> Nat
  | Pat.PWild => 0
  | Pat.PBind _ => 0
  | Pat.PBindMut _ => 0
  | Pat.PPath _ => 0
  | Pat.PTuple _ => 0
  | Pat.PTupleStruct _ _ => 0
  | Pat.PStruct _ _ _ => 0
  | Pat.PLit _ => 0
  | Pat.PNil => 0
  | Pat.PCons _ tail => (patLen (tail) + 1)

@[expose] public def exprLen : (chain : Expr) -> Nat
  | Expr.EVar _ => 0
  | Expr.ELit _ => 0
  | Expr.EPath _ => 0
  | Expr.ECall _ _ => 0
  | Expr.EMethod _ _ _ => 0
  | Expr.EField _ _ => 0
  | Expr.EFieldNamed _ _ => 0
  | Expr.EBorrow _ => 0
  | Expr.EEquals _ _ => 0
  | Expr.EMatch _ _ => 0
  | Expr.ELet _ _ _ => 0
  | Expr.EAssign _ _ => 0
  | Expr.EIf _ _ _ => 0
  | Expr.EWhile _ _ => 0
  | Expr.EWhileLet _ _ _ => 0
  | Expr.ELoop _ => 0
  | Expr.EFor _ _ _ => 0
  | Expr.EBreak => 0
  | Expr.EReturn _ => 0
  | Expr.ETuple _ => 0
  | Expr.EStructLit _ _ _ => 0
  | Expr.EBinary _ _ _ => 0
  | Expr.ENot _ => 0
  | Expr.ECast _ _ => 0
  | Expr.EIndex _ _ => 0
  | Expr.ERange _ _ _ => 0
  | Expr.EDeref _ => 0
  | Expr.EBorrowMut _ => 0
  | Expr.EMethodOf _ _ _ _ => 0
  | Expr.EBlock _ _ => 0
  | Expr.ENone => 0
  | Expr.ENil => 0
  | Expr.ECons _ tail => (exprLen (tail) + 1)
  | Expr.ELetS _ _ _ _ => 0
  | Expr.EArm _ _ _ _ => 0

@[expose] public def typeArity (name : ByteArray) : Option (Nat) := (if (LexLeanRuntime.equal (name) (rtext ("Result")) : Bool) then Option.some (2) else (if (LexLeanRuntime.equal (name) (rtext ("Option")) : Bool) then Option.some (1) else (if (LexLeanRuntime.equal (name) (rtext ("AtomicU64")) : Bool) then Option.some (0) else (if (LexLeanRuntime.equal (name) (rtext ("Rc")) : Bool) then Option.some (1) else (if (LexLeanRuntime.equal (name) (rtext ("Vec")) : Bool) then Option.some (1) else (if (LexLeanRuntime.equal (name) (rtext ("String")) : Bool) then Option.some (0) else Option.none))))))

@[expose] public def typeHeap (name : ByteArray) : Bool := (if (LexLeanRuntime.equal (name) (rtext ("Rc")) : Bool) then true else (if (LexLeanRuntime.equal (name) (rtext ("Vec")) : Bool) then true else (if (LexLeanRuntime.equal (name) (rtext ("String")) : Bool) then true else false)))

@[expose] public def isInteger (name : ByteArray) : Bool := (if (LexLeanRuntime.equal (name) (rtext ("u8")) : Bool) then true else (if (LexLeanRuntime.equal (name) (rtext ("u16")) : Bool) then true else (if (LexLeanRuntime.equal (name) (rtext ("u32")) : Bool) then true else (if (LexLeanRuntime.equal (name) (rtext ("u64")) : Bool) then true else (if (LexLeanRuntime.equal (name) (rtext ("u128")) : Bool) then true else (if (LexLeanRuntime.equal (name) (rtext ("i8")) : Bool) then true else (if (LexLeanRuntime.equal (name) (rtext ("i16")) : Bool) then true else (if (LexLeanRuntime.equal (name) (rtext ("i32")) : Bool) then true else (if (LexLeanRuntime.equal (name) (rtext ("i64")) : Bool) then true else (if (LexLeanRuntime.equal (name) (rtext ("i128")) : Bool) then true else false))))))))))

@[expose] public def callDirectArity (owner : ByteArray) (name : ByteArray) : Option (Nat) := (if (if (LexLeanRuntime.equal (owner) (rtext ("AtomicU64")) : Bool) then (LexLeanRuntime.equal (name) (rtext ("new")) : Bool) else false) then Option.some (1) else (if (if (LexLeanRuntime.equal (owner) (rtext ("Rc")) : Bool) then (LexLeanRuntime.equal (name) (rtext ("new")) : Bool) else false) then Option.some (1) else (if (if (LexLeanRuntime.equal (owner) (rtext ("Rc")) : Bool) then (LexLeanRuntime.equal (name) (rtext ("from")) : Bool) else false) then Option.some (1) else (if (if (LexLeanRuntime.equal (owner) (rtext ("Rc")) : Bool) then (LexLeanRuntime.equal (name) (rtext ("try_unwrap")) : Bool) else false) then Option.some (1) else (if (if (LexLeanRuntime.equal (owner) (rtext ("Rc")) : Bool) then (LexLeanRuntime.equal (name) (rtext ("get_mut")) : Bool) else false) then Option.some (1) else (if (if (LexLeanRuntime.equal (owner) (rtext ("Vec")) : Bool) then (LexLeanRuntime.equal (name) (rtext ("new")) : Bool) else false) then Option.some (0) else (if (if (LexLeanRuntime.equal (owner) (rtext ("Vec")) : Bool) then (LexLeanRuntime.equal (name) (rtext ("with_capacity")) : Bool) else false) then Option.some (1) else (if (if (LexLeanRuntime.equal (owner) (rtext ("String")) : Bool) then (LexLeanRuntime.equal (name) (rtext ("new")) : Bool) else false) then Option.some (0) else (if (if (LexLeanRuntime.equal (owner) (rtext ("mem")) : Bool) then (LexLeanRuntime.equal (name) (rtext ("replace")) : Bool) else false) then Option.some (2) else (if (if (LexLeanRuntime.equal (owner) (rtext ("str")) : Bool) then (LexLeanRuntime.equal (name) (rtext ("from_utf8")) : Bool) else false) then Option.some (1) else (if (if (LexLeanRuntime.equal (owner) (rtext ("usize")) : Bool) then (LexLeanRuntime.equal (name) (rtext ("from")) : Bool) else false) then Option.some (1) else (if (if (LexLeanRuntime.equal (owner) (rtext ("usize")) : Bool) then (LexLeanRuntime.equal (name) (rtext ("try_from")) : Bool) else false) then Option.some (1) else Option.none))))))))))))

@[expose] public def callDirectHeap (owner : ByteArray) (name : ByteArray) : Bool := (if (if (LexLeanRuntime.equal (owner) (rtext ("AtomicU64")) : Bool) then (LexLeanRuntime.equal (name) (rtext ("new")) : Bool) else false) then false else (if (if (LexLeanRuntime.equal (owner) (rtext ("Rc")) : Bool) then (LexLeanRuntime.equal (name) (rtext ("new")) : Bool) else false) then true else (if (if (LexLeanRuntime.equal (owner) (rtext ("Rc")) : Bool) then (LexLeanRuntime.equal (name) (rtext ("from")) : Bool) else false) then true else (if (if (LexLeanRuntime.equal (owner) (rtext ("Rc")) : Bool) then (LexLeanRuntime.equal (name) (rtext ("try_unwrap")) : Bool) else false) then true else (if (if (LexLeanRuntime.equal (owner) (rtext ("Rc")) : Bool) then (LexLeanRuntime.equal (name) (rtext ("get_mut")) : Bool) else false) then true else (if (if (LexLeanRuntime.equal (owner) (rtext ("Vec")) : Bool) then (LexLeanRuntime.equal (name) (rtext ("new")) : Bool) else false) then true else (if (if (LexLeanRuntime.equal (owner) (rtext ("Vec")) : Bool) then (LexLeanRuntime.equal (name) (rtext ("with_capacity")) : Bool) else false) then true else (if (if (LexLeanRuntime.equal (owner) (rtext ("String")) : Bool) then (LexLeanRuntime.equal (name) (rtext ("new")) : Bool) else false) then true else (if (if (LexLeanRuntime.equal (owner) (rtext ("mem")) : Bool) then (LexLeanRuntime.equal (name) (rtext ("replace")) : Bool) else false) then false else (if (if (LexLeanRuntime.equal (owner) (rtext ("str")) : Bool) then (LexLeanRuntime.equal (name) (rtext ("from_utf8")) : Bool) else false) then false else (if (if (LexLeanRuntime.equal (owner) (rtext ("usize")) : Bool) then (LexLeanRuntime.equal (name) (rtext ("from")) : Bool) else false) then false else (if (if (LexLeanRuntime.equal (owner) (rtext ("usize")) : Bool) then (LexLeanRuntime.equal (name) (rtext ("try_from")) : Bool) else false) then false else false))))))))))))

@[expose] public def callArity (owner : ByteArray) (name : ByteArray) : Option (Nat) := (match callDirectArity (owner) (name) with | Option.some count => Option.some (count) | Option.none => (if isInteger (owner) then callDirectArity (rtext ("usize")) (name) else Option.none))

@[expose] public def callHeap (owner : ByteArray) (name : ByteArray) : Bool := (match callDirectArity (owner) (name) with | Option.some _ => callDirectHeap (owner) (name) | Option.none => (if isInteger (owner) then callDirectHeap (rtext ("usize")) (name) else false))

@[expose] public def wrapperArity (name : ByteArray) : Option (Nat) := (if (LexLeanRuntime.equal (name) (rtext ("Some")) : Bool) then Option.some (1) else (if (LexLeanRuntime.equal (name) (rtext ("Ok")) : Bool) then Option.some (1) else (if (LexLeanRuntime.equal (name) (rtext ("Err")) : Bool) then Option.some (1) else Option.none)))

@[expose] public def methodArity (name : ByteArray) : Option (Nat) := (if (LexLeanRuntime.equal (name) (rtext ("fetch_add")) : Bool) then Option.some (2) else (if (LexLeanRuntime.equal (name) (rtext ("load")) : Bool) then Option.some (1) else (if (LexLeanRuntime.equal (name) (rtext ("cmp")) : Bool) then Option.some (1) else (if (LexLeanRuntime.equal (name) (rtext ("len")) : Bool) then Option.some (0) else (if (LexLeanRuntime.equal (name) (rtext ("is_empty")) : Bool) then Option.some (0) else (if (LexLeanRuntime.equal (name) (rtext ("chars")) : Bool) then Option.some (0) else (if (LexLeanRuntime.equal (name) (rtext ("next")) : Bool) then Option.some (0) else (if (LexLeanRuntime.equal (name) (rtext ("min")) : Bool) then Option.some (1) else (if (LexLeanRuntime.equal (name) (rtext ("max")) : Bool) then Option.some (1) else (if (LexLeanRuntime.equal (name) (rtext ("push")) : Bool) then Option.some (1) else (if (LexLeanRuntime.equal (name) (rtext ("pop")) : Bool) then Option.some (0) else (if (LexLeanRuntime.equal (name) (rtext ("truncate")) : Bool) then Option.some (1) else (if (LexLeanRuntime.equal (name) (rtext ("push_str")) : Bool) then Option.some (1) else (if (LexLeanRuntime.equal (name) (rtext ("extend_from_slice")) : Bool) then Option.some (1) else (if (LexLeanRuntime.equal (name) (rtext ("iter")) : Bool) then Option.some (0) else (if (LexLeanRuntime.equal (name) (rtext ("rev")) : Bool) then Option.some (0) else (if (LexLeanRuntime.equal (name) (rtext ("get")) : Bool) then Option.some (1) else (if (LexLeanRuntime.equal (name) (rtext ("copied")) : Bool) then Option.some (0) else (if (LexLeanRuntime.equal (name) (rtext ("ok")) : Bool) then Option.some (0) else (if (LexLeanRuntime.equal (name) (rtext ("clone")) : Bool) then Option.some (0) else (if (LexLeanRuntime.equal (name) (rtext ("to_string")) : Bool) then Option.some (0) else (if (LexLeanRuntime.equal (name) (rtext ("as_bytes")) : Bool) then Option.some (0) else (if (LexLeanRuntime.equal (name) (rtext ("strip_prefix")) : Bool) then Option.some (1) else (if (LexLeanRuntime.equal (name) (rtext ("starts_with")) : Bool) then Option.some (1) else (if (LexLeanRuntime.equal (name) (rtext ("split")) : Bool) then Option.some (1) else (if (LexLeanRuntime.equal (name) (rtext ("collect")) : Bool) then Option.some (0) else (if (LexLeanRuntime.equal (name) (rtext ("is_ascii_digit")) : Bool) then Option.some (0) else Option.none)))))))))))))))))))))))))))

@[expose] public def methodHeap (name : ByteArray) : Bool := (if (LexLeanRuntime.equal (name) (rtext ("push")) : Bool) then true else (if (LexLeanRuntime.equal (name) (rtext ("pop")) : Bool) then true else (if (LexLeanRuntime.equal (name) (rtext ("truncate")) : Bool) then true else (if (LexLeanRuntime.equal (name) (rtext ("push_str")) : Bool) then true else (if (LexLeanRuntime.equal (name) (rtext ("extend_from_slice")) : Bool) then true else (if (LexLeanRuntime.equal (name) (rtext ("to_string")) : Bool) then true else (if (LexLeanRuntime.equal (name) (rtext ("collect")) : Bool) then true else false)))))))

@[expose] public def turbofishTypes (name : ByteArray) : Option (Nat) := (if (LexLeanRuntime.equal (name) (rtext ("parse")) : Bool) then Option.some (1) else (if (LexLeanRuntime.equal (name) (rtext ("collect")) : Bool) then Option.some (1) else Option.none))

@[expose] public def turbofishArgs (name : ByteArray) : Nat := (if (LexLeanRuntime.equal (name) (rtext ("parse")) : Bool) then 0 else (if (LexLeanRuntime.equal (name) (rtext ("collect")) : Bool) then 0 else 0))

@[expose] public def stdTraitMethod (name : ByteArray) : Option (ByteArray) := (if (LexLeanRuntime.equal (name) (rtext ("Clone")) : Bool) then Option.some (rtext ("clone")) else (if (LexLeanRuntime.equal (name) (rtext ("Drop")) : Bool) then Option.some (rtext ("drop")) else Option.none))

@[expose] public def stdTraitExclusive (name : ByteArray) : Bool := (if (LexLeanRuntime.equal (name) (rtext ("Clone")) : Bool) then false else (if (LexLeanRuntime.equal (name) (rtext ("Drop")) : Bool) then true else false))

@[expose] public def isWrapperName (name : ByteArray) : Bool := (if (LexLeanRuntime.equal (name) (rtext ("Some")) : Bool) then true else (if (LexLeanRuntime.equal (name) (rtext ("Ok")) : Bool) then true else (if (LexLeanRuntime.equal (name) (rtext ("Err")) : Bool) then true else false)))

@[expose] public def findTrait (sc : Sc) (wanted : ByteArray) : Option (TraitE) := (LexLeanCollections.listFold ((fun (found : Option (TraitE)) (entry : TraitE) => (match found with | Option.some held => Option.some (held) | Option.none => (if bEq ((entry).name) (wanted) then Option.some (entry) else Option.none)))) (Option.none) ((sc).traits) : Option (TraitE))

@[expose] public def findStruct (sc : Sc) (wanted : ByteArray) : Option (StructE) := (LexLeanCollections.listFold ((fun (found : Option (StructE)) (entry : StructE) => (match found with | Option.some held => Option.some (held) | Option.none => (if bEq ((entry).name) (wanted) then Option.some (entry) else Option.none)))) (Option.none) ((sc).structs) : Option (StructE))

@[expose] public def findEnum (sc : Sc) (wanted : ByteArray) : Option (EnumE) := (LexLeanCollections.listFold ((fun (found : Option (EnumE)) (entry : EnumE) => (match found with | Option.some held => Option.some (held) | Option.none => (if bEq ((entry).name) (wanted) then Option.some (entry) else Option.none)))) (Option.none) ((sc).enums) : Option (EnumE))

@[expose] public def findAlias (sc : Sc) (wanted : ByteArray) : Option (Entry) := (LexLeanCollections.listFold ((fun (found : Option (Entry)) (entry : Entry) => (match found with | Option.some held => Option.some (held) | Option.none => (if bEq ((entry).name) (wanted) then Option.some (entry) else Option.none)))) (Option.none) ((sc).aliases) : Option (Entry))

@[expose] public def findFunction (sc : Sc) (wanted : ByteArray) : Option (Entry) := (LexLeanCollections.listFold ((fun (found : Option (Entry)) (entry : Entry) => (match found with | Option.some held => Option.some (held) | Option.none => (if bEq ((entry).name) (wanted) then Option.some (entry) else Option.none)))) (Option.none) ((sc).functions) : Option (Entry))

@[expose] public def methodCounts (sc : Sc) (wanted : ByteArray) : List (Nat) := (LexLeanCollections.listFold ((fun (counts : List (Nat)) (entry : Entry) => (if bEq ((entry).name) (wanted) then (LexLeanRuntime.append (counts) (((entry).n :: ([] : List (Nat)))) : List (Nat)) else counts))) (([] : List (Nat))) ((sc).methods) : List (Nat))

@[expose] public def hasCount (counts : List (Nat)) (given : Nat) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (count : Nat) => (if found then true else (Nat.beq (count) (given))))) (false) (counts) : Bool)

@[expose] public def joinCounts (counts : List (Nat)) : ByteArray := (LexLeanCollections.listFold ((fun (text : ByteArray) (count : Nat) => (if (LexLeanRuntime.equal (text) (rtext ("")) : Bool) then rdec (count) else rcat (text) (rcat (rtext ("/")) (rdec (count)))))) (rtext ("")) (counts) : ByteArray)

@[expose] public def variantName (v : Variant) : ByteArray := (match v with | Variant.VUnit n => n | Variant.VTuple n _ => n | Variant.VRecord n _ _ => n)

@[expose] public def variantTypes (v : Variant) : Nat := (match v with | Variant.VUnit _ => 0 | Variant.VTuple _ ts => tyLen (ts) | Variant.VRecord _ _ ts => tyLen (ts))

@[expose] public def findVariant (variants : List (Variant)) (wanted : ByteArray) : Option (Variant) := (LexLeanCollections.listFold ((fun (found : Option (Variant)) (entry : Variant) => (match found with | Option.some held => Option.some (held) | Option.none => (if bEq (variantName (entry)) (wanted) then Option.some (entry) else Option.none)))) (Option.none) (variants) : Option (Variant))

@[expose] public def variantTuple (variants : List (Variant)) (wanted : ByteArray) : Option (Nat) := (match findVariant (variants) (wanted) with | Option.none => Option.none | Option.some held => (match held with | Variant.VUnit _ => Option.none | Variant.VTuple _ ts => Option.some (tyLen (ts)) | Variant.VRecord _ _ _ => Option.none))

@[expose] public def variantRecord (variants : List (Variant)) (wanted : ByteArray) : Option (List (ByteArray)) := (match findVariant (variants) (wanted) with | Option.none => Option.none | Option.some held => (match held with | Variant.VUnit _ => Option.none | Variant.VTuple _ _ => Option.none | Variant.VRecord _ fs _ => Option.some (fs)))

@[expose] public def recordFields (sc : Sc) (path : List (ByteArray)) : Option (List (ByteArray)) := (match path with | List.nil => Option.none | List.cons head rest => (match rest with | List.nil => (match findStruct (sc) (head) with | Option.some held => Option.some ((held).fields) | Option.none => Option.none) | List.cons middle more => (match more with | List.nil => (match findEnum (sc) (head) with | Option.some held => variantRecord ((held).variants) (middle) | Option.none => Option.none) | List.cons _ _ => Option.none)))

@[expose] public def tupleStructArity (sc : Sc) (name : ByteArray) : Option (Nat) := (match findStruct (sc) (name) with | Option.some held => Option.some ((LexLeanRuntime.length ((held).fields) : Nat)) | Option.none => Option.none)

@[expose] public def variantArity (sc : Sc) (owner : ByteArray) (name : ByteArray) : Option (Nat) := (match findEnum (sc) (owner) with | Option.some held => variantTuple ((held).variants) (name) | Option.none => Option.none)

@[expose] public def indexNames : (tys : Ty) -> («at» : Nat) -> List (ByteArray)
  | Ty.TyUnit, _at => ([] : List (ByteArray))
  | Ty.TyBool, _at => ([] : List (ByteArray))
  | Ty.TyNat, _at => ([] : List (ByteArray))
  | Ty.TyFixed, _at => ([] : List (ByteArray))
  | Ty.TyOrdering, _at => ([] : List (ByteArray))
  | Ty.TySelf, _at => ([] : List (ByteArray))
  | Ty.TyParam _, _at => ([] : List (ByteArray))
  | Ty.TyRef _, _at => ([] : List (ByteArray))
  | Ty.TyRefMut _, _at => ([] : List (ByteArray))
  | Ty.TyPair _ _, _at => ([] : List (ByteArray))
  | Ty.TyNamed _ _, _at => ([] : List (ByteArray))
  | Ty.TyWide, _at => ([] : List (ByteArray))
  | Ty.TySize, _at => ([] : List (ByteArray))
  | Ty.TyChar, _at => ([] : List (ByteArray))
  | Ty.TyStr, _at => ([] : List (ByteArray))
  | Ty.TySlice _, _at => ([] : List (ByteArray))
  | Ty.TyUnknown, _at => ([] : List (ByteArray))
  | Ty.TyAbsent, _at => ([] : List (ByteArray))
  | Ty.TyNil, _at => ([] : List (ByteArray))
  | Ty.TyCons _ tail, «at» => (rdec («at») :: indexNames (tail) ((«at» + 1)))

@[expose] public def findVar (vars : List (Var)) (wanted : ByteArray) : Option (Var) := (LexLeanCollections.listFold ((fun (found : Option (Var)) (entry : Var) => (match found with | Option.some held => Option.some (held) | Option.none => (if bEq ((entry).name) (wanted) then Option.some (entry) else Option.none)))) (Option.none) (vars) : Option (Var))

@[expose] public def nthBounds : (gs : List (Generic)) -> («at» : Nat) -> List (ByteArray)
  | List.nil, _at => ([] : List (ByteArray))
  | List.cons head rest, «at» => (match «at» with | Nat.zero => (head).bounds | Nat.succ below => nthBounds (rest) (below))

@[expose] public def tyShow : (subject : Ty) -> ByteArray
  | Ty.TyUnit => rtext ("U")
  | Ty.TyBool => rtext ("B")
  | Ty.TyNat => rtext ("N")
  | Ty.TyFixed => rtext ("F")
  | Ty.TyOrdering => rtext ("O")
  | Ty.TySelf => rtext ("S")
  | Ty.TyParam n => rcat (rtext ("P")) (rcat (rdec (n)) (rtext (";")))
  | Ty.TyRef t => rcat (rtext ("R(")) (rcat (tyShow (t)) (rtext (")")))
  | Ty.TyRefMut t => rcat (rtext ("M(")) (rcat (tyShow (t)) (rtext (")")))
  | Ty.TyPair a b => rcat (rtext ("T(")) (rcat (tyShow (a)) (rcat (rtext (",")) (rcat (tyShow (b)) (rtext (")")))))
  | Ty.TyNamed name args => rcat (rtext ("D")) (rcat (rtick (name)) (rcat (rtext ("(")) (rcat (tyShow (args)) (rtext (")")))))
  | Ty.TyWide => rtext ("W")
  | Ty.TySize => rtext ("Z")
  | Ty.TyChar => rtext ("C")
  | Ty.TyStr => rtext ("Q")
  | Ty.TySlice t => rcat (rtext ("L(")) (rcat (tyShow (t)) (rtext (")")))
  | Ty.TyUnknown => rtext ("?")
  | Ty.TyAbsent => rtext ("-")
  | Ty.TyNil => rtext ("")
  | Ty.TyCons head tail => rcat (tyShow (head)) (rcat (rtext (";")) (tyShow (tail)))

@[expose] public def sigShow (sig : Sig) : ByteArray := rcat ((sig).name) (rcat (rtext ("|")) (rcat ((if (sig).receiver then rtext ("r") else rtext ("-"))) (rcat ((if (sig).exclusive then rtext ("x") else rtext ("-"))) (rcat (rtext ("|")) (rcat ((LexLeanCollections.listFold ((fun (text : ByteArray) (param : Param) => rcat (text) (rcat ((param).name) (rcat ((if (param).mutable then rtext (":m:") else rtext (":-:"))) (rcat (tyShow ((param).ty)) (rtext (","))))))) (rtext ("")) ((sig).params) : ByteArray)) (rcat (rtext ("|")) (tyShow ((sig).result))))))))

@[expose] public def bindsOf : (acc : List (Var)) -> (p : Pat) -> Except (ByteArray) (List (Var))
  | acc, Pat.PWild => Except.ok (acc)
  | acc, Pat.PBind name => (match identErr ("a pattern binding") (name) with | Option.some bad => Except.error (bad) | Option.none => (match findVar (acc) (name) with | Option.some _ => Except.error (rcat (rtext ("runtime support: pattern-duplicate: ")) (rcat (rtick (name)) (rtext (" is bound twice in one pattern")))) | Option.none => Except.ok ((({ name := name, mutable := false, ty := Ty.TyUnknown } : Var) :: acc))))
  | acc, Pat.PBindMut name => (match identErr ("a pattern binding") (name) with | Option.some bad => Except.error (bad) | Option.none => (match findVar (acc) (name) with | Option.some _ => Except.error (rcat (rtext ("runtime support: pattern-duplicate: ")) (rcat (rtick (name)) (rtext (" is bound twice in one pattern")))) | Option.none => Except.ok ((({ name := name, mutable := true, ty := Ty.TyUnknown } : Var) :: acc))))
  | acc, Pat.PPath _ => Except.ok (acc)
  | acc, Pat.PTuple items => bindsOf (acc) (items)
  | acc, Pat.PTupleStruct _ items => bindsOf (acc) (items)
  | acc, Pat.PStruct _ _ items => bindsOf (acc) (items)
  | acc, Pat.PLit _ => Except.ok (acc)
  | acc, Pat.PNil => Except.ok (acc)
  | acc, Pat.PCons head tail => (match bindsOf (acc) (head) with | Except.ok next => bindsOf (next) (tail) | Except.error problem => Except.error (problem))

@[expose] public def typeExpected (sc : Sc) (name : ByteArray) : Option (Nat) := (match findStruct (sc) (name) with | Option.some held => Option.some ((held).gcount) | Option.none => (match findEnum (sc) (name) with | Option.some held2 => Option.some ((held2).gcount) | Option.none => (match findAlias (sc) (name) with | Option.some held3 => Option.some ((held3).n) | Option.none => typeArity (name))))

@[expose] public def isTableHeap (sc : Sc) (name : ByteArray) : Bool := (match findStruct (sc) (name) with | Option.some _ => false | Option.none => (match findEnum (sc) (name) with | Option.some _ => false | Option.none => (match findAlias (sc) (name) with | Option.some _ => false | Option.none => typeHeap (name))))

@[expose] public def tyOk : (sc : Sc) -> (subject : Ty) -> (params : Nat) -> (hasSelf : Bool) -> Option (ByteArray)
  | _sc, Ty.TyUnit, _params, _hasSelf => Option.none
  | _sc, Ty.TyBool, _params, _hasSelf => Option.none
  | _sc, Ty.TyNat, _params, _hasSelf => Option.none
  | _sc, Ty.TyFixed, _params, _hasSelf => Option.none
  | _sc, Ty.TyOrdering, _params, _hasSelf => Option.none
  | _sc, Ty.TySelf, _params, hasSelf => (if hasSelf then Option.none else Option.some (rcat (rtext ("runtime support: self-type: ")) (rtext ("`Self` outside a trait or an impl"))))
  | _sc, Ty.TyParam «at», params, _hasSelf => (if (Nat.blt («at») (params)) then Option.none else Option.some (rcat (rtext ("runtime support: type-parameter: ")) (rcat (rtext ("parameter ")) (rcat (rdec («at»)) (rcat (rtext (" of ")) (rcat (rdec (params)) (rtext (" in scope"))))))))
  | sc, Ty.TyRef t, params, hasSelf => tyOk (sc) (t) (params) (hasSelf)
  | sc, Ty.TyRefMut t, params, hasSelf => tyOk (sc) (t) (params) (hasSelf)
  | sc, Ty.TyPair a b, params, hasSelf => (match tyOk (sc) (a) (params) (hasSelf) with | Option.some stop => Option.some (stop) | Option.none => tyOk (sc) (b) (params) (hasSelf))
  | sc, Ty.TyNamed name args, params, hasSelf => (match typeExpected (sc) (name) with | Option.none => Option.some (rcat (rtext ("runtime support: type-name: ")) (rcat (rtick (name)) (rtext (" is neither declared nor in the closed table")))) | Option.some expected => (if (if (sc).core then isTableHeap (sc) (name) else false) then Option.some (rcat (rtext ("runtime support: hidden-allocation: ")) (rcat (rtext ("rust-core renders ")) (rcat (rtick (name)) (rtext (", which requires heap allocation"))))) else (if (Nat.beq (expected) (tyLen (args))) then tyOk (sc) (args) (params) (hasSelf) else Option.some (rcat (rtext ("runtime support: type-arity: ")) (rcat (rtick (name)) (rcat (rtext (" takes ")) (rcat (rdec (expected)) (rcat (rtext (" arguments, given ")) (rdec (tyLen (args)))))))))))
  | _sc, Ty.TyWide, _params, _hasSelf => Option.none
  | _sc, Ty.TySize, _params, _hasSelf => Option.none
  | _sc, Ty.TyChar, _params, _hasSelf => Option.none
  | _sc, Ty.TyStr, _params, _hasSelf => Option.none
  | sc, Ty.TySlice t, params, hasSelf => tyOk (sc) (t) (params) (hasSelf)
  | _sc, Ty.TyUnknown, _params, _hasSelf => Option.none
  | _sc, Ty.TyAbsent, _params, _hasSelf => Option.none
  | _sc, Ty.TyNil, _params, _hasSelf => Option.none
  | sc, Ty.TyCons head tail, params, hasSelf => (match tyOk (sc) (head) (params) (hasSelf) with | Option.some stop => Option.some (stop) | Option.none => tyOk (sc) (tail) (params) (hasSelf))

@[expose] public def genericsOk (sc : Sc) (gs : List (Generic)) : Option (ByteArray) := (LexLeanCollections.listFold ((fun (found : Option (ByteArray)) (g : Generic) => (match found with | Option.some stop => Option.some (stop) | Option.none => (match identErr ("a generic parameter") ((g).name) with | Option.some bad => Option.some (bad) | Option.none => (LexLeanCollections.listFold ((fun (foundBound : Option (ByteArray)) (bound : ByteArray) => (match foundBound with | Option.some stop => Option.some (stop) | Option.none => (if (match findTrait (sc) (bound) with | Option.some _ => true | Option.none => (match stdTraitMethod (bound) with | Option.some _ => true | Option.none => false)) then Option.none else Option.some (rcat (rtext ("runtime support: bound-trait: ")) (rcat (rtick (rcat ((g).name) (rcat (rtext (": ")) (bound)))) (rtext (" names no declared trait")))))))) (Option.none) ((g).bounds) : Option (ByteArray)))))) (Option.none) (gs) : Option (ByteArray))

@[expose] public def paramsOk (sc : Sc) (ps : List (Param)) (params : Nat) (hasSelf : Bool) : Option (ByteArray) := (LexLeanCollections.listFold ((fun (found : Option (ByteArray)) (p : Param) => (match found with | Option.some stop => Option.some (stop) | Option.none => (match identErr ("a parameter name") ((p).name) with | Option.some bad => Option.some (bad) | Option.none => tyOk (sc) ((p).ty) (params) (hasSelf))))) (Option.none) (ps) : Option (ByteArray))

@[expose] public def sigOk (sc : Sc) (sig : Sig) (params : Nat) (hasSelf : Bool) : Option (ByteArray) := (match identErr ("a method name") ((sig).name) with | Option.some bad => Option.some (bad) | Option.none => (if (if (sig).exclusive then (!(sig).receiver) else false) then Option.some (rcat (rtext ("runtime support: function-receiver: ")) (rcat (rtick ((sig).name)) (rtext (" is exclusive without a receiver")))) else (match paramsOk (sc) ((sig).params) (params) (hasSelf) with | Option.some stop => Option.some (stop) | Option.none => tyOk (sc) ((sig).result) (params) (hasSelf))))

@[expose] public def provided (sc : Sc) (bounds : List (ByteArray)) (method : ByteArray) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (bound : ByteArray) => (if found then true else (match findTrait (sc) (bound) with | Option.some held => (LexLeanCollections.listFold ((fun (has : Bool) (sig : Sig) => (if has then true else bEq ((sig).name) (method)))) (false) ((held).sigs) : Bool) | Option.none => (match stdTraitMethod (bound) with | Option.some m => bEq (m) (method) | Option.none => false))))) (false) (bounds) : Bool)

@[expose] public def pathCore (sc : Sc) (path : List (ByteArray)) : Option (ByteArray) := (let head : ByteArray := headName (path); (match findEnum (sc) (head) with | Option.some held => (match nthName (path) (1) with | Option.some variant => (match findVariant ((held).variants) (variant) with | Option.some _ => Option.none | Option.none => Option.some (rcat (rtext ("runtime support: variant-name: ")) (rcat (rtick (joinWith (path) ("::"))) (rcat (rtext (" is not a variant of ")) (rtick (head)))))) | Option.none => Option.some (rcat (rtext ("runtime support: variant-name: ")) (rcat (rtick (joinWith (path) ("::"))) (rcat (rtext (" is not a variant of ")) (rtick (head)))))) | Option.none => (if (if inList ((sc).imported) (head) then true else (if (match findStruct (sc) (head) with | Option.some _ => true | Option.none => false) then true else (if inList ((sc).statics) (head) then true else (if isInteger (head) then true else (if bEq (head) (rtext ("None")) then true else (if bEq (head) (rtext ("Self")) then true else bEq (head) (rtext ("usize")))))))) then Option.none else Option.some (rcat (rtext ("runtime support: path-name: ")) (rcat (rtick (joinWith (path) ("::"))) (rcat (rtext (" starts at ")) (rcat (rtick (head)) (rtext (", which no `use` or item declares")))))))))

@[expose] public def pathOk (sc : Sc) (path : List (ByteArray)) : Option (ByteArray) := (match segmentsErr ("a path segment") (path) with | Option.some bad => Option.some (bad) | Option.none => pathCore (sc) (path))

@[expose] public def patWrapperPath (path : List (ByteArray)) : Bool := (match path with | List.nil => false | List.cons head rest => (match rest with | List.nil => (match wrapperArity (head) with | Option.some _ => true | Option.none => false) | List.cons _ _ => false))

@[expose] public def patArity (sc : Sc) (path : List (ByteArray)) (items : Pat) : Option (ByteArray) := (match nthName (path) (1) with | Option.none => Option.none | Option.some variant => (match findEnum (sc) (headName (path)) with | Option.none => Option.none | Option.some held => (match findVariant ((held).variants) (variant) with | Option.some found => (if (Nat.beq (variantTypes (found)) (patLen (items))) then Option.none else Option.some (rcat (rtext ("runtime support: pattern-fields: ")) (rcat (rtick (joinWith (path) ("::"))) (rcat (rtext (" is matched with ")) (rcat (rdec (patLen (items))) (rtext (" patterns"))))))) | Option.none => Option.some (rcat (rtext ("runtime support: pattern-fields: ")) (rcat (rtick (joinWith (path) ("::"))) (rcat (rtext (" is matched with ")) (rcat (rdec (patLen (items))) (rtext (" patterns")))))))))

@[expose] public def patFields (sc : Sc) (path : List (ByteArray)) (names : List (ByteArray)) : Option (ByteArray) := (match recordFields (sc) (path) with | Option.none => Option.none | Option.some held => (LexLeanCollections.listFold ((fun (found : Option (ByteArray)) (name : ByteArray) => (match found with | Option.some stop => Option.some (stop) | Option.none => (if inList (held) (name) then Option.none else Option.some (rcat (rtext ("runtime support: pattern-fields: ")) (rcat (rtick (joinWith (path) ("::"))) (rcat (rtext (" has no field ")) (rtick (name))))))))) (Option.none) (names) : Option (ByteArray)))

@[expose] public def patOk : (sc : Sc) -> (p : Pat) -> Option (ByteArray)
  | _sc, Pat.PWild => Option.none
  | _sc, Pat.PBind _ => Option.none
  | _sc, Pat.PBindMut _ => Option.none
  | sc, Pat.PPath path => pathOk (sc) (path)
  | sc, Pat.PTuple items => patOk (sc) (items)
  | sc, Pat.PTupleStruct path items => (if (if patWrapperPath (path) then true else false) then patOk (sc) (items) else (match pathOk (sc) (path) with | Option.some stop => Option.some (stop) | Option.none => (match patArity (sc) (path) (items) with | Option.some stop => Option.some (stop) | Option.none => patOk (sc) (items))))
  | sc, Pat.PStruct path names items => (match pathOk (sc) (path) with | Option.some stop => Option.some (stop) | Option.none => (match identsErr ("a pattern field name") (names) with | Option.some bad => Option.some (bad) | Option.none => (if (Nat.beq ((LexLeanRuntime.length (names) : Nat)) (patLen (items))) then (match patFields (sc) (path) (names) with | Option.some stop => Option.some (stop) | Option.none => patOk (sc) (items)) else Option.some (rcat (rtext ("runtime support: pattern-fields: ")) (rtext ("a struct pattern with unequal names and patterns"))))))
  | _sc, Pat.PLit scalars => literalErr (scalars)
  | _sc, Pat.PNil => Option.none
  | sc, Pat.PCons head tail => (match patOk (sc) (head) with | Option.some stop => Option.some (stop) | Option.none => patOk (sc) (tail))

@[expose] public def isAbsent (t : Ty) : Bool := (match t with | Ty.TyUnit => false | Ty.TyBool => false | Ty.TyNat => false | Ty.TyFixed => false | Ty.TyOrdering => false | Ty.TySelf => false | Ty.TyParam _ => false | Ty.TyRef _ => false | Ty.TyRefMut _ => false | Ty.TyPair _ _ => false | Ty.TyNamed _ _ => false | Ty.TyWide => false | Ty.TySize => false | Ty.TyChar => false | Ty.TyStr => false | Ty.TySlice _ => false | Ty.TyUnknown => false | Ty.TyAbsent => true | Ty.TyNil => false | Ty.TyCons _ _ => false)

@[expose] public def isUnknown (t : Ty) : Bool := (match t with | Ty.TyUnit => false | Ty.TyBool => false | Ty.TyNat => false | Ty.TyFixed => false | Ty.TyOrdering => false | Ty.TySelf => false | Ty.TyParam _ => false | Ty.TyRef _ => false | Ty.TyRefMut _ => false | Ty.TyPair _ _ => false | Ty.TyNamed _ _ => false | Ty.TyWide => false | Ty.TySize => false | Ty.TyChar => false | Ty.TyStr => false | Ty.TySlice _ => false | Ty.TyUnknown => true | Ty.TyAbsent => false | Ty.TyNil => false | Ty.TyCons _ _ => false)

@[expose] public def stripRefs : (t : Ty) -> Ty
  | Ty.TyUnit => Ty.TyUnit
  | Ty.TyBool => Ty.TyBool
  | Ty.TyNat => Ty.TyNat
  | Ty.TyFixed => Ty.TyFixed
  | Ty.TyOrdering => Ty.TyOrdering
  | Ty.TySelf => Ty.TySelf
  | Ty.TyParam d0 => Ty.TyParam (d0)
  | Ty.TyRef inner => stripRefs (inner)
  | Ty.TyRefMut inner => stripRefs (inner)
  | Ty.TyPair d0 d1 => Ty.TyPair (d0) (d1)
  | Ty.TyNamed d0 d1 => Ty.TyNamed (d0) (d1)
  | Ty.TyWide => Ty.TyWide
  | Ty.TySize => Ty.TySize
  | Ty.TyChar => Ty.TyChar
  | Ty.TyStr => Ty.TyStr
  | Ty.TySlice d0 => Ty.TySlice (d0)
  | Ty.TyUnknown => Ty.TyUnknown
  | Ty.TyAbsent => Ty.TyAbsent
  | Ty.TyNil => Ty.TyNil
  | Ty.TyCons d0 d1 => Ty.TyCons (d0) (d1)

@[expose] public def withBound (fr : Fr) (bound : List (Var)) (loop : Bool) : Fr := ({ generics := (fr).generics, vars := (LexLeanRuntime.append (bound) ((fr).vars) : List (Var)), selfTy := (fr).selfTy, inLoop := (if (fr).inLoop then true else loop) } : Fr)

@[expose] public def emptyFr (_unit : Unit) : Fr := ({ generics := ([] : List (Generic)), vars := ([] : List (Var)), selfTy := Ty.TyAbsent, inLoop := false } : Fr)

@[expose] public def isRefMut (t : Ty) : Bool := (match t with | Ty.TyUnit => false | Ty.TyBool => false | Ty.TyNat => false | Ty.TyFixed => false | Ty.TyOrdering => false | Ty.TySelf => false | Ty.TyParam _ => false | Ty.TyRef _ => false | Ty.TyRefMut _ => true | Ty.TyPair _ _ => false | Ty.TyNamed _ _ => false | Ty.TyWide => false | Ty.TySize => false | Ty.TyChar => false | Ty.TyStr => false | Ty.TySlice _ => false | Ty.TyUnknown => false | Ty.TyAbsent => false | Ty.TyNil => false | Ty.TyCons _ _ => false)

@[expose] public def resolveSelf (fr : Fr) (t : Ty) : Ty := (match t with | Ty.TyUnit => t | Ty.TyBool => t | Ty.TyNat => t | Ty.TyFixed => t | Ty.TyOrdering => t | Ty.TySelf => (fr).selfTy | Ty.TyParam _ => t | Ty.TyRef _ => t | Ty.TyRefMut _ => t | Ty.TyPair _ _ => t | Ty.TyNamed _ _ => t | Ty.TyWide => t | Ty.TySize => t | Ty.TyChar => t | Ty.TyStr => t | Ty.TySlice _ => t | Ty.TyUnknown => t | Ty.TyAbsent => t | Ty.TyNil => t | Ty.TyCons _ _ => t)

@[expose] public def pairPart (t : Ty) («at» : Nat) : Ty := (match t with | Ty.TyUnit => Ty.TyUnknown | Ty.TyBool => Ty.TyUnknown | Ty.TyNat => Ty.TyUnknown | Ty.TyFixed => Ty.TyUnknown | Ty.TyOrdering => Ty.TyUnknown | Ty.TySelf => Ty.TyUnknown | Ty.TyParam _ => Ty.TyUnknown | Ty.TyRef _ => Ty.TyUnknown | Ty.TyRefMut _ => Ty.TyUnknown | Ty.TyPair a b => (if (Nat.beq («at») (0)) then a else (if (Nat.beq («at») (1)) then b else Ty.TyUnknown)) | Ty.TyNamed _ _ => Ty.TyUnknown | Ty.TyWide => Ty.TyUnknown | Ty.TySize => Ty.TyUnknown | Ty.TyChar => Ty.TyUnknown | Ty.TyStr => Ty.TyUnknown | Ty.TySlice _ => Ty.TyUnknown | Ty.TyUnknown => Ty.TyUnknown | Ty.TyAbsent => Ty.TyUnknown | Ty.TyNil => Ty.TyUnknown | Ty.TyCons _ _ => Ty.TyUnknown)

@[expose] public def typeOf : (fr : Fr) -> (e : Expr) -> Ty
  | fr, Expr.EVar name => (match findVar ((fr).vars) (name) with | Option.some held => (held).ty | Option.none => Ty.TyUnknown)
  | _fr, Expr.ELit _ => Ty.TyUnknown
  | _fr, Expr.EPath _ => Ty.TyUnknown
  | _fr, Expr.ECall _ _ => Ty.TyUnknown
  | _fr, Expr.EMethod _ _ _ => Ty.TyUnknown
  | fr, Expr.EField inner «at» => pairPart (resolveSelf (fr) (stripRefs (typeOf (fr) (inner)))) («at»)
  | _fr, Expr.EFieldNamed _ _ => Ty.TyUnknown
  | fr, Expr.EBorrow inner => (let t : Ty := typeOf (fr) (inner); (if isUnknown (t) then Ty.TyUnknown else Ty.TyRef (t)))
  | _fr, Expr.EEquals _ _ => Ty.TyUnknown
  | _fr, Expr.EMatch _ _ => Ty.TyUnknown
  | _fr, Expr.ELet _ _ _ => Ty.TyUnknown
  | _fr, Expr.EAssign _ _ => Ty.TyUnknown
  | _fr, Expr.EIf _ _ _ => Ty.TyUnknown
  | _fr, Expr.EWhile _ _ => Ty.TyUnknown
  | _fr, Expr.EWhileLet _ _ _ => Ty.TyUnknown
  | _fr, Expr.ELoop _ => Ty.TyUnknown
  | _fr, Expr.EFor _ _ _ => Ty.TyUnknown
  | _fr, Expr.EBreak => Ty.TyUnknown
  | _fr, Expr.EReturn _ => Ty.TyUnknown
  | _fr, Expr.ETuple _ => Ty.TyUnknown
  | _fr, Expr.EStructLit _ _ _ => Ty.TyUnknown
  | _fr, Expr.EBinary _ _ _ => Ty.TyUnknown
  | _fr, Expr.ENot _ => Ty.TyUnknown
  | _fr, Expr.ECast _ _ => Ty.TyUnknown
  | _fr, Expr.EIndex _ _ => Ty.TyUnknown
  | _fr, Expr.ERange _ _ _ => Ty.TyUnknown
  | _fr, Expr.EDeref _ => Ty.TyUnknown
  | _fr, Expr.EBorrowMut _ => Ty.TyUnknown
  | _fr, Expr.EMethodOf _ _ _ _ => Ty.TyUnknown
  | _fr, Expr.EBlock _ _ => Ty.TyUnknown
  | _fr, Expr.ENone => Ty.TyUnknown
  | _fr, Expr.ENil => Ty.TyUnknown
  | _fr, Expr.ECons _ _ => Ty.TyUnknown
  | _fr, Expr.ELetS _ _ _ _ => Ty.TyUnknown
  | _fr, Expr.EArm _ _ _ _ => Ty.TyUnknown

@[expose] public def assignable : (fr : Fr) -> (place : Expr) -> Option (ByteArray)
  | fr, Expr.EVar name => (match findVar ((fr).vars) (name) with | Option.none => Option.some (rcat (rtext ("runtime support: variable-scope: ")) (rcat (rtick (name)) (rtext (" is not in scope")))) | Option.some held => (if (held).mutable then Option.none else (if isRefMut ((held).ty) then Option.none else Option.some (rcat (rtext ("runtime support: assign-immutable: ")) (rcat (rtick (name)) (rtext (" is assigned and is not `mut` or `&mut`")))))))
  | _fr, Expr.ELit _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | _fr, Expr.EPath _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | _fr, Expr.ECall _ _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | _fr, Expr.EMethod _ _ _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | fr, Expr.EField inner _ => assignable (fr) (inner)
  | fr, Expr.EFieldNamed inner _ => assignable (fr) (inner)
  | _fr, Expr.EBorrow _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | _fr, Expr.EEquals _ _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | _fr, Expr.EMatch _ _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | _fr, Expr.ELet _ _ _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | _fr, Expr.EAssign _ _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | _fr, Expr.EIf _ _ _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | _fr, Expr.EWhile _ _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | _fr, Expr.EWhileLet _ _ _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | _fr, Expr.ELoop _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | _fr, Expr.EFor _ _ _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | _fr, Expr.EBreak => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | _fr, Expr.EReturn _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | _fr, Expr.ETuple _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | _fr, Expr.EStructLit _ _ _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | _fr, Expr.EBinary _ _ _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | _fr, Expr.ENot _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | _fr, Expr.ECast _ _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | fr, Expr.EIndex inner _ => assignable (fr) (inner)
  | _fr, Expr.ERange _ _ _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | fr, Expr.EDeref inner => assignable (fr) (inner)
  | _fr, Expr.EBorrowMut _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | _fr, Expr.EMethodOf _ _ _ _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | _fr, Expr.EBlock _ _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | _fr, Expr.ENone => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | _fr, Expr.ENil => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | _fr, Expr.ECons _ _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | _fr, Expr.ELetS _ _ _ _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))
  | _fr, Expr.EArm _ _ _ _ => Option.some (rcat (rtext ("runtime support: assign-place: ")) (rtext ("an assignment to a value that is no place")))

@[expose] public def callAllocates (sc : Sc) (path : List (ByteArray)) : Bool := (if (sc).core then (match path with | List.nil => false | List.cons owner rest => (match rest with | List.nil => false | List.cons name more => (match more with | List.nil => callHeap (owner) (name) | List.cons _ _ => false))) else false)

@[expose] public def callExpected (sc : Sc) (path : List (ByteArray)) : Option (Nat) := (match path with | List.nil => Option.none | List.cons owner rest => (match rest with | List.nil => (match findFunction (sc) (owner) with | Option.some held => Option.some ((held).n) | Option.none => (match wrapperArity (owner) with | Option.some count => Option.some (count) | Option.none => tupleStructArity (sc) (owner))) | List.cons name more => (match more with | List.nil => (match callArity (owner) (name) with | Option.some count => Option.some (count) | Option.none => variantArity (sc) (owner) (name)) | List.cons _ _ => Option.none)))

@[expose] public def callCheck (sc : Sc) (path : List (ByteArray)) (given : Nat) : Option (ByteArray) := (if callAllocates (sc) (path) then Option.some (rcat (rtext ("runtime support: hidden-allocation: ")) (rcat (rtext ("rust-core renders ")) (rcat (rtick (joinWith (path) ("::"))) (rtext (", which requires heap allocation"))))) else (match callExpected (sc) (path) with | Option.none => Option.some (rcat (rtext ("runtime support: call-name: ")) (rcat (rtick (joinWith (path) ("::"))) (rtext (" is neither declared nor in the closed table")))) | Option.some expected => (if (Nat.beq (expected) (given)) then Option.none else Option.some (rcat (rtext ("runtime support: call-arity: ")) (rcat (rtick (joinWith (path) ("::"))) (rcat (rtext (" takes ")) (rcat (rdec (expected)) (rcat (rtext (" arguments, given ")) (rdec (given))))))))))

@[expose] public def boundCheck (sc : Sc) (fr : Fr) (recv : Expr) (name : ByteArray) : Option (ByteArray) := (match stripRefs (typeOf (fr) (recv)) with | Ty.TyParam «at» => (if provided (sc) (nthBounds ((fr).generics) («at»)) (name) then Option.none else Option.some (rcat (rtext ("runtime support: method-bound: ")) (rcat (rtick (name)) (rtext (" is called on a parameter whose bounds do not provide it"))))) | Ty.TyUnit => Option.none | Ty.TyBool => Option.none | Ty.TyNat => Option.none | Ty.TyFixed => Option.none | Ty.TyOrdering => Option.none | Ty.TySelf => Option.none | Ty.TyRef _ => Option.none | Ty.TyRefMut _ => Option.none | Ty.TyPair _ _ => Option.none | Ty.TyNamed _ _ => Option.none | Ty.TyWide => Option.none | Ty.TySize => Option.none | Ty.TyChar => Option.none | Ty.TyStr => Option.none | Ty.TySlice _ => Option.none | Ty.TyUnknown => Option.none | Ty.TyAbsent => Option.none | Ty.TyNil => Option.none | Ty.TyCons _ _ => Option.none)

@[expose] public def methodOk (sc : Sc) (fr : Fr) (recv : Expr) (name : ByteArray) (given : Nat) : Option (ByteArray) := (match methodArity (name) with | Option.some count => (if (if methodHeap (name) then (sc).core else false) then Option.some (rcat (rtext ("runtime support: hidden-allocation: ")) (rcat (rtext ("rust-core renders ")) (rcat (rtick (name)) (rtext (", which requires heap allocation"))))) else (if (Nat.beq (count) (given)) then Option.none else Option.some (rcat (rtext ("runtime support: method-arity: ")) (rcat (rtick (name)) (rcat (rtext (" takes ")) (rcat (rdec (count)) (rcat (rtext (" arguments, given ")) (rdec (given))))))))) | Option.none => (let counts : List (Nat) := methodCounts (sc) (name); (match counts with | List.nil => Option.some (rcat (rtext ("runtime support: method-name: ")) (rcat (rtick (name)) (rtext (" is a method of no declared trait or impl and not in the closed table")))) | List.cons _ _ => (if hasCount (counts) (given) then boundCheck (sc) (fr) (recv) (name) else Option.some (rcat (rtext ("runtime support: method-arity: ")) (rcat (rtick (name)) (rcat (rtext (" takes ")) (rcat (joinCounts (counts)) (rcat (rtext (" arguments, given ")) (rdec (given)))))))))))

@[expose] public def isNone (e : Expr) : Bool := (match e with | Expr.EVar _ => false | Expr.ELit _ => false | Expr.EPath _ => false | Expr.ECall _ _ => false | Expr.EMethod _ _ _ => false | Expr.EField _ _ => false | Expr.EFieldNamed _ _ => false | Expr.EBorrow _ => false | Expr.EEquals _ _ => false | Expr.EMatch _ _ => false | Expr.ELet _ _ _ => false | Expr.EAssign _ _ => false | Expr.EIf _ _ _ => false | Expr.EWhile _ _ => false | Expr.EWhileLet _ _ _ => false | Expr.ELoop _ => false | Expr.EFor _ _ _ => false | Expr.EBreak => false | Expr.EReturn _ => false | Expr.ETuple _ => false | Expr.EStructLit _ _ _ => false | Expr.EBinary _ _ _ => false | Expr.ENot _ => false | Expr.ECast _ _ => false | Expr.EIndex _ _ => false | Expr.ERange _ _ _ => false | Expr.EDeref _ => false | Expr.EBorrowMut _ => false | Expr.EMethodOf _ _ _ _ => false | Expr.EBlock _ _ => false | Expr.ENone => true | Expr.ENil => false | Expr.ECons _ _ => false | Expr.ELetS _ _ _ _ => false | Expr.EArm _ _ _ _ => false)

@[expose] public def isEmptyChain (e : Expr) : Bool := (match e with | Expr.EVar _ => false | Expr.ELit _ => false | Expr.EPath _ => false | Expr.ECall _ _ => false | Expr.EMethod _ _ _ => false | Expr.EField _ _ => false | Expr.EFieldNamed _ _ => false | Expr.EBorrow _ => false | Expr.EEquals _ _ => false | Expr.EMatch _ _ => false | Expr.ELet _ _ _ => false | Expr.EAssign _ _ => false | Expr.EIf _ _ _ => false | Expr.EWhile _ _ => false | Expr.EWhileLet _ _ _ => false | Expr.ELoop _ => false | Expr.EFor _ _ _ => false | Expr.EBreak => false | Expr.EReturn _ => false | Expr.ETuple _ => false | Expr.EStructLit _ _ _ => false | Expr.EBinary _ _ _ => false | Expr.ENot _ => false | Expr.ECast _ _ => false | Expr.EIndex _ _ => false | Expr.ERange _ _ _ => false | Expr.EDeref _ => false | Expr.EBorrowMut _ => false | Expr.EMethodOf _ _ _ _ => false | Expr.EBlock _ _ => false | Expr.ENone => false | Expr.ENil => true | Expr.ECons _ _ => false | Expr.ELetS _ _ _ _ => false | Expr.EArm _ _ _ _ => false)

@[expose] public def countIn (names : List (ByteArray)) (wanted : ByteArray) : Nat := (LexLeanCollections.listFold ((fun (count : Nat) (name : ByteArray) => (if bEq (name) (wanted) then (count + 1) else count))) (0) (names) : Nat)

@[expose] public def sameFields (names : List (ByteArray)) (held : List (ByteArray)) (values : Nat) : Bool := (if (Nat.beq ((LexLeanRuntime.length (names) : Nat)) ((LexLeanRuntime.length (held) : Nat))) then (if (Nat.beq ((LexLeanRuntime.length (names) : Nat)) (values)) then (LexLeanCollections.listFold ((fun (all : Bool) (name : ByteArray) => (if all then (Nat.beq (countIn (names) (name)) (countIn (held) (name))) else false))) (true) (held) : Bool) else false) else false)

mutual
@[expose] public def exprOk : (sc : Sc) -> (fr : Fr) -> (e : Expr) -> Option (ByteArray)
  | sc, fr, Expr.EVar name => (match findVar ((fr).vars) (name) with | Option.some _ => Option.none | Option.none => (if inList ((sc).statics) (name) then Option.none else Option.some (rcat (rtext ("runtime support: variable-scope: ")) (rcat (rtick (name)) (rtext (" is not in scope"))))))
  | _sc, _fr, Expr.ELit scalars => literalErr (scalars)
  | sc, _fr, Expr.EPath path => pathOk (sc) (path)
  | sc, fr, Expr.ECall path args => (match callCheck (sc) (path) (exprLen (args)) with | Option.some stop => Option.some (stop) | Option.none => exprsOk (sc) (fr) (args))
  | sc, fr, Expr.EMethod recv name args => (match exprOk (sc) (fr) (recv) with | Option.some stop => Option.some (stop) | Option.none => (match methodOk (sc) (fr) (recv) (name) (exprLen (args)) with | Option.some stop => Option.some (stop) | Option.none => exprsOk (sc) (fr) (args)))
  | sc, fr, Expr.EField inner _ => exprOk (sc) (fr) (inner)
  | sc, fr, Expr.EFieldNamed inner name => (match identErr ("a field access") (name) with | Option.some bad => Option.some (bad) | Option.none => exprOk (sc) (fr) (inner))
  | sc, fr, Expr.EBorrow inner => exprOk (sc) (fr) (inner)
  | sc, fr, Expr.EEquals a b => (match exprOk (sc) (fr) (a) with | Option.some stop => Option.some (stop) | Option.none => exprOk (sc) (fr) (b))
  | sc, fr, Expr.EMatch scrutinee arms => (match orElse (headErr (scrutinee)) (exprOk (sc) (fr) (scrutinee)) with | Option.some stop => Option.some (stop) | Option.none => (if isEmptyChain (arms) then Option.some (rcat (rtext ("runtime support: match-arms: ")) (rtext ("a match with no arm"))) else armsOk (sc) (fr) (arms)))
  | _sc, _fr, Expr.ELet _ _ _ => Option.some (rcat (rtext ("runtime support: let-position: ")) (rtext ("a `let` that is not a statement of a block")))
  | sc, fr, Expr.EAssign place value => (match assignable (fr) (place) with | Option.some stop => Option.some (stop) | Option.none => (match exprOk (sc) (fr) (place) with | Option.some stop => Option.some (stop) | Option.none => exprOk (sc) (fr) (value)))
  | sc, fr, Expr.EIf cond thenBlock elseBlock => (match orElse (headErr (cond)) (exprOk (sc) (fr) (cond)) with | Option.some stop => Option.some (stop) | Option.none => (match blockOk (sc) (fr) (thenBlock) with | Option.some stop => Option.some (stop) | Option.none => blockOk (sc) (fr) (elseBlock)))
  | sc, fr, Expr.EWhile cond body => (match orElse (headErr (cond)) (exprOk (sc) (fr) (cond)) with | Option.some stop => Option.some (stop) | Option.none => blockOk (sc) (withBound (fr) (([] : List (Var))) (true)) (body))
  | sc, fr, Expr.EWhileLet pat value body => (match orElse (headErr (value)) (exprOk (sc) (fr) (value)) with | Option.some stop => Option.some (stop) | Option.none => (match patOk (sc) (pat) with | Option.some stop => Option.some (stop) | Option.none => (match bindsOf (([] : List (Var))) (pat) with | Except.error problem => Option.some (problem) | Except.ok bound => blockOk (sc) (withBound (fr) (bound) (true)) (body))))
  | sc, fr, Expr.ELoop body => blockOk (sc) (withBound (fr) (([] : List (Var))) (true)) (body)
  | sc, fr, Expr.EFor pat value body => (match orElse (headErr (value)) (exprOk (sc) (fr) (value)) with | Option.some stop => Option.some (stop) | Option.none => (match patOk (sc) (pat) with | Option.some stop => Option.some (stop) | Option.none => (match bindsOf (([] : List (Var))) (pat) with | Except.error problem => Option.some (problem) | Except.ok bound => blockOk (sc) (withBound (fr) (bound) (true)) (body))))
  | _sc, fr, Expr.EBreak => (if (fr).inLoop then Option.none else Option.some (rcat (rtext ("runtime support: break-position: ")) (rtext ("a `break` outside a loop"))))
  | sc, fr, Expr.EReturn value => (if isNone (value) then Option.none else exprOk (sc) (fr) (value))
  | sc, fr, Expr.ETuple items => (if (Nat.blt (exprLen (items)) (2)) then Option.some (rcat (rtext ("runtime support: tuple-arity: ")) (rcat (rtext ("a tuple of ")) (rdec (exprLen (items))))) else exprsOk (sc) (fr) (items))
  | sc, fr, Expr.EStructLit path names values => (match pathOk (sc) (path) with | Option.some stop => Option.some (stop) | Option.none => (match recordFields (sc) (path) with | Option.none => Option.some (rcat (rtext ("runtime support: struct-fields: ")) (rcat (rtick (joinWith (path) ("::"))) (rtext (" is no record struct or record variant")))) | Option.some held => (if sameFields (names) (held) (exprLen (values)) then exprsOk (sc) (fr) (values) else Option.some (rcat (rtext ("runtime support: struct-fields: ")) (rcat (rtick (joinWith (path) ("::"))) (rcat (rtext (" is given ")) (rcat (joinWith (names) (", ")) (rcat (rtext (", not ")) (joinWith (held) (", "))))))))))
  | sc, fr, Expr.EBinary _ a b => (match exprOk (sc) (fr) (a) with | Option.some stop => Option.some (stop) | Option.none => exprOk (sc) (fr) (b))
  | sc, fr, Expr.ENot inner => exprOk (sc) (fr) (inner)
  | sc, fr, Expr.ECast inner ty => (match orElse (tyOk (sc) (ty) ((LexLeanRuntime.length ((fr).generics) : Nat)) ((!isAbsent ((fr).selfTy)))) (unscopedErr ("a cast type") (ty)) with | Option.some stop => Option.some (stop) | Option.none => exprOk (sc) (fr) (inner))
  | sc, fr, Expr.EIndex a b => (match exprOk (sc) (fr) (a) with | Option.some stop => Option.some (stop) | Option.none => exprOk (sc) (fr) (b))
  | sc, fr, Expr.ERange inner lower upper => (match exprOk (sc) (fr) (inner) with | Option.some stop => Option.some (stop) | Option.none => (match (if isNone (lower) then Option.none else exprOk (sc) (fr) (lower)) with | Option.some stop => Option.some (stop) | Option.none => (if isNone (upper) then Option.none else exprOk (sc) (fr) (upper))))
  | sc, fr, Expr.EDeref inner => exprOk (sc) (fr) (inner)
  | sc, fr, Expr.EBorrowMut inner => exprOk (sc) (fr) (inner)
  | sc, fr, Expr.EMethodOf recv name tys args => (match exprOk (sc) (fr) (recv) with | Option.some stop => Option.some (stop) | Option.none => (match turbofishTypes (name) with | Option.none => Option.some (rcat (rtext ("runtime support: method-name: ")) (rcat (rtick (name)) (rtext (" takes no type arguments in the closed table")))) | Option.some count => (if (if (Nat.beq (count) (tyLen (tys))) then (Nat.beq (turbofishArgs (name)) (exprLen (args))) else false) then (if (if (sc).core then bEq (name) (rtext ("collect")) else false) then Option.some (rcat (rtext ("runtime support: hidden-allocation: ")) (rcat (rtext ("rust-core renders ")) (rcat (rtick (name)) (rtext (", which requires heap allocation"))))) else (match orElse (tyOk (sc) (tys) ((LexLeanRuntime.length ((fr).generics) : Nat)) ((!isAbsent ((fr).selfTy)))) (unscopedErr ("a type argument") (tys)) with | Option.some stop => Option.some (stop) | Option.none => exprsOk (sc) (fr) (args))) else Option.some (rcat (rtext ("runtime support: method-arity: ")) (rcat (rtick (name)) (rcat (rtext (" takes ")) (rcat (rdec (count)) (rcat (rtext (" type arguments and ")) (rcat (rdec (turbofishArgs (name))) (rtext (" arguments")))))))))))
  | sc, fr, Expr.EBlock stmts tail => (match stmtsOk (sc) (fr) (stmts) with | Except.error problem => Option.some (problem) | Except.ok inner => exprOk (sc) (inner) (tail))
  | _sc, _fr, Expr.ENone => Option.none
  | _sc, _fr, Expr.ENil => Option.none
  | _sc, _fr, Expr.ECons _ _ => Option.none
  | _sc, _fr, Expr.ELetS _ _ _ _ => Option.none
  | _sc, _fr, Expr.EArm _ _ _ _ => Option.none
termination_by structural _ _ e => e

@[expose] public def exprsOk : (sc : Sc) -> (fr : Fr) -> (es : Expr) -> Option (ByteArray)
  | _sc, _fr, Expr.EVar _ => Option.none
  | _sc, _fr, Expr.ELit _ => Option.none
  | _sc, _fr, Expr.EPath _ => Option.none
  | _sc, _fr, Expr.ECall _ _ => Option.none
  | _sc, _fr, Expr.EMethod _ _ _ => Option.none
  | _sc, _fr, Expr.EField _ _ => Option.none
  | _sc, _fr, Expr.EFieldNamed _ _ => Option.none
  | _sc, _fr, Expr.EBorrow _ => Option.none
  | _sc, _fr, Expr.EEquals _ _ => Option.none
  | _sc, _fr, Expr.EMatch _ _ => Option.none
  | _sc, _fr, Expr.ELet _ _ _ => Option.none
  | _sc, _fr, Expr.EAssign _ _ => Option.none
  | _sc, _fr, Expr.EIf _ _ _ => Option.none
  | _sc, _fr, Expr.EWhile _ _ => Option.none
  | _sc, _fr, Expr.EWhileLet _ _ _ => Option.none
  | _sc, _fr, Expr.ELoop _ => Option.none
  | _sc, _fr, Expr.EFor _ _ _ => Option.none
  | _sc, _fr, Expr.EBreak => Option.none
  | _sc, _fr, Expr.EReturn _ => Option.none
  | _sc, _fr, Expr.ETuple _ => Option.none
  | _sc, _fr, Expr.EStructLit _ _ _ => Option.none
  | _sc, _fr, Expr.EBinary _ _ _ => Option.none
  | _sc, _fr, Expr.ENot _ => Option.none
  | _sc, _fr, Expr.ECast _ _ => Option.none
  | _sc, _fr, Expr.EIndex _ _ => Option.none
  | _sc, _fr, Expr.ERange _ _ _ => Option.none
  | _sc, _fr, Expr.EDeref _ => Option.none
  | _sc, _fr, Expr.EBorrowMut _ => Option.none
  | _sc, _fr, Expr.EMethodOf _ _ _ _ => Option.none
  | _sc, _fr, Expr.EBlock _ _ => Option.none
  | _sc, _fr, Expr.ENone => Option.none
  | _sc, _fr, Expr.ENil => Option.none
  | sc, fr, Expr.ECons head tail => (match exprOk (sc) (fr) (head) with | Option.some stop => Option.some (stop) | Option.none => exprsOk (sc) (fr) (tail))
  | _sc, _fr, Expr.ELetS _ _ _ _ => Option.none
  | _sc, _fr, Expr.EArm _ _ _ _ => Option.none
termination_by structural _ _ es => es

@[expose] public def blockOk : (sc : Sc) -> (fr : Fr) -> (b : Expr) -> Option (ByteArray)
  | _sc, _fr, Expr.EVar _ => Option.none
  | _sc, _fr, Expr.ELit _ => Option.none
  | _sc, _fr, Expr.EPath _ => Option.none
  | _sc, _fr, Expr.ECall _ _ => Option.none
  | _sc, _fr, Expr.EMethod _ _ _ => Option.none
  | _sc, _fr, Expr.EField _ _ => Option.none
  | _sc, _fr, Expr.EFieldNamed _ _ => Option.none
  | _sc, _fr, Expr.EBorrow _ => Option.none
  | _sc, _fr, Expr.EEquals _ _ => Option.none
  | _sc, _fr, Expr.EMatch _ _ => Option.none
  | _sc, _fr, Expr.ELet _ _ _ => Option.none
  | _sc, _fr, Expr.EAssign _ _ => Option.none
  | _sc, _fr, Expr.EIf _ _ _ => Option.none
  | _sc, _fr, Expr.EWhile _ _ => Option.none
  | _sc, _fr, Expr.EWhileLet _ _ _ => Option.none
  | _sc, _fr, Expr.ELoop _ => Option.none
  | _sc, _fr, Expr.EFor _ _ _ => Option.none
  | _sc, _fr, Expr.EBreak => Option.none
  | _sc, _fr, Expr.EReturn _ => Option.none
  | _sc, _fr, Expr.ETuple _ => Option.none
  | _sc, _fr, Expr.EStructLit _ _ _ => Option.none
  | _sc, _fr, Expr.EBinary _ _ _ => Option.none
  | _sc, _fr, Expr.ENot _ => Option.none
  | _sc, _fr, Expr.ECast _ _ => Option.none
  | _sc, _fr, Expr.EIndex _ _ => Option.none
  | _sc, _fr, Expr.ERange _ _ _ => Option.none
  | _sc, _fr, Expr.EDeref _ => Option.none
  | _sc, _fr, Expr.EBorrowMut _ => Option.none
  | _sc, _fr, Expr.EMethodOf _ _ _ _ => Option.none
  | sc, fr, Expr.EBlock stmts tail => (match stmtsOk (sc) (fr) (stmts) with | Except.error problem => Option.some (problem) | Except.ok inner => exprOk (sc) (inner) (tail))
  | _sc, _fr, Expr.ENone => Option.none
  | _sc, _fr, Expr.ENil => Option.none
  | _sc, _fr, Expr.ECons _ _ => Option.none
  | _sc, _fr, Expr.ELetS _ _ _ _ => Option.none
  | _sc, _fr, Expr.EArm _ _ _ _ => Option.none
termination_by structural _ _ b => b

@[expose] public def stmtsOk : (sc : Sc) -> (fr : Fr) -> (stmts : Expr) -> Except (ByteArray) (Fr)
  | _sc, fr, Expr.EVar _ => Except.ok (fr)
  | _sc, fr, Expr.ELit _ => Except.ok (fr)
  | _sc, fr, Expr.EPath _ => Except.ok (fr)
  | _sc, fr, Expr.ECall _ _ => Except.ok (fr)
  | _sc, fr, Expr.EMethod _ _ _ => Except.ok (fr)
  | _sc, fr, Expr.EField _ _ => Except.ok (fr)
  | _sc, fr, Expr.EFieldNamed _ _ => Except.ok (fr)
  | _sc, fr, Expr.EBorrow _ => Except.ok (fr)
  | _sc, fr, Expr.EEquals _ _ => Except.ok (fr)
  | _sc, fr, Expr.EMatch _ _ => Except.ok (fr)
  | _sc, fr, Expr.ELet _ _ _ => Except.ok (fr)
  | _sc, fr, Expr.EAssign _ _ => Except.ok (fr)
  | _sc, fr, Expr.EIf _ _ _ => Except.ok (fr)
  | _sc, fr, Expr.EWhile _ _ => Except.ok (fr)
  | _sc, fr, Expr.EWhileLet _ _ _ => Except.ok (fr)
  | _sc, fr, Expr.ELoop _ => Except.ok (fr)
  | _sc, fr, Expr.EFor _ _ _ => Except.ok (fr)
  | _sc, fr, Expr.EBreak => Except.ok (fr)
  | _sc, fr, Expr.EReturn _ => Except.ok (fr)
  | _sc, fr, Expr.ETuple _ => Except.ok (fr)
  | _sc, fr, Expr.EStructLit _ _ _ => Except.ok (fr)
  | _sc, fr, Expr.EBinary _ _ _ => Except.ok (fr)
  | _sc, fr, Expr.ENot _ => Except.ok (fr)
  | _sc, fr, Expr.ECast _ _ => Except.ok (fr)
  | _sc, fr, Expr.EIndex _ _ => Except.ok (fr)
  | _sc, fr, Expr.ERange _ _ _ => Except.ok (fr)
  | _sc, fr, Expr.EDeref _ => Except.ok (fr)
  | _sc, fr, Expr.EBorrowMut _ => Except.ok (fr)
  | _sc, fr, Expr.EMethodOf _ _ _ _ => Except.ok (fr)
  | _sc, fr, Expr.EBlock _ _ => Except.ok (fr)
  | _sc, fr, Expr.ENone => Except.ok (fr)
  | _sc, fr, Expr.ENil => Except.ok (fr)
  | sc, fr, Expr.ECons stmt rest => (match exprOk (sc) (fr) (stmt) with | Option.some problem => Except.error (problem) | Option.none => stmtsOk (sc) (fr) (rest))
  | sc, fr, Expr.ELetS pat ty value rest => (match (if isAbsent (ty) then Option.none else orElse (tyOk (sc) (ty) ((LexLeanRuntime.length ((fr).generics) : Nat)) ((!isAbsent ((fr).selfTy)))) (unscopedErr ("a `let` type") (ty))) with | Option.some problem => Except.error (problem) | Option.none => (match exprOk (sc) (fr) (value) with | Option.some problem2 => Except.error (problem2) | Option.none => (match bindsOf (([] : List (Var))) (pat) with | Except.error problem3 => Except.error (problem3) | Except.ok bound => (match patOk (sc) (pat) with | Option.some problem4 => Except.error (problem4) | Option.none => stmtsOk (sc) (withBound (fr) (bound) (false)) (rest)))))
  | _sc, fr, Expr.EArm _ _ _ _ => Except.ok (fr)
termination_by structural _ _ stmts => stmts

@[expose] public def armsOk : (sc : Sc) -> (fr : Fr) -> (arms : Expr) -> Option (ByteArray)
  | _sc, _fr, Expr.EVar _ => Option.none
  | _sc, _fr, Expr.ELit _ => Option.none
  | _sc, _fr, Expr.EPath _ => Option.none
  | _sc, _fr, Expr.ECall _ _ => Option.none
  | _sc, _fr, Expr.EMethod _ _ _ => Option.none
  | _sc, _fr, Expr.EField _ _ => Option.none
  | _sc, _fr, Expr.EFieldNamed _ _ => Option.none
  | _sc, _fr, Expr.EBorrow _ => Option.none
  | _sc, _fr, Expr.EEquals _ _ => Option.none
  | _sc, _fr, Expr.EMatch _ _ => Option.none
  | _sc, _fr, Expr.ELet _ _ _ => Option.none
  | _sc, _fr, Expr.EAssign _ _ => Option.none
  | _sc, _fr, Expr.EIf _ _ _ => Option.none
  | _sc, _fr, Expr.EWhile _ _ => Option.none
  | _sc, _fr, Expr.EWhileLet _ _ _ => Option.none
  | _sc, _fr, Expr.ELoop _ => Option.none
  | _sc, _fr, Expr.EFor _ _ _ => Option.none
  | _sc, _fr, Expr.EBreak => Option.none
  | _sc, _fr, Expr.EReturn _ => Option.none
  | _sc, _fr, Expr.ETuple _ => Option.none
  | _sc, _fr, Expr.EStructLit _ _ _ => Option.none
  | _sc, _fr, Expr.EBinary _ _ _ => Option.none
  | _sc, _fr, Expr.ENot _ => Option.none
  | _sc, _fr, Expr.ECast _ _ => Option.none
  | _sc, _fr, Expr.EIndex _ _ => Option.none
  | _sc, _fr, Expr.ERange _ _ _ => Option.none
  | _sc, _fr, Expr.EDeref _ => Option.none
  | _sc, _fr, Expr.EBorrowMut _ => Option.none
  | _sc, _fr, Expr.EMethodOf _ _ _ _ => Option.none
  | _sc, _fr, Expr.EBlock _ _ => Option.none
  | _sc, _fr, Expr.ENone => Option.none
  | _sc, _fr, Expr.ENil => Option.none
  | _sc, _fr, Expr.ECons _ _ => Option.none
  | _sc, _fr, Expr.ELetS _ _ _ _ => Option.none
  | sc, fr, Expr.EArm pat guard body rest => (match patOk (sc) (pat) with | Option.some stop => Option.some (stop) | Option.none => (match bindsOf (([] : List (Var))) (pat) with | Except.error problem => Option.some (problem) | Except.ok bound => (let inner : Fr := withBound (fr) (bound) (false); (match (if isNone (guard) then Option.none else exprOk (sc) (inner) (guard)) with | Option.some stop => Option.some (stop) | Option.none => (match exprOk (sc) (inner) (body) with | Option.some stop => Option.some (stop) | Option.none => armsOk (sc) (fr) (rest))))))
termination_by structural _ _ arms => arms
end

@[expose] public def paramVars (acc : Except (ByteArray) (List (Var))) (ps : List (Param)) (fname : ByteArray) : Except (ByteArray) (List (Var)) := (LexLeanCollections.listFold ((fun (state : Except (ByteArray) (List (Var))) (param : Param) => (match state with | Except.error problem => Except.error (problem) | Except.ok vars => (match findVar (vars) ((param).name) with | Option.some _ => Except.error (rcat (rtext ("runtime support: param-duplicate: ")) (rcat (rtick ((param).name)) (rcat (rtext (" names two parameters of ")) (rtick (fname))))) | Option.none => Except.ok ((({ name := (param).name, mutable := (param).mutable, ty := (param).ty } : Var) :: vars)))))) (acc) (ps) : Except (ByteArray) (List (Var)))

@[expose] public def fnOk (sc : Sc) (outer : List (Generic)) (selfTy : Ty) (f : FnDef) : Option (ByteArray) := (match genericsOk (sc) ((f).generics) with | Option.some stop => Option.some (stop) | Option.none => (let gens : List (Generic) := (LexLeanRuntime.append (outer) ((f).generics) : List (Generic)); (let count : Nat := (LexLeanRuntime.length (gens) : Nat); (let sg : Sig := (f).sig; (match sigOk (sc) (sg) (count) ((!isAbsent (selfTy))) with | Option.some stop => Option.some (stop) | Option.none => (match (if (sg).receiver then (if isAbsent (selfTy) then Except.error (rcat (rtext ("runtime support: function-receiver: ")) (rcat (rtick ((sg).name)) (rtext (" takes `&self` outside an impl")))) else Except.ok ((({ name := rtext ("self"), mutable := (sg).exclusive, ty := (if (sg).exclusive then Ty.TyRefMut (selfTy) else Ty.TyRef (selfTy)) } : Var) :: ([] : List (Var))))) else Except.ok (([] : List (Var)))) with | Except.error problem => Option.some (problem) | Except.ok start => (match paramVars (Except.ok (start)) ((sg).params) ((sg).name) with | Except.error problem2 => Option.some (problem2) | Except.ok vars => blockOk (sc) (({ generics := gens, vars := vars, selfTy := selfTy, inLoop := false } : Fr)) ((f).body))))))))

@[expose] public def sigsOfTrait (sc : Sc) (tr : ByteArray) : Option (List (Sig)) := (match findTrait (sc) (tr) with | Option.some held => Option.some ((held).sigs) | Option.none => (match stdTraitMethod (tr) with | Option.some method => Option.some ((({ name := method, receiver := true, exclusive := stdTraitExclusive (tr), params := ([] : List (Param)), result := (if stdTraitExclusive (tr) then Ty.TyUnit else Ty.TySelf) } : Sig) :: ([] : List (Sig)))) | Option.none => Option.none))

@[expose] public def countNamed (fns : List (FnDef)) (wanted : ByteArray) : Nat := (LexLeanCollections.listFold ((fun (count : Nat) (f : FnDef) => (if bEq (((f).sig).name) (wanted) then (count + 1) else count))) (0) (fns) : Nat)

@[expose] public def firstNamed (fns : List (FnDef)) (wanted : ByteArray) : Option (FnDef) := (LexLeanCollections.listFold ((fun (found : Option (FnDef)) (f : FnDef) => (match found with | Option.some held => Option.some (held) | Option.none => (if bEq (((f).sig).name) (wanted) then Option.some (f) else Option.none)))) (Option.none) (fns) : Option (FnDef))

@[expose] public def hasSigNamed (sigs : List (Sig)) (wanted : ByteArray) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (sig : Sig) => (if found then true else bEq ((sig).name) (wanted)))) (false) (sigs) : Bool)

@[expose] public def completes (sc : Sc) (tr : ByteArray) (generics : List (Generic)) (ty : Ty) (fns : List (FnDef)) : Option (ByteArray) := (match genericsOk (sc) (generics) with | Option.some stop => Option.some (stop) | Option.none => (match sigsOfTrait (sc) (tr) with | Option.none => Option.some (rcat (rtext ("runtime support: impl-trait: ")) (rcat (rtext ("`impl ")) (rcat (tr) (rtext ("` names no declared trait"))))) | Option.some sigs => (match (LexLeanCollections.listFold ((fun (found : Option (ByteArray)) (sig : Sig) => (match found with | Option.some stop => Option.some (stop) | Option.none => (let count : Nat := countNamed (fns) ((sig).name); (if (Nat.beq (count) (0)) then Option.some (rcat (rtext ("runtime support: impl-complete: ")) (rcat (rtext ("`impl ")) (rcat (tr) (rcat (rtext ("` does not state ")) (rtick ((sig).name)))))) else (if (Nat.beq (count) (1)) then (match firstNamed (fns) ((sig).name) with | Option.some held => (if (LexLeanRuntime.equal (sigShow ((held).sig)) (sigShow (sig)) : Bool) then Option.none else Option.some (rcat (rtext ("runtime support: impl-signature: ")) (rcat (rtext ("`impl ")) (rcat (tr) (rcat (rtext ("` states ")) (rcat (rtick ((sig).name)) (rtext (" with a signature other than the trait's")))))))) | Option.none => Option.none) else Option.some (rcat (rtext ("runtime support: impl-complete: ")) (rcat (rtext ("`impl ")) (rcat (tr) (rcat (rtext ("` states ")) (rcat (rtick ((sig).name)) (rtext (" twice"))))))))))))) (Option.none) (sigs) : Option (ByteArray)) with | Option.some stop => Option.some (stop) | Option.none => (LexLeanCollections.listFold ((fun (foundFn : Option (ByteArray)) (f : FnDef) => (match foundFn with | Option.some stop => Option.some (stop) | Option.none => (if (!hasSigNamed (sigs) (((f).sig).name)) then Option.some (rcat (rtext ("runtime support: impl-extra: ")) (rcat (rtext ("`impl ")) (rcat (tr) (rcat (rtext ("` states ")) (rcat (rtick (((f).sig).name)) (rtext (", which the trait does not"))))))) else (if (f).«public» then Option.some (rcat (rtext ("runtime support: impl-visibility: ")) (rcat (rtick (((f).sig).name)) (rcat (rtext (" of `impl ")) (rcat (tr) (rtext ("` is public; a trait method has the trait's visibility")))))) else fnOk (sc) (generics) (ty) (f)))))) (Option.none) (fns) : Option (ByteArray)))))

@[expose] public def declare (sc : Sc) (name : ByteArray) : Except (ByteArray) (Sc) := (if inList ((sc).declared) (name) then Except.error (rcat (rtext ("runtime support: duplicate-name: ")) (rcat (rtick (name)) (rtext (" is declared twice")))) else Except.ok (({ core := (sc).core, declared := (LexLeanRuntime.append ((sc).declared) ((name :: ([] : List (ByteArray)))) : List (ByteArray)), traits := (sc).traits, structs := (sc).structs, enums := (sc).enums, aliases := (sc).aliases, statics := (sc).statics, functions := (sc).functions, methods := (sc).methods, imported := (sc).imported } : Sc)))

@[expose] public def addMethods (sc : Sc) (sigs : List (Sig)) : Sc := (LexLeanCollections.listFold ((fun (scope : Sc) (sig : Sig) => ({ core := (scope).core, declared := (scope).declared, traits := (scope).traits, structs := (scope).structs, enums := (scope).enums, aliases := (scope).aliases, statics := (scope).statics, functions := (scope).functions, methods := (LexLeanRuntime.append ((scope).methods) ((({ name := (sig).name, n := (LexLeanRuntime.length ((sig).params) : Nat) } : Entry) :: ([] : List (Entry)))) : List (Entry)), imported := (scope).imported } : Sc))) (sc) (sigs) : Sc)

@[expose] public def addFnMethods (sc : Sc) (fns : List (FnDef)) : Sc := (LexLeanCollections.listFold ((fun (scope : Sc) (f : FnDef) => ({ core := (scope).core, declared := (scope).declared, traits := (scope).traits, structs := (scope).structs, enums := (scope).enums, aliases := (scope).aliases, statics := (scope).statics, functions := (scope).functions, methods := (LexLeanRuntime.append ((scope).methods) ((({ name := ((f).sig).name, n := (LexLeanRuntime.length (((f).sig).params) : Nat) } : Entry) :: ([] : List (Entry)))) : List (Entry)), imported := (scope).imported } : Sc))) (sc) (fns) : Sc)

public structure Seen where
  names : List (ByteArray)
  dup : Option (ByteArray)

@[expose] public def firstDuplicate (variants : List (Variant)) : Option (ByteArray) := ((LexLeanCollections.listFold ((fun (seen : Seen) (variant : Variant) => (match (seen).dup with | Option.some _ => seen | Option.none => (if inList ((seen).names) (variantName (variant)) then ({ names := (seen).names, dup := Option.some (variantName (variant)) } : Seen) else ({ names := (variantName (variant) :: (seen).names), dup := Option.none } : Seen))))) (({ names := ([] : List (ByteArray)), dup := Option.none } : Seen)) (variants) : Seen)).dup

@[expose] public def firstDuplicateSig (sigs : List (Sig)) : Option (ByteArray) := ((LexLeanCollections.listFold ((fun (seen : Seen) (sig : Sig) => (match (seen).dup with | Option.some _ => seen | Option.none => (if inList ((seen).names) ((sig).name) then ({ names := (seen).names, dup := Option.some ((sig).name) } : Seen) else ({ names := ((sig).name :: (seen).names), dup := Option.none } : Seen))))) (({ names := ([] : List (ByteArray)), dup := Option.none } : Seen)) (sigs) : Seen)).dup

@[expose] public def preCheck (check : Option (ByteArray)) (after : Except (ByteArray) (Sc)) : Except (ByteArray) (Sc) := (match check with | Option.some problem => Except.error (problem) | Option.none => after)

@[expose] public def postCheck (done : Except (ByteArray) (Sc)) (check : Option (ByteArray)) : Except (ByteArray) (Sc) := (match done with | Except.error problem => Except.error (problem) | Except.ok _ => (match check with | Option.some stop => Except.error (stop) | Option.none => done))

@[expose] public def variantProblem (enumName : ByteArray) (variants : List (Variant)) : Option (ByteArray) := ((LexLeanCollections.listFold ((fun (seen : Seen) (variant : Variant) => (match (seen).dup with | Option.some _ => seen | Option.none => (match identErr ("a variant name") (variantName (variant)) with | Option.some bad => ({ names := (seen).names, dup := Option.some (bad) } : Seen) | Option.none => (match (match variant with | Variant.VUnit _ => Option.none | Variant.VTuple _ _ => Option.none | Variant.VRecord _ fs _ => identsErr ("a record variant field name") (fs)) with | Option.some bad2 => ({ names := (seen).names, dup := Option.some (bad2) } : Seen) | Option.none => (if inList ((seen).names) (variantName (variant)) then ({ names := (seen).names, dup := Option.some (rcat (rtext ("runtime support: variant-name: ")) (rcat (rtick (enumName)) (rcat (rtext (" declares ")) (rcat (rtick (variantName (variant))) (rtext (" twice")))))) } : Seen) else ({ names := (variantName (variant) :: (seen).names), dup := Option.none } : Seen))))))) (({ names := ([] : List (ByteArray)), dup := Option.none } : Seen)) (variants) : Seen)).dup

@[expose] public def pass1Item (sc : Sc) (item : Item) : Except (ByteArray) (Sc) := (match item with | Item.IUse path alias => postCheck ((match path with | List.nil => Except.error (rtext ("runtime support: use-path: empty")) | List.cons root _ => (if (if bEq (root) (rtext ("core")) then true else (if (sc).core then false else bEq (root) (rtext ("std")))) then Except.ok (({ core := (sc).core, declared := (sc).declared, traits := (sc).traits, structs := (sc).structs, enums := (sc).enums, aliases := (sc).aliases, statics := (sc).statics, functions := (sc).functions, methods := (sc).methods, imported := (LexLeanRuntime.append ((sc).imported) (((match alias with | List.nil => lastName (path) | List.cons renamed _ => renamed) :: ([] : List (ByteArray)))) : List (ByteArray)) } : Sc)) else Except.error (rcat (rtext ("runtime support: profile-root: ")) (rcat (rtick (rcat (rtext ("use ")) (joinWith (path) ("::")))) (rcat (rtext (" starts at ")) (rcat (rtick (root)) (rtext (", which the profile does not carry"))))))))) (orElse (segmentsErr ("a `use` segment") (path)) (identsErr ("a `use` alias") (alias))) | Item.IUnit name derived => preCheck (identErr ("a unit struct name") (name)) (postCheck ((match declare (sc) (name) with | Except.error problem => Except.error (problem) | Except.ok s2 => Except.ok (({ core := (s2).core, declared := (s2).declared, traits := (s2).traits, structs := (LexLeanRuntime.append ((s2).structs) ((({ name := name, fields := ([] : List (ByteArray)), gcount := 0 } : StructE) :: ([] : List (StructE)))) : List (StructE)), enums := (s2).enums, aliases := (s2).aliases, statics := (s2).statics, functions := (s2).functions, methods := (s2).methods, imported := (s2).imported } : Sc)))) (derivesErr ("a unit struct") (derived))) | Item.IAlias name params _ => preCheck (identErr ("an alias name") (name)) (postCheck ((match declare (sc) (name) with | Except.error problem => Except.error (problem) | Except.ok s2 => Except.ok (({ core := (s2).core, declared := (s2).declared, traits := (s2).traits, structs := (s2).structs, enums := (s2).enums, aliases := (LexLeanRuntime.append ((s2).aliases) ((({ name := name, n := (LexLeanRuntime.length (params) : Nat) } : Entry) :: ([] : List (Entry)))) : List (Entry)), statics := (s2).statics, functions := (s2).functions, methods := (s2).methods, imported := (s2).imported } : Sc)))) (identsErr ("an alias parameter") (params))) | Item.IStatic name _ _ => preCheck (identErr ("a static name") (name)) ((match declare (sc) (name) with | Except.error problem => Except.error (problem) | Except.ok s2 => Except.ok (({ core := (s2).core, declared := (s2).declared, traits := (s2).traits, structs := (s2).structs, enums := (s2).enums, aliases := (s2).aliases, statics := (LexLeanRuntime.append ((s2).statics) ((name :: ([] : List (ByteArray)))) : List (ByteArray)), functions := (s2).functions, methods := (s2).methods, imported := (s2).imported } : Sc)))) | Item.ITrait name sigs => preCheck (identErr ("a trait name") (name)) ((match declare (sc) (name) with | Except.error problem => Except.error (problem) | Except.ok s2 => Except.ok (addMethods (({ core := (s2).core, declared := (s2).declared, traits := (LexLeanRuntime.append ((s2).traits) ((({ name := name, sigs := sigs } : TraitE) :: ([] : List (TraitE)))) : List (TraitE)), structs := (s2).structs, enums := (s2).enums, aliases := (s2).aliases, statics := (s2).statics, functions := (s2).functions, methods := (s2).methods, imported := (s2).imported } : Sc)) (sigs)))) | Item.IImpl _ _ _ fns => Except.ok (addFnMethods (sc) (fns)) | Item.IFn f => preCheck (identErr ("a function name") (((f).sig).name)) ((match declare (sc) (((f).sig).name) with | Except.error problem => Except.error (problem) | Except.ok s2 => Except.ok (({ core := (s2).core, declared := (s2).declared, traits := (s2).traits, structs := (s2).structs, enums := (s2).enums, aliases := (s2).aliases, statics := (s2).statics, functions := (LexLeanRuntime.append ((s2).functions) ((({ name := ((f).sig).name, n := (LexLeanRuntime.length (((f).sig).params) : Nat) } : Entry) :: ([] : List (Entry)))) : List (Entry)), methods := (s2).methods, imported := (s2).imported } : Sc)))) | Item.ITuple name generics derived tys => preCheck (identErr ("a tuple struct name") (name)) (postCheck ((match declare (sc) (name) with | Except.error problem => Except.error (problem) | Except.ok s2 => Except.ok (({ core := (s2).core, declared := (s2).declared, traits := (s2).traits, structs := (LexLeanRuntime.append ((s2).structs) ((({ name := name, fields := indexNames (tys) (0), gcount := (LexLeanRuntime.length (generics) : Nat) } : StructE) :: ([] : List (StructE)))) : List (StructE)), enums := (s2).enums, aliases := (s2).aliases, statics := (s2).statics, functions := (s2).functions, methods := (s2).methods, imported := (s2).imported } : Sc)))) (orElse (identsErr ("a tuple struct generic") (generics)) (derivesErr ("a tuple struct") (derived)))) | Item.IRecord name generics derived fields tys => preCheck (identErr ("a record struct name") (name)) ((match declare (sc) (name) with | Except.error problem => Except.error (problem) | Except.ok s2 => (match orElse (identsErr ("a record struct generic") (generics)) (orElse (derivesErr ("a record struct") (derived)) (identsErr ("a record field name") (fields))) with | Option.some stop => Except.error (stop) | Option.none => (if (Nat.beq ((LexLeanRuntime.length (fields) : Nat)) (tyLen (tys))) then Except.ok (({ core := (s2).core, declared := (s2).declared, traits := (s2).traits, structs := (LexLeanRuntime.append ((s2).structs) ((({ name := name, fields := fields, gcount := (LexLeanRuntime.length (generics) : Nat) } : StructE) :: ([] : List (StructE)))) : List (StructE)), enums := (s2).enums, aliases := (s2).aliases, statics := (s2).statics, functions := (s2).functions, methods := (s2).methods, imported := (s2).imported } : Sc)) else Except.error (rcat (rtext ("runtime support: struct-fields: ")) (rcat (rtick (name)) (rcat (rtext (" has ")) (rcat (rdec ((LexLeanRuntime.length (fields) : Nat))) (rcat (rtext (" field names and ")) (rcat (rdec (tyLen (tys))) (rtext (" types")))))))))))) | Item.IEnum name generics derived variants => preCheck (identErr ("an enum name") (name)) ((match declare (sc) (name) with | Except.error problem => Except.error (problem) | Except.ok s2 => (match orElse (identsErr ("an enum generic") (generics)) (orElse (derivesErr ("an enum") (derived)) (variantProblem (name) (variants))) with | Option.some stop => Except.error (stop) | Option.none => (match firstDuplicate (variants) with | Option.some twice => Except.error (rcat (rtext ("runtime support: variant-name: ")) (rcat (rtick (name)) (rcat (rtext (" declares ")) (rcat (rtick (twice)) (rtext (" twice")))))) | Option.none => Except.ok (({ core := (s2).core, declared := (s2).declared, traits := (s2).traits, structs := (s2).structs, enums := (LexLeanRuntime.append ((s2).enums) ((({ name := name, variants := variants, gcount := (LexLeanRuntime.length (generics) : Nat) } : EnumE) :: ([] : List (EnumE)))) : List (EnumE)), aliases := (s2).aliases, statics := (s2).statics, functions := (s2).functions, methods := (s2).methods, imported := (s2).imported } : Sc)))))))

@[expose] public def pass1 (sc : Sc) (items : List (Item)) : Except (ByteArray) (Sc) := (LexLeanCollections.listFold ((fun (state : Except (ByteArray) (Sc)) (item : Item) => (match state with | Except.error problem => Except.error (problem) | Except.ok held => pass1Item (held) (item)))) (Except.ok (sc)) (items) : Except (ByteArray) (Sc))

@[expose] public def pass2Item (sc : Sc) (item : Item) : Option (ByteArray) := (match item with | Item.IUse _ _ => Option.none | Item.IUnit _ _ => Option.none | Item.IAlias _ params ty => tyOk (sc) (ty) ((LexLeanRuntime.length (params) : Nat)) (false) | Item.IStatic _ ty init => (match tyOk (sc) (ty) (0) (false) with | Option.some stop => Option.some (stop) | Option.none => exprOk (sc) (emptyFr (())) (init)) | Item.ITrait name sigs => (match (match firstDuplicateSig (sigs) with | Option.some twice => Option.some (rcat (rtext ("runtime support: trait-method: ")) (rcat (rtick (name)) (rcat (rtext (" declares ")) (rcat (rtick (twice)) (rtext (" twice")))))) | Option.none => Option.none) with | Option.some stop => Option.some (stop) | Option.none => (LexLeanCollections.listFold ((fun (found : Option (ByteArray)) (sig : Sig) => (match found with | Option.some stop => Option.some (stop) | Option.none => sigOk (sc) (sig) (0) (true)))) (Option.none) (sigs) : Option (ByteArray))) | Item.IImpl generics tr ty fns => (match genericsOk (sc) (generics) with | Option.some stop => Option.some (stop) | Option.none => (match tyOk (sc) (ty) ((LexLeanRuntime.length (generics) : Nat)) (false) with | Option.some stop => Option.some (stop) | Option.none => (match tr with | List.cons named _ => completes (sc) (named) (generics) (ty) (fns) | List.nil => (LexLeanCollections.listFold ((fun (found : Option (ByteArray)) (f : FnDef) => (match found with | Option.some stop => Option.some (stop) | Option.none => fnOk (sc) (generics) (ty) (f)))) (Option.none) (fns) : Option (ByteArray))))) | Item.IFn f => (if (!(f).«public») then Option.some (rcat (rtext ("runtime support: function-visibility: ")) (rcat (rtick (((f).sig).name)) (rtext (" is a free function that is not public")))) else (if ((f).sig).receiver then Option.some (rcat (rtext ("runtime support: function-receiver: ")) (rcat (rtext ("free function ")) (rcat (rtick (((f).sig).name)) (rtext (" takes `&self`"))))) else fnOk (sc) (([] : List (Generic))) (Ty.TyAbsent) (f))) | Item.ITuple _ generics _ tys => tyOk (sc) (tys) ((LexLeanRuntime.length (generics) : Nat)) (false) | Item.IRecord _ generics _ _ tys => tyOk (sc) (tys) ((LexLeanRuntime.length (generics) : Nat)) (false) | Item.IEnum _ generics _ variants => (LexLeanCollections.listFold ((fun (found : Option (ByteArray)) (variant : Variant) => (match found with | Option.some stop => Option.some (stop) | Option.none => (match variant with | Variant.VUnit _ => Option.none | Variant.VTuple _ tys => tyOk (sc) (tys) ((LexLeanRuntime.length (generics) : Nat)) (false) | Variant.VRecord _ _ tys => tyOk (sc) (tys) ((LexLeanRuntime.length (generics) : Nat)) (false))))) (Option.none) (variants) : Option (ByteArray)))

@[expose] public def validateRuntime (krate : Crate) : Except (ByteArray) (Unit) := (match pass1 (({ core := (krate).core, declared := ([] : List (ByteArray)), traits := ([] : List (TraitE)), structs := ([] : List (StructE)), enums := ([] : List (EnumE)), aliases := ([] : List (Entry)), statics := ([] : List (ByteArray)), functions := ([] : List (Entry)), methods := ([] : List (Entry)), imported := ([] : List (ByteArray)) } : Sc)) ((krate).items) with | Except.error problem => Except.error (problem) | Except.ok sc => (match (LexLeanCollections.listFold ((fun (found : Option (ByteArray)) (item : Item) => (match found with | Option.some stop => Option.some (stop) | Option.none => pass2Item (sc) (item)))) (Option.none) ((krate).items) : Option (ByteArray)) with | Option.some problem2 => Except.error (problem2) | Option.none => Except.ok (())))

end LexLeanBoot.RuntimeValidate
