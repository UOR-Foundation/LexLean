module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Text
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.PrimitiveScan

public inductive AtomClass where
  | Control
  | Word
  | Numeral
  | Delimiter
  | AsciiSymbol
  | UnicodeSymbol
  | Whitespace

public structure Atom where
  kind : AtomClass
  byteStart : Nat
  byteEnd : Nat
  lineStart : Nat
  columnStart : Nat
  lineEnd : Nat
  columnEnd : Nat
  text : ByteArray

public structure Place where
  path : ByteArray
  byteStart : Nat
  byteEnd : Nat
  lineStart : Nat
  columnStart : Nat
  lineEnd : Nat
  columnEnd : Nat

public structure Rejection where
  code : ByteArray
  message : ByteArray
  place : Option (Place)
  help : Option (ByteArray)

public structure Cursor where
  position : Nat
  line : Nat
  column : Nat

public structure Lexed where
  kind : AtomClass
  after : Cursor
  error : Option (Rejection)

public structure ScanState where
  cursor : Cursor
  atoms : List (Atom)
  count : Nat
  error : Option (Rejection)

public structure Context where
  awaitingOpen : Bool
  depth : Nat
  quoted : Bool
  escaped : Bool
  environment : Nat
  body : Nat

public structure Payload where
  index : Nat
  depth : Nat
  quoted : Bool
  escaped : Bool

public structure CheckState where
  context : Context
  previous : Option (Atom)
  error : Option (Rejection)

@[expose] public def lit (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def cat (left : ByteArray) (right : ByteArray) : ByteArray := (LexLeanRuntime.append (left) (right) : ByteArray)

@[expose] public def ticks (name : ByteArray) : ByteArray := cat (cat (lit ("`")) (name)) (lit ("`"))

@[expose] public def listLength (Item : Type) (items : List (Item)) : Nat := (LexLeanCollections.listFold ((fun (count : Nat) (_item : Item) => (count + 1))) (0) (items) : Nat)

@[expose] public def reversed (Item : Type) (items : List (Item)) : List (Item) := (LexLeanCollections.listFold ((fun (turned : List (Item)) (item : Item) => (item :: turned))) (([] : List (Item))) (items) : List (Item))

@[expose] public def inRange (byte : UInt8) (low : UInt8) (high : UInt8) : Bool := (match (LexLeanRuntime.checkedSubtract (byte) (low) : Option (UInt8)) with | Option.some _ => (match (LexLeanRuntime.checkedSubtract (high) (byte) : Option (UInt8)) with | Option.some _ => true | Option.none => false) | Option.none => false)

@[expose] public def byteOrZero (subject : ByteArray) (position : Nat) : UInt8 := (match LexLeanStd.Text.byteAt (subject) (position) with | Option.some byte => byte | Option.none => (0 : UInt8))

@[expose] public def byteIs (subject : ByteArray) (position : Nat) (value : UInt8) : Bool := (match LexLeanStd.Text.byteAt (subject) (position) with | Option.some byte => (LexLeanRuntime.equal (byte) (value) : Bool) | Option.none => false)

@[expose] public def isAsciiDigit (byte : UInt8) : Bool := inRange (byte) ((48 : UInt8)) ((57 : UInt8))

@[expose] public def isAsciiAlpha (byte : UInt8) : Bool := (let lower : Bool := inRange (byte) ((97 : UInt8)) ((122 : UInt8)); (let upper : Bool := inRange (byte) ((65 : UInt8)) ((90 : UInt8)); (if lower then true else upper)))

@[expose] public def scalarWidth (subject : ByteArray) (position : Nat) : Nat := (let b0 : UInt8 := byteOrZero (subject) (position); (let ascii : Bool := inRange (b0) ((0 : UInt8)) ((127 : UInt8)); (if ascii then 1 else (let two : Bool := inRange (b0) ((128 : UInt8)) ((223 : UInt8)); (if two then 2 else (let three : Bool := inRange (b0) ((224 : UInt8)) ((239 : UInt8)); (if three then 3 else 4)))))))

@[expose] public def hexDigit (nibble : UInt8) : ByteArray := (let table : ByteArray := (LexLeanRuntime.utf8Encode ("0123456789ABCDEF") : ByteArray); (let idx : Nat := (if (LexLeanRuntime.equal (nibble) ((0 : UInt8)) : Bool) then 0 else (if (LexLeanRuntime.equal (nibble) ((1 : UInt8)) : Bool) then 1 else (if (LexLeanRuntime.equal (nibble) ((2 : UInt8)) : Bool) then 2 else (if (LexLeanRuntime.equal (nibble) ((3 : UInt8)) : Bool) then 3 else (if (LexLeanRuntime.equal (nibble) ((4 : UInt8)) : Bool) then 4 else (if (LexLeanRuntime.equal (nibble) ((5 : UInt8)) : Bool) then 5 else (if (LexLeanRuntime.equal (nibble) ((6 : UInt8)) : Bool) then 6 else (if (LexLeanRuntime.equal (nibble) ((7 : UInt8)) : Bool) then 7 else (if (LexLeanRuntime.equal (nibble) ((8 : UInt8)) : Bool) then 8 else (if (LexLeanRuntime.equal (nibble) ((9 : UInt8)) : Bool) then 9 else (if (LexLeanRuntime.equal (nibble) ((10 : UInt8)) : Bool) then 10 else (if (LexLeanRuntime.equal (nibble) ((11 : UInt8)) : Bool) then 11 else (if (LexLeanRuntime.equal (nibble) ((12 : UInt8)) : Bool) then 12 else (if (LexLeanRuntime.equal (nibble) ((13 : UInt8)) : Bool) then 13 else (if (LexLeanRuntime.equal (nibble) ((14 : UInt8)) : Bool) then 14 else 15))))))))))))))); LexLeanStd.Text.sliceOr (table) (idx) (1)))

@[expose] public def zeroStep (subject : ByteArray) (position : Nat) : Option (Nat) := (if byteIs (subject) (position) ((48 : UInt8)) then Option.some ((position + 1)) else Option.none)

@[expose] public def significant (digits : ByteArray) : ByteArray := (let zeros : Nat := ((LexLeanCollections.iterateUntil ((fun (position : Nat) => zeroStep (digits) (position))) ((LexLeanRuntime.length (digits) : Nat)) (0) : (Prod (Nat) (Bool)))).1; LexLeanStd.Text.sliceOr (digits) (zeros) ((LexLeanRuntime.subtract ((LexLeanRuntime.length (digits) : Nat)) (zeros) : Nat)))

@[expose] public def digit (number : Nat) (place : Nat) : ByteArray := LexLeanStd.Text.sliceOr (lit ("0123456789")) ((LexLeanRuntime.remainder ((LexLeanRuntime.quotient (number) (place) (0) : Nat)) (10) (0) : Nat)) (1)

@[expose] public def decimal (number : Nat) : ByteArray := (let zero : ByteArray := lit ("0"); (let digits : ByteArray := significant ((LexLeanRuntime.append (digit (number) (10000000000000000000)) ((LexLeanRuntime.append (digit (number) (1000000000000000000)) ((LexLeanRuntime.append (digit (number) (100000000000000000)) ((LexLeanRuntime.append (digit (number) (10000000000000000)) ((LexLeanRuntime.append (digit (number) (1000000000000000)) ((LexLeanRuntime.append (digit (number) (100000000000000)) ((LexLeanRuntime.append (digit (number) (10000000000000)) ((LexLeanRuntime.append (digit (number) (1000000000000)) ((LexLeanRuntime.append (digit (number) (100000000000)) ((LexLeanRuntime.append (digit (number) (10000000000)) ((LexLeanRuntime.append (digit (number) (1000000000)) ((LexLeanRuntime.append (digit (number) (100000000)) ((LexLeanRuntime.append (digit (number) (10000000)) ((LexLeanRuntime.append (digit (number) (1000000)) ((LexLeanRuntime.append (digit (number) (100000)) ((LexLeanRuntime.append (digit (number) (10000)) ((LexLeanRuntime.append (digit (number) (1000)) ((LexLeanRuntime.append (digit (number) (100)) ((LexLeanRuntime.append (digit (number) (10)) (digit (number) (1)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)); (if (Nat.beq (number) (0)) then zero else digits)))

@[expose] public def bytesInList (needle : ByteArray) (items : List (ByteArray)) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (item : ByteArray) => (if found then true else (LexLeanRuntime.equal (item) (needle) : Bool)))) (false) (items) : Bool)

@[expose] public def runMember (kind : Nat) (byte : UInt8) : Bool := (let space : Bool := (LexLeanRuntime.equal (byte) ((32 : UInt8)) : Bool); (let feed : Bool := (LexLeanRuntime.equal (byte) ((10 : UInt8)) : Bool); (let alpha : Bool := isAsciiAlpha (byte); (let numeric : Bool := isAsciiDigit (byte); (if (Nat.beq (kind) (0)) then (if space then true else feed) else (if (Nat.beq (kind) (1)) then alpha else numeric))))))

@[expose] public def runStep (subject : ByteArray) (n : Nat) (kind : Nat) (cur : Cursor) : Option (Cursor) := (let pos : Nat := (cur).position; (if (Nat.blt (pos) (n)) then (let b : UInt8 := byteOrZero (subject) (pos); (let member : Bool := runMember (kind) (b); (if member then (if (LexLeanRuntime.equal (b) ((10 : UInt8)) : Bool) then Option.some (({ position := (pos + 1), line := ((cur).line + 1), column := 1 } : Cursor)) else Option.some (({ position := (pos + 1), line := (cur).line, column := ((cur).column + 1) } : Cursor))) else Option.none))) else Option.none))

@[expose] public def runEnd (subject : ByteArray) (kind : Nat) (start : Cursor) : Cursor := (let n : Nat := (LexLeanRuntime.length (subject) : Nat); ((LexLeanCollections.iterateUntil ((fun (cur : Cursor) => runStep (subject) (n) (kind) (cur))) ((LexLeanRuntime.subtract (n) ((start).position) : Nat)) (start) : (Prod (Cursor) (Bool)))).1)

@[expose] public def bumpOne (subject : ByteArray) (cur : Cursor) : Cursor := (let pos : Nat := (cur).position; (let width : Nat := scalarWidth (subject) (pos); (let feed : Bool := byteIs (subject) (pos) ((10 : UInt8)); (if feed then ({ position := (pos + 1), line := ((cur).line + 1), column := 1 } : Cursor) else ({ position := (pos + width), line := (cur).line, column := ((cur).column + 1) } : Cursor)))))

@[expose] public def placeOf (path : ByteArray) («from» : Cursor) (to : Cursor) : Place := ({ path := path, byteStart := («from»).position, byteEnd := (to).position, lineStart := («from»).line, columnStart := («from»).column, lineEnd := (to).line, columnEnd := (to).column } : Place)

@[expose] public def placeOfAtom (path : ByteArray) (atom : Atom) : Place := ({ path := path, byteStart := (atom).byteStart, byteEnd := (atom).byteEnd, lineStart := (atom).lineStart, columnStart := (atom).columnStart, lineEnd := (atom).lineEnd, columnEnd := (atom).columnEnd } : Place)

@[expose] public def rejectBackslash (path : ByteArray) («from» : Cursor) (after : Cursor) : Rejection := (let place : Place := placeOf (path) («from») (after); ({ code := lit ("LLL1004"), message := lit ("a control sequence needs ASCII letters or one ASCII nonletter after the backslash"), place := Option.some (place), help := Option.none } : Rejection))

@[expose] public def rejectScalar (path : ByteArray) (subject : ByteArray) (cur : Cursor) : Rejection := (let b0 : UInt8 := byteOrZero (subject) ((cur).position); (let high : UInt8 := (match (LexLeanRuntime.shiftRight (b0) ((4 : UInt32)) : Option (UInt8)) with | Option.some shifted => shifted | Option.none => (0 : UInt8)); (let low : UInt8 := (LexLeanRuntime.bitAnd (b0) ((15 : UInt8)) : UInt8); (let d1 : ByteArray := hexDigit (high); (let d2 : ByteArray := hexDigit (low); (let past : Cursor := ({ position := ((cur).position + 1), line := (cur).line, column := ((cur).column + 1) } : Cursor); (let place : Place := placeOf (path) (cur) (past); ({ code := lit ("LLL1001"), message := cat (cat (lit ("forbidden scalar U+00")) (d1)) (d2), place := Option.some (place), help := Option.none } : Rejection))))))))

@[expose] public def limitRejection (path : ByteArray) (maxAtoms : Nat) : Rejection := ({ code := lit ("LLS8002"), message := cat (cat (cat (lit ("max_primitive_atoms exceeded: configured ")) (decimal (maxAtoms))) (lit (", scanning "))) (path), place := Option.none, help := Option.none } : Rejection)

@[expose] public def lexAtom (path : ByteArray) (subject : ByteArray) (cur : Cursor) : Lexed := (let n : Nat := (LexLeanRuntime.length (subject) : Nat); (let pos : Nat := (cur).position; (let b0 : UInt8 := byteOrZero (subject) (pos); (let wsEnd : Cursor := runEnd (subject) (0) (cur); (let alphaEnd : Cursor := runEnd (subject) (1) (cur); (let numEnd : Cursor := runEnd (subject) (2) (cur); (let one : Cursor := bumpOne (subject) (cur); (let nextPos : Nat := (one).position; (let nb : UInt8 := byteOrZero (subject) (nextPos); (let present : Bool := (Nat.blt (nextPos) (n)); (let nbAlpha : Bool := isAsciiAlpha (nb); (let nbAscii : Bool := inRange (nb) ((0 : UInt8)) ((127 : UInt8)); (let wordEnd : Cursor := runEnd (subject) (1) (one); (let two : Cursor := bumpOne (subject) (one); (let alpha : Bool := isAsciiAlpha (b0); (let numeric : Bool := isAsciiDigit (b0); (let printable : Bool := inRange (b0) ((32 : UInt8)) ((126 : UInt8)); (let ascii : Bool := inRange (b0) ((0 : UInt8)) ((127 : UInt8)); (let delimiter : Bool := (if (LexLeanRuntime.equal (b0) ((123 : UInt8)) : Bool) then true else (if (LexLeanRuntime.equal (b0) ((125 : UInt8)) : Bool) then true else (if (LexLeanRuntime.equal (b0) ((40 : UInt8)) : Bool) then true else (if (LexLeanRuntime.equal (b0) ((41 : UInt8)) : Bool) then true else (if (LexLeanRuntime.equal (b0) ((91 : UInt8)) : Bool) then true else (LexLeanRuntime.equal (b0) ((93 : UInt8)) : Bool)))))); (if (if (LexLeanRuntime.equal (b0) ((32 : UInt8)) : Bool) then true else (LexLeanRuntime.equal (b0) ((10 : UInt8)) : Bool)) then ({ kind := AtomClass.Whitespace, after := wsEnd, error := Option.none } : Lexed) else (if (LexLeanRuntime.equal (b0) ((92 : UInt8)) : Bool) then (if (if present then nbAlpha else false) then ({ kind := AtomClass.Control, after := wordEnd, error := Option.none } : Lexed) else (if (if present then nbAscii else false) then ({ kind := AtomClass.Control, after := two, error := Option.none } : Lexed) else (let rejected : Rejection := rejectBackslash (path) (cur) (one); ({ kind := AtomClass.Control, after := one, error := Option.some (rejected) } : Lexed)))) else (if alpha then ({ kind := AtomClass.Word, after := alphaEnd, error := Option.none } : Lexed) else (if numeric then ({ kind := AtomClass.Numeral, after := numEnd, error := Option.none } : Lexed) else (if delimiter then ({ kind := AtomClass.Delimiter, after := one, error := Option.none } : Lexed) else (if printable then ({ kind := AtomClass.AsciiSymbol, after := one, error := Option.none } : Lexed) else (if ascii then (let rejected : Rejection := rejectScalar (path) (subject) (cur); ({ kind := AtomClass.Control, after := cur, error := Option.some (rejected) } : Lexed)) else ({ kind := AtomClass.UnicodeSymbol, after := one, error := Option.none } : Lexed)))))))))))))))))))))))))))

@[expose] public def stepScan (path : ByteArray) (subject : ByteArray) (n : Nat) (maxAtoms : Nat) (state : ScanState) : Option (ScanState) := (let cur : Cursor := (state).cursor; (let pos : Nat := (cur).position; (match (state).error with | Option.some _ => Option.none | Option.none => (if (Nat.blt (pos) (n)) then (let lexed : Lexed := lexAtom (path) (subject) (cur); (match (lexed).error with | Option.some rejection => Option.some (({ cursor := cur, atoms := (state).atoms, count := (state).count, error := Option.some (rejection) } : ScanState)) | Option.none => (let after : Cursor := (lexed).after; (let atom : Atom := ({ kind := (lexed).kind, byteStart := pos, byteEnd := (after).position, lineStart := (cur).line, columnStart := (cur).column, lineEnd := (after).line, columnEnd := (after).column, text := LexLeanStd.Text.sliceOr (subject) (pos) ((LexLeanRuntime.subtract ((after).position) (pos) : Nat)) } : Atom); (let count : Nat := ((state).count + 1); (if (Nat.blt (maxAtoms) (count)) then (let rejection : Rejection := limitRejection (path) (maxAtoms); Option.some (({ cursor := after, atoms := (state).atoms, count := count, error := Option.some (rejection) } : ScanState))) else Option.some (({ cursor := after, atoms := (atom :: (state).atoms), count := count, error := Option.none } : ScanState)))))))) else Option.none))))

@[expose] public def scanSource (path : ByteArray) (text : ByteArray) (maxAtoms : Nat) : Except (Rejection) (List (Atom)) := (let n : Nat := (LexLeanRuntime.length (text) : Nat); (let final : ScanState := ((LexLeanCollections.iterateUntil ((fun (state : ScanState) => stepScan (path) (text) (n) (maxAtoms) (state))) ((n + 1)) (({ cursor := ({ position := 0, line := 1, column := 1 } : Cursor), atoms := ([] : List (Atom)), count := 0, error := Option.none } : ScanState)) : (Prod (ScanState) (Bool)))).1; (match (final).error with | Option.some rejection => Except.error (rejection) | Option.none => Except.ok (reversed (Atom) ((final).atoms)))))

@[expose] public def classCode (atom : Atom) : Nat := (match (atom).kind with | AtomClass.Control => 0 | AtomClass.Word => 1 | AtomClass.Numeral => 2 | AtomClass.Delimiter => 3 | AtomClass.AsciiSymbol => 4 | AtomClass.UnicodeSymbol => 5 | AtomClass.Whitespace => 6)

@[expose] public def isDataControl (name : ByteArray) : Bool := (if (LexLeanRuntime.equal (name) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 92)), (_root_.UInt8.ofNat (nat_lit 115)), (_root_.UInt8.ofNat (nat_lit 101)), (_root_.UInt8.ofNat (nat_lit 109)), (_root_.UInt8.ofNat (nat_lit 97)), (_root_.UInt8.ofNat (nat_lit 110)), (_root_.UInt8.ofNat (nat_lit 116)), (_root_.UInt8.ofNat (nat_lit 105)), (_root_.UInt8.ofNat (nat_lit 99)), (_root_.UInt8.ofNat (nat_lit 100)), (_root_.UInt8.ofNat (nat_lit 97)), (_root_.UInt8.ofNat (nat_lit 116)), (_root_.UInt8.ofNat (nat_lit 97))]) : Bool) then true else (if (LexLeanRuntime.equal (name) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 92)), (_root_.UInt8.ofNat (nat_lit 116)), (_root_.UInt8.ofNat (nat_lit 111)), (_root_.UInt8.ofNat (nat_lit 111)), (_root_.UInt8.ofNat (nat_lit 108)), (_root_.UInt8.ofNat (nat_lit 105)), (_root_.UInt8.ofNat (nat_lit 110)), (_root_.UInt8.ofNat (nat_lit 103)), (_root_.UInt8.ofNat (nat_lit 100)), (_root_.UInt8.ofNat (nat_lit 97)), (_root_.UInt8.ofNat (nat_lit 116)), (_root_.UInt8.ofNat (nat_lit 97))]) : Bool) then true else (LexLeanRuntime.equal (name) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 92)), (_root_.UInt8.ofNat (nat_lit 99)), (_root_.UInt8.ofNat (nat_lit 111)), (_root_.UInt8.ofNat (nat_lit 114)), (_root_.UInt8.ofNat (nat_lit 101)), (_root_.UInt8.ofNat (nat_lit 100)), (_root_.UInt8.ofNat (nat_lit 97)), (_root_.UInt8.ofNat (nat_lit 116)), (_root_.UInt8.ofNat (nat_lit 97))]) : Bool)))

@[expose] public def payloadStep (text : ByteArray) (track : Bool) (state : Payload) : Option (Payload) := (let index : Nat := (state).index; (if (Nat.blt (index) ((LexLeanRuntime.length (text) : Nat))) then (let b : UInt8 := byteOrZero (text) (index); (let isQuote : Bool := (LexLeanRuntime.equal (b) ((34 : UInt8)) : Bool); (let isSlash : Bool := (LexLeanRuntime.equal (b) ((92 : UInt8)) : Bool); (let isOpen : Bool := (if track then (LexLeanRuntime.equal (b) ((123 : UInt8)) : Bool) else false); (let isClose : Bool := (if track then (LexLeanRuntime.equal (b) ((125 : UInt8)) : Bool) else false); (let depth : Nat := (state).depth; (let quoted : Bool := (state).quoted; (let escaped : Bool := (state).escaped; (if quoted then (if escaped then Option.some (({ index := (index + 1), depth := depth, quoted := true, escaped := false } : Payload)) else (if isSlash then Option.some (({ index := (index + 1), depth := depth, quoted := true, escaped := true } : Payload)) else (if isQuote then Option.some (({ index := (index + 1), depth := depth, quoted := false, escaped := escaped } : Payload)) else Option.some (({ index := (index + 1), depth := depth, quoted := true, escaped := escaped } : Payload))))) else (if isQuote then Option.some (({ index := (index + 1), depth := depth, quoted := true, escaped := escaped } : Payload)) else (if isOpen then Option.some (({ index := (index + 1), depth := (depth + 1), quoted := false, escaped := escaped } : Payload)) else (if isClose then Option.some (({ index := (index + 1), depth := (LexLeanRuntime.subtract (depth) (1) : Nat), quoted := false, escaped := escaped } : Payload)) else Option.some (({ index := (index + 1), depth := depth, quoted := false, escaped := escaped } : Payload)))))))))))))) else Option.none))

@[expose] public def payloadRun (text : ByteArray) (track : Bool) (context : Context) : Payload := ((LexLeanCollections.iterateUntil ((fun (state : Payload) => payloadStep (text) (track) (state))) ((LexLeanRuntime.length (text) : Nat)) (({ index := 0, depth := (context).depth, quoted := (context).quoted, escaped := (context).escaped } : Payload)) : (Prod (Payload) (Bool)))).1

@[expose] public def consume (ctx : Context) (atom : Atom) : Context := (let bd : Nat := (ctx).body; (let dep : Nat := (ctx).depth; (let env : Nat := (ctx).environment; (let t : ByteArray := (atom).text; (let code : Nat := classCode (atom); (let ctl : Bool := (Nat.beq (code) (0)); (let ws : Bool := (Nat.beq (code) (6)); (let named : Bool := isDataControl (t); (let data : Bool := (if ctl then named else false); (let endName : Bool := (LexLeanRuntime.equal (t) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 92)), (_root_.UInt8.ofNat (nat_lit 101)), (_root_.UInt8.ofNat (nat_lit 110)), (_root_.UInt8.ofNat (nat_lit 100))]) : Bool); (let isEnd : Bool := (if ctl then endName else false); (let run : Payload := payloadRun (t) ((Nat.beq (bd) (0))) (ctx); (let «notation» : Context := (if (if (ctx).quoted then false else isEnd) then ({ awaitingOpen := (ctx).awaitingOpen, depth := (ctx).depth, quoted := (ctx).quoted, escaped := false, environment := (ctx).environment, body := 0 } : Context) else ({ awaitingOpen := (ctx).awaitingOpen, depth := (ctx).depth, quoted := (run).quoted, escaped := (run).escaped, environment := (ctx).environment, body := 2 } : Context)); (let payload : Context := ({ awaitingOpen := (ctx).awaitingOpen, depth := (run).depth, quoted := (run).quoted, escaped := (run).escaped, environment := (ctx).environment, body := (ctx).body } : Context); (let isBegin : Bool := (if ctl then (LexLeanRuntime.equal (t) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 92)), (_root_.UInt8.ofNat (nat_lit 98)), (_root_.UInt8.ofNat (nat_lit 101)), (_root_.UInt8.ofNat (nat_lit 103)), (_root_.UInt8.ofNat (nat_lit 105)), (_root_.UInt8.ofNat (nat_lit 110))]) : Bool) else false); (let isOpenB : Bool := (LexLeanRuntime.equal (t) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 123))]) : Bool); (let isCloseB : Bool := (LexLeanRuntime.equal (t) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 125))]) : Bool); (let isModule : Bool := (if (Nat.beq (code) (1)) then (if (LexLeanRuntime.equal (t) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 115)), (_root_.UInt8.ofNat (nat_lit 101)), (_root_.UInt8.ofNat (nat_lit 109)), (_root_.UInt8.ofNat (nat_lit 97)), (_root_.UInt8.ofNat (nat_lit 110)), (_root_.UInt8.ofNat (nat_lit 116)), (_root_.UInt8.ofNat (nat_lit 105)), (_root_.UInt8.ofNat (nat_lit 99)), (_root_.UInt8.ofNat (nat_lit 109)), (_root_.UInt8.ofNat (nat_lit 111)), (_root_.UInt8.ofNat (nat_lit 100)), (_root_.UInt8.ofNat (nat_lit 117)), (_root_.UInt8.ofNat (nat_lit 108)), (_root_.UInt8.ofNat (nat_lit 101))]) : Bool) then true else (if (LexLeanRuntime.equal (t) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 116)), (_root_.UInt8.ofNat (nat_lit 111)), (_root_.UInt8.ofNat (nat_lit 111)), (_root_.UInt8.ofNat (nat_lit 108)), (_root_.UInt8.ofNat (nat_lit 105)), (_root_.UInt8.ofNat (nat_lit 110)), (_root_.UInt8.ofNat (nat_lit 103)), (_root_.UInt8.ofNat (nat_lit 109)), (_root_.UInt8.ofNat (nat_lit 111)), (_root_.UInt8.ofNat (nat_lit 100)), (_root_.UInt8.ofNat (nat_lit 117)), (_root_.UInt8.ofNat (nat_lit 108)), (_root_.UInt8.ofNat (nat_lit 101))]) : Bool) then true else (LexLeanRuntime.equal (t) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 99)), (_root_.UInt8.ofNat (nat_lit 111)), (_root_.UInt8.ofNat (nat_lit 114)), (_root_.UInt8.ofNat (nat_lit 101)), (_root_.UInt8.ofNat (nat_lit 109)), (_root_.UInt8.ofNat (nat_lit 111)), (_root_.UInt8.ofNat (nat_lit 100)), (_root_.UInt8.ofNat (nat_lit 117)), (_root_.UInt8.ofNat (nat_lit 108)), (_root_.UInt8.ofNat (nat_lit 101))]) : Bool))) else false); (let envNext0 : Nat := (if (Nat.beq (env) (0)) then (if isBegin then 1 else 0) else (if (Nat.beq (env) (1)) then (if isOpenB then 2 else 0) else (if (Nat.beq (env) (2)) then (if isModule then 3 else 0) else (if (Nat.beq (env) (3)) then (if isCloseB then 4 else 0) else 0)))); (let finished : Bool := (Nat.beq (envNext0) (4)); (let envNext : Nat := (if finished then 0 else envNext0); (let environment : Context := (if ws then ctx else (if finished then ({ awaitingOpen := (ctx).awaitingOpen, depth := (ctx).depth, quoted := (ctx).quoted, escaped := (ctx).escaped, environment := 0, body := 1 } : Context) else (if (if (ctx).awaitingOpen then isOpenB else false) then ({ awaitingOpen := false, depth := 1, quoted := (ctx).quoted, escaped := (ctx).escaped, environment := envNext, body := (ctx).body } : Context) else ({ awaitingOpen := data, depth := (ctx).depth, quoted := (ctx).quoted, escaped := (ctx).escaped, environment := envNext, body := (ctx).body } : Context)))); (if (Nat.beq (bd) (1)) then (if ws then ctx else (if data then ({ awaitingOpen := true, depth := (ctx).depth, quoted := (ctx).quoted, escaped := (ctx).escaped, environment := (ctx).environment, body := 0 } : Context) else «notation»)) else (if (Nat.beq (bd) (2)) then «notation» else (if (Nat.beq (dep) (0)) then environment else payload)))))))))))))))))))))))))

@[expose] public def forbiddenRejection (path : ByteArray) (atom : Atom) : Rejection := (let place : Place := placeOfAtom (path) (atom); ({ code := lit ("LLL1002"), message := cat (cat (cat (ticks ((atom).text)) (lit (" is an always-forbidden TeX control ("))) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 194)), (_root_.UInt8.ofNat (nat_lit 167))])) (lit ("12.4); LexLean does not expand TeX")), place := Option.some (place), help := Option.none } : Rejection))

@[expose] public def canonicalDecimal (digits : ByteArray) : ByteArray := (let stripped : ByteArray := significant (digits); (if (Nat.beq ((LexLeanRuntime.length (stripped) : Nat)) (0)) then lit ("0") else stripped))

@[expose] public def leadingRejection (path : ByteArray) (atom : Atom) : Rejection := (let place : Place := placeOfAtom (path) (atom); (let canonical : ByteArray := canonicalDecimal ((atom).text); (let shown : ByteArray := ticks ((atom).text); ({ code := lit ("LLL1003"), message := cat (cat (cat (cat (lit ("numeral ")) (shown)) (lit (" has a redundant leading zero; the canonical decimal is "))) (ticks (canonical))) (lit ("")), place := Option.some (place), help := Option.some (cat (cat (cat (lit ("replace ")) (shown)) (lit (" with "))) (ticks (canonical))) } : Rejection))))

@[expose] public def atomRejection (path : ByteArray) (atom : Atom) (previous : Option (Atom)) (ctx : Context) (forbidden : List (ByteArray)) : Option (Rejection) := (let t : ByteArray := (atom).text; (let code : Nat := classCode (atom); (let hit : Bool := bytesInList (t) (forbidden); (let zeroLead : Bool := byteIs (t) (0) ((48 : UInt8)); (let longer : Bool := (Nat.blt (1) ((LexLeanRuntime.length (t) : Nat))); (let leading : Bool := (if (Nat.beq (code) (2)) then (if longer then zeroLead else false) else false); (let continues : Bool := (match previous with | Option.some before => (let before_code : Nat := classCode (before); (let adjacent : Bool := (Nat.beq ((before).byteEnd) ((atom).byteStart)); (let joiner : Bool := (let bt : ByteArray := (before).text; (if (LexLeanRuntime.equal (bt) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 95))]) : Bool) then true else (if (LexLeanRuntime.equal (bt) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 39))]) : Bool) then true else (LexLeanRuntime.equal (bt) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 45))]) : Bool)))); (if adjacent then (if (Nat.beq (before_code) (1)) then true else (if (Nat.beq (before_code) (4)) then joiner else false)) else false)))) | Option.none => false); (if (if (Nat.beq (code) (0)) then hit else false) then (let rejection : Rejection := forbiddenRejection (path) (atom); Option.some (rejection)) else (if leading then (if continues then Option.none else (if (ctx).quoted then Option.none else (let rejection : Rejection := leadingRejection (path) (atom); Option.some (rejection)))) else Option.none)))))))))

@[expose] public def checkStep (path : ByteArray) (forbidden : List (ByteArray)) (state : CheckState) (atom : Atom) : CheckState := (match (state).error with | Option.some _ => state | Option.none => (let ctx : Context := (state).context; (let found : Option (Rejection) := atomRejection (path) (atom) ((state).previous) (ctx) (forbidden); (let following : Context := consume (ctx) (atom); ({ context := following, previous := Option.some (atom), error := found } : CheckState)))))

@[expose] public def rejectAtoms (path : ByteArray) (atoms : List (Atom)) (forbidden : List (ByteArray)) : Option (Rejection) := (let final : CheckState := (LexLeanCollections.listFold ((fun (state : CheckState) (atom : Atom) => checkStep (path) (forbidden) (state) (atom))) (({ context := ({ awaitingOpen := false, depth := 0, quoted := false, escaped := false, environment := 0, body := 0 } : Context), previous := Option.none, error := Option.none } : CheckState)) (atoms) : CheckState); (final).error)

end LexLeanBoot.PrimitiveScan
