module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanBoot.AxiomOutput
public import LexLeanBoot.CoreJson
public import LexLeanStd.Json
public import LexLeanStd.Numeral
public import LexLeanStd.Order
public import LexLeanStd.Text
public import LexLeanBoot.SurfaceDoc
public import LexLeanBoot.SurfaceGeneric
public import LexLeanBoot.SurfaceLex
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.CorePrint

public inductive Tdoc where
  | TNil
  | TText (_ : ByteArray)
  | TSeq (_ : Tdoc) (_ : Tdoc)
  | TNest (_ : Nat) (_ : Tdoc)
  | TLine
  | TGroup (_ : Tdoc)
  | TVar (_ : Nat)
  | TConst (_ : ByteArray) (_ : ByteArray)
  | TBind (_ : ByteArray) (_ : Tdoc)

public inductive Term where
  | Atom (_ : Tdoc)
  | App (_ : Tdoc) (_ : List (Tdoc))
  | Prefix (_ : Tdoc)
  | Low (_ : Tdoc)

public structure Head where
  head : Tdoc
  args : List (Tdoc)

public structure Shown where
  base : ByteArray
  zero : Bool
  count : Nat

public structure Slot where
  index : Nat
  start : Nat
  height : Nat
  uses : Nat
  term : Term

public structure Take where
  stack : List (Slot)
  terms : List (Term)
  tallest : Nat

public structure Pass where
  index : Nat
  stack : List (Slot)
  refs : List (Nat)
  proofs : List (Nat)
  error : Option (ByteArray)

public structure Zeros where
  cursor : Nat
  found : Bool

@[expose] public def lit (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def cat (left : ByteArray) (right : ByteArray) : ByteArray := (LexLeanRuntime.append (left) (right) : ByteArray)

@[expose] public def both (left : Bool) (right : Bool) : Bool := (if left then right else false)

@[expose] public def same (left : ByteArray) (right : ByteArray) : Bool := (LexLeanRuntime.equal (left) (right) : Bool)

@[expose] public def num (number : Nat) : ByteArray := LexLeanStd.Numeral.decimal (number)

@[expose] public def listed (needle : ByteArray) (items : List (ByteArray)) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (item : ByteArray) => (if found then true else (LexLeanRuntime.equal (item) (needle) : Bool)))) (false) (items) : Bool)

@[expose] public def joined (pieces : List (ByteArray)) (separator : ByteArray) : ByteArray := ((LexLeanCollections.listFold ((fun (state : (Prod (Bool) (ByteArray))) (piece : ByteArray) => (false, (if (state).1 then piece else cat (cat ((state).2) (separator)) (piece))))) ((true, lit (""))) (pieces) : (Prod (Bool) (ByteArray)))).2

@[expose] public def codeAt (text : ByteArray) (position : Nat) : Nat := (match LexLeanStd.Text.byteAt (text) (position) with | Option.some byte => (let __widen0 : UInt8 := byte; ((((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((128 : UInt8)) : UInt8)) ((128 : UInt8)) : Bool) then 128 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((64 : UInt8)) : UInt8)) ((64 : UInt8)) : Bool) then 64 else 0)) + ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((32 : UInt8)) : UInt8)) ((32 : UInt8)) : Bool) then 32 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((16 : UInt8)) : UInt8)) ((16 : UInt8)) : Bool) then 16 else 0))) + (((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((8 : UInt8)) : UInt8)) ((8 : UInt8)) : Bool) then 8 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((4 : UInt8)) : UInt8)) ((4 : UInt8)) : Bool) then 4 else 0)) + ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((2 : UInt8)) : UInt8)) ((2 : UInt8)) : Bool) then 2 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((1 : UInt8)) : UInt8)) ((1 : UInt8)) : Bool) then 1 else 0))))) | Option.none => 0)

@[expose] public def isDigit (code : Nat) : Bool := both ((Nat.ble (48) (code))) ((Nat.ble (code) (57)))

@[expose] public def isAlpha (code : Nat) : Bool := (if both ((Nat.ble (65) (code))) ((Nat.ble (code) (90))) then true else both ((Nat.ble (97) (code))) ((Nat.ble (code) (122))))

@[expose] public def nameByte (text : ByteArray) (position : Nat) : Bool := (let code : Nat := codeAt (text) (position); (if (if isAlpha (code) then true else isDigit (code)) then true else (if listed (LexLeanStd.Text.sliceOr (text) (position) (1)) ((lit ("_") :: (lit ("'") :: (lit ("!") :: (lit ("?") :: (lit (".") :: (lit ("@") :: (lit ("#") :: ([] : List (ByteArray)))))))))) then true else (if (Nat.blt (code) (128)) then false else (if (Nat.blt (code) (192)) then true else (Nat.beq (LexLeanBoot.AxiomOutput.whiteLength (text) (position)) (0)))))))

@[expose] public def nameStart (text : ByteArray) : Bool := (let code : Nat := codeAt (text) (0); (if isAlpha (code) then true else (if (Nat.beq (code) (95)) then true else both ((Nat.ble (192) (code))) ((Nat.beq (LexLeanBoot.AxiomOutput.whiteLength (text) (0)) (0))))))

@[expose] public def allName (text : ByteArray) : Bool := (let stop : Nat := ((LexLeanCollections.iterateUntil ((fun (position : Nat) => (if both ((Nat.blt (position) ((LexLeanRuntime.length (text) : Nat)))) (nameByte (text) (position)) then Option.some ((position + 1)) else Option.none))) (((LexLeanRuntime.length (text) : Nat) + 1)) (0) : (Prod (Nat) (Bool)))).1; (Nat.beq (stop) ((LexLeanRuntime.length (text) : Nat))))

@[expose] public def digitsEnd (text : ByteArray) («from» : Nat) : Nat := ((LexLeanCollections.iterateUntil ((fun (position : Nat) => (if both ((Nat.blt (position) ((LexLeanRuntime.length (text) : Nat)))) (isDigit (codeAt (text) (position))) then Option.some ((position + 1)) else Option.none))) (((LexLeanRuntime.length (text) : Nat) + 1)) («from») : (Prod (Nat) (Bool)))).1

@[expose] public def zeroStep (text : ByteArray) (state : Zeros) : Option (Zeros) := (let position : Nat := (state).cursor; (if (if (state).found then true else (Nat.ble ((LexLeanRuntime.length (text) : Nat)) (position))) then Option.none else (if isDigit (codeAt (text) (position)) then (let stop : Nat := digitsEnd (text) (position); (let before : Nat := codeAt (text) ((LexLeanRuntime.subtract (position) (1) : Nat)); (let continues : Bool := both ((Nat.blt (0) (position))) ((if isAlpha (before) then true else (if (Nat.beq (before) (95)) then true else (Nat.beq (before) (39))))); Option.some (({ cursor := stop, found := both ((Nat.blt (1) ((LexLeanRuntime.subtract (stop) (position) : Nat)))) (both ((Nat.beq (codeAt (text) (position)) (48))) ((if continues then false else true))) } : Zeros))))) else Option.some (({ cursor := (position + 1), found := false } : Zeros)))))

@[expose] public def redundantZero (text : ByteArray) : Bool := (((LexLeanCollections.iterateUntil ((fun (state : Zeros) => zeroStep (text) (state))) (((LexLeanRuntime.length (text) : Nat) + 1)) (({ cursor := 0, found := false } : Zeros)) : (Prod (Zeros) (Bool)))).1).found

@[expose] public def keywords : List (ByteArray) := (lit ("fun") :: (lit ("let") :: (lit ("have") :: (lit ("proof") :: (lit ("Sort") :: (lit ("proj") :: (lit ("annot") :: (lit ("const") :: (lit ("max") :: (lit ("imax") :: (lit ("true") :: (lit ("false") :: (lit ("bool") :: (lit ("nat") :: (lit ("int") :: (lit ("str") :: (lit ("name") :: ([] : List (ByteArray)))))))))))))))))))

@[expose] public def bare (text : ByteArray) : Bool := both ((Nat.blt (0) ((LexLeanRuntime.length (text) : Nat)))) (both (nameStart (text)) (both (allName (text)) (both ((if LexLeanStd.Text.endsWith (text) (lit (".")) then false else true)) (both ((if same (text) (lit ("_")) then false else true)) (both ((if listed (text) (keywords) then false else true)) ((if redundantZero (text) then false else true)))))))

@[expose] public def spell (text : ByteArray) : ByteArray := (if bare (text) then text else LexLeanBoot.SurfaceLex.quoteText (text))

@[expose] public def binderName (text : ByteArray) : ByteArray := (if (Nat.beq ((LexLeanRuntime.length (text) : Nat)) (0)) then lit ("_") else spell (text))

@[expose] public def canonicalNatural (text : ByteArray) : Bool := both ((Nat.blt (0) ((LexLeanRuntime.length (text) : Nat)))) (both ((Nat.beq (digitsEnd (text) (0)) ((LexLeanRuntime.length (text) : Nat)))) ((if same (text) (lit ("0")) then true else (if LexLeanStd.Text.startsWith (text) (lit ("0")) then false else true))))

@[expose] public def canonicalInteger (text : ByteArray) : Bool := (let digits : ByteArray := (if LexLeanStd.Text.startsWith (text) (lit ("-")) then LexLeanStd.Text.sliceOr (text) (1) ((LexLeanRuntime.subtract ((LexLeanRuntime.length (text) : Nat)) (1) : Nat)) else text); both (canonicalNatural (digits)) ((if same (text) (lit ("-0")) then false else true)))

@[expose] public def shownText (shown : Shown) : ByteArray := (if (Nat.beq ((shown).count) (0)) then (if (shown).zero then lit ("0") else (shown).base) else (if (shown).zero then num ((shown).count) else cat (cat ((shown).base) (lit ("+"))) (num ((shown).count))))

@[expose] public def levelShown : (level : LexLeanBoot.CoreJson.Level) -> Shown
  | LexLeanBoot.CoreJson.Level.LZero => ({ base := lit (""), zero := true, count := 0 } : Shown)
  | LexLeanBoot.CoreJson.Level.LParam label => ({ base := spell (label), zero := false, count := 0 } : Shown)
  | LexLeanBoot.CoreJson.Level.LSucc inner => (let below : Shown := levelShown (inner); ({ base := (below).base, zero := (below).zero, count := ((below).count + 1) } : Shown))
  | LexLeanBoot.CoreJson.Level.LMax low high => (let left : Shown := levelShown (low); (let right : Shown := levelShown (high); ({ base := cat (cat (cat (cat (lit ("max(")) (shownText (left))) (lit (", "))) (shownText (right))) (lit (")")), zero := false, count := 0 } : Shown)))
  | LexLeanBoot.CoreJson.Level.LIMax low high => (let left : Shown := levelShown (low); (let right : Shown := levelShown (high); ({ base := cat (cat (cat (cat (lit ("imax(")) (shownText (left))) (lit (", "))) (shownText (right))) (lit (")")), zero := false, count := 0 } : Shown)))

@[expose] public def levelText (level : LexLeanBoot.CoreJson.Level) : ByteArray := shownText (levelShown (level))

@[expose] public def levelsText (levels : List (LexLeanBoot.CoreJson.Level)) : ByteArray := (if (Nat.beq ((LexLeanRuntime.length (levels) : Nat)) (0)) then lit ("") else cat (cat (lit (".{")) (joined (LexLeanStd.Order.reverseList (ByteArray) ((LexLeanCollections.listFold ((fun (out : List (ByteArray)) (level : LexLeanBoot.CoreJson.Level) => (levelText (level) :: out))) (([] : List (ByteArray))) (levels) : List (ByteArray)))) (lit (", ")))) (lit ("}")))

@[expose] public def metaText (entry : LexLeanBoot.CoreJson.Entry) : ByteArray := cat (cat (spell ((entry).key)) (lit (" := "))) ((match (entry).value with | LexLeanBoot.CoreJson.Meta.MBool flag => (if flag then lit ("bool true") else lit ("bool false")) | LexLeanBoot.CoreJson.Meta.MNat text => cat (lit ("nat ")) ((if canonicalNatural (text) then text else LexLeanBoot.SurfaceLex.quoteText (text))) | LexLeanBoot.CoreJson.Meta.MInt text => cat (lit ("int ")) ((if canonicalInteger (text) then text else LexLeanBoot.SurfaceLex.quoteText (text))) | LexLeanBoot.CoreJson.Meta.MStr text => cat (lit ("str ")) (LexLeanBoot.SurfaceLex.quoteText (text)) | LexLeanBoot.CoreJson.Meta.MName text => cat (lit ("name ")) (spell (text))))

@[expose] public def tt (text : ByteArray) : Tdoc := Tdoc.TText (text)

@[expose] public def tseq (parts : List (Tdoc)) : Tdoc := (LexLeanCollections.listFold ((fun (rest : Tdoc) (part : Tdoc) => Tdoc.TSeq (part) (rest))) (Tdoc.TNil) (LexLeanStd.Order.reverseList (Tdoc) (parts)) : Tdoc)

@[expose] public def paren (inner : Tdoc) : Tdoc := tseq ((tt (lit ("(")) :: (inner :: (tt (lit (")")) :: ([] : List (Tdoc))))))

@[expose] public def appDoc (head : Tdoc) (argsRev : List (Tdoc)) : Tdoc := (match argsRev with | List.nil => head | List.cons _ _ => Tdoc.TGroup (tseq ((head :: (Tdoc.TNest (2) (tseq ((LexLeanCollections.listFold ((fun (out : List (Tdoc)) (argument : Tdoc) => (Tdoc.TLine :: (argument :: out)))) (([] : List (Tdoc))) (argsRev) : List (Tdoc)))) :: ([] : List (Tdoc)))))))

@[expose] public def whole (term : Term) : Tdoc := (match term with | Term.Atom inner => inner | Term.App head argsRev => appDoc (head) (argsRev) | Term.Prefix inner => inner | Term.Low inner => inner)

@[expose] public def asArgument (term : Term) : Tdoc := (match term with | Term.Atom inner => inner | Term.App _ _ => paren (whole (term)) | Term.Prefix inner => paren (inner) | Term.Low inner => paren (inner))

@[expose] public def asDomain (term : Term) : Tdoc := (match term with | Term.Low inner => paren (inner) | Term.Atom inner => inner | Term.App _ _ => whole (term) | Term.Prefix inner => inner)

@[expose] public def headOf (term : Term) : Head := (match term with | Term.Atom inner => ({ head := inner, args := ([] : List (Tdoc)) } : Head) | Term.App head argsRev => ({ head := head, args := argsRev } : Head) | Term.Prefix inner => ({ head := paren (inner), args := ([] : List (Tdoc)) } : Head) | Term.Low inner => ({ head := paren (inner), args := ([] : List (Tdoc)) } : Head))

@[expose] public def binderDoc (info : LexLeanBoot.CoreJson.Binder) (label : ByteArray) (bound : Term) : Tdoc := (let marks : (Prod (ByteArray) (ByteArray)) := (match info with | LexLeanBoot.CoreJson.Binder.BExplicit => (lit ("("), lit (")")) | LexLeanBoot.CoreJson.Binder.BImplicit => (lit ("{"), lit ("}")) | LexLeanBoot.CoreJson.Binder.BStrict => (lit ("{{"), lit ("}}")) | LexLeanBoot.CoreJson.Binder.BInstance => (lit ("["), lit ("]"))); tseq ((tt (cat (cat ((marks).1) (binderName (label))) (lit (" : "))) :: (whole (bound) :: (tt ((marks).2) :: ([] : List (Tdoc)))))))

@[expose] public def isExplicit (info : LexLeanBoot.CoreJson.Binder) : Bool := (match info with | LexLeanBoot.CoreJson.Binder.BExplicit => true | LexLeanBoot.CoreJson.Binder.BImplicit => false | LexLeanBoot.CoreJson.Binder.BStrict => false | LexLeanBoot.CoreJson.Binder.BInstance => false)

@[expose] public def scalarValid (scalar : Nat) : Bool := both ((Nat.ble (scalar) (1114111))) ((if both ((Nat.ble (55296) (scalar))) ((Nat.ble (scalar) (57343))) then false else true))

@[expose] public def utf8 (scalar : Nat) : ByteArray := (if (Nat.blt (scalar) (128)) then LexLeanBoot.SurfaceLex.byteOf (scalar) else (if (Nat.blt (scalar) (2048)) then cat (LexLeanBoot.SurfaceLex.byteOf ((192 + (LexLeanRuntime.quotient (scalar) (64) (0) : Nat)))) (LexLeanBoot.SurfaceLex.byteOf ((128 + (LexLeanRuntime.remainder (scalar) (64) (0) : Nat)))) else (if (Nat.blt (scalar) (65536)) then cat (cat (LexLeanBoot.SurfaceLex.byteOf ((224 + (LexLeanRuntime.quotient (scalar) (4096) (0) : Nat)))) (LexLeanBoot.SurfaceLex.byteOf ((128 + (LexLeanRuntime.remainder ((LexLeanRuntime.quotient (scalar) (64) (0) : Nat)) (64) (0) : Nat))))) (LexLeanBoot.SurfaceLex.byteOf ((128 + (LexLeanRuntime.remainder (scalar) (64) (0) : Nat)))) else cat (cat (cat (LexLeanBoot.SurfaceLex.byteOf ((240 + (LexLeanRuntime.quotient (scalar) (262144) (0) : Nat)))) (LexLeanBoot.SurfaceLex.byteOf ((128 + (LexLeanRuntime.remainder ((LexLeanRuntime.quotient (scalar) (4096) (0) : Nat)) (64) (0) : Nat))))) (LexLeanBoot.SurfaceLex.byteOf ((128 + (LexLeanRuntime.remainder ((LexLeanRuntime.quotient (scalar) (64) (0) : Nat)) (64) (0) : Nat))))) (LexLeanBoot.SurfaceLex.byteOf ((128 + (LexLeanRuntime.remainder (scalar) (64) (0) : Nat)))))))

@[expose] public def termAt (terms : List (Term)) (position : Nat) : Term := (match (LexLeanRuntime.index (terms) (position) : Option (Term)) with | Option.some term => term | Option.none => Term.Atom (Tdoc.TNil))

@[expose] public def stringError (index : Nat) (scalars : List (Nat)) : Option (ByteArray) := (LexLeanCollections.listFold ((fun (found : Option (ByteArray)) (scalar : Nat) => (match found with | Option.some _ => found | Option.none => (if scalarValid (scalar) then Option.none else Option.some (cat (cat (cat (cat (lit ("node ")) (num (index))) (lit (": "))) (num (scalar))) (lit (" is not a Unicode scalar value"))))))) (Option.none) (scalars) : Option (ByteArray))

@[expose] public def nodeTerm (node : LexLeanBoot.CoreJson.Node) (terms : List (Term)) : Term := (match node with | LexLeanBoot.CoreJson.Node.NBVar position => Term.Atom (Tdoc.TVar (position)) | LexLeanBoot.CoreJson.Node.NSort level => Term.Prefix (tt (cat (lit ("Sort ")) (levelText (level)))) | LexLeanBoot.CoreJson.Node.NConst label levels => Term.Atom (Tdoc.TConst (label) (levelsText (levels))) | LexLeanBoot.CoreJson.Node.NApp _ _ => (let head : Head := headOf (termAt (terms) (0)); Term.App ((head).head) ((asArgument (termAt (terms) (1)) :: (head).args))) | LexLeanBoot.CoreJson.Node.NLambda label info _ _ => Term.Low (Tdoc.TGroup (tseq ((tt (lit ("fun ")) :: (binderDoc (info) (label) (termAt (terms) (0)) :: (tt (lit (" =>")) :: (Tdoc.TLine :: (Tdoc.TBind (label) (whole (termAt (terms) (1))) :: ([] : List (Tdoc)))))))))) | LexLeanBoot.CoreJson.Node.NForall label info _ _ => Term.Low (Tdoc.TGroup (tseq (((if both ((Nat.beq ((LexLeanRuntime.length (label) : Nat)) (0))) (isExplicit (info)) then asDomain (termAt (terms) (0)) else binderDoc (info) (label) (termAt (terms) (0))) :: (tt (lit (" ->")) :: (Tdoc.TLine :: (Tdoc.TBind (label) (whole (termAt (terms) (1))) :: ([] : List (Tdoc))))))))) | LexLeanBoot.CoreJson.Node.NLet label _ _ _ dependent => Term.Low (Tdoc.TGroup (tseq ((tt (cat (cat ((if dependent then lit ("have ") else lit ("let "))) (binderName (label))) (lit (" : "))) :: (whole (termAt (terms) (0)) :: (tt (lit (" := ")) :: (whole (termAt (terms) (1)) :: (tt (lit (";")) :: (Tdoc.TLine :: (Tdoc.TBind (label) (whole (termAt (terms) (2))) :: ([] : List (Tdoc)))))))))))) | LexLeanBoot.CoreJson.Node.NNat text => Term.Atom (tt ((if canonicalNatural (text) then text else cat (lit ("nat ")) (LexLeanBoot.SurfaceLex.quoteText (text))))) | LexLeanBoot.CoreJson.Node.NString scalars => Term.Atom (tt (LexLeanBoot.SurfaceLex.quoteText ((LexLeanCollections.listFold ((fun (out : ByteArray) (scalar : Nat) => cat (out) (utf8 (scalar)))) (lit ("")) (scalars) : ByteArray)))) | LexLeanBoot.CoreJson.Node.NMdata entries _ => Term.Prefix (Tdoc.TGroup (tseq ((tt (cat (cat (lit ("annot [")) (joined (LexLeanStd.Order.reverseList (ByteArray) ((LexLeanCollections.listFold ((fun (out : List (ByteArray)) (entry : LexLeanBoot.CoreJson.Entry) => (metaText (entry) :: out))) (([] : List (ByteArray))) (entries) : List (ByteArray)))) (lit (", ")))) (lit ("]"))) :: (Tdoc.TLine :: (asArgument (termAt (terms) (0)) :: ([] : List (Tdoc)))))))) | LexLeanBoot.CoreJson.Node.NProj label «structure» position _ => Term.Prefix (Tdoc.TGroup (tseq ((tt (cat (cat (cat (cat (cat (lit ("proj ")) (spell («structure»))) (lit (" "))) (spell (label))) (lit (" "))) (num (position))) :: (Tdoc.TLine :: (asArgument (termAt (terms) (0)) :: ([] : List (Tdoc)))))))))

@[expose] public def childrenOf (node : LexLeanBoot.CoreJson.Node) : List (Nat) := (match node with | LexLeanBoot.CoreJson.Node.NApp function applied => (function :: (applied :: ([] : List (Nat)))) | LexLeanBoot.CoreJson.Node.NLambda _ _ bound body => (bound :: (body :: ([] : List (Nat)))) | LexLeanBoot.CoreJson.Node.NForall _ _ bound body => (bound :: (body :: ([] : List (Nat)))) | LexLeanBoot.CoreJson.Node.NLet _ bound value rest _ => (bound :: (value :: (rest :: ([] : List (Nat))))) | LexLeanBoot.CoreJson.Node.NMdata _ inner => (inner :: ([] : List (Nat))) | LexLeanBoot.CoreJson.Node.NProj _ _ _ inner => (inner :: ([] : List (Nat))) | LexLeanBoot.CoreJson.Node.NBVar _ => ([] : List (Nat)) | LexLeanBoot.CoreJson.Node.NSort _ => ([] : List (Nat)) | LexLeanBoot.CoreJson.Node.NConst _ _ => ([] : List (Nat)) | LexLeanBoot.CoreJson.Node.NNat _ => ([] : List (Nat)) | LexLeanBoot.CoreJson.Node.NString _ => ([] : List (Nat)))

@[expose] public def asNat (found : Option (LexLeanStd.Json.Json)) : Option (Nat) := (match found with | Option.some value => (match value with | LexLeanStd.Json.Json.JInt negative magnitude => (if negative then Option.none else Option.some (magnitude)) | LexLeanStd.Json.Json.JBool _ => Option.none | LexLeanStd.Json.Json.JStr _ => Option.none | LexLeanStd.Json.Json.JArr _ => Option.none | LexLeanStd.Json.Json.JObj _ => Option.none | LexLeanStd.Json.Json.JNil => Option.none | LexLeanStd.Json.Json.JItem _ _ => Option.none | LexLeanStd.Json.Json.JMember _ _ _ => Option.none) | Option.none => Option.none)

@[expose] public def lookStep (key : ByteArray) (state : (Prod (LexLeanStd.Json.Json) (Option (LexLeanStd.Json.Json)))) : Option ((Prod (LexLeanStd.Json.Json) (Option (LexLeanStd.Json.Json)))) := (match (state).2 with | Option.some _ => Option.none | Option.none => (match (state).1 with | LexLeanStd.Json.Json.JMember name value rest => Option.some ((if same (name) (key) then (LexLeanStd.Json.Json.JNil, Option.some (value)) else (rest, Option.none))) | LexLeanStd.Json.Json.JBool _ => Option.none | LexLeanStd.Json.Json.JInt _ _ => Option.none | LexLeanStd.Json.Json.JStr _ => Option.none | LexLeanStd.Json.Json.JArr _ => Option.none | LexLeanStd.Json.Json.JObj _ => Option.none | LexLeanStd.Json.Json.JNil => Option.none | LexLeanStd.Json.Json.JItem _ _ => Option.none))

@[expose] public def getIn (value : LexLeanStd.Json.Json) (key : ByteArray) : Option (LexLeanStd.Json.Json) := (match value with | LexLeanStd.Json.Json.JObj chain => (((LexLeanCollections.iterateUntil ((fun (state : (Prod (LexLeanStd.Json.Json) (Option (LexLeanStd.Json.Json)))) => lookStep (key) (state))) (1000000) ((chain, Option.none)) : (Prod ((Prod (LexLeanStd.Json.Json) (Option (LexLeanStd.Json.Json)))) (Bool)))).1).2 | LexLeanStd.Json.Json.JBool _ => Option.none | LexLeanStd.Json.Json.JInt _ _ => Option.none | LexLeanStd.Json.Json.JStr _ => Option.none | LexLeanStd.Json.Json.JArr _ => Option.none | LexLeanStd.Json.Json.JNil => Option.none | LexLeanStd.Json.Json.JItem _ _ => Option.none | LexLeanStd.Json.Json.JMember _ _ _ => Option.none)

@[expose] public def getOpt (value : Option (LexLeanStd.Json.Json)) (key : ByteArray) : Option (LexLeanStd.Json.Json) := (match value with | Option.some inner => getIn (inner) (key) | Option.none => Option.none)

@[expose] public def itemStep (state : (Prod (LexLeanStd.Json.Json) (List (LexLeanStd.Json.Json)))) : Option ((Prod (LexLeanStd.Json.Json) (List (LexLeanStd.Json.Json)))) := (match (state).1 with | LexLeanStd.Json.Json.JItem element rest => Option.some ((rest, (element :: (state).2))) | LexLeanStd.Json.Json.JBool _ => Option.none | LexLeanStd.Json.Json.JInt _ _ => Option.none | LexLeanStd.Json.Json.JStr _ => Option.none | LexLeanStd.Json.Json.JArr _ => Option.none | LexLeanStd.Json.Json.JObj _ => Option.none | LexLeanStd.Json.Json.JNil => Option.none | LexLeanStd.Json.Json.JMember _ _ _ => Option.none)

@[expose] public def itemsOf (value : Option (LexLeanStd.Json.Json)) : List (LexLeanStd.Json.Json) := (match value with | Option.some array => (match array with | LexLeanStd.Json.Json.JArr chain => LexLeanStd.Order.reverseList (LexLeanStd.Json.Json) ((((LexLeanCollections.iterateUntil ((fun (state : (Prod (LexLeanStd.Json.Json) (List (LexLeanStd.Json.Json)))) => itemStep (state))) (1000000) ((chain, ([] : List (LexLeanStd.Json.Json)))) : (Prod ((Prod (LexLeanStd.Json.Json) (List (LexLeanStd.Json.Json)))) (Bool)))).1).2) | LexLeanStd.Json.Json.JBool _ => ([] : List (LexLeanStd.Json.Json)) | LexLeanStd.Json.Json.JInt _ _ => ([] : List (LexLeanStd.Json.Json)) | LexLeanStd.Json.Json.JStr _ => ([] : List (LexLeanStd.Json.Json)) | LexLeanStd.Json.Json.JObj _ => ([] : List (LexLeanStd.Json.Json)) | LexLeanStd.Json.Json.JNil => ([] : List (LexLeanStd.Json.Json)) | LexLeanStd.Json.Json.JItem _ _ => ([] : List (LexLeanStd.Json.Json)) | LexLeanStd.Json.Json.JMember _ _ _ => ([] : List (LexLeanStd.Json.Json))) | Option.none => ([] : List (LexLeanStd.Json.Json)))

@[expose] public def strOf (value : Option (LexLeanStd.Json.Json)) : ByteArray := (match value with | Option.some inner => (match inner with | LexLeanStd.Json.Json.JStr text => text | LexLeanStd.Json.Json.JBool _ => lit ("") | LexLeanStd.Json.Json.JInt _ _ => lit ("") | LexLeanStd.Json.Json.JArr _ => lit ("") | LexLeanStd.Json.Json.JObj _ => lit ("") | LexLeanStd.Json.Json.JNil => lit ("") | LexLeanStd.Json.Json.JItem _ _ => lit ("") | LexLeanStd.Json.Json.JMember _ _ _ => lit ("")) | Option.none => lit (""))

@[expose] public def pushOpt (out : List (Nat)) (found : Option (Nat)) : List (Nat) := (match found with | Option.some value => (LexLeanRuntime.append (out) ((value :: ([] : List (Nat)))) : List (Nat)) | Option.none => out)

@[expose] public def rootsOf (declaration : LexLeanStd.Json.Json) : List (Nat) := (let equations : Option (LexLeanStd.Json.Json) := getIn (declaration) (lit ("eqn_info")); (let base : List (Nat) := pushOpt (pushOpt (pushOpt (pushOpt (([] : List (Nat))) (asNat (getIn (declaration) (lit ("type"))))) (asNat (getIn (declaration) (lit ("value"))))) (asNat (getOpt (equations) (lit ("type"))))) (asNat (getOpt (equations) (lit ("value")))); (LexLeanCollections.listFold ((fun (out : List (Nat)) (field : LexLeanStd.Json.Json) => pushOpt (out) (asNat (getIn (field) (lit ("auto_param")))))) (base) (itemsOf (getOpt (getOpt (getIn (declaration) (lit ("inductive"))) (lit ("structure"))) (lit ("fields")))) : List (Nat))))

@[expose] public def edgeError (nodes : List (LexLeanBoot.CoreJson.Node)) : Option (ByteArray) := ((LexLeanCollections.listFold ((fun (state : (Prod (Nat) (Option (ByteArray)))) (node : LexLeanBoot.CoreJson.Node) => (let index : Nat := (state).1; ((index + 1), (match (state).2 with | Option.some _ => (state).2 | Option.none => (LexLeanCollections.listFold ((fun (found : Option (ByteArray)) (child : Nat) => (match found with | Option.some _ => found | Option.none => (if (Nat.ble (index) (child)) then Option.some (cat (cat (cat (cat (lit ("node ")) (num (index))) (lit (" points to the node "))) (num (child))) (lit (", which is not earlier"))) else Option.none)))) (Option.none) (childrenOf (node)) : Option (ByteArray))))))) ((0, Option.none)) (nodes) : (Prod (Nat) (Option (ByteArray))))).2

@[expose] public def proofError (proofs : List (Nat)) (count : Nat) : Option (ByteArray) := (let fine : (Prod (Bool) (Option (Nat))) := (LexLeanCollections.listFold ((fun (state : (Prod (Bool) (Option (Nat)))) (proof : Nat) => (both ((state).1) (both ((Nat.blt (proof) (count))) ((match (state).2 with | Option.some prior => (Nat.blt (prior) (proof)) | Option.none => true))), Option.some (proof)))) ((true, Option.none)) (proofs) : (Prod (Bool) (Option (Nat)))); (if (fine).1 then Option.none else Option.some (lit ("the proof nodes are not strictly increasing nodes of the module"))))

@[expose] public def rootError (declarations : List (LexLeanStd.Json.Json)) (count : Nat) : Option (ByteArray) := (LexLeanCollections.listFold ((fun (found : Option (ByteArray)) (declaration : LexLeanStd.Json.Json) => (match found with | Option.some _ => found | Option.none => (if (LexLeanCollections.listFold ((fun (bad : Bool) (root : Nat) => (if bad then true else (Nat.ble (count) (root))))) (false) (rootsOf (declaration)) : Bool) then Option.some (cat (cat (lit ("`")) (strOf (getIn (declaration) (lit ("name"))))) (lit ("` has a root that is not a node of the module"))) else Option.none)))) (Option.none) (declarations) : Option (ByteArray))

@[expose] public def keyOf (value : Nat) : ByteArray := ((LexLeanCollections.iterate ((fun (state : (Prod (Nat) (ByteArray))) => ((LexLeanRuntime.quotient ((state).1) (256) (0) : Nat), cat (LexLeanBoot.SurfaceLex.byteOf ((LexLeanRuntime.remainder ((state).1) (256) (0) : Nat))) ((state).2)))) (8) ((value, lit (""))) : (Prod (Nat) (ByteArray)))).2

@[expose] public def sortedRefs (nodes : List (LexLeanBoot.CoreJson.Node)) (declarations : List (LexLeanStd.Json.Json)) : List (Nat) := (let all : List (Nat) := (LexLeanCollections.listFold ((fun (out : List (Nat)) (declaration : LexLeanStd.Json.Json) => (LexLeanCollections.listFold ((fun (inner : List (Nat)) (root : Nat) => (root :: inner))) (out) (rootsOf (declaration)) : List (Nat)))) ((LexLeanCollections.listFold ((fun (out : List (Nat)) (node : LexLeanBoot.CoreJson.Node) => (LexLeanCollections.listFold ((fun (inner : List (Nat)) (child : Nat) => (child :: inner))) (out) (childrenOf (node)) : List (Nat)))) (([] : List (Nat))) (nodes) : List (Nat))) (declarations) : List (Nat)); (LexLeanCollections.listFold ((fun (out : List (Nat)) (entry : (Prod (ByteArray) (Nat))) => ((entry).2 :: out))) (([] : List (Nat))) (LexLeanStd.Order.reverseList ((Prod (ByteArray) (Nat))) (LexLeanStd.Order.sortByKey (Nat) ((LexLeanCollections.listFold ((fun (out : List ((Prod (ByteArray) (Nat)))) (value : Nat) => ((keyOf (value), value) :: out))) (([] : List ((Prod (ByteArray) (Nat))))) (all) : List ((Prod (ByteArray) (Nat))))))) : List (Nat)))

@[expose] public def popCount (refs : List (Nat)) (index : Nat) : (Prod (Nat) (List (Nat))) := ((LexLeanCollections.iterateUntil ((fun (state : (Prod (Nat) (List (Nat)))) => (match (state).2 with | List.cons head tail => (if (Nat.beq (head) (index)) then Option.some ((((state).1 + 1), tail)) else Option.none) | List.nil => Option.none))) (((LexLeanRuntime.length (refs) : Nat) + 1)) ((0, refs)) : (Prod ((Prod (Nat) (List (Nat)))) (Bool)))).1

@[expose] public def takeStep (take : Take) (child : Nat) : Take := (let fallback : Take := ({ stack := (take).stack, terms := (Term.Atom (tt (cat (lit ("@")) (num (child)))) :: (take).terms), tallest := (take).tallest } : Take); (match (take).stack with | List.cons top below => (if both ((Nat.beq ((top).index) (child))) (both ((Nat.beq ((top).uses) (1))) ((Nat.blt ((top).height) (48)))) then ({ stack := below, terms := ((top).term :: (take).terms), tallest := (if (Nat.blt ((take).tallest) ((top).height)) then (top).height else (take).tallest) } : Take) else fallback) | List.nil => fallback))

@[expose] public def wrapProof (term : Term) (proof : Bool) : Term := (if proof then Term.Atom (tseq ((tt (lit ("proof(")) :: (Tdoc.TNest (2) (whole (term)) :: (tt (lit (")")) :: ([] : List (Tdoc))))))) else term)

@[expose] public def passStep (pass : Pass) (node : LexLeanBoot.CoreJson.Node) : Pass := (match (pass).error with | Option.some _ => pass | Option.none => (let index : Nat := (pass).index; (let counted : (Prod (Nat) (List (Nat))) := popCount ((pass).refs) (index); (let proof : Bool := (match (pass).proofs with | List.cons head _ => (Nat.beq (head) (index)) | List.nil => false); (let take : Take := (LexLeanCollections.listFold ((fun (state : Take) (child : Nat) => takeStep (state) (child))) (({ stack := (pass).stack, terms := ([] : List (Term)), tallest := 0 } : Take)) (LexLeanStd.Order.reverseList (Nat) (childrenOf (node))) : Take); (let failure : Option (ByteArray) := (match node with | LexLeanBoot.CoreJson.Node.NString scalars => stringError (index) (scalars) | LexLeanBoot.CoreJson.Node.NBVar _ => Option.none | LexLeanBoot.CoreJson.Node.NSort _ => Option.none | LexLeanBoot.CoreJson.Node.NConst _ _ => Option.none | LexLeanBoot.CoreJson.Node.NApp _ _ => Option.none | LexLeanBoot.CoreJson.Node.NLambda _ _ _ _ => Option.none | LexLeanBoot.CoreJson.Node.NForall _ _ _ _ => Option.none | LexLeanBoot.CoreJson.Node.NLet _ _ _ _ _ => Option.none | LexLeanBoot.CoreJson.Node.NNat _ => Option.none | LexLeanBoot.CoreJson.Node.NMdata _ _ => Option.none | LexLeanBoot.CoreJson.Node.NProj _ _ _ _ => Option.none); (let start : Nat := (match (take).stack with | List.cons top _ => ((top).index + 1) | List.nil => 0); ({ index := (index + 1), stack := (({ index := index, start := start, height := ((take).tallest + 1), uses := (counted).1, term := wrapProof (nodeTerm (node) ((take).terms)) (proof) } : Slot) :: (take).stack), refs := (counted).2, proofs := (if proof then (match (pass).proofs with | List.cons _ tail => tail | List.nil => ([] : List (Nat))) else (pass).proofs), error := failure } : Pass))))))))

public structure Near where
  seen : Nat
  hit : Bool

@[expose] public def nearStep (state : Near) (nearer : ByteArray) (label : ByteArray) (position : Nat) : Near := ({ seen := ((state).seen + 1), hit := (if (state).hit then true else both ((Nat.blt ((state).seen) (position))) (same (nearer) (label))) } : Near)

@[expose] public def «variable» (scope : List (ByteArray)) (position : Nat) : ByteArray := (let fallback : ByteArray := cat (lit ("#")) (num (position)); (match (LexLeanRuntime.index (scope) (position) : Option (ByteArray)) with | Option.some label => (if both (bare (label)) ((if ((LexLeanCollections.listFold ((fun (state : Near) (nearer : ByteArray) => nearStep (state) (nearer) (label) (position))) (({ seen := 0, hit := false } : Near)) (scope) : Near)).hit then false else true)) then label else fallback) | Option.none => fallback))

@[expose] public def resolve : (node : Tdoc) -> (scope : List (ByteArray)) -> LexLeanBoot.SurfaceDoc.Doc
  | Tdoc.TNil, _scope => LexLeanBoot.SurfaceDoc.Doc.DNil
  | Tdoc.TText text, _scope => LexLeanBoot.SurfaceDoc.Doc.DText (text)
  | Tdoc.TSeq head tail, scope => LexLeanBoot.SurfaceDoc.Doc.DSeq (resolve (head) (scope)) (resolve (tail) (scope))
  | Tdoc.TNest amount inner, scope => LexLeanBoot.SurfaceDoc.Doc.DNest (amount) (resolve (inner) (scope))
  | Tdoc.TLine, _scope => LexLeanBoot.SurfaceDoc.Doc.DLine
  | Tdoc.TGroup inner, scope => LexLeanBoot.SurfaceDoc.Doc.DGroup (resolve (inner) (scope))
  | Tdoc.TVar position, scope => LexLeanBoot.SurfaceDoc.Doc.DText («variable» (scope) (position))
  | Tdoc.TConst label levels, scope => LexLeanBoot.SurfaceDoc.Doc.DText ((if both (bare (label)) ((if listed (label) (scope) then false else true)) then cat (label) (levels) else cat (cat (lit ("const ")) (LexLeanBoot.SurfaceLex.quoteText (label))) (levels)))
  | Tdoc.TBind label inner, scope => resolve (inner) ((label :: scope))

@[expose] public def render (node : LexLeanBoot.SurfaceDoc.Doc) : ByteArray := LexLeanBoot.SurfaceDoc.renderDoc (node) (100) (LexLeanBoot.SurfaceGeneric.docNodes (node))

@[expose] public def withoutKeys (chain : LexLeanStd.Json.Json) : LexLeanStd.Json.Json := (let kept : List ((Prod (ByteArray) (LexLeanStd.Json.Json))) := (((LexLeanCollections.iterateUntil ((fun (state : (Prod (LexLeanStd.Json.Json) (List ((Prod (ByteArray) (LexLeanStd.Json.Json)))))) => (match (state).1 with | LexLeanStd.Json.Json.JMember key value rest => Option.some ((rest, (if listed (key) ((lit ("kind") :: (lit ("name") :: (lit ("levels") :: (lit ("type") :: (lit ("value") :: ([] : List (ByteArray)))))))) then (state).2 else ((key, value) :: (state).2)))) | LexLeanStd.Json.Json.JBool _ => Option.none | LexLeanStd.Json.Json.JInt _ _ => Option.none | LexLeanStd.Json.Json.JStr _ => Option.none | LexLeanStd.Json.Json.JArr _ => Option.none | LexLeanStd.Json.Json.JObj _ => Option.none | LexLeanStd.Json.Json.JNil => Option.none | LexLeanStd.Json.Json.JItem _ _ => Option.none))) (1000000) ((chain, ([] : List ((Prod (ByteArray) (LexLeanStd.Json.Json)))))) : (Prod ((Prod (LexLeanStd.Json.Json) (List ((Prod (ByteArray) (LexLeanStd.Json.Json)))))) (Bool)))).1).2; (LexLeanCollections.listFold ((fun (rest : LexLeanStd.Json.Json) (entry : (Prod (ByteArray) (LexLeanStd.Json.Json))) => LexLeanStd.Json.Json.JMember ((entry).1) ((entry).2) (rest))) (LexLeanStd.Json.Json.JNil) (kept) : LexLeanStd.Json.Json))

@[expose] public def declarationText (declaration : LexLeanStd.Json.Json) : Except (ByteArray) (ByteArray) := (match declaration with | LexLeanStd.Json.Json.JObj chain => (match getIn (declaration) (lit ("kind")) with | Option.some kind => (match kind with | LexLeanStd.Json.Json.JStr written => (let levels : List (ByteArray) := (LexLeanCollections.listFold ((fun (out : List (ByteArray)) (level : LexLeanStd.Json.Json) => (LexLeanRuntime.append (out) ((spell (strOf (Option.some (level))) :: ([] : List (ByteArray)))) : List (ByteArray)))) (([] : List (ByteArray))) (itemsOf (getIn (declaration) (lit ("levels")))) : List (ByteArray)); (let head : ByteArray := cat (cat (cat (cat (cat (written) (lit (" "))) (spell (strOf (getIn (declaration) (lit ("name")))))) ((if (Nat.beq ((LexLeanRuntime.length (levels) : Nat)) (0)) then lit ("") else cat (cat (lit (".{")) (joined (levels) (lit (", ")))) (lit ("}"))))) ((match asNat (getIn (declaration) (lit ("type"))) with | Option.some root => cat (lit (" : @")) (num (root)) | Option.none => lit ("")))) ((match asNat (getIn (declaration) (lit ("value"))) with | Option.some root => cat (lit (" := @")) (num (root)) | Option.none => lit (""))); (let rest : LexLeanStd.Json.Json := withoutKeys (chain); Except.ok (render ((match rest with | LexLeanStd.Json.Json.JNil => LexLeanBoot.SurfaceDoc.Doc.DText (head) | LexLeanStd.Json.Json.JBool _ => LexLeanBoot.SurfaceDoc.Doc.DText (head) | LexLeanStd.Json.Json.JInt _ _ => LexLeanBoot.SurfaceDoc.Doc.DText (head) | LexLeanStd.Json.Json.JStr _ => LexLeanBoot.SurfaceDoc.Doc.DText (head) | LexLeanStd.Json.Json.JArr _ => LexLeanBoot.SurfaceDoc.Doc.DText (head) | LexLeanStd.Json.Json.JObj _ => LexLeanBoot.SurfaceDoc.Doc.DText (head) | LexLeanStd.Json.Json.JItem _ _ => LexLeanBoot.SurfaceDoc.Doc.DText (head) | LexLeanStd.Json.Json.JMember _ _ _ => LexLeanBoot.SurfaceDoc.Doc.DSeq (LexLeanBoot.SurfaceDoc.Doc.DText (head)) (LexLeanBoot.SurfaceDoc.Doc.DSeq (LexLeanBoot.SurfaceDoc.Doc.DText (lit (" "))) (LexLeanBoot.SurfaceDoc.Doc.DSeq (LexLeanBoot.SurfaceGeneric.docValue (LexLeanStd.Json.Json.JObj (rest))) (LexLeanBoot.SurfaceDoc.Doc.DNil))))))))) | LexLeanStd.Json.Json.JBool _ => Except.error (lit ("a declaration has a kind")) | LexLeanStd.Json.Json.JInt _ _ => Except.error (lit ("a declaration has a kind")) | LexLeanStd.Json.Json.JArr _ => Except.error (lit ("a declaration has a kind")) | LexLeanStd.Json.Json.JObj _ => Except.error (lit ("a declaration has a kind")) | LexLeanStd.Json.Json.JNil => Except.error (lit ("a declaration has a kind")) | LexLeanStd.Json.Json.JItem _ _ => Except.error (lit ("a declaration has a kind")) | LexLeanStd.Json.Json.JMember _ _ _ => Except.error (lit ("a declaration has a kind"))) | Option.none => Except.error (lit ("a declaration has a kind"))) | LexLeanStd.Json.Json.JBool _ => Except.error (lit ("a declaration is an object")) | LexLeanStd.Json.Json.JInt _ _ => Except.error (lit ("a declaration is an object")) | LexLeanStd.Json.Json.JStr _ => Except.error (lit ("a declaration is an object")) | LexLeanStd.Json.Json.JArr _ => Except.error (lit ("a declaration is an object")) | LexLeanStd.Json.Json.JNil => Except.error (lit ("a declaration is an object")) | LexLeanStd.Json.Json.JItem _ _ => Except.error (lit ("a declaration is an object")) | LexLeanStd.Json.Json.JMember _ _ _ => Except.error (lit ("a declaration is an object")))

@[expose] public def refusal (message : ByteArray) : Except (LexLeanBoot.SurfaceLex.Failure) (ByteArray) := Except.error (({ offset := 0, message := message } : LexLeanBoot.SurfaceLex.Failure))

@[expose] public def firstOf (options : List (Option (ByteArray))) : Option (ByteArray) := (LexLeanCollections.listFold ((fun (found : Option (ByteArray)) (option : Option (ByteArray)) => (match found with | Option.some _ => found | Option.none => option))) (Option.none) (options) : Option (ByteArray))

@[expose] public def statements (module : LexLeanBoot.CoreJson.Module) : Except (ByteArray) (ByteArray) := (let final : Pass := (LexLeanCollections.listFold ((fun (pass : Pass) (node : LexLeanBoot.CoreJson.Node) => passStep (pass) (node))) (({ index := 0, stack := ([] : List (Slot)), refs := sortedRefs ((module).nodes) ((module).declarations), proofs := (module).proofNodes, error := Option.none } : Pass)) ((module).nodes) : Pass); (match (final).error with | Option.some message => Except.error (message) | Option.none => Except.ok ((LexLeanCollections.listFold ((fun (out : ByteArray) (slot : Slot) => cat (out) (cat (render (resolve (tseq ((tt (cat (cat (lit ("@")) (num ((slot).index))) (lit (" := "))) :: (Tdoc.TNest (2) (whole ((slot).term)) :: ([] : List (Tdoc)))))) (([] : List (ByteArray))))) (lit ("\n"))))) (lit ("")) (LexLeanStd.Order.reverseList (Slot) ((final).stack)) : ByteArray))))

@[expose] public def printCoreModule (module : LexLeanBoot.CoreJson.Module) : Except (LexLeanBoot.SurfaceLex.Failure) (ByteArray) := (if same ((module).spec) (lit ("lexlean/core-module/2")) then (let count : Nat := (LexLeanRuntime.length ((module).nodes) : Nat); (match firstOf ((edgeError ((module).nodes) :: (proofError ((module).proofNodes) (count) :: (rootError ((module).declarations) (count) :: ([] : List (Option (ByteArray))))))) with | Option.some message => refusal (message) | Option.none => (let declared : Except (ByteArray) (ByteArray) := (LexLeanCollections.listFold ((fun (state : Except (ByteArray) (ByteArray)) (declaration : LexLeanStd.Json.Json) => (match state with | Except.ok text => (match declarationText (declaration) with | Except.ok line => Except.ok (cat (cat (text) (line)) (lit ("\n"))) | Except.error message => Except.error (message)) | Except.error _ => state))) (Except.ok (lit (""))) ((module).declarations) : Except (ByteArray) (ByteArray)); (match declared with | Except.error message => refusal (message) | Except.ok lines => (match statements (module) with | Except.error message => refusal (message) | Except.ok body => Except.ok (cat (cat (cat (cat (render (LexLeanBoot.SurfaceDoc.Doc.DSeq (LexLeanBoot.SurfaceDoc.Doc.DText (lit ("imports "))) (LexLeanBoot.SurfaceDoc.Doc.DSeq (LexLeanBoot.SurfaceGeneric.delimitedRev (lit ("[")) (lit ("]")) ((LexLeanCollections.listFold ((fun (out : List (LexLeanBoot.SurfaceDoc.Doc)) (name : ByteArray) => (LexLeanBoot.SurfaceDoc.Doc.DText (spell (name)) :: out))) (([] : List (LexLeanBoot.SurfaceDoc.Doc))) ((module).imports) : List (LexLeanBoot.SurfaceDoc.Doc)))) (LexLeanBoot.SurfaceDoc.Doc.DNil)))) (lit ("\n\n"))) (lines)) (lit ("\n"))) (body))))))) else refusal (cat (cat (lit ("the notation names the schema `lexlean/core-module/2`, not `")) ((module).spec)) (lit ("`"))))

end LexLeanBoot.CorePrint
