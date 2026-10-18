module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Text
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.AxiomOutput

public structure Record where
  declaration : ByteArray
  axioms : List (ByteArray)
  expected : Option (ByteArray)

public structure Rejection where
  message : ByteArray
  declaration : Option (ByteArray)

public inductive Scan where
  | between (_ : List (Record)) (_ : List (ByteArray))
  | inside (_ : List (Record)) (_ : List (ByteArray)) (_ : ByteArray) (_ : Option (ByteArray))
  | stopped (_ : Rejection)

@[expose] public def text (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def cat (left : ByteArray) (right : ByteArray) : ByteArray := (LexLeanRuntime.append (left) (right) : ByteArray)

@[expose] public def quoted (name : ByteArray) : ByteArray := cat (cat (text ("`")) (name)) (text ("`"))

@[expose] public def listLength (Item : Type) (items : List (Item)) : Nat := (LexLeanCollections.listFold ((fun (count : Nat) (_item : Item) => (count + 1))) (0) (items) : Nat)

@[expose] public def reversed (Item : Type) (items : List (Item)) : List (Item) := (LexLeanCollections.listFold ((fun (turned : List (Item)) (item : Item) => (item :: turned))) (([] : List (Item))) (items) : List (Item))

@[expose] public def rejected (message : ByteArray) (declaration : Option (ByteArray)) : Rejection := ({ message := message, declaration := declaration } : Rejection)

@[expose] public def refuse (message : ByteArray) (declaration : Option (ByteArray)) : Except (Rejection) (List ((Prod (ByteArray) (List (ByteArray))))) := Except.error (rejected (message) (declaration))

@[expose] public def inRange (byte : UInt8) (low : UInt8) (high : UInt8) : Bool := (match (LexLeanRuntime.checkedSubtract (byte) (low) : Option (UInt8)) with | Option.some _ => (match (LexLeanRuntime.checkedSubtract (high) (byte) : Option (UInt8)) with | Option.some _ => true | Option.none => false) | Option.none => false)

@[expose] public def isFirst (byte : UInt8) : Bool := (if inRange (byte) ((65 : UInt8)) ((90 : UInt8)) then true else (if inRange (byte) ((97 : UInt8)) ((122 : UInt8)) then true else (LexLeanRuntime.equal (byte) ((95 : UInt8)) : Bool)))

@[expose] public def isLater (byte : UInt8) : Bool := (if isFirst (byte) then true else (if inRange (byte) ((48 : UInt8)) ((57 : UInt8)) then true else (if (LexLeanRuntime.equal (byte) ((63 : UInt8)) : Bool) then true else (LexLeanRuntime.equal (byte) ((39 : UInt8)) : Bool))))

@[expose] public def nameMode (mode : Nat) (byte : UInt8) : Nat := (if (Nat.beq (mode) (2)) then 2 else (if (LexLeanRuntime.equal (byte) ((46 : UInt8)) : Bool) then (if (Nat.beq (mode) (1)) then 0 else 2) else (if isFirst (byte) then 1 else (if (Nat.beq (mode) (1)) then (if isLater (byte) then 1 else 2) else 2))))

@[expose] public def nameStep (subject : ByteArray) (state : (Prod (Nat) (Nat))) : Option ((Prod (Nat) (Nat))) := (if (Nat.beq ((state).2) (2)) then Option.none else (match LexLeanStd.Text.byteAt (subject) ((state).1) with | Option.some byte => Option.some ((((state).1 + 1), nameMode ((state).2) (byte))) | Option.none => Option.none))

@[expose] public def isClosedLeanName (raw : ByteArray) : Bool := (Nat.beq ((((LexLeanCollections.iterateUntil ((fun (state : (Prod (Nat) (Nat))) => nameStep (raw) (state))) ((LexLeanRuntime.length (raw) : Nat)) ((0, 0)) : (Prod ((Prod (Nat) (Nat))) (Bool)))).1).2) (1))

@[expose] public def byteIs (subject : ByteArray) (position : Nat) (value : UInt8) : Bool := (match LexLeanStd.Text.byteAt (subject) (position) with | Option.some byte => (LexLeanRuntime.equal (byte) (value) : Bool) | Option.none => false)

@[expose] public def byteIn (subject : ByteArray) (position : Nat) (low : UInt8) (high : UInt8) : Bool := (match LexLeanStd.Text.byteAt (subject) (position) with | Option.some byte => inRange (byte) (low) (high) | Option.none => false)

@[expose] public def whiteLength (subject : ByteArray) (position : Nat) : Nat := (if (if byteIn (subject) (position) ((9 : UInt8)) ((13 : UInt8)) then true else (if byteIs (subject) (position) ((32 : UInt8)) then true else false)) then 1 else (if byteIs (subject) (position) ((194 : UInt8)) then (if (if byteIs (subject) ((position + 1)) ((133 : UInt8)) then true else (if byteIs (subject) ((position + 1)) ((160 : UInt8)) then true else false)) then 2 else 0) else (if byteIs (subject) (position) ((225 : UInt8)) then (if (if byteIs (subject) ((position + 1)) ((154 : UInt8)) then (if byteIs (subject) ((position + 2)) ((128 : UInt8)) then true else false) else false) then 3 else 0) else (if byteIs (subject) (position) ((226 : UInt8)) then (if byteIs (subject) ((position + 1)) ((128 : UInt8)) then (if (if byteIn (subject) ((position + 2)) ((128 : UInt8)) ((138 : UInt8)) then true else (if byteIs (subject) ((position + 2)) ((168 : UInt8)) then true else (if byteIs (subject) ((position + 2)) ((169 : UInt8)) then true else (if byteIs (subject) ((position + 2)) ((175 : UInt8)) then true else false)))) then 3 else 0) else (if (if byteIs (subject) ((position + 1)) ((129 : UInt8)) then (if byteIs (subject) ((position + 2)) ((159 : UInt8)) then true else false) else false) then 3 else 0)) else (if byteIs (subject) (position) ((227 : UInt8)) then (if (if byteIs (subject) ((position + 1)) ((128 : UInt8)) then (if byteIs (subject) ((position + 2)) ((128 : UInt8)) then true else false) else false) then 3 else 0) else 0)))))

@[expose] public def whiteEnding (subject : ByteArray) («end» : Nat) : Nat := (if (Nat.beq («end») (0)) then 0 else (if (Nat.beq (whiteLength (subject) ((LexLeanRuntime.subtract («end») (1) : Nat))) (1)) then 1 else (if (if (Nat.ble (2) («end»)) then (Nat.beq (whiteLength (subject) ((LexLeanRuntime.subtract («end») (2) : Nat))) (2)) else false) then 2 else (if (if (Nat.ble (3) («end»)) then (Nat.beq (whiteLength (subject) ((LexLeanRuntime.subtract («end») (3) : Nat))) (3)) else false) then 3 else 0))))

@[expose] public def startsWhite (subject : ByteArray) : Bool := (Nat.blt (0) (whiteLength (subject) (0)))

@[expose] public def skipWhite (subject : ByteArray) : Nat := ((LexLeanCollections.iterateUntil ((fun (position : Nat) => (if (Nat.blt (0) (whiteLength (subject) (position))) then Option.some ((position + whiteLength (subject) (position))) else Option.none))) ((LexLeanRuntime.length (subject) : Nat)) (0) : (Prod (Nat) (Bool)))).1

@[expose] public def dropWhiteEnd (subject : ByteArray) (start : Nat) : Nat := ((LexLeanCollections.iterateUntil ((fun («end» : Nat) => (if (if (Nat.blt (0) (whiteEnding (subject) («end»))) then (Nat.ble ((start + whiteEnding (subject) («end»))) («end»)) else false) then Option.some ((LexLeanRuntime.subtract («end») (whiteEnding (subject) («end»)) : Nat)) else Option.none))) ((LexLeanRuntime.length (subject) : Nat)) ((LexLeanRuntime.length (subject) : Nat)) : (Prod (Nat) (Bool)))).1

@[expose] public def trimmed (subject : ByteArray) : ByteArray := (let start : Nat := skipWhite (subject); (let «end» : Nat := dropWhiteEnd (subject) (start); LexLeanStd.Text.sliceOr (subject) (start) ((LexLeanRuntime.subtract («end») (start) : Nat))))

@[expose] public def stripPrefix (subject : ByteArray) («prefix» : ByteArray) : Option (ByteArray) := (if LexLeanStd.Text.startsWith (subject) («prefix») then Option.some (LexLeanStd.Text.sliceOr (subject) ((LexLeanRuntime.length («prefix») : Nat)) ((LexLeanRuntime.subtract ((LexLeanRuntime.length (subject) : Nat)) ((LexLeanRuntime.length («prefix») : Nat)) : Nat))) else Option.none)

@[expose] public def stripSuffix (subject : ByteArray) (suffix : ByteArray) : Option (ByteArray) := (if LexLeanStd.Text.endsWith (subject) (suffix) then Option.some (LexLeanStd.Text.sliceOr (subject) (0) ((LexLeanRuntime.subtract ((LexLeanRuntime.length (subject) : Nat)) ((LexLeanRuntime.length (suffix) : Nat)) : Nat))) else Option.none)

@[expose] public def splitOnce (subject : ByteArray) (delimiter : ByteArray) : Option ((Prod (ByteArray) (ByteArray))) := (match LexLeanStd.Text.find (subject) (delimiter) with | Option.some position => Option.some ((LexLeanStd.Text.sliceOr (subject) (0) (position), LexLeanStd.Text.sliceOr (subject) ((position + (LexLeanRuntime.length (delimiter) : Nat))) ((LexLeanRuntime.subtract ((LexLeanRuntime.length (subject) : Nat)) ((position + (LexLeanRuntime.length (delimiter) : Nat))) : Nat)))) | Option.none => Option.none)

@[expose] public def lastStep (subject : ByteArray) (needle : ByteArray) (position : Nat) : Option (Nat) := (if LexLeanStd.Text.matchAt (subject) (needle) (position) then Option.none else (if (Nat.blt (0) (position)) then Option.some ((LexLeanRuntime.subtract (position) (1) : Nat)) else Option.none))

@[expose] public def splitLast (subject : ByteArray) (needle : ByteArray) : Option ((Prod (ByteArray) (ByteArray))) := (let total : Nat := (LexLeanRuntime.length (subject) : Nat); (let size : Nat := (LexLeanRuntime.length (needle) : Nat); (if (Nat.blt (total) (size)) then Option.none else (let position : Nat := ((LexLeanCollections.iterateUntil ((fun (position : Nat) => lastStep (subject) (needle) (position))) ((total + 1)) ((LexLeanRuntime.subtract (total) (size) : Nat)) : (Prod (Nat) (Bool)))).1; (if LexLeanStd.Text.matchAt (subject) (needle) (position) then Option.some ((LexLeanStd.Text.sliceOr (subject) (0) (position), LexLeanStd.Text.sliceOr (subject) ((position + size)) ((LexLeanRuntime.subtract (total) ((position + size)) : Nat)))) else Option.none)))))

@[expose] public def digitStep (subject : ByteArray) (position : Nat) : Option (Nat) := (if byteIn (subject) (position) ((48 : UInt8)) ((57 : UInt8)) then Option.some ((position + 1)) else Option.none)

@[expose] public def allDigits (subject : ByteArray) : Bool := (Nat.beq (((LexLeanCollections.iterateUntil ((fun (position : Nat) => digitStep (subject) (position))) ((LexLeanRuntime.length (subject) : Nat)) (0) : (Prod (Nat) (Bool)))).1) ((LexLeanRuntime.length (subject) : Nat)))

@[expose] public def zeroStep (subject : ByteArray) (position : Nat) : Option (Nat) := (if byteIs (subject) (position) ((48 : UInt8)) then Option.some ((position + 1)) else Option.none)

@[expose] public def significant (digits : ByteArray) : ByteArray := (let zeros : Nat := ((LexLeanCollections.iterateUntil ((fun (position : Nat) => zeroStep (digits) (position))) ((LexLeanRuntime.length (digits) : Nat)) (0) : (Prod (Nat) (Bool)))).1; LexLeanStd.Text.sliceOr (digits) (zeros) ((LexLeanRuntime.subtract ((LexLeanRuntime.length (digits) : Nat)) (zeros) : Nat)))

@[expose] public def digit (number : Nat) (place : Nat) : ByteArray := LexLeanStd.Text.sliceOr (text ("0123456789")) ((LexLeanRuntime.remainder ((LexLeanRuntime.quotient (number) (place) (0) : Nat)) (10) (0) : Nat)) (1)

@[expose] public def decimal (number : Nat) : ByteArray := (let zero : ByteArray := text ("0"); (let digits : ByteArray := significant ((LexLeanRuntime.append (digit (number) (10000000000000000000)) ((LexLeanRuntime.append (digit (number) (1000000000000000000)) ((LexLeanRuntime.append (digit (number) (100000000000000000)) ((LexLeanRuntime.append (digit (number) (10000000000000000)) ((LexLeanRuntime.append (digit (number) (1000000000000000)) ((LexLeanRuntime.append (digit (number) (100000000000000)) ((LexLeanRuntime.append (digit (number) (10000000000000)) ((LexLeanRuntime.append (digit (number) (1000000000000)) ((LexLeanRuntime.append (digit (number) (100000000000)) ((LexLeanRuntime.append (digit (number) (10000000000)) ((LexLeanRuntime.append (digit (number) (1000000000)) ((LexLeanRuntime.append (digit (number) (100000000)) ((LexLeanRuntime.append (digit (number) (10000000)) ((LexLeanRuntime.append (digit (number) (1000000)) ((LexLeanRuntime.append (digit (number) (100000)) ((LexLeanRuntime.append (digit (number) (10000)) ((LexLeanRuntime.append (digit (number) (1000)) ((LexLeanRuntime.append (digit (number) (100)) ((LexLeanRuntime.append (digit (number) (10)) (digit (number) (1)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)); (if (Nat.beq (number) (0)) then zero else digits)))

@[expose] public def fitsWord (digits : ByteArray) : Bool := (if (Nat.blt ((LexLeanRuntime.length (digits) : Nat)) (20)) then true else (if (Nat.beq ((LexLeanRuntime.length (digits) : Nat)) (20)) then (!(LexLeanRuntime.equal ((LexLeanRuntime.compareBytes (digits) (text ("18446744073709551615")) : Ordering)) ((LexLeanRuntime.compareBytes (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 1))]) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 0))]) : Ordering)) : Bool)) else false))

@[expose] public def isWord (raw : ByteArray) : Bool := (let digits : ByteArray := (match stripPrefix (raw) (text ("+")) with | Option.some unsigned => unsigned | Option.none => raw); (let fits : Bool := fitsWord (significant (digits)); (if (Nat.blt (0) ((LexLeanRuntime.length (digits) : Nat))) then (if allDigits (digits) then fits else false) else false)))

@[expose] public def isCount (field : ByteArray) : Bool := isWord (trimmed (field))

@[expose] public def afterLabel (rest : ByteArray) : Option (ByteArray) := (match stripPrefix (rest) (text (" ")) with | Option.some spaced => stripPrefix (spaced) (text ("info: ")) | Option.none => Option.none)

@[expose] public def afterLocation (column : ByteArray) (rest : ByteArray) : Option (ByteArray) := (match splitOnce (column) (text ("-")) with | Option.some span => (let low : ByteArray := (span).1; (let high : ByteArray := (span).2; (match splitOnce (rest) (text (":")) with | Option.some tail => (let stop_column : ByteArray := (tail).1; (let after : ByteArray := (tail).2; (let labeled : Option (ByteArray) := afterLabel (after); (if isCount (low) then (if isCount (high) then (if isCount (stop_column) then labeled else Option.none) else Option.none) else Option.none)))) | Option.none => Option.none))) | Option.none => (let labeled : Option (ByteArray) := afterLabel (rest); (if isCount (column) then labeled else Option.none)))

@[expose] public def located (line : ByteArray) : Option (ByteArray) := (match splitOnce (line) (text (":")) with | Option.some head => (match splitOnce ((head).2) (text (":")) with | Option.some middle => (match splitOnce ((middle).2) (text (":")) with | Option.some tail => (let line_number : ByteArray := (middle).1; (let column : ByteArray := (tail).1; (let rest : ByteArray := (tail).2; (let located_rest : Option (ByteArray) := afterLocation (column) (rest); (if isCount (line_number) then located_rest else Option.none))))) | Option.none => Option.none) | Option.none => Option.none) | Option.none => Option.none)

@[expose] public def envelope (line : ByteArray) : Option (ByteArray) := (if LexLeanStd.Text.startsWith (line) (text ("'")) then Option.some (line) else (match stripPrefix (line) (text ("info: ")) with | Option.some payload => Option.some (payload) | Option.none => located (line)))

@[expose] public def terminates (payload : ByteArray) : Bool := (match stripSuffix (payload) (text ("]")) with | Option.some _ => true | Option.none => (match stripSuffix (payload) (text (" does not depend on any axioms")) with | Option.some _ => true | Option.none => false))

@[expose] public def lessBytes (left : ByteArray) (right : ByteArray) : Bool := (LexLeanRuntime.equal ((LexLeanRuntime.compareBytes (left) (right) : Ordering)) ((LexLeanRuntime.compareBytes (_root_.ByteArray.mk #[]) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 1))]) : Ordering)) : Bool)

@[expose] public def insertAxiom : (lemma : ByteArray) -> (sorted : List (ByteArray)) -> Option (List (ByteArray))
  | lemma, List.nil => Option.some ((lemma :: ([] : List (ByteArray))))
  | lemma, List.cons head tail => (if (LexLeanRuntime.equal (lemma) (head) : Bool) then Option.none else (if lessBytes (lemma) (head) then Option.some ((lemma :: (head :: tail))) else (match insertAxiom (lemma) (tail) with | Option.some inserted => Option.some ((head :: inserted)) | Option.none => Option.none)))

@[expose] public def insertEntry : (name : ByteArray) -> (axioms : List (ByteArray)) -> (entries : List ((Prod (ByteArray) (List (ByteArray))))) -> Option (List ((Prod (ByteArray) (List (ByteArray)))))
  | name, axioms, List.nil => Option.some (((name, axioms) :: ([] : List ((Prod (ByteArray) (List (ByteArray)))))))
  | name, axioms, List.cons head tail => (if (LexLeanRuntime.equal (name) ((head).1) : Bool) then Option.none else (if lessBytes (name) ((head).1) then Option.some (((name, axioms) :: (head :: tail))) else (match insertEntry (name) (axioms) (tail) with | Option.some inserted => Option.some ((head :: inserted)) | Option.none => Option.none)))

@[expose] public def notLean (name : ByteArray) : ByteArray := cat (quoted (name)) (text (" is not a Lean name"))

@[expose] public def parsed (name : ByteArray) (axioms : List (ByteArray)) : Except (ByteArray) ((Prod (ByteArray) (List (ByteArray)))) := Except.ok ((name, axioms))

@[expose] public def malformed (message : ByteArray) : Except (ByteArray) ((Prod (ByteArray) (List (ByteArray)))) := Except.error (message)

@[expose] public def unrecognized (payload : ByteArray) : Except (ByteArray) ((Prod (ByteArray) (List (ByteArray)))) := malformed (cat (text ("unrecognized axiom payload: ")) (payload))

@[expose] public def collectStep (state : Except (ByteArray) (List (ByteArray))) (piece : ByteArray) : Except (ByteArray) (List (ByteArray)) := (match state with | Except.error _ => state | Except.ok sorted => (let lemma : ByteArray := trimmed (piece); (if isClosedLeanName (lemma) then (match insertAxiom (lemma) (sorted) with | Option.some inserted => Except.ok (inserted) | Option.none => Except.error (cat (text ("duplicate axiom name ")) (quoted (lemma)))) else Except.error (notLean (lemma)))))

@[expose] public def axiomList (name : ByteArray) (list : ByteArray) : Except (ByteArray) ((Prod (ByteArray) (List (ByteArray)))) := (match (LexLeanCollections.listFold ((fun (state : Except (ByteArray) (List (ByteArray))) (piece : ByteArray) => collectStep (state) (piece))) (Except.ok (([] : List (ByteArray)))) (LexLeanStd.Text.splitOn (list) ((44 : UInt8))) : Except (ByteArray) (List (ByteArray))) with | Except.error message => malformed (message) | Except.ok sorted => (match sorted with | List.nil => malformed (text ("an axiom list cannot be empty")) | List.cons _ _ => parsed (name) (sorted)))

@[expose] public def listed (payload : ByteArray) (body : ByteArray) : Except (ByteArray) ((Prod (ByteArray) (List (ByteArray)))) := (match splitLast (body) (text ("' depends on axioms: [")) with | Option.some split => (let name : ByteArray := (split).1; (let list : ByteArray := (split).2; (let listed_axioms : Except (ByteArray) ((Prod (ByteArray) (List (ByteArray)))) := axiomList (name) (list); (let refused : Except (ByteArray) ((Prod (ByteArray) (List (ByteArray)))) := malformed (notLean (name)); (if isClosedLeanName (name) then listed_axioms else refused))))) | Option.none => unrecognized (payload))

@[expose] public def parsePayload (payload : ByteArray) : Except (ByteArray) ((Prod (ByteArray) (List (ByteArray)))) := (match stripPrefix (payload) (text ("'")) with | Option.none => unrecognized (payload) | Option.some rest => (match stripSuffix (rest) (text ("' does not depend on any axioms")) with | Option.some name => (let accepted : Except (ByteArray) ((Prod (ByteArray) (List (ByteArray)))) := parsed (name) (([] : List (ByteArray))); (let refused : Except (ByteArray) ((Prod (ByteArray) (List (ByteArray)))) := malformed (notLean (name)); (if isClosedLeanName (name) then accepted else refused))) | Option.none => (match stripSuffix (rest) (text ("]")) with | Option.some body => listed (payload) (body) | Option.none => unrecognized (payload))))

@[expose] public def headOf (names : List (ByteArray)) : Option (ByteArray) := (match names with | List.nil => Option.none | List.cons head _ => Option.some (head))

@[expose] public def tailOf (names : List (ByteArray)) : List (ByteArray) := (match names with | List.nil => ([] : List (ByteArray)) | List.cons _ tail => tail)

@[expose] public def stop (message : ByteArray) (declaration : Option (ByteArray)) : Scan := Scan.stopped (rejected (message) (declaration))

@[expose] public def extend (records : List (Record)) (remaining : List (ByteArray)) (payload : ByteArray) (about : Option (ByteArray)) : Scan := (if terminates (payload) then (match parsePayload (payload) with | Except.error message => stop (message) (about) | Except.ok found => Scan.between ((({ declaration := (found).1, axioms := (found).2, expected := about } : Record) :: records)) (tailOf (remaining))) else Scan.inside (records) (remaining) (payload) (about))

@[expose] public def continuation (line : ByteArray) : Option (ByteArray) := (let cleaned : ByteArray := trimmed (line); (if startsWhite (line) then (if (Nat.beq (0) ((LexLeanRuntime.length (cleaned) : Nat))) then Option.none else Option.some (cleaned)) else Option.none))

@[expose] public def scanStep (state : Scan) (line : ByteArray) : Scan := (match state with | Scan.stopped _ => state | Scan.inside records remaining payload about => (match continuation (line) with | Option.some more => extend (records) (remaining) (cat (cat (payload) (text (" "))) (more)) (about) | Option.none => stop (cat (text ("unterminated axiom payload: ")) (payload)) (about)) | Scan.between records remaining => (if (Nat.beq (0) ((LexLeanRuntime.length (trimmed (line)) : Nat))) then state else (match envelope (trimmed (line)) with | Option.some payload => extend (records) (remaining) (payload) (headOf (remaining)) | Option.none => stop (cat (text ("unrecognized audit line: ")) (trimmed (line))) (headOf (remaining)))))

@[expose] public def auditStep (state : Except (Rejection) (List ((Prod (ByteArray) (List (ByteArray)))))) (entry : Record) : Except (Rejection) (List ((Prod (ByteArray) (List (ByteArray))))) := (match state with | Except.error _ => state | Except.ok entries => (let want : ByteArray := (match (entry).expected with | Option.some name => name | Option.none => _root_.ByteArray.mk #[]); (let duplicate : Except (Rejection) (List ((Prod (ByteArray) (List (ByteArray))))) := refuse (cat (text ("duplicate record for ")) (quoted ((entry).declaration))) (Option.some (want)); (let mismatch : Except (Rejection) (List ((Prod (ByteArray) (List (ByteArray))))) := refuse (cat (cat (cat (text ("expected a record for ")) (quoted (want))) (text (", found "))) (quoted ((entry).declaration))) (Option.some (want)); (if (LexLeanRuntime.equal ((entry).declaration) (want) : Bool) then (match insertEntry ((entry).declaration) ((entry).axioms) (entries) with | Option.some inserted => Except.ok (inserted) | Option.none => duplicate) else mismatch)))))

@[expose] public def finish (scan : Scan) (expected : List (ByteArray)) : Except (Rejection) (List ((Prod (ByteArray) (List (ByteArray))))) := (match scan with | Scan.stopped rejection => Except.error (rejection) | Scan.inside _ _ payload about => refuse (cat (text ("unterminated axiom payload: ")) (payload)) (about) | Scan.between records _ => (let counted : Except (Rejection) (List ((Prod (ByteArray) (List (ByteArray))))) := refuse (cat (cat (cat (text ("expected ")) (decimal (listLength (ByteArray) (expected)))) (text (" audit records, found "))) (decimal (listLength (Record) (records)))) (Option.none); (if (Nat.beq (listLength (Record) (records)) (listLength (ByteArray) (expected))) then (LexLeanCollections.listFold ((fun (state : Except (Rejection) (List ((Prod (ByteArray) (List (ByteArray)))))) (entry : Record) => auditStep (state) (entry))) (Except.ok (([] : List ((Prod (ByteArray) (List (ByteArray))))))) (reversed (Record) (records)) : Except (Rejection) (List ((Prod (ByteArray) (List (ByteArray)))))) else counted)))

@[expose] public def parseAuditOutput (stdout : ByteArray) (expected : List (ByteArray)) : Except (Rejection) (List ((Prod (ByteArray) (List (ByteArray))))) := finish ((LexLeanCollections.listFold ((fun (state : Scan) (line : ByteArray) => scanStep (state) (line))) (Scan.between (([] : List (Record))) (expected)) (LexLeanStd.Text.lines (stdout)) : Scan)) (expected)

end LexLeanBoot.AxiomOutput
