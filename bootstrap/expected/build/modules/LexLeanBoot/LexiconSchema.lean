module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Text
public import LexLeanBoot.LexiconLse
public import LexLeanBoot.PrimitiveScan
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.LexiconSchema

public structure Shape where
  permitted : Bool
  fixedArity : Option (Nat)
  coreOnly : Bool
  hasSignature : Bool
  requiredChannels : List (Nat)
  needsSome : Bool

public structure Cm where
  pos : Nat
  mode : Nat
  found : Option (Nat)

@[expose] public def lit (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def cat (left : ByteArray) (right : ByteArray) : ByteArray := (LexLeanRuntime.append (left) (right) : ByteArray)

@[expose] public def indexIn (table : List (ByteArray)) (text : ByteArray) : Option (Nat) := ((LexLeanCollections.listFold ((fun (state : (Prod (Nat) (Option (Nat)))) (row : ByteArray) => (match (state).2 with | Option.some _ => state | Option.none => (if (LexLeanRuntime.equal (row) (text) : Bool) then ((state).1, Option.some ((state).1)) else (((state).1 + 1), Option.none))))) ((0, Option.none)) (table) : (Prod (Nat) (Option (Nat))))).2

@[expose] public def natAt (table : List (Nat)) (wanted : Nat) : Nat := ((LexLeanCollections.listFold ((fun (state : (Prod (Nat) (Nat))) (row : Nat) => (if (Nat.beq ((state).1) (wanted)) then (((state).1 + 1), row) else (((state).1 + 1), (state).2)))) ((0, 0)) (table) : (Prod (Nat) (Nat)))).2

@[expose] public def categories : List (ByteArray) := (lit ("structural") :: (lit ("grammar") :: (lit ("label-word") :: (lit ("type-noun") :: (lit ("term-constant") :: (lit ("function") :: (lit ("prefix-function") :: (lit ("postfix-function") :: (lit ("infix-function") :: (lit ("noun-function") :: (lit ("binary-noun-function") :: (lit ("predicate-constant") :: (lit ("adjective-predicate") :: (lit ("intransitive-predicate") :: (lit ("transitive-predicate") :: (lit ("infix-predicate") :: (lit ("proof-constant") :: ([] : List (ByteArray)))))))))))))))))))

@[expose] public def frames : List (ByteArray) := (lit ("atom") :: (lit ("call") :: (lit ("prefix") :: (lit ("postfix") :: (lit ("infix") :: (lit ("noun-of") :: (lit ("binary-noun-of") :: (lit ("adjective") :: (lit ("intransitive") :: (lit ("transitive") :: ([] : List (ByteArray))))))))))))

@[expose] public def channels : List (ByteArray) := (lit ("text") :: (lit ("math") :: (lit ("both") :: ([] : List (ByteArray)))))

@[expose] public def associativities : List (ByteArray) := (lit ("left") :: (lit ("right") :: (lit ("none") :: ([] : List (ByteArray)))))

@[expose] public def features : List (ByteArray) := (lit ("singular") :: (lit ("plural") :: (lit ("sentence-case") :: (lit ("lower-case") :: (lit ("article-a") :: (lit ("article-an") :: ([] : List (ByteArray))))))))

@[expose] public def tableOf (which : Nat) : List (ByteArray) := (if (Nat.beq (which) (0)) then categories else (if (Nat.beq (which) (1)) then frames else (if (Nat.beq (which) (2)) then channels else (if (Nat.beq (which) (3)) then associativities else features))))

@[expose] public def tokenIndex (which : Nat) (text : ByteArray) : Option (Nat) := indexIn (tableOf (which)) (text)

@[expose] public def framesOfCategories : List (Nat) := (0 :: (0 :: (0 :: (0 :: (0 :: (1 :: (2 :: (3 :: (4 :: (5 :: (6 :: (0 :: (7 :: (8 :: (9 :: (4 :: (0 :: ([] : List (Nat)))))))))))))))))))

@[expose] public def channelsOfCategories : List (Nat) := (0 :: (0 :: (1 :: (0 :: (0 :: (2 :: (2 :: (2 :: (2 :: (1 :: (1 :: (2 :: (1 :: (1 :: (1 :: (2 :: (2 :: ([] : List (Nat)))))))))))))))))))

@[expose] public def permittedFrame (category : Nat) (arity : Nat) : Nat := (if (Nat.beq (category) (16)) then (if (Nat.beq (arity) (0)) then 0 else 1) else natAt (framesOfCategories) (category))

@[expose] public def fixedArity (frame : Nat) : Option (Nat) := (if (Nat.beq (frame) (1)) then Option.none else (if (Nat.beq (frame) (0)) then Option.some (0) else (if (if (Nat.beq (frame) (4)) then true else (if (Nat.beq (frame) (6)) then true else (Nat.beq (frame) (9)))) then Option.some (2) else Option.some (1))))

@[expose] public def requiredChannels (category : Nat) : List (Nat) := (let code : Nat := natAt (channelsOfCategories) (category); (if (Nat.beq (code) (1)) then (0 :: ([] : List (Nat))) else (if (Nat.beq (code) (2)) then (1 :: ([] : List (Nat))) else ([] : List (Nat)))))

@[expose] public def shapeFacts (category : Nat) (frame : Nat) (arity : Nat) : Shape := ({ permitted := (Nat.beq (frame) (permittedFrame (category) (arity))), fixedArity := fixedArity (frame), coreOnly := (Nat.blt (category) (2)), hasSignature := (Nat.blt (2) (category)), requiredChannels := requiredChannels (category), needsSome := (if (Nat.beq (category) (3)) then true else (Nat.beq (category) (4))) } : Shape)

@[expose] public def inRange (code : Nat) (low : Nat) (high : Nat) : Bool := (if (Nat.blt (code) (low)) then false else (Nat.ble (code) (high)))

@[expose] public def isMathSafeCode (code : Nat) : Bool := ((((((Nat.beq (code) (172)) || (Nat.beq (code) (177))) || ((Nat.beq (code) (183)) || (Nat.beq (code) (215)))) || (((Nat.beq (code) (247)) || (if inRange (code) (913) (937) then (if (Nat.beq (code) (930)) then false else true) else false)) || (inRange (code) (945) (969) || (Nat.beq (code) (8230))))) || (((inRange (code) (8242) (8244) || inRange (code) (8448) (8527)) || (inRange (code) (8592) (8703) || inRange (code) (8704) (8959))) || ((inRange (code) (8968) (8971) || inRange (code) (10176) (10223)) || (inRange (code) (10624) (10751) || inRange (code) (10752) (11007))))) || inRange (code) (119808) (120831))

@[expose] public def scalarWidth (lead : Nat) : Nat := (if (Nat.blt (lead) (128)) then 1 else (if (Nat.blt (lead) (224)) then 2 else (if (Nat.blt (lead) (240)) then 3 else 4)))

@[expose] public def contByte (text : ByteArray) (position : Nat) : Nat := (LexLeanRuntime.remainder (LexLeanBoot.LexiconLse.byteNum (text) (position)) (64) (0) : Nat)

@[expose] public def scalarCode (text : ByteArray) (position : Nat) : Nat := (let lead : Nat := LexLeanBoot.LexiconLse.byteNum (text) (position); (let width : Nat := scalarWidth (lead); (if (Nat.beq (width) (1)) then lead else (if (Nat.beq (width) (2)) then ((LexLeanRuntime.multiply ((LexLeanRuntime.remainder (lead) (32) (0) : Nat)) (64) : Nat) + contByte (text) ((position + 1))) else (if (Nat.beq (width) (3)) then (((LexLeanRuntime.multiply ((LexLeanRuntime.remainder (lead) (16) (0) : Nat)) (4096) : Nat) + (LexLeanRuntime.multiply (contByte (text) ((position + 1))) (64) : Nat)) + contByte (text) ((position + 2))) else ((((LexLeanRuntime.multiply ((LexLeanRuntime.remainder (lead) (8) (0) : Nat)) (262144) : Nat) + (LexLeanRuntime.multiply (contByte (text) ((position + 1))) (4096) : Nat)) + (LexLeanRuntime.multiply (contByte (text) ((position + 2))) (64) : Nat)) + contByte (text) ((position + 3))))))))

@[expose] public def mathSafeStep (text : ByteArray) (n : Nat) (position : Nat) : Option (Nat) := (if (Nat.blt (position) (n)) then (if isMathSafeCode (scalarCode (text) (position)) then Option.some ((position + scalarWidth (LexLeanBoot.LexiconLse.byteNum (text) (position)))) else Option.none) else Option.none)

@[expose] public def allMathSafe (text : ByteArray) : Bool := (let n : Nat := (LexLeanRuntime.length (text) : Nat); (Nat.ble (n) (((LexLeanCollections.iterateUntil ((fun (position : Nat) => mathSafeStep (text) (n) (position))) ((n + 1)) (0) : (Prod (Nat) (Bool)))).1)))

@[expose] public def textSafePunctuation : List (ByteArray) := (lit (":") :: (lit ("-") :: (lit (",") :: (lit (".") :: (lit (";") :: (lit ("'") :: ([] : List (ByteArray))))))))

@[expose] public def rendererUnsafeAscii : List (ByteArray) := (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 92))] :: (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 37))] :: (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 36))] :: (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 35))] :: (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 38))] :: (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 126))] :: (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 94))] :: (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 95))] :: (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 123))] :: (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 125))] :: (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 34))] :: (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 96))] :: ([] : List (ByteArray))))))))))))))

@[expose] public def mathDelimiters : List (ByteArray) := (lit ("(") :: (lit (")") :: (lit ("[") :: (lit ("]") :: ([] : List (ByteArray))))))

@[expose] public def isIn (table : List (ByteArray)) (text : ByteArray) : Bool := (match indexIn (table) (text) with | Option.some _ => true | Option.none => false)

@[expose] public def textOk (atom : LexLeanBoot.PrimitiveScan.Atom) : Bool := (match (atom).kind with | LexLeanBoot.PrimitiveScan.AtomClass.Control => false | LexLeanBoot.PrimitiveScan.AtomClass.Word => true | LexLeanBoot.PrimitiveScan.AtomClass.Numeral => false | LexLeanBoot.PrimitiveScan.AtomClass.Delimiter => false | LexLeanBoot.PrimitiveScan.AtomClass.AsciiSymbol => isIn (textSafePunctuation) ((atom).text) | LexLeanBoot.PrimitiveScan.AtomClass.UnicodeSymbol => false | LexLeanBoot.PrimitiveScan.AtomClass.Whitespace => true)

@[expose] public def mathOk (atom : LexLeanBoot.PrimitiveScan.Atom) (place : Nat) : Bool := (match (atom).kind with | LexLeanBoot.PrimitiveScan.AtomClass.Control => false | LexLeanBoot.PrimitiveScan.AtomClass.Word => true | LexLeanBoot.PrimitiveScan.AtomClass.Numeral => (Nat.blt (0) (place)) | LexLeanBoot.PrimitiveScan.AtomClass.Delimiter => isIn (mathDelimiters) ((atom).text) | LexLeanBoot.PrimitiveScan.AtomClass.AsciiSymbol => (if isIn (rendererUnsafeAscii) ((atom).text) then false else true) | LexLeanBoot.PrimitiveScan.AtomClass.UnicodeSymbol => allMathSafe ((atom).text) | LexLeanBoot.PrimitiveScan.AtomClass.Whitespace => true)

@[expose] public def className (atom : LexLeanBoot.PrimitiveScan.Atom) : ByteArray := (match (atom).kind with | LexLeanBoot.PrimitiveScan.AtomClass.Control => lit ("control") | LexLeanBoot.PrimitiveScan.AtomClass.Word => lit ("word") | LexLeanBoot.PrimitiveScan.AtomClass.Numeral => lit ("numeral") | LexLeanBoot.PrimitiveScan.AtomClass.Delimiter => lit ("delimiter") | LexLeanBoot.PrimitiveScan.AtomClass.AsciiSymbol => lit ("ascii-symbol") | LexLeanBoot.PrimitiveScan.AtomClass.UnicodeSymbol => lit ("unicode-symbol") | LexLeanBoot.PrimitiveScan.AtomClass.Whitespace => lit ("whitespace"))

@[expose] public def channelName (channel : Nat) : ByteArray := (if (Nat.beq (channel) (0)) then lit ("text") else (if (Nat.beq (channel) (1)) then lit ("math") else lit ("text and math")))

@[expose] public def atomSafe (atom : LexLeanBoot.PrimitiveScan.Atom) (place : Nat) (channel : Nat) : Bool := (if (Nat.beq (channel) (0)) then textOk (atom) else (if (Nat.beq (channel) (1)) then mathOk (atom) (place) else (if textOk (atom) then mathOk (atom) (place) else false)))

@[expose] public def unsafeMessage (atom : LexLeanBoot.PrimitiveScan.Atom) (channel : Nat) : ByteArray := cat (cat (cat (cat (cat (cat (lit ("`")) ((atom).text)) (lit ("` ("))) (className (atom))) (lit (") is not renderer-safe in the "))) (channelName (channel))) (lit (" channel"))

@[expose] public def surfaceSafety (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (channel : Nat) : Option (ByteArray) := ((LexLeanCollections.listFold ((fun (state : (Prod (Nat) (Option (ByteArray)))) (atom : LexLeanBoot.PrimitiveScan.Atom) => (match (state).2 with | Option.some _ => state | Option.none => (if atomSafe (atom) ((state).1) (channel) then (((state).1 + 1), Option.none) else (((state).1 + 1), Option.some (unsafeMessage (atom) (channel))))))) ((0, Option.none)) (atoms) : (Prod (Nat) (Option (ByteArray))))).2

@[expose] public def isNameStart (value : Nat) : Bool := (if LexLeanBoot.LexiconLse.isAlphaNum (value) then true else (Nat.beq (value) (95)))

@[expose] public def isNameRest (value : Nat) : Bool := (if LexLeanBoot.LexiconLse.isAlnumNum (value) then true else (Nat.beq (value) (95)))

@[expose] public def segAt (st : (Prod (Nat) (Bool))) : Bool := (st).2

@[expose] public def leanStep (text : ByteArray) (n : Nat) (st : (Prod (Nat) (Bool))) : Option ((Prod (Nat) (Bool))) := (let position : Nat := (st).1; (if (Nat.blt (position) (n)) then (let value : Nat := LexLeanBoot.LexiconLse.byteNum (text) (position); (if (Nat.beq (value) (46)) then (if segAt (st) then Option.none else Option.some (((position + 1), true))) else (if segAt (st) then (if isNameStart (value) then Option.some (((position + 1), false)) else Option.none) else (if isNameRest (value) then Option.some (((position + 1), false)) else Option.none)))) else Option.none))

@[expose] public def isLeanName (text : ByteArray) : Bool := (let n : Nat := (LexLeanRuntime.length (text) : Nat); (let final : (Prod (Nat) (Bool)) := ((LexLeanCollections.iterateUntil ((fun (st : (Prod (Nat) (Bool))) => leanStep (text) (n) (st))) ((n + 1)) ((0, true)) : (Prod ((Prod (Nat) (Bool))) (Bool)))).1; (if (Nat.beq ((final).1) (n)) then (if segAt (final) then false else true) else false)))

@[expose] public def cmMove (st : Cm) (step : Nat) (mode : Nat) : Option (Cm) := Option.some (({ pos := ((st).pos + step), mode := mode, found := Option.none } : Cm))

@[expose] public def plainStep (text : ByteArray) (st : Cm) : Option (Cm) := (let position : Nat := (st).pos; (if LexLeanStd.Text.matchAt (text) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 34)), (_root_.UInt8.ofNat (nat_lit 34)), (_root_.UInt8.ofNat (nat_lit 34))]) (position) then cmMove (st) (3) (3) else (if LexLeanStd.Text.matchAt (text) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 39)), (_root_.UInt8.ofNat (nat_lit 39)), (_root_.UInt8.ofNat (nat_lit 39))]) (position) then cmMove (st) (3) (4) else (let value : Nat := LexLeanBoot.LexiconLse.byteNum (text) (position); (if (Nat.beq (value) (34)) then cmMove (st) (1) (1) else (if (Nat.beq (value) (39)) then cmMove (st) (1) (2) else (if (Nat.beq (value) (35)) then Option.some (({ pos := position, mode := 0, found := Option.some (position) } : Cm)) else cmMove (st) (1) (0))))))))

@[expose] public def commentStep (text : ByteArray) (n : Nat) (st : Cm) : Option (Cm) := (match (st).found with | Option.some _ => Option.none | Option.none => (let position : Nat := (st).pos; (if (Nat.blt (position) (n)) then (let mode : Nat := (st).mode; (let value : Nat := LexLeanBoot.LexiconLse.byteNum (text) (position); (if (Nat.beq (mode) (0)) then plainStep (text) (st) else (if (Nat.beq (mode) (1)) then (if (Nat.beq (value) (92)) then cmMove (st) (2) (1) else (if (if (Nat.beq (value) (34)) then true else (Nat.beq (value) (10))) then cmMove (st) (1) (0) else cmMove (st) (1) (1))) else (if (Nat.beq (mode) (2)) then (if (if (Nat.beq (value) (39)) then true else (Nat.beq (value) (10))) then cmMove (st) (1) (0) else cmMove (st) (1) (2)) else (if (Nat.beq (mode) (3)) then (if LexLeanStd.Text.matchAt (text) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 34)), (_root_.UInt8.ofNat (nat_lit 34)), (_root_.UInt8.ofNat (nat_lit 34))]) (position) then cmMove (st) (3) (0) else (if (Nat.beq (value) (92)) then cmMove (st) (2) (3) else cmMove (st) (1) (3))) else (if LexLeanStd.Text.matchAt (text) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 39)), (_root_.UInt8.ofNat (nat_lit 39)), (_root_.UInt8.ofNat (nat_lit 39))]) (position) then cmMove (st) (3) (0) else cmMove (st) (1) (4)))))))) else Option.none)))

@[expose] public def tomlCommentAt (text : ByteArray) : Option (Nat) := (let n : Nat := (LexLeanRuntime.length (text) : Nat); (((LexLeanCollections.iterateUntil ((fun (st : Cm) => commentStep (text) (n) (st))) ((n + 2)) (({ pos := 0, mode := 0, found := Option.none } : Cm)) : (Prod (Cm) (Bool)))).1).found)

@[expose] public def entryPath (localId : ByteArray) : ByteArray := cat (cat (lit ("entries/")) (LexLeanStd.Text.joinWith (LexLeanStd.Text.splitOn (localId) ((46 : UInt8))) (lit ("/")))) (lit (".toml"))

end LexLeanBoot.LexiconSchema
