module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Json
public import LexLeanStd.Numeral
public import LexLeanStd.Order
public import LexLeanStd.Text
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.Diagnostics

public structure Span where
  path : ByteArray
  byteStart : Nat
  byteEnd : Nat
  lineStart : Nat
  columnStart : Nat
  lineEnd : Nat
  columnEnd : Nat

public structure Label where
  message : ByteArray
  span : Span

public structure Note where
  message : ByteArray
  span : Option (Span)

public structure Diagnostic where
  code : ByteArray
  message : ByteArray
  primary : Option (Span)
  labels : List (Label)
  notes : List (Note)
  help : List (ByteArray)
  causes : List (ByteArray)

@[expose] public def lit (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def cat (left : ByteArray) (right : ByteArray) : ByteArray := (LexLeanRuntime.append (left) (right) : ByteArray)

@[expose] public def jsonString (text : ByteArray) : ByteArray := LexLeanStd.Json.escapeString (text)

@[expose] public def num (value : Nat) : ByteArray := LexLeanStd.Numeral.decimal (value)

@[expose] public def field (key : ByteArray) (shown : ByteArray) : ByteArray := cat (cat (jsonString (key)) (lit (":"))) (shown)

@[expose] public def objectOf (fields : List (ByteArray)) : ByteArray := cat (cat (lit ("{")) (LexLeanStd.Text.joinWith (fields) (lit (",")))) (lit ("}"))

@[expose] public def arrayOf (items : List (ByteArray)) : ByteArray := cat (cat (lit ("[")) (LexLeanStd.Text.joinWith (items) (lit (",")))) (lit ("]"))

@[expose] public def spanText (span : Span) : ByteArray := objectOf ((field (lit ("byte_end")) (num ((span).byteEnd)) :: (field (lit ("byte_start")) (num ((span).byteStart)) :: (field (lit ("column_end")) (num ((span).columnEnd)) :: (field (lit ("column_start")) (num ((span).columnStart)) :: (field (lit ("line_end")) (num ((span).lineEnd)) :: (field (lit ("line_start")) (num ((span).lineStart)) :: (field (lit ("path")) (jsonString ((span).path)) :: ([] : List (ByteArray))))))))))

@[expose] public def labelText (label : Label) : ByteArray := objectOf ((field (lit ("message")) (jsonString ((label).message)) :: (field (lit ("span")) (spanText ((label).span)) :: ([] : List (ByteArray)))))

@[expose] public def noteText (note : Note) : ByteArray := (match (note).span with | Option.some span => objectOf ((field (lit ("message")) (jsonString ((note).message)) :: (field (lit ("span")) (spanText (span)) :: ([] : List (ByteArray))))) | Option.none => objectOf ((field (lit ("message")) (jsonString ((note).message)) :: ([] : List (ByteArray)))))

@[expose] public def strings (items : List (ByteArray)) : ByteArray := arrayOf ((LexLeanCollections.listFold ((fun (acc : List (ByteArray)) (item : ByteArray) => (jsonString (item) :: acc))) (([] : List (ByteArray))) (LexLeanStd.Order.reverseList (ByteArray) (items)) : List (ByteArray)))

@[expose] public def labelsText (items : List (Label)) : ByteArray := arrayOf ((LexLeanCollections.listFold ((fun (acc : List (ByteArray)) (item : Label) => (labelText (item) :: acc))) (([] : List (ByteArray))) (LexLeanStd.Order.reverseList (Label) (items)) : List (ByteArray)))

@[expose] public def notesText (items : List (Note)) : ByteArray := arrayOf ((LexLeanCollections.listFold ((fun (acc : List (ByteArray)) (item : Note) => (noteText (item) :: acc))) (([] : List (ByteArray))) (LexLeanStd.Order.reverseList (Note) (items)) : List (ByteArray)))

@[expose] public def diagnosticText (given : Diagnostic) : ByteArray := (let tail : List (ByteArray) := (field (lit ("severity")) (jsonString (lit ("error"))) :: (field (lit ("spec")) (jsonString (lit ("lexlean/diagnostic/1"))) :: ([] : List (ByteArray)))); (let located : List (ByteArray) := (match (given).primary with | Option.some span => (field (lit ("primary")) (spanText (span)) :: tail) | Option.none => tail); objectOf ((field (lit ("causes")) (strings ((given).causes)) :: (field (lit ("code")) (jsonString ((given).code)) :: (field (lit ("help")) (strings ((given).help)) :: (field (lit ("labels")) (labelsText ((given).labels)) :: (field (lit ("message")) (jsonString ((given).message)) :: (field (lit ("notes")) (notesText ((given).notes)) :: located)))))))))

@[expose] public def byteText (value : Nat) : ByteArray := LexLeanStd.Numeral.pick (LexLeanStd.Numeral.byteValues) (value)

@[expose] public def beStep (state : (Prod (Nat) ((Prod (Nat) (ByteArray))))) : Option ((Prod (Nat) ((Prod (Nat) (ByteArray))))) := (if (Nat.beq ((state).1) (0)) then Option.none else Option.some (((LexLeanRuntime.subtract ((state).1) (1) : Nat), ((LexLeanRuntime.quotient (((state).2).1) (256) (0) : Nat), cat (byteText ((LexLeanRuntime.remainder (((state).2).1) (256) (0) : Nat))) (((state).2).2)))))

@[expose] public def bigEndian (value : Nat) : ByteArray := ((((LexLeanCollections.iterateUntil ((fun (state : (Prod (Nat) ((Prod (Nat) (ByteArray))))) => beStep (state))) (9) ((8, (value, _root_.ByteArray.mk #[]))) : (Prod ((Prod (Nat) ((Prod (Nat) (ByteArray))))) (Bool)))).1).2).2

@[expose] public def orderStep (path : ByteArray) (n : Nat) (state : (Prod (Nat) (ByteArray))) : Option ((Prod (Nat) (ByteArray))) := (let position : Nat := (state).1; (if (Nat.blt (position) (n)) then (match LexLeanStd.Text.byteAt (path) (position) with | Option.some byte => (if (LexLeanRuntime.equal (byte) ((0 : UInt8)) : Bool) then Option.some (((position + 1), cat ((state).2) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 0)), (_root_.UInt8.ofNat (nat_lit 1))]))) else Option.some (((position + 1), cat ((state).2) (byteText ((let __widen0 : UInt8 := byte; ((((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((128 : UInt8)) : UInt8)) ((128 : UInt8)) : Bool) then 128 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((64 : UInt8)) : UInt8)) ((64 : UInt8)) : Bool) then 64 else 0)) + ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((32 : UInt8)) : UInt8)) ((32 : UInt8)) : Bool) then 32 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((16 : UInt8)) : UInt8)) ((16 : UInt8)) : Bool) then 16 else 0))) + (((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((8 : UInt8)) : UInt8)) ((8 : UInt8)) : Bool) then 8 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((4 : UInt8)) : UInt8)) ((4 : UInt8)) : Bool) then 4 else 0)) + ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((2 : UInt8)) : UInt8)) ((2 : UInt8)) : Bool) then 2 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((1 : UInt8)) : UInt8)) ((1 : UInt8)) : Bool) then 1 else 0)))))))))) | Option.none => Option.none) else Option.none))

@[expose] public def orderedPath (path : ByteArray) : ByteArray := (let n : Nat := (LexLeanRuntime.length (path) : Nat); cat ((((LexLeanCollections.iterateUntil ((fun (state : (Prod (Nat) (ByteArray))) => orderStep (path) (n) (state))) ((n + 1)) ((0, _root_.ByteArray.mk #[])) : (Prod ((Prod (Nat) (ByteArray))) (Bool)))).1).2) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 0)), (_root_.UInt8.ofNat (nat_lit 0))]))

@[expose] public def sortKey (given : Diagnostic) : ByteArray := (let place : (Prod (ByteArray) (Nat)) := (match (given).primary with | Option.some span => ((span).path, (span).byteStart) | Option.none => (_root_.ByteArray.mk #[], 0)); cat (cat (cat (orderedPath ((place).1)) (bigEndian ((place).2))) ((given).code)) ((given).message))

@[expose] public def keyed (given : Diagnostic) : (Prod (ByteArray) (Diagnostic)) := (sortKey (given), given)

@[expose] public def sortDiagnostics (items : List (Diagnostic)) : List (Diagnostic) := (LexLeanCollections.listFold ((fun (acc : List (Diagnostic)) (entry : (Prod (ByteArray) (Diagnostic))) => ((entry).2 :: acc))) (([] : List (Diagnostic))) (LexLeanStd.Order.reverseList ((Prod (ByteArray) (Diagnostic))) (LexLeanStd.Order.sortByKey (Diagnostic) (LexLeanStd.Order.reverseList ((Prod (ByteArray) (Diagnostic))) ((LexLeanCollections.listFold ((fun (acc : List ((Prod (ByteArray) (Diagnostic)))) (item : Diagnostic) => (keyed (item) :: acc))) (([] : List ((Prod (ByteArray) (Diagnostic))))) (items) : List ((Prod (ByteArray) (Diagnostic)))))))) : List (Diagnostic))

@[expose] public def isEnvironmentCode (code : ByteArray) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (listed : ByteArray) => (if found then true else (LexLeanRuntime.equal (code) (listed) : Bool)))) (false) ((lit ("LLB6004") :: (lit ("LLV7001") :: (lit ("LLV7007") :: (lit ("LLV7008") :: (lit ("LLV7009") :: (lit ("LLV7010") :: (lit ("LLV7012") :: (lit ("LLV7014") :: (lit ("LLV7017") :: (lit ("LLV7018") :: ([] : List (ByteArray))))))))))))) : Bool)

@[expose] public def codeExit (code : ByteArray) : Nat := (if isEnvironmentCode (code) then 3 else (match LexLeanStd.Text.byteAt (code) (2) with | Option.some letter => (let value : Nat := (let __widen0 : UInt8 := letter; ((((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((128 : UInt8)) : UInt8)) ((128 : UInt8)) : Bool) then 128 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((64 : UInt8)) : UInt8)) ((64 : UInt8)) : Bool) then 64 else 0)) + ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((32 : UInt8)) : UInt8)) ((32 : UInt8)) : Bool) then 32 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((16 : UInt8)) : UInt8)) ((16 : UInt8)) : Bool) then 16 else 0))) + (((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((8 : UInt8)) : UInt8)) ((8 : UInt8)) : Bool) then 8 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((4 : UInt8)) : UInt8)) ((4 : UInt8)) : Bool) then 4 else 0)) + ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((2 : UInt8)) : UInt8)) ((2 : UInt8)) : Bool) then 2 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((1 : UInt8)) : UInt8)) ((1 : UInt8)) : Bool) then 1 else 0))))); (if (Nat.beq (value) (67)) then 2 else (if (Nat.beq (value) (83)) then 4 else (if (Nat.beq (value) (73)) then 70 else 1)))) | Option.none => 1))

@[expose] public def worstExit (items : List (Diagnostic)) (empty : Nat) : Nat := (match items with | List.nil => empty | List.cons _ _ => (LexLeanCollections.listFold ((fun (worst : Nat) (item : Diagnostic) => (let rank : Nat := codeExit ((item).code); (if (Nat.blt (worst) (rank)) then rank else worst)))) (0) (items) : Nat))

@[expose] public def takeStep (state : (Prod (Nat) ((Prod (List (Diagnostic)) (List (Diagnostic)))))) : Option ((Prod (Nat) ((Prod (List (Diagnostic)) (List (Diagnostic)))))) := (if (Nat.beq ((state).1) (0)) then Option.none else (match ((state).2).1 with | List.nil => Option.none | List.cons head rest => Option.some (((LexLeanRuntime.subtract ((state).1) (1) : Nat), (rest, (head :: ((state).2).2))))))

@[expose] public def firstOf (items : List (Diagnostic)) (count : Nat) : List (Diagnostic) := LexLeanStd.Order.reverseList (Diagnostic) (((((LexLeanCollections.iterateUntil ((fun (state : (Prod (Nat) ((Prod (List (Diagnostic)) (List (Diagnostic)))))) => takeStep (state))) ((count + 1)) ((count, (items, ([] : List (Diagnostic))))) : (Prod ((Prod (Nat) ((Prod (List (Diagnostic)) (List (Diagnostic)))))) (Bool)))).1).2).2)

@[expose] public def exceeded (phase : ByteArray) (limit : Nat) (observed : Nat) : Diagnostic := ({ code := lit ("LLS8002"), message := cat (cat (cat (cat (cat (cat (cat (cat (lit ("max_diagnostics exceeded in phase ")) (phase)) (lit (": configured "))) (num (limit))) (lit (", observed "))) (num (observed))) (lit ("; the first "))) (num (limit))) (lit (" diagnostics are reported")), primary := Option.none, labels := ([] : List (Label)), notes := ([] : List (Note)), help := ([] : List (ByteArray)), causes := ([] : List (ByteArray)) } : Diagnostic)

@[expose] public def reportDiagnostics (items : List (Diagnostic)) (limit : Nat) (phase : ByteArray) : (Prod (Nat) (List (Diagnostic))) := (let sorted : List (Diagnostic) := sortDiagnostics (items); (let observed : Nat := (LexLeanRuntime.length (items) : Nat); (if (Nat.ble (observed) (limit)) then (worstExit (sorted) (70), sorted) else (let kept : List (Diagnostic) := LexLeanStd.Order.reverseList (Diagnostic) ((exceeded (phase) (limit) (observed) :: LexLeanStd.Order.reverseList (Diagnostic) (firstOf (sorted) (limit)))); (worstExit (kept) (70), kept)))))

end LexLeanBoot.Diagnostics
