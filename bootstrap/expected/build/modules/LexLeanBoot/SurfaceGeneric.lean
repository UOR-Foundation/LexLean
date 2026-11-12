module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Json
public import LexLeanStd.Numeral
public import LexLeanStd.Order
public import LexLeanStd.Text
public import LexLeanBoot.SurfaceDoc
public import LexLeanBoot.SurfaceLex
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.SurfaceGeneric

public inductive Frame where
  | FrameArr (_ : List (LexLeanStd.Json.Json))
  | FrameObj (_ : ByteArray) (_ : Nat) (_ : List ((Prod (ByteArray) (LexLeanStd.Json.Json))))

public structure Rd where
  pos : Nat
  depth : Nat
  mode : Nat
  stack : List (Frame)
  value : LexLeanStd.Json.Json
  error : Option (LexLeanBoot.SurfaceLex.Failure)

@[expose] public def lit (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def cat (left : ByteArray) (right : ByteArray) : ByteArray := (LexLeanRuntime.append (left) (right) : ByteArray)

@[expose] public def failed (offset : Nat) (message : ByteArray) : LexLeanBoot.SurfaceLex.Failure := ({ offset := offset, message := message } : LexLeanBoot.SurfaceLex.Failure)

@[expose] public def byteNo (text : ByteArray) (pos : Nat) : Nat := (match LexLeanStd.Text.byteAt (text) (pos) with | Option.some byte => (let __widen0 : UInt8 := byte; ((((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((128 : UInt8)) : UInt8)) ((128 : UInt8)) : Bool) then 128 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((64 : UInt8)) : UInt8)) ((64 : UInt8)) : Bool) then 64 else 0)) + ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((32 : UInt8)) : UInt8)) ((32 : UInt8)) : Bool) then 32 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((16 : UInt8)) : UInt8)) ((16 : UInt8)) : Bool) then 16 else 0))) + (((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((8 : UInt8)) : UInt8)) ((8 : UInt8)) : Bool) then 8 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((4 : UInt8)) : UInt8)) ((4 : UInt8)) : Bool) then 4 else 0)) + ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((2 : UInt8)) : UInt8)) ((2 : UInt8)) : Bool) then 2 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((1 : UInt8)) : UInt8)) ((1 : UInt8)) : Bool) then 1 else 0))))) | Option.none => 256)

@[expose] public def digitStep (text : ByteArray) (pos : Nat) : Option (Nat) := (let value : Nat := byteNo (text) (pos); (if (Nat.blt (47) (value)) then (if (Nat.blt (value) (58)) then Option.some ((pos + 1)) else Option.none) else Option.none))

@[expose] public def digitsEnd (text : ByteArray) (start : Nat) : Nat := ((LexLeanCollections.iterateUntil ((fun (pos : Nat) => digitStep (text) (pos))) (((LexLeanRuntime.length (text) : Nat) + 1)) (start) : (Prod (Nat) (Bool)))).1

@[expose] public def zeroStep (text : ByteArray) (pos : Nat) : Option (Nat) := (if (Nat.beq (byteNo (text) (pos)) (48)) then Option.some ((pos + 1)) else Option.none)

@[expose] public def zerosEnd (text : ByteArray) (start : Nat) : Nat := ((LexLeanCollections.iterateUntil ((fun (pos : Nat) => zeroStep (text) (pos))) (((LexLeanRuntime.length (text) : Nat) + 1)) (start) : (Prod (Nat) (Bool)))).1

@[expose] public def isInteger (text : ByteArray) : Bool := (let start : Nat := (if (Nat.beq (byteNo (text) (0)) (45)) then 1 else 0); (if (Nat.blt (start) ((LexLeanRuntime.length (text) : Nat))) then (Nat.beq (digitsEnd (text) (start)) ((LexLeanRuntime.length (text) : Nat))) else false))

@[expose] public def inI64 (negative : Bool) (magnitude : Nat) : Bool := (if negative then (if (Nat.blt (0) (magnitude)) then (Nat.ble (magnitude) (9223372036854775808)) else false) else (Nat.ble (magnitude) (9223372036854775807)))

@[expose] public def intOf (text : ByteArray) : Option (LexLeanStd.Json.Json) := (let negative : Bool := (Nat.beq (byteNo (text) (0)) (45)); (let start : Nat := (if negative then 1 else 0); (let first_digit : Nat := zerosEnd (text) (start); (let digits : ByteArray := LexLeanStd.Text.sliceOr (text) (first_digit) ((LexLeanRuntime.subtract ((LexLeanRuntime.length (text) : Nat)) (first_digit) : Nat)); (if (Nat.beq ((LexLeanRuntime.length (digits) : Nat)) (0)) then Option.some (LexLeanStd.Json.Json.JInt (false) (0)) else (if (Nat.blt (19) ((LexLeanRuntime.length (digits) : Nat))) then Option.none else (match LexLeanStd.Numeral.parseDecimal (digits) with | Option.some magnitude => (if inI64 (negative) (magnitude) then Option.some (LexLeanStd.Json.Json.JInt (negative) (magnitude)) else Option.none) | Option.none => Option.none)))))))

@[expose] public def bareStart (value : Nat) : Bool := (if (if (Nat.blt (64) (value)) then (Nat.blt (value) (91)) else false) then true else (if (if (Nat.blt (96) (value)) then (Nat.blt (value) (123)) else false) then true else (Nat.beq (value) (95))))

@[expose] public def bareByte (value : Nat) : Bool := (if bareStart (value) then true else (if (if (Nat.blt (47) (value)) then (Nat.blt (value) (58)) else false) then true else (if (Nat.beq (value) (46)) then true else (if (Nat.beq (value) (39)) then true else (if (Nat.beq (value) (47)) then true else (Nat.beq (value) (45)))))))

@[expose] public def bareStep (text : ByteArray) (pos : Nat) : Option (Nat) := (if bareByte (byteNo (text) (pos)) then Option.some ((pos + 1)) else Option.none)

@[expose] public def bareEnd (text : ByteArray) (start : Nat) : Nat := ((LexLeanCollections.iterateUntil ((fun (pos : Nat) => bareStep (text) (pos))) (((LexLeanRuntime.length (text) : Nat) + 1)) (start) : (Prod (Nat) (Bool)))).1

@[expose] public def isKeyword (text : ByteArray) : Bool := (if (LexLeanRuntime.equal (text) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 116)), (_root_.UInt8.ofNat (nat_lit 114)), (_root_.UInt8.ofNat (nat_lit 117)), (_root_.UInt8.ofNat (nat_lit 101))]) : Bool) then true else (LexLeanRuntime.equal (text) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 102)), (_root_.UInt8.ofNat (nat_lit 97)), (_root_.UInt8.ofNat (nat_lit 108)), (_root_.UInt8.ofNat (nat_lit 115)), (_root_.UInt8.ofNat (nat_lit 101))]) : Bool))

@[expose] public def isBare (text : ByteArray) : Bool := (if (Nat.beq ((LexLeanRuntime.length (text) : Nat)) (0)) then false else (if bareStart (byteNo (text) (0)) then (if (Nat.beq (bareEnd (text) (1)) ((LexLeanRuntime.length (text) : Nat))) then (!isKeyword (text)) else false) else false))

@[expose] public def tokenCode (tok : LexLeanBoot.SurfaceLex.Token) : Nat := (match tok with | LexLeanBoot.SurfaceLex.Token.LBrace => 0 | LexLeanBoot.SurfaceLex.Token.RBrace => 1 | LexLeanBoot.SurfaceLex.Token.LBrack => 2 | LexLeanBoot.SurfaceLex.Token.RBrack => 3 | LexLeanBoot.SurfaceLex.Token.LParen => 4 | LexLeanBoot.SurfaceLex.Token.RParen => 5 | LexLeanBoot.SurfaceLex.Token.Comma => 6 | LexLeanBoot.SurfaceLex.Token.Colon => 7 | LexLeanBoot.SurfaceLex.Token.ColonEq => 8 | LexLeanBoot.SurfaceLex.Token.Scope => 9 | LexLeanBoot.SurfaceLex.Token.Semi => 10 | LexLeanBoot.SurfaceLex.Token.StrToken _ => 11 | LexLeanBoot.SurfaceLex.Token.WordToken _ => 12)

@[expose] public def tokenText (tok : LexLeanBoot.SurfaceLex.Token) : ByteArray := (match tok with | LexLeanBoot.SurfaceLex.Token.LBrace => _root_.ByteArray.mk #[] | LexLeanBoot.SurfaceLex.Token.RBrace => _root_.ByteArray.mk #[] | LexLeanBoot.SurfaceLex.Token.LBrack => _root_.ByteArray.mk #[] | LexLeanBoot.SurfaceLex.Token.RBrack => _root_.ByteArray.mk #[] | LexLeanBoot.SurfaceLex.Token.LParen => _root_.ByteArray.mk #[] | LexLeanBoot.SurfaceLex.Token.RParen => _root_.ByteArray.mk #[] | LexLeanBoot.SurfaceLex.Token.Comma => _root_.ByteArray.mk #[] | LexLeanBoot.SurfaceLex.Token.Colon => _root_.ByteArray.mk #[] | LexLeanBoot.SurfaceLex.Token.ColonEq => _root_.ByteArray.mk #[] | LexLeanBoot.SurfaceLex.Token.Scope => _root_.ByteArray.mk #[] | LexLeanBoot.SurfaceLex.Token.Semi => _root_.ByteArray.mk #[] | LexLeanBoot.SurfaceLex.Token.StrToken payload => payload | LexLeanBoot.SurfaceLex.Token.WordToken payload => payload)

@[expose] public def describe (tok : LexLeanBoot.SurfaceLex.Token) : ByteArray := (match tok with | LexLeanBoot.SurfaceLex.Token.LBrace => lit ("`{`") | LexLeanBoot.SurfaceLex.Token.RBrace => lit ("`}`") | LexLeanBoot.SurfaceLex.Token.LBrack => lit ("`[`") | LexLeanBoot.SurfaceLex.Token.RBrack => lit ("`]`") | LexLeanBoot.SurfaceLex.Token.LParen => lit ("`(`") | LexLeanBoot.SurfaceLex.Token.RParen => lit ("`)`") | LexLeanBoot.SurfaceLex.Token.Comma => lit ("`,`") | LexLeanBoot.SurfaceLex.Token.Colon => lit ("`:`") | LexLeanBoot.SurfaceLex.Token.ColonEq => lit ("`:=`") | LexLeanBoot.SurfaceLex.Token.Scope => lit ("`::`") | LexLeanBoot.SurfaceLex.Token.Semi => lit ("`;`") | LexLeanBoot.SurfaceLex.Token.StrToken payload => cat (lit ("the string ")) (LexLeanBoot.SurfaceLex.quoteText (payload)) | LexLeanBoot.SurfaceLex.Token.WordToken payload => cat (cat (lit ("`")) (payload)) (lit ("`")))

@[expose] public def tokenAt (toks : List (LexLeanBoot.SurfaceLex.Spanned)) (pos : Nat) : Option (LexLeanBoot.SurfaceLex.Spanned) := (LexLeanRuntime.index (toks) (pos) : Option (LexLeanBoot.SurfaceLex.Spanned))

@[expose] public def codeAt (toks : List (LexLeanBoot.SurfaceLex.Spanned)) (pos : Nat) : Nat := (match tokenAt (toks) (pos) with | Option.some found => tokenCode ((found).token) | Option.none => 99)

@[expose] public def textAt (toks : List (LexLeanBoot.SurfaceLex.Spanned)) (pos : Nat) : ByteArray := (match tokenAt (toks) (pos) with | Option.some found => tokenText ((found).token) | Option.none => _root_.ByteArray.mk #[])

@[expose] public def offsetAt (toks : List (LexLeanBoot.SurfaceLex.Spanned)) (size : Nat) (pos : Nat) : Nat := (match tokenAt (toks) (pos) with | Option.some found => (found).start | Option.none => size)

@[expose] public def describeAt (toks : List (LexLeanBoot.SurfaceLex.Spanned)) (pos : Nat) : ByteArray := (match tokenAt (toks) (pos) with | Option.some found => describe ((found).token) | Option.none => lit ("the end of the text"))

@[expose] public def itemChain (items : List (LexLeanStd.Json.Json)) : LexLeanStd.Json.Json := (LexLeanCollections.listFold ((fun (chain : LexLeanStd.Json.Json) (item : LexLeanStd.Json.Json) => LexLeanStd.Json.Json.JItem (item) (chain))) (LexLeanStd.Json.Json.JNil) (items) : LexLeanStd.Json.Json)

@[expose] public def memberChain (sorted : List ((Prod (ByteArray) (LexLeanStd.Json.Json)))) : LexLeanStd.Json.Json := (LexLeanCollections.listFold ((fun (chain : LexLeanStd.Json.Json) (entry : (Prod (ByteArray) (LexLeanStd.Json.Json))) => LexLeanStd.Json.Json.JMember ((entry).1) ((entry).2) (chain))) (LexLeanStd.Json.Json.JNil) (LexLeanStd.Order.reverseList ((Prod (ByteArray) (LexLeanStd.Json.Json))) (sorted)) : LexLeanStd.Json.Json)

@[expose] public def hasKey (members : List ((Prod (ByteArray) (LexLeanStd.Json.Json)))) (key : ByteArray) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (entry : (Prod (ByteArray) (LexLeanStd.Json.Json))) => (if found then true else (LexLeanRuntime.equal ((entry).1) (key) : Bool)))) (false) (members) : Bool)

@[expose] public def refuse (st : Rd) (offset : Nat) (message : ByteArray) : Rd := ({ pos := (st).pos, depth := (st).depth, mode := 9, stack := (st).stack, value := (st).value, error := Option.some (failed (offset) (message)) } : Rd)

@[expose] public def closeTop (st : Rd) : Rd := (match (st).stack with | List.nil => refuse (st) (0) (lit ("nothing to close")) | List.cons top rest => (match top with | Frame.FrameArr items => ({ pos := ((st).pos + 1), depth := (LexLeanRuntime.subtract ((st).depth) (1) : Nat), mode := 3, stack := rest, value := LexLeanStd.Json.Json.JArr (itemChain (items)), error := Option.none } : Rd) | Frame.FrameObj _ _ members => ({ pos := ((st).pos + 1), depth := (LexLeanRuntime.subtract ((st).depth) (1) : Nat), mode := 3, stack := rest, value := LexLeanStd.Json.Json.JObj (memberChain (LexLeanStd.Order.sortByKey (LexLeanStd.Json.Json) (members))), error := Option.none } : Rd)))

@[expose] public def atComma (toks : List (LexLeanBoot.SurfaceLex.Spanned)) (st : Rd) (stack : List (Frame)) (head : Nat) (tail : Nat) : Rd := (if (Nat.beq (codeAt (toks) ((st).pos)) (6)) then ({ pos := ((st).pos + 1), depth := (st).depth, mode := head, stack := stack, value := LexLeanStd.Json.Json.JNil, error := Option.none } : Rd) else ({ pos := (st).pos, depth := (st).depth, mode := tail, stack := stack, value := LexLeanStd.Json.Json.JNil, error := Option.none } : Rd))

@[expose] public def completeValue (toks : List (LexLeanBoot.SurfaceLex.Spanned)) (st : Rd) : Rd := (match (st).stack with | List.nil => ({ pos := (st).pos, depth := (st).depth, mode := 9, stack := ([] : List (Frame)), value := (st).value, error := Option.none } : Rd) | List.cons top rest => (match top with | Frame.FrameArr items => atComma (toks) (st) ((Frame.FrameArr (((st).value :: items)) :: rest)) (1) (4) | Frame.FrameObj key offset members => (if hasKey (members) (key) then refuse (st) (offset) (cat (cat (lit ("the member `")) (key)) (lit ("` is written twice"))) else atComma (toks) (st) ((Frame.FrameObj (_root_.ByteArray.mk #[]) (0) (((key, (st).value) :: members)) :: rest)) (2) (5))))

@[expose] public def valueDone (st : Rd) (pos : Nat) (value : LexLeanStd.Json.Json) : Rd := ({ pos := pos, depth := (st).depth, mode := 3, stack := (st).stack, value := value, error := Option.none } : Rd)

@[expose] public def valuePush (st : Rd) (pos : Nat) (frame : Frame) (mode : Nat) : Rd := ({ pos := pos, depth := ((st).depth + 1), mode := mode, stack := (frame :: (st).stack), value := LexLeanStd.Json.Json.JNil, error := Option.none } : Rd)

@[expose] public def wordValue (toks : List (LexLeanBoot.SurfaceLex.Spanned)) (_size : Nat) (st : Rd) (start : Nat) : Rd := (let pos : Nat := (st).pos; (let word : ByteArray := textAt (toks) (pos); (if (LexLeanRuntime.equal (word) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 116)), (_root_.UInt8.ofNat (nat_lit 114)), (_root_.UInt8.ofNat (nat_lit 117)), (_root_.UInt8.ofNat (nat_lit 101))]) : Bool) then valueDone (st) ((pos + 1)) (LexLeanStd.Json.Json.JBool (true)) else (if (LexLeanRuntime.equal (word) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 102)), (_root_.UInt8.ofNat (nat_lit 97)), (_root_.UInt8.ofNat (nat_lit 108)), (_root_.UInt8.ofNat (nat_lit 115)), (_root_.UInt8.ofNat (nat_lit 101))]) : Bool) then valueDone (st) ((pos + 1)) (LexLeanStd.Json.Json.JBool (false)) else (if isInteger (word) then (match intOf (word) with | Option.some number => valueDone (st) ((pos + 1)) (number) | Option.none => refuse (st) (start) (cat (cat (lit ("`")) (word)) (lit ("` does not fit a 64-bit integer")))) else (if isBare (word) then (if (Nat.beq (codeAt (toks) ((pos + 1))) (0)) then valuePush (st) ((pos + 2)) (Frame.FrameObj (_root_.ByteArray.mk #[]) (0) (((lit ("kind"), LexLeanStd.Json.Json.JStr (word)) :: ([] : List ((Prod (ByteArray) (LexLeanStd.Json.Json))))))) (2) else valueDone (st) ((pos + 1)) (LexLeanStd.Json.Json.JStr (word))) else refuse (st) (start) (cat (cat (lit ("`")) (word)) (lit ("` is not a name; quote it to write a string")))))))))

@[expose] public def valueStart (toks : List (LexLeanBoot.SurfaceLex.Spanned)) (size : Nat) (st : Rd) : Rd := (let pos : Nat := (st).pos; (if (Nat.blt (256) ((st).depth)) then refuse (st) (offsetAt (toks) (size) (pos)) (lit ("nested more deeply than the notation reads")) else (let code : Nat := codeAt (toks) (pos); (if (Nat.beq (code) (99)) then refuse (st) (size) (lit ("expected a value, found the end of the text")) else (if (Nat.beq (code) (11)) then valueDone (st) ((pos + 1)) (LexLeanStd.Json.Json.JStr (textAt (toks) (pos))) else (if (Nat.beq (code) (2)) then valuePush (st) ((pos + 1)) (Frame.FrameArr (([] : List (LexLeanStd.Json.Json)))) (1) else (if (Nat.beq (code) (0)) then valuePush (st) ((pos + 1)) (Frame.FrameObj (_root_.ByteArray.mk #[]) (0) (([] : List ((Prod (ByteArray) (LexLeanStd.Json.Json)))))) (2) else (if (Nat.beq (code) (12)) then wordValue (toks) (size) (st) (offsetAt (toks) (size) (pos)) else refuse (st) (offsetAt (toks) (size) (pos)) (cat (lit ("expected a value, found ")) (describeAt (toks) (pos)))))))))))

@[expose] public def keyAt (toks : List (LexLeanBoot.SurfaceLex.Spanned)) (size : Nat) (st : Rd) (members : List ((Prod (ByteArray) (LexLeanStd.Json.Json)))) (stack : List (Frame)) : Rd := (let pos : Nat := (st).pos; (let code : Nat := codeAt (toks) (pos); (let word : ByteArray := textAt (toks) (pos); (if (if (Nat.beq (code) (11)) then true else (if (Nat.beq (code) (12)) then isBare (word) else false)) then (if (Nat.beq (codeAt (toks) ((pos + 1))) (7)) then ({ pos := (pos + 2), depth := (st).depth, mode := 0, stack := (Frame.FrameObj (word) (offsetAt (toks) (size) (pos)) (members) :: stack), value := LexLeanStd.Json.Json.JNil, error := Option.none } : Rd) else refuse (st) (offsetAt (toks) (size) ((pos + 1))) (lit ("expected `:` after a member name"))) else refuse (st) (offsetAt (toks) (size) (pos)) (lit ("expected a member name"))))))

@[expose] public def objectHead (toks : List (LexLeanBoot.SurfaceLex.Spanned)) (size : Nat) (st : Rd) : Rd := (match (st).stack with | List.nil => refuse (st) (0) (lit ("no object is open")) | List.cons top rest => (match top with | Frame.FrameArr _ => refuse (st) (0) (lit ("no object is open")) | Frame.FrameObj _ _ members => (if (Nat.beq (codeAt (toks) ((st).pos)) (1)) then closeTop (st) else keyAt (toks) (size) (st) (members) (rest))))

@[expose] public def arrayHead (toks : List (LexLeanBoot.SurfaceLex.Spanned)) (st : Rd) : Rd := (if (Nat.beq (codeAt (toks) ((st).pos)) (3)) then closeTop (st) else ({ pos := (st).pos, depth := (st).depth, mode := 0, stack := (st).stack, value := LexLeanStd.Json.Json.JNil, error := Option.none } : Rd))

@[expose] public def closing (toks : List (LexLeanBoot.SurfaceLex.Spanned)) (size : Nat) (st : Rd) (code : Nat) (message : ByteArray) : Rd := (if (Nat.beq (codeAt (toks) ((st).pos)) (code)) then closeTop (st) else refuse (st) (offsetAt (toks) (size) ((st).pos)) (message))

@[expose] public def rdMode (toks : List (LexLeanBoot.SurfaceLex.Spanned)) (size : Nat) (st : Rd) : Rd := (let mode : Nat := (st).mode; (if (Nat.beq (mode) (0)) then valueStart (toks) (size) (st) else (if (Nat.beq (mode) (1)) then arrayHead (toks) (st) else (if (Nat.beq (mode) (2)) then objectHead (toks) (size) (st) else (if (Nat.beq (mode) (3)) then completeValue (toks) (st) else (if (Nat.beq (mode) (4)) then closing (toks) (size) (st) (3) (lit ("expected `,` or `]`")) else closing (toks) (size) (st) (1) (lit ("expected `,` or `}`"))))))))

@[expose] public def rdStep (toks : List (LexLeanBoot.SurfaceLex.Spanned)) (size : Nat) (st : Rd) : Option (Rd) := (match (st).error with | Option.some _ => Option.none | Option.none => (if (Nat.beq ((st).mode) (9)) then Option.none else Option.some (rdMode (toks) (size) (st))))

@[expose] public def readFrom (toks : List (LexLeanBoot.SurfaceLex.Spanned)) (size : Nat) (pos : Nat) (depth : Nat) : Rd := ((LexLeanCollections.iterateUntil ((fun (st : Rd) => rdStep (toks) (size) (st))) (((LexLeanRuntime.multiply ((LexLeanRuntime.length (toks) : Nat)) (4) : Nat) + 8)) (({ pos := pos, depth := depth, mode := 0, stack := ([] : List (Frame)), value := LexLeanStd.Json.Json.JNil, error := Option.none } : Rd)) : (Prod (Rd) (Bool)))).1

@[expose] public def parseTokens (toks : List (LexLeanBoot.SurfaceLex.Spanned)) (size : Nat) : Except (LexLeanBoot.SurfaceLex.Failure) (LexLeanStd.Json.Json) := (let final : Rd := readFrom (toks) (size) (0) (0); (match (final).error with | Option.some failure => Except.error (failure) | Option.none => (if (Nat.beq ((final).mode) (9)) then (if (Nat.blt ((final).pos) ((LexLeanRuntime.length (toks) : Nat))) then Except.error (failed (offsetAt (toks) (size) ((final).pos)) (lit ("unexpected text after the value"))) else Except.ok ((final).value)) else Except.error (failed (size) (lit ("the reader ran out of steps"))))))

@[expose] public def parseGeneric (text : ByteArray) : Except (LexLeanBoot.SurfaceLex.Failure) (LexLeanStd.Json.Json) := (match LexLeanBoot.SurfaceLex.lexText (text) with | Except.ok toks => parseTokens (toks) ((LexLeanRuntime.length (text) : Nat)) | Except.error failure => Except.error (failure))

@[expose] public def spell (text : ByteArray) : ByteArray := (if isBare (text) then text else LexLeanBoot.SurfaceLex.quoteText (text))

@[expose] public def rank (key : ByteArray) : Nat := (if (LexLeanRuntime.equal (key) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 110)), (_root_.UInt8.ofNat (nat_lit 97)), (_root_.UInt8.ofNat (nat_lit 109)), (_root_.UInt8.ofNat (nat_lit 101))]) : Bool) then 0 else (if (LexLeanRuntime.equal (key) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 116)), (_root_.UInt8.ofNat (nat_lit 121)), (_root_.UInt8.ofNat (nat_lit 112)), (_root_.UInt8.ofNat (nat_lit 101)), (_root_.UInt8.ofNat (nat_lit 95)), (_root_.UInt8.ofNat (nat_lit 112)), (_root_.UInt8.ofNat (nat_lit 97)), (_root_.UInt8.ofNat (nat_lit 114)), (_root_.UInt8.ofNat (nat_lit 97)), (_root_.UInt8.ofNat (nat_lit 109)), (_root_.UInt8.ofNat (nat_lit 101)), (_root_.UInt8.ofNat (nat_lit 116)), (_root_.UInt8.ofNat (nat_lit 101)), (_root_.UInt8.ofNat (nat_lit 114)), (_root_.UInt8.ofNat (nat_lit 115))]) : Bool) then 1 else (if (LexLeanRuntime.equal (key) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 112)), (_root_.UInt8.ofNat (nat_lit 97)), (_root_.UInt8.ofNat (nat_lit 114)), (_root_.UInt8.ofNat (nat_lit 97)), (_root_.UInt8.ofNat (nat_lit 109)), (_root_.UInt8.ofNat (nat_lit 101)), (_root_.UInt8.ofNat (nat_lit 116)), (_root_.UInt8.ofNat (nat_lit 101)), (_root_.UInt8.ofNat (nat_lit 114)), (_root_.UInt8.ofNat (nat_lit 115))]) : Bool) then 2 else (if (LexLeanRuntime.equal (key) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 116)), (_root_.UInt8.ofNat (nat_lit 121)), (_root_.UInt8.ofNat (nat_lit 112)), (_root_.UInt8.ofNat (nat_lit 101))]) : Bool) then 3 else (if (LexLeanRuntime.equal (key) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 114)), (_root_.UInt8.ofNat (nat_lit 101)), (_root_.UInt8.ofNat (nat_lit 115)), (_root_.UInt8.ofNat (nat_lit 117)), (_root_.UInt8.ofNat (nat_lit 108)), (_root_.UInt8.ofNat (nat_lit 116))]) : Bool) then 4 else (if (LexLeanRuntime.equal (key) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 115)), (_root_.UInt8.ofNat (nat_lit 116)), (_root_.UInt8.ofNat (nat_lit 97)), (_root_.UInt8.ofNat (nat_lit 116)), (_root_.UInt8.ofNat (nat_lit 101)), (_root_.UInt8.ofNat (nat_lit 109)), (_root_.UInt8.ofNat (nat_lit 101)), (_root_.UInt8.ofNat (nat_lit 110)), (_root_.UInt8.ofNat (nat_lit 116))]) : Bool) then 5 else (if (LexLeanRuntime.equal (key) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 102)), (_root_.UInt8.ofNat (nat_lit 105)), (_root_.UInt8.ofNat (nat_lit 101)), (_root_.UInt8.ofNat (nat_lit 108)), (_root_.UInt8.ofNat (nat_lit 100)), (_root_.UInt8.ofNat (nat_lit 115))]) : Bool) then 6 else (if (LexLeanRuntime.equal (key) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 99)), (_root_.UInt8.ofNat (nat_lit 111)), (_root_.UInt8.ofNat (nat_lit 110)), (_root_.UInt8.ofNat (nat_lit 115)), (_root_.UInt8.ofNat (nat_lit 116)), (_root_.UInt8.ofNat (nat_lit 114)), (_root_.UInt8.ofNat (nat_lit 117)), (_root_.UInt8.ofNat (nat_lit 99)), (_root_.UInt8.ofNat (nat_lit 116)), (_root_.UInt8.ofNat (nat_lit 111)), (_root_.UInt8.ofNat (nat_lit 114)), (_root_.UInt8.ofNat (nat_lit 115))]) : Bool) then 7 else (if (LexLeanRuntime.equal (key) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 97)), (_root_.UInt8.ofNat (nat_lit 114)), (_root_.UInt8.ofNat (nat_lit 103)), (_root_.UInt8.ofNat (nat_lit 117)), (_root_.UInt8.ofNat (nat_lit 109)), (_root_.UInt8.ofNat (nat_lit 101)), (_root_.UInt8.ofNat (nat_lit 110)), (_root_.UInt8.ofNat (nat_lit 116)), (_root_.UInt8.ofNat (nat_lit 115))]) : Bool) then 8 else (if (LexLeanRuntime.equal (key) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 98)), (_root_.UInt8.ofNat (nat_lit 111)), (_root_.UInt8.ofNat (nat_lit 100)), (_root_.UInt8.ofNat (nat_lit 121))]) : Bool) then 9 else (if (LexLeanRuntime.equal (key) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 112)), (_root_.UInt8.ofNat (nat_lit 114)), (_root_.UInt8.ofNat (nat_lit 111)), (_root_.UInt8.ofNat (nat_lit 111)), (_root_.UInt8.ofNat (nat_lit 102))]) : Bool) then 10 else (if (LexLeanRuntime.equal (key) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 97)), (_root_.UInt8.ofNat (nat_lit 120)), (_root_.UInt8.ofNat (nat_lit 105)), (_root_.UInt8.ofNat (nat_lit 111)), (_root_.UInt8.ofNat (nat_lit 109)), (_root_.UInt8.ofNat (nat_lit 115))]) : Bool) then 11 else 12))))))))))))

@[expose] public def rankKey (key : ByteArray) : ByteArray := cat (LexLeanBoot.SurfaceLex.byteOf (rank (key))) (key)

@[expose] public def fieldDoc (key : ByteArray) (shown : LexLeanBoot.SurfaceDoc.Doc) : LexLeanBoot.SurfaceDoc.Doc := LexLeanBoot.SurfaceDoc.Doc.DSeq (LexLeanBoot.SurfaceDoc.Doc.DText (cat (spell (key)) (lit (": ")))) (LexLeanBoot.SurfaceDoc.Doc.DSeq (shown) (LexLeanBoot.SurfaceDoc.Doc.DNil))

@[expose] public def joinStep (state : (Prod (Bool) (LexLeanBoot.SurfaceDoc.Doc))) (item : LexLeanBoot.SurfaceDoc.Doc) : (Prod (Bool) (LexLeanBoot.SurfaceDoc.Doc)) := (if (state).1 then (false, LexLeanBoot.SurfaceDoc.Doc.DSeq (item) (LexLeanBoot.SurfaceDoc.Doc.DNil)) else (false, LexLeanBoot.SurfaceDoc.Doc.DSeq (item) (LexLeanBoot.SurfaceDoc.Doc.DSeq (LexLeanBoot.SurfaceDoc.Doc.DText (lit (","))) (LexLeanBoot.SurfaceDoc.Doc.DSeq (LexLeanBoot.SurfaceDoc.Doc.DLine) ((state).2)))))

@[expose] public def delimitedRev («open» : ByteArray) (close : ByteArray) (itemsRev : List (LexLeanBoot.SurfaceDoc.Doc)) : LexLeanBoot.SurfaceDoc.Doc := (match itemsRev with | List.nil => LexLeanBoot.SurfaceDoc.Doc.DText (cat («open») (close)) | List.cons _ _ => (let joined : (Prod (Bool) (LexLeanBoot.SurfaceDoc.Doc)) := (LexLeanCollections.listFold ((fun (state : (Prod (Bool) (LexLeanBoot.SurfaceDoc.Doc))) (item : LexLeanBoot.SurfaceDoc.Doc) => joinStep (state) (item))) ((true, LexLeanBoot.SurfaceDoc.Doc.DNil)) (itemsRev) : (Prod (Bool) (LexLeanBoot.SurfaceDoc.Doc))); LexLeanBoot.SurfaceDoc.Doc.DGroup (LexLeanBoot.SurfaceDoc.Doc.DSeq (LexLeanBoot.SurfaceDoc.Doc.DText («open»)) (LexLeanBoot.SurfaceDoc.Doc.DSeq (LexLeanBoot.SurfaceDoc.Doc.DNest (2) (LexLeanBoot.SurfaceDoc.Doc.DSeq (LexLeanBoot.SurfaceDoc.Doc.DSoft) (LexLeanBoot.SurfaceDoc.Doc.DSeq ((joined).2) (LexLeanBoot.SurfaceDoc.Doc.DNil)))) (LexLeanBoot.SurfaceDoc.Doc.DSeq (LexLeanBoot.SurfaceDoc.Doc.DSoft) (LexLeanBoot.SurfaceDoc.Doc.DSeq (LexLeanBoot.SurfaceDoc.Doc.DText (close)) (LexLeanBoot.SurfaceDoc.Doc.DNil)))))))

@[expose] public def kindTag : (members : LexLeanStd.Json.Json) -> Option (ByteArray)
  | LexLeanStd.Json.Json.JBool _ => Option.none
  | LexLeanStd.Json.Json.JInt _ _ => Option.none
  | LexLeanStd.Json.Json.JStr _ => Option.none
  | LexLeanStd.Json.Json.JArr _ => Option.none
  | LexLeanStd.Json.Json.JObj _ => Option.none
  | LexLeanStd.Json.Json.JNil => Option.none
  | LexLeanStd.Json.Json.JItem _ _ => Option.none
  | LexLeanStd.Json.Json.JMember key inner rest => (if (LexLeanRuntime.equal (key) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 107)), (_root_.UInt8.ofNat (nat_lit 105)), (_root_.UInt8.ofNat (nat_lit 110)), (_root_.UInt8.ofNat (nat_lit 100))]) : Bool) then (match inner with | LexLeanStd.Json.Json.JStr tag => (if isBare (tag) then Option.some (tag) else Option.none) | LexLeanStd.Json.Json.JBool _ => Option.none | LexLeanStd.Json.Json.JInt _ _ => Option.none | LexLeanStd.Json.Json.JArr _ => Option.none | LexLeanStd.Json.Json.JObj _ => Option.none | LexLeanStd.Json.Json.JNil => Option.none | LexLeanStd.Json.Json.JItem _ _ => Option.none | LexLeanStd.Json.Json.JMember _ _ _ => Option.none) else kindTag (rest))

@[expose] public def objectDoc (tag : Option (ByteArray)) (pairs : List ((Prod (ByteArray) (LexLeanBoot.SurfaceDoc.Doc)))) : LexLeanBoot.SurfaceDoc.Doc := (let entries : List ((Prod (ByteArray) (LexLeanBoot.SurfaceDoc.Doc))) := (LexLeanCollections.listFold ((fun (kept : List ((Prod (ByteArray) (LexLeanBoot.SurfaceDoc.Doc)))) (pair : (Prod (ByteArray) (LexLeanBoot.SurfaceDoc.Doc))) => (match tag with | Option.some _ => (if (LexLeanRuntime.equal ((pair).1) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 107)), (_root_.UInt8.ofNat (nat_lit 105)), (_root_.UInt8.ofNat (nat_lit 110)), (_root_.UInt8.ofNat (nat_lit 100))]) : Bool) then kept else ((rankKey ((pair).1), fieldDoc ((pair).1) ((pair).2)) :: kept)) | Option.none => ((rankKey ((pair).1), fieldDoc ((pair).1) ((pair).2)) :: kept)))) (([] : List ((Prod (ByteArray) (LexLeanBoot.SurfaceDoc.Doc))))) (pairs) : List ((Prod (ByteArray) (LexLeanBoot.SurfaceDoc.Doc)))); delimitedRev ((match tag with | Option.some name => cat (name) (lit ("{")) | Option.none => lit ("{"))) (lit ("}")) ((LexLeanCollections.listFold ((fun (docs : List (LexLeanBoot.SurfaceDoc.Doc)) (entry : (Prod (ByteArray) (LexLeanBoot.SurfaceDoc.Doc))) => ((entry).2 :: docs))) (([] : List (LexLeanBoot.SurfaceDoc.Doc))) (LexLeanStd.Order.sortByKey (LexLeanBoot.SurfaceDoc.Doc) (entries)) : List (LexLeanBoot.SurfaceDoc.Doc))))

mutual
@[expose] public def docValue : (value : LexLeanStd.Json.Json) -> LexLeanBoot.SurfaceDoc.Doc
  | LexLeanStd.Json.Json.JBool flag => LexLeanBoot.SurfaceDoc.Doc.DText ((if flag then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 116)), (_root_.UInt8.ofNat (nat_lit 114)), (_root_.UInt8.ofNat (nat_lit 117)), (_root_.UInt8.ofNat (nat_lit 101))] else _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 102)), (_root_.UInt8.ofNat (nat_lit 97)), (_root_.UInt8.ofNat (nat_lit 108)), (_root_.UInt8.ofNat (nat_lit 115)), (_root_.UInt8.ofNat (nat_lit 101))]))
  | LexLeanStd.Json.Json.JInt negative magnitude => LexLeanBoot.SurfaceDoc.Doc.DText (LexLeanStd.Json.integerText (negative) (magnitude))
  | LexLeanStd.Json.Json.JStr text => LexLeanBoot.SurfaceDoc.Doc.DText (spell (text))
  | LexLeanStd.Json.Json.JArr items => delimitedRev (lit ("[")) (lit ("]")) (LexLeanStd.Order.reverseList (LexLeanBoot.SurfaceDoc.Doc) (docItems (items)))
  | LexLeanStd.Json.Json.JObj members => objectDoc (kindTag (members)) (docMembers (members))
  | LexLeanStd.Json.Json.JNil => LexLeanBoot.SurfaceDoc.Doc.DNil
  | LexLeanStd.Json.Json.JItem _ _ => LexLeanBoot.SurfaceDoc.Doc.DNil
  | LexLeanStd.Json.Json.JMember _ _ _ => LexLeanBoot.SurfaceDoc.Doc.DNil
termination_by structural value => value

@[expose] public def docItems : (items : LexLeanStd.Json.Json) -> List (LexLeanBoot.SurfaceDoc.Doc)
  | LexLeanStd.Json.Json.JBool _ => ([] : List (LexLeanBoot.SurfaceDoc.Doc))
  | LexLeanStd.Json.Json.JInt _ _ => ([] : List (LexLeanBoot.SurfaceDoc.Doc))
  | LexLeanStd.Json.Json.JStr _ => ([] : List (LexLeanBoot.SurfaceDoc.Doc))
  | LexLeanStd.Json.Json.JArr _ => ([] : List (LexLeanBoot.SurfaceDoc.Doc))
  | LexLeanStd.Json.Json.JObj _ => ([] : List (LexLeanBoot.SurfaceDoc.Doc))
  | LexLeanStd.Json.Json.JNil => ([] : List (LexLeanBoot.SurfaceDoc.Doc))
  | LexLeanStd.Json.Json.JItem element rest => (docValue (element) :: docItems (rest))
  | LexLeanStd.Json.Json.JMember _ _ _ => ([] : List (LexLeanBoot.SurfaceDoc.Doc))
termination_by structural items => items

@[expose] public def docMembers : (members : LexLeanStd.Json.Json) -> List ((Prod (ByteArray) (LexLeanBoot.SurfaceDoc.Doc)))
  | LexLeanStd.Json.Json.JBool _ => ([] : List ((Prod (ByteArray) (LexLeanBoot.SurfaceDoc.Doc))))
  | LexLeanStd.Json.Json.JInt _ _ => ([] : List ((Prod (ByteArray) (LexLeanBoot.SurfaceDoc.Doc))))
  | LexLeanStd.Json.Json.JStr _ => ([] : List ((Prod (ByteArray) (LexLeanBoot.SurfaceDoc.Doc))))
  | LexLeanStd.Json.Json.JArr _ => ([] : List ((Prod (ByteArray) (LexLeanBoot.SurfaceDoc.Doc))))
  | LexLeanStd.Json.Json.JObj _ => ([] : List ((Prod (ByteArray) (LexLeanBoot.SurfaceDoc.Doc))))
  | LexLeanStd.Json.Json.JNil => ([] : List ((Prod (ByteArray) (LexLeanBoot.SurfaceDoc.Doc))))
  | LexLeanStd.Json.Json.JItem _ _ => ([] : List ((Prod (ByteArray) (LexLeanBoot.SurfaceDoc.Doc))))
  | LexLeanStd.Json.Json.JMember key inner rest => ((key, docValue (inner)) :: docMembers (rest))
termination_by structural members => members
end

@[expose] public def docNodes : (node : LexLeanBoot.SurfaceDoc.Doc) -> Nat
  | LexLeanBoot.SurfaceDoc.Doc.DNil => 1
  | LexLeanBoot.SurfaceDoc.Doc.DText _ => 1
  | LexLeanBoot.SurfaceDoc.Doc.DSeq head tail => (let left : Nat := docNodes (head); (let right : Nat := docNodes (tail); ((1 + left) + right)))
  | LexLeanBoot.SurfaceDoc.Doc.DNest _ inner => (let inside : Nat := docNodes (inner); (1 + inside))
  | LexLeanBoot.SurfaceDoc.Doc.DLine => 1
  | LexLeanBoot.SurfaceDoc.Doc.DSoft => 1
  | LexLeanBoot.SurfaceDoc.Doc.DGroup inner => (let inside : Nat := docNodes (inner); (1 + inside))

@[expose] public def printGeneric (value : LexLeanStd.Json.Json) : ByteArray := (let node : LexLeanBoot.SurfaceDoc.Doc := docValue (value); LexLeanBoot.SurfaceDoc.renderDoc (node) (100) (docNodes (node)))

end LexLeanBoot.SurfaceGeneric
