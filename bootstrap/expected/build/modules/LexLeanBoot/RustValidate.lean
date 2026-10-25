module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Text
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.RustValidate

public inductive IntKind where
  | U8
  | U16
  | U32
  | U64
  | I8
  | I16
  | I32
  | I64

public structure Origin where
  element : ByteArray
  width : Option (IntKind)

public structure Rt where
  family : Nat
  width : Option (IntKind)

public inductive Id where
  | IdLocal (_ : Nat)
  | IdHolder (_ : Nat)
  | IdOperand (_ : Nat)
  | IdCallee (_ : Nat)
  | IdBoxed (_ : Nat)
  | IdPart (_ : Nat)
  | IdCapture (_ : Nat)
  | IdParam (_ : Nat)
  | IdFunction (_ : Nat)
  | IdExport (_ : ByteArray)

public inductive LeafKind where
  | LkUnit
  | LkBool
  | LkNat
  | LkInt
  | LkOrdering
  | LkStr
  | LkBytes
  | LkOverflow
  | LkI128

public inductive UnaryKind where
  | UkOption
  | UkList
  | UkRc
  | UkFallible
  | UkRef

public inductive BinaryKind where
  | BkResult
  | BkPair

public inductive NameKind where
  | NkAdt
  | NkFn
  | NkParam

public inductive Ty where
  | TyLeaf (_ : LeafKind)
  | TyFixed (_ : IntKind)
  | TyUnary (_ : UnaryKind) (_ : Ty)
  | TyBinary (_ : BinaryKind) (_ : Ty) (_ : Ty)
  | TyNamed (_ : NameKind) (_ : Nat)
  | TyNil
  | TyCons (_ : Ty) (_ : Ty)

public inductive Lit where
  | LitUnit
  | LitBool
  | LitNat
  | LitInt
  | LitFixed (_ : IntKind)
  | LitStr
  | LitBytes
  | LitOrdering
  | LitOverflow

public inductive Con where
  | ConNone (_ : Ty)
  | ConSome
  | ConOk (_ : Ty) (_ : Ty)
  | ConErr (_ : Ty) (_ : Ty)
  | ConAdt (_ : Nat) (_ : Nat)
  | ConClosure (_ : Nat) (_ : Nat)
  | ConCons
  | ConNil (_ : Ty)

public inductive PatLeaf where
  | PlWild
  | PlUnit
  | PlNone
  | PlOrdering

public inductive PatWrap where
  | PwSome
  | PwOk
  | PwErr

public inductive Pat where
  | PLeaf (_ : PatLeaf)
  | PBind (_ : Id)
  | PTuple (_ : Pat)
  | PWrap (_ : PatWrap) (_ : Pat)
  | PAdt (_ : Nat) (_ : Nat) (_ : Pat)
  | PNil
  | PCons (_ : Pat) (_ : Pat)

public inductive Callee where
  | CalFn (_ : Nat)
  | CalRt (_ : Rt)
  | CalMachine (_ : Nat)

public inductive Cap where
  | CapCopy
  | CapClone
  | CapUnbox
  | CapUnit

public inductive ReadKind where
  | RkMove
  | RkClone
  | RkCopy
  | RkDeref
  | RkUnbox

public inductive WrapKind where
  | WkNot
  | WkBox
  | WkWiden
  | WkSucceed

public inductive ProbeKind where
  | PkIsZero
  | PkNonZero
  | PkUncons
  | PkPred

public inductive Expr where
  | ELit (_ : Lit) (_ : Origin)
  | ERead (_ : ReadKind) (_ : Id) (_ : Origin)
  | EWrap (_ : WrapKind) (_ : Expr) (_ : Origin)
  | ECall (_ : Callee) (_ : Expr) (_ : Bool) (_ : Origin)
  | EApply (_ : Id) (_ : Expr) (_ : Bool) (_ : Origin)
  | EConstruct (_ : Con) (_ : Expr) (_ : Origin)
  | EPair (_ : Expr) (_ : Expr) (_ : Origin)
  | EIf (_ : Expr) (_ : Expr) (_ : Expr) (_ : Origin)
  | EMatch (_ : Expr) (_ : Expr) (_ : Origin)
  | EBlockE (_ : Expr)
  | EProbe (_ : ProbeKind) (_ : Id) (_ : Origin)
  | EBlock (_ : Expr) (_ : Expr)
  | ENil
  | ECons (_ : Expr) (_ : Expr)
  | ELet (_ : Pat) (_ : Bool) (_ : Ty) (_ : Expr) (_ : Origin) (_ : Expr)
  | EArm (_ : Pat) (_ : Expr) (_ : Expr)
  | ELoop (_ : Id) (_ : Expr)

public inductive Row where
  | RowNil
  | RowCons (_ : Nat) (_ : Ty) (_ : Row)

public inductive Dis where
  | DisNil
  | DisCons (_ : Nat) (_ : List (Cap)) (_ : Bool) (_ : Dis)

public inductive Par where
  | ParNil
  | ParCons (_ : Pat) (_ : Ty) (_ : Par)

public inductive Def where
  | DEnum (_ : Ty) (_ : Row) (_ : Origin)
  | DApply (_ : Nat) (_ : Ty) (_ : Ty) (_ : Bool) (_ : Dis) (_ : Origin)
  | DFunction (_ : Id) (_ : Par) (_ : Ty) (_ : Expr) (_ : Origin)
  | DNil
  | DCons (_ : Def) (_ : Def)

public structure Crate where
  core : Bool
  items : Def

public structure Decl where
  adts : List ((Prod (Nat) (List ((Prod (Nat) (Nat))))))
  closures : List ((Prod (Nat) (List ((Prod (Nat) (Nat))))))
  applies : List ((Prod (Nat) (Bool)))
  functions : List ((Prod (Nat) (Bool)))

public structure St where
  bound : List (Id)
  scope : List (Id)
  moved : List (Id)
  parted : List ((Prod (Id) (List (Nat))))
  propagates : Bool

public structure ItemRow where
  family : Nat
  stem : ByteArray
  form : Nat
  fallible : Bool
  heap : Bool

public structure Cx where
  core : Bool
  decl : Decl
  fallible : Bool
  items : List (ItemRow)

public structure Inst where
  construct : ByteArray
  element : ByteArray
  originWidth : Option (IntKind)
  width : Option (IntKind)

public structure Found where
  insts : List (Inst)
  types : List (ByteArray)
  items : List (ItemRow)

@[expose] public def text (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def cat (left : ByteArray) (right : ByteArray) : ByteArray := (LexLeanRuntime.append (left) (right) : ByteArray)

@[expose] public def tick (name : ByteArray) : ByteArray := cat (text ("`")) (cat (name) (text ("`")))

@[expose] public def byteIs (subject : ByteArray) (position : Nat) (value : UInt8) : Bool := (match LexLeanStd.Text.byteAt (subject) (position) with | Option.some byte => (LexLeanRuntime.equal (byte) (value) : Bool) | Option.none => false)

@[expose] public def zeroStep (subject : ByteArray) (position : Nat) : Option (Nat) := (if byteIs (subject) (position) ((48 : UInt8)) then Option.some ((position + 1)) else Option.none)

@[expose] public def significant (digits : ByteArray) : ByteArray := (let zeros : Nat := ((LexLeanCollections.iterateUntil ((fun (position : Nat) => zeroStep (digits) (position))) ((LexLeanRuntime.length (digits) : Nat)) (0) : (Prod (Nat) (Bool)))).1; LexLeanStd.Text.sliceOr (digits) (zeros) ((LexLeanRuntime.subtract ((LexLeanRuntime.length (digits) : Nat)) (zeros) : Nat)))

@[expose] public def digit (number : Nat) (place : Nat) : ByteArray := LexLeanStd.Text.sliceOr (text ("0123456789")) ((LexLeanRuntime.remainder ((LexLeanRuntime.quotient (number) (place) (0) : Nat)) (10) (0) : Nat)) (1)

@[expose] public def decimal (number : Nat) : ByteArray := (let zero : ByteArray := text ("0"); (let digits : ByteArray := significant ((LexLeanRuntime.append (digit (number) (10000000000000000000)) ((LexLeanRuntime.append (digit (number) (1000000000000000000)) ((LexLeanRuntime.append (digit (number) (100000000000000000)) ((LexLeanRuntime.append (digit (number) (10000000000000000)) ((LexLeanRuntime.append (digit (number) (1000000000000000)) ((LexLeanRuntime.append (digit (number) (100000000000000)) ((LexLeanRuntime.append (digit (number) (10000000000000)) ((LexLeanRuntime.append (digit (number) (1000000000000)) ((LexLeanRuntime.append (digit (number) (100000000000)) ((LexLeanRuntime.append (digit (number) (10000000000)) ((LexLeanRuntime.append (digit (number) (1000000000)) ((LexLeanRuntime.append (digit (number) (100000000)) ((LexLeanRuntime.append (digit (number) (10000000)) ((LexLeanRuntime.append (digit (number) (1000000)) ((LexLeanRuntime.append (digit (number) (100000)) ((LexLeanRuntime.append (digit (number) (10000)) ((LexLeanRuntime.append (digit (number) (1000)) ((LexLeanRuntime.append (digit (number) (100)) ((LexLeanRuntime.append (digit (number) (10)) (digit (number) (1)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)); (if (Nat.beq (number) (0)) then zero else digits)))

@[expose] public def kindUpper (kind : IntKind) : ByteArray := (match kind with | IntKind.U8 => text ("U8") | IntKind.U16 => text ("U16") | IntKind.U32 => text ("U32") | IntKind.U64 => text ("U64") | IntKind.I8 => text ("I8") | IntKind.I16 => text ("I16") | IntKind.I32 => text ("I32") | IntKind.I64 => text ("I64"))

@[expose] public def kindLower (kind : IntKind) : ByteArray := (match kind with | IntKind.U8 => text ("u8") | IntKind.U16 => text ("u16") | IntKind.U32 => text ("u32") | IntKind.U64 => text ("u64") | IntKind.I8 => text ("i8") | IntKind.I16 => text ("i16") | IntKind.I32 => text ("i32") | IntKind.I64 => text ("i64"))

@[expose] public def optKindDebug (width : Option (IntKind)) : ByteArray := (match width with | Option.some kind => cat (text ("Some(")) (cat (kindUpper (kind)) (text (")"))) | Option.none => text ("None"))

@[expose] public def optKindEq (left : Option (IntKind)) (right : Option (IntKind)) : Bool := (LexLeanRuntime.equal (optKindDebug (left)) (optKindDebug (right)) : Bool)

@[expose] public def idTag (who : Id) : Nat := (match who with | Id.IdLocal _ => 0 | Id.IdHolder _ => 1 | Id.IdOperand _ => 2 | Id.IdCallee _ => 3 | Id.IdBoxed _ => 4 | Id.IdPart _ => 5 | Id.IdCapture _ => 6 | Id.IdParam _ => 7 | Id.IdFunction _ => 8 | Id.IdExport _ => 9)

@[expose] public def idNum (who : Id) : Nat := (match who with | Id.IdLocal n => n | Id.IdHolder n => n | Id.IdOperand n => n | Id.IdCallee n => n | Id.IdBoxed n => n | Id.IdPart n => n | Id.IdCapture n => n | Id.IdParam n => n | Id.IdFunction n => n | Id.IdExport _ => 0)

@[expose] public def idName (who : Id) : ByteArray := (match who with | Id.IdLocal _ => _root_.ByteArray.mk #[] | Id.IdHolder _ => _root_.ByteArray.mk #[] | Id.IdOperand _ => _root_.ByteArray.mk #[] | Id.IdCallee _ => _root_.ByteArray.mk #[] | Id.IdBoxed _ => _root_.ByteArray.mk #[] | Id.IdPart _ => _root_.ByteArray.mk #[] | Id.IdCapture _ => _root_.ByteArray.mk #[] | Id.IdParam _ => _root_.ByteArray.mk #[] | Id.IdFunction _ => _root_.ByteArray.mk #[] | Id.IdExport n => n)

@[expose] public def idText (who : Id) : ByteArray := (match who with | Id.IdLocal n => cat (text ("v")) (decimal (n)) | Id.IdHolder n => cat (text ("m")) (decimal (n)) | Id.IdOperand n => cat (text ("a")) (decimal (n)) | Id.IdCallee n => cat (text ("c")) (decimal (n)) | Id.IdBoxed n => cat (text ("r")) (decimal (n)) | Id.IdPart n => cat (text ("h")) (decimal (n)) | Id.IdCapture n => cat (text ("k")) (decimal (n)) | Id.IdParam n => cat (text ("p")) (decimal (n)) | Id.IdFunction n => cat (text ("f")) (decimal (n)) | Id.IdExport n => n)

@[expose] public def idEq (left : Id) (right : Id) : Bool := (if (Nat.beq (idTag (left)) (idTag (right))) then (if (Nat.beq (idNum (left)) (idNum (right))) then (LexLeanRuntime.equal (idName (left)) (idName (right)) : Bool) else false) else false)

@[expose] public def idHas (items : List (Id)) (who : Id) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (item : Id) => (let same : Bool := idEq (item) (who); (found || same)))) (false) (items) : Bool)

@[expose] public def leafName (kind : LeafKind) : ByteArray := (match kind with | LeafKind.LkUnit => text ("Unit") | LeafKind.LkBool => text ("Bool") | LeafKind.LkNat => text ("Nat") | LeafKind.LkInt => text ("Int") | LeafKind.LkOrdering => text ("Ordering") | LeafKind.LkStr => text ("Str") | LeafKind.LkBytes => text ("Bytes") | LeafKind.LkOverflow => text ("Overflow") | LeafKind.LkI128 => text ("I128"))

@[expose] public def leafKey (kind : LeafKind) : ByteArray := (match kind with | LeafKind.LkUnit => text ("unit") | LeafKind.LkBool => text ("bool") | LeafKind.LkNat => text ("nat") | LeafKind.LkInt => text ("int") | LeafKind.LkOrdering => text ("ordering") | LeafKind.LkStr => text ("string") | LeafKind.LkBytes => text ("bytes") | LeafKind.LkOverflow => text ("overflow") | LeafKind.LkI128 => text ("i128"))

@[expose] public def leafHeap (kind : LeafKind) : Bool := (match kind with | LeafKind.LkUnit => false | LeafKind.LkBool => false | LeafKind.LkNat => false | LeafKind.LkInt => false | LeafKind.LkOrdering => false | LeafKind.LkStr => true | LeafKind.LkBytes => true | LeafKind.LkOverflow => false | LeafKind.LkI128 => false)

@[expose] public def unaryName (kind : UnaryKind) : ByteArray := (match kind with | UnaryKind.UkOption => text ("Option") | UnaryKind.UkList => text ("List") | UnaryKind.UkRc => text ("Rc") | UnaryKind.UkFallible => text ("Fallible") | UnaryKind.UkRef => text ("Ref"))

@[expose] public def unaryKey (kind : UnaryKind) : ByteArray := (match kind with | UnaryKind.UkOption => text ("option") | UnaryKind.UkList => text ("list") | UnaryKind.UkRc => text ("rc") | UnaryKind.UkFallible => text ("fallible") | UnaryKind.UkRef => text ("ref"))

@[expose] public def unaryHeap (kind : UnaryKind) : Bool := (match kind with | UnaryKind.UkOption => false | UnaryKind.UkList => true | UnaryKind.UkRc => true | UnaryKind.UkFallible => false | UnaryKind.UkRef => false)

@[expose] public def unaryFallible (kind : UnaryKind) : Bool := (match kind with | UnaryKind.UkOption => false | UnaryKind.UkList => false | UnaryKind.UkRc => false | UnaryKind.UkFallible => true | UnaryKind.UkRef => false)

@[expose] public def binaryName (kind : BinaryKind) : ByteArray := (match kind with | BinaryKind.BkResult => text ("Result") | BinaryKind.BkPair => text ("Pair"))

@[expose] public def binaryKey (kind : BinaryKind) : ByteArray := (match kind with | BinaryKind.BkResult => text ("result") | BinaryKind.BkPair => text ("pair"))

@[expose] public def nameName (kind : NameKind) : ByteArray := (match kind with | NameKind.NkAdt => text ("Adt") | NameKind.NkFn => text ("Fn") | NameKind.NkParam => text ("Param"))

@[expose] public def nameKey (kind : NameKind) : ByteArray := (match kind with | NameKind.NkAdt => text ("adt") | NameKind.NkFn => text ("fn") | NameKind.NkParam => text ("param"))

@[expose] public def nameIsFn (kind : NameKind) : Bool := (match kind with | NameKind.NkAdt => false | NameKind.NkFn => true | NameKind.NkParam => false)

@[expose] public def nameIsParam (kind : NameKind) : Bool := (match kind with | NameKind.NkAdt => false | NameKind.NkFn => false | NameKind.NkParam => true)

@[expose] public def tyDebug : (ty : Ty) -> ByteArray
  | Ty.TyLeaf k => leafName (k)
  | Ty.TyFixed k => cat (text ("Fixed(")) (cat (kindUpper (k)) (text (")")))
  | Ty.TyUnary k inner => cat (unaryName (k)) (cat (text ("(")) (cat (tyDebug (inner)) (text (")"))))
  | Ty.TyBinary k left right => cat (binaryName (k)) (cat (text ("(")) (cat (tyDebug (left)) (cat (text (", ")) (cat (tyDebug (right)) (text (")"))))))
  | Ty.TyNamed k n => cat (nameName (k)) (cat (text ("(")) (cat (decimal (n)) (text (")"))))
  | Ty.TyNil => text ("unreachable")
  | Ty.TyCons _ _ => text ("unreachable")

@[expose] public def tyHeap : (ty : Ty) -> Bool
  | Ty.TyLeaf k => leafHeap (k)
  | Ty.TyFixed _ => false
  | Ty.TyUnary k inner => (match unaryHeap (k) with | Bool.true => true | Bool.false => tyHeap (inner))
  | Ty.TyBinary _ left right => (let l : Bool := tyHeap (left); (let r : Bool := tyHeap (right); (l || r)))
  | Ty.TyNamed k _ => nameIsParam (k)
  | Ty.TyNil => false
  | Ty.TyCons _ _ => false

@[expose] public def tyLen : (tys : Ty) -> Nat
  | Ty.TyLeaf _ => 0
  | Ty.TyFixed _ => 0
  | Ty.TyUnary _ _ => 0
  | Ty.TyBinary _ _ _ => 0
  | Ty.TyNamed _ _ => 0
  | Ty.TyNil => 0
  | Ty.TyCons _ tail => (let rest : Nat := tyLen (tail); (1 + rest))

@[expose] public def tyIsFallible (ty : Ty) : Bool := (match ty with | Ty.TyLeaf _ => false | Ty.TyFixed _ => false | Ty.TyUnary k _ => unaryFallible (k) | Ty.TyBinary _ _ _ => false | Ty.TyNamed _ _ => false | Ty.TyNil => false | Ty.TyCons _ _ => false)

@[expose] public def patLen : (pats : Pat) -> Nat
  | Pat.PLeaf _ => 0
  | Pat.PBind _ => 0
  | Pat.PTuple _ => 0
  | Pat.PWrap _ _ => 0
  | Pat.PAdt _ _ _ => 0
  | Pat.PNil => 0
  | Pat.PCons _ tail => (let rest : Nat := patLen (tail); (1 + rest))

@[expose] public def patIsWild (pat : Pat) : Bool := (match pat with | Pat.PLeaf k => (match k with | PatLeaf.PlWild => true | PatLeaf.PlUnit => false | PatLeaf.PlNone => false | PatLeaf.PlOrdering => false) | Pat.PBind _ => false | Pat.PTuple _ => false | Pat.PWrap _ _ => false | Pat.PAdt _ _ _ => false | Pat.PNil => false | Pat.PCons _ _ => false)

@[expose] public def patIsTuple (pat : Pat) : Bool := (match pat with | Pat.PLeaf _ => false | Pat.PBind _ => false | Pat.PTuple _ => true | Pat.PWrap _ _ => false | Pat.PAdt _ _ _ => false | Pat.PNil => false | Pat.PCons _ _ => false)

@[expose] public def patHasWild : (pats : Pat) -> Bool
  | Pat.PLeaf _ => false
  | Pat.PBind _ => false
  | Pat.PTuple _ => false
  | Pat.PWrap _ _ => false
  | Pat.PAdt _ _ _ => false
  | Pat.PNil => false
  | Pat.PCons head tail => (let here : Bool := patIsWild (head); (let rest : Bool := patHasWild (tail); (here || rest)))

@[expose] public def exprLen : (exprs : Expr) -> Nat
  | Expr.ELit _ _ => 0
  | Expr.ERead _ _ _ => 0
  | Expr.EWrap _ _ _ => 0
  | Expr.ECall _ _ _ _ => 0
  | Expr.EApply _ _ _ _ => 0
  | Expr.EConstruct _ _ _ => 0
  | Expr.EPair _ _ _ => 0
  | Expr.EIf _ _ _ _ => 0
  | Expr.EMatch _ _ _ => 0
  | Expr.EBlockE _ => 0
  | Expr.EProbe _ _ _ => 0
  | Expr.EBlock _ _ => 0
  | Expr.ENil => 0
  | Expr.ECons _ tail => (let rest : Nat := exprLen (tail); (1 + rest))
  | Expr.ELet _ _ _ _ _ _ => 0
  | Expr.EArm _ _ _ => 0
  | Expr.ELoop _ _ => 0

@[expose] public def lookupNat (entries : List ((Prod (Nat) (Nat)))) (key : Nat) : Option (Nat) := (LexLeanCollections.listFold ((fun (found : Option (Nat)) (entry : (Prod (Nat) (Nat))) => (match found with | Option.some value => Option.some (value) | Option.none => (if (Nat.beq ((entry).1) (key)) then Option.some ((entry).2) else Option.none)))) (Option.none) (entries) : Option (Nat))

@[expose] public def lookupFlag (entries : List ((Prod (Nat) (Bool)))) (key : Nat) : Option (Bool) := (LexLeanCollections.listFold ((fun (found : Option (Bool)) (entry : (Prod (Nat) (Bool))) => (match found with | Option.some value => Option.some (value) | Option.none => (if (Nat.beq ((entry).1) (key)) then Option.some ((entry).2) else Option.none)))) (Option.none) (entries) : Option (Bool))

@[expose] public def lookupVariants (entries : List ((Prod (Nat) (List ((Prod (Nat) (Nat))))))) (key : Nat) : Option (List ((Prod (Nat) (Nat)))) := (LexLeanCollections.listFold ((fun (found : Option (List ((Prod (Nat) (Nat))))) (entry : (Prod (Nat) (List ((Prod (Nat) (Nat)))))) => (match found with | Option.some value => Option.some (value) | Option.none => (if (Nat.beq ((entry).1) (key)) then Option.some ((entry).2) else Option.none)))) (Option.none) (entries) : Option (List ((Prod (Nat) (Nat)))))

@[expose] public def isSome (found : Option (Nat)) : Bool := (match found with | Option.some _ => true | Option.none => false)

@[expose] public def hasFlag (entries : List ((Prod (Nat) (Bool)))) (key : Nat) : Bool := (match lookupFlag (entries) (key) with | Option.some _ => true | Option.none => false)

@[expose] public def hasVariants (entries : List ((Prod (Nat) (List ((Prod (Nat) (Nat))))))) (key : Nat) : Bool := (match lookupVariants (entries) (key) with | Option.some _ => true | Option.none => false)

@[expose] public def adtArity (decl : Decl) (adt : Nat) (constructor : Nat) : Option (Nat) := (match lookupVariants ((decl).adts) (adt) with | Option.some variants => lookupNat (variants) (constructor) | Option.none => Option.none)

@[expose] public def closureArity (decl : Decl) (fnType : Nat) (function : Nat) : Option (Nat) := (match lookupVariants ((decl).closures) (fnType) with | Option.some variants => lookupNat (variants) (function) | Option.none => Option.none)

@[expose] public def aritiesStep : (acc : List ((Prod (Nat) (Nat)))) -> (row : Row) -> List ((Prod (Nat) (Nat)))
  | acc, Row.RowNil => acc
  | acc, Row.RowCons number fields rest => aritiesStep (((number, tyLen (fields)) :: acc)) (rest)

@[expose] public def declareDefs : (decl : Decl) -> (defs : Def) -> Except (ByteArray) (Decl)
  | decl, Def.DEnum _ _ _ => Except.ok (decl)
  | decl, Def.DApply _ _ _ _ _ _ => Except.ok (decl)
  | decl, Def.DFunction _ _ _ _ _ => Except.ok (decl)
  | decl, Def.DNil => Except.ok (decl)
  | decl, Def.DCons head tail => (match head with | Def.DEnum name variants _ => (match name with | Ty.TyLeaf _ => Except.error (cat (text ("an enum named ")) (cat (tyDebug (name)) (text (" is not a declared type")))) | Ty.TyFixed _ => Except.error (cat (text ("an enum named ")) (cat (tyDebug (name)) (text (" is not a declared type")))) | Ty.TyUnary _ _ => Except.error (cat (text ("an enum named ")) (cat (tyDebug (name)) (text (" is not a declared type")))) | Ty.TyBinary _ _ _ => Except.error (cat (text ("an enum named ")) (cat (tyDebug (name)) (text (" is not a declared type")))) | Ty.TyNamed k n => (match nameIsFn (k) with | Bool.true => (match hasVariants ((decl).closures) (n) with | Bool.true => Except.error (cat (text ("hygiene: ")) (cat (tyDebug (name)) (text (" is declared twice")))) | Bool.false => declareDefs (({ adts := (decl).adts, closures := ((n, aritiesStep (([] : List ((Prod (Nat) (Nat))))) (variants)) :: (decl).closures), applies := (decl).applies, functions := (decl).functions } : Decl)) (tail)) | Bool.false => (match hasVariants ((decl).adts) (n) with | Bool.true => Except.error (cat (text ("hygiene: ")) (cat (tyDebug (name)) (text (" is declared twice")))) | Bool.false => declareDefs (({ adts := ((n, aritiesStep (([] : List ((Prod (Nat) (Nat))))) (variants)) :: (decl).adts), closures := (decl).closures, applies := (decl).applies, functions := (decl).functions } : Decl)) (tail))) | Ty.TyNil => Except.error (cat (text ("an enum named ")) (cat (tyDebug (name)) (text (" is not a declared type")))) | Ty.TyCons _ _ => Except.error (cat (text ("an enum named ")) (cat (tyDebug (name)) (text (" is not a declared type"))))) | Def.DApply fnType _ _ fallible _ _ => (match hasFlag ((decl).applies) (fnType) with | Bool.true => Except.error (cat (text ("hygiene: Fn")) (cat (decimal (fnType)) (text ("::apply is declared twice")))) | Bool.false => declareDefs (({ adts := (decl).adts, closures := (decl).closures, applies := ((fnType, fallible) :: (decl).applies), functions := (decl).functions } : Decl)) (tail)) | Def.DFunction name _ result _ _ => (match name with | Id.IdLocal _ => declareDefs (decl) (tail) | Id.IdHolder _ => declareDefs (decl) (tail) | Id.IdOperand _ => declareDefs (decl) (tail) | Id.IdCallee _ => declareDefs (decl) (tail) | Id.IdBoxed _ => declareDefs (decl) (tail) | Id.IdPart _ => declareDefs (decl) (tail) | Id.IdCapture _ => declareDefs (decl) (tail) | Id.IdParam _ => declareDefs (decl) (tail) | Id.IdFunction n => (match hasFlag ((decl).functions) (n) with | Bool.true => Except.error (cat (text ("hygiene: f")) (cat (decimal (n)) (text (" is declared twice")))) | Bool.false => declareDefs (({ adts := (decl).adts, closures := (decl).closures, applies := (decl).applies, functions := ((n, tyIsFallible (result)) :: (decl).functions) } : Decl)) (tail)) | Id.IdExport _ => declareDefs (decl) (tail)) | Def.DNil => declareDefs (decl) (tail) | Def.DCons _ _ => declareDefs (decl) (tail))

@[expose] public def heapSt (cx : Cx) (st : St) (what : ByteArray) : Except (ByteArray) (St) := (match (cx).core with | Bool.true => Except.error (cat (text ("hidden allocation: rust-core renders ")) (cat (what) (text (", which requires heap allocation")))) | Bool.false => Except.ok (st))

@[expose] public def heapUnit (cx : Cx) (what : ByteArray) : Except (ByteArray) (Unit) := (match (cx).core with | Bool.true => Except.error (cat (text ("hidden allocation: rust-core renders ")) (cat (what) (text (", which requires heap allocation")))) | Bool.false => Except.ok (()))

@[expose] public def tyWalk : (cx : Cx) -> (ty : Ty) -> Except (ByteArray) (Unit)
  | _cx, Ty.TyLeaf _ => Except.ok (())
  | _cx, Ty.TyFixed _ => Except.ok (())
  | cx, Ty.TyUnary _ inner => tyWalk (cx) (inner)
  | cx, Ty.TyBinary _ left right => (match tyWalk (cx) (left) with | Except.ok _ => tyWalk (cx) (right) | Except.error problem1 => Except.error (problem1))
  | cx, Ty.TyNamed k n => (match nameIsParam (k) with | Bool.true => Except.ok (()) | Bool.false => (match nameIsFn (k) with | Bool.true => (match hasVariants (((cx).decl).closures) (n) with | Bool.true => Except.ok (()) | Bool.false => Except.error (cat (text ("type Fn")) (cat (decimal (n)) (text (" is not declared"))))) | Bool.false => (match hasVariants (((cx).decl).adts) (n) with | Bool.true => Except.ok (()) | Bool.false => Except.error (cat (text ("type Adt")) (cat (decimal (n)) (text (" is not declared")))))))
  | _cx, Ty.TyNil => Except.ok (())
  | _cx, Ty.TyCons _ _ => Except.ok (())

@[expose] public def tyOk (cx : Cx) (ty : Ty) : Except (ByteArray) (Unit) := (let heapy : Bool := tyHeap (ty); (let blocked : Bool := (if (cx).core then heapy else false); (match blocked with | Bool.true => Except.error (cat (text ("hidden allocation: rust-core renders the heap type ")) (cat (tyDebug (ty)) (text (", which rust-core does not provide")))) | Bool.false => tyWalk (cx) (ty))))

@[expose] public def tysOk : (cx : Cx) -> (tys : Ty) -> Except (ByteArray) (Unit)
  | _cx, Ty.TyLeaf _ => Except.ok (())
  | _cx, Ty.TyFixed _ => Except.ok (())
  | _cx, Ty.TyUnary _ _ => Except.ok (())
  | _cx, Ty.TyBinary _ _ _ => Except.ok (())
  | _cx, Ty.TyNamed _ _ => Except.ok (())
  | _cx, Ty.TyNil => Except.ok (())
  | cx, Ty.TyCons head tail => (match tyOk (cx) (head) with | Except.ok _ => tysOk (cx) (tail) | Except.error problem2 => Except.error (problem2))

@[expose] public def rowTysOk : (cx : Cx) -> (row : Row) -> Except (ByteArray) (Unit)
  | _cx, Row.RowNil => Except.ok (())
  | cx, Row.RowCons _ fields rest => (match tysOk (cx) (fields) with | Except.ok _ => rowTysOk (cx) (rest) | Except.error problem3 => Except.error (problem3))

@[expose] public def natHas (items : List (Nat)) (key : Nat) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (item : Nat) => (found || (Nat.beq (item) (key))))) (false) (items) : Bool)

@[expose] public def partsGet (parted : List ((Prod (Id) (List (Nat))))) (who : Id) : Option (List (Nat)) := (LexLeanCollections.listFold ((fun (found : Option (List (Nat))) (entry : (Prod (Id) (List (Nat)))) => (match found with | Option.some value => Option.some (value) | Option.none => (let same : Bool := idEq ((entry).1) (who); (if same then Option.some ((entry).2) else Option.none))))) (Option.none) (parted) : Option (List (Nat)))

@[expose] public def partsHas (parted : List ((Prod (Id) (List (Nat))))) (who : Id) : Bool := (match partsGet (parted) (who) with | Option.some _ => true | Option.none => false)

@[expose] public def partsDrop (parted : List ((Prod (Id) (List (Nat))))) (who : Id) : List ((Prod (Id) (List (Nat)))) := (LexLeanCollections.listFold ((fun (kept : List ((Prod (Id) (List (Nat))))) (entry : (Prod (Id) (List (Nat)))) => (let same : Bool := idEq ((entry).1) (who); (if same then kept else (entry :: kept))))) (([] : List ((Prod (Id) (List (Nat)))))) (parted) : List ((Prod (Id) (List (Nat)))))

@[expose] public def partsPut (parted : List ((Prod (Id) (List (Nat))))) (who : Id) (taken : List (Nat)) : List ((Prod (Id) (List (Nat)))) := ((who, taken) :: partsDrop (parted) (who))

@[expose] public def unionNats (left : List (Nat)) (right : List (Nat)) : List (Nat) := (LexLeanCollections.listFold ((fun (acc : List (Nat)) (item : Nat) => (let present : Bool := natHas (acc) (item); (if present then acc else (item :: acc))))) (left) (right) : List (Nat))

@[expose] public def unionIds (left : List (Id)) (right : List (Id)) : List (Id) := (LexLeanCollections.listFold ((fun (acc : List (Id)) (item : Id) => (let present : Bool := idHas (acc) (item); (if present then acc else (item :: acc))))) (left) (right) : List (Id))

@[expose] public def unionParts (left : List ((Prod (Id) (List (Nat))))) (right : List ((Prod (Id) (List (Nat))))) : List ((Prod (Id) (List (Nat)))) := (LexLeanCollections.listFold ((fun (acc : List ((Prod (Id) (List (Nat))))) (entry : (Prod (Id) (List (Nat)))) => (match partsGet (acc) ((entry).1) with | Option.some «have» => partsPut (acc) ((entry).1) (unionNats («have») ((entry).2)) | Option.none => partsPut (acc) ((entry).1) ((entry).2)))) (left) (right) : List ((Prod (Id) (List (Nat)))))

@[expose] public def readOk (st : St) (who : Id) : Except (ByteArray) (Unit) := (match idHas ((st).scope) (who) with | Bool.true => (match idHas ((st).moved) (who) with | Bool.true => Except.error (cat (text ("ownership: ")) (cat (tick (idText (who))) (text (" is read after it was moved")))) | Bool.false => (match partsHas ((st).parted) (who) with | Bool.true => Except.error (cat (text ("ownership: ")) (cat (tick (idText (who))) (text (" is read after a component was moved out of it")))) | Bool.false => Except.ok (()))) | Bool.false => Except.error (cat (tick (idText (who))) (text (" is not bound"))))

@[expose] public def consume (st : St) (who : Id) : Except (ByteArray) (St) := (match idHas ((st).scope) (who) with | Bool.true => (match partsHas ((st).parted) (who) with | Bool.true => Except.error (cat (text ("ownership: ")) (cat (tick (idText (who))) (text (" is moved after a component was moved out of it")))) | Bool.false => (match idHas ((st).moved) (who) with | Bool.true => Except.error (cat (text ("ownership: ")) (cat (tick (idText (who))) (text (" is moved twice")))) | Bool.false => Except.ok (({ bound := (st).bound, scope := (st).scope, moved := (who :: (st).moved), parted := (st).parted, propagates := (st).propagates } : St)))) | Bool.false => Except.error (cat (tick (idText (who))) (text (" is not bound"))))

@[expose] public def takeParts : (taken : List (Nat)) -> (index : Nat) -> (parts : Pat) -> (owner : ByteArray) -> Except (ByteArray) (List (Nat))
  | taken, _index, Pat.PLeaf _, _owner => Except.ok (taken)
  | taken, _index, Pat.PBind _, _owner => Except.ok (taken)
  | taken, _index, Pat.PTuple _, _owner => Except.ok (taken)
  | taken, _index, Pat.PWrap _ _, _owner => Except.ok (taken)
  | taken, _index, Pat.PAdt _ _ _, _owner => Except.ok (taken)
  | taken, _index, Pat.PNil, _owner => Except.ok (taken)
  | taken, index, Pat.PCons head tail, owner => (let wild : Bool := patIsWild (head); (match wild with | Bool.true => takeParts (taken) ((index + 1)) (tail) (owner) | Bool.false => (match natHas (taken) (index) with | Bool.true => Except.error (cat (text ("ownership: component ")) (cat (decimal (index)) (cat (text (" of ")) (cat (owner) (text (" is moved twice")))))) | Bool.false => takeParts ((index :: taken)) ((index + 1)) (tail) (owner))))

@[expose] public def consumeParts (st : St) (who : Id) (parts : Pat) : Except (ByteArray) (St) := (match idHas ((st).scope) (who) with | Bool.true => (match idHas ((st).moved) (who) with | Bool.true => Except.error (cat (text ("ownership: a component of ")) (cat (tick (idText (who))) (text (" is moved after it was moved")))) | Bool.false => (let «have» : List (Nat) := (match partsGet ((st).parted) (who) with | Option.some found => found | Option.none => ([] : List (Nat))); (match takeParts («have») (0) (parts) (tick (idText (who))) with | Except.ok taken => Except.ok (({ bound := (st).bound, scope := (st).scope, moved := (st).moved, parted := partsPut ((st).parted) (who) (taken), propagates := (st).propagates } : St)) | Except.error problem4 => Except.error (problem4)))) | Bool.false => Except.error (cat (tick (idText (who))) (text (" is not bound"))))

@[expose] public def bindPat : (st : St) -> (decl : Decl) -> (pat : Pat) -> Except (ByteArray) (St)
  | st, _decl, Pat.PLeaf _ => Except.ok (st)
  | st, _decl, Pat.PBind who => (match idHas ((st).bound) (who) with | Bool.true => Except.error (cat (text ("hygiene: ")) (cat (tick (idText (who))) (text (" is bound twice in one function")))) | Bool.false => Except.ok (({ bound := (who :: (st).bound), scope := (who :: (st).scope), moved := (st).moved, parted := (st).parted, propagates := (st).propagates } : St)))
  | st, decl, Pat.PTuple inner => bindPat (st) (decl) (inner)
  | st, decl, Pat.PWrap _ inner => bindPat (st) (decl) (inner)
  | st, decl, Pat.PAdt adt constructor fields => (match adtArity (decl) (adt) (constructor) with | Option.none => Except.error (cat (text ("pattern Adt")) (cat (decimal (adt)) (cat (text ("::C")) (cat (decimal (constructor)) (text (" is not declared")))))) | Option.some arity => (match (Nat.beq (arity) (patLen (fields))) with | Bool.true => bindPat (st) (decl) (fields) | Bool.false => Except.error (cat (text ("pattern Adt")) (cat (decimal (adt)) (cat (text ("::C")) (cat (decimal (constructor)) (cat (text (" binds ")) (cat (decimal (patLen (fields))) (cat (text (" fields of ")) (decimal (arity)))))))))))
  | st, _decl, Pat.PNil => Except.ok (st)
  | st, decl, Pat.PCons head tail => (match bindPat (st) (decl) (head) with | Except.ok next => bindPat (next) (decl) (tail) | Except.error problem5 => Except.error (problem5))

@[expose] public def callOk (st : St) (cx : Cx) (fallible : Bool) (propagate : Bool) (tail : Bool) (what : ByteArray) : Except (ByteArray) (St) := (let excused : Bool := (if tail then (cx).fallible else false); (match fallible with | Bool.true => (match propagate with | Bool.true => Except.ok (({ bound := (st).bound, scope := (st).scope, moved := (st).moved, parted := (st).parted, propagates := true } : St)) | Bool.false => (match excused with | Bool.true => Except.ok (st) | Bool.false => Except.error (cat (text ("arithmetic: the fallible call of ")) (cat (what) (text (" does not propagate its overflow")))))) | Bool.false => (match propagate with | Bool.true => Except.error (cat (text ("arithmetic: the infallible call of ")) (cat (what) (text (" propagates a failure it cannot have")))) | Bool.false => Except.ok (st))))

@[expose] public def itemRowOf (rows : List (ItemRow)) (family : Nat) : Option (ItemRow) := (LexLeanCollections.listFold ((fun (found : Option (ItemRow)) (row : ItemRow) => (match found with | Option.some value => Option.some (value) | Option.none => (if (Nat.beq ((row).family) (family)) then Option.some (row) else Option.none)))) (Option.none) (rows) : Option (ItemRow))

@[expose] public def widthKind (width : Option (IntKind)) : IntKind := (match width with | Option.some kind => kind | Option.none => IntKind.U8)

@[expose] public def itemPath (rows : List (ItemRow)) (item : Rt) : ByteArray := (match itemRowOf (rows) ((item).family) with | Option.none => _root_.ByteArray.mk #[] | Option.some row => (if (Nat.beq ((row).form) (0)) then (row).stem else (if (Nat.beq ((row).form) (1)) then cat (text ("fixed_")) (cat (kindLower (widthKind ((item).width))) (cat (text ("::")) ((row).stem))) else cat ((row).stem) (kindLower (widthKind ((item).width))))))

@[expose] public def itemWidth (rows : List (ItemRow)) (item : Rt) : Option (IntKind) := (match itemRowOf (rows) ((item).family) with | Option.none => Option.none | Option.some row => (if (Nat.beq ((row).form) (0)) then Option.none else Option.some (widthKind ((item).width))))

@[expose] public def itemFallible (rows : List (ItemRow)) (item : Rt) : Bool := (match itemRowOf (rows) ((item).family) with | Option.none => false | Option.some row => (row).fallible)

@[expose] public def itemHeap (rows : List (ItemRow)) (item : Rt) : Bool := (match itemRowOf (rows) ((item).family) with | Option.none => false | Option.some row => (row).heap)

@[expose] public def lastSegment (path : ByteArray) : ByteArray := (match LexLeanStd.Text.find (path) (text ("::")) with | Option.some «at» => LexLeanStd.Text.sliceOr (path) ((«at» + 2)) ((LexLeanRuntime.subtract ((LexLeanRuntime.length (path) : Nat)) ((«at» + 2)) : Nat)) | Option.none => path)

@[expose] public def runtimeKind (base : ByteArray) : ByteArray := (match LexLeanStd.Text.startsWith (base) (text ("checked_neg_")) with | Bool.true => text ("checked_neg") | Bool.false => (match LexLeanStd.Text.startsWith (base) (text ("format_")) with | Bool.true => text ("format_decimal") | Bool.false => (match LexLeanStd.Text.startsWith (base) (text ("parse_")) with | Bool.true => text ("parse_decimal") | Bool.false => (match LexLeanStd.Text.startsWith (base) (text ("append_")) with | Bool.true => text ("append") | Bool.false => (match LexLeanStd.Text.startsWith (base) (text ("length_")) with | Bool.true => text ("length") | Bool.false => (match LexLeanStd.Text.startsWith (base) (text ("index_")) with | Bool.true => text ("index") | Bool.false => (match LexLeanStd.Text.startsWith (base) (text ("slice_")) with | Bool.true => text ("slice") | Bool.false => (match (LexLeanRuntime.equal (base) (text ("nat_succ")) : Bool) with | Bool.true => text ("succ") | Bool.false => base))))))))

@[expose] public def bytesLess (left : ByteArray) (right : ByteArray) : Bool := (LexLeanRuntime.equal ((LexLeanRuntime.compareBytes (left) (right) : Ordering)) ((LexLeanRuntime.compareBytes (_root_.ByteArray.mk #[]) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 1))]) : Ordering)) : Bool)

@[expose] public def bytesHas (items : List (ByteArray)) (key : ByteArray) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (item : ByteArray) => (found || (LexLeanRuntime.equal (item) (key) : Bool)))) (false) (items) : Bool)

@[expose] public def sortedStep (key : ByteArray) (state : (Prod (Bool) (List (ByteArray)))) (item : ByteArray) : (Prod (Bool) (List (ByteArray))) := (let placed : Bool := (state).1; (let rev : List (ByteArray) := (state).2; (match placed with | Bool.true => (true, (item :: rev)) | Bool.false => (match bytesLess (key) (item) with | Bool.true => (true, (item :: (key :: rev))) | Bool.false => (false, (item :: rev))))))

@[expose] public def sortedInsert (items : List (ByteArray)) (key : ByteArray) : List (ByteArray) := (match bytesHas (items) (key) with | Bool.true => items | Bool.false => (let done : (Prod (Bool) (List (ByteArray))) := (LexLeanCollections.listFold ((fun (state : (Prod (Bool) (List (ByteArray)))) (item : ByteArray) => sortedStep (key) (state) (item))) ((false, ([] : List (ByteArray)))) (items) : (Prod (Bool) (List (ByteArray)))); (match (done).1 with | Bool.true => LexLeanStd.Text.reverse ((done).2) | Bool.false => LexLeanStd.Text.reverse ((key :: (done).2)))))

@[expose] public def foundType (found : Found) (key : ByteArray) : Found := ({ insts := (found).insts, types := sortedInsert ((found).types) (key), items := (found).items } : Found)

@[expose] public def instSame (left : Inst) (right : Inst) : Bool := (if (LexLeanRuntime.equal ((left).construct) ((right).construct) : Bool) then (if (LexLeanRuntime.equal ((left).element) ((right).element) : Bool) then (if optKindEq ((left).originWidth) ((right).originWidth) then optKindEq ((left).width) ((right).width) else false) else false) else false)

@[expose] public def instHas (insts : List (Inst)) (inst : Inst) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (seen : Inst) => (let same : Bool := instSame (seen) (inst); (found || same)))) (false) (insts) : Bool)

@[expose] public def foundWide (found : Found) (construct : ByteArray) (origin : Origin) (width : Option (IntKind)) : Found := (let inst : Inst := ({ construct := construct, element := (origin).element, originWidth := (origin).width, width := width } : Inst); (match instHas ((found).insts) (inst) with | Bool.true => found | Bool.false => ({ insts := (inst :: (found).insts), types := (found).types, items := (found).items } : Found)))

@[expose] public def foundAt (found : Found) (construct : ByteArray) (origin : Origin) : Found := foundWide (found) (construct) (origin) (Option.none)

@[expose] public def foundTy : (found : Found) -> (ty : Ty) -> Found
  | found, Ty.TyLeaf k => foundType (found) (cat (text ("type:")) (leafKey (k)))
  | found, Ty.TyFixed _ => foundType (found) (text ("type:fixed"))
  | found, Ty.TyUnary k inner => (let f1 : Found := foundType (found) (cat (text ("type:")) (unaryKey (k))); foundTy (f1) (inner))
  | found, Ty.TyBinary k left right => (let f1 : Found := foundType (found) (cat (text ("type:")) (binaryKey (k))); (let f2 : Found := foundTy (f1) (left); foundTy (f2) (right)))
  | found, Ty.TyNamed k _ => foundType (found) (cat (text ("type:")) (nameKey (k)))
  | found, Ty.TyNil => found
  | found, Ty.TyCons head tail => (let f1 : Found := foundTy (found) (head); foundTy (f1) (tail))

@[expose] public def litKind (lit : Lit) : ByteArray := (match lit with | Lit.LitUnit => text ("lit:unit") | Lit.LitBool => text ("lit:bool") | Lit.LitNat => text ("lit:nat") | Lit.LitInt => text ("lit:int") | Lit.LitFixed _ => text ("lit:fixed") | Lit.LitStr => text ("lit:string") | Lit.LitBytes => text ("lit:bytes") | Lit.LitOrdering => text ("lit:ordering") | Lit.LitOverflow => text ("lit:overflow"))

@[expose] public def litWidth (lit : Lit) : Option (IntKind) := (match lit with | Lit.LitUnit => Option.none | Lit.LitBool => Option.none | Lit.LitNat => Option.none | Lit.LitInt => Option.none | Lit.LitFixed k => Option.some (k) | Lit.LitStr => Option.none | Lit.LitBytes => Option.none | Lit.LitOrdering => Option.none | Lit.LitOverflow => Option.none)

@[expose] public def isOverflowLit (lit : Lit) : Bool := (match lit with | Lit.LitUnit => false | Lit.LitBool => false | Lit.LitNat => false | Lit.LitInt => false | Lit.LitFixed _ => false | Lit.LitStr => false | Lit.LitBytes => false | Lit.LitOrdering => false | Lit.LitOverflow => true)

@[expose] public def patLeafConstruct (kind : PatLeaf) : Option (ByteArray) := (match kind with | PatLeaf.PlWild => Option.none | PatLeaf.PlUnit => Option.none | PatLeaf.PlNone => Option.some (text ("match:option")) | PatLeaf.PlOrdering => Option.some (text ("match:ordering")))

@[expose] public def patWrapConstruct (kind : PatWrap) : ByteArray := (match kind with | PatWrap.PwSome => text ("match:option") | PatWrap.PwOk => text ("match:result") | PatWrap.PwErr => text ("match:result"))

@[expose] public def patConstruct (pat : Pat) : Option (ByteArray) := (match pat with | Pat.PLeaf k => patLeafConstruct (k) | Pat.PBind _ => Option.none | Pat.PTuple _ => Option.none | Pat.PWrap k _ => Option.some (patWrapConstruct (k)) | Pat.PAdt _ _ _ => Option.some (text ("match:adt")) | Pat.PNil => Option.none | Pat.PCons _ _ => Option.none)

@[expose] public def foundArmPats : (found : Found) -> (arms : Expr) -> (origin : Origin) -> Found
  | found, Expr.ELit _ _, _origin => found
  | found, Expr.ERead _ _ _, _origin => found
  | found, Expr.EWrap _ _ _, _origin => found
  | found, Expr.ECall _ _ _ _, _origin => found
  | found, Expr.EApply _ _ _ _, _origin => found
  | found, Expr.EConstruct _ _ _, _origin => found
  | found, Expr.EPair _ _ _, _origin => found
  | found, Expr.EIf _ _ _ _, _origin => found
  | found, Expr.EMatch _ _ _, _origin => found
  | found, Expr.EBlockE _, _origin => found
  | found, Expr.EProbe _ _ _, _origin => found
  | found, Expr.EBlock _ _, _origin => found
  | found, Expr.ENil, _origin => found
  | found, Expr.ECons _ _, _origin => found
  | found, Expr.ELet _ _ _ _ _ _, _origin => found
  | found, Expr.EArm pat _ rest, origin => (match patConstruct (pat) with | Option.some kind => foundArmPats (foundAt (found) (kind) (origin)) (rest) (origin) | Option.none => foundArmPats (found) (rest) (origin))
  | found, Expr.ELoop _ _, _origin => found

@[expose] public def isNilExpr (e : Expr) : Bool := (match e with | Expr.ELit _ _ => false | Expr.ERead _ _ _ => false | Expr.EWrap _ _ _ => false | Expr.ECall _ _ _ _ => false | Expr.EApply _ _ _ _ => false | Expr.EConstruct _ _ _ => false | Expr.EPair _ _ _ => false | Expr.EIf _ _ _ _ => false | Expr.EMatch _ _ _ => false | Expr.EBlockE _ => false | Expr.EProbe _ _ _ => false | Expr.EBlock _ _ => false | Expr.ENil => true | Expr.ECons _ _ => false | Expr.ELet _ _ _ _ _ _ => false | Expr.EArm _ _ _ => false | Expr.ELoop _ _ => false)

@[expose] public def probeIsUncons (kind : ProbeKind) : Bool := (match kind with | ProbeKind.PkIsZero => false | ProbeKind.PkNonZero => false | ProbeKind.PkUncons => true | ProbeKind.PkPred => false)

@[expose] public def probeIsNat (kind : ProbeKind) : Bool := (match kind with | ProbeKind.PkIsZero => true | ProbeKind.PkNonZero => true | ProbeKind.PkUncons => false | ProbeKind.PkPred => false)

@[expose] public def probeNegatable (kind : ProbeKind) : Bool := (match kind with | ProbeKind.PkIsZero => true | ProbeKind.PkNonZero => true | ProbeKind.PkUncons => false | ProbeKind.PkPred => true)

@[expose] public def readConstruct (kind : ReadKind) : ByteArray := (match kind with | ReadKind.RkMove => text ("read") | ReadKind.RkClone => text ("read") | ReadKind.RkCopy => text ("read") | ReadKind.RkDeref => text ("read") | ReadKind.RkUnbox => text ("box"))

@[expose] public def wrapConstruct (kind : WrapKind) : ByteArray := (match kind with | WrapKind.WkNot => text ("if") | WrapKind.WkBox => text ("box") | WrapKind.WkWiden => text ("widen") | WrapKind.WkSucceed => text ("succeed"))

@[expose] public def isUncons (e : Expr) : Bool := (match e with | Expr.ELit _ _ => false | Expr.ERead _ _ _ => false | Expr.EWrap _ _ _ => false | Expr.ECall _ _ _ _ => false | Expr.EApply _ _ _ _ => false | Expr.EConstruct _ _ _ => false | Expr.EPair _ _ _ => false | Expr.EIf _ _ _ _ => false | Expr.EMatch _ _ _ => false | Expr.EBlockE _ => false | Expr.EProbe k _ _ => probeIsUncons (k) | Expr.EBlock _ _ => false | Expr.ENil => false | Expr.ECons _ _ => false | Expr.ELet _ _ _ _ _ _ => false | Expr.EArm _ _ _ => false | Expr.ELoop _ _ => false)

@[expose] public def conKind (con : Con) : ByteArray := (match con with | Con.ConNone _ => text ("none") | Con.ConSome => text ("some") | Con.ConOk _ _ => text ("ok") | Con.ConErr _ _ => text ("err") | Con.ConAdt _ _ => text ("adt") | Con.ConClosure _ _ => text ("closure") | Con.ConCons => text ("cons") | Con.ConNil _ => text ("nil"))

@[expose] public def foundCon (found : Found) (con : Con) : Found := (match con with | Con.ConNone ty => foundTy (found) (ty) | Con.ConSome => found | Con.ConOk ok error => foundTy (foundTy (found) (ok)) (error) | Con.ConErr ok error => foundTy (foundTy (found) (ok)) (error) | Con.ConAdt _ _ => found | Con.ConClosure _ _ => found | Con.ConCons => found | Con.ConNil ty => foundTy (found) (ty))

@[expose] public def foundExpr : (found : Found) -> (e : Expr) -> Found
  | found, Expr.ELit lit org => (match isOverflowLit (lit) with | Bool.true => found | Bool.false => foundWide (found) (litKind (lit)) (org) (litWidth (lit)))
  | found, Expr.ERead k _ org => foundAt (found) (readConstruct (k)) (org)
  | found, Expr.EWrap k inner org => (let f1 : Found := foundAt (found) (wrapConstruct (k)) (org); foundExpr (f1) (inner))
  | found, Expr.ECall callee args _ org => (let f1 : Found := (match callee with | Callee.CalFn _ => foundAt (found) (text ("call:function")) (org) | Callee.CalMachine _ => foundAt (found) (text ("call:machine")) (org) | Callee.CalRt item => foundWide (found) (cat (text ("call:runtime:")) (runtimeKind (lastSegment (itemPath ((found).items) (item))))) (org) (itemWidth ((found).items) (item))); foundExpr (f1) (args))
  | found, Expr.EApply _ args _ org => (let f1 : Found := foundAt (found) (text ("apply")) (org); foundExpr (f1) (args))
  | found, Expr.EConstruct con args org => (let f1 : Found := foundAt (foundCon (found) (con)) (cat (text ("construct:")) (conKind (con))) (org); foundExpr (f1) (args))
  | found, Expr.EPair left right org => (let f1 : Found := foundAt (found) (text ("construct:pair")) (org); (let f2 : Found := foundExpr (f1) (left); foundExpr (f2) (right)))
  | found, Expr.EIf condition thenBranch elseBranch org => (let f1 : Found := foundAt (found) (text ("if")) (org); (let f2 : Found := foundExpr (f1) (condition); (let f3 : Found := foundExpr (f2) (thenBranch); foundExpr (f3) (elseBranch))))
  | found, Expr.EMatch scrutinee arms org => (let head : Found := (match isNilExpr (arms) with | Bool.true => foundAt (found) (text ("match:empty")) (org) | Bool.false => (match isUncons (scrutinee) with | Bool.true => foundAt (found) (text ("match:list")) (org) | Bool.false => foundArmPats (found) (arms) (org))); (let f2 : Found := foundExpr (head) (scrutinee); foundExpr (f2) (arms)))
  | found, Expr.EBlockE inner => foundExpr (found) (inner)
  | found, Expr.EProbe k _ org => (match probeIsNat (k) with | Bool.true => foundAt (found) (text ("match:nat")) (org) | Bool.false => found)
  | found, Expr.EBlock lets tail => (let f1 : Found := foundExpr (found) (lets); foundExpr (f1) (tail))
  | found, Expr.ENil => found
  | found, Expr.ECons head tail => (let f1 : Found := foundExpr (found) (head); foundExpr (f1) (tail))
  | found, Expr.ELet pat hasTy ty value org rest => (let kind : ByteArray := (match patIsTuple (pat) with | Bool.true => text ("destructure:pair") | Bool.false => text ("let")); (let typed : Found := (match hasTy with | Bool.true => foundTy (found) (ty) | Bool.false => found); (let f1 : Found := foundAt (typed) (kind) (org); (let f2 : Found := foundExpr (f1) (value); foundExpr (f2) (rest)))))
  | found, Expr.EArm _ body rest => (let f1 : Found := foundExpr (found) (body); foundExpr (f1) (rest))
  | found, Expr.ELoop _ body => foundExpr (found) (body)

@[expose] public def minConstruct (insts : List (Inst)) : ByteArray := (match (LexLeanCollections.listFold ((fun (best : Option (ByteArray)) (inst : Inst) => (match best with | Option.none => Option.some ((inst).construct) | Option.some seen => (match bytesLess ((inst).construct) (seen) with | Bool.true => Option.some ((inst).construct) | Bool.false => Option.some (seen))))) (Option.none) (insts) : Option (ByteArray)) with | Option.some least => least | Option.none => text ("a read"))

@[expose] public def constructsOf (rows : List (ItemRow)) (e : Expr) : ByteArray := minConstruct ((foundExpr (({ insts := ([] : List (Inst)), types := ([] : List (ByteArray)), items := rows } : Found)) (e)).insts)

@[expose] public def negatesZero (e : Expr) : Bool := (match e with | Expr.ELit _ _ => false | Expr.ERead _ _ _ => false | Expr.EWrap _ _ _ => false | Expr.ECall _ _ _ _ => false | Expr.EApply _ _ _ _ => false | Expr.EConstruct _ _ _ => false | Expr.EPair _ _ _ => false | Expr.EIf _ _ _ _ => false | Expr.EMatch _ _ _ => false | Expr.EBlockE _ => false | Expr.EProbe k _ _ => probeNegatable (k) | Expr.EBlock _ _ => false | Expr.ENil => false | Expr.ECons _ _ => false | Expr.ELet _ _ _ _ _ _ => false | Expr.EArm _ _ _ => false | Expr.ELoop _ _ => false)

@[expose] public def readIsMove (kind : ReadKind) : Bool := (match kind with | ReadKind.RkMove => true | ReadKind.RkClone => false | ReadKind.RkCopy => false | ReadKind.RkDeref => false | ReadKind.RkUnbox => false)

@[expose] public def tupleMoveTarget (pat : Pat) (value : Expr) : Option (Id) := (match pat with | Pat.PLeaf _ => Option.none | Pat.PBind _ => Option.none | Pat.PTuple parts => (match value with | Expr.ELit _ _ => Option.none | Expr.ERead k who _ => (match readIsMove (k) with | Bool.true => (match patHasWild (parts) with | Bool.true => Option.some (who) | Bool.false => Option.none) | Bool.false => Option.none) | Expr.EWrap _ _ _ => Option.none | Expr.ECall _ _ _ _ => Option.none | Expr.EApply _ _ _ _ => Option.none | Expr.EConstruct _ _ _ => Option.none | Expr.EPair _ _ _ => Option.none | Expr.EIf _ _ _ _ => Option.none | Expr.EMatch _ _ _ => Option.none | Expr.EBlockE _ => Option.none | Expr.EProbe _ _ _ => Option.none | Expr.EBlock _ _ => Option.none | Expr.ENil => Option.none | Expr.ECons _ _ => Option.none | Expr.ELet _ _ _ _ _ _ => Option.none | Expr.EArm _ _ _ => Option.none | Expr.ELoop _ _ => Option.none) | Pat.PWrap _ _ => Option.none | Pat.PAdt _ _ _ => Option.none | Pat.PNil => Option.none | Pat.PCons _ _ => Option.none)

@[expose] public def conOk (cx : Cx) (con : Con) (argc : Nat) : Except (ByteArray) (Unit) := (match con with | Con.ConNone ty => tyOk (cx) (ty) | Con.ConSome => Except.ok (()) | Con.ConOk ok error => (match tyOk (cx) (ok) with | Except.ok _ => tyOk (cx) (error) | Except.error problem7 => Except.error (problem7)) | Con.ConErr ok error => (match tyOk (cx) (ok) with | Except.ok _ => tyOk (cx) (error) | Except.error problem8 => Except.error (problem8)) | Con.ConAdt adt constructor => (match adtArity ((cx).decl) (adt) (constructor) with | Option.none => Except.error (cat (text ("Adt")) (cat (decimal (adt)) (cat (text ("::C")) (cat (decimal (constructor)) (text (" is not declared")))))) | Option.some arity => (match (Nat.beq (arity) (argc)) with | Bool.true => Except.ok (()) | Bool.false => Except.error (cat (text ("Adt")) (cat (decimal (adt)) (cat (text ("::C")) (cat (decimal (constructor)) (cat (text (" receives ")) (cat (decimal (argc)) (cat (text (" fields of ")) (decimal (arity))))))))))) | Con.ConClosure fnType function => (match closureArity ((cx).decl) (fnType) (function) with | Option.none => Except.error (cat (text ("Fn")) (cat (decimal (fnType)) (cat (text ("::F")) (cat (decimal (function)) (text (" is not declared")))))) | Option.some arity => (match (Nat.beq (arity) (argc)) with | Bool.true => Except.ok (()) | Bool.false => Except.error (cat (text ("Fn")) (cat (decimal (fnType)) (cat (text ("::F")) (cat (decimal (function)) (cat (text (" captures ")) (cat (decimal (argc)) (cat (text (" values of ")) (decimal (arity))))))))))) | Con.ConCons => heapUnit (cx) (text ("a list cell")) | Con.ConNil ty => (match heapUnit (cx) (text ("an empty list")) with | Except.ok _ => tyOk (cx) (ty) | Except.error problem6 => Except.error (problem6)))

@[expose] public def conIsErr (con : Con) : Bool := (match con with | Con.ConNone _ => false | Con.ConSome => false | Con.ConOk _ _ => false | Con.ConErr _ _ => true | Con.ConAdt _ _ => false | Con.ConClosure _ _ => false | Con.ConCons => false | Con.ConNil _ => false)

@[expose] public def isOverflowExpr (e : Expr) : Bool := (match e with | Expr.ELit lit _ => isOverflowLit (lit) | Expr.ERead _ _ _ => false | Expr.EWrap _ _ _ => false | Expr.ECall _ _ _ _ => false | Expr.EApply _ _ _ _ => false | Expr.EConstruct _ _ _ => false | Expr.EPair _ _ _ => false | Expr.EIf _ _ _ _ => false | Expr.EMatch _ _ _ => false | Expr.EBlockE _ => false | Expr.EProbe _ _ _ => false | Expr.EBlock _ _ => false | Expr.ENil => false | Expr.ECons _ _ => false | Expr.ELet _ _ _ _ _ _ => false | Expr.EArm _ _ _ => false | Expr.ELoop _ _ => false)

@[expose] public def isOverflowTy (ty : Ty) : Bool := (match ty with | Ty.TyLeaf kind => (match kind with | LeafKind.LkUnit => false | LeafKind.LkBool => false | LeafKind.LkNat => false | LeafKind.LkInt => false | LeafKind.LkOrdering => false | LeafKind.LkStr => false | LeafKind.LkBytes => false | LeafKind.LkOverflow => true | LeafKind.LkI128 => false) | Ty.TyFixed _ => false | Ty.TyUnary _ _ => false | Ty.TyBinary _ _ _ => false | Ty.TyNamed _ _ => false | Ty.TyNil => false | Ty.TyCons _ _ => false)

@[expose] public def isOverflowArgs (args : Expr) : Bool := (match args with | Expr.ELit _ _ => false | Expr.ERead _ _ _ => false | Expr.EWrap _ _ _ => false | Expr.ECall _ _ _ _ => false | Expr.EApply _ _ _ _ => false | Expr.EConstruct _ _ _ => false | Expr.EPair _ _ _ => false | Expr.EIf _ _ _ _ => false | Expr.EMatch _ _ _ => false | Expr.EBlockE _ => false | Expr.EProbe _ _ _ => false | Expr.EBlock _ _ => false | Expr.ENil => false | Expr.ECons head tail => (let single : Bool := (Nat.beq (exprLen (tail)) (0)); (let overflow : Bool := isOverflowExpr (head); (single && overflow))) | Expr.ELet _ _ _ _ _ _ => false | Expr.EArm _ _ _ => false | Expr.ELoop _ _ => false)

@[expose] public def isOverflowErr (con : Con) (args : Expr) : Bool := (match con with | Con.ConNone _ => false | Con.ConSome => false | Con.ConOk _ _ => false | Con.ConErr _ errTy => (let typed : Bool := isOverflowTy (errTy); (let payload : Bool := isOverflowArgs (args); (typed && payload))) | Con.ConAdt _ _ => false | Con.ConClosure _ _ => false | Con.ConCons => false | Con.ConNil _ => false)

@[expose] public def returnsResult (e : Expr) : Bool := (match e with | Expr.ELit _ _ => false | Expr.ERead _ _ _ => false | Expr.EWrap k _ _ => (match k with | WrapKind.WkNot => false | WrapKind.WkBox => false | WrapKind.WkWiden => false | WrapKind.WkSucceed => true) | Expr.ECall _ _ propagate _ => (if propagate then false else true) | Expr.EApply _ _ propagate _ => (if propagate then false else true) | Expr.EConstruct con args _ => isOverflowErr (con) (args) | Expr.EPair _ _ _ => false | Expr.EIf _ _ _ _ => true | Expr.EMatch _ _ _ => true | Expr.EBlockE _ => true | Expr.EProbe _ _ _ => false | Expr.EBlock _ _ => false | Expr.ENil => false | Expr.ECons _ _ => false | Expr.ELet _ _ _ _ _ _ => false | Expr.EArm _ _ _ => false | Expr.ELoop _ _ => true)

@[expose] public def idLess (left : Id) (right : Id) : Bool := (if (Nat.beq (idTag (left)) (idTag (right))) then (if (Nat.beq (idTag (left)) (9)) then bytesLess (idName (left)) (idName (right)) else (Nat.blt (idNum (left)) (idNum (right)))) else (Nat.blt (idTag (left)) (idTag (right))))

@[expose] public def dropId (items : List (Id)) (who : Id) : List (Id) := (LexLeanCollections.listFold ((fun (kept : List (Id)) (item : Id) => (let same : Bool := idEq (item) (who); (if same then kept else (item :: kept))))) (([] : List (Id))) (items) : List (Id))

@[expose] public def natsWithin (left : List (Nat)) (right : List (Nat)) : Bool := (LexLeanCollections.listFold ((fun (all : Bool) (item : Nat) => (let present : Bool := natHas (right) (item); (if all then present else false)))) (true) (left) : Bool)

@[expose] public def partsSame (entry : List ((Prod (Id) (List (Nat))))) (who : Id) (parts : List (Nat)) : Bool := (match partsGet (entry) (who) with | Option.some «have» => (let forward : Bool := natsWithin (parts) («have»); (let back : Bool := natsWithin («have») (parts); (if forward then back else false))) | Option.none => false)

@[expose] public def lesserId (best : Option (Id)) (who : Id) : Option (Id) := (match best with | Option.some seen => (match idLess (who) (seen) with | Bool.true => Option.some (who) | Bool.false => Option.some (seen)) | Option.none => Option.some (who))

@[expose] public def movedInLoop (after : List (Id)) (state : Id) (entry : List (Id)) (outside : List (Id)) : Option (Id) := (LexLeanCollections.listFold ((fun (best : Option (Id)) (item : Id) => (let same : Bool := idEq (item) (state); (let kept : Bool := idHas (entry) (item); (let seen : Bool := idHas (outside) (item); (let offends : Bool := (if same then false else (if kept then false else seen)); (if offends then lesserId (best) (item) else best))))))) (Option.none) (after) : Option (Id))

@[expose] public def partedInLoop (after : List ((Prod (Id) (List (Nat))))) (entry : List ((Prod (Id) (List (Nat))))) (outside : List (Id)) : Option (Id) := (LexLeanCollections.listFold ((fun (best : Option (Id)) (item : (Prod (Id) (List (Nat)))) => (let seen : Bool := idHas (outside) ((item).1); (let same : Bool := partsSame (entry) ((item).1) ((item).2); (let offends : Bool := (if seen then (if same then false else true) else false); (if offends then lesserId (best) ((item).1) else best)))))) (Option.none) (after) : Option (Id))

@[expose] public def endsArgs : (rows : List (ItemRow)) -> (args : Expr) -> Except (ByteArray) (Unit)
  | _rows, Expr.ELit _ _ => Except.ok (())
  | _rows, Expr.ERead _ _ _ => Except.ok (())
  | _rows, Expr.EWrap _ _ _ => Except.ok (())
  | _rows, Expr.ECall _ _ _ _ => Except.ok (())
  | _rows, Expr.EApply _ _ _ _ => Except.ok (())
  | _rows, Expr.EConstruct _ _ _ => Except.ok (())
  | _rows, Expr.EPair _ _ _ => Except.ok (())
  | _rows, Expr.EIf _ _ _ _ => Except.ok (())
  | _rows, Expr.EMatch _ _ _ => Except.ok (())
  | _rows, Expr.EBlockE _ => Except.ok (())
  | _rows, Expr.EProbe _ _ _ => Except.ok (())
  | _rows, Expr.EBlock _ _ => Except.ok (())
  | _rows, Expr.ENil => Except.ok (())
  | rows, Expr.ECons head rest => (match returnsResult (head) with | Bool.true => endsArgs (rows) (rest) | Bool.false => Except.error (cat (text ("arithmetic: the value a fallible loop ends with, ")) (cat (constructsOf (rows) (head)) (text (", is not a fallible value")))))
  | _rows, Expr.ELet _ _ _ _ _ _ => Except.ok (())
  | _rows, Expr.EArm _ _ _ => Except.ok (())
  | _rows, Expr.ELoop _ _ => Except.ok (())

mutual
@[expose] public def endsBlock : (rows : List (ItemRow)) -> (b : Expr) -> Except (ByteArray) (Unit)
  | _rows, Expr.ELit _ _ => Except.ok (())
  | _rows, Expr.ERead _ _ _ => Except.ok (())
  | _rows, Expr.EWrap _ _ _ => Except.ok (())
  | _rows, Expr.ECall _ _ _ _ => Except.ok (())
  | _rows, Expr.EApply _ _ _ _ => Except.ok (())
  | _rows, Expr.EConstruct _ _ _ => Except.ok (())
  | _rows, Expr.EPair _ _ _ => Except.ok (())
  | _rows, Expr.EIf _ _ _ _ => Except.ok (())
  | _rows, Expr.EMatch _ _ _ => Except.ok (())
  | _rows, Expr.EBlockE _ => Except.ok (())
  | _rows, Expr.EProbe _ _ _ => Except.ok (())
  | rows, Expr.EBlock _ bodyTail => endsTail (rows) (bodyTail)
  | _rows, Expr.ENil => Except.ok (())
  | _rows, Expr.ECons _ _ => Except.ok (())
  | _rows, Expr.ELet _ _ _ _ _ _ => Except.ok (())
  | _rows, Expr.EArm _ _ _ => Except.ok (())
  | _rows, Expr.ELoop _ _ => Except.ok (())
termination_by structural _ b => b

@[expose] public def endsTail : (rows : List (ItemRow)) -> (t : Expr) -> Except (ByteArray) (Unit)
  | _rows, Expr.ELit _ _ => Except.ok (())
  | _rows, Expr.ERead _ _ _ => Except.ok (())
  | _rows, Expr.EWrap _ _ _ => Except.ok (())
  | _rows, Expr.ECall _ _ _ _ => Except.ok (())
  | _rows, Expr.EApply _ _ _ _ => Except.ok (())
  | rows, Expr.EConstruct con args _ => (match conIsErr (con) with | Bool.true => endsArgs (rows) (args) | Bool.false => Except.ok (()))
  | _rows, Expr.EPair _ _ _ => Except.ok (())
  | rows, Expr.EIf _ thenBranch elseBranch _ => (match endsBlock (rows) (thenBranch) with | Except.ok _ => endsBlock (rows) (elseBranch) | Except.error problem52 => Except.error (problem52))
  | rows, Expr.EMatch _ arms _ => endsArms (rows) (arms)
  | rows, Expr.EBlockE inner => endsBlock (rows) (inner)
  | _rows, Expr.EProbe _ _ _ => Except.ok (())
  | _rows, Expr.EBlock _ _ => Except.ok (())
  | _rows, Expr.ENil => Except.ok (())
  | _rows, Expr.ECons _ _ => Except.ok (())
  | _rows, Expr.ELet _ _ _ _ _ _ => Except.ok (())
  | _rows, Expr.EArm _ _ _ => Except.ok (())
  | _rows, Expr.ELoop _ _ => Except.ok (())
termination_by structural _ t => t

@[expose] public def endsArms : (rows : List (ItemRow)) -> (arms : Expr) -> Except (ByteArray) (Unit)
  | _rows, Expr.ELit _ _ => Except.ok (())
  | _rows, Expr.ERead _ _ _ => Except.ok (())
  | _rows, Expr.EWrap _ _ _ => Except.ok (())
  | _rows, Expr.ECall _ _ _ _ => Except.ok (())
  | _rows, Expr.EApply _ _ _ _ => Except.ok (())
  | _rows, Expr.EConstruct _ _ _ => Except.ok (())
  | _rows, Expr.EPair _ _ _ => Except.ok (())
  | _rows, Expr.EIf _ _ _ _ => Except.ok (())
  | _rows, Expr.EMatch _ _ _ => Except.ok (())
  | _rows, Expr.EBlockE _ => Except.ok (())
  | _rows, Expr.EProbe _ _ _ => Except.ok (())
  | _rows, Expr.EBlock _ _ => Except.ok (())
  | _rows, Expr.ENil => Except.ok (())
  | _rows, Expr.ECons _ _ => Except.ok (())
  | _rows, Expr.ELet _ _ _ _ _ _ => Except.ok (())
  | rows, Expr.EArm _ body rest => (match endsBlock (rows) (body) with | Except.ok _ => endsArms (rows) (rest) | Except.error problem53 => Except.error (problem53))
  | _rows, Expr.ELoop _ _ => Except.ok (())
termination_by structural _ arms => arms
end

@[expose] public def machineName (m : Nat) : ByteArray := (if (Nat.beq (m) (0)) then text ("checked_add") else (if (Nat.beq (m) (1)) then text ("checked_sub") else (if (Nat.beq (m) (2)) then text ("checked_mul") else (if (Nat.beq (m) (3)) then text ("checked_div") else (if (Nat.beq (m) (4)) then text ("checked_neg") else (if (Nat.beq (m) (5)) then text ("checked_shl") else (if (Nat.beq (m) (6)) then text ("checked_shr") else (if (Nat.beq (m) (7)) then text ("convert") else (if (Nat.beq (m) (8)) then text ("bit_and") else (if (Nat.beq (m) (9)) then text ("bit_or") else (if (Nat.beq (m) (10)) then text ("bit_xor") else (if (Nat.beq (m) (11)) then text ("bit_not") else (if (Nat.beq (m) (12)) then text ("nat_eq") else (if (Nat.beq (m) (13)) then text ("nat_le") else (if (Nat.beq (m) (14)) then text ("nat_lt") else (if (Nat.beq (m) (15)) then text ("nat_rem") else (if (Nat.beq (m) (16)) then text ("int_rem") else (if (Nat.beq (m) (17)) then text ("nat_add") else (if (Nat.beq (m) (18)) then text ("nat_sub") else (if (Nat.beq (m) (19)) then text ("nat_mul") else (if (Nat.beq (m) (20)) then text ("nat_quot") else (if (Nat.beq (m) (21)) then text ("int_add") else (if (Nat.beq (m) (22)) then text ("int_sub") else (if (Nat.beq (m) (23)) then text ("int_mul") else (if (Nat.beq (m) (24)) then text ("int_neg") else (if (Nat.beq (m) (25)) then text ("int_quot") else (if (Nat.beq (m) (26)) then text ("list_length") else (if (Nat.beq (m) (27)) then text ("list_append") else (if (Nat.beq (m) (28)) then text ("list_index") else (if (Nat.beq (m) (29)) then text ("list_slice") else (if (Nat.beq (m) (30)) then text ("bytes_append") else (if (Nat.beq (m) (31)) then text ("bytes_length") else (if (Nat.beq (m) (32)) then text ("string_length") else (if (Nat.beq (m) (33)) then text ("bytes_index") else (if (Nat.beq (m) (34)) then text ("bytes_slice") else (if (Nat.beq (m) (35)) then text ("utf8_encode") else (if (Nat.beq (m) (36)) then text ("utf8_decode") else (if (Nat.beq (m) (37)) then text ("bytes_compare") else (if (Nat.beq (m) (38)) then text ("string_split") else (if (Nat.beq (m) (39)) then text ("string_join") else _root_.ByteArray.mk #[]))))))))))))))))))))))))))))))))))))))))

@[expose] public def machineArity (m : Nat) : Nat := (if (Nat.beq (m) (0)) then 2 else (if (Nat.beq (m) (1)) then 2 else (if (Nat.beq (m) (2)) then 2 else (if (Nat.beq (m) (3)) then 2 else (if (Nat.beq (m) (4)) then 1 else (if (Nat.beq (m) (5)) then 2 else (if (Nat.beq (m) (6)) then 2 else (if (Nat.beq (m) (7)) then 1 else (if (Nat.beq (m) (8)) then 2 else (if (Nat.beq (m) (9)) then 2 else (if (Nat.beq (m) (10)) then 2 else (if (Nat.beq (m) (11)) then 1 else (if (Nat.beq (m) (12)) then 2 else (if (Nat.beq (m) (13)) then 2 else (if (Nat.beq (m) (14)) then 2 else (if (Nat.beq (m) (15)) then 3 else (if (Nat.beq (m) (16)) then 3 else (if (Nat.beq (m) (17)) then 2 else (if (Nat.beq (m) (18)) then 2 else (if (Nat.beq (m) (19)) then 2 else (if (Nat.beq (m) (20)) then 3 else (if (Nat.beq (m) (21)) then 2 else (if (Nat.beq (m) (22)) then 2 else (if (Nat.beq (m) (23)) then 2 else (if (Nat.beq (m) (24)) then 1 else (if (Nat.beq (m) (25)) then 3 else (if (Nat.beq (m) (26)) then 1 else (if (Nat.beq (m) (27)) then 2 else (if (Nat.beq (m) (28)) then 2 else (if (Nat.beq (m) (29)) then 3 else (if (Nat.beq (m) (30)) then 2 else (if (Nat.beq (m) (31)) then 1 else (if (Nat.beq (m) (32)) then 1 else (if (Nat.beq (m) (33)) then 2 else (if (Nat.beq (m) (34)) then 3 else (if (Nat.beq (m) (35)) then 1 else (if (Nat.beq (m) (36)) then 1 else (if (Nat.beq (m) (37)) then 2 else (if (Nat.beq (m) (38)) then 3 else (if (Nat.beq (m) (39)) then 2 else 0))))))))))))))))))))))))))))))))))))))))

@[expose] public def machineCallC (st : St) (cx : Cx) (m : Nat) (args : Expr) (propagate : Bool) (tail : Bool) : Except (ByteArray) (St) := (match (Nat.beq (exprLen (args)) (machineArity (m))) with | Bool.true => callOk (st) (cx) (false) (propagate) (tail) (tick (machineName (m))) | Bool.false => Except.error (cat (text ("machine: ")) (cat (tick (machineName (m))) (cat (text (" receives ")) (cat (decimal (exprLen (args))) (cat (text (" operands of ")) (decimal (machineArity (m)))))))))

mutual
@[expose] public def blockC : (st : St) -> (cx : Cx) -> (b : Expr) -> (tail : Bool) -> Except (ByteArray) (St)
  | st, _cx, Expr.ELit _ _, _tail => Except.ok (st)
  | st, _cx, Expr.ERead _ _ _, _tail => Except.ok (st)
  | st, _cx, Expr.EWrap _ _ _, _tail => Except.ok (st)
  | st, _cx, Expr.ECall _ _ _ _, _tail => Except.ok (st)
  | st, _cx, Expr.EApply _ _ _ _, _tail => Except.ok (st)
  | st, _cx, Expr.EConstruct _ _ _, _tail => Except.ok (st)
  | st, _cx, Expr.EPair _ _ _, _tail => Except.ok (st)
  | st, _cx, Expr.EIf _ _ _ _, _tail => Except.ok (st)
  | st, _cx, Expr.EMatch _ _ _, _tail => Except.ok (st)
  | st, _cx, Expr.EBlockE _, _tail => Except.ok (st)
  | st, _cx, Expr.EProbe _ _ _, _tail => Except.ok (st)
  | st, cx, Expr.EBlock lets bodyTail, tail => (let saved : List (Id) := (st).scope; (match letsC (st) (cx) (lets) with | Except.ok s1 => (match exprC (s1) (cx) (bodyTail) (tail) with | Except.ok s2 => Except.ok (({ bound := (s2).bound, scope := saved, moved := (s2).moved, parted := (s2).parted, propagates := (s2).propagates } : St)) | Except.error problem33 => Except.error (problem33)) | Except.error problem34 => Except.error (problem34)))
  | st, _cx, Expr.ENil, _tail => Except.ok (st)
  | st, _cx, Expr.ECons _ _, _tail => Except.ok (st)
  | st, _cx, Expr.ELet _ _ _ _ _ _, _tail => Except.ok (st)
  | st, _cx, Expr.EArm _ _ _, _tail => Except.ok (st)
  | st, _cx, Expr.ELoop _ _, _tail => Except.ok (st)
termination_by structural _ _ b _ => b

@[expose] public def exprC : (st : St) -> (cx : Cx) -> (e : Expr) -> (tail : Bool) -> Except (ByteArray) (St)
  | st, cx, Expr.ELit lit org, tail => (let gated : Bool := (if tail then (cx).fallible else false); (match gated with | Bool.true => Except.error (cat (text ("arithmetic: the value a fallible function returns, ")) (cat (constructsOf ((cx).items) (Expr.ELit (lit) (org))) (text (", is not a fallible value")))) | Bool.false => (match lit with | Lit.LitUnit => Except.ok (st) | Lit.LitBool => Except.ok (st) | Lit.LitNat => Except.ok (st) | Lit.LitInt => Except.ok (st) | Lit.LitFixed _ => Except.ok (st) | Lit.LitStr => heapSt (cx) (st) (text ("a string literal")) | Lit.LitBytes => heapSt (cx) (st) (text ("a byte string literal")) | Lit.LitOrdering => Except.ok (st) | Lit.LitOverflow => Except.error (text ("overflow: the overflow value stands outside the payload of an `Err`")))))
  | st, cx, Expr.ERead k who org, tail => (let gated : Bool := (if tail then (cx).fallible else false); (match gated with | Bool.true => Except.error (cat (text ("arithmetic: the value a fallible function returns, ")) (cat (constructsOf ((cx).items) (Expr.ERead (k) (who) (org))) (text (", is not a fallible value")))) | Bool.false => (match k with | ReadKind.RkMove => consume (st) (who) | ReadKind.RkClone => (match readOk (st) (who) with | Except.ok _ => Except.ok (st) | Except.error problem9 => Except.error (problem9)) | ReadKind.RkCopy => (match readOk (st) (who) with | Except.ok _ => Except.ok (st) | Except.error problem10 => Except.error (problem10)) | ReadKind.RkDeref => (match readOk (st) (who) with | Except.ok _ => Except.ok (st) | Except.error problem11 => Except.error (problem11)) | ReadKind.RkUnbox => (match heapUnit (cx) (text ("a boxed read")) with | Except.ok _ => (match readOk (st) (who) with | Except.ok _ => Except.ok (st) | Except.error problem12 => Except.error (problem12)) | Except.error problem13 => Except.error (problem13)))))
  | st, cx, Expr.EWrap k inner org, tail => (match k with | WrapKind.WkNot => (let gated : Bool := (if tail then (cx).fallible else false); (match gated with | Bool.true => Except.error (cat (text ("arithmetic: the value a fallible function returns, ")) (cat (constructsOf ((cx).items) (Expr.EWrap (k) (inner) (org))) (text (", is not a fallible value")))) | Bool.false => (match negatesZero (inner) with | Bool.true => Except.error (text ("a negated zero test or predecessor: a zero test is negated as its complement, and a predecessor is not a Boolean")) | Bool.false => exprC (st) (cx) (inner) (false)))) | WrapKind.WkBox => (let gated : Bool := (if tail then (cx).fallible else false); (match gated with | Bool.true => Except.error (cat (text ("arithmetic: the value a fallible function returns, ")) (cat (constructsOf ((cx).items) (Expr.EWrap (k) (inner) (org))) (text (", is not a fallible value")))) | Bool.false => (match heapUnit (cx) (text ("a box")) with | Except.ok _ => exprC (st) (cx) (inner) (false) | Except.error problem14 => Except.error (problem14)))) | WrapKind.WkWiden => (let gated : Bool := (if tail then (cx).fallible else false); (match gated with | Bool.true => Except.error (cat (text ("arithmetic: the value a fallible function returns, ")) (cat (constructsOf ((cx).items) (Expr.EWrap (k) (inner) (org))) (text (", is not a fallible value")))) | Bool.false => exprC (st) (cx) (inner) (false))) | WrapKind.WkSucceed => exprC (st) (cx) (inner) (false))
  | st, cx, Expr.ECall callee args propagate org, tail => (match propagate with | Bool.true => (let gated : Bool := (if tail then (cx).fallible else false); (match gated with | Bool.true => Except.error (cat (text ("arithmetic: the value a fallible function returns, ")) (cat (constructsOf ((cx).items) (Expr.ECall (callee) (args) (propagate) (org))) (text (", is not a fallible value")))) | Bool.false => (match argsC (st) (cx) (false) (args) with | Except.ok s1 => (match callee with | Callee.CalFn n => (match lookupFlag (((cx).decl).functions) (n) with | Option.none => Except.error (cat (text ("f")) (cat (decimal (n)) (text (" is not declared")))) | Option.some declared => callOk (s1) (cx) (declared) (propagate) (tail) (cat (text ("f")) (decimal (n)))) | Callee.CalMachine m => machineCallC (s1) (cx) (m) (args) (propagate) (tail) | Callee.CalRt item => (let path : ByteArray := itemPath ((cx).items) (item); (match (match itemHeap ((cx).items) (item) with | Bool.true => heapUnit (cx) (cat (text ("the runtime function ")) (tick (path))) | Bool.false => Except.ok (())) with | Except.ok _ => callOk (s1) (cx) (itemFallible ((cx).items) (item)) (propagate) (tail) (tick (path)) | Except.error problem15 => Except.error (problem15)))) | Except.error problem16 => Except.error (problem16)))) | Bool.false => (match argsC (st) (cx) (false) (args) with | Except.ok s1 => (match callee with | Callee.CalFn n => (match lookupFlag (((cx).decl).functions) (n) with | Option.none => Except.error (cat (text ("f")) (cat (decimal (n)) (text (" is not declared")))) | Option.some declared => callOk (s1) (cx) (declared) (propagate) (tail) (cat (text ("f")) (decimal (n)))) | Callee.CalMachine m => machineCallC (s1) (cx) (m) (args) (propagate) (tail) | Callee.CalRt item => (let path : ByteArray := itemPath ((cx).items) (item); (match (match itemHeap ((cx).items) (item) with | Bool.true => heapUnit (cx) (cat (text ("the runtime function ")) (tick (path))) | Bool.false => Except.ok (())) with | Except.ok _ => callOk (s1) (cx) (itemFallible ((cx).items) (item)) (propagate) (tail) (tick (path)) | Except.error problem15 => Except.error (problem15)))) | Except.error problem16 => Except.error (problem16)))
  | st, cx, Expr.EApply who args propagate org, tail => (match propagate with | Bool.true => (let gated : Bool := (if tail then (cx).fallible else false); (match gated with | Bool.true => Except.error (cat (text ("arithmetic: the value a fallible function returns, ")) (cat (constructsOf ((cx).items) (Expr.EApply (who) (args) (propagate) (org))) (text (", is not a fallible value")))) | Bool.false => (match readOk (st) (who) with | Except.ok _ => (match argsC (st) (cx) (false) (args) with | Except.ok s1 => Except.ok (({ bound := (s1).bound, scope := (s1).scope, moved := (s1).moved, parted := (s1).parted, propagates := (if propagate then true else (s1).propagates) } : St)) | Except.error problem17 => Except.error (problem17)) | Except.error problem18 => Except.error (problem18)))) | Bool.false => (match readOk (st) (who) with | Except.ok _ => (match argsC (st) (cx) (false) (args) with | Except.ok s1 => Except.ok (({ bound := (s1).bound, scope := (s1).scope, moved := (s1).moved, parted := (s1).parted, propagates := (if propagate then true else (s1).propagates) } : St)) | Except.error problem17 => Except.error (problem17)) | Except.error problem18 => Except.error (problem18)))
  | st, cx, Expr.EConstruct con args org, tail => (let gated : Bool := (if tail then (if (cx).fallible then (!isOverflowErr (con) (args)) else false) else false); (match gated with | Bool.true => Except.error (cat (text ("arithmetic: the value a fallible function returns, ")) (cat (constructsOf ((cx).items) (Expr.EConstruct (con) (args) (org))) (text (", is not a fallible value")))) | Bool.false => (match argsC (st) (cx) (conIsErr (con)) (args) with | Except.ok s1 => (match conOk (cx) (con) (exprLen (args)) with | Except.ok _ => Except.ok (s1) | Except.error problem19 => Except.error (problem19)) | Except.error problem20 => Except.error (problem20))))
  | st, cx, Expr.EPair left right org, tail => (let gated : Bool := (if tail then (cx).fallible else false); (match gated with | Bool.true => Except.error (cat (text ("arithmetic: the value a fallible function returns, ")) (cat (constructsOf ((cx).items) (Expr.EPair (left) (right) (org))) (text (", is not a fallible value")))) | Bool.false => (match exprC (st) (cx) (left) (false) with | Except.ok s1 => exprC (s1) (cx) (right) (false) | Except.error problem21 => Except.error (problem21))))
  | st, cx, Expr.EIf condition thenBranch elseBranch _, tail => (match exprC (st) (cx) (condition) (false) with | Except.ok s1 => (let bThen0 : St := ({ bound := (s1).bound, scope := (s1).scope, moved := (s1).moved, parted := (s1).parted, propagates := (s1).propagates } : St); (match bindPat (bThen0) ((cx).decl) (Pat.PLeaf (PatLeaf.PlWild)) with | Except.ok bThen2 => (match blockC (bThen2) (cx) (thenBranch) (tail) with | Except.ok bThen3 => (let r1 : St := ({ bound := (bThen3).bound, scope := (s1).scope, moved := unionIds ((s1).moved) ((bThen3).moved), parted := unionParts ((s1).parted) ((bThen3).parted), propagates := (bThen3).propagates } : St); (let bElse0 : St := ({ bound := (r1).bound, scope := (s1).scope, moved := (s1).moved, parted := (s1).parted, propagates := (r1).propagates } : St); (match bindPat (bElse0) ((cx).decl) (Pat.PLeaf (PatLeaf.PlWild)) with | Except.ok bElse2 => (match blockC (bElse2) (cx) (elseBranch) (tail) with | Except.ok bElse3 => (let r2 : St := ({ bound := (bElse3).bound, scope := (s1).scope, moved := unionIds ((r1).moved) ((bElse3).moved), parted := unionParts ((r1).parted) ((bElse3).parted), propagates := (bElse3).propagates } : St); Except.ok (r2)) | Except.error problem24 => Except.error (problem24)) | Except.error problem25 => Except.error (problem25)))) | Except.error problem22 => Except.error (problem22)) | Except.error problem23 => Except.error (problem23))) | Except.error problem26 => Except.error (problem26))
  | st, cx, Expr.EMatch scrutinee arms _, tail => (match exprC (st) (cx) (scrutinee) (false) with | Except.ok s1 => armsC (s1) (cx) (s1) ((s1).scope) (arms) (tail) | Except.error problem27 => Except.error (problem27))
  | st, cx, Expr.EBlockE inner, tail => blockC (st) (cx) (inner) (tail)
  | st, cx, Expr.EProbe k who org, tail => (let gated : Bool := (if tail then (cx).fallible else false); (match gated with | Bool.true => Except.error (cat (text ("arithmetic: the value a fallible function returns, ")) (cat (constructsOf ((cx).items) (Expr.EProbe (k) (who) (org))) (text (", is not a fallible value")))) | Bool.false => (match k with | ProbeKind.PkIsZero => (match readOk (st) (who) with | Except.ok _ => Except.ok (st) | Except.error problem30 => Except.error (problem30)) | ProbeKind.PkNonZero => (match readOk (st) (who) with | Except.ok _ => Except.ok (st) | Except.error problem31 => Except.error (problem31)) | ProbeKind.PkUncons => (match heapUnit (cx) (text ("a list match")) with | Except.ok _ => (match readOk (st) (who) with | Except.ok _ => Except.ok (st) | Except.error problem28 => Except.error (problem28)) | Except.error problem29 => Except.error (problem29)) | ProbeKind.PkPred => (match readOk (st) (who) with | Except.ok _ => Except.ok (st) | Except.error problem32 => Except.error (problem32)))))
  | st, _cx, Expr.EBlock _ _, _tail => Except.ok (st)
  | st, _cx, Expr.ENil, _tail => Except.ok (st)
  | st, _cx, Expr.ECons _ _, _tail => Except.ok (st)
  | st, _cx, Expr.ELet _ _ _ _ _ _, _tail => Except.ok (st)
  | st, _cx, Expr.EArm _ _ _, _tail => Except.ok (st)
  | st, cx, Expr.ELoop state body, tail => (let gated : Bool := (if tail then (cx).fallible else false); (let ends : Except (ByteArray) (Unit) := (match gated with | Bool.true => endsBlock ((cx).items) (body) | Bool.false => Except.ok (())); (match ends with | Except.ok _ => (match consume (st) (state) with | Except.ok s1 => (let entry : St := ({ bound := (s1).bound, scope := (s1).scope, moved := dropId ((s1).moved) (state), parted := (s1).parted, propagates := (s1).propagates } : St); (match blockC (entry) (cx) (body) (false) with | Except.ok s2 => (match movedInLoop ((s2).moved) (state) ((entry).moved) ((s1).scope) with | Option.some who => Except.error (cat (text ("ownership: ")) (cat (tick (idText (who))) (text (" is moved in the body of a loop that runs again")))) | Option.none => (match partedInLoop ((s2).parted) ((entry).parted) ((s1).scope) with | Option.some who2 => Except.error (cat (text ("ownership: a component of ")) (cat (tick (idText (who2))) (text (" is moved in the body of a loop that runs again")))) | Option.none => Except.ok (({ bound := (s2).bound, scope := (s2).scope, moved := (s1).moved, parted := (s1).parted, propagates := (s2).propagates } : St)))) | Except.error problem51 => Except.error (problem51))) | Except.error problem50 => Except.error (problem50)) | Except.error problem49 => Except.error (problem49))))
termination_by structural _ _ e _ => e

@[expose] public def letsC : (st : St) -> (cx : Cx) -> (lets : Expr) -> Except (ByteArray) (St)
  | st, _cx, Expr.ELit _ _ => Except.ok (st)
  | st, _cx, Expr.ERead _ _ _ => Except.ok (st)
  | st, _cx, Expr.EWrap _ _ _ => Except.ok (st)
  | st, _cx, Expr.ECall _ _ _ _ => Except.ok (st)
  | st, _cx, Expr.EApply _ _ _ _ => Except.ok (st)
  | st, _cx, Expr.EConstruct _ _ _ => Except.ok (st)
  | st, _cx, Expr.EPair _ _ _ => Except.ok (st)
  | st, _cx, Expr.EIf _ _ _ _ => Except.ok (st)
  | st, _cx, Expr.EMatch _ _ _ => Except.ok (st)
  | st, _cx, Expr.EBlockE _ => Except.ok (st)
  | st, _cx, Expr.EProbe _ _ _ => Except.ok (st)
  | st, _cx, Expr.EBlock _ _ => Except.ok (st)
  | st, _cx, Expr.ENil => Except.ok (st)
  | st, _cx, Expr.ECons _ _ => Except.ok (st)
  | st, cx, Expr.ELet pat hasTy ty value _ rest => (match (match hasTy with | Bool.true => tyOk (cx) (ty) | Bool.false => Except.ok (())) with | Except.ok _ => (match (match tupleMoveTarget (pat) (value) with | Option.some who => (match pat with | Pat.PLeaf _ => exprC (st) (cx) (value) (false) | Pat.PBind _ => exprC (st) (cx) (value) (false) | Pat.PTuple parts => consumeParts (st) (who) (parts) | Pat.PWrap _ _ => exprC (st) (cx) (value) (false) | Pat.PAdt _ _ _ => exprC (st) (cx) (value) (false) | Pat.PNil => exprC (st) (cx) (value) (false) | Pat.PCons _ _ => exprC (st) (cx) (value) (false)) | Option.none => exprC (st) (cx) (value) (false)) with | Except.ok s1 => (match bindPat (s1) ((cx).decl) (pat) with | Except.ok s2 => letsC (s2) (cx) (rest) | Except.error problem35 => Except.error (problem35)) | Except.error problem36 => Except.error (problem36)) | Except.error problem37 => Except.error (problem37))
  | st, _cx, Expr.EArm _ _ _ => Except.ok (st)
  | st, _cx, Expr.ELoop _ _ => Except.ok (st)
termination_by structural _ _ lets => lets

@[expose] public def argsC : (st : St) -> (cx : Cx) -> (err : Bool) -> (args : Expr) -> Except (ByteArray) (St)
  | st, _cx, _err, Expr.ELit _ _ => Except.ok (st)
  | st, _cx, _err, Expr.ERead _ _ _ => Except.ok (st)
  | st, _cx, _err, Expr.EWrap _ _ _ => Except.ok (st)
  | st, _cx, _err, Expr.ECall _ _ _ _ => Except.ok (st)
  | st, _cx, _err, Expr.EApply _ _ _ _ => Except.ok (st)
  | st, _cx, _err, Expr.EConstruct _ _ _ => Except.ok (st)
  | st, _cx, _err, Expr.EPair _ _ _ => Except.ok (st)
  | st, _cx, _err, Expr.EIf _ _ _ _ => Except.ok (st)
  | st, _cx, _err, Expr.EMatch _ _ _ => Except.ok (st)
  | st, _cx, _err, Expr.EBlockE _ => Except.ok (st)
  | st, _cx, _err, Expr.EProbe _ _ _ => Except.ok (st)
  | st, _cx, _err, Expr.EBlock _ _ => Except.ok (st)
  | st, _cx, _err, Expr.ENil => Except.ok (st)
  | st, cx, err, Expr.ECons head tail => (let skip : Bool := (if err then isOverflowExpr (head) else false); (match skip with | Bool.true => argsC (st) (cx) (err) (tail) | Bool.false => (match exprC (st) (cx) (head) (false) with | Except.ok s1 => argsC (s1) (cx) (err) (tail) | Except.error problem38 => Except.error (problem38))))
  | st, _cx, _err, Expr.ELet _ _ _ _ _ _ => Except.ok (st)
  | st, _cx, _err, Expr.EArm _ _ _ => Except.ok (st)
  | st, _cx, _err, Expr.ELoop _ _ => Except.ok (st)
termination_by structural _ _ _ args => args

@[expose] public def armsC : (before : St) -> (cx : Cx) -> (acc : St) -> (fork : List (Id)) -> (arms : Expr) -> (tail : Bool) -> Except (ByteArray) (St)
  | _before, _cx, acc, _fork, Expr.ELit _ _, _tail => Except.ok (acc)
  | _before, _cx, acc, _fork, Expr.ERead _ _ _, _tail => Except.ok (acc)
  | _before, _cx, acc, _fork, Expr.EWrap _ _ _, _tail => Except.ok (acc)
  | _before, _cx, acc, _fork, Expr.ECall _ _ _ _, _tail => Except.ok (acc)
  | _before, _cx, acc, _fork, Expr.EApply _ _ _ _, _tail => Except.ok (acc)
  | _before, _cx, acc, _fork, Expr.EConstruct _ _ _, _tail => Except.ok (acc)
  | _before, _cx, acc, _fork, Expr.EPair _ _ _, _tail => Except.ok (acc)
  | _before, _cx, acc, _fork, Expr.EIf _ _ _ _, _tail => Except.ok (acc)
  | _before, _cx, acc, _fork, Expr.EMatch _ _ _, _tail => Except.ok (acc)
  | _before, _cx, acc, _fork, Expr.EBlockE _, _tail => Except.ok (acc)
  | _before, _cx, acc, _fork, Expr.EProbe _ _ _, _tail => Except.ok (acc)
  | _before, _cx, acc, _fork, Expr.EBlock _ _, _tail => Except.ok (acc)
  | _before, _cx, acc, _fork, Expr.ENil, _tail => Except.ok (acc)
  | _before, _cx, acc, _fork, Expr.ECons _ _, _tail => Except.ok (acc)
  | _before, _cx, acc, _fork, Expr.ELet _ _ _ _ _ _, _tail => Except.ok (acc)
  | before, cx, acc, fork, Expr.EArm pat body rest, tail => (let bArm0 : St := ({ bound := (acc).bound, scope := fork, moved := (before).moved, parted := (before).parted, propagates := (acc).propagates } : St); (match bindPat (bArm0) ((cx).decl) (pat) with | Except.ok bArm2 => (match blockC (bArm2) (cx) (body) (tail) with | Except.ok bArm3 => (let r : St := ({ bound := (bArm3).bound, scope := fork, moved := unionIds ((acc).moved) ((bArm3).moved), parted := unionParts ((acc).parted) ((bArm3).parted), propagates := (bArm3).propagates } : St); armsC (before) (cx) (r) (fork) (rest) (tail)) | Except.error problem39 => Except.error (problem39)) | Except.error problem40 => Except.error (problem40)))
  | _before, _cx, acc, _fork, Expr.ELoop _ _, _tail => Except.ok (acc)
termination_by structural _ _ _ _ arms _ => arms
end

@[expose] public def typesGet (types : List ((Prod (Id) (Nat)))) (who : Id) : Option (Nat) := (LexLeanCollections.listFold ((fun (found : Option (Nat)) (entry : (Prod (Id) (Nat))) => (match found with | Option.some value => Option.some (value) | Option.none => (let same : Bool := idEq ((entry).1) (who); (if same then Option.some ((entry).2) else Option.none))))) (Option.none) (types) : Option (Nat))

@[expose] public def patBindId (pat : Pat) : Option (Id) := (match pat with | Pat.PLeaf _ => Option.none | Pat.PBind who => Option.some (who) | Pat.PTuple _ => Option.none | Pat.PWrap _ _ => Option.none | Pat.PAdt _ _ _ => Option.none | Pat.PNil => Option.none | Pat.PCons _ _ => Option.none)

@[expose] public def tyFnIndex (ty : Ty) : Option (Nat) := (match ty with | Ty.TyLeaf _ => Option.none | Ty.TyFixed _ => Option.none | Ty.TyUnary _ _ => Option.none | Ty.TyBinary _ _ _ => Option.none | Ty.TyNamed k n => (match nameIsFn (k) with | Bool.true => Option.some (n) | Bool.false => Option.none) | Ty.TyNil => Option.none | Ty.TyCons _ _ => Option.none)

@[expose] public def typesWith (types : List ((Prod (Id) (Nat)))) (pat : Pat) (ty : Ty) : List ((Prod (Id) (Nat))) := (match patBindId (pat) with | Option.some who => (match tyFnIndex (ty) with | Option.some n => ((who, n) :: types) | Option.none => types) | Option.none => types)

@[expose] public def paramTypes : (types : List ((Prod (Id) (Nat)))) -> (params : Par) -> List ((Prod (Id) (Nat)))
  | types, Par.ParNil => types
  | types, Par.ParCons pat ty rest => paramTypes (typesWith (types) (pat) (ty)) (rest)

mutual
@[expose] public def applyBlock : (types : List ((Prod (Id) (Nat)))) -> (decl : Decl) -> (b : Expr) -> (tail : Bool) -> Except (ByteArray) (List ((Prod (Id) (Nat))))
  | types, _decl, Expr.ELit _ _, _tail => Except.ok (types)
  | types, _decl, Expr.ERead _ _ _, _tail => Except.ok (types)
  | types, _decl, Expr.EWrap _ _ _, _tail => Except.ok (types)
  | types, _decl, Expr.ECall _ _ _ _, _tail => Except.ok (types)
  | types, _decl, Expr.EApply _ _ _ _, _tail => Except.ok (types)
  | types, _decl, Expr.EConstruct _ _ _, _tail => Except.ok (types)
  | types, _decl, Expr.EPair _ _ _, _tail => Except.ok (types)
  | types, _decl, Expr.EIf _ _ _ _, _tail => Except.ok (types)
  | types, _decl, Expr.EMatch _ _ _, _tail => Except.ok (types)
  | types, _decl, Expr.EBlockE _, _tail => Except.ok (types)
  | types, _decl, Expr.EProbe _ _ _, _tail => Except.ok (types)
  | types, decl, Expr.EBlock lets bodyTail, tail => (match applyLets (types) (decl) (lets) with | Except.ok t1 => applyExpr (t1) (decl) (bodyTail) (tail) | Except.error problem45 => Except.error (problem45))
  | types, _decl, Expr.ENil, _tail => Except.ok (types)
  | types, _decl, Expr.ECons _ _, _tail => Except.ok (types)
  | types, _decl, Expr.ELet _ _ _ _ _ _, _tail => Except.ok (types)
  | types, _decl, Expr.EArm _ _ _, _tail => Except.ok (types)
  | types, _decl, Expr.ELoop _ _, _tail => Except.ok (types)
termination_by structural _ _ b _ => b

@[expose] public def applyExpr : (types : List ((Prod (Id) (Nat)))) -> (decl : Decl) -> (e : Expr) -> (tail : Bool) -> Except (ByteArray) (List ((Prod (Id) (Nat))))
  | types, _decl, Expr.ELit _ _, _tail => Except.ok (types)
  | types, _decl, Expr.ERead _ _ _, _tail => Except.ok (types)
  | types, decl, Expr.EWrap _ inner _, _tail => applyExpr (types) (decl) (inner) (false)
  | types, decl, Expr.ECall _ args _ _, _tail => applyArgs (types) (decl) (args)
  | types, decl, Expr.EApply who args propagate _, tail => (match typesGet (types) (who) with | Option.none => Except.error (cat (tick (idText (who))) (text (" is applied without a function type"))) | Option.some fnType => (match lookupFlag ((decl).applies) (fnType) with | Option.none => Except.error (cat (text ("Fn")) (cat (decimal (fnType)) (text ("::apply is not declared")))) | Option.some fallible => (let differs : Bool := (if fallible then (if propagate then false else true) else propagate); (let excused : Bool := (if tail then fallible else false); (match differs with | Bool.true => (match excused with | Bool.true => applyArgs (types) (decl) (args) | Bool.false => Except.error (cat (text ("arithmetic: an application of Fn")) (cat (decimal (fnType)) (cat (text (" ")) (cat ((if propagate then text ("propagates") else text ("drops"))) (cat (text (" a failure its apply ")) ((if fallible then text ("can have") else text ("cannot have"))))))))) | Bool.false => applyArgs (types) (decl) (args))))))
  | types, decl, Expr.EConstruct _ args _, _tail => applyArgs (types) (decl) (args)
  | types, decl, Expr.EPair left right _, _tail => (match applyExpr (types) (decl) (left) (false) with | Except.ok t1 => applyExpr (t1) (decl) (right) (false) | Except.error problem41 => Except.error (problem41))
  | types, decl, Expr.EIf condition thenBranch elseBranch _, tail => (match applyExpr (types) (decl) (condition) (false) with | Except.ok t1 => (match applyBlock (t1) (decl) (thenBranch) (tail) with | Except.ok t2 => applyBlock (t2) (decl) (elseBranch) (tail) | Except.error problem42 => Except.error (problem42)) | Except.error problem43 => Except.error (problem43))
  | types, decl, Expr.EMatch scrutinee arms _, tail => (match applyExpr (types) (decl) (scrutinee) (false) with | Except.ok t1 => applyArms (t1) (decl) (arms) (tail) | Except.error problem44 => Except.error (problem44))
  | types, decl, Expr.EBlockE inner, tail => applyBlock (types) (decl) (inner) (tail)
  | types, _decl, Expr.EProbe _ _ _, _tail => Except.ok (types)
  | types, _decl, Expr.EBlock _ _, _tail => Except.ok (types)
  | types, _decl, Expr.ENil, _tail => Except.ok (types)
  | types, _decl, Expr.ECons _ _, _tail => Except.ok (types)
  | types, _decl, Expr.ELet _ _ _ _ _ _, _tail => Except.ok (types)
  | types, _decl, Expr.EArm _ _ _, _tail => Except.ok (types)
  | types, decl, Expr.ELoop _ body, _tail => applyBlock (types) (decl) (body) (false)
termination_by structural _ _ e _ => e

@[expose] public def applyLets : (types : List ((Prod (Id) (Nat)))) -> (decl : Decl) -> (lets : Expr) -> Except (ByteArray) (List ((Prod (Id) (Nat))))
  | types, _decl, Expr.ELit _ _ => Except.ok (types)
  | types, _decl, Expr.ERead _ _ _ => Except.ok (types)
  | types, _decl, Expr.EWrap _ _ _ => Except.ok (types)
  | types, _decl, Expr.ECall _ _ _ _ => Except.ok (types)
  | types, _decl, Expr.EApply _ _ _ _ => Except.ok (types)
  | types, _decl, Expr.EConstruct _ _ _ => Except.ok (types)
  | types, _decl, Expr.EPair _ _ _ => Except.ok (types)
  | types, _decl, Expr.EIf _ _ _ _ => Except.ok (types)
  | types, _decl, Expr.EMatch _ _ _ => Except.ok (types)
  | types, _decl, Expr.EBlockE _ => Except.ok (types)
  | types, _decl, Expr.EProbe _ _ _ => Except.ok (types)
  | types, _decl, Expr.EBlock _ _ => Except.ok (types)
  | types, _decl, Expr.ENil => Except.ok (types)
  | types, _decl, Expr.ECons _ _ => Except.ok (types)
  | types, decl, Expr.ELet pat hasTy ty value _ rest => (let typed : List ((Prod (Id) (Nat))) := (match hasTy with | Bool.true => typesWith (types) (pat) (ty) | Bool.false => types); (match applyExpr (typed) (decl) (value) (false) with | Except.ok t1 => applyLets (t1) (decl) (rest) | Except.error problem46 => Except.error (problem46)))
  | types, _decl, Expr.EArm _ _ _ => Except.ok (types)
  | types, _decl, Expr.ELoop _ _ => Except.ok (types)
termination_by structural _ _ lets => lets

@[expose] public def applyArgs : (types : List ((Prod (Id) (Nat)))) -> (decl : Decl) -> (args : Expr) -> Except (ByteArray) (List ((Prod (Id) (Nat))))
  | types, _decl, Expr.ELit _ _ => Except.ok (types)
  | types, _decl, Expr.ERead _ _ _ => Except.ok (types)
  | types, _decl, Expr.EWrap _ _ _ => Except.ok (types)
  | types, _decl, Expr.ECall _ _ _ _ => Except.ok (types)
  | types, _decl, Expr.EApply _ _ _ _ => Except.ok (types)
  | types, _decl, Expr.EConstruct _ _ _ => Except.ok (types)
  | types, _decl, Expr.EPair _ _ _ => Except.ok (types)
  | types, _decl, Expr.EIf _ _ _ _ => Except.ok (types)
  | types, _decl, Expr.EMatch _ _ _ => Except.ok (types)
  | types, _decl, Expr.EBlockE _ => Except.ok (types)
  | types, _decl, Expr.EProbe _ _ _ => Except.ok (types)
  | types, _decl, Expr.EBlock _ _ => Except.ok (types)
  | types, _decl, Expr.ENil => Except.ok (types)
  | types, decl, Expr.ECons head rest => (match applyExpr (types) (decl) (head) (false) with | Except.ok t1 => applyArgs (t1) (decl) (rest) | Except.error problem47 => Except.error (problem47))
  | types, _decl, Expr.ELet _ _ _ _ _ _ => Except.ok (types)
  | types, _decl, Expr.EArm _ _ _ => Except.ok (types)
  | types, _decl, Expr.ELoop _ _ => Except.ok (types)
termination_by structural _ _ args => args

@[expose] public def applyArms : (types : List ((Prod (Id) (Nat)))) -> (decl : Decl) -> (arms : Expr) -> (tail : Bool) -> Except (ByteArray) (List ((Prod (Id) (Nat))))
  | types, _decl, Expr.ELit _ _, _tail => Except.ok (types)
  | types, _decl, Expr.ERead _ _ _, _tail => Except.ok (types)
  | types, _decl, Expr.EWrap _ _ _, _tail => Except.ok (types)
  | types, _decl, Expr.ECall _ _ _ _, _tail => Except.ok (types)
  | types, _decl, Expr.EApply _ _ _ _, _tail => Except.ok (types)
  | types, _decl, Expr.EConstruct _ _ _, _tail => Except.ok (types)
  | types, _decl, Expr.EPair _ _ _, _tail => Except.ok (types)
  | types, _decl, Expr.EIf _ _ _ _, _tail => Except.ok (types)
  | types, _decl, Expr.EMatch _ _ _, _tail => Except.ok (types)
  | types, _decl, Expr.EBlockE _, _tail => Except.ok (types)
  | types, _decl, Expr.EProbe _ _ _, _tail => Except.ok (types)
  | types, _decl, Expr.EBlock _ _, _tail => Except.ok (types)
  | types, _decl, Expr.ENil, _tail => Except.ok (types)
  | types, _decl, Expr.ECons _ _, _tail => Except.ok (types)
  | types, _decl, Expr.ELet _ _ _ _ _ _, _tail => Except.ok (types)
  | types, decl, Expr.EArm _ body rest, tail => (match applyBlock (types) (decl) (body) (tail) with | Except.ok t1 => applyArms (t1) (decl) (rest) (tail) | Except.error problem48 => Except.error (problem48))
  | types, _decl, Expr.ELoop _ _, _tail => Except.ok (types)
termination_by structural _ _ arms _ => arms
end

@[expose] public def parsOk : (cx : Cx) -> (st : St) -> (params : Par) -> Except (ByteArray) (St)
  | _cx, st, Par.ParNil => Except.ok (st)
  | cx, st, Par.ParCons pat ty rest => (match tyOk (cx) (ty) with | Except.ok _ => (match bindPat (st) ((cx).decl) (pat) with | Except.ok s1 => parsOk (cx) (s1) (rest) | Except.error problem49 => Except.error (problem49)) | Except.error problem50 => Except.error (problem50))

@[expose] public def capsUnbox (caps : List (Cap)) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (cap : Cap) => (match cap with | Cap.CapCopy => found | Cap.CapClone => found | Cap.CapUnbox => true | Cap.CapUnit => found))) (false) (caps) : Bool)

@[expose] public def disOk : (cx : Cx) -> (fnType : Nat) -> (fallible : Bool) -> (arms : Dis) -> Except (ByteArray) (Unit)
  | _cx, _fnType, _fallible, Dis.DisNil => Except.ok (())
  | cx, fnType, fallible, Dis.DisCons function captures functionFallible rest => (match lookupFlag (((cx).decl).functions) (function) with | Option.none => Except.error (cat (text ("Fn")) (cat (decimal (fnType)) (cat (text (" dispatches to undeclared f")) (decimal (function))))) | Option.some isFunction => (let differs : Bool := (if isFunction then (if functionFallible then false else true) else functionFallible); (let overstated : Bool := (if functionFallible then (if fallible then false else true) else false); (match differs with | Bool.true => Except.error (cat (text ("arithmetic: Fn")) (cat (decimal (fnType)) (cat (text ("::apply misstates the failure of f")) (decimal (function))))) | Bool.false => (match overstated with | Bool.true => Except.error (cat (text ("arithmetic: Fn")) (cat (decimal (fnType)) (cat (text ("::apply misstates the failure of f")) (decimal (function))))) | Bool.false => (match capsUnbox (captures) with | Bool.true => (match heapUnit (cx) (text ("a boxed capture")) with | Except.ok _ => disOk (cx) (fnType) (fallible) (rest) | Except.error problem51 => Except.error (problem51)) | Bool.false => disOk (cx) (fnType) (fallible) (rest)))))))

@[expose] public def validateItem (core : Bool) (decl : Decl) (rows : List (ItemRow)) (item : Def) : Except (ByteArray) (Unit) := (match item with | Def.DEnum _ variants _ => rowTysOk (({ core := core, decl := decl, fallible := false, items := rows } : Cx)) (variants) | Def.DApply fnType parameters result fallible arms _ => (let cx : Cx := ({ core := core, decl := decl, fallible := false, items := rows } : Cx); (match tysOk (cx) (parameters) with | Except.ok _ => (match tyOk (cx) (result) with | Except.ok _ => disOk (cx) (fnType) (fallible) (arms) | Except.error problem52 => Except.error (problem52)) | Except.error problem53 => Except.error (problem53))) | Def.DFunction name parameters result body _ => (let fallible : Bool := tyIsFallible (result); (let cx : Cx := ({ core := core, decl := decl, fallible := fallible, items := rows } : Cx); (match tyOk (cx) (result) with | Except.ok _ => (match parsOk (cx) (({ bound := ([] : List (Id)), scope := ([] : List (Id)), moved := ([] : List (Id)), parted := ([] : List ((Prod (Id) (List (Nat))))), propagates := false } : St)) (parameters) with | Except.ok s1 => (match blockC (s1) (cx) (body) (true) with | Except.ok s2 => (match (if (s2).propagates then (!fallible) else false) with | Bool.true => Except.error (cat (text ("arithmetic: ")) (cat (tick (idText (name))) (text (" propagates a failure its result type does not carry")))) | Bool.false => (match applyBlock (paramTypes (([] : List ((Prod (Id) (Nat))))) (parameters)) (decl) (body) (fallible) with | Except.ok _ => Except.ok (()) | Except.error problem54 => Except.error (problem54))) | Except.error problem55 => Except.error (problem55)) | Except.error problem56 => Except.error (problem56)) | Except.error problem57 => Except.error (problem57)))) | Def.DNil => Except.ok (()) | Def.DCons _ _ => Except.ok (()))

@[expose] public def validateItems : (core : Bool) -> (decl : Decl) -> (rows : List (ItemRow)) -> (defs : Def) -> Except (ByteArray) (Unit)
  | _core, _decl, _rows, Def.DEnum _ _ _ => Except.ok (())
  | _core, _decl, _rows, Def.DApply _ _ _ _ _ _ => Except.ok (())
  | _core, _decl, _rows, Def.DFunction _ _ _ _ _ => Except.ok (())
  | _core, _decl, _rows, Def.DNil => Except.ok (())
  | core, decl, rows, Def.DCons head tail => (match validateItem (core) (decl) (rows) (head) with | Except.ok _ => validateItems (core) (decl) (rows) (tail) | Except.error problem58 => Except.error (problem58))

@[expose] public def validateCrate (krate : Crate) : Except (ByteArray) (Unit) := (let itemRows0 : List (ItemRow) := (({ family := 0, stem := text ("nat_add"), form := 0, fallible := true, heap := false } : ItemRow) :: (({ family := 1, stem := text ("nat_sub"), form := 0, fallible := false, heap := false } : ItemRow) :: (({ family := 2, stem := text ("nat_mul"), form := 0, fallible := true, heap := false } : ItemRow) :: (({ family := 3, stem := text ("nat_quot"), form := 0, fallible := false, heap := false } : ItemRow) :: (({ family := 4, stem := text ("nat_rem"), form := 0, fallible := false, heap := false } : ItemRow) :: (({ family := 5, stem := text ("nat_eq"), form := 0, fallible := false, heap := false } : ItemRow) :: (({ family := 6, stem := text ("nat_le"), form := 0, fallible := false, heap := false } : ItemRow) :: (({ family := 7, stem := text ("nat_lt"), form := 0, fallible := false, heap := false } : ItemRow) :: (({ family := 8, stem := text ("nat_succ"), form := 0, fallible := true, heap := false } : ItemRow) :: (({ family := 9, stem := text ("int_add"), form := 0, fallible := true, heap := false } : ItemRow) :: ([] : List (ItemRow)))))))))))); (let itemRows1 : List (ItemRow) := (({ family := 10, stem := text ("int_sub"), form := 0, fallible := true, heap := false } : ItemRow) :: (({ family := 11, stem := text ("int_mul"), form := 0, fallible := true, heap := false } : ItemRow) :: (({ family := 12, stem := text ("int_neg"), form := 0, fallible := true, heap := false } : ItemRow) :: (({ family := 13, stem := text ("int_quot"), form := 0, fallible := true, heap := false } : ItemRow) :: (({ family := 14, stem := text ("int_rem"), form := 0, fallible := false, heap := false } : ItemRow) :: (({ family := 15, stem := text ("bool_not"), form := 0, fallible := false, heap := false } : ItemRow) :: (({ family := 16, stem := text ("bool_and"), form := 0, fallible := false, heap := false } : ItemRow) :: (({ family := 17, stem := text ("bool_or"), form := 0, fallible := false, heap := false } : ItemRow) :: (({ family := 18, stem := text ("equal"), form := 0, fallible := false, heap := false } : ItemRow) :: (({ family := 19, stem := text ("compare"), form := 0, fallible := false, heap := false } : ItemRow) :: itemRows0)))))))))); (let itemRows2 : List (ItemRow) := (({ family := 20, stem := text ("checked_add"), form := 1, fallible := false, heap := false } : ItemRow) :: (({ family := 21, stem := text ("checked_sub"), form := 1, fallible := false, heap := false } : ItemRow) :: (({ family := 22, stem := text ("checked_mul"), form := 1, fallible := false, heap := false } : ItemRow) :: (({ family := 23, stem := text ("checked_quot"), form := 1, fallible := false, heap := false } : ItemRow) :: (({ family := 24, stem := text ("checked_neg_"), form := 2, fallible := false, heap := false } : ItemRow) :: (({ family := 25, stem := text ("bit_and"), form := 1, fallible := false, heap := false } : ItemRow) :: (({ family := 26, stem := text ("bit_or"), form := 1, fallible := false, heap := false } : ItemRow) :: (({ family := 27, stem := text ("bit_xor"), form := 1, fallible := false, heap := false } : ItemRow) :: (({ family := 28, stem := text ("bit_not"), form := 1, fallible := false, heap := false } : ItemRow) :: (({ family := 29, stem := text ("shift_left"), form := 1, fallible := false, heap := false } : ItemRow) :: itemRows1)))))))))); (let itemRows3 : List (ItemRow) := (({ family := 30, stem := text ("shift_right"), form := 1, fallible := false, heap := false } : ItemRow) :: (({ family := 31, stem := text ("convert"), form := 1, fallible := false, heap := false } : ItemRow) :: (({ family := 32, stem := text ("append_list"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 33, stem := text ("append_bytes"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 34, stem := text ("length_list"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 35, stem := text ("length_bytes"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 36, stem := text ("length_string"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 37, stem := text ("index_list"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 38, stem := text ("index_bytes"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 39, stem := text ("slice_list"), form := 0, fallible := false, heap := true } : ItemRow) :: itemRows2)))))))))); (let itemRows4 : List (ItemRow) := (({ family := 40, stem := text ("slice_bytes"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 41, stem := text ("utf8_encode"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 42, stem := text ("utf8_decode"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 43, stem := text ("compare_bytes"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 44, stem := text ("split_exact"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 45, stem := text ("join"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 46, stem := text ("format_i64"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 47, stem := text ("format_"), form := 2, fallible := false, heap := true } : ItemRow) :: (({ family := 48, stem := text ("parse_int"), form := 0, fallible := true, heap := true } : ItemRow) :: (({ family := 49, stem := text ("parse_"), form := 2, fallible := false, heap := true } : ItemRow) :: itemRows3)))))))))); (match declareDefs (({ adts := ([] : List ((Prod (Nat) (List ((Prod (Nat) (Nat))))))), closures := ([] : List ((Prod (Nat) (List ((Prod (Nat) (Nat))))))), applies := ([] : List ((Prod (Nat) (Bool)))), functions := ([] : List ((Prod (Nat) (Bool)))) } : Decl)) ((krate).items) with | Except.ok decl => validateItems ((krate).core) (decl) (itemRows4) ((krate).items) | Except.error problem59 => Except.error (problem59)))))))

@[expose] public def foundRowTys : (found : Found) -> (row : Row) -> Found
  | found, Row.RowNil => found
  | found, Row.RowCons _ fields rest => foundRowTys (foundTy (found) (fields)) (rest)

@[expose] public def foundParTys : (found : Found) -> (params : Par) -> Found
  | found, Par.ParNil => found
  | found, Par.ParCons _ ty rest => foundParTys (foundTy (found) (ty)) (rest)

@[expose] public def tyIsFn (ty : Ty) : Bool := (match ty with | Ty.TyLeaf _ => false | Ty.TyFixed _ => false | Ty.TyUnary _ _ => false | Ty.TyBinary _ _ _ => false | Ty.TyNamed k _ => nameIsFn (k) | Ty.TyNil => false | Ty.TyCons _ _ => false)

@[expose] public def foundDefs : (found : Found) -> (defs : Def) -> Found
  | found, Def.DEnum name variants org => foundRowTys (foundAt (found) ((match tyIsFn (name) with | Bool.true => text ("enum:closures") | Bool.false => text ("enum:adt"))) (org)) (variants)
  | found, Def.DApply _ parameters result _ _ org => foundTy (foundTy (foundAt (found) (text ("apply:dispatch")) (org)) (parameters)) (result)
  | found, Def.DFunction _ parameters result body org => foundExpr (foundTy (foundParTys (foundAt (found) (text ("function")) (org)) (parameters)) (result)) (body)
  | found, Def.DNil => found
  | found, Def.DCons head tail => (let f1 : Found := foundDefs (found) (head); foundDefs (f1) (tail))

@[expose] public def reverseInsts (insts : List (Inst)) : List (Inst) := (LexLeanCollections.listFold ((fun (turned : List (Inst)) (inst : Inst) => (inst :: turned))) (([] : List (Inst))) (insts) : List (Inst))

@[expose] public def rowOf (table : List ((Prod (ByteArray) (List (ByteArray))))) (construct : ByteArray) : Option (List (ByteArray)) := (LexLeanCollections.listFold ((fun (found : Option (List (ByteArray))) (row : (Prod (ByteArray) (List (ByteArray)))) => (match found with | Option.some value => Option.some (value) | Option.none => (if (LexLeanRuntime.equal ((row).1) (construct) : Bool) then Option.some ((row).2) else Option.none)))) (Option.none) (table) : Option (List (ByteArray)))

@[expose] public def anyIn (pieces : List (ByteArray)) (elements : List (ByteArray)) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (piece : ByteArray) => (let here : Bool := bytesHas (elements) (piece); (found || here)))) (false) (pieces) : Bool)

@[expose] public def debugRow (pieces : List (ByteArray)) : ByteArray := cat (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 91)), (_root_.UInt8.ofNat (nat_lit 34))]) (cat (LexLeanStd.Text.joinWith (pieces) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 34)), (_root_.UInt8.ofNat (nat_lit 44)), (_root_.UInt8.ofNat (nat_lit 32)), (_root_.UInt8.ofNat (nat_lit 34))])) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 34)), (_root_.UInt8.ofNat (nat_lit 93))]))

@[expose] public def checkInst (table : List ((Prod (ByteArray) (List (ByteArray))))) (elements : List (ByteArray)) (inst : Inst) : Option (ByteArray) := (let construct : ByteArray := (inst).construct; (let element : ByteArray := (inst).element; (match rowOf (table) (construct) with | Option.none => Option.some (cat (text ("the construct ")) (cat (tick (construct)) (text (" has no target-semantics correspondence")))) | Option.some pieces => (match bytesHas (pieces) (element) with | Bool.true => (match bytesHas (elements) (element) with | Bool.true => (let differs : Bool := (!optKindEq ((inst).width) ((inst).originWidth)); (let needs : Bool := (match (inst).width with | Option.some _ => true | Option.none => LexLeanStd.Text.startsWith (construct) (text ("call:runtime:"))); (match (if differs then needs else false) with | Bool.true => Option.some (cat (text ("the construct ")) (cat (tick (construct)) (cat (text (" works at width ")) (cat (optKindDebug ((inst).width)) (cat (text (", but ")) (cat (tick (element)) (cat (text (" is at ")) (optKindDebug ((inst).originWidth))))))))) | Bool.false => Option.none))) | Bool.false => Option.some (cat (text ("the construct ")) (cat (tick (construct)) (cat (text (" realizes ")) (cat (tick (element)) (text (", which the program does not use"))))))) | Bool.false => Option.some (cat (text ("the construct ")) (cat (tick (construct)) (cat (text (" does not realize ")) (cat (tick (element)) (text (", the element it was lowered from"))))))))))

@[expose] public def checkType (table : List ((Prod (ByteArray) (List (ByteArray))))) (elements : List (ByteArray)) (construct : ByteArray) : Option (ByteArray) := (match rowOf (table) (construct) with | Option.none => Option.some (cat (text ("the construct ")) (cat (tick (construct)) (text (" has no target-semantics correspondence")))) | Option.some pieces => (match anyIn (pieces) (elements) with | Bool.true => Option.none | Bool.false => Option.some (cat (text ("the construct ")) (cat (tick (construct)) (cat (text (" realizes none of ")) (cat (debugRow (pieces)) (text (", which the program does not use"))))))))

@[expose] public def correspondTable (_unit : Unit) : List ((Prod (ByteArray) (List (ByteArray)))) := (let rows0 : List ((Prod (ByteArray) (List (ByteArray)))) := ((text ("function"), (text ("function") :: (text ("export") :: ([] : List (ByteArray))))) :: ((text ("enum:adt"), (text ("type:adt") :: ([] : List (ByteArray)))) :: ((text ("enum:closures"), (text ("type:fn") :: (text ("expr:closure") :: ([] : List (ByteArray))))) :: ((text ("apply:dispatch"), (text ("type:fn") :: (text ("expr:closure") :: ([] : List (ByteArray))))) :: ((text ("let"), (text ("expr:let") :: (text ("expr:match") :: (text ("expr:prim") :: (text ("expr:apply") :: (text ("expr:call") :: (text ("expr:closure") :: (text ("expr:cond") :: (text ("expr:field") :: (text ("shape:some") :: (text ("shape:ok") :: (text ("shape:error") :: (text ("shape:cons") :: (text ("shape:adt") :: (text ("shape:succ") :: (text ("shape:zero") :: (text ("shape:unit") :: (text ("shape:true") :: (text ("indirection") :: (text ("overflow") :: ([] : List (ByteArray)))))))))))))))))))))) :: ((text ("read"), (text ("expr:var") :: (text ("expr:match") :: (text ("expr:prim") :: (text ("expr:call") :: (text ("expr:apply") :: (text ("expr:closure") :: (text ("expr:first") :: (text ("expr:second") :: (text ("expr:field") :: (text ("expr:cond") :: (text ("shape:true") :: (text ("shape:zero") :: (text ("shape:some") :: (text ("shape:ok") :: (text ("shape:error") :: (text ("shape:cons") :: (text ("shape:adt") :: (text ("export") :: (text ("type:fn") :: (text ("type:adt") :: (text ("type:pair") :: (text ("type:result") :: ([] : List (ByteArray))))))))))))))))))))))))) :: ([] : List ((Prod (ByteArray) (List (ByteArray))))))))))); (let rows1 : List ((Prod (ByteArray) (List (ByteArray)))) := ((text ("box"), (text ("indirection") :: ([] : List (ByteArray)))) :: ((text ("destructure:pair"), (text ("expr:first") :: (text ("expr:second") :: (text ("shape:pair") :: (text ("expr:call") :: ([] : List (ByteArray))))))) :: ((text ("type:unit"), (text ("type:unit") :: ([] : List (ByteArray)))) :: ((text ("type:bool"), (text ("type:bool") :: ([] : List (ByteArray)))) :: ((text ("type:nat"), (text ("type:nat") :: (text ("type:fixed") :: ([] : List (ByteArray))))) :: ((text ("type:int"), (text ("type:int") :: (text ("type:fixed") :: ([] : List (ByteArray))))) :: rows0)))))); (let rows2 : List ((Prod (ByteArray) (List (ByteArray)))) := ((text ("type:fixed"), (text ("type:fixed") :: ([] : List (ByteArray)))) :: ((text ("type:ordering"), (text ("type:ordering") :: ([] : List (ByteArray)))) :: ((text ("type:string"), (text ("type:string") :: ([] : List (ByteArray)))) :: ((text ("type:bytes"), (text ("type:bytes") :: ([] : List (ByteArray)))) :: ((text ("type:option"), (text ("type:option") :: ([] : List (ByteArray)))) :: ((text ("type:result"), (text ("type:result") :: (text ("expr:call") :: ([] : List (ByteArray))))) :: rows1)))))); (let rows3 : List ((Prod (ByteArray) (List (ByteArray)))) := ((text ("type:list"), (text ("type:list") :: ([] : List (ByteArray)))) :: ((text ("type:pair"), (text ("type:pair") :: (text ("expr:call") :: ([] : List (ByteArray))))) :: ((text ("type:adt"), (text ("type:adt") :: ([] : List (ByteArray)))) :: ((text ("type:fn"), (text ("type:fn") :: (text ("expr:closure") :: ([] : List (ByteArray))))) :: ((text ("type:rc"), (text ("indirection") :: ([] : List (ByteArray)))) :: ((text ("type:fallible"), (text ("overflow") :: ([] : List (ByteArray)))) :: rows2)))))); (let rows4 : List ((Prod (ByteArray) (List (ByteArray)))) := ((text ("type:ref"), (text ("export") :: ([] : List (ByteArray)))) :: ((text ("lit:unit"), (text ("value:unit") :: (text ("shape:unit") :: (text ("expr:var") :: (text ("expr:call") :: (text ("expr:apply") :: (text ("expr:closure") :: (text ("shape:some") :: (text ("shape:ok") :: (text ("shape:error") :: (text ("shape:cons") :: (text ("shape:adt") :: (text ("export") :: (text ("overflow") :: ([] : List (ByteArray)))))))))))))))) :: ((text ("lit:bool"), (text ("value:bool") :: (text ("shape:true") :: (text ("shape:false") :: (text ("expr:cond") :: ([] : List (ByteArray))))))) :: ((text ("lit:nat"), (text ("value:nat") :: (text ("shape:zero") :: ([] : List (ByteArray))))) :: ((text ("lit:int"), (text ("value:int") :: ([] : List (ByteArray)))) :: ((text ("lit:fixed"), (text ("value:u8") :: (text ("value:u16") :: (text ("value:u32") :: (text ("value:u64") :: (text ("value:i8") :: (text ("value:i16") :: (text ("value:i32") :: (text ("value:i64") :: ([] : List (ByteArray))))))))))) :: rows3)))))); (let rows5 : List ((Prod (ByteArray) (List (ByteArray)))) := ((text ("lit:string"), (text ("value:string") :: ([] : List (ByteArray)))) :: ((text ("lit:bytes"), (text ("value:bytes") :: ([] : List (ByteArray)))) :: ((text ("lit:ordering"), (text ("value:ordering") :: (text ("shape:lt") :: (text ("shape:eq") :: (text ("shape:gt") :: ([] : List (ByteArray))))))) :: ((text ("construct:none"), (text ("value:none") :: (text ("shape:none") :: ([] : List (ByteArray))))) :: ((text ("construct:some"), (text ("value:some") :: (text ("shape:some") :: ([] : List (ByteArray))))) :: ((text ("construct:ok"), (text ("value:ok") :: (text ("shape:ok") :: (text ("expr:call") :: ([] : List (ByteArray)))))) :: rows4)))))); (let rows6 : List ((Prod (ByteArray) (List (ByteArray)))) := ((text ("construct:err"), (text ("value:error") :: (text ("shape:error") :: (text ("expr:call") :: ([] : List (ByteArray)))))) :: ((text ("construct:adt"), (text ("value:adt") :: (text ("shape:adt") :: ([] : List (ByteArray))))) :: ((text ("construct:closure"), (text ("expr:closure") :: ([] : List (ByteArray)))) :: ((text ("construct:cons"), (text ("value:list") :: (text ("shape:cons") :: ([] : List (ByteArray))))) :: ((text ("construct:nil"), (text ("value:list") :: (text ("shape:nil") :: ([] : List (ByteArray))))) :: ((text ("construct:pair"), (text ("value:pair") :: (text ("shape:pair") :: (text ("expr:call") :: ([] : List (ByteArray)))))) :: rows5)))))); (let rows7 : List ((Prod (ByteArray) (List (ByteArray)))) := ((text ("if"), (text ("expr:cond") :: (text ("shape:true") :: (text ("shape:zero") :: ([] : List (ByteArray)))))) :: ((text ("match:nat"), (text ("shape:zero") :: ([] : List (ByteArray)))) :: ((text ("match:list"), (text ("shape:nil") :: ([] : List (ByteArray)))) :: ((text ("match:option"), (text ("shape:none") :: ([] : List (ByteArray)))) :: ((text ("match:result"), (text ("shape:ok") :: ([] : List (ByteArray)))) :: ((text ("match:ordering"), (text ("shape:lt") :: ([] : List (ByteArray)))) :: rows6)))))); (let rows8 : List ((Prod (ByteArray) (List (ByteArray)))) := ((text ("match:adt"), (text ("shape:adt") :: (text ("expr:field") :: ([] : List (ByteArray))))) :: ((text ("match:empty"), (text ("type:fn") :: (text ("type:adt") :: (text ("type:pair") :: (text ("type:result") :: ([] : List (ByteArray))))))) :: ((text ("call:function"), (text ("expr:call") :: (text ("export") :: ([] : List (ByteArray))))) :: ((text ("apply"), (text ("expr:apply") :: ([] : List (ByteArray)))) :: ((text ("widen"), (text ("prim:convert") :: ([] : List (ByteArray)))) :: ((text ("succeed"), (text ("overflow") :: ([] : List (ByteArray)))) :: rows7)))))); (let rows9 : List ((Prod (ByteArray) (List (ByteArray)))) := ((text ("call:runtime:succ"), (text ("shape:succ") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:nat_add"), (text ("prim:nat_add") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:nat_sub"), (text ("prim:nat_sub") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:nat_mul"), (text ("prim:nat_mul") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:nat_quot"), (text ("prim:nat_quot") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:nat_rem"), (text ("prim:nat_rem") :: ([] : List (ByteArray)))) :: rows8)))))); (let rows10 : List ((Prod (ByteArray) (List (ByteArray)))) := ((text ("call:runtime:nat_eq"), (text ("prim:nat_eq") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:nat_le"), (text ("prim:nat_le") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:nat_lt"), (text ("prim:nat_lt") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:int_add"), (text ("prim:int_add") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:int_sub"), (text ("prim:int_sub") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:int_mul"), (text ("prim:int_mul") :: ([] : List (ByteArray)))) :: rows9)))))); (let rows11 : List ((Prod (ByteArray) (List (ByteArray)))) := ((text ("call:runtime:int_neg"), (text ("prim:int_neg") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:int_quot"), (text ("prim:int_quot") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:int_rem"), (text ("prim:int_rem") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:bool_not"), (text ("prim:bool_not") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:bool_and"), (text ("prim:bool_and") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:bool_or"), (text ("prim:bool_or") :: ([] : List (ByteArray)))) :: rows10)))))); (let rows12 : List ((Prod (ByteArray) (List (ByteArray)))) := ((text ("call:runtime:equal"), (text ("prim:equal") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:compare"), (text ("prim:compare") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:checked_add"), (text ("prim:checked_add") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:checked_sub"), (text ("prim:checked_sub") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:checked_mul"), (text ("prim:checked_mul") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:checked_quot"), (text ("prim:checked_quot") :: ([] : List (ByteArray)))) :: rows11)))))); (let rows13 : List ((Prod (ByteArray) (List (ByteArray)))) := ((text ("call:runtime:checked_neg"), (text ("prim:checked_neg") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:bit_and"), (text ("prim:bit_and") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:bit_or"), (text ("prim:bit_or") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:bit_xor"), (text ("prim:bit_xor") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:bit_not"), (text ("prim:bit_not") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:shift_left"), (text ("prim:shift_left") :: ([] : List (ByteArray)))) :: rows12)))))); (let rows14 : List ((Prod (ByteArray) (List (ByteArray)))) := ((text ("call:runtime:shift_right"), (text ("prim:shift_right") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:convert"), (text ("prim:convert") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:append"), (text ("prim:append") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:length"), (text ("prim:length") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:index"), (text ("prim:index") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:slice"), (text ("prim:slice") :: ([] : List (ByteArray)))) :: rows13)))))); (let rows15 : List ((Prod (ByteArray) (List (ByteArray)))) := ((text ("call:runtime:utf8_encode"), (text ("prim:utf8_encode") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:utf8_decode"), (text ("prim:utf8_decode") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:compare_bytes"), (text ("prim:compare_bytes") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:split_exact"), (text ("prim:split_exact") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:join"), (text ("prim:join") :: ([] : List (ByteArray)))) :: ((text ("call:runtime:format_decimal"), (text ("prim:format_decimal") :: ([] : List (ByteArray)))) :: rows14)))))); (let rows16 : List ((Prod (ByteArray) (List (ByteArray)))) := ((text ("call:runtime:parse_decimal"), (text ("prim:parse_decimal") :: ([] : List (ByteArray)))) :: rows15); rows16)))))))))))))))))

@[expose] public def correspondCrate (krate : Crate) (elements : List (ByteArray)) : Except (ByteArray) (Unit) := (let itemRows0 : List (ItemRow) := (({ family := 0, stem := text ("nat_add"), form := 0, fallible := true, heap := false } : ItemRow) :: (({ family := 1, stem := text ("nat_sub"), form := 0, fallible := false, heap := false } : ItemRow) :: (({ family := 2, stem := text ("nat_mul"), form := 0, fallible := true, heap := false } : ItemRow) :: (({ family := 3, stem := text ("nat_quot"), form := 0, fallible := false, heap := false } : ItemRow) :: (({ family := 4, stem := text ("nat_rem"), form := 0, fallible := false, heap := false } : ItemRow) :: (({ family := 5, stem := text ("nat_eq"), form := 0, fallible := false, heap := false } : ItemRow) :: (({ family := 6, stem := text ("nat_le"), form := 0, fallible := false, heap := false } : ItemRow) :: (({ family := 7, stem := text ("nat_lt"), form := 0, fallible := false, heap := false } : ItemRow) :: (({ family := 8, stem := text ("nat_succ"), form := 0, fallible := true, heap := false } : ItemRow) :: (({ family := 9, stem := text ("int_add"), form := 0, fallible := true, heap := false } : ItemRow) :: ([] : List (ItemRow)))))))))))); (let itemRows1 : List (ItemRow) := (({ family := 10, stem := text ("int_sub"), form := 0, fallible := true, heap := false } : ItemRow) :: (({ family := 11, stem := text ("int_mul"), form := 0, fallible := true, heap := false } : ItemRow) :: (({ family := 12, stem := text ("int_neg"), form := 0, fallible := true, heap := false } : ItemRow) :: (({ family := 13, stem := text ("int_quot"), form := 0, fallible := true, heap := false } : ItemRow) :: (({ family := 14, stem := text ("int_rem"), form := 0, fallible := false, heap := false } : ItemRow) :: (({ family := 15, stem := text ("bool_not"), form := 0, fallible := false, heap := false } : ItemRow) :: (({ family := 16, stem := text ("bool_and"), form := 0, fallible := false, heap := false } : ItemRow) :: (({ family := 17, stem := text ("bool_or"), form := 0, fallible := false, heap := false } : ItemRow) :: (({ family := 18, stem := text ("equal"), form := 0, fallible := false, heap := false } : ItemRow) :: (({ family := 19, stem := text ("compare"), form := 0, fallible := false, heap := false } : ItemRow) :: itemRows0)))))))))); (let itemRows2 : List (ItemRow) := (({ family := 20, stem := text ("checked_add"), form := 1, fallible := false, heap := false } : ItemRow) :: (({ family := 21, stem := text ("checked_sub"), form := 1, fallible := false, heap := false } : ItemRow) :: (({ family := 22, stem := text ("checked_mul"), form := 1, fallible := false, heap := false } : ItemRow) :: (({ family := 23, stem := text ("checked_quot"), form := 1, fallible := false, heap := false } : ItemRow) :: (({ family := 24, stem := text ("checked_neg_"), form := 2, fallible := false, heap := false } : ItemRow) :: (({ family := 25, stem := text ("bit_and"), form := 1, fallible := false, heap := false } : ItemRow) :: (({ family := 26, stem := text ("bit_or"), form := 1, fallible := false, heap := false } : ItemRow) :: (({ family := 27, stem := text ("bit_xor"), form := 1, fallible := false, heap := false } : ItemRow) :: (({ family := 28, stem := text ("bit_not"), form := 1, fallible := false, heap := false } : ItemRow) :: (({ family := 29, stem := text ("shift_left"), form := 1, fallible := false, heap := false } : ItemRow) :: itemRows1)))))))))); (let itemRows3 : List (ItemRow) := (({ family := 30, stem := text ("shift_right"), form := 1, fallible := false, heap := false } : ItemRow) :: (({ family := 31, stem := text ("convert"), form := 1, fallible := false, heap := false } : ItemRow) :: (({ family := 32, stem := text ("append_list"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 33, stem := text ("append_bytes"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 34, stem := text ("length_list"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 35, stem := text ("length_bytes"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 36, stem := text ("length_string"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 37, stem := text ("index_list"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 38, stem := text ("index_bytes"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 39, stem := text ("slice_list"), form := 0, fallible := false, heap := true } : ItemRow) :: itemRows2)))))))))); (let itemRows4 : List (ItemRow) := (({ family := 40, stem := text ("slice_bytes"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 41, stem := text ("utf8_encode"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 42, stem := text ("utf8_decode"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 43, stem := text ("compare_bytes"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 44, stem := text ("split_exact"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 45, stem := text ("join"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 46, stem := text ("format_i64"), form := 0, fallible := false, heap := true } : ItemRow) :: (({ family := 47, stem := text ("format_"), form := 2, fallible := false, heap := true } : ItemRow) :: (({ family := 48, stem := text ("parse_int"), form := 0, fallible := true, heap := true } : ItemRow) :: (({ family := 49, stem := text ("parse_"), form := 2, fallible := false, heap := true } : ItemRow) :: itemRows3)))))))))); (let table : List ((Prod (ByteArray) (List (ByteArray)))) := correspondTable (()); (let found : Found := foundDefs (({ insts := ([] : List (Inst)), types := ([] : List (ByteArray)), items := itemRows4 } : Found)) ((krate).items); (match (LexLeanCollections.listFold ((fun (failure : Option (ByteArray)) (inst : Inst) => (match failure with | Option.some message => Option.some (message) | Option.none => checkInst (table) (elements) (inst)))) (Option.none) (reverseInsts ((found).insts)) : Option (ByteArray)) with | Option.some message => Except.error (message) | Option.none => (match (LexLeanCollections.listFold ((fun (failure : Option (ByteArray)) (construct : ByteArray) => (match failure with | Option.some message => Option.some (message) | Option.none => checkType (table) (elements) (construct)))) (Option.none) ((found).types) : Option (ByteArray)) with | Option.some message => Except.error (message) | Option.none => Except.ok (()))))))))))

end LexLeanBoot.RustValidate
