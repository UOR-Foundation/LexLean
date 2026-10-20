module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Text
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.CalculusCheck

public inductive IntKind where
  | U8
  | U16
  | U32
  | U64
  | I8
  | I16
  | I32
  | I64

public inductive OrderingValue where
  | OLt
  | OEq
  | OGt

public inductive Ty where
  | TUnit
  | TBool
  | TNat
  | TInt
  | TFixed (_ : IntKind)
  | TString
  | TBytes
  | TOrdering
  | TOption (_ : Ty)
  | TResult (_ : Ty) (_ : Ty)
  | TList (_ : Ty)
  | TPair (_ : Ty) (_ : Ty)
  | TAdt (_ : Nat)
  | TFn (_ : Ty) (_ : Ty)
  | TNil
  | TCons (_ : Ty) (_ : Ty)

public inductive Value where
  | VUnit
  | VBool (_ : Bool)
  | VNat (_ : ByteArray)
  | VInt (_ : ByteArray)
  | VU8 (_ : ByteArray)
  | VU16 (_ : ByteArray)
  | VU32 (_ : ByteArray)
  | VU64 (_ : ByteArray)
  | VI8 (_ : ByteArray)
  | VI16 (_ : ByteArray)
  | VI32 (_ : ByteArray)
  | VI64 (_ : ByteArray)
  | VString (_ : ByteArray)
  | VBytes (_ : ByteArray)
  | VOrdering (_ : OrderingValue)
  | VNone
  | VSome (_ : Value)
  | VOk (_ : Value)
  | VError (_ : Value)
  | VList (_ : Value)
  | VPair (_ : Value) (_ : Value)
  | VAdt (_ : Nat) (_ : Value)
  | VClosure (_ : Nat) (_ : Value)
  | VNil
  | VCons (_ : Value) (_ : Value)

public inductive Shape where
  | SNone
  | SSome
  | SOk
  | SError
  | SNil
  | SCons
  | SZero
  | SSucc
  | SPair
  | STrue
  | SFalse
  | SUnit
  | SLt
  | SEq
  | SGt
  | SAdt (_ : Nat)

public inductive Prim where
  | NatAdd
  | NatSub
  | NatMul
  | NatQuot
  | NatRem
  | NatEq
  | NatLe
  | NatLt
  | IntAdd
  | IntSub
  | IntMul
  | IntNeg
  | IntQuot
  | IntRem
  | CheckedAdd
  | CheckedSub
  | CheckedMul
  | CheckedNeg
  | CheckedQuot
  | BitAnd
  | BitOr
  | BitXor
  | BitNot
  | ShiftLeft
  | ShiftRight
  | Equal
  | BoolNot
  | BoolAnd
  | BoolOr
  | Append
  | Length
  | Index
  | Slice
  | Utf8Encode
  | Utf8Decode
  | CompareBytes
  | SplitExact
  | Join
  | FormatDecimal
  | Compare
  | Convert (_ : IntKind)
  | ParseDecimal (_ : Ty)

public inductive Expr where
  | EValue (_ : Ty) (_ : Value)
  | EVar (_ : Nat)
  | ELet (_ : Nat) (_ : Ty) (_ : Expr) (_ : Expr)
  | ECond (_ : Expr) (_ : Expr) (_ : Expr)
  | EMatch (_ : Ty) (_ : Expr) (_ : Expr)
  | EBuild (_ : Shape) (_ : Ty) (_ : Expr)
  | ECall (_ : Nat) (_ : Expr)
  | EClosure (_ : Nat) (_ : Expr)
  | EApply (_ : Expr) (_ : Expr)
  | EPrim (_ : Prim) (_ : Expr)
  | EFirst (_ : Expr)
  | ESecond (_ : Expr)
  | EField (_ : Expr) (_ : Nat)
  | ENil
  | ECons (_ : Expr) (_ : Expr)
  | EArm (_ : Shape) (_ : List (Nat)) (_ : Expr)

public inductive Outcome where
  | OFailed (_ : ByteArray)
  | OTyped (_ : Ty)
  | OCovered (_ : List (Shape))
  | OFine

public structure Program where
  spec : ByteArray
  adts : Ty
  parameters : List (List (Nat))
  types : Ty
  results : Ty
  bodies : Expr

@[expose] public def text (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def cat (left : ByteArray) (right : ByteArray) : ByteArray := (LexLeanRuntime.append (left) (right) : ByteArray)

@[expose] public def quoted (name : ByteArray) : ByteArray := cat (text ("`")) (cat (name) (text ("`")))

@[expose] public def listLength (Item : Type) (items : List (Item)) : Nat := (LexLeanCollections.listFold ((fun (count : Nat) (_item : Item) => (count + 1))) (0) (items) : Nat)

@[expose] public def byteIs (subject : ByteArray) (position : Nat) (value : UInt8) : Bool := (match LexLeanStd.Text.byteAt (subject) (position) with | Option.some byte => (LexLeanRuntime.equal (byte) (value) : Bool) | Option.none => false)

@[expose] public def zeroStep (subject : ByteArray) (position : Nat) : Option (Nat) := (if byteIs (subject) (position) ((48 : UInt8)) then Option.some ((position + 1)) else Option.none)

@[expose] public def significant (digits : ByteArray) : ByteArray := (let zeros : Nat := ((LexLeanCollections.iterateUntil ((fun (position : Nat) => zeroStep (digits) (position))) ((LexLeanRuntime.length (digits) : Nat)) (0) : (Prod (Nat) (Bool)))).1; LexLeanStd.Text.sliceOr (digits) (zeros) ((LexLeanRuntime.subtract ((LexLeanRuntime.length (digits) : Nat)) (zeros) : Nat)))

@[expose] public def digit (number : Nat) (place : Nat) : ByteArray := LexLeanStd.Text.sliceOr (text ("0123456789")) ((LexLeanRuntime.remainder ((LexLeanRuntime.quotient (number) (place) (0) : Nat)) (10) (0) : Nat)) (1)

@[expose] public def decimal (number : Nat) : ByteArray := (let zero : ByteArray := text ("0"); (let digits : ByteArray := significant ((LexLeanRuntime.append (digit (number) (10000000000000000000)) ((LexLeanRuntime.append (digit (number) (1000000000000000000)) ((LexLeanRuntime.append (digit (number) (100000000000000000)) ((LexLeanRuntime.append (digit (number) (10000000000000000)) ((LexLeanRuntime.append (digit (number) (1000000000000000)) ((LexLeanRuntime.append (digit (number) (100000000000000)) ((LexLeanRuntime.append (digit (number) (10000000000000)) ((LexLeanRuntime.append (digit (number) (1000000000000)) ((LexLeanRuntime.append (digit (number) (100000000000)) ((LexLeanRuntime.append (digit (number) (10000000000)) ((LexLeanRuntime.append (digit (number) (1000000000)) ((LexLeanRuntime.append (digit (number) (100000000)) ((LexLeanRuntime.append (digit (number) (10000000)) ((LexLeanRuntime.append (digit (number) (1000000)) ((LexLeanRuntime.append (digit (number) (100000)) ((LexLeanRuntime.append (digit (number) (10000)) ((LexLeanRuntime.append (digit (number) (1000)) ((LexLeanRuntime.append (digit (number) (100)) ((LexLeanRuntime.append (digit (number) (10)) (digit (number) (1)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)); (if (Nat.beq (number) (0)) then zero else digits)))

@[expose] public def tyTag (ty : Ty) : Nat := (match ty with | Ty.TUnit => 0 | Ty.TBool => 1 | Ty.TNat => 2 | Ty.TInt => 3 | Ty.TFixed _ => 4 | Ty.TString => 5 | Ty.TBytes => 6 | Ty.TOrdering => 7 | Ty.TOption _ => 8 | Ty.TResult _ _ => 9 | Ty.TList _ => 10 | Ty.TPair _ _ => 11 | Ty.TAdt _ => 12 | Ty.TFn _ _ => 13 | Ty.TNil => 14 | Ty.TCons _ _ => 15)

@[expose] public def tyFirstOption (ty : Ty) : Option (Ty) := (match ty with | Ty.TUnit => Option.none | Ty.TBool => Option.none | Ty.TNat => Option.none | Ty.TInt => Option.none | Ty.TFixed _ => Option.none | Ty.TString => Option.none | Ty.TBytes => Option.none | Ty.TOrdering => Option.none | Ty.TOption value => Option.some (value) | Ty.TResult ok _ => Option.some (ok) | Ty.TList element => Option.some (element) | Ty.TPair left _ => Option.some (left) | Ty.TAdt _ => Option.none | Ty.TFn parameters _ => Option.some (parameters) | Ty.TNil => Option.none | Ty.TCons head _ => Option.some (head))

@[expose] public def tySecondOption (ty : Ty) : Option (Ty) := (match ty with | Ty.TUnit => Option.none | Ty.TBool => Option.none | Ty.TNat => Option.none | Ty.TInt => Option.none | Ty.TFixed _ => Option.none | Ty.TString => Option.none | Ty.TBytes => Option.none | Ty.TOrdering => Option.none | Ty.TOption _ => Option.none | Ty.TResult _ error => Option.some (error) | Ty.TList _ => Option.none | Ty.TPair _ right => Option.some (right) | Ty.TAdt _ => Option.none | Ty.TFn _ result => Option.some (result) | Ty.TNil => Option.none | Ty.TCons _ tail => Option.some (tail))

@[expose] public def tyFirst (ty : Ty) : Ty := (match tyFirstOption (ty) with | Option.some found => found | Option.none => Ty.TUnit)

@[expose] public def tySecond (ty : Ty) : Ty := (match tySecondOption (ty) with | Option.some found => found | Option.none => Ty.TUnit)

@[expose] public def tyAdtIndex (ty : Ty) : Nat := (match ty with | Ty.TUnit => 0 | Ty.TBool => 0 | Ty.TNat => 0 | Ty.TInt => 0 | Ty.TFixed _ => 0 | Ty.TString => 0 | Ty.TBytes => 0 | Ty.TOrdering => 0 | Ty.TOption _ => 0 | Ty.TResult _ _ => 0 | Ty.TList _ => 0 | Ty.TPair _ _ => 0 | Ty.TAdt index => index | Ty.TFn _ _ => 0 | Ty.TNil => 0 | Ty.TCons _ _ => 0)

@[expose] public def tyWidth (ty : Ty) : IntKind := (match ty with | Ty.TUnit => IntKind.U8 | Ty.TBool => IntKind.U8 | Ty.TNat => IntKind.U8 | Ty.TInt => IntKind.U8 | Ty.TFixed width => width | Ty.TString => IntKind.U8 | Ty.TBytes => IntKind.U8 | Ty.TOrdering => IntKind.U8 | Ty.TOption _ => IntKind.U8 | Ty.TResult _ _ => IntKind.U8 | Ty.TList _ => IntKind.U8 | Ty.TPair _ _ => IntKind.U8 | Ty.TAdt _ => IntKind.U8 | Ty.TFn _ _ => IntKind.U8 | Ty.TNil => IntKind.U8 | Ty.TCons _ _ => IntKind.U8)

@[expose] public def isNilTy (ty : Ty) : Bool := (Nat.beq (tyTag (ty)) (14))

@[expose] public def kindIndex (kind : IntKind) : Nat := (match kind with | IntKind.U8 => 0 | IntKind.U16 => 1 | IntKind.U32 => 2 | IntKind.U64 => 3 | IntKind.I8 => 4 | IntKind.I16 => 5 | IntKind.I32 => 6 | IntKind.I64 => 7)

@[expose] public def kindSigned (kind : IntKind) : Bool := (match kind with | IntKind.U8 => false | IntKind.U16 => false | IntKind.U32 => false | IntKind.U64 => false | IntKind.I8 => true | IntKind.I16 => true | IntKind.I32 => true | IntKind.I64 => true)

@[expose] public def tyChainLength : (ty : Ty) -> Nat
  | Ty.TUnit => 0
  | Ty.TBool => 0
  | Ty.TNat => 0
  | Ty.TInt => 0
  | Ty.TFixed _ => 0
  | Ty.TString => 0
  | Ty.TBytes => 0
  | Ty.TOrdering => 0
  | Ty.TOption _ => 0
  | Ty.TResult _ _ => 0
  | Ty.TList _ => 0
  | Ty.TPair _ _ => 0
  | Ty.TAdt _ => 0
  | Ty.TFn _ _ => 0
  | Ty.TNil => 0
  | Ty.TCons _ tail => (let rest : Nat := tyChainLength (tail); (1 + rest))

@[expose] public def tyChainAt : (ty : Ty) -> («at» : Nat) -> Option (Ty)
  | Ty.TUnit, _at => Option.none
  | Ty.TBool, _at => Option.none
  | Ty.TNat, _at => Option.none
  | Ty.TInt, _at => Option.none
  | Ty.TFixed _, _at => Option.none
  | Ty.TString, _at => Option.none
  | Ty.TBytes, _at => Option.none
  | Ty.TOrdering, _at => Option.none
  | Ty.TOption _, _at => Option.none
  | Ty.TResult _ _, _at => Option.none
  | Ty.TList _, _at => Option.none
  | Ty.TPair _ _, _at => Option.none
  | Ty.TAdt _, _at => Option.none
  | Ty.TFn _ _, _at => Option.none
  | Ty.TNil, _at => Option.none
  | Ty.TCons head tail, «at» => (match «at» with | Nat.zero => Option.some (head) | Nat.succ previous => tyChainAt (tail) (previous))

@[expose] public def exprChainLength : (expr : Expr) -> Nat
  | Expr.EValue _ _ => 0
  | Expr.EVar _ => 0
  | Expr.ELet _ _ _ _ => 0
  | Expr.ECond _ _ _ => 0
  | Expr.EMatch _ _ _ => 0
  | Expr.EBuild _ _ _ => 0
  | Expr.ECall _ _ => 0
  | Expr.EClosure _ _ => 0
  | Expr.EApply _ _ => 0
  | Expr.EPrim _ _ => 0
  | Expr.EFirst _ => 0
  | Expr.ESecond _ => 0
  | Expr.EField _ _ => 0
  | Expr.ENil => 0
  | Expr.EArm _ _ _ => 0
  | Expr.ECons _ tail => (let rest : Nat := exprChainLength (tail); (1 + rest))

@[expose] public def valueChainLength : (value : Value) -> Nat
  | Value.VUnit => 0
  | Value.VBool _ => 0
  | Value.VNat _ => 0
  | Value.VInt _ => 0
  | Value.VU8 _ => 0
  | Value.VU16 _ => 0
  | Value.VU32 _ => 0
  | Value.VU64 _ => 0
  | Value.VI8 _ => 0
  | Value.VI16 _ => 0
  | Value.VI32 _ => 0
  | Value.VI64 _ => 0
  | Value.VString _ => 0
  | Value.VBytes _ => 0
  | Value.VOrdering _ => 0
  | Value.VNone => 0
  | Value.VSome _ => 0
  | Value.VOk _ => 0
  | Value.VError _ => 0
  | Value.VList _ => 0
  | Value.VPair _ _ => 0
  | Value.VAdt _ _ => 0
  | Value.VClosure _ _ => 0
  | Value.VNil => 0
  | Value.VCons _ tail => (let rest : Nat := valueChainLength (tail); (1 + rest))

@[expose] public def tyEq : (left : Ty) -> (right : Ty) -> Bool
  | Ty.TUnit, right => (Nat.beq (tyTag (right)) (0))
  | Ty.TBool, right => (Nat.beq (tyTag (right)) (1))
  | Ty.TNat, right => (Nat.beq (tyTag (right)) (2))
  | Ty.TInt, right => (Nat.beq (tyTag (right)) (3))
  | Ty.TString, right => (Nat.beq (tyTag (right)) (5))
  | Ty.TBytes, right => (Nat.beq (tyTag (right)) (6))
  | Ty.TOrdering, right => (Nat.beq (tyTag (right)) (7))
  | Ty.TNil, right => (Nat.beq (tyTag (right)) (14))
  | Ty.TFixed width, right => (if (Nat.beq (tyTag (right)) (4)) then (Nat.beq (kindIndex (width)) (kindIndex (tyWidth (right)))) else false)
  | Ty.TOption value, right => (let same : Bool := tyEq (value) (tyFirst (right)); (if (Nat.beq (tyTag (right)) (8)) then same else false))
  | Ty.TResult ok error, right => (let okSame : Bool := tyEq (ok) (tyFirst (right)); (let errorSame : Bool := tyEq (error) (tySecond (right)); (if (Nat.beq (tyTag (right)) (9)) then (if okSame then errorSame else false) else false)))
  | Ty.TList element, right => (let same : Bool := tyEq (element) (tyFirst (right)); (if (Nat.beq (tyTag (right)) (10)) then same else false))
  | Ty.TPair left2 right2, right => (let leftSame : Bool := tyEq (left2) (tyFirst (right)); (let rightSame : Bool := tyEq (right2) (tySecond (right)); (if (Nat.beq (tyTag (right)) (11)) then (if leftSame then rightSame else false) else false)))
  | Ty.TAdt index, right => (if (Nat.beq (tyTag (right)) (12)) then (Nat.beq (index) (tyAdtIndex (right))) else false)
  | Ty.TFn parameters result, right => (let parametersSame : Bool := tyEq (parameters) (tyFirst (right)); (let resultSame : Bool := tyEq (result) (tySecond (right)); (if (Nat.beq (tyTag (right)) (13)) then (if parametersSame then resultSame else false) else false)))
  | Ty.TCons head tail, right => (let headSame : Bool := tyEq (head) (tyFirst (right)); (let tailSame : Bool := tyEq (tail) (tySecond (right)); (if (Nat.beq (tyTag (right)) (15)) then (if headSame then tailSame else false) else false)))

@[expose] public def orderable : (ty : Ty) -> Bool
  | Ty.TNat => true
  | Ty.TInt => true
  | Ty.TBool => true
  | Ty.TString => true
  | Ty.TFixed _ => true
  | Ty.TPair left right => (let leftOk : Bool := orderable (left); (let rightOk : Bool := orderable (right); (if leftOk then rightOk else false)))
  | Ty.TUnit => false
  | Ty.TBytes => false
  | Ty.TOrdering => false
  | Ty.TNil => false
  | Ty.TOption _ => false
  | Ty.TResult _ _ => false
  | Ty.TList _ => false
  | Ty.TAdt _ => false
  | Ty.TFn _ _ => false
  | Ty.TCons _ _ => false

@[expose] public def shapeTag (shape : Shape) : Nat := (match shape with | Shape.SNone => 0 | Shape.SSome => 1 | Shape.SOk => 2 | Shape.SError => 3 | Shape.SNil => 4 | Shape.SCons => 5 | Shape.SZero => 6 | Shape.SSucc => 7 | Shape.SPair => 8 | Shape.STrue => 9 | Shape.SFalse => 10 | Shape.SUnit => 11 | Shape.SLt => 12 | Shape.SEq => 13 | Shape.SGt => 14 | Shape.SAdt _ => 15)

@[expose] public def shapeArg (shape : Shape) : Nat := (match shape with | Shape.SNone => 0 | Shape.SSome => 0 | Shape.SOk => 0 | Shape.SError => 0 | Shape.SNil => 0 | Shape.SCons => 0 | Shape.SZero => 0 | Shape.SSucc => 0 | Shape.SPair => 0 | Shape.STrue => 0 | Shape.SFalse => 0 | Shape.SUnit => 0 | Shape.SLt => 0 | Shape.SEq => 0 | Shape.SGt => 0 | Shape.SAdt constructor => constructor)

@[expose] public def shapeEq (left : Shape) (right : Shape) : Bool := (if (Nat.beq (shapeTag (left)) (shapeTag (right))) then (Nat.beq (shapeArg (left)) (shapeArg (right))) else false)

@[expose] public def shapeIn : (shapes : List (Shape)) -> (shape : Shape) -> Bool
  | List.nil, _shape => false
  | List.cons head tail, shape => (let here : Bool := shapeEq (head) (shape); (let later : Bool := shapeIn (tail) (shape); (if here then true else later)))

@[expose] public def natIn : (items : List (Nat)) -> (item : Nat) -> Bool
  | List.nil, _item => false
  | List.cons head tail, item => (let later : Bool := natIn (tail) (item); (if (Nat.beq (head) (item)) then true else later))

@[expose] public def hasRepeat : (items : List (Nat)) -> Bool
  | List.nil => false
  | List.cons head tail => (let repeats : Bool := hasRepeat (tail); (let seen : Bool := natIn (tail) (head); (if seen then true else repeats)))

@[expose] public def listAt (Item : Type) : (items : List (Item)) -> (index : Nat) -> Option (Item)
  | List.nil, _index => Option.none
  | List.cons head tail, index => (match index with | Nat.zero => Option.some (head) | Nat.succ previous => listAt (Item) (tail) (previous))

@[expose] public def consTy (head : Ty) (tail : Ty) : Ty := Ty.TCons (head) (tail)

@[expose] public def closureTy (parameters : Ty) (result : Ty) : Ty := Ty.TFn (parameters) (result)

@[expose] public def consShape (head : Shape) (tail : List (Shape)) : List (Shape) := (head :: tail)

@[expose] public def kindDebug (kind : IntKind) : ByteArray := (match kind with | IntKind.U8 => text ("U8") | IntKind.U16 => text ("U16") | IntKind.U32 => text ("U32") | IntKind.U64 => text ("U64") | IntKind.I8 => text ("I8") | IntKind.I16 => text ("I16") | IntKind.I32 => text ("I32") | IntKind.I64 => text ("I64"))

@[expose] public def kindName (kind : IntKind) : ByteArray := (match kind with | IntKind.U8 => text ("u8") | IntKind.U16 => text ("u16") | IntKind.U32 => text ("u32") | IntKind.U64 => text ("u64") | IntKind.I8 => text ("i8") | IntKind.I16 => text ("i16") | IntKind.I32 => text ("i32") | IntKind.I64 => text ("i64"))

@[expose] public def orderingDebug (value : OrderingValue) : ByteArray := (match value with | OrderingValue.OLt => text ("Lt") | OrderingValue.OEq => text ("Eq") | OrderingValue.OGt => text ("Gt"))

@[expose] public def boolDebug (value : Bool) : ByteArray := (if value then text ("true") else text ("false"))

@[expose] public def hexNibble (n : Nat) : ByteArray := LexLeanStd.Text.sliceOr (text ("0123456789abcdef")) (n) (1)

@[expose] public def hexByte (n : Nat) : ByteArray := (let single : ByteArray := hexNibble (n); (let double : ByteArray := cat (hexNibble ((LexLeanRuntime.quotient (n) (16) (0) : Nat))) (hexNibble ((LexLeanRuntime.remainder (n) (16) (0) : Nat))); (if (Nat.blt (n) (16)) then single else double)))

@[expose] public def unicodeEscape (n : Nat) : ByteArray := cat (text ("\\u{")) (cat (hexByte (n)) (text ("}")))

@[expose] public def byteNat (subject : ByteArray) (position : Nat) : Nat := (match LexLeanStd.Text.byteAt (subject) (position) with | Option.some byte => (let __widen0 : UInt8 := byte; ((((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((128 : UInt8)) : UInt8)) ((128 : UInt8)) : Bool) then 128 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((64 : UInt8)) : UInt8)) ((64 : UInt8)) : Bool) then 64 else 0)) + ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((32 : UInt8)) : UInt8)) ((32 : UInt8)) : Bool) then 32 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((16 : UInt8)) : UInt8)) ((16 : UInt8)) : Bool) then 16 else 0))) + (((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((8 : UInt8)) : UInt8)) ((8 : UInt8)) : Bool) then 8 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((4 : UInt8)) : UInt8)) ((4 : UInt8)) : Bool) then 4 else 0)) + ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((2 : UInt8)) : UInt8)) ((2 : UInt8)) : Bool) then 2 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((1 : UInt8)) : UInt8)) ((1 : UInt8)) : Bool) then 1 else 0))))) | Option.none => 256)

@[expose] public def escapePiece (subject : ByteArray) (position : Nat) (byte : Nat) : (Prod (Nat) (ByteArray)) := (let following : Nat := byteNat (subject) ((position + 1)); (if (Nat.beq (byte) (34)) then (1, text ("\\\"")) else (if (Nat.beq (byte) (92)) then (1, text ("\\\\")) else (if (Nat.beq (byte) (10)) then (1, text ("\\n")) else (if (Nat.beq (byte) (13)) then (1, text ("\\r")) else (if (Nat.beq (byte) (9)) then (1, text ("\\t")) else (if (Nat.beq (byte) (0)) then (1, text ("\\0")) else (if (Nat.blt (byte) (32)) then (1, unicodeEscape (byte)) else (if (Nat.beq (byte) (127)) then (1, unicodeEscape (byte)) else (if (Nat.beq (byte) (194)) then (if (Nat.ble (128) (following)) then (if (Nat.ble (following) (159)) then (2, unicodeEscape (following)) else (1, LexLeanStd.Text.sliceOr (subject) (position) (1))) else (1, LexLeanStd.Text.sliceOr (subject) (position) (1))) else (1, LexLeanStd.Text.sliceOr (subject) (position) (1))))))))))))

@[expose] public def escapeStep (subject : ByteArray) (state : (Prod (Nat) (ByteArray))) : Option ((Prod (Nat) (ByteArray))) := (let position : Nat := (state).1; (let byte : Nat := byteNat (subject) (position); (if (Nat.beq (byte) (256)) then Option.none else (let piece : (Prod (Nat) (ByteArray)) := escapePiece (subject) (position) (byte); Option.some (((position + (piece).1), cat ((state).2) ((piece).2)))))))

@[expose] public def strDebug (subject : ByteArray) : ByteArray := cat (text ("\"")) (cat ((((LexLeanCollections.iterateUntil ((fun (state : (Prod (Nat) (ByteArray))) => escapeStep (subject) (state))) ((LexLeanRuntime.length (subject) : Nat)) ((0, _root_.ByteArray.mk #[])) : (Prod ((Prod (Nat) (ByteArray))) (Bool)))).1).2) (text ("\"")))

@[expose] public def tyDebug : (ty : Ty) -> ByteArray
  | Ty.TUnit => text ("Unit")
  | Ty.TBool => text ("Bool")
  | Ty.TNat => text ("Nat")
  | Ty.TInt => text ("Int")
  | Ty.TFixed width => cat (text ("Fixed { width: ")) (cat (kindDebug (width)) (text (" }")))
  | Ty.TString => text ("String")
  | Ty.TBytes => text ("Bytes")
  | Ty.TOrdering => text ("Ordering")
  | Ty.TOption value => cat (text ("Option { value: ")) (cat (tyDebug (value)) (text (" }")))
  | Ty.TResult ok error => cat (text ("Result { ok: ")) (cat (tyDebug (ok)) (cat (text (", error: ")) (cat (tyDebug (error)) (text (" }")))))
  | Ty.TList element => cat (text ("List { element: ")) (cat (tyDebug (element)) (text (" }")))
  | Ty.TPair left right => cat (text ("Pair { left: ")) (cat (tyDebug (left)) (cat (text (", right: ")) (cat (tyDebug (right)) (text (" }")))))
  | Ty.TAdt index => cat (text ("Adt { index: ")) (cat (decimal (index)) (text (" }")))
  | Ty.TFn parameters result => cat (text ("Fn { parameters: [")) (cat (tyDebug (parameters)) (cat (text ("], result: ")) (cat (tyDebug (result)) (text (" }")))))
  | Ty.TNil => text ("")
  | Ty.TCons head tail => (let headText : ByteArray := tyDebug (head); (let rest : ByteArray := tyDebug (tail); (if isNilTy (tail) then headText else cat (headText) (cat (text (", ")) (rest)))))

@[expose] public def tyList (ty : Ty) : ByteArray := cat (text ("[")) (cat (tyDebug (ty)) (text ("]")))

@[expose] public def shapeDebug (shape : Shape) : ByteArray := (match shape with | Shape.SNone => text ("None") | Shape.SSome => text ("Some") | Shape.SOk => text ("Ok") | Shape.SError => text ("Error") | Shape.SNil => text ("Nil") | Shape.SCons => text ("Cons") | Shape.SZero => text ("Zero") | Shape.SSucc => text ("Succ") | Shape.SPair => text ("Pair") | Shape.STrue => text ("True") | Shape.SFalse => text ("False") | Shape.SUnit => text ("Unit") | Shape.SLt => text ("Lt") | Shape.SEq => text ("Eq") | Shape.SGt => text ("Gt") | Shape.SAdt constructor => cat (text ("Adt { constructor: ")) (cat (decimal (constructor)) (text (" }"))))

@[expose] public def primDebug (operation : Prim) : ByteArray := (match operation with | Prim.NatAdd => text ("NatAdd") | Prim.NatSub => text ("NatSub") | Prim.NatMul => text ("NatMul") | Prim.NatQuot => text ("NatQuot") | Prim.NatRem => text ("NatRem") | Prim.NatEq => text ("NatEq") | Prim.NatLe => text ("NatLe") | Prim.NatLt => text ("NatLt") | Prim.IntAdd => text ("IntAdd") | Prim.IntSub => text ("IntSub") | Prim.IntMul => text ("IntMul") | Prim.IntNeg => text ("IntNeg") | Prim.IntQuot => text ("IntQuot") | Prim.IntRem => text ("IntRem") | Prim.CheckedAdd => text ("CheckedAdd") | Prim.CheckedSub => text ("CheckedSub") | Prim.CheckedMul => text ("CheckedMul") | Prim.CheckedNeg => text ("CheckedNeg") | Prim.CheckedQuot => text ("CheckedQuot") | Prim.BitAnd => text ("BitAnd") | Prim.BitOr => text ("BitOr") | Prim.BitXor => text ("BitXor") | Prim.BitNot => text ("BitNot") | Prim.ShiftLeft => text ("ShiftLeft") | Prim.ShiftRight => text ("ShiftRight") | Prim.Equal => text ("Equal") | Prim.BoolNot => text ("BoolNot") | Prim.BoolAnd => text ("BoolAnd") | Prim.BoolOr => text ("BoolOr") | Prim.Append => text ("Append") | Prim.Length => text ("Length") | Prim.Index => text ("Index") | Prim.Slice => text ("Slice") | Prim.Utf8Encode => text ("Utf8Encode") | Prim.Utf8Decode => text ("Utf8Decode") | Prim.CompareBytes => text ("CompareBytes") | Prim.SplitExact => text ("SplitExact") | Prim.Join => text ("Join") | Prim.FormatDecimal => text ("FormatDecimal") | Prim.Compare => text ("Compare") | Prim.Convert target => cat (text ("Convert { target: ")) (cat (kindDebug (target)) (text (" }"))) | Prim.ParseDecimal target => cat (text ("ParseDecimal { target: ")) (cat (tyDebug (target)) (text (" }"))))

@[expose] public def isNilValue (value : Value) : Bool := (match value with | Value.VUnit => false | Value.VBool _ => false | Value.VNat _ => false | Value.VInt _ => false | Value.VU8 _ => false | Value.VU16 _ => false | Value.VU32 _ => false | Value.VU64 _ => false | Value.VI8 _ => false | Value.VI16 _ => false | Value.VI32 _ => false | Value.VI64 _ => false | Value.VString _ => false | Value.VBytes _ => false | Value.VOrdering _ => false | Value.VNone => false | Value.VSome _ => false | Value.VOk _ => false | Value.VError _ => false | Value.VList _ => false | Value.VPair _ _ => false | Value.VAdt _ _ => false | Value.VClosure _ _ => false | Value.VNil => true | Value.VCons _ _ => false)

@[expose] public def valDebug : (value : Value) -> ByteArray
  | Value.VUnit => text ("Unit")
  | Value.VBool flag => cat (text ("Bool { value: ")) (cat (boolDebug (flag)) (text (" }")))
  | Value.VNat digits => cat (text ("Nat { value: ")) (cat (strDebug (digits)) (text (" }")))
  | Value.VInt digits => cat (text ("Int { value: ")) (cat (strDebug (digits)) (text (" }")))
  | Value.VU8 digits => cat (text ("U8 { value: ")) (cat (strDebug (digits)) (text (" }")))
  | Value.VU16 digits => cat (text ("U16 { value: ")) (cat (strDebug (digits)) (text (" }")))
  | Value.VU32 digits => cat (text ("U32 { value: ")) (cat (strDebug (digits)) (text (" }")))
  | Value.VU64 digits => cat (text ("U64 { value: ")) (cat (strDebug (digits)) (text (" }")))
  | Value.VI8 digits => cat (text ("I8 { value: ")) (cat (strDebug (digits)) (text (" }")))
  | Value.VI16 digits => cat (text ("I16 { value: ")) (cat (strDebug (digits)) (text (" }")))
  | Value.VI32 digits => cat (text ("I32 { value: ")) (cat (strDebug (digits)) (text (" }")))
  | Value.VI64 digits => cat (text ("I64 { value: ")) (cat (strDebug (digits)) (text (" }")))
  | Value.VString contents => cat (text ("String { value: ")) (cat (strDebug (contents)) (text (" }")))
  | Value.VBytes hex => cat (text ("Bytes { hex: ")) (cat (strDebug (hex)) (text (" }")))
  | Value.VOrdering order => cat (text ("Ordering { value: ")) (cat (orderingDebug (order)) (text (" }")))
  | Value.VNone => text ("None")
  | Value.VSome inner => cat (text ("Some { value: ")) (cat (valDebug (inner)) (text (" }")))
  | Value.VOk inner => cat (text ("Ok { value: ")) (cat (valDebug (inner)) (text (" }")))
  | Value.VError inner => cat (text ("Error { value: ")) (cat (valDebug (inner)) (text (" }")))
  | Value.VList items => cat (text ("List { items: [")) (cat (valDebug (items)) (text ("] }")))
  | Value.VPair left right => cat (text ("Pair { left: ")) (cat (valDebug (left)) (cat (text (", right: ")) (cat (valDebug (right)) (text (" }")))))
  | Value.VAdt constructor fields => cat (text ("Adt { constructor: ")) (cat (decimal (constructor)) (cat (text (", fields: [")) (cat (valDebug (fields)) (text ("] }")))))
  | Value.VClosure function captures => cat (text ("Closure { function: ")) (cat (decimal (function)) (cat (text (", captures: [")) (cat (valDebug (captures)) (text ("] }")))))
  | Value.VNil => text ("")
  | Value.VCons head tail => (let headText : ByteArray := valDebug (head); (let rest : ByteArray := valDebug (tail); (if isNilValue (tail) then headText else cat (headText) (cat (text (", ")) (rest)))))

@[expose] public def digitAt (subject : ByteArray) (position : Nat) : Bool := (let n : Nat := byteNat (subject) (position); (if (Nat.blt (47) (n)) then (Nat.blt (n) (58)) else false))

@[expose] public def digitStep (subject : ByteArray) (position : Nat) : Option (Nat) := (if digitAt (subject) (position) then Option.some ((position + 1)) else Option.none)

@[expose] public def allDigits (subject : ByteArray) : Bool := (Nat.beq (((LexLeanCollections.iterateUntil ((fun (position : Nat) => digitStep (subject) (position))) ((LexLeanRuntime.length (subject) : Nat)) (0) : (Prod (Nat) (Bool)))).1) ((LexLeanRuntime.length (subject) : Nat)))

@[expose] public def hexDigitAt (subject : ByteArray) (position : Nat) : Bool := (let n : Nat := byteNat (subject) (position); (if (Nat.blt (47) (n)) then (if (Nat.blt (n) (58)) then true else (if (Nat.blt (96) (n)) then (Nat.blt (n) (103)) else false)) else false))

@[expose] public def hexStep (subject : ByteArray) (position : Nat) : Option (Nat) := (if hexDigitAt (subject) (position) then Option.some ((position + 1)) else Option.none)

@[expose] public def allHex (subject : ByteArray) : Bool := (Nat.beq (((LexLeanCollections.iterateUntil ((fun (position : Nat) => hexStep (subject) (position))) ((LexLeanRuntime.length (subject) : Nat)) (0) : (Prod (Nat) (Bool)))).1) ((LexLeanRuntime.length (subject) : Nat)))

@[expose] public def startsWithByte (subject : ByteArray) (value : UInt8) : Bool := byteIs (subject) (0) (value)

@[expose] public def bytesLess (left : ByteArray) (right : ByteArray) : Bool := (LexLeanRuntime.equal ((LexLeanRuntime.compareBytes (left) (right) : Ordering)) ((LexLeanRuntime.compareBytes (_root_.ByteArray.mk #[]) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 1))]) : Ordering)) : Bool)

@[expose] public def magnitudeAtMost (digits : ByteArray) (limit : ByteArray) : Bool := (let shorter : Bool := (Nat.blt ((LexLeanRuntime.length (digits) : Nat)) ((LexLeanRuntime.length (limit) : Nat))); (let same : Bool := (Nat.beq ((LexLeanRuntime.length (digits) : Nat)) ((LexLeanRuntime.length (limit) : Nat))); (let above : Bool := bytesLess (limit) (digits); (if shorter then true else (if same then (!above) else false)))))

public structure Decimal where
  negative : Bool
  digits : ByteArray

@[expose] public def decimalAt (negative : Bool) (digits : String) : Decimal := ({ negative := negative, digits := text (digits) } : Decimal)

@[expose] public def decimalAtMost (left : Decimal) (right : Decimal) : Bool := (let downward : Bool := magnitudeAtMost ((right).digits) ((left).digits); (let upward : Bool := magnitudeAtMost ((left).digits) ((right).digits); (if (left).negative then (if (right).negative then downward else true) else (if (right).negative then false else upward))))

@[expose] public def kindLow (kind : IntKind) : Decimal := (match kind with | IntKind.U8 => decimalAt (false) ("0") | IntKind.U16 => decimalAt (false) ("0") | IntKind.U32 => decimalAt (false) ("0") | IntKind.U64 => decimalAt (false) ("0") | IntKind.I8 => decimalAt (true) ("128") | IntKind.I16 => decimalAt (true) ("32768") | IntKind.I32 => decimalAt (true) ("2147483648") | IntKind.I64 => decimalAt (true) ("9223372036854775808"))

@[expose] public def kindHigh (kind : IntKind) : Decimal := (match kind with | IntKind.U8 => decimalAt (false) ("255") | IntKind.U16 => decimalAt (false) ("65535") | IntKind.U32 => decimalAt (false) ("4294967295") | IntKind.U64 => decimalAt (false) ("18446744073709551615") | IntKind.I8 => decimalAt (false) ("127") | IntKind.I16 => decimalAt (false) ("32767") | IntKind.I32 => decimalAt (false) ("2147483647") | IntKind.I64 => decimalAt (false) ("9223372036854775807"))

@[expose] public def canonicalDigits (digits : ByteArray) (literal : ByteArray) : Bool := (if (Nat.blt (0) ((LexLeanRuntime.length (digits) : Nat))) then (if allDigits (digits) then (if (LexLeanRuntime.equal (digits) (text ("0")) : Bool) then (!(LexLeanRuntime.equal (literal) (text ("-0")) : Bool)) else (!byteIs (digits) (0) ((48 : UInt8)))) else false) else false)

@[expose] public def parseDecimal (literal : ByteArray) : Except (ByteArray) (Decimal) := (let negative : Bool := byteIs (literal) (0) ((45 : UInt8)); (let digits : ByteArray := (let rest : ByteArray := LexLeanStd.Text.sliceOr (literal) (1) ((LexLeanRuntime.subtract ((LexLeanRuntime.length (literal) : Nat)) (1) : Nat)); (if negative then rest else literal)); (let canonical : Bool := canonicalDigits (digits) (literal); (let fitsPositive : Bool := magnitudeAtMost (digits) (text ("170141183460469231731687303715884105727")); (let fitsNegative : Bool := magnitudeAtMost (digits) (text ("170141183460469231731687303715884105728")); (if canonical then (if (if negative then fitsNegative else fitsPositive) then Except.ok (({ negative := negative, digits := digits } : Decimal)) else Except.error (cat (quoted (literal)) (text (" is outside every realized integer")))) else Except.error (cat (quoted (literal)) (text (" is not a canonical decimal")))))))))

@[expose] public def fixedLiteral (literal : ByteArray) (kind : IntKind) : Except (ByteArray) (Unit) := (match parseDecimal (literal) with | Except.ok number => (let low : Decimal := kindLow (kind); (let high : Decimal := kindHigh (kind); (let above : Bool := decimalAtMost (low) (number); (let below : Bool := decimalAtMost (number) (high); (if above then (if below then Except.ok (()) else Except.error (cat (literal) (cat (text (" is outside ")) (kindName (kind))))) else Except.error (cat (literal) (cat (text (" is outside ")) (kindName (kind))))))))) | Except.error problem => Except.error (problem))

@[expose] public def natLiteral (literal : ByteArray) : Except (ByteArray) (Unit) := (match parseDecimal (literal) with | Except.ok number => (let fits : Bool := magnitudeAtMost ((number).digits) (text ("18446744073709551615")); (if (number).negative then Except.error (cat (text ("nat literal ")) (cat (literal) (text (" is outside the 64-bit realization")))) else (if fits then Except.ok (()) else Except.error (cat (text ("nat literal ")) (cat (literal) (text (" is outside the 64-bit realization"))))))) | Except.error problem => Except.error (problem))

@[expose] public def bytesLiteral (hex : ByteArray) : Except (ByteArray) (Unit) := (if (Nat.beq ((LexLeanRuntime.remainder ((LexLeanRuntime.length (hex) : Nat)) (2) (0) : Nat)) (0)) then (if allHex (hex) then Except.ok (()) else Except.error (cat (text ("byte literal `")) (cat (hex) (text ("` is not even-length lowercase hexadecimal"))))) else Except.error (cat (text ("byte literal `")) (cat (hex) (text ("` is not even-length lowercase hexadecimal")))))

@[expose] public def adtAt (program : Program) (position : Nat) : Option (Ty) := tyChainAt ((program).adts) (position)

@[expose] public def functionTypes (program : Program) (position : Nat) : Option (Ty) := tyChainAt ((program).types) (position)

@[expose] public def functionResult (program : Program) (position : Nat) : Ty := (match tyChainAt ((program).results) (position) with | Option.some found => found | Option.none => Ty.TUnit)

@[expose] public def undeclaredAdt (index : Nat) : ByteArray := cat (text ("ADT ")) (cat (decimal (index)) (text (" is not declared")))

@[expose] public def undeclaredFunction (index : Nat) : ByteArray := cat (text ("function ")) (cat (decimal (index)) (text (" is not declared")))

@[expose] public def tyOk : (program : Program) -> (ty : Ty) -> Except (ByteArray) (Unit)
  | _program, Ty.TUnit => Except.ok (())
  | _program, Ty.TBool => Except.ok (())
  | _program, Ty.TNat => Except.ok (())
  | _program, Ty.TInt => Except.ok (())
  | _program, Ty.TString => Except.ok (())
  | _program, Ty.TBytes => Except.ok (())
  | _program, Ty.TOrdering => Except.ok (())
  | _program, Ty.TNil => Except.ok (())
  | _program, Ty.TFixed _ => Except.ok (())
  | program, Ty.TOption value => tyOk (program) (value)
  | program, Ty.TResult ok error => (match tyOk (program) (ok) with | Except.ok _ => tyOk (program) (error) | Except.error problem => Except.error (problem))
  | program, Ty.TList element => tyOk (program) (element)
  | program, Ty.TPair left right => (match tyOk (program) (left) with | Except.ok _ => tyOk (program) (right) | Except.error problem => Except.error (problem))
  | program, Ty.TAdt index => (match adtAt (program) (index) with | Option.some _ => Except.ok (()) | Option.none => Except.error (undeclaredAdt (index)))
  | program, Ty.TFn parameters result => (if isNilTy (parameters) then Except.error (text ("a function type takes at least one parameter")) else (match tyOk (program) (parameters) with | Except.ok _ => tyOk (program) (result) | Except.error problem => Except.error (problem)))
  | program, Ty.TCons head tail => (match tyOk (program) (head) with | Except.ok _ => tyOk (program) (tail) | Except.error problem => Except.error (problem))

@[expose] public def expectTy (actual : Ty) (expected : Ty) (what : ByteArray) : Except (ByteArray) (Unit) := (let yes1 : Except (ByteArray) (Unit) := Except.ok (()); (let no2 : Except (ByteArray) (Unit) := Except.error (cat (what) (cat (text (" has type ")) (cat (tyDebug (actual)) (cat (text (", expected ")) (tyDebug (expected)))))); (if tyEq (actual) (expected) then yes1 else no2)))

@[expose] public def ctorFields (program : Program) (adt : Nat) (constructor : Nat) : Except (ByteArray) (Ty) := (match adtAt (program) (adt) with | Option.none => Except.error (undeclaredAdt (adt)) | Option.some declaration => (match tyChainAt (declaration) (constructor) with | Option.some fields => Except.ok (fields) | Option.none => Except.error (cat (text ("ADT ")) (cat (decimal (adt)) (cat (text (" has no constructor ")) (decimal (constructor)))))))

@[expose] public def valueOk : (program : Program) -> (value : Value) -> (ty : Ty) -> (mode : Nat) -> Except (ByteArray) (Unit)
  | _program, Value.VUnit, ty, _mode => (let yes3 : Except (ByteArray) (Unit) := Except.ok (()); (let no4 : Except (ByteArray) (Unit) := Except.error (cat (text ("the literal ")) (cat (valDebug (Value.VUnit)) (cat (text (" does not have type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (0)) then yes3 else no4)))
  | _program, Value.VBool flag, ty, _mode => (let yes5 : Except (ByteArray) (Unit) := Except.ok (()); (let no6 : Except (ByteArray) (Unit) := Except.error (cat (text ("the literal ")) (cat (valDebug (Value.VBool (flag))) (cat (text (" does not have type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (1)) then yes5 else no6)))
  | _program, Value.VNat digits, ty, _mode => (let yes7 : Except (ByteArray) (Unit) := natLiteral (digits); (let no8 : Except (ByteArray) (Unit) := Except.error (cat (text ("the literal ")) (cat (valDebug (Value.VNat (digits))) (cat (text (" does not have type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (2)) then yes7 else no8)))
  | _program, Value.VInt digits, ty, _mode => (let yes9 : Except (ByteArray) (Unit) := fixedLiteral (digits) (IntKind.I64); (let no10 : Except (ByteArray) (Unit) := Except.error (cat (text ("the literal ")) (cat (valDebug (Value.VInt (digits))) (cat (text (" does not have type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (3)) then yes9 else no10)))
  | _program, Value.VU8 digits, ty, _mode => (let yes11 : Except (ByteArray) (Unit) := fixedLiteral (digits) (IntKind.U8); (let no12 : Except (ByteArray) (Unit) := Except.error (cat (text ("the literal ")) (cat (valDebug (Value.VU8 (digits))) (cat (text (" does not have type ")) (tyDebug (ty))))); (if (if (Nat.beq (tyTag (ty)) (4)) then (Nat.beq (kindIndex (tyWidth (ty))) (0)) else false) then yes11 else no12)))
  | _program, Value.VU16 digits, ty, _mode => (let yes13 : Except (ByteArray) (Unit) := fixedLiteral (digits) (IntKind.U16); (let no14 : Except (ByteArray) (Unit) := Except.error (cat (text ("the literal ")) (cat (valDebug (Value.VU16 (digits))) (cat (text (" does not have type ")) (tyDebug (ty))))); (if (if (Nat.beq (tyTag (ty)) (4)) then (Nat.beq (kindIndex (tyWidth (ty))) (1)) else false) then yes13 else no14)))
  | _program, Value.VU32 digits, ty, _mode => (let yes15 : Except (ByteArray) (Unit) := fixedLiteral (digits) (IntKind.U32); (let no16 : Except (ByteArray) (Unit) := Except.error (cat (text ("the literal ")) (cat (valDebug (Value.VU32 (digits))) (cat (text (" does not have type ")) (tyDebug (ty))))); (if (if (Nat.beq (tyTag (ty)) (4)) then (Nat.beq (kindIndex (tyWidth (ty))) (2)) else false) then yes15 else no16)))
  | _program, Value.VU64 digits, ty, _mode => (let yes17 : Except (ByteArray) (Unit) := fixedLiteral (digits) (IntKind.U64); (let no18 : Except (ByteArray) (Unit) := Except.error (cat (text ("the literal ")) (cat (valDebug (Value.VU64 (digits))) (cat (text (" does not have type ")) (tyDebug (ty))))); (if (if (Nat.beq (tyTag (ty)) (4)) then (Nat.beq (kindIndex (tyWidth (ty))) (3)) else false) then yes17 else no18)))
  | _program, Value.VI8 digits, ty, _mode => (let yes19 : Except (ByteArray) (Unit) := fixedLiteral (digits) (IntKind.I8); (let no20 : Except (ByteArray) (Unit) := Except.error (cat (text ("the literal ")) (cat (valDebug (Value.VI8 (digits))) (cat (text (" does not have type ")) (tyDebug (ty))))); (if (if (Nat.beq (tyTag (ty)) (4)) then (Nat.beq (kindIndex (tyWidth (ty))) (4)) else false) then yes19 else no20)))
  | _program, Value.VI16 digits, ty, _mode => (let yes21 : Except (ByteArray) (Unit) := fixedLiteral (digits) (IntKind.I16); (let no22 : Except (ByteArray) (Unit) := Except.error (cat (text ("the literal ")) (cat (valDebug (Value.VI16 (digits))) (cat (text (" does not have type ")) (tyDebug (ty))))); (if (if (Nat.beq (tyTag (ty)) (4)) then (Nat.beq (kindIndex (tyWidth (ty))) (5)) else false) then yes21 else no22)))
  | _program, Value.VI32 digits, ty, _mode => (let yes23 : Except (ByteArray) (Unit) := fixedLiteral (digits) (IntKind.I32); (let no24 : Except (ByteArray) (Unit) := Except.error (cat (text ("the literal ")) (cat (valDebug (Value.VI32 (digits))) (cat (text (" does not have type ")) (tyDebug (ty))))); (if (if (Nat.beq (tyTag (ty)) (4)) then (Nat.beq (kindIndex (tyWidth (ty))) (6)) else false) then yes23 else no24)))
  | _program, Value.VI64 digits, ty, _mode => (let yes25 : Except (ByteArray) (Unit) := fixedLiteral (digits) (IntKind.I64); (let no26 : Except (ByteArray) (Unit) := Except.error (cat (text ("the literal ")) (cat (valDebug (Value.VI64 (digits))) (cat (text (" does not have type ")) (tyDebug (ty))))); (if (if (Nat.beq (tyTag (ty)) (4)) then (Nat.beq (kindIndex (tyWidth (ty))) (7)) else false) then yes25 else no26)))
  | _program, Value.VString contents, ty, _mode => (let yes27 : Except (ByteArray) (Unit) := Except.ok (()); (let no28 : Except (ByteArray) (Unit) := Except.error (cat (text ("the literal ")) (cat (valDebug (Value.VString (contents))) (cat (text (" does not have type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (5)) then yes27 else no28)))
  | _program, Value.VBytes hex, ty, _mode => (let yes29 : Except (ByteArray) (Unit) := bytesLiteral (hex); (let no30 : Except (ByteArray) (Unit) := Except.error (cat (text ("the literal ")) (cat (valDebug (Value.VBytes (hex))) (cat (text (" does not have type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (6)) then yes29 else no30)))
  | _program, Value.VOrdering order, ty, _mode => (let yes31 : Except (ByteArray) (Unit) := Except.ok (()); (let no32 : Except (ByteArray) (Unit) := Except.error (cat (text ("the literal ")) (cat (valDebug (Value.VOrdering (order))) (cat (text (" does not have type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (7)) then yes31 else no32)))
  | _program, Value.VNone, ty, _mode => (let yes33 : Except (ByteArray) (Unit) := Except.ok (()); (let no34 : Except (ByteArray) (Unit) := Except.error (cat (text ("the literal ")) (cat (valDebug (Value.VNone)) (cat (text (" does not have type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (8)) then yes33 else no34)))
  | program, Value.VSome inner, ty, _mode => (let yes35 : Except (ByteArray) (Unit) := valueOk (program) (inner) (tyFirst (ty)) (0); (let no36 : Except (ByteArray) (Unit) := Except.error (cat (text ("the literal ")) (cat (valDebug (Value.VSome (inner))) (cat (text (" does not have type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (8)) then yes35 else no36)))
  | program, Value.VOk inner, ty, _mode => (let yes37 : Except (ByteArray) (Unit) := valueOk (program) (inner) (tyFirst (ty)) (0); (let no38 : Except (ByteArray) (Unit) := Except.error (cat (text ("the literal ")) (cat (valDebug (Value.VOk (inner))) (cat (text (" does not have type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (9)) then yes37 else no38)))
  | program, Value.VError inner, ty, _mode => (let yes39 : Except (ByteArray) (Unit) := valueOk (program) (inner) (tySecond (ty)) (0); (let no40 : Except (ByteArray) (Unit) := Except.error (cat (text ("the literal ")) (cat (valDebug (Value.VError (inner))) (cat (text (" does not have type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (9)) then yes39 else no40)))
  | program, Value.VList items, ty, _mode => (let yes41 : Except (ByteArray) (Unit) := valueOk (program) (items) (tyFirst (ty)) (1); (let no42 : Except (ByteArray) (Unit) := Except.error (cat (text ("the literal ")) (cat (valDebug (Value.VList (items))) (cat (text (" does not have type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (10)) then yes41 else no42)))
  | program, Value.VPair left right, ty, _mode => (let onLeft : Except (ByteArray) (Unit) := valueOk (program) (left) (tyFirst (ty)) (0); (let onRight : Except (ByteArray) (Unit) := valueOk (program) (right) (tySecond (ty)) (0); (let yes43 : Except (ByteArray) (Unit) := (match onLeft with | Except.ok _ => onRight | Except.error problem => Except.error (problem)); (let no44 : Except (ByteArray) (Unit) := Except.error (cat (text ("the literal ")) (cat (valDebug (Value.VPair (left) (right))) (cat (text (" does not have type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (11)) then yes43 else no44)))))
  | program, Value.VAdt constructor fields, ty, _mode => (let yes47 : Except (ByteArray) (Unit) := (match ctorFields (program) (tyAdtIndex (ty)) (constructor) with | Except.error problem => Except.error (problem) | Except.ok declared => (let inner : Except (ByteArray) (Unit) := valueOk (program) (fields) (declared) (2); (let yes45 : Except (ByteArray) (Unit) := inner; (let no46 : Except (ByteArray) (Unit) := Except.error (cat (text ("constructor ")) (cat (decimal (constructor)) (cat (text (" of ADT ")) (cat (decimal (tyAdtIndex (ty))) (cat (text (" takes ")) (cat (decimal (tyChainLength (declared))) (cat (text (" fields, the literal has ")) (decimal (valueChainLength (fields)))))))))); (if (Nat.beq (valueChainLength (fields)) (tyChainLength (declared))) then yes45 else no46))))); (let no48 : Except (ByteArray) (Unit) := Except.error (cat (text ("the literal ")) (cat (valDebug (Value.VAdt (constructor) (fields))) (cat (text (" does not have type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (12)) then yes47 else no48)))
  | _program, Value.VClosure _ _, _ty, _mode => Except.error (text ("a closure has no literal form; build it with `closure`"))
  | _program, Value.VNil, _ty, _mode => Except.ok (())
  | program, Value.VCons head tail, ty, mode => (let element : Ty := (if (Nat.beq (mode) (2)) then tyFirst (ty) else ty); (let later : Ty := (if (Nat.beq (mode) (2)) then tySecond (ty) else ty); (let onHead : Except (ByteArray) (Unit) := valueOk (program) (head) (element) (0); (let onTail : Except (ByteArray) (Unit) := valueOk (program) (tail) (later) (mode); (if (Nat.beq (mode) (0)) then Except.ok (()) else (if (if (Nat.beq (mode) (2)) then isNilTy (ty) else false) then Except.ok (()) else (match onHead with | Except.ok _ => onTail | Except.error problem => Except.error (problem))))))))

@[expose] public def shapeFields (program : Program) (shape : Shape) (ty : Ty) : Except (ByteArray) (Ty) := (match shape with | Shape.SNone => (let yes49 : Except (ByteArray) (Ty) := Except.ok (Ty.TNil); (let no50 : Except (ByteArray) (Ty) := Except.error (cat (text ("shape ")) (cat (shapeDebug (shape)) (cat (text (" does not belong to type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (8)) then yes49 else no50))) | Shape.SSome => (let yes51 : Except (ByteArray) (Ty) := Except.ok (Ty.TCons (tyFirst (ty)) (Ty.TNil)); (let no52 : Except (ByteArray) (Ty) := Except.error (cat (text ("shape ")) (cat (shapeDebug (shape)) (cat (text (" does not belong to type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (8)) then yes51 else no52))) | Shape.SOk => (let yes53 : Except (ByteArray) (Ty) := Except.ok (Ty.TCons (tyFirst (ty)) (Ty.TNil)); (let no54 : Except (ByteArray) (Ty) := Except.error (cat (text ("shape ")) (cat (shapeDebug (shape)) (cat (text (" does not belong to type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (9)) then yes53 else no54))) | Shape.SError => (let yes55 : Except (ByteArray) (Ty) := Except.ok (Ty.TCons (tySecond (ty)) (Ty.TNil)); (let no56 : Except (ByteArray) (Ty) := Except.error (cat (text ("shape ")) (cat (shapeDebug (shape)) (cat (text (" does not belong to type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (9)) then yes55 else no56))) | Shape.SNil => (let yes57 : Except (ByteArray) (Ty) := Except.ok (Ty.TNil); (let no58 : Except (ByteArray) (Ty) := Except.error (cat (text ("shape ")) (cat (shapeDebug (shape)) (cat (text (" does not belong to type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (10)) then yes57 else no58))) | Shape.SCons => (let yes59 : Except (ByteArray) (Ty) := Except.ok (Ty.TCons (tyFirst (ty)) (Ty.TCons (ty) (Ty.TNil))); (let no60 : Except (ByteArray) (Ty) := Except.error (cat (text ("shape ")) (cat (shapeDebug (shape)) (cat (text (" does not belong to type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (10)) then yes59 else no60))) | Shape.SZero => (let yes61 : Except (ByteArray) (Ty) := Except.ok (Ty.TNil); (let no62 : Except (ByteArray) (Ty) := Except.error (cat (text ("shape ")) (cat (shapeDebug (shape)) (cat (text (" does not belong to type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (2)) then yes61 else no62))) | Shape.SSucc => (let yes63 : Except (ByteArray) (Ty) := Except.ok (Ty.TCons (Ty.TNat) (Ty.TNil)); (let no64 : Except (ByteArray) (Ty) := Except.error (cat (text ("shape ")) (cat (shapeDebug (shape)) (cat (text (" does not belong to type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (2)) then yes63 else no64))) | Shape.SPair => (let yes65 : Except (ByteArray) (Ty) := Except.ok (Ty.TCons (tyFirst (ty)) (Ty.TCons (tySecond (ty)) (Ty.TNil))); (let no66 : Except (ByteArray) (Ty) := Except.error (cat (text ("shape ")) (cat (shapeDebug (shape)) (cat (text (" does not belong to type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (11)) then yes65 else no66))) | Shape.STrue => (let yes67 : Except (ByteArray) (Ty) := Except.ok (Ty.TNil); (let no68 : Except (ByteArray) (Ty) := Except.error (cat (text ("shape ")) (cat (shapeDebug (shape)) (cat (text (" does not belong to type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (1)) then yes67 else no68))) | Shape.SFalse => (let yes69 : Except (ByteArray) (Ty) := Except.ok (Ty.TNil); (let no70 : Except (ByteArray) (Ty) := Except.error (cat (text ("shape ")) (cat (shapeDebug (shape)) (cat (text (" does not belong to type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (1)) then yes69 else no70))) | Shape.SUnit => (let yes71 : Except (ByteArray) (Ty) := Except.ok (Ty.TNil); (let no72 : Except (ByteArray) (Ty) := Except.error (cat (text ("shape ")) (cat (shapeDebug (shape)) (cat (text (" does not belong to type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (0)) then yes71 else no72))) | Shape.SLt => (let yes73 : Except (ByteArray) (Ty) := Except.ok (Ty.TNil); (let no74 : Except (ByteArray) (Ty) := Except.error (cat (text ("shape ")) (cat (shapeDebug (shape)) (cat (text (" does not belong to type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (7)) then yes73 else no74))) | Shape.SEq => (let yes75 : Except (ByteArray) (Ty) := Except.ok (Ty.TNil); (let no76 : Except (ByteArray) (Ty) := Except.error (cat (text ("shape ")) (cat (shapeDebug (shape)) (cat (text (" does not belong to type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (7)) then yes75 else no76))) | Shape.SGt => (let yes77 : Except (ByteArray) (Ty) := Except.ok (Ty.TNil); (let no78 : Except (ByteArray) (Ty) := Except.error (cat (text ("shape ")) (cat (shapeDebug (shape)) (cat (text (" does not belong to type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (7)) then yes77 else no78))) | Shape.SAdt constructor => (let fields : Except (ByteArray) (Ty) := ctorFields (program) (tyAdtIndex (ty)) (constructor); (let yes79 : Except (ByteArray) (Ty) := fields; (let no80 : Except (ByteArray) (Ty) := Except.error (cat (text ("shape ")) (cat (shapeDebug (shape)) (cat (text (" does not belong to type ")) (tyDebug (ty))))); (if (Nat.beq (tyTag (ty)) (12)) then yes79 else no80)))))

@[expose] public def adtShapes : (constructors : Ty) -> (start : Nat) -> List (Shape)
  | Ty.TUnit, _start => ([] : List (Shape))
  | Ty.TBool, _start => ([] : List (Shape))
  | Ty.TNat, _start => ([] : List (Shape))
  | Ty.TInt, _start => ([] : List (Shape))
  | Ty.TFixed _, _start => ([] : List (Shape))
  | Ty.TString, _start => ([] : List (Shape))
  | Ty.TBytes, _start => ([] : List (Shape))
  | Ty.TOrdering, _start => ([] : List (Shape))
  | Ty.TOption _, _start => ([] : List (Shape))
  | Ty.TResult _ _, _start => ([] : List (Shape))
  | Ty.TList _, _start => ([] : List (Shape))
  | Ty.TPair _ _, _start => ([] : List (Shape))
  | Ty.TAdt _, _start => ([] : List (Shape))
  | Ty.TFn _ _, _start => ([] : List (Shape))
  | Ty.TNil, _start => ([] : List (Shape))
  | Ty.TCons _ tail, start => consShape (Shape.SAdt (start)) (adtShapes (tail) ((start + 1)))

@[expose] public def shapesOf (program : Program) (ty : Ty) : Except (ByteArray) (List (Shape)) := (match ty with | Ty.TUnit => Except.ok ((Shape.SUnit :: ([] : List (Shape)))) | Ty.TBool => Except.ok ((Shape.STrue :: (Shape.SFalse :: ([] : List (Shape))))) | Ty.TNat => Except.ok ((Shape.SZero :: (Shape.SSucc :: ([] : List (Shape))))) | Ty.TInt => Except.error (cat (text ("a value of type ")) (cat (tyDebug (ty)) (text (" cannot be matched")))) | Ty.TFixed _ => Except.error (cat (text ("a value of type ")) (cat (tyDebug (ty)) (text (" cannot be matched")))) | Ty.TString => Except.error (cat (text ("a value of type ")) (cat (tyDebug (ty)) (text (" cannot be matched")))) | Ty.TBytes => Except.error (cat (text ("a value of type ")) (cat (tyDebug (ty)) (text (" cannot be matched")))) | Ty.TOrdering => Except.ok ((Shape.SLt :: (Shape.SEq :: (Shape.SGt :: ([] : List (Shape)))))) | Ty.TOption _ => Except.ok ((Shape.SNone :: (Shape.SSome :: ([] : List (Shape))))) | Ty.TResult _ _ => Except.ok ((Shape.SOk :: (Shape.SError :: ([] : List (Shape))))) | Ty.TList _ => Except.ok ((Shape.SNil :: (Shape.SCons :: ([] : List (Shape))))) | Ty.TPair _ _ => Except.ok ((Shape.SPair :: ([] : List (Shape)))) | Ty.TAdt index => (match adtAt (program) (index) with | Option.none => Except.error (undeclaredAdt (index)) | Option.some declaration => Except.ok (adtShapes (declaration) (0))) | Ty.TFn _ _ => Except.error (cat (text ("a value of type ")) (cat (tyDebug (ty)) (text (" cannot be matched")))) | Ty.TNil => Except.error (cat (text ("a value of type ")) (cat (tyDebug (ty)) (text (" cannot be matched")))) | Ty.TCons _ _ => Except.error (cat (text ("a value of type ")) (cat (tyDebug (ty)) (text (" cannot be matched")))))

@[expose] public def firstMissing : (required : List (Shape)) -> (covered : List (Shape)) -> Option (Shape)
  | List.nil, _covered => Option.none
  | List.cons head tail, covered => (let later : Option (Shape) := firstMissing (tail) (covered); (let yes81 : Option (Shape) := later; (let no82 : Option (Shape) := Option.some (head); (if shapeIn (covered) (head) then yes81 else no82))))

@[expose] public def exactly (operation : Prim) (expected : Ty) (types : Ty) (result : Ty) : Except (ByteArray) (Ty) := (let yes83 : Except (ByteArray) (Ty) := Except.ok (result); (let no84 : Except (ByteArray) (Ty) := Except.error (cat (text ("primitive ")) (cat (primDebug (operation)) (cat (text (" takes ")) (cat (tyList (expected)) (cat (text (", received ")) (tyList (types))))))); (if tyEq (types) (expected) then yes83 else no84)))

@[expose] public def inapplicable (operation : Prim) (types : Ty) : Except (ByteArray) (Ty) := Except.error (cat (text ("primitive ")) (cat (primDebug (operation)) (cat (text (" does not apply to ")) (tyList (types)))))

@[expose] public def primType (operation : Prim) (types : Ty) : Except (ByteArray) (Ty) := (let headTy : Ty := tyFirst (types); (let lead : Ty := (if isNilTy (types) then Ty.TUnit else headTy); (match operation with | Prim.NatAdd => exactly (operation) (Ty.TCons (Ty.TNat) (Ty.TCons (Ty.TNat) (Ty.TNil))) (types) (Ty.TNat) | Prim.NatSub => exactly (operation) (Ty.TCons (Ty.TNat) (Ty.TCons (Ty.TNat) (Ty.TNil))) (types) (Ty.TNat) | Prim.NatMul => exactly (operation) (Ty.TCons (Ty.TNat) (Ty.TCons (Ty.TNat) (Ty.TNil))) (types) (Ty.TNat) | Prim.NatQuot => exactly (operation) (Ty.TCons (Ty.TNat) (Ty.TCons (Ty.TNat) (Ty.TCons (Ty.TNat) (Ty.TNil)))) (types) (Ty.TNat) | Prim.NatRem => exactly (operation) (Ty.TCons (Ty.TNat) (Ty.TCons (Ty.TNat) (Ty.TCons (Ty.TNat) (Ty.TNil)))) (types) (Ty.TNat) | Prim.NatEq => exactly (operation) (Ty.TCons (Ty.TNat) (Ty.TCons (Ty.TNat) (Ty.TNil))) (types) (Ty.TBool) | Prim.NatLe => exactly (operation) (Ty.TCons (Ty.TNat) (Ty.TCons (Ty.TNat) (Ty.TNil))) (types) (Ty.TBool) | Prim.NatLt => exactly (operation) (Ty.TCons (Ty.TNat) (Ty.TCons (Ty.TNat) (Ty.TNil))) (types) (Ty.TBool) | Prim.IntAdd => exactly (operation) (Ty.TCons (Ty.TInt) (Ty.TCons (Ty.TInt) (Ty.TNil))) (types) (Ty.TInt) | Prim.IntSub => exactly (operation) (Ty.TCons (Ty.TInt) (Ty.TCons (Ty.TInt) (Ty.TNil))) (types) (Ty.TInt) | Prim.IntMul => exactly (operation) (Ty.TCons (Ty.TInt) (Ty.TCons (Ty.TInt) (Ty.TNil))) (types) (Ty.TInt) | Prim.IntNeg => exactly (operation) (Ty.TCons (Ty.TInt) (Ty.TNil)) (types) (Ty.TInt) | Prim.IntQuot => exactly (operation) (Ty.TCons (Ty.TInt) (Ty.TCons (Ty.TInt) (Ty.TCons (Ty.TInt) (Ty.TNil)))) (types) (Ty.TInt) | Prim.IntRem => exactly (operation) (Ty.TCons (Ty.TInt) (Ty.TCons (Ty.TInt) (Ty.TCons (Ty.TInt) (Ty.TNil)))) (types) (Ty.TInt) | Prim.CheckedAdd => (let yes85 : Except (ByteArray) (Ty) := exactly (operation) (Ty.TCons (lead) (Ty.TCons (lead) (Ty.TNil))) (types) (Ty.TOption (lead)); (let no86 : Except (ByteArray) (Ty) := inapplicable (operation) (types); (if (Nat.beq (tyTag (lead)) (4)) then yes85 else no86))) | Prim.CheckedSub => (let yes87 : Except (ByteArray) (Ty) := exactly (operation) (Ty.TCons (lead) (Ty.TCons (lead) (Ty.TNil))) (types) (Ty.TOption (lead)); (let no88 : Except (ByteArray) (Ty) := inapplicable (operation) (types); (if (Nat.beq (tyTag (lead)) (4)) then yes87 else no88))) | Prim.CheckedMul => (let yes89 : Except (ByteArray) (Ty) := exactly (operation) (Ty.TCons (lead) (Ty.TCons (lead) (Ty.TNil))) (types) (Ty.TOption (lead)); (let no90 : Except (ByteArray) (Ty) := inapplicable (operation) (types); (if (Nat.beq (tyTag (lead)) (4)) then yes89 else no90))) | Prim.CheckedQuot => (let yes91 : Except (ByteArray) (Ty) := exactly (operation) (Ty.TCons (lead) (Ty.TCons (lead) (Ty.TNil))) (types) (Ty.TOption (lead)); (let no92 : Except (ByteArray) (Ty) := inapplicable (operation) (types); (if (Nat.beq (tyTag (lead)) (4)) then yes91 else no92))) | Prim.CheckedNeg => (let yes93 : Except (ByteArray) (Ty) := exactly (operation) (Ty.TCons (lead) (Ty.TNil)) (types) (Ty.TOption (lead)); (let no94 : Except (ByteArray) (Ty) := inapplicable (operation) (types); (if (if (Nat.beq (tyTag (lead)) (4)) then kindSigned (tyWidth (lead)) else false) then yes93 else no94))) | Prim.BitAnd => (let yes95 : Except (ByteArray) (Ty) := exactly (operation) (Ty.TCons (lead) (Ty.TCons (lead) (Ty.TNil))) (types) (lead); (let no96 : Except (ByteArray) (Ty) := inapplicable (operation) (types); (if (Nat.beq (tyTag (lead)) (4)) then yes95 else no96))) | Prim.BitOr => (let yes97 : Except (ByteArray) (Ty) := exactly (operation) (Ty.TCons (lead) (Ty.TCons (lead) (Ty.TNil))) (types) (lead); (let no98 : Except (ByteArray) (Ty) := inapplicable (operation) (types); (if (Nat.beq (tyTag (lead)) (4)) then yes97 else no98))) | Prim.BitXor => (let yes99 : Except (ByteArray) (Ty) := exactly (operation) (Ty.TCons (lead) (Ty.TCons (lead) (Ty.TNil))) (types) (lead); (let no100 : Except (ByteArray) (Ty) := inapplicable (operation) (types); (if (Nat.beq (tyTag (lead)) (4)) then yes99 else no100))) | Prim.BitNot => (let yes101 : Except (ByteArray) (Ty) := exactly (operation) (Ty.TCons (lead) (Ty.TNil)) (types) (lead); (let no102 : Except (ByteArray) (Ty) := inapplicable (operation) (types); (if (Nat.beq (tyTag (lead)) (4)) then yes101 else no102))) | Prim.ShiftLeft => (let yes103 : Except (ByteArray) (Ty) := exactly (operation) (Ty.TCons (lead) (Ty.TCons (Ty.TFixed (IntKind.U32)) (Ty.TNil))) (types) (Ty.TOption (lead)); (let no104 : Except (ByteArray) (Ty) := inapplicable (operation) (types); (if (Nat.beq (tyTag (lead)) (4)) then yes103 else no104))) | Prim.ShiftRight => (let yes105 : Except (ByteArray) (Ty) := exactly (operation) (Ty.TCons (lead) (Ty.TCons (Ty.TFixed (IntKind.U32)) (Ty.TNil))) (types) (Ty.TOption (lead)); (let no106 : Except (ByteArray) (Ty) := inapplicable (operation) (types); (if (Nat.beq (tyTag (lead)) (4)) then yes105 else no106))) | Prim.Equal => (let yes107 : Except (ByteArray) (Ty) := exactly (operation) (Ty.TCons (lead) (Ty.TCons (lead) (Ty.TNil))) (types) (Ty.TBool); (let no108 : Except (ByteArray) (Ty) := inapplicable (operation) (types); (if (if (Nat.beq (tyTag (lead)) (2)) then true else (if (Nat.beq (tyTag (lead)) (1)) then true else (if (Nat.beq (tyTag (lead)) (4)) then true else (if (Nat.beq (tyTag (lead)) (5)) then true else (if (Nat.beq (tyTag (lead)) (6)) then true else (if (Nat.beq (tyTag (lead)) (7)) then true else false)))))) then yes107 else no108))) | Prim.BoolNot => exactly (operation) (Ty.TCons (Ty.TBool) (Ty.TNil)) (types) (Ty.TBool) | Prim.BoolAnd => exactly (operation) (Ty.TCons (Ty.TBool) (Ty.TCons (Ty.TBool) (Ty.TNil))) (types) (Ty.TBool) | Prim.BoolOr => exactly (operation) (Ty.TCons (Ty.TBool) (Ty.TCons (Ty.TBool) (Ty.TNil))) (types) (Ty.TBool) | Prim.Append => (let yes109 : Except (ByteArray) (Ty) := exactly (operation) (Ty.TCons (lead) (Ty.TCons (lead) (Ty.TNil))) (types) (lead); (let no110 : Except (ByteArray) (Ty) := inapplicable (operation) (types); (if (if (Nat.beq (tyTag (lead)) (10)) then true else (if (Nat.beq (tyTag (lead)) (6)) then true else false)) then yes109 else no110))) | Prim.Length => (let yes111 : Except (ByteArray) (Ty) := exactly (operation) (Ty.TCons (lead) (Ty.TNil)) (types) (Ty.TNat); (let no112 : Except (ByteArray) (Ty) := inapplicable (operation) (types); (if (if (Nat.beq (tyTag (lead)) (10)) then true else (if (Nat.beq (tyTag (lead)) (6)) then true else (if (Nat.beq (tyTag (lead)) (5)) then true else false))) then yes111 else no112))) | Prim.Index => (let onList : Except (ByteArray) (Ty) := exactly (operation) (Ty.TCons (lead) (Ty.TCons (Ty.TNat) (Ty.TNil))) (types) (Ty.TOption (tyFirst (lead))); (let onBytes : Except (ByteArray) (Ty) := exactly (operation) (Ty.TCons (Ty.TBytes) (Ty.TCons (Ty.TNat) (Ty.TNil))) (types) (Ty.TOption (Ty.TFixed (IntKind.U8))); (if (Nat.beq (tyTag (lead)) (10)) then onList else (if (Nat.beq (tyTag (lead)) (6)) then onBytes else Except.error (cat (text ("primitive Index of ")) (tyDebug (lead))))))) | Prim.Slice => (let yes113 : Except (ByteArray) (Ty) := exactly (operation) (Ty.TCons (lead) (Ty.TCons (Ty.TNat) (Ty.TCons (Ty.TNat) (Ty.TNil)))) (types) (Ty.TOption (lead)); (let no114 : Except (ByteArray) (Ty) := inapplicable (operation) (types); (if (if (Nat.beq (tyTag (lead)) (10)) then true else (if (Nat.beq (tyTag (lead)) (6)) then true else false)) then yes113 else no114))) | Prim.Utf8Encode => exactly (operation) (Ty.TCons (Ty.TString) (Ty.TNil)) (types) (Ty.TBytes) | Prim.Utf8Decode => exactly (operation) (Ty.TCons (Ty.TBytes) (Ty.TNil)) (types) (Ty.TOption (Ty.TString)) | Prim.CompareBytes => exactly (operation) (Ty.TCons (Ty.TBytes) (Ty.TCons (Ty.TBytes) (Ty.TNil))) (types) (Ty.TOrdering) | Prim.SplitExact => exactly (operation) (Ty.TCons (Ty.TString) (Ty.TCons (Ty.TString) (Ty.TCons (Ty.TFixed (IntKind.U32)) (Ty.TNil)))) (types) (Ty.TOption (Ty.TList (Ty.TString))) | Prim.Join => exactly (operation) (Ty.TCons (Ty.TList (Ty.TString)) (Ty.TCons (Ty.TString) (Ty.TNil))) (types) (Ty.TString) | Prim.FormatDecimal => (let yes115 : Except (ByteArray) (Ty) := exactly (operation) (Ty.TCons (lead) (Ty.TNil)) (types) (Ty.TString); (let no116 : Except (ByteArray) (Ty) := inapplicable (operation) (types); (if (if (Nat.beq (tyTag (lead)) (3)) then true else (if (Nat.beq (tyTag (lead)) (4)) then true else false)) then yes115 else no116))) | Prim.Compare => (let yes117 : Except (ByteArray) (Ty) := exactly (operation) (Ty.TCons (lead) (Ty.TCons (lead) (Ty.TNil))) (types) (Ty.TOrdering); (let no118 : Except (ByteArray) (Ty) := inapplicable (operation) (types); (if orderable (lead) then yes117 else no118))) | Prim.Convert target => (let yes119 : Except (ByteArray) (Ty) := exactly (operation) (Ty.TCons (lead) (Ty.TNil)) (types) (Ty.TOption (Ty.TFixed (target))); (let no120 : Except (ByteArray) (Ty) := inapplicable (operation) (types); (if (if (Nat.beq (tyTag (lead)) (3)) then true else (if (Nat.beq (tyTag (lead)) (4)) then true else false)) then yes119 else no120))) | Prim.ParseDecimal target => (let targetTag : Nat := tyTag (target); (let yes121 : Except (ByteArray) (Ty) := exactly (operation) (Ty.TCons (Ty.TString) (Ty.TNil)) (types) (Ty.TOption (target)); (let no122 : Except (ByteArray) (Ty) := inapplicable (operation) (types); (if (if (Nat.beq (targetTag) (3)) then true else (Nat.beq (targetTag) (4))) then yes121 else no122)))))))

@[expose] public def extendScope (name : Nat) (ty : Ty) (scope : List ((Prod (Nat) (Ty)))) : List ((Prod (Nat) (Ty))) := ((name, ty) :: scope)

@[expose] public def scopeFind : (scope : List ((Prod (Nat) (Ty)))) -> (name : Nat) -> Option (Ty)
  | List.nil, _name => Option.none
  | List.cons head tail, name => (let later : Option (Ty) := scopeFind (tail) (name); (if (Nat.beq ((head).1) (name)) then Option.some ((head).2) else later))

@[expose] public def bindAll : (binders : List (Nat)) -> (tys : Ty) -> (scope : List ((Prod (Nat) (Ty)))) -> List ((Prod (Nat) (Ty)))
  | List.nil, _tys, scope => scope
  | List.cons binder more, tys, scope => (let deeper : List ((Prod (Nat) (Ty))) := bindAll (more) (tySecond (tys)) (extendScope (binder) (tyFirst (tys)) (scope)); (if isNilTy (tys) then scope else deeper))

@[expose] public def walk : (program : Program) -> (expr : Expr) -> (scope : List ((Prod (Nat) (Ty)))) -> (mode : Nat) -> (expected : Ty) -> (subject : Ty) -> (what : ByteArray) -> (position : Nat) -> (covered : List (Shape)) -> Outcome
  | program, Expr.EValue ty value, _scope, mode, _expected, _subject, _what, _position, _covered => (if (Nat.beq (mode) (0)) then (match tyOk (program) (ty) with | Except.ok _ => (match valueOk (program) (value) (ty) (0) with | Except.ok _ => Outcome.OTyped (ty) | Except.error problem => Outcome.OFailed (problem)) | Except.error problem => Outcome.OFailed (problem)) else Outcome.OFailed (text ("unreachable")))
  | _program, Expr.EVar name, scope, mode, _expected, _subject, _what, _position, _covered => (if (Nat.beq (mode) (0)) then (match scopeFind (scope) (name) with | Option.some found => Outcome.OTyped (found) | Option.none => Outcome.OFailed (cat (text ("local ")) (cat (decimal (name)) (text (" is unbound"))))) else Outcome.OFailed (text ("unreachable")))
  | program, Expr.ELet name ty bound body, scope, mode, _expected, _subject, _what, _position, _covered => (if (Nat.beq (mode) (0)) then (match tyOk (program) (ty) with | Except.ok _ => (match walk (program) (bound) (scope) (0) (Ty.TNil) (Ty.TNil) (text ("")) (0) (([] : List (Shape))) with | Outcome.OTyped actual => (match expectTy (actual) (ty) (cat (text ("the value bound to ")) (decimal (name))) with | Except.ok _ => walk (program) (body) (extendScope (name) (ty) (scope)) (0) (Ty.TNil) (Ty.TNil) (text ("")) (0) (([] : List (Shape))) | Except.error problem => Outcome.OFailed (problem)) | Outcome.OFailed problem => Outcome.OFailed (problem) | Outcome.OCovered _ => Outcome.OFailed (text ("unreachable")) | Outcome.OFine => Outcome.OFailed (text ("unreachable"))) | Except.error problem => Outcome.OFailed (problem)) else Outcome.OFailed (text ("unreachable")))
  | program, Expr.ECond condition thenBranch elseBranch, scope, mode, _expected, _subject, _what, _position, _covered => (if (Nat.beq (mode) (0)) then (match walk (program) (condition) (scope) (0) (Ty.TNil) (Ty.TNil) (text ("")) (0) (([] : List (Shape))) with | Outcome.OTyped conditionTy => (match expectTy (conditionTy) (Ty.TBool) (text ("a condition")) with | Except.ok _ => (match walk (program) (thenBranch) (scope) (0) (Ty.TNil) (Ty.TNil) (text ("")) (0) (([] : List (Shape))) with | Outcome.OTyped thenTy => (match walk (program) (elseBranch) (scope) (0) (Ty.TNil) (Ty.TNil) (text ("")) (0) (([] : List (Shape))) with | Outcome.OTyped elseTy => (match expectTy (elseTy) (thenTy) (text ("the else branch")) with | Except.ok _ => Outcome.OTyped (thenTy) | Except.error problem => Outcome.OFailed (problem)) | Outcome.OFailed problem => Outcome.OFailed (problem) | Outcome.OCovered _ => Outcome.OFailed (text ("unreachable")) | Outcome.OFine => Outcome.OFailed (text ("unreachable"))) | Outcome.OFailed problem => Outcome.OFailed (problem) | Outcome.OCovered _ => Outcome.OFailed (text ("unreachable")) | Outcome.OFine => Outcome.OFailed (text ("unreachable"))) | Except.error problem => Outcome.OFailed (problem)) | Outcome.OFailed problem => Outcome.OFailed (problem) | Outcome.OCovered _ => Outcome.OFailed (text ("unreachable")) | Outcome.OFine => Outcome.OFailed (text ("unreachable"))) else Outcome.OFailed (text ("unreachable")))
  | program, Expr.EMatch ty scrutinee arms, scope, mode, _expected, _subject, _what, _position, _covered => (if (Nat.beq (mode) (0)) then (match tyOk (program) (ty) with | Except.ok _ => (match walk (program) (scrutinee) (scope) (0) (Ty.TNil) (Ty.TNil) (text ("")) (0) (([] : List (Shape))) with | Outcome.OTyped scrutineeTy => (match shapesOf (program) (scrutineeTy) with | Except.ok required => (match walk (program) (arms) (scope) (4) (ty) (scrutineeTy) (text ("")) (0) (([] : List (Shape))) with | Outcome.OCovered seen => (match firstMissing (required) (seen) with | Option.some missing => Outcome.OFailed (cat (text ("the match is not exhaustive: ")) (cat (shapeDebug (missing)) (text (" is not covered")))) | Option.none => Outcome.OTyped (ty)) | Outcome.OFailed problem => Outcome.OFailed (problem) | Outcome.OTyped _ => Outcome.OFailed (text ("unreachable")) | Outcome.OFine => Outcome.OFailed (text ("unreachable"))) | Except.error problem => Outcome.OFailed (problem)) | Outcome.OFailed problem => Outcome.OFailed (problem) | Outcome.OCovered _ => Outcome.OFailed (text ("unreachable")) | Outcome.OFine => Outcome.OFailed (text ("unreachable"))) | Except.error problem => Outcome.OFailed (problem)) else Outcome.OFailed (text ("unreachable")))
  | program, Expr.EBuild shape ty operands, scope, mode, _expected, _subject, _what, _position, _covered => (if (Nat.beq (mode) (0)) then (match tyOk (program) (ty) with | Except.ok _ => (match shapeFields (program) (shape) (ty) with | Except.ok fields => (let what130 : ByteArray := cat (text ("constructor ")) (shapeDebug (shape)); (if (Nat.beq (exprChainLength (operands)) (tyChainLength (fields))) then (match walk (program) (operands) (scope) (2) (fields) (Ty.TNil) (what130) (0) (([] : List (Shape))) with | Outcome.OFine => Outcome.OTyped (ty) | Outcome.OFailed problem => Outcome.OFailed (problem) | Outcome.OTyped _ => Outcome.OFailed (text ("unreachable")) | Outcome.OCovered _ => Outcome.OFailed (text ("unreachable"))) else Outcome.OFailed (cat (what130) (cat (text (" takes ")) (cat (decimal (tyChainLength (fields))) (cat (text (" operand(s), received ")) (decimal (exprChainLength (operands))))))))) | Except.error problem => Outcome.OFailed (problem)) | Except.error problem => Outcome.OFailed (problem)) else Outcome.OFailed (text ("unreachable")))
  | program, Expr.ECall function operands, scope, mode, _expected, _subject, _what, _position, _covered => (if (Nat.beq (mode) (0)) then (match functionTypes (program) (function) with | Option.none => Outcome.OFailed (undeclaredFunction (function)) | Option.some declared => (let what132 : ByteArray := cat (text ("function ")) (decimal (function)); (if (Nat.beq (exprChainLength (operands)) (tyChainLength (declared))) then (match walk (program) (operands) (scope) (2) (declared) (Ty.TNil) (what132) (0) (([] : List (Shape))) with | Outcome.OFine => Outcome.OTyped (functionResult (program) (function)) | Outcome.OFailed problem => Outcome.OFailed (problem) | Outcome.OTyped _ => Outcome.OFailed (text ("unreachable")) | Outcome.OCovered _ => Outcome.OFailed (text ("unreachable"))) else Outcome.OFailed (cat (what132) (cat (text (" takes ")) (cat (decimal (tyChainLength (declared))) (cat (text (" operand(s), received ")) (decimal (exprChainLength (operands)))))))))) else Outcome.OFailed (text ("unreachable")))
  | program, Expr.EClosure function captures, scope, mode, _expected, _subject, _what, _position, _covered => (if (Nat.beq (mode) (0)) then (match functionTypes (program) (function) with | Option.none => Outcome.OFailed (undeclaredFunction (function)) | Option.some declared => (let total : Nat := tyChainLength (declared); (let given : Nat := exprChainLength (captures); (if (Nat.ble (total) (given)) then Outcome.OFailed (cat (text ("closure of function ")) (cat (decimal (function)) (cat (text (" captures ")) (cat (decimal (given)) (cat (text (" of its ")) (cat (decimal (total)) (text (" parameters; at least one must remain")))))))) else (match walk (program) (captures) (scope) (3) (declared) (Ty.TNil) (cat (text ("closure of function ")) (decimal (function))) (0) (([] : List (Shape))) with | Outcome.OTyped remaining => Outcome.OTyped (closureTy (remaining) (functionResult (program) (function))) | Outcome.OFailed problem => Outcome.OFailed (problem) | Outcome.OCovered _ => Outcome.OFailed (text ("unreachable")) | Outcome.OFine => Outcome.OFailed (text ("unreachable"))))))) else Outcome.OFailed (text ("unreachable")))
  | program, Expr.EApply target operands, scope, mode, _expected, _subject, _what, _position, _covered => (if (Nat.beq (mode) (0)) then (match walk (program) (target) (scope) (0) (Ty.TNil) (Ty.TNil) (text ("")) (0) (([] : List (Shape))) with | Outcome.OTyped targetTy => (if (Nat.beq (tyTag (targetTy)) (13)) then (let what133 : ByteArray := text ("an application"); (if (Nat.beq (exprChainLength (operands)) (tyChainLength (tyFirst (targetTy)))) then (match walk (program) (operands) (scope) (2) (tyFirst (targetTy)) (Ty.TNil) (what133) (0) (([] : List (Shape))) with | Outcome.OFine => Outcome.OTyped (tySecond (targetTy)) | Outcome.OFailed problem => Outcome.OFailed (problem) | Outcome.OTyped _ => Outcome.OFailed (text ("unreachable")) | Outcome.OCovered _ => Outcome.OFailed (text ("unreachable"))) else Outcome.OFailed (cat (what133) (cat (text (" takes ")) (cat (decimal (tyChainLength (tyFirst (targetTy)))) (cat (text (" operand(s), received ")) (decimal (exprChainLength (operands))))))))) else Outcome.OFailed (text ("only a function value can be applied"))) | Outcome.OFailed problem => Outcome.OFailed (problem) | Outcome.OCovered _ => Outcome.OFailed (text ("unreachable")) | Outcome.OFine => Outcome.OFailed (text ("unreachable"))) else Outcome.OFailed (text ("unreachable")))
  | program, Expr.EPrim operation operands, scope, mode, _expected, _subject, _what, _position, _covered => (if (Nat.beq (mode) (0)) then (match walk (program) (operands) (scope) (1) (Ty.TNil) (Ty.TNil) (text ("")) (0) (([] : List (Shape))) with | Outcome.OTyped types => (match primType (operation) (types) with | Except.ok result => Outcome.OTyped (result) | Except.error problem => Outcome.OFailed (problem)) | Outcome.OFailed problem => Outcome.OFailed (problem) | Outcome.OCovered _ => Outcome.OFailed (text ("unreachable")) | Outcome.OFine => Outcome.OFailed (text ("unreachable"))) else Outcome.OFailed (text ("unreachable")))
  | program, Expr.EFirst value, scope, mode, _expected, _subject, _what, _position, _covered => (if (Nat.beq (mode) (0)) then (match walk (program) (value) (scope) (0) (Ty.TNil) (Ty.TNil) (text ("")) (0) (([] : List (Shape))) with | Outcome.OTyped valueTy => (if (Nat.beq (tyTag (valueTy)) (11)) then Outcome.OTyped (tyFirst (valueTy)) else Outcome.OFailed (cat (text ("`first` of ")) (tyDebug (valueTy)))) | Outcome.OFailed problem => Outcome.OFailed (problem) | Outcome.OCovered _ => Outcome.OFailed (text ("unreachable")) | Outcome.OFine => Outcome.OFailed (text ("unreachable"))) else Outcome.OFailed (text ("unreachable")))
  | program, Expr.ESecond value, scope, mode, _expected, _subject, _what, _position, _covered => (if (Nat.beq (mode) (0)) then (match walk (program) (value) (scope) (0) (Ty.TNil) (Ty.TNil) (text ("")) (0) (([] : List (Shape))) with | Outcome.OTyped valueTy => (if (Nat.beq (tyTag (valueTy)) (11)) then Outcome.OTyped (tySecond (valueTy)) else Outcome.OFailed (cat (text ("`second` of ")) (tyDebug (valueTy)))) | Outcome.OFailed problem => Outcome.OFailed (problem) | Outcome.OCovered _ => Outcome.OFailed (text ("unreachable")) | Outcome.OFine => Outcome.OFailed (text ("unreachable"))) else Outcome.OFailed (text ("unreachable")))
  | program, Expr.EField value index, scope, mode, _expected, _subject, _what, _position, _covered => (if (Nat.beq (mode) (0)) then (match walk (program) (value) (scope) (0) (Ty.TNil) (Ty.TNil) (text ("")) (0) (([] : List (Shape))) with | Outcome.OTyped valueTy => (if (Nat.beq (tyTag (valueTy)) (12)) then (let adt : Nat := tyAdtIndex (valueTy); (match adtAt (program) (adt) with | Option.none => Outcome.OFailed (undeclaredAdt (adt)) | Option.some constructors => (if (Nat.beq (tyChainLength (constructors)) (1)) then (match tyChainAt (constructors) (0) with | Option.some only => (match tyChainAt (only) (index) with | Option.some found => Outcome.OTyped (found) | Option.none => Outcome.OFailed (cat (text ("ADT ")) (cat (decimal (adt)) (cat (text (" has no field ")) (decimal (index)))))) | Option.none => Outcome.OFailed (text ("unreachable"))) else Outcome.OFailed (cat (text ("`field` requires an ADT with exactly one constructor; ADT ")) (cat (decimal (adt)) (cat (text (" has ")) (decimal (tyChainLength (constructors))))))))) else Outcome.OFailed (text ("`field` requires an ADT value"))) | Outcome.OFailed problem => Outcome.OFailed (problem) | Outcome.OCovered _ => Outcome.OFailed (text ("unreachable")) | Outcome.OFine => Outcome.OFailed (text ("unreachable"))) else Outcome.OFailed (text ("unreachable")))
  | _program, Expr.ENil, _scope, mode, expected, _subject, _what, _position, covered => (if (Nat.beq (mode) (1)) then Outcome.OTyped (Ty.TNil) else (if (Nat.beq (mode) (2)) then Outcome.OFine else (if (Nat.beq (mode) (3)) then Outcome.OTyped (expected) else (if (Nat.beq (mode) (4)) then Outcome.OCovered (covered) else Outcome.OFailed (text ("unreachable"))))))
  | program, Expr.ECons head tail, scope, mode, expected, subject, what, position, covered => (if (Nat.beq (mode) (1)) then (match walk (program) (head) (scope) (0) (Ty.TNil) (Ty.TNil) (text ("")) (0) (([] : List (Shape))) with | Outcome.OTyped one => (match walk (program) (tail) (scope) (1) (Ty.TNil) (Ty.TNil) (text ("")) (0) (([] : List (Shape))) with | Outcome.OTyped rest => Outcome.OTyped (consTy (one) (rest)) | Outcome.OFailed problem => Outcome.OFailed (problem) | Outcome.OCovered _ => Outcome.OFailed (text ("unreachable")) | Outcome.OFine => Outcome.OFailed (text ("unreachable"))) | Outcome.OFailed problem => Outcome.OFailed (problem) | Outcome.OCovered _ => Outcome.OFailed (text ("unreachable")) | Outcome.OFine => Outcome.OFailed (text ("unreachable"))) else (if (Nat.beq (mode) (2)) then (if isNilTy (expected) then Outcome.OFine else (match walk (program) (head) (scope) (0) (Ty.TNil) (Ty.TNil) (text ("")) (0) (([] : List (Shape))) with | Outcome.OTyped actual => (match expectTy (actual) (tyFirst (expected)) (cat (what) (cat (text (" operand ")) (decimal (position)))) with | Except.ok _ => walk (program) (tail) (scope) (2) (tySecond (expected)) (Ty.TNil) (what) ((position + 1)) (([] : List (Shape))) | Except.error problem => Outcome.OFailed (problem)) | Outcome.OFailed problem => Outcome.OFailed (problem) | Outcome.OCovered _ => Outcome.OFailed (text ("unreachable")) | Outcome.OFine => Outcome.OFailed (text ("unreachable")))) else (if (Nat.beq (mode) (3)) then (if isNilTy (expected) then Outcome.OTyped (Ty.TNil) else (match walk (program) (head) (scope) (0) (Ty.TNil) (Ty.TNil) (text ("")) (0) (([] : List (Shape))) with | Outcome.OTyped actual => (match expectTy (actual) (tyFirst (expected)) (cat (what) (cat (text (" operand ")) (decimal (position)))) with | Except.ok _ => walk (program) (tail) (scope) (3) (tySecond (expected)) (Ty.TNil) (what) ((position + 1)) (([] : List (Shape))) | Except.error problem => Outcome.OFailed (problem)) | Outcome.OFailed problem => Outcome.OFailed (problem) | Outcome.OCovered _ => Outcome.OFailed (text ("unreachable")) | Outcome.OFine => Outcome.OFailed (text ("unreachable")))) else (if (Nat.beq (mode) (4)) then (match walk (program) (head) (scope) (5) (expected) (subject) (text ("")) (0) (covered) with | Outcome.OCovered updated => walk (program) (tail) (scope) (4) (expected) (subject) (text ("")) (0) (updated) | Outcome.OFailed problem => Outcome.OFailed (problem) | Outcome.OTyped _ => Outcome.OFailed (text ("unreachable")) | Outcome.OFine => Outcome.OFailed (text ("unreachable"))) else Outcome.OFailed (text ("unreachable"))))))
  | program, Expr.EArm shape binders body, scope, mode, expected, subject, _what, _position, covered => (if (Nat.beq (mode) (5)) then (match shapeFields (program) (shape) (subject) with | Except.ok fields => (let named : Nat := listLength (Nat) (binders); (let bound : Nat := tyChainLength (fields); (if (Nat.beq (named) (bound)) then (if hasRepeat (binders) then Outcome.OFailed (cat (text ("shape ")) (cat (shapeDebug (shape)) (text (" binds the same name twice")))) else (if shapeIn (covered) (shape) then Outcome.OFailed (cat (text ("the arm for ")) (cat (shapeDebug (shape)) (text (" is unreachable")))) else (match walk (program) (body) (bindAll (binders) (fields) (scope)) (0) (Ty.TNil) (Ty.TNil) (text ("")) (0) (([] : List (Shape))) with | Outcome.OTyped bodyTy => (match expectTy (bodyTy) (expected) (text ("an arm")) with | Except.ok _ => Outcome.OCovered (consShape (shape) (covered)) | Except.error problem => Outcome.OFailed (problem)) | Outcome.OFailed problem => Outcome.OFailed (problem) | Outcome.OCovered _ => Outcome.OFailed (text ("unreachable")) | Outcome.OFine => Outcome.OFailed (text ("unreachable"))))) else Outcome.OFailed (cat (text ("shape ")) (cat (shapeDebug (shape)) (cat (text (" binds ")) (cat (decimal (bound)) (cat (text (" value(s), the arm names ")) (decimal (named)))))))))) | Except.error problem => Outcome.OFailed (problem)) else Outcome.OFailed (text ("unreachable")))

@[expose] public def checkCtors : (program : Program) -> (constructors : Ty) -> Except (ByteArray) (Unit)
  | _program, Ty.TUnit => Except.ok (())
  | _program, Ty.TBool => Except.ok (())
  | _program, Ty.TNat => Except.ok (())
  | _program, Ty.TInt => Except.ok (())
  | _program, Ty.TFixed _ => Except.ok (())
  | _program, Ty.TString => Except.ok (())
  | _program, Ty.TBytes => Except.ok (())
  | _program, Ty.TOrdering => Except.ok (())
  | _program, Ty.TOption _ => Except.ok (())
  | _program, Ty.TResult _ _ => Except.ok (())
  | _program, Ty.TList _ => Except.ok (())
  | _program, Ty.TPair _ _ => Except.ok (())
  | _program, Ty.TAdt _ => Except.ok (())
  | _program, Ty.TFn _ _ => Except.ok (())
  | _program, Ty.TNil => Except.ok (())
  | program, Ty.TCons head tail => (match tyOk (program) (head) with | Except.ok _ => checkCtors (program) (tail) | Except.error problem => Except.error (problem))

@[expose] public def checkAdts : (program : Program) -> (adts : Ty) -> (index : Nat) -> Except (ByteArray) (Unit)
  | _program, Ty.TUnit, _index => Except.ok (())
  | _program, Ty.TBool, _index => Except.ok (())
  | _program, Ty.TNat, _index => Except.ok (())
  | _program, Ty.TInt, _index => Except.ok (())
  | _program, Ty.TFixed _, _index => Except.ok (())
  | _program, Ty.TString, _index => Except.ok (())
  | _program, Ty.TBytes, _index => Except.ok (())
  | _program, Ty.TOrdering, _index => Except.ok (())
  | _program, Ty.TOption _, _index => Except.ok (())
  | _program, Ty.TResult _ _, _index => Except.ok (())
  | _program, Ty.TList _, _index => Except.ok (())
  | _program, Ty.TPair _ _, _index => Except.ok (())
  | _program, Ty.TAdt _, _index => Except.ok (())
  | _program, Ty.TFn _ _, _index => Except.ok (())
  | _program, Ty.TNil, _index => Except.ok (())
  | program, Ty.TCons declaration tail, index => (if isNilTy (declaration) then Except.error (cat (text ("ADT ")) (cat (decimal (index)) (text (" has no constructor")))) else (match checkCtors (program) (declaration) with | Except.ok _ => checkAdts (program) (tail) ((index + 1)) | Except.error problem => Except.error (cat (text ("ADT ")) (cat (decimal (index)) (cat (text (": ")) (problem))))))

@[expose] public def functionNames (program : Program) (position : Nat) : List (Nat) := (match listAt (List (Nat)) ((program).parameters) (position) with | Option.some found => found | Option.none => ([] : List (Nat)))

@[expose] public def functionTypesOr (program : Program) (position : Nat) : Ty := (match functionTypes (program) (position) with | Option.some found => found | Option.none => Ty.TNil)

@[expose] public def checkFunction (program : Program) (body : Expr) (index : Nat) : Except (ByteArray) (Unit) := (let names : List (Nat) := functionNames (program) (index); (let types : Ty := functionTypesOr (program) (index); (let result : Ty := functionResult (program) (index); (let named : Nat := listLength (Nat) (names); (let typed : Nat := tyChainLength (types); (if (Nat.beq (named) (typed)) then (if hasRepeat (names) then Except.error (cat (text ("function ")) (cat (decimal (index)) (cat (text (": ")) (text ("a parameter name repeats"))))) else (match tyOk (program) (types) with | Except.error problem => Except.error (cat (text ("function ")) (cat (decimal (index)) (cat (text (": ")) (problem)))) | Except.ok _ => (match tyOk (program) (result) with | Except.error problem => Except.error (cat (text ("function ")) (cat (decimal (index)) (cat (text (": ")) (problem)))) | Except.ok _ => (match walk (program) (body) (bindAll (names) (types) (([] : List ((Prod (Nat) (Ty)))))) (0) (Ty.TNil) (Ty.TNil) (text ("")) (0) (([] : List (Shape))) with | Outcome.OFailed problem => Except.error (cat (text ("function ")) (cat (decimal (index)) (cat (text (": ")) (problem)))) | Outcome.OFine => Except.error (text ("unreachable")) | Outcome.OCovered _ => Except.error (text ("unreachable")) | Outcome.OTyped bodyTy => (match expectTy (bodyTy) (result) (text ("the body")) with | Except.error problem => Except.error (cat (text ("function ")) (cat (decimal (index)) (cat (text (": ")) (problem)))) | Except.ok _ => Except.ok (())))))) else Except.error (cat (text ("function ")) (cat (decimal (index)) (cat (text (": ")) (cat (decimal (named)) (cat (text (" parameter names for ")) (cat (decimal (typed)) (text (" parameter types"))))))))))))))

@[expose] public def checkFunctions : (program : Program) -> (bodies : Expr) -> (index : Nat) -> Except (ByteArray) (Unit)
  | _program, Expr.ENil, _index => Except.ok (())
  | program, Expr.ECons head tail, index => (match checkFunction (program) (head) (index) with | Except.ok _ => checkFunctions (program) (tail) ((index + 1)) | Except.error problem => Except.error (problem))
  | _program, Expr.EValue _ _, _index => Except.ok (())
  | _program, Expr.EVar _, _index => Except.ok (())
  | _program, Expr.ELet _ _ _ _, _index => Except.ok (())
  | _program, Expr.ECond _ _ _, _index => Except.ok (())
  | _program, Expr.EMatch _ _ _, _index => Except.ok (())
  | _program, Expr.EBuild _ _ _, _index => Except.ok (())
  | _program, Expr.ECall _ _, _index => Except.ok (())
  | _program, Expr.EClosure _ _, _index => Except.ok (())
  | _program, Expr.EApply _ _, _index => Except.ok (())
  | _program, Expr.EPrim _ _, _index => Except.ok (())
  | _program, Expr.EFirst _, _index => Except.ok (())
  | _program, Expr.ESecond _, _index => Except.ok (())
  | _program, Expr.EField _ _, _index => Except.ok (())
  | _program, Expr.EArm _ _ _, _index => Except.ok (())

@[expose] public def checkProgram (spec : ByteArray) (adts : Ty) (parameters : List (List (Nat))) (types : Ty) (results : Ty) (bodies : Expr) : Except (ByteArray) (Unit) := (let program : Program := ({ spec := spec, adts := adts, parameters := parameters, types := types, results := results, bodies := bodies } : Program); (if (LexLeanRuntime.equal ((program).spec) (text ("lexlean/target-program/1")) : Bool) then (match checkAdts (program) ((program).adts) (0) with | Except.ok _ => checkFunctions (program) ((program).bodies) (0) | Except.error problem => Except.error (problem)) else Except.error (cat (text ("target program has spec ")) (quoted ((program).spec)))))

@[expose] public def checkArguments (spec : ByteArray) (adts : Ty) (parameters : List (List (Nat))) (types : Ty) (results : Ty) (bodies : Expr) (entry : Nat) (arguments : Value) : Except (ByteArray) (Unit) := (let program : Program := ({ spec := spec, adts := adts, parameters := parameters, types := types, results := results, bodies := bodies } : Program); (match functionTypes (program) (entry) with | Option.none => Except.error (undeclaredFunction (entry)) | Option.some declared => (let yes137 : Except (ByteArray) (Unit) := valueOk (program) (arguments) (declared) (2); (let no138 : Except (ByteArray) (Unit) := Except.error (cat (text ("function ")) (cat (decimal (entry)) (cat (text (" takes ")) (cat (decimal (tyChainLength (declared))) (cat (text (" argument(s), received ")) (decimal (valueChainLength (arguments)))))))); (if (Nat.beq (valueChainLength (arguments)) (tyChainLength (declared))) then yes137 else no138)))))

end LexLeanBoot.CalculusCheck
